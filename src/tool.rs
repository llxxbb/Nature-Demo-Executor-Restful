use nature_common::{Instance, KeyCondition, Result};

use super::{CLIENT, GET_BY_META};

pub async fn get_by_meta(cond: &KeyCondition) -> Result<Vec<Instance>> {
    // let rtn = CLIENT.post(&*GET_BY_META).json(cond).send().await?.json::<ConverterReturned>().await?;
    let res = CLIENT.post(&*GET_BY_META).json(cond).send().await?;
    let rtn = res.json::<Result<Vec<Instance>>>().await?;
    // let _ = dbg!(&rtn);
    rtn
}

#[cfg(test)]
mod test {
    use tokio::runtime::Runtime;

    use super::*;

    #[test]
    fn test() {
        let para = KeyCondition {
            id: "".to_string(),
            meta: "B:sale/order:1".to_string(),
            key_gt: "M".to_string(),
            para: "".to_string(),
            state_version: 0,
            time_ge: None,
            time_lt: None,
            limit: 100,
        };
        let mut rt = Runtime::new().unwrap();
        let result = rt.block_on(get_by_meta(&para));
        let _ = dbg!(result);
    }
}