extern crate dotenv;
extern crate futures;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate reqwest;

use std::env;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::web::Json;
use dotenv::dotenv;
use reqwest::blocking::Client;

use nature_common::{ConverterParameter, ConverterReturned, DelayedInstances, Instance, Result, setup_logger};

lazy_static! {
    static ref CLIENT : Client = Client::new();
    static ref NATURE_CALLBACK_ADDRESS: String = env::var("NATURE_CALLBACK_ADDRESS").unwrap_or_else(|_| "http://localhost:8080/callback".to_string());
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


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _ = setup_logger();
    let port = env::var("DEMO_CONVERTER_PORT").unwrap_or_else(|_| "8082".to_string());
    HttpServer::new(
        || App::new()
            .route("/send_to_warehouse", web::post().to(send_to_warehouse))
            .route("/add_score", web::post().to(add_score)))
        .bind("127.0.0.1:".to_owned() + &port).unwrap()
        .run().await
}

#[cfg(test)]
mod reqwest_test {
    use reqwest::{Client, Error};
    use tokio::runtime::Runtime;

    use nature_common::{ConverterParameter, ConverterReturned};

    #[test]
    fn reqwest_test() {
        let _rtn = Runtime::new().unwrap().block_on(http_call());
    }

    async fn http_call() -> Result<(), Error> {
        let para = ConverterParameter {
            from: Default::default(),
            last_state: None,
            task_id: vec![],
            master: None,
            cfg: "".to_string(),
        };
        let client = Client::new();
        let rtn = client.post("http://localhost:8082/send_to_warehouse").json(&para).send().await?.json::<ConverterReturned>().await?;
        dbg!(rtn);
        Ok(())
    }
}