use diesel::prelude::*;

use crate::config::get_connection;
use crate::models::user;
use crate::models::user::{NewUserDto, UpdateUserDto, User};
use crate::schema::users::dsl::*;

pub fn get_users() -> QueryResult<Vec<User>> {
    users
        .select()
        .load(get_connection())
        .expect("Error loading users")
}

pub fn insert_user(user: &NewUserDto) -> User {
    diesel::insert_into(users)
        .values(&user)
        .returning(User::as_returning())
        .get_result(get_connection())
        .expect("Error creating the user")
}

pub fn update_user(user: &User) -> User {
    user.save_changes(get_connection())
        .expect("Error saving the user")
}
