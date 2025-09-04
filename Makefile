.PHONY: build-agent build-wasm build-all run-agent run-wasm run-wasm-sample upload-wasm-ipfs upload-policy-ipfs upload-policy-params-ipfs upload-params-schema-ipfs clean help

build-agent:
	cargo build -p trade-agent --release

build-wasm:
	@echo "Building WASM component..."
	@RUSTFLAGS="" cargo component build -p newton-trade-agent-wasm --release || \
		(echo "Falling back to standard cargo build..." && \
		 RUSTFLAGS="" cargo build -p newton-trade-agent-wasm --target wasm32-wasip1 --release)

build-all: build-agent build-wasm

run-wasm: build-wasm
	wasmtime run -S http target/wasm32-wasip2/release/main.wasm

agent-help:
	./target/release/trade-agent --help

help:
	@echo "Available Make Targets:"
	@echo "  build-agent        - Build the main trading agent"
	@echo "  build-wasm         - Build the WASM market analysis component"
	@echo "  build-all          - Build both agent and WASM components"
	@echo "  run-agent          - Execute a trade (requires client, token, amount, trade params)"
	@echo "  run-wasm           - Run the WASM component for market analysis"
	@echo "  upload-wasm-ipfs   - Build WASM (release) and upload to Pinata IPFS"
	@echo "  upload-policy-ipfs - Upload policy.rego file to Pinata IPFS"
	@echo "  upload-policy-params-ipfs - Upload policy_params.json to Pinata IPFS"
	@echo "  upload-params-schema-ipfs - Upload params_schema.json to Pinata IPFS"
	@echo "  agent-help         - Show trading agent CLI help"
	@echo "  clean              - Clean build artifacts"
	@echo "  help               - Show this help message"
	@echo ""
	@echo "Examples:"
	@echo "  make upload-wasm-ipfs         # Build and upload WASM to IPFS"
	@echo "  make upload-policy-ipfs       # Upload policy.rego to IPFS"
	@echo "  make upload-policy-params-ipfs # Upload policy_params.json to IPFS"
	@echo "  make upload-params-schema-ipfs # Upload params_schema.json to IPFS"
	@echo "  make run-agent client=0x123... token=0xabc... amount=1000000000 trade=buy"
	@echo ""
	@echo "For IPFS upload setup and troubleshooting, see: IPFS_UPLOAD.md"

# client: Address, token: Address, amount: u64, trade: BuyOrSell
run-agent: build-agent
	./target/release/trade-agent --client $(client) --token $(token) --amount $(amount) --trade $(trade)

# Upload WASM to IPFS via Pinata
upload-wasm-ipfs: build-wasm
	@echo "================================================"
	@echo "============ Upload WASM component ============="
	@echo "================================================"
	@WASM_FILE=""; \
	if [ -f target/wasm32-wasip2/release/main.wasm ]; then \
		WASM_FILE="target/wasm32-wasip2/release/main.wasm"; \
	elif [ -f target/wasm32-wasip1/release/main.wasm ]; then \
		WASM_FILE="target/wasm32-wasip1/release/main.wasm"; \
	elif [ -f target/wasm32-wasip1/release/newton-trade-agent-wasm.wasm ]; then \
		WASM_FILE="target/wasm32-wasip1/release/newton-trade-agent-wasm.wasm"; \
	else \
		echo "Error: No WASM file found. Checked:"; \
		echo "  - target/wasm32-wasip2/release/main.wasm"; \
		echo "  - target/wasm32-wasip1/release/main.wasm"; \
		echo "  - target/wasm32-wasip1/release/newton-trade-agent-wasm.wasm"; \
		exit 1; \
	fi; \
	echo "Using WASM file: $$WASM_FILE"; \
	echo "$$WASM_FILE" > /tmp/wasm_file_path
	@echo "Setting up Pinata configuration..."
	@source .env && \
	if [[ "$$PINATA_JWT" =~ ^\*+$$ ]]; then \
		echo "⚠️  PINATA_JWT is redacted in .env file. Please replace with actual JWT."; \
		echo "Current JWT: $$PINATA_JWT"; \
		echo "You need to replace the asterisks with your real JWT from Pinata."; \
		echo "For now, skipping automatic authentication..."; \
	else \
		echo "Authenticating with Pinata and setting gateway..." && \
		./pinata-auth.expect "$$PINATA_JWT" "$$PINATA_GATEWAY" > /dev/null 2>&1 && \
		echo "Pinata configuration complete."; \
	fi
	@echo "Uploading WASM to Pinata IPFS..."
	@WASM_FILE=$$(cat /tmp/wasm_file_path); \
	source .env && ~/.local/share/pinata/pinata upload "$$WASM_FILE" --name "newton-trade-agent-wasm-$(shell date +%Y%m%d-%H%M%S)" | tee /tmp/pinata_upload.log
	@echo ""
	@echo "=== IPFS Upload Results ==="
	@IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_upload.log | head -1); \
	if [ -n "$$IPFS_HASH" ]; then \
		echo "IPFS Hash: $$IPFS_HASH"; \
		echo "Getting gateway link..."; \
		GATEWAY_LINK=$$(~/.local/share/pinata/pinata gateways link "$$IPFS_HASH" 2>/dev/null || echo "https://gateway.pinata.cloud/ipfs/$$IPFS_HASH"); \
		echo "Direct IPFS Link: $$GATEWAY_LINK"; \
		echo "Public IPFS Link: https://ipfs.io/ipfs/$$IPFS_HASH"; \
	else \
		echo "Warning: Could not extract IPFS hash from upload output"; \
		cat /tmp/pinata_upload.log; \
	fi
	@rm -f /tmp/pinata_upload.log /tmp/wasm_file_path

