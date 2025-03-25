use crate::models::{
    Character, CharacterCreate, CharacterUpdate, 
    Quest, QuestCreate, QuestUpdate,
    Skill, SkillCreate
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Quest routes
        crate::handlers::quest_handler::get_quests,
        crate::handlers::quest_handler::create_quest,
        crate::handlers::quest_handler::get_quest_by_id,
        crate::handlers::quest_handler::update_quest,
        crate::handlers::quest_handler::delete_quest,
        crate::handlers::quest_handler::get_available_quests,
        
        // Skill routes
        crate::handlers::skill_handler::get_skills,
        crate::handlers::skill_handler::create_skill,
        
        // Character routes
        crate::handlers::character_handler::get_characters,
        crate::handlers::character_handler::create_character,
        crate::handlers::character_handler::get_character_by_id,
        crate::handlers::character_handler::update_character,
        crate::handlers::character_handler::get_character_skills,
        crate::handlers::character_handler::add_character_skills
    ),
    components(
        schemas(
            Quest, QuestCreate, QuestUpdate,
            Skill, SkillCreate,
            Character, CharacterCreate, CharacterUpdate
        )
    ),
    tags(
        (name = "quests", description = "Quest management endpoints"),
        (name = "skills", description = "Skill management endpoints"),
        (name = "characters", description = "Character management endpoints")
    ),
    info(
        title = "Guild Quests API",
        version = "1.0.0",
        description = "API for managing quests, characters, and skills in the guild system"
    ),
    servers(
        (url = "http://localhost:8000/api/v1", description = "Development server")
    )
)]
pub struct ApiDoc; 