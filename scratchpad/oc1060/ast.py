import re,sys
class Stmt:
    def __init__(s,t): s.t=t
    def __repr__(s): return s.t
class Block:
    def __init__(s,l,b): s.l=l; s.b=b
class If:
    def __init__(s,c,a,b): s.c=c; s.a=a; s.b=b
class Break:
    def __init__(s,l): s.l=l
def parse(lines):
    pos=[0]
    def seq():
        out=[]
        while pos[0]<len(lines):
            l=lines[pos[0]].strip()
            if l=='' : pos[0]+=1; continue
            if l=='}' or l=='} else {': return out
            pos[0]+=1
            m=re.match(r"('[\w]+): \{$",l)
            if m:
                b=seq(); assert lines[pos[0]].strip()=='}',(pos[0],lines[pos[0]]); pos[0]+=1
                out.append(Block(m.group(1),b)); continue
            m=re.match(r"if (.*) \{$",l)
            if m:
                a=seq(); e=[]
                if lines[pos[0]].strip()=='} else {':
                    pos[0]+=1; e=seq()
                assert lines[pos[0]].strip()=='}',(pos[0],lines[pos[0]]); pos[0]+=1
                out.append(If(m.group(1),a,e)); continue
            m=re.match(r"break ('\w+);$",l)
            if m: out.append(Break(m.group(1))); continue
            out.append(Stmt(l))
        return out
    r=seq(); return r
def neg(c):
    if c=='t': return '!t'
    if c=='!t': return 't'
    if c.startswith('!'): return c[1:]
    return '!'+c
def tail_strip(nodes,X):
    """remove tail breaks to X; return nodes or None if non-tail break exists"""
    if nodes and isinstance(nodes[-1],Break) and nodes[-1].l==X: return nodes[:-1]
    if nodes and isinstance(nodes[-1],If):
        i=nodes[-1]
        return nodes[:-1]+[If(i.c,tail_strip(i.a,X),tail_strip(i.b,X))]
    return nodes
def has_break(nodes,X):
    for n in nodes:
        if isinstance(n,Break) and n.l==X: return True
        if isinstance(n,Block) and has_break(n.b,X): return True
        if isinstance(n,If) and (has_break(n.a,X) or has_break(n.b,X)): return True
    return False
def xform(nodes):
    out=[]
    for n in nodes:
        if isinstance(n,Block): n=Block(n.l,xform(n.b))
        if isinstance(n,If): n=If(n.c,xform(n.a),xform(n.b))
        out.append(n)
    res=[]
    for n in out:
        if isinstance(n,Block): n=shapeN(n) or n
        res.append(n)
    return res
def shapeN(B):
    # Block(X2,[Block(X1,[PRE.., let t=COND, DELAY.., If(t,[Break X1],[ELSE.., let t=true, DELAY2.., If(t,[Break X2],[Break X1])])]), NEG.., Break X2])
    X2=B.l
    if not(B.b and isinstance(B.b[0],Block) and isinstance(B.b[-1],Break) and B.b[-1].l==X2): return None
    inner=B.b[0]; X1=inner.l; NEG=B.b[1:-1]
    ib=inner.b
    if not(ib and isinstance(ib[-1],If)): return None
    f=ib[-1]
    if not(f.c=='t' and len(f.a)==1 and isinstance(f.a[0],Break) and f.a[0].l==X1): return None
    pre=ib[:-1]
    # find let t = COND in pre (last)
    idx=max(i for i,n in enumerate(pre) if isinstance(n,Stmt) and n.t.startswith('let t = ') )
    el=f.b
    if not(isinstance(el[-1],If) and el[-1].c=='t'): return None
    last=el[-1]
    if not(len(last.a)==1 and isinstance(last.a[0],Break) and last.a[0].l==X2 and len(last.b)==1 and last.b[0].l==X1): return None
    el=el[:-1]
    j=next(i for i,n in enumerate(el) if isinstance(n,Stmt) and n.t=='let t = true;')
    ELSE=el[:j]; D2=el[j+1:]
    cond=pre[idx].t[len('let t = '):-1]
    new=pre[:idx]+[Stmt('let neg = %s;'%cond)]+pre[idx+1:]
    new.append(If('neg',xform_n(NEG),ELSE+D2))
    return ('SEQ',new)
def xform_n(x): return x
def flatten(nodes):
    out=[]
    for n in nodes:
        if isinstance(n,tuple): out+=flatten(n[1])
        elif isinstance(n,Block): out.append(Block(n.l,flatten(n.b)))
        elif isinstance(n,If): out.append(If(n.c,flatten(n.a),flatten(n.b)))
        else: out.append(n)
    return out
def shapeE(nodes):
    res=[]
    for n in nodes:
        if isinstance(n,Block): n=Block(n.l,shapeE(n.b))
        if isinstance(n,If): n=If(n.c,shapeE(n.a),shapeE(n.b))
        if isinstance(n,Block) and n.b and isinstance(n.b[-1],If):
            body=tail_strip(n.b,n.l)
            if not has_break(body,n.l):
                res+=body; continue
        res.append(n)
    return res
def tidy(nodes):
    out=[]
    for n in nodes:
        if isinstance(n,If):
            a=tidy(n.a); b=tidy(n.b)
            if not a and b: n=If(neg(n.c),b,[])
            else: n=If(n.c,a,b)
        elif isinstance(n,Block): n=Block(n.l,tidy(n.b))
        out.append(n)
    return out
def emit(nodes,ind=0):
    out=[]; p='    '*ind
    for n in nodes:
        if isinstance(n,Stmt): out.append(p+n.t)
        elif isinstance(n,Break): out.append(p+'break %s;'%n.l)
        elif isinstance(n,Block):
            out.append(p+'%s: {'%n.l); out+=emit(n.b,ind+1); out.append(p+'}')
        elif isinstance(n,If):
            out.append(p+'if %s {'%n.c); out+=emit(n.a,ind+1)
            if n.b: out.append(p+'} else {'); out+=emit(n.b,ind+1)
            out.append(p+'}')
    return out
def convert(lines):
    t=parse(lines)
    t=flatten(xform(t))
    t=shapeE(t)
    # run N again for outer flattening after E
    t=flatten(xform(t))
    t=shapeE(t)
    return tidy(t)
if __name__=='__main__':
    src=open('body.rs').read().split('\n')
    # strip fn header and closing
    i=next(k for k,l in enumerate(src) if l.startswith('pub fn f_00383a10__generated'))
    j=max(k for k,l in enumerate(src) if l.startswith('/// Slot for'))
    body=src[i+1:j]
    # drop final '}' of fn
    k=max(k for k,l in enumerate(body) if l=='}')
    body=body[:k]
    t=convert(body)
    open('struct.rs','w').write('\n'.join(emit(t)))
