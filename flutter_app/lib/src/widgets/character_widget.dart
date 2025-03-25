import 'dart:async';

import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../models/character.dart';
import '../models/enums/character_state.dart';
import '../providers/animation_settings_provider.dart';

/// Widget to display a character in the game world
class CharacterWidget extends StatefulWidget {
  /// The character to display
  final Character character;
  
  /// Constructor
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