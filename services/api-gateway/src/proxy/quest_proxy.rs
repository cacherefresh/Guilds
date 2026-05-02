use crate::proxy::{ProxyError, ServiceProxy};
use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use log::{error, debug};

pub struct QuestServiceProxy {
    client: Client,
    base_url: String,
}

impl QuestServiceProxy {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }
}

#[async_trait]
impl ServiceProxy for QuestServiceProxy {
    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ProxyError> {
        let url = format!("{}{}", self.base_url, path);
        debug!("GET request to Quest service: {}", url);
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send GET request to quest service: {}", e);
                ProxyError::RequestFailed(e)
            })?;
            
        match response.status() {
            StatusCode::OK => {
                response.json::<T>().await.map_err(|e| {
                    error!("Failed to deserialize response from quest service: {}", e);
                    ProxyError::RequestFailed(e)
                })
            },
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                error!("Authentication error with quest service: {}", response.status());
                Err(ProxyError::AuthError(format!("Status code: {}", response.status())))
            },
            StatusCode::SERVICE_UNAVAILABLE => {
                error!("Quest service unavailable: {}", response.status());
                Err(ProxyError::ServiceUnavailable("Quest service is unavailable".to_string()))
            },
            status => {
                error!("Unexpected response from quest service: {}", status);
                Err(ProxyError::Other(format!("Unexpected status code: {}", status)))
            }
        }
    }
    
    async fn post<B: Serialize + Send, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T, ProxyError> {
        let url = format!("{}{}", self.base_url, path);
        debug!("POST request to Quest service: {}", url);
        
        let response = self.client.post(&url)
            .json(body)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send POST request to quest service: {}", e);
                ProxyError::RequestFailed(e)
            })?;
            
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json::<T>().await.map_err(|e| {
                    error!("Failed to deserialize response from quest service: {}", e);
                    ProxyError::RequestFailed(e)
                })
            },
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                error!("Authentication error with quest service: {}", response.status());
                Err(ProxyError::AuthError(format!("Status code: {}", response.status())))
            },
            StatusCode::SERVICE_UNAVAILABLE => {
                error!("Quest service unavailable: {}", response.status());
                Err(ProxyError::ServiceUnavailable("Quest service is unavailable".to_string()))
            },
            status => {
                error!("Unexpected response from quest service: {}", status);
                Err(ProxyError::Other(format!("Unexpected status code: {}", status)))
            }
        }
    }
    
    async fn put<B: Serialize + Send, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T, ProxyError> {
        let url = format!("{}{}", self.base_url, path);
        debug!("PUT request to Quest service: {}", url);
        
        let response = self.client.put(&url)
            .json(body)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send PUT request to quest service: {}", e);
                ProxyError::RequestFailed(e)
            })?;
            
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                response.json::<T>().await.map_err(|e| {
                    error!("Failed to deserialize response from quest service: {}", e);
                    ProxyError::RequestFailed(e)
                })
            },
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                error!("Authentication error with quest service: {}", response.status());
                Err(ProxyError::AuthError(format!("Status code: {}", response.status())))
            },
            StatusCode::SERVICE_UNAVAILABLE => {
                error!("Quest service unavailable: {}", response.status());
                Err(ProxyError::ServiceUnavailable("Quest service is unavailable".to_string()))
            },
            status => {
                error!("Unexpected response from quest service: {}", status);
                Err(ProxyError::Other(format!("Unexpected status code: {}", status)))
            }
        }
    }
    
    async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, ProxyError> {
        let url = format!("{}{}", self.base_url, path);
        debug!("DELETE request to Quest service: {}", url);
        
        let response = self.client.delete(&url)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send DELETE request to quest service: {}", e);
                ProxyError::RequestFailed(e)
            })?;
            
        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => {
                if response.status() == StatusCode::NO_CONTENT {
                    // Create an empty response for NO_CONTENT
                    let empty_json = serde_json::json!({});
                    Ok(serde_json::from_value(empty_json)
                        .expect("Failed to create empty response"))
                } else {
                    response.json::<T>().await.map_err(|e| {
                        error!("Failed to deserialize response from quest service: {}", e);
                        ProxyError::RequestFailed(e)
                    })
                }
            },
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                error!("Authentication error with quest service: {}", response.status());
                Err(ProxyError::AuthError(format!("Status code: {}", response.status())))
            },
            StatusCode::SERVICE_UNAVAILABLE => {
                error!("Quest service unavailable: {}", response.status());
                Err(ProxyError::ServiceUnavailable("Quest service is unavailable".to_string()))
            },
            status => {
                error!("Unexpected response from quest service: {}", status);
                Err(ProxyError::Other(format!("Unexpected status code: {}", status)))
            }
        }
    }
} 