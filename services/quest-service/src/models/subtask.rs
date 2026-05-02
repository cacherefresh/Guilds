use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Subtask {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub quest_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SubtaskCreate {
    pub title: String,
    pub description: String,
    pub status: String,
    pub quest_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SubtaskUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubtasksResponse {
    pub subtasks: Vec<Subtask>,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SubtaskQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub status: Option<String>,
    pub quest_id: Option<Uuid>,
}

impl Default for SubtaskQuery {
    fn default() -> Self {
        Self {
            offset: Some(0),
            limit: Some(10),
            status: None,
            quest_id: None,
        }
    }
} 