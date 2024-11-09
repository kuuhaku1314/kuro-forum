pub mod dbutil;
pub mod email;
pub mod encrypt;
pub mod rand;
pub mod template;
pub mod time;

#[deny(dead_code)]
pub(super) fn init() {
    template::init();
    email::init();
}
