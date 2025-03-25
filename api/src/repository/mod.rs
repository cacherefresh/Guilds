// Re-export the repositories
pub mod character_repository;
pub mod guild_repository;
pub mod quest_repository;
pub mod skill_repository;
pub mod town_repository;

pub use character_repository::CharacterRepository;
pub use guild_repository::GuildRepository;
pub use quest_repository::QuestRepository;
pub use skill_repository::SkillRepository;
pub use town_repository::TownRepository; 