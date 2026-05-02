use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Skill {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(example = "Lockpicking")]
    pub name: String,
    #[schema(example = "Ability to open locks without a key")]
    pub description: Option<String>,
    #[schema(example = "STEALTH")]
    pub category: String,
    #[schema(example = 5)]
    pub level: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SkillCreate {
    #[schema(example = "Lockpicking")]
    pub name: String,
    #[schema(example = "Ability to open locks without a key")]
    pub description: Option<String>,
    #[schema(example = "STEALTH")]
    pub category: String,
    #[schema(example = 5)]
    pub level: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SkillListResponse {
    pub skills: Vec<Skill>,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
} 