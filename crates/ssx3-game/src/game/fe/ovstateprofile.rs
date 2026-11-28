//! `src/fe/ovstateprofile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintscore::gp;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::festaterewards::lw_3;
pub(crate) use crate::game::fe::ovstatepause::close_top_state_once;
pub(crate) use crate::game::fe::ovstatepause::oc1013_screen_dtor;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::fe::ovstatetrophy::oc1015_dtor;
pub(crate) use crate::game::fe::ovstatetrophy::OC1015_DATA;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw as lw_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::movie::movieplayer::at as at_3;
use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0021_11a0,
        ov_screen_profile_save_pda_init,
        "OVScreenProfileSavePDA_Init",
    ),
    (
        0x0021_1220,
        ov_screen_profile_save_pda_v26,
        "OVScreenProfileSavePDA_v26",
    ),
    (
        0x0021_1970,
        ov_screen_profile_save_pda_v38,
        "OVScreenProfileSavePDA_v38",
    ),
    (
        0x0021_1aa8,
        ov_screen_profile_save_pda_v33,
        "OVScreenProfileSavePDA_v33",
    ),
    (
        0x0021_2080,
        ov_screen_profile_save_pda_init_slot_labels,
        "OVScreenProfileSavePDA_InitSlotLabels",
    ),
    (
        0x0021_2138,
        ov_screen_autosave_ctor,
        "OVScreenAutosave_ctor",
    ),
    (
        0x0021_2198,
        ov_screen_autosave_init,
        "OVScreenAutosave_Init",
    ),
    (0x0021_22e8, ov_screen_autosave_v26, "OVScreenAutosave_v26"),
    (0x0021_24c8, ov_screen_autosave_v33, "OVScreenAutosave_v33"),
    (
        0x0021_1270,
        ov_screen_profile_save_pda_reset_slot_list,
        "OVScreenProfileSavePDA_ResetSlotList",
    ),
    (
        0x0021_1380,
        ov_screen_profile_save_pda_refresh_card_info,
        "OVScreenProfileSavePDA_RefreshCardInfo",
    ),
    (
        0x0021_1850,
        ov_state_profile_hide_action_buttons,
        "OVScreenProfileSavePDA_v37",
    ),
    (
        0x0021_1a08,
        ov_state_profile_hide_main_button,
        "OVScreenProfileSavePDA_v39",
    ),
    (
        0x0021_2360,
        ov_screen_autosave_update_icon,
        "OVScreenAutosave_UpdateIcon",
    ),
    (
        0x0021_2680,
        ov_screen_debug_printf_stub,
        "OVScreen_DebugPrintfStub",
    ),
    (
        0x0022_1d88,
        ov_screen_profile_save_pda_dtor,
        "OVScreenProfileSavePDA_dtor",
    ),
    (
        0x0022_1db0,
        ov_screen_profile_save_pda_ctor_2,
        "OVScreenProfileSavePDA_ctor_2",
    ),
    (
        0x0022_2280,
        ov_screen_autosave_dtor,
        "OVScreenAutosave_dtor",
    ),
    (
        0x0022_22b8,
        ov_screen_autosave_ctor_2,
        "OVScreenAutosave_ctor_2",
    ),
    (
        0x0021_1b10,
        ov_screen_profile_save_pda_shutdown_2,
        "OVScreenProfileSavePDA_Shutdown",
    ),
    (
        0x0021_1b38,
        ov_screen_profile_save_pda_on_widget_notify_2,
        "OVScreenProfileSavePDA_OnWidgetNotify",
    ),
    (
        0x0021_1bc8,
        ov_screen_profile_save_pda_on_widget_action_2,
        "OVScreenProfileSavePDA_OnWidgetAction",
    ),
    (
        0x0021_2208,
        ov_screen_autosave_update_2,
        "OVScreenAutosave_Update",
    ),
    (
        0x0021_2508,
        ov_screen_autosave_shutdown_2,
        "OVScreenAutosave_Shutdown",
    ),
    (
        0x0021_22e0,
        ov_screen_autosave_handle_input_002122e0,
        "OVScreenAutosave_HandleInput",
    ),
    (0x0021_2788, ov_list_clear, "OVList_Clear"),
    (
        0x0022_22a8,
        ov_screen_autosave_on_widget_action_002222a8,
        "OVScreenAutosave_OnWidgetAction",
    ),
    (
        0x0022_22b0,
        ov_screen_autosave_v37_002222b0,
        "OVScreenAutosave_v37",
    ),
];

/// Direct and cross-shard callees, by guest address.
pub(crate) mod callee {
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET as MC_MANAGER_GET_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_SCREEN_FIND_OBJECT;
}

/// `void OVScreenAutosave_v33(Screen* s)` (0x2124c8).
pub fn ov_screen_autosave_v33(c: &mut Cpu) {
    close_top_state_once(c, 0x0021_24f0)
}

// ---------------------------------------------------------------------------
// Front end / profile save.

