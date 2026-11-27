//! `src/fe/ovstatetrophy.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::rider::world;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp_2_m2;
pub(crate) use crate::game::fe::feasyncfile::p as p_3;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_4;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::gp as gp_2;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::festatenethelppopup::fe_state_screen_init;
pub(crate) use crate::game::fe::festaterewards::p as p_m2;
pub(crate) use crate::game::fe::festaterewards::r;
pub(crate) use crate::game::fe::ovstaterewardslist::ov_screen_widget_action;
pub(crate) use crate::game::fe::ovtemplatepausemenu::ov_state_dtor_body;
pub(crate) use crate::game::fe::ovtemplatepausemenu::ov_state_fill_fields;
pub(crate) use crate::game::fe::ovtemplatepausemenu::OV_STATE_UI_CTOR;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
#[cfg(test)]
pub(crate) use crate::game::render::bezierman::f;
pub(crate) use crate::game::render::bezierman::gp as gp_m2;
pub(crate) use crate::game::world::worldview::p_2 as p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001c_b418,
        c_keyboard_popup_set_text,
        "cKeyboardPopup_SetText",
    ),
    (
        0x001c_cf00,
        c_keyboard_popup_set_static_text,
        "cKeyboardPopup_SetStaticText",
    ),
    (
        0x001c_d088,
        c_keyboard_popup_set_max_length,
        "cKeyboardPopup_SetMaxLength",
    ),
    (
        0x001c_e3c8,
        c_keyboard_popup_set_key_enabled,
        "cKeyboardPopup_SetKeyEnabled",
    ),
    (
        0x001c_e468,
        c_keyboard_popup_set_function_keys_enabled,
        "cKeyboardPopup_SetFunctionKeysEnabled",
    ),
    (0x0020_e938, ov_screen_trophy_init, "OVScreenTrophy_Init"),
    (0x0020_eaa0, hud_meter_ctor, "HUDMeter_ctor"),
    (0x0020_eb08, hud_meter_dtor, "HUDMeter_dtor"),
    (
        0x0020_eb50,
        cov_state_hud1p_load_meter_assets,
        "cOVStateHUD1P_LoadMeterAssets",
    ),
    (
        0x0021_05b0,
        race_progress_reset_rider,
        "RaceProgress_ResetRider",
    ),
    (
        0x0021_05d0,
        race_progress_set_path_data,
        "RaceProgress_SetPathData",
    ),
    (
        0x0021_0608,
        race_progress_clear_path_data,
        "RaceProgress_ClearPathData",
    ),
    (
        0x0021_0d20,
        ov_screen_read_private_message_update,
        "OVScreenReadPrivateMessage_Update",
    ),
    (
        0x0021_0dd0,
        ov_screen_read_private_message_on_child_event,
        "OVScreenReadPrivateMessage_OnChildEvent",
    ),
    (
        0x0020_eda0,
        c_ov_state_hud1p_draw_progress_meter,
        "cOVStateHUD1P_DrawProgressMeter",
    ),
    (
        0x0020_ed20,
        hud1p_update_race_progress,
        "cOVStateHUD1P_UpdateRaceProgress",
    ),
    (
        0x0021_0618,
        race_progress_update_rider,
        "RaceProgress_UpdateRider",
    ),
    (
        0x0021_0820,
        race_progress_project_rider,
        "RaceProgress_ProjectRider",
    ),
    (
        0x0021_08f8,
        race_progress_reset_all,
        "RaceProgress_ResetAll",
    ),
    (
        0x0021_0f40,
        ov_screen_read_private_message_open_keyboard,
        "OVScreenReadPrivateMessage_OpenKeyboard",
    ),
    (
        0x0021_0fa0,
        ovscreen_read_private_message_create_keyboard_popup,
        "OVScreenReadPrivateMessage_CreateKeyboardPopup",
    ),
    (0x0020_ec18, hud_meter_set_rect_2, "HUDMeter_SetRect"),
    (
        0x0020_e9a0,
        ov_screen_trophy_on_widget_action,
        "OVScreenTrophy_OnWidgetAction",
    ),
    (
        0x0020_fb40,
        c_ov_state_hud1_p_draw_progress_meter_mp,
        "cOVStateHUD1P_DrawProgressMeterMP",
    ),
    (
        0x0021_0940,
        ov_screen_read_private_message_init,
        "OVScreenReadPrivateMessage_Init",
    ),
    (
        0x0021_0b58,
        ov_screen_read_private_message_transition_in,
        "OVScreenReadPrivateMessage_TransitionIn",
    ),
    (
        0x0021_0bd8,
        ov_screen_read_private_message_set_message_text,
        "OVScreenReadPrivateMessage_SetMessageText",
    ),
    (
        0x0021_0df0,
        ov_screen_read_private_message_on_widget_action,
        "OVScreenReadPrivateMessage_OnWidgetAction",
    ),
    (
        0x0021_1088,
        ov_screen_profile_save_pda_ctor,
        "OVScreenProfileSavePDA_ctor",
    ),
    (
        0x0022_1408,
        ov_screen_read_private_message_ctor,
        "OVScreenReadPrivateMessage_ctor",
    ),
    (
        0x0022_1468,
        ov_screen_read_private_message_ctor_2,
        "OVScreenReadPrivateMessage_ctor_2",
    ),
    (
        0x0022_14d0,
        ov_screen_read_private_message_dtor,
        "OVScreenReadPrivateMessage_dtor",
    ),
    (0x0022_22f8, ov_screen_trophy_ctor, "OVScreenTrophy_ctor"),
    (
        0x0022_2358,
        ov_screen_trophy_ctor_2,
        "OVScreenTrophy_ctor_2",
    ),
    (0x0022_23c0, ov_screen_trophy_dtor, "OVScreenTrophy_dtor"),
];

// ---------------------------------------------------------------------------
// Callees (address + the generated shard function, which dispatches through
// its override slot). Entries for calls that the generator emitted as
// `call_indirect` are kept for reference next to the `call_ptr` sites.

#[allow(dead_code)]
pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::RACE_PROGRESS_RESET_ALL;

    pub(crate) use crate::game::bx::bxstringctor::callee::OV_STATE_OVERLAY_UPDATE;

    pub(crate) use crate::game::bx::bxstringctor::callee::READ_PRIVATE_MESSAGE_SET_TEXT;

    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::render::bezierman::callee::PROJECT_RIDER;
    pub(crate) use crate::game::render::bezierman::callee::RIDER_GET_DIST_TO_FINISH;
}

// ---------------------------------------------------------------------------
// Keyboard popup.

/// `void cKeyboardPopup_SetMaxLength(cKeyboardPopup *this, int max)` (0x1cd088):
/// stores `max` (if 0 <= max < 64), else 0x3f for a non-negative value, else 0.
pub fn c_keyboard_popup_set_max_length(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    let max = c.g[A1][0];
    c.cycles += 2;
    let non_negative = (max as i64) >= 0;
    c.g[V0][0] = ((max as i64) < 64) as u64;
    if non_negative {
        c.cycles += 2;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x3f;
        c.cycles += 2;
        if in_range {
            this.off(0x60).sw(c, max);
        } else {
            this.off(0x60).sw(c, c.g[V0][0]);
        }
    } else {
        c.cycles += 2;
        this.off(0x60).sw(c, 0);
    }
    abi::ret(c);
}

