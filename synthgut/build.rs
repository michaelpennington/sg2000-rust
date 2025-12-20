fn main() {
    // Add OUT_DIR to the linker search path
    println!("cargo:rustc-link-arg=-Tmemory.x");
    println!("cargo:rustc-link-arg=-Tdevice.x");
    println!("cargo:rustc-link-arg=-Tlink.x");

    // Trigger a rebuild if memory.x changes
    println!("cargo:rerun-if-changed=build.rs");
}
