use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::models::User;

pub struct UserRepository {
    users: RwLock<HashMap<usize, User>>,
}

impl UserRepository {
    pub fn new() -> Self {
        UserRepository {
            users: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_all(&self) -> Vec<User> {
        let users = self.users.read().await;
        users.values().cloned().collect()
    }

    pub async fn get(&self, id: usize) -> Option<User> {
        let users = self.users.read().await;
        users.get(&id).cloned()
    }

    pub async fn create(&self, user: User) -> Result<(), String> {
        let mut users = self.users.write().await;
        if users.contains_key(&user.id) {
            Err("User ID already exists".to_string())
        } else {
            users.insert(user.id, user);
            Ok(())
        }
    }

    pub async fn update(&self, id: usize, user: User) -> Result<(), String> {
        let mut users = self.users.write().await;
        if user.id != id && users.contains_key(&user.id) {
            return Err("New user ID already exists".to_string());
        }
        if users.contains_key(&id) {
            users.remove(&id);
            users.insert(user.id, user);
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    pub async fn delete(&self, id: usize) -> bool {
        let mut users = self.users.write().await;
        users.remove(&id).is_some()
    }
}

pub type UserRepositoryArc = Arc<UserRepository>;