/// `void cKeyboardPopup_SetKeyEnabled(cKeyboardPopup *this, int index, int value, int key)`
/// (0x1ce3c8): stores `value` into the per-key table at +0x148 (key 0 or 2) or
/// +0x298 (key 1 or 2), leaving `index*4 + this` in v0.
pub fn c_keyboard_popup_set_key_enabled(c: &mut Cpu) {
    let this = c.g[A0][0];
    let index = c.g[A1][0];
    let value = c.g[A2][0];
    let key = c.g[A3][0];
    c.cycles += 2;
    let key_is_zero = key == 0;
    c.g[V0][0] = 2;
    let mut store_148 = false;
    if key_is_zero {
        store_148 = true;
    } else {
        c.cycles += 2;
        let key_not_two = key != c.g[V0][0];
        c.g[V0][0] = sx((key as u32).wrapping_sub(1));
        if !key_not_two {
            store_148 = true;
        }
    }
    if store_148 {
        c.cycles += 4;
        c.g[V0][0] = sx((index as u32) << 2);
        c.g[V0][0] = sx((this as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[V0][0] as u32).off(0x148).sw(c, value);
        c.g[V0][0] = sx((key as u32).wrapping_sub(1));
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    let skip = c.g[V0][0] == 0;
    c.g[V0][0] = sx((index as u32) << 2);
    if !skip {
        c.cycles += 2;
        c.g[V0][0] = sx((this as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[V0][0] as u32).off(0x298).sw(c, value);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void RaceProgress_ResetRider(int rider)` (0x2105b0): stores 1 into the
/// per-rider record at `rider*0x18 + 0x4c8bc8`.
pub fn race_progress_reset_rider(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V1][0] = 0x18;
    c.g[V0][0] = 0x4d0000;
    let scaled = abi::mult(c, abi::r32(c, A0), 0x18);
    c.g[A0][0] = sx(scaled);
    c.g[V0][0] = sx(0x004d_0000u32.wrapping_sub(0x7438));
    c.g[V1][0] = 1;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void RaceProgress_ClearPathData(void)` (0x210608): clears the three
/// gp-relative path pointers.
pub fn race_progress_clear_path_data(c: &mut Cpu) {
    c.cycles += 4;
    Addr(abi::r32(c, GP).wrapping_sub(0x9a0)).sw(c, 0);
    Addr(abi::r32(c, GP).wrapping_sub(0x99c)).sw(c, 0);
    Addr(abi::r32(c, GP).wrapping_sub(0x998)).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Race progress and game state.

/// `void RaceProgress_SetPathData(RaceProgress* self, FileBlock* path)`
/// (0x2105d0): record the path block, its data and the course-name string
/// in the progress globals, then reset all progress.
pub fn race_progress_set_path_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr((c.g[GP][0] as u32).wrapping_sub(0x9a0)).sw(c, c.g[A0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr((c.g[GP][0] as u32).wrapping_sub(0x99c)).sw(c, c.g[V0][0]);
        Addr((c.g[GP][0] as u32).wrapping_sub(0x998)).sw(c, c.g[A0][0]);
        callee::RACE_PROGRESS_RESET_ALL.call(c, 0x0021_05f8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void HUDMeter_dtor(self, flags)` (0x20eb08): drop the HUD meter
/// (`+0x38` gets its base vtable), decrement the meter count (`gp-0x984`) —
/// clearing the "meters loaded" flag (`gp-0x988`) on the last one — and free
/// `self` when `flags & 1`.
pub fn hud_meter_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(gp(c, -0x984)).lw(c);
        c.g[V0][0] = 0x47_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x47_0000u32.wrapping_add(0x4230));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let self_ = abi::r32(c, A0);
        Addr(self_.wrapping_add(0x38)).sw(c, c.g[V0][0]);
        let more = c.g[V1][0] != 0;
        Addr(gp(c, -0x984)).sw(c, c.g[V1][0]);
        if !more {
            c.cycles += 1;
            Addr(gp(c, -0x988)).sw(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A1][0] & 1;
        let keep = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !keep {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e50, 0x0020_eb44)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenReadPrivateMessage_OnChildEvent(OVScreen*, int id, int ev)`
/// (0x00210dd0): the event 0x16 clears the pending message id at +0xe0.
pub fn ov_screen_read_private_message_on_child_event(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = abi::arg(c, 0);
    let id = abi::arg64(c, 1);
    c.g[V0][0] = 0x16;
    if abi::arg64(c, 2) == 0x16 {
        c.cycles += 3;
        c.g[V0][0] = Addr(p(self_, 0xe0)).lw(c);
        if id == c.g[V0][0] {
            c.sw(p(self_, 0xe0), 0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void cKeyboardPopup_SetFunctionKeysEnabled(cKeyboardPopup* self, int on)`
/// (0x1ce468): enable/disable the fourteen function keys (indices 0..0xd).
pub fn c_keyboard_popup_set_function_keys_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = 2;
            call_ptr(c, 0x001c_e3c8, 0x001c_e4a0)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 14i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_e490);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenTrophy_Init(OVScreenTrophy* self)` (0x20e938).
pub fn ov_screen_trophy_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0047_1cd8, [0x0020_e95c, 0x0020_e970, 0x0020_e988])
    })
}

/// `void HUDMeter_ctor(HUDMeter* self)` (0x20eaa0): install the vtable, set a
/// full-screen 640x480 rectangle and hand it to `HUDMeter_SetRect` as both
/// the source and the destination.
pub fn hud_meter_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x30);
        c.g[AT][0] = 0x4420_0000;
        c.mtc1(1, 0);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x470000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[AT][0] = 0x43f0_0000;
        c.mtc1(1, 1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4230));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x38), c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[A2][0] = c.g[SP][0];
        c.sw(c.g[SP][0] as u32, 0u64);
        c.g[RA][0] = sx(0x0020_eae8);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
        call_ptr(c, 0x0020_ec18, 0x0020_eae8)?;
        c.g[V1][0] = c.lw(gp_2(c, -0x984));
        c.g[V0][0] = c.g[S0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw(gp_2(c, -0x984), c.g[V1][0]);
        close(c, 0x30);
        Ok(())
    })
}

/// `cKeyboardPopup::SetStaticText(self, text)` (0x1ccf00): with no screen yet,
/// keep the string in the popup; otherwise set it on the found text object.
pub fn c_keyboard_popup_set_static_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p_3(c.g[S0][0] as u32, 0x40)).lw(c);
        let have_screen = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !have_screen {
            c.cycles += 4;
            c.g[A0][0] = sx(p_3(c.g[S0][0] as u32, 0x424));
            call_ptr(c, 0x0031_84e8, 0x001c_cf28)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 8;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6580));
            call_ptr(c, 0x0031_7670, 0x001c_cf3c)?;
            c.g[A0][0] = Addr(p_3(c.g[S0][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001c_cf48)?;
            let missing = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !missing {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003a_0c50, 0x001c_cf58)?;
            }
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

/// `void OVScreenReadPrivateMessage_TransitionIn(self, has_sender)`
/// (0x210b58): when a sender is given, copy its message into the screen;
/// otherwise open the on-screen keyboard for a new message.
pub fn ovscreenreadprivatemessage_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let no_sender = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if !no_sender {
            c.cycles += 7;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
            c.g[A0][0] = lw_at(c, gw(c, -0x204));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0x41;
            call_ptr(c, callee::C_GAME_COMM_RECV_GAME_PACKET.addr, 0x0021_0b84)?;
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !empty {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0;
                call_ptr(c, 0x0021_0bd8, 0x0021_0b98)?;
                c.cycles += 5;
                c.g[V0][0] = 1;
                frame.restore(c, 16, 0x20);
                frame.restore(c, 17, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 8;
        c.g[A2][0] = 0x4a2560;
        c.g[A1][0] = 0;
        c.g[T0][0] = 0;
        c.g[A3][0] = c.g[A2][0];
        c.g[T1][0] = 0;
        c.g[T2][0] = 0x3d;
        call_ptr(c, 0x0021_0f40, 0x0021_0bc0)?;
        c.cycles += 1;
        frame.restore(c, 16, 0x20);
        c.cycles += 5;
        c.g[V0][0] = 1;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `OVScreenReadPrivateMessage_Update` (0x210d20): run the overlay update; if
// the net object is idle, close its popups and set the message flag; when the
// screen has content, read the pending message into it.
pub fn ov_screen_read_private_message_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::OV_STATE_OVERLAY_UPDATE.call(c, 0x0021_0d38)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
        let no_net = c.g[V1][0] == 0;
        if no_net {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x68));
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = 0u64;
            let mut closed = idle;
            if !idle {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                closed = c.g[V0][0] == 0;
            }
            if closed {
                c.cycles += 1;
                c.g[A0][0] = 0x1;
            }
            c.cycles += 2;
            if !closed {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            } else {
                c.cycles += 9;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039f698, 0x0021_0d74)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[V1][0] = c.g[V1][0] | 0x780;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            }
        }
        c.cycles += 2;
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
        if !empty {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0x41;
            call_ptr(c, 0x00258160, 0x0021_0da4)?;
            let no_msg = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !no_msg {
                c.cycles += 3;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0u64;
                callee::READ_PRIVATE_MESSAGE_SET_TEXT.call(c, 0x0021_0db8)?;
            }
        }
        c.cycles += 5;
        c.lq(16, frame.at(c, 0x20));
        c.lq(17, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cOVStateHUD1P_LoadMeterAssets(self, a1, a2)` (0x20eb50): find the
/// meter textures/font and apply the rect.
pub fn cov_state_hud1p_load_meter_assets(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(gp_3(c, -0x988)).lw(c);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !ready {
            c.cycles += 7;
            c.g[V0][0] = sx(world(c));
            c.g[V1][0] = 0x004d_0000;
            c.g[S0][0] = 0x004c_8c58;
            c.g[S2][0] = 0xa;
            c.g[A0][0] = Addr(p_4(c.g[V0][0], 0x84)).lw(c);
            c.g[S1][0] = Addr(p_4(c.g[A0][0], 0x48)).lw(c);
            loop {
                c.cycles += 7;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x58));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                call_ptr(c, 0x0039_8380, 0x0020_ebb0)?;
                Addr(p_4(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
                let more = (c.g[S2][0] as i64) >= 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0020_eba0);
                }
            }
            c.cycles += 10;
            c.g[A0][0] = 0x004a_21f0;
            c.g[S0][0] = Addr(p_4(c.g[S1][0], 0x8)).lw(c);
            call_ptr(c, 0x0031_7670, 0x0020_ebcc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_83f0, 0x0020_ebd8)?;
            c.g[V1][0] = 1;
            Addr(gp_3(c, -0x98c)).sw(c, c.g[V0][0]);
            Addr(gp_3(c, -0x988)).sw(c, c.g[V1][0]);
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S5][0];
        call_ptr(c, 0x0020_ec18, 0x0020_ebf4)?;
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

// ---------------------------------------------------------------------------
// cKeyboardPopup_SetText (0x00001cb418).

/// `cKeyboardPopup_SetText`: put the edited text into the keyboard popup and rebuild its entry list.
pub fn c_keyboard_popup_set_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_1cb488: {
            c.cycles += 20;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S2][0] = 0x1;
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x418), c.g[S2][0]);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x004166f4, 0x001cb444)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x00416810, 0x001cb44c)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x138), 0u64);
            c.g[A0][0] = c.g[S1][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x134), c.g[V0][0]);
            call_ptr(c, 0x00416810, 0x001cb45c)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
            let t = c.g[V1][0] == 0u64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x13c), c.g[V0][0]);
            if t {
                break 'b_1cb488;
            } else {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00416810, 0x001cb470)?;
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.g[S0][0];
                if c.g[V0][0] != 0 {
                    c.g[A2][0] = c.g[S2][0];
                }
                c.g[A1][0] = 0x4b;
                c.g[A3][0] = 0x2;
                call_ptr(c, 0x001ce3c8, 0x001cb488)?;
                break 'b_1cb488;
            }
        }
        'b_1cb4c8: {
            'b_1cb4b8: {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00416810, 0x001cb490)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x37;
                if t {
                    break 'b_1cb4b8;
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0x2;
                    call_ptr(c, 0x001ce3c8, 0x001cb4b0)?;
                    let t = true;

                    if t {
                        break 'b_1cb4c8;
                    } else {
                        break 'b_1cb4b8;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0x1;
            c.g[A3][0] = 0x2;
            call_ptr(c, 0x001ce3c8, 0x001cb4c8)?;
            break 'b_1cb4c8;
        }
        'b_1cb4e4: {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001cd708, 0x001cb4d0)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x41c));
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[S0][0];
            if t {
                break 'b_1cb4e4;
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x001cc848, 0x001cb4e4)?;
                break 'b_1cb4e4;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}

/// `void func_0020E9A0(OVScreen*, Widget* w, int action)` (0x20e9a0): the
/// trophy screen's widget-action handler.
pub fn ovscreen_trophy_on_widget_action(c: &mut Cpu) {
    guest(c, |c| ov_screen_widget_action(c, 0x0020_ea08))
}

/// `void cOVStateHUD1P::DrawProgressMeter(Rider* player)` (0x20eda0,
/// `src/fe/ovstatetrophy.cpp`, asm only in the reference): the race
/// progress meter of the 1P HUD.
///
/// Skipped when the HUD is hidden (`gp-0x990`), the meter is off
/// (`gp-0x994`), there is no player or no meter definition (`gp-0x9a0`).
/// Otherwise it asks the player for its racer slot (rider vtable +0x38),
/// slews each racer's displayed position (table 0x4c8bc8, six 0x18-byte
/// entries: +0x8 course distance, +0x10 shown position, +0x14 visible)
/// toward its distance (clamped to +-2000, stepped by the `.lit4` speeds
/// `gp-0x54ec`/`gp-0x54e8`), opens a sprite batch on the renderer
/// (`gp-0x854`, vtable at +0x10d8; GS packet pointer at +0xe84), draws the
/// percentage text (`sprintf("%d%%")` + `Font_DrawTextCentered` with the
/// HUD font at `gp-0x98c`), then the bar, every racer marker that falls in
/// the visible window (player distance -5000 .. +5000), the course markers
/// listed at `gp-0x998` (count at meter def +0x8), closes the batch and
/// remembers the player slot at `self+0x34`.
///
/// The body keeps the original's block structure (each block's cycle
/// charge, the scratch registers it leaves, the `.lit4` reads through
/// `gp`), with the calls through the call protocol and the loop back edges'
/// event checks through [`event_due`]/[`deliver_event_here`].
/// Slides the float register `r` (an x of the meter's layout, in the HUD's
/// virtual screen) with the race HUD's edge (`ui::hud_slide_x`): the meter
/// draws outside the HUD element system, so `HUD_GetElemPos` never sees it.
fn slide_meter_x(c: &mut Cpu, r: usize) {
    let x = f32::from_bits(c.f[r]);
    let y = crate::ui::hud_slide_x(c, x);
    if y != x {
        c.f[r] = y.to_bits();
    }
}

pub fn c_ov_state_hud1p_draw_progress_meter(c: &mut Cpu) {
    guest(c, |c| {
        'b_20faf8: {
            'b_20faf4: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1d0));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x990));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[RA][0]);
                c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x1b8));
                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x198));
                let t = c.g[V0][0] != 0u64;
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x190));
                if t {
                    break 'b_20faf4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x994));
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                    if t {
                        break 'b_20faf8;
                    } else {
                        c.cycles += 2;
                        let t = c.g[A1][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x998));
                        if t {
                            break 'b_20faf8;
                        } else {
                            c.cycles += 3;
                            c.g[S7][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9a0));
                            let t = c.g[S7][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[V0][0]);
                            if t {
                                break 'b_20faf8;
                            } else {
                                c.cycles += 33;
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x6c0));
                                c.g[S0][0] = 0x4c0000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
                                c.g[S5][0] = 0x4c0000;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0020ee34)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[S6][0] = c.g[V0][0];
                                c.g[V1][0] = 0x18;
                                c.g[V0][0] = 0x4d0000;
                                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.mult(0, 3, 22, 3);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7438));
                                c.g[AT][0] = 0xffffffffc4fa0000;
                                c.mtc1(1, 7);
                                c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                                c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                                c.g[AT][0] = 0x44fa0000;
                                c.mtc1(1, 6);
                                c.g[A3][0] = 0x1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0xe4), c.g[A1][0]);
                                c.g[A0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[V1][0] = c.g[A0][0];
                                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x54ec));
                                c.lwc1(20, (c.g[V1][0] as u32).wrapping_add(0x8));
                                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                                c.g[A1][0] = 0u64;
                                c.lwc1(4, (c.g[A0][0] as u32).wrapping_add(0xc));
                                c.g[S3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                                'l_20ee98: loop {
                                    'b_20eea4: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                                        if c.g[V0][0] != c.g[S6][0] {
                                            c.sw(c.g[V1][0] as u32, 0u64);
                                            break 'b_20eea4;
                                        } else {
                                            break 'b_20eea4;
                                        }
                                    }
                                    'b_20ef30: {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[A1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            break 'b_20ef30;
                                        } else {
                                            'b_20eeec: {
                                                'b_20eed4: {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[V1][0] as u32).wrapping_sub(0x8),
                                                    );
                                                    c.sub_s(0, 0, 4);
                                                    c.c_lt_s(0, 7);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_20eed4;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[AT][0] = 0xffffffffc4fa0000;
                                                        c.mtc1(1, 0);
                                                        let t = true;
                                                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                        if t {
                                                            break 'b_20eeec;
                                                        } else {
                                                            break 'b_20eed4;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.c_lt_s(6, 0);
                                                let t = !c.fcc();
                                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                if t {
                                                    break 'b_20eeec;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[AT][0] = 0x44fa0000;
                                                    c.mtc1(1, 0);
                                                    break 'b_20eeec;
                                                }
                                            }
                                            'b_20ef28: {
                                                c.cycles += 2;
                                                if c.g[V0][0] == 0u64 {
                                                    c.swc1(0, c.g[A2][0] as u32);
                                                    break 'b_20ef28;
                                                } else {
                                                    'b_20ef24: {
                                                        'b_20ef14: {
                                                            c.cycles += 6;
                                                            c.lwc1(2, c.g[A2][0] as u32);
                                                            c.lwc1(
                                                                3,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54e8),
                                                            );
                                                            c.sub_s(1, 0, 2);
                                                            c.c_lt_s(3, 1);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_20ef14;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.add_s(0, 2, 3);
                                                                if t {
                                                                    break 'b_20ef24;
                                                                } else {
                                                                    break 'b_20ef14;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.c_lt_s(1, 5);
                                                        if c.fcc() {
                                                            c.sub_s(0, 2, 3);
                                                            break 'b_20ef24;
                                                        } else {
                                                            break 'b_20ef24;
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.swc1(0, c.g[A2][0] as u32);
                                                    break 'b_20ef28;
                                                }
                                            }
                                            c.cycles += 2;
                                            c.sw(c.g[V1][0] as u32, c.g[A3][0]);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            break 'b_20ef30;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = ((c.g[A1][0] as i64) < 6i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x18));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0020ee98);
                                        }
                                        continue 'l_20ee98;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V0][0] = 0x3;
                                        c.g[V1][0] = 0xffffffffffffffff;
                                        'l_20ef48: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                            let t = c.g[V0][0] != c.g[V1][0];
                                            c.g[A0][0] = 0x80;
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0020ef48);
                                                }
                                                continue 'l_20ef48;
                                            } else {
                                                c.cycles += 5;
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 0);
                                                c.g[V1][0] = c.g[SP][0];
                                                c.g[A1][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    c.g[A0][0],
                                                );
                                                'l_20ef78: loop {
                                                    c.cycles += 8;
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x14),
                                                        c.g[A0][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                        c.g[A0][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x1c),
                                                        c.g[A0][0],
                                                    );
                                                    c.swc1(
                                                        0,
                                                        (c.g[V1][0] as u32).wrapping_add(0x8),
                                                    );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x30));
                                                    c.g[V0][0] = ((c.g[V1][0] as i64)
                                                        < (c.g[A1][0] as i64))
                                                        as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.sw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x10),
                                                            c.g[A0][0],
                                                        );
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0020ef78);
                                                        }
                                                        continue 'l_20ef78;
                                                    } else {
                                                        'b_20f114: {
                                                            c.cycles += 86;
                                                            c.g[A1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32)
                                                                .wrapping_add(0x10d8));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0xf8));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xfc));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x0020efb0)?;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[T0][0] = 0xfffffffffe7f0000;
                                                            c.g[T0][0] = c.g[T0][0] | 0xffff;
                                                            c.g[T4][0] = 0x1000000;
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[T1][0] = 0xffffffffffcf0000;
                                                            c.g[T1][0] = c.g[T1][0] | 0xffff;
                                                            c.g[T3][0] = 0xfffffffffff00000;
                                                            c.ldl(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(5, c.g[V0][0] as u32);
                                                            c.ldl(
                                                                6,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                6,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A3][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.sdl(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x1b),
                                                            );
                                                            c.sdr(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x14),
                                                            );
                                                            c.sdl(
                                                                6,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x23),
                                                            );
                                                            c.sdr(
                                                                6,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x1c),
                                                            );
                                                            c.sw(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x24),
                                                                c.g[A3][0],
                                                            );
                                                            c.g[T3][0] = c.g[T3][0] | 0xfff;
                                                            c.g[T2][0] = 0x10000;
                                                            c.g[T5][0] = 0xffffffffffffff83;
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[T2][0] = c.g[T2][0] | 0x4000;
                                                            c.g[A2][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[A1][0] = 0x500000;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0xe60));
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xe84),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V0][0] = 0x500000;
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[T6][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1420));
                                                            c.ldl(
                                                                4,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(4, c.g[T6][0] as u32);
                                                            c.ldl(
                                                                6,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                6,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A3][0] = c.lw((c.g[T6][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.sdl(
                                                                4,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(4, c.g[V1][0] as u32);
                                                            c.sdl(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x10),
                                                                c.g[A3][0],
                                                            );
                                                            c.g[A3][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[A0][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                                                            c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A0][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                                                            c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A2][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
                                                            c.g[V0][0] = c.g[V0][0] | 0x14;
                                                            c.sw(
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x10d8));
                                                            c.g[A0][0] = c.lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x110));
                                                            c.g[A2][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x114));
                                                            let t = c.g[A2][0] as u32;
                                                            c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x0020f0b8)?;
                                                            c.g[V0][0] = 0x18;
                                                            c.lwc1(
                                                                0,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54e4),
                                                            );
                                                            c.mult(0, 2, 22, 2);
                                                            c.g[A0][0] = 0x4d0000;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x7438));
                                                            c.sub_s(25, 20, 0);
                                                            c.lwc1(
                                                                1,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54e0),
                                                            );
                                                            c.lwc1(
                                                                2,
                                                                (c.g[S7][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x1dc4));
                                                            c.lwc1(
                                                                0,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.div_s(26, 0, 2);
                                                            let t = c.g[V1][0] != 0u64;
                                                            c.add_s(24, 25, 1);
                                                            if t {
                                                                break 'b_20f114;
                                                            } else {
                                                                c.cycles += 9;
                                                                c.g[AT][0] = 0x3f800000;
                                                                c.mtc1(1, 0);
                                                                c.g[V0][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x6868));
                                                                c.g[V1][0] = 0x1;
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0xc),
                                                                );
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dc4),
                                                                    c.g[V1][0],
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x6868),
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                );
                                                                break 'b_20f114;
                                                            }
                                                        }
                                                        'b_20f148: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x1dc8));
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x1dd8));
                                                            if t {
                                                                break 'b_20f148;
                                                            } else {
                                                                c.cycles += 10;
                                                                c.g[V0][0] = sx((c.g[S5][0]
                                                                    as u32)
                                                                    .wrapping_add(0x6878));
                                                                c.g[AT][0] = 0x3f800000;
                                                                c.mtc1(1, 0);
                                                                c.sw(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0xc),
                                                                    0u64,
                                                                );
                                                                c.g[V1][0] = 0x1;
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[S5][0] as u32)
                                                                        .wrapping_add(0x6878),
                                                                );
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dc8),
                                                                    c.g[V1][0],
                                                                );
                                                                c.sw(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                    0u64,
                                                                );
                                                                c.sw(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                    0u64,
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dd8));
                                                                break 'b_20f148;
                                                            }
                                                        }
                                                        'b_20f164: {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = 0x1;
                                                            if t {
                                                                break 'b_20f164;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[AT][0] = 0x40000000;
                                                                c.mtc1(1, 0);
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dd8),
                                                                    c.g[V0][0],
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dd4),
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dd0),
                                                                );
                                                                break 'b_20f164;
                                                            }
                                                        }
                                                        'b_20f3f0: {
                                                            c.cycles += 67;
                                                            c.g[AT][0] = 0x42c80000;
                                                            c.mtc1(1, 0);
                                                            c.g[V0][0] = 0x4d0000;
                                                            c.g[AT][0] = 0x3f000000;
                                                            c.mtc1(1, 1);
                                                            c.g[A1][0] = 0x4a0000;
                                                            c.mul_s(0, 26, 0);
                                                            c.g[A0][0] = c.g[FP][0];
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x2690));
                                                            c.g[S4][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x7368));
                                                            c.add_s(0, 0, 1);
                                                            c.cvt_w_s(1, 0);
                                                            c.mfc1(6, 1);
                                                            c.g[S2][0] = 0u64;
                                                            call_ptr(c, 0x00417828, 0x0020f1a0)?;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.g[A3][0] = 0xfffffffffffffc1f;
                                                            c.lwc1(
                                                                2,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54dc),
                                                            );
                                                            c.g[A1][0] = c.g[FP][0];
                                                            c.g[T0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x6868));
                                                            c.ldl(
                                                                2,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(2, c.g[T0][0] as u32);
                                                            c.ldl(
                                                                3,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                3,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                2,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x47),
                                                            );
                                                            c.sdr(
                                                                2,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x40),
                                                            );
                                                            c.sdl(
                                                                3,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4f),
                                                            );
                                                            c.sdr(
                                                                3,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x48),
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.lwc1(
                                                                0,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x34),
                                                            );
                                                            c.mul_s(0, 0, 2);
                                                            c.mul_s(1, 1, 2);
                                                            c.swc1(
                                                                0,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x38),
                                                            );
                                                            c.swc1(
                                                                1,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x3c),
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.g[A2][0] = sx((c.g[S5][0] as u32)
                                                                .wrapping_add(0x6878));
                                                            c.ldl(
                                                                14,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(14, c.g[A2][0] as u32);
                                                            c.ldl(
                                                                3,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                3,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                14,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x57),
                                                            );
                                                            c.sdr(
                                                                14,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x50),
                                                            );
                                                            c.sdl(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x5f),
                                                            );
                                                            c.sdr(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x58),
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.ldl(
                                                                8,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x1dd7),
                                                            );
                                                            c.ldr(
                                                                8,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x1dd0),
                                                            );
                                                            c.sdl(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x2f),
                                                            );
                                                            c.sdr(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x28),
                                                            );
                                                            c.g[A2][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                                                            c.g[V0][0] = c.g[V0][0] | 0x140;
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                                c.g[V0][0],
                                                            );
                                                            c.lwc1(12, c.g[S1][0] as u32);
                                                            slide_meter_x(c, 12);
                                                            c.lwc1(
                                                                13,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            call_ptr(c, 0x00392908, 0x0020f254)?;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.lwc1(
                                                                0,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x34),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x38),
                                                            );
                                                            c.swc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x3c),
                                                            );
                                                            let t = (c.g[S3][0] as i64) <= 0;
                                                            c.g[S0][0] = c.lw((c.g[S4][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_20f3f0;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.g[AT][0] = 0x3f800000;
                                                                c.mtc1(1, 20);
                                                                c.g[S5][0] = 0x4d0000;
                                                                c.g[S5][0] = sx((c.g[S5][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x7438));
                                                                c.g[FP][0] = 0xfffffffffffffc1f;
                                                                c.g[V0][0] = 0x18;
                                                                'l_20f288: loop {
                                                                    'b_20f3e4: {
                                                                        'b_20f2b8: {
                                                                            c.cycles += 7;
                                                                            c.mult(0, 2, 18, 2);
                                                                            c.g[V1][0] =
                                                                                sx((c.g[S5][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        c.g[V0][0]
                                                                                            as u32,
                                                                                    ));
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[V1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                            );
                                                                            c.c_le_s(25, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[A1][0] = 0x4d0000;
                                                                            if t {
                                                                                break 'b_20f2b8;
                                                                            } else {
                                                                                c.cycles += 4;
                                                                                c.c_le_s(1, 24);
                                                                                if c.fcc() {
                                                                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                                                    break 'b_20f3e4;
                                                                                } else {
                                                                                    c.cycles += 1;
                                                                                    c.g[A1][0] =
                                                                                        0x4d0000;
                                                                                    break 'b_20f2b8;
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_20f30c: {
                                                                            c.cycles += 18;
                                                                            c.lwc1(
                                                                                4,
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0x54d8,
                                                                                    ),
                                                                            );
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x7428,
                                                                                    ));
                                                                            c.c_lt_s(24, 1);
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        c.g[A1][0]
                                                                                            as u32,
                                                                                    ));
                                                                            c.lwc1(
                                                                                5,
                                                                                (c.g[S0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x18,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                c.g[V0][0] as u32,
                                                                            );
                                                                            c.lwc1(
                                                                                3,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x10,
                                                                                    ),
                                                                            );
                                                                            c.mov_s(8, 5);
                                                                            c.neg_s(0, 0);
                                                                            c.lwc1(
                                                                                2,
                                                                                (c.g[S4][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x4,
                                                                                    ),
                                                                            );
                                                                            c.g[AT][0] = 0x3f000000;
                                                                            c.mtc1(1, 1);
                                                                            c.lwc1(
                                                                                7,
                                                                                (c.g[S0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xc,
                                                                                    ),
                                                                            );
                                                                            c.mul_s(0, 0, 4);
                                                                            c.lwc1(
                                                                                6,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xc,
                                                                                    ),
                                                                            );
                                                                            c.mul_s(2, 2, 1);
                                                                            let t = !c.fcc();
                                                                            c.add_s(4, 3, 0);
                                                                            if t {
                                                                                break 'b_20f30c;
                                                                            } else {
                                                                                c.cycles += 3;
                                                                                c.mov_s(5, 7);
                                                                                c.lwc1(6, (c.g[S1][0] as u32).wrapping_add(0x8));
                                                                                c.mov_s(7, 8);
                                                                                break 'b_20f30c;
                                                                            }
                                                                        }
                                                                        c.cycles += 54;
                                                                        c.lwc1(
                                                                            0,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                        );
                                                                        c.sub_s(3, 4, 2);
                                                                        c.swc1(
                                                                            5,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x4),
                                                                        );
                                                                        c.add_s(4, 4, 2);
                                                                        c.swc1(
                                                                            0,
                                                                            c.g[SP][0] as u32,
                                                                        );
                                                                        c.g[A1][0] = 0x4;
                                                                        c.g[A2][0] = c.g[SP][0];
                                                                        c.g[A3][0] = 0u64;
                                                                        c.lwc1(
                                                                            1,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x14),
                                                                        );
                                                                        c.lwc1(
                                                                            2,
                                                                            (c.g[S4][0] as u32)
                                                                                .wrapping_add(0x8),
                                                                        );
                                                                        c.swc1(
                                                                            5,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x34),
                                                                        );
                                                                        c.swc1(
                                                                            1,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x30),
                                                                        );
                                                                        c.add_s(2, 6, 2);
                                                                        c.lwc1(
                                                                            0,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                        );
                                                                        c.swc1(
                                                                            7,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x64),
                                                                        );
                                                                        c.swc1(
                                                                            0,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x60),
                                                                        );
                                                                        c.lwc1(
                                                                            1,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x14),
                                                                        );
                                                                        c.swc1(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd8),
                                                                            0u64,
                                                                        );
                                                                        c.swc1(
                                                                            6,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd4),
                                                                        );
                                                                        c.swc1(
                                                                            20,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xdc),
                                                                        );
                                                                        c.swc1(
                                                                            1,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x90),
                                                                        );
                                                                        c.lq(
                                                                            4,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.swc1(
                                                                            4,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd8),
                                                                            0u64,
                                                                        );
                                                                        c.swc1(
                                                                            7,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x94),
                                                                        );
                                                                        c.lq(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.swc1(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.swc1(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd4),
                                                                        );
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd8),
                                                                            0u64,
                                                                        );
                                                                        c.sq(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x50),
                                                                        );
                                                                        c.lq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.swc1(
                                                                            4,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.sq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x80),
                                                                        );
                                                                        c.sq(
                                                                            4,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x20),
                                                                        );
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd8),
                                                                            0u64,
                                                                        );
                                                                        c.lq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.g[T0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x854));
                                                                        c.sq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xb0),
                                                                        );
                                                                        c.g[A0][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xe84));
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[A0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        c.g[V0][0] =
                                                                            c.g[V0][0] & c.g[FP][0];
                                                                        c.g[V0][0] =
                                                                            c.g[V0][0] | 0x140;
                                                                        c.sw(
                                                                            (c.g[A0][0] as u32)
                                                                                .wrapping_add(0x8),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xe84));
                                                                        c.g[V0][0] =
                                                                            c.lw(c.g[S0][0] as u32);
                                                                        c.sh(
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[T0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x854));
                                                                        c.g[V0][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10d8));
                                                                        c.g[A0][0] = c.lh((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x238));
                                                                        c.g[V1][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x23c));
                                                                        let t = c.g[V1][0] as u32;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[T0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[A0][0]
                                                                                        as u32,
                                                                                ));
                                                                        call_ptr(c, t, 0x0020f3e0)?;
                                                                        c.g[S2][0] =
                                                                            sx((c.g[S2][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        break 'b_20f3e4;
                                                                    }
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = ((c.g[S2][0]
                                                                        as i64)
                                                                        < (c.g[S3][0] as i64))
                                                                        as u64;
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.g[V0][0] = 0x18;
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x0020f288,
                                                                            );
                                                                        }
                                                                        continue 'l_20f288;
                                                                    } else {
                                                                        break 'b_20f3f0;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        'b_20f420: {
                                                            c.cycles += 8;
                                                            c.lwc1(
                                                                0,
                                                                (c.g[S7][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.g[A2][0] = 0x4d0000;
                                                            c.g[V0][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_sub(0x73a8));
                                                            c.lwc1(
                                                                21,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.c_lt_s(0, 24);
                                                            c.g[T2][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc));
                                                            let t = !c.fcc();
                                                            c.lwc1(
                                                                22,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x14),
                                                            );
                                                            if t {
                                                                break 'b_20f420;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.sub_s(0, 24, 0);
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x1c),
                                                                );
                                                                c.mul_s(0, 0, 1);
                                                                c.add_s(22, 22, 0);
                                                                break 'b_20f420;
                                                            }
                                                        }
                                                        'b_20f444: {
                                                            c.cycles += 5;
                                                            c.mtc1(0, 23);
                                                            c.c_lt_s(25, 23);
                                                            if !c.fcc() {
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[T2][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                break 'b_20f444;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x1c),
                                                                );
                                                                c.mul_s(0, 25, 0);
                                                                c.add_s(21, 21, 0);
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[T2][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                break 'b_20f444;
                                                            }
                                                        }
                                                        c.cycles += 83;
                                                        c.g[T1][0] = 0xfffffffffffffc1f;
                                                        c.lwc1(
                                                            2,
                                                            (c.g[S1][0] as u32).wrapping_add(0x10),
                                                        );
                                                        slide_meter_x(c, 2);
                                                        c.g[A1][0] = 0x4;
                                                        c.swc1(0, c.g[SP][0] as u32);
                                                        c.g[A2][0] = c.g[SP][0];
                                                        c.lwc1(
                                                            1,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x54d4),
                                                        );
                                                        c.g[A3][0] = 0u64;
                                                        c.lwc1(
                                                            0,
                                                            (c.g[T2][0] as u32).wrapping_add(0xc),
                                                        );
                                                        c.sub_s(3, 2, 1);
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 20);
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x4),
                                                        );
                                                        c.add_s(2, 2, 1);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[T2][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x30),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[T2][0] as u32).wrapping_add(0xc),
                                                        );
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x34),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[T2][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x60),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[T2][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x64),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[T2][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x90),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[T2][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd4),
                                                        );
                                                        c.swc1(
                                                            23,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd8),
                                                        );
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0xdc),
                                                        );
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x94),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            22,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd4),
                                                        );
                                                        c.sq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x50),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x80),
                                                        );
                                                        c.swc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.g[T0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.g[A0][0] = c
                                                            .lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                                                        c.g[V0][0] = c.g[V0][0] | 0x120;
                                                        c.sw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x8),
                                                            c.g[V0][0],
                                                        );
                                                        c.g[V0][0] = c
                                                            .lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                        c.g[V1][0] = c.lw(c.g[T2][0] as u32);
                                                        c.sh(
                                                            (c.g[V0][0] as u32).wrapping_add(0x10),
                                                            c.g[V1][0],
                                                        );
                                                        c.g[T0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                        c.g[V0][0] = c.lw((c.g[T0][0] as u32)
                                                            .wrapping_add(0x10d8));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x238));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x23c));
                                                        let t = c.g[V1][0] as u32;
                                                        c.g[A0][0] = sx((c.g[T0][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        call_ptr(c, t, 0x0020f524)?;
                                                        c.lwc1(
                                                            2,
                                                            (c.g[S1][0] as u32).wrapping_add(0x10),
                                                        );
                                                        slide_meter_x(c, 2);
                                                        c.sub_s(1, 22, 21);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x54d0),
                                                        );
                                                        c.g[A2][0] = 0x80;
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd4),
                                                        );
                                                        c.g[A1][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc0));
                                                        c.sub_s(3, 2, 0);
                                                        c.swc1(
                                                            23,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd8),
                                                        );
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0xdc),
                                                        );
                                                        c.add_s(2, 2, 0);
                                                        c.mul_s(1, 1, 26);
                                                        c.g[A0][0] = c.g[SP][0];
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.lq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.add_s(1, 21, 1);
                                                        c.swc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd4),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x50),
                                                        );
                                                        c.lq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x80),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.sw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x10),
                                                            c.g[A2][0],
                                                        );
                                                        'l_20f590: loop {
                                                            c.cycles += 7;
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x14),
                                                                0u64,
                                                            );
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                                0u64,
                                                            );
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x30));
                                                            c.g[V0][0] = ((c.g[A0][0] as i64)
                                                                < (c.g[A1][0] as i64))
                                                                as u64;
                                                            if c.g[V0][0] != 0u64 {
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    c.g[A2][0],
                                                                );
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x0020f590,
                                                                    );
                                                                }
                                                                continue 'l_20f590;
                                                            } else {
                                                                c.cycles += 23;
                                                                c.g[T0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x854));
                                                                c.g[V1][0] = 0xfffffffffffffc1f;
                                                                c.g[T1][0] = 0xffffffffffffffff;
                                                                c.g[A1][0] = 0x4;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[T0][0] as u32)
                                                                        .wrapping_add(0xe84));
                                                                c.g[A2][0] = c.g[SP][0];
                                                                c.g[A3][0] = 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] & c.g[V1][0];
                                                                c.g[V0][0] = c.g[V0][0] | 0x140;
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                    c.g[V0][0],
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[T0][0] as u32)
                                                                        .wrapping_add(0xe84));
                                                                c.sh(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    c.g[T1][0],
                                                                );
                                                                c.g[T0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x854));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[T0][0] as u32)
                                                                        .wrapping_add(0x10d8));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x238));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x23c));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[A0][0] = sx((c.g[T0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                call_ptr(c, t, 0x0020f5f8)?;
                                                                c.g[A0][0] = 0x80;
                                                                c.g[A1][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0xc0));
                                                                c.g[V1][0] = c.g[SP][0];
                                                                c.sw(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    c.g[A0][0],
                                                                );
                                                                'l_20f608: loop {
                                                                    c.cycles += 7;
                                                                    c.sw(
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x14),
                                                                        c.g[A0][0],
                                                                    );
                                                                    c.sw(
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x18),
                                                                        c.g[A0][0],
                                                                    );
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x30));
                                                                    c.g[V0][0] = ((c.g[V1][0]
                                                                        as i64)
                                                                        < (c.g[A1][0] as i64))
                                                                        as u64;
                                                                    if c.g[V0][0] != 0u64 {
                                                                        c.sw(
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                            c.g[A0][0],
                                                                        );
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x0020f608,
                                                                            );
                                                                        }
                                                                        continue 'l_20f608;
                                                                    } else {
                                                                        'b_20f8a4: {
                                                                            c.cycles += 8;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x1c,
                                                                                    ),
                                                                            );
                                                                            c.g[S0][0] = 0u64;
                                                                            c.g[AT][0] = 0x41a00000;
                                                                            c.mtc1(1, 0);
                                                                            c.div_s(0, 0, 1);
                                                                            c.add_s(24, 24, 0);
                                                                            let t = (c.g[S3][0]
                                                                                as i64)
                                                                                <= 0;
                                                                            c.sub_s(23, 25, 0);
                                                                            if t {
                                                                                break 'b_20f8a4;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[V0][0] =
                                                                                    0x4d0000;
                                                                                c.g[AT][0] =
                                                                                    0x3f000000;
                                                                                c.mtc1(1, 26);
                                                                                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7438));
                                                                                c.g[AT][0] =
                                                                                    0x3f800000;
                                                                                c.mtc1(1, 22);
                                                                                c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                                                                                c.g[V0][0] = 0x18;
                                                                                'l_20f668: loop {
                                                                                    'b_20f898: {
                                                                                        c.cycles +=
                                                                                            7;
                                                                                        c.mult(
                                                                                            0, 4,
                                                                                            16, 2,
                                                                                        );
                                                                                        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                                                                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                                                        c.c_lt_s(
                                                                                            0, 23,
                                                                                        );
                                                                                        if c.fcc() {
                                                                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                                                            break 'b_20f898;
                                                                                        } else {
                                                                                            c.cycles += 4;
                                                                                            c.c_lt_s(24, 0);
                                                                                            if c.fcc() {
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
break 'b_20f898;
} else {
'b_20f718: {
c.cycles += 10;
c.sub_s(0, 0, 25);
c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x1c));
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
c.g[T1][0] = 0xc;
c.lwc1(20, (c.g[S1][0] as u32).wrapping_add(0x10));
slide_meter_x(c, 20);
c.g[V0][0] = 0x1;
c.mul_s(0, 0, 2);
c.mov_s(4, 1);
let t = c.g[S6][0] == c.g[S0][0];
c.sub_s(21, 1, 0);
if t {
break 'b_20f718;
} else {
'b_20f70c: {
c.cycles += 17;
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x54cc));
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
c.lwc1(0, c.g[V1][0] as u32);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
c.mul_s(0, 0, 1);
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x40));
c.sub_s(20, 20, 0);
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
let t = c.g[V0][0] as u32;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
call_ptr(c, t, 0x0020f6f8)?;
let t = c.g[V0][0] == 0u64;
c.g[T1][0] = 0xd;
if t {
break 'b_20f70c;
} else {
c.cycles += 3;
c.g[V0][0] = 0x3;
let t = true;
c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x18));
if t {
break 'b_20f718;
} else {
break 'b_20f70c;
}
}
}
c.cycles += 3;
c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x18));
c.g[T1][0] = 0xe;
c.g[V0][0] = 0x2;
break 'b_20f718;
}
}
'b_20f894: {
c.cycles += 12;
c.g[T0][0] = 0x4d0000;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x73a8));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
c.g[T3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
c.mul_s(2, 0, 26);
c.sub_s(6, 21, 2);
c.c_le_s(4, 6);
let t = c.fcc();
c.add_s(7, 21, 2);
if t {
break 'b_20f894;
} else {
c.cycles += 5;
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
c.c_le_s(7, 1);
if c.fcc() {
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
break 'b_20f898;
} else {
'b_20f7bc: {
'b_20f7b8: {
'b_20f78c: {
c.cycles += 4;
c.c_lt_s(6, 1);
c.lwc1(3, (c.g[T3][0] as u32).wrapping_add(0xc));
let t = !c.fcc();
c.lwc1(5, (c.g[T3][0] as u32).wrapping_add(0x18));
if t {
break 'b_20f78c;
} else {
c.cycles += 8;
c.mov_s(6, 1);
c.sub_s(0, 3, 5);
c.sub_s(1, 7, 1);
c.add_s(2, 2, 2);
c.mul_s(0, 0, 1);
c.div_s(0, 0, 2);
let t = true;
c.add_s(3, 5, 0);
if t {
break 'b_20f7b8;
} else {
break 'b_20f78c;
}
}
}
c.cycles += 4;
c.c_lt_s(4, 7);
if !c.fcc() {
c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0x10));
break 'b_20f7bc;
} else {
c.cycles += 7;
c.sub_s(0, 5, 3);
c.mov_s(7, 4);
c.sub_s(1, 4, 6);
c.add_s(2, 2, 2);
c.mul_s(0, 0, 1);
c.div_s(0, 0, 2);
c.add_s(5, 3, 0);
break 'b_20f7b8;
}
}
c.cycles += 1;
c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0x10));
break 'b_20f7bc;
}
c.cycles += 54;
c.g[T2][0] = 0xfffffffffffffc1f;
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[T1][0] = sx((c.g[T1][0] as u32) << 5);
c.swc1(0, c.g[SP][0] as u32);
c.g[A1][0] = 0x4;
c.g[A2][0] = c.g[SP][0];
c.g[A3][0] = 0u64;
c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0x14));
c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mul_s(2, 2, 26);
c.lwc1(1, (c.g[T3][0] as u32).wrapping_add(0x10));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x64));
c.sub_s(3, 20, 2);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x60));
c.add_s(2, 20, 2);
c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0x14));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xdc));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x94));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = c.lw(c.g[T3][0] as u32);
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
let t = c.g[V1][0] as u32;
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
call_ptr(c, t, 0x0020f894)?;
break 'b_20f894;
}
}
}
c.cycles += 1;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
break 'b_20f898;
}
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 3;
                                                                                    c.g[V0][0] = ((c
                                                                                        .g[S0][0]
                                                                                        as i64)
                                                                                        < (c.g[S3]
                                                                                            [0]
                                                                                            as i64))
                                                                                        as u64;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != 0u64;
                                                                                    c.g[V0][0] =
                                                                                        0x18;
                                                                                    if t {
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x0020f668);
                                                                                        }
                                                                                        continue 'l_20f668;
                                                                                    } else {
                                                                                        break 'b_20f8a4;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_20facc: {
                                                                            c.cycles += 3;
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [S7][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                            let t =
                                                                                c.g[A1][0] == 0u64;
                                                                            c.g[V1][0] = 0x530000;
                                                                            if t {
                                                                                break 'b_20facc;
                                                                            } else {
                                                                                'b_20f8d4: {
                                                                                    c.cycles += 5;
                                                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                                                                                    c.g[V0][0] = sx(
                                                                                        ((c.g[V0][0]
                                                                                            as u32)
                                                                                            as i32
                                                                                            >> 9)
                                                                                            as u32,
                                                                                    );
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        & 0x1;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        == 0u64;
                                                                                    c.g[S3][0] =
                                                                                        0x1;
                                                                                    if t {
                                                                                        break 'b_20f8d4;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                                                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x88));
                                                                                        if c.g[A0]
                                                                                            [0]
                                                                                            != 0
                                                                                        {
                                                                                            c.g[S3][0] = 0u64;
                                                                                        }
                                                                                        break 'b_20f8d4;
                                                                                    }
                                                                                }
                                                                                c.cycles += 2;
                                                                                let t = (c.g[A1][0]
                                                                                    as i64)
                                                                                    <= 0;
                                                                                c.g[S2][0] = 0u64;
                                                                                if t {
                                                                                    break 'b_20facc;
                                                                                } else {
                                                                                    c.cycles += 9;
                                                                                    c.g[V0][0] =
                                                                                        0x4d0000;
                                                                                    c.g[AT][0] =
                                                                                        0x3f000000;
                                                                                    c.mtc1(1, 21);
                                                                                    c.g[AT][0] =
                                                                                        0x3f800000;
                                                                                    c.mtc1(1, 20);
                                                                                    c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7358));
                                                                                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                                                                                    c.g[S4][0] = 0xfffffffffffffc1f;
                                                                                    'l_20f900: loop {
                                                                                        'b_20fabc: {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                            c.c_lt_s(1, 23);
                                                                                            if c.fcc() {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
} else {
c.cycles += 4;
c.c_lt_s(24, 1);
if c.fcc() {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
} else {
'b_20f940: {
c.cycles += 4;
c.g[V1][0] = c.lw(c.g[S0][0] as u32);
c.g[V0][0] = 0x1;
if c.g[V1][0] != c.g[V0][0] {
c.sub_s(1, 1, 25);
break 'b_20f940;
} else {
c.cycles += 2;
if c.g[S3][0] == 0u64 {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
} else {
c.cycles += 1;
c.sub_s(1, 1, 25);
break 'b_20f940;
}
}
}
'b_20fab8: {
c.cycles += 16;
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x18));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
c.mul_s(1, 1, 0);
c.lwc1(7, (c.g[V0][0] as u32).wrapping_add(0x8));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
c.mul_s(2, 7, 21);
c.g[T1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
c.sub_s(1, 4, 1);
c.mul_s(9, 0, 21);
c.sub_s(6, 1, 2);
c.c_le_s(4, 6);
let t = c.fcc();
c.add_s(8, 1, 2);
if t {
break 'b_20fab8;
} else {
c.cycles += 5;
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
c.c_le_s(8, 0);
if c.fcc() {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
} else {
'b_20f9ec: {
'b_20f9e8: {
'b_20f9c0: {
c.cycles += 4;
c.c_lt_s(6, 0);
c.lwc1(3, (c.g[T1][0] as u32).wrapping_add(0xc));
let t = !c.fcc();
c.lwc1(5, (c.g[T1][0] as u32).wrapping_add(0x18));
if t {
break 'b_20f9c0;
} else {
c.cycles += 7;
c.mov_s(6, 0);
c.sub_s(1, 3, 5);
c.sub_s(0, 8, 0);
c.mul_s(1, 1, 0);
c.div_s(1, 1, 7);
let t = true;
c.add_s(3, 5, 1);
if t {
break 'b_20f9e8;
} else {
break 'b_20f9c0;
}
}
}
c.cycles += 4;
c.c_lt_s(4, 8);
if !c.fcc() {
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x10));
break 'b_20f9ec;
} else {
c.cycles += 6;
c.sub_s(1, 5, 3);
c.mov_s(8, 4);
c.sub_s(0, 4, 6);
c.mul_s(1, 1, 0);
c.div_s(1, 1, 7);
c.add_s(5, 3, 1);
break 'b_20f9e8;
}
}
c.cycles += 1;
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x10));
break 'b_20f9ec;
}
c.cycles += 51;
c.g[A1][0] = 0x4;
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A2][0] = c.g[SP][0];
c.swc1(0, c.g[SP][0] as u32);
c.g[A3][0] = 0u64;
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x14));
c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x10));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sub_s(3, 2, 9);
c.add_s(2, 2, 9);
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x10));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x64));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lwc1(1, (c.g[T1][0] as u32).wrapping_add(0x14));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xdc));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x94));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[S4][0];
c.g[V0][0] = c.g[V0][0] | 0x160;
c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = c.lw(c.g[T1][0] as u32);
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
let t = c.g[V1][0] as u32;
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
call_ptr(c, t, 0x0020fab8)?;
break 'b_20fab8;
}
}
}
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
}
}
                                                                                        }
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                                                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                                                                        let t = c.g
                                                                                            [V0][0]
                                                                                            != 0u64;
                                                                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        if t {
                                                                                            if event_due(c) { deliver_event_here(c, 0x0020f900); }
                                                                                            continue 'l_20f900;
                                                                                        } else {
                                                                                            break 'b_20facc;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x854));
                                                                        c.g[V0][0] = c.lw((c.g[A0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xe84));
                                                                        c.g[A1][0] = c.lw((c.g[A0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10d8));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x14,
                                                                                ));
                                                                        c.sw(
                                                                            (c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0xe84,
                                                                                ),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[V1][0] = c.lh((c.g[A1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x100));
                                                                        c.g[V0][0] = c.lw((c.g[A1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x104));
                                                                        let t = c.g[V0][0] as u32;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                ));
                                                                        call_ptr(c, t, 0x0020faf0)?;
                                                                        c.sw(
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x34),
                                                                            c.g[S6][0],
                                                                        );
                                                                        break 'b_20faf4;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
            break 'b_20faf8;
        }
        c.cycles += 18;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x1b8));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1a8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x198));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x190));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.pc = t;
        return Ok(());
    })
}

// ---------------------------------------------------------------------------
// cOVStateHUD1P_UpdateRaceProgress (0x20ed20)

pub(crate) const RACE_PROGRESS_UPDATE_RIDER: Callee = Callee {
    addr: 0x0021_0618,
    f: ee::s05::RaceProgress_UpdateRider,
};

/// The course (`gp-0x9a0`, null outside a race), the world (`gp-0x848`)
/// and the "progress changed" word (`gp-0x990`).
pub(crate) const GP_RACE_COURSE: u32 = 0x9a0;

pub(crate) const GP_WORLD: u32 = 0x848;

pub(crate) const GP_PROGRESS_DIRTY: u32 = 0x990;

/// world+0x84 -> racers, racers+0xc -> race, race+0x78: the rider count.
pub(crate) const WORLD_RACERS: u32 = 0x84;

pub(crate) const RACERS_RACE: u32 = 0xc;

pub(crate) const RACE_RIDER_COUNT: u32 = 0x78;

/// The race-progress slots (0x18 bytes each; the active word at +0).
pub(crate) const PROGRESS_SLOTS: u32 = 0x004c_8bc8;

pub(crate) const PROGRESS_SLOT_SIZE: u32 = 0x18;

/// `void cOVStateHUD1P_UpdateRaceProgress()` (0x20ed20): in a race, update
/// every rider's progress (`RaceProgress_UpdateRider(i)`) and clear its
/// slot's active word, then clear the "progress changed" word. Frame 0x40
/// (ra at 0, s2 / s1 / s0 at 0x10 / 0x20 / 0x30).
pub fn hud1p_update_race_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        let gp = r32(c, GP);
        c.g[V0][0] = c.lw(gp.wrapping_sub(GP_RACE_COURSE));
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[V0][0] = c.lw(gp.wrapping_sub(GP_WORLD));
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(WORLD_RACERS));
            c.g[S2][0] = c.lw(r32(c, V1).wrapping_add(RACERS_RACE));
            c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(RACE_RIDER_COUNT));
            c.g[S0][0] = 0;
            if (c.g[V0][0] as i64) > 0 {
                // s0: the rider, s1: its slot.
                c.cycles += 3;
                c.g[V0][0] = 0x004d_0000;
                c.g[S1][0] = sx(PROGRESS_SLOTS);
                loop {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    RACE_PROGRESS_UPDATE_RIDER.call(c, 0x0020_ed68)?;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    c.sw(r32(c, S1), 0);
                    c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(RACE_RIDER_COUNT));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(PROGRESS_SLOT_SIZE));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0020_ed60);
                    }
                }
            }
            c.cycles += 1;
            let gp = r32(c, GP);
            c.sw(gp.wrapping_sub(GP_PROGRESS_DIRTY), 0);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

pub(crate) mod progress {

    pub const ZONES: i32 = -0x99c;

    pub const COURSE: i32 = -0x9a0;

    pub const WORLD: i32 = -0x848;

    /// The step limit (138.8889) at `gp-0x54ac`.
    pub const STEP_LIMIT: i32 = -0x54ac;

    pub const SEG: u32 = 0x4;

    pub const DIST: u32 = 0x8;

    pub const POS: u32 = 0xc;

    pub const E: u32 = 0x14;

    pub const STRIDE: u32 = 0x18;

    /// Course: segment count at +0, distance at +0x10.
    pub const COURSE_COUNT: u32 = 0x0;

    pub const COURSE_DIST: u32 = 0x10;

    /// One segment (`sZone`): scale x/y at +0/+4, origin x/y at +8/+0xc,
    /// distance at +0x10 (stride 0x14).
    pub const SEG_DIST: u32 = 0x10;

    pub const SEG_STRIDE: u32 = 0x14;

    /// Race manager: riders at +0x28, rider count at +0x78.
    pub const RIDERS: u32 = 0x28;

    pub const RIDER_COUNT: u32 = 0x78;

    pub const RIDER_POS: u32 = 0x6c0;

    pub const ZONE_SY: u32 = 0x4;

    pub const ZONE_X: u32 = 0x8;

    pub const ZONE_Y: u32 = 0xc;
}

// ---------------------------------------------------------------------------
// RaceProgress_ProjectRider.

/// `float RaceProgress_ProjectRider(int i)` (0x210820): the rider's lateral
/// offset from its zone line. The zone index is the slot's `+4`, the zone
/// `{scale x, scale y, origin x, origin y, dist}` at `zones + zone * 0x14`;
/// the offset is `(pos.x - origin.x) * scale.x + (pos.y - origin.y) *
/// scale.y`, with the rider position read through `rider+0x6c0`'s virtual
/// slot at `+0x2c`. An active slot also has its `+0x14` cleared.
pub fn race_progress_project_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0x40);
        let world = c.lw(gp_m2(c, progress::WORLD));
        c.g[A1][0] = world;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        let idx = r32(c, A0);
        c.g[S2][0] = c.g[A0][0];
        c.g[A0][0] = sx((idx as u32) << 2);
        let zones = c.lw(gp_m2(c, progress::ZONES));
        c.g[S1][0] = zones;
        c.g[V1][0] = c.lw((world as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xc));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(progress::RIDERS));
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(progress::RIDER_POS));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(progress::RIDER_POS));
        c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x2c));
        let v05 = r32(c, V0);
        c.g[RA][0] = sx(0x0021_0868);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        call_ptr(c, v05, 0x0021_0868)?;
        // The slot (0x18 bytes) and the zone offset (0x14 bytes); the
        // original's `mult`s leave the two products in a3 and v1.
        c.g[V1][0] = 0x18;
        c.g[A0][0] = 0x4d_0000;
        c.mult(0, S2, S2, V1);
        c.g[T1][0] = sx(0x4d_0000u32.wrapping_sub(0x7438));
        c.lq(3, r32(c, V0)); // the position the virtual call returned
        c.g[A1][0] = progress::SEG_STRIDE as u64;
        let sp = sp(c);
        c.sq(3, sp);
        c.g[A0][0] = sx(r32(c, T1).wrapping_add(r32(c, S2)));
        c.g[A2][0] = sx(r32(c, S2).wrapping_add(r32(c, T1)));
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(progress::SEG));
        c.lwc1(2, sp.wrapping_add(4));
        c.mult(0, A3, V0, A1);
        c.mult(0, V1, V0, A1);
        c.lwc1(1, sp);
        c.g[T0][0] = c.lw(r32(c, A2));
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, S1)));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S1)));
        c.lwc1(0, r32(c, A0).wrapping_add(progress::ZONE_X));
        c.g[A3][0] = c.g[A0][0];
        c.lwc1(3, r32(c, V1).wrapping_add(progress::ZONE_Y));
        c.g[V0][0] = c.g[A0][0];
        c.sub_s(1, 1, 0);
        c.lwc1(4, r32(c, V0).wrapping_add(progress::ZONE_SY));
        c.sub_s(2, 2, 3);
        c.lwc1(0, r32(c, A3));
        c.mul_s(1, 1, 0);
        c.mul_s(2, 2, 4);
        let inactive = c.g[T0][0] == 0;
        c.add_s(0, 2, 1);
        if !inactive {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, S2)));
            c.sw(r32(c, V0).wrapping_add(progress::E), 0);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RaceProgress_UpdateRider.

