//! `src/render/bezierman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::Cpu;
use crate::events::back_edge_unwinds;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::fe::debugfe::at as at_2;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_b190, ps2_bezier_man_v22, "PS2BezierMan_v22"),
    (0x0038_b338, ps2bezier_man_v21, "PS2BezierMan_v21"),
    (0x0038_ca08, ps2bezier_man_test_patch_bounds, "PS2BezierMan_TestPatchBounds"),
    (0x0038_d638, ps2_bezier_man_call_v17, "PS2BezierMan_CallV17"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;


    /// `PS2GraphicsMan_CullAgainstLights`.
    pub const CULL_AGAINST_LIGHTS: Callee = Callee {
        addr: 0x0037_df88,
        f: ee::s12::PS2GraphicsMan_CullAgainstLights,
    };

    /// `float RaceProgress_ProjectRider(int i)`.
    pub const PROJECT_RIDER: Callee = Callee {
        addr: 0x0021_0820,
        f: ee::s05::RaceProgress_ProjectRider,
    };


    // Cross-shard calls the generator emitted as `call_indirect`.
    pub const RIDER_GET_DIST_TO_FINISH: u32 = 0x0011_3128;

    pub const RIDER_SPHERE_TREE_CTOR: u32 = 0x0032_9910;

    pub const RIDER_SPHERE_TREE_GET_BOUNDS: u32 = 0x0032_9dc8;

    pub const SND_LOCK: u32 = 0x003b_58a0;

    pub const SND_UNLOCK: u32 = 0x003b_58d8;

    pub const SND_STREAM_POSITION: u32 = 0x003b_6670;

    pub const SND_SUB_TIME: u32 = 0x003b_8ad8;

    pub const SND_SUB_STATUS: u32 = 0x003b_86c0;

    pub const SND_STREAM_INFO: u32 = 0x003b_7b48;

    pub const SND_SUB_STATE: u32 = 0x003b_7c40;

    pub const SND_STATUS: u32 = 0x003b_7f18;
}

/// `gp`-relative globals used here.
pub(crate) mod g {
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
}


// ---------------------------------------------------------------------------
// Register and memory helpers.

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}


/// A float's bit pattern (test fixtures).
#[inline(always)]
#[cfg(test)]
pub(crate) fn f(x: f32) -> u32 {
    x.to_bits()
}


// ---------------------------------------------------------------------------
// PS2BezierMan_v21.

