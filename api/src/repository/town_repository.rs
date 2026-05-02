use crate::error::ApiError;
use crate::models::{Town, TownCreate, TownUpdate, TownSummary};
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;
use uuid::Uuid;
use std::sync::Arc;
use log::error;

pub struct TownRepository {
    pool: Arc<Pool>,
}

impl TownRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
    
    // Get all towns with pagination
    pub async fn get_towns(&self, limit: i64, offset: i64) -> Result<(Vec<TownSummary>, i64), ApiError> {
        let client = self.pool.get().await?;
        
        // Get total count
        let query = "SELECT COUNT(*) FROM guild_app.towns";
        let row = client.query_one(query, &[]).await?;
        let total_count = row.get::<_, i64>(0);
        
        // Get towns with pagination
        let query = "
            SELECT
                t.id,
                t.name,
                t.description,
                t.region,
                (SELECT COUNT(*) FROM guild_app.guilds WHERE town_id = t.id) as guild_count
            FROM
                guild_app.towns t
            ORDER BY
                t.name ASC
            LIMIT $1 OFFSET $2
        ";
        
        let rows = client.query(query, &[&limit, &offset]).await?;
        
        let towns = rows.into_iter().map(|row| {
            TownSummary {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                region: row.get("region"),
                guild_count: row.get("guild_count"),
            }
        }).collect();
        
        Ok((towns, total_count))
    }
    
    // Get a town by ID
    pub async fn get_town_by_id(&self, town_id: Uuid) -> Result<Town, ApiError> {
        let client = self.pool.get().await?;
        
        let query = "
            SELECT
                t.id,
                t.name,
                t.description,
                t.region,
                t.properties,
                t.created_at,
                t.updated_at,
                (SELECT COUNT(*) FROM guild_app.guilds WHERE town_id = t.id) as guild_count
            FROM
                guild_app.towns t
            WHERE
                t.id = $1
        ";
        
        let row = client.query_opt(query, &[&town_id]).await?;
        
        match row {
            Some(row) => {
                Ok(Town {
                    id: row.get("id"),
                    name: row.get("name"),
                    description: row.get("description"),
                    region: row.get("region"),
                    properties: row.get("properties"),
                    guild_count: Some(row.get::<_, i64>("guild_count") as i32),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
            },
            None => Err(ApiError::NotFoundError(format!("Town with ID {} not found", town_id))),
        }
    }
    
    // Search towns by name or region
    pub async fn search_towns(&self, query: &str) -> Result<Vec<TownSummary>, ApiError> {
        let client = self.pool.get().await?;
        
        let search_query = "
            SELECT
                t.id,
                t.name,
                t.description,
                t.region,
                (SELECT COUNT(*) FROM guild_app.guilds WHERE town_id = t.id) as guild_count
            FROM
                guild_app.towns t
            WHERE
                t.name ILIKE $1 OR t.region ILIKE $1
            ORDER BY
                t.name ASC
            LIMIT 20
        ";
        
        let search_pattern = format!("%{}%", query);
        let rows = client.query(search_query, &[&search_pattern]).await?;
        
        let towns = rows.into_iter().map(|row| {
            TownSummary {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                region: row.get("region"),
                guild_count: row.get("guild_count"),
            }
        }).collect();
        
        Ok(towns)
    }
    
    // Create a new town
    pub async fn create_town(&self, town: TownCreate) -> Result<Town, ApiError> {
        let client = self.pool.get().await?;
        
        // Check if a town with the same name already exists
        let exists = client
            .query_one(
                "SELECT COUNT(*) FROM guild_app.towns WHERE name = $1",
                &[&town.name]
            )
            .await?
            .get::<_, i64>(0) > 0;
            
        if exists {
            return Err(ApiError::ConflictError(format!("Town with name '{}' already exists", town.name)));
        }
        
        let query = "
            INSERT INTO guild_app.towns (name, description, region, properties)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, description, region, properties, created_at, updated_at
        ";
        
        let row = client.query_one(
            query,
            &[
                &town.name,
                &town.description,
                &town.region,
                &town.properties,
            ]
        ).await?;
        
        Ok(Town {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            region: row.get("region"),
            properties: row.get("properties"),
            guild_count: Some(0), // New town has no guilds
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
    
    // Update an existing town
    pub async fn update_town(&self, town_id: Uuid, town: TownUpdate) -> Result<Town, ApiError> {
        let client = self.pool.get().await?;
        
        // Check if the town exists
        let exists = client
            .query_one("SELECT COUNT(*) FROM guild_app.towns WHERE id = $1", &[&town_id])
            .await?
            .get::<_, i64>(0) > 0;
            
        if !exists {
            return Err(ApiError::NotFoundError(format!("Town with ID {} not found", town_id)));
        }
        
        // If name is provided, check for name conflicts
        if let Some(name) = &town.name {
            let name_taken = client
                .query_one(
                    "SELECT COUNT(*) FROM guild_app.towns WHERE name = $1 AND id != $2",
                    &[name, &town_id]
                )
                .await?
                .get::<_, i64>(0) > 0;
                
            if name_taken {
                return Err(ApiError::ConflictError(format!("Town with name '{}' already exists", name)));
            }
        }
        
        // Build and execute the update query
        let mut query = String::from("UPDATE guild_app.towns SET ");
        let mut params: Vec<Box<dyn ToSql + Sync>> = Vec::new();
        let mut param_idx = 1;
        
        if let Some(name) = &town.name {
            query.push_str(&format!("name = ${}, ", param_idx));
            params.push(Box::new(name.clone()));
            param_idx += 1;
        }
        
        if let Some(description) = &town.description {
            query.push_str(&format!("description = ${}, ", param_idx));
            params.push(Box::new(description.clone()));
            param_idx += 1;
        }
        
        if let Some(region) = &town.region {
            query.push_str(&format!("region = ${}, ", param_idx));
            params.push(Box::new(region.clone()));
            param_idx += 1;
        }
        
        if let Some(properties) = &town.properties {
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
            params.push(Box::new(town_id));
            
            // Create a slice of references to the ToSql trait objects
            let param_refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p.as_ref()).collect();
            
            // Execute the update
            client.execute(&query, &param_refs[..]).await?;
        }
        
        // Get the updated town
        self.get_town_by_id(town_id).await
    }
    
    // Delete a town
    pub async fn delete_town(&self, town_id: Uuid) -> Result<(), ApiError> {
        let client = self.pool.get().await?;
        
        // Check if the town exists
        let exists = client
            .query_one("SELECT COUNT(*) FROM guild_app.towns WHERE id = $1", &[&town_id])
            .await?
            .get::<_, i64>(0) > 0;
            
        if !exists {
            return Err(ApiError::NotFoundError(format!("Town with ID {} not found", town_id)));
        }
        
        // Check if there are guilds in this town
        let has_guilds = client
            .query_one("SELECT COUNT(*) FROM guild_app.guilds WHERE town_id = $1", &[&town_id])
            .await?
            .get::<_, i64>(0) > 0;
            
        if has_guilds {
            return Err(ApiError::ConflictError(
                format!("Cannot delete town with ID {} because it has guilds. Remove all guilds from the town first.", town_id)
            ));
        }
        
        // Delete the town
        client.execute("DELETE FROM guild_app.towns WHERE id = $1", &[&town_id]).await?;
        
        Ok(())
    }
    
    // Get towns by region
    pub async fn get_towns_by_region(&self, region: &str) -> Result<Vec<TownSummary>, ApiError> {
        let client = self.pool.get().await?;
        
        let query = "
            SELECT
                t.id,
                t.name,
                t.description,
                t.region,
                (SELECT COUNT(*) FROM guild_app.guilds WHERE town_id = t.id) as guild_count
            FROM
                guild_app.towns t
            WHERE
                t.region ILIKE $1
            ORDER BY
                t.name ASC
        ";
        
        let rows = client.query(query, &[&region]).await?;
        
        let towns = rows.into_iter().map(|row| {
            TownSummary {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                region: row.get("region"),
                guild_count: row.get("guild_count"),
            }
        }).collect();
        
        Ok(towns)
    }
} 