use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TerminologySetting {
    /// Unique identifier for the terminology setting
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
    /// Name of the terminology setting
    #[schema(example = "Fantasy Terms")]
    pub name: String,
    
    /// Description of the terminology setting
    #[schema(example = "Fantasy-themed terminology for the application")]
    pub description: String,
    
    /// Term used for quests
    #[schema(example = "Quest")]
    pub quest_term: String,
    
    /// Term used for epics
    #[schema(example = "Epic")]
    pub epic_term: String,
    
    /// Term used for adventures
    #[schema(example = "Adventure")]
    pub adventure_term: String,
    
    /// Term used for characters
    #[schema(example = "Hero")]
    pub character_term: String,
    
    /// Term used for guilds
    #[schema(example = "Guild")]
    pub guild_term: String,
    
    /// Term used for skills
    #[schema(example = "Skill")]
    pub skill_term: String,
    
    /// Term used for rewards
    #[schema(example = "Reward")]
    pub reward_term: String,
    
    /// Whether this is the current active terminology
    pub is_current: bool,
    
    /// When the terminology setting was created
    pub created_at: DateTime<Utc>,
    
    /// When the terminology setting was last updated
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TerminologyResponse {
    pub terminology_settings: Vec<TerminologySetting>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TerminologyRequest {
    pub setting_id: Uuid, 
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CurrentTerminology {
    pub setting_id: Uuid,
    pub setting_name: String,
    pub quest_term: String,
    pub guild_term: String,
    pub adventure_term: String,
    pub epic_term: String,
    pub character_term: String,
    pub minion_term: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TerminologyCreate {
    /// Name of the terminology setting
    #[schema(example = "Fantasy Terms")]
    pub name: String,
    
    /// Description of the terminology setting
    #[schema(example = "Fantasy-themed terminology for the application")]
    pub description: String,
    
    /// Term used for quests
    #[schema(example = "Quest")]
    pub quest_term: String,
    
    /// Term used for epics
    #[schema(example = "Epic")]
    pub epic_term: String,
    
    /// Term used for adventures
    #[schema(example = "Adventure")]
    pub adventure_term: String,
    
    /// Term used for characters
    #[schema(example = "Hero")]
    pub character_term: String,
    
    /// Term used for guilds
    #[schema(example = "Guild")]
    pub guild_term: String,
    
    /// Term used for skills
    #[schema(example = "Skill")]
    pub skill_term: String,
    
    /// Term used for rewards
    #[schema(example = "Reward")]
    pub reward_term: String,
    
    /// Whether this should be the current active terminology
    #[schema(default = false)]
    pub is_current: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TerminologyUpdate {
    /// Optional updated name of the terminology setting
    #[schema(example = "Modern Terms")]
    pub name: Option<String>,
    
    /// Optional updated description of the terminology setting
    #[schema(example = "Modern-themed terminology for the application")]
    pub description: Option<String>,
    
    /// Optional updated term used for quests
    #[schema(example = "Task")]
    pub quest_term: Option<String>,
    
    /// Optional updated term used for epics
    #[schema(example = "Project")]
    pub epic_term: Option<String>,
    
    /// Optional updated term used for adventures
    #[schema(example = "Campaign")]
    pub adventure_term: Option<String>,
    
    /// Optional updated term used for characters
    #[schema(example = "Agent")]
    pub character_term: Option<String>,
    
    /// Optional updated term used for guilds
    #[schema(example = "Team")]
    pub guild_term: Option<String>,
    
    /// Optional updated term used for skills
    #[schema(example = "Capability")]
    pub skill_term: Option<String>,
    
    /// Optional updated term used for rewards
    #[schema(example = "Benefit")]
    pub reward_term: Option<String>,
    
    /// Optional whether this should be the current active terminology
    pub is_current: Option<bool>,
} 