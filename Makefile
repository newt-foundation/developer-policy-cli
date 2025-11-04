.PHONY: upload-and-deploy-policy deploy-policy deploy-client deploy-client-factory upload-all-ipfs create-policy-cids-json upload-wasm-ipfs upload-wasm-args-ipfs upload-policy-ipfs upload-params-schema-ipfs help

help:
	@echo "Available Make Targets:"
	@echo "  upload-and-deploy-policy         - Upload all policy files in ./policy-files/ to Pinata IPFS and deploy the Policy contract"
	@echo "  deploy-policy                    - Deploy the policy given an existing policy_cids.json file"
	@echo "  deploy-client                    - Deploy the PolicyClient contract"
	@echo "  set-client-policy-params         - Set the policy parameters for a deployed PolicyClient"
	@echo "  deploy-client-factory            - Deploy a factory for deploying individual PolicyClients"
	@echo "  upload-all-ipfs                  - Upload all policy files in ./policy-files/ to Pinata IPFS"
	@echo "  create-policy-cids-json          - Upload all policy files in ./policy-files/ to Pinata IPFS and create policy_cids.json for deployment"
	@echo "  upload-wasm-ipfs                 - Upload policy.wasm to Pinata IPFS"
	@echo "  upload-wasm-args-ipfs            - Upload wasm_args.json to Pinata IPFS"
	@echo "  upload-policy-ipfs               - Upload policy.rego file to Pinata IPFS"
	@echo "  upload-params-schema-ipfs        - Upload params_schema.json to Pinata IPFS"
	@echo "  upload-policy-metadata-ipfs      - Upload policy_metadata.json to Pinata IPFS"
	@echo "  upload-policy-data-metadata-ipfs - Upload policy_data_metadata.json to Pinata IPFS"
	@echo "  upload-policy-data-metadata-ipfs - Upload policy_data_metadata.json to Pinata IPFS"
	@echo "  help                             - Show this help message"
	@echo ""
	@echo "See README.md for help on configuration"

# Upload WASM to IPFS via Pinata
upload-wasm-ipfs:
	@rm -f /tmp/pinata_wasm_upload.log
	@echo "================================================"
	@echo "============ Upload policy.wasm ================"
	@echo "================================================"
	@if [ ! -f policy-files/policy.wasm ]; then \
		echo "Error: policy.wasm file not found in policy-files directory"; \
		exit 1; \
	fi
	@echo "Uploading policy.wasm to Pinata IPFS..."
	@source .env && ~/.local/share/pinata/pinata upload policy-files/policy.wasm | tee /tmp/pinata_wasm_upload.log
	@echo ""
	@echo "=== IPFS Upload Results ==="
	@IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_wasm_upload.log | head -1); \
	if [ -n "$$IPFS_HASH" ]; then \
		echo "IPFS Hash: $$IPFS_HASH"; \
		echo "Getting gateway link..."; \
		GATEWAY_LINK=$$(~/.local/share/pinata/pinata gateways link "$$IPFS_HASH" 2>/dev/null || echo "https://gateway.pinata.cloud/ipfs/$$IPFS_HASH"); \
		echo "Direct IPFS Link: $$GATEWAY_LINK"; \
		echo "Public IPFS Link: https://ipfs.io/ipfs/$$IPFS_HASH"; \
	else \
		echo "Warning: Could not extract IPFS hash from upload output"; \
		cat /tmp/pinata_wasm_upload.log; \
	fi

