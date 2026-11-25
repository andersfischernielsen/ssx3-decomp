//! Staging module oc587 (wave wave233): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc587`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0034_fca0,
        world_obj_run_instance_script_1a78,
        "WorldObj_RunInstanceScript1a78",
    ),
    (
        0x0034_fce0,
        world_obj_run_update_script,
        "WorldObj_RunUpdateScript",
    ),
    (0x0035_0e70, dead_fade_save, "DeadFade_Save"),
    (
        0x0035_1410,
        intersect_info_save_channels,
        "IntersectInfo_SaveChannels",
    ),
    (
        0x0035_1448,
        intersect_info_load_channels,
        "IntersectInfo_LoadChannels",
    ),
    (0x0035_1330, intersect_info_ctor, "IntersectInfo_ctor"),
    (0x0035_1480, intersect_info_ctor_2, "IntersectInfo_ctor_2"),
    (0x0035_1a10, intersect_info_ctor_3, "IntersectInfo_ctor_3"),
    (
        0x0035_1a60,
        intersect_info_evaluate_channels,
        "IntersectInfo_EvaluateChannels",
    ),
    (0x0035_2168, mesh_anim_ctor, "MeshAnim_ctor"),
    (0x0035_2780, mesh_anim_kill, "MeshAnim_Kill"),
    (
        0x0035_2e98,
        modifier_block_update_transform,
        "ModifierBlock_UpdateTransform",
    ),
    (0x0035_3ae8, sky_box_dtor, "SkyBox_dtor"),
];

// Callees (direct calls go through the readable alias, which dispatches
// through the callee's override slot).
pub(crate) const WORLD_OBJ_SAVE_CALLEE: Callee = Callee {
    addr: 0x0034_fe90,
    f: ee::s11::f_0034fe90,
};
pub(crate) const WORLD_OBJ_REPLACE_DEAD_CALLEE: Callee = Callee {
    addr: 0x0034_fe28,
    f: ee::s11::f_0034fe28,
};
pub(crate) const WORLD_OBJ_CTOR_2_CALLEE: Callee = Callee {
    addr: 0x0034_fb90,
    f: ee::s11::f_0034fb90,
};
pub(crate) const GAME_PROCESS_DTOR_CALLEE: Callee = Callee {
    addr: 0x0035_46c8,
    f: ee::s11::f_003546c8,
};
pub(crate) const INTERSECT_INFO_CLEAR_CALLEE: Callee = Callee {
    addr: 0x0035_13d0,
    f: ee::s11::f_003513d0,
};
pub(crate) const INTERSECT_INFO_SET_SOURCE_CALLEE: Callee = Callee {
    addr: 0x0035_1398,
    f: ee::s11::f_00351398,
};
pub(crate) const INTERSECT_INFO_EVALUATE_CALLEE: Callee = Callee {
    addr: 0x0035_1538,
    f: ee::s11::f_00351538,
};
pub(crate) const INTERSECT_INFO_EVALUATE_MATRIX_CALLEE: Callee = Callee {
    addr: 0x0035_1800,
    f: ee::s11::f_00351800,
};
pub(crate) const INTERSECT_INFO_CTOR_CALLEE: Callee = Callee {
    addr: 0x0035_1330,
    f: ee::s11::f_00351330,
};
pub(crate) const INTERSECT_INFO_EVALUATE_CHANNELS_CALLEE: Callee = Callee {
    addr: 0x0035_1a60,
    f: ee::s11::f_00351a60,
};

/// `void WorldObj_RunInstanceScript1a78(WorldObj *self)` (0x34fca0): run the
/// script hook at `self+0x18` through `WScriptMan_RunObjScript0Hook`
/// (0x2d1a78) and return.
pub fn world_obj_run_instance_script_1a78(c: &mut Cpu) {
    tail_wrapper(c, Some(0x18), Target::Table(0x002d_1a78), 0x0034_fcb0);
}

/// `void WorldObj_RunUpdateScript(WorldObj *self)` (0x34fce0): as the
/// instance-script version, through the script hook at 0x2d1a30.
pub fn world_obj_run_update_script(c: &mut Cpu) {
    tail_wrapper(c, Some(0x18), Target::Table(0x002d_1a30), 0x0034_fcf0);
}

/// `void DeadFade_Save(DeadFade *self)` (0x350e70): save the world object
/// part through `WorldObj_Save` and return.
pub fn dead_fade_save(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(WORLD_OBJ_SAVE_CALLEE), 0x0035_0e80);
}

