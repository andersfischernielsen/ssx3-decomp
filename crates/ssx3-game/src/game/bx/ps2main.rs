//! `src/bx/ps2main.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::movie::movieplayer::p;
pub(crate) use crate::game::util::menu::gp_5 as gp;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0031_b310, mat4_adjoint, "Mat4_Adjoint")];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::movie::movieplayer::callee::AIR_APPLY_WIND;
    pub(crate) use crate::game::movie::movieplayer::callee::AIR_PREDICTOR_UPDATE;
    pub(crate) use crate::game::movie::movieplayer::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::movie::movieplayer::callee::AUDIO_RESET_BOOST_PITCH;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_CHAR_ATTRIB_FACTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILD_OCCLUDER_PLANES;
    pub(crate) use crate::game::movie::movieplayer::callee::COLL_SEGMENT_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::MATH_DET3X3;
    pub(crate) use crate::game::movie::movieplayer::callee::OVERLAP_LIST_FIND_CLOSEST;
    pub(crate) use crate::game::movie::movieplayer::callee::PACK_POSITION;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL128_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL16_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL256_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL32_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL512_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL64_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL8_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::QSORT;
    pub(crate) use crate::game::movie::movieplayer::callee::REMOVE_HUD_MESSAGE;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_GET_SUBSTATE;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_ORIENTATION_IMPLICIT;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_UPDATE_ORIENTATION;
    pub(crate) use crate::game::movie::movieplayer::callee::SET_TIMED_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::SHOW_HUD_NUMBER;
    pub(crate) use crate::game::movie::movieplayer::callee::VU0_UPLOAD_OCCLUDER;
}

// ---------------------------------------------------------------------------
// 4x4 adjugate.

