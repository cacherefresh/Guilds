# Flutter Implementation Plan

## Language Pack & Terminology System

### 1. Create Language Provider
```dart
// lib/providers/language_provider.dart
class LanguageProvider extends ChangeNotifier {
  bool _useGameTerminology = true; // Default to game terminology
  Map<String, String> _currentTerminology = {};
  
  bool get useGameTerminology => _useGameTerminology;
  String getTerm(String key) => _currentTerminology[key] ?? key;
  
  Future<void> initialize() async {
    // Load default terms from local storage or API
    await _loadTerminology();
  }
  
  Future<void> toggleTerminology() async {
    _useGameTerminology = !_useGameTerminology;
    await _loadTerminology();
    notifyListeners();
  }
  
  Future<void> _loadTerminology() async {
    // In standalone mode, always use game terminology
    if (isStandaloneMode()) {
      _useGameTerminology = true;
    }
    
    _currentTerminology = _useGameTerminology 
        ? await _fetchGameTerminology()
        : await _fetchProfessionalTerminology();
  }
  
  // Implementation methods to fetch terminology sets
}
```

### 2. Update Main App
```dart
// lib/main.dart
void main() {
  // Determine if running in standalone mode
  final bool isStandalone = determineIfStandalone();
  
  runApp(
    MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (_) => LanguageProvider()),
        // Other providers
      ],
      child: MyApp(isStandalone: isStandalone),
    ),
  );
}
```

### 3. Create Term Widget
```dart
// lib/widgets/term_text.dart
class TermText extends StatelessWidget {
  final String termKey;
  final TextStyle? style;
  
  const TermText(this.termKey, {Key? key, this.style}) : super(key: key);
  
  @override
  Widget build(BuildContext context) {
    final term = Provider.of<LanguageProvider>(context).getTerm(termKey);
    return Text(term, style: style);
  }
}
```

### 4. Update Existing Screens
- Replace all hardcoded terminology with TermText widgets
- Add language toggle in settings (hidden in standalone mode)

## Character Magic System

### 1. Update Magic Model
```dart
// lib/models/magic.dart
enum MagicType {
  shadow,
  elemental,
  divine,
  nature,
  arcane
}

class Spell {
  final String id;
  final String name;
  final String description;
  final MagicType type;
  final bool requiresGrimoire;
  
  const Spell({
    required this.id,
    required this.name,
    required this.description,
    required this.type,
    this.requiresGrimoire = false,
  });
}
```

### 2. Create Spell Repository
```dart
// lib/repositories/spell_repository.dart
class SpellRepository {
  List<Spell> getAllSpells() {
    return [
      // Shadow magic spells
      Spell(
        id: 'create_minion',
        name: 'Create Minion',
        description: 'Summon a minion to assist you',
        type: MagicType.shadow,
        requiresGrimoire: true,
      ),
      Spell(
        id: 'shadow_clone',
        name: 'Shadow Clone',
        description: 'Create a temporary clone of yourself',
        type: MagicType.shadow,
        requiresGrimoire: true,
      ),
      // Other magic types
      Spell(
        id: 'fireball',
        name: 'Fireball',
        description: 'Launch a ball of fire',
        type: MagicType.elemental,
        requiresGrimoire: false,
      ),
      // More spells...
    ];
  }
  
  List<Spell> getSpellsByType(MagicType type) {
    return getAllSpells().where((spell) => spell.type == type).toList();
  }
  
  List<Spell> getAvailableSpells(Character character, bool atGrimoire) {
    return getAllSpells().where((spell) {
      if (spell.requiresGrimoire && !atGrimoire) return false;
      // Add other conditions (level requirements, etc.)
      return true;
    }).toList();
  }
}
```

