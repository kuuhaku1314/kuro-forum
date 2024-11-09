pub mod local_config;

pub fn config() -> local_config::Config {
    local_config::config()
}

#[deny(dead_code)]
pub fn init() {
    local_config::init()
}
