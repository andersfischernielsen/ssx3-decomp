# Real-time lighting: reported bugs

Bugs the owner reported on the "Real-time lighting" setting (F10,
`SSX3_SHADOWS`; branch `shadow-maps`), with their status. Look at one
course with `SSX3_EVENT=<code>` (Snow Jam `ARA1`, Metro City `BRA2`); the
debug views are in F10's debug group and `docs/options.md`.

| # | Reported (2026-10-25) | Status |
| --- | --- | --- |
| 1 | Not all lights on a course emit light: Metro City is at night with many lights shining onto the track, but they light nothing. | Partly: the game's real lights (`docs/investigation/world-lights.md`: point and spot records in the Nash effects lists, kind 6) now light the terrain and trails with the game's own formula (the 64 nearest, `SSX3_LAMPS=gain`). Metro City has only 39 loaded (coloured spot fills around the start, blue lamps further on); most of its coloured light pools are painted into the baked layer, not lights. Open: the lamp and neon halos (lens-flare entries, kind 8: 68 on Metro City) as point lights, or the bake's light pools kept as emissive light. |
| 2 | Strange rectangular shadows appear and disappear on the snow (Snow Jam, frame by frame). | Fixed: terrain strips of opposite winding cancelled each other's smooth normals (polygon-shaped light and shade flipping as the draws changed); each strip is turned up before its normals are summed. Left: thin moving shadows (cables) and the banner boards' long shadows, which are real. |
| 3 | Blocky shadows on the riders (Snow Jam start gate, the player rider's back). | Fixed: the riders' sun shadows used flat face normals, so the faces near the terminator shadowed themselves; they now use the smooth skinned normals. |
| 4 | Riders do not cast proper shadows: in the air, the drop shadow on the snow shows only the helmet and the board bindings, not the body. | Fixed: the shadow pass alpha-tested every blended draw; the bodies are drawn with Cs (ALPHA 0x2a), their alpha anything, so they were dropped. Only blends that use the alpha are tested now. |
| 5 | A far shadow plane appears at the very beginning of the race (Snow Jam, first 2000 frames). | Fixed: the lighting multiplied the colour after the fog, so distant terrain fogged into the sky was darkened into a visible slab; it is applied before the fog now. |
| 6 | The game's baked tree shadows still show with the setting on. | Fixed: the baked layer is no longer drawn at all (its colour part carried the tree shadows); `SSX3_SHADOW_DEBUG=tint` / F10 "Baked lighting: Tint" brings its colour back. |
| 7 | The snow trails should be shaded like the rest of the world. | Fixed: the board trails and wakes (depth-tested `Mat3Strip` draws) are lit like the terrain under them, their colour (the baked light where they were laid: dark lanes at the start) neutral. |

Seen while fixing these (not reported):

- The snow spray behind the riders takes the baked light of where it is
  (dark puffs over lit snow at the start).

Related requests (done): "Texture mipmaps", "Terrain detail" and "Rider detail
dist." removed (the game runs at their defaults); the lighting debug views as
env vars and a debug group in F10.
