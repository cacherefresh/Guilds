import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import '../models/character_model.dart';
import '../widgets/quest_board.dart';
import 'dart:math';
import 'dart:async';
import 'dart:convert';

class SimpleCharacterView extends StatefulWidget {
  const SimpleCharacterView({Key? key}) : super(key: key);

  @override
  State<SimpleCharacterView> createState() => _SimpleCharacterViewState();
}

class _SimpleCharacterViewState extends State<SimpleCharacterView> {
  // Room items with their positions
  final Map<String, Map<String, dynamic>> _roomItems = {
    'dimension': {
      'position': const Offset(500, 80),
      'name': 'Dimension Portal',
      'icon': Icons.blur_circular,
    },
    'todoBoard': {
      'position': const Offset(200, 150),
      'name': 'Quest Board',
      'icon': Icons.assignment,
    },
    'laptop': {
      'position': const Offset(500, 300),
      'name': 'Laptop',
      'icon': Icons.laptop,
    },
    'turntables': {
      'position': const Offset(800, 500),
      'name': 'Turntables',
      'icon': Icons.music_note,
    },
    'webcam': {
      'position': const Offset(300, 200),
      'name': 'Webcam',
      'icon': Icons.videocam,
    },
    'grimoire': {
      'position': const Offset(650, 200),
      'name': 'Grimoire',
      'icon': Icons.book,
    },
    'mainThrone': {
      'position': const Offset(500, 600),
      'name': 'Your Throne',
      'icon': Icons.chair,
    },
    'teammateThrone': {
      'position': const Offset(700, 600),
      'name': 'Teammate Throne',
      'icon': Icons.event_seat,
    },
  };

  // Main character position
  Offset _characterPosition = const Offset(500, 400);
  
  // Teammate character position
  Offset _teammatePosition = const Offset(600, 400);
  String _teammateAction = 'idle';
  bool _controllingTeammate = false;
  
  // Summoned characters
  List<Map<String, dynamic>> _summonedCharacters = [];
  int _activeSummonIndex = -1; // -1 means no summon is active
  
  // Magic abilities with their icons and casting state
  final List<Map<String, dynamic>> _quickCastMagic = [
    {
      'name': 'Shadow Clone',
      'icon': Icons.person_outline,
      'color': Colors.purple,
      'isCasting': false,
    },
    {
      'name': 'Minion',
      'icon': Icons.pets,
      'color': Colors.brown,
      'isCasting': false,
    },
    {
      'name': 'Play Music',
      'icon': Icons.music_note,
      'color': Colors.cyan,
      'isCasting': false,
    },
    {
      'name': 'Healing Light',
      'icon': Icons.healing,
      'color': Colors.green,
      'isCasting': false,
    },
    {
      'name': 'Arcane Blast',
      'icon': Icons.flash_on,
      'color': Colors.blue,
      'isCasting': false,
    },
  ];
  
  // Timers for automatic movements
  Timer? _summonMovementTimer;
  Timer? _idleCheckTimer;
  
  // Focus node for keyboard input
  final FocusNode _focusNode = FocusNode();
  
  // Game state JSON store
  Map<String, dynamic> _gameState = {};
  
  @override
  void initState() {
    super.initState();
    
    // Initialize the game state store
    _initializeGameState();
    
    // Start timers for automatic movements
    _summonMovementTimer = Timer.periodic(const Duration(seconds: 5), (_) {
      _moveSummonsRandomly();
    });
    
    _idleCheckTimer = Timer.periodic(const Duration(seconds: 8), (_) {
      _checkAndHandleIdleCharacters();
    });
  }
  
  @override
  void dispose() {
    _summonMovementTimer?.cancel();
    _idleCheckTimer?.cancel();
    _focusNode.dispose();
    super.dispose();
  }
  
  // Initialize game state with default values
  void _initializeGameState() {
    _gameState = {
      'character': {
        'name': 'Guild Master',
        'position': {
          'x': _characterPosition.dx,
          'y': _characterPosition.dy,
        },
        'action': 'idle',
        'afterimage': 'off',
      },
      'teammate': {
        'position': {
          'x': _teammatePosition.dx,
          'y': _teammatePosition.dy,
        },
        'action': _teammateAction,
      },
      'summons': [],
      'adventures': [
        {
          'id': '1',
          'name': 'Main Campaign',
          'description': 'The primary adventure of your guild',
          'status': 'Active',
          'created': DateTime.now().millisecondsSinceEpoch,
        },
        {
          'id': '2',
          'name': 'Side Missions',
          'description': 'Optional tasks and smaller objectives',
          'status': 'Active',
          'created': DateTime.now().millisecondsSinceEpoch,
        },
      ],
      'quests': [
        {
          'id': '1',
          'title': 'Complete tutorial',
          'description': 'Learn the basics of the guild system',
          'status': 'Active',
          'category': 'Tutorial',
          'adventureId': '1',
        },
        {
          'id': '2',
          'title': 'Recruit a teammate',
          'description': 'Find someone to join your guild',
          'status': 'Complete',
          'category': 'Guild',
          'adventureId': '1',
        },
        {
          'id': '3',
          'title': 'Craft a potion',
          'description': 'Use the alchemy table to make your first item',
          'status': 'Pending',
          'category': 'Crafting',
          'adventureId': '2',
        },
      ],
    };
  }

