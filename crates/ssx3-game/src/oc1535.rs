//! Staging module oc1535 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1535`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0011_3e50,
        race_state_shutdown_enter_00113e50,
        "RaceStateShutdown_Enter",
    ),
    (
        0x0011_6e08,
        trick_get_bonus_trick_name_00116e08,
        "Trick_GetBonusTrickName",
    ),
    (
        0x0011_7138,
        chudfx_show_delayed_00117138,
        "cHUDFX_ShowDelayed",
    ),
    (0x0011_7290, rider_metrix_dtor_00117290, "RiderMetrix_dtor"),
    (
        0x0014_c548,
        bolts_set_bolt_field10_0014c548,
        "Bolts_SetBoltField10",
    ),
    (
        0x0014_c590,
        bolts_set_bolt_field1_0014c590,
        "Bolts_SetBoltField1",
    ),
];

/// `void RaceStateShutdown_Enter(RaceState* self, AI* ai)` (0x113e50): calls
/// `AI_DestroyAllRiders(ai)` (the second argument moves into `a0`), then
/// returns. A frame of 0x10 holding `ra`.
pub fn race_state_shutdown_enter_00113e50(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        Callee {
            addr: 0x0012_b090,
            f: ee::s00::AI_DestroyAllRiders,
        }
        .call(c, 0x0011_3e60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Trick_GetBonusTrickName(char* buf, unsigned trick, int size)` (0x116e08):
/// searches `gMonsterComboTable` (24 entries of 0x10 bytes at 0x43d608: the
/// trick id word first, the request fields at +8..+0xe) for `trick`. On a
/// match it packs the entry's request fields into a 64-bit trick id on the
/// stack (`MEM_fill` zeroes it first), then `Trick_BuildName` (0x116950)
/// builds the name into `buf`; without a match `buf` becomes "".
///
/// The bit packing is the generated op sequence, kept op for op: the masks
/// and shifts place the fields of the trick id word.
pub fn trick_get_bonus_trick_name_00116e08(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x0044_0000;
        frame.save(c, S0, 0x40);
        c.g[V1][0] = 0;
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x29f8));
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A2][0];
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
            let t = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            if !t {
                // Match: pack the request fields into the id word on the stack.
                c.cycles += 70;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0x8;
                call_ptr(c, 0x003e_6448, 0x0011_6e4c)?; // EA_MemSet(&id, 0, 8)
                c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0xffff_ffff_ffff_f3ff;
                c.g[V1][0] = c.ld((c.g[SP][0] as u32));
                c.g[A2][0] = 0xffff_ffff_ffff_0000;
                c.g[A2][0] = c.g[A2][0] | 0xfff;
                c.g[V0][0] = c.g[V0][0] & 0x3;
                c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x9));
                c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] << 10;
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0xf;
                c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
                c.g[A0][0] = c.g[A0][0] << 12;
                c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
                c.g[A1][0] = 0xffff_ffff_fff8_0000;
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xb));
                c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] << 16;
                c.g[A1][0] = 0xffff_ffff_ffc7_0000;
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x7;
                c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                c.g[A0][0] = c.g[A0][0] << 19;
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                c.g[A1][0] = c.g[A1][0] << 16;
                c.g[A1][0] = c.g[A1][0] | 0xfff;
                c.g[A1][0] = c.g[A1][0] << 16;
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                c.g[V0][0] = c.g[V0][0] & 0xf;
                c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xd));
                c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] << 28;
                c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xe));
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x7f;
                c.g[V0][0] = 0xffff_ffff_ffff_fc07;
                c.g[V0][0] = c.g[V0][0] << 16;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.g[V0][0] = c.g[V0][0] << 16;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.g[A0][0] = c.g[A0][0] << 35;
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0x7f;
                c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                c.g[V0][0] = 0xffff_ffff_fffc_0000;
                c.g[V0][0] = c.g[V0][0] | 0x7ff;
                c.g[V0][0] = c.g[V0][0] << 16;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.g[V0][0] = c.g[V0][0] << 16;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.g[A0][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] << 43;
                c.g[A0][0] = c.g[A0][0] | c.g[A1][0];
                c.g[V1][0] = 0x30000;
                c.sd((c.g[SP][0] as u32), c.g[A0][0]);
                c.g[V1][0] = c.g[V1][0] | 0xf800;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                if c.g[V0][0] == 0u64 {
                    c.g[A0][0] = c.g[S1][0];
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = 0x8000;
                    c.g[V0][0] = c.g[V0][0] << 27;
                    c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
                    c.sd((c.g[SP][0] as u32), c.g[V0][0]);
                    c.g[A0][0] = c.g[S1][0];
                }
                c.cycles += 5;
                c.g[A2][0] = c.g[S2][0];
                c.g[A1][0] = c.g[SP][0];
                Callee {
                    addr: 0x0011_6950,
                    f: ee::s00::Trick_BuildName,
                }
                .call(c, 0x0011_6f74)?;
                frame.restore(c, S0, 0x40);
                break;
            }
            // No match at this entry: next entry, or give up at the table end.
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 24i64) as u64;
            let more = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0011_6e30);
                }
                continue;
            }
            c.cycles += 2;
            c.sb((c.g[S1][0] as u32), 0u64);
            frame.restore(c, S0, 0x40);
            break;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cHUDFX_ShowDelayed(cHUDFX* fx, int fxId, char* text, int data, float decay)`
/// (0x117138): sets the text, then the id, data, decay (`-decay`, the current
/// decay), `MaxDecay = -1.0f` and `DisplayInt = 0`.
pub fn chudfx_show_delayed_00117138(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x40);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        c.g[A1][0] = c.g[A2][0];
        Callee {
            addr: 0x0011_7008,
            f: ee::s00::cHUDFX_SetText,
        }
        .call(c, 0x0011_7168)?;
        c.neg_s(20, 20);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        c.sw((c.g[S0][0] as u32), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[S2][0]);
        c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x8));
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `cRiderMetrix::~cRiderMetrix(this, flags)` (0x117290): frees the HUD
/// effect object (`this+0x1b0`) if there is one, then deletes `this` when
/// bit 0 of `flags` is set.
pub fn rider_metrix_dtor_00117290(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x1b0).lw(c);
        c.g[S1][0] = c.g[A1][0];
        if c.g[A0][0] != 0u64 {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x0011_72b8)?; // EA_MemFree(HUDfx)
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] == 0u64 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0011_72cc)?; // __builtin_delete(this)
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Bolts_SetBoltField1` / `Bolts_SetBoltField10` (0x14c590, 0x14c548): stores
/// the low byte of `value` at `+field` of the table entry for `(group, key)`,
/// the entry index from `Bolts_FindBoltIndex` (0x14d448) times 0x38 plus the
/// table base at `self+4`. The two differ only in the field offset and in the
/// return address of the index call.
fn bolts_set_bolt_field(c: &mut Cpu, ret_find: u32, field: u32) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A3][0];
        Callee {
            addr: 0x0014_d448,
            f: ee::s01::Bolts_FindBoltIndex,
        }
        .call(c, ret_find)?;
        c.g[V1][0] = 0x38;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
        c.mult(0, 2, 2, 3);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sb((c.g[V0][0] as u32).wrapping_add(field), c.g[S1][0]);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Bolts_SetBoltField1(sBEGroupTable* self, int group, int key, char value)`
