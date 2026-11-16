# Development

## Reproducible generation

Generation requires Python 3, jq, and the pinned Rust toolchain:

```sh
cargo install cargo-typify --version 0.8.0 --locked
python3 scripts/sync-schemas.py --fetch
bash scripts/generate-opent8.sh
```

`schema/sources.json` records upstream commits, paths, and SHA-256 checksums.
Sync verifies files before replacing them; omit `--fetch` to check local files.
When adding a version, update its provenance, generation script, module, and tests.

The `licenses` module includes attribution and licence texts in rustdoc. Keep
that page when distributing the docs.

Generator adaptations affect temporary input only; vendored schemas stay unchanged.

## Rust support

Rust **1.88** is the minimum supported version. CI tests it and stable Rust on
Linux. Update `Cargo.toml` and CI together when raising it.

`rust-toolchain.toml` pins the tools used for development and code generation.

## Release checks

```sh
cargo fmt --check
cargo test --locked
cargo +1.88.0 test --locked
cargo clippy --locked --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps
bash scripts/generate-opent8.sh --check
cargo package --list
cargo publish --dry-run
```

Before the first release, verify the crates.io name is available and review the
package contents.
