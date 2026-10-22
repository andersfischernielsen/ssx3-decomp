#!/usr/bin/env python3
"""Move the hand-written replacements out of the batch modules (`ocN.rs`)
into the C++ source tree layout (`crates/ssx3-game/src/game/<dir>/<unit>.rs`,
the matching decompilation's `src/<dir>/<unit>.cpp`, `tools/refsrc.py`).

    tools/reorg_modules.py plan  oc10 oc11 ...   # print where each function goes
    tools/reorg_modules.py apply oc10 oc11 ...   # move them (rewrites game/)

Each replacement (an `OVERRIDES` entry) goes to its function's unit; its
differential tests follow it. Helpers (functions, constants, `mod callee`
entries, test fixtures) go with the replacements that use them: copied into
each unit that needs them, identical copies merged, a name clash with a
different body renamed (`name_2`). Functions without an `OVERRIDES` entry
(parked ports) go to the unit of the address in their doc comment, else
`game/parked.rs`. Functions the reference does not know are placed by name
prefix (the unit most functions of that prefix are in), else `game/misc.rs`.

`apply` writes the unit files, `game/mod.rs` and the per-directory `mod.rs`
(with `overrides()`), and deletes the moved `ocN.rs`; `lib.rs`, `Cargo.toml`
and `overrides.txt` comments are updated separately (see the plan in
docs/plans/port-architecture.md, task 2).
"""
import collections
import json
import os
import re
import subprocess
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'crates/ssx3-game/src')
GAME = os.path.join(SRC, 'game')
CACHE = os.path.join(R, 'tools/.reorg-units.json')

# --------------------------------------------------------------------------
# Lexing Rust into top-level items.


def skip_ws_comments(s, i):
    n = len(s)
    while i < n:
        if s[i].isspace():
            i += 1
        elif s.startswith('//', i):
            j = s.find('\n', i)
            i = n if j < 0 else j + 1
        elif s.startswith('/*', i):
            i = skip_block_comment(s, i)
        else:
            break
    return i


def skip_block_comment(s, i):
    depth = 0
    n = len(s)
    while i < n:
        if s.startswith('/*', i):
            depth += 1
            i += 2
        elif s.startswith('*/', i):
            depth -= 1
            i += 2
            if depth == 0:
                return i
        else:
            i += 1
    return n


def skip_string(s, i):
    """s[i] is the opening quote (or 'r'/'b' prefix handled by caller)."""
    n = len(s)
    i += 1
    while i < n:
        if s[i] == '\\':
            i += 2
        elif s[i] == '"':
            return i + 1
        else:
            i += 1
    return n


def skip_raw_string(s, i):
    # r#*" ... "#*
    j = i + 1
    hashes = 0
    while s[j] == '#':
        hashes += 1
        j += 1
    assert s[j] == '"'
    end = '"' + '#' * hashes
    k = s.find(end, j + 1)
    return len(s) if k < 0 else k + len(end)


def char_literal_end(s, i):
    """s[i] == "'": a char literal's end, or None (a lifetime)."""
    if s.startswith("'\\", i):
        j = s.find("'", i + 2)
        # '\'' and '\\'
        if s[i + 2] in "'\\" and s[i + 3] == "'":
            return i + 4
        return j + 1
    if i + 2 < len(s) and s[i + 2] == "'":
        return i + 3
    # multibyte char
    m = re.match(r"'[^'\\\n]'", s[i:i + 6])
    if m:
        return i + len(m.group(0))
    return None


def scan(s, i, stop_at_depth0):
    """Advance from i over code; returns the index after the item's end:
    a ';' at depth 0, or the '}' closing depth back to 0 (plus a following
    ';')."""
    n = len(s)
    depth = 0
    while i < n:
        c = s[i]
        if s.startswith('//', i):
            j = s.find('\n', i)
            i = n if j < 0 else j + 1
            continue
        if s.startswith('/*', i):
            i = skip_block_comment(s, i)
            continue
        if c == '"':
            i = skip_string(s, i)
            continue
        if c in 'rb' and (i == 0 or not (s[i - 1].isalnum() or s[i - 1] == '_')):
            if re.match(r'(br|r)#*"', s[i:i + 40]):
                i = skip_raw_string(s, i + (1 if c == 'b' else 0))
                continue
            if s.startswith('b"', i):
                i = skip_string(s, i + 1)
                continue
            if s.startswith("b'", i):
                e = char_literal_end(s, i + 1)
                if e:
                    i = e
                    continue
        if c == "'":
            e = char_literal_end(s, i)
            if e:
                i = e
                continue
            i += 1
            continue
        if c in '{[(':
            depth += 1
        elif c in '}])':
            depth -= 1
            if depth == 0 and c == ']' and not stop_at_depth0:
                return i + 1
            if depth == 0 and c == '}':
                j = skip_ws_comments(s, i + 1)
                if j < n and s[j] == ';':
                    return j + 1
                return i + 1
        elif c == ';' and depth == 0:
            return i + 1
        i += 1
    return n


ITEM_RE = re.compile(
    r'^(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+|const\s+(?=fn)|extern\s+"C"\s+)*'
    r'(fn|const|static|mod|struct|enum|impl|type|use|trait|macro_rules!)\s*'
    r'(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)?')


def split_items(s):
    """[(prefix_comments_attrs, body_text)] for each top-level item, and the
    module's leading `//!` doc."""
    items = []
    i = 0
    n = len(s)
    header = []
    # Leading inner docs.
    while True:
        j = skip_ws(s, i)
        if s.startswith('//!', j):
            k = s.find('\n', j)
            header.append(s[j:k + 1])
            i = k + 1
        elif s.startswith('#![', j):
            k = scan(s, j + 2, False)
            header.append(s[j:k] + '\n')
            i = k
        else:
            break
    while True:
        start = i
        # Comments and attributes belong to the next item.
        while True:
            j = skip_ws(s, i)
            if s.startswith('//', j):
                k = s.find('\n', j)
                i = n if k < 0 else k + 1
            elif s.startswith('/*', j):
                i = skip_block_comment(s, j)
            elif s.startswith('#[', j):
                i = scan(s, j + 1, False)
            else:
                i = j
                break
        if i >= n:
            break
        end = scan(s, i, True)
        items.append((s[start:i], s[i:end]))
        i = end
    return ''.join(header), items


