# Plugins (extra content on top of SSX 3)

Goal: `ssx3-decomp` stays "SSX 3". Anything else (courses from SSX Tricky,
SSX On Tour or the original SSX; later riders, music, modes) lives in plugin
folders that the engine discovers and loads. Every plugin course appears as its
own peak on the mountain. The first user is the SSX Tricky work of
[`npiriou/ssx3-decomp-more`](https://github.com/npiriou/ssx3-decomp-more).

With no plugin the game is unchanged: no plugin code runs, and the race RAM
gate stays identical.

Status (2026-11-22): S0 to S3 are implemented on the branch `feat/plugins`
(see "Stages"); S4 onwards is not started. The author guide is
[`docs/plugins.md`](../plugins.md).

## The source games (surveyed 2026-11-22, directory listings and headers only)

| Disc | Boot file | World data |
| --- | --- | --- |
| SSX 3 (USA) | `SLUS_207.72` | one `DATA/WORLDS/BAM.BIG` (113 MB): `bam.sdb`, `.ssb`, `.phm`, `.psm` |
| SSX On Tour (USA) | `SLUS_212.78` | `DATA/WORLDS/BAM.BIG` (193 MB): the same four files plus 25 `.tp` / `.sp` pairs; `BIG4` archives, `SCHl` movies |
| SSX Tricky (USA) | `SLUS_203.26` | one RefPack-compressed `DATA/MODELS/<course>.BIG` per course |
| SSX (2000, Europe PAL) | `SLES_500.30` | one `DATA/MODELS/<course>.BIG` per course: `.aip .map .wfx .wds .wdx .wdf .wdr .mdr .ssh` |

Consequences:

- Three different world formats, so a converter (importer) per source game is
  unavoidable. The contract between importer and engine is SSX 3's own course
  data (the `sdb/ssb/phm/psm` world, race paths, start grid, textures).
- On Tour uses the same container and the same four world files as SSX 3 and
  is the likeliest to convert cheaply. What its `.tp` / `.sp` files are is not
  yet known.
- The original SSX is the furthest from SSX 3 (older `wd*` files). Its courses
  mostly share names with Tricky's, so every ID is namespaced by plugin.
- The movie containers differ (`MPCh` against `SCHl`); a plugin that wants its
  movies needs a reader.
- A disc check accepts a list of serials: the original SSX image here is PAL.
- Formats: the engine reads only plain `.iso` (the SSX 3 image) in its core; the
  plugin layer also reads raw single-track `.bin` to match a boot serial. Any
  other image format is the plugin's: its manifest names a `detect` command and
  the extensions to try, and its importer reads the image itself.

## A plugin is a folder

```
plugins/tricky/
  plugin.toml      id, api version, inputs (disc serials), peaks, events
  content/         converted files, written by prepare (never in git)
  prepare/         the importer (any language it likes; run by the engine)
  src/, Cargo.toml optional: Rust code, for hooks and new mechanics
```

```toml
[plugin]   id = "tricky"  api = "0.1"
[inputs]   disc = { serials = ["SLUS-20326"] }
[[peak]]   id = "tricky"  name = "Peak Tricky"  picture = "ui/peak.png"
[[event]]  id = "tricky/garibaldi-race"  peak = "tricky"  kind = "race"
           template = "peak1-race1"  world = "garibaldi"
```

- **Data-only plugins** are read at run time: nothing to build. This covers a
  new peak of courses whose behaviour the template event already gives.
- **Plugins with code** are Rust crates in the same folder. They are not loaded
  as libraries. `ssx3 plugins build` generates a workspace with the enabled
  plugin crates and builds one front end with them linked in, as `ssx3-web`
  links the engine. Adding or removing a code plugin needs that one command.
- A plugin activates only when its inputs exist (for Tricky: a Tricky disc
  image); otherwise it is skipped silently.
- Rust only: there is no C interface and no plugin binary format.

## How a plugin's world is loaded: per event, not merged

`cGame_loadTrack` (0x22f6b0) builds `data/worlds/<location>` for each event
from a location name in the event's record, opens `<path>.big`, and builds the
world, sky box and section manager from scratch (they are destroyed when the
event ends). Every retail event says `BAM`, but the game already opens a
world archive by name per event.

So a plugin event carries its own location name, and the file layer serves that
plugin's archive (`data/worlds/<id>.big`, complete with its own IDs). SSX 3
events still say `BAM`. Nothing is merged and nothing is renumbered, so plugins
cannot disturb each other. The Tricky fork instead writes a modified copy of
`BAM.BIG` with its courses appended; that does not compose (the retail SDB keeps
next-free texture and lightmap IDs, 788 and 623 on the USA disc), so it stays a
fallback for a course that needs resources from two archives.

The four things to verify before relying on it, with what was found
(2026-11-22; "S0 result" below has the run, "What a world needs" the code reading):

1. Nothing else in the load path assumes the location is `BAM`. **Checked** for
   a renamed copy of `BAM`.
2. The texture slot pools are sized once at boot. **Read in the code**: a
   world with more than 1,500 textures needs a boot-time setting.
3. The section and event tables in guest memory are indexed per event.
   **Read in the code**: a world with the same section names as the template
   event works as it is; one with a different layout needs the tables patched.
4. Unloading an event leaves no state behind, so that two events with
   different archives can follow each other in one run. **Checked** for
   `BAM` then `BAX` in one process.

### S0 result (2026-11-22)

`examples/plugins/world-copy`: the importer makes `DATA/WORLDS/BAX.BIG` from
SSX 3's own `BAM.BIG` (the four world entries renamed `bam.*` to `bax.*`, the
rest byte for byte) in the cache folder, and a hook on `cGame_loadTrack`
(0x22f6b0) changes the location in the race record from `BAM` to `BAX`. With
`SSX3_EVENT=ARA1` run headless for 1,300 frames:

