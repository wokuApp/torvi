use crate::config::database::MongoDB;
use crate::modules::users::model::User;
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use mongodb::bson::doc;
use uuid::Uuid;

#[async_trait]
pub trait UserService {
    async fn create_user(
        &self,
        email: String,
        name: String,
        password: String,
    ) -> Result<User, String>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;
    async fn verify_credentials(&self, email: &str, password: &str)
        -> Result<Option<User>, String>;
}

pub struct UserServiceImpl {
    mongodb: MongoDB,
}

impl UserServiceImpl {
    pub fn new(mongodb: MongoDB) -> Self {
        Self { mongodb }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        self.mongodb
            .db
            .collection::<User>("users")
            .find_one(doc! { "email": email }, None)
            .await
            .map_err(|e| format!("Error finding user: {}", e))
    }

    async fn create_user(
        &self,
        email: String,
        name: String,
        password: String,
    ) -> Result<User, String> {
        if email.trim().is_empty() || password.trim().is_empty() {
            return Err("Email and password cannot be empty".to_string());
        }

        if let Some(_) = self.find_by_email(&email).await? {
            return Err("Email already exists".to_string());
        }

        let hashed_password = hash(password.as_bytes(), DEFAULT_COST)
            .map_err(|e| format!("Failed to hash password: {}", e))?;

        let now = Utc::now();
        let user = User {
            id: Uuid::new_v4(),
            email: email.trim().to_string(),
            name,
            password: hashed_password,
            created_at: now,
            updated_at: now,
        };

        self.mongodb
            .db
            .collection::<User>("users")
            .insert_one(&user, None)
            .await
            .map_err(|e| format!("Error creating user: {}", e))?;

        Ok(user)
    }

    async fn verify_credentials(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<User>, String> {
        let user = match self.find_by_email(email).await? {
            Some(user) => user,
            None => return Ok(None),
        };

        match verify(password.as_bytes(), &user.password) {
            Ok(true) => Ok(Some(user)),
            Ok(false) => Ok(None),
            Err(e) => Err(format!("Error verifying password: {}", e)),
        }
    }
}
