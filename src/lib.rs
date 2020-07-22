#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use std::{env, thread};
use std::str::FromStr;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;
use actix_web::web::Json;
use reqwest::Client;

use nature_common::{ConverterParameter, ConverterReturned, Instance, Result};

use crate::emall::send_to_warehouse_thread;
use crate::sale::make_order_item;

lazy_static! {
    pub static ref CLIENT : Client = Client::new();
    pub static ref CALLBACK_ADDRESS: String = "http://localhost:8080/callback".to_string();
    pub static ref GET_BY_META: String = "http://localhost:8080/get_by_key_range".to_string();
}

async fn send_to_warehouse(para: Json<ConverterParameter>) -> HttpResponse {
    thread::spawn(move || send_to_warehouse_thread(para.0));
    // wait 60 seconds to simulate the process of warehouse business.
    HttpResponse::Ok().json(ConverterReturned::Delay(60))
}

async fn add_score(para: Json<Vec<Instance>>) -> HttpResponse {
    let mut rtn = para.0;
    rtn.iter_mut().for_each(|one| {
        if one.para.contains("subject2") {
            let points = u16::from_str(&one.content).unwrap();
            let content = (points + 4).to_string();
            one.data.content = content;
        }
    });
    HttpResponse::Ok().json(Ok(rtn) as Result<Vec<Instance>>)
}

async fn order_to_item(p: Json<ConverterParameter>) -> HttpResponse {
    HttpResponse::Ok().json(make_order_item(p.0).await)
}

pub fn start_actrix() -> Server {
    let port = env::var("DEMO_CONVERTER_PORT").unwrap_or_else(|_| "8082".to_string());
    HttpServer::new(
        || App::new()
            .route("/send_to_warehouse", web::post().to(send_to_warehouse))
            .route("/add_score", web::post().to(add_score))
            .route("/order_to_item", web::post().to(order_to_item)))
        .bind("127.0.0.1:".to_owned() + &port).unwrap()
        .run()
}

pub mod emall;
pub mod sale;
pub mod tool;