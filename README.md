# WakaTime client for Rust

A client to fetch your coding statistics from [WakaTime](https://wakatime.com/) given your API key.

## Supported endpoints

- [All Time Since Today](https://wakatime.com/developers#all_time_since_today) — `all_time_since_today`
- [Commits](https://wakatime.com/developers#commits) — `commit` (single), `commits` (paginated list)
- [Custom Rules](https://wakatime.com/developers#custom_rules) — `custom_rules`, `custom_rules_progress`
- [Data Dumps](https://wakatime.com/developers#data_dumps) — `data_dumps`
- [Durations](https://wakatime.com/developers#durations) — `durations`
- [Editors](https://wakatime.com/developers#editors) — `editors`
- [External Durations](https://wakatime.com/developers#external_durations) — `external_durations`
- [Goals](https://wakatime.com/developers#goals) — `goals` (list), `goal` (single)
- [Heartbeats](https://wakatime.com/developers#heartbeats) — `heartbeats`
- [Insights](https://wakatime.com/developers#insights) — `insights`
- [Leaders](https://wakatime.com/developers#leaders) — `leaders`
- [Machine Names](https://wakatime.com/developers#machine_names) — `machine_names`
- [Meta](https://wakatime.com/developers#meta) — `meta`
- [Private Leaderboards](https://wakatime.com/developers#private_leaderboards) — `private_leaderboards`, `private_leaderboard_leaders`
- [Program Languages](https://wakatime.com/developers#program_languages) — `program_languages`
- [Projects](https://wakatime.com/developers#projects) — `projects`
- [Stats](https://wakatime.com/developers#stats) — `stats`
- [Stats Aggregated](https://wakatime.com/developers#stats_aggregated) — `stats_aggregated`
- [Status Bar](https://wakatime.com/developers#status_bar) — `status_bar_today`
- [Summaries](https://wakatime.com/developers#summaries) — `summaries`
- [User Agents](https://wakatime.com/developers#user_agents) — `user_agents`
- [Users](https://wakatime.com/developers#users) — `user`

Write endpoints (posting heartbeats, managing custom rules) and the
organization dashboards are not implemented.

## Usage

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
