//! Receiving the authorization code: via a local HTTP listener, or pasted by the user.

use crate::output::sanitize;
use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use url::Url;

/// The largest request head the listener reads before giving up on a connection.
const MAX_REQUEST_HEAD: usize = 8 * 1024;

/// What WakaTime passed back to the redirect URI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Callback {
    /// The user approved the app.
    Code { code: String, state: Option<String> },
    /// The user declined, or the request was invalid.
    Denied {
        error: String,
        description: Option<String>,
        state: Option<String>,
    },
}

impl Callback {
    /// Extracts the callback from a URL's query parameters.
    pub fn from_url(url: &Url) -> Result<Self, Box<dyn Error>> {
        let mut code = None;
        let mut state = None;
        let mut error = None;
        let mut description = None;
        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "code" => code = Some(value.into_owned()),
                "state" => state = Some(value.into_owned()),
                "error" => error = Some(value.into_owned()),
                "error_description" => description = Some(value.into_owned()),
                _ => {}
            }
        }
        match (code, error) {
            (_, Some(error)) => Ok(Callback::Denied {
                error,
                description,
                state,
            }),
            (Some(code), None) if !code.is_empty() => Ok(Callback::Code { code, state }),
            _ => Err("the redirect contains neither a code nor an error".into()),
        }
    }

    /// Parses what the user pasted in manual mode: the full redirected URL,
    /// or just the code.
    pub fn from_pasted(input: &str) -> Result<Self, Box<dyn Error>> {
        let input = input.trim();
        if input.is_empty() {
            return Err("no code was entered".into());
        }
        if input.contains("://") {
            let url = Url::parse(input).map_err(|e| format!("invalid URL: {e}"))?;
            return Self::from_url(&url);
        }
        if input.contains(['?', '=', '&']) {
            // A bare query string, e.g. "?code=…&state=…".
            let url = Url::parse(&format!(
                "http://localhost/?{}",
                input.trim_start_matches('?')
            ))?;
            return Self::from_url(&url);
        }
        Ok(Callback::Code {
            code: input.to_string(),
            state: None,
        })
    }

    /// Whether the callback carries the given state.
    pub fn has_state(&self, expected_state: &str) -> bool {
        let state = match self {
            Callback::Code { state, .. } | Callback::Denied { state, .. } => state,
        };
        state
            .as_deref()
            .is_some_and(|state| constant_time_eq(state.as_bytes(), expected_state.as_bytes()))
    }

    /// Returns the code if the user approved and the state matches.
    ///
    /// A missing state is accepted only when `allow_missing_state` is set,
    /// i.e. when the user pasted a bare code.
    pub fn into_code(
        self,
        expected_state: &str,
        allow_missing_state: bool,
    ) -> Result<String, Box<dyn Error>> {
        let state_ok = self.has_state(expected_state);
        match self {
            Callback::Code { state: None, code } if allow_missing_state => Ok(code),
            Callback::Code { code, .. } if state_ok => Ok(code),
            // Without a matching state, a denial may be forged as well; still
            // report it, since nothing is exchanged either way.
            Callback::Denied {
                error, description, ..
            } => Err(match description {
                Some(description) => format!(
                    "authorization failed: {}: {}",
                    sanitize(&error),
                    sanitize(&description)
                ),
                None => format!("authorization failed: {}", sanitize(&error)),
            }
            .into()),
            Callback::Code { .. } => Err("the state parameter does not match; the redirect did \
                                          not come from this sign-in attempt"
                .into()),
        }
    }
}

