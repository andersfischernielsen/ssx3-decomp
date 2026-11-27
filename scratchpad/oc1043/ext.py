import re,sys,glob
def body(addr):
    for f in glob.glob('generated/ee/s*/src/lib.rs'):
        L=open(f).read().split('\n')
        for i,l in enumerate(L):
            if l.startswith('pub fn f_%s__generated('%addr):
                j=i
                while not L[j].startswith('/// Slot for the hand-written'): j+=1
                # strip trailing blank
                b=L[i+1:j]
                while b and b[-1]=='' : b.pop()
                assert b[-1]=='}'
                return f.split('/')[2],b[:-1]
for a in sys.argv[1:]:
    s,b=body(a)
    print(a,s,len(b))
    pats={}
    for l in b:
        k=re.sub(r'0x[0-9a-f]+|\d+','N',l.strip())
        if 'c.pc' in l or 'return' in l or 'jump' in l or 'event' in l: pats[k]=pats.get(k,0)+1
    for k,v in pats.items(): print('  ',v,k)
