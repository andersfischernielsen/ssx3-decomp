# Plugins

Plugins add content to SSX 3 without changing this repository: courses from
SSX Tricky, SSX On Tour or the original SSX, each as its own peak on the
mountain, and anything else that a plugin's code can do. The design and its
open questions are in [`docs/plans/plugins.md`](plans/plugins.md).

A plugin is a **folder**. With no plugin the game is unchanged.

```
plugins/tricky/
  plugin.toml      id, api version, inputs (disc serials), peaks, events
  content/         files the plugin ships (data-only plugins)
  prepare/         an importer that makes content from your disc image
  Cargo.toml, src/ optional: Rust code (hooks, new mechanics)
```

## Using plugins

Put plugin folders in `plugins/` next to the game data (or in `SSX3_PLUGIN_DIR`)
and your disc images next to the SSX 3 image (or in `SSX3_DISC_DIR`).

```sh
ssx3 plugins list              # what is found, and what each one still needs
ssx3 plugins prepare [id]      # make a plugin's content from your disc image
ssx3 plugins build             # a front end with the Rust code of the plugins that have it
```

- A plugin that makes its content from a disc (`[prepare]`) loads once
  `prepare` has run; the content goes in the cache folder, never into git.
- A plugin with Rust code (`[code]`) needs a build with that code in it:
  `ssx3 plugins build` writes `target/ssx3-custom/` and builds
  `ssx3-custom`, which is `ssx3` with those plugins linked in. A plugin whose
  code is not in the running build stays out, and says so in the log.
- `SSX3_PLUGINS=0` turns plugins off; `SSX3_PLUGINS=a,b` loads only those.
- The log lines start with `[plugins]`.

## Writing a plugin: `plugin.toml`

```toml
[plugin]
id = "tricky"             # lower-case letters, digits, '-'
api = "0.1"               # the engine's plugin API; before 1.0 it must match

[inputs]
disc = { serials = ["SLUS-20326"] }   # which disc image the content comes from

[prepare]                 # absent: the folder ships its content in content/
command = ["python3", "prepare/prepare.py"]

[code]                    # absent: data only
package = "ssx3-plugin-tricky"

[[peak]]
id = "tricky"
name = "Peak Tricky"
picture = "ui/peak.png"   # inside the content folder: relative, no '..', no ':'

[[event]]
id = "garibaldi-race"
peak = "tricky"
kind = "race"             # or "freestyle"
template = "ARA1"  # the native event (`SSX3_EVENT=list`) whose rules and flow it takes
world = "garibaldi"  # data/worlds/<world>.big; letters, digits, '_', at most 15       # the location name: data/worlds/garibaldi.big
```

Ids are local; the engine names them `<plugin>/<id>` (`tricky/garibaldi-race`).
`ssx3 plugins list` shows every problem a manifest has.

**Disc images.** To find the disc a plugin needs, the engine reads `.iso` and
raw single-track `.bin` images and matches the boot serial in `SYSTEM.CNF`.
Any other format (a multi-track `.cue`, `.chd`, `.nrg`, ...) is the plugin's to
read: name the program in `detect` and the file extensions in `extensions`:

```toml
disc = { serials = ["SLES-50030"], extensions = ["chd"], detect = ["python3", "prepare/detect.py"] }
```

`detect` runs in the plugin folder with an image's path appended, prints the
boot serial on its first line and exits 0, or exits non-zero for a file it does
not understand. The importer reads the image itself (`SSX3_PLUGIN_DISC`), in
whatever format, so the engine never needs to know the format.

**The importer** runs in the plugin folder with no shell and these variables:
`SSX3_PLUGIN_ID`, `SSX3_PLUGIN_DISC` (the disc image that matched `serials`),
`SSX3_BASE_DISC` (the SSX 3 image) and `SSX3_PLUGIN_OUT` (where to write). It
writes its files under `$SSX3_PLUGIN_OUT/overlay/` mirroring the disc's paths
(`overlay/DATA/WORLDS/garibaldi.big`), then creates `$SSX3_PLUGIN_OUT/ready`.
Derived data must never go anywhere else, and never into git.

**The overlay** is served before the disc's own files, case-insensitively, as
if it were part of the disc (`SSX3_FILE_LOG` shows which source served a read).

## Writing a plugin: Rust code

The package exports `plugin()`:

```rust
use ssx3_plugin::{Cpu, HookError, Hooks, Plugin, Prev};

static PREV: Prev = Prev::new();

struct Mine;

impl Plugin for Mine {
    fn id(&self) -> &str { "mine" }

    fn install(&self, hooks: &mut Hooks<'_>) -> Result<(), HookError> {
        hooks.chain(0x0017_6448, wrap, &PREV)
    }
}

fn wrap(c: &mut Cpu) {
    PREV.call(c);               // the function that was in the slot
    // ... change what it returned
}

pub fn plugin() -> Box<dyn Plugin> { Box::new(Mine) }
```

- `chain` wraps: the hook runs what was in the slot through its own `Prev`
  (one `static` per hook). Hooks of several plugins on one function run in load
  order. `replace` drops what was there and is an error when another plugin
  already hooked the function.
- Hooks are installed last, after every installer of the game itself (the file
  layer, the draw capture, the debug hooks), so a hook wraps what those left in
  the slot and cannot be replaced by them.
- Only functions with an override slot can be hooked: every ported function
  (`crates/ssx3-game/overrides.txt`) and the addresses of
  `crates/ssx3-game/hookable.txt`. Add the address you need to `hookable.txt`
  in your pull request; regenerate with `python3 tools/integrate_wave.py regen`.
- A hook is ordinary game code: `Cpu` registers, guest memory and cycle counts
  as in the ported functions (`docs/porting/writer-brief.md`).
- A plugin's own tests cover its behaviour; `examples/plugins/hello` is a
  complete small one.
- The plugin API is `0.x`: pin the commit of `ssx3-decomp` your plugin builds
  against.

## Rules

- No game data in a plugin repository: the importer reads the user's disc and
  writes to the cache. The same CI check as this repository.
- Code plugins run with the rights of the process, so the user builds them
  from source; nothing loads a binary from elsewhere.

## Playing a plugin event

`SSX3_EVENT=<plugin>/<event>` boots straight into a plugin's event: the game
sets up its `template` event and, at the world load, reads `data/worlds/<world>`
(from your overlay) in place of the template's own. The swap is undone at the
next world load. Verified: a copy of Snow Jam's world played as `ARA1` leaves
the race RAM identical except for the world's name.

From code, `ssx3_game::event_boot::set_world(Some(name))` makes the next world
load use `name`, `set_world(None)` goes back to the course's own; whoever starts
a native event after a plugin event must clear it.
