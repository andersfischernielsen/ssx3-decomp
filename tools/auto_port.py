#!/usr/bin/env python3
"""Mechanically port tiny straight-line leaf functions to staged Rust modules.

    python3 tools/auto_port.py [LIST] [--wave 276] [--first-oc 1501] [--per 45]

LIST (default target/gate/noee/auto-leaf.txt) has `0xADDR<TAB>bytes` lines. For
each function whose generated body is straight-line, returns through `ra` and
only uses the plain ALU/load/store/FPU statements this tool understands, it
writes a hand-written port (the generated statements verbatim, the final
`c.pc = t; return;` replaced by `ret(c)`) and a differential test against
`ee::<shard>::f_ADDR__generated`, into staged modules `ocN.rs` (about `--per`
functions each, wired into Cargo.toml / lib.rs like `stage_wave.py stage`).
It also writes `target/gate/waveW/{waveW-all.txt,list-ocN.txt,models.tsv,
skipped.tsv}` and copies the wave file to `target/gate/waves/`.

Everything else is skipped with a reason (see skipped.tsv): calls, syscalls,
events, branches, returns through another register, unknown statements, base
registers whose value the fixture cannot make valid.

The fixture: every register the body reads before writing is an input.
Registers used as a memory base (directly, or `ptr + index`) point into a
random-filled data area (16-byte aligned, offsets of the body covered); when a
base is itself loaded from memory the area is filled with pointers into itself
(pointer chasing); other inputs get edge-case 64-bit values (0, 1, -1, small,
random); FPU inputs get `Rng::edge_f32`. Then run `python3 tools/integrate_wave.py
slots target/gate/waveW/waveW-all.txt` and `python3 tools/lint_port.py ocN ...`.
The run is idempotent: it regenerates the modules of its own numbering.
"""
import os
import re
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import stage_wave  # noqa: E402

R = stage_wave.R
SRC = stage_wave.SRC
GEN = os.path.join(R, "generated/ee")

REGS = {
    "ZERO": 0, "AT": 1, "V0": 2, "V1": 3, "A0": 4, "A1": 5, "A2": 6, "A3": 7,
    "T0": 8, "T1": 9, "T2": 10, "T3": 11, "T4": 12, "T5": 13, "T6": 14, "T7": 15,
    "S0": 16, "S1": 17, "S2": 18, "S3": 19, "S4": 20, "S5": 21, "S6": 22, "S7": 23,
    "T8": 24, "T9": 25, "K0": 26, "K1": 27, "GP": 28, "SP": 29, "FP": 30, "S8": 30, "RA": 31,
}
NAMES = {v: k for k, v in REGS.items() if k != "S8"}
FIXED = {0, 28, 29, 31}  # zero, gp, sp, ra: never randomised

RESERVED = {
    "ret", "sx", "sp", "guest", "r32", "call_ptr", "check", "check_lohi", "machine", "fill",
    "scramble", "leaf_val", "leaf_fill", "as", "break", "const", "continue", "crate", "else",
    "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "static", "struct", "super", "trait",
    "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn", "abstract",
    "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    "virtual", "yield", "try", "gen", "main", "test", "tests", "c", "r", "seed", "ee",
}

LOADS = {"lw", "ld", "lb", "lbu", "lh", "lhu", "lwu", "lq", "lwc1"}
STORES = {"sw", "sd", "sb", "sh", "sq", "swc1"}

REG = r"c\.g\[(\w+)\]\[0\]"
ADDR = re.compile(r"\(c\.g\[(\w+)\]\[0\] as u32\)(?:\.wrapping_(add|sub)\((0x[0-9a-fA-F]+|\d+)\))?")


def regno(tok):
    return REGS[tok] if tok in REGS else int(tok)


# ---------------------------------------------------------------- inputs

_shard_text = {}


def shard_lines(s):
    if s not in _shard_text:
        _shard_text[s] = open(os.path.join(GEN, s, "src/lib.rs")).read().split("\n")
    return _shard_text[s]


ALIASES = {}
FIRST_OC = 1501


