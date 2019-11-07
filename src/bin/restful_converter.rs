extern crate dotenv;

use std::env;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::web::Json;
use dotenv::dotenv;

use nature_common::ConverterParameter;

fn index(_para: Json<ConverterParameter>) -> HttpResponse {
    HttpResponse::Ok().json("x")
}


fn main() {
    dotenv().ok();
    let port = env::var("DEMO_CONVERTER_PORT").unwrap_or_else(|_| "8082".to_string());
    HttpServer::new(
        || App::new().service(
            web::resource("/{id}/{name}/index.html").to(index)))
        .bind("127.0.0.1:".to_owned() + &port).unwrap()
        .run().unwrap();
}



