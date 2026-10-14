//! OAuth 2.0 sign-in, and choosing the credentials for API calls.

pub mod callback;
pub mod store;

use crate::cli::AuthCommand;
use callback::{Callback, CallbackListener};
use std::error::Error;
use std::time::Duration;
use store::{Store, StoredCredentials};
use url::Url;
use waka::ApiError;
use waka::oauth::{AuthorizeOptions, OAuthClient, Scope, Token};

/// How long to wait for the user to approve the app in the browser.
const LOGIN_TIMEOUT: Duration = Duration::from_secs(5 * 60);

/// The credentials given on the command line or in the environment.
#[derive(Debug, Default, Clone, Copy)]
pub struct CredentialSources<'a> {
    pub bearer_token: Option<&'a str>,
    pub api_key: Option<&'a str>,
    /// Whether the API key was passed as a flag, rather than via the environment.
    pub api_key_explicit: bool,
}

/// The credentials to authenticate API calls with.
pub enum Credential {
    BearerToken(String),
    ApiKey(String),
    OAuth(Box<StoredCredentials>, Store),
}

impl Credential {
    /// Picks the credentials to use, in this order:
    ///
    /// 1. `--bearer-token` / `WAKATIME_BEARER_TOKEN`
    /// 2. `--api-key`, when given as a flag
    /// 3. the token stored by `auth login`
    /// 4. `WAKATIME_API_KEY`, e.g. from a `.env` file
    ///
    /// An API key from the environment comes last because API keys are
    /// write-only, and a long-lived `.env` entry should not shadow a sign-in.
    pub async fn resolve(
        sources: CredentialSources<'_>,
        store: &Store,
    ) -> Result<Self, Box<dyn Error>> {
        if let Some(token) = sources.bearer_token {
            return Ok(Credential::BearerToken(token.to_string()));
        }
        if let (Some(api_key), true) = (sources.api_key, sources.api_key_explicit) {
            return Ok(Credential::ApiKey(api_key.to_string()));
        }
        if let Some((credentials, from)) = store.load().await? {
            return Ok(Credential::OAuth(Box::new(credentials), from));
        }
        if let Some(api_key) = sources.api_key {
            return Ok(Credential::ApiKey(api_key.to_string()));
        }
        Err(
            "missing credentials: sign in with `waka-cli auth login`, or set --bearer-token \
             (WAKATIME_BEARER_TOKEN) or --api-key (WAKATIME_API_KEY; write-only)"
                .into(),
        )
    }

    /// The client builder authenticating with these credentials.
    pub fn builder(&self) -> waka::WakaTimeClientBuilder {
        match self {
            Credential::BearerToken(token) => {
                waka::WakaTimeClientBuilder::new_with_bearer_token(token)
            }
            Credential::ApiKey(api_key) => waka::WakaTimeClientBuilder::new_with_api_key(api_key),
            Credential::OAuth(credentials, _) => {
                waka::WakaTimeClientBuilder::new_with_oauth_token(&credentials.token)
            }
        }
    }

    /// A description of where the credentials come from.
    fn describe(&self) -> String {
        match self {
            Credential::BearerToken(_) => {
                "bearer token from --bearer-token / WAKATIME_BEARER_TOKEN".into()
            }
            Credential::ApiKey(_) => {
                "API key from --api-key / WAKATIME_API_KEY (write-only: cannot read data)".into()
            }
            Credential::OAuth(_, from) => format!("OAuth token from {from}"),
        }
    }

