#!/usr/bin/env python3
"""Find per-tick updates that carry no 1/60 marker.

    python3 tools/scan_tick_patterns.py [--prof EDGES.TSV --frames N] [--tsv OUT]

`scan_framerate.py` finds the loads of 1/60 (the step) and friends. Code that
moves a value by a fixed amount *per tick* has no such marker: the start
gate's `launch += 0.015`, `v *= 0.98`, `x += (t - x) * 0.1`, `frames++`.
This scan follows each function's instructions in address order (ignoring
branches, so both arms of an `if` are seen) and reports every store of a
value computed from the same memory field it stores to plus a constant:

- STEP     field = field +/- k              (a float step per tick)
- SCALE    field = field * k                (decay/growth per tick)
- LERP     field = field + (t - field) * k  (smoothing per tick)
- CLAMP    the same with min/max            (a rate-limited approach)
- OTHER    any other mix of the field and a constant
- COUNT    integer field = field +/- n      (frame counters, but also
           indexes and reference counts)

A value that also involves 1/60, 1/30, 1/50 (the scanned step entries) or
the app manager's step is already rate-aware and is not reported. Constants
0 and +/-1 alone (a sign, a clear) are not reported for floats.

With a `SSX3_CALLPROF` profile (`--prof`, over `--frames` frames) each row
gets the function's calls per frame; functions that never ran are kept with
0. `ported` says whether the function is hand-written and `conv` whether
its port already uses `tick::` (some conversion is there).
"""
import bisect
import collections
import glob
import os
import re
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from scan_framerate import ELF, ROOT, sections  # noqa: E402

RATE_AWARE = {0x3C888889, 0x3D088889, 0x3CA3D70A, 0x3C888CE7}  # 1/60 1/30 1/50 0.01667
APPMAN = 0x4A5B64
COP1_S = {0: "add", 1: "sub", 2: "mul", 3: "div", 5: "abs", 6: "mov", 7: "neg",
          0x16: "rsqrt", 0x04: "sqrt", 0x18: "adda", 0x19: "suba", 0x1A: "mula",
          0x1C: "madd", 0x1D: "msub", 0x1E: "madda", 0x1F: "msuba",
          0x28: "max", 0x29: "min"}


def f32(w):
    return struct.unpack("<f", struct.pack("<I", w & 0xFFFFFFFF))[0]


class Val:
    """A float value's leaves (fields, constants) and the operations seen."""
    __slots__ = ("leaves", "ops")

    def __init__(self, leaves=frozenset(), ops=frozenset()):
        self.leaves, self.ops = leaves, ops

    def join(self, other, op):
        return Val(self.leaves | other.leaves, self.ops | other.ops | {op})


def overrides():
    """Hand-written functions: address -> whether the port uses tick::."""
    out = {}
    for path in glob.glob(os.path.join(ROOT, "crates/ssx3-game/src/**/*.rs"), recursive=True):
        t = open(path).read()
        for m in re.finditer(r"\(0x([0-9a-f_]+),\s*([a-z0-9_]+),\s*\"", t):
            addr = int(m.group(1).replace("_", ""), 16)
            name = m.group(2)
            body = re.search(r"\bfn " + re.escape(name) + r"\b\(.*?(?=\n(?:pub(?:\(crate\))? )?fn |\Z)", t, re.S)
            conv = bool(body and re.search(r"tick::|rate_step|per_tick_60|frame60|ticks_from_frames60", body.group(0)))
            out[addr] = out.get(addr, False) or conv
    return out


