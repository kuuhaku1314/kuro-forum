use crate::config::config;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;
use std::sync::LazyLock;

#[deny(dead_code)]
pub(super) fn init() {
    LazyLock::force(&INNER_SMTP_CLIENT);
}

static INNER_SMTP_CLIENT: LazyLock<SmtpTransport> = LazyLock::new(|| {
    SmtpTransport::relay(config().get_string("email.smtp").unwrap().as_str())
        .unwrap()
        .credentials(Credentials::new(
            config().get_string("email.username").unwrap(),
            config().get_string("email.password").unwrap(),
        ))
        .build()
});
pub struct SmtpClient {
    __private: (),
}

#[derive(Debug)]
pub struct EmailBody {
    pub from: String,
    pub to: String,
    pub title: String,
    pub content: String,
}

pub fn smtp_client() -> SmtpClient {
    SmtpClient { __private: () }
}

impl SmtpClient {
    pub fn send_email(&self, email_body: EmailBody) -> Result<(), Box<dyn Error>> {
        let email = Message::builder()
            .to(email_body.to.parse()?)
            .from(email_body.from.parse()?)
            .subject(email_body.title)
            .header(ContentType::TEXT_HTML)
            .body(email_body.content)?;
        match INNER_SMTP_CLIENT.send(&email) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
