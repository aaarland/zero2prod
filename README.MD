# Zero 2 Production

This is the code repository for [Zero 2 Production](https://www.zero2prod.com/), a book about building production-ready applications in Rust.

## Running the code

The code in this repository is organized in chapters, each of which is a Cargo workspace. To run the code in a chapter, `cd` into the chapter directory and run `cargo run`.

## CI integration

### Testing

```bash
# Run all tests
cargo test
# Run all tests in a chapter
cargo test -p chapter_01
# Run a single test
cargo test -p chapter_01 -- test_name
```

### Code coverage

```bash 
# Install tarpaulin
cargo install cargo-tarpaulin
# Run tarpaulin
cargo tarpaulin --all --out Html
# Open the coverage report
open tarpaulin-report.html
```

### Linting

```bash
# Install clippy
rustup component add clippy
# Run clippy
cargo clippy
# Run clippy and fail the pipeline if there are warnings
cargo clippy -- -D warnings
```

### Formatting

```bash 
# Install rustfmt
rustup component add rustfmt
# Run rustfmt
cargo fmt
# Run rustfmt and fail the pipeline if there are warnings
cargo fmt -- --check
```

### Security audit

```bash
# Install cargo audit
cargo install cargo-audit
# Run cargo audit
cargo audit
```
