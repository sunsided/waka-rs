# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2026-06-05

### Added

- New `waka-cli` crate: a clap-based command-line client covering every
  endpoint of the library, with `.env` support (dotenvy), environment
  variable configuration (`WAKATIME_API_KEY`, `WAKATIME_BEARER_TOKEN`,
  `WAKATIME_USER`, `WAKATIME_BASE_URL`, `WAKATIME_TIMEOUT`), human-readable
  output by default and a `--json` flag for raw API responses. Shell
  completions are available via the `completions` subcommand.

### Changed

- The repository is now a Cargo workspace; the library lives in `waka/`
  and shared metadata and dependency versions are declared at the root.
- Upgraded `query-string-builder` to 0.7 and adopted its zero-allocation
  borrowing builder; query strings now render without per-pair string
  allocations. No public API changes.
- MSRV raised to 1.87, following the upgraded dependency.

## [0.2.0] - 2026-06-05

### Added

- Full coverage of the documented WakaTime API. New read endpoints:
  `stats`, `projects`, `durations`, `heartbeats`, `user`, `commits` (list),
  `goals`, `goal`, `insights`, `leaders`, `machine_names`, `user_agents`,
  `status_bar_today`, `custom_rules`, `custom_rules_progress`, `data_dumps`,
  `external_durations`, `editors`, `program_languages`, `meta`,
  `stats_aggregated`, `private_leaderboards`, `private_leaderboard_leaders`,
  and the organization cluster (`orgs`, `org_dashboards`,
  `org_dashboard_members`, `org_dashboard_durations`,
  `org_dashboard_summaries`, `org_dashboard_member_durations`,
  `org_dashboard_member_summaries`, `org_custom_rules`).
- Write endpoints: `send_heartbeat`, `send_heartbeats`, `delete_heartbeats`,
  `send_external_duration`, `send_external_durations`,
  `delete_external_durations`, `set_custom_rules`, `delete_custom_rule`,
  `delete_custom_rules_progress`, `create_data_dump`. These follow the
  developer docs and are tested against mocks only.
- OAuth 2.0 bearer token authentication via
  `WakaTimeClientBuilder::new_with_bearer_token`.
- `WakaTimeClientBuilder::with_timeout` for per-request timeouts and
  `with_base_url` for pointing the client at a different host.
- `Range` and `InsightType` enums; the `stats`, `insights`, and
  `stats_aggregated` methods accept them or plain strings.
- `projects_all` and `commits_all` helpers that fetch every page.
- Fine-grained error variants: `PaymentRequired`, `Forbidden`, `NotFound`,
  `RateLimited` (with the parsed `Retry-After` header), and `Timeout`.
  Error displays include the API's error messages.
- Mocked integration test suite covering every endpoint and error path.

### Fixed

- User-provided URL path parameters (project names, commit hashes, org and
  dashboard identifiers, …) are now percent-encoded, so values containing
  `/`, `?`, `#`, spaces, or other special characters produce correct
  request URLs.

### Changed

- **Breaking:** upgraded reqwest to 0.13, whose types appear in the public
  error enums; the default TLS backend is now rustls.
- **Breaking:** `ApiError` and `BuilderError` are `#[non_exhaustive]`;
  matches need a wildcard arm.
- **Breaking:** `ErrorsResponse` gained an `error` field for the API's
  singular error body form.
- The commits list deserializes the live API's `commits` key as well as
  the documented `data` key.
- **Breaking:** standardized numeric model types: `total_seconds`,
  `percent`, and fractional `seconds` fields are now `f64` (previously
  `f32` in the summaries and all-time models), and the `minutes`/`seconds`
  clock fields are now `u32` (previously `u8`).
- All public items now carry documentation; the crate denies
  `missing_docs`.
- Upgraded to Rust edition 2024; MSRV is 1.86.

## [0.1.0] - 2023-07-09

### Added

- Initial release with `summaries`, `all_time_since_today`, and `commit`
  endpoints, authenticated via API key.

[0.2.1]: https://github.com/sunsided/waka-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/sunsided/waka-rs/compare/0.1.0...v0.2.0
[0.1.0]: https://github.com/sunsided/waka-rs/releases/tag/0.1.0
