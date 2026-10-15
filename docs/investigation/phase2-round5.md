# Phase 2, round 5: round-4 packages and trivial-function waves

_Started 2026-10-15 by the orchestrating agent, taking over from the previous
session's briefing. Companion docs: [`phase2-candidates-3.md`](phase2-candidates-3.md)
(the round-4 package lists), [`phase2-worker-guide.md`](phase2-worker-guide.md)
(how a worker rewrites one function), [`phase2-overrides.md`](phase2-overrides.md)
(the replacement contract and integration steps)._

## State found (2026-10-15 ~18:20)

Contrary to the hand-off note, the uncommitted modules in
`crates/ssx3-game/src/` are not empty of overrides: `snd_lib.rs` (12),
`audio_system.rs` (25), `rider_logic.rs` (14) and `world_cache.rs` (17) all
list replacements, and the four OpenCode pilot modules `oc1.rs`–`oc4.rs` list
10 each. All of them compile under `--all-features` and (except `audio_system`)
carry one differential test per listed function.

`audio_system.rs` is the outlier: 25 `OVERRIDES`, only 5 tests — the 20 big
`SSXAudioSystem_*` / bank-monitor functions are implemented but unverified, so
the module is not integration-ready until they have tests.

`world_painter.rs`, `graphics.rs`, `streaming.rs` and `replay_script.rs`
(packages E3/E5/E6/E7) are still bare stubs (empty `OVERRIDES`).

## Trivial-function catalogue

`generated/ee/sNN/src/lib.rs` was re-classified (a throwaway script) into
`addr / shard / size / name / flags`. With calls detected correctly — including
calls emitted through the generated `pub use` aliases, e.g. a wrapper calling
`AI_ResetRiderRelations(c)` rather than `f_0010f3b8(c)` — the **trivial set**
(named, not yet overridden, no loop / no direct or indirect call / no syscall /
no VU0 / no `__entries`, 9..256 bytes) is **648 functions**. This matches the
hand-off's "about 470 trivial functions" once the ~180 already written are
excluded. A first pass that missed the alias calls over-counted (2234); the
alias case is why.

Waves of 20 are emitted as per-worker function lists; each worker gets its own
feature-gated module (`oc5.rs` … `oc12.rs`, features `oc5` … `oc12`, scaffolded
in commit-less edits to `lib.rs`/`Cargo.toml`).

## Round-5 work launched

Background subagents, one file each (never a shared file):

| worker | module / feature | work |
| --- | --- | --- |
| E3 | `world_painter` | 25 functions (FPU-heavy, some calls) |
| E5 | `graphics` | 25 functions (VU0, indirect calls; hardest) |
| E6 | `streaming` | 25 functions (file/stream, calls) |
| E7 | `replay_script` | 25 functions (replay/script, calls) |
| oc5 | `oc5` | trivial wave 1 |
| oc6 | `oc6` | trivial wave 2 |
| oc7 | `oc7` | trivial wave 3 |
| oc8 | `oc8` | trivial wave 4 |

Next batch (queued): `snd_lib` (+13 of E1), `rider_logic` (+11 of E4),
`world_cache` (+8 of E8), an `audio_system` completion (write the 20 missing
tests, fix or drop mismatches), and trivial waves 5–12.

## Integration (gated, done only after review)

Per [`phase2-overrides.md`](phase2-overrides.md): add the reviewed addresses to
`crates/ssx3-game/overrides.txt`, enable the features in `default`, rerun
`recomp-ee`, build `ssx3`, run
`cargo test --profile game -p ssx3-game --all-features`, then the scripted race
replay with and without overrides (`SSX3_NO_OVERRIDES=1`, `SSX3_RAM_DUMP=…`),
screenshots and the RAM dump byte-identical. `recomp-ee` is not run while
workers are still reading `generated/`.
