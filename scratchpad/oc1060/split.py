import re
L=open('body.rs').read().split('\n')
# find section starts
starts=[i for i,l in enumerate(L) if re.search(r'c\.g\[V0\]\[0\] = c\.g\[T1\]\[0\] & 0x[0-9a-f]+;',l)]
print(starts)
def norm(ls):
    out=[]
    for l in ls:
        l=re.sub(r"'b_[0-9a-f]+","'L",l)
        out.append(l)
    return out
secs=[]
b=starts+[len(L)]
for k in range(len(starts)):
    secs.append(norm(L[starts[k]-1:b[k+1]-1]))
for k,s in enumerate(secs):
    open(f'sec{k}.txt','w').write('\n'.join(s))
    print(k,len(s))
