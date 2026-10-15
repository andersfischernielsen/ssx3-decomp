# Ghidra script (Jython; Script Manager -> run with the SSX 3 program open).
#
# Creates a function at every entry point the recompiler found that Ghidra
# has no function for, and names it from generated/discovered-names.tsv
# (naming pass 5, docs/investigation/naming.md). Entries marked NOTFUNC_ are
# skipped. Afterwards: tools/export-ghidra-functions.py, then save (Cmd-S).
#
# The HTTP backend has no endpoint for creating functions, hence a script.
#@category SSX3

import os

from ghidra.app.cmd.disassemble import DisassembleCommand
from ghidra.program.model.symbol import SourceType

REPO = os.path.expanduser("~/Desktop/ssx3-decomp")
path = askFile("discovered-names.tsv", "Use").getAbsolutePath() if not os.path.exists(
    os.path.join(REPO, "generated", "discovered-names.tsv")) else os.path.join(REPO, "generated", "discovered-names.tsv")

fm = currentProgram.getFunctionManager()
created = named = skipped = failed = 0
for line in open(path):
    if line.startswith("#") or not line.strip():
        continue
    cols = line.rstrip("\n").split("\t")
    addr, name = cols[0], cols[1]
    if name.startswith("NOTFUNC_"):
        skipped += 1
        continue
    a = toAddr(int(addr, 16))
    f = fm.getFunctionAt(a)
    if f is None:
        if getInstructionAt(a) is None:
            DisassembleCommand(a, None, True).applyTo(currentProgram, monitor)
        f = createFunction(a, name)
        if f is None:
            # Inside another function's body: Ghidra refuses; report it.
            print("could not create %s %s (inside %s)" % (addr, name, fm.getFunctionContaining(a)))
            failed += 1
            continue
        created += 1
    if f.getName() != name:
        f.setName(name, SourceType.USER_DEFINED)
        named += 1
print("created %d, renamed %d, NOTFUNC skipped %d, failed %d" % (created, named, skipped, failed))
