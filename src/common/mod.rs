pub mod http_msg;

pub type Error = Box<dyn std::error::Error>;

pub type Result<T> = std::result::Result<T, Error>;
