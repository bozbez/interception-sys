use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let lib_dir = Path::new(&root_dir)
        .join("interception-1.0.1")
        .join("x64");

    let stlib = lib_dir.join("interception.lib");
    let dylib = lib_dir.join("interception.dll");

    let stlib_out = Path::new(&out_dir).join("interception.lib");
    let dylib_out = Path::new(&out_dir).join("interception.dll");

    fs::copy(&dylib, &dylib_out).unwrap();
    fs::copy(&stlib, &stlib_out).unwrap();

    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=dylib=interception");
}
