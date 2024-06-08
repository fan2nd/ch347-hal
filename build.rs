use std::env;
use std::path::PathBuf;

fn main() {
    // let mut search_path = env::current_dir().unwrap();
    // search_path.push("vendor_lib");
    let mut search_path = PathBuf::from("vendor_lib");
    search_path.push(env::var("CARGO_CFG_TARGET_OS").unwrap());
    match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "aarch64" => search_path.push("arm64"),
        "x86_64" => search_path.push("amd64"),
        arch => panic!("Target OS not supported: {arch}"),
    }
    println!("cargo:rustc-link-search=native={}", search_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=ch347");
}
