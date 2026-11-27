import re, subprocess
REPO = '/Users/fischer/Desktop/ssx3/ssx3-decomp'
GEN = REPO + '/generated/ee/s13/src/lib.rs'
MOD = REPO + '/crates/ssx3-game/src/oc1100.rs'
L = open(GEN).read().split('\n')

CFG = [
 dict(gen='f_003bf8d0', fn='snd_enc_filter_stage2', addr='0x003b_f8d0', label='SNDENC_FilterStage2',
      maps={'A0': 'src', 'A1': 'dst'}, upd=['A0', 'A1'], doc=[
  '/// `SNDENC_FilterStage2` (0x003bf8d0): sound encoder filter pass 2 (VU0 macro',
  '/// code). Clears the VF1 running sum, then for each of four 0x80-byte input rows',
  '/// adds its eight quads into a running sum, stores the quads at `dst` (+0, +0x10,',
  '/// ..) and the running sums at `dst + 0x100` (+0x100 .. +0x130) and advances both',
  '/// pointers. Callers keep the generated register and cycle totals.']),
 dict(gen='f_003bf960', fn='snd_enc_filter_stage3', addr='0x003b_f960', label='SNDENC_FilterStage3',
      maps={'A0': 'src', 'A1': 'dst'}, upd=[], doc=[
  '/// `SNDENC_FilterStage3` (0x003bf960): sound encoder filter pass 3 (VU0 macro',
  '/// code). Straight-line: sixteen quads from `src` are passed through a butterfly',
  '/// of vector adds and stored back to `dst` in an interleaved order, with the',
  '/// second half of the block handled in the same way. Pointers are not advanced.']),
 dict(gen='f_003bfac0', fn='snd_enc_filter_stage4', addr='0x003b_fac0', label='SNDENC_FilterStage4',
      maps={'A0': 'src', 'A1': 'dst'}, upd=['A0', 'A1'], doc=[
  '/// `SNDENC_FilterStage4` (0x003bfac0): sound encoder filter pass 4 (VU0 macro',
  '/// code). A prologue adds eight input quads pairwise and stores them, then a',
  '/// three-iteration loop (0x80 bytes of input and output per pass) repeats the',
  '/// pairwise sum-and-store on the next block. Pointers and the counter are',
  '/// written back before each event check and before the return.']),
 dict(gen='f_003bfb90', fn='snd_enc_filter_stage5', addr='0x003b_fb90', label='SNDENC_FilterStage5',
      maps={'A0': 'src', 'A1': 'dst'}, upd=['A0', 'A1'], doc=[
  '/// `SNDENC_FilterStage5` (0x003bfb90): sound encoder filter pass 5 (VU0 macro',
  '/// code). Sums and differences of quad pairs from each 0x80-byte input block,',
  '/// stored to `dst`, repeated for three blocks in a loop with the event check at',
  '/// the back edge.']),
 dict(gen='f_003bfc50', fn='snd_enc_filter_stage6', addr='0x003b_fc50', label='SNDENC_FilterStage6',
      maps={'A0': 'src', 'A1': 'coef'}, upd=['A0'], doc=[
  '/// `SNDENC_FilterStage6` (0x003bfc50): sound encoder filter pass 6 (VU0 macro',
  '/// code). Works in place on `src` (a0): butterflies of the quads, scaled by the',
  '/// coefficients loaded from `coef` (a1, VF16), three blocks of 0x80 bytes.']),
 dict(gen='f_003bfd60', fn='snd_enc_filter_stage7', addr='0x003b_fd60', label='SNDENC_FilterStage7',
      maps={'A0': 'src', 'A1': 'coef', 'A2': 'out'}, upd=['A0', 'A2'], doc=[
  '/// `SNDENC_FilterStage7` (0x003bfd60): sound encoder filter pass 7 (VU0 macro',
  '/// code). Scales the input quads from `src` (a0) by the coefficients at `coef`',
  '/// (a1, VF15/VF16), forms sums and differences, and stores them to `out` (a2),',
  '/// three blocks of 0x80 bytes in a loop.']),
 dict(gen='f_003bfe80', fn='snd_enc_filter_stage8', addr='0x003b_fe80', label='SNDENC_FilterStage8',
      maps={'A0': 'src', 'A1': 'coef', 'A2': 'out'}, upd=['A0', 'A2'], doc=[
  '/// `SNDENC_FilterStage8` (0x003bfe80): sound encoder filter pass 8 (VU0 macro',
  '/// code). Like stage 7 with the second coefficient quad at `coef + 0x20`: a',
  '/// prologue, then a loop over three 0x80-byte blocks of `src` writing `out`.']),
 dict(gen='f_003c03e0', fn='snd_enc_find_subband_peaks', addr='0x003c_03e0', label='SNDENC_FindSubbandPeaks',
      maps={'A0': 'src', 'A1': 'dst'}, upd=['A0', 'A1'], doc=[
  '/// `SNDENC_FindSubbandPeaks` (0x003c03e0): per subband, masks the sign bit off',
  '/// the sixteen quads of a 0x100-byte block of `src` (a0) and max-reduces them',
  '/// into one quad of four 32-bit peaks, stored at `dst` (a1). Six blocks; `src`',
  '/// advances 0x100 and `dst` 0x10 per block.']),
]

