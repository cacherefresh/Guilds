import 'dart:async';

import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../models/enums/minion_state.dart';
import '../models/minion.dart';
import '../providers/animation_settings_provider.dart';

/// Widget to display a minion in the game world
class MinionWidget extends StatefulWidget {
  /// The minion to display
  final Minion minion;
  
  /// Constructor
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
              padding: const EdgeInsets.all(4),
              decoration: BoxDecoration(
                color: Colors.black.withOpacity(0.7),
                borderRadius: BorderRadius.circular(4),
              ),
              child: Column(
                children: [
                  // State label
                  Text(
                    _getStateText(widget.minion.state),
                    style: const TextStyle(color: Colors.white, fontSize: 10),
                  ),
                  // Animation name
                  Text(
                    'Animation: ${widget.minion.animation.currentAnimationName}',
                    style: const TextStyle(color: Colors.white, fontSize: 10),
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