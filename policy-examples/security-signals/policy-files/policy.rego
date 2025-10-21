
# Chainalysis Sanctions Policy
# --------------------------------

package security_signals_agent

# By default, deny requests.
default allow := false

# From Intent
to_address := input.to

# From Policy Data
api_call_status := data.data.status
checked_address := data.data.address
risk_level := data.data.risk_level

# From Policy Params
min_allowed_risk_level := input.min_allowed_risk_level

# Map risk levels to numeric values for comparison
risk_level_value := {
    "low": 1,
    "medium": 2,
    "high": 3,
    "critical": 4
}

allow if {
    api_call_status == 200
    checked_address == to_address
    risk_level_value[risk_level] <= risk_level_value[min_allowed_risk_level]
}
