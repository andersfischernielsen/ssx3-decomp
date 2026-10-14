# PCSX2 as a reference oracle (PINE)

PCSX2 v2.8.2 (installed at `/Applications/PCSX2.app`) runs the same disc and is
the behavioural reference. It has no remote debug server; its only remote
interface is **PINE** (Settings → Advanced → Enable PINE; it starts when a game
boots). On macOS it is a Unix socket: `$TMPDIR/pcsx2.sock`.

`tools/pine.py` is a minimal client:

```sh
tools/pine.py info                       # PCSX2 v2.8.2 | SSX 3 | SLUS-20772 1.00 | running
tools/pine.py read32 0x4a19cc 4          # EE memory reads (read8/16/32/64)
tools/pine.py write32 0x4a19cc 0         # writes
tools/pine.py dump 0x100000 0x1000 f.bin # batched 64-bit block read (size multiple of 8)
tools/pine.py save 1 / load 1            # savestate slots
```
From Python: `sys.path.insert(0, "tools"); from pine import Pine`.

What PINE can do: EE main memory (incl. scratchpad? untested) read/write,
status, title/ID/version, savestate save/load. What it can't: CPU/VU registers,
VU memory, GS registers/VRAM, breakpoints, single-stepping. For GS/VRAM-level
comparison the alternative is a PCSX2 GS dump (`.gs`) and teaching `gs_replay`
to read it.

Verified 2026-10-14: the code segment in PCSX2 RAM (first 128 KiB from
0x100000) is word-identical to `orig/SLUS_207.72`.

Using it for differential debugging: PCSX2's RTC is the host clock and the game
seeds its RNG from it (rider line-up etc.), so line runs up by game state, not
frame numbers: e.g. compare the camera/rider/matrix structures in EE RAM once
both sides are at the same race position, or compare static per-course data
(display lists, vertex source buffers, VIF packets built by the EE) which does
not depend on the RNG.
