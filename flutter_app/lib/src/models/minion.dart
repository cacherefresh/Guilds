import 'dart:async';

import 'enums/minion_state.dart';
import 'minion_animation.dart';
import 'position.dart';
import 'quest.dart';
import 'skill.dart';

/// Represents a minion/assistant in the game
class Minion {
  /// Unique identifier
  final String id;
  
  /// Minion name
  final String name;
  
  /// Minion level
  int level;
  
  /// Skills the minion possesses
  List<Skill> skills;
  
  /// Current position in the world
  Position position;
  
  /// Current state
  MinionState state;
  
  /// Current assigned quest (if any)
  Quest? assignedQuest;
  
  /// Animation controller
  final MinionAnimation animation;
  
  /// Timer for periodic animations
  Timer? animationTimer;
  
  /// Constructor
  Minion({
    required this.id,
    required this.name,
    this.level = 1,
    List<Skill>? skills,
    Position? position,
    MinionState? state,
    this.assignedQuest,
    MinionAnimation? animation,
  }) : 
    skills = skills ?? [],
    position = position ?? const Position(x: 0, y: 0),
    state = state ?? MinionState.idle,
    animation = animation ?? MinionAnimation();
  
  /// Set the minion's state
  void setState(MinionState newState) {
    if (state == newState) return;
    
    state = newState;
    
    // Update animation based on state
    switch (state) {
      case MinionState.idle:
        animation.play('idle');
        break;
      case MinionState.following:
        animation.play('follow');
        break;
      case MinionState.assigned:
        animation.play('idle'); // Default for assigned, will be overridden by running
        break;
      case MinionState.running:
        animation.play('quest_running');
        break;
      case MinionState.completing:
        animation.play('walk');
        break;
      case MinionState.completed:
        animation.play('completed_task');
        break;
    }
  }
  
  /// Play a specific animation
  void playAnimation(String animationName) {
    animation.play(animationName);
  }
  
  /// Reset the current animation
  void resetAnimation() {
    animation.reset();
  }
  
  /// Update the minion state
  void update(int deltaTimeMs) {
    // Update animation
    animation.update(deltaTimeMs);
    
    // Other update logic
  }
  
  /// Dispose resources
  void dispose() {
    animationTimer?.cancel();
  }
} 