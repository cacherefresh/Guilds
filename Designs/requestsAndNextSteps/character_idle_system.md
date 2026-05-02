# Character Idle System & Minion Behavior Implementation

## Overview

This document outlines the implementation of the character idle system and minion behavior in the Guild application. The system detects when the user has been inactive for a few minutes, sets the character to idle state, and initiates appropriate animations and behaviors for both the main character and their minions.

## Character Idle Detection

### Activity Tracker

```dart
// lib/system/activity_tracker.dart
class ActivityTracker {
  // Last recorded interaction time
  DateTime _lastInteractionTime = DateTime.now();
  
  // Timer for checking idle status
  Timer? _idleTimer;
  
  // Idle threshold (2 minutes)
  final Duration idleThreshold = Duration(minutes: 2);
  
  // Current state
  bool _isIdle = false;
  
  // Callbacks
  final Function() onBecomeIdle;
  final Function() onBecomeActive;
  
  ActivityTracker({
    required this.onBecomeIdle,
    required this.onBecomeActive,
  }) {
    _startIdleTimer();
  }
  
  bool get isIdle => _isIdle;
  
  void recordActivity() {
    _lastInteractionTime = DateTime.now();
    
    // If was idle, notify activity resumed
    if (_isIdle) {
      _isIdle = false;
      onBecomeActive();
    }
  }
  
  void _startIdleTimer() {
    _idleTimer = Timer.periodic(Duration(seconds: 30), (timer) {
      // Check if idle threshold has been reached
      final timeSinceLastInteraction = 
          DateTime.now().difference(_lastInteractionTime);
      
      if (!_isIdle && timeSinceLastInteraction >= idleThreshold) {
        _isIdle = true;
        onBecomeIdle();
      }
    });
  }
  
  void dispose() {
    _idleTimer?.cancel();
  }
}
```

### Menu State Tracker

```dart
// lib/system/menu_state_tracker.dart
class MenuStateTracker {
  bool _isAnyMenuOpen = false;
  final Set<String> _openMenus = {};
  
  bool get isAnyMenuOpen => _isAnyMenuOpen;
  
  void registerMenuOpen(String menuId) {
    _openMenus.add(menuId);
    _updateState();
  }
  
  void registerMenuClose(String menuId) {
    _openMenus.remove(menuId);
    _updateState();
  }
  
  void _updateState() {
    _isAnyMenuOpen = _openMenus.isNotEmpty;
  }
}
```

## Character Motion Controllers

### Character Idle Controller

```dart
// lib/controllers/character_idle_controller.dart
class CharacterIdleController {
  final Character character;
  final LocationManager locationManager;
  final MenuStateTracker menuStateTracker;
  
  CharacterIdleController({
    required this.character,
    required this.locationManager,
    required this.menuStateTracker,
  });
  
  Future<void> initiateIdleSequence() async {
    // If a menu is open, just set idle state but don't move character
    if (menuStateTracker.isAnyMenuOpen) {
      character.setState(CharacterState.idle);
      return;
    }
    
    // Find the throne location
    final throneLocation = locationManager
        .getLocationsOfType(LocationType.throne)
        .firstOrNull;
    
    // If no throne found, just set idle state
    if (throneLocation == null) {
      character.setState(CharacterState.idle);
      return;
    }
    
    // Walk to throne
    await walkToThrone(throneLocation);
    
    // Sit on throne
    sitOnThrone();
  }
  
  Future<void> walkToThrone(WorldLocation throneLocation) async {
    character.setState(CharacterState.walking);
    
    // Calculate path to throne
    final path = PathFinder.findPath(
      character.position,
      throneLocation.position,
    );
    
    // Walk along path
    for (final point in path) {
      character.position = point;
      // Wait for animation frame
      await Future.delayed(Duration(milliseconds: 100));
    }
  }
  
  void sitOnThrone() {
    character.setState(CharacterState.sittingOnThrone);
    character.playAnimation('sit_on_throne');
  }
  
  void exitIdleState() {
    character.setState(CharacterState.active);
    character.playAnimation('stand_up');
  }
}
```

