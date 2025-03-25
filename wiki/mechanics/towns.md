# Towns System

Towns provide the geographic and economic framework for guilds and quests in the game. Each town has unique characteristics, resources, and quest opportunities that make it an attractive location for guilds to establish themselves.

## Town Features

- **Unique Identity**: Each town has a name, description, and regional location.
- **Specializations**: Towns have different specializations (trading, crafting, magic, etc.).
- **Guild Hosting**: Towns can host multiple guilds, creating a competitive marketplace.
- **Quest Hub**: Towns generate and offer quests based on their needs and specializations.
- **Regional Influences**: Towns in the same region share certain characteristics and quest types.

## Town Geography

- **Regions**: Towns are grouped into geographic regions (e.g., Sword Coast, Silver Marches).
- **Proximity Effects**: Nearby towns may have related quests or shared resources.
- **Town Tiers**: Towns come in different sizes and importance levels, from small villages to major cities.

## Guild Relationships with Towns

- **Home Base**: Each guild designates a town as its base of operations.
- **Local Reputation**: Guilds build reputation with specific towns through quest completion.
- **Town Benefits**: Guilds may receive benefits based on their standing with a town.
- **Inter-Town Relations**: A guild's reputation in one town may affect its standing in others.

## Town Search and Discovery

- **Regional Search**: Players can search for towns in specific regions.
- **Guild-Based Search**: Players can find towns based on the guilds present there.
- **Quest-Based Search**: Players can locate towns offering specific types of quests.

## Technical Implementation

### Database Structure

The town system uses the following database components:

- `towns`: Stores town information, including name, description, region, and properties.
- `guilds`: Contains a reference to the town where each guild is based.
- `town_quests` (implicit): Quests are associated with towns through quest generation logic.

### Properties System

Towns have a flexible properties system implemented as a JSONB field that can store:

- Population
- Government type
- Local economy specialization
- Notable landmarks or features
- Special resources

### API Endpoints

The town system exposes several RESTful endpoints:

- Town management: Create, update, delete, and retrieve towns.
- Town search: Find towns by region, name, or properties.
- Town-guild relationships: Find guilds in a specific town.

## Sample Towns

The system comes pre-populated with several towns:

1. **Waterdeep**
   - Region: Sword Coast
   - Known for: Commerce and adventuring opportunities
   - Population: 100,000
   - Government: Council
   
2. **Neverwinter**
   - Region: Sword Coast
   - Known for: Skilled craftsmen
   - Population: 50,000
   - Government: Lord
   
3. **Baldur's Gate**
   - Region: Sword Coast
   - Known for: Strong military and trade connections
   - Population: 80,000
   - Government: Council
   
4. **Silverymoon**
   - Region: Silver Marches
   - Known for: Magic institutions and spellcasters
   - Population: 40,000
   - Government: High Mage
   
5. **Mirabar**
   - Region: Silver Marches
   - Known for: Mining precious metals and gems
   - Population: 35,000
   - Government: Marchion

## Example Usage

1. **Retrieving Town Information**:
   ```
   GET /api/towns/550e8400-e29b-41d4-a716-446655440001
   ```

2. **Finding Towns in a Region**:
   ```
   GET /api/towns/region/Sword%20Coast
   ```

3. **Searching Towns by Name or Features**:
   ```
   GET /api/towns/search?query=mining
   ```

4. **Finding Guilds in a Town**:
   ```
   GET /api/guilds?town_id=550e8400-e29b-41d4-a716-446655440001
   ```

5. **Creating a New Town**:
   ```json
   POST /api/towns
   {
     "name": "Luskan",
     "description": "A rough harbor town known for its pirates and illicit trade",
     "region": "Sword Coast",
     "properties": {
       "population": 15000,
       "specialty": "shipping",
       "government": "captains' council"
     }
   }
   ``` 