use dotenv::dotenv;

use sqlx::migrate::MigrateDatabase;
use sqlx::migrate::Migrator;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::ConnectOptions;
use sqlx::Sqlite;
use std::env;
use std::fs;

use std::str::FromStr;

#[tokio::main]
pub async fn create_database() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("error");

    if Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        return;
    }
    fs::create_dir_all("database").unwrap();

    Sqlite::create_database(&db_url).await.unwrap();

    let mut connection = SqliteConnectOptions::from_str(&db_url)
        .unwrap()
        .connect()
        .await
        .unwrap();

    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

    MIGRATOR.run(&mut connection).await.unwrap();
}
