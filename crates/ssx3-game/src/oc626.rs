//! Staging module oc626 (wave g06.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc626`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_41b8, danger_man_get_rider_danger_level, "DangerMan_GetRiderDangerLevel"),
    (0x0014_6070, bne_player_int_set_rider_state_flag, "BENewPlayerInt_SetRiderStateFlag"),
    (0x0014_6150, bne_player_int_get_peak_flag, "BENewPlayerInt_GetPeakFlag"),
];

/// `cBENewPlayerInterface_tempPlayers` (0x1c-byte entries; character id at
/// +0x11) and the slot table that `BENewPlayerInt_GetSlot` reads.
#[cfg(test)]
pub(crate) const BNE_TEMP_PLAYERS: u32 = 0x0053_4fe0;

/// `BE_GetRiderProfile(index)` (0x14a0e0).
pub(crate) const BE_GET_RIDER_PROFILE: Callee = Callee {
    addr: 0x0014_a0e0,
    f: ee::s01::BE_GetRiderProfile,
};

/// `BENewPlayerInt_GetSlot(index)` (0x147798).
pub(crate) const BNE_GET_SLOT: Callee = Callee {
    addr: 0x0014_7798,
    f: ee::s01::BENewPlayerInt_GetSlot,
};

/// `(word << shift) >> 32 & 1`: bit `32 - shift` of the word's low half, as
/// the generated code extracts the peak flags.
#[inline]
pub(crate) fn bne_shifted_bit(word: u64, shift: u32) -> u64 {
    (((word << shift) as i64) >> 32) as u64 & 1
}

/// `DangerMan_GetRiderDangerLevel(this, pl)` (0x1441b8, `cDangerMan::
/// CalcAppliedDangerLevel`): the danger level applied to rider `pl`. It starts
/// from the rider's base level (`mRiderDangerTrack[pl].baseLevel`), raised to
/// an avalanche band when one is active near the rider, or to 1 for a
/// tracked, unsafe zone element around it.
pub fn danger_man_get_rider_danger_level(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        c.g[V1][0] = 0xc;
        frame.save(c, S1, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.mult(0, A0, A1, V1); // pl * 12: the track entry stride
        frame.save(c, S2, 0x40);
        frame.save_ra(c, 0x30);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2); // pl * 4: Riders[pl]
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c); // mpAI
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[S2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x70c)).lw(c); // lvl
        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c); // rider
        if c.g[S2][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            danger_level_search(c)?;
        }
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        c.cycles += 6;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// The part of `DangerMan_GetRiderDangerLevel` after the base level is known
/// to be non-zero. Sets `V0` to the result and returns (the caller runs the
/// epilogue); an unwinding callee propagates.
fn danger_level_search(c: &mut Cpu) -> Flow {
    // rider->getPosition(): a virtual call through the rider's vtable.
    c.cycles += 10;
    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x6c0)).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0)); // pos (this)
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lh(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2c)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0014_4214)?;
    c.g[S0][0] = c.g[V0][0]; // S0 = pos
    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x708)).lw(c); // mNumActiveAvalanche
    c.g[A0][0] = c.g[SP][0];
    if c.g[V0][0] != 0 {
        // Avalanche band: 3 within 1200.446 of the ground, 2 within 2999.697,
        // else 1. Raise to the band when the level is below it.
        c.cycles += 14;
        fpu_load(c, 1, S0, 0);
        c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
        fpu_store(c, 1, SP, 0x20);
        fpu_load(c, 0, S0, 0x4);
        fpu_store(c, 0, SP, 0x24);
        fpu_load(c, 1, S0, 0x8);
        fpu_store(c, 1, SP, 0x28);
        call_ptr(c, 0x002d_a1c0, 0x0014_4244)?; // cAvalanche_getAvalancheDataByPosition
        fpu_load(c, 1, SP, 0x14); // ground.m_distance
        fpu_load(c, 0, GP, 0u32.wrapping_sub(0x6e24)); // 1200.446f
        let near = fpu_lt(c, 1, 0);
        c.g[V1][0] = 1;
        if !near {
            c.cycles += 5;
            fpu_load(c, 0, GP, 0u32.wrapping_sub(0x6e20)); // 2999.697f
            if fpu_lt(c, 1, 0) {
                c.g[V1][0] = 2;
            }
        } else {
            c.cycles += 2;
            c.g[V1][0] = 3;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let band_wins = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0];
        if band_wins {
            return Ok(());
        }
    }
    // Base level below 2: 1.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[S2][0] as i64) < 2) as u64;
    let below_two = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if below_two {
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x704)).lw(c); // tracked element count
    let count_nonzero = c.g[V0][0] != 0;
    c.g[V1][0] = c.g[V0][0];
    if !count_nonzero {
        c.cycles += 2;
        c.g[V0][0] = 1;
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4)); // mTrackedDangerElems[0]
    c.g[A1][0] = 0; // i
    c.g[A2][0] = 2; // state of a skipped element
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lw(c);
    loop {
        // 'b_144334
        c.cycles += 2;
        if c.g[V0][0] == c.g[A2][0] {
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        } else if danger_element_step(c) {
            return Ok(());
        }
        // 'b_144348: the next element, while i < 64 and elements remain
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A1][0] as i64) < 64) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c));
        if in_range {
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0014_42b8);
                }
                continue;
            }
        }
        break;
    }
    c.cycles += 1;
    c.g[V0][0] = 1;
    Ok(())
}

