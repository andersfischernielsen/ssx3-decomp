//! Staging module oc1494 (wave wave275): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1494`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_3938, lbl_003a3938, "lbl_003A3938"),
    (0x003a_3940, lbl_003a3940, "lbl_003A3940"),
    (0x003a_39d4, lbl_003a39d4, "lbl_003A39D4"),
    (0x002f_0e88, rider_base_ctor_attack_fx, "RiderBase_CtorAttackFX"),
    (
        0x0016_c4c8,
        camera_trigger_man_read_trigger_volume,
        "CameraTriggerMan_ReadTriggerVolume",
    ),
    (0x0040_a440, func_0040a440, "func_0040A440"),
    (0x0017_fdf0, cfe_input_get_state, "cFEInput_getState"),
];

/// Shared body of the three one-store labels of the caller's dispatcher: `swc1
/// f1, off(t0)`, then a jump to the loop tail at 0x3a39e0 (its own
/// `f_003a39e0` entry, which checks the loop count and returns to `ra`).
fn oc1494_store_f1_then_tail(c: &mut Cpu, off: u32) {
    c.cycles += 2;
    c.swc1(1, (c.g[T0][0] as u32).wrapping_add(off));
    c.pc = 0x003a_39e0;
    ee::s13::f_003a39e0(c);
}

/// `lbl_003A3938` (0x3a3938): stores f1 at `t0 + 0x44`, then jumps to 0x3a39e0.
pub fn lbl_003a3938(c: &mut Cpu) {
    oc1494_store_f1_then_tail(c, 0x44);
}

/// `lbl_003A3940` (0x3a3940): stores f1 at `t0 + 0x48`, then jumps to 0x3a39e0.
pub fn lbl_003a3940(c: &mut Cpu) {
    oc1494_store_f1_then_tail(c, 0x48);
}

/// `lbl_003A39D4` (0x3a39d4): stores f1 at `t0 + 0x60`, then jumps to 0x3a39e0.
pub fn lbl_003a39d4(c: &mut Cpu) {
    oc1494_store_f1_then_tail(c, 0x60);
}

