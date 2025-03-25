use crate::error::ApiError;
use crate::models::{Skill, SkillCreate};
use deadpool_postgres::Pool;
use uuid::Uuid;
use std::sync::Arc;
use log::error;

pub struct SkillRepository {
    pool: Arc<Pool>,
}

impl SkillRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    // Get all skills
    pub async fn get_skills(&self, limit: i64, offset: i64) -> Result<(Vec<Skill>, i64), ApiError> {
        let client = self.pool.get().await?;
        
        // Get total count
        let query = "
            SELECT COUNT(*) FROM guild_app.skills
        ";
        let row = client.query_one(query, &[]).await?;
        let total_count = row.get::<_, i64>(0);
        
        // Get skills with pagination
        let query = "
            SELECT id, name, description, category, level
            FROM guild_app.skills
            ORDER BY name ASC
            LIMIT $1 OFFSET $2
        ";
        
        let rows = client.query(query, &[&limit, &offset]).await?;
        
        let skills = rows.into_iter().map(|row| {
            Skill {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                category: row.get("category"),
                level: row.get("level"),
            }
        }).collect();
        
        Ok((skills, total_count))
    }

    // Get a skill by ID
    pub async fn get_skill_by_id(&self, skill_id: Uuid) -> Result<Skill, ApiError> {
        let client = self.pool.get().await?;
        
        let query = "
            SELECT id, name, description, category, level
            FROM guild_app.skills
            WHERE id = $1
        ";
        
        let row = client.query_opt(query, &[&skill_id]).await?;
        
        match row {
            Some(row) => {
                Ok(Skill {
                    id: row.get("id"),
                    name: row.get("name"),
                    description: row.get("description"),
                    category: row.get("category"),
                    level: row.get("level"),
                })
            },
            None => Err(ApiError::NotFoundError(format!("Skill with ID {} not found", skill_id))),
        }
    }

    // Create a new skill
    pub async fn create_skill(&self, skill: SkillCreate) -> Result<Skill, ApiError> {
        let client = self.pool.get().await?;
        
        // Check if a skill with the same name already exists
        let exists = client
            .query_one(
                "SELECT COUNT(*) FROM guild_app.skills WHERE name = $1",
                &[&skill.name]
            )
            .await?
            .get::<_, i64>(0) > 0;
            
        if exists {
            return Err(ApiError::ConflictError(format!("Skill with name '{}' already exists", skill.name)));
        }
        
        // Insert the skill
        let query = "
            INSERT INTO guild_app.skills (name, description, category, level)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, description, category, level
        ";
        
        let level = skill.level.unwrap_or(1);
        
        let row = client.query_one(
            query,
            &[&skill.name, &skill.description, &skill.category, &level]
        ).await?;
        
        Ok(Skill {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            category: row.get("category"),
            level: row.get("level"),
        })
    }

    // Get skills by IDs
    pub async fn get_skills_by_ids(&self, skill_ids: &[Uuid]) -> Result<Vec<Skill>, ApiError> {
        if skill_ids.is_empty() {
            return Ok(Vec::new());
        }
        
        let client = self.pool.get().await?;
        
        // Create a query with the right number of parameters
        let params_str = (1..=skill_ids.len())
            .map(|i| format!("${}", i))
            .collect::<Vec<_>>()
            .join(",");
            
        let query = format!(
            "SELECT id, name, description, category, level 
             FROM guild_app.skills 
             WHERE id IN ({}) 
             ORDER BY name ASC",
            params_str
        );
        
        // Convert skill_ids to a vec of ToSql references
        let params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = skill_ids
            .iter()
            .map(|id| id as &(dyn tokio_postgres::types::ToSql + Sync))
            .collect();
            
        let rows = client.query(&query, &params).await?;
        
        let skills = rows.into_iter().map(|row| {
            Skill {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                category: row.get("category"),
                level: row.get("level"),
            }
        }).collect();
        
        Ok(skills)
    }

    // Get skills for a character
    pub async fn get_character_skills(&self, character_id: Uuid) -> Result<Vec<Skill>, ApiError> {
        let client = self.pool.get().await?;
        
        let query = "
            SELECT s.id, s.name, s.description, s.category, cs.level
            FROM guild_app.character_skills cs
            JOIN guild_app.skills s ON cs.skill_id = s.id
            WHERE cs.character_id = $1
            ORDER BY s.name ASC
        ";
        
        let rows = client.query(query, &[&character_id]).await?;
        
        let skills = rows.into_iter().map(|row| {
            Skill {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                category: row.get("category"),
                level: row.get("level"),
            }
        }).collect();
        
        Ok(skills)
    }
    
    // Add skills to a character
    pub async fn add_skills_to_character(
        &self,
        character_id: Uuid,
        skill_ids: &[Uuid],
    ) -> Result<Vec<Skill>, ApiError> {
        let client = self.pool.get().await?;
        
        // Start a transaction
        let tx = client.transaction().await?;
        
        for skill_id in skill_ids {
            // Check if the skill exists
            let skill_exists = tx
                .query_one(
                    "SELECT COUNT(*) FROM guild_app.skills WHERE id = $1",
                    &[skill_id]
                )
                .await?
                .get::<_, i64>(0) > 0;
                
            if !skill_exists {
                return Err(ApiError::NotFoundError(format!("Skill with ID {} not found", skill_id)));
            }
            
            // Check if the character already has this skill
            let char_skill_exists = tx
                .query_one(
                    "SELECT COUNT(*) FROM guild_app.character_skills WHERE character_id = $1 AND skill_id = $2",
                    &[&character_id, skill_id]
                )
                .await?
                .get::<_, i64>(0) > 0;
                
            if char_skill_exists {
                // Update the level (increment by 1)
                tx.execute(
                    "UPDATE guild_app.character_skills SET level = level + 1 WHERE character_id = $1 AND skill_id = $2",
                    &[&character_id, skill_id]
                ).await?;
            } else {
                // Add the skill to the character
                tx.execute(
                    "INSERT INTO guild_app.character_skills (character_id, skill_id, level) VALUES ($1, $2, $3)",
                    &[&character_id, skill_id, &1]
                ).await?;
            }
        }
        
        // Commit the transaction
        tx.commit().await?;
        
        // Get the updated skills for the character
        self.get_character_skills(character_id).await
    }
} 