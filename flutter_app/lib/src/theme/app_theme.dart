import 'package:flutter/material.dart';

class AppTheme {
  // Color Constants
  static const Color primaryColor = Color(0xFF6A0DAD); // Purple
  static const Color secondaryColor = Color(0xFF00C853); // Green
  static const Color backgroundColor = Color(0xFF121212); // Dark/Black
  static const Color accentColor = Color(0xFF9C27B0); // Light Purple
  static const Color textColor = Colors.white;
  static const Color secondaryTextColor = Color(0xFFB0BEC5);

  // Theme Data
  static final ThemeData themeData = ThemeData(
    primaryColor: primaryColor,
    scaffoldBackgroundColor: backgroundColor,
    colorScheme: const ColorScheme.dark(
      primary: primaryColor,
      secondary: secondaryColor,
      surface: Color(0xFF1E1E1E),
    ),
    textTheme: const TextTheme(
      headlineLarge: TextStyle(
        color: textColor,
        fontWeight: FontWeight.bold,
      ),
      headlineMedium: TextStyle(
        color: textColor,
        fontWeight: FontWeight.w600,
      ),
      bodyLarge: TextStyle(
        color: textColor,
      ),
      bodyMedium: TextStyle(
        color: secondaryTextColor,
      ),
    ),
    elevatedButtonTheme: ElevatedButtonThemeData(
      style: ElevatedButton.styleFrom(
        backgroundColor: primaryColor,
        foregroundColor: textColor,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(8),
        ),
      ),
    ),
    appBarTheme: const AppBarTheme(
      backgroundColor: backgroundColor,
      elevation: 0,
      iconTheme: IconThemeData(color: textColor),
    ),
    floatingActionButtonTheme: const FloatingActionButtonThemeData(
      backgroundColor: secondaryColor,
      foregroundColor: backgroundColor,
    ),
    cardTheme: CardThemeData(
      color: const Color(0xFF1E1E1E),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
      ),
    ),
  );
} 