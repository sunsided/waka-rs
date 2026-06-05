//! Tests for builder options, typed parameters, and pagination helpers.

mod common;

use assert2::check;
use waka::{InsightType, ProjectsOptions, Range, StatsOptions, WakaTimeClientBuilder};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn json_response(body: &'static str) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_raw(body, "application/json")
}

#[test]
fn range_displays_as_api_values() {
    check!(Range::Last7Days.to_string() == "last_7_days");
    check!(Range::Last30Days.to_string() == "last_30_days");
    check!(Range::Last6Months.to_string() == "last_6_months");
    check!(Range::LastYear.to_string() == "last_year");
    check!(Range::AllTime.to_string() == "all_time");
    check!(Range::Year(2024).to_string() == "2024");
}

#[test]
fn insight_type_displays_as_api_values() {
    check!(InsightType::Weekdays.to_string() == "weekdays");
    check!(InsightType::BestDay.to_string() == "best_day");
    check!(InsightType::OperatingSystems.to_string() == "operating_systems");
}

#[tokio::test]
async fn stats_accepts_range_enum() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/stats/last_30_days"))
        .respond_with(json_response(include_str!("fixtures/stats.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .stats(Range::Last30Days, StatsOptions::default())
        .await
        .expect("request failed");

    check!(result.status == "ok");
}

#[tokio::test]
async fn bearer_token_sets_authorization_header() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/user_agents"))
        .and(header("authorization", "Bearer my-oauth-token"))
        .respond_with(json_response(include_str!("fixtures/user_agents.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = WakaTimeClientBuilder::new_with_bearer_token("my-oauth-token")
        .with_base_url(server.uri())
        .build()
        .expect("failed to build client");

    let result = client.user_agents().await.expect("request failed");
    check!(result.data.len() == 1);
}

#[tokio::test]
async fn timeout_applies_to_requests() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/user_agents"))
        .respond_with(
            json_response(include_str!("fixtures/user_agents.json"))
                .set_delay(std::time::Duration::from_secs(5)),
        )
        .mount(&server)
        .await;

    let client = WakaTimeClientBuilder::new_with_api_key("test-api-key")
        .with_base_url(server.uri())
        .with_timeout(std::time::Duration::from_millis(100))
        .build()
        .expect("failed to build client");

    let result = client.user_agents().await;
    assert2::assert!(let Err(waka::ApiError::Timeout(_)) = result);
}

#[tokio::test]
async fn projects_all_fetches_every_page() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/projects"))
        .and(query_param("page", "1"))
        .respond_with(json_response(
            r#"{"data": [{"id": "project-id-1", "name": "one"}], "page": 1, "total_pages": 2, "total": 2}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/users/current/projects"))
        .and(query_param("page", "2"))
        .respond_with(json_response(
            r#"{"data": [{"id": "project-id-2", "name": "two"}], "page": 2, "total_pages": 2, "total": 2}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client.projects_all(None).await.expect("request failed");

    check!(result.len() == 2);
    check!(result[0].name == "one");
    check!(result[1].name == "two");
}

#[tokio::test]
async fn projects_single_page_still_works() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/projects"))
        .respond_with(json_response(include_str!("fixtures/projects.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .projects(ProjectsOptions::default())
        .await
        .expect("request failed");
    check!(result.data.len() == 1);
}
