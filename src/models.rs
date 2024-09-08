use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use diesel::prelude::*;
use ulid::Ulid;

#[derive(Serialize, Deserialize, Clone, ToSchema, Queryable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    #[schema(example = "01F8Z1YWXC8P4GJ9HZ3S3Q9X4Y")]
    pub id: String,
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john.doe@example.com")]
    pub email: String,
}

impl User {
    pub fn new(id: Option<String>, name: String, email: String) -> Self {
        User {
            id: id.unwrap_or_else(|| Ulid::new().to_string()),
            name,
            email,
        }
    }
}