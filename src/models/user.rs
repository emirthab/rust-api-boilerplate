use serde::{Deserialize, Serialize};

use crate::models::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = "users")]
pub struct User {
    pub id: i32,
    pub guid: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub phone: Option<String>,
    pub role: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub guid: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
    pub phone: Option<&'a str>,
    pub role: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}