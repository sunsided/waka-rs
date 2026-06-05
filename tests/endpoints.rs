//! Mocked-HTTP tests for all client endpoints.

mod common;

use assert2::check;
use waka::{
    AllTimesSinceTodayOptions, CommitOptions, CommitsOptions, DurationsOptions, ProjectsOptions,
    StatsOptions, SummariesOptions,
};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn json_response(body: &'static str) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_raw(body, "application/json")
}

#[tokio::test]
async fn all_time_since_today_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/all_time_since_today"))
        .and(header("authorization", "Basic dGVzdC1hcGkta2V5"))
        .respond_with(json_response(include_str!(
            "fixtures/all_time_since_today.json"
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .all_time_since_today(AllTimesSinceTodayOptions::default())
        .await
        .expect("request failed");

    check!(result.is_up_to_date == true);
    check!(result.timeout == 15);
    check!(result.total_seconds == 5401800.0);
    check!(result.range.timezone == "Europe/Berlin");
}

#[tokio::test]
async fn summaries_returns_data_and_sends_query_params() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/summaries"))
        .and(query_param("start", "2026-06-04"))
        .and(query_param("end", "2026-06-05"))
        .and(query_param("project", "waka-rs"))
        .respond_with(json_response(include_str!("fixtures/summaries.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .summaries(
            "2026-06-04",
            "2026-06-05",
            SummariesOptions {
                project: Some("waka-rs"),
                ..Default::default()
            },
        )
        .await
        .expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].grand_total.total_seconds == 9000.0);
    check!(result.cumulative_total.decimal == "2.50");
    check!(result.daily_average.days_including_holidays == 1);
}

#[tokio::test]
async fn commit_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/users/current/projects/waka-rs/commits/0123456789abcdef0123456789abcdef01234567",
        ))
        .respond_with(json_response(include_str!("fixtures/commit.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .commit(
            "waka-rs",
            "0123456789abcdef0123456789abcdef01234567",
            CommitOptions::default(),
        )
        .await
        .expect("request failed");

    check!(result.branch == "main");
    check!(result.commit.truncated_hash == "0123456789");
    check!(result.project.name == "waka-rs");
}

#[tokio::test]
async fn commits_list_returns_page() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/projects/waka-rs/commits"))
        .and(query_param("page", "1"))
        .respond_with(json_response(include_str!("fixtures/commits.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .commits(
            "waka-rs",
            CommitsOptions {
                page: Some(1),
                ..Default::default()
            },
        )
        .await
        .expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].message == "Add new feature");
    check!(result.branch.as_deref() == Some("main"));
    check!(result.pagination.page == Some(1));
    check!(result.pagination.total_pages == Some(1));
    check!(result.pagination.total == Some(1));
}

#[tokio::test]
async fn stats_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/stats/last_7_days"))
        .and(query_param("writes_only", "true"))
        .respond_with(json_response(include_str!("fixtures/stats.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .stats(
            "last_7_days",
            StatsOptions {
                writes_only: Some(true),
                ..Default::default()
            },
        )
        .await
        .expect("request failed");

    check!(result.status == "ok");
    check!(result.range == "last_7_days");
    check!(result.total_seconds == Some(86400.5));
    let languages = result.languages.expect("languages missing");
    check!(languages.len() == 1);
    check!(languages[0].name == "Rust");
    let best_day = result.best_day.expect("best_day missing");
    check!(best_day.date == "2026-06-01");
}

#[tokio::test]
async fn projects_returns_page_and_sends_filter() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/projects"))
        .and(query_param("q", "waka"))
        .respond_with(json_response(include_str!("fixtures/projects.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .projects(ProjectsOptions {
            q: Some("waka"),
            ..Default::default()
        })
        .await
        .expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].name == "waka-rs");
    check!(result.pagination.page == Some(1));
    check!(result.pagination.total == Some(1));
}

#[tokio::test]
async fn durations_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/durations"))
        .and(query_param("date", "2026-06-04"))
        .respond_with(json_response(include_str!("fixtures/durations.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .durations("2026-06-04", DurationsOptions::default())
        .await
        .expect("request failed");

    check!(result.data.len() == 2);
    check!(result.data[0].project.as_deref() == Some("waka-rs"));
    check!(result.data[0].duration == 1800.0);
    check!(result.timezone == "Europe/Berlin");
}

#[tokio::test]
async fn heartbeats_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/heartbeats"))
        .and(query_param("date", "2026-06-04"))
        .respond_with(json_response(include_str!("fixtures/heartbeats.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .heartbeats("2026-06-04")
        .await
        .expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].entity == "/home/user/project/src/lib.rs");
    check!(result.data[0].r#type == "file");
    check!(result.data[0].is_write == Some(true));
}

#[tokio::test]
async fn user_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current"))
        .respond_with(json_response(include_str!("fixtures/user.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client.user().await.expect("request failed");

    check!(result.id == "user-id-1");
    check!(result.username.as_deref() == Some("testuser"));
    check!(result.plan.as_deref() == Some("free"));
}

#[tokio::test]
async fn custom_user_is_used_in_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/somebody/durations"))
        .and(query_param("date", "2026-06-04"))
        .respond_with(json_response(include_str!("fixtures/durations.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = waka::WakaTimeClientBuilder::new_with_api_key("test-api-key")
        .with_user("somebody")
        .with_base_url(server.uri())
        .build()
        .expect("failed to build client");

    let result = client
        .durations("2026-06-04", DurationsOptions::default())
        .await
        .expect("request failed");

    check!(result.data.len() == 2);
}
