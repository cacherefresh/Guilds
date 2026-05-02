# Project Refactoring Plan

## Current Issues
- Duplicate backend implementation (backend folder and api folder)
- Need to switch from SQL migration files to Liquibase for database changes
- Need to restructure into microservices for better separation of concerns

## Phase 1: Consolidation & Liquibase Migration

### Step 1: Migrate to Liquibase
1. Create a new `liquibase` directory in the `api` folder
2. Convert all SQL migrations (including `V5__add_guild_and_town_tables.sql`) to Liquibase XML format
3. Create a master changelog to reference all changesets
4. Update any database initialization scripts to use Liquibase

### Step 2: Consolidate Backend
1. Compare models and functionality between `backend/src` and `api/src`
2. Move any unique functionality from `backend` to `api`
3. Update the `Cargo.toml` in `api` to include any necessary dependencies from `backend`
4. Once everything is migrated, remove the redundant `backend` folder

## Phase 2: Microservice Architecture

### Step 1: Restructure into Services
Create four main services:

1. **quest-service**
   - Responsible for quest CRUD operations
   - Quest assignment and completion
   - Quest rewards calculation

2. **guild-service**
   - Guild and town management
   - Guild membership
   - Guild rewards and treasury

3. **character-service**
   - Character CRUD operations
   - Skills and abilities
   - Character progression

4. **reward-service**
   - Inventory management
   - Guild and character rewards
   - Currency and item tracking

### Step 2: For Each Service
1. Create appropriate directory structure
2. Migrate relevant models, repositories, and handlers
3. Set up independent Liquibase changelogs
4. Create Docker configuration
5. Establish service-to-service communication

### Step 3: API Gateway
1. Create an API gateway to route requests to appropriate services
2. Implement authentication/authorization at the gateway level
3. Configure CORS and other shared middleware

## Implementation Timeline

### Week 1
- Complete Phase 1 (Consolidation & Liquibase Migration)
- Set up basic structure for microservices

### Week 2
- Implement quest-service and character-service
- Configure communication between services
- Update client application to use new service endpoints

### Week 3
- Implement guild-service and reward-service
- Complete API gateway implementation
- Finalize documentation 