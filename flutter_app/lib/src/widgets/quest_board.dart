import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../models/character_model.dart';

class QuestBoard extends StatefulWidget {
  const QuestBoard({Key? key}) : super(key: key);

  @override
  State<QuestBoard> createState() => _QuestBoardState();
}

class _QuestBoardState extends State<QuestBoard> {
  // Controller for adding new quests
  final TextEditingController _titleController = TextEditingController();
  final TextEditingController _descriptionController = TextEditingController();
  String _selectedCategory = 'Guild';
  String? _selectedAdventureId;

  // Controllers for creating new adventures
  final TextEditingController _adventureNameController = TextEditingController();
  final TextEditingController _adventureDescController = TextEditingController();

  // Currently selected adventure for filtering quests
  String? _filterAdventureId;

  // List of available categories
  final List<String> _categories = [
    'Guild',
    'Combat',
    'Crafting',
    'Exploration',
    'Tutorial',
    'Other',
  ];

  // External reference to _gameState - set by parent
  Map<String, dynamic>? _gameState;
  
  // External reference to summoned characters - set by parent
  List<Map<String, dynamic>> _summonedCharacters = [];

  // Get quests from game state
  List<Map<String, dynamic>> get _quests {
    if (_gameState == null || !_gameState!.containsKey('quests')) {
      return [];
    }
    
    return List<Map<String, dynamic>>.from(_gameState!['quests']);
  }

  // Set quests in game state
  set _quests(List<Map<String, dynamic>> quests) {
    if (_gameState != null) {
      _gameState!['quests'] = quests;
    }
  }

  // Get adventures from game state
  List<Map<String, dynamic>> get _adventures {
    if (_gameState == null || !_gameState!.containsKey('adventures')) {
      return [];
    }
    
    return List<Map<String, dynamic>>.from(_gameState!['adventures']);
  }

  // Set adventures in game state
  set _adventures(List<Map<String, dynamic>> adventures) {
    if (_gameState != null) {
      _gameState!['adventures'] = adventures;
    }
  }

  // Get filtered quests based on selected adventure
  List<Map<String, dynamic>> get _filteredQuests {
    if (_filterAdventureId == null) {
      return _quests;
    }
    
    return _quests.where((quest) => 
      quest['adventureId'] == _filterAdventureId
    ).toList();
  }

