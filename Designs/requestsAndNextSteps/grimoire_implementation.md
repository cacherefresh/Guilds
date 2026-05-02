# Grimoire & Shadow Magic Implementation Guide

## Overview
The grimoire is a special location in the guild where powerful magic can be performed. Only certain spells (primarily shadow magic) can be cast when the character is at the grimoire location. This document details the implementation of this system within the Flutter application.

## Grimoire Location

### Location Detection
```dart
// lib/world/locations.dart
class WorldLocation {
  final String id;
  final String name;
  final Position position;
  final double interactionRadius;
  final LocationType type;
  
  const WorldLocation({
    required this.id,
    required this.name,
    required this.position,
    this.interactionRadius = 1.0,
    required this.type,
  });
  
  bool isCharacterInRange(Position characterPosition) {
    final distance = position.distanceTo(characterPosition);
    return distance <= interactionRadius;
  }
}

enum LocationType {
  questBoard,
  grimoire,
  throne,
  guildHall,
  trainingArea,
  entryway
}

class LocationManager {
  final List<WorldLocation> _locations = [
    WorldLocation(
      id: 'main_grimoire',
      name: 'Arcane Grimoire',
      position: Position(x: 15.0, y: 10.0),
      interactionRadius: 2.0,
      type: LocationType.grimoire,
    ),
    WorldLocation(
      id: 'guild_throne',
      name: 'Guild Master Throne',
      position: Position(x: 20.0, y: 5.0),
      interactionRadius: 2.0,
      type: LocationType.throne,
    ),
    // Additional locations...
  ];
  
  WorldLocation? getLocationById(String id) {
    return _locations.firstWhere((loc) => loc.id == id, orElse: () => null);
  }
  
  List<WorldLocation> getLocationsOfType(LocationType type) {
    return _locations.where((loc) => loc.type == type).toList();
  }
  
  WorldLocation? getNearestLocationOfType(Position characterPosition, LocationType type) {
    final locations = getLocationsOfType(type);
    if (locations.isEmpty) return null;
    
    locations.sort((a, b) {
      final distA = a.position.distanceTo(characterPosition);
      final distB = b.position.distanceTo(characterPosition);
      return distA.compareTo(distB);
    });
    
    return locations.first;
  }
  
  bool isCharacterAtGrimoire(Position characterPosition) {
    final grimoires = getLocationsOfType(LocationType.grimoire);
    for (final grimoire in grimoires) {
      if (grimoire.isCharacterInRange(characterPosition)) {
        return true;
      }
    }
    return false;
  }
}
```

## Shadow Magic Implementation

### Shadow Magic Spells
```dart
// lib/magic/shadow_magic.dart
class ShadowMagic {
  final CharacterManager characterManager;
  final MinionManager minionManager;
  
  ShadowMagic({
    required this.characterManager,
    required this.minionManager,
  });
  
  // Creates a new minion that follows and assists the character
  Future<Minion?> createMinion(String name) async {
    final character = characterManager.currentCharacter;
    
    // Check if character has sufficient mana/energy
    if (character.mana < 50) {
      return null; // Not enough mana
    }
    
    // Create minion
    final minion = Minion(
      id: Uuid().v4(),
      name: name,
      level: max(1, character.level - 5), // Minion is weaker than master
      skills: character.skills.map((s) => 
        Skill(id: s.id, name: s.name, level: max(1, s.level - 3))
      ).toList(),
    );
    
    // Consume mana
    characterManager.consumeMana(50);
    
    // Add minion to character's minions
    minionManager.addMinion(minion);
    
    // Play creation animation
    await _playMinionCreationAnimation();
    
    return minion;
  }
  
  // Creates a temporary clone of the character
  Future<void> createShadowClone() async {
    final character = characterManager.currentCharacter;
    
    // Check if character has sufficient mana/energy
    if (character.mana < 80) {
      return; // Not enough mana
    }
    
    // Create shadow clone (visual effect only)
    final shadowClone = ShadowClone(
      position: character.position.copyWith(
        x: character.position.x + 1.0
      ),
      appearance: character.appearance,
    );
    
    // Consume mana
    characterManager.consumeMana(80);
    
    // Play clone animation
    await _playShadowCloneAnimation(shadowClone);
    
    // Shadow clone lasts for 30 seconds
    Future.delayed(Duration(seconds: 30), () {
      _dismissShadowClone(shadowClone);
    });
  }
  
  // Animation for creating minions
  Future<void> _playMinionCreationAnimation() async {
    // Implementation for minion creation animation
    // Dark smoke effect, glowing runes, etc.
  }
  
  // Animation for shadow clone
  Future<void> _playShadowCloneAnimation(ShadowClone clone) async {
    // Implementation for shadow clone animation
    // Shadow portal, mist effect, etc.
  }
  
  // Dismiss the shadow clone with fade effect
  Future<void> _dismissShadowClone(ShadowClone clone) async {
    // Implementation for shadow clone dismissal
  }
}
```

