use crate::modules::users::model::{UpdateUserDto, User};
use crate::modules::users::repository::UserRepository;
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use mongodb::bson::{oid::ObjectId, DateTime};
use std::sync::Arc;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create_user(
        &self,
        email: String,
        name: String,
        password: String,
    ) -> Result<User, String>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<User>, String>;
    async fn verify_credentials(&self, email: &str, password: &str)
        -> Result<Option<User>, String>;
    async fn update_user(&self, id: &ObjectId, dto: UpdateUserDto) -> Result<User, String>;
    async fn delete_user(&self, id: &ObjectId) -> Result<(), String>;
}

pub struct UserServiceImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        self.user_repository.find_by_email(email).await
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

        if password.trim().len() < 8 {
            return Err("Password must be at least 8 characters".to_string());
        }

        if let Some(_) = self.user_repository.find_by_email(&email).await? {
            return Err("Email already exists".to_string());
        }

        let hashed_password = hash(password.as_bytes(), DEFAULT_COST)
            .map_err(|e| format!("Failed to hash password: {}", e))?;

        let mut user = User::new(email.trim().to_string(), name, hashed_password);
        user.id = Some(ObjectId::new());

        self.user_repository.create(&user).await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<User>, String> {
        self.user_repository.find_by_id(id).await
    }

    async fn verify_credentials(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<User>, String> {
        let user = match self.user_repository.find_by_email(email).await? {
            Some(user) => user,
            None => return Ok(None),
        };

        match verify(password.as_bytes(), &user.password) {
            Ok(true) => Ok(Some(user)),
            Ok(false) => Ok(None),
            Err(e) => Err(format!("Error verifying password: {}", e)),
        }
    }

    async fn update_user(&self, id: &ObjectId, dto: UpdateUserDto) -> Result<User, String> {
        let mut user = self
            .user_repository
            .find_by_id(id)
            .await?
            .ok_or("User not found")?;

        if let Some(email) = dto.email {
            if email.trim().is_empty() {
                return Err("Email cannot be empty".to_string());
            }
            if let Some(existing) = self.user_repository.find_by_email(&email).await? {
                if existing.id != user.id {
                    return Err("Email already exists".to_string());
                }
            }
            user.email = email.trim().to_string();
        }

        if let Some(name) = dto.name {
            user.name = name;
        }

        if let Some(password) = dto.password {
            if password.trim().len() < 8 {
                return Err("Password must be at least 8 characters".to_string());
            }
            user.password = hash(password.as_bytes(), DEFAULT_COST)
                .map_err(|e| format!("Failed to hash password: {}", e))?;
        }

        user.updated_at = DateTime::now();
        self.user_repository.update(&user).await?;

        Ok(user)
    }

    async fn delete_user(&self, id: &ObjectId) -> Result<(), String> {
        self.user_repository.delete(id).await
    }
}
