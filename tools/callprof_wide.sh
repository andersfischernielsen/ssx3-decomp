#!/bin/sh
# The call profile for pick_wave.py (target/gate/edges.tsv): all overrides off,
# the whole race replay (boot, menus, race) plus one run per event type,
# merged (counts summed, frames summed). About 15 minutes; one heavy process.
cd "$(dirname "$0")/.."
set -e
P=target/gate/prof; mkdir -p $P
CARGO_TARGET_DIR=target-callprof caffeinate -s cargo build --profile game -p ssx3 --bin ssx3-headless --features callprof > $P/build.log 2>&1
B=target-callprof/game/ssx3-headless
IN="$(python3 tools/integrate_wave.py race-input)"
SSX3_NO_OVERRIDES=1 SSX3_INPUT="$IN" SSX3_CALLPROF=$P/replay.tsv:1:6000 caffeinate -s $B 6000 > $P/replay.log 2>&1 || echo "replay failed"
for e in ARA1 BRA2 CRA3 DRA4 ERA5 ASS1 ABA1 BHP1 ABC1:6; do
  n=$(echo $e | tr ':' '_')
  SSX3_NO_OVERRIDES=1 SSX3_EVENT=$e SSX3_CALLPROF=$P/ev-$n.tsv:400:1600 caffeinate -s $B 1600 > $P/ev-$n.log 2>&1 || echo "$e failed"
done
python3 - <<'PY'
import glob, re
P='target/gate/prof'; counts={}; frames=0
for f in sorted(glob.glob(P+'/*.tsv')):
    for l in open(f):
        m=re.match(r'# frames (\d+)\.\.=(\d+)', l)
        if m: frames+=int(m[2])-int(m[1])+1; continue
        if l.startswith('#') or not l.strip(): continue
        a,b,n=l.split(); counts[(a,b)]=counts.get((a,b),0)+int(n)
with open('target/gate/edges.tsv','w') as o:
    o.write(f'# frames 1..={frames}\n# caller\tcallee\tcount\n')
    for (a,b),n in sorted(counts.items()): o.write(f'{a}\t{b}\t{n}\n')
print('frames', frames, 'edges', len(counts), 'callees', len({b for _,b in counts}))
PY
