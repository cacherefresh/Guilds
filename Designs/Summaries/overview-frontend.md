# Frontend Architecture Overview

## Core Architecture

The Flutter frontend follows a layered architecture with clean separation of concerns:

- **Models**: Data structures that represent domain entities like Character, Minion, Quest.
- **Widgets**: UI components for rendering entities and handling user interactions.
- **Providers**: State management using Provider pattern for reactive UI updates.
- **Services**: Handle external communication and persistent storage.
- **Screens**: Composed widgets that represent full application views.

## Animation System

The application implements a dual-mode animation system:

### No-Animation Mode (Default)
- Characters and minions are displayed with static images
- Current state and animation names are displayed as text labels
- State changes trigger a golden flicker effect for 1 second
- Models still move to correct positions but skip intermediate steps
- Minimal processor usage, ideal for lower-end devices

### Full Animation Mode
- Characters and minions display fluid animations
- Frame-by-frame animation with configurable frame rates
- Can be enabled via settings for users with more powerful devices

## Terminology System

The application uses a flexible terminology system:

- Text is displayed via a `TermText` widget that handles translation
- Game terminology (quests, guilds, minions) is used by default in standalone mode
- Professional terminology (projects, teams, assistants) can be toggled in full mode
- Settings allow switching between terminology sets

## Character & Minion Management

Characters and minions have a comprehensive state system:

- States like idle, walking, interacting, casting
- Minions transition between states based on quest assignments
- Idle system automatically sends characters to throne after inactivity
- Minions follow the main character when idle or perform quest animations

## Quest Interaction System

The quest system provides interfaces for:

- Viewing available quests based on character eligibility
- Accepting and tracking quest progress
- Assigning minions to assist with quests
- Completing quests and collecting rewards

## Grimoire System

The grimoire is a special interaction point where advanced features are accessible:

- Shadow magic (minion creation, shadow clone) requires grimoire proximity
- Visual feedback indicates when character is in range
- Magic system categorizes spells by type with specific requirements

## Game Loop Management

The application implements a game loop for consistent updates:

- Updates animations and character states at 60 FPS
- Delta-time based updates for smooth motion regardless of device performance
- Optimized to reduce battery usage in no-animation mode

## Settings & Preferences

User preferences are persisted between sessions:

- Animation mode preference (defaulting to No-Animation)
- Terminology preference (defaulting to game terminology in standalone mode)
- Performance options to adapt to device capabilities 