//! Staging module oc418 (wave wave219): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc418`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

// ---------------------------------------------------------------------------
// RiderRailslideSub_RotateStance (0x00132060)

/// `cRailslideControl` self: current spin angle (radians) at +0, the rider at +8.
pub(crate) const RRS_SELF_ANGLE: u32 = 0x0;
pub(crate) const RRS_SELF_RIDER: u32 = 0x8;
/// Rider fields: flip flags (+0x320), stance (1..4, +0x328), model flips
/// (+0x238, +0x240), the anim controller (+0x784) and the metrix (+0x790).
pub(crate) const RRS_RIDER_FLIP: u32 = 0x320;
pub(crate) const RRS_RIDER_STANCE: u32 = 0x328;
pub(crate) const RRS_RIDER_FLIP_X: u32 = 0x238;
pub(crate) const RRS_RIDER_FLIP_Y: u32 = 0x240;
pub(crate) const RRS_RIDER_ANIM: u32 = 0x784;
pub(crate) const RRS_RIDER_METRIX: u32 = 0x790;
/// The data-segment constants (0x4ff130, 0x4ff160) the rotation quad is built from.
#[cfg(test)]
pub(crate) const RRS_ROT_CONSTS: u32 = 0x4f_f130;

/// What the stance (1..4) of the rider selects.
#[derive(Clone, Copy)]
enum RrsStance {
    One,
    Two,
    Three,
    Four,
    Other,
}

/// The stance switch: cycles and v0/a3 as the generated compare chain leaves
/// them. `state` is `v1`, the stance as loaded (sign-extended).
fn rrs_pick(c: &mut Cpu, neg: bool, state: u64) -> RrsStance {
    if state == 2 {
        return RrsStance::Two;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    if (state as i64) < 3 {
        c.cycles += 2;
        let hit = state == 1;
        c.g[V0][0] = 0x500000;
        if hit {
            return RrsStance::One;
        }
        c.cycles += 2;
        let rider = c.g[S3][0] as u32;
        c.g[V0][0] = c.lw(rider.wrapping_add(RRS_SELF_RIDER));
        return RrsStance::Other;
    }
    c.cycles += 3;
    c.g[V0][0] = 3;
    let hit = state == 3;
    c.g[V0][0] = 4;
    if hit {
        return RrsStance::Three;
    }
    c.cycles += 2;
    let hit = state == 4;
    if neg {
        c.g[V0][0] = 0x500000;
    } else {
        c.g[A3][0] = 1;
    }
    if hit {
        return RrsStance::Four;
    }
    c.cycles += 2;
    let rider = c.g[S3][0] as u32;
    c.g[V0][0] = c.lw(rider.wrapping_add(RRS_SELF_RIDER));
    RrsStance::Other
}

/// The rail-spin vector: `(sin*x, cos, sin*y)` scaled by the Math_SinCos
/// results, stored into the stack temp, then `base` (+4/+8, s0-0xea0) read.
fn rrs_scale(c: &mut Cpu, base: usize, out: u32, anim_id: u64) {
    let base_addr = c.g[base][0] as u32;
    let s0 = c.g[S0][0] as u32;
    let sp0 = c.g[SP][0] as u32;
    c.lwc1(1, base_addr.wrapping_add(8));
    c.g[A1][0] = anim_id;
    c.lwc1(0, sp0.wrapping_add(out));
    c.g[A2][0] = 0;
    c.lwc1(2, s0.wrapping_sub(0xea0));
    c.lwc1(4, base_addr.wrapping_add(4));
    c.mul_s(1, 0, 1);
    c.mul_s(2, 0, 2);
    c.lwc1(3, sp0.wrapping_add(out + 4));
}

/// Finish the vector (`* f4`, w = -1.0) and store it as the anim's rotation
/// quad at `dest + 0x40`.
fn rrs_store(c: &mut Cpu, dest: usize) {
    let sp0 = c.g[SP][0] as u32;
    c.mul_s(0, 0, 4);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 12);
    c.swc1(3, sp0.wrapping_add(0xc));
    c.swc1(1, sp0.wrapping_add(0x8));
    c.swc1(2, sp0);
    c.swc1(0, sp0.wrapping_add(0x4));
    c.lq(2, sp0);
    let d = c.g[dest][0] as u32;
    c.sq(2, d.wrapping_add(0x40));
}

