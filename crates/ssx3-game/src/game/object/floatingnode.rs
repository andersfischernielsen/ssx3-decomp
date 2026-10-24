//! `src/object/floatingnode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::be::bestate2::gp_lw;
pub(crate) use crate::game::be::bestate2::gp_rel;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::sound::soundsys::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0034_ef08,
        floating_free_bound_box,
        "Floating_FreeBoundBox",
    ),
    (0x0034_f048, floating_v49, "Floating_v49"),
    (0x0034_f930, floating_v15, "Floating_v15"),
    (0x0034_fa88, floating_save, "Floating_Save"),
];

// ---------------------------------------------------------------------------
// Intra-shard callees the generator called directly (its `jal` aliases).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
}

/// `__builtin_delete` (0x317e50).
pub(crate) const DELETE: u32 = 0x0031_7e50;

/// `void Floating_FreeBoundBox(this)` (0x34ef08): delete the bounding box at
/// `+0x78` and clear the pointer.
pub fn floating_free_bound_box(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A0][0] = Addr((this as u32).wrapping_add(0x78)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, DELETE, 0x0034_ef2c)?;
            c.sw((this as u32).wrapping_add(0x78), 0);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `lwc1 fN, off(base)`.
#[inline(always)]
pub(crate) fn ldf_at(c: &mut Cpu, n: usize, base: u64, off: u32) {
    c.lwc1(n, ea(base, off));
}

/// `swc1 fN, off(base)`.
#[inline(always)]
pub(crate) fn stf_at(c: &mut Cpu, n: usize, base: u64, off: u32) {
    c.swc1(n, ea(base, off));
}

// ---------------------------------------------------------------------------
// Comparators and predicates.

/// `float Floating_v15(Floating* self)` (0x34F930): one step of the
/// three-axis damped spring used by `sFloatTilt` (board float), then clamp the
/// lean angles to +-80 degrees and clear the inputs.
pub fn floating_v15(c: &mut Cpu) {
    c.cycles += 68;
    let a0 = A0;
    ldf_at(c, 14, c.g[a0][0], 0x50); // pos1
    c.g[V1][0] = sx(ea(c.g[a0][0], 0x2c));
    ldf_at(c, 2, c.g[a0][0], 0x2c); // k
    ldf_at(c, 0, c.g[a0][0], 0x4c); // rest1
    ldf_at(c, 1, c.g[a0][0], 0x38); // input1
    c.neg_s(2, 2);
    c.sub_s(0, 14, 0);
    ldf_at(c, 13, c.g[a0][0], 0x64); // vel2
    c.neg_s(1, 1);
    ldf_at(c, 12, c.g[a0][0], 0x54); // vel1
    ldf_at(c, 7, c.g[a0][0], 0x40); // gain
    ldf_at(c, 3, c.g[a0][0], 0x60); // k2
    c.mul_s(2, 2, 0);
    c.mul_s(5, 1, 13);
    ldf_at(c, 4, c.g[a0][0], 0x3c); // damp
    c.g[V0][0] = gp_lw(c, 0x2a74); // the frame counter object
    c.mul_s(3, 7, 3);
    ldf_at(c, 9, c.g[a0][0], 0x44); // input2
    c.mul_s(1, 1, 12);
    ldf_at(c, 0, c.g[a0][0], 0x70); // pos3
    c.neg_s(4, 4);
    ldf_at(c, 8, c.g[a0][0], 0x34); // pos2
    ldf_at(c, 15, c.g[V0][0], 0x10); // frame delta
    c.cvt_s_w(15, 15);
    ldf_at(c, 6, c.g[a0][0], 0x30); // damp2
    c.mul_s(7, 7, 0);
    ldf_at(c, 11, c.g[a0][0], 0x6c); // vel3
    c.mul_s(8, 8, 9);
    c.neg_s(6, 6);
    ldf_at(c, 9, c.g[a0][0], 0x48); // input3
    ldf_at(c, 10, c.g[a0][0], 0x5c); // rest2
    c.add_s(1, 1, 3);
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 0);
    c.mul_s(3, 4, 11);
    c.mul_s(4, 4, 10);
    c.div_s(0, 0, 15); // dt = 1 / fps
    c.add_s(2, 2, 8);
    c.add_s(5, 5, 7);
    c.lwc1(8, gp_rel(c, -0x2a98)); // +80 degrees
    c.mul_s(6, 6, 9);
    c.lwc1(7, gp_rel(c, -0x2a94)); // -80 degrees
    c.add_s(1, 1, 4);
    c.add_s(5, 5, 3);
    c.add_s(2, 2, 6);
    c.mul_s(3, 9, 0);
    c.mul_s(4, 10, 0);
    c.mul_s(2, 2, 0);
    c.mul_s(5, 5, 0);
    c.mul_s(1, 1, 0);
    c.mul_s(0, 11, 0);
    c.add_s(14, 14, 3);
    c.add_s(9, 9, 2);
    c.add_s(13, 13, 0);
    c.add_s(12, 12, 4);
    stf_at(c, 14, c.g[a0][0], 0x50);
    c.add_s(10, 10, 1);
    stf_at(c, 9, c.g[a0][0], 0x48);
    c.add_s(11, 11, 5);
    stf_at(c, 13, c.g[a0][0], 0x64);
    stf_at(c, 12, c.g[a0][0], 0x54);
    stf_at(c, 10, c.g[a0][0], 0x5c);
    stf_at(c, 11, c.g[a0][0], 0x6c);
    ldf_at(c, 0, c.g[V1][0], 0x28);
    c.c_le_s(7, 0);
    let keep = !c.fcc();
    c.mov_s(15, 7);
    if !keep {
        c.cycles += 1;
        c.min_s(15, 0, 8);
    }
    stf_at(c, 15, c.g[a0][0], 0x54);
    c.cycles += 6;
    ldf_at(c, 0, c.g[V1][0], 0x38);
    c.c_le_s(7, 0);
    let keep = !c.fcc();
    c.mov_s(1, 7);
    if !keep {
        c.cycles += 1;
        c.min_s(1, 0, 8);
    }
    c.cycles += 9;
    stf_at(c, 1, c.g[a0][0], 0x64);
    c.g[V1][0] = c.lhu(ea(c.g[a0][0], 0x12));
    c.g[V0][0] = 1;
    c.g[V1][0] |= 1;
    c.sw(ea(c.g[a0][0], 0x44), 0);
    c.sh(ea(c.g[a0][0], 0x12), c.g[V1][0]);
    c.sw(ea(c.g[a0][0], 0x60), 0);
    c.sw(ea(c.g[a0][0], 0x70), 0);
    abi::ret(c);
}

