use crate::{
    models::{user::{NewUser, User}, user_login::{NewUserLogin, UserLogin}},
    DbError,
};
use diesel::prelude::*;

pub fn add_user(_username : &String, _email: &String, _password: &String, conn: &PgConnection) -> Result<User, DbError> {
    use crate::models::schema::users::dsl::*;

    let new_user = NewUser {
        guid: &uuid::Uuid::new_v4().to_string(),
        username: _username,
        email: _email,
        password_hash: &format!("{:x}", md5::compute(_password)),
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
        phone: None,
        role: &"user".to_owned(),
    };

    let res = diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)?;
    Ok(res)
}

pub fn get_user_by_username(_username : &String, conn: &PgConnection) -> Result<User, DbError> {
    use crate::models::schema::users::dsl::*;
     let user = users
        .filter(username.eq(_username))
        .get_result::<User>(conn)?;
    Ok(user)
}

pub fn add_user_login(_user_id: &i32, _device_id: &String, _ip_address: &String, conn: &PgConnection) -> Result<UserLogin, DbError> {
    use crate::models::schema::user_logins::dsl::*;
    let new_user_login = NewUserLogin {
        user_id: _user_id,
        device_id: _device_id,
        ip_address: _ip_address,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local()
    };

    let res = diesel::insert_into(user_logins)
        .values(&new_user_login)
        .get_result(conn)?;
    
    Ok(res)
}