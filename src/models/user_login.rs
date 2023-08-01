use serde::{Deserialize, Serialize};

use super::schema::user_logins;

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = "user_logins")]
pub struct UserLogin {
    pub id: i32,
    pub user_id: i32,
    pub device_id: String,
    pub ip_address: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(belongs_to(User))]
#[table_name = "user_logins"]
pub struct NewUserLogin<'a> {
    pub user_id: &'a i32,
    pub device_id: &'a str,
    pub ip_address: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}