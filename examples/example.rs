use std::error::Error;
use waka::{
    AllTimesSinceTodayOptions, CommitOptions, CommitsOptions, DurationsOptions, EditorsOptions,
    ExternalDurationsOptions, InsightsOptions, LeadersOptions, ProjectsOptions, StatsOptions,
    SummariesOptions, WakaTimeClientBuilder,
};

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

    let all_time_since_today = client
        .all_time_since_today(AllTimesSinceTodayOptions::default())
        .await?;
    println!("{all_time_since_today:?}");

    let commit = client
        .commit(
            "cartoonify",
            "a9cb579b28b39880474c76471c3f337fb6bb9752",
            CommitOptions::default(),
        )
        .await?;
    println!("{commit:?}");

    let commits = client
        .commits("cartoonify", CommitsOptions::default())
        .await?;
    println!("{commits:?}");

    let user = client.user().await?;
    println!("{user:?}");

    let stats = client.stats("last_7_days", StatsOptions::default()).await?;
    println!("{stats:?}");

    let projects = client.projects(ProjectsOptions::default()).await?;
    println!("{projects:?}");

    let durations = client
        .durations("2023-01-02", DurationsOptions::default())
        .await?;
    println!("{durations:?}");

    let heartbeats = client.heartbeats("2023-01-02").await?;
    println!("{heartbeats:?}");

    let goals = client.goals().await?;
    println!("{goals:?}");

    let insights = client
        .insights("weekdays", "last_7_days", InsightsOptions::default())
        .await?;
    println!("{insights:?}");

    let leaders = client.leaders(LeadersOptions::default()).await?;
    println!("{leaders:?}");

    let machine_names = client.machine_names().await?;
    println!("{machine_names:?}");

    let user_agents = client.user_agents().await?;
    println!("{user_agents:?}");

    let status_bar = client.status_bar_today().await?;
    println!("{status_bar:?}");

    let editors = client.editors(EditorsOptions::default()).await?;
    println!("{editors:?}");

    let program_languages = client.program_languages().await?;
    println!("{program_languages:?}");

    let meta = client.meta().await?;
    println!("{meta:?}");

    let stats_aggregated = client.stats_aggregated("last_7_days").await?;
    println!("{stats_aggregated:?}");

    let private_leaderboards = client.private_leaderboards().await?;
    println!("{private_leaderboards:?}");

    let data_dumps = client.data_dumps().await?;
    println!("{data_dumps:?}");

    let custom_rules = client.custom_rules().await?;
    println!("{custom_rules:?}");

    let external_durations = client
        .external_durations("2023-01-02", ExternalDurationsOptions::default())
        .await?;
    println!("{external_durations:?}");

    Ok(())
}
