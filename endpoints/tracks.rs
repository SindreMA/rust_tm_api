
use actix_web::{get, http::Error, HttpResponse, Responder};
use paperclip::actix::{
    // extension trait for actix_web::App and proc-macro attributes
    api_v2_operation, web::{self, Json}, Apiv2Schema, OpenApiExt
};
use paperclip::actix::{
    web::{self, Json}, // <- Include the Responder
};

struct Track {
    id: u32,
    name: String,
}


#[get("/api/v1/tracks/{id}")]
fn get_single_track(body: Json<Track>) -> Result<Json<Track>, Error> {
    Ok(body)
}

#[get("/api/v1/tracks")]
async fn get_tracks() -> impl Responder {
    HttpResponse::Ok().body("List of users")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_single_track);
    cfg.service(get_tracks);
}