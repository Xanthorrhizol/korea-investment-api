use crate::Error;

pub(crate) fn get_json_inner(val: &json::JsonValue, key: &str) -> Result<json::JsonValue, Error> {
    let keys = key.split('.').collect::<Vec<&str>>();
    let mut val = val;
    for key in keys {
        match val {
            json::JsonValue::Object(o) => {
                match o.get(key) {
                    Some(result) => {
                        val = result;
                    }
                    None => {
                        return Err(Error::InvalidData);
                    }
                };
            }
            _ => {
                return Err(Error::InvalidData);
            }
        }
    }
    Ok(val.to_owned())
}
