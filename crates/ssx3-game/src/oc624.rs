//! Staging module oc624 (wave g04.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc624`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0013_1428,
    rider_board_press_sub_finish_press,
    "RiderBoardPressSub_FinishPress",
)];

/// `void RiderBoardPressSub_FinishPress(RiderBoardPressSub* self)` (0x131428):
/// ends a board press from the rider's current animation. In the two
/// board-press animations (37..38 and 29..30) the press either spins (when
/// the board is held past the threshold: pivot metrics, one boost, and the
/// pivot animation with `PressMode` set) or flips the stance and plays the
/// matching animation. Any other animation leaves it alone.
pub fn rider_board_press_sub_finish_press(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 2;
        c.g[V0][0] = Addr(r32(c, S0)).off(0x14).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x784).lw(c);
        call_ptr(c, 0x0031_2aa0, 0x0013_1448)?;
        c.g[V1][0] = c.g[V0][0];
        let extra_cycle = finish_press_paths(c)?;
        if extra_cycle {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The branches after the animation id is known (`V1`). Returns whether the
/// path ends in the shared `+1` exit (the spin paths and the 29..30 flip).
fn finish_press_paths(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x25));
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    if c.g[V0][0] != 0 {
        // Animations 37..38: the pivot is the press.
        c.cycles += 8;
        c.g[V0][0] = Addr(r32(c, S0)).off(0x14).lw(c);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 1);
        let slew = r32(c, V0).wrapping_add(0x280);
        c.lwc1(0, slew);
        c.abs_s(0, 0);
        c.c_le_s(0, 1);
        if c.fcc() {
            c.cycles += 18;
            finish_press_spin(c, 0x1c, 1, [0x0013_1488, 0x0013_1498, 0x0013_14b4])?;
            return Ok(true);
        }
        c.cycles += 11;
        finish_press_flip(c, 0x24, 0x0013_14cc, 0x0013_14e8)?;
        return Ok(false);
    }
    // Animations 29..30: the mirrored press.
    c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x1d));
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    c.cycles += 8;
    c.g[V0][0] = Addr(r32(c, S0)).off(0x14).lw(c);
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 1);
    let slew = r32(c, V0).wrapping_add(0x280);
    c.lwc1(0, slew);
    c.abs_s(0, 0);
    c.c_le_s(1, 0);
    if c.fcc() {
        c.cycles += 18;
        finish_press_spin(c, 0x24, 2, [0x0013_1528, 0x0013_1538, 0x0013_1554])?;
    } else {
        c.cycles += 9;
        finish_press_flip(c, 0x1c, 0x0013_156c, 0x0013_1588)?;
    }
    Ok(true)
}

/// The press spin. `V0` holds the rider. Calls `RiderMetrix_SetPressSpin`
/// (metrix, PressMode), `Rider_AddBoost(rider, 1)` and the pivot animation
/// `anim` with blend -1 and layer 0; then stores `mode` as PressMode.
fn finish_press_spin(
    c: &mut Cpu,
    anim: u32,
    mode: u64,
    rets: [u32; 3],
) -> Result<(), crate::call::Unwound> {
    let rider = r32(c, V0);
    c.g[A1][0] = Addr(rider).off(0x330).lw(c);
    c.g[A0][0] = Addr(rider).off(0x790).lw(c);
    call_ptr(c, 0x0011_9ad8, rets[0])?;
    c.g[A0][0] = Addr(r32(c, S0)).off(0x14).lw(c);
    c.mov_s(12, 0);
    c.g[A1][0] = 1;
    call_ptr(c, 0x0010_e098, rets[1])?;
    c.g[V0][0] = Addr(r32(c, S0)).off(0x14).lw(c);
    c.g[A1][0] = anim as u64;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 12);
    c.g[A2][0] = 0;
    c.g[A0][0] = Addr(r32(c, V0)).off(0x784).lw(c);
    call_ptr(c, 0x0031_28e8, rets[2])?;
    c.g[V1][0] = Addr(r32(c, S0)).off(0x14).lw(c);
    c.g[V0][0] = mode;
    let v = c.g[V0][0];
    Addr(r32(c, V1)).off(0x330).sw(c, v);
    Ok(())
}