- the game opened `data/worlds/BAX.big`, `BAX.sdb` and `BAX.ssb`, served from
  the plugin's overlay, and the log shows no `BAM` read;
- the sequence of file reads (names with `BAX` read as `BAM`, offsets, sizes)
  is identical to the stock run; only the first 2 KB of the archive differ (its
  renamed directory);
- the frames at 650 and 1,300 are byte-identical to the stock run's;
- the game did not halt.

So an event can load a world archive chosen by the plugin, through the file
overlay, with no change to the archive's contents and nothing merged.

**Two archives in one run.** The hook counts world loads (`SSX3_WORLD_COPY=2`:
swap from the second load on). The run: the Alaska-start event `ARA1` from
`SSX3_EVENT`, the pause menu (Start, Down x5), Quit Game (Up, X), which goes
back to the title and destroys the world, then the menu route of
`tools/scenarios.py` for a second event (`p1_race_snow_jam`, shifted to start
at the title). The log shows `world load #1` (`BAM.big`) and `world load #2`
(`BAX.big`), the second race is under way at frame 5,750 (4th of 6, 25
seconds in), its frames at 5,500 and 5,750 are byte-identical to the same run
without the plugin, and the game did not halt. (The pause menu's Restart
does not reload the world: the archive is opened once.) With
`SSX3_WORLD_COPY=3` the hook renamed the record's location in place and never
put `BAM` back, so both loads opened `BAX` (frames again identical): `BAX`
then `BAM` is not tested, and a hook that renames must restore the name.

What none of this shows: a world with different data (it is a copy of SSX 3's
own).

### What a world needs (read from the code, 2026-11-22; not yet run)

**Texture pool.** `TexMan` is a fixed table of 2,000 entries. Ids 0 to 1,499
are "bound" (caller-chosen: world textures, `SDB` shared images at +0x2A and
lightmaps at +0x2C, 788 + 623 = 1,411 in retail) and 1,500 to 1,999 are a
free list for dynamic ones. The 1,500 is written into `TexMan_InitTable`
(0x3670e0, ported in `railmodifier.rs`), `TexMan_FreeNBTexInfo` (0x3671c8) and
`TEX_FIRST_NB` in `TexMan_FreeTexture` (0x367bc0); nothing sizes it from the
archive. A world with more than 1,500 textures would collide with live dynamic
ones and corrupt the free list; ids of 2,000 or more overwrite the head of the
free list. The Tricky fork reads the counts from `BAM.BIG` before boot and
moves the boundary in the first two functions. For the plugin API: a manifest
field `world-textures = N`; the engine takes the maximum of 1,500 and the
values of the enabled plugins (and rejects 1,999 or more) as a host value
read by those three functions, before the guest starts. Worlds of SSX 3's own
size need nothing.

**Section and event tables.** `SectionMan_Init` (0x22cd40) builds the logical
section table at 0x44216c (50 entries of 16 bytes) at run time: for each slot it
takes a name from the event interface and matches it by string against the
section names in the world's `SDB` (from +80, 88 bytes each); the first match
is the slot's world section. The event table at 0x442488 (rows of 0x28 bytes,
indexed by event or track id) is static data in the executable: +4 the number
of sections (up to 6), +8 the sky slot, +0xc the shared slot, +0x10 the
logical slot ids. Nothing in the game writes it. So a renamed copy of `BAM`
works because its section names are the same. A world with another layout
needs the slot-to-section mapping and the template event's row changed for
the event, and put back when it ends. This is what the fork does: it keeps a
native event as the template, overwrites one logical slot with its world's
section index, edits the template's row in place, restores both on exit, and
makes `SectionMan_IsEventLoaded` accept a missing sky or shared section. For
the plugin API: per event, `template` plus the world's section names (at most
6, optional `sky` and `shared`), applied by a typed hook after
`SectionMan_Init`, with the restore on unload and the `IsEventLoaded`
tolerance built in, so a plugin states data and does not patch tables. This is
inferred from reading the fork and the ported functions; it needs the ports
of S4 and a run with a world of another layout to confirm.

## What a plugin can touch

1. **Prepare.** A one-time import from the user's own discs into the cache
   directory (`paths.cache_root/<id>/`). May be slow; skipped when current.