def skip_ws(s, i):
    while i < len(s) and s[i].isspace():
        i += 1
    return i


def strip_code(s):
    """The text without comments and string/char literals (for scanning
    identifiers)."""
    out = []
    i = 0
    n = len(s)
    while i < n:
        c = s[i]
        if s.startswith('//', i):
            j = s.find('\n', i)
            i = n if j < 0 else j
            continue
        if s.startswith('/*', i):
            i = skip_block_comment(s, i)
            continue
        if c == '"':
            i = skip_string(s, i)
            out.append('""')
            continue
        if c == "'":
            e = char_literal_end(s, i)
            if e:
                i = e
                out.append("' '")
                continue
        out.append(c)
        i += 1
    return ''.join(out)


IDENT = re.compile(r'[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)?')


class Item:
    def __init__(self, pre, body, scope=''):
        self.pre = pre
        self.body = body
        self.scope = scope  # '' top level, 'callee' etc. inner mod, 'tests'
        m = ITEM_RE.match(body.lstrip())
        self.kind = m.group(1) if m else '?'
        self.name = m.group(2) if m and m.group(2) else None
        if self.kind == 'impl':
            m2 = re.match(r'(?:unsafe\s+)?impl(?:<[^>]*>)?\s+(?:[A-Za-z_:<>]+\s+for\s+)?([A-Za-z_][A-Za-z0-9_]*)', body.lstrip())
            self.name = ('impl', m2.group(1) if m2 else None)
        code = strip_code(pre + body)
        self.refs = set()
        for t in IDENT.findall(code):
            self.refs.add(t)
            if '::' in t:
                self.refs.update(t.split('::'))
        # Method calls (`c.name(`) can reach trait methods.
        self.refs.update(re.findall(r'\.([A-Za-z_][A-Za-z0-9_]*)\s*\(', code))
        # Extra names this item defines: a trait's methods, the functions a
        # macro invocation expands to (`name!{ fn_name, ... }` lists them).
        self.defs = set()
        if self.kind == 'trait':
            self.defs = set(re.findall(r'\bfn\s+([A-Za-z_][A-Za-z0-9_]*)', body))
        if self.kind == 'impl':
            m3 = re.match(r'(?:unsafe\s+)?impl(?:<[^>]*>)?\s+([A-Za-z_][A-Za-z0-9_]*)\s+for\s+', body.lstrip())
            if m3:
                self.refs.add(m3.group(1))
        if self.kind == '?':
            mm = re.match(r'([A-Za-z_][A-Za-z0-9_]*)!', body.lstrip())
            if mm:
                self.kind = 'macro_call'
                self.defs = set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*', strip_code(body)))
        self.targets = set()

    def text(self):
        return self.pre + self.body


def parse_module(path):
    s = open(path).read()
    header, raw = split_items(s)
    main, inner_mods, tests = [], {}, None
    for pre, body in raw:
        it = Item(pre, body)
        if it.kind == 'mod' and '{' in body:
            inner_pre_body = body[body.index('{') + 1: body.rindex('}')]
            _, sub = split_items(inner_pre_body)
            subitems = [Item(p, b, it.name) for p, b in sub]
            if it.name == 'tests' or 'cfg(test)' in pre:
                tests = (it, subitems)
            else:
                inner_mods[it.name] = (it, subitems)
        else:
            main.append(it)
    return header, main, inner_mods, tests


OV_RE = re.compile(r'\(\s*0x([0-9a-fA-F_]+)\s*,\s*([A-Za-z0-9_:]+)\s*,\s*"([^"]+)",?\s*\)')


def overrides_of(main):
    for it in main:
        if it.kind == 'const' and it.name == 'OVERRIDES':
            return [(int(a.replace('_', ''), 16), f, n) for a, f, n in OV_RE.findall(it.body)], it
    return [], None


# --------------------------------------------------------------------------
# Units.


def unit_of_addrs(addrs):
    cache = json.load(open(CACHE)) if os.path.exists(CACHE) else {}
    need = [a for a in addrs if f'{a:08x}' not in cache]
    if need:
        tsv = os.path.join(R, 'tools/.reorg-q.tsv')
        open(tsv, 'w').write(''.join(f'0x{a:08x}\n' for a in need))
        out = subprocess.run([sys.executable, os.path.join(R, 'tools/refsrc.py'), '--tsv', tsv],
                             capture_output=True, text=True).stdout
        os.remove(tsv)
        for line in out.splitlines():
            parts = line.split('\t')
            if len(parts) >= 3 and parts[0].startswith('0x'):
                m = re.match(r'src/(.+)\.(?:cpp|c|cc|h)(?::|$)', parts[2])
                cache[f'{int(parts[0], 16):08x}'] = [m.group(1) if m else None, parts[1]]
        json.dump(cache, open(CACHE, 'w'), indent=0, sort_keys=True)
    return {a: cache.get(f'{a:08x}', [None, None]) for a in addrs}


def modname(seg):
    s = re.sub(r'[^A-Za-z0-9_]', '_', seg).lower()
    if s[0].isdigit() or s in RESERVED:
        s = 'u_' + s
    return s