/// `void OVScreenProfileSavePDA_v26(OVScreenProfileSavePDA* self)`
/// (0x211220): flag the pending info write and refresh the slot list (or the
/// card info when the screen sits in mode 3).
pub fn ov_screen_profile_save_pda_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V0][0] = Addr(this).off(0x1c0).lw(c);
        let has_info = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if has_info {
            c.cycles += 1;
            Addr(this).off(0x22c).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(this).off(0x214).lw(c);
        c.g[V0][0] = 3;
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 3;
            callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST.call(c, 0x0021_1260)?;
        } else {
            c.cycles += 4;
            callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO.call(c, 0x0021_1250)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenAutosave_ctor(OVScreenAutosave* self, int a1, void* a2)`
/// (0x212138): construct the autosave overlay widget.
pub fn ov_screen_autosave_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = c.g[A0][0];
        let a2 = c.g[A2][0];
        c.g[S0][0] = self_;
        c.g[S1][0] = a2;
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        call_ptr(c, 0x001d_53b0, 0x0021_2158)?;
        c.g[V1][0] = 0x471_f80;
        c.g[A1][0] = 0x25;
        c.g[V1][0] = sx(0x0047_1f80);
        c.g[A0][0] = 0x1;
        Addr(at(c, S0, 0x22c)).sw(c, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c, S0, 0x8)).sw(c, c.g[V1][0]);
        Addr(at(c, S0, 0xc)).sw(c, c.g[A1][0]);
        Addr(at(c, S0, 0x208)).sw(c, c.g[A0][0]);
        Addr(at(c, S0, 0x1c4)).sw(c, 0);
        Addr(at(c, S0, 0x230)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenProfileSavePDA_v33(OVScreenProfileSavePDA* self)` (0x211aa8):
/// leave the top profile state (or quit the game module when the flow is at
/// state 0x18), then flag the screen as done.
pub fn ov_screen_profile_save_pda_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = lw_2(c, p(this as u32, 0x1ac));
        let done = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !done {
            c.cycles += 4;
            c.g[V1][0] = lw_2(c, p(this as u32, 0x278));
            c.g[V0][0] = 0x18;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A0][0] = lw_2(c, p(this as u32, 0x10));
            } else {
                c.cycles += 6;
                c.g[V0][0] = lw_2(c, gp(c, -0x848));
                c.g[A1][0] = 0;
                c.g[A0][0] = lw_2(c, p(c.g[V0][0] as u32, 0x84));
                call_ptr(c, 0x0023_0488, 0x0021_1ae4)?;
                c.g[V0][0] = 1;
                c.cycles += 5;
                frame.restore_ra(c, 0);
                sw(c, p(this as u32, 0x1ac), c.g[V0][0]);
                frame.restore(c, S0, 0x10);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f718, 0x0021_1af4)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        sw(c, p(this as u32, 0x1ac), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenAutosave_Init(OVScreenAutosave* self)` (0x212198): the same
/// initialisation with the autosave screen's hash name.
pub fn ov_screen_autosave_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x471e_28;
        c.g[S0][0] = Addr(at_2(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0021_21bc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0021_21d0)?;
        let t = c.g[V0][0] == 0;
        Addr(at_2(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0021_21e8)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x0021_21f0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_002122E8` (0x2122e8): `OVScreenAutosave`. When the autosave state at
/// `+0x1c0` is 0 or 6, looks up the widget named `0x471e38`, finds it
/// (`0x39d860(this->+0x40, hash)`) and calls its `vtable[0x48]` with 0.
/// Any other state just refreshes the icon (`0x212360`).
/// Signature: `void update(this)`.
pub fn ov_screen_autosave_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = lw_r(c, S0, 0x1c0);
        let zero = c.g[V1][0] == 0;
        c.g[V0][0] = 0x6;
        if zero {
            c.cycles += 1;
            c.g[A0][0] = 0x47_0000;
        } else {
            c.cycles += 2;
            if c.g[V1][0] == c.g[V0][0] {
                c.g[A0][0] = 0x47_0000;
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0021_2314);
                callee::AUTOSAVE_UPDATE_ICON.call(c, 0x0021_2314)?;
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 12;
        c.g[RA][0] = sx(0x0021_2328);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x1e38));
        call_ptr(c, 0x0031_7670, 0x0021_2328)?;
        c.g[A0][0] = lw_r(c, S0, 0x40);
        c.g[RA][0] = sx(0x0021_2334);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0021_2334)?;
        c.g[V1][0] = lw_r(c, V0, 0x8);
        c.g[A1][0] = 0;
        c.g[A0][0] = lh_r(c, V1, 0x48);
        c.g[A2][0] = lw_r(c, V1, 0x4c);
        let target = low(c, A2);
        c.g[RA][0] = sx(0x0021_234c);
        c.g[A0][0] = sx(low(c, V0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0021_234c)?;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenProfileSavePDA_Init(self)` (0x002111a0): create the profile
/// save PDA screen, open the template and label its slots.
pub fn ov_screen_profile_save_pda_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x470000);
        c.g[A0][0] = sx(0x470000u32.wrapping_add(0x1d50));
        c.g[S0][0] = Addr(at_3(c, S1, 0x10)).lw(c);
        callee::GET_HASH_VALUE_32.call(c, 0x0021_11c4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME.call(c, 0x0021_11d8)?;
        let absent = c.g[V0][0] == 0;
        Addr(at_3(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::UI_SCREEN_PLAY_FRAME.call(c, 0x0021_11f0)?;
        }
        c.cycles += 11;
        callee::OV_SCREEN_PDA_TEMPLATE_OPEN.call(c, 0x0021_11f8)?;
        c.g[A0][0] = c.g[S1][0];
        callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS.call(c, 0x0021_1200)?;
        c.g[A0][0] = c.g[S1][0];
        callee::C_OV_STATE_OVERLAY_INIT.call(c, 0x0021_1208)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Profile / PDA overlay.

/// `int OVScreenProfileSavePDA_OnWidgetNotify(OVScreen*, int, int key)`
/// (0x211b38): the save-PDA overlay answers widget 6/8 with the MC manager's
/// state and widget 9 with "handled".
pub fn ov_screen_profile_save_pda_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        callee::MC_MANAGER_GET_2.call(c, 0x0021_1b50)?;
        let mgr = r32(c, V0);
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 8;
        let key = r32(c, S0);
        let is8 = key == 8;
        let below9 = key < 9;
        c.g[V0][0] = below9 as u64;
        let mut result = 0x101u64;
        let mut vtable = is8;
        if !is8 {
            c.cycles += 2;
            c.g[V0][0] = 6;
            if !below9 {
                c.cycles += 3;
                c.g[V0][0] = 9;
                let is9 = key == 9;
                c.g[V0][0] = 0x101;
                if is9 {
                    c.cycles += 1;
                    result = 0x100;
                } else {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 2;
                let is6 = key == 6;
                c.g[V0][0] = 0x101;
                if is6 {
                    vtable = true;
                } else {
                    c.cycles += 2;
                }
            }
        }
        if vtable {
            c.cycles += 9;
            c.g[A2][0] = Addr(p(mgr, 0x434)).lw(c);
            c.g[A1][0] = Addr(p(mgr, 0x428)).lw(c);
            c.g[V1][0] = Addr(r32(c, A2)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x1b0)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x1b4)).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, target, 0x0021_1ba8)?;
            let nonzero = c.g[V0][0] != 0;
            c.g[V0][0] = 0x101;
            c.cycles += 1;
            if !nonzero {
                result = 0x100;
                c.cycles += 1;
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = result;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of the `OVScreen*_v38` visibility helpers: hide/show two named
/// UI objects and record the flag at_4 +0x220. `entry` is needed for the return
/// addresses the generated code uses.
pub(crate) fn ov_screen_set_two_visible(c: &mut Cpu, entry: u32, first_string: u32) -> Flow {
    c.cycles += 37;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x20);
    c.g[A0][0] = 0x470000;
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A1][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(first_string));
    call_ptr(c, table_callee::GET_HASH_VALUE32, entry + 0x24)?;
    c.g[S0][0] = c.g[S0][0] ^ 1;
    c.g[A0][0] = lw_3(c, S1, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, table_callee::UI_SCREEN_FIND_OBJECT, entry + 0x34)?;
    c.g[V1][0] = lw_3(c, V0, 0x8);
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
    c.g[A2][0] = lw_3(c, V1, 0x4c);
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, c.g[A2][0] as u32, entry + 0x4c)?;
    c.g[A0][0] = 0x4a0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20a0));
    call_ptr(c, table_callee::GET_HASH_VALUE32, entry + 0x58)?;
    c.g[A0][0] = lw_3(c, S1, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, table_callee::UI_SCREEN_FIND_OBJECT, entry + 0x64)?;
    c.g[V1][0] = lw_3(c, V0, 0x8);
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
    c.g[A2][0] = lw_3(c, V1, 0x4c);
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, c.g[A2][0] as u32, entry + 0x7c)?;
    Addr(c.g[S1][0] as u32).off(0x220).sw(c, c.g[S0][0]);
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void OVScreenProfileSavePDA_v38(self, on)` (0x211970).
pub fn ov_screen_profile_save_pda_v38(c: &mut Cpu) {
    guest(c, |c| ov_screen_set_two_visible(c, 0x0021_1970, 0x1db0))
}

/// `u32 OVScreenSaveReplay_v39(self, _, visible)` (0x1fe648) and
/// `OVScreenSaveRecords_v39` (0x1ff310): show/hide the screen's replay and
/// records widgets and store the inverted visibility.
pub(crate) fn ov_screen_set_widget_visibility(c: &mut Cpu, visible: u64) -> crate::call::Flow {
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    frame.save(c, S2, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = visible;
    c.g[A0][0] = 0x004a_2090;
    c.g[S0][0] ^= 1;
    call_ptr(c, table::STR_HASH, 0x001f_e674)?;
    c.g[A0][0] = lw_4(c, S1, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, table::UI_SCREEN_FIND_OBJECT, 0x001f_e680)?;
    c.g[A0][0] = 0x004a_2568;
    c.g[S2][0] = c.g[V0][0];
    call_ptr(c, table::STR_HASH, 0x001f_e690)?;
    c.g[A0][0] = lw_4(c, S1, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, table::UI_SCREEN_FIND_OBJECT, 0x001f_e69c)?;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(8).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
    c.g[A2][0] = lw_4(c, V1, 0x4c);
    let target = c.g[A2][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001f_e6b4)?;
    sw_2(c, S1, 0x224, c.g[S0][0]);
    c.g[A1][0] = c.g[S0][0];
    c.g[V0][0] = Addr(c.g[S2][0] as u32).off(8).lw(c);
    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
    c.g[V1][0] = lw_4(c, V0, 0x4c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001f_e6d0)?;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void OVScreenProfileSavePDA_v39(self, _, visible)` (0x211a08): same widget
/// visibility update as the replay/records screens.
pub fn ov_screen_profile_save_pda_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        ov_screen_set_widget_visibility(c, c.g[A2][0])
    })
}

/// `void OVScreenProfileSavePDA_InitSlotLabels(self)` (0x212080): set the
/// seven save-slot labels ("Slot 1".."Slot 6" and the empty slot).
pub fn ov_screen_profile_save_pda_init_slot_labels(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = 0x1;
        frame.save(c, S2, 0x30);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0x470000;
        frame.save_ra(c, 0x10);
        c.g[S3][0] = 0x4a0000;
        c.g[A0][0] = c.g[SP][0];
        loop {
            c.cycles += 26;
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2138));
            call_ptr(c, 0x0041_7828, 0x0021_20bc)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x0021_20c4)?;
            c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_20d0)?;
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x0021_20dc)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1d68));
            call_ptr(c, 0x0041_7828, 0x0021_20ec)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x0021_20f4)?;
            c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_2100)?;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.sw(p(c.g[V0][0] as u32, 0x18), c.g[V1][0]);
            c.g[V0][0] = ((c.g[S0][0] as i64) < 7) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[SP][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0021_20b0);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenProfileSavePDA_RefreshCardInfo(OVScreen* self)` (0x211380): refresh the
/// memory-card page of the profile save screen: device name, the six slot names with
/// their enabled/static states, and the two directory info icons.
pub fn ov_screen_profile_save_pda_refresh_card_info(c: &mut Cpu) {
    guest(c, |c| {
        'b_211820: {
            'b_211438: {
                c.cycles += 23;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x720));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x6f0));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x700));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x710));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x6e0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x6d0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x6c0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x6b0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x6a0));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x690));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x680), c.g[RA][0]);
                c.g[RA][0] = sx(0x002113bc);
                Callee {
                    addr: 0x00227f80,
                    f: ee::s05::MCManager_Get,
                }
                .call(c, 0x002113bc)?;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x002113d4);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002113d4)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    break 'b_211438;
                } else {
                    'b_211414: {
                        c.cycles += 8;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x002113f4);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002113f4)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x214));
                            break 'b_211414;
                        } else {
                            c.cycles += 6;
                            c.g[RA][0] = sx(0x00211404);
                            call_ptr(c, 0x001d8240, 0x00211404)?;
                            c.g[RA][0] = sx(0x0021140c);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x00211270,
                                f: ee::s05::OVScreenProfileSavePDA_ResetSlotList,
                            }
                            .call(c, 0x0021140c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x710));
                            if t {
                                break 'b_211820;
                            } else {
                                break 'b_211414;
                            }
                        }
                    }
                    'b_211434: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[A1][0] = 0x3;
                        if t {
                            break 'b_211434;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x118));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00211434);
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00211434)?;
                            break 'b_211434;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    break 'b_211438;
                }
            }
            'b_21181c: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00211448);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00211448)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A2][0] = c.g[SP][0];
                if t {
                    break 'b_21181c;
                } else {
                    c.cycles += 60;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                    c.g[A3][0] = 0x320;
                    c.g[RA][0] = sx(0x00211460);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00241710, 0x00211460)?;
                    c.g[S7][0] = 0u64;
                    c.g[A0][0] = 0x470000;
                    c.g[S5][0] = 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d78));
                    c.g[RA][0] = sx(0x00211478);
                    c.g[FP][0] = 0x470000;
                    call_ptr(c, 0x00317670, 0x00211478)?;
                    c.g[S6][0] = sx((c.g[S2][0] as u32).wrapping_add(0x234));
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0021148c);
                    c.g[S3][0] = 0u64;
                    call_ptr(c, 0x0039d860, 0x0021148c)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00211498);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0d00, 0x00211498)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x21c), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x158));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x15c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002114b4);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x002114b4)?;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x002114c0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf0));
                    call_ptr(c, 0x00317670, 0x002114c0)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x002114cc);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x002114cc)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = 0x4c0000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66c8));
                    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.ldr(3, c.g[A2][0] as u32);
                    c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                    c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x8f));
                    c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x88));
                    c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x97));
                    c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x90));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x200));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x204));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00211514);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00211514)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00211520);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0e90, 0x00211520)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00211538);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00211538)?;
                    c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
                    'l_211540: loop {
                        'b_211708: {
                            'b_2116d4: {
                                'b_2116c4: {
                                    'b_21163c: {
                                        c.cycles += 30;
                                        c.g[A2][0] = c.g[S5][0];
                                        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1d68));
                                        c.g[RA][0] = sx(0x00211550);
                                        c.g[A0][0] = c.g[S4][0];
                                        call_ptr(c, 0x00417828, 0x00211550)?;
                                        c.g[RA][0] = sx(0x00211558);
                                        c.g[A0][0] = c.g[S4][0];
                                        call_ptr(c, 0x00317670, 0x00211558)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                        c.g[RA][0] = sx(0x00211564);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x00211564)?;
                                        c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.g[S6][0];
                                        c.g[A1][0] = c.g[S3][0];
                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00211588);
                                        c.g[A0][0] =
                                            sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x00211588)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00211594);
                                        c.g[A1][0] = c.g[S6][0];
                                        call_ptr(c, 0x003a0e90, 0x00211594)?;
                                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                        c.g[A1][0] = c.g[S3][0];
                                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x002115b0);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x002115b0)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                        if t {
                                            break 'b_21163c;
                                        } else {
                                            c.cycles += 8;
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x002115d0);
                                            c.g[A0][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x002115d0)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A2][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                                break 'b_21163c;
                                            } else {
                                                c.cycles += 8;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x1b8));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x1bc));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x002115f0);
                                                c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x002115f0)?;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[A2][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x434)
                                                        );
                                                    break 'b_21163c;
                                                } else {
                                                    'b_211614: {
                                                        c.cycles += 2;
                                                        let t = c.g[S7][0] != 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        if t {
                                                            break 'b_211614;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[S7][0] = 0x1;
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x40));
                                                            let t = true;
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x44));
                                                            if t {
                                                                break 'b_2116c4;
                                                            } else {
                                                                break 'b_211614;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 10;
                                                    c.g[A1][0] = 0x1;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x00211628);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x00211628)?;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                    let t = true;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_2116c4;
                                                    } else {
                                                        break 'b_21163c;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00211654);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00211654)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.g[S3][0];
                                    if t {
                                        break 'b_2116d4;
                                    } else {
                                        'b_2116bc: {
                                            'b_21169c: {
                                                c.cycles += 8;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x00211674);
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00211674)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = 0x1;
                                                if t {
                                                    break 'b_21169c;
                                                } else {
                                                    c.cycles += 8;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x00211690);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x00211690)?;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    let t = true;
                                                    c.g[A1][0] = 0x1;
                                                    if t {
                                                        break 'b_2116bc;
                                                    } else {
                                                        break 'b_21169c;
                                                    }
                                                }
                                            }
                                            c.cycles += 8;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x002116b4);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x002116b4)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0u64;
                                            break 'b_2116bc;
                                        }
                                        c.cycles += 2;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                        break 'b_2116c4;
                                    }
                                }
                                c.cycles += 4;
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x002116cc);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x002116cc)?;
                                let t = true;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_211708;
                                } else {
                                    break 'b_2116d4;
                                }
                            }
                            c.cycles += 13;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x002116ec);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x002116ec)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00211704);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00211704)?;
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                            break 'b_211708;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S5][0] as i64) < 7i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00211540);
                            }
                            continue 'l_211540;
                        } else {
                            'b_21179c: {
                                'b_21176c: {
                                    c.cycles += 8;
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x00211728);
                                    c.g[A3][0] = 0x1;
                                    call_ptr(c, 0x001d97e0, 0x00211728)?;
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    c.g[V1][0] = 0x470000;
                                    if t {
                                        break 'b_21176c;
                                    } else {
                                        c.cycles += 14;
                                        c.g[V0][0] = 0x470000;
                                        c.g[RA][0] = sx(0x00211740);
                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1d90));
                                        call_ptr(c, 0x00317670, 0x00211740)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                        c.g[RA][0] = sx(0x0021174c);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x0021174c)?;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[A2][0] as u32;
                                        c.g[RA][0] = sx(0x00211764);
                                        c.g[A0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x00211764)?;
                                        let t = true;
                                        c.g[A1][0] = c.g[S1][0];
                                        if t {
                                            break 'b_21179c;
                                        } else {
                                            break 'b_21176c;
                                        }
                                    }
                                }
                                c.cycles += 12;
                                c.g[RA][0] = sx(0x00211774);
                                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d90));
                                call_ptr(c, 0x00317670, 0x00211774)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x00211780);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00211780)?;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                let t = c.g[A2][0] as u32;
                                c.g[RA][0] = sx(0x00211798);
                                c.g[A0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00211798)?;
                                c.g[A1][0] = c.g[S1][0];
                                break 'b_21179c;
                            }
                            'b_2117f0: {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
                                c.g[RA][0] = sx(0x002117ac);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x001d97e0, 0x002117ac)?;
                                c.g[V1][0] = 0xffffffffffffffff;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[A2][0] = 0x470000;
                                if t {
                                    break 'b_2117f0;
                                } else {
                                    c.cycles += 14;
                                    c.g[A1][0] = 0x470000;
                                    c.g[RA][0] = sx(0x002117c4);
                                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1da0));
                                    call_ptr(c, 0x00317670, 0x002117c4)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x002117d0);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x002117d0)?;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[A2][0] as u32;
                                    c.g[RA][0] = sx(0x002117e8);
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x002117e8)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x710));
                                    if t {
                                        break 'b_211820;
                                    } else {
                                        break 'b_2117f0;
                                    }
                                }
                            }
                            c.cycles += 11;
                            c.g[RA][0] = sx(0x002117f8);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1da0));
                            call_ptr(c, 0x00317670, 0x002117f8)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x00211804);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00211804)?;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = c.g[A2][0] as u32;
                            c.g[RA][0] = sx(0x0021181c);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0021181c)?;
                            break 'b_21181c;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x710));
            break 'b_211820;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x700));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x6f0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x6e0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x6d0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x6c0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x690));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x680));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x720));
        c.pc = t;
        return Ok(());
    });
}

// ---------------------------------------------------------------- OVScreenProfileSavePDA_ResetSlotList

/// `void cOVState_PROFILE::displayOff(this)` (0x211270): clears the save
/// screen's slot list. First, when the screen's profile record (`+0x8`)
/// has a valid handler (`+0x154`), the card is checked through the memory
/// card manager; when that is also clear, the card's info is refreshed
/// (`OVScreenProfileSavePDA_RefreshCardInfo`) and the function returns.
/// Otherwise the slots `1..=6` are cleared one by one: each slot name is
/// formatted (`sprintf` of the `0x471d68` pattern with the slot number),
/// its UI object is looked up by hash, and its two methods are called.
pub fn ov_screen_profile_save_pda_reset_slot_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S2][0] = c.g[A0][0];
        'exit_all: {
            'refresh: {
                c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x150)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x154)).lw(c);
                let handler = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, handler, 0x0021_12a0)?;
                let no_handler = c.g[V0][0] == 0;
                c.g[S1][0] = 0x1;
                if no_handler {
                    break 'refresh;
                }
                c.cycles += 10;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                call_ptr(c, 0x0022_7f80, 0x0021_12b0)?;
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x434)).lw(c);
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1a8)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1ac)).lw(c);
                let check = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, check, 0x0021_12c8)?;
                let card_clear = c.g[V0][0] == 0;
                c.g[S1][0] = 0x1;
                if card_clear {
                    break 'refresh;
                }
                c.cycles += 10;
                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A1][0] = 0x3;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x118)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x11c)).lw(c);
                let refresh_fn = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, refresh_fn, 0x0021_12e8)?;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0021_1380, 0x0021_12f0)?;
                frame.restore(c, S0, 0x80);
                break 'exit_all;
            }

            // Clear the slots 1..=6 (the loop's counter is `s1`).
            c.cycles += 2;
            c.g[S3][0] = 0x470000;
            c.g[A0][0] = c.g[SP][0];
            'slots: loop {
                c.cycles += 25;
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1d68));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x0041_7828, 0x0021_1310)?;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0031_7670, 0x0021_1318)?;
                c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x40)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0021_1324)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc8)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xcc)).lw(c);
                let clear_fn = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, clear_fn, 0x0021_1340)?;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A1][0] = 0x1;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x44)).lw(c);
                let hide_fn = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, hide_fn, 0x0021_1358)?;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 7i64) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.g[SP][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0021_1300);
                    }
                    continue 'slots;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x80);
                break 'exit_all;
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// A field of the overlay object whose pointer is held in `reg` (`off(reg)`).
/// Read from the callee-saved register each time, as the compiled code does.
pub(crate) fn profile_self_at(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr(c.g[reg][0] as u32).off(off)
}

