pub mod email;
pub mod error;
pub mod user;

#[deny(dead_code)]
pub(super) fn init() {
    user::init();
}
