#!/usr/bin/env python3
"""thread_inventory.py TRACE [--frames A:B] > report.md

Summarise a headless run's thread behaviour. Produce the trace with

    SSX3_TRACE_SYSCALLS=1 SSX3_TRACE_THREADS=1 target/game/ssx3-headless N 2> TRACE

Reads `[sys]` lines (syscall, args, stub pc, thread, ra, cycle) and `[thread]`
lines (create, switch). Maps addresses to functions with
`generated/ee-symbols.tsv`. Report: threads (creator, entry, priority, stack),
blocking/signalling sites per thread (syscall, calling function, semaphore),
switches per frame, and the schedule of one frame window (--frames).
"""
import bisect, collections, re, sys

FRAME_CYCLES = 294_912_000 // 60

BLOCKING = {'WaitSema', 'SleepThread', 'DelayThread', 'SuspendThread', 'RotateThreadReadyQueue'}
WAKING = {'SignalSema', 'iSignalSema', 'WakeupThread', 'iWakeupThread', 'ResumeThread', 'iResumeThread'}


def load_symbols(path='generated/ee-symbols.tsv'):
    starts, rows = [], []
    for l in open(path):
        if l.startswith('#'):
            continue
        p = l.rstrip('\n').split('\t')
        if len(p) < 5:
            continue
        starts.append(int(p[1], 16)); rows.append((int(p[2], 16), p[4].split(',')[0]))
    order = sorted(range(len(starts)), key=lambda i: starts[i])
    return [starts[i] for i in order], [rows[i] for i in order]


def main():
    args = sys.argv[1:]
    window = None
    if '--frames' in args:
        i = args.index('--frames'); a, b = args[i + 1].split(':'); window = (int(a), int(b)); del args[i:i + 2]
    trace = args[0]
    starts, rows = load_symbols()

    def fn(a):
        i = bisect.bisect_right(starts, a) - 1
        if i >= 0 and a < rows[i][0]:
            return rows[i][1] if rows[i][1] != '-' else f'f_{starts[i]:08x}'
        return f'{a:#x}'

    sys_re = re.compile(r'\[sys\] (\w+)\((0x[0-9a-f]+)\) (\w+) (\w+) (\w+) (\w+) from 0x([0-9a-f]+) thread (-?\d+) ra ([0-9a-f]+) cyc (\d+)')
    create_re = re.compile(r'\[thread\] create (\d+) entry 0x([0-9a-f]+) stack 0x([0-9a-f]+)\+0x([0-9a-f]+) prio (\d+)')
    switch_re = re.compile(r'\[thread\] (-?\d+) -> (idle|\d+)(?: at 0x([0-9a-f]+))? cyc (\d+)')

    threads = {}
    creators = {}
    sites = collections.Counter()
    first_cyc = {}
    per_frame_switch = collections.Counter()
    sched = []
    last_cyc = 0
    cur = 0
    for line in open(trace, errors='replace'):
        m = sys_re.match(line)
        if m:
            name, _, a0, a1, a2, a3, stub, th, ra, cyc = m.groups()
            th = int(th); cyc = int(cyc); last_cyc = cyc
            ra = int(ra, 16)
            if name in ('CreateThread', 'iCreateThread'):
                creators[len(creators)] = (fn(ra), th)
            if name in BLOCKING | WAKING | {'ChangeThreadPriority', 'StartThread', 'ExitThread', 'ExitDeleteThread',
                                            'CreateSema', 'DeleteSema', 'PollSema', 'iPollSema', 'SetAlarm', 'iSetAlarm',
                                            'ReleaseAlarm', 'iReleaseAlarm', 'EnableIntc', 'DisableIntc'}:
                key = (th, name, fn(ra), a0 if 'Sema' in name else '')
                sites[key] += 1
                first_cyc.setdefault(key, cyc)
            continue
        m = create_re.match(line)
        if m:
            tid, entry, stack, size, prio = m.groups()
            who = creators.get(len(threads), ('?', 0))
            threads[int(tid)] = dict(entry=int(entry, 16), stack=int(stack, 16), size=int(size, 16), prio=int(prio), creator=who[0], by=who[1])
            continue
        m = switch_re.match(line)
        if m:
            frm, to, pc, cyc = m.groups(); cyc = int(cyc); last_cyc = cyc
            f = cyc // FRAME_CYCLES
            per_frame_switch[f] += 1
            if window and window[0] <= f < window[1]:
                sched.append((f, cyc, int(frm), to, fn(int(pc, 16)) if pc else 'idle'))

    out = []
    out.append(f'# Thread inventory\n\nTrace `{trace}`, {last_cyc // FRAME_CYCLES} frames.\n')
    out.append('## Threads\n\n| id | entry | prio | stack | created by | note |\n| --- | --- | --- | --- | --- | --- |')
    for tid, t in sorted(threads.items()):
        out.append(f"| {tid} | `{fn(t['entry'])}` ({t['entry']:#x}) | {t['prio']} | {t['stack']:#x}+{t['size']:#x} | `{t['creator']}` (thread {t['by']}) | |")
    out.append('\n## Thread-relevant syscalls by site\n\nthread, syscall, calling function, semaphore argument: count (first cycle).\n')
    out.append('| thread | syscall | caller | arg | count | first cycle |\n| --- | --- | --- | --- | --- | --- |')
    for key, n in sorted(sites.items(), key=lambda kv: (kv[0][0], kv[0][1], -kv[1])):
        th, name, caller, arg = key
        out.append(f'| {th} | {name} | `{caller}` | {arg} | {n} | {first_cyc[key]} |')
    if per_frame_switch:
        fr = sorted(per_frame_switch)
        out.append('\n## Context switches per frame\n')
        steady = [per_frame_switch[f] for f in fr[len(fr) // 2:]]
        out.append(f'Frames {fr[0]}..{fr[-1]}; in the second half: min {min(steady)}, max {max(steady)} per frame.\n')
    if sched:
        out.append(f'\n## Schedule, frames {window[0]}..{window[1]}\n\n| frame | cycle | from | to | resumes in |\n| --- | --- | --- | --- | --- |')
        i = 0
        while i < len(sched):
            f, cyc, frm, to, where = sched[i]
            if to == 'idle' and frm == 0:
                j = i
                while j < len(sched) and sched[j][3] == 'idle' and sched[j][2] == 0:
                    j += 1
                out.append(f'| {f} | {cyc} | 0 | idle | x{j - i} (idle ticks, no thread ready) |')
                i = j
                continue
            out.append(f'| {f} | {cyc} | {frm} | {to} | `{where}` |')
            i += 1
    print('\n'.join(out))


if __name__ == '__main__':
    main()
