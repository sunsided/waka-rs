//! Command-line client for the WakaTime API.

mod cli;
mod human;
mod output;

use clap::Parser;
use cli::{
    Cli, Command, CustomRulesCommand, DataDumpsCommand, ExternalDurationsCommand,
    HeartbeatsCommand, OrgCommand,
};
use output::emit;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::io::Read;
use std::path::Path;
use waka::{WakaTimeClient, WakaTimeClientBuilder, model};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();
    if let Err(e) = run(cli).await {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

/// Builds the client from the global arguments.
fn build_client(cli: &Cli) -> Result<WakaTimeClient, Box<dyn Error>> {
    let mut builder = match (&cli.api_key, &cli.bearer_token) {
        (Some(api_key), _) => WakaTimeClientBuilder::new_with_api_key(api_key),
        (_, Some(token)) => WakaTimeClientBuilder::new_with_bearer_token(token),
        (None, None) => {
            return Err(
                "missing credentials: set --api-key (WAKATIME_API_KEY) or --bearer-token \
                 (WAKATIME_BEARER_TOKEN), e.g. in a .env file"
                    .into(),
            );
        }
    };
    if let Some(user) = &cli.user {
        builder = builder.with_user(user);
    }
    if let Some(base_url) = &cli.base_url {
        builder = builder.with_base_url(base_url);
    }
    if let Some(timeout) = cli.timeout {
        builder = builder.with_timeout(std::time::Duration::from_secs(timeout));
    }
    Ok(builder.build()?)
}

/// Reads a JSON value from a file, or from stdin when the path is `-` or absent.
fn read_json_input<T: DeserializeOwned>(file: Option<&Path>) -> Result<T, Box<dyn Error>> {
    let text = match file {
        Some(path) if path != Path::new("-") => std::fs::read_to_string(path)?,
        _ => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };
    Ok(serde_json::from_str(&text)?)
}

/// The current time as a UNIX epoch timestamp.
fn epoch_now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or_default()
}

async fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let client = build_client(&cli)?;
    let json = cli.json;
    match cli.command {
        Command::User => emit(json, &client.user().await?, human::user),

        Command::AllTime { project } => emit(
            json,
            &client
                .all_time_since_today(waka::AllTimesSinceTodayOptions {
                    project: project.as_deref(),
                })
                .await?,
            human::all_time,
        ),

        Command::Stats {
            range,
            keystroke_timeout,
            writes_only,
            project,
        } => emit(
            json,
            &client
                .stats(
                    &range,
                    waka::StatsOptions {
                        timeout: keystroke_timeout,
                        writes_only: writes_only.then_some(true),
                        project: project.as_deref(),
                    },
                )
                .await?,
            human::stats,
        ),

        Command::StatsAggregated { range } => emit(
            json,
            &client.stats_aggregated(&range).await?,
            human::stats_aggregated,
        ),

        Command::Summaries {
            start,
            end,
            project,
            branches,
            keystroke_timeout,
            writes_only,
            timezone,
            range,
        } => emit(
            json,
            &client
                .summaries(
                    &start,
                    &end,
                    waka::SummariesOptions {
                        project: project.as_deref(),
                        branches: branches.as_deref(),
                        timeout: keystroke_timeout,
                        writes_only: writes_only.then_some(true),
                        timezone: timezone.as_deref(),
                        range: range.as_deref(),
                    },
                )
                .await?,
            human::summaries,
        ),

        Command::Durations {
            date,
            project,
            branches,
            keystroke_timeout,
            writes_only,
            timezone,
            slice_by,
        } => emit(
            json,
            &client
                .durations(
                    &date,
                    waka::DurationsOptions {
                        project: project.as_deref(),
                        branches: branches.as_deref(),
                        timeout: keystroke_timeout,
                        writes_only: writes_only.then_some(true),
                        timezone: timezone.as_deref(),
                        slice_by: slice_by.as_deref(),
                    },
                )
                .await?,
            human::durations,
        ),

        Command::Insights {
            insight_type,
            range,
            keystroke_timeout,
            writes_only,
            weekday,
        } => emit(
            json,
            &client
                .insights(
                    &insight_type,
                    &range,
                    waka::InsightsOptions {
                        timeout: keystroke_timeout,
                        writes_only: writes_only.then_some(true),
                        weekday: weekday.as_deref(),
                    },
                )
                .await?,
            human::insight,
        ),

        Command::Projects { query, page, all } => {
            if all {
                emit(
                    json,
                    &client.projects_all(query.as_deref()).await?,
                    |projects| human::project_list(projects),
                )
            } else {
                emit(
                    json,
                    &client
                        .projects(waka::ProjectsOptions {
                            q: query.as_deref(),
                            page,
                        })
                        .await?,
                    human::projects,
                )
            }
        }

        Command::Commits {
            project,
            author,
            branch,
            page,
            all,
        } => {
            let options = waka::CommitsOptions {
                author: author.as_deref(),
                branch: branch.as_deref(),
                page,
            };
            if all {
                emit(
                    json,
                    &client.commits_all(&project, options).await?,
                    |commits| human::commit_list(commits),
                )
            } else {
                emit(
                    json,
                    &client.commits(&project, options).await?,
                    human::commits_page,
                )
            }
        }

        Command::Commit {
            project,
            hash,
            branch,
        } => emit(
            json,
            &client
                .commit(
                    &project,
                    &hash,
                    waka::CommitOptions {
                        branch: branch.as_deref(),
                    },
                )
                .await?,
            human::commit,
        ),

        Command::Goals => emit(json, &client.goals().await?, human::goals),

        Command::Goal { id } => emit(json, &client.goal(&id).await?, human::goal),

        Command::Leaders {
            language,
            hireable,
            country_code,
            page,
        } => emit(
            json,
            &client
                .leaders(waka::LeadersOptions {
                    language: language.as_deref(),
                    is_hireable: hireable.then_some(true),
                    country_code: country_code.as_deref(),
                    page,
                })
                .await?,
            human::leaders,
        ),

        Command::Leaderboards => emit(
            json,
            &client.private_leaderboards().await?,
            human::private_leaderboards,
        ),

        Command::Leaderboard {
            board,
            language,
            country_code,
            page,
        } => emit(
            json,
            &client
                .private_leaderboard_leaders(
                    &board,
                    waka::PrivateLeaderboardLeadersOptions {
                        language: language.as_deref(),
                        country_code: country_code.as_deref(),
                        page,
                    },
                )
                .await?,
            human::leaders,
        ),

        Command::MachineNames => emit(json, &client.machine_names().await?, human::machine_names),

        Command::UserAgents => emit(json, &client.user_agents().await?, human::user_agents),

        Command::Editors { unreleased } => emit(
            json,
            &client
                .editors(waka::EditorsOptions {
                    unreleased: unreleased.then_some(true),
                })
                .await?,
            human::editors,
        ),

        Command::ProgramLanguages => emit(
            json,
            &client.program_languages().await?,
            human::program_languages,
        ),

        Command::Meta => emit(json, &client.meta().await?, human::meta),

        Command::StatusBar => emit(json, &client.status_bar_today().await?, human::status_bar),

        Command::Heartbeats { command } => run_heartbeats(&client, json, command).await,

        Command::ExternalDurations { command } => {
            run_external_durations(&client, json, command).await
        }

        Command::DataDumps { command } => run_data_dumps(&client, json, command).await,

        Command::CustomRules { command } => run_custom_rules(&client, json, command).await,

        Command::Org { command } => run_org(&client, json, command).await,
    }
}

