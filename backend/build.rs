use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let mode = env::var("MODE").expect("MODE must be set");
    println!("cargo:rustc-cfg=feature=\"{}\"", mode);
}
