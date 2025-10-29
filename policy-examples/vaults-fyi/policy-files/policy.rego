 package vaults_fyi_demo

# Deny by default
default allow := false

allow if {
	apy_ok
}

# Inputs and parameters
min_30day_apy := to_number(data.params.min_30day_apy)

# Wasm data
apy30 := data.data.apy["30day"].total

apy_ok if {
	apy30 >= min_30day_apy
}