/// `void RaceProgress_UpdateRider(int i)` (0x210618): the rider's distance
/// along the course. `d = course.dist - Rider_GetDistToFinish(rider)` is
/// clamped to the slot's previous distance ±138.8889 unless the slot is
/// active; the slot's segment is then found on the zone/segment table
/// (jumping to the last segment past the end, else walking to the interval),
/// and the slot's distance plus `RaceProgress_ProjectRider`'s offset are
/// stored.
pub fn race_progress_update_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        let course = c.lw(gp_m2(c, progress::COURSE));
        c.g[S3][0] = course;
        let idx = r32(c, A0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        let no_course = course == 0;
        let segs = c.lw(gp_m2(c, progress::ZONES));
        c.g[S0][0] = segs;
        if !no_course {
            c.cycles += 7;
            c.g[V0][0] = c.lw(gp_m2(c, progress::WORLD));
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
            c.g[V1][0] = c.lw(r32(c, V1).wrapping_add(0xc));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(progress::RIDER_COUNT));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let known = c.g[V0][0] != 0;
            c.g[V0][0] = sx(idx << 2);
            if known {
                // `d = course.dist - Rider_GetDistToFinish(rider)`.
                c.cycles += 13;
                c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[RA][0] = sx(0x0021_0668);
                c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(progress::RIDERS));
                call_ptr(c, callee::RIDER_GET_DIST_TO_FINISH, 0x0021_0668)?;
                c.g[V1][0] = 0x18;
                c.g[A0][0] = 0x4d_0000;
                c.mult(0, A0, S2, V1); // a0 = i * 0x18
                c.g[A1][0] = sx(0x4d_0000u32.wrapping_sub(0x7438));
                c.g[S1][0] = 0x4d_0000;
                let course_dist = r32(c, S3).wrapping_add(progress::COURSE_DIST);
                c.lwc1(1, course_dist);
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                c.g[V1][0] = c.lw(r32(c, V0));
                let active = c.g[V1][0] != 0;
                c.sub_s(1, 1, 0); // d
                if !active {
                    // Clamp the step against the previous distance.
                    c.cycles += 8;
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    c.lwc1(4, gp_m2(c, progress::STEP_LIMIT));
                    c.lwc1(2, r32(c, V0).wrapping_add(progress::DIST));
                    c.sub_s(3, 1, 2); // d - previous
                    c.c_lt_s(4, 3); // limit < step
                    if c.fcc() {
                        c.cycles += 2;
                        c.add_s(1, 2, 4);
                    } else {
                        c.neg_s(0, 4);
                        c.cycles += 4;
                        c.c_lt_s(3, 0);
                        if c.fcc() {
                            c.sub_s(1, 2, 4);
                        }
                    }
                }

                'branches: {
                    // `d <= 0`: the start of the course.
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_le_s(1, 0);
                    let ahead = !c.fcc();
                    c.g[A0][0] = c.g[S2][0];
                    if !ahead {
                        c.cycles += 8;
                        c.g[S0][0] = progress::STRIDE as u64;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x7438));
                        c.mult(0, S0, S2, S0); // s0 = i * 0x18
                        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                        c.sw(r32(c, V0).wrapping_add(progress::SEG), 0);
                        c.g[V1][0] = c.g[V0][0];
                        c.swc1(0, r32(c, V1).wrapping_add(progress::DIST));
                        break 'branches;
                    }
                    // Past the last segment?
                    c.cycles += 10;
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(progress::COURSE_COUNT));
                    c.g[A0][0] = progress::SEG_STRIDE as u64;
                    c.g[A1][0] = sx(r32(c, V0).wrapping_sub(1)); // last
                    c.mult(0, V0, A1, A0); // v0 = last * 0x14
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                    c.lwc1(0, r32(c, V1).wrapping_add(progress::SEG_DIST));
                    c.c_le_s(0, 1);
                    let before_last = !c.fcc();
                    c.g[V0][0] = progress::STRIDE as u64;
                    if !before_last {
                        c.cycles += 10;
                        c.g[S0][0] = progress::STRIDE as u64;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x7438));
                        c.mult(0, S0, S2, S0);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                        c.sw(r32(c, V0).wrapping_add(progress::SEG), c.g[A1][0]);
                        c.g[V1][0] = c.g[V0][0];
                        c.lwc1(0, course_dist);
                        c.swc1(0, r32(c, V1).wrapping_add(progress::DIST));
                        break 'branches;
                    }
                    // Walk from the slot's segment to the interval holding `d`.
                    c.cycles += 11;
                    c.g[V1][0] = sx(r32(c, S1).wrapping_sub(0x7438));
                    c.mult(0, V0, S2, V0); // v0 = i * 0x18
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[A1][0] = c.lw(r32(c, V1).wrapping_add(progress::SEG)); // k
                    c.mult(0, V1, A1, A0); // v1 = k * 0x14
                    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S0)));
                    c.lwc1(0, r32(c, V0).wrapping_add(progress::SEG_DIST));
                    c.c_le_s(0, 1);
                    let after = !c.fcc();
                    c.g[V0][0] = sx(r32(c, V1).wrapping_add(progress::SEG_DIST));
                    if !after {
                        c.cycles += 2;
                        let mut dist_at = r32(c, S0).wrapping_add(r32(c, V0));
                        loop {
                            c.cycles += 6;
                            dist_at = dist_at.wrapping_add(progress::SEG_STRIDE);
                            c.lwc1(0, dist_at);
                            c.c_le_s(0, 1);
                            let more = c.fcc();
                            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                            if !more {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x0021_0780);
                            }
                        }
                    }
                    // `d < segs[k].dist`: step back to the lower segment.
                    c.cycles += 8;
                    c.g[V0][0] = progress::SEG_STRIDE as u64;
                    c.mult(0, V0, A1, V0); // v0 = k * 0x14
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                    c.lwc1(0, r32(c, V1).wrapping_add(progress::SEG_DIST));
                    c.c_lt_s(1, 0);
                    let below = !c.fcc();
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(progress::SEG_DIST));
                    if !below {
                        c.cycles += 2;
                        let mut dist_at = r32(c, S0).wrapping_add(r32(c, V0));
                        loop {
                            c.cycles += 6;
                            dist_at = dist_at.wrapping_sub(progress::SEG_STRIDE);
                            c.lwc1(0, dist_at);
                            c.c_lt_s(1, 0);
                            let more = c.fcc();
                            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                            if !more {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x0021_07c0);
                            }
                        }
                    }
                    c.cycles += 8;
                    c.g[S0][0] = progress::STRIDE as u64;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x7438));
                    c.mult(0, S0, S2, S0);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                    c.g[V1][0] = c.g[V0][0];
                    c.sw(r32(c, V0).wrapping_add(progress::SEG), c.g[A1][0]);
                    c.swc1(1, r32(c, V1).wrapping_add(progress::DIST));
                }
                // The slot's lateral offset.
                c.cycles += 3;
                c.g[RA][0] = sx(0x0021_0800);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                callee::PROJECT_RIDER.call(c, 0x0021_0800)?;
                c.swc1(0, r32(c, S1).wrapping_add(progress::POS));
            }
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

