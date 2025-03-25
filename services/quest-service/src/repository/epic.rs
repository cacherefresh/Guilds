use tokio_postgres::{Client, Row};
use uuid::Uuid;
use chrono::Utc;
use log::{info, error};
use crate::models::epic::{Epic, EpicCreate, EpicUpdate};
use crate::repository::db::DbError;

pub struct EpicRepository {
    client: Client,
}

impl EpicRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    
    pub async fn get_epics(
        &self,
        offset: i64,
        limit: i64,
        status: Option<&str>,
        adventure_id: Option<Uuid>,
    ) -> Result<(Vec<Epic>, i64), DbError> {
        let mut query = String::from(
            "SELECT id, title, description, status, adventure_id, created_at, updated_at 
            FROM epics 
            WHERE 1=1"
        );
        
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_count = 1;
        
        if let Some(stat) = status {
            query.push_str(&format!(" AND status = ${}", param_count));
            params.push(&stat);
            param_count += 1;
        }
        
        if let Some(adv_id) = adventure_id {
            query.push_str(&format!(" AND adventure_id = ${}", param_count));
            params.push(&adv_id);
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
        
        let epics = rows.iter()
            .map(|row| self.row_to_epic(row))
            .collect();
            
        Ok((epics, total))
    }
    
    pub async fn get_epic_by_id(&self, id: Uuid) -> Result<Epic, DbError> {
        let row = self.client.query_one(
            "SELECT id, title, description, status, adventure_id, created_at, updated_at 
            FROM epics 
            WHERE id = $1",
            &[&id],
        ).await.map_err(|e| {
            if e.to_string().contains("no rows") {
                DbError::NoDataReturned
            } else {
                DbError::from(e)
            }
        })?;
        
        Ok(self.row_to_epic(&row))
    }
    
    pub async fn create_epic(&self, epic: EpicCreate) -> Result<Epic, DbError> {
        let row = self.client.query_one(
            "INSERT INTO epics (title, description, status, adventure_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $5)
            RETURNING id, title, description, status, adventure_id, created_at, updated_at",
            &[
                &epic.title,
                &epic.description,
                &epic.status,
                &epic.adventure_id,
                &Utc::now(),
            ],
        ).await.map_err(DbError::from)?;
        
        Ok(self.row_to_epic(&row))
    }
    
    pub async fn update_epic(&self, id: Uuid, epic: EpicUpdate) -> Result<Epic, DbError> {
        // First check if the epic exists
        let current = self.get_epic_by_id(id).await?;
        
        // Build the update query
        let mut query = String::from("UPDATE epics SET updated_at = $1");
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        
        let now = Utc::now();
        params.push(&now);
        
        let mut param_count = 2;
        
        if let Some(title) = &epic.title {
            query.push_str(&format!(", title = ${}", param_count));
            params.push(title);
            param_count += 1;
        }
        
        if let Some(description) = &epic.description {
            query.push_str(&format!(", description = ${}", param_count));
            params.push(description);
            param_count += 1;
        }
        
        if let Some(status) = &epic.status {
            query.push_str(&format!(", status = ${}", param_count));
            params.push(status);
            param_count += 1;
        }
        
        if let Some(adventure_id) = &epic.adventure_id {
            query.push_str(&format!(", adventure_id = ${}", param_count));
            params.push(adventure_id);
            param_count += 1;
        }
        
        // Add WHERE clause and RETURNING
        query.push_str(&format!(" WHERE id = ${} RETURNING id, title, description, status, adventure_id, created_at, updated_at", param_count));
            
        params.push(&id);
        
        let row = self.client.query_one(
            &query,
            &params[..],
        ).await.map_err(DbError::from)?;
        
        Ok(self.row_to_epic(&row))
    }
    
    pub async fn delete_epic(&self, id: Uuid) -> Result<(), DbError> {
        // Check if any quests belong to this epic
        let count_row = self.client.query_one(
            "SELECT COUNT(*) FROM quests WHERE epic_id = $1",
            &[&id],
        ).await.map_err(DbError::from)?;
        
        let quest_count: i64 = count_row.get(0);
        
        if quest_count > 0 {
            return Err(DbError::Other(format!(
                "Cannot delete epic with ID {} because it has {} quests associated with it", 
                id, quest_count
            )));
        }
        
        let result = self.client.execute(
            "DELETE FROM epics WHERE id = $1",
            &[&id],
        ).await.map_err(DbError::from)?;
        
        if result == 0 {
            return Err(DbError::NoDataReturned);
        }
        
        Ok(())
    }
    
    // Helper function to convert a Row to an Epic
    fn row_to_epic(&self, row: &Row) -> Epic {
        Epic {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            status: row.get("status"),
            adventure_id: row.get("adventure_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
} 