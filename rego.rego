# Newton Trading Agent Policy
# --------------------------------

package newton_trading_agent

# By default, deny requests.
default allow := false

## User configured policy parameters

# allowed function names to call
allowed_action := data.params.allowed_actions[input.chain_id]
# whitelisted contracts to call allowed functions from
whitelist_contracts := data.params.whitelist_contracts[input.chain_id]
# Max limit for per trade (amount in)
max_limit := data.params.max_limit[input.chain_id]

## Agent Intent to be evaluated

# Contract function name
function_name := input.function.name
# Contract function arguments
token := input.decoded_function_arguments[0]
amount_in := to_number(input.decoded_function_arguments[1])

# Disallow if token is not whitelisted
allow = false if {
    # Token is not whitelisted
    not token in whitelist_contracts
    # Function name is not allowed
    function_name not in allowed_action
    # Amount in is greater than the max limit
    amount_in > max_limit
}

# Allow only if all conditions are met
allow {
    ## Policy Real-time Context Data

    # Current token price
    token_price := data.data.prices_usd[token]
    
    # Current token daily moving average past 200 days
    token_daily_moving_average   := data.data.indicators.dma_200[token]
    
    # Current token market cap rank
    token_market_cap_rank  := data.data.indicators.market_cap_rank[token]

    # Current token market cap rank must be less than or equal to 200
    token_market_cap_rank <= 200

    # Current token price must be greater than or equal to the current token daily moving average
    token_price >= token_daily_moving_average
}