/// `RiderBase_CtorAttackFX` (0x2f0e88; the dossier calls it `cPunchFX_cPunchFX`):
/// sets up the attack-FX object at `this` (a0). When `arg` (a1) is nonzero the
/// sub-object at `this + 0xb0` is built by `0x11b698` and its pointer is stored at
/// `*this`. Then the object's pointers at +0x6c0, +0x6d0 and +0x6e8 are set from
/// the game's constant tables (for a zero `arg` two 0x20-byte tables are also
/// copied to the stack), and `RiderBase_ResetAttackFX` clears the attack state.
pub fn rider_base_ctor_attack_fx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save(c, S1, 0x110);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x100);
        if c.g[S1][0] != 0 {
            c.cycles += 4;
            let sub = sx((c.g[S0][0] as u32).wrapping_add(0xb0));
            c.g[V0][0] = sub;
            c.g[A0][0] = sub;
            c.g[RA][0] = sx(0x002f_0eb4);
            Addr(c.g[S0][0] as u32).sw(c, sub);
            call_ptr(c, 0x0011_b698, 0x002f_0eb4)?;
        }

        c.cycles += 15;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = 0x490000;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7828));
        c.g[A0][0] = 0x460000;
        let table_a = c.g[A2][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x6e8)).sw(c, table_a);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
        c.g[V1][0] = 0x490000;
        c.g[A3][0] = c.g[V0][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[T2][0] = c.g[V1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x77e0));
        let fn_ptr = c.g[A0][0];
        Addr((c.g[A1][0] as u32).wrapping_add(0x6d0)).sw(c, fn_ptr);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let t = c.g[S1][0] != 0;
        let table_b = c.g[V1][0];
        Addr((c.g[V0][0] as u32).wrapping_add(0x6c0)).sw(c, table_b);

        if !t {
            c.cycles += 4;
            c.g[A1][0] = c.g[A2][0];
            c.g[V1][0] = c.g[SP][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
            c.g[T0][0] = sx(sp(c).wrapping_add(0x50));
            // Copy the first 0x40-byte table to the stack (two 0x20-byte rows).
            loop {
                c.cycles += 11;
                let src = c.g[A1][0] as u32;
                c.g[A0][0] = Addr(src).ld(c);
                c.g[A2][0] = Addr(src.wrapping_add(8)).ld(c);
                c.g[T1][0] = Addr(src.wrapping_add(0x10)).ld(c);
                c.g[T3][0] = Addr(src.wrapping_add(0x18)).ld(c);
                let dst = c.g[V1][0] as u32;
                let (w0, w1, w2, w3) = (c.g[A0][0], c.g[A2][0], c.g[T1][0], c.g[T3][0]);
                Addr(dst).sd(c, w0);
                Addr(dst.wrapping_add(8)).sd(c, w1);
                Addr(dst.wrapping_add(0x10)).sd(c, w2);
                Addr(dst.wrapping_add(0x18)).sd(c, w3);
                c.g[A1][0] = sx(src.wrapping_add(0x20));
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V1][0] = sx(dst.wrapping_add(0x20));
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_0f00);
                }
            }
            c.cycles += 14;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7828));
            c.g[A2][0] = Addr(c.g[A1][0] as u32).ld(c);
            let w = c.g[A2][0];
            Addr(c.g[V1][0] as u32).sd(c, w);
            let sp_word = c.g[SP][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x6e8)).sw(c, sp_word);
            c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x77e0));
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lhu(c);
            c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = c.g[T0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb0));
            c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            let hw = c.g[V1][0];
            Addr(sp(c).wrapping_add(8)).sh(c, hw);
            // Copy the second table (0x488820..0x4888c0) to the stack.
            loop {
                c.cycles += 11;
                let src = c.g[A2][0] as u32;
                c.g[V0][0] = Addr(src).ld(c);
                c.g[V1][0] = Addr(src.wrapping_add(8)).ld(c);
                c.g[A0][0] = Addr(src.wrapping_add(0x10)).ld(c);
                c.g[T3][0] = Addr(src.wrapping_add(0x18)).ld(c);
                let dst = c.g[A1][0] as u32;
                let (w0, w1, w2, w3) = (c.g[V0][0], c.g[V1][0], c.g[A0][0], c.g[T3][0]);
                Addr(dst).sd(c, w0);
                Addr(dst.wrapping_add(8)).sd(c, w1);
                Addr(dst.wrapping_add(0x10)).sd(c, w2);
                Addr(dst.wrapping_add(0x18)).sd(c, w3);
                c.g[A2][0] = sx(src.wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[T1][0];
                c.g[A1][0] = sx(dst.wrapping_add(0x20));
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_0f64);
                }
            }
            c.cycles += 10;
            c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x77e0));
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lhu(c);
            c.g[V0][0] = Addr(c.g[A2][0] as u32).ld(c);
            c.g[T1][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).ld(c);
            let w0 = c.g[V0][0];
            let w1 = c.g[T1][0];
            Addr(c.g[A1][0] as u32).sd(c, w0);
            Addr((c.g[A1][0] as u32).wrapping_add(8)).sd(c, w1);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            let t0 = c.g[T0][0];
            Addr((c.g[A0][0] as u32).wrapping_add(0x6c0)).sw(c, t0);
            let hw = c.g[V1][0];
            Addr(sp(c).wrapping_add(0x58)).sh(c, hw);
        }

        c.cycles += 12;
        c.g[V0][0] = 3;
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x002f_1148,
            f: ee::s09::f_002f1148,
        }
        .call(c, 0x002f_0fd0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x110);
        frame.restore(c, S0, 0x120);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    });
}

