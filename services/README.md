# Guilds Microservices Architecture

This directory contains the microservices that power the Guilds application. Each service is responsible for a specific domain within the system.

## Services Overview

### API Gateway
The API Gateway serves as the entry point for all client requests. It routes requests to the appropriate microservice based on the URL path. It also handles cross-cutting concerns such as authentication, authorization, and rate limiting.

### Quest Service
The Quest Service manages all quest-related functionality, including:
- Creating and updating quests
- Assigning quests to characters or guilds
- Quest completion and rewards calculation
- Search and filtering of available quests

### Guild Service
The Guild Service manages all guild and town-related functionality, including:
- Guild creation and management
- Guild membership
- Town data and relationships with guilds
- Guild-based quest assignments

### Character Service
The Character Service manages all character-related functionality, including:
- Character creation and management
- Skills and progression
- Character-to-guild relationships
- Character-based quest assignments

### Reward Service
The Reward Service manages all reward and inventory-related functionality, including:
- Inventory management for characters and guilds
- Currency tracking and updates
- Reward distribution for completed quests
- Guild stash management

## Architecture Decisions

### Database
All services share a single PostgreSQL database but operate on different schemas for isolation. This approach balances data isolation with operational simplicity.

### Communication
Services communicate via HTTP/REST when cross-service calls are needed. Each service exposes a well-defined API.

### Deployment
All services are containerized using Docker and can be deployed together using docker-compose or individually for development purposes.

### Liquibase
Database schema management is handled through Liquibase, ensuring consistent database schema across all environments.

## Development Workflow

To run the entire system locally:
```
docker-compose up
```

To run an individual service for development:
```
cd services/<service-name>
cargo run
```

## Service Dependencies

- **API Gateway**: Depends on all other services
- **Quest Service**: Depends on Guild Service and Character Service for assignment functionality
- **Guild Service**: Depends on Character Service for membership management
- **Character Service**: No direct service dependencies
- **Reward Service**: Depends on all other services for reward management 