use std::collections::HashMap;
use tokio::sync::RwLock;
use async_trait::async_trait;
use crate::models::User;
use super::UserRepository;
use ulid::Ulid;

pub struct InMemoryUserRepository {
    users: RwLock<HashMap<String, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository {
            users: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn get_all(&self) -> Vec<User> {
        let users = self.users.read().await;
        users.values().cloned().collect()
    }

    async fn get(&self, id: &str) -> Option<User> {
        let users = self.users.read().await;
        users.get(id).cloned()
    }

    async fn create(&self, mut user: User) -> Result<User, String> {
        let mut users = self.users.write().await;
        if user.id.is_empty() {
            user.id = Ulid::new().to_string();
        }
        if users.contains_key(&user.id) {
            Err("User ID already exists".to_string())
        } else {
            let created_user = user.clone();
            users.insert(user.id.clone(), user);
            Ok(created_user)
        }
    }

    async fn update(&self, id: &str, user: User) -> Result<(), String> {
        let mut users = self.users.write().await;
        if user.id != id && users.contains_key(&user.id) {
            return Err("New user ID already exists".to_string());
        }
        if users.contains_key(id) {
            users.remove(id);
            users.insert(user.id.clone(), user);
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    async fn delete(&self, id: &str) -> bool {
        let mut users = self.users.write().await;
        users.remove(id).is_some()
    }
}