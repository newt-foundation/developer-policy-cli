package example

# Allow if sender is authorized, value is within limit, and target is allowed
allow if {
    input.sender in data.authorized_senders
    input.value <= data.max_transaction_value
    input.target in data.allowed_contracts
}

# Deny if target is blacklisted
deny if {
    input.target in data.blacklisted_contracts
}
