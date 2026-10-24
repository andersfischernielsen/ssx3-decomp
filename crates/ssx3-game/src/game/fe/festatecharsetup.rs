//! `src/fe/festatecharsetup.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::gp;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_ACTIVATED;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
pub(crate) use crate::game::wscript::wscriptman::p;
pub(crate) use crate::game::wscript::wscriptman::sw;
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
    (
        0x0018_2c38,
        cfe_state_char_setup_update,
        "cFEStateCharSetup_Update",
    ),
    (
        0x0018_2c68,
        fe_state_rider_detail_on_widget_notify,
        "cFEStateRiderDetail_OnWidgetNotify",
    ),
    (
        0x0018_31d0,
        cfe_state_char_setup_set_rider_preview_active,
        "cFEStateCharSetup_SetRiderPreviewActive",
    ),
    (
        0x0018_3238,
        c_fe_state_rider_detail_ctor,
        "cFEStateRiderDetail_ctor",
    ),
    (0x0031_1a50, anim_controller_reset, "AnimController_Reset"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_SAVE_DATA_SIZE;
    pub(crate) use crate::game::fe::festateload::callee::BOLT_FILE_CACHE_INIT;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SELECT_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::EA_MEM_FREE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_UPDATE;
    pub(crate) use crate::game::fe::festateload::callee::I_SIGNAL_SEMA;
    pub(crate) use crate::game::fe::festateload::callee::MEM_CARD_BASE_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V42;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V43;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_HANDLE_EVENT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_METRIX_SET_SPIN_AMOUNT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_PREVIEW_APPLY_TRANSFORM;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_RESUME;
    pub(crate) use crate::game::fe::festateload::callee::STRLEN_COPY;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_TRIGGER_MANAGER_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_CACHE_SET_DATA;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_GOTO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_JUMP_TO_TASK;
}

/// `gp`-relative globals used here.
pub(crate) mod g {
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
}

