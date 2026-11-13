# Reference tools: Ghidra and PCSX2

## Ghidra and the GhidraMCP backend

In agent sessions the `mcp__ghidra__*` tools load with ToolSearch (`+ghidra decompile`);
the REST endpoints below are the fallback.

### Install

- Ghidra **12.1.4**, user settings dir `~/Library/ghidra/ghidra_12.1.4_PUBLIC`
  (the Homebrew cask lives in `/opt/homebrew/Cellar/ghidra/12.1.4`).
- Extensions installed into `.../Extensions/`:
  - **GhidraMCP** (`com.lauriewired`) — serves the loopback HTTP backend.
  - **ghidra-emotionengine-reloaded** (`chaoticgd`) — adds the R5900 language.

### How GhidraMCP works (v2.0.0)

It is **two** pieces:

1. A Ghidra plugin (`GhidraMCPPlugin`) that starts an in-process HTTP server on
   `127.0.0.1:8080` (configurable under *Edit → Tool Options → GhidraMCP HTTP
   Server*). It is **loopback-only** and only runs when the plugin is enabled in
   an open CodeBrowser with a program loaded.
2. A Python MCP server (`uvx --from git+https://github.com/LaurieWired/GhidraMCP
   ghidra-mcp`) that translates MCP calls into requests to that backend.

Lifecycle: `init()` starts the listener, `dispose()` stops it. Toggling the
plugin checkbox is the start/stop control. If it is disabled the backend is
simply absent (`curl http://127.0.0.1:8080/health` fails).

### Useful REST endpoints (hit directly with curl)

```sh
curl "http://127.0.0.1:8080/health"
curl "http://127.0.0.1:8080/program_info"
curl "http://127.0.0.1:8080/functions?offset=0&limit=2000"
curl "http://127.0.0.1:8080/function?address=0x001001c8"
curl "http://127.0.0.1:8080/disassemble_function?address=0x00100008&limit=5000"
```

Notes:
- List endpoints page; **`/functions` caps a page at 2000** regardless of
  `limit`, so full exports must paginate.
- `/function` returns `body_start`, **`body_end`** (inclusive) and `body_size`.
  This is how exact function extents are obtained.
- `disassemble_function` reports bytes in **raw memory order** (little-endian),
  e.g. `entry` starts `280c0070` = word `0x70000c28`.
- The backend handles requests on a single dedicated thread; sequential calls
  run ~0.8 ms each (≈10 s for 11,844 `/function` queries).


### Program state (as analysed)

- Name `SLUS_207.72`, format ELF, language **`r5900:LE:32:default`** (`MIPS-R5900`),
  image base `00100000`, entry `00100008`, little-endian, 32-bit.
- After Auto Analyze: **11,844 functions**.
- MD5 `9e64f3df7ed6e898061411ce43fe75eb`,
  SHA-256 `1b49d05ca2793922180851b9e1ce9ae2291d61a7863565ac4e71f12e967af7bc`.

## PCSX2 as a reference oracle (PINE)

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

A first comparison (2026-10-14: PCSX2 paused in a Snow Jam race against our frame-5000
RAM dump; the 32 MiB dump over PINE takes ~18 s) found code and static data identical and the
course loaded at the same addresses, so word-level diffs of course/geometry regions are
meaningful; per-frame and rider data differ.
