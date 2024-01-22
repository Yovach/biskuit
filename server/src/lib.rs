use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use jwt::{Header, Token, VerifyWithKey};
use serde::Serialize;
use serde_json::Value;
use sha2::Sha256;
use std::{
    collections::BTreeMap,
    env,
    fmt::{self, Formatter},
    time::SystemTime,
};

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

#[derive(Serialize)]
pub enum JwtErrorCode {
    InvalidKey,
    InvalidSubject,
    InvalidExpiration,
    Expired,
}

impl fmt::Display for JwtErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JwtErrorCode::InvalidKey => "JWT_INVALID_KEY",
                JwtErrorCode::Expired => "JWT_EXPIRED",
                JwtErrorCode::InvalidSubject => "JWT_INVALID_SUBJECT",
                JwtErrorCode::InvalidExpiration => "JWT_INVALID_SUBJECT",
            }
        )
    }
}

pub fn is_jwt_valid(jwt: &String, sub: &String) -> Result<(), JwtErrorCode> {
    let secret_env = env::var("JWT_SECRET").expect("i expected a value here");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret_env.as_bytes()).unwrap();
    let verification: Result<Token<Header, BTreeMap<String, Value>, _>, _> =
        jwt.verify_with_key(&key);

    if let Err(_e) = verification {
        return Err(JwtErrorCode::InvalidKey);
    }

    let jwt_data = verification.unwrap();
    let claims = jwt_data.claims();

    // retrieve sub value from JWT claims
    let subject = claims.get("sub");
    if let Some(value) = subject {
        // check if sub value is correct
        if !sub.eq(value) {
            return Err(JwtErrorCode::InvalidSubject);
        }
    } else {
        return Err(JwtErrorCode::InvalidSubject);
    }

    let exp = claims.get("exp");
    if let Some(expiration) = exp {
        if let Some(exp_val) = expiration.as_u64() {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if exp_val.lt(&now) {
                return Err(JwtErrorCode::Expired);
            }
        } else {
            return Err(JwtErrorCode::InvalidExpiration);
        }
    } else {
        return Err(JwtErrorCode::InvalidExpiration);
    }

    return Ok(());
}
