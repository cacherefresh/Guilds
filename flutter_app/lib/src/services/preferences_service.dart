import 'package:shared_preferences/shared_preferences.dart';

/// Service to manage user preferences
class PreferencesService {
  static const String _ANIMATIONS_ENABLED_KEY = 'animations_enabled';
  static const String _GAME_TERMINOLOGY_KEY = 'game_terminology';
  
  /// Save animation settings to persistent storage
  Future<void> saveAnimationSettings(bool enabled) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool(_ANIMATIONS_ENABLED_KEY, enabled);
  }
  
  /// Load animation settings from persistent storage
  /// Defaults to no-animation mode (false) if not set
  Future<bool> loadAnimationSettings() async {
    final prefs = await SharedPreferences.getInstance();
    // Default to no-animation mode (false)
    return prefs.getBool(_ANIMATIONS_ENABLED_KEY) ?? false;
  }
  
  /// Save terminology settings to persistent storage
  Future<void> saveTerminologySettings(bool useGameTerminology) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool(_GAME_TERMINOLOGY_KEY, useGameTerminology);
  }
  
  /// Load terminology settings from persistent storage
  /// Defaults to game terminology in standalone mode
  Future<bool> loadTerminologySettings() async {
    final prefs = await SharedPreferences.getInstance();
    // Default to game terminology in standalone mode, otherwise load from prefs
    return isStandaloneMode() 
        ? true 
        : (prefs.getBool(_GAME_TERMINOLOGY_KEY) ?? true);
  }
  
  /// Determine if the app is running in standalone mode
  bool isStandaloneMode() {
    // Implementation details would depend on how standalone mode is detected
    // For now, return true as the default value
    return true;
  }
} 