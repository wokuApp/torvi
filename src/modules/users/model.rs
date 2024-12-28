use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: ObjectId,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl User {
    pub fn new(email: String, name: String, password: String) -> Self {
        let now = DateTime::now();
        Self {
            id: ObjectId::new(),
            email,
            name,
            password,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: ObjectId,
    pub email: String,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.unwrap(),
            email: user.email,
            name: user.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
