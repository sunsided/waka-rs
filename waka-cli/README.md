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

WakaTime API keys are **write-only**: they can send heartbeats, but reading
stats, summaries and everything else requires signing in with OAuth 2.0.

### Signing in with OAuth

WakaTime needs a client secret for the sign-in and doesn't support PKCE, so
`waka-cli` can't ship shared app credentials. Register your own app once:

1. Create an app at [wakatime.com/apps](https://wakatime.com/apps).
2. Add `http://127.0.0.1:8737/callback` to its authorized redirect URLs.
3. Copy its App ID and App Secret.

Then sign in:

```sh
export WAKATIME_CLIENT_ID='…'      # App ID
export WAKATIME_CLIENT_SECRET='…'  # App Secret
waka-cli auth login
```

This opens the browser, waits for WakaTime to redirect back to the local
address, and stores the token with the app credentials in the OS keyring
(Keychain, Credential Manager, or Secret Service). Where no keyring is
available it falls back to `credentials.json` in the user's config directory
(e.g. `~/.config/waka-cli/`), readable only by you. Use `--credentials-file`
(`WAKATIME_CREDENTIALS_FILE`) to always use a specific file.

```sh
waka-cli auth status    # which credentials are in use; the stored token's scopes
waka-cli auth refresh   # replace the access token using the refresh token
waka-cli auth logout    # revoke the token and delete it
```

- **Scopes:** by default, `login` requests read access to everything
  (`read_summaries,read_stats,read_goals,read_heartbeats,read_orgs,read_private_leaderboards,email`).
  Pick others with `--scope`, e.g. `--scope read_stats,write_heartbeats`.
- **Remote machines:** with `--manual`, paste the URL you were redirected to
  (or just the code) instead of having `waka-cli` listen for the redirect.
  This also works with a non-local `--redirect-uri`, such as
  `https://wakatime.com/oauth/test`, if you add it to your app.
- **Token limit:** WakaTime keeps at most 8 active tokens per app and user;
  a 9th sign-in revokes the oldest.

### API keys and tokens

An API key ([wakatime.com/api-key](https://wakatime.com/api-key)) still works
for sending heartbeats. An existing OAuth access token can be passed directly.
Both are given via a flag, an environment variable, or a `.env` file in the
working directory:

```sh
waka-cli --api-key 'waka_…' heartbeats send …
echo "WAKATIME_API_KEY=waka_…" > .env
waka-cli --bearer-token 'waka_tok_…' stats last_7_days
```

Credentials are picked in this order: `--bearer-token` / `WAKATIME_BEARER_TOKEN`,
then `--api-key` passed as a flag, then the token stored by `auth login`, and
finally `WAKATIME_API_KEY` from the environment or `.env`. That way a
write-only key in `.env` doesn't shadow a sign-in.

### Environment variables

A `.env` file may set credentials and settings, but not `WAKATIME_BASE_URL`,
`WAKATIME_CREDENTIALS_FILE` or `WAKATIME_REDIRECT_URI`: `waka-cli` refuses to
run if it does. These decide where credentials are sent, and `.env` is read
from the working directory, which may be an untrusted checkout. Set them in
the shell environment or pass the flag instead.

| Variable | Flag | Description |
|---|---|---|
| `WAKATIME_API_KEY` | `--api-key` | API key, sent as HTTP Basic auth; write-only |
| `WAKATIME_BEARER_TOKEN` | `--bearer-token` | OAuth 2.0 access token, sent as Bearer auth |
| `WAKATIME_CLIENT_ID` | `auth login --client-id` | App ID of your OAuth app |
| `WAKATIME_CLIENT_SECRET` | `auth login --client-secret` | App secret of your OAuth app |
| `WAKATIME_REDIRECT_URI` | `auth login --redirect-uri` | Redirect URI; defaults to `http://127.0.0.1:8737/callback` |
| `WAKATIME_CREDENTIALS_FILE` | `--credentials-file` | Store the OAuth token in this file instead of the OS keyring |
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
