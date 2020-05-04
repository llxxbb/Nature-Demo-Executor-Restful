use nature_common::{Instance, KeyCondition, Result};

use super::{CLIENT, GET_BY_META};

pub fn get_by_meta(cond: &KeyCondition) -> Result<Vec<Instance>> {
    // let rtn = CLIENT.post(&*GET_BY_META).json(cond).send().await?.json::<ConverterReturned>().await?;
    let res = CLIENT.post(&*GET_BY_META).json(cond).send()?;
    let rtn = res.json::<Result<Vec<Instance>>>()?;
    // let _ = dbg!(&rtn);
    rtn
}

#[cfg(test)]
mod test {
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
        let result = get_by_meta(&para);
        let _ = dbg!(result);
    }
}