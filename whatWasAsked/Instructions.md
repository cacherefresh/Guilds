# Project Requirements Summary

## Application Architecture
- Flutter app with app.dart as main entry point
- Two wrapper classes:
  - Main.dart for iOS/Android/Windows desktop app
  - WebMain.dart for browser/JS app
  - Both call into App.dart
- Rust backend with PostgreSQL database
- Designed for scalability
- OpenAPI 3 YAML for contract-first design
- API client generation for Flutter to communicate with Rust backend
- Liquibase folder for SQL scripts

## 3D Character & Environment
- 3D model created in Blender
- Character specifications:
  - Tall, slightly skinny with brown hair
  - Black shirt and jeans
  - Flat brim hat
  - Journal/grimoire in left hand
  - Ability to point with right hand
  - Ability to grab chin in thinking pose
- Character controls:
  - ASDW keys for directional movement
  - Click-to-move functionality
- Interactive room with:
  - Todo board (quest board)
  - Table with laptop (character can sit)
  - Webcam that faces character
  - Turntables for music in corner
- Green, purple, and black color theme throughout

## Character Functionality
- Persistent data storage for:
  - Interests (preset: music, programming, bringing AI to life)
  - Skills
  - Magic abilities
- Shadow summoning abilities:
  - Shadow clone (digital twin)
  - Shadow minions for tasks
- Minion task states:
  - Loaded
  - Start
  - Doing
  - Complete
  - Canceled
  - isDone flag (true for canceled or complete)
- Guild concept with room labeling
- Character naming

## Quest/Task System
- Todo board with available quests
- Quest properties:
  - Category
  - Reward amount
  - Description
  - Short description
  - Requestor (another character)
  - Assignee (who accepts the quest)
- Ability to create new quests with popup dialog
- Self-assignment capability (user can be both requestor and assignee) 