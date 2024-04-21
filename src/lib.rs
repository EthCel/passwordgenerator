use diesel::PgConnection;
use dotenv::var;

pub fn establish_connection() -> PgConnection {
    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
