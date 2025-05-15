// This is free and unencumbered software released into the public domain.

use bs58;
use uuid::Uuid;

/// Generate an OpenAI-style identifier with the given prefix.
pub fn generate_openai_id(prefix: &str) -> String {
    let uuid = Uuid::new_v4();
    let uuid_bytes = uuid.as_bytes();
    let encoded = bs58::encode(uuid_bytes).into_string();
    format!("{}-{}", prefix, encoded)
}