def symbols():
    out = {}
    for l in open(os.path.join(R, "generated/ee-symbols.tsv")):
        if l[0] == "#":
            continue
        p = l.rstrip("\n").split("\t")
        out[int(p[1], 16)] = p[0]
        if len(p) > 4 and p[4] not in ("-", ""):
            ALIASES[int(p[1], 16)] = p[4].split(",")[0]
    return out


def fn_names():
    out = {}
    for l in open(os.path.join(R, "generated/functions.tsv")):
        p = l.rstrip("\n").split("\t")
        try:
            out[int(p[0], 16)] = p[2] if len(p) > 2 else None
        except ValueError:
            pass
    return out


def body_of(shard, a):
    L = shard_lines(shard)
    heads = [i for i, x in enumerate(L) if x.startswith(f"pub fn f_{a:08x}__generated(") or x.startswith(f"pub fn f_{a:08x}(")]
    g = [i for i in heads if "__generated" in L[i]]
    heads = g or heads
    if len(heads) != 1:
        return None
    i = heads[0] + 1
    out = []
    while i < len(L) and L[i] != "}":
        out.append(L[i])
        i += 1
    return out


def registered_addrs():
    """Addresses that already have a hand-written OVERRIDES entry."""
    pat = re.compile(r'\(\s*(0x[0-9a-fA-F_]{8,10})\s*,')
    out = set()
    for root, _d, files in os.walk(SRC):
        for f in files:
            if f.endswith(".rs"):
                mo = re.fullmatch(r"oc(\d+)\.rs", f)
                if mo and int(mo.group(1)) >= FIRST_OC:
                    continue  # our own staged modules (rerun)
                t = open(os.path.join(root, f)).read()
                i = t.find("pub const OVERRIDES")
                if i < 0:
                    continue
                for m in pat.finditer(t[i:]):
                    out.add(int(m.group(1).replace("_", ""), 16))
    return out


def crate_fn_names():
    names = set()
    for root, _d, files in os.walk(SRC):
        for f in files:
            if f.endswith(".rs"):
                names.update(re.findall(r"\bfn (\w+)", open(os.path.join(root, f)).read()))
    return names


# ---------------------------------------------------------------- analysis


class Skip(Exception):
    pass


class Analysis:
    """Abstract interpretation of a straight-line body: which registers are inputs
    and how each input is used (memory base pointer, index, plain value)."""

    def __init__(self):
        self.written = set()
        self.state = {}  # reg -> abstract value
        self.reads = []  # gpr read before written (initial values)
        self.fread = []
        self.fwritten = set()
        self.ptr = set()
        self.idx = set()
        self.chase = False
        self.lo = 0
        self.hi = 0
        self.has_base = False
        self.uses_mult = False
        self.heur = False  # two-input sum used as a base: pointer/index guessed

    def val(self, r):
        if r == 0:
            return ("const",)
        if r in self.state:
            return self.state[r]
        return ("in", r)

    def read(self, r):
        if r not in self.written and r not in FIXED and r not in self.reads:
            self.reads.append(r)

    def write(self, r, v):
        self.written.add(r)
        self.state[r] = v

    def fread_(self, f):
        if f not in self.fwritten and f not in self.fread:
            self.fread.append(f)

    def use_base(self, v, off):
        """The abstract value `v` is used as a memory base (+ off)."""
        k = v[0]
        if k == "const":
            return
        self.lo = min(self.lo, off)
        self.hi = max(self.hi, off)
        if k == "in":
            if v[1] in FIXED:
                return
            self.ptr.add(v[1])
            self.has_base = True
        elif k == "sum":
            a, b = v[1], v[2]
            if a[0] == "const":
                a, b = b, a
            if b[0] == "const":
                self.use_base(a, off)
                return
            if a[0] == "in" and b[0] in ("in", "idx"):
                self.heur = self.heur or b[0] == "in"
                self.ptr.add(a[1])
                self.idx.add(b[1])
                self.has_base = True
            elif a[0] == "idx" and b[0] == "in":
                self.heur = True
                self.ptr.add(b[1])
                self.idx.add(a[1])
                self.has_base = True
            elif a[0] == "ld":
                self.chase = True
                self.has_base = True
            elif a[0] == "in":  # ptr + loaded/other
                self.ptr.add(a[1])
                self.has_base = True
                self.chase = self.chase or b[0] == "ld"
            else:
                raise Skip("unclassifiable base (sum)")
        elif k == "ld":
            self.chase = True
        elif k == "idx":
            raise Skip("base is a scaled index")
        else:
            raise Skip("unclassifiable base (computed)")

    def has_chase_source(self):
        return self.has_base


