//! Human-readable renderers for API responses.

use waka::model;

/// Formats a number of seconds as `3h 24m 10s`.
pub fn hms(total_seconds: f64) -> String {
    let s = total_seconds.round() as u64;
    let h = s / 3600;
    let m = (s % 3600) / 60;
    let sec = s % 60;
    if h > 0 {
        format!("{h}h {m:02}m {sec:02}s")
    } else if m > 0 {
        format!("{m}m {sec:02}s")
    } else {
        format!("{sec}s")
    }
}

/// Formats a UNIX epoch timestamp as a UTC `HH:MM:SS` time of day.
fn epoch_hms(epoch: f64) -> String {
    let s = epoch.round() as u64 % 86_400;
    format!("{:02}:{:02}:{:02}", s / 3600, (s % 3600) / 60, s % 60)
}

/// Returns the string or `-` when absent.
fn opt(value: &Option<String>) -> &str {
    value.as_deref().unwrap_or("-")
}

/// Prints rows as a left-aligned table with two-space gaps.
fn table(rows: &[Vec<String>]) {
    let cols = rows.iter().map(Vec::len).max().unwrap_or(0);
    let widths: Vec<usize> = (0..cols)
        .map(|c| {
            rows.iter()
                .filter_map(|r| r.get(c))
                .map(|s| s.chars().count())
                .max()
                .unwrap_or(0)
        })
        .collect();
    for row in rows {
        let mut line = String::new();
        for (c, cell) in row.iter().enumerate() {
            if c > 0 {
                line.push_str("  ");
            }
            line.push_str(cell);
            if c + 1 < row.len() {
                let pad = widths[c].saturating_sub(cell.chars().count());
                line.push_str(&" ".repeat(pad));
            }
        }
        println!("{}", line.trim_end());
    }
}

fn pagination(p: &model::Pagination) {
    if let (Some(page), Some(total_pages)) = (p.page, p.total_pages) {
        match p.total {
            Some(total) => println!("Page {page} of {total_pages} ({total} total)"),
            None => println!("Page {page} of {total_pages}"),
        }
    }
}

/// Prints a percentage breakdown table, e.g. for languages or projects.
fn breakdown(title: &str, entries: &[(String, f64, Option<f64>)]) {
    if entries.is_empty() {
        return;
    }
    println!();
    println!("{title}:");
    let rows: Vec<Vec<String>> = entries
        .iter()
        .map(|(name, seconds, percent)| {
            vec![
                name.clone(),
                hms(*seconds),
                percent.map(|p| format!("{p:.1}%")).unwrap_or_default(),
            ]
        })
        .collect();
    table(&rows);
}

pub fn user(u: &model::User) {
    println!("{} ({})", opt(&u.display_name), u.id);
    let mut rows = Vec::new();
    let mut push = |k: &str, v: Option<&str>| {
        if let Some(v) = v {
            rows.push(vec![format!("{k}:"), v.to_string()]);
        }
    };
    push("Username", u.username.as_deref());
    push("Full name", u.full_name.as_deref());
    push("Email", u.email.as_deref());
    push("Location", u.location.as_deref());
    push("Timezone", u.timezone.as_deref());
    push("Plan", u.plan.as_deref());
    push("Website", u.human_readable_website.as_deref());
    push("Last heartbeat", u.last_heartbeat_at.as_deref());
    push("Last plugin", u.last_plugin_name.as_deref());
    push("Last project", u.last_project.as_deref());
    push("Created", u.created_at.as_deref());
    table(&rows);
}

pub fn all_time(t: &model::AllTimeSinceToday) {
    println!("{} ({}) since {}", t.text, t.digital, t.range.start_date);
    if !t.is_up_to_date {
        match t.percent_calculated {
            Some(p) => println!("Stats are being refreshed ({p}% calculated)."),
            None => println!("Stats are being refreshed."),
        }
    }
}

fn stats_breakdown(
    title: &str,
    aggregates: &Option<Vec<model::stats::StatsAggregate>>,
    limit: usize,
) {
    if let Some(aggregates) = aggregates {
        let entries: Vec<_> = aggregates
            .iter()
            .take(limit)
            .map(|a| (a.name.clone(), a.total_seconds, a.percent))
            .collect();
        breakdown(title, &entries);
    }
}

