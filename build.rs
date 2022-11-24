fn main() {
    let build_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("cargo:rerun-if-changed=src/codegen/mod.rs");
    println!("cargo:rustc-env=PEGINATOR_BUILD_TIME={}", build_time);
}
