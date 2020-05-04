use nature_common::{ConverterParameter, ConverterReturned, Instance, KeyCondition};

use crate::tool::get_by_meta;

pub fn make_order_item(p: ConverterParameter) -> ConverterReturned {
    let orders = load_order();
    dbg!(orders);
    unimplemented!()
}

fn load_order() -> Vec<Instance> {
    let para = KeyCondition {
        id: "".to_string(),
        meta: "B:sale/order:1".to_string(),
        key_gt: "".to_string(),
        para: "".to_string(),
        state_version: 0,
        time_ge: None,
        time_lt: None,
        limit: 100,
    };
    get_by_meta(&para).unwrap()
}