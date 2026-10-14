#!/usr/bin/env python3
"""List every file on an ISO9660 image with its LSN and size: iso-ls.py IMAGE [LSN...]"""
import struct, sys
f = open(sys.argv[1], 'rb')
def sec(n, c=1):
    f.seek(n * 2048); return f.read(c * 2048)
out = []
def walk(lsn, size, path):
    d = sec(lsn, (size + 2047) // 2048); o = 0
    while o < size:
        l = d[o]
        if l == 0:
            o = (o // 2048 + 1) * 2048; continue
        ext, sz, fl, nl = struct.unpack_from('<I', d, o + 2)[0], struct.unpack_from('<I', d, o + 10)[0], d[o + 25], d[o + 32]
        name = d[o + 33:o + 33 + nl]
        if name not in (b'\0', b'\1'):
            p = path + '/' + name.decode(errors='replace').split(';')[0]
            if fl & 2: walk(ext, sz, p)
            else: out.append((ext, sz, p))
        o += l
pvd = sec(16); r = pvd[156:190]
walk(struct.unpack_from('<I', r, 2)[0], struct.unpack_from('<I', r, 10)[0], '')
out.sort()
if len(sys.argv) > 2:
    for q in sys.argv[2:]:
        q = int(q, 16)
        for e, s, p in out:
            if e <= q < e + (s + 2047) // 2048: print(f'{q:#x} -> {p} (+{q-e} sectors, size {s})')
else:
    for e, s, p in out: print(f'{e:#8x} {s:10d} {p}')
