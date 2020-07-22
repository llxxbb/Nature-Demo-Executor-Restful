use std::collections::HashMap;

use nature_common::{ConverterParameter, ConverterReturned, Instance, KeyCondition, SEPARATOR_INS_PARA};
use nature_demo_common::Order;

use crate::tool::get_by_meta;

pub async fn make_order_item(p: ConverterParameter) -> ConverterReturned {
    let para = p.from.para.to_string();
    let orders = load_order(p).await;
    // dbg!(&orders);

    let mut volume: HashMap<u32, u32> = HashMap::new();
    let mut money: HashMap<u32, u32> = HashMap::new();
    for o in orders {
        let o = serde_json::from_str::<Order>(&o.content).unwrap();
        o.items.iter().for_each(|one| {
            let val = volume.get(&one.item.id);
            let v = match val {
                Some(v) => *v,
                None => 0
            };
            volume.insert(one.item.id, v + one.num);
            let val = money.get(&one.item.id);
            let v = match val {
                Some(v) => *v,
                None => 0
            };
            money.insert(one.item.id, v + one.num * (one.item.price as u32));
        })
    }

    let mut rtn: Vec<Instance> = vec![];
    volume.iter().for_each(|o| {
        let mut instance = Instance::default();
        instance.meta = "B:sale/item/volume/s:1".to_string();
        instance.para = format!("{}/{}", o.0, para);
        instance.content = o.1.to_string();
        rtn.push(instance)
    });
    money.iter().for_each(|o| {
        let mut instance = Instance::default();
        instance.meta = "B:sale/item/money/s:1".to_string();
        instance.para = format!("{}/{}", o.0, para);
        instance.content = o.1.to_string();
        rtn.push(instance)
    });
    ConverterReturned::Instances(rtn)
}

async fn load_order(p: ConverterParameter) -> Vec<Instance> {
    let part: Vec<&str> = p.from.para.split(&*SEPARATOR_INS_PARA).collect();
    let para = KeyCondition {
        id: "".to_string(),
        meta: p.from.from.as_ref().unwrap().meta.to_string(),
        key_gt: "".to_string(),
        key_ge: "".to_string(),
        key_lt: "".to_string(),
        key_le: "".to_string(),
        para: "".to_string(),
        state_version: 0,
        time_ge: Some(part[0].parse::<i64>().unwrap()),
        time_lt: Some(part[1].parse::<i64>().unwrap()),
        limit: 100,
    };
    get_by_meta(&para).await.unwrap()
}