/// Compares two byte strings in time independent of where they differ.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// Generates an unguessable `state` value.
pub fn random_state() -> Result<String, Box<dyn Error>> {
    let mut bytes = [0u8; 24];
    getrandom::fill(&mut bytes).map_err(|e| format!("cannot generate random state: {e}"))?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

/// The local address to listen on for the redirect, if the redirect URI
/// points at this machine over plain HTTP.
pub fn loopback_address(redirect_uri: &Url) -> Option<SocketAddr> {
    if redirect_uri.scheme() != "http" {
        return None;
    }
    let port = redirect_uri.port_or_known_default()?;
    match redirect_uri.host()? {
        url::Host::Ipv4(ip) if ip.is_loopback() => Some(SocketAddr::from((ip, port))),
        url::Host::Ipv6(ip) if ip.is_loopback() => Some(SocketAddr::from((ip, port))),
        url::Host::Domain("localhost") => Some(SocketAddr::from(([127, 0, 0, 1], port))),
        _ => None,
    }
}

/// Listens for the redirect from WakaTime.
pub struct CallbackListener {
    listener: TcpListener,
    path: String,
}

impl CallbackListener {
    /// Starts listening on the address the redirect URI points at.
    pub async fn bind(address: SocketAddr, redirect_uri: &Url) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind(address).await.map_err(|e| {
            format!("cannot listen on {address} for the redirect: {e}; use --manual or another --redirect-uri")
        })?;
        Ok(Self {
            listener,
            path: redirect_uri.path().to_string(),
        })
    }

    /// The address actually listened on.
    #[cfg(test)]
    pub fn local_addr(&self) -> SocketAddr {
        self.listener.local_addr().expect("listener has an address")
    }

    /// Waits for a request to the redirect path carrying `expected_state`,
    /// answers it, and returns its parameters.
    ///
    /// Requests to other paths, like a browser's favicon request, get a 404.
    /// Redirects with a missing or different state get a 400 and are ignored,
    /// so another local process or web page cannot abort the sign-in by
    /// sending a forged redirect first.
    pub async fn wait(
        self,
        timeout: Duration,
        expected_state: &str,
    ) -> Result<Callback, Box<dyn Error>> {
        tokio::time::timeout(timeout, async {
            loop {
                let (mut stream, _) = self.listener.accept().await?;
                // A client that connects but never sends must not block the sign-in.
                let target = match tokio::time::timeout(
                    Duration::from_secs(10),
                    read_request_target(&mut stream),
                )
                .await
                {
                    Ok(Ok(target)) => target,
                    _ => continue,
                };
                let url = match Url::parse("http://localhost")?.join(&target) {
                    Ok(url) if url.path() == self.path => url,
                    _ => {
                        respond(&mut stream, "404 Not Found", "Not found.").await;
                        continue;
                    }
                };
                match Callback::from_url(&url) {
                    Ok(callback) if !callback.has_state(expected_state) => {
                        respond(
                            &mut stream,
                            "400 Bad Request",
                            "This redirect does not belong to the current sign-in.",
                        )
                        .await;
                    }
                    Ok(callback @ Callback::Code { .. }) => {
                        respond(
                            &mut stream,
                            "200 OK",
                            "Signed in to WakaTime. You can close this tab and return to the terminal.",
                        )
                        .await;
                        return Ok(callback);
                    }
                    Ok(callback @ Callback::Denied { .. }) => {
                        respond(&mut stream, "200 OK", "Sign-in was not completed. See the terminal for details.").await;
                        return Ok(callback);
                    }
                    Err(_) => {
                        respond(&mut stream, "400 Bad Request", "Missing code.").await;
                    }
                }
            }
        })
        .await
        .map_err(|_| "timed out waiting for the sign-in to complete")?
    }
}

/// Reads an HTTP request head and returns the target of a GET request,
/// e.g. `/callback?code=…`.
async fn read_request_target(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let mut head = Vec::new();
    let mut buffer = [0u8; 1024];
    while !head.windows(4).any(|w| w == b"\r\n\r\n") {
        let n = stream.read(&mut buffer).await?;
        if n == 0 || head.len() + n > MAX_REQUEST_HEAD {
            return Err("incomplete or oversized request".into());
        }
        head.extend_from_slice(&buffer[..n]);
    }
    parse_request_line(&String::from_utf8_lossy(&head))
}

/// Extracts the target from the request line of a GET request.
fn parse_request_line(head: &str) -> Result<String, Box<dyn Error>> {
    let line = head.lines().next().unwrap_or_default();
    let mut parts = line.split(' ');
    match (parts.next(), parts.next(), parts.next()) {
        (Some("GET"), Some(target), Some(version))
            if target.starts_with('/') && version.starts_with("HTTP/") =>
        {
            Ok(target.to_string())
        }
        _ => Err("not a GET request".into()),
    }
}

