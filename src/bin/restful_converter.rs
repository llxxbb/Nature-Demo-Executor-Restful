extern crate dotenv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use std::env;
use std::thread;
use std::time::Duration;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::web::Json;
use dotenv::dotenv;
use reqwest::Client;

use nature_common::{ConverterParameter, ConverterReturned, DelayedInstances, setup_logger};

lazy_static! {
    static ref CLIENT : Client = Client::new();
    static ref NATURE_CALLBACK_ADDRESS: String = env::var("NATURE_CALLBACK_ADDRESS").unwrap_or_else(|_| "http://localhost:8080/callback".to_string());
}

fn send_to_warehouse(para: Json<ConverterParameter>) -> HttpResponse {
    thread::spawn(move || send_to_warehouse_thread(para.0));
    // wait 60 seconds to simulate the process of warehouse business.
    HttpResponse::Ok().json(ConverterReturned::Delay(60))
}

fn send_to_warehouse_thread(para: ConverterParameter) {
    // wait 50ms
    thread::sleep(Duration::new(0, 50000));
    // send result to Nature
    let rtn = DelayedInstances {
        task_id: para.task_id,
        result: ConverterReturned::Instances(vec![para.from]),
    };
    let rtn = CLIENT.post(&*NATURE_CALLBACK_ADDRESS).json(&rtn).send();
    let text: String = rtn.unwrap().text().unwrap();
    if text.contains("Err") {
        error!("{}", text);
    } else {
        debug!("warehouse business processed!")
    }
}


fn main() {
    dotenv().ok();
    let _ = setup_logger();
    let port = env::var("DEMO_CONVERTER_PORT").unwrap_or_else(|_| "8082".to_string());
    HttpServer::new(
        || App::new()
            .route("/send_to_warehouse", web::post().to(send_to_warehouse)))
        .bind("127.0.0.1:".to_owned() + &port).unwrap()
        .run().unwrap();
}



