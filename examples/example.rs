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

    let all_time_since_today = client.all_time_since_today(None).await?;
    println!("{all_time_since_today:?}");

    let commit = client
        .commit(
            "cartoonify",
            "a9cb579b28b39880474c76471c3f337fb6bb9752",
            None,
        )
        .await?;
    println!("{commit:?}");

    Ok(())
}
