package top200dma200

default allow = false

# hardcoded address to symbol mappings to use
token_symbols := {
  "0xba2ae424d960c26247dd6c32edc70b295c744c43": "doge",
  "0x2170ed0880ac9a755fd29b2688956bd959f933f8": "eth",
  "0x0eb3a705fc54725037cc9e008bdede697f62f335": "atom" # less than 200dma
}

requested_token := lower(input.decoded_function_arguments[0])
requested_symbol := token_symbols[lower]

# array of whitelisted addr
whitelisted_tokens := data.params.whitelisted_tokens

# Set of symbols returned in the WASM response (validated with 200dma)
wasm_allowed_symbols[s] {
  qs := data.data[_]
  is_string(qs.symbol)
  s := qs.symbol
}

# Allow only if requested is non-empty AND present in both user whitelisted and wasm response
allow if {
  requested_token != ""
  whitelisted_tokens[requested_token]
  wasm_allowed_symbols[requested_symbol]
}
