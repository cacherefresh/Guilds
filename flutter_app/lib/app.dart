import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'src/models/character_model.dart';
import 'src/screens/home_screen.dart';
import 'src/theme/app_theme.dart';

class App extends StatelessWidget {
  const App({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (_) => CharacterModel()),
      ],
      child: MaterialApp(
        title: 'Guild App',
        theme: AppTheme.themeData,
        home: const HomeScreen(),
      ),
    );
  }
} 