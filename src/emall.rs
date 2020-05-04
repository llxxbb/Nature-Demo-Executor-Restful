use std::thread;
use std::time::Duration;

use reqwest::blocking::Client;

use nature_common::{ConverterParameter, ConverterReturned, DelayedInstances};

use crate::CALLBACK_ADDRESS;

lazy_static! {
    pub static ref CLIENT : Client = Client::new();
}

pub fn send_to_warehouse_thread(para: ConverterParameter) {
    // wait 50ms
    thread::sleep(Duration::new(0, 50000));
    // send result to Nature
    let rtn = DelayedInstances {
        task_id: para.task_id,
        result: ConverterReturned::Instances(vec![para.from]),
    };
    let rtn = CLIENT.post(&*CALLBACK_ADDRESS).json(&rtn).send();
    let text: String = rtn.unwrap().text().unwrap();
    if text.contains("Err") {
        error!("{}", text);
    } else {
        debug!("warehouse business processed!")
    }
}
