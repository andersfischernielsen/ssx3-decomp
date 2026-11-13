# Translated VU microcode performance (2026-10-14)

With the GPU GS (`SSX3_GS=gpu`), races ran at about 20–24 fps headless and
the emulation thread was dominated by VU1 microcode. This note covers the
profile, what changed in the translator and runtime, how bit-exactness was
checked, and the results.

**Result:** in-race frame rate went from **23.8 to 53.7 fps** (frames
4320–4680 of the scripted race: 15.1 s to 6.7 s). The run to frame 4200
went from 65.1 s to 38.0 s. Translated VU code alone (capture replay) is
**3.5× faster**: 31.5 to 8.9 ns per instruction pair. All 47 screenshots of
the run (frames 100–4700) are byte-identical before and after, and every
captured VU invocation replays to an identical full-state digest.

## Measuring

```sh
SSX3_RTC=1792000000 SSX3_GS=gpu \
SSX3_INPUT="1600:0008:8,2000:4000:8,2500:4000:8,2700:4000:8,2900:4000:8,3100:4000:8,3300:4000:8,3500:4000:8,3700:4000:8,4250:4000:8" \
  ./target/game/ssx3-headless 4700 --shot-every 100
```

`SSX3_RTC` pins the clock. Without it the rider line-up and the camera depend
on the wall clock and runs are not comparable. The race starts at about frame
4250. The status line is printed every 60 frames, and the time between the
4320 and 4680 lines is the race figure. Apple M1, other jobs running (load
about 3–4).

| | to frame 4200 | frames 4320–4680 | race fps |
| --- | ---: | ---: | ---: |
| before (d13ea55) | 65.1 s | 15.1 s | 23.8 |
| translator v1 + NEON FMAC | 39.5 s | 7.1 s | 50.7 |
| + macro-mode NEON, VIF, lookup, upload-hash memo (final) | 38.0 s | 6.7 s | 53.7 |

Each row is the same in two runs. Runs of the old and new binaries were
interleaved.

**VU-only benchmark.** `vu_diff bench <capture> [iters]` replays every record
of a capture through the translated code. Code hashing is done outside the
timed region. Race capture `race3` has 3000 records and 2.09 M pairs:

| step | ns/pair |
| --- | ---: |
| before | 31.5 |
| translate-time pipeline analysis, lazy Q/P/flags, locals instead of staging | 17.8 |
| branch-free scalar FMAC, force-inlined | 22.4 (slower: too much code) |
| four-lane NEON FMAC | 11.4 |
| deferred MAC/status commit | 10.5 |
| known-normal operands skip `norm`, NEON ITOF/FTOI | 9.2 |
| SHRN product narrowing, no redundant shift mask | 8.9 |

## Profile (macOS `sample`, race frames)

**Before.** All guest threads are serialized by the baton, so wall time is
the sum over threads. Out of 7323 samples (12 s):

- **About 65% is VU exec.** These samples sit in the DMA thread (a guest
  thread that kicks VIF1 DMA). They break down as
  `vu::float::add`/`madd`, `Vu::stage_fmac` (closures per lane),
  `commit_stage`, `advance` (pipelines retired every pair) and the
  generated `run`.
- **About 15% is the GS.** This is `gif_write` from XGKICK and DIRECT.
- **About 9% is the EE main thread.** It includes 12% of its own time in
  `getenv`: `machine::call_indirect` read `SSX3_TRACE_MIDCALL` on every
  indirect call. About 40% of the EE thread was VU0 macro mode (COP2),
  which went through the same slow staging path.

Program `9fa47842`, which draws the riders, was 75% of VU time. One 31-pair
block (`0x0628`, mostly MADDA/MADD chains) runs 1.4% of all pairs per pair
address.

**After.** Out of 3473 samples (8 s):

- **About 38% is VU exec.** `9fa47842` itself is 1126 samples.
- **About 28% is GS** (`gif_write`, `GpuGs::push`, `Region::add_rect`,
  `Pages::rect`, `raster::setup`).
- **About 9% is the rest of VIF and DMA.**
- **About 14% is the EE thread.** Macro-mode VU0 is now small there.
- **About 7% is the main thread** (vblank, GPU submit).

At 53.7 fps a frame takes about 18.6 ms. Reaching 60 fps needs about 2 ms
less per frame. Even with VU time at zero, the GS CPU side plus EE plus VIF
would come to about 11.5 ms. Further gains are split between the GS CPU side
(per-primitive page/region bookkeeping, about 5–7 µs per frame per ns per
primitive at about 50k primitives per frame) and the FMAC arithmetic itself.

