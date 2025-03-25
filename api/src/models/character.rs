use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use utoipa::ToSchema;

use super::{Skill, GuildMembership};

// Character types enum
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub enum CharacterType {
    #[schema(example = "PLAYER")]
    Player,
    #[schema(example = "NPC")]
    Npc,
    #[schema(example = "MONSTER")]
    Monster,
    #[schema(example = "AI_TEAMMATE")]
    AiTeammate,
}

impl CharacterType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CharacterType::Player => "PLAYER",
            CharacterType::Npc => "NPC",
            CharacterType::Monster => "MONSTER",
            CharacterType::AiTeammate => "AI_TEAMMATE",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "PLAYER" => Ok(CharacterType::Player),
            "NPC" => Ok(CharacterType::Npc),
            "MONSTER" => Ok(CharacterType::Monster),
            "AI_TEAMMATE" => Ok(CharacterType::AiTeammate),
            _ => Err(format!("Invalid character type: {}", s)),
        }
    }
}

// Character models
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Character {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub id: Uuid,
    
    #[schema(example = "Aragorn")]
    pub name: String,
    
    #[schema(example = "warrior")]
    pub type_: String,
    
    pub skills: Vec<Skill>,
    
    #[schema(example = 5)]
    pub level: i32,
    
    #[schema(example = 2500)]
    pub xp: i32,
    
    /// ID of the character's active guild
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub active_guild_id: Option<Uuid>,
    
    /// Active guild details (if requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_guild: Option<GuildReference>,
    
    /// All guild memberships (if requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_memberships: Option<Vec<GuildMembership>>,
    
    /// ID of the character's team
    #[schema(example = "123e4567-e89b-12d3-a456-426614174005")]
    pub team_id: Option<Uuid>,
    
    /// Additional character properties as JSON
    pub properties: Value,
    
    pub created_at: DateTime<Utc>,
    
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CharacterCreate {
    #[schema(example = "Aragorn")]
    pub name: String,
    
    #[schema(example = "warrior")]
    pub type_: String,
    
    #[schema(example = "[\"123e4567-e89b-12d3-a456-426614174010\", \"123e4567-e89b-12d3-a456-426614174011\"]")]
    pub skills: Option<Vec<Uuid>>,
    
    #[schema(example = 1)]
    pub level: Option<i32>,
    
    #[schema(example = 0)]
    pub xp: Option<i32>,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub guild_id: Option<Uuid>,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174005")]
    pub team_id: Option<Uuid>,
    
    #[schema(example = "{\"strength\": 10, \"dexterity\": 8, \"constitution\": 12, \"intelligence\": 14, \"wisdom\": 10, \"charisma\": 16}")]
    pub properties: Option<Value>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CharacterUpdate {
    #[schema(example = "Aragorn the Ranger")]
    pub name: Option<String>,
    
    #[schema(example = "ranger")]
    pub type_: Option<String>,
    
    #[schema(example = "6")]
    pub level: Option<i32>,
    
    #[schema(example = "3500")]
    pub xp: Option<i32>,
    
    #[schema(example = "123e4567-e89b-12d3-a456-426614174005")]
    pub team_id: Option<Uuid>,
    
    #[schema(example = "{\"strength\": 12, \"dexterity\": 14, \"constitution\": 12, \"intelligence\": 14, \"wisdom\": 12, \"charisma\": 16}")]
    pub properties: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct CharacterSummary {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub id: Uuid,
    
    #[schema(example = "Aragorn")]
    pub name: String,
    
    #[schema(example = "warrior")]
    pub type_: String,
    
    #[schema(example = 5)]
    pub level: i32,
    
    /// Active guild name (if character is in a guild)
    #[schema(example = "Black Dragon Guild")]
    pub guild: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CharacterListResponse {
    pub characters: Vec<CharacterSummary>,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
}

// Guild reference for character model
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct GuildReference {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
    #[schema(example = "Black Dragon Guild")]
    pub name: String,
    
    #[schema(example = "A guild of skilled adventurers specializing in dragon hunting")]
    pub description: Option<String>,
}

// Request and response helper models
#[derive(Debug, Deserialize, ToSchema)]
pub struct AddSkillsRequest {
    #[schema(example = "[\"123e4567-e89b-12d3-a456-426614174000\", \"223e4567-e89b-12d3-a456-426614174000\"]")]
    pub skill_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CharacterSkillsResponse {
    pub skills: Vec<Skill>,
    pub character_id: Uuid,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CharacterGuildsResponse {
    pub guilds: Vec<GuildMembership>,
    pub character_id: Uuid,
} 