use diesel::prelude::*;

use crate::config::get_connection;
use crate::models::user::{NewUserDto, User};
use crate::schema::users::dsl::*;

pub fn get_users() -> QueryResult<Vec<User>, > {
    let mut con = get_connection();
    con.transaction(|conn| users.load::<User>(conn))
}

pub fn insert_user(user: &NewUserDto) -> QueryResult<User> {
    let mut con = get_connection();
    con.transaction(|conn| {
        diesel::insert_into(users)
            .values(&*user)
            .returning(User::as_returning())
            .get_result::<User>(conn)
    })
}

pub fn update_user(user: &User) -> QueryResult<User> {
    let mut con = get_connection();
    con.transaction(|conn| user.save_changes(conn))
}

pub fn delete_user(user: &User) -> QueryResult<User> {
    let mut con = get_connection();
    con.transaction(|conn| {
        diesel::delete(user)
            .get_result::<User>(conn)
    })
}
