#!/bin/sh
# Review harness (docs/plans/port-architecture.md, task 1): run the scripted
# scenarios of tools/scenarios.py with the ports under review installed and
# skipped (SSX3_SKIP_OVERRIDES), compare the RAM at the end. Deterministic
# (the generated light query). Prints one line per scenario.
#   scripts/scenarios.sh OUTDIR SKIPLIST [NAME...]
# SKIPLIST: file with one hex address per line (the ports under review).
set -e
cd "$(dirname "$0")/.."
OUT=$1; SKIP=$(tr '\n' ',' < "$2"); shift 2
NAMES=${*:-$(python3 tools/scenarios.py list)}
mkdir -p "$OUT"
for n in $NAMES; do
  IN=$(python3 tools/scenarios.py input $n); F=$(python3 tools/scenarios.py frames $n)
  CARD=$(python3 tools/scenarios.py card $n)
  # Each run gets its own copy of the card (the game may save).
  card() { rm -f "$OUT/card.ps2"; [ "$CARD" = fresh ] || cp "$CARD" "$OUT/card.ps2"; }
  card; SSX3_MEMCARD="$OUT/card.ps2" SSX3_DEBUG_LENSFX_VIS=1 SSX3_INPUT="$IN" SSX3_RAM_DUMP="$OUT/$n.on.bin" ./target/game/ssx3-headless $F > "$OUT/$n.on.log" 2>&1 || true
  card; SSX3_MEMCARD="$OUT/card.ps2" SSX3_DEBUG_LENSFX_VIS=1 SSX3_SKIP_OVERRIDES="$SKIP" SSX3_INPUT="$IN" SSX3_RAM_DUMP="$OUT/$n.off.bin" ./target/game/ssx3-headless $F > "$OUT/$n.off.log" 2>&1 || true
  git checkout -q generated/entries.txt 2>/dev/null || true
  h1=$(grep -c fatal "$OUT/$n.on.log" || true); h0=$(grep -c fatal "$OUT/$n.off.log" || true)
  if cmp -s "$OUT/$n.on.bin" "$OUT/$n.off.bin"; then r=same; else r="DIFF ($(cmp -l "$OUT/$n.on.bin" "$OUT/$n.off.bin" | wc -l | tr -d ' ') bytes)"; fi
  echo "$n: $r; halts on/off $h1/$h0"
done
