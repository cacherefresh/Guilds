use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

// Full Town model with all details
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct Town {
    /// Unique identifier for the town
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    
    /// Name of the town
    #[schema(example = "Waterdeep")]
    pub name: String,
    
    /// Town description
    #[schema(example = "A bustling coastal city known for its commerce and adventuring opportunities")]
    pub description: String,
    
    /// Region where the town is located
    #[schema(example = "Sword Coast")]
    pub region: String,
    
    /// Additional town properties as JSON
    pub properties: Value,
    
    /// Number of guilds in this town (if requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_count: Option<i32>,
    
    /// When the town was created
    pub created_at: DateTime<Utc>,
    
    /// When the town was last updated
    pub updated_at: DateTime<Utc>,
}

// Simplified town data for listing purposes
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct TownSummary {
    /// Unique identifier for the town
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    
    /// Name of the town
    #[schema(example = "Waterdeep")]
    pub name: String,
    
    /// Town description
    #[schema(example = "A bustling coastal city known for its commerce and adventuring opportunities")]
    pub description: String,
    
    /// Region where the town is located
    #[schema(example = "Sword Coast")]
    pub region: String,
    
    /// Number of guilds in this town
    #[schema(example = 12)]
    pub guild_count: i64,
}

// For creating a new town
#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct TownCreate {
    /// Name of the town
    #[schema(example = "Waterdeep")]
    pub name: String,
    
    /// Town description
    #[schema(example = "A bustling coastal city known for its commerce and adventuring opportunities")]
    pub description: String,
    
    /// Region where the town is located
    #[schema(example = "Sword Coast")]
    pub region: String,
    
    /// Additional town properties as JSON
    #[schema(example = "{ \"population\": 100000, \"specialty\": \"commerce\" }")]
    pub properties: Value,
}

// For updating an existing town
#[derive(Deserialize, ToSchema, Clone, Debug)]
pub struct TownUpdate {
    /// Name of the town
    #[schema(example = "Waterdeep")]
    pub name: Option<String>,
    
    /// Town description
    #[schema(example = "A bustling coastal city known for its commerce and adventuring opportunities")]
    pub description: Option<String>,
    
    /// Region where the town is located
    #[schema(example = "Sword Coast")]
    pub region: Option<String>,
    
    /// Additional town properties as JSON
    #[schema(example = "{ \"population\": 100000, \"specialty\": \"commerce\" }")]
    pub properties: Option<Value>,
}

// Town response with pagination
#[derive(Serialize, ToSchema, Clone, Debug)]
pub struct TownListResponse {
    /// List of towns
    pub towns: Vec<TownSummary>,
    
    /// Total count of towns
    #[schema(example = 50)]
    pub total: i64,
    
    /// Current offset
    #[schema(example = 0)]
    pub offset: i64,
    
    /// Current limit
    #[schema(example = 10)]
    pub limit: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TownWithGuilds {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(example = "Oakvale")]
    pub name: String,
    #[schema(example = "A bustling market town with many guilds")]
    pub description: Option<String>,
    pub guilds_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
} 