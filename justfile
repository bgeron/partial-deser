
# _default: clippy

_default-dev: fmt clippy-allow-dead

all: clippy test doc

clippy:
    cargo clippy --all-targets --all-features
    cargo clippy --all-targets --no-default-features

clippy-allow-dead:
    cargo clippy --all-targets --all-features -- --allow dead_code
    cargo clippy --all-targets --no-default-features -- --allow dead_code

test: clippy
    cargo test --all-targets --all-features
    cargo test --all-targets --no-default-features
    cargo test --all-features --doc

fmt:
    cargo fmt

doc:
    RUSTDOCFLAGS="--cfg docsrs" CARGO_TARGET_DIR=target-nightly cargo +nightly doc --lib --bins --examples --all-features

doc-open:
    RUSTDOCFLAGS="--cfg docsrs" CARGO_TARGET_DIR=target-nightly cargo +nightly doc --lib --bins --examples --all-features --document-private-items --open