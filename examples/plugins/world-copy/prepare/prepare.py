#!/usr/bin/env python3
"""Spike importer: DATA/WORLDS/BAX.BIG, a copy of SSX 3's BAM.BIG with the
entries renamed bam.* -> bax.* (same length, so the archive layout is
unchanged). Reads the SSX 3 disc, writes only under $SSX3_PLUGIN_OUT.
"""
import os
import struct
import sys

SECTOR = 2048


def sectors(f, lba, n):
    f.seek(lba * SECTOR)
    return f.read(n * SECTOR)


def find(f, path):
    pvd = sectors(f, 16, 1)
    lba, size = struct.unpack_from("<I", pvd, 156 + 2)[0], struct.unpack_from("<I", pvd, 156 + 10)[0]
    cur = (lba, size)
    for part in path.upper().split("/"):
        data = sectors(f, cur[0], (cur[1] + SECTOR - 1) // SECTOR)[: cur[1]]
        p, hit = 0, None
        while p < len(data):
            n = data[p]
            if n == 0:
                p = (p // SECTOR + 1) * SECTOR
                continue
            name = data[p + 33 : p + 33 + data[p + 32]].decode("latin1").split(";")[0].upper()
            if name == part:
                hit = (struct.unpack_from("<I", data, p + 2)[0], struct.unpack_from("<I", data, p + 10)[0])
                break
            p += n
        if hit is None:
            sys.exit(f"{path}: not on the disc")
        cur = hit
    return cur


def main():
    disc, out = os.environ["SSX3_BASE_DISC"], os.environ["SSX3_PLUGIN_OUT"]
    dest = os.path.join(out, "overlay", "DATA", "WORLDS", "BAX.BIG")
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    with open(disc, "rb") as f:
        lba, size = find(f, "DATA/WORLDS/BAM.BIG")
        first = sectors(f, lba, 1)
        if first[:4] != b"BIGF":
            sys.exit("BAM.BIG is not a BIGF archive")
        header_size = struct.unpack_from(">I", first, 12)[0]
        head = bytearray(sectors(f, lba, (header_size + SECTOR - 1) // SECTOR + 1))
        head = head[: min(len(head), size)]
        n = struct.unpack_from(">I", head, 8)[0]
        p = 16
        for _ in range(n):
            p += 8
            end = head.index(b"\0", p)
            if bytes(head[p:end]).lower().startswith(b"data/worlds/bam."):
                head[p + len(b"data/worlds/") : p + len(b"data/worlds/bam")] = b"bax"
            p = end + 1
        with open(dest, "wb") as o:
            o.write(head)
            f.seek(lba * SECTOR + len(head))
            left = size - len(head)
            while left > 0:
                chunk = f.read(min(left, 8 << 20))
                if not chunk:
                    sys.exit("short read")
                o.write(chunk)
                left -= len(chunk)
    open(os.path.join(out, "ready"), "w").write("")
    print(f"wrote {dest}")


if __name__ == "__main__":
    main()
