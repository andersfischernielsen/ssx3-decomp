# Running without `generated/ee`

Goal: the default build and a full run (boot, menus, race replay) use only the
hand-written crates; `generated/ee` (the recompiled game, never committed) is
needed only by an optional reference feature for the differential tests.

## State (start of the branch)

13,306 EE functions; 11,792 have a registered port; 1,514 do not (198 KB).
The race replay plus nine event runs call 389 of the unported ones (87 KB).
Production code still reaches the generated bodies in 390 places: 307
resume guards (`if c.pc != ENTRY { return ee::..::f_X__generated(c) }`, a port
that is only the first entry of a resumable function) and 83 direct uses
(`native_files`, `capture`, `movie`, `debug`, `event_boot`).

## What "off generated/ee" means here

`tools/ee_skeleton.py write` already produces the same API as the generated
crates (slots, wrappers, `FUNCTIONS` table, `set_override`) with bodies that
panic. A build against the skeleton that runs the replay and reproduces the
gate RAM is the proof: no generated body executes. The last step replaces the
script with a committed crate whose build script emits the skeleton from
`generated/ee-symbols.tsv`, and makes the real generated code an optional
`ee-reference` feature.

## Work

1. Port every unported function.
   - 895 leaf, straight-line functions of at most 64 bytes: `tools/auto_port.py`
     writes the module and its differential test.
   - The other 619 (156 of them hot): the normal writer waves (hot first).
2. Native resume: every guarded port gets a native resume body generated from
   the `__entries` twin (`tools/resume_port.py`), so no resume reaches
   `__generated`.
3. The 83 direct uses: ported or removed case by case.
4. Skeleton run: build against the skeleton, run the replay, compare with the
   gate baseline; fix what panics (its message names the address).
5. Replace `generated/ee` by the committed skeleton crate (build script) and
   the `ee-reference` feature; update CI and `docs/options.md`/README.

Gate (`python3 tools/integrate_wave.py gate`) stays green after every step.
