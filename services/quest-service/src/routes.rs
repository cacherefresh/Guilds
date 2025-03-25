use axum::{
    routing::{get, post, patch, delete},
    Router,
};

use crate::{
    handlers::{
        get_quests, get_quest_by_id, create_quest, update_quest, delete_quest,
        get_epics, get_epic_by_id, get_epic_details, create_epic, update_epic, delete_epic,
        get_adventures, get_adventure_by_id, get_adventure_details, create_adventure, update_adventure, delete_adventure,
        get_all_terminology_settings, get_terminology_setting_by_id, get_current_terminology, 
        create_terminology_setting, update_terminology_setting, set_current_terminology, delete_terminology_setting,
        health_check,
    },
    AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check route
        .route("/api/health", get(health_check))
        
        // Quest routes
        .route("/api/quests", get(get_quests).post(create_quest))
        .route("/api/quests/:id", get(get_quest_by_id).patch(update_quest).delete(delete_quest))
        
        // Epic routes
        .route("/api/epics", get(get_epics).post(create_epic))
        .route("/api/epics/:id", get(get_epic_by_id).patch(update_epic).delete(delete_epic))
        .route("/api/epics/:id/details", get(get_epic_details))
        
        // Adventure routes
        .route("/api/adventures", get(get_adventures).post(create_adventure))
        .route("/api/adventures/:id", get(get_adventure_by_id).patch(update_adventure).delete(delete_adventure))
        .route("/api/adventures/:id/details", get(get_adventure_details))
        
        // Terminology routes
        .route("/api/terminology", get(get_all_terminology_settings).post(create_terminology_setting))
        .route("/api/terminology/current", get(get_current_terminology))
        .route("/api/terminology/:id", get(get_terminology_setting_by_id).patch(update_terminology_setting).delete(delete_terminology_setting))
        .route("/api/terminology/:id/set-current", post(set_current_terminology))
        
        .with_state(state)
} 