# Race geometry: "girders" across the screen (2026-10-14)

## Symptom

In races, long flat polygons cut across the screen: salmon, blue and white
slabs and spikes that run from the course out to the screen edges, plus big
dark rectangles around the sky. They get worse the further the race goes.
Textures (PATH3 masking, `race-textures.md`), ADC/XYZ3 handling and VU
microcode semantics against a reference interpreter (`vu-differential.md`)
had already been ruled out.

BEFORE_AFTER_TABLE

## Finding the bad triangles

1. Capture one frame: `SSX3_GS_CAPTURE=4999:2:f.gstrace` and every VU
   program of the same frames: `SSX3_VU_CAPTURE=f.vucap:12000:4999:1:5001`.
2. `gs_replay f.gstrace --draws d` lists the draw setups. Dozens of
   *triangle fans* (`prim 0x5d`, 1–3 primitives each) have a bounding box
   covering most of the 512x448 window. The course is drawn as triangle
   strips with ADC culling; fans only come out of the course program's
   near-plane clipper (`d159716b`, entry 0x3a08).
3. `gs_replay f.gstrace --packets` (new) prints each GIF tag's vertices.
   The fans contain vertices with **negative Q**, that is w < 0: a point
   behind the eye, kicked as part of a polygon that was just clipped
   against the near plane.

   ```
   seg 1 ev 797 path 1 prim 0x5d nloop 3 nreg 3
     v  0 x    115.25 y   -580.44 z 0x7fffffff q  2.586305e-2 adc 0
     v  1 x    416.56 y     -6.12 z 0xf98fabe8 q -1.337686e-2 adc 0
     v  2 x    360.56 y    744.94 z 0x0b88b3a0 q  2.404207e-2 adc 0
   ```

4. `vu_diff find f.vucap f98fabe8` (new) finds the VU1 invocations that
   kicked that Z word (records 1267, 1279, 1363: `d159716b` pc 0x3a08), and
   `vu_diff trace f.vucap 1267` gives the per-pair state.

## Root cause: the upper op of a WAITQ pair read the old Q

The clipper walks the polygon's edges (Sutherland–Hodgman against
z = -w). For an edge that crosses the plane it computes
t = (z0 + w0) / ((z0 + w0) - (z1 + w1)) with `DIV` and then interpolates:

```
0x16e8  NOP                      | DIV   Q, vf23w, vf27w
...
0x1700  MULAw.xyzw ACC, vf20, vf0w | SQ ...
0x1710  MADDq.xyzw vf21, vf22, Q | WAITQ          <- new vertex = v0 + t (v1 - v0)
```

The `MADDq` sits in the same pair as `WAITQ`, four pairs after the DIV
(latency 7). On hardware, and in PCSX2, the WAITQ stalls the *whole pair*
before it executes, so the MADD uses the new Q:

- microVU: `mVU_WAITQ` pass 1 does `mVUstall = max(mVUstall, mVUregs.q)`,
  and `analyzeQreg` does the same for DIV/SQRT/RSQRT. The pair's stall is
  applied by `mVUincCycles` before the upper op, which also swaps in the
  new Q instance.
- Interpreter: `_vu1Exec` runs `_vuTestUpperStalls` and
  `_vuTestLowerStalls` (FDIV pipe: wait for `fdiv`) and flushes the pipes
  before the upper op executes.

Our VU model ran the upper op first and only then let `waitq()` advance
the cycle counter and land Q, so the MADD used the *previous* edge's t. In
record 1267 the edge goes from v0 = (-13.3, -16.3, -146.2, w -86.0), behind
the eye, to v1 = (4.27, -21.2, -18.4, w 41.6). The correct t is 0.909; the
stale Q was 0.088 (the previous intersection):

| | x | y | z | w |
| --- | --- | --- | --- | --- |
| old Q (0.088) | -11.76 | -16.76 | -134.92 | **-74.76** |
| new Q (0.909) | 2.67 | -20.74 | -29.99 | 29.99 (on the plane) |

The "clipped" vertex stayed behind the eye. After the perspective divide
it lands on the far side of the screen, so the fan becomes a sliver that
crosses the whole view. Every polygon that straddles the near plane
(walls and rails beside the rider, overhead structures, the sky panels)
was affected, which is why it gets worse in enclosed parts of the course.

The differential test against ps2-web-recomp did not catch it. That
reference has no Q latency at all, and the comparison used `--ref-model`
(Q lands immediately) to remove the "Q/P latency" class, which also hid
this case.

## Fix

- `Vu::stall_q()` (`crates/ssx3-ps2/src/vu/mod.rs`): if a DIV-unit result
  is pending, advance the cycle to its ready time, retire the pipelines,
  and land Q.
- The translator (`vu/recomp.rs`) emits `vu.stall_q();` between
  `begin_pair` and the upper op whenever the pair's lower op is WAITQ, DIV,
  SQRT or RSQRT (`decode::lower_waits_q`). The reference interpreter
  (`vu/reference.rs`) does the same. `generated/vu` was regenerated: 326
  pairs in the 7 programs gained the call.
- Software-pipelined DIVs (a pair with `MULq` + a new `DIV` while the
  previous one is in flight) now also see the previous DIV's result in the
  upper op, as in PCSX2.
- Only lower ops read P, so the EFU/WAITP wait staying inside the pair
  changes no values.

Tests (`vu/tests.rs`): `waitq_pair_upper_reads_new_q`,
`div_pair_upper_reads_previous_div_result` (both fail without the fix),
and `waitq_translation_stalls_before_upper`.

## Tools added

- `gs_replay TRACE --packets [--far N]`: every PATH1/PATH2 PACKED GIF tag
  with its vertices in window coordinates, Z, Q and ADC; vertices more
  than N pixels outside the window are marked `FAR`.
- `vu_diff find CAPTURE HEXWORD`: the captured VU invocations whose XGKICK
  packets contain a 32-bit word, to go from a GS vertex back to the VU
  program run that produced it.

RESULTS_SECTION
