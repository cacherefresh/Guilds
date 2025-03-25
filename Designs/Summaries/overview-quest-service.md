# Quest Service Architecture Overview

## Core Components

The Quest Service is a microservice responsible for managing all quest-related functionality:

- **Quest Management**: CRUD operations for quests, subtasks, and prerequisites
- **Epic & Adventure Management**: Grouping of related quests into larger storylines
- **Language Pack Management**: Handling of terminology and translations
- **Quest Assignment**: Allocation of quests to characters or minions
- **Quest Completion & Rewards**: Validation and reward distribution

## Data Models

### Quest Models
- **Quest**: Core entity with properties like title, description, difficulty, skills required
- **QuestPrerequisite**: Relationships between quests, defining which must be completed first
- **QuestDetail**: Expanded quest information including subtasks and prerequisites
- **SubTask**: Smaller components of a quest that must be completed

### Epic & Adventure Models
- **Epic**: Collection of related quests forming a coherent storyline
- **Adventure**: Collection of epics forming a complete narrative arc

### Terminology Models
- **LanguagePack**: Collection of terms and their translations
- **Terminology**: Individual term mappings for different usage contexts

## Database Schema

The service maintains several tables:

- `quests`: Stores core quest information
- `quest_prerequisites`: Tracks dependencies between quests
- `quest_subtasks`: Stores components of larger quests
- `epics`: Groups related quests
- `adventures`: Groups related epics
- `language_packs`: Stores terminology sets
- `quest_assignments`: Tracks who is assigned to which quests

## API Endpoints

The service exposes RESTful endpoints:

### Quest Endpoints
- `GET /quests`: List quests with filtering options
- `POST /quests`: Create a new quest
- `GET /quests/{id}`: Get detailed quest information
- `PUT /quests/{id}`: Update a quest
- `DELETE /quests/{id}`: Delete a quest

### Epic & Adventure Endpoints
- `GET /epics`: List epics with filtering
- `POST /epics`: Create a new epic
- `GET /adventures`: List adventures
- `POST /adventures`: Create a new adventure

### Language Pack Endpoints
- `GET /language-packs`: List available language packs
- `POST /language-packs`: Create a new language pack
- `GET /language-packs/current`: Get the current active language pack
- `PUT /language-packs/current`: Set the current active language pack

## Integration Points

The Quest Service integrates with:

- **Character Service**: To check skill requirements and assign quests
- **Guild Service**: To manage guild-specific quests and assignments
- **Reward Service**: To distribute rewards upon quest completion

## Technical Implementation

- **Framework**: Rust with Actix-Web
- **Database**: PostgreSQL with SQLx for type-safe queries
- **Migrations**: Liquibase for version-controlled schema changes
- **Documentation**: OpenAPI/Swagger for API specification
- **Testing**: Integration and unit tests with test containers

## Special Features

### Quest Types
Different quest types with specific behaviors:
- `Design`: Planning and architecture quests
- `ProofOfConcept`: Experimental implementation
- `Implement`: Full implementation of functionality
- `BugCheck`: Testing and verification
- `Subdivide`: Breaking down larger tasks

### Terminology System
The service supports dual terminology:
- Game terminology (quests, adventures, skills)
- Professional terminology (tasks, projects, competencies)

### Quest Relationships
Complex relationship modeling:
- Prerequisites: Quests that must be completed first
- Dependencies: Quests that depend on this quest
- Parent-child relationships: Quests can have subtasks 