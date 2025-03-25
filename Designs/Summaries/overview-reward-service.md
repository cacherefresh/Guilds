# Reward Service Architecture Overview

## Core Components

The Reward Service manages all aspects of rewards and achievements:

- **Reward Management**: Different types of rewards (XP, gold, items)
- **Achievement Management**: Special accomplishments and recognitions
- **XP & Leveling**: Experience point accumulation and level progression
- **Resource Distribution**: Allocation of rewards to characters and guilds
- **Reward History**: Tracking of all reward transactions

## Data Models

### Reward Models
- **Reward**: Base reward entity with type and value
- **XpReward**: Experience points that contribute to leveling
- **GoldReward**: In-game currency for purchases
- **ItemReward**: Special items or equipment
- **SkillPointReward**: Points that can be allocated to skills

### Achievement Models
- **Achievement**: Accomplishments that can be earned
- **CharacterAchievement**: Achievements earned by characters
- **GuildAchievement**: Achievements earned by guilds
- **AchievementCriteria**: Conditions for earning achievements

### Supporting Models
- **RewardTransaction**: Record of reward distributions
- **LevelThreshold**: XP requirements for different levels
- **RewardTemplate**: Predefined reward packages for different activities

## Database Schema

The service maintains several tables:

- `rewards`: Stores different types of rewards
- `achievements`: Stores achievement definitions
- `character_achievements`: Junction for character-achievement relationships
- `guild_achievements`: Junction for guild-achievement relationships
- `reward_transactions`: Historical record of all reward distributions
- `level_thresholds`: XP requirements for different levels
- `reward_templates`: Predefined reward combinations

## API Endpoints

The service exposes RESTful endpoints:

### Reward Endpoints
- `POST /rewards`: Create a new reward
- `GET /rewards/templates`: List reward templates
- `POST /rewards/distribute`: Distribute rewards to recipients
- `GET /characters/{id}/rewards`: List rewards for a character
- `GET /guilds/{id}/rewards`: List rewards for a guild

### Achievement Endpoints
- `GET /achievements`: List all achievements
- `POST /achievements`: Create a new achievement
- `GET /achievements/{id}`: Get achievement details
- `GET /characters/{id}/achievements`: List character achievements
- `POST /characters/{id}/achievements`: Award achievement to character

### XP & Leveling Endpoints
- `GET /levels`: List level thresholds
- `GET /characters/{id}/xp`: Get character XP and level
- `POST /characters/{id}/xp`: Award XP to character
- `GET /skills/{id}/levels`: Get skill level thresholds

## Integration Points

The Reward Service integrates with:

- **Character Service**: To update character XP, level, and achievements
- **Guild Service**: To distribute rewards to guilds and track guild achievements
- **Quest Service**: To determine rewards for completed quests

## Technical Implementation

- **Framework**: Rust with Actix-Web
- **Database**: PostgreSQL with SQLx for type-safe queries
- **Migrations**: Liquibase for version-controlled schema changes
- **Documentation**: OpenAPI/Swagger for API specification
- **Testing**: Integration and unit tests with test containers

## Special Features

### Dynamic Reward Scaling
Intelligent reward calculation based on:
- Character level and skills
- Quest difficulty and complexity
- Time spent on quests
- Quality of quest completion

### Achievement System
Comprehensive achievement tracking:
- Tiered achievements with increasing difficulty
- Secret achievements that aren't visible until earned
- Guild-wide achievements requiring team effort
- Time-limited achievements for special events

### Leveling System
Sophisticated experience and leveling mechanisms:
- Non-linear XP requirements for levels
- Skill-specific leveling separate from character level
- Specialized paths for different character types
- Diminishing returns for repetitive activities

### Reward Notifications
Real-time reward distribution alerts:
- In-app notifications for earned rewards
- Animation effects for significant achievements
- Leaderboards for competitive achievements
- Weekly and monthly reward summaries 