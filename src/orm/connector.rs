use sea_orm::{Database, DatabaseConnection};
use std::env;

/// Attempts to establish a database connection using the `DATABASE_URL` environment variable.
///
/// This asynchronous function retrieves the database URL from the `DATABASE_URL`
/// environment variable, then attempts to connect to the database using the provided URL.
/// If the connection is successful, it returns a `DatabaseConnection` instance wrapped
/// in a `Result`. In case of failure, it returns an error.
pub async fn try_connect_from_env() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL")?;

    let conn = Database::connect(db_url).await?;
    Ok(conn)
}
