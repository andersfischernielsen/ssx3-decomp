//! `src/ui/uistatestack.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sra32;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::p_2 as p;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::render::particle::this_with_delta;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0039_f238,
        ui_state_mgr_add_background_state,
        "UIStateMgr_AddBackgroundState",
    ),
    (
        0x0039_f320,
        uistatemgr_switch_state_immediate,
        "UIStateMgr_SwitchStateImmediate",
    ),
    (
        0x0039_f400,
        ui_state_mgr_switch_state,
        "UIStateMgr_SwitchState",
    ),
    (
        0x0039_f4c0,
        ui_state_mgr_switch_state_close_popups,
        "UIStateMgr_SwitchStateClosePopups",
    ),
    (
        0x0039_f600,
        ui_state_mgr_close_unpinned_popups,
        "UIStateMgr_CloseUnpinnedPopups",
    ),
    (
        0x0039_f698,
        uistatemgr_close_all_popups,
        "UIStateMgr_CloseAllPopups",
    ),
    (
        0x0039_f718,
        ui_state_mgr_close_top_state_and_popups,
        "UIStateMgr_CloseTopStateAndPopups",
    ),
    (0x0039_fae8, ui_list_first_wrapper, "UIList_First_Wrapper"),
    (0x0039_fc48, uiobject_dtor, "UIObject_dtor"),
    (
        0x0039_feb8,
        uiobject_apply_alignment,
        "UIObject_ApplyAlignment",
    ),
    (0x0039_ff60, uiobject_find_by_id, "UIObject_FindById"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST_3;

    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_3;

    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY;

    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_FIRST;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
}

// ---------------------------------------------------------------------------
// UI object

/// `void UIObject_ApplyAlignment(UIObject *obj, Vec2 *dst, const Vec2 *size)`
/// (0x39feb8): apply the object's alignment flags (bits 13..18 of the word at
/// +0x14) to adjust `dst` against the size.
pub fn uiobject_apply_alignment(c: &mut Cpu) {
    c.cycles += 6;
    let obj = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    c.g[V0][0] = Addr(obj).off(0x14).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 13);
    c.g[A0][0] = c.g[V0][0] & 0x3f;
    c.g[V1][0] = c.g[A0][0] & 0x1;
    let centered = c.g[V1][0] != 0;
    c.g[V0][0] = c.g[A0][0] & 0x8;
    if !centered {
        c.cycles += 3;
        c.g[V0][0] = c.g[A0][0] & 0x4;
        if c.g[V0][0] == 0 {
            c.f[0] = Addr(a2).off(0x4).read_fpu(c).0;
            c.cycles += 6;
            c.g[AT][0] = 0x3f00_0000;
            c.f[2] = c.g[1][0] as u32;
            c.f[1] = Addr(a1).off(0x4).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
            c.f[1] = (FpuF32(c.f[1]) - FpuF32(c.f[0])).0;
            Addr(a1).off(0x4).write_fpu(c, FpuF32(c.f[1]));
        } else {
            c.cycles += 5;
            c.f[1] = Addr(a2).off(0x4).read_fpu(c).0;
            c.f[0] = Addr(a1).off(0x4).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) - FpuF32(c.f[1])).0;
            Addr(a1).off(0x4).write_fpu(c, FpuF32(c.f[0]));
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0] & 0x8;
    }
    c.cycles += 2;
    let skip_horizontal = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[A0][0] & 0x20;
    if !skip_horizontal {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.f[0] = Addr(a2).read_fpu(c).0;
            c.cycles += 6;
            c.g[AT][0] = 0x3f00_0000;
            c.f[2] = c.g[1][0] as u32;
            c.f[1] = Addr(a1).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
            c.f[1] = (FpuF32(c.f[1]) - FpuF32(c.f[0])).0;
            Addr(a1).write_fpu(c, FpuF32(c.f[1]));
        } else {
            c.cycles += 5;
            c.f[1] = Addr(a2).read_fpu(c).0;
            c.f[0] = Addr(a1).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) - FpuF32(c.f[1])).0;
            Addr(a1).write_fpu(c, FpuF32(c.f[0]));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `UIObject *UIObject_FindById(UIObject *obj, int id)` (0x39ff60): `obj` when
