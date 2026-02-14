use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
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
            id: None,
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
    pub updated_at: DateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.expect("User must have an id to convert to response"),
            email: user.email,
            name: user.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_new_has_no_id() {
        let user = User::new(
            "test@example.com".to_string(),
            "Test User".to_string(),
            "hashed_password".to_string(),
        );
        assert!(user.id.is_none());
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test User");
    }

    #[test]
    fn test_user_response_from_user_with_id() {
        let mut user = User::new(
            "test@example.com".to_string(),
            "Test User".to_string(),
            "hashed_password".to_string(),
        );
        user.id = Some(ObjectId::new());

        let response = UserResponse::from(user.clone());
        assert_eq!(response.id, user.id.unwrap());
        assert_eq!(response.email, user.email);
        assert_eq!(response.name, user.name);
    }

    #[test]
    #[should_panic(expected = "User must have an id")]
    fn test_user_response_from_user_without_id_panics() {
        let user = User::new(
            "test@example.com".to_string(),
            "Test User".to_string(),
            "hashed_password".to_string(),
        );
        let _ = UserResponse::from(user);
    }

    #[test]
    fn test_user_clone() {
        let user = User::new(
            "test@example.com".to_string(),
            "Test User".to_string(),
            "password".to_string(),
        );
        let cloned = user.clone();
        assert_eq!(cloned.email, user.email);
        assert_eq!(cloned.name, user.name);
    }
}