/// `void cFEStateCharSetup_Update(cFEStateCharSetup* self)` (0x182c38): the
/// base state's update, then the setup state's rider-preview update.
pub fn cfe_state_char_setup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_V13, 0x0018_2c4c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW.call(c, 0x0018_2c54)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int cFEStateRiderDetail_OnWidgetNotify(...)` (0x182C68): the `Ok` widget
/// is 9.
pub fn fe_state_rider_detail_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] ^= 9;
    c.g[V1][0] = WIDGET_ACTIVATED;
    c.g[V0][0] = WIDGET_IGNORED;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `void cFEStateCharSetup_SetRiderPreviewActive(self, int on)` (0x1831d0):
/// look the rider up in the game's rider manager and toggle the rider
/// preview.
pub fn cfe_state_char_setup_set_rider_preview_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        open(c, 0x30);
        c.g[V0][0] = c.lw(gp(c, -0x848));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        let no_mgr = c.g[A0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !no_mgr {
            c.cycles += 5;
            c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
            c.g[RA][0] = sx(0x0018_3200);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a_0548, 0x0018_3200)?;
            if c.g[S0][0] != 0 {
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[S0][0]);
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0018_3214);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0019_e538, 0x0018_3214)?;
                let still = c.g[S0][0] != 0;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[S0][0]);
                if !still {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                }
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `void* cFEStateRiderDetail_ctor(cFEStateRiderDetail* self, void* a1, int
/// index)` (0x183238): construct the char-setup base state and fill in the
/// rider-detail class and pad mask.
pub fn c_fe_state_rider_detail_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0018_28c0, 0x0018_325c)?;
        c.g[V0][0] = 0xd;
        c.g[V1][0] = 0x46cd_90;
        Addr(at(c, S0, 0xc)).sw(c, c.g[V0][0]);
        Addr(at(c, S0, 8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        Addr(at(c, S0, 0x44)).sb(c, c.g[S1][0]);
        c.g[V0][0] = Addr(at(c, S0, 0x10)).lw(c);
        c.g[A0][0] = Addr(at(c, V0, 0)).lw(c);
        call_ptr(c, 0x001a_1cd0, 0x0018_3284)?;
        Addr(at(c, S0, 0x15)).sb(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c, S0, 0x4c)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Animation controller.

/// `void AnimController_Reset(AnimController* self)` (0x311a50): clear the six
/// animation sequence lists at `+0x50` (8 bytes each), set each current
/// animation id (`+0..+0x14`) to 0x1b6, and restore the defaults: `+0x18` = 0,
/// `+0x1c` = 1.0, `+0x20` = -1 and the two 16-byte constants at `0x4ff230`
/// copied to `+0x30`/`+0x40`.
pub fn anim_controller_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[S1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0x1b6;
        loop {
            c.cycles += 9;
            c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x50));
            c.g[A0][0] = sx((c.g[S2][0] as u32) << 3);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, 0x0031_4668, 0x0031_1a8c)?; // AnimSeqList_Clear
            sw(c, c.g[S0][0] as u32, c.g[S3][0]);
            c.g[V0][0] = ((c.g[S2][0] as i64) < 6) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_1a78);
                }
                continue;
            }
            break;
        }
        c.cycles += 19;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = 0x0050_0000;
        sw(c, p(c.g[S1][0] as u32, 0x18), 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xdd0));
        c.swc1(0, p(c.g[S1][0] as u32, 0x1c));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        frame.restore(c, S0, 0x40);
        c.lq(2, c.g[A0][0] as u32);
        frame.restore(c, S2, 0x20);
        c.sq(2, p(c.g[S1][0] as u32, 0x30));
        frame.restore(c, S3, 0x10);
        c.lq(3, p(c.g[A0][0] as u32, 0x10));
        Addr(p(c.g[S1][0] as u32, 0x20)).sd(c, c.g[A1][0]);
        c.sq(3, p(c.g[S1][0] as u32, 0x40));
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD;
    use ssx3_ee as ee;
    use ssx3_ee::s00;
    use ssx3_ee::s01;
    use ssx3_ee::s02;
    use ssx3_ee::s03;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn cfe_state_char_setup_update_matches() {
        check(
            s02::f_00182c38__generated,
            cfe_state_char_setup_update,
            120,
            &|c, seed| {
                machine(c, seed, 0x182c38, 2);
                globals(c);
                clear(c, OBJ, 0x100);
                c.g[A0][0] = sx(OBJ);
                let _ = seed;
            },
        );
    }

    #[test]
    fn fe_state_rider_detail_on_widget_notify_matches() {
        check(
            ee::s02::f_00182c68__generated,
            fe_state_rider_detail_on_widget_notify,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0018_2c68, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 9,
                    1 => 0,
                    _ => seed as u64,
                };
            },
        );
    }

    #[test]
    fn cfe_state_char_setup_set_rider_preview_active_matches() {
        check(
            ee::s02::f_001831d0__generated,
            cfe_state_char_setup_set_rider_preview_active,
            40,
            &|c, seed| {
                base(c, seed, 0x18_31d0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x100) };
                // The rider lookup leaves v0 = a2 + module; keep a2 small so the
                // preview object stays inside the test data.
                c.g[A2][0] = 0;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
                c.bus.write32(
                    DATA + 0x1000 + 0x7c,
                    if seed % 4 == 0 { 0 } else { DATA + 0x1100 },
                );
                c.bus.write8(DATA + 0x44, (seed % 4) as u8);
                c.bus.write32(DATA + 0x48, 0xdead);
                c.bus.write32(DATA + 0x4c, 0xdead);
            },
        );
    }

    #[test]
    fn c_fe_state_rider_detail_ctor_matches() {
        check(
            ee::s02::f_00183238__generated,
            c_fe_state_rider_detail_ctor,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1832_38);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (1 + seed % 4) as u64;
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    // --------------------------------------------------- animation controller

    #[test]
    fn anim_controller_reset_matches() {
        check(
            ssx3_ee::s10::f_00311a50__generated,
            anim_controller_reset,
            12,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x0031_1a50);
                let anim = DATA + 0x100;
                let lists = if seed % 7 == 0 {
                    DATA + 0x2000
                } else {
                    DATA + 0x2000
                };
                clear_2(c, anim, 0x60);
                clear_2(c, lists, 0x30);
                w(c, anim + 0x50, lists);
                c.g[A0][0] = sx(anim);
                for off in [0u32, 0x18, 0x1c, 0x20, 0x30, 0x40] {
                    w(c, anim + off, r.next_u32());
                }
            },
        );
    }
}