def add_vals(a, b):
    if a[0] == "const" and b[0] == "const":
        return ("const",)
    return ("sum", a, b)


def parse_body(lines):
    """-> (statements, analysis, uses_ra_late). Raises Skip."""
    st = []
    for l in lines:
        s = l.strip()
        if not s or s.startswith("ssx3_ee_rt::prof::enter"):
            continue
        st.append(s)
    if len(st) < 3:
        raise Skip("too short")
    if st[-1] != "c.pc = t; return;":
        raise Skip("does not end with `c.pc = t; return;`")
    lets = [i for i, s in enumerate(st) if s.startswith("let ")]
    if len(lets) != 1 or st[lets[0]] != "let t = c.g[RA][0] as u32;":
        raise Skip("tail is not `let t = ra`: other return or conditional")
    if st[0].startswith("c.cycles +=") is False:
        raise Skip("no leading cycle charge")
    A = Analysis()
    body = []
    for i, s in enumerate(st):
        if i == lets[0] or i == len(st) - 1:
            continue
        body.append(s)
        analyse_line(A, s)
    late = any(
        re.match(r"c\.g\[RA\]\[0\] =", s) for s in st[lets[0] + 1 : -1]
    )
    return st, lets[0], A, late


def pure_expr(e):
    left = re.sub(REG, "", e)
    left = re.sub(r"\b(sx|as|u32|u64|i64|i32|wrapping_add|wrapping_sub|wrapping_mul)\b", "", left)
    left = re.sub(r"(?:0x[0-9a-fA-F]+|\d+)(?:u64)?", "", left)
    return re.fullmatch(r"[\s().<>=&|^!+*-]*", left) is not None


def regs_in(expr):
    return [regno(m) for m in re.findall(REG, expr)]


def mem_addr(A, text):
    m = ADDR.search(text)
    if not m:
        raise Skip(f"unparsed address in `{text}`")
    base = regno(m.group(1))
    off = 0
    if m.group(2):
        off = int(m.group(3), 0)
        if m.group(2) == "sub":
            off = -off
    A.read(base)
    A.use_base(A.val(base), off)
    return base


