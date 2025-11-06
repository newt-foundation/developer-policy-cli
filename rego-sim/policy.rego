package example_policy

default allow := false

allow if {
	input.chain_id == data.params.chain_id
	current_gas_price < max_gas_price
}

max_gas_price := data.params.max_gas_price
current_gas_price := data.data[format_int(data.params.chain_id, 10)]