## Grimoire UI Component

### Grimoire Interaction Widget
```dart
// lib/widgets/grimoire_widget.dart
class GrimoireWidget extends StatefulWidget {
  @override
  _GrimoireWidgetState createState() => _GrimoireWidgetState();
}

class _GrimoireWidgetState extends State<GrimoireWidget> {
  bool _isOpen = false;
  
  @override
  Widget build(BuildContext context) {
    final characterPosition = Provider.of<CharacterProvider>(context).currentCharacter.position;
    final locationManager = Provider.of<LocationManager>(context);
    final isAtGrimoire = locationManager.isCharacterAtGrimoire(characterPosition);
    
    // If not at grimoire, show nothing or inactive grimoire
    if (!isAtGrimoire && !_isOpen) {
      return SizedBox.shrink();
    }
    
    // If grimoire is open, show full UI
    if (_isOpen) {
      return _buildGrimoireUI(context);
    }
    
    // Show interaction prompt
    return GestureDetector(
      onTap: () {
        setState(() {
          _isOpen = true;
        });
        // Notify character is interacting with grimoire
        Provider.of<CharacterProvider>(context, listen: false)
          .setCharacterAction(CharacterAction.usingGrimoire);
      },
      child: Container(
        padding: EdgeInsets.all(8),
        decoration: BoxDecoration(
          color: Colors.black.withOpacity(0.7),
          borderRadius: BorderRadius.circular(8),
        ),
        child: TermText(
          'grimoire_interact',
          style: TextStyle(color: Colors.purple[200]),
        ),
      ),
    );
  }
  
  Widget _buildGrimoireUI(BuildContext context) {
    return Container(
      width: double.infinity,
      height: double.infinity,
      decoration: BoxDecoration(
        color: Colors.black.withOpacity(0.9),
        border: Border.all(color: Colors.purple[800]!, width: 2),
      ),
      child: Column(
        children: [
          _buildGrimoireHeader(),
          Expanded(
            child: MagicScreen(atGrimoire: true),
          ),
        ],
      ),
    );
  }
  
  Widget _buildGrimoireHeader() {
    return Container(
      padding: EdgeInsets.all(16),
      decoration: BoxDecoration(
        border: Border(bottom: BorderSide(color: Colors.purple[800]!, width: 2)),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          TermText(
            'grimoire_title',
            style: TextStyle(
              color: Colors.purple[200],
              fontSize: 24,
              fontWeight: FontWeight.bold,
            ),
          ),
          IconButton(
            icon: Icon(Icons.close, color: Colors.purple[200]),
            onPressed: () {
              setState(() {
                _isOpen = false;
              });
              // Reset character action
              Provider.of<CharacterProvider>(context, listen: false)
                .setCharacterAction(CharacterAction.idle);
            },
          ),
        ],
      ),
    );
  }
}
```

## Magic System Integration

### Magic System Provider
```dart
// lib/providers/magic_provider.dart
class MagicProvider extends ChangeNotifier {
  final LocationManager locationManager;
  final CharacterProvider characterProvider;
  final ShadowMagic shadowMagic;
  
  MagicProvider({
    required this.locationManager,
    required this.characterProvider,
    required this.shadowMagic,
  });
  
  bool get canCastShadowMagic {
    final characterPosition = characterProvider.currentCharacter.position;
    return locationManager.isCharacterAtGrimoire(characterPosition);
  }
  
  Future<bool> castSpell(Spell spell) async {
    // Check if spell can be cast
    if (spell.requiresGrimoire && !canCastShadowMagic) {
      return false;
    }
    
    // Handle different spell types
    switch (spell.id) {
      case 'create_minion':
        final minion = await shadowMagic.createMinion('Shadow Servant');
        return minion != null;
        
      case 'shadow_clone':
        await shadowMagic.createShadowClone();
        return true;
        
      // Handle other spell types...
      
      default:
        return false;
    }
  }
}
```

## Integration with Game World

