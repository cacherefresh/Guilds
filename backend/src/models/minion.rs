use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MinionState {
    #[serde(rename = "loaded")]
    Loaded,
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "doing")]
    Doing,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "canceled")]
    Canceled,
}

impl std::fmt::Display for MinionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MinionState::Loaded => write!(f, "loaded"),
            MinionState::Start => write!(f, "start"),
            MinionState::Doing => write!(f, "doing"),
            MinionState::Complete => write!(f, "complete"),
            MinionState::Canceled => write!(f, "canceled"),
        }
    }
}

impl From<String> for MinionState {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "loaded" => MinionState::Loaded,
            "start" => MinionState::Start,
            "doing" => MinionState::Doing,
            "complete" => MinionState::Complete,
            "canceled" => MinionState::Canceled,
            _ => MinionState::Loaded,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Minion {
    pub id: Uuid,
    pub task_description: String,
    pub category: String,
    pub requestor: String,
    pub assignee: String,
    pub state: String, // Stored as string in the database
    pub reward: f64,
}

impl Minion {
    pub fn get_state(&self) -> MinionState {
        MinionState::from(self.state.clone())
    }
    
    pub fn is_done(&self) -> bool {
        let state = self.get_state();
        state == MinionState::Complete || state == MinionState::Canceled
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMinionDto {
    pub task_description: String,
    pub category: String,
    pub requestor: Option<String>,
    pub assignee: Option<String>,
    pub reward: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMinionStateDto {
    pub state: MinionState,
} 