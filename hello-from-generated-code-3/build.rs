
use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let pwd_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={:?}/lib", pwd_dir);
    println!("cargo:rustc-link-lib=dylib=add");
    // println!("cargo:rerun-if-changed=src/hello.c");
}