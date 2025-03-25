mod quest_handler;
mod guild_handler;
mod character_handler;
mod reward_handler;
mod auth_handler;
mod health_handler;

use actix_web::web;

// Configure all routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Health check
    cfg.service(health_handler::health_check);
    
    // Authentication
    cfg.service(
        web::scope("/auth")
            .service(auth_handler::login)
            .service(auth_handler::register)
    );
    
    // API routes
    cfg.service(
        web::scope("/quests")
            .service(quest_handler::get_quests)
            .service(quest_handler::get_quest_by_id)
            .service(quest_handler::create_quest)
            .service(quest_handler::update_quest)
            .service(quest_handler::delete_quest)
    );
    
    cfg.service(
        web::scope("/guilds")
            .service(guild_handler::get_guilds)
            .service(guild_handler::get_guild_by_id)
            .service(guild_handler::create_guild)
            .service(guild_handler::update_guild)
            .service(guild_handler::delete_guild)
            .service(guild_handler::get_guild_members)
    );
    
    cfg.service(
        web::scope("/characters")
            .service(character_handler::get_characters)
            .service(character_handler::get_character_by_id)
            .service(character_handler::create_character)
            .service(character_handler::update_character)
            .service(character_handler::delete_character)
            .service(character_handler::get_character_skills)
    );
    
    cfg.service(
        web::scope("/rewards")
            .service(reward_handler::get_rewards)
            .service(reward_handler::claim_reward)
    );
} 