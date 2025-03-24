import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../models/character_model.dart';
import '../widgets/character_view.dart';
import '../widgets/control_panel.dart';
import '../widgets/quest_board.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final characterModel = Provider.of<CharacterModel>(context);
    
    return Scaffold(
      appBar: AppBar(
        title: Text('${characterModel.guildName} - Guild Room'),
        actions: [
          IconButton(
            icon: const Icon(Icons.settings),
            onPressed: () {
              // Navigate to settings
            },
          ),
        ],
      ),
      body: Row(
        children: [
          // 3D View (Main Area) - Takes 70% of screen width
          Expanded(
            flex: 7,
            child: Container(
              color: Theme.of(context).colorScheme.background,
              child: const CharacterView(),
            ),
          ),
          
          // Side Panel - Takes 30% of screen width
          Expanded(
            flex: 3,
            child: Container(
              color: Theme.of(context).colorScheme.surface,
              child: Column(
                children: [
                  // Character Info Panel
                  Container(
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          characterModel.name,
                          style: Theme.of(context).textTheme.headlineMedium,
                        ),
                        const SizedBox(height: 8),
                        Text(
                          'Guild: ${characterModel.guildName}',
                          style: Theme.of(context).textTheme.bodyLarge,
                        ),
                        const Divider(),
                        Text(
                          'Interests:',
                          style: Theme.of(context).textTheme.bodyLarge,
                        ),
                        ...characterModel.interests.map((interest) => 
                          Text('• $interest', style: Theme.of(context).textTheme.bodyMedium)
                        ),
                      ],
                    ),
                  ),
                  
                  // Control Panel for Character Actions
                  const Expanded(
                    child: ControlPanel(),
                  ),
                  
                  // Quest Board Panel
                  const Expanded(
                    flex: 2,
                    child: QuestBoard(),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          // Show dialog to add new quest
          showDialog(
            context: context,
            builder: (_) => const AddQuestDialog(),
          );
        },
        child: const Icon(Icons.add),
        tooltip: 'Add New Quest',
      ),
    );
  }
}

class AddQuestDialog extends StatefulWidget {
  const AddQuestDialog({Key? key}) : super(key: key);

  @override
  State<AddQuestDialog> createState() => _AddQuestDialogState();
}

class _AddQuestDialogState extends State<AddQuestDialog> {
  final _formKey = GlobalKey<FormState>();
  final _titleController = TextEditingController();
  final _descriptionController = TextEditingController();
  String _category = 'General';
  double _reward = 0.0;
  
  @override
  void dispose() {
    _titleController.dispose();
    _descriptionController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Add New Quest'),
      content: Form(
        key: _formKey,
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              TextFormField(
                controller: _titleController,
                decoration: const InputDecoration(
                  labelText: 'Title',
                  border: OutlineInputBorder(),
                ),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a title';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 16),
              TextFormField(
                controller: _descriptionController,
                decoration: const InputDecoration(
                  labelText: 'Description',
                  border: OutlineInputBorder(),
                ),
                maxLines: 3,
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a description';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 16),
              DropdownButtonFormField<String>(
                value: _category,
                decoration: const InputDecoration(
                  labelText: 'Category',
                  border: OutlineInputBorder(),
                ),
                items: ['General', 'Coding', 'Design', 'Research', 'Other']
                    .map((category) => DropdownMenuItem(
                          value: category,
                          child: Text(category),
                        ))
                    .toList(),
                onChanged: (value) {
                  if (value != null) {
                    setState(() {
                      _category = value;
                    });
                  }
                },
              ),
              const SizedBox(height: 16),
              TextFormField(
                decoration: const InputDecoration(
                  labelText: 'Reward',
                  border: OutlineInputBorder(),
                  suffixText: 'points',
                ),
                keyboardType: TextInputType.number,
                onChanged: (value) {
                  if (value.isNotEmpty) {
                    setState(() {
                      _reward = double.tryParse(value) ?? 0.0;
                    });
                  }
                },
              ),
            ],
          ),
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () {
            if (_formKey.currentState!.validate()) {
              // Add quest logic
              Navigator.of(context).pop();
            }
          },
          child: const Text('Add Quest'),
        ),
      ],
    );
  }
} 