### 3. Create Magic Screen
```dart
// lib/screens/magic_screen.dart
class MagicScreen extends StatelessWidget {
  final bool atGrimoire;
  
  const MagicScreen({Key? key, required this.atGrimoire}) : super(key: key);
  
  @override
  Widget build(BuildContext context) {
    final spellRepo = SpellRepository();
    final character = Provider.of<CharacterProvider>(context).currentCharacter;
    final availableSpells = spellRepo.getAvailableSpells(character, atGrimoire);
    
    return DefaultTabController(
      length: MagicType.values.length,
      child: Scaffold(
        appBar: AppBar(
          title: TermText('magic_spellbook'),
          bottom: TabBar(
            tabs: MagicType.values.map((type) => 
              Tab(text: type.name.capitalize())
            ).toList(),
          ),
        ),
        body: TabBarView(
          children: MagicType.values.map((type) {
            final spells = availableSpells
                .where((spell) => spell.type == type)
                .toList();
            
            return ListView.builder(
              itemCount: spells.length,
              itemBuilder: (context, index) {
                final spell = spells[index];
                return ListTile(
                  title: Text(spell.name),
                  subtitle: Text(spell.description),
                  trailing: spell.requiresGrimoire
                      ? Icon(Icons.book, color: atGrimoire ? Colors.green : Colors.red)
                      : null,
                  onTap: () {
                    if (spell.requiresGrimoire && !atGrimoire) {
                      showDialog(
                        context: context,
                        builder: (context) => AlertDialog(
                          title: TermText('grimoire_required'),
                          content: TermText('must_be_at_grimoire'),
                          actions: [
                            TextButton(
                              onPressed: () => Navigator.pop(context),
                              child: TermText('ok'),
                            ),
                          ],
                        ),
                      );
                      return;
                    }
                    
                    // Cast spell
                    _castSpell(context, spell);
                  },
                );
              },
            );
          }).toList(),
        ),
      ),
    );
  }
  
  void _castSpell(BuildContext context, Spell spell) {
    // Implementation for casting different spells
  }
}
```

## Character Idle & Animation System

### 1. Create Character State Manager
```dart
// lib/managers/character_state_manager.dart
enum CharacterState {
  active,
  idle,
  casting,
  interacting
}

class CharacterStateManager {
  CharacterState _state = CharacterState.active;
  DateTime _lastInteractionTime = DateTime.now();
  Timer? _idleTimer;
  final Duration idleThreshold = Duration(minutes: 2);
  bool _menuOpen = false;
  
  CharacterStateManager() {
    _startIdleTimer();
  }
  
  void recordInteraction() {
    _lastInteractionTime = DateTime.now();
    if (_state == CharacterState.idle) {
      _state = CharacterState.active;
    }
  }
  
  void setMenuOpen(bool isOpen) {
    _menuOpen = isOpen;
  }
  
  void _startIdleTimer() {
    _idleTimer = Timer.periodic(Duration(seconds: 30), (timer) {
      final timeSinceLastInteraction = DateTime.now().difference(_lastInteractionTime);
      if (timeSinceLastInteraction >= idleThreshold && _state != CharacterState.idle) {
        _setIdle();
      }
    });
  }
  
  void _setIdle() {
    _state = CharacterState.idle;
    if (!_menuOpen) {
      // Walk to throne and sit
      _walkToThrone();
    }
  }
  
  void _walkToThrone() {
    // Animation logic to walk character to throne
    // Then set sitting animation
  }
  
  void dispose() {
    _idleTimer?.cancel();
  }
}
```

### 2. Create Minion Manager
```dart
// lib/managers/minion_manager.dart
enum MinionState {
  idle,
  assigned,
  completed,
  following
}

class Minion {
  final String id;
  final String name;
  MinionState state = MinionState.idle;
  Quest? assignedQuest;
  Timer? animationTimer;
  
  Minion({required this.id, required this.name});
}

class MinionManager {
  List<Minion> minions = [];
  
  void assignQuest(Minion minion, Quest quest) {
    minion.state = MinionState.assigned;
    minion.assignedQuest = quest;
    
    // Walk to bottom left position
    _walkToWorkPosition(minion);
    
    // Set up periodic animation
    minion.animationTimer = Timer.periodic(Duration(seconds: 5), (timer) {
      _playRunningAnimation(minion);
    });
  }
  
  void completeQuest(Minion minion) {
    minion.state = MinionState.completed;
    minion.animationTimer?.cancel();
    
    // Walk to center bottom
    _walkToCompletionPosition(minion);
    
    // Play completion animation
    _playCompletionAnimation(minion);
    
    // Reset after animation
    Future.delayed(Duration(seconds: 3), () {
      minion.state = MinionState.idle;
      minion.assignedQuest = null;
    });
  }
  
  void updateIdleMinions(CharacterState ownerState, Position ownerPosition) {
    // If owner is idle and sitting on throne, idle minions should follow
    if (ownerState == CharacterState.idle) {
      for (var minion in minions) {
        if (minion.state == MinionState.idle) {
          minion.state = MinionState.following;
          _followOwner(minion, ownerPosition);
        }
      }
    }
  }
  
  // Implementation methods for animations and movement
  void _walkToWorkPosition(Minion minion) { /* Implementation */ }
  void _walkToCompletionPosition(Minion minion) { /* Implementation */ }
  void _playRunningAnimation(Minion minion) { /* Implementation */ }
  void _playCompletionAnimation(Minion minion) { /* Implementation */ }
  void _followOwner(Minion minion, Position ownerPosition) { /* Implementation */ }
}
```

