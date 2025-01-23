
_default: all

dev: fmt clippy-allow-dead

all: fmt clippy test doc

motivating-example *extra_args:
    @cargo build --target-dir target --example print-slowly --example show-live --all-features
    @cat examples/motivating-example.json-fragment | target/debug/examples/print-slowly -t 0.1 --delay-at-start 2.5 | target/debug/examples/show-live --schema travel-modes {{extra_args}}

clippy:
    cargo clippy --all-features
    cargo clippy
    cargo clippy --no-default-features --features serde_json --features rand
    cargo clippy --no-default-features --features serde_yaml --features rand
    cargo clippy --no-default-features --features serde_yaml
    cargo clippy --no-default-features --features serde_json
    cargo clippy --no-default-features
    cargo clippy --no-default-features --features tracing
    cargo clippy --all-targets --all-features

clippy-allow-dead:
    cargo clippy --all-targets --all-features -- --allow dead_code
    cargo clippy --all-targets --no-default-features -- --allow dead_code

main-test:
    cargo test --no-fail-fast --all-targets --all-features

test:
    cargo test --no-fail-fast --all-targets --all-features
    cargo test --no-fail-fast --all-targets --no-default-features
    cargo test --no-fail-fast --all-features --doc

insta:
    cargo test --no-fail-fast --all-targets --all-features

fmt:
    cargo +nightly fmt

doc:
    RUSTDOCFLAGS="--cfg docsrs" CARGO_TARGET_DIR=target-nightly cargo +nightly doc --lib --bins --examples --all-features --document-private-items --no-deps

doc-open:
    RUSTDOCFLAGS="--cfg docsrs" CARGO_TARGET_DIR=target-nightly cargo +nightly doc --lib --bins --examples --all-features --document-private-items --no-deps --open

doc-deps:
    # ratatui somehow cannot be built with `--cfg docsrs`
    CARGO_TARGET_DIR=target-nightly-deps cargo +nightly doc --lib --bins --examples --all-features

doc-deps-open:
    CARGO_TARGET_DIR=target-nightly-deps cargo +nightly doc --lib --bins --examples --all-features --open

tokei:
    tokei --exclude json_output --exclude yaml_output