package top200dma200

default allow = false

requested_token := lower(input.to)

# Allow only if requested is non-empty AND present in both user whitelisted and wasm response
allow {
    requested_token != ""
    lower(input.function.name) == "swap"

    # Argument 0 must be > 0
    to_number(input.decoded_function_arguments[0]) > 0

    # Argument 1 must equal 1
    to_number(input.decoded_function_arguments[1]) == 1

    price := data.data.prices_usd[requested_token]
    dma   := data.data.indicators.dma_200[requested_token]
    rank  := data.data.indicators.market_cap_rank[requested_token]

    is_number(price)
    is_number(dma)
    is_number(rank)

    price >= dma
    rank <= 200
}