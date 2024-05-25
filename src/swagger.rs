use aide::{axum::{routing::get, ApiRouter, IntoApiResponse}, openapi::OpenApi};
use axum::{http::HeaderValue, Extension, Json};
use axum_swagger_ui::swagger_ui;
use hyper::{HeaderMap, StatusCode};

pub fn route_swagger() -> ApiRouter {
    let mut api = ApiRouter::new();

    api = api.api_route("/api.json", get(serve_api));
    api = api.api_route("/swagger", get(swagger_ui_page));

    api
}

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

async fn swagger_ui_page() -> impl IntoApiResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("text/html"));

    let mut html = swagger_ui("/api.json");

    let css_dark_mode_link = "/statics/swagger_darkmode.css";

    html = html.replace(
        "</head>",
        &format!("<link rel=\"stylesheet\" type=\"text/css\" href=\"{}\"></head>", css_dark_mode_link)
    );


    (StatusCode::OK, headers, html)
}