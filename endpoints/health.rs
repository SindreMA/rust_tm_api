use actix_web::{get, web, HttpResponse, Responder};

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}