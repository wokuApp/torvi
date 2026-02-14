use crate::modules::users::model::User;
use async_trait::async_trait;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Database;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), String>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<User>, String>;
}

pub struct UserRepositoryImpl {
    db: Database,
}

impl UserRepositoryImpl {
    pub fn new(db: &Database) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: &User) -> Result<(), String> {
        self.db
            .collection::<User>("users")
            .insert_one(user)
            .await
            .map_err(|e| format!("Error creating user: {}", e))?;
        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        self.db
            .collection::<User>("users")
            .find_one(doc! { "email": email })
            .await
            .map_err(|e| format!("Error finding user: {}", e))
    }

    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<User>, String> {
        self.db
            .collection::<User>("users")
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| format!("Error finding user: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_repository_impl_creation() {
        let _repo_fn = UserRepositoryImpl::new;
    }
}