## Minion Behavior System

### Minion State Controller

```dart
// lib/controllers/minion_controller.dart
enum MinionState {
  idle,
  following,
  assigned,
  running,
  completing,
  completed,
}

class MinionController {
  final Minion minion;
  final Character owner;
  final LocationManager locationManager;
  
  // Animation timer
  Timer? _animationTimer;
  
  MinionController({
    required this.minion,
    required this.owner,
    required this.locationManager,
  });
  
  void updateState() {
    // If minion is assigned to a quest, handle quest animations
    if (minion.state == MinionState.assigned) {
      // Minion is already in correct state
      return;
    }
    
    // If owner is idle and sitting on throne, follow owner
    if (owner.state == CharacterState.sittingOnThrone) {
      if (minion.state != MinionState.following) {
        followOwner();
      }
      return;
    }
    
    // Default to idle state if no other conditions apply
    if (minion.state != MinionState.idle) {
      minion.state = MinionState.idle;
    }
  }
  
  // Follow owner behavior
  void followOwner() {
    minion.state = MinionState.following;
    
    // Calculate position near owner's throne
    final ownerPosition = owner.position;
    final followPosition = Position(
      x: ownerPosition.x + (Random().nextDouble() * 4 - 2),
      y: ownerPosition.y + (Random().nextDouble() * 2 + 2),
    );
    
    // Walk to follow position
    walkTo(followPosition);
  }
  
  // Assign minion to quest
  void assignToQuest(Quest quest) {
    minion.assignedQuest = quest;
    minion.state = MinionState.assigned;
    
    // Cancel any existing animation timer
    _animationTimer?.cancel();
    
    // Walk to bottom left of guild
    final questWorkPosition = Position(x: 5.0, y: 25.0);
    walkTo(questWorkPosition).then((_) {
      // Start running animation every 5 seconds
      _startQuestRunningAnimation();
    });
  }
  
  void _startQuestRunningAnimation() {
    _animationTimer = Timer.periodic(Duration(seconds: 5), (timer) {
      minion.state = MinionState.running;
      minion.playAnimation('quest_running');
      
      // Reset to assigned state after animation completes
      Future.delayed(Duration(milliseconds: 800), () {
        if (minion.state == MinionState.running) {
          minion.state = MinionState.assigned;
        }
      });
    });
  }
  
  // Complete quest
  void completeQuest() {
    // Cancel running animation timer
    _animationTimer?.cancel();
    
    // Set completing state
    minion.state = MinionState.completing;
    
    // Walk to center bottom
    final completionPosition = Position(x: 15.0, y: 28.0);
    walkTo(completionPosition).then((_) {
      // Play completion animation
      minion.state = MinionState.completed;
      minion.playAnimation('completed_task');
      
      // Reset to idle after animation
      Future.delayed(Duration(seconds: 3), () {
        minion.state = MinionState.idle;
        minion.assignedQuest = null;
      });
    });
  }
  
  // Generic walk to position method
  Future<void> walkTo(Position position) async {
    // Calculate path to position
    final path = PathFinder.findPath(
      minion.position,
      position,
    );
    
    // Walk along path
    for (final point in path) {
      minion.position = point;
      // Play walking animation
      minion.playAnimation('walk');
      // Wait for animation frame
      await Future.delayed(Duration(milliseconds: 100));
    }
    
    // Reset animation
    minion.resetAnimation();
  }
  
  void dispose() {
    _animationTimer?.cancel();
  }
}
```

## Input Handling

### Keyboard and Mouse Input

```dart
// lib/input/input_handler.dart
class InputHandler {
  final ActivityTracker activityTracker;
  
  InputHandler({required this.activityTracker});
  
  void handleMouseMove(PointerEvent event) {
    activityTracker.recordActivity();
  }
  
  void handleMouseClick(PointerEvent event) {
    activityTracker.recordActivity();
  }
  
  void handleKeyPress(KeyEvent event) {
    activityTracker.recordActivity();
  }
}
```

### Input Binding

