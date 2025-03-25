import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import '../models/character_model.dart';

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

  // Character position
  Offset _characterPosition = const Offset(250, 250);
  
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
    
    setState(() {
      if (event.logicalKey == LogicalKeyboardKey.keyW || 
          event.logicalKey == LogicalKeyboardKey.arrowUp) {
        // Move up
        _characterPosition = Offset(_characterPosition.dx, _characterPosition.dy - moveStep);
        characterModel.updateAction('walking');
      } else if (event.logicalKey == LogicalKeyboardKey.keyA || 
                 event.logicalKey == LogicalKeyboardKey.arrowLeft) {
        // Move left
        _characterPosition = Offset(_characterPosition.dx - moveStep, _characterPosition.dy);
        characterModel.updateAction('walking');
      } else if (event.logicalKey == LogicalKeyboardKey.keyS || 
                 event.logicalKey == LogicalKeyboardKey.arrowDown) {
        // Move down
        _characterPosition = Offset(_characterPosition.dx, _characterPosition.dy + moveStep);
        characterModel.updateAction('walking');
      } else if (event.logicalKey == LogicalKeyboardKey.keyD || 
                 event.logicalKey == LogicalKeyboardKey.arrowRight) {
        // Move right
        _characterPosition = Offset(_characterPosition.dx + moveStep, _characterPosition.dy);
        characterModel.updateAction('walking');
      }
    });
    
    // Update character model position
    characterModel.updatePosition(
      _characterPosition.dx,
      0,
      _characterPosition.dy,
    );
    
    // Check for interaction with items
    _checkInteractionWithItems(_characterPosition);
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
          // Move character to tapped position
          setState(() {
            _characterPosition = details.localPosition;
          });
          
          // Update the character model position
          characterModel.updatePosition(
            _characterPosition.dx,
            0,
            _characterPosition.dy,
          );
          
          // Check if tapped near any room item
          _checkInteractionWithItems(details.localPosition);
          
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
              
              return Positioned(
                left: position.dx - 25,
                top: position.dy - 25,
                child: Column(
                  children: [
                    Icon(
                      item['icon'] as IconData,
                      color: Colors.white70,
                      size: 50,
                    ),
                    Text(
                      item['name'] as String,
                      style: const TextStyle(color: Colors.white70),
                    ),
                  ],
                ),
              );
            }),
            
            // Simple character representation
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
                      color: const Color(0xFF6A0DAD),
                      borderRadius: BorderRadius.circular(10),
                      boxShadow: [
                        BoxShadow(
                          color: Colors.black.withOpacity(0.5),
                          spreadRadius: 1,
                          blurRadius: 3,
                          offset: const Offset(0, 2),
                        ),
                      ],
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
                      'Click - Move to position',
                      style: TextStyle(color: Colors.white70),
                    ),
                  ],
                ),
              ),
            ),
            
            // Focus indicator (to show keyboard focus is active)
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
                      _focusNode.hasFocus ? Icons.keyboard : Icons.keyboard_off,
                      color: Colors.white,
                      size: 16,
                    ),
                    const SizedBox(width: 6),
                    Text(
                      _focusNode.hasFocus ? 'Keyboard Active' : 'Click to Enable Keyboard',
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
        _interactWithItem(entry.key);
        break;
      }
    }
  }
  
  void _interactWithItem(String itemKey) {
    final characterModel = Provider.of<CharacterModel>(context, listen: false);
    
    // Update character action based on the item
    switch (itemKey) {
      case 'todoBoard':
        characterModel.updateAction('pointing at board');
        break;
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
    
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('Interacting with ${_roomItems[itemKey]!['name']}'),
        duration: const Duration(seconds: 2),
      ),
    );
  }
} 