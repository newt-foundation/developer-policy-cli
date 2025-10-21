# Investment guardrails policy
# --------------------------------
#
# For more information see:
#
#	* Rego comparison to other systems: https://www.openpolicyagent.org/docs/latest/comparison-to-other-systems/
#	* Rego Iteration: https://www.openpolicyagent.org/docs/latest/#iteration

package investment_guardrails

default allow := false

yield_1_month = data.data.yield_1_month
yield_3_month = data.data.yield_3_month
yield_1_year = data.data.yield_1_year
yield_2_year = data.data.yield_2_year
yield_5_year = data.data.yield_5_year
yield_10_year = data.data.yield_10_year
yield_30_year = data.data.yield_30_year

strategy = data.params.strategy
function_name := input.function.name

# ------------ Indices ------------
long_term_growth_index :=
  (yield_10_year - yield_2_year) +
  0.5 * (yield_30_year - yield_10_year) -
  (yield_1_month - yield_2_year)

short_term_growth_index :=
  (yield_2_year - yield_1_month) +
  0.5 * (yield_1_year - yield_3_month)


# ------------ Thresholds ------------
lt_buy_threshold := 1.0    # strong long-term growth
lt_sell_threshold := -1.0  # clear contraction risk

st_buy_threshold := 0.5    # near-term support/easing
st_sell_threshold := -0.5  # near-term drag/tightness

# ------------ Allow Rules ------------
# Long-term strategy
allow if {
  strategy == "long_term"
  function_name == "buy"
  long_term_growth_index > lt_buy_threshold
}

allow if {
  strategy == "long_term"
  function_name == "sell"
  long_term_growth_index < lt_sell_threshold
}

# Short-term strategy
allow if {
  strategy == "short_term"
  function_name == "buy"
  short_term_growth_index > st_buy_threshold
}

allow if {
  strategy == "short_term"
  function_name == "sell"
  short_term_growth_index < st_sell_threshold
}


