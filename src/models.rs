use serde::{Deserialize, Serialize};
use validator::Validate;

// Common response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// Ping response
#[derive(Serialize)]
pub struct PingResponse {
    pub message: String,
}

// Keypair response
#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

// Token create request
#[derive(Deserialize, Validate)]
pub struct CreateTokenRequest {
    #[validate(length(min = 32, max = 44))]
    pub mint_authority: String,
    #[validate(length(min = 32, max = 44))]
    pub mint: String,
    #[validate(range(min = 0, max = 9))]
    pub decimals: u8,
}

// Token mint request
#[derive(Deserialize, Validate)]
pub struct MintTokenRequest {
    #[validate(length(min = 32, max = 44))]
    pub mint: String,
    #[validate(length(min = 32, max = 44))]
    pub destination: String,
    #[validate(length(min = 32, max = 44))]
    pub authority: String,
    #[validate(range(min = 1))]
    pub amount: u64,
}

// Message sign request
#[derive(Deserialize, Validate)]
pub struct SignMessageRequest {
    #[validate(length(min = 1))]
    pub message: String,
    #[validate(length(min = 32, max = 88))]
    pub secret: String,
}

// Message verify request
#[derive(Deserialize, Validate)]
pub struct VerifyMessageRequest {
    #[validate(length(min = 1))]
    pub message: String,
    #[validate(length(min = 1))]
    pub signature: String,
    #[validate(length(min = 32, max = 44))]
    pub pubkey: String,
}

// Send SOL request
#[derive(Deserialize, Validate)]
pub struct SendSolRequest {
    #[validate(length(min = 32, max = 44))]
    pub from: String,
    #[validate(length(min = 32, max = 44))]
    pub to: String,
    #[validate(range(min = 1))]
    pub lamports: u64,
}

// Send token request
#[derive(Deserialize, Validate)]
pub struct SendTokenRequest {
    #[validate(length(min = 32, max = 44))]
    pub destination: String,
    #[validate(length(min = 32, max = 44))]
    pub mint: String,
    #[validate(length(min = 32, max = 44))]
    pub owner: String,
    #[validate(range(min = 1))]
    pub amount: u64,
}

// Instruction response
#[derive(Serialize)]
pub struct InstructionResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMeta>,
    pub instruction_data: String,
}

// Account metadata
#[derive(Serialize)]
pub struct AccountMeta {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

// Sign message response
#[derive(Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

// Verify message response
#[derive(Serialize)]
pub struct VerifyMessageResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
} 