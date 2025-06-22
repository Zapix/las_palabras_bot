# 🇪🇸 Telegram Spanish Vocabulary Bot

A modular, Domain-Driven Design (DDD) Telegram chatbot built in **Rust** using the **Axum framework**. The bot helps users learn and practice Spanish vocabulary and verb conjugation, with support for Russian translations.

## 🚀 Features

- 📖 Translate words from Spanish ➝ Russian
- 🔄 Translate words from Russian ➝ Spanish
- ⌛ Verb conjugation practice with tenses
- 📊 User performance tracking
- ⚙️ RESTful API to manage vocabulary entries
- 🧱 Clean DDD architecture (domain/app/infra/api split)
- 🐳 Dockerized and ECS-ready
- ☁️ Designed for future AWS Lambda compatibility

---

## 📁 Project Structure

las_parablas_bot/
├── src/
│   ├── main.rs               # ECS entry point
│   ├── lib.rs                # Shared run logic (Lambda compatible)
│   ├── config.rs             # Env configuration loader
│   ├── telemetry.rs          # Logging setup
│
│   ├── api/                  # Axum HTTP routes
│   │   ├── routes.rs
│   │   └── handlers/         # HTTP handlers
│
│   ├── app/                  # Application (use cases)
│   │   ├── vocab_service.rs
│   │   └── user_stats_service.rs
│
│   ├── domain/               # Domain models and interfaces
│   │   ├── vocab/
│   │   └── stats/
│
│   └── infra/                # Infrastructure layer
│       ├── postgres/         # Postgres repos
│       └── telegram/         # Telegram client integration


## Getting Started

### Prerequisites

- Rust (latest stable version)
- Docker (for containerization)
- PostgreSQL (for local development)
- Telegram Bot API token

### Installation

1. Clone the repository:
   ```bash
   git clone
   ```
2. Navigate to the project directory:
   ```bash
   cd las_parablas_bot
   ```
3. Set up environment variables:
    Create a `.env` file in the root directory with your Telegram Bot API token and other configurations:
    ```plaintext
    TELEGRAM_BOT_TOKEN=your_token_here
    ```
4. Build the project::
    ```bash
    cargo build --release
    ```
5. Run the application:
    ```bash
    cargo run
    ```

### Development
For local development, you can use Docker to run the application and PostgreSQL database:
```bash
docker-compose up --build
```
### Testing
Run the test suite to ensure everything is working correctly:
```bash
cargo test
```

### Git Hooks

To ensure code quality, this project uses Git hooks for formatting and linting. Make sure to install the hooks after cloning the repository:
```bash
ln -s ./git-hooks/pre-commit .git/hooks/pre-commit
```

