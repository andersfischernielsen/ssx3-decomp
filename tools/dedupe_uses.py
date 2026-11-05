#!/usr/bin/env python3
"""After `reorg_modules.py apply`: drop duplicated names from `use` statements reported as E0252 in a cargo log.

    cargo test -p ssx3-game --lib --no-run > LOG 2>&1; python3 tools/dedupe_uses.py LOG
(single- or multi-line `use a::b::{x, y};` and `use a::b::x;`)."""
import re,sys,collections
log=open(sys.argv[1]).read()
hits=collections.defaultdict(lambda: collections.defaultdict(set))
for m in re.finditer(r"error\[E0252\]: the name `(\w+)` is defined multiple times\n\s*--> ([^:]+):(\d+):\d+",log):
    hits[m.group(2)][int(m.group(3))].add(m.group(1))
for f,lines in hits.items():
    L=open(f).read().split('\n')
    # group reported lines by their enclosing use statement
    stmts=collections.defaultdict(set)
    for l,names in lines.items():
        i=l-1
        while i>=0 and not re.match(r"\s*(pub(\(crate\))? )?use ",L[i]): i-=1
        j=i
        while j<len(L) and not L[j].rstrip().endswith(';'): j+=1
        stmts[(i,j)]|=names
    for (i,j),names in sorted(stmts.items(),reverse=True):
        text='\n'.join(L[i:j+1])
        m=re.match(r"(\s*(?:pub(?:\(crate\))? )?use [\w:]+::)\{(.*)\};\s*$",text,re.S)
        if m:
            last=m.group(1).rstrip(':').rsplit('::',1)[-1].split()[-1]  # `self` imports this name
            keep=[x.strip() for x in m.group(2).replace('\n',' ').split(',') if x.strip() and x.strip() not in names
                  and not (x.strip()=='self' and last in names)]
            new=[] if not keep else [f"{m.group(1)}{{{', '.join(keep)}}};"]
        else:
            m=re.match(r"\s*(?:pub(?:\(crate\))? )?use (?:[\w:]+::)?(\w+)(?: as (\w+))?;\s*$",text)
            nm = m and (m.group(2) or m.group(1))
            new=[] if nm in names else L[i:j+1]
        L[i:j+1]=new
    open(f,'w').write('\n'.join(L))
