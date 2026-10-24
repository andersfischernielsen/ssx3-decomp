# Development and verification

## Build and regenerate

```sh
tools/bootstrap.sh                                   # fresh checkout: translate EE, VU, IOP into generated/
./target/release/recomp-ee                           # regenerate generated/ee (host-thread form)
./target/release/recomp-ee --unwind --out generated/ee-unwind --package-prefix ssx3-ee-unwind   # browser form
cargo build --profile game -p ssx3                   # window + headless
CARGO_TARGET_DIR=target-unwind cargo build --profile game -p ssx3 --bin ssx3-headless \
  --no-default-features --features ee-unwind         # the browser's code form, natively
```

Regenerate after editing `crates/ssx3-game/overrides.txt` (a new override
slot) or the recompiler. A change to `ssx3-ee-rt` recompiles all generated
crates (`investigation/crate-split.md`).

**Machine limits (8 GB):** one heavy process at a time: a build (`-j 4`) or
one headless run, never two runs or a run during a build.

## Headless runs

`target/game/ssx3-headless FRAMES [--shot-every N]` runs without a window,
deterministically (fixed RTC). Common variables:

| Variable | Effect |
| --- | --- |
| `SSX3_INPUT="frame:buttons:frames,..."` | Scripted pad input (libpad bits: Select 0x1, Start 0x8, Up 0x10, Right 0x20, Down 0x40, Left 0x80, L2 0x100, R2 0x200, L1 0x400, R1 0x800, Triangle 0x1000, Circle 0x2000, Cross 0x4000, Square 0x8000) |
| `SSX3_SHOTS=dir` | Screenshots (with `--shot-every`) |
| `SSX3_RAM_DUMP=file` | The 32 MiB of EE RAM at exit, for A/B comparisons |
| `SSX3_SKIP_MOVIES=0` | Play the boot movies (headless skips them by default) |
| `SSX3_DEBUG_LENSFX_VIS=1` | The generated light-visibility query instead of the native one (deterministic everywhere) |

The reference race: `tools/scenarios.py input p1_race_snow_jam`, or the
classic `1600:0008:8,2000:4000:8,2500:4000:8,2700:4000:8,2900:4000:8,3100:4000:8,3300:4000:8,3500:4000:8,3700:4000:8,4250:4000:8,4600:4000:3000`
to 6000 frames.

## Verifying hand-written code

- **Differential tests** per function (`cargo test --profile game -p ssx3-game --lib -- <module>::`):
  the port against its generated body from random states
  (`investigation/phase2-worker-guide.md`).
- **Replay gate:** the race replay's RAM must stay identical
  (`tools/integrate_wave.py gate`).
- **Scenario harness:** `scripts/scenarios.sh OUT LIST` runs every scenario
  of `tools/scenarios.py` (all races and freestyle events of the three peaks
  with boost and tricks, random menu input, ...) with the ports in `LIST`
  installed and skipped, and compares the RAM; `tools/bisect_overrides.py`
  finds the port behind a difference.
- **`SSX3_DIFF_STRICT=args`** (unit tests): also compare the arguments of
  every call the function makes (the `calltrace` feature records them);
  `SSX3_DIFF_STRICT=1`: also every scratch register and the call sequence.
- **`SSX3_SHADOW=addr,...|all`:** each call of the listed replacements runs
  the hand-written form and the generated body from the same state and
  reports the first differing registers, cycles or memory
  (`SSX3_SHADOW_FIRST`, `SSX3_SHADOW_EVERY` for sampling).
- **`SSX3_SKIP_OVERRIDES=addr,...`**, `SSX3_OVERRIDES=addr,...` (only
  these), `SSX3_NO_OVERRIDES=1` (none; this also drops the native draw
  functions, so RAM differs by design).
- **Draw functions:** `scripts/draw-check.sh` (`SSX3_DRAW_CHECK=1`) compares
  each native draw function with its generated body per call.

## Porting workflow

`tools/pick_wave.py` (hot functions from a `callprof` profile),
`tools/refsrc.py` / `refbody.py` / `refdossier2.py` (the matching C++
decompilation, `~/Desktop/devvydont-ssx3`, by address),
`tools/integrate_wave.py` (wire, regenerate, test, build, gate). Profile:
`CARGO_TARGET_DIR=target-prof cargo build --profile game -p ssx3 --bin ssx3-headless --features callprof`,
then `SSX3_CALLPROF=edges.tsv:5000:5300`.

## Finding your way through the game

`scripts/explore.sh OUT "INPUT" FRAMES [FIRST [EVERY]]` takes screenshots
and a contact sheet (`OUT.png`) to map menus and timings.

## Verification builds

The default build has no GS emulation. The emulated GS (`SSX3_RENDER=emu` /
`shadow`, GS captures, VRAM dumps, `gs_replay`, the GS and VIF unit tests)
needs `--features vu1-microcode` (which brings `emu-gs`):
`cargo build --profile game -p ssx3 --features vu1-microcode`,
`cargo test -p ssx3-ps2 --features emu-gs`.

## Native-path switches

| Variable | Effect |
| --- | --- |
| `SSX3_PACKETS=1` | The generated draw functions (building packets) instead of the native ones |
| `SSX3_NATIVE_RENDER_FRAME=0` | The generated `RenderFrame` |
| `SSX3_NATIVE_KICK=0` | The guest DMA kick thread instead of the native one |
| `SSX3_NATIVE_MPEG=0` | libmpeg + the IPU model instead of the native movie decoder; `SSX3_MOVIE_HASH=1` prints a hash per picture |
| `SSX3_DEPTH_SYNC=0` | Light queries read the latest depth instead of the frame two back |
| `SSX3_PACKETLESS_SKIP=Name,...` | Keep these draw functions generated |

## Diagnostics

`SSX3_MMIO_PROFILE=1` (hardware registers the game touches),
`SSX3_FRAME_ISSUES=1`, `SSX3_FRAME_DUMP=n` (+ `gfx_replay`),
`SSX3_TRACE_THREADS`, `SSX3_TRACE_SYSCALLS`, `SSX3_TRACE_SIF`,
`SSX3_WAV=file` (audio). Runtime misses (untranslated code) are appended to
`generated/entries.txt`; rerun `recomp-ee`.

## What the disc contains

| File | Purpose |
| --- | --- |
| `SYSTEM.CNF` | Boot config: `BOOT2=cdrom0:\SLUS_207.72;1` |
| `SLUS_207.72` | The executable: ELF32, little-endian, MIPS R5900, stripped |
| `DATA/` | ~2.3 GB of level and asset archives |
| `NETGUI/` | ~8 MB network UI |

Image base `0x00100000`, entry point `0x00100008`, 93 segments; `.vutext`
(`0x0042e590`) holds the VU1 microcode. The ELF is stripped, but it was built
with `-ffunction-sections`, so its `.gnu.linkonce.t.*` section names still
carry mangled C++ symbol names.

## Analysis in Ghidra

Optional: the function inventory (`generated/functions.tsv`) is committed.

1. Import `orig/SLUS_207.72` as an ELF.
2. Set the language to `r5900:LE:32:default` from
   [ghidra-emotionengine-reloaded](https://github.com/chaoticgd/ghidra-emotionengine-reloaded)
   (plain MIPS does not decode EE instructions).
3. Run Analysis → Auto Analyze.
4. `tools/import-ghidra-names.py` imports the project's names; with the
   GhidraMCP plugin running, `tools/export-ghidra-functions.py` writes them
   back to `generated/functions.{json,tsv}`. See
   [`investigation/ghidra-mcp.md`](investigation/ghidra-mcp.md).