## What changed

### Translator (`vu/recomp.rs`) and fast-path helpers (`vu/fast.rs`)

Each translated pair used to call `begin_pair` with a slice of reads, a
staging upper method, the lower method and `end_pair`. Every pair advanced
the Q, P, flag and clip pipelines, cleared the stage and moved the VI
backup. That gave the exact PCSX2-style timing, but almost all of it is
unobservable most of the time. The translator now builds a pair-level
control-flow graph and decides per pair. The graph is the union of every
way a pair can be left: fall-through, branch target when it runs as a delay
slot, End after an E-bit delay slot, Dyn for JR/JALR or leaving the segment.

- **Stall checks.** A pair only checks VF fields written by one of the 3
  preceding pairs along static edges (backward window over predecessors). A
  field written 4 or more pairs back is always ready. Writes are committed
  at cycle c with ready time c+4, and every pair takes at least one cycle.
- **Dynamic entries** are program start, budget resume, JR and cross-program
  jumps. They enter through a second `match` of single-pair arms that check
  every read. `slow = 3` keeps the first 3 pairs on that path. Fast blocks
  are entered only through static control flow, so their predecessors are
  known.
- **Ready times.** `vf_ready` is written only when a read of that field can
  occur within 3 pairs. `ready_max`, which the end-of-program flush uses for
  the cycle count, is written only when End or Dyn is within 3 pairs. A
  skipped ready time leaves an older, smaller value, and any read that could
  see the difference is more than 3 pairs away, so it would not stall anyway.
- **Q and P** are retired only right before an op that observes them:
  `q_sync` before an upper op that reads Q and before FS* and FSSET, and
  `p_sync` before MFP. Retiring later at the same check gives the same
  value. The DIV status bits commute with the FMAC status bits, and FSSET
  syncs first.
- **Flags.** An FMAC or CLIP op queues a snapshot (`flags_push`) only if a
  flag op (FM*, FS*, FC*, including FSSET and FCSET) or a Dyn exit is within
  3 pairs. Otherwise its flags are made visible at once (`flags_eager`),
  which also drops older queued snapshots, because no observer can tell the
  difference. Queued snapshots are retired lazily before flag ops
  (`flags_sync`) and when pushing.
- **Deferred MAC/status.** In a run of eager FMAC ops, generated code keeps
  `fl` (the last MAC) and `fa` (the OR of the MACs). The sticky status bits
  are an OR of the per-op nibble bits, and nibble(a|b) = nibble(a)|nibble(b).
  `mac_commit(fl, fa)` and `flags_eager` run once, before the next flag op,
  CLIP, queued snapshot or arm end.
- **Upper results go to locals.** The lower op runs against the old state,
  then the result is written. The "upper wins over a lower write to the same
  VF" rule becomes a translate-time drop of the lower write. No `Stage`
  struct is used.
- **VI backup** (`vi_bk`, `vi_brf`). The backed-up register carries the
  index of the pair that wrote it (`pb + k`, where `pb` is `vu.pairs` at arm
  start). This replaces clearing it every pair. A stall clears it, as in
  `begin_pair`.
- **Known-normal operands.** Within an arm, the translator tracks VF fields
  and ACC lanes that hold FMAC or ITOF results. These are never denormal or
  exponent 255, so the denormal/±max normalisation of those operands is
  skipped (`madd4::<D, NA, NS, NT>`). The product inside MADD/MSUB is never
  re-normalised.
- **Counters.** Budget and `pairs` are updated once per arm. Per-pair
  `cycle += 1` lines are merged into one add before the next line that uses
  the cycle counter.
- **Cold code.** Single-pair (dynamic-entry) arms call `#[inline(never)]`
  copies (`fast::cold`) to keep the generated functions small.

### Arithmetic (`vu/float.rs`, `vu/simd.rs`)

- `float::add` and `float::mul` are now branch-free: both paths are computed
  and the result is selected. They implement the same integer algorithm (one
  guard bit, truncation, clamp/flush, O/U). The original ports are kept as
  `add_ref` and `mul_ref`.
- `simd.rs` is the same algorithm on four lanes with AArch64 NEON:
  - `vmull` for the 48-bit products, narrowed with `shrn`.
  - `clz` for renormalisation.
  - Masks for the zero, cancellation, overflow and underflow cases.
  - MAC flags of the dest lanes, assembled with one horizontal add.
- ITOF and FTOI also have NEON versions. Other targets use the scalar code.
- `Vu::add/sub/mul/madd/msub(+A)`, used by macro mode and the reference
  interpreter, now use the same four-lane functions.

