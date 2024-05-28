use std::env;

use aide::{
    axum::ApiRouter,
    openapi::{Contact, Info, OpenApi},
};
use dotenv::dotenv;
use tower_http::services::ServeDir;
extern crate dotenv;

use axum::Extension;

mod swagger;
mod endpoints;

#[tokio::main]
async fn main() {
    print!("Starting server..");

    print!("Loading .env file..");
    let env = dotenv().ok().expect("Failed to load .env file");

    let sql_server = env::var("SQL_SERVER").expect("SQL_SERVER not found in .env file");
    let sql_user = env::var("SQL_USER").expect("SQL_USER not found in .env file");
    let sql_password = env::var("SQL_PASSWORD").expect("SQL_PASSWORD not found in .env file");

    let port = env::var("PORT").expect("PORT not found in .env file");

    let app =  ApiRouter::new()
    .merge(swagger::route_swagger())
    .merge(endpoints::config::create_endpoints())
    .nest_service("/statics", ServeDir::new("src/statics"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_string() + &port).await.unwrap();

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