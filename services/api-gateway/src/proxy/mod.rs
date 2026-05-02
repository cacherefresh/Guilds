mod quest_proxy;
mod guild_proxy;
mod character_proxy;
mod reward_proxy;

pub use quest_proxy::QuestServiceProxy;
pub use guild_proxy::GuildServiceProxy;
pub use character_proxy::CharacterServiceProxy;
pub use reward_proxy::RewardServiceProxy;

use reqwest::{Client, Error as ReqwestError};
use thiserror::Error;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Error, Debug)]
pub enum ProxyError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] ReqwestError),
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    
    #[error("Request timeout")]
    Timeout,
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Unexpected error: {0}")]
    Other(String),
}

// Common proxy trait for all service proxies
#[async_trait::async_trait]
pub trait ServiceProxy {
    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ProxyError>;
    async fn post<B: Serialize + Send, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T, ProxyError>;
    async fn put<B: Serialize + Send, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T, ProxyError>;
    async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, ProxyError>;
}

// Factory for creating service proxies
pub struct ProxyFactory {
    client: Client,
}

impl ProxyFactory {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");
            
        Self { client }
    }
    
    pub fn create_quest_proxy(&self, base_url: String) -> QuestServiceProxy {
        QuestServiceProxy::new(self.client.clone(), base_url)
    }
    
    pub fn create_guild_proxy(&self, base_url: String) -> GuildServiceProxy {
        GuildServiceProxy::new(self.client.clone(), base_url)
    }
    
    pub fn create_character_proxy(&self, base_url: String) -> CharacterServiceProxy {
        CharacterServiceProxy::new(self.client.clone(), base_url)
    }
    
    pub fn create_reward_proxy(&self, base_url: String) -> RewardServiceProxy {
        RewardServiceProxy::new(self.client.clone(), base_url)
    }
} 