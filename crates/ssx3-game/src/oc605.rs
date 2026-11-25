//! Staging module oc605 (wave l587b.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc605`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0035_0750, dead_fade_ctor_2, "DeadFade_ctor_2")];

/// `cInstanceNode_cInstanceNode1` (0x34fb00): the world-object base of the
/// fade node, kind 0x16 (the generated body calls it directly).
pub(crate) const DEAD_FADE_CTOR_2_BASE: Callee = Callee {
    addr: 0x0034_fb00,
    f: ee::s11::f_0034fb00,
};

/// The blend of one part's four rows (`src`, the part's source quads) with
/// the matrix `mat` (four quads), written through the stack scratch at `sp`
/// and into the output block at `T1`, with the `T3` matrix. The two
/// `DeadFade_ctor_2` loop bodies differ only in where `src` and `mat` come
/// from and in the register side effects their callers set up.
fn dead_fade_ctor_2_blend_rows(c: &mut Cpu, src: u32, mat: u32) {
    let s = sp(c);
    let t3 = r32(c, T3);
    let t1 = r32(c, T1);
    c.lwc1(6, s);
    c.lq(2, src);
    c.lwc1(7, s.wrapping_add(4));
    c.sq(2, s.wrapping_add(0x90));
    c.sq(2, s.wrapping_add(0x50));
    c.lwc1(5, s.wrapping_add(8));
    c.lq(2, src.wrapping_add(0x10));
    c.lwc1(4, s.wrapping_add(0xc));
    c.sq(2, s.wrapping_add(0x90));
    c.sq(2, s.wrapping_add(0x60));
    c.lq(3, src.wrapping_add(0x20));
    c.sq(3, s.wrapping_add(0x90));
    c.sq(3, s.wrapping_add(0x70));
    c.lq(2, src.wrapping_add(0x30));
    c.sq(2, s.wrapping_add(0xa0));
    c.lwc1(1, s.wrapping_add(0xa0));
    c.lwc1(2, s.wrapping_add(0xa4));
    c.lwc1(3, s.wrapping_add(0xa8));
    c.mul_s(1, 1, 6);
    c.lwc1(0, s.wrapping_add(0xac));
    c.mul_s(2, 2, 7);
    c.mul_s(3, 3, 5);
    c.mul_s(0, 0, 4);
    c.swc1(1, s.wrapping_add(0x90));
    c.swc1(2, s.wrapping_add(0x94));
    c.swc1(3, s.wrapping_add(0x98));
    c.swc1(0, s.wrapping_add(0x9c));
    c.lq(2, s.wrapping_add(0x90));
    c.sq(2, s.wrapping_add(0x80));
    c.lqc2(4, mat);
    c.lqc2(5, mat.wrapping_add(0x10));
    c.lqc2(6, mat.wrapping_add(0x20));
    c.lqc2(7, mat.wrapping_add(0x30));
    c.lqc2(8, t3);
    c.lqc2(9, t3.wrapping_add(0x10));
    c.lqc2(10, t3.wrapping_add(0x20));
    c.lqc2(11, t3.wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.vu0().mula(0b1111, 4, Src::Bc(9, X));
    c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
    c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
    c.vu0().mula(0b1111, 4, Src::Bc(10, X));
    c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
    c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
    c.vu0().mula(0b1111, 4, Src::Bc(11, X));
    c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
    c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
    c.sqc2(12, t1);
    c.sqc2(13, t1.wrapping_add(0x10));
    c.sqc2(14, t1.wrapping_add(0x20));
    c.sqc2(15, t1.wrapping_add(0x30));
}

/// `void cDeadFadeNode::cDeadFadeNode(self, bucket, obj, desc, other)`
/// (0x350750): builds a dead-fade node in place. The world-object part comes
/// from `cInstanceNode` (kind 0x16) with `obj`; then the node takes the
/// vtable 0x491980, its fade rate (`getUpdateTime / desc->FadeTime`) and
/// distance rate (`desc->ZVel * getUpdateTime`), and starts at distance 0 and
/// fade 1. Its per-part animation matrices (`AnimMats`, one 64-byte row per
/// model part) are copied from `other` when it has them, else composed from
/// the instance transform and the model's parts (parent -1 starts from the
/// instance's root transform). `other` is deleted afterwards and the
/// instance's flags get bit 2 (bits 5 and 6 cleared).
/// Frame 0x150: `ra` at +0xf0, s0..s4 at +0x140..+0x100.
pub fn dead_fade_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0x150);
        frame.save(c, S0, 0x140);
        frame.save(c, S1, 0x130);
        c.g[S0][0] = c.g[A3][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x120);
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S3, 0x110);
        frame.save(c, S4, 0x100);
        frame.save_ra(c, 0xf0);
        c.g[S2][0] = c.g[T0][0];
        c.g[A2][0] = 0x16;
        c.g[S4][0] = sx(r32(c, S1).wrapping_add(0x38));
        DEAD_FADE_CTOR_2_BASE.call(c, 0x0035_0788)?;

        c.g[V1][0] = 0x49_0000;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x1980));
        c.g[V0][0] |= 0xffff;
        c.sw(r32(c, S1).wrapping_add(0xc), c.g[V1][0]);
        c.sw(r32(c, S1).wrapping_add(0x1c), c.g[V0][0]);
        c.ldl(2, r32(c, S0).wrapping_add(0x7));
        c.ldr(2, r32(c, S0));
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x8));
        c.sdl(2, r32(c, S1).wrapping_add(0x23));
        c.sdr(2, r32(c, S1).wrapping_add(0x1c));
        c.sw(r32(c, S1).wrapping_add(0x24), c.g[V1][0]);

        // FadeRate = getUpdateTime / desc->FadeTime.
        call_ptr(c, 0x002d_1c70, 0x0035_07c0)?;
        c.lwc1(1, r32(c, S0).wrapping_add(0x8));
        c.div_s(0, 0, 1);
        c.swc1(0, r32(c, S1).wrapping_add(0x2c));
        // DistRate = desc->ZVel * getUpdateTime.
        call_ptr(c, 0x002d_1c70, 0x0035_07d0)?;
        c.lwc1(1, r32(c, S0).wrapping_add(0xc));
        c.g[A1][0] = 0x49_0000;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 2);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1758));
        c.mul_s(1, 1, 0);
        c.sw(r32(c, S1).wrapping_add(0x30), 0u64);
        c.swc1(2, r32(c, S1).wrapping_add(0x28));
        c.g[A2][0] = 0x2000_0000;
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x18));
        c.g[A3][0] = 0u64;
        c.swc1(1, r32(c, S1).wrapping_add(0x34));
        c.g[S3][0] = c.lw(r32(c, V0).wrapping_add(0x80));
        c.g[S0][0] = c.lw(r32(c, S3).wrapping_add(0x4));
        c.g[A0][0] = sx(r32(c, S0) << 6);
        // AnimMats = new (str_DeadFadeAnimMats, 0x20000000, 0) [count].
        call_ptr(c, 0x0031_7e30, 0x0035_0810)?;
        let no_parts = c.g[S0][0] == 0u64;
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(0x1));
        if !no_parts {
            c.cycles += 2;
            c.g[A0][0] = 0xffff_ffff_ffff_ffff;
        }

        'b_350b24: {
            'b_350878: {
                'b_350874: {
                    c.cycles += 2;
                    let other_is_null = c.g[S2][0] == 0u64;
                    c.sw(r32(c, S4), c.g[V0][0]);
                    if other_is_null {
                        break 'b_350874;
                    }
                    // other->hasAnimMats()
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(0xc));
                    c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0xd0));
                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xd4));
                    let has_anim_mats = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
                    call_ptr(c, has_anim_mats, 0x0035_083c)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x18));
                        break 'b_350878;
                    }
                    // other->getAnimMats(): copy model->count rows of 64 bytes.
                    c.cycles += 12;
                    c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(0xc));
                    c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0xd8));
                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xdc));
                    let get_anim_mats = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
                    call_ptr(c, get_anim_mats, 0x0035_0858)?;
                    c.g[A2][0] = c.lw(r32(c, S3).wrapping_add(0x4));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x38));
                    c.g[A2][0] = sx(r32(c, A2) << 6);
                    call_ptr(c, 0x003e_6574, 0x0035_086c)?;
                    break 'b_350b24;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x18));
            }

            // base transform: the instance's root transform when `other` gave none.
            'b_350898: {
                c.cycles += 2;
                let other_is_null = c.g[S2][0] == 0u64;
                c.g[T4][0] = sx(r32(c, V0).wrapping_add(0x10));
                if other_is_null {
                    break 'b_350898;
                }
                c.cycles += 6;
                c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(0xc));
                c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0xc0));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xc4));
                let root = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
                call_ptr(c, root, 0x0035_0894)?;
                c.g[T4][0] = c.g[V0][0];
            }

            c.cycles += 12;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x18));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.lwc1(0, r32(c, V0).wrapping_add(0x84));
            c.g[A3][0] = c.lw(r32(c, S3).wrapping_add(0x8));
            let s = sp(c);
            c.swc1(0, s.wrapping_add(0x8));
            c.swc1(1, s.wrapping_add(0xc));
            c.swc1(0, s);
            c.swc1(0, s.wrapping_add(0x4));
            c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x4));
            let no_parts = (c.g[V0][0] as i64) <= 0;
            c.g[T0][0] = 0u64;
            if no_parts {
                break 'b_350b24;
            }
            c.cycles += 4;
            c.g[T5][0] = 0xffff_ffff_ffff_ffff;
            c.g[T2][0] = sx(s.wrapping_add(0x10));
            c.g[T1][0] = sx(s.wrapping_add(0xb0));
            c.g[T3][0] = sx(s.wrapping_add(0x50));
            'l_3508d8: loop {
                c.cycles += 3;
                c.g[A2][0] = c.lw(r32(c, A3));
                let parent_is_none = c.g[A2][0] == c.g[T5][0];
                c.g[A2][0] = sx(r32(c, A2) << 6);
                if parent_is_none {
                    // Root part: the source is the part's own quads, the
                    // matrix the base transform in T4.
                    c.cycles += 58;
                    c.g[A0][0] = c.lw(r32(c, A3).wrapping_add(0xc));
                    let src = r32(c, A0);
                    let base = r32(c, T4);
                    dead_fade_ctor_2_blend_rows(c, src, base);
                } else {
                    // Child part: the matrix is the parent's row in AnimMats.
                    c.cycles += 62;
                    c.g[A1][0] = c.lw(r32(c, A3).wrapping_add(0xc));
                    c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x38));
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    let src = r32(c, A1);
                    let parent_row = r32(c, A2);
                    dead_fade_ctor_2_blend_rows(c, src, parent_row);
                }
                // Copy the blended row to the output block and advance.
                c.cycles += 24;
                c.lqc2(1, r32(c, T1));
                c.lqc2(2, r32(c, T1).wrapping_add(0x10));
                c.lqc2(3, r32(c, T1).wrapping_add(0x20));
                c.lqc2(4, r32(c, T1).wrapping_add(0x30));
                c.sqc2(1, r32(c, T2));
                c.sqc2(2, r32(c, T2).wrapping_add(0x10));
                c.sqc2(3, r32(c, T2).wrapping_add(0x20));
                c.sqc2(4, r32(c, T2).wrapping_add(0x30));
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x38));
                c.g[V0][0] = sx(r32(c, T0) << 6);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.lqc2(1, r32(c, T2));
                c.lqc2(2, r32(c, T2).wrapping_add(0x10));
                c.lqc2(3, r32(c, T2).wrapping_add(0x20));
                c.lqc2(4, r32(c, T2).wrapping_add(0x30));
                c.sqc2(1, r32(c, V0));
                c.sqc2(2, r32(c, V0).wrapping_add(0x10));
                c.sqc2(3, r32(c, V0).wrapping_add(0x20));
                c.sqc2(4, r32(c, V0).wrapping_add(0x30));
                c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x4));
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more_parts = c.g[V0][0] != 0u64;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x10));
                if more_parts {
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_08d8);
                    }
                    continue 'l_3508d8;
                }
                break 'b_350b24;
            }
        }

        'b_350b40: {
            c.cycles += 2;
            let other_is_null = c.g[S2][0] == 0u64;
            c.g[A1][0] = 0x3;
            if other_is_null {
                break 'b_350b40;
            }
            // The C++ `delete other`: a virtual call through other's vtable
            // slot at +0xc (delta at +8, as for the calls above).
            c.cycles += 5;
            c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(0xc));
            c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x8));
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
            let delete_slot = r32(c, V1);
            c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
            call_ptr(c, delete_slot, 0x0035_0b40)?;
        }

        // Instance flags: clear bits 5 and 6, set bit 2.
        c.cycles += 22;
        c.g[A2][0] = c.lw(r32(c, S1).wrapping_add(0x18));
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[A0][0] |= 0xff9f;
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x8));
        c.g[A1][0] |= 0xfffd;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x140);
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        frame.restore(c, S2, 0x120);
        c.sw(r32(c, A2).wrapping_add(0x8), c.g[V1][0]);
        frame.restore(c, S3, 0x110);
        c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x18));
        frame.restore(c, S1, 0x130);
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x8));
        frame.restore(c, S4, 0x100);
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        frame.restore_ra(c, 0xf0);
        c.g[V1][0] |= 0x4;
        c.sw(r32(c, A0).wrapping_add(0x8), c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{fake, fall_back, hook, machine, vtable};
    use crate::difftest::{fill, DATA};

    /// Word that arms the test fake of the world-object constructor.
    const OC605_CTRL: u32 = DATA + 0xf_0000;
    const OC605_MAGIC: u32 = 0x0605_0750;

    static OC605_PREV_WORLD_OBJ_CTOR: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    /// `cInstanceNode_cInstanceNode1` with the gate on: a logged fake call
    /// (it can unwind); with the gate off, the body the slot held before.
    fn oc605_fake_world_obj_ctor(c: &mut Cpu) {
        if c.bus.read32(OC605_CTRL) != OC605_MAGIC {
            return fall_back(
                c,
                &OC605_PREV_WORLD_OBJ_CTOR,
                ee::s11::f_0034fb00__generated,
            );
        }
        let ret = c.g[RA][0] as u32;
        fake(c, 0x0034_fb00, ret);
    }

    /// A node with `seed % 4` parts (the instance, model and parts written
    /// out), `other` null on every third seed, and on every fifth seed its
    /// `hasAnimMats` slot returns zero (the real leaf at 0x1406c0).
    fn oc605_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x35_0750, 6);
        c.pc = 0x0035_0750;
        c.bus.write32(OC605_CTRL, OC605_MAGIC);
        crate::game::scripter::ssxscriptengine::tests::heap_init(c);

        let this = DATA + 0x100;
        let obj = DATA + 0x1000;
        let desc = DATA + 0x2000;
        let other = if seed % 3 == 0 { 0 } else { DATA + 0x3000 };
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(r.next_u32());
        c.g[A2][0] = sx(obj);
        c.g[A3][0] = sx(desc);
        c.g[T0][0] = sx(other);

        c.bus.write32(desc + 0x8, r.f32(20.0));
        c.bus.write32(desc + 0xc, r.f32(20.0));

        let inst = DATA + 0x4000;
        let model = DATA + 0x4400;
        let parts = DATA + 0x4800;
        c.bus.write32(this + 0x18, inst);
        c.bus.write32(inst + 0x80, model);
        c.bus.write32(inst + 0x84, r.f32(2.0));
        c.bus.write32(model + 0x4, seed % 4);
        c.bus.write32(model + 0x8, parts);
        for i in 0..3u32 {
            let part = parts + 0x10 * i;
            let parent = if (seed + i) % 2 == 0 { u32::MAX } else { 0 };
            c.bus.write32(part, parent);
            c.bus.write32(part + 0xc, DATA + 0x5000 + 0x100 * i);
        }
        fill(c, &mut r, DATA + 0x5000, 0x40);

        if other != 0 {
            let vt = vtable(c, &mut r, &[0xc0, 0xd0, 0xd8]);
            if seed % 5 == 0 {
                c.bus.write32(vt + 0xd4, 0x0014_06c0);
            }
            c.bus.write32(other + 0xc, vt);
        }
    }

    #[test]
    fn dead_fade_ctor_2_matches() {
        hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0034FB00),
            oc605_fake_world_obj_ctor,
            &OC605_PREV_WORLD_OBJ_CTOR,
        );
        check(
            ee::s11::f_00350750__generated,
            dead_fade_ctor_2,
            200,
            &|c, seed| oc605_setup(c, seed),
        );
    }
}
