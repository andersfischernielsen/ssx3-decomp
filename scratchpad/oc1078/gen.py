import re
src=open('generated/ee/s13/src/lib.rs').read().split('\n')
start=next(i for i,l in enumerate(src) if l.startswith('pub fn f_003a2bc8__generated'))
end=next(i for i in range(start,len(src)) if src[i].startswith('/// Slot for the hand-written'))
body=src[start+1:end]
# strip trailing "}" and blanks
while body and body[-1].strip() in ('','}'):
    last=body.pop()
    if last.strip()=='}': break
out=[]
pend_ra=None
for l in body:
    s=l
    m=re.match(r'c\.g\[RA\]\[0\] = sx\((0x[0-9a-f]+)\);$',s)
    if m:
        pend_ra=m.group(1); continue
    m=re.match(r'c\.pc = (0x[0-9a-f]+); call_indirect\(c, (0x[0-9a-f]+)\);$',s)
    if m:
        out.append(f'call_ptr(c, {m.group(2)}, {pend_ra})?;'); continue
    if s.startswith('if c.pc != 0x') and s.endswith('{ return; }'): continue
    m=re.match(r'if c\.cycles >= c\.next_event \{ c\.pc = (0x[0-9a-f]+); c\.event\(\); \} (continue .*)$',s)
    if m:
        out.append(f'if event_due(c) {{ deliver_event_here(c, {m.group(1)}); }}'); out.append(m.group(2)); continue
    if s=='c.pc = t; return;':
        out.append('c.pc = t;'); out.append('return Ok(());'); continue
    out.append(s)
open('scratchpad/oc1078/body.rs','w').write('\n'.join(out))
print(len(out), out[-6:])
