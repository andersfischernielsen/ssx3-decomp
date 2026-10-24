# Options

Most options can be changed in the game with the settings overlay (F10): display rate,
field of view, aspect ratio, resolution, anisotropic filtering and fullscreen apply at once; renderer, sound,
pads and memory card, game files and intro movies after a restart. They are saved in
`saves/ssx3-settings.cfg` as `SSX3_*` lines; an environment variable of the same name
overrides the file.

All options are environment variables. The defaults are the verified, exact
configuration. The plain command above is the recommended way to play.

**Everyday:**

| Variable | Effect |
| --- | --- |
| `SSX3_SKIP_MOVIES=1` | Skip the EA, THX and intro movies at boot. |
| `SSX3_ISO=<path>` | Disc image location (default `./SSX 3 (USA).iso`). |
| `SSX3_MEMCARD=<path>` | Memory card file (default `saves/ssx3-mc0.ps2`). |
| `SSX3_PACK=<path>` | Play from a data pack made by `tools/bundle.py pack` instead of the disc image. |
| `SSX3_PAD2=1` | Plug in pad 2; a second controller drives it (Multi Play). |
| `SSX3_AUDIO=0` | No audio output. |
| `SSX3_REAL_CLOCK=1` | Give the PS2 the host's date and time (the default is a fixed time, so runs are reproducible). |

**Straight into an event** (debugging, testing, benchmarks):

| Variable | Effect |
| --- | --- |
| `SSX3_EVENT=<code>[:<event>]` | Boot straight into an event: the setup the menus would do is done through the game database, and the title, the intro and the race card are pressed through, so the race runs at about frame 1,050 instead of ~4,600. Codes: `ARA1` Snow Jam, `BRA2` Metro City, `CRA3` Ruthless Ridge, `DRA4` Intimidator, `ERA5` Gravitude; `ASS1`/`DSS2`/`ESS3` slopestyle, `ABA1`/`CBA2`/`EBA3` big air, `BHP1`/`CHP2`/`EHP3` superpipe; `ABC1`/`DBC2`/`EBC3` backcountry (add `:<event>`, e.g. `ABC1:6`). `SSX3_EVENT=list` prints them all when the main menu comes up. |
| `SSX3_RIDER=<name\|index>` | With `SSX3_EVENT`: the player's rider: `moby`, `kaori`, `allegra`, `mac`, `zoe`, `griff`, `elise`, `nate`, `psymon`, `viggo` (or the index 0..9). |

**Rendering.** By default the game's frames are drawn by the native renderer
(`crates/ssx3-gfx`, wgpu). The frame's DMA chains are not run through the emulated
VIF/VU1/GS at all, and the GS core is detached: menus, movies and loading screens come
through the native renderer too.

