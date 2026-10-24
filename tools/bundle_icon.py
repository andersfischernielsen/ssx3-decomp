"""The app icon for standalone builds (tools/bundle.py), made from the game's
own SSX 3 logo: the `ssxt` texture of `DATA/UI/FE_1.SSH` on the disc (the
title screen's logo; the riders' silhouettes below it on the same sheet are
cut off). Plain Python (no Pillow): SSH decoding, compositing, resizing, PNG,
ICO, and the macOS iconset for `iconutil`.

The icon: the logo on a dark blue square (see `compose`).
"""
import math
import os
import struct
import zlib

SHEET = "/DATA/UI/FE_1.SSH"
# The logo's width as a share of the icon's.
LOGO_WIDTH = 0.84
LOGO = b"ssxt"


def ssh_entry(data, name):
    """(width, height, RGBA bytes) of the 8-bit paletted SSH entry `name`."""
    if data[:4] != b"SHPS":
        raise ValueError("not an SSH sheet")
    count = struct.unpack_from("<I", data, 8)[0]
    for i in range(count):
        if data[16 + 8 * i:20 + 8 * i] != name:
            continue
        off = struct.unpack_from("<I", data, 20 + 8 * i)[0]
        kind = data[off]
        nxt = int.from_bytes(data[off + 1:off + 4], "little")
        w, h = struct.unpack_from("<HH", data, off + 4)
        if kind != 0x02 or not nxt:
            raise ValueError(f"{name}: unexpected format {kind:#x}")
        px = data[off + 16:off + 16 + w * h]
        pal_off = off + nxt
        if data[pal_off] != 0x21:
            raise ValueError(f"{name}: no 32-bit palette")
        pal = data[pal_off + 16:pal_off + 16 + 256 * 4]

        # The PS2 stores 256-entry palettes with entries 8-15 and 16-23 of
        # each 32 swapped (CSM1); alpha is 0..0x80.
        def entry(k):
            k = (k & 0xE7) | ((k & 8) << 1) | ((k & 16) >> 1)
            r, g, b, a = pal[4 * k:4 * k + 4]
            return bytes((r, g, b, min(255, a * 2)))

        lut = [entry(k) for k in range(256)]
        return w, h, b"".join(lut[b] for b in px)
    raise KeyError(name)


def logo_from_disc(read_file):
    """The logo, cropped to its pixels: (w, h, [r, g, b, a] floats rows)."""
    w, h, rgba = ssh_entry(read_file(SHEET), LOGO)
    alpha = lambda x, y: rgba[4 * (y * w + x) + 3]
    rows = [any(alpha(x, y) > 16 for x in range(w)) for y in range(h)]
    # The logo is the first block of rows with pixels; the silhouettes are
    # below an empty gap.
    top = rows.index(True)
    bottom = top
    while bottom < h and any(rows[bottom:bottom + 24]):
        bottom += 1
    cols = [any(alpha(x, y) > 16 for y in range(top, bottom)) for x in range(w)]
    left, right = cols.index(True), w - cols[::-1].index(True)
    out = []
    for y in range(top, bottom):
        o = 4 * (y * w)
        out.append([list(rgba[o + 4 * x:o + 4 * x + 4]) for x in range(left, right)])
    return right - left, bottom - top, out


def compose(logo, size=1024):
    """The icon at `size`: a full square with a vertical blue gradient and the
    logo centred (area resampling), as RGBA rows of floats 0..255. Square on
    purpose: macOS masks app icons into its own rounded shape (a full-bleed
    square is what it masks cleanly), Windows and Linux show squares. The
    logo keeps clear of the corners macOS cuts off."""
    lw, lh, lpx = logo
    img = []
    for y in range(size):
        t = y / (size - 1)
        img.append([[10 + 8 * t, 26 + 20 * t, 58 + 40 * t, 255.0] for _ in range(size)])
    scale = min(size * LOGO_WIDTH / lw, size * LOGO_WIDTH / lh)
    tw, th = max(1, round(lw * scale)), max(1, round(lh * scale))
    small = resize((lw, lh, lpx), tw, th)
    ox, oy = round((size - tw) / 2), round((size - th) / 2)
    for y in range(th):
        for x in range(tw):
            r, g, b, a = small[y][x]
            if a <= 0:
                continue
            p = img[oy + y][ox + x]
            k = a / 255.0
            p[0] = r * k + p[0] * (1 - k)
            p[1] = g * k + p[1] * (1 - k)
            p[2] = b * k + p[2] * (1 - k)
    return size, size, img


def resize(src, tw, th):
    """Area-average resampling of RGBA float rows (alpha-weighted colour)."""
    sw, sh, px = src
    out = []
    fx, fy = sw / tw, sh / th
    for ty in range(th):
        y0, y1 = ty * fy, (ty + 1) * fy
        row = []
        for tx in range(tw):
            x0, x1 = tx * fx, (tx + 1) * fx
            acc = [0.0, 0.0, 0.0, 0.0]
            area = 0.0
            for y in range(int(y0), min(sh, math.ceil(y1))):
                wy = min(y1, y + 1) - max(y0, y)
                for x in range(int(x0), min(sw, math.ceil(x1))):
                    wgt = wy * (min(x1, x + 1) - max(x0, x))
                    r, g, b, a = px[y][x]
                    aw = a * wgt
                    acc[0] += r * aw
                    acc[1] += g * aw
                    acc[2] += b * aw
                    acc[3] += aw
                    area += wgt
            if acc[3] > 0:
                row.append([acc[0] / acc[3], acc[1] / acc[3], acc[2] / acc[3], acc[3] / area])
            else:
                row.append([0.0, 0.0, 0.0, 0.0])
        out.append(row)
    return out


def png(img):
    """PNG bytes of RGBA float rows."""
    w, h, px = img
    raw = b"".join(b"\0" + bytes(max(0, min(255, round(v))) for p in row for v in p) for row in px)

    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)

    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress(raw, 9)) + chunk(b"IEND", b""))


def sizes(icon, wanted):
    """{size: PNG bytes} for each size in `wanted`: halvings of the icon one
    after another (each from the one before), other sizes from the next
    larger halving."""
    levels = {icon[0]: icon[2]}
    s = icon[0]
    while s > min(wanted):
        levels[s // 2] = resize((s, s, levels[s]), s // 2, s // 2)
        s //= 2
    out = {}
    for w in wanted:
        if w in levels:
            out[w] = png((w, w, levels[w]))
        else:
            src = min(k for k in levels if k > w)
            out[w] = png((w, w, resize((src, src, levels[src]), w, w)))
    return out


def write_ico(pngs, path):
    """A Windows .ico holding PNG images (Vista+)."""
    items = sorted(pngs.items())
    head = struct.pack("<HHH", 0, 1, len(items))
    off = 6 + 16 * len(items)
    dirs, blobs = b"", b""
    for s, data in items:
        dirs += struct.pack("<BBBBHHII", s % 256, s % 256, 0, 0, 1, 32, len(data), off)
        off += len(data)
        blobs += data
    with open(path, "wb") as f:
        f.write(head + dirs + blobs)


def write_iconset(pngs, path):
    """A macOS .iconset directory (for `iconutil -c icns`)."""
    os.makedirs(path, exist_ok=True)
    for base in (16, 32, 128, 256, 512):
        with open(os.path.join(path, f"icon_{base}x{base}.png"), "wb") as f:
            f.write(pngs[base])
        with open(os.path.join(path, f"icon_{base}x{base}@2x.png"), "wb") as f:
            f.write(pngs[base * 2])
