import 'package:flutter/foundation.dart';
import 'minion_model.dart';

class CharacterModel with ChangeNotifier {
  String _name = "Default Character";
  String _guildName = "Default Guild";
  List<String> _interests = ["Music", "Programming", "Bringing AI to Life"];
  List<String> _skills = [];
  List<String> _magicAbilities = ["Shadow Clone", "Shadow Minion"];
  List<MinionModel> _minions = [];
  
  // Position in the room
  double _positionX = 0.0;
  double _positionY = 0.0;
  double _positionZ = 0.0;
  
  // Current action
  String _currentAction = "idle"; // idle, walking, thinking, pointing
  
  // Character properties
  final Map<String, String> _properties = {
    'afterimage': 'off'
  };
  
  // Getters
  String get name => _name;
  String get guildName => _guildName;
  List<String> get interests => List.unmodifiable(_interests);
  List<String> get skills => List.unmodifiable(_skills);
  List<String> get magicAbilities => List.unmodifiable(_magicAbilities);
  List<MinionModel> get minions => List.unmodifiable(_minions);
  double get positionX => _positionX;
  double get positionY => _positionY;
  double get positionZ => _positionZ;
  String get currentAction => _currentAction;
  
  // Setters with notification
  set name(String newName) {
    _name = newName;
    notifyListeners();
  }
  
  set guildName(String newGuildName) {
    _guildName = newGuildName;
    notifyListeners();
  }
  
  // Method to add interests
  void addInterest(String interest) {
    if (!_interests.contains(interest)) {
      _interests.add(interest);
      notifyListeners();
    }
  }
  
  // Method to add skills
  void addSkill(String skill) {
    if (!_skills.contains(skill)) {
      _skills.add(skill);
      notifyListeners();
    }
  }
  
  // Method to add magic abilities
  void addMagicAbility(String ability) {
    if (!_magicAbilities.contains(ability)) {
      _magicAbilities.add(ability);
      notifyListeners();
    }
  }
  
  // Method to summon a shadow clone
  void summonShadowClone() {
    // Logic for shadow clone summoning
    notifyListeners();
  }
  
  // Method to summon a shadow minion for a task
  void summonShadowMinion(String taskDescription, String category) {
    final minion = MinionModel(
      id: DateTime.now().millisecondsSinceEpoch.toString(),
      taskDescription: taskDescription,
      category: category,
    );
    _minions.add(minion);
    notifyListeners();
  }
  
  // Method to update position
  void updatePosition(double x, double y, double z) {
    _positionX = x;
    _positionY = y;
    _positionZ = z;
    notifyListeners();
  }
  
  // Method to update current action
  void updateAction(String action) {
    _currentAction = action;
    notifyListeners();
  }
  
  // Method to walk to a specific position
  void walkTo(double x, double y, double z) {
    updateAction("walking");
    // In a real app, you'd animate the movement
    updatePosition(x, y, z);
    // After reaching, return to idle
    updateAction("idle");
  }
  
  // Method to think (grab chin)
  void think() {
    updateAction("thinking");
  }
  
  // Method to point at something
  void pointAt(double x, double y, double z) {
    updateAction("pointing");
    // Logic for pointing at a specific location
  }
  
  // Get a character property
  String getProperty(String key) {
    return _properties[key] ?? '';
  }
  
  // Set a character property
  void setProperty(String key, String value) {
    _properties[key] = value;
    notifyListeners();
  }
} 