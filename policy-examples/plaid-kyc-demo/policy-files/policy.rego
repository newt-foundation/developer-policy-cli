package plaid_demo

# Deny by default
default allow := false

# Wasm data
verification := data.data
steps = verification.steps

allow if {
    verification.status == "success"
    steps.accept_tos == "success"
    steps.kyc_check == "success"
    steps.documentary_verification == "success"
    steps.selfie_check == "success"
    steps.watchlist_screening == "success"
    steps.risk_check == "success"
}
