# Newton Trading Agent Policy
# --------------------------------

package newton_trading_agent

# By default, deny requests.
default allow := false

####################################################################
## User configured policy parameters
user_policy_params := data.params[input.chain_id]

####################################################################
## Agent Intent to be evaluated

# Contract function name
function_name := input.function.name

# Contract function arguments
# 1. token to buy or sell
token := input.decoded_function_arguments[0]
# 2. amount in to buy or sell
amount_in := to_number(input.decoded_function_arguments[1])
# 3. slippage to buy or sell
# slippage to buy or sell
slippage := to_number(input.decoded_function_arguments[2])

####################################################################

# Allow only if all conditions are met
allow {

    ################################################################
    ## User Policy Parameters Checks

    # Check if token is whitelisted
    whitelist_contracts := object.keys(user_policy_params)
    token in whitelist_contracts

    # Check if function name is an allowed action
    allowed_actions := object.keys(user_policy_params[token])
    function_name in allowed_actions

    # Check if amount in is within the max limit
    max_limit := user_policy_params[token][function_name][0].value
    amount_in <= max_limit

    # Check if slippage is within the max slippage
    max_slippage := user_policy_params[token][function_name][1].value
    slippage <= max_slippage

    ################################################################
    ## Policy Real-time Context Data Checks

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