/// The stance flip (`RiderBoardPressSub_FlipStance(self)`) followed by the
/// animation `anim` with blend -1 and layer 0.
fn finish_press_flip(
    c: &mut Cpu,
    anim: u32,
    ret_flip: u32,
    ret_play: u32,
) -> Result<(), crate::call::Unwound> {
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0012_fec8, ret_flip)?;
    c.g[V0][0] = Addr(r32(c, S0)).off(0x14).lw(c);
    c.g[A1][0] = anim as u64;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 12);
    c.g[A2][0] = 0;
    c.g[A0][0] = Addr(r32(c, V0)).off(0x784).lw(c);
    call_ptr(c, 0x0031_28e8, ret_play)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fake, fall_back, hook, machine};
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word and the animation id the anim-id fake returns (at +4).
    const OC624_CTRL: u32 = DATA + 0x4000;
    const OC624_MAGIC: u32 = 0x0624_0624;

    static OC624_PREV_ANIM: AtomicUsize = AtomicUsize::new(0);

    /// `RiderAnimBase_getCurrentAnimID`: logs the call and returns the id
    /// the fixture writes at `CTRL + 4`.
    fn oc624_fake_anim_id(c: &mut Cpu) {
        if c.bus.read32(OC624_CTRL) != OC624_MAGIC {
            return fall_back(c, &OC624_PREV_ANIM, ee::s10::f_00312aa0__generated);
        }
        let ra = c.g[RA][0] as u32;
        let anim = c.bus.read32(OC624_CTRL + 4);
        fake(c, 0x0031_2aa0, ra);
        if c.pc == ra {
            c.g[V0][0] = anim as u64;
        }
    }

    macro_rules! oc624_fake {
        ($fake:ident, $prev:ident, $target:literal, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC624_CTRL) != OC624_MAGIC {
                    return fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fake(c, $target, ra);
            }
        };
    }

    oc624_fake!(
        oc624_fake_press_spin,
        OC624_PREV_SPIN,
        0x0011_9ad8,
        ee::s00::f_00119ad8__generated
    );
    oc624_fake!(
        oc624_fake_add_boost,
        OC624_PREV_BOOST,
        0x0010_e098,
        ee::s00::f_0010e098__generated
    );
    oc624_fake!(
        oc624_fake_flip,
        OC624_PREV_FLIP,
        0x0012_fec8,
        ee::s00::f_0012fec8__generated
    );
    oc624_fake!(
        oc624_fake_play,
        OC624_PREV_PLAY,
        0x0031_28e8,
        ee::s10::f_003128e8__generated
    );

    fn oc624_install() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00312AA0),
            oc624_fake_anim_id,
            &OC624_PREV_ANIM,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00119AD8),
            oc624_fake_press_spin,
            &OC624_PREV_SPIN,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0010E098),
            oc624_fake_add_boost,
            &OC624_PREV_BOOST,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0012FEC8),
            oc624_fake_flip,
            &OC624_PREV_FLIP,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_003128E8),
            oc624_fake_play,
            &OC624_PREV_PLAY,
        );
    }

    #[test]
    fn rider_board_press_sub_finish_press_matches() {
        oc624_install();
        check(
            ee::s00::f_00131428__generated,
            rider_board_press_sub_finish_press,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x0624_1428, 0);
                // Animation: 37, 38, 29, 30 or an unrelated one.
                let anim = [37, 38, 29, 30, 5][(seed % 5) as usize];
                // Board-press stick: below and above the 0.5 threshold.
                let slew = [0.25f32, 0.5, -0.5, 0.75, -1.0][((seed / 5) % 5) as usize];
                let this = DATA + 0x100;
                let rider = DATA + 0x2000;
                c.bus.write32(OC624_CTRL, OC624_MAGIC);
                c.bus.write32(OC624_CTRL + 4, anim);
                c.bus.write32(this + 0x14, rider);
                c.bus.write32(rider + 0x280, slew.to_bits());
                c.bus.write32(rider + 0x330, seed % 3);
                c.bus.write32(rider + 0x784, DATA + 0x3000);
                c.bus.write32(rider + 0x790, DATA + 0x3800);
                c.g[A0][0] = sx(this);
                c.pc = 0x0013_1428;
            },
        );
    }
}
