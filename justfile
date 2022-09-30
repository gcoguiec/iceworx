_default:
  @just --list

# Lint the code.
lint:
  cargo check --workspace --all-targets
  cargo clippy --workspace --all-targets
  dprint check
  cspell "**/*.{rs,json,toml,md,yml}"

# Check code format.
fmt:
  cargo fmt
  dprint fmt

# Run test suites.
test:
  cargo nextest run --all-targets

# Run test suites and generate HTML coverage report (grcov).
grcov:
  rm -rf .coverage/*.profraw
  RUSTFLAGS="-C instrument-coverage -C link-dead-code" \
    LLVM_PROFILE_FILE=".coverage/makurosu-%p-%m.profraw" cargo test --all-targets
  grcov . -s . --ignore "/*" --ignore "**/*_test.rs" --ignore "tests/*" --ignore "target/*" \
    -b ./target/debug/ --branch --llvm -t html -o ./target/debug/coverage/ \
    --ignore-not-existing \
    --excl-br-line "#\[derive\(" --excl-line "#\[derive\("
  open target/debug/coverage/index.html

# Run test suites and generate HTML coverage report (llvm-cov).
llvm-cov:
  cargo llvm-cov --html --open --ignore-filename-regex _test.rs
