#!/bin/sh
# Stage C check (docs/plans/native-rendering.md): headless runs with every
# native draw function checked against its generated body
# (SSX3_DRAW_CHECK=1; SSX3_DRAW_CHECK=0 for a plain native run), screenshots into <out>/<run>. Prints the check
# lines. Usage: scripts/draw-check.sh <out dir> [race|freestyle|menu ...]
set -e
cd "$(dirname "$0")/.."
OUT=$1; shift
RUNS=${*:-race menu}
B="1600:0008:8,2000:4000:8,2500:4000:8,2700:4000:8,2900:4000:8"
for r in $RUNS; do
  mkdir -p "$OUT/$r"
  case $r in
    race) IN="$B,3100:4000:8,3300:4000:8,3500:4000:8,3700:4000:8,4250:4000:8,4600:4000:3000"; N=6000 ;;
    freestyle) IN="$B,3000:0040:6,3100:4000:8,3300:4000:8,3500:4000:8,3700:4000:8,4250:4000:8,4600:4000:2400"; N=7000 ;;
    menu) IN=""; N=1600 ;;
    *) echo "unknown run $r"; exit 1 ;;
  esac
  SSX3_DRAW_CHECK=${SSX3_DRAW_CHECK-1} SSX3_INPUT="$IN" SSX3_SHOTS="$OUT/$r" ./target/game/ssx3-headless $N --shot-every 1000 > "$OUT/$r/log" 2>&1 || echo "$r: exit $?"
  grep -E "draw-check\] (total|.* call [0-9]+:)|panicked" "$OUT/$r/log" | sed "s/^/$r: /" | tail -40
done
