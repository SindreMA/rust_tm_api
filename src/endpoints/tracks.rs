use aide::axum::{routing::get, ApiRouter, IntoApiResponse};


fn get_single_track(id: u32) -> impl IntoApiResponse {
    "Single track"
}

async fn get_tracks() -> impl IntoApiResponse {
    "List of tracks"
}

pub fn create_endpoints() -> ApiRouter {
    let base_path = "/api/v1/tracks";

    let api = ApiRouter::new()
    //.api_route(&format!("{}/{{id}}", base_path), get(get_single_track));
    .api_route(base_path, get(get_tracks));

    api
}