#!/usr/bin/env python3
"""Fit the renderer's terrain lighting to the game's baked look
(docs/plans/terrain-lighting.md).

    tools/fit_lighting.py REF NOSUB NORMALS SUNLIT [--sun x,y,z]

Each argument is a directory of headless screenshots of the same frames:

  REF      the game as it is (the baked layer on): the target;
  NOSUB    the same frames without the baked layer (SSX3_SHADOW_DEBUG=no-sub);
  NORMALS  SSX3_SHADOWS=1 SSX3_SHADOW_SHOW=normals (n·0.5+0.5, lit terrain only);
  SUNLIT   SSX3_SHADOWS=1 SSX3_SHADOW_SHOW=sunlit (lit amount, N·L, 1).

SUNLIT's log.txt (run with SSX3_SHADOW_DEBUG=lightlog) gives the game's live
light at each screenshot (the lit meshes' ambient and key colour, 128 = 1):
the lighting triggers change it along a course.

On the lit terrain pixels (SUNLIT blue = 255) of the lower half of each frame
(near terrain: the game's distance fog adds least there) the ratio
REF / NOSUB is the bake's light. Per channel it is fitted, by least squares,
as  sun·colour·(N·L·lit) + sky·ambient·h + ground·ambient·(1 − h),
h = n.z·0.5 + 0.5: the gains from the live light to the terrain's. Prints the
SSX3_LIGHT string (the gains), the fit's error, and the sun direction (no
shadows) that would match the bake best, for comparison.
"""
import glob
import os
import sys

import numpy as np
from PIL import Image


def load(d, f):
    return np.asarray(Image.open(os.path.join(d, f)).convert("RGB")).astype(np.float64)


def main(argv):
    if len(argv) < 4:
        sys.exit(__doc__)
    ref, nosub, normals, sunlit = argv[:4]
    frames = sorted(os.path.basename(p) for p in glob.glob(os.path.join(sunlit, "frame*.png")))
    # The live light at each screenshot (the last [light] before its [shot]).
    live, cur = {}, None
    for line in open(os.path.join(sunlit, "log.txt"), errors="replace"):
        if line.startswith("[light]"):
            v = line.split()
            try:
                cur = ([float(x) for x in v[v.index("amb") + 1 : v.index("amb") + 4]], [float(x) for x in v[v.index("col") + 1 : v.index("col") + 4]])
            except ValueError:
                pass
        elif line.startswith("[shot]") and cur:
            live[f"frame{int(line.split()[1]):05}.png"] = cur
    rows = []
    for f in frames:
        if not all(os.path.exists(os.path.join(d, f)) for d in (ref, nosub, normals)) or f not in live:
            continue
        amb, col = (np.array(x) / 128 for x in live[f])
        r, b, n, s = load(ref, f), load(nosub, f), load(normals, f), load(sunlit, f)
        h = r.shape[0]
        mask = (s[..., 2] > 250) & (b.min(2) > 40)
        mask[: h // 2] = False
        if mask.sum() < 100:
            continue
        idx = np.flatnonzero(mask)
        if len(idx) > 4000:
            idx = np.random.default_rng(0).choice(idx, 4000, replace=False)
        rr = r.reshape(-1, 3)[idx] / np.maximum(b.reshape(-1, 3)[idx], 1)
        nn = n.reshape(-1, 3)[idx] / 255 * 2 - 1
        ss = s.reshape(-1, 3)[idx] / 255
        rows.append((rr, nn, ss, np.tile(amb, (len(idx), 1)), np.tile(col, (len(idx), 1))))
    if not rows:
        sys.exit("no lit terrain pixels")
    rr = np.concatenate([x[0] for x in rows])
    nn = np.concatenate([x[1] for x in rows])
    ss = np.concatenate([x[2] for x in rows])
    am = np.concatenate([x[3] for x in rows])
    co = np.concatenate([x[4] for x in rows])
    hz = nn[:, 2] * 0.5 + 0.5
    sunterm = ss[:, 0] * ss[:, 1]
    g, k, u, errs = [], [], [], []
    for c in range(3):
        a = np.stack([am[:, c] * (1 - hz), am[:, c] * hz, co[:, c] * sunterm], 1)
        f = np.linalg.lstsq(a, rr[:, c], rcond=None)[0]
        g.append(f[0]); k.append(f[1]); u.append(f[2])
        errs.append(np.abs(a @ f - rr[:, c]).mean())
    err = float(np.mean(errs))
    base = np.abs(rr - rr.mean(0)).mean()
    fmt = lambda c: ",".join(f"{x:.3f}" for x in c)
    print(f"pixels {len(rr)}  mean abs error {err:.4f} (constant: {base:.4f})")
    print(f'SSX3_LIGHT="sun {fmt(u)};sky {fmt(k)};ground {fmt(g)}"')
    # The bake's own sun: the direction whose N·L (no shadows) fits best.
    best = None
    for el in range(5, 90, 5):
        for az in range(0, 360, 10):
            e, z = np.radians(el), np.radians(az)
            l = np.array([np.cos(e) * np.cos(z), np.cos(e) * np.sin(z), np.sin(e)])
            aa = np.stack([1 - hz, hz, np.maximum(nn @ l, 0)], 1)
            ff = np.linalg.lstsq(aa, rr, rcond=None)[0]
            ee = np.abs(aa @ ff - rr).mean()
            if best is None or ee < best[0]:
                best = (ee, az, el)
    print(f"bake's best sun (no shadows): azimuth {best[1]}, elevation {best[2]}, error {best[0]:.4f}")


if __name__ == "__main__":
    main(sys.argv[1:])