pub fn stats(s: &model::Stats) {
    println!(
        "Stats for {}",
        s.human_readable_range.as_deref().unwrap_or(&s.range)
    );
    if let Some(total) = s.total_seconds {
        println!("Total:         {}", hms(total));
    }
    if let Some(avg) = s.daily_average {
        println!("Daily average: {}", hms(avg));
    }
    if let (Some(days), Some(active)) = (s.days_including_holidays, s.days_minus_holidays) {
        println!("Days:          {days} ({active} active)");
    }
    if let Some(best) = &s.best_day {
        println!("Best day:      {} ({})", best.date, best.text);
    }
    stats_breakdown("Languages", &s.languages, 10);
    stats_breakdown("Editors", &s.editors, 10);
    stats_breakdown("Projects", &s.projects, 10);
    stats_breakdown("Operating systems", &s.operating_systems, 10);
    stats_breakdown("Categories", &s.categories, 10);
    stats_breakdown("Machines", &s.machines, 10);
    if !s.is_up_to_date {
        println!();
        println!("Stats are being refreshed (status: {}).", s.status);
    }
}

fn aggregated_measures(name: &str, m: &model::stats_aggregated::AggregatedMeasures) -> Vec<String> {
    let value = |v: &Option<model::stats_aggregated::AggregatedValue>| {
        v.as_ref()
            .and_then(|v| v.text.clone().or_else(|| v.seconds.map(hms)))
            .unwrap_or_else(|| "-".to_string())
    };
    vec![
        name.to_string(),
        value(&m.average),
        value(&m.median),
        value(&m.max),
        value(&m.sum),
    ]
}

pub fn stats_aggregated(s: &model::AggregatedStats) {
    let mut rows = vec![vec![
        "".to_string(),
        "average".to_string(),
        "median".to_string(),
        "max".to_string(),
        "sum".to_string(),
    ]];
    if let Some(total) = &s.total {
        rows.push(aggregated_measures("Total", total));
    }
    if let Some(avg) = &s.daily_average {
        rows.push(aggregated_measures("Daily average", avg));
    }
    table(&rows);
    let sections: [(&str, &Option<Vec<model::stats_aggregated::AggregatedEntry>>); 4] = [
        ("Languages", &s.languages),
        ("Editors", &s.editors),
        ("Categories", &s.categories),
        ("Operating systems", &s.operating_systems),
    ];
    for (title, entries) in sections {
        if let Some(entries) = entries {
            println!();
            println!("{title}:");
            let mut rows = vec![vec![
                "".to_string(),
                "average".to_string(),
                "median".to_string(),
                "max".to_string(),
                "sum".to_string(),
            ]];
            for e in entries {
                rows.push(aggregated_measures(&e.name, &e.measures));
            }
            table(&rows);
        }
    }
}

fn top_name<'a>(names: impl Iterator<Item = (&'a str, f64)>) -> &'a str {
    names
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(n, _)| n)
        .unwrap_or("-")
}

pub fn summaries(s: &model::Summaries) {
    let rows: Vec<Vec<String>> = s
        .data
        .iter()
        .map(|day| {
            vec![
                day.range.date.clone(),
                day.grand_total.text.clone(),
                top_name(
                    day.projects
                        .iter()
                        .map(|p| (p.name.as_str(), p.total_seconds)),
                )
                .to_string(),
                top_name(
                    day.languages
                        .iter()
                        .map(|l| (l.name.as_str(), l.total_seconds)),
                )
                .to_string(),
            ]
        })
        .collect();
    let mut all = vec![vec![
        "date".to_string(),
        "total".to_string(),
        "top project".to_string(),
        "top language".to_string(),
    ]];
    all.extend(rows);
    table(&all);
    println!();
    println!("Cumulative total: {}", s.cumulative_total.text);
    println!(
        "Daily average:    {} over {} active days",
        s.daily_average.text, s.daily_average.days_minus_holidays
    );
}

