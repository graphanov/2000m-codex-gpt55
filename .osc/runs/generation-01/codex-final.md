Implemented generation 1 and left it in the worktree.

Changed/created:
- [Cargo.toml](<local-produced-game-repo>/Cargo.toml), [Cargo.lock](<local-produced-game-repo>/Cargo.lock)
- [2000m.json](<local-produced-game-repo>/2000m.json)
- [src/main.rs](<local-produced-game-repo>/src/main.rs), [src/sim.rs](<local-produced-game-repo>/src/sim.rs)
- [README.md](<local-produced-game-repo>/README.md)
- generation evidence under [.osc/runs/generation-01](<local-produced-game-repo>/.osc/runs/generation-01/summary.md) and [.osc/evolve/codex-gpt5.5-2000m](<local-produced-game-repo>/.osc/evolve/codex-gpt5.5-2000m/frontier.json)

Verification:
- `cargo fmt --check`: passed
- `cargo build --locked`: passed
- `cargo test --locked`: passed, 3 tests
- protocol smoke via `cargo run --quiet --bin driver`: passed
- `./verify.sh`: passed, 6 pass / 0 fail / 0 warn
- Official scorer: ran read-only using `--manifest-path <local-2000m-judge-repo>/Cargo.toml` and `CARGO_TARGET_DIR=/tmp/2000m-judge-target`; result is determinism pass and 16/16 ACs. JSON written to [.osc/runs/generation-01/conformance.json](<local-produced-game-repo>/.osc/runs/generation-01/conformance.json).

Against the 16 ACs: all pass in the official conformance run. No known mechanical gaps. I did not commit, push, merge, or modify `<local-2000m-judge-repo>` or `<local-2000m-results-repo>`.