# Open Scaffold Prompt: Group D

Run ID: 20260530T213403Z-001-codex-gpt55-2000m-run-run
Task ID: (none supplied)
Plan: 001-codex-gpt55-2000m-run
Goal: Produce and score Codex/GPT-5.5's first 2000m benchmark trajectory across up to eight genuine Codex generations, then prepare reviewable game and results branches without merging them.

## Assignment
final publication prep after the last scored generation.

## Execution lane
- Executor: omx-codex
- Harness skill: $ultrawork
- Repository: <local-produced-game-repo>
- Operator surface: discord

## Rules
- Follow the plan and its amendments; do not silently expand scope.
- If scope changes, propose an amendment instead of editing the original plan.
- Produce evidence tied to acceptance criteria, not vibes.
- This generic prompt does not spawn agents; paste it into your selected runtime.
- Treat chat threads as operator-surface bindings, not canonical task/run identity.
- If blocking open questions exist, stop and clarify before implementation.

## Acceptance criteria
- A genuine Codex/GPT-5.5 run produces each attempted generation; prompts/logs are preserved.
- Each generation is scored with `cargo run -p m2000-conformance -- <game-dir> --json-out <path>` from `graphanov/2000m`.
- The trajectory records generation number, commit/hash or worktree state, pass count, determinism verdict, and score JSON path.
- The final game branch builds locally or its failure is honestly recorded as the final state.
- `graphanov/2000m-results` receives `models/codex-gpt5.5/trajectory.json`, `final-conformance.json`, `summary.md`, and regenerated `leaderboard.md` on a review branch.
- Human-feel verdict remains a placeholder pending the owner's playtest and is not blended into the mechanical rank.

## Verification steps
1. `cargo build` from this repo after each generation that creates Rust code.
2. `cargo run -q -p m2000-conformance -- <this-repo-or-driver-dir> --json-out .osc/runs/generation-N/conformance.json` from `<local-2000m-judge-repo>` for every generation.
3. `npx --yes open-scaffold@latest evolve check .osc/evolve/codex-gpt5.5-2000m` from this repo after recording attempts.
4. `python3 scripts/validate_results.py && python3 scripts/build_leaderboard.py --check` from `<local-2000m-results-repo>` after adding the results folder.
5. GitHub PR/check links show review branches are pushed; no merge is performed without owner approval.

## Blocking open questions
- None blocking.
