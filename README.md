# Guild Application

A scalable application featuring a 3D character in a virtual space with task management capabilities. The application consists of a Flutter frontend and a Rust backend with PostgreSQL database.

## Project Structure

- **Flutter App**: Flutter-based front-end application with support for multiple platforms.
- **Rust Backend**: Actix-based API server with PostgreSQL database.
- **Blender Models**: 3D models for character and room environment.
- **API Specification**: OpenAPI 3.0 specification for contract-first design.

## Features

- 3D character that can walk around via keyboard controls (ASDW) or mouse clicks
- Interactive room environment with a quest board, laptop, webcam, and turntables
- Character abilities including shadow clone and shadow minions
- Task management system with quest board
- Guild and character customization

## Getting Started

### Prerequisites

- Flutter (latest stable version)
- Rust (latest stable version)
- PostgreSQL
- Liquibase (for database migrations)
- Blender (for 3D model editing)

### Setup

#### Database Setup

1. Install PostgreSQL
2. Create a database named `guild_db`
3. Run the Liquibase migrations:

```bash
cd backend/liquibase
liquibase --changeLogFile=changelog.xml update
```

#### Backend Setup

1. Navigate to the backend directory:

```bash
cd backend
```

2. Create a `.env` file with the following content:

```
DATABASE_URL=postgres://postgres:postgres@localhost/guild_db
HOST=127.0.0.1
PORT=8080
```

3. Run the Rust server:

```bash
cargo run
```

#### Frontend Setup

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

## Project Requirements

See [Instructions.md](whatWasAsked/Instructions.md) for detailed project requirements.

## API Documentation

The API is documented using OpenAPI 3.0. The specification can be found in [docs/api/openapi.yaml](docs/api/openapi.yaml).

## 3D Models

For information about the 3D models, see [blender_models/README.md](blender_models/README.md).

## License

This project is licensed under the MIT License. 