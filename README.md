# 2000m Codex/GPT-5.5

This repository is the inaugural **produced-game** run for [`graphanov/2000m`](https://github.com/graphanov/2000m), a neutral Rust SkiFree-reproduction benchmark.

The architecture is intentionally split:

- [`graphanov/2000m`](https://github.com/graphanov/2000m) is the neutral judge: protocol + conformance suite + stub.
- [`graphanov/2000m-results`](https://github.com/graphanov/2000m-results) is the scalable leaderboard spine: one light folder per model.
- This repo is the optional showcase: Codex/GPT-5.5's produced Rust game plus the Open Scaffold work record used to build and score it.

## Benchmark contract

The game must implement `2000m.driver.v0` from the judge repo:

- JSON-line stdin/stdout driver process.
- Fixed-tick deterministic simulation.
- Rust implementation only.
- No copyrighted SkiFree assets.
- No scenario hints, shortcut config, or benchmark-specific flags.

Mechanical rank comes only from the 16 acceptance criteria scored by the judge. the owner's human-feel verdict is a separate note and is never blended into the rank.

## Current status

This repository starts as an Open Scaffold work record. Codex/GPT-5.5 generations and score artifacts will land on the benchmark run branch before the owner's merge gate.
