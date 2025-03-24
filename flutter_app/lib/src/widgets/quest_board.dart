import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../models/character_model.dart';
import '../models/minion_model.dart';

class QuestBoard extends StatelessWidget {
  const QuestBoard({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final characterModel = Provider.of<CharacterModel>(context);
    
    // Sample quests - in a real app, these would come from an API
    final List<Map<String, dynamic>> sampleQuests = [
      {
        'id': '1',
        'title': 'Create 3D Model',
        'description': 'Design a 3D character model with animations',
        'category': 'Design',
        'requestor': 'System',
        'reward': 100.0,
      },
      {
        'id': '2',
        'title': 'Fix Login Bug',
        'description': 'Fix authentication issue in login screen',
        'category': 'Coding',
        'requestor': 'System',
        'reward': 75.0,
      },
      {
        'id': '3',
        'title': 'Research New Framework',
        'description': 'Evaluate pros and cons of new framework',
        'category': 'Research',
        'requestor': 'System',
        'reward': 50.0,
      },
    ];
    
    return Card(
      margin: const EdgeInsets.all(8),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  'Quest Board',
                  style: Theme.of(context).textTheme.headlineSmall,
                ),
                TextButton.icon(
                  icon: const Icon(Icons.refresh),
                  label: const Text('Refresh'),
                  onPressed: () {
                    // Refresh quests from API
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(
                        content: Text('Refreshing quests...'),
                        duration: Duration(seconds: 1),
                      ),
                    );
                  },
                ),
              ],
            ),
            const SizedBox(height: 8),
            Expanded(
              child: ListView.builder(
                itemCount: sampleQuests.length,
                itemBuilder: (context, index) {
                  final quest = sampleQuests[index];
                  return _buildQuestCard(context, quest, characterModel);
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
  
  Widget _buildQuestCard(
    BuildContext context,
    Map<String, dynamic> quest,
    CharacterModel characterModel,
  ) {
    // Determine category color
    Color categoryColor;
    switch (quest['category']) {
      case 'Design':
        categoryColor = Colors.purple;
        break;
      case 'Coding':
        categoryColor = Colors.green;
        break;
      case 'Research':
        categoryColor = Colors.blue;
        break;
      default:
        categoryColor = Colors.grey;
        break;
    }
    
    return Card(
      margin: const EdgeInsets.symmetric(vertical: 4),
      child: ExpansionTile(
        leading: CircleAvatar(
          backgroundColor: categoryColor,
          child: Text(
            quest['category'][0], // First letter of category
            style: const TextStyle(color: Colors.white),
          ),
        ),
        title: Text(quest['title']),
        subtitle: Text(
          'Reward: ${quest['reward']} points • Requestor: ${quest['requestor']}',
          style: Theme.of(context).textTheme.bodySmall,
        ),
        children: [
          Padding(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  'Description:',
                  style: Theme.of(context).textTheme.titleMedium,
                ),
                const SizedBox(height: 8),
                Text(quest['description']),
                const SizedBox(height: 16),
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    OutlinedButton.icon(
                      icon: const Icon(Icons.visibility),
                      label: const Text('View Details'),
                      onPressed: () {
                        // Show quest details
                      },
                    ),
                    const SizedBox(width: 8),
                    ElevatedButton.icon(
                      icon: const Icon(Icons.assignment_turned_in),
                      label: const Text('Accept Quest'),
                      onPressed: () {
                        // Accept the quest
                        _acceptQuest(context, quest, characterModel);
                      },
                    ),
                  ],
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
  
  void _acceptQuest(
    BuildContext context,
    Map<String, dynamic> quest,
    CharacterModel characterModel,
  ) {
    // Create a new minion for this quest
    characterModel.summonShadowMinion(
      quest['description'],
      quest['category'],
    );
    
    // Show confirmation
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('Quest "${quest['title']}" accepted!'),
        duration: const Duration(seconds: 2),
      ),
    );
  }
} 