#!/usr/bin/env python3
"""Fit the renderer's terrain lighting to the game's baked look
(docs/plans/terrain-lighting.md).

    tools/fit_lighting.py REF NOSUB NORMALS SUNLIT
    tools/fit_lighting.py REF NOSUB FIT
    tools/fit_lighting.py --tone REF NOSUB      (NOSUB run with lightlog too)
    tools/fit_lighting.py --exposure REF HDR    (sun scale and exposure)

FIT (one run instead of NORMALS and SUNLIT): SSX3_SHADOWS=1
SSX3_SHADOW_SHOW=fit SSX3_SHADOW_DEBUG=lightlog (n.x, n.y as 0..1, the lit
amount as 0.5..1; N·L from the logged sun).

Each argument is a directory of headless screenshots of the same frames:

  REF      the game as it is (the baked layer on): the target;
  NOSUB    the same frames without the baked layer (SSX3_SHADOW_DEBUG=no-sub);
  NORMALS  SSX3_SHADOWS=1 SSX3_SHADOW_SHOW=normals (n·0.5+0.5, lit terrain only);
  SUNLIT   SSX3_SHADOWS=1 SSX3_SHADOW_SHOW=sunlit (lit amount, N·L, 1).

SUNLIT's log.txt (run with SSX3_SHADOW_DEBUG=lightlog) gives the game's live
light at each screenshot (the lit meshes' ambient and key colour, 128 = 1):
the lighting triggers change it along a course.

On the lit terrain pixels (SUNLIT blue = 255) that show snow in NOSUB, the ratio
REF / NOSUB is the bake's light. Per channel it is fitted, by least squares,
as  sun·colour·(N·L·lit) + ambient_gain·ambient (sky = ground: terrain
normals barely vary in z, the two are not separable): the gains from the live light to the terrain's. Prints the
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


def live_light(log):
    """The live light at each screenshot of a lightlog run: (ambient, key
    colour), 128 = 1."""
    out, cur = {}, None
    for line in open(log, errors="replace"):
        if line.startswith("[light]"):
            v = line.split()
            try:
                cur = (
                    np.array([float(x) for x in v[v.index("amb") + 1 : v.index("amb") + 4]]) / 128,
                    np.array([float(x) for x in v[v.index("col") + 1 : v.index("col") + 4]]) / 128,
                )
            except ValueError:
                pass
        elif line.startswith("[shot]") and cur:
            out[f"frame{int(line.split()[1]):05}.png"] = cur
    return out


def tone(ref, nosub, sun_gain=0.6, sunlit_share=0.7):
    """The bake's tone: the per-channel ambient gain that makes the mean
    terrain light match the bake's mean (REF / NOSUB on snow), with the sun
    gain fixed (the bake's spatial pattern is art: the spatial structure is
    the real sun's). `sunlit_share`: the assumed mean N·L·lit."""
    live = live_light(os.path.join(nosub, "log.txt"))
    gains = []
    for f in sorted(live):
        if not os.path.exists(os.path.join(ref, f)):
            continue
        r, b = load(ref, f), load(nosub, f)
        m = (b.min(2) > 120) & (b.max(2) - b.min(2) < 60)
        if m.sum() < 1000:
            continue
        rat = (r[m] / np.maximum(b[m], 1)).mean(0)
        if rat.mean() < 0.6 or rat.mean() > 1.2:
            continue
        amb, col = live[f]
        gains.append((rat - sun_gain * col * sunlit_share) / np.maximum(amb, 1e-3))
    if not gains:
        sys.exit("no snow frames")
    g = np.median(np.array(gains), 0)
    print(f"frames {len(gains)}  ambient gain {g.round(3)}  (sun gain {sun_gain})")
    fmt = lambda c: ",".join(f"{x:.3f}" for x in c)
    print(f'SSX3_LIGHT="sun {sun_gain},{sun_gain},{sun_gain};sky {fmt(g)};ground {fmt(g)}"')
    return g


KNEE = 0.8


def shoulder(y, k=KNEE):
    """draw.wgsl `lit_linear`'s highlight shoulder on linear luminance."""
    return np.where(y > k, k + (1 - k) * (1 - np.exp(-(y - k) / (1 - k))), y)


def exposure(ref, hdr):
    """The sun scale and exposure that make the lit terrain's brightness
    match the reference (the game's baked look): HDR is a run with
    SSX3_SHADOWS=1 SSX3_SHADOW_SHOW=hdr SSX3_BLOOM=off (red: log2 of the
    terrain's linear luminance + 8 over 12, before the shoulder; blue: the
    sun's share of its light; green 255: terrain the game's image shows,
    everything else black). Fits the 10/50/90 % quantiles of the displayed
    luminance and the per-pixel error; prints (sun scale, exposure)."""
    ys, shares, refs = [], [], []
    for f in sorted(os.listdir(hdr)):
        if not f.endswith(".png") or not os.path.exists(os.path.join(ref, f)):
            continue
        h, r = load(hdr, f), load(ref, f)
        m = (h[..., 1] > 250) & (h[..., 0] > 0)
        if m.sum() < 2000:
            continue
        ys.append(2 ** (h[..., 0][m] / 255 * 12 - 8))
        shares.append(h[..., 2][m] / 255)
        refs.append((r @ [0.2126, 0.7152, 0.0722])[m])
    if not ys:
        sys.exit("no terrain")
    y, s, lr = (np.concatenate(x) for x in (ys, shares, refs))
    idx = np.random.default_rng(0).choice(len(y), min(len(y), 300000), replace=False)
    y, s, lr = y[idx], s[idx], lr[idx]
    qr = np.percentile(lr, [10, 50, 90])
    best = None
    for a in np.exp(np.linspace(np.log(0.25), np.log(12), 40)):
        ya = y * ((1 - s) + s * a)
        for e in np.exp(np.linspace(np.log(0.05), np.log(4), 80)):
            pred = 255 * np.clip(shoulder(e * ya), 0, 1) ** (1 / 2.2)
            err = np.abs(np.percentile(pred, [10, 50, 90]) - qr).mean() + 0.5 * np.median(np.abs(pred - lr))
            if best is None or err < best[0]:
                best = (err, a, e, np.percentile(pred, [10, 50, 90]), (pred >= 250).mean())
    err, a, e, qp, clip = best
    print(f"ref p10/50/90 {qr.round(1)} clipped {(lr >= 250).mean():.3f}")
    print(f"fit p10/50/90 {qp.round(1)} clipped {clip:.3f} error {err:.1f}")
    print(f"sun x{a:.3f} exposure {e:.3f}")
    return a, e


def main(argv):
    if argv and argv[0] == "--tone":
        tone(argv[1], argv[2])
        return
    if argv and argv[0] == "--exposure":
        exposure(argv[1], argv[2])
        return
    if len(argv) < 3:
        sys.exit(__doc__)
    combined = len(argv) == 3
    if combined:
        ref, nosub, sunlit = argv[:3]
        normals = sunlit
    else:
        ref, nosub, normals, sunlit = argv[:4]
    frames = sorted(os.path.basename(p) for p in glob.glob(os.path.join(sunlit, "frame*.png")))
    # The live light at each screenshot (the last [light] before its [shot]).
    live, cur = {}, None
    for line in open(os.path.join(sunlit, "log.txt"), errors="replace"):
        if line.startswith("[light]"):
            v = line.split()
            try:
                sun = [float(x) for x in v[v.index("sun") + 1 : v.index("sun") + 4]] if "sun" in v else [0, 0, 1]
                cur = (
                    [float(x) for x in v[v.index("amb") + 1 : v.index("amb") + 4]],
                    [float(x) for x in v[v.index("col") + 1 : v.index("col") + 4]],
                    sun,
                )
            except ValueError:
                pass
        elif line.startswith("[shot]") and cur:
            live[f"frame{int(line.split()[1]):05}.png"] = cur
    rows = []
    for f in frames:
        if not all(os.path.exists(os.path.join(d, f)) for d in (ref, nosub, normals)) or f not in live:
            continue
        amb, col = (np.array(x) / 128 for x in live[f][:2])
        sun = np.array(live[f][2])
        r, b, n, s = load(ref, f), load(nosub, f), load(normals, f), load(sunlit, f)
        if combined:
            # Back to the separate views: normals (z from x, y: facing the
            # camera, mostly up) and (lit, N·L, mask).
            nx, ny = n[..., 0] / 255 * 2 - 1, n[..., 1] / 255 * 2 - 1
            nz = np.sqrt(np.clip(1 - nx * nx - ny * ny, 0, 1))
            nv = np.stack([nx, ny, nz], -1)
            lit = np.clip((s[..., 2] / 255 - 0.5) * 2, 0, 1)
            terr = s[..., 2] >= 120
            n = (nv * 0.5 + 0.5) * 255
            ndl = np.clip(nv @ sun, 0, 1)
            s = np.stack([lit * 255, ndl * 255, np.where(terr, 255.0, 0.0)], -1)
        # Lit terrain that shows as snow in the bake-free frame (riders and
        # objects in front of it are not snow); frames whose ratio is far
        # off (menus, results screens over the course) left out.
        # Only where our sun shadows leave the surface lit: the bake's own
        # shadows are elsewhere (art), and fitting across them would flatten
        # the light; the shade's colour comes from the sky terms.
        mask = (s[..., 2] > 250) & (b.min(2) > 120) & (b.max(2) - b.min(2) < 60) & (s[..., 0] > 250)
        if mask.sum() < 1000:
            continue
        if (r[mask] / np.maximum(b[mask], 1)).mean() < 0.6:
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
    # One ambient gain (sky = ground: the terrain's normals barely vary in
    # z, so the two are not separable) and one sun gain per channel.
    for c in range(3):
        a = np.stack([am[:, c], co[:, c] * sunterm], 1)
        f = np.linalg.lstsq(a, rr[:, c], rcond=None)[0]
        g.append(f[0]); k.append(f[0]); u.append(f[1])
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
