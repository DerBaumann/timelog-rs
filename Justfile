export TIMELOG_STOREPATH := env("TIMELOG_STOREPATH", "./store.json")

# TODO: Maybe add short aliases

default: build

# Runs the program
run *args:
    cargo run -- {{ args }}

# Runs the program in release mode
run-release *args:
    cargo run --release -- {{ args }}

# Builds the release binary
build:
    cargo build --release

# Fast compile check
check:
    cargo check

# Lint
lint:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Run tests
test *args:
    cargo test {{ args }}
