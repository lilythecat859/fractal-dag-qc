use std::process::Command;
fn main() {
    Command::new("bash")
        .args(&["scripts/build_wasm.sh"])
        .status()
        .expect("WASM build failed");
    println!("cargo:rerun-if-changed=scripts/build_wasm.sh");
}