```dart
// lib/main.dart (or relevant widget)
class GameScreen extends StatefulWidget {
  @override
  _GameScreenState createState() => _GameScreenState();
}

class _GameScreenState extends State<GameScreen> {
  late ActivityTracker _activityTracker;
  late MenuStateTracker _menuStateTracker;
  late CharacterIdleController _idleController;
  late List<MinionController> _minionControllers = [];
  
  @override
  void initState() {
    super.initState();
    
    final character = Provider.of<CharacterProvider>(context, listen: false).currentCharacter;
    final locationManager = Provider.of<LocationManager>(context, listen: false);
    
    _menuStateTracker = MenuStateTracker();
    
    _idleController = CharacterIdleController(
      character: character,
      locationManager: locationManager,
      menuStateTracker: _menuStateTracker,
    );
    
    _activityTracker = ActivityTracker(
      onBecomeIdle: () {
        _idleController.initiateIdleSequence();
        _updateMinionStates();
      },
      onBecomeActive: () {
        _idleController.exitIdleState();
        _updateMinionStates();
      },
    );
    
    // Initialize minion controllers
    final minions = Provider.of<MinionProvider>(context, listen: false).minions;
    _minionControllers = minions.map((minion) => 
      MinionController(
        minion: minion,
        owner: character,
        locationManager: locationManager,
      )
    ).toList();
    
    // Start game loop
    _startGameLoop();
  }
  
  void _startGameLoop() {
    Timer.periodic(Duration(milliseconds: 16), (timer) {
      if (!mounted) {
        timer.cancel();
        return;
      }
      
      // Update minion states
      _updateMinionStates();
    });
  }
  
  void _updateMinionStates() {
    for (final controller in _minionControllers) {
      controller.updateState();
    }
  }
  
  @override
  Widget build(BuildContext context) {
    // Create input handler
    final inputHandler = InputHandler(activityTracker: _activityTracker);
    
    return Listener(
      onPointerMove: inputHandler.handleMouseMove,
      onPointerDown: inputHandler.handleMouseClick,
      child: KeyboardListener(
        focusNode: FocusNode(),
        onKeyEvent: inputHandler.handleKeyPress,
        child: Stack(
          children: [
            // Game world
            GameWorldWidget(),
            
            // UI elements
            for (final menuWidget in _buildMenuWidgets())
              menuWidget,
          ],
        ),
      ),
    );
  }
  
  List<Widget> _buildMenuWidgets() {
    return [
      // Quest menu
      _buildMenu(
        'quest_menu',
        QuestMenuWidget(
          onOpen: () => _menuStateTracker.registerMenuOpen('quest_menu'),
          onClose: () => _menuStateTracker.registerMenuClose('quest_menu'),
        ),
      ),
      // Other menus...
    ];
  }
  
  Widget _buildMenu(String id, Widget menuWidget) {
    // Wrap menu widget with logic to track open/closed state
    return menuWidget;
  }
  
  @override
  void dispose() {
    _activityTracker.dispose();
    for (final controller in _minionControllers) {
      controller.dispose();
    }
    super.dispose();
  }
}
```

## Animation System

### Character Animations