pub fn durations(d: &model::Durations) {
    let mut rows = vec![vec![
        "start".to_string(),
        "duration".to_string(),
        "project".to_string(),
    ]];
    let mut total = 0.0;
    for duration in &d.data {
        total += duration.duration;
        rows.push(vec![
            epoch_hms(duration.time),
            hms(duration.duration),
            opt(&duration.project).to_string(),
        ]);
    }
    table(&rows);
    println!();
    println!(
        "Total: {} over {} durations ({})",
        hms(total),
        d.data.len(),
        d.timezone
    );
}

pub fn insight(i: &model::Insight) {
    if let Some(range) = i.human_readable_range.as_deref().or(i.range.as_deref()) {
        println!("Insight for {range}");
    }
    if let Some(status) = &i.status {
        println!("Status: {status}");
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&i.payload).unwrap_or_default()
    );
}

fn project_rows(projects: &[model::projects::ProjectSummary]) -> Vec<Vec<String>> {
    let mut rows = vec![vec![
        "name".to_string(),
        "last heartbeat".to_string(),
        "id".to_string(),
    ]];
    rows.extend(projects.iter().map(|p| {
        vec![
            p.name.clone(),
            p.human_readable_last_heartbeat_at
                .clone()
                .or_else(|| p.last_heartbeat_at.clone())
                .unwrap_or_else(|| "-".to_string()),
            p.id.clone(),
        ]
    }));
    rows
}

pub fn projects(p: &model::Projects) {
    table(&project_rows(&p.data));
    pagination(&p.pagination);
}

pub fn project_list(projects: &[model::projects::ProjectSummary]) {
    table(&project_rows(projects));
    println!("{} projects", projects.len());
}

fn commit_rows(commits: &[model::commit::Commit]) -> Vec<Vec<String>> {
    let mut rows = vec![vec![
        "hash".to_string(),
        "date".to_string(),
        "time".to_string(),
        "author".to_string(),
        "message".to_string(),
    ]];
    rows.extend(commits.iter().map(|c| {
        let message: String = c.message.lines().next().unwrap_or_default().to_string();
        vec![
            c.truncated_hash.clone(),
            c.author_date.clone(),
            c.human_readable_total.clone(),
            c.author_name.clone(),
            message,
        ]
    }));
    rows
}

pub fn commits_page(c: &model::CommitsPage) {
    if let Some(project) = &c.project {
        println!("Commits of {} on branch {}", project.name, opt(&c.branch));
        println!();
    }
    table(&commit_rows(&c.data));
    pagination(&c.pagination);
}

pub fn commit_list(commits: &[model::commit::Commit]) {
    table(&commit_rows(commits));
    println!("{} commits", commits.len());
}

pub fn commit(c: &model::Commits) {
    let commit = &c.commit;
    println!("commit {} ({})", commit.hash, c.branch);
    println!("Author:  {} <{}>", commit.author_name, commit.author_email);
    println!("Date:    {}", commit.author_date);
    println!("Time:    {}", commit.human_readable_total_with_seconds);
    println!("Project: {}", c.project.name);
    println!();
    for line in commit.message.lines() {
        println!("    {line}");
    }
}

fn goal_rows(goals: &[model::goals::Goal]) -> Vec<Vec<String>> {
    let mut rows = vec![vec![
        "title".to_string(),
        "status".to_string(),
        "range".to_string(),
        "id".to_string(),
    ]];
    rows.extend(goals.iter().map(|g| {
        vec![
            g.custom_title
                .clone()
                .or_else(|| g.title.clone())
                .unwrap_or_else(|| "-".to_string()),
            opt(&g.status).to_string(),
            opt(&g.range_text).to_string(),
            g.id.clone(),
        ]
    }));
    rows
}

pub fn goals(g: &model::Goals) {
    table(&goal_rows(&g.data));
    pagination(&g.pagination);
}

pub fn goal(g: &model::CachedGoal) {
    table(&goal_rows(std::slice::from_ref(&g.data)));
    if let Some(chart) = &g.data.chart_data {
        println!();
        let mut rows = vec![vec![
            "range".to_string(),
            "actual".to_string(),
            "goal".to_string(),
            "status".to_string(),
        ]];
        rows.extend(chart.iter().map(|c| {
            vec![
                c.range
                    .as_ref()
                    .and_then(|r| r.date.clone().or_else(|| r.text.clone()))
                    .unwrap_or_else(|| "-".to_string()),
                opt(&c.actual_seconds_text).to_string(),
                opt(&c.goal_seconds_text).to_string(),
                opt(&c.range_status).to_string(),
            ]
        }));
        table(&rows);
    }
}

