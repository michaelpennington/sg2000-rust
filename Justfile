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

# Run the xtask automation (wraps your .cargo/config.toml alias)
xtask +args:
    cargo xtask {{args}}

# Generate documentation for the workspace
doc:
    cargo doc --no-deps --open

# Check everything (host tools + firmware)
check:
  cargo check -p xtask --target x86_64-unknown-linux-gnu
  cargo check -p sg2000-hal --target riscv64gc-unknown-none-elf
  cargo check -p sg2000-pac --target riscv64gc-unknown-none-elf
  cargo check -p synthgut --target riscv64gc-unknown-none-elf
