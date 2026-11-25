//! `src/object/multiparticle.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::set_fcc;
use crate::abi::set_fpr;
use crate::abi::set_reg;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::aianimeventmap::BUILTIN_DELETE;
pub(crate) use crate::game::animation::animmodel::EA_MEM_FREE_ADDR;
pub(crate) use crate::game::animation::rideranimbase::OPERATOR_NEW_NAMED2_ADDR;
pub(crate) use crate::game::dirtysock::tagsunk::gp_3 as gp;
pub(crate) use crate::game::fe::feasyncfile::addu;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp_13 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p;
pub(crate) use crate::game::fe::feasyncfile::sub;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fepopup::at as at_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::fe::fepopupmisc::gp_6 as gp_5;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::intersect::worldsphtree::save_field_block;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::p_2 as p_3;
pub(crate) use crate::game::main::ssxappdtor::p as p_7;
pub(crate) use crate::game::object::modifierblock::oc664_stream_read;
pub(crate) use crate::game::object::movenode::MP_COUNT;
pub(crate) use crate::game::object::movenode::MP_EMITTER;
pub(crate) use crate::game::object::movenode::MP_STREAM_WRITE;
pub(crate) use crate::game::seg::seg_1fbe38::gp_oc322;
pub(crate) use crate::game::sound::soundsys::gp_4;
pub(crate) use crate::game::sound::soundsys::p as p_5;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_8120, multi_particle_update, "MultiParticle_Update"),
    (
        0x0035_8140,
        multi_particle_remove_section_instances,
        "MultiParticle_RemoveSectionInstances",
    ),
    (
        0x0035_81b8,
        multi_particle_add_instance,
        "MultiParticle_AddInstance",
    ),
    (
        0x0035_81f0,
        multi_particle_remove_instance,
        "MultiParticle_RemoveInstance",
    ),
    (0x0035_84f0, rail_man_ctor, "RailMan_ctor"),
    (0x0035_8588, railman_ctor_2, "RailMan_ctor_2"),
    (0x0035_8668, rail_man_dtor, "RailMan_dtor"),
    (
        0x0035_8700,
        railman_on_section_unload,
        "RailMan_OnSectionUnload",
    ),
    (0x0035_8998, rail_man_enable_rail, "RailMan_EnableRail"),
    (0x0035_8c30, rail_man_save, "RailMan_Save"),
    (0x0035_8db0, anim_combo_ctor_2, "AnimCombo_ctor_2"),
    (0x0035_8e50, anim_combo_dtor, "AnimCombo_dtor"),
    (0x0035_8f90, anim_combo_v42, "AnimCombo_v42"),
    (0x0035_8fd0, anim_combo_v36, "AnimCombo_v36"),
    (
        0x0035_9030,
        anim_combo_snapshot_matrices,
        "AnimCombo_SnapshotMatrices",
    ),
    (0x0035_90c8, anim_combo_kill, "AnimCombo_Kill"),
    (0x0035_90f0, anim_combo_save, "AnimCombo_Save"),
    (0x0035_9140, anim_combo_v52_2, "AnimCombo_v52"),
    (0x0035_9228, anim_delta_ctor, "AnimDelta_ctor"),
    (0x0035_93d0, anim_delta_v36, "AnimDelta_v36"),
    (0x0035_9410, anim_delta_save, "AnimDelta_Save"),
    (
        0x0035_8260,
        multi_particle_emit_at_instances,
        "MultiParticle_EmitAtInstances",
    ),
    (0x0035_4920, sync_process_dtor, "SyncProcess_dtor"),
    (0x0035_8380, multi_particle_save, "MultiParticle_Save"),
    (0x0035_9460, spline_modifier_ctor, "SplineModifier_ctor"),
    (
        0x0035_8020,
        multi_particle_create_emitter,
        "MultiParticle_CreateEmitter",
    ),
    (
        0x0035_8468,
        boost_modifier_base_ctor_2,
        "BoostModifier_BaseCtor_2",
    ),
    (0x0035_8780, rail_man_disable_rail, "RailMan_DisableRail"),
    (
        0x0035_8b28,
        rail_man_set_rail_enabled,
        "RailMan_SetRailEnabled",
    ),
    (0x0035_8c80, anim_combo_ctor, "AnimCombo_ctor"),
    (0x0035_8eb8, anim_combo_v15, "AnimCombo_v15"),
    (0x0035_9270, anim_delta_ctor_2_3, "AnimDelta_ctor_2"),
    (0x0035_92d0, anim_delta_v15_3, "AnimDelta_v15"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::ai::rider::callee::RAIL_MAN_ENABLE_RAIL;

    pub(crate) use crate::game::ai::rider::callee::SYNC_PROCESS_DTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::ANIM_OBJECT_V36;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ANIM_OBJECT_SAVE;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SYNC_PROCESS_SAVE;

    pub const ANIM_OBJECT_CTOR: Callee = Callee {
        addr: 0x0034_1aa0,
        f: ee::s11::f_00341aa0,
    };
}

// ---------------------------------------------------------------------------
// AnimCombo_v52 (0x00359140).

/// `cAnimCombo` fields used here.
pub(crate) mod anim_combo {

    /// `+0x14`/`+0x15`/`+0x17`: per-part enable bytes.
    pub const PART_A: u32 = 0x14;

    pub const PART_B: u32 = 0x15;

    pub const PART_C: u32 = 0x17;

    /// The object whose vertex arrays are blended.
    pub const MODEL: u32 = 0x60;

    /// The animation object and its per-part output matrices.
    pub const ANIM: u32 = 0x7c;

    pub const MATRICES: u32 = 0x84;
}

// ---------------------------------------------------------------------------
// Particles / modifiers.

/// `void MultiParticle_AddInstance(MultiParticle *p, void *inst)`
/// (0x3581b8): append `inst` to the instance array if there is room.
pub fn multi_particle_add_instance(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A2][0] = c.g[A0][0];
    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, c.g[A1][0]);
        c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[A0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

pub(crate) const ANIM_OBJECT_DTOR: Callee = Callee {
    addr: 0x0034_1cf0,
    f: ee::s11::f_00341cf0,
};

pub(crate) const MOD_OBJ_V42: Callee = Callee {
    addr: 0x0035_6ae0,
    f: ee::s11::f_00356ae0,
};

pub(crate) const WORLD_OBJ_KILL: Callee = Callee {
    addr: 0x0034_fd90,
    f: ee::s11::f_0034fd90,
};

pub(crate) const ANIM_OBJECT_SAVE: Callee = Callee {
    addr: 0x0034_2150,
    f: ee::s11::f_00342150,
};

pub(crate) const ANIM_OBJECT_V52: Callee = Callee {
    addr: 0x0034_e348,
    f: ee::s11::f_0034e348,
};

pub(crate) const ANIM_OBJECT_CTOR_2: Callee = Callee {
    addr: 0x0034_1c80,
    f: ee::s11::f_00341c80,
};

pub(crate) const ANIM_COMBO_SNAPSHOT: Callee = Callee {
    addr: 0x0035_9030,
    f: ee::s11::f_00359030,
};

pub(crate) const INTERSECT_INFO_MUL_MATRIX: Callee = Callee {
    addr: 0x0035_1948,
    f: ee::s11::f_00351948,
};

/// `AnimCombo_dtor(AnimCombo *this, uint flags)` (0x358e50): set the vtable at
/// +0x54, free the snapshot matrices (+0x84), destruct the `AnimObject` at
/// +0x18 and, when `flags & 1`, delete `this`.
pub fn anim_combo_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x49_0000u32.wrapping_sub(0x13d8));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x84)).lw(c);
        let no_snap = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_snap {
            c.cycles += 2;
            call_ptr(c, EA_MEM_FREE_ADDR, 0x0035_8e84)?;
        }
        c.cycles += 6;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = 0;
        ANIM_OBJECT_DTOR.call(c, 0x0035_8e90)?;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            BUILTIN_DELETE.call(c, 0x0035_8ea4)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AnimCombo_v42(SubObj *sub, void *out)` (0x358f90): call `ModObj_v42` on
/// `sub+0x48`, then copy the two fixed quads at 0x4ff120 to `out+0x20` and
/// `out+0x30`.
pub fn anim_combo_v42(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        MOD_OBJ_V42.call(c, 0x0035_8fa8)?;
        c.g[A0][0] = 0x50_0000;
        frame.restore_ra(c, 0);
        c.g[A0][0] = sx(0x50_0000u32.wrapping_sub(0xee0));
        c.lq(2, c.g[A0][0] as u32);
        c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.lq(3, c.g[A0][0] as u32);
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0x30));
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `AnimCombo *AnimCombo_ctor_2(AnimCombo *this, void *a1, void *a2)`
/// (0x358db0): construct the `AnimObject` at +0x18, run the vtable slot at
/// `*a2 -> +0x10/+0x14`, allocate the snapshot buffer from the bone count and
/// snapshot the matrices.
pub fn anim_combo_ctor_2(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x84));
        ANIM_OBJECT_CTOR_2.call(c, 0x0035_8dd8)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = sx(0x49_0000u32.wrapping_sub(0x13d8));
        c.g[A2][0] = 0x18;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V1][0]);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x0035_8e00)?;
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x74)).lw(c);
        c.g[A1][0] = 0x49_0000;
        c.g[A1][0] = sx(0x49_0000u32.wrapping_sub(0x15e0));
        c.g[A2][0] = 0x2000_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32) << 6);
        c.g[A3][0] = 0;
        call_ptr(c, OPERATOR_NEW_NAMED2_ADDR, 0x0035_8e1c)?;
        let empty = c.g[S0][0] == 0;
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
        if !empty {
            c.cycles += 1;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 10;
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        ANIM_COMBO_SNAPSHOT.call(c, 0x0035_8e34)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimCombo_Kill(AnimCombo *this)` (0x3590c8): kill the `WorldObj` at
/// +0x48 unless the alive flag at +0x15 is set.
pub fn anim_combo_kill(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x15)).lb(c);
        let alive = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !alive {
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
            WORLD_OBJ_KILL.call(c, 0x0035_90e4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void AnimCombo_Save(AnimCombo *this, State *out)` (0x3590f0): save the
/// `AnimObject` at +0x18 and forward to `out->save` (`out+0 -> +8/+0xc`).
pub fn anim_combo_save(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
        ANIM_OBJECT_SAVE.call(c, 0x0035_9110)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x18;
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0035_912c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimCombo_v52(AnimCombo *this)` (0x359140): forward to
/// `AnimObject_v52(this+0x34)` and, when the bones are ready, multiply every
/// bone matrix into the snapshot buffer.
pub fn anim_combo_v52(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
        ANIM_OBJECT_V52.call(c, 0x0035_916c)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x7c)).lw(c);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x60);
            anim_combo_v52_epilogue(c, &frame);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x14)).lb(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x17)).lb(c);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x60);
                anim_combo_v52_epilogue(c, &frame);
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x15)).lb(c);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x60);
                anim_combo_v52_epilogue(c, &frame);
                return Ok(());
            }
            c.cycles += 1;
        }
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x60)).lw(c);
        c.cycles += 6;
        c.g[S5][0] = 0;
        c.g[S2][0] = 0;
        c.g[S3][0] = Addr(c.g[V1][0] as u32).off(0x80).lw(c);
        c.g[V0][0] = Addr(c.g[S3][0] as u32).off(4).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = Addr(c.g[S3][0] as u32).off(8).lw(c);
        if !empty {
            c.cycles += 2;
            c.g[S4][0] = 0;
            loop {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(8).lw(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1c).lw(c);
                let skip = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[S5][0] as u32) << 6);
                if !skip {
                    c.cycles += 7;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x7c)).lw(c);
                    c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0xd0));
                    INTERSECT_INFO_MUL_MATRIX.call(c, 0x0035_91ec)?;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S3][0] as u32).off(4).lw(c);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_91c0);
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x60);
        anim_combo_v52_epilogue(c, &frame);
        Ok(())
    })
}

/// The shared epilogue of [`anim_combo_v52`].
pub(crate) fn anim_combo_v52_epilogue(c: &mut Cpu, frame: &Frame) {
    c.cycles += 8;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void AnimCombo_SnapshotMatrices(AnimCombo *this)` (0x359030): copy each
/// bone's four quads (stepping 0xd0) into the snapshot buffer; VU0 `lqc2`/
/// `sqc2` set VF1-VF4 (the last copied bone is left in them).
pub fn anim_combo_snapshot_matrices(c: &mut Cpu) {
    c.cycles += 4;
    let t0 = c.g[A0][0];
    c.g[T0][0] = t0;
    c.g[V0][0] = Addr(t0 as u32).off(0x74).lw(c);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[T2][0] = 0;
    if empty {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = Addr(t0 as u32).off(0x60).lw(c);
    c.g[A2][0] = 0;
    c.g[A3][0] = Addr(c.g[V1][0] as u32).off(0x80).lw(c);
    c.g[V0][0] = Addr(c.g[A3][0] as u32).off(4).lw(c);
    let empty2 = (c.g[V0][0] as i64) <= 0;
    c.g[A1][0] = Addr(c.g[A3][0] as u32).off(8).lw(c);
    if empty2 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[T3][0] = sx((t0 as u32).wrapping_add(0x34));
    c.g[T1][0] = 0;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(8).lw(c);
        let skip = c.g[V0][0] == 0;
        c.g[V1][0] = sx((c.g[T2][0] as u32) << 6);
        if !skip {
            c.cycles += 15;
            c.g[V0][0] = Addr(c.g[T3][0] as u32).off(0x48).lw(c);
            c.g[A0][0] = Addr(t0 as u32).off(0x84).lw(c);
            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x90));
            let src = c.g[V0][0] as u32;
            let dst = c.g[V1][0] as u32;
            c.lqc2(1, src);
            c.lqc2(2, src.wrapping_add(0x10));
            c.lqc2(3, src.wrapping_add(0x20));
            c.lqc2(4, src.wrapping_add(0x30));
            c.sqc2(1, dst);
            c.sqc2(2, dst.wrapping_add(0x10));
            c.sqc2(3, dst.wrapping_add(0x20));
            c.sqc2(4, dst.wrapping_add(0x30));
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0xd0));
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
        }
        c.cycles += 5;
        c.g[V0][0] = Addr(c.g[A3][0] as u32).off(4).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0035_9060);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void MultiParticle_Update(MultiParticle* p)` (0x358120): update the
/// particle system at `p->+0x10`, stepping it by the frame time at
/// `gp - 0x2a80`.
pub fn multi_particle_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.lwc1(12, r32(c, GP).wrapping_sub(0x2a80));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0x10));
        call_ptr(c, 0x0037_0788, 0x0035_8134)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimDelta_v36(AnimObject* self, int id, float v)` (0x3593D0): id 0x8C
/// advances the object's delta at +0x6C by v/30, any other id is the generic
/// anim-object update.
pub fn anim_delta_v36(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x8c;
        let id = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        if id != c.g[V0][0] {
            c.cycles += 2;
            callee::ANIM_OBJECT_V36.call(c, 0x0035_9400)?;
        } else {
            c.cycles += 6;
            c.ld_f(0, gp(c, -0x2a68));
            c.ld_f(1, (this as u32).wrapping_add(0x6c));
            c.mul_s(0, 12, 0);
            c.add_s(1, 1, 0);
            c.st_f(1, (this as u32).wrapping_add(0x6c));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AnimDelta* AnimDelta_ctor(this, a, b, c)` (0x359228): construct the