```dart
// lib/animation/character_animation.dart
class CharacterAnimation {
  // Animation frames
  final Map<String, List<Frame>> _animations = {
    'idle': [
      Frame(image: 'assets/animations/character/idle_1.png', duration: 500),
      Frame(image: 'assets/animations/character/idle_2.png', duration: 500),
    ],
    'walk': [
      Frame(image: 'assets/animations/character/walk_1.png', duration: 150),
      Frame(image: 'assets/animations/character/walk_2.png', duration: 150),
      Frame(image: 'assets/animations/character/walk_3.png', duration: 150),
      Frame(image: 'assets/animations/character/walk_4.png', duration: 150),
    ],
    'sit_on_throne': [
      Frame(image: 'assets/animations/character/sit_1.png', duration: 200),
      Frame(image: 'assets/animations/character/sit_2.png', duration: 200),
      Frame(image: 'assets/animations/character/sit_3.png', duration: 200),
      Frame(image: 'assets/animations/character/sitting.png', duration: -1), // -1 means hold
    ],
    'stand_up': [
      Frame(image: 'assets/animations/character/sit_3.png', duration: 200),
      Frame(image: 'assets/animations/character/sit_2.png', duration: 200),
      Frame(image: 'assets/animations/character/sit_1.png', duration: 200),
      Frame(image: 'assets/animations/character/idle_1.png', duration: 200),
    ],
    // Other animations...
  };
  
  // Current animation
  String _currentAnimation = 'idle';
  int _currentFrame = 0;
  int _frameTimer = 0;
  
  String get currentAnimationName => _currentAnimation;
  String get currentFrameImage => _animations[_currentAnimation]![_currentFrame].image;
  
  void update(int deltaTimeMs) {
    final frames = _animations[_currentAnimation]!;
    final frameDuration = frames[_currentFrame].duration;
    
    // If duration is -1, hold frame indefinitely
    if (frameDuration == -1) return;
    
    _frameTimer += deltaTimeMs;
    
    if (_frameTimer >= frameDuration) {
      _frameTimer = 0;
      _currentFrame = (_currentFrame + 1) % frames.length;
    }
  }
  
  void play(String animationName) {
    if (_animations.containsKey(animationName) && _currentAnimation != animationName) {
      _currentAnimation = animationName;
      _currentFrame = 0;
      _frameTimer = 0;
    }
  }
}
```

### Minion Animations

```dart
// lib/animation/minion_animation.dart
class MinionAnimation {
  // Animation frames
  final Map<String, List<Frame>> _animations = {
    'idle': [
      Frame(image: 'assets/animations/minion/idle_1.png', duration: 500),
      Frame(image: 'assets/animations/minion/idle_2.png', duration: 500),
    ],
    'walk': [
      Frame(image: 'assets/animations/minion/walk_1.png', duration: 150),
      Frame(image: 'assets/animations/minion/walk_2.png', duration: 150),
      Frame(image: 'assets/animations/minion/walk_3.png', duration: 150),
      Frame(image: 'assets/animations/minion/walk_4.png', duration: 150),
    ],
    'quest_running': [
      Frame(image: 'assets/animations/minion/run_1.png', duration: 100),
      Frame(image: 'assets/animations/minion/run_2.png', duration: 100),
      Frame(image: 'assets/animations/minion/run_3.png', duration: 100),
      Frame(image: 'assets/animations/minion/run_4.png', duration: 100),
      Frame(image: 'assets/animations/minion/run_5.png', duration: 100),
      Frame(image: 'assets/animations/minion/run_6.png', duration: 100),
    ],
    'completed_task': [
      Frame(image: 'assets/animations/minion/complete_1.png', duration: 200),
      Frame(image: 'assets/animations/minion/complete_2.png', duration: 200),
      Frame(image: 'assets/animations/minion/complete_3.png', duration: 200),
      Frame(image: 'assets/animations/minion/complete_4.png', duration: 200),
      Frame(image: 'assets/animations/minion/complete_5.png', duration: 200),
    ],
    // Other animations...
  };
  
  // Current animation
  String _currentAnimation = 'idle';
  int _currentFrame = 0;
  int _frameTimer = 0;
  
  String get currentAnimationName => _currentAnimation;
  String get currentFrameImage => _animations[_currentAnimation]![_currentFrame].image;
  
  void update(int deltaTimeMs) {
    final frames = _animations[_currentAnimation]!;
    final frameDuration = frames[_currentFrame].duration;
    
    // If duration is -1, hold frame indefinitely
    if (frameDuration == -1) return;
    
    _frameTimer += deltaTimeMs;
    
    if (_frameTimer >= frameDuration) {
      _frameTimer = 0;
      _currentFrame = (_currentFrame + 1) % frames.length;
    }
  }
  
  void play(String animationName) {
    if (_animations.containsKey(animationName) && _currentAnimation != animationName) {
      _currentAnimation = animationName;
      _currentFrame = 0;
      _frameTimer = 0;
    }
  }
}
```

