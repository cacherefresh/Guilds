import 'dart:convert';

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

class Character {
  final String id;
  final String name;
  final CharacterType type;
  final List<dynamic> skills;
  final int level;
  final int xp;
  final String? guild;
  final String? teamId;
  final Map<String, dynamic> properties;
  final DateTime createdAt;
  final DateTime updatedAt;
  
  Character({
    required this.id,
    required this.name,
    required this.type,
    required this.skills,
    required this.level,
    required this.xp,
    this.guild,
    this.teamId,
    required this.properties,
    required this.createdAt,
    required this.updatedAt,
  });
  
  factory Character.fromJson(Map<String, dynamic> json) {
    return Character(
      id: json['id'],
      name: json['name'],
      type: CharacterTypeExtension.fromString(json['type']),
      skills: json['skills'] ?? [],
      level: json['level'],
      xp: json['xp'],
      guild: json['guild'],
      teamId: json['team_id'],
      properties: json['properties'] is String 
          ? jsonDecode(json['properties']) 
          : (json['properties'] ?? {}),
      createdAt: DateTime.parse(json['created_at']),
      updatedAt: DateTime.parse(json['updated_at']),
    );
  }
  
  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name': name,
      'type': type.name,
      'skills': skills,
      'level': level,
      'xp': xp,
      'guild': guild,
      'team_id': teamId,
      'properties': properties,
      'created_at': createdAt.toIso8601String(),
      'updated_at': updatedAt.toIso8601String(),
    };
  }
}

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