  // Reset game state to a fresh start (keep character name but reset quests and summons)
  void _initNew() {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    final characterName = characterModel.name;
    
    setState(() {
      // Clear summons
      _summonedCharacters = [];
      _activeSummonIndex = -1;
      
      // Reset game state
      _initializeGameState();
      
      // Keep character name
      _gameState['character']['name'] = characterName;
      
      // Update character model
      characterModel.updateAction('idle');
      characterModel.setProperty('afterimage', 'off');
      
      // Reset teammate
      _teammateAction = 'idle';
    });
    
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
        content: Text('Game state reset to default. All quests and summons cleared.'),
        duration: Duration(seconds: 3),
      ),
    );
  }

  // Save current game state to JSON
  void _saveGameState() {
    // Update game state with current values
    _updateGameStateFromCurrent();
    
    // Convert game state to JSON string
    final jsonString = _getGameStateJson();
    
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Save Game State'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Icon(Icons.save, size: 48, color: Colors.blue),
            const SizedBox(height: 16),
            const Text(
              'Your game state has been saved to JSON.',
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 16),
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: Colors.black87,
                borderRadius: BorderRadius.circular(8),
              ),
              height: 150,
              width: double.infinity,
              child: SingleChildScrollView(
                child: SelectableText(
                  jsonString,
                  style: const TextStyle(
                    color: Colors.green,
                    fontFamily: 'monospace',
                    fontSize: 12,
                  ),
                ),
              ),
            ),
            const SizedBox(height: 16),
            const Text(
              'Copy this JSON to load it later or save to a file.',
              textAlign: TextAlign.center,
              style: TextStyle(fontSize: 12, fontStyle: FontStyle.italic),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () {
              Navigator.of(context).pop();
              _focusNode.requestFocus();
            },
            child: const Text('Close'),
          ),
        ],
      ),
    );
  }

  // Load game state from JSON
  void _loadGameState() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Load Game State'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Icon(Icons.upload_file, size: 48, color: Colors.orange),
            const SizedBox(height: 16),
            const Text(
              'Paste your saved game state JSON below:',
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 16),
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: Colors.black87,
                borderRadius: BorderRadius.circular(8),
                border: Border.all(color: Colors.grey),
              ),
              height: 150,
              width: double.infinity,
              child: TextField(
                maxLines: null,
                decoration: const InputDecoration(
                  border: InputBorder.none,
                  hintText: 'Paste your JSON here...',
                  hintStyle: TextStyle(color: Colors.grey),
                ),
                style: const TextStyle(
                  color: Colors.green,
                  fontFamily: 'monospace',
                  fontSize: 12,
                ),
                onChanged: (value) {
                  // Store the input JSON temporarily
                  _loadJsonString = value;
                },
              ),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () {
              Navigator.of(context).pop();
              _focusNode.requestFocus();
            },
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              _applyLoadedGameState(_loadJsonString);
              Navigator.of(context).pop();
              _focusNode.requestFocus();
            },
            child: const Text('Load'),
          ),
        ],
      ),
    );
  }
  
  // Temporary storage for loaded JSON
  String _loadJsonString = '';

  // Update game state object with current values
  void _updateGameStateFromCurrent() {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    _gameState = {
      'character': {
        'name': characterModel.name,
        'position': {
          'x': _characterPosition.dx,
          'y': _characterPosition.dy,
        },
        'action': characterModel.currentAction,
        'afterimage': characterModel.getProperty('afterimage'),
      },
      'teammate': {
        'position': {
          'x': _teammatePosition.dx,
          'y': _teammatePosition.dy,
        },
        'action': _teammateAction,
      },
      'summons': _summonedCharacters.map((summon) => {
        'type': summon['type'],
        'name': summon['name'],
        'position': {
          'x': (summon['position'] as Offset).dx,
          'y': (summon['position'] as Offset).dy,
        },
        'action': summon['action'],
        'height': summon['height'],
        'color': {
          'value': (summon['color'] as Color).value,
        },
        'quests': summon['quests'] ?? [],
      }).toList(),
      'adventures': _gameState['adventures'] ?? [],
      'quests': _gameState['quests'] ?? [],
      'activeSummonIndex': _activeSummonIndex,
      'controllingTeammate': _controllingTeammate,
    };
  }

  // Get game state as JSON string
  String _getGameStateJson() {
    return const JsonEncoder.withIndent('  ').convert(_gameState);
  }

  // Apply loaded game state from JSON string
  void _applyLoadedGameState(String jsonString) {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    try {
      // Parse JSON string
      final Map<String, dynamic> loadedState = jsonDecode(jsonString);
      
      setState(() {
        // Store the parsed game state
        _gameState = loadedState;
        
        // Apply character state
        final characterState = loadedState['character'];
        if (characterState != null) {
          characterModel.name = characterState['name'] ?? 'Guild Master';
          
          if (characterState['position'] != null) {
            _characterPosition = Offset(
              characterState['position']['x'] ?? 500.0,
              characterState['position']['y'] ?? 400.0,
            );
            
            characterModel.updatePosition(
              _characterPosition.dx,
              0,
              _characterPosition.dy,
            );
          }
          
          characterModel.updateAction(characterState['action'] ?? 'idle');
          characterModel.setProperty('afterimage', characterState['afterimage'] ?? 'off');
        }
        
        // Apply teammate state
        final teammateState = loadedState['teammate'];
        if (teammateState != null) {
          if (teammateState['position'] != null) {
            _teammatePosition = Offset(
              teammateState['position']['x'] ?? 600.0,
              teammateState['position']['y'] ?? 400.0,
            );
          }
          
          _teammateAction = teammateState['action'] ?? 'idle';
        }
        
        // Apply summons state
        _summonedCharacters = [];
        final summonsList = loadedState['summons'] as List<dynamic>? ?? [];
        for (final summonData in summonsList) {
          final summonPosition = summonData['position'] != null 
              ? Offset(
                  summonData['position']['x'] ?? 500.0,
                  summonData['position']['y'] ?? 400.0,
                )
              : const Offset(500, 400);
              
          final summonColorValue = summonData['color'] != null 
              ? summonData['color']['value'] ?? 0xFF6A0DAD
              : 0xFF6A0DAD;
              
          _summonedCharacters.add({
            'type': summonData['type'] ?? 'Minion',
            'name': summonData['name'] ?? 'Unknown Summon',
            'position': summonPosition,
            'action': summonData['action'] ?? 'idle',
            'height': summonData['height'] ?? 70.0,
            'color': Color(summonColorValue),
            'quests': summonData['quests'] ?? [],
          });
        }
        
        // Restore control state
        _activeSummonIndex = loadedState['activeSummonIndex'] ?? -1;
        _controllingTeammate = loadedState['controllingTeammate'] ?? false;
      });
      
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Game state loaded successfully!'),
          duration: Duration(seconds: 3),
        ),
      );
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Error loading game state: $e'),
          backgroundColor: Colors.red,
          duration: const Duration(seconds: 3),
        ),
      );
    }
  }

  // Show dimension options dialog
  void _showDimensionOptions() {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    // Set character action
    characterModel.updateAction('interacting with dimension');
    
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Dimension Portal'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Icon(Icons.blur_circular, size: 50, color: Colors.purpleAccent),
            const SizedBox(height: 16),
            const Text(
              'The Dimension Portal allows you to manipulate the fabric of this reality.',
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 20),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              children: [
                _buildDimensionActionButton(
                  icon: Icons.save,
                  label: 'Save',
                  color: Colors.blue,
                  onTap: () {
                    Navigator.of(context).pop();
                    _saveGameState();
                  },
                ),
                _buildDimensionActionButton(
                  icon: Icons.upload_file,
                  label: 'Load',
                  color: Colors.orange,
                  onTap: () {
                    Navigator.of(context).pop();
                    _loadGameState();
                  },
                ),
                _buildDimensionActionButton(
                  icon: Icons.refresh,
                  label: 'Init New',
                  color: Colors.red,
                  onTap: () {
                    Navigator.of(context).pop();
                    _showInitNewConfirmation();
                  },
                ),
              ],
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () {
              characterModel.updateAction('idle');
              Navigator.of(context).pop();
              _focusNode.requestFocus();
            },
            child: const Text('Close'),
          ),
        ],
      ),
    );
  }

  // Build dimension action button
  Widget _buildDimensionActionButton({
    required IconData icon,
    required String label,
    required Color color,
    required VoidCallback onTap,
  }) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(8),
      child: Container(
        padding: const EdgeInsets.all(12),
        decoration: BoxDecoration(
          color: color.withOpacity(0.1),
          borderRadius: BorderRadius.circular(8),
          border: Border.all(color: color),
        ),
        child: Column(
          children: [
            Icon(icon, color: color, size: 30),
            const SizedBox(height: 8),
            Text(
              label,
              style: TextStyle(color: color),
            ),
          ],
        ),
      ),
    );
  }

  // Show confirmation dialog for init_new
  void _showInitNewConfirmation() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Reset Game State?'),
        content: const Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.warning, size: 48, color: Colors.orangeAccent),
            SizedBox(height: 16),
            Text(
              'This will reset all quests and remove all summoned characters. '
              'Your character name will be preserved.\n\n'
              'This action cannot be undone!',
              textAlign: TextAlign.center,
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () {
              Navigator.of(context).pop();
              _focusNode.requestFocus();
            },
            child: const Text('Cancel'),
          ),
          TextButton(
            style: TextButton.styleFrom(
              foregroundColor: Colors.red,
            ),
            onPressed: () {
              Navigator.of(context).pop();
              _initNew();
              _focusNode.requestFocus();
            },
            child: const Text('Reset'),
          ),
        ],
      ),
    );
  }

  // Move summons randomly around their summoner
  void _moveSummonsRandomly() {
    if (_summonedCharacters.isEmpty) return;
    
    setState(() {
      for (int i = 0; i < _summonedCharacters.length; i++) {
        // Skip if this summon is being actively controlled
        if (_activeSummonIndex == i) continue;
        
        final summon = _summonedCharacters[i];
        
        // Create random movement around the main character
        final double randomX = (_characterPosition.dx + (100 * (Random().nextDouble() * 2 - 1))).clamp(100, 900);
        final double randomY = (_characterPosition.dy + (100 * (Random().nextDouble() * 2 - 1))).clamp(100, 600);
        
        // Update summon position and action
        summon['position'] = Offset(randomX, randomY);
        summon['action'] = 'walking';
        
        // After 2 seconds, reset action to idle
        Future.delayed(const Duration(seconds: 2), () {
          if (mounted) {
            setState(() {
              if (i < _summonedCharacters.length) {
                _summonedCharacters[i]['action'] = 'idle';
              }
            });
          }
        });
      }
    });
  }
  
  // Check if characters are idle and move them to thrones
  void _checkAndHandleIdleCharacters() {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    // Check main character
    if (characterModel.currentAction == 'idle' && 
        !_controllingTeammate && 
        _activeSummonIndex < 0 &&
        (_characterPosition - _roomItems['mainThrone']!['position']).distance > 50) {
      
      setState(() {
        _moveCharacterToThrone(false);
      });
    }
    
    // Check teammate
    if (_teammateAction == 'idle' && 
        (_teammatePosition - _roomItems['teammateThrone']!['position']).distance > 50) {
      
      setState(() {
        _moveTeammateToThrone();
      });
    }
  }
  
  // Move main character to throne
  void _moveCharacterToThrone(bool immediate) {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    final thronePosition = _roomItems['mainThrone']!['position'] as Offset;
    
    if (immediate) {
      _characterPosition = thronePosition;
      characterModel.updateAction('sitting on throne');
      characterModel.updatePosition(
        _characterPosition.dx,
        0,
        _characterPosition.dy,
      );
    } else {
      characterModel.updateAction('walking to throne');
      
      // Simulate walking animation by moving in steps
      const steps = 10;
      final dx = (thronePosition.dx - _characterPosition.dx) / steps;
      final dy = (thronePosition.dy - _characterPosition.dy) / steps;
      
      for (int i = 1; i <= steps; i++) {
        Future.delayed(Duration(milliseconds: i * 300), () {
          if (mounted) {
            setState(() {
              _characterPosition = Offset(
                _characterPosition.dx + dx,
                _characterPosition.dy + dy,
              );
              
              characterModel.updatePosition(
                _characterPosition.dx,
                0,
                _characterPosition.dy,
              );
              
              // When reached throne
              if (i == steps) {
                characterModel.updateAction('sitting on throne');
              }
            });
          }
        });
      }
    }
  }
  
  // Move teammate to throne
  void _moveTeammateToThrone() {
    final thronePosition = _roomItems['teammateThrone']!['position'] as Offset;
    
    setState(() {
      _teammateAction = 'walking to throne';
    });
    
    // Simulate walking animation by moving in steps
    const steps = 10;
    final dx = (thronePosition.dx - _teammatePosition.dx) / steps;
    final dy = (thronePosition.dy - _teammatePosition.dy) / steps;
    
    for (int i = 1; i <= steps; i++) {
      Future.delayed(Duration(milliseconds: i * 300), () {
        if (mounted) {
          setState(() {
            _teammatePosition = Offset(
              _teammatePosition.dx + dx,
              _teammatePosition.dy + dy,
            );
            
            // When reached throne
            if (i == steps) {
              _teammateAction = 'sitting on throne';
            }
          });
        }
      });
    }
  }
  
  // Handle keyboard input for ASDW controls
  void _handleKeyEvent(RawKeyEvent event) {
    if (event is! RawKeyDownEvent) return;
    
    const double moveStep = 10.0;
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    // Switch between main character, teammate, and summons
    if (event.logicalKey == LogicalKeyboardKey.tab) {
      setState(() {
        if (_activeSummonIndex >= 0) {
          // If controlling a summon, switch back to main character
          _activeSummonIndex = -1;
          _controllingTeammate = false;
        } else if (_controllingTeammate) {
          // If controlling teammate, switch to first summon if available
          _controllingTeammate = false;
          if (_summonedCharacters.isNotEmpty) {
            _activeSummonIndex = 0;
          }
        } else if (_summonedCharacters.isNotEmpty) {
          // If controlling main character, switch to teammate
          _controllingTeammate = true;
          _activeSummonIndex = -1;
        } else {
          // If no summons, toggle between main and teammate
          _controllingTeammate = !_controllingTeammate;
        }
      });
      return;
    }
    
    setState(() {
      // Determine which character to move
      Offset position;
      if (_activeSummonIndex >= 0) {
        position = _summonedCharacters[_activeSummonIndex]['position'] as Offset;
      } else if (_controllingTeammate) {
        position = _teammatePosition;
      } else {
        position = _characterPosition;
      }
      
      if (event.logicalKey == LogicalKeyboardKey.keyW || 
          event.logicalKey == LogicalKeyboardKey.arrowUp) {
        // Move up
        position = Offset(position.dx, position.dy - moveStep);
        if (_activeSummonIndex >= 0) {
          _summonedCharacters[_activeSummonIndex]['action'] = 'walking';
        } else if (_controllingTeammate) {
          _teammateAction = 'walking';
        } else {
          characterModel.updateAction('walking');
        }
      } else if (event.logicalKey == LogicalKeyboardKey.keyA || 
                 event.logicalKey == LogicalKeyboardKey.arrowLeft) {
        // Move left
        position = Offset(position.dx - moveStep, position.dy);
        if (_activeSummonIndex >= 0) {
          _summonedCharacters[_activeSummonIndex]['action'] = 'walking';
        } else if (_controllingTeammate) {
          _teammateAction = 'walking';
        } else {
          characterModel.updateAction('walking');
        }
      } else if (event.logicalKey == LogicalKeyboardKey.keyS || 
                 event.logicalKey == LogicalKeyboardKey.arrowDown) {
        // Move down
        position = Offset(position.dx, position.dy + moveStep);
        if (_activeSummonIndex >= 0) {
          _summonedCharacters[_activeSummonIndex]['action'] = 'walking';
        } else if (_controllingTeammate) {
          _teammateAction = 'walking';
        } else {
          characterModel.updateAction('walking');
        }
      } else if (event.logicalKey == LogicalKeyboardKey.keyD || 
                 event.logicalKey == LogicalKeyboardKey.arrowRight) {
        // Move right
        position = Offset(position.dx + moveStep, position.dy);
        if (_activeSummonIndex >= 0) {
          _summonedCharacters[_activeSummonIndex]['action'] = 'walking';
        } else if (_controllingTeammate) {
          _teammateAction = 'walking';
        } else {
          characterModel.updateAction('walking');
        }
      }
      
      // Update the appropriate character position
      if (_activeSummonIndex >= 0) {
        _summonedCharacters[_activeSummonIndex]['position'] = position;
      } else if (_controllingTeammate) {
        _teammatePosition = position;
      } else {
        _characterPosition = position;
        // Update character model position
        characterModel.updatePosition(
          _characterPosition.dx,
          0,
          _characterPosition.dy,
        );
      }
    });
    
    // Check for interaction with items
    Offset positionToCheck;
    if (_activeSummonIndex >= 0) {
      positionToCheck = _summonedCharacters[_activeSummonIndex]['position'] as Offset;
    } else if (_controllingTeammate) {
      positionToCheck = _teammatePosition;
    } else {
      positionToCheck = _characterPosition;
    }
    _checkInteractionWithItems(positionToCheck);
  }

  // Show the quest board popup
  void _showQuestBoard() {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    final String characterName = _controllingTeammate ? 'Teammate' : characterModel.name;
    
    // Set character action to interact
    if (_controllingTeammate) {
      setState(() => _teammateAction = 'interacting with board');
    } else {
      characterModel.updateAction('interacting with board');
    }

    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('$characterName\'s Quest Board'),
        content: SizedBox(
          width: MediaQuery.of(context).size.width * 0.8,
          height: MediaQuery.of(context).size.height * 0.7,
          child: StatefulBuilder(
            builder: (context, setState) {
              // Create a QuestBoard widget with current game state and summoned characters
              return QuestBoard(
                gameState: _gameState,
                summonedCharacters: _summonedCharacters,
              );
            },
          ),
        ),
        actions: [
          TextButton(
            onPressed: () {
              // Reset character action when closing the dialog
              if (_controllingTeammate) {
                setState(() => _teammateAction = 'idle');
              } else {
                characterModel.updateAction('idle');
              }
              Navigator.of(context).pop();
              
              // Return focus to game area
              _focusNode.requestFocus();
            },
            child: const Text('Close'),
          ),
        ],
      ),
    );
  }

  // Show the grimoire popup
  void _showGrimoire() {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    final String characterName = _controllingTeammate ? 'Teammate' : characterModel.name;
    
    // Only the main character can use the grimoire
    if (_controllingTeammate) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Only the main character can use the Grimoire'),
          duration: Duration(seconds: 2),
        ),
      );
      return;
    }
    
    // Set character action to using grimoire
    characterModel.updateAction('using grimoire');

    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('$characterName\'s Grimoire'),
        content: SizedBox(
          width: MediaQuery.of(context).size.width * 0.8,
          height: MediaQuery.of(context).size.height * 0.7,
          child: DefaultTabController(
            length: 3,
            child: Column(
              children: [
                const TabBar(
                  tabs: [
                    Tab(text: 'Quests', icon: Icon(Icons.assignment)),
                    Tab(text: 'Magic', icon: Icon(Icons.auto_fix_high)),
                    Tab(text: 'Skills', icon: Icon(Icons.psychology)),
                  ],
                  labelColor: Colors.purple,
                ),
                Expanded(
                  child: TabBarView(
                    children: [
                      // Quests Tab
                      _buildQuestsTab(characterModel),
                      
                      // Magic Tab
                      _buildMagicTab(characterModel),
                      
                      // Skills Tab
                      _buildSkillsTab(characterModel),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () {
              // Reset character action when closing the dialog
              characterModel.updateAction('idle');
              Navigator.of(context).pop();
              
              // Return focus to game area
              _focusNode.requestFocus();
            },
            child: const Text('Close'),
          ),
        ],
      ),
    );
  }
  
  // Build the quests tab in the grimoire
  Widget _buildQuestsTab(CharacterModel characterModel) {
    return DefaultTabController(
      length: 2,
      child: Column(
        children: [
          const TabBar(
            tabs: [
              Tab(text: 'My Quests'),
              Tab(text: 'Manage Summons'),
            ],
            labelColor: Colors.purple,
          ),
          Expanded(
            child: TabBarView(
              children: [
                // My Quests subtab
                ListView(
                  children: const [
                    ListTile(
                      title: Text('Complete tutorial'),
                      subtitle: Text('Learn the basics of the guild system'),
                      leading: Icon(Icons.star, color: Colors.amber),
                      trailing: Text('Active'),
                    ),
                    ListTile(
                      title: Text('Recruit a teammate'),
                      subtitle: Text('Find someone to join your guild'),
                      leading: Icon(Icons.people, color: Colors.blue),
                      trailing: Text('Complete'),
                    ),
                    ListTile(
                      title: Text('Craft a potion'),
                      subtitle: Text('Use the alchemy table to make your first item'),
                      leading: Icon(Icons.science, color: Colors.green),
                      trailing: Text('Pending'),
                    ),
                  ],
                ),
                
                // Manage Summons subtab
                _summonedCharacters.isEmpty
                    ? const Center(child: Text('No summons active. Use magic to summon creatures.'))
                    : ListView.builder(
                        itemCount: _summonedCharacters.length,
                        itemBuilder: (context, index) {
                          final summon = _summonedCharacters[index];
                          return ListTile(
                            title: Text(summon['name']),
                            subtitle: Text(summon['type']),
                            leading: Icon(
                              summon['type'] == 'Shadow Clone' ? Icons.person_outline : Icons.pets,
                              color: summon['type'] == 'Shadow Clone' ? Colors.purple : Colors.brown,
                            ),
                            trailing: Row(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                Text(summon['action']),
                                IconButton(
                                  icon: const Icon(Icons.delete),
                                  onPressed: () {
                                    setState(() {
                                      _summonedCharacters.removeAt(index);
                                      if (_activeSummonIndex == index) {
                                        _activeSummonIndex = -1;
                                      } else if (_activeSummonIndex > index) {
                                        _activeSummonIndex--;
                                      }
                                    });
                                    Navigator.of(context).pop();
                                    _showGrimoire();
                                  },
                                ),
                              ],
                            ),
                            onTap: () {
                              setState(() {
                                _activeSummonIndex = _activeSummonIndex == index ? -1 : index;
                              });
                              Navigator.of(context).pop();
                              _showGrimoire();
                              
                              ScaffoldMessenger.of(context).showSnackBar(
                                SnackBar(
                                  content: Text(
                                    _activeSummonIndex == index
                                        ? 'Now controlling ${summon['name']}'
                                        : 'Stopped controlling ${summon['name']}'
                                  ),
                                  duration: const Duration(seconds: 2),
                                ),
                              );
                            },
                          );
                        },
                      ),
              ],
            ),
          ),
        ],
      ),
    );
  }
  
  // Build the magic tab in the grimoire
  Widget _buildMagicTab(CharacterModel characterModel) {
    return ListView(
      children: [
        ListTile(
          title: const Text('Summon Shadow Clone'),
          subtitle: const Text('Create a duplicate of yourself to assist you'),
          leading: const Icon(Icons.person_outline, color: Colors.purple),
          onTap: () {
            _summonCharacter('Shadow Clone');
            Navigator.of(context).pop();
            
            // Return focus to game area
            _focusNode.requestFocus();
          },
        ),
        ListTile(
          title: const Text('Summon Minion'),
          subtitle: const Text('Call forth a small creature to do your bidding'),
          leading: const Icon(Icons.pets, color: Colors.brown),
          onTap: () {
            _summonCharacter('Minion');
            Navigator.of(context).pop();
            
            // Return focus to game area
            _focusNode.requestFocus();
          },
        ),
        ListTile(
          title: const Text('Play Music'),
          subtitle: const Text('Cast music magic to play sounds through the turntables'),
          leading: const Icon(Icons.music_note, color: Colors.cyan),
          onTap: () {
            // Close grimoire first
            Navigator.of(context).pop();
            
            // Walk to turntables and play music
            _walkToTurntables();
            
            // Return focus to game area
            _focusNode.requestFocus();
          },
        ),
        const Divider(),
        ListTile(
          title: const Text('Healing Light'),
          subtitle: const Text('Restore health to yourself or an ally'),
          leading: const Icon(Icons.healing, color: Colors.green),
          onTap: () {
            Navigator.of(context).pop();
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('Healing spell cast. You feel rejuvenated.'),
                duration: Duration(seconds: 2),
              ),
            );
            
            // Return focus to game area
            _focusNode.requestFocus();
          },
        ),
        ListTile(
          title: const Text('Arcane Blast'),
          subtitle: const Text('Launch a powerful magic attack'),
          leading: const Icon(Icons.flash_on, color: Colors.blue),
          onTap: () {
            Navigator.of(context).pop();
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('Arcane energy bursts from your hands!'),
                duration: Duration(seconds: 2),
              ),
            );
            
            // Return focus to game area
            _focusNode.requestFocus();
          },
        ),
      ],
    );
  }
  
  // Build the skills tab in the grimoire
  Widget _buildSkillsTab(CharacterModel characterModel) {
    return ListView(
      children: [
        ListTile(
          title: const Text('Meditation'),
          subtitle: const Text('Restore your mental energy'),
          leading: const Icon(Icons.self_improvement, color: Colors.lightBlue),
          onTap: () {
            Navigator.of(context).pop();
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('You feel centered and refreshed.'),
                duration: Duration(seconds: 2),
              ),
            );
            
            // Return focus to game area
            _focusNode.requestFocus();
          },
        ),
        ListTile(
          title: const Text('Lockpicking'),
          subtitle: const Text('Attempt to open a lock without a key'),
          leading: const Icon(Icons.vpn_key, color: Colors.amber),
          onTap: () {
            Navigator.of(context).pop();
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('You attempt to pick a lock. Success!'),
                duration: Duration(seconds: 2),
              ),
            );
            
            // Return focus to game area
            _focusNode.requestFocus();
          },
        ),
        ListTile(
          title: const Text('Stealth'),
          subtitle: const Text('Move quietly and remain unseen'),
          leading: const Icon(Icons.visibility_off, color: Colors.grey),
          onTap: () {
            Navigator.of(context).pop();
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('You fade into the shadows.'),
                duration: Duration(seconds: 2),
              ),
            );
            
            // Return focus to game area
            _focusNode.requestFocus();
          },
        ),
      ],
    );
  }
  
  // Summon a new character
  void _summonCharacter(String type) {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    // Create random offset near the character
    final double randomX = (_characterPosition.dx + (80 * (Random().nextDouble() * 2 - 1))).clamp(100, 900);
    final double randomY = (_characterPosition.dy + (80 * (Random().nextDouble() * 2 - 1))).clamp(100, 600);
    
    // Determine height based on type
    final double height = type == 'Shadow Clone' ? 90.0 : 70.0;
    
    // Create new summon
    final newSummon = {
      'type': type,
      'name': type == 'Shadow Clone' 
          ? '${characterModel.name}\'s Clone' 
          : 'Shadow Minion ${_summonedCharacters.where((s) => s['type'] == 'Minion').length + 1}',
      'position': Offset(randomX, randomY),
      'action': 'idle',
      'color': type == 'Shadow Clone' 
          ? const Color(0xFF6A0DAD).withOpacity(0.9) 
          : const Color(0xFF4A148C),
      'height': height,
      'quests': [],
    };
    
    setState(() {
      // Toggle afterimage effect on main character
      characterModel.updateAction('summoning${type == 'Shadow Clone' ? ' clone' : ''}');
      
      _summonedCharacters.add(newSummon);
      // Automatically control the new summon
      _activeSummonIndex = _summonedCharacters.length - 1;
      
      // Add afterimage effect
      characterModel.setProperty('afterimage', 'on');
    });
    
    Future.delayed(const Duration(seconds: 1), () {
      if (mounted) {
        setState(() {
          characterModel.updateAction('idle');
        });
      }
    });
    
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('${newSummon['name']} has been summoned! Use Tab to switch back to main character.'),
        duration: const Duration(seconds: 3),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final characterModel = Provider.of<CharacterModel>(context);
    
    // Automatically request focus when the widget is built
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _focusNode.requestFocus();
    });
    
    return RawKeyboardListener(
      focusNode: _focusNode,
      onKey: _handleKeyEvent,
      autofocus: true,
      child: GestureDetector(
        onTapDown: (details) {
          // Move the active character to tapped position
          setState(() {
            if (_activeSummonIndex >= 0) {
              _summonedCharacters[_activeSummonIndex]['position'] = details.localPosition;
            } else if (_controllingTeammate) {
              _teammatePosition = details.localPosition;
            } else {
              _characterPosition = details.localPosition;
              // Update the character model position
              characterModel.updatePosition(
                _characterPosition.dx,
                0,
                _characterPosition.dy,
              );
            }
          });
          
          // Check if tapped near any room item
          Offset positionToCheck;
          if (_activeSummonIndex >= 0) {
            positionToCheck = _summonedCharacters[_activeSummonIndex]['position'] as Offset;
          } else if (_controllingTeammate) {
            positionToCheck = _teammatePosition;
          } else {
            positionToCheck = _characterPosition;
          }
          _checkInteractionWithItems(positionToCheck);
          
          // Make sure we keep focus for keyboard input
          _focusNode.requestFocus();
        },
        child: Stack(
          children: [
            // Room background - now larger guild hall
            Container(
              decoration: BoxDecoration(
                color: const Color(0xFF121212),
                border: Border.all(
                  color: const Color(0xFF6A0DAD),
                  width: 3,
                ),
                image: const DecorationImage(
                  image: AssetImage('assets/images/guild_hall_bg.png'),
                  fit: BoxFit.cover,
                  opacity: 0.4,
                ),
              ),
              width: 1000,
              height: 800,
            ),
            
            // Room items
            ..._roomItems.entries.map((entry) {
              final item = entry.value;
              final position = item['position'] as Offset;
              final isQuestBoard = entry.key == 'todoBoard';
              final isGrimoire = entry.key == 'grimoire';
              
              return Positioned(
                left: position.dx - 25,
                top: position.dy - 25,
                child: Column(
                  children: [
                    Stack(
                      alignment: Alignment.center,
                      children: [
                        // Base item icon
                        Icon(
                          item['icon'] as IconData,
                          color: isGrimoire ? Colors.purple : Colors.white70,
                          size: 50,
                        ),
                        
                        // Interactive overlay for quest board
                        if (isQuestBoard)
                          Positioned.fill(
                            child: Material(
                              color: Colors.transparent,
                              child: InkWell(
                                borderRadius: BorderRadius.circular(25),
                                splashColor: Colors.purple.withOpacity(0.3),
                                onTap: () {
                                  // Check if character is close enough to interact
                                  Offset activePosition;
                                  if (_activeSummonIndex >= 0) {
                                    activePosition = _summonedCharacters[_activeSummonIndex]['position'] as Offset;
                                  } else if (_controllingTeammate) {
                                    activePosition = _teammatePosition;
                                  } else {
                                    activePosition = _characterPosition;
                                  }
                                  
                                  if ((activePosition - position).distance < 80) {
                                    _showQuestBoard();
                                  } else {
                                    ScaffoldMessenger.of(context).showSnackBar(
                                      const SnackBar(
                                        content: Text('Move closer to interact with the Quest Board'),
                                        duration: Duration(seconds: 2),
                                      ),
                                    );
                                  }
                                },
                              ),
                            ),
                          ),
                        
                        // Interactive overlay for grimoire
                        if (isGrimoire)
                          Positioned.fill(
                            child: Material(
                              color: Colors.transparent,
                              child: InkWell(
                                borderRadius: BorderRadius.circular(25),
                                splashColor: Colors.amber.withOpacity(0.3),
                                onTap: () {
                                  // Check if character is close enough to interact
                                  Offset activePosition;
                                  if (_activeSummonIndex >= 0) {
                                    activePosition = _summonedCharacters[_activeSummonIndex]['position'] as Offset;
                                  } else if (_controllingTeammate) {
                                    activePosition = _teammatePosition;
                                  } else {
                                    activePosition = _characterPosition;
                                  }
                                  
                                  if ((activePosition - position).distance < 80) {
                                    _showGrimoire();
                                  } else {
                                    ScaffoldMessenger.of(context).showSnackBar(
                                      const SnackBar(
                                        content: Text('Move closer to interact with the Grimoire'),
                                        duration: Duration(seconds: 2),
                                      ),
                                    );
                                  }
                                },
                              ),
                            ),
                          ),
                      ],
                    ),
                    Text(
                      item['name'] as String,
                      style: TextStyle(
                        color: isGrimoire ? Colors.amber : Colors.white70,
                        fontWeight: isGrimoire ? FontWeight.bold : FontWeight.normal,
                      ),
                    ),
                    // Only for interactive items - add interaction hint
                    if (isQuestBoard || isGrimoire)
                      Text(
                        'Click to interact',
                        style: TextStyle(
                          color: isGrimoire ? Colors.amber[200] : Colors.purple[200],
                          fontSize: 10,
                          fontStyle: FontStyle.italic,
                        ),
                      ),
                  ],
                ),
              );
            }),
            
            // Summoned characters
            ..._summonedCharacters.asMap().entries.map((entry) {
              final index = entry.key;
              final summon = entry.value;
              final position = summon['position'] as Offset;
              final isActive = _activeSummonIndex == index;
              final height = summon['height'] as double;
              
              return Positioned(
                left: position.dx - 25,
                top: position.dy - 25,
                child: Column(
                  children: [
                    // Status text - now placed above character
                    Text(
                      summon['action'] as String,
                      style: const TextStyle(color: Colors.white70, fontSize: 10),
                    ),
                    Text(
                      summon['name'] as String,
                      style: const TextStyle(color: Colors.white, fontSize: 12),
                    ),
                    const SizedBox(height: 5),
                    AnimatedContainer(
                      duration: const Duration(milliseconds: 300),
                      width: 50,
                      height: height,
                      decoration: BoxDecoration(
                        color: isActive
                            ? summon['color'] as Color
                            : (summon['color'] as Color).withOpacity(0.7),
                        borderRadius: BorderRadius.circular(10),
                        boxShadow: [
                          BoxShadow(
                            color: Colors.black.withOpacity(0.5),
                            spreadRadius: 1,
                            blurRadius: 3,
                            offset: const Offset(0, 2),
                          ),
                        ],
                        border: isActive
                            ? Border.all(color: Colors.white, width: 2)
                            : null,
                      ),
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                          Container(
                            width: 30,
                            height: 30,
                            decoration: const BoxDecoration(
                              color: Colors.white,
                              shape: BoxShape.circle,
                            ),
                            child: Center(
                              child: Icon(
                                summon['type'] == 'Shadow Clone' ? Icons.person_outline : Icons.blur_on,
                                color: summon['type'] == 'Shadow Clone' ? Colors.purple : Colors.deepPurple,
                                size: summon['type'] == 'Shadow Clone' ? 20 : 18,
                              ),
                            ),
                          ),
                          const SizedBox(height: 5),
                          Container(
                            width: 40,
                            height: 30,
                            color: Colors.black,
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              );
            }),
            
            // Main character representation with afterimage effect
            Positioned(
              left: _characterPosition.dx - 25,
              top: _characterPosition.dy - 25,
              child: Column(
                children: [
                  // Status text - now placed above character
                  Text(
                    Provider.of<CharacterModel>(context).currentAction,
                    style: const TextStyle(color: Colors.white70, fontSize: 12),
                  ),
                  if (Provider.of<CharacterModel>(context).getProperty('afterimage') == 'on')
                    const Text(
                      'Afterimage: ON',
                      style: TextStyle(color: Colors.purple, fontSize: 10),
                    ),
                  Text(
                    Provider.of<CharacterModel>(context).name,
                    style: const TextStyle(color: Colors.white),
                  ),
                  const SizedBox(height: 5),
                  Stack(
                    children: [
                      // Afterimage effect
                      if (Provider.of<CharacterModel>(context).getProperty('afterimage') == 'on')
                        Positioned(
                          right: -5,
                          child: Container(
                            width: 50,
                            height: 80,
                            decoration: BoxDecoration(
                              color: const Color(0xFF6A0DAD).withOpacity(0.4),
                              borderRadius: BorderRadius.circular(10),
                            ),
                          ),
                        ),
                      // Main character
                      AnimatedContainer(
                        duration: const Duration(milliseconds: 300),
                        width: 50,
                        height: 80,
                        decoration: BoxDecoration(
                          color: !_controllingTeammate && _activeSummonIndex < 0
                              ? const Color(0xFF6A0DAD) // Purple for active
                              : const Color(0xFF6A0DAD).withOpacity(0.7), // Dimmed when inactive
                          borderRadius: BorderRadius.circular(10),
                          boxShadow: [
                            BoxShadow(
                              color: Colors.black.withOpacity(0.5),
                              spreadRadius: 1,
                              blurRadius: 3,
                              offset: const Offset(0, 2),
                            ),
                          ],
                          border: !_controllingTeammate && _activeSummonIndex < 0
                              ? Border.all(color: Colors.white, width: 2)
                              : null,
                        ),
                        child: Column(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            Container(
                              width: 30,
                              height: 30,
                              decoration: const BoxDecoration(
                                color: Colors.white,
                                shape: BoxShape.circle,
                              ),
                              child: Center(
                                child: Text(
                                  Provider.of<CharacterModel>(context).name[0],
                                  style: const TextStyle(
                                    color: Color(0xFF6A0DAD),
                                    fontWeight: FontWeight.bold,
                                  ),
                                ),
                              ),
                            ),
                            const SizedBox(height: 5),
                            Container(
                              width: 40,
                              height: 30,
                              color: Colors.black,
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
            
            // Quick cast magic buttons (only for main character when active)
            if (!_controllingTeammate && _activeSummonIndex < 0)
              ..._quickCastMagic.asMap().entries.map((entry) {
                final index = entry.key;
                final magic = entry.value;
                
                // Calculate position in a semicircle on the right side of the character
                final double angle = (pi / 4) + (index * (pi / 2) / (_quickCastMagic.length - 1));
                const double radius = 60.0;
                final double xOffset = cos(angle) * radius;
                final double yOffset = sin(angle) * radius;
                
                return Positioned(
                  left: _characterPosition.dx + xOffset - 15, // Center the button (half of width)
                  top: _characterPosition.dy + yOffset - 15, // Center the button (half of height)
                  child: Stack(
                    alignment: Alignment.center,
                    children: [
                      // Magic icon button
                      Material(
                        color: Colors.transparent,
                        child: InkWell(
                          onTap: () => _handleQuickCastMagic(index),
                          customBorder: const CircleBorder(),
                          child: Tooltip(
                            message: magic['name'],
                            preferBelow: false, // Show tooltip above the icon
                            verticalOffset: 20, // Increase distance from the icon
                            child: Container(
                              width: 35,
                              height: 35,
                              decoration: BoxDecoration(
                                color: magic['color'],
                                shape: BoxShape.circle,
                                boxShadow: [
                                  BoxShadow(
                                    color: Colors.black.withOpacity(0.5),
                                    spreadRadius: 1,
                                    blurRadius: 3,
                                    offset: const Offset(0, 1),
                                  ),
                                ],
                                border: Border.all(
                                  color: Colors.white.withOpacity(0.8),
                                  width: 2,
                                ),
                              ),
                              child: Icon(
                                magic['icon'],
                                color: Colors.white,
                                size: 22,
                              ),
                            ),
                          ),
                        ),
                      ),
                      
                      // Hourglass indicator for casting
                      if (magic['isCasting'])
                        Positioned(
                          top: -5,
                          right: -5,
                          child: Container(
                            width: 18,
                            height: 18,
                            decoration: BoxDecoration(
                              color: Colors.amber,
                              shape: BoxShape.circle,
                              border: Border.all(
                                color: Colors.white,
                                width: 1,
                              ),
                            ),
                            child: const Icon(
                              Icons.hourglass_empty,
                              color: Colors.white,
                              size: 12,
                            ),
                          ),
                        ),
                    ],
                  ),
                );
              }).toList(),
            
            // Teammate character representation
            Positioned(
              left: _teammatePosition.dx - 25,
              top: _teammatePosition.dy - 25,
              child: Column(
                children: [
                  // Status text - now placed above character
                  Text(
                    _teammateAction,
                    style: const TextStyle(color: Colors.white70, fontSize: 12),
                  ),
                  const Text(
                    'Teammate',
                    style: TextStyle(color: Colors.white),
                  ),
                  const SizedBox(height: 5),
                  AnimatedContainer(
                    duration: const Duration(milliseconds: 300),
                    width: 50,
                    height: 80,
                    decoration: BoxDecoration(
                      color: _controllingTeammate 
                          ? const Color(0xFF00C853) // Green for active
                          : const Color(0xFF00C853).withOpacity(0.7), // Dimmed when inactive
                      borderRadius: BorderRadius.circular(10),
                      boxShadow: [
                        BoxShadow(
                          color: Colors.black.withOpacity(0.5),
                          spreadRadius: 1,
                          blurRadius: 3,
                          offset: const Offset(0, 2),
                        ),
                      ],
                      border: _controllingTeammate
                          ? Border.all(color: Colors.white, width: 2)
                          : null,
                    ),
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Container(
                          width: 30,
                          height: 30,
                          decoration: const BoxDecoration(
                            color: Colors.white,
                            shape: BoxShape.circle,
                          ),
                          child: const Center(
                            child: Text(
                              'T', // Teammate
                              style: TextStyle(
                                color: Color(0xFF00C853),
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                          ),
                        ),
                        const SizedBox(height: 5),
                        Container(
                          width: 40,
                          height: 30,
                          color: Colors.black,
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
            
            // Controls info
            Positioned(
              left: 20,
              bottom: 20,
              child: Container(
                padding: const EdgeInsets.all(8),
                decoration: BoxDecoration(
                  color: Colors.black54,
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text(
                      'Controls:',
                      style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 5),
                    const Text(
                      'W, A, S, D - Move character',
                      style: TextStyle(color: Colors.white70),
                    ),
                    Text(
                      'Tab - Switch between characters (${_summonedCharacters.length + 2} total)',
                      style: const TextStyle(color: Colors.white70),
                    ),
                    const Text(
                      'Click - Move to position',
                      style: TextStyle(color: Colors.white70),
                    ),
                    const Text(
                      'Click on objects to interact',
                      style: TextStyle(color: Colors.white70),
                    ),
                  ],
                ),
              ),
            ),
            
            // Focus indicator and active character
            Positioned(
              right: 20,
              top: 20,
              child: Container(
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                decoration: BoxDecoration(
                  color: _focusNode.hasFocus 
                      ? const Color(0xFF00C853).withOpacity(0.8) 
                      : Colors.grey.withOpacity(0.5),
                  borderRadius: BorderRadius.circular(16),
                ),
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Icon(
                      _focusNode.hasFocus ? Icons.keyboard : Icons.keyboard_hide,
                      color: Colors.white,
                      size: 16,
                    ),
                    const SizedBox(width: 6),
                    Text(
                      _focusNode.hasFocus 
                          ? 'Controlling: ${_activeSummonIndex >= 0 ? _summonedCharacters[_activeSummonIndex]['name'] : (_controllingTeammate ? "Teammate" : "Main")}' 
                          : 'Click to Enable Keyboard',
                      style: const TextStyle(color: Colors.white, fontSize: 12),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
  
  void _checkInteractionWithItems(Offset position) {
    for (final entry in _roomItems.entries) {
      final itemPosition = entry.value['position'] as Offset;
      
      if ((position - itemPosition).distance < 50) {
        // Special handling for different items
        if (entry.key == 'todoBoard') {
          // Check if close enough
          if ((position - itemPosition).distance < 80) {
            _showQuestBoard();
          } else {
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('Move closer to interact with the Quest Board'),
                duration: Duration(seconds: 2),
              ),
            );
          }
        } else if (entry.key == 'grimoire') {
          // Check if close enough
          if ((position - itemPosition).distance < 80) {
            _showGrimoire();
          } else {
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('Move closer to interact with the Grimoire'),
                duration: Duration(seconds: 2),
              ),
            );
          }
        } else if (entry.key == 'dimension') {
          // The dimension portal can be accessed from anywhere
          _showDimensionOptions();
        } else {
          // Regular items
          _interactWithItem(entry.key);
        }
        break;
      }
    }
  }
  
  void _interactWithItem(String itemKey) {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    // Update character action based on the item
    if (_activeSummonIndex >= 0) {
      // Update summon action
      switch (itemKey) {
        case 'laptop':
          setState(() => _summonedCharacters[_activeSummonIndex]['action'] = 'sitting at laptop');
          break;
        case 'turntables':
          setState(() => _summonedCharacters[_activeSummonIndex]['action'] = 'using turntables');
          break;
        case 'webcam':
          setState(() => _summonedCharacters[_activeSummonIndex]['action'] = 'thinking');
          break;
      }
    } else if (_controllingTeammate) {
      // Update teammate action
      switch (itemKey) {
        case 'laptop':
          setState(() => _teammateAction = 'sitting at laptop');
          break;
        case 'turntables':
          setState(() => _teammateAction = 'using turntables');
          break;
        case 'webcam':
          setState(() => _teammateAction = 'thinking');
          break;
      }
    } else {
      // Update main character action
      switch (itemKey) {
        case 'laptop':
          characterModel.updateAction('sitting at laptop');
          break;
        case 'turntables':
          characterModel.updateAction('using turntables');
          break;
        case 'webcam':
          characterModel.updateAction('thinking');
          break;
      }
    }
    
    String characterName;
    if (_activeSummonIndex >= 0) {
      characterName = _summonedCharacters[_activeSummonIndex]['name'] as String;
    } else if (_controllingTeammate) {
      characterName = 'Teammate';
    } else {
      characterName = characterModel.name;
    }
    
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('$characterName is interacting with ${_roomItems[itemKey]!['name']}'),
        duration: const Duration(seconds: 2),
      ),
    );
  }

  void _walkToTurntables() {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    final turntablesPosition = _roomItems['turntables']!['position'] as Offset;
    
    // Set character action to walking to turntables
    characterModel.updateAction('walking to turntables');
    
    // Simulate walking animation by moving in steps
    const steps = 10;
    final dx = (turntablesPosition.dx - _characterPosition.dx) / steps;
    final dy = (turntablesPosition.dy - _characterPosition.dy) / steps;
    
    for (int i = 1; i <= steps; i++) {
      Future.delayed(Duration(milliseconds: i * 300), () {
        if (mounted) {
          setState(() {
            _characterPosition = Offset(
              _characterPosition.dx + dx,
              _characterPosition.dy + dy,
            );
            
            characterModel.updatePosition(
              _characterPosition.dx,
              0,
              _characterPosition.dy,
            );
            
            // When reached turntables
            if (i == steps) {
              characterModel.updateAction('using turntables');
              // Show music selection dialog
              _showMusicSelectionDialog();
            }
          });
        }
      });
    }
  }
  
  // Show music selection dialog
  void _showMusicSelectionDialog() {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Music Magic'),
        content: SizedBox(
          width: MediaQuery.of(context).size.width * 0.7,
          height: MediaQuery.of(context).size.height * 0.6,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text(
                'Select your music source:',
                style: TextStyle(fontWeight: FontWeight.bold, fontSize: 18),
              ),
              const SizedBox(height: 20),
              Expanded(
                child: ListView(
                  children: [
                    // MP3 Upload option
                    Card(
                      child: ListTile(
                        leading: const Icon(Icons.upload_file, color: Colors.blue),
                        title: const Text('Upload MP3'),
                        subtitle: const Text('Upload an MP3 file from your device'),
                        onTap: () {
                          Navigator.of(context).pop();
                          _showFileUploadDialog();
                        },
                      ),
                    ),
                    const SizedBox(height: 8),
                    // Spotify option
                    Card(
                      child: ListTile(
                        leading: const Icon(Icons.music_note, color: Colors.green),
                        title: const Text('Connect Spotify'),
                        subtitle: const Text('Play music from your Spotify account'),
                        onTap: () {
                          Navigator.of(context).pop();
                          _connectToSpotify();
                        },
                      ),
                    ),
                    const SizedBox(height: 8),
                    // YouTube Music option
                    Card(
                      child: ListTile(
                        leading: const Icon(Icons.music_video, color: Colors.red),
                        title: const Text('YouTube Music'),
                        subtitle: const Text('Play music from YouTube Music'),
                        onTap: () {
                          Navigator.of(context).pop();
                          _connectToYouTubeMusic();
                        },
                      ),
                    ),
                    const SizedBox(height: 8),
                    // SoundCloud option
                    Card(
                      child: ListTile(
                        leading: const Icon(Icons.cloud, color: Colors.orange),
                        title: const Text('SoundCloud'),
                        subtitle: const Text('Play music from SoundCloud'),
                        onTap: () {
                          Navigator.of(context).pop();
                          _connectToSoundCloud();
                        },
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
        actions: [
          TextButton(
            onPressed: () {
              // Reset character action when closing the dialog
              characterModel.updateAction('idle');
              Navigator.of(context).pop();
              
              // Return focus to game area
              _focusNode.requestFocus();
            },
            child: const Text('Cancel'),
          ),
        ],
      ),
    );
  }
  
  // File upload dialog
  void _showFileUploadDialog() {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Upload MP3'),
        content: const Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.upload_file, size: 48, color: Colors.blue),
            SizedBox(height: 16),
            Text(
              'This would open a file dialog to select an MP3 file from your device.',
              textAlign: TextAlign.center,
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () {
              characterModel.updateAction('playing music');
              Navigator.of(context).pop();
              
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(
                  content: Text('Now playing: Your MP3 file'),
                  duration: Duration(seconds: 3),
                ),
              );
              
              // Simulate stopping music after some time
              Future.delayed(const Duration(seconds: 10), () {
                if (mounted) {
                  characterModel.updateAction('idle');
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(
                      content: Text('Music stopped'),
                      duration: Duration(seconds: 2),
                    ),
                  );
                }
              });
              
              _focusNode.requestFocus();
            },
            child: const Text('Simulate Upload'),
          ),
          TextButton(
            onPressed: () {
              characterModel.updateAction('idle');
              Navigator.of(context).pop();
              _focusNode.requestFocus();
            },
            child: const Text('Cancel'),
          ),
        ],
      ),
    );
  }
  
  // Connect to Spotify
  void _connectToSpotify() {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Spotify Integration'),
        content: const Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.music_note, size: 48, color: Colors.green),
            SizedBox(height: 16),
            Text(
              'This would integrate with Spotify to play music from your account.\n\n'
              'In a full implementation, this would open Spotify authorization.',
              textAlign: TextAlign.center,
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () {
              characterModel.updateAction('playing music');
              Navigator.of(context).pop();
              
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(
                  content: Text('Connected to Spotify. Music is now playing.'),
                  duration: Duration(seconds: 3),
                ),
              );
              
              _focusNode.requestFocus();
            },
            child: const Text('Simulate Connection'),
          ),
          TextButton(
            onPressed: () {
              characterModel.updateAction('idle');
              Navigator.of(context).pop();
              _focusNode.requestFocus();
            },
            child: const Text('Cancel'),
          ),
        ],
      ),
    );
  }
  
  // Connect to YouTube Music
  void _connectToYouTubeMusic() {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('YouTube Music Integration'),
        content: const Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.music_video, size: 48, color: Colors.red),
            SizedBox(height: 16),
            Text(
              'This would integrate with YouTube Music to play songs from your account.',
              textAlign: TextAlign.center,
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () {
              characterModel.updateAction('playing music');
              Navigator.of(context).pop();
              
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(
                  content: Text('Connected to YouTube Music. Music is now playing.'),
                  duration: Duration(seconds: 3),
                ),
              );
              
              _focusNode.requestFocus();
            },
            child: const Text('Simulate Connection'),
          ),
          TextButton(
            onPressed: () {
              characterModel.updateAction('idle');
              Navigator.of(context).pop();
              _focusNode.requestFocus();
            },
            child: const Text('Cancel'),
          ),
        ],
      ),
    );
  }
  
  // Connect to SoundCloud
  void _connectToSoundCloud() {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('SoundCloud Integration'),
        content: const Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.cloud, size: 48, color: Colors.orange),
            SizedBox(height: 16),
            Text(
              'This would integrate with SoundCloud to play tracks from your account.',
              textAlign: TextAlign.center,
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () {
              characterModel.updateAction('playing music');
              Navigator.of(context).pop();
              
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(
                  content: Text('Connected to SoundCloud. Music is now playing.'),
                  duration: Duration(seconds: 3),
                ),
              );
              
              _focusNode.requestFocus();
            },
            child: const Text('Simulate Connection'),
          ),
          TextButton(
            onPressed: () {
              characterModel.updateAction('idle');
              Navigator.of(context).pop();
              _focusNode.requestFocus();
            },
            child: const Text('Cancel'),
          ),
        ],
      ),
    );
  }

  // Handle quick cast magic
  void _handleQuickCastMagic(int index) {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    // Don't allow teammate or summons to use magic
    if (_controllingTeammate || _activeSummonIndex >= 0) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Only the main character can cast magic'),
          duration: Duration(seconds: 2),
        ),
      );
      return;
    }
    
    // Set the casting state
    setState(() {
      _quickCastMagic[index]['isCasting'] = true;
    });
    
    // Cast the magic based on its name
    switch (_quickCastMagic[index]['name']) {
      case 'Shadow Clone':
        _summonCharacter('Shadow Clone');
        break;
      case 'Minion':
        _summonCharacter('Minion');
        break;
      case 'Play Music':
        _walkToTurntables();
        break;
      case 'Healing Light':
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Healing spell cast. You feel rejuvenated.'),
            duration: Duration(seconds: 2),
          ),
        );
        break;
      case 'Arcane Blast':
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Arcane energy bursts from your hands!'),
            duration: Duration(seconds: 2),
          ),
        );
        break;
    }
    
    // Reset the casting state after a delay
    Future.delayed(const Duration(seconds: 2), () {
      if (mounted) {
        setState(() {
          _quickCastMagic[index]['isCasting'] = false;
        });
      }
    });
  }
} 