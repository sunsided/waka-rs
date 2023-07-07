# WakaTime client for Rust

A client to fetch your coding statistics from [WakaTime](https://wakatime.com/) given your API key.

```rust
use std::error::Error;
use waka::WakaTimeClientBuilder;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("WAKATIME_API_KEY").expect("Missing WAKATIME_API_KEY variable");

    let client = WakaTimeClientBuilder::with_api_key(api_key)
        .with_user("sunside")
        .build()?;

    let summary = client
        .summaries(
            "2023-01-01",
            "2023-01-08",
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    println!("{summary:?}");

    Ok(())
}
```