/// Sends a minimal HTML response and closes the connection.
async fn respond(stream: &mut TcpStream, status: &str, message: &str) {
    let body = format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>waka-cli</title></head>\
         <body><p>{message}</p></body></html>"
    );
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\n\
         Content-Length: {length}\r\nConnection: close\r\n\r\n{body}",
        length = body.len()
    );
    // The browser going away early is not worth failing the sign-in for.
    let _ = stream.write_all(response.as_bytes()).await;
    let _ = stream.shutdown().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{assert, check};

    fn url(s: &str) -> Url {
        Url::parse(s).unwrap()
    }

    #[test]
    fn callback_from_url_extracts_code_and_state() {
        let callback = Callback::from_url(&url(
            "http://127.0.0.1:8737/callback?code=abc%2B123&state=xyz&extra=1",
        ))
        .unwrap();
        check!(
            callback
                == Callback::Code {
                    code: "abc+123".into(),
                    state: Some("xyz".into())
                }
        );
    }

    #[test]
    fn callback_from_url_prefers_errors() {
        let callback = Callback::from_url(&url(
            "http://127.0.0.1/callback?error=access_denied&error_description=User+declined&code=x",
        ))
        .unwrap();
        check!(
            callback
                == Callback::Denied {
                    error: "access_denied".into(),
                    description: Some("User declined".into()),
                    state: None,
                }
        );
    }

    #[test]
    fn callback_from_url_requires_code_or_error() {
        check!(Callback::from_url(&url("http://127.0.0.1/callback")).is_err());
        check!(Callback::from_url(&url("http://127.0.0.1/callback?code=")).is_err());
    }

    #[test]
    fn pasted_input_accepts_urls_queries_and_bare_codes() {
        check!(
            Callback::from_pasted("  https://wakatime.com/oauth/test?code=c1&state=s1\n").unwrap()
                == Callback::Code {
                    code: "c1".into(),
                    state: Some("s1".into())
                }
        );
        check!(
            Callback::from_pasted("?code=c2&state=s2").unwrap()
                == Callback::Code {
                    code: "c2".into(),
                    state: Some("s2".into())
                }
        );
        check!(
            Callback::from_pasted("c3").unwrap()
                == Callback::Code {
                    code: "c3".into(),
                    state: None
                }
        );
        check!(Callback::from_pasted("   ").is_err());
        check!(Callback::from_pasted("https://[bad").is_err());
    }

    #[test]
    fn into_code_checks_state() {
        let callback = |state: Option<&str>| Callback::Code {
            code: "the-code".into(),
            state: state.map(Into::into),
        };

        check!(callback(Some("s")).into_code("s", false).unwrap() == "the-code");
        check!(callback(Some("other")).into_code("s", false).is_err());
        check!(callback(Some("other")).into_code("s", true).is_err());
        check!(callback(Some("")).into_code("s", true).is_err());
        check!(callback(None).into_code("s", false).is_err());
        check!(callback(None).into_code("s", true).unwrap() == "the-code");
    }

    #[test]
    fn into_code_reports_denial() {
        let result = Callback::Denied {
            error: "access_denied".into(),
            description: None,
            state: Some("s".into()),
        }
        .into_code("s", false);

        assert!(let Err(e) = result);
        check!(e.to_string() == "authorization failed: access_denied");
    }

    #[test]
    fn into_code_strips_terminal_escapes_from_denials() {
        let result = Callback::Denied {
            error: "x\u{1b}]0;pwned\u{7}".into(),
            description: Some("line\r\nfake: ok\u{1b}[2J".into()),
            state: None,
        }
        .into_code("s", true);

        assert!(let Err(e) = result);
        let message = e.to_string();
        check!(!message.chars().any(char::is_control));
        check!(
            message
                == "authorization failed: x\u{fffd}]0;pwned\u{fffd}: line\u{fffd}\u{fffd}fake: ok\u{fffd}[2J"
        );
    }

    #[test]
    fn has_state_requires_an_exact_match() {
        let code = |state: Option<&str>| Callback::Code {
            code: "c".into(),
            state: state.map(Into::into),
        };
        check!(code(Some("s")).has_state("s"));
        check!(!code(Some("s2")).has_state("s"));
        check!(!code(Some("")).has_state("s"));
        check!(!code(None).has_state("s"));
    }

    #[test]
    fn random_state_is_unique_hex() {
        let a = random_state().unwrap();
        let b = random_state().unwrap();
        check!(a.len() == 48);
        check!(a.chars().all(|c| c.is_ascii_hexdigit()));
        check!(a != b);
    }

    #[test]
    fn loopback_address_accepts_only_local_http() {
        check!(
            loopback_address(&url("http://127.0.0.1:8737/callback"))
                == Some(SocketAddr::from(([127, 0, 0, 1], 8737)))
        );
        check!(
            loopback_address(&url("http://localhost:9000/cb"))
                == Some(SocketAddr::from(([127, 0, 0, 1], 9000)))
        );
        check!(
            loopback_address(&url("http://[::1]:8737/cb"))
                == Some(SocketAddr::from((std::net::Ipv6Addr::LOCALHOST, 8737)))
        );
        check!(
            loopback_address(&url("http://127.0.0.1/cb"))
                == Some(SocketAddr::from(([127, 0, 0, 1], 80)))
        );
        check!(loopback_address(&url("https://127.0.0.1:8737/callback")).is_none());
        check!(loopback_address(&url("https://wakatime.com/oauth/test")).is_none());
        check!(loopback_address(&url("http://192.168.1.2:8737/callback")).is_none());
    }

    #[test]
    fn request_line_parsing() {
        check!(
            parse_request_line("GET /callback?code=x HTTP/1.1\r\nHost: a\r\n\r\n").unwrap()
                == "/callback?code=x"
        );
        check!(parse_request_line("POST /callback HTTP/1.1\r\n\r\n").is_err());
        check!(parse_request_line("GET http://evil/ HTTP/1.1\r\n\r\n").is_err());
        check!(parse_request_line("garbage").is_err());
    }

    /// Sends a raw request to the listener and returns the response.
    async fn send(addr: SocketAddr, request: &str) -> String {
        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream.write_all(request.as_bytes()).await.unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).await.unwrap();
        response
    }

    async fn listener() -> CallbackListener {
        CallbackListener::bind(
            SocketAddr::from(([127, 0, 0, 1], 0)),
            &url("http://127.0.0.1:0/callback"),
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn listener_ignores_other_paths_and_returns_the_callback() {
        let listener = listener().await;
        let addr = listener.local_addr();
        let browser = async {
            let favicon = send(addr, "GET /favicon.ico HTTP/1.1\r\nHost: x\r\n\r\n").await;
            let missing = send(addr, "GET /callback HTTP/1.1\r\nHost: x\r\n\r\n").await;
            // Forged redirects without the right state must not end the sign-in.
            let forged = send(
                addr,
                "GET /callback?code=evil&state=wrong HTTP/1.1\r\nHost: x\r\n\r\n",
            )
            .await;
            let forged_denial = send(
                addr,
                "GET /callback?error=access_denied HTTP/1.1\r\nHost: x\r\n\r\n",
            )
            .await;
            let ok = send(
                addr,
                "GET /callback?code=the-code&state=s HTTP/1.1\r\nHost: x\r\n\r\n",
            )
            .await;
            (favicon, missing, forged, forged_denial, ok)
        };

        let (callback, (favicon, missing, forged, forged_denial, ok)) =
            tokio::join!(listener.wait(Duration::from_secs(10), "s"), browser);

        check!(favicon.starts_with("HTTP/1.1 404 Not Found"));
        check!(missing.starts_with("HTTP/1.1 400 Bad Request"));
        check!(forged.starts_with("HTTP/1.1 400 Bad Request"));
        check!(forged.contains("does not belong to the current sign-in"));
        check!(forged_denial.starts_with("HTTP/1.1 400 Bad Request"));
        check!(ok.starts_with("HTTP/1.1 200 OK"));
        check!(ok.contains("You can close this tab"));
        let callback = callback.unwrap();
        check!(
            callback
                == Callback::Code {
                    code: "the-code".into(),
                    state: Some("s".into())
                }
        );
    }

    #[tokio::test]
    async fn listener_survives_garbage_connections() {
        let listener = listener().await;
        let addr = listener.local_addr();
        let browser = async {
            // A connection that closes without a full request.
            drop(TcpStream::connect(addr).await.unwrap());
            // A non-GET request.
            send(addr, "POST /callback HTTP/1.1\r\n\r\n").await;
            send(
                addr,
                "GET /callback?error=access_denied&state=s HTTP/1.1\r\nHost: x\r\n\r\n",
            )
            .await;
        };

        let (callback, ()) = tokio::join!(listener.wait(Duration::from_secs(10), "s"), browser);
        let callback = callback.unwrap();
        check!(
            callback
                == Callback::Denied {
                    error: "access_denied".into(),
                    description: None,
                    state: Some("s".into()),
                }
        );
    }

    #[tokio::test]
    async fn listener_times_out() {
        let result = listener().await.wait(Duration::from_millis(50), "s").await;

        assert!(let Err(e) = result);
        check!(e.to_string().contains("timed out"));
    }
}
