# Gas price send transaction policy
# --------------------------------
#
# For more information see:
#
#	* Rego comparison to other systems: https://www.openpolicyagent.org/docs/latest/comparison-to-other-systems/
#	* Rego Iteration: https://www.openpolicyagent.org/docs/latest/#iteration

package max_gas_price

# By default, deny requests.
default allow := false

# Allow the action if the current gas price is below the configured threshold
allow if {
	input.chain_id == format_int(data.params.chain_id, 10)
	current_gas_price < max_gas_price
}

max_gas_price := to_number(data.params.max_gas_price)
current_gas_price := data.data[format_int(data.params.chain_id, 10)]
