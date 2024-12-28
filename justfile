
_default: clippy

# _list:
#     just --list

all: clippy test doc

clippy:
    cargo clippy --all-targets --all-features
    cargo clippy --all-targets

test: clippy
    cargo test --all-targets --all-features
    cargo test --all-features --doc
    cargo test --all-targets

fmt:
    cargo fmt

doc:
    RUSTDOCFLAGS="--cfg docsrs" CARGO_TARGET_DIR=target-nightly cargo +nightly doc --all-features

doc-open:
    RUSTDOCFLAGS="--cfg docsrs" CARGO_TARGET_DIR=target-nightly cargo +nightly doc --all-features --document-private-items --open