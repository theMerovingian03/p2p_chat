use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: i64,    // Expiration timestamp
    pub iat: i64,    // Issued-at timestamp
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsClaims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub token_type: String,
}
