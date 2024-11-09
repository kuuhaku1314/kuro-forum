pub mod user;

#[deny(dead_code)]
pub fn init() {
    user::init()
}

pub fn user_cache() -> user::UserCache {
    user::user_cache()
}