/// anim-object base (`AnimObject_ctor(this, a, 2, b, c)`), then install the
/// delta vtable and clear the delta state.
pub fn anim_delta_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[T0][0] = c.g[A3][0];
        frame.save(c, S0, 0x10);
        c.g[A3][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x2;
        ANIM_OBJECT_CTOR.call(c, 0x0035_9248)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_sub(0x1590)); // &D_0048EA70
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x6c).sw(c, 0);
        Addr(this).off(0x3c).sw(c, c.g[V1][0]);
        Addr(this).off(4).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RailMan_Save(SyncProcess* self, Stream* stream)` (0x358c30): 0x200
/// bytes at `+0x18`.
pub fn rail_man_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(
            c,
            callee::SYNC_PROCESS_SAVE,
            0x18,
            0x200,
            0x0035_8c4c,
            0x0035_8c68,
        )
    })
}

/// `void AnimDelta_Save(AnimDelta* self, Stream* stream)` (0x359410): 4 bytes
/// at `+0x6c`.
pub fn anim_delta_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(
            c,
            callee::ANIM_OBJECT_SAVE,
            0x6c,
            4,
            0x0035_942c,
            0x0035_9448,
        )
    })
}

// ---------------------------------------------------------------------------
// Animation objects.

/// `void AnimCombo_v36(AnimCombo* self, int msg)` (0x358fd0): on the "snapshot"
/// message copy the current pose into the blend source; otherwise pass the
/// message to the animation object it embeds.
pub fn anim_combo_v36(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x82;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let msg_is_snapshot = c.g[A1][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if !msg_is_snapshot {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x14)).lhu(c);
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 8;
                c.lwc1(0, p(c.g[S0][0], 0x38));
                c.swc1(0, p(c.g[S0][0], 0x10));
                call_ptr(c, 0x0035_9030, 0x0035_9000)?;
                c.lwc1(0, p(c.g[S0][0], 8));
                c.g[V0][0] = 1;
                Addr(p(c.g[S0][0], 0x14)).sb(c, c.g[V0][0]);
                c.swc1(0, p(c.g[S0][0], 4));
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
            }
        } else {
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0034_1fe8, 0x0035_901c)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `AnimDelta* AnimDelta_ctor_2(AnimDelta* self, int a1, Stream* stream)`
/// (0x359270): base animation object, its vtable and the stream registration.
pub fn anim_delta_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0034_1c80, 0x0035_928c)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1590));
        c.g[A2][0] = 4;
        Addr(p(c.g[S0][0], 0x3c)).sw(c, c.g[V1][0]);
        c.g[V0][0] = lw(c, c.g[S1][0] as u32);
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x10)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_92b4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MultiParticle_RemoveInstance(MultiParticle* p_2, int key)`
/// (0x3581f0): remove `key` from the instance array (swap with the last) or
/// mark the slot empty.
pub fn multi_particle_remove_instance(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T0][0] = Addr(p_2(r32(c, A0), 4)).lw(c);
    let none = (c.g[T0][0] as i64) <= 0;
    c.g[A3][0] = 0;
    if none {
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
        return;
    }
    c.cycles += 3;
    c.g[T2][0] = 0xffff_ffff_ffff_ffff;
    c.g[T1][0] = Addr(p_2(r32(c, A0), 0xc)).lw(c);
    loop {
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
        c.g[A2][0] = abi::addu(c.g[V0][0], c.g[T1][0]);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        if c.g[V1][0] != c.g[A1][0] {
            c.g[T0][0] = Addr(p_2(r32(c, A0), 4)).lw(c);
            c.cycles += 4;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[T1][0] = Addr(p_2(r32(c, A0), 0xc)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_8208);
                }
                continue;
            }
            break;
        } else {
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let last = c.g[V0][0] == 0;
            Addr(p_2(r32(c, A0), 4)).sw(c, c.g[V1][0]);
            if last {
                c.cycles += 2;
                Addr(c.g[A2][0] as u32).sw(c, c.g[T2][0]);
            } else {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = abi::addu(c.g[V0][0], c.g[T1][0]);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                Addr(c.g[A2][0] as u32).sw(c, c.g[V1][0]);
            }
            let ra = c.g[RA][0] as u32;
            c.pc = ra;
            return;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
}

/// `lbu off(reg)`.
#[inline]
pub(crate) fn lbu_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lbu(c)
}

