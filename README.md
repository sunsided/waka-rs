# WakaTime client for Rust

A client to fetch your coding statistics from [WakaTime](https://wakatime.com/) given your API key.

## Supported endpoints

- [All Time Since Today](https://wakatime.com/developers#all_time_since_today) — `all_time_since_today`
- [Commits](https://wakatime.com/developers#commits) — `commit` (single), `commits` (paginated list)
- [Durations](https://wakatime.com/developers#durations) — `durations`
- [Heartbeats](https://wakatime.com/developers#heartbeats) — `heartbeats`
- [Projects](https://wakatime.com/developers#projects) — `projects`
- [Stats](https://wakatime.com/developers#stats) — `stats`
- [Summaries](https://wakatime.com/developers#summaries) — `summaries`
- [Users](https://wakatime.com/developers#users) — `user`

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
