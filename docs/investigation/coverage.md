# Coverage: which game paths have been exercised

Phase 1 is done when every reachable path has run at least once (runtime
misses are recorded in `generated/entries.txt`). Scripted headless runs with
the memory card present; input `frame:buttons:duration` (libpad bits: 0008
Start, 0010 Up, 0040 Down, 4000 Cross).

Main menu (frame ~1700 after Start at 1600): Single Event, Conquer The
Mountain, Multi Play (greyed: needs a second controller; skipped by the
cursor), Previews, Online.

| Path | Script | Result |
| --- | --- | --- |
| Boot, movies, title, main menu | any | ✅ no misses |
| Options → Save/Load (memory card) | see `sio2-memcard.md` | ✅ save + load |
| Single Event → Race → Snow Jam, full race to results | `playable-push.md` race script, to frame 20000 | ✅ no misses; renders correctly after the WAITQ + clip-latency fixes (`race-geometry.md`) |
| Conquer The Mountain → character select → intro flight → free ride (0/44 collectibles) | `1600:0008:8,1800:0040:6,1900:4000:8,2300:4000:8,…` every 400 frames to 4300 | ✅ no misses (`coverage-conquer-the-mountain.png`; taken before the geometry fix) |
| Online → network configuration | Down ×3 from Single Event, then Cross | ✅ no misses; shows "memory card … unformatted" for the network-config lookup although saves work (probably a BWNETCNF/mc getInfo detail; online is dead anyway) |
| Previews | not reached yet | ⬜ |
| Multi Play (2 controllers) | needs pad 2 input in `SSX3_INPUT` | ⬜ |
| Other Single Event types (freestyle, slopestyle, superpipe, big air), other courses | | ⬜ |
| Career progression (events in Conquer The Mountain, unlocks, shop) | | ⬜ |
| Replays, records, "Next event", "Restart" from results | | ⬜ |
