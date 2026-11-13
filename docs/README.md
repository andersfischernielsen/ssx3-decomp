# Documentation map

Addresses are EE virtual addresses in hex; "the image" is `orig/SLUS_207.72`.
**Current** docs describe how things work and are kept up to date;
**reference** docs hold durable game facts (layouts, protocols, behaviour)
written during research, with dated status lines that may be old;
`archive/` is history (see the bottom). Names of functions live in Ghidra and
`generated/functions.tsv`, not here.

## Workflow (current)

| doc | for |
| --- | --- |
| [`../AGENTS.md`](../AGENTS.md) | Rules, the porting loop, the tick-rate pass |
| [`porting/writer-brief.md`](porting/writer-brief.md) | Binding brief for porting writers (subagents) |
| [`investigation/phase2-worker-guide.md`](investigation/phase2-worker-guide.md) | The writer's guide: rules, helpers, worked example, test |
| [`investigation/phase2-overrides.md`](investigation/phase2-overrides.md) | How overrides replace generated functions; the contract; shared helpers |
| [`plans/configurable-tick-rate.md`](plans/configurable-tick-rate.md) | Tick-rate pass: rate switch, conversion rules, status; inventory [`plans/tick-rate-functions.tsv`](plans/tick-rate-functions.tsv), classifications [`plans/tick/`](plans/tick/README.md) |
| [`development.md`](development.md) | Building, headless runs, verification switches, diagnostics, Ghidra |
| [`options.md`](options.md) | Every setting and environment variable (players and developers) |
| [`architecture.md`](architecture.md) | How the crates fit together |
| [`frontends.md`](frontends.md) | The engine API; writing a front end |

## Plans

| doc | status |
| --- | --- |
| [`plans/roadmap.md`](plans/roadmap.md) | The phases: recomp → typed Rust → native port; where we are |
| [`plans/native-rendering.md`](plans/native-rendering.md) | Native rendering and the rest of the PS2 (N1-N6); N1, N2, N5 done |
| [`plans/port-architecture.md`](plans/port-architecture.md) | Review harness and module layout (done), modern runtime (in progress) |
| [`plans/configurable-tick-rate.md`](plans/configurable-tick-rate.md) | T1-T5 done; design decisions left |
| [`plans/modern-graphics.md`](plans/modern-graphics.md) | Optional modern rendering options (F10) |
| [`plans/shadow-maps.md`](plans/shadow-maps.md), [`plans/lighting-bugs.md`](plans/lighting-bugs.md) | Real-time sun shadows and lighting; reported bugs |
| [`plans/terrain-lighting.md`](plans/terrain-lighting.md) | Future: live terrain lighting with per-course tuning |

## Game reference (`investigation/`)

| doc | covers |
| --- | --- |
| [`investigation/structs/`](investigation/structs/README.md) | Field layouts: rider, camera, world, render, input, app |
| [`investigation/naming.md`](investigation/naming.md) | Class map, vtable slot meanings, naming conventions, unapplied guesses |
| [`investigation/frame-callgraph.md`](investigation/frame-callgraph.md) | What a race frame runs: main loop, threads, hot paths (`callprof`) |
| [`investigation/boot-path.md`](investigation/boot-path.md) | Boot to the main loop: crt0, syscalls, IOP modules, threads, interrupts |
| [`investigation/disc-and-image.md`](investigation/disc-and-image.md) | The ISO, the ELF, the memory map |
| [`investigation/phase3-files.md`](investigation/phase3-files.md) | Native files: EA's file platform seam |
| [`investigation/phase3-sound.md`](investigation/phase3-sound.md) | The SNDDRV/LIBSD ↔ EE protocol the native sound driver implements |
| [`investigation/phase3-render.md`](investigation/phase3-render.md) | The seven VU programs, draw records, 2D/post effects: the native renderer's design |
| [`investigation/n2-render-layer.md`](investigation/n2-render-layer.md) | Draw entry points, DeferMan, TexMan at the game's render layer |
| [`investigation/world-lights.md`](investigation/world-lights.md), [`investigation/world-instances.md`](investigation/world-instances.md) | Placed lights and static instances read from RAM (real-time lighting, shadow casters) |
| [`investigation/flare-occlusion.md`](investigation/flare-occlusion.md) | Open: light glows through geometry |
| [`investigation/limitations.md`](investigation/limitations.md) | Known limitations |
| [`investigation/tools.md`](investigation/tools.md) | Ghidra (GhidraMCP) and PCSX2 (PINE) as reference tools |

`images/` holds the README's pictures. The large external PS2 hardware/ISA
compilation is [`archive/ps2-bible.md`](archive/ps2-bible.md) (searchable with
`rg --no-ignore`).

## Archive

[`archive/`](archive/README.md): the emulator-era and phase-1 notes (GS, VU/VIF,
IOP, IPU models, the recompiler's design, build-time split, coverage, the
phase-3 boundary research, the rendering progress log, the ps2-web-recomp
digest). Kept for the reference build (`--features vu1-microcode`) and for
history; not needed for porting or the tick-rate pass. `docs/.ignore` hides it
from ripgrep (and the Grep tool); search it with `rg --no-ignore`.

Older code comments point at docs pruned in commit `eaaa404` as
`eaaa404^:docs/...`; read them with `git show eaaa404^:docs/<path>`.