## Integration with Game World

### Character Rendering

```dart
// lib/widgets/character_widget.dart
class CharacterWidget extends StatelessWidget {
  final Character character;
  
  const CharacterWidget({Key? key, required this.character}) : super(key: key);
  
  @override
  Widget build(BuildContext context) {
    // Get character position
    final position = character.position;
    
    // Get current animation frame
    final animation = character.animation;
    final frameImage = animation.currentFrameImage;
    
    return Positioned(
      left: position.x,
      top: position.y,
      child: Image.asset(
        frameImage,
        width: 64,
        height: 64,
      ),
    );
  }
}
```

### Minion Rendering

```dart
// lib/widgets/minion_widget.dart
class MinionWidget extends StatelessWidget {
  final Minion minion;
  
  const MinionWidget({Key? key, required this.minion}) : super(key: key);
  
  @override
  Widget build(BuildContext context) {
    // Get minion position
    final position = minion.position;
    
    // Get current animation frame
    final animation = minion.animation;
    final frameImage = animation.currentFrameImage;
    
    return Positioned(
      left: position.x,
      top: position.y,
      child: Image.asset(
        frameImage,
        width: 48, // Minions are smaller than main character
        height: 48,
      ),
    );
  }
}
```

## Testing

```dart
// test/idle_system_test.dart
void main() {
  testWidgets('Character goes idle after inactivity', (WidgetTester tester) async {
    // Setup test environment
    await tester.pumpWidget(TestApp());
    
    final activityTracker = ActivityTracker(
      onBecomeIdle: () {},
      onBecomeActive: () {},
    );
    
    // Simulate time passing (2 minutes)
    activityTracker._lastInteractionTime = 
        DateTime.now().subtract(Duration(minutes: 3));
    
    // Manually trigger idle check
    activityTracker._idleTimer?.tick(1);
    
    expect(activityTracker.isIdle, true);
  });
  
  testWidgets('Minion follows character to throne', (WidgetTester tester) async {
    // Setup test environment with character and minion
    final character = Character();
    final minion = Minion(id: '1', name: 'Test Minion');
    final locationManager = LocationManager();
    
    // Set up controllers
    final idleController = CharacterIdleController(
      character: character,
      locationManager: locationManager,
      menuStateTracker: MenuStateTracker(),
    );
    
    final minionController = MinionController(
      minion: minion,
      owner: character,
      locationManager: locationManager,
    );
    
    // Simulate character going idle and sitting on throne
    await idleController.initiateIdleSequence();
    
    // Update minion state
    minionController.updateState();
    
    // Expect minion to be in following state
    expect(minion.state, MinionState.following);
  });
  
  // Additional tests...
}
```

## Game Loop Integration

```dart
// lib/game/game_loop.dart
class GameLoop {
  Timer? _timer;
  int _lastFrameTime = 0;
  
  final List<Function(int)> _updateCallbacks = [];
  
  void start() {
    _lastFrameTime = DateTime.now().millisecondsSinceEpoch;
    
    _timer = Timer.periodic(Duration(milliseconds: 16), (timer) {
      final currentTime = DateTime.now().millisecondsSinceEpoch;
      final deltaTime = currentTime - _lastFrameTime;
      _lastFrameTime = currentTime;
      
      // Call all update callbacks
      for (final callback in _updateCallbacks) {
        callback(deltaTime);
      }
    });
  }
  
  void registerUpdateCallback(Function(int) callback) {
    _updateCallbacks.add(callback);
  }
  
  void stop() {
    _timer?.cancel();
  }
}
```

## Language Integration

```dart
// assets/language_packs/game_terminology.json
{
  "idle_throne_prompt": "Return to Throne",
  "minion_assigned": "Minion assigned to quest",
  "minion_completed": "Minion completed quest",
  "character_idle": "Resting on throne"
}

// assets/language_packs/professional_terminology.json
{
  "idle_throne_prompt": "Return to Desk",
  "minion_assigned": "Team member assigned to task",
  "minion_completed": "Team member completed task",
  "character_idle": "Taking a break"
}
``` 