    /// Suggests a fix for an API error caused by these credentials.
    pub fn hint(&self, error: &(dyn Error + 'static)) -> Option<&'static str> {
        let error = error.downcast_ref::<ApiError>()?;
        match (error, self) {
            (ApiError::Unauthorized(_) | ApiError::Forbidden(_), Credential::ApiKey(_)) => Some(
                "WakaTime API keys are write-only; sign in with `waka-cli auth login` to read data",
            ),
            (ApiError::Unauthorized(_), _) => Some(
                "the token may be invalid or revoked; sign in again with `waka-cli auth login`",
            ),
            (ApiError::Forbidden(_), _) => Some(
                "the token may lack the scope this command needs; check `waka-cli auth status` \
                 and sign in again with `waka-cli auth login --scope …`",
            ),
            _ => None,
        }
    }
}

/// Options for [`login`].
pub struct LoginOptions<'a> {
    pub scopes: &'a [Scope],
    pub force_approve: bool,
    pub no_browser: bool,
    pub manual: bool,
}

/// Runs the authorization code flow and returns the issued token.
///
/// `open_browser` opens the authorization URL and reports whether that worked;
/// `read_pasted` reads the redirected URL or code from the user in manual mode.
pub async fn login(
    oauth: &OAuthClient,
    options: LoginOptions<'_>,
    open_browser: impl FnOnce(&str) -> bool,
    read_pasted: impl FnOnce() -> std::io::Result<String>,
) -> Result<Token, Box<dyn Error>> {
    let redirect_uri = Url::parse(oauth.redirect_uri())
        .map_err(|e| format!("invalid redirect URI {}: {e}", oauth.redirect_uri()))?;
    let state = callback::random_state()?;
    let url = oauth.authorize_url(AuthorizeOptions {
        scopes: options.scopes,
        state: Some(&state),
        force_approve: options.force_approve,
    });

    let listener = match (options.manual, callback::loopback_address(&redirect_uri)) {
        (false, Some(address)) => Some(CallbackListener::bind(address, &redirect_uri).await?),
        (false, None) => {
            eprintln!(
                "The redirect URI {redirect_uri} is not a local http:// address, so the \
                 redirect has to be pasted here."
            );
            None
        }
        (true, _) => None,
    };

    eprintln!("Open this URL in a browser to sign in to WakaTime:\n\n    {url}\n");
    if !options.no_browser && open_browser(&url) {
        eprintln!("(Opened in your default browser.)");
    }

    let code = match listener {
        Some(listener) => {
            eprintln!("Waiting for the redirect to {redirect_uri} …");
            listener
                .wait(LOGIN_TIMEOUT)
                .await?
                .into_code(&state, false)?
        }
        None => {
            eprintln!("After approving, paste the URL you were redirected to, or just the code:");
            let input = read_pasted()?;
            // A bare code carries no state, which is acceptable since the user
            // copied it from their own browser.
            Callback::from_pasted(&input)?.into_code(&state, true)?
        }
    };
    Ok(oauth.exchange_code(&code).await?)
}

/// Runs an `auth` subcommand.
pub async fn run(
    command: AuthCommand,
    sources: CredentialSources<'_>,
    store: &Store,
    oauth_base_url: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let with_base_url = |client: OAuthClient| match oauth_base_url {
        Some(base_url) => client.with_base_url(base_url),
        None => client,
    };

    match command {
        AuthCommand::Login {
            client_id,
            client_secret,
            redirect_uri,
            scopes,
            no_browser,
            manual,
            force_approve,
        } => {
            let oauth = with_base_url(OAuthClient::new(&client_id, &client_secret, &redirect_uri)?);
            let token = login(
                &oauth,
                LoginOptions {
                    scopes: &scopes,
                    force_approve,
                    no_browser,
                    manual,
                },
                |url| webbrowser::open(url).is_ok(),
                || {
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line)?;
                    Ok(line)
                },
            )
            .await?;

            let credentials = StoredCredentials {
                client_id,
                client_secret,
                redirect_uri,
                token,
            };
            let saved_to = store.save(&credentials).await?;
            println!("Signed in. The token is stored in {saved_to}.");
            print_token(&credentials.token);
            if sources.bearer_token.is_some() || sources.api_key_explicit {
                eprintln!(
                    "note: --bearer-token / --api-key / WAKATIME_BEARER_TOKEN take precedence \
                     over the stored token"
                );
            }
            Ok(())
        }

        AuthCommand::Status => {
            match Credential::resolve(sources, store).await {
                Ok(credential) => println!("In use: {}", credential.describe()),
                Err(e) => println!("In use: none ({e})"),
            }
            match store.load().await? {
                Some((credentials, from)) => {
                    println!("Stored OAuth token, in {from}:");
                    println!("  client ID: {}", credentials.client_id);
                    println!("  redirect URI: {}", credentials.redirect_uri);
                    print_token(&credentials.token);
                }
                None => println!("No stored OAuth token; sign in with `waka-cli auth login`."),
            }
            Ok(())
        }

        AuthCommand::Refresh => {
            let (mut credentials, _) = store
                .load()
                .await?
                .ok_or("not signed in; run `waka-cli auth login`")?;
            let refresh_token = credentials
                .token
                .refresh_token
                .clone()
                .ok_or("the stored token has no refresh token; run `waka-cli auth login`")?;
            let oauth = with_base_url(credentials.oauth_client()?);
            let mut token = oauth.refresh(&refresh_token).await?;
            if token.refresh_token.is_none() {
                token.refresh_token = Some(refresh_token);
            }
            credentials.token = token;
            let saved_to = store.save(&credentials).await?;
            println!("Refreshed the token in {saved_to}.");
            print_token(&credentials.token);
            Ok(())
        }

        AuthCommand::Logout => {
            let Some((credentials, _)) = store.load().await? else {
                store.delete().await?;
                println!("Not signed in.");
                return Ok(());
            };
            let oauth = with_base_url(credentials.oauth_client()?);
            let token = &credentials.token;
            for secret in std::iter::once(&token.access_token).chain(&token.refresh_token) {
                if let Err(e) = oauth.revoke(secret).await {
                    eprintln!("warning: could not revoke the token: {e}");
                }
            }
            store.delete().await?;
            println!("Signed out; the token was revoked and deleted.");
            Ok(())
        }
    }
}

