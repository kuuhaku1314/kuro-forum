use crate::common::Result;
use crate::config::config;
use crate::constant::email::EmailKey;
use crate::util::email::EmailBody;
use crate::util::{email, template};
use std::collections::HashMap;
use tracing::{info, instrument};

#[instrument(err)]
pub fn send_email_by_template(
    receiver: &str,
    data: &HashMap<&'static str, String>,
    email_key: EmailKey,
) -> Result<()> {
    let email_title = format!("{}_{}_{}", email_key, "title", "cn");
    let email_content = format!("{}_{}_{}", email_key, "content", "cn");
    let title = template::template().render_template(&email_title, data)?;
    let content = template::template().render_template(&email_content, data)?;
    let email_body = EmailBody {
        from: config().get_string("email.address").unwrap_or_default(),
        to: receiver.to_owned(),
        title,
        content,
    };
    info!("send email={:?}", email_body);
    email::smtp_client().send_email(email_body)
}
