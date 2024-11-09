use handlebars::Handlebars;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::sync::LazyLock;
use toml::Value;
use tracing::error;

#[deny(dead_code)]
pub(super) fn init() {
    LazyLock::force(&INNER_TEMPLATE);
}
static INNER_TEMPLATE: LazyLock<Handlebars> = LazyLock::new(|| {
    let mut hb = Handlebars::new();
    // init email template
    let template_map = read_email_template().unwrap();
    for (key, value) in template_map.iter() {
        hb.register_template_string(key, value).unwrap();
    }
    hb
});
pub struct Template {
    __private: (),
}

pub fn template() -> Template {
    Template { __private: () }
}

fn read_email_template() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut template_map = HashMap::new();
    let toml_str = fs::read_to_string("src/util/template/email.toml")?;
    let parsed_toml: HashMap<String, Value> =
        toml::from_str(&toml_str).expect("Failed to parse TOML.");
    for (key, value) in parsed_toml.iter() {
        match value {
            Value::Table(tab) => {
                for (field, value) in tab {
                    match value {
                        Value::String(s) => {
                            template_map.insert(format!("{}_{}", key, field), s.to_owned());
                        }
                        _ => {
                            error!("unsupported template: key={}, value={}", key, value);
                        }
                    }
                }
            }
            _ => {
                error!("unsupported template: key={}, value={}", key, value);
            }
        };
    }
    Ok(template_map)
}

impl Template {
    pub fn render_template<T>(&self, template: &str, data: &T) -> Result<String, Box<dyn Error>>
    where
        T: Serialize,
    {
        match INNER_TEMPLATE.render(template, data) {
            Ok(result) => Ok(result),
            Err(e) => Err(Box::new(e)),
        }
    }
}