/// `func_00358140` (0x358140): remove every instance of section `a1` from a
/// multi-particle object. Walks the instance array (`+0xc`, count at `+4`) and
/// for each match replaces the slot with the last instance (and shrinks the
/// count), then retries the same index. The loop back edge carries an event
/// check. Signature: `void remove(MultiParticle* this, u8 section)`.
pub fn multi_particle_remove_section_instances(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T0][0] = lw_r(c, A0, 0x4);
    let empty = (c.g[T0][0] as i64) <= 0;
    c.g[A2][0] = 0;
    if empty {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[T2][0] = 0xffff_ffff_ffff_0000;
    c.g[T2][0] = c.g[T2][0] | 0xffff;
    c.g[T1][0] = lw_r(c, A0, 0xc);
    loop {
        c.cycles += 5;
        c.g[V0][0] = sx(low(c, A2) << 2);
        c.g[A3][0] = sx(low(c, V0).wrapping_add(low(c, T1)));
        c.g[V1][0] = lbu_r(c, A3, 0);
        if c.g[V1][0] != c.g[A1][0] {
            c.g[T0][0] = lw_r(c, A0, 0x4);
        } else {
            c.cycles += 4;
            c.g[V1][0] = sx(low(c, T0).wrapping_sub(1));
            let less = (c.g[A2][0] as i64) < (c.g[V1][0] as i64);
            c.g[V0][0] = less as u64;
            let last = c.g[V0][0] == 0;
            sw_r(c, A0, 0x4, c.g[V1][0]);
            if last {
                c.cycles += 1;
                sw_r(c, A3, 0, c.g[T2][0]);
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx(low(c, V1) << 2);
                c.g[A2][0] = sx(low(c, A2).wrapping_sub(1));
                c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, T1)));
                c.g[V1][0] = Addr(low(c, V0)).lw(c);
                sw_r(c, A3, 0, c.g[V1][0]);
            }
            c.cycles += 1;
            c.g[T0][0] = lw_r(c, A0, 0x4);
        }
        c.cycles += 4;
        c.g[A2][0] = sx(low(c, A2).wrapping_add(1));
        let more = (c.g[A2][0] as i64) < (c.g[T0][0] as i64);
        c.g[V0][0] = more as u64;
        if !more {
            break;
        }
        c.g[T1][0] = lw_r(c, A0, 0xc);
        if event_due(c) {
            deliver_event_here(c, 0x0035_8158);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void RailMan_OnSectionUnload(kind)` (0x358700): for every rail slot
/// (`D_004A4040`, 128 entries) with a kind tag matching `kind`, re-enable the
/// rail.
pub fn railman_on_section_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = lw_at(c, gw(c, 0xf50));
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        let empty = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !empty {
            c.cycles += 4;
            c.g[S1][0] = 0xffff_ffff_ffff_0000;
            c.g[S0][0] = 0;
            c.g[S1][0] |= 0xffff;
            let mut m = lw_at(c, gw(c, 0xf50));
            loop {
                c.cycles += 5;
                c.g[A0][0] = m;
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                let slot = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = slot;
                c.g[V1][0] = lw_at(c, p_3(slot as u32, 0x18));
                let dead = c.g[V1][0] == c.g[S1][0];
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                if !dead {
                    c.cycles += 3;
                    c.g[V1][0] = Addr(p_3(c.g[V0][0] as u32, 0x18)).lbu(c);
                    if c.g[V1][0] != c.g[S2][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x0035_8998, 0x0035_8758)?;
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    }
                } else {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 128) as u64;
                c.g[A0][0] = lw_at(c, gw(c, 0xf50));
                if c.g[V0][0] == 0 {
                    break;
                }
                m = c.g[A0][0];
                if event_due(c) {
                    deliver_event_here(c, 0x0035_8730);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RailMan_dtor(self, flags)` (0x358668): free every live rail object and
/// chain to the base destructor.
pub fn rail_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x490000;
        frame.save(c, S2, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1220));
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
        frame.save(c, S3, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0xffff_ffff_ffff_0000;
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        Addr(c.g[S2][0] as u32).off(0xc).sw(c, c.g[V0][0]);
        c.g[S3][0] = c.g[S3][0] | 0xffff;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            let unused = c.g[V0][0] == c.g[S3][0];
            c.g[A0][0] = c.g[S2][0];
            if !unused {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                callee::RAIL_MAN_ENABLE_RAIL.call(c, 0x0035_86bc)?;
            }
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 128) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_86a8);
            }
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S4][0];
        Addr(gp_2(c, 0xf50)).sw(c, 0);
        callee::SYNC_PROCESS_DTOR.call(c, 0x0035_86dc)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RailMan_ctor` (0x3584f0): construct the sync process and clear the rail
/// manager's two 128-entry tables.
pub fn rail_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0x12;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0035_8508);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        callee::SYNC_PROCESS_CTOR.call(c, 0x0035_8508)?;
        c.g[V0][0] = 0x0049_0000;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x1220));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        Addr(p_4(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = 0x7f;
        c.g[V0][0] = sx(this.wrapping_add(0x18));
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        loop {
            c.cycles += 7;
            Addr(r(c, V0)).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
            let more = c.g[V1][0] != c.g[A1][0];
            c.g[V0][0] = sx(r(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_8528);
            }
        }
        c.cycles += 5;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        Addr(gp_3(c, 0xf50)).sw(c, c.g[S0][0]);
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.g[V1][0] = 0x7f;
        c.g[V0][0] = sx(this.wrapping_add(0x214));
        loop {
            c.cycles += 7;
            Addr(r(c, V0)).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(r(c, V0).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_8558);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rail manager.

/// `RailMan* RailMan_ctor_2(RailMan* self, Stream* stream, void* arg)`
/// (0x358588): init the sync process, clear the 128 rail handles, read the
/// handle table and disable every used rail.
pub fn railman_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0035_48c0, 0x0035_85ac)?;
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1220));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        Addr(p_5(r32(c, S2), 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = 0x7f;
        c.g[V0][0] = sx(p_5(r32(c, S2), 0x18));
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let more = c.g[V1][0] != c.g[A1][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_85d0);
            }
        }
        c.cycles += 13;
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A3][0] = sx(p_5(r32(c, S2), 0x18));
        Addr(gp_4(c, 0xf50)).sw(c, c.g[S2][0]);
        c.g[A1][0] = c.g[A3][0];
        c.g[A0][0] = Addr(p_5(r32(c, V0), 0x10)).lh(c);
        c.g[A2][0] = 0x200;
        c.g[V1][0] = Addr(p_5(r32(c, V0), 0x14)).lw(c);
        c.g[S3][0] = 0xffff_ffff_ffff_0000;
        c.g[A0][0] = sx(p_5(r32(c, S0), r32(c, A0)));
        let read = r32(c, V1);
        c.g[S1][0] = c.g[A3][0];
        call_ptr(c, read, 0x0035_8618)?;
        c.g[S0][0] = 0;
        c.g[S3][0] = c.g[S3][0] | 0xffff;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            let empty = c.g[V0][0] == c.g[S3][0];
            c.g[A0][0] = c.g[S2][0];
            if !empty {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0035_8780, 0x0035_8634)?;
            }
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 128i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx(p_5(r32(c, S1), 4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_8620);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimCombo_v52(cAnimCombo* self)`: update the underlying animation
/// object and, when the combo is playing, multiply every part's matrix with
/// the animation's transform.
pub fn anim_combo_v52_2(c: &mut Cpu) {
    guest(c, |c| {
        'b_359204: {
            c.cycles += 14;
            c.g[SP][0] = sub(c.g[SP][0], 0x70);
            c.sq(16, p_6(c.g[SP][0] as u32, 0x60));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, p_6(c.g[SP][0] as u32, 0x50));
            c.sq(18, p_6(c.g[SP][0] as u32, 0x40));
            c.sq(19, p_6(c.g[SP][0] as u32, 0x30));
            c.sq(20, p_6(c.g[SP][0] as u32, 0x20));
            c.sq(21, p_6(c.g[SP][0] as u32, 0x10));
            c.sd(p_6(c.g[SP][0] as u32, 0), c.g[RA][0]);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
            call_ptr(c, 0x0034_e348, 0x0035_916c)?;
            c.g[V0][0] = lw(c, p_6(c.g[S0][0] as u32, anim_combo::ANIM));
            if c.g[V0][0] == 0 {
                c.lq(16, p_6(c.g[SP][0] as u32, 0x60));
                break 'b_359204;
            }
            'b_3591a0: {
                c.cycles += 3;
                c.g[V0][0] = c.lb(p_6(c.g[S0][0] as u32, anim_combo::PART_A));
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = lw(c, p_6(c.g[S0][0] as u32, anim_combo::MODEL));
                    break 'b_3591a0;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(p_6(c.g[S0][0] as u32, anim_combo::PART_C));
                if c.g[V0][0] == 0 {
                    c.lq(16, p_6(c.g[SP][0] as u32, 0x60));
                    break 'b_359204;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(p_6(c.g[S0][0] as u32, anim_combo::PART_B));
                if c.g[V0][0] == 0 {
                    c.lq(16, p_6(c.g[SP][0] as u32, 0x60));
                    break 'b_359204;
                }
                c.cycles += 1;
                c.g[V1][0] = lw(c, p_6(c.g[S0][0] as u32, anim_combo::MODEL));
                break 'b_3591a0;
            }
            'b_359200: {
                c.cycles += 6;
                c.g[S5][0] = 0;
                c.g[S2][0] = 0;
                c.g[S3][0] = lw(c, p_6(c.g[V1][0] as u32, 0x80));
                c.g[V0][0] = lw(c, p_6(c.g[S3][0] as u32, 4));
                let none = (c.g[V0][0] as i64) <= 0;
                c.g[S1][0] = lw(c, p_6(c.g[S3][0] as u32, 8));
                if none {
                    break 'b_359200;
                }
                c.cycles += 2;
                c.g[S4][0] = 0;
                loop {
                    'b_3591ec: {
                        c.cycles += 4;
                        c.g[V0][0] = lw(c, p_6(c.g[S1][0] as u32, 8));
                        c.g[V1][0] = lw(c, p_6(c.g[V0][0] as u32, 0x1c));
                        let empty = c.g[V1][0] == 0;
                        c.g[V0][0] = sx((c.g[S5][0] as u32) << 6);
                        if empty {
                            break 'b_3591ec;
                        }
                        c.cycles += 7;
                        c.g[A0][0] = lw(c, p_6(c.g[S0][0] as u32, anim_combo::ANIM));
                        c.g[A1][0] = lw(c, p_6(c.g[S0][0] as u32, anim_combo::MATRICES));
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(1));
                        c.g[A0][0] = addu(c.g[A0][0], c.g[S4][0]);
                        c.g[A1][0] = addu(c.g[A1][0], c.g[V0][0]);
                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0xd0));
                        call_ptr(c, 0x0035_1948, 0x0035_91ec)?;
                        break 'b_3591ec;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = lw(c, p_6(c.g[S3][0] as u32, 4));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                    if !more {
                        break 'b_359200;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_91c0);
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, p_6(c.g[SP][0] as u32, 0x60));
            break 'b_359204;
        }
        c.cycles += 8;
        c.lq(17, p_6(c.g[SP][0] as u32, 0x50));
        c.lq(18, p_6(c.g[SP][0] as u32, 0x40));
        c.lq(19, p_6(c.g[SP][0] as u32, 0x30));
        c.lq(20, p_6(c.g[SP][0] as u32, 0x20));
        c.lq(21, p_6(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p_6(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}

/// `int func_003592D0(AnimDelta* s)` (0x3592d0): tick a particle/animation
/// delta: reset the two 1e10 guards, and when the object is not attached to a
/// live animation, count the delta down (clamped at 0), run the animation
/// object step once it reaches 0, and flag the parent; an attached object gets
/// its callback with the "0x30" payload. Returns 1 except when the callback
/// path ran.
pub fn anim_delta_v15(c: &mut Cpu) {
    /// What the tail of the function does.
    enum Tail {
        /// "+1; v0 = 1": report the object as alive.
        Alive,
        /// "+2; *(u16*)(s+0x42) |= 1; +1; v0 = 1".
        Flag,
        /// Return straight away (the callback path already set `v0`).
        Return,
    }
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.lwc1(0, gp_5(c, -0x2a6c)); // 1e10
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let s = c.g[S0][0] as u32;
        c.swc1(0, p_7(s, 0x28));
        c.g[V0][0] = c.lw(p_7(s, 8));
        let attached = c.g[V0][0] != 0;
        c.swc1(0, p_7(s, 0x24));
        let tail;
        if attached {
            tail = Tail::Alive;
        } else {
            c.cycles += 6;
            c.lwc1(1, p_7(s, 0x6c)); // v
            c.mtc1(0, 2); // f2 = 0.0
            c.c_lt_s(2, 1); // 0 < v
            let v_le_zero = !c.fcc();
            c.g[V0][0] = c.lw(p_7(s, 0xc)); // t
            if v_le_zero {
                // t > 0: count it down
                c.cycles += 2;
                let t_le_zero = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                if !t_le_zero {
                    c.cycles += 1;
                    c.sw(p_7(s, 0xc), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(p_7(s, 8));
                let no_parent = c.g[V0][0] == 0;
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                if no_parent {
                    c.cycles += 1;
                    c.g[V0][0] = c.lhu(p_7(c.g[S0][0] as u32, 0x42));
                    tail = Tail::Flag;
                } else {
                    // the slot-0x110 callback: the object is attached elsewhere
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(p_7(c.g[S0][0] as u32, 0x3c));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh(p_7(c.g[V1][0] as u32, 0x110));
                    c.g[V0][0] = c.lw(p_7(c.g[V1][0] as u32, 0x114));
                    let func = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0035_93a4);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, func, 0x0035_93a4)?;
                    c.g[V0][0] = 0;
                    tail = Tail::Return;
                }
            } else if (c.g[V0][0] as i64) < 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lhu(p_7(c.g[S0][0] as u32, 0x42));
                tail = Tail::Flag;
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    if !c.fcc() {
                        // t == 0 and v > 0: bleed the delta by |s+0x10|
                        // No 60/rate here: +0x10 is already a per-tick step
                        // (AnimObject msg 0x66 and the ctor divide the 30 fps
                        // value by the rate), so the budget drains in the same
                        // wall-clock time at every rate.
                        c.cycles += 7;
                        c.lwc1(0, p_7(c.g[S0][0] as u32, 0x10));
                        c.abs_s(0, 0);
                        c.sub_s(0, 1, 0);
                        c.c_lt_s(0, 2);
                        let negative = !c.fcc();
                        c.swc1(0, p_7(c.g[S0][0] as u32, 0x6c));
                        if !negative {
                            c.cycles += 1;
                            c.swc1(2, p_7(c.g[S0][0] as u32, 0x6c));
                        }
                    }
                }
                c.cycles += 8;
                c.g[RA][0] = sx(0x0035_934c);
                c.g[A0][0] = c.g[S0][0];
                c.pc = 0x0034_1d48;
                ee::s11::AnimObject_v15(c);
                if c.pc != 0x0035_934c {
                    return Err(Unwound);
                }
                c.lwc1(1, p_7(c.g[S0][0] as u32, 0x6c));
                c.mtc1(0, 0); // f0 = 0.0
                c.c_lt_s(0, 1); // 0 < v
                let stopped = !c.fcc(); // v <= 0 after the step
                c.g[V0][0] = 0x1;
                if stopped {
                    c.cycles += 2;
                    c.sw(p_7(c.g[S0][0] as u32, 4), 0);
                    tail = Tail::Alive;
                } else {
                    c.cycles += 2;
                    c.sw(p_7(c.g[S0][0] as u32, 4), c.g[V0][0]);
                    tail = Tail::Return;
                }
            }
        }
        match tail {
            Tail::Alive => {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
            }
            Tail::Flag => {
                c.cycles += 2;
                c.g[V0][0] = c.g[V0][0] | 0x1;
                c.sh(p_7(c.g[S0][0] as u32, 0x42), c.g[V0][0]);
                c.cycles += 1;
                c.g[V0][0] = 0x1;
            }
            Tail::Return => {}
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00358998(void* railman, int idx)` (0x358998): enable rail
/// `idx`: fetch its part from the world, disable each collider node's box
/// (inserted into the spatial hash), then mark the reference unused
/// (`-1`).
pub fn rail_man_enable_rail(c: &mut Cpu) {
    let frame = Frame::open(c, 0xc0);
    'b_358a24: {
        'b_358a00: {
            c.cycles += 24;
            frame.save(c, S1, 0x90);
            frame.save(c, S6, 0x40);
            c.g[S1][0] = c.g[A1][0];
            c.g[S6][0] = c.g[A0][0];
            frame.save(c, S2, 0x80);
            frame.save(c, S3, 0x70);
            c.g[V0][0] = sx(r32(c, S1) << 2);
            frame.save(c, S4, 0x60);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S6)));
            frame.save(c, S5, 0x50);
            frame.save_ra(c, 0x30);
            c.swc1(20, at_2(c, SP, 0xb0));
            frame.save(c, S0, 0xa0);
            c.g[RA][0] = sx(0x003589d8);
            c.g[S0][0] = lw_3(c, at_2(c, V0, 0x18));
            if call_ptr(c, 0x002d1bd8, 0x003589d8).is_err() {
                return;
            }
            c.g[A0][0] = lw_3(c, r32(c, V0));
            c.g[V0][0] = c.g[S0][0] & 0xff;
            c.g[V1][0] = lw_3(c, at_2(c, A0, 0x8));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[V1][0] = lw_3(c, r32(c, V0));
            if c.g[V1][0] != 0u64 {
                c.g[A0][0] = lw_3(c, at_2(c, V1, 0x44));
                break 'b_358a00;
            } else {
                c.cycles += 2;
                let t = true;
                c.g[S4][0] = 0u64;
                if t {
                    break 'b_358a24;
                } else {
                    break 'b_358a00;
                }
            }
        }
        'b_358a20: {
            c.cycles += 7;
            c.g[V0][0] = sx(r32(c, S0) >> 8);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = lw_3(c, r32(c, V0));
            c.g[V0][0] = sx(r32(c, V1) >> 8);
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            if t {
                break 'b_358a20;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_358a20;
            }
        }
        c.cycles += 1;
        c.g[S4][0] = c.g[V0][0];
        break 'b_358a24;
    }
    'b_358ae8: {
        c.cycles += 5;
        c.g[S5][0] = sx(r32(c, S1) << 2);
        c.g[V0][0] = lw_3(c, at_2(c, S4, 0x20));
        c.g[S1][0] = lw_3(c, at_2(c, S4, 0x24));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S2][0] = 0u64;
        if t {
            break 'b_358ae8;
        } else {
            c.cycles += 4;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 20);
            c.g[S3][0] = 0xffffffffffffffff;
            'l_358a48: loop {
                c.cycles += 40;
                c.lwc1(1, at_2(c, S1, 0x6c));
                c.g[S2][0] = sx(at_2(c, S2, 0x1));
                c.swc1(1, r32(c, SP));
                c.lwc1(0, at_2(c, S1, 0x70));
                c.swc1(0, at_2(c, SP, 0x4));
                c.lwc1(1, at_2(c, S1, 0x74));
                c.swc1(20, at_2(c, SP, 0xc));
                c.swc1(1, at_2(c, SP, 0x8));
                c.lwc1(0, at_2(c, S1, 0x78));
                c.swc1(0, at_2(c, SP, 0x10));
                c.lwc1(1, at_2(c, S1, 0x7c));
                c.swc1(1, at_2(c, SP, 0x14));
                c.lwc1(0, at_2(c, S1, 0x80));
                c.swc1(20, at_2(c, SP, 0x1c));
                c.g[RA][0] = sx(0x00358a88);
                c.swc1(0, at_2(c, SP, 0x18));
                if call_ptr(c, 0x002d1be0, 0x00358a88).is_err() {
                    return;
                }
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx(at_2(c, SP, 0x20));
                c.g[RA][0] = sx(0x00358a98);
                c.g[A1][0] = c.g[SP][0];
                if call_ptr(c, 0x00328f28, 0x00358a98).is_err() {
                    return;
                }
                c.g[V0][0] = lw_3(c, at_2(c, SP, 0x24));
                c.g[V1][0] = sx(at_2(c, S0, 0x50));
                c.g[A1][0] = lw_3(c, at_2(c, SP, 0x28));
                c.g[A2][0] = c.g[S1][0];
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[T0][0] = lw_3(c, at_2(c, SP, 0x2c));
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.g[S0][0];
                }
                c.g[A1][0] = ((c.g[S3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[A0][0] = sx(at_2(c, V1, 0x28));
                c.g[T0][0] = ((c.g[S3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                if c.g[A1][0] != 0 {
                    c.g[A0][0] = c.g[V1][0];
                }
                c.g[A3][0] = sx(at_2(c, SP, 0x20));
                c.g[V0][0] = sx(at_2(c, A0, 0x14));
                c.g[A1][0] = 0x2;
                c.g[RA][0] = sx(0x00358ad8);
                if c.g[T0][0] == 0 {
                    c.g[A0][0] = c.g[V0][0];
                }
                if call_ptr(c, 0x00328c20, 0x00358ad8).is_err() {
                    return;
                }
                c.g[V0][0] = lw_3(c, at_2(c, S4, 0x20));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S1][0] = lw_3(c, at_2(c, S1, 0x64));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00358a48);
                    }
                    continue 'l_358a48;
                } else {
                    break 'b_358ae8;
                }
            }
        }
    }
    c.cycles += 15;
    c.g[V1][0] = sx(r32(c, S5).wrapping_add(r32(c, S6)));
    c.g[V0][0] = 0xffffffffffff0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    c.g[RA][0] = ld(c, at_2(c, SP, 0x30));
    c.lwc1(20, at_2(c, SP, 0xb0));
    sw(c, at_2(c, V1, 0x18), c.g[V0][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_2(c, SP, 0xc0));
    c.pc = t;
    return;
}

// -------------------------------------------- MultiParticle_EmitAtInstances

/// `void MultiParticle_EmitAtInstances(MultiParticle* self)` (0x358260):
/// for each of the `count` world-instance handles in `refs`, resolve the
/// handle through the world's handle table (low byte: table index, upper
/// bits: slot); for a live instance, offset the emit box (+0x20 min, +0x30
/// max) by the instance position (+0x40) and render the emitter descriptor
/// (+0x10) there, at instance position + `center` (+0x40).
pub fn multi_particle_emit_at_instances(c: &mut Cpu) {
    /// `MultiParticle` fields.
    const COUNT: u32 = 0x4;
    const REFS: u32 = 0xc;
    const DESC: u32 = 0x10;
    const BOX_MIN: u32 = 0x20;
    const BOX_MAX: u32 = 0x30;
    const CENTER: u32 = 0x40;
    /// Instance position.
    const INSTANCE_POS: u32 = 0x40;
    /// `World* World_Get()` (`*(gp+0x16c8)`).
    const WORLD_GET: u32 = 0x002d_1bd8;
    /// `Emitter_RenderAt(desc, box, pos)`.
    const EMITTER_RENDER_AT: u32 = 0x0037_0888;
    /// The loop head (event checks).
    const LOOP: u32 = 0x0035_8280;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x50);
        c.g[V0][0] = Addr(this).off(COUNT).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            loop {
                c.cycles += 14;
                call_ptr(c, WORLD_GET, 0x0035_8288)?;
                let world = r32(c, V0);
                let i = r32(c, S1);
                // Resolve the handle.
                let refs = Addr(this).off(REFS).lw(c);
                c.g[A0][0] = refs;
                let tables = Addr(world).lw(c);
                c.g[A1][0] = tables;
                let handle = Addr((i << 2).wrapping_add(refs as u32)).lw(c);
                c.g[V1][0] = handle;
                let table = Addr(tables as u32).off(8).lw(c);
                let entry_at = ((handle as u32 & 0xff) << 2).wrapping_add(table as u32);
                let entry = Addr(entry_at).lw(c);
                c.g[A0][0] = entry;
                let slot = handle as u32 >> 8;
                c.g[V0][0] = sx(slot);
                let instance = if entry == 0 {
                    c.cycles += 2;
                    0
                } else {
                    c.cycles += 7;
                    let slots = Addr(entry as u32).off(0x1c).lw(c);
                    c.g[A0][0] = slots;
                    let word = Addr((slot << 2).wrapping_add(slots as u32)).lw(c);
                    c.g[V1][0] = word;
                    let index = word as u32 >> 8;
                    let mut p = sx(index << 2);
                    if index == 0 {
                        c.cycles += 1;
                        p = 0;
                    }
                    c.g[V0][0] = p;
                    c.cycles += 1;
                    p
                };
                c.g[A0][0] = instance;

                c.cycles += 2;
                c.g[A1][0] = c.g[SP][0];
                if instance != 0 {
                    c.cycles += 27;
                    let inst = instance as u32;
                    let at = |c: &Cpu, off: u32| sp(c).wrapping_add(off);
                    // box = self->box + pos (both corners).
                    c.lq(V0, this.wrapping_add(BOX_MIN));
                    c.lq(V1, this.wrapping_add(BOX_MAX));
                    c.g[A2][0] = sx(at(c, 0x20));
                    c.sq(V0, at(c, 0));
                    c.sq(V1, at(c, 0x10));
                    c.lq(V0, inst.wrapping_add(INSTANCE_POS));
                    c.sq(V0, at(c, 0x20));
                    c.lqc2(3, at(c, 0));
                    c.lqc2(4, at(c, 0x20));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, 0));
                    c.lq(V0, inst.wrapping_add(INSTANCE_POS));
                    c.sq(V0, at(c, 0x20));
                    c.lqc2(3, at(c, 0x10));
                    c.lqc2(4, at(c, 0x20));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, 0x10));
                    // at = pos + center.
                    c.lq(V0, inst.wrapping_add(INSTANCE_POS));
                    c.sq(V0, at(c, 0x30));
                    c.lqc2(3, at(c, 0x30));
                    c.lqc2(4, this.wrapping_add(CENTER));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, 0x40));
                    c.lq(V0, at(c, 0x40));
                    c.g[A0][0] = Addr(this).off(DESC).lw(c);
                    c.sq(V0, at(c, 0x20));
                    call_ptr(c, EMITTER_RENDER_AT, 0x0035_8358)?;
                }

                c.cycles += 5;
                let count = Addr(this).off(COUNT).lw(c);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let more = (c.g[S1][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOOP);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    });
}

/// `SyncProcess::~SyncProcess()` (0x354920): installs the `SyncProcess`
/// vtable (0x491e00 at +0xc) and runs the `TypedProcess` destructor (which
/// returns `this`, or frees it when the delete flag in a1 is set).
pub fn sync_process_dtor(c: &mut Cpu) {
    const SYNC_PROCESS_VTABLE: u32 = 0x0049_1e00;
    const PROCESS_VTABLE: u32 = 0xc;
    const TYPED_PROCESS_DTOR: Callee = Callee {
        addr: 0x0035_4808,
        f: ee::s11::TypedProcess_dtor,
    };
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(SYNC_PROCESS_VTABLE);
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(PROCESS_VTABLE),
            sx(SYNC_PROCESS_VTABLE),
        );
        TYPED_PROCESS_DTOR.call(c, 0x0035_493c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const MP_TEMPLATE: u32 = 0x8;

pub(crate) const MP_ITEMS: u32 = 0xc;

pub(crate) const MP_BOUNDS: u32 = 0x20;

pub(crate) const MP_ORIGIN: u32 = 0x40;

/// `Emitter_Save(emitter, stream)`.
pub(crate) const MP_EMITTER_SAVE: u32 = 0x0037_0aa8;

/// `stream->Write(a1, size)` with the stream in s0, as the original issues
/// it: v1 = vtable and v0 = function, or (`swapped`) the other way round.
pub(crate) fn multi_particle_write(
    c: &mut Cpu,
    data: u64,
    size: u64,
    swapped: bool,
    ret: u32,
) -> Flow {
    let stream = reg(c, S0);
    set_reg(c, A1, data);
    set_reg(c, A2, size);
    let vt = Addr(stream as u32).lw(c);
    let write = VSlot::read(c, vt, MP_STREAM_WRITE);
    if swapped {
        set_reg(c, V0, vt);
        set_reg(c, V1, write.func);
    } else {
        set_reg(c, V1, vt);
        set_reg(c, V0, write.func);
    }
    set_reg(c, A0, write.this(stream));
    call_ptr(c, write.func as u32, ret)
}

/// `void MultiParticle_Save(MultiParticle* self, Stream* stream)`
/// (0x358380): writes the object's first word, its item count, the shared
/// 0xd8-byte particle template, its emitter (`Emitter_Save`), the origin
/// (0x10 bytes) and bounds (0x20 bytes), then the item array (count words)
/// if it has any.
pub fn multi_particle_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0x30);
        set_reg(c, A2, 4);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let stream = reg(c, A1);
        set_reg(c, S0, stream);
        frame.save_ra(c, 0);
        let this = reg(c, A0);
        set_reg(c, S1, this);

        multi_particle_write(c, this, 4, false, 0x0035_83b4)?;
        let this = reg(c, S1);
        multi_particle_write(c, addu(this, MP_COUNT as u64), 4, false, 0x0035_83d0)?;
        let template = Addr(r32(c, S1)).off(MP_TEMPLATE).lw(c);
        multi_particle_write(c, template, 0xd8, false, 0x0035_83ec)?;
        let emitter = Addr(r32(c, S1)).off(MP_EMITTER).lw(c);
        set_reg(c, A0, emitter);
        let stream = reg(c, S0);
        set_reg(c, A1, stream);
        call_ptr(c, MP_EMITTER_SAVE, 0x0035_83f8)?;
        let this = reg(c, S1);
        multi_particle_write(c, addu(this, MP_ORIGIN as u64), 0x10, false, 0x0035_8414)?;
        let this = reg(c, S1);
        multi_particle_write(c, addu(this, MP_BOUNDS as u64), 0x20, false, 0x0035_8430)?;

        let count = Addr(r32(c, S1)).off(MP_COUNT).lw(c);
        set_reg(c, A2, sx((count as u32) << 2));
        if (count as i64) > 0 {
            c.cycles += 6;
            let items = Addr(r32(c, S1)).off(MP_ITEMS).lw(c);
            let size = reg(c, A2);
            multi_particle_write(c, items, size, true, 0x0035_8454)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SplineModifier_ctor (0x359460)
// ---------------------------------------------------------------------------

/// The `cSplineModifier` vtable.
pub(crate) const SPLINE_MODIFIER_CTOR_VTABLE: u32 = 0x48_f250;

/// `D_004FF120`: the quad written to both `sMpQuad` slots (`+0xb0`, `+0xc0`).
#[cfg(test)]
pub(crate) const SPLINE_MODIFIER_CTOR_QUAD: u32 = 0x004f_f120;

/// The `tAddSplineData` fields the constructor reads.
pub(crate) const SPLINE_MODIFIER_CTOR_D_PATH: u32 = 0x4;

pub(crate) const SPLINE_MODIFIER_CTOR_D_A: u32 = 0x8;

pub(crate) const SPLINE_MODIFIER_CTOR_D_B: u32 = 0xc;

pub(crate) const SPLINE_MODIFIER_CTOR_D_SPEED: u32 = 0x10;

pub(crate) const SPLINE_MODIFIER_CTOR_D_ANGLE: u32 = 0x14;

pub(crate) const SPLINE_MODIFIER_CTOR_D_OVERRIDE: u32 = 0x18;

pub(crate) const SPLINE_MODIFIER_CTOR_D_JITTER: u32 = 0x1c;

pub(crate) const SPLINE_MODIFIER_CTOR_D_FLAG: u32 = 0x20;

pub(crate) const SPLINE_MODIFIER_CTOR_D_SCALE: u32 = 0x24;

pub(crate) const SPLINE_MODIFIER_CTOR_D_END: u32 = 0x28;

/// The `.lit4` constants: 0.0174533f (degrees to radians) at `gp-0x2a64` and
/// 0.1f at `gp-0x2a60`.
#[cfg(test)]
pub(crate) const SPLINE_MODIFIER_CTOR_DEG2RAD: u32 = 0x3c8e_fa36;

#[cfg(test)]
pub(crate) const SPLINE_MODIFIER_CTOR_TENTH: u32 = 0x3dcc_cccd;

/// `cSplineModifier* cSplineModifier_ctor(self, const tAddSplineData& sdata)`
/// (0x359460): build a spline modifier from a descriptor: the path and speed,
/// the per-frame angle in radians and the per-second speed scale, the initial
/// distance (0, or the descriptor's override plus a random jitter, clamped
/// into [0, path length]), the flags and the two default `sMpQuad`s.
pub fn spline_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, 16, 0x20);
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(SPLINE_MODIFIER_CTOR_VTABLE);
        frame.save_fpr(c, 20, 0x30);
        c.g[V1][0] |= 0xffff;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[S1][0] = c.g[A1][0]; // the descriptor
        c.sw((c.g[S0][0] as u32).wrapping_add(0xd8), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xe0), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xdc), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xe4), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0);
        c.lwc1(1, gp_oc322(c, 0xffff_d59c)); // -0x2a64: degrees -> radians
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_A));
        c.g[AT][0] = 0x42c8_0000; // 100.0f
        c.mtc1(1, 2);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_B));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[V0][0]);
        c.lwc1(
            0,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_ANGLE),
        );
        c.mul_s(0, 0, 1);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x38));
        c.lwc1(
            1,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_SCALE),
        );
        c.mul_s(1, 1, 2);
        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x40));
        c.lwc1(
            0,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_END),
        );
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x44));
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_PATH));
        Callee {
            addr: 0x0034_51c0,
            f: ee::s11::SplineModifier_InitPath,
        }
        .call(c, 0x0035_94ec)?;
        c.lwc1(
            12,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_SPEED),
        );
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0035_9688,
            f: ee::s11::SplineModifier_SetSpeed,
        }
        .call(c, 0x0035_94f8)?;
        // The initial distance: 0 with a forward speed, else the path length.
        c.mtc1(0, 1);
        c.lwc1(
            0,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_SPEED),
        );
        c.c_le_s(1, 0);
        if c.fcc() {
            c.cycles += 2;
            c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x3c));
        } else {
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xe4));
            c.cycles += 1;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x3c));
        }
        c.cycles += 10;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_FLAG));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), 0);
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        c.mtc1(0, 20); // f20 = 0.0f
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
        c.lwc1(
            0,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_OVERRIDE),
        );
        c.c_eq_s(0, 20);
        if !c.fcc() {
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x3c));
        }
        // `dist += rand(-1, 1) * jitter`, clamped into [0, path length].
        c.cycles += 14;
        c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
        c.mtc1(1, 12);
        c.g[AT][0] = 0x3f80_0000; // 1.0f
        c.mtc1(1, 13);
        call_ptr(c, 0x0031_7830, 0x0035_955c)?;
        c.lwc1(
            1,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_JITTER),
        );
        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x3c));
        c.mul_s(0, 0, 1);
        c.add_s(2, 2, 0);
        c.c_lt_s(2, 20);
        let mut clamp = c.fcc();
        c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x3c));
        if !clamp {
            c.cycles += 5;
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xe4));
            c.c_lt_s(0, 2);
            let over = !c.fcc();
            c.g[A1][0] = 0x0050_0000;
            clamp = !over;
        }
        if clamp {
            c.cycles += 2;
            c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x3c));
            c.g[A1][0] = 0x0050_0000;
        }
        c.cycles += 16;
        c.g[A2][0] = 1;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xee0)); // &D_004FF120
        c.g[V0][0] = c.g[S0][0];
        c.lq(3, c.g[A1][0] as u32);
        frame.restore(c, 17, 0x10);
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0xb0));
        frame.restore_ra(c, 0);
        c.lq(4, c.g[A1][0] as u32);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A2][0]);
        c.sq(4, (c.g[S0][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xd0), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xd4), 0);
        frame.restore(c, 16, 0x20);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void MultiParticle_CreateEmitter(MultiParticle* self)` (0x358020): allocate the
/// index table (`count` words, all -1) at `+0xc`, the 0x190-byte emitter block and
/// its object (`+0x10`), clear the table again through the new object and register
/// the emitter with the particle system.
#[allow(unreachable_code)]
pub fn multi_particle_create_emitter(c: &mut Cpu) {
    guest(c, |c| {
        'b_35808c: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[A1][0] = 0x4a0000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4030));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A2][0] = 0x20000000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A3][0] = 0u64;
            c.g[S0][0] = c.lw(c.g[S1][0] as u32);
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
            call_ptr(c, 0x00317e30, 0x00358058)?;
            c.g[A0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_35808c;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0xffffffffffff0000;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                'l_358070: loop {
                    c.cycles += 7;
                    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V1][0] != c.g[A2][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00358070);
                        }
                        continue 'l_358070;
                    } else {
                        break 'b_35808c;
                    }
                }
            }
        }
        'b_3580f0: {
            c.cycles += 13;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = 0x190;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4038));
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0u64;
            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
            call_ptr(c, 0x00317d70, 0x003580a8)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x00370018, 0x003580b0)?;
            c.g[V1][0] = c.lw(c.g[S1][0] as u32);
            c.g[A0][0] = 0u64;
            let t = (c.g[V1][0] as i64) <= 0;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            if t {
                break 'b_3580f0;
            } else {
                c.cycles += 4;
                c.g[A1][0] = 0xffffffffffff0000;
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                'l_3580d0: loop {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[A1][0]);
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        if event_due(c) {
                            deliver_event_here(c, 0x003580d0);
                        }
                        continue 'l_3580d0;
                    } else {
                        break 'b_3580f0;
                    }
                }
            }
        }
        c.cycles += 11;
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x500000;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe60));
        call_ptr(c, 0x003705e0, 0x00358104)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `BoostModifier* BoostModifier_BaseCtor_2(BoostModifier* self, Obj* o)`
