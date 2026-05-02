use crate::error::ApiError;
use crate::models::{Character, CharacterCreate, CharacterUpdate};
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;
use uuid::Uuid;
use std::sync::Arc;
use log::error;
use serde_json::Value;

pub struct CharacterRepository {
    pool: Arc<Pool>,
}

impl CharacterRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
    
    // Get all characters
    pub async fn get_characters(&self, limit: i64, offset: i64) -> Result<(Vec<Character>, i64), ApiError> {
        let client = self.pool.get().await?;
        
        // Get total count
        let query = "
            SELECT COUNT(*) FROM guild_app.characters
        ";
        let row = client.query_one(query, &[]).await?;
        let total_count = row.get::<_, i64>(0);
        
        // Get characters with pagination
        let query = "
            SELECT id, name, type, level, xp, guild, team_id, properties, created_at, updated_at
            FROM guild_app.characters
            ORDER BY name ASC
            LIMIT $1 OFFSET $2
        ";
        
        let rows = client.query(query, &[&limit, &offset]).await?;
        
        let mut characters = Vec::new();
        
        for row in rows {
            let character_id: Uuid = row.get("id");
            
            // Get the character's skills
            let skills = self.get_character_skills(&client, character_id).await?;
            
            characters.push(Character {
                id: character_id,
                name: row.get("name"),
                type_: row.get("type"),
                skills,
                level: row.get("level"),
                xp: row.get("xp"),
                guild: row.get("guild"),
                team_id: row.get("team_id"),
                properties: row.get("properties"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
        
        Ok((characters, total_count))
    }
    
    // Get a character by ID
    pub async fn get_character_by_id(&self, character_id: Uuid) -> Result<Character, ApiError> {
        let client = self.pool.get().await?;
        
        let query = "
            SELECT id, name, type, level, xp, guild, team_id, properties, created_at, updated_at
            FROM guild_app.characters
            WHERE id = $1
        ";
        
        let row = client.query_opt(query, &[&character_id]).await?;
        
        match row {
            Some(row) => {
                // Get the character's skills
                let skills = self.get_character_skills(&client, character_id).await?;
                
                Ok(Character {
                    id: character_id,
                    name: row.get("name"),
                    type_: row.get("type"),
                    skills,
                    level: row.get("level"),
                    xp: row.get("xp"),
                    guild: row.get("guild"),
                    team_id: row.get("team_id"),
                    properties: row.get("properties"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
            },
            None => Err(ApiError::NotFoundError(format!("Character with ID {} not found", character_id))),
        }
    }
    
    // Create a new character
    pub async fn create_character(&self, character: CharacterCreate) -> Result<Character, ApiError> {
        let client = self.pool.get().await?;
        
        // Start a transaction
        let tx = client.transaction().await?;
        
        // Insert the character
        let query = "
            INSERT INTO guild_app.characters 
            (name, type, level, xp, guild, team_id, properties)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, name, type, level, xp, guild, team_id, properties, created_at, updated_at
        ";
        
        let level = character.level.unwrap_or(1);
        let xp = character.xp.unwrap_or(0);
        let properties = character.properties.unwrap_or_else(|| serde_json::json!({}));
        
        let row = tx.query_one(
            query, 
            &[
                &character.name, 
                &character.type_, 
                &level, 
                &xp, 
                &character.guild, 
                &character.team_id, 
                &properties
            ]
        ).await?;
        
        let character_id: Uuid = row.get("id");
        
        // Add skills if provided
        if let Some(skill_ids) = &character.skill_ids {
            for skill_id in skill_ids {
                let query = "
                    INSERT INTO guild_app.character_skills
                    (character_id, skill_id, level)
                    VALUES ($1, $2, $3)
                ";
                
                tx.execute(query, &[&character_id, skill_id, &1]).await?;
            }
        }
        
        // Commit the transaction
        tx.commit().await?;
        
        // Get the created character with its skills
        self.get_character_by_id(character_id).await
    }
    
    // Update an existing character
    pub async fn update_character(&self, character_id: Uuid, character: CharacterUpdate) -> Result<Character, ApiError> {
        let client = self.pool.get().await?;
        
        // Check if the character exists
        let exists = client
            .query_one("SELECT COUNT(*) FROM guild_app.characters WHERE id = $1", &[&character_id])
            .await?
            .get::<_, i64>(0) > 0;
            
        if !exists {
            return Err(ApiError::NotFoundError(format!("Character with ID {} not found", character_id)));
        }
        
        // Start a transaction
        let tx = client.transaction().await?;
        
        // Update the character fields
        let mut query = String::from("UPDATE guild_app.characters SET ");
        let mut params: Vec<Box<dyn ToSql + Sync>> = Vec::new();
        let mut param_idx = 1;
        
        if let Some(name) = &character.name {
            query.push_str(&format!("name = ${}, ", param_idx));
            params.push(Box::new(name.clone()));
            param_idx += 1;
        }
        
        if let Some(type_) = &character.type_ {
            query.push_str(&format!("type = ${}, ", param_idx));
            params.push(Box::new(type_.clone()));
            param_idx += 1;
        }
        
        if let Some(level) = &character.level {
            query.push_str(&format!("level = ${}, ", param_idx));
            params.push(Box::new(*level));
            param_idx += 1;
        }
        
        if let Some(xp) = &character.xp {
            query.push_str(&format!("xp = ${}, ", param_idx));
            params.push(Box::new(*xp));
            param_idx += 1;
        }
        
        if let Some(guild) = &character.guild {
            query.push_str(&format!("guild = ${}, ", param_idx));
            params.push(Box::new(guild.clone()));
            param_idx += 1;
        }
        
        if let Some(team_id) = &character.team_id {
            query.push_str(&format!("team_id = ${}, ", param_idx));
            params.push(Box::new(team_id.clone()));
            param_idx += 1;
        }
        
        if let Some(properties) = &character.properties {
            query.push_str(&format!("properties = ${}, ", param_idx));
            params.push(Box::new(properties.clone()));
            param_idx += 1;
        }
        
        // Only proceed if there are fields to update
        if param_idx > 1 {
            // Remove the trailing comma and space
            query.truncate(query.len() - 2);
            
            // Add the WHERE clause
            query.push_str(&format!(" WHERE id = ${}", param_idx));
            params.push(Box::new(character_id));
            
            // Create a slice of references to the ToSql trait objects
            let param_refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p.as_ref()).collect();
            
            // Execute the update
            tx.execute(&query, &param_refs[..]).await?;
        }
        
        // Update skills if provided
        if let Some(skill_ids) = &character.skill_ids {
            // Delete existing skills
            tx.execute(
                "DELETE FROM guild_app.character_skills WHERE character_id = $1",
                &[&character_id]
            ).await?;
            
            // Add new skills
            for skill_id in skill_ids {
                tx.execute(
                    "INSERT INTO guild_app.character_skills (character_id, skill_id, level) VALUES ($1, $2, $3)",
                    &[&character_id, skill_id, &1]
                ).await?;
            }
        }
        
        // Commit the transaction
        tx.commit().await?;
        
        // Get the updated character
        self.get_character_by_id(character_id).await
    }
    
    // Delete a character
    pub async fn delete_character(&self, character_id: Uuid) -> Result<(), ApiError> {
        let client = self.pool.get().await?;
        
        // Check if the character exists
        let exists = client
            .query_one("SELECT COUNT(*) FROM guild_app.characters WHERE id = $1", &[&character_id])
            .await?
            .get::<_, i64>(0) > 0;
            
        if !exists {
            return Err(ApiError::NotFoundError(format!("Character with ID {} not found", character_id)));
        }
        
        // Delete the character (cascade will handle related records)
        client.execute(
            "DELETE FROM guild_app.characters WHERE id = $1",
            &[&character_id]
        ).await?;
        
        Ok(())
    }
    
    // Get all characters in a team
    pub async fn get_team_characters(&self, team_id: Uuid) -> Result<Vec<Character>, ApiError> {
        let client = self.pool.get().await?;
        
        let query = "
            SELECT id, name, type, level, xp, guild, team_id, properties, created_at, updated_at
            FROM guild_app.characters
            WHERE team_id = $1
            ORDER BY name ASC
        ";
        
        let rows = client.query(query, &[&team_id]).await?;
        
        let mut characters = Vec::new();
        
        for row in rows {
            let character_id: Uuid = row.get("id");
            
            // Get the character's skills
            let skills = self.get_character_skills(&client, character_id).await?;
            
            characters.push(Character {
                id: character_id,
                name: row.get("name"),
                type_: row.get("type"),
                skills,
                level: row.get("level"),
                xp: row.get("xp"),
                guild: row.get("guild"),
                team_id: row.get("team_id"),
                properties: row.get("properties"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
        
        Ok(characters)
    }
    
    // Helper method to get skills for a character
    async fn get_character_skills<C>(&self, client: &C, character_id: Uuid) -> Result<Vec<Value>, ApiError>
    where
        C: tokio_postgres::GenericClient
    {
        let query = "
            SELECT s.id, s.name, s.description, s.category, cs.level
            FROM guild_app.character_skills cs
            JOIN guild_app.skills s ON cs.skill_id = s.id
            WHERE cs.character_id = $1
        ";
        
        let rows = client.query(query, &[&character_id]).await.map_err(|e| {
            error!("Error fetching skills for character {}: {}", character_id, e);
            ApiError::DatabaseError(e)
        })?;
        
        let skills = rows.into_iter().map(|row| {
            serde_json::json!({
                "id": row.get::<_, Uuid>("id").to_string(),
                "name": row.get::<_, String>("name"),
                "description": row.get::<_, String>("description"),
                "category": row.get::<_, String>("category"),
                "level": row.get::<_, i32>("level")
            })
        }).collect();
        
        Ok(skills)
    }
} 