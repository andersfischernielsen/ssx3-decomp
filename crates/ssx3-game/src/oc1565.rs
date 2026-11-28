//! Staging module oc1565 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1565`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_2eec, lbl_00392eec_div3_tail, "lbl_00392EEC"),
    (0x003a_0290, ui_object_get_world_position, "UIObject_GetWorldPosition"),
    (0x003c_55dc, lbl_003c55dc_set_output_ptr, "lbl_003C55DC"),
    (0x0040_8508, libmpeg_set_output_done, "libmpeg_SetOutputDone"),
];

/// `0x00392eec` (no name): `V1 = A3 / 3` (LO of the divide), then a tail jump
/// to the shared body at 0x00392f5c with `A3 = V1`. Kept as the resumable
/// match: a resume at 0x392ef0 / 0x392efc continues natively.
pub fn lbl_00392eec_div3_tail(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x0039_2eec => {
                c.cycles += 1;
                c.g[V0][0] = 0x3;
                pc = 0x0039_2ef0;
            }
            0x0039_2ef0 => {
                c.cycles += 3;
                c.div(0, 7, 2);
                pc = 0x0039_2efc;
            }
            0x0039_2efc => {
                c.cycles += 3;
                c.g[V1][0] = c.lo[0];
                c.g[A3][0] = c.g[V1][0];
                c.pc = 0x0039_2f5c;
                ee::s13::f_00392f5c(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x0039_2eec, 0x0039_2f08);
                return;
            }
        }
    }
}

/// `UIObject_GetWorldPosition` (0x003a0290, `cUIThing::getAbsPosition`): `out`
/// (`A1`) = the object's local position (12 bytes at +0x44) plus the local
/// positions of all its parents (the parent pointer is at +0x40). A resumable
/// match: any resume pc continues natively.
pub fn ui_object_get_world_position(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003a_0290 => {
                c.cycles += 9;
                c.ldl(2, (c.g[A0][0] as u32).wrapping_add(0x4b));
                c.ldr(2, (c.g[A0][0] as u32).wrapping_add(0x44));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4c));
                c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[A1][0] as u32);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                let t = c.g[V0][0] == 0u64;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                pc = if t { 0x003a_030c } else { 0x003a_02b4 };
            }
            0x003a_02b4 => {
                c.cycles += 1;
                pc = 0x003a_02b8;
            }
            0x003a_02b8 => {
                c.cycles += 18;
                c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(3, c.g[SP][0] as u32);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.lwc1(0, c.g[A1][0] as u32);
                c.lwc1(3, c.g[SP][0] as u32);
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.add_s(0, 0, 3);
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.add_s(1, 1, 4);
                c.swc1(0, c.g[A1][0] as u32);
                c.add_s(2, 2, 5);
                c.swc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.swc1(2, (c.g[A1][0] as u32).wrapping_add(0x8));
                pc = 0x003a_0300;
            }
            0x003a_0300 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                if c.g[V0][0] != 0u64 {
                    if event_due(c) {
                        c.pc = 0x003a_02b8;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x003a_02b8;
                } else {
                    pc = 0x003a_030c;
                }
            }
            0x003a_030c => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003a_0290, 0x003a_0314);
                return;
            }
        }
    }
}

/// `0x003c55dc` (no name): stores the constant 0x00495920 (a callback
/// address) through `A1` and returns 0.
pub fn lbl_003c55dc_set_output_ptr(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003c_55dc => {
                c.cycles += 4;
                c.g[V0][0] = 0x490000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5920));
                c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                pc = 0x003c_55f4;
            }
            0x003c_55ec => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0xffff_ffff_ffff_fffb;
                c.pc = t;
                return;
            }
            0x003c_55f4 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0u64;
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003c_55dc, 0x003c_55fc);
                return;
            }
        }
    }
}

/// `libmpeg_SetOutputDone` (0x00408508): `sys` (`A0`): when state (+0x8) is
/// not 2, set it to 2 and copy +0x118 to +0xac; then set the flag at +0x820
/// (V0 = 1 on both paths).
pub fn libmpeg_set_output_done(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x0040_8508 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x2;
                let t = c.g[V0][0] == c.g[V1][0];
                c.g[V0][0] = 0x1;
                pc = if t { 0x0040_8528 } else { 0x0040_8518 };
            }
            0x0040_8518 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x118));
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0xac), c.g[V0][0]);
                c.g[V0][0] = 0x1;
                pc = 0x0040_8528;
            }
            0x0040_8528 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x820), c.g[V0][0]);
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x0040_8508, 0x0040_8530);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, check_lohi, require_paths, DATA};

    /// A3 / 3 then the shared body at 0x392f5c (a leaf) reads a record at T0.
    #[test]
    fn lbl_00392eec_matches() {
        check_lohi(
            ee::s13::f_00392eec__generated,
            lbl_00392eec_div3_tail,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1565_2eec, 0);
                c.pc = 0x0039_2eec;
                c.g[T0][0] = (DATA + 0x1000) as u64;
                for k in 0..0x40u32 {
                    let v = r.next_u32() % 64;
                    c.bus.write32(DATA + 0x1000 + 4 * k, v);
                }
                c.g[A3][0] = if seed % 2 == 0 {
                    r.next_u32() as u64
                } else {
                    (r.next_u32() % 100) as u64
                };
            },
        );
    }

    /// Chains of 0..4 parents with random float positions.
    #[test]
    fn ui_object_get_world_position_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x1565_a290, 0);
            c.pc = 0x003a_0290;
            let depth = seed % 5;
            let node = |k: u32| DATA + 0x2000 + 0x100 * k;
            for k in 0..=depth {
                let n = node(k);
                for f in 0..3u32 {
                    let v = ((r.next_u32() % 20000) as f32 - 10000.0) / 16.0;
                    c.bus.write32(n + 0x44 + 4 * f, v.to_bits());
                }
                let parent = if k == depth { 0 } else { node(k + 1) };
                c.bus.write32(n + 0x40, parent);
            }
            c.g[A0][0] = node(0) as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            c.g[RA][0] = crate::difftest::RA as u64;
        };
        require_paths(ee::s13::f_003a0290__generated, 10, &[11, 33, 54, 75, 96], &setup);
        check(
            ee::s13::f_003a0290__generated,
            ui_object_get_world_position,
            200,
            &setup,
        );
    }

    #[test]
    fn lbl_003c55dc_matches() {
        check(
            ee::s13::f_003c55dc__generated,
            lbl_003c55dc_set_output_ptr,
            50,
            &|c, seed| {
                machine(c, seed, 0x1565_55dc, 0);
                c.pc = 0x003c_55dc;
                c.g[A1][0] = (DATA + 0x100 + 4 * (seed % 8)) as u64;
            },
        );
    }

    #[test]
    fn libmpeg_set_output_done_matches() {
        check(
            ee::s15::f_00408508__generated,
            libmpeg_set_output_done,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1565_8508, 0);
                c.pc = 0x0040_8508;
                let sys = DATA + 0x1000;
                c.g[A0][0] = sys as u64;
                // State 2 on half the seeds (skips the copy).
                let state = if seed % 2 == 0 { 2 } else { r.next_u32() % 5 };
                c.bus.write32(sys + 0x8, state);
                c.bus.write32(sys + 0x118, r.next_u32());
            },
        );
    }
}
