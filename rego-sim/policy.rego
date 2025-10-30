package example_policy

default allow := false

allow if {
  data.params.max_gas_price < data.data["11155111"]
  input.function.name == "buy"
}
