#!/usr/bin/env python3
"""List the files on a raw PS2 memory card image (528-byte pages, e.g.
saves/ssx3-mc0.ps2 written by the emulated card in crates/ssx3-iop/src/memcard.rs).

    tools/mcls.py saves/ssx3-mc0.ps2

Follows the PS2 memory card file system: superblock (page 0), indirect FAT
cluster list, FAT, directory entries (512 bytes each). ECC is not checked.
"""
import struct
import sys

d = open(sys.argv[1], 'rb').read()


def page(n):
    return d[n * 528:n * 528 + 512]


sb = page(0) + page(1)
if not sb.startswith(b'Sony PS2 Memory Card Format '):
    sys.exit('unformatted (no superblock)')
print(sb[:28].decode(), 'version', sb[28:40].split(b'\0')[0].decode())
ps, ppc, ppb, _u, clusters, alloc_off, alloc_end, rootdir = struct.unpack_from('<HHHHIIII', sb, 0x28)
print(f'page {ps}, {ppc} pages/cluster, {ppb} pages/block, {clusters} clusters, '
      f'alloc offset {alloc_off}, alloc end {alloc_end}, root cluster {rootdir}')
ifc = struct.unpack_from('<32I', sb, 0x50)


def cluster(c):
    return b''.join(page(c * ppc + i) for i in range(ppc))


per = ps * ppc // 4
ind = struct.unpack_from(f'<{per}I', cluster(ifc[0]))


def fat(n):
    fc = struct.unpack_from(f'<{per}I', cluster(ind[n // per]))
    return fc[n % per]


def chain(c):
    out = [c]
    while True:
        v = fat(c)
        if v == 0xFFFFFFFF or not v & 0x80000000:
            return out
        c = v & 0x7FFFFFFF
        out.append(c)


def read(c, size):
    return b''.join(cluster(x + alloc_off) for x in chain(c))[:size]


def entries(c, n):
    b = read(c, n * 512)
    for i in range(n):
        e = b[i * 512:(i + 1) * 512]
        mode, = struct.unpack_from('<H', e, 0)
        length, = struct.unpack_from('<I', e, 4)
        cl, = struct.unpack_from('<I', e, 0x10)
        yield mode, length, cl, e[0x40:0x60].split(b'\0')[0].decode(errors='replace')


root_len = next(entries(rootdir, 1))[1]
for mode, length, cl, name in entries(rootdir, root_len):
    if name in ('.', '..'):
        continue
    print(f'{name}/' if mode & 0x20 else name, f'({length} entries)' if mode & 0x20 else f'{length} bytes')
    if mode & 0x20:
        for m2, l2, _c2, n2 in entries(cl, length):
            if n2 not in ('.', '..'):
                print(f'    {n2:32} {l2:8} bytes')