async fn run_heartbeats(
    client: &WakaTimeClient,
    json: bool,
    command: HeartbeatsCommand,
) -> Result<(), Box<dyn Error>> {
    match command {
        HeartbeatsCommand::List { date } => {
            emit(json, &client.heartbeats(&date).await?, human::heartbeats)
        }
        HeartbeatsCommand::Send {
            entity,
            entity_type,
            time,
            category,
            project,
            branch,
            language,
            lines,
            lineno,
            cursorpos,
            is_write,
        } => {
            let heartbeat = model::HeartbeatInput {
                entity,
                r#type: entity_type,
                time: time.unwrap_or_else(epoch_now),
                category,
                project,
                branch,
                language,
                lines,
                lineno,
                cursorpos,
                is_write: is_write.then_some(true),
                ..Default::default()
            };
            emit(
                json,
                &client.send_heartbeat(&heartbeat).await?,
                human::created_heartbeat,
            )
        }
        HeartbeatsCommand::SendBulk { file } => {
            let heartbeats: Vec<model::HeartbeatInput> = read_json_input(file.as_deref())?;
            emit(
                json,
                &client.send_heartbeats(&heartbeats).await?,
                human::json_value,
            )
        }
        HeartbeatsCommand::Delete { date, ids } => {
            let ids: Vec<&str> = ids.iter().map(String::as_str).collect();
            client.delete_heartbeats(&date, &ids).await?;
            println!("Deleted {} heartbeats on {date}.", ids.len());
            Ok(())
        }
    }
}

async fn run_external_durations(
    client: &WakaTimeClient,
    json: bool,
    command: ExternalDurationsCommand,
) -> Result<(), Box<dyn Error>> {
    match command {
        ExternalDurationsCommand::List {
            date,
            project,
            branches,
            timezone,
        } => emit(
            json,
            &client
                .external_durations(
                    &date,
                    waka::ExternalDurationsOptions {
                        project: project.as_deref(),
                        branches: branches.as_deref(),
                        timezone: timezone.as_deref(),
                    },
                )
                .await?,
            human::external_durations,
        ),
        ExternalDurationsCommand::Send {
            external_id,
            entity,
            entity_type,
            start,
            end,
            category,
            project,
            branch,
            language,
            meta,
        } => {
            let duration = model::ExternalDurationInput {
                external_id,
                entity,
                r#type: entity_type,
                start_time: start,
                end_time: end,
                category,
                project,
                branch,
                language,
                meta,
            };
            emit(
                json,
                &client.send_external_duration(&duration).await?,
                human::external_duration,
            )
        }
        ExternalDurationsCommand::SendBulk { file } => {
            let durations: Vec<model::ExternalDurationInput> = read_json_input(file.as_deref())?;
            emit(
                json,
                &client.send_external_durations(&durations).await?,
                human::json_value,
            )
        }
        ExternalDurationsCommand::Delete { date, ids } => {
            let ids: Vec<&str> = ids.iter().map(String::as_str).collect();
            client.delete_external_durations(&date, &ids).await?;
            println!("Deleted {} external durations on {date}.", ids.len());
            Ok(())
        }
    }
}

