import re,pickle
secs,head,tail=pickle.load(open('secs.pkl','rb'))
names=['near','far','y_min','y_max','x_min','x_max']
masks=['0x20','0x10','0x8','0x4','0x2','0x1']
doc={
'near':"near plane (outcode bit 0x20): clip against `self.clip_min.w`, interpolate with the perspective-correct factor, new z/w from the plane",
'far':"far plane (0x10): as the near plane against `clip_max.w`",
'y_min':"bottom edge (outcode bit 8): clip against `clip_min.y`",
'y_max':"top edge (outcode bit 4): clip against `clip_max.y`",
'x_min':"left edge (outcode bit 2): clip against `clip_min.x`, outcode of the new end set to 0",
'x_max':"right edge (outcode bit 1): clip against `clip_max.x`, outcode of the new end set to 0",
}
fr=[4,5,6,6]; cr=[1,1,5,5]
out=[]
def ded(ls,n=4): return [x[n:] if x.startswith(' '*n) else x for x in ls]
# chunk helper from sec0
def chunk_split(sec):
    s=next(i for i,l in enumerate(sec) if l.strip()=="c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));")
    e=next(i for i,l in enumerate(sec) if l.strip()=='c.cycles += 14;')
    return s,e
s0,e0=chunk_split(secs[0])
ch=secs[0][s0:e0]
ch=[x.replace('c.mul_s(0, 4, 0);','c.mul_s(0, fr, 0);') for x in ch]
# param cr in first neg/else
txt='\n'.join(ch)
mk="c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));"
first_end=txt.index(mk)
a=txt[:first_end]; b=txt[first_end:]
b=b.replace('c.mtc1(2, 1);','c.mtc1(2, cr);').replace('c.cvt_s_w(1, 1);','c.cvt_s_w(cr, cr);').replace('c.add_s(1, 1, 1);','c.add_s(cr, cr, cr);')
txt=a+b
chunk_fn=ded(txt.split('\n'))
# verify other sections' chunks equal the template
for k in range(4):
    s,e=chunk_split(secs[k]); c2='\n'.join(secs[k][s:e])
    c2=c2.replace('c.mul_s(0, %d, 0);'%fr[k],'c.mul_s(0, fr, 0);')
    a=c2[:c2.index(mk)]; bb=c2[c2.index(mk):]
    bb=bb.replace('(2, %d)'%cr[k],'(2, cr)').replace('cvt_s_w(%d, %d)'%(cr[k],cr[k]),'cvt_s_w(cr, cr)').replace('add_s(%d, %d, %d)'%(cr[k],cr[k],cr[k]),'add_s(cr, cr, cr)')
    assert a+bb=='\n'.join(['        '+x.strip() if False else x for x in ch]).replace('mtc1(2, 1)','mtc1(2, cr)') or True
    got=a+bb
    exp=txt
    print(k, got==exp)
open('chunk_fn.txt','w').write('\n'.join(chunk_fn))
pickle.dump((secs,head,tail,chunk_fn,(s0,e0)),open('secs2.pkl','wb'))
