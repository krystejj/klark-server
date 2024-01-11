pub mod structs;
use sqlx::SqlitePool;
use std::fs::File;
use std::path::Path;

const SQLITE_DB_FILE_PATH: &str = "data.sqlite";

pub async fn init_sqlite_db() -> SqlitePool {
  let does_db_exist = Path::new(SQLITE_DB_FILE_PATH).exists();
  // Create database file if it doesn't exist
  if !does_db_exist {
    File::create(SQLITE_DB_FILE_PATH).expect("couldn't create sqlite database file");
  }

  // Create connection pool
  let db = SqlitePool::connect(SQLITE_DB_FILE_PATH)
    .await
    .expect("couldn't create sqlite database connection pool");

  // Create default schema for sled database if it was just created
  if !does_db_exist {
    sqlx::query(
      "
CREATE TABLE user (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  username TEXT NOT NULL UNIQUE,
  name TEXT,
  password TEXT,
  is_admin INTEGER NOT NULL DEFAULT 0
)
      ",
    )
    .execute(&db)
    .await
    .expect("couldn't create default schema in the sqlite database");
  }
  db
}
