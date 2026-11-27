import re
L=open('struct.rs').read().split('\n')
starts=[i for i,l in enumerate(L) if re.match(r'    c\.g\[V0\]\[0\] = c\.g\[T1\]\[0\] & 0x[0-9a-f]+;',l)]
print(starts)
# section k spans: guard start gs[k] .. next gs
gs=[starts[0]]+[s-1 for s in starts[1:]]   # sec0 guard begins at V0= line (head before)
tail_i=next(i for i,l in enumerate(L) if i>gs[-1] and l=='    c.cycles += 2;' and L[i+2].strip()=='c.g[V0][0] = 0x1;')
ends=gs[1:]+[tail_i]
secs=[L[a:b] for a,b in zip(gs,ends)]
head=L[1:gs[0]]
print(head)
def chunk_split(sec):
    s=next(i for i,l in enumerate(sec) if l.strip()=="c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));")
    e=next(i for i,l in enumerate(sec) if l.strip()=='c.cycles += 14;')
    return s,e
chunks=[]
for k in range(4):
    s,e=chunk_split(secs[k]); chunks.append(secs[k][s:e])
def normc(ch,fr):
    return [re.sub(r'mul_s\(0, %d, 0\)'%fr,'mul_s(0, FR, 0)',x) for x in ch]
fr=[4,5,6,6]
n=[normc(chunks[k],fr[k]) for k in range(4)]
print(n[0]==n[1])
for a,b in zip(n[0],n[2]):
    if a!=b: print(a,'|',b)
import pickle
pickle.dump((secs,head,L[tail_i:]),open('secs.pkl','wb'))
for k in range(6): print(k,len(secs[k]))
print(L[tail_i:])
