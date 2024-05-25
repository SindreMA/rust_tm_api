use actix_web::{get, web, HttpResponse, Responder};

#[get("/api/v1/players/{id}")]
async fn get_single_player(id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Single player: {}", id))
}

#[get("/api/v1/players")]
async fn get_players() -> impl Responder {
    HttpResponse::Ok().body("List of users")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_single_player);
    cfg.service(get_players);
}