// ---------------------------------------------------------------------------
// Race progress reset.

/// `void RaceProgress_ResetAll()` (0x2108f8): clear the six race-progress
/// records at 0x4c8bc8 (0x18 bytes each: an "in use" word then five zero
/// words) and set the "progress initialised" flag (`gp_2_m2-0x990`).
pub fn race_progress_reset_all(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x4d_0000;
    c.g[A1][0] = 1;
    c.g[V0][0] = sx(0x4d_0000u32.wrapping_sub(0x7438));
    c.g[V1][0] = c.g[V0][0];
    let first = c.g[V1][0] as u32;
    c.g[A0][0] = sx(first.wrapping_add(0x90));
    let one = c.g[A1][0];
    Addr(first).sw(c, one);
    let mut record = first;
    loop {
        c.cycles += 9;
        Addr(record.wrapping_add(4)).sw(c, 0);
        Addr(record.wrapping_add(8)).sw(c, 0);
        Addr(record.wrapping_add(0xc)).sw(c, 0);
        Addr(record.wrapping_add(0x10)).sw(c, 0);
        Addr(record.wrapping_add(0x14)).sw(c, 0);
        record = record.wrapping_add(0x18);
        c.g[V1][0] = sx(record);
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        Addr(record).sw(c, one);
        if event_due(c) {
            deliver_event_here(c, 0x0021_0910);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = 1;
    Addr(gp_2_m2(c, -0x990)).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void OVScreenReadPrivateMessage_OpenKeyboard(...)` (0x210f40): open the
/// keyboard popup for the private-message overlay.
pub fn ov_screen_read_private_message_open_keyboard(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let a2 = c.g[A2][0];
        frame.save(c, S2, 0x10);
        let self_ = c.g[A0][0];
        c.g[S0][0] = a2;
        c.g[A2][0] = c.g[T0][0];
        c.g[S1][0] = self_;
        let a3 = c.g[A3][0];
        c.g[S2][0] = a3;
        frame.save_ra(c, 0);
        c.g[T0][0] = c.g[T2][0];
        c.g[A3][0] = c.g[T1][0];
        call_ptr(c, 0x0021_0fa0, 0x0021_0f70)?;
        c.g[A0][0] = Addr(at(c, S1, 0xe0)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001c_cf00, 0x0021_0f7c)?;
        c.g[A0][0] = Addr(at(c, S1, 0xe0)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x001c_b418, 0x0021_0f88)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Keyboard popup.

/// `cKeyboardPopup* OVScreenReadPrivateMessage_CreateKeyboardPopup(
/// self, arg1, arg2, arg3, arg4)` (0x210fa0): allocate a 0x444-byte keyboard
/// popup, construct it, push it on the screen's state manager, set its
/// max length / key set / function keys, and remember it on the screen.
pub fn ovscreen_read_private_message_create_keyboard_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 58;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S3][0] = c.g[A3][0];
        c.g[A1][0] = 0x470000;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[T0][0];
        c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x1030));
        c.g[A0][0] = 0x444;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0021_0fe8)?;

        c.g[A1][0] = Addr(r(c, S0)).off(0x10).lw(c);
        c.g[T0][0] = 0xf;
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001c_b030, 0x0021_1000)?;

        c.g[V1][0] = Addr(r(c, S0)).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        c.sw(Addr(r(c, S0)).off(0xe0).0, c.g[V0][0]);
        c.g[V0][0] = Addr(r(c, V1)).off(0xd0).lw(c);
        c.g[A0][0] = Addr(r(c, V0)).off(0x10).lw(c);
        c.g[A0][0] = sx(p_m2(r(c, A0), 0x18));
        call_ptr(c, 0x0039_f290, 0x0021_101c)?;

        c.g[V0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.sw(Addr(r(c, V0)).off(0x18).0, c.g[S3][0]);
        c.g[A0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        call_ptr(c, 0x001c_d088, 0x0021_1030)?;

        c.g[V0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        c.g[A2][0] = 1;
        c.g[A3][0] = 2;
        c.g[A1][0] = 0x4b;
        c.sw(Addr(r(c, V0)).off(0x43c).0, 0);
        c.g[A0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        call_ptr(c, 0x001c_e3c8, 0x0021_104c)?;

        c.g[A0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        c.g[A1][0] = 1;
        call_ptr(c, 0x001c_e468, 0x0021_1058)?;

        c.g[V1][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        c.g[V0][0] = 1;
        c.lq(17, p_m2(r(c, SP), 0x40));
        c.sw(Addr(r(c, V1)).off(0x440).0, c.g[V0][0]);
        c.sw(Addr(r(c, S0)).off(0xdc).0, c.g[S4][0]);
        c.lq(16, p_m2(r(c, SP), 0x50));
        c.lq(18, p_m2(r(c, SP), 0x30));
        c.lq(19, p_m2(r(c, SP), 0x20));
        c.lq(20, p_m2(r(c, SP), 0x10));
        c.g[RA][0] = c.ld(r(c, SP));
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x60));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// HUD.

/// `HUDMeter_SetRect(out, _, area)` (0x20ec18): derives the meter's draw
/// rectangles (`out`, 0x34 bytes of floats) from the layout rectangle `area`
/// (x, y, w, h floats at `a2`).
///
/// An inset (`f7`) comes from the active meter's size (`gp-0x98c` object:
/// the int at +0x14 as float times the float at +0x34, times the float at
/// `gp-0x54f4`), or the constant 30.0 when there is no meter (`gp-0x988` is
/// 0). The result is written to `out` together with a copy of `area`'s
/// 16 bytes at `out+0x20` and a scaled `out+0x2c` at `out+0x30` (scale float
/// at `gp-0x54f0`). The float maths runs on the guest FPU registers like the
/// generated body.
pub fn hud_meter_set_rect_2(c: &mut Cpu) {
    let gp = c.g[GP][0] as u32;
    c.cycles += 3;
    c.g[V0][0] = c.lw(gp.wrapping_sub(0x988));
    let no_meter = c.g[V0][0] == 0;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    if !no_meter {
        c.cycles += 8;
        c.g[V0][0] = c.lw(gp.wrapping_sub(0x98c));
        c.lwc1(2, gp.wrapping_sub(0x54f4));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
        c.cvt_s_w(0, 0);
        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x34));
        c.mul_s(0, 0, 1);
        c.mul_s(7, 0, 2);
    } else {
        c.cycles += 2;
        c.g[AT][0] = 0x41f0_0000;
        c.mtc1(1, 7);
    }
    c.cycles += 53;
    c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = 0x4d0000;
    c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 5);
    c.add_s(0, 0, 2);
    c.lwc1(1, c.g[A2][0] as u32);
    c.mul_s(3, 3, 5);
    c.lwc1(4, (c.g[V0][0] as u32).wrapping_sub(0x7360));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x54f0));
    c.sub_s(0, 0, 7);
    c.add_s(1, 1, 3);
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
    c.swc1(1, c.g[SP][0] as u32);
    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
    c.ldr(2, c.g[SP][0] as u32);
    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
    c.sdr(2, c.g[A0][0] as u32);
    c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x8));
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.add_s(1, 1, 0);
    c.sub_s(1, 1, 7);
    c.sub_s(1, 1, 4);
    c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0xc));
    c.sub_s(3, 1, 3);
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.lwc1(2, c.g[A2][0] as u32);
    c.sub_s(3, 3, 4);
    c.mul_s(0, 0, 5);
    c.mul_s(3, 3, 6);
    c.add_s(2, 2, 0);
    c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x1c));
    c.add_s(0, 0, 4);
    c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0x18));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x14));
    c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
    c.ldr(2, c.g[A2][0] as u32);
    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0xf));
    c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x27));
    c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0x2f));
    c.sdr(3, (c.g[A0][0] as u32).wrapping_add(0x28));
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x2c));
    c.mul_s(0, 0, 6);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x30));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    c.pc = ra;
}

