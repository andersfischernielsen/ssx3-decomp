#!/usr/bin/env python3
"""resume_port.py: native resume bodies for guarded first-entry ports.

A guarded port starts with
    if c.pc != 0xA { return ee::sNN::f_A__generated(c); }      (or __entries)
so that a resume (re-entry at a loop head / call-return pc) runs the generated
body.  This tool converts that generated resumable body (`f_A__entries` if the
shard has it, else the `match pc` dispatcher `f_A__generated`) into
`fn NAME_resume(c: &mut Cpu)` right after NAME (calls through Callee/call_ptr,
events through back_edge_unwinds/event_due, everything inside `guest`), and
rewrites the guard to call it.

  tools/resume_port.py --unit crates/ssx3-game/src/game/fe/ovstatepause.rs [--dry-run]
  tools/resume_port.py --all [--dry-run]
  tools/resume_port.py --stats

--dry-run prints a unified diff instead of writing.  Idempotent: a guard that
no longer names a generated body is left alone.  Unhandled guards are listed
on stderr with the reason.  Production code only (text before the first
`#[cfg(test)]`); `ocN.rs` modules are never touched.
"""
import argparse, difflib, glob, os, re, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
GAME = os.path.join(ROOT, 'crates/ssx3-game/src/game')

_shard_cache = {}
_syms = None


def symbols():
    """start address (int) -> (shard, flags)."""
    global _syms
    if _syms is None:
        _syms = {}
        with open(os.path.join(ROOT, 'generated/ee-symbols.tsv')) as f:
            for l in f:
                if l.startswith('#') or not l.strip():
                    continue
                p = l.rstrip('\n').split('\t')
                _syms[int(p[1], 16)] = (p[0], p[3])
    return _syms


def shard_text(shard):
    if shard not in _shard_cache:
        with open(os.path.join(ROOT, f'generated/ee/{shard}/src/lib.rs')) as f:
            _shard_cache[shard] = f.read()
    return _shard_cache[shard]


def gen_body(a, shard):
    """(kind, body text without the outer `pub fn ... {` / `}`) or (None, reason)."""
    t = shard_text(shard)
    sym = f'f_{a:08x}'
    for kind, key in (('entries', f'pub fn {sym}__entries(c: &mut Cpu) {{\n'),
                      ('generated', f'pub fn {sym}__generated(c: &mut Cpu) {{\n')):
        i = t.find(key)
        if i < 0:
            continue
        j = t.index('\n}\n', i)
        body = t[i + len(key):j]
        if kind == 'generated' and 'let mut pc: u32 = c.pc;' not in body:
            # structured body without resume points: it ignores c.pc and always runs from the start
            if re.search(r'call_indirect|c\.pc != 0x|c\.event\(\)\s*\{ return|\(c\); if c\.pc', body):
                return 'none-calls', body
            return 'none', body
        if 'let mut pc: u32 = c.pc;' not in body:
            return None, f'{sym}__{kind} has no `let mut pc: u32 = c.pc;`'
        return kind, body
    return None, f'no {sym}__entries / __generated in {shard}'


def convert(body, shard):
    b = body
    b = re.sub(r'\s*ssx3_ee_rt::prof::enter\(c, 0x[0-9a-f]+\);', '', b)
    b = re.sub(r'c\.pc = (0x[0-9a-f]+|t); call_indirect\(c, (?:0x[0-9a-f]+|t)\);\s*if c\.pc != (0x[0-9a-f]+) \{ return; \}',
               lambda m: f'call_ptr(c, {m.group(1)}, {m.group(2)})?;', b)
    b = re.sub(r'c\.pc = (0x[0-9a-f]+); (\w+)\(c\);\s*if c\.pc != (0x[0-9a-f]+) \{ return; \}',
               lambda m: f'Callee {{ addr: {m.group(1)}, f: ee::{shard}::{m.group(2)} }}.call(c, {m.group(3)})?;', b)
    b = re.sub(r'c\.pc = (0x[0-9a-f]+); (\w+)\(c\);(?= return;)',
               lambda m: f'c.pc = {m.group(1)}; ee::{shard}::{m.group(2)}(c);', b)
    b = re.sub(r'if c\.cycles >= c\.next_event \{ c\.pc = (0x[0-9a-f]+); if c\.event\(\) \{ return; \} \}',
               lambda m: f'if back_edge_unwinds(c, {m.group(1)}) {{ return Ok(()); }}', b)
    b = re.sub(r'if c\.cycles >= c\.next_event \{ c\.pc = (0x[0-9a-f]+); c\.event\(\); \}',
               lambda m: f'if event_due(c) {{ deliver_event_here(c, {m.group(1)}); }}', b)
    b = re.sub(r'\breturn;', 'return Ok(());', b)
    left = [l.strip() for l in b.split('\n')
            if re.search(r'call_indirect|c\.event\(\)|next_event|__entries|__generated|c\.pc = [^;]*; \w+\(c\);', l)]
    return b, left


