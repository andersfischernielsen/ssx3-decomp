# Function inventory

## Export

`tools/export-ghidra-functions.py` talks to the GhidraMCP backend:

- Phase 1 pages `/functions` (page cap 2000) for names.
- Phase 2 queries `/function?address=` per entry for the exact `body_end`.

Output (committed, ~600 KB):

```
generated/functions.json   [{ "name", "address", "end" }] sorted by address
generated/functions.tsv    "<hex-address>\t<hex-end>\t<name>"  (embedded by ssx3-recomp)
```

`end` is **exclusive** (Ghidra's inclusive `body_end` + 1).

## Shape

| | |
| --- | --- |
| functions | **11,844** |
| entry range | `00100008` .. `0042e508` |
| bodies | 8 B .. 40,536 B; 3,204,968 B total |
| recovered names | **75** (`entry`, `SetGsCrt`, `GsPutIMR`, `AddIntcHandler`, `_Exit`, `_LoadExecPS2`, `SetVTLBRefillHandler`, ...) |
| auto placeholders | 11,769 (`FUN_…`) |

All function entries are within `.text` (11,843) plus one in a linkonce section.

## Why extents come from Ghidra, not neighbours

Deriving a function's end from the next entry's address **overestimates**: e.g.
`FUN_001001c8` is only 8 bytes while the next entry is 16 bytes later (there is
alignment/other data between functions). Ghidra's `body_end` is authoritative and
is what the inventory stores.

## Query tools

`ssx3-recomp` embeds the TSV; `FunctionIndex` offers `by_address`, `containing`
(entry-inclusive, end-exclusive), `search` and named-vs-auto classification. The
`recomp-stats` CLI wraps it:

```sh
recomp-stats                 # summary + named list
recomp-stats 0x0012ab34      # address lookup (or containing function)
recomp-stats SetGsCrt        # name search
recomp-stats --coverage      # decode the whole image, report coverage
recomp-stats --disasm entry  # disassemble a function
```
