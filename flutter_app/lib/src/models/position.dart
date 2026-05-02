import 'dart:math';

/// Represents a 2D position in the game world
class Position {
  /// X coordinate
  final double x;
  
  /// Y coordinate
  final double y;
  
  /// Constructor
  const Position({
    required this.x,
    required this.y,
  });
  
  /// Create a new position with updated values
  Position copyWith({
    double? x,
    double? y,
  }) {
    return Position(
      x: x ?? this.x,
      y: y ?? this.y,
    );
  }
  
  /// Calculate distance to another position
  double distanceTo(Position other) {
    final dx = x - other.x;
    final dy = y - other.y;
    return sqrt(dx * dx + dy * dy);
  }
  
  @override
  String toString() => 'Position(x: $x, y: $y)';
} 