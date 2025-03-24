use actix_web::dev::ServiceRequest;
use actix_web::{Error, HttpMessage};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::www_authenticate::basic::Basic;
use futures::future::{ready, Ready};

// Simple basic auth middleware for development
// In a production app, you would use proper authentication with JWT or similar
pub async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // For development, accept any username with a simple password check
    // In production, you would check against a database
    let password = credentials.password();
    
    if let Some(pass) = password {
        if pass.len() >= 3 {
            // Store username in request extensions for logging
            req.extensions_mut().insert(credentials.user_id().to_string());
            return Ok(req);
        }
    }
    
    // Authentication failed
    Err((
        AuthenticationError::from(Basic::default()).into(),
        req,
    ))
} 