/// (0x358468): installs the base vtable (0x4913b8) at `self+4`, then calls
/// the source object's hook (receiver adjusted by the halfword at its
/// vtable +0x10) with `self` as the argument; returns `self`.
pub fn boost_modifier_base_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x13b8));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let vtable = c.g[V0][0];
        Addr(r32(c, S0).wrapping_add(0x4)).sw(c, vtable);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0x4;
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x10)).lh(c);
        c.g[A3][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let target = r32(c, A3);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        call_ptr(c, target, 0x0035_84a4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `AnimDelta* AnimDelta_ctor_2(AnimDelta* self, int, AnimCombo* combo?)`
/// (0x359270): builds the `AnimObject` base (0x341c80), installs the
/// `AnimDelta` vtable (0x48ea70) at `self+0x3c` and calls the second
/// argument's hook (receiver adjusted by the halfword at its vtable +0x10)
/// with `self+0x6c` as the argument; returns `self`.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn anim_delta_ctor_2_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0034_1c80, 0x0035_928c)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0x6c));
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x1590));
        c.g[A2][0] = 4;
        let vtable = c.g[V1][0];
        Addr(r32(c, S0).wrapping_add(0x3c)).sw(c, vtable);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x10)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_92b4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cRailManNode::ActiveNode`: the global rail manager pointer, at gp+0xf50.
pub(crate) const RAIL_ACTIVE_NODE_GP: u32 = 0xf50;

/// `cRailManNode::RailInfo[128]`: one 4-byte RailRID per slot, from +0x18.
pub(crate) const RAIL_INFO: u32 = 0x18;

/// Slots in `RailInfo`.
pub(crate) const RAIL_SLOTS: u64 = 128;

/// `void cRailManNode::addRail(tResourceID key, int add)` (0x358b28, the
/// reference's `cRailManNode_addRail`). The manager is created on first use
/// (`cMemMan_alloc` of 0x218 bytes, then its constructor). The slot whose
/// RailRID is `key`, or else the first free slot (RailRID -1), gets `key`
/// and is enabled when `add != 0`, disabled otherwise. Nothing happens when
/// no slot is found, or when the slot already holds the same state.
pub fn rail_man_set_rail_enabled(c: &mut Cpu) {
    guest(c, |c| {
        // S0 = add, S1 = key (callee-saved, kept in the registers).
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        let active = Addr((c.g[GP][0] as u32).wrapping_add(RAIL_ACTIVE_NODE_GP)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        if active == 0 {
            // cRailManNode_ActiveNode = new(0x218) cRailManNode(1)
            c.cycles += 9;
            c.g[A1][0] = 0x004a_4048; // "RailMan" name
            c.g[A0][0] = 0x218;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0035_8b60)?; // cMemMan_alloc
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            Callee {
                addr: 0x0035_84f0,
                f: ee::s11::f_003584f0,
            }
            .call(c, 0x0035_8b6c)?; // cRailManNode_cRailManNode1
        }

        // Scan for the key (and the first free slot) in RailInfo.
        c.cycles += 9;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(RAIL_ACTIVE_NODE_GP)).lw(c);
        c.g[A3][0] = u64::MAX; // 0xffff_ffff_ffff_0000 | 0xffff: all ones
        c.g[A1][0] = u64::MAX; // idx = -1
        c.g[A2][0] = 0; // i
        c.g[T1][0] = u64::MAX; // -1
        c.g[T0][0] = c.g[S1][0]; // key
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(RAIL_INFO)); // &RailInfo[i]
        'scan: loop {
            c.cycles += 2;
            let rid = Addr(c.g[V1][0] as u32).lw(c);
            c.g[V0][0] = rid;
            if c.g[A1][0] == c.g[T1][0] {
                // no slot yet: a free slot (RailRID -1) is the candidate
                c.cycles += 3;
                if (rid ^ c.g[A3][0]) == 0 {
                    c.g[A1][0] = c.g[A2][0];
                }
            }
            c.cycles += 2;
            if rid == c.g[T0][0] {
                c.g[A1][0] = c.g[A2][0];
                break 'scan;
            }
            c.cycles += 4;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A2][0] as i64) < RAIL_SLOTS as i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            if !more {
                break 'scan;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_8b90);
            }
        }

        'c1c: {
            'c18: {
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                let no_slot = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                if no_slot {
                    break 'c18;
                }
                'bf8: {
                    'bec: {
                        c.cycles += 5;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(RAIL_INFO).lw(c);
                        if c.g[V1][0] != c.g[S1][0] {
                            break 'bec;
                        }
                        // the slot already holds the key
                        c.cycles += 2;
                        let add_zero = c.g[S0][0] == 0;
                        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        if add_zero {
                            break 'c18;
                        }
                        c.cycles += 2;
                        break 'bf8;
                    }
                    // another key is in the slot
                    c.cycles += 2;
                    if c.g[S0][0] != 0 {
                        frame.restore(c, S0, 0x20);
                        break 'c1c;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                }
                // store the key, then enable (add != 0) or disable
                c.cycles += 2;
                let add_zero = c.g[S0][0] == 0;
                let key = c.g[S1][0];
                Addr(c.g[V0][0] as u32).off(RAIL_INFO).sw(c, key);
                if !add_zero {
                    c.cycles += 4;
                    Callee {
                        addr: 0x0035_8998,
                        f: ee::s11::f_00358998,
                    }
                    .call(c, 0x0035_8c08)?; // cRailManNode_enableRail
                    frame.restore(c, S0, 0x20);
                    break 'c1c;
                }
                c.cycles += 2;
                Callee {
                    addr: 0x0035_8780,
                    f: ee::s11::f_00358780,
                }
                .call(c, 0x0035_8c18)?; // cRailManNode_disableRail
            }
            // leaving through 'c18 (not through the 'c1c breaks above)
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }

        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cAnimComboNode_cAnimComboNode1(self, bucket, pInstance, adata)` (0x358c80,
/// `AnimCombo_ctor`): the animation object base (type 3), then the combo's
/// vtable, its rate and life times (the frame times from `adata`, turned into
/// seconds at 30 Hz), the speed per second over the timer rate, the stop flags
/// and the transform array (`NumAnimMeshes` elements of 64 bytes).
pub fn anim_combo_ctor(c: &mut Cpu) {
    guest(c, |c| {
        // S0 = pInstance, S1 = self, S2 = adata
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A3][0];
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
        frame.save(c, S3, 0x10);
        c.g[A2][0] = 3;
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S2][0];
        Callee {
            addr: 0x0034_1aa0,
            f: ee::s11::f_00341aa0,
        }
        .call(c, 0x0035_8cb8)?; // AnimObject_ctor(self+0x18, bucket, 3, pInstance, adata)
        c.g[V0][0] = 0x490000;
        c.mtc1(0, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x13d8));
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8)); // &self->FromTime2 (rate)
        let vtable = c.g[V0][0];
        Addr((c.g[S1][0] as u32).wrapping_add(0x54)).sw(c, vtable); // mVtbl

        // rate = (adata->FromTime2 < 0 ? adata->ToTime : adata->FromTime2) / 30
        c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x28));
        c.c_lt_s(1, 0);
        let from_neg = c.fcc();
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x80)).lw(c); // pInstance->m_Model
        if from_neg {
            c.cycles += 1;
            c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x10));
        }
        c.cycles += 9;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2a7c));
        c.mul_s(1, 1, 0);
        c.swc1(1, c.g[V1][0] as u32);

        // life = (adata->ToTime2 < 0 ? pModel->m_fAnimTime : adata->ToTime2 / 30)
        c.mtc1(0, 0);
        c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x2c));
        c.c_lt_s(1, 0);
        let life_neg = c.fcc();
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        if life_neg {
            c.cycles += 2;
            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x14));
        } else {
            c.cycles += 2;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2a78));
            c.mul_s(0, 1, 0);
        }

        // AnimSpeed2, Stop, StopInPlace, and the transform array
        c.cycles += 25;
        c.swc1(0, c.g[V0][0] as u32);
        c.g[A1][0] = 0x490000;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c); // cAppMan_TheAppMan
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x15e0));
        c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x30));
        c.g[A2][0] = 0x2000_0000;
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2a74));
        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x10)); // TimerHz (an int)
        c.cvt_s_w(2, 2);
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x74)).lw(c); // NumAnimMeshes
        c.g[A3][0] = 0;
        c.mul_s(0, 0, 1);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x84)); // &self->ComboXform
        c.g[A0][0] = sx((c.g[S0][0] as u32) << 6);
        c.div_s(0, 0, 2);
        c.swc1(0, c.g[S1][0] as u32);
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x34)).lw(c); // adata->Stop
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        c.sb((c.g[S1][0] as u32).wrapping_add(0x16), c.g[V0][0]);
        c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x34)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) >> 31);
        c.sb((c.g[S1][0] as u32).wrapping_add(0x17), c.g[V1][0]);
        call_ptr(c, 0x0031_7e30, 0x0035_8d74)?; // operator_new_named(n << 6, ...)
        let none = c.g[S0][0] == 0;
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
        if !none {
            c.cycles += 1;
            c.g[A0][0] = u64::MAX;
        }

        c.cycles += 12;
        let combo = c.g[S3][0] as u32;
        let xform = c.g[V0][0];
        Addr(combo).sw(c, xform); // self->ComboXform = new[n]
        c.g[V0][0] = c.g[S1][0];
        c.sb((c.g[S1][0] as u32).wrapping_add(0x14), 0);
        c.sb((c.g[S1][0] as u32).wrapping_add(0x15), 0);
        Addr((c.g[S1][0] as u32).wrapping_add(0x4)).sw(c, 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cAnimComboNode` fields the clock reads and writes (offsets from the
/// generated body; the names are the reference's).
pub(crate) const ANIM_COMBO_V15_SPEED: u32 = 0x00;

// AnimSpeed2 (float)
pub(crate) const ANIM_COMBO_V15_CURR2: u32 = 0x04;

// CurrTime2 (float)
pub(crate) const ANIM_COMBO_V15_TO2: u32 = 0x0c;

// ToTime2 (float): the clamp
pub(crate) const ANIM_COMBO_V15_ACTIVE2: u32 = 0x14;

// AnimActive2 (byte)
pub(crate) const ANIM_COMBO_V15_COMPLETE2: u32 = 0x15;

// AnimComplete2 (byte)
pub(crate) const ANIM_COMBO_V15_STOP: u32 = 0x16;

// Stop (byte)
pub(crate) const ANIM_COMBO_V15_ANIM_TYPE: u32 = 0x18;

// AnimType: the object the step runs on
pub(crate) const ANIM_COMBO_V15_DELAY: u32 = 0x24;

// Delay (int, frames to wait)
pub(crate) const ANIM_COMBO_V15_CURR: u32 = 0x34;

// CurrTime: the frame time
pub(crate) const ANIM_COMBO_V15_FRAME_END: u32 = 0x3c;

// FrameEnd (float)
pub(crate) const ANIM_COMBO_V15_FRAME_START: u32 = 0x40;

// FrameStart (float)
pub(crate) const ANIM_COMBO_V15_FLAGS: u32 = 0x5a;

// Flags (halfword)
/// gp-relative 1e10f: the "no frame" sentinel that FrameStart/FrameEnd get.
pub(crate) const ANIM_COMBO_V15_SENTINEL_GP: u32 = 0x2a70;

/// Return addresses of the two calls (the generated `c.g[RA] = ...` lines).
pub(crate) const ANIM_COMBO_V15_RET_STEP: u32 = 0x0035_8f64;

pub(crate) const ANIM_COMBO_V15_RET_FRAME: u32 = 0x0035_8f4c;

pub(crate) fn anim_combo_v15_fpr(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

/// The `Flags |= 1` tail: v0 holds the flags halfword; the function then
/// returns 1.
pub(crate) fn anim_combo_v15_mark_flags(c: &mut Cpu, s: u32) {
    c.cycles += 2;
    c.g[V0][0] = c.g[V0][0] | 0x1;
    let flags = c.g[V0][0];
    Addr(s + ANIM_COMBO_V15_FLAGS).sh(c, flags);
    c.cycles += 1;
    c.g[V0][0] = 0x1;
}

/// The clock, entered when `AnimComplete2` is 0. Steps the object (when
/// `AnimActive2` is 0), or counts `Delay` down and advances the clock once it
/// has run out.
pub(crate) fn anim_combo_v15_clock(c: &mut Cpu, s: u32) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(s + ANIM_COMBO_V15_ACTIVE2).lb(c);
    if c.g[V0][0] == 0 {
        // Not active: step the attached object once and return 1.
        c.cycles += 4;
        c.g[A0][0] = sx(s.wrapping_add(ANIM_COMBO_V15_ANIM_TYPE));
        Callee {
            addr: 0x0034_1d48,
            f: ee::s11::AnimObject_v15,
        }
        .call(c, ANIM_COMBO_V15_RET_STEP)?;
        c.g[V0][0] = 0x1;
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(s + ANIM_COMBO_V15_DELAY).lw(c);
    if (c.g[V0][0] as i64) < 0 {
        c.g[V0][0] = Addr(s + ANIM_COMBO_V15_FLAGS).lhu(c);
        anim_combo_v15_mark_flags(c, s);
        return Ok(());
    }
    c.cycles += 2;
    let expired = (c.g[V0][0] as i64) <= 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
    if !expired {
        c.cycles += 2;
        let delay = c.g[V0][0];
        Addr(s + ANIM_COMBO_V15_DELAY).sw(c, delay);
        c.cycles += 1;
        c.g[V0][0] = Addr(s + ANIM_COMBO_V15_FLAGS).lhu(c);
        anim_combo_v15_mark_flags(c, s);
        return Ok(());
    }
    anim_combo_v15_advance(c, s)
}

/// The countdown has run out: advance `CurrTime2` by `AnimSpeed2`, clamp it at
/// `ToTime2` (completing on `Stop`, and deactivating), then run the frame
/// update. A zero return from the update ends the function with 0.
pub(crate) fn anim_combo_v15_advance(c: &mut Cpu, s: u32) -> Flow {
    c.cycles += 10;
    let curr2 = Addr(s + ANIM_COMBO_V15_CURR2).read_fpu(c);
    set_fpr(c, 0, curr2);
    let speed = Addr(s + ANIM_COMBO_V15_SPEED).read_fpu(c);
    set_fpr(c, 1, speed);
    let to2 = Addr(s + ANIM_COMBO_V15_TO2).read_fpu(c);
    set_fpr(c, 3, to2);
    let pos = anim_combo_v15_fpr(c, 0) + anim_combo_v15_fpr(c, 1);
    set_fpr(c, 0, pos);
    let frame_time = Addr(s + ANIM_COMBO_V15_CURR).read_fpu(c);
    set_fpr(c, 2, frame_time);
    Addr(s + ANIM_COMBO_V15_FRAME_START).write_fpu(c, frame_time);
    let reached = anim_combo_v15_fpr(c, 3).le(anim_combo_v15_fpr(c, 0));
    set_fcc(c, reached);
    Addr(s + ANIM_COMBO_V15_FRAME_END).write_fpu(c, frame_time);
    Addr(s + ANIM_COMBO_V15_CURR2).write_fpu(c, pos);
    if reached {
        c.cycles += 3;
        c.g[V0][0] = Addr(s + ANIM_COMBO_V15_STOP).lb(c);
        let stop = c.g[V0][0] != 0;
        Addr(s + ANIM_COMBO_V15_CURR2).write_fpu(c, anim_combo_v15_fpr(c, 3));
        if stop {
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            let complete = c.g[V0][0];
            Addr(s + ANIM_COMBO_V15_COMPLETE2).sb(c, complete);
        }
        c.cycles += 1;
        Addr(s + ANIM_COMBO_V15_ACTIVE2).sb(c, 0);
    }
    c.cycles += 4;
    c.g[A0][0] = sx(s.wrapping_add(ANIM_COMBO_V15_CURR));
    Callee {
        addr: 0x0034_eba0,
        f: ee::s11::AnimObject_FireEndScript,
    }
    .call(c, ANIM_COMBO_V15_RET_FRAME)?;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(s + ANIM_COMBO_V15_FLAGS).lhu(c);
        anim_combo_v15_mark_flags(c, s);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    Ok(())
}

/// `int AnimCombo_v15(cAnimComboNode* self)` (0x358eb8): one step of a
/// combo's own clock. It first sets FrameStart and FrameEnd to the 1e10
/// sentinel. Unless `AnimComplete2` is set, it steps the object when the
/// combo is not active, and otherwise runs the clock: count `Delay` down,
/// then advance `CurrTime2` by `AnimSpeed2`, clamp at `ToTime2` (completing
/// on `Stop`), and run `cAnimNode_frameUpdate`. Returns 1, or 0 when the frame
/// update says the clock has finished.
pub fn anim_combo_v15(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        let sentinel =
            Addr((c.g[GP][0] as u32).wrapping_sub(ANIM_COMBO_V15_SENTINEL_GP)).read_fpu(c);
        set_fpr(c, 0, sentinel);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let s = c.g[S0][0] as u32;
        Addr(s + ANIM_COMBO_V15_FRAME_START).write_fpu(c, anim_combo_v15_fpr(c, 0));
        c.g[V0][0] = Addr(s + ANIM_COMBO_V15_COMPLETE2).lb(c);
        let complete = c.g[V0][0] != 0;
        Addr(s + ANIM_COMBO_V15_FRAME_END).write_fpu(c, anim_combo_v15_fpr(c, 0));
        if complete {
            c.cycles += 1;
            c.g[V0][0] = 0x1;
        } else {
            anim_combo_v15_clock(c, s)?;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RailMan_DisableRail` (0x00358780): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn rail_man_disable_rail(c: &mut Cpu) {
    if c.pc != 0x00358780 {
        return ee::s11::f_00358780__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00358780;
        loop {
            match pc {
                0x00358780 => {
                    c.cycles += 20;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[RA][0] = sx(0x003587b0);
                    c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x002d1bd8, 0x003587b0)?;
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = c.g[S0][0] & 0xff;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);

                    if c.g[V1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                        pc = 0x003587d8;
                        continue;
                    }
                    pc = 0x003587d0;
                    continue;
                }
                0x003587d0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S4][0] = 0u64;
                    if t {
                        pc = 0x003587fc;
                        continue;
                    }
                    pc = 0x003587d8;
                    continue;
                }
                0x003587d8 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) >> 8);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x003587f8;
                        continue;
                    }
                    pc = 0x003587f4;
                    continue;
                }
                0x003587f4 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x003587f8;
                    continue;
                }
                0x003587f8 => {
                    c.cycles += 1;
                    c.g[S4][0] = c.g[V0][0];
                    pc = 0x003587fc;
                    continue;
                }
                0x003587fc => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
                    c.g[S2][0] = 0u64;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x24));
                    if t {
                        pc = 0x00358970;
                        continue;
                    }
                    pc = 0x0035880c;
                    continue;
                }
                0x0035880c => {
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 20);
                    c.g[S3][0] = 0xffffffffffffffff;
                    pc = 0x00358818;
                    continue;
                }
                0x00358818 => {
                    c.cycles += 44;
                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x6c));
                    c.swc1(0, c.g[SP][0] as u32);
                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x70));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x74));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x78));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x7c));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x80));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1c));
                    c.g[RA][0] = sx(0x00358854);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x002d1be0, 0x00358854)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x00358870);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                    call_ptr(c, 0x00328f28, 0x00358870)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[A1][0] = 0x2;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A0][0] = ((c.g[S3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[T0][0] = ((c.g[S3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[A2][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    if c.g[T0][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x003588b8);
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x003284b8, 0x003588b8)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x003588c8;
                    continue;
                }
                0x003588c8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x003588d4;
                    continue;
                }
                0x003588d4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x003588e0;
                    continue;
                }
                0x003588e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x003588ec;
                    continue;
                }
                0x003588ec => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x003588f8;
                    continue;
                }
                0x003588f8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x00358904;
                    continue;
                }
                0x00358904 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x00358910;
                    continue;
                }
                0x00358910 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x00358924;
                        continue;
                    }
                    pc = 0x0035891c;
                    continue;
                }
                0x0035891c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0035894c;
                        continue;
                    }
                    pc = 0x00358924;
                    continue;
                }
                0x00358924 => {
                    c.cycles += 3;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = 0u64;
                    pc = 0x00358930;
                    continue;
                }
                0x00358930 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0035891c) {
                            return Ok(());
                        }
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x0035893c;
                    continue;
                }
                0x0035893c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x00358930) {
                            return Ok(());
                        }
                        pc = 0x00358930;
                        continue;
                    }
                    pc = 0x00358948;
                    continue;
                }
                0x00358948 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x0035894c;
                    continue;
                }
                0x0035894c => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
                        pc = 0x00358964;
                        continue;
                    }
                    pc = 0x00358954;
                    continue;
                }
                0x00358954 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0035895c);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317e50, 0x0035895c)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
                    pc = 0x00358964;
                    continue;
                }
                0x00358964 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                    if t {
                        if back_edge_unwinds(c, 0x00358818) {
                            return Ok(());
                        }
                        pc = 0x00358818;
                        continue;
                    }
                    pc = 0x00358970;
                    continue;
                }
                0x00358970 => {
                    c.cycles += 9;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00358780, 0x00358994);
                    return Ok(());
                }
            }
        }
    });
}

