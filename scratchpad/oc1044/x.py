import re,sys
f=open('generated/ee/s09/src/lib.rs').read().split('\n')
def body(name):
    i=[k for k,l in enumerate(f) if l.startswith('pub fn '+name+'(')][0]
    j=i
    while not f[j].startswith('/// Slot for'): j+=1
    return f[i+1:j-1]
def tr(l):
    l=re.sub(r'\(c\.g\[(\w+)\]\[0\] as u32\)\.wrapping_add\((0x[0-9a-f]+)\)',r'at(c, \1, \2)',l)
    l=re.sub(r'\(c\.g\[(\w+)\]\[0\] as u32\)\.wrapping_sub\((0x[0-9a-f]+)\)',r'back(c, \1, \2)',l)
    l=re.sub(r'\(c\.g\[(\w+)\]\[0\] as u32\)',r'at(c, \1, 0)',l)
    return l
for n in sys.argv[1:]:
    print('\n'.join(tr(l) for l in body(n)))
    print("=====")