/// its id (+0x38) equals `id`, else null.
pub fn uiobject_find_by_id(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0x38).lw(c);
    c.g[V0][0] = 0;
    c.g[V1][0] = c.g[V1][0] ^ c.g[A1][0];
    if c.g[V1][0] == 0 {
        c.g[V0][0] = c.g[A0][0];
    }
    abi::ret(c);
}

/// `void *UIList_First_Wrapper(cList *list)` (0x0039fae8): the list's first
/// node.
pub fn ui_list_first_wrapper(c: &mut Cpu) {
    forward(c, 7, 0x10, |_| {}, callee::UI_LIST_FIRST, 0x0039_faf8)
}

/// `void UIStateMgr_AddBackgroundState(UIStateMgr* self, State* state)`
/// (0x39f238): mark the state as a background state and append it to the
/// background list.
pub fn ui_state_mgr_add_background_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x1c).lw(c);
        c.g[V1][0] = Addr(r32(c, S0)).off(0x8).lw(c);
        c.g[V0][0] |= 0x60;
        Addr(r32(c, S0)).off(0x1c).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x20).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x24).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0039_f270);
        c.g[A0][0] = this_with_delta(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, target, 0x0039_f270)?;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x1c));
        c.g[RA][0] = sx(0x0039_f27c);
        c.g[A1][0] = c.g[S0][0];
        callee::UI_LIST_PUSH_BACK_2.call(c, 0x0039_f27c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIStateMgr_CloseAllPopups()` (0x39f698): walk the popup list and
/// clear the "popping" bits (7, 8, and the 3-bit popup kind when it is below
/// 6) of every entry; returns the last node.
pub fn uistatemgr_close_all_popups(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::UI_LIST_FIRST_2.call(c, 0x0039_f6a8)?;
        let node = c.g[V0][0];
        c.g[A0][0] = node;
        let mut v0 = 0u64;
        if node != 0 {
            c.cycles += 4;
            c.g[V0][0] = lw_at(c, p(node as u32, 0x1c));
            c.g[V0][0] &= 1;
            c.g[A2][0] = 0xffff_ffff_ffff_c0ff;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = 0xffff_ffff_ffff_ff7f;
                c.g[V1][0] = lw_at(c, p(node as u32, 0x1c));
                loop {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    c.g[V0][0] &= 0x3f;
                    c.g[V0][0] = (c.g[V0][0] < 6) as u64;
                    let small = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0] & c.g[A2][0];
                    if !small {
                        c.cycles += 4;
                        c.g[V0][0] |= 0x700;
                        c.g[V0][0] &= c.g[A1][0];
                        c.g[V0][0] |= 0x80;
                        c.sw(p(c.g[A0][0] as u32, 0x1c), c.g[V0][0]);
                    }
                    c.cycles += 5;
                    c.g[A0][0] = lw_at(c, p(c.g[A0][0] as u32, 4));
                    c.g[V0][0] = lw_at(c, p(c.g[A0][0] as u32, 0x1c));
                    c.g[V0][0] &= 1;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V1][0] = lw_at(c, p(c.g[A0][0] as u32, 0x1c));
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_f6d0);
                    }
                }
            }
            c.cycles += 1;
            v0 = c.g[A0][0];
        }
        c.g[V0][0] = v0;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIObject_dtor(self, flags)` (0x39fc48): destroy the child object, then
/// the node body, and free the object when `flags & 1`.
pub fn uiobject_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x490000;
        frame.save(c, 16, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4bd8));
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(p(r32(c, S0), 8), c.g[V0][0]);
        c.g[A2][0] = lw_at(c, p(r32(c, S0), 0xc));
        let no_child = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_child {
            c.cycles += 6;
            c.g[V0][0] = lw_at(c, p(c.g[A2][0] as u32, 0x10));
            c.g[A1][0] = 3;
            let func = vslot(c, c.g[V0][0], 8, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, 0x0039_fc8c)?;
        }
        c.cycles += 8;
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4cc0));
        c.sw(p(r32(c, S0), 8), c.g[V0][0]);
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039_fca0)?;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::BUILTIN_DELETE_3.addr, 0x0039_fcb4)?;
        }
        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIStateMgr_CloseUnpinnedPopups` (0x39f600): walk the UI list and close
/// every unpinned popup (clearing its pin state bits).
pub fn ui_state_mgr_close_unpinned_popups(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0039_f610);
        callee::UI_LIST_FIRST_3.call(c, 0x0039_f610)?;
        c.g[A0][0] = c.g[V0][0];
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 4;
            c.g[V1][0] = Addr(p(r(c, A0), 0x1c)).lw(c);
            c.g[V0][0] = c.g[V1][0] & 1;
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = sra32(c.g[V1][0], 15);
            let mut direct = false;
            if !t {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 1;
                let pinned = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[A0][0];
                if pinned {
                    direct = true;
                } else {
                    c.cycles += 4;
                    c.g[A2][0] = 0xffff_ffff_ffff_c0ff;
                    c.g[A1][0] = 0xffff_ffff_ffff_ff7f;
                    c.g[V1][0] = Addr(p(r(c, A0), 0x1c)).lw(c);
                    c.g[V0][0] = sx(r(c, V1) >> 8);
                    loop {
                        c.cycles += 4;
                        c.g[V0][0] = c.g[V0][0] & 0x3f;
                        c.g[V0][0] = (c.g[V0][0] < 6) as u64;
                        let zero = c.g[V0][0] == 0;
                        c.g[V0][0] = c.g[V1][0] & c.g[A2][0];
                        if !zero {
                            c.cycles += 4;
                            c.g[V0][0] = c.g[V0][0] | 0x700;
                            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                            c.g[V0][0] = c.g[V0][0] | 0x80;
                            Addr(p(r(c, A0), 0x1c)).sw(c, c.g[V0][0]);
                        }
                        c.cycles += 5;
                        c.g[A0][0] = Addr(p(r(c, A0), 4)).lw(c);
                        c.g[V1][0] = Addr(p(r(c, A0), 0x1c)).lw(c);
                        c.g[V0][0] = c.g[V1][0] & 1;
                        let t = c.g[V0][0] == 0;
                        c.g[V0][0] = sra32(c.g[V1][0], 15);
                        if t {
                            break;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V0][0] & 1;
                        let cont = c.g[V0][0] == 0;
                        c.g[V0][0] = sx(r(c, V1) >> 8);
                        if !cont {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0039_f648);
                        }
                    }
                }
            }
            if !direct {
                c.cycles += 1;
                c.g[V0][0] = c.g[A0][0];
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIListNode* UIStateMgr_CloseTopStateAndPopups(list)` (0x39f718): for the
/// list's first node, clear the "visible" bit on each id<6 node until the
/// sentinel and return the sentinel's next.
pub fn ui_state_mgr_close_top_state_and_popups(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, table::UI_LIST_FIRST, 0x0039_f728)?;
        let first = c.g[V0][0];
        c.g[V1][0] = first;
        let empty = c.g[V1][0] == 0;
        c.g[V0][0] = 0;
        if empty {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
        c.g[V0][0] &= 1;
        let clean = c.g[V0][0] == 0;
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
        if !clean {
            c.cycles += 3;
            c.g[A2][0] = 0xffff_ffff_ffff_c0ff;
            c.g[A1][0] = 0xffff_ffff_ffff_ff7f;
            'nodes: loop {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[A0][0] as u32) >> 8);
                c.g[V0][0] &= 0x3f;
                c.g[V0][0] = (c.g[V0][0] < 6) as u64;
                let small = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[A0][0] & c.g[A2][0];
                if !small {
                    c.cycles += 4;
                    c.g[V0][0] |= 0x700;
                    c.g[V0][0] &= c.g[A1][0];
                    c.g[V0][0] |= 0x80;
                    Addr(c.g[V1][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
                }
                c.cycles += 5;
                c.g[V1][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
                c.g[V0][0] &= 1;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_f750);
                    }
                    continue 'nodes;
                }
                break 'nodes;
            }
        }
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[A0][0] as u32) >> 8);
        c.g[V0][0] &= 0x3f;
        c.g[V0][0] = (c.g[V0][0] < 6) as u64;
        let small = c.g[V0][0] == 0;
        c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
        if !small {
            c.cycles += 3;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            c.g[V0][0] |= 0x780;
            Addr(c.g[V1][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
        }
        c.cycles += 1;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIStateMgr_SwitchState` (0x0039f400).
//
// `void UIStateMgr::SwitchState(UIStateMgr* m, UIListNode* n)`: push the new
// state node in front of the current one and patch its visibility bits.

pub fn ui_state_mgr_switch_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 16, 0x30);
        let mgr = c.g[A0][0];
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        let node = c.g[A1][0];
        c.g[S1][0] = mgr;
        c.g[S0][0] = node;
        call_ptr(c, 0x0039_7840, 0x0039_f420)?;
        c.g[A0][0] = c.g[V0][0];
        let a0 = c.g[A0][0];
        c.g[S2][0] = 0;
        let v0;
        if a0 == 0 {
            v0 = c.g[V0][0];
        } else {
            c.cycles += 5;
            let v1 = lw_2(c, a0 as u32, 0x1c);
            c.g[V1][0] = v1;
            c.g[V0][0] = sx((((v1 as u32) as i32) >> 5) as u32);
            c.g[V0][0] = c.g[V0][0] & 1;
            let clear5 = c.g[V0][0] == 0;
            c.g[V0][0] = sx((((v1 as u32) as i32) >> 6) as u32);
            if clear5 {
                v0 = c.g[V0][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 1;
                let set6 = c.g[V0][0] != 0;
                c.g[V0][0] = sx((v1 as u32) >> 8);
                if set6 {
                    v0 = c.g[V0][0];
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[V0][0] & 0x3f;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 6) as u64;
                    let big = c.g[V0][0] == 0;
                    c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
                    if big {
                        v0 = c.g[V0][0];
                    } else {
                        c.cycles += 4;
                        c.g[S2][0] = 1;
                        c.g[V0][0] = v1 & c.g[V0][0];
                        c.g[V0][0] = c.g[V0][0] | 0x680;
                        sw_2(c, a0 as u32, 0x1c, c.g[V0][0]);
                        v0 = c.g[V0][0];
                    }
                }
            }
        }
        // b_39f48c / b_39f498.
        c.cycles += 2;
        let s2 = c.g[S2][0] == 0;
        c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
        if s2 {
            c.cycles += 3;
            c.g[V0][0] = lw_2(c, node as u32, 0x1c);
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x220;
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw_2(c, node as u32, 0x1c);
            c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x100;
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        }
        let _ = v0;
        c.cycles += 10;
        sw_2(c, node as u32, 0x1c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0039_78f0, 0x0039_f4a8)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI state stack.

/// `void UIStateMgr_SwitchStateImmediate(mgr, st, a2, a3)` (0x39f320): close
/// the current state when it is a popup of id 5, then push `st` and mark its
/// flags.
pub fn uistatemgr_switch_state_immediate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        call_ptr(c, 0x0039_fa88, 0x0039_f34c)?;
        c.g[A1][0] = c.g[V0][0];
        let mut plus_one = false;
        if c.g[A1][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
            c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
            c.g[S3][0] = 1;
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x200;
            Addr(p(r32(c, S0), 0x1c)).sw(c, c.g[V0][0]);
            plus_one = true;
        } else {
            c.cycles += 5;
            c.g[A0][0] = Addr(p(r32(c, A1), 0x1c)).lw(c);
            c.g[V0][0] = sx(((r32(c, A0) as i32) >> 5) as u32);
            c.g[V0][0] = c.g[V0][0] & 1;
            let not_popup = c.g[V0][0] == 0;
            c.g[V0][0] = sx(r32(c, A0) >> 8);
            if not_popup {
                plus_one = true;
            } else {
                c.cycles += 4;
                c.g[V1][0] = 5;
                c.g[V0][0] = c.g[V0][0] & 0x3f;
                if c.g[V0][0] != c.g[V1][0] {
                    // not the buy popup: nothing to close
                } else {
                    c.cycles += 10;
                    c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
                    c.g[V1][0] = c.g[S1][0] & 1;
                    c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                    c.g[V1][0] = sx(r32(c, V1) << 7);
                    c.g[A0][0] = 0xffff_ffff_ffff_ff7f;
                    c.g[V0][0] = c.g[V0][0] | 0x700;
                    c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    Addr(p(r32(c, A1), 0x1c)).sw(c, c.g[V0][0]);
                    plus_one = true;
                }
            }
        }
        if plus_one {
            c.cycles += 1;
        }
        c.g[A0][0] = c.g[S2][0];
        c.cycles += 16;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0039_78f0, 0x0039_f3c4)?;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
        c.g[V1][0] = c.g[S3][0] & 1;
        c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
        c.g[V1][0] = sx(r32(c, V1) << 5);
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        frame.restore(c, S2, 0x20);
        Addr(p(r32(c, S0), 0x1c)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIStateMgr_SwitchStateClosePopups` (0x0039f4c0): close the popup widgets
/// of a UI state switch.  Starting from the first widget of `a0`'s list
/// (`UIList_First`), every widget still marked visible (bit 0 of `+0x1c`)
/// is walked: its `0x3f`-field is raised to `0x7` when below 6, bit 14 is
/// set, and the loop stops at the first hidden one; finally the transition
/// flags at `a1+0x1c` are updated (bit 8 with bit 5 clear, or `0x220` if a
/// visible widget was seen) and the widget is pushed to the front of `a0`'s
/// list.
pub fn ui_state_mgr_switch_state_close_popups(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0039_f4e8);
        c.g[S3][0] = 0u64;
        call_ptr(c, 0x0039_7840, 0x0039_f4e8)?;
        c.g[A0][0] = c.g[V0][0];
        let no_list = c.g[A0][0] == 0;
        c.g[S0][0] = 0u64;
        'outer: {
            if no_list {
                break 'outer;
            }
            'walk: {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let hidden = c.g[V0][0] == 0;
                if hidden {
                    break 'walk;
                }
                c.cycles += 3;
                c.g[A3][0] = 0xffff_ffff_ffff_c0ff;
                c.g[A2][0] = 0xffff_ffff_ffff_ff7f;
                c.g[A1][0] = 0xffff_ffff_ffff_bfff;
                'lv: loop {
                    'level: {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                        c.g[V0][0] = c.g[V0][0] & 0x3f;
                        c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                        let low = c.g[V0][0] == 0;
                        c.g[V0][0] = c.g[V1][0] & c.g[A3][0];
                        if !low {
                            c.cycles += 4;
                            c.g[V0][0] = c.g[V0][0] | 0x700;
                            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                            c.g[V0][0] = c.g[V0][0] | 0x80;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                        }
                        c.cycles += 9;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                        c.g[V0][0] = c.g[V0][0] | 0x4000;
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        let more = c.g[V0][0] != 0;
                        c.g[S0][0] = 0x1;
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0039_f510);
                            }
                            continue 'lv;
                        }
                        break 'level;
                    }
                    break 'walk;
                }
            }
            'mark: {
                c.cycles += 2;
                let seen = c.g[S0][0] != 0;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                if seen {
                    break 'mark;
                }
                c.cycles += 4;
                c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let clear = c.g[V0][0] == 0;
                if clear {
                    break 'outer;
                }
            }
            c.cycles += 4;
            c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let set = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            if set {
                break 'outer;
            }
            c.cycles += 4;
            c.g[V0][0] = c.g[V0][0] & 0x3f;
            c.g[V0][0] = ((c.g[V0][0] as i64) < 6i64) as u64;
            let low = c.g[V0][0] == 0;
            c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
            if low {
                break 'outer;
            }
            c.cycles += 4;
            c.g[S3][0] = 0x1;
            c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
            c.g[V0][0] = c.g[V0][0] | 0x680;
            c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        }
        'flags: {
            'plain: {
                c.cycles += 2;
                let no_seen = c.g[S3][0] == 0;
                c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
                if no_seen {
                    break 'plain;
                }
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | 0x100;
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                break 'flags;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x220;
            break 'flags;
        }
        c.cycles += 11;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0039_f5e0);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_78f0, 0x0039_f5e0)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::*;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::be::beintplayer::tests::r_u32;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_2;

    pub(crate) use crate::game::main::gamestate::tests::base_3 as base;
    pub(crate) use crate::game::main::gamestate::tests::slot_2 as slot;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::common;

    pub(crate) use crate::game::render::particle::tests::vtable_2;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_4;

    pub(crate) use crate::game::world::worldcache::tests::check_all;
    use ssx3_ee as ee;

    #[test]
    fn uiobject_apply_alignment_matches() {
        check(
            ssx3_ee::s13::f_0039feb8,
            uiobject_apply_alignment,
            300,
            &|c, seed| {
                let mut r = Rng(0x39fe_b800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                c.bus.write32(DATA + 0x14, (r.next_u32() & 0x3f) << 13);
                c.bus.write32(DATA + 0x100, r.edge_f32());
                c.bus.write32(DATA + 0x104, r.edge_f32());
                c.bus.write32(DATA + 0x200, r.edge_f32());
                c.bus.write32(DATA + 0x204, r.edge_f32());
            },
        );
    }

    #[test]
    fn uiobject_find_by_id_matches() {
        check(
            ssx3_ee::s13::f_0039ff60,
            uiobject_find_by_id,
            300,
            &|c, seed| {
                let mut r = Rng(0x39ff_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
                let id = r.next_u32();
                c.g[A1][0] = if seed % 3 == 0 {
                    id as u64
                } else {
                    (id as u64) | ((r.next_u32() as u64) << 32)
                };
                c.bus.write32(DATA + 0x38, id);
            },
        );
    }

    #[test]
    fn ui_list_first_wrapper_matches() {
        check(
            ee::s13::f_0039fae8__generated,
            ui_list_first_wrapper,
            200,
            &|c, seed| {
                machine(c, seed, 0x39_fae8, 0);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                match seed % 3 {
                    0 => c.bus.write32(DATA + 0x1000 + 4, 0),
                    1 => {
                        let node = DATA + 0x800;
                        c.bus.write32(DATA + 0x1000 + 4, node);
                        c.bus.write32(node + 4, node);
                    }
                    _ => {
                        let node = DATA + 0x800;
                        let last = DATA + 0x900;
                        c.bus.write32(DATA + 0x1000 + 4, node);
                        c.bus.write32(node + 4, last);
                    }
                }
            },
        );
    }

    #[test]
    fn ui_state_mgr_add_background_state_matches() {
        check(
            ee::s13::f_0039f238__generated,
            ui_state_mgr_add_background_state,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x39f238, 2);
                let this = DATA + 0x100;
                let state = DATA + 0x200;
                let vt = DATA + 0x300;
                clear(c, this, 0x100);
                clear(c, state, 0x100);
                clear(c, vt, 0x200);
                c.bus.write32(state + 8, vt);
                vtable_2(c, vt, &[(0x20, 0, FAKE)]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(state);
            },
        );
    }

    #[test]
    fn uistatemgr_close_all_popups_matches() {
        check(
            ee::s13::f_0039f698__generated,
            uistatemgr_close_all_popups,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xf698, 0);
                let list = DATA + 0x800;
                let n1 = DATA + 0x840;
                let n2 = DATA + 0x880;
                c.g[A0][0] = sx(list);
                c.bus.write32(list + 4, if seed % 5 == 0 { 0 } else { n1 });
                c.bus.write32(n1 + 4, if seed % 2 == 0 { 0 } else { n2 });
                c.bus.write32(n1 + 0x1c, ((seed % 8) << 8) | 1);
                c.bus.write32(n2 + 4, n2);
                // Even: the walk stops after this node.
                c.bus.write32(n2 + 0x1c, (r_u32(seed, 7) << 1) & !1);
            },
        );
    }

    #[test]
    fn uiobject_dtor_matches() {
        covered(
            ee::s13::f_0039fc48__generated,
            uiobject_dtor,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0xfc48, 1);
                let s = DATA + 0x100;
                let child = DATA + 0x200;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed & 1) as u64;
                c.bus
                    .write32(s + 0xc, if seed % 3 == 0 { 0 } else { child });
                let vt = DATA + 0x300;
                slot(c, vt, 8, 0, FAKE);
                c.bus.write32(child + 0x10, vt);
            },
        );
    }

    #[test]
    fn ui_state_mgr_close_unpinned_popups_matches() {
        check(
            ee::s13::f_0039f600__generated,
            ui_state_mgr_close_unpinned_popups,
            SEEDS_2,
            &|c, seed| {
                let _ = base_2(c, seed, 0xf600, 2);
                let head = DATA;
                let entry = DATA + 0x800;
                w(c, head + 4, if seed & 1 == 0 { head } else { entry });
                w(c, entry + 4, head);
                w(c, head + 0x1c, 0);
                let flags = ((seed & 1) as u32) | ((seed & 2) << 14);
                w(c, entry + 0x1c, flags);
                c.g[A0][0] = head as u64;
            },
        );
    }

    #[test]
    fn ui_state_mgr_close_top_state_and_popups_matches() {
        check(
            ssx3_ee::s13::f_0039f718__generated,
            ui_state_mgr_close_top_state_and_popups,
            30,
            &|c, seed| {
                let _r = base_3(c, seed, 0x39f718, 1);
                let list = DATA + 0x1000;
                c.g[A0][0] = list as u64;
                let sentinel = DATA + 0x5000;
                w_2(c, sentinel, sentinel);
                w_2(c, sentinel + 4, sentinel);
                if seed % 4 == 0 {
                    w_2(c, list + 4, sentinel);
                } else {
                    let n0 = DATA + 0x2000;
                    let n1 = DATA + 0x3000;
                    w_2(c, list + 4, n0);
                    w_2(c, n0, sentinel);
                    w_2(c, n0 + 4, if seed % 2 == 0 { n1 } else { sentinel });
                    w_2(c, n0 + 0x1c, (seed % 2) | ((seed % 5) << 8));
                    w_2(c, n1, sentinel);
                    w_2(c, n1 + 4, sentinel);
                    w_2(c, n1 + 0x1c, ((seed / 2) % 5) << 8);
                }
            },
        );
    }

    #[test]
    fn ui_state_mgr_switch_state_matches() {
        check(
            ee::s13::f_0039f400__generated,
            ui_state_mgr_switch_state,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39f4, 4);
                let mgr = DATA;
                let node = DATA + 0x1000;
                c.bus.write32(mgr, mgr);
                c.bus.write32(mgr + 4, node);
                c.bus.write32(node + 0x1c, r.next_u32());
                c.bus.write32(node + 4, mgr);
                c.g[A0][0] = sx(mgr);
                c.g[A1][0] = sx(node);
            },
        );
    }

    #[test]
    fn uistatemgr_switch_state_immediate_matches() {
        check(
            ee::s13::f_0039f320__generated,
            uistatemgr_switch_state_immediate,
            8,
            &|c, seed| {
                let _ = base_4(c, seed, 0x39f320, 2);
                let mgr = DATA;
                let st = DATA + 0x200;
                clear_2(c, mgr, 0x200);
                clear_2(c, st, 0x200);
                c.g[A0][0] = mgr as u64;
                c.g[A1][0] = st as u64;
                c.g[A2][0] = (seed % 2) as u64;
                c.g[A3][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn ui_state_mgr_switch_state_close_popups_matches() {
        const S: u64 = 0x0039_f4c0;
        check_all(
            ee::s13::f_0039f4c0__generated,
            ui_state_mgr_switch_state_close_popups,
            40,
            &|c, seed| {
                machine(c, seed, S, 0);
                let this = DATA + 0x1000;
                let head = DATA + 0x2000;
                let node0 = DATA + 0x3000;
                let node1 = DATA + 0x4000;
                let trans = DATA + 0x5000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(trans);
                w(c, this + 4, head);
                w(c, head, head);
                if seed % 5 == 0 {
                    // An empty list: `UIList_First` returns 0.
                    w(c, head + 4, head);
                } else {
                    w(c, head + 4, node0);
                    w(c, node0 + 4, node1);
                    w(c, node1 + 4, 0);
                    let f0 = (seed as u32) << 4 | 1; // bit 0: visible
                    let f1 = (seed as u32) << 8; // bit 0 clear: hidden
                    w(c, node0 + 0x1c, f0);
                    w(c, node1 + 0x1c, f1);
                    w(c, node0 + 0x10, (seed % 3) as u32);
                    w(c, node1 + 0x10, (seed % 3) as u32);
                }
                let tf = match seed % 4 {
                    0 => 0,
                    1 => 0x20,
                    2 => 0x40,
                    _ => 0x20 | 0x40,
                };
                w(c, trans + 0x1c, tf | ((seed as u32) << 12));
            },
        );
    }
}