RESERVED = {'mod', 'fn', 'type', 'impl', 'use', 'self', 'super', 'crate', 'match', 'move', 'ref',
            'static', 'struct', 'trait', 'loop', 'while', 'for', 'if', 'else', 'let', 'in',
            'box', 'async', 'await', 'dyn', 'abstract', 'final', 'macro', 'override', 'priv',
            'typeof', 'unsized', 'virtual', 'yield', 'try', 'core', 'std', 'tests'}


def unit_path(unit):
    """'fe/fepopup' -> ('fe', 'fepopup') as module names."""
    return tuple(modname(p) for p in unit.split('/'))


# Functions the reference has no unit for (libraries, unnamed units): by
# name prefix.
PREFIX_UNITS = {
    'snd': 'lib/snd', 'sndenc': 'lib/snd', 'sndi': 'lib/snd', 'sndstrm': 'lib/snd',
    'spch': 'lib/snd', 'sndbank': 'lib/snd',
    'addintchandler': 'lib/kernel', 'adddmachandler': 'lib/kernel',
    'date': 'lib/libc', 'time': 'lib/libc',
    'twpigd': 'worldpainter/wpigd', 'cworldpainterman': 'worldpainter/worldpainterman',
    'wpfile': 'worldpainter/wpfile',
    'rideranim': 'animation/rideranim', 'attachedemitterfx': 'visualfx/attachedemitterfx',
    'aianimeventmap': 'ai/aianimeventmap', 'focusman': 'ui/focusman',
    'csplashmanager': 'fe/splashmanager', 'dangerman': 'ai/dangerman',
    'racestategameinit': 'main/racestate', 'racestatefreeride': 'main/racestate',
    'racestatecountdown': 'main/racestate', 'trick': 'ai/trick', 'chudfx': 'visualfx/hudfx',
    'dnasauth': 'dirtysock/dnasauth', 'cgamecomm': 'dirtysock/gamecomm',
    'morton': 'render/morton', 'render2d': 'render/render2d', 'shps': 'render/shps',
}


KERNEL = {'LoadExecPS2', 'EnableIntc', 'DisableIntc', 'EnableDmac', 'DisableDmac', 'StartThread',
          'WakeupThread', 'SuspendThread', 'ExecOSD', 'InitTLB', 'SyncDCache', 'InvalidDCache',
          'GetThreadId', 'SignalSema', 'WaitSema', 'PollSema', 'FlushCache', 'Exit'}
PREFIX_UNITS.update({'asyncfile': 'fe/feasyncfile', 'eafile': 'lib/eafile', 'crider': 'ai/rider',
                     'lunovm': 'luno/lunovm'})


def prefix(name):
    n = name or ''
    m = re.match(r'([A-Za-z0-9]+?)(?:_|::|$)', n)
    return m.group(1).lower() if m else n.lower()


# --------------------------------------------------------------------------


def plan(mods):
    parsed = {}
    allov = []
    for m in mods:
        parsed[m] = parse_module(os.path.join(SRC, m + '.rs'))
        ovs, _ = overrides_of(parsed[m][1])
        allov += [(m, a, f, n) for a, f, n in ovs]
    # Parked functions: an address in the doc comment.
    parked = {}
    for m, (_, main, _, _) in parsed.items():
        ovnames = {f for (mm, a, f, n) in allov if mm == m}
        for it in main:
            if it.kind == 'fn' and it.body.lstrip().startswith('pub') and it.name not in ovnames:
                a = re.search(r'\(`?0x([0-9a-fA-F_]{6,9})`?\)', it.pre)
                if a:
                    parked[(m, it.name)] = int(a.group(1).replace('_', ''), 16)
    units = unit_of_addrs(sorted({a for (_, a, _, _) in allov} | set(parked.values())))
    # Prefix vote for the unknown.
    votes = collections.defaultdict(collections.Counter)
    for (_, a, f, n) in allov:
        u = units[a][0]
        if u:
            votes[prefix(n)][u] += 1
    def place(a, name):
        u = units.get(a, [None])[0]
        if u:
            return unit_path(u)
        v = votes.get(prefix(name))
        if v:
            return unit_path(v.most_common(1)[0][0])
        n = name or ''
        if n.startswith('__'):
            return unit_path('lib/gcc')
        if n.lstrip('_i') in KERNEL or n.lstrip('_') in KERNEL:
            return unit_path('lib/kernel')
        if n.startswith(('sceSif', 'isceSif')):
            return unit_path('lib/sif')
        if n.startswith('_') or n in ('vfprintf',):
            return unit_path('lib/libc')
        if prefix(name) in PREFIX_UNITS:
            return unit_path(PREFIX_UNITS[prefix(name)])
        return ('misc',)
    target = {}
    for (m, a, f, n) in allov:
        target[(m, f)] = place(a, n)
    for (m, f), a in parked.items():
        target[(m, f)] = place(a, f)
    return parsed, allov, target


