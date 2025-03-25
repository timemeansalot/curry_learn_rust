/// test env::var in rust
/// # date: 2025-03-25
/// # how to run:
/// ```bash
/// export IS_FLAG=true && cargo run
/// cargo run
/// ```
use std::env;
fn main() {
    let flag = env::var("IS_FLAG").unwrap_or_else(|_| "false".to_string());
    if flag == "true" {
        println!("read true from env!");
    } else {
        println!("read false from env!");
    }
}
