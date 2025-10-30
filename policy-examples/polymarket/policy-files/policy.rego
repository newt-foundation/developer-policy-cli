# Newton Polymarket Agent Policy
# --------------------------------
# Evaluates Fed interest rate sentiment from Polymarket data
#
# WASM agent returns:
# {
#   "sentiment": "decrease" | "not_decrease",
#   "decrease_percentage": <float>,
#   "not_decrease_percentage": <float>
# }

package newton_polymarket_agent

# By default, deny requests
default allow := false

# Get polymarket sentiment from wasm response
polymarket_sentiment := data.data.sentiment

# Get required sentiment from parameters
required_sentiment := data.params.sentiment

# Allow if required sentiment matches polymarket result
allow if {
    required_sentiment == polymarket_sentiment
}