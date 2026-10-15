# Phase 2 candidates: which functions to rewrite next

_Compiled 2026-10-15 for roadmap phase 2 (`../plans/roadmap.md`). The
mechanism and the contract a replacement must meet are in
[`phase2-overrides.md`](phase2-overrides.md); the first finished replacement is
`Rider_BuildWorldMatrix` (`crates/ssx3-game/src/rider.rs`)._

This catalogue ranks the ~13,300 recompiled EE functions by how easy they are
to replace by hand and how much it helps. Tier A is split into six work
packages (A1–A6) that do not depend on each other, so several people or agents
can take one each.

## How the data was made

- **Static classification** of every `pub fn f_XXXXXXXX` in
  `generated/ee/s*/src/lib.rs`. For overridden functions the classifier reads the
  `__generated` body. The script is a throwaway, but it is simple to rebuild
  (see the end of this file). Each function gets:
  - **calls**: direct calls `c.pc = X; f_X(c)`, cross-shard direct calls
    `call_indirect(c, 0xconst)`, indirect `jalr` calls `call_indirect(c, t)`,
    and syscalls.
  - **loops**: `loop {` / `continue 'l_…` / `match pc` dispatch, each with
    `c.event()` checks.
  - **cycles**: the `c.cycles += N` statements. A single block means one fixed
    cost. With several blocks, the "total" column is the sum over all blocks,
    which is an upper bound and not the cost of one path.
  - **op counts**: VU0 (`c.vu0()`, `lqc2/sqc2/qmtc2/qmfc2/cfc2/ctc2`), FPU
    (`lwc1/swc1/*_s/cvt/c_*`), MMI (`p*`, `qfsrv`), memory, stores, COP0.
  - **inputs read before written**: FPU registers (f12–f19 are float arguments)
    and VU0 registers. The `vsub vfN, vfN, vfN` zeroing idiom is ignored.
  - **`__entries` twin**: present means the function has an external
    mid-function entry and cannot be overridden.
- **Size** comes from the function table (`generated/ee/game/src/lib.rs`
  `FnEntry`). **Names** come from `generated/functions.tsv` and the `///`
  doc comments; they match the live Ghidra project (4,473 named).
- **Hotness** comes from the race-path profile (macOS `sample`, 1 ms, scripted
  Snow Jam race; `scratchpad/prof/sample.txt`). For each guest function the
  profile gives the number of samples with that function on the stack, with
  recursion counted once. Many small leaves get inlined into their callers by
  rustc and so never show up in it. The race flag therefore has three levels:
  - **P**: seen in the profile;
  - **c**: a direct callee of a profiled function;
  - **c2**: a callee of a **c** function.

  Virtual methods have no direct callers, so they show up only if they are
  sampled (`callers=0` in the tables).
- **Semantics** come from the Ghidra decompilation (GhidraMCP HTTP backend) and
  from reading the generated Rust. Field names come from `structs/*.md`.

## Summary statistics

Race columns: **P** = seen in the race profile; **P/c/c2** = seen, or a
callee up to two levels below a profiled function.

| set | functions | named | P | P/c/c2 |
|---|---:|---:|---:|---:|
| all generated EE functions | 13306 | 4473 | 677 | 2216 |
| with an `__entries` twin (**not overridable**) | 365 | 126 | 35 | 73 |
| **leaf** (no direct or indirect calls, no syscalls) | 3371 | 588 | 103 | 554 |
| **leaf, no loop** | 2791 | 401 | 52 | 393 |
| leaf, no loop, one cycle block (straight-line) | 2122 | 246 | 27 | 223 |
| leaf, no loop, ≤ 256 bytes | 2681 | 376 | 39 | 350 |
| leaf with loops | 580 | 187 | 51 | 161 |
| non-leaf | 9570 | 3759 | 539 | 1589 |
| non-leaf, direct calls only (no `jalr`), no loop | 5483 | 2125 | 216 | 780 |
| … of which ≤ 256 bytes and ≤ 3 calls | 4088 | 1557 | 113 | 496 |
| with an indirect call (`jalr`: virtual call or function pointer) | 2331 | 886 | 204 | 462 |
| with a syscall | 83 | 76 | 5 | 25 |
| using VU0 macro ops | 497 | 114 | 131 | 242 |
| using the FPU | 2407 | 676 | 321 | 846 |

The "leaf, no loop" rows below are broken down by the most demanding kind of
instruction each function uses (a function is counted once):

| leaf, no loop, by kind | functions | named | P | P/c/c2 |
|---|---:|---:|---:|---:|
| integer only (no memory access) | 992 | 40 | 4 | 40 |
| integer + memory | 1303 | 277 | 15 | 216 |
| FPU (COP1) | 390 | 64 | 16 | 95 |
| VU0 macro (COP2) | 96 | 19 | 16 | 39 |
| MMI (128-bit integer) | 3 | 0 | 0 | 1 |
| COP0 / interrupt control | 7 | 1 | 1 | 2 |

What the numbers mean:

- **848 of the 992 integer-only leaves are 8-byte stubs** (`jr ra; li v0, K`):
  default virtual methods and empty handlers. 729 of them are not Ghidra
  functions at all. Rewriting them one by one is pointless; a later
  "return-constant" family can do them in bulk. They are left out of Tier A.
- **Of the 2,791 leaf, no-loop functions only 401 are named.** The named ones
  near the race path (~100) make up Tier A. The unnamed but hot leaves are
  listed separately as candidates to *name first* (§ Name first).
- **2,331 functions make an indirect call** (vtables, function pointers). A
  replacement can make the same call through `call_indirect(c, target)`, but
  these are Tier B, not A.
- 148 leaf, no-loop functions take float arguments in f12–f19, and 6 read
  callee-saved f20+. The typed ABI needs `abi::farg(i)` and
  `abi::set_fret()` helpers (f12…, f0), which do not exist yet.

## Rules that apply to every package

These come from the contract in `phase2-overrides.md` and from what the
generated code actually does.

1. **Cycles by block, not by total.** Every `'b_…:` block and every `match pc`
   arm charges its own `c.cycles += N` when it is entered. A branchy function
   must add the same per-block amounts along the path it takes. The A-tables
   list the block costs for each function.
2. **Loops keep their event checks.** At each backward branch the generated
   code does
   `if c.cycles >= c.next_event { c.pc = <loop head>; if c.event() { return; } }`.
   Some older shapes call `c.event();` without checking the result. A
   replacement must:
   - make the same check at the same iteration boundaries;
   - set `c.pc` to the original loop-head address before calling `c.event()`;
   - return at once when it reports an unwind.
3. **Calls keep the unwind check.** After each call the generated code tests
   `if c.pc != <return address> { return; }`. A replacement must set `ra`, call
   the same callee (through `ssx3_ee_sNN::f_X`, or `call_indirect`) and
   propagate the unwind the same way. A callee can switch threads.
4. **Return values are 64-bit sign-extended.** `lw` results and `sx()` values
   land in 64-bit GPRs. `abi::set_ret` sign-extends. For `lhu/lbu/sltu` results
   zero-extension is correct, and since they are below 2^31 the same helper
   works. Functions that return a pointer built with `lui` are fine as well.
5. **Two float units, one rounding core.**
   - VU0 macro ops (`c.vu0().madd` …) use `ps2f::VuF32`.
   - FPU code (`c.add_s`, `c.madd_s`, `c.div_s`, `c.sqrt_s`, `c.cvt_w_s`) has no
     typed wrapper yet: add `ps2f::FpuF32` in A1 and share it.
   - Both go through the same `ssx3_ee_rt::float` primitives:
     - no Inf/NaN (results clamp to ±0x7f7fffff), denormals flushed,
       truncation;
     - MADD/MSUB (VU) and `madd.s`/`madda.s` (FPU) are `add(acc, mul(a, b))`
       with **two** truncations, so the operation order must be kept exactly;
     - `div`/`sqrt`/`rsqrt` results are the `.value` of the helpers;
     - `cvt.w.s` truncates and saturates;
     - FPU `c.lt.s`/`c.le.s` compare with the sign-magnitude key in `fpu.rs`,
       not IEEE rules.
   - Never use host `f32` arithmetic for a value that is stored or returned.
6. **Scratch state.** VU0 registers, caller-saved GPRs, HI/LO (`mult`) and the
   FPU accumulator need not be reproduced: callers do not read them, and replays
   prove it. Two exceptions:
   - a few hand-written VU0 routines read VF registers on entry (VU0 context
     save/restore at 0x31a308/0x3c0858, `libdev_Vu1PutReg`/`GetReg`, 0x31a268,
     0x3c07a8). These are not Tier A.
   - `cfc2 $x, vi22` reads the Q register right after a `vsqrt`/`vdiv` + WAITQ
     (CameraBase_GetPitch, ChaseCameraAlgorithm_CheckDistanceReset). Reproduce
     the value, not the register.
7. **Quadword accesses ignore the low 4 address bits** (`lq/sq/lqc2/sqc2`).
   `guest::Addr::read_vec4` already does this; keep it when the pointer
   arithmetic is not obviously aligned.
8. **Two code shapes.** Most functions are structured blocks
   (`'b_X: { … break 'b_X; }`). Irreducible control flow (libc `memset`,
   `strlen`, …) uses a `loop { match pc { … } }` dispatcher. Both charge cycles
   per block.
9. **Stack scratch counts as memory.** A function that stores into its own
   frame (`sq`/`sqc2`/`swc1` to `sp+N`, common in compiled VU0 code that moves
   values between the FPU and VU0 through memory) leaves those bytes in RAM.
   The end-of-run RAM dump compares them. Either reproduce the stack stores, or
   show with a replay that later frames overwrite them before the dump. The
   former is safer.
10. **One function per PR, with an A/B replay.** Check `SSX3_NO_OVERRIDES=1`
   against the overrides, or use `SSX3_OVERRIDES=addr` to isolate one. Log it in
   `phase2-overrides.md`.

## Tier A: named leaves without loops (six work packages)

Selection: named; no calls of any kind (direct, cross-shard, `jalr`, syscall); no loops; no `__entries` twin; on or next to the race path (P/c/c2), or clearly core (math, camera, rider). Packages are independent; within a package the order is a suggestion (each package's notes give one). **Cycles**: a single number = one block; `≤N (K)` = sum of K blocks, path-dependent: see the per-function gotchas for the block costs. **Race**: P = in the race profile, c = callee of a profiled function, c2 = one level further, blank = reached only via virtual calls or outside the profiled race window. **Callers** = direct call sites (0 = called only through vtables/function pointers).

| package | functions | VU0 | FPU | in race profile (P) | suggested owner skills |
|---|---:|---:|---:|---:|---|
| A1 Math & numeric (FPU) | 18 | 0 | 15 | 5 | float semantics, numerics |
| A2 Rider & animation | 20 | 1 | 11 | 2 | rider structs, FPU |
| A3 Camera | 13 | 1 | 5 | 0 | VU0 math, camera structs |
| A4 Render packets & graphics | 16 | 0 | 3 | 1 | GS registers, 64-bit packing |
| A5 Sound, streaming & files | 20 | 0 | 1 | 1 | linked lists, sound driver |
| A6 Game state, lists, input & replay | 21 | 0 | 0 | 1 | struct views, lists |

### A1 Math & numeric (FPU)

FPU math used all over the rider, camera and FX code (`Math_SinCos` has 83 call sites and 15 profile samples). **Build `ps2f::FpuF32` here first**: it is an FPU-semantics float (add/sub/mul/div/sqrt/cvt/compare key) mirroring `VuF32`, plus `abi::farg`/`abi::set_fret`. Every other package that touches the FPU depends on it. Pure functions with exactly known inputs make these the easiest to unit-test: compare against the generated body on random bit patterns in a `Cpu` test harness before any replay.

| addr | name | size | cycles | kind | race | callers |
|---|---|---:|---|---|---|---:|
| 0x31be50 | `Math_SinCos` | 272 | ≤68 (14) | FPU | P | 83 |
| 0x31bf60 | `Math_Sin` | 224 | ≤56 (8) | FPU | P | 13 |
| 0x31c040 | `Math_Cos` | 228 | ≤57 (8) | FPU | P | 14 |
| 0x31c128 | `Math_Asin` | 256 | ≤64 (5) | FPU | P | 19 |
| 0x31c228 | `Math_Atan` | 296 | ≤74 (7) | FPU | P | 47 |
| 0x16bce0 | `Math_WrapAnglePi` | 76 | ≤19 (3) | FPU |  | 1 |
| 0x2d18b0 | `Noise_Sample1D` | 116 | ≤29 (2) | FPU | c | 1 |
| 0x31d5a8 | `cCSICubicSpline_EvalSegment` | 60 | 15 | FPU | c2 | 1 |
| 0x317a08 | `Rand_Next` | 224 | ≤56 (10) | int+mem | c2 | 3 |
| 0x317958 | `Rand_Seed` | 172 | 43 | int+mem |  | 2 |
| 0x40f8a0 | `fabsf` | 28 | 7 | FPU | c | 2 |
| 0x40f9d0 | `isnanf` | 36 | 9 | FPU | c | 2 |
| 0x40f8c0 | `finitef` | 36 | 9 | FPU | c2 | 1 |
| 0x40fb58 | `copysignf` | 48 | 12 | FPU |  | 1 |
| 0x40f8e8 | `floorf` | 228 | ≤57 (15) | FPU |  | 1 |
| 0x4114d0 | `__muldi3` | 96 | 24 | int | c2 | 7 |
| 0x2b2690 | `SongInstance_GetBeatMs` | 36 | 9 | FPU |  | 4 |
| 0x231f80 | `LoadIndicator_Advance` | 60 | ≤15 (3) | FPU |  | 4 |

Conventions: `gp` = 0x4a30f0. `c.f[n]` are raw u32 float bits. "FPU" ops follow `ssx3-ee-rt/src/fpu.rs`: truncating rounding, no inf/NaN (overflow clamps to +-0x7f7fffff), denormals flushed, `sqrt_s` takes sqrt(|x|), `div_s` by zero gives +-0x7f7fffff, `cvt_w_s` truncates and saturates, `c_lt_s` sets the FCC bit in fcr31 (a side effect the generated code leaves behind in every function below that compares). All float constants below were read from RAM (little-endian floats).

#### `Math_SinCos` — 0x0031be50
*272 B · cycles: 68 total over 14 blocks, path-dependent · ops: FPU 44 (incl. 1 sqrt), VU0 0, 14 loads/stores (8 stores)*

Computes sine and cosine of an angle in radians in one call: `void Math_SinCos(float rad /*f12*/, float *out_sin /*a0*/, float *out_cos /*a1*/)`. Range-reduces with n = trunc(x*(2/pi) +/- 0.5) (round to nearest, ties away), p = x - float(n)*(pi/2), s = p*p, then sin_p = (((s*K0 + K1)*s + K2)*s + K3)*s + 1.0) * p and cos_p = sqrt(1.0 - sin_p*sin_p). The quadrant n&3 selects the outputs: 0 -> (sin_p, cos_p); 1 -> (cos_p, -sin_p); 2 -> (-sin_p, -cos_p); 3 -> (-cos_p, sin_p), written as `*a0 = sin(x)`, `*a1 = cos(x)`. Constants (gp-relative loads): 0x4a0230 = 0x3f22f983 (2/pi), 0x4a0234 = 0x3fc90fdb (pi/2), K0 0x4a0238 = 0x3638ef1f, K1 0x4a023c = 0xb9500d03, K2 0x4a0240 = 0x3c088889, K3 0x4a0244 = 0xbe2aaaab. No v0/f0 return value (v0 is a scratch value that differs per path; callers do not use it).