  @override
  void initState() {
    super.initState();

    // Default selected adventure to the first one if available
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (_adventures.isNotEmpty && _selectedAdventureId == null) {
        setState(() {
          _selectedAdventureId = _adventures.first['id'];
          _filterAdventureId = _adventures.first['id'];
        });
      }
    });
  }

  // Add a new quest
  void _addQuest() {
    if (_titleController.text.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Quest title cannot be empty'),
          duration: Duration(seconds: 2),
        ),
      );
      return;
    }

    if (_selectedAdventureId == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Please select an adventure for this quest'),
          duration: Duration(seconds: 2),
        ),
      );
      return;
    }

    final newQuest = {
      'id': DateTime.now().millisecondsSinceEpoch.toString(),
      'title': _titleController.text,
      'description': _descriptionController.text.isEmpty 
          ? 'No description provided' 
          : _descriptionController.text,
      'status': 'Active',
      'category': _selectedCategory,
      'adventureId': _selectedAdventureId,
      'assignedTo': null,
    };

    setState(() {
      _quests = [..._quests, newQuest];
      _titleController.clear();
      _descriptionController.clear();
    });

    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
        content: Text('New quest added!'),
        duration: Duration(seconds: 2),
      ),
    );
  }

  // Add a new adventure
  void _addAdventure() {
    if (_adventureNameController.text.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Adventure name cannot be empty'),
          duration: Duration(seconds: 2),
        ),
      );
      return;
    }

    final newAdventure = {
      'id': DateTime.now().millisecondsSinceEpoch.toString(),
      'name': _adventureNameController.text,
      'description': _adventureDescController.text.isEmpty 
          ? 'No description provided' 
          : _adventureDescController.text,
      'status': 'Active',
      'created': DateTime.now().millisecondsSinceEpoch,
    };

    setState(() {
      _adventures = [..._adventures, newAdventure];
      _adventureNameController.clear();
      _adventureDescController.clear();
      
      // Select the newly created adventure
      _selectedAdventureId = newAdventure['id'];
      _filterAdventureId = newAdventure['id'];
    });

    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
        content: Text('New adventure created!'),
        duration: Duration(seconds: 2),
      ),
    );

    // Close the dialog
    Navigator.of(context).pop();
  }

  // Update quest status
  void _updateQuestStatus(String questId, String newStatus) {
    setState(() {
      final questList = _quests;
      final questIndex = questList.indexWhere((q) => q['id'] == questId);
      if (questIndex >= 0) {
        questList[questIndex]['status'] = newStatus;
        _quests = questList;
      }
    });
  }

  // Delete a quest
  void _deleteQuest(String questId) {
    setState(() {
      final questList = _quests;
      questList.removeWhere((q) => q['id'] == questId);
      _quests = questList;
    });

    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
        content: Text('Quest deleted'),
        duration: Duration(seconds: 2),
      ),
    );
  }

  // Delete an adventure and all its quests
  void _deleteAdventure(String adventureId) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Adventure?'),
        content: const Text(
          'This will delete the adventure and all associated quests. This action cannot be undone!',
          style: TextStyle(color: Colors.red),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              setState(() {
                // Remove the adventure
                final adventureList = _adventures;
                adventureList.removeWhere((a) => a['id'] == adventureId);
                _adventures = adventureList;
                
                // Remove all quests associated with this adventure
                final questList = _quests;
                questList.removeWhere((q) => q['adventureId'] == adventureId);
                _quests = questList;
                
                // Reset filters
                if (_filterAdventureId == adventureId) {
                  _filterAdventureId = _adventures.isNotEmpty ? _adventures.first['id'] : null;
                }
                
                if (_selectedAdventureId == adventureId) {
                  _selectedAdventureId = _adventures.isNotEmpty ? _adventures.first['id'] : null;
                }
              });
              
              Navigator.of(context).pop();
              
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(
                  content: Text('Adventure and all associated quests deleted'),
                  duration: Duration(seconds: 2),
                ),
              );
            },
            style: TextButton.styleFrom(foregroundColor: Colors.red),
            child: const Text('Delete'),
          ),
        ],
      ),
    );
  }

  // Assign quest to a summon
  void _assignQuestToSummon(String questId, String summonName) {
    setState(() {
      final questList = _quests;
      final questIndex = questList.indexWhere((q) => q['id'] == questId);
      if (questIndex >= 0) {
        questList[questIndex]['assignedTo'] = summonName;
        _quests = questList;
      }
    });

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('Quest assigned to $summonName'),
        duration: const Duration(seconds: 2),
      ),
    );
  }

  // Show dialog to create a new adventure
  void _showCreateAdventureDialog() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Create New Adventure'),
        content: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              TextField(
                controller: _adventureNameController,
                decoration: const InputDecoration(
                  labelText: 'Adventure Name',
                  border: OutlineInputBorder(),
                  prefixIcon: Icon(Icons.explore),
                ),
              ),
              const SizedBox(height: 16),
              TextField(
                controller: _adventureDescController,
                decoration: const InputDecoration(
                  labelText: 'Description',
                  border: OutlineInputBorder(),
                  prefixIcon: Icon(Icons.description),
                  alignLabelWithHint: true,
                ),
                maxLines: 3,
              ),
            ],
          ),
        ),
        actions: [
          TextButton(
            onPressed: () {
              _adventureNameController.clear();
              _adventureDescController.clear();
              Navigator.of(context).pop();
            },
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: _addAdventure,
            style: ElevatedButton.styleFrom(
              backgroundColor: Colors.purple,
              foregroundColor: Colors.white,
            ),
            child: const Text('Create Adventure'),
          ),
        ],
      ),
    );
  }

  @override
  void dispose() {
    _titleController.dispose();
    _descriptionController.dispose();
    _adventureNameController.dispose();
    _adventureDescController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 3,
      child: Column(
        children: [
          const TabBar(
            tabs: [
              Tab(text: 'View Quests'),
              Tab(text: 'Add Quest'),
              Tab(text: 'Adventures'),
            ],
            labelColor: Colors.purple,
          ),
          Expanded(
            child: TabBarView(
              children: [
                // View Quests tab
                _buildQuestsListTab(_summonedCharacters),
                
                // Add Quest tab
                _buildAddQuestTab(),
                
                // Adventures tab
                _buildAdventuresTab(),
              ],
            ),
          ),
        ],
      ),
    );
  }

  // Build the quests list tab
  Widget _buildQuestsListTab(List<Map<String, dynamic>> summonedCharacters) {
    final filteredQuests = _filteredQuests;

    return Column(
      children: [
        // Adventure filter dropdown
        if (_adventures.isNotEmpty)
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Row(
              children: [
                const Text('Adventure: ', style: TextStyle(fontWeight: FontWeight.bold)),
                const SizedBox(width: 8),
                Expanded(
                  child: DropdownButtonFormField<String>(
                    value: _filterAdventureId,
                    decoration: const InputDecoration(
                      border: OutlineInputBorder(),
                      contentPadding: EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                    ),
                    items: [
                      const DropdownMenuItem<String>(
                        value: null,
                        child: Text('All Adventures'),
                      ),
                      ..._adventures.map((adventure) => DropdownMenuItem<String>(
                        value: adventure['id'],
                        child: Text(adventure['name']),
                      )),
                    ],
                    onChanged: (value) {
                      setState(() {
                        _filterAdventureId = value;
                      });
                    },
                  ),
                ),
              ],
            ),
          ),
          
        // Quest list
        Expanded(
          child: filteredQuests.isEmpty
              ? const Center(
                  child: Text(
                    'No quests available. Add some quests from the "Add Quest" tab.',
                    textAlign: TextAlign.center,
                  ),
                )
              : ListView.builder(
                  itemCount: filteredQuests.length,
                  itemBuilder: (context, index) {
                    final quest = filteredQuests[index];
                    final Color statusColor = _getStatusColor(quest['status'] as String);
                    
                    // Find the adventure this quest belongs to
                    final adventure = _adventures.firstWhere(
                      (a) => a['id'] == quest['adventureId'],
                      orElse: () => {'name': 'Unknown Adventure'},
                    );

                    return Card(
                      margin: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                      child: ExpansionTile(
                        title: Text(quest['title'] as String),
                        subtitle: Text('${quest['category']} - ${quest['status']} (${adventure['name']})'),
                        leading: CircleAvatar(
                          backgroundColor: statusColor,
                          child: const Icon(Icons.assignment, color: Colors.white),
                        ),
                        trailing: quest['assignedTo'] != null
                            ? Chip(
                                label: Text('Assigned to: ${quest['assignedTo']}'),
                                backgroundColor: Colors.purple.withOpacity(0.2),
                              )
                            : null,
                        children: [
                          Padding(
                            padding: const EdgeInsets.all(16.0),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text(
                                  quest['description'] as String,
                                  style: const TextStyle(fontSize: 14),
                                ),
                                const SizedBox(height: 16),
                                Row(
                                  mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                                  children: [
                                    _buildStatusButton('Active', quest, statusColor),
                                    _buildStatusButton('In Progress', quest, statusColor),
                                    _buildStatusButton('Complete', quest, statusColor),
                                  ],
                                ),
                                const SizedBox(height: 16),
                                
                                // Summon assignment section
                                if (summonedCharacters.isNotEmpty)
                                  Column(
                                    crossAxisAlignment: CrossAxisAlignment.start,
                                    children: [
                                      const Text(
                                        'Assign to summon:',
                                        style: TextStyle(fontWeight: FontWeight.bold),
                                      ),
                                      const SizedBox(height: 8),
                                      SizedBox(
                                        height: 50,
                                        child: ListView.builder(
                                          scrollDirection: Axis.horizontal,
                                          itemCount: summonedCharacters.length,
                                          itemBuilder: (context, idx) {
                                            final summon = summonedCharacters[idx];
                                            return Padding(
                                              padding: const EdgeInsets.only(right: 8.0),
                                              child: ActionChip(
                                                avatar: Icon(
                                                  summon['type'] == 'Shadow Clone' 
                                                      ? Icons.person_outline 
                                                      : Icons.pets,
                                                  color: Colors.white,
                                                  size: 16,
                                                ),
                                                backgroundColor: Colors.deepPurple,
                                                label: Text(
                                                  summon['name'] as String,
                                                  style: const TextStyle(color: Colors.white),
                                                ),
                                                onPressed: () {
                                                  _assignQuestToSummon(
                                                    quest['id'] as String,
                                                    summon['name'] as String,
                                                  );
                                                },
                                              ),
                                            );
                                          },
                                        ),
                                      ),
                                    ],
                                  ),
                                
                                const SizedBox(height: 16),
                                Align(
                                  alignment: Alignment.centerRight,
                                  child: TextButton.icon(
                                    icon: const Icon(Icons.delete, color: Colors.red),
                                    label: const Text(
                                      'Delete Quest',
                                      style: TextStyle(color: Colors.red),
                                    ),
                                    onPressed: () => _deleteQuest(quest['id'] as String),
                                  ),
                                ),
                              ],
                            ),
                          ),
                        ],
                      ),
                    );
                  },
                ),
        ),
      ],
    );
  }

  // Build buttons for changing quest status
  Widget _buildStatusButton(String status, Map<String, dynamic> quest, Color currentColor) {
    final isCurrentStatus = quest['status'] == status;
    final Color buttonColor = _getStatusColor(status);
    
    return ElevatedButton(
      style: ElevatedButton.styleFrom(
        backgroundColor: isCurrentStatus ? buttonColor : Colors.grey.shade200,
        foregroundColor: isCurrentStatus ? Colors.white : Colors.black87,
      ),
      onPressed: isCurrentStatus 
          ? null 
          : () => _updateQuestStatus(quest['id'] as String, status),
      child: Text(status),
    );
  }

  // Get color for quest status
  Color _getStatusColor(String status) {
    switch (status) {
      case 'Active':
        return Colors.blue;
      case 'In Progress':
        return Colors.orange;
      case 'Complete':
        return Colors.green;
      default:
        return Colors.grey;
    }
  }

  // Build the add quest tab
  Widget _buildAddQuestTab() {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text(
                'Create a new quest',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                ),
              ),
              TextButton.icon(
                onPressed: _showCreateAdventureDialog,
                icon: const Icon(Icons.add),
                label: const Text('New Adventure'),
              ),
            ],
          ),
          const SizedBox(height: 16),
          TextField(
            controller: _titleController,
            decoration: const InputDecoration(
              labelText: 'Quest Title',
              border: OutlineInputBorder(),
              prefixIcon: Icon(Icons.title),
            ),
            maxLength: 50,
          ),
          const SizedBox(height: 16),
          TextField(
            controller: _descriptionController,
            decoration: const InputDecoration(
              labelText: 'Quest Description',
              border: OutlineInputBorder(),
              prefixIcon: Icon(Icons.description),
              alignLabelWithHint: true,
            ),
            maxLines: 3,
            maxLength: 200,
          ),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(
                child: DropdownButtonFormField<String>(
                  decoration: const InputDecoration(
                    labelText: 'Category',
                    border: OutlineInputBorder(),
                    prefixIcon: Icon(Icons.category),
                  ),
                  value: _selectedCategory,
                  items: _categories.map((category) {
                    return DropdownMenuItem<String>(
                      value: category,
                      child: Text(category),
                    );
                  }).toList(),
                  onChanged: (value) {
                    if (value != null) {
                      setState(() {
                        _selectedCategory = value;
                      });
                    }
                  },
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: DropdownButtonFormField<String>(
                  decoration: const InputDecoration(
                    labelText: 'Adventure',
                    border: OutlineInputBorder(),
                    prefixIcon: Icon(Icons.explore),
                  ),
                  value: _selectedAdventureId,
                  items: _adventures.map((adventure) {
                    return DropdownMenuItem<String>(
                      value: adventure['id'],
                      child: Text(adventure['name']),
                    );
                  }).toList(),
                  onChanged: (value) {
                    setState(() {
                      _selectedAdventureId = value;
                    });
                  },
                ),
              ),
            ],
          ),
          const SizedBox(height: 32),
          SizedBox(
            width: double.infinity,
            height: 50,
            child: ElevatedButton.icon(
              icon: const Icon(Icons.add),
              label: const Text('Add Quest'),
              style: ElevatedButton.styleFrom(
                backgroundColor: Colors.purple,
                foregroundColor: Colors.white,
              ),
              onPressed: _addQuest,
            ),
          ),
        ],
      ),
    );
  }

  // Build the adventures tab
  Widget _buildAdventuresTab() {
    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.all(8.0),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              const Text(
                'Adventures',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                ),
              ),
              ElevatedButton.icon(
                onPressed: _showCreateAdventureDialog,
                icon: const Icon(Icons.add),
                label: const Text('New Adventure'),
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.purple,
                  foregroundColor: Colors.white,
                ),
              ),
            ],
          ),
        ),
        Expanded(
          child: _adventures.isEmpty
              ? const Center(
                  child: Text(
                    'No adventures available. Create a new adventure to begin organizing your quests.',
                    textAlign: TextAlign.center,
                  ),
                )
              : ListView.builder(
                  itemCount: _adventures.length,
                  itemBuilder: (context, index) {
                    final adventure = _adventures[index];
                    final questCount = _quests.where((q) => q['adventureId'] == adventure['id']).length;
                    final completedCount = _quests.where(
                      (q) => q['adventureId'] == adventure['id'] && q['status'] == 'Complete'
                    ).length;
                    
                    return Card(
                      margin: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                      child: ListTile(
                        title: Text(adventure['name']),
                        subtitle: Text(
                          '${adventure['description']}\nQuests: $completedCount/$questCount complete'
                        ),
                        leading: CircleAvatar(
                          backgroundColor: Colors.deepPurple,
                          child: Text(
                            adventure['name'].substring(0, 1).toUpperCase(),
                            style: const TextStyle(color: Colors.white),
                          ),
                        ),
                        trailing: Row(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            IconButton(
                              icon: const Icon(Icons.filter_list),
                              tooltip: 'View Quests',
                              onPressed: () {
                                setState(() {
                                  _filterAdventureId = adventure['id'];
                                });
                                // Switch to quests tab
                                DefaultTabController.of(context).animateTo(0);
                              },
                            ),
                            IconButton(
                              icon: const Icon(Icons.delete, color: Colors.red),
                              tooltip: 'Delete Adventure',
                              onPressed: () => _deleteAdventure(adventure['id']),
                            ),
                          ],
                        ),
                        isThreeLine: true,
                      ),
                    );
                  },
                ),
        ),
      ],
    );
  }
} 