2. **Data overlay.** Files that replace or extend the disc's by path, in load
   order; the ISO is the fallback. `native_files` already serves files from
   the host, and the fork's `SSX3_DATA_OVERLAY_DIR` is a small delta on it.
   `SSX3_FILE_LOG` shows which source served each read.
3. **Catalog.** Extra peaks and events as data, with stable string IDs. The
   guest's fixed tables (the 17-entry `TrackInfo`, save records) are never
   indexed by a plugin; the engine maps IDs to runtime slots.
4. **Hooks.** Wrapping or replacing game functions at their override slots
   (code plugins only).
5. **Saves.** Best times and unlocks go in plugin-owned files, never in the
   memory card format.

```rust
pub trait Plugin: Send + Sync {
    fn id(&self) -> &'static str;
    /// Code plugins: install hooks, after the built-in overrides.
    fn install(&self, hooks: &mut Hooks);
}
```

`hooks.chain(addr, f)` puts `f` in the function's slot and keeps what was
there; `f` runs the previous body with `hooks::call_prev(c, addr)`. This is the
chain `difftest::fakes::hook` and `fall_back` already use in the tests, made a
public API. It fails (does not panic) when the address has no slot.

**Reach.** Only addresses in `overrides.txt` have a slot (7,438 of 13,306
symbols at 2026-11-22). Every ported function has one, so the reach grows with
the port and ends up covering everything. Until then
`crates/ssx3-game/hookable.txt` lists addresses that always get a slot;
plugins ask for additions by pull request, and a plugin's CI checks that its
addresses are slotted.

**Typed extension points** come as functions are ported natively (event start
and finish, race rules, the peak and event lists, roster, audio banks); a
plugin prefers them over raw hooks. Raw hooks stay as the escape hatch.

## Rules

- Load order, with a version check (`api`) and conflict detection: two
  plugins that replace (not chain) one slot is an error with a clear message.
  Settings list the plugins; each can be disabled, and `SSX3_PLUGINS=0` turns
  all off.
- Data-only plugins are inert. A code plugin runs with the process's rights:
  the user builds it, from source, and nothing loads a binary from elsewhere.
- No game data in any plugin repository; derived data lives in the user's
  cache. The same CI check as the main repository.
- With no plugin loaded the gate is identical. The main repository tests the
  mechanism with a small built-in test plugin (one hook, one overlay file, one
  peak with one event); a plugin tests its own behaviour.
- The API is pre-1.0: a plugin pins a commit of `ssx3-decomp`.

## Stages

| Stage | What | Done when |
| --- | --- | --- |
| S0 | **Spike**: one event loads a copy of `BAM` under another location name | done: one event, and `BAM` then `BAX` in one run; checks 2 and 3 are read from the code only |
| S1 | Generalise the data overlay in `native_files`; per-plugin overlays in load order | **done** (#67): a file in the overlay shadows the ISO; unlisted paths fall back; gate identical |
| S2 | `hookable.txt`, `Hooks::chain`, `call_prev` | **done** (#67): two chained hooks end at the generated body; a missing slot is an error |
| S3 | Folder discovery, `plugin.toml`, `prepare`, `ssx3 plugins build`, the built-in test plugin | **done** (#67): the test plugin loads, hooks and serves an overlay file |
| S4 | Port the front-end and game-state functions the Tricky fork overrides and are still generated (32 at 2026-11-22: `EventSelect_*`, `GameStatePreRaceLoad_*`, `GameStateFinish_Update`, `GameStateResults_Enter`, `GameModeMan_OnRiderFinished`, UI text and screen creation, `BEScoreInt_SubmitRaceResult`, ...) | each has a differential test |
| S5 | `Catalog`; the Peak and event lists read it | the test plugin adds a peak with one event to the menu |
| S6 | Tricky as the first real plugin, with its author | the 19 events start, run and finish |
| S7 | A second source (On Tour first) | its first course runs, proving the design is general |

S0 to S3 are small and independent of the Tricky work. S4 is ordinary porting
and useful on its own.

## What the Tricky fork becomes

| Today in the fork | In the plugin |
| --- | --- |
| `tools/prepare_tricky_tracks.py` and the `ssx3_*` / `tricky_*` Python | `prepare/` |
| `.cache/overlay`, `custom-tracks.ready` | `content/` |
| `custom_tracks.rs`, `tricky_catalog.rs`, the bonus menu | `plugin.toml` peaks and events |
| 69 installed overrides | code: for the 37 this repository has since ported, chain hooks on the typed versions (or drop them where equivalent); for the other 32, after S4, catalog entries or small branches |
| 129 `custom_*.rs` files marked uninstalled | stay in the fork until installed and tested |
| frame pacing, halt snapshots, other engine changes | separate pull requests here, if wanted |

## Open questions

- Who owns the Tricky plugin repository (the fork's author, presumably)?
- Python or Rust for importers, and how they run on Windows, macOS and Linux.
- Whether S0 shows that the swap needs engine changes beyond the file layer.
- The fork names functions differently (`overlap-functions.tsv` maps 9,515
  addresses between the two names); a plugin uses this repository's names.
- The browser: the overlay would need a second disc image fetched by the page,
  and code plugins need a rebuild of the web front end. Out of scope until
  native works.
