import 'dart:convert';
import 'character_animation.dart';
import 'enums/character_state.dart';
import 'position.dart';
import 'skill.dart';

enum CharacterType {
  player,
  npc,
  monster
}

extension CharacterTypeExtension on CharacterType {
  String get name {
    switch (this) {
      case CharacterType.player:
        return 'PLAYER';
      case CharacterType.npc:
        return 'NPC';
      case CharacterType.monster:
        return 'MONSTER';
    }
  }
  
  static CharacterType fromString(String type) {
    switch (type) {
      case 'PLAYER':
        return CharacterType.player;
      case 'NPC':
        return CharacterType.npc;
      case 'MONSTER':
        return CharacterType.monster;
      default:
        return CharacterType.player;
    }
  }
}

/// Represents a character in the game
class Character {
  /// Unique identifier
  final String id;
  
  /// Character name
  final String name;
  
  /// Character type/class
  final String type;
  
  /// Skills the character possesses
  final List<Skill> skills;
  
  /// Character level
  int level;
  
  /// Experience points
  int xp;
  
  /// Current position in the world
  Position position;
  
  /// Current state (idle, walking, etc.)
  CharacterState state;
  
  /// Animation controller
  final CharacterAnimation animation;
  
  /// Current mana/energy
  int mana;
  
  /// Maximum mana/energy
  int maxMana;
  
  /// Constructor
  Character({
    required this.id,
    required this.name,
    required this.type,
    required this.skills,
    this.level = 1,
    this.xp = 0,
    Position? position,
    CharacterState? state,
    CharacterAnimation? animation,
    this.mana = 100,
    this.maxMana = 100,
  }) : 
    this.position = position ?? Position(x: 0, y: 0),
    this.state = state ?? CharacterState.idle,
    this.animation = animation ?? CharacterAnimation();
  
  /// Set the character's state
  void setState(CharacterState newState) {
    if (state == newState) return;
    
    state = newState;
    
    // Update animation based on state
    switch (state) {
      case CharacterState.active:
        animation.play('idle');
        break;
      case CharacterState.idle:
        animation.play('idle');
        break;
      case CharacterState.walking:
        animation.play('walk');
        break;
      case CharacterState.sittingOnThrone:
        animation.play('sit_on_throne');
        break;
      case CharacterState.casting:
        animation.play('cast');
        break;
      case CharacterState.interacting:
        animation.play('interact');
        break;
    }
  }
  
  /// Play a specific animation
  void playAnimation(String animationName) {
    animation.play(animationName);
  }
  
  /// Consume mana for casting spells
  bool consumeMana(int amount) {
    if (mana < amount) return false;
    
    mana -= amount;
    return true;
  }
  
  /// Regenerate mana over time
  void regenerateMana(int amount) {
    mana = min(mana + amount, maxMana);
  }
  
  /// Update the character state
  void update(int deltaTimeMs) {
    // Update animation
    animation.update(deltaTimeMs);
    
    // Other update logic
  }
}

// Helper function for min value
int min(int a, int b) => a < b ? a : b;

class CharacterCreate {
  final String name;
  final CharacterType type;
  final List<String>? skillIds;
  final int? level;
  final int? xp;
  final String? guild;
  final String? teamId;
  final Map<String, dynamic>? properties;
  
  CharacterCreate({
    required this.name,
    required this.type,
    this.skillIds,
    this.level,
    this.xp,
    this.guild,
    this.teamId,
    this.properties,
  });
  
  Map<String, dynamic> toJson() {
    return {
      'name': name,
      'type': type.name,
      'skill_ids': skillIds,
      'level': level,
      'xp': xp,
      'guild': guild,
      'team_id': teamId,
      'properties': properties,
    };
  }
}

class CharacterUpdate {
  final String? name;
  final CharacterType? type;
  final List<String>? skillIds;
  final int? level;
  final int? xp;
  final String? guild;
  final String? teamId;
  final Map<String, dynamic>? properties;
  
  CharacterUpdate({
    this.name,
    this.type,
    this.skillIds,
    this.level,
    this.xp,
    this.guild,
    this.teamId,
    this.properties,
  });
  
  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = {};
    
    if (name != null) data['name'] = name;
    if (type != null) data['type'] = type.name;
    if (skillIds != null) data['skill_ids'] = skillIds;
    if (level != null) data['level'] = level;
    if (xp != null) data['xp'] = xp;
    if (guild != null) data['guild'] = guild;
    if (teamId != null) data['team_id'] = teamId;
    if (properties != null) data['properties'] = properties;
    
    return data;
  }
} 