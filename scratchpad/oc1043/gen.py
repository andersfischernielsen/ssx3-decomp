import re,sys,glob
sys.path.insert(0,'scratchpad/oc1043')
def body(addr):
    for f in glob.glob('generated/ee/s*/src/lib.rs'):
        L=open(f).read().split('\n')
        for i,l in enumerate(L):
            if l.startswith('pub fn f_%s__generated('%addr):
                j=i
                while not L[j].startswith('/// Slot for the hand-written'): j+=1
                b=L[i+1:j]
                while b and b[-1]=='' : b.pop()
                return f.split('/')[2],b[:-1]
ALIAS={}
for f in glob.glob('generated/ee/s*/src/lib.rs'):
    sh=f.split('/')[2]
    for l in open(f):
        m=re.match(r'pub use self::f_[0-9a-f]+ as (\w+);',l)
        if m: ALIAS[m.group(1)]=sh
def conv(addr,name,doc,fnname):
    sh,b=body(addr)
    out=[]
    for l in b:
        m=re.match(r'c\.pc = (0x[0-9a-f]+|t); call_indirect\(c, (0x[0-9a-f]+|t)\);$',l)
        if m:
            out.append('call_indirect(c, %s);'%m.group(2)) if False else None
        l=re.sub(r'^c\.pc = (0x[0-9a-f]+); (\w+)\(c\);$',lambda m:'c.pc = %s; ee::%s::%s(c);'%(m.group(1),ALIAS[m.group(2)],m.group(2)),l)
        l=l.replace('{ return; }','{ return Err(Unwound); }')
        l=l.replace('c.pc = t; return;','c.pc = t; return Ok(());')
        m=re.match(r'if c\.cycles >= c\.next_event \{ c\.pc = (0x[0-9a-f]+); c\.event\(\); \} continue (\'\w+);$',l)
        if m: l="if event_due(c) { deliver_event_here(c, %s); }\ncontinue %s;"%(m.group(1),m.group(2))
        out.append(l)
    s=doc+"\n#[allow(unused_labels, unused_assignments, unused_mut, unreachable_code, unused_variables)]\npub fn %s(c: &mut Cpu) {\n    guest(c, |c| {\n"%fnname
    s+='\n'.join('        '+l if l else l for l in out)+"\n    })\n}\n"
    return s
if __name__=='__main__':
    spec=[('002aec58','PATH_PlatGetTrackStatus','path_plat_get_track_status'),]
