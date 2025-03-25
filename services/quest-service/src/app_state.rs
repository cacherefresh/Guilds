use axum::extract::FromRef;
use std::sync::Arc;
use tokio_postgres::Client;

use crate::repository::{
    QuestRepository, 
    EpicRepository,
    AdventureRepository,
    TerminologyRepository,
};

#[derive(Clone, FromRef)]
pub struct AppState {
    db_client: Arc<Client>,
}

impl AppState {
    pub fn new(db_client: Client) -> Self {
        Self {
            db_client: Arc::new(db_client),
        }
    }
    
    pub fn quest_repository(&self) -> QuestRepository {
        QuestRepository::new(self.db_client.as_ref().clone())
    }
    
    pub fn epic_repository(&self) -> EpicRepository {
        EpicRepository::new(self.db_client.as_ref().clone())
    }
    
    pub fn adventure_repository(&self) -> AdventureRepository {
        AdventureRepository::new(self.db_client.as_ref().clone())
    }
    
    pub fn terminology_repository(&self) -> TerminologyRepository {
        TerminologyRepository::new(self.db_client.as_ref().clone())
    }
} 