/// `void cOVStateHUD1P::DrawProgressMeterMP(cRider* rider, int right)` (0x0020fb40):
/// the multiplayer course-progress strip of the 1P HUD. Draws a vertical
/// meter (`this` = `a0`: X/Y/W/H at +0x20..+0x2c, pixels per course unit at
/// +0x30, last drawn rider index at +0x34; `a1` the rider, `a2` which side)
/// with one icon per racer (own, human, AI: draw prims 12/13/14) and per
/// course marker, fading at both ends. Early outs: HUD-off flag (gp-0x990),
/// no radar header, no rider. The racers' relative positions are rate-limited
/// towards their targets (`limitStep`, 46.296 per call, clamp +-2000) and
/// kept in the six-entry table at 0x4c8bc8. Every register, cycle and call is
/// kept as in the generated body (the body is its straight conversion with
/// the calls through `call_ptr` and the loop event checks).
pub fn c_ov_state_hud1_p_draw_progress_meter_mp(c: &mut Cpu) {
    guest(c, |c| {
        'b_210558: {
            'b_210554: {
                c.cycles += 25;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1d0));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x990));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.g[S3][0] = c.g[A2][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x1c8));
                c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x1b8));
                c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x198));
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x188));
                let t = c.g[V0][0] != 0u64;
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x180));
                if t {
                    break 'b_210554;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x994));
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
                    if t {
                        break 'b_210558;
                    } else {
                        c.cycles += 2;
                        let t = c.g[A1][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x998));
                        if t {
                            break 'b_210558;
                        } else {
                            c.cycles += 3;
                            c.g[S7][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9a0));
                            let t = c.g[S7][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[V0][0]);
                            if t {
                                break 'b_210558;
                            } else {
                                c.cycles += 29;
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x6c0));
                                c.g[FP][0] = 0x4d0000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x0020fbe0);
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0020fbe0)?;
                                c.g[S5][0] = c.g[V0][0];
                                c.g[A0][0] = 0x18;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.mult(0, 4, 21, 4);
                                c.g[V0][0] = 0x4d0000;
                                c.g[AT][0] = 0xffffffffc4fa0000;
                                c.mtc1(1, 7);
                                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7438));
                                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                                c.g[V1][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[S6][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] = c.g[V1][0];
                                c.g[AT][0] = 0x44fa0000;
                                c.mtc1(1, 6);
                                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x54c8));
                                c.g[T1][0] = 0x1;
                                c.lwc1(4, (c.g[V1][0] as u32).wrapping_add(0xc));
                                c.g[T0][0] = 0u64;
                                c.g[S4][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x78));
                                c.lwc1(22, (c.g[V0][0] as u32).wrapping_add(0x8));
                                'l_20fc38: loop {
                                    'b_20fc44: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                        if c.g[V0][0] != c.g[S5][0] {
                                            c.sw(c.g[A2][0] as u32, 0u64);
                                            break 'b_20fc44;
                                        } else {
                                            break 'b_20fc44;
                                        }
                                    }
                                    'b_20fcd0: {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[T0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                            break 'b_20fcd0;
                                        } else {
                                            'b_20fc8c: {
                                                'b_20fc74: {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[A2][0] as u32).wrapping_sub(0x8),
                                                    );
                                                    c.sub_s(0, 0, 4);
                                                    c.c_lt_s(0, 7);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_20fc74;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[AT][0] = 0xffffffffc4fa0000;
                                                        c.mtc1(1, 0);
                                                        let t = true;
                                                        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                                        if t {
                                                            break 'b_20fc8c;
                                                        } else {
                                                            break 'b_20fc74;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.c_lt_s(6, 0);
                                                let t = !c.fcc();
                                                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                                if t {
                                                    break 'b_20fc8c;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[AT][0] = 0x44fa0000;
                                                    c.mtc1(1, 0);
                                                    break 'b_20fc8c;
                                                }
                                            }
                                            'b_20fcc8: {
                                                c.cycles += 2;
                                                if c.g[V0][0] == 0u64 {
                                                    c.swc1(0, c.g[A3][0] as u32);
                                                    break 'b_20fcc8;
                                                } else {
                                                    'b_20fcc4: {
                                                        'b_20fcb4: {
                                                            c.cycles += 6;
                                                            c.lwc1(2, c.g[A3][0] as u32);
                                                            c.lwc1(
                                                                3,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54c4),
                                                            );
                                                            c.sub_s(1, 0, 2);
                                                            c.c_lt_s(3, 1);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_20fcb4;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.add_s(0, 2, 3);
                                                                if t {
                                                                    break 'b_20fcc4;
                                                                } else {
                                                                    break 'b_20fcb4;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.c_lt_s(1, 5);
                                                        if c.fcc() {
                                                            c.sub_s(0, 2, 3);
                                                            break 'b_20fcc4;
                                                        } else {
                                                            break 'b_20fcc4;
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.swc1(0, c.g[A3][0] as u32);
                                                    break 'b_20fcc8;
                                                }
                                            }
                                            c.cycles += 2;
                                            c.sw(c.g[A2][0] as u32, c.g[T1][0]);
                                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                            break 'b_20fcd0;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = ((c.g[T0][0] as i64) < 6i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x18));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0020fc38);
                                        }
                                        continue 'l_20fc38;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V0][0] = 0x3;
                                        c.g[V1][0] = 0xffffffffffffffff;
                                        'l_20fce8: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                            let t = c.g[V0][0] != c.g[V1][0];
                                            c.g[A0][0] = 0x80;
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0020fce8);
                                                }
                                                continue 'l_20fce8;
                                            } else {
                                                c.cycles += 5;
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 0);
                                                c.g[V1][0] = c.g[SP][0];
                                                c.g[A1][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    c.g[A0][0],
                                                );
                                                'l_20fd18: loop {
                                                    c.cycles += 8;
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x14),
                                                        c.g[A0][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                        c.g[A0][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x1c),
                                                        c.g[A0][0],
                                                    );
                                                    c.swc1(
                                                        0,
                                                        (c.g[V1][0] as u32).wrapping_add(0x8),
                                                    );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x30));
                                                    c.g[V0][0] = ((c.g[V1][0] as i64)
                                                        < (c.g[A1][0] as i64))
                                                        as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.sw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x10),
                                                            c.g[A0][0],
                                                        );
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0020fd18);
                                                        }
                                                        continue 'l_20fd18;
                                                    } else {
                                                        'b_20fd48: {
                                                            c.cycles += 2;
                                                            let t = c.g[S3][0] != 0u64;
                                                            c.lwc1(
                                                                21,
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            if t {
                                                                break 'b_20fd48;
                                                            } else {
                                                                c.cycles += 2;
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[S2][0] as u32)
                                                                        .wrapping_add(0x28),
                                                                );
                                                                c.add_s(21, 21, 0);
                                                                break 'b_20fd48;
                                                            }
                                                        }
                                                        'b_210240: {
                                                            c.cycles += 146;
                                                            c.g[A1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[S0][0] = 0u64;
                                                            c.g[AT][0] = 0x3f800000;
                                                            c.mtc1(1, 20);
                                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32)
                                                                .wrapping_add(0x10d8));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0xf8));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xfc));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x0020fd6c);
                                                            c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x0020fd6c)?;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[T0][0] = 0xfffffffffe7f0000;
                                                            c.g[T0][0] = c.g[T0][0] | 0xffff;
                                                            c.g[T4][0] = 0x1000000;
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[T1][0] = 0xffffffffffcf0000;
                                                            c.g[T1][0] = c.g[T1][0] | 0xffff;
                                                            c.g[T3][0] = 0xfffffffffff00000;
                                                            c.ldl(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(3, c.g[V0][0] as u32);
                                                            c.ldl(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A2][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.sdl(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x1b),
                                                            );
                                                            c.sdr(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x14),
                                                            );
                                                            c.sdl(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x23),
                                                            );
                                                            c.sdr(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x1c),
                                                            );
                                                            c.sw(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x24),
                                                                c.g[A2][0],
                                                            );
                                                            c.g[T3][0] = c.g[T3][0] | 0xfff;
                                                            c.g[T2][0] = 0x10000;
                                                            c.g[T5][0] = 0xffffffffffffff83;
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[T2][0] = c.g[T2][0] | 0x4000;
                                                            c.g[A2][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[A1][0] = 0x500000;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0xe60));
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xe84),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V0][0] = 0x500000;
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[A3][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1420));
                                                            c.ldl(
                                                                14,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(14, c.g[A3][0] as u32);
                                                            c.ldl(
                                                                4,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                4,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A2][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.sdl(
                                                                14,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(14, c.g[V1][0] as u32);
                                                            c.sdl(
                                                                4,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                4,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x10),
                                                                c.g[A2][0],
                                                            );
                                                            c.g[A3][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[A0][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                                                            c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A0][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                                                            c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A2][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
                                                            c.g[V0][0] = c.g[V0][0] | 0x14;
                                                            c.sw(
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x10d8));
                                                            c.g[A0][0] = c.lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x110));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x114));
                                                            let t = c.g[V0][0] as u32;
                                                            c.g[RA][0] = sx(0x0020fe74);
                                                            c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x0020fe74)?;
                                                            c.g[V0][0] = sx((c.g[FP][0] as u32)
                                                                .wrapping_sub(0x73a8));
                                                            c.lwc1(
                                                                27,
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0x24),
                                                            );
                                                            c.g[T0][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc));
                                                            c.g[T2][0] = 0xfffffffffffffc1f;
                                                            c.lwc1(
                                                                0,
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0x2c),
                                                            );
                                                            c.g[A1][0] = 0x4;
                                                            c.lwc1(
                                                                1,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.g[A2][0] = c.g[SP][0];
                                                            c.add_s(23, 27, 0);
                                                            c.lwc1(
                                                                4,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54c0),
                                                            );
                                                            c.swc1(1, c.g[SP][0] as u32);
                                                            c.g[A3][0] = 0u64;
                                                            c.lwc1(
                                                                0,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54bc),
                                                            );
                                                            c.sub_s(5, 21, 4);
                                                            c.lwc1(
                                                                3,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.add_s(4, 21, 4);
                                                            c.sub_s(26, 22, 0);
                                                            c.lwc1(
                                                                2,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54b8),
                                                            );
                                                            c.swc1(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.lwc1(
                                                                1,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x14),
                                                            );
                                                            c.add_s(29, 26, 2);
                                                            c.swc1(
                                                                1,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x34),
                                                            );
                                                            c.lwc1(
                                                                1,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.swc1(
                                                                1,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x60),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x64),
                                                            );
                                                            c.lwc1(
                                                                1,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x14),
                                                            );
                                                            c.swc1(
                                                                1,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x90),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.swc1(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.swc1(
                                                                23,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc4),
                                                            );
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc8),
                                                                0u64,
                                                            );
                                                            c.swc1(
                                                                20,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xcc),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x94),
                                                            );
                                                            c.lq(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.swc1(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc8),
                                                                0u64,
                                                            );
                                                            c.sq(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.swc1(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x50),
                                                            );
                                                            c.swc1(
                                                                27,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc4),
                                                            );
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc8),
                                                                0u64,
                                                            );
                                                            c.lq(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.g[T1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.swc1(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc8),
                                                                0u64,
                                                            );
                                                            c.sq(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x80),
                                                            );
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xb0),
                                                            );
                                                            c.g[A0][0] = c.lw((c.g[T1][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                                                            c.g[V0][0] = c.g[V0][0] | 0x120;
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw(c.g[T0][0] as u32);
                                                            c.g[V0][0] = c.lw((c.g[T1][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.sh(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                                c.g[V1][0],
                                                            );
                                                            c.g[T0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[V0][0] = c.lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0x10d8));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x238));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x23c));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x0020ff7c);
                                                            c.g[A0][0] = sx((c.g[T0][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x0020ff7c)?;
                                                            c.g[AT][0] = 0x42700000;
                                                            c.mtc1(1, 0);
                                                            c.sub_s(25, 23, 0);
                                                            let t = (c.g[S4][0] as i64) <= 0;
                                                            c.add_s(24, 27, 0);
                                                            if t {
                                                                break 'b_210240;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.lwc1(
                                                                    28,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x54b4),
                                                                );
                                                                c.mov_s(22, 20);
                                                                c.g[V1][0] = 0x4d0000;
                                                                'l_20ffa0: loop {
                                                                    'b_210234: {
                                                                        'b_210230: {
                                                                            c.cycles += 9;
                                                                            c.g[V0][0] = 0x18;
                                                                            c.g[V1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x7430,
                                                                                    ));
                                                                            c.lo[0] = c.g[V1][0];
                                                                            c.madd(0, 2, 16, 2);
                                                                            c.lwc1(
                                                                                0,
                                                                                c.g[V0][0] as u32,
                                                                            );
                                                                            c.c_lt_s(0, 26);
                                                                            let t = c.fcc();
                                                                            c.g[S1][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_210230;
                                                                            } else {
                                                                                c.cycles += 4;
                                                                                c.c_lt_s(29, 0);
                                                                                if c.fcc() {
                                                                                    c.g[S0][0] =
                                                                                        c.g[S1][0];
                                                                                    break 'b_210234;
                                                                                } else {
                                                                                    'b_21002c: {
                                                                                        c.cycles +=
                                                                                            7;
                                                                                        c.sub_s(
                                                                                            0, 0,
                                                                                            26,
                                                                                        );
                                                                                        c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x30));
                                                                                        c.g[A0]
                                                                                            [0] =
                                                                                            0xc;
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x8;
                                                                                        c.mul_s(
                                                                                            0, 0, 1,
                                                                                        );
                                                                                        let t = c.g[S5][0] == c.g[S0][0];
                                                                                        c.sub_s(
                                                                                            20, 23,
                                                                                            0,
                                                                                        );
                                                                                        if t {
                                                                                            break 'b_21002c;
                                                                                        } else {
                                                                                            'b_210024: {
                                                                                                c.cycles += 11;
                                                                                                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                                                                                                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                                                                                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                                                                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                                                                                                let t = c.g[V0][0] as u32;
                                                                                                c.g[RA][0] = sx(0x00210014);
                                                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                                                                                call_ptr(c, t, 0x00210014)?;
                                                                                                let t = c.g[V0][0] == 0u64;
                                                                                                c.g[A0][0] = 0xd;
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_210024;
                                                                                                } else {
                                                                                                    c.cycles += 2;
                                                                                                    let t = true;
                                                                                                    c.g[V1][0] = 0xa;
                                                                                                    if t {
break 'b_21002c;
} else {
break 'b_210024;
}
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 2;
                                                                                            c.g[A0][0] = 0xe;
                                                                                            c.g[V1][0] = 0x9;
                                                                                            break 'b_21002c;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 14;
                                                                                    c.g[V1][0] = sx(
                                                                                        (c.g[V1][0]
                                                                                            as u32)
                                                                                            << 4,
                                                                                    );
                                                                                    c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x73a8));
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                    c.g[AT][0] =
                                                                                        0x3f000000;
                                                                                    c.mtc1(1, 1);
                                                                                    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                                                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                                                    c.g[T1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                                                                    c.mul_s(
                                                                                        0, 0, 1,
                                                                                    );
                                                                                    c.sub_s(
                                                                                        4, 20, 0,
                                                                                    );
                                                                                    c.c_le_s(4, 27);
                                                                                    let t = c.fcc();
                                                                                    c.add_s(
                                                                                        5, 20, 0,
                                                                                    );
                                                                                    if t {
                                                                                        break 'b_210230;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.c_le_s(
                                                                                            23, 5,
                                                                                        );
                                                                                        if c.fcc() {
                                                                                            c.g[S0][0] = c.g[S1][0];
                                                                                            break 'b_210234;
                                                                                        } else {
                                                                                            'b_2100a8: {
                                                                                                'b_2100a0: {
                                                                                                    'b_210090: {
                                                                                                        c.cycles += 5;
                                                                                                        c.c_lt_s(4, 24);
                                                                                                        c.g[AT][0] = 0x3f800000;
                                                                                                        c.mtc1(1, 1);
                                                                                                        let t = !c.fcc();

                                                                                                        if t {
break 'b_210090;
} else {
c.cycles += 2;
let t = true;
c.sub_s(0, 24, 4);
if t {
break 'b_2100a0;
} else {
break 'b_210090;
}
}
                                                                                                    }
                                                                                                    c.cycles += 3;
                                                                                                    c.c_lt_s(25, 5);
                                                                                                    let t = !c.fcc();

                                                                                                    if t {
break 'b_2100a8;
} else {
c.cycles += 1;
c.sub_s(0, 5, 25);
break 'b_2100a0;
}
                                                                                                }
                                                                                                c.cycles += 2;
                                                                                                c.mul_s(0, 0, 28);
                                                                                                c.sub_s(1, 1, 0);
                                                                                                break 'b_2100a8;
                                                                                            }
                                                                                            'b_2100bc: {
                                                                                                c.cycles += 5;
                                                                                                c.mtc1(0, 0);
                                                                                                c.c_lt_s(1, 0);
                                                                                                if c.fcc() {
c.mov_s(1, 0);
break 'b_2100bc;
} else {
break 'b_2100bc;
}
                                                                                            }
                                                                                            'b_2100d4: {
                                                                                                c.cycles += 4;
                                                                                                c.lwc1(2, (c.g[T1][0] as u32).wrapping_add(0x10));
                                                                                                c.lwc1(3, (c.g[T1][0] as u32).wrapping_add(0x14));
                                                                                                let t = c.g[S3][0] != 0u64;
                                                                                                c.mov_s(0, 2);
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_2100d4;
                                                                                                } else {
                                                                                                    c.cycles += 2;
                                                                                                    c.mov_s(2, 3);
                                                                                                    c.mov_s(3, 0);
                                                                                                    break 'b_2100d4;
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 9;
                                                                                            c.g[AT][0] = 0x43000000;
                                                                                            c.mtc1(
                                                                                                1,
                                                                                                0,
                                                                                            );
                                                                                            c.g[T2][0] = sx((c.g[A0][0] as u32) << 5);
                                                                                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                                                            c.g[A0][0] = 0x3;
                                                                                            c.mul_s(0, 1, 0);
                                                                                            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xac));
                                                                                            c.cvt_w_s(1, 0);
                                                                                            c.mfc1(
                                                                                                5,
                                                                                                1,
                                                                                            );
                                                                                            'l_2100f8: loop {
                                                                                                c.cycles += 7;
                                                                                                c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                                                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                                                                                let t = (c.g[A0][0] as i64) >= 0;
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                                                                                                if t
                                                                                                {
                                                                                                    if event_due(c) { deliver_event_here(c, 0x002100f8); }
                                                                                                    continue 'l_2100f8;
                                                                                                } else {
                                                                                                    'b_2101d4: {
                                                                                                        'b_210190: {
                                                                                                            c.cycles += 13;
                                                                                                            c.swc1(2, c.g[SP][0] as u32);
                                                                                                            c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
                                                                                                            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                                            c.lwc1(1, (c.g[T1][0] as u32).wrapping_add(0xc));
                                                                                                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                                                                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                                                                                                            c.lwc1(2, (c.g[T1][0] as u32).wrapping_add(0x18));
                                                                                                            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                                                                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x64));
                                                                                                            c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x18));
                                                                                                            let t = c.g[S3][0] == 0u64;
                                                                                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
                                                                                                            if t {
break 'b_210190;
} else {
c.cycles += 18;
c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.add_s(0, 21, 0);
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xc4));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xcc));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc4));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
let t = true;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
if t {
break 'b_2101d4;
} else {
break 'b_210190;
}
}
                                                                                                        }
                                                                                                        c.cycles += 17;
                                                                                                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                                                                                                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
                                                                                                        c.sub_s(0, 21, 0);
                                                                                                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xc4));
                                                                                                        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xcc));
                                                                                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
                                                                                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                                        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
                                                                                                        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc4));
                                                                                                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                                                                        c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        break 'b_2101d4;
                                                                                                    }
                                                                                                    c.cycles += 23;
                                                                                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
                                                                                                    c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                                                                                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                                                                                    c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                                                                                    c.g[V1][0] = 0xfffffffffffffc1f;
                                                                                                    c.g[A1][0] = 0x4;
                                                                                                    c.g[A2][0] = c.g[SP][0];
                                                                                                    c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
                                                                                                    c.g[A3][0] = 0u64;
                                                                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                                                                                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                                                                                    c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                                                                                                    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                                                                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
                                                                                                    c.g[V1][0] = c.lw(c.g[T1][0] as u32);
                                                                                                    c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                                                                                                    c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                                                                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                                                                                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
                                                                                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
                                                                                                    let t = c.g[V1][0] as u32;
                                                                                                    c.g[RA][0] = sx(0x00210230);
                                                                                                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                                                                                    call_ptr(c, t, 0x00210230)?;
                                                                                                    break 'b_210230;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 1;
                                                                        c.g[S0][0] = c.g[S1][0];
                                                                        break 'b_210234;
                                                                    }
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = ((c.g[S0][0]
                                                                        as i64)
                                                                        < (c.g[S4][0] as i64))
                                                                        as u64;
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.g[V1][0] = 0x4d0000;
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x0020ffa0,
                                                                            );
                                                                        }
                                                                        continue 'l_20ffa0;
                                                                    } else {
                                                                        break 'b_210240;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[A0][0] = 0x80;
                                                        c.g[V1][0] = 0x3;
                                                        c.g[V0][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xac));
                                                        'l_210250: loop {
                                                            c.cycles += 7;
                                                            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            let t = (c.g[V1][0] as i64) >= 0;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x30));
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x00210250,
                                                                    );
                                                                }
                                                                continue 'l_210250;
                                                            } else {
                                                                'b_21052c: {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[S7][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    let t = c.g[A1][0] == 0u64;
                                                                    c.g[V1][0] = 0x530000;
                                                                    if t {
                                                                        break 'b_21052c;
                                                                    } else {
                                                                        'b_21029c: {
                                                                            c.cycles += 5;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x8d0,
                                                                                ));
                                                                            c.g[V0][0] = sx(((c.g
                                                                                [V0][0]
                                                                                as u32)
                                                                                as i32
                                                                                >> 9)
                                                                                as u32);
                                                                            c.g[V0][0] =
                                                                                c.g[V0][0] & 0x1;
                                                                            let t =
                                                                                c.g[V0][0] == 0u64;
                                                                            c.g[S1][0] = 0x1;
                                                                            if t {
                                                                                break 'b_21029c;
                                                                            } else {
                                                                                c.cycles += 4;
                                                                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x88));
                                                                                if c.g[A0][0] != 0 {
                                                                                    c.g[S1][0] =
                                                                                        0u64;
                                                                                }
                                                                                break 'b_21029c;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        let t = (c.g[A1][0] as i64)
                                                                            <= 0;
                                                                        c.g[A0][0] = 0u64;
                                                                        if t {
                                                                            break 'b_21052c;
                                                                        } else {
                                                                            c.cycles += 9;
                                                                            c.g[V0][0] = 0x4d0000;
                                                                            c.g[AT][0] = 0x3f000000;
                                                                            c.mtc1(1, 22);
                                                                            c.lwc1(
                                                                                28,
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0x54b0,
                                                                                    ),
                                                                            );
                                                                            c.g[S4][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x7358,
                                                                                    ));
                                                                            c.g[AT][0] = 0x3f800000;
                                                                            c.mtc1(1, 20);
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xd0,
                                                                                ));
                                                                            'l_2102c8: loop {
                                                                                'b_21051c: {
                                                                                    'b_210518: {
                                                                                        c.cycles +=
                                                                                            7;
                                                                                        c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
                                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                        c.c_lt_s(
                                                                                            1, 26,
                                                                                        );
                                                                                        let t =
                                                                                            c.fcc();
                                                                                        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                                                        if t {
                                                                                            break 'b_210518;
                                                                                        } else {
                                                                                            c.cycles += 4;
                                                                                            c.c_lt_s(29, 1);
                                                                                            if c.fcc() {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_21051c;
} else {
'b_210310: {
c.cycles += 4;
c.g[V1][0] = c.lw(c.g[V0][0] as u32);
c.g[V0][0] = 0x1;
if c.g[V1][0] != c.g[V0][0] {
c.sub_s(1, 1, 26);
break 'b_210310;
} else {
c.cycles += 2;
if c.g[S1][0] == 0u64 {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_21051c;
} else {
c.cycles += 1;
c.sub_s(1, 1, 26);
break 'b_210310;
}
}
}
c.cycles += 16;
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x30));
c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
c.mul_s(1, 1, 0);
c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
c.g[T1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
c.mul_s(7, 2, 22);
c.mul_s(0, 0, 22);
c.sub_s(1, 23, 1);
c.sub_s(5, 1, 0);
c.c_le_s(5, 27);
let t = c.fcc();
c.add_s(6, 1, 0);
if t {
break 'b_210518;
} else {
c.cycles += 4;
c.c_le_s(23, 6);
if c.fcc() {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_21051c;
} else {
'b_210394: {
'b_21038c: {
'b_21037c: {
c.cycles += 5;
c.c_lt_s(5, 24);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 2);
let t = !c.fcc();

if t {
break 'b_21037c;
} else {
c.cycles += 2;
let t = true;
c.sub_s(0, 24, 5);
if t {
break 'b_21038c;
} else {
break 'b_21037c;
}
}
}
c.cycles += 3;
c.c_lt_s(25, 6);
let t = !c.fcc();

if t {
break 'b_210394;
} else {
c.cycles += 1;
c.sub_s(0, 6, 25);
break 'b_21038c;
}
}
c.cycles += 2;
c.mul_s(0, 0, 28);
c.sub_s(2, 2, 0);
break 'b_210394;
}
'b_2103a8: {
c.cycles += 5;
c.mtc1(0, 0);
c.c_lt_s(2, 0);
if c.fcc() {
c.mov_s(2, 0);
break 'b_2103a8;
} else {
break 'b_2103a8;
}
}
'b_2103c8: {
'b_2103c0: {
c.cycles += 3;
c.lwc1(3, (c.g[T1][0] as u32).wrapping_add(0x10));
let t = c.g[S3][0] == 0u64;
c.lwc1(4, (c.g[T1][0] as u32).wrapping_add(0x14));
if t {
break 'b_2103c0;
} else {
c.cycles += 3;
c.add_s(0, 3, 4);
let t = true;
c.mul_s(3, 0, 22);
if t {
break 'b_2103c8;
} else {
break 'b_2103c0;
}
}
}
c.cycles += 2;
c.add_s(0, 3, 4);
c.mul_s(4, 0, 22);
break 'b_2103c8;
}
c.cycles += 20;
c.swc1(3, c.g[SP][0] as u32);
c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.g[AT][0] = 0x43000000;
c.mtc1(1, 0);
c.g[V1][0] = 0x3;
c.lwc1(1, (c.g[T1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xac));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mul_s(0, 2, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.lwc1(2, (c.g[T1][0] as u32).wrapping_add(0xc));
c.cvt_w_s(1, 0);
c.mfc1(4, 1);
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x60));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
c.lwc1(1, (c.g[T1][0] as u32).wrapping_add(0x18));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x90));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x64));
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x18));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
'l_210418: loop {
c.cycles += 7;
c.sw(c.g[V0][0] as u32, c.g[A0][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
let t = (c.g[V1][0] as i64) >= 0;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
if t {
if event_due(c) { deliver_event_here(c, 0x00210418); } continue 'l_210418;
} else {
'b_2104bc: {
'b_210480: {
c.cycles += 2;
if c.g[S3][0] == 0u64 {
c.sub_s(0, 21, 7);
break 'b_210480;
} else {
c.cycles += 17;
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.add_s(0, 21, 7);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc4));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xc4));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
let t = true;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
if t {
break 'b_2104bc;
} else {
break 'b_210480;
}
}
}
c.cycles += 15;
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc4));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xc4));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
break 'b_2104bc;
}
c.cycles += 23;
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V1][0] = 0xfffffffffffffc1f;
c.g[A1][0] = 0x4;
c.g[A2][0] = c.g[SP][0];
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[A3][0] = 0u64;
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | 0x160;
c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = c.lw(c.g[T1][0] as u32);
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x00210518);
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
call_ptr(c, t, 0x00210518)?;
break 'b_210518;
}
}
}
}
}
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 1;
                                                                                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
                                                                                    break 'b_21051c;
                                                                                }
                                                                                c.cycles += 4;
                                                                                c.g[A0][0] =
                                                                                    c.g[S0][0];
                                                                                c.g[V0][0] = ((c.g
                                                                                    [A0][0]
                                                                                    as i64)
                                                                                    < (c.g[V0][0]
                                                                                        as i64))
                                                                                    as u64;
                                                                                let t = c.g[V0][0]
                                                                                    != 0u64;
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                                                                                if t {
                                                                                    if event_due(c)
                                                                                    {
                                                                                        deliver_event_here(c, 0x002102c8);
                                                                                    }
                                                                                    continue 'l_2102c8;
                                                                                } else {
                                                                                    break 'b_21052c;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 10;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x854));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0xe84));
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x10d8));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x14));
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0xe84),
                                                                    c.g[V0][0],
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lh((c.g[A1][0] as u32)
                                                                        .wrapping_add(0x100));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A1][0] as u32)
                                                                        .wrapping_add(0x104));
                                                                let t = c.g[V0][0] as u32;
                                                                c.g[RA][0] = sx(0x00210550);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V1][0] as u32,
                                                                    ));
                                                                call_ptr(c, t, 0x00210550)?;
                                                                c.sw(
                                                                    (c.g[S2][0] as u32)
                                                                        .wrapping_add(0x34),
                                                                    c.g[S5][0],
                                                                );
                                                                break 'b_210554;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
            break 'b_210558;
        }
        c.cycles += 21;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x1c8));
        c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x1b8));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x1a8));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x198));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x188));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x180));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenReadPrivateMessage_Init(OVStateOverlay* self)` (0x210940): create the
/// ReadPrivateMessage screen, show the sender name, set up the agreement scroll bar and
/// the chat text scroll widget, then run `cOVStateOverlay_Init`.
pub fn ov_screen_read_private_message_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_210b3c: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = 0x470000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ce8));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[RA][0] = sx(0x0021096c);
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x0021096c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x00210980);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x00210980)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_210b3c;
            } else {
                'b_2109f0: {
                    c.cycles += 13;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00210998);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039c870, 0x00210998)?;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x002109a4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d00));
                    call_ptr(c, 0x00317670, 0x002109a4)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x002109b0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x002109b0)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        break 'b_2109f0;
                    } else {
                        c.cycles += 13;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x002109d0);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002109d0)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = 0x7;
                        c.g[RA][0] = sx(0x002109e0);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                        call_ptr(c, 0x0014dd58, 0x002109e0)?;
                        c.g[A1][0] = 0x530000;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x002109f0);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4fc8));
                        call_ptr(c, 0x003a0c50, 0x002109f0)?;
                        break 'b_2109f0;
                    }
                }
                'b_210b34: {
                    c.cycles += 44;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x002109fc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d10));
                    call_ptr(c, 0x00317670, 0x002109fc)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00210a08);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00210a08)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d20));
                    c.g[V0][0] = c.g[V0][0] | 0x1;
                    c.g[RA][0] = sx(0x00210a24);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x00210a24)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00210a30);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00210a30)?;
                    c.g[A0][0] = 0x470000;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00210a40);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d30));
                    call_ptr(c, 0x00317670, 0x00210a40)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00210a4c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00210a4c)?;
                    c.g[A0][0] = 0x470000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00210a5c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d40));
                    call_ptr(c, 0x00317670, 0x00210a5c)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00210a68);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00210a68)?;
                    c.g[V1][0] = 0x2;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x80), c.g[S0][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x7c), c.g[S1][0]);
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x17f8));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x88), c.g[V1][0]);
                    c.g[RA][0] = sx(0x00210a88);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x84), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x00210a88)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00210a94);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00210a94)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_210b34;
                    } else {
                        c.cycles += 37;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = c.g[V0][0] | 0x1;
                        c.g[RA][0] = sx(0x00210ab4);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                        call_ptr(c, 0x003a1310, 0x00210ab4)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00210acc);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x00210acc)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.g[A1][0] = c.g[SP][0];
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.g[V0][0] | 0x80;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.swc1(0, c.g[SP][0] as u32);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x00210b08);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x00210b08)?;
                        c.g[V1][0] = 0x4d0000;
                        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x72f8));
                        c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.ldr(2, c.g[A2][0] as u32);
                        c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0xaf));
                        c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0xa8));
                        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0xb7));
                        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0xb0));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xb8), c.g[S2][0]);
                        break 'b_210b34;
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x00210b3c);
                c.g[A0][0] = c.g[S3][0];
                Callee {
                    addr: 0x0020a380,
                    f: ee::s05::cOVStateOverlay_Init,
                }
                .call(c, 0x00210b3c)?;
                break 'b_210b3c;
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

// ---------------------------------------------------------------- OVScreenTrophy_OnWidgetAction

/// `void cOVStateTrophy::onWidgetEvent(cOVStateTrophy* self, Event* ev,
/// int kind)` (0x20e9a0). Kind 5 calls the item's virtual method at
/// `+0x5c` with the event's (1.0, 1.0, 0, 0) vector, and, when the item is
/// the active one (`+0x18 == 1`), the list's current item's virtual method
/// at `+0x24`. Kind 6 calls the list's item method at `+0x2c` and, for a
/// non-null result, pushes a UI state (`cUIStateStack_push`). Every
/// path ends by popping the frame; kinds other than 5 and 6 pop at once.
pub fn ov_screen_trophy_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let no_event = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        'done: {
            'cleanup: {
                if no_event {
                    break 'cleanup;
                }
                'slot: {
                    'kind_is_five: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x5;
                        let is_five = c.g[A2][0] == c.g[V0][0];
                        c.g[V0][0] = 0x6;
                        if is_five {
                            break 'kind_is_five;
                        }
                        c.cycles += 2;
                        if c.g[A2][0] == c.g[V0][0] {
                            c.g[V0][0] = Addr(trophy_s1(c).wrapping_add(0x10)).lw(c);
                            break 'slot;
                        }
                        c.cycles += 2;
                        frame.restore(c, S0, 0x30);
                        break 'done;
                    }

                    // Kind 5: the item's method with the event's vector.
                    'pick: {
                        c.cycles += 16;
                        c.g[V1][0] = Addr(trophy_s0(c).wrapping_add(0x8)).lw(c);
                        c.g[A1][0] = c.g[SP][0];
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 0);
                        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
                        c.swc1(0, sp(c).wrapping_add(0x4));
                        c.swc1(0, sp(c));
                        Addr(sp(c).wrapping_add(0x8)).sw(c, 0);
                        Addr(sp(c).wrapping_add(0xc)).sw(c, 0);
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                        let method = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(trophy_s0(c).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0020_ea08)?;
                        c.g[V1][0] = Addr(trophy_s0(c).wrapping_add(0x18)).lw(c);
                        c.g[V0][0] = 0x1;
                        let not_active = c.g[V1][0] != c.g[V0][0];
                        c.g[A1][0] = 0;
                        if not_active {
                            break 'pick;
                        }
                        c.cycles += 10;
                        c.g[V0][0] = Addr(trophy_s1(c).wrapping_add(0x10)).lw(c);
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = 0x1;
                        c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
                        c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x4)).lw(c);
                        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
                        let method2 = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method2, 0x0020_ea3c)?;
                        c.g[A1][0] = c.g[V0][0];
                    }
                    c.cycles += 5;
                    c.g[A0][0] = Addr(trophy_s1(c).wrapping_add(0x10)).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, 0x0020_ea4c)?;
                    frame.restore(c, S0, 0x30);
                    break 'done;
                }

                // Kind 6: the list's current item's method.
                c.cycles += 10;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = Addr(trophy_s0(c).wrapping_add(0x18)).lw(c);
                c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2c)).lw(c);
                let method3 = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method3, 0x0020_ea74)?;
                let none = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if !none {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(trophy_s1(c).wrapping_add(0x10)).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, 0x0020_ea88)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The trophy screen (`s1`) as an address.
