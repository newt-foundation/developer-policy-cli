use rego_sim::generate_local_policy_input_data;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 6 {
        eprintln!("Usage: marshal <policy.rego> <policy_params_data.json> <test_intent.json> <entrypoint> <output.json>");
        std::process::exit(1);
    }
    let policy_path = &args[1];
    let params_path = &args[2];
    let intent_path = &args[3];
    let entrypoint = &args[4];
    let output_path = &args[5];

    match generate_local_policy_input_data(policy_path, params_path, intent_path, None, entrypoint) {
        Ok(result) => {
            std::fs::write(output_path, serde_json::to_string_pretty(&result).unwrap()).unwrap();
            println!("Wrote marshaled and evaluated result to {}", output_path);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}
