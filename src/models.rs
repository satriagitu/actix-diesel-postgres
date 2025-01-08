
// models.rs
use serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable};
use uuid::Uuid;
use crate::schema::users;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
