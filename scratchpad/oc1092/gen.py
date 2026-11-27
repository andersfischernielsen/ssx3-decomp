import re
L=open('generated/ee/s13/src/lib.rs').read().split('\n')
s=next(i for i,l in enumerate(L) if l.startswith('pub fn f_003bde08__generated'))
e=next(i for i in range(s,len(L)) if L[i].startswith('/// Slot for the hand-written replacement of `f_003bde08`'))
body=L[s+1:e]
# strip trailing '}' (fn close) and blank
while body and body[-1].strip()=='' : body.pop()
assert body[-1]=='}'; body.pop()
out=[];i=0
while i<len(body):
    l=body[i]
    m=re.match(r'c\.pc = (0x[0-9a-f]+); (\w+)\(c\);$',l)
    if m:
        m2=re.match(r'if c\.pc != (0x[0-9a-f]+) \{ return; \}$',body[i+1])
        a=m.group(1); r=m2.group(1)
        out.append(f'call_ptr(c, {a[:4]}_{a[4:6]}_{a[6:]} if False else {a}, {r})?; // {m.group(2)}'.replace(f'{a[:4]}_{a[4:6]}_{a[6:]} if False else ',''))
        i+=2; continue
    m=re.match(r'if c\.cycles >= c\.next_event \{ c\.pc = (0x[0-9a-f]+); c\.event\(\); \} (continue .*)$',l)
    if m:
        out.append(f'if event_due(c) {{ deliver_event_here(c, {m.group(1)}); }} {m.group(2)}'); i+=1; continue
    if l=='c.pc = t; return;':
        out.append('c.pc = t; return Ok(());'); i+=1; continue
    out.append(l); i+=1
src='''
/// `SNDENC_EncodeChannelMode2(ch)` (0x3bde08): encode one channel of an audio
/// frame in the mode-2 layout. Finds the subband peaks of the shared encoder
/// state (D_0050AC6C), derives 24 scale-factor indices from them (peak times
/// the per-band factor; below 4.0 by a byte table, above by the float exponent
/// and a table lookup), then quantises the subbands in three groups (5 x 16,
/// 9 x 16 and 5 pairs of 16 values) and packs the results as bit fields into
/// the frame buffer (D_0050AC68->out + ch * 172 + 284).
///
/// Transcribed block by block from the generated body: the packing sequences
/// leave their scratch registers, byte for byte, in the call log.
pub fn sndenc_encode_channel_mode2(c: &mut Cpu) {
    guest(c, |c| {
'''
src+='\n'.join(out)+'''
        Ok(())
    });
}
'''
open('scratchpad/oc1092/port.rs','w').write(src)
