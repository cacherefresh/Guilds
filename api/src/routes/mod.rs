pub mod quest_routes;
pub mod skill_routes;
pub mod character_routes;

pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/api")
            .configure(quest_routes::configure_routes)
            .configure(skill_routes::configure_routes)
            .configure(character_routes::configure_routes)
    );
} 