async fn run_data_dumps(
    client: &WakaTimeClient,
    json: bool,
    command: DataDumpsCommand,
) -> Result<(), Box<dyn Error>> {
    match command {
        DataDumpsCommand::List => emit(json, &client.data_dumps().await?, human::data_dumps),
        DataDumpsCommand::Create {
            dump_type,
            no_email,
        } => emit(
            json,
            &client
                .create_data_dump(&dump_type, no_email.then_some(false))
                .await?,
            human::data_dump,
        ),
    }
}

async fn run_custom_rules(
    client: &WakaTimeClient,
    json: bool,
    command: CustomRulesCommand,
) -> Result<(), Box<dyn Error>> {
    match command {
        CustomRulesCommand::List => emit(json, &client.custom_rules().await?, human::custom_rules),
        CustomRulesCommand::Set { file } => {
            let rules: Vec<model::CustomRuleInput> = read_json_input(file.as_deref())?;
            emit(
                json,
                &client.set_custom_rules(&rules).await?,
                human::custom_rule_changes,
            )
        }
        CustomRulesCommand::Delete { id } => {
            client.delete_custom_rule(&id).await?;
            println!("Deleted custom rule {id}.");
            Ok(())
        }
        CustomRulesCommand::Progress { job_id } => emit(
            json,
            &client.custom_rules_progress(&job_id).await?,
            human::custom_rules_progress,
        ),
        CustomRulesCommand::ClearProgress => {
            client.delete_custom_rules_progress().await?;
            println!("Cleared custom rules job progress.");
            Ok(())
        }
    }
}

async fn run_org(
    client: &WakaTimeClient,
    json: bool,
    command: OrgCommand,
) -> Result<(), Box<dyn Error>> {
    match command {
        OrgCommand::List => emit(json, &client.orgs().await?, human::orgs),
        OrgCommand::Dashboards { org } => emit(
            json,
            &client.org_dashboards(&org).await?,
            human::org_dashboards,
        ),
        OrgCommand::Members { org, dashboard } => emit(
            json,
            &client.org_dashboard_members(&org, &dashboard).await?,
            human::org_dashboard_members,
        ),
        OrgCommand::Durations {
            org,
            dashboard,
            date,
            project,
            branches,
            slice_by,
        } => emit(
            json,
            &client
                .org_dashboard_durations(
                    &org,
                    &dashboard,
                    &date,
                    waka::OrgDurationsOptions {
                        project: project.as_deref(),
                        branches: branches.as_deref(),
                        slice_by: slice_by.as_deref(),
                    },
                )
                .await?,
            human::org_dashboard_durations,
        ),
        OrgCommand::Summaries {
            org,
            dashboard,
            date,
            project,
            branches,
        } => emit(
            json,
            &client
                .org_dashboard_summaries(
                    &org,
                    &dashboard,
                    &date,
                    waka::OrgSummariesOptions {
                        project: project.as_deref(),
                        branches: branches.as_deref(),
                    },
                )
                .await?,
            human::org_dashboard_summaries,
        ),
        OrgCommand::MemberDurations {
            org,
            dashboard,
            member,
            date,
            project,
            branches,
            slice_by,
        } => emit(
            json,
            &client
                .org_dashboard_member_durations(
                    &org,
                    &dashboard,
                    &member,
                    &date,
                    waka::OrgDurationsOptions {
                        project: project.as_deref(),
                        branches: branches.as_deref(),
                        slice_by: slice_by.as_deref(),
                    },
                )
                .await?,
            human::durations,
        ),
        OrgCommand::MemberSummaries {
            org,
            dashboard,
            member,
            start,
            end,
            project,
            branches,
            range,
        } => emit(
            json,
            &client
                .org_dashboard_member_summaries(
                    &org,
                    &dashboard,
                    &member,
                    &start,
                    &end,
                    waka::OrgMemberSummariesOptions {
                        project: project.as_deref(),
                        branches: branches.as_deref(),
                        range: range.as_deref(),
                    },
                )
                .await?,
            human::org_member_summaries,
        ),
        OrgCommand::CustomRules { org } => emit(
            json,
            &client.org_custom_rules(&org).await?,
            human::org_custom_rules,
        ),
    }
}
