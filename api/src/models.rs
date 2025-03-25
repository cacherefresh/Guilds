use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

// Quest models
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Quest {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(example = "Rescue the princess")]
    pub title: String,
    #[schema(example = "The princess has been captured by a dragon. Save her!")]
    pub description: String,
    #[schema(example = 5)]
    pub difficulty: i32,
    pub required_skills: Vec<Skill>,
    #[schema(example = "1000 gold coins")]
    pub reward: Option<String>,
    #[schema(example = 500)]
    pub xp_reward: i32,
    #[schema(example = 1000)]
    pub gold_reward: i32,
    #[schema(example = "AVAILABLE")]
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct QuestCreate {
    #[schema(example = "Rescue the princess")]
    pub title: String,
    #[schema(example = "The princess has been captured by a dragon. Save her!")]
    pub description: String,
    #[schema(example = 5)]
    pub difficulty: Option<i32>,
    #[schema(example = "[\"123e4567-e89b-12d3-a456-426614174000\", \"223e4567-e89b-12d3-a456-426614174000\"]")]
    pub required_skill_ids: Vec<Uuid>,
    #[schema(example = "1000 gold coins")]
    pub reward: Option<String>,
    #[schema(example = 500)]
    pub xp_reward: Option<i32>,
    #[schema(example = 1000)]
    pub gold_reward: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct QuestUpdate {
    #[schema(example = "Rescue the princess")]
    pub title: Option<String>,
    #[schema(example = "The princess has been captured by a dragon. Save her!")]
    pub description: Option<String>,
    #[schema(example = 5)]
    pub difficulty: Option<i32>,
    #[schema(example = "[\"123e4567-e89b-12d3-a456-426614174000\", \"223e4567-e89b-12d3-a456-426614174000\"]")]
    pub required_skill_ids: Option<Vec<Uuid>>,
    #[schema(example = "1000 gold coins")]
    pub reward: Option<String>,
    #[schema(example = 500)]
    pub xp_reward: Option<i32>,
    #[schema(example = 1000)]
    pub gold_reward: Option<i32>,
    #[schema(example = "ASSIGNED")]
    pub status: Option<String>,
}

// Skill models
#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

// Character models
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Character {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(example = "Gandalf")]
    pub name: String,
    #[schema(example = "MAIN")]
    pub type_: String,
    pub skills: Vec<Skill>,
    #[schema(example = 10)]
    pub level: i32,
    #[schema(example = 1500)]
    pub xp: i32,
    #[schema(example = "Wizards Guild")]
    pub guild: Option<String>,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub team_id: Option<Uuid>,
    pub properties: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CharacterCreate {
    #[schema(example = "Gandalf")]
    pub name: String,
    #[schema(example = "MAIN")]
    pub type_: String,
    #[schema(example = "[\"123e4567-e89b-12d3-a456-426614174000\", \"223e4567-e89b-12d3-a456-426614174000\"]")]
    pub skill_ids: Option<Vec<Uuid>>,
    #[schema(example = 10)]
    pub level: Option<i32>,
    #[schema(example = 1500)]
    pub xp: Option<i32>,
    #[schema(example = "Wizards Guild")]
    pub guild: Option<String>,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub team_id: Option<Uuid>,
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CharacterUpdate {
    #[schema(example = "Gandalf the Grey")]
    pub name: Option<String>,
    #[schema(example = 11)]
    pub level: Option<i32>,
    #[schema(example = 2000)]
    pub xp: Option<i32>,
    #[schema(example = "Wizards Guild of Middle-earth")]
    pub guild: Option<String>,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub team_id: Option<Uuid>,
    pub properties: Option<serde_json::Value>,
}

// Request and response helper models
#[derive(Debug, Deserialize, ToSchema)]
pub struct AddSkillsRequest {
    #[schema(example = "[\"123e4567-e89b-12d3-a456-426614174000\", \"223e4567-e89b-12d3-a456-426614174000\"]")]
    pub skill_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestsResponse {
    pub quests: Vec<Quest>,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AvailableQuestsResponse {
    pub quests: Vec<Quest>,
    pub skills_covered: Vec<Skill>,
} 