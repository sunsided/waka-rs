//! Tests for the OAuth 2.0 authorization code flow.

use assert2::{assert, check};
use waka::WakaTimeClientBuilder;
use waka::oauth::{AuthorizeOptions, OAuthClient, OAuthError, Scope, Token, UnknownScope};
use wiremock::matchers::{body_string, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const REDIRECT_URI: &str = "http://127.0.0.1:8737/callback";

fn oauth_client_for(server: &MockServer) -> OAuthClient {
    OAuthClient::new("my-client-id", "my-client-secret", REDIRECT_URI)
        .expect("failed to build OAuth client")
        .with_base_url(server.uri())
}

fn token_response() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_raw(
        include_str!("fixtures/oauth_token.json"),
        "application/json",
    )
}

#[test]
fn scope_round_trips_through_display_and_from_str() {
    for scope in Scope::ALL {
        check!(scope.to_string().parse::<Scope>() == Ok(*scope));
    }
    check!(Scope::ReadStatsBestDay.to_string() == "read_stats.best_day");
    check!(Scope::ReadSummariesOperatingSystems.to_string() == "read_summaries.operating_systems");
    check!(Scope::Email.to_string() == "email");
}

#[test]
fn scope_rejects_unknown_names() {
    check!("read_everything".parse::<Scope>() == Err(UnknownScope("read_everything".into())));
}

#[test]
fn authorize_url_contains_all_parameters() {
    let client = OAuthClient::new("my-client-id", "my-client-secret", REDIRECT_URI)
        .expect("failed to build OAuth client");

    let url = client.authorize_url(AuthorizeOptions {
        scopes: &[Scope::ReadStats, Scope::ReadSummaries, Scope::Email],
        state: Some("s3cr3t-state"),
        force_approve: true,
    });

    check!(
        url == "https://wakatime.com/oauth/authorize?client_id=my-client-id&response_type=code\
                &redirect_uri=http://127.0.0.1:8737/callback&scope=read_stats,read_summaries,email\
                &state=s3cr3t-state&force_approve=true"
    );
    check!(!url.contains("my-client-secret"));
}

#[test]
fn authorize_url_omits_unset_options() {
    let client = OAuthClient::new("id", "secret", "https://example.com/cb?x=1&y=2")
        .expect("failed to build OAuth client")
        .with_base_url("http://localhost:1234/oauth");

    let url = client.authorize_url(AuthorizeOptions::default());

    check!(
        url == "http://localhost:1234/oauth/authorize?client_id=id&response_type=code\
                &redirect_uri=https://example.com/cb?x%3D1%26y%3D2"
    );
}

#[test]
fn oauth_client_debug_redacts_secret() {
    let client = OAuthClient::new("my-client-id", "my-client-secret", REDIRECT_URI)
        .expect("failed to build OAuth client");

    let debug = format!("{client:?}");
    check!(debug.contains("my-client-id"));
    check!(!debug.contains("my-client-secret"));
}

#[tokio::test]
async fn exchange_code_posts_form_and_parses_token() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .and(header("accept", "application/json"))
        .and(header("content-type", "application/x-www-form-urlencoded"))
        .and(body_string(
            "client_id=my-client-id&client_secret=my-client-secret\
             &redirect_uri=http%3A%2F%2F127.0.0.1%3A8737%2Fcallback\
             &grant_type=authorization_code&code=the-code",
        ))
        .respond_with(token_response())
        .expect(1)
        .mount(&server)
        .await;

    let token = oauth_client_for(&server)
        .exchange_code("the-code")
        .await
        .expect("exchange failed");

    check!(token.access_token == "waka_tok_access");
    check!(token.refresh_token.as_deref() == Some("waka_ref_refresh"));
    check!(token.token_type.as_deref() == Some("bearer"));
    check!(token.uid.as_deref() == Some("5b3e4a8f-0000-4000-8000-000000000000"));
    check!(token.expires_in == Some(3_153_600_000));
    check!(token.expires_at.as_deref() == Some("2126-09-20T10:06:17Z"));
    check!(token.scopes() == vec!["email", "read_stats", "read_summaries"]);
    check!(token.has_scope(Scope::ReadStats));
    check!(!token.has_scope(Scope::WriteHeartbeats));
}

#[tokio::test]
async fn refresh_posts_refresh_grant() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .and(body_string(
            "client_id=my-client-id&client_secret=my-client-secret\
             &redirect_uri=http%3A%2F%2F127.0.0.1%3A8737%2Fcallback\
             &grant_type=refresh_token&refresh_token=waka_ref_old",
        ))
        .respond_with(token_response())
        .expect(1)
        .mount(&server)
        .await;

    let token = oauth_client_for(&server)
        .refresh("waka_ref_old")
        .await
        .expect("refresh failed");

    check!(token.access_token == "waka_tok_access");
}

