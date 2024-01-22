use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use jwt::{Header, Token, VerifyWithKey};
use serde_json::Value;
use sha2::Sha256;
use std::{collections::BTreeMap, env, time::SystemTime};

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

pub fn is_jwt_valid(jwt: &String, sub: &String) -> Option<String> {
    let secret_env = env::var("JWT_SECRET").expect("i expected a value here");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret_env.as_bytes()).unwrap();
    let verification: Result<Token<Header, BTreeMap<String, Value>, _>, _> =
        jwt.verify_with_key(&key);
    if verification.is_err() {
        return Some("invalid token".to_string());
    }

    let jwt_data = verification.unwrap();
    let claims = jwt_data.claims();

    // here, we check the subject
    let subject = claims.get("sub");
    if subject.is_none() || !subject.unwrap().eq(sub) {
        return Some("the subject is not valid".to_string());
    }

    let exp = claims.get("exp");
    if exp.is_none() {
        return Some("the expiration is not valid".to_string());
    }

    let expiration = exp.unwrap().as_u64();
    if expiration.is_none() {
        return Some("the expiration is not valid".to_string());
    }

    let exp_val = expiration.unwrap();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    if exp_val.lt(&now) {
        return Some("the token is expired".to_string());
    }

    return None;
}
