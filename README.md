# Newton Policy Deployment CLI

This project includes automated functionality to upload Policy files to Pinata IPFS and deploy the Policy contract, configured for your policy rego code.

## Setup

### 1. Install dependencies

install the git submodules

```bash
git submodule init && git submodule update --remote
```

create the directory for your policy

```bash
mkdir policy-files
```

install pinata cli, the ipfs upload manager

```bash
curl -fsSL https://cli.pinata.cloud/install | bash
```

### 2. Get Your Pinata Credentials

1. Go to [Pinata](https://app.pinata.cloud/developers/api-keys)
2. Create a new API key. If you don't just make an admin key, make sure the API key has write permissions for files and read permission for gateways.
3. Copy the API Key, API Secret, and JWT
4. Login in your terminal via `pinata auth`
5. Note your gateway domain (e.g., `your-subdomain.mypinata.cloud`), this is in a different tab than API keys

### 3. Configure Environment Variables

Create a file `.env` by copying and renaming `.env.example`

Update your `.env` file with your Pinata credentials:

```env
# Pinata IPFS Configuration
PINATA_API_KEY=your_pinata_api_key_here
PINATA_API_SECRET=your_pinata_api_secret_here
PINATA_JWT=your_pinata_jwt_here
PINATA_GATEWAY=your_pinata_gateway_domain_here
```

Additionally fill in the values for deploying the contract
```env
# Forge config parameters
PRIVATE_KEY=your_funded_deployer_address_private_key
RPC_URL=make_sure_this_matches_the_chain_you're_deploying_to
```

### 4. Provide Policy Files

Put your policy files in the `policy-files` folder. You can look in `policy-files-examples` for some example files to start you off. See [Policy Files](#policy-files-overview) for an explanation of what each file is for.

## Usage

### Deploy your Policy contract

If everything in `policy-files` is good to go, you can deploy your policy contract using the following command.

```bash
make upload-and-deploy-policy
```

It will ask you for some additional inputs including:
- the args for your policy data WASM: this value is if your WASM requires any case by case input.
- the attester address: this is your EoA for attesting correct policy data
- the entrypoint: this is the part of your rego code that allows for successful execution of a task
- the expiry: this is how long after approval your task remains valid
- the deployment chainid: this is already set in your RPC_URL env variable, but is asked here to prevent accidental deploys to the wrong chain. NOTE: policies deployed to mainnet will not be useable until they are whitelisted.

### Additional Commands

#### Upload individual Policy Files to IPFS

You can upload your files individually to IPFS via pinata without deploying the contract. If you do, make sure to note down the IPFS hash generated as it will be necessary for manually creating your `policy_uris.json` file.

```bash
make upload-wasm-ipfs
```

Uploads the wasm file that sources data for the policy.


```bash
make upload-policy-ipfs
```

Uploads the Rego policy file that defines trading rules and restrictions.


```bash
make upload-policy-params-ipfs
```

Uploads the policy parameters configuration file with chain-specific contract allowlists.


```bash
make upload-params-schema-ipfs
```

Uploads the schema configuration file which defines inputs.


```bash
make upload-policy-metadata-ipfs
```

Uploads the metadata associated with the policy.


```bash
make upload-policy-data-metadata-ipfs
```

Uploads the metadata associated with the policy data source.


```bash
make upload-all-ipfs
```

Uploads all necessary files to IPFS


```bash
make create-policy-uris-json
```

Uploads all files and creates the `policy_uris.json` file for contract deployment.

####  Standalone Policy Deploy

```bash
make deploy-policy
```

If you have already uploaded all your files to IPFS and just want to deploy the Policy contract, you can use this command given you format the `policy_uris.json` file correctly. Use the template in `policy-files-examples` for correct formatting and explanation of the properties.

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

### policy.wasm
not sure what to put here still
- **Purpose**: Defines the data source for the policy
- **Content**: Compiled wasm code
- **MIME Type**: `application/wasm`


### policy.rego
- **Purpose**: Defines the trading policy rules in Rego language
- **Content**: Authorization logic, token whitelists, trading limits, market conditions
- **MIME Type**: `text/plain; charset=UTF-8`

### policy_params.json
- **Purpose**: Configuration parameters for different blockchain networks and contracts
- **Content**: Chain-specific contract allowlists, trading limits, slippage settings
- **MIME Type**: `application/json`

### params_schema.json
- **Purpose**: JSON Schema validation for policy_params.json structure
- **Content**: Schema definitions, validation rules, parameter descriptions
- **MIME Type**: `application/json`

### policy_metadata.json
- **Purpose**: JSON description and attributation for the policy rego code
- **Content**: Name, version, author, link, and description
- **MIME Type**: `application/json`

### policy_data_metadata.json
- **Purpose**: JSON description and attributation for the policy data source(?) wasm
- **Content**: Name, version, author, link, and description
- **MIME Type**: `application/json`

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