/// `func_0034FA88` (0x34fa88): `Floating_Save`, the floating-modifier
/// serialiser. Saves the modifier object (0x356b30), writes a 4-byte flag
/// (`this->+0x78 > 0`) plus the base pointer from the stack, then the 0x48-byte
/// block at `this+0x2c` through the archive writer.
pub fn floating_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x0034_faa4);
        c.g[S1][0] = c.g[A0][0];
        callee::MOD_OBJ_SAVE.call(c, 0x0034_faa4)?;
        c.g[V1][0] = lw_r(c, S0, 0);
        c.g[A1][0] = c.g[SP][0];
        c.g[V0][0] = lw_r(c, S1, 0x78);
        c.g[A2][0] = 0x4;
        c.g[A0][0] = lh_r(c, V1, 0x8);
        c.g[A3][0] = lw_r(c, V1, 0xc);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        let target = low(c, A3);
        c.g[RA][0] = sx(0x0034_facc);
        sw_r(c, SP, 0, c.g[V0][0]);
        call_ptr(c, target, 0x0034_facc)?;
        c.g[V1][0] = lw_r(c, S0, 0);
        c.g[A1][0] = sx(low(c, S1).wrapping_add(0x2c));
        c.g[A2][0] = 0x48;
        c.g[A0][0] = lh_r(c, V1, 0x8);
        c.g[V0][0] = lw_r(c, V1, 0xc);
        let target = low(c, V0);
        c.g[RA][0] = sx(0x0034_fae8);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0034_fae8)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Floating modifier.