PAT_UPD = re.compile(r'^c\.g\[(A\d)\]\[0\] = sx\(\(c\.g\[A\d\]\[0\] as u32\)\.wrapping_add\((0x[0-9a-f]+)\)\);$')
PAT_EV = re.compile(r'^if c\.cycles >= c\.next_event \{ c\.pc = (0x[0-9a-f]+); c\.event\(\); \} continue \'l_[0-9a-f]+;$')
PAT_LOOP = re.compile(r"^'l_[0-9a-f]+: loop \{$")

def body_of(gen):
    start = L.index(f'pub fn {gen}__generated(c: &mut Cpu) {{')
    end = start + 1
    while end < len(L) and not re.match(r'^(pub fn |pub static|///|#\[)', L[end]):
        end += 1
    body = L[start + 1:end]
    while body and body[-1].strip() == '':
        body.pop()
    assert body[-1] == '}', (gen, body[-1])
    return body[:-1]

def transform(gen, cfg):
    maps, upd = cfg['maps'], cfg['upd']
    rev = {v: k for k, v in maps.items()}
    src = body_of(gen)
    out = []
    i = 0
    while i < len(src):
        s = src[i].strip()
        if s == 'if t {':
            m = PAT_EV.match(src[i + 1].strip())
            assert m, (gen, src[i + 1])
            assert src[i + 2].strip() == '} else {'
            assert src[i + 3].strip() == 'c.cycles += 2;'
            assert src[i + 4].strip() == 'let t = c.g[RA][0] as u32;'
            assert src[i + 5].strip() == 'c.pc = t; return;'
            assert src[i + 6].strip() == '}'
            for r in upd:
                out.append(f'c.g[{r}][0] = sx({maps[r]});')
            out += ['if !t {', 'c.cycles += 2;', 'ret(c);', 'return;', '}']
            out.append(f'if event_due(c) {{ deliver_event_here(c, {m.group(1)}); }}')
            i += 7
            continue
        if PAT_LOOP.match(s):
            out.append('loop {')
            i += 1
            continue
        if s == 'let t = c.g[RA][0] as u32;':
            assert src[i + 1].strip() == 'c.pc = t; return;'
            out.append('ret(c);')
            i += 2
            continue
        m = PAT_UPD.match(s)
        if m:
            reg = m.group(1)
            assert reg in upd, (gen, s)
            out.append(f'{maps[reg]} = {maps[reg]}.wrapping_add({m.group(2)});')
            i += 1
            continue
        s = re.sub(r'\(c\.g\[(A\d)\]\[0\] as u32\)', lambda mm: maps[mm.group(1)], s)
        assert 'c.g[A' not in s or s.startswith('c.g[V0]'), (gen, s)
        out.append(s)
        i += 1
    body = '\n'.join(out)
    decl = []
    for reg, name in maps.items():
        if re.search(r'\b%s\b' % name, body):
            mut = 'mut ' if reg in upd else ''
            decl.append(f'let {mut}{name} = c.g[{reg}][0] as u32;')
    return '\n'.join(decl + [body])

FNS = []
for cfg in CFG:
    txt = transform(cfg['gen'], cfg)
    head = '\n'.join(cfg['doc']) + f"\npub fn {cfg['fn']}(c: &mut Cpu) {{\n"
    FNS.append(head + txt + '\n}\n')

HEADER = '''//! Staging module oc1100 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1100`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
'''
for cfg in CFG:
    HEADER += f"    ({cfg['addr']}, {cfg['fn']}, \"{cfg['label']}\"),\n"
HEADER += '];\n\n'

TESTS = '''
#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// Quad-aligned input block for the filter passes (0x800 bytes).
    pub(crate) const OC1100_SRC: u32 = DATA + 0x1000;
    /// Coefficient quads read through `a1` by stages 6 to 8.
    pub(crate) const OC1100_COEF: u32 = DATA + 0x2000;
    /// Output block (0x800 bytes).
    pub(crate) const OC1100_DST: u32 = DATA + 0x4000;

    pub(crate) fn oc1100_fill(c: &mut Cpu, r: &mut Rng, base: u32, bytes: u32) {
        for i in (0..bytes).step_by(4) {
            c.bus.write32(base + i, r.next_u32());
        }
    }

    /// Scramble every register, fill the blocks with random words and set the
    /// argument registers `a0`, `a1`, `a2`.
    pub(crate) fn oc1100_setup(c: &mut Cpu, seed: u32, salt: u64, a0: u32, a1: u32, a2: u32) {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        oc1100_fill(c, &mut r, OC1100_SRC, 0x800);
        oc1100_fill(c, &mut r, OC1100_COEF, 0x40);
        oc1100_fill(c, &mut r, OC1100_DST, 0x800);
        c.g[A0][0] = a0 as u64;
        c.g[A1][0] = a1 as u64;
        c.g[A2][0] = a2 as u64;
    }
'''
for cfg in CFG:
    g = cfg['gen']
    a = cfg['maps']
    a0 = 'OC1100_SRC'
    a1 = 'OC1100_COEF' if 'coef' in a.values() else 'OC1100_DST'
    a2 = 'OC1100_DST' if 'out' in a.values() else '0'
    if cfg['fn'] == 'snd_enc_filter_stage6':
        a2 = '0'
    salt = int(cfg['addr'].replace('_', ''), 16)
    TESTS += f'''
    #[test]
    fn {cfg['fn']}_matches() {{
        check(ee::s13::{g}__generated, {cfg['fn']}, 200, &|c, seed| {{
            oc1100_setup(c, seed, 0x{salt:x}, {a0}, {a1}, {a2});
        }});
    }}
'''
TESTS += '}\n'

open(MOD, 'w').write(HEADER + '\n' + '\n'.join(FNS) + TESTS)
print('written', MOD)
