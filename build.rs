use std::process::Command;
use std::path::Path;

#[cfg(target_os = "macos")]
pub fn main() {
    use std::env;
    let toml_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    Command::new("sh").arg(&format!("{}/buildsetup.sh", toml_dir)).status().unwrap();
    println!("cargo:rustc-link-search=native={}", toml_dir);
    println!("cargo:rustc-link-lib=static=setup");
}