/// Stance 1/4 style (no flip): copy the rotation constant to the anim, spin
/// by the angle in gp-pool `gp_off`, vector from `RRS_ROT_B`.
fn rrs_plain(c: &mut Cpu, cycles: u64, gp_off: u32, out: u32, ret: u32, anim_id: u64) -> Flow {
    c.cycles += cycles;
    let rider = c.g[A0][0] as u32;
    c.g[S2][0] = c.lw(rider.wrapping_add(RRS_RIDER_ANIM));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(gp_off));
    let v0 = c.g[V0][0] as u32;
    c.lq(3, v0);
    let sp0 = c.g[SP][0] as u32;
    c.g[A0][0] = sx(sp0.wrapping_add(out));
    c.g[A1][0] = sx(sp0.wrapping_add(out + 4));
    c.g[S0][0] = 0x500000;
    let s2 = c.g[S2][0] as u32;
    c.sq(3, s2.wrapping_add(0x30));
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xea0));
    call_ptr(c, 0x0031be50, ret)?;
    rrs_scale(c, S1, out, anim_id);
    rrs_store(c, S2);
    Ok(())
}

/// Stance 2/3 style (flip the model): clear or set the flip flags, negate the
/// two model flips, spin by `gp_off`.
fn rrs_flip(
    c: &mut Cpu,
    cycles: u64,
    flip: u64,
    gp_off: u32,
    out: u32,
    ret: u32,
    anim_id: u64,
    store: bool,
) -> Flow {
    c.cycles += cycles;
    let rider = c.g[A0][0] as u32;
    if flip != 0 {
        c.g[A3][0] = flip;
    }
    c.sw(rider.wrapping_add(RRS_RIDER_FLIP), flip);
    c.g[A2][0] = 0x500000;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xed0));
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(gp_off));
    let sp0 = c.g[SP][0] as u32;
    let me = c.g[S3][0] as u32;
    c.g[V1][0] = c.lw(me.wrapping_add(RRS_SELF_RIDER));
    c.g[A0][0] = sx(sp0.wrapping_add(out));
    c.g[A1][0] = sx(sp0.wrapping_add(out + 4));
    c.g[S0][0] = 0x500000;
    let v1 = c.g[V1][0] as u32;
    c.g[V0][0] = c.lw(v1.wrapping_add(RRS_RIDER_ANIM));
    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xea0));
    let v0 = c.g[V0][0] as u32;
    c.sw(v0.wrapping_add(0x18), flip);
    c.g[V1][0] = c.lw(me.wrapping_add(RRS_SELF_RIDER));
    let v1 = c.g[V1][0] as u32;
    c.lwc1(0, v1.wrapping_add(RRS_RIDER_FLIP_X));
    c.neg_s(0, 0);
    c.swc1(0, v1.wrapping_add(RRS_RIDER_FLIP_X));
    c.g[V0][0] = c.lw(me.wrapping_add(RRS_SELF_RIDER));
    let v0 = c.g[V0][0] as u32;
    c.lwc1(0, v0.wrapping_add(RRS_RIDER_FLIP_Y));
    c.neg_s(0, 0);
    c.swc1(0, v0.wrapping_add(RRS_RIDER_FLIP_Y));
    c.g[V1][0] = c.lw(me.wrapping_add(RRS_SELF_RIDER));
    let a2 = c.g[A2][0] as u32;
    c.lq(2, a2);
    let v1 = c.g[V1][0] as u32;
    c.g[S1][0] = c.lw(v1.wrapping_add(RRS_RIDER_ANIM));
    let s1 = c.g[S1][0] as u32;
    c.sq(2, s1.wrapping_add(0x30));
    call_ptr(c, 0x0031be50, ret)?;
    rrs_scale(c, S2, out, anim_id);
    if store {
        rrs_store(c, S1);
    }
    Ok(())
}

