use crate::data::sled::get_from_state;
use crate::AppState;
use actix_web::web::Json;
use actix_web::{get, web, Responder, Result};
use serde_json::json;

#[get("/status")]
async fn status() -> Result<impl Responder> {
  Ok(Json(json!({
    "status": "ok",
  })))
}

#[get("/init-conf")]
async fn init_conf(app_state: web::Data<AppState>) -> Result<impl Responder> {
  Ok(Json(json!({
    "isInProgress": get_from_state::<bool>(&app_state, "init_conf", "is_in_progress"),
    "nextStep": get_from_state::<u8>(&app_state, "init_conf", "next_step"),
  })))
}
