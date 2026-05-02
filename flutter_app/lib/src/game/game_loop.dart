import 'dart:async';

/// Game loop to update game state at regular intervals
class GameLoop {
  /// Timer for the game loop
  Timer? _timer;
  
  /// Last frame time in milliseconds
  int _lastFrameTime = 0;
  
  /// List of callbacks to call on each update
  final List<Function(int)> _updateCallbacks = [];
  
  /// Start the game loop
  void start() {
    _lastFrameTime = DateTime.now().millisecondsSinceEpoch;
    
    _timer = Timer.periodic(const Duration(milliseconds: 16), (timer) {
      final currentTime = DateTime.now().millisecondsSinceEpoch;
      final deltaTime = currentTime - _lastFrameTime;
      _lastFrameTime = currentTime;
      
      // Call all update callbacks with delta time
      for (final callback in _updateCallbacks) {
        callback(deltaTime);
      }
    });
  }
  
  /// Register a callback to be called on each update
  void registerUpdateCallback(Function(int) callback) {
    _updateCallbacks.add(callback);
  }
  
  /// Stop the game loop
  void stop() {
    _timer?.cancel();
    _timer = null;
  }
  
  /// Check if the game loop is running
  bool get isRunning => _timer != null;
} 