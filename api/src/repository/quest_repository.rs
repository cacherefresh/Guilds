use crate::error::ApiError;
use crate::models::{Quest, QuestCreate, QuestUpdate, Skill};
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;
use uuid::Uuid;
use std::sync::Arc;
use log::error;

pub struct QuestRepository {
    pool: Arc<Pool>,
}

impl QuestRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    // Get all quests with optional filtering
    pub async fn get_quests(
        &self,
        skill_ids: Option<Vec<Uuid>>,
        exact_match: Option<bool>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Quest>, i64), ApiError> {
        let client = self.pool.get().await?;
        
        let mut quests = Vec::new();
        let mut total_count = 0;

        // If skill IDs are provided, use the appropriate filter
        if let Some(skill_ids) = skill_ids {
            if skill_ids.is_empty() {
                // If an empty skill list is provided, just get all quests
                let query = "
                    SELECT COUNT(*) FROM guild_app.quests
                    WHERE status = 'AVAILABLE'
                ";
                let count_row = client.query_one(query, &[]).await?;
                total_count = count_row.get::<_, i64>(0);

                let query = "
                    SELECT id, title, description, difficulty, reward, xp_reward, gold_reward, 
                           status, created_at, updated_at
                    FROM guild_app.quests
                    WHERE status = 'AVAILABLE'
                    ORDER BY created_at DESC
                    LIMIT $1 OFFSET $2
                ";
                let rows = client.query(query, &[&limit, &offset]).await?;

                for row in rows {
                    let quest_id: Uuid = row.get("id");
                    
                    // Get required skills for the quest
                    let required_skills = self.get_quest_required_skills(&client, quest_id).await?;
                    
                    quests.push(Quest {
                        id: quest_id,
                        title: row.get("title"),
                        description: row.get("description"),
                        difficulty: row.get("difficulty"),
                        required_skills,
                        reward: row.get("reward"),
                        xp_reward: row.get("xp_reward"),
                        gold_reward: row.get("gold_reward"),
                        status: row.get("status"),
                        created_at: row.get("created_at"),
                        updated_at: row.get("updated_at"),
                    });
                }
            } else {
                // Use exact match or subset based on the parameter
                let exact_match = exact_match.unwrap_or(false);
                
                if exact_match {
                    // Get quests that match exactly the provided skills
                    let query = format!(
                        "SELECT * FROM guild_app.find_quests_by_exact_skills($1::uuid[])"
                    );
                    
                    let rows = client.query(&query, &[&skill_ids]).await?;
                    total_count = rows.len() as i64;
                    
                    // Apply limit and offset manually (since we're using a function)
                    let rows = rows.into_iter()
                        .skip(offset as usize)
                        .take(limit as usize)
                        .collect::<Vec<_>>();
                    
                    for row in rows {
                        let quest_id: Uuid = row.get("quest_id");
                        
                        // Get required skills for the quest
                        let required_skills = self.get_quest_required_skills(&client, quest_id).await?;
                        
                        // Get the full quest details
                        let query = "
                            SELECT reward, xp_reward, gold_reward, status, created_at, updated_at
                            FROM guild_app.quests
                            WHERE id = $1
                        ";
                        let quest_row = client.query_one(query, &[&quest_id]).await?;
                        
                        quests.push(Quest {
                            id: quest_id,
                            title: row.get("title"),
                            description: row.get("description"),
                            difficulty: row.get("difficulty"),
                            required_skills,
                            reward: quest_row.get("reward"),
                            xp_reward: quest_row.get("xp_reward"),
                            gold_reward: quest_row.get("gold_reward"),
                            status: quest_row.get("status"),
                            created_at: quest_row.get("created_at"),
                            updated_at: quest_row.get("updated_at"),
                        });
                    }
                } else {
                    // Get quests that require a subset of the provided skills
                    let query = format!(
                        "SELECT * FROM guild_app.find_quests_by_subset_skills($1::uuid[])"
                    );
                    
                    let rows = client.query(&query, &[&skill_ids]).await?;
                    total_count = rows.len() as i64;
                    
                    // Apply limit and offset manually (since we're using a function)
                    let rows = rows.into_iter()
                        .skip(offset as usize)
                        .take(limit as usize)
                        .collect::<Vec<_>>();
                    
                    for row in rows {
                        let quest_id: Uuid = row.get("quest_id");
                        
                        // Get required skills for the quest
                        let required_skills = self.get_quest_required_skills(&client, quest_id).await?;
                        
                        // Get the full quest details
                        let query = "
                            SELECT reward, xp_reward, gold_reward, status, created_at, updated_at
                            FROM guild_app.quests
                            WHERE id = $1
                        ";
                        let quest_row = client.query_one(query, &[&quest_id]).await?;
                        
                        quests.push(Quest {
                            id: quest_id,
                            title: row.get("title"),
                            description: row.get("description"),
                            difficulty: row.get("difficulty"),
                            required_skills,
                            reward: quest_row.get("reward"),
                            xp_reward: quest_row.get("xp_reward"),
                            gold_reward: quest_row.get("gold_reward"),
                            status: quest_row.get("status"),
                            created_at: quest_row.get("created_at"),
                            updated_at: quest_row.get("updated_at"),
                        });
                    }
                }
            }
        } else {
            // No skill filters, just get all quests
            let query = "
                SELECT COUNT(*) FROM guild_app.quests
            ";
            let count_row = client.query_one(query, &[]).await?;
            total_count = count_row.get::<_, i64>(0);

            let query = "
                SELECT id, title, description, difficulty, reward, xp_reward, gold_reward, 
                       status, created_at, updated_at
                FROM guild_app.quests
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
            ";
            let rows = client.query(query, &[&limit, &offset]).await?;

            for row in rows {
                let quest_id: Uuid = row.get("id");
                
                // Get required skills for the quest
                let required_skills = self.get_quest_required_skills(&client, quest_id).await?;
                
                quests.push(Quest {
                    id: quest_id,
                    title: row.get("title"),
                    description: row.get("description"),
                    difficulty: row.get("difficulty"),
                    required_skills,
                    reward: row.get("reward"),
                    xp_reward: row.get("xp_reward"),
                    gold_reward: row.get("gold_reward"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                });
            }
        }

        Ok((quests, total_count))
    }

    // Get a specific quest by ID
    pub async fn get_quest_by_id(&self, quest_id: Uuid) -> Result<Quest, ApiError> {
        let client = self.pool.get().await?;
        
        let query = "
            SELECT id, title, description, difficulty, reward, xp_reward, gold_reward, 
                   status, created_at, updated_at
            FROM guild_app.quests
            WHERE id = $1
        ";
        
        let row = client.query_opt(query, &[&quest_id]).await?;
        
        match row {
            Some(row) => {
                // Get required skills for the quest
                let required_skills = self.get_quest_required_skills(&client, quest_id).await?;
                
                Ok(Quest {
                    id: row.get("id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    difficulty: row.get("difficulty"),
                    required_skills,
                    reward: row.get("reward"),
                    xp_reward: row.get("xp_reward"),
                    gold_reward: row.get("gold_reward"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
            },
            None => Err(ApiError::NotFoundError(format!("Quest with ID {} not found", quest_id))),
        }
    }

    // Create a new quest
    pub async fn create_quest(&self, quest: QuestCreate) -> Result<Quest, ApiError> {
        let client = self.pool.get().await?;
        
        // Start a transaction
        let tx = client.transaction().await?;
        
        // Insert the quest
        let query = "
            INSERT INTO guild_app.quests 
            (title, description, difficulty, reward, xp_reward, gold_reward)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, title, description, difficulty, reward, xp_reward, gold_reward, 
                      status, created_at, updated_at
        ";
        
        let difficulty = quest.difficulty.unwrap_or(1);
        let xp_reward = quest.xp_reward.unwrap_or(0);
        let gold_reward = quest.gold_reward.unwrap_or(0);
        
        let row = tx.query_one(
            query, 
            &[
                &quest.title, 
                &quest.description, 
                &difficulty, 
                &quest.reward, 
                &xp_reward, 
                &gold_reward
            ]
        ).await?;
        
        let quest_id: Uuid = row.get("id");
        
        // Add required skills for the quest
        for skill_id in &quest.required_skill_ids {
            let query = "
                INSERT INTO guild_app.quest_required_skills
                (quest_id, skill_id, minimum_level)
                VALUES ($1, $2, $3)
            ";
            
            tx.execute(query, &[&quest_id, skill_id, &1]).await?;
        }
        
        // Commit the transaction
        tx.commit().await?;
        
        // Get the created quest with its skills
        self.get_quest_by_id(quest_id).await
    }

    // Update an existing quest
    pub async fn update_quest(&self, quest_id: Uuid, quest: QuestUpdate) -> Result<Quest, ApiError> {
        let client = self.pool.get().await?;
        
        // Check if the quest exists
        let exists = client
            .query_one("SELECT COUNT(*) FROM guild_app.quests WHERE id = $1", &[&quest_id])
            .await?
            .get::<_, i64>(0) > 0;
            
        if !exists {
            return Err(ApiError::NotFoundError(format!("Quest with ID {} not found", quest_id)));
        }
        
        // Start a transaction
        let tx = client.transaction().await?;
        
        // Update the quest fields
        let mut query = String::from("UPDATE guild_app.quests SET ");
        let mut params: Vec<Box<dyn ToSql + Sync>> = Vec::new();
        let mut param_idx = 1;
        
        if let Some(title) = &quest.title {
            query.push_str(&format!("title = ${}, ", param_idx));
            params.push(Box::new(title.clone()));
            param_idx += 1;
        }
        
        if let Some(description) = &quest.description {
            query.push_str(&format!("description = ${}, ", param_idx));
            params.push(Box::new(description.clone()));
            param_idx += 1;
        }
        
        if let Some(difficulty) = &quest.difficulty {
            query.push_str(&format!("difficulty = ${}, ", param_idx));
            params.push(Box::new(*difficulty));
            param_idx += 1;
        }
        
        if let Some(reward) = &quest.reward {
            query.push_str(&format!("reward = ${}, ", param_idx));
            params.push(Box::new(reward.clone()));
            param_idx += 1;
        }
        
        if let Some(xp_reward) = &quest.xp_reward {
            query.push_str(&format!("xp_reward = ${}, ", param_idx));
            params.push(Box::new(*xp_reward));
            param_idx += 1;
        }
        
        if let Some(gold_reward) = &quest.gold_reward {
            query.push_str(&format!("gold_reward = ${}, ", param_idx));
            params.push(Box::new(*gold_reward));
            param_idx += 1;
        }
        
        if let Some(status) = &quest.status {
            query.push_str(&format!("status = ${}, ", param_idx));
            params.push(Box::new(status.clone()));
            param_idx += 1;
        }
        
        // Only proceed if there are fields to update
        if param_idx > 1 {
            // Remove the trailing comma and space
            query.truncate(query.len() - 2);
            
            // Add the WHERE clause
            query.push_str(&format!(" WHERE id = ${}", param_idx));
            params.push(Box::new(quest_id));
            
            // Create a slice of references to the ToSql trait objects
            let param_refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p.as_ref()).collect();
            
            // Execute the update
            tx.execute(&query, &param_refs[..]).await?;
        }
        
        // Update required skills if provided
        if let Some(skill_ids) = &quest.required_skill_ids {
            // Delete existing skills
            tx.execute(
                "DELETE FROM guild_app.quest_required_skills WHERE quest_id = $1",
                &[&quest_id]
            ).await?;
            
            // Add new skills
            for skill_id in skill_ids {
                tx.execute(
                    "INSERT INTO guild_app.quest_required_skills (quest_id, skill_id, minimum_level) VALUES ($1, $2, $3)",
                    &[&quest_id, skill_id, &1]
                ).await?;
            }
        }
        
        // Commit the transaction
        tx.commit().await?;
        
        // Get the updated quest
        self.get_quest_by_id(quest_id).await
    }

    // Delete a quest
    pub async fn delete_quest(&self, quest_id: Uuid) -> Result<(), ApiError> {
        let client = self.pool.get().await?;
        
        // Check if the quest exists
        let exists = client
            .query_one("SELECT COUNT(*) FROM guild_app.quests WHERE id = $1", &[&quest_id])
            .await?
            .get::<_, i64>(0) > 0;
            
        if !exists {
            return Err(ApiError::NotFoundError(format!("Quest with ID {} not found", quest_id)));
        }
        
        // Delete the quest (cascade will handle related records)
        client.execute(
            "DELETE FROM guild_app.quests WHERE id = $1",
            &[&quest_id]
        ).await?;
        
        Ok(())
    }

    // Get available quests for a character
    pub async fn get_available_quests_for_character(
        &self, 
        character_id: Uuid,
        include_team: bool
    ) -> Result<Vec<Quest>, ApiError> {
        let client = self.pool.get().await?;
        
        // Use the SQL function to find available quests
        let query = "
            SELECT * FROM guild_app.find_available_quests_for_character($1, $2)
        ";
        
        let rows = client.query(query, &[&character_id, &include_team]).await?;
        
        let mut quests = Vec::new();
        
        for row in rows {
            let quest_id: Uuid = row.get("quest_id");
            
            // Get the full quest details
            let quest = self.get_quest_by_id(quest_id).await?;
            quests.push(quest);
        }
        
        Ok(quests)
    }

    // Helper method to get required skills for a quest
    async fn get_quest_required_skills<C>(&self, client: &C, quest_id: Uuid) -> Result<Vec<Skill>, ApiError>
    where
        C: tokio_postgres::GenericClient
    {
        let query = "
            SELECT s.id, s.name, s.description, s.category, s.level
            FROM guild_app.quest_required_skills qrs
            JOIN guild_app.skills s ON qrs.skill_id = s.id
            WHERE qrs.quest_id = $1
        ";
        
        let rows = client.query(query, &[&quest_id]).await.map_err(|e| {
            error!("Error fetching required skills for quest {}: {}", quest_id, e);
            ApiError::DatabaseError(e)
        })?;
        
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
} 