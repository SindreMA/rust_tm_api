use aide::{axum::IntoApiResponse, openapi::OpenApi};
use axum::{http::HeaderValue, Extension, Json};
use axum_swagger_ui::swagger_ui;
use hyper::{HeaderMap, StatusCode};

pub async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

pub async fn swagger_ui_page() -> impl IntoApiResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("text/html"));

    let html = swagger_ui("/api.json");

    (StatusCode::OK, headers, html)
}