### 3. Update Game Loop
```dart
// lib/game/game_loop.dart
class GameLoop {
  final CharacterStateManager characterManager;
  final MinionManager minionManager;
  Timer? _gameLoopTimer;
  
  GameLoop({
    required this.characterManager,
    required this.minionManager,
  });
  
  void start() {
    _gameLoopTimer = Timer.periodic(Duration(milliseconds: 16), (timer) {
      // Update character state
      
      // Update minion states
      minionManager.updateIdleMinions(
        characterManager._state,
        _getCurrentCharacterPosition(),
      );
      
      // Other game updates (animations, etc.)
    });
  }
  
  Position _getCurrentCharacterPosition() {
    // Implementation to get current character position
    return Position(x: 0, y: 0);
  }
  
  void dispose() {
    _gameLoopTimer?.cancel();
  }
}
```

## Integration with Input Handlers

### 1. Update Input Handlers
```dart
// lib/input/input_handler.dart
class InputHandler {
  final CharacterStateManager characterManager;
  
  InputHandler({required this.characterManager});
  
  void handleMouseMove() {
    characterManager.recordInteraction();
  }
  
  void handleKeyPress() {
    characterManager.recordInteraction();
  }
  
  void handleMenuOpen(bool isOpen) {
    characterManager.setMenuOpen(isOpen);
  }
}
```

### 2. Connect to UI
```dart
// lib/widgets/game_screen.dart
class GameScreen extends StatefulWidget {
  @override
  _GameScreenState createState() => _GameScreenState();
}

class _GameScreenState extends State<GameScreen> {
  late CharacterStateManager _characterManager;
  late MinionManager _minionManager;
  late GameLoop _gameLoop;
  late InputHandler _inputHandler;
  
  @override
  void initState() {
    super.initState();
    _characterManager = CharacterStateManager();
    _minionManager = MinionManager();
    _gameLoop = GameLoop(
      characterManager: _characterManager,
      minionManager: _minionManager,
    );
    _inputHandler = InputHandler(characterManager: _characterManager);
    
    _gameLoop.start();
  }
  
  @override
  Widget build(BuildContext context) {
    return Listener(
      onPointerMove: (_) => _inputHandler.handleMouseMove(),
      onKeyEvent: (key) => _inputHandler.handleKeyPress(),
      child: Stack(
        children: [
          // Game world
          GameWorld(),
          
          // UI elements
          Positioned(
            top: 10,
            right: 10,
            child: IconButton(
              icon: Icon(Icons.menu),
              onPressed: () {
                _inputHandler.handleMenuOpen(true);
                _showMenu();
              },
            ),
          ),
        ],
      ),
    );
  }
  
  void _showMenu() {
    showDialog(
      context: context,
      builder: (context) {
        return GameMenu(
          onClose: () => _inputHandler.handleMenuOpen(false),
        );
      },
    );
  }
  
  @override
  void dispose() {
    _characterManager.dispose();
    _gameLoop.dispose();
    super.dispose();
  }
}
```

## Implementation Timeline

### Week 1: Foundation
- Create Language Provider and Term Widget
- Update main app to detect standalone mode
- Create basic character and minion state managers

### Week 2: Magic System
- Implement magic model and repository
- Create grimoire location detection
- Build magic screen with category tabs
- Implement spell casting functionality

### Week 3: Animation & Idle System
- Implement character idle detection
- Create throne-sitting animation sequence
- Build minion follow behavior
- Implement quest assignment animations

### Week 4: Integration & Polish
- Connect input handlers to state managers
- Ensure proper menu state preservation during idle
- Update all screens to use TermText widget
- Test idle and animation sequences
- Finalize standalone mode behavior

## Testing Scenarios

1. **Language System**
   - App starts with game terminology in standalone mode
   - All UI elements show correct terminology
   - Terms update when toggling between modes (if not standalone)

2. **Magic System**
   - Shadow magic spells only available at grimoire
   - Other spells available based on location
   - Proper error shown when attempting restricted spells

3. **Idle Behavior**
   - Character goes idle after 2 minutes of inactivity
   - Character walks to throne when idle (if no menus open)
   - Idle minions follow character to throne
   - Character returns to active state on input

4. **Minion Behavior**
   - Minions walk to correct position when assigned quests
   - Running animation plays every 5 seconds
   - Completion animation plays at correct location
   - Minions return to idle state after completion 