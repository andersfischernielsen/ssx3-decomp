import pickle,re
secs,head,tail,chunk_fn,(s0,e0)=pickle.load(open('secs2.pkl','rb'))
names=['near','far','y_min','y_max','x_min','x_max']
doc=[
"near plane (outcode bit 0x20): the end that is behind `clip_min.w` moves onto the plane; z/w come from the plane, x/y/u/v/colour are interpolated with the perspective factor `frac * w1 / plane`. Re-derives that end's outcode and rejects when the two outcodes now share a bit.",
"far plane (outcode bit 0x10): as the near plane, against `clip_max.w`.",
"bottom edge (outcode bit 8): the end below `clip_min.y` moves onto the edge; x/z/w/u/v/colour interpolate linearly. Rejects on a shared outcode bit (x outcodes re-derived).",
"top edge (outcode bit 4): as the bottom edge, against `clip_max.y`.",
"left edge (outcode bit 2): clip against `clip_min.x`; the moved end's outcode becomes 0 (no reject test: it is the last but one stage).",
"right edge (outcode bit 1): clip against `clip_max.x`; the moved end's outcode becomes 0.",
]
fr=[4,5,6,6]; cr=[1,1,5,5]
out=[]
for k in range(6):
    sec=list(secs[k])
    if k<4:
        s,e=(next(i for i,l in enumerate(sec) if l.strip()=="c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));"),next(i for i,l in enumerate(sec) if l.strip()=='c.cycles += 14;'))
        sec=sec[:s]+['        clipline_colours(c, %d, %d);'%(fr[k],cr[k])]+sec[e:]
    txt='\n'.join(sec).replace("break 'b_383d9c;","return true;")
    ret = 'bool' if k<4 else ''
    if k>=4: assert "b_383d9c" not in txt
    out.append('/// Stage of `PS2GraphicsMan_ClipLineSegment`: %s\n#[inline(never)]\nfn clipline_%s(c: &mut Cpu)%s {\n%s\n%s}\n'%(doc[k],names[k],' -> bool' if k<4 else '',txt,'    false\n' if k<4 else ''))
open('stages.rs','w').write('\n'.join(out))
colour='''/// The four colour channels of a clipped end (`verts[k].r..a`, `u32` words at
/// +0x10..+0x1c): `(float)v0 + frac * (float)(v1 - v0)` truncated, the unsigned
/// word turned into a float the compiler's way (halve, keep the low bit,
/// double when bit 31 is set). `fr` is the FPU register holding the factor,
/// `cr` the register of the last channel's `(float)v0`. The scratch registers it
/// leaves differ per stage, so the instruction order is kept exactly.
#[inline(never)]
fn clipline_colours(c: &mut Cpu, fr: usize, cr: usize) {
%s
}
'''%'\n'.join(chunk_fn)
open('colours.rs','w').write(colour)