# Upload wasm_args.json to IPFS via Pinata
upload-wasm-args-ipfs:
	@rm -f /tmp/pinata_args_upload.log
	@echo "================================================"
	@echo "========== Upload wasm_args.json ==========="
	@echo "================================================"
	@if [ ! -f policy-files/wasm_args.json ]; then \
		echo "Error: wasm_args.json file not found in policy-files directory"; \
		exit 1; \
	fi
	@echo "Uploading wasm_args.json to Pinata IPFS..."
	@source .env && ~/.local/share/pinata/pinata upload policy-files/wasm_args.json | tee /tmp/pinata_args_upload.log
	@echo ""
	@echo "=== IPFS Upload Results ==="
	@IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_args_upload.log | head -1); \
	if [ -n "$$IPFS_HASH" ]; then \
		echo "IPFS Hash: $$IPFS_HASH"; \
		echo "Getting gateway link..."; \
		GATEWAY_LINK=$$(~/.local/share/pinata/pinata gateways link "$$IPFS_HASH" 2>/dev/null || echo "https://gateway.pinata.cloud/ipfs/$$IPFS_HASH"); \
		echo "Direct IPFS Link: $$GATEWAY_LINK"; \
		echo "Public IPFS Link: https://ipfs.io/ipfs/$$IPFS_HASH"; \
	else \
		echo "Warning: Could not extract IPFS hash from upload output"; \
		cat /tmp/pinata_args_upload.log; \
	fi

# Upload policy.rego to IPFS via Pinata
upload-policy-ipfs:
	@rm -f /tmp/pinata_policy_upload.log
	@echo "================================================"
	@echo "============== Upload policy.rego =============="
	@echo "================================================"
	@if [ ! -f policy-files/policy.rego ]; then \
		echo "Error: policy.rego file not found in policy-files directory"; \
		exit 1; \
	fi
	@echo "Uploading policy.rego to Pinata IPFS..."
	@source .env && ~/.local/share/pinata/pinata upload policy-files/policy.rego | tee /tmp/pinata_policy_upload.log
	@echo ""
	@echo "=== IPFS Upload Results ==="
	@IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_policy_upload.log | head -1); \
	if [ -n "$$IPFS_HASH" ]; then \
		echo "IPFS Hash: $$IPFS_HASH"; \
		echo "Getting gateway link..."; \
		GATEWAY_LINK=$$(~/.local/share/pinata/pinata gateways link "$$IPFS_HASH" 2>/dev/null || echo "https://gateway.pinata.cloud/ipfs/$$IPFS_HASH"); \
		echo "Direct IPFS Link: $$GATEWAY_LINK"; \
		echo "Public IPFS Link: https://ipfs.io/ipfs/$$IPFS_HASH"; \
	else \
		echo "Warning: Could not extract IPFS hash from upload output"; \
		cat /tmp/pinata_policy_upload.log; \
	fi

# Upload params_schema.json to IPFS via Pinata
upload-params-schema-ipfs:
	@rm -f /tmp/pinata_schema_upload.log
	@echo "================================================"
	@echo "========== Upload params_schema.json ==========="
	@echo "================================================"
	@if [ ! -f policy-files/params_schema.json ]; then \
		echo "Error: params_schema.json file not found in policy-files directory"; \
		exit 1; \
	fi
	@echo "Uploading params_schema.json to Pinata IPFS..."
	@source .env && ~/.local/share/pinata/pinata upload policy-files/params_schema.json | tee /tmp/pinata_schema_upload.log
	@echo ""
	@echo "=== IPFS Upload Results ==="
	@IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_schema_upload.log | head -1); \
	if [ -n "$$IPFS_HASH" ]; then \
		echo "IPFS Hash: $$IPFS_HASH"; \
		echo "Getting gateway link..."; \
		GATEWAY_LINK=$$(~/.local/share/pinata/pinata gateways link "$$IPFS_HASH" 2>/dev/null || echo "https://gateway.pinata.cloud/ipfs/$$IPFS_HASH"); \
		echo "Direct IPFS Link: $$GATEWAY_LINK"; \
		echo "Public IPFS Link: https://ipfs.io/ipfs/$$IPFS_HASH"; \
	else \
		echo "Warning: Could not extract IPFS hash from upload output"; \
		cat /tmp/pinata_schema_upload.log; \
	fi

