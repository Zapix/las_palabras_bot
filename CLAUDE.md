# CLAUDE.md

Guidance for working in this repository. Derived from `AGENTS.md`, corrected and expanded
against the current code.

## What this is

`las_palabras_bot` — a Rust service for a Spanish/Russian vocabulary learning game
(Telegram bot planned; today the deliverable is the HTTP API). Postgres persistence via
`sqlx`, HTTP via **actix-web** (note: `README.md` and older docs mention Axum — that is
stale, the app is actix-web), OpenAPI/Swagger via `utoipa`.

## Project structure

- `src/main.rs` — binary entry point: loads `.env`, sets up tracing, runs `Application`.
- `src/lib.rs` — module root (`api`, `application`, `configuration`, `domain`, `telemetry`).
- `src/application.rs` — `Application`: binds the `TcpListener`, builds the actix `App`,
  registers **every route** and the `ApiDoc` OpenAPI path list. New endpoints must be added
  in both places here.
- `src/api/` — HTTP layer, one directory per resource (`games/`, `verbs/`, `vocabulary/`)
  plus `info.rs`, `pagination.rs`, and `health` in `mod.rs`. One handler per file; the
  resource `mod.rs` re-exports handlers (`pub use create::create_game;`).
- `src/domain/` — domain areas (`vocabulary/`, `verbs/`, `word_game/`). Each holds its
  models and a `repository/` submodule with `traits.rs` (trait), `db.rs` (Postgres impl),
  and sometimes `error.rs` / `filters.rs`, re-exported from `repository/mod.rs`.
- `src/configuration/` — layered settings (`Settings::load`), one struct per file.
- `src/bin/` — small `clap` CLIs that drive the domain directly (`ask_question.rs`,
  `start_game.rs`, `load_verbs.rs`, …). Useful for manual checks without the HTTP layer.
- `tests/api/` — integration tests, one binary rooted at `tests/api/main.rs`; new test
  modules must be declared there (or in the resource's `mod.rs`).
- `migrations/` — sqlx migrations. `.sqlx/` — offline query cache (committed).
- `config/` — `base.yaml` + `<environment>.yaml`. `git-hooks/pre-commit`, `docker-compose.yml`.

There is no `src/app/` or `src/infra/` layer despite what `AGENTS.md` says — use cases live
in the handlers and repositories today.

## Commands

```bash
docker-compose up -d                                        # Postgres for local dev/tests
cargo run                                                   # run the API (reads .env + config/)
cargo build --release
cargo test                                                  # needs a reachable Postgres
cargo nextest run --no-fail-fast                            # used by the pre-commit hook
cargo clippy --all-targets --all-features -- -D warnings    # must be clean
cargo fmt
cargo sqlx prepare                                           # refresh .sqlx after query changes
```

Swagger UI: `http://localhost:8080/swagger-ui/` — OpenAPI JSON at `/api-docs/openapi.json`.

Install the hook once per clone: `ln -s ../../git-hooks/pre-commit .git/hooks/pre-commit`.

## Database and sqlx

- Queries use the compile-time-checked macros (`sqlx::query_as!`), so **the build needs
  either a live `DATABASE_URL` or the committed `.sqlx/` cache**. After adding or changing a
  query, run `cargo sqlx prepare` and commit the `.sqlx/` changes, or the build breaks for
  everyone offline and in CI.
- Adding a column/table means a new pair of files in `migrations/`; the newest migration
  (`game_table`) uses explicit `.up.sql`/`.down.sql`.
- `game.game_status` is a JSONB column deserialized into `Json<GameStatus>`; the state
  machine lives in `src/domain/word_game/game_status.rs` and transitions are enforced in
  `repository/db.rs` inside a transaction.

## Testing

- Integration tests call the real HTTP surface: `spawn_app()` (in `tests/api/helpers.rs`)
  loads settings, **creates a fresh `test_<uuid>` database**, runs migrations, binds port 0,
  and spawns the server. Each test ends with `app.drop_database().await`.
- So integration tests require a running Postgres reachable with `config/` credentials and
  permission to create/drop databases.
- Pure logic (config, enums, response mapping, conjugation) is tested inline with
  `#[cfg(test)] mod tests` next to the code — prefer that over a new integration test when
  no HTTP or DB round-trip is needed.
- Tests are `#[tokio::test]`, named `test_<behavior>` with the expected status for API
  cases, e.g. `test_ask_question_409_for_invalid_transition`.

## Conventions

- Rust standard style: rustfmt, 4-space indent, Clippy-clean (`-D warnings`).
  `snake_case` functions/modules, `CamelCase` types, `SCREAMING_SNAKE_CASE` constants.
- Keep the layering: handlers do HTTP concerns only (extract, call a repository/converter,
  map errors to responses); domain code must not depend on `actix_web`.
- Repositories are borrow-based structs constructed per request: `GameDb::new(db_pool.as_ref())`,
  behind a trait in `traits.rs` so bins and tests can reuse them.
- Errors: domain repositories return `thiserror` enums (`GameRepositoryError`) or
  `anyhow::Result`; the API layer maps them to responses with a local `map_*_error` function
  or a `ResponseError` impl (see `api/vocabulary/detail_word_error.rs`). Never leak internal
  error text — return a generic message with the right status.
- Every handler carries `#[tracing::instrument(..., skip(db_pool), err)]` and a
  `#[utoipa::path(...)]` annotation with all documented statuses and a `tag`.
- JSON payloads/responses are `#[serde(rename_all = "camelCase")]`; list endpoints return
  `Pagination<T>` with `page`/`per_page` query params (`DEFAULT_PAGE = 20`).

## Configuration and secrets

- `Settings::load()` reads `config/base.yaml`, then `config/<APP_ENVIRONMENT>.yaml`
  (default `development`), then `APP_`-prefixed env vars; `version` comes from
  `CARGO_PKG_VERSION`.
- `.env` is git-ignored and currently holds `DATABASE_URL` (for the sqlx macros/CLI) and
  `GITHUB_PAT_TOKEN`; it is loaded by `main.rs` and by `spawn_app()`. The Telegram token and
  DB credentials come from `config/` today with placeholder values — override them via
  `APP_*` env vars rather than committing real ones. Secret fields are wrapped in `secrecy`.

## Commits and PRs

- Commit subjects start with the issue number: `59 ask question in game`.
- Branches follow `<issue>-<slug>`, e.g. `59-ask-question-in-game`.
- Keep commits small and scoped. PRs: clear summary, linked issue, and an explicit note for
  schema changes, migrations, or `.sqlx/` updates.
- CI (`.github/workflows/rust.yaml`) runs `cargo build`, `cargo test`, and Clippy with
  `-D warnings` against a Postgres 14 service.