### Other hot-path fixes found in the profile

| fix | file |
| --- | --- |
| `getenv` per `call_indirect` / `jump_out` / GS transfer start → `OnceLock` | `machine.rs`, `gs/transfer.rs` |
| `GpuGs::push`: per-primitive `Vec` (alias check) and 2 KiB `Region` (texture RAW check) → array, reused scratch | `gs/gpu/mod.rs` |
| VIF UNPACK: no byte copy of the payload; unmasked/plain-mode unpacks decode through a per-format monomorphised loop | `vif/mod.rs` |
| `Vu::lookup`: one-entry cache before the SipHash registry map | `vu/mod.rs` |
| Upload hash memo: the game re-uploads the same microcode every frame; FNV over up to 16 KiB per MSCAL was 2% of the race; recently hashed uploads are recognised with `memcmp` | `vu/mod.rs` |

## Exactness checks

1. **Capture digests** (`vu_diff digest <capture> [--ref-model]`). Each
   record is replayed through the translated code and one FNV hash is
   printed per record. The hash covers:
   - the XGKICK packets
   - all VF, VI and ACC registers
   - Q, P, I, R
   - MAC, status and clip, plus their visible copies
   - TPC
   - data memory
   - the pair count, the cycle count, the error count and the running flag

   Digests were produced with the original code (d13ea55 semantics) and
   compared after every step, in both the timing model and `--ref-model`:

   | capture | records |
   | --- | ---: |
   | `race` | 500 |
   | `race2` | 1720 |
   | `race3` | 3000 |
   | `menu` | 1500 |
   | `charsel` | 300 |
   | `synth` (sane random states, every entry point) | 444 |
   | `synthr` (unrestricted random states) | 592 |

   **All terminating runs are identical.** The 404 synthetic runs that never
   end (budget-limited) stop at a different point, because budget is charged
   per arm and dynamic entries now take single-pair arms. They are skipped.
   The checker caught one real bug during development: a queued snapshot
   after deferred eager ops hid those ops from a flag op inside the queued
   op's latency. That bug never reached a commit.
2. **`vu_diff self`** also compares the in-game cycle count of each record.
   race, race3, menu and charsel give 0 differences. `race2` predates the
   MAX/MINI fix.
3. **`cargo test -p ssx3-ps2`.** All tests pass, including these:
   - The oracle compares translated code against the reference interpreter
     on all 7 programs, every entry point and random states.
   - `float::branch_free_matches_reference`, with configurable iterations
     (50 M pairs were run).
   - `simd::neon_matches_scalar`. It covers add, mul, madd, msub, the
     skip-`norm` variants, ITOF/FTOI with every fraction, and MAC (5–10 M
     vectors per run).
   - `vif::unpack_plain_path_matches_general`.
4. **End to end.** Screenshots at every 100 frames of the scripted run
   (frames 100–4700) are byte-identical to the original build.

## Tools added

- `vu_diff digest <cap> [--ref-model]`: full-state hash per record.
- `vu_diff bench <cap> [iters]`: VU-only replay throughput.
- `vu_diff hot <cap> [n]`: hottest pair addresses, from the reference tracer.
- `vu_diff self` now also checks cycle counts.

## Notes and limits

- **Budget exits** leave lazily retired state (Q, P, visible flags)
  un-retired until the program resumes. This matters only if the EE reads VU
  control registers while a program is still running after a budget expiry.
  That does not happen in SSX 3, where `MSCAL` runs to completion with a
  2 M-pair budget.
- **Generated code** is about 5.5 MB (the slow arms are compact). A release
  build of `ssx3-ps2` takes about 3 minutes, because the force-inlined NEON
  code sits in large functions.
- **Debug builds.** The helpers are `inline(always)` only when
  `debug_assertions` is off. Force-inlined into the translated functions,
  they gave a debug frame of more than 5 MB, which overflowed the 2 MB test
  threads.
- **Bootstrap.** When the library's helper signatures change, the old
  `generated/vu` may not compile against it. Build `vu_recomp` with an empty
  registry (`pub static PROGRAMS: &[ProgramEntry] = &[];` in
  `generated/vu/mod.rs`), then run it.
- **Where the time goes now.** Remaining VU time is the FMAC arithmetic
  itself: about 100 NEON instructions per MADD pair, including flags. The
  next big items are the GS CPU side and VU-parallel execution. Running VU1
  concurrently with the EE would need asynchronous VIF1 and a determinism
  story.
