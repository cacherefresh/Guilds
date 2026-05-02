use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Epic {
    /// Unique identifier for the epic
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
    /// Title of the epic
    #[schema(example = "Dragon Invasion")]
    pub title: String,
    
    /// Detailed description of the epic
    #[schema(example = "A series of quests related to the dragon invasion of the kingdom")]
    pub description: String,
    
    /// Current status of the epic (planning, in-progress, completed, cancelled)
    #[schema(example = "in-progress")]
    pub status: String,
    
    /// Optional ID of the adventure this epic belongs to
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub adventure_id: Option<Uuid>,
    
    /// When the epic was created
    pub created_at: DateTime<Utc>,
    
    /// When the epic was last updated
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EpicCreate {
    /// Title of the epic
    #[schema(example = "Dragon Invasion")]
    pub title: String,
    
    /// Detailed description of the epic
    #[schema(example = "A series of quests related to the dragon invasion of the kingdom")]
    pub description: String,
    
    /// Current status of the epic (planning, in-progress, completed, cancelled)
    #[schema(example = "planning")]
    pub status: String,
    
    /// Optional ID of the adventure this epic belongs to
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub adventure_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EpicUpdate {
    /// Optional updated title of the epic
    #[schema(example = "Dragon Invasion")]
    pub title: Option<String>,
    
    /// Optional updated description of the epic
    #[schema(example = "A series of quests related to the dragon invasion of the kingdom")]
    pub description: Option<String>,
    
    /// Optional updated status of the epic
    #[schema(example = "in-progress")]
    pub status: Option<String>,
    
    /// Optional updated adventure ID
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub adventure_id: Option<Uuid>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EpicListResponse {
    /// List of epics
    pub epics: Vec<Epic>,
    
    /// Total number of epics (for pagination)
    pub total: i64,
    
    /// Current offset (for pagination)
    pub offset: i64,
    
    /// Current limit (for pagination)
    pub limit: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EpicDetail {
    /// The epic details
    pub epic: Epic,
    
    /// Quests belonging to this epic
    pub quests: Vec<crate::models::quest::Quest>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EpicQuery {
    /// Optional status to filter by
    #[schema(example = "in-progress")]
    pub status: Option<String>,
    
    /// Optional adventure ID to filter by
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub adventure_id: Option<Uuid>,
    
    /// Pagination offset
    #[schema(default = 0)]
    pub offset: Option<i64>,
    
    /// Pagination limit
    #[schema(default = 20)]
    pub limit: Option<i64>,
}

impl Default for EpicQuery {
    fn default() -> Self {
        Self {
            offset: Some(0),
            limit: Some(10),
            status: None,
            adventure_id: None,
        }
    }
} 