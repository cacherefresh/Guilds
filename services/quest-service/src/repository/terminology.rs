use tokio_postgres::{Client, Row};
use uuid::Uuid;
use chrono::Utc;
use log::{info, error};
use crate::models::terminology::{TerminologySetting, TerminologyCreate, TerminologyUpdate};
use crate::repository::db::DbError;

pub struct TerminologyRepository {
    client: Client,
}

impl TerminologyRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    
    pub async fn get_all_terminology_settings(&self) -> Result<Vec<TerminologySetting>, DbError> {
        let rows = self.client.query(
            "SELECT id, name, description, quest_term, epic_term, adventure_term, character_term, 
                   guild_term, skill_term, reward_term, is_current, created_at, updated_at 
            FROM terminology_settings 
            ORDER BY created_at DESC",
            &[],
        ).await.map_err(DbError::from)?;
        
        let settings = rows.iter()
            .map(|row| self.row_to_terminology_setting(row))
            .collect();
            
        Ok(settings)
    }
    
    pub async fn get_terminology_setting_by_id(&self, id: Uuid) -> Result<TerminologySetting, DbError> {
        let row = self.client.query_one(
            "SELECT id, name, description, quest_term, epic_term, adventure_term, character_term, 
                    guild_term, skill_term, reward_term, is_current, created_at, updated_at 
            FROM terminology_settings 
            WHERE id = $1",
            &[&id],
        ).await.map_err(|e| {
            if e.to_string().contains("no rows") {
                DbError::NoDataReturned
            } else {
                DbError::from(e)
            }
        })?;
        
        Ok(self.row_to_terminology_setting(&row))
    }
    
    pub async fn get_current_terminology(&self) -> Result<TerminologySetting, DbError> {
        let row = self.client.query_one(
            "SELECT id, name, description, quest_term, epic_term, adventure_term, character_term, 
                    guild_term, skill_term, reward_term, is_current, created_at, updated_at 
            FROM terminology_settings 
            WHERE is_current = true",
            &[],
        ).await.map_err(|e| {
            if e.to_string().contains("no rows") {
                DbError::NoDataReturned
            } else {
                DbError::from(e)
            }
        })?;
        
        Ok(self.row_to_terminology_setting(&row))
    }
    
    pub async fn create_terminology_setting(&self, setting: TerminologyCreate) -> Result<TerminologySetting, DbError> {
        // Start a transaction since we might need to update multiple rows
        let tx = self.client.transaction().await.map_err(DbError::from)?;
        
        // If this is set as current, update all other settings to not be current
        if setting.is_current.unwrap_or(false) {
            tx.execute(
                "UPDATE terminology_settings SET is_current = false",
                &[],
            ).await.map_err(DbError::from)?;
        }
        
        // Insert the new setting
        let row = tx.query_one(
            "INSERT INTO terminology_settings (
                name, description, quest_term, epic_term, adventure_term, character_term,
                guild_term, skill_term, reward_term, is_current, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $11)
            RETURNING id, name, description, quest_term, epic_term, adventure_term, character_term, 
                     guild_term, skill_term, reward_term, is_current, created_at, updated_at",
            &[
                &setting.name,
                &setting.description,
                &setting.quest_term,
                &setting.epic_term,
                &setting.adventure_term,
                &setting.character_term,
                &setting.guild_term,
                &setting.skill_term,
                &setting.reward_term,
                &setting.is_current.unwrap_or(false),
                &Utc::now(),
            ],
        ).await.map_err(DbError::from)?;
        
        // Commit the transaction
        tx.commit().await.map_err(DbError::from)?;
        
        Ok(self.row_to_terminology_setting(&row))
    }
    
    pub async fn update_terminology_setting(&self, id: Uuid, setting: TerminologyUpdate) -> Result<TerminologySetting, DbError> {
        // First check if the terminology setting exists
        let current = self.get_terminology_setting_by_id(id).await?;
        
        // Start a transaction since we might need to update multiple rows
        let tx = self.client.transaction().await.map_err(DbError::from)?;
        
        // If this is being set as current, update all other settings to not be current
        if let Some(is_current) = setting.is_current {
            if is_current {
                tx.execute(
                    "UPDATE terminology_settings SET is_current = false",
                    &[],
                ).await.map_err(DbError::from)?;
            }
        }
        
        // Build the update query
        let mut query = String::from("UPDATE terminology_settings SET updated_at = $1");
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        
        let now = Utc::now();
        params.push(&now);
        
        let mut param_count = 2;
        
        if let Some(name) = &setting.name {
            query.push_str(&format!(", name = ${}", param_count));
            params.push(name);
            param_count += 1;
        }
        
        if let Some(description) = &setting.description {
            query.push_str(&format!(", description = ${}", param_count));
            params.push(description);
            param_count += 1;
        }
        
        if let Some(quest_term) = &setting.quest_term {
            query.push_str(&format!(", quest_term = ${}", param_count));
            params.push(quest_term);
            param_count += 1;
        }
        
        if let Some(epic_term) = &setting.epic_term {
            query.push_str(&format!(", epic_term = ${}", param_count));
            params.push(epic_term);
            param_count += 1;
        }
        
        if let Some(adventure_term) = &setting.adventure_term {
            query.push_str(&format!(", adventure_term = ${}", param_count));
            params.push(adventure_term);
            param_count += 1;
        }
        
        if let Some(character_term) = &setting.character_term {
            query.push_str(&format!(", character_term = ${}", param_count));
            params.push(character_term);
            param_count += 1;
        }
        
        if let Some(guild_term) = &setting.guild_term {
            query.push_str(&format!(", guild_term = ${}", param_count));
            params.push(guild_term);
            param_count += 1;
        }
        
        if let Some(skill_term) = &setting.skill_term {
            query.push_str(&format!(", skill_term = ${}", param_count));
            params.push(skill_term);
            param_count += 1;
        }
        
        if let Some(reward_term) = &setting.reward_term {
            query.push_str(&format!(", reward_term = ${}", param_count));
            params.push(reward_term);
            param_count += 1;
        }
        
        if let Some(is_current) = &setting.is_current {
            query.push_str(&format!(", is_current = ${}", param_count));
            params.push(is_current);
            param_count += 1;
        }
        
        // Add WHERE clause and RETURNING
        query.push_str(&format!(" WHERE id = ${} 
            RETURNING id, name, description, quest_term, epic_term, adventure_term, character_term, 
                     guild_term, skill_term, reward_term, is_current, created_at, updated_at", 
            param_count
        ));
            
        params.push(&id);
        
        // Execute the update
        let row = tx.query_one(
            &query,
            &params[..],
        ).await.map_err(DbError::from)?;
        
        // Commit the transaction
        tx.commit().await.map_err(DbError::from)?;
        
        Ok(self.row_to_terminology_setting(&row))
    }
    
    pub async fn set_current_terminology(&self, id: Uuid) -> Result<(), DbError> {
        // Start a transaction
        let tx = self.client.transaction().await.map_err(DbError::from)?;
        
        // First set all settings to not current
        tx.execute(
            "UPDATE terminology_settings SET is_current = false",
            &[],
        ).await.map_err(DbError::from)?;
        
        // Then set the specified one as current
        let result = tx.execute(
            "UPDATE terminology_settings SET is_current = true, updated_at = $1 WHERE id = $2",
            &[&Utc::now(), &id],
        ).await.map_err(DbError::from)?;
        
        if result == 0 {
            tx.rollback().await.map_err(DbError::from)?;
            return Err(DbError::NoDataReturned);
        }
        
        // Commit the transaction
        tx.commit().await.map_err(DbError::from)?;
        
        Ok(())
    }
    
    pub async fn delete_terminology_setting(&self, id: Uuid) -> Result<(), DbError> {
        let result = self.client.execute(
            "DELETE FROM terminology_settings WHERE id = $1",
            &[&id],
        ).await.map_err(DbError::from)?;
        
        if result == 0 {
            return Err(DbError::NoDataReturned);
        }
        
        Ok(())
    }
    
    // Helper function to convert a Row to a TerminologySetting
    fn row_to_terminology_setting(&self, row: &Row) -> TerminologySetting {
        TerminologySetting {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            quest_term: row.get("quest_term"),
            epic_term: row.get("epic_term"),
            adventure_term: row.get("adventure_term"),
            character_term: row.get("character_term"),
            guild_term: row.get("guild_term"),
            skill_term: row.get("skill_term"),
            reward_term: row.get("reward_term"),
            is_current: row.get("is_current"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
} 