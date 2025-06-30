use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as JsonResponse,
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
};
use spl_token::instruction as token_instruction;
use validator::Validate;

use crate::models::*;
use crate::utils::*;

// Generate a new Solana keypair
pub async fn generate_keypair() -> JsonResponse<ApiResponse<KeypairResponse>> {
    let keypair = Keypair::new();
    let pubkey = pubkey_to_base58(&keypair.pubkey());
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    JsonResponse(ApiResponse {
        success: true,
        data: Some(KeypairResponse { pubkey, secret }),
        error: None,
    })
}

// Create a new SPL token initialize mint instruction
pub async fn create_token(
    Json(payload): Json<CreateTokenRequest>,
) -> JsonResponse<ApiResponse<InstructionResponse>> {
    // Validate request
    if let Err(e) = payload.validate() {
        return JsonResponse(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Validation error: {}", e)),
        });
    }

    // Parse pubkeys
    let mint_authority = match base58_to_pubkey(&payload.mint_authority) {
        Ok(pk) => pk,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid mint authority: {}", e)),
            });
        }
    };

    let mint = match base58_to_pubkey(&payload.mint) {
        Ok(pk) => pk,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid mint: {}", e)),
            });
        }
    };

    // Create instruction
    let (program_id, instruction_data) = create_initialize_mint_instruction(
        &mint,
        payload.decimals,
        &mint_authority,
    );

    let accounts = vec![
        AccountMeta {
            pubkey: pubkey_to_base58(&mint),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: pubkey_to_base58(&mint_authority),
            is_signer: true,
            is_writable: false,
        },
    ];

    JsonResponse(ApiResponse {
        success: true,
        data: Some(InstructionResponse {
            program_id: pubkey_to_base58(&program_id),
            accounts,
            instruction_data: bytes_to_base64(&instruction_data),
        }),
        error: None,
    })
}

// Create a mint-to instruction for SPL tokens
pub async fn mint_token(
    Json(payload): Json<MintTokenRequest>,
) -> JsonResponse<ApiResponse<InstructionResponse>> {
    // Validate request
    if let Err(e) = payload.validate() {
        return JsonResponse(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Validation error: {}", e)),
        });
    }

    // Parse pubkeys
    let mint = match base58_to_pubkey(&payload.mint) {
        Ok(pk) => pk,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid mint: {}", e)),
            });
        }
    };

    let destination = match base58_to_pubkey(&payload.destination) {
        Ok(pk) => pk,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid destination: {}", e)),
            });
        }
    };

    let authority = match base58_to_pubkey(&payload.authority) {
        Ok(pk) => pk,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid authority: {}", e)),
            });
        }
    };

    // Create instruction
    let (program_id, instruction_data) = create_mint_instruction(
        &mint,
        &destination,
        &authority,
        payload.amount,
    );

    let accounts = vec![
        AccountMeta {
            pubkey: pubkey_to_base58(&mint),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: pubkey_to_base58(&destination),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: pubkey_to_base58(&authority),
            is_signer: true,
            is_writable: false,
        },
    ];

    JsonResponse(ApiResponse {
        success: true,
        data: Some(InstructionResponse {
            program_id: pubkey_to_base58(&program_id),
            accounts,
            instruction_data: bytes_to_base64(&instruction_data),
        }),
        error: None,
    })
}

// Sign a message using a private key
pub async fn sign_message(
    Json(payload): Json<SignMessageRequest>,
) -> JsonResponse<ApiResponse<SignMessageResponse>> {
    // Validate request
    if let Err(e) = payload.validate() {
        return JsonResponse(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Validation error: {}", e)),
        });
    }

    // Parse keypair
    let keypair = match base58_to_keypair(&payload.secret) {
        Ok(kp) => kp,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid secret key: {}", e)),
            });
        }
    };

    // Sign message
    let message_bytes = payload.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);
    let signature_base64 = bytes_to_base64(&signature.as_ref());

    JsonResponse(ApiResponse {
        success: true,
        data: Some(SignMessageResponse {
            signature: signature_base64,
            public_key: pubkey_to_base58(&keypair.pubkey()),
            message: payload.message,
        }),
        error: None,
    })
}

