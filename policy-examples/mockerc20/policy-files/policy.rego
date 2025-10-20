# MockERC20 Token Buy Policy
# --------------------------------
#
# For more information see:
#
#	* Rego comparison to other systems: https://www.openpolicyagent.org/docs/latest/comparison-to-other-systems/
#	* Rego Iteration: https://www.openpolicyagent.org/docs/latest/#iteration

package mockerc20

# By default, deny requests.
default allow := false

# Allow admins to do anything.
allow if user_is_admin

# user_is_admin is true if sender is the admin
user_is_admin if {
    data.params.admin == input.from
}

# Allow the action if the user is granted permission to perform the action.
allow if {
	base_symbol == "BTC"
	price > amount_out
	quote_symbol == "USD"
	token == lower(whitelist_token.address)
	function_name == allowed_action.function_name
	token == lower(allowed_action.address)
	allowed_action.max_limit > amount_out
}

function_name := input.function.name
token := input.decoded_function_arguments[0]
amount_out := to_number(input.decoded_function_arguments[1])
allowed_action := data.params.allowed_actions[input.chain_id]
whitelist_token := data.params.token_whitelist[input.chain_id]
base_symbol := data.data.base_symbol
price := to_number(data.data.price)
quote_symbol := data.data.quote_symbol