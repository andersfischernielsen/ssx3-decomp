#!/usr/bin/env python3
"""prune_target.py [--apply] [--keep N] [--hours H] [PROFILE_DIR]

Every feature set `ssx3-game` is built with (the staging modules `ocN`, test
runs with `--features ...`) leaves its own build directory, fingerprint, rlib
and test binary in `target/<profile>`: hundreds of them after a few waves
(about 15 GB of the 37 GB `target/game` seen at wave 236). Cargo never removes
them. This deletes the variants other than the newest N and those touched in the
last H hours. A deleted variant only costs a rebuild if that exact feature set
is built again.

Without `--apply` it only reports what it would free.
"""
import glob
import os
import shutil
import sys
import time

args = sys.argv[1:]
apply = '--apply' in args
keep_n = int(args[args.index('--keep') + 1]) if '--keep' in args else 12
hours = float(args[args.index('--hours') + 1]) if '--hours' in args else 3.0
rest = [a for i, a in enumerate(args) if not a.startswith('--') and (i == 0 or args[i - 1] not in ('--keep', '--hours'))]
profile = rest[0] if rest else os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'target', 'game')
build = os.path.join(profile, 'build', 'ssx3-game')
if not os.path.isdir(build):
    sys.exit(f'no {build}')


def size(p):
    if os.path.isfile(p):
        return os.path.getsize(p)
    t = 0
    for r, _, fs in os.walk(p):
        for f in fs:
            try:
                t += os.path.getsize(os.path.join(r, f))
            except OSError:
                pass
    return t


now = time.time()
variants = sorted(((os.path.getmtime(os.path.join(build, h)), h) for h in os.listdir(build)), reverse=True)
keep = {h for _, h in variants[:keep_n]} | {h for m, h in variants if now - m < hours * 3600}
paths = []
for _, h in variants:
    if h in keep:
        continue
    paths += [os.path.join(build, h), os.path.join(profile, '.fingerprint', f'ssx3-game-{h}')]
    paths += glob.glob(os.path.join(profile, 'deps', f'*{h}*'))
total = sum(size(p) for p in paths if os.path.exists(p))
print(f'{len(variants)} variants, keeping {len(keep)}, stale {len(variants) - len(keep)}: {total / 2**30:.1f} GB')
if apply:
    for p in paths:
        if os.path.isdir(p):
            shutil.rmtree(p, ignore_errors=True)
        elif os.path.exists(p):
            os.remove(p)
    print('deleted')
else:
    print('(dry run: pass --apply)')
