#!/bin/sh
# Find one's way through the game's screens: a headless run with screenshots
# and a contact sheet of them (tools/contact_sheet.py).
#   scripts/explore.sh OUTDIR "INPUT" FRAMES [FIRST [EVERY]]
# INPUT: SSX3_INPUT presses (frame:buttons:frames, see tools/scenarios.py).
# Writes OUTDIR/frame*.png, OUTDIR/log and OUTDIR.png (frames FIRST..FRAMES).
set -e
cd "$(dirname "$0")/.."
OUT=$1
rm -rf "$OUT"; mkdir -p "$OUT"
SSX3_DEBUG_LENSFX_VIS=1 SSX3_INPUT="$2" SSX3_SHOTS="$OUT" ./target/game/ssx3-headless "$3" --shot-every "${5:-50}" > "$OUT/log" 2>&1 || true
git checkout -q generated/entries.txt 2>/dev/null || true
python3 tools/contact_sheet.py "$OUT" "$OUT.png" "${4:-0}" "$3"
