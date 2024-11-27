use actix_web::{get, HttpResponse, Responder};
use tracing::{info, instrument};

#[get("/health")]
#[instrument(name = "Health Check", level = "info", target = "racing_tournament")]
pub async fn health_check() -> impl Responder {
    info!("Health check");
    HttpResponse::Ok().finish()
}
