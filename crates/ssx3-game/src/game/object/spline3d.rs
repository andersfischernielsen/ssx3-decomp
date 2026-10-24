//! `src/object/spline3d.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p;
pub(crate) use crate::game::object::movenode::GFX_MAN;
pub(crate) use crate::game::object::movenode::ONE;
use crate::guest::Addr;
use crate::ps2f::vu0_scratch;
use crate::ps2f::FpuF32;
use crate::ps2f::Vec4;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0034_5570,
        spline_modifier_load_path,
        "SplineModifier_LoadPath",
    ),
    (
        0x0034_5638,
        spline_modifier_is_node_visible,
        "SplineModifier_IsNodeVisible",
    ),
];

/// Same-shard direct callees used above.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_NEXT_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_PREV_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_NODE_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILD_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_GET_RIDER_PREVIEW_2 as FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUTSYNC_IS_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_HEAP_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_ACTIVATE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_FIND_ENABLED_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED_3 as MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_GRID_MESH_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BASE_RESET_FX;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_CATEGORY_TITLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_BUY_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_EQUIP_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_GET_PATH_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::START_SCREEN_CTOR;
}

// ---------------------------------------------------------------------------
// Spline modifier.

/// `bool SplineModifier_IsNodeVisible(SplineNode *node)` (0x345638): build
/// the node's box `{min (+0x6c), 1; max (+0x78), 1}` on the stack, ask the
/// camera (graphics manager method 0x158) for its view data, then run the
/// visibility test (method 0x2e8 on the box and the view data); true unless
/// the test answered exactly 1.
pub fn spline_modifier_is_node_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0xb0);
        let gfx = Addr(r32(c, GP).wrapping_add(GFX_MAN)).lw(c);
        c.g[A2][0] = gfx;
        frame.save(c, S2, 0x80);
        let node = c.g[A0][0];
        c.g[V0][0] = node;
        frame.save(c, S3, 0x70);
        let (min, max, view) = (frame.at(c, 0), frame.at(c, 0x10), frame.at(c, 0x20));
        c.g[S2][0] = sx(view);
        frame.save_ra(c, 0x60);
        c.g[S3][0] = sx(max);
        frame.save(c, S0, 0xa0);
        c.g[A0][0] = sx(view);
        frame.save(c, S1, 0x90);
        let cam_vt = Addr((gfx as u32).wrapping_add(0x10d8)).lw(c);
        c.g[S1][0] = cam_vt;
        let n = Addr(node as u32);
        let corner = [
            n.off(0x6c).read_u32(c),
            n.off(0x70).read_u32(c),
            n.off(0x74).read_u32(c),
            n.off(0x78).read_u32(c),
            n.off(0x7c).read_u32(c),
            n.off(0x80).read_u32(c),
        ];
        c.f[..5].copy_from_slice(&corner[..5]);
        c.f[6] = corner[5];
        c.g[AT][0] = ONE as u64;
        c.f[5] = ONE;
        let test_delta = Addr((cam_vt as u32).wrapping_add(0x2e8)).lh(c);
        c.g[S0][0] = addu(gfx, test_delta);
        for (i, &w) in corner[..3].iter().enumerate() {
            Addr(min).off(4 * i as u32).write_u32(c, w);
            Addr(max).off(4 * i as u32).write_u32(c, corner[3 + i]);
        }
        Addr(min).off(0xc).write_u32(c, ONE);
        Addr(max).off(0xc).write_u32(c, ONE);
        // The camera's view data, into the buffer at sp+0x20.
        let vt = Addr((gfx as u32).wrapping_add(0x10d8)).lw(c);
        c.g[V0][0] = vt;
        let query = VSlot::read(c, vt, 0x158);
        c.g[A1][0] = query.this(gfx);
        c.g[V1][0] = query.func;
        call_ptr(c, query.func as u32, 0x0034_56c4)?;
        // test(this, box, max, view).
        let test = Addr((cam_vt as u32).wrapping_add(0x2ec)).lw(c);
        c.g[V0][0] = test;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, test as u32, 0x0034_56dc)?;
        let answer = c.g[V0][0] ^ 1;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        c.g[V0][0] = (answer != 0) as u64;
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Spline paths.

