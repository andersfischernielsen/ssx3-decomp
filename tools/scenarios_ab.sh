#!/bin/sh
# Run scripted scenarios (tools/scenarios.py) with two headless binaries and compare
# the final RAM: the tree under test against a reference build (the previous
# kernel). One heavy process at a time.
#   tools/scenarios_ab.sh REF_HEADLESS OUTDIR NAME...
set -e
cd "$(dirname "$0")/.."
REF=$1; OUT=$2; shift 2
mkdir -p "$OUT"
for n in "$@"; do
  IN=$(python3 tools/scenarios.py input $n); F=$(python3 tools/scenarios.py frames $n)
  CARD=$(python3 tools/scenarios.py card $n)
  card() { rm -f "$OUT/card.ps2"; [ "$CARD" = fresh ] || cp "$CARD" "$OUT/card.ps2"; }
  card; SSX3_MEMCARD="$OUT/card.ps2" SSX3_INPUT="$IN" SSX3_RAM_DUMP="$OUT/$n.new.bin" ./target/game/ssx3-headless $F > "$OUT/$n.new.log" 2>&1 || true
  card; SSX3_MEMCARD="$OUT/card.ps2" SSX3_INPUT="$IN" SSX3_RAM_DUMP="$OUT/$n.ref.bin" "$REF" $F > "$OUT/$n.ref.log" 2>&1 || true
  h1=$(grep -c "fatal\|deadlock" "$OUT/$n.new.log" || true); h0=$(grep -c fatal "$OUT/$n.ref.log" || true)
  if cmp -s "$OUT/$n.new.bin" "$OUT/$n.ref.bin"; then r=same; else r="DIFF ($(cmp -l "$OUT/$n.new.bin" "$OUT/$n.ref.bin" | wc -l | tr -d ' ') bytes)"; fi
  echo "$n: $r; halts new/ref $h1/$h0"
done
