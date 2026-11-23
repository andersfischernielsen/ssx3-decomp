# Plugin conversion status

Base: fork `npiriou/ssx3-decomp-more` (commit 941d4b6) converted to a plugin of ssx3-decomp
`feat/plugin-events`. Everything below was checked by running (macOS, Garibaldi first, then all
19 events); "not verified" lists what was not.

## Verified (headless, scripted pad input, `tools/menu_run.py`)

- `ssx3 plugins prepare tricky` imports all 19 course archives in about 2 minutes (and
  `ui/tricky.png`, the peak picture). `ssx3 plugins build` links 38 hooks.
- Menu: Single Event, character, Continue shows **Peak Tricky** after the game's three peaks with
  the Tricky logo; Race (9 events) and Freestyle (10) lists. The rows are built by the engine from
  `plugin.toml`; the plugin has no menu code. Frames were looked at.
- All 19 events start from the menu, load and run 7,500 frames: no halt, no panic
  (garibaldi, snowdream, elysium-alps, mesablanca, merqury-city, tokyo-megaplex, aloha-ice-jam,
  alaska, untracked + the freestyle of each + pipedream, trick-tutorial). Frames of several were
  looked at (mesablanca, pipedream, tokyo, alaska).
- Garibaldi race, no steering input, ridden to the finish and the results screen ("Garibaldi -
  Race", 5th of 6, about 36,000 frames). Garibaldi freestyle to its results screen. Restart from
  the results works (rules screen again). Quit returns to Select Peak and a game event then
  starts normally (Snow Jam world BAM).
- Retail untouched: `SSX3_EVENT=ARA1` (full race to results, 45,000 frames) and `ASS1` with the
  plugin loaded give the same RAM as with every plugin hook off (except the two TexMan table-size
  hooks, which differ by design when courses need more than 1500 texture slots).
- Upstream (ssx3-decomp `feat/plugin-events`): the no-plugin gate prints `GATE: race replay RAM
  identical to the baseline`, 0 warnings, `tools/plugin_menu_test.py` all checks pass.

## Not verified / known gaps

- No steering input: races were not won, only ridden out. The sound of the Mtn bank patch was not
  listened to (the log shows the bank is served: `[tricky audio] mountain bank of GARI ...`).
  Movie screens and crowds were not looked at specifically.
- After quitting a Tricky event the game shows the in-game Select Peak (`OV.LUI`), which the
  engine does not patch: no Peak Tricky row there (needs upstream `lui_patch` for OV.LUI and the
  `FLOW < 2` gates of `pluginmenu.rs` lifted).
- Plugin events record no medals or times (engine rule, `docs/plugins.md`).
- `cargo test` of this crate on its own does not build (it needs the engine workspace's generated
  code); the unit tests that existed need the upstream `difftest` (cfg(test)) and were dropped.
- `ssx3-gfx`/`ssx3-ps2`/front-end edits of the fork (GS alpha/gpu edits, ground-mark trace,
  settings) are not converted: not generic hooks; they need either their own upstream change or
  stay out.

## Hooks (38)

Hooks that wrap the engine's ported function with Tricky rules (`core_*.rs`, 33 chains and
replacements: boot, objects, race, results/finish, world texture slots, lazy block pools) and
the Tricky-only replacements `custom_finish_update`, `custom_snap_ai`, `custom_tricky_rail_probe`
and a few more (see `lib.rs tables()`).

Dropped in this round (49 plus the menu modules): every `custom_*` replacement of a function
that upstream now ports, and the 7 replacements of generated code, were compared with the
engine's own function by skipping the hook (`SSX3_TRICKY_SKIP=0xaddr,...`) and comparing the whole
of EE RAM after full runs (Garibaldi race to results and restart 52,000 frames, Garibaldi
freestyle 30,000, 12 other events 8,000 frames). Identical RAM: the hook was redundant, upstream
is authoritative. The ones whose removal changed RAM stayed (chains with Tricky rules, finish
update, AI snap, rail probe). Not covered: code paths no run reached (other courses beyond 8,000
frames, retail multiplayer).

## Upstream changes used (ssx3-decomp, branch feat/plugin-events)

- `plugin_host::add_event_listener` (started/ended events from the menu), used by `core_boot`.
- `native_files::add_read_patch`: the mountain sound bank follows the course
  (`custom_audio_banks`, `core_boot::read_patch`).
- `packetless::set_opaque_blend_hook`: movie-screen blend (`core_race::screen_blend`).
- `ModObj_Render`'s crowd/movie material table is set at the entry of the draw call it makes
  (`core_race::draw_model`), no upstream change.
- `hookable.txt`: only `GameModule_QuitToLoadScreen` (the engine's own menu) is left.
- `plugin_host::overlays()`.
