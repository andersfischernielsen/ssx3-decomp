# SSX Tricky courses, as a plugin for ssx3-decomp

Ride the SSX Tricky courses (Garibaldi, Snowdream, Elysium Alps, Mesablanca, Merqury City
Meltdown, Tokyo Megaplex, Aloha Ice Jam, Alaska, Untracked, Pipedream, Trick Tutorial; 19 events)
in SSX 3, with SSX 3's riders, tricks and physics.

This repository is a **plugin** of [ssx3-decomp](https://github.com/andersfischernielsen/ssx3-decomp)
(see its `docs/plugins.md`). ssx3-decomp stays "SSX 3"; Tricky is added through the plugin
system: a folder with a manifest (`plugin.toml`), an importer that builds the course data from
**your own disc**, and a Rust crate with the game code the courses need.

> SSX 3 and SSX Tricky are (c) Electronic Arts. This repository contains no game data and no game
> code. Everything game related is generated on your machine from your own discs, goes to the
> plugin cache folder, and must never be shared or committed.

Status: see [`PLAN_STATUS.md`](PLAN_STATUS.md) for what was verified and what is not.

## What you need

- ssx3-decomp (the `feat/plugin-events` branch or a later `main` with plugin events), built as its README says.
- Your own disc images, both USA: `SSX 3 (USA).iso` (SLUS-20772) and `SSX Tricky (USA).iso` (SLUS-20326).
- Python 3.9 or newer, and Rust.

## Install

```sh
git clone <ssx3-decomp> && cd ssx3-decomp          # and set it up as its README says
git clone <this repository> plugins/tricky          # the folder name does not matter, the location does:
                                                    # Cargo.toml reaches the engine as ../../crates/...
# put the two disc images next to each other (SSX3_DISC_DIR works too)
cargo run --profile game -p ssx3 -- plugins list     # shows what the plugin still needs
cargo run --profile game -p ssx3 -- plugins prepare tricky   # converts the courses (long, once)
cargo run --profile game -p ssx3 -- plugins build            # builds target/game/ssx3-custom
target/game/ssx3-custom                                      # play
```

`prepare` writes to the cache folder (`~/Library/Caches/SSX3/plugins/tricky` on macOS), never
into a git tree. `SSX3_TRICKY_COURSES=garibaldi,alaska` limits the import to some courses;
`SSX3_TRICKY_FORCE=1` rebuilds. `SSX3_PLUGINS=0` turns plugins off.

## Playing

Boot straight into an event (the engine's development aid):

```sh
SSX3_EVENT=tricky/garibaldi target/game/ssx3-custom
```

Event ids: `garibaldi`, `garibaldi-freestyle`, `snowdream`, `elysium-alps`, `mesablanca`,
`merqury-city`, `tokyo-megaplex`, `aloha-ice-jam`, `alaska`, `untracked`, `pipedream`,
`trick-tutorial` (each course with a `-freestyle` variant where it has one).

From the menus: Single Event, character, Continue, then **Peak Tricky** (the row after the game's
three peaks), Race or Freestyle, and the course. The rows come from the `[[peak]]` and `[[event]]`
tables of `plugin.toml`; the engine builds the screen, so this plugin has no menu code. The peak's
picture, `ui/tricky.png` (the Tricky logo), is made by the importer into the cache folder.
`tools/menu_run.py` drives the menus with scripted pad input (headless) to start an event.

Known limit: after quitting a Tricky event the game returns to the in-game version of Select Peak
(`OV.LUI`), which the engine does not patch: Peak Tricky is not listed there until you go back
through the main menu.

## Layout

| path | what |
| --- | --- |
| `plugin.toml` | id, disc serial, importer, peak (with picture), 19 events |
| `prepare/prepare.py` | importer entry point, run by `ssx3 plugins prepare` (also writes `ui/tricky.png`) |
| `tools/menu_run.py` | scripted menu run: start a Tricky event headless |
| `tools/` | the converters (terrain, textures, collision, splines, AI paths, scripts, sky, movies, UI) |
| `src/` | the plugin crate `ssx3-plugin-tricky`: hooks on game functions |
| `retained/` | audit ports the fork kept but never installed (not compiled) |
| `docs/investigation`, `docs/modding` | how each Tricky mechanic was found and converted |

## Credits

The port of SSX 3 to native Rust is Anders Fischer-Nielsen's ssx3-decomp. The Tricky course
conversion, the importers and the mechanics code are npiriou's work in
[ssx3-decomp-more](https://github.com/npiriou/ssx3-decomp-more); this repository is that work
repackaged as a plugin. Also: DevvyDont/ssx3, GlitcherOG/SSX-Library, swax/OpenSlope.

MIT licensed (see `LICENSE`): the code in this repository, not any EA game material.
