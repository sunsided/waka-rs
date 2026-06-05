//! GET-only smoke test against the live WakaTime API.
//!
//! Calls every read endpoint with the API key from `WAKATIME_API_KEY`
//! (or `.env`) and reports whether the response deserializes, making it
//! easy to spot model drift against the real API.

use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use waka::{
    AllTimesSinceTodayOptions, CommitsOptions, DurationsOptions, EditorsOptions,
    ExternalDurationsOptions, InsightsOptions, LeadersOptions, ProjectsOptions, StatsOptions,
    SummariesOptions, WakaTimeClient, WakaTimeClientBuilder,
};

/// Converts days since the UNIX epoch to a `YYYY-MM-DD` date string
/// (civil-from-days, see Howard Hinnant's date algorithms).
fn civil_date(days: i64) -> String {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

fn days_ago(n: i64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs() as i64;
    civil_date(now / 86_400 - n)
}

async fn report<T: std::fmt::Debug>(
    name: &str,
    result: Result<T, waka::ApiError>,
    failures: &mut Vec<String>,
) {
    match result {
        Ok(_) => println!("PASS  {name}"),
        // Plan- or account-dependent: the endpoint exists but this account
        // cannot access it, which says nothing about our models.
        Err(
            e @ (waka::ApiError::Forbidden(_)
            | waka::ApiError::NotFound(_)
            | waka::ApiError::PaymentRequired(_)),
        ) => println!("SKIP  {name}: {e}"),
        Err(e) => {
            println!("FAIL  {name}: {e}");
            failures.push(format!("{name}: {e}"));
        }
    }
}

async fn run(client: &WakaTimeClient, failures: &mut Vec<String>) {
    let yesterday = days_ago(1);
    let week_ago = days_ago(7);

    report(
        "all_time_since_today",
        client
            .all_time_since_today(AllTimesSinceTodayOptions::default())
            .await,
        failures,
    )
    .await;
    report(
        "summaries",
        client
            .summaries(&week_ago, &yesterday, SummariesOptions::default())
            .await,
        failures,
    )
    .await;
    report(
        "stats",
        client.stats("last_7_days", StatsOptions::default()).await,
        failures,
    )
    .await;
    let projects = client.projects(ProjectsOptions::default()).await;
    let first_project = projects
        .as_ref()
        .ok()
        .and_then(|p| p.data.first())
        .map(|p| p.name.clone());
    report("projects", projects, failures).await;
    report(
        "durations",
        client
            .durations(&yesterday, DurationsOptions::default())
            .await,
        failures,
    )
    .await;
    report("heartbeats", client.heartbeats(&yesterday).await, failures).await;
    report("user", client.user().await, failures).await;
    report("goals", client.goals().await, failures).await;
    report(
        "insights",
        client
            .insights("weekdays", "last_7_days", InsightsOptions::default())
            .await,
        failures,
    )
    .await;
    report(
        "leaders",
        client.leaders(LeadersOptions::default()).await,
        failures,
    )
    .await;
    report("machine_names", client.machine_names().await, failures).await;
    report("user_agents", client.user_agents().await, failures).await;
    report(
        "status_bar_today",
        client.status_bar_today().await,
        failures,
    )
    .await;
    report("custom_rules", client.custom_rules().await, failures).await;
    report("data_dumps", client.data_dumps().await, failures).await;
    report(
        "external_durations",
        client
            .external_durations(&yesterday, ExternalDurationsOptions::default())
            .await,
        failures,
    )
    .await;
    report(
        "editors",
        client.editors(EditorsOptions::default()).await,
        failures,
    )
    .await;
    report(
        "program_languages",
        client.program_languages().await,
        failures,
    )
    .await;
    report("meta", client.meta().await, failures).await;
    report(
        "stats_aggregated",
        client.stats_aggregated("last_7_days").await,
        failures,
    )
    .await;
    report(
        "private_leaderboards",
        client.private_leaderboards().await,
        failures,
    )
    .await;
    report("orgs", client.orgs().await, failures).await;

    if let Some(project) = first_project {
        let commits = client.commits(&project, CommitsOptions::default()).await;
        let first_hash = commits
            .as_ref()
            .ok()
            .and_then(|c| c.data.first())
            .map(|c| c.hash.clone());
        report("commits", commits, failures).await;

        if let Some(hash) = first_hash {
            report(
                "commit",
                client
                    .commit(&project, &hash, waka::CommitOptions::default())
                    .await,
                failures,
            )
            .await;
        } else {
            println!("SKIP  commit: no commits found");
        }
    } else {
        println!("SKIP  commits: no projects found");
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("WAKATIME_API_KEY").expect("Missing WAKATIME_API_KEY variable");

    let client = WakaTimeClientBuilder::new_with_api_key(api_key).build()?;

    let mut failures = Vec::new();
    run(&client, &mut failures).await;

    if failures.is_empty() {
        println!("\nall endpoints OK");
        Ok(())
    } else {
        println!("\n{} endpoint(s) failed", failures.len());
        std::process::exit(1);
    }
}