/// Stance 4/3 style (rotation back to 0): no gp angle, f12 = -0.0.
fn rrs_zero(c: &mut Cpu, out: u32, ret: u32, anim_id: u64) -> Flow {
    c.cycles += 21;
    c.mtc1(0, 12);
    c.g[V0][0] = 0x500000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
    let rider = c.g[A0][0] as u32;
    c.g[S1][0] = c.lw(rider.wrapping_add(RRS_RIDER_ANIM));
    let v0 = c.g[V0][0] as u32;
    c.lq(3, v0);
    c.neg_s(12, 12);
    let sp0 = c.g[SP][0] as u32;
    c.g[A0][0] = sx(sp0.wrapping_add(out));
    c.g[A1][0] = sx(sp0.wrapping_add(out + 4));
    let s1 = c.g[S1][0] as u32;
    c.sq(3, s1.wrapping_add(0x30));
    c.g[S0][0] = 0x500000;
    call_ptr(c, 0x0031be50, ret)?;
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xea0));
    rrs_scale(c, V0, out, anim_id);
    Ok(())
}

/// Play the anim (`AnimController_PlayAnim(rider.anim)`, f12 = -1.0 from the
/// vector tail) and set the new stance: `Some(v0)`, or `s4` (the `None` tail).
fn rrs_play(c: &mut Cpu, ret: u32, stance: Option<u64>) -> Flow {
    let me = c.g[S3][0] as u32;
    c.g[V1][0] = c.lw(me.wrapping_add(RRS_SELF_RIDER));
    let v1 = c.g[V1][0] as u32;
    c.g[A0][0] = c.lw(v1.wrapping_add(RRS_RIDER_ANIM));
    call_ptr(c, 0x003128e8, ret)?;
    c.g[V1][0] = c.lw(me.wrapping_add(RRS_SELF_RIDER));
    let v1 = c.g[V1][0] as u32;
    let value = match stance {
        Some(v) => {
            c.g[V0][0] = v;
            v
        }
        None => c.g[S4][0],
    };
    c.sw(v1.wrapping_add(RRS_RIDER_STANCE), value);
    Ok(())
}