def assign(parsed, target):
    """Per module: item -> set of unit paths."""
    out = {}
    for m, (header, main, inner, tests) in parsed.items():
        byname = {}
        for it in main:
            if it.name and not isinstance(it.name, tuple):
                byname.setdefault(it.name, []).append(it)
            if it.kind == 'trait':
                for d in it.defs:
                    byname.setdefault(d, []).append(it)
        if any(it.name == 'OVERRIDES' for it in main):
            pass
        globbed = {mn for mn in inner for it in main
                   if it.kind == 'use' and re.search(r'\buse\s+(?:self::)?' + mn + r'::', it.body)}
        for mname, (mit, subs) in inner.items():
            for it in subs:
                if it.name:
                    byname.setdefault(f'{mname}::{it.name}', []).append(it)
                    if mname in globbed:
                        byname.setdefault(it.name, []).append(it)
        roots = []
        for it in main:
            t = target.get((m, it.name))
            if it.kind == 'fn' and t:
                it.targets.add(t)
                roots.append(it)
            if it.kind == 'macro_call':
                for d in it.defs:
                    t = target.get((m, d))
                    if t:
                        it.targets.add(t)
                        byname.setdefault(d, []).append(it)
        # Propagate to helpers: referenced (transitively) by an item with
        # targets gets them.
        def refs_of(it):
            for r in it.refs:
                if r in inner:
                    for x in inner[r][1]:
                        yield x
                for cand in (r, ) + ((r.split('::')[-1],) if '::' in r and r.split('::')[0] in ('self', 'super') else ()):
                    for x in byname.get(cand, []):
                        if x is not it:
                            yield x
        changed = True
        while changed:
            changed = False
            for it in list(main) + [s for (_, ss) in inner.values() for s in ss]:
                if not it.targets:
                    continue
                for x in refs_of(it):
                    # A replacement stays in its own unit (others import it).
                    if x.kind == 'fn' and target.get((m, x.name)):
                        continue
                    if not it.targets <= x.targets:
                        x.targets |= it.targets
                        changed = True
            # impl blocks follow their type or trait (and pull in what they use).
            for it in main:
                if it.kind == 'impl':
                    for r in [it.name[1]] + [r for r in it.refs if byname.get(r) and any(x.kind == 'trait' for x in byname[r])]:
                        for x in byname.get(r, []):
                            if not x.targets <= it.targets:
                                it.targets |= x.targets
                                changed = True
        # Tests.
        if tests:
            tit, subs = tests
            tbyname = {s.name: s for s in subs if s.name}
            for s_ in subs:
                if s_.kind == 'macro_call':
                    # `name!(defined_fn, ...)`: the first argument.
                    mm_ = re.match(r'[A-Za-z_][A-Za-z0-9_]*!\s*[({\[]\s*([A-Za-z_][A-Za-z0-9_]*)', s_.body.lstrip())
                    if mm_:
                        tbyname.setdefault(mm_.group(1), s_)
            for s in subs:
                if s.kind == 'fn' and '#[test]' in s.pre:
                    for r in s.refs:
                        nm = r.split('::')[-1]
                        for x in byname.get(nm, []):
                            if (x.kind == 'fn' and target.get((m, x.name))) or (x.kind == 'macro_call' and x.targets):
                                s.targets |= x.targets
                    if len(s.targets) > 1:
                        # A test of several functions: the first one's unit.
                        first = None
                        for r in re.findall(r'[A-Za-z_][A-Za-z0-9_]*', strip_code(s.body)):
                            for x in byname.get(r, []):
                                if x.kind == 'fn' and target.get((m, x.name)):
                                    first = target[(m, x.name)]
                                    break
                            if first:
                                break
                        s.targets = {first} if first else {sorted(s.targets)[0]}
            changed = True
            while changed:
                changed = False
                for s in subs:
                    if not s.targets:
                        continue
                    for r in s.refs:
                        nm = r.split('::')[-1] if r.startswith(('self::', 'super::')) else r
                        x = tbyname.get(nm)
                        if x is not None and x is not s and not s.targets <= x.targets:
                            x.targets |= s.targets
                            changed = True
                        if nm in inner:
                            for x in inner[nm][1]:
                                if not s.targets <= x.targets:
                                    x.targets |= s.targets
                                    changed = True
                        # Test fixtures may use main helpers too.
                        for x in byname.get(nm, []):
                            if x.kind == 'fn' and target.get((m, x.name)):
                                continue
                            if not s.targets <= x.targets:
                                x.targets |= s.targets
                                changed = True
        # Left over (unused helpers, tests of them): the module's main unit.
        cnt = collections.Counter(t for it in main for t in it.targets)
        if cnt:
            major = cnt.most_common(1)[0][0]
            for it in main:
                if not it.targets and it.kind != 'use' and it.name != 'OVERRIDES':
                    it.targets.add(major)
            for mname, (mit, subs) in inner.items():
                for x in subs:
                    if not x.targets and x.kind != 'use':
                        x.targets.add(major)
            if tests:
                for x in tests[1]:
                    if not x.targets and x.kind != 'use':
                        x.targets.add(major)
                # Fixtures the left-over tests use.
                tbyname = {x.name: x for x in tests[1] if x.name}
                changed = True
                while changed:
                    changed = False
                    for x in tests[1]:
                        for r in x.refs:
                            y = tbyname.get(r)
                            if y is not None and not x.targets <= y.targets:
                                y.targets |= x.targets
                                changed = True
        out[m] = (header, main, inner, tests)
    return out


def rename_tokens(text, ren):
    """Rename identifiers (not method calls `.x(`, not paths `a::x` unless
    the key itself is a path)."""
    if not ren:
        return text
    plain = [k for k in ren if '::' not in k]
    qual = [k for k in ren if '::' in k]
    alts = []
    if qual:
        alts.append(r'(?<![A-Za-z0-9_:.])(?:' + '|'.join(re.escape(k) for k in sorted(qual, key=len, reverse=True)) + r')')
    if plain:
        alts.append(r'(?<![A-Za-z0-9_.:])(?:' + '|'.join(re.escape(k) for k in sorted(plain, key=len, reverse=True)) + r')')
    rx = re.compile('(' + '|'.join(alts) + r')(?![A-Za-z0-9_])')
    # Do not touch string literals: split on them.
    parts = re.split(r'("(?:[^"\\]|\\.)*")', text)
    return ''.join(p if i % 2 else rx.sub(lambda m: ren[m.group(1)], p) for i, p in enumerate(parts))