fn leader_name(user: &model::leaders::LeaderUser) -> String {
    user.display_name
        .clone()
        .or_else(|| user.username.clone())
        .or_else(|| user.full_name.clone())
        .unwrap_or_else(|| user.id.clone())
}

pub fn leaders(l: &model::Leaders) {
    if let Some(range) = l.range.as_ref().and_then(|r| r.text.clone()) {
        println!("Leaderboard for {range}");
        println!();
    }
    let mut rows = vec![vec![
        "rank".to_string(),
        "user".to_string(),
        "total".to_string(),
        "daily avg".to_string(),
    ]];
    rows.extend(l.data.iter().map(|e| {
        let (total, avg) = e
            .running_total
            .as_ref()
            .map(|t| {
                (
                    t.human_readable_total
                        .clone()
                        .unwrap_or_else(|| hms(t.total_seconds)),
                    t.human_readable_daily_average
                        .clone()
                        .or_else(|| t.daily_average.map(hms))
                        .unwrap_or_else(|| "-".to_string()),
                )
            })
            .unwrap_or_else(|| ("-".to_string(), "-".to_string()));
        vec![format!("#{}", e.rank), leader_name(&e.user), total, avg]
    }));
    table(&rows);
    if let Some(current) = &l.current_user {
        match current.rank {
            Some(rank) => println!("Your rank: #{rank}"),
            None => println!("You are not on this leaderboard."),
        }
    }
    pagination(&l.pagination);
}

pub fn private_leaderboards(b: &model::PrivateLeaderboards) {
    let mut rows = vec![vec![
        "name".to_string(),
        "members".to_string(),
        "range".to_string(),
        "id".to_string(),
    ]];
    rows.extend(b.data.iter().map(|l| {
        vec![
            opt(&l.name).to_string(),
            l.members_count
                .map(|c| c.to_string())
                .unwrap_or_else(|| "-".to_string()),
            opt(&l.time_range).to_string(),
            l.id.clone(),
        ]
    }));
    table(&rows);
    pagination(&b.pagination);
}

pub fn machine_names(m: &model::MachineNames) {
    let mut rows = vec![vec![
        "name".to_string(),
        "ip".to_string(),
        "last seen".to_string(),
        "id".to_string(),
    ]];
    rows.extend(m.data.iter().map(|machine| {
        vec![
            machine
                .name
                .clone()
                .or_else(|| machine.value.clone())
                .unwrap_or_else(|| "-".to_string()),
            opt(&machine.ip).to_string(),
            opt(&machine.last_seen_at).to_string(),
            machine.id.clone(),
        ]
    }));
    table(&rows);
    pagination(&m.pagination);
}

pub fn user_agents(u: &model::UserAgents) {
    let mut rows = vec![vec![
        "editor".to_string(),
        "os".to_string(),
        "version".to_string(),
        "last seen".to_string(),
        "id".to_string(),
    ]];
    rows.extend(u.data.iter().map(|agent| {
        vec![
            opt(&agent.editor).to_string(),
            opt(&agent.os).to_string(),
            opt(&agent.version).to_string(),
            opt(&agent.last_seen_at).to_string(),
            agent.id.clone(),
        ]
    }));
    table(&rows);
    pagination(&u.pagination);
}

pub fn editors(e: &model::Editors) {
    let mut rows = vec![vec![
        "name".to_string(),
        "plugin version".to_string(),
        "released".to_string(),
        "website".to_string(),
    ]];
    rows.extend(e.data.iter().map(|editor| {
        vec![
            editor.name.clone(),
            opt(&editor.version).to_string(),
            editor
                .released
                .map(|r| if r { "yes" } else { "no" }.to_string())
                .unwrap_or_else(|| "-".to_string()),
            opt(&editor.website).to_string(),
        ]
    }));
    table(&rows);
}

