use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub guild_name: String,
    pub interests: Vec<String>,
    pub skills: Vec<String>,
    pub magic_abilities: Vec<String>,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub current_action: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCharacterDto {
    pub name: String,
    pub guild_name: Option<String>,
    pub interests: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
    pub magic_abilities: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCharacterDto {
    pub name: Option<String>,
    pub guild_name: Option<String>,
    pub interests: Option<Vec<String>>,
    pub skills: Option<Vec<String>>,
    pub magic_abilities: Option<Vec<String>>,
    pub current_action: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterLocationDto {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Character {
    pub fn new(
        id: Uuid,
        name: String,
        guild_name: Option<String>,
        interests: Option<Vec<String>>,
        skills: Option<Vec<String>>,
        magic_abilities: Option<Vec<String>>,
    ) -> Self {
        Self {
            id,
            name,
            guild_name: guild_name.unwrap_or_else(|| "Default Guild".to_string()),
            interests: interests.unwrap_or_else(|| vec!["Music".to_string(), "Programming".to_string(), "Bringing AI to Life".to_string()]),
            skills: skills.unwrap_or_else(Vec::new),
            magic_abilities: magic_abilities.unwrap_or_else(|| vec!["Shadow Clone".to_string(), "Shadow Minion".to_string()]),
            position_x: 0.0,
            position_y: 0.0,
            position_z: 0.0,
            current_action: "idle".to_string(),
        }
    }
} 