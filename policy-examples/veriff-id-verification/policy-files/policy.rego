# Newton Veriff Agent Policy
# --------------------------------

package newton_veriff_agent

# By default, deny requests.
default allow := false

####################################################################
## Real-time Veriff Data from backend

# Get verification status from WASM response
verification_status := data.data.verification_status

# Get country from WASM response (uppercase)
country := upper(data.data.country)

# Get allowed countries from input parameters (expecting array of uppercase country codes)
# Example: data.params.allowed_countries = ["US", "CA", "UK", "DE"]
allowed_countries := data.params.allowed_countries

####################################################################
## Allow Rules

# Allow if verification is approved AND country is in the allowed list
allow if {
    # Check verification status
    verification_status == "approved"

    # Check if country is in the allowed countries list
    country_allowed
}

# Helper rule to check if country is allowed
country_allowed if {
    # Check if allowed_countries is defined and not empty
    count(allowed_countries) > 0

    # Check if the user's country is in the allowed list
    country in allowed_countries
}