def main():
    d = open(ELF, "rb").read()
    sec = sections(d)
    gp = struct.unpack_from("<i", d, sec[".reginfo"][1] + 20)[0] & 0xFFFFFFFF

    def word_at(addr):
        for name in (".lit4", ".rodata", ".sdata", ".data", ".lit8", ".sbss"):
            if name not in sec:
                continue
            a, off, size = sec[name]
            if a <= addr < a + size and name != ".sbss":
                return struct.unpack_from("<I", d, off + addr - a)[0]
        return None

    funcs = []
    for line in open(os.path.join(ROOT, "generated/functions.tsv")):
        p = line.rstrip("\n").split("\t")
        if len(p) >= 3:
            funcs.append((int(p[0], 16), int(p[1], 16), p[2]))
    funcs.sort()

    calls = collections.Counter()
    frames = 1
    if "--prof" in sys.argv:
        frames = int(sys.argv[sys.argv.index("--frames") + 1])
        for line in open(sys.argv[sys.argv.index("--prof") + 1]):
            p = line.split()
            if len(p) >= 3 and not line.startswith("#"):
                try:
                    calls[int(p[1], 16)] += int(p[2])
                except ValueError:
                    pass
    ported = overrides()
    inventory = set()
    inv = os.path.join(ROOT, "docs/plans/tick-rate-functions.tsv")
    if os.path.exists(inv):
        for line in open(inv):
            if line.startswith("0x"):
                inventory.add(int(line.split()[0], 16))

    ta, toff, tsize = sec[".text"]
    rows = []
    for start, end, name in funcs:
        if not (ta <= start < ta + tsize):
            continue
        ver = [0] * 32  # GPR write versions (a field is (base, version, off))
        lui = {}  # GPR -> known constant
        app = set()  # GPRs holding the cAppMan pointer
        ints = {}  # GPR -> ("F", key) or ("INC", key, n)
        fpr = {}  # FPR -> Val
        acc = Val()
        found = []

        def wr(r):
            if r:
                ver[r] += 1
                lui.pop(r, None)
                app.discard(r)
                ints.pop(r, None)

        def key(base, off):
            if base == 28:
                return ("G", (gp + off) & 0xFFFFFFFF)
            if base == 29:
                return None  # the stack: locals
            return ("F", base, ver[base], off)

        call_at = None
        for a in range(start, end, 4):
            if call_at is not None and a == call_at + 8:
                fpr = {r: v for r, v in fpr.items() if r >= 20}
                for r in list(range(1, 16)) + [24, 25, 31]:
                    wr(r)
                call_at = None
            w = struct.unpack_from("<I", d, toff + a - ta)[0]
            op, rs, rt, imm = w >> 26, (w >> 21) & 31, (w >> 16) & 31, w & 0xFFFF
            simm = imm - 0x10000 if imm & 0x8000 else imm
            if op == 0x0F:  # lui
                wr(rt)
                lui[rt] = imm << 16
            elif op == 0x0D and rs in lui:  # ori
                v = lui[rs] | imm
                wr(rt)
                lui[rt] = v
            elif op == 0x09 and rs in lui:  # addiu from a lui
                v = (lui[rs] + simm) & 0xFFFFFFFF
                wr(rt)
                lui[rt] = v
            elif op == 0x09 and rs == 0:  # li
                wr(rt)
                lui[rt] = simm & 0xFFFFFFFF
            elif op == 0x09 or op == 0x19:  # addiu/daddiu
                src = ints.get(rs)
                wr(rt)
                if src and src[0] == "F" and simm and abs(simm) <= 16:
                    ints[rt] = ("INC", src[1], simm)
            elif op in (0x23, 0x21, 0x25, 0x20, 0x24, 0x37):  # lw lh lhu lb lbu ld
                k = key(rs, simm)
                is_app = rs == 28 and (gp + simm) & 0xFFFFFFFF == APPMAN
                wr(rt)
                if is_app:
                    app.add(rt)
                elif k:
                    ints[rt] = ("F", k)
            elif op in (0x2B, 0x29, 0x28, 0x3F):  # sw sh sb sd
                v = ints.get(rt)
                k = key(rs, simm)
                if v and v[0] == "INC" and k and v[1] == k:
                    found.append((a, "COUNT", f"{v[2]:+d}"))
            elif op == 0x31:  # lwc1
                if rs in app and simm == 0x14:
                    fpr[rt] = Val(frozenset({("DT",)}))
                    continue
                tgt = None
                if rs == 28:
                    tgt = (gp + simm) & 0xFFFFFFFF
                elif rs in lui:
                    tgt = (lui[rs] + simm) & 0xFFFFFFFF
                if tgt is not None and sec[".lit4"][0] <= tgt < sec[".lit4"][0] + sec[".lit4"][2]:
                    fpr[rt] = Val(frozenset({("K", word_at(tgt))}))
                else:
                    k = key(rs, simm) if tgt is None else ("G", tgt)
                    fpr[rt] = Val(frozenset({k})) if k else Val()
            elif op == 0x39:  # swc1
                k = key(rs, simm) if rs != 28 else ("G", (gp + simm) & 0xFFFFFFFF)
                v = fpr.get(rt)
                if k and v and k in v.leaves and v.ops:
                    consts = [x[1] for x in v.leaves if x[0] == "K"]
                    if any(x[0] == "DT" for x in v.leaves) or any(c in RATE_AWARE for c in consts if c is not None):
                        continue
                    consts = [c for c in consts if c is not None and f32(c) not in (0.0, 1.0, -1.0)
                              or (c is not None and f32(c) in (1.0, -1.0) and v.ops <= {"add", "sub"})]
                    if not consts:
                        continue
                    ops = v.ops - {"mov", "neg", "abs"}
                    if ops <= {"add", "sub"}:
                        cls = "STEP"
                    elif ops == {"mul"}:
                        cls = "SCALE"
                    elif ops & {"max", "min"}:
                        cls = "CLAMP"
                    elif {"sub", "mul"} <= ops and ops & {"add", "madd", "madda", "adda"}:
                        cls = "LERP"
                    elif ops & {"madd", "msub"} and len(ops) <= 2:
                        cls = "LERP"
                    else:
                        cls = "OTHER"
                    found.append((a, cls, " ".join(f"{f32(c):.6g}" for c in sorted(set(consts)))))
            elif op == 0x11:
                fmt = rs
                if fmt == 0x04:  # mtc1 rt -> fs
                    fs = (w >> 11) & 31
                    fpr[fs] = Val(frozenset({("K", lui[rt] if rt in lui else (0 if rt == 0 else None))})) \
                        if (rt in lui or rt == 0) else Val()
                elif fmt == 0x10:  # .s
                    fn = w & 63
                    ft, fs, fd = rt, (w >> 11) & 31, (w >> 6) & 31
                    o = COP1_S.get(fn)
                    a_, b_ = fpr.get(fs, Val()), fpr.get(ft, Val())
                    if o in ("add", "sub", "mul", "div", "max", "min"):
                        fpr[fd] = a_.join(b_, o)
                    elif o in ("mov", "neg", "abs", "sqrt", "rsqrt"):
                        fpr[fd] = Val(a_.leaves, a_.ops | {o})
                    elif o in ("adda", "suba", "mula"):
                        acc = a_.join(b_, o.rstrip("a"))
                    elif o in ("madda", "msuba"):
                        acc = acc.join(a_.join(b_, "mul"), "add")
                    elif o in ("madd", "msub"):
                        fpr[fd] = acc.join(a_.join(b_, "mul"), o)
                    elif fn >= 0x30:
                        pass  # compare
                    else:
                        fpr[fd] = Val()
                elif fmt == 0x14:  # .w (cvt.s.w)
                    fd, fs = (w >> 6) & 31, (w >> 11) & 31
                    fpr[fd] = Val()
                elif fmt == 0x00:  # mfc1 -> rt
                    wr(rt)
            elif (op == 0 and (w & 63) == 0x09) or op == 0x03:  # jalr/jal
                call_at = a  # clobbers temporaries after the delay slot
            elif op in (0x00,) and (w & 63) in (0x21, 0x2D, 0x25, 0x23, 0x2F, 0x24, 0x2A, 0x2B):
                rd = (w >> 11) & 31
                # move rd, rs (addu/daddu/or with $zero) keeps the field
                src = None
                if (w & 63) in (0x21, 0x2D, 0x25):
                    if rt == 0:
                        src = rs
                    elif rs == 0:
                        src = rt
                info = (ints.get(src), lui.get(src), src in app) if src is not None else None
                wr(rd)
                if info:
                    if info[0]:
                        ints[rd] = info[0]
                    if info[1] is not None:
                        lui[rd] = info[1]
                    if info[2]:
                        app.add(rd)
            elif op in (0x0A, 0x0B, 0x0C, 0x0E, 0x18, 0x1A, 0x1B):
                wr(rt)
            elif op == 0x00 and w != 0:
                fn = w & 63
                if fn not in (0x08, 0x09, 0x0C, 0x0D, 0x0F, 0x18, 0x19, 0x1A, 0x1B):
                    wr((w >> 11) & 31)
            elif op == 0x1C or op == 0x12:
                wr((w >> 11) & 31)
        for a, cls, k in found:
            rows.append((start, name, a, cls, k))

    names = {f[0]: f[2] for f in funcs}
    by_func = collections.OrderedDict()
    for start, name, a, cls, k in rows:
        by_func.setdefault((start, name), []).append((a, cls, k))
    out = sys.stdout
    if "--tsv" in sys.argv:
        out = open(sys.argv[sys.argv.index("--tsv") + 1], "w")
    out.write("func\tname\tcalls_per_frame\tported\tconv\tin_inventory\tsite\tclass\tconstants\n")
    order = sorted(by_func, key=lambda f: (-calls[f[0]], f[0]))
    per_class = collections.Counter()
    hot = 0
    for f in order:
        cpf = calls[f[0]] / frames
        hot += cpf > 0
        for a, cls, k in by_func[f]:
            per_class[cls] += 1
            out.write(f"0x{f[0]:08x}\t{f[1]}\t{cpf:.3f}\t{'yes' if f[0] in ported else 'no'}\t"
                      f"{'yes' if ported.get(f[0]) else 'no'}\t{'yes' if f[0] in inventory else 'no'}\t"
                      f"0x{a:x}\t{cls}\t{k}\n")
    print(f"{sum(per_class.values())} sites in {len(by_func)} functions ({hot} ran in the profile)",
          file=sys.stderr)
    print("by class:", ", ".join(f"{k}={v}" for k, v in per_class.most_common()), file=sys.stderr)


if __name__ == "__main__":
    main()
