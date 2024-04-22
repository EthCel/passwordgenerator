use diesel::prelude::*;
use schema::users;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
}

/*
pub fn insert_user(conn: &PgConnection, name: &str, password: &str) -> QueryResult<usize> {
    let new_user = NewUser { name, password };
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
}
*/
