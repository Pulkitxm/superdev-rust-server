use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as JsonResponse,
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    signer::SignerError,
    system_instruction,
    transaction::Transaction,
};
use spl_token::{
    instruction as token_instruction,
    state::{Mint, Account},
};
use std::str::FromStr;
use tower_http::cors::CorsLayer;
use tracing::{info, error};

mod handlers;
mod models;
mod utils;

use handlers::*;
use models::*;
use utils::*;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting Solana HTTP server...");

    // Configure CORS
    let cors = CorsLayer::permissive();

    // Build our application with a route
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token))
        .layer(cors);

    // Run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Server listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
