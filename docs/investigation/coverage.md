# Coverage: which game paths have been exercised

Phase 1 is done when every reachable path has run at least once (runtime
misses are recorded in `generated/entries.txt`). Scripted headless runs with
the memory card present; input `frame:buttons:duration` (libpad bits: 0008
Start, 0010 Up, 0040 Down, 4000 Cross).

Single Event menu chain (Cross presses in the race script): 2000 main →
2500 character → 2700 setup "Continue" → 2900 peak (Peak 1; Peak 2/3 listed) →
3100 mode (Race / Freestyle) → 3300 event (Race: Snow Jam, Metro-City,
Happiness) → 3500 "My Rules" Continue → 3700 loading card → 4250 start.

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
| Previews → trailer list (NFS Underground, NFL Street, NBA Street Vol. 2) → trailer playback (EA Games logo, disclaimer) | Down ×2 from Single Event (Multi Play is skipped), Cross | ✅ no misses |
| Multi Play (2 controllers) | `SSX3_INPUT` now takes an optional 4th field for the pad (`frame:bits:len:2`) | ⬜ greyed out: the SIO2 model has pad 2 unplugged (`pads[1].connected = false`, `crates/ssx3-iop/src/sio2.rs`). Make it pluggable (env + second gilrs controller in the window) after the crate split, since `ssx3-iop` changes rebuild all generated crates today |
| Single Event → Peak 1 → Race → Metro-City | race script with `3200:0040:6` before the event Cross at 3300 | ✅ no misses, renders correctly (`coverage-peak1-events.png` row 1) |
| Single Event → Peak 1 → Race → Happiness (head-to-head, 2 riders) | Down ×2 at 3200/3250 | ✅ no misses (row 2) |
| Single Event → Peak 1 → Freestyle (Snow Jam freestyle vs opponent, scoring, timer) | `3000:0040:6` before the mode Cross at 3100 | ✅ no misses (row 3) |
| Peaks 2 and 3, their events | `2850:0010:6` (Up) before the peak Cross: "LOCKED: Get a Peak 2 pass in Conquer the Mountain mode" | ⬜ needs career progress (win passes in Conquer The Mountain) or a progressed save |
| Peak 1 Freestyle events: R&B (slopestyle), Crow's Nest (Big Air), The Junction (Super Pipe), Happiness Jam | Freestyle, then Down ×k (40 frames apart) after 3150 | ✅ all four, no misses; Big Air runs to its "Final run" card (`coverage-peak1-freestyle.png`) |
| Career progression (events in Conquer The Mountain, unlocks, shop) | | ⬜ |
| Results → Replay | race script to 20000, then `20100:0040:6,20200:4000:8` | ✅ replay viewer (web-cam, help overlay), no misses |
| Records, "Next event", "Restart" from results | | ⬜ |

## Progressed save (2026-10-15)

`saves/ssx-3.28551.xps` (SharkPort export of `BASLUS-20772-GAM0001`, found
online by the owner; Conquer The Mountain complete) was imported into a fresh
card with mymcplus (`pip install mymcplus`; `mymcplus card.ps2 format`, then
`mymcplus card.ps2 import save.xps`) and installed as `saves/ssx3-mc0.ps2`
(the previous, unformatted card is kept as `saves/ssx3-mc0.ps2.bak-*`). The
game loads it at boot: character select defaults to Mac with all stats 11.0,
Peak 2/3 unlocked. Scripted runs now navigate one screen the same way as
before (same Cross timings). With all peaks open in Single Event, peaks 2/3
are covered through free play instead of career mode.
