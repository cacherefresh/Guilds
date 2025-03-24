use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::auth;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::basic(auth::validator);
    
    cfg.service(
        web::scope("/quests")
            .wrap(auth)
            // Services will be added in a later iteration
    );
} 