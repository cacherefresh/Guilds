import 'package:flutter/material.dart';

/// Widget to display text based on the current terminology setting
/// Will be enhanced to use LanguageProvider when implemented
class TermText extends StatelessWidget {
  /// The term key to look up
  final String termKey;
  
  /// Optional text style
  final TextStyle? style;
  
  /// Constructor
  const TermText(this.termKey, {Key? key, this.style}) : super(key: key);
  
  @override
  Widget build(BuildContext context) {
    // TODO: Implement language provider integration
    // For now, just return the term key with some basic formatting
    String displayText = _formatTermKey(termKey);
    
    return Text(displayText, style: style);
  }
  
  /// Format a term key for display (temporary until language provider is implemented)
  String _formatTermKey(String key) {
    // Split by underscore and capitalize each word
    final words = key.split('_');
    final formattedWords = words.map((word) {
      if (word.isEmpty) return '';
      return word[0].toUpperCase() + word.substring(1);
    });
    
    return formattedWords.join(' ');
  }
} 