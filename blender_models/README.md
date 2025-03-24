# Character and Environment 3D Models

This directory contains Blender models for the character and environment used in the Guild application.

## Character Model Requirements

The character model should match the following specifications:

- Tall, slightly skinny male
- Brown hair
- Black shirt and black jeans
- Flat-brim styled hat
- Journal/grimoire in left hand
- Right hand available for pointing gestures
- Chin-grabbing pose for "thinking" animation

## Required Animations

The character should have the following animations:

1. Idle - Default standing pose
2. Walking - Basic walk cycle
3. Pointing - Extending right arm to point
4. Thinking - Grabbing chin in contemplative pose
5. Sitting - For when at the laptop table

## Room Environment Model

The room should include the following elements:

1. Todo/Quest Board - A wall-mounted board for displaying quests
2. Table with Laptop - A desk with a laptop where the character can sit
3. Webcam - On or near the laptop facing the character
4. Turntables - Music equipment in a corner of the room

## Color Scheme

- Primary: Purple (#6A0DAD)
- Secondary: Green (#00C853)
- Background: Dark/Black (#121212)

## Export Requirements

- Export models in glTF 2.0 format for web compatibility
- Include all textures and animations
- Optimize for mobile performance
- Place exported files in the Flutter app's assets/models directory

## Development Guidelines

1. First create a base character model with proper rigging
2. Create basic animations
3. Add props (grimoire, hat, etc.)
4. Create the room environment
5. Test animations and interactions
6. Optimize for performance
7. Export in proper format

## Tools

- Blender 2.9+ recommended
- Texture resolution should be 2K or less for performance 