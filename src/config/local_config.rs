use serde_yaml::Value;
use std::fs::File;
use std::io::Read;
use std::sync::LazyLock;

#[deny(dead_code)]
pub(super) fn init() {
    LazyLock::force(&LOCAL_CONFIG);
}
static LOCAL_CONFIG: LazyLock<Value> = LazyLock::new(|| {
    let mut file = File::open("conf.yaml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_yaml::from_str(&contents).unwrap()
});
pub struct Config {
    __private: (),
}

pub(super) fn config() -> Config {
    Config { __private: () }
}

impl Config {
    pub fn get_string(&self, str: &str) -> Option<String> {
        Some(match self.raw_data(str)? {
            Value::Bool(v) => {
                if *v {
                    String::from("true")
                } else {
                    String::from("false")
                }
            }
            Value::Number(v) => v.to_string(),
            Value::String(v) => v.to_owned(),
            _ => return None,
        })
    }

    pub fn get_i64(&self, str: &str) -> Option<i64> {
        Some(match self.raw_data(str)? {
            Value::Number(v) => v.as_i64()?,
            Value::String(v) => match v.parse::<i64>() {
                Ok(v) => v,
                Err(_) => return None,
            },
            _ => return None,
        })
    }

    fn raw_data(&self, str: &str) -> Option<&Value> {
        let mut search_path = str.split(".");
        let mut result;
        result = LOCAL_CONFIG.get(search_path.next()?)?;
        for x in search_path {
            result = result.get(x)?;
        }
        Some(result)
    }
}
