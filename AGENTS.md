# AGENTS.md

## Running cargo

Type cargo commands as usual. The `cargo` plugin rewrites each one to run
inside the api dev container, so they use the pinned toolchain and the
installed components.

```
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -Dwarnings
cargo test --all-features
```

Run the commands from `apps/api`, or pass `--manifest-path apps/api/Cargo.toml`.

## Quality metrics

`docs/harness.md` is the source of truth for thresholds and tools.
`.github/workflows/quality.yml` runs the full check on every push and pull
request.
