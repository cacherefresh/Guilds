# No-Animation Mode Implementation

## Overview

This document outlines the implementation of a "No-Animation Mode" for the Guild application. This mode will skip animations but maintain model positioning, display state information as text labels, and use color indicators for state changes.

## Animation Settings Provider

```dart
// lib/providers/animation_settings_provider.dart
class AnimationSettings extends ChangeNotifier {
  // Default to no-animation mode
  bool _animationsEnabled = false;
  
  // Duration for state change flicker effect
  final Duration flickerDuration = Duration(milliseconds: 1000);
  
  // Get animation state
  bool get animationsEnabled => _animationsEnabled;
  
  // Toggle animations
  void toggleAnimations() {
    _animationsEnabled = !_animationsEnabled;
    notifyListeners();
  }
  
  // Enable animations
  void enableAnimations() {
    if (!_animationsEnabled) {
      _animationsEnabled = true;
      notifyListeners();
    }
  }
  
  // Disable animations
  void disableAnimations() {
    if (_animationsEnabled) {
      _animationsEnabled = false;
      notifyListeners();
    }
  }
}
```

## Main App Configuration

```dart
// lib/main.dart
void main() {
  runApp(
    MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (_) => AnimationSettings()),
        ChangeNotifierProvider(create: (_) => LanguageProvider()),
        // Other providers
      ],
      child: MyApp(),
    ),
  );
}
```

## Character Widget with No-Animation Support

```dart
// lib/widgets/character_widget.dart
class CharacterWidget extends StatefulWidget {
  final Character character;
  
  const CharacterWidget({Key? key, required this.character}) : super(key: key);
  
  @override
  _CharacterWidgetState createState() => _CharacterWidgetState();
}

class _CharacterWidgetState extends State<CharacterWidget> {
  // Track previous state to detect changes
  CharacterState? _previousState;
  String? _previousAnimation;
  bool _isFlickering = false;
  Timer? _flickerTimer;
  
  @override
  void initState() {
    super.initState();
    _previousState = widget.character.state;
    _previousAnimation = widget.character.animation.currentAnimationName;
  }
  
  @override
  void didUpdateWidget(CharacterWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    
    // Check for state or animation changes
    if (_previousState != widget.character.state || 
        _previousAnimation != widget.character.animation.currentAnimationName) {
      // Start flickering effect
      _startFlicker();
      
      // Update previous values
      _previousState = widget.character.state;
      _previousAnimation = widget.character.animation.currentAnimationName;
    }
  }
  
  void _startFlicker() {
    // Cancel existing flicker if any
    _flickerTimer?.cancel();
    
    // Start flickering
    setState(() {
      _isFlickering = true;
    });
    
    // Stop flickering after duration
    _flickerTimer = Timer(
      Provider.of<AnimationSettings>(context, listen: false).flickerDuration,
      () {
        if (mounted) {
          setState(() {
            _isFlickering = false;
          });
        }
      }
    );
  }
  
  @override
  Widget build(BuildContext context) {
    final position = widget.character.position;
    final animationSettings = Provider.of<AnimationSettings>(context);
    
    // Choose image based on animation setting
    final String displayImage = animationSettings.animationsEnabled
        ? widget.character.animation.currentFrameImage
        : 'assets/images/character/static.png';
    
    return Positioned(
      left: position.x,
      top: position.y,
      child: Column(
        children: [
          // Character sprite with conditional flicker effect
          Container(
            decoration: _isFlickering 
                ? BoxDecoration(
                    border: Border.all(color: Colors.amber, width: 2),
                    boxShadow: [
                      BoxShadow(
                        color: Colors.amber.withOpacity(0.6),
                        blurRadius: 10,
                        spreadRadius: 5,
                      ),
                    ],
                  )
                : null,
            child: Image.asset(
              displayImage,
              width: 64,
              height: 64,
              color: _isFlickering ? Colors.amber.withOpacity(0.3) : null,
              colorBlendMode: _isFlickering ? BlendMode.srcATop : null,
            ),
          ),
          
          // Only show state text in no-animation mode
          if (!animationSettings.animationsEnabled)
            Container(
              padding: EdgeInsets.all(4),
              decoration: BoxDecoration(
                color: Colors.black.withOpacity(0.7),
                borderRadius: BorderRadius.circular(4),
              ),
              child: Column(
                children: [
                  // State label
                  Text(
                    _getStateText(widget.character.state),
                    style: TextStyle(color: Colors.white, fontSize: 10),
                  ),
                  // Animation name
                  Text(
                    'Animation: ${widget.character.animation.currentAnimationName}',
                    style: TextStyle(color: Colors.white, fontSize: 10),
                  ),
                ],
              ),
            ),
        ],
      ),
    );
  }
  
  String _getStateText(CharacterState state) {
    switch (state) {
      case CharacterState.active:
        return 'Active';
      case CharacterState.idle:
        return 'Idle';
      case CharacterState.walking:
        return 'Walking';
      case CharacterState.sittingOnThrone:
        return 'Sitting on Throne';
      case CharacterState.casting:
        return 'Casting';
      case CharacterState.interacting:
        return 'Interacting';
      default:
        return 'Unknown';
    }
  }
  
  @override
  void dispose() {
    _flickerTimer?.cancel();
    super.dispose();
  }
}
```

