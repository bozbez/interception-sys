use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let lib = Path::new(&root_dir)
        .join("interception-1.0.1")
        .join("x64")
        .join("interception.dll");

    let lib_out = Path::new(&out_dir).join("interception.dll");
    fs::copy(&lib, &lib_out).unwrap();

    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=dylib=interception");
}
