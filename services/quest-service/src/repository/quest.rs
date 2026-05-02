use tokio_postgres::{Client, Row};
use uuid::Uuid;
use chrono::Utc;
use log::{info, error};
use crate::models::quest::{Quest, QuestCreate, QuestUpdate, QuestPrerequisite, QuestDetail};
use crate::models::subtask::Subtask;
use crate::repository::db::DbError;

pub struct QuestRepository {
    client: Client,
}

impl QuestRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    
    pub async fn get_quests(
        &self,
        offset: i64,
        limit: i64,
        epic_id: Option<Uuid>,
        guild_id: Option<Uuid>,
        contact_character_id: Option<Uuid>,
        assigned_to_id: Option<Uuid>,
        assigned_to_type: Option<&str>,
        quest_type: Option<&str>,
        skills_required: Option<Vec<String>>,
        creator_character_id: Option<Uuid>,
        requestor_character_id: Option<Uuid>,
    ) -> Result<(Vec<Quest>, i64), DbError> {
        let mut query = String::from(
            "SELECT id, title, description, status, xp_reward, gold_reward, epic_id, guild_id, 
            contact_character_id, difficulty, deadline, assigned_to_id, assigned_to_type,
            skills_required, quest_type, creator_character_id, requestor_character_id,
            created_at, updated_at FROM quests WHERE 1=1"
        );
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_count = 1;

        if let Some(epic_id_val) = epic_id {
            query.push_str(&format!(" AND epic_id = ${}", param_count));
            params.push(&epic_id_val);
            param_count += 1;
        }

        if let Some(guild_id_val) = guild_id {
            query.push_str(&format!(" AND guild_id = ${}", param_count));
            params.push(&guild_id_val);
            param_count += 1;
        }

        if let Some(contact_character_id_val) = contact_character_id {
            query.push_str(&format!(" AND contact_character_id = ${}", param_count));
            params.push(&contact_character_id_val);
            param_count += 1;
        }

        if let Some(assigned_to_id_val) = assigned_to_id {
            query.push_str(&format!(" AND assigned_to_id = ${}", param_count));
            params.push(&assigned_to_id_val);
            param_count += 1;

            if let Some(assigned_to_type_val) = assigned_to_type {
                query.push_str(&format!(" AND assigned_to_type = ${}", param_count));
                params.push(&assigned_to_type_val);
                param_count += 1;
            }
        }
        
        if let Some(quest_type_val) = quest_type {
            query.push_str(&format!(" AND quest_type = ${}", param_count));
            params.push(&quest_type_val);
            param_count += 1;
        }
        
        if let Some(skills) = &skills_required {
            if !skills.is_empty() {
                // Use array overlap operator
                query.push_str(&format!(" AND skills_required && ${}", param_count));
                params.push(skills);
                param_count += 1;
            }
        }
        
        if let Some(creator_id) = creator_character_id {
            query.push_str(&format!(" AND creator_character_id = ${}", param_count));
            params.push(&creator_id);
            param_count += 1;
        }
        
        if let Some(requestor_id) = requestor_character_id {
            query.push_str(&format!(" AND requestor_character_id = ${}", param_count));
            params.push(&requestor_id);
            param_count += 1;
        }

        // Count total before applying pagination
        let count_query = format!("SELECT COUNT(*) FROM ({}) as count_query", query);
        let count_row = self.client.query_one(
            &count_query,
            &params[..],
        ).await.map_err(DbError::from)?;
        let total: i64 = count_row.get(0);

        // Apply pagination
        query.push_str(&format!(" ORDER BY created_at DESC LIMIT ${} OFFSET ${}", 
            param_count, param_count + 1));
        params.push(&limit);
        params.push(&offset);

        let rows = self.client.query(
            &query,
            &params[..],
        ).await.map_err(DbError::from)?;

        let quests = rows.iter().map(|row| {
            Quest {
                id: row.get("id"),
                title: row.get("title"),
                description: row.get("description"),
                status: row.get("status"),
                xp_reward: row.get("xp_reward"),
                gold_reward: row.get("gold_reward"),
                epic_id: row.get("epic_id"),
                guild_id: row.get("guild_id"),
                contact_character_id: row.get("contact_character_id"),
                difficulty: row.get("difficulty"),
                deadline: row.get("deadline"),
                assigned_to_id: row.get("assigned_to_id"),
                assigned_to_type: row.get("assigned_to_type"),
                skills_required: row.get("skills_required"),
                quest_type: row.get("quest_type"),
                creator_character_id: row.get("creator_character_id"),
                requestor_character_id: row.get("requestor_character_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();

        Ok((quests, total))
    }
    
    pub async fn get_quest_by_id(&self, id: Uuid) -> Result<Quest, DbError> {
        let row = self.client.query_one(
            "SELECT id, title, description, status, xp_reward, gold_reward, epic_id, guild_id, 
            contact_character_id, difficulty, deadline, assigned_to_id, assigned_to_type,
            skills_required, quest_type, creator_character_id, requestor_character_id, 
            created_at, updated_at FROM quests WHERE id = $1",
            &[&id],
        ).await.map_err(|e| {
            if e.to_string().contains("no rows") {
                DbError::NoDataReturned
            } else {
                DbError::from(e)
            }
        })?;

        Ok(Quest {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            status: row.get("status"),
            xp_reward: row.get("xp_reward"),
            gold_reward: row.get("gold_reward"),
            epic_id: row.get("epic_id"),
            guild_id: row.get("guild_id"),
            contact_character_id: row.get("contact_character_id"),
            difficulty: row.get("difficulty"),
            deadline: row.get("deadline"),
            assigned_to_id: row.get("assigned_to_id"),
            assigned_to_type: row.get("assigned_to_type"),
            skills_required: row.get("skills_required"),
            quest_type: row.get("quest_type"),
            creator_character_id: row.get("creator_character_id"),
            requestor_character_id: row.get("requestor_character_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
    
    pub async fn get_quest_detail(&self, id: Uuid) -> Result<QuestDetail, DbError> {
        // Get the quest
        let quest = self.get_quest_by_id(id).await?;
        
        // Get prerequisites
        let prerequisite_rows = self.client.query(
            "SELECT q.id, q.title, q.description, q.status, q.xp_reward, q.gold_reward, 
            q.epic_id, q.guild_id, q.contact_character_id, q.difficulty, q.deadline, 
            q.assigned_to_id, q.assigned_to_type, q.skills_required, q.quest_type,
            q.creator_character_id, q.requestor_character_id, q.created_at, q.updated_at 
            FROM quests q 
            JOIN quest_prerequisites qp ON q.id = qp.prerequisite_quest_id 
            WHERE qp.quest_id = $1",
            &[&id],
        ).await.map_err(DbError::from)?;
        
        let prerequisites = prerequisite_rows.iter().map(|row| {
            Quest {
                id: row.get("id"),
                title: row.get("title"),
                description: row.get("description"),
                status: row.get("status"),
                xp_reward: row.get("xp_reward"),
                gold_reward: row.get("gold_reward"),
                epic_id: row.get("epic_id"),
                guild_id: row.get("guild_id"),
                contact_character_id: row.get("contact_character_id"),
                difficulty: row.get("difficulty"),
                deadline: row.get("deadline"),
                assigned_to_id: row.get("assigned_to_id"),
                assigned_to_type: row.get("assigned_to_type"),
                skills_required: row.get("skills_required"),
                quest_type: row.get("quest_type"),
                creator_character_id: row.get("creator_character_id"),
                requestor_character_id: row.get("requestor_character_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();
        
        // Get subtasks
        let subtask_rows = self.client.query(
            "SELECT id, quest_id, title, description, status, created_at, updated_at 
            FROM quest_subtasks 
            WHERE quest_id = $1",
            &[&id],
        ).await.map_err(DbError::from)?;
        
        let subtasks = subtask_rows.iter().map(|row| {
            Subtask {
                id: row.get("id"),
                quest_id: row.get("quest_id"),
                title: row.get("title"),
                description: row.get("description"),
                status: row.get("status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();
        
        Ok(QuestDetail {
            quest,
            prerequisites,
            subtasks,
        })
    }
    
    pub async fn create_quest(&self, quest: QuestCreate) -> Result<Quest, DbError> {
        // Start a transaction for quest creation with potential prerequisites
        let tx = self.client.transaction().await.map_err(DbError::from)?;
        
        let now = Utc::now();
        let row = tx.query_one(
            "INSERT INTO quests (
                title, description, status, xp_reward, gold_reward, epic_id, guild_id,
                contact_character_id, difficulty, deadline, assigned_to_id, assigned_to_type,
                skills_required, quest_type, creator_character_id, requestor_character_id,
                created_at, updated_at
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $17) 
            RETURNING id, title, description, status, xp_reward, gold_reward, epic_id, guild_id, 
                contact_character_id, difficulty, deadline, assigned_to_id, assigned_to_type,
                skills_required, quest_type, creator_character_id, requestor_character_id,
                created_at, updated_at",
            &[
                &quest.title, 
                &quest.description, 
                &quest.status, 
                &quest.xp_reward, 
                &quest.gold_reward, 
                &quest.epic_id, 
                &quest.guild_id,
                &quest.contact_character_id, 
                &quest.difficulty, 
                &quest.deadline, 
                &quest.assigned_to_id, 
                &quest.assigned_to_type,
                &quest.skills_required,
                &quest.quest_type,
                &quest.creator_character_id,
                &quest.requestor_character_id,
                &now
            ],
        ).await.map_err(DbError::from)?;

        let quest_id = row.get::<_, Uuid>("id");
        
        // Add prerequisites if provided
        if let Some(prerequisite_ids) = &quest.prerequisite_quest_ids {
            for &prereq_id in prerequisite_ids {
                tx.execute(
                    "INSERT INTO quest_prerequisites (quest_id, prerequisite_quest_id) VALUES ($1, $2)",
                    &[&quest_id, &prereq_id],
                ).await.map_err(DbError::from)?;
            }
        }
        
        // Add subtasks if provided
        if let Some(subtasks) = &quest.subtasks {
            for subtask in subtasks {
                tx.execute(
                    "INSERT INTO quest_subtasks (quest_id, title, description, status, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $5)",
                    &[&quest_id, &subtask.title, &subtask.description, &subtask.status, &now],
                ).await.map_err(DbError::from)?;
            }
        }
        
        // Commit the transaction
        tx.commit().await.map_err(DbError::from)?;
        
        Ok(Quest {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            status: row.get("status"),
            xp_reward: row.get("xp_reward"),
            gold_reward: row.get("gold_reward"),
            epic_id: row.get("epic_id"),
            guild_id: row.get("guild_id"),
            contact_character_id: row.get("contact_character_id"),
            difficulty: row.get("difficulty"),
            deadline: row.get("deadline"),
            assigned_to_id: row.get("assigned_to_id"),
            assigned_to_type: row.get("assigned_to_type"),
            skills_required: row.get("skills_required"),
            quest_type: row.get("quest_type"),
            creator_character_id: row.get("creator_character_id"),
            requestor_character_id: row.get("requestor_character_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
    
    pub async fn update_quest(&self, id: Uuid, quest: QuestUpdate) -> Result<Quest, DbError> {
        // Start a transaction for updating the quest and potentially its prerequisites
        let tx = self.client.transaction().await.map_err(DbError::from)?;
        
        // Check if the quest exists
        let _ = self.get_quest_by_id(id).await?;
        
        let now = Utc::now();
        let mut query = String::from("UPDATE quests SET updated_at = $1");
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        
        params.push(&now);
        let mut param_count = 2;
        
        if let Some(title) = &quest.title {
            query.push_str(&format!(", title = ${}", param_count));
            params.push(title);
            param_count += 1;
        }
        
        if let Some(description) = &quest.description {
            query.push_str(&format!(", description = ${}", param_count));
            params.push(description);
            param_count += 1;
        }
        
        if let Some(status) = &quest.status {
            query.push_str(&format!(", status = ${}", param_count));
            params.push(status);
            param_count += 1;
        }
        
        if let Some(xp_reward) = &quest.xp_reward {
            query.push_str(&format!(", xp_reward = ${}", param_count));
            params.push(xp_reward);
            param_count += 1;
        }
        
        if let Some(gold_reward) = &quest.gold_reward {
            query.push_str(&format!(", gold_reward = ${}", param_count));
            params.push(gold_reward);
            param_count += 1;
        }
        
        if let Some(epic_id) = &quest.epic_id {
            query.push_str(&format!(", epic_id = ${}", param_count));
            params.push(epic_id);
            param_count += 1;
        }
        
        if let Some(guild_id) = &quest.guild_id {
            query.push_str(&format!(", guild_id = ${}", param_count));
            params.push(guild_id);
            param_count += 1;
        }
        
        if let Some(contact_character_id) = &quest.contact_character_id {
            query.push_str(&format!(", contact_character_id = ${}", param_count));
            params.push(contact_character_id);
            param_count += 1;
        }
        
        if let Some(difficulty) = &quest.difficulty {
            query.push_str(&format!(", difficulty = ${}", param_count));
            params.push(difficulty);
            param_count += 1;
        }
        
        if let Some(deadline) = &quest.deadline {
            query.push_str(&format!(", deadline = ${}", param_count));
            params.push(deadline);
            param_count += 1;
        }
        
        if let Some(assigned_to_id) = &quest.assigned_to_id {
            query.push_str(&format!(", assigned_to_id = ${}", param_count));
            params.push(assigned_to_id);
            param_count += 1;
        }
        
        if let Some(assigned_to_type) = &quest.assigned_to_type {
            query.push_str(&format!(", assigned_to_type = ${}", param_count));
            params.push(assigned_to_type);
            param_count += 1;
        }
        
        if let Some(skills_required) = &quest.skills_required {
            query.push_str(&format!(", skills_required = ${}", param_count));
            params.push(skills_required);
            param_count += 1;
        }
        
        if let Some(quest_type) = &quest.quest_type {
            query.push_str(&format!(", quest_type = ${}", param_count));
            params.push(quest_type);
            param_count += 1;
        }
        
        if let Some(creator_character_id) = &quest.creator_character_id {
            query.push_str(&format!(", creator_character_id = ${}", param_count));
            params.push(creator_character_id);
            param_count += 1;
        }
        
        if let Some(requestor_character_id) = &quest.requestor_character_id {
            query.push_str(&format!(", requestor_character_id = ${}", param_count));
            params.push(requestor_character_id);
            param_count += 1;
        }
        
        query.push_str(&format!(" WHERE id = ${} RETURNING id, title, description, status, xp_reward, gold_reward, epic_id, guild_id, 
                contact_character_id, difficulty, deadline, assigned_to_id, assigned_to_type, skills_required, quest_type,
                creator_character_id, requestor_character_id, created_at, updated_at", 
            param_count
        ));
        
        params.push(&id);
        
        let row = tx.query_one(
            &query,
            &params[..],
        ).await.map_err(DbError::from)?;
        
        // Update prerequisites if provided
        if let Some(prerequisite_ids) = &quest.prerequisite_quest_ids {
            // Delete existing prerequisites
            tx.execute(
                "DELETE FROM quest_prerequisites WHERE quest_id = $1",
                &[&id],
            ).await.map_err(DbError::from)?;
            
            // Add new prerequisites
            for &prereq_id in prerequisite_ids {
                tx.execute(
                    "INSERT INTO quest_prerequisites (quest_id, prerequisite_quest_id) VALUES ($1, $2)",
                    &[&id, &prereq_id],
                ).await.map_err(DbError::from)?;
            }
        }
        
        // Update subtasks if provided
        if let Some(subtasks) = &quest.subtasks {
            // Handle subtasks (we'll need the existing ones to determine updates vs. creations)
            let existing_subtasks = tx.query(
                "SELECT id FROM quest_subtasks WHERE quest_id = $1",
                &[&id],
            ).await.map_err(DbError::from)?;
            
            let existing_ids: Vec<Uuid> = existing_subtasks.iter()
                .map(|row| row.get::<_, Uuid>("id"))
                .collect();
            
            // Process each subtask
            for subtask in subtasks {
                if let Some(subtask_id) = subtask.id {
                    if existing_ids.contains(&subtask_id) {
                        // Update existing subtask
                        let mut subtask_query = String::from("UPDATE quest_subtasks SET updated_at = $1");
                        let mut subtask_params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
                        
                        subtask_params.push(&now);
                        let mut subtask_param_count = 2;
                        
                        if let Some(title) = &subtask.title {
                            subtask_query.push_str(&format!(", title = ${}", subtask_param_count));
                            subtask_params.push(title);
                            subtask_param_count += 1;
                        }
                        
                        if let Some(description) = &subtask.description {
                            subtask_query.push_str(&format!(", description = ${}", subtask_param_count));
                            subtask_params.push(description);
                            subtask_param_count += 1;
                        }
                        
                        if let Some(status) = &subtask.status {
                            subtask_query.push_str(&format!(", status = ${}", subtask_param_count));
                            subtask_params.push(status);
                            subtask_param_count += 1;
                        }
                        
                        subtask_query.push_str(&format!(" WHERE id = ${}", subtask_param_count));
                        subtask_params.push(&subtask_id);
                        
                        tx.execute(
                            &subtask_query,
                            &subtask_params[..],
                        ).await.map_err(DbError::from)?;
                    }
                } else {
                    // Create new subtask
                    let title = subtask.title.as_ref().unwrap_or(&"New Subtask".to_string());
                    let description = subtask.description.as_ref().unwrap_or(&"".to_string());
                    let status = subtask.status.as_ref().unwrap_or(&"todo".to_string());
                    
                    tx.execute(
                        "INSERT INTO quest_subtasks (quest_id, title, description, status, created_at, updated_at)
                        VALUES ($1, $2, $3, $4, $5, $5)",
                        &[&id, title, description, status, &now],
                    ).await.map_err(DbError::from)?;
                }
            }
        }
        
        // Commit the transaction
        tx.commit().await.map_err(DbError::from)?;
        
        Ok(Quest {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            status: row.get("status"),
            xp_reward: row.get("xp_reward"),
            gold_reward: row.get("gold_reward"),
            epic_id: row.get("epic_id"),
            guild_id: row.get("guild_id"),
            contact_character_id: row.get("contact_character_id"),
            difficulty: row.get("difficulty"),
            deadline: row.get("deadline"),
            assigned_to_id: row.get("assigned_to_id"),
            assigned_to_type: row.get("assigned_to_type"),
            skills_required: row.get("skills_required"),
            quest_type: row.get("quest_type"),
            creator_character_id: row.get("creator_character_id"),
            requestor_character_id: row.get("requestor_character_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
    
    pub async fn delete_quest(&self, id: Uuid) -> Result<(), DbError> {
        // Start a transaction
        let transaction = self.client.transaction().await
            .map_err(DbError::from)?;
            
        // Delete prerequisites
        transaction.execute(
            "DELETE FROM quest_prerequisites WHERE quest_id = $1 OR prerequisite_quest_id = $1",
            &[&id],
        ).await.map_err(DbError::from)?;
        
        // Delete subtasks
        transaction.execute(
            "DELETE FROM quest_subtasks WHERE quest_id = $1",
            &[&id],
        ).await.map_err(DbError::from)?;
        
        // Delete the quest
        let result = transaction.execute(
            "DELETE FROM quests WHERE id = $1",
            &[&id],
        ).await.map_err(DbError::from)?;
        
        if result == 0 {
            return Err(DbError::NoDataReturned);
        }
        
        // Commit the transaction
        transaction.commit().await.map_err(DbError::from)?;
        
        Ok(())
    }
    
    pub async fn add_prerequisite(&self, quest_id: Uuid, prerequisite_id: Uuid) -> Result<(), DbError> {
        // Check if both quests exist
        let _ = self.get_quest_by_id(quest_id).await?;
        let _ = self.get_quest_by_id(prerequisite_id).await?;
        
        // Check if the prerequisite already exists
        let existing = self.client.query_opt(
            "SELECT 1 FROM quest_prerequisites WHERE quest_id = $1 AND prerequisite_quest_id = $2",
            &[&quest_id, &prerequisite_id],
        ).await.map_err(DbError::from)?;
        
        if existing.is_some() {
            return Ok(());  // Prerequisite already exists
        }
        
        // Add the prerequisite
        self.client.execute(
            "INSERT INTO quest_prerequisites (quest_id, prerequisite_quest_id) VALUES ($1, $2)",
            &[&quest_id, &prerequisite_id],
        ).await.map_err(DbError::from)?;
        
        Ok(())
    }
    
    pub async fn remove_prerequisite(&self, quest_id: Uuid, prerequisite_id: Uuid) -> Result<(), DbError> {
        let result = self.client.execute(
            "DELETE FROM quest_prerequisites WHERE quest_id = $1 AND prerequisite_quest_id = $2",
            &[&quest_id, &prerequisite_id],
        ).await.map_err(DbError::from)?;
        
        if result == 0 {
            return Err(DbError::NoDataReturned);
        }
        
        Ok(())
    }
} 