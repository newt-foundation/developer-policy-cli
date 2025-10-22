package farcaster_demo

# Deny by default
default allow := false

allow if {
	verification_ok
	sufficient_score
}

# Inputs and parameters
min_score := to_number(data.params.min_score)
require_verified := data.params.require_verified

# Wasm data
inquiry := data.data
score := inquiry.score
verified_accounts := inquiry.verified_accounts

sufficient_score if {
	score >= min_score
}

has_verified_account if {
	count(verified_accounts) > 0
}

verification_ok if {
	not require_verified
}

verification_ok if {
	has_verified_account
}
