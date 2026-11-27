//! `src/visualfx/avalanche.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::rider::gp_at;
pub(crate) use crate::game::dirtysock::tagsunk::gp_9 as gp;
pub(crate) use crate::game::dirtysock::tagsunk::lw;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::festateloadhint::gp as gp_2;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::object::instancenode::ui_vector_lw;
pub(crate) use crate::game::object::movenode::AVALANCHE_GRMAN_GP;
pub(crate) use crate::game::render::graphicsman::RegMem;
use crate::guest::Addr;
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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_6378, vfx_avalanche_dtor, "VfxAvalanche_dtor"),
    (0x002d_6410, vfx_avalanche_init, "VfxAvalanche_Init"),
    (0x002d_64d8, avalanche_clamp_vector, "Avalanche_ClampVector"),
    (
        0x002d_7c00,
        avalanche_compute_instance_fade,
        "Avalanche_ComputeInstanceFade",
    ),
    (
        0x002d_7dd8,
        avalanche_release_instance,
        "Avalanche_ReleaseInstance",
    ),
    (
        0x002d_7e60,
        avalanche_build_active_node_array,
        "Avalanche_BuildActiveNodeArray",
    ),
    (
        0x002d_7ef8,
        avalanche_update_active,
        "Avalanche_UpdateActive",
    ),
    (0x002d_9660, avalanche_bake_group, "Avalanche_BakeGroup"),
    (
        0x002d_9738,
        avalanche_add_node_event,
        "Avalanche_AddNodeEvent",
    ),
    (
        0x002d_9a80,
        avalanche_free_histories,
        "Avalanche_FreeHistories",
    ),
    (
        0x002d_9c00,
        avalanche_get_object_matrix,
        "Avalanche_GetObjectMatrix",
    ),
    (
        0x002d_9fb8,
        avalanche_set_history_data,
        "Avalanche_SetHistoryData",
    ),
    (
        0x002f_7e70,
        avalanche_destroy_all_thunk,
        "Avalanche_DestroyAllThunk",
    ),
    (0x002d_9130, vfx_avalanche_render, "VfxAvalanche_Render"),
    (
        0x002d_a028,
        avalanche_apply_history_data,
        "Avalanche_ApplyHistoryData",
    ),
    (0x002d_8948, vfx_avalanche_update, "VfxAvalanche_Update"),
    (0x002d_9cb0, avalanche_save_state, "Avalanche_SaveState"),
    (
        0x002d_8ea8,
        avalanche_render_active,
        "Avalanche_RenderActive",
    ),
    (
        0x002d_5778,
        avalanche_sample_history,
        "Avalanche_SampleHistory",
    ),
    (
        0x002d_66a0,
        avalanche_bake_node_history,
        "Avalanche_BakeNodeHistory",
    ),
    (0x002d_81b0, active_avalanche_end, "Avalanche_Stop"),
    (
        0x002d_8258,
        avalanche_export_history_files,
        "Avalanche_ExportHistoryFiles",
    ),
    (
        0x002d_83b8,
        avalanche_write_history_file,
        "Avalanche_WriteHistoryFile",
    ),
    (
        0x002d_87d0,
        avalanche_load_history_file,
        "Avalanche_LoadHistoryFile",
    ),
    (
        0x002d_8a00,
        avalanche_debug_draw_instance,
        "Avalanche_DebugDrawInstance",
    ),
    (0x002d_92d0, avalanche_add_node, "Avalanche_AddNode"),
    (
        0x002d_9538,
        avalanche_set_node_emitter,
        "Avalanche_SetNodeEmitter",
    ),
    (0x002d_96e0, avalanche_bake_all_2, "Avalanche_BakeAll"),
    (0x002d_97a8, avalanche_start, "Avalanche_Start"),
    (0x002d_9d68, avalanche_load_state, "Avalanche_LoadState"),
    (
        0x002d_7ca8,
        oc1075_avalanche_update_instance,
        "Avalanche_UpdateInstance",
    ),
    (
        0x002d_82a0,
        oc1075_avalanche_write_data,
        "Avalanche_WriteData",
    ),
    (
        0x002d_a1c0,
        oc1075_avalanche_get_proximity_info,
        "Avalanche_GetProximityInfo",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::AVALANCHE_LOAD_HISTORY_FILE;

    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;

    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_2;

    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
}

/// `void Avalanche_ComputeInstanceFade(void *self, float dt)` (0x2d7c00):
/// advance the 30Hz fade of the particle instance at `self+0x2e0`, writing the
/// fade ramp at `self+0xb0` and the fade fraction at `self+0xb4`.
pub fn avalanche_compute_instance_fade(c: &mut Cpu) {
    c.cycles += 9;
    c.g[AT][0] = 0x41f0_0000; // 30.0
    c.mtc1(AT, 1);
    let this = abi::arg(c, 0);
    let inst = Addr(Addr(this).off(0x2e0).lw(c) as u32);
    c.g[V0][0] = inst.0 as u64;
    c.mul_s(12, 12, 1);
    c.lwc1(0, inst.off(0x104).0);
    c.c_lt_s(12, 0);
    if !c.fcc() {
        // dt >= lifetime: either ramp down from the range, or snap to 1.
        c.lwc1(1, inst.off(0x100).0);
        c.cycles += 6;
        c.lwc1(2, inst.off(0x108).0);
        c.sub_s(0, 1, 2);
        c.c_lt_s(12, 0);
        if !c.fcc() {
            c.g[V1][0] = inst.off(0xf0).lhu(c);
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 7;
                c.sub_s(0, 1, 12);
                c.mtc1(ZERO, 1);
                c.div_s(0, 0, 2);
                c.c_lt_s(1, 0);
                c.swc1(0, this.wrapping_add(0xb4));
                let nonpos = !c.fcc();
                c.swc1(0, this.wrapping_add(0xb0));
                if !nonpos {
                    c.cycles += 2;
                    c.sqrt_s(0, 0);
                    c.swc1(0, this.wrapping_add(0xb0));
                }
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        } else {
            c.cycles += 5;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.swc1(0, this.wrapping_add(0xb0));
            c.swc1(0, this.wrapping_add(0xb4));
            abi::ret(c);
            return;
        }
    } else {
        c.cycles += 7;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.swc1(1, this.wrapping_add(0xb0));
        c.lwc1(0, inst.off(0x104).0);
        c.div_s(0, 12, 0);
        c.swc1(0, this.wrapping_add(0xb4));
        abi::ret(c);
        return;
    }
}

// ---------------------------------------------------------------------------
// Avalanche node events.

/// `void Avalanche_AddNodeEvent(Avalanche *ava, int event, int node)`
/// (0x2d9738): if the node's event count at +0xe is below 0x40, look up the
/// node in the list at +4 (following +0xf4) and store `event` in the halfword
/// table at `ava + 0x110 + count*2` and the node's index in the table at
/// `ava + 0x112 + count*2`, then bump the count. Back-edge event check at
/// 0x2d9768.
pub fn avalanche_add_node_event(c: &mut Cpu) {
    const COUNT: u32 = 0xe;
    const LIST: u32 = 0x4;
    const LINK: u32 = 0xf4;
    const EVENTS: u32 = 0x110;
    const NODES: u32 = 0x112;
    const HEAD: u32 = 0x002d_9768;
    c.cycles += 4;
    let ava = abi::arg(c, 0);
    let event = abi::arg64(c, 1);
    let node = abi::arg64(c, 2);
    c.g[V0][0] = Addr(ava).off(COUNT).lhu(c);
    c.g[V0][0] = (c.g[V0][0] < 0x40) as u64;
    let full = c.g[V0][0] == 0;
    c.g[A3][0] = 0;
    if full {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    let mut immediate = false;
    {
        c.cycles += 5;
        let mut link = Addr(ava).off(LIST).lw(c);
        c.g[V1][0] = link;
        c.g[V0][0] = Addr(ava).off(COUNT).lhu(c);
        c.g[T0][0] = sx(ava.wrapping_add(EVENTS));
        let empty = link == 0;
        c.g[T1][0] = sx(ava.wrapping_add(NODES));
        if !empty {
            c.cycles += 2;
            if link == node {
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                immediate = true;
            } else {
                c.cycles += 1;
                link = Addr(link as u32).off(LINK).lw(c);
                c.g[V1][0] = link;
                loop {
                    c.cycles += 2;
                    let end = link == 0;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    if end {
                        break;
                    }
                    c.cycles += 2;
                    if link != node {
                        link = Addr(link as u32).off(LINK).lw(c);
                        c.g[V1][0] = link;
                        if event_due(c) {
                            deliver_event_here(c, HEAD);
                        }
                        continue;
                    }
                    break;
                }
            }
        }
    }
    if !immediate {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    }
    c.cycles += 9;
    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sh(c.g[V0][0] as u32, event);
    c.g[V1][0] = Addr(ava).off(COUNT).lhu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sh(c.g[V1][0] as u32, c.g[A3][0]);
    c.g[V0][0] = Addr(ava).off(COUNT).lhu(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.sh(ava.wrapping_add(COUNT), c.g[V0][0]);
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Avalanche active-node array.

/// `void Avalanche_BuildActiveNodeArray(Avalanche *ava)` (0x2d7e60): count
/// the nodes reachable through `ava->nodes(+4)` -> +0x2e4, allocate
/// `count*4` bytes (name 0x487548) and store the pointer at +8, then fill the
/// array with the node pointers. Back-edge event checks at 0x2d7e80 /
/// 0x2d7ec8.
pub fn avalanche_build_active_node_array(c: &mut Cpu) {
    guest(c, |c| {
        const NODES: u32 = 0x4;
        const ARRAY: u32 = 0x8;
        const LINK: u32 = 0x2e4;
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let ava = abi::arg(c, 0);
        c.g[S0][0] = ava as u64;
        let mut link = Addr(ava).off(NODES).lw(c);
        c.g[A1][0] = link;
        c.g[A0][0] = 0;
        if link != 0 {
            c.cycles += 1;
            let mut count = 0u32;
            loop {
                c.cycles += 7;
                link = Addr(link as u32).off(LINK).lw(c);
                c.g[A1][0] = link;
                let more = link != 0;
                count = count.wrapping_add(1);
                c.g[A0][0] = sx(count);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_7e80);
                }
            }
        }
        // operator_new_named(count*4, 0x487548, 0, 0)
        c.cycles += 10;
        c.g[A1][0] = 0x0048_0000;
        let size = (c.g[A0][0] as u32) << 2;
        c.g[A0][0] = sx(size);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_add(0x7548));
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x002d_7eb4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x002d_7eb4)?;
        let array = c.g[V0][0];
        let mut link = Addr(ava).off(NODES).lw(c);
        c.g[A1][0] = link;
        c.g[A0][0] = 0;
        let empty = link == 0;
        Addr(ava).off(ARRAY).sw(c, array);
        if !empty {
            c.cycles += 1;
            let mut i = 0u32;
            loop {
                c.cycles += 7;
                let base = Addr(ava).off(ARRAY).lw(c);
                let p = (i << 2).wrapping_add(base as u32);
                c.g[V0][0] = sx(p);
                c.g[V1][0] = base;
                Addr(p).sw(c, link);
                link = Addr(link as u32).off(LINK).lw(c);
                c.g[A1][0] = link;
                let more = link != 0;
                i = i.wrapping_add(1);
                c.g[A0][0] = sx(i);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_7ec8);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Avalanche_DestroyAllThunk(void)` (0x2f7e70): `Avalanche_DestroyAll()`.
pub fn avalanche_destroy_all_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x002d_9b40,
            f: ssx3_ee::s09::f_002d9b40,
        }
        .call(c, 0x002f_7e80)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Avalanche_BakeGroup(AvalancheGroup* self)` (0x2d9660): bake the
/// history of every node linked from `+0x4` (through `+0xf4`).
pub fn avalanche_bake_group(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = Addr(this).off(4).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                callee::AVALANCHE_BAKE_NODE_HISTORY.call(c, 0x002d_9690)?;
                c.g[S0][0] = Addr(r32(c, S0)).off(0xf4).lw(c);
                let more = c.g[S0][0] != 0;
                c.g[A0][0] = c.g[S0][0];
                if !more {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9688);
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Avalanche_BakeAll(void)` (0x2d96e0): free the histories, bake every
/// group, then export and reload the history files.
///
/// Not installed: the unconditional `Avalanche_ExportHistoryFiles` /
/// `Avalanche_LoadHistoryFile` enter the EAFile request queue, which cannot be
/// bounded in a test fixture (no request slots / no disc backing), so the body
/// has no differential test.
pub fn avalanche_bake_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        callee::AVALANCHE_FREE_HISTORIES.call(c, 0x002d_96f4)?;
        c.g[S0][0] = lw(c, gp(c, 0x9c0));
        if c.g[S0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[RA][0] = sx(0x002d_9708);
                c.g[A0][0] = c.g[S0][0];
                callee::AVALANCHE_BAKE_GROUP.call(c, 0x002d_9708)?;
                c.g[S0][0] = Addr(r32(c, S0)).off(8).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9700);
                }
            }
        }
        c.cycles += 8;
        c.g[RA][0] = sx(0x002d_971c);
        callee::AVALANCHE_EXPORT_HISTORY_FILES.call(c, 0x002d_971c)?;
        c.g[RA][0] = sx(0x002d_9724);
        callee::AVALANCHE_LOAD_HISTORY_FILE.call(c, 0x002d_9724)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Avalanche_SetHistoryData(void* data)` (0x2d9fb8): replace the
/// avalanche history data, freeing the old block and re-resolving the new
/// data's pointers when it carries the magic tag.
pub fn avalanche_set_history_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call::call_ptr(c, 0x002d_9b40, 0x002d_9fcc)?;
        c.g[A0][0] = Addr(gp_2(c, 0x9c4)).lw(c);
        let no_old = c.g[A0][0] == 0;
        c.g[V0][0] = Addr(gp_2(c, 0x9c8)).lw(c);
        let mut set = false;
        if no_old {
            set = true;
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                Addr(gp_2(c, 0x9c4)).sw(c, c.g[S0][0]);
            } else {
                c.cycles += 2;
                call::call_ptr(c, 0x0031_7e98, 0x002d_9fe8)?;
                set = true;
            }
        }
        if set {
            c.cycles += 1;
            Addr(gp_2(c, 0x9c4)).sw(c, c.g[S0][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = 0x02be_ef00;
        Addr(gp_2(c, 0x9c8)).sw(c, 0);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        let magic = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 1;
        if !magic {
            c.cycles += 3;
            Addr(gp_2(c, 0x9c4)).sw(c, 0);
            Addr(gp_2(c, 0x9d4)).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 2;
            Addr(gp_2(c, 0x9d4)).sw(c, 0);
            call::call_ptr(c, 0x002d_a028, 0x002d_a018)?;
        }
        c.cycles += 4;
        lqsp(c, 16, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Avalanche.

/// `void Avalanche_ReleaseInstance(Avalanche* self, int force)` (0x2d7dd8):
/// walk to the live emitter (or use the first one when forced), release its
/// sub-emitter and reset the avalanche's colours.
pub fn avalanche_release_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p(r32(c, S0), 0x2e0)).lw(c);
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            // Pick the emitter: the first when forced, else the first live one.
            let mut release = true;
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                c.g[A0][0] = Addr(p(r32(c, A0), 0xf8)).lw(c);
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(p(r32(c, A0), 0xf0)).lhu(c);
                c.g[V0][0] = 0x2;
                if c.g[V1][0] == c.g[V0][0] {
                    release = false;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = Addr(p(r32(c, A0), 0xf8)).lw(c);
                }
            }
            if !release {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x2e0)).lw(c);
                } else {
                    c.cycles += 3;
                    c.g[A2][0] = Addr(p(r32(c, A0), 0xc)).lw(c);
                    let no_sub = c.g[A2][0] == 0;
                    c.g[A1][0] = 0x3;
                    if !no_sub {
                        c.cycles += 5;
                        c.g[V0][0] = Addr(p(r32(c, A2), 0xc)).lw(c);
                        c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
                        c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
                        let target = r32(c, V1);
                        c.g[RA][0] = sx(0x002d_7e38);
                        c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                        call_ptr(c, target, 0x002d_7e38)?;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x2e0)).lw(c);
                }
                c.cycles += 5;
                c.g[A0][0] = sx(p(r32(c, S0), 0xd0));
                Addr(p(r32(c, V0), 0x114)).sw(c, 0);
                c.g[RA][0] = sx(0x002d_7e4c);
                Addr(p(r32(c, S0), 0x2e0)).sw(c, 0);
                call_ptr(c, 0x0037_1600, 0x002d_7e4c)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `VfxAvalanche_dtor` (0x2d6378): destroy every avalanche effect in the
/// static bank, install the base vtable and optionally free the object.
pub fn vfx_avalanche_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x79b8));
        frame.save(c, S0, 0x40);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x002d_63a8);
        Addr(this).sw(c, c.g[V0][0]);
        callee::AVALANCHE_DESTROY_ALL.call(c, 0x002d_63a8)?;
        c.g[V0][0] = 0x004f_0000;
        c.g[V1][0] = 0xbc00;
        c.g[S0][0] = sx(0x004f_0000u32.wrapping_sub(0x17c0));
        c.g[S2][0] = sx(r(c, S0).wrapping_add(r(c, V1)));
        c.g[A0][0] = c.g[S0][0];
        loop {
            c.cycles += 6;
            c.g[RA][0] = sx(0x002d_63c8);
            c.g[A1][0] = 1;
            call_ptr(c, callee::AVALANCHE_RESET_ONE, 0x002d_63c8)?;
            c.g[S0][0] = sx(r(c, S0).wrapping_add(0x2f0));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_63c0);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0x0049_0000;
        c.g[V1][0] = c.g[S3][0] & 1;
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x7980));
        let free_self = c.g[V1][0] == 0;
        Addr(this).sw(c, c.g[V0][0]);
        if !free_self {
            c.cycles += 2;
            c.g[RA][0] = sx(0x002d_63f4);
            c.g[A0][0] = c.g[S1][0];
            callee::BUILTIN_DELETE_2.call(c, 0x002d_63f4)?;
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

/// `Avalanche` fields used by the update.
mod av {
    /// The avalanche definition (0 = not playing): `+0xc` spawn key count,
    /// `+0xe` sound cue count, spawn keys at `+0x10` (8 bytes: time, info),
    /// sound cues at `+0x110` (4 bytes: halfword frame, halfword node).
    pub const DEF: u32 = 0x0;
    /// The live instances (linked at `+0x2e4`).
    pub const INSTANCES: u32 = 0x4;
    /// Node slots (one word per instance / node index).
    pub const NODES: u32 = 0x8;
    /// Cursors into the spawn keys and the sound cues.
    pub const SPAWN_CURSOR: u32 = 0x10;
    pub const CUE_CURSOR: u32 = 0x14;
    /// Playback time (s).
    pub const TIME: u32 = 0x18;
}

/// Direct callees of the avalanche update.
mod av_callee {
    use super::{ee, Callee};
    pub const UPDATE_INSTANCE: Callee = Callee {
        addr: 0x002d_7ca8,
        f: ee::s09::Avalanche_UpdateInstance,
    };
    pub const STOP: Callee = Callee {
        addr: 0x002d_81b0,
        f: ee::s09::Avalanche_Stop,
    };
    /// `EA_MemAllocNamed`, `AnimObject_ctor`, `SSXAudioSystem_GetInstance`,
    /// `SSXAudioSystem_AvalancheUpdateStub` (through the function table).
    pub const ALLOC: u32 = 0x0031_7d70;
    pub const ANIM_OBJECT_CTOR: u32 = 0x0034_1aa0;
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
    pub const AUDIO_AVALANCHE_STUB: u32 = 0x0029_e560;

    pub const BAKE_ALL: Callee = Callee {
        addr: 0x002d_96e0,
        f: ee::s09::f_002d96e0,
    };

    pub const APPLY_HISTORY_DATA: Callee = Callee {
        addr: 0x002d_a028,
        f: ee::s09::f_002da028,
    };

    pub const START: Callee = Callee {
        addr: 0x002d_97a8,
        f: ee::s09::f_002d97a8,
    };

    pub const UPDATE_ACTIVE: Callee = Callee {
        addr: 0x002d_7ef8,
        f: ee::s09::f_002d7ef8,
    };
}

/// `void Avalanche_UpdateActive(Avalanche* self)` (0x2d7ef8): one tick of a
/// playing avalanche. Advances the playback time by `gp+0x974` (0.009, a
/// debug tunable), updates every live instance at that time (clearing the
/// node slot of each one that ended), spawns the anim objects whose key time
/// has come (allocating one the first time a key has no object, after setting
/// up the shared spawn record at 0x4d5770 once), fires the sound cues whose
/// frame (`time * 30`) has come, and stops the avalanche when no instance is
/// alive any more.
///
/// Tick rate: the time step is per tick, so above 60 it is scaled by
/// `60/rate` at the add (the `.sdata` tunable itself is not touched): the
/// avalanche plays in the same wall time. Nothing changes at 60.
pub fn avalanche_update_active(c: &mut Cpu) {
    use av::*;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S1, 0x80);
        frame.save(c, S0, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 21, 0xa8);
        frame.save_fpr(c, 20, 0xa0);
        let this = r32(c, S1);
        c.g[V0][0] = Addr(this).off(DEF).lw(c);
        c.g[S5][0] = 0;
        if c.g[V0][0] != 0 {
            update_active_body(c, this)?;
            // Nothing alive any more: stop.
            c.cycles += 2;
            let alive = c.g[S5][0] != 0;
            frame.restore(c, S0, 0x90);
            if !alive {
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                av_callee::STOP.call(c, 0x002d_8180)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x90);
            }
        } else {
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 21, 0xa8);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// The time step, the instances, the spawn keys and the sound cues (up to
/// 0x2d8170).
fn update_active_body(c: &mut Cpu, this: u32) -> Flow {
    use av::*;
    // Advance the time; update the instances.
    c.cycles += 7;
    c.lwc1(0, this.wrapping_add(TIME));
    c.lwc1(1, gp(c, 0x974));
    // Tick rate: the step is per 60 Hz tick.
    crate::game::ai::rider::per_tick_60(c, 1);
    c.g[S2][0] = 0;
    c.g[S0][0] = Addr(this).off(INSTANCES).lw(c);
    c.add_s(0, 0, 1);
    c.swc1(0, this.wrapping_add(TIME));
    if c.g[S0][0] != 0 {
        loop {
            c.cycles += 6;
            c.lwc1(12, this.wrapping_add(TIME));
            c.g[A0][0] = c.g[S0][0];
            av_callee::UPDATE_INSTANCE.call(c, 0x002d_7f5c)?;
            c.g[V1][0] = (0 < c.g[V0][0]) as u64;
            c.g[S5][0] |= c.g[V1][0];
            if c.g[V0][0] == 0 {
                // Ended: clear its node slot.
                c.cycles += 4;
                c.g[V1][0] = Addr(this).off(NODES).lw(c);
                c.g[V0][0] = sx(r32(c, S2) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                Addr(r32(c, V0)).sw(c, 0);
            }
            c.cycles += 3;
            c.g[S0][0] = Addr(r32(c, S0)).off(0x2e4).lw(c);
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            if c.g[S0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_7f50);
            }
        }
    }

    // Spawn keys whose time has come.
    c.cycles += 6;
    c.g[A0][0] = Addr(this).off(DEF).lw(c);
    c.g[V1][0] = Addr(this).off(SPAWN_CURSOR).lw(c);
    c.g[V0][0] = Addr(r32(c, A0)).off(0xc).lhu(c);
    let more = (c.g[V1][0] as i64) < (c.g[V0][0] as i64);
    c.g[V0][0] = sx(r32(c, V1) << 3);
    if more {
        c.cycles += 7;
        c.lwc1(1, this.wrapping_add(TIME));
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.lwc1(0, r32(c, V0).wrapping_add(0x10));
        c.c_le_s(0, 1);
        let due = c.fcc();
        c.g[S4][0] = 0x4d_0000;
        if due {
            c.cycles += 8;
            c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
            c.mtc1(AT, 20);
            // 30.0 (0x2d7fc0): the spawn record's speed (+0x14), authored per
            // 1/30 s; AnimObject_ctor divides it by the app manager's rate
            // itself. Not a step: stays 30 at every rate.
            c.g[AT][0] = 0x41f0_0000; // 30.0
            c.mtc1(AT, 21);
            c.g[S0][0] = sx(r32(c, S4).wrapping_add(0x5770));
            c.g[S3][0] = 1;
            c.g[S6][0] = 0x48_0000;
            c.g[V0][0] = sx(r32(c, V1) << 3);
            loop {
                spawn_key(c, this)?;
                // Next key.
                c.g[V0][0] = Addr(this).off(SPAWN_CURSOR).lw(c);
                c.g[A0][0] = Addr(this).off(DEF).lw(c);
                c.g[V1][0] = sx(r32(c, V0).wrapping_add(1));
                Addr(this).off(SPAWN_CURSOR).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(r32(c, A0)).off(0xc).lhu(c);
                let more = (c.g[V1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = sx(r32(c, V1) << 3);
                if !more {
                    break;
                }
                c.cycles += 7;
                c.lwc1(0, this.wrapping_add(TIME));
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.lwc1(1, r32(c, V0).wrapping_add(0x10));
                c.c_le_s(1, 0);
                let due = c.fcc();
                c.g[V0][0] = sx(r32(c, V1) << 3);
                if !due {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_7fd8);
                }
            }
        }
    }

    // Sound cues whose frame (time * 30) has come.
    c.cycles += 6;
    c.g[A1][0] = Addr(this).off(DEF).lw(c);
    c.g[V1][0] = Addr(this).off(CUE_CURSOR).lw(c);
    // 30.0 (0x2d80e4): playback time -> the cues' authored 30 fps frame
    // numbers. Not a step: the time already advances in wall time (the step
    // above follows the rate), so the cues fire at the same moment.
    c.g[AT][0] = 0x41f0_0000; // 30.0
    c.mtc1(AT, 20);
    c.g[V0][0] = Addr(r32(c, A1)).off(0xe).lhu(c);
    loop {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            return Ok(());
        }
        c.cycles += 11;
        c.g[A0][0] = sx(r32(c, V1) << 2);
        c.lwc1(1, this.wrapping_add(TIME));
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        c.g[V1][0] = Addr(r32(c, V0)).off(0x110).lhu(c);
        c.mul_s(1, 1, 20);
        c.mtc1(V1, 0);
        c.cvt_s_w(0, 0);
        c.c_le_s(0, 1);
        if !c.fcc() {
            return Ok(());
        }
        c.g[V1][0] = Addr(r32(c, V0)).off(0x112).lhu(c);
        if event_due(c) {
            deliver_event_here(c, 0x002d_80f8);
        }
        // The cue's node, if it has one, plays its sound.
        c.cycles += 6;
        c.g[A0][0] = Addr(this).off(NODES).lw(c);
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        c.g[S0][0] = Addr(r32(c, V1)).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            call_ptr(c, av_callee::AUDIO_GET_INSTANCE, 0x002d_8118)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = sx(r32(c, S0).wrapping_add(0x60));
            call_ptr(c, av_callee::AUDIO_AVALANCHE_STUB, 0x002d_8124)?;
        }
        c.g[V0][0] = Addr(this).off(CUE_CURSOR).lw(c);
        c.cycles += 4;
        c.g[A1][0] = Addr(this).off(DEF).lw(c);
        c.g[V1][0] = sx(r32(c, V0).wrapping_add(1));
        Addr(this).off(CUE_CURSOR).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(r32(c, A1)).off(0xe).lhu(c);
    }
}

/// One due spawn key (`v0` = cursor * 8, `a0` = the definition): create its
/// anim object if it has none, then tell the audio system (0x2d7fd8 to
/// 0x2d80a0).
fn spawn_key(c: &mut Cpu, this: u32) -> Flow {
    use av::*;
    c.cycles += 5;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
    c.g[V1][0] = Addr(r32(c, V0)).off(0x14).lw(c);
    c.g[A0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
    let has_object = c.g[A0][0] != 0;
    c.g[V0][0] = Addr(gp(c, 0x23d8)).lw(c);
    if !has_object {
        c.cycles += 2;
        let ready = c.g[V0][0] != 0;
        c.g[S2][0] = 0x4d_0000;
        if !ready {
            // The shared spawn record at 0x4d5770, once.
            c.cycles += 16;
            let rec = r32(c, S0);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[V1][0] = Addr(rec).off(0x28).lbu(c);
            Addr(rec).off(0x4).sw(c, c.g[S3][0]);
            c.g[V0][0] |= 0xffff;
            Addr(r32(c, S4)).off(0x5770).sw(c, c.g[V0][0]);
            Addr(rec).off(0x28).sb(c, c.g[V1][0]);
            Addr(rec).off(0x8).sw(c, 0);
            c.swc1(20, rec.wrapping_add(0xc));
            c.swc1(20, rec.wrapping_add(0x10));
            c.swc1(21, rec.wrapping_add(0x14));
            Addr(rec).off(0x18).sw(c, 0);
            c.swc1(20, rec.wrapping_add(0x1c));
            Addr(rec).off(0x20).sw(c, 0);
            Addr(rec).off(0x24).sw(c, c.g[S3][0]);
            Addr(sp(c)).sb(c, c.g[V1][0]);
            Addr(gp(c, 0x23d8)).sw(c, c.g[S3][0]);
        }
        c.cycles += 15;
        c.g[A0][0] = 0x6c;
        c.g[A1][0] = sx(r32(c, S6).wrapping_add(0x7558));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, av_callee::ALLOC, 0x002d_8048)?;
        c.g[V1][0] = Addr(this).off(SPAWN_CURSOR).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = Addr(this).off(DEF).lw(c);
        c.g[T0][0] = sx(r32(c, S2).wrapping_add(0x5770));
        c.g[V1][0] = sx(r32(c, V1) << 3);
        c.g[A1][0] = 1;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[A2][0] = 1;
        c.g[A3][0] = Addr(r32(c, V0)).off(0x14).lw(c);
        call_ptr(c, av_callee::ANIM_OBJECT_CTOR, 0x002d_8070)?;
    }
    c.cycles += 20;
    call_ptr(c, av_callee::AUDIO_GET_INSTANCE, 0x002d_8078)?;
    c.g[A3][0] = Addr(this).off(SPAWN_CURSOR).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = Addr(this).off(DEF).lw(c);
    c.g[A1][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A3][0] = sx(r32(c, A3) << 3);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, A3)));
    c.g[V1][0] = Addr(r32(c, A2)).off(0x14).lw(c);
    c.lq(V0, r32(c, V1).wrapping_add(0x40));
    c.sq(V0, sp(c).wrapping_add(0x10));
    call_ptr(c, av_callee::AUDIO_AVALANCHE_STUB, 0x002d_80a0)
}

/// `void Avalanche_Stop(self)` (0x2d81b0): free the spawn buffer, release
/// every active instance and tell the weather manager the avalanche ended.
pub fn avalanche_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(r32(c, S1), 8)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x002d81d8)?;
            Addr(p(r32(c, S1), 8)).sw(c, 0);
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        }
        c.cycles += 2;
        let done = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !done {
            c.cycles += 3;
            c.g[S0][0] = Addr(p(r32(c, S1), 4)).lw(c);
            let empty = c.g[S0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !empty {
                c.cycles += 1;
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x002d_7dd8, 0x002d8200)?;
                    c.g[S0][0] = Addr(p(r32(c, S0), 0x2e4)).lw(c);
                    let more = c.g[S0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_81f8);
                    }
                }
            }
            c.cycles += 12;
            Addr(r32(c, S1)).sw(c, 0);
            call_ptr(c, 0x0028_b180, 0x002d8214)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0029_def0, 0x002d8220)?;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0xc)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A0), 0xa8)).lw(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x708)).lw(c);
            let zero = c.g[V0][0] == 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            if !zero {
                c.cycles += 1;
                Addr(p(r32(c, V1), 0x708)).sw(c, c.g[V0][0]);
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Avalanche_GetObjectMatrix` (0x2d9c00): find the avalanche node whose object
// id matches, copy its rotation matrix, scale it and store its position in
// the output matrix.
pub fn avalanche_get_object_matrix(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x4f0000;
    c.g[V1][0] = 0xbc00;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1890));
    c.g[A2][0] = c.g[V0][0];
    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70));
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x2e0));
        let matched = if c.g[V0][0] == 0 {
            false
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xf8));
            c.g[V0][0] == c.g[A0][0]
        };
        if matched {
            c.cycles += 26;
            c.lqc2(1, c.g[V1][0] as u32);
            c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
            c.sqc2(1, c.g[A1][0] as u32);
            c.sqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.sqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
            c.sqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0xb0));
            c.mfc1(3, 0);
            c.lqc2(4, c.g[A1][0] as u32);
            c.qmtc2(3, 3);
            c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.lqc2(6, (c.g[A1][0] as u32).wrapping_add(0x20));
            c.lqc2(7, (c.g[A1][0] as u32).wrapping_add(0x30));
            c.vu0().mul(0b1111, 8, 4, Src::Bc(3, X));
            c.vu0().mul(0b1111, 9, 5, Src::Bc(3, X));
            c.vu0().mul(0b1111, 10, 6, Src::Bc(3, X));
            c.vu0().mul(0b1111, 11, 7, Src::Bc(3, X));
            c.sqc2(8, c.g[A1][0] as u32);
            c.sqc2(9, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.sqc2(10, (c.g[A1][0] as u32).wrapping_add(0x20));
            c.sqc2(11, (c.g[A1][0] as u32).wrapping_add(0x30));
            c.lq(2, (c.g[A2][0] as u32).wrapping_add(0x60));
            c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x30));
            abi::ret(c);
            return;
        }
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f0));
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2f0));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002d_9c18);
        }
    }
}

// ---------------------------------------------------------------------------
// `Avalanche_FreeHistories` (0x002d9a80).
//
// `void Avalanche::FreeHistories()`: stop every avalanche that has a history
// and release the history links.

pub fn avalanche_free_histories(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = 0x540000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x76c8));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[V0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c0));
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        'l_2d9aa8: loop {
            'b_2d9abc: {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
                    break 'b_2d9abc;
                } else {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x002d9ab8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002d81b0, 0x002d9ab8)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
                    break 'b_2d9abc;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x002d9aa8);
                }
                continue 'l_2d9aa8;
            } else {
                'b_2d9b30: {
                    'b_2d9b2c: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c8));
                        let t = c.g[V0][0] == 0u64;
                        c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
                        if t {
                            break 'b_2d9b2c;
                        } else {
                            c.cycles += 2;
                            let t = c.g[S1][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_2d9b30;
                            } else {
                                c.cycles += 1;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                'l_2d9ae0: loop {
                                    'b_2d9b24: {
                                        c.cycles += 2;
                                        if c.g[S0][0] == 0u64 {
                                            c.g[S1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            break 'b_2d9b24;
                                        } else {
                                            c.cycles += 2;
                                            c.g[V0][0] =
                                                c.lbu((c.g[S0][0] as u32).wrapping_add(0xf3));
                                            'l_2d9af0: loop {
                                                'b_2d9b10: {
                                                    c.cycles += 2;
                                                    if c.g[V0][0] == 0u64 {
                                                        c.sb(
                                                            (c.g[S0][0] as u32).wrapping_add(0xf3),
                                                            0u64,
                                                        );
                                                        break 'b_2d9b10;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xfc));
                                                        if c.g[A0][0] == 0u64 {
                                                            c.sb(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0xf3),
                                                                0u64,
                                                            );
                                                            break 'b_2d9b10;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[RA][0] = sx(0x002d9b0c);
                                                            call_ptr(c, 0x00317e98, 0x002d9b0c)?;
                                                            c.sb(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0xf3),
                                                                0u64,
                                                            );
                                                            break 'b_2d9b10;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.sw((c.g[S0][0] as u32).wrapping_add(0xfc), 0u64);
                                                c.g[S0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xf4));
                                                if c.g[S0][0] != 0u64 {
                                                    c.g[V0][0] = c.lbu(
                                                        (c.g[S0][0] as u32).wrapping_add(0xf3),
                                                    );
                                                    if crate::events::event_due(c) {
                                                        crate::events::deliver_event_here(
                                                            c, 0x002d9af0,
                                                        );
                                                    }
                                                    continue 'l_2d9af0;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[S1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    break 'b_2d9b24;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    if c.g[S1][0] != 0u64 {
                                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        if crate::events::event_due(c) {
                                            crate::events::deliver_event_here(c, 0x002d9ae0);
                                        }
                                        continue 'l_2d9ae0;
                                    } else {
                                        break 'b_2d9b2c;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    break 'b_2d9b30;
                }
                c.cycles += 4;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `void VfxAvalanche_Init(void)` (0x2d6410): clear the 2048 avalanche
/// particles and the 32 trail entries.
pub fn vfx_avalanche_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = 0x4d0000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b70));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0xff;
        'l_2d6430: loop {
            c.cycles += 15;
            c.g[V0][0] = c.g[A1][0] & 0x1;
            c.g[V1][0] = c.g[A2][0];
            if c.g[V0][0] == 0 {
                c.g[V1][0] = 0u64;
            }
            c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.g[V0][0] = ((c.g[A1][0] as i64) < 2048i64) as u64;
            c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
            c.sw(c.g[A0][0] as u32, 0u64);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x4), 0u64);
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002d6430);
                }
                continue 'l_2d6430;
            } else {
                c.cycles += 7;
                c.g[V0][0] = 0x4f0000;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2490));
                c.g[A0][0] = 0xff;
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x600));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                'l_2d6488: loop {
                    c.cycles += 11;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), 0u64);
                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                        if event_due(c) {
                            deliver_event_here(c, 0x002d6488);
                        }
                        continue 'l_2d6488;
                    } else {
                        c.cycles += 9;
                        c.g[A0][0] = 0x540000;
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7bb0));
                        c.g[A2][0] = 0x400;
                        c.g[RA][0] = sx(0x002d64cc);
                        c.sw((c.g[GP][0] as u32).wrapping_add(0x9d0), 0u64);
                        call_ptr(c, 0x00416210, 0x002d64cc)?;
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `void Avalanche_ClampVector(void* self, Vec4* v)` (0x2d64d8): clamp the
/// vector to the avalanche speed limit (`D_004A3A94`); when any component is
/// a PS2 inf/NaN, replace the vector with (10, 10, 10, 0).
pub fn avalanche_clamp_vector(c: &mut Cpu) {
    'b_2d6568: {
        c.cycles += 15;
        c.lwc1(0, c.pa(GP, 0x9a4));
        c.lqc2(3, c.pa(A1, 0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 1);
        c.c_lt_s(0, 1);
        let t = !c.fcc();
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        if t {
            break 'b_2d6568;
        } else {
            c.cycles += 21;
            c.lqc2(3, c.pa(A1, 0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, c.pa(SP, 0x20));
            c.lq(2, c.pa(SP, 0x20));
            c.lwc1(0, c.pa(GP, 0x9a4));
            c.sq(2, c.pa(SP, 0x10));
            c.mfc1(3, 0);
            c.lqc2(4, c.pa(SP, 0x10));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, c.pa(SP, 0x20));
            c.lq(2, c.pa(SP, 0x20));
            c.sq(2, c.pa(A1, 0));
            c.sq(2, c.pa(SP, 0));
            break 'b_2d6568;
        }
    }
    'b_2d65a4: {
        'b_2d65a0: {
            c.cycles += 5;
            c.g[V1][0] = c.ldw_2(A1, 0);
            c.g[A2][0] = 0u64;
            c.g[V0][0] = 0x7f800000;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = 0u64;
            if t {
                break 'b_2d65a0;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x7fc00000;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[A0][0] = 0x1;
                    break 'b_2d65a4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffffffffff800000;
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[A0][0] = 0x1;
                        break 'b_2d65a4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xffffffffffc00000;
                        let t = c.g[V1][0] != c.g[V0][0];

                        if t {
                            break 'b_2d65a4;
                        } else {
                            break 'b_2d65a0;
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[A0][0] = 0x1;
        break 'b_2d65a4;
    }
    'b_2d666c: {
        c.cycles += 2;
        if c.g[A0][0] != 0u64 {
            c.g[A2][0] = 0x1;
            break 'b_2d666c;
        } else {
            'b_2d65e4: {
                'b_2d65e0: {
                    c.cycles += 4;
                    c.g[V1][0] = c.ldw_2(A1, 0x4);
                    c.g[V0][0] = 0x7f800000;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0u64;
                    if t {
                        break 'b_2d65e0;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x7fc00000;
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[A0][0] = 0x1;
                            break 'b_2d65e4;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0xffffffffff800000;
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[A0][0] = 0x1;
                                break 'b_2d65e4;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0xffffffffffc00000;
                                let t = c.g[V1][0] != c.g[V0][0];

                                if t {
                                    break 'b_2d65e4;
                                } else {
                                    break 'b_2d65e0;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = 0x1;
                break 'b_2d65e4;
            }
            c.cycles += 2;
            if c.g[A0][0] != 0u64 {
                c.g[A2][0] = 0x1;
                break 'b_2d666c;
            } else {
                'b_2d6624: {
                    'b_2d6620: {
                        c.cycles += 4;
                        c.g[V1][0] = c.ldw_2(A1, 0x8);
                        c.g[V0][0] = 0x7f800000;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[A0][0] = 0u64;
                        if t {
                            break 'b_2d6620;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x7fc00000;
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[A0][0] = 0x1;
                                break 'b_2d6624;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0xffffffffff800000;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[A0][0] = 0x1;
                                    break 'b_2d6624;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0xffffffffffc00000;
                                    let t = c.g[V1][0] != c.g[V0][0];

                                    if t {
                                        break 'b_2d6624;
                                    } else {
                                        break 'b_2d6620;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = 0x1;
                    break 'b_2d6624;
                }
                c.cycles += 2;
                if c.g[A0][0] != 0u64 {
                    c.g[A2][0] = 0x1;
                    break 'b_2d666c;
                } else {
                    'b_2d6664: {
                        'b_2d6660: {
                            c.cycles += 4;
                            c.g[V1][0] = c.ldw_2(A1, 0xc);
                            c.g[V0][0] = 0x7f800000;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[A0][0] = 0u64;
                            if t {
                                break 'b_2d6660;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0x7fc00000;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[A0][0] = 0x1;
                                    break 'b_2d6664;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0xffffffffff800000;
                                    if c.g[V1][0] == c.g[V0][0] {
                                        c.g[A0][0] = 0x1;
                                        break 'b_2d6664;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0xffffffffffc00000;
                                        let t = c.g[V1][0] != c.g[V0][0];

                                        if t {
                                            break 'b_2d6664;
                                        } else {
                                            break 'b_2d6660;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = 0x1;
                        break 'b_2d6664;
                    }
                    c.cycles += 2;
                    if c.g[A0][0] != 0u64 {
                        c.g[A2][0] = 0x1;
                        break 'b_2d666c;
                    } else {
                        break 'b_2d666c;
                    }
                }
            }
        }
    }
    'b_2d6694: {
        c.cycles += 2;
        let t = c.g[A2][0] == 0u64;

        if t {
            break 'b_2d6694;
        } else {
            c.cycles += 8;
            c.g[AT][0] = 0x41200000;
            c.mtc1(1, 0);
            c.stw_2(SP, 0xc, 0u64);
            c.swc1(0, c.pa(SP, 0x8));
            c.swc1(0, c.pa(SP, 0));
            c.swc1(0, c.pa(SP, 0x4));
            c.lq(2, c.pa(SP, 0));
            c.sq(2, c.pa(A1, 0));
            break 'b_2d6694;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = ra;
    return;
}

pub(crate) mod avalanche {

    /// `gp+0x9c0` the first group, `gp+0x9c4` the loaded history data,
    /// `gp+0x9cc` the data was applied.
    pub const GP_GROUPS: u32 = 0x9c0;

    pub const GP_DATA: u32 = 0x9c4;

    pub const GP_APPLIED: u32 = 0x9cc;

    /// Group: `+0x4` first node, `+0x8` next group, `+0xc` pair count
    /// (`u16`, zeroed after), `+0xe` index count, `+0x10` pairs (`{f32
    /// v, u32 0}`), `+0x110` index pairs (`{u16 b, u16 a}`).
    pub const GROUP_NODES: u32 = 0x4;

    pub const GROUP_NEXT: u32 = 0x8;

    pub const GROUP_PAIRS: u32 = 0xc;

    pub const GROUP_INDICES: u32 = 0xe;

    pub const GROUP_PAIR_ARRAY: u32 = 0x10;

    pub const GROUP_INDEX_ARRAY: u32 = 0x110;

    /// Node: `+0xe0` position, `+0xf3` owns its data, `+0xf4` next node,
    /// `+0xfc` data, `+0x100` data count (float; 10 bytes per entry).
    pub const NODE_POS: u32 = 0xe0;

    pub const NODE_OWNED: u32 = 0xf3;

    pub const NODE_NEXT: u32 = 0xf4;

    pub const NODE_DATA: u32 = 0xfc;

    pub const NODE_COUNT: u32 = 0x100;

    pub const ENTRY_SIZE: u64 = 0xa;

    /// Loop heads (event checks at their back edges).
    pub const HEAD_GROUPS: u32 = 0x002d_a070;

    pub const HEAD_NODES: u32 = 0x002d_a088;

    pub const HEAD_PAIRS: u32 = 0x002d_a120;

    pub const HEAD_INDICES: u32 = 0x002d_a158;
}

/// `cMemMan_free(p)` (0x317e98, other shard).
pub(crate) const CMEMMAN_FREE: u32 = 0x0031_7e98;

/// `addiu reg, reg, n`.
pub(crate) fn bump(c: &mut Cpu, reg: usize, n: u32) -> u32 {
    let v = (c.g[reg][0] as u32).wrapping_add(n);
    c.g[reg][0] = sx(v);
    v
}

/// `void cAvalanche::resolveDataPointers()` (0x2da028, proposed
/// `Avalanche_ApplyHistoryData`): once, when there are groups and loaded
/// history data, walk the data alongside the groups: each node takes its
/// position (w = 1) and points at its slice of the data (freeing data it
/// owned), each group reads its pairs (value, 0) and index pairs; then the
/// data counts as applied.
pub fn avalanche_apply_history_data(c: &mut Cpu) {
    use avalanche::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x70);
        let gp = c.g[GP][0] as u32;
        let groups = c.lw(gp.wrapping_add(GP_GROUPS));
        c.g[A0][0] = groups;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 20, 0x60);
        'body: {
            if groups == 0 {
                break 'body;
            }
            c.cycles += 3;
            let data = c.lw(gp.wrapping_add(GP_DATA));
            c.g[V1][0] = data;
            let applied = c.lw(gp.wrapping_add(GP_APPLIED));
            c.g[V0][0] = applied;
            if data == 0 {
                break 'body;
            }
            c.cycles += 2;
            frame.restore(c, S0, 0x50);
            if applied != 0 {
                return epilogue(c, frame);
            }
            c.cycles += 4;
            c.g[S0][0] = sx((data as u32).wrapping_add(8));
            c.g[S2][0] = groups;
            c.g[S1][0] = c.lw((groups as u32).wrapping_add(GROUP_NODES));
            loop {
                // Group loop head: the nodes.
                c.cycles += 2;
                if c.g[S1][0] != 0 {
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 20);
                    c.g[S3][0] = ENTRY_SIZE;
                    loop {
                        // Node loop head: the position (x, y, z, 1).
                        c.cycles += 13;
                        let p = bump(c, S0, 4);
                        c.lwc1(0, p);
                        c.lwc1(1, p.wrapping_add(4));
                        c.lwc1(2, p.wrapping_add(8));
                        let s = sp(c);
                        c.swc1(0, s);
                        bump(c, S0, 0xc);
                        c.swc1(1, s.wrapping_add(4));
                        c.swc1(2, s.wrapping_add(8));
                        c.swc1(20, s.wrapping_add(0xc));
                        c.lq(V0, s);
                        let node = c.g[S1][0] as u32;
                        let owned = c.lbu(node.wrapping_add(NODE_OWNED));
                        c.g[V1][0] = owned;
                        c.sq(V0, node.wrapping_add(NODE_POS));
                        if owned == 0 {
                            c.cycles += 1;
                        } else {
                            c.cycles += 3;
                            let old = c.lw(node.wrapping_add(NODE_DATA));
                            c.g[A0][0] = old;
                            if old != 0 {
                                c.cycles += 2;
                                call_ptr(c, CMEMMAN_FREE, 0x002d_a0d0)?;
                                c.cycles += 1;
                            }
                        }
                        c.lwc1(0, node.wrapping_add(NODE_COUNT));
                        // The node's data slice; the next node.
                        c.cycles += 8;
                        c.sw(node.wrapping_add(NODE_DATA), c.g[S0][0]);
                        c.cvt_w_s(1, 0);
                        c.mfc1(V0, 1);
                        c.sb(node.wrapping_add(NODE_OWNED), 0);
                        c.g[S1][0] = c.lw(node.wrapping_add(NODE_NEXT));
                        c.mult(0, V1, V0, S3);
                        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        if c.g[S1][0] == 0 {
                            c.cycles += 1;
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, HEAD_NODES);
                        }
                    }
                }
                c.g[V0][0] = c.lhu(c.g[S0][0] as u32);

                // The group's counts and pairs.
                let group = c.g[S2][0] as u32;
                c.cycles += 8;
                c.g[A0][0] = 0;
                bump(c, S0, 2);
                c.sh(group.wrapping_add(GROUP_PAIRS), c.g[V0][0]);
                c.g[V1][0] = c.lhu(c.g[S0][0] as u32);
                c.g[V0][0] &= 0xffff;
                bump(c, S0, 2);
                c.sh(group.wrapping_add(GROUP_INDICES), c.g[V1][0]);
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V1][0] = sx(group.wrapping_add(GROUP_PAIR_ARRAY + 4));
                    loop {
                        c.cycles += 9;
                        c.lwc1(0, c.g[S0][0] as u32);
                        bump(c, A0, 1);
                        let pair = c.g[V1][0] as u32;
                        c.sw(pair, 0);
                        bump(c, S0, 8);
                        c.swc1(0, pair.wrapping_sub(4));
                        let n = c.lhu(group.wrapping_add(GROUP_PAIRS));
                        let more = ((c.g[A0][0] as i64) < (n as i64)) as u64;
                        c.g[V0][0] = more;
                        bump(c, V1, 8);
                        if more == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, HEAD_PAIRS);
                        }
                    }
                }

                // The index pairs (stored swapped).
                c.cycles += 3;
                c.g[V0][0] = c.lhu(group.wrapping_add(GROUP_INDICES));
                c.g[A1][0] = 0;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[A0][0] = sx(group.wrapping_add(GROUP_INDEX_ARRAY));
                    loop {
                        c.cycles += 11;
                        let at = c.g[A0][0] as u32;
                        c.g[V1][0] = c.lhu(c.g[S0][0] as u32);
                        bump(c, A1, 1);
                        bump(c, S0, 2);
                        c.sh(at.wrapping_add(2), c.g[V1][0]);
                        c.g[V0][0] = c.lhu(c.g[S0][0] as u32);
                        bump(c, S0, 2);
                        c.sh(at, c.g[V0][0]);
                        let n = c.lhu(group.wrapping_add(GROUP_INDICES));
                        let more = ((c.g[A1][0] as i64) < (n as i64)) as u64;
                        c.g[V0][0] = more;
                        bump(c, A0, 4);
                        if more == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, HEAD_INDICES);
                        }
                    }
                }

                // Next group.
                c.cycles += 4;
                c.sh(group.wrapping_add(GROUP_PAIRS), 0);
                let next = c.lw(group.wrapping_add(GROUP_NEXT));
                c.g[S2][0] = next;
                if next == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    c.sw(gp.wrapping_add(GP_APPLIED), 1);
                    break 'body;
                }
                c.g[S1][0] = c.lw((next as u32).wrapping_add(GROUP_NODES));
                if event_due(c) {
                    deliver_event_here(c, HEAD_GROUPS);
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x50);
        epilogue(c, frame)
    });

    fn epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    }
}

// --------------------------------------------------- VfxAvalanche_Render

/// `void VfxAvalanche_Render(void)` (0x2d9130): draw the avalanche effect.
/// Runs when the avalanche is on (gp+0x940) and visible (gp+0x9cc), or when
/// forced (gp+0x10fc). Pushes the graphics manager's render state (a copy
/// of the top, then the avalanche template at 0x501420, blend bits 7, no
/// texture), sets the current camera's (world +0x84 -> +0x84, index +0x14)
/// matrix, position and rotation through the manager's vtable (+0x10d8:
/// slots +0xc0, +0xc8, +0x110), renders the 16 avalanche slots (0x538938,
/// 0x1c bytes each) while the avalanche is on, and pops the state.
pub fn vfx_avalanche_render(c: &mut Cpu) {
    const ON: u32 = 0x940;
    const VISIBLE: u32 = 0x9cc;
    const FORCED: u32 = 0x10fc;
    /// gp-0x848: the world (app) pointer.
    const WORLD: u32 = 0x848;
    const GFX_MAN: u32 = 0x2a90;
    const STATE_TOP: u32 = 0xe84;
    const VTABLE: u32 = 0x10d8;
    const STATE_SIZE: u32 = 0x14;
    /// The avalanche's render-state template.
    const TEMPLATE: u32 = 0x0050_1420;
    const SLOTS: u32 = 0x0053_8938;
    const SLOT_SIZE: u32 = 0x1c;
    const SLOT_COUNT: u32 = 16;
    const RENDER_ACTIVE: Callee = Callee {
        addr: 0x002d_8ea8,
        f: ee::s09::Avalanche_RenderActive,
    };

    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = Addr(gp.wrapping_add(ON)).lw(c);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let mut draw = false;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            draw = Addr(gp.wrapping_add(VISIBLE)).lw(c) != 0;
            c.g[V0][0] = Addr(gp.wrapping_sub(WORLD)).lw(c);
        }
        if !draw {
            c.cycles += 3;
            draw = Addr(gp.wrapping_add(FORCED)).lw(c) != 0;
            c.g[V0][0] = Addr(gp.wrapping_sub(WORLD)).lw(c);
        }
        if draw {
            c.cycles += 72;
            c.g[A3][0] = 0x50_0000;
            let gfx = Addr(gp.wrapping_add(GFX_MAN)).lw(c);
            c.g[S1][0] = gfx;
            let gfx = gfx as u32;
            c.g[T0][0] = !0x3e0;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
            c.g[T1][0] = u64::MAX;
            let top = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.g[A1][0] = top;
            let top = top as u32;
            c.g[A2][0] = 1;
            // The current camera: views[cur] (s0).
            c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x84)).lw(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[S0][0] = Addr(r32(c, V1).wrapping_add(0x4)).lw(c);
            // Push a copy of the top.
            c.ldl(V0, top.wrapping_add(0x7));
            c.ldr(V0, top);
            c.ldl(V1, top.wrapping_add(0xf));
            c.ldr(V1, top.wrapping_add(0x8));
            c.g[A0][0] = Addr(top.wrapping_add(0x10)).lw(c);
            c.sdl(V0, top.wrapping_add(0x1b));
            c.sdr(V0, top.wrapping_add(0x14));
            c.sdl(V1, top.wrapping_add(0x23));
            c.sdr(V1, top.wrapping_add(0x1c));
            c.sw(top.wrapping_add(0x24), c.g[A0][0]);
            c.g[V1][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(STATE_SIZE));
            c.sw(gfx.wrapping_add(STATE_TOP), c.g[V0][0]);
            // ... and overwrite the new top with the template.
            c.g[T2][0] = sx(TEMPLATE);
            let old = r32(c, V1);
            c.ldl(V0, TEMPLATE + 0x7);
            c.ldr(V0, TEMPLATE);
            c.ldl(A0, TEMPLATE + 0xf);
            c.ldr(A0, TEMPLATE + 0x8);
            c.g[A1][0] = Addr(TEMPLATE + 0x10).lw(c);
            c.sdl(V0, old.wrapping_add(0x1b));
            c.sdr(V0, old.wrapping_add(0x14));
            c.sdl(A0, old.wrapping_add(0x23));
            c.sdr(A0, old.wrapping_add(0x1c));
            c.sw(old.wrapping_add(0x24), c.g[A1][0]);
            // Blend bits 7, texture -1.
            c.g[A0][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            let flags = r32(c, A0).wrapping_add(0x8);
            c.g[V0][0] = (Addr(flags).lw(c) & c.g[T0][0]) | 0xe0;
            c.sw(flags, c.g[V0][0]);
            c.g[V1][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.sh(r32(c, V1).wrapping_add(0x10), c.g[T1][0]);

            // setMatrix(&cam->mats[cur], 1).
            c.g[V0][0] = Addr(gp.wrapping_sub(WORLD)).lw(c);
            c.g[A1][0] = Addr(gfx.wrapping_add(VTABLE)).lw(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
            c.g[A3][0] = Addr(r32(c, A1).wrapping_add(0xc4)).lw(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
            c.g[A0][0] = Addr(r32(c, A1).wrapping_add(0xc0)).lh(c);
            c.g[A1][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
            c.g[A0][0] = sx(gfx.wrapping_add(r32(c, A0)));
            c.g[A1][0] = sx(r32(c, A1) << 4);
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x18));
            c.g[A1][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
            call_ptr(c, r32(c, A3), 0x002d_9240)?;

            // setPos(view[0..3]).
            let view = Addr(r32(c, S0));
            c.g[V0][0] = Addr(gfx.wrapping_add(VTABLE)).lw(c);
            c.f[12] = view.read_fpu(c).0;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc8)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xcc)).lw(c);
            c.f[13] = view.off(0x4).read_fpu(c).0;
            c.g[A0][0] = sx(gfx.wrapping_add(r32(c, A0)));
            c.f[14] = view.off(0x8).read_fpu(c).0;
            call_ptr(c, r32(c, V1), 0x002d_9260)?;

            // setRot(view + 0x10 floats).
            c.g[V0][0] = Addr(gfx.wrapping_add(VTABLE)).lw(c);
            c.g[A1][0] = sx(view.0.wrapping_add(0x40));
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x110)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x114)).lw(c);
            c.g[A0][0] = sx(gfx.wrapping_add(r32(c, A0)));
            call_ptr(c, r32(c, V1), 0x002d_9278)?;

            let on = Addr(gp.wrapping_add(ON)).lw(c) != 0;
            c.g[V0][0] = 0x54_0000;
            if on {
                c.cycles += 3;
                c.g[S0][0] = sx(SLOTS);
                c.g[S2][0] = sx(SLOTS + SLOT_SIZE * SLOT_COUNT);
                loop {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    RENDER_ACTIVE.call(c, 0x002d_9298)?;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(SLOT_SIZE));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_9290);
                    }
                }
            }
            // Pop.
            c.cycles += 3;
            c.g[V0][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(STATE_SIZE));
            c.sw(gfx.wrapping_add(STATE_TOP), c.g[V0][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod avg {

    /// Avalanches enabled.
    pub const ENABLED: i32 = 0x940;

    /// Bake requested (re-bake every history at the next update).
    pub const BAKE_PENDING: i32 = 0x944;

    /// Start requested (`< 0`: one-shot, cleared after the start).
    pub const START_PENDING: i32 = 0x96c;

    /// Update active instances (`< 0`: one-shot, cleared after the pass).
    pub const UPDATE_PENDING: i32 = 0x970;

    /// The avalanche to start.
    pub const START_ID: i32 = 0x98c;

    /// Has data (avalanche histories loaded).
    pub const HAVE_DATA: i32 = 0x9cc;

    /// Cleared together with the bake request.
    pub const BAKE_CLEAR: i32 = 0x9d4;

    /// The 16 active avalanches (0x1c bytes each) at 0x538938.
    pub const ACTIVE: u32 = 0x0054_0000 - 0x76c8;

    pub const ACTIVE_SIZE: u32 = 0x1c;

    pub const ACTIVE_COUNT: u32 = 16;
}

/// `void VfxAvalanche_Update(void)` (0x2d8948): the avalanche manager's per
/// frame update. When enabled: runs a pending bake (`Avalanche_BakeAll`),
/// re-applies the history data, starts a requested avalanche, and updates
/// the 16 active avalanches when histories are loaded and an update is
/// requested. Negative (one-shot) requests are cleared after use.
pub fn vfx_avalanche_update(c: &mut Cpu) {
    use avg::*;
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = gp_at(c, ENABLED).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        if c.g[V0][0] != 0 {
            update_enabled(c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The enabled part of `VfxAvalanche_Update` (up to the epilogue).
pub(crate) fn update_enabled(c: &mut Cpu) -> call::Flow {
    use avg::*;
    c.cycles += 3;
    c.g[V0][0] = gp_at(c, BAKE_PENDING).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        av_callee::BAKE_ALL.call(c, 0x002d_8974)?;
        gp_at(c, BAKE_PENDING).sw(c, 0);
        gp_at(c, BAKE_CLEAR).sw(c, 0);
    }
    c.cycles += 5;
    av_callee::APPLY_HISTORY_DATA.call(c, 0x002d_8984)?;
    let start = gp_at(c, START_PENDING).lw(c);
    c.g[V0][0] = gp_at(c, ENABLED).lw(c);
    if start != 0 {
        c.cycles += 5;
        c.g[A0][0] = gp_at(c, START_ID).lw(c);
        av_callee::START.call(c, 0x002d_8998)?;
        c.g[V1][0] = gp_at(c, START_PENDING).lw(c);
        if (c.g[V1][0] as i64) < 0 {
            gp_at(c, START_PENDING).sw(c, 0);
        }
        c.cycles += 1;
        c.g[V0][0] = gp_at(c, ENABLED).lw(c);
    }
    // Still enabled, histories loaded, update requested.
    c.cycles += 2;
    let enabled = c.g[V0][0] != 0;
    c.g[V0][0] = gp_at(c, HAVE_DATA).lw(c);
    if !enabled {
        return Ok(());
    }
    c.cycles += 2;
    let have_data = c.g[V0][0] != 0;
    c.g[V0][0] = gp_at(c, UPDATE_PENDING).lw(c);
    if !have_data {
        return Ok(());
    }
    c.cycles += 2;
    let update = c.g[V0][0] != 0;
    c.g[V0][0] = 0x0054_0000;
    if !update {
        return Ok(());
    }
    c.cycles += 2;
    c.g[S0][0] = sx(ACTIVE);
    c.g[S1][0] = sx(ACTIVE + ACTIVE_COUNT * ACTIVE_SIZE);
    loop {
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        av_callee::UPDATE_ACTIVE.call(c, 0x002d_89d0)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(ACTIVE_SIZE));
        let more = (c.g[S0][0] as i64) < (c.g[S1][0] as i64);
        c.g[V0][0] = gp_at(c, UPDATE_PENDING).lw(c);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002d_89c8);
        }
    }
    c.cycles += 2;
    if (c.g[V0][0] as i64) < 0 {
        gp_at(c, UPDATE_PENDING).sw(c, 0);
    }
    Ok(())
}

/// `void Avalanche_SaveState(cPbStream* s)` (0x2d9cb0): writes the state of
/// the 16 avalanche slots (table at 0x5389c0, 0x1c bytes each) as one
/// 0x80-byte record: the definition index of each slot (`*slot->def`, -1
/// when idle), then each slot's playback time (+0x18; 0.0 when idle)
/// (`s->Write(&rec, 0x80)`, vtable +8).
pub fn avalanche_save_state(c: &mut Cpu) {
    const SLOTS: u32 = 0x0054_0000 - 0x76c8;
    const SLOT_SIZE: u64 = 0x1c;
    const SLOT_TIME: u32 = 0x18;
    const SLOT_COUNT: i64 = 16;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = sx(SLOTS);
        frame.save_ra(c, 0x80);
        c.g[T1][0] = sx(SLOTS);
        c.g[T2][0] = c.g[A0][0];
        c.g[A3][0] = 0;
        c.g[T3][0] = sx(SLOTS + SLOT_TIME);
        c.g[T4][0] = sx(SLOTS);
        c.g[T0][0] = sx(frame.at(c, 0x40));
        c.g[A2][0] = c.g[SP][0];
        loop {
            // The definition index (-1: idle slot).
            c.cycles += 3;
            let def = c.lw(c.g[T1][0] as u32);
            let index = if def == 0 {
                u64::MAX
            } else {
                c.cycles += 1;
                c.lw(def as u32)
            };
            c.g[V0][0] = index;
            c.cycles += 7;
            c.sw(c.g[A2][0] as u32, index);
            c.g[A1][0] = SLOT_SIZE;
            let i = c.g[A3][0] as u32;
            let off = abi::mult(c, i, SLOT_SIZE as u32);
            c.g[V0][0] = sx(off);
            let slot = sx(off.wrapping_add(c.g[T4][0] as u32));
            c.g[V1][0] = slot;
            let def = c.lw(slot as u32);
            c.g[V0][0] = def;
            c.g[A0][0] = c.g[T0][0];
            // The playback time (0.0: idle slot).
            if def != 0 {
                c.cycles += 4;
                let i = c.g[A3][0] as u32;
                let off = abi::mult(c, i, SLOT_SIZE as u32);
                c.g[V1][0] = sx(off);
                let at = sx(off.wrapping_add(c.g[T3][0] as u32));
                c.g[V0][0] = at;
                c.f[0] = c.lw(at as u32) as u32;
            } else {
                c.cycles += 1;
                c.f[0] = 0;
            }
            c.cycles += 7;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.sw(c.g[A0][0] as u32, c.f[0] as u64);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(4));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            let more = (c.g[A3][0] as i64) < SLOT_COUNT;
            c.g[V0][0] = more as u64;
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(SLOT_SIZE as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_9ce0);
            }
        }
        // s->Write(&rec, 0x80)
        c.cycles += 10;
        let stream = c.g[T2][0] as u32;
        let vt = c.lw(stream);
        c.g[V0][0] = vt;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x80;
        let delta = c.lh((vt as u32).wrapping_add(0x8));
        let write = c.lw((vt as u32).wrapping_add(0xc));
        c.g[V1][0] = write;
        c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
        call_ptr(c, write as u32, 0x002d_9d58)?;
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Avalanche_RenderActive (0x2d8ea8)

/// `tActiveAvalanche` (the slot): group, list of active nodes, limit.
pub(crate) const AVALANCHE_SLOT_ACTIVE: u32 = 0x4;

pub(crate) const AVALANCHE_SLOT_LIMIT: u32 = 0x18;

/// Active node: data pointer, next node, emitter at +0xd0.
pub(crate) const AVALANCHE_NODE_DATA: u32 = 0x2e0;

pub(crate) const AVALANCHE_NODE_NEXT: u32 = 0x2e4;

/// Debug globals: draw lines for the first N nodes (`gp + 0x968`), draw the
/// node effects (`gp + 0x964`).
pub(crate) const AVALANCHE_DEBUG_LINES_GP: u32 = 0x968;

pub(crate) const AVALANCHE_DEBUG_FX_GP: u32 = 0x964;

/// The debug line vertices (`D_vtx`, 0x30 bytes each, two per pair).
pub(crate) const AVALANCHE_LINE_VERTS: u32 = 0x004e_db70;

pub(crate) const AVALANCHE_DEBUG_DRAW_INSTANCE: Callee = Callee {
    addr: 0x002d_8a00,
    f: ee::s09::Avalanche_DebugDrawInstance,
};

/// `void tActiveAvalanche::render(tActiveAvalanche* this)` (0x2d8ea8): the
/// avalanche debug view. For each active node with data: its lines (first
/// `gp+0x968` nodes) and its emitter (`gp+0x964`, data `+0xf2`); then, with
/// lines on, one line per group pair from the pair's node to the rider
/// (red below the slot's limit, green at or above) drawn through GrMan
/// (vtable `+0x248`) with the draw state's bits 23-24 forced to 2.
pub fn avalanche_render_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S3, 0x70);
        frame.save(c, S6, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        let this = c.g[S3][0];
        let gp = c.g[GP][0] as u32;
        let group = ui_vector_lw(c, this, 0);
        c.g[V0][0] = group;
        let grman = Addr(gp.wrapping_add(AVALANCHE_GRMAN_GP)).lw(c);
        c.g[S6][0] = grman;
        if group == 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0xa0);
            avalanche_render_active_epilogue(c, frame);
            return Ok(());
        }
        // The active nodes.
        c.cycles += 3;
        c.g[S0][0] = ui_vector_lw(c, this, AVALANCHE_SLOT_ACTIVE);
        c.g[S1][0] = 0;
        while c.g[S0][0] != 0 {
            let node = c.g[S0][0];
            c.cycles += 3;
            let data = ui_vector_lw(c, node, AVALANCHE_NODE_DATA);
            c.g[V0][0] = Addr(gp.wrapping_add(AVALANCHE_DEBUG_LINES_GP)).lw(c);
            'next: {
                if data != 0 {
                    c.cycles += 3;
                    let draw_lines = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                    c.g[V0][0] = draw_lines as u64;
                    c.g[V0][0] = Addr(gp.wrapping_add(AVALANCHE_DEBUG_FX_GP)).lw(c);
                    if draw_lines {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S0][0];
                        AVALANCHE_DEBUG_DRAW_INSTANCE.call(c, 0x002d_8f10)?;
                        c.g[V0][0] = Addr(gp.wrapping_add(AVALANCHE_DEBUG_FX_GP)).lw(c);
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        let node = c.g[S0][0];
                        c.g[S0][0] = ui_vector_lw(c, node, AVALANCHE_NODE_NEXT);
                        break 'next;
                    }
                    c.cycles += 4;
                    let node = c.g[S0][0];
                    let data = ui_vector_lw(c, node, AVALANCHE_NODE_DATA);
                    c.g[V0][0] = data;
                    let fx = Addr((data as u32).wrapping_add(0xf2)).lbu(c);
                    c.g[V1][0] = fx;
                    c.g[A0][0] = sx((node as u32).wrapping_add(0xd0));
                    if fx != 0 {
                        c.cycles += 2;
                        c.g[A1][0] = 7;
                        call_ptr(c, 0x0037_1688, 0x002d_8f34)?;
                    }
                }
                c.cycles += 1;
                let node = c.g[S0][0];
                c.g[S0][0] = ui_vector_lw(c, node, AVALANCHE_NODE_NEXT);
            }
            c.cycles += 2;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            if c.g[S0][0] != 0 && event_due(c) {
                deliver_event_here(c, 0x002d_8ef0);
            }
        }
        c.cycles += 3;
        let lines = Addr(gp.wrapping_add(AVALANCHE_DEBUG_LINES_GP)).lw(c);
        c.g[V0][0] = lines;
        frame.restore(c, S0, 0xa0);
        if lines == 0 {
            avalanche_render_active_epilogue(c, frame);
            return Ok(());
        }
        // One line per pair: pair node's position (+0x40) to the rider.
        c.cycles += 4;
        let group = ui_vector_lw(c, this, 0);
        c.g[V1][0] = group;
        c.g[V0][0] = Addr((group as u32).wrapping_add(0xc)).lhu(c);
        c.g[S2][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x4f_0000;
            c.g[S4][0] = 1;
            c.g[S7][0] = sx(AVALANCHE_LINE_VERTS + 0x20);
            c.g[S5][0] = sx(AVALANCHE_LINE_VERTS);
            c.g[FP][0] = sx(AVALANCHE_LINE_VERTS + 0x50);
            loop {
                c.cycles += 18;
                let j = c.g[S2][0] as u32;
                let pair_off = j << 3;
                c.g[S0][0] = sx(pair_off);
                c.g[V0][0] = 0x60;
                c.mult(0, S1, S2, V0);
                let s1 = c.g[S1][0] as u32;
                let pair = (c.g[V1][0] as u32).wrapping_add(pair_off);
                c.g[V1][0] = sx(pair);
                let pnode = Addr(pair).off(0x14).lw(c);
                c.g[V0][0] = pnode;
                c.g[A1][0] = 0;
                c.lq(V1, (pnode as u32).wrapping_add(0x40));
                let v = s1.wrapping_add(c.g[S7][0] as u32);
                c.g[V0][0] = sx(v);
                c.sq(V1, v);
                let sp = frame.at(c, 0);
                c.sq(V1, sp);
                let below = avalanche_pair_below_limit(c, this, pair_off);
                c.g[A0][0] = sx(s1.wrapping_add(c.g[S5][0] as u32));
                if below {
                    c.cycles += 1;
                    c.g[A1][0] = 0xff;
                }
                c.cycles += 9;
                let a0 = c.g[A0][0] as u32;
                Addr(a0).off(0x10).sw(c, c.g[A1][0]);
                let above = avalanche_pair_at_limit(c, this, pair_off);
                c.g[V1][0] = 0;
                if above {
                    c.cycles += 1;
                    c.g[V1][0] = 0xff;
                }
                c.cycles += 25;
                Addr(a0).off(0x14).sw(c, c.g[V1][0]);
                // The rider's position: (SSXApp->Game->AI->rider + 0x6c0)->getPos().
                let app = Addr(gp.wrapping_sub(0x848)).lw(c);
                c.g[V0][0] = app;
                let game = ui_vector_lw(c, app, 0x84);
                let ai = ui_vector_lw(c, game, 0xc);
                c.g[A1][0] = ai;
                let rider = ui_vector_lw(c, ai, 0x28);
                let vt = ui_vector_lw(c, rider, 0x6c0);
                c.g[V1][0] = vt;
                let delta = Addr((vt as u32).wrapping_add(0x28)).lh(c);
                c.g[A1][0] = delta;
                let func = ui_vector_lw(c, vt, 0x2c);
                c.g[V0][0] = func;
                c.g[A0][0] = sx((rider as u32)
                    .wrapping_add(0x6c0)
                    .wrapping_add(delta as u32));
                call_ptr(c, func as u32, 0x002d_9010)?;
                c.lq(V1, c.g[V0][0] as u32);
                c.g[A0][0] = 0x30;
                let s1 = c.g[S1][0] as u32;
                let v = s1.wrapping_add(c.g[FP][0] as u32);
                c.g[V0][0] = sx(v);
                c.sq(V1, v);
                c.mult(0, V0, S4, A0);
                let vert = (c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32);
                c.g[V1][0] = sx(vert);
                let pair_off = c.g[S0][0] as u32;
                let below = avalanche_pair_below_limit(c, this, pair_off);
                c.g[A1][0] = 0;
                if below {
                    c.cycles += 1;
                    c.g[A1][0] = 0xff;
                }
                c.cycles += 9;
                Addr(vert).off(0x10).sw(c, c.g[A1][0]);
                let above = avalanche_pair_at_limit(c, this, pair_off);
                c.g[A0][0] = 0;
                if above {
                    c.cycles += 1;
                    c.g[A0][0] = 0xff;
                }
                c.cycles += 7;
                Addr(vert).off(0x14).sw(c, c.g[A0][0]);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                let group = ui_vector_lw(c, this, 0);
                c.g[V1][0] = group;
                let n = Addr((group as u32).wrapping_add(0xc)).lhu(c);
                let more = (c.g[S2][0] as i64) < (n as i64);
                c.g[V0][0] = more as u64;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_8f70);
                }
            }
        }
        // Draw state word +4: bits 23-24 = 2 for the draw, then put back.
        c.cycles += 28;
        let grman = c.g[S6][0];
        let ds = ui_vector_lw(c, grman, 0xe84);
        c.g[A0][0] = ds;
        c.g[S1][0] = 0xffff_ffff_fe7f_ffff;
        c.g[V1][0] = 0x0100_0000;
        let w = ui_vector_lw(c, ds, 4);
        c.g[S2][0] = 0x0180_0000;
        c.g[A3][0] = 0;
        c.g[A2][0] = sx(AVALANCHE_LINE_VERTS);
        c.g[V0][0] = (w & 0xffff_ffff_fe7f_ffff) | 0x0100_0000;
        Addr((ds as u32).wrapping_add(4)).sw(c, c.g[V0][0]);
        c.g[S0][0] = sx(((w & 0x0180_0000) as u32) >> 23);
        let vt = ui_vector_lw(c, grman, 0x10d8);
        c.g[V1][0] = vt;
        let group = ui_vector_lw(c, this, 0);
        c.g[V0][0] = group;
        let delta = Addr((vt as u32).wrapping_add(0x248)).lh(c);
        let func = ui_vector_lw(c, vt, 0x24c);
        c.g[T0][0] = func;
        c.g[A1][0] = Addr((group as u32).wrapping_add(0xc)).lhu(c);
        c.g[A0][0] = sx((grman as u32).wrapping_add(delta as u32));
        call_ptr(c, func as u32, 0x002d_90e4)?;
        let grman = c.g[S6][0];
        let ds = ui_vector_lw(c, grman, 0xe84);
        c.g[V1][0] = ds;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 23) & c.g[S2][0];
        let w = ui_vector_lw(c, ds, 4);
        c.g[V0][0] = (w & c.g[S1][0]) | c.g[S0][0];
        Addr((ds as u32).wrapping_add(4)).sw(c, c.g[V0][0]);
        c.cycles += 1;
        frame.restore(c, S0, 0xa0);
        avalanche_render_active_epilogue(c, frame);
        Ok(())
    })
}

/// `group->pairs[j].v < this->limit` (`c.lt.s`; f0, f1, v0 as the original
/// leaves them).
pub(crate) fn avalanche_pair_below_limit(c: &mut Cpu, this: u64, pair_off: u32) -> bool {
    let group = ui_vector_lw(c, this, 0);
    c.lwc1(1, (this as u32).wrapping_add(AVALANCHE_SLOT_LIMIT));
    let pair = (group as u32).wrapping_add(pair_off);
    c.g[V0][0] = sx(pair);
    c.lwc1(0, pair.wrapping_add(0x10));
    c.c_lt_s(0, 1);
    c.fcc()
}

/// `this->limit <= group->pairs[j].v` (`c.le.s`).
pub(crate) fn avalanche_pair_at_limit(c: &mut Cpu, this: u64, pair_off: u32) -> bool {
    let group = ui_vector_lw(c, this, 0);
    c.lwc1(1, (this as u32).wrapping_add(AVALANCHE_SLOT_LIMIT));
    let pair = (group as u32).wrapping_add(pair_off);
    c.g[V0][0] = sx(pair);
    c.lwc1(0, pair.wrapping_add(0x10));
    c.c_le_s(1, 0);
    c.fcc()
}

pub(crate) fn avalanche_render_active_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    frame.restore(c, FP, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

// Avalanche node (`self`) fields.
/// `self->frame`: the history frame the node was last advanced to.
#[cfg(test)]
pub(crate) const AVSH_FRAME: u32 = 0x0;

/// `self->pos` (t4Vector): the accumulated position.
pub(crate) const AVSH_POS: u32 = 0x10;

/// `self->mat` (4x4): the accumulated orientation.
pub(crate) const AVSH_MAT: u32 = 0x20;

/// `self->fB4`: the fade fraction `Avalanche_ComputeInstanceFade` writes.
pub(crate) const AVSH_FADE: u32 = 0xb4;

/// `self->emitter` (cDynamicColourEmitter).
pub(crate) const AVSH_EMITTER: u32 = 0xd0;

/// `self->f1FC`: the fade the last emitted particle used.
pub(crate) const AVSH_LAST_FADE: u32 = 0x1fc;

/// `self->data`: the packed history (see `AVSH_D_*`).
pub(crate) const AVSH_DATA: u32 = 0x2e0;

/// `self->emitting`: emit particles while replaying.
pub(crate) const AVSH_EMITTING: u32 = 0x2ec;

// Packed history (`self->data`) fields.
/// Start position (t4Vector) the history is reset to.
pub(crate) const AVSH_D_START: u32 = 0xe0;

/// Pointer to the 10-byte frame records.
pub(crate) const AVSH_D_FRAMES: u32 = 0xfc;

/// Frame count, as a float.
pub(crate) const AVSH_D_LENGTH: u32 = 0x100;

/// Playback rate multiplier, as a float.
pub(crate) const AVSH_D_RATE: u32 = 0x10c;

#[inline(always)]
pub(crate) fn avsh_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

#[inline(always)]
pub(crate) fn avsh_at_sub(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_sub(off)
}

/// `tFrameData Avalanche_SampleHistory(tFrameData *out, sAvAct *self, float time)`
/// (0x2d5778; the reference calls it `tActiveAvalancheNode_getFrameData`):
/// samples the packed avalanche history at `time` (30 Hz frames, 10 bytes per
/// frame: velocity, colour, axis, angle). It first updates the instance fade
/// (`Avalanche_ComputeInstanceFade`), then, when the frame differs from
/// `self->frame`, restarts from the start pose if it goes backwards and replays
/// every frame up to it (accumulating position and rotation with `Math_SinCos`,
/// and emitting particles through `DynamicEmitter_SpawnAt` while `emitting`),
/// and finally writes the position, the orientation and the interpolated colour
/// of the sampled frame into `out`.
///
/// Registers as the generated body: s1 = self, s4 = frame, f20 = time (then the
/// emit time), f22 = frac, f26 = 1 - frac; sp+0xf0 = out, sp+0x100 = &self->mat.
#[allow(unused_assignments, unused_variables, unused_labels)]
pub fn avalanche_sample_history(c: &mut Cpu) {
    guest(c, |c| {
        'b_2d581c: {
            c.cycles += 38;
            c.g[SP][0] = sx(avsh_at_sub(c, SP, 0x1f0));
            c.sq(17, avsh_at(c, SP, 0x190));
            c.swc1(20, avsh_at(c, SP, 0x1b0));
            c.g[S1][0] = c.g[A1][0];
            c.mov_s(20, 12);
            c.sw(avsh_at(c, SP, 0xf0), c.g[A0][0]);
            c.sq(20, avsh_at(c, SP, 0x160));
            c.swc1(22, avsh_at(c, SP, 0x1c0));
            c.g[A0][0] = c.g[S1][0];
            c.sq(16, avsh_at(c, SP, 0x1a0));
            c.sq(18, avsh_at(c, SP, 0x180));
            c.sq(19, avsh_at(c, SP, 0x170));
            c.sq(21, avsh_at(c, SP, 0x150));
            c.sq(22, avsh_at(c, SP, 0x140));
            c.sq(23, avsh_at(c, SP, 0x130));
            c.sq(30, avsh_at(c, SP, 0x120));
            c.sd(avsh_at(c, SP, 0x110), c.g[RA][0]);
            c.swc1(26, avsh_at(c, SP, 0x1e0));
            c.swc1(25, avsh_at(c, SP, 0x1d8));
            c.swc1(24, avsh_at(c, SP, 0x1d0));
            c.swc1(23, avsh_at(c, SP, 0x1c8));
            c.swc1(21, avsh_at(c, SP, 0x1b8));
            call_ptr(c, 0x002d_7c00, 0x002d_57d4)?;
            c.g[AT][0] = 0x41f00000;
            c.mtc1(1, 0);
            c.g[A0][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
            c.mul_s(20, 20, 0);
            c.lwc1(1, avsh_at(c, A0, AVSH_D_LENGTH));
            c.cvt_w_s(0, 20);
            c.mfc1(20, 0);
            c.cvt_w_s(0, 1);
            c.mfc1(3, 0);
            c.mtc1(20, 0);
            c.cvt_s_w(0, 0);
            c.g[V0][0] = sx(avsh_at_sub(c, V1, 0x1));
            c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let t = c.g[V0][0] != 0u64;
            c.sub_s(22, 20, 0);
            if t {
                break 'b_2d581c;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 22);
                c.g[S4][0] = sx(avsh_at_sub(c, V1, 0x2));
                break 'b_2d581c;
            }
        }
        // Replay the recorded frames up to `frame`, restarting when going back.
        'b_2d5e30: {
            'b_2d5e2c: {
                c.cycles += 16;
                c.g[V0][0] = sx(avsh_at(c, S1, AVSH_MAT));
                c.g[V1][0] = sx(avsh_at(c, SP, 0xe8));
                c.sw(avsh_at(c, SP, 0x100), c.g[V0][0]);
                c.g[S7][0] = sx(avsh_at(c, SP, 0x20));
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                c.g[FP][0] = sx(avsh_at(c, SP, 0x90));
                c.sw(avsh_at(c, SP, 0xf4), c.g[V1][0]);
                c.g[V1][0] = sx(avsh_at(c, SP, 0xec));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.sw(avsh_at(c, SP, 0xf8), c.g[V1][0]);
                c.g[V1][0] = c.lw(avsh_at(c, SP, 0xf0));
                c.sub_s(26, 0, 22);
                c.g[V1][0] = sx(avsh_at(c, V1, 0x20));
                let t = c.g[S4][0] == c.g[V0][0];
                c.sw(avsh_at(c, SP, 0xfc), c.g[V1][0]);
                if t {
                    break 'b_2d5e2c;
                } else {
                    'b_2d58a4: {
                        c.cycles += 2;
                        let t = c.g[S4][0] != 0u64;
                        c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if t {
                            break 'b_2d58a4;
                        } else {
                            c.cycles += 16;
                            c.sw(c.g[S1][0] as u32, 0u64);
                            c.g[V1][0] = 0x500000;
                            c.g[V1][0] = sx(avsh_at_sub(c, V1, 0xe60));
                            c.lq(2, avsh_at(c, A0, AVSH_D_START));
                            c.sq(2, avsh_at(c, S1, AVSH_POS));
                            c.g[A0][0] = c.lw(avsh_at(c, SP, 0x100));
                            c.lqc2(1, c.g[V1][0] as u32);
                            c.lqc2(2, avsh_at(c, V1, 0x10));
                            c.lqc2(3, avsh_at(c, V1, 0x20));
                            c.lqc2(4, avsh_at(c, V1, 0x30));
                            c.sqc2(1, c.g[A0][0] as u32);
                            c.sqc2(2, avsh_at(c, A0, 0x10));
                            c.sqc2(3, avsh_at(c, A0, 0x20));
                            c.sqc2(4, avsh_at(c, A0, 0x30));
                            let t = true;
                            c.g[A2][0] = 0xa;
                            if t {
                                break 'b_2d5e30;
                            } else {
                                break 'b_2d58a4;
                            }
                        }
                    }
                    'b_2d58e0: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = 0x500000;
                        if t {
                            break 'b_2d58e0;
                        } else {
                            c.cycles += 13;
                            c.sw(c.g[S1][0] as u32, 0u64);
                            c.g[V1][0] = sx(avsh_at_sub(c, V1, 0xe60));
                            c.lq(2, avsh_at(c, A0, AVSH_D_START));
                            c.sq(2, avsh_at(c, S1, AVSH_POS));
                            c.g[V0][0] = c.lw(avsh_at(c, SP, 0x100));
                            c.lqc2(1, c.g[V1][0] as u32);
                            c.lqc2(2, avsh_at(c, V1, 0x10));
                            c.lqc2(3, avsh_at(c, V1, 0x20));
                            c.lqc2(4, avsh_at(c, V1, 0x30));
                            c.sqc2(1, c.g[V0][0] as u32);
                            c.sqc2(2, avsh_at(c, V0, 0x10));
                            c.sqc2(3, avsh_at(c, V0, 0x20));
                            c.sqc2(4, avsh_at(c, V0, 0x30));
                            break 'b_2d58e0;
                        }
                    }
                    // Walk the recorded frames one step at a time (emit particles while `emitting`).
                    'b_2d5e28: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                        c.mtc1(0, 23);
                        let t = c.g[V0][0] == c.g[S4][0];
                        c.mov_s(24, 0);
                        if t {
                            break 'b_2d5e28;
                        } else {
                            c.cycles += 4;
                            c.lwc1(25, avsh_at_sub(c, GP, 0x3d14));
                            c.g[S6][0] = c.lw(avsh_at(c, SP, 0x100));
                            c.g[S5][0] = 0xa;
                            'l_2d5900: loop {
                                'b_2d5b5c: {
                                    'b_2d5b58: {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw(avsh_at(c, S1, AVSH_EMITTING));
                                        c.g[S3][0] = sx(avsh_at(c, SP, 0x40));
                                        c.g[S0][0] = sx(avsh_at(c, SP, 0xe0));
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S2][0] = sx(avsh_at(c, SP, 0xe4));
                                        if t {
                                            break 'b_2d5b58;
                                        } else {
                                            c.cycles += 9;
                                            c.g[AT][0] = 0x41f00000;
                                            c.mtc1(1, 1);
                                            c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                                            c.mul_s(0, 23, 1);
                                            c.cvt_w_s(2, 0);
                                            c.mfc1(2, 2);
                                            if c.g[V0][0] != c.g[V1][0] {
                                                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                                break 'b_2d5b5c;
                                            } else {
                                                c.cycles += 4;
                                                c.g[AT][0] = 0x3c000000;
                                                c.mtc1(1, 21);
                                                c.mul_s(10, 23, 1);
                                                'l_2d5948: loop {
                                                    'b_2d5984: {
                                                        c.cycles += 12;
                                                        c.g[A0][0] =
                                                            c.lw(avsh_at(c, S1, AVSH_DATA));
                                                        c.lwc1(3, avsh_at(c, A0, AVSH_D_LENGTH));
                                                        c.cvt_w_s(0, 10);
                                                        c.mfc1(5, 0);
                                                        c.cvt_w_s(0, 3);
                                                        c.mfc1(3, 0);
                                                        c.mtc1(5, 0);
                                                        c.cvt_s_w(0, 0);
                                                        c.g[V0][0] = sx(avsh_at_sub(c, V1, 0x1));
                                                        c.g[V0][0] = ((c.g[A1][0] as i64)
                                                            < (c.g[V0][0] as i64))
                                                            as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.sub_s(8, 10, 0);
                                                        if t {
                                                            break 'b_2d5984;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[AT][0] = 0x3f800000;
                                                            c.mtc1(1, 8);
                                                            c.g[A1][0] =
                                                                sx(avsh_at_sub(c, V1, 0x2));
                                                            break 'b_2d5984;
                                                        }
                                                    }
                                                    'b_2d59b4: {
                                                        c.cycles += 11;
                                                        c.lwc1(0, avsh_at(c, GP, 0x974));
                                                        c.sub_s(9, 24, 8);
                                                        c.lwc1(2, avsh_at(c, A0, AVSH_D_RATE));
                                                        c.lwc1(1, avsh_at_sub(c, GP, 0x3d10));
                                                        c.mul_s(0, 0, 2);
                                                        c.mul_s(1, 3, 1);
                                                        c.add_s(0, 23, 0);
                                                        c.c_lt_s(0, 1);
                                                        let t = !c.fcc();
                                                        c.mov_s(20, 1);
                                                        if t {
                                                            break 'b_2d59b4;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.mov_s(20, 0);
                                                            break 'b_2d59b4;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.c_lt_s(10, 3);
                                                    let t = !c.fcc();
                                                    c.mov_s(23, 20);
                                                    if t {
                                                        break 'b_2d5b58;
                                                    } else {
                                                        'b_2d5b20: {
                                                            c.cycles += 87;
                                                            c.mult(0, 5, 5, 21);
                                                            c.g[V0][0] =
                                                                c.lw(avsh_at(c, A0, AVSH_D_FRAMES));
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[T1][0] =
                                                                sx(avsh_at(c, S1, AVSH_EMITTER));
                                                            c.g[T2][0] =
                                                                sx(avsh_at_sub(c, V1, 0xee0));
                                                            c.g[A3][0] = sx(avsh_at(c, SP, 0x10));
                                                            c.g[T0][0] = 0x1;
                                                            c.g[V0][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.g[V1][0] = c.lb(avsh_at(c, V0, 0x2));
                                                            c.g[A0][0] = c.lb(c.g[V0][0] as u32);
                                                            c.g[A2][0] = c.lb(avsh_at(c, V0, 0x1));
                                                            c.mtc1(3, 1);
                                                            c.cvt_s_w(1, 1);
                                                            c.mtc1(4, 2);
                                                            c.cvt_s_w(2, 2);
                                                            c.sw(avsh_at(c, SP, 0x1c), 0u64);
                                                            c.mtc1(6, 0);
                                                            c.cvt_s_w(0, 0);
                                                            c.mul_s(1, 1, 8);
                                                            c.mul_s(2, 2, 8);
                                                            c.mul_s(0, 0, 8);
                                                            c.add_s(1, 1, 1);
                                                            c.add_s(2, 2, 2);
                                                            c.add_s(0, 0, 0);
                                                            c.swc1(1, avsh_at(c, SP, 0x18));
                                                            c.swc1(2, avsh_at(c, SP, 0x10));
                                                            c.swc1(0, avsh_at(c, SP, 0x14));
                                                            c.lqc2(3, avsh_at(c, SP, 0x10));
                                                            c.lqc2(4, avsh_at(c, S1, AVSH_POS));
                                                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                            c.sqc2(5, avsh_at(c, SP, 0x20));
                                                            c.lq(2, avsh_at(c, SP, 0x20));
                                                            c.g[V1][0] =
                                                                c.lw(avsh_at(c, S1, AVSH_DATA));
                                                            c.sq(2, c.g[SP][0] as u32);
                                                            c.lwc1(6, avsh_at(c, GP, 0x9b8));
                                                            c.g[V0][0] =
                                                                c.lw(avsh_at(c, V1, AVSH_D_FRAMES));
                                                            c.lwc1(7, avsh_at(c, S1, AVSH_FADE));
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.g[V1][0] = c.lbu(avsh_at(c, A1, 0xf));
                                                            c.g[V0][0] = c.lbu(avsh_at(c, A1, 0x3));
                                                            c.mtc1(3, 5);
                                                            c.cvt_s_w(5, 5);
                                                            c.g[A0][0] = c.lbu(avsh_at(c, A1, 0xd));
                                                            c.mtc1(2, 2);
                                                            c.cvt_s_w(2, 2);
                                                            c.g[A2][0] = c.lbu(avsh_at(c, A1, 0x4));
                                                            c.g[V0][0] = c.lbu(avsh_at(c, A1, 0x5));
                                                            c.mtc1(4, 4);
                                                            c.cvt_s_w(4, 4);
                                                            c.g[V1][0] = c.lbu(avsh_at(c, A1, 0xe));
                                                            c.mtc1(6, 1);
                                                            c.cvt_s_w(1, 1);
                                                            c.mtc1(2, 0);
                                                            c.cvt_s_w(0, 0);
                                                            c.swc1(7, avsh_at(c, SP, 0x2c));
                                                            c.mtc1(3, 3);
                                                            c.cvt_s_w(3, 3);
                                                            c.mul_s(5, 5, 8);
                                                            c.mul_s(2, 2, 9);
                                                            c.mul_s(0, 0, 9);
                                                            c.mul_s(4, 4, 8);
                                                            c.mul_s(1, 1, 9);
                                                            c.mul_s(3, 3, 8);
                                                            c.add_s(2, 2, 4);
                                                            c.add_s(0, 0, 5);
                                                            c.add_s(1, 1, 3);
                                                            c.mul_s(2, 2, 21);
                                                            c.mul_s(0, 0, 21);
                                                            c.mul_s(1, 1, 21);
                                                            c.swc1(2, avsh_at(c, SP, 0x20));
                                                            c.swc1(0, avsh_at(c, SP, 0x28));
                                                            c.swc1(1, avsh_at(c, SP, 0x24));
                                                            c.mfc1(3, 6);
                                                            c.lqc2(4, avsh_at(c, SP, 0x20));
                                                            c.qmtc2(3, 3);
                                                            c.vu0().mul(
                                                                0b1111,
                                                                5,
                                                                4,
                                                                Src::Bc(3, X),
                                                            );
                                                            c.sqc2(5, avsh_at(c, SP, 0x30));
                                                            c.lq(2, avsh_at(c, SP, 0x30));
                                                            c.sq(2, avsh_at(c, SP, 0x10));
                                                            c.lwc1(1, avsh_at(c, S1, AVSH_FADE));
                                                            c.g[V0][0] =
                                                                c.lw(avsh_at(c, S1, AVSH_DATA));
                                                            c.swc1(
                                                                1,
                                                                avsh_at(c, S1, AVSH_LAST_FADE),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                avsh_at(c, V0, AVSH_D_LENGTH),
                                                            );
                                                            c.c_lt_s(10, 0);
                                                            if !c.fcc() {
                                                                c.g[T0][0] = 0u64;
                                                                break 'b_2d5b20;
                                                            } else {
                                                                break 'b_2d5b20;
                                                            }
                                                        }
                                                        c.cycles += 14;
                                                        c.lwc1(12, avsh_at_sub(c, GP, 0x3d0c));
                                                        c.g[A0][0] = c.g[T1][0];
                                                        c.g[A2][0] = c.g[T2][0];
                                                        c.g[A1][0] = c.g[SP][0];
                                                        call_ptr(c, 0x003717c0, 0x002d5b34)?;
                                                        c.g[AT][0] = 0x41f00000;
                                                        c.mtc1(1, 1);
                                                        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                                        c.mul_s(0, 20, 1);
                                                        c.cvt_w_s(2, 0);
                                                        c.mfc1(3, 2);
                                                        if c.g[V1][0] == c.g[V0][0] {
                                                            c.mul_s(10, 23, 1);
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x002d5948);
                                                            }
                                                            continue 'l_2d5948;
                                                        } else {
                                                            break 'b_2d5b58;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                    break 'b_2d5b5c;
                                }
                                'b_2d5c60: {
                                    'b_2d5c58: {
                                        c.cycles += 55;
                                        c.g[A0][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
                                        c.mult(0, 2, 2, 21);
                                        c.mtc1(0, 3);
                                        c.g[V1][0] = c.lw(avsh_at(c, A0, AVSH_D_FRAMES));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                        c.g[A0][0] = c.lb(avsh_at(c, V0, 0x2));
                                        c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                                        c.g[A1][0] = c.lb(avsh_at(c, V0, 0x1));
                                        c.mtc1(4, 1);
                                        c.cvt_s_w(1, 1);
                                        c.mtc1(3, 2);
                                        c.cvt_s_w(2, 2);
                                        c.swc1(3, avsh_at(c, SP, 0xc));
                                        c.mtc1(5, 0);
                                        c.cvt_s_w(0, 0);
                                        c.add_s(1, 1, 1);
                                        c.add_s(2, 2, 2);
                                        c.add_s(0, 0, 0);
                                        c.swc1(1, avsh_at(c, SP, 0x8));
                                        c.swc1(2, c.g[SP][0] as u32);
                                        c.swc1(0, avsh_at(c, SP, 0x4));
                                        c.lqc2(3, avsh_at(c, S1, AVSH_POS));
                                        c.lqc2(4, c.g[SP][0] as u32);
                                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, avsh_at(c, S1, AVSH_POS));
                                        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                        c.g[A0][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
                                        c.mult(0, 2, 2, 21);
                                        c.g[V1][0] = c.lw(avsh_at(c, A0, AVSH_D_FRAMES));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                        c.g[A0][0] = c.lb(avsh_at(c, V0, 0x6));
                                        c.g[A1][0] = c.lb(avsh_at(c, V0, 0x7));
                                        c.mtc1(4, 2);
                                        c.cvt_s_w(2, 2);
                                        c.g[V1][0] = c.lb(avsh_at(c, V0, 0x8));
                                        c.mtc1(5, 0);
                                        c.cvt_s_w(0, 0);
                                        c.mtc1(3, 1);
                                        c.cvt_s_w(1, 1);
                                        c.mul_s(5, 2, 25);
                                        c.mul_s(6, 0, 25);
                                        c.mul_s(4, 1, 25);
                                        c.mul_s(0, 5, 5);
                                        c.swc1(5, avsh_at(c, SP, 0x20));
                                        c.mul_s(2, 6, 6);
                                        c.swc1(6, avsh_at(c, SP, 0x24));
                                        c.mul_s(1, 4, 4);
                                        c.swc1(4, avsh_at(c, SP, 0x28));
                                        c.add_s(0, 0, 2);
                                        c.add_s(0, 0, 1);
                                        c.sqrt_s(0, 0);
                                        c.c_eq_s(0, 3);
                                        if c.fcc() {
                                            c.swc1(5, avsh_at(c, SP, 0x10));
                                            break 'b_2d5c58;
                                        } else {
                                            c.cycles += 8;
                                            c.div_s(0, 24, 0);
                                            c.mul_s(1, 4, 0);
                                            c.mul_s(2, 5, 0);
                                            c.mul_s(0, 6, 0);
                                            c.swc1(1, avsh_at(c, SP, 0x18));
                                            c.swc1(2, avsh_at(c, SP, 0x10));
                                            let t = true;
                                            c.swc1(0, avsh_at(c, SP, 0x14));
                                            if t {
                                                break 'b_2d5c60;
                                            } else {
                                                break 'b_2d5c58;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.swc1(6, avsh_at(c, SP, 0x14));
                                    c.swc1(4, avsh_at(c, SP, 0x18));
                                    break 'b_2d5c60;
                                }
                                c.cycles += 114;
                                c.lwc1(0, avsh_at(c, SP, 0x10));
                                c.lwc1(1, avsh_at(c, SP, 0x14));
                                c.lwc1(2, avsh_at(c, SP, 0x18));
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                c.swc1(0, c.g[SP][0] as u32);
                                c.swc1(1, avsh_at(c, SP, 0x4));
                                c.mult(0, 4, 4, 21);
                                c.swc1(2, avsh_at(c, SP, 0x8));
                                c.sw(avsh_at(c, SP, 0xc), 0u64);
                                c.lwc1(0, avsh_at_sub(c, GP, 0x3d08));
                                c.g[V1][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
                                c.g[V0][0] = c.lw(avsh_at(c, V1, AVSH_D_FRAMES));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[V1][0] = c.lbu(avsh_at(c, A0, 0x9));
                                c.mtc1(3, 12);
                                c.cvt_s_w(12, 12);
                                c.mul_s(12, 12, 0);
                                c.lqc2(1, c.g[S6][0] as u32);
                                c.lqc2(2, avsh_at(c, S6, 0x10));
                                c.lqc2(3, avsh_at(c, S6, 0x20));
                                c.lqc2(4, avsh_at(c, S6, 0x30));
                                c.sqc2(1, c.g[S3][0] as u32);
                                c.sqc2(2, avsh_at(c, S3, 0x10));
                                c.sqc2(3, avsh_at(c, S3, 0x20));
                                c.sqc2(4, avsh_at(c, S3, 0x30));
                                c.g[V0][0] = 0x500000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[V1][0] = sx(avsh_at_sub(c, V0, 0xed0));
                                c.lq(5, c.g[V1][0] as u32);
                                c.lq(2, avsh_at(c, SP, 0x70));
                                c.sq(5, avsh_at(c, SP, 0x70));
                                c.sq(2, avsh_at(c, SP, 0x80));
                                c.g[A1][0] = c.g[S2][0];
                                call_ptr(c, 0x0031be50, 0x002d5ce8)?;
                                c.lwc1(11, avsh_at(c, SP, 0xe4));
                                c.lwc1(0, c.g[SP][0] as u32);
                                c.sub_s(1, 24, 11);
                                c.lwc1(3, avsh_at(c, SP, 0x8));
                                c.lwc1(7, avsh_at(c, SP, 0x4));
                                c.lwc1(8, avsh_at(c, SP, 0xe0));
                                c.mul_s(2, 1, 3);
                                c.lq(2, avsh_at(c, SP, 0x80));
                                c.mul_s(4, 1, 0);
                                c.sw(avsh_at(c, S1, 0x50), 0u64);
                                c.mul_s(1, 1, 7);
                                c.sw(avsh_at(c, S1, 0x54), 0u64);
                                c.mul_s(13, 8, 3);
                                c.sw(avsh_at(c, S1, 0x58), 0u64);
                                c.mul_s(12, 8, 0);
                                c.swc1(24, avsh_at(c, S1, 0x5c));
                                c.mul_s(9, 2, 3);
                                c.sq(2, avsh_at(c, S1, 0x50));
                                c.mul_s(10, 4, 3);
                                c.sw(avsh_at(c, S1, 0x2c), 0u64);
                                c.mul_s(5, 2, 0);
                                c.sw(avsh_at(c, S1, 0x3c), 0u64);
                                c.mul_s(6, 4, 0);
                                c.sw(avsh_at(c, S1, 0x4c), 0u64);
                                c.mul_s(3, 1, 3);
                                c.mul_s(0, 1, 0);
                                c.mul_s(8, 8, 7);
                                c.mul_s(2, 2, 7);
                                c.mul_s(4, 4, 7);
                                c.mul_s(1, 1, 7);
                                c.add_s(5, 5, 8);
                                c.sub_s(0, 0, 13);
                                c.sub_s(2, 2, 12);
                                c.add_s(9, 9, 11);
                                c.swc1(5, avsh_at(c, S1, 0x28));
                                c.add_s(6, 6, 11);
                                c.swc1(0, avsh_at(c, S1, 0x24));
                                c.add_s(4, 4, 13);
                                c.swc1(2, avsh_at(c, S1, 0x38));
                                c.sub_s(10, 10, 8);
                                c.swc1(9, avsh_at(c, S1, 0x48));
                                c.add_s(1, 1, 11);
                                c.swc1(6, avsh_at(c, S1, AVSH_MAT));
                                c.add_s(3, 3, 12);
                                c.swc1(4, avsh_at(c, S1, 0x30));
                                c.swc1(10, avsh_at(c, S1, 0x40));
                                c.swc1(1, avsh_at(c, S1, 0x34));
                                c.swc1(3, avsh_at(c, S1, 0x44));
                                c.lqc2(4, c.g[S6][0] as u32);
                                c.lqc2(5, avsh_at(c, S6, 0x10));
                                c.lqc2(6, avsh_at(c, S6, 0x20));
                                c.lqc2(7, avsh_at(c, S6, 0x30));
                                c.lqc2(8, c.g[S3][0] as u32);
                                c.lqc2(9, avsh_at(c, S3, 0x10));
                                c.lqc2(10, avsh_at(c, S3, 0x20));
                                c.lqc2(11, avsh_at(c, S3, 0x30));
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
                                c.sqc2(12, c.g[S6][0] as u32);
                                c.sqc2(13, avsh_at(c, S6, 0x10));
                                c.sqc2(14, avsh_at(c, S6, 0x20));
                                c.sqc2(15, avsh_at(c, S6, 0x30));
                                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                c.g[V0][0] = sx(avsh_at(c, V0, 0x1));
                                let t = c.g[V0][0] != c.g[S4][0];
                                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002d5900);
                                    }
                                    continue 'l_2d5900;
                                } else {
                                    break 'b_2d5e28;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.sw(avsh_at(c, S1, AVSH_EMITTING), 0u64);
                    break 'b_2d5e2c;
                }
            }
            c.cycles += 1;
            c.g[A2][0] = 0xa;
            break 'b_2d5e30;
        }
        // Result: interpolate frame `frame` with `frac` into the output record.
        'b_2d6038: {
            'b_2d6030: {
                c.cycles += 118;
                c.g[V1][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
                c.mult(0, 6, 20, 6);
                c.g[AT][0] = 0x3c000000;
                c.mtc1(1, 6);
                c.g[V0][0] = c.lw(avsh_at(c, V1, AVSH_D_FRAMES));
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lb(avsh_at(c, V0, 0x2));
                c.g[A0][0] = c.lb(c.g[V0][0] as u32);
                c.g[A1][0] = c.lb(avsh_at(c, V0, 0x1));
                c.mtc1(3, 1);
                c.cvt_s_w(1, 1);
                c.mtc1(4, 2);
                c.cvt_s_w(2, 2);
                c.sw(avsh_at(c, SP, 0x7c), 0u64);
                c.mtc1(5, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(1, 1, 22);
                c.mul_s(2, 2, 22);
                c.mul_s(0, 0, 22);
                c.add_s(1, 1, 1);
                c.add_s(2, 2, 2);
                c.add_s(0, 0, 0);
                c.swc1(1, avsh_at(c, SP, 0x78));
                c.swc1(2, avsh_at(c, SP, 0x70));
                c.swc1(0, avsh_at(c, SP, 0x74));
                c.lqc2(3, avsh_at(c, SP, 0x70));
                c.lqc2(4, avsh_at(c, S1, AVSH_POS));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, avsh_at(c, SP, 0x80));
                c.lq(2, avsh_at(c, SP, 0x80));
                c.g[V1][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
                c.sq(2, c.g[SP][0] as u32);
                c.sq(2, avsh_at(c, SP, 0x60));
                c.lwc1(8, avsh_at(c, GP, 0x9b8));
                c.g[V0][0] = c.lw(avsh_at(c, V1, AVSH_D_FRAMES));
                c.lwc1(7, avsh_at(c, S1, AVSH_FADE));
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lbu(avsh_at(c, V0, 0xf));
                c.g[A0][0] = c.lbu(avsh_at(c, V0, 0x3));
                c.mtc1(3, 5);
                c.cvt_s_w(5, 5);
                c.g[A1][0] = c.lbu(avsh_at(c, V0, 0xd));
                c.mtc1(4, 2);
                c.cvt_s_w(2, 2);
                c.g[A3][0] = c.lbu(avsh_at(c, V0, 0x4));
                c.g[V1][0] = c.lbu(avsh_at(c, V0, 0xe));
                c.mtc1(5, 4);
                c.cvt_s_w(4, 4);
                c.g[A0][0] = c.lbu(avsh_at(c, V0, 0x5));
                c.mtc1(7, 1);
                c.cvt_s_w(1, 1);
                c.mtc1(3, 3);
                c.cvt_s_w(3, 3);
                c.swc1(7, avsh_at(c, SP, 0x7c));
                c.mtc1(4, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(2, 2, 26);
                c.mul_s(4, 4, 22);
                c.mul_s(0, 0, 26);
                c.mul_s(1, 1, 26);
                c.mul_s(3, 3, 22);
                c.mul_s(5, 5, 22);
                c.add_s(2, 2, 4);
                c.add_s(1, 1, 3);
                c.add_s(0, 0, 5);
                c.mul_s(2, 2, 6);
                c.mul_s(1, 1, 6);
                c.mul_s(0, 0, 6);
                c.swc1(2, avsh_at(c, SP, 0x70));
                c.swc1(1, avsh_at(c, SP, 0x74));
                c.swc1(0, avsh_at(c, SP, 0x78));
                c.mfc1(3, 8);
                c.lqc2(4, avsh_at(c, SP, 0x70));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, avsh_at(c, SP, 0x80));
                c.lq(2, avsh_at(c, SP, 0x80));
                c.sq(2, avsh_at(c, SP, 0x60));
                c.sq(2, avsh_at(c, SP, 0x10));
                c.g[V1][0] = c.lw(avsh_at(c, SP, 0x100));
                c.lqc2(1, c.g[V1][0] as u32);
                c.lqc2(2, avsh_at(c, V1, 0x10));
                c.lqc2(3, avsh_at(c, V1, 0x20));
                c.lqc2(4, avsh_at(c, V1, 0x30));
                c.sqc2(1, c.g[S7][0] as u32);
                c.sqc2(2, avsh_at(c, S7, 0x10));
                c.sqc2(3, avsh_at(c, S7, 0x20));
                c.sqc2(4, avsh_at(c, S7, 0x30));
                c.g[V0][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
                c.lwc1(1, avsh_at_sub(c, GP, 0x3d04));
                c.g[V1][0] = c.lw(avsh_at(c, V0, AVSH_D_FRAMES));
                c.mtc1(0, 4);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = c.lb(avsh_at(c, A2, 0x6));
                c.g[A0][0] = c.lb(avsh_at(c, A2, 0x7));
                c.g[V1][0] = c.lb(avsh_at(c, A2, 0x8));
                c.mtc1(2, 3);
                c.cvt_s_w(3, 3);
                c.mtc1(4, 0);
                c.cvt_s_w(0, 0);
                c.mtc1(3, 2);
                c.cvt_s_w(2, 2);
                c.mul_s(3, 3, 1);
                c.mul_s(6, 0, 1);
                c.mul_s(5, 2, 1);
                c.mul_s(0, 3, 3);
                c.swc1(3, avsh_at(c, SP, 0x80));
                c.mul_s(2, 6, 6);
                c.swc1(6, avsh_at(c, SP, 0x84));
                c.mul_s(1, 5, 5);
                c.swc1(5, avsh_at(c, SP, 0x88));
                c.add_s(0, 0, 2);
                c.add_s(0, 0, 1);
                c.sqrt_s(1, 0);
                c.c_eq_s(1, 4);
                if c.fcc() {
                    c.swc1(3, avsh_at(c, SP, 0x70));
                    break 'b_2d6030;
                } else {
                    c.cycles += 10;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.div_s(0, 0, 1);
                    c.mul_s(1, 5, 0);
                    c.mul_s(2, 3, 0);
                    c.mul_s(0, 6, 0);
                    c.swc1(1, avsh_at(c, SP, 0x78));
                    c.swc1(2, avsh_at(c, SP, 0x70));
                    let t = true;
                    c.swc1(0, avsh_at(c, SP, 0x74));
                    if t {
                        break 'b_2d6038;
                    } else {
                        break 'b_2d6030;
                    }
                }
            }
            c.cycles += 2;
            c.swc1(6, avsh_at(c, SP, 0x74));
            c.swc1(5, avsh_at(c, SP, 0x78));
            break 'b_2d6038;
        }
        c.cycles += 150;
        c.lwc1(1, avsh_at(c, SP, 0x74));
        c.g[A0][0] = 0xa;
        c.lwc1(0, avsh_at(c, SP, 0x78));
        c.mult(0, 4, 20, 4);
        c.lwc1(2, avsh_at(c, SP, 0x70));
        c.g[S0][0] = c.g[S7][0];
        c.swc1(1, avsh_at(c, SP, 0x64));
        c.swc1(0, avsh_at(c, SP, 0x68));
        c.swc1(2, avsh_at(c, SP, 0x60));
        c.lwc1(0, avsh_at_sub(c, GP, 0x3d00));
        c.g[V0][0] = c.lw(avsh_at(c, S1, AVSH_DATA));
        c.sw(avsh_at(c, SP, 0x6c), 0u64);
        c.lwc1(1, avsh_at_sub(c, GP, 0x3cfc));
        c.g[V1][0] = c.lw(avsh_at(c, V0, AVSH_D_FRAMES));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = c.lbu(avsh_at(c, A0, 0x9));
        c.mtc1(2, 12);
        c.cvt_s_w(12, 12);
        c.mul_s(12, 12, 0);
        c.mul_s(12, 22, 12);
        c.mul_s(12, 12, 1);
        c.lqc2(1, c.g[S0][0] as u32);
        c.lqc2(2, avsh_at(c, S0, 0x10));
        c.lqc2(3, avsh_at(c, S0, 0x20));
        c.lqc2(4, avsh_at(c, S0, 0x30));
        c.sqc2(1, c.g[FP][0] as u32);
        c.sqc2(2, avsh_at(c, FP, 0x10));
        c.sqc2(3, avsh_at(c, FP, 0x20));
        c.sqc2(4, avsh_at(c, FP, 0x30));
        c.g[A0][0] = 0x500000;
        c.lq(2, avsh_at(c, SP, 0xc0));
        c.g[V1][0] = sx(avsh_at_sub(c, A0, 0xed0));
        c.lq(5, c.g[V1][0] as u32);
        c.g[A0][0] = c.lw(avsh_at(c, SP, 0xf4));
        c.sq(5, avsh_at(c, SP, 0xc0));
        c.g[A1][0] = c.lw(avsh_at(c, SP, 0xf8));
        c.sq(2, avsh_at(c, SP, 0xd0));
        call_ptr(c, 0x0031be50, 0x002d60d0)?;
        c.lwc1(11, avsh_at(c, SP, 0xec));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 5);
        c.lwc1(0, avsh_at(c, SP, 0x60));
        c.sub_s(1, 5, 11);
        c.lwc1(3, avsh_at(c, SP, 0x68));
        c.lwc1(7, avsh_at(c, SP, 0x64));
        c.lwc1(8, avsh_at(c, SP, 0xe8));
        c.mul_s(2, 1, 3);
        c.swc1(5, avsh_at(c, SP, 0x5c));
        c.mul_s(4, 1, 0);
        c.lq(2, avsh_at(c, SP, 0xd0));
        c.mul_s(1, 1, 7);
        c.sw(avsh_at(c, SP, 0x50), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(avsh_at(c, SP, 0x54), 0u64);
        c.mul_s(12, 8, 0);
        c.sw(avsh_at(c, SP, 0x58), 0u64);
        c.mul_s(9, 2, 3);
        c.sq(2, avsh_at(c, SP, 0x50));
        c.mul_s(10, 4, 3);
        c.sw(avsh_at(c, SP, 0x2c), 0u64);
        c.mul_s(5, 2, 0);
        c.sw(avsh_at(c, SP, 0x3c), 0u64);
        c.mul_s(6, 4, 0);
        c.sw(avsh_at(c, SP, 0x4c), 0u64);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 7);
        c.mul_s(2, 2, 7);
        c.mul_s(4, 4, 7);
        c.mul_s(1, 1, 7);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, avsh_at(c, SP, 0x28));
        c.add_s(6, 6, 11);
        c.swc1(0, avsh_at(c, SP, 0x24));
        c.add_s(4, 4, 13);
        c.swc1(2, avsh_at(c, SP, 0x38));
        c.sub_s(10, 10, 8);
        c.swc1(9, avsh_at(c, SP, 0x48));
        c.add_s(1, 1, 11);
        c.swc1(6, avsh_at(c, SP, 0x20));
        c.add_s(3, 3, 12);
        c.swc1(4, avsh_at(c, SP, 0x30));
        c.swc1(10, avsh_at(c, SP, 0x40));
        c.swc1(1, avsh_at(c, SP, 0x34));
        c.swc1(3, avsh_at(c, SP, 0x44));
        c.lqc2(4, c.g[S0][0] as u32);
        c.lqc2(5, avsh_at(c, S0, 0x10));
        c.lqc2(6, avsh_at(c, S0, 0x20));
        c.lqc2(7, avsh_at(c, S0, 0x30));
        c.lqc2(8, c.g[FP][0] as u32);
        c.lqc2(9, avsh_at(c, FP, 0x10));
        c.lqc2(10, avsh_at(c, FP, 0x20));
        c.lqc2(11, avsh_at(c, FP, 0x30));
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
        c.sqc2(12, c.g[S0][0] as u32);
        c.sqc2(13, avsh_at(c, S0, 0x10));
        c.sqc2(14, avsh_at(c, S0, 0x20));
        c.sqc2(15, avsh_at(c, S0, 0x30));
        c.lq(3, avsh_at(c, SP, 0x10));
        c.g[A0][0] = c.lw(avsh_at(c, SP, 0xf0));
        c.lq(2, c.g[SP][0] as u32);
        c.sq(3, avsh_at(c, A0, 0x10));
        c.sq(2, c.g[A0][0] as u32);
        c.g[V0][0] = c.lw(avsh_at(c, SP, 0xfc));
        c.lqc2(1, c.g[S0][0] as u32);
        c.lqc2(2, avsh_at(c, S0, 0x10));
        c.lqc2(3, avsh_at(c, S0, 0x20));
        c.lqc2(4, avsh_at(c, S0, 0x30));
        c.sqc2(1, c.g[V0][0] as u32);
        c.sqc2(2, avsh_at(c, V0, 0x10));
        c.sqc2(3, avsh_at(c, V0, 0x20));
        c.sqc2(4, avsh_at(c, V0, 0x30));
        c.g[V0][0] = c.lw(avsh_at(c, SP, 0xf0));
        c.lq(16, avsh_at(c, SP, 0x1a0));
        c.lq(17, avsh_at(c, SP, 0x190));
        c.lq(18, avsh_at(c, SP, 0x180));
        c.lq(19, avsh_at(c, SP, 0x170));
        c.lq(20, avsh_at(c, SP, 0x160));
        c.lq(21, avsh_at(c, SP, 0x150));
        c.lq(22, avsh_at(c, SP, 0x140));
        c.lq(23, avsh_at(c, SP, 0x130));
        c.lq(30, avsh_at(c, SP, 0x120));
        c.g[RA][0] = c.ld(avsh_at(c, SP, 0x110));
        c.lwc1(26, avsh_at(c, SP, 0x1e0));
        c.lwc1(25, avsh_at(c, SP, 0x1d8));
        c.lwc1(24, avsh_at(c, SP, 0x1d0));
        c.lwc1(23, avsh_at(c, SP, 0x1c8));
        c.lwc1(22, avsh_at(c, SP, 0x1c0));
        c.lwc1(21, avsh_at(c, SP, 0x1b8));
        c.lwc1(20, avsh_at(c, SP, 0x1b0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(avsh_at(c, SP, 0x1f0));
        c.pc = t;
        Ok(())
    })
}

/// `void Avalanche_BakeNodeHistory(sAvNode* node, sAv* av)` (0x002d66a0,
/// `tAvalancheNode_calculate` in the peak3 fork): bakes one avalanche node's
/// path. It allocates the node's frame array (10 bytes per frame, `length`
/// frames), then simulates a rolling snow ball frame by frame (`D_004A3AC8`
/// sub-steps each): drag/noise on the velocity, a sphere sweep against the
/// world (`Nash_FindClosestObjectContact`) that records hit objects in the
/// av table, a second sweep for surface contact with bounce/slide response and
/// avalanche events, surface colour sampling interpolated along the path, and
/// the packed per frame colour, orientation and velocity bytes. Finally the
/// object table and the event entries are padded and sorted (`qsort`).
/// The body is the generated one (labelled blocks) with its calls through
/// `call_ptr`/`Callee`: every register, cycle and call is kept.
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn avalanche_bake_node_history(c: &mut Cpu) {
    if c.pc != 0x002d66a0 {
        return ee::s09::f_002d66a0__generated(c);
    }
    guest(c, |c| {
        'b_2d7af0: {
            c.cycles += 99;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x3e0));
            c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x380));
            c.g[T0][0] = 0xa;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x370));
            c.g[S0][0] = 0x500000;
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x330));
            c.g[S1][0] = 0x7f0000;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x320));
            c.g[S5][0] = c.g[A0][0];
            c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x3b8));
            c.g[S6][0] = c.g[A1][0];
            c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x3b0));
            c.g[A1][0] = 0x480000;
            c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x3a8));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7538));
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x3a0));
            c.g[S1][0] = c.g[S1][0] | 0xffff;
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x398));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x390));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x360));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x350));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x340));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x310));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x300));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x2f0), c.g[RA][0]);
            c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x3d0));
            c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x3c8));
            c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x3c0));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 25);
            c.lwc1(0, (c.g[S5][0] as u32).wrapping_add(0x100));
            c.g[A2][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xf8));
            c.cvt_w_s(1, 0);
            c.mfc1(4, 1);
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x84));
            c.lq(3, (c.g[A2][0] as u32).wrapping_add(0x40));
            c.g[A3][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20));
            c.mult(0, 0, 4, 8);
            c.g[A2][0] = 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c0), c.g[V0][0]);
            c.sq(3, (c.g[S5][0] as u32).wrapping_add(0xe0));
            c.swc1(25, (c.g[S5][0] as u32).wrapping_add(0xec));
            c.g[A0][0] = c.lo[0];
            c.g[RA][0] = sx(0x002d675c);
            c.sq(3, c.g[SP][0] as u32);
            call_ptr(c, 0x00317e30, 0x002d675c)?;
            c.g[V1][0] = 0x1;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x4;
            c.sb((c.g[S5][0] as u32).wrapping_add(0xf3), c.g[V1][0]);
            c.g[RA][0] = sx(0x002d6778);
            c.sw((c.g[S5][0] as u32).wrapping_add(0xfc), c.g[V0][0]);
            call_ptr(c, 0x00416210, 0x002d6778)?;
            c.g[V0][0] = 0x500000;
            c.lwc1(23, (c.g[GP][0] as u32).wrapping_add(0x990));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
            c.sw(c.g[SP][0] as u32, 0u64);
            c.lq(4, c.g[V0][0] as u32);
            c.neg_s(22, 23);
            c.g[V0][0] = 0xffffffffffffffff;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xec0));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c4), c.g[V0][0]);
            c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x3cf8));
            c.lq(2, (c.g[S5][0] as u32).wrapping_add(0xe0));
            c.lq(3, c.g[S0][0] as u32);
            c.mul_s(22, 22, 21);
            c.lwc1(24, c.g[SP][0] as u32);
            c.mul_s(21, 23, 21);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c8), 0u64);
            c.g[S0][0] = 0x3f800000;
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sub_s(21, 21, 22);
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
            c.g[RA][0] = sx(0x002d67dc);
            c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x003177f0, 0x002d67dc)?;
            c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
            c.g[V0][0] = c.g[V0][0] | c.g[S0][0];
            c.mtc1(2, 0);
            c.sub_s(20, 0, 25);
            c.mul_s(20, 21, 20);
            c.g[RA][0] = sx(0x002d67f8);
            c.add_s(20, 22, 20);
            call_ptr(c, 0x003177f0, 0x002d67f8)?;
            c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
            c.lwc1(4, (c.g[S5][0] as u32).wrapping_add(0x100));
            c.g[V0][0] = c.g[V0][0] | c.g[S0][0];
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.mtc1(2, 1);
            c.c_lt_s(24, 4);
            c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x58));
            c.sub_s(0, 1, 25);
            c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x5c));
            c.mul_s(21, 21, 0);
            c.add_s(22, 22, 21);
            let t = !c.fcc();
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x54));
            if t {
                break 'b_2d7af0;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 28);
                'l_2d6838: loop {
                    'b_2d68a0: {
                        'b_2d689c: {
                            'b_2d6874: {
                                c.cycles += 13;
                                c.lwc1(12, (c.g[S5][0] as u32).wrapping_add(0x108));
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c8));
                                c.cvt_s_w(0, 0);
                                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3cf4));
                                c.sub_s(2, 4, 12);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x60), 0u64);
                                c.mov_s(27, 0);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x6c));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x64), 0u64);
                                c.c_le_s(0, 2);
                                let t = !c.fcc();
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x68), 0u64);
                                if t {
                                    break 'b_2d6874;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.lwc1(21, (c.g[S5][0] as u32).wrapping_add(0x110));
                                    if t {
                                        break 'b_2d689c;
                                    } else {
                                        break 'b_2d6874;
                                    }
                                }
                            }
                            c.cycles += 9;
                            c.sub_s(0, 4, 27);
                            c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0x110));
                            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.mul_s(1, 1, 0);
                            c.div_s(21, 1, 12);
                            c.c_lt_s(2, 21);
                            let t = !c.fcc();
                            c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_2d68a0;
                            } else {
                                c.cycles += 1;
                                c.sqrt_s(21, 21);
                                break 'b_2d689c;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                        break 'b_2d68a0;
                    }
                    'b_2d7690: {
                        c.cycles += 17;
                        c.g[AT][0] = 0x438c0000;
                        c.mtc1(1, 0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2d4), c.g[V0][0]);
                        c.g[S0][0] = 0u64;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
                        c.min_s(21, 21, 0);
                        c.g[AT][0] = 0x42700000;
                        c.mtc1(1, 1);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9d8));
                        c.max_s(21, 21, 1);
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[AT][0] = 0xffffffffbf800000;
                        c.mtc1(1, 26);
                        let t = (c.g[V1][0] as i64) <= 0;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2dc), c.g[A0][0]);
                        if t {
                            break 'b_2d7690;
                        } else {
                            c.cycles += 17;
                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                            c.g[S7][0] = 0x7f0000;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 23);
                            c.mov_s(22, 26);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x2d0), c.g[A1][0]);
                            c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x2d8), c.g[A2][0]);
                            c.g[S4][0] = 0x7f800000;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x2cc), c.g[V0][0]);
                            c.g[S3][0] = 0x7fc00000;
                            c.g[S2][0] = 0xffffffffff800000;
                            c.g[S1][0] = 0xffffffffffc00000;
                            c.g[S7][0] = c.g[S7][0] | 0xffff;
                            'l_2d6928: loop {
                                'b_2d6e2c: {
                                    'b_2d6e28: {
                                        c.cycles += 34;
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        c.g[A0][0] = c.g[S5][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2e0), c.g[S0][0]);
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.g[RA][0] = sx(0x002d6968);
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                                        Callee {
                                            addr: 0x002d64d8,
                                            f: ee::s09::Avalanche_ClampVector,
                                        }
                                        .call(c, 0x002d6968)?;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2cc));
                                        c.mov_s(12, 21);
                                        c.sq(2, c.g[SP][0] as u32);
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x002d6988);
                                        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xc));
                                        call_ptr(c, 0x0032fb48, 0x002d6988)?;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c0));
                                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2cc));
                                        c.g[RA][0] = sx(0x002d699c);
                                        c.g[A3][0] = 0x1;
                                        call_ptr(c, 0x003378c0, 0x002d699c)?;
                                        c.cvt_w_s(1, 0);
                                        c.mfc1(2, 1);
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
                                        if t {
                                            break 'b_2d6e28;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                            let t = c.g[A0][0] == 0u64;
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
                                            if t {
                                                break 'b_2d6e2c;
                                            } else {
                                                'b_2d6a64: {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = c
                                                        .lhu((c.g[S6][0] as u32).wrapping_add(0xc));
                                                    c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x9dc)
                                                        );
                                                    if t {
                                                        break 'b_2d6a64;
                                                    } else {
                                                        'b_2d6a60: {
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x002d69d4);
                                                            call_ptr(c, 0x00350698, 0x002d69d4)?;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V1][0] = 0u64;
                                                            if t {
                                                                break 'b_2d6a60;
                                                            } else {
                                                                'b_2d6a20: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c.lhu(
                                                                        (c.g[S6][0] as u32)
                                                                            .wrapping_add(0xc),
                                                                    );
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A2][0] = c.g[V0][0];
                                                                    if t {
                                                                        break 'b_2d6a20;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[V0][0] = c.lw((c.g[S6]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        c.g[A0][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xf0));
                                                                        let t = c.g[V0][0]
                                                                            == c.g[A0][0];

                                                                        if t {
                                                                            break 'b_2d6a20;
                                                                        } else {
                                                                            c.cycles += 4;
                                                                            c.g[A3][0] = c.g[A0][0];
                                                                            c.g[A1][0] = c.g[A2][0];
                                                                            c.g[A0][0] =
                                                                                sx((c.g[S6][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x14,
                                                                                    ));
                                                                            c.g[V1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1,
                                                                                    ));
                                                                            'l_2d6a08: loop {
                                                                                c.cycles += 3;
                                                                                c.g[V0][0] = ((c.g
                                                                                    [V1][0]
                                                                                    as i64)
                                                                                    < (c.g[A1][0]
                                                                                        as i64))
                                                                                    as u64;
                                                                                let t = c.g[V0][0]
                                                                                    == 0u64;
                                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                                                                                if t {
                                                                                    break 'b_2d6a20;
                                                                                } else {
                                                                                    c.cycles += 3;
                                                                                    c.g[V0][0] = c
                                                                                        .lw(
                                                                                        c.g[A0][0]
                                                                                            as u32,
                                                                                    );
                                                                                    if c.g[V0][0]
                                                                                        != c.g[A3]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x002d6a08);
                                                                                        }
                                                                                        continue 'l_2d6a08;
                                                                                    } else {
                                                                                        break 'b_2d6a20;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[V1][0] != c.g[A2][0];
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(0x9dc));
                                                                if t {
                                                                    break 'b_2d6a64;
                                                                } else {
                                                                    c.cycles += 14;
                                                                    c.lwc1(
                                                                        0,
                                                                        (c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x3cf0),
                                                                    );
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        << 3);
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x2d4));
                                                                    c.mul_s(0, 27, 0);
                                                                    c.g[V1][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V1][0] as u32,
                                                                        ));
                                                                    c.swc1(0, c.g[V1][0] as u32);
                                                                    c.g[V0][0] = c.lhu(
                                                                        (c.g[S6][0] as u32)
                                                                            .wrapping_add(0xc),
                                                                    );
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        << 3);
                                                                    c.g[V0][0] = sx((c.g[S6][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V0][0] as u32,
                                                                        ));
                                                                    c.sw(
                                                                        (c.g[V0][0] as u32)
                                                                            .wrapping_add(0x14),
                                                                        c.g[A0][0],
                                                                    );
                                                                    c.g[V1][0] = c.lhu(
                                                                        (c.g[S6][0] as u32)
                                                                            .wrapping_add(0xc),
                                                                    );
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    c.sh(
                                                                        (c.g[S6][0] as u32)
                                                                            .wrapping_add(0xc),
                                                                        c.g[V1][0],
                                                                    );
                                                                    break 'b_2d6a60;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x9dc));
                                                        break 'b_2d6a64;
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
                                                if t {
                                                    break 'b_2d6e2c;
                                                } else {
                                                    'b_2d6b88: {
                                                        c.cycles += 65;
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.lqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.qmfc2(2, 4);
                                                        c.mtc1(2, 0);
                                                        c.g[A1][0] = 0u64;
                                                        c.mfc1(2, 0);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.qmtc2(2, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.g[A0][0] = 0u64;
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.mfc1(3, 22);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.qmtc2(3, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.vu0().rsqrt(0, 3, 4, 0);
                                                        c.vu0().waitq();
                                                        c.vu0().mul(0b1111, 5, 3, Src::Q);
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x200),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x200),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[GP][0] as u32).wrapping_add(0x9a0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.vu0().sqrt(4, 0);
                                                        c.vu0().waitq();
                                                        c.cfc2(3, 22);
                                                        c.mtc1(3, 0);
                                                        c.mul_s(0, 0, 1);
                                                        c.mfc1(3, 0);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                        );
                                                        c.qmtc2(3, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x200),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x200),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x130));
                                                        if c.g[V0][0] == c.g[S4][0] {
                                                            c.g[A0][0] = 0x1;
                                                            break 'b_2d6b88;
                                                        } else {
                                                            c.cycles += 2;
                                                            if c.g[V0][0] == c.g[S3][0] {
                                                                c.g[A0][0] = 0x1;
                                                                break 'b_2d6b88;
                                                            } else {
                                                                c.cycles += 2;
                                                                if c.g[V0][0] == c.g[S2][0] {
                                                                    c.g[A0][0] = 0x1;
                                                                    break 'b_2d6b88;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    if c.g[V0][0] == c.g[S1][0] {
                                                                        c.g[A0][0] = 0x1;
                                                                        break 'b_2d6b88;
                                                                    } else {
                                                                        break 'b_2d6b88;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2d6c20: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] != 0u64 {
                                                            c.g[A1][0] = 0x1;
                                                            break 'b_2d6c20;
                                                        } else {
                                                            'b_2d6bb8: {
                                                                'b_2d6bb4: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x134));
                                                                    let t =
                                                                        c.g[V0][0] == c.g[S4][0];
                                                                    c.g[V1][0] = 0u64;
                                                                    if t {
                                                                        break 'b_2d6bb4;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] == c.g[S3][0]
                                                                        {
                                                                            c.g[V1][0] = 0x1;
                                                                            break 'b_2d6bb8;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S2][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d6bb8;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = c.g[V0][0]
                                                                                    != c.g[S1][0];

                                                                                if t {
                                                                                    break 'b_2d6bb8;
                                                                                } else {
                                                                                    break 'b_2d6bb4;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V1][0] = 0x1;
                                                                break 'b_2d6bb8;
                                                            }
                                                            c.cycles += 2;
                                                            if c.g[V1][0] != 0u64 {
                                                                c.g[A1][0] = 0x1;
                                                                break 'b_2d6c20;
                                                            } else {
                                                                'b_2d6be8: {
                                                                    'b_2d6be4: {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x138));
                                                                        let t = c.g[V0][0]
                                                                            == c.g[S4][0];
                                                                        c.g[V1][0] = 0u64;
                                                                        if t {
                                                                            break 'b_2d6be4;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S3][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d6be8;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S2][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d6be8;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S1]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_2d6be8;
                                                                                    } else {
                                                                                        break 'b_2d6be4;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_2d6be8;
                                                                }
                                                                c.cycles += 2;
                                                                if c.g[V1][0] != 0u64 {
                                                                    c.g[A1][0] = 0x1;
                                                                    break 'b_2d6c20;
                                                                } else {
                                                                    'b_2d6c18: {
                                                                        'b_2d6c14: {
                                                                            c.cycles += 3;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x13c,
                                                                                ));
                                                                            let t = c.g[V0][0]
                                                                                == c.g[S4][0];
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_2d6c14;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S3][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d6c18;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    if c.g[V0][0]
                                                                                        == c.g[S2]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        break 'b_2d6c18;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t = c.g[V0][0] != c.g[S1][0];

                                                                                        if t {
                                                                                            break 'b_2d6c18;
                                                                                        } else {
                                                                                            break 'b_2d6c14;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 1;
                                                                        c.g[V1][0] = 0x1;
                                                                        break 'b_2d6c18;
                                                                    }
                                                                    c.cycles += 2;
                                                                    if c.g[V1][0] != 0u64 {
                                                                        c.g[A1][0] = 0x1;
                                                                        break 'b_2d6c20;
                                                                    } else {
                                                                        break 'b_2d6c20;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2d6c34: {
                                                        c.cycles += 2;
                                                        let t = c.g[A1][0] == 0u64;
                                                        c.g[V1][0] = 0x500000;
                                                        if t {
                                                            break 'b_2d6c34;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0xee0));
                                                            c.lq(2, c.g[V1][0] as u32);
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x130),
                                                            );
                                                            break 'b_2d6c34;
                                                        }
                                                    }
                                                    'b_2d6d30: {
                                                        'b_2d6d2c: {
                                                            c.cycles += 56;
                                                            c.lqc2(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1e0),
                                                            );
                                                            c.lqc2(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x130),
                                                            );
                                                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                            c.sqc2(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1e0),
                                                            );
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.lwc1(
                                                                0,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x99c),
                                                            );
                                                            c.g[A1][0] = c.g[FP][0];
                                                            c.mfc1(3, 0);
                                                            c.lqc2(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1e0),
                                                            );
                                                            c.qmtc2(3, 3);
                                                            c.vu0().mul(
                                                                0b1111,
                                                                5,
                                                                4,
                                                                Src::Bc(3, X),
                                                            );
                                                            c.sqc2(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x200),
                                                            );
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x200),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1f0),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            c.mfc1(3, 21);
                                                            c.lqc2(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xb0),
                                                            );
                                                            c.qmtc2(3, 3);
                                                            c.vu0().mul(
                                                                0b1111,
                                                                5,
                                                                4,
                                                                Src::Bc(3, X),
                                                            );
                                                            c.sqc2(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x200),
                                                            );
                                                            c.lqc2(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xa0),
                                                            );
                                                            c.lqc2(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x200),
                                                            );
                                                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                            c.sqc2(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            c.g[RA][0] = sx(0x002d6cac);
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1f0),
                                                            );
                                                            Callee {
                                                                addr: 0x002d64d8,
                                                                f: ee::s09::Avalanche_ClampVector,
                                                            }
                                                            .call(c, 0x002d6cac)?;
                                                            c.lqc2(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            c.lqc2(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xb0),
                                                            );
                                                            c.vu0().opmula(4, 5);
                                                            c.vu0().opmsub(6, 5, 4);
                                                            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                                                            c.sqc2(
                                                                6,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.g[A0][0] = 0u64;
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.g[V1][0] = 0u64;
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x200),
                                                            );
                                                            c.lqc2(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x200),
                                                            );
                                                            c.vu0().add(
                                                                0b1000,
                                                                6,
                                                                0,
                                                                Src::Bc(0, W),
                                                            );
                                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                            c.vu0().madd(
                                                                0b1000,
                                                                4,
                                                                6,
                                                                Src::Bc(4, W),
                                                            );
                                                            c.vu0().rsqrt(0, 3, 4, 0);
                                                            c.vu0().waitq();
                                                            c.vu0().mul(0b1111, 5, 3, Src::Q);
                                                            c.sqc2(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1f0),
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                            let t = c.g[V0][0] == c.g[S4][0];
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1c),
                                                                0u64,
                                                            );
                                                            if t {
                                                                break 'b_2d6d2c;
                                                            } else {
                                                                c.cycles += 2;
                                                                if c.g[V0][0] == c.g[S3][0] {
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_2d6d30;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    if c.g[V0][0] == c.g[S2][0] {
                                                                        c.g[V1][0] = 0x1;
                                                                        break 'b_2d6d30;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = c.g[V0][0]
                                                                            != c.g[S1][0];

                                                                        if t {
                                                                            break 'b_2d6d30;
                                                                        } else {
                                                                            break 'b_2d6d2c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V1][0] = 0x1;
                                                        break 'b_2d6d30;
                                                    }
                                                    'b_2d6dc8: {
                                                        c.cycles += 2;
                                                        if c.g[V1][0] != 0u64 {
                                                            c.g[A0][0] = 0x1;
                                                            break 'b_2d6dc8;
                                                        } else {
                                                            'b_2d6d60: {
                                                                'b_2d6d5c: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    let t =
                                                                        c.g[V0][0] == c.g[S4][0];
                                                                    c.g[V1][0] = 0u64;
                                                                    if t {
                                                                        break 'b_2d6d5c;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] == c.g[S3][0]
                                                                        {
                                                                            c.g[V1][0] = 0x1;
                                                                            break 'b_2d6d60;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S2][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d6d60;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = c.g[V0][0]
                                                                                    != c.g[S1][0];

                                                                                if t {
                                                                                    break 'b_2d6d60;
                                                                                } else {
                                                                                    break 'b_2d6d5c;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V1][0] = 0x1;
                                                                break 'b_2d6d60;
                                                            }
                                                            c.cycles += 2;
                                                            if c.g[V1][0] != 0u64 {
                                                                c.g[A0][0] = 0x1;
                                                                break 'b_2d6dc8;
                                                            } else {
                                                                'b_2d6d90: {
                                                                    'b_2d6d8c: {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        let t = c.g[V0][0]
                                                                            == c.g[S4][0];
                                                                        c.g[V1][0] = 0u64;
                                                                        if t {
                                                                            break 'b_2d6d8c;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S3][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d6d90;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S2][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d6d90;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S1]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_2d6d90;
                                                                                    } else {
                                                                                        break 'b_2d6d8c;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_2d6d90;
                                                                }
                                                                c.cycles += 2;
                                                                if c.g[V1][0] != 0u64 {
                                                                    c.g[A0][0] = 0x1;
                                                                    break 'b_2d6dc8;
                                                                } else {
                                                                    'b_2d6dc0: {
                                                                        'b_2d6dbc: {
                                                                            c.cycles += 3;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1c,
                                                                                ));
                                                                            let t = c.g[V0][0]
                                                                                == c.g[S4][0];
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_2d6dbc;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S3][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d6dc0;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    if c.g[V0][0]
                                                                                        == c.g[S2]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        break 'b_2d6dc0;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t = c.g[V0][0] != c.g[S1][0];

                                                                                        if t {
                                                                                            break 'b_2d6dc0;
                                                                                        } else {
                                                                                            break 'b_2d6dbc;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 1;
                                                                        c.g[V1][0] = 0x1;
                                                                        break 'b_2d6dc0;
                                                                    }
                                                                    c.cycles += 2;
                                                                    if c.g[V1][0] != 0u64 {
                                                                        c.g[A0][0] = 0x1;
                                                                        break 'b_2d6dc8;
                                                                    } else {
                                                                        break 'b_2d6dc8;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2d6e20: {
                                                        'b_2d6de4: {
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[A0][0] = 0x500000;
                                                            if t {
                                                                break 'b_2d6de4;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.mtc1(0, 24);
                                                                c.g[V1][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0xec0));
                                                                c.lq(2, c.g[V1][0] as u32);
                                                                let t = true;
                                                                c.sq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                if t {
                                                                    break 'b_2d6e20;
                                                                } else {
                                                                    break 'b_2d6de4;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 15;
                                                        c.lwc1(
                                                            1,
                                                            (c.g[GP][0] as u32).wrapping_add(0x978),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.vu0().sqrt(4, 0);
                                                        c.vu0().waitq();
                                                        c.cfc2(2, 22);
                                                        c.mtc1(2, 2);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x3cec),
                                                        );
                                                        c.mul_s(1, 1, 2);
                                                        c.mul_s(0, 21, 0);
                                                        c.div_s(24, 1, 0);
                                                        break 'b_2d6e20;
                                                    }
                                                    c.cycles += 2;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x3ce8),
                                                    );
                                                    c.min_s(24, 24, 0);
                                                    break 'b_2d6e28;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
                                    break 'b_2d6e2c;
                                }
                                'b_2d7654: {
                                    'b_2d7650: {
                                        c.cycles += 15;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.mov_s(12, 21);
                                        c.g[RA][0] = sx(0x002d6e3c);
                                        c.g[A2][0] = 0u64;
                                        call_ptr(c, 0x0032fb48, 0x002d6e3c)?;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c0));
                                        c.g[A1][0] = c.g[S0][0];
                                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d8));
                                        c.g[RA][0] = sx(0x002d6e50);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00336850, 0x002d6e50)?;
                                        c.mtc1(0, 1);
                                        c.mov_s(26, 0);
                                        c.c_le_s(1, 26);
                                        let t = !c.fcc();
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xec));
                                        if t {
                                            break 'b_2d7650;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0xffffffffffffffff;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x994));
                                            if t {
                                                break 'b_2d7654;
                                            } else {
                                                'b_2d6ebc: {
                                                    c.cycles += 15;
                                                    c.lqc2(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                    );
                                                    c.lqc2(
                                                        5,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.qmfc2(2, 4);
                                                    c.mtc1(2, 0);
                                                    c.lwc1(
                                                        1,
                                                        (c.g[GP][0] as u32).wrapping_add(0x960),
                                                    );
                                                    c.abs_s(0, 0);
                                                    c.c_lt_s(1, 0);
                                                    let t = !c.fcc();
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x2c8)
                                                        );
                                                    if t {
                                                        break 'b_2d6ebc;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A0][0] = c.g[S6][0];
                                                        c.g[RA][0] = sx(0x002d6ebc);
                                                        c.g[A2][0] = c.g[S5][0];
                                                        Callee {
                                                            addr: 0x002d9738,
                                                            f: ee::s09::Avalanche_AddNodeEvent,
                                                        }
                                                        .call(c, 0x002d6ebc)?;
                                                        break 'b_2d6ebc;
                                                    }
                                                }
                                                'b_2d7470: {
                                                    'b_2d7270: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c.lhu(
                                                            (c.g[S5][0] as u32).wrapping_add(0xf0),
                                                        );
                                                        let t = c.g[V0][0] != 0u64;

                                                        if t {
                                                            break 'b_2d7270;
                                                        } else {
                                                            c.cycles += 11;
                                                            c.g[RA][0] = sx(0x002d6ed0);
                                                            call_ptr(c, 0x003177f0, 0x002d6ed0)?;
                                                            c.g[V0][0] = c.g[V0][0] & c.g[S7][0];
                                                            c.g[A1][0] = 0x3f800000;
                                                            c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                                                            c.lwc1(
                                                                1,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x97c),
                                                            );
                                                            c.mtc1(2, 2);
                                                            c.sub_s(0, 2, 23);
                                                            c.c_lt_s(0, 1);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_2d7270;
                                                            } else {
                                                                'b_2d70d0: {
                                                                    c.cycles += 113;
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xb0),
                                                                    );
                                                                    c.lqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x20),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1000,
                                                                        6,
                                                                        0,
                                                                        Src::Bc(0, W),
                                                                    );
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        4,
                                                                        3,
                                                                        Src::Vec(5),
                                                                    );
                                                                    c.vu0().adda(
                                                                        0b1000,
                                                                        4,
                                                                        Src::Bc(4, Y),
                                                                    );
                                                                    c.vu0().madda(
                                                                        0b1000,
                                                                        6,
                                                                        Src::Bc(4, Z),
                                                                    );
                                                                    c.vu0().madd(
                                                                        0b1000,
                                                                        4,
                                                                        6,
                                                                        Src::Bc(4, W),
                                                                    );
                                                                    c.qmfc2(2, 4);
                                                                    c.mtc1(2, 0);
                                                                    c.g[A1][0] = 0u64;
                                                                    c.mfc1(2, 0);
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xb0),
                                                                    );
                                                                    c.qmtc2(2, 3);
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        4,
                                                                        Src::Bc(3, X),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.g[A0][0] = 0u64;
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.mfc1(3, 22);
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.qmtc2(3, 3);
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        4,
                                                                        Src::Bc(3, X),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x120),
                                                                    );
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x20),
                                                                    );
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x120),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1111,
                                                                        5,
                                                                        3,
                                                                        Src::Vec(4),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x60),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x60),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1000,
                                                                        6,
                                                                        0,
                                                                        Src::Bc(0, W),
                                                                    );
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        4,
                                                                        3,
                                                                        Src::Vec(3),
                                                                    );
                                                                    c.vu0().adda(
                                                                        0b1000,
                                                                        4,
                                                                        Src::Bc(4, Y),
                                                                    );
                                                                    c.vu0().madda(
                                                                        0b1000,
                                                                        6,
                                                                        Src::Bc(4, Z),
                                                                    );
                                                                    c.vu0().madd(
                                                                        0b1000,
                                                                        4,
                                                                        6,
                                                                        Src::Bc(4, W),
                                                                    );
                                                                    c.vu0().rsqrt(0, 3, 4, 0);
                                                                    c.vu0().waitq();
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        3,
                                                                        Src::Q,
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lwc1(
                                                                        1,
                                                                        (c.g[GP][0] as u32)
                                                                            .wrapping_add(0x9a0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x120),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1000,
                                                                        6,
                                                                        0,
                                                                        Src::Bc(0, W),
                                                                    );
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        4,
                                                                        3,
                                                                        Src::Vec(3),
                                                                    );
                                                                    c.vu0().adda(
                                                                        0b1000,
                                                                        4,
                                                                        Src::Bc(4, Y),
                                                                    );
                                                                    c.vu0().madda(
                                                                        0b1000,
                                                                        6,
                                                                        Src::Bc(4, Z),
                                                                    );
                                                                    c.vu0().madd(
                                                                        0b1000,
                                                                        4,
                                                                        6,
                                                                        Src::Bc(4, W),
                                                                    );
                                                                    c.vu0().sqrt(4, 0);
                                                                    c.vu0().waitq();
                                                                    c.cfc2(3, 22);
                                                                    c.mtc1(3, 0);
                                                                    c.mul_s(0, 0, 1);
                                                                    c.mfc1(3, 0);
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.qmtc2(3, 3);
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        4,
                                                                        Src::Bc(3, X),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x60),
                                                                    );
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1111,
                                                                        5,
                                                                        3,
                                                                        Src::Vec(4),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x60),
                                                                    );
                                                                    c.lwc1(
                                                                        0,
                                                                        (c.g[GP][0] as u32)
                                                                            .wrapping_add(0x998),
                                                                    );
                                                                    c.mfc1(3, 0);
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x120),
                                                                    );
                                                                    c.qmtc2(3, 3);
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        4,
                                                                        Src::Bc(3, X),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lwc1(
                                                                        0,
                                                                        (c.g[GP][0] as u32)
                                                                            .wrapping_add(0x99c),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.mfc1(3, 0);
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x60),
                                                                    );
                                                                    c.qmtc2(3, 3);
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        4,
                                                                        Src::Bc(3, X),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x200),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x200),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1111,
                                                                        5,
                                                                        3,
                                                                        Src::Vec(4),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x200),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x200),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x20),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xb0),
                                                                    );
                                                                    c.lqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x60),
                                                                    );
                                                                    c.vu0().opmula(4, 5);
                                                                    c.vu0().opmsub(6, 5, 4);
                                                                    c.vu0().sub(
                                                                        0b0001,
                                                                        6,
                                                                        6,
                                                                        Src::Vec(6),
                                                                    );
                                                                    c.sqc2(
                                                                        6,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1000,
                                                                        6,
                                                                        0,
                                                                        Src::Bc(0, W),
                                                                    );
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        4,
                                                                        3,
                                                                        Src::Vec(3),
                                                                    );
                                                                    c.vu0().adda(
                                                                        0b1000,
                                                                        4,
                                                                        Src::Bc(4, Y),
                                                                    );
                                                                    c.vu0().madda(
                                                                        0b1000,
                                                                        6,
                                                                        Src::Bc(4, Z),
                                                                    );
                                                                    c.vu0().madd(
                                                                        0b1000,
                                                                        4,
                                                                        6,
                                                                        Src::Bc(4, W),
                                                                    );
                                                                    c.vu0().rsqrt(0, 3, 4, 0);
                                                                    c.vu0().waitq();
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        3,
                                                                        Src::Q,
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130));
                                                                    if c.g[V0][0] == c.g[S4][0] {
                                                                        c.g[A0][0] = 0x1;
                                                                        break 'b_2d70d0;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] == c.g[S3][0]
                                                                        {
                                                                            c.g[A0][0] = 0x1;
                                                                            break 'b_2d70d0;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S2][0]
                                                                            {
                                                                                c.g[A0][0] = 0x1;
                                                                                break 'b_2d70d0;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S1][0]
                                                                                {
                                                                                    c.g[A0][0] =
                                                                                        0x1;
                                                                                    break 'b_2d70d0;
                                                                                } else {
                                                                                    break 'b_2d70d0;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_2d7168: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] != 0u64 {
                                                                        c.g[A1][0] = 0x1;
                                                                        break 'b_2d7168;
                                                                    } else {
                                                                        'b_2d7100: {
                                                                            'b_2d70fc: {
                                                                                c.cycles += 3;
                                                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x134));
                                                                                let t = c.g[V0][0]
                                                                                    == c.g[S4][0];
                                                                                c.g[V1][0] = 0u64;
                                                                                if t {
                                                                                    break 'b_2d70fc;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    if c.g[V0][0]
                                                                                        == c.g[S3]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        break 'b_2d7100;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        if c.g[V0]
                                                                                            [0]
                                                                                            == c.g
                                                                                                [S2]
                                                                                                [0]
                                                                                        {
                                                                                            c.g[V1][0] = 0x1;
                                                                                            break 'b_2d7100;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            let t = c.g[V0][0] != c.g[S1][0];

                                                                                            if t {
                                                                                                break 'b_2d7100;
                                                                                            } else {
                                                                                                break 'b_2d70fc;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 1;
                                                                            c.g[V1][0] = 0x1;
                                                                            break 'b_2d7100;
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[V1][0] != 0u64 {
                                                                            c.g[A1][0] = 0x1;
                                                                            break 'b_2d7168;
                                                                        } else {
                                                                            'b_2d7130: {
                                                                                'b_2d712c: {
                                                                                    c.cycles += 3;
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        == c.g[S4]
                                                                                            [0];
                                                                                    c.g[V1][0] =
                                                                                        0u64;
                                                                                    if t {
                                                                                        break 'b_2d712c;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        if c.g[V0]
                                                                                            [0]
                                                                                            == c.g
                                                                                                [S3]
                                                                                                [0]
                                                                                        {
                                                                                            c.g[V1][0] = 0x1;
                                                                                            break 'b_2d7130;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d7130;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d7130;
} else {
break 'b_2d712c;
}
}
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 1;
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d7130;
                                                                            }
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] != 0u64 {
                                                                                c.g[A1][0] = 0x1;
                                                                                break 'b_2d7168;
                                                                            } else {
                                                                                'b_2d7160: {
                                                                                    'b_2d715c: {
                                                                                        c.cycles +=
                                                                                            3;
                                                                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x13c));
                                                                                        let t = c.g[V0][0] == c.g[S4][0];
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0u64;
                                                                                        if t {
                                                                                            break 'b_2d715c;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d7160;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d7160;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d7160;
} else {
break 'b_2d715c;
}
}
}
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 1;
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d7160;
                                                                                }
                                                                                c.cycles += 2;
                                                                                if c.g[V1][0]
                                                                                    != 0u64
                                                                                {
                                                                                    c.g[A1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d7168;
                                                                                } else {
                                                                                    break 'b_2d7168;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_2d717c: {
                                                                    c.cycles += 2;
                                                                    let t = c.g[A1][0] == 0u64;
                                                                    c.g[V1][0] = 0x500000;
                                                                    if t {
                                                                        break 'b_2d717c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0xee0,
                                                                                ));
                                                                        c.lq(2, c.g[V1][0] as u32);
                                                                        c.sq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x130,
                                                                                ),
                                                                        );
                                                                        break 'b_2d717c;
                                                                    }
                                                                }
                                                                c.cycles += 61;
                                                                c.g[RA][0] = sx(0x002d7184);
                                                                c.lwc1(
                                                                    20,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x980),
                                                                );
                                                                call_ptr(
                                                                    c, 0x003177f0, 0x002d7184,
                                                                )?;
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] & c.g[S7][0];
                                                                c.g[A0][0] = 0x3f800000;
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] | c.g[A0][0];
                                                                c.neg_s(0, 20);
                                                                c.mtc1(2, 2);
                                                                c.sub_s(20, 20, 0);
                                                                c.sub_s(1, 2, 23);
                                                                c.mul_s(20, 20, 1);
                                                                c.add_s(0, 0, 20);
                                                                c.mfc1(3, 0);
                                                                c.lqc2(
                                                                    4,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x130),
                                                                );
                                                                c.qmtc2(3, 3);
                                                                c.vu0().mul(
                                                                    0b1111,
                                                                    5,
                                                                    4,
                                                                    Src::Bc(3, X),
                                                                );
                                                                c.sqc2(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1f0),
                                                                );
                                                                c.lq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1f0),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1e0),
                                                                );
                                                                c.lqc2(
                                                                    3,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lqc2(
                                                                    4,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1e0),
                                                                );
                                                                c.vu0().add(
                                                                    0b1111,
                                                                    5,
                                                                    3,
                                                                    Src::Vec(4),
                                                                );
                                                                c.sqc2(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lqc2(
                                                                    3,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xb0),
                                                                );
                                                                c.vu0().add(
                                                                    0b1000,
                                                                    6,
                                                                    0,
                                                                    Src::Bc(0, W),
                                                                );
                                                                c.vu0().mul(
                                                                    0b1111,
                                                                    4,
                                                                    3,
                                                                    Src::Vec(3),
                                                                );
                                                                c.vu0().adda(
                                                                    0b1000,
                                                                    4,
                                                                    Src::Bc(4, Y),
                                                                );
                                                                c.vu0().madda(
                                                                    0b1000,
                                                                    6,
                                                                    Src::Bc(4, Z),
                                                                );
                                                                c.vu0().madd(
                                                                    0b1000,
                                                                    4,
                                                                    6,
                                                                    Src::Bc(4, W),
                                                                );
                                                                c.vu0().rsqrt(0, 3, 4, 0);
                                                                c.vu0().waitq();
                                                                c.vu0().mul(0b1111, 5, 3, Src::Q);
                                                                c.sqc2(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x200),
                                                                );
                                                                c.lq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x200),
                                                                );
                                                                c.lwc1(
                                                                    20,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x984),
                                                                );
                                                                c.g[RA][0] = sx(0x002d720c);
                                                                c.sq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1f0),
                                                                );
                                                                call_ptr(
                                                                    c, 0x003177f0, 0x002d720c,
                                                                )?;
                                                                c.g[V1][0] = 0x3f800000;
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] & c.g[S7][0];
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] | c.g[V1][0];
                                                                c.mtc1(2, 1);
                                                                c.sub_s(0, 1, 23);
                                                                c.mul_s(20, 20, 0);
                                                                c.mfc1(3, 20);
                                                                c.lqc2(
                                                                    4,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1f0),
                                                                );
                                                                c.qmtc2(3, 3);
                                                                c.vu0().mul(
                                                                    0b1111,
                                                                    5,
                                                                    4,
                                                                    Src::Bc(3, X),
                                                                );
                                                                c.sqc2(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x200),
                                                                );
                                                                c.lq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x200),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1e0),
                                                                );
                                                                c.lqc2(
                                                                    3,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lqc2(
                                                                    4,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1e0),
                                                                );
                                                                c.vu0().add(
                                                                    0b1111,
                                                                    5,
                                                                    3,
                                                                    Src::Vec(4),
                                                                );
                                                                c.sqc2(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x988),
                                                                );
                                                                c.mfc1(2, 0);
                                                                c.lqc2(
                                                                    4,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.qmtc2(2, 3);
                                                                c.vu0().mul(
                                                                    0b1111,
                                                                    5,
                                                                    4,
                                                                    Src::Bc(3, X),
                                                                );
                                                                c.sqc2(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                let t = true;

                                                                if t {
                                                                    break 'b_2d7470;
                                                                } else {
                                                                    break 'b_2d7270;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2d7390: {
                                                        c.cycles += 66;
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.lqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.qmfc2(2, 4);
                                                        c.mtc1(2, 0);
                                                        c.g[A1][0] = 0u64;
                                                        c.mfc1(2, 0);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.qmtc2(2, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.g[A0][0] = 0u64;
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.mfc1(3, 22);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.qmtc2(3, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x60),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x60),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.vu0().rsqrt(0, 3, 4, 0);
                                                        c.vu0().waitq();
                                                        c.vu0().mul(0b1111, 5, 3, Src::Q);
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[GP][0] as u32).wrapping_add(0x9a0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.vu0().sqrt(4, 0);
                                                        c.vu0().waitq();
                                                        c.cfc2(3, 22);
                                                        c.mtc1(3, 0);
                                                        c.mul_s(0, 0, 1);
                                                        c.mfc1(3, 0);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.qmtc2(3, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x130));
                                                        if c.g[V0][0] == c.g[S4][0] {
                                                            c.g[A0][0] = 0x1;
                                                            break 'b_2d7390;
                                                        } else {
                                                            c.cycles += 2;
                                                            if c.g[V0][0] == c.g[S3][0] {
                                                                c.g[A0][0] = 0x1;
                                                                break 'b_2d7390;
                                                            } else {
                                                                c.cycles += 2;
                                                                if c.g[V0][0] == c.g[S2][0] {
                                                                    c.g[A0][0] = 0x1;
                                                                    break 'b_2d7390;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    if c.g[V0][0] == c.g[S1][0] {
                                                                        c.g[A0][0] = 0x1;
                                                                        break 'b_2d7390;
                                                                    } else {
                                                                        break 'b_2d7390;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2d7428: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] != 0u64 {
                                                            c.g[A1][0] = 0x1;
                                                            break 'b_2d7428;
                                                        } else {
                                                            'b_2d73c0: {
                                                                'b_2d73bc: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x134));
                                                                    let t =
                                                                        c.g[V0][0] == c.g[S4][0];
                                                                    c.g[V1][0] = 0u64;
                                                                    if t {
                                                                        break 'b_2d73bc;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] == c.g[S3][0]
                                                                        {
                                                                            c.g[V1][0] = 0x1;
                                                                            break 'b_2d73c0;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S2][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d73c0;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = c.g[V0][0]
                                                                                    != c.g[S1][0];

                                                                                if t {
                                                                                    break 'b_2d73c0;
                                                                                } else {
                                                                                    break 'b_2d73bc;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V1][0] = 0x1;
                                                                break 'b_2d73c0;
                                                            }
                                                            c.cycles += 2;
                                                            if c.g[V1][0] != 0u64 {
                                                                c.g[A1][0] = 0x1;
                                                                break 'b_2d7428;
                                                            } else {
                                                                'b_2d73f0: {
                                                                    'b_2d73ec: {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x138));
                                                                        let t = c.g[V0][0]
                                                                            == c.g[S4][0];
                                                                        c.g[V1][0] = 0u64;
                                                                        if t {
                                                                            break 'b_2d73ec;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S3][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d73f0;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S2][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d73f0;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S1]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_2d73f0;
                                                                                    } else {
                                                                                        break 'b_2d73ec;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_2d73f0;
                                                                }
                                                                c.cycles += 2;
                                                                if c.g[V1][0] != 0u64 {
                                                                    c.g[A1][0] = 0x1;
                                                                    break 'b_2d7428;
                                                                } else {
                                                                    'b_2d7420: {
                                                                        'b_2d741c: {
                                                                            c.cycles += 3;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x13c,
                                                                                ));
                                                                            let t = c.g[V0][0]
                                                                                == c.g[S4][0];
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_2d741c;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S3][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d7420;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    if c.g[V0][0]
                                                                                        == c.g[S2]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        break 'b_2d7420;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t = c.g[V0][0] != c.g[S1][0];

                                                                                        if t {
                                                                                            break 'b_2d7420;
                                                                                        } else {
                                                                                            break 'b_2d741c;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 1;
                                                                        c.g[V1][0] = 0x1;
                                                                        break 'b_2d7420;
                                                                    }
                                                                    c.cycles += 2;
                                                                    if c.g[V1][0] != 0u64 {
                                                                        c.g[A1][0] = 0x1;
                                                                        break 'b_2d7428;
                                                                    } else {
                                                                        break 'b_2d7428;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2d743c: {
                                                        c.cycles += 2;
                                                        let t = c.g[A1][0] == 0u64;
                                                        c.g[V1][0] = 0x500000;
                                                        if t {
                                                            break 'b_2d743c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0xee0));
                                                            c.lq(2, c.g[V1][0] as u32);
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x130),
                                                            );
                                                            break 'b_2d743c;
                                                        }
                                                    }
                                                    c.cycles += 13;
                                                    c.lqc2(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x60),
                                                    );
                                                    c.lqc2(
                                                        4,
                                                        (c.g[SP][0] as u32).wrapping_add(0x130),
                                                    );
                                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                    c.sqc2(
                                                        5,
                                                        (c.g[SP][0] as u32).wrapping_add(0x60),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_add(0x99c),
                                                    );
                                                    c.mfc1(3, 0);
                                                    c.lqc2(
                                                        4,
                                                        (c.g[SP][0] as u32).wrapping_add(0x60),
                                                    );
                                                    c.qmtc2(3, 3);
                                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                    c.sqc2(
                                                        5,
                                                        (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                    );
                                                    c.lq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                    );
                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                    c.sq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                    );
                                                    break 'b_2d7470;
                                                }
                                                'b_2d7558: {
                                                    'b_2d7554: {
                                                        c.cycles += 51;
                                                        c.mfc1(2, 21);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.qmtc2(2, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.g[A0][0] = c.g[S5][0];
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.g[A1][0] = c.g[FP][0];
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xa0),
                                                        );
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[GP][0] as u32).wrapping_add(0x994),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x30),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.sub_s(0, 23, 0);
                                                        c.mfc1(3, 0);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.qmtc2(3, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.g[RA][0] = sx(0x002d74d4);
                                                        Callee {
                                                            addr: 0x002d64d8,
                                                            f: ee::s09::Avalanche_ClampVector,
                                                        }
                                                        .call(c, 0x002d74d4)?;
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.vu0().opmula(4, 5);
                                                        c.vu0().opmsub(6, 5, 4);
                                                        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                                                        c.sqc2(
                                                            6,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.g[A0][0] = 0u64;
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.g[V1][0] = 0u64;
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.vu0().rsqrt(0, 3, 4, 0);
                                                        c.vu0().waitq();
                                                        c.vu0().mul(0b1111, 5, 3, Src::Q);
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                        let t = c.g[V0][0] == c.g[S4][0];
                                                        c.sw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x1c),
                                                            0u64,
                                                        );
                                                        if t {
                                                            break 'b_2d7554;
                                                        } else {
                                                            c.cycles += 2;
                                                            if c.g[V0][0] == c.g[S3][0] {
                                                                c.g[V1][0] = 0x1;
                                                                break 'b_2d7558;
                                                            } else {
                                                                c.cycles += 2;
                                                                if c.g[V0][0] == c.g[S2][0] {
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_2d7558;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t =
                                                                        c.g[V0][0] != c.g[S1][0];

                                                                    if t {
                                                                        break 'b_2d7558;
                                                                    } else {
                                                                        break 'b_2d7554;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[V1][0] = 0x1;
                                                    break 'b_2d7558;
                                                }
                                                'b_2d75f0: {
                                                    c.cycles += 2;
                                                    if c.g[V1][0] != 0u64 {
                                                        c.g[A0][0] = 0x1;
                                                        break 'b_2d75f0;
                                                    } else {
                                                        'b_2d7588: {
                                                            'b_2d7584: {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x14));
                                                                let t = c.g[V0][0] == c.g[S4][0];
                                                                c.g[V1][0] = 0u64;
                                                                if t {
                                                                    break 'b_2d7584;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    if c.g[V0][0] == c.g[S3][0] {
                                                                        c.g[V1][0] = 0x1;
                                                                        break 'b_2d7588;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] == c.g[S2][0]
                                                                        {
                                                                            c.g[V1][0] = 0x1;
                                                                            break 'b_2d7588;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = c.g[V0][0]
                                                                                != c.g[S1][0];

                                                                            if t {
                                                                                break 'b_2d7588;
                                                                            } else {
                                                                                break 'b_2d7584;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 1;
                                                            c.g[V1][0] = 0x1;
                                                            break 'b_2d7588;
                                                        }
                                                        c.cycles += 2;
                                                        if c.g[V1][0] != 0u64 {
                                                            c.g[A0][0] = 0x1;
                                                            break 'b_2d75f0;
                                                        } else {
                                                            'b_2d75b8: {
                                                                'b_2d75b4: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    let t =
                                                                        c.g[V0][0] == c.g[S4][0];
                                                                    c.g[V1][0] = 0u64;
                                                                    if t {
                                                                        break 'b_2d75b4;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] == c.g[S3][0]
                                                                        {
                                                                            c.g[V1][0] = 0x1;
                                                                            break 'b_2d75b8;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S2][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d75b8;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = c.g[V0][0]
                                                                                    != c.g[S1][0];

                                                                                if t {
                                                                                    break 'b_2d75b8;
                                                                                } else {
                                                                                    break 'b_2d75b4;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V1][0] = 0x1;
                                                                break 'b_2d75b8;
                                                            }
                                                            c.cycles += 2;
                                                            if c.g[V1][0] != 0u64 {
                                                                c.g[A0][0] = 0x1;
                                                                break 'b_2d75f0;
                                                            } else {
                                                                'b_2d75e8: {
                                                                    'b_2d75e4: {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        let t = c.g[V0][0]
                                                                            == c.g[S4][0];
                                                                        c.g[V1][0] = 0u64;
                                                                        if t {
                                                                            break 'b_2d75e4;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S3][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d75e8;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S2][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d75e8;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S1]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_2d75e8;
                                                                                    } else {
                                                                                        break 'b_2d75e4;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_2d75e8;
                                                                }
                                                                c.cycles += 2;
                                                                if c.g[V1][0] != 0u64 {
                                                                    c.g[A0][0] = 0x1;
                                                                    break 'b_2d75f0;
                                                                } else {
                                                                    break 'b_2d75f0;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                'b_2d7648: {
                                                    'b_2d760c: {
                                                        c.cycles += 2;
                                                        let t = c.g[A0][0] == 0u64;
                                                        c.g[A0][0] = 0x500000;
                                                        if t {
                                                            break 'b_2d760c;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.mtc1(0, 24);
                                                            c.g[V1][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0xec0));
                                                            c.lq(2, c.g[V1][0] as u32);
                                                            let t = true;
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            if t {
                                                                break 'b_2d7648;
                                                            } else {
                                                                break 'b_2d760c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 15;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[GP][0] as u32).wrapping_add(0x978),
                                                    );
                                                    c.lqc2(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.vu0().sqrt(4, 0);
                                                    c.vu0().waitq();
                                                    c.cfc2(2, 22);
                                                    c.mtc1(2, 2);
                                                    c.lwc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x3ce4),
                                                    );
                                                    c.mul_s(1, 1, 2);
                                                    c.mul_s(0, 21, 0);
                                                    c.div_s(24, 1, 0);
                                                    break 'b_2d7648;
                                                }
                                                c.cycles += 2;
                                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3ce0));
                                                c.min_s(24, 24, 0);
                                                break 'b_2d7650;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x994));
                                    break 'b_2d7654;
                                }
                                c.cycles += 15;
                                c.g[A0][0] = c.g[S5][0];
                                c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2e0));
                                c.g[A1][0] = c.g[FP][0];
                                c.sub_s(0, 23, 0);
                                c.mfc1(2, 0);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.qmtc2(2, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[RA][0] = sx(0x002d7680);
                                Callee {
                                    addr: 0x002d64d8,
                                    f: ee::s09::Avalanche_ClampVector,
                                }
                                .call(c, 0x002d7680)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9d8));
                                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002d6928);
                                    }
                                    continue 'l_2d6928;
                                } else {
                                    break 'b_2d7690;
                                }
                            }
                        }
                    }
                    'b_2d7958: {
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_le_s(0, 26);
                        let t = !c.fcc();
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf4));
                        if t {
                            break 'b_2d7958;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                            if t {
                                break 'b_2d7958;
                            } else {
                                'b_2d76f0: {
                                    c.cycles += 4;
                                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x156));
                                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                                    let t = (c.g[A1][0] as i64) < 0;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_2d76f0;
                                    } else {
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x155));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                        c.g[V0][0] =
                                            sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                                        c.g[V1][0] = c.g[V1][0] ^ 0x6;
                                        if c.g[V1][0] != 0u64 {
                                            c.g[V0][0] = 0u64;
                                            break 'b_2d76f0;
                                        } else {
                                            c.cycles += 6;
                                            c.g[V0][0] = 0x18;
                                            c.mult(0, 3, 5, 2);
                                            c.g[V0][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x3f0));
                                            c.g[V1][0] = c.g[V1][0] ^ 0x3;
                                            c.g[V0][0] = (c.g[V1][0] < 0x1u64) as u64;
                                            break 'b_2d76f0;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                                if t {
                                    break 'b_2d7958;
                                } else {
                                    'b_2d77e8: {
                                        c.cycles += 50;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf4));
                                        c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x10c));
                                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                                        c.g[RA][0] = sx(0x002d770c);
                                        c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        Callee {
                                            addr: 0x002edb20,
                                            f: ee::s09::WorldPainter_SampleSurfaceColours,
                                        }
                                        .call(c, 0x002d770c)?;
                                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x80));
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x84));
                                        c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x8c));
                                        c.sub_s(1, 25, 1);
                                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x88));
                                        c.sub_s(2, 25, 2);
                                        c.sub_s(4, 25, 4);
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x9b4));
                                        c.sub_s(3, 25, 3);
                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x150));
                                        c.mul_s(9, 1, 0);
                                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x154));
                                        c.mul_s(7, 2, 0);
                                        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x120));
                                        c.mul_s(8, 4, 0);
                                        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x124));
                                        c.mul_s(0, 3, 0);
                                        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x128));
                                        c.sub_s(6, 25, 9);
                                        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x12c));
                                        c.sub_s(5, 25, 7);
                                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x158));
                                        c.sub_s(2, 25, 0);
                                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x15c));
                                        c.sub_s(1, 25, 8);
                                        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x134));
                                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x138));
                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x13c));
                                        c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x140));
                                        c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x144));
                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x148));
                                        c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x14c));
                                        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x137));
                                        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x13f));
                                        c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x138));
                                        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x87));
                                        c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                                        c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x8f));
                                        c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x88));
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c4));
                                        let t = c.g[A0][0] != c.g[V0][0];
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
                                        if t {
                                            break 'b_2d77e8;
                                        } else {
                                            c.cycles += 10;
                                            c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x87));
                                            c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                                            c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x8f));
                                            c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x88));
                                            c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x47));
                                            c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x4f));
                                            c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x48));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c4), 0u64);
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
                                            break 'b_2d77e8;
                                        }
                                    }
                                    'b_2d7910: {
                                        c.cycles += 13;
                                        c.g[S0][0] = 0u64;
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c4));
                                        c.g[V0][0] =
                                            sx((c.g[A0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x47));
                                        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x4f));
                                        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x48));
                                        c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x137));
                                        c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x13f));
                                        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x138));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[S2][0] = c.g[V0][0];
                                        if t {
                                            break 'b_2d7910;
                                        } else {
                                            c.cycles += 7;
                                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c4));
                                            c.cvt_s_w(0, 0);
                                            c.g[A2][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x2c4));
                                            c.g[V0][0] = 0xa;
                                            c.mult(0, 17, 6, 2);
                                            c.sub_s(20, 27, 0);
                                            'l_2d7838: loop {
                                                c.cycles += 54;
                                                c.mtc1(16, 4);
                                                c.cvt_s_w(4, 4);
                                                c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x44));
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                                                c.div_s(4, 4, 20);
                                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x84));
                                                c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x48));
                                                c.sub_s(0, 0, 7);
                                                c.lwc1(9, (c.g[SP][0] as u32).wrapping_add(0x4c));
                                                c.sub_s(1, 1, 8);
                                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x88));
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x8c));
                                                c.sub_s(2, 2, 6);
                                                c.g[A0][0] =
                                                    c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
                                                c.sub_s(3, 3, 9);
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x160));
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x164));
                                                c.mul_s(0, 0, 4);
                                                c.mul_s(1, 1, 4);
                                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x168));
                                                c.mul_s(5, 3, 4);
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x16c));
                                                c.mul_s(2, 2, 4);
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x150));
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x154));
                                                c.add_s(0, 0, 7);
                                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x15c));
                                                c.add_s(1, 1, 8);
                                                c.add_s(6, 2, 6);
                                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x158));
                                                c.add_s(5, 5, 9);
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x144));
                                                c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[S1][0] as u32));
                                                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x148));
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0xa));
                                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x14c));
                                                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x147));
                                                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x14f));
                                                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x148));
                                                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x137));
                                                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x13f));
                                                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x138));
                                                c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x134));
                                                c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x138));
                                                c.g[RA][0] = sx(0x002d7904);
                                                c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x13c));
                                                Callee {
                                                    addr: 0x002d55c0,
                                                    f: ee::s09::Avalanche_PackClamped255x3,
                                                }
                                                .call(c, 0x002d7904)?;
                                                c.g[V0][0] = ((c.g[S0][0] as i64)
                                                    < (c.g[S2][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;

                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d7838);
                                                    }
                                                    continue 'l_2d7838;
                                                } else {
                                                    break 'b_2d7910;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 18;
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
                                    c.g[A0][0] = 0xa;
                                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
                                    c.mult(0, 4, 3, 4);
                                    c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x84));
                                    c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x88));
                                    c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x8c));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x2c4), c.g[V1][0]);
                                    c.g[RA][0] = sx(0x002d7938);
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    Callee {
                                        addr: 0x002d55c0,
                                        f: ee::s09::Avalanche_PackClamped255x3,
                                    }
                                    .call(c, 0x002d7938)?;
                                    c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x87));
                                    c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                                    c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x8f));
                                    c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x88));
                                    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x47));
                                    c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                                    c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x4f));
                                    c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x48));
                                    break 'b_2d7958;
                                }
                            }
                        }
                    }
                    'b_2d7a5c: {
                        c.cycles += 44;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
                        c.g[V0][0] = 0xa;
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.mult(0, 16, 3, 2);
                        c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x3cdc));
                        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
                        c.mul_s(15, 0, 15);
                        c.lwc1(12, c.g[SP][0] as u32);
                        c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.g[RA][0] = sx(0x002d7988);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        Callee {
                            addr: 0x002d5658,
                            f: ee::s09::Avalanche_PackSnorm3Alpha,
                        }
                        .call(c, 0x002d7988)?;
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.sub_s(0, 0, 3);
                        c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x74));
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x38));
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x78));
                        c.sub_s(2, 2, 4);
                        c.mul_s(0, 0, 28);
                        c.g[AT][0] = 0x42f00000;
                        c.mtc1(1, 5);
                        c.sub_s(1, 1, 3);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x12c), 0u64);
                        c.mul_s(2, 2, 28);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.mul_s(1, 1, 28);
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x128));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.vu0().sqrt(4, 0);
                        c.vu0().waitq();
                        c.cfc2(2, 22);
                        c.mtc1(2, 0);
                        c.c_lt_s(5, 0);
                        if !c.fcc() {
                            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
                            break 'b_2d7a5c;
                        } else {
                            c.cycles += 21;
                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.vu0().rsqrt(0, 3, 4, 0);
                            c.vu0().waitq();
                            c.vu0().mul(0b1111, 5, 3, Src::Q);
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                            c.mfc1(3, 5);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x140));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
                            break 'b_2d7a5c;
                        }
                    }
                    c.cycles += 37;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2dc));
                    c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x124));
                    c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x128));
                    c.g[RA][0] = sx(0x002d7a78);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x2c8), c.g[A1][0]);
                    Callee {
                        addr: 0x002d5598,
                        f: ee::s09::Avalanche_PackBytes3,
                    }
                    .call(c, 0x002d7a78)?;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
                    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x2c8));
                    c.cvt_s_w(3, 3);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x2));
                    c.g[A0][0] = c.lb(c.g[V0][0] as u32);
                    c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.mtc1(3, 1);
                    c.cvt_s_w(1, 1);
                    c.mtc1(4, 2);
                    c.cvt_s_w(2, 2);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), 0u64);
                    c.mtc1(5, 0);
                    c.cvt_s_w(0, 0);
                    c.add_s(1, 1, 1);
                    c.add_s(2, 2, 2);
                    c.add_s(0, 0, 0);
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x148));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x144));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.lwc1(4, (c.g[S5][0] as u32).wrapping_add(0x100));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.c_lt_s(3, 4);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.fcc();
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002d6838);
                        }
                        continue 'l_2d6838;
                    } else {
                        break 'b_2d7af0;
                    }
                }
            }
        }
        'b_2d7b3c: {
            c.cycles += 6;
            c.g[A0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xc));
            c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2d4), c.g[A2][0]);
            c.g[V0][0] = ((c.g[A0][0] as i64) < 32i64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x110));
            if t {
                break 'b_2d7b3c;
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3cd8));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S6][0] as u32));
                'l_2d7b20: loop {
                    c.cycles += 7;
                    c.swc1(0, c.g[A1][0] as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 32i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002d7b20);
                        }
                        continue 'l_2d7b20;
                    } else {
                        break 'b_2d7b3c;
                    }
                }
            }
        }
        'b_2d7b7c: {
            c.cycles += 4;
            c.g[A0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xe));
            c.g[V0][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
            if t {
                break 'b_2d7b7c;
            } else {
                c.cycles += 5;
                c.g[A2][0] = 0xffff;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x112));
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S6][0] as u32));
                'l_2d7b60: loop {
                    c.cycles += 7;
                    c.sh(c.g[A1][0] as u32, c.g[A2][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.sh(c.g[V1][0] as u32, 0u64);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002d7b60);
                        }
                        continue 'l_2d7b60;
                    } else {
                        break 'b_2d7b7c;
                    }
                }
            }
        }
        c.cycles += 33;
        c.g[A1][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xc));
        c.g[A3][0] = 0x2d0000;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d4));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5718));
        c.g[RA][0] = sx(0x002d7b94);
        c.g[A2][0] = 0x8;
        call_ptr(c, 0x00418ef8, 0x002d7b94)?;
        c.g[A1][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xe));
        c.g[A3][0] = 0x2d0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5750));
        c.g[RA][0] = sx(0x002d7bac);
        c.g[A2][0] = 0x4;
        call_ptr(c, 0x00418ef8, 0x002d7bac)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x380));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x370));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x360));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x350));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x340));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x330));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x320));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x310));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x300));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x2f0));
        c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x3d0));
        c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x3c8));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x3c0));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x3b8));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x3b0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x3a8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x3a0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x398));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x390));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3e0));
        c.pc = t;
        return Ok(());
    });
}

/// `void Avalanche_WriteHistoryFile(bool direct, const char* name)` (0x2d83b8,
/// `cAvalanche::SaveData`): when the avalanche list exists, opens the course's
/// history file, writes the magic, the size and then every avalanche's nodes,
/// collisions and ground collisions, closes it and clears `cAvalanche_m_Save`.
/// Transcription of the generated body (labels kept).
#[allow(
    unused_labels,
    unused_assignments,
    unused_mut,
    unreachable_code,
    unused_variables
)]
pub fn avalanche_write_history_file(c: &mut Cpu) {
    guest(c, |c| {
        'b_2d87ac: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x180));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.g[S2][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
            c.g[S4][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
            let t = c.g[V0][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
            if t {
                break 'b_2d87ac;
            } else {
                'b_2d87a8: {
                    c.cycles += 25;
                    c.g[RA][0] = sx(0x002d83f0);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), 0u64);
                    call_ptr(c, 0x0014dc80, 0x002d83f0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x002d83fc);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dd58, 0x002d83fc)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[S0][0] = 0x4a0000;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3af0));
                    c.g[RA][0] = sx(0x002d8410);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00144be0, 0x002d8410)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002d841c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00144c48, 0x002d841c)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x002d8430);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00417828, 0x002d8430)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x26;
                    c.g[A2][0] = 0x64;
                    c.g[RA][0] = sx(0x002d8444);
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x104));
                    call_ptr(c, 0x003debf0, 0x002d8444)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
                    if t {
                        break 'b_2d87a8;
                    } else {
                        'b_2d84f4: {
                            c.cycles += 3;
                            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x108));
                            let t = c.g[S1][0] == 0u64;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10c));
                            if t {
                                break 'b_2d84f4;
                            } else {
                                c.cycles += 2;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                'l_2d8460: loop {
                                    'b_2d8494: {
                                        c.cycles += 2;
                                        if c.g[A0][0] == 0u64 {
                                            c.g[V0][0] =
                                                c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                                            break 'b_2d8494;
                                        } else {
                                            c.cycles += 2;
                                            c.g[A1][0] = 0xa;
                                            'l_2d8470: loop {
                                                c.cycles += 8;
                                                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x100));
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                c.g[A0][0] =
                                                    c.lw((c.g[A0][0] as u32).wrapping_add(0xf4));
                                                c.cvt_w_s(1, 0);
                                                c.mfc1(2, 1);
                                                c.mult(0, 2, 2, 5);
                                                let t = c.g[A0][0] != 0u64;
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d8470);
                                                    }
                                                    continue 'l_2d8470;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = c
                                                        .lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                                                    break 'b_2d8494;
                                                }
                                            }
                                        }
                                    }
                                    'b_2d84bc: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_2d84bc;
                                        } else {
                                            c.cycles += 1;
                                            'l_2d84a0: loop {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d84a0);
                                                    }
                                                    continue 'l_2d84a0;
                                                } else {
                                                    break 'b_2d84bc;
                                                }
                                            }
                                        }
                                    }
                                    'b_2d84e8: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xe));
                                        if c.g[V0][0] == 0u64 {
                                            c.g[S1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            break 'b_2d84e8;
                                        } else {
                                            'l_2d84c8: loop {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d84c8);
                                                    }
                                                    continue 'l_2d84c8;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[S1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    break 'b_2d84e8;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    if c.g[S1][0] != 0u64 {
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002d8460);
                                        }
                                        continue 'l_2d8460;
                                    } else {
                                        c.cycles += 1;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
                                        break 'b_2d84f4;
                                    }
                                }
                            }
                        }
                        'b_2d879c: {
                            c.cycles += 20;
                            c.g[V0][0] = 0x2be0000;
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                            c.g[V0][0] = c.g[V0][0] | 0xef00;
                            c.g[A0][0] = c.g[S4][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x108), c.g[V0][0]);
                            c.g[A2][0] = 0u64;
                            c.g[T0][0] = 0x4;
                            c.g[RA][0] = sx(0x002d8518);
                            c.g[T1][0] = 0x1;
                            Callee {
                                addr: 0x002d82a0,
                                f: ee::s09::Avalanche_WriteData,
                            }
                            .call(c, 0x002d8518)?;
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                            c.g[A3][0] = c.g[S0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A2][0] = 0x4;
                            c.g[T0][0] = 0x4;
                            c.g[RA][0] = sx(0x002d8534);
                            c.g[T1][0] = 0x1;
                            Callee {
                                addr: 0x002d82a0,
                                f: ee::s09::Avalanche_WriteData,
                            }
                            .call(c, 0x002d8534)?;
                            c.g[V1][0] = 0x8;
                            c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
                            if t {
                                break 'b_2d879c;
                            } else {
                                c.cycles += 3;
                                c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                'l_2d8550: loop {
                                    'b_2d8630: {
                                        c.cycles += 2;
                                        if c.g[S0][0] == 0u64 {
                                            c.g[V0][0] =
                                                c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                                            break 'b_2d8630;
                                        } else {
                                            c.cycles += 2;
                                            c.g[S2][0] = c.g[S5][0];
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                            'l_2d8560: loop {
                                                c.cycles += 51;
                                                c.g[V0][0] = 0xbeef;
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.sh(
                                                    (c.g[SP][0] as u32).wrapping_add(0x100),
                                                    c.g[V0][0],
                                                );
                                                c.g[A3][0] = c.g[S2][0];
                                                c.g[T0][0] = 0x2;
                                                c.g[RA][0] = sx(0x002d8580);
                                                c.g[T1][0] = 0x1;
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d8580)?;
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[V1][0] = 0xa;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[A3][0] = c.g[S2][0];
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[A2][0],
                                                );
                                                c.g[T0][0] = 0x2;
                                                c.g[T1][0] = 0x1;
                                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x100));
                                                c.cvt_w_s(1, 0);
                                                c.mfc1(2, 1);
                                                c.mult(0, 2, 2, 3);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0xc));
                                                c.g[RA][0] = sx(0x002d85c0);
                                                c.sh(
                                                    (c.g[SP][0] as u32).wrapping_add(0x100),
                                                    c.g[V0][0],
                                                );
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d85c0)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A3][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0xe0));
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[T0][0] = 0x4;
                                                c.g[A2][0] = c.g[V1][0];
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[V1][0],
                                                );
                                                c.g[RA][0] = sx(0x002d85e8);
                                                c.g[T1][0] = 0x3;
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d85e8)?;
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A0][0] = 0x1;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[T0][0] = 0xa;
                                                c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[A2][0],
                                                );
                                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x100));
                                                c.cvt_w_s(1, 0);
                                                c.mfc1(9, 1);
                                                c.g[RA][0] = sx(0x002d8614);
                                                c.g[A3][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xfc));
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d8614)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[V1][0],
                                                );
                                                c.g[S0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xf4));
                                                let t = c.g[S0][0] != 0u64;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d8560);
                                                    }
                                                    continue 'l_2d8560;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = c
                                                        .lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                                                    break 'b_2d8630;
                                                }
                                            }
                                        }
                                    }
                                    'b_2d8714: {
                                        c.cycles += 26;
                                        c.g[A0][0] = c.g[S4][0];
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                        c.g[A3][0] = c.g[S5][0];
                                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                        c.g[T0][0] = 0x2;
                                        c.sh((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x002d8650);
                                        c.g[T1][0] = 0x1;
                                        Callee {
                                            addr: 0x002d82a0,
                                            f: ee::s09::Avalanche_WriteData,
                                        }
                                        .call(c, 0x002d8650)?;
                                        c.g[S3][0] = 0u64;
                                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                        c.g[A0][0] = c.g[S4][0];
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                        c.g[A3][0] = c.g[S5][0];
                                        c.g[A2][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[T0][0] = 0x2;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[A2][0]);
                                        c.g[T1][0] = 0x1;
                                        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xe));
                                        c.g[RA][0] = sx(0x002d8680);
                                        c.sh((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
                                        Callee {
                                            addr: 0x002d82a0,
                                            f: ee::s09::Avalanche_WriteData,
                                        }
                                        .call(c, 0x002d8680)?;
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
                                        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] =
                                                c.lhu((c.g[S1][0] as u32).wrapping_add(0xe));
                                            break 'b_2d8714;
                                        } else {
                                            c.cycles += 4;
                                            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
                                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                            'l_2d86a8: loop {
                                                c.cycles += 26;
                                                c.g[A3][0] = c.g[S0][0];
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[T0][0] = 0x4;
                                                c.g[RA][0] = sx(0x002d86c0);
                                                c.g[T1][0] = 0x1;
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d86c0)?;
                                                c.g[S3][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[T0][0] = 0x4;
                                                c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[T1][0] = 0x1;
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[A2][0],
                                                );
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A3][0] = c.lw(c.g[S2][0] as u32);
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x8));
                                                c.g[RA][0] = sx(0x002d86f4);
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x78));
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d86f4)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[V1][0],
                                                );
                                                c.g[V0][0] =
                                                    c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                                                c.g[V0][0] = ((c.g[S3][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d86a8);
                                                    }
                                                    continue 'l_2d86a8;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = c
                                                        .lhu((c.g[S1][0] as u32).wrapping_add(0xe));
                                                    break 'b_2d8714;
                                                }
                                            }
                                        }
                                    }
                                    'b_2d8790: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S3][0] = 0u64;
                                        if t {
                                            break 'b_2d8790;
                                        } else {
                                            c.cycles += 3;
                                            c.g[S2][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x112));
                                            c.g[S0][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x110));
                                            'l_2d8728: loop {
                                                c.cycles += 26;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[A3][0] = c.g[S2][0];
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[T0][0] = 0x2;
                                                c.g[RA][0] = sx(0x002d8744);
                                                c.g[T1][0] = 0x1;
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d8744)?;
                                                c.g[S3][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A3][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[T0][0] = 0x2;
                                                c.g[A2][0] = c.g[V1][0];
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[V1][0],
                                                );
                                                c.g[T1][0] = 0x1;
                                                c.g[RA][0] = sx(0x002d8774);
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x4));
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d8774)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[V1][0],
                                                );
                                                c.g[V0][0] =
                                                    c.lhu((c.g[S1][0] as u32).wrapping_add(0xe));
                                                c.g[V0][0] = ((c.g[S3][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d8728);
                                                    }
                                                    continue 'l_2d8728;
                                                } else {
                                                    break 'b_2d8790;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    if c.g[S1][0] != 0u64 {
                                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002d8550);
                                        }
                                        continue 'l_2d8550;
                                    } else {
                                        break 'b_2d879c;
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                        c.g[RA][0] = sx(0x002d87a8);
                        c.g[A1][0] = 0x64;
                        call_ptr(c, 0x003deca0, 0x002d87a8)?;
                        break 'b_2d87a8;
                    }
                }
                c.cycles += 1;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x958), 0u64);
                break 'b_2d87ac;
            }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x180));
        c.pc = t;
        return Ok(());
    })
}

/// `reg + off` as a 32-bit address.
#[inline(always)]
pub(crate) fn oc1044_at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `reg - off` as a 32-bit address.
#[inline(always)]
pub(crate) fn oc1044_back(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_sub(off)
}

/// `ra + rb` (32-bit, for `addu`).
#[inline(always)]
pub(crate) fn oc1044_add32(c: &Cpu, a: usize, b: usize) -> u32 {
    (c.g[a][0] as u32).wrapping_add(c.g[b][0] as u32)
}

/// Node field: the instance record the node draws (`tAvalancheObject*`).
pub(crate) const AVNODE_OBJ: u32 = 0x2e0;

/// Instance record: pointer to the 10-byte line records.
pub(crate) const AVOBJ_RECS: u32 = 0xfc;

/// Instance record: number of records (float, converted with `cvt.w.s`).
pub(crate) const AVOBJ_COUNT: u32 = 0x100;

/// Instance record: origin of the first line (a vector, 16 bytes).
pub(crate) const AVOBJ_POS: u32 = 0xe0;

/// Debug-draw flag (`gp + 0x968`).
pub(crate) const AVDEBUG_FLAG: u32 = 0x968;

/// Global render state (`gp - 0x854`).
pub(crate) const AVRS_GLOBAL: u32 = 0x854;

/// `void Avalanche_DebugDrawInstance(Node* node)` (0x2d8a00): when the debug
/// flag is set, draws every line record of the node's instance as a small
/// oriented marker. Each record's offset/axis/colour bytes are turned into a
/// rotation matrix about the (normalised) axis, the matrix is multiplied with
/// the running transform and stored with the colour into a table at
/// 0x4d5b70 (0x30 bytes per record); finally the render state's draw method
/// (vtable slot 0x248) is called with `(count, table, 0)`. The render-state
/// stack pointer (`*(world + 0xe84)`) is pushed with a copy of the matrix at
/// 0x4ff1a0 before and popped after.
pub fn avalanche_debug_draw_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x1d0);
        frame.save(c, 16, 0x1b0);
        frame.save(c, 17, 0x1a0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x190);
        frame.save(c, 19, 0x180);
        frame.save(c, 20, 0x170);
        frame.save(c, 21, 0x160);
        frame.save(c, 22, 0x150);
        frame.save(c, 23, 0x140);
        frame.save(c, 30, 0x130);
        frame.save_ra(c, 0x120);
        frame.save_fpr(c, 21, 0x1c8);
        frame.save_fpr(c, 20, 0x1c0);
        c.g[V0][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
        let no_obj = c.g[V0][0] == 0;
        c.g[V0][0] = 0x500000;
        'done: {
            if no_obj {
                break 'done;
            }
            'pushed: {
                c.cycles += 34;
                // Copy the 4x4 matrix at 0x4ff1a0 to the stack.
                c.g[V0][0] = sx(oc1044_back(c, V0, 0xe60));
                c.lqc2(1, oc1044_at(c, V0, 0));
                c.lqc2(2, oc1044_at(c, V0, 0x10));
                c.lqc2(3, oc1044_at(c, V0, 0x20));
                c.lqc2(4, oc1044_at(c, V0, 0x30));
                c.sqc2(1, oc1044_at(c, SP, 0));
                c.sqc2(2, oc1044_at(c, SP, 0x10));
                c.sqc2(3, oc1044_at(c, SP, 0x20));
                c.sqc2(4, oc1044_at(c, SP, 0x30));
                // Push a 0x14-byte render-state record (copy of the top one).
                c.g[A0][0] = c.lw(oc1044_back(c, GP, AVRS_GLOBAL));
                c.g[A3][0] = 0xffff_ffff_ffff_ff83;
                c.g[V0][0] = c.lw(oc1044_at(c, A0, 0xe84));
                c.ldl(3, oc1044_at(c, V0, 0x7));
                c.ldr(3, oc1044_at(c, V0, 0));
                c.ldl(5, oc1044_at(c, V0, 0xf));
                c.ldr(5, oc1044_at(c, V0, 0x8));
                c.g[A2][0] = c.lw(oc1044_at(c, V0, 0x10));
                c.sdl(3, oc1044_at(c, V0, 0x1b));
                c.sdr(3, oc1044_at(c, V0, 0x14));
                c.sdl(5, oc1044_at(c, V0, 0x23));
                c.sdr(5, oc1044_at(c, V0, 0x1c));
                c.sw(oc1044_at(c, V0, 0x24), c.g[A2][0]);
                c.g[V1][0] = c.lw(oc1044_at(c, A0, 0xe84));
                c.g[A2][0] = c.lw(oc1044_back(c, GP, AVRS_GLOBAL));
                c.g[V1][0] = sx(oc1044_at(c, V1, 0x14));
                c.sw(oc1044_at(c, A0, 0xe84), c.g[V1][0]);
                // Set bit 2 / clear the 0x7c bits (mask -125) in the new top's flags.
                c.g[A1][0] = c.lw(oc1044_at(c, A2, 0xe84));
                c.g[V0][0] = c.lw(oc1044_at(c, A1, 0x4));
                c.g[V0][0] &= c.g[A3][0];
                c.g[V0][0] |= 0x4;
                c.sw(oc1044_at(c, A1, 0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(oc1044_at(c, GP, AVDEBUG_FLAG));
                let flag_off = c.g[V1][0] == 0;
                c.g[S2][0] = 0;
                if flag_off {
                    break 'pushed;
                }
                'lines: {
                    c.cycles += 9;
                    c.g[V1][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                    c.lq(2, oc1044_at(c, V1, AVOBJ_POS));
                    c.sq(2, oc1044_at(c, SP, 0x40));
                    c.lwc1(0, oc1044_at(c, V1, AVOBJ_COUNT));
                    c.cvt_w_s(1, 0);
                    c.mfc1(2, 1);
                    let none = (c.g[V0][0] as i64) <= 0;
                    c.g[FP][0] = 0x4d0000;
                    if none {
                        break 'lines;
                    }
                    c.cycles += 10;
                    c.g[V0][0] = 0x500000;
                    c.g[S5][0] = sx(oc1044_at(c, FP, 0x5b70));
                    c.lwc1(21, oc1044_back(c, GP, 0x3cd0));
                    c.g[S7][0] = sx(oc1044_back(c, V0, 0xed0));
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 20);
                    c.g[S6][0] = sx(oc1044_at(c, S5, 0x20));
                    c.g[S4][0] = c.g[SP][0];
                    c.g[S3][0] = sx(oc1044_at(c, SP, 0x80));
                    c.g[S1][0] = 0;
                    loop {
                        // -- one record: position step, axis, angle --
                        c.cycles += 52;
                        c.g[V1][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.mtc1(0, 3);
                        c.g[V0][0] = c.lw(oc1044_at(c, V1, AVOBJ_RECS));
                        c.g[V0][0] = sx(oc1044_add32(c, S1, V0));
                        c.g[V1][0] = c.lb(oc1044_at(c, V0, 0x2));
                        c.g[A0][0] = c.lb(oc1044_at(c, V0, 0));
                        c.g[A1][0] = c.lb(oc1044_at(c, V0, 0x1));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.mtc1(4, 2);
                        c.cvt_s_w(2, 2);
                        c.swc1(3, oc1044_at(c, SP, 0x5c));
                        c.mtc1(5, 0);
                        c.cvt_s_w(0, 0);
                        c.add_s(1, 1, 1);
                        c.add_s(2, 2, 2);
                        c.add_s(0, 0, 0);
                        c.swc1(1, oc1044_at(c, SP, 0x58));
                        c.swc1(2, oc1044_at(c, SP, 0x50));
                        c.swc1(0, oc1044_at(c, SP, 0x54));
                        c.lqc2(3, oc1044_at(c, SP, 0x40));
                        c.lqc2(4, oc1044_at(c, SP, 0x50));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, oc1044_at(c, SP, 0x40));
                        c.g[V0][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.g[V1][0] = c.lw(oc1044_at(c, V0, AVOBJ_RECS));
                        c.g[V1][0] = sx(oc1044_add32(c, S1, V1));
                        c.g[V0][0] = c.lb(oc1044_at(c, V1, 0x6));
                        c.g[A1][0] = c.lb(oc1044_at(c, V1, 0x7));
                        c.mtc1(2, 2);
                        c.cvt_s_w(2, 2);
                        c.g[A0][0] = c.lb(oc1044_at(c, V1, 0x8));
                        c.mtc1(5, 0);
                        c.cvt_s_w(0, 0);
                        c.mtc1(4, 1);
                        c.cvt_s_w(1, 1);
                        c.mul_s(5, 2, 21);
                        c.mul_s(6, 0, 21);
                        c.mul_s(4, 1, 21);
                        c.mul_s(0, 5, 5);
                        c.swc1(5, oc1044_at(c, SP, 0x70));
                        c.mul_s(2, 6, 6);
                        c.swc1(6, oc1044_at(c, SP, 0x74));
                        c.mul_s(1, 4, 4);
                        c.swc1(4, oc1044_at(c, SP, 0x78));
                        c.add_s(0, 0, 2);
                        c.add_s(0, 0, 1);
                        c.sqrt_s(0, 0);
                        c.c_eq_s(0, 3);
                        if c.fcc() {
                            // zero-length axis: stored unnormalised
                            c.swc1(5, oc1044_at(c, SP, 0x60));
                            c.cycles += 2;
                            c.swc1(6, oc1044_at(c, SP, 0x64));
                            c.swc1(4, oc1044_at(c, SP, 0x68));
                        } else {
                            c.cycles += 8;
                            c.div_s(0, 20, 0);
                            c.mul_s(1, 4, 0);
                            c.mul_s(2, 5, 0);
                            c.mul_s(0, 6, 0);
                            c.swc1(1, oc1044_at(c, SP, 0x68));
                            c.swc1(2, oc1044_at(c, SP, 0x60));
                            c.swc1(0, oc1044_at(c, SP, 0x64));
                        }
                        // -- rotation about the axis by the angle byte --
                        c.cycles += 136;
                        c.lwc1(0, oc1044_at(c, SP, 0x60));
                        c.lwc1(1, oc1044_at(c, SP, 0x64));
                        c.lwc1(2, oc1044_at(c, SP, 0x68));
                        c.swc1(0, oc1044_at(c, SP, 0x50));
                        c.swc1(1, oc1044_at(c, SP, 0x54));
                        c.swc1(2, oc1044_at(c, SP, 0x58));
                        c.sw(oc1044_at(c, SP, 0x5c), 0);
                        c.lwc1(0, oc1044_back(c, GP, 0x3ccc));
                        c.g[V1][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.g[V0][0] = c.lw(oc1044_at(c, V1, AVOBJ_RECS));
                        c.g[V0][0] = sx(oc1044_add32(c, S1, V0));
                        c.g[V1][0] = c.lbu(oc1044_at(c, V0, 0x9));
                        c.mtc1(3, 12);
                        c.cvt_s_w(12, 12);
                        c.mul_s(12, 12, 0);
                        c.lqc2(1, oc1044_at(c, S4, 0));
                        c.lqc2(2, oc1044_at(c, S4, 0x10));
                        c.lqc2(3, oc1044_at(c, S4, 0x20));
                        c.lqc2(4, oc1044_at(c, S4, 0x30));
                        c.sqc2(1, oc1044_at(c, S3, 0));
                        c.sqc2(2, oc1044_at(c, S3, 0x10));
                        c.sqc2(3, oc1044_at(c, S3, 0x20));
                        c.sqc2(4, oc1044_at(c, S3, 0x30));
                        c.lq(2, oc1044_at(c, SP, 0xb0));
                        c.g[A0][0] = sx(oc1044_at(c, SP, 0x110));
                        c.lq(3, oc1044_at(c, S7, 0));
                        c.g[A1][0] = sx(oc1044_at(c, SP, 0x114));
                        c.sq(2, oc1044_at(c, SP, 0xc0));
                        c.sq(3, oc1044_at(c, SP, 0xb0));
                        // Math_SinCos(angle, &sin, &cos)
                        call_ptr(c, 0x0031_be50, 0x002d_8c88)?;
                        c.lwc1(11, oc1044_at(c, SP, 0x114));
                        c.lwc1(0, oc1044_at(c, SP, 0x50));
                        c.sub_s(1, 20, 11);
                        c.lwc1(3, oc1044_at(c, SP, 0x58));
                        c.lwc1(7, oc1044_at(c, SP, 0x54));
                        c.lwc1(8, oc1044_at(c, SP, 0x110));
                        c.mul_s(2, 1, 3);
                        c.lq(2, oc1044_at(c, SP, 0xc0));
                        c.mul_s(4, 1, 0);
                        c.sw(oc1044_at(c, SP, 0x30), 0);
                        c.mul_s(1, 1, 7);
                        c.sw(oc1044_at(c, SP, 0x34), 0);
                        c.mul_s(13, 8, 3);
                        c.sw(oc1044_at(c, SP, 0x38), 0);
                        c.mul_s(12, 8, 0);
                        c.swc1(20, oc1044_at(c, SP, 0x3c));
                        c.mul_s(9, 2, 3);
                        c.sq(2, oc1044_at(c, SP, 0x30));
                        c.mul_s(10, 4, 3);
                        c.sw(oc1044_at(c, SP, 0xc), 0);
                        c.mul_s(5, 2, 0);
                        c.sw(oc1044_at(c, SP, 0x1c), 0);
                        c.mul_s(6, 4, 0);
                        c.sw(oc1044_at(c, SP, 0x2c), 0);
                        c.mul_s(3, 1, 3);
                        c.mul_s(0, 1, 0);
                        c.mul_s(8, 8, 7);
                        c.mul_s(2, 2, 7);
                        c.mul_s(4, 4, 7);
                        c.mul_s(1, 1, 7);
                        c.add_s(5, 5, 8);
                        c.sub_s(0, 0, 13);
                        c.sub_s(2, 2, 12);
                        c.add_s(9, 9, 11);
                        c.swc1(5, oc1044_at(c, SP, 0x8));
                        c.add_s(6, 6, 11);
                        c.swc1(0, oc1044_at(c, SP, 0x4));
                        c.add_s(4, 4, 13);
                        c.swc1(2, oc1044_at(c, SP, 0x18));
                        c.sub_s(10, 10, 8);
                        c.swc1(9, oc1044_at(c, SP, 0x28));
                        c.add_s(1, 1, 11);
                        c.swc1(6, oc1044_at(c, SP, 0));
                        c.add_s(3, 3, 12);
                        c.swc1(4, oc1044_at(c, SP, 0x10));
                        c.swc1(10, oc1044_at(c, SP, 0x20));
                        c.swc1(1, oc1044_at(c, SP, 0x14));
                        c.swc1(3, oc1044_at(c, SP, 0x24));
                        // running = running * rotation (VU0 4x4 multiply)
                        c.lqc2(4, oc1044_at(c, S4, 0));
                        c.lqc2(5, oc1044_at(c, S4, 0x10));
                        c.lqc2(6, oc1044_at(c, S4, 0x20));
                        c.lqc2(7, oc1044_at(c, S4, 0x30));
                        c.lqc2(8, oc1044_at(c, S3, 0));
                        c.lqc2(9, oc1044_at(c, S3, 0x10));
                        c.lqc2(10, oc1044_at(c, S3, 0x20));
                        c.lqc2(11, oc1044_at(c, S3, 0x30));
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
                        c.sqc2(12, oc1044_at(c, S4, 0));
                        c.sqc2(13, oc1044_at(c, S4, 0x10));
                        c.sqc2(14, oc1044_at(c, S4, 0x20));
                        c.sqc2(15, oc1044_at(c, S4, 0x30));
                        // -- table entry: matrix row + colour bytes --
                        c.lq(7, oc1044_at(c, SP, 0x40));
                        c.g[A1][0] = 0x30;
                        c.g[V0][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.mult(0, 5, 18, 5);
                        c.sq(7, oc1044_at(c, SP, 0x30));
                        c.g[S2][0] = sx(oc1044_at(c, S2, 1));
                        c.g[V1][0] = c.lw(oc1044_at(c, V0, AVOBJ_RECS));
                        c.g[A2][0] = sx(oc1044_add32(c, A1, S5));
                        c.g[V1][0] = sx(oc1044_add32(c, S1, V1));
                        c.g[A1][0] = sx(oc1044_add32(c, A1, S6));
                        c.g[V0][0] = c.lbu(oc1044_at(c, V1, 0x3));
                        c.sw(oc1044_at(c, A2, 0x10), c.g[V0][0]);
                        c.g[V1][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.g[V0][0] = c.lw(oc1044_at(c, V1, AVOBJ_RECS));
                        c.g[V0][0] = sx(oc1044_add32(c, S1, V0));
                        c.g[V1][0] = c.lbu(oc1044_at(c, V0, 0x4));
                        c.sw(oc1044_at(c, A2, 0x14), c.g[V1][0]);
                        c.g[A0][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.g[V0][0] = c.lw(oc1044_at(c, A0, AVOBJ_RECS));
                        c.g[V0][0] = sx(oc1044_add32(c, S1, V0));
                        c.g[V1][0] = c.lbu(oc1044_at(c, V0, 0x5));
                        c.sw(oc1044_at(c, A2, 0x18), c.g[V1][0]);
                        c.sq(7, oc1044_at(c, A1, 0));
                        c.g[V1][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                        c.lwc1(0, oc1044_at(c, V1, AVOBJ_COUNT));
                        c.cvt_w_s(1, 0);
                        c.mfc1(2, 1);
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[S1][0] = sx(oc1044_at(c, S1, 0xa));
                        if !more {
                            break 'lines;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_8b18);
                        }
                    }
                }
                // Submit: vtable slot 0x248 of the render state's draw object.
                c.cycles += 12;
                c.g[V1][0] = c.lw(oc1044_at(c, S0, AVNODE_OBJ));
                c.g[A2][0] = sx(oc1044_at(c, FP, 0x5b70));
                c.g[T0][0] = c.lw(oc1044_back(c, GP, AVRS_GLOBAL));
                c.g[A3][0] = 0;
                c.lwc1(0, oc1044_at(c, V1, AVOBJ_COUNT));
                c.g[V0][0] = c.lw(oc1044_at(c, T0, 0x10d8));
                c.cvt_w_s(1, 0);
                c.mfc1(5, 1);
                c.g[A0][0] = c.lh(oc1044_at(c, V0, 0x248));
                c.g[V1][0] = c.lw(oc1044_at(c, V0, 0x24c));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx(oc1044_add32(c, T0, A0));
                call_ptr(c, target, 0x002d_8e60)?;
            }
            // Pop the render-state record.
            c.cycles += 4;
            c.g[V1][0] = c.lw(oc1044_back(c, GP, AVRS_GLOBAL));
            c.g[V0][0] = c.lw(oc1044_at(c, V1, 0xe84));
            c.g[V0][0] = sx(oc1044_back(c, V0, 0x14));
            c.sw(oc1044_at(c, V1, 0xe84), c.g[V0][0]);
        }
        c.cycles += 14;
        frame.restore(c, 16, 0x1b0);
        frame.restore(c, 17, 0x1a0);
        frame.restore(c, 18, 0x190);
        frame.restore(c, 19, 0x180);
        frame.restore(c, 20, 0x170);
        frame.restore(c, 21, 0x160);
        frame.restore(c, 22, 0x150);
        frame.restore(c, 23, 0x140);
        frame.restore(c, 30, 0x130);
        frame.restore_ra(c, 0x120);
        frame.restore_fpr(c, 21, 0x1c8);
        frame.restore_fpr(c, 20, 0x1c0);
        frame.close(c);
        Ok(())
    })
}

/// Node-list head (`gp + 0x9c0`): singly linked, `next` at +8.
pub(crate) const AVALANCHE_NODE_LIST: u32 = 0x9c0;

/// Free-record counter (`gp + 0x9d0`): index of the next record slot.
pub(crate) const AVALANCHE_REC_COUNT: u32 = 0x9d0;

/// Flag cleared by `Avalanche_AddNode` (`gp + 0x9cc`).
pub(crate) const AVALANCHE_DIRTY: u32 = 0x9cc;

/// `void Avalanche_AddNode(int id, tInstanceRecord* inst, int kind,
/// float x, float y, float z, float w)` (0x2d92d0): finds (or allocates, 0x120
/// bytes, linked at the head of the node list) the node for `id`, then
/// allocates a 0x210-byte record, links it in front of the node's record
/// list and fills it: the four floats (scaled by 32 for x, y, z), the owner
/// `inst`, a mode halfword derived from `kind` (2 -> 0, 5 -> 2, else 1), and
/// the largest-extent ratio of the instance's bounding box, then registers the
/// record in the global table and bumps the counter.
pub fn avalanche_add_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 16, 0x60);
        frame.save(c, 17, 0x50);
        frame.save(c, 18, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 19, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 20, 0x20);
        c.g[S3][0] = 0x480000;
        frame.save(c, 21, 0x10);
        c.g[S4][0] = c.g[A2][0];
        frame.save_fpr(c, 23, 0x88);
        c.g[S5][0] = 0x540000;
        frame.save_fpr(c, 22, 0x80);
        c.mov_s(23, 13);
        frame.save_fpr(c, 21, 0x78);
        c.mov_s(22, 12);
        frame.save_fpr(c, 20, 0x70);
        c.mov_s(21, 15);
        c.g[S0][0] = c.lw(oc1044_at(c, GP, AVALANCHE_NODE_LIST));
        c.mov_s(20, 14);
        frame.save_ra(c, 0);

        // Look the node up by id.
        let mut found = false;
        loop {
            c.cycles += 2;
            let end = c.g[S0][0] == 0;
            c.g[A1][0] = 0x480000;
            if end {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            if c.g[V0][0] != c.g[S2][0] {
                c.g[S0][0] = c.lw(oc1044_at(c, S0, 0x8));
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9328);
                }
                continue;
            }
            // Same id: s0 is non-null here, so the node is found.
            c.cycles += 2;
            c.g[A0][0] = 0x120;
            found = true;
            break;
        }
        if !found {
            // New node: EA_MemAllocNamed(0x210, name, 0, 0), linked at the head.
            c.cycles += 14;
            c.g[A0][0] = 0x210;
            c.g[A1][0] = sx(oc1044_at(c, A1, 0x7568));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x002d_935c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(oc1044_at(c, GP, AVALANCHE_NODE_LIST));
            c.sw(c.g[S0][0] as u32, c.g[S2][0]);
            c.sw(oc1044_at(c, S0, 0x8), c.g[V0][0]);
            c.sh(oc1044_at(c, S0, 0xc), 0);
            c.sh(oc1044_at(c, S0, 0xe), 0);
            c.sw(oc1044_at(c, S0, 0x4), 0);
            c.sw(oc1044_at(c, GP, AVALANCHE_NODE_LIST), c.g[S0][0]);
            c.g[A0][0] = 0x120;
        }

        // The new record: EA_MemAllocNamed(0x120, name, 0, 0).
        c.cycles += 33;
        c.g[A1][0] = sx(oc1044_at(c, S3, 0x7578));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x002d_9390)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[AT][0] = 0x41f0_0000; // 30.0
        c.mtc1(1, 3);
        c.g[V0][0] = c.lw(oc1044_at(c, S0, 0x4));
        c.g[V1][0] = sx(oc1044_at(c, A0, 0x110));
        c.mul_s(6, 20, 3);
        c.sw(oc1044_at(c, A0, 0xf4), c.g[V0][0]);
        c.mul_s(5, 22, 3);
        c.sw(oc1044_at(c, S0, 0x4), c.g[A0][0]);
        c.mul_s(3, 23, 3);
        c.swc1(21, oc1044_at(c, A0, 0x10c));
        c.lwc1(2, oc1044_at(c, S1, 0x60));
        c.lwc1(0, oc1044_at(c, S1, 0x6c));
        c.lwc1(1, oc1044_at(c, S1, 0x70));
        c.lwc1(4, oc1044_at(c, S1, 0x64));
        c.sub_s(0, 0, 2);
        c.abs_s(7, 0);
        c.lwc1(2, oc1044_at(c, S1, 0x74));
        c.sub_s(1, 1, 4);
        c.lwc1(0, oc1044_at(c, S1, 0x68));
        c.abs_s(1, 1);
        c.swc1(5, oc1044_at(c, A0, 0x100));
        c.c_lt_s(7, 1);
        c.swc1(3, oc1044_at(c, A0, 0x104));
        c.sub_s(2, 2, 0);
        c.swc1(6, oc1044_at(c, A0, 0x108));
        c.abs_s(0, 2);
        let dx_not_less_dy = !c.fcc();
        c.sw(oc1044_at(c, A0, 0xf8), c.g[S1][0]);

        // Pick the ratio of the extents by which axis is the largest:
        // 'x_over_y: |dx| < |dy|; each arm ends loading f2/f0 for the ratio.
        #[derive(PartialEq)]
        enum Next {
            LoadX,  // b_2d9430
            RatioZ, // b_2d9470
        }
        let next;
        if dx_not_less_dy {
            c.cycles += 4;
            c.c_lt_s(1, 0);
            if !c.fcc() {
                c.lwc1(0, oc1044_at(c, S1, 0x68));
                next = Next::RatioZ;
            } else {
                c.cycles += 1;
                c.lwc1(2, oc1044_at(c, S1, 0x60));
                next = Next::LoadX;
            }
        } else {
            c.cycles += 4;
            c.c_lt_s(7, 0);
            if c.fcc() {
                c.lwc1(2, oc1044_at(c, S1, 0x60));
                next = Next::LoadX;
            } else {
                c.cycles += 2;
                c.lwc1(0, oc1044_at(c, S1, 0x68));
                next = Next::RatioZ;
            }
        }
        if next == Next::LoadX {
            // b_2d9430 / b_2d9464
            c.cycles += 10;
            c.lwc1(1, oc1044_at(c, S1, 0x6c));
            c.lwc1(3, oc1044_at(c, S1, 0x64));
            c.lwc1(0, oc1044_at(c, S1, 0x70));
            c.sub_s(1, 1, 2);
            c.abs_s(2, 1);
            c.sub_s(0, 0, 3);
            c.abs_s(1, 0);
            c.c_lt_s(2, 1);
            c.cycles += 3;
            if !c.fcc() {
                c.lwc1(0, oc1044_back(c, GP, 0x3cc4));
                c.mul_s(1, 1, 0);
            } else {
                c.lwc1(0, oc1044_back(c, GP, 0x3cc8));
                c.mul_s(1, 2, 0);
            }
        } else {
            c.cycles += 5;
            c.lwc1(1, oc1044_at(c, S1, 0x74));
            c.lwc1(2, oc1044_back(c, GP, 0x3cc0));
            c.sub_s(1, 1, 0);
            c.abs_s(1, 1);
            c.mul_s(1, 1, 2);
        }

        // Store the ratio, clear fields, mode halfword from `kind`.
        c.cycles += 5;
        c.swc1(1, c.g[V1][0] as u32);
        c.g[V1][0] = 0x2;
        c.sw(oc1044_at(c, A0, 0xfc), 0);
        let kind_is_2 = c.g[S4][0] == c.g[V1][0];
        c.sb(oc1044_at(c, A0, 0xf3), 0);
        if kind_is_2 {
            c.cycles += 2;
            c.sh(oc1044_at(c, A0, 0xf0), 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            let kind_not_5 = c.g[S4][0] != c.g[V0][0];
            c.g[V0][0] = 0x1;
            if kind_not_5 {
                c.cycles += 1;
                c.sh(oc1044_at(c, A0, 0xf0), c.g[V0][0]);
            } else {
                c.cycles += 2;
                c.sh(oc1044_at(c, A0, 0xf0), c.g[V1][0]);
            }
        }

        // Register the record in the global table and bump the counter.
        c.cycles += 32;
        c.g[A1][0] = c.lw(oc1044_at(c, GP, AVALANCHE_REC_COUNT));
        c.g[V1][0] = sx(oc1044_back(c, S5, 0x7bb0));
        c.sb(oc1044_at(c, A0, 0xf2), 0);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        c.sw(oc1044_at(c, A0, 0x114), 0);
        c.g[A2][0] = c.lw(oc1044_at(c, A0, 0xf8));
        c.g[V0][0] = sx(oc1044_add32(c, V0, V1));
        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 8);
        c.g[V1][0] |= 0x2;
        c.g[A1][0] = sx(oc1044_at(c, A1, 0x1));
        c.g[V0][0] = c.lbu(oc1044_at(c, A2, 0x92));
        frame.restore(c, 16, 0x60);
        c.sh(oc1044_at(c, A2, 0x92), c.g[V0][0]);
        frame.restore(c, 17, 0x50);
        c.g[A0][0] = c.lw(oc1044_at(c, A0, 0xf8));
        frame.restore(c, 18, 0x40);
        c.g[V0][0] = c.lhu(oc1044_at(c, A0, 0x92));
        frame.restore(c, 19, 0x30);
        c.g[V0][0] |= c.g[V1][0];
        frame.restore(c, 20, 0x20);
        c.sh(oc1044_at(c, A0, 0x92), c.g[V0][0]);
        frame.restore(c, 21, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x88);
        frame.restore_fpr(c, 22, 0x80);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        c.sw(oc1044_at(c, GP, AVALANCHE_REC_COUNT), c.g[A1][0]);
        c.sw(oc1044_at(c, GP, AVALANCHE_DIRTY), 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Avalanche_UpdateInstance` (0x2d7ca8): advance one active avalanche node by
// `t` (scaled by the data block's rate), sample its history, and either copy
// the frame into the node and emit particles, or end the node. Returns in V0
// whether the node is still alive.

/// `tActiveAvalancheNode` fields.
pub(crate) const UPD_DATA: u32 = 0x2e0;

pub(crate) const UPD_FB4: u32 = 0xb4;

pub(crate) const UPD_CUR_POS: u32 = 0x60;

pub(crate) const UPD_CUR_MAT: u32 = 0x70;

pub(crate) const UPD_CUR_COLOUR: u32 = 0xc0;

pub(crate) const UPD_EMITTER: u32 = 0xd0;

pub(crate) const UPD_F1FC: u32 = 0x1fc;

/// `sAvData` fields: length, rate, type (u16), emits (u8).
pub(crate) const UPD_D_LENGTH: u32 = 0x100;

pub(crate) const UPD_D_RATE: u32 = 0x10c;

pub(crate) const UPD_D_TYPE: u32 = 0xf0;

pub(crate) const UPD_D_EMITS: u32 = 0xf2;

/// The sampled frame in the caller's stack: position, colour, matrix.
pub(crate) const UPD_OUT_COLOUR: u32 = 0x10;

pub(crate) const UPD_OUT_MAT: u32 = 0x20;

/// `bool Avalanche_UpdateInstance(self, float t)` (0x2d7ca8).
#[allow(dead_code)]
pub fn avalanche_update_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x70);
        frame.save_fpr(c, 20, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x60);
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(UPD_DATA)).lw(c);
        let has_data = c.g[V0][0] != 0;
        c.mov_s(20, 12);
        if has_data {
            upd_sample_and_emit(c)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// The body for an instance with data (V0 = its data block on entry): sample
/// the history at `t * rate`; past the length, end the node; otherwise copy
/// the sampled frame into the node and emit (or end the node when it emits
/// nothing).
pub(crate) fn upd_sample_and_emit(c: &mut Cpu) -> Flow {
    c.cycles += 15;
    let data = r32(c, V0);
    c.lwc1(0, data.wrapping_add(UPD_D_RATE));
    c.g[A0][0] = sx(sp(c));
    c.g[A1][0] = sx(r32(c, S0));
    c.mul_s(20, 20, 0);
    c.mov_s(12, 20);
    Callee {
        addr: 0x002d_5778,
        f: ee::s09::Avalanche_SampleHistory,
    }
    .call(c, 0x002d_7ce8)?;
    c.g[AT][0] = 0x41f0_0000; // 30.0
    c.mtc1(AT, 0);
    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(UPD_DATA)).lw(c);
    c.mul_s(0, 20, 0);
    let length = r32(c, V0).wrapping_add(UPD_D_LENGTH);
    c.lwc1(1, length);
    c.c_lt_s(0, 1);
    let inside = c.fcc();
    c.g[A1][0] = sx(r32(c, S0).wrapping_add(UPD_CUR_POS));
    if !inside {
        return upd_past_end(c);
    }
    // Copy the sampled frame into the node: position (v0), matrix, colour.
    c.cycles += 19;
    c.lq(2, sp(c));
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(UPD_CUR_MAT));
    let out_mat = sp(c).wrapping_add(UPD_OUT_MAT);
    c.g[V1][0] = sx(out_mat);
    c.sq(2, r32(c, S0).wrapping_add(UPD_CUR_POS));
    c.lqc2(1, out_mat);
    c.lqc2(2, out_mat.wrapping_add(0x10));
    c.lqc2(3, out_mat.wrapping_add(0x20));
    c.lqc2(4, out_mat.wrapping_add(0x30));
    let cur_mat = r32(c, A0);
    c.sqc2(1, cur_mat);
    c.sqc2(2, cur_mat.wrapping_add(0x10));
    c.sqc2(3, cur_mat.wrapping_add(0x20));
    c.sqc2(4, cur_mat.wrapping_add(0x30));
    c.lq(2, sp(c).wrapping_add(UPD_OUT_COLOUR));
    c.g[T1][0] = Addr(r32(c, S0).wrapping_add(UPD_DATA)).lw(c);
    c.g[A0][0] = 2;
    c.sq(2, r32(c, S0).wrapping_add(UPD_CUR_COLOUR));
    c.g[V1][0] = Addr(r32(c, T1).wrapping_add(UPD_D_TYPE)).lhu(c);
    let typed = c.g[V1][0] == c.g[A0][0];
    c.g[A3][0] = sx(r32(c, S0).wrapping_add(UPD_CUR_COLOUR));
    if typed {
        return upd_alive(c);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, T1).wrapping_add(UPD_D_EMITS)).lbu(c);
    let silent = c.g[V0][0] == 0;
    c.g[V0][0] = 0x0050_0000;
    if silent {
        return upd_alive(c);
    }
    upd_emit(c)
}

/// The sample is past the instance's length: a type-2 instance stays alive,
/// any other one is released and ends (V0 = 0).
pub(crate) fn upd_past_end(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(UPD_D_TYPE)).lhu(c);
    c.g[V0][0] = 2;
    let typed = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = sx(r32(c, S0));
    if typed {
        return upd_alive(c);
    }
    c.cycles += 4;
    c.g[A1][0] = 0;
    Callee {
        addr: 0x002d_7dd8,
        f: ee::s09::Avalanche_ReleaseInstance,
    }
    .call(c, 0x002d_7db8)?;
    c.g[V0][0] = 0;
    Ok(())
}

/// The node stays alive (V0 = 1) after its frame was copied.
pub(crate) fn upd_alive(c: &mut Cpu) -> Flow {
    c.cycles += 1;
    c.g[V0][0] = 1;
    Ok(())
}

/// Emit particles from the node's emitter at the sampled position (the
/// `DynamicEmitter` call through the function table); the node stays alive.
pub(crate) fn upd_emit(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.lwc1(0, r32(c, S0).wrapping_add(UPD_FB4));
    c.g[A2][0] = sx(r32(c, V0).wrapping_sub(0xee0));
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(UPD_EMITTER));
    c.swc1(0, r32(c, S0).wrapping_add(UPD_F1FC));
    c.lwc1(0, r32(c, T1).wrapping_add(UPD_D_LENGTH));
    c.c_lt_s(20, 0);
    let before_end = c.fcc();
    c.g[T0][0] = 1;
    if !before_end {
        c.cycles += 1;
        c.g[T0][0] = 0;
    }
    c.cycles += 5;
    c.lwc1(12, r32(c, GP).wrapping_sub(0x3cd4));
    call_ptr(c, 0x0037_17c0, 0x002d_7d98)?;
    c.g[V0][0] = 1;
    Ok(())
}

// ---------------------------------------------------------------------------
// `Avalanche_Stop` (0x2d81b0): free the spawn buffer, release every live
// instance, stop the avalanche sound and count the danger manager's active
// avalanches down.

/// `void Avalanche_Stop(self)` (0x2d81b0).
pub fn active_avalanche_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x002d_81d8)?;
            Addr(r32(c, S1).wrapping_add(0x8)).sw(c, 0);
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        }
        c.cycles += 2;
        let ended = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !ended {
            stop_end_live(c)?;
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

/// The `*(int*)self != 0` branch of `Avalanche_Stop`: release the instance
/// list at +4, clear the flag, stop the sound and count down the danger
/// manager's active avalanches.
pub(crate) fn stop_end_live(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x4)).lw(c);
    let empty = c.g[S0][0] == 0;
    c.g[A0][0] = c.g[S0][0];
    if !empty {
        c.cycles += 1;
        loop {
            c.cycles += 5;
            c.g[A1][0] = 1;
            Callee {
                addr: 0x002d_7dd8,
                f: ee::s09::Avalanche_ReleaseInstance,
            }
            .call(c, 0x002d_8200)?;
            c.g[S0][0] = Addr(r32(c, S0).wrapping_add(0x2e4)).lw(c);
            let more = c.g[S0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_81f8);
            }
        }
    }
    c.cycles += 12;
    Addr(r32(c, S1)).sw(c, 0);
    call_ptr(c, 0x0028_b180, 0x002d_8214)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0029_def0, 0x002d_8220)?;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0xa8)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x708)).lw(c);
    let zero = c.g[V0][0] == 0;
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
    if !zero {
        c.cycles += 1;
        let count = c.g[V0][0];
        Addr(r32(c, V1).wrapping_add(0x708)).sw(c, count);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// `Avalanche_ExportHistoryFiles` (0x2d8258): write the three history files.

/// `void Avalanche_ExportHistoryFiles(void)` (0x2d8258).
pub fn avalanche_export_history_files(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x004a_3ad0);
        c.g[A0][0] = 1;
        Callee {
            addr: 0x002d_83b8,
            f: ee::s09::Avalanche_WriteHistoryFile,
        }
        .call(c, 0x002d_8270)?;
        c.g[A1][0] = sx(0x004a_3ad8);
        c.g[A0][0] = 1;
        Callee {
            addr: 0x002d_83b8,
            f: ee::s09::Avalanche_WriteHistoryFile,
        }
        .call(c, 0x002d_8280)?;
        c.g[A1][0] = sx(0x004a_3ae0);
        c.g[A0][0] = 0;
        Callee {
            addr: 0x002d_83b8,
            f: ee::s09::Avalanche_WriteHistoryFile,
        }
        .call(c, 0x002d_8290)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Avalanche_WriteData` (0x2d82a0): write `rows` rows of `stride` bytes. A
// direct write goes out as it is; otherwise each row is reversed into a
// buffer first. Returns `stride * rows`.

/// `int Avalanche_WriteData(direct, handle, offset, src, stride, rows)`
/// (0x2d82a0).
#[allow(dead_code)]
pub fn avalanche_write_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[T1][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[T0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        c.g[S5][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 9;
            c.mult(0, 0, S3, S1);
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.lo[0];
            c.g[T0][0] = 0x64;
            call_ptr(c, 0x003d_ec80, 0x002d_82f4)?;
            c.mult(0, 2, S3, S1);
        } else {
            wd_buffered(c)?;
        }
        // The epilogue (`'b_2d8394`).
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The buffered branch of `Avalanche_WriteData`: copy every row reversed
/// into a buffer, write the buffer, and free it. Leaves V0 = stride * rows.
pub(crate) fn wd_buffered(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    c.mult(0, 0, S3, S1);
    c.g[A1][0] = sx(0x004a_3ae8);
    c.g[A2][0] = 0;
    c.g[A0][0] = c.lo[0];
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, 0x002d_8318)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V1][0] = 0;
    let no_rows = (c.g[S1][0] as i64) <= 0;
    c.g[A1][0] = c.g[S2][0];
    if !no_rows {
        loop {
            // One row: the bytes from `end` back to `src`, into the buffer.
            c.cycles += 4;
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, S3)));
            c.g[A2][0] = sx(r32(c, V1).wrapping_add(1));
            let empty_row = c.g[S0][0] == c.g[A0][0];
            c.g[V1][0] = c.g[A0][0];
            if !empty_row {
                loop {
                    c.cycles += 7;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.g[V0][0] = Addr(r32(c, V1)).lbu(c);
                    let byte = c.g[V0][0];
                    Addr(r32(c, A1)).sb(c, byte);
                    let more = c.g[S0][0] != c.g[V1][0];
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_8338);
                    }
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[A2][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more_rows = c.g[V0][0] != 0;
            c.g[S0][0] = c.g[A0][0];
            if !more_rows {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_8328);
            }
        }
    }
    c.cycles += 9;
    c.mult(0, 0, S3, S1);
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.lo[0];
    c.g[T0][0] = 0x64;
    call_ptr(c, 0x003d_ec80, 0x002d_8380)?;
    let no_buf = c.g[S2][0] == 0;
    c.mult(0, 2, S3, S1);
    if !no_buf {
        c.cycles += 3;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_7e98, 0x002d_8390)?;
        c.mult(0, 2, S3, S1);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// `Avalanche_LoadHistoryFile` (0x2d87d0): open the history file of the
// current location, read its header, and load the history when the magic
// matches; then apply the loaded history.

/// `void Avalanche_LoadHistoryFile(void)` (0x2d87d0).
pub fn avalanche_load_history_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x140);
        frame.save(c, S0, 0x130);
        frame.save(c, S1, 0x120);
        frame.save_ra(c, 0x110);
        c.g[S0][0] = 0x004a_0000;
        call_ptr(c, 0x0014_dc80, 0x002d_87e8)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x002d_87f4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x3af0));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0014_4be0, 0x002d_8804)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4c48, 0x002d_8810)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = sx(0x004a_3ad8);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0041_7828, 0x002d_8828)?;
        c.g[V1][0] = 1;
        Addr(r32(c, GP).wrapping_add(0x9cc)).sw(c, 0);
        let one = c.g[V1][0];
        Addr(r32(c, GP).wrapping_add(0x9d4)).sw(c, one);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 0x64;
        c.g[A3][0] = sx(sp(c).wrapping_add(0x100));
        call_ptr(c, 0x003d_ebf0, 0x002d_8848)?;
        if c.g[V0][0] != 0 {
            lhf_read_file(c)?;
        }
        // The tail (`'b_2d892c`): the history is applied either way.
        c.cycles += 7;
        Addr(r32(c, GP).wrapping_add(0x95c)).sw(c, 0);
        Callee {
            addr: 0x002d_a028,
            f: ee::s09::Avalanche_ApplyHistoryData,
        }
        .call(c, 0x002d_8934)?;
        frame.restore(c, S0, 0x130);
        frame.restore(c, S1, 0x120);
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    })
}

/// The opened-file branch of `Avalanche_LoadHistoryFile`: drop the old file
/// data when it is owned, read the 8-byte header, and when its magic matches
/// read the history into a new buffer.
pub(crate) fn lhf_read_file(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    Callee {
        addr: 0x002d_9a80,
        f: ee::s09::Avalanche_FreeHistories,
    }
    .call(c, 0x002d_8858)?;
    c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0x9c4)).lw(c);
    let has_data = c.g[A0][0] != 0;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(0x9c8)).lw(c);
    // The file data is freed when it exists and is owned; an unowned one
    // skips the shared tail (`break 'b_2d8878`).
    let mut tail = true;
    if has_data {
        c.cycles += 2;
        let owned = c.g[V0][0] != 0;
        c.g[A1][0] = sx(sp(c));
        if owned {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x002d_8874)?;
        } else {
            tail = false;
        }
    }
    if tail {
        c.cycles += 1;
        c.g[A1][0] = sx(sp(c));
    }
    // Read the 8-byte header into a new buffer.
    c.cycles += 16;
    c.g[A0][0] = 8;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, 0x002d_8888)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = Addr(sp(c).wrapping_add(0x100)).lw(c);
    c.g[A1][0] = 0;
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = 8;
    c.g[T0][0] = 0x64;
    call_ptr(c, 0x003d_ec60, 0x002d_88a4)?;
    c.g[V1][0] = 0x02be_0000 | 0xef00;
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    let magic_ok = c.g[V0][0] == c.g[V1][0];
    if !magic_ok {
        // Bad magic: drop the header buffer and the file data pointer.
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            Addr(r32(c, GP).wrapping_add(0x9c4)).sw(c, 0);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e98, 0x002d_88c8)?;
            Addr(r32(c, GP).wrapping_add(0x9c4)).sw(c, 0);
        }
        c.cycles += 2;
        Addr(r32(c, GP).wrapping_add(0x9c8)).sw(c, 0);
        return Ok(());
    }
    // Good magic: the size is in the header; free the header buffer.
    c.cycles += 2;
    let hdr_null = c.g[S0][0] == 0;
    c.g[S1][0] = Addr(r32(c, S0).wrapping_add(0x4)).lw(c);
    if !hdr_null {
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7e98, 0x002d_88e4)?;
    }
    // Allocate the history buffer, read it, and close the file.
    c.cycles += 18;
    c.g[A1][0] = c.g[SP][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, 0x002d_88f8)?;
    c.g[A0][0] = Addr(sp(c).wrapping_add(0x100)).lw(c);
    c.g[V1][0] = 1;
    c.g[A2][0] = c.g[V0][0];
    c.g[A3][0] = sx(r32(c, S1).wrapping_add(8));
    let one = c.g[V1][0];
    Addr(r32(c, GP).wrapping_add(0x9c8)).sw(c, one);
    c.g[A1][0] = 0;
    c.g[T0][0] = 0x64;
    let buf = c.g[V0][0];
    Addr(r32(c, GP).wrapping_add(0x9c4)).sw(c, buf);
    call_ptr(c, 0x003d_ec60, 0x002d_891c)?;
    c.g[A0][0] = Addr(sp(c).wrapping_add(0x100)).lw(c);
    c.g[A1][0] = 0x64;
    call_ptr(c, 0x003d_eca0, 0x002d_8928)?;
    Addr(r32(c, GP).wrapping_add(0x9d4)).sw(c, 0);
    Ok(())
}

// ---------------------------------------------------------------------------
// Avalanche_Start (0x2d97a8) and Avalanche_LoadState (0x2d9d68)

/// The 16 active-avalanche slots (`cAvalanche_m_activeAvalanches`, 0x1c bytes
/// each: avalanche, active-node list, .., time at +0x18).
pub(crate) const AVS_SLOTS: u32 = 0x0054_0000 - 0x76c8;

pub(crate) const AVS_SLOTS_END: u32 = AVS_SLOTS + 0x1c0;

pub(crate) const AVS_SLOT_SIZE: u32 = 0x1c;

/// The 64 active nodes (`cAvalanche_m_activeNodes`, 0x2f0 bytes each).
pub(crate) const AVS_NODES: u32 = 0x004f_0000 - 0x1890;

pub(crate) const AVS_NODE_SIZE: u32 = 0x2f0;

/// `t4mat_Identity`.
pub(crate) const AVS_IDENTITY: u32 = 0x0050_0000 - 0xe60;

/// "No slot": `0x80000000` as the sign-extended return value.
pub(crate) const AVS_NONE: u64 = 0xffff_ffff_8000_0000;

/// `gp` offsets: the avalanche list head and the node-table fields.
pub(crate) const AVS_GP_LIST: u32 = 0x9c0;

pub(crate) const AVS_APPLY_HISTORY: Callee = Callee {
    addr: 0x002d_a028,
    f: ee::s09::Avalanche_ApplyHistoryData,
};

pub(crate) const AVS_FADE: Callee = Callee {
    addr: 0x002d_7c00,
    f: ee::s09::Avalanche_ComputeInstanceFade,
};

pub(crate) const AVS_BUILD_ARRAY: Callee = Callee {
    addr: 0x002d_7e60,
    f: ee::s09::Avalanche_BuildActiveNodeArray,
};

pub(crate) const AVS_STOP: Callee = Callee {
    addr: 0x002d_81b0,
    f: ee::s09::Avalanche_Stop,
};

pub(crate) const AVS_START: Callee = Callee {
    addr: 0x002d_97a8,
    f: ee::s09::Avalanche_Start,
};

pub(crate) const AVS_SAMPLE_HISTORY: Callee = Callee {
    addr: 0x002d_5778,
    f: ee::s09::Avalanche_SampleHistory,
};

pub(crate) const AVS_WEATHER_R: Callee = Callee {
    addr: 0x002e_e7c8,
    f: ee::s09::WorldPainter_GetWeather68,
};

pub(crate) const AVS_WEATHER_G: Callee = Callee {
    addr: 0x002e_e810,
    f: ee::s09::WorldPainter_GetWeather70,
};

pub(crate) const AVS_WEATHER_B: Callee = Callee {
    addr: 0x002e_e858,
    f: ee::s09::WorldPainter_GetWeather78,
};

/// `int Avalanche_Start(int id)` (0x2d97a8): start the avalanche with this id
/// in a free one of the 16 slots (-0x80000000 when unknown, already running or
/// out of active nodes, else the slot index). Every node of the avalanche
/// takes a free active node (64 in all) with identity matrices and white
/// colour; emitter nodes also reset their dynamic emitter with the snow-flake
/// colour. Then the node array is built, the avalanche sound is started and the
/// danger manager counts one more active avalanche.
pub fn avalanche_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 21, 0x98);
        frame.save_fpr(c, 20, 0x90);
        AVS_APPLY_HISTORY.call(c, 0x002d_97dc)?;
        let gp = c.g[GP][0] as u32;
        c.g[A1][0] = c.lw(gp.wrapping_add(AVS_GP_LIST));
        c.g[V0][0] = 0x0054_0000;
        'done: {
            'none: {
                if c.g[A1][0] == 0 {
                    break 'none;
                }
                c.cycles += 2;
                c.g[S6][0] = sx(AVS_SLOTS);
                c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                // For each avalanche in the list, until the id matches.
                'avalanches: loop {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S0][0] {
                        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = 0x0054_0000;
                        c.g[V1][0] = c.g[S6][0];
                        c.g[A2][0] = sx(AVS_SLOTS_END);
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        // Already running in a slot?
                        loop {
                            c.cycles += 2;
                            let running = c.g[V0][0] == c.g[A1][0];
                            c.g[V0][0] = AVS_NONE;
                            if running {
                                break 'done;
                            }
                            c.cycles += 4;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(AVS_SLOT_SIZE));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                break;
                            }
                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                            if event_due(c) {
                                deliver_event_here(c, 0x002d_9808);
                            }
                        }
                        c.cycles += 2;
                        c.g[S3][0] = sx(AVS_SLOTS);
                        c.g[V1][0] = 0;
                        // The first free slot.
                        loop {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                            if c.g[V0][0] == 0 {
                                avalanche_start_in_slot(c)?;
                                break 'done;
                            }
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 16i64) as u64;
                            let more = c.g[V0][0] != 0;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(AVS_SLOT_SIZE));
                            if !more {
                                c.cycles += 1;
                                c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x002d_9828);
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[A1][0] == 0 {
                        break 'none;
                    }
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_97f0);
                    }
                    continue 'avalanches;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = AVS_NONE;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// Avalanche_Start, free slot `s3` found (`v1` its index): claim it, give every
/// node of the avalanche (`a1`) an active node, then finish (node array, sound,
/// danger counter) leaving the slot index in `v0`; or release everything again
/// and leave -0x80000000 in `v0` when the 64 active nodes run out.
pub(crate) fn avalanche_start_in_slot(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    c.sw(c.g[S3][0] as u32, c.g[A1][0]);
    c.g[V0][0] = 0x004f_0000;
    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0);
    c.g[S1][0] = sx(AVS_NODES);
    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), 0);
    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0);
    c.sw((c.g[S3][0] as u32).wrapping_add(0x14), 0);
    c.g[S2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    let no_nodes = c.g[S2][0] == 0;
    c.g[S4][0] = 0;
    if !no_nodes {
        c.cycles += 1;
        c.g[V0][0] = ((c.g[S4][0] as i64) < 64i64) as u64;
        // For each node of the avalanche (s2, linked at +0xf4).
        loop {
            c.cycles += 2;
            let full = c.g[V0][0] == 0;
            c.g[V0][0] = 0x0050_0000;
            if !full {
                c.cycles += 4;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.g[A1][0] = sx(AVS_IDENTITY);
                c.g[S5][0] = 1;
                // The next free active node (s1).
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2e0));
                    if c.g[V0][0] == 0 {
                        avalanche_start_init_node(c)?;
                        break;
                    }
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S4][0] as i64) < 64i64) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(AVS_NODE_SIZE));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_9878);
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = 0x40;
            if c.g[S4][0] != c.g[V0][0] {
                c.g[S2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf4));
            } else {
                // Out of active nodes: free the ones taken so far.
                c.cycles += 3;
                c.g[S1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                let empty = c.g[S1][0] == 0;
                c.sw(c.g[S3][0] as u32, 0);
                if empty {
                    c.cycles += 1;
                    c.g[V0][0] = AVS_NONE;
                    return Ok(());
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2e0));
                loop {
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x114), 0);
                    }
                    c.cycles += 4;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x2e0), 0);
                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2e4));
                    if c.g[S1][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2e0));
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_99b8);
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = AVS_NONE;
                return Ok(());
            }
            c.cycles += 2;
            let more = c.g[S2][0] != 0;
            c.g[V0][0] = ((c.g[S4][0] as i64) < 64i64) as u64;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_9860);
            }
        }
    }
    c.cycles += 20;
    c.g[A0][0] = c.g[S3][0];
    AVS_BUILD_ARRAY.call(c, 0x002d_99e8)?;
    call_ptr(c, 0x0028_b180, 0x002d_99f0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    call_ptr(c, 0x0029_def0, 0x002d_99fc)?;
    let gp = c.g[GP][0] as u32;
    c.g[V1][0] = c.lw(gp.wrapping_sub(0x848));
    c.g[A0][0] = 0xffff_ffff_b6db_0000;
    c.g[A0][0] |= 0x6db7;
    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[S6][0] as u32));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
    // The slot index: (slot - slots) / 0x1c as a multiply by the inverse.
    c.mult(0, 2, 2, 4);
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x708));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x708), c.g[V1][0]);
    Ok(())
}

/// Avalanche_Start, free active node `s1` for avalanche node `s2`, slot `s3`:
/// link it into the slot's list, load identity matrices and white colour, scale
/// at 0, and for emitter nodes reset and set up the dynamic emitter with the
/// snow-flake colour (weather 0).
pub(crate) fn avalanche_start_init_node(c: &mut Cpu) -> Flow {
    c.cycles += 43;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x2e0), c.g[S2][0]);
    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x20));
    c.sw((c.g[S2][0] as u32).wrapping_add(0x114), c.g[S1][0]);
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
    c.sw((c.g[S1][0] as u32).wrapping_add(0x2e4), c.g[V0][0]);
    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[S1][0]);
    c.sw(c.g[S1][0] as u32, 0);
    c.lq(2, (c.g[S2][0] as u32).wrapping_add(0xe0));
    c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x10));
    let ident = c.g[A1][0] as u32;
    let to = c.g[V1][0] as u32;
    avalanche_start_copy_matrix(c, ident, to);
    c.sw((c.g[S1][0] as u32).wrapping_add(0x2ec), 0);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf8));
    c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x40));
    let sp0 = c.g[SP][0] as u32;
    c.sq(2, sp0);
    c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x60));
    let to = c.g[A0][0] as u32;
    avalanche_start_copy_matrix(c, ident, to);
    c.swc1(0, sp0.wrapping_add(0xc));
    c.g[A0][0] = c.g[S1][0];
    c.swc1(0, sp0);
    c.swc1(0, sp0.wrapping_add(0x4));
    c.swc1(0, sp0.wrapping_add(0x8));
    c.mtc1(0, 12);
    c.lq(2, sp0);
    c.sq(2, (c.g[S1][0] as u32).wrapping_add(0xc0));
    AVS_FADE.call(c, 0x002d_9924)?;
    c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0xf2));
    let has_emitter = c.g[V0][0] != 0;
    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
    if has_emitter {
        c.cycles += 23;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0037_1600, 0x002d_9938)?;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 12);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0037_0dc8, 0x002d_994c)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x2d0), c.g[S5][0]);
        c.g[A0][0] = 0;
        AVS_WEATHER_R.call(c, 0x002d_9958)?;
        c.mov_s(21, 0);
        c.g[A0][0] = 0;
        AVS_WEATHER_G.call(c, 0x002d_9964)?;
        c.mov_s(20, 0);
        c.g[A0][0] = 0;
        AVS_WEATHER_B.call(c, 0x002d_9970)?;
        let sp0 = c.g[SP][0] as u32;
        c.swc1(21, sp0);
        c.swc1(20, sp0.wrapping_add(0x4));
        c.swc1(0, sp0.wrapping_add(0x8));
        c.sw(sp0.wrapping_add(0xc), 0);
        c.lq(2, sp0);
        c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x1e0));
    }
    Ok(())
}

/// `vu0CopyMatrixAV`: copy the 4x4 matrix at `from` to `to` through VU0
/// registers vf1..vf4.
pub(crate) fn avalanche_start_copy_matrix(c: &mut Cpu, from: u32, to: u32) {
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, from.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, to.wrapping_add(0x10 * i));
    }
}

/// `void Avalanche_LoadState(ReplayStream *s)` (0x2d9d68): restore the active
/// avalanches from a replay frame. Stops all 16 slots, reads the 0x80-byte
/// record (16 ids, 16 times) through the stream's vtable (+0x10 / +0x14), then
/// restarts each avalanche with a valid id and winds it to its saved time: the
/// time is stored in the slot, each active node recomputes its fade and
/// history sample (emitter nodes flagged to re-emit), and the slot's collision
/// and ground-collision cursors skip every event up to that time.
pub fn avalanche_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x150);
        c.g[V0][0] = 0x0054_0000;
        frame.save(c, S0, 0x140);
        c.g[V0][0] = sx(AVS_SLOTS);
        frame.save(c, S1, 0x130);
        c.g[S0][0] = c.g[V0][0];
        frame.save(c, S2, 0x120);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x110);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c0));
        frame.save(c, S4, 0x100);
        frame.save(c, S5, 0xf0);
        frame.save_ra(c, 0xe0);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        // Stop every slot in use.
        loop {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            let idle = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(AVS_SLOT_SIZE));
            if !idle {
                c.cycles += 2;
                AVS_STOP.call(c, 0x002d_9db4)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            if event_due(c) {
                deliver_event_here(c, 0x002d_9da0);
            }
        }
        // s->Read(buf, 0x80) through the vtable at +0x10 (delta) / +0x14 (fn).
        c.cycles += 12;
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x80;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let read = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, read, 0x002d_9ddc)?;
        c.g[V1][0] = 0x0054_0000;
        c.g[A0][0] = 0;
        c.g[S5][0] = sx(AVS_SLOTS);
        c.g[S0][0] = sx((c.g[A0][0] as u32) << 2);
        // For each of the 16 saved entries (a0 = i, s0 = 4 * i).
        loop {
            c.cycles += 5;
            c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            if (c.g[A0][0] as i64) < 0 {
                c.g[A0][0] = c.g[S4][0];
            } else {
                avalanche_load_state_entry(c)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 16i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[A0][0] as u32) << 2);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9df0);
                }
                continue;
            }
            break;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore(c, S4, 0x100);
        frame.restore(c, S5, 0xf0);
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// Avalanche_LoadState, one saved entry whose id (`a0`) is not negative:
/// start it and wind the new slot to the saved time. Ends with `a0 = s4` (the
/// next index).
pub(crate) fn avalanche_load_state_entry(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    AVS_START.call(c, 0x002d_9e0c)?;
    c.g[A0][0] = c.g[V0][0];
    let failed = (c.g[A0][0] as i64) < 0;
    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    'next: {
        if failed {
            break 'next;
        }
        c.cycles += 9;
        c.g[V1][0] = AVS_SLOT_SIZE as u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.mult(0, 5, 4, 3);
        c.lwc1(1, c.g[V0][0] as u32);
        c.mtc1(0, 0);
        c.c_eq_s(1, 0);
        c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S5][0] as u32));
        let zero_time = c.fcc();
        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
        if zero_time {
            break 'next;
        }
        // The slot's active nodes.
        c.cycles += 3;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        if c.g[S0][0] == 0 {
            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
        } else {
            c.cycles += 4;
            c.g[S2][0] = c.g[V0][0];
            c.g[S3][0] = 1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e0));
            loop {
                c.cycles += 3;
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xf2));
                let no_emitter = c.g[V1][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !no_emitter {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2ec), c.g[S3][0]);
                }
                c.cycles += 9;
                c.lwc1(12, c.g[S2][0] as u32);
                AVS_FADE.call(c, 0x002d_9e70)?;
                c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                AVS_SAMPLE_HISTORY.call(c, 0x002d_9e80)?;
                c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e4));
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                    break;
                }
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e0));
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9e58);
                }
            }
        }
        // Skip the collisions (8-byte records, time first) up to the time.
        'collisions: {
            c.cycles += 5;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let none = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
            if !none {
                c.cycles += 8;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
                c.lwc1(0, c.g[V0][0] as u32);
                c.c_le_s(0, 1);
                if !c.fcc() {
                    c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                    break 'collisions;
                }
                c.cycles += 3;
                c.g[A3][0] = c.g[V1][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                loop {
                    c.cycles += 7;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
                    c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    let end = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if end {
                        break;
                    }
                    c.cycles += 5;
                    c.lwc1(0, c.g[V1][0] as u32);
                    c.c_le_s(0, 1);
                    let passed = c.fcc();
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    if !passed {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_9ed0);
                    }
                }
            }
            c.cycles += 1;
            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
        }
        // Ground collisions (4-byte records at +0x110: node id, time).
        c.cycles += 5;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0xe));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let none = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        if none {
            break 'next;
        }
        c.cycles += 10;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x110));
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = c.lhu(c.g[V0][0] as u32);
        c.mtc1(3, 0);
        c.cvt_s_w(0, 0);
        c.c_le_s(0, 1);
        if !c.fcc() {
            // The first ground collision is later than the time: next entry
            // without the extra cycle.
            c.g[A0][0] = c.g[S4][0];
            return Ok(());
        }
        c.cycles += 2;
        c.g[A3][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        loop {
            c.cycles += 7;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.g[A1][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0xe));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let end = c.g[V0][0] == 0;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
            if end {
                break 'next;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lhu(c.g[V1][0] as u32);
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.c_le_s(0, 1);
            let passed = c.fcc();
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            if !passed {
                break 'next;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_9f48);
            }
        }
    }
    c.cycles += 1;
    c.g[A0][0] = c.g[S4][0];
    Ok(())
}

/// `cAvalanche::setAvalancheNodeEmitterData(id, params)` (`Avalanche_SetNodeEmitter`,
/// 0x2d9538). Walks the avalanche groups (`cAvalanche_m_avalancheList`, gp+0x9c0)
/// and their node lists for the node whose instance is `id`. A permanent node
/// (base type 2 at `+0xf0`) is left alone; any other gets `+0xf2 = 1` and the
/// 0xd8 bytes of `*params` copied over its start. Nothing found: no change.
///
/// Register and cycle results follow the generated body block by block.
pub fn avalanche_set_node_emitter(c: &mut Cpu) {
    'done: {
        c.cycles += 3;
        c.g[A2][0] = Addr(c.g[GP][0] as u32).off(0x9c0).lw(c);
        if c.g[A2][0] == 0 {
            break 'done;
        }
        c.cycles += 1;
        c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x4).lw(c);
        'groups: loop {
            'group: {
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    c.g[A2][0] = Addr(c.g[A2][0] as u32).off(0x8).lw(c);
                    break 'group;
                }
                c.cycles += 4;
                c.g[A3][0] = 0x2;
                c.g[T0][0] = 0x1;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xf8).lw(c);
                'nodes: loop {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A0][0] {
                        c.g[V1][0] = Addr(c.g[V1][0] as u32).off(0xf4).lw(c);
                        c.cycles += 2;
                        if c.g[V1][0] != 0 {
                            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xf8).lw(c);
                            if event_due(c) {
                                deliver_event_here(c, 0x002d_9560);
                            }
                            continue 'nodes;
                        }
                        c.cycles += 1;
                        c.g[A2][0] = Addr(c.g[A2][0] as u32).off(0x8).lw(c);
                        break 'group;
                    }
                    // The instance matches: this node's base type decides.
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xf0).lhu(c);
                    c.g[A0][0] = c.g[V1][0];
                    if c.g[V0][0] == c.g[A3][0] {
                        break 'done;
                    }
                    copy_emitter_data(c);
                    return;
                }
            }
            c.cycles += 2;
            if c.g[A2][0] == 0 {
                break 'done;
            }
            c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x4).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x002d_9548);
            }
            continue 'groups;
        }
    }
    c.cycles += 2;
    ret(c);
}

/// The emitter part of `Avalanche_SetNodeEmitter` for a matched, non-permanent
/// node (A0 = node, A1 = params): `+0xf2 = T0` (1), then `*params` (0xd8
/// bytes) into the node. The copy is 8-byte doublewords when both pointers
/// are 8-byte aligned, else unaligned `ldl/ldr` pairs for the first 0xc0 bytes;
/// both finish with the same 0x18-byte tail. Returns to the caller.
pub(crate) fn copy_emitter_data(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.g[A1][0] | c.g[A0][0];
    c.g[V0][0] &= 0x7;
    let aligned = c.g[V0][0] == 0;
    let flag = c.g[T0][0];
    Addr(c.g[V1][0] as u32).off(0xf2).sb(c, flag);
    if aligned {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc0));
        loop {
            c.cycles += 11;
            let src = c.g[A1][0] as u32;
            let dst = c.g[A0][0] as u32;
            c.g[V1][0] = Addr(src).ld(c);
            c.g[A2][0] = Addr(src.wrapping_add(0x8)).ld(c);
            c.g[A3][0] = Addr(src.wrapping_add(0x10)).ld(c);
            c.g[T0][0] = Addr(src.wrapping_add(0x18)).ld(c);
            let (v1, a2, a3, t0) = (c.g[V1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]);
            Addr(dst).sd(c, v1);
            Addr(dst.wrapping_add(0x8)).sd(c, a2);
            Addr(dst.wrapping_add(0x10)).sd(c, a3);
            Addr(dst.wrapping_add(0x18)).sd(c, t0);
            c.g[A1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[A1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_95e0);
            }
        }
    } else {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc0));
        loop {
            c.cycles += 19;
            let src = c.g[A1][0] as u32;
            let dst = c.g[A0][0] as u32;
            c.g[V1][0] = Addr(src).read_u64_unaligned(c);
            c.g[A2][0] = Addr(src.wrapping_add(0x8)).read_u64_unaligned(c);
            c.g[A3][0] = Addr(src.wrapping_add(0x10)).read_u64_unaligned(c);
            c.g[T0][0] = Addr(src.wrapping_add(0x18)).read_u64_unaligned(c);
            let (v1, a2, a3, t0) = (c.g[V1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]);
            Addr(dst).write_u64_unaligned(c, v1);
            Addr(dst.wrapping_add(0x8)).write_u64_unaligned(c, a2);
            Addr(dst.wrapping_add(0x10)).write_u64_unaligned(c, a3);
            Addr(dst.wrapping_add(0x18)).write_u64_unaligned(c, t0);
            c.g[A1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[A1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_9588);
            }
        }
    }
    // Tail: the last 0x18 bytes of the 0xd8-byte struct.
    c.cycles += 13;
    let src = c.g[A1][0] as u32;
    let dst = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(src).read_u64_unaligned(c);
    c.g[V1][0] = Addr(src.wrapping_add(0x8)).read_u64_unaligned(c);
    c.g[A2][0] = Addr(src.wrapping_add(0x10)).read_u64_unaligned(c);
    let (v0, v1, a2) = (c.g[V0][0], c.g[V1][0], c.g[A2][0]);
    Addr(dst).write_u64_unaligned(c, v0);
    Addr(dst.wrapping_add(0x8)).write_u64_unaligned(c, v1);
    Addr(dst.wrapping_add(0x10)).write_u64_unaligned(c, a2);
    ret(c);
}

/// `Avalanche_BakeAll` (0x2d96e0): frees the histories
/// (`Avalanche_FreeHistories`, 0x2d9a80), bakes each group of the avalanche
/// list (`gp+0x9c0`, linked through `+8`) with `Avalanche_BakeGroup` (0x2d9660),
/// then exports and reloads the history files (0x2d8258, 0x2d87d0).
pub fn avalanche_bake_all_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002d_9a80, 0x002d_96f4)?;
        c.g[S0][0] = Addr(c.g[GP][0] as u32).off(0x9c0).lw(c);
        if c.g[S0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002d_9660, 0x002d_9708)?;
                c.g[S0][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9700);
                }
            }
        }
        c.cycles += 8;
        call_ptr(c, 0x002d_8258, 0x002d_971c)?;
        call_ptr(c, 0x002d_87d0, 0x002d_9724)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// `Avalanche_UpdateInstance` (0x2d7ca8): advance one active avalanche node by
// `t` (scaled by the data block's rate), sample its history, and either copy
// the frame into the node and emit particles, or end the node. Returns in V0
// whether the node is still alive.

/// `tActiveAvalancheNode` fields.
pub(crate) const AU_DATA: u32 = 0x2e0;

pub(crate) const AU_FB4: u32 = 0xb4;

pub(crate) const AU_CUR_POS: u32 = 0x60;

pub(crate) const AU_CUR_MAT: u32 = 0x70;

pub(crate) const AU_CUR_COLOUR: u32 = 0xc0;

pub(crate) const AU_EMITTER: u32 = 0xd0;

pub(crate) const AU_F1FC: u32 = 0x1fc;

/// `sAvData` fields: length, rate, type (u16), emits (u8).
pub(crate) const AU_D_LENGTH: u32 = 0x100;

pub(crate) const AU_D_RATE: u32 = 0x10c;

pub(crate) const AU_D_TYPE: u32 = 0xf0;

pub(crate) const AU_D_EMITS: u32 = 0xf2;

/// The sampled frame in this function's stack frame: position at +0, colour
/// at +0x10, matrix at +0x20.
pub(crate) const AU_OUT_COLOUR: u32 = 0x10;

pub(crate) const AU_OUT_MAT: u32 = 0x20;

/// `bool Avalanche_UpdateInstance(self, float t)` (0x2d7ca8). A node without
/// a data block is dead (V0 = 0). Otherwise the history is sampled at
/// `t * rate`; at 30 samples per second a position past the data's length
/// ends the node (released, V0 = 0) unless the data is of type 2 (which stays
/// alive); inside, the sampled position, matrix and colour become the node's
/// current ones and a type-other-than-2 emitting node spawns particles at it
/// through `DynamicEmitter_SpawnAt`. V0 = 1 for a node that lives on.
pub fn oc1075_avalanche_update_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x70);
        frame.save_fpr(c, 20, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x60);
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(AU_DATA)).lw(c);
        let has_data = c.g[V0][0] != 0;
        c.mov_s(20, 12);
        if has_data {
            au_sample_and_emit(c)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// The body for a node with data (V0 = its data block on entry): sample the
/// history at `t * rate`; past the length, end the node; otherwise copy the
/// sampled frame into the node and emit (or just stay alive when it emits
/// nothing).
pub(crate) fn au_sample_and_emit(c: &mut Cpu) -> Flow {
    c.cycles += 15;
    let data = r32(c, V0);
    c.lwc1(0, data.wrapping_add(AU_D_RATE));
    c.g[A0][0] = sx(sp(c));
    c.g[A1][0] = sx(r32(c, S0));
    c.mul_s(20, 20, 0);
    c.mov_s(12, 20);
    Callee {
        addr: 0x002d_5778,
        f: ee::s09::Avalanche_SampleHistory,
    }
    .call(c, 0x002d_7ce8)?;
    c.g[AT][0] = 0x41f0_0000; // 30.0
    c.mtc1(AT, 0);
    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(AU_DATA)).lw(c);
    c.mul_s(0, 20, 0);
    let length = r32(c, V0).wrapping_add(AU_D_LENGTH);
    c.lwc1(1, length);
    c.c_lt_s(0, 1);
    let inside = c.fcc();
    c.g[A1][0] = sx(r32(c, S0).wrapping_add(AU_CUR_POS));
    if !inside {
        return au_past_end(c);
    }
    // Copy the sampled frame into the node: position (v0), matrix, colour.
    c.cycles += 19;
    c.lq(2, sp(c));
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(AU_CUR_MAT));
    let out_mat = sp(c).wrapping_add(AU_OUT_MAT);
    c.g[V1][0] = sx(out_mat);
    c.sq(2, r32(c, S0).wrapping_add(AU_CUR_POS));
    c.lqc2(1, out_mat);
    c.lqc2(2, out_mat.wrapping_add(0x10));
    c.lqc2(3, out_mat.wrapping_add(0x20));
    c.lqc2(4, out_mat.wrapping_add(0x30));
    let cur_mat = r32(c, A0);
    c.sqc2(1, cur_mat);
    c.sqc2(2, cur_mat.wrapping_add(0x10));
    c.sqc2(3, cur_mat.wrapping_add(0x20));
    c.sqc2(4, cur_mat.wrapping_add(0x30));
    c.lq(2, sp(c).wrapping_add(AU_OUT_COLOUR));
    c.g[T1][0] = Addr(r32(c, S0).wrapping_add(AU_DATA)).lw(c);
    c.g[A0][0] = 2;
    c.sq(2, r32(c, S0).wrapping_add(AU_CUR_COLOUR));
    c.g[V1][0] = Addr(r32(c, T1).wrapping_add(AU_D_TYPE)).lhu(c);
    let typed = c.g[V1][0] == c.g[A0][0];
    c.g[A3][0] = sx(r32(c, S0).wrapping_add(AU_CUR_COLOUR));
    if typed {
        return au_alive(c);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, T1).wrapping_add(AU_D_EMITS)).lbu(c);
    let silent = c.g[V0][0] == 0;
    c.g[V0][0] = 0x0050_0000;
    if silent {
        return au_alive(c);
    }
    au_emit(c)
}

/// The sample is past the node's length: a type-2 node stays alive, any other
/// one is released and ends (V0 = 0).
pub(crate) fn au_past_end(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(AU_D_TYPE)).lhu(c);
    c.g[V0][0] = 2;
    let typed = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = sx(r32(c, S0));
    if typed {
        return au_alive(c);
    }
    c.cycles += 4;
    c.g[A1][0] = 0;
    Callee {
        addr: 0x002d_7dd8,
        f: ee::s09::Avalanche_ReleaseInstance,
    }
    .call(c, 0x002d_7db8)?;
    c.g[V0][0] = 0;
    Ok(())
}

/// The node stays alive (V0 = 1) after its frame was copied.
pub(crate) fn au_alive(c: &mut Cpu) -> Flow {
    c.cycles += 1;
    c.g[V0][0] = 1;
    Ok(())
}

/// Emit particles from the node's emitter at the sampled position
/// (`DynamicEmitter_SpawnAt(dt, emitter, pos, &zero, colour, before_end)`);
/// the node stays alive.
pub(crate) fn au_emit(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.lwc1(0, r32(c, S0).wrapping_add(AU_FB4));
    c.g[A2][0] = sx(r32(c, V0).wrapping_sub(0xee0));
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(AU_EMITTER));
    c.swc1(0, r32(c, S0).wrapping_add(AU_F1FC));
    c.lwc1(0, r32(c, T1).wrapping_add(AU_D_LENGTH));
    c.c_lt_s(20, 0);
    let before_end = c.fcc();
    c.g[T0][0] = 1;
    if !before_end {
        c.cycles += 1;
        c.g[T0][0] = 0;
    }
    c.cycles += 5;
    c.lwc1(12, r32(c, GP).wrapping_sub(0x3cd4));
    call_ptr(c, 0x0037_17c0, 0x002d_7d98)?;
    c.g[V0][0] = 1;
    Ok(())
}

// ---------------------------------------------------------------------------
// `Avalanche_WriteData` (0x2d82a0).

/// `int Avalanche_WriteData(direct, handle, offset, src, stride, rows)`
/// (0x2d82a0): write `rows` rows of `stride` bytes at `src` to a file through
/// `EAFile_Write`. A direct write goes out as it is; otherwise each row is
/// byte-reversed into a temporary buffer first (the history file is
/// big-endian), which is written and freed. Returns `stride * rows`.
pub fn oc1075_avalanche_write_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[T1][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[T0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        c.g[S5][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 9;
            c.mult(0, 0, S3, S1);
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.lo[0];
            c.g[T0][0] = 0x64;
            call_ptr(c, 0x003d_ec80, 0x002d_82f4)?;
            c.mult(0, 2, S3, S1);
        } else {
            aw_buffered(c)?;
        }
        // The epilogue (`'b_2d8394`).
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The buffered branch of `Avalanche_WriteData`: copy every row reversed into
/// a buffer, write the buffer, and free it. Leaves V0 = stride * rows.
pub(crate) fn aw_buffered(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    c.mult(0, 0, S3, S1);
    c.g[A1][0] = sx(0x004a_3ae8);
    c.g[A2][0] = 0;
    c.g[A0][0] = c.lo[0];
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7e30, 0x002d_8318)?; // operator_new_named
    c.g[S2][0] = c.g[V0][0];
    c.g[V1][0] = 0;
    let no_rows = (c.g[S1][0] as i64) <= 0;
    c.g[A1][0] = c.g[S2][0];
    if !no_rows {
        loop {
            // One row: the bytes from `end` back to `src`, into the buffer.
            c.cycles += 4;
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, S3)));
            c.g[A2][0] = sx(r32(c, V1).wrapping_add(1));
            let empty_row = c.g[S0][0] == c.g[A0][0];
            c.g[V1][0] = c.g[A0][0];
            if !empty_row {
                loop {
                    c.cycles += 7;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.g[V0][0] = Addr(r32(c, V1)).lbu(c);
                    let byte = c.g[V0][0];
                    Addr(r32(c, A1)).sb(c, byte);
                    let more = c.g[S0][0] != c.g[V1][0];
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_8338);
                    }
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[A2][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more_rows = c.g[V0][0] != 0;
            c.g[S0][0] = c.g[A0][0];
            if !more_rows {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_8328);
            }
        }
    }
    c.cycles += 9;
    c.mult(0, 0, S3, S1);
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.lo[0];
    c.g[T0][0] = 0x64;
    call_ptr(c, 0x003d_ec80, 0x002d_8380)?; // EAFile_Write
    let no_buf = c.g[S2][0] == 0;
    c.mult(0, 2, S3, S1);
    if !no_buf {
        c.cycles += 3;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_7e98, 0x002d_8390)?; // EA_MemFree
        c.mult(0, 2, S3, S1);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// `Avalanche_GetProximityInfo` (0x2da1c0).

/// `void Avalanche_GetProximityInfo(out, pos)` (0x2da1c0), a.k.a.
/// `cAvalanche::getAvalancheDataByPosition`: scans the 16 active avalanche
/// slots (0x1c bytes each at 0x538938: avalanche ptr, node list head) and
/// every live node (`+0x2e0` data, `+0x2e4` next) whose position (`+0x60`) is
/// closer than 15000 units (squared distance below 225e6) to `pos`: sums the
/// positions into `out` (a quad, w = 1 each), sums the node scales (`+0xb0`),
/// takes the smallest squared distance, counts the nodes and sets the "any"
/// flag unless a counted node is of type 2. Finally divides the centre and
/// the scale sum by the count. `out` = { centre.xyzw, minDist, count, total...}.
pub fn oc1075_avalanche_get_proximity_info(c: &mut Cpu) {
    c.cycles += 24;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x60));
    c.g[V0][0] = 0x0050_0000;
    c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
    c.ldr(3, c.g[A1][0] as u32);
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    c.sdl(3, sp(c).wrapping_add(0x7));
    c.sdr(3, sp(c));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
    c.sw(sp(c).wrapping_add(0x8), c.g[A2][0]);
    c.g[T2][0] = c.g[A0][0];
    c.lq(3, c.g[V0][0] as u32);
    c.g[A1][0] = 0x0054_0000;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3cbc));
    c.g[T4][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x76c8));
    c.sq(3, sp(c).wrapping_add(0x10));
    c.g[T0][0] = c.g[SP][0];
    c.swc1(0, sp(c).wrapping_add(0x24));
    c.g[A0][0] = 0;
    c.sw(sp(c).wrapping_add(0x28), 0);
    c.g[T3][0] = 0x1c;
    c.sw(sp(c).wrapping_add(0x20), 0);
    c.sw(sp(c).wrapping_add(0x2c), 0);
    c.mult(0, 2, 4, 11);
    'l_2da220: loop {
        'b_2da32c: {
            'b_2da328: {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                let empty_slot = c.g[V0][0] == 0;
                c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                if empty_slot {
                    break 'b_2da328;
                }
                c.cycles += 3;
                c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                if c.g[A3][0] == 0 {
                    c.g[A0][0] = c.g[T1][0];
                    break 'b_2da32c;
                }
                c.cycles += 5;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 5);
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3cb8));
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2e0));
                loop {
                    // One node: skipped without data, or when too far away.
                    'b_2da320: {
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            c.g[A3][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2e4));
                            break 'b_2da320;
                        }
                        c.cycles += 27;
                        c.lwc1(0, c.g[T0][0] as u32);
                        c.swc1(0, sp(c).wrapping_add(0x40));
                        c.lwc1(1, (c.g[T0][0] as u32).wrapping_add(0x4));
                        c.swc1(1, sp(c).wrapping_add(0x44));
                        c.lwc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                        c.swc1(5, sp(c).wrapping_add(0x4c));
                        c.swc1(0, sp(c).wrapping_add(0x48));
                        c.lqc2(3, sp(c).wrapping_add(0x40));
                        c.lqc2(4, (c.g[A3][0] as u32).wrapping_add(0x60));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, sp(c).wrapping_add(0x50));
                        c.lq(2, sp(c).wrapping_add(0x50));
                        c.sq(2, sp(c).wrapping_add(0x30));
                        c.lqc2(3, sp(c).wrapping_add(0x30));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.vu0().sqrt(4, 0);
                        c.vu0().waitq();
                        c.cfc2(3, 22);
                        c.mtc1(3, 3);
                        c.c_lt_s(3, 4);
                        if !c.fcc() {
                            c.g[A3][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2e4));
                            break 'b_2da320;
                        }
                        // Close enough: accumulate centre, scale, min, count, any.
                        c.cycles += 23;
                        c.lqc2(3, sp(c).wrapping_add(0x10));
                        c.lqc2(4, (c.g[A3][0] as u32).wrapping_add(0x60));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, sp(c).wrapping_add(0x10));
                        c.lwc1(2, (c.g[A3][0] as u32).wrapping_add(0xb0));
                        c.lwc1(1, sp(c).wrapping_add(0x20));
                        c.lwc1(0, sp(c).wrapping_add(0x24));
                        c.add_s(1, 1, 2);
                        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x2c));
                        c.min_s(0, 0, 3);
                        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x28));
                        c.swc1(0, sp(c).wrapping_add(0x24));
                        c.g[A0][0] = c.g[A2][0] | 0x1;
                        c.swc1(1, sp(c).wrapping_add(0x20));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2e0));
                        c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0xf0));
                        c.sw(sp(c).wrapping_add(0x28), c.g[A1][0]);
                        c.g[V1][0] ^= 0x2;
                        if c.g[V1][0] != 0 {
                            c.g[A0][0] = c.g[A2][0];
                        }
                        c.g[A0][0] = (0u64 < c.g[A0][0]) as u64;
                        c.sw(sp(c).wrapping_add(0x2c), c.g[A0][0]);
                        c.g[A3][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2e4));
                    }
                    c.cycles += 2;
                    if c.g[A3][0] != 0 {
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x2e0));
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_a250);
                        }
                        continue;
                    }
                    break 'b_2da328;
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[T1][0];
            break 'b_2da32c;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A0][0] as i64) < 16i64) as u64;
        let more_slots = c.g[V0][0] != 0;
        c.mult(0, 2, 4, 11);
        if more_slots {
            if event_due(c) {
                deliver_event_here(c, 0x002d_a220);
            }
            continue 'l_2da220;
        }
        // Divide the centre and the scale sum by the count.
        c.cycles += 21;
        c.lwc1(0, sp(c).wrapping_add(0x28));
        c.cvt_s_w(0, 0);
        c.g[V0][0] = c.g[T2][0];
        c.mfc1(3, 0);
        c.qmtc2(3, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, sp(c).wrapping_add(0x10));
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, sp(c).wrapping_add(0x10));
        c.lwc1(1, sp(c).wrapping_add(0x28));
        c.cvt_s_w(1, 1);
        c.lwc1(0, sp(c).wrapping_add(0x20));
        c.lq(3, sp(c).wrapping_add(0x10));
        c.div_s(0, 0, 1);
        c.sq(3, c.g[T2][0] as u32);
        c.swc1(0, sp(c).wrapping_add(0x20));
        c.lq(3, sp(c).wrapping_add(0x20));
        c.sq(3, (c.g[T2][0] as u32).wrapping_add(0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x60));
        c.pc = ra;
        return;
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;
    use crate::difftest::put_vec;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::be::bedbprofile::tests::strict as strict_2;
    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base_2;
    pub(crate) use crate::game::render::particle::tests::f;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_4;

    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2;
    pub(crate) use crate::game::input::inputmap::tests::alloc;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::dummy_world;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn avalanche_compute_instance_fade_matches() {
        check(
            ssx3_ee::s09::f_002d7c00,
            avalanche_compute_instance_fade,
            300,
            &|c, seed| {
                let mut r = Rng(0x1003 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let inst = DATA + 0x400;
                c.bus.write32(DATA + 0x2e0, inst);
                c.bus.write32(inst + 0x100, r.edge_f32());
                c.bus.write32(inst + 0x104, r.edge_f32());
                c.bus.write32(inst + 0x108, r.edge_f32());
                c.bus.write16(
                    inst + 0xf0,
                    if seed % 3 == 0 {
                        2
                    } else {
                        (r.next_u32() & 0xffff) as u16
                    },
                );
                c.f[12] = match seed % 5 {
                    0 => 0,
                    1 => 0x8000_0000,
                    2 => 0x7f7f_ffff,
                    3 => r.edge_f32(),
                    _ => r.f32(1e4),
                };
            },
        );
    }

    /// A node list at DATA + 0x1000, linked at +0xf4, returning the first node
    /// (or 0 for an empty list) and the target node (`node`).
    pub(crate) fn node_list(c: &mut Cpu, r: &mut Rng, ava: u32, found: bool) -> u64 {
        let n = (r.next_u32() % 4) as u32;
        let mut node = DATA + 0x1000;
        c.bus.write32(ava + 4, if n == 0 { 0 } else { node });
        let mut target = DATA + 0x900;
        for i in 0..n {
            if found && i == n - 1 {
                target = node;
            }
            c.bus
                .write32(node + 0xf4, if i + 1 < n { node + 0x10 } else { 0 });
            node += 0x10;
        }
        target as u64
    }

    #[test]
    fn avalanche_add_node_event_matches() {
        check_events(
            ssx3_ee::s09::f_002d9738,
            avalanche_add_node_event,
            80,
            &|c, seed| {
                let mut r = Rng(0x2d97_3800 ^ seed as u64);
                scramble(c, &mut r);
                let ava = DATA;
                c.g[A0][0] = ava as u64;
                c.g[A1][0] = (r.next_u32() & 0xffff) as u64;
                let found = r.next_u32() % 2 == 0;
                let target = node_list(c, &mut r, ava, found);
                c.g[A2][0] = if seed % 7 == 0 {
                    r.next_u32() as u64
                } else {
                    target
                };
                c.bus.write16(
                    ava + 0xe,
                    if seed % 5 == 0 {
                        0x40
                    } else {
                        (r.next_u32() % 0x40) as u16
                    },
                );
                for i in 0..0x40 {
                    c.bus.write16(ava + 0x110 + 2 * i, r.next_u32() as u16);
                }
                let mode = schedule(c, &mut r, seed, 0x002d_9738);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn avalanche_build_active_node_array_matches() {
        check_events(
            ssx3_ee::s09::f_002d7e60,
            avalanche_build_active_node_array,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 24, 0);
                let ava = DATA;
                c.g[A0][0] = ava as u64;
                let n = r.next_u32() % 4;
                let mut node = DATA + 0x1000;
                c.bus.write32(ava + 0x4, if n == 0 { 0 } else { node });
                for i in 0..n {
                    c.bus
                        .write32(node + 0x2e4, if i + 1 < n { node + 0x10 } else { 0 });
                    node += 0x10;
                }
                c.bus.write32(ava + 0x8, r.next_u32());
                let mode = schedule(c, &mut r, seed, 0x002d_7e60);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn avalanche_destroy_all_thunk_matches() {
        check(
            ssx3_ee::s09::f_002f7e70,
            avalanche_destroy_all_thunk,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7e70, 0);
                // Kill the singleton so the callee takes its short path.
                c.bus.write32(GAME_GP.wrapping_add(0x9c0), 0);
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn avalanche_bake_group_matches() {
        check(
            ee::s09::f_002d9660__generated,
            avalanche_bake_group,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2d96_60, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                dummy_world(c, DATA + 0x2000);
                let nodes = (seed % 3) as u32;
                let mut next = 0u32;
                for i in (0..nodes).rev() {
                    let node = DATA + 0x6000 + 0x200 * i;
                    clear(c, node, 0x80);
                    c.bus.write32(node + 0xf4, next);
                    next = node;
                }
                c.bus.write32(this + 4, next);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn avalanche_set_history_data_matches() {
        check(
            ssx3_ee::s09::f_002d9fb8__generated,
            avalanche_set_history_data,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x2d9fb8, 0);
                let data = DATA + 0x1000;
                c.g[A0][0] = data as u64;
                w32(
                    c,
                    data,
                    if seed % 4 == 0 {
                        0x02be_ef00
                    } else {
                        0x1234_5678
                    },
                );
                w32(c, GAME_GP + 0x9c0, 0); // no live history blocks
                                            // Never the (freeing) "old data with buffers" combination.
                w32(
                    c,
                    GAME_GP + 0x9c4,
                    if seed % 2 == 0 { 0 } else { DATA + 0x3000 },
                );
                w32(c, GAME_GP + 0x9c8, 0);
            },
        );
    }

    #[test]
    fn avalanche_release_instance_matches() {
        check(
            ssx3_ee::s09::f_002d7dd8__generated,
            avalanche_release_instance,
            SEEDS_3,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d7dd8, 4);
                be_library(c);
                world(c);
                let av = DATA + 0x100;
                c.g[A0][0] = av as u64;
                c.g[A1][0] = (seed % 2) as u64;
                if seed % 4 == 0 {
                    w(c, av + 0x2e0, 0);
                } else {
                    let first = DATA + 0x300;
                    let live = DATA + 0x400;
                    let vt = DATA + 0x500;
                    w(c, av + 0x2e0, first);
                    w(c, first + 0xf0, 2);
                    w(c, first + 0xf8, live);
                    w(c, live + 0xf0, 0);
                    w(c, live + 0xf8, 0);
                    if seed % 3 == 0 {
                        w(c, live + 0xc, 0);
                    } else {
                        let sub = DATA + 0x600;
                        w(c, live + 0xc, sub);
                        w(c, sub + 0xc, vt);
                        slot(c, vt, 8, 0, FAKE);
                    }
                }
            },
        );
    }

    #[test]
    fn vfx_avalanche_dtor_matches() {
        check(
            ee::s09::f_002d6378__generated,
            vfx_avalanche_dtor,
            SEEDS_4,
            &alloc_base_setup(|c, seed, _r| {
                let free_self = seed & 1;
                let this = if free_self != 0 { alloc(c, 0x80) } else { DATA };
                c.g[A0][0] = this as u64;
                c.g[A1][0] = free_self as u64;
            }),
        );
    }

    #[test]
    fn avalanche_get_object_matrix_matches() {
        check(
            ee::s09::f_002d9c00__generated,
            avalanche_get_object_matrix,
            60,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x2d9c);
                let out = DATA + 0x400;
                let node0 = 0x004e_e770u32;
                let hit = (seed % 3 != 0) as u32;
                c.g[A0][0] = if hit == 1 { (seed % 64) as u64 } else { 0x1234 };
                c.g[A1][0] = sx(out);
                let node = node0 + hit * 0x2f0;
                c.bus.write32(node + 0x2e0, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0xf8, seed as u32 % 64);
                for k in 0..4 {
                    c.bus.write32(node + 0x70 + 4 * k, r.next_u32());
                }
                c.bus.write32(node + 0xb0, f(2.0));
                for k in 0..4 {
                    c.bus.write32(node + 0x60 + 4 * k, r.next_u32());
                }
                fill(c, &mut r, out, 0x40);
            },
        );
    }

    #[test]
    fn avalanche_free_histories_matches() {
        strict(
            ee::s09::f_002d9a80__generated,
            avalanche_free_histories,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d9a, 8);
                let base = DATA + 0x1000;
                for k in 0..16 {
                    c.bus.write32(base + 0x1c * k, (r.next_u32() % 2) as u32);
                }
                c.bus.write32(
                    GAME_GP + 0x9c0,
                    if seed % 3 == 0 { 0 } else { DATA + 0x2000 },
                );
                c.bus
                    .write32(GAME_GP + 0x9c8, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x2000 + 4, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0xf4, 0);
                c.bus.write32(DATA + 0x3000 + 0xfc, 0);
                c.bus.write8(DATA + 0x3000 + 0xf3, 1);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn vfx_avalanche_init_matches() {
        strict_2(
            ee::s09::f_002d6410__generated,
            vfx_avalanche_init,
            4,
            &|c, seed| {
                let _r = base_3(c, seed, 0x2d64);
                for i in 0..0x20 {
                    c.bus.write32(0x004d_5b70 + 0x30 * i, seed + i as u32);
                    c.bus.write32(0x004e_db70 + 0x30 * i, seed + i as u32);
                }
            },
        );
    }

    #[test]
    fn avalanche_clamp_vector_matches() {
        same(
            ssx3_ee::s09::f_002d64d8__generated,
            avalanche_clamp_vector,
            200,
            &|c, seed| {
                let mut r = Rng(0x2d64d8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let v = DATA + 0x100;
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = v as u64;
                c.bus.write32(GAME_GP + 0x9a4, r.f32(100.0));
                let mut q = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                if seed % 3 == 0 {
                    q[(seed as usize) % 4] = 0x7f80_0000;
                }
                put_vec(c, v, q);
            },
        );
    }

    /// A playing avalanche at `DATA` for `Avalanche_UpdateActive`: `live`
    /// says per instance whether it has data (a type-2 instance whose history
    /// index is already at the sample, so `Avalanche_SampleHistory` takes its
    /// straight path and the instance stays alive) or not (it ends at once).
    /// `keys` spawn keys (0.3 s apart, all with an object) and `cues` sound
    /// cues (7 frames apart). The audio system and the world for
    /// `Avalanche_Stop`.
    fn update_active_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        time: f32,
        live: &[bool],
        keys: u32,
        cues: u32,
    ) {
        let this = DATA;
        let def = DATA + 0x1000;
        c.bus.write32(this, def);
        c.bus.write32(this + 0x18, time.to_bits());
        // Instances.
        let mut next = 0;
        for (i, &l) in live.iter().enumerate().rev() {
            let inst = DATA + 0x8000 + 0x400 * i as u32;
            let data = DATA + 0x6000 + 0x400 * i as u32;
            if l {
                c.bus.write32(inst + 0x2e0, data);
                c.bus.write16(data + 0xf0, 2);
                c.bus.write32(data + 0x100, 1000.0f32.to_bits());
                c.bus.write32(data + 0x104, 1.0f32.to_bits());
                c.bus.write32(data + 0x10c, 1.0f32.to_bits());
                c.bus.write32(data + 0xfc, data + 0x200);
                c.bus.write32(inst, 15);
            }
            c.bus.write32(inst + 0x2e4, next);
            next = inst;
        }
        c.bus.write32(this + 4, next);
        let any_live = live.iter().any(|&l| l);
        // Node slots (no array when the avalanche stops: Avalanche_Stop would
        // free it).
        let nodes = if any_live { DATA + 0x3000 } else { 0 };
        c.bus.write32(this + 8, nodes);
        if nodes != 0 {
            for k in 0..4 {
                c.bus.write32(
                    nodes + 4 * k,
                    if k % 2 == 0 {
                        DATA + 0x5000 + 0x100 * k
                    } else {
                        0
                    },
                );
            }
        }
        // Spawn keys.
        c.bus.write16(def + 0xc, keys as u16);
        for k in 0..keys {
            let info = DATA + 0x4000 + 0x100 * k;
            c.bus
                .write32(def + 0x10 + 8 * k, (0.3 * k as f32).to_bits());
            c.bus.write32(def + 0x14 + 8 * k, info);
            c.bus.write32(info + 0xc, 1);
            for w in 0..4 {
                c.bus.write32(info + 0x40 + 4 * w, r.next_u32());
            }
        }
        c.bus.write32(this + 0x10, r.next_u32() % 2);
        // Sound cues.
        c.bus.write16(def + 0xe, cues as u16);
        for k in 0..cues {
            c.bus.write16(def + 0x110 + 4 * k, (7 * k) as u16);
            c.bus.write16(def + 0x112 + 4 * k, (k % 3) as u16);
        }
        c.bus.write32(this + 0x14, r.next_u32() % 2);
        // The audio system (avalanche sound refcount stays above 0) and the
        // world's active-avalanche count.
        let aud = DATA + 0x1_0000;
        c.bus.write32(GAME_GP + 0x410, aud);
        c.bus.write32(aud + 0x6040, 5);
        let wld = DATA + 0x2_0000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), wld);
        c.bus.write32(wld + 0x84, wld + 0x100);
        c.bus.write32(wld + 0x100 + 0xc, wld + 0x200);
        c.bus.write32(wld + 0x200 + 0xa8, wld + 0x300);
        c.bus.write32(wld + 0x300 + 0x708, 3);
    }

    #[test]
    fn avalanche_update_active_matches() {
        check(
            ee::s09::f_002d7ef8__generated,
            avalanche_update_active,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d7ef8, 0);
                elf_image(c);
                let live: Vec<bool> = (0..seed % 3).map(|i| (seed / 3 + i) % 2 == 0).collect();
                let time = if live.iter().any(|&l| l) {
                    0.5
                } else {
                    (seed % 7) as f32 * 0.1
                };
                update_active_fixture(c, &mut r, time, &live, (seed / 2) % 4, (seed / 5) % 5);
                if seed % 13 == 12 {
                    c.bus.write32(DATA, 0);
                }
                c.g[A0][0] = DATA as u64;
                // The generated body dispatches on `pc` (its entry).
                c.pc = 0x002d_7ef8;
            },
        );
    }

    /// The playback time advances per 60 Hz frame: one update at 60 and
    /// `rate/60` updates at 120 / 240 Hz move it by the same amount.
    #[test]
    fn avalanche_update_active_follows_the_rate() {
        fn after(hz: u32) -> f32 {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = machine(c, 3, 0x2d7ef9, 0);
            elf_image(c);
            crate::tick::set_for_test(c, hz);
            update_active_fixture(c, &mut r, 0.5, &[true], 0, 0);
            for _ in 0..hz / 60 {
                c.g[A0][0] = DATA as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.g[SP][0] = crate::difftest::SP as u64;
                avalanche_update_active(c);
                assert_eq!(c.pc, crate::difftest::RA);
                assert_ne!(c.bus.read32(DATA), 0, "{hz} Hz: stopped");
            }
            f32::from_bits(c.bus.read32(DATA + 0x18))
        }
        let base = after(60);
        assert!((base - 0.509).abs() < 1e-6, "{base}");
        for hz in [120, 240] {
            let t = after(hz);
            assert!((t - base).abs() < 1e-6, "{hz} Hz: time {t} vs {base} at 60");
        }
    }

    use crate::difftest::fakes;

    use ssx3_ee_rt::Cpu;

    /// Avalanche_ApplyHistoryData: no groups, no data, already applied;
    /// otherwise 1-3 groups of 0-3 nodes (owning heap data that is freed,
    /// owning none, or not owning) with 0-4 pairs and index pairs each,
    /// read from a history buffer laid out as the function walks it.
    #[test]
    fn avalanche_apply_history_data_matches() {
        use super::avalanche::*;
        use crate::game::scripter::ssxscriptengine::tests::{alloc, heap_init};
        check(
            ee::s09::f_002da028__generated,
            avalanche_apply_history_data,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2da028, 0);
                let groups_at = DATA + 0x1_0000;
                let nodes_at = DATA + 0x2_0000;
                let data = DATA + 0x3_0000;
                let mut heap = false;
                // The history buffer, written as it is read.
                let mut p = data + 8;
                let n_groups = 1 + r.next_u32() % 3;
                for g in 0..n_groups {
                    let group = groups_at + 0x400 * g;
                    for off in (0..0x200).step_by(4) {
                        c.bus.write32(group + off, r.next_u32());
                    }
                    let next = if g + 1 < n_groups { group + 0x400 } else { 0 };
                    c.bus.write32(group + GROUP_NEXT, next);
                    let n_nodes = r.next_u32() % 4;
                    let mut prev_link = group + GROUP_NODES;
                    for k in 0..n_nodes {
                        let node = nodes_at + 0x1000 * g + 0x200 * k;
                        for off in (0xe0..0x110).step_by(4) {
                            c.bus.write32(node + off, r.next_u32());
                        }
                        c.bus.write32(prev_link, node);
                        prev_link = node + NODE_NEXT;
                        let count = 2 * (r.next_u32() % 3);
                        c.bus.write32(node + NODE_COUNT, (count as f32).to_bits());
                        let owned = r.next_u32() % 3;
                        c.bus.write8(node + NODE_OWNED, (owned != 0) as u8);
                        let old = if owned == 1 {
                            if !heap {
                                heap_init(c);
                                heap = true;
                            }
                            alloc(c, 0x40)
                        } else if owned == 2 {
                            0
                        } else {
                            r.next_u32()
                        };
                        c.bus.write32(node + NODE_DATA, old);
                        c.bus.write32(p, r.next_u32());
                        for i in 0..3 {
                            c.bus.write32(p + 4 + 4 * i, r.f32(1000.0));
                        }
                        p += 16;
                        for i in 0..count * 10 / 4 {
                            c.bus.write32(p + 4 * i, r.next_u32());
                        }
                        p += count * 10;
                    }
                    c.bus.write32(prev_link, 0);
                    let pairs = r.next_u32() % 5;
                    let indices = r.next_u32() % 5;
                    c.bus.write16(p, pairs as u16);
                    c.bus.write16(p + 2, indices as u16);
                    p += 4;
                    for i in 0..pairs {
                        c.bus.write32(p + 8 * i, r.f32(10.0));
                        c.bus.write32(p + 8 * i + 4, r.next_u32());
                    }
                    p += 8 * pairs;
                    for i in 0..indices {
                        c.bus.write32(p + 4 * i, r.next_u32());
                    }
                    p += 4 * indices;
                }
                let groups = if seed % 9 == 0 { 0 } else { groups_at };
                let buffer = if seed % 9 == 1 { 0 } else { data };
                let applied = if seed % 9 == 2 { 1 } else { 0 };
                c.bus.write32(GAME_GP + GP_GROUPS, groups);
                c.bus.write32(GAME_GP + GP_DATA, buffer);
                c.bus.write32(GAME_GP + GP_APPLIED, applied);
                c.pc = 0x002d_a028;
            },
        );
    }

    /// `VfxAvalanche_Render`: on/visible/forced combinations (both skip and
    /// draw paths); the manager's three vtable methods are fakes (each may
    /// unwind); the 16 avalanche slots are inactive, so the real
    /// `Avalanche_RenderActive` returns at once.
    #[test]
    fn vfx_avalanche_render_matches() {
        use crate::difftest::GAME_GP;
        check(
            ee::s09::f_002d9130__generated,
            vfx_avalanche_render,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2d9130, 3);
                c.pc = 0x002d_9130;
                let world = DATA;
                let cams = DATA + 0x100;
                let cam = DATA + 0x200;
                let gfx = DATA + 0x1000;
                let top = DATA + 0x3000 + 4 * (seed % 4);
                let vt = DATA + 0x4000;
                let view = DATA + 0x5000;
                let bits = seed % 8;
                c.bus
                    .write32(GAME_GP + 0x940, (bits & 1) * (1 + r.next_u32() % 5));
                c.bus.write32(GAME_GP + 0x9cc, (bits >> 1) & 1);
                c.bus.write32(GAME_GP + 0x10fc, (bits >> 2) & 1);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, cams);
                c.bus.write32(cams + 0x84, cam);
                let cur = r.next_u32() % 3;
                c.bus.write32(cam + 0x14, cur);
                c.bus.write32(cam + 4 + 4 * cur, view);
                fill(c, &mut r, view, 0x20);
                c.bus.write32(GAME_GP + 0x2a90, gfx);
                c.bus.write32(gfx + 0xe84, top);
                fill(c, &mut r, top, 10);
                fill(c, &mut r, 0x0050_1420, 5);
                c.bus.write32(gfx + 0x10d8, vt);
                for (i, slot) in [0xc0u32, 0xc8, 0x110].into_iter().enumerate() {
                    c.bus.write16(vt + slot, (r.next_u32() as i16 & !3) as u16);
                    c.bus.write32(vt + slot + 4, fakes::FAKE + 0x10 * i as u32);
                }
            },
        );
    }

    pub(crate) use crate::game::bx::memblockman::tests::put_gp;

    /// Every combination of the manager's flags except a bake request (the
    /// bake branch, `Avalanche_BakeAll`, reads history files from disc and is
    /// not reached here); no histories loaded
    /// (gp+0x9c0 = 0) and empty active slots, so the real callees take their
    /// short paths.
    #[test]
    fn vfx_avalanche_update_matches() {
        use avg::*;
        check(
            ssx3_ee::s09::f_002d8948__generated,
            vfx_avalanche_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d8948, 0);
                let flag = |r: &mut Rng, k: u32| match (seed / 3u32.pow(k)) % 3 {
                    0 => 0,
                    1 => 1 + r.next_u32() % 0x7fff,
                    _ => 0x8000_0000 | r.next_u32(),
                };
                let enabled = if seed % 7 == 0 { 0 } else { 1 };
                // No bake request: Avalanche_BakeAll always exports and reloads
                // the history files (disc access, unbounded on test data).
                let _ = flag(&mut r, 0);
                let bake = 0;
                let start = flag(&mut r, 1);
                let data = flag(&mut r, 2);
                let update = flag(&mut r, 3);
                put_gp(c, ENABLED, enabled);
                put_gp(c, BAKE_PENDING, bake);
                put_gp(c, START_PENDING, start);
                put_gp(c, UPDATE_PENDING, update);
                put_gp(c, HAVE_DATA, data);
                put_gp(c, BAKE_CLEAR, r.next_u32());
                put_gp(c, START_ID, DATA + (r.next_u32() & 0xfff0));
                put_gp(c, 0x9c0, 0);
                // The BELibrary singleton (gp-0x1f04), so nothing builds one.
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x6_0000);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.pc = 0x002d_8948;
            },
        );
    }

    use crate::difftest::fakes::covered;

    /// `Avalanche_SaveState`: the 16 avalanche slots at 0x5389c0, each idle
    /// (def 0) or pointing at a definition with a random index, with a random
    /// time at +0x18; the stream's `Write` (vtable +8) is a fake.
    #[test]
    fn avalanche_save_state_matches() {
        const SLOTS: u32 = 0x0054_0000 - 0x76c8;
        const STREAM: u32 = DATA + 0x100;
        const STREAM_VT: u32 = DATA + 0x200;
        const DEFS: u32 = DATA + 0x1000;
        covered(
            ee::s09::f_002d9cb0__generated,
            avalanche_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d9cb0, 1);
                c.pc = 0x002d_9cb0;
                for i in 0..16u32 {
                    let slot = SLOTS + 0x1c * i;
                    fill(c, &mut r, slot, 7);
                    let idle = (seed + i) % 3 == 0 || seed % 11 == 0;
                    c.bus.write32(slot, if idle { 0 } else { DEFS + 0x10 * i });
                    c.bus.write32(DEFS + 0x10 * i, r.next_u32() % 40);
                    c.bus.write32(slot + 0x18, r.edge_f32());
                }
                c.bus.write32(STREAM, STREAM_VT);
                c.bus
                    .write16(STREAM_VT + 8, (r.next_u32() % 0x20) as u16 & !3);
                c.bus.write32(STREAM_VT + 0xc, FAKE);
                c.g[A0][0] = sx(STREAM);
            },
        );
    }

    pub(crate) use crate::game::object::movenode::tests::oc383_vtable;

    #[test]
    fn avalanche_render_active_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s09::f_002d8ea8__generated,
            avalanche_render_active,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8ea8, 8);
                c.pc = 0x002d_8ea8;
                let this = DATA;
                let group = DATA + 0x1000;
                fill(c, &mut r, this, 0x10);
                c.bus.write32(this, if seed % 13 == 7 { 0 } else { group });
                c.bus.write32(this + AVALANCHE_SLOT_LIMIT, r.f32(10.0));
                // Group pairs: v at +0x10 + 8j, node at +0x14 + 8j.
                let pairs = (seed / 3) % 4;
                c.bus.write16(group + 0xc, pairs as u16);
                for j in 0..pairs {
                    let v = if seed % 5 == 0 {
                        c.bus.read32(this + AVALANCHE_SLOT_LIMIT)
                    } else {
                        r.f32(10.0)
                    };
                    c.bus.write32(group + 0x10 + 8 * j, v);
                    let node = DATA + 0x7000 + 0x100 * j;
                    c.bus.write32(group + 0x14 + 8 * j, node);
                    fill(c, &mut r, node + 0x40, 4);
                }
                // Active nodes: data (or none), next; emitter `+0xd0` without texture.
                let nodes = seed % 4;
                c.bus.write32(
                    this + AVALANCHE_SLOT_ACTIVE,
                    if nodes == 0 { 0 } else { DATA + 0x2000 },
                );
                for k in 0..nodes {
                    let node = DATA + 0x2000 + 0x400 * k;
                    let data = DATA + 0x8000 + 0x200 * k;
                    let has = r.next_u32() % 4 != 0;
                    c.bus
                        .write32(node + AVALANCHE_NODE_DATA, if has { data } else { 0 });
                    let next = if k + 1 < nodes { node + 0x400 } else { 0 };
                    c.bus.write32(node + AVALANCHE_NODE_NEXT, next);
                    c.bus.write32(node + 0xd0 + 0x174, 0);
                    c.bus.write8(data + 0xf2, (r.next_u32() & 1) as u8);
                    fill(c, &mut r, data + 0xe0, 4);
                    // No line segments in Avalanche_DebugDrawInstance.
                    c.bus.write32(
                        data + 0x100,
                        if r.next_u32() & 1 == 0 {
                            0
                        } else {
                            0xbf80_0000
                        },
                    );
                }
                c.bus
                    .write32(GAME_GP + AVALANCHE_DEBUG_LINES_GP, (seed / 2) % 3);
                c.bus
                    .write32(GAME_GP + AVALANCHE_DEBUG_FX_GP, (seed / 7) % 2);
                // GrMan (both globals), its draw state and vtable.
                let grman = DATA + 0x5000;
                c.bus.write32(GAME_GP + AVALANCHE_GRMAN_GP, grman);
                c.bus.write32(GAME_GP - 0x854, grman);
                c.bus.write32(grman + 0xe84, DATA + 0x6000);
                fill(c, &mut r, DATA + 0x6000, 0x10);
                c.bus.write32(grman + 0x10d8, DATA + 0x5800);
                oc383_vtable(c, &mut r, DATA + 0x5800, &[0x248]);
                // SSXApp->Game->AI->rider + 0x6c0 with its getPos slot.
                c.bus.write32(GAME_GP - 0x848, DATA + 0x9000);
                c.bus.write32(DATA + 0x9000 + 0x84, DATA + 0x9100);
                c.bus.write32(DATA + 0x9100 + 0xc, DATA + 0x9200);
                c.bus.write32(DATA + 0x9200 + 0x28, DATA + 0x9300);
                c.bus.write32(DATA + 0x9300 + 0x6c0, DATA + 0x9c00);
                oc383_vtable(c, &mut r, DATA + 0x9c00, &[0x28]);
            },
        );
    }

    use crate::difftest::fakes::machine_rt;

    use crate::difftest::fresh;
    use crate::difftest::run_generated;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc1043_box_r;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc1043_box_w;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc1043_fake;

    pub(crate) use crate::game::sound::bankmonitor::tests::OC1043_CTRL;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC1043_MAGIC;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this test's fake in the machine's RAM.
    pub(crate) const OC1041_CTRL: u32 = DATA + 0x21_0040;

    pub(crate) const OC1041_MAGIC: u32 = 0x0c10_4141;

    pub(crate) const OC1041_SELF: u32 = DATA + 0x1000;

    pub(crate) const OC1041_OUT: u32 = DATA + 0x2000;

    pub(crate) const OC1041_DATA: u32 = DATA + 0x3000;

    pub(crate) const OC1041_FRAMES: u32 = DATA + 0x4000;

    pub(crate) static OC1041_PREV_SPAWN_AT: AtomicUsize = AtomicUsize::new(0);

    /// `DynamicEmitter_SpawnAt` stand-in: logs every register the caller set
    /// up and returns (or unwinds), unless the gate word is off.
    pub(crate) fn oc1041_fake_spawn_at(c: &mut Cpu) {
        if c.bus.read32(OC1041_CTRL) != OC1041_MAGIC {
            return fakes::fall_back(c, &OC1041_PREV_SPAWN_AT, ee::s12::f_003717c0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0037_17c0, ra);
    }

    /// A packed history of 3..8 frames, a node at some frame and a sample
    /// time that is the same frame, frame 0, a step back, a step forward or
    /// past the end; emitting or not; zero and non-zero rotation axes.
    pub(crate) fn oc1041_setup(c: &mut Cpu, seed: u32) -> Events {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_003717C0),
            oc1041_fake_spawn_at,
            &OC1041_PREV_SPAWN_AT,
        );
        let mut r = machine(c, seed, 0x2d5778, 4);
        elf_image(c);
        c.bus.write32(OC1041_CTRL, OC1041_MAGIC);
        // cAvalanche_m_UpdateTimeStep = 1/60, PatchColourScale2 = 1.
        c.bus.write32(GAME_GP.wrapping_add(0x974), 0x3c88_8889);
        c.bus.write32(GAME_GP.wrapping_add(0x9b8), 0x3f80_0000);

        let len = 3 + seed % 6;
        let len_f = if seed % 5 == 4 {
            len as f32 + 0.5
        } else {
            len as f32
        };
        // The history.
        c.bus.write32(OC1041_DATA + AVSH_D_LENGTH, len_f.to_bits());
        let life = match seed % 4 {
            0 => 1000.0f32,
            1 => 0.1,
            2 => len as f32,
            _ => f32::from_bits(r.f32(20.0)).abs(),
        };
        c.bus.write32(OC1041_DATA + 0x104, life.to_bits());
        let flat = r.f32(2.0);
        c.bus.write32(OC1041_DATA + 0x108, flat);
        let mode = if seed % 3 == 0 {
            2
        } else {
            (r.next_u32() & 0xffff) as u16
        };
        c.bus.write16(OC1041_DATA + 0xf0, mode);
        let rate = match seed % 5 {
            3 => 4.0f32,
            1 => 0.25,
            _ => 1.0,
        };
        c.bus.write32(OC1041_DATA + AVSH_D_RATE, rate.to_bits());
        put_vec(
            c,
            OC1041_DATA + AVSH_D_START,
            [r.f32(5.0), r.f32(5.0), r.f32(5.0), r.f32(5.0)],
        );
        c.bus.write32(OC1041_DATA + AVSH_D_FRAMES, OC1041_FRAMES);
        fill(c, &mut r, OC1041_FRAMES, 0x40);
        // Frame records are 10 bytes: vel 0..2, colour 3..5, axis 6..8, angle 9.
        for i in 0..len + 3 {
            let rec = OC1041_FRAMES + 10 * i;
            if seed % 4 == 0 || (seed % 7 == 0 && i % 2 == 0) {
                for k in 6..9 {
                    c.bus.write8(rec + k, 0);
                }
            } else if r.next_u32() % 5 == 0 {
                c.bus.write8(rec + 6, 0x40);
            }
        }

        // The node.
        let target = (seed / 2) % (len + 2);
        // The frame the function samples (clamped to the last pair).
        let target_eff = if target + 1 >= len { len - 2 } else { target };
        let frame_now = match (seed / 3) % 5 {
            0 => target_eff,
            1 => 0,
            2 => target_eff + 1,
            3 => target_eff.saturating_sub(1),
            _ => r.next_u32() % (len + 1),
        };
        c.bus.write32(OC1041_SELF + AVSH_FRAME, frame_now);
        c.bus
            .write32(OC1041_SELF + AVSH_EMITTING, (seed % 3 != 1) as u32);
        put_vec(
            c,
            OC1041_SELF + AVSH_POS,
            [r.f32(9.0), r.f32(9.0), r.f32(9.0), r.f32(9.0)],
        );
        for i in 0..16 {
            c.bus.write32(OC1041_SELF + AVSH_MAT + 4 * i, r.f32(4.0));
        }
        c.bus.write32(OC1041_SELF + AVSH_FADE, r.f32(1.0));
        c.bus.write32(OC1041_SELF + AVSH_DATA, OC1041_DATA);
        for i in 0..4 {
            c.bus.write32(OC1041_OUT + 4 * i, 0xdead_0000 + i);
        }

        let frac = [0.0f32, 0.25, 0.75][(seed % 3) as usize];
        c.g[A0][0] = OC1041_OUT as u64;
        c.g[A1][0] = OC1041_SELF as u64;
        let mode = schedule(c, &mut r, seed, 0x002d_5778);
        c.f[12] = ((target as f32 + frac) / 30.0).to_bits();
        if mode != Events::Off {
            c.next_event = c.cycles;
        }
        mode
    }

    /// `Avalanche_SampleHistory` against the generated body: same registers,
    /// cycles, memory and `DynamicEmitter_SpawnAt` call logs (the call and
    /// its unwind through the gated fake), with event delivery in both
    /// replay loops.
    #[test]
    fn avalanche_sample_history_matches() {
        check_events(
            ssx3_ee::s09::f_002d5778__generated,
            avalanche_sample_history,
            240,
            &oc1041_setup,
        );
    }

    /// The fixture reaches the reset, the replay and the emit paths.
    #[test]
    fn avalanche_sample_history_paths() {
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..240 {
            let mut c = fresh();
            oc1041_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ssx3_ee::s09::f_002d5778__generated);
            totals.insert(c.cycles - before);
        }
        assert!(totals.len() >= 12, "only {} distinct paths", totals.len());
    }

    /// Gate word of this module's fakes (own address and magic in the
    /// machine's RAM: other tests' machines run the real bodies), then the
    /// word of random bits that steers the faked collision answers.
    pub(crate) const OC1042_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC1042_MAGIC: u32 = 0x1042_f00d;

    pub(crate) const OC1042_STEER: u32 = OC1042_CTRL + 4;

    /// Fixture objects, clear of the fake log (`DATA+0x100000`) and of the
    /// pointers the fakes return (`DATA..DATA+0x40000`).
    pub(crate) const OC1042_NODE: u32 = DATA + 0x5_0000;

    pub(crate) const OC1042_SRC: u32 = DATA + 0x5_1000;

    pub(crate) const OC1042_TRI: u32 = DATA + 0x5_2000;

    pub(crate) const OC1042_TBL: u32 = DATA + 0x5_3000;

    pub(crate) const OC1042_BASE: u32 = DATA + 0x5_3100;

    pub(crate) const OC1042_AV: u32 = DATA + 0x5_4000;

    pub(crate) const OC1042_OBJ: u32 = DATA + 0x5_6000;

    pub(crate) const OC1042_WORLD: u32 = DATA + 0x5_7000;

    pub(crate) const OC1042_WORLD2: u32 = DATA + 0x5_7100;

    pub(crate) const OC1042_CTX: u32 = DATA + 0x5_7200;

    /// Every faked callee: a gated fake in its override slot that logs the
    /// call (all registers) and returns a pointer; `oc1042_after` then
    /// steers the collision answers.
    macro_rules! oc1042_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1042_CTRL) != OC1042_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                    oc1042_after(c, $addr);
                }
            )*
            /// Puts every fake in its override slot (once per process).
            fn oc1042_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    /// Bit `k` of the steering word, rotated by the call index.
    pub(crate) fn oc1042_bit(c: &mut Cpu, k: u32) -> bool {
        let n = c.bus.read32(fakes::LOG_COUNT).wrapping_sub(1);
        c.bus.read32(OC1042_STEER) >> ((n * 7 + k) % 32) & 1 != 0
    }

    /// The faked answers: the first sweep test (`Nash_FindClosestObjectContact`)
    /// finds a contact or not, the second (`0x336850`) returns a signed
    /// distance, `0x350698` accepts the object or not.
    pub(crate) fn oc1042_after(c: &mut Cpu, target: u32) {
        match target {
            0x0033_78c0 => {
                if !oc1042_bit(c, 0) {
                    c.g[V0][0] = 0;
                }
            }
            0x0033_6850 => {
                c.f[0] = if oc1042_bit(c, 1) {
                    1.5f32.to_bits()
                } else {
                    (-1.0f32).to_bits()
                };
            }
            0x0035_0698 => c.g[V0][0] = oc1042_bit(c, 2) as u64,
            _ => {}
        }
    }

    oc1042_fakes! {
        oc1042_fake_00317e30, 0x0031_7e30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated;
        oc1042_fake_003177f0, 0x0031_77f0, ee::s10::OVERRIDE_003177F0, ee::s10::f_003177f0__generated;
        oc1042_fake_0032fb48, 0x0032_fb48, ee::s11::OVERRIDE_0032FB48, ee::s11::f_0032fb48__generated;
        oc1042_fake_003378c0, 0x0033_78c0, ee::s11::OVERRIDE_003378C0, ee::s11::f_003378c0__generated;
        oc1042_fake_00350698, 0x0035_0698, ee::s11::OVERRIDE_00350698, ee::s11::f_00350698__generated;
        oc1042_fake_00336850, 0x0033_6850, ee::s11::OVERRIDE_00336850, ee::s11::f_00336850__generated;
        oc1042_fake_00418ef8, 0x0041_8ef8, ee::s15::OVERRIDE_00418EF8, ee::s15::f_00418ef8__generated;
        oc1042_fake_00416210, 0x0041_6210, ee::s15::OVERRIDE_00416210, ee::s15::f_00416210__generated;
        oc1042_fake_002d64d8, 0x002d_64d8, ee::s09::OVERRIDE_002D64D8, ee::s09::f_002d64d8__generated;
        oc1042_fake_002d9738, 0x002d_9738, ee::s09::OVERRIDE_002D9738, ee::s09::f_002d9738__generated;
        oc1042_fake_002edb20, 0x002e_db20, ee::s09::OVERRIDE_002EDB20, ee::s09::f_002edb20__generated;
        oc1042_fake_002d55c0, 0x002d_55c0, ee::s09::OVERRIDE_002D55C0, ee::s09::f_002d55c0__generated;
        oc1042_fake_002d5658, 0x002d_5658, ee::s09::OVERRIDE_002D5658, ee::s09::f_002d5658__generated;
        oc1042_fake_002d5598, 0x002d_5598, ee::s09::OVERRIDE_002D5598, ee::s09::f_002d5598__generated;
    }

    /// Hooked fakes, the game image (constants, `D_004FF120` vectors), the
    /// node and the avalanche table, a random hit record on the stack.
    /// `calls`: the call index at which a fake may unwind.
    pub(crate) fn oc1042_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        oc1042_hook();
        let mut r = fakes::machine(c, seed, 0x1042, calls);
        elf_image(c);
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        w(c, OC1042_CTRL, OC1042_MAGIC);
        w(c, OC1042_STEER, r.next_u32());
        // World -> +0x84 -> +0x20: the collision context.
        w(c, GAME_GP.wrapping_sub(0x848), OC1042_WORLD);
        w(c, OC1042_WORLD + 0x84, OC1042_WORLD2);
        w(c, OC1042_WORLD2 + 0x20, OC1042_CTX);
        // Sub-steps per frame and the "bounce on contact" flag.
        w(c, GAME_GP + 0x9d8, 1 + seed % 3);
        w(c, GAME_GP + 0x9dc, (seed / 3) % 2);
        // The node: source (a vec4 at +0x40), length, fade, radius, type.
        for i in 0..4 {
            w(c, OC1042_SRC + 0x40 + 4 * i, r.f32(500.0));
        }
        w(c, OC1042_NODE + 0xf0, (seed / 2) % 2);
        w(c, OC1042_NODE + 0xf8, OC1042_SRC);
        let length = [1.0f32, 3.0, 4.0, 2.5, 5.0][(seed % 5) as usize];
        let fade = [1.0f32, 2.0, 3.0][((seed / 5) % 3) as usize];
        let radius = [100.0f32, 30.0, 400.0, 200.0][((seed / 7) % 4) as usize];
        w(c, OC1042_NODE + 0x100, length.to_bits());
        w(c, OC1042_NODE + 0x108, fade.to_bits());
        w(c, OC1042_NODE + 0x110, radius.to_bits());
        // The avalanche table: object count (< 32 or full), event count, rows.
        let count = [0u32, 1, 3, 31, 32][((seed / 4) % 5) as usize];
        c.bus.write16(OC1042_AV + 0xc, count as u16);
        c.bus.write16(
            OC1042_AV + 0xe,
            [0u16, 5, 63, 64][((seed / 9) % 4) as usize],
        );
        let obj = if seed % 6 == 0 {
            0
        } else {
            OC1042_OBJ + 0x40 * (seed % 3)
        };
        for k in 0..count.min(32) {
            // Row k holds the hit object when `seed / 3` is even and k == 0.
            let o = if k == 0 && (seed / 3) % 2 == 0 {
                obj
            } else {
                OC1042_OBJ + 0x400 + 4 * k
            };
            w(c, OC1042_AV + 0x10 + 8 * k, r.next_u32());
            w(c, OC1042_AV + 0x14 + 8 * k, o);
        }
        // The surface test of the hit triangle: a table entry of kind 6 and
        // a record of kind 3 (some seeds fail one of them).
        w(c, GAME_GP + 0x16c8, OC1042_TBL);
        w(c, OC1042_TBL, OC1042_BASE);
        let tri_ok = seed % 4 != 1;
        c.bus.write8(OC1042_TRI + 0x155, 1);
        c.bus
            .write16(OC1042_TRI + 0x156, if seed % 7 == 3 { 0xffff } else { 2 });
        w(c, OC1042_BASE + 8 + 0x24, if seed % 5 == 2 { 4 } else { 6 });
        w(
            c,
            OC1042_BASE + 0x30 + 0x3f0,
            if seed % 3 == 2 { 1 } else { 3 },
        );
        // The stack frame below the call: a random hit record (palette of
        // zero, -1, small floats and random bits), then the pieces the
        // faked callees would have filled.
        let sp0 = (c.g[SP][0] as u32).wrapping_sub(0x3e0);
        for i in 0..0xf8u32 {
            let v = match r.next_u32() % 12 {
                0 => 0,
                1 => 1,
                2 => 0xffff_ffff,
                3 => 1.0f32.to_bits(),
                4 => (-1.0f32).to_bits(),
                5 => 0.5f32.to_bits(),
                6 => r.f32(2.0),
                7 => r.f32(300.0),
                _ => r.next_u32(),
            };
            w(c, sp0 + 4 * i, v);
        }
        w(c, sp0 + 0xf0, obj);
        w(c, sp0 + 0xf4, if tri_ok { OC1042_TRI } else { 0 });
        c.g[A0][0] = sx(OC1042_NODE);
        c.g[A1][0] = sx(OC1042_AV);
        r
    }

    /// `Avalanche_BakeNodeHistory`: one to five frames of one to three
    /// sub-steps; contact or not, object already in the table or not (and the
    /// table full), bounce flag, both node types, surface record valid or not,
    /// fakes that unwind after any call.
    #[test]
    fn avalanche_bake_node_history_matches() {
        covered(
            ee::s09::f_002d66a0__generated,
            avalanche_bake_node_history,
            300,
            &|c, seed| {
                oc1042_setup(c, seed, 60);
            },
        );
    }

    /// The call arms the seeds reach (return addresses of logged calls).
    #[test]
    fn avalanche_bake_node_history_reaches_every_call() {
        fakes::check_reaching(
            ee::s09::f_002d66a0__generated,
            avalanche_bake_node_history,
            300,
            &[
                0x002d_675c,
                0x002d_6778,
                0x002d_67dc,
                0x002d_67f8,
                0x002d_6968,
                0x002d_6988,
                0x002d_699c,
                0x002d_69d4,
                0x002d_6cac,
                0x002d_6e3c,
                0x002d_6e50,
                0x002d_6ebc,
                0x002d_6ed0,
                0x002d_7184,
                0x002d_720c,
                0x002d_74d4,
                0x002d_7680,
                0x002d_770c,
                0x002d_7904,
                0x002d_7938,
                0x002d_7988,
                0x002d_7a78,
                0x002d_7b94,
                0x002d_7bac,
            ],
            &|c, seed| {
                oc1042_setup(c, seed, 0);
            },
        );
    }

    /// The seeds make many different paths (distinct cycle totals).
    #[test]
    fn avalanche_bake_node_history_path_count() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..300 {
            let mut c = crate::difftest::fresh();
            oc1042_setup(&mut c, seed, 0);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s09::f_002d66a0__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 40, "only {} distinct paths", seen.len());
    }

    /// Events delivered or unwinding at the loop back edges.
    #[test]
    fn avalanche_bake_node_history_events() {
        check_events(
            ee::s09::f_002d66a0__generated,
            avalanche_bake_node_history,
            120,
            &|c, seed| {
                let mut r = oc1042_setup(c, seed, 0);
                schedule(c, &mut r, seed, 0x002d_66a0)
            },
        );
    }

    // ----------------------------------------------------------------------
    // Avalanche_WriteHistoryFile.

    oc1043_fake!(
        oc1043_fake_av_get_be,
        OC1043_PREV_AV_GET_BE,
        ee::s01::f_0014dc80__generated,
        |_c, _a| {}
    );

    oc1043_fake!(
        oc1043_fake_av_get_interface,
        OC1043_PREV_AV_GET_INTERFACE,
        ee::s01::f_0014dd58__generated,
        |_c, _a| {}
    );

    oc1043_fake!(
        oc1043_fake_av_location_id,
        OC1043_PREV_AV_LOCATION_ID,
        ee::s01::f_00144be0__generated,
        |_c, _a| {}
    );

    oc1043_fake!(
        oc1043_fake_av_location_file,
        OC1043_PREV_AV_LOCATION_FILE,
        ee::s01::f_00144c48__generated,
        |_c, _a| {}
    );

    oc1043_fake!(
        oc1043_fake_av_sprintf,
        OC1043_PREV_AV_SPRINTF,
        ee::s15::f_00417828__generated,
        |_c, _a| {}
    );

    // 0x3debf0 opensync: success flag in v0, the handle through a3.
    oc1043_fake!(
        oc1043_fake_av_open,
        OC1043_PREV_AV_OPEN,
        ee::s14::f_003debf0__generated,
        |c, a| {
            let ok = oc1043_box_r(c, 0);
            let fd = oc1043_box_r(c, 1);
            c.bus.write32(a[3], fd);
            c.g[V0][0] = ok as u64;
        }
    );

    oc1043_fake!(
        oc1043_fake_av_close,
        OC1043_PREV_AV_CLOSE,
        ee::s14::f_003deca0__generated,
        |_c, _a| {}
    );

    // 0x2d82a0 writeData: returns the bytes it wrote (size * count).
    oc1043_fake!(
        oc1043_fake_av_write,
        OC1043_PREV_AV_WRITE,
        ee::s09::f_002d82a0__generated,
        |c, a| {
            c.g[V0][0] = (a[4].wrapping_mul(a[5])) as u64;
        }
    );

    /// The avalanche history file: no list, a failing open, and 1-2
    /// avalanches with 0-2 nodes (life from 0 to 10, negative too) and 0-3
    /// collisions and ground collisions each.
    #[test]
    fn avalanche_write_history_file_matches() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc1043_fake_av_get_be,
            &OC1043_PREV_AV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc1043_fake_av_get_interface,
            &OC1043_PREV_AV_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BE0),
            oc1043_fake_av_location_id,
            &OC1043_PREV_AV_LOCATION_ID,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144C48),
            oc1043_fake_av_location_file,
            &OC1043_PREV_AV_LOCATION_FILE,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc1043_fake_av_sprintf,
            &OC1043_PREV_AV_SPRINTF,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DEBF0),
            oc1043_fake_av_open,
            &OC1043_PREV_AV_OPEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DECA0),
            oc1043_fake_av_close,
            &OC1043_PREV_AV_CLOSE,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D82A0),
            oc1043_fake_av_write,
            &OC1043_PREV_AV_WRITE,
        );
        covered(
            ee::s09::f_002d83b8__generated,
            avalanche_write_history_file,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d_83b8, 12);
                c.bus.write32(OC1043_CTRL, OC1043_MAGIC);
                oc1043_box_w(c, 0, (seed % 6 != 0) as u32);
                oc1043_box_w(c, 1, r.next_u32());
                c.g[A0][0] = (seed % 2) as u64;
                c.g[A1][0] = (DATA + 0x2e_f000) as u64;
                let base = DATA + 0x31_0000;
                let n_av = 1 + r.next_u32() % 2;
                c.bus
                    .write32(GAME_GP + 0x9c0, if seed % 8 == 0 { 0 } else { base });
                let lives = [0u32, 0x40a0_0000, 0x4120_0000, 0xc000_0000, 0x3f80_0000];
                let mut node_at = DATA + 0x33_0000;
                for i in 0..n_av {
                    let av = base + 0x1000 * i;
                    c.bus
                        .write32(av + 8, if i + 1 < n_av { av + 0x1000 } else { 0 });
                    // nodes
                    let nn = r.next_u32() % 3;
                    let mut head = 0;
                    for _ in 0..nn {
                        let node = node_at;
                        node_at += 0x200;
                        c.bus.write32(node + 0xf4, head);
                        c.bus.write32(node + 0xfc, DATA + 0x34_0000);
                        c.bus
                            .write32(node + 0x100, lives[(r.next_u32() % 5) as usize]);
                        fill(c, &mut r, node + 0xe0, 3);
                        head = node;
                    }
                    c.bus.write32(av + 4, head);
                    let ncoll = r.next_u32() % 4;
                    let ngr = r.next_u32() % 4;
                    c.bus.write16(av + 0xc, ncoll as u16);
                    c.bus.write16(av + 0xe, ngr as u16);
                    for k in 0..ncoll {
                        c.bus.write32(av + 0x10 + 8 * k, r.next_u32());
                        c.bus
                            .write32(av + 0x14 + 8 * k, DATA + 0x32_0000 + 0x100 * k);
                    }
                    for k in 0..ngr {
                        c.bus.write16(av + 0x110 + 4 * k, r.next_u32() as u16);
                        c.bus.write16(av + 0x112 + 4 * k, r.next_u32() as u16);
                    }
                }
                for k in 0..4 {
                    c.bus
                        .write32(DATA + 0x32_0000 + 0x100 * k + 0x78, r.next_u32());
                }
            },
        );
    }

    /// Write a random float (edge cases or a plain range) at `a`.
    pub(crate) fn oc1044_rand_f32(c: &mut Cpu, r: &mut Rng, a: u32) {
        let v = if r.next_u32() % 5 == 0 {
            r.edge_f32()
        } else {
            r.f32(100.0)
        };
        c.bus.write32(a, v);
    }

    /// Debug-draw fixture: world with a render-state stack, a node whose
    /// instance holds `count` (0..3) line records, the vtable draw method a
    /// fake. Seeds also reach: no instance, debug flag off, count <= 0,
    /// zero-length axes (bytes 6..8 all 0).
    pub(crate) fn oc1044_debug_draw_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0c10_4401, 1);
        elf_image(c);
        let node = DATA + 0x1000;
        let obj = DATA + 0x2000;
        let recs = DATA + 0x2800;
        let world = DATA + 0x3000;
        let stack = DATA + 0x3400;
        // The draw object: halfword `this` delta at +0x248, method at +0x24c.
        let draw = DATA + 0x3800;
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(draw + 0x248, delta as u16);
        c.bus.write32(draw + 0x24c, fakes::FAKE);
        c.g[A0][0] = sx(node);
        c.bus
            .write32(node + AVNODE_OBJ, if seed % 8 == 0 { 0 } else { obj });
        let gp = c.g[GP][0] as u32;
        c.bus.write32(gp.wrapping_sub(AVRS_GLOBAL), world);
        c.bus.write32(world + 0xe84, stack);
        c.bus.write32(world + 0x10d8, draw);
        for k in 0..0x30 / 4 {
            let v = r.next_u32();
            c.bus.write32(stack + 4 * k, v);
        }
        c.bus.write32(
            gp + AVDEBUG_FLAG,
            if seed % 8 == 1 { 0 } else { r.next_u32() | 1 },
        );
        let count: i32 = match seed % 5 {
            0 => 0,
            1 => -1,
            n => n as i32 - 1, // 1, 2, 3
        };
        c.bus.write32(obj + AVOBJ_COUNT, (count as f32).to_bits());
        c.bus.write32(obj + AVOBJ_RECS, recs);
        for k in 0..4 {
            let v = r.next_u32();
            c.bus.write32(obj + AVOBJ_POS + 4 * k, v);
        }
        for k in 0..8 {
            let mut v = r.next_u32();
            if seed % 3 == 0 && k >= 1 {
                v &= 0x0000_ffff; // zero some bytes (zero-length axes)
            }
            c.bus.write32(recs + 4 * k, v);
        }
        // 1/127 and 2*pi/256-ish constants at gp-0x3cd0 / gp-0x3ccc.
        c.bus
            .write32(gp.wrapping_sub(0x3cd0), 0.007_874_016_f32.to_bits());
        c.bus
            .write32(gp.wrapping_sub(0x3ccc), 0.008_213_315_f32.to_bits());
        // the 0x4ff1a0 matrix and the 0x4ff130 row
        for k in 0..16 {
            let a = 0x4ff1a0 + 4 * k;
            oc1044_rand_f32(c, &mut r, a);
        }
        for k in 0..4 {
            oc1044_rand_f32(c, &mut r, 0x4ff130 + 4 * k);
        }
    }

    /// `Avalanche_DebugDrawInstance` against the generated body.
    #[test]
    fn avalanche_debug_draw_instance_matches() {
        fakes::covered(
            ee::s09::f_002d8a00__generated,
            avalanche_debug_draw_instance,
            200,
            &oc1044_debug_draw_setup,
        );
    }

    /// `Avalanche_AddNode`: node found first / second, absent, empty list;
    /// all `kind` values; every ordering of the instance's extents.
    #[test]
    fn avalanche_add_node_matches() {
        check(
            ee::s09::f_002d92d0__generated,
            avalanche_add_node,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0c10_4402, 0);
                elf_image(c);
                let gp = c.g[GP][0] as u32;
                let id = 0x40 + seed % 5;
                let inst = DATA + 0x1000;
                let n1 = DATA + 0x2000;
                let n2 = DATA + 0x2100;
                c.g[A0][0] = sx(id);
                c.g[A1][0] = sx(inst);
                c.g[A2][0] = [2u64, 5, 1, 3][(seed / 3 % 4) as usize];
                for f in 12..16 {
                    c.f[f] = if r.next_u32() % 4 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(50.0)
                    };
                }
                for k in 0..8 {
                    oc1044_rand_f32(c, &mut r, inst + 0x60 + 4 * k);
                }
                c.bus.write32(inst + 0x90, r.next_u32());
                match seed % 4 {
                    0 => c.bus.write32(gp + AVALANCHE_NODE_LIST, 0),
                    1 => {
                        // first node matches
                        c.bus.write32(gp + AVALANCHE_NODE_LIST, n1);
                        c.bus.write32(n1, id);
                        c.bus.write32(n1 + 8, n2);
                        c.bus.write32(n1 + 4, DATA + 0x2200);
                    }
                    2 => {
                        // second node matches
                        c.bus.write32(gp + AVALANCHE_NODE_LIST, n1);
                        c.bus.write32(n1, id + 1);
                        c.bus.write32(n1 + 8, n2);
                        c.bus.write32(n2, id);
                        c.bus.write32(n2 + 8, 0);
                    }
                    _ => {
                        // no match: allocate a new node
                        c.bus.write32(gp + AVALANCHE_NODE_LIST, n1);
                        c.bus.write32(n1, id + 1);
                        c.bus.write32(n1 + 8, 0);
                    }
                }
                c.bus.write32(gp + AVALANCHE_REC_COUNT, seed % 40);
                c.bus.write32(gp + AVALANCHE_DIRTY, r.next_u32());
                for off in [0x3cc8u32, 0x3cc4, 0x3cc0] {
                    oc1044_rand_f32(c, &mut r, gp.wrapping_sub(off));
                }
            },
        );
    }

    /// The world, the audio system and one active node (`DATA + 0x100`) with
    /// up to two instances on its list, for `Avalanche_Stop`.
    pub(crate) fn stop_fixture(c: &mut Cpu, seed: u32) {
        let aud = DATA + 0x1_0000;
        c.bus.write32(GAME_GP + 0x410, aud);
        c.bus.write32(aud + 0x6040, 5);
        let wld = DATA + 0x2_0000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), wld);
        c.bus.write32(wld + 0x84, wld + 0x100);
        c.bus.write32(wld + 0x100 + 0xc, wld + 0x200);
        c.bus.write32(wld + 0x200 + 0xa8, wld + 0x300);
        c.bus.write32(wld + 0x300 + 0x708, 3);
        let this = DATA + 0x100;
        c.g[A0][0] = this as u64;
        c.bus.write32(this, if seed % 4 == 0 { 0 } else { 1 });
        c.bus.write32(this + 8, 0);
        let first = DATA + 0x400;
        let second = DATA + 0x500;
        let count = seed % 3;
        c.bus.write32(this + 4, if count == 0 { 0 } else { first });
        if count >= 1 {
            // Dead instances (no data): ReleaseInstance returns at once.
            c.bus.write32(first + 0x2e0, 0);
            c.bus
                .write32(first + 0x2e4, if count == 2 { second } else { 0 });
        }
        if count == 2 {
            c.bus.write32(second + 0x2e0, 0);
            c.bus.write32(second + 0x2e4, 0);
        }
    }

    #[test]
    fn active_avalanche_end_matches() {
        check(
            ee::s09::f_002d81b0__generated,
            active_avalanche_end,
            120,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x2d81b0, 3);
                elf_image(c);
                stop_fixture(c, seed);
            },
        );
    }

    #[test]
    fn avalanche_export_history_files_matches() {
        check(
            ee::s09::f_002d8258__generated,
            avalanche_export_history_files,
            40,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x2d8258, 2);
                elf_image(c);
            },
        );
    }

    #[test]
    fn avalanche_load_history_file_matches() {
        check(
            ee::s09::f_002d87d0__generated,
            avalanche_load_history_file,
            60,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x2d87d0, 2);
                elf_image(c);
                c.bus.write32(GAME_GP + 0x9c4, 0);
                c.bus.write32(GAME_GP + 0x9c8, seed % 2);
            },
        );
    }

    /// Avalanche records (0x400 bytes each), their nodes (0x200 bytes, 4 per
    /// avalanche), the nodes' instances and the danger-manager chain.
    pub(crate) const OC1063_AV: u32 = DATA + 0x5_0000;

    pub(crate) const OC1063_AV_NODES: u32 = DATA + 0x6_0000;

    pub(crate) const OC1063_INST: u32 = DATA + 0x7_0000;

    pub(crate) const OC1063_DANGER: u32 = DATA + 0x8_0000;

    /// A harmless node-sized block that "occupied" active nodes point at.
    pub(crate) const OC1063_DUMMY: u32 = DATA + 0x9_0000;

    pub(crate) const OC1063_IDS: [u32; 3] = [10, 20, 30];

    /// An avalanche list of `n_av` avalanches (ids 10, 20, 30) at `gp+0x9c0`,
    /// each with up to `max_nodes` nodes (emitter flag at +0xf2 random when
    /// `emitters`), 0-5 collisions (ascending times) and 0-5 ground collisions,
    /// and the danger-manager chain reached through the world at `gp-0x848`.
    pub(crate) fn oc1063_world(
        c: &mut Cpu,
        r: &mut Rng,
        n_av: u32,
        max_nodes: u32,
        emitters: bool,
    ) {
        for a in 0..n_av {
            let av = OC1063_AV + 0x400 * a;
            c.bus.write32(av, OC1063_IDS[a as usize]);
            c.bus
                .write32(av + 8, if a + 1 < n_av { av + 0x400 } else { 0 });
            let n_nodes = r.next_u32() % (max_nodes + 1);
            c.bus.write32(
                av + 4,
                if n_nodes == 0 {
                    0
                } else {
                    OC1063_AV_NODES + 0x1000 * a
                },
            );
            for k in 0..n_nodes {
                let node = OC1063_AV_NODES + 0x1000 * a + 0x200 * k;
                for off in (0..0x120).step_by(4) {
                    c.bus.write32(node + off, r.f32(100.0));
                }
                c.bus.write16(node + 0xf0, (r.next_u32() % 4) as u16);
                let emitter = emitters && r.next_u32() % 2 == 0;
                c.bus.write8(node + 0xf2, emitter as u8);
                let next = if k + 1 < n_nodes { node + 0x200 } else { 0 };
                c.bus.write32(node + 0xf4, next);
                let inst = OC1063_INST + 0x100 * (4 * a + k);
                c.bus.write32(node + 0xf8, inst);
                c.bus.write32(node + 0x114, 0);
                for off in (0..0x80).step_by(4) {
                    c.bus.write32(inst + off, r.f32(100.0));
                }
            }
            // Collisions: {time f32, 0} at +0x10; ground: {u16 node, u16 time} at +0x110.
            let n_coll = r.next_u32() % 6;
            let n_ground = r.next_u32() % 6;
            c.bus.write16(av + 0xc, n_coll as u16);
            c.bus.write16(av + 0xe, n_ground as u16);
            let mut t = 0.0f32;
            for i in 0..n_coll {
                t += (r.next_u32() % 20) as f32 * 0.25;
                c.bus.write32(av + 0x10 + 8 * i, t.to_bits());
            }
            let mut t = 0u32;
            for i in 0..n_ground {
                t += r.next_u32() % 6;
                c.bus.write16(av + 0x110 + 4 * i, r.next_u32() as u16);
                c.bus.write16(av + 0x112 + 4 * i, t as u16);
            }
        }
        let gp = GAME_GP;
        c.bus
            .write32(gp + AVS_GP_LIST, if n_av == 0 { 0 } else { OC1063_AV });
        // Loaded history data that is already applied (ApplyHistoryData returns at once).
        let loaded = r.next_u32() % 2 == 0;
        c.bus
            .write32(gp + 0x9c4, if loaded { OC1063_DUMMY } else { 0 });
        c.bus.write32(gp + 0x9cc, loaded as u32);
        // gp-0x848: the world; +0x84 -> +0xc -> +0xa8 -> the danger manager (+0x708: active count).
        c.bus.write32(gp.wrapping_sub(0x848), OC1063_DANGER);
        c.bus.write32(OC1063_DANGER + 0x84, OC1063_DANGER + 0x100);
        c.bus.write32(OC1063_DANGER + 0x10c, OC1063_DANGER + 0x200);
        c.bus.write32(OC1063_DANGER + 0x2a8, OC1063_DANGER + 0x300);
        c.bus.write32(OC1063_DANGER + 0xa08, r.next_u32() % 4);
    }

    /// Avalanche_Start: ids that match the first, a later or no avalanche; the
    /// slots free, partly used, all used (the next avalanche is tried), or
    /// holding this avalanche already; the 64 active nodes free, partly used
    /// or all but 1-2 used (the nodes run out, the slot is given back).
    #[test]
    fn avalanche_start_matches() {
        covered(
            ee::s09::f_002d97a8__generated,
            avalanche_start,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d97a8, 6);
                c.pc = 0x002d_97a8;
                let n_av = 1 + r.next_u32() % 3;
                oc1063_world(c, &mut r, n_av, 3, seed % 3 != 0);
                let target = r.next_u32() % n_av;
                c.g[A0][0] = if seed % 13 == 0 {
                    99
                } else {
                    OC1063_IDS[target as usize] as u64
                };
                let slots_mode = seed % 6;
                for i in 0..16u32 {
                    let slot = AVS_SLOTS + 0x1c * i;
                    let used = match slots_mode {
                        1 => r.next_u32() % 2 == 0,
                        2 => i == (seed / 6) % 16,
                        3 => true,
                        _ => false,
                    };
                    if used {
                        let v = if slots_mode == 2 {
                            OC1063_AV + 0x400 * target
                        } else {
                            OC1063_DUMMY + 0x40 * i
                        };
                        c.bus.write32(slot, v);
                    }
                }
                let nodes_mode = (seed / 6) % 4;
                let free = 1 + r.next_u32() % 2;
                let free_at = [r.next_u32() % 64, r.next_u32() % 64];
                for i in 0..64u32 {
                    let used = match nodes_mode {
                        1 => i < r.next_u32() % 6,
                        2 => !(free_at[..free as usize].contains(&i)),
                        3 => true,
                        _ => false,
                    };
                    if used {
                        c.bus
                            .write32(AVS_NODES + AVS_NODE_SIZE * i + 0x2e0, OC1063_DUMMY);
                    }
                }
            },
        );
    }

    /// Gate and mailbox of this module's fake `Avalanche_SampleHistory`.
    pub(crate) const OC1063_GATE: u32 = DATA + 0xf_e000;

    pub(crate) const OC1063_MAGIC: u32 = 0x4156_5331;

    pub(crate) static OC1063_PREV_SAMPLE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1063_fake_sample_history(c: &mut Cpu) {
        if c.bus.read32(OC1063_GATE) != OC1063_MAGIC {
            return fakes::fall_back(c, &OC1063_PREV_SAMPLE, ee::s09::f_002d5778__generated);
        }
        fakes::fake_call(c, 0x002d_5778);
    }

    /// Avalanche_LoadState: 16 saved entries (valid ids, repeats, unknown ids,
    /// negative ids; zero and positive times) restore into slots that were
    /// idle or in use; `Read` and `Avalanche_SampleHistory` are fakes.
    #[test]
    fn avalanche_load_state_matches() {
        const STREAM: u32 = DATA + 0x400;
        const STREAM_VT: u32 = DATA + 0x500;
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D5778),
            oc1063_fake_sample_history,
            &OC1063_PREV_SAMPLE,
        );
        covered(
            ee::s09::f_002d9d68__generated,
            avalanche_load_state,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d9d68, 6);
                c.pc = 0x002d_9d68;
                c.bus.write32(OC1063_GATE, OC1063_MAGIC);
                let n_av = r.next_u32() % 4;
                oc1063_world(c, &mut r, n_av, 2, seed % 4 == 0);
                c.bus.write32(STREAM, STREAM_VT);
                c.bus
                    .write16(STREAM_VT + 0x10, (4 * (r.next_u32() % 4)) as u16);
                c.bus.write32(STREAM_VT + 0x14, FAKE);
                c.g[A0][0] = STREAM as u64;
                // The record the (fake) Read leaves in the frame at sp - 0x150.
                let buf = STACK_TOP - 0x150;
                for i in 0..16u32 {
                    let id = match r.next_u32() % 6 {
                        0 => 0xffff_ffff,
                        1 => 77,
                        _ => OC1063_IDS[(r.next_u32() % 3) as usize],
                    };
                    c.bus.write32(buf + 4 * i, id);
                    let time = match r.next_u32() % 4 {
                        0 => 0,
                        _ => ((r.next_u32() % 40) as f32 * 0.25).to_bits(),
                    };
                    c.bus.write32(buf + 0x40 + 4 * i, time);
                }
                for i in 0..16u32 {
                    if r.next_u32() % 4 == 0 {
                        c.bus.write32(AVS_SLOTS + 0x1c * i, OC1063_DUMMY + 0x40 * i);
                    }
                }
            },
        );
    }

    pub(crate) const OC1067_CTRL: u32 = DATA + 0xf_1100;

    pub(crate) const OC1067_MAGIC: u32 = 0x1067_c0de;

    pub(crate) static OC1067_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1067_PREV_BAKE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1067_PREV_RELEASE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1067_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1067_PREV_TAIL: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1067_PREV_EXPORT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1067_PREV_LOAD: AtomicUsize = AtomicUsize::new(0);

    /// Gated fakes for the callees of the two drivers: they only log the call
    /// while the gate word is set, else run the body they replaced.
    pub(crate) fn oc1067_free_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_FREE, ee::s09::f_002d9a80__generated);
        }
        fakes::fake_call(c, 0x002d_9a80);
    }

    pub(crate) fn oc1067_bake_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_BAKE, ee::s09::f_002d9660__generated);
        }
        fakes::fake_call(c, 0x002d_9660);
    }

    pub(crate) fn oc1067_release_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_RELEASE, ee::s09::f_002db890__generated);
        }
        fakes::fake_call(c, 0x002d_b890);
    }

    pub(crate) fn oc1067_delete_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        fakes::fake_call(c, 0x0031_7e50);
    }

    pub(crate) fn oc1067_export_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_EXPORT, ee::s09::f_002d8258__generated);
        }
        fakes::fake_call(c, 0x002d_8258);
    }

    pub(crate) fn oc1067_load_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_LOAD, ee::s09::f_002d87d0__generated);
        }
        fakes::fake_call(c, 0x002d_87d0);
    }

    pub(crate) fn oc1067_tail_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_TAIL, ee::s06::f_002523a8__generated);
        }
        fakes::fake_call(c, 0x0025_23a8);
    }

    /// Hooks the fakes once per process and raises this machine's gate.
    pub(crate) fn oc1067_gate(c: &mut Cpu) {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D9A80),
            oc1067_free_fake,
            &OC1067_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D9660),
            oc1067_bake_fake,
            &OC1067_PREV_BAKE,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002DB890),
            oc1067_release_fake,
            &OC1067_PREV_RELEASE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1067_delete_fake,
            &OC1067_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002523A8),
            oc1067_tail_fake,
            &OC1067_PREV_TAIL,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D8258),
            oc1067_export_fake,
            &OC1067_PREV_EXPORT,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D87D0),
            oc1067_load_fake,
            &OC1067_PREV_LOAD,
        );
        c.bus.write32(OC1067_CTRL, OC1067_MAGIC);
    }

    /// 0..3 groups in the avalanche list (or none); the callees are logged
    /// fakes, and some seeds unwind at one of their calls.
    #[test]
    fn avalanche_bake_all_matches() {
        check(
            ee::s09::f_002d96e0__generated,
            avalanche_bake_all_2,
            200,
            &|c, seed| {
                machine(c, seed, 0x002d_96e0, 3);
                oc1067_gate(c);
                let mut head = 0u32;
                for g in (0..seed % 4).rev() {
                    let group = DATA + 0x1000 + g * 0x40;
                    c.bus.write32(group + 0x8, head);
                    head = group;
                }
                c.bus.write32(GAME_GP.wrapping_add(0x9c0), head);
            },
        );
    }

    /// Node groups at `gp+0x9c0`: 0..3 groups, each with 0..3 nodes (ids
    /// 1..12); a target id that is sometimes absent; permanent and other base
    /// types; `params` and the nodes at both alignments.
    #[test]
    fn avalanche_set_node_emitter_matches() {
        check(
            ssx3_ee::s09::f_002d9538__generated,
            avalanche_set_node_emitter,
            300,
            &|c, seed| {
                let mut r = Rng(0x002d_9538 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                let groups = seed % 4;
                let mut head = 0u32;
                for g in (0..groups).rev() {
                    let group = DATA + 0x1000 + g * 0x40;
                    let count = (seed / (g + 1)) % 4;
                    let skew = if (seed / 4) % 2 == 1 { 4 } else { 0 };
                    let mut first = 0u32;
                    for k in (0..count).rev() {
                        let node = DATA + 0x4000 + (g * 4 + k) * 0x100 + skew;
                        let base = if (seed + k) % 4 == 0 { 2 } else { (g + k) % 2 };
                        c.bus.write32(node + 0xf8, 1 + g * 4 + k);
                        c.bus.write32(node + 0xf4, first);
                        c.bus.write16(node + 0xf0, base as u16);
                        c.bus.write8(node + 0xf2, 0);
                        fill(c, &mut r, node, 0x36);
                        first = node;
                    }
                    c.bus.write32(group + 0x4, first);
                    c.bus.write32(group + 0x8, head);
                    head = group;
                }
                c.bus.write32(GAME_GP.wrapping_add(0x9c0), head);
                let target = if seed % 5 == 4 {
                    99
                } else {
                    1 + r.next_u32() % 12
                };
                c.g[A0][0] = sx(target);
                let params = DATA + 0x2000 + (seed % 4);
                fill(c, &mut r, params, 0x36);
                c.g[A1][0] = sx(params);
            },
        );
    }

    use crate::difftest::fakes::check_reaching;
    use crate::difftest::put_words;

    // Fixture objects live in DATA+0x1000.. (clear of the fake log at
    // DATA+0x100000 and of the allocator regions); this module's fakes are
    // gated by one word in the machine's RAM.

    /// Gate word of this module's fakes, and its magic value.
    pub(crate) const OC1075_CTRL: u32 = DATA + 0x2f_0000;

    pub(crate) const OC1075_MAGIC: u32 = 0x4f43_1075;

    /// Where the `EAFile_Write` fake copies the buffer it was given.
    pub(crate) const OC1075_CAPTURE: u32 = DATA + 0x2f_2000;

    /// The gated fake callee: logs the call like `fakes::fake_call` and, for
    /// `EAFile_Write`, captures the written bytes; for
    /// `Avalanche_SampleHistory`, fills the sampled frame (0x60 bytes at `a0`).
    pub(crate) fn oc1075_fake_call(c: &mut Cpu) {
        let target = c.pc;
        let ret = c.g[RA][0] as u32;
        let (a0, a2, a3) = (c.g[A0][0] as u32, c.g[A2][0] as u32, c.g[A3][0] as u32);
        fakes::fake(c, target, ret);
        if c.pc != ret {
            return;
        }
        match target {
            0x003d_ec80 => {
                let n = a3.min(0x100);
                for i in 0..n {
                    let b = c.bus.read8(a2 + i);
                    c.bus.write8(OC1075_CAPTURE + i, b);
                }
                c.bus.write32(OC1075_CAPTURE + 0x100, n);
            }
            0x002d_5778 => {
                for i in 0..0x18u32 {
                    let w = fakes::mix(0x5778 ^ (i as u64) << 20 ^ c.cycles) as u32;
                    c.bus.write32(a0 + 4 * i, w);
                }
            }
            _ => {}
        }
    }

    macro_rules! oc1075_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1075_CTRL) != OC1075_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1075_fake_call(c);
                }
            )*
            pub(crate) fn oc1075_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1075_fakes!(
        (
            oc1075_f_3b58a0,
            OC1075_P_3B58A0,
            ee::s13::OVERRIDE_003B58A0,
            ee::s13::f_003b58a0__generated
        ),
        (
            oc1075_f_3b89e0,
            OC1075_P_3B89E0,
            ee::s13::OVERRIDE_003B89E0,
            ee::s13::f_003b89e0__generated
        ),
        (
            oc1075_f_3b78f8,
            OC1075_P_3B78F8,
            ee::s13::OVERRIDE_003B78F8,
            ee::s13::f_003b78f8__generated
        ),
        (
            oc1075_f_3b58d8,
            OC1075_P_3B58D8,
            ee::s13::OVERRIDE_003B58D8,
            ee::s13::f_003b58d8__generated
        ),
        (
            oc1075_f_3b9880,
            OC1075_P_3B9880,
            ee::s13::OVERRIDE_003B9880,
            ee::s13::f_003b9880__generated
        ),
        (
            oc1075_f_3dec80,
            OC1075_P_3DEC80,
            ee::s14::OVERRIDE_003DEC80,
            ee::s14::f_003dec80__generated
        ),
        (
            oc1075_f_2d5778,
            OC1075_P_2D5778,
            ee::s09::OVERRIDE_002D5778,
            ee::s09::f_002d5778__generated
        ),
        (
            oc1075_f_2d7dd8,
            OC1075_P_2D7DD8,
            ee::s09::OVERRIDE_002D7DD8,
            ee::s09::f_002d7dd8__generated
        ),
        (
            oc1075_f_3717c0,
            OC1075_P_3717C0,
            ee::s12::OVERRIDE_003717C0,
            ee::s12::f_003717c0__generated
        ),
        (
            oc1075_f_2a6f38,
            OC1075_P_2A6F38,
            ee::s08::OVERRIDE_002A6F38,
            ee::s08::f_002a6f38__generated
        ),
        (
            oc1075_f_2a4be0,
            OC1075_P_2A4BE0,
            ee::s08::OVERRIDE_002A4BE0,
            ee::s08::f_002a4be0__generated
        ),
        (
            oc1075_f_2a82d8,
            OC1075_P_2A82D8,
            ee::s08::OVERRIDE_002A82D8,
            ee::s08::f_002a82d8__generated
        ),
        (
            oc1075_f_2adee8,
            OC1075_P_2ADEE8,
            ee::s08::OVERRIDE_002ADEE8,
            ee::s08::f_002adee8__generated
        ),
        (
            oc1075_f_28bb10,
            OC1075_P_28BB10,
            ee::s07::OVERRIDE_0028BB10,
            ee::s07::f_0028bb10__generated
        ),
        (
            oc1075_f_28fea0,
            OC1075_P_28FEA0,
            ee::s07::OVERRIDE_0028FEA0,
            ee::s07::f_0028fea0__generated
        ),
        (
            oc1075_f_317e50,
            OC1075_P_317E50,
            ee::s10::OVERRIDE_00317E50,
            ee::s10::f_00317e50__generated
        ),
        (
            oc1075_f_317e98,
            OC1075_P_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated
        ),
        (
            oc1075_f_11b978,
            OC1075_P_11B978,
            ee::s00::OVERRIDE_0011B978,
            ee::s00::f_0011b978__generated
        ),
    );

    /// A test machine (random registers, miss hooks, a call that may unwind
    /// among the first `calls`), the game image, and the gate open.
    pub(crate) fn oc1075_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        elf_image(c);
        oc1075_install_fakes();
        c.bus.write32(OC1075_CTRL, OC1075_MAGIC);
        r
    }

    /// `Avalanche_UpdateInstance`: a node without data, positions past and
    /// inside the data's length, type-2 and other types, emitting or not,
    /// callees (`SampleHistory`, `SpawnAt`, `ReleaseInstance`) that unwind.
    #[test]
    fn oc1075_avalanche_update_instance_matches() {
        check_reaching(
            ee::s09::f_002d7ca8__generated,
            oc1075_avalanche_update_instance,
            400,
            &[0x002d_7ce8, 0x002d_7d98, 0x002d_7db8],
            &|c, seed| {
                let mut r = oc1075_machine(c, seed, 0x2d_7ca8, 3);
                let node = DATA + 0x3000;
                let data = DATA + 0x4_0000;
                c.bus
                    .write32(node + AU_DATA, if r.next_u32() % 7 == 0 { 0 } else { data });
                let rate = [0.5f32, 1.0, 2.0][(r.next_u32() % 3) as usize];
                let length = [1.0f32, 10.0, 100.0][(r.next_u32() % 3) as usize];
                let t = [0.0f32, 0.1, 1.0, 3.0][(r.next_u32() % 4) as usize];
                c.bus.write32(data + AU_D_RATE, rate.to_bits());
                c.bus.write32(data + AU_D_LENGTH, length.to_bits());
                c.bus.write16(data + AU_D_TYPE, (r.next_u32() % 3) as u16);
                c.bus.write8(data + AU_D_EMITS, (r.next_u32() % 2) as u8);
                c.bus.write32(node + AU_FB4, r.next_u32());
                put_words(c, GAME_GP - 0x3cd4, &[(1.0f32 / 60.0).to_bits()]);
                c.g[A0][0] = sx(node);
                c.f[12] = t.to_bits();
                c.pc = 0x002d_7ca8;
            },
        );
    }

    /// `Avalanche_WriteData`: a direct write, and buffered writes of 0..4 rows
    /// of 0..8 bytes (reversed copies; the write fake captures the buffer so
    /// the diff sees it), `EAFile_Write` that unwinds.
    #[test]
    fn oc1075_avalanche_write_data_matches() {
        check(
            ee::s09::f_002d82a0__generated,
            oc1075_avalanche_write_data,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x2d_82a0, 1);
                elf_image(c);
                oc1075_install_fakes();
                c.bus.write32(OC1075_CTRL, OC1075_MAGIC);
                let src = DATA + 0x100;
                fill(c, &mut r, src, 0x40);
                let rows = r.next_u32() % 5;
                let stride = [0u32, 1, 3, 5, 8][(r.next_u32() % 5) as usize];
                c.g[A0][0] = (seed % 3 == 0) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = sx(src);
                c.g[T0][0] = stride as u64;
                c.g[T1][0] = rows as u64;
                c.pc = 0x002d_82a0;
            },
        );
    }

    /// `Avalanche_GetProximityInfo`: 16 slots, empty / active, with 0..3
    /// nodes each (some without data, near and far from the position, types 1
    /// and 2), so the centre, scale, minimum, count and flag all vary.
    #[test]
    fn oc1075_avalanche_get_proximity_info_matches() {
        check(
            ee::s09::f_002da1c0__generated,
            oc1075_avalanche_get_proximity_info,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d_a1c0, 0);
                put_words(c, GAME_GP - 0x3cbc, &[999_999.0f32.to_bits()]);
                put_words(c, GAME_GP - 0x3cb8, &[225_000_000.0f32.to_bits()]);
                let pos = DATA + 0x200;
                for i in 0..3 {
                    c.bus.write32(pos + 4 * i, r.f32(8000.0));
                }
                let mut k = 0u32;
                for slot in 0..16u32 {
                    let at = 0x0053_8938 + 0x1c * slot;
                    let active = r.next_u32() % 3 != 0;
                    let mut head = 0;
                    let mut prev = 0;
                    for _ in 0..(r.next_u32() % 4) {
                        let node = DATA + 0x1000 + 0x400 * k;
                        let nd = DATA + 0x1_8000 + 0x100 * k;
                        k += 1;
                        if prev == 0 {
                            head = node;
                        } else {
                            c.bus.write32(prev + 0x2e4, node);
                        }
                        prev = node;
                        c.bus.write32(node + 0x2e4, 0);
                        c.bus
                            .write32(node + 0x2e0, if r.next_u32() % 5 == 0 { 0 } else { nd });
                        for j in 0..3 {
                            c.bus.write32(node + 0x60 + 4 * j, r.f32(8000.0));
                        }
                        c.bus.write32(node + 0x6c, 1.0f32.to_bits());
                        c.bus.write32(node + 0xb0, r.f32(10.0));
                        c.bus.write16(nd + 0xf0, 1 + (r.next_u32() % 2) as u16);
                    }
                    c.bus.write32(at, active as u32);
                    c.bus.write32(at + 4, head);
                }
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(pos);
                c.pc = 0x002d_a1c0;
            },
        );
    }
}