/// (0x14c590): the entry's byte at +0x1.
pub fn bolts_set_bolt_field1_0014c590(c: &mut Cpu) {
    bolts_set_bolt_field(c, 0x0014_c5ac, 0x1)
}

/// `void Bolts_SetBoltField10(sBEGroupTable* self, int group, int key, char value)`
/// (0x14c548): the entry's byte at +0x10.
pub fn bolts_set_bolt_field10_0014c548(c: &mut Cpu) {
    bolts_set_bolt_field(c, 0x0014_c564, 0x10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Magic word a gate holds while its fake is on (the machine's RAM).
    const OC1535_MAGIC: u32 = 0x1535_c0de;
    const OC1535_AI_GATE: u32 = DATA + 0x20_1000;
    const OC1535_TEXT_GATE: u32 = DATA + 0x20_1100;
    const OC1535_FREE_GATE: u32 = DATA + 0x20_1200;
    const OC1535_DELETE_GATE: u32 = DATA + 0x20_1300;
    const OC1535_BOLT_GATE: u32 = DATA + 0x20_1400;
    const OC1535_MEMSET_GATE: u32 = DATA + 0x20_1500;
    const OC1535_BUILD_GATE: u32 = DATA + 0x20_1600;

    static OC1535_AI_PREV: AtomicUsize = AtomicUsize::new(0);
    static OC1535_TEXT_PREV: AtomicUsize = AtomicUsize::new(0);
    static OC1535_FREE_PREV: AtomicUsize = AtomicUsize::new(0);
    static OC1535_DELETE_PREV: AtomicUsize = AtomicUsize::new(0);
    static OC1535_BOLT_PREV: AtomicUsize = AtomicUsize::new(0);
    static OC1535_MEMSET_PREV: AtomicUsize = AtomicUsize::new(0);
    static OC1535_BUILD_PREV: AtomicUsize = AtomicUsize::new(0);

    /// `AI_DestroyAllRiders` (0x12b090), gated: logs the call when the gate is on.
    fn oc1535_fake_destroy_riders(c: &mut Cpu) {
        if c.bus.read32(OC1535_AI_GATE) != OC1535_MAGIC {
            return fakes::fall_back(c, &OC1535_AI_PREV, ee::s00::f_0012b090__generated);
        }
        fakes::fake_call(c, 0x0012_b090);
    }

    /// `cHUDFX_SetText` (0x117008), gated.
    fn oc1535_fake_set_text(c: &mut Cpu) {
        if c.bus.read32(OC1535_TEXT_GATE) != OC1535_MAGIC {
            return fakes::fall_back(c, &OC1535_TEXT_PREV, ee::s00::f_00117008__generated);
        }
        fakes::fake_call(c, 0x0011_7008);
    }

    /// `EA_MemFree` (0x317e98), gated.
    fn oc1535_fake_free(c: &mut Cpu) {
        if c.bus.read32(OC1535_FREE_GATE) != OC1535_MAGIC {
            return fakes::fall_back(c, &OC1535_FREE_PREV, ee::s10::f_00317e98__generated);
        }
        fakes::fake_call(c, 0x0031_7e98);
    }

    /// `__builtin_delete` (0x317e50), gated.
    fn oc1535_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC1535_DELETE_GATE) != OC1535_MAGIC {
            return fakes::fall_back(c, &OC1535_DELETE_PREV, ee::s10::f_00317e50__generated);
        }
        fakes::fake_call(c, 0x0031_7e50);
    }

    /// `Bolts_FindBoltIndex` (0x14d448), gated.
    fn oc1535_fake_find_bolt(c: &mut Cpu) {
        if c.bus.read32(OC1535_BOLT_GATE) != OC1535_MAGIC {
            return fakes::fall_back(c, &OC1535_BOLT_PREV, ee::s01::f_0014d448__generated);
        }
        fakes::fake_call(c, 0x0014_d448);
    }

    /// `EA_MemSet` (0x3e6448), gated.
    fn oc1535_fake_mem_set(c: &mut Cpu) {
        if c.bus.read32(OC1535_MEMSET_GATE) != OC1535_MAGIC {
            return fakes::fall_back(c, &OC1535_MEMSET_PREV, ee::s14::f_003e6448__generated);
        }
        fakes::fake_call(c, 0x003e_6448);
    }

    /// `Trick_BuildName` (0x116950), gated.
    fn oc1535_fake_build_name(c: &mut Cpu) {
        if c.bus.read32(OC1535_BUILD_GATE) != OC1535_MAGIC {
            return fakes::fall_back(c, &OC1535_BUILD_PREV, ee::s00::f_00116950__generated);
        }
        fakes::fake_call(c, 0x0011_6950);
    }

    #[test]
    fn race_state_shutdown_enter_00113e50_matches() {
        check(
            ee::s00::f_00113e50__generated,
            race_state_shutdown_enter_00113e50,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113e50, 1);
                fakes::hook(
                    addr_of_mut!(ee::s00::OVERRIDE_0012B090),
                    oc1535_fake_destroy_riders,
                    &OC1535_AI_PREV,
                );
                c.bus.write32(OC1535_AI_GATE, OC1535_MAGIC);
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    #[test]
    fn trick_get_bonus_trick_name_00116e08_matches() {
        check(
            ee::s00::f_00116e08__generated,
            trick_get_bonus_trick_name_00116e08,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x116e08, 2);
                fakes::hook(
                    addr_of_mut!(ee::s14::OVERRIDE_003E6448),
                    oc1535_fake_mem_set,
                    &OC1535_MEMSET_PREV,
                );
                fakes::hook(
                    addr_of_mut!(ee::s00::OVERRIDE_00116950),
                    oc1535_fake_build_name,
                    &OC1535_BUILD_PREV,
                );
                c.bus.write32(OC1535_MEMSET_GATE, OC1535_MAGIC);
                c.bus.write32(OC1535_BUILD_GATE, OC1535_MAGIC);
                // The monster combo table in the game image (0x43d608): 24
                // entries, the id word first, request bytes at +8..+0xe. One
                // entry matches the trick (seed % 25 == 24: none does).
                let table = 0x0043_d608;
                let trick = 0x0100 + seed % 0x40;
                let hit = seed % 25;
                for i in 0..24u32 {
                    let e = table + 0x10 * i;
                    let id = if i == hit { trick } else { 0x0200 + i };
                    c.bus.write32(e, id);
                    for off in 8..0xf {
                        c.bus.write8(e + off, r.next_u32() as u8);
                    }
                }
                c.g[A0][0] = sx(DATA + 0x5000);
                c.g[A1][0] = sx(trick);
                c.g[A2][0] = sx(0x40 + seed % 8);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    #[test]
    fn chudfx_show_delayed_00117138_matches() {
        check(
            ee::s00::f_00117138__generated,
            chudfx_show_delayed_00117138,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x117138, 1);
                fakes::hook(
                    addr_of_mut!(ee::s00::OVERRIDE_00117008),
                    oc1535_fake_set_text,
                    &OC1535_TEXT_PREV,
                );
                c.bus.write32(OC1535_TEXT_GATE, OC1535_MAGIC);
                fill(c, &mut r, DATA + 0x3000, 0x10);
                c.g[A0][0] = sx(DATA + 0x3000);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(DATA + 0x3100);
                c.g[A3][0] = sx(r.next_u32());
                c.f[12] = r.f32(100.0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    #[test]
    fn rider_metrix_dtor_00117290_matches() {
        check(
            ee::s00::f_00117290__generated,
            rider_metrix_dtor_00117290,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x117290, 2);
                fakes::hook(
                    addr_of_mut!(ee::s10::OVERRIDE_00317E98),
                    oc1535_fake_free,
                    &OC1535_FREE_PREV,
                );
                fakes::hook(
                    addr_of_mut!(ee::s10::OVERRIDE_00317E50),
                    oc1535_fake_delete,
                    &OC1535_DELETE_PREV,
                );
                c.bus.write32(OC1535_FREE_GATE, OC1535_MAGIC);
                c.bus.write32(OC1535_DELETE_GATE, OC1535_MAGIC);
                let metrix = DATA + 0x6000;
                fill(c, &mut r, metrix, 0x80);
                // HUD effect object present on 2 seeds in 3; the flags bit 0 on half.
                let hud = if seed % 3 == 0 { 0 } else { DATA + 0x7000 };
                c.bus.write32(metrix + 0x1b0, hud);
                c.g[A0][0] = sx(metrix);
                c.g[A1][0] = sx(r.next_u32());
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// The shared setup of the two `Bolts_SetBoltField*` tests: the index
    /// call is a gated fake that answers a small entry index.
    fn bolt_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 1);
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014D448),
            oc1535_fake_find_bolt,
            &OC1535_BOLT_PREV,
        );
        c.bus.write32(OC1535_BOLT_GATE, OC1535_MAGIC);
        c.bus.write32(fakes::LOG_RESULTS, 1 + seed % 0x20);
        let this = DATA + 0x3000;
        c.bus.write32(this + 4, DATA + 0x4000);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(r.next_u32() % 4);
        c.g[A2][0] = sx(r.next_u32());
        c.g[A3][0] = sx(r.next_u32());
        c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
    }

    #[test]
    fn bolts_set_bolt_field1_0014c590_matches() {
        check(
            ee::s01::f_0014c590__generated,
            bolts_set_bolt_field1_0014c590,
            200,
            &|c, seed| bolt_setup(c, seed, 0x14c590),
        );
    }

    #[test]
    fn bolts_set_bolt_field10_0014c548_matches() {
        check(
            ee::s01::f_0014c548__generated,
            bolts_set_bolt_field10_0014c548,
            200,
            &|c, seed| bolt_setup(c, seed, 0x14c548),
        );
    }
}