/// `void cOVState_PROFILE_hideActionButtons(self, show)` (0x211850): hides or
/// shows the profile overlay's action buttons ("ps2x_b", "x_save", the delete
/// text, "ps2sq_b") when the hidden state changes, and caches it at +0x230.
/// `on` is 1 while `mIsOn` (+0x22c) is clear, otherwise `show`. Each widget
/// gets `setVisible(!on)` through its vtable slot 9; the two icons are
/// null-checked, the texts are not.
pub fn ov_state_profile_hide_action_buttons(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[V0][0] = profile_self_at(c, S0, 0x22c).lw(c);
        c.g[V1][0] = profile_self_at(c, S0, 0x230).lw(c);
        if c.g[V0][0] != 0 {
            c.g[S2][0] = c.g[A1][0];
        }
        let unchanged = c.g[V1][0] == c.g[S2][0];
        c.g[S1][0] = c.g[S2][0] ^ 0x1;
        if !unchanged {
            c.cycles += 9;
            c.g[A0][0] = 0x004a_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2090));
            call_ptr(c, 0x0031_7670, 0x0021_188c)?;
            c.g[A0][0] = profile_self_at(c, S0, 0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_1898)?;
            c.g[A2][0] = c.g[V0][0];
            if c.g[A2][0] == 0 {
                c.g[A0][0] = 0x004a_0000;
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0021_18bc)?;
                c.g[A0][0] = 0x004a_0000;
            }
            // "x_save" (never null-checked).
            c.cycles += 32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2568));
            call_ptr(c, 0x0031_7670, 0x0021_18c8)?;
            c.g[A0][0] = profile_self_at(c, S0, 0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_18d4)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(8).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
            c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0021_18ec)?;
            // The delete text (never null-checked).
            c.g[A0][0] = 0x0047_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1db0));
            call_ptr(c, 0x0031_7670, 0x0021_18f8)?;
            c.g[A0][0] = profile_self_at(c, S0, 0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_1904)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(8).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
            c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0021_191c)?;
            // "ps2sq_b" (null-checked).
            c.g[A0][0] = 0x004a_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20a0));
            call_ptr(c, 0x0031_7670, 0x0021_1928)?;
            c.g[A0][0] = profile_self_at(c, S0, 0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_1934)?;
            c.g[A2][0] = c.g[V0][0];
            let is_null = c.g[A2][0] == 0;
            c.g[A1][0] = c.g[S1][0];
            if !is_null {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0021_1954)?;
            }
            c.cycles += 1;
            let hidden = c.g[S2][0];
            profile_self_at(c, S0, 0x230).sw(c, hidden);
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

/// `void cOVState_PROFILE_hideMainButton(self, loadBlocked, on)` (0x211a08):
/// hides or shows the profile overlay's main ("ps2x_b") and save ("x_save")
/// buttons, stores `mMainButtonActive = !on` at +0x224 and sets the X icon
/// with `setVisible(!on)`. `loadBlocked` is not used.
pub fn ov_state_profile_hide_main_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x004a_0000;
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2090));
        c.g[S0][0] = c.g[S0][0] ^ 0x1;
        call_ptr(c, 0x0031_7670, 0x0021_1a34)?;
        c.g[A0][0] = profile_self_at(c, S1, 0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0021_1a40)?;
        c.g[A0][0] = 0x004a_0000;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2568));
        call_ptr(c, 0x0031_7670, 0x0021_1a50)?;
        c.g[A0][0] = profile_self_at(c, S1, 0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0021_1a5c)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(8).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
        c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
        let target = c.g[A2][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0021_1a74)?;
        let active = c.g[S0][0];
        profile_self_at(c, S1, 0x224).sw(c, active);
        c.g[A1][0] = c.g[S0][0];
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(8).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0021_1a90)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The field at `off` from the pointer held in register `reg`, read as the
/// compiled code does (32-bit address arithmetic).
pub(crate) fn oc997_reg_at(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr((c.g[reg][0] as u32).wrapping_add(off))
}

