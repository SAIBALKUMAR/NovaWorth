//! Time-based one-time password (RFC 6238) using HMAC-SHA256, dependency-free:
//! HMAC is assembled from the `sha2` crate directly. Secrets are 32-byte hex.

use argon2::password_hash::rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;
use common::errors::AppError;

const STEP_SECS: u64 = 30;
const DIGITS: u32 = 6;

fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; 32] {
    let mut key = key.to_vec();
    if key.len() > 64 {
        key = Sha256::digest(&key).to_vec();
    }
    let mut ipad = [0x36u8; 64];
    let mut opad = [0x5cu8; 64];
    for i in 0..key.len() {
        ipad[i] ^= key[i];
        opad[i] ^= key[i];
    }
    let mut inner = ipad.to_vec();
    inner.extend_from_slice(msg);
    let h_inner = Sha256::digest(&inner);
    let mut outer = opad.to_vec();
    outer.extend_from_slice(&h_inner);
    let h = Sha256::digest(&outer);
    let mut out = [0u8; 32];
    out.copy_from_slice(&h);
    out
}

fn dynamic_truncate(h: &[u8]) -> u32 {
    let off = (h[h.len() - 1] & 0x0f) as usize;
    (((h[off] & 0x7f) as u32) << 24)
        | ((h[off + 1] as u32) << 16)
        | ((h[off + 2] as u32) << 8)
        | (h[off + 3] as u32)
}

fn hotp(key: &[u8], counter: u64) -> u32 {
    dynamic_truncate(&hmac_sha256(key, &counter.to_be_bytes())) % 10u32.pow(DIGITS)
}

fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

/// Generate a fresh 32-byte secret (hex).
pub fn generate_secret() -> String {
    let mut b = [0u8; 32];
    OsRng.fill_bytes(&mut b);
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// Verify a 6-digit code, allowing ±1 time step (RFC 6238 tolerance).
pub fn verify(secret_hex: &str, code: &str, unix_secs: u64) -> bool {
    let Some(key) = hex_decode(secret_hex) else {
        return false;
    };
    let Ok(code) = code.parse::<u32>() else {
        return false;
    };
    let step = unix_secs / STEP_SECS;
    for c in [step.saturating_sub(1), step, step + 1] {
        if hotp(&key, c) == code {
            return true;
        }
    }
    false
}

/// Begin (or reset) 2FA for a user; returns the new hex secret.
pub async fn setup(pool: &PgPool, user_id: Uuid) -> Result<String, AppError> {
    let secret = generate_secret();
    sqlx::query(
        "INSERT INTO user_totp (user_id, secret, confirmed, updated_at)
         VALUES ($1, $2, false, now())
         ON CONFLICT (user_id) DO UPDATE SET secret = $2, confirmed = false, updated_at = now()",
    )
    .bind(user_id)
    .bind(&secret)
    .execute(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(secret)
}

/// The confirmed TOTP secret for a user, if 2FA is enabled.
pub async fn enabled_secret(pool: &PgPool, user_id: Uuid) -> Result<Option<String>, AppError> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT secret FROM user_totp WHERE user_id = $1 AND confirmed = true",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(row.map(|(s,)| s))
}

/// Confirm 2FA by checking `code` against the stored secret and marking it enabled.
pub async fn confirm_code(
    pool: &PgPool,
    user_id: Uuid,
    code: &str,
    unix_secs: u64,
) -> Result<(), AppError> {
    let row: Option<(String,)> = sqlx::query_as("SELECT secret FROM user_totp WHERE user_id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?;

    let Some((secret,)) = row else {
        return Err(AppError::Validation("2FA is not set up".into()));
    };
    if !verify(&secret, code, unix_secs) {
        return Err(AppError::Validation("Invalid 2FA code".into()));
    }

    sqlx::query("UPDATE user_totp SET confirmed = true, updated_at = now() WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
    Ok(())
}