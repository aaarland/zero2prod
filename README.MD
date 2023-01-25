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

### Static analysis

```bash
# Install cargo geiger
cargo install cargo-geiger
# Run cargo geiger
cargo geiger
```

## Cargo tools

### Cargo nightly

    ```bash
    # Install cargo nightly
    rustup toolchain install nightly --allow-downgrade
    # Run cargo nightly
    cargo +nightly build
    ```

### Cargo watch

    ```bash
    # Install cargo watch
    cargo install cargo-watch
    # Run cargo watch
    cargo watch -x build
    ```

### Cargo udeps

    ```bash
    # Install cargo udeps
    cargo install cargo-udeps
    # Run cargo udeps
    cargo +nightly udeps
    ```

### Telemetry

    tracing
    open telemetry jeager honeycomb

## Email newsletter

### Specifications

User stroies:

- As a blog visitor, I want to subscribe to the newsletter so that I can receive updates about new blog posts.
- As a blog visitor, I want to unsubscribe from the newsletter so that I don't receive updates about new blog posts.
- As a blog author, I want to send emails to subscribers so that I can notify them about new blog posts.