pub fn program_languages(l: &model::ProgramLanguages) {
    let mut rows = vec![vec![
        "name".to_string(),
        "extensions".to_string(),
        "verified".to_string(),
    ]];
    rows.extend(l.data.iter().map(|lang| {
        vec![
            lang.name.clone(),
            lang.extensions
                .as_ref()
                .map(|e| e.join(", "))
                .unwrap_or_default(),
            lang.is_verified
                .map(|v| if v { "yes" } else { "no" }.to_string())
                .unwrap_or_default(),
        ]
    }));
    table(&rows);
    pagination(&l.pagination);
}

pub fn meta(m: &model::Meta) {
    let sections: [(&str, Option<&model::meta::MetaIpList>); 3] = match &m.ips {
        Some(ips) => [
            ("api", ips.api.as_ref()),
            ("website", ips.website.as_ref()),
            ("worker", ips.worker.as_ref()),
        ],
        None => [("api", None), ("website", None), ("worker", None)],
    };
    for (name, list) in sections {
        let Some(list) = list else { continue };
        if let Some(description) = m.ip_descriptions.as_ref().and_then(|d| d.get(name)) {
            println!("{name}: {description}");
        } else {
            println!("{name}:");
        }
        for ip in list.v4.iter().flatten().chain(list.v6.iter().flatten()) {
            println!("  {ip}");
        }
    }
    if let Some(modified) = &m.last_modified_at {
        println!("Last modified: {modified}");
    }
}

pub fn status_bar(s: &model::StatusBar) {
    println!("Today ({}): {}", s.data.range.date, s.data.grand_total.text);
    if let Some(projects) = &s.data.projects {
        let entries: Vec<_> = projects
            .iter()
            .map(|p| (p.name.clone(), p.total_seconds, Some(p.percent)))
            .collect();
        breakdown("Projects", &entries);
    }
    if let Some(languages) = &s.data.languages {
        let entries: Vec<_> = languages
            .iter()
            .map(|l| (l.name.clone(), l.total_seconds, Some(l.percent)))
            .collect();
        breakdown("Languages", &entries);
    }
    if let Some(cached_at) = &s.cached_at {
        println!();
        println!("Cached at {cached_at}");
    }
}

pub fn heartbeats(h: &model::Heartbeats) {
    let mut rows = vec![vec![
        "time".to_string(),
        "entity".to_string(),
        "type".to_string(),
        "project".to_string(),
        "id".to_string(),
    ]];
    rows.extend(h.data.iter().map(|hb| {
        vec![
            epoch_hms(hb.time),
            hb.entity.clone(),
            hb.r#type.clone(),
            opt(&hb.project).to_string(),
            hb.id.clone(),
        ]
    }));
    table(&rows);
    println!();
    println!("{} heartbeats", h.data.len());
}

pub fn created_heartbeat(h: &model::CreatedHeartbeat) {
    println!("Created heartbeat {}.", h.id);
}

pub fn external_durations(d: &model::ExternalDurations) {
    let mut rows = vec![vec![
        "start".to_string(),
        "end".to_string(),
        "entity".to_string(),
        "provider".to_string(),
        "project".to_string(),
        "id".to_string(),
    ]];
    rows.extend(d.data.iter().map(|duration| {
        vec![
            duration
                .start_time
                .map(epoch_hms)
                .unwrap_or_else(|| "-".to_string()),
            duration
                .end_time
                .map(epoch_hms)
                .unwrap_or_else(|| "-".to_string()),
            opt(&duration.entity).to_string(),
            opt(&duration.provider).to_string(),
            opt(&duration.project).to_string(),
            duration.id.clone(),
        ]
    }));
    table(&rows);
    println!();
    println!("{} external durations", d.data.len());
}

pub fn external_duration(d: &model::ExternalDuration) {
    println!("Created external duration {}.", d.id);
}

