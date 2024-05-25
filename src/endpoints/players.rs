use aide::axum::{routing::get, ApiRouter, IntoApiResponse};
use axum::extract::Path;

async fn get_single_player(Path(id): Path<u32>) -> impl IntoApiResponse {
    "Single user"
}

async fn get_players() -> impl IntoApiResponse {
    "List of users"
}

pub fn create_endpoints() -> ApiRouter {
    let base_path = "/api/v1/players";

    let mut api = ApiRouter::new();


    api = api.api_route(&format!("{}/{{id}}", base_path), get(get_single_player));
    api = api.api_route(base_path, get(get_players));

    api
}