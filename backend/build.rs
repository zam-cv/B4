use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let mode = env::var("MODE").expect("MY_CONFIG must be set");
    println!("cargo:rustc-cfg=feature=\"{}\"", mode);
}
