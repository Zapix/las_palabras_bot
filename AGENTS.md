# Repository Guidelines

## Project Structure & Module Organization
- `src/`: Rust application code organized by layers (`api/`, `app/`, `domain/`, `infra/`) plus entry points (`main.rs`, `lib.rs`).
- `tests/`: Integration-style tests grouped by area (`tests/api/`, `tests/domain/`).
- `migrations/`: SQLx migration files for Postgres.
- `config/`: Configuration files used by the app.
- `docker-compose.yml`: Local Postgres container for development.
- `git-hooks/`: Pre-commit checks (Clippy + Nextest).

## Build, Test, and Development Commands
- `cargo build --release`: Build optimized binaries.
- `cargo run`: Run the bot locally using `.env` configuration.
- `cargo test`: Run the standard Rust test suite.
- `cargo clippy --all-targets --all-features -- -D warnings`: Lint with Clippy (required by pre-commit).
- `cargo fmt`: Format code with rustfmt.
- `cargo nextest run --no-fail-fast`: Faster test runner used in pre-commit hooks.
- `docker-compose up --build`: Start Postgres for local development.

## Coding Style & Naming Conventions
- Rust standard style: 4-space indentation, rustfmt formatting, Clippy-clean code.
- Naming: `snake_case` for functions/modules, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Keep modules aligned to DDD layers (`domain`, `app`, `infra`, `api`).

## Testing Guidelines
- Primary framework: Rust `#[test]` with `cargo test`; Nextest is used in hooks.
- Place API-focused tests under `tests/api/` and domain tests under `tests/domain/`.
- Name test files and functions descriptively, e.g., `tests/api/verbs.rs` or `test_fetch_verb_details`.

## Commit & Pull Request Guidelines
- Commit messages commonly start with an issue number (e.g., `47 add api endpoint to fetch verbs details`).
- Keep commits small and scoped to a single change.
- PRs should include a clear summary, link related issues, and note any schema changes or migrations.

## Configuration & Secrets
- Provide local configuration via `.env` (e.g., `TELEGRAM_BOT_TOKEN=...`).
- Keep secrets out of version control; prefer environment variables over hardcoded values.