/// `void Floating_v49(Floating* self)` (0x34f048): refresh the bound box from
/// the instance's min/max and notify the collision system, or run the base
/// update when there is no box info.
pub fn floating_v49(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S0, 0x70);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x50);
        c.g[A1][0] = Addr(p(r32(c, S1), 0x78)).lw(c);
        let no_info = c.g[A1][0] == 0;
        if no_info {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0035_67e0, 0x0034_f10c)?;
            c.lq(S0, frame.at(c, 0x70));
        } else {
            c.cycles += 39;
            c.lq(2, r32(c, A1));
            c.g[V1][0] = Addr(p(r32(c, S1), 0x18)).lw(c);
            c.sq(2, p(r32(c, SP), 0x20));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 2);
            c.lq(2, p(r32(c, A1), 0x10));
            c.sq(2, p(r32(c, SP), 0x30));
            c.lwc1(0, p(r32(c, V1), 0x60));
            c.swc1(0, p(r32(c, SP), 0x40));
            c.lwc1(1, p(r32(c, V1), 0x64));
            c.swc1(1, p(r32(c, SP), 0x44));
            c.lwc1(0, p(r32(c, V1), 0x68));
            c.swc1(2, p(r32(c, SP), 0x4c));
            c.swc1(0, p(r32(c, SP), 0x48));
            c.lq(4, p(r32(c, SP), 0x40));
            c.sq(4, r32(c, SP));
            c.lwc1(0, p(r32(c, V1), 0x6c));
            c.swc1(0, p(r32(c, SP), 0x40));
            c.lwc1(1, p(r32(c, V1), 0x70));
            c.swc1(1, p(r32(c, SP), 0x44));
            c.lwc1(0, p(r32(c, V1), 0x74));
            c.swc1(0, p(r32(c, SP), 0x48));
            c.lq(2, p(r32(c, SP), 0x40));
            c.sq(2, p(r32(c, SP), 0x10));
            c.sq(4, r32(c, A1));
            c.lq(2, p(r32(c, SP), 0x10));
            c.sq(2, p(r32(c, A1), 0x10));
            c.g[S0][0] = Addr(p(r32(c, S1), 0x18)).lw(c);
            call_ptr(c, 0x002d_1be0, 0x0034_f0dc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A3][0] = c.g[SP][0];
            c.g[T0][0] = sx(p(r32(c, SP), 0x20));
            call_ptr(c, 0x0032_91e0, 0x0034_f0f4)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0034_ef08, 0x0034_f0fc)?;
            c.lq(S0, frame.at(c, 0x70));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x60);
        frame.restore_ra(c, 0x50);
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
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_vec;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::camera::trigger::cameratriggerlist::tests::small_pointer;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS;
    pub(crate) use crate::game::object::animnode::tests::fb;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool;
    pub(crate) use crate::game::sound::soundsys::tests::wf;
    pub(crate) use crate::game::sound::soundsys::tests::BE;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD;
    use ssx3_ee as ee;

    #[test]
    fn floating_free_bound_box_matches() {
        check(
            ssx3_ee::s11::f_0034ef08__generated,
            floating_free_bound_box,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34ef_08, 0);
                c.g[A0][0] = DATA as u64;
                let p = if seed % 3 == 0 {
                    0
                } else {
                    small_pointer(seed)
                };
                c.bus.write32(DATA + 0x78, p);
                c.bus.write32(DATA + 0xc, r.next_u32());
            },
        );
    }

    #[test]
    fn floating_v15_matches() {
        check(
            ee::s11::f_0034f930__generated,
            floating_v15,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_f930, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                for off in [
                    0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x5c, 0x60,
                    0x64, 0x6c, 0x70,
                ] {
                    w(c, DATA + off, r.edge_f32());
                }
                w(c, GAME_GP + 0x2a74, DATA + 0x900);
                w(c, DATA + 0x910, 24 + (seed % 6));
                w(c, GAME_GP - 0x2a98, fb(1.396_263_5));
                w(c, GAME_GP - 0x2a94, fb(-1.396_263_5));
            },
        );
    }

    #[test]
    fn floating_save_matches() {
        check(
            ee::s11::f_0034fa88__generated,
            floating_save,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x34fa88, 4);
                be_library(c);
                world(c);
                let this = DATA;
                let arch = DATA + 0x400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = arch as u64;
                w(c, arch, DATA + 0x2000);
                slot(c, DATA + 0x2000, 8, 0, FAKE);
                w(c, this + 0x78, (seed % 3) as u32);
                w(c, this + 0x70, DATA + 0x900);
            },
        );
    }

    #[test]
    fn floating_v49_matches() {
        check(
            ee::s11::f_0034f048__generated,
            floating_v49,
            12,
            &|c, seed| {
                let _ = base(c, seed, 0x34f048, 2);
                let this = DATA;
                clear(c, this, 0x400);
                c.g[A0][0] = this as u64;
                w(c, this + 0x78, if seed % 3 == 0 { 0 } else { DATA + 0x200 });
                w(c, this + 0x18, DATA + 0x300);
                let info = DATA + 0x200;
                clear(c, info, 0x100);
                for k in 0..8u32 {
                    wf(c, info + 4 * k, 1.0 + k as f32);
                }
                let inst = DATA + 0x300;
                clear(c, inst, 0x100);
                for k in 0..8u32 {
                    wf(c, inst + 0x60 + 4 * k, 0.25 * k as f32);
                }
            },
        );
    }
}