# Upload policy_metadata.json to IPFS via Pinata
upload-policy-metadata-ipfs:
	@rm -f /tmp/pinata_metadata_upload.log
	@echo "================================================"
	@echo "========== Upload policy_metadata.json ========="
	@echo "================================================"
	@if [ ! -f policy-files/policy_metadata.json ]; then \
		echo "Error: policy_metadata.json file not found in policy-files directory"; \
		exit 1; \
	fi
	@echo "Uploading policy_metadata.json to Pinata IPFS..."
	@source .env && ~/.local/share/pinata/pinata upload policy-files/policy_metadata.json | tee /tmp/pinata_metadata_upload.log
	@echo ""
	@echo "=== IPFS Upload Results ==="
	@IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_metadata_upload.log | head -1); \
	if [ -n "$$IPFS_HASH" ]; then \
		echo "IPFS Hash: $$IPFS_HASH"; \
		echo "Getting gateway link..."; \
		GATEWAY_LINK=$$(~/.local/share/pinata/pinata gateways link "$$IPFS_HASH" 2>/dev/null || echo "https://gateway.pinata.cloud/ipfs/$$IPFS_HASH"); \
		echo "Direct IPFS Link: $$GATEWAY_LINK"; \
		echo "Public IPFS Link: https://ipfs.io/ipfs/$$IPFS_HASH"; \
	else \
		echo "Warning: Could not extract IPFS hash from upload output"; \
		cat /tmp/pinata_metadata_upload.log; \
	fi

# Upload policy_data_metadata.json to IPFS via Pinata
upload-policy-data-metadata-ipfs:
	@rm -f /tmp/pinata_data_metadata_upload.log
	@echo "================================================"
	@echo "======== Upload policy_data_metadata.json ======"
	@echo "================================================"
	@if [ ! -f policy-files/policy_data_metadata.json ]; then \
		echo "Error: policy_data_metadata.json file not found in policy-files directory"; \
		exit 1; \
	fi
	@echo "Uploading policy_data_metadata.json to Pinata IPFS..."
	@source .env && ~/.local/share/pinata/pinata upload policy-files/policy_data_metadata.json | tee /tmp/pinata_data_metadata_upload.log
	@echo ""
	@echo "=== IPFS Upload Results ==="
	@IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_data_metadata_upload.log | head -1); \
	if [ -n "$$IPFS_HASH" ]; then \
		echo "IPFS Hash: $$IPFS_HASH"; \
		echo "Getting gateway link..."; \
		GATEWAY_LINK=$$(~/.local/share/pinata/pinata gateways link "$$IPFS_HASH" 2>/dev/null || echo "https://gateway.pinata.cloud/ipfs/$$IPFS_HASH"); \
		echo "Direct IPFS Link: $$GATEWAY_LINK"; \
		echo "Public IPFS Link: https://ipfs.io/ipfs/$$IPFS_HASH"; \
	else \
		echo "Warning: Could not extract IPFS hash from upload output"; \
		cat /tmp/pinata_data_metadata_upload.log; \
	fi

upload-all-ipfs: upload-wasm-ipfs upload-wasm-args-ipfs upload-policy-ipfs upload-params-schema-ipfs upload-policy-metadata-ipfs upload-policy-data-metadata-ipfs
	@echo "================================================================================"
	@echo ""

ENTRYPOINT ?= $(shell read -p "Input rego policy entrypoint (i.e. my_policy_name.allow): " entrypoint; echo $$entrypoint)

create-policy-cids-json: upload-all-ipfs
	@rm -f policy-files/policy_cids.json
	@touch policy-files/policy_cids.json
	@source .env; \
	WASM_IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_wasm_upload.log | head -1); \
	WASM_ARGS_IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_args_upload.log | head -1); \
	POLICY_IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_policy_upload.log | head -1); \
	SCHEMA_IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_schema_upload.log | head -1); \
	METADATA_IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_metadata_upload.log | head -1); \
	DATA_METADATA_IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_data_metadata_upload.log | head -1); \
	echo "{\"wasmCid\": \"$$WASM_IPFS_HASH\",\"wasmArgs\": \"$$WASM_ARGS_IPFS_HASH\",\"policyCid\": \"$$POLICY_IPFS_HASH\",\"schemaCid\": \"$$SCHEMA_IPFS_HASH\",\"attester\": \"0x4883282094755C01cd0d15dFE74753c9E189d194\",\"entrypoint\": \"$(ENTRYPOINT)\",\"policyDataMetadataCid\": \"$$DATA_METADATA_IPFS_HASH\",\"policyMetadataCid\": \"$$METADATA_IPFS_HASH\"}" >> policy-files/policy_cids.json

