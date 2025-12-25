use std::{env, path::PathBuf, process::Command};

fn main() {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("generate") => generate_pac(),
        _ => print_help(),
    }
}

fn generate_pac() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned();
    let pac_path = root.join("sg2000-pac");
    let svd_path = pac_path.join("svd/SG2000.svd");
    let config_path = pac_path.join("svd2rust.toml");

    println!("Generating PAC from {}", svd_path.display());

    if !Command::new("svd2rust")
        .arg("-c")
        .arg(&config_path)
        .arg("-i")
        .arg(&svd_path)
        .arg("--strict")
        .arg("--atomics")
        .current_dir(&pac_path)
        .status()
        .is_ok_and(|status| status.success())
    {
        panic!("svd2rust failed");
    }

    if !Command::new("form")
        .arg("-i")
        .arg("lib.rs")
        .arg("-o")
        .arg("src")
        .current_dir(&pac_path)
        .status()
        .is_ok_and(|status| status.success())
    {
        panic!("form failed");
    }

    std::fs::remove_file(pac_path.join("lib.rs")).ok();

    if !Command::new("cargo")
        .arg("fmt")
        .current_dir("sg2000-pac")
        .status()
        .is_ok_and(|status| status.success())
    {
        panic!("cargo fmt failed");
    }
}

fn print_help() {
    println!("Usage: cargo xtask generate");
}