/// `void IntersectInfo_EvaluateChannels(IntersectInfo *self)` (0x351a60):
/// evaluate the intersect channels through `IntersectInfo_Evaluate`.
pub fn intersect_info_evaluate_channels(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(INTERSECT_INFO_EVALUATE_CALLEE),
        0x0035_1a70,
    );
}

/// `void IntersectInfo_SaveChannels(IntersectInfo *self, Obj **a1)` (0x351410):
/// call the hook at `*(*a1 + 0xc)` with `a0 = a1 + lh(*a1 + 8)`, `a1 = self`,
/// `a2 = 0x40`.
pub fn intersect_info_save_channels(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = 0x40;
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_1438)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void IntersectInfo_LoadChannels(IntersectInfo *self, Obj **a1)` (0x351448):
/// the load twin of [`intersect_info_save_channels`], with the hook at
/// `*(*a1 + 0x14)` and `a0 = a1 + lh(*a1 + 0x10)`.
pub fn intersect_info_load_channels(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = 0x40;
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_1470)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void IntersectInfo_ctor(IntersectInfo *self, float r)` (0x351330): clear,
/// set the source, then evaluate, with `r` kept in f20 across the calls
/// (and passed again to the evaluation in f12).
pub fn intersect_info_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_add(0xaf0));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        let vt = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x84)).sw(c, vt);
        c.f[20] = c.f[12];
        c.g[RA][0] = sx(0x0035_1360);
        INTERSECT_INFO_CLEAR_CALLEE.call(c, 0x0035_1360)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        INTERSECT_INFO_SET_SOURCE_CALLEE.call(c, 0x0035_136c)?;
        c.f[12] = c.f[20];
        c.g[A0][0] = c.g[S0][0];
        INTERSECT_INFO_EVALUATE_CALLEE.call(c, 0x0035_1378)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void IntersectInfo_ctor_2(IntersectInfo *self, Obj *o, float r)` (0x351480):
/// the plain constructor, then copy the 64-byte default matrix from
/// 0x4ff1a0 into `self+0x90` (VU0 quadwords), store `o` at `+0x40`, and
/// evaluate the matrix.
pub fn intersect_info_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.f[20] = c.f[12];
        c.g[RA][0] = sx(0x0035_14a4);
        INTERSECT_INFO_CTOR_CALLEE.call(c, 0x0035_14a4)?;
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_add(0xad0));
        c.g[A0][0] = sx(0x0050_0000u32.wrapping_sub(0xe60));
        let vt = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x84)).sw(c, vt);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
        let src = c.g[A0][0] as u32;
        let dst = c.g[V0][0] as u32;
        c.lqc2(1, src);
        c.lqc2(2, src.wrapping_add(0x10));
        c.lqc2(3, src.wrapping_add(0x20));
        c.lqc2(4, src.wrapping_add(0x30));
        c.sqc2(1, dst);
        c.sqc2(2, dst.wrapping_add(0x10));
        c.sqc2(3, dst.wrapping_add(0x20));
        c.sqc2(4, dst.wrapping_add(0x30));
        let owner = c.g[S1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x40)).sw(c, owner);
        c.f[12] = c.f[20];
        c.g[A0][0] = c.g[S0][0];
        INTERSECT_INFO_EVALUATE_MATRIX_CALLEE.call(c, 0x0035_14ec)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void IntersectInfo_ctor_3(IntersectInfo *self, float r)` (0x351a10): the