/// `void Mat4_Adjoint(Mat4* m)` (0x31b310): replace `m` with the transpose of
/// its cofactor matrix (the adjugate), built from sixteen 3x3 determinants.
pub fn mat4_adjoint(c: &mut Cpu) {
    c.cycles += 237;
    let frame = Frame::open(c, 0xa0);
    let m = abi::arg(c, 0);
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0x20);
    c.g[S0][0] = c.g[A0][0];
    frame.save_fpr(c, 31, 0x98);
    frame.save_fpr(c, 30, 0x90);
    frame.save_fpr(c, 29, 0x88);
    frame.save_fpr(c, 28, 0x80);
    frame.save_fpr(c, 27, 0x78);
    frame.save_fpr(c, 26, 0x70);
    frame.save_fpr(c, 25, 0x68);
    frame.save_fpr(c, 24, 0x60);
    frame.save_fpr(c, 23, 0x58);
    frame.save_fpr(c, 22, 0x50);
    frame.save_fpr(c, 21, 0x48);
    frame.save_fpr(c, 20, 0x40);
    // Load the cofactor operands into the argument registers f12..f19 and
    // keep spares on the frame.
    c.lwc1(0, p(m, 0x14));
    c.swc1(0, p(sp(c), 0x10));
    c.mov_s(12, 0);
    c.lwc1(1, p(m, 0x18));
    c.swc1(1, p(sp(c), 0x18));
    c.mov_s(15, 1);
    c.lwc1(0, p(m, 0x24));
    c.lwc1(30, p(m, 0x1c));
    c.swc1(0, p(sp(c), 0x14));
    c.mov_s(13, 0);
    c.mov_s(18, 30);
    c.lwc1(1, p(m, 0x28));
    c.swc1(1, p(sp(c), 0x1c));
    c.mov_s(16, 1);
    c.lwc1(20, p(m, 0x3c));
    c.lwc1(25, p(m, 0x2c));
    c.lwc1(23, p(m, 0x34));
    c.lwc1(22, p(m, 0x38));
    c.mov_s(19, 25);
    c.lwc1(28, m);
    c.mov_s(14, 23);
    c.swc1(20, sp(c));
    c.mov_s(17, 22);
    c.lwc1(29, p(m, 0x04));
    c.lwc1(31, p(m, 0x08));
    c.lwc1(24, p(m, 0x0c));
    c.lwc1(26, p(m, 0x10));
    c.lwc1(27, p(m, 0x20));
    c.lwc1(21, p(m, 0x30));
    if callee::MATH_DET3X3.call(c, 0x0031_b3c8).is_err() {
        return;
    }
    c.swc1(0, m);
    c.mov_s(12, 26);
    c.swc1(20, sp(c));
    c.mov_s(13, 27);
    c.lwc1(15, p(sp(c), 0x18));
    c.mov_s(14, 21);
    c.lwc1(16, p(sp(c), 0x1c));
    c.mov_s(17, 22);
    c.mov_s(18, 30);
    c.mov_s(19, 25);
    if callee::MATH_DET3X3.call(c, 0x0031_b3f4).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(20, sp(c));
    c.lwc1(15, p(sp(c), 0x10));
    c.mov_s(12, 26);
    c.mov_s(13, 27);
    c.mov_s(14, 21);
    c.swc1(0, p(m, 0x10));
    c.mov_s(17, 23);
    c.mov_s(18, 30);
    c.mov_s(19, 25);
    c.lwc1(16, p(sp(c), 0x14));
    if callee::MATH_DET3X3.call(c, 0x0031_b424).is_err() {
        return;
    }
    c.swc1(0, p(m, 0x20));
    c.mov_s(12, 26);
    c.swc1(22, sp(c));
    c.mov_s(13, 27);
    c.lwc1(15, p(sp(c), 0x10));
    c.mov_s(14, 21);
    c.lwc1(16, p(sp(c), 0x14));
    c.mov_s(17, 23);
    c.lwc1(18, p(sp(c), 0x18));
    c.lwc1(19, p(sp(c), 0x1c));
    if callee::MATH_DET3X3.call(c, 0x0031_b450).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(20, sp(c));
    c.lwc1(13, p(sp(c), 0x14));
    c.mov_s(12, 29);
    c.mov_s(14, 23);
    c.mov_s(15, 31);
    c.swc1(0, p(m, 0x30));
    c.mov_s(17, 22);
    c.mov_s(18, 24);
    c.mov_s(19, 25);
    c.lwc1(16, p(sp(c), 0x1c));
    if callee::MATH_DET3X3.call(c, 0x0031_b480).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(20, sp(c));
    c.mov_s(12, 28);
    c.mov_s(13, 27);
    c.mov_s(14, 21);
    c.mov_s(15, 31);
    c.swc1(0, p(m, 0x04));
    c.mov_s(17, 22);
    c.mov_s(18, 24);
    c.mov_s(19, 25);
    c.lwc1(16, p(sp(c), 0x1c));
    if callee::MATH_DET3X3.call(c, 0x0031_b4b0).is_err() {
        return;
    }
    c.swc1(0, p(m, 0x14));
    c.mov_s(12, 28);
    c.swc1(20, sp(c));
    c.mov_s(13, 27);
    c.lwc1(16, p(sp(c), 0x14));
    c.mov_s(14, 21);
    c.mov_s(15, 29);
    c.mov_s(17, 23);
    c.mov_s(18, 24);
    c.mov_s(19, 25);
    if callee::MATH_DET3X3.call(c, 0x0031_b4dc).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(22, sp(c));
    c.mov_s(12, 28);
    c.mov_s(13, 27);
    c.mov_s(14, 21);
    c.mov_s(15, 29);
    c.swc1(0, p(m, 0x24));
    c.mov_s(17, 23);
    c.mov_s(18, 31);
    c.lwc1(16, p(sp(c), 0x14));
    c.lwc1(19, p(sp(c), 0x1c));
    if callee::MATH_DET3X3.call(c, 0x0031_b50c).is_err() {
        return;
    }
    c.swc1(0, p(m, 0x34));
    c.mov_s(12, 29);
    c.swc1(20, sp(c));
    c.mov_s(14, 23);
    c.lwc1(13, p(sp(c), 0x10));
    c.mov_s(15, 31);
    c.lwc1(16, p(sp(c), 0x18));
    c.mov_s(17, 22);
    c.mov_s(18, 24);
    c.mov_s(19, 30);
    if callee::MATH_DET3X3.call(c, 0x0031_b538).is_err() {
        return;
    }
    c.swc1(0, p(m, 0x08));
    c.mov_s(12, 28);
    c.swc1(20, sp(c));
    c.mov_s(13, 26);
    c.lwc1(16, p(sp(c), 0x18));
    c.mov_s(14, 21);
    c.mov_s(15, 31);
    c.mov_s(17, 22);
    c.mov_s(18, 24);
    c.mov_s(19, 30);
    if callee::MATH_DET3X3.call(c, 0x0031_b564).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(20, sp(c));
    c.mov_s(12, 28);
    c.mov_s(13, 26);
    c.mov_s(14, 21);
    c.mov_s(15, 29);
    c.swc1(0, p(m, 0x18));
    c.mov_s(17, 23);
    c.mov_s(18, 24);
    c.mov_s(19, 30);
    c.lwc1(16, p(sp(c), 0x10));
    if callee::MATH_DET3X3.call(c, 0x0031_b594).is_err() {
        return;
    }
    c.swc1(22, sp(c));
    c.mov_s(14, 21);
    c.swc1(0, p(m, 0x28));
    c.mov_s(17, 23);
    c.mov_s(12, 28);
    c.mov_s(13, 26);
    c.lwc1(16, p(sp(c), 0x10));
    c.mov_s(15, 29);
    c.lwc1(19, p(sp(c), 0x18));
    c.mov_s(18, 31);
    if callee::MATH_DET3X3.call(c, 0x0031_b5c0).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(25, sp(c));
    c.lwc1(13, p(sp(c), 0x10));
    c.mov_s(12, 29);
    c.lwc1(14, p(sp(c), 0x14));
    c.mov_s(15, 31);
    c.swc1(0, p(m, 0x38));
    c.mov_s(18, 24);
    c.mov_s(19, 30);
    c.lwc1(16, p(sp(c), 0x18));
    c.lwc1(17, p(sp(c), 0x1c));
    if callee::MATH_DET3X3.call(c, 0x0031_b5f0).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(25, sp(c));
    c.mov_s(12, 28);
    c.mov_s(13, 26);
    c.mov_s(14, 27);
    c.mov_s(15, 31);
    c.swc1(0, p(m, 0x0c));
    c.mov_s(18, 24);
    c.mov_s(19, 30);
    c.lwc1(16, p(sp(c), 0x18));
    c.lwc1(17, p(sp(c), 0x1c));
    if callee::MATH_DET3X3.call(c, 0x0031_b620).is_err() {
        return;
    }
    c.swc1(25, sp(c));
    c.mov_s(18, 24);
    c.swc1(0, p(m, 0x1c));
    c.mov_s(19, 30);
    c.mov_s(12, 28);
    c.mov_s(13, 26);
    c.lwc1(16, p(sp(c), 0x10));
    c.mov_s(14, 27);
    c.lwc1(17, p(sp(c), 0x14));
    c.mov_s(15, 29);
    if callee::MATH_DET3X3.call(c, 0x0031_b64c).is_err() {
        return;
    }
    c.lwc1(1, p(sp(c), 0x1c));
    c.neg_s(0, 0);
    c.mov_s(12, 28);
    c.mov_s(13, 26);
    c.swc1(1, sp(c));
    c.mov_s(14, 27);
    c.swc1(0, p(m, 0x2c));
    c.mov_s(15, 29);
    c.mov_s(18, 31);
    c.lwc1(16, p(sp(c), 0x10));
    c.lwc1(17, p(sp(c), 0x14));
    c.lwc1(19, p(sp(c), 0x18));
    if callee::MATH_DET3X3.call(c, 0x0031_b680).is_err() {
        return;
    }
    c.swc1(0, p(m, 0x3c));
    frame.restore(c, S0, 0x30);
    frame.restore_ra(c, 0x20);
    frame.restore_fpr(c, 31, 0x98);
    frame.restore_fpr(c, 30, 0x90);
    frame.restore_fpr(c, 29, 0x88);
    frame.restore_fpr(c, 28, 0x80);
    frame.restore_fpr(c, 27, 0x78);
    frame.restore_fpr(c, 26, 0x70);
    frame.restore_fpr(c, 25, 0x68);
    frame.restore_fpr(c, 24, 0x60);
    frame.restore_fpr(c, 23, 0x58);
    frame.restore_fpr(c, 22, 0x50);
    frame.restore_fpr(c, 21, 0x48);
    frame.restore_fpr(c, 20, 0x40);
    frame.close(c);
}

