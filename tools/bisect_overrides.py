#!/usr/bin/env python3
"""Find which hand-written port makes a scripted run differ (task 1 of
docs/plans/port-architecture.md).

    tools/bisect_overrides.py LIST REF.bin INPUT FRAMES [WORKDIR]

LIST: file of hex addresses (the ports under suspicion). REF.bin: the RAM
dump (`SSX3_RAM_DUMP`) of the run with all of LIST skipped. Installs halves
of LIST (`SSX3_SKIP_OVERRIDES` for the rest) until one address is left whose
installation alone makes the RAM differ from REF. Environment variables pass
through (use SSX3_DEBUG_LENSFX_VIS=1 as the scenario runs do; SSX3_CARD is
the memory card each run starts from a copy of: default saves/ssx3-mc0.ps2,
`fresh` for none, as `tools/scenarios.py card NAME`; BISECT_SKIP=addr,...
stays skipped in every run, e.g. the native libmpeg, which differs by
design). Then shadow
the culprit: SSX3_SHADOW=addr shows its first differing call.
"""
import filecmp, os, shutil, subprocess, sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

def main():
    lst, ref, inp, frames = sys.argv[1:5]
    work = sys.argv[5] if len(sys.argv) > 5 else '/tmp'
    allv = open(lst).read().split()
    dump = os.path.join(work, 'bisect.bin')
    card = os.environ.get('SSX3_CARD', 'saves/ssx3-mc0.ps2')
    copy = os.path.join(work, 'bisect-card.ps2')

    def diverges(installed):
        inst = set(installed)
        skip = [a for a in allv if a not in inst] + [a for a in os.environ.get('BISECT_SKIP', '').split(',') if a]
        if os.path.exists(copy):
            os.remove(copy)
        if card != 'fresh':
            shutil.copy(os.path.join(R, card), copy)
        env = dict(os.environ, SSX3_MEMCARD=copy, SSX3_SKIP_OVERRIDES=','.join(skip), SSX3_RAM_DUMP=dump, SSX3_INPUT=inp)
        subprocess.run([R + '/target/game/ssx3-headless', frames], cwd=R, env=env,
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        subprocess.run(['git', 'checkout', '-q', 'generated/entries.txt'], cwd=R)
        d = not filecmp.cmp(ref, dump, shallow=False)
        print(f"  install {len(installed)}: {'DIFF' if d else 'same'}", flush=True)
        return d

    x = allv[:]
    while len(x) > 1:
        a, b = x[:len(x) // 2], x[len(x) // 2:]
        if diverges(a):
            x = a
        elif diverges(b):
            x = b
        else:
            print('the difference needs both halves; stopped at', len(x))
            break
    print('culprit:', ' '.join(x[:20]))

if __name__ == '__main__':
    main()