pub(crate) const ANIMD_OBJECT_CTOR: Callee = Callee {
    addr: 0x0034_1c80,
    f: ee::s11::f_00341c80,
};

pub(crate) const ANIMD_OBJECT_V15: Callee = Callee {
    addr: 0x0034_1d48,
    f: ee::s11::f_00341d48,
};

/// `cAnimDeltaNode *cAnimDeltaNode_cAnimDeltaNode(self, int bucket, Stream *stream)`
/// (0x359270): the anim object base, the delta-node vtable, then the float
/// delta (+0x6c) read from the stream. Frame 0x30. Returns self.
pub fn anim_delta_ctor_2_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        ANIMD_OBJECT_CTOR.call(c, 0x0035_928c)?;

        let this = c.g[S0][0] as u32;
        let stream = c.g[S1][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = sx(this.wrapping_add(0x6c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1590));
        c.g[A2][0] = 4;
        let vtable = c.g[V1][0];
        Addr(this + 0x3c).sw(c, vtable);
        oc664_stream_read(c, stream, V0, V1, 0x0035_92b4)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0x10);
        frame.restore(c, 16, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Flags path of the update: `flags |= 1`, then return 1 (the tail's `+1`).
pub(crate) fn oc664_set_flag_return(c: &mut Cpu, this: u32) -> u64 {
    c.cycles += 2;
    let flags = Addr(this + 0x42).lhu(c) | 1;
    Addr(this + 0x42).sh(c, flags);
    c.cycles += 1;
    1
}

/// The update of a node that is not complete (`AnimComplete == 0`): returns
/// v0 for the epilogue and charges the cycles of the path it takes.
pub(crate) fn oc664_anim_update(c: &mut Cpu, this: u32) -> Result<u64, Unwound> {
    // Frame-end marker is already stored. f1 = Delta, f2 = 0.0, fcc = 0 < Delta.
    c.cycles += 6;
    let delta = Addr(this + 0x6c).read_fpu(c);
    abi::set_fpr(c, 1, delta);
    abi::set_fpr(c, 2, FpuF32(0));
    let positive = abi::c_lt_s(c, FpuF32(0), delta);
    c.g[V0][0] = Addr(this + 0xc).lw(c);
    if !positive {
        // Idle node: the countdown steps down, then the flag path.
        c.cycles += 2;
        let delay = c.g[V0][0] as i64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if delay > 0 {
            c.cycles += 1;
            let left = c.g[V0][0];
            Addr(this + 0xc).sw(c, left);
        }
        c.cycles += 3;
        c.g[A2][0] = sx(this.wrapping_add(0x30));
        c.g[V0][0] = Addr(this + 8).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = Addr(this + 0x42).lhu(c);
            return Ok(oc664_set_flag_return(c, this));
        }
        // AnimComplete changed (unreachable here: nothing wrote it): the vtable call.
        c.cycles += 8;
        c.g[V1][0] = Addr(this + 0x3c).lw(c);
        c.g[A1][0] = 1;
        let table = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(table + 0x110).lh(c);
        c.g[V0][0] = Addr(table + 0x114).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_93a4)?;
        return Ok(0);
    }

    // Moving node: countdown >= 0 is required for an update.
    let delay = c.g[V0][0] as i64;
    c.cycles += 2;
    if delay < 0 {
        // Countdown negative: the flag path.
        return Ok(oc664_set_flag_return(c, this));
    }
    if delay == 0 {
        c.cycles += 2;
        if c.fcc() {
            // Delta still running: step it by the speed (never below zero).
            c.cycles += 7;
            let speed = Addr(this + 0x10).read_fpu(c);
            abi::set_fpr(c, 0, speed.abs());
            let stepped = FpuF32(c.f[1]) - FpuF32(c.f[0]);
            abi::set_fpr(c, 0, stepped);
            let below_zero = abi::c_lt_s(c, stepped, FpuF32(c.f[2]));
            Addr(this + 0x6c).write_fpu(c, stepped);
            if below_zero {
                c.cycles += 1;
                Addr(this + 0x6c).write_fpu(c, FpuF32(c.f[2]));
            }
        }
    }

    // The update: the object's per-frame method, then AnimActive from the new Delta.
    c.cycles += 8;
    c.g[A0][0] = c.g[S0][0];
    ANIMD_OBJECT_V15.call(c, 0x0035_934c)?;
    let after = Addr(this + 0x6c).read_fpu(c);
    abi::set_fpr(c, 1, after);
    abi::set_fpr(c, 0, FpuF32(0));
    let moving = abi::c_lt_s(c, FpuF32(0), after);
    c.g[V0][0] = 1;
    let one = c.g[V0][0];
    if moving {
        c.cycles += 2;
        Addr(this + 4).sw(c, one);
        return Ok(1);
    }
    c.cycles += 2;
    Addr(this + 4).sw(c, 0);
    c.cycles += 1;
    Ok(1)
}

