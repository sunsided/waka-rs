//! Error-path tests for the status-code dispatch.

mod common;

use assert2::{assert, check};
use waka::{ApiError, ProjectsOptions};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn mock_projects_status(server: &MockServer, response: ResponseTemplate) {
    Mock::given(method("GET"))
        .and(path("/users/current/projects"))
        .respond_with(response)
        .expect(1)
        .mount(server)
        .await;
}

#[tokio::test]
async fn returns_unauthorized_on_401() {
    let server = MockServer::start().await;
    mock_projects_status(
        &server,
        ResponseTemplate::new(401)
            .set_body_raw(include_str!("fixtures/error_401.json"), "application/json"),
    )
    .await;

    let client = common::client_for(&server);
    let result = client.projects(ProjectsOptions::default()).await;

    assert!(let Err(ApiError::Unauthorized(Some(errors))) = result);
    check!(errors.errors == vec!["Unauthorized.".to_string()]);
}

#[tokio::test]
async fn returns_payment_required_on_402() {
    let server = MockServer::start().await;
    mock_projects_status(&server, ResponseTemplate::new(402)).await;

    let client = common::client_for(&server);
    let result = client.projects(ProjectsOptions::default()).await;

    assert!(let Err(ApiError::PaymentRequired(_)) = result);
}

#[tokio::test]
async fn returns_forbidden_on_403() {
    let server = MockServer::start().await;
    mock_projects_status(&server, ResponseTemplate::new(403)).await;

    let client = common::client_for(&server);
    let result = client.projects(ProjectsOptions::default()).await;

    assert!(let Err(ApiError::Forbidden(_)) = result);
}

#[tokio::test]
async fn returns_not_found_on_404() {
    let server = MockServer::start().await;
    mock_projects_status(&server, ResponseTemplate::new(404)).await;

    let client = common::client_for(&server);
    let result = client.projects(ProjectsOptions::default()).await;

    assert!(let Err(ApiError::NotFound(_)) = result);
}

#[tokio::test]
async fn parses_singular_error_body_on_404() {
    // Some endpoints return {"error": "..."} instead of {"errors": [...]},
    // as observed live on /users/current/orgs.
    let server = MockServer::start().await;
    mock_projects_status(
        &server,
        ResponseTemplate::new(404).set_body_raw(r#"{"error": "Not found."}"#, "application/json"),
    )
    .await;

    let client = common::client_for(&server);
    let result = client.projects(ProjectsOptions::default()).await;

    assert!(let Err(ApiError::NotFound(Some(errors))) = result);
    check!(errors.error.as_deref() == Some("Not found."));
    check!(errors.errors.is_empty());
}

#[tokio::test]
async fn returns_rate_limited_on_429_with_retry_after() {
    let server = MockServer::start().await;
    mock_projects_status(
        &server,
        ResponseTemplate::new(429)
            .insert_header("retry-after", "30")
            .set_body_raw(include_str!("fixtures/error_429.json"), "application/json"),
    )
    .await;

    let client = common::client_for(&server);
    let result = client.projects(ProjectsOptions::default()).await;

    assert!(let
        Err(ApiError::RateLimited {
            retry_after,
            errors
        }) = result
    );
    check!(retry_after == Some(30));
    assert!(let Some(errors) = errors);
    check!(errors.errors == vec!["Too many requests. Please slow down.".to_string()]);
}

#[tokio::test]
async fn returns_unspecified_on_unhandled_status() {
    let server = MockServer::start().await;
    mock_projects_status(&server, ResponseTemplate::new(503)).await;

    let client = common::client_for(&server);
    let result = client.projects(ProjectsOptions::default()).await;

    assert!(let Err(ApiError::Unspecified(503, _)) = result);
}

#[tokio::test]
async fn returns_invalid_format_on_unexpected_body() {
    let server = MockServer::start().await;
    mock_projects_status(
        &server,
        ResponseTemplate::new(200).set_body_raw(r#"{"unexpected": true}"#, "application/json"),
    )
    .await;

    let client = common::client_for(&server);
    let result = client.projects(ProjectsOptions::default()).await;

    assert!(let Err(ApiError::InvalidFormat(_)) = result);
}