## Minion Widget with No-Animation Support

```dart
// lib/widgets/minion_widget.dart
class MinionWidget extends StatefulWidget {
  final Minion minion;
  
  const MinionWidget({Key? key, required this.minion}) : super(key: key);
  
  @override
  _MinionWidgetState createState() => _MinionWidgetState();
}

class _MinionWidgetState extends State<MinionWidget> {
  // Track previous state to detect changes
  MinionState? _previousState;
  String? _previousAnimation;
  bool _isFlickering = false;
  Timer? _flickerTimer;
  
  @override
  void initState() {
    super.initState();
    _previousState = widget.minion.state;
    _previousAnimation = widget.minion.animation.currentAnimationName;
  }
  
  @override
  void didUpdateWidget(MinionWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    
    // Check for state or animation changes
    if (_previousState != widget.minion.state || 
        _previousAnimation != widget.minion.animation.currentAnimationName) {
      // Start flickering effect
      _startFlicker();
      
      // Update previous values
      _previousState = widget.minion.state;
      _previousAnimation = widget.minion.animation.currentAnimationName;
    }
  }
  
  void _startFlicker() {
    // Cancel existing flicker if any
    _flickerTimer?.cancel();
    
    // Start flickering
    setState(() {
      _isFlickering = true;
    });
    
    // Stop flickering after duration
    _flickerTimer = Timer(
      Provider.of<AnimationSettings>(context, listen: false).flickerDuration,
      () {
        if (mounted) {
          setState(() {
            _isFlickering = false;
          });
        }
      }
    );
  }
  
  @override
  Widget build(BuildContext context) {
    final position = widget.minion.position;
    final animationSettings = Provider.of<AnimationSettings>(context);
    
    // Choose image based on animation setting
    final String displayImage = animationSettings.animationsEnabled
        ? widget.minion.animation.currentFrameImage
        : 'assets/images/minion/static.png';
    
    return Positioned(
      left: position.x,
      top: position.y,
      child: Column(
        children: [
          // Minion sprite with conditional flicker effect
          Container(
            decoration: _isFlickering 
                ? BoxDecoration(
                    border: Border.all(color: Colors.amber, width: 2),
                    boxShadow: [
                      BoxShadow(
                        color: Colors.amber.withOpacity(0.6),
                        blurRadius: 10,
                        spreadRadius: 5,
                      ),
                    ],
                  )
                : null,
            child: Image.asset(
              displayImage,
              width: 48,
              height: 48,
              color: _isFlickering ? Colors.amber.withOpacity(0.3) : null,
              colorBlendMode: _isFlickering ? BlendMode.srcATop : null,
            ),
          ),
          
          // Only show state text in no-animation mode
          if (!animationSettings.animationsEnabled)
            Container(
              padding: EdgeInsets.all(4),
              decoration: BoxDecoration(
                color: Colors.black.withOpacity(0.7),
                borderRadius: BorderRadius.circular(4),
              ),
              child: Column(
                children: [
                  // State label
                  Text(
                    _getStateText(widget.minion.state),
                    style: TextStyle(color: Colors.white, fontSize: 10),
                  ),
                  // Animation name
                  Text(
                    'Animation: ${widget.minion.animation.currentAnimationName}',
                    style: TextStyle(color: Colors.white, fontSize: 10),
                  ),
                ],
              ),
            ),
        ],
      ),
    );
  }
  
  String _getStateText(MinionState state) {
    switch (state) {
      case MinionState.idle:
        return 'Idle';
      case MinionState.following:
        return 'Following';
      case MinionState.assigned:
        return 'Assigned';
      case MinionState.running:
        return 'Running';
      case MinionState.completing:
        return 'Completing';
      case MinionState.completed:
        return 'Completed';
      default:
        return 'Unknown';
    }
  }
  
  @override
  void dispose() {
    _flickerTimer?.cancel();
    super.dispose();
  }
}
```

