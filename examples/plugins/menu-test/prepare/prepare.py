#!/usr/bin/env python3
"""Importer of the menu-test plugin: the world copy of ../../world-copy (a
renamed copy of SSX 3's own BAM.BIG, from your disc into $SSX3_PLUGIN_OUT) and
a picture for the first peak, drawn here (no game material)."""
import os
import runpy
import struct
import sys
import zlib

HERE = os.path.dirname(os.path.abspath(__file__))


def png(width, height, pixel):
    raw = b"".join(
        b"\0" + b"".join(bytes(pixel(x, y)) for x in range(width)) for y in range(height)
    )

    def chunk(kind, data):
        body = kind + data
        return struct.pack(">I", len(data)) + body + struct.pack(">I", zlib.crc32(body))

    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(raw, 9))
        + chunk(b"IEND", b"")
    )


def sky_and_peak(x, y):
    # 192x144: a vertical gradient with a white triangle and a sun.
    if (x - 150) ** 2 + (y - 30) ** 2 < 15 ** 2:
        return (255, 200, 40)
    if y > 140 - 100 * (1 - abs(x - 96) / 80) and abs(x - 96) < 80 and y > 40:
        return (240, 240, 250)
    return (30, 60 + y // 2, 160)


def main():
    out = os.path.join(os.environ["SSX3_PLUGIN_OUT"], "ui")
    os.makedirs(out, exist_ok=True)
    with open(os.path.join(out, "peak.png"), "wb") as f:
        f.write(png(192, 144, sky_and_peak))
    # Last: it creates `ready`.
    runpy.run_path(os.path.join(HERE, "..", "..", "world-copy", "prepare", "prepare.py"), run_name="__main__")


if __name__ == "__main__":
    sys.exit(main())
