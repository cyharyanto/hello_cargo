use async_trait::async_trait;
use std::sync::Arc;
use crate::models::User;

pub type UserRepositoryArc = Arc<dyn UserRepository>;

// Re-export specific implementations
pub mod in_memory_repository;
pub mod postgres_repository;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_all(&self) -> Vec<User>;
    async fn get(&self, id: &str) -> Option<User>;
    async fn create(&self, user: User) -> Result<User, String>;
    async fn update(&self, id: &str, user: User) -> Result<(), String>;
    async fn delete(&self, id: &str) -> bool;
}
