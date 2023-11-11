use chrono::Duration;
use chrono::{DateTime, Local};
use hmac::{digest::InvalidLength, Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha256;
use std::{collections::BTreeMap, ops::Add, ops::Sub};

#[derive(Debug, Clone)]
pub struct JWT {
    sign_key: Hmac<Sha256>,
    exp_duration: i64,
}

pub trait JWTTrait {
    fn init(secret_key: &str, exp_duration: i64) -> Result<Self, ()>
    where
        Self: Sized;
    fn generate(self) -> Result<String, jwt::Error>;
    fn verify(self, token_str: &str) -> Result<(), &str>;
}

impl JWTTrait for JWT {
    fn init(secret_key: &str, exp_duration: i64) -> Result<Self, ()> {
        let result: Result<Hmac<Sha256>, InvalidLength> =
            Hmac::new_from_slice(secret_key.as_bytes());
        match result {
            Ok(sign_key) => Ok(Self {
                sign_key,
                exp_duration,
            }),
            Err(_) => Err(()),
        }
    }

    fn generate(self) -> Result<String, jwt::Error> {
        let mut claims: BTreeMap<&str, &str> = BTreeMap::new();

        let now: DateTime<Local> = Local::now();
        let now_str = now.timestamp().to_string();

        let exp = now.add(Duration::seconds(self.exp_duration));
        let exp_str = exp.timestamp().to_string();

        claims.insert("sub", "user");
        claims.insert("iss", "system");
        claims.insert("iat", &now_str);
        claims.insert("exp", &exp_str);

        claims.sign_with_key(&self.sign_key)
    }

    fn verify(self, token_str: &str) -> Result<(), &str> {
        let verification_result: Result<Token<Header, BTreeMap<String, String>, _>, jwt::Error> =
            token_str.verify_with_key(&self.sign_key);

        if verification_result.is_err() {
            return Err("Verification failed");
        }

        let res = verification_result.unwrap();
        let exp = if res.claims().get("exp").is_some() {
            res.claims().get("exp").unwrap()
        } else {
            return Err("Cannot get 'exp' claim");
        };

        let parsed_exp = if exp.parse::<i64>().is_ok() {
            exp.parse().unwrap()
        } else {
            return Err("Cannot parse 'exp' to integer");
        };

        let local: DateTime<Local> = Local::now();
        let exp_duration = Duration::seconds(parsed_exp);
        let sub_duration = local.sub(exp_duration);

        if sub_duration.timestamp() > 0 {
            return Err("Token is expired");
        }

        Ok(())
    }
}
