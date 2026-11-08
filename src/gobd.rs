#![allow(dead_code)]
use sha2::{Digest, Sha256};

pub const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// Computes the SHA-256 tamper-evident cryptographic hash for a GoBD audit trail entry
pub fn calculate_entry_hash(
    prev_hash: &str,
    seq: i64,
    timestamp: &str,
    entity_type: &str,
    entity_id: &str,
    action: &str,
    details: &str,
    actor: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(prev_hash.as_bytes());
    hasher.update(b"|");
    hasher.update(seq.to_string().as_bytes());
    hasher.update(b"|");
    hasher.update(timestamp.as_bytes());
    hasher.update(b"|");
    hasher.update(entity_type.as_bytes());
    hasher.update(b"|");
    hasher.update(entity_id.as_bytes());
    hasher.update(b"|");
    hasher.update(action.as_bytes());
    hasher.update(b"|");
    hasher.update(details.as_bytes());
    hasher.update(b"|");
    hasher.update(actor.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
