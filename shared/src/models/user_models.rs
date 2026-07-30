use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Type, Debug)]
pub struct UserSearchModel {
    pub id: Uuid,
    pub email: String,
    pub username: String,
}

#[derive(Deserialize, Serialize, Type, Debug)]
pub struct UserSearchRequestModel {
    pub query: String,
}
