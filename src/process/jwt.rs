use crate::get_buf;
use crate::process::process_genpass;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    exp: u64,
}

pub fn process_sign(sub: &str, aud: &str, exp: Duration, secret_path: &str) -> Result<String> {
    let secret = get_buf(secret_path)?;

    let exp = chrono::Utc::now() + exp;

    let claims = Claims {
        sub: sub.to_string(),
        aud: aud.to_string(),
        exp: exp.timestamp() as u64,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn process_verify(token: &str, aud: &str, secret_path: &str) -> Result<bool> {
    let secret = get_buf(secret_path)?;

    let mut validation = jsonwebtoken::Validation::default();
    validation.set_audience(&[aud.to_string()]);

    if let Err(e) = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    ) {
        eprintln!("{}", e);
        Ok(false)
    } else {
        Ok(true)
    }
}

pub fn process_generate(len: u8) -> Result<String> {
    let secret = process_genpass(len, false, false, false, false)?;
    Ok(secret)
}