/// `void PS2BezierMan_v21(PS2BezierMan* self, void* a, void* b, void* c)`
/// (0x38b338): the manager's entry, which forwards to its renderer (vtable
/// slot 22, a `{short delta; void (*fn)();}` entry at `*(self+4)+0xb0`)
/// unless the global at `0x4a44d4` disables it.
pub fn ps2bezier_man_v21(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw(gp(c, 0x13e4));
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[T2][0] = c.g[A0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 6;
            let vt = c.lw(self_.wrapping_add(4));
            c.g[V0][0] = vt;
            c.g[T1][0] = 0;
            c.g[A0][0] = Addr((vt as u32).wrapping_add(0xb0)).lh(c);
            let func = c.lw((vt as u32).wrapping_add(0xb4));
            c.g[V1][0] = func;
            c.g[A0][0] = sx(self_.wrapping_add(r32(c, A0)));
            call_ptr(c, func as u32, 0x0038_b364)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// PS2BezierMan_TestPatchBounds.

/// `void PS2BezierMan_TestPatchBounds(PS2BezierMan* self, Patch* patch)`
/// (0x38ca08): the patch's two corner points (+0x158 and +0x164, w = 1) are
/// handed to the graphics manager at `*(0x4a5b80)`'s light cull.
pub fn ps2bezier_man_test_patch_bounds(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let sp = sp(c);
        let self_ = r32(c, A1);
        c.g[V0][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = c.g[SP][0]; // a = first point
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 5); // f5 = 1.0
        c.g[A2][0] = sx(sp.wrapping_add(0x10)); // b = second point
        c.lwc1(6, self_.wrapping_add(0x16c));
        c.lwc1(4, self_.wrapping_add(0x158));
        c.lwc1(0, self_.wrapping_add(0x15c));
        c.lwc1(1, self_.wrapping_add(0x160));
        c.lwc1(2, self_.wrapping_add(0x164));
        c.lwc1(3, self_.wrapping_add(0x168));
        c.g[A0][0] = c.lw(gp(c, 0x2a90));
        c.swc1(4, sp);
        c.swc1(0, sp.wrapping_add(4));
        c.swc1(1, sp.wrapping_add(8));
        c.swc1(2, sp.wrapping_add(0x10));
        c.swc1(3, sp.wrapping_add(0x14));
        c.swc1(6, sp.wrapping_add(0x18));
        c.swc1(5, sp.wrapping_add(0x1c));
        c.g[RA][0] = sx(0x0038_ca64);
        c.swc1(5, sp.wrapping_add(0xc));
        callee::CULL_AGAINST_LIGHTS.call(c, 0x0038_ca64)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}


/// `void PS2BezierMan_CallV17(PS2BezierMan* self)` (0x38d638): call slot 0x11
/// (`+0x88` delta, `+0x8c` method) of the object at `self->+0x4`.
pub fn ps2_bezier_man_call_v17(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(abi::arg(c, 0), 4)).lw(c);
        let obj = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(at(obj, 0x88)).lh(c);
        c.g[A1][0] = Addr(at(obj, 0x8c)).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0038_d654)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void PS2BezierMan_v22(PS2BezierMan* self, void* obj, u8* types, void*
/// extra)` (0x38B190): queue a bezier patch. Four equal type bytes go to the
/// per-size list for that size (4, 6 or 8); a mixed patch goes to the mix list
/// with kind 2 when any type is 8, else 0.
pub fn ps2_bezier_man_v22(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = c.lbu(ea(c.g[A2][0], 0));
    c.g[V0][0] = c.lbu(ea(c.g[A2][0], 2));
    let mut uniform = c.g[A3][0] == c.g[V0][0];
    if uniform {
        c.cycles += 4;
        c.g[V1][0] = c.lbu(ea(c.g[A2][0], 1));
        c.g[V0][0] = c.lbu(ea(c.g[A2][0], 3));
        uniform = c.g[V1][0] == c.g[V0][0];
        if uniform {
            c.cycles += 2;
            uniform = c.g[A3][0] == c.g[V1][0];
            if !uniform {
                c.g[A3][0] = c.lw(ea(c.g[A0][0], 0x4d84));
            }
        } else {
            c.g[A3][0] = c.lw(ea(c.g[A0][0], 0x4d84));
        }
    } else {
        c.g[A3][0] = c.lw(ea(c.g[A0][0], 0x4d84));
    }
    if uniform {
        // One list per patch size: 4 (+0x458, 900 entries), 6 (+0x3c9c,
        // 220) and 8 (+0x4a60, 50); the entry is 16 bytes at the list base.
        c.cycles += 4;
        c.g[A2][0] = c.g[A3][0];
        c.g[V0][0] = 6;
        let is_six = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[A2][0] as i64) < 7) as u64;
        if is_six {
            c.cycles += 4;
            c.g[V1][0] = c.lw(ea(c.g[A0][0], 0x3c9c));
            c.g[V0][0] = ((c.g[V1][0] as i64) < 220) as u64;
            let full = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
            if full {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            c.cycles += 11;
            c.g[V1][0] = sx(ea(c.g[V1][0], 1));
            c.g[V0][0] = sx(ea(c.g[V0][0], 0x3ca0));
            c.sw(ea(c.g[A0][0], 0x3c9c), c.g[V1][0]);
            c.g[V0][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
            c.g[V1][0] = 1;
            c.sw(ea(c.g[V0][0], 4), c.g[T0][0]);
            c.sw(ea(c.g[V0][0], 8), c.g[V1][0]);
            c.sw(ea(c.g[V0][0], 0), c.g[A1][0]);
            c.g[V1][0] = c.lhu(ea(c.g[A1][0], 0x1a8));
            c.sh(ea(c.g[V0][0], 0xc), c.g[V1][0]);
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        let at_least_seven = c.g[V0][0] == 0;
        c.g[V0][0] = 4;
        if at_least_seven {
            c.cycles += 3;
            c.g[V0][0] = 8;
            if c.g[A2][0] == c.g[V0][0] {
                c.g[V1][0] = c.lw(ea(c.g[A0][0], 0x4a60));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 50) as u64;
                let full = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
                if full {
                    c.cycles += 2;
                    abi::ret(c);
                    return;
                }
                c.cycles += 11;
                c.g[V1][0] = sx(ea(c.g[V1][0], 1));
                c.g[V0][0] = sx(ea(c.g[V0][0], 0x4a64));
                c.sw(ea(c.g[A0][0], 0x4a60), c.g[V1][0]);
                c.g[V0][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
                c.g[V1][0] = 2;
                c.sw(ea(c.g[V0][0], 4), c.g[T0][0]);
                c.sw(ea(c.g[V0][0], 8), c.g[V1][0]);
                c.sw(ea(c.g[V0][0], 0), c.g[A1][0]);
                c.g[V1][0] = c.lhu(ea(c.g[A1][0], 0x1aa));
                c.sh(ea(c.g[V0][0], 0xc), c.g[V1][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        if c.g[A2][0] == c.g[V0][0] {
            c.g[V1][0] = c.lw(ea(c.g[A0][0], 0x458));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 900) as u64;
            let full = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
            if full {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            c.cycles += 10;
            c.g[V1][0] = sx(ea(c.g[V1][0], 1));
            c.g[V0][0] = sx(ea(c.g[V0][0], 0x45c));
            c.sw(ea(c.g[A0][0], 0x458), c.g[V1][0]);
            c.g[V0][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
            c.sw(ea(c.g[V0][0], 4), c.g[T0][0]);
            c.sw(ea(c.g[V0][0], 0), c.g[A1][0]);
            c.sw(ea(c.g[V0][0], 8), 0);
            c.g[V1][0] = c.lhu(ea(c.g[A1][0], 0x1a6));
            c.sh(ea(c.g[V0][0], 0xc), c.g[V1][0]);
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    // Mixed patch: capacity 75, kind 2 when any type is 8.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A3][0] as i64) < 75) as u64;
    let full = c.g[V0][0] == 0;
    c.g[V0][0] = 0x14;
    if full {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    let mut kind;
    c.cycles += 11;
    c.g[V1][0] = sx(ea(c.g[A3][0], 1));
    // `mult a3, v0` (the generator routes the LO product through v0).
    c.mult(0, V0, A3, V0);
    c.sw(ea(c.g[A0][0], 0x4d84), c.g[V1][0]);
    c.g[A3][0] = 8;
    c.g[V0][0] = sx(ea(c.g[V0][0], 0x4d88));
    c.g[V1][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
    c.sw(ea(c.g[V1][0], 4), c.g[T0][0]);
    c.sw(ea(c.g[V1][0], 0), c.g[A1][0]);
    c.g[V0][0] = c.lbu(ea(c.g[A2][0], 0));
    let is_eight0 = c.g[V0][0] == c.g[A3][0];
    c.g[A0][0] = sx(ea(c.g[V1][0], 8));
    if is_eight0 {
        c.cycles += 1;
        kind = 2;
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lbu(ea(c.g[A2][0], 1));
        let is_eight1 = c.g[V0][0] == c.g[A3][0];
        c.g[V0][0] = 2;
        if is_eight1 {
            kind = 2;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lbu(ea(c.g[A2][0], 2));
            let is_eight2 = c.g[V0][0] == c.g[A3][0];
            c.g[V0][0] = 2;
            if is_eight2 {
                kind = 2;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(ea(c.g[A2][0], 3));
                let not_eight3 = c.g[V0][0] != c.g[A3][0];
                c.g[V0][0] = 0;
                if not_eight3 {
                    kind = 0;
                } else {
                    c.cycles += 1;
                    kind = 2;
                }
            }
        }
    }
    c.g[V0][0] = kind;
    c.cycles += 5;
    c.sw(ea(c.g[A0][0], 0), c.g[V0][0]);
    c.g[T0][0] = sx(ea(c.g[V1][0], 0x10));
    c.g[A3][0] = 0;
    c.g[V0][0] = c.lhu(ea(c.g[A1][0], 0x1aa));
    c.sh(ea(c.g[V1][0], 0xc), c.g[V0][0]);
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx(ea(c.g[A2][0], c.g[A3][0] as u32));
        c.g[A0][0] = sx(ea(c.g[T0][0], c.g[A3][0] as u32));
        c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
        c.g[A3][0] = sx(ea(c.g[A3][0], 1));
        c.g[V0][0] = ((c.g[A3][0] as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.sb(ea(c.g[A0][0], 0), c.g[V1][0]);
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_b310);
        }
    }
}


/// `void PS2BezierMan_FreeCaches(PS2BezierMan* man)` (0x38b0f8): free the
/// renderer's three cache blocks.
pub fn ps2_bezier_man_free_caches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let man = c.g[A0][0];
        c.g[S0][0] = man;
        call_ptr(c, 0x0038_d660, 0x0038_b10c)?;
        c.g[A0][0] = Addr(at_2(c, S0, 0x440)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b120)?;
        }
        c.g[A0][0] = Addr(at_2(c, S0, 0x444)).lw(c);
        'second: {
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.g[A0][0] = Addr(at_2(c, S0, 0x448)).lw(c);
                break 'second;
            }
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b134)?;
            c.g[A0][0] = Addr(at_2(c, S0, 0x448)).lw(c);
        }
        'third: {
            c.cycles += 2;
            let none = c.g[A0][0] == 0;
            frame.restore(c, S0, 0x10);
            if none {
                break 'third;
            }
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b148)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::check_lohi;
    use ssx3_ee::s01;
    use ssx3_ee::s03;
    use ssx3_ee::s00;
    use ssx3_ee::s02;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::be::beintoption::tests::VT;
    pub(crate) use crate::game::be::beintoption::tests::slot;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::object::railmodifier::tests::h;
    pub(crate) use crate::game::be::bestate2::tests::belib;


    // PS2BezierMan_v21 -----------------------------------------------------

    pub(crate) const BEZIER: u32 = DATA;


    #[test]
    fn ps2bezier_man_v21_matches() {
        covered(
            ssx3_ee::s12::f_0038b338__generated,
            ps2bezier_man_v21,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38b338, 1);
                c.g[A0][0] = BEZIER as u64;
                c.bus
                    .write32(GAME_GP + 0x13e4, if seed % 3 == 0 { 0 } else { 1 });
                let vt = vtable(c, &mut r, &[0xb0]);
                c.bus.write32(BEZIER + 4, vt);
            },
        );
    }


    // PS2BezierMan_TestPatchBounds ----------------------------------------

    pub(crate) const PATCH: u32 = DATA + 0x2000;

    pub(crate) const GMAN: u32 = DATA + 0x4000;

    pub(crate) const LIGHT_VT: u32 = DATA + 0x5000;


    #[test]
    fn ps2bezier_man_test_patch_bounds_matches() {
        covered(
            ssx3_ee::s12::f_0038ca08__generated,
            ps2bezier_man_test_patch_bounds,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38ca08, 1);
                c.g[A1][0] = PATCH as u64;
                c.bus.write32(GAME_GP + 0x2a90, GMAN);
                for k in 0..7 {
                    c.bus.write32(PATCH + 0x158 + 4 * k, r.edge_f32());
                }
                // The manager's light list: empty half the time, else one item
                // whose method is a fake.
                let count = (seed % 2) as u32;
                c.bus.write32(GMAN + 0x13ec, count);
                if count != 0 {
                    c.bus.write32(GMAN + 0x10d8, LIGHT_VT);
                    c.bus.write16(LIGHT_VT + 0x2e8, 0);
                    c.bus.write32(LIGHT_VT + 0x2ec, FAKE);
                }
            },
        );
    }


    #[test]
    fn ps2_bezier_man_call_v17_matches() {
        covered(s12::f_0038d638__generated, ps2_bezier_man_call_v17, 120, &|c, seed| {
            machine(c, seed, 0x38d638, 1);
            clear(c, OBJ, 0x40);
            clear(c, VT, 0x200);
            slot(c, VT, 0x88);
            c.bus.write32(OBJ + 4, VT);
            c.g[A0][0] = sx(OBJ);
            let _ = seed;
        });
    }


    pub(crate) fn b(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }


    #[test]
    fn ps2_bezier_man_v22_matches() {
        check(ee::s12::f_0038b190__generated, ps2_bezier_man_v22, 100, &|c, seed| {
            machine(c, seed, 0x0038_b190, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x1000) as u64;
            c.g[A2][0] = (DATA + 0x2000) as u64;
            c.g[T0][0] = (DATA + 0x3000) as u64;
            let types: [u8; 4] = match seed % 6 {
                0 => [4, 4, 4, 4],
                1 => [6, 6, 6, 6],
                2 => [8, 8, 8, 8],
                3 => [1, 2, 8, 3],
                4 => [1, 2, 3, 4],
                _ => [2, 2, 2, 2],
            };
            for (i, t) in types.into_iter().enumerate() {
                b(c, DATA + 0x2000 + i as u32, t);
            }
            h(c, DATA + 0x1000 + 0x1a6, 0x11);
            h(c, DATA + 0x1000 + 0x1a8, 0x22);
            h(c, DATA + 0x1000 + 0x1aa, 0x33);
            match seed % 4 {
                1 => w(c, DATA + 0x3c9c, 220), // 6-list full
                2 => {
                    w(c, DATA + 0x458, 900); // 4-list full
                    w(c, DATA + 0x4d84, 75); // mix list full
                }
                3 => {
                    w(c, DATA + 0x4a60, 50); // 8-list full
                    w(c, DATA + 0x3c9c, 220);
                }
                _ => {}
            }
        });
    }
}
