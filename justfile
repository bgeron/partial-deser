
_default: all

dev: fmt clippy-allow-dead

all: fmt clippy test doc failing-test

clippy:
    cargo clippy --all-targets --all-features
    cargo clippy --all-targets --no-default-features

clippy-allow-dead:
    cargo clippy --all-targets --all-features -- --allow dead_code
    cargo clippy --all-targets --no-default-features -- --allow dead_code

main-test:
    cargo test --no-fail-fast --all-targets --all-features

test:
    cargo test --no-fail-fast --all-targets --all-features
    cargo test --no-fail-fast --all-targets --no-default-features

failing-test:
    cargo test --no-fail-fast --all-features --doc

insta:
    cargo test --no-fail-fast --all-targets --all-features

fmt:
    cargo +nightly fmt

doc:
    RUSTDOCFLAGS="--cfg docsrs" CARGO_TARGET_DIR=target-nightly cargo +nightly doc --lib --bins --examples --all-features

doc-open:
    RUSTDOCFLAGS="--cfg docsrs" CARGO_TARGET_DIR=target-nightly cargo +nightly doc --lib --bins --examples --all-features --document-private-items --open