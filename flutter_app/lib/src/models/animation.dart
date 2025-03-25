import 'animation_frame.dart';

/// Base class for character and minion animations
abstract class Animation {
  /// Map of animation name to frames
  final Map<String, List<Frame>> animations = {};
  
  /// Current animation being played
  String _currentAnimation = 'idle';
  
  /// Current frame index
  int _currentFrame = 0;
  
  /// Timer for tracking frame duration
  int _frameTimer = 0;
  
  /// Get the current animation name
  String get currentAnimationName => _currentAnimation;
  
  /// Get the current frame image
  String get currentFrameImage => 
      animations[_currentAnimation]?[_currentFrame].image ?? 
      'assets/images/default.png';
  
  /// Update the animation based on elapsed time
  void update(int deltaTimeMs) {
    if (!animations.containsKey(_currentAnimation)) return;
    
    final frames = animations[_currentAnimation]!;
    if (frames.isEmpty) return;
    
    final frameDuration = frames[_currentFrame].duration;
    
    // If duration is -1, hold frame indefinitely
    if (frameDuration == -1) return;
    
    _frameTimer += deltaTimeMs;
    
    if (_frameTimer >= frameDuration) {
      _frameTimer = 0;
      _currentFrame = (_currentFrame + 1) % frames.length;
    }
  }
  
  /// Play an animation by name
  void play(String animationName) {
    if (animations.containsKey(animationName) && _currentAnimation != animationName) {
      _currentAnimation = animationName;
      _currentFrame = 0;
      _frameTimer = 0;
    }
  }
  
  /// Reset the current animation
  void reset() {
    _currentFrame = 0;
    _frameTimer = 0;
  }
  
  /// Get a static frame for no-animation mode
  String getStaticFrameForAnimation(String animationName) {
    if (!animations.containsKey(animationName)) {
      return 'assets/images/default.png';
    }
    
    // Return the most representative frame from the animation
    final frames = animations[animationName]!;
    if (frames.isEmpty) return 'assets/images/default.png';
    
    // For animations with a hold frame (duration -1), use that
    for (int i = 0; i < frames.length; i++) {
      if (frames[i].duration == -1) {
        return frames[i].image;
      }
    }
    
    // Otherwise use the first frame
    return frames.first.image;
  }
} 