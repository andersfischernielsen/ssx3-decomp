import re,sys
src=open('generated/ee/s13/src/lib.rs').read().split('\n')
def body(name):
    i=next(k for k,l in enumerate(src) if l.startswith('pub fn '+name+'('))
    j=next(k for k in range(i,len(src)) if src[k].startswith('/// Slot for'))
    return '\n'.join(src[i+1:j]).rstrip()
def conv(t):
    t=t.rstrip()
    assert t.endswith('}')
    t=t[:-1].rstrip()   # drop fn closing brace
    t=re.sub(r'c\.pc = (\w+); call_indirect\(c, \1\);\nif c\.pc != (0x[0-9a-f]+) \{ return; \}',r'call_ptr(c, \1, \2)?;',t)
    t=re.sub(r'c\.pc = (0x[0-9a-f]+); \w+\(c\);\nif c\.pc != (0x[0-9a-f]+) \{ return; \}',r'call_ptr(c, \1, \2)?;',t)
    t=re.sub(r'if c\.cycles >= c\.next_event \{ c\.pc = (0x[0-9a-f]+); c\.event\(\); \} continue (\'\w+);',
             r'if event_due(c) { deliver_event_here(c, \1); } continue \2;',t)
    t=t.replace('c.pc = t; return;','c.pc = t;\nreturn Ok(());')
    assert 'call_indirect' not in t and 'return;' not in t, [l for l in t.split('\n') if 'call_indirect' in l or 'return;' in l]
    return t
open(sys.argv[1],'w').write(conv(body(sys.argv[2])))
