
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
    cargo doc --all-features

doc-open:
    cargo doc --all-features --document-private-items --open