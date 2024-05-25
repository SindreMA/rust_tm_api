use aide::axum::{routing::get, ApiRouter, IntoApiResponse};
use axum::extract::{Path, Query};

async fn get_track_leaderboard(Path(id): Path<u32>, Query(start): Query<u32>, Query(end): Query<u32>, Query(country): Query<String>) -> impl IntoApiResponse {
    "List of track leaderboard"
}

async fn get_player_leaderboard(Path(id): Path<u32>, Query(start): Query<u32>, Query(end): Query<u32>, Query(country): Query<String>) -> impl IntoApiResponse {
    "List of user leaderboard"
}

pub fn create_endpoints() -> ApiRouter {
    let base_path = "/api/v1/leaderboard";

    let api = ApiRouter::new()

    .api_route(&format!("{}/track/{{id}}", base_path), get(get_track_leaderboard))
    .api_route(&format!("{}/player/{{id}}", base_path), get(get_player_leaderboard));

    api
}