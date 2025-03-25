import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../providers/animation_settings_provider.dart';
import '../services/preferences_service.dart';
import '../widgets/term_text.dart';

/// Screen to manage app settings
class SettingsScreen extends StatelessWidget {
  /// Constructor
  const SettingsScreen({Key? key}) : super(key: key);
  
  @override
  Widget build(BuildContext context) {
    final prefsService = PreferencesService();
    
    return Scaffold(
      appBar: AppBar(
        title: const TermText('settings_title'),
      ),
      body: ListView(
        children: [
          // Animation toggle
          SwitchListTile(
            title: const TermText('enable_animations'),
            subtitle: const TermText('animations_description'),
            value: Provider.of<AnimationSettings>(context).animationsEnabled,
            onChanged: (value) {
              Provider.of<AnimationSettings>(context, listen: false)
                  .toggleAnimations();
            },
          ),
          
          // Language toggle (if not in standalone mode)
          if (!prefsService.isStandaloneMode())
            SwitchListTile(
              title: const TermText('use_game_terminology'),
              subtitle: const TermText('terminology_description'),
              value: true, // This will be connected to a language provider
              onChanged: (value) {
                // This will toggle language when implemented
              },
            ),
            
          // Other settings go here
          const Divider(),
          
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: Text(
              'Performance Settings',
              style: Theme.of(context).textTheme.titleLarge,
            ),
          ),
          
          // No-animation mode explanation
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16.0),
            child: Text(
              'No-Animation Mode is enabled by default for best performance. '
              'This mode skips animations but still shows character states '
              'and positions them correctly in the world.',
              style: Theme.of(context).textTheme.bodyMedium,
            ),
          ),
          
          const SizedBox(height: 16),
          
          // Golden flicker explanation
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16.0),
            child: Text(
              'When a character or minion changes state in No-Animation Mode, '
              'it will briefly flicker with a golden highlight to indicate '
              'the change.',
              style: Theme.of(context).textTheme.bodyMedium,
            ),
          ),
        ],
      ),
    );
  }
} 