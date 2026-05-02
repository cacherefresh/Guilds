use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;
use crate::models::PaginationMeta;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct LanguagePack {
    /// Unique identifier for the language pack
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
    /// Language code (e.g., 'eng', 'fr', 'jp')
    #[schema(example = "eng")]
    pub language_code: String,
    
    /// Name of the language in English
    #[schema(example = "English")]
    pub language_name: String,
    
    /// When the language pack was created
    pub created_at: DateTime<Utc>,
    
    /// When the language pack was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Whether this is the current active language
    pub is_current: bool,
    
    /// Real-world terminology
    pub real_world: TerminologySet,
    
    /// In-game terminology
    pub in_game: TerminologySet,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TerminologySet {
    /// Term used for quests/stories
    #[schema(example = "Story")]
    pub quest_term: String,
    
    /// Term used for quest rewards
    #[schema(example = "Points")]
    pub quest_reward_term: String,
    
    /// Term used for epics/features
    #[schema(example = "Feature")]
    pub epic_term: String,
    
    /// Term used for adventures/projects
    #[schema(example = "Project")]
    pub adventure_term: String,
    
    /// Term used for characters/users
    #[schema(example = "User")]
    pub character_term: String,
    
    /// Term used for guilds/teams
    #[schema(example = "Team")]
    pub guild_term: String,
    
    /// Term used for skills
    #[schema(example = "Skill")]
    pub skill_term: String,
    
    /// Terms for quest types
    pub quest_types: QuestTypes,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct QuestTypes {
    /// Design phase quests (like planning, architecture, wireframing)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub design: Option<String>,
    
    /// Proof of concept quests (like prototyping)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_of_concept: Option<String>,
    
    /// Implementation quests (like coding, building)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implement: Option<String>,
    
    /// Testing and bug fixing quests
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bug_check: Option<String>,
    
    /// Subdividing tasks into smaller quests
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdivide: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LanguagePackCreate {
    /// Language code (e.g., 'eng', 'fr', 'jp')
    #[schema(example = "eng")]
    pub language_code: String,
    
    /// Name of the language in English
    #[schema(example = "English")]
    pub language_name: String,
    
    /// Whether this should be the current active language
    #[schema(default = false)]
    pub is_current: Option<bool>,
    
    /// Real-world terminology
    pub real_world: TerminologySetCreate,
    
    /// In-game terminology
    pub in_game: TerminologySetCreate,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TerminologySetCreate {
    /// Term used for quests/stories
    #[schema(example = "Story")]
    pub quest_term: String,
    
    /// Term used for quest rewards
    #[schema(example = "Points")]
    pub quest_reward_term: String,
    
    /// Term used for epics/features
    #[schema(example = "Feature")]
    pub epic_term: String,
    
    /// Term used for adventures/projects
    #[schema(example = "Project")]
    pub adventure_term: String,
    
    /// Term used for characters/users
    #[schema(example = "User")]
    pub character_term: String,
    
    /// Term used for guilds/teams
    #[schema(example = "Team")]
    pub guild_term: String,
    
    /// Term used for skills
    #[schema(example = "Skill")]
    pub skill_term: String,
    
    /// Terms for quest types
    pub quest_types: QuestTypesCreate,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct QuestTypesCreate {
    /// Term for design quests
    #[schema(example = "Design")]
    pub design: String,
    
    /// Term for proof of concept quests
    #[schema(example = "Proof of Concept")]
    pub proof_of_concept: String,
    
    /// Term for implementation quests
    #[schema(example = "Implementation")]
    pub implement: String,
    
    /// Term for bug checking quests
    #[schema(example = "Bug Fix")]
    pub bug_check: String,
    
    /// Term for subdivision quests
    #[schema(example = "Subdivision")]
    pub subdivide: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LanguagePackUpdate {
    /// Optional updated language code
    #[schema(example = "eng")]
    pub language_code: Option<String>,
    
    /// Optional updated language name
    #[schema(example = "English")]
    pub language_name: Option<String>,
    
    /// Optional whether this should be the current active language
    pub is_current: Option<bool>,
    
    /// Optional updated real-world terminology
    pub real_world: Option<TerminologySetUpdate>,
    
    /// Optional updated in-game terminology
    pub in_game: Option<TerminologySetUpdate>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TerminologySetUpdate {
    /// Optional updated term used for quests/stories
    #[schema(example = "Story")]
    pub quest_term: Option<String>,
    
    /// Optional updated term used for quest rewards
    #[schema(example = "Points")]
    pub quest_reward_term: Option<String>,
    
    /// Optional updated term used for epics/features
    #[schema(example = "Feature")]
    pub epic_term: Option<String>,
    
    /// Optional updated term used for adventures/projects
    #[schema(example = "Project")]
    pub adventure_term: Option<String>,
    
    /// Optional updated term used for characters/users
    #[schema(example = "User")]
    pub character_term: Option<String>,
    
    /// Optional updated term used for guilds/teams
    #[schema(example = "Team")]
    pub guild_term: Option<String>,
    
    /// Optional updated term used for skills
    #[schema(example = "Skill")]
    pub skill_term: Option<String>,
    
    /// Optional updated terms for quest types
    pub quest_types: Option<QuestTypesUpdate>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct QuestTypesUpdate {
    /// Optional updated term for design quests
    #[schema(example = "Design")]
    pub design: Option<String>,
    
    /// Optional updated term for proof of concept quests
    #[schema(example = "Proof of Concept")]
    pub proof_of_concept: Option<String>,
    
    /// Optional updated term for implementation quests
    #[schema(example = "Implementation")]
    pub implement: Option<String>,
    
    /// Optional updated term for bug checking quests
    #[schema(example = "Bug Fix")]
    pub bug_check: Option<String>,
    
    /// Optional updated term for subdivision quests
    #[schema(example = "Subdivision")]
    pub subdivide: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LanguagePackListResponse {
    pub data: Vec<LanguagePack>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LanguagePackQuery {
    /// Optional language code to filter by
    #[schema(example = "eng")]
    pub language_code: Option<String>,
    
    /// Pagination offset
    #[schema(default = 0)]
    pub offset: Option<i64>,
    
    /// Pagination limit
    #[schema(default = 20)]
    pub limit: Option<i64>,
} 