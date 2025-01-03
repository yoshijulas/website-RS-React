use diesel_async::{AsyncConnection, AsyncPgConnection};
use dotenvy::dotenv;
use std::env;

// pub async fn initialize_database(db: &PgConnection) {
//     // Read the SQL file
//     let sql =
//         std::fs::read_to_string("./../../database/db.sql").expect("Failed to read db.sql file");
//     // Execute the SQL commands
//     db.execute_unprepared(sql.as_str())
//         .await
//         .expect("Failed to initialize the database");
// }

pub async fn establish_connection() -> AsyncPgConnection {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        std::env::var("DATABASE_URL")?,
    );
    let pool = Pool::builder(config).build()?;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
