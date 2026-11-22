# Plugins (extra content on top of SSX 3)

Goal: `ssx3-decomp` stays "SSX 3". Anything else (courses from SSX Tricky,
SSX On Tour or the original SSX; later riders, music, modes) lives in plugin
folders that the engine discovers and loads. Every plugin course appears as its
own peak on the mountain. The first user is the SSX Tricky work of
[`npiriou/ssx3-decomp-more`](https://github.com/npiriou/ssx3-decomp-more).

With no plugin the game is unchanged: no plugin code runs, and the race RAM
gate stays identical.

Status (2026-11-22): proposal. Nothing here is implemented, and the spike in
"Stages" has not been run.

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

To verify before relying on it:

1. Nothing else in the load path assumes the location is `BAM`.
2. The texture slot pools are sized once at boot (the fork reads 1,500 from the
   archive): size them for the largest world a plugin declares.
3. The section and event tables in guest memory are indexed per event: the
   catalog supplies the entries for a plugin's events.
4. Unloading an event leaves no state behind in global tables.

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
| S0 | **Spike**: one event loads a copy of `BAM` under another location name | the event runs; the four checks above are answered |
| S1 | Generalise the data overlay in `native_files`; per-plugin overlays in load order | a file in the overlay shadows the ISO; unlisted paths fall back; gate identical |
| S2 | `hookable.txt`, `Hooks::chain`, `call_prev` | two chained hooks end at the generated body; a missing slot is an error |
| S3 | Folder discovery, `plugin.toml`, `prepare`, `ssx3 plugins build`, the built-in test plugin | the test plugin loads, hooks and serves an overlay file |
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
