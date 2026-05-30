# Plan: 001-codex-gpt55-2000m-run

## Status

active

## Context

`graphanov/2000m` is live as the neutral benchmark judge, and `graphanov/2000m-results` is live as the scalable leaderboard spine. This produced-game repository is the inaugural showcase run: Codex/GPT-5.5 must build a Rust SkiFree-inspired implementation through an Open Scaffold work record, while Hermes only supervises and scores.

## Goal

Produce and score Codex/GPT-5.5's first 2000m benchmark trajectory across up to eight genuine Codex generations, then prepare reviewable game and results branches without merging them.

## Constraints / Out of scope

- Rust only; no JavaScript/Python game implementation as the scored artifact.
- No copyrighted SkiFree assets; original code/assets only.
- No scenario hints, shortcut config, or benchmark-specific flags.
- Hermes must not hand-write game fixes; Codex/GPT-5.5 owns each generation's implementation attempt.
- The neutral judge repo must not receive Open Scaffold files or produced-game code.
- Stop before repository merge, release, npm publish, or the owner's human-feel/taste verdict.

## Files to touch

- `Cargo.toml` — Rust workspace for the produced game.
- `driver/` — JSON-line protocol driver binary implementing `2000m.driver.v0`.
- `game/` or equivalent Rust modules — deterministic SkiFree-inspired simulation state and mechanics.
- `.osc/evolve/codex-gpt5.5-2000m/` — evolve loop ledger for generation attempts.
- `.osc/runs/generation-*/` — Codex prompts/logs and local scoring evidence.
- `README.md` — public status and play/build instructions.
- `MISSION.md` / `.osc/releases/*` — evidence notes and closeout material.

## Execution strategy

### Task decomposition

| ID | Task | Dependencies | Parallel group |
|----|------|--------------|----------------|
| T1 | Initialize Open Scaffold evolve loop from this plan | None | A |
| T2 | Run Codex/GPT-5.5 generation 1 from the fixed benchmark brief | T1 | B |
| T3 | Score generation output with `graphanov/2000m` conformance suite | T2 | C |
| T4 | Feed score/evidence into the next Codex generation prompt | T3 | B/C repeated |
| T5 | Record each attempt into the evolve ledger | T3 | C repeated |
| T6 | Prepare final game PR branch and results repo branch | T2-T5 | D |

### Parallel groups

- **Group A**: local setup and loop initialization.
- **Group B/C**: generation and scoring loop; each generation depends on the previous generation's scored output.
- **Group D**: final publication prep after the last scored generation.

### Dependencies

- Later generations must inherit the actual previous code and score evidence; no reset unless a generation is recorded as failed and restarted from the latest committed branch state.
- Results repo updates depend on real conformance JSON produced by the neutral judge.

### Delegation notes

- Codex/GPT-5.5 is the implementation worker via Codex CLI or OMX.
- Hermes is the supervisor and verifier: it scores, records, and refuses to invent or repair game code manually.

## Implementation Architecture Coverage

- Strengthens: benchmark reproducibility, work-record evidence, model-comparison traceability.
- Audit envelope: `.osc/evolve/codex-gpt5.5-2000m/`, `.osc/runs/generation-*/`, conformance JSON, Codex logs, and results repo branch.
- Evaluation envelope: neutral `graphanov/2000m` conformance scorer, deterministic pass/fail output, leaderboard generator validation.
- Feedback routing: build/test/score failures are recorded as generation evidence; the owner handles merge/human-feel gates.
- Boundary: no benchmark judge mutation, no hidden hand-authored game fixes, no general LLM benchmark claim.

## Acceptance criteria

- [ ] A genuine Codex/GPT-5.5 run produces each attempted generation; prompts/logs are preserved.
- [ ] Each generation is scored with `cargo run -p m2000-conformance -- <game-dir> --json-out <path>` from `graphanov/2000m`.
- [ ] The trajectory records generation number, commit/hash or worktree state, pass count, determinism verdict, and score JSON path.
- [ ] The final game branch builds locally or its failure is honestly recorded as the final state.
- [ ] `graphanov/2000m-results` receives `models/codex-gpt5.5/trajectory.json`, `final-conformance.json`, `summary.md`, and regenerated `leaderboard.md` on a review branch.
- [ ] Human-feel verdict remains a placeholder pending the owner's playtest and is not blended into the mechanical rank.

## Verification steps

1. `cargo build` from this repo after each generation that creates Rust code.
2. `cargo run -q -p m2000-conformance -- <this-repo-or-driver-dir> --json-out .osc/runs/generation-N/conformance.json` from `<local-2000m-judge-repo>` for every generation.
3. `npx --yes open-scaffold@latest evolve check .osc/evolve/codex-gpt5.5-2000m` from this repo after recording attempts.
4. `python3 scripts/validate_results.py && python3 scripts/build_leaderboard.py --check` from `<local-2000m-results-repo>` after adding the results folder.
5. GitHub PR/check links show review branches are pushed; no merge is performed without owner approval.

## Open questions

- The owner will provide the separate human-feel verdict after a playable WASM/browser build exists.
- If Codex reaches 16/16 before generation 8, stop early only after recording the full winning evidence and explaining why additional generations would be leaderboard noise.
