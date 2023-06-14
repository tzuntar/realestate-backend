use std::time::SystemTime;

use diesel::prelude::*;

use crate::schema::users;

#[derive(Queryable, Selectable, AsChangeset, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    name: String,
    username: String,
    email: String,
    password: String,
    registration_date: Option<SystemTime>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUserDto<'a> {
    name: &'a str,
    username: &'a str,
    email: &'a str,
    password: &'a str,
}
