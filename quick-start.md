# Quick Start Guide

This guide will help you run the Guild application with minimal setup, using placeholder components.

## Prerequisites

- Flutter SDK installed
- Rust and Cargo installed
- PostgreSQL installed and running

## Running the Flutter App Only (Quickest Option)

If you just want to run the Flutter app with mock data without setting up the backend:

1. Navigate to the Flutter app directory:
   ```bash
   cd flutter_app
   ```

2. Get dependencies:
   ```bash
   flutter pub get
   ```

3. Run the app on your preferred platform:
   ```bash
   # For desktop/mobile:
   flutter run -t lib/main.dart
   
   # For web:
   flutter run -t lib/web_main.dart -d chrome
   ```

The app will run with placeholder graphics and mock data. You can interact with the character by clicking in the room.

## Running the Complete Stack (Flutter + Rust + PostgreSQL)

1. Set up the PostgreSQL database:
   ```bash
   # Create the database
   createdb guild_db
   ```

2. Run the Rust backend:
   ```bash
   cd backend
   cargo run
   ```

3. In a separate terminal, run the Flutter app:
   ```bash
   cd flutter_app
   flutter run
   ```

## Keyboard Controls

- W: Move character up
- A: Move character left
- S: Move character down
- D: Move character right
- Click: Move character to position

## Placeholder Elements

- The character is currently represented by a simple animated shape instead of a 3D model
- Room items are represented by icons
- Quest data is mocked

## Next Steps for Enhancement

1. Replace the `SimpleCharacterView` with actual 3D models
2. Connect to the Rust backend instead of using mock data
3. Implement proper authentication
4. Add animations for character movements 