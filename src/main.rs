mod api;
mod data;
use actix_web::{web, App, HttpServer};
use api::handlers::status;
use data::config::{init_config, Config};
use data::sled::init_sled_db;
use data::sqlite::init_sqlite_db;
use std::sync::Mutex;

struct AppState {
  config: Mutex<Config>,
  sled_db: sled::Db,
  sqlite_db: sqlx::SqlitePool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let config = init_config();
  let app_state = web::Data::new(AppState {
    config: Mutex::new(config.clone()),
    sled_db: init_sled_db(),
    sqlite_db: init_sqlite_db().await,
  });
  HttpServer::new(move || App::new().app_data(app_state.clone()).service(status))
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
