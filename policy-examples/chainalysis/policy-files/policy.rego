
# Chainalysis Sanctions Policy
# --------------------------------

package chainalysis_sanctions_agent

# By default, deny requests.
default allow := false

# From Intent
to_address := input.to

# From Policy Data
api_call_status := data.data.status
checked_address := data.data.address
is_sanctioned := data.data.sanctioned

# Allow the action if the `to` address is not sanctioned
allow if {
    api_call_status == 200
    checked_address == to_address
    not is_sanctioned
}