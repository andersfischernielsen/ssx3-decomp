#!/usr/bin/env python3
"""port_dispatchers_multi.py OCM < lines "ADDR SHARD Name fn": one staging module for several dispatcher-shaped bodies."""
import subprocess, sys
ocm = sys.argv[1]
rows = [l.split() for l in sys.stdin if l.strip()]
hdr = f'''//! Staging module {ocm}: dispatcher-shaped bodies converted from the generated
//! code, each with a differential test. Moved into the source-tree units by
//! `tools/reorg_modules.py apply {ocm}`.

#![allow(unused_imports)]

use crate::call::{{call_ptr, guest, Callee, Frame}};
use crate::events::{{back_edge_unwinds, deliver_event_here, event_due}};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{{reg::*, sx}};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{{Src, W, X, Y, Z}};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
'''
for a, sh, n, fn in rows:
    hdr += f'    (0x{int(a,16):08x}, {fn}, "{n}"),\n'
hdr += '];\n\n'
bodies = ''
tests = '''
#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
'''
for a, sh, n, fn in rows:
    ai = int(a, 16)
    r = subprocess.run(['python3', 'tools/port_dispatcher.py', a, sh, n, fn], capture_output=True, text=True)
    if r.returncode: sys.stderr.write(f'{n}: FAILED {r.stderr[-300:]}\n'); continue
    if r.stderr: sys.stderr.write(f'{n}: {r.stderr}')
    bodies += r.stdout + '\n'
    tests += f'''
    #[test]
    fn {fn}_matches() {{
        check(
            ee::{sh}::f_{ai:08x}__generated,
            {fn},
            200,
            &|c, seed| {{
                machine(c, seed, 0x{ai & 0xffff:x}, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x{ai:08x};
            }},
        );
    }}
'''
tests += '}\n'
open(f'crates/ssx3-game/src/{ocm}.rs', 'w').write(hdr + bodies + tests)
