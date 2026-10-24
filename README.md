# SSX 3, decompiled

[![SSX 3 running natively: title screen, mode select, race start and three moments of a Snow Jam race](docs/images/screenshots.jpg)](docs/ssx3.mp4)

▶ **[Video](docs/ssx3.mp4)**

> SSX 3 is © Electronic Arts; "SSX" is an EA trademark. This project is not affiliated
> with or endorsed by EA. **The repository contains no game data and no game code.** The
> disc image, the executable and everything translated from them (`orig/`, `generated/`)
> are produced locally from **your own copy** of the game, are ignored by git, and must
> never be committed or shared.

A native **Rust** port of **SSX 3 (USA)** (`SLUS_207.72`, PlayStation 2). The game's code
was statically recompiled to Rust and is being replaced, function by function, with
hand-written Rust. Rendering, movies, sound, files, controllers and the memory card are
native. No emulator runs the game, and no PS2 graphics, video or audio hardware is
emulated.

- **Fully playable**: every race and freestyle event, Conquer the Mountain, music, saves.
- **Native renderer** (wgpu): any resolution and aspect ratio, widescreen and ultrawide
  through the game's own projection, adjustable field of view, texture packs, optional
  modern graphics (AA, AO, bloom, draw distance).
- **Configurable game rate** (60/120/144/240 Hz), with no interpolation.
- **5,210 of 11,844 functions are hand-written Rust** (about 90% of a race's per-frame
  work), each checked against the recompiled original. Every function is named.
- **Runs on macOS and Windows**, builds on Linux, plus an experimental browser build ([`crates/ssx3-web`](crates/ssx3-web/README.md)).

The guest memory, the EE kernel's threads (coroutines on one host thread) and VU0 macro ops
are still PS2-shaped; see [`docs/plans/roadmap.md`](docs/plans/roadmap.md) and
[known limitations](docs/investigation/limitations.md).

![Snow Jam at 2560x1080](docs/images/ultrawide.jpg)
![Metro City at night at 1920x1080](docs/images/metro-city-night.jpg)

## Getting started

**You need:**
- **A disc image of SSX 3 (USA)** (`SLUS-20772`, Redump `SSX 3 (USA)`, about 3 GB) made
  from a disc you own. Other regions are not supported; setup checks the executable's
  SHA-256 (`1b49d05ca2793922180851b9e1ce9ae2291d61a7863565ac4e71f12e967af7bc`).
