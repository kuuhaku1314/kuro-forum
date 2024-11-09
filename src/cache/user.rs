use moka::sync::Cache;
use std::hash::RandomState;
use std::sync::LazyLock;
use std::time::Duration;

#[deny(dead_code)]
pub(super) fn init() {
    LazyLock::force(&INTERNAL_USER_CACHE);
}

static INTERNAL_USER_CACHE: LazyLock<Cache<i64, String, RandomState>> = LazyLock::new(|| {
    Cache::builder()
        .time_to_live(Duration::from_secs(30 * 60))
        .max_capacity(10000)
        .build()
});

pub struct UserCache {
    __private: (),
}

pub(super) fn user_cache() -> UserCache {
    UserCache { __private: () }
}

impl UserCache {
    pub fn store(&self, userid: i64, secret: String) {
        INTERNAL_USER_CACHE.insert(userid, secret)
    }

    pub fn load(&self, userid: i64) -> Option<String> {
        INTERNAL_USER_CACHE.get(&userid)
    }

    pub fn remove(&self, userid: i64) -> Option<String> {
        INTERNAL_USER_CACHE.remove(&userid)
    }
}