## Character and Minion Animation Controllers

```dart
// lib/animation/character_animation.dart
class CharacterAnimation {
  // ... existing code ...
  
  // Add method to get static frame for no-animation mode
  String getStaticFrameForAnimation(String animationName) {
    if (!_animations.containsKey(animationName)) {
      return 'assets/images/character/static.png';
    }
    
    // Return the most representative frame from the animation
    final frames = _animations[animationName]!;
    
    // For animations with a hold frame (duration -1), use that
    for (int i = 0; i < frames.length; i++) {
      if (frames[i].duration == -1) {
        return frames[i].image;
      }
    }
    
    // Otherwise use the first frame
    return frames.first.image;
  }
}

// lib/animation/minion_animation.dart
class MinionAnimation {
  // ... existing code ...
  
  // Add method to get static frame for no-animation mode
  String getStaticFrameForAnimation(String animationName) {
    if (!_animations.containsKey(animationName)) {
      return 'assets/images/minion/static.png';
    }
    
    // Return the most representative frame from the animation
    final frames = _animations[animationName]!;
    
    // For animations with a hold frame (duration -1), use that
    for (int i = 0; i < frames.length; i++) {
      if (frames[i].duration == -1) {
        return frames[i].image;
      }
    }
    
    // Otherwise use the first frame
    return frames.first.image;
  }
}
```

## Modifying Character and Minion Controllers

```dart
// lib/controllers/character_idle_controller.dart
class CharacterIdleController {
  // ... existing code ...
  
  Future<void> walkToThrone(WorldLocation throneLocation) async {
    character.setState(CharacterState.walking);
    
    // Get animation settings
    final animationsEnabled = AnimationSettings().animationsEnabled;
    
    // Calculate path to throne
    final path = PathFinder.findPath(
      character.position,
      throneLocation.position,
    );
    
    if (animationsEnabled) {
      // Walk along path with animation
      for (final point in path) {
        character.position = point;
        // Wait for animation frame
        await Future.delayed(Duration(milliseconds: 100));
      }
    } else {
      // In no-animation mode, just move to final position
      // but still set the animation name so it's displayed in the label
      character.playAnimation('walk');
      character.position = throneLocation.position;
      await Future.delayed(Duration(milliseconds: 100));
    }
  }
  
  // ... rest of existing code ...
}

// lib/controllers/minion_controller.dart
class MinionController {
  // ... existing code ...
  
  Future<void> walkTo(Position position) async {
    // Get animation settings
    final animationsEnabled = AnimationSettings().animationsEnabled;
    
    // Calculate path to position
    final path = PathFinder.findPath(
      minion.position,
      position,
    );
    
    if (animationsEnabled) {
      // Walk along path with animation
      for (final point in path) {
        minion.position = point;
        // Play walking animation
        minion.playAnimation('walk');
        // Wait for animation frame
        await Future.delayed(Duration(milliseconds: 100));
      }
    } else {
      // In no-animation mode, just move to final position
      // but still set the animation name so it's displayed in the label
      minion.playAnimation('walk');
      minion.position = position;
      await Future.delayed(Duration(milliseconds: 100));
    }
    
    // Reset animation
    minion.resetAnimation();
  }
  
  // ... rest of existing code ...
}
```

## Settings UI for Animation Control

