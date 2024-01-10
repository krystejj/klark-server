use actix_web::web::Json;
use actix_web::{get, Responder, Result};
use serde_json::json;

#[get("/status")]
async fn status() -> Result<impl Responder> {
  Ok(Json(json!({
    "status": "ok",
  })))
}
