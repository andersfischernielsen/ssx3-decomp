# Phase 3, subsystem 1: native files

Game files come from the host through `std::fs` instead of the emulated CD
(`crates/ssx3-game/src/native_files.rs`). This is the first phase 3 subsystem;
`phase3-boundaries.md` §1 explains how the seam was chosen.

## Use

| Variable | Effect |
|---|---|
| `SSX3_NATIVE_FILES=1` | Turn native files on (off by default, so the default path stays byte-identical). |
| `SSX3_DATA_DIR=<dir>` | Read from an extracted or mounted copy of the disc (any case: `DATA/UI/...` matches `data/ui/...`). Files can be replaced on the host. If the directory does not exist, the game keeps the emulated CD. |
| (neither) | Read from the disc image by ISO9660 name (`SSX3_ISO`, default `SSX 3 (USA).iso`). |
| `SSX3_FILE_LOG=<file>` | In either mode, log every completed read as `name offset size fnv64`. Compare two logs with `tools/compare-file-logs.py`. |

## The seam

EA's file library has a platform layer of six functions, each called only by
the request queue (`EAFile_ServiceQueue`). Everything above that layer is
unchanged generated game code: the priority queue, BIG archive lookup,
`AsyncFile`/`StreamFile`, callbacks, the sync wrappers.

| Function | Native behaviour |
|---|---|
| `EAFile_PlatformOpen` 0x3e3208 | Resolves the path as `EAFile_ResolvePath` does (`cd:` prefix, root path, `\` → `/`). A `cd:` file is looked up on the host with `EAFile_CdLookup`'s name rule (upper case, no leading `/`) and gets handle `3 << 24 \| index` (device 3 = native). `host0:` paths keep the generated body. |
| `EAFile_PlatformRead` 0x3e3350 | For a device-3 handle: copies the bytes into EE RAM, then calls `EAFile_CompleteRequest(ok)`, as the CD path does for a sector it already holds. Other handles keep the phase 2 replacement. |
| `EAFile_PlatformGetSize` 0x3e34e8, `EAFile_PlatformClose` 0x3e32f8 | Answered for device 3; generated body otherwise. |
| `EAFile_HostReadService` 0x3e33b0 | Finishes a read stranded by a guest thread switch (below), then runs the generated body. |

Reads inside a BIG archive arrive as `PlatformRead(big handle, buf, big offset
+ offset)`, so the BIG layer needs nothing.

### Completion without recursion

Completing a request runs its callback, which often queues the next read
(`AsyncFile` chains open → size → 32 KB chunks). `EAFile_CompleteRequest` then
calls `EAFile_ServiceQueue`, which dispatches that read into `PlatformRead`
again. Completing synchronously would therefore nest one level per chunk.

Instead, a read issued while a native read is completing is parked, and the
outer call runs it in a loop, so nesting stays flat.

The loop's frame has the same shape as `EAFile_PlatformRead`'s, and it calls
`CompleteRequest` with that function's return address. If a callback switches
guest threads (host-thread unwind), the generated body therefore resumes and
returns correctly. A read parked at that moment is finished by the next
`EAFile_HostReadService`, the timer task that runs every frame.

## Verification

Run the race script to frame 5000 with emulated and with native files, and
compare the two `SSX3_FILE_LOG` logs (2026-10-15):

- **Reads:** 975 reads (909 distinct) in each run, with **0 content mismatches**. Only the
  order differs, because native reads complete sooner.
- **Race:** the native run reaches the same race at the same time. Frame 5000 shows the same
  scene with small rider position differences, from load timing.
- **Source:** a run from a mounted copy of the disc (`SSX3_DATA_DIR=/Volumes/SSX3`) is
  byte-identical to a run from the image at frame 5000. Native runs are deterministic.
- **Full race:** to frame 9000, both runs complete the race with no misses and no halts. 1,021 distinct reads are common to both, with 0 content mismatches. The roughly 30 reads seen in only one run are music-stream positions and speech lines, which differ because play diverged slightly.
- **Emulated CD traffic:** 2,101 `sceCdRead` RPCs in the emulated run, 20 single sectors in
  the native one.
- **Default path:** without the variables, the override A/B replay stays byte-identical.

## What still goes through the emulated CD

`EAFile_CdInit` at boot still runs: `sceCdInit`, disc type, TOC, and its walk
of the disc's directories into the CD name table (the 20 sectors above).

The walk cannot simply be skipped: `EAFile_CdInit` returns the number of
entries, and startup treats 0 as "no file system". Removing it means either
overriding `EAFile_CdInit` (and the CD read thread it starts) or filling the
table natively. Neither is needed for correctness.

Movies (`AV_Player`) and streams (`SNDSTRM`) read through `StreamFile` →
EAFile, so they are native already. The memory card is a separate path (SIO2).

## Next

Make native files the default once the windowed game has been played through
with it; extract `DATA/` with a tool (`tools/extract-from-iso.sh` only takes
the ELF); then the frame loop and sound (`phase3-boundaries.md` §2-3).