pub(crate) fn trophy_s1(c: &Cpu) -> u32 {
    c.g[S1][0] as u32
}

/// The widget event (`s0`) as an address.
pub(crate) fn trophy_s0(c: &Cpu) -> u32 {
    c.g[S0][0] as u32
}

// ---------------------------------------------------------------- OVScreenReadPrivateMessage_TransitionIn

/// `void OVScreenReadPrivateMessage_TransitionIn(screen, message)`
/// (0x210b58). With a message, it takes the game comm's queued chat line
/// (`cGameComm_getQueuedChat`, kind 0x41, into `screen+0x9c`) and shows
/// it with `OVScreenReadPrivateMessage_SetMessageText`; with no line it
/// opens the keyboard for a reply (`OVScreenReadPrivateMessage_OpenKeyboard`,
/// the 0x4a2560 prompt, kind 0x3d). Without a message it only pops the frame.
/// Returns 1.
pub fn ov_screen_read_private_message_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let no_message = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        'done: {
            'after_text: {
                if no_message {
                    break 'after_text;
                }
                'queued: {
                    c.cycles += 7;
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0021_0b84);
                    c.g[A2][0] = 0x41;
                    call_ptr(c, 0x0025_8160, 0x0021_0b84)?;
                    let no_line = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if no_line {
                        break 'queued;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x0021_0bd8, 0x0021_0b98)?;
                    frame.restore(c, S0, 0x20);
                    break 'done;
                }
                // No queued line: open the keyboard for a reply.
                c.cycles += 8;
                c.g[A2][0] = 0x4a0000;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2560));
                c.g[A1][0] = 0;
                c.g[T0][0] = 0;
                c.g[A3][0] = c.g[A2][0];
                c.g[T1][0] = 0;
                c.g[T2][0] = 0x3d;
                call_ptr(c, 0x0021_0f40, 0x0021_0bc0)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 5;
        c.g[V0][0] = 0x1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------- OVScreenReadPrivateMessage_SetMessageText

/// `void OVScreenReadPrivateMessage_SetMessageText(screen, name, a)`
/// (0x210bd8). Finds the screen's text widget (`screen+0x40`, UI hash
/// `0x4717f8`), and when it exists writes the message text into its
/// scroller: a string built from the name (`0x534fb8` / `0x534fc8` prefix,
/// the `0x4a2710` separator, and the name in `s2`), added with
/// `cUITextScroll_addAsciiString`. With `a == 0` the string uses the
/// shorter prefix pair. Finishes with `func_001A83D8` (the widget's
/// layout) on the widget's fields at +0xc4..+0xc8.
pub fn ov_screen_read_private_message_set_message_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = 0x470000;
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x17f8));
        call_ptr(c, 0x0031_7670, 0x0021_0c10)?;
        c.g[A0][0] = Addr(text_set_s0(c).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0021_0c1c)?;
        c.g[S0][0] = c.g[V0][0];
        let no_widget = c.g[S0][0] == 0;
        c.g[V0][0] = Addr(text_set_gp(c, 0x848)).lw(c);
        'done: {
            if no_widget {
                break 'done;
            }
            'text_block: {
                'with_pair: {
                    c.cycles += 8;
                    c.g[A1][0] = 0x7;
                    c.g[S5][0] = Addr(text_set_s0(c).wrapping_add(0xc8)).lhu(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x78)).lw(c);
                    c.g[S3][0] = Addr(text_set_s0(c).wrapping_add(0xc4)).lhu(c);
                    c.g[S4][0] = Addr(text_set_s0(c).wrapping_add(0xc6)).lhu(c);
                    call_ptr(c, 0x0014_dd58, 0x0021_0c40)?;
                    let no_flag = c.g[S1][0] == 0;
                    c.g[A1][0] = 0x530000;
                    if no_flag {
                        break 'with_pair;
                    }
                    c.cycles += 19;
                    c.g[A1][0] = 0x530000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4fb8));
                    call_ptr(c, 0x0031_82d8, 0x0021_0c58)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2710));
                    call_ptr(c, 0x0031_8910, 0x0021_0c68)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x0031_8910, 0x0021_0c74)?;
                    c.g[A1][0] = Addr(c.g[SP][0] as u32).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a_19f8, 0x0021_0c80)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0031_8260, 0x0021_0c8c)?;
                    c.g[A0][0] = c.g[S0][0];
                    break 'text_block;
                }
                // Without the flag: the short prefix.
                c.cycles += 17;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4fc8));
                call_ptr(c, 0x0031_82d8, 0x0021_0ca0)?;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2710));
                call_ptr(c, 0x0031_8910, 0x0021_0cb0)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0031_8910, 0x0021_0cbc)?;
                c.g[A1][0] = Addr(c.g[SP][0] as u32).lw(c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003a_19f8, 0x0021_0cc8)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0031_8260, 0x0021_0cd4)?;
                c.g[A0][0] = c.g[S0][0];
            }
            // Layout of the widget from its fields.
            c.cycles += 8;
            c.g[A3][0] = c.g[S5][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[T1][0] = c.g[S4][0];
            c.g[A1][0] = 0x1e;
            c.g[A2][0] = 0x8;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0x1;
            call_ptr(c, 0x001a_83d8, 0x0021_0cf8)?;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The widget event's screen (`s0` before the lookup) as an address.
pub(crate) fn text_set_s0(c: &Cpu) -> u32 {
    c.g[S0][0] as u32
}

/// `GP - off`.
pub(crate) fn text_set_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

// ---------------------------------------------------------------- OVScreenReadPrivateMessage_OnWidgetAction

/// `void OVScreenReadPrivateMessage_OnWidgetAction(screen, ev, kind)`
/// (0x210df0), the read-message screen's widget handler. Does nothing
/// without the screen's widget (`screen+0x40`). Kind 6 clears the
/// widget's status bits (`+0x1c`, to `0x780`) after the item's method
/// call; kind 5 opens the reply keyboard (or, with a reply already in
/// `+0xe0`, leaves); kinds 0..6 other than those hand the event to
/// `cOVState_onWidgetEvent`; kind 13 sends the queued reply (`strlen` of
/// the reply text at `+0xe0 + 0x74`, `cGameComm_sendChat`) and shows it.
pub fn ov_screen_read_private_message_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        'out: {
            'body: {
                c.g[V0][0] = Addr(read_msg_s1(c).wrapping_add(0x40)).lw(c);
                let no_widget = c.g[V0][0] == 0;
                c.g[V0][0] = 0x6;
                if no_widget {
                    break 'body;
                }
                'widget: {
                    'pick: {
                        'state: {
                            'kind_six: {
                                c.cycles += 2;
                                let is_six = c.g[A2][0] == c.g[V0][0];
                                c.g[V0][0] = (c.g[A2][0] < 0x7) as u64;
                                if is_six {
                                    break 'kind_six;
                                }
                                'kind_five: {
                                    c.cycles += 2;
                                    let below = c.g[V0][0] == 0;
                                    c.g[V0][0] = 0x5;
                                    if below {
                                        break 'kind_five;
                                    }
                                    c.cycles += 2;
                                    if c.g[A2][0] == c.g[V0][0] {
                                        c.g[V0][0] = Addr(read_msg_s1(c).wrapping_add(0xe0)).lw(c);
                                        break 'state;
                                    }
                                    c.cycles += 2;
                                    break 'widget;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0xd;
                                let is_thirteen = c.g[A2][0] == c.g[V0][0];
                                c.g[S0][0] = sx(read_msg_s1(c).wrapping_add(0x9c));
                                if is_thirteen {
                                    break 'pick;
                                }
                                c.cycles += 2;
                                break 'widget;
                            }

                            // Kind 6: clear the widget's status bits.
                            c.cycles += 3;
                            c.g[V0][0] = Addr(read_msg_s1(c).wrapping_add(0xe0)).lw(c);
                            let has_reply = c.g[V0][0] != 0;
                            frame.restore(c, S0, 0x20);
                            if has_reply {
                                break 'out;
                            }
                            c.cycles += 10;
                            c.g[A0][0] = Addr(read_msg_s1(c).wrapping_add(0x10)).lw(c);
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039_f190, 0x0021_0e60)?;
                            c.g[V1][0] = Addr(read_msg_s1(c).wrapping_add(0x1c)).lw(c);
                            c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
                            c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                            c.g[V1][0] = c.g[V1][0] | 0x780;
                            let status = c.g[V1][0];
                            Addr(read_msg_s1(c).wrapping_add(0x1c)).sw(c, status);
                            break 'body;
                        }

                        // Kind 5: open the reply keyboard unless a reply is queued.
                        c.cycles += 2;
                        let has_reply = c.g[V0][0] != 0;
                        frame.restore(c, S0, 0x20);
                        if has_reply {
                            break 'out;
                        }
                        c.cycles += 16;
                        c.g[A2][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2560));
                        c.g[A1][0] = 0;
                        c.g[A3][0] = c.g[A2][0];
                        c.g[T0][0] = 0;
                        c.g[T1][0] = 0;
                        c.g[T2][0] = 0x3d;
                        call_ptr(c, 0x0021_0f40, 0x0021_0ea4)?;
                        c.g[A0][0] = Addr(read_msg_s1(c).wrapping_add(0xe0)).lw(c);
                        c.g[A1][0] = 0x4b;
                        c.g[A2][0] = 0x1;
                        c.g[A3][0] = 0x2;
                        call_ptr(c, 0x001c_e3c8, 0x0021_0eb8)?;
                        frame.restore(c, S0, 0x20);
                        break 'out;
                    }

                    // Kind 13: the queued chat line, if any, is shown as the reply.
                    c.cycles += 6;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0x41;
                    call_ptr(c, 0x0025_8160, 0x0021_0ed0)?;
                    let no_line = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S0][0];
                    if !no_line {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = 0;
                        call_ptr(c, 0x0021_0bd8, 0x0021_0ee4)?;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = Addr(read_msg_s1(c).wrapping_add(0xe0)).lw(c);
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x74));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0041_6810, 0x0021_0ef4)?;
                    if c.g[V0][0] == 0 {
                        frame.restore(c, S0, 0x20);
                        break 'out;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0025_8c50, 0x0021_0f08)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0021_0bd8, 0x0021_0f18)?;
                    frame.restore(c, S0, 0x20);
                    break 'out;
                }

                // Every other kind: the generic overlay widget handler.
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0020_a6a8, 0x0021_0f28)?;
            }
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

/// The read-message screen (`s1`) as an address.
pub(crate) fn read_msg_s1(c: &Cpu) -> u32 {
    c.g[S1][0] as u32
}

// ---------------------------------------------------------------- OVScreenProfileSavePDA_ctor

/// `cOVState_PROFILE::cOVState_PROFILE(this, mgr, type, a3)` (0x211088),
/// the profile save screen's constructor. Runs the base memory-card
/// constructor, stores the screen's vtable and sizes, then, for a nonzero
/// `type`, scans the bits of `type >> 1` for the slot count (the first
/// set bit above bit 0, at most nine). Looks up the BE interface and
/// counts the profile slots of the game's save list, storing the slot
/// index at `+0x1cc` when it finds the matching one. Returns `this`.
pub fn ov_screen_profile_save_pda_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A2][0] & 0xff;
        c.g[S2][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A3][0];
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S4][0] = 0;
        call_ptr(c, 0x001d_53b0, 0x0021_10c0)?;
        c.g[S1][0] = sx((c.g[S1][0] as u32) >> 1);
        c.g[V1][0] = 0x24;
        c.g[V0][0] = 0x470000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2928));
        let size = c.g[V1][0];
        Addr((c.g[S2][0] as u32).wrapping_add(0xc)).sw(c, size);
        let vtable = c.g[V0][0];
        Addr((c.g[S2][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        c.g[V1][0] = 0x1;
        let mgr = c.g[S0][0];
        Addr((c.g[S2][0] as u32).wrapping_add(0x278)).sw(c, mgr);
        c.g[S5][0] = 0x530000;
        Addr((c.g[S2][0] as u32).wrapping_add(0x1cc)).sw(c, 0);
        Addr((c.g[S2][0] as u32).wrapping_add(0x22c)).sw(c, 0);
        let no_type = c.g[S1][0] == 0;
        Addr((c.g[S2][0] as u32).wrapping_add(0x230)).sw(c, 0);
        'scan_bits: {
            if no_type {
                break 'scan_bits;
            }
            c.cycles += 1;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            'bit_loop: loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 9i64) as u64;
                let in_range = c.g[V0][0] != 0;
                c.g[V0][0] = sx(((c.g[S1][0] as u32) as i32 >> ((c.g[V1][0] as u32) & 31)) as u32);
                if !in_range {
                    break 'scan_bits;
                }
                c.cycles += 3;
                c.g[S1][0] = c.g[V0][0] & 0xff;
                if c.g[S1][0] != 0 {
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0021_10f8);
                    }
                    continue 'bit_loop;
                }
                c.cycles += 1;
                c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                break 'scan_bits;
            }
        }

        // The BE interface and the save list's slot count.
        c.cycles += 17;
        c.g[S1][0] = 0x1;
        call_ptr(c, 0x0014_dc80, 0x0021_111c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014_dd58, 0x0021_1128)?;
        c.g[S0][0] = 0;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0021_1134)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0021_1140)?;
        c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5bc8));
        c.g[A0][0] = 0x2;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x49)).lb(c);
        c.g[V0][0] ^= 0x2;
        if c.g[V0][0] == 0 {
            c.g[S1][0] = c.g[A0][0];
        }
        'scan: loop {
            'slot_done: {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S3][0];
                if !more {
                    break 'slot_done;
                }
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0014_7318, 0x0021_116c)?;
                if c.g[V0][0] != c.g[S4][0] {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0021_1158);
                    }
                    continue 'scan;
                }
                c.cycles += 1;
                let slot = c.g[S0][0];
                Addr((c.g[S2][0] as u32).wrapping_add(0x1cc)).sw(c, slot);
            }
            break 'scan;
        }
        c.cycles += 10;
        c.g[V0][0] = c.g[S2][0];
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