```dart
// lib/screens/settings_screen.dart
class SettingsScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: TermText('settings_title'),
      ),
      body: ListView(
        children: [
          // Animation toggle
          SwitchListTile(
            title: TermText('enable_animations'),
            subtitle: TermText('animations_description'),
            value: Provider.of<AnimationSettings>(context).animationsEnabled,
            onChanged: (value) {
              Provider.of<AnimationSettings>(context, listen: false)
                  .toggleAnimations();
            },
          ),
          
          // Language toggle (if not in standalone mode)
          if (!isStandaloneMode())
            SwitchListTile(
              title: TermText('use_game_terminology'),
              subtitle: TermText('terminology_description'),
              value: Provider.of<LanguageProvider>(context).useGameTerminology,
              onChanged: (value) {
                Provider.of<LanguageProvider>(context, listen: false)
                    .toggleTerminology();
              },
            ),
            
          // Other settings...
        ],
      ),
    );
  }
}
```

## Storage for User Preferences

```dart
// lib/services/preferences_service.dart
class PreferencesService {
  static const String _ANIMATIONS_ENABLED_KEY = 'animations_enabled';
  static const String _GAME_TERMINOLOGY_KEY = 'game_terminology';
  
  Future<void> saveAnimationSettings(bool enabled) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool(_ANIMATIONS_ENABLED_KEY, enabled);
  }
  
  Future<bool> loadAnimationSettings() async {
    final prefs = await SharedPreferences.getInstance();
    // Default to no-animation mode (false)
    return prefs.getBool(_ANIMATIONS_ENABLED_KEY) ?? false;
  }
  
  Future<void> saveTerminologySettings(bool useGameTerminology) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool(_GAME_TERMINOLOGY_KEY, useGameTerminology);
  }
  
  Future<bool> loadTerminologySettings() async {
    final prefs = await SharedPreferences.getInstance();
    // Default to game terminology in standalone mode, otherwise load from prefs
    return isStandaloneMode() 
        ? true 
        : (prefs.getBool(_GAME_TERMINOLOGY_KEY) ?? true);
  }
}
```

## Initialize Settings on App Start

```dart
// lib/main.dart
void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Load preferences
  final prefsService = PreferencesService();
  final animationsEnabled = await prefsService.loadAnimationSettings();
  final useGameTerminology = await prefsService.loadTerminologySettings();
  
  // Create providers with loaded settings
  final animationSettings = AnimationSettings()
    ..(_animationsEnabled = animationsEnabled);
  
  final languageProvider = LanguageProvider()
    ..(_useGameTerminology = useGameTerminology)
    ..initialize();
  
  runApp(
    MultiProvider(
      providers: [
        ChangeNotifierProvider.value(value: animationSettings),
        ChangeNotifierProvider.value(value: languageProvider),
        // Other providers
      ],
      child: MyApp(),
    ),
  );
}
```

## Persist Settings on Change

```dart
// lib/providers/animation_settings_provider.dart
class AnimationSettings extends ChangeNotifier {
  // ... existing code ...
  
  void toggleAnimations() {
    _animationsEnabled = !_animationsEnabled;
    // Save preference
    PreferencesService().saveAnimationSettings(_animationsEnabled);
    notifyListeners();
  }
  
  // ... rest of existing code ...
}

// lib/providers/language_provider.dart
class LanguageProvider extends ChangeNotifier {
  // ... existing code ...
  
  Future<void> toggleTerminology() async {
    _useGameTerminology = !_useGameTerminology;
    // Save preference
    await PreferencesService().saveTerminologySettings(_useGameTerminology);
    await _loadTerminology();
    notifyListeners();
  }
  
  // ... rest of existing code ...
}
```

## Implementation Steps

1. **Create Animation Settings Provider**
   - Default to no-animation mode (`animationsEnabled = false`)
   - Implement toggle functionality
   - Add flicker duration setting

2. **Update Character and Minion Widgets**
   - Add state change detection
   - Implement golden flicker effect
   - Add state and animation text display
   - Conditionally show animations based on settings

3. **Modify Controllers**
   - Update movement methods to check animation settings
   - Skip intermediate positions in no-animation mode
   - Maintain correct state information

4. **Create Settings UI**
   - Add toggle for animation settings
   - Ensure settings are preserved

5. **Add Storage**
   - Implement preference persistence
   - Default to no-animation mode on fresh installs

6. **Testing**
   - Verify animations are disabled by default
   - Test state change detection and flickering
   - Ensure text labels correctly display state and animation
   - Confirm transitions still work correctly in no-animation mode 