def prod_end(text):
    m = re.search(r'^#\[cfg\(test\)\]', text, re.M)
    return m.start() if m else len(text)


def test_ranges(text):
    """Spans of the `#[cfg(test)]` items (the reorg interleaves them with production code)."""
    out = []
    for m in re.finditer(r'^[ \t]*#\[cfg\(test\)\]', text, re.M):
        j = text.find('{', m.end())
        k = text.find(';', m.end())
        if j < 0 or 0 <= k < j:
            out.append((m.start(), k + 1 if k >= 0 else m.end()))
        else:
            out.append((m.start(), match_brace(text, j)))
    return out


def match_brace(text, i):
    """index just past the `}` matching the `{` at text[i]; skips strings/comments/char literals."""
    depth = 0
    n = len(text)
    while i < n:
        ch = text[i]
        if text.startswith('//', i):
            i = text.find('\n', i)
            if i < 0:
                return None
            continue
        if text.startswith('/*', i):
            i = text.find('*/', i) + 2
            continue
        if ch == '"':
            i += 1
            while text[i] != '"':
                i += 2 if text[i] == '\\' else 1
            i += 1
            continue
        if ch == "'":
            m = re.compile(r"'(\\.[0-9a-fx{}]*|[^'\\])'").match(text, i)
            if m:
                i = m.end()
                continue
        if ch == '{':
            depth += 1
        elif ch == '}':
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    return None


GUARD = re.compile(r'if c\.pc != (0x[0-9a-f_]+|[A-Z][A-Z0-9_]*) \{')
CALL = re.compile(r'(?:ssx3_ee|ee)::(s\d\d)::f_([0-9a-f]{8})__(generated|entries)\(c\)')
FN = re.compile(r'^([ \t]*)(?:pub(?:\([a-z]+\))? )?fn (\w+)\(c: &mut Cpu\)\s*\{', re.M)

# name -> use line to add when missing (module path style of this crate)
NEEDS = [
    ('Callee', 'use crate::call::Callee;', r'\bCallee\s*\{'),
    ('call_ptr', 'use crate::call::call_ptr;', r'\bcall_ptr\('),
    ('guest', 'use crate::call::guest;', r'\bguest\('),
    ('back_edge_unwinds', 'use crate::events::back_edge_unwinds;', r'\bback_edge_unwinds\('),
    ('deliver_event_here', 'use crate::events::deliver_event_here;', r'\bdeliver_event_here\('),
    ('event_due', 'use crate::events::event_due;', r'\bevent_due\('),
    ('sx', 'use ssx3_ee_rt::cpu::sx;', r'\bsx\('),
    ('Efu', 'use ssx3_ee_rt::vu0::Efu;', r'\bEfu\b'),
    ('Src', 'use ssx3_ee_rt::vu0::Src;', r'\bSrc\b'),
    ('W', 'use ssx3_ee_rt::vu0::W;', r'(?<![\w:])W\b'),
    ('X', 'use ssx3_ee_rt::vu0::X;', r'(?<![\w:])X\b(?!\w)'),
    ('Y', 'use ssx3_ee_rt::vu0::Y;', r'(?<![\w:])Y\b'),
    ('Z', 'use ssx3_ee_rt::vu0::Z;', r'(?<![\w:])Z\b'),
    ('ee', 'use ssx3_ee as ee;', r'\bee::s\d\d::'),
    ('Cpu', 'use ssx3_ee_rt::Cpu;', r'\bCpu\b'),
]


