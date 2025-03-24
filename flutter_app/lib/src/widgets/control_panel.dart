import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../models/character_model.dart';

class ControlPanel extends StatelessWidget {
  const ControlPanel({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final characterModel = Provider.of<CharacterModel>(context);
    
    return Card(
      margin: const EdgeInsets.all(8),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Actions',
              style: Theme.of(context).textTheme.headlineSmall,
            ),
            const SizedBox(height: 16),
            Expanded(
              child: GridView.count(
                crossAxisCount: 2,
                mainAxisSpacing: 8,
                crossAxisSpacing: 8,
                childAspectRatio: 2.5,
                children: [
                  _buildActionButton(
                    context,
                    icon: Icons.person,
                    label: 'Idle',
                    onPressed: () => characterModel.updateAction('idle'),
                    isActive: characterModel.currentAction == 'idle',
                  ),
                  _buildActionButton(
                    context,
                    icon: Icons.directions_walk,
                    label: 'Walk',
                    onPressed: () => characterModel.updateAction('walking'),
                    isActive: characterModel.currentAction == 'walking',
                  ),
                  _buildActionButton(
                    context,
                    icon: Icons.psychology,
                    label: 'Think',
                    onPressed: () => characterModel.think(),
                    isActive: characterModel.currentAction == 'thinking',
                  ),
                  _buildActionButton(
                    context,
                    icon: Icons.back_hand,
                    label: 'Point',
                    onPressed: () => characterModel.updateAction('pointing'),
                    isActive: characterModel.currentAction == 'pointing',
                  ),
                  _buildActionButton(
                    context,
                    icon: Icons.content_copy,
                    label: 'Clone',
                    onPressed: () => characterModel.summonShadowClone(),
                    isActive: false,
                  ),
                  _buildActionButton(
                    context,
                    icon: Icons.assignment,
                    label: 'Minion',
                    onPressed: () => _showSummonMinionDialog(context),
                    isActive: false,
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
  
  Widget _buildActionButton(
    BuildContext context, {
    required IconData icon,
    required String label,
    required VoidCallback onPressed,
    required bool isActive,
  }) {
    return ElevatedButton.icon(
      onPressed: onPressed,
      icon: Icon(icon),
      label: Text(label),
      style: ElevatedButton.styleFrom(
        backgroundColor: isActive
            ? Theme.of(context).colorScheme.secondary
            : Theme.of(context).colorScheme.primary,
        foregroundColor: Colors.white,
      ),
    );
  }
  
  void _showSummonMinionDialog(BuildContext context) {
    final formKey = GlobalKey<FormState>();
    String taskDescription = '';
    String category = 'General';
    
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Summon Shadow Minion'),
        content: Form(
          key: formKey,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              TextFormField(
                decoration: const InputDecoration(
                  labelText: 'Task Description',
                  border: OutlineInputBorder(),
                ),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a task description';
                  }
                  return null;
                },
                onSaved: (value) {
                  taskDescription = value ?? '';
                },
              ),
              const SizedBox(height: 16),
              DropdownButtonFormField<String>(
                value: category,
                decoration: const InputDecoration(
                  labelText: 'Category',
                  border: OutlineInputBorder(),
                ),
                items: ['General', 'Coding', 'Design', 'Research', 'Other']
                    .map((c) => DropdownMenuItem(
                          value: c,
                          child: Text(c),
                        ))
                    .toList(),
                onChanged: (value) {
                  if (value != null) {
                    category = value;
                  }
                },
              ),
            ],
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () {
              if (formKey.currentState!.validate()) {
                formKey.currentState!.save();
                
                // Summon the minion
                final characterModel = Provider.of<CharacterModel>(
                  context,
                  listen: false,
                );
                characterModel.summonShadowMinion(taskDescription, category);
                
                // Show feedback
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(
                    content: Text('Shadow Minion summoned!'),
                    duration: Duration(seconds: 2),
                  ),
                );
                
                Navigator.of(context).pop();
              }
            },
            child: const Text('Summon'),
          ),
        ],
      ),
    );
  }
} 