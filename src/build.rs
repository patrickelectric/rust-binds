extern crate cbindgen;

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let bindings_file = std::path::Path::new(&target_dir)
        .join(&profile)
        .join("bindings.h");
    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file(bindings_file);
}