def top_uses(text):
    return '\n'.join(re.findall(r'^use [^;]*;', text[:prod_end(text)], re.M | re.S))


def in_scope(name, uses, prod):
    if re.search(rf'\b{name}\b', uses):
        return True
    if name in ('guest', 'Callee', 'call_ptr') and re.search(r'crate::call::\*', uses):
        return True
    if name in ('back_edge_unwinds', 'deliver_event_here', 'event_due') and re.search(r'crate::events::\*', uses):
        return True
    if name == 'sx' and re.search(r'cpu::\*', uses):
        return True
    if name in ('W', 'X', 'Y', 'Z', 'Src', 'Efu') and re.search(r'vu0::\*', uses):
        return True
    return False


def add_uses(text, bodies):
    pe = prod_end(text)
    prod = text[:pe]
    uses = top_uses(text)
    need = []
    # registers are used unqualified (SP, A0, ...): glob must be there
    if re.search(r'\bc\.g\[[A-Z]', bodies) and not re.search(r'reg::\*|reg::\{', uses):
        need.append('use ssx3_ee_rt::cpu::reg::*;')
    for name, line, pat in NEEDS:
        if re.search(pat, bodies) and not in_scope(name, uses, prod):
            need.append(line)
    if not need:
        return text, []
    ms = list(re.finditer(r'^use [^;]*;\n', prod, re.M | re.S))
    if not ms:
        # after leading //! docs and #![...] attributes
        m = re.match(r'(?:(?://!.*|#!\[.*\]|)\n)*', prod)
        at = m.end()
        return text[:at] + '\n'.join(need) + '\n' + text[at:], need
    at = ms[-1].end()
    return text[:at] + '\n'.join(need) + '\n' + text[at:], need


def process(path, dry, report):
    text = open(path).read()
    orig = text
    pe = prod_end(text)
    out = []
    pos = 0
    resumes = []
    unhandled = []
    n_guard = 0
    edits = []  # (start, end, replacement)
    tr = test_ranges(text)
    for g in GUARD.finditer(text):
        if any(a <= g.start() < b for a, b in tr):
            continue
        gend = match_brace(text, g.end() - 1)
        block = text[g.start():gend]
        cm = CALL.search(block)
        if not cm:
            continue
        n_guard += 1
        shard_g, af, kind_g = cm.group(1), int(cm.group(2), 16), cm.group(3)
        ag = int(g.group(1).replace('_', ''), 16) if g.group(1).startswith('0x') else af  # named entry const
        # enclosing fn: last `fn NAME(c: &mut Cpu) {` before the guard
        fns = [m for m in FN.finditer(text, 0, g.start())]
        if not fns:
            unhandled.append((ag, '?', 'no enclosing fn(c: &mut Cpu)'))
            continue
        fm = fns[-1]
        name = fm.group(2)
        between = text[fm.end():g.start()]
        between_clean = re.sub(r'^\s*use [^;]*;', '', re.sub(r'//[^\n]*', '', between), flags=re.M)
        if re.sub(r'\s|guest\(c,\s*\|c\|\s*\{|return\s*Ok\(\(\)\)', '', between_clean) != '':
            unhandled.append((ag, name, 'guard is not the first statement'))
            continue
        if af != ag:
            unhandled.append((ag, name, f'guard address 0x{ag:08x} != called f_{af:08x}'))
            continue
        sym = symbols().get(af)
        if not sym:
            unhandled.append((ag, name, 'address not in generated/ee-symbols.tsv'))
            continue
        shard = sym[0]
        if shard != shard_g:
            unhandled.append((ag, name, f'shard {shard_g} in guard vs {shard} in symbols'))
            continue
        if re.search(rf'\bfn {name}_resume\b', text):
            unhandled.append((ag, name, f'{name}_resume already exists'))
            continue
        kind, body = gen_body(af, shard)
        if kind is None:
            unhandled.append((ag, name, body))
            continue
        if kind in ('none', 'none-calls'):
            # the generated body ignores c.pc (no resume points): the guard is a no-op, drop it
            new_block = ('// no resume points: the generated body ignores `c.pc` and always starts at the\n'
                         '    // entry, so every pc runs the body below (the guard is not needed).')
            edits.append((g.start(), gend, new_block))
            report['converted'].append((path, name, ag, kind))
            continue
        b, left = convert(body, shard)
        if left:
            unhandled.append((ag, name, 'unconverted: ' + ' | '.join(left[:3])))
            continue
        # function extent: the `{` at fm.end()-1
        fend = match_brace(text, fm.end() - 1)
        indent = fm.group(1)
        rs = (f'\n/// Native resume of `{name}` (converted from the `{kind}` body): any entry\n'
              f'/// other than 0x{ag:08x}, after an unwound callee or event.\n'
              f'#[allow(unreachable_code, unused_assignments, unused_labels, unused_mut, unused_variables)]\n'
              f'fn {name}_resume(c: &mut Cpu) {{\n    guest(c, |c| {{\n{b}\n    }});\n}}\n')
        # guard rewrite: only the call expression
        new_block = CALL.sub(f'{name}_resume(c)', block)
        edits.append((g.start(), gend, new_block))
        edits.append((fend, fend, '\n' + rs.rstrip('\n') + '\n'))
        resumes.append(b)
        report['converted'].append((path, name, ag, kind))
    for a, n, why in unhandled:
        report['unhandled'].append((path, n, a, why))
    report['guards'] += n_guard
    if not edits:
        return
    # apply from the end; edits must not overlap (guard is inside fn, insert is at fn end)
    edits.sort(key=lambda e: (e[0], e[1]))
    for s, e, r in reversed(edits):
        text = text[:s] + r + text[e:]
    text, added = add_uses(text, '\n'.join(resumes))
    if added:
        report['uses'].append((path, added))
    if text == orig:
        return
    if dry:
        sys.stdout.writelines(difflib.unified_diff(orig.splitlines(True), text.splitlines(True),
                                                    path, path, n=2))
    else:
        open(path, 'w').write(text)


