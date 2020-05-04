use dotenv::dotenv;
use nature_common::setup_logger;
use nature_demo_executor_restful::start_actrix;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _ = setup_logger();
    start_actrix().await
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
            task_id: "".to_string(),
            master: None,
            cfg: "".to_string(),
        };
        let client = Client::new();
        let rtn = client.post("http://localhost:8082/send_to_warehouse").json(&para).send().await?.json::<ConverterReturned>().await?;
        dbg!(rtn);
        Ok(())
    }
}