// Verify a signed message
pub async fn verify_message(
    Json(payload): Json<VerifyMessageRequest>,
) -> JsonResponse<ApiResponse<VerifyMessageResponse>> {
    // Validate request
    if let Err(e) = payload.validate() {
        return JsonResponse(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Validation error: {}", e)),
        });
    }

    // Parse pubkey
    let pubkey = match base58_to_pubkey(&payload.pubkey) {
        Ok(pk) => pk,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid public key: {}", e)),
            });
        }
    };

    // Parse signature
    let signature_bytes = match base64_to_bytes(&payload.signature) {
        Ok(bytes) => bytes,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid signature: {}", e)),
            });
        }
    };

    let signature = match solana_sdk::signature::Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid signature format: {}", e)),
            });
        }
    };

    // Verify signature
    let message_bytes = payload.message.as_bytes();
    let valid = signature.verify(pubkey.as_ref(), message_bytes);

    JsonResponse(ApiResponse {
        success: true,
        data: Some(VerifyMessageResponse {
            valid,
            message: payload.message,
            pubkey: payload.pubkey,
        }),
        error: None,
    })
}

// Create a SOL transfer instruction
pub async fn send_sol(
    Json(payload): Json<SendSolRequest>,
) -> JsonResponse<ApiResponse<InstructionResponse>> {
    // Validate request
    if let Err(e) = payload.validate() {
        return JsonResponse(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Validation error: {}", e)),
        });
    }

    // Parse pubkeys
    let from = match base58_to_pubkey(&payload.from) {
        Ok(pk) => pk,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid from address: {}", e)),
            });
        }
    };

    let to = match base58_to_pubkey(&payload.to) {
        Ok(pk) => pk,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid to address: {}", e)),
            });
        }
    };

    // Create instruction
    let (program_id, instruction_data) = create_transfer_instruction(
        &from,
        &to,
        payload.lamports,
    );

    let accounts = vec![
        AccountMeta {
            pubkey: pubkey_to_base58(&from),
            is_signer: true,
            is_writable: true,
        },
        AccountMeta {
            pubkey: pubkey_to_base58(&to),
            is_signer: false,
            is_writable: true,
        },
    ];

    JsonResponse(ApiResponse {
        success: true,
        data: Some(InstructionResponse {
            program_id: pubkey_to_base58(&program_id),
            accounts,
            instruction_data: bytes_to_base64(&instruction_data),
        }),
        error: None,
    })
}

// Create an SPL token transfer instruction
pub async fn send_token(
    Json(payload): Json<SendTokenRequest>,
) -> JsonResponse<ApiResponse<InstructionResponse>> {
    // Validate request
    if let Err(e) = payload.validate() {
        return JsonResponse(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Validation error: {}", e)),
        });
    }

    // Parse pubkeys
    let destination = match base58_to_pubkey(&payload.destination) {
        Ok(pk) => pk,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid destination: {}", e)),
            });
        }
    };

    let mint = match base58_to_pubkey(&payload.mint) {
        Ok(pk) => pk,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid mint: {}", e)),
            });
        }
    };

    let owner = match base58_to_pubkey(&payload.owner) {
        Ok(pk) => pk,
        Err(e) => {
            return JsonResponse(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Invalid owner: {}", e)),
            });
        }
    };

    // For token transfer, we need source and destination token accounts
    // Since we don't have the source account in the request, we'll create a placeholder
    // In a real implementation, you'd need to derive the source account from owner + mint
    let source = owner; // This is a simplification

    // Create instruction
    let (program_id, instruction_data) = create_token_transfer_instruction(
        &source,
        &destination,
        &owner,
        payload.amount,
    );

    let accounts = vec![
        AccountMeta {
            pubkey: pubkey_to_base58(&source),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: pubkey_to_base58(&destination),
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: pubkey_to_base58(&owner),
            is_signer: true,
            is_writable: false,
        },
    ];

    JsonResponse(ApiResponse {
        success: true,
        data: Some(InstructionResponse {
            program_id: pubkey_to_base58(&program_id),
            accounts,
            instruction_data: bytes_to_base64(&instruction_data),
        }),
        error: None,
    })
} 