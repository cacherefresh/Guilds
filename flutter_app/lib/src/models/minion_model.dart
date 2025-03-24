import 'package:flutter/foundation.dart';

enum MinionState {
  loaded,
  start,
  doing,
  complete,
  canceled
}

class MinionModel extends ChangeNotifier {
  final String id;
  final String taskDescription;
  final String category;
  String _requestor;
  String _assignee;
  MinionState _state = MinionState.loaded;
  double _reward = 0.0;
  
  MinionModel({
    required this.id,
    required this.taskDescription,
    required this.category,
    String? requestor,
    String? assignee,
    double reward = 0.0,
  }) : 
    _requestor = requestor ?? "Self",
    _assignee = assignee ?? "Self",
    _reward = reward;
  
  // Getters
  String get requestor => _requestor;
  String get assignee => _assignee;
  MinionState get state => _state;
  double get reward => _reward;
  bool get isDone => _state == MinionState.complete || _state == MinionState.canceled;
  
  // Setters with notification
  set requestor(String newRequestor) {
    _requestor = newRequestor;
    notifyListeners();
  }
  
  set assignee(String newAssignee) {
    _assignee = newAssignee;
    notifyListeners();
  }
  
  set reward(double newReward) {
    _reward = newReward;
    notifyListeners();
  }
  
  // State transition methods
  void start() {
    if (_state == MinionState.loaded) {
      _state = MinionState.start;
      notifyListeners();
    }
  }
  
  void doing() {
    if (_state == MinionState.start) {
      _state = MinionState.doing;
      notifyListeners();
    }
  }
  
  void complete() {
    if (_state == MinionState.doing) {
      _state = MinionState.complete;
      notifyListeners();
    }
  }
  
  void cancel() {
    if (_state != MinionState.complete && _state != MinionState.canceled) {
      _state = MinionState.canceled;
      notifyListeners();
    }
  }
  
  // Method to convert to JSON for API communication
  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'taskDescription': taskDescription,
      'category': category,
      'requestor': _requestor,
      'assignee': _assignee,
      'state': _state.toString().split('.').last,
      'reward': _reward,
      'isDone': isDone,
    };
  }
  
  // Method to create from JSON from API
  factory MinionModel.fromJson(Map<String, dynamic> json) {
    return MinionModel(
      id: json['id'],
      taskDescription: json['taskDescription'],
      category: json['category'],
      requestor: json['requestor'],
      assignee: json['assignee'],
      reward: json['reward'],
    )..setStateFromString(json['state']);
  }
  
  // Helper to set state from string
  void setStateFromString(String stateStr) {
    switch (stateStr) {
      case 'loaded':
        _state = MinionState.loaded;
        break;
      case 'start':
        _state = MinionState.start;
        break;
      case 'doing':
        _state = MinionState.doing;
        break;
      case 'complete':
        _state = MinionState.complete;
        break;
      case 'canceled':
        _state = MinionState.canceled;
        break;
      default:
        _state = MinionState.loaded;
    }
  }
} 