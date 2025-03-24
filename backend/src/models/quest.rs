use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuestStatus {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "assigned")]
    Assigned,
    #[serde(rename = "completed")]
    Completed,
}

impl std::fmt::Display for QuestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestStatus::Available => write!(f, "available"),
            QuestStatus::Assigned => write!(f, "assigned"),
            QuestStatus::Completed => write!(f, "completed"),
        }
    }
}

impl From<String> for QuestStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "available" => QuestStatus::Available,
            "assigned" => QuestStatus::Assigned,
            "completed" => QuestStatus::Completed,
            _ => QuestStatus::Available,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Quest {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub short_description: Option<String>,
    pub category: String,
    pub requestor: String,
    pub assignee: Option<String>,
    pub reward: f64,
    pub status: String, // Stored as string in the database
}

impl Quest {
    pub fn get_status(&self) -> QuestStatus {
        QuestStatus::from(self.status.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuestDto {
    pub title: String,
    pub description: String,
    pub short_description: Option<String>,
    pub category: String,
    pub requestor: String,
    pub reward: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignQuestDto {
    pub assignee: String,
} 