class Skill {
  final String id;
  final String name;
  final String description;
  final String category;
  final int level;
  
  Skill({
    required this.id,
    required this.name,
    required this.description,
    required this.category,
    required this.level,
  });
  
  factory Skill.fromJson(Map<String, dynamic> json) {
    return Skill(
      id: json['id'],
      name: json['name'],
      description: json['description'],
      category: json['category'],
      level: json['level'],
    );
  }
  
  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name': name,
      'description': description,
      'category': category,
      'level': level,
    };
  }
}

class SkillCreate {
  final String name;
  final String description;
  final String category;
  final int? level;
  
  SkillCreate({
    required this.name,
    required this.description,
    required this.category,
    this.level,
  });
  
  Map<String, dynamic> toJson() {
    return {
      'name': name,
      'description': description,
      'category': category,
      'level': level,
    };
  }
} 