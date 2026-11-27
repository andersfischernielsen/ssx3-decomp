import random,re,sys
from ast import *
def compile_(nodes,code,cnt=[0]):
    for n in nodes:
        if isinstance(n,Stmt): code.append(('s',n.t))
        elif isinstance(n,Break): code.append(('goto',n.l))
        elif isinstance(n,Block):
            compile_(n.b,code); code.append(('label',n.l))
        elif isinstance(n,If):
            cnt[0]+=1; le='E%d'%cnt[0]; ln='N%d'%cnt[0]
            code.append(('br',n.c,le)); compile_(n.a,code); code.append(('goto',ln)); code.append(('label',le)); compile_(n.b,code); code.append(('label',ln))
def run(code,seed):
    rnd=random.Random(seed)
    labels={}
    for i,x in enumerate(code):
        if x[0]=='label': labels[x[1]]=i
    pc=0; env={}; out=[]
    while pc<len(code):
        x=code[pc]; pc+=1
        if x[0]=='s':
            t=x[1]
            m=re.match(r'let (\w+) = (.*);$',t)
            if m and m.group(2)!='true' and m.group(1) in('t','neg'):
                env[m.group(1)]=rnd.random()<0.5; out.append('let t = '+m.group(2)+' => '+str(env[m.group(1)]))
            elif m and m.group(2)=='true' and m.group(1)=='t':
                env['t']=True; pass
            else: out.append(t)
        elif x[0]=='goto': pc=labels[x[1]]
        elif x[0]=='br':
            c=x[1]; neg_=False
            if c.startswith('!') and c[1:] in env: neg_=True; c=c[1:]
            if c in env: v=env[c]^neg_
            else:
                f=rnd.random()<0.5; out.append('dec '+c.lstrip('!')+' '+str(f)); v=f^c.startswith('!')
            if not v: pc=labels[x[2]]
    return out
if __name__=='__main__':
    src=open('body.rs').read().split('\n')
    i=next(k for k,l in enumerate(src) if l.startswith('pub fn f_00383a10__generated'))
    j=max(k for k,l in enumerate(src) if l.startswith('/// Slot for'))
    body=src[i+1:j]; k=max(k for k,l in enumerate(body) if l=='}'); body=body[:k]
    a=parse(body)
    b=parse(open('struct.rs').read().split('\n'))
    ca=[];cb=[]; compile_(a,ca); compile_(b,cb)
    bad=0
    for s in range(30000):
        ra=run(ca,s); rb=run(cb,s)
        if ra!=rb:
            bad+=1
            if bad<3:
                for x,(p,q) in enumerate(zip(ra,rb)):
                    if p!=q: print(s,x,p,'|',q); break
                else: print(len(ra),len(rb))
    print('bad',bad)
