# Cardano Prototype

## Getting Started
If you don't already have a Rust environment established, you may need to install Rustup, or a similar Rust toolchain:
```bash
# installing Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Once you have an environment set up, you need to specify your Bitfrost `project_id` token.  You may set this via the `BLOCKFROST_PROJECT_ID` environment variable. 
The command line utility accepts 2 arguments.  `--policy-id` and `--output-dir`, or `-p` and `-d` in short form.  If the output directory does not exist, it will be created.
```bash
export BLOCKFROST_PROJECT_ID=<your project token here>
# you may optionally set up logging levels as an environment variable.  Example:
export RUST_LOG=debug
# specify a policy id which is also a Book.io collection.
cargo run -- -p af3566f6e17b0d6a829f673c99a2f4c782bea73b04d8e1cc63db01c6 -d $(pwd)/downloads

```
## Requirements:
- [x] Command-line application
- [x] Accepts 2 arguments:
  - [x] Cardano Policy ID
  - [x] Path to output directory
- [x] Validate the Policy ID against book.io collections
  * Right now, it panics when validation fails 
- [x] Download the "High-res cover image" from 10 books
  * The logic to limit the download to 10 unique books isn't built out yet.
- [x] Store images in the specified output directory
- [x] The application must be idempotent

## What's left to do:
* Unit tests and code coverage
* CI/CD enhancements via GitHub actions
* Add support for additional IPFS endpoints
* Improve error handling and add more recoverable error conditions
