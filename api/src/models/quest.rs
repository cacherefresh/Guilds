use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

use super::{Skill, CharacterSummary, GuildSummary};

// Quest status enum
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub enum QuestStatus {
    #[schema(example = "AVAILABLE")]
    Available,
    #[schema(example = "IN_PROGRESS")]
    InProgress,
    #[schema(example = "COMPLETED")]
    Completed,
    #[schema(example = "FAILED")]
    Failed,
    #[schema(example = "EXPIRED")]
    Expired,
}

impl QuestStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            QuestStatus::Available => "AVAILABLE",
            QuestStatus::InProgress => "IN_PROGRESS",
            QuestStatus::Completed => "COMPLETED",
            QuestStatus::Failed => "FAILED",
            QuestStatus::Expired => "EXPIRED",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "AVAILABLE" => Ok(QuestStatus::Available),
            "IN_PROGRESS" => Ok(QuestStatus::InProgress),
            "COMPLETED" => Ok(QuestStatus::Completed),
            "FAILED" => Ok(QuestStatus::Failed),
            "EXPIRED" => Ok(QuestStatus::Expired),
            _ => Err(format!("Invalid quest status: {}", s)),
        }
    }
}

// Quest models
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[schema(example = json!({
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "title": "Defeat the Dragon",
    "description": "A dangerous dragon is terrorizing the countryside",
    "difficulty": 5,
    "required_skills": [
        {
            "id": "223e4567-e89b-12d3-a456-426614174000",
            "name": "Fire Magic",
            "description": "The ability to control fire",
            "category": "Magic",
            "level": 3
        }
    ],
    "reward": "1000 gold, Dragon Scale Armor",
    "xp_reward": 500,
    "gold_reward": 1000,
    "status": "AVAILABLE",
    "assigned_character": null,
    "assigned_guild": null,
    "contact_character": null,
    "created_at": "2023-01-01T12:00:00Z",
    "updated_at": "2023-01-01T12:00:00Z"
}))]
pub struct Quest {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
    #[schema(example = "Defeat the Dragon")]
    pub title: String,
    
    #[schema(example = "A dangerous dragon is terrorizing the countryside")]
    pub description: String,
    
    #[schema(example = 5)]
    pub difficulty: i32,
    
    pub required_skills: Vec<Skill>,
    
    #[schema(example = "1000 gold, Dragon Scale Armor")]
    pub reward: String,
    
    #[schema(example = 500)]
    pub xp_reward: i32,
    
    #[schema(example = 1000)]
    pub gold_reward: i32,
    
    #[schema(example = "AVAILABLE")]
    pub status: String,
    
    /// Character assigned to the quest (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_character: Option<CharacterSummary>,
    
    /// Guild assigned to the quest (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_guild: Option<GuildSummary>,
    
    /// Character who is the point of contact (if assigned to both character and guild)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_character: Option<CharacterSummary>,
    
    /// Additional quest properties as JSON
    pub properties: Value,
    
    pub created_at: DateTime<Utc>,
    
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct QuestCreate {
    #[schema(example = "Defeat the Dragon")]
    pub title: String,
    
    #[schema(example = "A dangerous dragon is terrorizing the countryside")]
    pub description: String,
    
    #[schema(example = 5)]
    pub difficulty: i32,
    
    #[schema(example = "[\"223e4567-e89b-12d3-a456-426614174000\", \"323e4567-e89b-12d3-a456-426614174000\"]")]
    pub required_skill_ids: Vec<Uuid>,
    
    #[schema(example = "1000 gold, Dragon Scale Armor")]
    pub reward: String,
    
    #[schema(example = 500)]
    pub xp_reward: i32,
    
    #[schema(example = 1000)]
    pub gold_reward: i32,
    
    #[schema(example = "AVAILABLE")]
    pub status: Option<String>,
    
    /// Character ID to assign the quest to (optional)
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub assigned_character_id: Option<Uuid>,
    
    /// Guild ID to assign the quest to (optional)
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub assigned_guild_id: Option<Uuid>,
    
    /// Character ID who is the point of contact (required if both guild and character are assigned)
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub contact_character_id: Option<Uuid>,
    
    /// Additional quest properties as JSON
    #[schema(example = "{\"location\": \"Dragon Mountain\", \"time_limit_days\": 7}")]
    pub properties: Option<Value>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct QuestUpdate {
    #[schema(example = "Defeat the Ancient Dragon")]
    pub title: Option<String>,
    
    #[schema(example = "A dangerous ancient dragon is terrorizing the countryside")]
    pub description: Option<String>,
    
    #[schema(example = 7)]
    pub difficulty: Option<i32>,
    
    #[schema(example = "[\"223e4567-e89b-12d3-a456-426614174000\", \"323e4567-e89b-12d3-a456-426614174000\"]")]
    pub required_skill_ids: Option<Vec<Uuid>>,
    
    #[schema(example = "2000 gold, Dragon Scale Armor, Magic Staff")]
    pub reward: Option<String>,
    
    #[schema(example = 750)]
    pub xp_reward: Option<i32>,
    
    #[schema(example = 2000)]
    pub gold_reward: Option<i32>,
    
    #[schema(example = "IN_PROGRESS")]
    pub status: Option<String>,
    
    /// Character ID to assign the quest to (null to remove)
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub assigned_character_id: Option<Uuid>,
    
    /// Guild ID to assign the quest to (null to remove)
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub assigned_guild_id: Option<Uuid>,
    
    /// Character ID who is the point of contact (null to remove)
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub contact_character_id: Option<Uuid>,
    
    /// Additional quest properties as JSON (will be merged with existing)
    #[schema(example = "{\"location\": \"Ancient Dragon Mountain\", \"time_limit_days\": 10}")]
    pub properties: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct QuestSummary {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
    #[schema(example = "Defeat the Dragon")]
    pub title: String,
    
    #[schema(example = "A dangerous dragon is terrorizing the countryside")]
    pub description: String,
    
    #[schema(example = 5)]
    pub difficulty: i32,
    
    #[schema(example = "1000 gold, Dragon Scale Armor")]
    pub reward: String,
    
    #[schema(example = 500)]
    pub xp_reward: i32,
    
    #[schema(example = 1000)]
    pub gold_reward: i32,
    
    #[schema(example = "AVAILABLE")]
    pub status: String,
    
    /// Name of the character assigned to the quest (if any)
    #[schema(example = "Aragorn")]
    pub assigned_character_name: Option<String>,
    
    /// Name of the guild assigned to the quest (if any)
    #[schema(example = "Black Dragon Guild")]
    pub assigned_guild_name: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestListResponse {
    pub quests: Vec<QuestSummary>,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
}

// Quest completion request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CompleteQuestRequest {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub quest_id: Uuid,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub character_id: Uuid,
    
    #[schema(example = true)]
    pub success: bool,
    
    #[schema(example = "Defeated the dragon by exploiting its weakness to water")]
    pub completion_notes: Option<String>,
}

// Available quests response
#[derive(Debug, Serialize, ToSchema)]
pub struct AvailableQuestsResponse {
    pub quests: Vec<QuestSummary>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestSkillMatch {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub quest_id: Uuid,
    #[schema(example = "Rescue the princess")]
    pub title: String,
    #[schema(example = "The princess has been captured by a dragon. Save her!")]
    pub description: String,
    #[schema(example = 5)]
    pub difficulty: i32,
    #[schema(example = 3)]
    pub matching_skills_count: i64,
    #[schema(example = 5)]
    pub total_required_skills: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GuildQuestsResponse {
    pub quests: Vec<QuestSkillMatch>,
    pub guild_id: Uuid,
} 