def stats():
    n = ent = plain = 0
    for path in glob.glob(GAME + '/**/*.rs', recursive=True):
        text = open(path).read()
        for g in GUARD.finditer(text, 0, prod_end(text)):
            gend = match_brace(text, g.end() - 1)
            cm = CALL.search(text[g.start():gend])
            if cm:
                n += 1
                if cm.group(3) == 'entries':
                    ent += 1
                else:
                    plain += 1
    print(f'guards naming a generated body: {n}; literal __entries: {ent}; literal __generated: {plain}')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--unit', action='append', default=[])
    ap.add_argument('--all', action='store_true')
    ap.add_argument('--dry-run', action='store_true')
    ap.add_argument('--stats', action='store_true')
    a = ap.parse_args()
    if a.stats:
        stats()
        return
    files = [os.path.abspath(u) for u in a.unit]
    if a.all:
        files = sorted(glob.glob(GAME + '/**/*.rs', recursive=True))
    files = [f for f in files if not re.match(r'oc\d+\.rs$', os.path.basename(f))]
    report = {'converted': [], 'unhandled': [], 'uses': [], 'guards': 0}
    for f in files:
        process(f, a.dry_run, report)
    rel = lambda p: os.path.relpath(p, ROOT)
    print(f'guards found: {report["guards"]}, converted: {len(report["converted"])}, '
          f'unhandled: {len(report["unhandled"])}', file=sys.stderr)
    for p, n, ad, why in report['unhandled']:
        print(f'UNHANDLED {rel(p)} {n} 0x{ad:08x}: {why}', file=sys.stderr)
    for p, u in report['uses']:
        print(f'USES ADDED {rel(p)}: {u}', file=sys.stderr)
    for k in ('none', 'none-calls'):
        print(f'guard dropped ({k}): {sum(1 for x in report["converted"] if x[3] == k)}', file=sys.stderr)
    files_changed = sorted({p for p, *_ in report['converted']})
    if not a.dry_run:
        for p in files_changed:
            print('MODIFIED', rel(p), file=sys.stderr)


if __name__ == '__main__':
    main()
