# Copilot instructions for Guilds repository

Short, actionable guidance to help AI coding agents be productive in this workspace.

- Purpose: The repo is a Flutter frontend plus Rust microservices backend (Actix/Tokio).

- Big picture:
  - Frontend: `flutter_app/` (Flutter web/mobile). Entry points: `lib/main.dart`, `lib/web_main.dart`.
  - Backend: `services/` contains Rust microservices (api-gateway, quest-service, character-service, guild-service, reward-service). Each service is a separate Cargo project using `actix-web`.
  - Local orchestration: top-level `docker-compose.yml` boots Postgres, Liquibase, and the services. API Gateway is exposed on host port 8080 per `docker-compose.yml`.
  - Database migrations: Liquibase changelogs live under each service's `liquibase` directory (example: `services/quest-service/liquibase/master.xml`).

- How to run (developer flows) — concrete commands:
  - Prepare secrets: copy templates from `SECRETS_TEMPLATE/` into `SECRETS/` and edit values.
    - Example: `cp -r SECRETS_TEMPLATE/* SECRETS/`
  - Run full stack (recommended for integration):
    - `docker-compose up` (from repo root). This runs Postgres, Liquibase, and all services.
  - Run a single Rust service locally (fast iteration):
    - `cd services/quest-service` then `cargo run` (service listens on port configured in its Dockerfile/ENV; docker-compose maps 8081 for quest-service).
  - Run Flutter app locally:
    - `cd flutter_app` then `flutter pub get` and `flutter run -t lib/main.dart` (or `-t lib/web_main.dart -d chrome` for web).

- Tests and verification:
  - Rust unit/integration tests: `cd services/<service>` then `cargo test`.
  - Flutter tests: `cd flutter_app` then `flutter test`.

- Secrets and environment conventions:
  - Secrets are kept under `SECRETS/` (not checked in). Per-service env files live under `SECRETS/services/<service>/` and are referenced by `docker-compose.yml`.
  - Services read `SECRETS_PATH` at runtime; when running locally with `cargo run`, set equivalent env vars or point to local secrets files.

- Project-specific patterns and conventions (do not invent alternatives):
  - Each service is a standalone Cargo package. Use the service folder as the working directory for builds/tests.
  - Inter-service communication is HTTP via the API Gateway. Gateway env vars point to internal Docker hostnames and ports (see `docker-compose.yml`: `QUEST_SERVICE_URL`, `CHARACTER_SERVICE_URL`, etc.).
  - Database migration is centralized in Docker via Liquibase in `docker-compose.yml`. Manual Liquibase runs are supported from service folders (`liquibase --changeLogFile=liquibase/master.xml update`).
  - Persistence options: services support `STORAGE_MODE` env var — default is `postgres`. Do not change storage assumptions without checking `api-gateway` and other services for `STORAGE_MODE` usage.

- Integration points and key files to check when changing functionality:
  - `docker-compose.yml` — service ports, names, env propagation, and volumes.
  - `services/*/Dockerfile` — how each service is built and what files are baked into containers.
  - `services/*/Cargo.toml` — dependencies and runtime crates (Actix, Tokio, Utoipa for OpenAPI).
  - `services/*/liquibase` — migration changelogs.
  - `flutter_app/lib` — API endpoint configuration (search for `localhost` or `8000/8080` URLs if updating networking).

- Quick examples to reference in PRs/edits:
  - To add an API route in a service: follow existing pattern in `services/quest-service/src/` (handlers, models, repository separation). Mirror `utoipa` OpenAPI annotations when adding endpoints.
  - To change database schema: add Liquibase changelog under the service's `liquibase/changelogs` and update `master.xml`.

- When the agent is unsure:
  - Prefer reading `docker-compose.yml` and the service's `Cargo.toml`/`src` before changing runtime behavior.
  - Ask the user before modifying secrets, storage backends, or Docker networking.

Keep responses concise and reference the exact file(s) you examined when suggesting code changes.
