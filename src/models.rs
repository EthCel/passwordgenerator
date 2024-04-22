use diesel::prelude::*;
//use schema::users;
use diesel::prelude::*;

use crate::users;

#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
}
/*
pub fn insert_user(conn: &PgConnection, name: &str, password: &str) -> QueryResult<usize> {
    let new_user = User { name, password };
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
}
*/
//comment out the function try it for a minute
//
//then uncomment try it
