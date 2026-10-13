# The disc image and the game executable

## The ISO is a container, not a program

`SSX 3 (USA).iso` (3,005,415,424 bytes) is a **UDF 1.5** filesystem, volume label
`SSX3`. It cannot be decompiled directly; the program is an ELF inside it. macOS
mounts it read-only (`hdiutil attach -nobrowse -readonly`). Disc root:

| Entry | Size | Notes |
| --- | --- | --- |
| `SYSTEM.CNF` | 49 B | boot config |
| `SLUS_207.72` | 3,890,784 B | **the game executable** |
| `DATA/` | ~2.3 GB | level/asset archives |
| `NETGUI/` | ~8 MB | network UI |
| `PAD0.000`, `PAD1.000` | 256 MB each | disc padding |

`SYSTEM.CNF`:

```
BOOT2=cdrom0:\SLUS_207.72;1
VER=1.00
VMODE=NTSC
```

`tools/extract-from-iso.sh` mounts the image and copies `SLUS_207.72` and
`SYSTEM.CNF` into `orig/` (gitignored).

## The executable

- ELF 32-bit LSB, **MIPS** (MIPS-III), statically linked, **stripped**.
- Entry `0x00100008`, image base `0x00100000`.
- SHA-256 `1b49d05ca2793922180851b9e1ce9ae2291d61a7863565ac4e71f12e967af7bc`.

Even though it is stripped, it was built with `-ffunction-sections`, so section
names still carry mangled C++ names (`.gnu.linkonce.t.CreateFrame__23PS2_SONY_CODEC_INTERNALUiUi`,
`..._Q24RCMP7DECODER`, ...) — a free naming source.

## EE memory map (Ghidra segments)

Program image:

| Segment | Range | Size |
| --- | --- | --- |
| `.text` | `00100000`–`0042e01f` | 3,334,176 |
| `.gnu.linkonce.t.*` | `0042e020`–`0042e587` | — |
| `.vutext` | `0042e590`–`0043cd6f` | 59,360 (VU1 microcode) |
| `.data` | `0043ce00`–`0045680b` | 104,972 |
| `.rodata` | `00456900`–`0049b017` | 280,344 |
| `.lit4` | `0049b100`–`004a0e57` | 23,896 |
| `.sdata` | `004a0e80`–`004a4bf3` | 15,732 |
| `.sbss` | `004a4c00`–`004a5bbb` | 4,028 |
| `.bss` | `004a5c00`–`0053eadb` | 626,396 |

Hardware windows:

| Window | Base | Size | Notes |
| --- | --- | --- | --- |
| `registers.io` | `10000000` | 64 KiB | timers, DMA, INTC |
| `vu0.code` / `vu0.data` | `11000000` / `11004000` | 4 KiB each | |
| `vu1.code` / `vu1.data` | `11008000` / `1100c000` | 16 KiB each | |
| `registers.gs` | `12000000` | 8 KiB | GS privileged regs |
| `iop_ram` | `1c000000` | 2 MiB (seen in a 32 MiB window) | IOP RAM |
| `RAM(uncached)` | `20000000` | 32 MiB | alias of main RAM |
| `RAM(uncached+accel)` | `30100000` | — | alias based at `30000000` |
| `scratchpad` | `70000000` | 16 KiB | SPR |
| BIOS ROM | `9fc00000` / `bfc00000` | 4 MiB | cached / uncached |

**Alias subtlety:** the accelerated RAM alias is a full 32 MiB mirror based at
`0x30000000`, so `0x30100000` maps to physical `0x00100000` (an offset of
`0x100000`, not 0). Ghidra labels the window from `0x30100000` because the first
MiB is not defined there.

## `entry`

112 instructions (448 bytes), the standard EE crt0 prologue: `clear` all GPRs,
`mthi $0`/`mtlo $0` and the MMI `mthi1`/`mtlo1`/`mtc0`/`mfc1` forms, then
`syscall(0x3c)` / `syscall(0x3d)` = **InitMainThread / InitHeap**, `EI`, and a
jump into the C runtime.
