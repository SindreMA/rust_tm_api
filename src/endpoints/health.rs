use aide::axum::{routing::get, ApiRouter, IntoApiResponse};

async fn health_check() -> impl IntoApiResponse {
    "OK"
}

pub fn create_endpoints() -> ApiRouter {
    let api = ApiRouter::new()
    .api_route("/health", get(health_check));

    api
}