/// `void cAnimDeltaNode_updateNode(cAnimDeltaNode *self)` (0x3592d0), returns
/// int: a complete node returns 1 at once; otherwise the frame ends are set to
/// 1e10, the countdown and delta step, and the object updates. Frame 0x20.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn anim_delta_v15_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        let no_frame = Addr((c.g[GP][0] as u32).wrapping_sub(0x2a6c)).read_fpu(c);
        abi::set_fpr(c, 0, no_frame);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        Addr(this + 0x28).write_fpu(c, no_frame);
        c.g[V0][0] = Addr(this + 8).lw(c);
        let live = c.g[V0][0] == 0;
        Addr(this + 0x24).write_fpu(c, no_frame);
        let v0 = if live {
            oc664_anim_update(c, this)?
        } else {
            c.cycles += 1;
            1
        };
        c.g[V0][0] = v0;
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool AnimDelta_v15(self)` (0x3592d0): one step of the delta animation:
/// stores the float literal at gp-0x2a6c to `+0x24` and `+0x28`, runs the
/// loop-counter / remaining-time bookkeeping (`+0xc`, `+0x6c`, `+0x10`, flags at `+0x42`) and calls `AnimObject_v15` or the
/// owner's virtual at `+0x110`. Returns 1 or 0.
pub fn anim_delta_v15_3(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_3593bc: {
            'b_3593b8: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2a6c));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                let t = c.g[V0][0] != 0u64;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x24));
                if t {
                    break 'b_3593b8;
                } else {
                    'b_3593b0: {
                        'b_359374: {
                            c.cycles += 6;
                            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x6c));
                            c.mtc1(0, 2);
                            c.c_lt_s(2, 1);
                            let t = !c.fcc();
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            if t {
                                break 'b_359374;
                            } else {
                                c.cycles += 2;
                                if (c.g[V0][0] as i64) < 0 {
                                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x42));
                                    break 'b_3593b0;
                                } else {
                                    'b_359344: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;

                                        if t {
                                            break 'b_359344;
                                        } else {
                                            c.cycles += 2;
                                            let t = !c.fcc();

                                            if t {
                                                break 'b_359344;
                                            } else {
                                                c.cycles += 7;
                                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x10));
                                                c.abs_s(0, 0);
                                                c.sub_s(0, 1, 0);
                                                c.c_lt_s(0, 2);
                                                let t = !c.fcc();
                                                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x6c));
                                                if t {
                                                    break 'b_359344;
                                                } else {
                                                    c.cycles += 1;
                                                    c.swc1(
                                                        2,
                                                        (c.g[S0][0] as u32).wrapping_add(0x6c),
                                                    );
                                                    break 'b_359344;
                                                }
                                            }
                                        }
                                    }
                                    'b_35936c: {
                                        c.cycles += 8;
                                        c.g[RA][0] = sx(0x0035934c);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x00341d48,
                                            f: ee::s11::AnimObject_v15,
                                        }
                                        .call(c, 0x0035934c)?;
                                        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x6c));
                                        c.mtc1(0, 0);
                                        c.c_lt_s(0, 1);
                                        let t = !c.fcc();
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_35936c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                            if t {
                                                break 'b_3593bc;
                                            } else {
                                                break 'b_35936c;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
                                    if t {
                                        break 'b_3593b8;
                                    } else {
                                        break 'b_359374;
                                    }
                                }
                            }
                        }
                        'b_359380: {
                            c.cycles += 2;
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            if t {
                                break 'b_359380;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                                break 'b_359380;
                            }
                        }
                        'b_3593ac: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_3593ac;
                            } else {
                                c.cycles += 8;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x110));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x114));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x003593a4);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x003593a4)?;
                                let t = true;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_3593bc;
                                } else {
                                    break 'b_3593ac;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x42));
                        break 'b_3593b0;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] | 0x1;
                    c.sh((c.g[S0][0] as u32).wrapping_add(0x42), c.g[V0][0]);
                    break 'b_3593b8;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_3593bc;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;

    use crate::difftest::fill;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;

    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4;
    pub(crate) use crate::game::bx::bxstringctor::tests::be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;

    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_4 as pool_allocator;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_6;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_6;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_3;

    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS as SEEDS_2;

    pub(crate) use crate::game::intersect::worldsphtree::tests::save_test;
    pub(crate) use crate::game::main::gameload::tests::fb;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_2;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_7;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_5;

    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn multi_particle_add_instance_matches() {
        check(
            ssx3_ee::s11::f_003581b8,
            multi_particle_add_instance,
            200,
            &|c, seed| {
                let mut r = Rng(0x1026 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((DATA + 0x100) as u64) | ((r.next_u32() as u64) << 32);
                let cap = 4 + (seed % 4);
                let count = if seed % 3 == 0 { cap } else { seed % 4 };
                c.bus.write32(DATA, cap);
                c.bus.write32(DATA + 4, count);
                c.bus.write32(DATA + 0xc, DATA + 0x200);
            },
        );
    }

    #[test]
    fn anim_combo_ctor_2_matches() {
        check(ee::s11::f_00358db0, anim_combo_ctor_2, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x8db0, 0);
            alloc_setup(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            let vt = vtable(c, &mut r, &[0x10]);
            c.bus.write32(DATA + 0x200, vt);
            c.bus
                .write32(this + 0x74, if seed % 3 == 0 { 0 } else { seed % 3 });
            crate::difftest::fill(c, &mut r, this + 0x18, 0x80);
            crate::difftest::fill(c, &mut r, DATA + 0x2000, 0x400 / 4);
        });
    }

    #[test]
    fn anim_combo_kill_matches() {
        check(ee::s11::f_003590c8, anim_combo_kill, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x90c8, 0);
            c.g[A0][0] = DATA as u64;
            c.bus.write8(DATA + 0x15, if seed % 2 == 0 { 0 } else { 1 });
            crate::difftest::fill(c, &mut r, DATA + 0x48, 0x40);
        });
    }

    #[test]
    fn anim_combo_save_matches() {
        check(ee::s11::f_003590f0, anim_combo_save, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x90f0, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            let vt = vtable(c, &mut r, &[8]);
            c.bus.write32(DATA + 0x200, vt);
            crate::difftest::fill(c, &mut r, DATA + 0x18, 0x40);
        });
    }

    #[test]
    fn anim_combo_dtor_matches() {
        check(ee::s11::f_00358e50, anim_combo_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x8e50, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.bus
                .write32(DATA + 0x84, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
        });
    }

    #[test]
    fn anim_combo_v42_matches() {
        check(ee::s11::f_00358f90, anim_combo_v42, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x8f90, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            crate::difftest::fill(c, &mut r, DATA + 0x48, 0x40);
            crate::difftest::fill(c, &mut r, 0x4ff120, 8);
        });
    }

    #[test]
    fn anim_combo_snapshot_matrices_matches() {
        check(
            ee::s11::f_00359030,
            anim_combo_snapshot_matrices,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x9030, 0);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus
                    .write32(this + 0x74, if seed % 5 == 0 { 0 } else { 1 + (seed % 3) });
                c.bus.write32(this + 0x60, DATA + 0x100);
                c.bus.write32(DATA + 0x100 + 0x80, DATA + 0x300);
                let n = seed % 4;
                c.bus.write32(DATA + 0x300 + 4, n);
                c.bus.write32(DATA + 0x300 + 8, DATA + 0x400);
                for i in 0..n {
                    c.bus.write32(
                        DATA + 0x400 + i * 0x10 + 8,
                        if (seed >> i) & 1 != 0 { 1 } else { 0 },
                    );
                    c.bus.write32(DATA + 0x400 + i * 0x10 + 0xc, r.next_u32());
                }
                c.bus.write32(this + 0x7c, DATA + 0x1000);
                c.bus.write32(this + 0x84, DATA + 0x2000);
                crate::difftest::fill(c, &mut r, DATA + 0x1000, 0x400 / 4);
                crate::difftest::fill(c, &mut r, DATA + 0x2000, 0x400 / 4);
            },
        );
    }

    #[test]
    fn multi_particle_update_matches() {
        crate::difftest::check(
            ee::s11::f_00358120__generated,
            multi_particle_update,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3581_20, 2);
                be_library(c);
                c.g[A0][0] = sx(DATA);
                // `p + 0x10` is the particle system; a null (or zeroed) system
                // makes `Emitter_Update` take its "descriptor time 0" exit.
                c.bus
                    .write32(DATA + 0x10, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
                c.bus.write32(DATA + 0x1000, 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x2a80), r.f32(1.0));
            },
        );
    }
    // gp - 0x4618: the 0.01 gain step
    pub(crate) const GP_1_30: u32 = 0x4a06_88;

    #[test]
    fn anim_delta_v36_matches() {
        check(
            ssx3_ee::s11::f_003593d0__generated,
            anim_delta_v36,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3593_d0, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0x8c } else { 0x10 };
                c.f[12] = r.edge_f32();
                w(c, DATA + 0x6c, r.edge_f32());
                w(c, GP_1_30, fb(1.0 / 30.0));
            },
        );
    }

    /// A nonzero BELibrary singleton, so the BE init path in `0x14dc80` does
    /// not try to create the library (and allocate) from disc.
    pub(crate) fn be_library_2(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
    }

    // -----------------------------------------------------------------------
    // 0x359228 AnimDelta_ctor.

    #[test]
    fn anim_delta_ctor_matches() {
        check(
            ee::s11::f_00359228__generated,
            anim_delta_ctor,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x3592_28, 0);
                pool_allocator(c);
                be_library_2(c);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x800);
                c.g[A3][0] = sx(DATA + 0xc00);
            },
        );
    }

    #[test]
    fn rail_man_save_matches() {
        save_test(
            ee::s11::f_00358c30__generated,
            rail_man_save,
            0x358c_30,
            0x18,
        );
    }

    #[test]
    fn anim_delta_save_matches() {
        save_test(
            ee::s11::f_00359410__generated,
            anim_delta_save,
            0x3594_10,
            0x6c,
        );
    }

    #[test]
    fn anim_combo_v36_matches() {
        check(
            ee::s11::f_00358fd0__generated,
            anim_combo_v36,
            SEEDS_4,
            &|c, seed| {
                let mut r = base(c, seed, 0x358f_d0);
                let s = DATA + 0x100;
                clear(c, s, 0x100);
                w_2(c, s + 0x14, if seed % 2 == 0 { 0 } else { 1 });
                w_2(c, s + 0x38, r.next_u32());
                w_2(c, s + 8, r.next_u32());
                w_2(c, s + 0x48, 0);
                w_2(c, s + 4, r.next_u32());
                c.f[12] = r.next_u32();
                c.g[A0][0] = sx(s);
                let msg = match seed % 6 {
                    0 => 0x82,
                    1 => 0x64,
                    2 => 0x65,
                    3 => 0x67,
                    4 => 0x68,
                    _ => 0xff,
                };
                c.g[A1][0] = msg as u64;
            },
        );
    }

    #[test]
    fn multi_particle_remove_instance_matches() {
        check(
            ssx3_ee::s11::f_003581f0__generated,
            multi_particle_remove_instance,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3581f0, 0);
                let p_2 = DATA + 0x1000;
                let entries = DATA + 0x2000;
                let count = (seed % 5) as u32;
                c.g[A0][0] = p_2 as u64;
                w32(c, p_2 + 4, count);
                w32(c, p_2 + 0xc, entries);
                for i in 0..5u32 {
                    w32(c, entries + 4 * i, if i < count { r.next_u32() } else { 0 });
                }
                // Half of the seeds look for a present key.
                c.g[A1][0] = if seed % 2 == 0 && count > 0 {
                    c.bus.read32(entries + 4 * ((seed % count) as u32)) as u64
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    #[test]
    fn multi_particle_remove_section_instances_matches() {
        check(
            ee::s11::f_00358140__generated,
            multi_particle_remove_section_instances,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x358140, 2);
                be_library_3(c);
                world(c);
                let this = DATA;
                let arr = DATA + 0x400;
                c.g[A0][0] = this as u64;
                let target = (seed % 3) as u64;
                c.g[A1][0] = target;
                let n = seed % 6;
                w(c, this + 4, n);
                w(c, this + 0xc, arr);
                let mut r = Rng(0x3581_0000 ^ seed as u64);
                for i in 0..n {
                    b(c, arr + i, (r.next_u32() % 3) as u8);
                }
            },
        );
    }

    #[test]
    fn railman_on_section_unload_matches() {
        check(
            ee::s11::f_00358700__generated,
            railman_on_section_unload,
            SEEDS_7,
            &|c, seed| {
                base_2(c, seed, 0x8700, 0);
                let m = DATA + 0x800;
                c.g[A0][0] = (seed % 3) as u64;
                c.bus
                    .write32(GAME_GP + 0xf50, if seed % 7 == 0 { 0 } else { m });
                for i in 0..0x80u32 {
                    let w = match (i + seed) % 8 {
                        0 => 0xffff_ffff,
                        1 => seed % 3,
                        _ => 0x100 + i,
                    };
                    c.bus.write32(m + 0x18 + 4 * i, w);
                }
            },
        );
    }

    #[test]
    fn rail_man_dtor_matches() {
        check(
            ee::s11::f_00358668__generated,
            rail_man_dtor,
            16,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x358668, 3);
                let obj = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                // One live rail plus unused slots, so the free loop stays short.
                for i in 0..0x80u32 {
                    w(c, obj + 0x18 + 4 * i, if i == 0 { 0 } else { 0xffff_ffff });
                }
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn rail_man_ctor_matches() {
        check(
            ee::s11::f_003584f0__generated,
            rail_man_ctor,
            SEEDS_8,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let _ = seed;
                c.g[A0][0] = this as u64;
            }),
        );
    }

    #[test]
    fn railman_ctor_2_matches() {
        check(
            ee::s11::f_00358588__generated,
            railman_ctor_2,
            8,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x358588, 2);
                let this = DATA;
                clear_2(c, this, 0x1000);
                let stream = DATA + 0x800;
                clear_2(c, stream, 0x100);
                let vt = vtable(c, &mut r, &[0x10]);
                w(c, stream, vt);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = stream as u64;
                c.g[A2][0] = (seed % 2) as u64;
                for i in 0..128u32 {
                    w(c, this + 0x18 + 4 * i, 0xffff_ffff);
                }
            },
        );
    }

    #[test]
    fn anim_combo_v52_matches() {
        diff(
            ee::s11::f_00359140__generated,
            anim_combo_v52_2,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_9140, 0);
                globals_3(c);
                let anim = DATA + 0x400;
                c.g[A0][0] = sx(DATA);
                // The animation object: no flags (early exit of AnimObject_v52).
                w(c, anim + 0x48, 0);
                w(c, DATA + 0x34, anim);
                w(
                    c,
                    DATA + anim_combo::ANIM,
                    if seed % 5 == 0 { 0 } else { DATA + 0x4000 },
                );
                c.bus.write8(DATA + anim_combo::PART_A, (seed % 2) as u8);
                c.bus
                    .write8(DATA + anim_combo::PART_B, ((seed >> 1) % 2) as u8);
                c.bus
                    .write8(DATA + anim_combo::PART_C, ((seed >> 2) % 2) as u8);
                w(c, DATA + anim_combo::MODEL, DATA + 0x6000);
                w(c, DATA + anim_combo::MATRICES, DATA + 0x5000);
                // The animation controller's part list.
                let ctrl = DATA + 0x6000;
                let count = (seed % 4) as u32;
                w(c, ctrl + 0x80, DATA + 0x7000);
                w(c, DATA + 0x7004, count);
                w(c, DATA + 0x7008, DATA + 0x7100);
                for i in 0..count {
                    let part = DATA + 0x7100 + 0x10 * i;
                    let obj = DATA + 0x7200 + 0x40 * i;
                    w(c, part + 8, obj);
                    w(c, obj + 0x1c, if seed % 3 == 0 { 0 } else { r.next_u32() });
                }
                for i in 0..8 {
                    w(c, DATA + 0x4000 + 4 * i, r.next_u32());
                    w(c, DATA + 0x5000 + 4 * i, r.next_u32());
                }
            },
        );
    }

    pub(crate) const RAIL_MAN: u32 = DATA + 0x1_0000;

    pub(crate) const RAIL_REF: u32 = DATA + 0x1_1000;

    pub(crate) const RAIL_PART: u32 = DATA + 0x1_2000;

    pub(crate) const RAIL_NODE: u32 = DATA + 0x1_3000;

    #[test]
    fn rail_man_enable_rail_matches() {
        check_all(
            ee::s11::f_00358998__generated,
            rail_man_enable_rail,
            24,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x3589_98, 8);
                c.g[A0][0] = sx(RAIL_MAN);
                c.g[A1][0] = sx(seed % 3);
                // `railman+0x18` is an array of 8-byte references whose object
                // (a multi-part) yields the collider list.
                c.bus.write32(RAIL_MAN + 0x18 + (seed % 3) * 8, RAIL_REF);
                c.bus.write32(RAIL_MAN + 0x1c + (seed % 3) * 8, 0);
                c.bus.write32(RAIL_PART, RAIL_NODE);
                c.bus
                    .write32(RAIL_PART + 0x4, if seed % 2 == 0 { 0 } else { 2 });
                c.bus.write32(
                    RAIL_NODE + 0x64,
                    if seed % 2 == 0 { 0 } else { RAIL_NODE + 0x100 },
                );
                for i in 0..6 {
                    c.bus.write32(RAIL_NODE + 0x6c + 4 * i, r.f32(10.0));
                }
            },
        );
    }

    // -------------------------------------------- MultiParticle_EmitAtInstances

    /// `MultiParticle_EmitAtInstances`: 0..4 handles, each with an empty
    /// table entry, an empty slot (index 0) or a live instance; the real
    /// `Emitter_RenderAt` runs on a descriptor with nothing pending (its two
    /// graphics-manager hooks are fakes, some unwind).
    #[test]
    fn multi_particle_emit_at_instances_matches() {
        use crate::difftest::put_vec;
        check(
            ee::s11::f_00358260__generated,
            multi_particle_emit_at_instances,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0035_8260, 8);
                c.pc = 0x0035_8260;
                let this = DATA + 0x1000;
                let refs = DATA + 0x1100;
                let desc = DATA + 0x2000;
                let world = DATA + 0x3000;
                let tables = DATA + 0x3100;
                let table = DATA + 0x3200;
                fill(c, &mut r, desc, 0x80);
                c.bus.write32(desc + 0xc, 0);
                c.bus.write32(GAME_GP + 0x16c8, world);
                c.bus.write32(world, tables);
                c.bus.write32(tables + 8, table);
                let count = seed % 5;
                c.bus.write32(
                    this + 4,
                    if seed % 11 == 0 {
                        (-(seed as i32)) as u32
                    } else {
                        count
                    },
                );
                c.bus.write32(this + 0xc, refs);
                c.bus.write32(this + 0x10, desc);
                for off in [0x20, 0x30, 0x40] {
                    put_vec(
                        c,
                        this + off,
                        [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)],
                    );
                }
                for i in 0..count {
                    let index = (r.next_u32() % 16) as u32;
                    let slot = r.next_u32() % 32;
                    c.bus.write32(refs + 4 * i, slot << 8 | index);
                    let entry = DATA + 0x4000 + 0x400 * i;
                    let slots = entry + 0x100;
                    let kind = r.next_u32() % 4;
                    c.bus
                        .write32(table + 4 * index, if kind == 0 { 0 } else { entry });
                    c.bus.write32(entry + 0x1c, slots);
                    let inst = DATA + 0x8000 + 0x100 * i;
                    let word = if kind == 1 {
                        r.next_u32() & 0xff
                    } else {
                        (inst >> 2) << 8 | (r.next_u32() & 0xff)
                    };
                    c.bus.write32(slots + 4 * slot, word);
                    put_vec(
                        c,
                        inst + 0x40,
                        [r.f32(500.0), r.f32(500.0), r.f32(500.0), r.f32(1.0)],
                    );
                }
            },
        );
    }

    /// `SyncProcess_dtor` with the real `TypedProcess`/`GameProcess`
    /// destructors on a heap-allocated process (objnode's fixture: linked or
    /// not, delete flag clear or set).
    #[test]
    fn sync_process_dtor_matches() {
        use crate::game::object::objnode::tests::oc286_heap_process;
        check(
            ee::s11::f_00354920__generated,
            sync_process_dtor,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_4920, 0);
                let this = oc286_heap_process(c, &mut r, seed);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx((seed & 3) as u32);
                c.pc = 0x0035_4920;
            },
        );
    }

    /// MultiParticle_Save: a stream whose `Write` slot is a fake (random
    /// `this` delta, some calls unwind), the real Emitter_Save, and an item
    /// count that is negative, zero or positive.
    #[test]
    fn multi_particle_save_matches() {
        covered(
            ee::s11::f_00358380__generated,
            multi_particle_save,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_8380, 7);
                c.pc = 0x0035_8380;
                let vt = vtable(c, &mut r, &[MP_STREAM_WRITE]);
                let this = DATA + 0x20_0000;
                let stream = DATA + 0x20_0400;
                let emitter = DATA + 0x20_0800;
                fill(c, &mut r, this, 0x20);
                fill(c, &mut r, emitter, 0x10);
                c.bus.write32(this + MP_EMITTER, emitter);
                let count = match seed % 4 {
                    0 => 0,
                    1 => (r.next_u32() | 0x8000_0000) as i32,
                    _ => (r.next_u32() % 0x40) as i32 + 1,
                };
                c.bus.write32(this + MP_COUNT, count as u32);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    use crate::difftest::put_vec;
    use crate::difftest::put_words;

    /// `SplineModifier_ctor`: forward and backward speeds (the initial
    /// distance), a descriptor override, zero and non-zero jitter (the clamp
    /// and no-clamp paths), and every float edge case.
    #[test]
    fn spline_modifier_ctor_matches() {
        check(
            ee::s11::f_00359460__generated,
            spline_modifier_ctor,
            240,
            &|c, seed| {
                let mut r = Rng(0x3594_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                let sdata = DATA + 0x400;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = sdata as u64;
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_PATH, r.next_u32() % 64);
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_A, r.next_u32());
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_B, r.next_u32());
                let speed = if seed % 3 == 0 {
                    r.f32(100.0) | 0x8000_0000
                } else {
                    r.f32(100.0)
                };
                c.bus.write32(sdata + SPLINE_MODIFIER_CTOR_D_SPEED, speed);
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_ANGLE, r.edge_f32());
                c.bus.write32(
                    sdata + SPLINE_MODIFIER_CTOR_D_OVERRIDE,
                    if seed % 3 == 0 { 0 } else { r.f32(20.0) },
                );
                c.bus.write32(
                    sdata + SPLINE_MODIFIER_CTOR_D_JITTER,
                    if seed % 7 == 0 { 0 } else { r.f32(10.0) },
                );
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_FLAG, (seed % 2) as u32);
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_SCALE, r.edge_f32());
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_END, r.edge_f32());
                // The `.lit4` constants the ctor and SetSpeed read, the RNG state
                // and the default `sMpQuad`.
                put_words(
                    c,
                    GAME_GP.wrapping_sub(0x2a64),
                    &[SPLINE_MODIFIER_CTOR_DEG2RAD],
                );
                put_words(
                    c,
                    GAME_GP.wrapping_sub(0x2a60),
                    &[SPLINE_MODIFIER_CTOR_TENTH],
                );
                c.bus.write32(0x004f_f030, r.next_u32() | 1);
                put_vec(
                    c,
                    SPLINE_MODIFIER_CTOR_QUAD,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
            },
        );
    }

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_run;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_w;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC433_FIX;

    /// `MultiParticle_CreateEmitter` against the generated body: 0..4
    /// emitters (an empty table allocates nothing to fill).
    #[test]
    fn multi_particle_create_emitter_matches() {
        oc433_run(
            ee::s11::f_00358020__generated,
            multi_particle_create_emitter,
            0x0035_8020,
            4,
            100,
            &|c, r, seed| {
                let this = OC433_FIX + 0x1000;
                oc433_w(c, this, seed % 5);
                fill(c, r, this + 8, 1);
                c.g[A0][0] = sx(this);
            },
        );
    }

    use crate::difftest::fakes::FAKE;
    use crate::difftest::RA as SENTINEL;

    use crate::difftest::fakes;
    use std::sync::atomic::AtomicUsize;

    pub(crate) use crate::game::object::movenode::tests::vt_obj;

    #[test]
    fn boost_modifier_base_ctor_2_matches() {
        check(
            ee::s11::f_00358468__generated,
            boost_modifier_base_ctor_2,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x358468, 1);
                let this = DATA + 0x100;
                let o = DATA + 0x200;
                let vt = DATA + 0x400;
                vt_obj(c, o, vt, (seed % 3) as u16 * 0x10, FAKE + 0x40);
                c.bus.write32(this + 0x4, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(o);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0035_8468;
            },
        );
    }

    /// The pools and arena `cMemMan_alloc` needs for the manager (the same
    /// setup as `rbc_test_allocator`).
    pub(crate) fn rail_test_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            for n in 0..16u32 {
                let node = nodes + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, nodes);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
        c.g[A0][0] = 0x004f_f250;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
    }

    #[test]
    fn rail_man_set_rail_enabled_matches() {
        check(
            ee::s11::f_00358b28__generated,
            rail_man_set_rail_enabled,
            300,
            &|c, seed| {
                let mut r = Rng(0x5a17_0b28 ^ seed as u64);
                let key = (r.next_u32() % 6) as u64;
                let add = (seed % 3) as u64;
                let gp = GAME_GP;
                c.g[GP][0] = gp as u64;
                let mgr = DATA + 0x1000;
                if seed % 4 == 0 {
                    // no manager yet: the allocator and constructor run
                    c.bus.write32(gp + RAIL_ACTIVE_NODE_GP, 0);
                    rail_test_allocator(c);
                } else {
                    c.bus.write32(gp + RAIL_ACTIVE_NODE_GP, mgr);
                    for i in 0..RAIL_SLOTS as u32 {
                        let v = match seed % 7 {
                            // no key and no free slot: the search runs to the end
                            0 => 100 + r.next_u32() % 50,
                            _ => match r.next_u32() % 4 {
                                0 => u32::MAX,
                                _ => r.next_u32() % 6,
                            },
                        };
                        c.bus.write32(mgr + RAIL_INFO + 4 * i, v);
                    }
                }
                c.g[A0][0] = key;
                c.g[A1][0] = add;
                c.pc = 0x0035_8b28;
            },
        );
    }

    pub(crate) fn put_f(c: &mut Cpu, addr: u32, v: f32) {
        c.bus.write32(addr, v.to_bits());
    }

    #[test]
    fn anim_combo_ctor_matches() {
        check(
            ee::s11::f_00358c80__generated,
            anim_combo_ctor,
            240,
            &|c, seed| {
                let mut r = Rng(0x3c80_0358 ^ seed as u64);
                let gp = GAME_GP;
                c.g[GP][0] = gp as u64;
                // the allocator for the transform array (operator new)
                rail_test_allocator(c);
                let this = DATA + 0x2000;
                let inst = DATA + 0x3000;
                let model = DATA + 0x4000;
                let adata = DATA + 0x3800;
                let timer = DATA + 0x5000;
                let frac = |r: &mut Rng| ((r.next_u32() % 2000) as f32 - 1000.0) * 0.01;
                // float constants of the game (1/30 and a scale), and the timer
                put_f(c, gp.wrapping_sub(0x2a7c), 1.0 / 30.0);
                put_f(c, gp.wrapping_sub(0x2a78), 1.0 / 30.0);
                put_f(c, gp.wrapping_sub(0x2a74), 0.5);
                c.bus.write32(gp + 0x2a74, timer); // cAppMan_TheAppMan
                c.bus.write32(timer + 0x10, 30 + seed % 30); // TimerHz
                c.bus.write32(inst + 0x80, model); // pInstance->m_Model
                put_f(c, model + 0x14, frac(&mut r)); // m_fAnimTime
                let from = frac(&mut r).abs() + 0.25;
                put_f(c, adata + 0x28, if seed % 2 == 0 { from } else { -from }); // FromTime2
                put_f(c, adata + 0x10, frac(&mut r)); // ToTime
                let to2 = frac(&mut r).abs() + 0.25;
                put_f(c, adata + 0x2c, if seed % 3 == 0 { -to2 } else { to2 }); // ToTime2
                put_f(c, adata + 0x30, frac(&mut r)); // Speed2
                let stop = [0u32, 1, u32::MAX, 2][((seed / 3) % 4) as usize];
                c.bus.write32(adata + 0x34, stop); // Stop
                c.bus.write32(this + 0x74, seed % 4); // NumAnimMeshes
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = inst as u64;
                c.g[A3][0] = adata as u64;
                c.pc = 0x0035_8c80;
            },
        );
    }

    /// Gate word for the fakes below: they act only when this machine's RAM
    /// holds `OC608_MAGIC` at `OC608_CTRL`.
    pub(crate) const OC608_CTRL: u32 = DATA + 0x2_0000;

    pub(crate) const OC608_MAGIC: u32 = 0x0608_0608;

    pub(crate) static OC608_PREV_STEP: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC608_PREV_FRAME: AtomicUsize = AtomicUsize::new(0);

    /// Gated `AnimObject_v15`: sets the object's `v` (+0x6c) from the fixture,
    /// or unwinds when asked to.
    pub(crate) fn oc608_fake_step(c: &mut Cpu) {
        if c.bus.read32(OC608_CTRL) != OC608_MAGIC {
            return fakes::fall_back(c, &OC608_PREV_STEP, ee::s11::f_00341d48__generated);
        }
        let this = c.g[A0][0] as u32;
        let v = c.bus.read32(OC608_CTRL + 4);
        c.bus.write32(this.wrapping_add(0x6c), v);
        if c.bus.read32(OC608_CTRL + 16) != 0 {
            c.pc = 0xdead_0000;
            return;
        }
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `cAnimNode_frameUpdate` (`AnimObject_FireEndScript`): returns the
    /// fixture's result, or unwinds when asked to.
    pub(crate) fn oc608_fake_frame(c: &mut Cpu) {
        if c.bus.read32(OC608_CTRL) != OC608_MAGIC {
            return fakes::fall_back(c, &OC608_PREV_FRAME, ee::s11::f_0034eba0__generated);
        }
        c.g[V0][0] = sx(c.bus.read32(OC608_CTRL + 8));
        if c.bus.read32(OC608_CTRL + 12) != 0 {
            c.pc = 0xdead_0000;
            return;
        }
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn oc608_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_00341D48),
            oc608_fake_step,
            &OC608_PREV_STEP,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0034EBA0),
            oc608_fake_frame,
            &OC608_PREV_FRAME,
        );
    }

    #[test]
    fn anim_combo_v15_matches() {
        oc608_install();
        check(
            ee::s11::f_00358eb8__generated,
            anim_combo_v15,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x358e_b8, 0);
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x2a70), 1e10f32.to_bits());
                let s = DATA + 0x100;
                fill(c, &mut r, s, 0x80);
                c.bus.write32(OC608_CTRL, OC608_MAGIC);
                c.bus.write32(OC608_CTRL + 4, r.f32(1.5));
                c.bus
                    .write32(OC608_CTRL + 8, if seed % 5 == 0 { 0 } else { 1 + seed % 3 });
                c.bus.write32(OC608_CTRL + 12, (seed % 11 == 0) as u32);
                c.bus.write32(OC608_CTRL + 16, (seed % 17 == 0) as u32);
                c.bus
                    .write32(s + 0x00, (0.25 + (seed % 4) as f32 * 0.5).to_bits());
                c.bus.write32(s + 0x04, ((seed % 5) as f32).to_bits());
                c.bus
                    .write32(s + 0x0c, (1.0 + (seed % 6) as f32 * 1.5).to_bits());
                c.bus.write32(s + 0x34, ((seed % 3) as f32 * 2.0).to_bits());
                c.bus.write32(
                    s + 0x24,
                    match (seed / 4) % 4 {
                        0 => (-2i32) as u32,
                        2 => 3,
                        _ => 0,
                    },
                );
                c.bus.write8(s + 0x14, if seed % 4 == 0 { 0 } else { 1 });
                c.bus.write8(s + 0x15, if seed % 9 == 0 { 1 } else { 0 });
                c.bus.write8(s + 0x16, ((seed / 2) % 2) as u8);
                c.g[A0][0] = sx(s);
                c.pc = 0x0035_8eb8;
            },
        );
    }

    #[test]
    fn rail_man_disable_rail_matches() {
        check(
            ee::s11::f_00358780__generated,
            rail_man_disable_rail,
            200,
            &|c, seed| {
                machine(c, seed, 0x8780, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00358780;
            },
        );
    }

    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::prev_cell;
    use crate::game::object::modifierblock::tests::oc664_install_slots;
    pub(crate) use crate::game::object::modifierblock::tests::OC664_CTRL;
    pub(crate) use crate::game::object::modifierblock::tests::OC664_MAGIC;
    use std::ptr::addr_of_mut;

    /// The stream's read method: outside the function table, so it reaches
    /// the machine's miss hook (`oc664_miss`).
    pub(crate) const OC664_READ: u32 = FAKE + 0x800;

    /// The read cursor, the stream object (its vtable at +0) and its vtable.
    pub(crate) const OC664_CURSOR: u32 = DATA + 0x3f40;

    pub(crate) const OC664_STREAM: u32 = DATA + 0x3f80;

    pub(crate) const OC664_VT: u32 = DATA + 0x3fc0;

    /// The bytes the reads consume.
    pub(crate) const OC664_BYTES: u32 = DATA + 0x3400;

    /// The block the gated `addModifierBlock` fake hands to a move node.
    pub(crate) const OC664_BLOCK: u32 = DATA + 0x3800;

    pub(crate) const OC664_ANIM: u32 = DATA + 0x3c00;

    /// The stream's read: copies the next `a2` bytes of the stream to `a1`.
    /// Other misses are plain fake calls.
    pub(crate) fn oc664_miss(c: &mut Cpu, target: u32) {
        if target != OC664_READ {
            return fake_call(c, target);
        }
        let dst = c.g[A1][0] as u32;
        let len = c.g[A2][0] as u32;
        let at = c.bus.read32(OC664_CURSOR);
        for i in 0..len {
            let b = c.bus.read8(OC664_BYTES + at + i);
            c.bus.write8(dst + i, b);
        }
        c.bus.write32(OC664_CURSOR, at + len);
        c.pc = c.g[RA][0] as u32;
    }

    /// Gate on, the miss hook, and the stream object with its read vtable.
    pub(crate) fn oc664_gate(c: &mut Cpu) {
        c.hooks.miss = oc664_miss;
        c.bus.write32(OC664_CTRL, OC664_MAGIC);
        c.bus.write32(OC664_CURSOR, 0);
        c.bus.write32(OC664_STREAM, OC664_VT);
        c.bus.write16(OC664_VT + 0x10, 0);
        c.bus.write32(OC664_VT + 0x14, OC664_READ);
    }

    /// `addModifierBlock`: the node's block pointer (+0x1c) is set, then the call is logged.
    pub(crate) fn oc664_fake_block(c: &mut Cpu) {
        if c.bus.read32(OC664_CTRL) != OC664_MAGIC {
            return fall_back(
                c,
                prev_cell(oc664_fake_block),
                ee::s11::f_00355468__generated,
            );
        }
        let node = c.g[A0][0] as u32;
        c.bus.write32(node + 0x1c, OC664_BLOCK);
        fake_call(c, 0x0035_5468);
    }

    pub(crate) fn oc664_install() {
        oc664_install_slots();
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_00355468),
            oc664_fake_block,
            prev_cell(oc664_fake_block),
        );
    }

    #[test]
    fn anim_delta_ctor_2_matches() {
        oc664_install();
        check(
            ee::s11::f_00359270__generated,
            anim_delta_ctor_2_3,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3592_70, 2);
                oc664_gate(c);
                c.bus.write32(OC664_BYTES, r.next_u32());
                c.g[A0][0] = OC664_ANIM as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = OC664_STREAM as u64;
            },
        );
    }

    pub(crate) use crate::game::object::flexrailnode::tests::FIX;

    pub(crate) use crate::game::object::flexrailnode::tests::oc687_gate;
    pub(crate) use crate::game::object::flexrailnode::tests::oc687_install_fakes;

    /// `AnimDelta_v15`: the update-state word, the loop counter at +0xc
    /// (negative, zero, positive), the float at +0x6c against zero, the
    /// delta at +0x10 and the flags halfword.
    #[test]
    fn anim_delta_v15_matches() {
        oc687_install_fakes();
        fakes::covered(
            ee::s11::f_003592d0__generated,
            anim_delta_v15_3,
            400,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x35_92d0, 2);
                oc687_gate(c);
                c.pc = 0x0035_92d0;
                let (me, owner) = (FIX, FIX + 0x1000);
                fill(c, &mut r, me, 0x20);
                c.bus
                    .write32(me + 8, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(
                    me + 0xc,
                    [u32::MAX, 0, 1, 5, 0x8000_0000, r.next_u32()][((seed / 2) % 6) as usize],
                );
                let f = match (seed / 4) % 5 {
                    0 => 0,
                    1 => 1.0f32.to_bits(),
                    2 => (-1.0f32).to_bits(),
                    3 => 0.25f32.to_bits(),
                    _ => r.f32(2.0),
                };
                c.bus.write32(me + 0x6c, f);
                c.bus.write32(me + 0x10, r.f32(2.0));
                c.bus.write32(me + 0x3c, owner);
                let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                c.bus.write16(owner + 0x110, delta as u16);
                c.bus.write32(owner + 0x114, fakes::FAKE);
                c.bus.write32(GAME_GP - 0x2a6c, r.f32(2.0));
                c.g[A0][0] = me as u64;
            },
        );
    }
}