CHAIN_ID ?= $(shell read -p "Confirm Chain ID (e.g. mainnet = 1, sepolia = 11155111): " chainid; echo $$chainid)

deploy-policy:
	@source .env; \
	export TEMP_CHAIN_ID=$(CHAIN_ID); \
	if [ $$(cast chain-id -r $$RPC_URL) != $$TEMP_CHAIN_ID ]; then \
		echo "Error: Chain ID does not match RPC_URL"; \
		exit 1; \
	fi; \
	if [ $$TEMP_CHAIN_ID != 1 ] && [ $$TEMP_CHAIN_ID != 11155111 ]; then \
		echo "Error: Chain ID does not match any existing deployment"; \
		exit 1; \
	fi; \
	if [ ! -f policy-files/policy_cids.json ]; then \
		echo "Error: generate or fill out policy_cids.json file first"; \
		exit 1; \
	fi; \
	DIRECTORY=$$(pwd); \
	cd lib/newton-contracts; \
	PRIVATE_KEY=$$PRIVATE_KEY ETHERSCAN_API_KEY=$$ETHERSCAN_API_KEY POLICY_CIDS_PATH="$$DIRECTORY/policy-files/policy_cids.json" DEPLOYMENT_ENV=$$DEPLOYMENT_ENV forge script script/PolicyDeployer.s.sol --rpc-url $$RPC_URL --private-key $$PRIVATE_KEY --broadcast

upload-and-deploy-policy: create-policy-cids-json deploy-policy

deploy-client-factory: 
	@source .env; \
	export TEMP_CHAIN_ID=$(CHAIN_ID); \
	if [ $$(cast chain-id -r $$RPC_URL) != $$TEMP_CHAIN_ID ]; then \
		echo "Error: Chain ID does not match RPC_URL"; \
		exit 1; \
	fi; \
	DEPLOYMENT_ENV=$$DEPLOYMENT_ENV forge script script/DeployClientFactory.s.sol:ClientFactoryDeployer --rpc-url $$RPC_URL --private-key $$PRIVATE_KEY --broadcast

POLICY ?= $(shell read -p "Input Policy address: " policy; echo $$policy)

deploy-client: 
	@source .env; \
	export TEMP_CHAIN_ID=$(CHAIN_ID); \
	if [ $$(cast chain-id -r $$RPC_URL) != $$TEMP_CHAIN_ID ]; then \
		echo "Error: Chain ID does not match RPC_URL"; \
		exit 1; \
	fi; \
	POLICY=$(POLICY) DEPLOYMENT_ENV=$$DEPLOYMENT_ENV forge script script/DeployPolicyClient.s.sol:ClientDeployer --rpc-url $$RPC_URL --private-key $$PRIVATE_KEY --broadcast

POLICY_CLIENT ?= $(shell read -p "Input Your Policy Client address: " policy_client; echo $$policy_client)
POLICY_PARAMS ?= $(shell read -p "Input Policy params JSON string: " params; echo $$params)
EXPIRE_AFTER ?= $(shell read -p "Input expireAfter (uint): " expire; echo $$expire)

set-client-policy-params: 
	@source .env; \
	export TEMP_CHAIN_ID=$(CHAIN_ID); \
	if [ $$(cast chain-id -r $$RPC_URL) != $$TEMP_CHAIN_ID ]; then \
		echo "Error: Chain ID does not match RPC_URL"; \
		exit 1; \
	fi; \
	POLICY_CLIENT=$(POLICY_CLIENT) POLICY_PARAMS=$(POLICY_PARAMS) EXPIRE_AFTER=$(EXPIRE_AFTER) DEPLOYMENT_ENV=$$DEPLOYMENT_ENV forge script script/SetPolicyClientParams.s.sol:ClientParamsSetter --rpc-url $$RPC_URL --private-key $$PRIVATE_KEY --broadcast