/// Prints the non-secret details of a token.
fn print_token(token: &Token) {
    if let Some(uid) = &token.uid {
        println!("  user ID: {uid}");
    }
    let scopes = token.scopes();
    if !scopes.is_empty() {
        println!("  scopes: {}", scopes.join(", "));
    }
    if let Some(expires_at) = &token.expires_at {
        println!("  expires: {expires_at}");
    }
    println!("  access token: {}", mask(&token.access_token));
}

/// Shows only the start of a secret, enough to tell tokens apart.
fn mask(secret: &str) -> String {
    if secret.chars().count() > 16 {
        format!("{}…", secret.chars().take(8).collect::<String>())
    } else {
        "…".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::{assert, check};
    use wiremock::matchers::{body_string_contains, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TOKEN_JSON: &str = r#"{"access_token": "waka_tok_stored", "refresh_token": "waka_ref_stored", "scope": "read_stats,email", "uid": "u1"}"#;

    fn stored_credentials() -> StoredCredentials {
        StoredCredentials {
            client_id: "my-client-id".into(),
            client_secret: "my-client-secret".into(),
            redirect_uri: "http://127.0.0.1:8737/callback".into(),
            token: serde_json::from_str(TOKEN_JSON).unwrap(),
        }
    }

    /// A file store in a temp dir, optionally holding credentials.
    async fn file_store(with_credentials: bool) -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::new(Some(&dir.path().join("credentials.json")));
        if with_credentials {
            store.save(&stored_credentials()).await.unwrap();
        }
        (dir, store)
    }

    fn sources<'a>(
        bearer_token: Option<&'a str>,
        api_key: Option<&'a str>,
        api_key_explicit: bool,
    ) -> CredentialSources<'a> {
        CredentialSources {
            bearer_token,
            api_key,
            api_key_explicit,
        }
    }

    #[tokio::test]
    async fn bearer_token_wins_over_everything() {
        let (_dir, store) = file_store(true).await;
        let credential = Credential::resolve(sources(Some("bt"), Some("key"), true), &store)
            .await
            .unwrap();
        assert!(let Credential::BearerToken(token) = credential);
        check!(token == "bt");
    }

    #[tokio::test]
    async fn explicit_api_key_wins_over_stored_token() {
        let (_dir, store) = file_store(true).await;
        let credential = Credential::resolve(sources(None, Some("key"), true), &store)
            .await
            .unwrap();
        assert!(let Credential::ApiKey(key) = credential);
        check!(key == "key");
    }

    #[tokio::test]
    async fn stored_token_wins_over_api_key_from_environment() {
        let (_dir, store) = file_store(true).await;
        let credential = Credential::resolve(sources(None, Some("key"), false), &store)
            .await
            .unwrap();
        assert!(let Credential::OAuth(credentials, from) = credential);
        check!(credentials.token.access_token == "waka_tok_stored");
        check!(from == store);
    }

    #[tokio::test]
    async fn api_key_from_environment_is_the_last_resort() {
        let (_dir, store) = file_store(false).await;
        let credential = Credential::resolve(sources(None, Some("key"), false), &store)
            .await
            .unwrap();
        assert!(let Credential::ApiKey(key) = credential);
        check!(key == "key");
    }

    #[tokio::test]
    async fn missing_credentials_point_to_login() {
        let (_dir, store) = file_store(false).await;
        let result = Credential::resolve(CredentialSources::default(), &store).await;
        assert!(let Err(e) = result);
        check!(e.to_string().contains("waka-cli auth login"));
    }

    #[test]
    fn hints_depend_on_error_and_credential() {
        let api_key = Credential::ApiKey("key".into());
        let token = Credential::BearerToken("bt".into());
        let unauthorized: Box<dyn Error> = Box::new(ApiError::Unauthorized(None));
        let forbidden: Box<dyn Error> = Box::new(ApiError::Forbidden(None));
        let not_found: Box<dyn Error> = Box::new(ApiError::NotFound(None));
        let other: Box<dyn Error> = "boom".into();

        check!(
            api_key
                .hint(unauthorized.as_ref())
                .unwrap()
                .contains("write-only")
        );
        check!(
            api_key
                .hint(forbidden.as_ref())
                .unwrap()
                .contains("write-only")
        );
        check!(
            token
                .hint(unauthorized.as_ref())
                .unwrap()
                .contains("revoked")
        );
        check!(token.hint(forbidden.as_ref()).unwrap().contains("scope"));
        check!(token.hint(not_found.as_ref()).is_none());
        check!(token.hint(other.as_ref()).is_none());
    }

    #[test]
    fn mask_hides_secrets() {
        check!(mask("waka_tok_0123456789abcdef") == "waka_tok…");
        check!(mask("short") == "…");
        check!(mask("") == "…");
    }

    async fn mock_token_endpoint(server: &MockServer, body_contains: &str) {
        Mock::given(method("POST"))
            .and(path("/token"))
            .and(body_string_contains(body_contains))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"access_token": "waka_tok_new", "refresh_token": "waka_ref_new", "scope": "read_stats"}"#,
                "application/json",
            ))
            .expect(1)
            .mount(server)
            .await;
    }

    /// Free local port for the redirect listener.
    fn free_port() -> u16 {
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    }

    #[tokio::test]
    async fn login_via_loopback_redirect() {
        let server = MockServer::start().await;
        mock_token_endpoint(&server, "grant_type=authorization_code&code=the-code").await;
        let redirect_uri = format!("http://127.0.0.1:{}/callback", free_port());
        let oauth = OAuthClient::new("my-client-id", "my-client-secret", &redirect_uri)
            .unwrap()
            .with_base_url(server.uri());

        // Plays the browser: follows the authorization URL's redirect with the
        // code and the state it was given.
        let open_browser = |authorize_url: &str| {
            let authorize_url = Url::parse(authorize_url).unwrap();
            let state = authorize_url
                .query_pairs()
                .find(|(k, _)| k == "state")
                .unwrap()
                .1
                .into_owned();
            let callback = format!("{redirect_uri}?code=the-code&state={state}");
            tokio::spawn(async move {
                let target = Url::parse(&callback).unwrap();
                let mut stream =
                    tokio::net::TcpStream::connect(target.socket_addrs(|| None).unwrap()[0])
                        .await
                        .unwrap();
                let request = format!(
                    "GET {}?{} HTTP/1.1\r\nHost: localhost\r\n\r\n",
                    target.path(),
                    target.query().unwrap()
                );
                tokio::io::AsyncWriteExt::write_all(&mut stream, request.as_bytes())
                    .await
                    .unwrap();
            });
            true
        };

        let token = login(
            &oauth,
            LoginOptions {
                scopes: &[Scope::ReadStats],
                force_approve: false,
                no_browser: false,
                manual: false,
            },
            open_browser,
            || panic!("must not ask for input"),
        )
        .await
        .unwrap();

        check!(token.access_token == "waka_tok_new");
    }

    #[tokio::test]
    async fn login_rejects_mismatched_state() {
        let server = MockServer::start().await;
        let redirect_uri = format!("http://127.0.0.1:{}/callback", free_port());
        let oauth = OAuthClient::new("id", "secret", &redirect_uri)
            .unwrap()
            .with_base_url(server.uri());

        let result = login(
            &oauth,
            LoginOptions {
                scopes: &[],
                force_approve: false,
                no_browser: true,
                manual: true,
            },
            |_| panic!("must not open a browser"),
            || Ok("https://wakatime.com/oauth/test?code=c&state=forged".into()),
        )
        .await;

        assert!(let Err(e) = result);
        check!(e.to_string().contains("state parameter does not match"));
        // The code must not have been exchanged.
        check!(server.received_requests().await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn manual_login_accepts_a_bare_code() {
        let server = MockServer::start().await;
        mock_token_endpoint(&server, "code=pasted-code").await;
        let oauth = OAuthClient::new("id", "secret", "https://wakatime.com/oauth/test")
            .unwrap()
            .with_base_url(server.uri());

        // A non-local redirect URI implies manual mode.
        let token = login(
            &oauth,
            LoginOptions {
                scopes: &[Scope::ReadStats],
                force_approve: false,
                no_browser: true,
                manual: false,
            },
            |_| panic!("must not open a browser"),
            || Ok("pasted-code\n".into()),
        )
        .await
        .unwrap();

        check!(token.access_token == "waka_tok_new");
    }

    #[tokio::test]
    async fn refresh_replaces_token_and_keeps_credentials() {
        let server = MockServer::start().await;
        mock_token_endpoint(
            &server,
            "grant_type=refresh_token&refresh_token=waka_ref_stored",
        )
        .await;
        let (_dir, store) = file_store(true).await;

        run(
            AuthCommand::Refresh,
            CredentialSources::default(),
            &store,
            Some(&server.uri()),
        )
        .await
        .unwrap();

        let (credentials, _) = store.load().await.unwrap().unwrap();
        check!(credentials.token.access_token == "waka_tok_new");
        check!(credentials.token.refresh_token.as_deref() == Some("waka_ref_new"));
        check!(credentials.client_secret == "my-client-secret");
    }

    #[tokio::test]
    async fn refresh_without_login_fails() {
        let (_dir, store) = file_store(false).await;
        let result = run(
            AuthCommand::Refresh,
            CredentialSources::default(),
            &store,
            None,
        )
        .await;
        assert!(let Err(e) = result);
        check!(e.to_string().contains("not signed in"));
    }

    #[tokio::test]
    async fn logout_revokes_both_tokens_and_deletes() {
        let server = MockServer::start().await;
        for token in ["token=waka_tok_stored", "token=waka_ref_stored"] {
            Mock::given(method("POST"))
                .and(path("/revoke"))
                .and(body_string_contains(token))
                .respond_with(ResponseTemplate::new(200))
                .expect(1)
                .mount(&server)
                .await;
        }
        let (_dir, store) = file_store(true).await;

        run(
            AuthCommand::Logout,
            CredentialSources::default(),
            &store,
            Some(&server.uri()),
        )
        .await
        .unwrap();

        check!(store.load().await.unwrap().is_none());
    }

    #[tokio::test]
    async fn logout_deletes_even_if_revoking_fails() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/revoke"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let (_dir, store) = file_store(true).await;

        run(
            AuthCommand::Logout,
            CredentialSources::default(),
            &store,
            Some(&server.uri()),
        )
        .await
        .unwrap();

        check!(store.load().await.unwrap().is_none());
    }
}
