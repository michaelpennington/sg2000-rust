# List all available recipes
default:
    @just --list

# Build the firmware (synthgut)
# We 'cd' into the directory so Cargo picks up .cargo/config.toml for the target & flags
build: 
    cargo build -p synthgut --target riscv64gc-unknown-none-elf --release

# Build the firmware and display size (requires cargo-binutils from your flake)
size: build
    rust-size target/riscv64gc-unknown-none-elf/release/synthgut

# Build and disassemble binary
objdump: build
    rust-objdump --disassemble --no-show-raw-insn --print-imm-hex target/riscv64gc-unknown-none-elf/release/synthgut

# Build and transfer binary to device via SFTP
flash: build
    echo "put target/riscv64gc-unknown-none-elf/release/synthgut synthgut.elf" | sftp -b - debian@10.42.0.1

# Generate PAC from SVD
generate:
    cd sg2000-pac && svd2rust -c svd2rust.toml -i svd/SG2000.svd --strict --atomics
    cd sg2000-pac && form -i lib.rs -o src
    rm sg2000-pac/lib.rs
    cd sg2000-pac && cargo fmt

# Generate documentation for the workspace
doc:
    cargo doc --no-deps --open

# Check everything (host tools + firmware)
check:
  cargo check -p sg2000-hal --target riscv64gc-unknown-none-elf
  cargo check -p sg2000-pac --target riscv64gc-unknown-none-elf
  cargo check -p synthgut --target riscv64gc-unknown-none-elf
