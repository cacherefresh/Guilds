import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../models/character_model.dart';

class CharacterView extends StatefulWidget {
  const CharacterView({Key? key}) : super(key: key);

  @override
  State<CharacterView> createState() => _CharacterViewState();
}

class _CharacterViewState extends State<CharacterView> {
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

  // Mock character position (will be replaced by actual 3D position)
  Offset _characterPosition = const Offset(250, 250);
  
  @override
  Widget build(BuildContext context) {
    final characterModel = Provider.of<CharacterModel>(context);
    
    return GestureDetector(
      onTapDown: (details) {
        // Move character to tapped position
        setState(() {
          _characterPosition = details.localPosition;
        });
        
        // Update the character model position
        // Note: This is just a 2D approximation. In 3D, we'd also update Z
        characterModel.updatePosition(
          _characterPosition.dx,
          0, // In 2D mockup, we keep Y as 0
          _characterPosition.dy,
        );
        
        // Check if tapped near any room item
        _checkInteractionWithItems(details.localPosition);
      },
      child: Stack(
        children: [
          // Background room
          Container(
            color: const Color(0xFF121212),
            width: double.infinity,
            height: double.infinity,
            child: const Center(
              child: Text(
                'TODO: Replace with 3D Room Rendering',
                style: TextStyle(color: Colors.white70),
              ),
            ),
          ),
          
          // Room items
          ..._roomItems.entries.map((entry) {
            final item = entry.value;
            final position = item['position'] as Offset;
            
            return Positioned(
              left: position.dx - 25, // Center the icon
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
          
          // Character representation (placeholder)
          Positioned(
            left: _characterPosition.dx - 25,
            top: _characterPosition.dy - 25,
            child: Column(
              children: [
                Container(
                  width: 50,
                  height: 50,
                  decoration: const BoxDecoration(
                    color: Color(0xFF6A0DAD),
                    shape: BoxShape.circle,
                  ),
                  child: const Icon(
                    Icons.person,
                    color: Colors.white,
                    size: 30,
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
          
          // Keyboard controls overlay
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
        ],
      ),
    );
  }
  
  void _checkInteractionWithItems(Offset tapPosition) {
    // For each room item, check if the tap is close enough to interact
    for (final entry in _roomItems.entries) {
      final itemPosition = entry.value['position'] as Offset;
      
      // If within 50 pixels of the item (simple collision detection)
      if ((tapPosition - itemPosition).distance < 50) {
        // Interact with the item
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
        characterModel.pointAt(
          _roomItems[itemKey]!['position'].dx,
          0,
          _roomItems[itemKey]!['position'].dy,
        );
        // TODO: Show quest board interaction
        break;
      case 'laptop':
        characterModel.updateAction('sitting');
        // TODO: Show laptop interaction
        break;
      case 'turntables':
        characterModel.updateAction('using turntables');
        // TODO: Show music interaction
        break;
      case 'webcam':
        characterModel.think();
        // TODO: Show webcam interaction
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