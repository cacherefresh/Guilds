use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Adventure {
    /// Unique identifier for the adventure
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
    /// Title of the adventure
    #[schema(example = "Dragon's Return")]
    pub title: String,
    
    /// Detailed description of the adventure
    #[schema(example = "A grand campaign involving the return of dragons to the realm")]
    pub description: String,
    
    /// Current status of the adventure (planning, in-progress, completed, cancelled)
    #[schema(example = "in-progress")]
    pub status: String,
    
    /// Total XP reward for all quests in this adventure
    #[schema(example = 500)]
    pub total_xp_reward: i32,
    
    /// Total gold reward for all quests in this adventure
    #[schema(example = 1000)]
    pub total_gold_reward: i32,
    
    /// Outline text for the adventure
    #[schema(example = "An outline of the adventure's major arcs and goals")]
    pub outline: Option<String>,
    
    /// List of components/skills/technologies used in this adventure
    #[schema(example = "[\"React\", \"TypeScript\", \"PostgreSQL\"]")]
    pub components: Option<Vec<String>>,
    
    /// When the adventure was created
    pub created_at: DateTime<Utc>,
    
    /// When the adventure was last updated
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AdventureCreate {
    /// Title of the adventure
    #[schema(example = "Dragon's Return")]
    pub title: String,
    
    /// Detailed description of the adventure
    #[schema(example = "A grand campaign involving the return of dragons to the realm")]
    pub description: String,
    
    /// Current status of the adventure (planning, in-progress, completed, cancelled)
    #[schema(example = "planning")]
    pub status: String,
    
    /// Outline text for the adventure
    #[schema(example = "An outline of the adventure's major arcs and goals")]
    pub outline: Option<String>,
    
    /// List of components/skills/technologies used in this adventure
    #[schema(example = "[\"React\", \"TypeScript\", \"PostgreSQL\"]")]
    pub components: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AdventureUpdate {
    /// Optional updated title of the adventure
    #[schema(example = "Dragon's Return")]
    pub title: Option<String>,
    
    /// Optional updated description of the adventure
    #[schema(example = "A grand campaign involving the return of dragons to the realm")]
    pub description: Option<String>,
    
    /// Optional updated status of the adventure
    #[schema(example = "in-progress")]
    pub status: Option<String>,
    
    /// Optional updated outline text for the adventure
    #[schema(example = "An updated outline of the adventure's major arcs and goals")]
    pub outline: Option<String>,
    
    /// Optional updated list of components/skills/technologies
    #[schema(example = "[\"React\", \"TypeScript\", \"PostgreSQL\", \"Redux\"]")]
    pub components: Option<Vec<String>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdventureListResponse {
    /// List of adventures
    pub adventures: Vec<Adventure>,
    
    /// Total number of adventures (for pagination)
    pub total: i64,
    
    /// Current offset (for pagination)
    pub offset: i64,
    
    /// Current limit (for pagination)
    pub limit: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdventureDetail {
    /// The adventure details
    pub adventure: Adventure,
    
    /// Epics belonging to this adventure
    pub epics: Vec<crate::models::epic::Epic>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AdventureQuery {
    /// Optional status to filter by
    #[schema(example = "in-progress")]
    pub status: Option<String>,
    
    /// Pagination offset
    #[schema(default = 0)]
    pub offset: Option<i64>,
    
    /// Pagination limit
    #[schema(default = 20)]
    pub limit: Option<i64>,
}

impl Default for AdventureQuery {
    fn default() -> Self {
        Self {
            offset: Some(0),
            limit: Some(10),
            status: None,
        }
    }
} 