/// One tracked danger element (`A0` points at it): the loop body of
/// `DangerMan_GetRiderDangerLevel`, testing the element's radius against the
/// rider's position (pos in S0). Returns true when the element returns the
/// level (`V0 = S2`); otherwise it advances the element index `A1`.
fn danger_element_step(c: &mut Cpu) -> bool {
    c.cycles += 10;
    fpu_load(c, 2, A0, 0xc); // z->pos.x
    fpu_load(c, 1, S0, 0); // pos.x
    fpu_load(c, 0, A0, 0); // z->radius
    let dx = FpuF32(c.f[2]) - FpuF32(c.f[1]);
    abi::set_fpr(c, 2, dx);
    let radius_sq = FpuF32(c.f[0]) * FpuF32(c.f[0]);
    abi::set_fpr(c, 3, radius_sq);
    let dist_sq = FpuF32(c.f[2]) * FpuF32(c.f[2]);
    abi::set_fpr(c, 2, dist_sq);
    let outside_x = fpu_lt(c, 3, 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1)); // one tracked element left
    if outside_x {
        c.cycles += 1;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        return false;
    }
    c.cycles += 8;
    fpu_load(c, 1, A0, 0x10); // z->pos.y
    fpu_load(c, 0, S0, 0x4); // pos.y
    let dy = FpuF32(c.f[1]) - FpuF32(c.f[0]);
    abi::set_fpr(c, 1, dy);
    let dy_sq = FpuF32(c.f[1]) * FpuF32(c.f[1]);
    abi::set_fpr(c, 1, dy_sq);
    if fpu_lt(c, 3, 1) {
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        return false;
    }
    c.cycles += 10;
    let sum_xy = FpuF32(c.f[2]) + FpuF32(c.f[1]);
    abi::set_fpr(c, 2, sum_xy);
    fpu_load(c, 0, A0, 0x14); // z->pos.z
    fpu_load(c, 1, S0, 0x8); // pos.z
    let dz = FpuF32(c.f[0]) - FpuF32(c.f[1]);
    abi::set_fpr(c, 0, dz);
    let dz_sq = FpuF32(c.f[0]) * FpuF32(c.f[0]);
    abi::set_fpr(c, 0, dz_sq);
    let dist_total = FpuF32(c.f[2]) + FpuF32(c.f[0]);
    abi::set_fpr(c, 2, dist_total);
    let inside = fpu_lt(c, 3, 2);
    c.g[V0][0] = c.g[S2][0];
    if !inside {
        return true;
    }
    c.cycles += 1;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    false
}

/// `f[ft] = *(float*)(gpr[base] + off)` (`lwc1`).
#[inline]
fn fpu_load(c: &mut Cpu, ft: usize, base: usize, off: u32) {
    let a = (c.g[base][0] as u32).wrapping_add(off);
    let v = Addr(a).read_fpu(c);
    abi::set_fpr(c, ft, v);
}

/// `*(float*)(gpr[base] + off) = f[ft]` (`swc1`).
#[inline]
fn fpu_store(c: &mut Cpu, ft: usize, base: usize, off: u32) {
    let a = (c.g[base][0] as u32).wrapping_add(off);
    let v = FpuF32(c.f[ft]);
    Addr(a).write_fpu(c, v);
}

/// `c.lt.s f[a], f[b]` (sets the condition bit).
#[inline]
fn fpu_lt(c: &mut Cpu, a: usize, b: usize) -> bool {
    let x = FpuF32(c.f[a]);
    let y = FpuF32(c.f[b]);
    abi::c_lt_s(c, x, y)
}

