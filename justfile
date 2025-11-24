check:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all-features
