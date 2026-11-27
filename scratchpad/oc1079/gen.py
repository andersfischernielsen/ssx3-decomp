import re
L=open('generated/ee/s13/src/lib.rs').read().split('\n')[34452:35000]
s=next(i for i,l in enumerate(L) if 'c.cycles += 493;' in l)+1
e=next(i for i,l in enumerate(L) if 'c.pc = 0x0039e288' in l)
body=L[s:e]
M=0xffffffffffffffff
def sx(v): v&=0xffffffff; return v if v<0x80000000 else v|0xffffffff00000000
names={'V0':2,'V1':3,'A0':4,'A1':5,'A2':6,'A3':7,'T0':8,'T1':9,'T2':10,'T3':11,'T4':12,'T5':13,'T6':14,'T7':15,'S0':16,'S1':17,'S2':18,'S3':19,'S4':20,'S5':21,'S6':22,'S7':23,'T8':24,'T9':25,'AT':1,'FP':30,'RA':31,'GP':28}
# gpr value: ('c',u64) or ('gp',off) meaning gp+off
g={'T5':('c',0x510000)}
f={}
stores=[]
def addr(r,op,off):
    v=g[r]
    assert v[0]=='c'
    return (v[1]+off if op=='add' else v[1]-off)&0xffffffff
def addr_s(r,op,off):
    v=g[r]
    if v[0]=='c': return ('c',(v[1]+off if op=='add' else v[1]-off)&0xffffffff)
    raise Exception
for l in body:
    m=re.match(r'c\.g\[(\w+)\]\[0\] = (0x[0-9a-f]+);$',l)
    if m: g[m[1]]=('c',int(m[2],16)); continue
    m=re.match(r'c\.g\[(\w+)\]\[0\] = sx\(\(c\.g\[(\w+)\]\[0\] as u32\)\.wrapping_(add|sub)\((0x[0-9a-f]+)\)\);$',l)
    if m:
        d,sr,op,off=m[1],m[2],m[3],int(m[4],16)
        if sr=='GP':
            assert op=='add'; g[d]=('gp',off)
        else:
            v=g[sr]; assert v[0]=='c'
            g[d]=('c',sx((v[1]+off if op=='add' else v[1]-off)))
        continue
    m=re.match(r'c\.g\[(\w+)\]\[0\] = c\.g\[(\w+)\]\[0\];$',l)
    if m: g[m[1]]=g[m[2]]; continue
    m=re.match(r'c\.lwc1\((\d+), \(c\.g\[GP\]\[0\] as u32\)\.wrapping_sub\((0x[0-9a-f]+)\)\);$',l)
    if m: f[int(m[1])]=('lit',int(m[2],16)); continue
    m=re.match(r'c\.mtc1\((\d+), (\d+)\);$',l)
    if m:
        v=g[next(k for k,n in names.items() if n==int(m[1]))]; assert v[0]=='c'
        f[int(m[2])]=('bits',v[1]&0xffffffff); continue
    m=re.match(r'c\.swc1\((\d+), \(c\.g\[(\w+)\]\[0\] as u32\)\.wrapping_(add|sub)\((0x[0-9a-f]+)\)\);$',l)
    if m:
        a=addr(m[2],m[3],int(m[4],16)); stores.append((a,f[int(m[1])])); continue
    m=re.match(r'c\.sw\(\(c\.g\[(\w+)\]\[0\] as u32\)\.wrapping_(add|sub)\((0x[0-9a-f]+)\), 0u64\);$',l)
    if m:
        a=addr(m[1],m[2],int(m[3],16)); stores.append((a,('bits',0))); continue
    m=re.match(r'c\.g\[RA\]\[0\] = sx\(0x003a4724\);',l)
    if m: g['RA']=('c',0x3a4724); continue
    raise Exception(l)
# check f literal never loaded from a stored place; emit
out=[]
out.append('const BASE_STORES: &[(u32, Src)] = &[')
for a,s_ in stores:
    out.append('    (0x%08x, Src::%s(0x%x)),'%(a,'Lit' if s_[0]=='lit' else 'Bits',s_[1]))
out.append('];')
print('\n'.join(out))
print('//GPR')
for k,v in g.items():
    if k=='RA': continue
    print(k,v)
print('//F')
for k,v in sorted(f.items()): print(k,v)
print(len(stores))

# ---- emit module
tbl=out
regs=[]
order=['T5','T6','T7','V0','S1','S2','S3','S5','S4','T3','A1','A3','T1','V1','T0','A2','T4','T9','T2','S6','FP','T8','S7','S0']
lines=[]
for k in order:
    v=g[k]; assert v[0]=='c'
    lines.append('        c.g[%s][0] = 0x%08x;'%(k,v[1]))
fl=[]
for k,v in sorted(f.items()):
    if v[0]=='lit': fl.append('        c.lwc1(%d, gp.wrapping_sub(0x%x));'%(k,v[1]))
    else: fl.append('        c.f[%d] = 0x%08x;'%(k,v[1]))
open('scratchpad/oc1079/gprs.rs','w').write('\n'.join(lines)+'\n')
open('scratchpad/oc1079/fprs.rs','w').write('\n'.join(fl)+'\n')
open('scratchpad/oc1079/stores.rs','w').write('\n'.join(tbl)+'\n')