def build(parsed, allov, target, seed=None):
    units = collections.defaultdict(lambda: {'uses': [], 'items': [], 'names': {}, 'inner': collections.defaultdict(lambda: {'uses': [], 'items': [], 'names': {}}),
                                              'tuses': [], 'titems': [], 'tnames': {}, 'ovs': [], 'docs': []})
    # Units that already exist: their names take part in the clash checks.
    for path, (main_, inner_, tests_) in (seed or {}).items():
        U = units[path]
        for it in main_:
            if isinstance(it.name, str) and it.kind not in ('use', 'impl'):
                U['names'][it.name] = ('<existing>', norm(it.text()))
        for mname, (_, subs) in inner_.items():
            for it in subs:
                if isinstance(it.name, str) and it.kind != 'use':
                    U['inner'][mname]['names'][it.name] = ('<existing>', norm(it.text()))
        for it in (tests_[1] if tests_ else []):
            if isinstance(it.name, str) and it.kind != 'use':
                U['tnames'][it.name] = ('<existing>', norm(it.text()))
    unassigned = []
    # Names each target imports (from any contributing module): a local item
    # with such a name is renamed, so both stay usable.
    imported = collections.defaultdict(set)
    timported = collections.defaultdict(set)
    for m, (header, main, inner, tests) in parsed.items():
        names = set()
        for it in main:
            if it.kind == 'use':
                out = []
                parse_use_tree(re.sub(r'^(pub(\([^)]*\))?\s+)?use\s+', '', it.body.strip()).rstrip(';'), [], out)
                names |= {n for p, a, n in out if n and not p.endswith('*')}
        for it in main:
            for t in it.targets:
                imported[t] |= names
        if tests:
            tnames_ = set()
            for it in tests[1]:
                if it.kind == 'use':
                    out = []
                    parse_use_tree(re.sub(r'^(pub(\([^)]*\))?\s+)?use\s+', '', it.body.strip()).rstrip(';'), [], out)
                    tnames_ |= {n for p, a, n in out if n and not p.endswith('*') and not p.startswith('super')}
            for it in tests[1]:
                for t in it.targets:
                    timported[t] |= tnames_
    trait_methods = collections.defaultdict(set)  # target -> methods already defined
    final_name = {}
    pending = []
    # Items several units need live once, in their module's main unit
    # (`pub(crate)`); the other units import them.
    home = {}
    for m, (header, main, inner, tests) in parsed.items():
        cnt = collections.Counter(t for it in main if it.kind == 'fn' for t in it.targets)
        major = cnt.most_common(1)[0][0] if cnt else None
        for it in main + [x for (_, ss) in inner.values() for x in ss] + (list(tests[1]) if tests else []):
            if len(it.targets) > 1:
                home[id(it)] = major if major in it.targets else sorted(it.targets)[0]
    shared_final = {}
    shared_pending = []
    # Trait methods: a module whose trait methods clash with an earlier
    # module's trait (both extend `Cpu`) gets its methods renamed everywhere.
    module_meth = {}
    seen_methods = set()
    for m, (header, main, inner, tests) in parsed.items():
        mm = {}
        for it in main:
            if it.kind == 'trait':
                for d in sorted(it.defs):
                    if d in seen_methods:
                        k = 2
                        while f'{d}_{k}' in seen_methods:
                            k += 1
                        mm[d] = f'{d}_{k}'
        for it in main:
            if it.kind == 'trait':
                seen_methods |= {mm.get(d, d) for d in it.defs}
        if mm:
            module_meth[m] = mm
    for m, (header, main, inner, tests) in parsed.items():
        ovs = {f: (a, n) for (mm, a, f, n) in allov if mm == m}
        alltargets = set()
        for it in main:
            alltargets |= it.targets
        pubfns = {it.name: it for it in main if it.kind == 'fn' and it.body.lstrip().startswith('pub')}
        for t in sorted(alltargets):
            U = units[t]
            ren = {}
            # First pass: names and clashes.
            def claim(names, it, key):
                nm = it.name
                if nm is None or isinstance(nm, tuple):
                    return
                if key == '' and nm in imported[t] and nm not in names:
                    names[nm] = ('<import>', None)
                if key == 'tests::' and nm in timported[t] and nm not in names:
                    names[nm] = ('<import>', None)
                prev = names.get(nm)
                if prev is None:
                    names[nm] = (m, norm(it.text()))
                    return
                if prev[1] == norm(it.text()):
                    ren.setdefault(key + nm, None)  # identical: drop
                    return
                k = 2
                while f'{nm}_{k}' in names:
                    k += 1
                names[f'{nm}_{k}'] = (m, norm(it.text()))
                ren[key + nm] = f'{nm}_{k}'
            for it in main:
                if t in it.targets and it.kind not in ('use', 'impl') and it.name != 'OVERRIDES':
                    claim(U['names'], it, '')
            for mname, (mit, subs) in inner.items():
                for it in subs:
                    if t in it.targets and it.kind != 'use':
                        claim(U['inner'][mname]['names'], it, mname + '::')
            if tests:
                for it in tests[1]:
                    if t in it.targets and it.kind != 'use':
                        claim(U['tnames'], it, 'tests::')
            drop = {k for k, v in ren.items() if v is None}
            top, qual, tst = {}, {}, {}
            inner_tok = collections.defaultdict(dict)
            for k, v in ren.items():
                if v is None:
                    continue
                if '::' in k:
                    mod, nm = k.split('::')
                    if mod == 'tests':
                        tst[nm] = v
                    else:
                        qual[f'{mod}::{nm}'] = f'{mod}::{v}'
                        inner_tok[mod][nm] = v
                else:
                    top[k] = v
            meth = module_meth.get(m, {})
            # A module that imports from its own inner modules (`use
            # callee::*`) names their items unqualified: rename those too.
            inner_imported = set()
            for it in main:
                if it.kind == 'use':
                    for mname in inner:
                        if re.search(r'\buse\s+(?:self::)?' + mname + r'::', it.body):
                            inner_imported.add(mname)
            for mname in inner_imported:
                for k_, v_ in inner_tok[mname].items():
                    top.setdefault(k_, v_)
            tok = top
            def emit(lst, it, key):
                if it.name and not isinstance(it.name, tuple) and (key + it.name) in drop:
                    return
                h = home.get(id(it))
                if h is not None and h != t:
                    if it.kind in ('impl', 'macro_call') or not isinstance(it.name, str):
                        return
                    in_t = ren.get(key + it.name) or it.name
                    shared_pending.append((t, key, m, id(it), it.name, in_t, '#[cfg(test)]' in it.pre))
                    return
                if key == '':
                    m_ = {**top, **qual}
                elif key == 'tests::':
                    m_ = {**top, **qual, **tst}
                else:
                    m_ = {**qual, **inner_tok[key[:-2]]}
                txt = it.text()
                if m_:
                    txt = rename_tokens(txt, m_)
                for a_, b_ in meth.items():
                    txt = re.sub(r'(\.\s*)' + a_ + r'(\s*\()', r'\g<1>' + b_ + r'\2', txt)
                    txt = re.sub(r'(\bfn\s+)' + a_ + r'\b', r'\g<1>' + b_, txt)
                if h is not None and isinstance(it.name, str):
                    shared_final[id(it)] = ren.get(key + it.name) or it.name
                if isinstance(it.name, str) and key != 'tests::' or (key == 'tests::' and it.kind != 'fn') or (key == 'tests::' and '#[test]' not in it.pre):
                    txt = make_shared(txt, it)
                lst.append(txt)
            # Uses (all of the module's, deduplicated later).
            for it in main:
                if it.kind == 'use':
                    b_ = it.body.strip()
                    mm_ = re.match(r'(?:pub(?:\([^)]*\))?\s+)?use\s+(?:self::)?([A-Za-z_][A-Za-z0-9_]*)::', b_)
                    if mm_ and mm_.group(1) in inner:
                        # Its own inner module: all of it (renamed items included).
                        b_ = f'use {mm_.group(1)}::*;'
                    U['uses'].append(b_)
            for it in main:
                if t in it.targets and it.kind != 'use' and it.name != 'OVERRIDES':
                    emit(U['items'], it, '')
            for mname, (mit, subs) in inner.items():
                if any(t in s.targets for s in subs):
                    I = U['inner'][mname]
                    I['pre'] = mit.pre
                    for s in subs:
                        if s.kind == 'use':
                            I['uses'].append(s.body.strip())
                        elif t in s.targets:
                            emit(I['items'], s, mname + '::')
            if tests:
                tit, subs = tests
                if any(t in s.targets for s in subs if s.kind == 'fn'):
                    for s in subs:
                        if s.kind == 'use':
                            U['tuses'].append(s.body.strip())
                        elif t in s.targets:
                            emit(U['titems'], s, 'tests::')
            for f, (a, n) in ovs.items():
                it = next((x for x in main if x.name == f), None)
                if it is None:
                    # Defined by a macro invocation.
                    it = next((x for x in main if x.kind == 'macro_call' and f in x.defs and target.get((m, f)) in x.targets), None)
                    if it is not None and target.get((m, f)) != t:
                        continue
                if it is not None and t in it.targets:
                    U['ovs'].append((a, tok.get(f, f), n))
                    final_name[(m, f, t)] = tok.get(f, f)
            # Public functions of this module placed in other units that the
            # items placed here call: import them.
            here = {it.name for it in main if t in it.targets and isinstance(it.name, str)}
            for it in main + (list(tests[1]) if tests else []):
                if t not in it.targets:
                    continue
                for r in it.refs:
                    if r in here or r not in pubfns:
                        continue
                    x = pubfns[r]
                    if t in x.targets or not x.targets:
                        continue
                    other = sorted(x.targets)[0]
                    pending.append((t, it.scope == 'tests', m, r, other))
        for it in main:
            if not it.targets and it.kind not in ('use',) and it.name != 'OVERRIDES':
                unassigned.append((m, it.kind, it.name))
        if tests:
            for s in tests[1]:
                if not s.targets and s.kind == 'fn':
                    unassigned.append((m, 'test', s.name))
    for t, key, m, iid, name, in_t, test_only in shared_pending:
        h = home[iid]
        fin = shared_final.get(iid, name)
        alias = f' as {in_t}' if in_t != fin else ''
        hp = '::'.join(h)
        if key == '':
            units[t]['uses'].append(('#[cfg(test)]\n' if test_only else '') + f'pub(crate) use crate::game::{hp}::{fin}{alias};')
        elif key == 'tests::':
            units[t]['tuses'].append(f'pub(crate) use crate::game::{hp}::tests::{fin}{alias};')
        else:
            mod = key[:-2]
            units[t]['inner'][mod]['uses'].append(f'pub(crate) use crate::game::{hp}::{mod}::{fin}{alias};')
            units[t]['inner'][mod].setdefault('pre', '')
    for t, in_tests, m, f, other in pending:
        fn = final_name.get((m, f, other), f)
        line = f'pub(crate) use crate::game::{"::".join(other)}::{fn}' + (f' as {f}' if fn != f else '') + ';'
        (units[t]['tuses'] if in_tests else units[t]['uses']).append(line)
    return units, unassigned