/// `void OVScreenAutosave_UpdateIcon(OVScreenAutosave* self)` (0x212360):
/// shows or hides the autosave icon and refreshes its label and animation
/// from the card state; `self` keeps the icon widget at +0x40.
pub fn ov_screen_autosave_update_icon(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        MC_MANAGER_GET.call(c, 0x0021_2384)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = oc997_reg_at(c, S1, 0x1a8).lw(c);
        c.g[V0][0] = oc997_reg_at(c, S1, 0x230).lw(c);
        'body: {
            if c.g[V1][0] == c.g[V0][0] {
                frame.restore(c, S0, 0x30);
                break 'body;
            }
            c.cycles += 10;
            let v1 = c.g[V1][0];
            oc997_reg_at(c, S1, 0x230).sw(c, v1);
            c.g[A0][0] = 0x004a_2770;
            call_ptr(c, 0x0031_7670, 0x0021_23a8)?;
            c.g[A0][0] = oc997_reg_at(c, S1, 0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_23b4)?;
            c.g[S2][0] = c.g[V0][0];
            if c.g[S2][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'body;
            }
            'b478: {
                c.cycles += 3;
                c.g[V0][0] = oc997_reg_at(c, S1, 0x1a8).lw(c);
                if c.g[V0][0] == 0 {
                    break 'b478;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0023_c898, 0x0021_23d4)?;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = 6;
                if t {
                    break 'b478;
                }
                c.cycles += 3;
                c.g[V1][0] = oc997_reg_at(c, S1, 0x1c0).lw(c);
                let t = c.g[V1][0] == c.g[V0][0];
                if t {
                    break 'b478;
                }
                'b424: {
                    c.cycles += 10;
                    c.g[S0][0] = oc997_reg_at(c, S1, 0x10).lw(c);
                    c.g[A0][0] = 0x0047_1e48;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    call_ptr(c, 0x0031_7670, 0x0021_23fc)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_7140, 0x0021_2408)?;
                    let t = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        break 'b424;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = 9;
                    c.g[A3][0] = 0;
                    c.g[T0][0] = 0;
                    call_ptr(c, 0x0039_fcc8, 0x0021_2424)?;
                }
                c.cycles += 21;
                c.g[A0][0] = 0x0047_1e38;
                call_ptr(c, 0x0031_7670, 0x0021_2430)?;
                c.g[A0][0] = oc997_reg_at(c, S1, 0x40).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0021_243c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                c.g[V0][0] = oc997_reg_at(c, S0, 0x8).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0021_2458)?;
                c.g[A0][0] = 0x0047_1e58;
                call_ptr(c, 0x0031_7670, 0x0021_2464)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x0021_2470)?;
                frame.restore(c, S0, 0x30);
                break 'body;
            }
            c.cycles += 15;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039_fd38, 0x0021_2480)?;
            c.g[A0][0] = 0x0047_1e38;
            call_ptr(c, 0x0031_7670, 0x0021_248c)?;
            c.g[A0][0] = oc997_reg_at(c, S1, 0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0021_2498)?;
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4c)).lw(c);
            let t = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0021_24b0)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreen_DebugPrintfStub(...)` (0x212680): the variadic debug-print
/// stub. It spills the argument registers a1..a3, t0..t3 and f12, f14, f16,
/// f18 into its frame, returns its first argument in v0, and does nothing else.
pub fn ov_screen_debug_printf_stub(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x80));
    c.g[V0][0] = c.g[A0][0];
    let base = sp(c);
    c.sd(base.wrapping_add(0x48), c.g[A1][0]);
    c.sd(base.wrapping_add(0x50), c.g[A2][0]);
    c.sd(base.wrapping_add(0x58), c.g[A3][0]);
    c.sd(base.wrapping_add(0x60), c.g[T0][0]);
    c.sd(base.wrapping_add(0x68), c.g[T1][0]);
    c.sd(base.wrapping_add(0x70), c.g[T2][0]);
    c.sd(base.wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, base.wrapping_add(0x38));
    c.swc1(14, base.wrapping_add(0x3c));
    c.swc1(16, base.wrapping_add(0x40));
    c.swc1(18, base.wrapping_add(0x44));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x80));
    c.pc = ra;
}

/// `OVScreenProfileSavePDA_dtor` (0x221d88): vtable `0x472928`, then the
/// `MemCardStateBase` destructor (0x1d5428).
pub fn ov_screen_profile_save_pda_dtor(c: &mut Cpu) {
    oc1013_screen_dtor(c, 0x2928, 0x001d_5428, 0x0022_1da4)
}

/// `OVScreenProfileSavePDA_ctor_2` (0x221db0): the `MemCardStateBase`
/// constructor (0x1d53b0) with a0 = this and a1 = a2 = 0, then the vtable
/// `0x472928` at this + 8. v0 ends as `this`.
pub fn ov_screen_profile_save_pda_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001d_53b0, 0x0022_1dcc)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2928));
        frame.restore_ra(c, 0);
        let vt = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vt);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const OC1015_VT_AUTOSAVE: u32 = OC1015_DATA + 0x1f80;

/// `cFEMemCard` constructor (0x1d53b0, the autosave state's base) and destructor
/// (0x1d5428).
pub(crate) const OC1015_FEMEM_CTOR: u32 = 0x001d_53b0;

pub(crate) const OC1015_FEMEM_DTOR: u32 = 0x001d_5428;

/// `OVScreenAutosave_dtor` (0x222280): the autosave overlay's destructor. Sets
/// the autosave vtable at +0x8, then runs the memory-card state's destructor
/// (0x1d5428).
pub fn ov_screen_autosave_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc1015_dtor(c, OC1015_VT_AUTOSAVE, OC1015_FEMEM_DTOR, 0x0022_229c)
    });
}

/// `OVScreenAutosave_ctor_2` (0x2222b8): constructs the autosave overlay state
/// on top of the memory-card state constructor (0x1d53b0, with null `a1` and
/// `a2`), then sets the autosave vtable at +0x8.
pub fn ov_screen_autosave_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OC1015_FEMEM_CTOR, 0x0022_22d4)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = OC1015_DATA as u64;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = OC1015_VT_AUTOSAVE as u64;
        f.restore_ra(c, 0);
        let vt = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vt);
        f.restore(c, S0, 0x10);
        f.close(c);
        Ok(())
    });
}

/// `OVScreenProfileSavePDA_OnWidgetAction` (0x00211bc8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_profile_save_pda_on_widget_action_2(c: &mut Cpu) {
    if c.pc != 0x00211bc8 {
        return ov_screen_profile_save_pda_on_widget_action_2_resume(c);
    }
    guest(c, |c| 'l_211bc8: loop {
        'b_212064: {
            'b_212060: {
                'b_21205c: {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                    if t {
                        break 'b_21205c;
                    } else {
                        'b_211fa8: {
                            'b_211f40: {
                                'b_211d14: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x470000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1e00));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x00211bc8 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00211bc8);
                                            }
                                            continue 'l_211bc8;
                                        }
                                        0x00211c14 => {
                                            'b_211cd4: {
                                                'b_211c6c: {
                                                    c.cycles += 17;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x1a4)
                                                        );
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[RA][0] = sx(0x00211c24);
                                                    c.g[S0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x74));
                                                    Callee {
                                                        addr: 0x00227f80,
                                                        f: ee::s05::MCManager_Get,
                                                    }
                                                    .call(c, 0x00211c24)?;
                                                    c.g[A0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x276));
                                                    c.g[RA][0] = sx(0x00211c30);
                                                    c.g[A1][0] = c.g[S0][0];
                                                    call_ptr(c, 0x002c2540, 0x00211c30)?;
                                                    c.g[RA][0] = sx(0x00211c38);
                                                    call_ptr(c, 0x0014dc80, 0x00211c38)?;
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x00211c44);
                                                    c.g[A1][0] = 0x1;
                                                    call_ptr(c, 0x0014dd58, 0x00211c44)?;
                                                    c.g[S4][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x00211c50);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x00416810, 0x00211c50)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A2][0] = c.g[S0][0];
                                                    if t {
                                                        break 'b_211c6c;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1cc));
                                                        c.g[RA][0] = sx(0x00211c64);
                                                        c.g[A0][0] = c.g[S4][0];
                                                        call_ptr(c, 0x00147138, 0x00211c64)?;
                                                        let t = true;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S4][0] as u32)
                                                                .wrapping_add(0xc));
                                                        if t {
                                                            break 'b_211cd4;
                                                        } else {
                                                            break 'b_211c6c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 26;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                c.g[A0][0] = 0x470000;
                                                c.g[S2][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x70));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1dc0));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[RA][0] = sx(0x00211c90);
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                call_ptr(c, 0x00317670, 0x00211c90)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00211ca0);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, t, 0x00211ca0)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x00211cac);
                                                c.g[A0][0] = c.g[S2][0];
                                                call_ptr(c, 0x002c2580, 0x00211cac)?;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                                                c.g[A1][0] = c.g[S2][0];
                                                c.g[A0][0] = c.g[SP][0];
                                                c.g[RA][0] = sx(0x00211cc0);
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                                call_ptr(c, 0x00417828, 0x00211cc0)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[RA][0] = sx(0x00211cd0);
                                                c.g[A2][0] = c.g[SP][0];
                                                call_ptr(c, 0x00147138, 0x00211cd0)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                                                break 'b_211cd4;
                                            }
                                            c.cycles += 16;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00211ce4);
                                            c.g[A0][0] =
                                                sx((c.g[S4][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00211ce4)?;
                                            c.g[RA][0] = sx(0x00211cec);
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            Callee {
                                                addr: 0x00227f80,
                                                f: ee::s05::MCManager_Get,
                                            }
                                            .call(c, 0x00211cec)?;
                                            c.g[A2][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x00211cfc);
                                            c.g[A1][0] = 0x1;
                                            call_ptr(c, 0x0023cae8, 0x00211cfc)?;
                                            c.g[V1][0] = 0x1;
                                            c.g[V0][0] = 0x3;
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1c0),
                                                c.g[V0][0],
                                            );
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1dc),
                                                c.g[V1][0],
                                            );
                                            let t = true;
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1a8),
                                                c.g[V1][0],
                                            );
                                            if t {
                                                break 'b_21205c;
                                            } else {
                                                break 'b_211d14;
                                            }
                                        }
                                        0x00211d14 => {
                                            break 'b_211d14;
                                        }
                                        0x00211f40 => {
                                            break 'b_211f40;
                                        }
                                        0x00211fa8 => {
                                            break 'b_211fa8;
                                        }
                                        0x0021205c => {
                                            break 'b_21205c;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x00211bc8, 0x0021207c);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S1][0] == 0u64;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                                if t {
                                    break 'b_212060;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                                    if c.g[V0][0] != 0u64 {
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                                        break 'b_212064;
                                    } else {
                                        c.cycles += 7;
                                        c.g[RA][0] = sx(0x00211d30);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        Callee {
                                            addr: 0x00227f80,
                                            f: ee::s05::MCManager_Get,
                                        }
                                        .call(c, 0x00211d30)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
                                        c.g[V0][0] = 0x3;
                                        if c.g[V1][0] != c.g[V0][0] {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                                            break 'b_212060;
                                        } else {
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                            c.g[V1][0] = 0x1;
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1c4),
                                                c.g[V0][0],
                                            );
                                            c.sw(
                                                (c.g[A0][0] as u32).wrapping_add(0xf8),
                                                c.g[V0][0],
                                            );
                                            c.g[S2][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                                            if c.g[S2][0] != c.g[V1][0] {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                                                break 'b_212060;
                                            } else {
                                                'b_211eb4: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x1a4)
                                                        );
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    if t {
                                                        break 'b_211eb4;
                                                    } else {
                                                        c.cycles += 82;
                                                        c.g[A1][0] = 0x470000;
                                                        c.g[A0][0] = 0x444;
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x1dd8));
                                                        c.g[A2][0] = 0x100;
                                                        c.g[RA][0] = sx(0x00211d84);
                                                        c.g[A3][0] = 0u64;
                                                        call_ptr(c, 0x00317d70, 0x00211d84)?;
                                                        c.g[T0][0] = c.lbu(
                                                            (c.g[S3][0] as u32).wrapping_add(0x15),
                                                        );
                                                        c.g[A2][0] = c.g[S3][0];
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[A3][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211d9c);
                                                        c.g[A0][0] = c.g[V0][0];
                                                        call_ptr(c, 0x001cb030, 0x00211d9c)?;
                                                        c.g[V1][0] = 0x37;
                                                        c.sw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x1a4),
                                                            c.g[V0][0],
                                                        );
                                                        c.sw(
                                                            (c.g[V0][0] as u32).wrapping_add(0x140),
                                                            c.g[V1][0],
                                                        );
                                                        c.g[A1][0] = 0x8;
                                                        c.g[RA][0] = sx(0x00211db4);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        call_ptr(c, 0x001cd088, 0x00211db4)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A2][0] = 0x1;
                                                        c.g[A3][0] = 0x2;
                                                        c.g[RA][0] = sx(0x00211dc8);
                                                        c.g[A1][0] = 0x4b;
                                                        call_ptr(c, 0x001ce3c8, 0x00211dc8)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x1;
                                                        c.sw(
                                                            (c.g[V0][0] as u32).wrapping_add(0x58),
                                                            c.g[S2][0],
                                                        );
                                                        c.g[RA][0] = sx(0x00211ddc);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        call_ptr(c, 0x001ce408, 0x00211ddc)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0xe;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211df0);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211df0)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x27;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e04);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e04)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x28;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e18);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e18)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x34;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e2c);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e2c)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x1a;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e40);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e40)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x19;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e54);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e54)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x35;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e68);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e68)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x40;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e7c);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e7c)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x41;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e90);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e90)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x42;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211ea4);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211ea4)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[RA][0] = sx(0x00211eb0);
                                                        c.g[A1][0] = 0x1;
                                                        call_ptr(c, 0x001cd0b0, 0x00211eb0)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        break 'b_211eb4;
                                                    }
                                                }
                                                c.cycles += 35;
                                                c.g[A0][0] = 0x470000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1de8));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[RA][0] = sx(0x00211ed0);
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                call_ptr(c, 0x00317670, 0x00211ed0)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00211ee0);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, t, 0x00211ee0)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                                                c.g[RA][0] = sx(0x00211eec);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, 0x001ccf70, 0x00211eec)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                                                c.g[RA][0] = sx(0x00211ef8);
                                                c.sw(
                                                    (c.g[V0][0] as u32).wrapping_add(0x54),
                                                    c.g[S2][0],
                                                );
                                                call_ptr(c, 0x0014dc80, 0x00211ef8)?;
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x00211f04);
                                                c.g[A1][0] = 0x1;
                                                call_ptr(c, 0x0014dd58, 0x00211f04)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                                                c.g[RA][0] = sx(0x00211f10);
                                                c.g[A0][0] = c.g[V0][0];
                                                call_ptr(c, 0x00147170, 0x00211f10)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                                                c.g[RA][0] = sx(0x00211f1c);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, 0x001cb418, 0x00211f1c)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                c.g[A1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                                c.g[RA][0] = sx(0x00211f34);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                                call_ptr(c, 0x0039f290, 0x00211f34)?;
                                                c.sw(
                                                    (c.g[S3][0] as u32).wrapping_add(0x1a0),
                                                    c.g[S2][0],
                                                );
                                                let t = true;
                                                c.sw((c.g[S3][0] as u32).wrapping_add(0x19c), 0u64);
                                                if t {
                                                    break 'b_21205c;
                                                } else {
                                                    break 'b_211f40;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x00211f48);
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            Callee {
                                addr: 0x00227f80,
                                f: ee::s05::MCManager_Get,
                            }
                            .call(c, 0x00211f48)?;
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A0][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                            let t = c.g[V0][0] == c.g[A0][0];
                            c.g[V0][0] = 0x6;
                            if t {
                                break 'b_21205c;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                                if t {
                                    break 'b_212060;
                                } else {
                                    'b_211fa0: {
                                        'b_211f88: {
                                            'b_211f84: {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == 0u64;
                                                c.sw(
                                                    (c.g[S3][0] as u32).wrapping_add(0x1e4),
                                                    c.g[A0][0],
                                                );
                                                if t {
                                                    break 'b_211f84;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x2;
                                                    if c.g[V1][0] == c.g[V0][0] {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x748));
                                                        break 'b_211f88;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] != c.g[A0][0];
                                                        c.g[V0][0] = 0x6;
                                                        if t {
                                                            break 'b_211fa0;
                                                        } else {
                                                            break 'b_211f84;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x748));
                                            break 'b_211f88;
                                        }
                                        c.cycles += 6;
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00211f9c);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x00211f9c)?;
                                        c.g[V0][0] = 0x6;
                                        break 'b_211fa0;
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                                    if t {
                                        break 'b_21205c;
                                    } else {
                                        break 'b_211fa8;
                                    }
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                        c.g[S2][0] = 0x1;
                        let t = c.g[V0][0] == c.g[S2][0];
                        c.g[V0][0] = 0x6;
                        if t {
                            break 'b_21205c;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                            if t {
                                break 'b_212060;
                            } else {
                                c.cycles += 12;
                                c.g[RA][0] = sx(0x00211fcc);
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                Callee {
                                    addr: 0x00227f80,
                                    f: ee::s05::MCManager_Get,
                                }
                                .call(c, 0x00211fcc)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x428));
                                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x00211fec);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00211fec)?;
                                if c.g[V0][0] == 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    break 'b_212060;
                                } else {
                                    c.cycles += 6;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c4), c.g[V0][0]);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
                                    c.g[S1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                                    if c.g[S1][0] != c.g[S2][0] {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        break 'b_212060;
                                    } else {
                                        c.cycles += 9;
                                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                                        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c4));
                                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x00212028);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x00212028)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x4;
                                        if t {
                                            break 'b_21205c;
                                        } else {
                                            c.cycles += 11;
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1a8),
                                                c.g[S1][0],
                                            );
                                            c.g[A0][0] = c.g[S0][0];
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1c0),
                                                c.g[V0][0],
                                            );
                                            c.g[RA][0] = sx(0x00212044);
                                            c.g[A1][0] = 0u64;
                                            call_ptr(c, 0x0023d570, 0x00212044)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                                            c.g[A1][0] = 0x18;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x0021205c);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0021205c)?;
                                            break 'b_21205c;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                break 'b_212060;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
            break 'b_212064;
        }
        c.cycles += 6;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
        c.pc = t;
        return Ok(());
    });
}

/// Native resume of `ov_screen_profile_save_pda_on_widget_action_2` (converted from the `entries` body): any entry
/// other than 0x00211bc8, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn ov_screen_profile_save_pda_on_widget_action_2_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00211bc8 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                    if t {
                        pc = 0x0021205c;
                        continue;
                    }
                    pc = 0x00211bf8;
                    continue;
                }
                0x00211bf8 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x470000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1e00));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00211c14 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[RA][0] = sx(0x00211c24);
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x74));
                    Callee {
                        addr: 0x00227f80,
                        f: ee::s05::MCManager_Get,
                    }
                    .call(c, 0x00211c24)?;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x276));
                    c.g[RA][0] = sx(0x00211c30);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x002c2540, 0x00211c30)?;
                    c.g[RA][0] = sx(0x00211c38);
                    call_ptr(c, 0x0014dc80, 0x00211c38)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00211c44);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014dd58, 0x00211c44)?;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00211c50);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00416810, 0x00211c50)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.g[S0][0];
                    if t {
                        pc = 0x00211c6c;
                        continue;
                    }
                    pc = 0x00211c58;
                    continue;
                }
                0x00211c58 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                    c.g[RA][0] = sx(0x00211c64);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00147138, 0x00211c64)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                    if t {
                        pc = 0x00211cd4;
                        continue;
                    }
                    pc = 0x00211c6c;
                    continue;
                }
                0x00211c6c => {
                    c.cycles += 26;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1dc0));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x00211c90);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x00211c90)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00211ca0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x00211ca0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00211cac);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x002c2580, 0x00211cac)?;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00211cc0);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00417828, 0x00211cc0)?;
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x00211cd0);
                    c.g[A2][0] = c.g[SP][0];
                    call_ptr(c, 0x00147138, 0x00211cd0)?;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                    pc = 0x00211cd4;
                    continue;
                }
                0x00211cd4 => {
                    c.cycles += 16;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00211ce4);
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00211ce4)?;
                    c.g[RA][0] = sx(0x00211cec);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    Callee {
                        addr: 0x00227f80,
                        f: ee::s05::MCManager_Get,
                    }
                    .call(c, 0x00211cec)?;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00211cfc);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0023cae8, 0x00211cfc)?;
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = 0x3;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1dc), c.g[V1][0]);
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1a8), c.g[V1][0]);
                    if t {
                        pc = 0x0021205c;
                        continue;
                    }
                    pc = 0x00211d14;
                    continue;
                }
                0x00211d14 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    if t {
                        pc = 0x00212060;
                        continue;
                    }
                    pc = 0x00211d1c;
                    continue;
                }
                0x00211d1c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));

                    if c.g[V0][0] != 0u64 {
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                        pc = 0x00212064;
                        continue;
                    }
                    pc = 0x00211d28;
                    continue;
                }
                0x00211d28 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00211d30);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    Callee {
                        addr: 0x00227f80,
                        f: ee::s05::MCManager_Get,
                    }
                    .call(c, 0x00211d30)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
                    c.g[V0][0] = 0x3;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                        pc = 0x00212060;
                        continue;
                    }
                    pc = 0x00211d44;
                    continue;
                }
                0x00211d44 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = 0x1;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c4), c.g[V0][0]);
                    c.sw((c.g[A0][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
                    c.g[S2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));

                    if c.g[S2][0] != c.g[V1][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                        pc = 0x00212060;
                        continue;
                    }
                    pc = 0x00211d60;
                    continue;
                }
                0x00211d60 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x00211eb4;
                        continue;
                    }
                    pc = 0x00211d6c;
                    continue;
                }
                0x00211d6c => {
                    c.cycles += 82;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = 0x444;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1dd8));
                    c.g[A2][0] = 0x100;
                    c.g[RA][0] = sx(0x00211d84);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x00211d84)?;
                    c.g[T0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(0x15));
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x00211d9c);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x001cb030, 0x00211d9c)?;
                    c.g[V1][0] = 0x37;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1a4), c.g[V0][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x140), c.g[V1][0]);
                    c.g[A1][0] = 0x8;
                    c.g[RA][0] = sx(0x00211db4);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    call_ptr(c, 0x001cd088, 0x00211db4)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = 0x2;
                    c.g[RA][0] = sx(0x00211dc8);
                    c.g[A1][0] = 0x4b;
                    call_ptr(c, 0x001ce3c8, 0x00211dc8)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[A1][0] = 0x1;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[S2][0]);
                    c.g[RA][0] = sx(0x00211ddc);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    call_ptr(c, 0x001ce408, 0x00211ddc)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[A1][0] = 0xe;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00211df0);
                    c.g[A3][0] = 0x2;
                    call_ptr(c, 0x001ce3c8, 0x00211df0)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[A1][0] = 0x27;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00211e04);
                    c.g[A3][0] = 0x2;
                    call_ptr(c, 0x001ce3c8, 0x00211e04)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[A1][0] = 0x28;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00211e18);
                    c.g[A3][0] = 0x2;
                    call_ptr(c, 0x001ce3c8, 0x00211e18)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[A1][0] = 0x34;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00211e2c);
                    c.g[A3][0] = 0x2;
                    call_ptr(c, 0x001ce3c8, 0x00211e2c)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[A1][0] = 0x1a;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00211e40);
                    c.g[A3][0] = 0x2;
                    call_ptr(c, 0x001ce3c8, 0x00211e40)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[A1][0] = 0x19;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00211e54);
                    c.g[A3][0] = 0x2;
                    call_ptr(c, 0x001ce3c8, 0x00211e54)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[A1][0] = 0x35;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00211e68);
                    c.g[A3][0] = 0x2;
                    call_ptr(c, 0x001ce3c8, 0x00211e68)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[A1][0] = 0x40;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00211e7c);
                    c.g[A3][0] = 0x2;
                    call_ptr(c, 0x001ce3c8, 0x00211e7c)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[A1][0] = 0x41;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00211e90);
                    c.g[A3][0] = 0x2;
                    call_ptr(c, 0x001ce3c8, 0x00211e90)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[A1][0] = 0x42;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00211ea4);
                    c.g[A3][0] = 0x2;
                    call_ptr(c, 0x001ce3c8, 0x00211ea4)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[RA][0] = sx(0x00211eb0);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x001cd0b0, 0x00211eb0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    pc = 0x00211eb4;
                    continue;
                }
                0x00211eb4 => {
                    c.cycles += 35;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1de8));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x00211ed0);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x00211ed0)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00211ee0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x00211ee0)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[RA][0] = sx(0x00211eec);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x001ccf70, 0x00211eec)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[RA][0] = sx(0x00211ef8);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x54), c.g[S2][0]);
                    call_ptr(c, 0x0014dc80, 0x00211ef8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00211f04);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014dd58, 0x00211f04)?;
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                    c.g[RA][0] = sx(0x00211f10);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00147170, 0x00211f10)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[RA][0] = sx(0x00211f1c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x001cb418, 0x00211f1c)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x00211f34);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039f290, 0x00211f34)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1a0), c.g[S2][0]);
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x19c), 0u64);
                    if t {
                        pc = 0x0021205c;
                        continue;
                    }
                    pc = 0x00211f40;
                    continue;
                }
                0x00211f40 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00211f48);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    Callee {
                        addr: 0x00227f80,
                        f: ee::s05::MCManager_Get,
                    }
                    .call(c, 0x00211f48)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                    let t = c.g[V0][0] == c.g[A0][0];
                    c.g[V0][0] = 0x6;
                    if t {
                        pc = 0x0021205c;
                        continue;
                    }
                    pc = 0x00211f5c;
                    continue;
                }
                0x00211f5c => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    if t {
                        pc = 0x00212060;
                        continue;
                    }
                    pc = 0x00211f68;
                    continue;
                }
                0x00211f68 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1e4), c.g[A0][0]);
                    if t {
                        pc = 0x00211f84;
                        continue;
                    }
                    pc = 0x00211f70;
                    continue;
                }
                0x00211f70 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x748));
                        pc = 0x00211f88;
                        continue;
                    }
                    pc = 0x00211f7c;
                    continue;
                }
                0x00211f7c => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = 0x6;
                    if t {
                        pc = 0x00211fa0;
                        continue;
                    }
                    pc = 0x00211f84;
                    continue;
                }
                0x00211f84 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x748));
                    pc = 0x00211f88;
                    continue;
                }
                0x00211f88 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00211f9c);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00211f9c)?;
                    c.g[V0][0] = 0x6;
                    pc = 0x00211fa0;
                    continue;
                }
                0x00211fa0 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                    if t {
                        pc = 0x0021205c;
                        continue;
                    }
                    pc = 0x00211fa8;
                    continue;
                }
                0x00211fa8 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                    c.g[S2][0] = 0x1;
                    let t = c.g[V0][0] == c.g[S2][0];
                    c.g[V0][0] = 0x6;
                    if t {
                        pc = 0x0021205c;
                        continue;
                    }
                    pc = 0x00211fb8;
                    continue;
                }
                0x00211fb8 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    if t {
                        pc = 0x00212060;
                        continue;
                    }
                    pc = 0x00211fc4;
                    continue;
                }
                0x00211fc4 => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x00211fcc);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    Callee {
                        addr: 0x00227f80,
                        f: ee::s05::MCManager_Get,
                    }
                    .call(c, 0x00211fcc)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x428));
                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00211fec);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00211fec)?;

                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                        pc = 0x00212060;
                        continue;
                    }
                    pc = 0x00211ff4;
                    continue;
                }
                0x00211ff4 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c4), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
                    c.g[S1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));

                    if c.g[S1][0] != c.g[S2][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                        pc = 0x00212060;
                        continue;
                    }
                    pc = 0x0021200c;
                    continue;
                }
                0x0021200c => {
                    c.cycles += 9;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c4));
                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00212028);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00212028)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x4;
                    if t {
                        pc = 0x0021205c;
                        continue;
                    }
                    pc = 0x00212030;
                    continue;
                }
                0x00212030 => {
                    c.cycles += 11;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1a8), c.g[S1][0]);
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                    c.g[RA][0] = sx(0x00212044);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0023d570, 0x00212044)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                    c.g[A1][0] = 0x18;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0021205c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0021205c)?;
                    pc = 0x0021205c;
                    continue;
                }
                0x0021205c => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    pc = 0x00212060;
                    continue;
                }
                0x00212060 => {
                    c.cycles += 1;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                    pc = 0x00212064;
                    continue;
                }
                0x00212064 => {
                    c.cycles += 6;
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00211bc8, 0x0021207c);
                    return Ok(());
                }
            }
        }
    });
}

/// `void OVScreenProfileSavePDA_Shutdown(OVScreenProfileSavePDA* self)`
/// (0x211b10): shuts the memory-card state base down and clears the
/// field at +0x1a4.
pub fn ov_screen_profile_save_pda_shutdown_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001d_58b8, 0x0021_1b24)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1a4), 0u64);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int OVScreenProfileSavePDA_OnWidgetNotify(self, a1, int key)` (0x211b38):
/// keys 6 and 8 ask the selected memory-card device (a virtual call on the
/// manager's object, +0x434 / +0x428) whether it is actionable (0x101 when
/// it is, else 0x100); key 9 returns 0x100, anything else 0x101.
pub fn ov_screen_profile_save_pda_on_widget_notify_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        Callee {
            addr: 0x0022_7f80,
            f: ee::s05::f_00227f80,
        }
        .call(c, 0x0021_1b50)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 0x8;
        let is_8 = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[S0][0] < 0x9u64) as u64;
        let mut ask_device = is_8;
        if !is_8 {
            c.cycles += 2;
            let ge_9 = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x6;
            if !ge_9 {
                c.cycles += 2;
                let is_6 = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 0x101;
                if is_6 {
                    ask_device = true;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x10);
                    ov_pda_notify_epilogue(c, frame);
                    return Ok(());
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x9;
                let is_9 = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 0x101;
                if is_9 {
                    c.cycles += 1;
                    c.g[V0][0] = 0x100;
                    c.cycles += 1;
                } else {
                    c.cycles += 2;
                }
                frame.restore(c, S0, 0x10);
                ov_pda_notify_epilogue(c, frame);
                return Ok(());
            }
        }
        debug_assert!(ask_device);
        c.cycles += 9;
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x434));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x428));
        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0021_1ba8)?;
        let actionable = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0x101;
        if !actionable {
            c.cycles += 1;
            c.g[V0][0] = 0x100;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        ov_pda_notify_epilogue(c, frame);
        Ok(())
    });
}