| Variable | Effect |
| --- | --- |
| `SSX3_RENDER=native` | The default, as above. |
| `SSX3_RENDER=emu` | The emulated GS only: the exact reference path (RAM-identical to the PS2 model). Needs a build with `--features vu1-microcode` (the VU1 programs and the emulated GS, feature `emu-gs`); the default build has no GS emulation. |
| (window size) | The native renderer draws at the window's pixel size and the game's 3D view is widened to the window's aspect (ultrawide included); the HUD keeps its 4:3 shape in the centre. |
| `SSX3_RENDER_SIZE=2560x1080` | A fixed output size instead (any size and aspect; headless too). |
| `SSX3_FOV=1.2` | Field of view relative to the game's (here 20% wider). |
| `SSX3_ANISO=16` | Anisotropic filtering, 1 (off) to 16 (default), for textures with mipmaps (texture-pack replacements). |
| `SSX3_CATCHUP=0` | Turn off catch-up (window; settings "Keep speed when slow", on by default). The window always runs the game on the wall clock at the game rate, whatever the display's refresh rate (a 60 Hz display at a 120 Hz game rate gets two game updates per image). When the host itself is too slow, catch-up queues extra updates per image (at most 4) and keeps the audio device fed from the wall clock; off, the game slows down instead. Replays and headless runs never use it. |
| `SSX3_AA`, `SSX3_MIPMAPS`, `SSX3_LOD_BIAS`, `SSX3_TERRAIN`, `SSX3_DRAW_DIST`, `SSX3_AO`, `SSX3_BLOOM`, `SSX3_SNOW` | Modern graphics options (F10, all off by default): see [`plans/modern-graphics.md`](plans/modern-graphics.md). `SSX3_GFX_SHOW=ao` shows the AO mask. |
| `SSX3_SHADOWS=1` | "Native shadows" (F10): the terrain lit by the renderer (real sun, sky fill, sun shadows from the whole loaded world, tree cut-outs) instead of the game's baked layer, whose colour tints stay; see [`plans/shadow-maps.md`](plans/shadow-maps.md) and [`plans/terrain-lighting.md`](plans/terrain-lighting.md). Tuning and debugging: `SSX3_SUN=az,el`, `SSX3_SHADOW_RANGE`, `SSX3_SHADOW_SIZE`, `SSX3_SHADOW_STRENGTH`, `SSX3_SHADOW_TINT`, `SSX3_LIGHT`, `SSX3_CASTERS_SHOWN`, `SSX3_SHADOW_SHOW=cascade\|factor\|ndl\|normals\|sunlit\|fit`, `SSX3_SHADOW_DEBUG=…` (switches listed in `crates/ssx3-gfx/src/shadow.rs`). |
| `SSX3_ASPECT=4:3\|16:9\|stretch` | The classic fixed-aspect image (512×448 scaled, letterboxed) instead of following the window. |
| `SSX3_RENDER_SCALE=2` (up to 8) | N× internal resolution (native renderer; with `SSX3_RENDER=emu SSX3_RENDER_NATIVE=all`, the emulated GS's). |
| `SSX3_RENDER=shadow` | Both: the native renderer draws next to the emulated GS, which is shown (headless writes `frameNNNNN-gfx.png` too). |

**Texture packs.** Textures can be dumped and replaced, e.g. with upscaled versions:

| Variable / folder | Effect |
| --- | --- |
| `SSX3_TEX_DUMP=1` | Write every texture the renderer decodes, once, as `textures/dump/<name>.png` (or `SSX3_TEX_DUMP=<dir>`). The name is a hash of what the texture decodes to, so it is the same in every run. |
| `textures/replace/<name>.png` | Used instead of that texture, at any size (keep the aspect ratio: 256×128 → 1024×512), with mipmaps generated at load. `SSX3_TEX_REPLACE=<dir>` reads another folder, `0` turns replacements off. |

`tools/upscale-textures.py` makes such a pack with AI upscaling (Real-ESRGAN with the
UltraSharp model, 4×; `--setup` downloads them into `textures/.tools`): it keeps tiling
textures seamless and upscales alpha separately. Replaced textures get mipmaps and
anisotropic filtering.

Alpha in the PNGs is the GS's doubled (255 = opaque). Render targets, the frame-feedback
passes and movie frames are not textures and are not dumped. `textures/` is ignored by git
(the dumps are game data).

**Native host implementations .** On by default; each has a switch back to the
emulated PS2 path for comparison:

| Variable | Effect |
| --- | --- |
| `SSX3_SND=iop` | The recompiled IOP sound driver instead of the native mixer (`ssx3-snd`). |
| `SSX3_NATIVE_FILES=0` | The emulated CD instead of reading game files through `std::fs` (from the ISO, or an extracted `DATA/` folder with `SSX3_DATA_DIR=<dir>`). |
| `SSX3_IOP_SIO2=1` | The recompiled IOP controller and memory-card modules (SIO2MAN, PADMAN, MCMAN, MCSERV) on a modelled card, instead of the native padman server and memory card (`ssx3-mc`). |
| `SSX3_VU1=emu` | The translated VU1 microcode instead of the native Rust programs (only used outside native rendering). |
| `SSX3_TICK_HZ=120` | Game rate (settings: "Game rate"): the game updates N times a second, one image per update (no interpolation needed). Every 1/60 step the game loads follows the rate; per-tick code the conversion pass has not reached yet may still run fast. See [`plans/configurable-tick-rate.md`](plans/configurable-tick-rate.md). |
| `SSX3_HOST_CLOCK=1` | Keep game time real-time when the host falls behind (extra 60 Hz updates, fewer rendered frames). Known issue: with a host that stays behind, the game can stall (all threads waiting); off by default. |
| `SSX3_GS=cpu` | With `SSX3_RENDER=emu`: software rasterizer instead of the GPU one (slow; for debugging). |
| `SSX3_RENDER_NATIVE=all` | With `SSX3_RENDER=emu`: the emulated GS draws with wgpu render pipelines instead of the compute GS. |

The fully emulated reference path is:

```sh
SSX3_RENDER=emu SSX3_SND=iop SSX3_NATIVE_FILES=0 target/game/ssx3
```

The native paths don't keep emulated timing byte-identical (native sound and the native
renderer's light queries shift how the game's threads interleave), so a scripted race
plays out a little differently than on the emulated path; each path is deterministic on its
own.

