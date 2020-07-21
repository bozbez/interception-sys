use std::env;
use std::path::Path;

fn main() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = Path::new(&root_dir).join("interception-1.0.1");

    println!("cargo:rustc-link-search={}", lib_dir.join("x64").to_str().unwrap());
    println!("cargo:rustc-link-lib=interception");
}
