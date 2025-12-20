use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Copy memory.x to the OUT_DIR
    fs::write(out_dir.join("memory.x"), include_bytes!("memory.x")).unwrap();

    // Add OUT_DIR to the linker search path
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-arg=-Tmemory.x");
    println!("cargo:rustc-link-arg=-Tdevice.x");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Clink-dead-code");

    // Trigger a rebuild if memory.x changes
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=device.x");
    println!("cargo:rerun-if-changed=link.x");
    println!("cargo:rerun-if-changed=build.rs");
}