/// The shared exit of `OVScreenProfileSavePDA_OnWidgetNotify`.
pub(crate) fn ov_pda_notify_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void OVScreenAutosave_Update(OVScreenAutosave* self)` (0x212208): the
/// autosave state machine. State 1 starts the save (writeSetFileType, state
/// 3); state 3 ends it (state 6) once the selected device is no longer
/// plugged in / valid; then the base screen's update runs.
pub fn ov_screen_autosave_update_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0022_7f80,
            f: ee::s05::f_00227f80,
        }
        .call(c, 0x0021_2228)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
        c.g[V0][0] = 0x1;
        'tail: {
            let at_check = if c.g[S1][0] != c.g[V0][0] {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                true
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                if c.g[V0][0] != 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                    true
                } else {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[A1][0];
                    c.g[A2][0] = 0u64;
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0023_cae8, 0x0021_2258)?;
                    c.g[V0][0] = 0x3;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f0), c.g[S1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[S1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1dc), c.g[S1][0]);
                    false
                }
            };
            if at_check {
                c.cycles += 3;
                c.g[V0][0] = 0x3;
                let not_saving = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0xe0;
                if not_saving {
                    break 'tail;
                }
                'device_ok: {
                    'device_bad: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
                        c.mult(0, A0, A0, V0);
                        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x350));
                        let unplugged = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.g[V1][0];
                        if !unplugged {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x344));
                            if c.g[V1][0] == 0u64 {
                                c.g[V0][0] = 0x1;
                                break 'device_ok;
                            }
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x348));
                            if c.g[V1][0] != 0u64 {
                                break 'tail;
                            }
                            break 'device_bad;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                }
                c.cycles += 5;
                c.g[V1][0] = 0x6;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1e4), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1dc), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), 0u64);
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_8de0, 0x0021_22cc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void OVScreenAutosave_Shutdown(OVScreenAutosave* self)` (0x212508): base
/// shutdown, then by the state at +0x22c: 1 tells the card manager to stop,
/// 6 and 7 open a popup (resource 0x4c / 0x58) and hand it to the popup
/// handlers, 0 rebuilds the front-end objects and re-runs the init chain.
pub fn ov_screen_autosave_shutdown_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x1;
        call_ptr(c, 0x001d_58b8, 0x0021_2528)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x22c));
        let not_1 = c.g[V1][0] != c.g[S2][0];
        c.g[V0][0] = 0x6;
        'out: {
            if !not_1 {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                call_ptr(c, 0x0023_0488, 0x0021_2544)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            'popup_done: {
                'popup_tail: {
                    'not_6: {
                        c.cycles += 2;
                        let ne = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x7;
                        if ne {
                            break 'not_6;
                        }
                        c.cycles += 11;
                        c.g[A1][0] = 0x0047_0000;
                        c.g[A0][0] = 0x4c;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1e70));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x0031_7d70, 0x0021_256c)?;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0019_4738, 0x0021_2578)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        break 'popup_tail;
                    }
                    c.cycles += 2;
                    let ne = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = 0x0047_0000;
                    if ne {
                        break 'popup_done;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = 0x58;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1e80));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x0031_7d70, 0x0021_259c)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x001f_3700, 0x0021_25a8)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                }
                c.cycles += 5;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f400, 0x0021_25b8)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            c.cycles += 2;
            let state_set = c.g[V1][0] != 0u64;
            frame.restore(c, S0, 0x30);
            if state_set {
                break 'out;
            }
            c.cycles += 40;
            call_ptr(c, 0x0014_dc80, 0x0021_25d0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014_dd58, 0x0021_25dc)?;
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dc80, 0x0021_25e4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014_dd58, 0x0021_25f0)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x4;
            call_ptr(c, 0x0014_4df0, 0x0021_2600)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xc;
            call_ptr(c, 0x0014_51e8, 0x0021_260c)?;
            c.g[A1][0] = 0u64;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0014_6d98, 0x0021_2618)?;
            c.g[V1][0] = 0x0053_0000;
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[S2][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x5c08), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0021_2638)?;
            c.g[A1][0] = 0x0047_0000;
            c.g[A0][0] = 0x8;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1e90));
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x0031_7d70, 0x0021_2650)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0023_2720, 0x0021_2658)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0031_6ef8, 0x0021_2664)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenAutosave_HandleInput` (0x0021_22e0): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_autosave_handle_input_002122e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `OVScreenAutosave_OnWidgetAction` (0x0022_22a8): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_autosave_on_widget_action_002222a8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVScreenAutosave_v37` (0x0022_22b0): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_autosave_v37_002222b0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVList_Clear(this)` (0x00212788): unlinks and deletes every node of the
/// list whose sentinel is at `this + 4` (`__builtin_delete` on each node), then
/// makes the sentinel point to itself. Returns nothing.
pub fn ov_list_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        let s = sp(c);
        c.sq(17, s.wrapping_add(0x10));
        c.sq(16, s.wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        let ra = c.g[RA][0];
        let s = sp(c);
        c.sd(s, ra);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[S0][0] = c.lw(c.g[V0][0] as u32);
        if c.g[S0][0] == c.g[V0][0] {
            let s = sp(c);
            c.lq(16, s.wrapping_add(0x20));
        } else {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                call_ptr(c, 0x0031_7e50, 0x0021_27b8)?;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                let t = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0021_27b0);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                let s = sp(c);
                c.lq(16, s.wrapping_add(0x20));
                break;
            }
        }
        c.cycles += 7;
        let v0 = c.g[V0][0];
        c.sw(v0 as u32, v0);
        let s = sp(c);
        c.g[RA][0] = c.ld(s);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let s = sp(c);
        c.lq(17, s.wrapping_add(0x10));
        let v1 = c.g[V1][0];
        c.sw((v1 as u32).wrapping_add(0x4), v1);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_3;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_4;
    pub(crate) use crate::game::fe::ovstatemap::tests::str_hash;
    pub(crate) use crate::game::fe::ovstatepause::tests::close_top_state_once_test;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base as alloc_base_2;
    pub(crate) use crate::game::movie::movieplayer::tests::w as w_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn ov_screen_autosave_v33_matches() {
        close_top_state_once_test(
            ssx3_ee::s05::f_002124c8__generated,
            ov_screen_autosave_v33,
            0x2f,
        );
    }

    /// The MC-manager and UI-manager singletons: zeroed dummies, so the
    /// callees that fetch them do not construct (and allocate) the real ones.
    pub(crate) fn dummy_singletons(c: &mut Cpu) {
        c.bus.write32(gp_at(-0x47c), DATA + 0x3e000);
        c.bus.write32(gp_at(-0x10c8), DATA + 0x3d000);
        clear(c, DATA + 0x3d000, 0x80);
        clear(c, DATA + 0x3e000, 0x200);
    }

    // ------------------------------------------------------------------
    // Front end / profile save.

    #[test]
    fn ov_screen_profile_save_pda_v26_matches() {
        check(
            ee::s05::f_00211220__generated,
            ov_screen_profile_save_pda_v26,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2112_20, 3);
                dummy_singletons(c);
                let this = DATA + 0x100;
                clear(c, this, 0x300);
                c.bus.write32(gp_at(-0x848), 0);
                c.g[A0][0] = sx(this);
                c.bus
                    .write32(this + 0x1c0, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus
                    .write32(this + 0x214, if seed % 3 == 0 { 3 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn ov_screen_autosave_ctor_matches() {
        check(
            ssx3_ee::s05::f_00212138__generated,
            ov_screen_autosave_ctor,
            16,
            &|c, seed| {
                fakes::machine(c, seed, 0x212138, 0);
                // `MemCardStateBase_InstallCallbacks` (through the object ctor)
                // reaches BELibrary_GetInstance: a created dummy keeps it from
                // building the real library (and loading from disc).
                w(c, GAME_GP - 0x1f04, OBJ3);
                w(c, GAME_GP - 0x1efc, 0);
                w(c, 0x0053_08a8, 0);
                c.bus.write32(OBJ, seed);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = OBJ2 as u64;
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_v33_matches() {
        check(
            ssx3_ee::s05::f_00211aa8__generated,
            ov_screen_profile_save_pda_v33,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x211aa8);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x1ac, u32::from(seed % 2 == 0));
                c.bus
                    .write32(DATA + 0x278, if seed % 4 == 0 { 0x18 } else { 0 });
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), 0);
            },
        );
    }

    #[test]
    fn ov_screen_autosave_init_matches() {
        check(
            ee::s05::f_00212198__generated,
            ov_screen_autosave_init,
            30,
            &|c, seed| {
                alloc_base(c, seed, 0x2121_98);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(W + 0x84, 0);
            },
        );
    }

    #[test]
    fn ov_screen_autosave_v26_matches() {
        check(
            ee::s05::f_002122e8__generated,
            ov_screen_autosave_v26,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x2122e8, 5);
                be_library(c);
                world(c);
                heap_init_2(c);
                let this = DATA;
                // `StrHash` clobbers the argument registers: hash first.
                let hash = str_hash(c, 0x0047_1e38);
                c.g[A0][0] = this as u64;
                w_2(c, this + 0x1c0, [0, 6, 0x11][(seed % 3) as usize]);
                if seed % 2 == 0 {
                    w_2(c, this + 0x40, 0);
                } else {
                    let screen = DATA + 0x400;
                    let arr = DATA + 0x500;
                    let child = DATA + 0x600;
                    w_2(c, this + 0x40, screen);
                    w_2(c, screen + 0x38, arr);
                    w_2(c, screen + 0x3c, arr + 8);
                    w_2(c, arr, 1);
                    w_2(c, arr + 8, child);
                    w_2(c, child + 0x38, hash);
                    w_2(c, child + 0x8, DATA + 0x2000);
                    slot(c, DATA + 0x2000, 0x48, 0, FAKE);
                }
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_init_matches() {
        check(
            ee::s05::f_002111a0__generated,
            ov_screen_profile_save_pda_init,
            8,
            &|c, seed| {
                let _ = alloc_base_2(c, seed, 0x2111_a0, 6);
                let engine = DATA + 0x1000;
                w_3(c, DATA + 0x10, engine);
                w_3(c, engine, DATA + 0x1100);
                w_3(c, engine + 4, 0);
                w_3(c, DATA + 0x1100, 0);
                w_3(c, GAME_GP.wrapping_sub(0x9e4), 0);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// A screen whose object list holds `node` with hash 0 (`StrHash("")` is 0
    /// on the test machine: the tag strings are not loaded).
    pub(crate) fn hash_object(c: &mut Cpu, screen: u32, node: u32) {
        w_4(c, screen + 0x38, DATA + 0x32_0100);
        w_4(c, DATA + 0x32_0100, 1);
        w_4(c, screen + 0x3c, DATA + 0x32_0000);
        w_4(c, DATA + 0x32_0000, node);
        w_4(c, node + 0x38, 0);
    }

    #[test]
    fn ov_screen_profile_save_pda_v38_matches() {
        check(
            ee::s05::f_00211970__generated,
            ov_screen_profile_save_pda_v38,
            60,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x211970, 4);
                let obj = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let node = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w_4(c, obj + 0x40, screen);
                hash_object(c, screen, if seed % 5 == 0 { 0 } else { node });
                w_4(c, node + 0x8, vt);
                slot_2(c, vt, 0x48, 6, FAKE);
                w_4(c, obj + 0x220, r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_init_slot_labels_matches() {
        check(
            ee::s05::f_00212080__generated,
            ov_screen_profile_save_pda_init_slot_labels,
            4,
            &|c, seed| {
                let _ = base_4(c, seed, 0x0021_2080, 3);
                c.g[A0][0] = OBJ_2 as u64;
                w_5(c, OBJ_2 + 0x40, OBJ2_2);
                w_5(c, 0x004a_2138, 0);
                w_5(c, 0x0047_1d68, 0);
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes::machine_rt;

    use crate::difftest::fill;
    use crate::difftest::fresh;
    use crate::difftest::run_generated;

    use crate::difftest::Rng;

    use crate::game::fe::festaterewards::tests::base_4 as profile_base;
    pub(crate) use crate::game::fe::ovstatemap::tests::oc988_gate;

    pub(crate) use crate::game::fe::ovstatemap::tests::oc988_names;

    pub(crate) use crate::game::fe::ovstatemap::tests::oc988_screen;
    pub(crate) use crate::game::fe::ovstatemap::tests::oc988_vtable;
    pub(crate) use crate::game::fe::ovstatemap::tests::oc988_w;
    pub(crate) use crate::game::fe::ovstatemap::tests::OC988_CTRL;

    pub(crate) use crate::game::fe::ovstatepause::tests::oc1013_base;

    pub(crate) use crate::game::fe::ovstatepause::tests::oc1013_install_fakes;

    pub(crate) use crate::game::fe::ovstatepause::tests::OC1013_O;

    pub(crate) use crate::game::fe::ovstatetrophy::tests::oc1015_object_setup;

    pub(crate) fn oc988_mailbox(c: &mut Cpu, n: u32, words: [u32; 4]) {
        for (i, w) in words.into_iter().enumerate() {
            c.bus
                .write32(OC988_CTRL + 0x30 + 0x10 * n + 4 * i as u32, w);
        }
    }

    /// `OVScreenProfileSavePDA_RefreshCardInfo`: the card manager (through
    /// `gp-0x47c`) and the screen's methods answer 0 or non-zero per slot, the
    /// six slot widgets and the directory icons are present or missing, and
    /// `StepSlotDir` answers -1 or 0.
    #[test]
    fn ov_screen_profile_save_pda_refresh_card_info_matches() {
        check(
            ee::s05::f_00211380__generated,
            ov_screen_profile_save_pda_refresh_card_info,
            90,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x211380, 40);
                oc988_gate(c);
                let this = DATA + 0x6000;
                let screen = DATA + 0x8000;
                let mgr = DATA + 0x5000;
                let card = DATA + 0x5600;
                let mut names = oc988_names(&mut r, &[0x0047_1d78, 0x0047_00f0], "e");
                for i in 1..7u32 {
                    names.push((
                        DATA + 0x5f00 + 8 * i,
                        format!("s{i}"),
                        r.next_u32() % 5 != 0,
                    ));
                }
                names.extend(oc988_names(&mut r, &[0x0047_1d90, 0x0047_1da0], "f"));
                oc988_screen(c, &mut r, screen, &names);
                // The screen's own vtable (`this + 8`) and the card
                // manager's (`card + 0`): per-slot results 0 or non-zero.
                oc988_vtable(c, &mut r, DATA + 0x5800, 2);
                oc988_vtable(c, &mut r, DATA + 0x5b00, 2);
                oc988_w(c, this + 8, DATA + 0x5800);
                oc988_w(c, this + 0x40, screen);
                oc988_w(c, this + 0x214, if seed % 3 == 0 { 3 } else { 1 });
                oc988_w(c, GAME_GP - 0x47c, mgr);
                oc988_w(c, mgr + 0x428, r.next_u32() % 4);
                oc988_w(c, mgr + 0x434, card);
                oc988_w(c, card, DATA + 0x5b00);
                let neg = 0xffff_ffffu32;
                oc988_mailbox(
                    c,
                    1,
                    [
                        if seed % 2 == 0 { neg } else { 0 },
                        if seed % 4 < 2 { neg } else { 0 },
                        0,
                        0,
                    ],
                );
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_reset_slot_list_matches() {
        check(
            ee::s05::f_00211270__generated,
            ov_screen_profile_save_pda_reset_slot_list,
            200,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0021_1270, 4);
                crate::difftest::elf_image(c);
                let screen = DATA + 0x1000;
                c.g[A0][0] = sx(screen);
                let profile = DATA + 0x1200;
                c.bus.write32(screen + 0x8, profile);
                let vt = DATA + 0x1300;
                let object = DATA + 0x1400;
                c.bus.write32(screen + 0x40, object);
                c.bus.write32(object + 0x8, vt);
                let world = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                fill(c, &mut r, world, 0x40);
            },
        );
    }

    /// The overlay object, its UI screen (hash table at +0x38/+0x3c), a hash
    /// node and the vtable of the widgets. Every string hashes to 0 on the
    /// test machine, so all lookups return the same node (0 = no widget).
    pub(crate) const HIDE_OBJ: u32 = DATA + 0x1000;

    pub(crate) const HIDE_SCREEN: u32 = DATA + 0x2000;

    pub(crate) const HIDE_NODE: u32 = DATA + 0x3000;

    pub(crate) const HIDE_VT: u32 = DATA + 0x4_0000;

    /// The screen with one hash node (`node`, 0 for none) and the widget
    /// vtable: slot 9 at +0x48 (delta 6) calls the fake. A null node reads
    /// its vtable through address 8, as the compiled code does.
    pub(crate) fn hide_buttons_setup(c: &mut Cpu, node: u32) {
        c.bus.write32(HIDE_OBJ + 0x40, HIDE_SCREEN);
        c.bus.write32(HIDE_SCREEN + 0x38, DATA + 0x30_0100);
        c.bus.write32(DATA + 0x30_0100, 1);
        c.bus.write32(HIDE_SCREEN + 0x3c, DATA + 0x30_0000);
        c.bus.write32(DATA + 0x30_0000, node);
        c.bus.write32(node + 0x38, 0);
        c.bus.write32(node + 0x8, HIDE_VT);
        c.bus.write16(HIDE_VT + 0x48, 6);
        c.bus.write32(HIDE_VT + 0x4c, FAKE);
    }

    #[test]
    fn ov_state_profile_hide_action_buttons_matches() {
        check(
            ssx3_ee::s05::f_00211850__generated,
            ov_state_profile_hide_action_buttons,
            60,
            &|c, seed| {
                let mut r = profile_base(c, seed, 0x0021_1850, 3);
                let is_on = if seed % 2 == 0 { 0 } else { r.next_u32() };
                let show = if seed % 3 == 0 { 0 } else { r.next_u32() };
                let on = if is_on == 0 { 1 } else { show };
                let hidden = match seed % 3 {
                    0 => on,
                    1 => on ^ 1,
                    _ => r.next_u32(),
                };
                c.g[A0][0] = sx(HIDE_OBJ);
                c.g[A1][0] = sx(show);
                c.bus.write32(HIDE_OBJ + 0x22c, is_on);
                c.bus.write32(HIDE_OBJ + 0x230, hidden);
                hide_buttons_setup(c, if seed % 5 == 0 { 0 } else { HIDE_NODE });
            },
        );
    }

    #[test]
    fn ov_state_profile_hide_main_button_matches() {
        check(
            ssx3_ee::s05::f_00211a08__generated,
            ov_state_profile_hide_main_button,
            60,
            &|c, seed| {
                let mut r = profile_base(c, seed, 0x0021_1a08, 2);
                c.g[A0][0] = sx(HIDE_OBJ);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(HIDE_OBJ + 0x220, r.next_u32());
                hide_buttons_setup(c, if seed % 5 == 0 { 0 } else { HIDE_NODE });
            },
        );
    }

    /// `__sti__all_in_one_fe_cpp(a0, a1)`: the front end's static initialiser.
    /// With `a1 == 0xffff` and `a0 != 0` it builds the overlay constant tables
    /// (float vectors, hashes, records); otherwise it returns at once.
    pub(crate) fn oc995_sti_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine_rt(c, seed, 0x2_127e8, 0);
        elf_image(c);
        let (a0, a1) = match seed % 4 {
            0 => (0, 0xffff),
            1 => (1, 0x1234),
            _ => (1 + r.next_u32() % 5, 0xffff),
        };
        c.g[A0][0] = sx(a0);
        c.g[A1][0] = a1 as u64;
        c.pc = 0x0021_27e8;
    }

    #[test]
    fn c_ov_state_overlay_static_init_destroy_covers_paths() {
        let mut costs = std::collections::BTreeSet::new();
        for seed in 0..8 {
            let mut c = fresh();
            oc995_sti_setup(&mut c, seed);
            run_generated(&mut c, ee::s05::f_002127e8__generated);
            costs.insert(c.cycles);
        }
        assert!(costs.len() >= 3, "only {} paths: {costs:?}", costs.len());
    }

    /// Clears `len` bytes at `addr` (word stores).
    pub(crate) fn oc997_clear(c: &mut Cpu, addr: u32, len: u32) {
        for off in (0..len).step_by(4) {
            c.bus.write32(addr + off, 0);
        }
    }

    /// The MC-manager and UI-manager singletons at gp-0x47c and gp-0x10c8:
    /// zeroed dummies, so the lookups do not build the real managers.
    pub(crate) fn oc997_singletons(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x47c), DATA + 0x3e000);
        c.bus.write32(GAME_GP.wrapping_sub(0x10c8), DATA + 0x3d000);
        oc997_clear(c, DATA + 0x3d000, 0x80);
        oc997_clear(c, DATA + 0x3e000, 0x200);
    }

    #[test]
    fn ov_screen_autosave_update_icon_matches() {
        check(
            ssx3_ee::s05::f_00212360__generated,
            ov_screen_autosave_update_icon,
            48,
            &|c, seed| {
                fakes::machine(c, seed, 0x0021_2360, 4);
                elf_image(c);
                oc997_singletons(c);
                let this = DATA + 0x100;
                oc997_clear(c, this, 0x300);
                c.g[A0][0] = sx(this);
                // Equal stored and current icon state: the early exit.
                c.bus.write32(this + 0x1a8, seed % 7);
                c.bus.write32(
                    this + 0x230,
                    if seed % 4 == 0 { seed % 7 } else { seed % 5 },
                );
                c.bus
                    .write32(this + 0x1c0, [0, 6, 3, 9][(seed % 4) as usize]);
                c.bus.write32(this + 0x10, DATA + 0x3f800);
                c.bus.write32(this + 0x40, DATA + 0x3f000);
                oc997_clear(c, DATA + 0x3f000, 0x100);
                oc997_clear(c, DATA + 0x3f800, 0x100);
            },
        );
    }

    #[test]
    fn ov_screen_debug_printf_stub_matches() {
        check(
            ssx3_ee::s05::f_00212680__generated,
            ov_screen_debug_printf_stub,
            16,
            &|c, seed| {
                let mut r: Rng = fakes::machine(c, seed, 0x0021_2680, 0);
                for f in [12usize, 14, 16, 18] {
                    c.f[f] = r.next_u32();
                }
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_dtor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221d88__generated,
            ov_screen_profile_save_pda_dtor,
            200,
            &|c, seed| {
                oc1013_base(c, seed, 0x1013_0002);
                c.g[A0][0] = sx(OC1013_O);
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_ctor_2_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221db0__generated,
            ov_screen_profile_save_pda_ctor_2,
            200,
            &|c, seed| {
                let mut r = oc1013_base(c, seed, 0x1013_0003);
                c.g[A0][0] = sx(OC1013_O);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_autosave_dtor_matches() {
        check(
            ee::s05::f_00222280__generated,
            ov_screen_autosave_dtor,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }

    /// The memory-card constructor (0x1d53b0) allocates for real, so the
    /// fixture gets the engine's allocator and the BELibrary dummy first.
    #[test]
    fn ov_screen_autosave_ctor_2_matches() {
        check(
            ee::s05::f_002222b8__generated,
            ov_screen_autosave_ctor_2,
            200,
            &|c, seed| {
                crate::difftest::fakes::machine_rt(c, seed, 0x0022_22b8, 0);
                oc1015_object_setup(c, seed)
            },
        );
    }

    use crate::difftest::check_lohi;
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (the machine's RAM, never
    /// `DATA+0x100000`).
    pub(crate) const OC1021_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC1021_MAGIC: u32 = 0x1021_c0de;

    /// Per-callee result mailbox: 0 keeps the fake's own result, 1 returns 0,
    /// anything else is returned as it is.
    pub(crate) const OC1021_MAIL: u32 = DATA + 0x20_0100;

    pub(crate) const OC1021_ZERO: u32 = 1;

    // Fixture layout.
    /// The state (`this`, a0).
    pub(crate) const OC1021_SELF: u32 = DATA + 0x1000;

    /// The MC manager returned by `MCManager_Get`.
    pub(crate) const OC1021_MC: u32 = DATA + 0x2000;

    pub(crate) const OC1021_WORLD: u32 = DATA + 0x3000;

    pub(crate) const OC1021_WOBJ: u32 = DATA + 0x3100;

    pub(crate) const OC1021_WOBJ2: u32 = DATA + 0x3200;

    pub(crate) const OC1021_A2OBJ: u32 = DATA + 0x3400;

    pub(crate) const OC1021_A2VT: u32 = DATA + 0x3500;

    pub(crate) const OC1021_WV: u32 = DATA + 0x3600;

    /// The widget event (a1).
    pub(crate) const OC1021_EV: u32 = DATA + 0x3800;

    pub(crate) const OC1021_CHAIN: u32 = DATA + 0x3900;

    pub(crate) const OC1021_CHAIN2: u32 = DATA + 0x3a00;

    /// Objects the faked constructor and allocator return.
    pub(crate) const OC1021_KEYBOARD: u32 = DATA + 0x5000;

    pub(crate) const OC1021_ALLOC: u32 = DATA + 0x5800;

    /// A real leaf that returns 0.
    pub(crate) const OC1021_LEAF0: u32 = 0x0014_06c0;

    /// Every direct callee: index, address.
    pub(crate) const OC1021_CALLEES: [u32; 21] = [
        0x002c_2540,
        0x0014_dc80,
        0x0014_dd58,
        0x0041_6810,
        0x0014_7138,
        0x002c_2580,
        0x0041_7828,
        0x0023_cae8,
        0x001c_b030,
        0x0031_7d70,
        0x001c_d088,
        0x001c_e3c8,
        0x001c_e408,
        0x001c_d0b0,
        0x001c_cf70,
        0x0014_7170,
        0x001c_b418,
        0x0039_f290,
        0x0023_d570,
        0x0031_7670,
        0x0022_7f80,
    ];

    pub(crate) const OC1021_IDX_STRLEN: u32 = 3;

    pub(crate) const OC1021_IDX_CTOR: u32 = 8;

    pub(crate) const OC1021_IDX_ALLOC: u32 = 9;

    pub(crate) const OC1021_IDX_MCGET: u32 = 20;

    pub(crate) fn oc1021_apply_mail(c: &mut Cpu, idx: u32) {
        match c.bus.read32(OC1021_MAIL + 4 * idx) {
            0 => {}
            OC1021_ZERO => c.g[V0][0] = 0,
            m => c.g[V0][0] = sx(m),
        }
    }

    /// Gated fakes in the override slots of the direct callees: each logs its
    /// call like `fakes::fake`, then answers from its mailbox; gate off, the
    /// slot runs the body it held.
    macro_rules! oc1021_gated {
        ($($fake:ident => $idx:literal, $slot:path, $generated:path;)*) => {
            $(
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1021_CTRL) != OC1021_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, OC1021_CALLEES[$idx], ra);
                    if c.pc == ra {
                        oc1021_apply_mail(c, $idx);
                    }
                }
            )*

            pub(crate) fn oc1021_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc1021_gated! {
        oc1021_fake_2c2540 => 0, ee::s08::OVERRIDE_002C2540, ee::s08::f_002c2540__generated;
        oc1021_fake_14dc80 => 1, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc1021_fake_14dd58 => 2, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc1021_fake_416810 => 3, ee::s15::OVERRIDE_00416810, ee::s15::f_00416810__generated;
        oc1021_fake_147138 => 4, ee::s01::OVERRIDE_00147138, ee::s01::f_00147138__generated;
        oc1021_fake_2c2580 => 5, ee::s08::OVERRIDE_002C2580, ee::s08::f_002c2580__generated;
        oc1021_fake_417828 => 6, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated;
        oc1021_fake_23cae8 => 7, ee::s06::OVERRIDE_0023CAE8, ee::s06::f_0023cae8__generated;
        oc1021_fake_1cb030 => 8, ee::s04::OVERRIDE_001CB030, ee::s04::f_001cb030__generated;
        oc1021_fake_317d70 => 9, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated;
        oc1021_fake_1cd088 => 10, ee::s04::OVERRIDE_001CD088, ee::s04::f_001cd088__generated;
        oc1021_fake_1ce3c8 => 11, ee::s04::OVERRIDE_001CE3C8, ee::s04::f_001ce3c8__generated;
        oc1021_fake_1ce408 => 12, ee::s04::OVERRIDE_001CE408, ee::s04::f_001ce408__generated;
        oc1021_fake_1cd0b0 => 13, ee::s04::OVERRIDE_001CD0B0, ee::s04::f_001cd0b0__generated;
        oc1021_fake_1ccf70 => 14, ee::s04::OVERRIDE_001CCF70, ee::s04::f_001ccf70__generated;
        oc1021_fake_147170 => 15, ee::s01::OVERRIDE_00147170, ee::s01::f_00147170__generated;
        oc1021_fake_1cb418 => 16, ee::s04::OVERRIDE_001CB418, ee::s04::f_001cb418__generated;
        oc1021_fake_39f290 => 17, ee::s13::OVERRIDE_0039F290, ee::s13::f_0039f290__generated;
        oc1021_fake_23d570 => 18, ee::s06::OVERRIDE_0023D570, ee::s06::f_0023d570__generated;
        oc1021_fake_317670 => 19, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated;
        oc1021_fake_227f80 => 20, ee::s05::OVERRIDE_00227F80, ee::s05::f_00227f80__generated;
    }

    pub(crate) fn oc1021_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// `OVScreenProfileSavePDA_OnWidgetAction(this, event, action)`: the PDA
    /// profile-save screen's widget handler. `action - 5` (0..9) picks one of
    /// five arms through the jump table (name edit, keyboard popup, MC result
    /// polling), anything else does nothing. The seed varies the action (also
    /// out of range), the state machine fields (`+0x1a8`, `+0x1c0`, `+0x214`),
    /// whether the keyboard popup exists, and what the memory-card manager's
    /// vtable calls and `strlen` answer.
    pub(crate) fn oc1021_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine_rt(c, seed, 0x211bc8, 16);
        elf_image(c);
        oc1021_install_fakes();
        c.bus.write32(OC1021_CTRL, OC1021_MAGIC);
        c.pc = 0x0021_1bc8;
        let gp = GAME_GP;
        let pick =
            |r: &mut crate::difftest::Rng, set: &[u32]| set[(r.next_u32() as usize) % set.len()];

        // Action 3..=16: the ten table entries and both out-of-range sides.
        let action = 3 + seed % 14;
        let event = if r.next_u32() % 5 == 0 { 0 } else { OC1021_EV };
        oc1021_w(c, gp - 0x848, OC1021_WORLD);
        oc1021_w(c, OC1021_WORLD + 0x8c, OC1021_WOBJ);
        oc1021_w(c, OC1021_WOBJ + 4, OC1021_WOBJ2);
        c.bus.write16(OC1021_WOBJ2 + 0x20, 0);
        oc1021_w(c, OC1021_WOBJ2 + 0x24, FAKE + 0x10);

        let this = OC1021_SELF;
        oc1021_w(c, this + 0x10, DATA + 0x6000);
        c.bus.write8(this + 0x15, r.next_u32() as u8);
        oc1021_w(c, this + 0x19c, r.next_u32());
        oc1021_w(c, this + 0x1a0, r.next_u32());
        let keyboard = if r.next_u32() % 2 == 0 {
            0
        } else {
            OC1021_KEYBOARD
        };
        oc1021_w(c, this + 0x1a4, keyboard);
        oc1021_w(c, this + 0x1a8, pick(&mut r, &[0, 1, 1, 2]));
        oc1021_w(c, this + 0x1c0, pick(&mut r, &[0, 1, 1, 2, 3, 6, 6]));
        oc1021_w(c, this + 0x1c4, r.next_u32());
        oc1021_w(c, this + 0x1cc, r.next_u32() % 16);
        oc1021_w(c, this + 0x1dc, r.next_u32());
        oc1021_w(c, this + 0x214, pick(&mut r, &[3, 3, 0]));
        oc1021_w(c, this + 0x40, OC1021_CHAIN);
        oc1021_w(c, OC1021_CHAIN + 0xd0, OC1021_CHAIN2);
        oc1021_w(c, OC1021_KEYBOARD + 0x58, 0);

        // The memory-card manager and the objects its vtable calls reach.
        oc1021_w(c, OC1021_MC + 0x434, OC1021_A2OBJ);
        oc1021_w(c, OC1021_MC + 0x428, r.next_u32());
        oc1021_w(c, OC1021_MC + 0x748, OC1021_WV);
        oc1021_w(c, OC1021_WV + 0xc, FAKE + 0x20);
        oc1021_w(c, OC1021_A2OBJ, OC1021_A2VT);
        oc1021_w(c, OC1021_EV + 0x18, r.next_u32() % 4);
        for off in [0x1b4, 0x21c] {
            let f = if r.next_u32() % 2 == 0 {
                OC1021_LEAF0
            } else {
                FAKE + 0x30
            };
            oc1021_w(c, OC1021_A2VT + off, f);
        }

        // Callee answers.
        let mail = |c: &mut Cpu, idx: u32, v: u32| oc1021_w(c, OC1021_MAIL + 4 * idx, v);
        mail(c, OC1021_IDX_MCGET, OC1021_MC);
        mail(c, OC1021_IDX_ALLOC, OC1021_ALLOC);
        mail(c, OC1021_IDX_CTOR, OC1021_KEYBOARD);
        mail(
            c,
            OC1021_IDX_STRLEN,
            if r.next_u32() % 2 == 0 {
                OC1021_ZERO
            } else {
                5
            },
        );

        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(event);
        c.g[A2][0] = sx(action);
    }

    /// The port matches the generated body: every action, state, popup and
    /// MC-manager outcome, and unwinds inside the call chains.
    #[test]
    fn ov_screen_profile_save_pda_on_widget_action_matches() {
        check(
            ee::s05::f_00211bc8__generated,
            ov_screen_profile_save_pda_on_widget_action_2,
            420,
            &oc1021_setup,
        );
    }

    /// The fixture reaches many distinct paths (total cycle costs of the
    /// generated body over the seeds).
    #[test]
    fn ov_screen_profile_save_pda_on_widget_action_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..420u32 {
            let mut c = fresh();
            oc1021_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s05::f_00211bc8__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 12, "only {} paths: {seen:?}", seen.len());
    }

    /// Gate word of this module's fakes (own address and magic in the machine's
    /// RAM: other tests' machines run the real bodies).
    pub(crate) const OC1024_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC1024_MAGIC: u32 = 0x1024_f00d;

    /// Fixture objects (clear of the fakes' result pointers, `DATA..+0x40000`).
    pub(crate) const OC1024_THIS: u32 = DATA + 0x5_0000;

    pub(crate) const OC1024_MGR: u32 = DATA + 0x5_4000;

    /// Every faked callee: a gated fake in its override slot that logs the
    /// call (all registers) and returns the next forced result or a pointer.
    macro_rules! oc1024_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1024_CTRL) != OC1024_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                    oc1024_after(c, $addr);
                }
            )*
            /// Puts every fake in its override slot (once per process).
            fn oc1024_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    /// `SndLoadQueue_FinishCurrent` fake: each call lets the busy word (+4 of
    /// `s0`) count down by one.
    pub(crate) fn oc1024_after(c: &mut Cpu, target: u32) {
        if target == 0x0028_9df0 {
            let this = c.g[S0][0] as u32;
            let busy = c.bus.read32(this + 4);
            if busy != 0 {
                c.bus.write32(this + 4, busy - 1);
            }
        }
    }

    oc1024_fakes! {
        oc1024_fake_001d58b8, 0x001d_58b8, ee::s04::OVERRIDE_001D58B8, ee::s04::f_001d58b8__generated;
        oc1024_fake_00227f80, 0x0022_7f80, ee::s05::OVERRIDE_00227F80, ee::s05::f_00227f80__generated;
        oc1024_fake_0023cae8, 0x0023_cae8, ee::s06::OVERRIDE_0023CAE8, ee::s06::f_0023cae8__generated;
        oc1024_fake_001d8de0, 0x001d_8de0, ee::s04::OVERRIDE_001D8DE0, ee::s04::f_001d8de0__generated;
        oc1024_fake_00230488, 0x0023_0488, ee::s06::OVERRIDE_00230488, ee::s06::f_00230488__generated;
        oc1024_fake_00317d70, 0x0031_7d70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated;
        oc1024_fake_00194738, 0x0019_4738, ee::s02::OVERRIDE_00194738, ee::s02::f_00194738__generated;
        oc1024_fake_001f3700, 0x001f_3700, ee::s04::OVERRIDE_001F3700, ee::s04::f_001f3700__generated;
        oc1024_fake_0039f400, 0x0039_f400, ee::s13::OVERRIDE_0039F400, ee::s13::f_0039f400__generated;
        oc1024_fake_0014dc80, 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc1024_fake_0014dd58, 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc1024_fake_00144df0, 0x0014_4df0, ee::s01::OVERRIDE_00144DF0, ee::s01::f_00144df0__generated;
        oc1024_fake_001451e8, 0x0014_51e8, ee::s01::OVERRIDE_001451E8, ee::s01::f_001451e8__generated;
        oc1024_fake_00146d98, 0x0014_6d98, ee::s01::OVERRIDE_00146D98, ee::s01::f_00146d98__generated;
        oc1024_fake_00232720, 0x0023_2720, ee::s06::OVERRIDE_00232720, ee::s06::f_00232720__generated;
        oc1024_fake_00316ef8, 0x0031_6ef8, ee::s10::OVERRIDE_00316EF8, ee::s10::f_00316ef8__generated;
        oc1024_fake_003e4fb0, 0x003e_4fb0, ee::s14::OVERRIDE_003E4FB0, ee::s14::f_003e4fb0__generated;
        oc1024_fake_00289df0, 0x0028_9df0, ee::s07::OVERRIDE_00289DF0, ee::s07::f_00289df0__generated;
        oc1024_fake_00317e98, 0x0031_7e98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated;
        oc1024_fake_0028b278, 0x0028_b278, ee::s07::OVERRIDE_0028B278, ee::s07::f_0028b278__generated;
        oc1024_fake_002ace40, 0x002a_ce40, ee::s08::OVERRIDE_002ACE40, ee::s08::f_002ace40__generated;
        oc1024_fake_0028a148, 0x0028_a148, ee::s07::OVERRIDE_0028A148, ee::s07::f_0028a148__generated;
        oc1024_fake_002adee8, 0x002a_dee8, ee::s08::OVERRIDE_002ADEE8, ee::s08::f_002adee8__generated;
        oc1024_fake_00317e50, 0x0031_7e50, ee::s10::OVERRIDE_00317E50, ee::s10::f_00317e50__generated;
    }

    /// Hooked fakes, scrambled registers, the gate on.
    pub(crate) fn oc1024_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1024_hook();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1024_CTRL, OC1024_MAGIC);
        r
    }

    /// Distinct call sequences (addresses) the generated body makes over the seeds.
    pub(crate) fn oc1024_paths(
        generated: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, generated);
            let n = c.bus.read32(fakes::LOG_COUNT);
            let seq: Vec<u32> = (0..n)
                .map(|i| c.bus.read32(fakes::LOG_START + i * 0x220))
                .collect();
            seen.insert((seq, c.cycles));
        }
        seen.len()
    }

    pub(crate) fn oc1024_shutdown_pda_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1024_machine(c, seed, 0x21_1b10, 1);
        c.bus.write32(OC1024_THIS + 0x1a4, r.next_u32());
        c.g[A0][0] = sx(OC1024_THIS);
        c.pc = 0x0021_1b10;
    }

    /// `OVScreenProfileSavePDA_Shutdown`: the base shutdown call, the field
    /// clear, and a callee unwind.
    #[test]
    fn ov_screen_profile_save_pda_shutdown_matches() {
        check(
            ee::s05::f_00211b10__generated,
            ov_screen_profile_save_pda_shutdown_2,
            40,
            &oc1024_shutdown_pda_setup,
        );
    }

    pub(crate) fn oc1024_pda_notify_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1024_machine(c, seed, 0x21_1b38, 2);
        let key = [6u32, 8, 9, 0, 5, 7, 10, 0x101, 3, 6, 8, 9, 4, 6][(seed % 14) as usize];
        let (obj, vt) = (DATA + 0x5_1000, DATA + 0x5_2000);
        // MCManager_Get is fake call 0: force the manager.
        c.bus.write32(fakes::LOG_RESULTS, OC1024_MGR);
        c.bus.write32(OC1024_MGR + 0x434, obj);
        c.bus.write32(OC1024_MGR + 0x428, r.next_u32());
        c.bus.write32(obj, vt);
        c.bus.write16(vt + 0x1b0, (r.next_u32() & 0xfffc) as u16);
        // The device answers non-zero (a fake) or zero (a real leaf).
        let leaf0 = 0x0014_06c0;
        c.bus.write32(
            vt + 0x1b4,
            if (seed / 14) % 2 == 0 {
                fakes::FAKE
            } else {
                leaf0
            },
        );
        c.g[A0][0] = r.next_u32() as u64;
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = key as u64;
        c.pc = 0x0021_1b38;
    }

    /// `OVScreenProfileSavePDA_OnWidgetNotify`: keys 6/8 (device actionable or
    /// not), 9, below and above, and callee unwinds.
    #[test]
    fn ov_screen_profile_save_pda_on_widget_notify_matches() {
        check(
            ee::s05::f_00211b38__generated,
            ov_screen_profile_save_pda_on_widget_notify_2,
            140,
            &oc1024_pda_notify_setup,
        );
        let paths = oc1024_paths(
            ee::s05::f_00211b38__generated,
            140,
            &oc1024_pda_notify_setup,
        );
        assert!(paths >= 5, "only {paths} paths");
    }

    pub(crate) fn oc1024_autosave_update_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1024_machine(c, seed, 0x21_2208, 3);
        c.bus.write32(fakes::LOG_RESULTS, OC1024_MGR);
        // (state, +0x1a8 word, device variant)
        let k = seed % 16;
        let (state, f1a8, variant) = match k {
            0..=2 => (1u32, 0u32, 0u32),
            3 => (1, 5, 0),
            4 => (0, 0, 0),
            5 => (6, 0, 0),
            _ => (3, 0, k - 6),
        };
        let this = OC1024_THIS;
        c.bus.write32(this + 0x1c0, state);
        c.bus.write32(this + 0x1a8, f1a8);
        for off in [0x1f0u32, 0x1dc, 0x1e4] {
            c.bus.write32(this + off, r.next_u32());
        }
        // The selected device record (0xe0 bytes each): plugged (+0x350),
        // valid (+0x344), formatted (+0x348).
        let idx = (seed / 16) % 3;
        c.bus.write32(OC1024_MGR + 0x428, idx);
        let dev = OC1024_MGR + idx * 0xe0;
        let (plugged, valid, formatted) =
            [(0u32, 0u32, 0u32), (1, 0, 0), (1, 1, 1), (1, 1, 0)][(variant % 4) as usize];
        c.bus.write32(dev + 0x350, plugged);
        c.bus.write32(dev + 0x344, valid);
        c.bus.write32(dev + 0x348, formatted);
        c.g[A0][0] = sx(this);
        c.pc = 0x0021_2208;
    }

    /// `OVScreenAutosave_Update`: state 1 (start save, or already started),
    /// state 3 with the device unplugged / invalid / formatted / not, other
    /// states, and callee unwinds. LO/HI too (the device index multiply).
    #[test]
    fn ov_screen_autosave_update_matches() {
        check_lohi(
            ee::s05::f_00212208__generated,
            ov_screen_autosave_update_2,
            192,
            &oc1024_autosave_update_setup,
        );
        let paths = oc1024_paths(
            ee::s05::f_00212208__generated,
            192,
            &oc1024_autosave_update_setup,
        );
        assert!(paths >= 5, "only {paths} paths");
    }

    pub(crate) fn oc1024_autosave_shutdown_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1024_machine(c, seed, 0x21_2508, 14);
        let state = [1u32, 6, 7, 0, 3, 0, 6, 7, 1, 2][(seed % 10) as usize];
        let this = OC1024_THIS;
        c.bus.write32(this + 0x22c, state);
        c.bus.write32(this + 0x10, r.next_u32());
        c.g[A0][0] = sx(this);
        c.pc = 0x0021_2508;
    }

    /// `OVScreenAutosave_Shutdown`: state 1, 6, 7, 0 (the long rebuild chain,
    /// ending in a virtual call through a zero vtable word) and others, with
    /// callee unwinds at random positions.
    #[test]
    fn ov_screen_autosave_shutdown_matches() {
        check(
            ee::s05::f_00212508__generated,
            ov_screen_autosave_shutdown_2,
            200,
            &oc1024_autosave_shutdown_setup,
        );
        let paths = oc1024_paths(
            ee::s05::f_00212508__generated,
            200,
            &oc1024_autosave_shutdown_setup,
        );
        assert!(paths >= 5, "only {paths} paths");
    }

    pub(crate) use crate::game::fe::ovstatereplay::tests::GATE_MAGIC;

    /// `OVScreenAutosave_HandleInput` (0x0021_22e0) against the generated body.
    #[test]
    fn ov_screen_autosave_handle_input_002122e0_matches() {
        check(
            ee::s05::f_002122e0__generated,
            ov_screen_autosave_handle_input_002122e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2122e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenAutosave_OnWidgetAction` (0x0022_22a8) against the generated body.
    #[test]
    fn ov_screen_autosave_on_widget_action_002222a8_matches() {
        check(
            ee::s05::f_002222a8__generated,
            ov_screen_autosave_on_widget_action_002222a8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2222a8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenAutosave_v37` (0x0022_22b0) against the generated body.
    #[test]
    fn ov_screen_autosave_v37_002222b0_matches() {
        check(
            ee::s05::f_002222b0__generated,
            ov_screen_autosave_v37_002222b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2222b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// Each fake's gate word in the test machine's RAM, and the magic that opens it.
    pub(crate) const GATE_DELETE: u32 = DATA + 0x20_1542;

    /// `__builtin_delete` (0x317e50), gated: logs the call and returns.
    pub(crate) fn oc1542_fake_delete(c: &mut Cpu) {
        if c.bus.read32(GATE_DELETE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1542_fake_delete),
                ee::s10::f_00317e50__generated,
            );
        }
        fakes::fake_call(c, 0x0031_7e50);
    }

    #[test]
    fn ov_list_clear_matches() {
        check(
            ee::s05::f_00212788__generated,
            ov_list_clear,
            300,
            &|c, seed| {
                machine(c, seed, 0x1542_2788, 5);
                fakes::hook(
                    addr_of_mut!(ee::s10::OVERRIDE_00317E50),
                    oc1542_fake_delete,
                    fakes::prev_cell(oc1542_fake_delete),
                );
                c.bus.write32(GATE_DELETE, GATE_MAGIC);
                let obj = DATA + 0x20_2000;
                let head = DATA + 0x20_2100;
                let count = seed % 5;
                let mut at = head;
                for i in 0..count {
                    let node = DATA + 0x20_2200 + 0x20 * i;
                    c.bus.write32(at, node);
                    at = node;
                }
                c.bus.write32(at, head);
                c.bus.write32(obj + 0x4, head);
                c.g[A0][0] = sx(obj);
            },
        );
    }
}
