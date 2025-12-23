use std::{env, fs, io::Write};

fn main() {
    // Add OUT_DIR to the linker search path
    println!("cargo:rustc-link-arg=-Tmemory.x");
    println!("cargo:rustc-link-arg=-Tdevice.x");
    println!("cargo:rustc-link-arg=-Tlink.x");

    // Trigger a rebuild if memory.x changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=build/always_rerun");

    let outdir = env::var("OUT_DIR").unwrap();
    let outfile = format!("{}/timestamp.rs", outdir);
    let mut fh = fs::File::create(&outfile).unwrap();
    write!(fh, r#""{}""#, chrono::Local::now()).ok();
}
