use std::{
    env,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

fn main() {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("generate") => generate_pac(),
        Some("size") => size(),
        Some("objdump") => objdump(),
        Some("flash") => flash(),
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

fn size() {
    build();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned();
    let elf_path = root.join("target/riscv64gc-unknown-none-elf/release/synthgut");

    if !Command::new("rust-size")
        .arg(&elf_path)
        .status()
        .is_ok_and(|status| status.success())
    {
        panic!("rust-size failed");
    }
}

fn objdump() {
    build();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned();
    let elf_path = root.join("target/riscv64gc-unknown-none-elf/release/synthgut");

    if !Command::new("rust-objdump")
        .arg("--disassemble")
        .arg("--no-show-raw-insn")
        .arg("--print-imm-hex")
        .arg(&elf_path)
        .status()
        .is_ok_and(|status| status.success())
    {
        panic!("rust-objdump failed");
    }
}

fn flash() {
    build();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned();
    let elf_path = root.join("target/riscv64gc-unknown-none-elf/release/synthgut");

    // Command: echo "put <path> synthgut.elf" | sftp -b - debian@10.42.0.1
    let mut child = Command::new("sftp")
        .arg("-b")
        .arg("-")
        .arg("debian@10.42.0.1")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn sftp");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    write!(stdin, "put {} synthgut.elf", elf_path.display()).expect("Failed to write to stdin");
    // Close stdin to signal EOF
    drop(stdin);

    if !child.wait().expect("Failed to wait on child").success() {
        panic!("sftp failed");
    }
}

fn build() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned();

    if !Command::new("cargo")
        .arg("build")
        .arg("-p")
        .arg("synthgut")
        .arg("--target")
        .arg("riscv64gc-unknown-none-elf")
        .arg("--release")
        .current_dir(root)
        .status()
        .is_ok_and(|status| status.success())
    {
        panic!("Build failed");
    }
}

fn print_help() {
    println!("Usage: cargo xtask <command>");
    println!("Commands:");
    println!("  generate    Generate PAC from SVD");
    println!("  size        Build and show binary size");
    println!("  objdump     Build and disassemble binary");
    println!("  flash       Build and transfer binary to device via SFTP");
}
