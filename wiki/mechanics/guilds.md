# Guild System

The guild system allows players to form, join, and participate in guilds within the game. Guilds provide a way for characters to work together, share resources, and take on quests as a collective entity.

## Guild Features

- **Guild Name and Identity**: Each guild has a unique name and description.
- **Guild Master**: Every guild has a designated guild master (the player who created it).
- **AI Teammate**: Each guild gets one AI-controlled companion character.
- **Guild Stash**: Guilds maintain a shared treasury of resources and gold.
- **Reward Distribution**: Customizable reward sharing between members and the guild stash.
- **Location**: Guilds are based in specific towns, which influences available quests and resources.

## Character Membership

- **Multiple Guilds**: Players can be members of up to 3 different guilds simultaneously.
- **Active Guild**: While a character can join multiple guilds, only one can be designated as "active" at any time.
- **Guild Benefits**: Characters receive benefits based on their guild's status and resources.
- **Guild Switching**: Players can switch their active guild at any time.

## Guild Reputation and Advancement

- **Quest Completion**: Guilds gain reputation and resources by completing quests.
- **Skill Specialization**: Guilds can specialize in specific skill sets, making them more attractive for certain quests.
- **Guild Ranking**: Guilds are ranked based on completed quests and member abilities.

## Towns and Guild Hubs

- **Town Location**: Every town has unique characteristics and available quests.
- **Guild Search**: Players can search for guilds based on skills, location, or name.
- **Guild Hall**: Each guild maintains a presence in its home town.

## Quest Assignment

- **Guild Quests**: Quests can be assigned to an entire guild.
- **Personal Quests**: Quests can be assigned to individual characters.
- **Combined Assignment**: Quests can be assigned to both a guild and a specific character who serves as the point of contact.
- **Quest Credit**: When a guild member completes a quest while representing their guild, the guild receives credit and rewards.

## Reward Distribution

- **Divider Percentage**: The guild defines what percentage of rewards goes to members versus the guild stash.
- **Guild Stash**: A portion of all quest rewards goes to the guild's treasury.
- **Member Rewards**: The remaining portion is distributed among participating members.

## Technical Implementation

### Database Structure

The guild system uses the following key tables:

- `guilds`: Stores guild information, including name, description, and settings.
- `towns`: Stores town information where guilds are located.
- `character_guilds`: Junction table for character-guild membership.
- `ai_teammates`: Stores information about AI companions for guilds.

### API Endpoints

The guild system exposes several RESTful endpoints:

- Guild management: Create, update, delete, and retrieve guilds.
- Membership management: Add/remove characters, set active guild.
- Guild search: Find guilds by name, skills, or location.
- Town management: Create, update, delete, and retrieve towns.

### Business Logic

The system enforces several business rules:

- Characters can be members of at most 3 guilds.
- Only one guild can be active per character.
- Guild masters are automatically added as members of their guild.
- Rewards are distributed according to the guild's divider percentage.

## Example Usage

1. **Creating a Guild**:
   ```json
   POST /api/guilds
   {
     "name": "Dragon Hunters",
     "description": "A guild specializing in hunting dangerous dragons",
     "guild_master_id": "550e8400-e29b-41d4-a716-446655440000",
     "town_id": "550e8400-e29b-41d4-a716-446655440001",
     "reward_divider_percentage": 70
   }
   ```

2. **Joining a Guild**:
   ```json
   POST /api/guilds/550e8400-e29b-41d4-a716-446655440002/members
   {
     "character_id": "550e8400-e29b-41d4-a716-446655440003",
     "set_active": true
   }
   ```

3. **Assigning a Quest to a Guild**:
   ```json
   POST /api/quests
   {
     "title": "Slay the Red Dragon",
     "description": "A dangerous dragon threatens the countryside",
     "difficulty": 8,
     "required_skill_ids": ["550e8400-e29b-41d4-a716-446655440004"],
     "reward": "5000 gold, Dragon Scale Armor",
     "xp_reward": 2000,
     "gold_reward": 5000,
     "assigned_guild_id": "550e8400-e29b-41d4-a716-446655440002",
     "contact_character_id": "550e8400-e29b-41d4-a716-446655440003"
   }
   ``` 