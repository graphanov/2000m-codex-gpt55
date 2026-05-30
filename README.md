# 2000m Codex/GPT-5.5

This repository is the inaugural **produced-game** run for [`graphanov/2000m`](https://github.com/graphanov/2000m), a neutral Rust SkiFree-reproduction benchmark.

The architecture is intentionally split:

- [`graphanov/2000m`](https://github.com/graphanov/2000m) is the neutral judge: protocol + conformance suite + stub.
- [`graphanov/2000m-results`](https://github.com/graphanov/2000m-results) is the scalable leaderboard spine: one light folder per model.
- This repo is the optional showcase: Codex/GPT-5.5's produced Rust game plus the Open Scaffold work record used to build and score it.

## Benchmark contract

The game implements `2000m.driver.v0` from the judge repo:

- JSON-line stdin/stdout driver process.
- Fixed-tick deterministic simulation.
- Rust implementation only.
- No copyrighted SkiFree assets.
- No scenario hints, shortcut config, or benchmark-specific flags.

Mechanical rank comes only from the 16 acceptance criteria scored by the judge. The owner's human-feel verdict is a separate note and is never blended into the rank.

## Current status

Generation 1 includes a Rust headless driver and deterministic SkiFree-inspired simulation. Hermes independently ran the neutral conformance suite from `graphanov/2000m` and recorded:

- Determinism: PASS.
- Mechanical ACs: **16/16**.
- Evidence: `.osc/runs/generation-01/conformance.json` and `.osc/runs/generation-01/conformance.txt`.

The agreed cap was 8 generations, but the run stopped after generation 1 because the mechanical ceiling was already reached.

## Running the driver

```bash
cargo build
cargo test
printf '%s\n' \
  '{"cmd":"init","seed":42,"config":{}}' \
  '{"cmd":"step","input":{"steer":1,"boost":false,"jump":false}}' \
  '{"cmd":"state"}' \
  '{"cmd":"reset","seed":42}' \
  | cargo run --quiet --bin driver
```

The produced-game manifest is `2000m.json`; the scorer launches `cargo run --quiet --bin driver` from this repo root and exchanges JSON lines over stdin/stdout.

## Scoring

From a local checkout of the judge repo:

```bash
cd /path/to/2000m
cargo run -q -p m2000-conformance -- /path/to/2000m-codex-gpt55 --json-out /tmp/codex-gpt55-2000m.json
```

Human-feel playtest remains pending and separate from the mechanical rank.
