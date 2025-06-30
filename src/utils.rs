use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use base64::{Engine as _, engine::general_purpose};

pub fn validate_pubkey(pubkey_str: &str) -> Result<Pubkey, String> {
    Pubkey::from_str(pubkey_str).map_err(|_| "Invalid public key format".to_string())
}

pub fn validate_base58(input: &str) -> Result<Vec<u8>, String> {
    bs58::decode(input)
        .into_vec()
        .map_err(|_| "Invalid base58 encoding".to_string())
}

pub fn encode_base64(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

pub fn decode_base64(data: &str) -> Result<Vec<u8>, String> {
    general_purpose::STANDARD.decode(data).map_err(|_| "Invalid base64 encoding".to_string())
}
