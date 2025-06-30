use axum::{extract::Json, http::StatusCode, response::Json as ResponseJson};
use solana_sdk::{
    signature::{Keypair, Signature, Signer},
    system_instruction,
};
use spl_token::instruction::{initialize_mint, mint_to, transfer};
use validator::Validate;
use crate::models::*;
use crate::utils::*;

pub async fn ping() -> ResponseJson<ApiResponse<String>> {
    ResponseJson(ApiResponse::success("pong".to_string()))
}

pub async fn generate_keypair() -> ResponseJson<ApiResponse<KeypairResponse>> {
    let keypair = Keypair::new();
    let response = KeypairResponse {
        pubkey: keypair.pubkey().to_string(),
        secret: bs58::encode(keypair.to_bytes()).into_string(),
    };
    ResponseJson(ApiResponse::success(response))
}

pub async fn create_token(
    Json(payload): Json<CreateTokenRequest>,
) -> Result<ResponseJson<ApiResponse<InstructionResponse>>, StatusCode> {
    if let Err(errors) = payload.validate() {
        return Ok(ResponseJson(ApiResponse::error(format!(
            "Validation error: {:?}",
            errors
        ))));
    }

    let mint_authority = match validate_pubkey(&payload.mint_authority) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(ResponseJson(ApiResponse::error(e))),
    };

    let mint = match validate_pubkey(&payload.mint) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(ResponseJson(ApiResponse::error(e))),
    };

    let instruction = initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        Some(&mint_authority),
        payload.decimals,
    )
    .map_err(|e| {
        return ResponseJson(ApiResponse::error(format!("Failed to create instruction: {}", e)));
    });

    match instruction {
        Ok(inst) => {
            let accounts: Vec<crate::models::AccountMeta> = inst
                .accounts
                .iter()
                .map(|acc| crate::models::AccountMeta {
                    pubkey: acc.pubkey.to_string(),
                    is_signer: acc.is_signer,
                    is_writable: acc.is_writable,
                })
                .collect();

            let response = InstructionResponse {
                program_id: inst.program_id.to_string(),
                accounts,
                instruction_data: encode_base64(&inst.data),
            };

            Ok(ResponseJson(ApiResponse::success(response)))
        }
        Err(response) => Ok(response),
    }
}

pub async fn mint_token(
    Json(payload): Json<MintTokenRequest>,
) -> Result<ResponseJson<ApiResponse<InstructionResponse>>, StatusCode> {
    if let Err(errors) = payload.validate() {
        return Ok(ResponseJson(ApiResponse::error(format!(
            "Validation error: {:?}",
            errors
        ))));
    }

    let mint = match validate_pubkey(&payload.mint) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(ResponseJson(ApiResponse::error(e))),
    };

    let destination = match validate_pubkey(&payload.destination) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(ResponseJson(ApiResponse::error(e))),
    };

    let authority = match validate_pubkey(&payload.authority) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(ResponseJson(ApiResponse::error(e))),
    };

    let instruction = mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[],
        payload.amount,
    )
    .map_err(|e| {
        return ResponseJson(ApiResponse::error(format!("Failed to create instruction: {}", e)));
    });

    match instruction {
        Ok(inst) => {
            let accounts: Vec<crate::models::AccountMeta> = inst
                .accounts
                .iter()
                .map(|acc| crate::models::AccountMeta {
                    pubkey: acc.pubkey.to_string(),
                    is_signer: acc.is_signer,
                    is_writable: acc.is_writable,
                })
                .collect();

            let response = InstructionResponse {
                program_id: inst.program_id.to_string(),
                accounts,
                instruction_data: encode_base64(&inst.data),
            };

            Ok(ResponseJson(ApiResponse::success(response)))
        }
        Err(response) => Ok(response),
    }
}

pub async fn sign_message(
    Json(payload): Json<SignMessageRequest>,
) -> Result<ResponseJson<ApiResponse<SignMessageResponse>>, StatusCode> {
    if let Err(errors) = payload.validate() {
        return Ok(ResponseJson(ApiResponse::error(format!(
            "Validation error: {:?}",
            errors
        ))));
    }

    let secret_bytes = match validate_base58(&payload.secret) {
        Ok(bytes) => bytes,
        Err(e) => return Ok(ResponseJson(ApiResponse::error(e))),
    };

    if secret_bytes.len() != 64 {
        return Ok(ResponseJson(ApiResponse::error(
            "Invalid secret key length".to_string(),
        )));
    }

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            return Ok(ResponseJson(ApiResponse::error(
                "Invalid secret key format".to_string(),
            )))
        }
    };

    let message_bytes = payload.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);

    let response = SignMessageResponse {
        signature: encode_base64(&signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: payload.message,
    };

    Ok(ResponseJson(ApiResponse::success(response)))
}

