use actix_web::{get, web, HttpResponse, Responder};

#[get("/api/v1/leaderboard/track/{id}")]
async fn get_track_leaderboard(id: web::Path<u32>, start: web::Query<u32>, end: web::Query<u32>, country: web::Query<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Single leaderboard: {}", id))
}

#[get("/api/v1/leaderboard/player/{id}")]
async fn get_player_leaderboard(id: web::Path<u32>, start: web::Query<u32>, end: web::Query<u32>, country: web::Query<String>) -> impl Responder {
    HttpResponse::Ok().body("List of users")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_track_leaderboard);
    cfg.service(get_player_leaderboard);
}