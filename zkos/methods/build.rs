use std::collections::HashMap;

fn main() {
    let options_map = HashMap::from([(
        "methods-guest",
        risc0_build::GuestOptions {
            // change to 16 to run miden fully (will increase compile time)
            code_limit: 12,
            features: vec![],
        },
    )]);
    risc0_build::embed_methods_with_options(options_map);
}
