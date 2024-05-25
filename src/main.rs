#[macro_use] extern crate rocket;


use rocket_okapi::openapi;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use schemars::JsonSchema;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}