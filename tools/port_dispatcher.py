#!/usr/bin/env python3
"""conv.py ADDR SHARD NAME FNNAME > module.rs : first-entry hand port of a
dispatcher-shaped generated body (the generated text, calls through call_ptr/Callee)."""
import re, sys
addr, shard, name, fn = sys.argv[1:5]
a = int(addr, 16)
t = open(f'generated/ee/{shard}/src/lib.rs').read()
sym = f'f_{a:08x}'
i = t.index(f'pub fn {sym}__generated(c: &mut Cpu)')
m = re.search(r'\n(?=/// |pub fn |pub use |#\[)', t[i + 10:])
body = t[i:i + 10 + m.start()].rstrip()
lines = body.split('\n')
assert lines[0].startswith('pub fn')
lines = lines[1:-1]
b = '\n'.join(lines)
b = re.sub(r'\s*ssx3_ee_rt::prof::enter\(c, 0x[0-9a-f]+\);', '', b)
dispatcher = 'let mut pc: u32 = c.pc;' in b
if dispatcher:
    b = b.replace('let mut pc: u32 = c.pc;', f'let mut pc: u32 = 0x{a:08x};')
else:
    b = re.sub(r'\s*if c\.pc != 0x[0-9a-f]+ \{ return f_[0-9a-f]+__entries\(c\); \}', '', b, 1)
# calls
b = re.sub(r'c\.pc = (0x[0-9a-f]+|t); call_indirect\(c, (?:0x[0-9a-f]+|t)\);\s*if c\.pc != (0x[0-9a-f]+) \{ return; \}',
           lambda m: f'call_ptr(c, {m.group(1)}, {m.group(2)})?;', b)
b = re.sub(r'c\.pc = (0x[0-9a-f]+); (\w+)\(c\);\s*if c\.pc != (0x[0-9a-f]+) \{ return; \}',
           lambda m: f'Callee {{ addr: {m.group(1)}, f: ee::{shard}::{m.group(2)} }}.call(c, {m.group(3)})?;', b)
# events
b = re.sub(r'if c\.cycles >= c\.next_event \{ c\.pc = (0x[0-9a-f]+); if c\.event\(\) \{ return; \} \}',
           lambda m: f'if back_edge_unwinds(c, {m.group(1)}) {{ return Ok(()); }}', b)
b = re.sub(r'if c\.cycles >= c\.next_event \{ c\.pc = (0x[0-9a-f]+); c\.event\(\); \}',
           lambda m: f'if event_due(c) {{ deliver_event_here(c, {m.group(1)}); }}', b)
b = re.sub(r'\breturn;', 'return Ok(());', b)
left = [l for l in b.split('\n') if re.search(r'call_indirect|c\.event\(\)|\bf_[0-9a-f]{8}__entries', l)]
if left:
    sys.stderr.write('UNCONVERTED:\n' + '\n'.join(left[:10]) + '\n')
guard = f'    if c.pc != 0x{a:08x} {{\n        return ee::{shard}::{sym}__generated(c);\n    }}\n'
out = f'''/// `{name}` (0x{a:08x}): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn {fn}(c: &mut Cpu) {{
{guard}    guest(c, |c| {{
{b}
    }});
}}
'''
sys.stdout.write(out)