/// `void RiderRailslideSub_RotateStance(cRailslideControl* self, int neg)`
/// (0x00132060): turn the rail spin a quarter (`neg`: the other way, which
/// also decides the flip): the angle moves by a gp-pool constant, the rider's
/// stance (1..4) steps to its neighbour, the matching spin animation is
/// started (anim 49..54, rotation quad from `Math_SinCos`), and the
/// rail-spin event is reported (`evRailSpin` metrix, `Rider_AddBoost`).
pub fn rider_railslide_sub_rotate_stance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, 19, 0x70);
        frame.save(c, 16, 0xa0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, 17, 0x90);
        frame.save(c, 18, 0x80);
        frame.save(c, 20, 0x60);
        let neg = c.g[A1][0] != 0;
        frame.save_ra(c, 0x50);
        // Angle += / -= a quarter turn; read the rider's stance.
        c.cycles += 9;
        let me = c.g[S3][0] as u32;
        c.lwc1(0, me.wrapping_add(RRS_SELF_ANGLE));
        c.g[S4][0] = 2;
        let gp = c.g[GP][0] as u32;
        c.lwc1(1, gp.wrapping_sub(if neg { 0x7590 } else { 0x7580 }));
        c.g[A0][0] = c.lw(me.wrapping_add(RRS_SELF_RIDER));
        if neg {
            c.sub_s(0, 0, 1);
        } else {
            c.add_s(0, 0, 1);
        }
        c.swc1(0, me.wrapping_add(RRS_SELF_ANGLE));
        let rider = c.g[A0][0] as u32;
        c.g[V1][0] = c.lw(rider.wrapping_add(RRS_RIDER_STANCE));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
        let state = c.g[V1][0];
        let stance = rrs_pick(c, neg, state);
        // Each case: build the spin, play it, set the next stance.
        match (neg, stance) {
            (_, RrsStance::Other) => {
                // Default: no change; skips the `+1` of the normal exit.
            }
            (true, RrsStance::One) => {
                rrs_plain(c, 29, 0x758c, 0x10, 0x00132104, 0x31)?;
                c.cycles += 7;
                rrs_play(c, 0x00132474, Some(3))?;
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(RRS_SELF_RIDER));
            }
            (true, RrsStance::Two) => {
                rrs_flip(c, 43, 0, 0x7588, 0x18, 0x001321b0, 0x35, true)?;
                c.cycles += 7;
                rrs_play(c, 0x001323b0, Some(4))?;
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(RRS_SELF_RIDER));
            }
            (true, RrsStance::Three) => {
                rrs_flip(c, 35, 1, 0x7584, 0x20, 0x00132260, 0x33, false)?;
                c.cycles += 14;
                rrs_store(c, S1);
                rrs_play(c, 0x001325cc, None)?;
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(RRS_SELF_RIDER));
            }
            (true, RrsStance::Four) => {
                rrs_zero(c, 0x28, 0x001322b0, 0x36)?;
                c.cycles += 16;
                rrs_store(c, S1);
                rrs_play(c, 0x00132508, Some(1))?;
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(RRS_SELF_RIDER));
            }
            (false, RrsStance::One) => {
                rrs_plain(c, 28, 0x757c, 0x30, 0x0013235c, 0x32)?;
                c.cycles += 7;
                rrs_play(c, 0x001323b0, Some(4))?;
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(RRS_SELF_RIDER));
            }
            (false, RrsStance::Two) => {
                rrs_flip(c, 42, 0, 0x7578, 0x38, 0x00132420, 0x34, true)?;
                c.cycles += 7;
                rrs_play(c, 0x00132474, Some(3))?;
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(RRS_SELF_RIDER));
            }
            (false, RrsStance::Three) => {
                rrs_zero(c, 0x40, 0x001324b0, 0x33)?;
                c.cycles += 16;
                rrs_store(c, S1);
                rrs_play(c, 0x00132508, Some(1))?;
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(RRS_SELF_RIDER));
            }
            (false, RrsStance::Four) => {
                rrs_flip(c, 33, 1, 0x7574, 0x48, 0x00132578, 0x36, false)?;
                c.cycles += 14;
                rrs_store(c, S1);
                rrs_play(c, 0x001325cc, None)?;
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(RRS_SELF_RIDER));
            }
        }
        // Report the spin event.
        c.cycles += 18;
        let me = c.g[S3][0] as u32;
        c.lwc1(12, me.wrapping_add(RRS_SELF_ANGLE));
        let rider = c.g[V0][0] as u32;
        c.g[A1][0] = c.lw(rider.wrapping_add(RRS_RIDER_STANCE));
        c.g[A0][0] = c.lw(rider.wrapping_add(RRS_RIDER_METRIX));
        call_ptr(c, 0x00119918, 0x001325e8)?;
        c.g[A0][0] = c.lw(me.wrapping_add(RRS_SELF_RIDER));
        c.mov_s(12, 0);
        c.g[A1][0] = 1;
        call_ptr(c, 0x0010e098, 0x001325f8)?;
        c.g[A0][0] = c.lw(me.wrapping_add(RRS_SELF_RIDER));
        call_ptr(c, 0x00116930, 0x00132600)?;
        frame.restore(c, 16, 0xa0);
        frame.restore(c, 17, 0x90);
        frame.restore(c, 18, 0x80);
        frame.restore(c, 19, 0x70);
        frame.restore(c, 20, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x00132060,
    rider_railslide_sub_rotate_stance,
    "RiderRailslideSub_RotateStance",
)];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{fill, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's callee fakes (other tests' machines fall back).
    const RRS_TEST_CTRL: u32 = DATA + 0x21_8200;
    const RRS_TEST_MAGIC: u32 = 0x5a17_0c32;
    const RRS_TEST_SELF: u32 = DATA + 0x100;
    const RRS_TEST_RIDER: u32 = DATA + 0x1000;
    const RRS_TEST_ANIM: u32 = DATA + 0x2800;
    const RRS_TEST_METRIX: u32 = DATA + 0x3000;

    static RRS_PREV_PLAY: AtomicUsize = AtomicUsize::new(0);
    static RRS_PREV_BOOST: AtomicUsize = AtomicUsize::new(0);

    fn rrs_fake_play(c: &mut Cpu) {
        if c.bus.read32(RRS_TEST_CTRL) != RRS_TEST_MAGIC {
            return fakes::fall_back(c, &RRS_PREV_PLAY, ee::s10::f_003128e8__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003128e8, ra);
    }

    fn rrs_fake_boost(c: &mut Cpu) {
        if c.bus.read32(RRS_TEST_CTRL) != RRS_TEST_MAGIC {
            return fakes::fall_back(c, &RRS_PREV_BOOST, ee::s00::f_0010e098__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0010e098, ra);
    }

    /// Rotate the stance: every stance (1..4, none, negative) in both
    /// directions; the animation play and the boost call are fakes (one of
    /// them unwinds on some seeds), the maths and metrix callees are real.
    #[test]
    fn rider_railslide_sub_rotate_stance_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003128E8),
            rrs_fake_play,
            &RRS_PREV_PLAY,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0010E098),
            rrs_fake_boost,
            &RRS_PREV_BOOST,
        );
        covered(
            ee::s01::f_00132060__generated,
            rider_railslide_sub_rotate_stance,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0013_2060, 2);
                c.pc = 0x00132060;
                c.bus.write32(RRS_TEST_CTRL, RRS_TEST_MAGIC);
                c.g[GP][0] = GAME_GP as u64;
                // Gp-pool angles (quarter turns and half turns, plus noise).
                for off in [
                    0x7590u32, 0x7580, 0x758c, 0x7588, 0x7584, 0x757c, 0x7578, 0x7574, 0x7a14,
                    0x2ec0, 0x2ebc, 0x2eb8, 0x2eb4, 0x2eb0,
                ] {
                    let bits = r.f32(4.0);
                    c.bus.write32(GAME_GP - off, bits);
                }
                fill(c, &mut r, RRS_TEST_RIDER, 0x400);
                fill(c, &mut r, RRS_TEST_ANIM, 0x40);
                fill(c, &mut r, RRS_TEST_METRIX, 0x20);
                fill(c, &mut r, RRS_ROT_CONSTS, 0x20);
                let states: [u32; 9] = [1, 2, 3, 4, 0, 5, 0xffff_ffff, 2, 3];
                let stance = states[seed as usize % 9];
                c.bus.write32(RRS_TEST_SELF + RRS_SELF_ANGLE, r.f32(4.0));
                c.bus.write32(RRS_TEST_SELF + RRS_SELF_RIDER, RRS_TEST_RIDER);
                c.bus.write32(RRS_TEST_RIDER + RRS_RIDER_STANCE, stance);
                c.bus.write32(RRS_TEST_RIDER + RRS_RIDER_ANIM, RRS_TEST_ANIM);
                c.bus.write32(RRS_TEST_RIDER + RRS_RIDER_METRIX, RRS_TEST_METRIX);
                c.g[A0][0] = RRS_TEST_SELF as u64;
                c.g[A1][0] = if (seed / 9) % 2 == 0 {
                    0
                } else {
                    (r.next_u32() as u64) << 32 | (r.next_u32() | 1) as u64
                };
            },
        );
    }
}