/// `CameraTriggerMan_ReadTriggerVolume` (0x16c4c8; the reference's `readVolume`):
/// reads the trigger's volume through `CameraTrigger_ReadSphereTrigger`
/// (0x172e18). Returns 0 when that read reports a volume, else 4.
pub fn camera_trigger_man_read_trigger_volume(c: &mut Cpu) {
    const RET: u32 = 0x0016_c4d8;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0017_2e18,
            f: ee::s02::f_00172e18,
        }
        .call(c, RET)?;
        c.g[V1][0] = 0x4;
        frame.restore_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = 0;
        }
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    });
}

/// `void func_0040A440(void *recvBlock)` (0x40a440): copies three words of the
/// received block (through the uncached alias `recvBlock | 0x20000000`) into the
/// globals at 0x5271e8, 0x5271ec and 0x5271f0, each one only when its global
/// holds a pointer. Leaves `recvBlock | 0x20000000` in a0.
pub fn func_0040a440(c: &mut Cpu) {
    const GLOBALS: u32 = 0x0052_0000;
    c.cycles += 5;
    c.g[V0][0] = GLOBALS as u64;
    c.g[V1][0] = 0x2000_0000;
    c.g[A1][0] = Addr(GLOBALS + 0x71e8).lw(c);
    let block = c.g[A0][0] | c.g[V1][0];
    c.g[A0][0] = block;
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(block as u32).lw(c);
        let word = c.g[V0][0];
        Addr(c.g[A1][0] as u32).sw(c, word);
    }

    c.cycles += 4;
    c.g[V0][0] = GLOBALS as u64;
    c.g[V1][0] = Addr(GLOBALS + 0x71ec).lw(c);
    let skip = c.g[V1][0] == 0;
    c.g[V0][0] = GLOBALS as u64;
    if !skip {
        c.cycles += 3;
        c.g[V0][0] = Addr((block as u32).wrapping_add(4)).lw(c);
        let word = c.g[V0][0];
        Addr(c.g[V1][0] as u32).sw(c, word);
        c.g[V0][0] = GLOBALS as u64;
    }

    c.cycles += 3;
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x71f0)).lw(c);
    if c.g[V1][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr((block as u32).wrapping_add(0x90)).lw(c);
        let word = c.g[V0][0];
        Addr(c.g[V1][0] as u32).sw(c, word);
    }
    c.cycles += 2;
    ret(c);
}

