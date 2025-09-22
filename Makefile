.PHONY: build-agent build-wasm build-all run-agent run-wasm run-wasm-sample upload-wasm-ipfs upload-policy-ipfs upload-policy-params-ipfs upload-params-schema-ipfs clean help

build-wasm:
	cargo build -p newton-trade-agent-wasm --target wasm32-wasip2 --release

help:
	@echo "Available Make Targets:"
	@echo "  build-wasm         - Build the WASM market analysis component"
	@echo "  upload-wasm-ipfs   - Build WASM (release) and upload to Pinata IPFS"
	@echo "  upload-policy-ipfs - Upload policy.rego file to Pinata IPFS"
	@echo "  upload-policy-params-ipfs - Upload policy_params.json to Pinata IPFS"
	@echo "  upload-params-schema-ipfs - Upload params_schema.json to Pinata IPFS"
	@echo "  upload-policy-metadata-ipfs - Upload policy_metadata.json to Pinata IPFS"
	@echo "  upload-policy-data-metadata-ipfs - Upload policy_data_metadata.json to Pinata IPFS"
	@echo "  help               - Show this help message"
	@echo ""
	@echo "Examples:"
	@echo "  make upload-wasm-ipfs         # Build and upload WASM to IPFS"
	@echo "  make upload-policy-ipfs       # Upload policy.rego to IPFS"
	@echo "  make upload-policy-params-ipfs # Upload policy_params.json to IPFS"
	@echo "  make upload-params-schema-ipfs # Upload params_schema.json to IPFS"
	@echo "  make upload-policy-metadata-ipfs # Upload policy_metadata.json to IPFS"
	@echo "  make upload-policy-data-metadata-ipfs # Upload policy_data_metadata.json to IPFS"
	@echo ""
	@echo "For IPFS upload setup and troubleshooting, see: IPFS_UPLOAD.md"

run-wasm: build-wasm
	cargo build -p op-sim --release
	./target/release/op-sim ./target/wasm32-wasip2/release/main.wasm {}

run-wasm-dev: build-wasm
	cargo build -p op-sim --release
	./target/release/op-sim ./target/wasm32-wasip2/release/main.wasm "development"

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

upload-all-ipfs: upload-wasm-ipfs upload-policy-ipfs upload-policy-params-ipfs upload-params-schema-ipfs upload-policy-metadata-ipfs upload-policy-data-metadata-ipfs
	@echo "================================================================================"
	@echo ""

ATTESTER ?= $(shell read -p "Input attester address: " attester; echo $$attester)
ENTRYPOINT ?= $(shell read -p "Input rego policy entrypoint (i.e. my_policy_name.allow): " entrypoint; echo $$entrypoint)

create-policy-uris-json: upload-all-ipfs
	@rm -f policy-files/policy_uris.json
	@touch policy-files/policy_uris.json
	@WASM_IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_wasm_upload.log | head -1); \
	POLICY_IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_policy_upload.log | head -1); \
	SCHEMA_IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_schema_upload.log | head -1); \
	METADATA_IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_metadata_upload.log | head -1); \
	DATA_METADATA_IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_data_metadata_upload.log | head -1); \
	echo "{\"policyDataLocation\": \"$$WASM_IPFS_HASH\",\"policyDataArgs\": \"\",\"policyUri\": \"$$POLICY_IPFS_HASH\",\"schemaUri\": \"$$SCHEMA_IPFS_HASH\",\"attester\": \"$(ATTESTER)\",\"entrypoint\": \"$(ENTRYPOINT)\",\"policyDataMetadataUri\": \"$$DATA_METADATA_IPFS_HASH\",\"policyMetadataUri\": \"$$METADATA_IPFS_HASH\"}" >> policy-files/policy_uris.json

CHAIN_ID ?= $(shell read -p "Confirm Chain ID (e.g. mainnet = 1, sepolia = 11155111): " chainid; echo $$chainid)

deploy-policy:
	@source .env; \
	if [ $$(cast chain-id -r $$RPC_URL) != $(CHAIN_ID) ]; then \
		echo "Error: Chain ID does not match RPC_URL"; \
		exit 1; \
	fi
	@if [ ! -f policy-files/policy_uris.json ]; then \
		echo "Error: generate or fill out policy_uris.json file first"; \
		exit 1; \
	fi
	@source .env; \
	DIRECTORY=$$(pwd); \
	cd newton-contracts; \
	PRIVATE_KEY=$$PRIVATE_KEY ETHERSCAN_API_KEY=$$ETHERSCAN_API_KEY POLICY_URIS_PATH="$$DIRECTORY/policy-files/policy_uris.json" forge script script/PolicyDeployer.s.sol --rpc-url $$RPC_URL --private-key $$PRIVATE_KEY --broadcast --slow
#remove the --slow later

upload-and-deploy-policy: create-policy-uris-json deploy-policy