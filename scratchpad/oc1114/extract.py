import sys
src=open('generated/ee/s14/src/lib.rs').read().split('\n')
out=[]
for a in sys.argv[1:]:
    name=f'pub fn f_{a}__generated('
    idx=[i for i,l in enumerate(src) if l.startswith(name)]
    i=idx[0]
    out.append(f'=== {a} line {i+1}')
    j=i+1
    while j<len(src) and not (src[j].startswith('pub fn') or src[j].startswith('///') or src[j].startswith('#[')):
        out.append(src[j]); j+=1
open('scratchpad/oc1114/bodies.txt','w').write('\n'.join(out)+'\n')
