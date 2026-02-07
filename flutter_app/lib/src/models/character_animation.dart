import 'animation.dart';
import 'animation_frame.dart';

/// Character-specific animation implementation
class CharacterAnimation extends Animation {
  CharacterAnimation() {
    // Initialize character animation frames
    animations['idle'] = [
      const Frame(image: 'assets/animations/character/idle_1.png', duration: 500),
      const Frame(image: 'assets/animations/character/idle_2.png', duration: 500),
    ];
    
    animations['walk'] = [
      const Frame(image: 'assets/animations/character/walk_1.png', duration: 150),
      const Frame(image: 'assets/animations/character/walk_2.png', duration: 150),
      const Frame(image: 'assets/animations/character/walk_3.png', duration: 150),
      const Frame(image: 'assets/animations/character/walk_4.png', duration: 150),
    ];
    
    animations['sit_on_throne'] = [
      const Frame(image: 'assets/animations/character/sit_1.png', duration: 200),
      const Frame(image: 'assets/animations/character/sit_2.png', duration: 200),
      const Frame(image: 'assets/animations/character/sit_3.png', duration: 200),
      const Frame(image: 'assets/animations/character/sitting.png', duration: -1), // -1 means hold
    ];
    
    animations['stand_up'] = [
      const Frame(image: 'assets/animations/character/sit_3.png', duration: 200),
      const Frame(image: 'assets/animations/character/sit_2.png', duration: 200),
      const Frame(image: 'assets/animations/character/sit_1.png', duration: 200),
      const Frame(image: 'assets/animations/character/idle_1.png', duration: 200),
    ];
    
    animations['cast'] = [
      const Frame(image: 'assets/animations/character/cast_1.png', duration: 150),
      const Frame(image: 'assets/animations/character/cast_2.png', duration: 150),
      const Frame(image: 'assets/animations/character/cast_3.png', duration: 150),
      const Frame(image: 'assets/animations/character/cast_4.png', duration: 150),
    ];
    
    animations['interact'] = [
      const Frame(image: 'assets/animations/character/interact_1.png', duration: 200),
      const Frame(image: 'assets/animations/character/interact_2.png', duration: 200),
      const Frame(image: 'assets/animations/character/interact_3.png', duration: 200),
    ];
  }
  
  @override
  String getStaticFrameForAnimation(String animationName) {
    // Override to provide character-specific defaults
    final result = super.getStaticFrameForAnimation(animationName);
    
    // If no suitable frame found, return character static image
    if (result == 'assets/images/default.png') {
      return 'assets/images/character/static.png';
    }
    
    return result;
  }
} 