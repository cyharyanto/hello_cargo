use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use ulid::Ulid;
use crate::models::User;
use crate::schema::users;
use super::UserRepository;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct PostgresUserRepository {
    pool: DbPool,
}

impl PostgresUserRepository {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        PostgresUserRepository { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn get_all(&self) -> Vec<User> {
        let conn = &mut self.pool.get().expect("Couldn't get db connection from pool");
        users::table.load::<User>(conn).expect("Error loading users")
    }

    async fn get(&self, id: &str) -> Option<User> {
        let conn = &mut self.pool.get().expect("Couldn't get db connection from pool");
        users::table.find(id).first::<User>(conn).ok()
    }

    async fn create(&self, mut user: User) -> Result<User, String> {
        let conn = &mut self.pool.get().expect("Couldn't get db connection from pool");
        if user.id.is_empty() {
            user.id = Ulid::new().to_string();
        }
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .map_err(|e| e.to_string())?;
        Ok(user)
    }

    async fn update(&self, id: &str, user: User) -> Result<(), String> {
        let conn = &mut self.pool.get().expect("Couldn't get db connection from pool");
        diesel::update(users::table.find(id))
            .set((
                users::name.eq(user.name),
                users::email.eq(user.email),
            ))
            .execute(conn)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn delete(&self, id: &str) -> bool {
        let conn = &mut self.pool.get().expect("Couldn't get db connection from pool");
        diesel::delete(users::table.find(id))
            .execute(conn)
            .map(|affected| affected > 0)
            .unwrap_or(false)
    }
}