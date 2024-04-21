use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://rohanshakur:password@localhost:5432/passwordgen";
    PgConnection::establish(&database_url)
        .expect("Error connecting to the database")
}
