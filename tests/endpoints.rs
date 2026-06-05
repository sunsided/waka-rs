//! Mocked-HTTP tests for all client endpoints.

mod common;

use assert2::check;
use waka::{
    AllTimesSinceTodayOptions, CommitOptions, CommitsOptions, DurationsOptions, EditorsOptions,
    ExternalDurationsOptions, InsightsOptions, LeadersOptions, OrgDurationsOptions,
    OrgMemberSummariesOptions, OrgSummariesOptions, PrivateLeaderboardLeadersOptions,
    ProjectsOptions, StatsOptions, SummariesOptions,
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

#[tokio::test]
async fn goals_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/goals"))
        .respond_with(json_response(include_str!("fixtures/goals.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client.goals().await.expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].id == "goal-id-1");
    check!(result.data[0].delta.as_deref() == Some("day"));
    check!(result.data[0].seconds == Some(7200));
    let chart_data = result.data[0].chart_data.as_ref().expect("chart_data");
    check!(chart_data.len() == 1);
    check!(chart_data[0].range_status.as_deref() == Some("success"));
    check!(result.pagination.total == Some(1));
}

#[tokio::test]
async fn goal_returns_cached_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/goals/goal-id-1"))
        .respond_with(json_response(include_str!("fixtures/goal.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client.goal("goal-id-1").await.expect("request failed");

    check!(result.cached_at.as_deref() == Some("2026-06-05T08:00:00Z"));
    check!(result.data.id == "goal-id-1");
    check!(result.data.status.as_deref() == Some("success"));
}

#[tokio::test]
async fn insights_returns_data_with_payload() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/insights/weekdays/last_7_days"))
        .and(query_param("writes_only", "false"))
        .respond_with(json_response(include_str!("fixtures/insights.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .insights(
            "weekdays",
            "last_7_days",
            InsightsOptions {
                writes_only: Some(false),
                ..Default::default()
            },
        )
        .await
        .expect("request failed");

    check!(result.range.as_deref() == Some("last_7_days"));
    check!(result.status.as_deref() == Some("ok"));
    check!(result.is_up_to_date == Some(true));
    let weekdays = result.payload.get("weekdays").expect("weekdays payload");
    check!(weekdays.as_array().map(|a| a.len()) == Some(2));
}

#[tokio::test]
async fn leaders_returns_data_and_sends_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/leaders"))
        .and(query_param("language", "Rust"))
        .and(query_param("page", "1"))
        .respond_with(json_response(include_str!("fixtures/leaders.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .leaders(LeadersOptions {
            language: Some("Rust"),
            page: Some(1),
            ..Default::default()
        })
        .await
        .expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].rank == 1);
    check!(result.data[0].user.username.as_deref() == Some("topcoder"));
    let running_total = result.data[0]
        .running_total
        .as_ref()
        .expect("running_total");
    check!(running_total.total_seconds == 360000.0);
    let current_user = result.current_user.as_ref().expect("current_user");
    check!(current_user.rank == Some(42));
    check!(result.pagination.page == Some(1));
    check!(result.pagination.total_pages == Some(100));
}

#[tokio::test]
async fn machine_names_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/machine_names"))
        .respond_with(json_response(include_str!("fixtures/machine_names.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client.machine_names().await.expect("request failed");

    check!(result.data.len() == 2);
    check!(result.data[0].name.as_deref() == Some("work-laptop"));
    check!(result.data[1].name == None);
    check!(result.pagination.total == Some(2));
}

#[tokio::test]
async fn user_agents_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/user_agents"))
        .respond_with(json_response(include_str!("fixtures/user_agents.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client.user_agents().await.expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].editor.as_deref() == Some("vscode"));
    check!(result.data[0].os.as_deref() == Some("linux"));
    check!(result.data[0].is_browser_extension == Some(false));
}

#[tokio::test]
async fn status_bar_today_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/status_bar/today"))
        .respond_with(json_response(include_str!("fixtures/status_bar.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client.status_bar_today().await.expect("request failed");

    check!(result.cached_at.as_deref() == Some("2026-06-05T08:00:00Z"));
    check!(result.data.grand_total.total_seconds == 4500.0);
    check!(result.data.range.text == "Today");
    let languages = result.data.languages.as_ref().expect("languages");
    check!(languages[0].name == "Rust");
    check!(result.has_team_features == Some(false));
}

#[tokio::test]
async fn custom_rules_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/custom_rules"))
        .respond_with(json_response(include_str!("fixtures/custom_rules.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client.custom_rules().await.expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].action.as_deref() == Some("change"));
    let destinations = result.data[0].destinations.as_ref().expect("destinations");
    check!(destinations[0].destination_value.as_deref() == Some("new-project-name"));
    check!(result.job_id.as_deref() == Some("job-id-1"));
}

#[tokio::test]
async fn custom_rules_progress_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/custom_rules_progress"))
        .and(query_param("job_id", "job-id-1"))
        .respond_with(json_response(include_str!(
            "fixtures/custom_rules_progress.json"
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .custom_rules_progress("job-id-1")
        .await
        .expect("request failed");

    check!(result.progress == Some(80));
    check!(result.job_id.as_deref() == Some("job-id-1"));
}

#[tokio::test]
async fn data_dumps_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/data_dumps"))
        .respond_with(json_response(include_str!("fixtures/data_dumps.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client.data_dumps().await.expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].r#type.as_deref() == Some("heartbeats"));
    check!(result.data[0].percent_complete == Some(100.0));
    check!(result.data[0].has_failed == Some(false));
}

#[tokio::test]
async fn editors_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/editors"))
        .and(query_param("unreleased", "true"))
        .respond_with(json_response(include_str!("fixtures/editors.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .editors(EditorsOptions {
            unreleased: Some(true),
        })
        .await
        .expect("request failed");

    check!(result.data.len() == 2);
    check!(result.data[0].id == "adobe-xd");
    check!(result.data[0].released == Some(true));
}

#[tokio::test]
async fn external_durations_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/external_durations"))
        .and(query_param("date", "2026-06-04"))
        .respond_with(json_response(include_str!(
            "fixtures/external_durations.json"
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .external_durations("2026-06-04", ExternalDurationsOptions::default())
        .await
        .expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].provider.as_deref() == Some("google_calendar"));
    check!(result.data[0].category.as_deref() == Some("meeting"));
}

#[tokio::test]
async fn meta_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/meta"))
        .respond_with(json_response(include_str!("fixtures/meta.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client.meta().await.expect("request failed");

    let ips = result.ips.expect("ips");
    let api = ips.api.expect("api ips");
    check!(api.v4.map(|v| v.len()) == Some(2));
    check!(result.last_modified_at.is_some());
}

#[tokio::test]
async fn private_leaderboards_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/leaderboards"))
        .respond_with(json_response(include_str!(
            "fixtures/private_leaderboards.json"
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client.private_leaderboards().await.expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].name.as_deref() == Some("Team Leaderboard"));
    check!(result.data[0].members_count == Some(5));
    check!(result.pagination.total == Some(1));
}

#[tokio::test]
async fn private_leaderboard_leaders_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/leaderboards/board-id-1"))
        .and(query_param("page", "1"))
        .respond_with(json_response(include_str!("fixtures/leaders.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .private_leaderboard_leaders(
            "board-id-1",
            PrivateLeaderboardLeadersOptions {
                page: Some(1),
                ..Default::default()
            },
        )
        .await
        .expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].rank == 1);
}

#[tokio::test]
async fn program_languages_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/program_languages"))
        .respond_with(json_response(include_str!(
            "fixtures/program_languages.json"
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client.program_languages().await.expect("request failed");

    check!(result.data.len() == 2);
    check!(result.data[0].is_verified == Some(true));
    check!(result.data[0].extensions.as_ref().map(|e| e.len()) == Some(2));
}

#[tokio::test]
async fn stats_aggregated_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/stats/last_7_days"))
        .respond_with(json_response(include_str!(
            "fixtures/stats_aggregated.json"
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .stats_aggregated("last_7_days")
        .await
        .expect("request failed");

    let total = result.total.expect("total");
    check!(total.average.expect("average").seconds.is_some());
    check!(total.count.expect("count").text.is_some());
    let languages = result.languages.expect("languages");
    check!(languages.len() == 2);
    check!(languages[0].measures.median.is_some());
}

#[tokio::test]
async fn orgs_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/orgs"))
        .respond_with(json_response(include_str!("fixtures/orgs.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client.orgs().await.expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].name.as_deref() == Some("Acme Corp"));
    check!(result.data[0].people_count == Some(10));
    check!(result.data[0].can_current_user_list_dashboards == Some(true));
}

#[tokio::test]
async fn org_custom_rules_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/orgs/org-id-1/custom_rules"))
        .respond_with(json_response(include_str!(
            "fixtures/org_custom_rules.json"
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .org_custom_rules("org-id-1")
        .await
        .expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].operation.as_deref() == Some("starts with"));
    let destinations = result.data[0].destinations.as_ref().expect("destinations");
    check!(destinations[0].destination_value.as_deref() == Some("scratch"));
}

#[tokio::test]
async fn org_dashboards_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/users/current/orgs/org-id-1/dashboards"))
        .respond_with(json_response(include_str!("fixtures/org_dashboards.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .org_dashboards("org-id-1")
        .await
        .expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].full_name.as_deref() == Some("Backend Team"));
    check!(result.data[0].members_count == Some(5));
    check!(result.next_page == None);
    check!(result.pagination.page == Some(1));
}

#[tokio::test]
async fn org_dashboard_members_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/users/current/orgs/org-id-1/dashboards/dashboard-id-1/members",
        ))
        .respond_with(json_response(include_str!(
            "fixtures/org_dashboard_members.json"
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .org_dashboard_members("org-id-1", "dashboard-id-1")
        .await
        .expect("request failed");

    check!(result.data.len() == 2);
    check!(result.data[0].is_view_only == Some(false));
    check!(result.data[1].is_view_only == Some(true));
    check!(result.pagination.total == Some(2));
}

#[tokio::test]
async fn org_dashboard_durations_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/users/current/orgs/org-id-1/dashboards/dashboard-id-1/durations",
        ))
        .and(query_param("date", "2026-06-04"))
        .and(query_param("slice_by", "language"))
        .respond_with(json_response(include_str!(
            "fixtures/org_dashboard_durations.json"
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .org_dashboard_durations(
            "org-id-1",
            "dashboard-id-1",
            "2026-06-04",
            OrgDurationsOptions {
                slice_by: Some("language"),
                ..Default::default()
            },
        )
        .await
        .expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].member.id == "user-id-1");
    check!(result.data[0].durations.len() == 1);
    check!(result.data[0].durations[0].duration == 1800.0);
}

#[tokio::test]
async fn org_dashboard_summaries_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/users/current/orgs/org-id-1/dashboards/dashboard-id-1/summaries",
        ))
        .and(query_param("date", "2026-06-04"))
        .respond_with(json_response(include_str!(
            "fixtures/org_dashboard_summaries.json"
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .org_dashboard_summaries(
            "org-id-1",
            "dashboard-id-1",
            "2026-06-04",
            OrgSummariesOptions::default(),
        )
        .await
        .expect("request failed");

    check!(result.data.len() == 1);
    let member = result.data[0].member.as_ref().expect("member");
    check!(member.username.as_deref() == Some("testuser"));
    check!(result.data[0].grand_total.total_seconds == 9000.0);
    let cumulative_total = result.cumulative_total.expect("cumulative_total");
    check!(cumulative_total.seconds == 9000.0);
}

#[tokio::test]
async fn org_dashboard_member_durations_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/users/current/orgs/org-id-1/dashboards/dashboard-id-1/members/user-id-1/durations",
        ))
        .and(query_param("date", "2026-06-04"))
        .respond_with(json_response(include_str!("fixtures/durations.json")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .org_dashboard_member_durations(
            "org-id-1",
            "dashboard-id-1",
            "user-id-1",
            "2026-06-04",
            OrgDurationsOptions::default(),
        )
        .await
        .expect("request failed");

    check!(result.data.len() == 2);
    check!(result.data[0].project.as_deref() == Some("waka-rs"));
}

#[tokio::test]
async fn org_dashboard_member_summaries_returns_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/users/current/orgs/org-id-1/dashboards/dashboard-id-1/members/user-id-1/summaries",
        ))
        .and(query_param("start", "2026-06-04"))
        .and(query_param("end", "2026-06-05"))
        .respond_with(json_response(include_str!(
            "fixtures/org_dashboard_member_summaries.json"
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .org_dashboard_member_summaries(
            "org-id-1",
            "dashboard-id-1",
            "user-id-1",
            "2026-06-04",
            "2026-06-05",
            OrgMemberSummariesOptions::default(),
        )
        .await
        .expect("request failed");

    check!(result.data.len() == 1);
    check!(result.data[0].member.is_none());
    check!(result.default_personal_privacy.as_deref() == Some("visible"));
    let daily_average = result.daily_average.expect("daily_average");
    check!(daily_average.days_including_holidays == 1);
}
