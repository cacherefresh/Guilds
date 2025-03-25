import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import '../models/character_model.dart';
import '../widgets/quest_board.dart';
import 'dart:math';

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
    'grimoire': {
      'position': const Offset(450, 150),
      'name': 'Grimoire',
      'icon': Icons.book,
    },
  };

  // Main character position
  Offset _characterPosition = const Offset(250, 250);
  
  // Teammate character position
  Offset _teammatePosition = const Offset(350, 250);
  String _teammateAction = 'idle';
  bool _controllingTeammate = false;
  
  // Summoned characters
  List<Map<String, dynamic>> _summonedCharacters = [];
  int _activeSummonIndex = -1; // -1 means no summon is active
  
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
                  children: [
                    ListTile(
                      title: const Text('Complete tutorial'),
                      subtitle: const Text('Learn the basics of the guild system'),
                      leading: const Icon(Icons.star, color: Colors.amber),
                      trailing: const Text('Active'),
                    ),
                    ListTile(
                      title: const Text('Recruit a teammate'),
                      subtitle: const Text('Find someone to join your guild'),
                      leading: const Icon(Icons.people, color: Colors.blue),
                      trailing: const Text('Complete'),
                    ),
                    ListTile(
                      title: const Text('Craft a potion'),
                      subtitle: const Text('Use the alchemy table to make your first item'),
                      leading: const Icon(Icons.science, color: Colors.green),
                      trailing: const Text('Pending'),
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
    final double randomX = (_characterPosition.dx + (50 * (Random().nextDouble() * 2 - 1))).clamp(100, 500);
    final double randomY = (_characterPosition.dy + (50 * (Random().nextDouble() * 2 - 1))).clamp(100, 400);
    
    // Create new summon
    final newSummon = {
      'type': type,
      'name': type == 'Shadow Clone' 
          ? '${characterModel.name}\'s Clone' 
          : 'Minion ${_summonedCharacters.where((s) => s['type'] == 'Minion').length + 1}',
      'position': Offset(randomX, randomY),
      'action': 'idle',
      'color': type == 'Shadow Clone' 
          ? const Color(0xFF6A0DAD).withOpacity(0.8) 
          : const Color(0xFFBF360C),
      'quests': [],
    };
    
    setState(() {
      _summonedCharacters.add(newSummon);
      // Automatically control the new summon
      _activeSummonIndex = _summonedCharacters.length - 1;
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
              
              return Positioned(
                left: position.dx - 25,
                top: position.dy - 25,
                child: Column(
                  children: [
                    AnimatedContainer(
                      duration: const Duration(milliseconds: 300),
                      width: 50,
                      height: 80,
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
                                summon['type'] == 'Shadow Clone' ? Icons.person_outline : Icons.pets,
                                color: summon['type'] == 'Shadow Clone' ? Colors.purple : Colors.brown,
                                size: 20,
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
                      summon['name'] as String,
                      style: const TextStyle(color: Colors.white, fontSize: 12),
                    ),
                    Text(
                      summon['action'] as String,
                      style: const TextStyle(color: Colors.white70, fontSize: 10),
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
        // Skip the quest board and grimoire as they have their own interaction
        if (entry.key != 'todoBoard' && entry.key != 'grimoire') {
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
} 