use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn rand_str(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}