- **Rust** (stable, via [rustup](https://rustup.rs)) and **Python 3**.
- A GPU with Metal, Vulkan or DX12, and about 7 GB of disk.
- On Linux: `pkg-config`, `libasound2-dev`, `libudev-dev`. On Windows: the MSVC toolchain
  (not WSL).

Tested on macOS (Apple Silicon) and Windows. On Linux (Ubuntu 24.04) the setup, tests and
build work; playing there is untested.

```sh
git clone https://github.com/andersfischernielsen/ssx3-decomp.git
cd ssx3-decomp
cp /path/to/your/image.iso "SSX 3 (USA).iso"   # or set SSX3_ISO
python3 tools/setup.py                         # extract, check, recompile into generated/
cargo run --profile game -p ssx3
```

Run `setup.py` again after pulling changes to the recompiler. The first build compiles
about 55 MB of generated Rust; later builds are incremental. The binary is
`target/game/ssx3`.

**Standalone build.** `python3 tools/bundle.py` packs the game data into a build that runs
without the ISO or the repository: `dist/SSX3.app` on macOS, one executable on Linux and
Windows (`tools/bundle/Dockerfile` builds the Linux one anywhere). It contains the game: for
your own use only, do not share it.

## Playing

A controller is picked up automatically (buttons map by position: bottom ✕, right ○, left
□, top △).

| Keyboard | PS2 | | Keyboard | |
| --- | --- | --- | --- | --- |
| Arrows | D-pad | | F10 | Settings (pauses) |
| X / S / Z / A | ✕ / ○ / □ / △ | | F11 | Fullscreen |
| Q / W, 1 / 2 | L1 / R1, L2 / R2 | | F12 | Screenshot |
| Enter / Right Shift | Start / Select | | | |

Most options are in the settings overlay (F10) and are saved to `saves/ssx3-settings.cfg`;
saves go to `saves/ssx3-mc0.ps2`. Every option is also an environment variable, e.g.
`SSX3_SKIP_MOVIES=1`, `SSX3_PAD2=1` (second player), `SSX3_TICK_HZ=120`,
`SSX3_RENDER_SIZE=2560x1080`, `SSX3_FOV=1.2`. Full list, texture packs and the
emulated reference paths: [`docs/options.md`](docs/options.md).

## Architecture

| Crate | Role |
| --- | --- |
| `ssx3` | Front ends: the windowed `ssx3` (winit + wgpu + gilrs + cpal) and `ssx3-headless`. |
| `ssx3-game` | The hand-written Rust: game functions in `src/game/<dir>/<unit>.rs` after the C++ source tree, each checked against the generated one; the native draw functions (`packetless`), native libmpeg (`movie.rs`), the native file layer, the tick rate (`tick.rs`). |
| `ssx3-recomp` | The function inventory, the R5900 decoder, and `recomp-ee`: ~808k instructions in 13.3k functions lowered to structured Rust, sharded into 16 crates under `generated/ee/`. `recomp-iop` does the same for the game's IOP modules. See [`docs/investigation/recompiler-v2.md`](docs/investigation/recompiler-v2.md). |
| `ssx3-ee-rt` | The EE core the generated code compiles against: `Cpu`, instruction semantics (MMI, bit-exact PS2 float), guest memory, the function table, the `Platform` trait. Kept small so hardware changes never recompile generated code. |
| `ssx3-ps2` | The platform (`Machine`): an HLE EE kernel (threads, semaphores, alarms), the hardware registers the game touches, SIF and the native IOP services, the frame sink and native DMA kick; the emulated GS, VIF and IPU for the reference path. |
| `ssx3-gfx` | The native renderer (wgpu): typed frames from the game's draws, the VU1 programs on CPU/GPU, post passes, modern graphics options. |
| `ssx3-render` | The typed frame (captured draws, the frame build) and the VU1 rendering programs as readable Rust. |
| `ssx3-mpeg` | The native MPEG-2 movie decoder. |
| `ssx3-snd` | The native sound driver (EA's IOP SNDDRV in Rust). |
| `ssx3-mc` | The memory card: the PS2 card filesystem on the raw `.ps2` image, answering libmc as MCMAN/MCSERV do (byte-identical saves). |
| `ssx3-vu` | VU0/VU1: registers, pipeline timing, the translated microcode (`generated/vu/`). |
| `ssx3-web`, `ssx3-vuworker` | The browser build: the single-thread form of the generated code as WebAssembly, drawn with WebGPU from a Web Worker; VU1 jobs in a worker of their own. |
| `ssx3-iop`, `ssx3-gs` | Reference paths only: the IOP runtime for the recompiled IRX modules, and the Graphics Synthesizer model (verification builds, `emu-gs`). |
| `ssx3-core`, `ssx3-runtime` | Machine-model and execution-state scaffolding (memory map, ISO reading). |

The game's threads run as coroutines (one stack each, `corosensei`) on the host thread that
drives frames, so a thread switch is a stack switch and a guest call is a plain Rust call.
`recomp-ee --unwind` generates the single-thread resumable form for the browser, using the
unwind/trampoline model from [ps2-web-recomp](https://github.com/snuri00/ps2-web-recomp)
(MIT). More in [`docs/architecture.md`](docs/architecture.md).

## Tools

Mostly Python 3, run from the repository root.

| Area | Tools |
| --- | --- |
| Setup and packaging | `setup.py` (extract, check, recompile; `bootstrap.sh` and `extract-from-iso.sh` are the POSIX-shell versions), `bundle.py` (standalone builds; `bundle_icon.py` makes the icon from the game's logo, `bundle/Dockerfile` is a Linux build environment), `upscale-textures.py` (AI-upscaled texture packs). |
| Porting to hand-written Rust | `pick_wave.py` (choose the next functions by runtime hotness from a `callgraph.py`/`SSX3_CALLPROF` profile), `refsrc.py`, `refbody.py`, `refindex.py`, `refdossier2.py` (look up the matching C++ decompilation and write per-function dossiers), `integrate_wave.py` (install ported modules: regenerate, test, build, RAM gate), `reorg_modules.py` (move ports into the source-tree layout). |
| Verification | `scenarios.py` (scripted headless scenarios), `bisect_overrides.py` (find the port that makes a run differ), `compare-file-logs.py`, `vu-diff/` (our VU against ps2-web-recomp's interpreter, step by step), `run-headless.sh`, `contact_sheet.py` (a page of headless screenshots). |
| Tick rate | `scan_framerate.py`, `scan_tick_patterns.py` (find 60 Hz assumptions), `tick_ab.py` (the same race at 60 Hz and N Hz, side by side). |
| Ghidra and inspection | `export-ghidra-functions.py`, `export-ghidra-boundaries.py`, `export-ghidra-disasm.py`, `import-ghidra-names.py`, `ghidra_create_discovered.py` (keep the function inventory and Ghidra in sync), `iso-ls.py` (files on the disc), `mcls.py` (files on a memory card image), `pine.py` (read and write PCSX2's memory for comparisons). |

## Contributing

- **Docs:** [`docs/architecture.md`](docs/architecture.md) (how it fits together),
  [`docs/development.md`](docs/development.md) (headless runs, verification, debugging,
  Ghidra), [`docs/plans/`](docs/plans/) (roadmap and active plans).
- **Porting functions to hand-written Rust:**
  [`docs/investigation/phase2-worker-guide.md`](docs/investigation/phase2-worker-guide.md)
  and [`AGENTS.md`](AGENTS.md). Ports live in `crates/ssx3-game`, are registered in
  `overrides.txt`, and each has a differential test against the generated code. The C++
  decompilation [DevvyDont/ssx3](https://github.com/DevvyDont/ssx3) is a readability aid;
  the generated code is the authority.
- **Exactness is the rule.** A scripted headless race must give identical RAM before and
  after a change unless the change is meant to alter behaviour
  (`python3 tools/integrate_wave.py gate`).
- **CI** builds and tests every crate that doesn't need the disc (`python3 tools/setup.py
  --stubs-only` gives the same setup locally). Changes to `ssx3-game` or the game itself
  need the differential tests and the RAM gate, run locally.
- **Never commit game material**: the disc or its files, `generated/` code, disassembly,
  RAM/VRAM dumps, or screenshots of large parts of the game.
