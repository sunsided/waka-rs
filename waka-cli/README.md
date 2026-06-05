# waka-cli

[![Crates.io](https://img.shields.io/crates/v/waka-cli)](https://crates.io/crates/waka-cli)
[![CI](https://github.com/sunsided/waka-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/sunsided/waka-rs/actions/workflows/rust.yml)
[![License: EUPL-1.2](https://img.shields.io/badge/license-EUPL--1.2-blue)](https://github.com/sunsided/waka-rs/blob/main/LICENSE.md)

A command-line client for the [WakaTime](https://wakatime.com/) API, built on the
[`waka`](https://crates.io/crates/waka) crate. It covers every documented endpoint:
coding statistics, summaries, durations, heartbeats, goals, insights, leaderboards,
organizations, and the write operations.

## Installation

With a Rust toolchain:

```sh
cargo install waka-cli
```

Or grab a prebuilt binary from the
[GitHub releases](https://github.com/sunsided/waka-rs/releases): each release
ships archives (with SHA-256 checksums) for

- Linux: `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl` (static)
- macOS: `aarch64-apple-darwin` (Apple Silicon), `x86_64-apple-darwin` (Intel)
- Windows: `x86_64-pc-windows-msvc`

## Authentication

Get your API key from [wakatime.com/api-key](https://wakatime.com/api-key) and provide
it via a flag, an environment variable, or a `.env` file in the working directory:

```sh
# flag
waka-cli --api-key 'waka_…' user

# environment
export WAKATIME_API_KEY='waka_…'
waka-cli user

# .env file
echo "WAKATIME_API_KEY=waka_…" > .env
waka-cli user
```

OAuth 2.0 access tokens are supported via `--bearer-token` / `WAKATIME_BEARER_TOKEN`.

### Environment variables

| Variable | Flag | Description |
|---|---|---|
| `WAKATIME_API_KEY` | `--api-key` | API key, sent as HTTP Basic auth |
| `WAKATIME_BEARER_TOKEN` | `--bearer-token` | OAuth 2.0 access token, sent as Bearer auth |
| `WAKATIME_USER` | `--user` | The user to query; defaults to `current` |
| `WAKATIME_BASE_URL` | `--base-url` | Override the API base URL |
| `WAKATIME_TIMEOUT` | `--timeout` | Request timeout in seconds |

## Usage

```sh
# Your coding stats for the last week
waka-cli stats last_7_days

# Daily summaries for a date range
waka-cli summaries 2026-01-01 2026-01-08

# Today's activity, as shown in editor status bars
waka-cli status-bar

# All projects, across all pages
waka-cli projects --all

# Send a heartbeat
waka-cli heartbeats send --entity /path/to/file.rs --type file --project my-project

# Log an external duration, e.g. a meeting
waka-cli external-durations send --external-id standup-42 --entity "Daily standup" \
    --type event --category meeting --start 1750000000 --end 1750000900

# Replace custom rules from a JSON file (or stdin)
waka-cli custom-rules set --file rules.json

# Request a data dump export
waka-cli data-dumps create daily
```

Output is a compact human-readable rendering, e.g.:

```text
$ waka-cli stats last_7_days
Stats for Last 7 Days
Total:         32h 07m 41s
Daily average: 4h 35m 23s
Days:          7 (6 active)
Best day:      2026-06-02 (8 hrs 12 mins)

Languages:
Rust      24h 02m 11s  74.8%
Markdown   3h 41m 09s  11.5%
TOML       1h 12m 54s   3.8%
```

Run `waka-cli --help` for the full list of subcommands, and
`waka-cli <command> --help` for the options of each command.

### Subcommands

| Command | Description |
|---|---|
| `user` | The user's profile |
| `all-time` | Total time logged since account creation |
| `stats`, `stats-aggregated` | Coding activity stats; aggregate stats of all users |
| `summaries` | Daily summaries for a date range |
| `durations` | A single day as an array of durations |
| `insights` | Insights such as `best_day`, `weekdays`, `languages` |
| `projects`, `commits`, `commit` | Projects and per-commit coding activity |
| `goals`, `goal` | Coding goals |
| `leaders`, `leaderboards`, `leaderboard` | Public and private leaderboards |
| `machine-names`, `user-agents` | Machines and plugins seen |
| `editors`, `program-languages`, `meta` | WakaTime service data |
| `status-bar` | Today's activity for editor status bars |
| `heartbeats list\|send\|send-bulk\|delete` | Inspect, send or delete heartbeats |
| `external-durations list\|send\|send-bulk\|delete` | Log time from external apps |
| `data-dumps list\|create` | Data dump exports |
| `custom-rules list\|set\|delete\|progress\|clear-progress` | Custom rules |
| `org list\|dashboards\|members\|durations\|summaries\|…` | Organization dashboards |
| `completions` | Shell completion scripts |

## Shell completions

Generate completions for your shell with the `completions` subcommand
(supported: `bash`, `zsh`, `fish`, `elvish`, `powershell`):

```sh
# bash
waka-cli completions bash > ~/.local/share/bash-completion/completions/waka-cli

# zsh
waka-cli completions zsh > ~/.zfunc/_waka-cli

# fish
waka-cli completions fish > ~/.config/fish/completions/waka-cli.fish
```

## JSON output

Every command supports `--json`, which prints the raw API response as
pretty-printed JSON to stdout instead of the human-readable rendering:

```sh
waka-cli stats last_7_days --json | jq '.languages[0]'
waka-cli summaries 2026-01-01 2026-01-08 --json > summaries.json
```

## License

Licensed under the European Union Public Licence (EUPL), Version 1.2.