pub async fn verify_message(
    Json(payload): Json<VerifyMessageRequest>,
) -> Result<ResponseJson<ApiResponse<VerifyMessageResponse>>, StatusCode> {
    if let Err(errors) = payload.validate() {
        return Ok(ResponseJson(ApiResponse::error(format!(
            "Validation error: {:?}",
            errors
        ))));
    }

    let pubkey = match validate_pubkey(&payload.pubkey) {
        Ok(pk) => pk,
        Err(e) => return Ok(ResponseJson(ApiResponse::error(e))),
    };

    let signature_bytes = match decode_base64(&payload.signature) {
        Ok(bytes) => bytes,
        Err(e) => return Ok(ResponseJson(ApiResponse::error(e))),
    };

    if signature_bytes.len() != 64 {
        return Ok(ResponseJson(ApiResponse::error(
            "Invalid signature length".to_string(),
        )));
    }

    let signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => {
            return Ok(ResponseJson(ApiResponse::error(
                "Invalid signature format".to_string(),
            )))
        }
    };

    let message_bytes = payload.message.as_bytes();
    let is_valid = signature.verify(&pubkey.to_bytes(), message_bytes);

    let response = VerifyMessageResponse {
        valid: is_valid,
        message: payload.message,
        pubkey: payload.pubkey,
    };

    Ok(ResponseJson(ApiResponse::success(response)))
}

pub async fn send_sol(
    Json(payload): Json<SendSolRequest>,
) -> Result<ResponseJson<ApiResponse<SendSolResponse>>, StatusCode> {
    if let Err(errors) = payload.validate() {
        return Ok(ResponseJson(ApiResponse::error(format!(
            "Validation error: {:?}",
            errors
        ))));
    }

    let from = match validate_pubkey(&payload.from) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(ResponseJson(ApiResponse::error(e))),
    };

    let to = match validate_pubkey(&payload.to) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(ResponseJson(ApiResponse::error(e))),
    };

    if payload.lamports == 0 {
        return Ok(ResponseJson(ApiResponse::error(
            "Amount must be greater than 0".to_string(),
        )));
    }

    if from == to {
        return Ok(ResponseJson(ApiResponse::error(
            "Cannot send to the same address".to_string(),
        )));
    }

    let instruction = system_instruction::transfer(&from, &to, payload.lamports);

    let response = SendSolResponse {
        program_id: instruction.program_id.to_string(),
        accounts: instruction
            .accounts
            .iter()
            .map(|acc| acc.pubkey.to_string())
            .collect(),
        instruction_data: encode_base64(&instruction.data),
    };

    Ok(ResponseJson(ApiResponse::success(response)))
}

pub async fn send_token(
    Json(payload): Json<SendTokenRequest>,
) -> Result<ResponseJson<ApiResponse<SendTokenResponse>>, StatusCode> {
    if let Err(errors) = payload.validate() {
        return Ok(ResponseJson(ApiResponse::error(format!(
            "Validation error: {:?}",
            errors
        ))));
    }

    let destination = match validate_pubkey(&payload.destination) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(ResponseJson(ApiResponse::error(e))),
    };

    let mint = match validate_pubkey(&payload.mint) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(ResponseJson(ApiResponse::error(e))),
    };

    let owner = match validate_pubkey(&payload.owner) {
        Ok(pubkey) => pubkey,
        Err(e) => return Ok(ResponseJson(ApiResponse::error(e))),
    };

    if payload.amount == 0 {
        return Ok(ResponseJson(ApiResponse::error(
            "Amount must be greater than 0".to_string(),
        )));
    }

    let source_ata = spl_associated_token_account::get_associated_token_address(&owner, &mint);
    
    if source_ata == destination {
        return Ok(ResponseJson(ApiResponse::error(
            "Cannot send to the same token account".to_string(),
        )));
    }

    let instruction = transfer(
        &spl_token::id(),
        &source_ata,
        &destination,
        &owner,
        &[],
        payload.amount,
    )
    .map_err(|e| {
        return ResponseJson(ApiResponse::error(format!("Failed to create instruction: {}", e)));
    });

    match instruction {
        Ok(inst) => {
            let accounts: Vec<SendTokenAccountMeta> = inst
                .accounts
                .iter()
                .map(|acc| SendTokenAccountMeta {
                    pubkey: acc.pubkey.to_string(),
                    is_signer: acc.is_signer,
                })
                .collect();

            let response = SendTokenResponse {
                program_id: inst.program_id.to_string(),
                accounts,
                instruction_data: encode_base64(&inst.data),
            };

            Ok(ResponseJson(ApiResponse::success(response)))
        }
        Err(response) => Ok(response),
    }
}
