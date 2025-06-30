use anyhow::{anyhow, Result};
use base64::{Engine as _, engine::general_purpose};
use bs58;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
};
use spl_token::instruction as token_instruction;
use std::str::FromStr;

// Convert base58 string to Pubkey
pub fn base58_to_pubkey(base58_str: &str) -> Result<Pubkey> {
    Pubkey::from_str(base58_str).map_err(|e| anyhow!("Invalid public key: {}", e))
}

// Convert base58 string to Keypair
pub fn base58_to_keypair(base58_str: &str) -> Result<Keypair> {
    let bytes = bs58::decode(base58_str)
        .into_vec()
        .map_err(|e| anyhow!("Invalid secret key: {}", e))?;
    
    if bytes.len() != 64 {
        return Err(anyhow!("Invalid secret key length"));
    }
    
    Ok(Keypair::from_bytes(&bytes)?)
}

// Convert Pubkey to base58 string
pub fn pubkey_to_base58(pubkey: &Pubkey) -> String {
    pubkey.to_string()
}

// Convert bytes to base64 string
pub fn bytes_to_base64(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}

// Convert base64 string to bytes
pub fn base64_to_bytes(base64_str: &str) -> Result<Vec<u8>> {
    general_purpose::STANDARD
        .decode(base64_str)
        .map_err(|e| anyhow!("Invalid base64: {}", e))
}

// Validate Solana address format
pub fn is_valid_solana_address(address: &str) -> bool {
    if address.len() < 32 || address.len() > 44 {
        return false;
    }
    
    // Check if it's valid base58
    bs58::decode(address).into_vec().is_ok()
}

// Create system program transfer instruction
pub fn create_transfer_instruction(
    from: &Pubkey,
    to: &Pubkey,
    lamports: u64,
) -> (Pubkey, Vec<u8>) {
    let instruction = system_instruction::transfer(from, to, lamports);
    (instruction.program_id, instruction.data)
}

// Create SPL token mint instruction
pub fn create_mint_instruction(
    mint: &Pubkey,
    destination: &Pubkey,
    authority: &Pubkey,
    amount: u64,
) -> (Pubkey, Vec<u8>) {
    let instruction = token_instruction::mint_to(
        &spl_token::id(),
        mint,
        destination,
        authority,
        &[],
        amount,
    );
    (instruction.program_id, instruction.data)
}

// Create SPL token transfer instruction
pub fn create_token_transfer_instruction(
    source: &Pubkey,
    destination: &Pubkey,
    owner: &Pubkey,
    amount: u64,
) -> (Pubkey, Vec<u8>) {
    let instruction = token_instruction::transfer(
        &spl_token::id(),
        source,
        destination,
        owner,
        &[],
        amount,
    );
    (instruction.program_id, instruction.data)
}

// Create SPL token initialize mint instruction
pub fn create_initialize_mint_instruction(
    mint: &Pubkey,
    decimals: u8,
    mint_authority: &Pubkey,
) -> (Pubkey, Vec<u8>) {
    let instruction = token_instruction::initialize_mint(
        &spl_token::id(),
        mint,
        mint_authority,
        Some(mint_authority),
        decimals,
    );
    (instruction.program_id, instruction.data)
} 