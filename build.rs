// build.rs
use std::env;

fn main() {
   let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search={}/previews-generator", project_dir); // the "-L" flag
    println!("cargo:rustc-link-lib=previews-generator"); // the "-l" flag
    println!("cargo:rustc-link-lib=stdc++"); // the "-l" flag
    println!("cargo:rustc-link-lib=skia"); // the "-l" flag
}