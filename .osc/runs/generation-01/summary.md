# Generation 01 Summary

## Result

- Determinism: pass.
- Mechanical ACs: 16/16.
- Score JSON: `.osc/runs/generation-01/conformance.json`.

## Implementation

- Added a Rust `driver` binary that implements `2000m.driver.v0`.
- Added deterministic seeded obstacle lanes with trees/stumps/rocks, ramps, and moguls.
- Added fixed-tick steering, horizontal wrap, acceleration/boost caps, crash halt/recovery, airborne landing style, and post-2000m monster chase/eat/flee behavior.

## Verification

```bash
cargo fmt --check
cargo build
cargo test
printf '%s\n' '{"cmd":"init","seed":42,"config":{}}' '{"cmd":"step","input":{"steer":1,"boost":false,"jump":false}}' '{"cmd":"state"}' '{"cmd":"reset","seed":42}' | cargo run --quiet --bin driver
CARGO_TARGET_DIR=/tmp/2000m-judge-target cargo run -q --manifest-path <local-2000m-judge-repo>/Cargo.toml -p m2000-conformance -- <local-produced-game-repo> --json-out <local-produced-game-repo>/.osc/runs/generation-01/conformance.json
```

The first direct `cargo build`/`cargo test` attempt before `Cargo.lock` existed failed because the sandbox could not resolve `index.crates.io`; after generating the lockfile from the local Cargo cache with `CARGO_NET_OFFLINE=true cargo build`, normal `cargo build` and `cargo test` passed.
