import re
src=open('scratchpad/oc1107/body.txt').read().split('\n')
out=[]
ra=None
i=0
callee={'0x00404758':'FP_SETUP_PRED_BLOCK','0x00405e30':'FP_DUAL_PRIME','0x00403470':'FP_ERRORF'}
# first pass: calls
lines=[]
for ln in src:
    m=re.match(r'c\.g\[RA\]\[0\] = sx\((0x[0-9a-f]+)\);$',ln)
    if m: ra=m.group(1); continue   # dropped; Callee::call sets it
    m=re.match(r'c\.pc = (0x[0-9a-f]+); (\w+)\(c\);$',ln)
    if m:
        lines.append(f'{callee[m.group(1)]}.call(c, {ra})?;'); continue
    if re.match(r'if c\.pc != 0x[0-9a-f]+ \{ return; \}$',ln): continue
    lines.append(ln)
# second pass: let t = true
res=[];i=0;pend=False
while i<len(lines):
    ln=lines[i]
    if ln=='let t = true;': pend=True; i+=1; continue
    if pend and ln=='if t {':
        res.append(lines[i+1]); i+=5; pend=False; continue
    res.append(ln); i+=1
body=res
# header/footer
assert body[0].startswith('pub fn f_00404050__generated')
body[0]='pub fn lib_mpeg_form_predictions(c: &mut Cpu) {\n    guest(c, |c| {'
# tail
j=len(body)-1
while body[j]!='}': j-=1
assert body[j-1]=='c.pc = t; return;', body[j-3:j+1]
body[j-1]='c.pc = t;\nOk(())\n})'
txt='\n'.join(body)
open('scratchpad/oc1107/port.rs','w').write(txt)
labels=set(re.findall(r"'(b_\w+): \{",txt))
for l in labels:
    n=len(re.findall(r"break '"+l+r"\b",txt))
    print(l,n)
print('return' , [l for l in body if 'return' in l])
print(txt.count('{')-txt.count('}'))
