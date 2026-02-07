import 'package:flutter/material.dart';
import '../services/preferences_service.dart';

/// Provides animation settings for the app
/// Default is no-animation mode with label display
class AnimationSettings extends ChangeNotifier {
  // Default to no-animation mode
  bool _animationsEnabled = false;
  
  // Duration for state change flicker effect
  static Duration flickerDuration = Duration(milliseconds: 1000);
  
  // Get animation state
  bool get animationsEnabled => _animationsEnabled;
  
  // Toggle animations
  void toggleAnimations() {
    _animationsEnabled = !_animationsEnabled;
    // Save preference
    PreferencesService().saveAnimationSettings(_animationsEnabled);
    notifyListeners();
  }
  
  // Enable animations
  void enableAnimations() {
    if (!_animationsEnabled) {
      _animationsEnabled = true;
      // Save preference
      PreferencesService().saveAnimationSettings(_animationsEnabled);
      notifyListeners();
    }
  }
  
  // Disable animations
  void disableAnimations() {
    if (_animationsEnabled) {
      _animationsEnabled = false;
      // Save preference
      PreferencesService().saveAnimationSettings(_animationsEnabled);
      notifyListeners();
    }
  }
} 