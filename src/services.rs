use crate::models::User;
use crate::repositories::UserRepositoryArc;

pub struct UserService {
    repository: UserRepositoryArc,
}

impl UserService {
    pub fn new(repository: UserRepositoryArc) -> Self {
        UserService { repository }
    }

    pub async fn get_all_users(&self) -> Vec<User> {
        self.repository.get_all().await
    }

    pub async fn get_user(&self, id: usize) -> Option<User> {
        self.repository.get(id).await
    }

    pub async fn create_user(&self, user: User) -> Result<(), String> {
        self.repository.create(user).await
    }

    pub async fn update_user(&self, id: usize, user: User) -> Result<(), String> {
        self.repository.update(id, user).await
    }

    pub async fn delete_user(&self, id: usize) -> bool {
        self.repository.delete(id).await
    }
}