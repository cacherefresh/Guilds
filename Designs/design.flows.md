# Guild App Flows

This document outlines all the user flows in the Guild application, covering both the game-style terminology and professional (real-world) terminology options.

## User Account & Character Flows

### Character Creation
1. User provides character name, type, and initial skills
2. System creates character profile
3. Character is assigned default level (1) and XP (0)
4. User is prompted to join an existing guild or create a new one

### Character Profile Management
1. User can view their character profile
2. User can update character information
3. User can view their skills, XP, and level
4. User can track quests (completed, in-progress, available)
5. User can view guild affiliations
6. User can update character appearance/avatar

### Skill Advancement
1. User completes quests to gain XP
2. XP contributes to level advancement
3. User can assign points to skills as they level up
4. Skills can be categorized (technical, management, design, etc.)
5. Skill proficiency affects quest eligibility

## Guild Management Flows

### Guild Creation
1. User selects "Create Guild" option
2. User provides guild name, description, and focus areas
3. User selects a town to establish the guild
4. User becomes guild leader automatically
5. System creates guild record and affiliates character with guild

### Guild Membership
1. Character can apply to join guild
2. Guild leader/officers review applications
3. Guild leader/officers can accept/reject applications
4. Character can leave guild voluntarily
5. Guild leader can remove members if needed
6. Characters can be promoted to officers or leaders

### Guild Dashboard
1. Members can view guild dashboard
2. Dashboard shows active quests, members, and guild stats
3. Officers/leaders can manage guild settings
4. Guild reputation and level displayed
5. Guild achievements and history tracked

## Town & Location Flows

### Town Exploration
1. User can view available towns
2. User can travel between towns
3. Each town has unique characteristics and available quests
4. Towns contain guilds, quest boards, and services
5. User can interact with town NPCs and locations

### Town Services
1. User can visit various town services
2. Services include shops, training facilities, quest boards
3. User can purchase items, train skills, or accept quests
4. Services may have town-specific specialties

## Quest Flows

### Quest Board Interaction
1. User approaches quest board in town
2. System checks if character is close enough to interact
3. User clicks on board to interact
4. Quest board dialog appears showing available quests
5. Quests are filtered based on character eligibility

### Quest Lifecycle
1. Quests are created by NPCs, guild leaders, or system
2. Quests include title, description, difficulty, skills required, rewards
3. Character accepts quest from board or guild
4. Character completes quest objectives
5. Character submits quest for review
6. Quest is validated and rewards distributed
7. Quest status updated to completed
8. Character gains XP, gold, and reputation

### Quest Creation (Admin/Guild Leader)
1. Admin/Guild leader selects "Create Quest" option
2. Admin provides quest details:
   - Title and description
   - Required skills
   - Difficulty level
   - Rewards (XP, gold, items)
   - Prerequisites if any
   - Time constraints if any
3. Admin assigns quest to specific quest board or guild
4. Quest becomes available to eligible characters

### Epic/Adventure Management
1. Related quests can be grouped into epics/adventures
2. Epics have overall story arc and cumulative rewards
3. Adventures contain multiple epics
4. Users can track progress through epics/adventures
5. Completing all quests in epic/adventure provides bonus rewards

## Team Collaboration Flows

### Team Formation
1. Users can create temporary teams for specific quests
2. Team creator invites other characters
3. Invited characters accept/decline
4. Team composition affects quest outcomes
5. Team disbands after quest completion or manually

### Team Collaboration
1. Team members can chat during quest
2. Team members coordinate on quest objectives
3. Team members can share resources
4. Quest rewards distribution options available

## Language & Terminology Flows

### Language Selection
1. User can select preferred language pack
2. System updates all terminology based on selection
3. Toggle between game-style and professional terminology
4. User interface adapts to terminology selection

### Terminology Settings
1. Admin can create/edit language packs
2. Admin can create/edit terminology sets
3. Admin can set default language and terminology
4. Users can override default with personal preference

## Administrative Flows

### Content Management
1. Admins can create/edit quests, epics, adventures
2. Admins can create/edit guilds, towns, NPCs
3. Admins can manage users and characters
4. Admins can configure system settings

### System Configuration
1. Configure terminology and language settings
2. Manage database connections
3. Configure API endpoints
4. Set up authentication systems
5. Configure notification systems

## Notification Flows

### In-app Notifications
1. User receives notifications for quest updates
2. User receives notifications for guild activities
3. User receives notifications for team invitations
4. User can manage notification preferences

### Email Notifications
1. Optional email notifications for key events
2. Account-related notifications
3. Weekly summary of activities
4. Notification preferences management

## General User Interface Flows

### Navigation
1. Main dashboard with key information
2. Character profile access
3. Guild dashboard access
4. Quest board access
5. Town/location exploration
6. Team management

### Help & Support
1. Context-sensitive help
2. Tutorial system for new users
3. FAQ access
4. Support request submission
5. Community forums access

### Settings & Preferences
1. User interface customization
2. Notification preferences
3. Privacy settings
4. Language and terminology preferences
5. Account management 