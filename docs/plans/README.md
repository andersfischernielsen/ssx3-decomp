# Plans

Working plans for the decompilation. Each file states the goal, the design, and
what "done" means. Update them as reality changes; do not let them drift.

| File | Scope |
| --- | --- |
| [`roadmap.md`](roadmap.md) | Phase list and current status |
| [`structured-control-flow.md`](structured-control-flow.md) | Remove the block-dispatch scaffolding |
| [`scale-out.md`](scale-out.md) | Translate the whole inventory |
| [`subsystems.md`](subsystems.md) | FPU, MMI, VU, GS/GIF, DMA/timers/INTC |

Principles:

1. **Recompiler, not emulator.** Generated Rust must contain no instruction
   fetch. Syscalls and calls are native/direct; no address dispatchers.
2. **Ghidra is authoritative** for boundaries and names; our own passes
   cross-check it (recursive descent, tests).
3. **Everything is tested.** New behaviour gets a test; findings go to
   `docs/investigation/`.
