use aide::{
    axum::ApiRouter,
    openapi::{Contact, Info, OpenApi},
};
use tower_http::services::ServeDir;


use axum::Extension;

mod swagger;
mod endpoints;

#[tokio::main]
async fn main() {
    let app =  ApiRouter::new()
    .merge(swagger::route_swagger())
    .merge(endpoints::config::create_endpoints())
    .nest_service("/statics", ServeDir::new("src/statics"));

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
            .finish_api(&mut api)
            .layer(Extension(api))
            .into_make_service(),
    )
    .await
    .unwrap();
}