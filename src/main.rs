/// test env::var in rust
/// # date: 2025-03-25
/// # how to run:
/// ```bash
/// export IS_FLAG=true && cargo run
/// cargo run
/// ```
use std::env;
fn main() {
    let default = "false".to_string();
    // unwrap_or: return value in Ok(value) or return the default value provided
    let flag = env::var("IS_FLAG").unwrap_or(default);
    // unwrap_or_else: return value in Ok(value) or calculate the value from closure provided
    //let flag = env::var("IS_FLAG").unwrap_or_else(|_| "false".to_string());
    if flag == "true" {
        println!("read true from env!");
    } else {
        println!("read false from env!");
    }
}