### Game World Integration
```dart
// lib/world/game_world.dart
class GameWorld extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        // Base world layer
        WorldMapLayer(),
        
        // Characters layer
        CharactersLayer(),
        
        // Interactive objects layer
        InteractiveObjectsLayer(),
        
        // Grimoire specific UI elements
        _buildGrimoireElements(context),
        
        // UI overlays
        UIOverlayLayer(),
      ],
    );
  }
  
  Widget _buildGrimoireElements(BuildContext context) {
    final characterPosition = Provider.of<CharacterProvider>(context).currentCharacter.position;
    final locationManager = Provider.of<LocationManager>(context);
    
    // Get all grimoire locations
    final grimoires = locationManager.getLocationsOfType(LocationType.grimoire);
    
    return Stack(
      children: grimoires.map((grimoire) {
        // Calculate position on screen
        final screenPosition = worldToScreenPosition(grimoire.position);
        
        // Determine if character is close enough to interact
        final isInRange = grimoire.isCharacterInRange(characterPosition);
        
        // Build grimoire visual
        return Positioned(
          left: screenPosition.x - 25, // Adjust based on sprite size
          top: screenPosition.y - 25,
          child: Column(
            children: [
              // Grimoire sprite/image
              Image.asset(
                'assets/images/grimoire.png',
                width: 50,
                height: 50,
              ),
              
              // Interaction indicator (only if in range)
              if (isInRange)
                Container(
                  padding: EdgeInsets.all(4),
                  decoration: BoxDecoration(
                    color: Colors.black.withOpacity(0.7),
                    borderRadius: BorderRadius.circular(4),
                  ),
                  child: TermText(
                    'press_to_use',
                    style: TextStyle(color: Colors.white, fontSize: 12),
                  ),
                ),
            ],
          ),
        );
      }).toList(),
    );
  }
}
```

## Testing the Grimoire System

### Test Cases
1. Character can only cast shadow magic when at grimoire
2. Visual feedback shows when character is in range of grimoire
3. Grimoire UI opens when interacted with
4. Shadow magic spells appear in the correct tab
5. Minion creation shows appropriate animation
6. Created minions follow the character until assigned tasks
7. Shadow clone appears and mimics character movements
8. Mana/energy consumption works correctly for all spells

### Testing Utility
```dart
// test/grimoire_test.dart
void main() {
  testWidgets('Shadow magic requires grimoire proximity', (WidgetTester tester) async {
    // Setup test environment
    await tester.pumpWidget(TestApp());
    
    // Position character away from grimoire
    final characterProvider = tester.widget<TestApp>(find.byType(TestApp)).characterProvider;
    characterProvider.setCharacterPosition(Position(x: 50, y: 50)); // Far from grimoire
    
    // Try to cast shadow magic
    final magicProvider = tester.widget<TestApp>(find.byType(TestApp)).magicProvider;
    final shadowSpell = SpellRepository().getSpellsByType(MagicType.shadow).first;
    
    // Assert spell cannot be cast
    expect(await magicProvider.castSpell(shadowSpell), false);
    
    // Move character to grimoire
    final locationManager = tester.widget<TestApp>(find.byType(TestApp)).locationManager;
    final grimoireLocation = locationManager.getLocationsOfType(LocationType.grimoire).first;
    characterProvider.setCharacterPosition(grimoireLocation.position);
    
    // Try casting again
    expect(await magicProvider.castSpell(shadowSpell), true);
  });
  
  // Additional tests...
}
```

## Integration with Character Animation System

```dart
// lib/animation/character_animations.dart
class CharacterAnimationController {
  final Map<String, Animation<double>> _animations = {};
  
  // Animation for using the grimoire
  Animation<double> get usingGrimoireAnimation => _animations['using_grimoire']!;
  
  void initializeAnimations(TickerProvider vsync) {
    // Create animations for different states
    final animationController = AnimationController(
      vsync: vsync,
      duration: Duration(milliseconds: 800),
    );
    
    _animations['using_grimoire'] = Tween<double>(
      begin: 0.0,
      end: 1.0,
    ).animate(
      CurvedAnimation(
        parent: animationController,
        curve: Curves.easeInOut,
      ),
    );
    
    // More animations...
  }
  
  // Play the animation for using grimoire
  void playGrimoireAnimation() {
    (_animations['using_grimoire'] as Animation<double>).controller.reset();
    (_animations['using_grimoire'] as Animation<double>).controller.forward();
  }
}
```

## Language & Terminology Integration

```dart
// assets/language_packs/game_terminology.json
{
  "grimoire_interact": "Open Tome of Shadows",
  "grimoire_title": "Ancient Grimoire",
  "grimoire_required": "Arcane Focus Required",
  "must_be_at_grimoire": "You must be at your grimoire to cast this spell",
  "magic_spellbook": "Spellbook",
  "shadow_magic": "Shadow Magic",
  "create_minion": "Summon Shadow Servant",
  "shadow_clone": "Create Shadow Clone",
  "press_to_use": "Press to Channel"
}

// assets/language_packs/professional_terminology.json
{
  "grimoire_interact": "Access Task Manager",
  "grimoire_title": "Task Management Console",
  "grimoire_required": "Console Access Required",
  "must_be_at_grimoire": "You must be at the management console to perform this action",
  "magic_spellbook": "Action Console",
  "shadow_magic": "Resource Management",
  "create_minion": "Create Team Member",
  "shadow_clone": "Create Duplicate Instance",
  "press_to_use": "Click to Access"
}
``` 