fn data_dump_rows(dumps: &[model::DataDump]) -> Vec<Vec<String>> {
    let mut rows = vec![vec![
        "type".to_string(),
        "status".to_string(),
        "progress".to_string(),
        "created".to_string(),
        "download".to_string(),
    ]];
    rows.extend(dumps.iter().map(|d| {
        vec![
            opt(&d.r#type).to_string(),
            opt(&d.status).to_string(),
            d.percent_complete
                .map(|p| format!("{p:.0}%"))
                .unwrap_or_else(|| "-".to_string()),
            opt(&d.created_at).to_string(),
            opt(&d.download_url).to_string(),
        ]
    }));
    rows
}

pub fn data_dumps(d: &model::DataDumps) {
    table(&data_dump_rows(&d.data));
}

pub fn data_dump(d: &model::DataDump) {
    println!("Requested data dump {}.", d.id);
    table(&data_dump_rows(std::slice::from_ref(d)));
}

fn custom_rule_row(
    id: Option<&str>,
    action: &Option<String>,
    source: &Option<String>,
    operation: &Option<String>,
    source_value: &Option<String>,
    destinations: &Option<Vec<model::custom_rules::CustomRuleDestination>>,
    priority: Option<i64>,
) -> Vec<String> {
    let destinations = destinations
        .as_ref()
        .map(|d| {
            d.iter()
                .map(|dest| {
                    format!(
                        "{} = {}",
                        dest.destination.as_deref().unwrap_or("?"),
                        dest.destination_value.as_deref().unwrap_or("?")
                    )
                })
                .collect::<Vec<_>>()
                .join("; ")
        })
        .unwrap_or_default();
    vec![
        priority
            .map(|p| p.to_string())
            .unwrap_or_else(|| "-".to_string()),
        action.as_deref().unwrap_or("-").to_string(),
        format!(
            "{} {} \"{}\"",
            source.as_deref().unwrap_or("?"),
            operation.as_deref().unwrap_or("?"),
            source_value.as_deref().unwrap_or("?")
        ),
        destinations,
        id.unwrap_or("-").to_string(),
    ]
}

fn custom_rule_header() -> Vec<String> {
    vec![
        "priority".to_string(),
        "action".to_string(),
        "condition".to_string(),
        "changes".to_string(),
        "id".to_string(),
    ]
}

pub fn custom_rules(r: &model::CustomRules) {
    let mut rows = vec![custom_rule_header()];
    rows.extend(r.data.iter().map(|rule| {
        custom_rule_row(
            Some(&rule.id),
            &rule.action,
            &rule.source,
            &rule.operation,
            &rule.source_value,
            &rule.destinations,
            rule.priority,
        )
    }));
    table(&rows);
    if let Some(job_id) = &r.job_id {
        println!();
        println!("A background job is applying rule changes: {job_id}");
    }
}

pub fn custom_rule_changes(c: &model::CustomRulesChanges) {
    if let Some(changes) = &c.changes {
        let count = |v: &Option<Vec<serde_json::Value>>| v.as_ref().map(Vec::len).unwrap_or(0);
        println!(
            "Custom rules replaced: {} added, {} removed, {} rearranged.",
            count(&changes.added),
            count(&changes.removed),
            count(&changes.rearranged)
        );
    } else {
        println!("Custom rules replaced.");
    }
    if let Some(job_id) = &c.job_id {
        println!("Background job applying the changes: {job_id}");
    }
}

pub fn custom_rules_progress(p: &model::CustomRulesProgress) {
    match (p.progress, &p.job_id) {
        (Some(progress), Some(job_id)) => println!("Job {job_id}: {progress}% complete."),
        (Some(progress), None) => println!("{progress}% complete."),
        (None, Some(job_id)) => println!("Job {job_id}: no progress reported."),
        (None, None) => println!("No progress reported."),
    }
}

pub fn orgs(o: &model::Orgs) {
    let mut rows = vec![vec![
        "name".to_string(),
        "members".to_string(),
        "timezone".to_string(),
        "id".to_string(),
    ]];
    rows.extend(o.data.iter().map(|org| {
        vec![
            opt(&org.name).to_string(),
            org.people_count
                .map(|c| c.to_string())
                .unwrap_or_else(|| "-".to_string()),
            opt(&org.timezone).to_string(),
            org.id.clone(),
        ]
    }));
    table(&rows);
    pagination(&o.pagination);
}

pub fn org_dashboards(d: &model::OrgDashboards) {
    let mut rows = vec![vec![
        "name".to_string(),
        "members".to_string(),
        "timezone".to_string(),
        "id".to_string(),
    ]];
    rows.extend(d.data.iter().map(|dashboard| {
        vec![
            opt(&dashboard.full_name).to_string(),
            dashboard
                .members_count
                .map(|c| c.to_string())
                .unwrap_or_else(|| "-".to_string()),
            opt(&dashboard.timezone).to_string(),
            dashboard.id.clone(),
        ]
    }));
    table(&rows);
    pagination(&d.pagination);
}

fn member_name(member: &model::org_dashboards::DashboardMember) -> String {
    member
        .full_name
        .clone()
        .or_else(|| member.username.clone())
        .or_else(|| member.email.clone())
        .unwrap_or_else(|| member.id.clone())
}

pub fn org_dashboard_members(m: &model::OrgDashboardMembers) {
    let mut rows = vec![vec![
        "name".to_string(),
        "email".to_string(),
        "view only".to_string(),
        "id".to_string(),
    ]];
    rows.extend(m.data.iter().map(|member| {
        vec![
            member_name(member),
            opt(&member.email).to_string(),
            member
                .is_view_only
                .map(|v| if v { "yes" } else { "no" }.to_string())
                .unwrap_or_else(|| "-".to_string()),
            member.id.clone(),
        ]
    }));
    table(&rows);
    pagination(&m.pagination);
}

pub fn org_dashboard_durations(d: &model::OrgDashboardDurations) {
    for member_durations in &d.data {
        let total: f64 = member_durations.durations.iter().map(|d| d.duration).sum();
        println!(
            "{}: {} over {} durations",
            member_name(&member_durations.member),
            hms(total),
            member_durations.durations.len()
        );
    }
}

fn org_summary_line(s: &model::org_dashboard_summaries::OrgSummary) -> Vec<String> {
    vec![
        s.member
            .as_ref()
            .map(member_name)
            .unwrap_or_else(|| s.range.date.clone()),
        s.grand_total.text.clone(),
        s.projects
            .as_ref()
            .map(|p| top_name(p.iter().map(|p| (p.name.as_str(), p.total_seconds))).to_string())
            .unwrap_or_else(|| "-".to_string()),
    ]
}

pub fn org_dashboard_summaries(s: &model::OrgDashboardSummaries) {
    let mut rows = vec![vec![
        "member".to_string(),
        "total".to_string(),
        "top project".to_string(),
    ]];
    rows.extend(s.data.iter().map(org_summary_line));
    table(&rows);
    if let Some(total) = &s.cumulative_total {
        println!();
        println!(
            "Cumulative total: {}",
            total.text.clone().unwrap_or_else(|| hms(total.seconds))
        );
    }
}

pub fn org_member_summaries(s: &model::OrgMemberSummaries) {
    let mut rows = vec![vec![
        "date".to_string(),
        "total".to_string(),
        "top project".to_string(),
    ]];
    rows.extend(s.data.iter().map(|summary| {
        vec![
            summary.range.date.clone(),
            summary.grand_total.text.clone(),
            summary
                .projects
                .as_ref()
                .map(|p| top_name(p.iter().map(|p| (p.name.as_str(), p.total_seconds))).to_string())
                .unwrap_or_else(|| "-".to_string()),
        ]
    }));
    table(&rows);
    if let Some(total) = &s.cumulative_total {
        println!();
        println!(
            "Cumulative total: {}",
            total.text.clone().unwrap_or_else(|| hms(total.seconds))
        );
    }
    if let Some(avg) = &s.daily_average {
        println!(
            "Daily average:    {} over {} active days",
            avg.text, avg.days_minus_holidays
        );
    }
}

pub fn org_custom_rules(r: &model::OrgCustomRules) {
    let mut rows = vec![custom_rule_header()];
    rows.extend(r.data.iter().map(|rule| {
        custom_rule_row(
            rule.id.as_deref(),
            &rule.action,
            &rule.source,
            &rule.operation,
            &rule.source_value,
            &rule.destinations,
            rule.priority,
        )
    }));
    table(&rows);
}

pub fn json_value(v: &serde_json::Value) {
    println!("{}", serde_json::to_string_pretty(v).unwrap_or_default());
}
