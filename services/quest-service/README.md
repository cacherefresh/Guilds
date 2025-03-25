# Quest Service

The Quest Service is a microservice responsible for quest management in the Guilds application. It provides APIs for creating, retrieving, updating, and deleting quests.

## Features

- Create new quests with title, description, difficulty, required skills, and rewards
- Retrieve quests with filtering and pagination
- Update existing quests
- Delete quests

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET    | /quests  | Get all quests with optional filtering by difficulty and status |
| GET    | /quests/{id} | Get a quest by ID |
| POST   | /quests  | Create a new quest |
| PUT    | /quests/{id} | Update an existing quest |
| DELETE | /quests/{id} | Delete a quest |

## Development

### Prerequisites

- Rust (1.70 or later)
- PostgreSQL (15 or later)

### Local Setup

1. Clone the repository
2. Navigate to the quest-service directory
3. Set up environment variables or config files as needed
4. Run the service:

```
cargo run
```

### Database Migrations

This service uses Liquibase for database migrations. The changelog files are in the `liquibase` directory.

To run migrations manually, use the Liquibase CLI:

```
liquibase --changeLogFile=liquibase/master.xml update
```

### Configuration

Configuration is managed through a combination of:

- JSON configuration files in the `config` directory
- Environment variables (prefixed with `APP_`)

Example configuration settings:

```json
{
  "host": "0.0.0.0",
  "port": 8001,
  "database_url": "postgres://postgres:postgres@localhost:5432/guilds",
  "database_schema": "public"
}
```

### Docker

Build the Docker image:

```
docker build -t quest-service .
```

Run the container:

```
docker run -p 8001:8001 quest-service
``` 