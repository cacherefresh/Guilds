import 'skill.dart';

enum QuestStatus {
  available,
  inProgress,
  completed,
  failed
}

extension QuestStatusExtension on QuestStatus {
  String get name {
    switch (this) {
      case QuestStatus.available:
        return 'AVAILABLE';
      case QuestStatus.inProgress:
        return 'IN_PROGRESS';
      case QuestStatus.completed:
        return 'COMPLETED';
      case QuestStatus.failed:
        return 'FAILED';
    }
  }
  
  static QuestStatus fromString(String status) {
    switch (status) {
      case 'AVAILABLE':
        return QuestStatus.available;
      case 'IN_PROGRESS':
        return QuestStatus.inProgress;
      case 'COMPLETED':
        return QuestStatus.completed;
      case 'FAILED':
        return QuestStatus.failed;
      default:
        return QuestStatus.available;
    }
  }
}

class Quest {
  final String id;
  final String title;
  final String description;
  final int difficulty;
  final List<Skill> requiredSkills;
  final String? reward;
  final int xpReward;
  final int goldReward;
  final QuestStatus status;
  final DateTime createdAt;
  final DateTime updatedAt;
  
  Quest({
    required this.id,
    required this.title,
    required this.description,
    required this.difficulty,
    required this.requiredSkills,
    this.reward,
    this.xpReward = 0,
    this.goldReward = 0,
    required this.status,
    required this.createdAt,
    required this.updatedAt,
  });
  
  factory Quest.fromJson(Map<String, dynamic> json) {
    return Quest(
      id: json['id'],
      title: json['title'],
      description: json['description'],
      difficulty: json['difficulty'],
      requiredSkills: (json['required_skills'] as List)
          .map((skill) => Skill.fromJson(skill))
          .toList(),
      reward: json['reward'],
      xpReward: json['xp_reward'],
      goldReward: json['gold_reward'],
      status: QuestStatusExtension.fromString(json['status']),
      createdAt: DateTime.parse(json['created_at']),
      updatedAt: DateTime.parse(json['updated_at']),
    );
  }
  
  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'title': title,
      'description': description,
      'difficulty': difficulty,
      'required_skills': requiredSkills.map((skill) => skill.toJson()).toList(),
      'reward': reward,
      'xp_reward': xpReward,
      'gold_reward': goldReward,
      'status': status.name,
      'created_at': createdAt.toIso8601String(),
      'updated_at': updatedAt.toIso8601String(),
    };
  }
}

class QuestCreate {
  final String title;
  final String description;
  final int? difficulty;
  final List<String> requiredSkillIds;
  final String? reward;
  final int? xpReward;
  final int? goldReward;
  
  QuestCreate({
    required this.title,
    required this.description,
    this.difficulty,
    required this.requiredSkillIds,
    this.reward,
    this.xpReward,
    this.goldReward,
  });
  
  Map<String, dynamic> toJson() {
    return {
      'title': title,
      'description': description,
      'difficulty': difficulty,
      'required_skill_ids': requiredSkillIds,
      'reward': reward,
      'xp_reward': xpReward,
      'gold_reward': goldReward,
    };
  }
}

class QuestUpdate {
  final String? title;
  final String? description;
  final int? difficulty;
  final List<String>? requiredSkillIds;
  final String? reward;
  final int? xpReward;
  final int? goldReward;
  final QuestStatus? status;
  
  QuestUpdate({
    this.title,
    this.description,
    this.difficulty,
    this.requiredSkillIds,
    this.reward,
    this.xpReward,
    this.goldReward,
    this.status,
  });
  
  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = {};
    
    if (title != null) data['title'] = title;
    if (description != null) data['description'] = description;
    if (difficulty != null) data['difficulty'] = difficulty;
    if (requiredSkillIds != null) data['required_skill_ids'] = requiredSkillIds;
    if (reward != null) data['reward'] = reward;
    if (xpReward != null) data['xp_reward'] = xpReward;
    if (goldReward != null) data['gold_reward'] = goldReward;
    if (status != null) data['status'] = status!.name;
    
    return data;
  }
} 