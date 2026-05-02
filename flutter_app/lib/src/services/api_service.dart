import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:flutter/foundation.dart';

import '../models/quest.dart';
import '../models/skill.dart';
import '../models/character.dart';

class ApiService {
  final String baseUrl;
  
  ApiService({required this.baseUrl});
  
  // Factory constructor for default API URL
  factory ApiService.defaultUrl() {
    // Use localhost for desktop, or 10.0.2.2 for Android emulator
    String host = kIsWeb ? 
      'localhost' : 
      (defaultTargetPlatform == TargetPlatform.android ? '10.0.2.2' : 'localhost');
    
    return ApiService(baseUrl: 'http://$host:8080/api');
  }
  
  // Helper for HTTP GET requests
  Future<dynamic> _get(String endpoint) async {
    try {
      final response = await http.get(
        Uri.parse('$baseUrl/$endpoint'),
        headers: {'Content-Type': 'application/json'},
      );
      
      if (response.statusCode >= 200 && response.statusCode < 300) {
        return jsonDecode(response.body);
      } else {
        throw _handleError(response);
      }
    } catch (e) {
      throw Exception('Network error: $e');
    }
  }
  
  // Helper for HTTP POST requests
  Future<dynamic> _post(String endpoint, Map<String, dynamic> data) async {
    try {
      final response = await http.post(
        Uri.parse('$baseUrl/$endpoint'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode(data),
      );
      
      if (response.statusCode >= 200 && response.statusCode < 300) {
        return jsonDecode(response.body);
      } else {
        throw _handleError(response);
      }
    } catch (e) {
      throw Exception('Network error: $e');
    }
  }
  
  // Helper for HTTP PUT requests
  Future<dynamic> _put(String endpoint, Map<String, dynamic> data) async {
    try {
      final response = await http.put(
        Uri.parse('$baseUrl/$endpoint'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode(data),
      );
      
      if (response.statusCode >= 200 && response.statusCode < 300) {
        return jsonDecode(response.body);
      } else {
        throw _handleError(response);
      }
    } catch (e) {
      throw Exception('Network error: $e');
    }
  }
  
  // Helper for HTTP DELETE requests
  Future<void> _delete(String endpoint) async {
    try {
      final response = await http.delete(
        Uri.parse('$baseUrl/$endpoint'),
        headers: {'Content-Type': 'application/json'},
      );
      
      if (response.statusCode < 200 || response.statusCode >= 300) {
        throw _handleError(response);
      }
    } catch (e) {
      throw Exception('Network error: $e');
    }
  }
  
  // Error handler
  Exception _handleError(http.Response response) {
    try {
      final error = jsonDecode(response.body);
      return Exception('API error (${response.statusCode}): ${error['message']}');
    } catch (e) {
      return Exception('API error (${response.statusCode}): ${response.body}');
    }
  }
  
  // QUEST METHODS
  
  // Get all quests
  Future<List<Quest>> getQuests({int limit = 10, int offset = 0}) async {
    final data = await _get('quests?limit=$limit&offset=$offset');
    return (data['data'] as List).map((json) => Quest.fromJson(json)).toList();
  }
  
  // Get quests by skills (exact match)
  Future<List<Quest>> getQuestsBySkills(List<String> skillIds, {bool exactMatch = true}) async {
    final skillsParam = skillIds.join(',');
    final data = await _get('quests?skills=$skillsParam&exact_match=$exactMatch');
    return (data['data'] as List).map((json) => Quest.fromJson(json)).toList();
  }
  
  // Get a quest by ID
  Future<Quest> getQuestById(String id) async {
    final data = await _get('quests/$id');
    return Quest.fromJson(data);
  }
  
  // Create a new quest
  Future<Quest> createQuest(QuestCreate quest) async {
    final data = await _post('quests', quest.toJson());
    return Quest.fromJson(data);
  }
  
  // Update a quest
  Future<Quest> updateQuest(String id, QuestUpdate quest) async {
    final data = await _put('quests/$id', quest.toJson());
    return Quest.fromJson(data);
  }
  
  // Delete a quest
  Future<void> deleteQuest(String id) async {
    await _delete('quests/$id');
  }
  
  // Get available quests for a character
  Future<List<Quest>> getAvailableQuestsForCharacter(String characterId, {bool includeTeam = false}) async {
    final data = await _get('quests/available/$characterId?include_team=$includeTeam');
    return (data['data'] as List).map((json) => Quest.fromJson(json)).toList();
  }
  
  // SKILL METHODS
  
  // Get all skills
  Future<List<Skill>> getSkills({int limit = 10, int offset = 0}) async {
    final data = await _get('skills?limit=$limit&offset=$offset');
    return (data['data'] as List).map((json) => Skill.fromJson(json)).toList();
  }
  
  // Get a skill by ID
  Future<Skill> getSkillById(String id) async {
    final data = await _get('skills/$id');
    return Skill.fromJson(data);
  }
  
  // Create a new skill
  Future<Skill> createSkill(SkillCreate skill) async {
    final data = await _post('skills', skill.toJson());
    return Skill.fromJson(data);
  }
  
  // Get character skills
  Future<List<Skill>> getCharacterSkills(String characterId) async {
    final data = await _get('skills/character/$characterId');
    return (data['data'] as List).map((json) => Skill.fromJson(json)).toList();
  }
  
  // Add skills to a character
  Future<List<Skill>> addSkillsToCharacter(String characterId, List<String> skillIds) async {
    final data = await _post('skills/character/$characterId', {'skill_ids': skillIds});
    return (data['data'] as List).map((json) => Skill.fromJson(json)).toList();
  }
  
  // CHARACTER METHODS
  
  // Get all characters
  Future<List<Character>> getCharacters({int limit = 10, int offset = 0}) async {
    final data = await _get('characters?limit=$limit&offset=$offset');
    return (data['data'] as List).map((json) => Character.fromJson(json)).toList();
  }
  
  // Get a character by ID
  Future<Character> getCharacterById(String id) async {
    final data = await _get('characters/$id');
    return Character.fromJson(data);
  }
  
  // Create a new character
  Future<Character> createCharacter(CharacterCreate character) async {
    final data = await _post('characters', character.toJson());
    return Character.fromJson(data);
  }
  
  // Update a character
  Future<Character> updateCharacter(String id, CharacterUpdate character) async {
    final data = await _put('characters/$id', character.toJson());
    return Character.fromJson(data);
  }
  
  // Delete a character
  Future<void> deleteCharacter(String id) async {
    await _delete('characters/$id');
  }
  
  // Get team characters
  Future<List<Character>> getTeamCharacters(String teamId) async {
    final data = await _get('characters/team/$teamId');
    return (data['data'] as List).map((json) => Character.fromJson(json)).toList();
  }
} 