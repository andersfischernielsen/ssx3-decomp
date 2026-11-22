# Plugin conversion status

Base: fork `npiriou/ssx3-decomp-more` (commit 941d4b6, based on ssx3-decomp 5082230) converted to a
plugin of ssx3-decomp `feat/plugin-events`. Nothing here was verified beyond what the
"Verified" section says.

## Verified

- `ssx3 plugins list` finds the plugin; `ssx3 plugins prepare tricky` (with
  `SSX3_TRICKY_COURSES=garibaldi`) imports Garibaldi race and freestyle from the Tricky disc
  into the cache folder (importer needed two Python 3.9 fixes).
- `ssx3 plugins build` compiles the crate; 107 hooks install, every address has a slot.
- `SSX3_EVENT=tricky/garibaldi target/game/ssx3-custom-headless 900`: boots into the event, reads
  `data/worlds/TR_GARI_R.*` from the overlay (292 reads in the file log, no `BAM` world), the
  Tricky course rules and rail code run (log lines `[tricky] ...`), the run ends without a halt
  or panic. Not looked at: the picture, a full race, the finish.
- Not run: the other 18 events, the menu flow ("Peak Tricky" rows), the gate (the plugin is not
  loaded by it), any differential test.

## Module status

| group | count | status |
| --- | --- | --- |
| `custom_*` modules installed by the fork (`lib.rs overrides()`) | 65 override tables (about 66 functions) | converted: `Hooks::replace` (or `chain` for the 4 that call the wrapped body: `custom_rail_diagnostics`, `custom_tricky_rail_probe`, `custom_snap_ai`, `custom_results_enter`). 57 of the addresses are now ported upstream too: the plugin's version replaces upstream's. Not differentially compared. |
| helper modules (`custom_tracks`, `_course_rules`, `_movies`, `_crowds`, `_fences`, ...) | 25 | converted (compile), called from the hooks |
| `bonus_menu_mode` | 1 | converted (compile); menu not run |
| retained audit ports (never installed in the fork) | 133 | moved to `retained/`, not compiled (they call `ssx3_ee::...` generated paths) |
| test-only modules of the fork (`custom_multi_spline_nodes`, `custom_texman_release`) | 2 | moved to `retained/`: need upstream's cfg(test) `difftest` |
| unit tests of the modules | 3 | dropped (`custom_course_rules`, `custom_nash_insert` need `difftest`); the catalog test is kept |

## The fork's in-place edits of core files

Written by `src/core_boot.rs` (mine), `core_object.rs`, `core_race.rs`, `core_ui.rs`
(each has a table in its module doc). W = wrapper (`chain`), R = replacement.

| edit | status |
| --- | --- |
| `native_files.rs`: asset roots, world texture slots, archive redirect | roots: `core_boot::configure`; archive redirect: replaced by the engine's world swap + overlay (importer names archives `TR_<CODE>_<R/F>.BIG`, `prepare_track_info` writes the name in the course record). World-texture-slot scan of the archives: NOT converted (slots stay 1500; Tricky courses need up to 1590 -> `TexMan_InitTable` wrapper exists but reads the unset value) |
| `native_files.rs`: object-sound bank patching on reads (`custom_audio_banks`) | NOT converted: needs a file-read hook in the engine |
| `event_boot.rs` (`SSX3_EVENT=<course>`, track info) | engine's `SSX3_EVENT=tricky/<id>` + `core_boot` chain on `FE_SetOnlineMode` |
| `state.rs`, `beintnewrace.rs`, `belibrary.rs` (track info/name, commit, high score) | W |
| `ai.rs`, `computer.rs` (restart, distance, far-behind/ahead, path events) | W. `Rider_UpdateRaceProgress` boost: runs BEFORE the function instead of mid-function (timing differs) |
| `camera.rs` `GameCamera_AvoidCollision` chase logic | W as a post-step; cycle count approximate |
| `riderspheretree.rs`, `worldsphtree.rs`, `ssxscriptengine.rs`, `capture.rs` | W; the per-candidate contact trace (diagnostic) skipped |
| `railmodifier.rs`, `debouncenode.rs`, `flagset.rs`, `movenode.rs` call sites | W/R: ModObj_v41, Counter_Update/HandleEvent, Fence_Update, TexMan_InitTable, RollerModifier_Update, Emitter_Render. NOT converted: `ModObj_Render` (crowd and movie-screen materials mid-function; needs an upstream hook or copying 333 lines with private helpers) |
| `packetless/model.rs` movie-screen blend | NOT converted: needs an upstream hook in `packetless::draw_model` |
| `uitemplatemap.rs`, `festateload.rs`, `ovstates5.rs`, `uitext.rs` (Tricky menu) | W with recomputed conditions (`EventSelect_OnWidgetAction` is approximate); not run |
| `game.rs`, `worldcache.rs`, `ps2graphicsman.rs`, `gamestate.rs`, `loadscreens_prestart.rs` | W (section remap, lazy world blocks, results exit) ; not run |
| `containers.rs` heap diagnostics | skipped (diagnostic) |
| `ssx3-gfx`, `ssx3-ps2`, `ssx3-render`, `ssx3` (front end) edits | NOT converted: rendering and front-end changes outside the game-function layer (GS alpha/gpu edits, ground-mark trace, settings), see the fork diff |

## Upstream changes (ssx3-decomp, branch feat/plugin-events)

- `hookable.txt`: 10 addresses (9 Tricky functions that were not ported + `EventSelect_GetEventRecord`), regenerated.
- `plugin_host::overlays()` (+ engine call): the plugin finds its derived content next to its overlay.
