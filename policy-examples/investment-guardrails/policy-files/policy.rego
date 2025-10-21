# Investment guardrails policy
# --------------------------------
#
# For more information see:
#
#	* Rego comparison to other systems: https://www.openpolicyagent.org/docs/latest/comparison-to-other-systems/
#	* Rego Iteration: https://www.openpolicyagent.org/docs/latest/#iteration

package investment_guardrails

# By default, deny requests.
default allow := false

# Allow the action if the current gas price is below the configured threshold
allow if {
	strategy == "long_term"
    function_name == "buy"
}
allow if {
	strategy == "long_term"
    function_name == sell
}
allow if {
	strategy == "short_term"
    function_name == "buy"
}
allow if {
	strategy == "short_term"
    function_name == sell
}

yield_1_month = data.data.yield_1_month
yield_3_month = data.data.yield_3_month
yield_1_year = data.data.yield_1_year
yield_2_year = data.data.yield_2_year
yield_5_year = data.data.yield_5_year
yield_10_year = data.data.yield_10_year
yield_30_year = data.data.yield_30_year
strategy = data.params.strategy
function_name := input.function.name

