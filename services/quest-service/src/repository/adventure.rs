use tokio_postgres::{Client, Row};
use uuid::Uuid;
use chrono::Utc;
use log::{info, error};
use crate::models::adventure::{Adventure, AdventureCreate, AdventureUpdate};
use crate::repository::db::DbError;

pub struct AdventureRepository {
    client: Client,
}

impl AdventureRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    
    pub async fn get_adventures(
        &self,
        offset: i64,
        limit: i64,
        status: Option<&str>,
    ) -> Result<(Vec<Adventure>, i64), DbError> {
        let mut query = String::from(
            "SELECT id, title, description, status, created_at, updated_at 
            FROM adventures 
            WHERE 1=1"
        );
        
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_count = 1;
        
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
        
        let adventures = rows.iter()
            .map(|row| self.row_to_adventure(row))
            .collect();
            
        Ok((adventures, total))
    }
    
    pub async fn get_adventure_by_id(&self, id: Uuid) -> Result<Adventure, DbError> {
        let row = self.client.query_one(
            "SELECT id, title, description, status, created_at, updated_at 
            FROM adventures 
            WHERE id = $1",
            &[&id],
        ).await.map_err(|e| {
            if e.to_string().contains("no rows") {
                DbError::NoDataReturned
            } else {
                DbError::from(e)
            }
        })?;
        
        Ok(self.row_to_adventure(&row))
    }
    
    pub async fn create_adventure(&self, adventure: AdventureCreate) -> Result<Adventure, DbError> {
        let row = self.client.query_one(
            "INSERT INTO adventures (title, description, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4)
            RETURNING id, title, description, status, created_at, updated_at",
            &[
                &adventure.title,
                &adventure.description,
                &adventure.status,
                &Utc::now(),
            ],
        ).await.map_err(DbError::from)?;
        
        Ok(self.row_to_adventure(&row))
    }
    
    pub async fn update_adventure(&self, id: Uuid, adventure: AdventureUpdate) -> Result<Adventure, DbError> {
        // First check if the adventure exists
        let current = self.get_adventure_by_id(id).await?;
        
        // Build the update query
        let mut query = String::from("UPDATE adventures SET updated_at = $1");
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        
        let now = Utc::now();
        params.push(&now);
        
        let mut param_count = 2;
        
        if let Some(title) = &adventure.title {
            query.push_str(&format!(", title = ${}", param_count));
            params.push(title);
            param_count += 1;
        }
        
        if let Some(description) = &adventure.description {
            query.push_str(&format!(", description = ${}", param_count));
            params.push(description);
            param_count += 1;
        }
        
        if let Some(status) = &adventure.status {
            query.push_str(&format!(", status = ${}", param_count));
            params.push(status);
            param_count += 1;
        }
        
        // Add WHERE clause and RETURNING
        query.push_str(&format!(" WHERE id = ${} RETURNING id, title, description, status, created_at, updated_at", param_count));
            
        params.push(&id);
        
        let row = self.client.query_one(
            &query,
            &params[..],
        ).await.map_err(DbError::from)?;
        
        Ok(self.row_to_adventure(&row))
    }
    
    pub async fn delete_adventure(&self, id: Uuid) -> Result<(), DbError> {
        // Check if any epics belong to this adventure
        let count_row = self.client.query_one(
            "SELECT COUNT(*) FROM epics WHERE adventure_id = $1",
            &[&id],
        ).await.map_err(DbError::from)?;
        
        let epic_count: i64 = count_row.get(0);
        
        if epic_count > 0 {
            return Err(DbError::Other(format!(
                "Cannot delete adventure with ID {} because it has {} epics associated with it", 
                id, epic_count
            )));
        }
        
        let result = self.client.execute(
            "DELETE FROM adventures WHERE id = $1",
            &[&id],
        ).await.map_err(DbError::from)?;
        
        if result == 0 {
            return Err(DbError::NoDataReturned);
        }
        
        Ok(())
    }
    
    // Helper function to convert a Row to an Adventure
    fn row_to_adventure(&self, row: &Row) -> Adventure {
        Adventure {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
} 