/// plain constructor, then store the 0x490ab0 table pointer at `+0x84` and
/// evaluate the channels.
pub fn intersect_info_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0035_1a2c);
        c.f[20] = c.f[12];
        INTERSECT_INFO_CTOR_CALLEE.call(c, 0x0035_1a2c)?;
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_add(0xab0));
        c.f[12] = c.f[20];
        c.g[A0][0] = c.g[S0][0];
        let table = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x84)).sw(c, table);
        INTERSECT_INFO_EVALUATE_CHANNELS_CALLEE.call(c, 0x0035_1a44)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void MeshAnim_ctor(MeshAnim *self, Obj **a2)` (0x352168): run the
/// world-object constructor, store the vtable word at `+0xc`, count down 24
/// steps (no effect), call the hook at `*(*a2 + 0x14)` with
/// `a0 = a2 + lh(*a2 + 0x10)`, and set the float at `self+0x1c` to -1.0.
pub fn mesh_anim_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        WORLD_OBJ_CTOR_2_CALLEE.call(c, 0x0035_2184)?;
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x928));
        c.g[A0][0] = u64::MAX;
        let vt = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sw(c, vt);
        c.g[V1][0] = 0x17;
        loop {
            c.cycles += 7;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = c.g[V1][0] != c.g[A0][0];
            c.g[V0][0] = 0x17;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_2198);
            }
        }
        c.cycles += 21;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = Addr(c.g[A2][0] as u32).off(0x10).lh(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x14).lw(c);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let target = c.g[V0][0] as u32;
        c.g[A2][0] = 0xc;
        call_ptr(c, target, 0x0035_21e4)?;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.f[0] = 0xbf80_0000;
        c.g[V0][0] = c.g[S0][0];
        let bits = c.f[0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x1c)).write_u32(c, bits);
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MeshAnim_Kill(MeshAnim *self)` (0x352780): when `self+0x6b0` is set,
/// rewrite the flags word at `+8` of the block at `self+0x18` (keep the
/// masked bits, or in the sign-extended high half, then set bit 1), and if
/// `self` is non-null call the hook at `*(self+0xc)`; when it is clear,
/// replace the node with a dead one.
pub fn mesh_anim_kill(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x6b0).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 16;
            c.g[A1][0] = Addr(c.g[A2][0] as u32).off(0x18).lw(c);
            c.g[V1][0] = 0xffff_ffff_ffff_0300;
            c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x8).lw(c);
            let mask = c.g[V1][0];
            c.g[V0][0] &= mask;
            let v = c.g[V0][0];
            Addr(c.g[A1][0] as u32).off(0x8).sw(c, v);
            c.g[A0][0] = Addr(c.g[A2][0] as u32).off(0x18).lw(c);
            c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x8).lw(c);
            c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
            let hi = c.g[V1][0];
            c.g[V0][0] |= hi;
            let v = c.g[V0][0];
            Addr(c.g[A0][0] as u32).off(0x8).sw(c, v);
            c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x18).lw(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x8).lw(c);
            c.g[V0][0] |= 0x2;
            let v = c.g[V0][0];
            Addr(c.g[V1][0] as u32).off(0x8).sw(c, v);
            if c.g[A2][0] == 0 {
                c.cycles += 1;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
                c.g[A1][0] = 0x3;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0035_27f0)?;
                frame.restore_ra(c, 0);
            }
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[A2][0];
            WORLD_OBJ_REPLACE_DEAD_CALLEE.call(c, 0x0035_2800)?;
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void ModifierBlock_UpdateTransform(ModifierBlock **self)` (0x352e98): when
/// the block `*self` is set, call the hook at `*(*block) + 0x1c` with
/// `a0 = block + lh(*(*block) + 0x18)` and `a1 = block`.
pub fn modifier_block_update_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(c.g[A0][0] as u32).lw(c);
        let null = c.g[A1][0] == 0;
        frame.restore_ra(c, 0);
        if !null {
            c.cycles += 6;
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0035_2ec0)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void SkyBox_dtor(SkyBox *self)` (0x353ae8): set the vtable word at
/// `self+0xc` and run `GameProcess_dtor` (0x3546c8) on it.
pub fn sky_box_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0xff8));
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        let vt = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, vt);
        GAME_PROCESS_DTOR_CALLEE.call(c, 0x0035_3b04)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fall_back, hook};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the no-op callee fakes (test RAM, away from the fixtures).
    const GATE: u32 = DATA + 0x20_0000;
    const GATE_ON: u32 = 0x0587_0587;
    const SEEDS: u32 = 64;

    fn gate_on(c: &mut Cpu) {
        c.bus.write32(GATE, GATE_ON);
    }

    /// A callee that returns at once with the registers as they are while
    /// the gate is on; otherwise the body it replaced.
    macro_rules! noop_fake {
        ($fake:ident, $prev:ident, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(GATE) != GATE_ON {
                    return fall_back(c, &$prev, $gen);
                }
                ret(c);
            }
        };
    }

    noop_fake!(
        oc587_fake_world_obj_save,
        OC587_PREV_WORLD_OBJ_SAVE,
        ee::s11::f_0034fe90__generated
    );
    noop_fake!(
        oc587_fake_game_process_dtor,
        OC587_PREV_GAME_PROCESS_DTOR,
        ee::s11::f_003546c8__generated
    );
    noop_fake!(
        oc587_fake_evaluate,
        OC587_PREV_EVALUATE,
        ee::s11::f_00351538__generated
    );
    noop_fake!(
        oc587_fake_clear,
        OC587_PREV_CLEAR,
        ee::s11::f_003513d0__generated
    );
    noop_fake!(
        oc587_fake_set_source,
        OC587_PREV_SET_SOURCE,
        ee::s11::f_00351398__generated
    );
    noop_fake!(
        oc587_fake_evaluate_matrix,
        OC587_PREV_EVALUATE_MATRIX,
        ee::s11::f_00351800__generated
    );
    noop_fake!(
        oc587_fake_world_obj_ctor_2,
        OC587_PREV_WORLD_OBJ_CTOR_2,
        ee::s11::f_0034fb90__generated
    );
    noop_fake!(
        oc587_fake_replace_dead,
        OC587_PREV_REPLACE_DEAD,
        ee::s11::f_0034fe28__generated
    );
    noop_fake!(
        oc587_fake_run_update_hook,
        OC587_PREV_RUN_UPDATE_HOOK,
        ee::s09::f_002d1a30__generated
    );

    /// Hook the gated fakes (once per process).
    fn install_fakes() {
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FE90),
            oc587_fake_world_obj_save,
            &OC587_PREV_WORLD_OBJ_SAVE,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_003546C8),
            oc587_fake_game_process_dtor,
            &OC587_PREV_GAME_PROCESS_DTOR,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_00351538),
            oc587_fake_evaluate,
            &OC587_PREV_EVALUATE,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_003513D0),
            oc587_fake_clear,
            &OC587_PREV_CLEAR,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_00351398),
            oc587_fake_set_source,
            &OC587_PREV_SET_SOURCE,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_00351800),
            oc587_fake_evaluate_matrix,
            &OC587_PREV_EVALUATE_MATRIX,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FB90),
            oc587_fake_world_obj_ctor_2,
            &OC587_PREV_WORLD_OBJ_CTOR_2,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FE28),
            oc587_fake_replace_dead,
            &OC587_PREV_REPLACE_DEAD,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D1A30),
            oc587_fake_run_update_hook,
            &OC587_PREV_RUN_UPDATE_HOOK,
        );
    }

    /// A table-call leaf: returns v0 = 0 with no other effect (0x1406c0).
    const LEAF: u32 = 0x0014_06c0;

    #[test]
    fn world_obj_run_instance_script_1a78_matches() {
        check(
            ee::s11::f_0034fca0__generated,
            world_obj_run_instance_script_1a78,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x587_1a78 ^ seed as u64);
                scramble(c, &mut r);
                gate_on(c);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x0034_fca0;
                let obj = DATA + 0x100 + (seed % 4) * 0x40;
                c.g[A0][0] = sx(obj);
                c.bus.write32(obj + 0x18, DATA + 0x200);
            },
        );
    }

    #[test]
    fn world_obj_run_update_script_matches() {
        install_fakes();
        check(
            ee::s11::f_0034fce0__generated,
            world_obj_run_update_script,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x587_fce0 ^ seed as u64);
                scramble(c, &mut r);
                gate_on(c);
                c.pc = 0x0034_fce0;
                let obj = DATA + 0x100 + (seed % 4) * 0x40;
                c.g[A0][0] = sx(obj);
                c.bus.write32(obj + 0x18, DATA + 0x200);
            },
        );
    }

    #[test]
    fn dead_fade_save_matches() {
        install_fakes();
        check(
            ee::s11::f_00350e70__generated,
            dead_fade_save,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x587_0e70 ^ seed as u64);
                scramble(c, &mut r);
                gate_on(c);
                c.pc = 0x0035_0e70;
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 3) * 0x40);
            },
        );
    }

    #[test]
    fn intersect_info_evaluate_channels_matches() {
        install_fakes();
        check(
            ee::s11::f_00351a60__generated,
            intersect_info_evaluate_channels,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x587_1a60 ^ seed as u64);
                scramble(c, &mut r);
                gate_on(c);
                c.pc = 0x0035_1a60;
                c.g[A0][0] = sx(DATA + 0x400 + (seed % 2) * 0x100);
            },
        );
    }

    /// The channel-table fixture shared by the save and load twins: `o`
    /// points at a record whose `+off` halfword and `+off+4` hook pointer
    /// the function reads.
    fn channel_fixture(c: &mut Cpu, seed: u32, off: u32) {
        c.bus.write32(DATA + 0x100, DATA + 0x200);
        c.bus
            .write16(DATA + 0x200 + off, ((seed % 4) * 0x10) as u16);
        c.bus.write32(DATA + 0x200 + off + 4, LEAF);
    }

    #[test]
    fn intersect_info_save_channels_matches() {
        check(
            ee::s11::f_00351410__generated,
            intersect_info_save_channels,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x587_1410 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0035_1410;
                c.g[A0][0] = sx(DATA + 0x300);
                c.g[A1][0] = sx(DATA + 0x100);
                channel_fixture(c, seed, 0x8);
            },
        );
    }

    #[test]
    fn intersect_info_load_channels_matches() {
        check(
            ee::s11::f_00351448__generated,
            intersect_info_load_channels,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x587_1448 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0035_1448;
                c.g[A0][0] = sx(DATA + 0x300);
                c.g[A1][0] = sx(DATA + 0x100);
                channel_fixture(c, seed, 0x10);
            },
        );
    }

    #[test]
    fn intersect_info_ctor_matches() {
        install_fakes();
        check(
            ee::s11::f_00351330__generated,
            intersect_info_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x587_1330 ^ seed as u64);
                scramble(c, &mut r);
                gate_on(c);
                c.pc = 0x0035_1330;
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x800 + (seed % 2) * 0x40);
                c.f[12] = r.next_u32();
            },
        );
    }

    #[test]
    fn intersect_info_ctor_2_matches() {
        install_fakes();
        check(
            ee::s11::f_00351480__generated,
            intersect_info_ctor_2,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x587_1480 ^ seed as u64);
                scramble(c, &mut r);
                gate_on(c);
                c.pc = 0x0035_1480;
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x800 + (seed % 2) * 0x40);
                c.f[12] = r.next_u32();
                for i in 0..16 {
                    c.bus.write32(0x004f_f1a0 + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn intersect_info_ctor_3_matches() {
        install_fakes();
        check(
            ee::s11::f_00351a10__generated,
            intersect_info_ctor_3,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x587_1a10 ^ seed as u64);
                scramble(c, &mut r);
                gate_on(c);
                c.pc = 0x0035_1a10;
                c.g[A0][0] = sx(DATA + 0x400 + (seed % 2) * 0x100);
                c.f[12] = r.next_u32();
            },
        );
    }

    #[test]
    fn mesh_anim_ctor_matches() {
        install_fakes();
        check(
            ee::s11::f_00352168__generated,
            mesh_anim_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x587_2168 ^ seed as u64);
                scramble(c, &mut r);
                gate_on(c);
                c.pc = 0x0035_2168;
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x500);
                c.bus.write32(DATA + 0x500, DATA + 0x600);
                c.bus.write16(DATA + 0x610, ((seed % 4) * 0x10) as u16);
                c.bus.write32(DATA + 0x614, LEAF);
            },
        );
    }

    #[test]
    fn mesh_anim_kill_matches() {
        install_fakes();
        check(
            ee::s11::f_00352780__generated,
            mesh_anim_kill,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x587_2780 ^ seed as u64);
                scramble(c, &mut r);
                gate_on(c);
                c.pc = 0x0035_2780;
                // mode 0: node already dead (replace), 1: live node with a hook,
                // 2: a null self (the store path with the null test taken)
                let mode = seed % 3;
                let this = if mode == 2 { 0 } else { DATA + 0x400 };
                c.g[A0][0] = sx(this);
                if mode == 2 {
                    c.bus.write32(0x6b0, 1);
                } else {
                    c.bus
                        .write32(DATA + 0x400 + 0x6b0, if mode == 0 { 0 } else { 1 });
                    c.bus.write32(DATA + 0x400 + 0x18, DATA + 0x800);
                    c.bus.write32(DATA + 0x400 + 0xc, DATA + 0x900);
                    c.bus.write32(DATA + 0x800 + 0x8, r.next_u32());
                    c.bus.write32(DATA + 0x800, 0x1234_5678);
                    c.bus.write16(DATA + 0x908, 0x20);
                    c.bus.write32(DATA + 0x90c, LEAF);
                }
            },
        );
    }

    #[test]
    fn modifier_block_update_transform_matches() {
        check(
            ee::s11::f_00352e98__generated,
            modifier_block_update_transform,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x587_2e98 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0035_2e98;
                c.g[A0][0] = sx(DATA + 0x100);
                c.bus
                    .write32(DATA + 0x100, if seed % 3 == 0 { 0 } else { DATA + 0x200 });
                c.bus.write32(DATA + 0x200, DATA + 0x300);
                c.bus.write16(DATA + 0x318, ((seed % 4) * 0x10) as u16);
                c.bus.write32(DATA + 0x31c, LEAF);
            },
        );
    }

    #[test]
    fn sky_box_dtor_matches() {
        install_fakes();
        check(
            ee::s11::f_00353ae8__generated,
            sky_box_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x587_3ae8 ^ seed as u64);
                scramble(c, &mut r);
                gate_on(c);
                c.pc = 0x0035_3ae8;
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 2) * 0x40);
            },
        );
    }
}