/// `cFEInput_getState` (0x17fdf0; the reference's switch over the key): the
/// signed low byte of `a1` selects an input id: keys 0..7 give 0x72..0x79 (the
/// case labels 0x17fe2c..0x17fe64 of the game's jump table at 0x45d530), any
/// other key gives 0. The id goes to the shared tail (0x17fe68), which calls
/// `cInput_getState(this->mpInput, id)` at 0x320c48 with `this` from the entry
/// a0 (kept in a2), then returns to the caller. Fully native: the case labels
/// are the game's constants, so no table read is needed.
pub fn cfe_input_get_state(c: &mut Cpu) {
    const JUMP_TABLE: u32 = 0x0045_d530;
    const CASE_LABELS: [u32; 8] = [
        0x0017_fe2c,
        0x0017_fe34,
        0x0017_fe3c,
        0x0017_fe44,
        0x0017_fe4c,
        0x0017_fe54,
        0x0017_fe5c,
        0x0017_fe64,
    ];
    const SHARED_TAIL: u32 = 0x0017_fe68;
    const SHARED_TAIL_RET: u32 = 0x0017_fe70;

    guest(c, |c| {
        let key = ((c.g[A1][0] as u32) << 24) as i32 >> 24;
        let case = usize::try_from(key).ok().filter(|&k| k < 8);
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A0][0];
        c.g[V1][0] = sx(key as u32);
        c.g[V0][0] = case.is_some() as u64;
        c.g[A1][0] = 0;
        if let Some(k) = case {
            // The switch's dispatch block, then the case block (2 cycles; key 7's
            // case is the 0x17fe64 block, one cycle less before the shared tail).
            c.cycles += 7;
            c.g[V0][0] = sx(JUMP_TABLE);
            c.g[V1][0] = sx(((k as u32) << 2).wrapping_add(JUMP_TABLE));
            c.g[A0][0] = sx(CASE_LABELS[k]);
            c.cycles += if k == 7 { 1 } else { 2 };
            c.g[A1][0] = 0x72 + k as u64;
        }
        // Shared tail at 0x17fe68.
        c.cycles += 5;
        c.g[RA][0] = sx(SHARED_TAIL_RET);
        c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x18)).lw(c);
        call_ptr(c, 0x0032_0c48, SHARED_TAIL_RET)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate and mailbox words of the test fakes (one pair per fake: a fake's
    /// gate must be its own word, see the writer brief).
    const OC1494_MAGIC: u32 = 0x1494_0c0d;
    const OC1494_SPHERE_GATE: u32 = DATA + 0x2000;
    const OC1494_SPHERE_RESULT: u32 = DATA + 0x2004;
    const OC1494_FX_GATE: u32 = DATA + 0x2010;
    const OC1494_INPUT_GATE: u32 = DATA + 0x2020;

    static OC1494_PREV_SPHERE: AtomicUsize = AtomicUsize::new(0);
    static OC1494_PREV_FX_CALL: AtomicUsize = AtomicUsize::new(0);
    static OC1494_PREV_INPUT_MAP: AtomicUsize = AtomicUsize::new(0);

    /// `CameraTrigger_ReadSphereTrigger` (0x172e18) with a forced "volume read"
    /// result: 0 or 1 in v0 (the real body allocates and reads the cursor).
    fn oc1494_fake_sphere(c: &mut Cpu) {
        if c.bus.read32(OC1494_SPHERE_GATE) != OC1494_MAGIC {
            return fakes::fall_back(c, &OC1494_PREV_SPHERE, ee::s02::f_00172e18__generated);
        }
        let ra = c.g[RA][0] as u32;
        let read = c.bus.read32(OC1494_SPHERE_RESULT) != 0;
        fakes::fake(c, 0x0017_2e18, ra);
        if c.pc == ra {
            c.g[V0][0] = read as u64;
        }
    }

    /// `0x11b698`, the sub-object constructor called by `RiderBase_CtorAttackFX`:
    /// logged and returned to, its result is not used.
    fn oc1494_fake_fx_call(c: &mut Cpu) {
        if c.bus.read32(OC1494_FX_GATE) != OC1494_MAGIC {
            return fakes::fall_back(c, &OC1494_PREV_FX_CALL, ee::s00::f_0011b698__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0011_b698, ra);
    }

    /// `InputMap_EvalBool`'s caller `0x320c48` (the tail of `cFEInput_getState`):
    /// logged and returned to.
    fn oc1494_fake_input_map(c: &mut Cpu) {
        if c.bus.read32(OC1494_INPUT_GATE) != OC1494_MAGIC {
            return fakes::fall_back(c, &OC1494_PREV_INPUT_MAP, ee::s10::f_00320c48__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_0c48, ra);
    }

    /// Fixture of the three one-store labels: `t0` points at scratch memory,
    /// the loop count `a1` is zero so the shared tail returns at once.
    fn oc1494_tail_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[T0][0] = sx(DATA + 0x100);
        c.g[A1][0] = 0;
        c.f[1] = r.next_u32();
    }

    #[test]
    fn lbl_003a3938_matches() {
        check(ee::s13::f_003a3938__generated, lbl_003a3938, 200, &|c, seed| {
            oc1494_tail_setup(c, seed, 0x003a_3938)
        });
    }

    #[test]
    fn lbl_003a3940_matches() {
        check(ee::s13::f_003a3940__generated, lbl_003a3940, 200, &|c, seed| {
            oc1494_tail_setup(c, seed, 0x003a_3940)
        });
    }

    #[test]
    fn lbl_003a39d4_matches() {
        check(ee::s13::f_003a39d4__generated, lbl_003a39d4, 200, &|c, seed| {
            oc1494_tail_setup(c, seed, 0x003a_39d4)
        });
    }

    #[test]
    fn rider_base_ctor_attack_fx_matches() {
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0011B698),
            oc1494_fake_fx_call,
            &OC1494_PREV_FX_CALL,
        );
        check(
            ee::s09::f_002f0e88__generated,
            rider_base_ctor_attack_fx,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002f_0e88, 2);
                c.bus.write32(OC1494_FX_GATE, OC1494_MAGIC);
                let this = DATA + 0x4000;
                // `this` (a0) and `arg` (a1): only its zero test matters.
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    sx(r.next_u32() | 1)
                };
                fill(c, &mut r, this + 4, 0x100);
                fill(c, &mut r, DATA + 0x6000, 0x200);
                // The two constant tables the copy loops read (0x487828.., 0x488820..).
                fill(c, &mut r, 0x0048_7800, 0x80);
                fill(c, &mut r, 0x0048_8800, 0x80);
                // `*this` is the pointer the stores go through.
                c.bus.write32(this, DATA + 0x6000);
            },
        );
    }

    #[test]
    fn camera_trigger_man_read_trigger_volume_matches() {
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00172E18),
            oc1494_fake_sphere,
            &OC1494_PREV_SPHERE,
        );
        // calls = 1: the fake may unwind, which the wrapper must pass through.
        check(
            ee::s02::f_0016c4c8__generated,
            camera_trigger_man_read_trigger_volume,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x0016_c4c8, 1);
                c.bus.write32(OC1494_SPHERE_GATE, OC1494_MAGIC);
                c.bus.write32(OC1494_SPHERE_RESULT, seed & 1);
            },
        );
    }

    #[test]
    fn func_0040a440_matches() {
        check(ee::s15::f_0040a440__generated, func_0040a440, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0040_a440, 0);
            let block = DATA + 0x400;
            fill(c, &mut r, block, 0x40);
            c.g[A0][0] = sx(block);
            // Each global holds a destination pointer or zero (every combination).
            for (i, off) in [0x71e8_u32, 0x71ec, 0x71f0].into_iter().enumerate() {
                let dst = if (seed >> i) & 1 != 0 {
                    DATA + 0x1000 + 0x100 * i as u32
                } else {
                    0
                };
                c.bus.write32(0x0052_0000 + off, dst);
            }
        });
    }

    /// The `jump_out` hook for `cFEInput_getState`: the generated body leaves
    /// through its switch (`jr` to a table label); run the label's real code
    /// (the case blocks and the shared tail) instead of logging a fake call.
    fn oc1494_jump_out(c: &mut Cpu, pc: u32, start: u32, end: u32) {
        if (0x0017_fe2c..0x0017_fe68).contains(&pc) {
            c.pc = pc;
            call_indirect(c, pc);
        } else {
            fakes::fake_jump_out(c, pc, start, end);
        }
    }

    #[test]
    fn cfe_input_get_state_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320C48),
            oc1494_fake_input_map,
            &OC1494_PREV_INPUT_MAP,
        );
        check(
            ee::s02::f_0017fdf0__generated,
            cfe_input_get_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0017_fdf0, 1);
                c.bus.write32(OC1494_INPUT_GATE, OC1494_MAGIC);
                c.hooks.jump_out = oc1494_jump_out;
                // The game's switch jump table (0x45d530) is data the generated body
                // reads; the test RAM does not hold it, so install the real labels.
                for (i, label) in [
                    0x0017_fe2c_u32,
                    0x0017_fe34,
                    0x0017_fe3c,
                    0x0017_fe44,
                    0x0017_fe4c,
                    0x0017_fe54,
                    0x0017_fe5c,
                    0x0017_fe64,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(0x0045_d530 + 4 * i as u32, label);
                }
                // Keys -2..=9: negative, the eight table cases, and the default.
                let key = (seed % 12) as i32 - 2;
                let a1 = (r.next_u32() & !0xff) | (key as u8 as u32);
                c.g[A1][0] = sx(a1);
                // `this` (a0); its mpInput pointer is read by the tail.
                c.g[A0][0] = sx(DATA + 0x400);
                c.bus.write32(DATA + 0x400 + 0x18, DATA + 0x800);
            },
        );
    }
}