def make_shared(txt, it):
    """`pub(crate)` for an item other units import."""
    items = split_items(txt)[1]
    if not items:
        return txt
    pre = items[0][0]
    body = txt[len(pre):]
    lead = body[:len(body) - len(body.lstrip())]
    b = body.lstrip()
    if it.kind == 'macro_rules!':
        return txt.rstrip() + f'\npub(crate) use {it.name};\n'
    if it.kind in ('impl', 'use', 'macro_call', '?'):
        return txt
    if b.startswith(('pub(super)', 'pub(self)', 'pub(in')):
        b = re.sub(r'^pub\((?:super|self|in [^)]*)\)\s*', '', b)
    elif b.startswith('pub'):
        return txt
    return pre + lead + 'pub(crate) ' + b


def norm(t):
    return re.sub(r'\s+', ' ', strip_code(t)).strip()


def render_unit(path, U):
    lines = [f'//! `src/{"/".join(path)}.cpp`: hand-written replacements (moved from the\n'
             f'//! batch modules by `tools/reorg_modules.py`).\n\n']
    lines += [u + '\n' for u in dedupe_uses(U['uses'], local_names(U['items']) | set(U['inner']))]
    lines.append('\n/// Replacements in this unit: (address, function, name).\n')
    lines.append('pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[\n')
    for a, f, n in sorted(U['ovs']):
        lines.append(f'    (0x{a >> 16:04x}_{a & 0xffff:04x}, {f}, "{n}"),\n')
    lines.append('];\n')
    for mname, I in U['inner'].items():
        if not I['items'] and not I['uses']:
            continue
        lines.append('\n' + I.get('pre', '').lstrip('\n'))
        lines.append(f'pub(crate) mod {mname} {{\n')
        lines += ['    ' + u + '\n' for u in dedupe_uses(I['uses'], local_names(I['items']))]
        for t in I['items']:
            lines.append(indent(t.rstrip() + '\n', '    '))
        lines.append('}\n')
    for t in U['items']:
        lines.append(t.rstrip() + '\n')
    if U['titems']:
        lines.append('\n#[cfg(test)]\npub(crate) mod tests {\n')
        lines += ['    ' + u + '\n' for u in dedupe_uses(U['tuses'], local_names(U['titems']))]
        for t in U['titems']:
            lines.append(indent(t.rstrip() + '\n', '    '))
        lines.append('}\n')
    return ''.join(lines)