**Gotchas:**
- Cycles: 6, then +4 if x < 0 (sub 0.5) else +3 (add 0.5); then +29 for the reduction/poly block; then by quadrant: n==1 +4; n==0 +2+2+2 = 6; n==2 +2+2+5 = 9; n==3 +2+2+2+3 = 9. Total range 6+3+29+4 = 42 up to 6+4+29+9 = 48.
- The 0.5 offset test is `c_lt_s(x, 0.0)`: x == -0.0 and x >= 0 take the +0.5 path.
- n comes from `cvt_w_s` (truncate toward zero, saturates for huge x); only n&3 on the 32-bit value is used (negative n works through two's complement).
- The sine polynomial here is NOT the one in `Math_Sin` (this one is `(poly + 1.0) * p`, Math_Sin is `(poly*s)*p + p`); results differ in the last bit, so do not share code.
- Cosine is computed through `sqrt_s(1.0 - sin_p^2)`; if sin_p^2 > 1 the PS2 sqrt takes |x| (no NaN). Cos is therefore always >= +0.
- Stores happen in the return delay slot order: `*a0` first, then `*a1` (matters only if the pointers alias). Both written exactly once, always.
- f-regs f0..f6, f12 are clobbered (f12 = p); not caller-visible by contract.

#### `Math_Sin` — 0x0031bf60
*224 B · cycles: 56 total over 8 blocks, path-dependent · ops: FPU 43, VU0 0, 9 loads*

`float Math_Sin(float rad /*f12*/)`, result in f0. Reduces like SinCos (n = trunc(x*2/pi +/- 0.5), p = x - float(n)*pi/2, s = p*p). If n is even it returns `((((s*a + b)*s + c)*s + d)*s) * p + p` with a=0x4a025c (0x3638ef1f), b=0x4a0260 (0xb9500d03), c=0x4a0264 (0x3c088889), d=0x4a0268 (0xbe2aaaab); if n is odd it returns the cosine polynomial `((((s*e + f)*s + g)*s + (-0.5))*s + 1.0)` with e=0x4a0250 (0x37d00d03), f=0x4a0254 (0xbab60b62), g=0x4a0258 (0x3d2aaaab). Negates the result if (n & 2) != 0. Range constants: 0x4a0248 = 0x3f22f983 (2/pi), 0x4a024c = 0x3fc90fdb (pi/2).

**Gotchas:**
- Cycles: 6; +4 if x < 0 else +3; +9; then +16 (n odd) or +13 (n even); +3 (the sign block; the negate itself is free); +2 on return. Range 36..40.
- The poly is evaluated as separate mul then add (two roundings each step), truncating FPU rounding; no fused ops. Order: `t = s*a; t = t + b; t = t*s; t = t + c; ...`.
- The odd-n path loads -0.5 and 1.0 as immediates (`0xbf000000`, `0x3f800000`), not from memory.
- Negation is a sign-bit XOR (`neg_s`), so a +0.0 result becomes -0.0 for n&2.
- v1 holds n (clobbered), f0 is the return value.

#### `Math_Cos` — 0x0031c040
*228 B · cycles: 57 total over 8 blocks, path-dependent · ops: FPU 43, VU0 0, 9 loads*

`float Math_Cos(float rad /*f12*/)`, result in f0. Same reduction as `Math_Sin` (constants 0x4a026c = 2/pi, 0x4a0270 = pi/2). If n is even it returns the cosine polynomial `(((s*h + i)*s + j)*s + (-0.5))*s + 1.0` with h=0x4a0284 (0x37d00d03), i=0x4a0288 (0xbab60b62), j=0x4a028c (0x3d2aaaab); if odd it returns the sine polynomial `(((s*0x4a0274 + 0x4a0278)*s + 0x4a027c)*s + 0x4a0280)*s * p + p` with 0x3638ef1f, 0xb9500d03, 0x3c088889, 0xbe2aaaab. Negates if ((n + 1) & 2) != 0.

**Gotchas:**
- Cycles: 6; +4 if x < 0 else +3; +9; then +14 (n odd, sine poly) or +15 (n even, cosine poly); +4 for the sign block (the `(n+1)&2` test); +2 return. Range 38..40.
- The n+1 is a sign-extended 32-bit add on the mfc1 value; same result as `((n as i32).wrapping_add(1) & 2)`.
- Constants live at different addresses than Math_Sin's copies (0x4a026c.. vs 0x4a0248..) but hold the same values; either may be hardcoded, but the rounding order must follow each function.
- The -0.5 and 1.0 are immediates; the polynomial order is mul-then-add with two roundings.

#### `Math_Asin` — 0x0031c128
*256 B · cycles: 64 total over 5 blocks, path-dependent · ops: FPU 55 (1 sqrt), VU0 0, 16 loads*

`float Math_Asin(float x /*f12*/)`, result in f0. Piecewise approximation of arcsin. Define P(t) = t*(c0 + t*(c1 + t*(c2 + t*c3))) + 1.0 with c0..c3 at 0x4a3ee0, 0x4a3ee4, 0x4a3ee8, 0x4a3eec (0x3e2bba25, 0x3d8a908b, 0x3d55033a, 0x3d6137ab). (1) If x < -0.70710677 (0x4a0290 = 0xbf3504f3): t = 1.0 - x*x, return sqrt(t)*P(t) + (-pi/2) (0x4a0294 = 0xbfc90fdb). (2) Else if 0.70710677 (0x4a0298 = 0x3f3504f3) < x: t = 1.0 - x*x, return pi/2 (0x4a029c = 0x3fc90fdb) - sqrt(t)*P(t). (3) Else s = x*x, return (s*(c0 + s*(c1 + s*(c2 + s*c3))))*x + x.

**Gotchas:**
- Cycles: 5 first block; x < -0.7071: +20 (return); else +5 more, then x > 0.7071: +20, or middle: +14. Totals 25 / 30 / 24.
- Note the inequality tests: `x < lo` is `c_lt_s(x, lo)`, the upper one is `c_lt_s(hi, x)`.
- Case 3 does NOT use +1.0: it is `(s*Q(s))*x + x` where Q is the Horner chain without the `+1.0` (the generated code: `f0 = s*Q; f0 = f0*x; f0 = f0 + x`).
- Inputs outside [-1, 1] never error: `1 - x*x` goes negative and `sqrt_s` takes |.|.
- Horner steps are mul then add (two roundings). In cases 1 and 2 the final multiply is `sqrt(t) * (t*Q + 1.0)`; compute `t*Q` first, then +1.0, then multiply by sqrt.
- c0..c3 are read from gp+0xdf0..0xdfc (0x4a3ee0..0x4a3eec); they are likely never written but check before hard-coding.

#### `Math_Atan` — 0x0031c228
*296 B · cycles: 74 total over 7 blocks, path-dependent · ops: FPU 62 (1 div, 1 sqrt), VU0 0, 14 loads*

`float Math_Atan(float x /*f12*/)`, result in f0. Uses the same c0..c3 polynomial as `Math_Asin`, evaluated at u = x*x/(1+x*x) or 1/(1+x*x). (1) x < -1.0: u = 1.0 / (x*x + 1.0); return sqrt(u)*(u*Q(u) + 1.0) + (-pi/2) (0x4a02a0 = 0xbfc90fdb). (2) x > 1.0: same u, return pi/2 (0x4a02a4 = 0x3fc90fdb) - sqrt(u)*(u*Q(u) + 1.0). (3) |x| <= 1: u = (x*x) / (x*x + 1.0); r = sqrt(u)*(u*Q(u) + 1.0); return x < 0 ? -r : r. Q(u) = c0 + u*(c1 + u*(c2 + u*c3)) with the constants at 0x4a3ee0..0x4a3eec.

**Gotchas:**
- Cycles: 6; x < -1: +21 (27 total). Else +6 more; x > 1: +19 (31 total); middle: +19, +1 more if x < 0 (negate), then +2 return (33 or 34 total).
- Two FPU divides' semantics matter: `div_s` here (a/b with b = x*x+1 >= 1, never zero) uses the f64-divide + truncating conversion in `float::div`; do not use host f32 division.
- Case 3 sign test is `c_lt_s(x, 0.0)` using a zero produced by `mtc1 $zero`; -0.0 is not less than 0, so the result stays +0 (sqrt(0) = 0).
- Negation is `neg_s` (sign flip); in case 3 the negate cycle (+1) is only charged when x < 0.
- Tests use `c_lt_s(x, -1.0)` and `c_lt_s(1.0, x)`; x == +-1 falls into case 3.
- Large |x| (x*x overflow to 0x7f7fffff clamp) still returns a finite value through the clamped arithmetic.

#### `Math_WrapAnglePi` — 0x0016bce0
*76 B · cycles: 19 total over 3 blocks, path-dependent (16 or 19) · ops: FPU 14, VU0 0, 4 loads/stores (1 store)*

`void Math_WrapAnglePi(float *angle /*a0*/)`: wraps a float angle in place into [-pi, pi). Computes t = x*(1/2pi) + 0.5 (0x49cc78 = 0x3e22f983), fl = float(trunc(t)), subtracts 1.0 if t < fl (a floor fixup for negative non-integers), then `*a0 = x - fl*(2pi)` (0x49cc7c = 0x40c90fdb). Only the memory at a0 is written; there is no return value.

**Gotchas:**
- Cycles: 11 first block; +3 if t < float(trunc(t)) (negative fractional t) else 0; +5 final. So 16 or 19.
- Truncation is via `cvt_w_s` then `cvt_s_w` (both FPU: truncate toward zero; `cvt_w_s` saturates).
- The store is a single `swc1` at the return delay slot of the original; no earlier store to *a0.
- Operation order: `t = x*k1; t = t + 0.5; fl = tof(trunc(t)); [fl = fl - 1.0]; r = x - fl*k2` (mul then sub, two roundings).
- The 0.5 and 1.0 are immediates.

#### `Noise_Sample1D` — 0x002d18b0
*116 B · cycles: 29 total over 2 blocks (10 + 19, unconditional) · ops: FPU 14, VU0 0, 2 loads*

`float Noise_Sample1D(float x /*f12*/)`: linear interpolation in a 161-entry float table at 0x00445ab0 (a0 is loaded with the constant 160 as the divisor and a1 with the table base). y = x*160.0 (0x43200000), n = trunc(y), i = n % 160 (signed 32-bit remainder from `div`, hi register), frac = y - float(n); returns `table[i]*(1.0 - frac) + table[i+1]*frac` in f0. Only caller is `Noise_Fractal1D` 0x2d1928.

**Gotchas:**
- Cycles: fixed 10 + 19 = 29 (the `div` has no extra latency charge; both branches of the first block fall to the same place).
- `i = n % 160` has the sign of n (C remainder), so negative inputs index BEFORE the table (table + negative*4) and i+1 can be 160 (needs 161 entries) for positive inputs; reproduce with signed `%`/`wrapping_rem`, read memory with `c.lwc1`-equivalent 32-bit loads.
- `n` comes from `cvt_w_s` (saturating; for huge y gives 0x7fffffff).
- `c.div(0, V0, A0)` also leaves hi/lo state (hi = remainder, lo = quotient); not read afterwards.
- Float order: `a = y_tab_i * (1.0 - frac); b = y_tab_i1 * frac; a + b`; `1.0 - frac` uses FPU sub.
- f12 is overwritten with frac (scratch).

#### `cCSICubicSpline_EvalSegment` — 0x0031d5a8
*60 B · cycles: 15 (single block) · ops: FPU 14, VU0 0, 5 loads*

`float cCSICubicSpline_EvalSegment(float x /*f12*/, const float *seg /*a0*/)`: evaluates one cubic segment stored as five floats seg[0..4] = {a, b, c, d, x0} (offsets 0x00, 0x04, 0x08, 0x0c, 0x10; the camera.md doc lists the owning spline as 12 B per point so this is a per-segment view). With t = x - seg[4]: returns `((seg[2]*t + seg[3]) + seg[1]*(t*t)) + (t*seg[0])*(t*t)` in f0. Called only from the spline lookup (race = callee of callee).

**Gotchas:**
- Cycles: straight 15, single block, no branches.
- Exact order from the generated code: `t = x - s4; t2 = t*t; p = s2*t; q = t*s0; r = s1*t2; p = p + s3; q = q*t2; p = p + r; ret = q + p`. FPU mul/add/sub (truncating), so `(t*a)*t2` is not `a*(t*t2)`.
- No stores; seg is read-only.

#### `Rand_Next` — 0x00317a08
*224 B · cycles: 56 total over 10 blocks, path-dependent · ops: FPU 0, 23 loads/stores (11 stores), integer only*

`int Rand_Next(uint32_t *state /*a0*/)`: advances a 6-word (24-byte) multiple-precision additive generator and returns the new s[0] in v0 (sign-extended 32-bit). With s = state: u4 = s5 + s4, c = (u4 < s5 || u4 < s4); s4 = u4; u3 = u4 + s3 + c; s3 = u3; u2 = u3 + s2 + (u3 < s3_old); s2 = u2; u1 = u2 + s1 + (u2 < s2_old); s1 = u1; u0 = u1 + s0 + (u1 < s1_old); s0 = u0. Then s5 += 1 and, if it wrapped to 0, ripple-increments s4, s3, s2, s1 (stopping at the first that does not wrap, and incrementing s0 only if all four wrapped). Returns the final s0. Reads/writes state[0..5] at offsets 0x00..0x14.

**Gotchas:**
- Cycles: 7; carry test of the first add: s5 carry -> +2; else +3 and, if u4 < s4, +2 more (total 5); else 3. Then +23 for the main block. Then the s5 wrap chain: s5 != 0 -> nothing; else +4 per word (s4, s3, s2, s1 checks) and +3 for the final s0 increment (max 4+4+4+4+3). Always +2 on return.
- The carry test uses only `sum < (the word being added)` with the already-carry-added sum (not a full carry-out), copy the compare operands exactly: carry2 = (new s3 < old s3), carry3 = (new s2 < old s2), carry4 = (new s1 < old s1); the generated code uses 64-bit unsigned compares of sign-extended values, equal to unsigned 32-bit compares.
- Store order: s4, s3, s2, s1, s5, s0 (s0 is stored in the delay slot of the s5 != 0 branch, before the wrap chain), then wrap-chain increments of s4, s3, s2, s1 (each stored as it is incremented), then possibly s0 again (stored with the +1). Matters only for write-watch/identical memory traces.
- Return value = sign-extend(final s0 as i32); on the all-wrap path s0+1 is returned and stored.
- Many caller-saved regs (t0, t1, a1..a3) are scratch.

#### `Rand_Seed` — 0x00317958
*172 B · cycles: 43 (single block) · ops: FPU 0, 6 stores, integer only*

`void Rand_Seed(uint32_t *state /*a0*/, uint32_t seed /*a1, low 32 bits*/)`: initialises the 6-word generator used by `Rand_Next`. Writes state[0..5] = seed + {-0x0dd2f1aa, -0x77ced917, -0x39db22d1, +0x0702c49c, -0x61cac083, +0x6fdf3b64} (wrapping 32-bit). In the generated code these are produced by a running 64-bit accumulator: a1 (zero-extended seed) += 0xf22d0e56, += 0x96041893, += 0x3df3b646, += 0x40dde76d, += 0x97327ae1, += 0xd1a9fbe7, each time storing the low 32 bits sign-extended-then-truncated.

**Gotchas:**
- Cycles: flat 43, single block.
- Stores in order [0], [1], [2], [3], [4], [5] (offsets 0x00, 0x04, 0x08, 0x0c, 0x10, 0x14); no loads.
- Values stored are the cumulative sums (the Ghidra constants above are the same thing expressed relative to the seed); verify against `state[i] = (seed as u32).wrapping_add(K_i)`.
- Seed is zero-extended from 32 bits (`<<32 >>32` logical); only the low 32 bits ever reach memory, so 32-bit wrapping add per slot is equivalent.

#### `fabsf` — 0x0040f8a0
*28 B · cycles: 7 (single block) · ops: FPU 2 (mfc1/mtc1), VU0 0, 0 loads/stores*

`float fabsf(float x /*f12*/)`: returns the float with the sign bit cleared in f0: `f0 = f12 & 0x7fffffff` (pure bit operation through GPRs, so a denormal or exp-255 value passes through unchanged). Callers: 2.

**Gotchas:**
- Cycles flat 7.
- Bit-level: `mfc1` then AND then `mtc1` into f0; no FPU flush; f12 is not modified.
- Side effects in GPRs: v1 = abs bits (sign-extended positive), v0 = 0x7fffffff. Not return values.

#### `isnanf` — 0x0040f9d0
*36 B · cycles: 9 (single block) · ops: FPU 1 (mfc1), VU0 0*

`int isnanf(float x /*f12*/)`: returns v0 = `(0x7f800000 - (bits(f12) & 0x7fffffff)) >> 31` (logical shift of the 32-bit difference, so 0 or 1). It is 1 only for an IEEE NaN bit pattern (|bits| > 0x7f800000), which is the integer test, independent of PS2 FPU rules (exp-255 values with zero mantissa return 0). Callers: 2.

**Gotchas:**
- Cycles flat 9.
- v0 result is the u32 shift result sign-extended (value is 0 or 1 only).
- Pure integer: uses `mfc1` bits (f12 raw), so denormals/exp 255 are NOT normalised.
- Clobbers a0 (0x7fffffff), v1 (0x7f800000).

#### `finitef` — 0x0040f8c0
*36 B · cycles: 9 (single block) · ops: FPU 1 (mfc1), VU0 0*

`int finitef(float x /*f12*/)`: returns v0 = `((bits & 0x7fffffff) + 0x80800000) >> 31` (32-bit wrapping add, logical shift), i.e. 1 if the exponent field is not 255 (|bits| < 0x7f800000) and 0 for inf/NaN bit patterns. Callers: 1.

**Gotchas:**
- Cycles flat 9.
- The add is `sx((a as u32).wrapping_add(0x80800000))` then `>> 31` on the u32; result 0 or 1.
- Clobbers a0 (0x7fffffff) and v1 (0xffffffff80800000).
- Bit-pattern test only: a real PS2 float with exp 255 (a valid large number there) returns 0.

#### `copysignf` — 0x0040fb58
*48 B · cycles: 12 (single block) · ops: FPU 3 (2 mfc1, 1 mtc1), VU0 0*

`float copysignf(float x /*f12*/, float y /*f13*/)`: returns in f0 the bits `(bits(x) & 0x7fffffff) | (bits(y) & 0x80000000)`. Pure bit operation. Callers: 1.

**Gotchas:**
- Cycles flat 12.
- The result is written to f0 with `mtc1` from a1 (low 32 bits only); a1 upper half contains sign-extension from y when y is negative (irrelevant, caller-saved).
- Clobbers a0, a1, v0, v1 (a0 ends as 0xffffffff80000000, v0 as 0x7fffffff).
- f12/f13 unchanged.

#### `floorf` — 0x0040f8e8
*228 B · cycles: 57 total over 15 blocks, path-dependent · ops: FPU 13, VU0 0, 0 loads/stores*

`float floorf(float x /*f12*/)`: a fdlibm-style bit-twiddling floor. i0 = bits(x), j0 = ((i0 >>arith 23) & 0xff) - 0x7f. If j0 >= 23: returns x if j0 != 0x80, else x + x (exp 255, via `add_s`). If j0 < 0 (|x| < 1): returns +0.0 for x >= 0 (sign bit clear: result bits 0), for negative returns -1.0 (0xbf800000) unless the magnitude bits are zero (then -0.0 unchanged). If 0 <= j0 < 23: m = 0x7fffff >> j0; if (i0 & m) == 0 returns x; else for negative i0 first adds 0x800000 >> j0, then clears the fraction bits `i0 &= ~m`; returns bits as float in f0. Constants: 1e30 = 0x7149f2ca (the "inexact" probe).

**Gotchas:**
- Cycles: 8 first block. j0 >= 23: +2+2 = 12 total for both (j0 == 0x80: add_s). j0 in [0,22]: +2 +5 (mask block), fraction zero: +2 (return) = 17 total; fraction non-zero: +8 (probe) +2 (+4 more for negative) +2 +3 => 30 (positive) or 34 (negative). j0 < 0: +2 +8, then positive: +2 +2 +3 = 25 total; negative: +2 +5 +3 = 28 total.
- The probe is `f0 = x + 1e30 (FPU add); c_lt_s(0.0, f0)`; for |x| < 2^23 it is always true, so the "else" skip path (returns f12 untouched) is unreachable in practice, but it does set the FCC bit in fcr31 and can be elided only if FCC is treated as dead (check).
- On modifying paths the new bits are moved with `mtc1` into f12 (in place) and `mov_s(f0, f12)`; on pass-through paths f0 = f12 via `mov_s`.
- Shifts use `(j0 & 31)`, arithmetic shifts of positive masks; `0x800000 >> j0` is an arithmetic shift as well.
- Integer sign handling is on `i0 as i32` (sign-extended from mfc1).
- No FPU rounding mode dependence: the result is built from integers (so no truncation concerns).

#### `__muldi3` — 0x004114d0
*96 B · cycles: 24 (single block) · ops: FPU 0, integer multiply (mult x2, multu x1), 0 loads/stores*

`uint64_t __muldi3(uint64_t a /*a0*/, uint64_t b /*a1*/)`: libgcc 64-bit multiply built from 32-bit multiplies; returns v0 = `a.wrapping_mul(b)` as a full 64-bit value in v0. Computed as lo = alo*blo (unsigned 64), hi = hi(lo-product) + alo*bhi + ahi*blo (32-bit wrapping), result = lo32 | (hi << 32). Callers: 7.

**Gotchas:**
- Cycles flat 24.
- It leaves HI/LO state behind: the two `mult`/`multu` forms write `lo[0]/hi[0]` (alo*blo unsigned product) and `lo[1]/hi[1]` pipe 1 results, plus GPR side destinations (v1 = alo*bhi low word, a2 = ahi*blo low word). Callers should not read these but a byte-identical RAM dump is not affected.
- v1, a0, a1, a2 are clobbered (a1 ends as the mask 0xffffffff). Only v0 is the return value.
- Rust: `c.g[V0][0] = a0.wrapping_mul(a1)` is exactly equal for the return value (the multiplications are signed `mult` on the cross terms but only the low 32 bits are used, which are sign-agnostic).

#### `SongInstance_GetBeatMs` — 0x002b2690
*36 B · cycles: 9 (single block) · ops: FPU 5, VU0 0, 2 loads*

`float SongInstance_GetBeatMs(SongInstance *song /*a0*/)`: reads `p = *(u32*)a0` (pointer to song info), `bpm = *(float*)(p + 0x90)`, and returns `(60.0 / bpm) * 1000.0` (milliseconds per beat) in f0. Constants are immediates: 0x42700000 (60.0) and 0x447a0000 (1000.0). Callers: 4.

**Gotchas:**
- Cycles flat 9.
- FPU divide semantics: bpm == 0 gives 0x7f7fffff (+max, sign from operands) rather than inf/NaN, and the *1000 clamps to 0x7f7fffff again; denormal bpm reads as 0.
- Two loads: `lw` of *a0 (32-bit, sign-extended into v0, used as an address via `as u32`), then `lwc1` from v0+0x90. v0 is left holding the struct pointer.
- Do the divide first then multiply (`(60/bpm)*1000`), not `60000/bpm`.

#### `LoadIndicator_Advance` — 0x00231f80
*60 B · cycles: 15 total over 3 blocks, path-dependent (12 or 15) · ops: FPU 11, VU0 0, 4 loads/stores (2 stores)*

`void LoadIndicator_Advance(float *phase /*a0, field of LoadIndicator*/)`: adds a constant step 0x3c888889 (~1/60, at gp-0x51a8 = 0x49df48) to `*a0` and wraps the result to [0, 1): s = *a0 + step; fl = float(trunc(s)); if s < fl then fl -= 1.0; `*a0 = s - fl`. Callers: 4. No return value.

**Gotchas:**
- Cycles: 9, +3 only if s < float(trunc(s)) (negative non-integer s), +3 final: 12 or 15.
- There are TWO stores to *a0: the unwrapped sum `s` is stored first (in the delay slot of the branch), then the wrapped result at return. Only the final value is visible unless an interrupt (`c.event`) or memory watch observes the intermediate; keep both stores if an exact write trace is needed, otherwise a single final store is enough.
- truncate via `cvt_w_s`/`cvt_s_w` (FPU, saturating), 1.0 is an immediate (`0x3f800000`).
- Operation order: `s = *p + step` (add), then `s - fl` (sub).

**Package notes.**

This package needs a `Ps2F32` (or `Fpu32`) type that mirrors `VuF32` but with FPU rules: `add/sub/mul/div/sqrt` from `ssx3_ee_rt::float` (truncating, clamped, denormals flushed; sqrt takes |x|), `to_int` = `float::to_fixed(x, 0)` (truncate + saturate), `from_int` = `float::from_fixed`, and `lt` using the `key()` ordering of `fpu.rs`. Two small helpers are widely reused: `floor_wrap(x) = x - floor(x)` using cvt_w/cvt_s and the "if t < fl then fl -= 1.0" fix (used by `Math_WrapAnglePi` with a scale and `LoadIndicator_Advance`), and a `poly` Horner helper doing separate mul then add per step. Suggested order: pure bit/int ones first (`fabsf`, `isnanf`, `finitef`, `copysignf`, `__muldi3`, `Rand_Seed`, `Rand_Next`), then the single-block float ones (`cCSICubicSpline_EvalSegment`, `SongInstance_GetBeatMs`, `LoadIndicator_Advance`, `Math_WrapAnglePi`, `Noise_Sample1D`), then `Math_Sin`/`Math_Cos` (can share one body parameterised by `phase` n or n+1 for the sign test, but note the two have different cycle constants: sign block +3 vs +4, odd/even path costs 16/13 vs 14/15), then `Math_Asin`/`Math_Atan` (share the c0..c3 polynomial `Q(t)` and the `sqrt(t)*(t*Q+1)` tail), then `Math_SinCos` (its sine polynomial is a different formula from `Math_Sin`, do not share), and `floorf` last (many cycle paths). Every function with a compare also sets the FCC bit in fcr31 and clobbers f-registers; confirm by replay that no caller reads them.

### A2 Rider & animation

Rider getters, state hooks and the animation-sequence helpers they call. Extends `rider::Rider` with `owner()` / `RiderBase` and adds an `AnimSequence` view; this is the groundwork for the rider state machine (Tier C).

| addr | name | size | cycles | kind | race | callers |
|---|---|---:|---|---|---|---:|
| 0x11fe98 | `Rider_GetMainState` | 12 | 3 | int+mem | c | 59 |
| 0x11fee8 | `Rider_GetSubState` | 12 | 3 | int+mem | c | 26 |
| 0x11ff48 | `Rider_GetHipsPosition` | 36 | 9 | int+mem | c | 3 |
| 0x11d640 | `Rider_IsLoaded` | 32 | ≤8 (3) | int+mem | c2 | 4 |
| 0x120e30 | `Rider_UpdateFocusBox` | 28 | ≤7 (3) | int+mem | c | 2 |
| 0x120d58 | `Rider_ResetBoostMeter` | 56 | ≤14 (5) | FPU |  | 4 |
| 0x119310 | `RiderMetrix_GetScoreTier` | 84 | ≤21 (9) | int | c | 2 |
| 0x136978 | `RiderStaticState_PostUpdate` | 8 | 2 | int | c | 1 |
| 0x312aa0 | `AnimController_GetLayerAnimId` | 16 | 4 | int+mem | c | 31 |
| 0x3115c8 | `AnimTable_GetEntry` | 24 | 6 | int | c | 7 |
| 0x313cf0 | `AnimSequence_SetTrackTime` | 28 | 7 | FPU | c | 15 |
| 0x313a10 | `AnimSequence_SetWeightFade` | 16 | 4 | FPU | c | 3 |
| 0x313800 | `AnimSequence_UpdateWeightFade` | 104 | ≤26 (6) | FPU | c | 12 |
| 0x313a20 | `AnimSequence_FadeOut` | 80 | ≤20 (6) | FPU | c | 2 |
| 0x313d70 | `AnimSequence_GetTrackBlendWeight` | 96 | ≤24 (5) | FPU | P | 1 |
| 0x313c50 | `AnimSequence_SetTrackAnim` | 160 | ≤40 (3) | FPU | P | 8 |
| 0x138b48 | `RiderHandplantState_Enter` | 84 | 21 | VU0+FPU |  | 1 |
| 0x13c5a0 | `RiderRailslideState_Exit` | 60 | 15 | FPU |  | 1 |
| 0x131d08 | `RiderRailslideSub_Enter` | 40 | 10 | FPU |  | 1 |
| 0x12fe98 | `RiderBoardPressSub_Exit` | 44 | 11 | FPU |  | 1 |

#### `Rider_GetMainState` — 0x0011fe98
*12 B · cycles: 3 (single block) · ops: FPU 0, VU0 0, 2 loads/stores (2 loads)*

Returns the owner's main-state enum: `Rider*` in a0, `owner = *(a0+0x77c)` (`Rider+0x77c` owner → `RiderBase`), result `RiderBase+0xde0` mainState (0 Ground, 1 Air, 2 Wipeout, 3 Static, 4 Railslide, 5 Handplant, 6 none) in v0. No globals. 59 callers, so it is cheap to override and widely exercised. Signature: `int Rider_GetMainState(Rider *r)`.

**Gotchas:**
- Single block, `c.cycles += 3`; no branches.
- v0 is the sign-extended `lw` result (use `Addr::read_u32` as i32 then sign-extend to 64 bit via the `abi::set_ret` convention).
- v1 is clobbered with the owner pointer (scratch, callers do not read it).
- Needs a new `Rider::owner()` (`+0x77c`) and a `RiderBase` view with `main_state()` (`+0xde0`).

#### `Rider_GetSubState` — 0x0011fee8
*12 B · cycles: 3 (single block) · ops: FPU 0, VU0 0, 2 loads*

Identical to `Rider_GetMainState` but returns `RiderBase+0xde4` subState (0 Cruise, 1 BoardPress, 2 Prewind, 3 Bump, 4 NaturalAir, 5 Spin, 6 Gate, 7 Railslide, 8 Wipeout, 9 Reset, 0xa FinishLine, 0xb Handplant, 0xc UberGrind, 0xd Void, 0xe none). Input a0 = `Rider*`; owner via `+0x77c`; result in v0. Signature: `int Rider_GetSubState(Rider *r)`.

**Gotchas:**
- `c.cycles += 3`, single block.
- Same v1 scratch clobber and v0 sign-extension as GetMainState.
- Share one `owner()` helper and one `RiderBase` view with GetMainState.

#### `Rider_GetHipsPosition` — 0x0011ff48
*36 B · cycles: 9 (single block) · ops: FPU 0, VU0 0, 5 loads/stores (one `lq`, one `sq`)*

Copies the hips bone's translation (a 16-byte vec4) from the rider's skeleton into a caller buffer. a0 = hidden return pointer (16-byte vec4 out), a1 = `Rider*` (note: Ghidra shows the struct return as `param_1`, so the real object is in a1). It computes `bone = *(Rider+0x780 model)+0x2c` (`RiderModel` bone transforms, 0x20 bytes per bone: vec4 translation + quat) `+ (*(Rider+0x89c hips index) << 5)`, `lq`s 16 bytes and `sq`s them to a0. Returns v0 = a0 (the out pointer). Signature: `Vec4 *Rider_GetHipsPosition(Vec4 *out, Rider *r)`.

**Gotchas:**
- Single block, `c.cycles += 9`.
- 128-bit `lq`/`sq` ignore the low 4 address bits: replicate with `read_vec4`/`write_vec4` and mask the address to 16 bytes (the bone array and out buffer must be aligned, else the generated code reads/writes the aligned block).
- Index shift is a 32-bit `sll 5` then sign-extended add; the hips index is a signed i32 (`lw`).
- Writes all 4 lanes (w included); nothing else is written. v0 = a0 (the unmasked pointer passed in).
- The out buffer is typically stack memory in the caller; the full 16 bytes must be written even though w is unused.

#### `Rider_IsLoaded` — 0x0011d640
*32 B · cycles: 6 on either path (4 + 2) · ops: FPU 0, VU0 0, 1 load*

Returns whether the rider has finished incremental loading: a0 = `Rider*`; reads `Rider+0x880` loadStep, returns v0 = 1 when it equals 7, else 0. No globals. Signature: `bool Rider_IsLoaded(Rider *r)`.

**Gotchas:**
- Block costs: first block 4, then each branch adds 2 (equal: 2, not equal: 2), so exactly 6 cycles either way.
- Not-equal path computes `(x ^ 7) < 1` unsigned, which is 0 whenever x != 7; return exactly 0 or 1 in v0.
- v1 holds the loaded value on exit (scratch).
- Compare is on the sign-extended 32-bit load (`lw`).

#### `Rider_UpdateFocusBox` — 0x00120e30
*28 B · cycles: 5 (focus box null) or 7 (non-null) · ops: FPU 0, VU0 0, 3 loads/stores (`lw`, `lq`, `sq`)*

Mirrors the rider position into its FocusMan box. a0 = `Rider*`; `box = *(Rider+0x78c)` (focusBox, 16-byte alloc); if non-null, copies the 16 bytes at `Rider+0x110` (position, w = 1) to `*box`. No return value, no globals. 2 callers. Signature: `void Rider_UpdateFocusBox(Rider *r)`.

**Gotchas:**
- Block costs: 3 for the load and test; +2 if the box is non-null (the copy); +2 on the common tail. Null box = 5, non-null = 7.
- `lq`/`sq` copy all 16 bytes including w and are 16-byte aligned (low 4 bits of the addresses are ignored).
- v1 is clobbered with the box pointer (scratch).
- Uses `Rider::position()` (already exists).

#### `Rider_ResetBoostMeter` — 0x00120d58
*56 B · cycles: 7 (boostMode 3), 9 (mode other than 0 and 3), 12 (mode 0) · ops: FPU 4 (lwc1/swc1/mtc1 only, no arithmetic), VU0 0, 5 loads/stores*

Initialises the boost meter: a0 = `Rider*`. Sets `Rider+0x2f8` boostMeter = `Rider+0xb24` boostMeterStart, then by `Rider+0x304` boostMode: 3 (boost disabled) stores 0, 0 (meter starts full) stores 1.0f (0x3f800000), anything else keeps the start value. No return value, no globals. Signature: `void Rider_ResetBoostMeter(Rider *r)`.

**Gotchas:**
- Block costs: 5 at entry; mode == 3: +2 (7 total); mode != 3: +2, then mode != 0: +2 (9 total) or mode == 0: +3 then +2 (12 total).
- Value copy `+0xb24 -> +0x2f8` is a raw 32-bit copy (lwc1/swc1 do not canonicalise NaN/denormals), so copy the bits, not a computed float.
- Stores are `sw 0` for mode 3 and bits 0x3f800000 for mode 0, so only the final memory state matters; the intermediate store of the start value in the delay slot is overwritten.
- f0/f1 and v0/v1/at are scratch only; the FPU condition bit is not touched.

#### `RiderMetrix_GetScoreTier` — 0x00119310
*84 B · cycles: 5 (tier 0), 7 (tier 1), 9 (tier 2), 11 (tier 3), 13 (tier 4 or 5) · ops: FPU 0, VU0 0, 0 loads/stores*

Maps a trick score to a tier 0..5. a0 = `RiderMetrix*` (unused); a1 = score as a full 64-bit signed value (Ghidra `long param_2`). Thresholds: `<1000` tier 0, `<2500` 1, `<4000` 2, `<7500` 3, `<11500` 4, otherwise 5. Returns the tier in v0. No globals. Signature: `int RiderMetrix_GetScoreTier(RiderMetrix *m, int64 score)`.

**Gotchas:**
- Block costs: 3 at entry; each further threshold passed adds 2; the final test adds 2 (tier 3) or 4 (tier 4/5, which also does the conditional move). The totals above follow from that.
- Comparison is a 64-bit signed `slti` on the whole `g[A1][0]` (not just the low 32 bits).
- Tier 4/5 uses `movn`: v0 = 5 unless a1 < 11500, then 4.
- a0 and v1 are scratch on exit; a0 is overwritten with `a1 < 11500` on the last path.

#### `RiderStaticState_PostUpdate` — 0x00136978
*8 B · cycles: 2 (single block) · ops: no FPU/VU0/memory*

Empty per-frame post-update hook for the Static main state (state object at `RiderBase+0x0a0`, PostUpdate slot). Takes no meaningful input (a0 = state object, ignored) and returns nothing. Signature: `void RiderStaticState_PostUpdate(void)`.

**Gotchas:**
- Charge `c.cycles += 2` and set `pc = ra`; nothing else. It is reached through `RiderBase_PostUpdateMainState`, so keep the call path.

#### `AnimController_GetLayerAnimId` — 0x00312aa0
*16 B · cycles: 4 (single block) · ops: FPU 0, VU0 0, 1 load*

Reads one layer's anim id from the controller: a0 = controller (array of u32 ids at +0), a1 = layer index; returns `*(a0 + a1*4)` in v0. No globals. 31 callers. Signature: `uint AnimController_GetLayerAnimId(AnimController *c, int layer)`.

**Gotchas:**
- `c.cycles += 4`, single block.
- Index arithmetic is 32-bit (`sll 2`, `addu`) sign-extended; the result is the sign-extended `lw`, so ids with bit 31 set come back sign-extended in v0.
- a0/a1 are clobbered with the shifted/added values (scratch).

#### `AnimTable_GetEntry` — 0x003115c8
*24 B · cycles: 6 (single block) · ops: FPU 0, VU0 0, 0 loads/stores (one `mult`)*

Returns the address of anim table entry `a0`: `0x446990 + a0 * 0x1c` (28-byte entries in a static table). Input a0 = index, output v0 = pointer. 7 callers. Signature: `AnimEntry *AnimTable_GetEntry(int idx)`.

**Gotchas:**
- `c.cycles += 6`.
- The compiled code uses `mult`, which also writes HI/LO and overwrites a0 (rd form); callers never read these, but a byte-identical replay might if LO/HI are live across calls. Risk is low; replays will tell.
- 32-bit wrapping arithmetic, sign-extended result in v0 (index is a signed int).

#### `AnimSequence_SetTrackTime` — 0x00313cf0
*28 B · cycles: 7 (single block) · ops: FPU 1 (swc1 of f12), VU0 0, 2 stores*

Sets the playback time of one track of an AnimSequence and marks it dirty. f12 = time (float), a0 = `AnimSequence*`, a1 = track index (Ghidra lists params in a different order). Tracks are 0x1c bytes each starting at +0: `track = a0 + a1*0x1c`; writes `f12` bits to `track+0x08`, and `1` to `a0+0xc4` (dirty). No return value. 15 callers. Signature: `void AnimSequence_SetTrackTime(float t, AnimSequence *s, int track)`.

**Gotchas:**
- `c.cycles += 7`, single block.
- The float is stored as raw bits of f12 (`swc1`, no rounding); take the bits from `c.f[12]`.
- The `mult` clobbers a2 (rd form) and HI/LO; v0/v1/a1/a2 are scratch.
- Track layout (from this and neighbours): +4 anim id, +8 time, +0xc rate (1.0), +0x10 length, +0x14 weight (1.0), +0x18 active flag, +0x1c reserved (0).

#### `AnimSequence_SetWeightFade` — 0x00313a10
*16 B · cycles: 4 (single block) · ops: FPU 2 (swc1 of f12/f13), VU0 0, 3 stores*

Starts a sequence weight fade: a0 = `AnimSequence*`, f12 = target weight, f13 = duration. Writes `+0x98` = f12 bits (target), `+0x9c` = f13 bits (remaining fade time), `+0xa0` = 0 (fade-out-and-remove flag cleared). No return value. 3 callers. Signature: `void AnimSequence_SetWeightFade(float target, float duration, AnimSequence *s)`.

**Gotchas:**
- `c.cycles += 4`, single block.
- Raw bit stores of f12/f13 (no float arithmetic); the sequence is the first integer argument a0 even though Ghidra shows it as param_3.
- Sequence weight fields: +0x94 current weight, +0x98 target, +0x9c remaining fade time, +0xa0 remove-when-done flag.

#### `AnimSequence_UpdateWeightFade` — 0x00313800
*104 B · cycles: 7 (no fade), 19 (fade continues), 16 (fade completes) · ops: FPU 16, VU0 0, 9 loads/stores*

Advances a sequence's weight fade by `dt`: f12 = dt (seconds), a0 = `AnimSequence*`. With `fade = +0x9c`: if `fade == 0` nothing happens; if `fade <= dt` the fade ends (`+0x94 = +0x98`, `+0x9c = 0`) and the function returns 1 when `+0xa0` (fade-out flag) is set, else 0; otherwise `+0x9c = fade - dt` and `+0x94 += (dt / fade) * (target - cur)`. Returns 0/1 in v0. 12 callers. Signature: `int AnimSequence_UpdateWeightFade(float dt, AnimSequence *s)`.

**Gotchas:**
- Block costs: entry 5; fade==0: +2 (7 total); fade > dt: +4, +8 for the arithmetic, +2 (19); fade <= dt: +4, +5, +2 (16, returns 0 or 1).
- Needs PS2 FPU semantics (`fpu.rs`: add/sub/mul/div with denormal flush and overflow clamp, not IEEE f32). Operation order: `q = dt / fade` (old fade) first, then `newfade = fade - dt`, `diff = target - cur`, `prod = q * diff`, `cur2 = cur + prod`; each op rounds separately (no fused ops).
- `fade == 0` and `fade <= dt` use the PS2 compare keys (so -0 == +0); `fade > dt` is the negation of `<=`.
- On completion the stored `+0x9c` is f1 (raw 0 bits); `+0x94` gets the raw bits of `+0x98`.
- It does not touch the dirty flag (+0xc4). FPU registers f0..f3 and the condition bit are scratch.

#### `AnimSequence_FadeOut` — 0x00313a20
*80 B · cycles: 10, 15, 18 or 20 depending on path (see below) · ops: FPU 9, VU0 0, 7 loads/stores*

Requests a fade-out of the sequence over `f12` seconds. a0 = `AnimSequence*`, f12 = time. Logic: if the target weight `+0x98 != 0` set `+0x9c = f12`; else if `+0x9c == 0` set `+0x9c = f12`; else if `f12 < +0x9c` set `+0x9c = f12` (keep the shorter). Then always `+0x98 = 0` (target) and `+0xa0 = 1` (remove when done). v0 ends as 1 (unused). 2 callers. Signature: `void AnimSequence_FadeOut(float t, AnimSequence *s)`.

**Gotchas:**
- Block costs: entry 6. Target != 0: +1 +3 = 10. Target == 0, fade == 0: +5, store, +1, +3 = 15. Target == 0, fade != 0, f12 >= fade: +5 +4 +3 = 18 (no store of +0x9c). Target == 0, fade != 0, f12 < fade: +5 +4 +1 +1 +3 = 20 (store).
- Comparisons use PS2 FPU compare (`c_eq_s`, `c_lt_s` with sign-magnitude keys, so -0.0 equals 0.0); `f12` is stored as raw bits.
- Writes: `+0x98` = 0, `+0xa0` = 1 always; `+0x9c` only in the cases above.
- The `swc1 f1`/`mtc1` zeroing of f1 is scratch.

#### `AnimSequence_GetTrackBlendWeight` — 0x00313d70
*96 B · cycles: 20 (no edge fade), 22 (one), 24 (both) · ops: FPU 15, VU0 0, 6 loads (profiled in the race, 1 caller)*

Computes a track's blend weight with fade-in and fade-out ramps. a0 = `AnimSequence*`, a1 = track index; `tb = a0 + a1*0x1c`. Starts with `w = seq+0x94 * tb+0x14` (sequence weight times track weight). If `t = tb+8` (time) `< seq+0xa8` (fade-in length) then `w = min(t / fadeIn, w)`; then `rem = tb+0x10 (length) - t`; if `rem < seq+0xa4` (fade-out length) then `w = min(rem / fadeOut, w)`. Returns the float in f0 (not v0). Signature: `float AnimSequence_GetTrackBlendWeight(AnimSequence *s, int track)`.

**Gotchas:**
- Block costs: entry 13; fade-in branch +2; second block +5; fade-out branch +2; tail +2. So 13+5+2 = 20, +2 per ramp taken, 24 max.
- Return is in f0 (copy of f1 via `mov.s`), leave `c.f[0]` set; the Rust override must write FPU register 0.
- PS2 FPU semantics: `mul_s`, `div_s` (division by zero yields clamped max rather than inf/NaN), `min_s` (`F::min`, ordering by sign-magnitude key), compares use `c_lt_s`; do not use native f32 `min`/`<`.
- `div` operand order: `t / fadeIn` and `rem / fadeOut`; `min(a, b)` is called as min(quotient, weight) (fs = quotient).
- `mult` for the 0x1c stride clobbers a2/HI/LO and v0/v1/a1 (scratch); no memory writes.

#### `AnimSequence_SetTrackAnim` — 0x00313c50
*160 B · cycles: 30 (track already active) or 40 (track activated) · ops: FPU 8, VU0 0, 14 loads/stores*

Assigns an anim id to a track and computes its length. a0 = `AnimSequence*`, a1 = track index, a2 = anim id (`id & 0xff` = bank index, `id >> 8` = clip index). It stores the id at `track+4`, then looks up the clip: bank pointer = `*(*(gp+0xd08) + (id&0xff)*4)`, clip record = `*(bank+8) + (id>>8)*0x14`, frame count = `lhu clip+0xc`; length (`track+0x10`) = float(frameCount - 1) * `*(float *)(gp-0x3224)` (frame duration). If the track's active flag (`track+0x18`) was 0 it also initialises it: `+8 = 0` (time), `+0xc = 1.0f`, `+0x14 = 1.0f`, `+0x18 = 1`, `+0x1c = 0`, and sets `seq+0xc4 = 1` (dirty). No return value (v0 is a scratch pointer). 8 callers; in the race profile. Signature: `void AnimSequence_SetTrackAnim(AnimSequence *s, int track, uint animId)`.

**Gotchas:**
- Block costs: 28 for the main body; active-flag already set: +2 (30); active-flag clear: +10 for the init block, +2 (40).
- gp = 0x4a30f0: bank table pointer is the word at gp+0xd08 = 0x4a3df8 (zero in the static image, filled at runtime; load it live), a pointer to an array of bank pointers; frame scale is the float at gp-0x3224 = 0x4a1ecc (static image holds 0x0000355f, so it is written at runtime; read it live, do not hardcode).
- Frame count is `lhu` (unsigned 16-bit); `count - 1` is computed in 32 bits, so count 0 gives -1.0 * scale. `cvt.s.w` then `mul.s` with PS2 FPU semantics (`cvt_s_w`, `mul_s`), stored as raw bits.
- Index multiplies (`*0x1c`, `*0x14`) use `mult`, which also clobbers HI/LO and a1/a2 (scratch). Track index and clip index are signed 32-bit.
- Order of writes does not matter for the final state but the active flag test reads the flag after the length store.

#### `RiderHandplantState_Enter` — 0x00138b48
*84 B · cycles: 21 (single block) · ops: FPU 2 (lwc1, mfc1), VU0 4 (`sub`, `div`, `waitq`, `mul` by Q), 12 loads/stores incl. 128-bit `lqc2`/`sqc2`/`lq`/`sq`*

Enter hook of the Handplant main state. a0 = state object at `RiderBase+0x110` (vbase ptr at its `+0xa0` is the `Rider*`; `+0x30` vec4 target position; `+0x84` float duration). It computes `rider.velocity (Rider+0x1e0) = (target - Rider.pos (+0x110)) * (1.0 / duration)` using VU0: vector sub (all 4 lanes), `Q = vf0.w / duration` (`vdiv`), then `vmulq`; writes all 4 lanes (w included) to `Rider+0x1e0`. Return value unused. Signature: `void RiderHandplantState_Enter(HandplantState *s)`.

**Gotchas:**
- Block cost 21, single block; no per-path cost.
- Must use VU0 FMAC semantics (`ps2f::VuF32` / `Vec4`): `sub` then `mul` by Q. `Q = 1.0 / duration` is the VU divider (`vdiv` with fs = vf0.w, ft = lane x of the register loaded by `qmtc2` from the duration bits); its zero/denormal handling differs from FPU `div`; `VuF32` has no `div` yet, so it needs `ssx3_ee_rt::float` division added. VU divide-by-zero also sets VU status/Q flags (scratch).
- The `qmtc2` source register is a0, which was overwritten by `mfc1 a0, f0` (duration bits, upper 32 of the GPR sign-extended); only lane x matters for the divide.
- Stack scratch: the compiled code subtracts 0x30 from sp and leaves stale data in the three 16-byte slots just below the caller's sp: `[sp-0x30]` = result vec, `[sp-0x20]` = (target - pos), `[sp-0x10]` = result vec. A RAM-dump comparison would see this; either write them or accept a diff in dead stack.
- 16-byte `lqc2`/`sqc2` addresses are alignment-masked; the target (+0x30 of the state object) and Rider+0x110/+0x1e0 are aligned.
- VU0 registers vf3/vf4/vf5 and Q are clobbered (callers never read).

#### `RiderRailslideState_Exit` — 0x0013c5a0
*60 B · cycles: 15 (single block) · ops: FPU 7 (mtc1, lwc1, swc1; no arithmetic), VU0 0, 10 loads/stores*

Exit hook of the Railslide main state. a0 = state object at `RiderBase+0x0b0`; `Rider* = *(a0+0x50)` (vbase pointer at `+0x100`). It resets the slew parameter #9 (`Rider+0x25c` cur, `+0x260` maxStep, `+0x264` target; slews are 12 bytes from `Rider+0x1f0`) to cur = -0.25 (0xbe800000), maxStep = float at `0x49bff8` (gp-0x70f8, 0.0333 = 1/30), target = 1.0 (0x3f800000), and clears `Rider+0x5ac` (rail flag) to 0. No return value. Signature: `void RiderRailslideState_Exit(RailslideState *s)`.

**Gotchas:**
- Single block, `c.cycles += 15`.
- Only final memory matters: `+0x260` is first zeroed then overwritten with the gp constant, `+0x264` first -0.25 then 1.0; the intermediate writes are unobservable.
- The constant at 0x49bff8 is read from memory (static value 0x3d088889); read it live or hardcode the bits and check against the replay.
- All stores are raw bit copies (no float arithmetic); `lui`/`mtc1` constants go in as exact bit patterns.
- Rider pointer is re-loaded from `a0+0x50` for each group of writes (three loads); the pointer cannot change in between.

#### `RiderRailslideSub_Enter` — 0x00131d08
*40 B · cycles: 10 (single block) · ops: FPU 2 (lwc1/swc1), VU0 0, 8 loads/stores*

Enter hook of the Railslide sub-state. a0 = sub-state object at `RiderBase+0x2b0` (vbase pointer at `+0x2b8`, so `Rider* = *(a0+8)`). It sets state-local `+0x00 = 0` and `+0x04 = -1`, then writes `Rider+0x360 = 0`, `Rider+0x200` (slew #1 maxStep) = float at `0x49bb44` (gp-0x75ac; static 0x3d888889 = 1/15) and `Rider+0x204` (slew #1 target) = 0. No return value. Signature: `void RiderRailslideSub_Enter(RailslideSub *s)`.

**Gotchas:**
- Single block, `c.cycles += 10`.
- The constant is a raw 32-bit copy from gp-0x75ac (0x49bb44); read it live.
- `+0x04` is set to 0xffffffff (i32 -1) with a 64-bit `-1` in v1; store the low 32 bits.
- Order of writes is immaterial for the final memory state.
- `Rider+0x360` has no documented name yet (near the spin accumulator at +0x350).

#### `RiderBoardPressSub_Exit` — 0x0012fe98
*44 B · cycles: 11 (single block) · ops: FPU 4 (lwc1/swc1), VU0 0, 10 loads/stores*

Exit hook of the BoardPress sub-state. a0 = sub-state object at `RiderBase+0x1d0` (vbase pointer at `+0x1e4`, so `Rider* = *(a0+0x14)`). It clears `Rider+0x330` (trick flag) to 0 and resets two slews: `Rider+0x278` (slew #11 maxStep) = float at `0x49ba1c` (gp-0x76d4, static 0x3c888889 = 1/60), `+0x27c` (target) = 0, and `Rider+0x26c` (slew #10 maxStep) = float at `0x49ba20` (gp-0x76d0, static 0x3d088889 = 1/30), `+0x270` (target) = 0. No return value. Signature: `void RiderBoardPressSub_Exit(BoardPressSub *s)`.

**Gotchas:**
- Single block, `c.cycles += 11`.
- Constants are raw 32-bit copies from the two adjacent gp words; read live or hardcode and verify.
- The Rider pointer is reloaded from `a0+0x14` for each group (3 loads), pointer identical.
- Slew #11 (`+0x274`) is read by tricks/camera, so the maxStep reset matters for later frames, but only this function's writes need reproducing.

**Package notes.** Shared typed views needed: extend `rider::Rider` with `owner()` (`+0x77c`), `model()` (`+0x780`), `focus_box()` (`+0x78c`), `load_step()` (`+0x880`), a `Slew` view (`Rider+0x1f0 + 12*i`: cur, maxStep, target) and a `RiderBase` view (`main_state` `+0xde0`, `sub_state` `+0xde4`); a small `AnimSequence` view (tracks at `+0x1c*i`: +4 id, +8 time, +0xc rate, +0x10 length, +0x14 weight, +0x18 active; seq +0x94 weight, +0x98 target, +0x9c fade time, +0xa0 remove flag, +0xa4/+0xa8 fade out/in lengths, +0xc4 dirty). An `ssx3_ee_rt` FPU-ops wrapper (or direct `c.add_s/mul_s/div_s/min_s/c_lt_s`) and a VU divide in `ps2f` are the only new numeric helpers; the anim functions use FPU, the Handplant Enter uses VU0. Suggested order: trivial getters first (GetMainState, GetSubState, IsLoaded, UpdateFocusBox, PostUpdate no-op, GetLayerAnimId, AnimTable_GetEntry), then the memory-only setters (SetTrackTime, SetWeightFade, ResetBoostMeter, state Exit/Enter slew resets), then FPU functions (FadeOut, UpdateWeightFade, GetTrackBlendWeight, SetTrackAnim), HandplantState_Enter last (VU0 divide plus stack scratch). GetMainState/GetSubState can share one helper; the three slew-reset hooks (Railslide Exit/Sub_Enter, BoardPress Exit) can share a `Slew::set` helper; FadeOut/SetWeightFade/UpdateWeightFade all work on the same four AnimSequence fade fields.

### A3 Camera

Camera matrices, chase-camera bookkeeping and shake. `CameraBase_UpdateMatrices` is the VU0 counterpart of `Rider_BuildWorldMatrix` and the best next VU0 target. Indirect-call camera functions (`ChaseCameraAlgorithm_InitFromRider`, `_Reset`, `GameCamera_SetTargetRider`, `RiderCameraTarget_GetVelocity`, `ManualCamController_Update`) moved to Tier B2.

| addr | name | size | cycles | kind | race | callers |
|---|---|---:|---|---|---|---:|
| 0x15d928 | `CameraBase_UpdateMatrices` | 408 | 102 | VU0+FPU |  | 2 |
| 0x15d698 | `CameraBase_ctor` | 100 | 25 | FPU |  | 2 |
| 0x1673a0 | `ChaseCameraAlgorithm_GetView` | 24 | 6 | int+mem | c | 2 |
| 0x162b80 | `ChaseCameraAlgorithm_SetLookAtFromRider` | 12 | 3 | int+mem | c | 7 |
| 0x162138 | `ChaseCamController_ResetState` | 56 | 14 | int+mem |  | 2 |
| 0x15e360 | `GameCamera_AddShake` | 256 | ≤64 (15) | FPU | c2 | 3 |
| 0x15dfd8 | `GameCamera_SetController` | 84 | ≤21 (9) | int+mem |  | 5 |
| 0x15e2a8 | `GameCamera_InitShakePresets` | 180 | 45 | FPU |  | 1 |
| 0x1603f0 | `CameraWobble_GetAngle` | 72 | ≤18 (3) | FPU |  | 4 |
| 0x162310 | `RiderCameraAlgorithm_SetTarget` | 8 | 2 | int+mem |  | 1 |
| 0x1694b0 | `ScriptCameraAlgo_GetScriptObj` | 8 | 2 | int+mem |  | 1 |
| 0x1694a8 | `ScriptCameraAlgo_SetScriptObj` | 8 | 2 | int+mem |  | 1 |
| 0x16c390 | `CameraTriggerMan_ResetHeader` | 24 | 6 | int+mem |  | 3 |

#### `CameraBase_UpdateMatrices` — 0x0015d928
*408 B · cycles: 102 (single block) · ops: VU0 47, FPU 17 (neg_s/mtc1/lwc1/swc1 only), 44 loads/stores (23 stores)*

Builds the world-to-camera view matrix of a `GameCamera` (a0). It first copies a constant 4x4 matrix C from 0x4c53a0..0x4c53df (loaded with lqc2, stored to `GameCamera+0x40..0x7f`; the contents are written at runtime, probably the Z-up to camera axis permutation, so read them from memory). It then turns the quaternion `rot` (`GameCamera+0x30`, x y z w) into a 3x3 rotation matrix R in row-vector layout and stores it on the stack: row0 = (1-2y²-2z², 2xy+2zw, 2zx-2yw, 0), row1 = (2xy-2zw, 1-2z²-2x², 2yz+2xw, 0), row2 = (2zx+2yw, 2yz-2xw, 1-2x²-2y², 0), row3 = vf0 = (0,0,0,1). The view rows become R·C (new row i = C0·R[i].x + C1·R[i].y + C2·R[i].z + C3·R[i].w, a MULA/MADDA/MADD chain per row), written back to `+0x40/+0x50/+0x60/+0x70`. Finally the translation row `+0x70` is replaced by (-eye.x, -eye.y, -eye.z, 1)·view (same 4-term chain, with `eye` = `GameCamera+0x20`, sign flipped with FPU `neg_s`, w = 1.0f), i.e. it adds -eye transformed by the rotated rows to row3. Input a0 = GameCamera*; no return value. Signature: `void CameraBase_UpdateMatrices(GameCamera *cam)`.

**Gotchas:**
- Single block, flat 102 cycles; no branches, no `c.event()`.
- All quaternion math is VU0 FMAC (`add/sub/mul/opmula/madd/msub/adda/suba/mula/madda`), each MADD/MSUB is one instruction (`VuF32::madd`/`msub`: product truncated, then sum truncated). Keep the exact order: `s=q+q` (xyz), `vf6=s*q` (2x²,2y²,2z²), `vf7=s*q.w`, then `opmula(s,q)` gives ACC=(2y·z, 2z·x, 2x·y) and a following `madd(vf0*vf0)` adds 0 (xyz, so -0 and rounding behave like a plain add). The diagonal terms are `suba(1 - 2·a²)` then `msub(... 2·b²)`, i.e. (1-2y²)-2z², not 1-(2y²+2z²). The off-diagonals are `adda(0 + 2ab)` then `madd(vf1(=1.0) * 2cw)`; vf1.xyz is set to 1.0 by `add(vf0.xyz + vf0.w)`, vf1.w keeps the old constant-row value but is never used in the products.
- ACC lane masks matter: each ACC op writes one lane; the w lanes of vf10..12 are zeroed by `sub(v,v)`.
- Matrix multiply is full 4 lanes (mask 0b1111), ACC is left as the last row result.
- `neg_s` is a bit flip (eye 0.0 gives -0.0); eye.w (+0x2c) is not read.
- Scratch left behind: vf1, vf4..vf15, ACC, FPU f0..f6 (f1 = 1.0), and guest stack memory sp+0x00..0x6f (R rows, 0,0,0,1, the (-e,1) vectors at sp+0x40/+0x50/+0x60). The stack bytes are dead but appear in a RAM dump of the stack; reproduce them if the end-of-run dump must stay identical. GPRs v0 (=0x4c53a0), v1 (=cam+0x40), at are clobbered.
- lqc2/sqc2 need 16-byte alignment (the `GameCamera` fields are aligned).

#### `CameraBase_ctor` — 0x0015d698
*100 B · cycles: 25 (single block) · ops: FPU 3 (mtc1, 2 swc1), 15 loads/stores (12 stores)*

Constructor of the camera base object (a0 = GameCamera*). Sets the vptr at `+0x90` to 0x45b8b0 (CameraBase vtable; `GameCamera_ctor` later overwrites it with 0x45b888), `eye` (`+0x20`) = (0,0,0,0), `rot` (`+0x30`) = (0.0, 1.0, 0.0, 1.0) (not a unit quaternion; the y and w words are 0x3f800000), and copies 16 bytes from 0x500000-0xec0 = 0x4ff140 into `forward` (`+0x80`). Returns a0 in v0. Signature: `GameCamera *CameraBase_ctor(GameCamera *cam)`.

**Gotchas:**
- Single block, 25 cycles.
- Uses `sp-0x10` as scratch: it writes (0, 1.0, 0, 1.0) to the 16 bytes at the old sp (zeros first, then words 1 and 3 = 1.0f). Dead stack bytes, but present in a RAM dump.
- The vptr is stored as the 32-bit value 0x45b8b0 (zero-extended is fine, value is positive); `+0x20` and `+0x30` are written with 16-byte `sq` (needs alignment, the object is 16-byte aligned); `+0x80` is read with `lq` from 0x4ff140 (static data, 16-aligned).
- No other fields (`+0x00..+0x1f`, `+0x40..+0x7f`) are touched.
- GPR upper halves: `lq`/`sq` use the full 128 bits of a3, a1, v1 (registers 7, 5, 3); a2, v1, at, a3, a1 end up as scratch, v0 = a0.

#### `ChaseCameraAlgorithm_GetView` — 0x001673a0
*24 B · cycles: 6 (single block) · ops: 4 loads/stores (2 lq, 2 sq)*

Virtual slot 2 `GetView(out)` of `ChaseCameraAlgorithm`. a0 = out buffer (32 bytes: vec4 eye, quat rot), a1 = algorithm (`ChaseCameraAlgorithm`). Copies the output eye `algo+0x60` (16 B) to `out+0x00` and the output orientation `algo+0x70` (16 B) to `out+0x10`; returns `out` in v0. Signature: `void *ChaseCameraAlgorithm_GetView(CameraView *out, const ChaseCameraAlgorithm *algo)`.

**Gotchas:**
- Flat 6 cycles. Quadword copy: both buffers must be 16-byte aligned (the generated `lq/sq` would fault/mask otherwise), exact bits copied, no float ops.
- Registers clobbered: a2 (reg 6) and v1 (reg 3) get the full 128-bit loaded values; v0 = a0.
- Because out and algo+0x60 could overlap in principle, load the first quad, store it, then load the second (generated order is lq, sq, lq, sq).

#### `ChaseCameraAlgorithm_SetLookAtFromRider` — 0x00162b80
*12 B · cycles: 3 (single block) · ops: 2 loads/stores (1 lq, 1 sq)*

Copies a vec4 from a1+0x10 to `algo+0x20` (the look-at point). a0 = algorithm, a1 = a record whose +0x10 is the rider position (the `RiderCamState` built by `GatherRiderState`, +0x10 = rider position). No return value. Signature: `void ChaseCameraAlgorithm_SetLookAtFromRider(ChaseCameraAlgorithm *algo, const RiderCamState *st)`.

**Gotchas:**
- 3 cycles. A single 128-bit copy (w lane included, bit-exact); both addresses 16-byte aligned.
- v0 (reg 2) is left holding the loaded 128 bits.
- Hot: 7 callers.

#### `ChaseCamController_ResetState` — 0x00162138
*56 B · cycles: 14 (single block) · ops: 7 loads/stores (2 loads, 5 stores)*

Resets a `ChaseCamController` (a0): `+0x34 = 0`, `+0x28 = +0x24` (copy of the restore-algorithm id), `+0x3c = 0` (spoke mode off), `+0x30 &= ~7` (clears flag bits 0, 1, 2), `+0x40 = 0`. No return value. Signature: `void ChaseCamController_ResetState(ChaseCamController *ctl)`.

**Gotchas:**
- 14 cycles flat. `+0x30` is a 32-bit load and the mask is `0xfffffff8` (done as three ANDs with ~2, ~1, ~4, same result); store as u32.
- The generated code reads `+0x24` and `+0x30` before storing; no aliasing concerns.
- Scratch: v0, v1, a1 (=-3), a2 (=-2), a3 (=-5) are clobbered; v0 is not a meaningful return value.

#### `GameCamera_AddShake` — 0x0015e360
*256 B · cycles: path-dependent, 64 total over 15 blocks (see below) · ops: FPU 24, 10 loads/stores (3 stores)*

Registers a camera shake request for the frame. Inputs: f12 = intensity, f13 = distance (or time) value, a0 = GameCamera*, a1 = preset index. The function first loads `*(gp-0x848)` (module pointer, gp = 0x4a30f0 so 0x4a28a8), then `[+0x84]`, `[+0x28]`, and reads the int state `s` pointed to. If `s` is in 1..9 and `(a1-1) < 4` (unsigned, i.e. preset 1..4) the request is ignored entirely (nothing is written). Otherwise it computes scale = 1.0 when f13 <= 0 or table range <= 0, else scale = clamp(2.0 - (f13+f13)/range, 0, 1), with `range` = float at `0x4c53e0 + a1*16 + 0xc`. It then forms `str = (f12 * scale) * table[a1].amp` (amp at `+0x8`), and if `cam+0x454 < str` writes `cam+0x450 = a1`, `cam+0x454 = str`. In all non-ignored cases it sets `cam+0x458 = 1` (shakeRequested). No return value. Signature: `void GameCamera_AddShake(float intensity, float dist, GameCamera *cam, int preset)`.

**Gotchas:**
- Per-block cycles: entry 7 (+1 more if s != 0); block 2: 2 (+3 if s in 1..9, and the skip exit then goes straight to the tail); scale block: 10 (this block always runs on the non-ignored path), +3 if f13 > 0, then +11 if range > 0 (the 2.0 - 2*f13/range computation) followed by the clamp chain +4, +4, +2 (not all taken); +2 when the scale is forced to 1.0; compare/store block 10, +2 if the new strength wins the compare; then +1 for the `+0x458` store; return +2. Charge exactly these per path.
- The clamp is a chain of FPU compares: `f0 <= 1.0`? else f2 = 1.0; `0.0 <= f0`? else f2 = 0.0; else f2 = f0. Note the delay-slot `mov` executes in every case, so branch-taken paths set f2 to 1.0 / 0.0 as listed. Ghidra's decompile shows the clamp slightly muddled; follow the Rust.
- FPU single-precision semantics: `add_s`, `div_s` (no divide-by-zero guard besides the range > 0 test), `sub_s`, `mul_s` order is `(f12*scale)*amp`; compares are `c.lt`/`c.le` on the FPU condition bit, not VU. All in PS2 FPU semantics (no denormals, clamped inf), use the FPU helpers from `fpu.rs`.
- The a1 range check uses an unsigned compare of `a1-1 < 4`, so a1 = 0 or a1 >= 5 is never ignored, and the table index is unchecked (reads past 0x4c5430 for a1 >= 5).
- Scratch: FPU f0..f4, v0/v1/a2/a3/at. FPU f12/f13 are float arguments (f_in = [12, 13]).
- The `*(gp-0x848)` pointer is read with `lw` (a32 sign-extended pointer, dereferenced thrice).

#### `GameCamera_SetController` — 0x0015dfd8
*84 B · cycles: path-dependent, 21 total over 9 blocks; per call 6 (a1=2), 9 (a1=1 or a1<=0), 10 (a1=3 or a1>3) · ops: 6 loads/stores (3 loads, 3 stores)*

Selects the active camera controller: a0 = GameCamera*, a1 = mode 1/2/3. `+0xa0 = +0xa4` (manual) for 1, `+0xa8` (chase) for 2, `+0xac` (script) for 3; any other value does nothing. No return value. Signature: `void GameCamera_SetController(GameCamera *cam, int mode)`.

**Gotchas:**
- Branch costs: entry 3; a1 == 2 then +3 (total 6, load+store at the exit); else +2, then a1 >= 3: +3 (and +2 for either the a1 == 3 write exit or the nothing exit, total 10); a1 < 3: +2, then a1 == 1: +2 (write, total 9) else +2 (total 9, nothing written).
- The generated store goes in the return delay slot so the write order relative to other effects is only the one store; pointers are 32-bit loaded with `lw` (sign-extended in v0).
- v0 is scratch (0x2/0x1/0x3 or the loaded pointer); no return value.

#### `GameCamera_InitShakePresets` — 0x0015e2a8
*180 B · cycles: 45 (single block) · ops: FPU 33 (13 lwc1, 20 swc1, 1 mtc1), 37 loads/stores (25 stores)*

One-time initialiser of the shake preset tables. Writes the 5-entry (16 bytes each) float table at 0x4c53e0 (+0x8 = amplitude, +0xc = range, as read by `GameCamera_AddShake`): 12 float constants are loaded from gp-0x6cb0..gp-0x6c84 (words at 0x49c440..0x49c46c) and stored into the entries (entry 0 gets four, entries 1..3 share the pair gp-0x6cac/-0x6ca8 in their first two words and a different pair in +0x8/+0x18..., entry 4 gets three), and the range field of entries 1..4 (+0x1c, +0x2c, +0x3c, +0x4c) is set to -1.0f (0xbf800000, meaning "no range scaling"). A second table of 5 ints at 0x4c5430..0x4c5440 gets {0, 1, 1, 1, 1}. No arguments or return. Signature: `void GameCamera_InitShakePresets(void)`.

**Gotchas:**
- 45 cycles flat; pure stores of gp-relative floats, no arithmetic. Reproduce the exact source offsets: f3=gp-0x6cb0 -> 0x4c53e0, f2=gp-0x6cac, f1=gp-0x6ca8, f6=-0x6ca4, f4=-0x6ca0, f5=-0x6c9c, f7=-0x6c98, f8=-0x6c94, f9=-0x6c90, f10=-0x6c8c, f11=-0x6c88, f12=-0x6c84 (see the generated body for which table word each store goes to; f2/f1 are stored three times at +0x10/+0x14, +0x20/+0x24, +0x30/+0x34).
- Use raw 32-bit word moves (the floats are copied through FPU registers, no arithmetic, so bit patterns are preserved; -1.0 comes via `mtc1` of 0xbf800000).
- Scratch: f0..f12, at, v0, v1, a0, a1, a2.

#### `CameraWobble_GetAngle` — 0x001603f0
*72 B · cycles: path-dependent, 18 total over 3 blocks (7 if amplitude == 0, else 16) · ops: FPU 13, 6 loads (no stores)*

Computes a wobble angle from a wobble object (a0, camera-wobble fields): if `+0x1c` (amplitude) == 0.0 returns 0.0, otherwise returns `amp * ((+0x24 / +0x2c) * K) * (+0x14 - +0x08)` where K = float at gp-0x6c1c = 0x49c4d4 (a constant, probably degrees-to-radians or a scale). Result in f0. Signature: `float CameraWobble_GetAngle(const CameraWobble *w)`.

**Gotchas:**
- Block costs: entry 5; zero amplitude exit +2 (total 7); else +11 (total 16).
- Operation order is fixed: t = (+0x24 / +0x2c); t = t*K; t = amp*t; result = t*(+0x14 - +0x08). The division has no zero check: `div_s` PS2 FPU semantics (x/0 gives clamped max float with the divide flags, not IEEE inf) so use the FPU helper, not native `/`.
- The zero test is an FPU compare `c.eq.s amp, 0.0` (so -0.0 counts as zero; NaN would not).
- Return value in f0 (the zero path returns the 0.0 loaded into f0); f1..f4 are scratch. No integer return.
- 4 callers; mixed paths so the cycle charge differs per path.

#### `RiderCameraAlgorithm_SetTarget` — 0x00162310
*8 B · cycles: 2 (single block) · ops: 1 store*

Virtual slot 12 `SetTarget`: stores a1 (a `RiderCameraTarget*`) into `algo+0x30`. a0 = algorithm. No return. Signature: `void RiderCameraAlgorithm_SetTarget(RiderCameraAlgorithm *algo, RiderCameraTarget *tgt)`.

**Gotchas:**
- 2 cycles; a 32-bit `sw` of the low word of a1.
- Nothing else is clobbered.

#### `ScriptCameraAlgo_GetScriptObj` — 0x001694b0
*8 B · cycles: 2 (single block) · ops: 1 load*

Returns the script object pointer stored at `algo+0x14` (a0 = ScriptCameraAlgo*). Return in v0. Signature: `ScriptObjCam *ScriptCameraAlgo_GetScriptObj(const ScriptCameraAlgo *a)`.

**Gotchas:**
- 2 cycles; `lw` sign-extends to 64 bits in v0 (pointers below 0x80000000, so no difference).

#### `ScriptCameraAlgo_SetScriptObj` — 0x001694a8
*8 B · cycles: 2 (single block) · ops: 1 store*

Stores a1 (script object pointer) into `algo+0x14` (a0 = ScriptCameraAlgo*). No return value. Signature: `void ScriptCameraAlgo_SetScriptObj(ScriptCameraAlgo *a, ScriptObjCam *o)`.

**Gotchas:**
- 2 cycles; 32-bit `sw` of the low word of a1.

#### `CameraTriggerMan_ResetHeader` — 0x0016c390
*24 B · cycles: 6 (single block) · ops: 4 stores*

Resets the header of a camera trigger manager (a0): `+0x10 = 0`, `+0x0c = 7`, `+0x04 = 0`, `+0x08 = 0`. No return value. Signature: `void CameraTriggerMan_ResetHeader(CameraTriggerMan *m)`.

**Gotchas:**
- 6 cycles flat; four 32-bit stores; `+0x00` is not touched. v0 is clobbered with 7.


Shared pieces needed: a `GameCamera` typed view (eye `+0x20`, rot `+0x30`, view `+0x40`, shake fields `+0x450/+0x454/+0x458`, controller slots `+0xa0..0xac`), a `ChaseCameraAlgorithm` view (look-at `+0x20`, target `+0x30`, output eye/rot `+0x60/+0x70`), a `ChaseCamController` view (flags `+0x30`), and a `Vec4`/`Mat4` helper on top of `ps2f::VuF32` with `mula/madda/madd` row-times-matrix (`vmuladd4`) and quaternion-to-matrix. `UpdateMatrices` is the only real work (VU0 chain, byte-identical risk, plus the dead stack scratch it leaves); do it last, once the trivial setters pass. Easy first batch: `SetTarget`, `Get/SetScriptObj`, `ResetHeader`, `ResetState`, `SetController`, `SetLookAtFromRider`, `GetView`, `CameraBase_ctor` (note the stack scratch). Shared implementation: `GetView` and `SetLookAtFromRider` are the same 16-byte copy helper; the two shake functions share the table at 0x4c53e0 (`ShakePreset {…, amp, range}`) and need FPU-semantics float helpers (`add_s/sub_s/mul_s/div_s`, compare, `neg`), as does `CameraWobble_GetAngle`.

### A4 Render packets & graphics

GS packet builders (register writes as 64-bit A+D pairs), texture-upload packets and small graphics state. These are integer and 64-bit store heavy with little float math, and they define the `GsPacket` cursor type that phase 3's renderer interface will want.

| addr | name | size | cycles | kind | race | callers |
|---|---|---:|---|---|---|---:|
| 0x3625c0 | `GsPkt_AddClamp` | 160 | ≤40 (14) | int+mem | c | 2 |
| 0x3623d8 | `GsPkt_AddScissor` | 108 | 27 | int+mem | c | 1 |
| 0x3626d8 | `GsPkt_AddTest` | 204 | ≤51 (17) | int+mem | c | 2 |
| 0x362660 | `GsPkt_AddZbuf` | 120 | ≤30 (7) | int+mem | c | 2 |
| 0x367db8 | `TexMan_BuildUploadPacket` | 348 | 87 | int+mem | P | 1 |
| 0x367f18 | `TexMan_BuildSpritePacket` | 544 | 136 | FPU |  | 1 |
| 0x362978 | `DeferMan_AppendPacket` | 64 | ≤16 (3) | int+mem | c | 3 |
| 0x3826e0 | `PS2GraphicsMan_AcquireFrameBuffer` | 76 | ≤19 (6) | int+mem | c | 1 |
| 0x382730 | `PS2GraphicsMan_IsRenderIdle` | 12 | 3 | int+mem |  | 1 |
| 0x2e2fa8 | `LensFX_AddSource` | 80 | 20 | int+mem | c2 | 1 |
| 0x2e2f98 | `LensFX_ClearSources` | 16 | 4 | int+mem | c | 1 |
| 0x353d98 | `SkyBox_Clear` | 16 | 4 | int+mem | c2 | 2 |
| 0x38f6a8 | `SprStream_NextIndex` | 36 | ≤9 (2) | int+mem | c | 3 |
| 0x116fb8 | `cHUDFX_Update` | 80 | ≤20 (5) | FPU | c | 1 |
| 0x231d18 | `AppModule_UpdateFade` | 68 | ≤17 (5) | FPU | c | 5 |
| 0x3fdaa0 | `sceGsGetGParam` | 12 | 3 | int | c2 | 2 |

#### `GsPkt_AddClamp` — 0x003625c0
*160 B · cycles: 40 total over 14 blocks, path-dependent (see Gotchas) · ops: 0 FPU, 15 loads/stores (7 stores), one 64-bit sd*

Appends one GS A+D pair (CLAMP register) to a GS packet builder. `a0` = packet `{int count @+0, u32 cursor @+4}`, `a1` = wrap mode, `a2` = context selector. The 64-bit data qword is a lookup on `a1`: 0 -> 0, 1 -> 4 (WMT clamp), 2 -> 1 (WMS clamp), 3 -> 5 (both), anything else (>3, negative) -> 0. The register-descriptor qword is 8 (CLAMP_1) unless `a2 == 1`, then 9 (CLAMP_2). It does `sd(cursor, data); sd(cursor+8, reg); cursor += 0x10; count += 1` (both re-read from the packet each time, stored with `sw`). Returns nothing; inferred C: `void GsPkt_AddClamp(GsPacket *p, int mode, int ctx)`.

**Gotchas:**
- Cycles = sel(a2) + mode(a1) + 9. sel: a2==0 -> 3, a2==1 -> 6, other -> 4. mode: a1==2 -> 7, a1==1 -> 10, a1==3 -> 12, a1>3 (signed) -> 9, a1<=0 (incl. negative) -> 8. Compares are on the full 64-bit register (signed `< 3`).
- The data qword is written even for out-of-range modes (value 0), never skipped.
- Stores are 64-bit `sd` of data and reg id; count/cursor are 32-bit `sw` of the sign-extended sum (`v0` is left holding junk; nobody reads it).
- The two A+D qwords must be 8-aligned only; the cursor is not masked.

#### `GsPkt_AddScissor` — 0x003623d8
*108 B · cycles: 27 (single block) · ops: 0 FPU, 12 loads/stores (6 stores), 64-bit shifts/or, 2x sd pair*

Appends two A+D pairs, SCISSOR_1 (0x40) and SCISSOR_2 (0x41), with the same 64-bit value: `SCAX0 = x`, `SCAX1 = x-1+w` (<<16), `SCAY0 = y` (<<32), `SCAY1 = y-1+h` (<<48). Args: `a0` = packet `{count@0, cursor@4}`, `a1` = x, `a2` = w, `a3` = y, `t0` = h. Writes at cursor+0/8/0x10/0x18 (value, 0x40, value, 0x41), then `cursor += 0x20`, `count += 2`. Inferred C: `void GsPkt_AddScissor(GsPacket *p, u64 x, int w, u32 y, int h)`.

**Gotchas:**
- Value = `a1 | (sx32(a1-1+a2) << 16) | (a3 << 32) | (sx32(a3-1+t0) << 48)` in 64-bit ops. `a1` is OR-ed unmasked as a 64-bit register and `a3 << 32` is a 64-bit shift of the full register, so out-of-range values bleed into neighbour fields; the two 32-bit sums are sign-extended before the shift.
- `a1-1` is a 32-bit `addu` (sx) then added to a2 (32-bit, sx); same for y.
- Cursor and count are re-read from the packet between stores (no aliasing in practice); stored with `sw`. Single block, 27 cycles, no branches.

#### `GsPkt_AddTest` — 0x003626d8
*204 B · cycles: 51 total over 17 blocks, path-dependent · ops: 0 FPU, 8 loads/stores (4 stores), 64-bit sd*

Appends one A+D pair writing TEST_1 (0x47) or TEST_2 (0x48). Args: `a0` = packet, `a1` = ZTST selector, `a2` = ATST selector, `a3` = AREF, `t0` = context (1 -> 0x48, otherwise 0x47). Value = `(atst << 1) | ((a3 & 0xffffffff) << 4) | 1 | (ztst << 17) | 0x11000`, i.e. ATE=1, AFAIL=1 (bit 12), ZTE=1 (bit 16). atst: a2==2 -> 2, a2==3 -> 6, otherwise 1. ztst: a1==1 -> 2, a1==0 -> 3, otherwise 1. Then `sd(cursor, value); sd(cursor+8, reg); cursor += 0x10; count += 1`. Inferred C: `void GsPkt_AddTest(GsPacket *p, long zsel, int asel, u64 aref, int ctx)`.

**Gotchas:**
- Cycles = ctx + asel + zsel + 20. ctx(t0): t0==0 -> 3, t0==1 -> 7, other -> 5. asel(a2): ==2 -> 5, <3 and !=2 -> 6, ==3 -> 9, >3 -> 10. zsel(a1): ==1 -> 5, >=2 -> 5, ==0 -> 7, negative -> 8. All compares on the full 64-bit registers (signed `< 2`, `< 3`).
- AREF = `(a3<<32)>>28` (logical): the low 32 bits of a3 shifted left 4, not masked to 8 bits, so a3 > 255 overflows into AFAIL/DATE/ZTE bits (bits 12+).
- The register value 64-bit OR chain; `sd` at the cursor; count/cursor `sw`.

#### `GsPkt_AddZbuf` — 0x00362660
*120 B · cycles: 30 total over 7 blocks, path-dependent · ops: 0 FPU, 8 loads/stores (4 stores), 64-bit sd*

Appends one A+D pair writing ZBUF_1 (0x4e) or ZBUF_2 (0x4f). Args: `a0` = packet, `a1` = ZBP, `a2` = psm-like value, `a3` = ZMSK flag, `t0` = context (1 -> 0x4f, otherwise 0x4e). Value = `(a1 & 0xffffffff) | ((u32)(a2 - 0x30) << 24)`, plus bit 32 (`0x8000 << 17`) when `a3 == 1`. Then `sd(cursor, value); sd(cursor+8, reg); cursor += 0x10; count += 1`. Inferred C: `void GsPkt_AddZbuf(GsPacket *p, u32 zbp, int psm, int zmsk, int ctx)`.

**Gotchas:**
- Cycles = ctx + 9 + (a3==1 ? 3 : 0) + 10. ctx(t0): 0 -> 3, 1 -> 7, other -> 5. So e.g. t0=0, a3!=1 -> 22.
- ZBP is the full 32 low bits (not masked to 9 bits). PSM field = `(a2-0x30)` as a 32-bit value shifted `<<24` (computed as `(sx32(a2-0x30) << 32) >> 8` logical), unmasked, so a2 < 0x30 smears ones into bits 28..55.
- `a3` compared as a 64-bit register against 1. Value and reg id stored with `sd`; count/cursor with `sw`.

#### `TexMan_BuildUploadPacket` — 0x00367db8
*348 B · cycles: 87 (single block) · ops: 0 FPU, 19 loads/stores (13 stores), 64-bit sd, many dsll/dsrl*

Builds a 6-qword DMA packet (CNT qwc=5 + VIF FLUSH + DIRECT 4 qw + GIF tag NLOOP=3 EOP A+D + 3 A+D pairs) that points the GS at a texture before upload. Inputs: `a0` = TexMan, `a1` = slot (tex = `*(a0 + a1*4 + 8)`), `a2` = `u32 *cursor` (in/out; `p = *a2`). Layout (u64 index): p[0]=0x10000005, p[1]=0, p[2]=0x1100000000000000, p[3]=0x5000000400000000, p[4]=0x1000000000008003, p[5]=0xe, then FRAME_1 (reg 0x4c) at p[6]/p[7], SCISSOR_1 (0x40) at p[8]/p[9], XYOFFSET_1 (0x18) at p[10]/p[11]. With `tex0 = ld(tex+0x38)`, `tw = (tex0>>26)&0xf`, `th = (tex0>>30)&0xf`, `psm = (tex0>>20)&0x3f`: FRAME = `(lw(tex+0x28) >> 5) | (((1<<tw)>>6) << 16) | (psm << 24)`; SCISSOR = `((1<<tw)-1 << 16) | ((1<<th)-1 << 48)`; XYOFFSET = `((0x800 - ((1<<tw)>>1))*16) | (((0x800 - ((1<<th)>>1))*16) << 32)`. Finally `*a2 = p + 0x60` (`sw`). Inferred C: `void TexMan_BuildUploadPacket(TexMan *tm, int slot, u64 **cursor)`.

**Gotchas:**
- Single block, 87 cycles, no branches. Returns nothing.
- `tex0` (u64 at tex+0x38) is re-read with `ld` three times between stores; same value, so read once. `lw(tex+0x28)` is sign-extended and arithmetically shifted `>> 5` (`sra`), so a negative address sets high FRAME bits (full 64-bit OR, no mask).
- Shifts of `1 << n` are 32-bit `sllv` with the low 5 bits of n (n is already 0..15); `>> 6` and `>> 1` are arithmetic 32-bit.
- XYOFFSET: OFX (low) is the sign-extended 64-bit result of the 32-bit `<<4`; if tw >= 12 it goes negative and the OR would overwrite OFY's bits, mimic by OR-ing the sign-extended value. FRAME FBW/psm terms and the XYOFFSET halves are OR-ed unmasked.
- All stores are 64-bit `sd`; p must be 8-aligned (really 16: DMA tag).

#### `TexMan_BuildSpritePacket` — 0x00367f18
*544 B · cycles: 136 (single block) · ops: 19 FPU (4 mul.s, 4 cvt.w.s, lwc1/mtc1/mfc1), 28 loads/stores (17 stores), 64-bit sd, 2 sq/2 lq stack spill*

Builds a 7-qword DMA packet that draws a flat-coloured full-texture sprite: DMA CNT qwc=6, VIF DIRECT 5 qw, GIF tag (REGLIST NREG=4: PRIM, RGBAQ, XYZ2, XYZ2), a second GIF tag (A+D, EOP) with SCISSOR_1. Inputs: `a0` = TexMan, `a1` = slot (tex = `*(a0+a1*4+8)`), `a2` = `float rgba[4]` (indices a,r,g,b), `a3` = `u32 *cursor` (in/out). Words: p[0]=0x10000006, p[1]=p[2]=0, p[3]=0x5000000500000000, p[4]=0x4400000000000001, p[5]=0x5510, p[6]=0x10e (PRIM: sprite, Gouraud, FST), p[7]=RGBAQ, p[8]=XYZ2 corner 1, p[9]=XYZ2 corner 2, p[10]=0x1000000000008001, p[11]=0xe, p[12]=SCISSOR value, p[13]=0x40. RGBAQ = `int(f[1]*128) | int(f[2]*128)<<8 | int(f[3]*128)<<16 | int(f[0]*128)<<24 | 0x3f80000000000000` (Q=1.0f). With `tw,th` from `ld(tex+0x38)` as in the upload packet and `w=1<<tw, h=1<<th`: corner1 = `((0x800-(w>>1))*16) | ((0x800-(h>>1))*16)<<16 | 1<<32`; corner2 = `(((w>>1)+0x801)*16) | (((h>>1)+0x801)*16)<<16 | 1<<32`; SCISSOR = `((w-2)<<16) | ((h-2)<<48) | 0x100000001` (SCAX0=1, SCAY0=1). Writes `*a3 = p + 0x70`. Inferred C: `void TexMan_BuildSpritePacket(TexMan *tm, int slot, const float *rgba, u64 **cursor)`.

**Gotchas:**
- Single block, 136 cycles. FPU: `0x43000000` (128.0f) via `mtc1`; the four products use `mul_s` (PS2 FPU semantics), converted by `cvt_w_s` which truncates toward zero and saturates (`float::to_fixed`). No FPU compare.
- `mfc1` sign-extends to 64 bits and the OR chain is unmasked: a colour component that converts negative or >255 smears into higher fields (and a negative one sets all upper bits including Q). Keep the 64-bit OR; do not mask to bytes.
- Float read order: a2+8, +0xc, +0, +4 (does not matter, no side effects).
- The function spills s0/s1 with `sq` to `sp-0x20` and `sp-0x10` and restores them with `lq`; the 32 stack bytes below sp (and s0/s1 values) end up in RAM, so a byte-identical RAM dump needs those two 128-bit stores (upper 64 bits of s0/s1 included; use `c.sq`/`c.lq`). s0 is also used as scratch (=0x100000001) and restored.
- `tex0` is re-read with `ld` per field (same value). `1<<n` is 32-bit `sllv`, `>>1` arithmetic; the FPU scratch regs f0..f5 and many GPRs are left modified (scratch).

#### `DeferMan_AppendPacket` — 0x00362978
*64 B · cycles: 6 (bucket empty) or 14 (bucket has packets), path-dependent · ops: 0 FPU, 7 loads/stores (5 stores), 64-bit sd*

Links a freshly built packet range into a DeferMan bucket's DMA chain. `a0` = bucket (0x20 struct: +0xc first packet start, +0x10 last packet end), `a1` = new packet start, `a2` = new packet end. If `bucket[+0xc] == 0` it records `bucket[+0xc] = a1; bucket[+0x10] = a2` and returns. Otherwise it patches the previous end with a DMA NEXT tag: at address `bucket[+0x10] | 0x30000000` it writes `sd(tag = (a1 << 32) | 0x20000000)` and `sd(+8, 0)`, then sets `bucket[+0x10] = a2`. Returns nothing. Inferred C: `void DeferMan_AppendPacket(Bucket *b, u32 start, u32 end)`.

**Gotchas:**
- Cycles: first-packet path 4+2 = 6; chained path 4+10 = 14.
- The patch address is the previous end OR 0x30000000 (the loaded word is sign-extended, then OR-ed in 64-bit, then truncated to u32), a different mapping than the plain pointer. The tag's address field is `a1 << 32` done as a 64-bit shift of the full register (a1 is a u32 address; if its upper register bits are set they would carry, normally clean).
- Stores: two `sd` for the NEXT tag (second qword half 0) and `sw` for +0x10 / +0xc. The empty path stores +0x10 before +0xc (immaterial).

#### `PS2GraphicsMan_AcquireFrameBuffer` — 0x003826e0
*76 B · cycles: 13 (state 0 or other), 16 (state 1), path-dependent · ops: 0 FPU, 5 loads/stores, 1 store*

Tries to claim the current frame buffer slot. `a0` = PS2GraphicsMan (gm). `idx = lw(gm+0x5a10)`, `p = gm + 0x5a90 + idx*4` (int state array). If the state is not 0 or 1 it returns 0; otherwise it sets state 0 -> 1 (`sw`) and returns 1. Inferred C: `int PS2GraphicsMan_AcquireFrameBuffer(PS2GraphicsMan *gm)`.

**Gotchas:**
- Cycles: state==0 -> 7+3+1+2 = 13 (store); state==1 -> 7+4+3+2 = 16 (no store); other -> 7+4+2 = 13 (returns 0).
- `v0` is the return value: `1u64` or `0u64`. `idx` is not bounds-checked (index `<<2` 32-bit).
- The state is loaded twice (second `lw` of the same word); no hazard, but a replacement must not move the state store before the state read.

#### `PS2GraphicsMan_IsRenderIdle` — 0x00382730
*12 B · cycles: 3 (single block) · ops: 0 FPU, 1 load, 0 stores*

Returns whether the render pipeline is idle: `v0 = (u32)lw(gm+0x5a8c) < 1` (i.e. equals 0), as 0 or 1. `a0` = PS2GraphicsMan. Inferred C: `int PS2GraphicsMan_IsRenderIdle(PS2GraphicsMan *gm)`.

**Gotchas:**
- Unsigned `sltiu` compare on the 64-bit sign-extended load, so only exactly 0 yields 1 (the result is 0/1 in `v0`, full-width).
- No stores, single block.

#### `LensFX_AddSource` — 0x002e2fa8
*80 B · cycles: 20 (single block) · ops: 0 FPU, 7 loads/stores (5 stores), 2 mult (3-operand), sh/sw*

Appends a lens-flare source record to one of several lists. `a0` = LensFX object, `a1` = value stored in the record, `a2` = list index. `cnt = a0 + 0x60 + a2*4` (int per-list count), `rec = a0 + cnt_val*0x30 + a2*0x3000`; writes `sw(rec+0x88, a1)`, `sh(rec+0x96, 0)`, `sh(rec+0x94, 0)`, `sw(rec+0x90, 0)` and then `*cnt += 1` (re-loaded, `sw`). Inferred C: `void LensFX_AddSource(LensFX *l, u32 val, int list)`.

**Gotchas:**
- Single block, 20 cycles, no bounds check on the count.
- Two `c.mult(0, rd, rs, rt)` three-operand multiplies (`a2*0x3000`, `n*0x30`) also write HI/LO[0] (and A2 / V0 as rd); a bit-exact replacement should reproduce the final LO/HI of pipe 0 if anything could read them (unlikely; replays will tell).
- Address math is 32-bit `addu` with sign extension; store sizes: two 16-bit `sh` and two 32-bit `sw`.

#### `LensFX_ClearSources` — 0x002e2f98
*16 B · cycles: 4 (single block) · ops: 0 FPU, 1 store*

Resets a lens source list: `sw(a0 + 0x60 + a1*4, 0)`. `a0` = LensFX, `a1` = list index. Inferred C: `void LensFX_ClearSources(LensFX *l, int list)`.

**Gotchas:**
- The index is shifted as 32-bit (`sll 2`); no bounds check. Only the count is cleared, records are not touched.

#### `SkyBox_Clear` — 0x00353d98
*16 B · cycles: 4 (single block) · ops: 0 FPU, 2 stores*

Clears a SkyBox: `sw(a0+0x14, 0)` and `sw(a0+0x10, 0xffffffff)` (section id -1). `a0` = SkyBox. Inferred C: `void SkyBox_Clear(SkyBox *s)`.

**Gotchas:**
- The store order is +0x14 first, then +0x10 (immaterial). `v0` ends as `0xffffffffffffffff` (scratch).

#### `SprStream_NextIndex` — 0x0038f6a8
*36 B · cycles: 9 (single block, branch-merged) · ops: 0 FPU, 3 loads/stores (1 store), signed div*

Advances the ring index of a scratchpad stream: `obj+0xc = (obj+0xc + 1) % obj+4` (signed 32-bit remainder via `div`/`mfhi`). `a0` = SprStream, +4 = buffer count, +0xc = current index. Inferred C: `void SprStream_NextIndex(SprStream *s)`.

**Gotchas:**
- Both arms of the zero-divisor check join with the same 3 cycles (total 6+3 = 9); the recompiled code does NOT trap on a zero divisor, it uses `Cpu::div` semantics (divide by 0: remainder = dividend, `HI = n`). Ghidra shows `trap(7)` but that is not executed.
- Signed `div` with the `n == i32::MIN, d == -1` special case handled by `Cpu::div`; also writes LO/HI[0] (quotient in LO), scratch.
- The remainder is sign-extended then stored with `sw`.

#### `cHUDFX_Update` — 0x00116fb8
*80 B · cycles: 6 (state==0x34), 12 (target==-1.0), 19 or 20 (update; 20 if it sets the state) · ops: 8 FPU (lwc1 x3, add.s, c.eq.s, c.le.s, swc1, mtc1), 6 loads/stores (2 stores)*

Per-frame HUD effect timer. `a0` = obj `{int state @+0, float target @+4, float acc @+8}`. If `state != 0x34` and `target != -1.0f` (bit pattern 0xbf800000 via `c.eq.s`): `acc += g(0x49b5d8)` (float at gp-0x7b18, `add.s`), store `acc`, then if `target <= acc` (`c.le.s`) set `state = 0x34`. Returns nothing. Inferred C: `void cHUDFX_Update(HudFx *f)`.

**Gotchas:**
- Cycles: state==0x34 -> 4+2; target==-1 -> 4+6+2; else 4+6+7+(1 if state written)+2 = 19/20.
- `acc` is stored even when the state flips. Float ops must use the PS2 FPU helpers (`add_s`, `c_eq_s`, `c_le_s` with the `key()` ordering; the condition bit in FCR31 is also set, scratch).
- `target` is compared with `-1.0f` as a float equality (also matches -0 issues via `key`), not an integer compare. State compared as 32-bit sign-extended int.

#### `AppModule_UpdateFade` — 0x00231d18
*68 B · cycles: 12 or 13 (dir==0), 7 (dir!=0), path-dependent · ops: 0..10 FPU (lwc1 x2, sub.s/add.s, c.lt.s, mtc1, swc1), 7 loads/stores (3 stores)*

Fades a level up or down. `a0` = obj `{float level @+4, int dir @+8}`. If `dir == 0`: `level = level - g(0x49df44)` (float at gp-0x51ac, `sub.s`), stored, then if `level < 0.0f` (`c.lt.s` against `mtc1` of reg 0 = 0.0) stores 0.0 (bits 0). Else (`dir != 0`): `level = level + g(0x49df40)` (gp-0x51b0, `add.s`), stored. No upper clamp. Returns nothing. Inferred C: `void AppModule_UpdateFade(Fade *f)`.

**Gotchas:**
- Cycles: dir==0 and level>=0 -> 3+7+2 = 12; dir==0 and level<0 -> 3+7+1+2 = 13; dir!=0 -> 3+4 = 7.
- The unclamped result is stored before the clamp store overwrites it (two `swc1` on the same address); only the final value is visible.
- `dir` is a 32-bit int compared to 0; the level is loaded (`lwc1`) before the branch (in the delay slot). Use PS2 FPU add/sub (`F::add`/`F::sub`) and `key()` comparison for `-0.0`.

#### `sceGsGetGParam` — 0x003fdaa0
*12 B · cycles: 3 (single block) · ops: 0 FPU, 0 loads/stores*

Returns the address of the SDK global GS parameter block: `v0 = 0x451550` (sign-extended). No arguments, no memory access. Inferred C: `void *sceGsGetGParam(void)`.

**Gotchas:**
- Trivial: `c.g[V0][0] = 0x451550`, `cycles += 3`, `pc = ra`.

---

**Package notes.** Five functions (`GsPkt_AddClamp/AddScissor/AddTest/AddZbuf`, plus the A+D tail in `TexMan_BuildUploadPacket`/`BuildSpritePacket`) share the packet cursor `GsPacket { count: i32 @0, cursor: u32 @4 }`; a helper `push_reg(&mut GsPacket, value: u64, reg: u64)` doing `sd/sd/+0x10/+1` serves the four `GsPkt_*` builders (AddScissor uses it twice, +2). The conditional cycle costs must stay in each function (they differ per branch). TexMan upload/sprite share a `tex_dims(tex0) -> (tw, th)` helper (`>>26 &0xf`, `>>30 &0xf`) and the same 64-bit OR-unmasked rules. Suggested order: trivial ones first (IsRenderIdle, sceGsGetGParam, SkyBox_Clear, LensFX_Clear/Add, SprStream_NextIndex, AcquireFrameBuffer, the two float updates), then the GsPkt_* builders, DeferMan_AppendPacket, and finally the two TexMan packet builders (sprite needs `c.sq/lq` for the stack spill and FPU `mul_s`/`cvt_w_s`). Float-involving ones (cHUDFX_Update, UpdateFade, BuildSpritePacket) must use the `ssx3_ee_rt::fpu` helpers; globals: 0x49b5d8 (HUD step), 0x49df40/0x49df44 (fade up/down steps), 0x451550 (GParam).

### A5 Sound, streaming & files

Sound-driver bookkeeping (channel/handle lookup, intrusive lists, banks), stream and file validation, timers. Integer-only apart from `SND_ChannelCalcVolume`. `SND_Lock`/`SND_Unlock` and `EAFile_SetRequestCallback` call through function pointers and are in Tier B2.

| addr | name | size | cycles | kind | race | callers |
|---|---|---:|---|---|---|---:|
| 0x3b5440 | `SND_IsInitialized` | 12 | 3 | int+mem |  | 3 |
| 0x3ba020 | `SND_InitPlayParams` | 84 | 21 | int+mem |  | 5 |
| 0x3c6278 | `SND_ListInit` | 16 | 4 | int+mem |  | 1 |
| 0x3c6320 | `SND_ListRemove` | 96 | ≤24 (9) | int+mem |  | 1 |
| 0x3ba6e8 | `SND_HandleToChannel` | 88 | ≤22 (6) | int+mem | c | 20 |
| 0x3ba7b0 | `SND_ChannelCalcVolume` | 216 | ≤54 (5) | FPU | P | 5 |
| 0x3c62b8 | `SND_ListPushBack` | 48 | ≤12 (3) | int+mem | c | 1 |
| 0x3c6288 | `SND_ListPushFront` | 48 | ≤12 (3) | int+mem | c2 | 3 |
| 0x3c62e8 | `SND_ListPopFront` | 56 | ≤14 (6) | int+mem | c | 2 |
| 0x3c6540 | `SND_BankGetPatch` | 60 | ≤15 (5) | int+mem | c2 | 3 |
| 0x3b6528 | `SND_BankGetSize` | 28 | 7 | int+mem | c2 | 3 |
| 0x3b64d8 | `SND_BankValidate` | 80 | ≤20 (6) | int+mem | c2 | 4 |
| 0x3b8d98 | `SNDMIDI_GetSlot` | 76 | ≤19 (5) | int+mem | c2 | 8 |
| 0x3b6a38 | `SNDSTRM_GetStream` | 56 | ≤14 (4) | int+mem | c | 16 |
| 0x3d8008 | `SPCH_MakeId` | 28 | 7 | int+mem | c2 | 44 |
| 0x3dfbd0 | `StreamFile_Validate` | 56 | ≤14 (5) | int+mem | c | 20 |
| 0x3dfc08 | `StreamFile_RingInRange` | 64 | ≤16 (8) | int | c | 3 |
| 0x3def70 | `AsyncFile_FromHandle` | 76 | ≤19 (4) | int+mem | c2 | 4 |
| 0x3e2500 | `BigFile_GetType` | 128 | ≤32 (6) | int+mem | c2 | 5 |
| 0x3e4e98 | `EATimer_GetTicks` | 16 | 4 | int+mem | c | 11 |

Base addresses used below: the sound state block is at 0x50a8e8 (generated code builds it as `0x510000 - 0x5718`); fields are listed as absolute addresses. All `c.lw` results are sign-extended to 64 bits; `c.lb/lh` are sign-extended, `lbu/lhu` zero-extended.

#### `SND_IsInitialized` — 0x003b5440
*12 B · cycles: 3 (single block) · ops: 0 FPU, 1 load*

Returns the sound-system "initialised" flag. Reads the signed byte at 0x50aa5c (0x510000 - 0x55a4) and returns it in v0. Signature: `s8 SND_IsInitialized(void)`.

**Gotchas:**
- `lb`, so the result is sign-extended to 64 bits (a flag of 0xff reads as -1).
- v1 is left = 0x510000 (scratch, not observable).
- One block, flat 3 cycles.

#### `SND_InitPlayParams` — 0x003ba020
*84 B · cycles: 21 (single block) · ops: 0 FPU, 14 stores*

Fills a 0x18-byte play-parameter struct with defaults. a0 = `SndPlayParams*`. Byte fields: +0=0x7f, +1=0x40, +2=0x3c, +3=0x7f, +4=0x7f, +5=0x7f. Halfwords: +8=0, +0xa=0, +0xc=0x1000, +0xe=0x1000, +0x10=0x1000, +0x12=0, +0x14=0xffff, +0x16=0. Returns 0. Signature: `int SND_InitPlayParams(SndPlayParams* p)` (always returns 0).

**Gotchas:**
- Mixed widths: 6 `sb` and 8 `sh`; +6/+7 are NOT written (leave them untouched).
- Return value v0 = 0 (sign-extended 0).
- Flat 21 cycles, no branches.

#### `SND_ListInit` — 0x003c6278
*16 B · cycles: 4 (single block) · ops: 0 FPU, 3 stores*

Clears an intrusive list header: a0 = `SndList*` with layout {head +0, tail +4, count +8}; all three words set to 0. No return value (v0 not written). Signature: `void SND_ListInit(SndList* l)`.

**Gotchas:**
- Flat 4 cycles.
- Header layout is shared by the other `SND_List*` functions here: node = {next +0, prev +4}.

#### `SND_ListRemove` — 0x003c6320
*96 B · cycles: 12..24 total over 9 blocks, path-dependent · ops: 0 FPU, 13 loads/5 stores*

Unlinks node a1 from the doubly-linked list a0 (header {head +0, tail +4, count +8}; node {next +0, prev +4}). If the node is the head, head = node.next; if it is the tail, tail = node.prev; then prev.next = node.next (if prev != 0) and next.prev = node.prev (if next != 0); count -= 1. No return value and the node's own links are not cleared. Signature: `void SND_ListRemove(SndList* l, SndNode* n)`.

**Gotchas:**
- Cycle blocks: 3 (head compare) +3 if node==head; 2 (tail compare) +3 if node==tail; 2 (prev test) +3 if prev!=0; 2 (next test) +3 if next!=0; final 3. Range 12 (middle node with both neighbours is 3+2+5+5+3 = 18; the bare 12 needs no head/tail/prev/next, impossible for a real member) to 24 (all four taken).
- The tail comparison uses the tail value loaded in the first block (list+4), the prev test uses node+4 as loaded in the tail block; keep the same order of reads/writes (head update happens before the tail update).
- Count decrement is a 32-bit `sx(count-1)` store; no underflow check.
- No node membership check (removing a node not in the list corrupts it, same as the original).

#### `SND_HandleToChannel` — 0x003ba6e8
*88 B · cycles: 5 / 11 / 17 / 20 by path (22 summed over 6 blocks) · ops: 0 FPU, 4 loads, 1 mult*

Validates a 32-bit voice handle and returns the channel index. a0 = handle: low 8 bits = channel slot, the full 32-bit value must equal the stored id at channel+0. Channel table base is the pointer at 0x50aad8 (stride 0x8c), slot count is the signed halfword at 0x50aa66. Returns the slot index (handle & 0xff) on success, else -8. Signature: `int SND_HandleToChannel(int handle)` (-8 = invalid/stale handle). Many callers (20).

**Gotchas:**
- Returns the INDEX, not a pointer, and -8 is `0xfffffffffffffff8` (full 64-bit sign extension).
- Checks in order: handle < 0 (signed) -> -8 (5 cycles); slot >= count (signed `slt`, `lh` count) -> -8 (11 cycles); then `lb` of channel+0x69 (active flag) == 0 -> -8 (17 cycles); then `lw` channel+0 != handle (64-bit compare, so a handle with bit31 set can never match, but those are already rejected) -> -8 (20 cycles); match -> 20 cycles.
- The `mult` clobbers HI/LO (not observable).

#### `SND_ChannelCalcVolume` — 0x003ba7b0
*216 B · cycles: 32..54 total over 5 blocks, path-dependent · ops: 26 FPU, 17 loads/1 store, 3 mult*

Computes the final float volume of a channel into channel+0x48. a0 = channel index (not a pointer); channel = lw(0x50aad8) + idx*0x8c. Step 1: `v = (float)(s8[+0x2b] * s16[+0x3a] * s16[+0x42]) / 2048383.0f` (0x49fa0bf8), stored to +0x48. Step 2: if ptr at +0x78 != 0: `v = v * (float)(s8)ptr[u8 +0x67] / 127.0f`. Step 3: if ptr at +0x70 != 0: `v = (float)(s8)tbl[(int)(v*127.0f)]` (float->int truncation, signed index). Step 4: `v = v * (float)(s8 at 0x50aa5d) / 127.0f` (master volume), stored to +0x48. Returns void. Signature: `void SND_ChannelCalcVolume(int ch_index)`.

**Gotchas:**
- Cycle blocks: 20 (+10 if +0x78 table present); 3 (+12 if +0x70 table present); 9 final. Total 32 (neither table), 42, 44 or 54.
- Integer product is a 32-bit wrapping multiply of sign-extended values (lb, lh, lh), then `cvt_s_w`; every float op must use the PS2 FPU helpers in `fpu.rs` (`div_s`, `mul_s`, `cvt_w_s` truncation, no IEEE NaN/denormal handling) in exactly this order: (x/2048383) then *tbl then /127, etc.
- Table lookups use `lb` (signed) for +0x78 indexed by `lbu` of +0x67, and signed index from the truncated float for +0x70 (index can be negative).
- The result is written to +0x48 after step 1 and again after step 2/3/4; intermediate stores are observable only if something reads in between (nothing does), but final value must be written.
- Master volume byte at 0x50aa5d is `lb` (signed).
- Race profile flag P: 2 samples (hot-ish).

#### `SND_ListPushBack` — 0x003c62b8
*48 B · cycles: 11 or 12 (12 summed over 3 blocks) · ops: 0 FPU, 9 mem/6 stores*

Appends node a1 to list a0. node.next = 0; node.prev = list.tail; if tail == 0 then list.head = node else tail.next = node; list.tail = node; count += 1. No return value. Signature: `void SND_ListPushBack(SndList* l, SndNode* n)`.

**Gotchas:**
- Cycles: 6 on entry; +1 only in the non-empty-list branch (tail.next store); final 5. Empty list = 11, non-empty = 12.
- Store order: node.next, node.prev, then head-or-tail.next, then tail, then count.
- Count is `sx(count+1)` 32-bit.

#### `SND_ListPushFront` — 0x003c6288
*48 B · cycles: 11 or 12 (12 summed over 3 blocks) · ops: 0 FPU, 9 mem/6 stores*

Mirror of PushBack: node.prev = 0; node.next = list.head; if head == 0 then list.tail = node else head.prev = node; list.head = node; count += 1. No return value. Signature: `void SND_ListPushFront(SndList* l, SndNode* n)`.

**Gotchas:**
- Cycles: 6; +1 if the list was non-empty; final 5. Empty = 11, non-empty = 12.
- Store order: node.prev, node.next, head.prev-or-tail, list.head, count.
- Race profile flag c2 (called in race).

#### `SND_ListPopFront` — 0x003c62e8
*56 B · cycles: 5 / 12 / 13 by path (14 summed over 6 blocks) · ops: 0 FPU, 7 mem/4 stores*

Removes and returns the head node of list a0 (0 if empty). head = node.next; if new head == 0 then tail = 0 else newhead.prev = 0; count -= 1. Returns v0 = old head. Signature: `SndNode* SND_ListPopFront(SndList* l)`.

**Gotchas:**
- Cycles: empty list: 3 + 2 = 5, no writes. Non-empty: 3 + 3, then 1 (next != 0, writes next.prev) or 2 (next == 0, writes tail = 0), then 3 (count) + 2 = 12 or 13.
- The popped node's links are NOT cleared.
- Return in v0 is the (sign-extended lw) pointer.

#### `SND_BankGetPatch` — 0x003c6540
*60 B · cycles: 6 / 10 / 11 by path (15 summed over 5 blocks) · ops: 0 FPU, 2 loads*

Looks up patch a1 inside a bank header a0. If `a1 < (u16)bank[+6]` (signed compare, count is `lhu`), reads the self-relative offset at bank+0x14+a1*4; if nonzero returns `bank + 0x14 + a1*4 + offset`, else 0. Signature: `void* SND_BankGetPatch(SndBank* bank, int index)`.

**Gotchas:**
- The range check is a SIGNED `slt`, so negative indices pass it and read before the table (replicate, do not clamp).
- Index*4 is a 32-bit shift.
- Cycles: out of range 4+2 = 6; offset == 0: 4+4+2 = 10; valid: 4+4+3 = 11.
- Returns 0 (not -8) on failure.

#### `SND_BankGetSize` — 0x003b6528
*28 B · cycles: 7 (single block) · ops: 0 FPU, 3 loads*

Returns the size of bank number a0: `*(u32*)(lw(0x50aadc) + a0*0x10)` is a pointer to the bank header, v0 = `lw(hdr + 8)`. Signature: `int SND_BankGetSize(int bank_id)`. Reads the bank-table pointer at 0x50aadc (0x510000 - 0x5524).

**Gotchas:**
- No range or null check (null header dereferences address 8).
- Table stride 0x10; result sign-extended 32-bit.
- Flat 7 cycles.

#### `SND_BankValidate` — 0x003b64d8
*80 B · cycles: 4 / 9 / 14 / 16 by path (20 summed over 6 blocks) · ops: 0 FPU, 4 loads*

Validates bank id a0. Returns 0 if valid and not "locked", -8 if out of range or no header, -18 (0xee) if the flag byte at entry+0xc is nonzero. Count = `lhu` at 0x50a910; table base = lw(0x50aadc) with stride 0x10, header pointer at entry+0. Signature: `int SND_BankValidate(int bank_id)`.

**Gotchas:**
- Both bounds checks are signed `slt` (count is `lhu` so 0..65535); negative -> -8.
- Return values are sign-extended: -8 = `0xfffffffffffffff8`, -18 = `0xffffffffffffffee`.
- Entry flag byte at +0xc is `lb`.
- Cycles: negative 2+2 = 4; >= count 2+5+2 = 9; null header 2+5+5+2 = 14; header present (either 0 or -18) 2+5+5+4 = 16.

#### `SNDMIDI_GetSlot` — 0x003b8d98
*76 B · cycles: 5 / 8 / 15 by path (19 summed over 5 blocks) · ops: 0 FPU, 1 load, 1 mult*

Validates a MIDI handle a0 and returns its slot index. slot = a0 & 0x1f; must be < 16; slots live at 0x50a120 + slot*0x70, and the word at +0x10 (= 0x50a130 + slot*0x70) must equal the handle. Returns slot, else -8. Signature: `int SNDMIDI_GetSlot(int handle)`.

**Gotchas:**
- Handle < 0 (signed) -> -8 in 5 cycles; slot >= 16 -> -8 in 8 cycles; otherwise 15 cycles for both match and mismatch.
- The comparison is a 64-bit xor of the sign-extended `lw` against a0 (nonnegative), so it matches only when the stored id equals the handle with bit 31 clear.
- Returns the slot index (0..15) not a pointer; -8 = `0xfffffffffffffff8`.
- 8 callers; race flag c2.

#### `SNDSTRM_GetStream` — 0x003b6a38
*56 B · cycles: 7 / 9 / 12 by path (14 summed over 4 blocks) · ops: 0 FPU, 2 loads*

Returns the stream object pointer for stream index a0, or 0. Count = `lbu` at 0x50a92e; table of u32 pointers at 0x50a0a0. Signature: `SndStream* SNDSTRM_GetStream(int idx)`.

**Gotchas:**
- Check order: `a0 < count` (signed slt; count is unsigned byte) else return 0 (7 cycles); then `a0 >= 0` else 0 (9 cycles); valid = 12 cycles.
- Returns the sign-extended `lw` (pointer 32-bit, 0 when out of range).
- 16 callers.

#### `SPCH_MakeId` — 0x003d8008
*28 B · cycles: 7 (single block) · ops: 0 FPU, 4 mem (3 stores)*

Packs a speech id: `(a0 << 24) | (a1 << 16) | (a2 & 0xffff)` as a 32-bit value, in v0. The code builds it through the stack: sb sp+3 = a0, sb sp+2 = a1, sh sp+0 = a2, then `lw(sp)`. Signature: `u32 SPCH_MakeId(u8 a, u8 b, u16 c)`.

**Gotchas:**
- Result is the sign-extended `lw` (an a0 >= 0x80 gives a negative 64-bit v0).
- It writes three bytes of stack scratch at sp-0x10+0..3 (below the post-return sp, dead); sp is restored. Dropping those stores is safe only if nothing reads that dead stack (replays confirm).
- Only the low 8 bits of a0/a1 and low 16 bits of a2 are used. 44 callers, so keep it cheap.
- Flat 7 cycles.

#### `StreamFile_Validate` — 0x003dfbd0
*56 B · cycles: 4 / 9 / 10 by path (14 summed over 5 blocks) · ops: 0 FPU, 2 loads, 2 stores*

Checks a stream-file handle. a0 = `StreamFile*` (first word = pointer to header); if a0 != 0 and `*(u32*)hdr == 0x4d525453` ("STRM" little-endian) then `*a2 = a0`, `*a1 = hdr`, return 0, else return 1. Signature: `int StreamFile_Validate(StreamFile* f, StreamHdr** out_hdr, StreamFile** out_f)` (0 = ok, 1 = invalid).

**Gotchas:**
- Outputs written only on success: a2 first, then a1.
- Cycles: a0 == 0: 2+2 = 4; magic mismatch: 2+5+2 = 9; success: 2+5+3 = 10.
- A non-null a0 whose first word is 0 dereferences address 0 (same as original).
- 20 callers.

#### `StreamFile_RingInRange` — 0x003dfc08
*64 B · cycles: 7 / 8 / 10 / 11 by path (16 summed over 8 blocks) · ops: 0 FPU, 0 mem*

Ring-buffer range test: is position a2 inside [a0, a1) allowing wrap-around. If `a0 <= a1`: returns `a0 <= a2 && a2 < a1`. Else (wrapped): returns `a2 >= a0 || a2 < a1`. Signature: `bool StreamFile_RingInRange(u64 start, u64 end, u64 pos)`; returns 0/1 in v0.

**Gotchas:**
- All compares are UNSIGNED on the full 64-bit registers (`sltu`), not 32-bit.
- When start == end it is the non-wrapped case: always returns 0.
- Cycles: non-wrapped: 7 if pos < start; 8 otherwise (result `pos < end`). Wrapped: 8 if pos >= start; 11 if pos < start and pos < end; 10 if pos < start and pos >= end.
- No memory access.

#### `AsyncFile_FromHandle` — 0x003def70
*76 B · cycles: 6 / 11 / 17 by path (19 summed over 4 blocks) · ops: 0 FPU, 3 loads, 1 mult*

Maps an async-file handle to its slot record. Requires `handle >= 256` (signed) and `(handle & 0xff) < lw(0x519bf0)` (signed); slot = lw(0x450c34) + (handle & 0xff) * 0x30; returns the slot if its first word equals the handle, else 0. Signature: `AsyncFile* AsyncFile_FromHandle(int handle)`.

**Gotchas:**
- Returns a pointer (0 on failure, not -8).
- Cycles: handle < 256: 4+2 = 6; index >= count: 4+5+2 = 11; otherwise 17 whether match or mismatch.
- Compare is 64-bit xor of the sign-extended `lw` against a0, so it only matches when the handle's 64-bit register equals the stored word sign-extended.
- Table pointer is the word at 0x450c34 (0x450000 + 0xc34); count word at 0x519bf0 (0x520000 - 0x6410).

#### `BigFile_GetType` — 0x003e2500
*128 B · cycles: 12 / 23 / 28 by path (32 summed over 6 blocks) · ops: 0 FPU, 4 byte loads*

Classifies a file header at a0 by magic bytes. b0,b1: `(b0<<8)|b1 == 0xc0fb` -> 1. Else big-endian word `b0..b3 == "BIGF"` (0x42494746) -> 2. Else if `b0..b2 == "BIG"` followed by anything (word with low byte masked == 0x42494700) -> 3. Else 0. Signature: `int BigFile_GetType(const u8* hdr)`.

**Gotchas:**
- All reads are `lbu` (zero-extended); the first load of b2 and b3 happens before the first compare.
- Cycles: type 1: 12; type 2: 23 (8 + 11 + 2 + 2); type 3 or 0: 28 (8 + 11 + 7 + 2).
- Order matters: type 3 test is only reached if type 2 fails ("BIGF" is type 2, "BIG?" other 4th byte is 3).
- Returns 0..3 in v0.

#### `EATimer_GetTicks` — 0x003e4e98
*16 B · cycles: 4 (single block) · ops: 0 FPU, 1 load*

Returns the EA timer tick counter: `lw` of the word at 0x450dcc (0x450000 + 0xdcc). Signature: `int EATimer_GetTicks(void)`.

**Gotchas:**
- Sign-extended 32-bit read (counter values >= 0x80000000 become negative 64-bit).
- Flat 4 cycles; 11 callers. The value is written by another routine, so a typed accessor should read live memory each time.

**Package notes.**

Most of this package is tiny validators and accessors that share patterns. One `SndList` view (head/tail/count at +0/+4/+8, node next/prev at +0/+4) covers ListInit, PushFront, PushBack, PopFront and Remove; implement them all on one struct with `read_u32/write_u32` helpers and charge cycles per block exactly as listed. A shared `handle_slot(handle, count, table, stride)` helper (reject negative, `idx = handle & mask`, signed compare against count, match stored id) can back `SND_HandleToChannel`, `SNDMIDI_GetSlot` and `AsyncFile_FromHandle`, but their masks (0xff, 0x1f), strides (0x8c, 0x70, 0x30), return conventions (index vs pointer, -8 vs 0) and per-path cycle counts differ, so keep a per-function cost table. The sound globals live in the 0x50a8e8 state block (a `SndState` view: init byte 0x50aa5c, master vol 0x50aa5d, chan count 0x50aa66 (s16), chan table ptr 0x50aad8, bank count 0x50a910 (u16), bank table ptr 0x50aadc, stream count 0x50a92e, stream ptr table 0x50a0a0, MIDI slots 0x50a120). Suggested order: IsInitialized, EATimer_GetTicks, SPCH_MakeId, InitPlayParams, the list functions, bank functions, handle/slot validators, StreamFile_*, BigFile_GetType, and last SND_ChannelCalcVolume (needs the fpu.rs helpers and is the only float function).

### A6 Game state, lists, input & replay

Global game-state accessors (`BE*`: race setup / player slots), pad queue, replay, UI and process lists. Mostly 2–20 cycle accessors; the payoff is the typed views (`BENewRaceInt`, `PadQueue`, intrusive lists) that many Tier B/C functions use. `BELibrary_GetInterface` (530 call sites, virtual) is in Tier B2.

| addr | name | size | cycles | kind | race | callers |
|---|---|---:|---|---|---|---:|
| 0x14a080 | `BE_GetRiderCharacter` | 48 | 12 | int+mem | c | 108 |
| 0x14a0e0 | `BE_GetRiderProfile` | 76 | ≤19 (3) | int+mem | c | 84 |
| 0x147798 | `BENewPlayerInt_GetSlot` | 24 | 6 | int+mem | c | 39 |
| 0x144bc0 | `BENewRaceInt_GetTrackInfo` | 32 | 8 | int+mem | c2 | 56 |
| 0x2d1bd8 | `World_GetInstance` | 8 | 2 | int+mem | c2 | 24 |
| 0x231278 | `GameModule_RequestOverlayState` | 8 | 2 | int+mem | c2 | 3 |
| 0x2cc098 | `MenuSys_IsActive` | 28 | ≤7 (3) | int+mem | c | 5 |
| 0x326cc8 | `PadQueue_GetPadData` | 36 | 9 | int+mem | c | 2 |
| 0x326ca0 | `PadQueue_GetPadStatus` | 36 | 9 | int+mem | c | 2 |
| 0x326b48 | `PadQueue_AdvanceWrite` | 64 | ≤16 (4) | int+mem | c | 1 |
| 0x26f4a0 | `Replay_SetState` | 8 | 2 | int+mem | c | 13 |
| 0x26e9c0 | `ReplayStream_RewindWrite` | 16 | 4 | int+mem | c | 1 |
| 0x26e5d0 | `SnapshotList_PushBack` | 56 | ≤14 (4) | int+mem | c2 | 4 |
| 0x22e348 | `SectionMan_NextQueued` | 24 | ≤6 (3) | int+mem | c | 1 |
| 0x397840 | `UIList_First` | 20 | 5 | int+mem | P | 44 |
| 0x397718 | `UIListNode_IsSentinel` | 36 | ≤9 (4) | int+mem | c | 25 |
| 0x397740 | `UIListNode_Unlink` | 72 | ≤18 (10) | int+mem | c | 6 |
| 0x3d0b18 | `PATH_QueueAppend` | 92 | 23 | int+mem | c2 | 3 |
| 0x3d5450 | `PATH_SelectMap` | 116 | ≤29 (8) | int+mem | c2 | 20 |
| 0x34fc78 | `WorldObj_LinkInstance` | 8 | 2 | int+mem |  | 1 |
| 0x34fc80 | `WorldObj_UnlinkInstance` | 28 | ≤7 (3) | int+mem |  | 1 |

#### `BE_GetRiderCharacter` — 0x0014a080
*48 B · cycles: 12 (single block) · ops: 2 loads, 1 mult*

Maps a player index to its rider-character byte. Reads the player->slot table `int slot = *(int*)(0x5305b0 + a0*4)`, then returns the signed byte at `0x535b20 + slot*0x1c + 0x11` (the BENewRaceInt per-slot record array, stride 0x1c). Globals: 0x5305b0 (slot table), 0x535b20 (record array). Signature: `int8_t BE_GetRiderCharacter(int player)`.

**Gotchas:**
- Result is `lb`, so it is sign-extended to 64 bits (values >= 0x80 return negative).
- No bounds check on `a0`; `slot*0x1c` uses a 32-bit `mult`.
- Clobbers V0/V1/A0/A1/A2 (callers should not depend on them).

#### `BE_GetRiderProfile` — 0x0014a0e0
*76 B · cycles: 16 (record +0xc == -1) or 17 (otherwise), 3 blocks (14 up front, then +2 / +3) · ops: 3 loads, 1 mult*

Same record array as above (`0x535b20 + slot*0x1c`, slot from `0x5305b0[a0]`). If the word at record+0xc equals -1 (compared as full 64-bit sign-extended, i.e. an `lw` result against `-1`), returns 2; otherwise returns word at record+0x10 masked with 1. Signature: `uint32_t BE_GetRiderProfile(int player)`.

**Gotchas:**
- Block costs: 14 charged before the branch; taken (== -1) path adds 2, fall-through adds 3.
- The `-1` test is on the sign-extended load, so it only matches a stored 0xffffffff.
- Return is 2 as an "invalid/none" sentinel, else 0 or 1.
- Clobbers A0 (set to -1), A1, V1.

#### `BENewPlayerInt_GetSlot` — 0x00147798
*24 B · cycles: 6 (single block) · ops: 1 load*

Returns `*(int*)(0x5305b0 + a0*4)`, the player index to race-slot table. Signature: `int BENewPlayerInt_GetSlot(int player)`.

**Gotchas:**
- Plain `lw` (sign-extended); no bounds check.
- Trivial; also the building block that BE_GetRiderCharacter/Profile inline.

#### `BENewRaceInt_GetTrackInfo` — 0x00144bc0
*32 B · cycles: 8 (single block) · ops: 1 load, 1 mult*

Returns a pointer to the current track-info record: `0x43d950 + *(int*)0x535c08 * 100`. Global 0x535c08 is the current track index; 0x43d950 is the record table (stride 100). Signature: `void* BENewRaceInt_GetTrackInfo(void)`.

**Gotchas:**
- Returns a pointer (guest address), not data; mult is 32-bit.
- Rust computes `0x440000 - 0x26b0 = 0x43d950`.
- No bounds check on the track index.

#### `World_GetInstance` — 0x002d1bd8
*8 B · cycles: 2 (single block) · ops: 1 load*

Returns `g_World` = word at gp+0x16c8 (absolute 0x4a47b8). Signature: `World* World_GetInstance(void)`.

**Gotchas:**
- Result sign-extended `lw` of a pointer (fine for 0x5xxxxx addresses).
- Reads GP from the register file (`c.g[GP]`); use the same convention.

#### `GameModule_RequestOverlayState` — 0x00231278
*8 B · cycles: 2 (single block) · ops: 1 store*

Stores `a1` into `*(int*)(a0+0x208)` (pendingOverlayId of the GameModule/World object). No return value. Signature: `void GameModule_RequestOverlayState(GameModule* m, int id)`.

**Gotchas:**
- Single `sw` of the low 32 bits of A1; no other side effects.
- Preserve A0/A1 (not modified).

#### `MenuSys_IsActive` — 0x002cc098
*28 B · cycles: 5 (`*a0 == 0`) or 7 (otherwise), 3 blocks · ops: 2 loads*

Returns whether the menu system has a pending/active state: 0 if `*(int*)a0 == 0`, otherwise `0 < (int64)*(int*)(a0+0x54)`. a0 is MenuSys*. Signature: `bool MenuSys_IsActive(MenuSys* m)` returning 0/1.

**Gotchas:**
- Cycles: first block 3; null path then +2 (final) = 5; non-null path +2 +2 = 7.
- The second test is a signed `slt` (0 < x), so a negative +0x54 yields 0, not 1 (Ghidra's `!= 0` is misleading).
- Return value is exactly 0 or 1.

#### `PadQueue_GetPadData` — 0x00326cc8
*36 B · cycles: 9 (single block) · ops: 1 load, 2 mult*

Returns a pointer to a pad's channel data in the current ring frame: `a0 + *(int*)(a0+0x2ee0)*400 + a1*100 + 4`. a0 = PadQueue*, a1 = pad index; +0x2ee0 is the consumer index; frame stride 0x190, per-pad stride 0x64, +4 skips the status word. Signature: `float* PadQueue_GetPadData(PadQueue* q, int pad)`.

**Gotchas:**
- Returns a pointer, not a value; 32-bit multiplies.
- No bounds check on index or pad.

#### `PadQueue_GetPadStatus` — 0x00326ca0
*36 B · cycles: 9 (single block) · ops: 2 loads, 2 mult*

Returns the status word of a pad sample: `*(int*)(a0 + readIdx*400 + a1*100)` where readIdx = `*(int*)(a0+0x2ee0)`. Signature: `int PadQueue_GetPadStatus(PadQueue* q, int pad)`.

**Gotchas:**
- Counterpart of GetPadData (same address minus the +4); result is a sign-extended `lw` (channel count, 24 or 0).
- Rust clobbers A1 (A1 += readIdx*400) and A0 (A0 += A1).

#### `PadQueue_AdvanceWrite` — 0x00326b48
*64 B · cycles: 13 (equal, returns 0) or 14 (advanced, returns 1); blocks 5 + 6 + (2 or 3) · ops: 3 loads (counting div), 1 store*

Advances the consumer index: `n = (q[0x2ee0] + 1) % 30` (unsigned `divu`). If `n == q[0x2ee4]` (would reach the producer index) returns 0 and writes nothing; otherwise stores `n` to `q+0x2ee0` and returns 1. Signature: `int PadQueue_AdvanceWrite(PadQueue* q)`.

**Gotchas:**
- Cost: 5 for the first block, 6 for the second, then +2 on the full path (return 0) or +3 on the store path (return 1).
- `divu` on the sign-extended index; modulus is unsigned 30. The generated divide-by-zero check on the constant 30 is dead (both arms identical).
- The compare is against the sign-extended `lw` of +0x2ee4.
- Clobbers A0 (reused for writeIdx), A1, V1.

#### `Replay_SetState` — 0x0026f4a0
*8 B · cycles: 2 (single block) · ops: 1 store*

Writes `a1` to `*(int*)a0` (the replay state word). Signature: `void Replay_SetState(Replay* r, int state)`.

**Gotchas:**
- Offset 0; no checks.
- Called from 13 sites; trivial.

#### `ReplayStream_RewindWrite` — 0x0026e9c0
*16 B · cycles: 4 (single block) · ops: 1 load, 2 stores*

Resets the stream write cursor: `s->+0x10 = 0; s->+0x8 = s->+0x4` (writePos = base/start pointer, byte counter zeroed). Signature: `void ReplayStream_RewindWrite(ReplayStream* s)`.

**Gotchas:**
- Store order: +0x10 first, then +0x8 (irrelevant unless aliased).
- V0 clobbered with the +4 value.

#### `SnapshotList_PushBack` — 0x0026e5d0
*56 B · cycles: 13 (head non-null) or 14 (head null); blocks 6 + 3 (+1) + 4 · ops: 4 loads, 4 stores (6 stores on the head path)*

Intrusive doubly linked append. List: `+0 head, +4 tail, +8 count`; node: `+0x14 next, +0x18 prev`. Sets `node.next = 0`, `node.prev = tail`, links `tail.next = node` if tail non-null, sets `list.tail = node`, sets `list.head = node` if head was null, and increments count. a0 = list, a1 = node; no return. Signature: `void SnapshotList_PushBack(SnapList* l, SnapNode* n)`.

**Gotchas:**
- Costs: 6 (first block), 3 for the head test, +1 only when head is null (the extra `sw`), then 4 for the count update.
- Tail is loaded twice (before and after the stores to node); keep the second read so aliasing behaves identically.
- Head test uses the value loaded before `list.tail` is stored; count is read/written last.

#### `SectionMan_NextQueued` — 0x0022e348
*24 B · cycles: 4 on both paths (2 + 2); 3 blocks · ops: 1 load*

Iterator step over the queued-section list: if `a1 == 0` returns `*(a0+0x19c)` (list head), else returns `*(a1+4)` (node next). a0 = SectionMan, a1 = current node or null. Signature: `Node* SectionMan_NextQueued(SectionMan* m, Node* cur)`.

**Gotchas:**
- Both paths charge 4 cycles total (the 2-cycle test block plus 2).
- Null test is on the full 64-bit A1.

#### `UIList_First` — 0x00397840
*20 B · cycles: 5 (single block) · ops: 2 loads*

Returns the first real node of a UI list, or 0 if empty. `n = *(a0+4)`; if `*(n+4) == n` returns 0, else returns `n`. a0 = UIList*; offset +4 holds the head/anchor node. Signature: `UINode* UIList_First(UIList* l)`.

**Gotchas:**
- Branchless (movz in Rust), fixed 5 cycles.
- The emptiness test is `n->+4 == n` (node's +4 link points at itself), same sentinel test used by UIListNode_IsSentinel.
- Clobbers V1.

#### `UIListNode_IsSentinel` — 0x00397718
*36 B · cycles: 6 (+4 == self), 8 (neither link is self), or 9 (+0 == self only); 4 blocks · ops: 2 loads*

Returns 1 if the node's `+4` link or `+0` link points to the node itself, else 0. a0 = node. Signature: `int UIListNode_IsSentinel(UINode* n)`.

**Gotchas:**
- Cycle paths: +4 == self: 3 + 1 + 2 = 6; both non-self: 3 + 3 + 2 = 8; +4 non-self but +0 == self: 3 + 3 + 1 + 2 = 9.
- The +0 link is only read when +4 isn't self (short-circuit).
- Returns exactly 0 or 1.

#### `UIListNode_Unlink` — 0x00397740
*72 B · cycles: 7 to 18, path-dependent (phase A 3/5/7/8, phase B 2/4/7/8, plus final 2); 10 blocks · ops: 10 loads/compares, up to 8 stores*

Removes a node from the intrusive list and makes it self-linked. With `a = n->+0`, `b = n->+4`: (1) if `a != 0 && a != n`, `a->+4 = (b != n) ? b : a`; (2) if `b == 0 || b == n`: `n->+0 = n`; else `b->+0 = (a != n) ? a : b` and `n->+0 = n`; (3) `n->+4 = n`. a0 = node; no return. Signature: `void UIListNode_Unlink(UINode* n)`.

**Gotchas:**
- Block costs: entry 3; a==0 stops (3); a==n adds 2 (5); a valid adds 2 + 2 (7) or +1 more when `b == n` (8). Phase B: 2 base; b==0 stops (2); b==n +2 (4); else +2 + 2 + 1 (7 if a != n) or +1 more (8 if a == n). Final +2 for the last store.
- Storing `*b = a` writes null when `a == 0` (the test is only `a != n`).
- Sentinel (self-link) nodes are tolerated: only the node's own links are rewritten.
- Writes happen in this order: `a->+4`, `b->+0`, `n->+0`, `n->+4` (matters only if nodes alias).

#### `PATH_QueueAppend` — 0x003d0b18
*92 B · cycles: 23 (single block) · ops: 1 load, 1 word store, 4 unaligned 64-bit loads, 4 unaligned 64-bit stores*

Appends a 16-byte entry to a 64-slot ring in the path object. a0 = queue, a1 = source entry. The write index is bits 16..21 of the word at `a0+0xc`; the entry goes to `a0 + 0x10 + idx*16`, and the header word is rewritten as `(hdr & 0xffc0ffff) | (((idx+1) & 0x3f) << 16)`. Entry is copied as two 64-bit halves. No return. Signature: `void PATH_QueueAppend(PathQueue* q, const uint8_t entry[16])`.

**Gotchas:**
- Header store occurs before the entry copy (matters if the source overlaps the queue).
- The copy uses `ldl/ldr` + `sdl/sdr` (unaligned 8-byte), not `lq/sq`; the source may be unaligned, so do two unaligned 64-bit moves (or a byte copy).
- Header mask in Rust is `0xffffffffffc0ffff` (sign-extended), only the low 32 bits are stored.
- Both halves are read into registers before any entry store (A1 is read fully first), so overlapping source and destination copy as a snapshot.
- No overflow detection; wraps modulo 64.

#### `PATH_SelectMap` — 0x003d5450
*116 B · cycles: path-dependent, 8 blocks: entry 4 (+3 if current id was 0), then +6 (a0 >= 4), +10 (valid map, returns early) or +12 (a0 < 4 but invalid), see below · ops: 3-4 loads, 2-3 stores, 1 mult*

Selects the active path map among four. Globals: 0x44f430 (current map id word), 0x44f420 (current map pointer), table at 0x517610 with stride 0x928 (map valid flag at +0x34, id at +0x30). First step: if `[0x44f430] == 0` it is set to -1. If `a0 < 4` unsigned and `[0x517610 + a0*0x928 + 0x34] != 0`: `[0x44f420] = &entry`, `[0x44f430] = entry+0x30`, return that id. Otherwise, if `a0 != -1` set `[0x44f430] = 0`; return `[0x44f430]`. Signature: `int PATH_SelectMap(uint32_t idx)` (idx may be -1 to query).

**Gotchas:**
- Cycle totals: base = 4 if `[0x44f430] != 0`, else 7 (the -1 store). Then second block 3, plus: `a0 >= 4` adds 1, then the common tail +2 = base + 6; `a0 < 4` adds 7: valid -> base + 10 and returns immediately (no tail), invalid -> base + 12 (7 + tail 2).
- The `a0 != -1` test is a 64-bit compare against sign-extended -1, so the caller's A0 must be sign-extended.
- The `[0x44f430] == 0 -> -1` write happens first and persists on all paths except when overwritten later.
- On invalid index the global is zeroed unless `a0 == -1`; return value is the global after that.
- Clobbers A1 (mult destination `a0*0x928`), A2, V1; the `mult` is the three-operand form (result goes to A1).

#### `WorldObj_LinkInstance` — 0x0034fc78
*8 B · cycles: 2 (single block) · ops: 1 store*

Sets the object's owner/instance link: `*(int*)(a1+0xc) = a0`. Note argument order: a0 = instance, a1 = obj. No return. Signature: `void WorldObj_LinkInstance(Instance* inst, WorldObj* obj)`.

**Gotchas:**
- Argument order is (value, object); not (object, value).
- Called only once; no null check.

#### `WorldObj_UnlinkInstance` — 0x0034fc80
*28 B · cycles: 4 (obj null) or 7 (obj non-null); 3 blocks · ops: 1 load, 1 store*

If `a1` is non-null and `*(int*)(a1+0xc) == a0`, clears that field to 0. a0 = instance, a1 = obj. No return. Signature: `void WorldObj_UnlinkInstance(Instance* inst, WorldObj* obj)`.

**Gotchas:**
- Costs: 2 entry, +3 for the load/compare block (non-null), then +2 final: null path = 2 + 2 = 4, non-null = 2 + 3 + 2 = 7 (store or not).
- Compare is 64-bit between a sign-extended `lw` and A0.
- Same argument order as LinkInstance (instance first).

**Package notes.** Most of this package is 2-to-9 cycle leaf accessors, so they can be written as straight Rust with one `c.cycles += N` (use the per-path totals noted above where branches exist). Shared helpers worth adding: a `BENewRaceInt` view (slot table 0x5305b0, record array 0x535b20 stride 0x1c, track idx 0x535c08, track table 0x43d950 stride 100) used by BE_GetRiderCharacter, BE_GetRiderProfile, BENewPlayerInt_GetSlot and GetTrackInfo (the first two should call the same slot lookup); a `PadQueue` view (readIdx +0x2ee0, writeIdx +0x2ee4, frame 0x190, pad 0x64) for GetPadStatus, GetPadData and AdvanceWrite; and an intrusive list view where UIList_First, UIListNode_IsSentinel and UIListNode_Unlink share the `+0 / +4` self-link convention (SnapshotList_PushBack uses a different layout: node +0x14/+0x18, list head/tail/count). Suggested order: trivial accessors and stores first (GetSlot, World_GetInstance, Replay_SetState, RewindWrite, Link/Unlink instance, RequestOverlayState), then UIList trio, PadQueue, SnapshotList_PushBack, SectionMan_NextQueued, and last PATH_QueueAppend (unaligned copy) and PATH_SelectMap (global side effects, multi-path cycle charging).

## Tier B: small functions with loops, calls or indirect calls

These have clear semantics and are short, but each one needs at least one of
the following:

- per-iteration cycle and event handling (**L**);
- calls that must be replayed in order, with the unwind check (**C**);
- an indirect call through a vtable or function pointer (**I**, done with
  `call_indirect(c, target)` after setting `ra`).

Do them after the Tier A helper types exist; several build on Tier A
functions. Columns:

- **cyc**: sum of the `c.cycles +=` blocks (an upper bound when the function
  has more than one block);
- **race**: P / c / c2 as in Tier A;
- **callers**: number of direct call sites.

### B1. Memory and string utilities (libc / EA runtime)

These are the most-called functions in the game. All are leaves with loops.
The newlib versions use 128-bit `lq/sq` and MMI (`pcpyh`, `pcpyld`,
`pcpyud`, …) for the word-at-a-time fast paths. A replacement can use plain
Rust loops for the *effects*, but it has to:

- charge cycles per block and per iteration exactly as the generated
  `match pc` dispatcher does. Port that cycle schedule mechanically: start from
  the dispatcher and replace the register traffic, not the control flow;
- make the event check in each loop.

| addr | name | size | cyc | shape | race | callers | semantics, notes |
|---|---|---:|---:|---|---|---:|---|
| 0x416210 | `memset` | 192 | 48 / 13 blk | L, MMI | P | 187 | newlib: byte loop to 16-byte alignment, then `sq` of a `pcpyh`/`pcpyld`-replicated byte 32 B per iteration, then `sd` 8 B, then a byte tail; returns `a0`. |
| 0x41605c | `memcpy` | 176 | 44 / 11 blk | L | P | 52 | newlib: if both pointers are aligned, a 64-byte `lq/sq` loop, then 16 B, then a byte tail; returns `a0`. |
| 0x3e6574 | `EA_MemCopy` | 280 | 70 / 17 blk | L | P | 175 | EA copy: if `n > 0x11`, align dst to 16 one byte at a time, then a 128-byte unrolled loop when src is aligned, else an `ldl/ldr`-style unaligned path. The alignment loop tests `--n < 1` *before* storing the byte; with `n > 0x11` that early exit cannot fire, but a port should keep the order. |
| 0x3e6448 | `EA_MemSet` | 300 | 75 / 24 blk | L | P | 126 | EA fill: aligns, then a 16-byte `sq` loop, then a tail. |
| 0x416810 | `strlen` | 328 | 82 / 12 blk | L, MMI | P | 114 | newlib word-at-a-time null scan (`pextlb`, …); returns the length. |
| 0x4165a8 | `strcmp` | 332 | 83 / 18 blk | L, MMI | P | 61 | Word-at-a-time compare when aligned, else bytewise; returns the byte difference (sign matters). |
| 0x4166f4 | `strcpy` | 284 | 71 / 10 blk | L, MMI | c | 120 | Word-at-a-time when aligned. |
| 0x416ce0 | `strncpy` | 460 | 115 / 18 blk | L, MMI | c | 37 | Zero-pads up to `n`. |
| 0x3dbca0 | `UStr_Len` | 52 | 13 / 4 blk | L | | 27 | UTF-16 length (halfword loop). |
| 0x2c2508 | `UStr_Copy` | 52 | 13 / 4 blk | L | | 31 | UTF-16 copy including the terminator. |
| 0x371d10 / 0x371dd8 | `Dma_ToScratchpad` / `Dma_FromScratchpad` | 196 each | 49 / 9 blk | L (2) | P | 2 | Start an SPR DMA (D8/D9 channel registers) after spinning on the channel's busy bit. The spin loop polls hardware, so its iteration count depends on the DMA model; port it literally. |

### B2. Thin wrappers and virtual dispatch (I / C)

Mostly the per-frame entry points of rider, camera and sound. They are short,
but they are the doors into the big state machines, which makes them good
first targets for typed signatures (roadmap step 3).

| addr | name | size | cyc | shape | race | callers | semantics, notes |
|---|---|---:|---:|---|---|---:|---|
| 0x14dd58 | `BELibrary_GetInterface` | 64 | 16 | I | P | **530** | `iface = g_beLibrary[*(gp-0x1efc)]` (table 0x5308a8); calls its vtable slot `vt+0xc` with `this + (short)vt[+8]` and returns whatever that returns. The game's most-called function. |
| 0x3b58a0 / 0x3b58d8 | `SND_Lock` / `SND_Unlock` | 56 | 14 | I | P | 79 / 79 | Call the lock hook at `*(0x44c46c)` (default `SND_DefaultLock` → `SND_MutexLock`), then increment / decrement the byte nesting counter at 0x50aa5e. |
| 0x3dcf70 | `EAFile_SetRequestCallback` | 80 | 20 / 3 blk | I | P | 16 | Stores a callback in a file request and may call it immediately. |
| 0x15df98 | `GameCamera_Update` | 64 | 16 | I + C | P | vt | `ctrl = cam+0xa0`; calls `ctrl->vt[+0x14]` (controller update), then `GameCamera_ComputeView(cam)`. |
| 0x1624e8 | `ChaseCameraAlgorithm_Tick` | 116 | 29 / 3 blk | I ×3 | P | vt | If `this+0x2f0`, calls vt slot `+0x24`; then slots `+0x2c` and `+0x44(arg)`. vptr at `this+0x10` (camera.md). |
| 0x168150 | `ChaseCameraAlgorithm_CheckDistanceReset` | 328 | 82 / 5 blk | I, VU0 | P | 14 | `d = |eye(+0x20) − target(+0x40)|` with VU0 `vmul`/`vadda`/`vsqrt`, read back through `cfc2 vi22`. If `d < 20` or `d > *(0x49c978)`, re-places the eye: target velocity × 200 (`0x43480000`) from `target->vt[+0xc]`. |
| 0x166550 | `ChaseCameraAlgorithm_InitFromRider` | 236 | 59 | I, VU0 | | 15 | Seeds the chase state from the rider (position, forward) and four float arguments (f12, f13, f20, f21). |
| 0x166f28 | `ChaseCameraAlgorithm_Reset` | 100 | 25 | I ×2 | | 1 | Clears the state and calls two virtual resets. |
| 0x15edc8 | `GameCamera_SetTargetRider` | 56 | 14 | I | | 3 | Stores the rider and notifies the controller (virtual). |
| 0x15f5b0 | `RiderCameraTarget_GetVelocity` | 72 | 18 | I | | vt | Calls a rider virtual and copies its velocity vec4. |
| 0x15f568 | `RiderCameraTarget_GetPosition` | 44 | 11 | C | | vt | Tail-wraps `Rider_GetHipsPosition` (A2). |
| 0x15cfe8 | `ManualCamController_Update` | 52 | 13 | I | | vt | Forwards to the selected algorithm's update. |
| 0x1216e0 | `Rider_UpdateMainState` | 28 | 7 | C | P | 1 | `RiderBase_UpdateMainState(rider->owner(+0x77c))`. **75 profile samples** below it: this is the main rider-physics door. |
| 0x121750 | `Rider_PostUpdateMainState` | 164 | 41 / 8 blk | C, FPU | P | 1 | Calls `RiderBase_PostUpdateMainState(owner)`. Then, if the vec4 at `rider+0x9d0` differs from the vec4 at 0x4ff120 (four FPU `c.eq.s` compares), calls `FUN_00310530(rider+0x780 model, rider+0x9d0)`. |
| 0x121068 | `Rider_ProcessControlInput` | 72 | 18 | I + C | P | 1 | Reads the control record and dispatches `RiderBase_HandleSubStateInput`. |
| 0x11fe78 / 0x11fec8 | `Rider_SetMainState` / `Rider_SetSubState` | 28 | 7 | C | c | 22 / 33 | Forward to `RiderBase_Set{Main,Sub}State(owner, s)`. These in turn (0x1112b8 / 0x111538, 64 B each) call Exit(old), then Enter(new), and store `+0xde0` / `+0xde4`. |
| 0x136958 | `RiderStaticState_Update` | 28 | 7 | C | c | 1 | Just `Rider_BuildWorldMatrix(rider)`; the first caller of the finished override. |
| 0x106538 | `Rider_Translate` | 116 | 29 | C, VU0 | c | 9 | Adds the vec4 `*a1` to rider `+0x110` (pos), `+0x9d0`, `+0x400` and `+0x410` (bounds) with VU0 `vadd`. Between the first two adds and the last two it calls `FUN_00329b40(rider+0xaa0)`, so the call order matters. |
| 0x168060 | `CameraBase_Update` | 28 | 7 | C | | vt | `CameraBase_UpdateMatrices(this)` (A3). |
| 0x3128e8 | `AnimController_PlayAnim` | 244 | 61 / 5 blk | I + C | P | 59 | Looks up the anim (`AnimTable_GetEntry` ×2) and starts it on a layer (`FUN_00311f00`). |
| 0x317e30 / 0x317d70 / 0x317e50 | `operator_new_named` / `EA_MemAllocNamed` / `__builtin_delete` | 28 / 192 / 72 | 7 / 48 / 18 | C | P | 213 / 452 / 444 | Allocator front ends: `EAMutex_Lock` → `MEM_AllocInternal` / `MEM_FreeInternal` → `EAMutex_Unlock`. Typing them gives every constructor a real `alloc(size, tag)`. |
| 0x395370 | `PS2GraphicsMan_SetViewportRect` | 64 | 16 | I, FPU | P | vt | Stores the viewport rect and notifies through a function pointer. |
| 0x376938 | `PS2GraphicsMan_SetScissorClamped` | 312 | 78 / 11 blk | I ×2, FPU | P | 3 | Clamps a float rect (f12–f15) to the screen, converts it with `cvt.w.s`, and emits the scissor. FPU compares need the sign-magnitude key. |

### B3. Small math and logic with calls or loops

| addr | name | size | cyc | shape | race | callers | semantics, notes |
|---|---|---:|---:|---|---|---:|---|
| 0x15d780 | `CameraBase_GetYaw` | 256 | 64 / 11 blk | C, FPU | | vt | Yaw from the camera quaternion at `+0x30` (x, y, z, w):
`s = 2(y·x − w·z)`, `c = 1 − 2(z² + x²)`, then `atan(c/s)` with quadrant fix-ups from the float constants at 0x49c430..0x49c43c (`fGpffff9340..934c`). |
| 0x15d880 | `CameraBase_GetPitch` | 164 | 41 | C, VU0 + FPU | | vt | Builds the forward vector from the quaternion, gets its length with VU0 `vsqrt` and reads it back through `cfc2 vi22`, then returns `Math_Asin(fwd.z / len)`. |
| 0x163010 | `ChaseCameraAlgorithm_ApplyFollowDistance` | 328 | 82 / 7 blk | C, VU0 + FPU | P | 7 | Spring-damped follow distance from the speed (`sqrt`), clamped by branchless min/max: an integer select on float bits, `(int)a*(a<b) \| (int)b*(a>=b)`. Writes `rider+0x1e8` / `+0x1ec`; calls `CheckDistanceReset`. |
| 0x162b90 / 0x163158 / 0x163270 / 0x1633b0 | `ChaseCameraAlgorithm_Apply{TargetHeight,SideOffset,CameraHeight,Fov}` | 160–316 | 40–79 | C, FPU (+VU0) | P/c | 3–7 | Siblings of ApplyFollowDistance; each ends with `CheckDistanceReset`. Do all five together after A3. |
| 0x2d1928 | `Noise_Fractal1D` | 144 | 36 / 4 blk | L + C, FPU | P | 2 | Sums octaves of `Noise_Sample1D` (A1) with halving amplitude. |
| 0x2c1cd8 | `QuadTree_FindLeaf` | 208 | 52 / 7 blk | L, FPU | P | 1 | Descends the world-painter quadtree by comparing x/y against node centres; returns the leaf. |
| 0x3baa00 | `SND_NextHandleChannel` | 192 | 48 / 14 blk | L | P | 14 | Iterates the channels that belong to a sound handle (with A5 `SND_HandleToChannel`). |
| 0x3b8790 / 0x3b8c00 | `SND_SetPitch` / `SND_SetPan` | 164 / 200 | 41 / 50 | L + C | P | 4 / 4 | For each channel of a handle: compute (`SND_ChannelCalcPitch` → `SND_CentsToPitch`) and apply. |
| 0x39f9d8 / 0x39fa88 | `UIStateMgr_FindState` / `UIStateMgr_GetActiveState` | 172 / 96 | 43 / 24 | L + C | P | 19 / 11 | Walk a UIList with `UIList_First` / `UIListNode_IsSentinel` (A6). |
| 0x146e98 | `BENewPlayerInt_GetHumanRider` | 168 | 42 / 9 blk | L + C | P | 47 | Scans the slots for the human-controlled rider (uses A6 `BENewPlayerInt_GetSlot`). |
| 0x12a250 / 0x12a180 | `AI_AllPlayersFinished` / `AI_AreAllRidersLoaded` | 92 / 112 | 23 / 28 | L (+C) | c | 11 / 5 | Loops over `AI+0x28` riders (world.md). |
| 0x391fb0 / 0x391cb0 / 0x392908 | `Font_MeasureText` / `Font_DrawText` / `Font_DrawTextCentered` | 572 / 384 / 120 | 143 / 96 / 30 | L / I / C, FPU | P | 23 / 19 / 7 | HUD text. DrawText is profiled at 10 samples. It has two `jalr`s through the font object; float x/y/scale arrive in f12/f13. |
| 0x4139f8 / 0x411f18 / 0x412550 | `__fixunssfsi` / `__adddf3` / `__floatsidf` | | | L / C | P | 17 / 10 / 10 | libgcc soft-float (double). This is integer code with exact IEEE semantics: a straight Rust port of libgcc `fp-bit.c` (`__unpack_d`, `__pack_d`, …) can be bit-exact, but the cycles must still be charged per block. Low priority. |

## Tier C: larger subsystems (later, one owner each)

These are where the race time goes and where typed Rust pays off most. Each
needs the Tier A/B helpers below it, a typed struct view (`structs/*.md`), and
care with `__entries` twins, which block a whole-function override until the
mid-function entry is understood. "prof" is the number of profile samples with
the function on the stack.

| subsystem | entry points (addr, prof) | size | blockers / notes |
|---|---|---|---|
| **Rider physics state machine** | `AI_Update` 0x128af0 (575, **has `__entries`**, 29 loops) → `Rider_UpdateMainState` → `RiderBase_UpdateMainState` 0x111408 (75, **`__entries`**) → `RiderGroundState_Update` 0x13d818 (68, 6.5 KB, 27 calls) and `RiderGroundState_PostUpdate` 0x13f178 (36); the unnamed hot core is 0x11eb98 (106, 2.1 KB, 132 VU0 ops), 0x13d1b8 (53, VU0) and 0x11e150 (VU0/FPU leaf, 2.5 KB) | 10+ KB | Needs the A2 views (`Rider`, `RiderBase`), A1 math and B2 wrappers. `RiderBase_{Update,PostUpdate}MainState` are switch-dispatched with external entries; override the per-state functions (`Rider*State_*`, `Rider*Sub_*`, ~80 functions, mostly 100–500 B) first. |
| **Chase camera** | `GameCamera_Update` 0x15df98 (32) → `GameCamera_ComputeView`; `ChaseCameraAlgorithm_Tick` / `_PostUpdate` 0x166228; the `Apply*` family (B3); `CameraBase_UpdateMatrices` (A3) | ~5 KB | The cleanest subsystem after the rider. camera.md maps nearly all fields. Do A3 → B2/B3 → ComputeView. |
| **Animation / skinning** | `AnimController_EvaluatePose` 0x312598 (132) → 0x30f2b0 (126, 3.7 KB, 13 loops, 65 VU0 ops) and 0x314418; 0x310200 (62), 0x310120 (37), 0x310640 (35) are VU0 bone-matrix leaves | ~8 KB | The hottest guest compute in a race. The VU0 matrix helpers 0x310120 (straight-line, 31 VU0 ops, 55 cycles) and 0x310200 should be **named first**; they are the natural follow-ups to `Rider_BuildWorldMatrix`. |
| **World render / process list** | `ProcessList_{Render,Update,PostUpdate}Priority` 0x3550a0 / 0x354f98 / 0x355028 (90 / 50 / 9), `ModObj_Render` 0x356298 (77, 10 jalr), `ModObj_Update`, `SkyBox_Render`, `DeferMan_BuildBuckets` 0x362de8 (22), `PS2GraphicsMan_RenderFrame` 0x363490 (89, 12 loops) | large | Virtual dispatch everywhere (`WorldObj` vtables, world.md). Start with the list walkers (small loops with jalr). |
| **Instance / geometry builders (VU0)** | 0x3cb540 (142, 1.5 KB VU0/FPU), 0x3c85d0 (170) / 0x3c8968 (171), 0x3cd878 (78), 0x3ce410 (35, 229 FPU ops), 0x32b6e0 (59, 92 VU0 ops), 0x32e9a0 (32, 166 VU0 ops), 0x38b370 (41, 4.6 KB) | large | Unnamed. These build VIF/VU1 packets for the course geometry. Naming pass (render.md) first. Many are leaves with loops, so they are attractive once named. |
| **Renderer DMA / GS kick** | `Renderer_DmaKickThread` 0x382760 (10 loops), `PS2GraphicsMan_EndFrame` 0x377a10, `Renderer_SetDisplayRegs` 0x382af0 (3.2 KB, 8 loops), `TexMan_BindTextures` 0x368660 (31), `SprStream_*` (B) | | Talks to hardware registers; polling loops depend on the DMA/GS model timing. Keep it generated until phase 3 replaces the renderer. |
| **HUD** | `cOVStateHUD1P_Render` 0x1ec3f8 (56, **19.7 KB**, 212 calls), `Font_*` (B3), `cHUDFX_*` | very large | One giant function (`ssx3_ee_s04::f_001ec3f8` in profiles). Split it only after its callees are typed. |
| **Sound** | `SND_EeThread` 0x3c19a8, `SND_Service` 0x3b5540 (17), `SSXAudioSystem_Update` 0x285bf8 (41, 25 calls), `cWorldSound_Update` | medium | Built on A5 + B2 (`SND_Lock`) + B3 (`SND_Set*`). Runs on its own guest thread: per-iteration event checks are essential. |
| **File / CD streaming** | `EAFile_CdReadThread` 0x3e3be0, `EAFile_QueueRead` 0x3dd648, `EAFile_CdIssueRead`, `StreamFile_*` | medium | Thread and semaphore heavy (syscalls); only the bookkeeping (A5) is worth rewriting before phase 3. |
| **Memory manager** | `MEM_AllocInternal` 0x319a90, `MEM_PoolAlloc`, `DLHeap_*` (dlmalloc), `VramHeap_*` (VRAM allocator, 0x366ce0 14 samples) | medium | Pure logic with loops; very good typed-Rust candidates. Allocation addresses must stay identical, so port the algorithm exactly. |
| **libc / libgcc / Sony libs** | `vfprintf_dispatch` (`sprintf`, 205 callers), soft-float `__*df3`, `__udivdi3`/`__umoddi3` (A1-adjacent, 79/74 blocks), `sce*` SIF/pad/cd | | Mechanical ports. Do them only where a typed caller needs them. |
| **Main loop / threads** | `entry`, `App_Run_MainLoop` 0x316d60, unnamed 0x316f00 / 0x22b008 (the frame dispatcher, 6.5 K samples inclusive), `PS2ExecMan_*` threads, `GameModule_Update` 0x2306b8 (701, 75 calls) | | Orchestration with syscalls and thread waits. Leave until last; `GameModule_Update` becomes a typed call list once its ~75 callees are typed. |

## Name first: hot unnamed functions

These are hot (race profile) and cheap, but unnamed. A short Ghidra naming pass
(owned by the naming agent, `naming.md`) would move them into Tier A/B.

| addr | prof | size | shape | what it looks like |
|---|---:|---:|---|---|
| 0x310120 | 37 | 220 | leaf, straight-line, 31 VU0 ops, 55 cycles | `(skel, bone, src)`: builds bone `bone`'s 4x4 matrix in the table at `*(skel+0x30) + bone*0x40` from a quaternion + translation pair at `src` (same quaternion-to-matrix VU0 sequence as `Rider_BuildWorldMatrix`, without the normalise); the obvious next override |
| 0x310200 | 62 | 492 | leaf, 2 loops, 21 VU0 + 16 FPU | per-bone matrix loop (animation) |
| 0x0037de88 | 52 | ? (not a Ghidra function) | leaf, 3 blocks, VU0 | small VU0 helper in the render path; create the Ghidra function first |
| 0x32b6a8 | 12 | 56 | leaf, straight-line, 14 cycles | **not a math helper**: loads four vec4s into VF regs, `ctc2` CMSAR0 = 0xdc, `vcallmsr` (runs a VU0 *micro* program) and returns `vi2 != 0`. A collision/overlap test whose replacement must keep the micro-program call; skip |
| 0x1e9290 | 1 | 20 | leaf, 5 cycles | copies two words `p+8`, `p+0xc` to `*a1` (a 2-float getter), 19 callers |
| 0x28b180 | 0 (c) | 8 | leaf | `return *(u32*)(gp+0x410)` = global at 0x4a3500 (singleton getter), 167 callers |
| 0x287968 | 0 (c) | 52 | leaf, 5 blocks | `(base, kind, idx)`: returns `base+0x636c+idx*4` if `kind==4 && idx<10`, else `base+0x62c8+kind*16` (slot address); 58 callers |
| 0x261460 | 0 | 48 | leaf | bool: `p+0xc8 != 0 && p+0xd8 != 1 && p+0xe0 != 0`; 59 callers |
| 0x001c57c0 | 0 | 28 | leaf, 2 stores | `(obj, i, v)`: `obj[i*8+0x28] = v; obj[i*8+0x24] = 3` (FE widget setter); 55 callers |
| 0x0011eb60 | 136 | 52 | 2 calls | `FUN_003123c0(rider+0x784 anim, rider+0xb1c); FUN_00312490(anim)`: anim-controller update for the rider |
| 0x00121700 / 0x00121728 | 136 / 106 | 40 | 1 call | `if (!rider->isPuppet(+0xac4)) FUN_0011eb60(rider+0x300…)`-style per-phase wrappers next to `Rider_UpdateMainState` |
| 0x0011eb98 | 106 | 2112 | 13 calls, 132 VU0 ops | rider physics integrator (Tier C) |
| 0x003ce410 | 35 | 940 | leaf, 1 loop, 229 FPU ops | FPU-heavy inner loop (geometry/LOD) |

## Shared prerequisites and order

Add these to `crates/ssx3-game` before or together with the first function of
the package that needs them. Each one is small; agree on the name in the first
PR so the parallel packages do not each add their own.

| helper | needed by | notes |
|---|---|---|
| `ps2f::FpuF32` (add/sub/mul/div/sqrt/rsqrt, `madd`=`add(acc, mul)`, `cvt_w`/`cvt_s`, compare key from `fpu.rs`) | A1, A2, A3, A4, A5, B | Wrap the same `ssx3_ee_rt::float` primitives that `fpu.rs` uses. A unit test can compare against `Cpu::add_s` etc. bit for bit. |
| `abi::farg(c, i)` (f12, f13, …) and `abi::set_fret(c, FpuF32)` (f0) | A1–A5 | The game's float ABI. `$f12` is the first float argument; integer arguments keep their own a0… numbering. |
| `VuF32::div` / `VuF32::sqrt` (Q-register results; `.value` of `float::div`/`sqrt`) | A2 (`RiderHandplantState_Enter`), A3, B3 | `rsqrt` already exists. |
| `guest::Addr` 8/16-bit and 64-bit accessors (`read_i8/u8/i16/u16`, `read_u64/write_u64`) with the same sign rules as `c.lb/lbu/lh/lhu/ld/sd` | A4, A5, A6 | |
| Struct views: `Rider::owner()` → `RiderBase`, `AnimSequence`, `GameCamera` / `CameraBase` / `ChaseCameraAlgorithm`, `GsPacket` cursor, `BENewRaceInt`, `PadQueue`, intrusive `UIList` / `SndList` | per package | Field names from `structs/*.md`. Keep them as views over guest memory (`structs/README.md`, "How these become Rust"). |
| A harness for differential unit tests: run `f_X__generated` and the replacement on cloned `Cpu`s with random inputs; compare memory writes, v0/f0 and `cycles` | all | This catches cycle-path and rounding mistakes long before a replay does. |

Suggested sequencing across people:

1. **A1** first: the `FpuF32` it adds is shared by four other packages.
   **A6** and **A5** need no floats and can start immediately.
2. **A2**, **A3** and **A4** after `FpuF32` lands. A3's
   `CameraBase_UpdateMatrices` and the unnamed bone-matrix helper 0x310120 (see
   § Name first) reuse the `Rider_BuildWorldMatrix` VU0 pattern.
3. **Tier B**: B1 (memory/string) any time; the rest once the Tier A pieces and
   views they call exist.
4. **Tier C**: one owner per subsystem, starting with the chase camera (best
   documented) and the per-state rider functions.

## Reproducing the classification

The numbers above come from a throwaway script, which was not committed. It
walks `generated/ee/s*/src/lib.rs` function by function and does three things:

- reads the `__generated` body when an override exists;
- treats `pub fn f_X__entries` as the "not overridable" marker;
- counts the patterns quoted in § How the data was made.

It joins sizes from `generated/ee/game/src/lib.rs` (`FnEntry { start, end }`)
and names from `generated/functions.tsv`. For hotness it parses the `sample`
call graph, keeping `ssx3_ee_sNN::f_XXXXXXXX` frames, counting a frame once per
stack and summing samples. It takes about a minute to rewrite. Rerun it after
renames or a regeneration. The profile is from 2026-10-14, made before the
crate split, but the guest functions are the same.
