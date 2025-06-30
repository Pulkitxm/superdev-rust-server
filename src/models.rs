use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateTokenRequest {
    #[validate(length(min = 32, max = 44))]
    #[serde(rename = "mintAuthority")]
    pub mint_authority: String,
    #[validate(length(min = 32, max = 44))]
    pub mint: String,
    #[validate(range(min = 0, max = 18))]
    pub decimals: u8,
}

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

#[derive(Deserialize, Validate)]
pub struct SignMessageRequest {
    #[validate(length(min = 1))]
    pub message: String,
    #[validate(length(min = 32))]
    pub secret: String,
}

#[derive(Deserialize, Validate)]
pub struct VerifyMessageRequest {
    #[validate(length(min = 1))]
    pub message: String,
    #[validate(length(min = 1))]
    pub signature: String,
    #[validate(length(min = 32, max = 44))]
    pub pubkey: String,
}

#[derive(Deserialize, Validate)]
pub struct SendSolRequest {
    #[validate(length(min = 32, max = 44))]
    pub from: String,
    #[validate(length(min = 32, max = 44))]
    pub to: String,
    #[validate(range(min = 1))]
    pub lamports: u64,
}

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

#[derive(Serialize)]
pub struct InstructionResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMeta>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct AccountMeta {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct VerifyMessageResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct SendSolResponse {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    pub program_id: String,
    pub accounts: Vec<SendTokenAccountMeta>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct SendTokenAccountMeta {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}