# Upload policy.rego to IPFS via Pinata
upload-policy-ipfs:
	@echo "================================================"
	@echo "============== Upload policy.rego =============="
	@echo "================================================"
	@if [ ! -f policy.rego ]; then \
		echo "Error: policy.rego file not found in current directory"; \
		exit 1; \
	fi
	@echo "Setting up Pinata configuration..."
	@source .env && \
	if [[ "$$PINATA_JWT" =~ ^\*+$$ ]]; then \
		echo "⚠️  PINATA_JWT is redacted in .env file. Please replace with actual JWT."; \
		echo "Current JWT: $$PINATA_JWT"; \
		echo "You need to replace the asterisks with your real JWT from Pinata."; \
		echo "For now, skipping automatic authentication..."; \
	else \
		echo "Authenticating with Pinata and setting gateway..." && \
		./pinata-auth.expect "$$PINATA_JWT" "$$PINATA_GATEWAY" > /dev/null 2>&1 && \
		echo "Pinata configuration complete."; \
	fi
	@echo "Uploading policy.rego to Pinata IPFS..."
	@source .env && ~/.local/share/pinata/pinata upload policy.rego --name "newton-trade-agent-policy-$(shell date +%Y%m%d-%H%M%S)" | tee /tmp/pinata_upload.log
	@echo ""
	@echo "=== IPFS Upload Results ==="
	@IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_upload.log | head -1); \
	if [ -n "$$IPFS_HASH" ]; then \
		echo "IPFS Hash: $$IPFS_HASH"; \
		echo "Getting gateway link..."; \
		GATEWAY_LINK=$$(~/.local/share/pinata/pinata gateways link "$$IPFS_HASH" 2>/dev/null || echo "https://gateway.pinata.cloud/ipfs/$$IPFS_HASH"); \
		echo "Direct IPFS Link: $$GATEWAY_LINK"; \
		echo "Public IPFS Link: https://ipfs.io/ipfs/$$IPFS_HASH"; \
	else \
		echo "Warning: Could not extract IPFS hash from upload output"; \
		cat /tmp/pinata_upload.log; \
	fi
	@rm -f /tmp/pinata_upload.log

# Upload params_schema.json to IPFS via Pinata
upload-params-schema-ipfs:
	@echo "================================================"
	@echo "========== Upload params_schema.json ==========="
	@echo "================================================"
	@if [ ! -f params_schema.json ]; then \
		echo "Error: params_schema.json file not found in current directory"; \
		exit 1; \
	fi
	@echo "Setting up Pinata configuration..."
	@source .env && \
	if [[ "$$PINATA_JWT" =~ ^\*+$$ ]]; then \
		echo "⚠️  PINATA_JWT is redacted in .env file. Please replace with actual JWT."; \
		echo "Current JWT: $$PINATA_JWT"; \
		echo "You need to replace the asterisks with your real JWT from Pinata."; \
		echo "For now, skipping automatic authentication..."; \
	else \
		echo "Authenticating with Pinata and setting gateway..." && \
		./pinata-auth.expect "$$PINATA_JWT" "$$PINATA_GATEWAY" > /dev/null 2>&1 && \
		echo "Pinata configuration complete."; \
	fi
	@echo "Uploading params_schema.json to Pinata IPFS..."
	@source .env && ~/.local/share/pinata/pinata upload params_schema.json --name "newton-trade-agent-params-schema-$(shell date +%Y%m%d-%H%M%S)" | tee /tmp/pinata_upload.log
	@echo ""
	@echo "=== IPFS Upload Results ==="
	@IPFS_HASH=$$(grep -o 'Qm[A-Za-z0-9]\{44\}\|baf[A-Za-z0-9]\{55,\}' /tmp/pinata_upload.log | head -1); \
	if [ -n "$$IPFS_HASH" ]; then \
		echo "IPFS Hash: $$IPFS_HASH"; \
		echo "Getting gateway link..."; \
		GATEWAY_LINK=$$(~/.local/share/pinata/pinata gateways link "$$IPFS_HASH" 2>/dev/null || echo "https://gateway.pinata.cloud/ipfs/$$IPFS_HASH"); \
		echo "Direct IPFS Link: $$GATEWAY_LINK"; \
		echo "Public IPFS Link: https://ipfs.io/ipfs/$$IPFS_HASH"; \
	else \
		echo "Warning: Could not extract IPFS hash from upload output"; \
		cat /tmp/pinata_upload.log; \
	fi
	@rm -f /tmp/pinata_upload.log

clean:
	cargo clean -p trade-agent -p newton-trade-agent-wasm -p shared

upload-all-ipfs: upload-wasm-ipfs upload-policy-ipfs upload-params-schema-ipfs
