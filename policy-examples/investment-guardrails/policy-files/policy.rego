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

# Allow buy if strategy is long term and the market is projected to grow long term
allow if {
	strategy == "long_term"
    function_name == "buy"
    (yield_10_year - yield_2_year) + 0.5 * (yield_30_year - yield_10_year) - (yield_1_month - yield_2_year) > 1
}

# Allow sell if strategy is long term and the market is projected to shrink long term
allow if {
	strategy == "long_term"
    function_name == "sell"
    (yield_10_year - yield_2_year) + 0.5 * (yield_30_year - yield_10_year) - (yield_1_month - yield_2_year) < 1
}

# Allow buy if strategy is short term and the market is projected to grow short term
allow if {
	strategy == "short_term"
    function_name == "buy"
    (yield_2_year - yield_1_month) + 0.5 * (yield_1_year - yield_3_month) > 1
}

# Allow sell if strategy is short term and the market is projected to shrink short term
allow if {
	strategy == "short_term"
    function_name == "sell"
    (yield_2_year - yield_1_month) + 0.5 * (yield_1_year - yield_3_month) < 1
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

