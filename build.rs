use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not defined");
    // Read the environment variable "TOPIC_LEN"; default to "64" if not set.
    let topic_len = env::var("TOPIC_LEN").unwrap_or_else(|_| "64".to_string());
    let dest_path = Path::new(&out_dir).join("constants.rs");
    let mut f = File::create(&dest_path).expect("Could not create constants file");
    write!(f, "pub const TOPIC_LEN: usize = {};", topic_len).expect("Could not write constant");

    // Tell Cargo to re-run the build script if "MY_TOPIC_LEN" changes.
    println!("cargo:rerun-if-env-changed=MY_TOPIC_LEN");
}