/// `OVScreenReadPrivateMessage_ctor` (0x221408): constructs the read-private-
/// message overlay state in `a0` with overlay id 0 and the message vtable.
pub fn ov_screen_read_private_message_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, 16, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OV_STATE_UI_CTOR, 0x0022_1420)?;
        ov_state_fill_fields(c, 0, 0x0047_3248, false);
        f.restore(c, 16, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `OVScreenReadPrivateMessage_ctor_2` (0x221468): as the constructor above,
/// with the overlay id taken from `a2`.
pub fn ov_screen_read_private_message_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let f = Frame::open(c, 0x30);
        f.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        f.save(c, 17, 0x10);
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OV_STATE_UI_CTOR, 0x0022_1484)?;
        let ov_id = c.g[S1][0];
        ov_state_fill_fields(c, ov_id, 0x0047_3248, false);
        f.restore(c, 17, 0x10);
        f.restore(c, 16, 0x20);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `OVScreenReadPrivateMessage_dtor` (0x2214d0): the same destructor as the
/// session overlay's (base vtable, then `UIState_dtor`).
pub fn ov_screen_read_private_message_dtor(c: &mut Cpu) {
    guest(c, |c| ov_state_dtor_body(c, 0x0022_14ec));
}

/// Data page `0x470000`: the overlay vtables sit at `+0x4e08` (`cOVState`),
/// `+0x2108` (`cOVState_REWARDS`), `+0x1ea8` (`cOVStateTrophy`) and `+0x1f80`
/// (`cOVState_AUTOSAVE`).
pub(crate) const OC1015_DATA: u32 = 0x0047_0000;

pub(crate) const OC1015_VT_BASE: u32 = OC1015_DATA + 0x4e08;

pub(crate) const OC1015_VT_TROPHY: u32 = OC1015_DATA + 0x1ea8;

/// The game-image data page (`0x4a0000`) holding `str_sp_b[0]` at +0x2088.
pub(crate) const OC1015_DATA_PAGE: u32 = 0x004a_0000;

/// `UIState_ctor` (0x39e2a0) and `UIState_dtor` (0x39e390): the base overlay
/// state constructor and destructor.
pub(crate) const OC1015_UI_CTOR: u32 = 0x0039_e2a0;

pub(crate) const OC1015_UI_DTOR: u32 = 0x0039_e390;

/// The destructor shape of the overlay states (frame 0x10, holding `ra`):
/// `this` (a0, copied to v1) gets the vtable `vptr` at +0x8, then `callee`
/// runs on `this` (a0 still `this`) and returns to `ret`.
pub(crate) fn oc1015_dtor(c: &mut Cpu, vptr: u32, callee: u32, ret: u32) -> Flow {
    c.cycles += 10;
    let f = Frame::open(c, 0x10);
    f.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = vptr as u64;
    let this = c.g[V1][0] as u32;
    let value = c.g[V0][0];
    Addr(this.wrapping_add(0x8)).sw(c, value);
    call_ptr(c, callee, ret)?;
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// The two-register constructor shape of the reward list and the trophy
/// screen (frame 0x30, `s0` = this, `s1` = the state word in `a2`): the base
/// `UIState` constructor, which returns to `call_ret`, then the state word at
/// +0x98, the base vtable, the flags at +0x48/+0x4c/+0x54 cleared, the final
/// vtable `vptr` at +0x8 and the chat halfword at +0x58.
pub(crate) fn oc1015_ctor2(c: &mut Cpu, call_ret: u32, vptr: u32) -> Flow {
    c.cycles += 26;
    let f = Frame::open(c, 0x30);
    f.save(c, S0, 0x20);
    c.g[S0][0] = c.g[A0][0];
    f.save(c, S1, 0x10);
    f.save_ra(c, 0);
    c.g[S1][0] = c.g[A2][0];
    call_ptr(c, OC1015_UI_CTOR, call_ret)?;
    let this = c.g[S0][0] as u32;
    c.g[V1][0] = OC1015_DATA as u64;
    let state = c.g[S1][0];
    Addr(this.wrapping_add(0x98)).sw(c, state);
    Addr(this.wrapping_add(0x48)).sw(c, 0);
    c.g[V1][0] = OC1015_VT_BASE as u64;
    let base_vt = c.g[V1][0];
    Addr(this.wrapping_add(0x8)).sw(c, base_vt);
    c.g[A1][0] = OC1015_DATA_PAGE as u64;
    Addr(this.wrapping_add(0x4c)).sw(c, 0);
    c.g[V1][0] = OC1015_DATA as u64;
    Addr(this.wrapping_add(0x54)).sw(c, 0);
    c.g[V1][0] = vptr as u64;
    c.g[V0][0] = c.g[S0][0];
    f.restore(c, S1, 0x10);
    c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
    let final_vt = c.g[V1][0];
    Addr(this.wrapping_add(0x8)).sw(c, final_vt);
    let chat = c.g[A0][0];
    Addr(this.wrapping_add(0x58)).sh(c, chat);
    f.restore(c, S0, 0x20);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `OVScreenTrophy_ctor` (0x2222f8): constructs the trophy overlay state in
/// `a0` with overlay id 0: the base `UIState` constructor, then the flags,
/// the base vtable, the trophy vtable and the chat halfword.
pub fn ov_screen_trophy_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, S0, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OC1015_UI_CTOR, 0x0022_2310)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = OC1015_DATA as u64;
        Addr(this.wrapping_add(0x98)).sw(c, 0);
        c.g[V1][0] = OC1015_VT_BASE as u64;
        Addr(this.wrapping_add(0x48)).sw(c, 0);
        let base_vt = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, base_vt);
        c.g[A1][0] = OC1015_DATA_PAGE as u64;
        Addr(this.wrapping_add(0x4c)).sw(c, 0);
        c.g[V1][0] = OC1015_DATA as u64;
        Addr(this.wrapping_add(0x54)).sw(c, 0);
        c.g[V1][0] = OC1015_VT_TROPHY as u64;
        c.g[V0][0] = c.g[S0][0];
        f.restore_ra(c, 0);
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
        let final_vt = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, final_vt);
        let chat = c.g[A0][0];
        Addr(this.wrapping_add(0x58)).sh(c, chat);
        f.restore(c, S0, 0x10);
        f.close(c);
        Ok(())
    });
}

/// `OVScreenTrophy_ctor_2` (0x222358): the trophy overlay constructor with the
/// state word from `a2`, the same shape as the reward list's.
pub fn ov_screen_trophy_ctor_2(c: &mut Cpu) {
    guest(c, |c| oc1015_ctor2(c, 0x0022_2374, OC1015_VT_TROPHY));
}