def analyse_line(A, s):
    if s.startswith("c.cycles +="):
        return
    m = re.match(rf"{REG} = (.*);$", s)
    if m:
        d = regno(m.group(1))
        e = m.group(2)
        if d in FIXED and d not in (0, 29):
            raise Skip(f"writes {NAMES[d]}")
        if d == 0:
            raise Skip("writes zero register")
        ld = re.match(r"c\.(\w+)\((.*)\)$", e)
        if ld:
            op = ld.group(1)
            if op not in LOADS:
                raise Skip(f"unhandled op c.{op}")
            mem_addr(A, ld.group(2))
            A.write(d, ("ld",))
            return
        for r in regs_in(e):
            A.read(r)
        if re.fullmatch(r"0u64|0x[0-9a-fA-F]+|\d+", e):
            A.write(d, ("const",))
        elif re.fullmatch(REG, e):
            A.write(d, A.val(regno(re.fullmatch(REG, e).group(1))))
        elif re.fullmatch(rf"sx\(\(c\.g\[(\w+)\]\[0\] as u32\)\.wrapping_(add|sub)\((0x[0-9a-fA-F]+|\d+)\)\)", e):
            y = regno(re.search(REG, e).group(1))
            A.write(d, A.val(y))
        elif re.fullmatch(rf"sx\(\(c\.g\[(\w+)\]\[0\] as u32\)\.wrapping_add\(\(c\.g\[(\w+)\]\[0\] as u32\)\)\)", e):
            ys = regs_in(e)
            A.write(d, add_vals(A.val(ys[0]), A.val(ys[1])))
        elif re.fullmatch(r"sx\(\(c\.g\[(\w+)\]\[0\] as u32\) (<<|>>) \d+\)", e):
            y = A.val(regno(re.search(REG, e).group(1)))
            if y[0] == "const":
                A.write(d, ("const",))
            elif y[0] == "in" and "<<" in e:
                A.write(d, ("idx", y[1]))
            else:
                A.write(d, ("unk",))
        elif re.fullmatch(rf"{REG} \| (0x[0-9a-fA-F]+|\d+)", e):
            A.write(d, A.val(regs_in(e)[0]))
        else:
            vs = [A.val(r) for r in regs_in(e)]
            if all(v[0] == "const" for v in vs):
                A.write(d, ("const",))
            elif not pure_expr(e):
                raise Skip(f"unhandled expression `{e}`")
            else:
                A.write(d, ("unk",))
        return
    m = re.match(r"c\.(\w+)\((.*)\);$", s)
    if m:
        op, args = m.group(1), m.group(2)
        if op in STORES:
            mem_addr(A, args)
            for r in regs_in(ADDR.sub("", args)):
                A.read(r)
            if op == "swc1":
                A.fread_(int(args.split(",")[0]))
            elif op == "sq":
                A.read(int(args.split(",")[0]))
            return
        if op in ("lwc1", "lq"):
            reg_or_f = int(args.split(",")[0])
            mem_addr(A, args)
            if op == "lwc1":
                A.fwritten.add(reg_or_f)
            else:
                A.write(reg_or_f, ("ld",))
            return
        if op == "mtc1":
            rt, fs = [int(x) for x in args.split(",")]
            A.read(rt)
            A.fwritten.add(fs)
            return
        if op == "mov_s":
            fd, fs = [int(x) for x in args.split(",")]
            A.fread_(fs)
            A.fwritten.add(fd)
            return
        if op == "mul_s":
            fd, fs, ft = [int(x) for x in args.split(",")]
            A.fread_(fs)
            A.fread_(ft)
            A.fwritten.add(fd)
            return
        if op == "mult":
            p, rd, rs, rt = [int(x) for x in args.split(",")]
            A.read(rs)
            A.read(rt)
            A.write(rd, ("unk",))
            A.uses_mult = True
            return
        raise Skip(f"unhandled op c.{op}")
    m = re.match(rf"if {REG} == (\d+) \{{ {REG} = 0u64; \}}$", s)
    if m:  # movz/movn style
        A.read(regno(m.group(1)))
        d = regno(m.group(3))
        A.read(d)
        A.write(d, ("unk",))
        return
    raise Skip(f"unhandled statement `{s[:60]}`")


# ---------------------------------------------------------------- emission

HELPERS = """    /// An edge-case 64-bit register value (0, 1, -1, small, random).
    #[allow(dead_code)]
    fn leaf_val(r: &mut Rng) -> u64 {
        match r.next_u32() % 8 {
            0 => 0,
            1 => 1,
            2 => u64::MAX,
            3 => (r.next_u32() % 64) as u64,
            4 => sx(r.next_u32()),
            5 => r.next_u32() as u64,
            6 => 0x8000_0000,
            _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        }
    }

    /// Fill `size` bytes at `start` with zero / small / random words, or, when
    /// `chase` is set, with pointers (16-byte aligned) into `ptr_lo..ptr_lo + 0x200`.
    #[allow(dead_code)]
    fn leaf_fill(c: &mut Cpu, r: &mut Rng, start: u32, size: u32, chase: bool, ptr_lo: u32) {
        for i in 0..size / 4 {
            let w = if chase {
                ptr_lo + 16 * (r.next_u32() % 32)
            } else {
                match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.next_u32() % 16,
                    _ => r.next_u32(),
                }
            };
            c.bus.write32(start + 4 * i, w);
        }
    }
"""


def snake(name):
    n = re.sub(r"[^0-9A-Za-z_]", "_", name)
    s = stage_wave.snake(n)
    s = re.sub(r"_+", "_", s).strip("_")
    if not s or s[0].isdigit():
        s = "f_" + s
    return s


