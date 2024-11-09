use crate::cache;
use crate::common::Result;
use crate::dao::user::*;
use crate::datasource::new_db;
use crate::entity::user::NewUserTab;
use crate::model::user;
use crate::model::user::UserTokenPayload;
use crate::util::dbutil;
use crate::util::time;
use scopeguard::defer;
use std::collections::HashMap;
use tracing::info;

use crate::constant;
use crate::service::email;
use crate::service::error::{
    ERR_INVALID_PARAM, ERR_INVALID_TOKEN, ERR_USER_EXISTED, ERR_USER_NOT_FOUND,
};
use crate::service::user::crypt::verify_encrypted_password;
use tracing::{error, instrument};

#[deny(dead_code)]
pub(super) fn init() {
    crypt::init();
}

mod crypt {
    use crate::common::Result;
    use crate::config;
    use crate::model::user::UserTokenPayload;
    use crate::util::encrypt::encrypt_md5;
    use crate::util::rand;
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
    use std::sync::LazyLock;
    #[deny(dead_code)]
    pub(super) fn init() {
        LazyLock::force(&JWT_PRIVATE_KEY);
        LazyLock::force(&TOKEN_SALT);
    }
    pub static JWT_PRIVATE_KEY: LazyLock<&'static str> = LazyLock::new(|| {
        Box::new(
            config::config()
                .get_string("user.token.jwt.secret")
                .unwrap(),
        )
        .leak()
    });

    pub static TOKEN_SALT: LazyLock<&'static str> =
        LazyLock::new(|| Box::new(config::config().get_string("user.token.salt").unwrap()).leak());

    fn generate_secret(password: &str) -> String {
        encrypt_md5(password, *TOKEN_SALT)
    }

    fn generate_salt() -> String {
        rand::rand_str(32)
    }

    pub fn generate_token(
        userid: i64,
        username: String,
        password: String,
        expired_time: i64,
    ) -> Result<(String, String)> {
        let secret = generate_secret(password.as_str());
        let token = encode(
            &Header::default(),
            &UserTokenPayload {
                userid,
                username,
                secret: secret.to_owned(),
                exp: expired_time as usize,
            },
            &EncodingKey::from_secret(JWT_PRIVATE_KEY.as_bytes()),
        )?;
        Ok((token, secret))
    }

    pub fn generate_encrypted_password(user_input_password: &str) -> (String, String) {
        let salt = generate_salt();
        (
            encrypt_md5(user_input_password, salt.to_owned().as_str()),
            salt,
        )
    }

    pub fn verify_encrypted_password(
        user_input_password: &str,
        salt: &str,
        encrypted_password: &str,
    ) -> bool {
        encrypt_md5(user_input_password, salt.to_owned().as_str()) == encrypted_password
    }

    pub fn verify_token(token: &str) -> Result<UserTokenPayload> {
        Ok(decode::<UserTokenPayload>(
            token,
            &DecodingKey::from_secret(JWT_PRIVATE_KEY.as_bytes()),
            &Validation::default(),
        )?
        .claims)
    }
}

#[instrument(err)]
pub fn signup(user: &user::User) -> Result<i64> {
    let now = time::now_timestamp();
    let (password, salt) = crypt::generate_encrypted_password(user.password.as_str());
    let record = &NewUserTab {
        nickname: user.nickname.to_owned(),
        username: user.username.to_owned(),
        password,
        salt,
        create_time: now,
        update_time: now,
    };
    let mut tm = dbutil::new_transaction()?;
    let result = create_user(tm.conn(), record);
    if let Err(err) = result {
        return if dbutil::is_duplicate(&*err) {
            Err(ERR_USER_EXISTED.into())
        } else {
            error!("[create_user]{}", err);
            Err(err)
        };
    }
    let mut email_data = HashMap::new();
    email_data.insert("username", user.username.to_owned());
    email_data.insert("url", "https://www.baidu.com".to_owned());
    email::send_email_by_template(
        &user.email,
        &email_data,
        constant::email::EMAIL_KEY_USER_REGISTER,
    )?;
    tm.commit()?;
    result
}

// fields(%context = format!("{}",backtrace::Backtrace::force_capture()))
#[instrument(err)]
pub fn login(
    username: &str,
    user_input_password: &str,
    token_expiration_period: ::time::Duration,
) -> Result<String> {
    let start = time::now_timestamp();
    defer!(
        let end = time::now_timestamp();
        info!("login cost time={}", end - start)
    );
    let result = get_user_by_username(&mut new_db(), username).map_err(|err| {
        if dbutil::is_not_found(&*err) {
            ERR_USER_NOT_FOUND.into()
        } else {
            err
        }
    })?;
    let ok = verify_encrypted_password(
        user_input_password,
        result.salt.as_str(),
        result.password.as_str(),
    );
    if !ok {
        return Err(ERR_INVALID_PARAM.into());
    }
    let (token, secret) = crypt::generate_token(
        result.id,
        result.username,
        result.password,
        time::now_timestamp() + token_expiration_period.as_seconds_f64() as i64,
    )?;
    cache::user_cache().store(result.id, secret);
    Ok(token)
}

#[instrument(err)]
pub fn decrypt_token(token: &str) -> Result<UserTokenPayload> {
    let user_token_info = crypt::verify_token(token)?;
    if user_token_info.exp < time::now_timestamp() as usize {
        return Err(ERR_INVALID_TOKEN.into());
    }
    Ok(user_token_info)
}
