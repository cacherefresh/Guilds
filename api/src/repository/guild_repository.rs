use crate::error::ApiError;
use crate::models::{
    Guild, GuildCreate, GuildUpdate, GuildSummary, GuildSkillMatch,
    Character, GuildMembership, GuildMemberRecord,
};
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;
use uuid::Uuid;
use std::sync::Arc;
use log::error;
use serde_json::Value;

pub struct GuildRepository {
    pool: Arc<Pool>,
}

impl GuildRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
    
    // Get all guilds
    pub async fn get_guilds(&self, limit: i64, offset: i64) -> Result<(Vec<GuildSummary>, i64), ApiError> {
        let client = self.pool.get().await?;
        
        // Get total count
        let query = "
            SELECT COUNT(*) FROM guild_app.guilds
        ";
        let row = client.query_one(query, &[]).await?;
        let total_count = row.get::<_, i64>(0);
        
        // Get guilds with pagination
        let query = "
            SELECT 
                g.id, 
                g.name, 
                g.description, 
                g.guild_master_id,
                c.name as guild_master_name,
                t.name as town_name,
                g.quests_completed,
                (SELECT COUNT(*) FROM guild_app.character_guilds WHERE guild_id = g.id) as member_count
            FROM 
                guild_app.guilds g
            LEFT JOIN 
                guild_app.characters c ON g.guild_master_id = c.id
            LEFT JOIN 
                guild_app.towns t ON g.town_id = t.id
            ORDER BY 
                g.name ASC
            LIMIT $1 OFFSET $2
        ";
        
        let rows = client.query(query, &[&limit, &offset]).await?;
        
        let guilds = rows.into_iter().map(|row| {
            GuildSummary {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                guild_master_name: row.get("guild_master_name"),
                town_name: row.get("town_name"),
                member_count: row.get("member_count"),
                quests_completed: row.get("quests_completed"),
            }
        }).collect();
        
        Ok((guilds, total_count))
    }
    
    // Get a guild by ID with full details
    pub async fn get_guild_by_id(&self, guild_id: Uuid) -> Result<Guild, ApiError> {
        let client = self.pool.get().await?;
        
        let query = "
            SELECT 
                g.id, 
                g.name, 
                g.description, 
                g.guild_master_id, 
                g.town_id,
                g.reward_divider_percentage,
                g.stash_reward,
                g.quests_completed,
                g.created_at, 
                g.updated_at
            FROM 
                guild_app.guilds g
            WHERE 
                g.id = $1
        ";
        
        let row = client.query_opt(query, &[&guild_id]).await?;
        
        match row {
            Some(row) => {
                // Get guild master
                let guild_master_id: Option<Uuid> = row.get("guild_master_id");
                let guild_master = if let Some(id) = guild_master_id {
                    self.get_character_summary(&client, id).await?
                } else {
                    None
                };
                
                // Get AI teammate
                let ai_teammate = self.get_ai_teammate(&client, guild_id).await?;
                
                // Get guild members
                let members = self.get_guild_members(&client, guild_id).await?;
                
                Ok(Guild {
                    id: row.get("id"),
                    name: row.get("name"),
                    description: row.get("description"),
                    guild_master_id: row.get("guild_master_id"),
                    guild_master,
                    town_id: row.get("town_id"),
                    town: None, // Town details would be fetched separately if needed
                    reward_divider_percentage: row.get("reward_divider_percentage"),
                    stash_reward: row.get("stash_reward"),
                    quests_completed: row.get("quests_completed"),
                    members: Some(members),
                    ai_teammate,
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
            },
            None => Err(ApiError::NotFoundError(format!("Guild with ID {} not found", guild_id))),
        }
    }
    
    // Create a new guild
    pub async fn create_guild(&self, guild: GuildCreate) -> Result<Guild, ApiError> {
        let client = self.pool.get().await?;
        
        // Check if a guild with the same name already exists
        let exists = client
            .query_one(
                "SELECT COUNT(*) FROM guild_app.guilds WHERE name = $1",
                &[&guild.name]
            )
            .await?
            .get::<_, i64>(0) > 0;
            
        if exists {
            return Err(ApiError::ConflictError(format!("Guild with name '{}' already exists", guild.name)));
        }
        
        // Check if character exists
        let character_exists = client
            .query_one(
                "SELECT COUNT(*) FROM guild_app.characters WHERE id = $1",
                &[&guild.guild_master_id]
            )
            .await?
            .get::<_, i64>(0) > 0;
            
        if !character_exists {
            return Err(ApiError::BadRequestError(format!("Character with ID {} not found", guild.guild_master_id)));
        }
        
        // If town_id is provided, check if it exists
        if let Some(town_id) = &guild.town_id {
            let town_exists = client
                .query_one(
                    "SELECT COUNT(*) FROM guild_app.towns WHERE id = $1",
                    &[town_id]
                )
                .await?
                .get::<_, i64>(0) > 0;
                
            if !town_exists {
                return Err(ApiError::BadRequestError(format!("Town with ID {} not found", town_id)));
            }
        }
        
        // Start a transaction
        let tx = client.transaction().await?;
        
        // Insert the guild
        let reward_divider = guild.reward_divider_percentage.unwrap_or(80);
        
        let query = "
            INSERT INTO guild_app.guilds 
            (name, description, guild_master_id, town_id, reward_divider_percentage)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, description, guild_master_id, town_id, 
                      reward_divider_percentage, stash_reward, quests_completed,
                      created_at, updated_at
        ";
        
        let row = tx.query_one(
            query,
            &[
                &guild.name, 
                &guild.description, 
                &guild.guild_master_id, 
                &guild.town_id,
                &reward_divider
            ]
        ).await?;
        
        let guild_id: Uuid = row.get("id");
        
        // Add the guild master as a member (this happens via trigger)
        
        // Commit the transaction
        tx.commit().await?;
        
        // Get the created guild with its members
        self.get_guild_by_id(guild_id).await
    }
    
    // Update an existing guild
    pub async fn update_guild(&self, guild_id: Uuid, guild: GuildUpdate) -> Result<Guild, ApiError> {
        let client = self.pool.get().await?;
        
        // Check if the guild exists
        let exists = client
            .query_one("SELECT COUNT(*) FROM guild_app.guilds WHERE id = $1", &[&guild_id])
            .await?
            .get::<_, i64>(0) > 0;
            
        if !exists {
            return Err(ApiError::NotFoundError(format!("Guild with ID {} not found", guild_id)));
        }
        
        // If name is provided, check for name conflicts
        if let Some(name) = &guild.name {
            let name_taken = client
                .query_one(
                    "SELECT COUNT(*) FROM guild_app.guilds WHERE name = $1 AND id != $2",
                    &[name, &guild_id]
                )
                .await?
                .get::<_, i64>(0) > 0;
                
            if name_taken {
                return Err(ApiError::ConflictError(format!("Guild with name '{}' already exists", name)));
            }
        }
        
        // If guild_master_id is provided, check if character exists
        if let Some(master_id) = &guild.guild_master_id {
            let character_exists = client
                .query_one(
                    "SELECT COUNT(*) FROM guild_app.characters WHERE id = $1",
                    &[master_id]
                )
                .await?
                .get::<_, i64>(0) > 0;
                
            if !character_exists {
                return Err(ApiError::BadRequestError(format!("Character with ID {} not found", master_id)));
            }
        }
        
        // If town_id is provided, check if it exists
        if let Some(town_id) = &guild.town_id {
            let town_exists = client
                .query_one(
                    "SELECT COUNT(*) FROM guild_app.towns WHERE id = $1",
                    &[town_id]
                )
                .await?
                .get::<_, i64>(0) > 0;
                
            if !town_exists {
                return Err(ApiError::BadRequestError(format!("Town with ID {} not found", town_id)));
            }
        }
        
        // Start a transaction
        let tx = client.transaction().await?;
        
        // Update the guild fields
        let mut query = String::from("UPDATE guild_app.guilds SET ");
        let mut params: Vec<Box<dyn ToSql + Sync>> = Vec::new();
        let mut param_idx = 1;
        
        if let Some(name) = &guild.name {
            query.push_str(&format!("name = ${}, ", param_idx));
            params.push(Box::new(name.clone()));
            param_idx += 1;
        }
        
        if let Some(description) = &guild.description {
            query.push_str(&format!("description = ${}, ", param_idx));
            params.push(Box::new(description.clone()));
            param_idx += 1;
        }
        
        if let Some(guild_master_id) = &guild.guild_master_id {
            query.push_str(&format!("guild_master_id = ${}, ", param_idx));
            params.push(Box::new(*guild_master_id));
            param_idx += 1;
        }
        
        if let Some(town_id) = &guild.town_id {
            query.push_str(&format!("town_id = ${}, ", param_idx));
            params.push(Box::new(*town_id));
            param_idx += 1;
        }
        
        if let Some(reward_divider) = &guild.reward_divider_percentage {
            query.push_str(&format!("reward_divider_percentage = ${}, ", param_idx));
            params.push(Box::new(*reward_divider));
            param_idx += 1;
        }
        
        // Only proceed if there are fields to update
        if param_idx > 1 {
            // Remove the trailing comma and space
            query.truncate(query.len() - 2);
            
            // Add the WHERE clause
            query.push_str(&format!(" WHERE id = ${}", param_idx));
            params.push(Box::new(guild_id));
            
            // Create a slice of references to the ToSql trait objects
            let param_refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p.as_ref()).collect();
            
            // Execute the update
            tx.execute(&query, &param_refs[..]).await?;
        }
        
        // Commit the transaction
        tx.commit().await?;
        
        // Get the updated guild
        self.get_guild_by_id(guild_id).await
    }
    
    // Delete a guild
    pub async fn delete_guild(&self, guild_id: Uuid) -> Result<(), ApiError> {
        let client = self.pool.get().await?;
        
        // Check if the guild exists
        let exists = client
            .query_one("SELECT COUNT(*) FROM guild_app.guilds WHERE id = $1", &[&guild_id])
            .await?
            .get::<_, i64>(0) > 0;
            
        if !exists {
            return Err(ApiError::NotFoundError(format!("Guild with ID {} not found", guild_id)));
        }
        
        // Delete the guild (cascade will handle related records)
        client.execute(
            "DELETE FROM guild_app.guilds WHERE id = $1",
            &[&guild_id]
        ).await?;
        
        Ok(())
    }
    
    // Add a character to a guild
    pub async fn add_character_to_guild(
        &self, 
        guild_id: Uuid, 
        character_id: Uuid,
        set_active: bool
    ) -> Result<GuildMemberRecord, ApiError> {
        let client = self.pool.get().await?;
        
        // Check if the guild exists
        let guild_exists = client
            .query_one("SELECT COUNT(*) FROM guild_app.guilds WHERE id = $1", &[&guild_id])
            .await?
            .get::<_, i64>(0) > 0;
            
        if !guild_exists {
            return Err(ApiError::NotFoundError(format!("Guild with ID {} not found", guild_id)));
        }
        
        // Check if the character exists
        let character_exists = client
            .query_one("SELECT COUNT(*) FROM guild_app.characters WHERE id = $1", &[&character_id])
            .await?
            .get::<_, i64>(0) > 0;
            
        if !character_exists {
            return Err(ApiError::NotFoundError(format!("Character with ID {} not found", character_id)));
        }
        
        // Check if the character is already a member of three guilds
        let guild_count = client
            .query_one(
                "SELECT COUNT(*) FROM guild_app.character_guilds WHERE character_id = $1",
                &[&character_id]
            )
            .await?
            .get::<_, i64>(0);
            
        let is_already_member = client
            .query_one(
                "SELECT COUNT(*) FROM guild_app.character_guilds WHERE character_id = $1 AND guild_id = $2",
                &[&character_id, &guild_id]
            )
            .await?
            .get::<_, i64>(0) > 0;
            
        if guild_count >= 3 && !is_already_member {
            return Err(ApiError::BadRequestError(
                format!("Character with ID {} is already a member of three guilds", character_id)
            ));
        }
        
        // Add the character to the guild or update membership
        let query = if is_already_member {
            "
                UPDATE guild_app.character_guilds 
                SET is_active = $3
                WHERE character_id = $1 AND guild_id = $2
                RETURNING character_id, guild_id, is_active, joined_at
            "
        } else {
            "
                INSERT INTO guild_app.character_guilds (character_id, guild_id, is_active)
                VALUES ($1, $2, $3)
                RETURNING character_id, guild_id, is_active, joined_at
            "
        };
        
        let row = client.query_one(query, &[&character_id, &guild_id, &set_active]).await?;
        
        Ok(GuildMemberRecord {
            character_id: row.get("character_id"),
            guild_id: row.get("guild_id"),
            is_active: row.get("is_active"),
            joined_at: row.get("joined_at"),
        })
    }
    
    // Remove a character from a guild
    pub async fn remove_character_from_guild(
        &self, 
        guild_id: Uuid, 
        character_id: Uuid
    ) -> Result<(), ApiError> {
        let client = self.pool.get().await?;
        
        // Check if the character is a member of the guild
        let is_member = client
            .query_one(
                "SELECT COUNT(*) FROM guild_app.character_guilds WHERE character_id = $1 AND guild_id = $2",
                &[&character_id, &guild_id]
            )
            .await?
            .get::<_, i64>(0) > 0;
            
        if !is_member {
            return Err(ApiError::NotFoundError(
                format!("Character with ID {} is not a member of guild with ID {}", character_id, guild_id)
            ));
        }
        
        // Check if the character is the guild master
        let is_guild_master = client
            .query_one(
                "SELECT COUNT(*) FROM guild_app.guilds WHERE id = $1 AND guild_master_id = $2",
                &[&guild_id, &character_id]
            )
            .await?
            .get::<_, i64>(0) > 0;
            
        if is_guild_master {
            return Err(ApiError::BadRequestError(
                format!("Character with ID {} is the guild master and cannot leave the guild", character_id)
            ));
        }
        
        // Remove the character from the guild
        client.execute(
            "DELETE FROM guild_app.character_guilds WHERE character_id = $1 AND guild_id = $2",
            &[&character_id, &guild_id]
        ).await?;
        
        Ok(())
    }
    
    // Set a guild as active for a character
    pub async fn set_active_guild(
        &self, 
        character_id: Uuid, 
        guild_id: Uuid
    ) -> Result<GuildMemberRecord, ApiError> {
        let client = self.pool.get().await?;
        
        // Check if the character is a member of the guild
        let is_member = client
            .query_one(
                "SELECT COUNT(*) FROM guild_app.character_guilds WHERE character_id = $1 AND guild_id = $2",
                &[&character_id, &guild_id]
            )
            .await?
            .get::<_, i64>(0) > 0;
            
        if !is_member {
            return Err(ApiError::NotFoundError(
                format!("Character with ID {} is not a member of guild with ID {}", character_id, guild_id)
            ));
        }
        
        // Update the character's active guild
        let query = "
            UPDATE guild_app.character_guilds 
            SET is_active = TRUE
            WHERE character_id = $1 AND guild_id = $2
            RETURNING character_id, guild_id, is_active, joined_at
        ";
        
        let row = client.query_one(query, &[&character_id, &guild_id]).await?;
        
        Ok(GuildMemberRecord {
            character_id: row.get("character_id"),
            guild_id: row.get("guild_id"),
            is_active: row.get("is_active"),
            joined_at: row.get("joined_at"),
        })
    }
    
    // Get guilds that a character is a member of
    pub async fn get_character_guilds(&self, character_id: Uuid) -> Result<Vec<GuildMembership>, ApiError> {
        let client = self.pool.get().await?;
        
        // Check if the character exists
        let character_exists = client
            .query_one("SELECT COUNT(*) FROM guild_app.characters WHERE id = $1", &[&character_id])
            .await?
            .get::<_, i64>(0) > 0;
            
        if !character_exists {
            return Err(ApiError::NotFoundError(format!("Character with ID {} not found", character_id)));
        }
        
        // Get the character's guild memberships
        let query = "
            SELECT cg.guild_id, g.name as guild_name, cg.is_active, cg.joined_at
            FROM guild_app.character_guilds cg
            JOIN guild_app.guilds g ON cg.guild_id = g.id
            WHERE cg.character_id = $1
            ORDER BY cg.is_active DESC, g.name ASC
        ";
        
        let rows = client.query(query, &[&character_id]).await?;
        
        let memberships = rows.into_iter().map(|row| {
            GuildMembership {
                guild_id: row.get("guild_id"),
                guild_name: row.get("guild_name"),
                is_active: row.get("is_active"),
                joined_at: row.get("joined_at"),
            }
        }).collect();
        
        Ok(memberships)
    }
    
    // Find guilds by skills
    pub async fn find_guilds_by_skills(&self, skill_ids: &[Uuid]) -> Result<Vec<GuildSkillMatch>, ApiError> {
        if skill_ids.is_empty() {
            return Ok(Vec::new());
        }
        
        let client = self.pool.get().await?;
        
        // Use the database function to find guilds
        let query = "SELECT * FROM guild_app.find_guilds_by_skills($1::uuid[])";
        let rows = client.query(query, &[&skill_ids]).await?;
        
        let guilds = rows.into_iter().map(|row| {
            GuildSkillMatch {
                guild_id: row.get("guild_id"),
                guild_name: row.get("guild_name"),
                description: row.get("description"),
                matching_skills_count: row.get("matching_skills_count"),
                total_skills: row.get("total_skills"),
                quests_completed: row.get("quests_completed"),
            }
        }).collect();
        
        Ok(guilds)
    }
    
    // Find guilds in a town
    pub async fn find_guilds_in_town(&self, town_id: Uuid) -> Result<Vec<GuildSummary>, ApiError> {
        let client = self.pool.get().await?;
        
        // Check if the town exists
        let town_exists = client
            .query_one("SELECT COUNT(*) FROM guild_app.towns WHERE id = $1", &[&town_id])
            .await?
            .get::<_, i64>(0) > 0;
            
        if !town_exists {
            return Err(ApiError::NotFoundError(format!("Town with ID {} not found", town_id)));
        }
        
        // Use the database function to find guilds in the town
        let query = "SELECT * FROM guild_app.find_guilds_in_town($1)";
        let rows = client.query(query, &[&town_id]).await?;
        
        let guilds = rows.into_iter().map(|row| {
            GuildSummary {
                id: row.get("guild_id"),
                name: row.get("guild_name"),
                description: row.get("description"),
                guild_master_name: row.get("guild_master_name"),
                town_name: None, // We already know the town
                member_count: row.get("member_count"),
                quests_completed: row.get("quests_completed"),
            }
        }).collect();
        
        Ok(guilds)
    }
    
    // Search guilds by name
    pub async fn search_guilds_by_name(&self, name: &str) -> Result<Vec<GuildSummary>, ApiError> {
        let client = self.pool.get().await?;
        
        let query = "
            SELECT 
                g.id, 
                g.name, 
                g.description, 
                c.name as guild_master_name,
                t.name as town_name,
                g.quests_completed,
                (SELECT COUNT(*) FROM guild_app.character_guilds WHERE guild_id = g.id) as member_count
            FROM 
                guild_app.guilds g
            LEFT JOIN 
                guild_app.characters c ON g.guild_master_id = c.id
            LEFT JOIN 
                guild_app.towns t ON g.town_id = t.id
            WHERE 
                g.name ILIKE $1
            ORDER BY 
                g.name ASC
            LIMIT 20
        ";
        
        let search_pattern = format!("%{}%", name);
        let rows = client.query(query, &[&search_pattern]).await?;
        
        let guilds = rows.into_iter().map(|row| {
            GuildSummary {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                guild_master_name: row.get("guild_master_name"),
                town_name: row.get("town_name"),
                member_count: row.get("member_count"),
                quests_completed: row.get("quests_completed"),
            }
        }).collect();
        
        Ok(guilds)
    }
    
    // Helper method to get a character summary
    async fn get_character_summary<C>(&self, client: &C, character_id: Uuid) -> Result<Option<Character>, ApiError>
    where
        C: tokio_postgres::GenericClient
    {
        let query = "
            SELECT id, name, type, level, xp, properties, created_at, updated_at
            FROM guild_app.characters
            WHERE id = $1
        ";
        
        let row = client.query_opt(query, &[&character_id]).await.map_err(|e| {
            error!("Error fetching character {}: {}", character_id, e);
            ApiError::DatabaseError(e)
        })?;
        
        match row {
            Some(row) => {
                Ok(Some(Character {
                    id: row.get("id"),
                    name: row.get("name"),
                    type_: row.get("type"),
                    skills: Vec::new(), // Skills would be fetched separately if needed
                    level: row.get("level"),
                    xp: row.get("xp"),
                    active_guild_id: None, // Not needed for summary
                    active_guild: None,
                    guild_memberships: None,
                    team_id: None, // Not needed for summary
                    properties: row.get("properties"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            },
            None => Ok(None),
        }
    }
    
    // Helper method to get AI teammate for a guild
    async fn get_ai_teammate<C>(&self, client: &C, guild_id: Uuid) -> Result<Option<Character>, ApiError>
    where
        C: tokio_postgres::GenericClient
    {
        let query = "
            SELECT 
                c.id, c.name, c.type, c.level, c.xp, c.properties, c.created_at, c.updated_at
            FROM 
                guild_app.ai_teammates at
            JOIN 
                guild_app.characters c ON at.character_id = c.id
            WHERE 
                at.guild_id = $1
        ";
        
        let row = client.query_opt(query, &[&guild_id]).await.map_err(|e| {
            error!("Error fetching AI teammate for guild {}: {}", guild_id, e);
            ApiError::DatabaseError(e)
        })?;
        
        match row {
            Some(row) => {
                Ok(Some(Character {
                    id: row.get("id"),
                    name: row.get("name"),
                    type_: row.get("type"),
                    skills: Vec::new(), // Skills would be fetched separately if needed
                    level: row.get("level"),
                    xp: row.get("xp"),
                    active_guild_id: Some(guild_id),
                    active_guild: None,
                    guild_memberships: None,
                    team_id: None,
                    properties: row.get("properties"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }))
            },
            None => Ok(None),
        }
    }
    
    // Helper method to get members of a guild
    async fn get_guild_members<C>(&self, client: &C, guild_id: Uuid) -> Result<Vec<Character>, ApiError>
    where
        C: tokio_postgres::GenericClient
    {
        let query = "
            SELECT 
                c.id, c.name, c.type, c.level, c.xp, c.active_guild_id, c.team_id,
                c.properties, c.created_at, c.updated_at,
                cg.is_active
            FROM 
                guild_app.character_guilds cg
            JOIN 
                guild_app.characters c ON cg.character_id = c.id
            WHERE 
                cg.guild_id = $1
            ORDER BY
                cg.is_active DESC, c.name ASC
        ";
        
        let rows = client.query(query, &[&guild_id]).await.map_err(|e| {
            error!("Error fetching members for guild {}: {}", guild_id, e);
            ApiError::DatabaseError(e)
        })?;
        
        let members = rows.into_iter().map(|row| {
            Character {
                id: row.get("id"),
                name: row.get("name"),
                type_: row.get("type"),
                skills: Vec::new(), // Skills would be fetched separately if needed
                level: row.get("level"),
                xp: row.get("xp"),
                active_guild_id: row.get("active_guild_id"),
                active_guild: None,
                guild_memberships: None,
                team_id: row.get("team_id"),
                properties: row.get("properties"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();
        
        Ok(members)
    }
} 