/// `void cBENewPlayerInterface::setPeakLocked(this, index, kind, value)`
/// (0x146070, `BENewPlayerInt_SetRiderStateFlag`): stores `value & 1` into
/// bit 12 (kind 1) or bit 13 (other kinds) of the profile-library word at
/// +0x278 of the rider's profile and character.
pub fn bne_player_int_set_rider_state_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A1][0]; // index
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A2][0]; // kind
        frame.save(c, S2, 0x20);
        c.g[S3][0] = c.g[A3][0]; // value
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0014_609c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0]; // profile
        BNE_GET_SLOT.call(c, 0x0014_60a8)?;
        c.g[A0][0] = 0x1c;
        c.g[V1][0] = 0x53_0000;
        c.mult(0, V0, V0, A0); // slot * 0x1c
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fe0));
        c.g[A0][0] = 0x1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x11)).lb(c); // character
        c.g[A2][0] = c.g[A1][0];
        if c.g[S1][0] == c.g[A0][0] {
            c.cycles += 10;
            c.g[V1][0] = 0xf88;
            c.g[V0][0] = 0x9b50;
            c.mult(0, V1, A1, V1);
            c.mult(0, V0, S2, V0);
            c.g[A0][0] = 0x4a_0000;
            c.g[A1][0] = c.g[S3][0] & 0x1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
            c.g[A2][0] = 0xffff_ffff_ffff_efff;
            c.g[A1][0] <<= 12;
        } else {
            c.cycles += 9;
            c.g[V1][0] = 0xf88;
            c.g[V0][0] = 0x9b50;
            c.mult(0, V1, A2, V1);
            c.mult(0, V0, S2, V0);
            c.g[A0][0] = 0x4a_0000;
            c.g[A1][0] = c.g[S3][0] & 0x1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
            c.g[A2][0] = 0xffff_ffff_ffff_dfff;
            c.g[A1][0] <<= 13;
        }
        c.cycles += 13;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let entry = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(entry.wrapping_add(0x278)).ld(c);
        c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
        c.sd(entry.wrapping_add(0x278), c.g[V0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool cBENewPlayerInterface::getPeakFlag(this, index, kind, flag)`
/// (0x146150, `BENewPlayerInt_GetPeakFlag`): with `flag` set, the rival
/// time-challenge played bit of peak `kind` (bits 3..5 of the word at +0x278,
/// kinds 0..2); otherwise the rival points-challenge bit (bit 0 for kind 0,
/// bits 1..2 for kinds 1..2). Other kinds read as 0.
pub fn bne_player_int_get_peak_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A1][0]; // index
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A3][0]; // flag
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0]; // kind
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0014_617c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S3][0] = c.g[V0][0]; // profile
        BNE_GET_SLOT.call(c, 0x0014_6188)?;
        c.g[A0][0] = 0x1c;
        c.g[V1][0] = 0x53_0000;
        c.mult(0, V0, V0, A0); // slot * 0x1c
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fe0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x11)).lb(c); // character
        c.g[A2][0] = c.g[A1][0];
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            if c.g[S2][0] == 0 {
                c.cycles += 13;
                let entry = bne_profile_entry(c, A1);
                c.g[V0][0] = bne_shifted_bit(Addr(entry.wrapping_add(0x278)).ld(c), 29);
            } else {
                c.g[V0][0] = 1;
                c.cycles += 2;
                if c.g[S2][0] == c.g[V0][0] {
                    c.cycles += 13;
                    let entry = bne_profile_entry(c, A1);
                    c.g[V0][0] = bne_shifted_bit(Addr(entry.wrapping_add(0x278)).ld(c), 28);
                } else {
                    c.g[V0][0] = 2;
                    c.cycles += 2;
                    if c.g[S2][0] == c.g[V0][0] {
                        c.cycles += 13;
                        let entry = bne_profile_entry(c, A1);
                        c.g[V0][0] = bne_shifted_bit(Addr(entry.wrapping_add(0x278)).ld(c), 27);
                    } else {
                        c.g[V0][0] = 0;
                    }
                }
            }
        } else {
            c.cycles += 2;
            if c.g[S2][0] == 0 {
                c.cycles += 11;
                let entry = bne_profile_entry(c, A1);
                c.g[V0][0] = Addr(entry.wrapping_add(0x278)).lw(c) & 1;
            } else {
                c.g[V0][0] = 1;
                c.cycles += 2;
                if c.g[S2][0] == c.g[V0][0] {
                    c.cycles += 13;
                    let entry = bne_profile_entry(c, A1);
                    c.g[V0][0] = bne_shifted_bit(Addr(entry.wrapping_add(0x278)).ld(c), 31);
                } else {
                    c.g[V0][0] = 2;
                    c.cycles += 2;
                    if c.g[S2][0] == c.g[V0][0] {
                        c.cycles += 12;
                        let entry = bne_profile_entry(c, A2);
                        c.g[V0][0] = bne_shifted_bit(Addr(entry.wrapping_add(0x278)).ld(c), 30);
                    } else {
                        c.g[V0][0] = 0;
                    }
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The profile-library entry `cBELibrary_profileLib[profile][char]` of the
/// rider in `S3`, the character in `char_reg`, as the generated code forms
/// it: `char * 0xf88 + profile * 0x9b50 + 0x4a6ca8`. Leaves `A0 = profile *
/// 0x9b50` and `V0 = 0x4a6ca8` as the original does.
fn bne_profile_entry(c: &mut Cpu, char_reg: usize) -> u32 {
    c.g[V1][0] = 0xf88;
    c.g[A0][0] = 0x9b50;
    c.mult(0, V1, char_reg, V1);
    c.mult(0, A0, S3, A0);
    c.g[V0][0] = 0x4a_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// The slot table, the player entries and the character ids: small
    /// slot numbers and random entry words keep every lookup in RAM.
    fn bne_fixture(c: &mut Cpu, r: &mut Rng) {
        for i in 0..0x400u32 {
            c.bus.write32(0x0053_0000 + 4 * i, r.next_u32() % 0x40);
        }
        for i in 0..0x400u32 {
            c.bus.write32(BNE_TEMP_PLAYERS + 4 * i, r.next_u32());
        }
    }

    #[test]
    fn bne_player_int_set_rider_state_flag_matches() {
        check(
            ee::s01::f_00146070__generated,
            bne_player_int_set_rider_state_flag,
            300,
            &|c, seed| {
                let mut r = Rng(0x7a11_0070 ^ seed as u64);
                scramble(c, &mut r);
                bne_fixture(c, &mut r);
                c.pc = 0x0014_6070;
                c.g[A1][0] = sx(seed % 4); // index
                c.g[A2][0] = sx(seed % 3); // kind: 0, 1 (bit 12), 2
                c.g[A3][0] = sx(r.next_u32()); // value
            },
        );
    }

    #[test]
    fn bne_player_int_get_peak_flag_matches() {
        check(
            ee::s01::f_00146150__generated,
            bne_player_int_get_peak_flag,
            300,
            &|c, seed| {
                let mut r = Rng(0x7a11_0150 ^ seed as u64);
                scramble(c, &mut r);
                bne_fixture(c, &mut r);
                c.pc = 0x0014_6150;
                c.g[A1][0] = sx(seed % 4); // index
                c.g[A2][0] = sx((seed / 2) % 4); // kind 0..3
                c.g[A3][0] = sx(seed % 2); // flag
            },
        );
    }

    #[test]
    fn danger_man_get_rider_danger_level_matches() {
        check(
            ee::s01::f_001441b8__generated,
            danger_man_get_rider_danger_level,
            300,
            &|c, seed| {
                let mut r = Rng(0x7a11_41b8 ^ seed as u64);
                scramble(c, &mut r);
                let me = DATA; // cDangerMan
                let ai = DATA + 0x1000; // mpAI
                let rider = DATA + 0x2000;
                let vt = DATA + 0x3000; // rider's virtual object
                c.pc = 0x0014_41b8;
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(seed % 3);
                c.bus.write32(me, ai);
                for pl in 0..3 {
                    c.bus.write32(ai + 0x28 + 4 * pl, rider);
                    c.bus.write32(me + 0x70c + 12 * pl, (seed + pl) % 4); // base level
                }
                // getPosition(): the vtable entry is the identity leaf 0x140ce0,
                // so pos = rider + 0x6c0 (the word there is the object pointer,
                // whose bit pattern is pos.x).
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write32(vt + 0x28, 0);
                c.bus.write32(vt + 0x2c, 0x0014_0ce0);
                c.bus.write32(rider + 0x6c4, r.f32(60.0));
                c.bus.write32(rider + 0x6c8, r.f32(60.0));
                c.bus.write32(me + 0x704, 1 + seed % 5); // tracked elements
                c.bus.write32(me + 0x708, (seed / 3) % 2); // active avalanches
                for i in 0..8u32 {
                    let e = me + 4 + 0x1c * i;
                    c.bus.write32(e, r.f32(40.0)); // radius
                    c.bus.write32(e + 0x8, r.next_u32() % 3); // state
                    c.bus.write32(e + 0xc, r.f32(60.0));
                    c.bus.write32(e + 0x10, r.f32(60.0));
                    c.bus.write32(e + 0x14, r.f32(60.0));
                }
            },
        );
    }
}
