# Character Service Architecture Overview

## Core Components

The Character Service manages all aspects of characters and minions:

- **Character Management**: CRUD operations for characters
- **Skill Management**: Character skills, advancement, and levels
- **Minion Management**: Creation and control of character minions
- **State Management**: Tracking character and minion states
- **Interaction**: Managing how characters interact with the game world

## Data Models

### Character Models
- **Character**: Core entity with properties like name, type, level, position, state
- **CharacterAnimation**: Animation frames and states for the character
- **CharacterSkill**: Skills a character possesses with proficiency levels

### Minion Models
- **Minion**: Assistants created by characters, with their own properties
- **MinionAnimation**: Animation frames and states for minions
- **MinionAssignment**: Tracking what quests minions are assigned to

### Supporting Models
- **Position**: Tracking location in the game world
- **Skill**: Reusable skill definitions with categories and levels
- **CharacterState/MinionState**: Enums defining possible states

## Database Schema

The service maintains several tables:

- `characters`: Stores core character information
- `character_skills`: Junction table for character-skill relationships
- `minions`: Stores minion information
- `skills`: Skill definitions and categories
- `character_states`: Historical record of character state changes
- `character_positions`: Historical record of character positions

## API Endpoints

The service exposes RESTful endpoints:

### Character Endpoints
- `GET /characters`: List characters with filtering
- `POST /characters`: Create a new character
- `GET /characters/{id}`: Get detailed character information
- `PUT /characters/{id}`: Update a character
- `DELETE /characters/{id}`: Delete a character
- `PUT /characters/{id}/state`: Update character state
- `PUT /characters/{id}/position`: Update character position

### Minion Endpoints
- `GET /characters/{id}/minions`: List minions for a character
- `POST /characters/{id}/minions`: Create a new minion
- `PUT /characters/{id}/minions/{minionId}`: Update a minion
- `DELETE /characters/{id}/minions/{minionId}`: Delete a minion
- `PUT /characters/{id}/minions/{minionId}/assign`: Assign minion to quest

### Skill Endpoints
- `GET /skills`: List all available skills
- `POST /skills`: Create a new skill
- `GET /characters/{id}/skills`: Get character skills
- `POST /characters/{id}/skills`: Add skill to character
- `PUT /characters/{id}/skills/{skillId}`: Update character skill level

## Integration Points

The Character Service integrates with:

- **Quest Service**: To check quest eligibility and handle assignments
- **Guild Service**: To manage guild memberships
- **Language Pack Service**: For terminology in character interactions

## Technical Implementation

- **Framework**: Rust with Actix-Web
- **Database**: PostgreSQL with SQLx for type-safe queries
- **Migrations**: Liquibase for version-controlled schema changes
- **Documentation**: OpenAPI/Swagger for API specification
- **Testing**: Integration and unit tests with test containers

## Special Features

### Character States
Different character states with specific behaviors:
- `Active`: Normal interaction state
- `Idle`: After period of inactivity
- `Walking`: Moving between locations
- `SittingOnThrone`: Special idle state
- `Casting`: Using magic abilities
- `Interacting`: Interacting with world objects

### Minion States
Different minion states with specific behaviors:
- `Idle`: Default state when not assigned
- `Following`: Following the character
- `Assigned`: Working on a quest
- `Running`: Special animation during quests
- `Completing`: Finishing a quest
- `Completed`: Quest completion animation

### Idle System
The service manages character idle behavior:
- Detects inactivity after configurable period
- Initiates walking to throne when idle
- Manages minion following behavior
- Preserves menu states during idle

### Magic System
Specialized character abilities:
- Spell casting with different magic types
- Location-based restrictions (grimoire)
- Mana/energy management
- Shadow magic for minion creation 