pub mod adventure;
pub mod epic;
pub mod language_pack;
pub mod quest;
pub mod terminology;

pub fn config(cfg: &mut web::ServiceConfig) {
    adventure::config(cfg);
    epic::config(cfg);
    language_pack::config(cfg);
    quest::config(cfg);
    terminology::config(cfg);
} 