/// `OVScreenTrophy_dtor` (0x2223c0): the trophy overlay's destructor, the same
/// as the base overlay state's.
pub fn ov_screen_trophy_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc1015_dtor(c, OC1015_VT_BASE, OC1015_UI_DTOR, 0x0022_23dc)
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;

    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base_4;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;

    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::hash_object;

    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2 as OBJ;

    pub(crate) use crate::game::fe::fereal::tests::base as base_2;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_2;
    use ssx3_ee as ee;

    #[test]
    fn c_keyboard_popup_set_max_length_matches() {
        check(
            ssx3_ee::s04::f_001cd088,
            c_keyboard_popup_set_max_length,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c10_0005 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => (r.next_u32() % 64) as u64,
                    1 => 64 + (r.next_u32() % 64) as u64,
                    2 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    _ => 0xffff_ffff_ffff_ffff,
                };
                c.bus.write32(DATA + 0x60, r.next_u32());
            },
        );
    }

    #[test]
    fn c_keyboard_popup_set_key_enabled_matches() {
        check(
            ssx3_ee::s04::f_001ce3c8,
            c_keyboard_popup_set_key_enabled,
            400,
            &|c, seed| {
                let mut r = Rng(0x0c10_0007 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.g[A3][0] = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 0x1_0000_0000,
                    _ => 0xffff_ffff_ffff_ffff,
                };
            },
        );
    }

    #[test]
    fn race_progress_reset_rider_matches() {
        check_lohi(
            ssx3_ee::s05::f_002105b0,
            race_progress_reset_rider,
            100,
            &|c, seed| {
                let mut r = Rng(0x0c10_0012 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn race_progress_clear_path_data_matches() {
        check(
            ssx3_ee::s05::f_00210608,
            race_progress_clear_path_data,
            50,
            &|c, seed| {
                let mut r = Rng(0x0c10_0013 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x9a0, r.next_u32());
                c.bus.write32(GAME_GP - 0x99c, r.next_u32());
                c.bus.write32(GAME_GP - 0x998, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------------------
    // Race progress / game state.

    #[test]
    fn race_progress_set_path_data_matches() {
        check(
            ssx3_ee::s05::f_002105d0__generated,
            race_progress_set_path_data,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x105d0, 0);
                let p = DATA + 0x100;
                c.bus.write32(p + 4, r.next_u32() % 0x800);
                c.bus.write32(p + 0xc, r.next_u32() % 0x800);
                c.g[A0][0] = sx(p);
            },
        );
    }

    #[test]
    fn hud_meter_dtor_matches() {
        check(
            ee::s05::f_0020eb08__generated,
            hud_meter_dtor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x20eb_08, 1);
                let flags = seed % 4;
                // The free branch runs on a null object: freeing test data would
                // hand a bogus pointer to the heap.
                let this = if flags & 1 != 0 { 0 } else { DATA + 0x100 };
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(flags);
                c.bus.write32(GAME_GP - 0x984, 1 + r.next_u32() % 2);
                c.bus.write32(GAME_GP - 0x988, 0x1234_5678);
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_on_child_event_matches() {
        check(
            ssx3_ee::s05::f_00210dd0__generated,
            ov_screen_read_private_message_on_child_event,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x210dd0, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 {
                    0x77
                } else {
                    r.next_u32() as u64
                };
                c.g[A2][0] = if seed % 3 == 0 {
                    0x16
                } else {
                    (seed % 0x20) as u64
                };
                word(
                    c,
                    DATA + 0xe0,
                    if seed % 2 == 0 { 0x77 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn keyboard_popup_set_function_keys_enabled_matches() {
        check(
            ee::s04::f_001ce468__generated,
            c_keyboard_popup_set_function_keys_enabled,
            48,
            &|c, seed| {
                let _ = base(c, seed, 0x1ce4_68);
                let s = DATA + 0x100;
                clear(c, s, 0x400);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn ov_screen_trophy_init_matches() {
        check(
            ssx3_ee::s05::f_0020e938__generated,
            ov_screen_trophy_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x20e938);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn hud_meter_ctor_matches() {
        check(
            ee::s05::f_0020eaa0__generated,
            hud_meter_ctor,
            40,
            &|c, seed| {
                base_2(c, seed, 0x20_eaa0);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    c.bus.write32(GAME_GP.wrapping_sub(0x988), 0);
                } else {
                    c.bus.write32(GAME_GP.wrapping_sub(0x988), 1);
                    c.bus.write32(GAME_GP.wrapping_sub(0x98c), DATA + 0x200);
                    c.bus.write32(DATA + 0x200 + 0x14, 320.0f32.to_bits());
                    c.bus.write32(DATA + 0x200 + 0x34, 240.0f32.to_bits());
                }
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x54f4), 1.0f32.to_bits());
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x54f0), 1.0f32.to_bits());
            },
        );
    }

    #[test]
    fn c_keyboard_popup_set_static_text_matches() {
        check(
            ee::s04::f_001ccf00__generated,
            c_keyboard_popup_set_static_text,
            8,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x001c_cf00, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (DATA + 0x4000) as u64;
                c.bus.write8(DATA + 0x4000, b'a');
                c.bus.write8(DATA + 0x4001, 0);
                let screen = DATA + 0x3800;
                match seed % 3 {
                    0 => w_2(c, OBJ + 0x40, 0),
                    1 => {
                        w_2(c, OBJ + 0x40, screen);
                        w_2(c, screen + 0x3c, 0);
                    }
                    _ => {
                        w_2(c, OBJ + 0x40, screen);
                        let node = DATA + 0x3c00;
                        hash_object(c, screen, node);
                        w_2(c, node + 0xb0, 0);
                        let _ = r.next_u32();
                    }
                }
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_update_matches() {
        check(
            ee::s05::f_00210d20__generated,
            ov_screen_read_private_message_update,
            60,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x210d);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x40, (seed % 2) as u32);
                c.bus
                    .write32(gp_at(-0x204), if seed % 3 == 0 { 0 } else { DATA + 0x1000 });
                c.bus.write32(DATA + 0x1000 + 0x68, (seed / 2 % 2) as u32);
                c.bus.write32(DATA + 0x1000 + 0x64, (seed / 4 % 2) as u32);
                c.bus.write32(DATA + 0x10, DATA + 0x2000);
                fill(c, &mut r, DATA + 0x9c, 0x50);
                fill(c, &mut r, DATA + 0x2000, 0x40);
            },
        );
    }

    #[test]
    fn cov_state_hud1p_load_meter_assets_matches() {
        check(
            ee::s05::f_0020eb50__generated,
            cov_state_hud1p_load_meter_assets,
            20,
            &|c, seed| {
                let _r = base_5(c, seed, 0x20eb);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                c.bus.write32(gp_at(-0x988), 0);
                c.bus.write32(gp_at(-0x98c), 0);
                c.bus.write32(DATA + 0x3000 + 8, seed as u32);
            },
        );
    }

    #[test]
    fn c_keyboard_popup_set_text_matches() {
        diff(
            ee::s04::f_001cb418__generated,
            c_keyboard_popup_set_text,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1c_b418);
                let hb = DATA + 0x1000;
                w_3(c, DATA + 0x74, hb + 0xc);
                w_3(c, hb, 1);
                w_3(c, hb + 4, 2);
                w_3(c, hb + 8, 0x40);
                c.bus.write8(hb + 0xc, b'h');
                c.bus.write8(hb + 0xd, b'i');
                c.bus.write8(hb + 0xe, 0);
                w_3(c, DATA + 0x58, if seed % 2 == 0 { 0 } else { r.next_u32() });
                w_3(c, DATA + 0x64, r.next_u32());
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    pub(crate) use crate::game::ai::rider::tests::meter;

    pub(crate) fn draw_progress_meter_setup(c: &mut Cpu, seed: u32) {
        use meter::*;
        crate::difftest::elf_image(c);
        let mut r = machine(c, seed, 0x0020_eda0, 24);
        // Early outs: HUD hidden, meter off, no player, no meter definition.
        let early = seed % 10;
        c.bus.write32(gp_at(-0x990), (early == 0) as u32);
        c.bus.write32(gp_at(-0x994), (early != 1) as u32);
        c.bus.write32(gp_at(-0x998), MARKERS);
        c.bus
            .write32(gp_at(-0x9a0), if early == 2 { 0 } else { METER });
        c.bus.write32(gp_at(-0x98c), FONT);
        c.bus.write32(gp_at(-0x848), WORLD);
        c.bus.write32(gp_at(-0x854), RENDERER);
        for off in [0x1dc4, 0x1dc8, 0x1dd8] {
            c.bus.write32(gp_at(off), r.next_u32() & 1);
        }
        c.g[A0][0] = HUD as u64;
        c.g[A1][0] = if early == 3 { 0 } else { PLAYER as u64 };
        fill(c, &mut r, HUD, 0x10);
        for i in 0..8 {
            c.bus.write32(HUD + 4 * i, r.f32(400.0));
        }
        c.bus.write32(HUD + 0x34, r.next_u32() % 4);
        // Rider vtable: slot 0x38 (the player's racer slot, a fake).
        let vt = vtable(c, &mut r, &[0x38, 0x40]);
        c.bus.write32(PLAYER + 0x6c0, vt);
        // Slot 0x40 (asked of every other racer) is a real 0/1 function.
        c.bus
            .write32(vt + 0x44, if seed % 3 == 0 { RET1 } else { RET0 });
        let slot = r.next_u32() % 6;
        c.bus.write32(LOG_RESULTS, slot);
        // World -> race manager -> race; mode flag (+0x88).
        c.bus.write32(WORLD + 0x84, RACE_MAN);
        c.bus.write32(WORLD + 0xc0, MODE);
        c.bus.write32(MODE + 0x88, r.next_u32() & 1);
        c.bus.write32(RACE_MAN + 0xc, RACE);
        c.bus.write32(RACE + 0x78, r.next_u32() % 8);
        for i in 0..8 {
            let racer = RACERS + 0x800 * i;
            c.bus.write32(RACE + 0x28 + 4 * i, racer);
            c.bus.write32(racer + 0x6c0, vt);
        }
        // Racer progress: distances around the player's so some fall
        // inside the +-5000 window and some outside.
        for i in 0..6 {
            let e = PROGRESS + 0x18 * i;
            for w in 0..5 {
                c.bus.write32(e + 4 * w, r.f32(9000.0));
            }
            c.bus.write32(e + 0x14, r.next_u32() & 1);
        }
        // Style table entries {f32, f32, f32, ptr}; the pointed-to records
        // hold a colour word and a few floats.
        for i in 0..10 {
            let e = STYLE_TABLE + 0x10 * i;
            for w in 0..3 {
                c.bus.write32(e + 4 * w, r.f32(300.0));
            }
            let rec = STYLES + 0x40 * i;
            c.bus.write32(e + 0xc, rec);
            c.bus.write32(rec, r.next_u32());
            for w in 1..8 {
                c.bus.write32(rec + 4 * w, r.f32(300.0));
            }
        }
        for w in 0..8 {
            c.bus.write32(VECS + 4 * w, r.f32(2.0));
        }
        c.bus.write32(FLAGS, r.next_u32());
        // Meter definition: +0x8 marker count, +0x10 course length.
        fill(c, &mut r, METER, 8);
        c.bus.write32(METER + 0x8, r.next_u32() % 4);
        c.bus.write32(
            METER + 0x10,
            (5000.0 + (r.next_u32() % 20000) as f32).to_bits(),
        );
        for i in 0..4 {
            c.bus.write32(MARKERS + 8 * i, r.next_u32() % 3);
            c.bus.write32(MARKERS + 8 * i + 4, r.f32(9000.0));
        }
        // Renderer: vtable slots begin/state/draw/end, GS packet buffer.
        let rvt = vtable(c, &mut r, &[0xf8, 0x100, 0x110, 0x238]);
        c.bus.write32(RENDERER + 0x10d8, rvt);
        c.bus.write32(RENDERER + 0xe84, PACKET);
        fill(c, &mut r, PACKET, 0x10);
        c.pc = 0x0020_eda0;
    }

    /// Every early out, and the full meter with 0..7 racers, markers in and
    /// out of the window, and unwinds at the renderer calls.
    #[test]
    fn c_ov_state_hud1p_draw_progress_meter_matches() {
        covered(
            ee::s05::f_0020eda0__generated,
            c_ov_state_hud1p_draw_progress_meter,
            200,
            &draw_progress_meter_setup,
        );
    }

    // ---- cAngerFX_Render (0x2d5048) ----

    /// Guest functions that just return 0 / 1 (`GameProcess_v09`,
    /// `WorldObj_v16`), for vtable slots whose result picks a branch.
    pub(crate) const RET0: u32 = 0x0036_0758;

    pub(crate) const RET1: u32 = 0x0036_0910;

    // ---------------------------------------- cOVStateHUD1P_UpdateRaceProgress

    /// `cOVStateHUD1P_UpdateRaceProgress`: no course; a course with zero to
    /// four riders, each updated by the real `RaceProgress_UpdateRider` on
    /// its fixture (world, course, segments, riders 700 from the finish;
    /// its projection is a fake, which may unwind).
    #[test]
    fn hud1p_update_race_progress_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, GAME_GP};
        use crate::game::fe::ovstatetrophy::tests::put_course;
        use crate::game::fe::ovstatetrophy::tests::put_rider;
        use crate::game::fe::ovstatetrophy::tests::put_world;
        use crate::game::fe::ovstatetrophy::tests::COURSE;
        use crate::game::fe::ovstatetrophy::tests::COURSE_SEGS;
        use crate::game::fe::ovstatetrophy::tests::RACE;
        use crate::game::fe::ovstatetrophy::tests::RIDER;
        check(
            ee::s05::f_0020ed20__generated,
            hud1p_update_race_progress,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0020_ed20, 4);
                c.pc = 0x0020_ed20;
                put_world(c);
                put_course(c);
                c.bus.write32(GAME_GP - 0x99c, COURSE_SEGS);
                c.bus.write32(GAME_GP - 0x54ac, 138.8889f32.to_bits());
                c.bus.write32(
                    COURSE + 0x10,
                    (1100.0f32 + f32::from_bits(r.f32(400.0))).to_bits(),
                );
                let riders = seed % 5;
                c.bus.write32(RACE + RACE_RIDER_COUNT, riders);
                for i in 0..riders {
                    put_rider(c, i);
                    c.bus.write32(RIDER + 0x100 * i + 0x4d0, 700.0f32.to_bits());
                    let slot = PROGRESS_SLOTS + PROGRESS_SLOT_SIZE * i;
                    c.bus.write32(slot, r.next_u32() % 2);
                    c.bus.write32(slot + 4, 0);
                    c.bus.write32(slot + 8, r.f32(1000.0));
                }
                c.bus.write32(GAME_GP - GP_PROGRESS_DIRTY, r.next_u32());
                if seed % 7 == 0 {
                    c.bus.write32(GAME_GP - GP_RACE_COURSE, 0);
                }
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::FAKE;
    pub(crate) use crate::game::be::beintstat::tests::allocator;
    pub(crate) use crate::game::fe::debugfe::tests::pool_allocator;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ as OBJ_m2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_m2;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_2_m2;

    // RaceProgress_ProjectRider ------------------------------------------

    pub(crate) const WORLD: u32 = DATA + 0x1000;

    pub(crate) const RACERS: u32 = DATA + 0x1100;

    pub(crate) const RACE: u32 = DATA + 0x1200;

    pub(crate) const RIDER: u32 = DATA + 0x2000;

    pub(crate) const ZONES: u32 = DATA + 0x3000;

    pub(crate) const POS_VT: u32 = DATA + 0x4000;

    /// The world → race manager chain and the zone table pointer.
    pub(crate) fn put_world(c: &mut Cpu) {
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, RACERS);
        c.bus.write32(RACERS + 0xc, RACE);
        c.bus.write32(RACE + 0x78, 4);
        c.bus.write32(GAME_GP - 0x99c, ZONES);
    }

    /// The rider `idx`: its position object's virtual method is a fake.
    pub(crate) fn put_rider(c: &mut Cpu, idx: u32) {
        let rider = RIDER + 0x100 * idx;
        c.bus.write32(RACE + 0x28 + 4 * idx, rider);
        c.bus.write32(rider + 0x6c0, POS_VT);
        c.bus.write16(POS_VT + 0x28, 0);
        c.bus.write32(POS_VT + 0x2c, FAKE);
    }

    #[test]
    fn race_progress_project_rider_matches() {
        covered(
            ssx3_ee::s05::f_00210820__generated,
            race_progress_project_rider,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x210820, 1);
                put_world(c);
                let idx = (seed % 2) as u32;
                put_rider(c, idx);
                c.g[A0][0] = idx as u64;
                // The slot and the zone it points at.
                let slot = 0x004c_8bc8 + 0x18 * idx;
                let zone = idx;
                c.bus.write32(slot, (seed % 3) as u32); // active
                c.bus.write32(slot + 4, zone);
                for i in 0..5 {
                    c.bus.write32(ZONES + 0x14 * zone + 4 * i, r.edge_f32());
                }
            },
        );
    }

    // RaceProgress_UpdateRider -------------------------------------------

    pub(crate) const COURSE: u32 = DATA + 0x5000;

    pub(crate) const COURSE_SEGS: u32 = DATA + 0x6000;

    pub(crate) fn put_course(c: &mut Cpu) {
        c.bus.write32(COURSE, 4);
        c.bus.write32(COURSE + 0x10, f(1000.0));
        c.bus.write32(GAME_GP - 0x9a0, COURSE);
        for i in 0..4u32 {
            c.bus.write32(COURSE_SEGS + 0x14 * i, 0);
            c.bus.write32(COURSE_SEGS + 0x14 * i + 4, 0);
            c.bus.write32(COURSE_SEGS + 0x14 * i + 8, 0);
            c.bus.write32(COURSE_SEGS + 0x14 * i + 0xc, 0);
            c.bus
                .write32(COURSE_SEGS + 0x14 * i + 0x10, f(100.0 * i as f32));
        }
    }

    #[test]
    fn race_progress_update_rider_matches() {
        covered(
            ssx3_ee::s05::f_00210618__generated,
            race_progress_update_rider,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x210618, 1);
                put_world(c);
                put_course(c);
                c.bus.write32(GAME_GP - 0x99c, COURSE_SEGS);
                c.bus.write32(GAME_GP - 0x54ac, f(138.8889));
                let idx = (seed % 2) as u32;
                put_rider(c, idx);
                c.g[A0][0] = idx as u64;
                c.bus.write32(RIDER + 0x100 * idx + 0x4d0, f(700.0));
                let slot = 0x004c_8bc8 + 0x18 * idx;
                // `d = course.dist - 700`, chosen per branch.
                let (active, seg, d, prev) = match seed % 8 {
                    0 => (0, 0, 100.0, 0.0),    // (course is nulled below)
                    1 => (0, 0, 100.0, 0.0),    // (rider count is zeroed below)
                    2 => (1, 0, 900.0, 0.0),    // active: no clamp, past the last
                    3 => (0, 0, 1000.0, 500.0), // clamp up
                    4 => (0, 0, 100.0, 5000.0), // clamp down
                    5 => (0, 0, 150.0, 100.0),  // no clamp, walk
                    6 => (0, 0, -10.0, 0.0),    // d <= 0
                    _ => (0, 0, 250.0, 0.0),    // walk
                };
                c.bus.write32(COURSE + 0x10, f(700.0 + d));
                c.bus.write32(slot, active);
                c.bus.write32(slot + 4, seg);
                c.bus.write32(slot + 8, f(prev));
                if seed % 8 == 0 {
                    c.bus.write32(GAME_GP - 0x9a0, 0);
                }
                if seed % 8 == 1 {
                    c.bus.write32(RACE + 0x78, 0);
                }
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn race_progress_reset_all_matches() {
        check(
            ee::s05::f_002108f8__generated,
            race_progress_reset_all,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x2108_f8, 0);
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_open_keyboard_matches() {
        check(
            ssx3_ee::s05::f_00210f40__generated,
            ov_screen_read_private_message_open_keyboard,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x210f40, 0);
                pool_allocator(c);
                w(c, OBJ_m2 + 0xe0, 0);
                c.g[A0][0] = OBJ_m2 as u64;
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = OBJ2 as u64;
                c.g[A3][0] = OBJ3 as u64;
                c.g[T0][0] = seed.wrapping_mul(3) as u64;
                c.g[T1][0] = seed.wrapping_mul(5) as u64;
                c.g[T2][0] = seed.wrapping_mul(7) as u64;
            },
        );
    }

    // -------------------------------------------------------- lobby pages

    #[test]
    fn ovscreen_read_private_message_create_keyboard_popup_matches() {
        strict(
            ee::s05::f_00210fa0__generated,
            ovscreen_read_private_message_create_keyboard_popup,
            40,
            &|c, seed| {
                let _ = base_m2(c, seed, 0x210f, 6);
                allocator(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = ((seed % 3) + 1) as u64;
                c.g[A3][0] = (seed * 3) as u64;
                c.g[T0][0] = (seed % 5) as u64;
                w_2_m2(c, DATA + 0x10, DATA + 0x900);
            },
        );
    }

    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc770_machine;

    /// A float word for `HUDMeter_SetRect`'s inputs: edge cases and ordinary values.
    pub(crate) fn oc770_float(r: &mut Rng) -> u32 {
        if r.next_u32() % 3 == 0 {
            r.edge_f32()
        } else {
            r.f32(300.0)
        }
    }

    /// `HUDMeter_SetRect`: with and without an active meter (`gp-0x988`).
    #[test]
    fn hud_meter_set_rect_matches() {
        const OUT: u32 = DATA + 0x100;
        const AREA: u32 = DATA + 0x200;
        const METER: u32 = DATA + 0x300;
        check(
            ee::s05::f_0020ec18__generated,
            hud_meter_set_rect_2,
            300,
            &|c, seed| {
                let mut r = oc770_machine(c, seed, 0x7701, 0);
                let gp = c.g[GP][0] as u32;
                c.pc = 0x0020_ec18;
                c.g[A0][0] = OUT as u64;
                c.g[A2][0] = AREA as u64;
                let meter = if seed % 3 == 0 { 0 } else { METER };
                c.bus.write32(gp - 0x988, meter);
                c.bus.write32(gp - 0x98c, METER);
                c.bus.write32(METER + 0x14, r.next_u32() % 5000);
                let w = oc770_float(&mut r);
                c.bus.write32(METER + 0x34, w);
                for off in [0x54f4u32, 0x54f0] {
                    let w = oc770_float(&mut r);
                    c.bus.write32(gp - off, w);
                }
                let w = oc770_float(&mut r);
                c.bus.write32(0x004d_0000 - 0x7360, w);
                for i in 0..4 {
                    let w = oc770_float(&mut r);
                    c.bus.write32(AREA + 4 * i, w);
                }
                c.bus.write32(OUT + 0x2c, r.next_u32());
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes::check_reaching;
    use crate::difftest::fakes::machine_rt;

    pub(crate) use crate::game::fe::ovstatemap::tests::oc988_gate;

    pub(crate) use crate::game::fe::ovstatemap::tests::oc988_names;

    pub(crate) use crate::game::fe::ovstatemap::tests::oc988_screen;

    pub(crate) use crate::game::fe::ovstatemap::tests::oc988_w;

    pub(crate) use crate::game::fe::ovstateprofile::tests::oc988_mailbox;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::ov_object_setup;

    pub(crate) fn oc981_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Fill a row of random floats at `a` (`n` words, range +-`range`).
    pub(crate) fn oc981_floats(
        c: &mut Cpu,
        r: &mut crate::difftest::Rng,
        a: u32,
        n: u32,
        range: f32,
    ) {
        for k in 0..n {
            let v = r.f32(range);
            c.bus.write32(a + 4 * k, v);
        }
    }

    /// `cOVStateHUD1P::DrawProgressMeterMP`: early outs (HUD off, no radar
    /// header, no rider, no course), own rider index 0 (the real leaf at
    /// 0x1406c0) or 1..5 (forced fake result), racers of both kinds (human
    /// or AI through the vtable at +0x44: fake or the leaf), 0..6 racers
    /// near and far from the meter range, 0..6 course markers of types 0..3
    /// (type 1 hidden in timed events), both sides, the fade at either end
    /// and the clipped icons. The renderer's vtable slots are fakes, the draw
    /// state stack and the rate-limit table are real memory.
    #[test]
    fn c_ov_state_hud1_p_draw_progress_meter_mp_matches() {
        check_reaching(
            ee::s05::f_0020fb40__generated,
            c_ov_state_hud1_p_draw_progress_meter_mp,
            300,
            &[
                0x0020_fbe0,
                0x0020_fd6c,
                0x0020_fe74,
                0x0020_ff7c,
                0x0021_0014,
                0x0021_0230,
                0x0021_0518,
                0x0021_0550,
            ],
            &|c, seed| {
                let mut r = machine(c, seed, 0x20fb40, 24);
                elf_image(c);
                c.pc = 0x0020_fb40;
                let gp = GAME_GP;
                let this = DATA + 0x9000;
                let rider = DATA + 0x1000;
                let world = DATA + 0x100;
                let race = DATA + 0x200;
                let list = DATA + 0x300;
                let my_vt = DATA + 0x8800;
                let course = DATA + 0xb800;
                let markers = DATA + 0xb000;
                let renderer = DATA + 0xc000;
                let rvt = DATA + 0xd000;
                let table = 0x004c_8bc8u32;
                let icons = 0x004c_8c58u32;
                // Early-out flags.
                oc981_w(c, gp - 0x990, (seed % 11 == 0) as u32);
                oc981_w(c, gp - 0x994, (seed % 13 != 0) as u32);
                oc981_w(c, gp - 0x998, markers);
                oc981_w(c, gp - 0x9a0, if seed % 19 == 0 { 0 } else { course });
                oc981_w(c, gp - 0x848, world);
                oc981_w(c, gp - 0x854, renderer);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 17 == 0 { 0 } else { sx(rider) };
                c.g[A2][0] = (seed % 2) as u64;
                // The player's rider: getAIIndex (vtable +0x38/+0x3c).
                oc981_w(c, rider + 0x6c0, my_vt);
                c.bus
                    .write16(my_vt + 0x38, ((r.next_u32() % 16) * 4) as u16);
                let me = if seed % 7 == 0 { 0 } else { 1 + seed % 5 };
                if me == 0 {
                    oc981_w(c, my_vt + 0x3c, 0x0014_06c0);
                } else {
                    oc981_w(c, my_vt + 0x3c, FAKE);
                    oc981_w(c, LOG_RESULTS, me);
                }
                // The world: race object, rider list, racer count.
                let n = r.next_u32() % 7;
                oc981_w(c, world + 0x84, race);
                oc981_w(c, race + 0xc, list);
                oc981_w(c, list + 0x78, n);
                oc981_w(c, world + 0xc0, DATA + 0x400);
                oc981_w(c, DATA + 0x400 + 0x88, (r.next_u32() % 2) as u32);
                oc981_w(
                    c,
                    0x0053_08d0,
                    if r.next_u32() % 3 == 0 { 0 } else { 1 << 9 },
                );
                for i in 0..6u32 {
                    let rd = DATA + 0x2000 + 0x800 * i;
                    let vt = DATA + 0x8000 + 0x100 * i;
                    oc981_w(c, list + 0x28 + 4 * i, rd);
                    oc981_w(c, rd + 0x6c0, vt);
                    c.bus.write16(vt + 0x40, ((r.next_u32() % 16) * 4) as u16);
                    // isHuman: pointer (fake) or 0 (the leaf).
                    let f = if r.next_u32() % 2 == 0 {
                        FAKE + 0x10
                    } else {
                        0x0014_06c0
                    };
                    oc981_w(c, vt + 0x44, f);
                }
                // The meter's geometry and last rider index.
                let top = 50.0 + (r.next_u32() % 100) as f32;
                let height = 150.0 + (r.next_u32() % 100) as f32;
                oc981_w(c, this + 0x20, ((r.next_u32() % 400) as f32).to_bits());
                oc981_w(c, this + 0x24, top.to_bits());
                oc981_w(
                    c,
                    this + 0x28,
                    (20.0f32 + (r.next_u32() % 20) as f32).to_bits(),
                );
                oc981_w(c, this + 0x2c, height.to_bits());
                let scale = height / 10000.0 * (0.7 + (r.next_u32() % 7) as f32 * 0.1);
                oc981_w(c, this + 0x30, scale.to_bits());
                oc981_w(c, this + 0x34, if r.next_u32() % 3 == 0 { me } else { 9 });
                // Racer table: dist +8, pos +0xc, rel +0x10, relValid +0x14.
                let my_dist = 3000.0f32 + (r.next_u32() % 4000) as f32;
                let my_pos = (r.next_u32() % 4000) as f32;
                for i in 0..6u32 {
                    let row = table + 0x18 * i;
                    let (dist, pos) = if i == me {
                        (my_dist, my_pos)
                    } else {
                        let a = r.f32(6000.0);
                        let b = r.f32(3000.0);
                        (my_dist + f32::from_bits(a), my_pos + f32::from_bits(b))
                    };
                    oc981_w(c, row + 8, dist.to_bits());
                    oc981_w(c, row + 0xc, pos.to_bits());
                    oc981_floats(c, &mut r, row + 0x10, 1, 2500.0);
                    oc981_w(c, row + 0x14, (r.next_u32() % 2) as u32);
                }
                // Icon table: w +4, h +8, texture +0xc (handle, then uv rect).
                for k in 0..11u32 {
                    let row = icons + 0x10 * k;
                    let tex = DATA + 0xa000 + 0x40 * k;
                    oc981_w(c, row + 4, (4.0f32 + (r.next_u32() % 20) as f32).to_bits());
                    oc981_w(c, row + 8, (4.0f32 + (r.next_u32() % 20) as f32).to_bits());
                    oc981_w(c, row + 0xc, tex);
                    oc981_w(c, tex, r.next_u32() % 64);
                    oc981_floats(c, &mut r, tex + 4, 7, 64.0);
                }
                // Course markers: type +0, dist +4.
                oc981_w(c, course + 8, r.next_u32() % 7);
                for i in 0..6u32 {
                    let m = markers + 8 * i;
                    oc981_w(c, m, r.next_u32() % 4);
                    let d = my_dist + f32::from_bits(r.f32(6000.0));
                    oc981_w(c, m + 4, d.to_bits());
                }
                // The renderer: draw-state stack and the vtable of fakes.
                oc981_w(c, renderer + 0xe84, DATA + 0xc800);
                oc981_w(c, DATA + 0xc800 + 4, r.next_u32());
                oc981_w(c, DATA + 0xc800 + 8, r.next_u32());
                oc981_w(c, renderer + 0x10d8, rvt);
                for (i, off) in [0xf8u32, 0x110, 0x238, 0x100].into_iter().enumerate() {
                    c.bus.write16(rvt + off, ((r.next_u32() % 16) * 4) as u16);
                    oc981_w(c, rvt + off + 4, FAKE + 0x20 + 0x10 * i as u32);
                }
            },
        );
    }

    /// `OVScreenReadPrivateMessage_Init`: the screen is created (or not: the
    /// create-screen fake answers 0), widgets are present or missing.
    #[test]
    fn ov_screen_read_private_message_init_matches() {
        check(
            ee::s05::f_00210940__generated,
            ov_screen_read_private_message_init,
            60,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x210940, 14);
                oc988_gate(c);
                let this = DATA + 0x6000;
                let screen = DATA + 0x8000;
                let names = oc988_names(
                    &mut r,
                    &[
                        0x0047_1d00,
                        0x0047_1d10,
                        0x0047_1d20,
                        0x0047_1d30,
                        0x0047_1d40,
                        0x0046_e808,
                    ],
                    "d",
                );
                oc988_screen(c, &mut r, screen, &names);
                oc988_mailbox(c, 0, [if seed % 5 == 0 { 0 } else { screen }, 0, 0, 0]);
                oc988_w(c, this + 0x10, DATA + 0x6400);
                // world -> +0x78 (an interface owner).
                let world = DATA + 0x2000;
                oc988_w(c, GAME_GP - 0x848, world);
                oc988_w(c, world + 0x78, DATA + 0x2100);
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn ov_screen_trophy_on_widget_action_matches() {
        check(
            ee::s05::f_0020e9a0__generated,
            ov_screen_trophy_on_widget_action,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0020_e9a0, 4);
                let screen = DATA + 0x1000;
                let event = DATA + 0x1100;
                c.g[A0][0] = sx(screen);
                c.g[A1][0] = if seed % 7 == 0 { 0 } else { sx(event) };
                c.g[A2][0] = [5u64, 6, 7][(seed % 3) as usize];
                fill(c, &mut r, screen, 0x10);
                fill(c, &mut r, event, 0x10);
                // The screen's list: item word at +0x10 points at the item
                // pointer block (item pointer, then its vtable).
                let items = DATA + 0x1400;
                let item = DATA + 0x1500;
                let vt = DATA + 0x1600;
                c.bus.write32(screen + 0x10, items);
                c.bus.write32(items, item);
                c.bus.write32(item + 0x4, vt);
                fill(c, &mut r, vt, 0x10);
                fill(c, &mut r, item, 0x10);
                // The event's object: vtable at +0x8 with method at +0x5c.
                let object = DATA + 0x1200;
                let object_vt = DATA + 0x1300;
                c.bus.write32(event + 0x8, object);
                c.bus.write32(object + 0x4, object_vt);
                c.bus
                    .write32(event + 0x18, if seed % 2 == 0 { 1 } else { r.next_u32() });
                fill(c, &mut r, object_vt, 0x10);
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_transition_in_matches() {
        check(
            ee::s05::f_00210b58__generated,
            ov_screen_read_private_message_transition_in,
            200,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0021_0b58, 4);
                let screen = DATA + 0x1000;
                c.g[A0][0] = sx(screen);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x1100) };
                fill(c, &mut r, screen, 0x40);
                c.bus.write32(GAME_GP.wrapping_sub(0x204), DATA + 0x1300);
                fill(c, &mut r, DATA + 0x1300, 0x20);
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_set_message_text_matches() {
        check(
            ee::s05::f_00210bd8__generated,
            ov_screen_read_private_message_set_message_text,
            200,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0021_0bd8, 3);
                let screen = DATA + 0x1000;
                c.g[A0][0] = sx(screen);
                c.g[A1][0] = sx(DATA + 0x1100);
                c.g[A2][0] = if seed % 3 == 0 { 0 } else { 1 };
                fill(c, &mut r, screen, 0x40);
                let widget = DATA + 0x1200;
                c.bus.write32(screen + 0x40, widget);
                fill(c, &mut r, widget, 0x40);
                c.bus.write32(DATA + 0x1100, 0x0043_4241);
                let world = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                fill(c, &mut r, world, 0x40);
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_on_widget_action_matches() {
        check(
            ee::s05::f_00210df0__generated,
            ov_screen_read_private_message_on_widget_action,
            200,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0021_0df0, 4);
                let screen = DATA + 0x1000;
                c.g[A0][0] = sx(screen);
                c.g[A1][0] = sx(DATA + 0x1100);
                c.g[A2][0] = (seed % 9) as u64;
                fill(c, &mut r, screen, 0x40);
                // The widget at +0x40 is absent for some seeds.
                let widget = if seed % 5 == 0 { 0 } else { DATA + 0x1200 };
                c.bus.write32(screen + 0x40, widget);
                fill(c, &mut r, DATA + 0x1200, 0x40);
                // The reply at +0xe0: absent or present.
                let reply = if seed % 2 == 0 { 0 } else { DATA + 0x1300 };
                c.bus.write32(screen + 0xe0, reply);
                fill(c, &mut r, DATA + 0x1300, 0x40);
                c.bus.write32(DATA + 0x1300 + 0x74, 0x0043_4241);
                c.bus.write32(GAME_GP.wrapping_sub(0x204), DATA + 0x1400);
                fill(c, &mut r, DATA + 0x1400, 0x20);
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_ctor_matches() {
        check(
            ee::s05::f_00211088__generated,
            ov_screen_profile_save_pda_ctor,
            200,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0021_1088, 4);
                let this = DATA + 0x1000;
                let mgr = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x3000);
                // The type word: zero, or a value whose bits give 1..8 slots.
                c.g[A2][0] = [0u64, 2, 4, 6, 8, 12, 16, 0xff][(seed % 8) as usize];
                c.g[A3][0] = sx(mgr);
                fill(c, &mut r, this, 0x80);
                fill(c, &mut r, DATA + 0x3000, 0x20);
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_ctor_matches() {
        check(
            ee::s05::f_00221408__generated,
            ov_screen_read_private_message_ctor,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_read_private_message_ctor_2_matches() {
        check(
            ee::s05::f_00221468__generated,
            ov_screen_read_private_message_ctor_2,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_read_private_message_dtor_matches() {
        check(
            ee::s05::f_002214d0__generated,
            ov_screen_read_private_message_dtor,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    /// Random junk in every register and in the object (+0..0x100) at
    /// `DATA + 0x1000`, the object in `a0`. The constructors and destructors
    /// have no branches, so one path is the whole space; the junk makes sure
    /// every stored field is visibly written.
    pub(crate) fn oc1015_object_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0022_21c8 ^ seed as u64);
        scramble(c, &mut r);
        let obj = DATA + 0x1000;
        for k in 0..0x40 {
            c.bus.write32(obj + 4 * k, r.next_u32());
        }
        c.g[A0][0] = sx(obj);
    }

    #[test]
    fn ov_screen_trophy_ctor_matches() {
        check(
            ee::s05::f_002222f8__generated,
            ov_screen_trophy_ctor,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_trophy_ctor_2_matches() {
        check(
            ee::s05::f_00222358__generated,
            ov_screen_trophy_ctor_2,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_trophy_dtor_matches() {
        check(
            ee::s05::f_002223c0__generated,
            ov_screen_trophy_dtor,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }
}
