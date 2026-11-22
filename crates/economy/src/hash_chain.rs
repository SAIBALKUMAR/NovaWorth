//! SHA-256 hash chain for ledger tamper-evidence.
//!
//! Each ledger entry commits to the digest of the previous entry **plus** its own
//! payload, so any out-of-band edit to an entry breaks every downstream hash.
//! Verification is cheap and can run in the reconciliation worker.

use sha2::{Digest, Sha256};

/// Hex-encode the digest of `bytes`.
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// SHA-256 of an arbitrary payload, hex-encoded.
pub fn sha256_hex(payload: &str) -> String {
    hex(&Sha256::digest(payload.as_bytes()))
}

/// Chain one link: `sha256(prev | seed)`, treating a missing predecessor as the
/// chain root (`GENESIS`).
pub fn next_hash(prev_hash: Option<&str>, seed: &str) -> String {
    sha256_hex(&format!("{}|{seed}", prev_hash.unwrap_or("GENESIS")))
}

/// Build a ledger entry's hash from its human-readable fields.
pub fn entry_hash(
    prev_hash: Option<&str>,
    user_id: &str,
    currency: &str,
    amount: i64,
    reason: &str,
) -> String {
    next_hash(prev_hash, &format!("{user_id}|{currency}|{amount}|{reason}"))
}

/// Verify a contiguous chain of `(hash, seed)` pairs.
///
/// Rebuilds each link via [`next_hash`] and returns `false` the moment any
/// reconstructed digest differs from the stored one.
pub fn verify_chain<'a>(
    items: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> bool {
    let mut prev: Option<String> = None;
    for (hash, seed) in items {
        let expected = next_hash(prev.as_deref(), seed);
        if hash != expected {
            return false;
        }
        prev = Some(hash.to_string());
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_deterministic() {
        let a = entry_hash(None, "u1", "XP", 100, "GATE_PASS");
        let b = entry_hash(None, "u1", "XP", 100, "GATE_PASS");
        assert_eq!(a, b);
    }

    #[test]
    fn hash_changes_with_payload() {
        let a = entry_hash(None, "u1", "XP", 100, "GATE_PASS");
        let b = entry_hash(None, "u1", "XP", 101, "GATE_PASS");
        assert_ne!(a, b);
    }

    #[test]
    fn chain_verifies_when_intact() {
        let h1 = sha256_hex("GENESIS|s1");
        let h2 = sha256_hex(&format!("{h1}|s2"));
        assert!(verify_chain(vec![(h1.as_str(), "s1"), (h2.as_str(), "s2")]));
    }

    #[test]
    fn chain_fails_on_tamper() {
        let h1 = sha256_hex("GENESIS|s1");
        let forged = sha256_hex("GENESIS|tampered");
        // Second link claims to chain from h1 but its seed differs.
        assert!(!verify_chain(vec![(h1.as_str(), "s1"), (forged.as_str(), "s1")]));
    }
}