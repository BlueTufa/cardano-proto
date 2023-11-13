# Cardano Prototype

## Getting Started
If you don't already have a Rust environment established, you may need to install Rustup, or a similar Rust toolchain:
```bash
# installing Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
```bash
cargo run -- -p af3566f6e17b0d6a829f673c99a2f4c782bea73b04d8e1cc63db01c6 -d ~/Downloads/ -k <your project_id token here>

# you may optionally set up logging levels as an environment variable.  Example:
export RUST_LOG=debug
```
## Requirements:
- [x] Command-line application
- [x] Accepts 2 arguments:
  - [x] Cardano Policy ID
  - [x] Path to output directory
- [x] Validate the Policy ID against book.io collections - it just panics for now
* Download the "High-res cover image" from 10 books
* Store images in the specified output directory
* The application must be idempotent