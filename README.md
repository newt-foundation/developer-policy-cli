# Newton Policy Deployment CLI

This project includes automated functionality to upload Policy files to Pinata IPFS and deploy the Policy contract, configured for your policy rego code.

## Setup

### 1. Install dependencies

First, install the git submodules

```bash
git submodule init && git submodule update --remote
```

Note: this may cause git to mark the submodule as changed. This is due to updates to the contract code and deployments as it is a living repo and is a desired result.

Next, install forge

```bash
curl -L https://foundry.paradigm.xyz | bash
```

Then you can use the command shown in the output of the install (`source ~/.zshenv` or whatever your shell environment is) or just open a new terminal window and run:

```bash
foundryup
```

which will finish installing forge.

Additionally install pinata cli, the ipfs upload manager (more on this in the next section).

```bash
curl -fsSL https://cli.pinata.cloud/install | bash
```

Finally, create the directory for uploading the policy files (make sure to put it in the root of this repo and use this directory name!)

```bash
mkdir policy-files
```

### 2. Get Your Pinata Credentials

1. Go to [Pinata](https://app.pinata.cloud/developers/api-keys)
2. Create a new API key. If you don't just make an admin key, make sure the API key has write permissions for files and read permission for gateways.
3. Copy the API Key, API Secret, and JWT
4. Login in your terminal via `pinata auth`
5. Note your gateway domain (e.g., `your-subdomain.mypinata.cloud`), this is in a different tab than API keys

### 3. Configure Environment Variables

Create a file `.env` by copying and renaming `.env.example`

```bash
cp .env.example .env
```

Update your `.env` file with your Pinata credentials:

```env
# Pinata IPFS Configuration
PINATA_API_KEY=your_pinata_api_key_here
PINATA_API_SECRET=your_pinata_api_secret_here
PINATA_JWT=your_pinata_jwt_here
PINATA_GATEWAY=your_pinata_gateway_domain_here
```

Additionally fill in the values for deploying the contract. Note that the private key needs to correspond to an account with some ether for deploying the policy contract. Additionally, the `RPC_URL` variable is chain dependent. Make sure it applies to the deployment you're making (either to sepolia testnet or mainnet).

```env
# Forge config parameters
PRIVATE_KEY=your_funded_deployer_address_private_key
RPC_URL=make_sure_this_matches_the_chain_you're_deploying_to
```

### 4. Provide Policy Files

Put your policy files in the `policy-files` folder. You can look in `policy-files-examples` for some example files to start you off or read the following quickstart guide. See [Policy Files](#policy-files-overview) for an explanation of what each file is for.

## Quickstart Guide

This guide deploys an example policy contract for testing and sandbox. Make sure to deploy this contract to sepolia testnet.

### Setup

In addition to the previous setup steps, copy the contents of the `policy-files-examples` folder into `policy-files`. Don't worry about filling out the, `policy_cids.json` file as it will be automatically filled.

### Walkthrough

Run the command:

```bash
make upload-and-deploy-policy
```

After the script uploads the files to your pinata cloud, it will prompt you for some inputs:

```
Input policy data args (put {} if unused): 
```
For this prompt, just press enter. The example WASM uses the default value of no inputs.

```
Input rego policy entrypoint (i.e. my_policy_name.allow): 
```
For this prompt, enter `newton_trading_agent.allow` and press enter. This corresponds to the example policy.rego file that is provided.

```
Confirm Chain ID (e.g. mainnet = 1, sepolia = 11155111):
```

For this prompt, input 11155111 if you are testing to deploy to sepolia. If this errors, check that your `RPC_URL` parameter in your `.env` file is an ethereum sepolia url (should say so in the url).

```
Input policy approval expiration time in seconds (default 1 hour, good for debugging):
```

For this prompt, you can just press enter and use the default value of 1 hour.

After these inputs, the script will deploy the contract. If successful you should be greeted with an output that looks like:

```
[⠊] Compiling...
No files changed, compilation skipped
Script ran successfully.

== Logs ==
  Policy: 0x9D8BB6B9E069B0a6594a5332356C64DD82c328F5
  Policy Implementation: 0x33451982CdDe3ED2ED7bc9fBC6A7db00132D7D09

## Setting up 1 EVM.

==========================

Chain 11155111

Estimated gas price: 0.001000034 gwei

Estimated total gas used for script: 2425840

Estimated amount required: 0.00000242592247856 ETH

==========================

##### sepolia
✅  [Success] Hash: 0x154ef7d7293b0e37a5d5f2519f73aefbd64a6f2ccbce69343233eede6a73bb59
Block: 9264857
Paid: 0.000000765488013075 ETH (765475 gas * 0.001000017 gwei)


##### sepolia
✅  [Success] Hash: 0xf2036e58cefac7d23c9a719dc8f33bb4a00b2c6cd2948f939ce6e2763d68dade
Block: 9264857
Paid: 0.000000861098638428 ETH (861084 gas * 0.001000017 gwei)


##### sepolia
✅  [Success] Hash: 0x8d8bed135b540787ea445c7a12a5a03de65f7423d73d765410001d0def7cdce3
Block: 9264857
Paid: 0.00000008468143956 ETH (84680 gas * 0.001000017 gwei)

✅ Sequence #1 on sepolia | Total Paid: 0.000001711268091063 ETH (1711239 gas * avg 0.001000017 gwei)
                                                                                                                                                    

==========================

ONCHAIN EXECUTION COMPLETE & SUCCESSFUL.

Transactions saved to: /Users/albertbrown/Documents/developer-policy-cli/newton-contracts/broadcast/PolicyDeployer.s.sol/11155111/run-latest.json

Sensitive values saved to: /Users/albertbrown/Documents/developer-policy-cli/newton-contracts/cache/PolicyDeployer.s.sol/11155111/run-latest.json
```
From this output, copy the values `Policy: 0x9D8BB6B9E069B0a6594a5332356C64DD82c328F5` (will show your contract address instead of the example), NOT the address listed as `Policy Implementation:`. Congratulations! This is your deployed policy contact. Refer back to the Newton Integration Guide for how to use it.

## Usage

### Deploy your Policy contract

If everything in `policy-files` is good to go, you can deploy your policy contract using the following command.

```bash
make upload-and-deploy-policy
```

It will ask you for some additional inputs including:
- the args for your policy data WASM: this value is if your WASM requires any case by case input.
- the entrypoint: this is the part of your rego code that allows for successful execution of a task
- the expiry: this is how long after approval your task remains valid
- the deployment chainid: this is already set in your RPC_URL env variable, but is asked here to prevent accidental deploys to the wrong chain. NOTE: policies deployed to mainnet will not be useable until they are whitelisted.

### Additional Commands

#### Upload individual Policy Files to IPFS

You can upload your files individually to IPFS via pinata without deploying the contract. If you do, make sure to note down the IPFS hash generated as it will be necessary for manually creating your `policy_cids.json` file.

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
make create-policy-cids-json
```

Uploads all files and creates the `policy_cids.json` file for contract deployment.

####  Standalone Policy Deploy

```bash
make deploy-policy
```

If you have already uploaded all your files to IPFS and just want to deploy the Policy contract, you can use this command given you format the `policy_cids.json` file correctly. Use the template in `policy-files-examples` for correct formatting and explanation of the properties. Do not change the `attester` property as it is a system address.

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
