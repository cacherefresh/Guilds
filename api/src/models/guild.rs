use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

use super::character::Character;
use super::town::Town;

// Full Guild model with all details
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Guild {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(example = "Crimson Blades")]
    pub name: String,
    #[schema(example = "A guild specializing in combat and tactical strategies")]
    pub description: Option<String>,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub guild_master_id: Option<Uuid>,
    pub guild_master: Option<Character>,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174002")]
    pub town_id: Option<Uuid>,
    pub town: Option<Town>,
    #[schema(example = "80")]
    pub reward_divider_percentage: i32,
    #[schema(example = "5000")]
    pub stash_reward: i32,
    #[schema(example = "42")]
    pub quests_completed: i32,
    pub members: Option<Vec<Character>>,
    pub ai_teammate: Option<Character>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Simplified guild data for listing purposes
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GuildSummary {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    #[schema(example = "Crimson Blades")]
    pub name: String,
    #[schema(example = "A guild specializing in combat and tactical strategies")]
    pub description: Option<String>,
    #[schema(example = "John Doe")]
    pub guild_master_name: Option<String>,
    #[schema(example = "Oakvale")]
    pub town_name: Option<String>,
    #[schema(example = "15")]
    pub member_count: i64,
    #[schema(example = "42")]
    pub quests_completed: i32,
}

// For creating a new guild
#[derive(Debug, Deserialize, ToSchema, Clone, Debug)]
pub struct GuildCreate {
    #[schema(example = "Crimson Blades")]
    pub name: String,
    #[schema(example = "A guild specializing in combat and tactical strategies")]
    pub description: Option<String>,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub guild_master_id: Uuid,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174002")]
    pub town_id: Option<Uuid>,
    #[schema(example = "80")]
    pub reward_divider_percentage: Option<i32>,
}

// For updating an existing guild
#[derive(Debug, Deserialize, ToSchema, Clone, Debug)]
pub struct GuildUpdate {
    #[schema(example = "Crimson Blades Elite")]
    pub name: Option<String>,
    #[schema(example = "An elite guild specializing in combat and tactical strategies")]
    pub description: Option<String>,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub guild_master_id: Option<Uuid>,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174002")]
    pub town_id: Option<Uuid>,
    #[schema(example = "85")]
    pub reward_divider_percentage: Option<i32>,
}

// For guild membership
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GuildMembership {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub character_id: Uuid,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub guild_id: Uuid,
    #[schema(example = "true")]
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
}

// Database record for guild membership
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GuildMemberRecord {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub character_id: Uuid,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub guild_id: Uuid,
    #[schema(example = true)]
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
}

// For skill match searches
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GuildSkillMatch {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub guild_id: Uuid,
    #[schema(example = "Crimson Blades")]
    pub guild_name: String,
    #[schema(example = "A guild specializing in combat and tactical strategies")]
    pub description: Option<String>,
    #[schema(example = "5")]
    pub matching_skills_count: i64,
    #[schema(example = "15")]
    pub total_skills: i64,
    #[schema(example = "42")]
    pub quests_completed: i32,
}

// Guild response with pagination
#[derive(Debug, Serialize, ToSchema, Clone, Debug)]
pub struct GuildListResponse {
    pub guilds: Vec<GuildSummary>,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
}

// Town reference for guild
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct TownReference {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "Waterdeep")]
    pub name: String,
    #[schema(example = "Sword Coast")]
    pub region: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GuildSearchResponse {
    pub guilds: Vec<GuildSkillMatch>,
    pub skills_used: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GuildJoinRequest {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub character_id: Uuid,
    #[schema(example = "true")]
    pub set_active: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GuildLeaveRequest {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub character_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GuildSetActiveRequest {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub character_id: Uuid,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub guild_id: Uuid,
} 