package persona_demo

# Deny by default
default allow := false

# Wasm data
inquiry := data.data
status := inquiry.status
age := inquiry.computed.age
state := inquiry.state
country := inquiry.country_code
bot_score := inquiry.bot_score


allow if {
    age >= 18
    country == "US"
    state != "NY"
    status == "approved"
    bot_score <= 2
}

