use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use postgres_types::FromSql;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Quest {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub required_skills: Vec<String>,
    pub reward: String,
    pub xp_reward: i32,
    pub gold_reward: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct QuestCreate {
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub required_skills: Vec<String>,
    pub reward: String,
    pub xp_reward: i32,
    pub gold_reward: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct QuestUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub required_skills: Option<Vec<String>>,
    pub reward: Option<String>,
    pub xp_reward: Option<i32>,
    pub gold_reward: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestsResponse {
    pub quests: Vec<Quest>,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct QuestQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub difficulty: Option<String>,
    pub status: Option<String>,
}

impl Default for QuestQuery {
    fn default() -> Self {
        Self {
            offset: Some(0),
            limit: Some(10),
            difficulty: None,
            status: None,
        }
    }
} 