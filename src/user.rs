use diesel::prelude::*;
use schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub password: &'a str,
}

pub fn insert_user(conn: &PgConnection, name: &str, password: &str) -> QueryResult<usize> {
    let new_user = NewUser { name, password };
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
}