def hexs(v, w=8):
    s = f"{v:0{w}x}"
    return "0x" + "_".join([s[: len(s) - 4], s[-4:]]) if len(s) > 4 else "0x" + s


def emit_fixture(a, A):
    """The `|c, seed| { ... }` body lines."""
    lines = []
    uses_r = bool(A.ptr or A.idx or A.reads or A.fread)
    lines.append(f"let {'mut r' if uses_r else '_r'} = machine(c, seed, {hexs(a & 0xffff, 4) if False else hex(a)}, 0);")
    cursor = 0x0080_2000
    pr = sorted(A.ptr)
    for r in pr:
        neg = (-A.lo + 15) & ~15 if A.lo < 0 else 0
        size = neg + 512 + max(A.hi, 0) + 0x1000
        size = (size + 0xfff) & ~0xfff
        start = cursor
        cursor += size + 0x1000
        base = start + neg
        lines.append(f"leaf_fill(c, &mut r, {hexs(start)}, {hex(size)}, {'true' if A.chase else 'false'}, {hexs(base)});")
        lines.append(f"c.g[{NAMES[r]}][0] = sx({hexs(base)} + 16 * (r.next_u32() % 32));")
    for r in sorted(A.idx - A.ptr):
        lines.append(f"c.g[{NAMES[r]}][0] = (r.next_u32() % 8 * 4) as u64;")
    for r in A.reads:
        if r in A.ptr or r in A.idx or r == 0:
            continue
        lines.append(f"c.g[{NAMES[r]}][0] = leaf_val(&mut r);")
    for f in A.fread:
        lines.append(f"c.f[{f}] = r.edge_f32();")
    lines.append("c.g[RA][0] = sx(0x0040_0000 + 8 * seed);")
    return lines


def doc_summary(A, st):
    nl = sum(1 for s in st if re.search(r"= c\.l\w+\(|c\.l(q|wc1)\(", s))
    ns = sum(1 for s in st if re.match(r"c\.s(w|d|b|h|q|wc1)\(", s))
    parts = []
    if nl:
        parts.append(f"{nl} load(s)")
    if ns:
        parts.append(f"{ns} store(s)")
    return "straight-line leaf" + (": " + ", ".join(parts) if parts else "")


def build_function(name, a, shard, st, ret_at, late_ra, A):
    out = [f"/// `{name}` ({hexs(a)}): {doc_summary(A, st)}. Mechanical port of the generated body.", f"pub fn {{fn}}(c: &mut Cpu) {{"]
    for i, s in enumerate(st):
        if i == ret_at:
            if late_ra:
                out.append("    let t = c.g[RA][0] as u32;")
            continue
        if i == len(st) - 1:
            out.append("    c.pc = t;" if late_ra else "    ret(c);")
            continue
        out.append("    " + s)
    out.append("}")
    return out


