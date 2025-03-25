import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import '../models/character_model.dart';
import '../widgets/quest_board.dart';

class SimpleCharacterView extends StatefulWidget {
  const SimpleCharacterView({Key? key}) : super(key: key);

  @override
  State<SimpleCharacterView> createState() => _SimpleCharacterViewState();
}

class _SimpleCharacterViewState extends State<SimpleCharacterView> {
  // Room items with their positions
  final Map<String, Map<String, dynamic>> _roomItems = {
    'todoBoard': {
      'position': const Offset(100, 100),
      'name': 'Quest Board',
      'icon': Icons.assignment,
    },
    'laptop': {
      'position': const Offset(300, 200),
      'name': 'Laptop',
      'icon': Icons.laptop,
    },
    'turntables': {
      'position': const Offset(500, 400),
      'name': 'Turntables',
      'icon': Icons.music_note,
    },
    'webcam': {
      'position': const Offset(300, 100),
      'name': 'Webcam',
      'icon': Icons.videocam,
    },
  };

  // Main character position
  Offset _characterPosition = const Offset(250, 250);
  
  // Teammate character position
  Offset _teammatePosition = const Offset(350, 250);
  String _teammateAction = 'idle';
  bool _controllingTeammate = false;
  
  // Focus node for keyboard input
  final FocusNode _focusNode = FocusNode();
  
  @override
  void dispose() {
    _focusNode.dispose();
    super.dispose();
  }
  
  // Handle keyboard input for ASDW controls
  void _handleKeyEvent(RawKeyEvent event) {
    if (event is! RawKeyDownEvent) return;
    
    const double moveStep = 10.0;
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    // Switch between main character and teammate
    if (event.logicalKey == LogicalKeyboardKey.tab) {
      setState(() {
        _controllingTeammate = !_controllingTeammate;
      });
      return;
    }
    
    setState(() {
      // Determine which character to move
      Offset position = _controllingTeammate ? _teammatePosition : _characterPosition;
      
      if (event.logicalKey == LogicalKeyboardKey.keyW || 
          event.logicalKey == LogicalKeyboardKey.arrowUp) {
        // Move up
        position = Offset(position.dx, position.dy - moveStep);
        if (_controllingTeammate) {
          _teammateAction = 'walking';
        } else {
          characterModel.updateAction('walking');
        }
      } else if (event.logicalKey == LogicalKeyboardKey.keyA || 
                 event.logicalKey == LogicalKeyboardKey.arrowLeft) {
        // Move left
        position = Offset(position.dx - moveStep, position.dy);
        if (_controllingTeammate) {
          _teammateAction = 'walking';
        } else {
          characterModel.updateAction('walking');
        }
      } else if (event.logicalKey == LogicalKeyboardKey.keyS || 
                 event.logicalKey == LogicalKeyboardKey.arrowDown) {
        // Move down
        position = Offset(position.dx, position.dy + moveStep);
        if (_controllingTeammate) {
          _teammateAction = 'walking';
        } else {
          characterModel.updateAction('walking');
        }
      } else if (event.logicalKey == LogicalKeyboardKey.keyD || 
                 event.logicalKey == LogicalKeyboardKey.arrowRight) {
        // Move right
        position = Offset(position.dx + moveStep, position.dy);
        if (_controllingTeammate) {
          _teammateAction = 'walking';
        } else {
          characterModel.updateAction('walking');
        }
      }
      
      // Update the appropriate character position
      if (_controllingTeammate) {
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
    Offset positionToCheck = _controllingTeammate ? _teammatePosition : _characterPosition;
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
          child: const QuestBoard(),
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
            if (_controllingTeammate) {
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
          _checkInteractionWithItems(_controllingTeammate ? _teammatePosition : _characterPosition);
          
          // Make sure we keep focus for keyboard input
          _focusNode.requestFocus();
        },
        child: Stack(
          children: [
            // Room background
            Container(
              decoration: BoxDecoration(
                color: const Color(0xFF121212),
                border: Border.all(
                  color: const Color(0xFF6A0DAD),
                  width: 3,
                ),
              ),
              width: double.infinity,
              height: double.infinity,
            ),
            
            // Room items
            ..._roomItems.entries.map((entry) {
              final item = entry.value;
              final position = item['position'] as Offset;
              final isQuestBoard = entry.key == 'todoBoard';
              
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
                          color: Colors.white70,
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
                                  final activePosition = _controllingTeammate 
                                      ? _teammatePosition 
                                      : _characterPosition;
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
                      ],
                    ),
                    Text(
                      item['name'] as String,
                      style: const TextStyle(color: Colors.white70),
                    ),
                    // Only for quest board - add interaction hint
                    if (isQuestBoard)
                      Text(
                        'Click to interact',
                        style: TextStyle(
                          color: Colors.purple[200],
                          fontSize: 10,
                          fontStyle: FontStyle.italic,
                        ),
                      ),
                  ],
                ),
              );
            }),
            
            // Teammate character representation
            Positioned(
              left: _teammatePosition.dx - 25,
              top: _teammatePosition.dy - 25,
              child: Column(
                children: [
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
                  const SizedBox(height: 5),
                  const Text(
                    'Teammate',
                    style: TextStyle(color: Colors.white),
                  ),
                  Text(
                    _teammateAction,
                    style: const TextStyle(color: Colors.white70, fontSize: 12),
                  ),
                ],
              ),
            ),
            
            // Main character representation
            Positioned(
              left: _characterPosition.dx - 25,
              top: _characterPosition.dy - 25,
              child: Column(
                children: [
                  AnimatedContainer(
                    duration: const Duration(milliseconds: 300),
                    width: 50,
                    height: 80,
                    decoration: BoxDecoration(
                      color: !_controllingTeammate 
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
                      border: !_controllingTeammate
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
                              characterModel.name[0],
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
                  const SizedBox(height: 5),
                  Text(
                    characterModel.name,
                    style: const TextStyle(color: Colors.white),
                  ),
                  Text(
                    characterModel.currentAction,
                    style: const TextStyle(color: Colors.white70, fontSize: 12),
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
                child: const Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'Controls:',
                      style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold),
                    ),
                    SizedBox(height: 5),
                    Text(
                      'W, A, S, D - Move character',
                      style: TextStyle(color: Colors.white70),
                    ),
                    Text(
                      'Tab - Switch between characters',
                      style: TextStyle(color: Colors.white70),
                    ),
                    Text(
                      'Click - Move to position',
                      style: TextStyle(color: Colors.white70),
                    ),
                    Text(
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
                          ? 'Controlling: ${_controllingTeammate ? "Teammate" : "Main"}' 
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
        // Skip the quest board as it has its own interaction
        if (entry.key != 'todoBoard') {
          _interactWithItem(entry.key);
        }
        break;
      }
    }
  }
  
  void _interactWithItem(String itemKey) {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    // Update character action based on the item
    if (_controllingTeammate) {
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
    
    String characterName = _controllingTeammate ? 'Teammate' : characterModel.name;
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('$characterName is interacting with ${_roomItems[itemKey]!['name']}'),
        duration: const Duration(seconds: 2),
      ),
    );
  }
} 