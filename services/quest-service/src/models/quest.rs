use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use postgres_types::FromSql;
use utoipa::ToSchema;
use crate::models::subtask::Subtask;

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
    pub epic_id: Option<Uuid>,
    pub guild_id: Option<Uuid>,
    pub contact_character_id: Option<Uuid>,
    pub assigned_to_id: Option<Uuid>,
    pub assigned_to_type: Option<String>,
    pub skills_required: Option<Vec<String>>,
    pub quest_type: Option<String>,
    pub creator_character_id: Option<Uuid>,
    pub requestor_character_id: Option<Uuid>,
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
    pub epic_id: Option<Uuid>,
    pub guild_id: Option<Uuid>,
    pub contact_character_id: Option<Uuid>,
    pub assigned_to_id: Option<Uuid>,
    pub assigned_to_type: Option<String>,
    pub skills_required: Option<Vec<String>>,
    pub quest_type: Option<String>,
    pub creator_character_id: Option<Uuid>,
    pub requestor_character_id: Option<Uuid>,
    pub prerequisite_quest_ids: Option<Vec<Uuid>>,
    pub subtasks: Option<Vec<String>>,
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
    pub epic_id: Option<Uuid>,
    pub guild_id: Option<Uuid>,
    pub contact_character_id: Option<Uuid>,
    pub assigned_to_id: Option<Uuid>,
    pub assigned_to_type: Option<String>,
    pub skills_required: Option<Vec<String>>,
    pub quest_type: Option<String>,
    pub creator_character_id: Option<Uuid>,
    pub requestor_character_id: Option<Uuid>,
    pub prerequisite_quest_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestListResponse {
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
    pub epic_id: Option<Uuid>,
    pub guild_id: Option<Uuid>,
    pub contact_character_id: Option<Uuid>,
    pub assigned_to_id: Option<Uuid>,
    pub assigned_to_type: Option<String>,
    pub quest_type: Option<String>,
    pub creator_character_id: Option<Uuid>,
    pub requestor_character_id: Option<Uuid>,
    pub skill_required: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestDetail {
    pub quest: Quest,
    pub prerequisites: Vec<QuestPrerequisite>,
    pub prerequisite_for: Vec<QuestPrerequisite>,
    pub subtasks: Vec<Subtask>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct QuestPrerequisite {
    pub quest_id: Uuid,
    pub prerequisite_quest_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PrerequisiteRequest {
    pub quest_id: Uuid,
    pub prerequisite_quest_id: Uuid,
}

impl Default for QuestQuery {
    fn default() -> Self {
        Self {
            offset: Some(0),
            limit: Some(10),
            difficulty: None,
            status: None,
            epic_id: None,
            guild_id: None,
            contact_character_id: None,
            assigned_to_id: None,
            assigned_to_type: None,
            quest_type: None,
            creator_character_id: None,
            requestor_character_id: None,
            skill_required: None,
        }
    }
} 