# Automated WASM Upload to Pinata IPFS

This project includes automated functionality to build and upload the WASM component to Pinata IPFS.

## Features

- **Automatic WASM Build**: Builds the WASM component as a release build
- **Automated Authentication**: Handles Pinata CLI authentication without manual input
- **Dynamic Gateway**: Uses your configured Pinata gateway instead of hardcoded URLs
- **Gateway Auto-Selection**: Automatically sets your preferred gateway
- **Multiple Link Types**: Provides both your personalized gateway and public IPFS links
- **Error Handling**: Graceful fallback and informative error messages

## Setup

### 1. Install Pinata CLI

```bash
curl -fsSL https://cli.pinata.cloud/install | bash
```

### 2. Get Your Pinata Credentials

1. Go to [Pinata](https://app.pinata.cloud/developers/api-keys)
2. Create a new API key. If you don't just make an admin key, make sure the API key has write permissions for files and read permission for gateways.
3. Copy the API Key, API Secret, and JWT
4. Note your gateway domain (e.g., `your-subdomain.mypinata.cloud`)
5. Login in your terminal via `pinata auth`

### 3. Configure Environment Variables

Update your `.env` file with your Pinata credentials:

```env
# Pinata IPFS Configuration
PINATA_API_KEY=your_pinata_api_key_here
PINATA_API_SECRET=your_pinata_api_secret_here
PINATA_JWT=your_pinata_jwt_here
PINATA_GATEWAY=your_pinata_gateway_domain_here
```

**Important**: Replace the redacted JWT (asterisks) with your actual JWT from Pinata.

## Usage

### Upload WASM to IPFS

```bash
make upload-wasm-ipfs
```

This command will:
1. Build the WASM component as a release build
2. Upload the WASM file to IPFS
3. Print the IPFS hash and direct links

### Upload Policy Files to IPFS

#### Upload policy.rego

```bash
make upload-policy-ipfs
```

Uploads the Rego policy file that defines trading rules and restrictions.

#### Upload policy_params.json

```bash
make upload-policy-params-ipfs
```

Uploads the policy parameters configuration file with chain-specific contract allowlists.

#### Upload params_schema.json

```bash
make upload-params-schema-ipfs
```

Uploads the JSON schema file that validates the policy parameters structure.

All policy upload commands:
1. Check if the target file exists in the current directory
2. Automatically authenticate with Pinata (if not already authenticated)
3. Set your preferred gateway as default
4. Upload the file to IPFS with timestamped naming
5. Print the IPFS hash and direct links

### Example Output

#### WASM Upload

```bash
Building WASM component...
    Finished `release` profile [optimized] target(s) in 0.17s
Building WASM component for IPFS upload...
Using WASM file: target/wasm32-wasip1/release/main.wasm
Setting up Pinata configuration...
Authenticating with Pinata and setting gateway...
Pinata configuration complete.
Uploading WASM to Pinata IPFS...
{
    "id": "01991320-60a2-7542-9519-f14ecef1459c",
    "name": "main.wasm",
    "cid": "bafybeig3vgrf3srvaegy627mz3tflsxnc5bk66moenbfxalfp324465mm4",
    "size": 291387,
    "created_at": "2025-09-04T05:08:34.221Z",
    "mime_type": "application/wasm"
}

=== IPFS Upload Results ===
IPFS Hash: bafybeig3vgrf3srvaegy627mz3tflsxnc5bk66moenbfxalfp324465mm4
Getting gateway link...
Direct IPFS Link: https://silver-socialist-eel-341.mypinata.cloud/ipfs/bafybeig3vgrf3srvaegy627mz3tflsxnc5bk66moenbfxalfp324465mm4
Public IPFS Link: https://ipfs.io/ipfs/bafybeig3vgrf3srvaegy627mz3tflsxnc5bk66moenbfxalfp324465mm4
```

#### Policy File Upload

```bash
$ make upload-policy-ipfs
Uploading policy.rego to Pinata IPFS...
Setting up Pinata configuration...
Authenticating with Pinata and setting gateway...
Pinata configuration complete.
Uploading policy.rego to Pinata IPFS...
{
    "id": "01991361-6fe8-72ed-916d-2f7ee4502667",
    "name": "policy.rego",
    "cid": "bafkreifvwxnaml4fwhwvd6gdq7qqlkw6zsjqvdfm2n2pdkhug3jwlqiv3u",
    "size": 2250,
    "created_at": "2025-09-04T06:19:37.423Z",
    "mime_type": "text/plain; charset=UTF-8"
}

=== IPFS Upload Results ===
IPFS Hash: bafkreifvwxnaml4fwhwvd6gdq7qqlkw6zsjqvdfm2n2pdkhug3jwlqiv3u
Direct IPFS Link: https://silver-socialist-eel-341.mypinata.cloud/ipfs/bafkreifvwxnaml4fwhwvd6gdq7qqlkw6zsjqvdfm2n2pdkhug3jwlqiv3u
Public IPFS Link: https://ipfs.io/ipfs/bafkreifvwxnaml4fwhwvd6gdq7qqlkw6zsjqvdfm2n2pdkhug3jwlqiv3u
```

## Troubleshooting

### JWT is Redacted Error

If you see:
```
⚠️  PINATA_JWT is redacted in .env file. Please replace with actual JWT.
```

This means you need to replace the asterisks in your `.env` file with your real JWT from Pinata.

### Authentication Fails

1. Verify your JWT is correct and not expired
2. Check that your API key has proper permissions
3. Try manually authenticating: `~/.local/share/pinata/pinata auth`

### Upload Errors (401, 403)

- **401 Unauthorized**: Authentication issue, check your JWT and API key permissions
- **403 Forbidden**: Permission issue, verify your API key permissions

## Policy Files Overview

This project includes several policy-related files that can be uploaded to IPFS:

### policy.rego
- **Purpose**: Defines the trading policy rules in Rego language
- **Content**: Authorization logic, token whitelists, trading limits, market conditions
- **MIME Type**: `text/plain; charset=UTF-8`
- **Size**: ~2.3KB

### policy_params.json
- **Purpose**: Configuration parameters for different blockchain networks and contracts
- **Content**: Chain-specific contract allowlists, trading limits, slippage settings
- **MIME Type**: `application/json`
- **Size**: ~1.8KB

### params_schema.json
- **Purpose**: JSON Schema validation for policy_params.json structure
- **Content**: Schema definitions, validation rules, parameter descriptions
- **MIME Type**: `application/json`
- **Size**: ~3.3KB

## Technical Details

### Files Created

- `pinata-auth.expect`: Automated authentication script (excluded from git)
- Temporary files: `/tmp/pinata_upload.log`, `/tmp/wasm_file_path` (auto-cleaned)

### WASM File Detection

The system automatically finds the WASM file in these locations:
1. `target/wasm32-wasip2/release/main.wasm`
2. `target/wasm32-wasip1/release/main.wasm`
3. `target/wasm32-wasip1/release/newton-trade-agent-wasm.wasm`

### Gateway Selection

- Uses `pinata gateways link <hash>` to get your personalized gateway
- Falls back to `https://gateway.pinata.cloud/ipfs/<hash>` if gateway command fails
- Also provides public IPFS gateway link: `https://ipfs.io/ipfs/<hash>`

## Security Notes

- The `.env` file contains sensitive credentials and is excluded from git
- The `pinata-auth.expect` script is also excluded from git as it may contain credentials
- JWT tokens should be kept secure and rotated regularly
