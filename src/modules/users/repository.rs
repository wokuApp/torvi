use crate::modules::users::model::User;
use async_trait::async_trait;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Database;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), String>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;
    async fn find_by_id(&self, id: &ObjectId) -> Result<Option<User>, String>;
    async fn update(&self, user: &User) -> Result<(), String>;
    async fn delete(&self, id: &ObjectId) -> Result<(), String>;
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

    async fn update(&self, user: &User) -> Result<(), String> {
        let id = user
            .id
            .as_ref()
            .ok_or("User must have an id to update")?;

        self.db
            .collection::<User>("users")
            .replace_one(doc! { "_id": id }, user)
            .await
            .map_err(|e| format!("Error updating user: {}", e))?;

        Ok(())
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), String> {
        let result = self
            .db
            .collection::<User>("users")
            .delete_one(doc! { "_id": id })
            .await
            .map_err(|e| format!("Error deleting user: {}", e))?;

        if result.deleted_count == 0 {
            return Err("User not found".to_string());
        }
        Ok(())
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
