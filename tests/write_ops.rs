//! Mocked-HTTP tests for the write endpoints.
//!
//! These only ever talk to a local wiremock server, never the real API.

mod common;

use assert2::check;
use waka::model::{CustomRuleInput, ExternalDurationInput, HeartbeatInput};
use wiremock::matchers::{body_json, body_partial_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn json_response(status: u16, body: &'static str) -> ResponseTemplate {
    ResponseTemplate::new(status).set_body_raw(body, "application/json")
}

#[tokio::test]
async fn send_heartbeat_posts_body_and_parses_201() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/users/current/heartbeats"))
        .and(body_partial_json(serde_json::json!({
            "entity": "/home/user/project/src/lib.rs",
            "type": "file",
            "time": 1717571200.0,
            "is_write": true
        })))
        .respond_with(json_response(
            201,
            r#"{"data": {"id": "heartbeat-id-1", "entity": "/home/user/project/src/lib.rs", "type": "file", "time": 1717571200.0}}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .send_heartbeat(&HeartbeatInput {
            entity: "/home/user/project/src/lib.rs".to_string(),
            r#type: "file".to_string(),
            time: 1717571200.0,
            is_write: Some(true),
            ..Default::default()
        })
        .await
        .expect("request failed");

    check!(result.id == "heartbeat-id-1");
    check!(result.time == Some(1717571200.0));
}

#[tokio::test]
async fn send_heartbeat_omits_unset_optional_fields() {
    let server = MockServer::start().await;
    // Exact body match: optional None fields must not be serialized.
    Mock::given(method("POST"))
        .and(path("/users/current/heartbeats"))
        .and(body_json(serde_json::json!({
            "entity": "example.com",
            "type": "domain",
            "time": 1717571200.0
        })))
        .respond_with(json_response(202, r#"{"data": {"id": "heartbeat-id-2"}}"#))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .send_heartbeat(&HeartbeatInput {
            entity: "example.com".to_string(),
            r#type: "domain".to_string(),
            time: 1717571200.0,
            ..Default::default()
        })
        .await
        .expect("request failed");

    check!(result.id == "heartbeat-id-2");
}

#[tokio::test]
async fn send_heartbeats_bulk_posts_array() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/users/current/heartbeats.bulk"))
        .respond_with(json_response(
            202,
            r#"{"responses": [[{"data": {"id": "heartbeat-id-1"}}, 201], [{"data": {"id": "heartbeat-id-2"}}, 201]]}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let heartbeats = vec![
        HeartbeatInput {
            entity: "a.rs".to_string(),
            r#type: "file".to_string(),
            time: 1717571200.0,
            ..Default::default()
        },
        HeartbeatInput {
            entity: "b.rs".to_string(),
            r#type: "file".to_string(),
            time: 1717571300.0,
            ..Default::default()
        },
    ];
    let result = client
        .send_heartbeats(&heartbeats)
        .await
        .expect("request failed");

    let responses = result.get("responses").expect("responses");
    check!(responses.as_array().map(|a| a.len()) == Some(2));
}

#[tokio::test]
async fn delete_heartbeats_sends_date_and_ids() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/users/current/heartbeats.bulk"))
        .and(body_json(serde_json::json!({
            "date": "2026-06-04",
            "ids": ["heartbeat-id-1", "heartbeat-id-2"]
        })))
        .respond_with(json_response(200, r#"{"data": {}}"#))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    client
        .delete_heartbeats("2026-06-04", &["heartbeat-id-1", "heartbeat-id-2"])
        .await
        .expect("request failed");
}

#[tokio::test]
async fn send_external_duration_posts_body_and_parses_201() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/users/current/external_durations"))
        .and(body_partial_json(serde_json::json!({
            "external_id": "meeting-123",
            "entity": "Sprint Planning",
            "type": "event"
        })))
        .respond_with(json_response(
            201,
            r#"{"data": {"id": "ext-duration-id-1", "external_id": "meeting-123", "entity": "Sprint Planning", "type": "event", "provider": "my_app", "category": "meeting", "start_time": 1717571200.0, "end_time": 1717574800.0}}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .send_external_duration(&ExternalDurationInput {
            external_id: "meeting-123".to_string(),
            entity: "Sprint Planning".to_string(),
            r#type: "event".to_string(),
            start_time: 1717571200.0,
            end_time: 1717574800.0,
            category: Some("meeting".to_string()),
            ..Default::default()
        })
        .await
        .expect("request failed");

    check!(result.id == "ext-duration-id-1");
    check!(result.provider.as_deref() == Some("my_app"));
}

#[tokio::test]
async fn delete_external_durations_sends_date_and_ids() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/users/current/external_durations.bulk"))
        .and(body_json(serde_json::json!({
            "date": "2026-06-04",
            "ids": ["ext-duration-id-1"]
        })))
        .respond_with(json_response(200, r#"{"data": {}}"#))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    client
        .delete_external_durations("2026-06-04", &["ext-duration-id-1"])
        .await
        .expect("request failed");
}

#[tokio::test]
async fn create_data_dump_posts_type() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/users/current/data_dumps"))
        .and(body_json(serde_json::json!({
            "type": "heartbeats",
            "email_when_finished": false
        })))
        .respond_with(json_response(
            201,
            r#"{"data": {"id": "dump-id-1", "type": "heartbeats", "status": "Pending…", "percent_complete": 0.0, "is_processing": false, "is_stuck": false, "has_failed": false}}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .create_data_dump("heartbeats", Some(false))
        .await
        .expect("request failed");

    check!(result.id == "dump-id-1");
    check!(result.percent_complete == Some(0.0));
}

#[tokio::test]
async fn set_custom_rules_puts_rules() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/users/current/custom_rules"))
        .and(body_json(serde_json::json!([{
            "action": "change",
            "source": "project",
            "operation": "equals",
            "source_value": "old-name",
            "destination": "project",
            "destination_value": "new-name",
            "priority": 1
        }])))
        .respond_with(json_response(
            200,
            r#"{"data": {"changes": {"added": [{}], "removed": [], "rearranged": []}, "job_id": "job-id-1"}}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    let result = client
        .set_custom_rules(&[CustomRuleInput {
            action: "change".to_string(),
            source: "project".to_string(),
            operation: "equals".to_string(),
            source_value: "old-name".to_string(),
            destination: "project".to_string(),
            destination_value: "new-name".to_string(),
            priority: 1,
            ..Default::default()
        }])
        .await
        .expect("request failed");

    check!(result.job_id.as_deref() == Some("job-id-1"));
    let changes = result.changes.expect("changes");
    check!(changes.added.map(|a| a.len()) == Some(1));
}

#[tokio::test]
async fn delete_custom_rule_uses_rule_path() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/users/current/custom_rules/rule-id-1"))
        .respond_with(json_response(200, r#"{"data": {}}"#))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    client
        .delete_custom_rule("rule-id-1")
        .await
        .expect("request failed");
}

#[tokio::test]
async fn delete_custom_rules_progress_succeeds() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/users/current/custom_rules_progress"))
        .respond_with(json_response(200, r#"{"data": {}}"#))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::client_for(&server);
    client
        .delete_custom_rules_progress()
        .await
        .expect("request failed");
}
