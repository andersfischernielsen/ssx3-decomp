//! Staging module oc1151 (wave wave273): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1151`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x003c_5c30, snd_init_with_output_mode, "SND_InitWithOutputMode")];

/// `int SND_InitWithOutputMode(void *pheap, int heapsize, int headerVersion)`
/// (0x3c5c30, reference `System_Init`): reads the sound config, builds the
/// render mode list from the output mode word at 0x44e1c4 (8 = Dolby: extra
/// modes 0x44/0x140/0x48; 0x120 only with `voicesiopcpu`), picks channels and
/// spdif mode from the jump table at 0x495970, fixes the IOP rate, writes the
/// config back and calls `SND_Init(heap, size, this)`. Returns 0.
pub fn snd_init_with_output_mode(c: &mut Cpu) {
    if c.pc != 0x003c5c30 {
        return ee::s13::f_003c5c30__generated(c);
    }
    guest(c, |c| {

'b_3c5c70: {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[S0][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
c.g[RA][0] = sx(0x003c5c48);
c.g[A0][0] = c.g[SP][0];
Callee { addr: 0x003b4dd8, f: ee::s13::SND_GetConfig }.call(c, 0x003c5c48)?;
c.g[V0][0] = 0x450000;
c.g[A2][0] = 0x8;
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x1e3c));
c.g[A1][0] = c.g[V0][0];
let t = c.g[V1][0] != c.g[A2][0];
c.sb((c.g[SP][0] as u32).wrapping_add(0x4c), 0u64);
if t {
break 'b_3c5c70;
} else {
c.cycles += 4;
c.g[V0][0] = 0x44;
c.g[V1][0] = 0x1;
c.sh((c.g[SP][0] as u32).wrapping_add(0x4e), c.g[V0][0]);
c.sb((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
break 'b_3c5c70;
}
}
'b_3c5ce8: {
c.cycles += 12;
c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x4c));
c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4e));
c.g[A3][0] = c.g[V1][0];
c.g[A0][0] = 0x24;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sh((c.g[V1][0] as u32), c.g[A0][0]);
c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x4c));
c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x2f));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
let t = c.g[V1][0] == 0u64;
c.sb((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
if t {
break 'b_3c5ce8;
} else {
'b_3c5ccc: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x1e3c));
let t = c.g[V0][0] != c.g[A2][0];
c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x4c));
if t {
break 'b_3c5ccc;
} else {
c.cycles += 8;
c.g[A0][0] = 0x140;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sh((c.g[V0][0] as u32), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x4c));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.sb((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x4c));
break 'b_3c5ccc;
}
}
c.cycles += 7;
c.g[A0][0] = 0x120;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sh((c.g[V0][0] as u32), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x4c));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.sb((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
break 'b_3c5ce8;
}
}
'b_3c5d18: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x1e3c));
c.g[V0][0] = 0x8;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x4c));
if t {
break 'b_3c5d18;
} else {
c.cycles += 8;
c.g[A0][0] = 0x48;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sh((c.g[V0][0] as u32), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x4c));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.sb((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x4c));
break 'b_3c5d18;
}
}
'b_3c5d94: {
c.cycles += 13;
c.g[A0][0] = 0x28;
c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x1e3c));
c.g[V1][0] = 0x1;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A1][0] = (c.g[A2][0] < 0x9u64) as u64;
c.sh((c.g[V0][0] as u32), c.g[A0][0]);
c.sb((c.g[SP][0] as u32).wrapping_add(0x49), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x4c));
c.sb((c.g[SP][0] as u32).wrapping_add(0x48), 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
let t = c.g[A1][0] == 0u64;
c.sb((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
if t {
break 'b_3c5d94;
} else {
'b_3c5d90: {
'b_3c5d8c: {
'b_3c5d7c: {
'b_3c5d78: {
'b_3c5d70: {
c.cycles += 7;
c.g[V0][0] = 0x490000;
c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5970));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[A0][0] as u32;
match t {
0x003c5d68 => {
c.cycles += 2;
let t = true;
c.g[V0][0] = 0x1;
if t {
break 'b_3c5d90;
} else {
break 'b_3c5d70;
}
}
0x003c5d70 => {
break 'b_3c5d70;
}
0x003c5d78 => {
break 'b_3c5d78;
}
0x003c5d8c => {
break 'b_3c5d8c;
}
0x003c5d94 => {
break 'b_3c5d94;
}
_ => { c.jump_out(t, 0x003c5c30, 0x003c5df0); return Ok(()); }
}
}
c.cycles += 2;
let t = true;
c.g[V0][0] = 0x4;
if t {
break 'b_3c5d7c;
} else {
break 'b_3c5d78;
}
}
c.cycles += 1;
c.g[V0][0] = 0x5;
break 'b_3c5d7c;
}
c.cycles += 4;
c.g[V1][0] = 0x2;
c.sb((c.g[SP][0] as u32).wrapping_add(0x47), c.g[V0][0]);
let t = true;
c.sb((c.g[SP][0] as u32).wrapping_add(0x49), c.g[V1][0]);
if t {
break 'b_3c5d94;
} else {
break 'b_3c5d8c;
}
}
c.cycles += 1;
c.g[V0][0] = 0x2;
break 'b_3c5d90;
}
c.cycles += 1;
c.sb((c.g[SP][0] as u32).wrapping_add(0x47), c.g[V0][0]);
break 'b_3c5d94;
}
}
'b_3c5dbc: {
c.cycles += 5;
c.g[A1][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x2a));
c.g[A0][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x2c));
c.g[V1][0] = c.g[A1][0] & 0xffff;
let t = c.g[V1][0] == c.g[A0][0];
c.g[V0][0] = 0x8ca0;
if t {
break 'b_3c5dbc;
} else {
c.cycles += 2;
if c.g[V1][0] != c.g[V0][0] {
c.sh((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[A1][0]);
break 'b_3c5dbc;
} else {
c.cycles += 3;
c.g[V0][0] = 0x5dc0;
if c.g[A0][0] != c.g[V0][0] {
c.sh((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[A1][0]);
break 'b_3c5dbc;
} else {
break 'b_3c5dbc;
}
}
}
}
c.cycles += 13;
c.g[RA][0] = sx(0x003c5dc4);
c.g[A0][0] = c.g[SP][0];
Callee { addr: 0x003b4fc0, f: ee::s13::SND_SetConfig }.call(c, 0x003c5dc4)?;
c.g[V1][0] = 0x450000;
c.g[V0][0] = 0x510000;
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x1e40));
c.g[A2][0] = c.g[S0][0];
c.g[RA][0] = sx(0x003c5ddc);
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x57b0));
Callee { addr: 0x003b5158, f: ee::s13::SND_Init }.call(c, 0x003c5ddc)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[V0][0] = 0u64;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{check_reaching, fake, fall_back, hook, machine};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const OC1151_GATE: u32 = DATA + 0x1000;
    const OC1151_MAGIC: u32 = 0x1151_cafe;

    /// Gated fakes for the three real callees: with the gate word set they log
    /// the call (and may unwind); otherwise the previous body runs.
    macro_rules! oc1151_fake {
        ($name:ident, $prev:ident, $gen:path, $addr:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1151_GATE) != OC1151_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fake(c, $addr, ra)
            }
        };
    }
    oc1151_fake!(oc1151_get_config, OC1151_PREV_GET, ee::s13::f_003b4dd8__generated, 0x003b_4dd8);
    oc1151_fake!(oc1151_set_config, OC1151_PREV_SET, ee::s13::f_003b4fc0__generated, 0x003b_4fc0);
    oc1151_fake!(oc1151_snd_init, OC1151_PREV_INIT, ee::s13::f_003b5158__generated, 0x003b_5158);

    fn oc1151_install() {
        hook(addr_of_mut!(ee::s13::OVERRIDE_003B4DD8), oc1151_get_config, &OC1151_PREV_GET);
        hook(addr_of_mut!(ee::s13::OVERRIDE_003B4FC0), oc1151_set_config, &OC1151_PREV_SET);
        hook(addr_of_mut!(ee::s13::OVERRIDE_003B5158), oc1151_snd_init, &OC1151_PREV_INIT);
    }
    use crate::difftest::{elf_image, DATA, SP as TEST_STACK};

    /// Output mode, voice flag and rate fields steer every branch and every
    /// jump-table arm (modes 0..8 and out of range).
    #[test]
    fn snd_init_with_output_mode_matches() {
        oc1151_install();
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_151_001, 3);
            elf_image(c);
            c.bus.write32(OC1151_GATE, OC1151_MAGIC);
            c.pc = 0x003c_5c30;
            c.g[SP][0] = sx(TEST_STACK);
            c.g[A0][0] = sx(DATA + 0x100);
            c.g[A1][0] = sx(DATA + 0x200);
            c.g[A2][0] = sx(DATA + 0x300);
            let mode = seed % 11;
            c.bus.write32(0x0044_e1c4, mode);
            let buf = TEST_STACK - 0x100;
            c.bus.write8(buf + 0x2f, (seed / 11 % 2) as u8);
            let (a, b) = match seed / 22 % 4 {
                0 => (0x1234, 0x1234),
                1 => (0x2222, 0x3333),
                2 => (0x8ca0, 0x5dc0),
                _ => (0x8ca0, 0x4444),
            };
            c.bus.write16(buf + 0x2a, a);
            c.bus.write16(buf + 0x2c, b);
            c.bus.write32(0x0051_57b0, 0x1000);
        };
        check_reaching(
            ee::s13::f_003c5c30__generated,
            snd_init_with_output_mode,
            200,
            &[0x003c_5c48, 0x003c_5dc4, 0x003c_5ddc],
            &setup,
        );
    }
}
