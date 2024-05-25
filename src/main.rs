use aide::{
    axum::{
        routing::get,
        ApiRouter,
    },
    openapi::{Contact, Info, OpenApi},
};

use axum::Extension;

mod swagger;
mod endpoints;

#[tokio::main]
async fn main() {

    let doc_url = "/api.json";

    let app =  ApiRouter::new()
    .route(doc_url, get(swagger::serve_api))
    .route("/swagger", get(swagger::swagger_ui_page))
    .merge(endpoints::config::create_endpoints());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let mut api = OpenApi {
        info: Info {
            summary: Some("A api for trackmania lookups".to_string()),
            description: Some("an example API".to_string()),
            title: "Trackmania API".to_string(),
            contact: Some(Contact { name: Some("SindreMA".to_string()),  email: Some("sindrema@gmail.com".to_string()), ..Contact::default() } ),
            version: "1.0.0".to_string(),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    axum::serve(
        listener,
        app
            // Generate the documentation.
            .finish_api(&mut api)
            // Expose the documentation to the handlers.
            .layer(Extension(api))
            .into_make_service(),
    )
    .await
    .unwrap();
}