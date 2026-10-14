# WakaTime client for Rust

[![Crates.io](https://img.shields.io/crates/v/waka)](https://crates.io/crates/waka)
[![Safety Dance](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![docs.rs](https://img.shields.io/docsrs/waka)](https://docs.rs/waka)
[![CI](https://github.com/sunsided/waka-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/sunsided/waka-rs/actions/workflows/rust.yml)
[![License: EUPL-1.2](https://img.shields.io/badge/license-EUPL--1.2-blue)](LICENSE.md)

A client for the [WakaTime](https://wakatime.com/) API, covering every documented
endpoint — coding statistics, summaries, durations, heartbeats, goals, insights,
leaderboards, organizations, and the write operations.

This repository is a workspace containing two crates:

- [`waka`](waka/) — the API client library (this document)
- [`waka-cli`](waka-cli/) — a command-line client built on top of it

## Supported endpoints

- [All Time Since Today](https://wakatime.com/developers#all_time_since_today) — `all_time_since_today`
- [Commits](https://wakatime.com/developers#commits) — `commit` (single), `commits` (paginated list), `commits_all`
- [Custom Rules](https://wakatime.com/developers#custom_rules) — `custom_rules`, `custom_rules_progress`, `set_custom_rules`, `delete_custom_rule`, `delete_custom_rules_progress`
- [Data Dumps](https://wakatime.com/developers#data_dumps) — `data_dumps`, `create_data_dump`
- [Durations](https://wakatime.com/developers#durations) — `durations`
- [Editors](https://wakatime.com/developers#editors) — `editors`
- [External Durations](https://wakatime.com/developers#external_durations) — `external_durations`, `send_external_duration(s)`, `delete_external_durations`
- [Goals](https://wakatime.com/developers#goals) — `goals` (list), `goal` (single)
- [Heartbeats](https://wakatime.com/developers#heartbeats) — `heartbeats`, `send_heartbeat(s)`, `delete_heartbeats`
- [Insights](https://wakatime.com/developers#insights) — `insights`
- [Leaders](https://wakatime.com/developers#leaders) — `leaders`
- [Machine Names](https://wakatime.com/developers#machine_names) — `machine_names`
- [Meta](https://wakatime.com/developers#meta) — `meta`
- [Orgs](https://wakatime.com/developers#orgs) — `orgs`
- [Org Custom Rules](https://wakatime.com/developers#org_custom_rules) — `org_custom_rules`
- [Org Dashboards](https://wakatime.com/developers#org_dashboards) — `org_dashboards`, `org_dashboard_members`
- [Org Dashboard Durations](https://wakatime.com/developers#org_dashboard_durations) — `org_dashboard_durations`, `org_dashboard_member_durations`
- [Org Dashboard Summaries](https://wakatime.com/developers#org_dashboard_summaries) — `org_dashboard_summaries`, `org_dashboard_member_summaries`
- [Private Leaderboards](https://wakatime.com/developers#private_leaderboards) — `private_leaderboards`, `private_leaderboard_leaders`
- [Program Languages](https://wakatime.com/developers#program_languages) — `program_languages`
- [Projects](https://wakatime.com/developers#projects) — `projects`, `projects_all`
- [Stats](https://wakatime.com/developers#stats) — `stats`
- [Stats Aggregated](https://wakatime.com/developers#stats_aggregated) — `stats_aggregated`
- [Status Bar](https://wakatime.com/developers#status_bar) — `status_bar_today`
- [Summaries](https://wakatime.com/developers#summaries) — `summaries`
- [User Agents](https://wakatime.com/developers#user_agents) — `user_agents`
- [Users](https://wakatime.com/developers#users) — `user`

Authentication works with an OAuth 2.0 access token (`new_with_bearer_token`,
`new_with_oauth_token`) or an API key (`new_with_api_key`). WakaTime API keys
are write-only: they can send heartbeats, but reading data requires an OAuth
token. The `oauth` module implements the authorization code flow
(`OAuthClient`: authorization URL, code exchange, refresh, revoke). The write
endpoints are tested against mocks only; their request shapes follow the
developer docs.

## Usage

Add the dependency:

```toml
[dependencies]
waka = "0.2.0"
```

```rust
use std::error::Error;
use waka::{SummariesOptions, WakaTimeClientBuilder};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("WAKATIME_API_KEY").expect("Missing WAKATIME_API_KEY variable");

    let client = WakaTimeClientBuilder::new_with_api_key(api_key)
        .with_user("sunside")
        .build()?;

    let summary = client
        .summaries("2023-01-01", "2023-01-08", SummariesOptions::default())
        .await?;
    println!("{summary:?}");

    Ok(())
}
```

See [examples/example.rs](examples/example.rs) for all supported endpoints.

### OAuth 2.0

Register an app at [wakatime.com/apps](https://wakatime.com/apps) and add your
redirect URI to its authorized redirect URLs. Then send the user to the
authorization URL and exchange the `code` WakaTime redirects back with:

```rust
use waka::WakaTimeClientBuilder;
use waka::oauth::{AuthorizeOptions, OAuthClient, Scope};

let oauth = OAuthClient::new("client-id", "client-secret", "https://example.com/callback")?;
let url = oauth.authorize_url(AuthorizeOptions {
    scopes: &[Scope::ReadStats, Scope::ReadSummaries],
    state: Some("an-unguessable-value"),
    ..Default::default()
});
// … redirect the user to `url`; WakaTime redirects back with `code` and `state` …

let token = oauth.exchange_code(code).await?;
let client = WakaTimeClientBuilder::new_with_oauth_token(&token).build()?;
```

`Token` serializes losslessly for storage; `OAuthClient::refresh` and
`OAuthClient::revoke` take its refresh and access tokens. WakaTime keeps at
most 8 active tokens per app and user, revoking the oldest on the 9th sign-in.
See [examples/oauth.rs](examples/oauth.rs) for a complete flow.