def local_names(texts):
    names = set()
    for t in texts:
        for pre, body in split_items(t)[1]:
            m = ITEM_RE.match(body.lstrip())
            if m and m.group(2) and m.group(1) not in ('use', 'impl'):
                names.add(m.group(2))
            if m and m.group(1) == 'macro_rules!':
                pass
    return names


def indent(t, pre):
    # Items already carry their own indentation from the source module (4
    # spaces inside `mod`); keep them as they are.
    return t


def parse_use_tree(t, base, out):
    """`a::b::{c, d::{e}, f as g, self, *}` -> out: [(path, alias)]."""
    t = t.strip()
    if not t:
        return
    if '{' in t and (t.find('{') < (t.find(',') if ',' in t else 10**9) or ',' not in t):
        i = t.index('{')
        head = t[:i].rstrip(':').strip()
        inner = t[i + 1:t.rindex('}')]
        nb = base + [x for x in head.split('::') if x]
        depth = 0
        cur = ''
        for ch in inner:
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
            if ch == ',' and depth == 0:
                parse_use_tree(cur, nb, out)
                cur = ''
            else:
                cur += ch
        parse_use_tree(cur, nb, out)
        return
    alias = None
    m = re.match(r'(.*?)\s+as\s+([A-Za-z_][A-Za-z0-9_]*)$', t)
    if m:
        t, alias = m.group(1).strip(), m.group(2)
    parts = base + [x for x in t.split('::') if x]
    if parts and parts[-1] == 'self':
        parts = parts[:-1]
    name = alias or (parts[-1] if parts else None)
    out.append(('::'.join(parts), alias, name))


def dedupe_uses(uses, local=()):
    """Merge `use` items: one import per name (the first), none for a name
    the file defines itself; globs kept once."""
    leaves = []
    for u in uses:
        attr = ''
        if u.startswith('#['):
            attr, u = u.split('\n', 1)
            attr += '\n'
        body = re.sub(r'^(pub(\([^)]*\))?\s+)?use\s+', '', u.strip()).rstrip(';')
        mv = re.match(r'(pub(?:\([^)]*\))?\s+)', u.strip())
        vis = mv.group(1) if mv else ''
        out = []
        parse_use_tree(body, [], out)
        leaves += [(attr + vis, p, a, n) for p, a, n in out]
    seen_names = set(local)
    seen_globs = set()
    lines = []
    for vis, path, alias, name in leaves:
        if path.endswith('*'):
            if path not in seen_globs:
                seen_globs.add(path)
                lines.append(f'{vis}use {path};')
            continue
        if name in seen_names:
            continue
        seen_names.add(name)
        lines.append(f'{vis}use {path}{" as " + alias if alias else ""};')
    return lines


def unit_file(path):
    return os.path.join(GAME, *path[:-1], path[-1] + '.rs')


def apply(mods):
    parsed, allov, target = plan(mods)
    parsed = assign(parsed, target)
    seed = {}
    for t in set(target.values()):
        f = unit_file(t)
        if os.path.exists(f):
            _, main_, inner_, tests_ = parse_module(f)
            seed[t] = (main_, inner_, tests_)
    units, unassigned = build(parsed, allov, target, seed)
    os.makedirs(GAME, exist_ok=True)
    written = []
    for path, U in sorted(units.items()):
        f = unit_file(path)
        os.makedirs(os.path.dirname(f), exist_ok=True)
        if os.path.exists(f):
            merged = merge_unit(open(f).read(), U)
            open(f, 'w').write(merged)
        else:
            open(f, 'w').write(render_unit(path, U))
        written.append(path)
    write_mods(all_units())
    return written, unassigned


def all_units():
    out = []
    for d, _, files in os.walk(GAME):
        rel = os.path.relpath(d, GAME)
        segs = [] if rel == '.' else rel.split(os.sep)
        for f in files:
            if f.endswith('.rs') and f != 'mod.rs':
                out.append(tuple(segs + [f[:-3]]))
    return out


