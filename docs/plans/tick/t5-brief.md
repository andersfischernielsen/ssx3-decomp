# T5 classification brief (the whole game)

You are a SUBAGENT doing analysis only in /Users/fischer/Desktop/ssx3-decomp
(a native Rust port of PS2 SSX 3). Do not run cargo or the game, do not edit
any file except your one output file, do not commit.

Read first: `docs/plans/configurable-tick-rate.md` ("Conversion rules" and
the "T4" section: what was found and fixed already), the top of
`crates/ssx3-game/src/tick.rs` (`PER_TICK`, `NOT_A_STEP`, `frame60_due`,
`countdowns60`, `ticks_from_frames60`), and one finished sheet for the format
and the level of detail expected (`docs/plans/tick/t4-a.tsv`, `t4-d.tsv`).

The game ran at a fixed 60 Hz tick; it now runs at 60/120/240 Hz. Every
`.lit4` 1/60 load already follows the rate. Wrong above 60 Hz is code that
changes a time-dependent value by a fixed amount EVERY TICK (or every call
of something called every tick): `x += k`, `v *= k`, `x += (t - x) * k`, a
rate-limited approach, `frames++` compared with a frame threshold or used as
time, a random chance rolled every tick, a constant step passed to a helper
that applies it per call (class CALLSTEP: check the helper; if the helper
already converts that argument for the rate, e.g. the chase camera helpers in
`crates/ssx3-game/src/chase.rs`, it is NOT_PER_TICK "converted in <helper>").

Your input (`docs/plans/tick/t5-input-<x>.tsv`) has one row per candidate
site from `tools/scan_tick_patterns.py` (classes STEP, SCALE, LERP, CLAMP,
OTHER, COUNT, CALLSTEP, CHANCE), with the highest calls per frame of the
function over the 33 scenarios of `tools/scenarios.py` and the scenario
where that happened (0 / `-`: the function never ran in any scenario: then
find out when it runs: replays, split-screen, online, other camera views,
menus not visited, debug-only, dead code). Most rows are false positives
(loop indexes in memory, list lengths, refcounts, cursors, IDs, event
counters, layout math). Work efficiently: decide all rows of a function from
one look at it (Ghidra decompile via ToolSearch
"select:mcp__ghidra__decompile_function_by_address,mcp__ghidra__disassemble_function,mcp__ghidra__read_memory,mcp__ghidra__get_function_xrefs";
the generated Rust in `generated/ee/s*/src/lib.rs`, search `f_<addr>`; the
hand-written port if `ported` is yes: grep the address like `0x0012_c408`
under `crates/ssx3-game/src`; `python3 tools/refsrc.py <Name>` for the C++).

Output `docs/plans/tick/t5-<x>.tsv`, tab separated, header:
`site	function	func_addr	ported	class	kind	constant	reason	conversion`
- class: PER_TICK, NOT_PER_TICK, ALREADY (already converted; say where),
  DEBUG (debug-only or dead code; say why) or UNSURE.
- kind: STEP / DECAY / SMOOTH / CLAMP_RATE / COUNTER / CHANCE / CALLSTEP / OTHER.
- reason: concrete (what the field is, who calls the function, how often).
- conversion: for PER_TICK the exact fix (`k*60/rate`, `k^(60/rate)`,
  `ticks_from_frames60(n)`, `frame60_due` gate, `1-(1-p)^(60/rate)`, or a
  single-site `.lit4` entry for `tick::PER_TICK`: give the entry address and
  its bits, and say whether it is loaded at exactly one site).
Add rows for per-tick updates you notice in these functions that the scan
missed. Reply with a short summary: counts per class, the PER_TICK list
(function, address, ported yes/no, site, what it is, how visible at 240 Hz),
and anything UNSURE.
