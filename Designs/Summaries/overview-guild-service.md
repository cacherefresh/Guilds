# Guild Service Architecture Overview

## Core Components

The Guild Service manages all aspects of guilds and towns:

- **Guild Management**: CRUD operations for guilds
- **Town Management**: Information about towns where guilds are located
- **Membership Management**: Handling character affiliations with guilds
- **Guild Quests**: Guild-specific quest management
- **Guild Rankings**: Tracking guild performance and reputation

## Data Models

### Guild Models
- **Guild**: Core entity with properties like name, description, focus areas
- **GuildMember**: Junction between characters and guilds, with roles
- **GuildQuest**: Quests specific to a guild or its members

### Town Models
- **Town**: Locations where guilds can be established
- **TownBuilding**: Structures within towns where activities occur
- **TownService**: Services available in specific towns

### Supporting Models
- **GuildRank**: Reputation and standing of guilds
- **MemberRole**: Roles within a guild (leader, officer, member)
- **GuildResource**: Resources owned or managed by guilds

## Database Schema

The service maintains several tables:

- `guilds`: Stores core guild information
- `towns`: Stores town information
- `guild_members`: Junction table for character-guild relationships
- `guild_quests`: Quests associated with specific guilds
- `town_buildings`: Buildings and locations within towns
- `guild_resources`: Resources owned by guilds
- `guild_rankings`: Historical and current guild rankings

## API Endpoints

The service exposes RESTful endpoints:

### Guild Endpoints
- `GET /guilds`: List guilds with filtering
- `POST /guilds`: Create a new guild
- `GET /guilds/{id}`: Get detailed guild information
- `PUT /guilds/{id}`: Update a guild
- `DELETE /guilds/{id}`: Delete a guild

### Town Endpoints
- `GET /towns`: List towns
- `POST /towns`: Create a new town
- `GET /towns/{id}`: Get detailed town information
- `GET /towns/{id}/guilds`: List guilds in a town
- `GET /towns/{id}/buildings`: List buildings in a town

### Membership Endpoints
- `GET /guilds/{id}/members`: List guild members
- `POST /guilds/{id}/members`: Add member to guild
- `PUT /guilds/{id}/members/{characterId}`: Update member role
- `DELETE /guilds/{id}/members/{characterId}`: Remove member from guild

### Guild Quest Endpoints
- `GET /guilds/{id}/quests`: List guild quests
- `POST /guilds/{id}/quests`: Create guild quest
- `PUT /guilds/{id}/quests/{questId}/assign`: Assign quest to member

## Integration Points

The Guild Service integrates with:

- **Character Service**: To manage guild memberships and verify character information
- **Quest Service**: To create and track guild-specific quests
- **Reward Service**: To distribute guild rewards and manage guild resources

## Technical Implementation

- **Framework**: Rust with Actix-Web
- **Database**: PostgreSQL with SQLx for type-safe queries
- **Migrations**: Liquibase for version-controlled schema changes
- **Documentation**: OpenAPI/Swagger for API specification
- **Testing**: Integration and unit tests with test containers

## Special Features

### Guild Locations
Guild-specific locations in towns:
- Guild halls with customizable interiors
- Training areas for skill advancement
- Quest boards for guild-specific quests
- Throne rooms for guild leaders

### Guild Roles & Permissions
Hierarchical permission system:
- **Guild Leader**: Full administrative control
- **Officers**: Member management and quest assignment
- **Members**: Basic participation and quest completion
- **Applicants**: Limited access while awaiting approval

### Guild Competition
Systems for guild competition and ranking:
- Guild reputation based on quest completion
- Guild rankings updated periodically
- Guild achievements and special recognitions
- Inter-guild competitions and events

### Town Services
Town-specific services and resources:
- Different towns specialize in different services
- Town reputation affects available services
- Towns can grow and develop over time
- Special buildings and locations in each town 