# Plugin peaks and events on the Select Peak screen

Status: design, 2026-11-22. Nothing here is implemented. "(verified)" means read in this
repository's code or in the fork; "(unverified)" means inferred and needing a run (Ghidra was
not reachable while writing this).

Goal: a plugin's `plugin.toml` (`[[peak]]` id/name/picture, `[[event]]` id/peak/kind/template/world,
`crates/ssx3-plugin/src/catalog.rs`) is enough to get its peaks onto the Select Peak screen and its
events into the event lists, with no menu hooks in the plugin. Plugins are not only Tricky or On Tour:
community-made maps (any number of peaks and events, content shipped in the plugin folder, no official
disc involved) must work the same way.

Fork reference: `ssx3-decomp-more` (SSX Tricky fork), called "the fork" below.

## 1. How the vanilla screen is built and navigated

### Screen and state

- The screen is the "Event Select" screen (id 0x5380) of `DATA/UI/FE.LUI` (front end) and `OV.LUI`
  (pause-menu map). The C++ name is `cUITemplate_MAP`; the Rust is `EventSelect_*` in
  `crates/ssx3-game/src/game/fe/uitemplatemap.rs` (plus `ovstates5.rs`, `festateload.rs`
  `cFEStateEventSelect_*`). One template object (`this`, size about 0x300) holds the state:
  - `+4/+8/+0xc` peak unlocked flags (3), `+0x10..0x1c` tab unlocked flags (4),
  - `+0x20` `available[peak][tab][j]`: stride 0x60 per peak, 0x20 per tab, 4 per row, i.e. `[3][3][8]`
    (ends at 0x140); `+0x140` `completed[3][3][8]` (ends at 0x260),
  - `+0x260` a copy of the chosen 0x6c-byte event record, `+0x2cc` confirm-popup flag, `+0x2d0` popup
    selection, `+0x2d4` loaded picture variant, `+0x2d8` async slot, `+0x2dc` picture loaded,
    `+0x2e4` map picture widget, `+0x2e8` screen, `+0x2ec` owning UI state, `+0x2f0` picture disabled.
  - gp globals: mode `gp-0xb54` (0 peaks, 1 a peak's goals, 2 an event list), tab `gp-0xb50`,
    peak `gp-0xb4c`, flow `gp-0xb5c` (0/1 front end single event, 2+ challenge variants, 3/4 in-game
    overlay), world singleton `gp-0x848`, `gp-0x1f38` "event started from the list".
- Widget tree (names are `GetHashValue32` hashes, found with `UIScreen_FindObject` 0x39d860; shown or
  hidden by name through the UI state, `UIState_ShowObject` 0x39e948 / `HideObject` 0x39e8b8):
  - `SelectPeak` container holding `SP_PeakList` (a UIMenu, hash 0x0BD0F864) with 4 children in list
    order `Peak3`, `Peak2`, `Peak1`, `Mountain` (Peak1 = hash 0x0056B7E1). Text rows at y = 0, 25, 50, 75.
    `SP_PeakList` has height 120 (verified in the fork's patcher). Also `SP_Info`, `SP_GameInfo`,
    `SE_Info`, `Indicators`, `peak{1,2,3}outline`, `phl` (highlight), `PEAKno`, `SubTitle`, `title_title`,
    `MapPic` (the picture).
  - `SelectPeakGoal` container: `SPG_Peak{1,2,3}Goals` menus with rows Race / Freestyle / Freeride
    (`Peak%dRaceGoal` etc.).
  - `SelectEvent` container: nine menus `Peak{1,2,3}{Race,Freestyle,Freeride}Locations` whose rows are
    `Peak%dRaceLoc%d` (4 rows), `Peak%dFreestyleLoc%d` (5), `Peak%dFreerideLoc%d` (8).
  - `ConfirmMenu_{Yes,No}*` popup.
- LUI screen data (verified from `tools/ssx3_bonus_menu.py` in the fork): `IYF\0` file, screen table
  (hash, offset) with each screen RefPack-compressed. A decoded screen is a header (20 bytes), N widget
  records (u16 class, u16 size at +2, u32 name hash at +4, text locale hash at +36 for text rows; a menu
  lists its child hashes after +36 with the count at +32), then a property table (16-byte header,
  count u16 at +14) of layout records with the same name hash at +4 and y (s16) at +18, width/height at
  +34/+38. `UIScreen_CreateObjects` (0x39ced0, `cUIScreen_createAllObjects`) walks the widget records
  in file order and creates each with `UIScreen_CreateObject` (0x39d078); menu children are attached by
  hash from the objects already created, so a child must precede its menu.

### Who sets what

- Row ids: `EventSelect_SetupPeakTexts` (0x200de8, `cUITemplate_MAP_setupMenus`) calls `setIndex(id)`
  (the widget's `+0x18`) by name: `Peak1`=0, `Peak2`=1, `Peak3`=2, `Mountain`=3; goal rows 0/1/2;
  event rows `j`. It also writes the `available`/`completed` arrays and the row texts from the three
  event tables (and `UIText_SetAsciiString` 0x3a0c50 / unicode string for rows that carry a name).
  The list position and the id differ: list order is Peak3, Peak2, Peak1, Mountain
  while ids are 2, 1, 0, 3; `EventSelect_OnWidgetCreated` (0x2009d0) selects child index 2 (Peak1) with
  `UIMenu_SetSelection` (0x39b960; its argument is a child index, not an id).
- Input reaches `EventSelect_OnWidgetAction` (0x2018a8; `(this, widget, action)`): 1 highlight, 2 unlocked
  colour, 5 accept, 6 back, 7 toggle map tab. Accept dispatches on the mode: peaks (`es_accept_peak`:
  needs `id < 3` and `this[1+id] != 0`; stores `gp-0xb4c = id`, `SetMode(1)`), goals (`es_accept_tab`:
  reads `this[0x10 + 4*id]`, stores tab, `SetMode(2)`), events (`es_accept_event`: reads
  `this[0x20 + peak*0x60 + tab*0x20 + 4*id]`, front-end flows start the event at once, others raise the
  confirm popup first).
- Starting: `es_start_event` calls `EventSelect_GetEventRecord` (0x200388, `func_00200388(peak, row, tab)`)
  which copies a 0x6c-byte record `{track, kind, event, ...}` from the fixed tables
  `0x4781d0` (race, `[3][4]`, peak stride 0x1b0), `0x4786e0` (freestyle, `[3][5]`, stride 0x21c),
  `0x478d38` (freeride, `[3][8]`, stride 0x360). It writes `track` to `0x535bc8+0x40`, then
  `BENewRaceInt_SetEventType/SetEvent`, `Commit` (0x144928), sets `gp-0x1f38`, and for flow 1
  `GameModule_QuitToLoadScreen`; the race then loads through `GameModule_LoadTrack` (0x22f6b0), which
  takes the world name from the course record (`0x43d950 + track*100 + 0x34`).
  `event_boot::start` (`crates/ssx3-game/src/event_boot.rs`) already repeats this from outside, and
  `event_boot::set_world` swaps the location name at the world load (done, plugin events boot).
- Panels: `EventSelect_SetMode` (0x2030a0) hides 14 panel widgets (peak list, 3 goal lists, 9 event lists,
  popup) and shows the one for the mode, picking by the hard-coded `gp-0xb4c` and `gp-0xb50`
  (`es_setmode_select_event`: peak 0/1/2 by branches). `EventSelect_UpdateModePanels` (0x202ad8) sets the
  title/sub-title text and visibility of the heading widgets. `EventSelect_RefreshPeakList` (0x203d40)
  reads the selected row's object through the list (`menu+0xa0` -> object, `+0x18` id): `id < 3` is a
  peak's progress, anything else takes the Mountain (totals) path.
  `EventSelect_RefreshPeakGoals` (0x205098) and `EventSelect_RefreshEventList` (0x207430) fill the info
  panels from the tables.
- Picture: `EventSelect_LoadMapImage` (0x203990): mode 0 loads the mountain (`cFEAsyncManager_Load3PeakPic`
  0x1a36e8, name `|map_mtn.ssh`), modes 1 and 2 `Load1PeakPic` (0x1a3768, `D_00441258[peak]`, 3 names)
  into the async slot `this+0x2d8`; the request is a name in the slot (`manager + slot*0x11c`, 0x108
  bytes), and the `|` prefix means "a member of `DATA/UI/MAPGFX.BIG`". `EventSelect_Update` (0x202828)
  binds the finished slot to `MapPic` and fades it.

### What limits it to 3 peaks and fixed lists

1. LUI has exactly the widgets above; nothing creates rows at run time.
2. `this+0x20` and `+0x140` are `[3][3][8]`; peak 3 would write into the next array (the completed flags)
   and peak 4+ into `+0x260` (the record copy), `+0x2cc` (popup) and beyond (verified from offsets).
3. `es_accept_peak` rejects `id >= 3`; SetMode/UpdateModePanels/RefreshPeakList choose widgets by
   `peak` with branches or fixed names.
4. The three event tables and `GetEventRecord` index `peak` in 0..2; the 17-entry `TrackInfo` array
   (`0x43d950`, 100 bytes each) is the only course list (the engine never indexes it by plugin ids).
5. Picture names come from a 3-entry table; texts from the locale table `CMNAMER.LOC` (hash to string).

## 2. What the fork changes (summary, by function)

The fork keeps the guest's three-peak data untouched and adds a fourth row plus separate panels:

| Fork piece | What it does |
| --- | --- |
| `tools/ssx3_bonus_menu.py` | Patches `DATA/UI/FE.LUI` and `OV.LUI` (RefPack decode, edit, literal re-encode, fix header offsets, keep 16-byte alignment) and `DATA/LOCALE/CMNAMER.LOC` (adds sorted hash entries and UTF-16 strings). Clones the `Peak1` record as `Peak4` (text `kT_BonusPeakName`), inserts it after Peak1 in `SP_PeakList` (child count 4 to 5, height 120 to 145, y 75; Mountain y 100), clones `SPG_Peak1Goals` as `SPG_BonusGoals` (rows `BonusRace`, `BonusFreestyle`) and `Peak1RaceLocations`/`Peak1FreestyleLocations` as `BonusRaceLocations`/`BonusFreestyleLocations` (rows `Tricky{Race,Freestyle}N`), adding each to the `SelectPeakGoal`/`SelectEvent` container. Output goes to the overlay, never to git. |
| `tools/tricky_ui_assets.py` | Builds picture members `tricky_*.ssh` into a copy of `MAPGFX.BIG` (the retail 16-byte shape envelope of `map_mtn.ssh` with the 256x256 indexed texels and CLUT replaced). |
| `custom_peak_widgets.rs` (`EventSelect_OnWidgetCreated`) | On creation, writes the widget id (`+0x18`): `Peak4`=3, `BonusRace`=0, `BonusFreestyle`=1, `Tricky*N`=N; then runs the stock body. |
| `custom_tracks::widget_row_id` + `uitemplatemap.rs` `event_select_on_widget_action` | Re-applies those ids on every action (the UIMenu can reassign child ids). |
| `uitemplatemap.rs` `es_highlight`/`es_unlocked` | For the Peak4 row and the bonus lists: always-unlocked colours (`SetHighlightColor`/`SetUnlockedColor`), preview image of the highlighted row, no per-peak tables. |
| `es_accept_peak/_tab/_event` | `Peak4` accept sets a host flag `BONUS_SELECTED`, plays sound 6, `PEAK=0`, `SetMode(1)`. Goal rows: `tab>1` denied, else `SetMode(2)`. Event rows: look up the host course by `(tab, row)`, `activate_in_game`, `prepare_track_info`, then **build the 3-word record from the template event** (`event_boot::find(events, track.template_event)`) instead of `GetEventRecord`, and go on with `es_start_event`. A one-frame guard (`accept_menu_step`) stops one press accepting two stacked menus. |
| `bonus_menu_mode.rs` (`EventSelect_SetMode`, plus calls from `es_refresh_mode`/`es_show_mode`) | After the stock SetMode, hides the original panel of the selected peak and shows `SPG_BonusGoals`/`BonusRaceLocations`/`BonusFreestyleLocations` (visibility via the object's vtable, not the UI state), sets row texts, title `kT_BonusPeakName`, subtitle (`PEAKno`), hides `SP_Info`, `SP_GameInfo`, `SE_Info`, `Indicators`, outlines while previewing, places the highlight `phl` under `Peak4`. |
| `custom_menu_panels.rs` (`UpdateModePanels`), `custom_event_menu_update.rs` (`EventSelect_Update`), `custom_peak_refresh.rs` (`RefreshPeakList`), `custom_menu_selection.rs` (`UIMenu_SetSelection`), `custom_ui_objects.rs`/`custom_ui_screen.rs` (`UIScreen_CreateObject(s)`) | Re-ports of the stock functions (same behaviour, documented as keyed by `UIObject.id` / child index rather than list position); the observable need is that a longer, reordered menu still works. (Whether any of these is truly required beyond the patched LUI is unverified: the diff against the stock bodies was not read in full.) |
| `custom_text_id.rs`, `custom_text_id_string.rs` (`UIText_SetTextId`, `SetWideStringInternal`) | Native text storage for long strings (course names). Reason unverified. |
| `uitext.rs` `cui_text_scroll_set_text_id` | Remaps the two help-text ids while a bonus course is highlighted. |
| `ovstates5.rs` `EventSelect_LoadMapImage`, `festateload.rs` `cFEStateEventSelect_Update`, `bonus_menu_mode::load_catalog_preview` | Writes the filename `|tricky_<x>.ssh` into the async slot request and latches `+0x2d4` to a variant id, so the preview loads from the added MAPGFX members. |
| `custom_async_data.rs` (`cFEAsyncManager_LoadDataFile`), `native_files.rs` | File layer serves the patched files from an overlay directory. |

The fork's weaknesses that the generic design avoids: the layout and text are hard-wired to one peak and
19 courses in a Python tool and in `if bonus_peak_selected()` branches inside core functions; the LOC
patch only covers the English table; the picture is built by a disc-specific tool; ids are fixed.

## 3. Generic design

### 3.1 Principles

- No plugin installed (or none with peaks): no LUI patch, no extra branch taken, same cycles: the race
  replay gate stays identical.
- The catalog is the only input. Everything dynamic is derived from it at boot into a `MenuPlan`.
- The guest's fixed tables are never indexed by plugin ids (as `docs/plans/plugins.md` already says);
  the host keeps `selected_plugin_peak: Option<usize>` and maps (peak, tab, row) to a catalog `Event`.
- Core code owns the menu (in `ssx3-game`, a new `game/fe/pluginmenu.rs`); plugins give data only. Code
  plugins keep raw hooks as an escape hatch.

### 3.2 `MenuPlan` (host data, `ssx3-game`)

Built once from `plugin_host::catalog()`:

```
struct MenuPlan { peaks: Vec<PeakPlan> }
struct PeakPlan { index: usize, id: String, name: String, picture: Option<PathBuf>,
                  race: Vec<EventPlan>, freestyle: Vec<EventPlan> }
struct EventPlan { id: String, label: Label, template: String, world: String }
enum Label { Key(u32 /* hash of name_key */), Ascii(String) }
```

- Order: catalog order (plugin id order, then manifest order). Events whose `kind` is neither race nor
  freestyle are skipped (a warning); an empty peak (no events) is not listed.
- Row widget names are index based and ASCII: `PlugPeak{p}`, `PlugGoals{p}` (rows `PlugGoal{p}Race`,
  `PlugGoal{p}Free`), `Plug{p}RaceLocs`/`Plug{p}FreeLocs` (rows `Plug{p}RaceLoc{j}`, `Plug{p}FreeLoc{j}`),
  hashed with `hashvalue::str_hash`. A name-to-(peak, tab, row) map is built from them.
- Widget ids: plugin peak rows get `4 + p` (the stock code treats `id >= 3` as the Mountain path, which
  `RefreshPeakList` accepts without touching tables; `es_accept_peak` rejects them, so plugin rows are
  handled before it). Goal rows 0/1, event rows `j`. Ids are written in `EventSelect_OnWidgetCreated`.
- Limits: the screen is 640x480 and the peak list sits next to the picture. Constants, enforced at
  catalog load with a log line, never a crash: at most 4 plugin peaks on screen (the list grows from 120
  by 25 per row; Mountain stays last); more peaks are dropped (log) until scrolling is checked (see
  risks: `UIMenu` has visible-count and first-visible fields, `+0x97/+0x98`, and `SetSelection` scrolls,
  so a taller list with a fixed visible count probably works); at most 12 events per (peak, kind) (rows
  22 high); names are cut to 31 ASCII characters. Peaks are listed in plugin-id order, so several plugins
  coexist as separate rows (a community map pack and Tricky together simply add rows). A plugin with more
  than one peak is fine; one peak per plugin is the expected case for map packs.

### 3.3 Patching FE.LUI without shipping game data

Options considered:

1. Run-time patch of the decoded screen in guest memory (hook `UIScreen_CreateObjects` 0x39ced0, build
   an extended record block, point `this+0x38` at it). Rejected for now: the layout (property) table is
   consumed through a second pointer I did not trace; two guest blocks must change together.
2. **Chosen: a host-side function that produces the patched file in memory when the file layer opens it.**
   `native_files` gets a third kind of file source next to overlay directories and the disc: a
   registered generator `fn(path, original: &dyn Fn() -> Vec<u8>) -> Option<Vec<u8>>` and a new
   `Loc::Mem(Arc<[u8]>)` open-file variant (`State::open` already builds `OpenFile {size, loc}`, so a
   memory file is a small addition). The plugin host registers generators for `DATA/UI/FE.LUI` (and
   later `DATA/UI/MAPGFX.BIG`) only when `MenuPlan` is non-empty. The bytes are computed once from the
   user's own SSX 3 disc (read through the disc reader), kept in RAM, and never written to disk, so no
   derived game data lands anywhere and the "no game data in git" CI check is untouched. (If boot time
   matters, cache the result in `paths.cache_root` keyed by hash of base file + plan; optional.)

`crates/ssx3-game/src/game/fe/lui_patch.rs` (new, pure, unit-testable with a synthetic screen):
RefPack decode and a literal-only encode (as `encode_refpack_literals` in the fork), `Screen::parse`,
`add_peak_rows(&mut Screen, &MenuPlan)` doing what `ssx3_bonus_menu.py` does but for N peaks and M rows:
clone the `Peak1` widget and layout records per plugin peak, insert after `Peak1`, shift `Mountain` down
by 25 per added row and grow `SP_PeakList` height and child list; clone the goals menu once per plugin
peak and the two location menus per peak and kind (rows cloned from `Peak1RaceLoc0`/`Peak1FreestyleLoc0`
with names above), each placed before its menu, and append the menu hashes to `SelectPeakGoal` and
`SelectEvent` (class 0x10 group, child count at +32); rebuild the property table; fix the file header
(`object_table` at +16, `font_table` at +20, later screens' offsets) and pad to 16 bytes (the fork found
word loads need the original alignment). The text hash field of each cloned row is left as the template's
and overwritten at run time (3.5). Text rows that carry a template's locale hash show wrong text for one
frame at most; the creation hook replaces it before the first draw (unverified).

The whole patch is checked in a unit test against synthetic input and, with the disc present, a
"parse, patch with zero rows, re-encode, decode equals original screen" round trip (skipped without a
disc, like other disc-dependent tests).

`OV.LUI` (pause-menu map) is not patched: plugin rows exist only in the front end (flow 0/1); every
branch below is gated by `FLOW < 2` and by the plugin widgets being present (find returns 0 otherwise).

### 3.4 Hooks (chained on existing slots; all these functions are already ported, so `Hooks::chain`
(S2) applies; in-tree they are installed by `pluginmenu::install()` after the built-in overrides, only
when `MenuPlan` is non-empty)

| Address | Function | Plugin behaviour |
| --- | --- | --- |
| 0x2009d0 | `EventSelect_OnWidgetCreated` | Before the stock body: if the widget's name hash is in the plan's map, write its id at `+0x18` and set its text (3.5). Child index selection stays native. |
| 0x2018a8 | `EventSelect_OnWidgetAction` | For a plugin row and flow < 2, handle the action in `pluginmenu` and return without running the stock body: 1 and 2 colour as unlocked (call `SetHighlightColor` 0x2083d8 / `SetUnlockedColor` 0x208438), set the picture preview; 5 accept: sound, then peak row sets `selected_plugin_peak = Some(p)`, `gp-0xb4c = 0`, `SetMode(1)` + panel refresh (`es_pick_peak`'s sequence); goal row sets the tab (0 race, 1 freestyle; a kind with no events is denied), `SetMode(2)`; event row starts the event (3.6). Action 6 (back) and 7 stay stock (they only change modes). A one-frame accept guard, as the fork has, because newly enabled menus come later in the input traversal. Stock code stays reachable for the three retail peaks and the Mountain. |
| 0x2030a0 | `EventSelect_SetMode` | Mode 0: `selected_plugin_peak = None`. After the stock body: hide every plugin panel, and when a plugin peak is selected hide the stock panel the stock code chose for `peak 0` (`SPG_Peak1Goals` or `Peak1{Race,Freestyle}Locations`) and show the plugin's goals (mode 1) or location list for the tab (mode 2). Visibility through `UIState_ShowObject/HideObject` by name (the fork used the object vtable; both exist, the UI state one is what the stock code uses (verified)). Preserve the caller's registers (the fork's `refresh_catalog_panels` saves and restores `c.g`/`c.f`). |
| 0x202ad8 | `EventSelect_UpdateModePanels` | After the stock body, when a plugin peak is selected: set `PEAKno`/`SubTitle`/`title_title` to the peak name (mode 0/1) and "Race"/"Freestyle" (mode 2), hide `SP_Info`, `SP_GameInfo`, `SE_Info`, `Indicators`, `peak{1,2,3}outline` (names verified from the fork). |
| 0x203d40 | `EventSelect_RefreshPeakList` | When the highlighted row is a plugin peak (menu `+0xa0` object, `+0x18` id >= 4): the stock body already takes the Mountain (totals) path for `id >= 3`; after it, hide the info panels above and place `phl` under the row. The totals shown by the stock path are the game's, so they are hidden, not edited. |
| 0x203990 / 0x202828 | `EventSelect_LoadMapImage` / `EventSelect_Update` | When a plugin peak or event is highlighted, request the plugin picture into the slot instead (3.7), set the variant latch `+0x2d4` to `0x100 + p`, clear `+0x2dc`; stock `Update` then binds the finished slot (fork says Update is independent of the peak index, to be confirmed). |
| 0x1a3cd8 | not hooked | The slot name is written by `LoadMapImage`; the loader is unchanged. |

Changes inside `uitemplatemap.rs` itself are not needed: everything above is reachable from entry
points. If a case needs an internal step (`es_start_event`, `es_pick_peak`), `pluginmenu` re-implements
the few lines (they are short and already documented) or the function is made `pub(crate)`.

### 3.5 Text and locale

- Peak name: the catalog's `name`, set as ASCII with `UIText_SetAsciiString` (0x3a0c50) then
  `UIText_RefreshText` (0x3a0528) on the row widget (the stock `SetupPeakTexts` does the same for rows
  that carry a name). Language independent, no `CMNAMER.LOC` patch (the fork's LOC patch would need one
  per language file).
- Event name: if the manifest gives `name_key`, set `UIText_SetTextId(hash(name_key))` (a key of the
  game's own locale table, so a plugin can use stock strings); else `name` as ASCII; else the id.
- Headings use the peak name. Help text for the highlighted event: none (the info panels are hidden).
- Risks: ASCII text is dropped by a locale reload; the screen is rebuilt on each entry so it is set again
  by the creation hook. UIText stores the wide string in a heap buffer with a capacity field (+0xb8); the
  fork re-ported `SetWideStringInternal` for long strings (reason unverified): names are capped at 31
  characters to stay below any suspected limit.

### 3.6 Starting an event (reusing `event_boot`)

On accept of plugin event `e` (catalog `Event`, tab matches `kind`):

1. `template = event_boot::find(&event_boot::events(c), &e.template)`; if missing, deny with sound 4.
2. Write the 3-word record `{track, kind, event}` where the stock code would have called
   `GetEventRecord` (the fork does the same at `sp`) and run the stock `es_start_event` tail. Because the
   hook replaces the action handler, the tail is re-implemented in `pluginmenu::start_event` using the
   same calls as `event_boot::start` after "Event select confirm": `gp-0x1f38 = 1`, `0x535c08 = track`
   (`TRACK_INDEX`), `BENewRaceInt_SetEventType/SetEvent` (0x144df0 / 0x1451e8), `Commit` (0x144928), then
   `GameModule_RequestState` as `es_event_confirm`'s front-end branch does, not the boot's direct
   pre-game load screen (the menu flow also pushes the loading-screen stack entries; reuse by calling
   the stock confirm path with `this+0x260` set to the record is the simplest, see risks).
3. `event_boot::set_world(Some(e.world))`. Every native event start must clear it: hook the same entry
   (`OnWidgetAction` with a retail event row) and `EventSelect_Shutdown` leaving by back, or clear it in
   `SetMode(0)`. `set_world` already restores the original location name at the next world load.
4. Loading screens and results show the template's course name. A later step hooks
   `BENewRaceInt_GetTrackName` (0x144bc0 area, ported in `beintnewrace.rs`) to return the plugin event's
   label while it is active (the fork does this with a copied TrackInfo record); until then the template's
   name shows.
5. Records/leaderboards: the template event's records would be written; the fork returns "no table" from
   `BEScore_GetHighScoreTable` for custom events. Do the same (plugin-owned saves are a separate item of
   the plugin plan).

### 3.7 Pictures

- Source: the manifest's `picture` (PNG in `content/`), or absent: a generic card. Host conversion in
  Rust to the front end's native shape: take the retail member `map_mtn.ssh` from `MAPGFX.BIG` as the
  16-byte envelope template, replace the 256x256 indexed texels (65,536 bytes) and the 1,024-byte CLUT
  (the fork's `native_shps`, with PSMT8 palette swizzle: swap bits 3 and 4 of the index; format details
  unverified), after resizing and quantizing the PNG (median cut to 256 colours, one transparent index).
- Serving: a generated `DATA/UI/MAPGFX.BIG` (the same generator mechanism as FE.LUI, BIGF with extra
  members `plug_{p}.ssh`). The hook writes `|plug_{p}.ssh` into the async slot (`manager + slot*0x11c`,
  after `UnloadFEAsyncFile` 0x1a38c0 and a `Load1PeakPic`-style status set, as the fork's
  `load_catalog_preview`), so the stock async loader reads it.
- Later: once the GS is gone and the renderer owns textures, replace this with a direct texture override
  for the picture widget; the plan (`picture` in the catalog) does not change.

### 3.8 Community maps: what the design must tolerate

- No disc input: a plugin without `[inputs]`/`[prepare]` ships `content/DATA/WORLDS/<world>.big` (and
  optionally audio and `ui/` pictures) and lists `[[peak]]`/`[[event]]`. The overlay layer (S1) serves it;
  the menu needs nothing else. The generated FE.LUI/MAPGFX come from the user's SSX 3 disc only (the base
  game is always required), never from the plugin's source disc.
- Many plugins, many peaks: ids are `<plugin>/<id>`; the plan namespaces widget names by index, so
  clashes are impossible. Two plugins shipping the same `world` name are a conflict (the overlay would
  shadow one): validation in `catalog` should reject a second `world` of an already used name (new check,
  not yet in `manifest.rs`; the world name space is global as `data/worlds/<world>.big`).
- World content constraints a community author hits (from `docs/plans/plugins.md`, "What a world needs"):
  at most 1,500 textures without the `world-textures` setting; section names equal to the template event's
  or the per-event section mapping (S4/S6 work); the 15-character world name rule is already validated.
  The catalog's `template` choice decides rules, rider count, scoring and the flow, so a community map
  picks the closest stock event; documentation for authors should list stock events (`SSX3_EVENT=list`).
- Screen limits (3.2): 4 peaks, 12 rows per list, 31-character names; surplus is dropped with a log line
  naming the plugin. A plugin with only a map and no picture gets the generic card.
- Unlock state and progress: plugin peaks are always unlocked; medals and best times belong to a plugin
  save file (separate work); nothing is written to the memory card format.
- Safety: all strings and pictures come from untrusted plugin data. Names are copied through the ASCII
  setter after truncation (no format strings); pictures decode with bounded sizes (reject over 4,096 px
  or 16 MB); a bad picture falls back to the generic card.
- Rider/character gating, multiplayer events and the freeride tab are out of scope (race and freestyle
  only; the manifest `Kind` already has just those).

## 4. Implementation plan (small, verifiable)

Each step ends with the gate (`GATE: race replay RAM identical to the baseline`) with no plugin
installed, and a unit test where stated. One heavy process at a time.

1. **`MenuPlan`** (`crates/ssx3-game/src/game/fe/pluginmenu.rs`): build from a `Catalog`, caps, names,
   widget-name map. Pure unit tests with `Catalog` literals (ordering, caps, dropping empty peaks, ASCII
   truncation, two plugins coexisting). No behaviour change.
2. **Generated-file source in `native_files`**: `Loc::Mem`, `register_generator(path, f)`; test with a
   temp-dir fake disc / the existing `state()` helper in `native_files.rs` tests: a generated file shadows
   the disc, others fall back; `SSX3_FILE_LOG` shows it. Gate unchanged (nothing registered).
3. **`lui_patch.rs`**: RefPack decode/encode (round trip test, literal-only encode decodes back),
   `Screen::parse`, `add_peak_rows`. Tests on a synthetic screen built to the layout above, then (disc
   present) on the real screen 0x5380: zero-plan round trip equals the original decoded screen; one
   plugin peak with 2 events parses and has the expected record counts (377 + rows + containers).
4. **Boot wiring**: when the plan is non-empty register the FE.LUI generator. Headless check with a test
   plugin (`examples/plugins/hello` already has a peak and event; add one with 5 events and a second
   peak): boot to the Select Peak screen with a scripted pad, dump the frame: the extra rows are visible
   (text wrong or blank at this step is expected). Verify the game does not halt and the three retail
   rows still work (stock input script `tools/scenarios.py` menu replay reaches the same race: gate with
   a plugin installed but the replay choosing a retail peak gives identical RAM except the menu's own
   objects; compare frames of the race, not RAM, for this run).
5. **Creation hook** (ids, ASCII/key text): frame at the peak list shows the names. Unit test the
   name-to-id mapping; run test through `difftest` is not possible (no generated counterpart), so the
   check is a headless frame and a RAM read of the widget `+0x18` values.
6. **SetMode/UpdateModePanels hooks and plugin accept path**: scripted input (Down to the plugin row,
   Cross, Cross on Race, Down, Cross on an event) with `SSX3_PAD_SCRIPT`-style input as the existing
   replay uses (`replay_script.rs`); assert via a debug log line `[plugin-menu] start <event id>`.
   With the `world-copy` plugin (a renamed copy of BAM, S0): the second load must be `BAX.big` and the
   race frames at fixed counts must equal the `SSX3_EVENT=world-copy/copy` run (already verified
   identical to the stock run). This is the end-to-end check of "menu start == boot start".
7. **Retail unaffected with plugins loaded**: replay the stock menu route to Snow Jam with the plugin
   installed; `set_world(None)` was applied: the world file log shows `BAM`; RAM identical to the
   baseline after the race loads (expected equal; if the extra widgets change allocation addresses, compare
   frames instead and record that in the plan).
8. **Pictures**: PNG decode (use an existing small dependency if the workspace has one; otherwise a
   minimal decoder for 8-bit PNG limited to the plugin's own art), quantize, build SSH, generate
   `MAPGFX.BIG`, slot request hook. Unit tests: quantizer determinism, SSH size equals template size, BIGF
   directory round trip. Headless: frame of the highlighted plugin row shows the picture.
9. **Track name and records**: `GetTrackName` / no-table hooks; loading screen shows the plugin event
   name. Frame check at the loading screen.
10. **Limits and polish**: peak scrolling beyond 4, tab denial for empty kinds, conflict validation of
    world names in `manifest`/`catalog`, docs: `docs/plugins.md` ("Peaks and events" for authors,
    including the community-map recipe: folder with `plugin.toml`, `content/DATA/WORLDS/<world>.big`,
    `ui/peak.png`).
11. **Tricky** (S6) drops its menu hooks and ships only `plugin.toml` + importer.
12. Last: browser smoke test (lowest priority platform); the generator and the PNG decode are pure Rust.

## 5. Risks and unknowns

- **UIMenu scrolling and the visible count** for more than about 5 rows: the fields exist
  (`+0x95` selected, `+0x97` visible count, `+0x98` first visible, `+0xa4` pending selection), the source
  of the visible count (LUI or height/row pitch) is not known. The fork grew the height. Needs a run.
- **Second LUI table** (property/layout records) and the `UIScreen_CreateObject` class table: the fork's
  patcher shows what to clone, but class ids and any per-class extra bytes were not re-derived here;
  cloning whole records of the same class avoids needing them. A change in row count could also move
  alignment-sensitive data (the fork pads to 16).
- **Whether the fork's re-ports** (`UIScreen_CreateObject(s)`, `UIMenu_SetSelection`, `UIText_*`,
  `UpdateModePanels`, `RefreshPeakList`, `Update`) are needed with a longer menu, or only documentation
  and bug fixes. I read their headers, not the diffs against the stock bodies. If a stock function does
  index by list position (for example selection by child index when the list has an extra row), the
  generic design needs the same fix in-tree; step 4/6 would show it as wrong rows highlighted.
- **Starting through the stock tail**: `es_event_confirm`/`es_start_event` use `this+0x260`, the
  loading-screen stack (`gp-0x69c`, `0x5366e8`) and `GameModule_RequestState`; re-implementing them in
  `pluginmenu` may drift from the guest. Safer: let the stock `es_accept_event` run with a record written
  where `GetEventRecord` returns it (hook `GetEventRecord` 0x200388, generated, so it needs `hookable.txt`
  or a port first: it is a simple table copy and a good S4-style port with a differential test, which
  would also let the template event stand in for any `(peak, tab, row)`).
- **Gate with a plugin installed**: widget allocations change guest heap addresses, so RAM identity is
  only promised with no plugin. Frame-level equality is the check with plugins.
- **Picture format** (the CLUT swizzle and the retail member's envelope) is taken from the fork's Python
  and not checked against the disc; a wrong swizzle shows as odd colours only.
- **Template events** carry rules (rider count, medals, scripts, sections). Community worlds with another
  section layout need the S4/S6 section-table work before their events run; the menu itself does not
  depend on it.
- **ASCII text persistence** across language change and the 800-character wide-string limit (the fork's
  re-port suggests a hazard): names are short; verify by switching language in a headless run.
- **Two plugins and one `world`/picture name**: validated at catalog build (to add); the MAPGFX member
  names use the peak index, so they cannot clash.
- Ghidra was not reachable, so addresses come from `generated/ee-symbols.tsv`, the Rust ports, the C++
  reference (`devvydont-ssx3` `uitemplatemap.cpp`, `ovstaterewardslist.cpp`, `feasyncfile.cpp`) and the
  fork; anything that only Ghidra could confirm is marked above.
