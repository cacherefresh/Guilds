use tokio_postgres::{Client, Row};
use uuid::Uuid;
use chrono::Utc;
use log::{info, error};
use crate::models::quest::{Quest, QuestCreate, QuestUpdate};
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
        difficulty: Option<&str>,
        status: Option<&str>,
    ) -> Result<(Vec<Quest>, i64), DbError> {
        let mut query = String::from(
            "SELECT id, title, description, difficulty, required_skills, reward, 
            xp_reward, gold_reward, status, created_at, updated_at 
            FROM quests 
            WHERE 1=1"
        );
        
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_count = 1;
        
        if let Some(diff) = difficulty {
            query.push_str(&format!(" AND difficulty = ${}", param_count));
            params.push(&diff);
            param_count += 1;
        }
        
        if let Some(stat) = status {
            query.push_str(&format!(" AND status = ${}", param_count));
            params.push(&stat);
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
        
        let quests = rows.iter()
            .map(|row| self.row_to_quest(row))
            .collect();
            
        Ok((quests, total))
    }
    
    pub async fn get_quest_by_id(&self, id: Uuid) -> Result<Quest, DbError> {
        let row = self.client.query_one(
            "SELECT id, title, description, difficulty, required_skills, reward, 
            xp_reward, gold_reward, status, created_at, updated_at 
            FROM quests 
            WHERE id = $1",
            &[&id],
        ).await.map_err(|e| {
            if e.to_string().contains("no rows") {
                DbError::NoDataReturned
            } else {
                DbError::from(e)
            }
        })?;
        
        Ok(self.row_to_quest(&row))
    }
    
    pub async fn create_quest(&self, quest: QuestCreate) -> Result<Quest, DbError> {
        let row = self.client.query_one(
            "INSERT INTO quests (title, description, difficulty, required_skills, reward, 
            xp_reward, gold_reward, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'AVAILABLE', $8, $8)
            RETURNING id, title, description, difficulty, required_skills, reward, 
            xp_reward, gold_reward, status, created_at, updated_at",
            &[
                &quest.title,
                &quest.description,
                &quest.difficulty,
                &quest.required_skills,
                &quest.reward,
                &quest.xp_reward,
                &quest.gold_reward,
                &Utc::now(),
            ],
        ).await.map_err(DbError::from)?;
        
        Ok(self.row_to_quest(&row))
    }
    
    pub async fn update_quest(&self, id: Uuid, quest: QuestUpdate) -> Result<Quest, DbError> {
        // First check if the quest exists
        let current = self.get_quest_by_id(id).await?;
        
        // Build the update query
        let mut query = String::from("UPDATE quests SET updated_at = $1");
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        
        let now = Utc::now();
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
        
        if let Some(difficulty) = &quest.difficulty {
            query.push_str(&format!(", difficulty = ${}", param_count));
            params.push(difficulty);
            param_count += 1;
        }
        
        if let Some(required_skills) = &quest.required_skills {
            query.push_str(&format!(", required_skills = ${}", param_count));
            params.push(required_skills);
            param_count += 1;
        }
        
        if let Some(reward) = &quest.reward {
            query.push_str(&format!(", reward = ${}", param_count));
            params.push(reward);
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
        
        if let Some(status) = &quest.status {
            query.push_str(&format!(", status = ${}", param_count));
            params.push(status);
            param_count += 1;
        }
        
        // Add WHERE clause and RETURNING
        query.push_str(&format!(" WHERE id = ${} RETURNING id, title, description, difficulty, required_skills, reward, 
            xp_reward, gold_reward, status, created_at, updated_at", param_count));
            
        params.push(&id);
        
        let row = self.client.query_one(
            &query,
            &params[..],
        ).await.map_err(DbError::from)?;
        
        Ok(self.row_to_quest(&row))
    }
    
    pub async fn delete_quest(&self, id: Uuid) -> Result<(), DbError> {
        let result = self.client.execute(
            "DELETE FROM quests WHERE id = $1",
            &[&id],
        ).await.map_err(DbError::from)?;
        
        if result == 0 {
            return Err(DbError::NoDataReturned);
        }
        
        Ok(())
    }
    
    // Helper function to convert a Row to a Quest
    fn row_to_quest(&self, row: &Row) -> Quest {
        Quest {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            difficulty: row.get("difficulty"),
            required_skills: row.get("required_skills"),
            reward: row.get("reward"),
            xp_reward: row.get("xp_reward"),
            gold_reward: row.get("gold_reward"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
} 