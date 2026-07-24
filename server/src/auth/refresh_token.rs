use crate::{errors::AppError, models::refresh_token_model::RefreshToken};
use hex;
use rand::random;
use sha2::{Digest, Sha256};

pub fn get_refresh_token() -> RefreshToken {
    // Generates random token
    let token = random::<[u8; 32]>();
    // Hash token using SHA-256
    let hash = Sha256::digest(token);
    // Encode token to send to client
    let refresh_token_string = hex::encode(token);
    // Encode hash to store in DB
    let refresh_token_hash = hex::encode(hash);
    // (send_to_client, store_in_db)
    RefreshToken {
        refresh_token_client: refresh_token_string,
        refresh_token_db: refresh_token_hash,
    }
}

pub fn decode_refresh_token(refresh_token: &str) -> Result<String, AppError> {
    // decodes client's refresh token
    let refresh_token_bytes = hex::decode(refresh_token).map_err(|_| AppError::Unauthorized)?;
    // hash token to search in DB
    let hash = Sha256::digest(&refresh_token_bytes);
    // Return token to search in DB
    Ok(hex::encode(hash))
}