/// `void ssx_LoadExecPS2(path, args, env)` (0x31b008): flush the CD/DVD
/// streams, shut the command server down and re-execute the ELF.
///
/// Ported for completeness but **not installed and not tested**: the
/// re-execution path runs the driver's initialisation (`sceCdInit` retries
/// its RPC bind forever on a machine without the CDVD server), so no fixture
/// can bound it.
pub fn ssx_loadexecps2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        c.g[A3][0] = lw_at(c, gw(c, 0x2a74));
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[V0][0] = lw_at(c, p(c.g[A3][0] as u32, 0x5c));
        let func = vslot(c, c.g[V0][0], 0x10, V1);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, 0x0031_b040)?;
        c.g[A0][0] = 2;
        call_ptr(c, 0x0040_2450, 0x0031_b048)?;
        c.g[A0][0] = 0;
        call_ptr(c, 0x0040_1718, 0x0031_b050)?;
        c.g[A0][0] = 5;
        call_ptr(c, 0x0040_1718, 0x0031_b058)?;
        call_ptr(c, 0x0042_5fb8, 0x0031_b060)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, 0x0042_c6a0, 0x0031_b070)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0031bcb0 Quat_Slerp
// ---------------------------------------------------------------------------

/// `void Quat_Slerp(Quat* dst, const Quat* a, const Quat* b, float t, ...)`
/// (0x31bcb0).
///
/// Normalised quaternion slerp: negates `b` when the dot product is negative,
/// then blends `a` and (possibly negated) `b` with `sin((1-t)θ)/sinθ` and
/// `sin(tθ)/sinθ`, falling back to a straight lerp when the two quaternions
/// are nearly parallel. `theta = acos(dot)` comes from `Math_Asin` on the dot
/// product with the quadrant constant at `gp-0x2ec4`.
pub fn quat_slerp(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x70);
        c.lwc1(5, c.g[A2][0] as u32);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save_fpr(c, 22, 0x60);
        frame.save_ra(c, 0x20);
        c.mov_s(22, 12);
        frame.save_fpr(c, 21, 0x58);
        frame.save_fpr(c, 20, 0x50);
        c.mtc1(0, 4); // f4 = 0.0
        c.lwc1(6, p(c.g[A2][0] as u32, 4));
        c.lwc1(0, c.g[S0][0] as u32);
        c.lwc1(2, p(c.g[S0][0] as u32, 4));
        c.mul_s(0, 0, 5);
        c.lwc1(7, p(c.g[A2][0] as u32, 8));
        c.mul_s(2, 2, 6);
        c.lwc1(3, p(c.g[S0][0] as u32, 8));
        c.lwc1(8, p(c.g[A2][0] as u32, 0xc));
        c.mul_s(3, 3, 7);
        c.lwc1(1, p(c.g[S0][0] as u32, 0xc));
        c.add_s(0, 0, 2);
        c.mul_s(1, 1, 8);
        c.add_s(0, 0, 3);
        c.add_s(12, 0, 1);
        c.c_lt_s(12, 4);
        let negate = !c.fcc();
        c.g[S1][0] = c.g[A0][0];
        'neg: {
            if !negate {
                break 'neg;
            }
            c.cycles += 10;
            c.neg_s(0, 5);
            c.neg_s(1, 6);
            c.neg_s(2, 7);
            c.neg_s(3, 8);
            c.swc1(0, c.g[SP][0] as u32);
            c.swc1(1, p(c.g[SP][0] as u32, 4));
            c.neg_s(12, 12);
            c.swc1(2, p(c.g[SP][0] as u32, 8));
            c.swc1(3, p(c.g[SP][0] as u32, 0xc));
            break 'neg;
        }
        c.cycles += 4;
        c.swc1(5, c.g[SP][0] as u32);
        c.swc1(6, p(c.g[SP][0] as u32, 4));
        c.swc1(7, p(c.g[SP][0] as u32, 8));
        c.swc1(8, p(c.g[SP][0] as u32, 0xc));

        // The blend weights: f20 for `a`, f12 for `b`.
        'weights: {
            'lerp: {
                c.cycles += 8;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 21); // f21 = 1.0
                c.lwc1(1, gp(c, -0x2ec8));
                c.sub_s(0, 21, 12);
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    // Nearly parallel: straight lerp.
                    c.sub_s(20, 21, 22);
                    break 'lerp;
                }
                c.cycles += 17;
                c.g[RA][0] = sx(0x0031_bd84);
                call_ptr(c, 0x0031_c128, 0x0031_bd84)?; // Math_Asin(dot)
                c.lwc1(20, gp(c, -0x2ec4));
                c.sub_s(20, 20, 0); // theta = half_pi - asin(dot)
                c.g[RA][0] = sx(0x0031_bd94);
                c.mov_s(12, 20);
                call_ptr(c, 0x0031_bf60, 0x0031_bd94)?; // sin(theta)
                c.sub_s(12, 21, 22);
                c.mov_s(21, 0);
                c.g[RA][0] = sx(0x0031_bda4);
                c.mul_s(12, 12, 20);
                call_ptr(c, 0x0031_bf60, 0x0031_bda4)?; // sin((1-t)theta)
                c.mul_s(12, 22, 20);
                c.div_s(20, 0, 21);
                c.g[RA][0] = sx(0x0031_bdb4);
                call_ptr(c, 0x0031_bf60, 0x0031_bdb4)?; // sin(t*theta)
                c.div_s(12, 0, 21);
                c.lwc1(7, p(c.g[S0][0] as u32, 0xc));
                break 'weights;
            }
            c.cycles += 2;
            c.mov_s(12, 22);
            c.lwc1(7, p(c.g[S0][0] as u32, 0xc));
        }

        // dst = a * f20 + b * f12
        c.cycles += 34;
        c.g[V0][0] = c.g[S1][0];
        c.lwc1(5, c.g[S0][0] as u32);
        c.lwc1(6, p(c.g[S0][0] as u32, 4));
        c.mul_s(7, 20, 7);
        c.lwc1(4, p(c.g[S0][0] as u32, 8));
        c.mul_s(5, 20, 5);
        c.lwc1(3, c.g[SP][0] as u32);
        c.mul_s(6, 20, 6);
        c.lwc1(2, p(c.g[SP][0] as u32, 4));
        c.mul_s(4, 20, 4);
        c.lwc1(1, p(c.g[SP][0] as u32, 8));
        c.mul_s(3, 12, 3);
        c.lwc1(0, p(c.g[SP][0] as u32, 0xc));
        c.mul_s(2, 12, 2);
        c.mul_s(1, 12, 1);
        c.lq(16, p(c.g[SP][0] as u32, 0x40));
        c.mul_s(0, 12, 0);
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x20));
        c.add_s(5, 5, 3);
        c.lwc1(22, p(c.g[SP][0] as u32, 0x60));
        c.add_s(6, 6, 2);
        c.lwc1(21, p(c.g[SP][0] as u32, 0x58));
        c.add_s(4, 4, 1);
        c.lwc1(20, p(c.g[SP][0] as u32, 0x50));
        c.add_s(7, 7, 0);
        c.swc1(5, p(c.g[SP][0] as u32, 0x10));
        c.swc1(6, p(c.g[SP][0] as u32, 0x14));
        c.swc1(4, p(c.g[SP][0] as u32, 0x18));
        c.swc1(7, p(c.g[SP][0] as u32, 0x1c));
        c.lq(3, p(c.g[SP][0] as u32, 0x10));
        c.sq(3, c.g[S1][0] as u32);
        c.lq(17, p(c.g[SP][0] as u32, 0x30));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = ra;
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    #[test]
    fn mat4_adjoint_matches() {
        check(
            ee::s10::f_0031b310__generated,
            mat4_adjoint,
            120,
            &|c, seed| {
                let mut r = Rng(0x31b3_10 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                for i in 0..4 {
                    let v = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                    put_vec(c, DATA + i * 0x10, v);
                }
            },
        );
    }
}
