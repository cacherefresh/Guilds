/// Represents a single frame in an animation sequence
class Frame {
  /// Path to the image asset
  final String image;
  
  /// Duration to display this frame in milliseconds
  /// Value of -1 means hold indefinitely
  final int duration;
  
  /// Constructor
  const Frame({
    required this.image,
    required this.duration,
  });
} 