def main():
    args = sys.argv[1:]
    listf = os.path.join(R, "target/gate/noee/auto-leaf.txt")
    wave, first, per = 276, 1501, 45
    i = 0
    while i < len(args):
        if args[i] == "--wave":
            wave = int(args[i + 1]); i += 2
        elif args[i] == "--first-oc":
            first = int(args[i + 1]); i += 2
        elif args[i] == "--per":
            per = int(args[i + 1]); i += 2
        else:
            listf = args[i]; i += 1
    global FIRST_OC
    FIRST_OC = first
    sym = symbols()
    names = fn_names()
    already = registered_addrs()
    taken = crate_fn_names()
    used = set()
    ported, skipped = [], []
    for l in open(listf):
        if not l.startswith("0x"):
            continue
        a = int(l.split("\t")[0], 16)
        if a in already:
            skipped.append((a, "already has a hand-written OVERRIDES entry"))
            continue
        shard = sym.get(a)
        if not shard:
            skipped.append((a, "not in ee-symbols"))
            continue
        lines = body_of(shard, a)
        if lines is None:
            skipped.append((a, "generated body not found/ambiguous"))
            continue
        try:
            st, ret_at, A, late = parse_body(lines)
        except Skip as e:
            skipped.append((a, str(e)))
            continue
        nm = names.get(a) or ALIASES.get(a)
        orig = re.sub(r"\W", "_", nm) if nm else f"leaf_{a:08x}"
        fn = snake(orig) if nm else f"leaf_{a:08x}"
        if fn in RESERVED or fn in taken or fn in used:
            fn = f"{fn}_{a:08x}"
        used.add(fn)
        ported.append(dict(a=a, shard=shard, orig=orig, fn=fn, st=st, ret_at=ret_at, A=A, late=late))

    # groups
    groups = [ported[i : i + per] for i in range(0, len(ported), per)]
    wdir = os.path.join(R, f"target/gate/wave{wave}")
    os.makedirs(wdir, exist_ok=True)
    mods = []
    cargo = open(stage_wave.CARGO).read()
    for k, g in enumerate(groups):
        m = f"oc{first + k}"
        mods.append(m)
        path = os.path.join(SRC, f"{m}.rs")
        text = stage_wave.SKELETON.format(m=m, wave=f"wave{wave}")
        text = text.replace(
            "pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];",
            "pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[\n"
            + "".join(f'    ({hexs(p["a"])}, {p["fn"]}, "{p["orig"]}"),\n' for p in g)
            + "];\n\n"
            + "\n\n".join("\n".join(build_function(p["orig"], p["a"], p["shard"], p["st"], p["ret_at"], p["late"], p["A"])).replace("{fn}", p["fn"]) for p in g),
        )
        tests = ["#[cfg(test)]\nmod tests {", "    use super::*;", "    use crate::difftest::fakes::machine;",
                 "    use crate::difftest::{check, check_lohi, Rng};", "", HELPERS]
        for p in g:
            A = p["A"]
            chk = "check_lohi" if A.uses_mult else "check"
            tests.append(f"    /// `{p['orig']}` ({hexs(p['a'])}) against the generated body.")
            tests.append("    #[test]")
            tests.append(f"    fn {p['fn']}_matches() {{")
            tests.append(f"        {chk}(ee::{p['shard']}::f_{p['a']:08x}__generated, {p['fn']}, 120, &|c, seed| {{")
            tests += ["            " + x for x in emit_fixture(p["a"], A)]
            tests.append("        });")
            tests.append("    }\n")
        tests.append("}")
        text = text[: text.index("#[cfg(test)]")] + "\n".join(tests) + "\n"
        open(path, "w").write(text)
        subprocess.run(["rustfmt", "--edition", "2021", path], check=False)
        with open(os.path.join(wdir, f"list-{m}.txt"), "w") as f:
            f.write(f"Auto-ported leaf functions ({len(g)}) in crates/ssx3-game/src/{m}.rs (tools/auto_port.py):\n")
            for p in g:
                f.write(f"- {p['a']:#010x} {p['orig']}\n")
        if f"{m} = []\n" not in cargo:
            stage_wave.wire([m])
            cargo = open(stage_wave.CARGO).read()
    with open(os.path.join(wdir, "models.tsv"), "w") as f:
        for m in mods:
            f.write(f"{m}\tnone\tauto-leaf\n")
    wave_all = os.path.join(wdir, f"wave{wave}-all.txt")
    with open(wave_all, "w") as f:
        for p in ported:
            f.write(f"0x{p['a']:08x}\t{p['orig']}\n")
    os.makedirs(os.path.join(R, "target/gate/waves"), exist_ok=True)
    subprocess.run(["cp", wave_all, os.path.join(R, "target/gate/waves/")])
    with open(os.path.join(wdir, "skipped.tsv"), "w") as f:
        for a, why in skipped:
            f.write(f"0x{a:08x}\t{why}\n")
    with open(os.path.join(wdir, "heuristic.tsv"), "w") as f:
        for p in ported:
            if p["A"].heur or p["A"].chase:
                f.write(f"0x{p['a']:08x}\t{p['orig']}\t{'sum-of-inputs' if p['A'].heur else ''} {'pointer-chasing' if p['A'].chase else ''}\n")
    for m, g in zip(mods, groups):
        print(f"{m}: {len(g)} function(s)")
    print(f"{len(ported)} ported, {len(skipped)} skipped (see {wdir}/skipped.tsv)")


if __name__ == "__main__":
    main()