def merge_unit(text, U):
    """Insert a wave's items into an existing unit file."""
    _, raw = split_items(text)
    # Existing uses (to merge with) and positions.
    pos_last_use = 0
    existing_uses = []
    off = 0
    spans = []
    for pre, body in raw:
        start = text.index(body, off)
        end = start + len(body)
        spans.append((pre, body, start, end))
        off = end
    for pre, body, start, end in spans:
        if ITEM_RE.match(body.lstrip()) and ITEM_RE.match(body.lstrip()).group(1) == 'use':
            existing_uses.append(body.strip())
            pos_last_use = end
    local = local_names([b for _, b, _, _ in spans]) | local_names(U['items'])
    new_uses = [u for u in dedupe_uses(existing_uses + U['uses'], local) if u not in dedupe_uses(existing_uses, local)]
    out = text
    # Bottom-up edits keep earlier offsets valid.
    edits = []
    if new_uses:
        edits.append((pos_last_use, '\n' + '\n'.join(new_uses)))
    for pre, body, start, end in spans:
        b = body.lstrip()
        if b.startswith('pub const OVERRIDES') and U['ovs']:
            close = text.rindex(']', start, end)
            ent = ''.join(f'    (0x{a >> 16:04x}_{a & 0xffff:04x}, {f}, "{n}"),\n' for a, f, n in sorted(U['ovs']))
            edits.append((close, ent))
        m_ = re.match(r'(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{', b)
        if m_:
            mname = m_.group(1)
            close = text.rindex('}', start, end)
            if mname == 'tests' and (U['titems'] or U['tuses']):
                add = ''.join('    ' + u + '\n' for u in U['tuses']) + ''.join(t_.rstrip() + '\n' for t_ in U['titems'])
                edits.append((close, add))
                U['_tests_done'] = True
            elif mname in U['inner'] and (U['inner'][mname]['items'] or U['inner'][mname]['uses']):
                I = U['inner'][mname]
                add = ''.join('    ' + u + '\n' for u in I['uses']) + ''.join(t_.rstrip() + '\n' for t_ in I['items'])
                edits.append((close, add))
                I['_done'] = True
    # Items (and new inner modules) before the tests module, else at the end.
    tests_at = next((st for pre, b, st, en in spans if re.match(r'(?:pub(?:\([^)]*\))?\s+)?mod\s+tests\b', b.lstrip())), None)
    if tests_at is not None:
        tests_at = text.rindex('#[cfg(test)]', 0, tests_at) if '#[cfg(test)]' in text[:tests_at] else tests_at
    body_add = ''
    for mname, I in U['inner'].items():
        if (I['items'] or I['uses']) and not I.get('_done'):
            body_add += f'\npub(crate) mod {mname} {{\n' + ''.join('    ' + u + '\n' for u in dedupe_uses(I['uses'], local_names(I['items']))) + ''.join(t_.rstrip() + '\n' for t_ in I['items']) + '}\n'
    body_add += ''.join('\n' + t_.rstrip() + '\n' for t_ in U['items'])
    if body_add:
        edits.append((tests_at if tests_at is not None else len(text), body_add + '\n'))
    if (U['titems']) and not U.get('_tests_done'):
        edits.append((len(text), '\n#[cfg(test)]\npub(crate) mod tests {\n' + ''.join('    ' + u + '\n' for u in dedupe_uses(U['tuses'], local_names(U['titems']))) + ''.join(t_.rstrip() + '\n' for t_ in U['titems']) + '}\n'))
    for at, add in sorted(edits, key=lambda e: e[0], reverse=True):
        out = out[:at] + add + out[at:]
    return out


def write_mods(paths):
    tree = {}
    for p in paths:
        node = tree
        for seg in p[:-1]:
            node = node.setdefault(seg, {})
        node.setdefault('__units__', []).append(p[-1])
    def emit(node, dirpath, top):
        units = sorted(node.get('__units__', []))
        subs = sorted(k for k in node if k != '__units__')
        lines = []
        if top:
            lines.append('//! The hand-written replacements, laid out like the matching C++\n'
                         '//! decompilation\'s `src/` tree (one module per `.cpp` unit;\n'
                         '//! `tools/reorg_modules.py`).\n\n')
        else:
            lines.append(f'//! `src/{"/".join(dirpath)}/`.\n\n')
        lines.append('use ssx3_ee_rt::funcs::GuestFn;\n\n')
        for s in subs:
            lines.append(f'pub mod {s};\n')
        for u in units:
            lines.append(f'pub mod {u};\n')
        lines.append('\n/// Every replacement in this directory.\n')
        lines.append('pub fn overrides() -> Vec<(u32, GuestFn, &\'static str)> {\n    let mut v = Vec::new();\n')
        for s in subs:
            lines.append(f'    v.extend({s}::overrides());\n')
        for u in units:
            lines.append(f'    v.extend_from_slice({u}::OVERRIDES);\n')
        lines.append('    v\n}\n')
        d = os.path.join(GAME, *dirpath)
        os.makedirs(d, exist_ok=True)
        open(os.path.join(d, 'mod.rs'), 'w').write(''.join(lines))
        for s in subs:
            emit(node[s], dirpath + [s], False)
    emit(tree, [], True)


def main():
    cmd = sys.argv[1]
    mods = sys.argv[2:]
    if cmd == 'plan':
        parsed, allov, target = plan(mods)
        parsed = assign(parsed, target)
        units, unassigned = build(parsed, allov, target)
        for path, U in sorted(units.items()):
            print('/'.join(path), len(U['ovs']), 'replacements,', len(U['items']), 'items,', len(U['titems']), 'test items')
        for u in unassigned:
            print('unassigned:', *u)
    elif cmd == 'apply':
        written, unassigned = apply(mods)
        print(len(written), 'units written')
        for u in unassigned:
            print('unassigned:', *u)


if __name__ == '__main__':
    main()
