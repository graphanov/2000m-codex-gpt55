# Mission

This repository is the inaugural produced-game showcase for the `2000m` benchmark: Codex/GPT-5.5 attempts to build a Rust SkiFree-inspired game through an Open Scaffold work record and is scored by the neutral `graphanov/2000m` conformance suite.

## What this repo is

- A **produced-game repository**: it combines the Open Scaffold evolve/work-record loop being measured with a Rust implementation of the `2000m.driver.v0` protocol being scored.
- A public, reproducible artifact of one model's benchmark attempt: generation prompts, run evidence, conformance results, and the final playable game code.
- The inaugural Codex/GPT-5.5 run for the `graphanov/2000m-results` leaderboard.

## What this repo is not

- Not the benchmark judge. The judge is `graphanov/2000m`, which stays neutral and contains no Open Scaffold apparatus.
- Not a general-intelligence benchmark. It measures how Codex/GPT-5.5 drives this Open Scaffold evolve loop on a Rust SkiFree-reproduction task.
- Not a copyrighted SkiFree clone. The implementation must use original code/assets and can only be a SkiFree-inspired homage.
- Not a place for hand-written Hermes game fixes. Codex/GPT-5.5 must produce the game generations; Hermes only scaffolds, supervises, scores, and records.

## Definition of done for this run

1. Up to 8 genuine Codex/GPT-5.5 generations have run against the same fixed `2000m` contract.
2. Every generation has a recorded prompt, run log, score JSON, and summary in this repo's Open Scaffold/evolve evidence.
3. The final generation builds and is scored by `graphanov/2000m` from a clean run.
4. `graphanov/2000m-results` contains `models/codex-gpt5.5/trajectory.json`, `final-conformance.json`, `summary.md`, and an updated generated leaderboard row.
5. Human-feel notes remain separate from the mechanical rank and await the owner's play verdict.

## Changelog

- 2026-05-30 — Created produced-game mission for the inaugural Codex/GPT-5.5 2000m benchmark run.

- 2026-05-30: closed 001-codex-gpt55-2000m-run — Codex GPT-5.5 reached 16/16 on generation 1; results review PR opened.