/// `void SplineModifier_LoadPath(SplineModifier *self, Stream *stream)`
/// (0x345570): let the stream load the modifier's path (`v02(self, 8)`),
/// resolve the first spline node, walk `count` nodes, and store the spline
/// length.
pub fn spline_modifier_load_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x8;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V1][0], 0x10)).lh(c);
        c.g[V0][0] = Addr(p(c.g[V1][0], 0x14)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_55a0)?;
        call_ptr(c, 0x002d_1bd8, 0x0034_55a8)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V1][0], 0x8)).lw(c);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
        if c.g[V1][0] != 0 {
            c.cycles += 7;
            c.g[A0][0] = Addr(p(c.g[V1][0], 0x44)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(p(c.g[V0][0], 0x24)).lw(c);
        c.g[A0][0] = Addr(p(c.g[S0][0], 0x4)).lw(c);
        let none = (c.g[A0][0] as i64) <= 0;
        Addr(p(c.g[S0][0], 0x8)).sw(c, c.g[V0][0]);
        if !none {
            c.cycles += 1;
            loop {
                c.cycles += 7;
                c.g[V1][0] = Addr(p(c.g[S0][0], 0x8)).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                c.g[V0][0] = Addr(p(c.g[V1][0], 0x64)).lw(c);
                let more = c.g[A0][0] != 0;
                Addr(p(c.g[S0][0], 0x8)).sw(c, c.g[V0][0]);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_5600);
                }
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        callee::SPLINE_GET_PATH_LENGTH.call(c, 0x0034_5624)?;
        Addr(p(c.g[S0][0], 0xc)).write_fpu(c, crate::ps2f::FpuF32(c.f[0]));
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::bx::bxstring2::tests::f;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W;
    pub(crate) use crate::game::object::movenode::tests::delta;
    pub(crate) use crate::game::object::movenode::tests::slot;
    use ssx3_ee as ee;

    // ------------------------------------------------------ spline modifier

    #[test]
    fn spline_modifier_is_node_visible_matches() {
        covered(
            ssx3_ee::s11::f_00345638__generated,
            spline_modifier_is_node_visible,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3456_3800, 2);
                let (gfx, cam_vt, node) = (DATA + 0x2000, DATA + 0x3000, DATA + 0x100);
                c.bus.write32(GAME_GP + GFX_MAN, gfx);
                c.bus.write32(gfx + 0x10d8, cam_vt);
                slot(c, cam_vt, 0x158, delta(&mut r), FAKE);
                slot(c, cam_vt, 0x2e8, delta(&mut r), FAKE + 0x10);
                fill(c, &mut r, node, 0x30);
                c.g[A0][0] = sx(node);
                // The test answers 1 (hidden) or a pointer.
                c.bus
                    .write32(LOG_RESULTS + 4, if seed % 2 == 0 { 1 } else { 0 });
            },
        );
    }

    #[test]
    fn spline_modifier_load_path_matches() {
        check(
            ee::s11::f_00345570__generated,
            spline_modifier_load_path,
            100,
            &|c, seed| {
                let _r = base(c, seed, 0x3455);
                c.g[A0][0] = DATA as u64;
                // The stream's `v02(self, 8)`.
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.bus.write32(DATA + 0x2000, DATA + 0x2100);
                c.bus.write16(DATA + 0x2110, 0);
                c.bus.write32(DATA + 0x2114, FAKE);
                // `World_GetInstance()` and the node table it returns.
                c.bus.write32(gp_at(0x16c8), DATA + 0x3000);
                c.bus.write32(DATA + 0x3008, DATA + 0x3100);
                let idx = seed % 4;
                c.bus.write32(DATA, idx | (seed % 3) << 8);
                c.bus.write32(DATA + 4, seed % 3);
                c.bus.write32(
                    DATA + 0x3100 + 4 * idx,
                    if seed % 2 == 0 { 0 } else { DATA + 0x3200 },
                );
                c.bus.write32(DATA + 0x3200 + 0x44, 0);
                c.bus.write32(DATA + 0x3200 + 0x24, DATA + 0x3300);
                c.bus.write32(DATA + 0x3300 + 0x68, DATA + 0x3400);
                c.bus.write32(DATA + 0x3300 + 0x64, DATA + 0x3300);
                c.bus.write32(DATA + 0x3400 + 0xc, f(1.5));
                c.bus.write32(DATA + 0x3400 + 0x84, f(2.5));
            },
        );
    }
}
