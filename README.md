# Guilds

A management application for guilds, quests, characters, and rewards with a Flutter frontend and Rust microservices backend.

## Project Structure

The project is organized with a Flutter frontend and a microservices backend architecture:

```
guilds/
├── flutter_app/        # Flutter frontend application
├── api/                # Legacy API (monolithic, being phased out)
├── services/           # Microservices architecture
│   ├── api-gateway/    # API Gateway service
│   ├── quest-service/  # Quest management service 
│   ├── (other services to be implemented)
├── SECRETS/            # Contains sensitive configuration (not in Git)
└── SECRETS_TEMPLATE/   # Templates for sensitive configuration
```

## Environment Setup

### Prerequisites

- Flutter (latest stable version)
- Rust 1.70+
- Docker and docker-compose
- PostgreSQL 15+

### Secrets Management

The project uses a secrets management approach where sensitive information is stored in a `SECRETS` directory that is not committed to Git. There is a `SECRETS_TEMPLATE` directory that contains templates for the files needed in `SECRETS`.

Before running the application, copy the template files from `SECRETS_TEMPLATE` to `SECRETS` and fill in the actual values:

```bash
# First time setup
cp -r SECRETS_TEMPLATE/* SECRETS/
# Then edit each file to provide actual credentials
```

## Development

### Frontend Setup (Flutter)

1. Navigate to the Flutter app directory:

```bash
cd flutter_app
```

2. Install dependencies:

```bash
flutter pub get
```

3. Run the application:

```bash
# For mobile/desktop:
flutter run -t lib/main.dart

# For web:
flutter run -t lib/web_main.dart -d chrome
```

The Flutter app is designed to run outside of Docker and connect to the backend services running in containers.

### Backend Setup (Microservices)

To run the entire backend:

```bash
docker-compose up
```

To run a specific service:

```bash
cd services/SERVICENAME
cargo run
```

### API Gateway

The API Gateway serves as the entry point for all client requests. It routes requests to the appropriate microservice and handles cross-cutting concerns like authentication.

Accessible at: http://localhost:8000

### Quest Service

The Quest Service manages the creation, retrieval, update, and deletion of quests. It provides APIs for quest management and maintains its own database schema.

## Connecting Flutter to Backend

The Flutter app is configured to connect to the API Gateway at `http://localhost:8000`. This is the single entry point for all backend services. You can modify the API endpoint in the Flutter app configuration if needed.

## Features

- 3D character that can walk around via keyboard controls (ASDW) or mouse clicks
- Interactive room environment with a quest board, laptop, webcam, and turntables
- Character abilities including shadow clone and shadow minions
- Task management system with quest board
- Guild and character customization
- Character Management: Create and manage characters with skills, levels, and properties
- Quest System: Create, assign, and complete quests with skill requirements
- Skill Framework: Define skills with categories and levels for characters
- Guild System: Form guilds, manage memberships, and collaborate on quests
- Town System: Explore different towns with unique properties and guild presence

## Database Migrations

Database migrations are managed using Liquibase. The changelog files are in each service's `liquibase` directory.

The migrations run automatically when using Docker, but can also be run manually:

```bash
cd services/quest-service
liquibase --changeLogFile=liquibase/master.xml update
```

## Testing

To test the backend services:

```bash
cd services/SERVICE_NAME
cargo test
```

To test the Flutter app:

```bash
cd flutter_app
flutter test
```

## License

This project is licensed under the MIT License. See the LICENSE file for details. 