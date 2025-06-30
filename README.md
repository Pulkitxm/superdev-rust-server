# Solana HTTP Server

A Rust-based HTTP server that provides Solana-related endpoints for keypair generation, SPL token operations, message signing/verification, and transaction instruction creation.

## Features

- 🔑 Generate Solana keypairs
- 🪙 Create and mint SPL tokens
- ✍️ Sign and verify messages using Ed25519
- 💰 Create SOL transfer instructions
- 🔄 Create SPL token transfer instructions
- 🛡️ Comprehensive input validation
- 🔒 Secure cryptographic operations

## Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd solana-assignment
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`

## API Endpoints

### 1. Generate Keypair
**POST** `/keypair`

Generates a new Solana keypair.

**Response:**
```json
{
  "success": true,
  "data": {
    "pubkey": "base58-encoded-public-key",
    "secret": "base58-encoded-secret-key"
  }
}
```

### 2. Create Token
**POST** `/token/create`

Creates a new SPL token initialize mint instruction.

**Request:**
```json
{
  "mintAuthority": "base58-encoded-public-key",
  "mint": "base58-encoded-public-key",
  "decimals": 6
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "program_id": "string",
    "accounts": [
      {
        "pubkey": "pubkey",
        "is_signer": false,
        "is_writable": true
      }
    ],
    "instruction_data": "base64-encoded-data"
  }
}
```

### 3. Mint Token
**POST** `/token/mint`

Creates a mint-to instruction for SPL tokens.

**Request:**
```json
{
  "mint": "mint-address",
  "destination": "destination-user-address",
  "authority": "authority-address",
  "amount": 1000000
}
```

### 4. Sign Message
**POST** `/message/sign`

Signs a message using a private key.

**Request:**
```json
{
  "message": "Hello, Solana!",
  "secret": "base58-encoded-secret-key"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "signature": "base64-encoded-signature",
    "public_key": "base58-encoded-public-key",
    "message": "Hello, Solana!"
  }
}
```

### 5. Verify Message
**POST** `/message/verify`

Verifies a signed message.

**Request:**
```json
{
  "message": "Hello, Solana!",
  "signature": "base64-encoded-signature",
  "pubkey": "base58-encoded-public-key"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "valid": true,
    "message": "Hello, Solana!",
    "pubkey": "base58-encoded-public-key"
  }
}
```

### 6. Send SOL
**POST** `/send/sol`

Creates a SOL transfer instruction.

**Request:**
```json
{
  "from": "sender-address",
  "to": "recipient-address",
  "lamports": 100000
}
```

### 7. Send Token
**POST** `/send/token`

Creates an SPL token transfer instruction.

**Request:**
```json
{
  "destination": "destination-user-address",
  "mint": "mint-address",
  "owner": "owner-address",
  "amount": 100000
}
```

## Error Handling

All endpoints return consistent error responses:

```json
{
  "success": false,
  "error": "Description of error"
}
```

Common error scenarios:
- Invalid Solana addresses (must be valid base58)
- Invalid secret keys
- Missing required fields
- Invalid signature formats
- Validation errors

## Security Considerations

- No private keys are stored on the server
- All cryptographic operations use standard Solana libraries
- Input validation prevents malicious inputs
- CORS is configured for cross-origin requests
- Proper error handling to avoid information leakage

## Development

### Project Structure

```
src/
├── main.rs          # Server entry point and routing
├── models.rs        # Request/response data structures
├── handlers.rs      # Endpoint handler implementations
└── utils.rs         # Utility functions and helpers
```

### Building for Production

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

## Dependencies

- **axum**: Web framework
- **solana-sdk**: Solana blockchain SDK
- **spl-token**: SPL token program
- **serde**: Serialization/deserialization
- **validator**: Input validation
- **bs58**: Base58 encoding
- **base64**: Base64 encoding

## License

This project is licensed under the MIT License. 