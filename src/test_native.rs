// Test the same HTTP code natively to prove it works
extern crate test_http_rust;

fn main() {
    // Call the same function that's in the WASM
    test_http_rust::run();
}