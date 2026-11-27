import re
def load(k): return open(f'sec{k}.txt').read().split('\n')
def chunk(k):
    L=load(k)
    s=next(i for i,l in enumerate(L) if 'wrapping_add(0x10));' in l and 'A1' in l and 'c.g[V0][0] = c.lw' in l)
    e=next(i for i,l in enumerate(L) if 'cycles += 14;' in l)
    return s,e,L
for k in range(6):
    s,e,L=chunk(k); print(k,s,e,len(L))
import sys
def body(k,fr):
    s,e,L=chunk(k); return [l.replace(f'mul_s(0, {fr}, 0)','mul_s(0, FR, 0)') for l in L[s:e]]
c0=body(0,4);c1=body(1,5);c2=body(2,6);c3=body(3,6)
print(c0==c1,c0==c2,c0==c3)
for a,b in zip(c0,c2):
    if a!=b: print(a,'|',b)