#[tokio::test]
async fn revoke_posts_token() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/revoke"))
        .and(body_string(
            "client_id=my-client-id&client_secret=my-client-secret&token=waka_tok_access",
        ))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let result = oauth_client_for(&server).revoke("waka_tok_access").await;

    check!(result.is_ok());
}

#[tokio::test]
async fn token_errors_map_to_rejected() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(400).set_body_raw(
            r#"{"error": "invalid_grant", "error_description": "Invalid code."}"#,
            "application/json",
        ))
        .expect(1)
        .mount(&server)
        .await;

    let result = oauth_client_for(&server).exchange_code("bad-code").await;

    assert!(let Err(err @ OAuthError::Rejected { .. }) = result);
    let OAuthError::Rejected {
        status,
        error,
        description,
    } = &err
    else {
        unreachable!()
    };
    check!(*status == 400);
    check!(error.as_deref() == Some("invalid_grant"));
    check!(description.as_deref() == Some("Invalid code."));
    check!(err.to_string() == "The OAuth request was rejected (400): invalid_grant: Invalid code.");
}

#[tokio::test]
async fn token_errors_accept_errors_list_and_empty_bodies() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(401).set_body_raw(
            r#"{"errors": ["Invalid client_secret."]}"#,
            "application/json",
        ))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/revoke"))
        .respond_with(ResponseTemplate::new(500).set_body_string("oops"))
        .mount(&server)
        .await;

    let client = oauth_client_for(&server);

    let result = client.refresh("waka_ref_old").await;
    assert!(let Err(OAuthError::Rejected { status: 401, error: None, description: Some(description) }) = result);
    check!(description == "Invalid client_secret.");

    let result = client.revoke("waka_tok_access").await;
    assert!(let Err(OAuthError::Rejected { status: 500, error: None, description: None }) = result);
}

#[tokio::test]
async fn malformed_token_response_is_invalid_response() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(r#"{"token": "nope"}"#, "application/json"),
        )
        .mount(&server)
        .await;

    let result = oauth_client_for(&server).exchange_code("the-code").await;

    assert!(let Err(OAuthError::InvalidResponse(_)) = result);
}

#[test]
fn token_tolerates_minimal_and_unusual_responses() {
    let token: Token = serde_json::from_str(
        r#"{"access_token": "a", "expires_in": "43200.5", "scope": "read_stats read_goals", "foo": 1}"#,
    )
    .expect("failed to parse token");

    check!(token.access_token == "a");
    check!(token.refresh_token.is_none());
    check!(token.expires_in == Some(43200));
    check!(token.scopes() == vec!["read_stats", "read_goals"]);
    check!(token.extra.get("foo") == Some(&serde_json::json!(1)));

    let token: Token = serde_json::from_str(r#"{"access_token": "a", "expires_in": 12.9}"#)
        .expect("failed to parse token");
    check!(token.expires_in == Some(12));
    check!(token.scopes().is_empty());
}

#[test]
fn token_round_trips_through_serde() {
    let token: Token = serde_json::from_str(include_str!("fixtures/oauth_token.json"))
        .expect("failed to parse token");

    let json = serde_json::to_string(&token).expect("failed to serialize token");
    let again: Token = serde_json::from_str(&json).expect("failed to parse token again");

    check!(serde_json::to_value(&again).unwrap() == serde_json::to_value(&token).unwrap());
    check!(again.extra.is_empty());
}

#[test]
fn token_debug_redacts_secrets() {
    let token: Token = serde_json::from_str(include_str!("fixtures/oauth_token.json"))
        .expect("failed to parse token");

    let debug = format!("{token:?}");
    check!(!debug.contains("waka_tok_access"));
    check!(!debug.contains("waka_ref_refresh"));
    check!(debug.contains("read_stats"));
}

#[tokio::test]
async fn oauth_token_authenticates_api_client() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/user_agents"))
        .and(header("authorization", "Bearer waka_tok_access"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            include_str!("fixtures/user_agents.json"),
            "application/json",
        ))
        .expect(1)
        .mount(&server)
        .await;

    let token: Token = serde_json::from_str(include_str!("fixtures/oauth_token.json"))
        .expect("failed to parse token");
    let client = WakaTimeClientBuilder::new_with_oauth_token(&token)
        .with_base_url(server.uri())
        .build()
        .expect("failed to build client");

    let result = client.user_agents().await.expect("request failed");
    check!(result.data.len() == 1);
}
