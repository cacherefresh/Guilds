import 'animation.dart';
import 'animation_frame.dart';

/// Minion-specific animation implementation
class MinionAnimation extends Animation {
  MinionAnimation() {
    // Initialize minion animation frames
    animations['idle'] = [
      Frame(image: 'assets/animations/minion/idle_1.png', duration: 500),
      Frame(image: 'assets/animations/minion/idle_2.png', duration: 500),
    ];
    
    animations['walk'] = [
      Frame(image: 'assets/animations/minion/walk_1.png', duration: 150),
      Frame(image: 'assets/animations/minion/walk_2.png', duration: 150),
      Frame(image: 'assets/animations/minion/walk_3.png', duration: 150),
      Frame(image: 'assets/animations/minion/walk_4.png', duration: 150),
    ];
    
    animations['quest_running'] = [
      Frame(image: 'assets/animations/minion/run_1.png', duration: 100),
      Frame(image: 'assets/animations/minion/run_2.png', duration: 100),
      Frame(image: 'assets/animations/minion/run_3.png', duration: 100),
      Frame(image: 'assets/animations/minion/run_4.png', duration: 100),
      Frame(image: 'assets/animations/minion/run_5.png', duration: 100),
      Frame(image: 'assets/animations/minion/run_6.png', duration: 100),
    ];
    
    animations['completed_task'] = [
      Frame(image: 'assets/animations/minion/complete_1.png', duration: 200),
      Frame(image: 'assets/animations/minion/complete_2.png', duration: 200),
      Frame(image: 'assets/animations/minion/complete_3.png', duration: 200),
      Frame(image: 'assets/animations/minion/complete_4.png', duration: 200),
      Frame(image: 'assets/animations/minion/complete_5.png', duration: 200),
    ];
    
    animations['follow'] = [
      Frame(image: 'assets/animations/minion/follow_1.png', duration: 200),
      Frame(image: 'assets/animations/minion/follow_2.png', duration: 200),
    ];
  }
  
  @override
  String getStaticFrameForAnimation(String animationName) {
    // Override to provide minion-specific defaults
    final result = super.getStaticFrameForAnimation(animationName);
    
    // If no suitable frame found, return minion static image
    if (result == 'assets/images/default.png') {
      return 'assets/images/minion/static.png';
    }
    
    return result;
  }
} 