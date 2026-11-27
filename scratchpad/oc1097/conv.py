import re
src=open('generated/ee/s14/src/lib.rs').read().split('\n')
i=next(k for k,l in enumerate(src) if l.startswith('pub fn f_003cbc58__generated'))
body=[]
for l in src[i+1:]:
    if l.startswith('pub fn') or l.startswith('///') or l.startswith('#['): break
    body.append(l)
t='\n'.join(body)
t=re.sub(r"c\.pc = (0x[0-9a-f]+); call_indirect\(c, \1\);\nif c\.pc != (0x[0-9a-f]+) \{ return; \}", r"call_ptr(c, \1, \2)?;", t)
t=re.sub(r"c\.pc = 0x003cf798; SND_DspBufferSource_Init\(c\);\nif c\.pc != (0x[0-9a-f]+) \{ return; \}", r"DSP_BUFFER_SOURCE_INIT.call(c, \1)?;", t)
t=re.sub(r"c\.pc = t; call_indirect\(c, t\);\nif c\.pc != (0x[0-9a-f]+) \{ return; \}", r"call_ptr(c, t, \1)?;", t)
t=re.sub(r"if c\.cycles >= c\.next_event \{ c\.pc = (0x[0-9a-f]+); c\.event\(\); \}", r"if event_due(c) { deliver_event_here(c, \1); }", t)
t=t.replace("c.pc = t; return;","c.pc = t;")
open('scratchpad/oc1097/body1.rs','w').write(t)
print(t.count('call_ptr'), 'c.pc =' in t, t.count('call_indirect'))
