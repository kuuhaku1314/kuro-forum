mod sqlite;

pub use self::sqlite::new_db;

#[deny(dead_code)]
pub fn init() {
    sqlite::init()
}
