# Plans

Working plans for the decompilation. Each file states the goal, the design, and
what "done" means. Update them as reality changes; do not let them drift.

| File | Scope |
| --- | --- |
| [`roadmap.md`](roadmap.md) | **Start here.** Long-term phases: full static recompilation → typed Rust → native port (→ optional browser) |
| [`playable-push.md`](playable-push.md) | Phase 1 progress log, next steps, how to resume, debug switches |
| [`structured-control-flow.md`](structured-control-flow.md) | Historical: removing the block-dispatch scaffolding (done) |
| [`scale-out.md`](scale-out.md) | Historical: translating the whole inventory (done) |
| [`subsystems.md`](subsystems.md) | Historical: FPU, MMI, VU, GS/GIF, DMA/timers/INTC (done; see `../investigation/`) |

Principles:

1. **Recompiler, not emulator.** Generated Rust must contain no instruction
   fetch or decode. Only hardware and firmware are native models; game code
   (EE, VU microcode, IOP modules) is statically translated.
2. **Ghidra is authoritative** for boundaries and names; our own passes
   cross-check it (recursive descent, tests).
3. **Everything is tested.** New behaviour gets a test; findings go to
   `docs/investigation/`. Runs are deterministic: compare screenshots byte
   for byte before and after a change.
