use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::SqliteConnection;
use std::fs;
use std::sync::LazyLock;

use crate::config;

#[deny(dead_code)]
pub(super) fn init() {
    ensure_db_initialized();
    LazyLock::force(&DB_POOL);
}

static DB_POOL: LazyLock<Pool<ConnectionManager<SqliteConnection>>> =
    LazyLock::new(init_connection_pool);

pub fn new_db() -> PooledConnection<ConnectionManager<SqliteConnection>> {
    DB_POOL.get().unwrap()
}
fn init_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let url = config::config()
        .get_string("database.url")
        .unwrap_or_default();
    let manager = ConnectionManager::<SqliteConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
fn ensure_db_initialized() {
    let db =
        rusqlite::Connection::open(config::config().get_string("database.url").unwrap()).unwrap();
    let init_sql =
        fs::read_to_string(config::config().get_string("database.init_sql").unwrap()).unwrap();
    init_sql
        .split(";")
        .map(|sql| sql.trim())
        .filter(|sql| !sql.is_empty())
        .for_each(|sql| {
            db.execute(sql, rusqlite::params![])
                .expect("init sqlite error");
        });
}
