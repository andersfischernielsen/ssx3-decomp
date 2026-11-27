import re,sys
L=open('generated/ee/s13/src/lib.rs').read().split('\n')
s=next(i for i,l in enumerate(L) if l.startswith('pub fn f_003be6e0__generated'))
body=L[s:s+420]
def find(pat,start=0):
    return next(i for i in range(start,len(body)) if pat in body[i])
o0=find('c.cycles += 130;'); o1=find("break 'b_3beb40;",o0)
e0=find('c.cycles += 129;'); e1=find("break 'b_3beb40;",e0)
def run(lines):
    env={}; out=[]
    for l in lines:
        m=re.match(r'c\.g\[(\w+)\]\[0\] = c\.lbu\(\(c\.g\[(\w+)\]\[0\] as u32\)(?:\.wrapping_add\((0x[0-9a-f]+)\)|\.wrapping_sub\((0x[0-9a-f]+)\))?\);',l)
        if m:
            r,b,a,sb=m.groups(); off=int(a,16) if a else (-int(sb,16) if sb else 0)
            env[r]=('ld',b,off); continue
        m=re.match(r'c\.g\[(\w+)\]\[0\] = sx\(\(c\.g\[(\w+)\]\[0\] as u32\) (<<|>>) (\d+)\);',l)
        if m:
            r,q,op,k=m.groups(); env[r]=('shl' if op=='<<' else 'shr',env[q],int(k)); continue
        m=re.match(r'c\.g\[(\w+)\]\[0\] = c\.g\[(\w+)\]\[0\] \| c\.g\[(\w+)\]\[0\];',l)
        if m:
            r,a,b=m.groups(); env[r]=('or',env[a],env[b]); continue
        m=re.match(r'c\.g\[(\w+)\]\[0\] = c\.g\[(\w+)\]\[0\] & (0x[0-9a-f]+);',l)
        if m:
            r,a,k=m.groups(); env[r]=('and',env[a],int(k,16)); continue
        m=re.match(r'c\.sb\(\(c\.g\[S0\]\[0\] as u32\)(?:\.wrapping_add\((0x[0-9a-f]+)\)|\.wrapping_sub\((0x[0-9a-f]+)\))?, c\.g\[(\w+)\]\[0\]\);',l)
        if m:
            a,sb,r=m.groups(); off=int(a,16) if a else (-int(sb,16) if sb else 0)
            out.append((off,env[r])); continue
        if 'let t' in l or l.startswith('if t') or 'break' in l or '}' in l or 'else' in l: continue
        print('UNPARSED',l,file=sys.stderr)
    return out
def fmt(e):
    t=e[0]
    if t=='ld':
        b,off=e[1],e[2]
        if b=='A2': return 'sf%d'%off
        if b=='A1': assert off==4; return 'sf0'
        return 'o(%d)'%off
    if t=='shl': return '(%s << %d)'%(fmt(e[1]),e[2])
    if t=='shr': return '(%s >> %d)'%(fmt(e[1]),e[2])
    if t=='or': return '%s | %s'%(fmt(e[1]),fmt(e[2]))
    if t=='and': return '(%s & %#x)'%(fmt(e[1]),e[2])
for name,(a,b) in {'odd':(o0,o1),'even':(e0,e1)}.items():
    print(name)
    for off,e in run(body[a:b]): print(off,fmt(e))
