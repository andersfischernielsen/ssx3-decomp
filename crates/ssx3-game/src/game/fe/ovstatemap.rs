//! `src/fe/ovstatemap.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
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
pub(crate) use crate::game::ai::computer::lh_2;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::be::beintnetwork::ldl;
pub(crate) use crate::game::be::beintnetwork::sdl;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::dirtysock::tagsunk::at_3 as at;
pub(crate) use crate::game::dirtysock::tagsunk::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tagsunk::p_2 as p;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::fe::fepopupmisc::lh;
pub(crate) use crate::game::fe::fepopupmisc::lw_2;
pub(crate) use crate::game::fe::fepopupmisc::p_2;
pub(crate) use crate::game::fe::ovstateenterlodge::ov_image_sym;
pub(crate) use crate::game::fe::ovstateenterlodge::OV_STATE_BASE_CTOR;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::ov_state_dtor_body;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::sound::bankmonitor::oc767_addiu;
pub(crate) use crate::game::sound::bankmonitor::oc767_ea;
pub(crate) use crate::game::sound::bankmonitor::oc767_lw;
use crate::guest::Addr;
use crate::tick;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0020_a088,
        ov_screen_session_destroy_indicators,
        "OVScreenSession_DestroyIndicators",
    ),
    (
        0x0020_8b00,
        ov_screen_session_on_activate,
        "OVScreenSession_OnActivate",
    ),
    (
        0x0020_8b78,
        ov_screen_session_shutdown,
        "OVScreenSession_Shutdown",
    ),
    (
        0x0020_8bb8,
        ov_screen_session_hide_map_pic,
        "OVScreenSession_HideMapPic",
    ),
    (
        0x0020_8ef8,
        ov_screen_session_on_widget_notify,
        "OVScreenSession_OnWidgetNotify",
    ),
    (
        0x0020_8f10,
        ov_screen_session_load_map_image,
        "OVScreenSession_LoadMapImage",
    ),
    (
        0x0020_9300,
        ov_screen_session_post_input,
        "OVScreenSession_PostInput",
    ),
    (
        0x0020_9370,
        ov_screen_session_update,
        "OVScreenSession_Update",
    ),
    (
        0x0020_95e8,
        ov_screen_session_set_highlight_color,
        "OVScreenSession_SetHighlightColor",
    ),
    (
        0x0020_9648,
        ov_screen_session_set_unlocked_color,
        "OVScreenSession_SetUnlockedColor",
    ),
    (
        0x0020_9e78,
        ov_screen_session_update_indicators,
        "OVScreenSession_UpdateIndicators",
    ),
    (
        0x0020_a6a8,
        ov_state_overlay_on_widget_action,
        "cOVStateOverlay_OnWidgetAction",
    ),
    (
        0x0020_a6f0,
        c_ov_state_overlay_play_object_anim,
        "cOVStateOverlay_PlayObjectAnim",
    ),
    (0x0020_a1a8, fe_format_min_sec, "FE_FormatMinSec"),
    (0x0020_86a8, ovscreensession_init, "OVScreenSession_Init"),
    (
        0x0020_87f0,
        ov_screen_session_transition_in,
        "OVScreenSession_TransitionIn",
    ),
    (
        0x0020_8c28,
        ov_screen_session_on_widget_action,
        "OVScreenSession_OnWidgetAction",
    ),
    (
        0x0020_8f88,
        ov_screen_session_update_confirm_popup,
        "OVScreenSession_UpdateConfirmPopup",
    ),
    (
        0x0020_96a8,
        ov_screen_session_create_player_indicator,
        "OVScreenSession_CreatePlayerIndicator",
    ),
    (
        0x0020_9970,
        ov_screen_session_create_map_indicators,
        "OVScreenSession_CreateMapIndicators",
    ),
    (
        0x0020_9fd0,
        ovscreensession_hide_indicators,
        "OVScreenSession_HideIndicators",
    ),
    (
        0x0020_a4e0,
        ovscreen_final_standings_recv_chat_message,
        "OVScreenFinalStandings_RecvChatMessage",
    ),
    (0x0022_12c8, ov_screen_session_ctor, "OVScreenSession_ctor"),
    (
        0x0022_1350,
        ov_screen_session_ctor_2,
        "OVScreenSession_ctor_2",
    ),
    (0x0022_13d8, ov_screen_session_dtor, "OVScreenSession_dtor"),
    (
        0x0020_87e8,
        ov_screen_session_on_widget_created_002087e8,
        "OVScreenSession_OnWidgetCreated",
    ),
    (
        0x0022_1400,
        ov_screen_session_v13_00221400,
        "OVScreenSession_v13",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH as STR_HASH_3;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;

    pub(crate) use crate::game::fe::fepopup::callee::C0020E900;

    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_SHUTDOWN;

    pub(crate) use crate::game::fe::ovstatepause::callee::FE_ASYNC_MANAGER_CANCEL;

    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;

    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
}

/// `void OVScreenSession_Shutdown(Session* s)` (0x208b78): cancel the
/// session's async file, destroy the indicators and shut the overlay down.
pub fn ov_screen_session_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        let g = crate::abi::r32(c, GP);
        let world = lw(c, g, 0xfffff7b8); // gp - 0x848
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[A1][0] = lw(c, this, 0xc4);
        c.g[A0][0] = lw(c, world as u32, 0x11c);
        callee::FE_ASYNC_MANAGER_CANCEL.call(c, 0x0020_8b98)?;
        c.g[A0][0] = c.g[S0][0];
        callee::OV_SCREEN_SESSION_DESTROY_INDICATORS.call(c, 0x0020_8ba0)?;
        c.g[A0][0] = c.g[S0][0];
        callee::C_OV_STATE_OVERLAY_SHUTDOWN.call(c, 0x0020_8ba8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cOVStateOverlay_OnWidgetAction(self, widget, action)` (0x20a6a8):
/// pass the action to the overlay screen's widget handler, when one exists.
pub fn ov_state_overlay_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw(gp(c, -0x848));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at(c, V0, 0x84));
        let no_race = c.g[A0][0] == 0;
        c.g[RA][0] = c.ld(p(sp(c), 0));
        if no_race {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A3][0] = c.lw(at(c, A0, 0x200));
        let no_screen = c.g[A3][0] == 0;
        if no_screen {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V0][0] = c.lw(at(c, A3, 0x0c));
        c.g[A0][0] = c.lh(p(r32(c, V0), 0x48));
        c.g[V1][0] = c.lw(p(r32(c, V0), 0x4c));
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0020_a6e0);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0020_a6e0)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void OVScreenSession_SetHighlightColor(Session* s, void* obj, int on)`
/// (0x2095e8): forward the highlight colour (`D_004C8B38`) or the default
/// colour (`D_004C8B28`) to the object's vtable slot 0x58.
pub fn ov_screen_session_set_highlight_color(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        let color = c.g[A2][0];
        frame.save_ra(c, 0);
        if color != 0 {
            c.cycles += 9;
            c.g[V1][0] = lw_2(c, p_2(c.g[A3][0], 8));
            c.g[A1][0] = 0x004d_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74c8));
            c.g[A0][0] = lh(c, p_2(c.g[V1][0], 0x58));
            c.g[V0][0] = lw_2(c, p_2(c.g[V1][0], 0x5c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_9614)?;
            c.g[RA][0] = Addr(p_2(c.g[SP][0], 0)).ld(c);
        } else {
            c.cycles += 8;
            c.g[V1][0] = lw_2(c, p_2(c.g[A3][0], 8));
            c.g[A1][0] = 0x004d_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74d8));
            c.g[A0][0] = lh(c, p_2(c.g[V1][0], 0x58));
            c.g[V0][0] = lw_2(c, p_2(c.g[V1][0], 0x5c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_9638)?;
            c.g[RA][0] = Addr(p_2(c.g[SP][0], 0)).ld(c);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenSession_SetUnlockedColor(Session* s, void* obj, int on)`
/// (0x209648): forward the unlocked colour (`D_004C8B58`) or the locked
/// colour (`D_004C8B48`) to the object's vtable slot 0x58.
pub fn ov_screen_session_set_unlocked_color(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        let color = c.g[A2][0];
        frame.save_ra(c, 0);
        if color != 0 {
            c.cycles += 9;
            c.g[V1][0] = lw_2(c, p_2(c.g[A3][0], 8));
            c.g[A1][0] = 0x004d_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74a8));
            c.g[A0][0] = lh(c, p_2(c.g[V1][0], 0x58));
            c.g[V0][0] = lw_2(c, p_2(c.g[V1][0], 0x5c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_9674)?;
            c.g[RA][0] = Addr(p_2(c.g[SP][0], 0)).ld(c);
        } else {
            c.cycles += 8;
            c.g[V1][0] = lw_2(c, p_2(c.g[A3][0], 8));
            c.g[A1][0] = 0x004d_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74b8));
            c.g[A0][0] = lh(c, p_2(c.g[V1][0], 0x58));
            c.g[V0][0] = lw_2(c, p_2(c.g[V1][0], 0x5c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_9698)?;
            c.g[RA][0] = Addr(p_2(c.g[SP][0], 0)).ld(c);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int OVScreenSession_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x00208ef8): 0 for the keys 8 and 9, 0x101 for the rest.
pub fn ov_screen_session_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x8));
    c.g[V0][0] = 0x101;
    c.g[A2][0] = (c.g[A2][0] < 0x2) as u64;
    if c.g[A2][0] != 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `void OVScreenSession_HideMapPic(OVScreenSession* self)` (0x208bb8):
/// drop both map pictures and hide the map.
pub fn ov_screen_session_hide_map_pic(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(at_2(c, S0, 0xdc)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(at_2(c, S0, 0xd0)).lw(c);
        let none = c.g[A2][0] == 0;
        Addr(at_2(c, S0, 0xc8)).sw(c, 0);
        if !none {
            c.cycles += 6;
            c.g[V0][0] = Addr(at_2(c, A2, 8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(at_2(c, V0, 0x48)).lh(c);
            c.g[V1][0] = Addr(at_2(c, V0, 0x4c)).lw(c);
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx(at_2(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0020_8bf4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(at_2(c, S0, 0xe0)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(at_2(c, A2, 8)).lw(c);
            c.g[A0][0] = Addr(at_2(c, V0, 0x48)).lh(c);
            c.g[V1][0] = Addr(at_2(c, V0, 0x4c)).lw(c);
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx(at_2(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0020_8c14)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenSession_PostInput(OVScreenSession* self, void* widget)`
/// (0x209300): when either of the widget's two status methods reports a
/// change, refresh the session's indicators.
pub fn ov_screen_session_post_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = Addr(at_2(c, S0, 8)).lw(c);
        c.g[A0][0] = Addr(at_2(c, V1, 0x88)).lh(c);
        c.g[V0][0] = Addr(at_2(c, V1, 0x8c)).lw(c);
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx(at_2(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0020_932c)?;
        let first = c.g[V0][0] != 0;
        let mut update = first;
        if !first {
            c.cycles += 7;
            c.g[V1][0] = Addr(at_2(c, S0, 8)).lw(c);
            c.g[A0][0] = Addr(at_2(c, V1, 0x90)).lh(c);
            c.g[V0][0] = Addr(at_2(c, V1, 0x94)).lw(c);
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx(at_2(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0020_9348)?;
            update = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x20);
        }
        if update {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0020_9e78, 0x0020_9358)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_00208B00` (0x208b00): `OVScreenSession::OnActivate`. Activates the
/// screen (`0x39e4c0`), looks up the widget named `0x470a20`
/// (`0x39d860(this->+0xd4, hash)`) and, if found, calls its `vtable[0x38]`
/// (with 0) and `vtable[0x48]` (with 0). Signature: `void on_activate(this)`.
pub fn ov_screen_session_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0020_8b14);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e4c0, 0x0020_8b14)?;
        c.g[A0][0] = 0x47_0000;
        c.g[RA][0] = sx(0x0020_8b20);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0xa20));
        call_ptr(c, 0x0031_7670, 0x0020_8b20)?;
        c.g[A0][0] = lw_r(c, S0, 0xd4);
        c.g[RA][0] = sx(0x0020_8b2c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0020_8b2c)?;
        c.g[S0][0] = c.g[V0][0];
        let missing = c.g[S0][0] == 0;
        c.g[A1][0] = 0;
        if !missing {
            c.cycles += 11;
            c.g[V0][0] = lw_r(c, S0, 0x8);
            c.g[A0][0] = lh_r(c, V0, 0x38);
            c.g[V1][0] = lw_r(c, V0, 0x3c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0020_8b4c);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0020_8b4c)?;
            c.g[V0][0] = lw_r(c, S0, 0x8);
            c.g[A1][0] = 0;
            c.g[A0][0] = lh_r(c, V0, 0x48);
            c.g[V1][0] = lw_r(c, V0, 0x4c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0020_8b64);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0020_8b64)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenSession_LoadMapImage(self)` (0x208f10): unload the previous
/// map image through the async manager, then start loading this screen's.
pub fn ov_screen_session_load_map_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.sq(17, sp_2(c).wrapping_add(0x20));
        c.sq(16, sp_2(c).wrapping_add(0x30));
        c.sq(18, sp_2(c).wrapping_add(0x10));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        callee::CBE_GET_BE.call(c, 0x0020_8f2c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0020_8f38)?;
        c.g[V1][0] = Addr(gpo(c, -0x848)).lw(c);
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = lw_r(c, S1, 0xc4);
        c.g[S0][0] = lw_r(c, V1, 0x11c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001a_38c0, 0x0020_8f50)?;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0014_4bc0, 0x0020_8f58)?;
        c.g[A2][0] = lw_r(c, S1, 0xc4);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = Addr(low(c, V0)).lw(c);
        c.g[A3][0] = 0x1;
        call_ptr(c, 0x001a_37f8, 0x0020_8f6c)?;
        sw_r(c, S1, 0xc8, 0);
        c.lq(16, sp_2(c).wrapping_add(0x30));
        c.lq(17, sp_2(c).wrapping_add(0x20));
        c.lq(18, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void cOVStateOverlay_PlayObjectAnim(cOVStateOverlay* self, char* widget,
/// char* anim)` (0x20a6f0): find the widget and play the named animation on
/// it (the overlay counterpart of `MemCardStateBase_PlayWidgetAnim`).
pub fn c_ov_state_overlay_play_object_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        open(c, 0x40);
        let sp0 = spv(c);
        c.sq(16, at_3(sp0, 0x30));
        c.sq(18, at_3(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, at_3(sp0, 0x20));
        c.g[A0][0] = c.g[A1][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, callee::STR_HASH_3, 0x0020_a714)?;
        let this = c.g[S0][0];
        c.g[A0][0] = lw_at(c, this, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x0020_a720)?;
        c.g[S1][0] = c.g[V0][0];
        let found = c.g[S1][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if found {
            c.cycles += 8;
            c.g[S0][0] = lw_at(c, this, 0x10);
            c.g[S0][0] = sx(at_3(c.g[S0][0] as u32, 0x50));
            call_ptr(c, callee::STR_HASH_3, 0x0020_a738)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_ANIM_SET_CREATE_ANIM, 0x0020_a744)?;
            let anim = c.g[V0][0];
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if !none {
                c.cycles += 5;
                c.g[A1][0] = anim;
                c.g[A2][0] = 3;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                call_ptr(c, callee::UI_OBJECT_SET_ANIM, 0x0020_a760)?;
            }
        }
        c.cycles += 6;
        c.lq(16, at_3(sp0, 0x30));
        c.lq(17, at_3(sp0, 0x20));
        c.lq(18, at_3(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_3(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}

/// The virtual method call the indicator destructor makes: `obj`'s vtable at
/// `+8`, slot `{delta at +delta_off, fn at +fn_off}`, called with
/// `this = obj + delta` and the caller's `a1` untouched.  Reproduces the
/// generated body's register dance (the target stays in `v1`, the loaded
/// delta in `a0`) and returns the unwind flow.
pub(crate) fn call_indicator_method(
    c: &mut Cpu,
    obj: u64,
    delta_off: u32,
    fn_off: u32,
    ret: u32,
) -> Flow {
    c.g[V0][0] = c.lw((obj as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(delta_off));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(fn_off));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `OVScreenSession_DestroyIndicators` (0x0020a088): destroy an overlay
/// screen session's indicator objects.  The object at `self+0xe0` gets its
/// `+0x4c` method (arg 0) and then its `+0xc` method (arg 3), and the eight
/// `self+0xe4..`/`self+0x104..` slots are cleared the same way, each pair
/// through the slot object's vtable at `+8`.
pub fn ov_screen_session_destroy_indicators(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe0));
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0u64;
        if !none {
            c.cycles += 8;
            call_indicator_method(c, c.g[A2][0], 0x48, 0x4c, 0x0020_a0c0)?;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe0));
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 0x3;
            if !none {
                c.cycles += 5;
                call_indicator_method(c, c.g[A2][0], 0x8, 0xc, 0x0020_a0e0)?;
            }
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xe0), 0u64);
        }
        c.cycles += 3;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe4));
        c.g[S2][0] = 0u64;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x104));
        loop {
            c.cycles += 3;
            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 0u64;
            if !none {
                c.cycles += 8;
                call_indicator_method(c, c.g[A2][0], 0x48, 0x4c, 0x0020_a110)?;
                c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                let none = c.g[A2][0] == 0;
                c.g[A1][0] = 0x3;
                if !none {
                    c.cycles += 5;
                    call_indicator_method(c, c.g[A2][0], 0x8, 0xc, 0x0020_a130)?;
                }
                c.cycles += 1;
                c.sw(c.g[S1][0] as u32, 0u64);
            }
            c.cycles += 3;
            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 0u64;
            if !none {
                c.cycles += 8;
                call_indicator_method(c, c.g[A2][0], 0x48, 0x4c, 0x0020_a154)?;
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                let none = c.g[A2][0] == 0;
                c.g[A1][0] = 0x3;
                if !none {
                    c.cycles += 5;
                    call_indicator_method(c, c.g[A2][0], 0x8, 0xc, 0x0020_a174)?;
                }
                c.cycles += 1;
                c.sw(c.g[S0][0] as u32, 0u64);
            }
            c.cycles += 5;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = ((c.g[S2][0] as i64) < 8i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0020_a0f0);
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

/// `void func_00209370(sMap_9370* self)` (0x209370): per-frame update of the
/// session/loading overlay: run the overlay's base update, then either fade
/// the image in (`fade += 0.1` up to 1) and push it to the colour setter, or
/// clear it; poll the async file status and, once the file is ready, hand the
/// loaded data to the image; then bob the eight map icons (sinusoidal scale
/// and offset) around the icon positions.
pub fn ov_screen_session_update(c: &mut Cpu) {
    let frame = Frame::open(c, 0x80);
    'b_2095c8: {
        c.cycles += 13;
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0x20);
        c.swc1(21, at(c, SP, 0x78));
        c.g[RA][0] = sx(0x00209398);
        c.swc1(20, at(c, SP, 0x70));
        if callee::C0020E900.call(c, 0x00209398).is_err() {
            return;
        }
        c.g[V0][0] = lw_3(c, at(c, S0, 0xdc));
        if c.g[V0][0] != 0u64 {
            frame.restore(c, S0, 0x60);
            break 'b_2095c8;
        } else {
            'b_209460: {
                'b_209440: {
                    c.cycles += 3;
                    c.g[V0][0] = lw_3(c, at(c, S0, 0xc8));
                    if c.g[V0][0] == 0u64 {
                        sw(c, at(c, S0, 0xcc), 0u64);
                        break 'b_209440;
                    } else {
                        'b_2093dc: {
                            c.cycles += 7;
                            c.lwc1(1, at(c, S0, 0xcc));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.c_lt_s(1, 0);
                            if !c.fcc() {
                                c.g[A0][0] = lw_3(c, at(c, S0, 0xd0));
                                break 'b_2093dc;
                            } else {
                                c.cycles += 4;
                                c.lwc1(0, r32(c, GP).wrapping_sub(0x5504));
                                // The fade step (0.1 at `gp-0x5504`) is one
                                // authored 60 Hz frame's worth of alpha; at a
                                // higher tick rate scale it so the fade still
                                // takes ten frames of wall time.
                                let r = tick::rate(c);
                                if r == 60 {
                                    c.add_s(0, 1, 0);
                                } else {
                                    c.g[AT][0] = (60.0f32 / r as f32).to_bits() as u64;
                                    c.mtc1(AT, 2);
                                    c.mul_s(0, 0, 2);
                                    c.add_s(0, 1, 0);
                                }
                                c.swc1(0, at(c, S0, 0xcc));
                                c.g[A0][0] = lw_3(c, at(c, S0, 0xd0));
                                break 'b_2093dc;
                            }
                        }
                        c.cycles += 25;
                        c.g[A1][0] = c.g[SP][0];
                        c.lwc1(0, at(c, S0, 0xcc));
                        ldl(c, 2, at(c, A0, 0x1c));
                        ldl(c, 3, at(c, A0, 0x24));
                        sdl(c, 2, r32(c, SP));
                        sdl(c, 3, at(c, SP, 0x8));
                        c.swc1(0, r32(c, SP));
                        c.g[V0][0] = lw_3(c, at(c, A0, 0x8));
                        c.g[V1][0] = lh_2(c, at(c, V0, 0x58));
                        c.g[A2][0] = lw_3(c, at(c, V0, 0x5c));
                        let t = c.g[A2][0] as u32;
                        c.g[RA][0] = sx(0x0020941c);
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                        if call_ptr(c, t, 0x0020941c).is_err() {
                            return;
                        }
                        c.g[A3][0] = lw_3(c, at(c, S0, 0xd0));
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = lw_3(c, at(c, A3, 0x8));
                        c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
                        c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00209438);
                        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x00209438).is_err() {
                            return;
                        }
                        let t = true;
                        c.g[V0][0] = lw_3(c, r32(c, GP).wrapping_sub(0x848));
                        if t {
                            break 'b_209460;
                        } else {
                            break 'b_209440;
                        }
                    }
                }
                c.cycles += 8;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = lw_3(c, at(c, S0, 0xd0));
                c.g[V0][0] = lw_3(c, at(c, A2, 0x8));
                c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
                c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0020945c);
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x0020945c).is_err() {
                    return;
                }
                c.g[V0][0] = lw_3(c, r32(c, GP).wrapping_sub(0x848));
                break 'b_209460;
            }
            'b_2094bc: {
                c.cycles += 7;
                c.g[A1][0] = lw_3(c, at(c, S0, 0xc4));
                c.g[S1][0] = lw_3(c, at(c, V0, 0x11c));
                c.g[RA][0] = sx(0x00209470);
                c.g[A0][0] = c.g[S1][0];
                if call_ptr(c, 0x001a39c0, 0x00209470).is_err() {
                    return;
                }
                c.g[V1][0] = 0x3;
                if c.g[V0][0] != c.g[V1][0] {
                    c.lwc1(1, at(c, S0, 0x1d0));
                    break 'b_2094bc;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw_3(c, at(c, S0, 0xc8));
                    if c.g[V0][0] != 0u64 {
                        c.lwc1(1, at(c, S0, 0x1d0));
                        break 'b_2094bc;
                    } else {
                        'b_2094b8: {
                            c.cycles += 3;
                            c.g[A2][0] = lw_3(c, at(c, S0, 0xd0));
                            let t = c.g[A2][0] == 0u64;
                            c.g[V1][0] = 0x11c;
                            if t {
                                break 'b_2094b8;
                            } else {
                                c.cycles += 9;
                                c.g[V0][0] = lw_3(c, at(c, S0, 0xc4));
                                c.g[A1][0] = 0x1;
                                c.mult(0, A0, V0, V1);
                                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                                c.g[A0][0] = lw_3(c, at(c, V0, 0x110));
                                sw(c, at(c, A2, 0x7c), 0u64);
                                sw(c, at(c, A2, 0x78), c.g[A0][0]);
                                sw(c, at(c, S0, 0xc8), c.g[A1][0]);
                                sw(c, at(c, S0, 0xcc), 0u64);
                                break 'b_2094b8;
                            }
                        }
                        c.cycles += 1;
                        c.lwc1(1, at(c, S0, 0x1d0));
                        break 'b_2094bc;
                    }
                }
            }
            'b_209518: {
                'b_209514: {
                    'b_2094f4: {
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_lt_s(0, 1);
                        let t = !c.fcc();
                        c.lwc1(1, at(c, S0, 0x1cc));
                        if t {
                            break 'b_2094f4;
                        } else {
                            c.cycles += 6;
                            c.g[AT][0] = 0x41ac0000;
                            c.mtc1(1, 0);
                            c.c_lt_s(0, 1);
                            if !c.fcc() {
                                c.lwc1(0, at(c, S0, 0x1cc));
                                break 'b_209518;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, r32(c, GP).wrapping_sub(0x5500));
                                let t = true;
                                c.swc1(0, at(c, S0, 0x1d0));
                                if t {
                                    break 'b_209514;
                                } else {
                                    break 'b_2094f4;
                                }
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[AT][0] = 0x41900000;
                    c.mtc1(1, 0);
                    c.c_lt_s(1, 0);
                    if !c.fcc() {
                        c.lwc1(0, at(c, S0, 0x1cc));
                        break 'b_209518;
                    } else {
                        c.cycles += 2;
                        c.lwc1(0, r32(c, GP).wrapping_sub(0x54fc));
                        c.swc1(0, at(c, S0, 0x1d0));
                        break 'b_209514;
                    }
                }
                c.cycles += 1;
                c.lwc1(0, at(c, S0, 0x1cc));
                break 'b_209518;
            }
            c.cycles += 10;
            c.g[S2][0] = sx(at(c, S0, 0x124));
            c.lwc1(1, at(c, S0, 0x1d0));
            c.g[S1][0] = sx(at(c, S0, 0x104));
            c.g[AT][0] = 0x41900000;
            c.mtc1(1, 21);
            c.g[S3][0] = 0x7;
            c.add_s(0, 0, 1);
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 20);
            c.swc1(0, at(c, S0, 0x1cc));
            'l_209540: loop {
                'b_2095b8: {
                    c.cycles += 3;
                    c.g[V0][0] = lw_3(c, r32(c, S1));
                    if c.g[V0][0] == 0u64 {
                        c.g[S2][0] = sx(at(c, S2, 0xc));
                        break 'b_2095b8;
                    } else {
                        c.cycles += 27;
                        c.lwc1(0, at(c, S0, 0x1cc));
                        c.lwc1(2, r32(c, S2));
                        c.sub_s(0, 0, 21);
                        c.lwc1(1, at(c, S2, 0x4));
                        c.mul_s(0, 0, 20);
                        c.sub_s(1, 1, 0);
                        c.sub_s(2, 2, 0);
                        c.swc1(1, at(c, SP, 0x4));
                        c.swc1(2, r32(c, SP));
                        ldl(c, 6, r32(c, SP));
                        c.g[A3][0] = lw_3(c, at(c, SP, 0x8));
                        sdl(c, 6, at(c, V0, 0x44));
                        sw(c, at(c, V0, 0x4c), c.g[A3][0]);
                        c.g[V0][0] = lw_3(c, r32(c, S1));
                        c.lwc1(0, at(c, S0, 0x1cc));
                        c.g[V1][0] = lw_3(c, at(c, V0, 0x8));
                        c.g[A0][0] = lh_2(c, at(c, V1, 0xa8));
                        c.swc1(0, at(c, SP, 0x14));
                        c.swc1(0, at(c, SP, 0x10));
                        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                        sw(c, at(c, SP, 0x18), 0u64);
                        c.g[V0][0] = lw_3(c, at(c, V1, 0xac));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x002095b4);
                        c.g[A1][0] = sx(at(c, SP, 0x10));
                        if call_ptr(c, t, 0x002095b4).is_err() {
                            return;
                        }
                        c.g[S2][0] = sx(at(c, S2, 0xc));
                        break 'b_2095b8;
                    }
                }
                c.cycles += 3;
                c.g[S3][0] = sx(r32(c, S3).wrapping_sub(0x1));
                let t = (c.g[S3][0] as i64) >= 0;
                c.g[S1][0] = sx(at(c, S1, 0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00209540);
                    }
                    continue 'l_209540;
                } else {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x60);
                    break 'b_2095c8;
                }
            }
        }
    }
    c.cycles += 8;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    c.g[RA][0] = ld(c, at(c, SP, 0x20));
    c.lwc1(21, at(c, SP, 0x78));
    c.lwc1(20, at(c, SP, 0x70));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x80));
    c.pc = t;
    return;
}

/// `void func_00209E78(void* self)` (0x209e78): update the session map's
/// indicator icons: show the cursor, read the list box's selection out of the
/// screen's object table and show the "selected" icon (and hide the other)
/// for the selected entry, the opposite for the rest.
pub fn ov_screen_session_update_indicators(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_209ebc: {
        c.cycles += 12;
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[A2][0] = lw_3(c, at(c, S2, 0xe0));
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = 0x1;
        if t {
            break 'b_209ebc;
        } else {
            c.cycles += 5;
            c.g[V0][0] = lw_3(c, at(c, A2, 0x8));
            c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
            c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00209ebc);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x00209ebc).is_err() {
                return;
            }
            break 'b_209ebc;
        }
    }
    'b_209ee4: {
        c.cycles += 9;
        c.g[A0][0] = 0x470000;
        c.g[RA][0] = sx(0x00209ec8);
        c.g[A0][0] = sx(at(c, A0, 0x1850));
        if call_ptr(c, 0x00317670, 0x00209ec8).is_err() {
            return;
        }
        c.g[A0][0] = lw_3(c, at(c, S2, 0xd4));
        c.g[RA][0] = sx(0x00209ed4);
        c.g[A1][0] = c.g[V0][0];
        if call_ptr(c, 0x0039d860, 0x00209ed4).is_err() {
            return;
        }
        c.g[V0][0] = lw_3(c, at(c, V0, 0xa0));
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = 0x1;
        if t {
            break 'b_209ee4;
        } else {
            c.cycles += 1;
            c.g[V1][0] = lw_3(c, at(c, V0, 0x18));
            break 'b_209ee4;
        }
    }
    'b_209fa8: {
        c.cycles += 3;
        c.g[V0][0] = lw_3(c, at(c, S2, 0xc0));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0u64;
        if t {
            break 'b_209fa8;
        } else {
            c.cycles += 4;
            c.g[S5][0] = sx(r32(c, V1).wrapping_sub(0x1));
            c.g[S4][0] = sx(at(c, S2, 0x104));
            c.g[S0][0] = sx(at(c, S2, 0xe4));
            c.g[S3][0] = 0u64;
            'l_209f00: loop {
                'b_209f94: {
                    'b_209f90: {
                        'b_209f50: {
                            c.cycles += 2;
                            let t = c.g[S1][0] != c.g[S5][0];
                            c.g[A2][0] = lw_3(c, r32(c, S0));
                            if t {
                                break 'b_209f50;
                            } else {
                                'b_209f24: {
                                    c.cycles += 2;
                                    let t = c.g[A2][0] == 0u64;
                                    c.g[A1][0] = 0u64;
                                    if t {
                                        break 'b_209f24;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = lw_3(c, at(c, A2, 0x8));
                                        c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
                                        c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00209f24);
                                        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                        if call_ptr(c, t, 0x00209f24).is_err() {
                                            return;
                                        }
                                        break 'b_209f24;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, S3)));
                                c.g[A2][0] = lw_3(c, r32(c, V0));
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_209f90;
                                } else {
                                    c.cycles += 7;
                                    c.g[V0][0] = lw_3(c, at(c, A2, 0x8));
                                    c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
                                    c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00209f48);
                                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x00209f48).is_err() {
                                        return;
                                    }
                                    let t = true;
                                    c.g[V0][0] = lw_3(c, at(c, S2, 0xc0));
                                    if t {
                                        break 'b_209f94;
                                    } else {
                                        break 'b_209f50;
                                    }
                                }
                            }
                        }
                        'b_209f6c: {
                            c.cycles += 2;
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_209f6c;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = lw_3(c, at(c, A2, 0x8));
                                c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
                                c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00209f6c);
                                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                if call_ptr(c, t, 0x00209f6c).is_err() {
                                    return;
                                }
                                break 'b_209f6c;
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, S3)));
                        c.g[A2][0] = lw_3(c, r32(c, V0));
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_209f90;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = lw_3(c, at(c, A2, 0x8));
                            c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
                            c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00209f90);
                            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                            if call_ptr(c, t, 0x00209f90).is_err() {
                                return;
                            }
                            break 'b_209f90;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = lw_3(c, at(c, S2, 0xc0));
                    break 'b_209f94;
                }
                c.cycles += 5;
                c.g[S1][0] = sx(at(c, S1, 0x1));
                c.g[S0][0] = sx(at(c, S0, 0x4));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S3][0] = sx(at(c, S3, 0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00209f00);
                    }
                    continue 'l_209f00;
                } else {
                    break 'b_209fa8;
                }
            }
        }
    }
    c.cycles += 9;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x70));
    c.pc = t;
    return;
}

/// Where `cBEOptionInterface_mOption` (the 0x288-byte option block) lives.
pub(crate) const FE_FORMAT_MIN_SEC_OPTIONS: u32 = 0x0053_5610;

/// `char* FE_FormatMinSec(int secs)` (0x20a1a8): formats `secs` as minutes
/// and seconds into the shared clock buffer at 0x4c6848 and returns it.
/// The format depends on the option block's mode (bits 22..24 of its first
/// word, copied to the stack first): 0/2 use the format at 0x4719f0, 1/3 the
/// one at 0x471a00, other modes leave the buffer alone.
///
/// The generated body also holds an unaligned copy loop for an option block
/// that is not 8-byte aligned. The block address is the constant 0x535610,
/// so that branch is dead and not reproduced.
pub fn fe_format_min_sec(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x2e0);
        frame.save(c, 16, 0x2d0);
        frame.save(c, 17, 0x2c0);
        frame.save(c, 18, 0x2b0);
        frame.save(c, 19, 0x2a0);
        frame.save_ra(c, 0x290);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0020_a1c8)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_dd58, 0x0020_a1d4)?; // cBE_getInterface
        c.g[V1][0] = 0x3c;
        c.g[V0][0] = 0x3c;
        c.div(0, S0, V1); // minutes = secs / 60, seconds = secs % 60

        // Copy the option block (0x280 bytes in 32-byte chunks, then 8 more
        // bytes) to the bottom of the frame.
        c.cycles += 7;
        c.g[A0][0] = sx(FE_FORMAT_MIN_SEC_OPTIONS);
        c.g[V0][0] = c.g[A0][0] & 7;
        c.g[S0][0] = c.lo[0];
        c.g[S1][0] = c.hi[0];
        c.g[V1][0] = c.g[SP][0];
        c.cycles += 2;
        oc767_addiu(c, V0, A0, 0x280);
        c.g[S3][0] = 0x4c0000;
        loop {
            c.cycles += 11;
            c.g[A1][0] = Addr(oc767_ea(c, A0, 0)).ld(c);
            c.g[A2][0] = Addr(oc767_ea(c, A0, 8)).ld(c);
            c.g[A3][0] = Addr(oc767_ea(c, A0, 0x10)).ld(c);
            c.g[T0][0] = Addr(oc767_ea(c, A0, 0x18)).ld(c);
            let (d, v) = (oc767_ea(c, V1, 0), c.g[A1][0]);
            c.sd(d, v);
            let (d, v) = (oc767_ea(c, V1, 8), c.g[A2][0]);
            c.sd(d, v);
            let (d, v) = (oc767_ea(c, V1, 0x10), c.g[A3][0]);
            c.sd(d, v);
            let (d, v) = (oc767_ea(c, V1, 0x18), c.g[T0][0]);
            c.sd(d, v);
            oc767_addiu(c, A0, A0, 0x20);
            let again = c.g[A0][0] != c.g[V0][0];
            oc767_addiu(c, V1, V1, 0x20);
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0020_a268);
            }
        }
        c.cycles += 10;
        let a = oc767_ea(c, A0, 7);
        c.ldl(2, a);
        let a = oc767_ea(c, A0, 0);
        c.ldr(2, a);
        let a = oc767_ea(c, V1, 7);
        c.sdl(2, a);
        let a = oc767_ea(c, V1, 0);
        c.sdr(2, a);
        c.g[V0][0] = oc767_lw(c, SP, 0); // option flags
        c.g[V1][0] = 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 22);
        c.g[A0][0] = c.g[V0][0] & 7;
        let mode = c.g[A0][0];

        // Mode 1 and 3 (format B), 0 and 2 (format A), others: no text.
        let mut format = 0; // 0: none, 1: A, 2: B
        if mode == c.g[V1][0] {
            format = 2;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 2;
            if mode == 0 {
                format = 1;
            } else {
                c.cycles += 2;
                let t = mode == c.g[V0][0];
                c.g[V0][0] = 3;
                if t {
                    format = 1;
                } else {
                    c.cycles += 2;
                    let t = mode == c.g[V0][0];
                    oc767_addiu(c, V0, S3, 0x6848);
                    if t {
                        format = 2;
                    } else {
                        c.cycles += 2;
                        frame.restore(c, 16, 0x2d0);
                    }
                }
            }
        }
        if format != 0 {
            let (cycles, fmt, r1, r2, r3) = if format == 1 {
                (17, 0x19f0, 0x0020_a2f4, 0x0020_a304, 0x0020_a318)
            } else {
                (16, 0x1a00, 0x0020_a338, 0x0020_a348, 0x0020_a35c)
            };
            c.cycles += cycles;
            c.mtc1(16, 12);
            c.cvt_s_w(12, 12);
            oc767_addiu(c, S2, S3, 0x6848);
            c.g[S0][0] = 0x470000;
            oc767_addiu(c, S0, S0, fmt);
            call_ptr(c, 0x0041_3af8, r1)?; // fptodp(minutes)
            c.mtc1(17, 12);
            c.cvt_s_w(12, 12);
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x0041_3af8, r2)?; // fptodp(seconds)
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[V0][0];
            call_ptr(c, 0x0041_7828, r3)?; // sprintf(buffer, fmt, min, sec)
            oc767_addiu(c, V0, S3, 0x6848);
            c.cycles += 1;
            frame.restore(c, 16, 0x2d0);
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x2c0);
        frame.restore(c, 18, 0x2b0);
        frame.restore(c, 19, 0x2a0);
        frame.restore_ra(c, 0x290);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenSession_OnWidgetAction` (0x00208c28): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_session_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x00208c28 {
        return ov_screen_session_on_widget_action_resume(c);
    }
    guest(c, |c| 'l_208c28: loop {
        'b_208ee4: {
            'b_208ee0: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.g[S1][0] = c.g[A1][0];
                let t = c.g[S1][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                if t {
                    break 'b_208ee0;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V1][0] < 0x7u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        break 'b_208ee0;
                    } else {
                        'b_208ec8: {
                            'b_208dd8: {
                                'b_208da0: {
                                    'b_208d90: {
                                        'b_208d80: {
                                            c.cycles += 6;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x1870));
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                            let t = c.g[A0][0] as u32;
                                            match t {
                                                0x00208c28 => {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x00208c28);
                                                    }
                                                    continue 'l_208c28;
                                                }
                                                0x00208c6c => {
                                                    'b_208d1c: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xbc));
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V1][0] = 0x1;
                                                        if t {
                                                            break 'b_208d1c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x18));
                                                            if c.g[V0][0] != 0u64 {
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0xbc),
                                                                    0u64,
                                                                );
                                                                break 'b_208d90;
                                                            } else {
                                                                c.cycles += 38;
                                                                c.g[RA][0] = sx(0x00208c8c);
                                                                call_ptr(
                                                                    c, 0x0014dc80, 0x00208c8c,
                                                                )?;
                                                                c.g[A1][0] = 0xa;
                                                                c.g[RA][0] = sx(0x00208c98);
                                                                c.g[A0][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x0014dd58, 0x00208c98,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0xd8));
                                                                c.g[RA][0] = sx(0x00208ca4);
                                                                c.sw(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    c.g[V1][0],
                                                                );
                                                                call_ptr(
                                                                    c, 0x0028b180, 0x00208ca4,
                                                                )?;
                                                                c.g[RA][0] = sx(0x00208cac);
                                                                c.g[A0][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x0028fae0, 0x00208cac,
                                                                )?;
                                                                c.g[T0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x69c));
                                                                c.g[V1][0] = 0x530000;
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x66e8));
                                                                c.g[A2][0] = 0x440000;
                                                                c.g[T0][0] = sx((c.g[T0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x1));
                                                                c.g[A2][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(0x28f0));
                                                                c.g[V0][0] =
                                                                    sx((c.g[T0][0] as u32) << 2);
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V1][0] as u32,
                                                                    ));
                                                                c.g[T1][0] = 0x3;
                                                                c.g[V1][0] =
                                                                    c.lw(c.g[V0][0] as u32);
                                                                c.g[A3][0] = 0u64;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[A1][0] as u32)
                                                                        .wrapping_add(0x84));
                                                                c.g[V1][0] =
                                                                    sx((c.g[V1][0] as u32) << 2);
                                                                c.g[A1][0] = 0xf;
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A2][0] as u32,
                                                                    ));
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x9f4),
                                                                    c.g[T1][0],
                                                                );
                                                                c.g[V0][0] =
                                                                    c.lw(c.g[V1][0] as u32);
                                                                c.g[A2][0] = 0u64;
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x69c),
                                                                    c.g[T0][0],
                                                                );
                                                                c.g[RA][0] = sx(0x00208d04);
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x6a0),
                                                                    c.g[V0][0],
                                                                );
                                                                call_ptr(
                                                                    c, 0x00231250, 0x00208d04,
                                                                )?;
                                                                c.g[RA][0] = sx(0x00208d0c);
                                                                c.g[A0][0] = c.g[S0][0];
                                                                Callee { addr: 0x0020a088, f: ee::s05::OVScreenSession_DestroyIndicators }.call(c, 0x00208d0c)?;
                                                                c.g[RA][0] = sx(0x00208d14);
                                                                c.g[A0][0] = c.g[S0][0];
                                                                Callee { addr: 0x00208bb8, f: ee::s05::OVScreenSession_HideMapPic }.call(c, 0x00208d14)?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                if t {
                                                                    break 'b_208ee4;
                                                                } else {
                                                                    break 'b_208d1c;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 25;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0xbc),
                                                        c.g[V1][0],
                                                    );
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                                    c.g[RA][0] = sx(0x00208d30);
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0xd8),
                                                        c.g[V0][0],
                                                    );
                                                    Callee { addr: 0x00208f88, f: ee::s05::OVScreenSession_UpdateConfirmPopup }.call(c, 0x00208d30)?;
                                                    c.g[A0][0] = 0x470000;
                                                    c.g[RA][0] = sx(0x00208d3c);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x6c8));
                                                    call_ptr(c, 0x00317670, 0x00208d3c)?;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0xd4));
                                                    c.g[RA][0] = sx(0x00208d48);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, 0x0039d860, 0x00208d48)?;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                                    c.g[AT][0] = 0x3f800000;
                                                    c.mtc1(1, 0);
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                    c.swc1(0, c.g[SP][0] as u32);
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x4),
                                                        0u64,
                                                    );
                                                    c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x8),
                                                        0u64,
                                                    );
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0xc),
                                                        0u64,
                                                    );
                                                    c.g[V0][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[RA][0] = sx(0x00208d78);
                                                    c.g[A1][0] = c.g[SP][0];
                                                    call_ptr(c, t, 0x00208d78)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_208ee4;
                                                    } else {
                                                        break 'b_208d80;
                                                    }
                                                }
                                                0x00208d80 => {
                                                    break 'b_208d80;
                                                }
                                                0x00208dd8 => {
                                                    break 'b_208dd8;
                                                }
                                                0x00208ec8 => {
                                                    break 'b_208ec8;
                                                }
                                                0x00208ee0 => {
                                                    break 'b_208ee0;
                                                }
                                                _ => {
                                                    c.jump_out(t, 0x00208c28, 0x00208ef4);
                                                    return Ok(());
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_208da0;
                                        } else {
                                            c.cycles += 1;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), 0u64);
                                            break 'b_208d90;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x00208d98);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00208f88,
                                        f: ee::s05::OVScreenSession_UpdateConfirmPopup,
                                    }
                                    .call(c, 0x00208d98)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_208ee4;
                                    } else {
                                        break 'b_208da0;
                                    }
                                }
                                c.cycles += 14;
                                c.g[RA][0] = sx(0x00208da8);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x0020a088,
                                    f: ee::s05::OVScreenSession_DestroyIndicators,
                                }
                                .call(c, 0x00208da8)?;
                                c.g[RA][0] = sx(0x00208db0);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00208bb8,
                                    f: ee::s05::OVScreenSession_HideMapPic,
                                }
                                .call(c, 0x00208db0)?;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = 0x1;
                                c.g[RA][0] = sx(0x00208dc0);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f190, 0x00208dc0)?;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                c.g[V0][0] = 0xffffffffffffc0ff;
                                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                c.g[V1][0] = c.g[V1][0] | 0x780;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                                if t {
                                    break 'b_208ee0;
                                } else {
                                    break 'b_208dd8;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_208ee0;
                            } else {
                                'b_208e64: {
                                    c.cycles += 6;
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x00208df0);
                                    c.g[A2][0] = 0x1;
                                    Callee {
                                        addr: 0x002095e8,
                                        f: ee::s05::OVScreenSession_SetHighlightColor,
                                    }
                                    .call(c, 0x00208df0)?;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A1][0] = 0x470000;
                                    if t {
                                        break 'b_208e64;
                                    } else {
                                        c.cycles += 26;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00208e0c);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6e8));
                                        call_ptr(c, 0x0039e948, 0x00208e0c)?;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00208e1c);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x700));
                                        call_ptr(c, 0x0039e8b8, 0x00208e1c)?;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00208e2c);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x718));
                                        call_ptr(c, 0x0039e948, 0x00208e2c)?;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00208e3c);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x730));
                                        call_ptr(c, 0x0039e8b8, 0x00208e3c)?;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00208e4c);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x748));
                                        call_ptr(c, 0x0039e948, 0x00208e4c)?;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00208e5c);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x760));
                                        call_ptr(c, 0x0039e8b8, 0x00208e5c)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_208ee4;
                                        } else {
                                            break 'b_208e64;
                                        }
                                    }
                                }
                                c.cycles += 25;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00208e70);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x700));
                                call_ptr(c, 0x0039e948, 0x00208e70)?;
                                c.g[A1][0] = 0x470000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00208e80);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6e8));
                                call_ptr(c, 0x0039e8b8, 0x00208e80)?;
                                c.g[A1][0] = 0x470000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00208e90);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x730));
                                call_ptr(c, 0x0039e948, 0x00208e90)?;
                                c.g[A1][0] = 0x470000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00208ea0);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x718));
                                call_ptr(c, 0x0039e8b8, 0x00208ea0)?;
                                c.g[A1][0] = 0x470000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00208eb0);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x760));
                                call_ptr(c, 0x0039e948, 0x00208eb0)?;
                                c.g[A1][0] = 0x470000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00208ec0);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x748));
                                call_ptr(c, 0x0039e8b8, 0x00208ec0)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_208ee4;
                                } else {
                                    break 'b_208ec8;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_208ee0;
                        } else {
                            c.cycles += 3;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00208ee0);
                            c.g[A2][0] = 0x1;
                            Callee {
                                addr: 0x00209648,
                                f: ee::s05::OVScreenSession_SetUnlockedColor,
                            }
                            .call(c, 0x00208ee0)?;
                            break 'b_208ee0;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_208ee4;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// Native resume of `ov_screen_session_on_widget_action` (converted from the `entries` body): any entry
/// other than 0x00208c28, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn ov_screen_session_on_widget_action_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00208c28 => {
                    c.cycles += 7;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.g[S1][0] = c.g[A1][0];
                    let t = c.g[S1][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    if t {
                        pc = 0x00208ee0;
                        continue;
                    }
                    pc = 0x00208c44;
                    continue;
                }
                0x00208c44 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V1][0] < 0x7u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        pc = 0x00208ee0;
                        continue;
                    }
                    pc = 0x00208c54;
                    continue;
                }
                0x00208c54 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1870));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00208c6c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0x1;
                    if t {
                        pc = 0x00208d1c;
                        continue;
                    }
                    pc = 0x00208c78;
                    continue;
                }
                0x00208c78 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));

                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), 0u64);
                        pc = 0x00208d90;
                        continue;
                    }
                    pc = 0x00208c84;
                    continue;
                }
                0x00208c84 => {
                    c.cycles += 38;
                    c.g[RA][0] = sx(0x00208c8c);
                    call_ptr(c, 0x0014dc80, 0x00208c8c)?;
                    c.g[A1][0] = 0xa;
                    c.g[RA][0] = sx(0x00208c98);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dd58, 0x00208c98)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
                    c.g[RA][0] = sx(0x00208ca4);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    call_ptr(c, 0x0028b180, 0x00208ca4)?;
                    c.g[RA][0] = sx(0x00208cac);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0028fae0, 0x00208cac)?;
                    c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
                    c.g[A2][0] = 0x440000;
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28f0));
                    c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[T1][0] = 0x3;
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A3][0] = 0u64;
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A1][0] = 0xf;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[T1][0]);
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A2][0] = 0u64;
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[T0][0]);
                    c.g[RA][0] = sx(0x00208d04);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V0][0]);
                    call_ptr(c, 0x00231250, 0x00208d04)?;
                    c.g[RA][0] = sx(0x00208d0c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0020a088,
                        f: ee::s05::OVScreenSession_DestroyIndicators,
                    }
                    .call(c, 0x00208d0c)?;
                    c.g[RA][0] = sx(0x00208d14);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00208bb8,
                        f: ee::s05::OVScreenSession_HideMapPic,
                    }
                    .call(c, 0x00208d14)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00208ee4;
                        continue;
                    }
                    pc = 0x00208d1c;
                    continue;
                }
                0x00208d1c => {
                    c.cycles += 25;
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[RA][0] = sx(0x00208d30);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xd8), c.g[V0][0]);
                    Callee {
                        addr: 0x00208f88,
                        f: ee::s05::OVScreenSession_UpdateConfirmPopup,
                    }
                    .call(c, 0x00208d30)?;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x00208d3c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c8));
                    call_ptr(c, 0x00317670, 0x00208d3c)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd4));
                    c.g[RA][0] = sx(0x00208d48);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00208d48)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                    c.swc1(0, c.g[SP][0] as u32);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00208d78);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, t, 0x00208d78)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00208ee4;
                        continue;
                    }
                    pc = 0x00208d80;
                    continue;
                }
                0x00208d80 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x00208da0;
                        continue;
                    }
                    pc = 0x00208d8c;
                    continue;
                }
                0x00208d8c => {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), 0u64);
                    pc = 0x00208d90;
                    continue;
                }
                0x00208d90 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00208d98);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00208f88,
                        f: ee::s05::OVScreenSession_UpdateConfirmPopup,
                    }
                    .call(c, 0x00208d98)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00208ee4;
                        continue;
                    }
                    pc = 0x00208da0;
                    continue;
                }
                0x00208da0 => {
                    c.cycles += 14;
                    c.g[RA][0] = sx(0x00208da8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0020a088,
                        f: ee::s05::OVScreenSession_DestroyIndicators,
                    }
                    .call(c, 0x00208da8)?;
                    c.g[RA][0] = sx(0x00208db0);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00208bb8,
                        f: ee::s05::OVScreenSession_HideMapPic,
                    }
                    .call(c, 0x00208db0)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x00208dc0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039f190, 0x00208dc0)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = 0xffffffffffffc0ff;
                    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                    c.g[V1][0] = c.g[V1][0] | 0x780;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                    if t {
                        pc = 0x00208ee0;
                        continue;
                    }
                    pc = 0x00208dd8;
                    continue;
                }
                0x00208dd8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00208ee0;
                        continue;
                    }
                    pc = 0x00208de4;
                    continue;
                }
                0x00208de4 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00208df0);
                    c.g[A2][0] = 0x1;
                    Callee {
                        addr: 0x002095e8,
                        f: ee::s05::OVScreenSession_SetHighlightColor,
                    }
                    .call(c, 0x00208df0)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        pc = 0x00208e64;
                        continue;
                    }
                    pc = 0x00208dfc;
                    continue;
                }
                0x00208dfc => {
                    c.cycles += 26;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00208e0c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6e8));
                    call_ptr(c, 0x0039e948, 0x00208e0c)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00208e1c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x700));
                    call_ptr(c, 0x0039e8b8, 0x00208e1c)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00208e2c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x718));
                    call_ptr(c, 0x0039e948, 0x00208e2c)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00208e3c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x730));
                    call_ptr(c, 0x0039e8b8, 0x00208e3c)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00208e4c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x748));
                    call_ptr(c, 0x0039e948, 0x00208e4c)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00208e5c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x760));
                    call_ptr(c, 0x0039e8b8, 0x00208e5c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00208ee4;
                        continue;
                    }
                    pc = 0x00208e64;
                    continue;
                }
                0x00208e64 => {
                    c.cycles += 25;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00208e70);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x700));
                    call_ptr(c, 0x0039e948, 0x00208e70)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00208e80);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6e8));
                    call_ptr(c, 0x0039e8b8, 0x00208e80)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00208e90);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x730));
                    call_ptr(c, 0x0039e948, 0x00208e90)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00208ea0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x718));
                    call_ptr(c, 0x0039e8b8, 0x00208ea0)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00208eb0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x760));
                    call_ptr(c, 0x0039e948, 0x00208eb0)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00208ec0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x748));
                    call_ptr(c, 0x0039e8b8, 0x00208ec0)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x00208ee4;
                        continue;
                    }
                    pc = 0x00208ec8;
                    continue;
                }
                0x00208ec8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00208ee0;
                        continue;
                    }
                    pc = 0x00208ed4;
                    continue;
                }
                0x00208ed4 => {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00208ee0);
                    c.g[A2][0] = 0x1;
                    Callee {
                        addr: 0x00209648,
                        f: ee::s05::OVScreenSession_SetUnlockedColor,
                    }
                    .call(c, 0x00208ee0)?;
                    pc = 0x00208ee0;
                    continue;
                }
                0x00208ee0 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    pc = 0x00208ee4;
                    continue;
                }
                0x00208ee4 => {
                    c.cycles += 4;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00208c28, 0x00208ef4);
                    return Ok(());
                }
            }
        }
    });
}

/// `OVScreenSession_TransitionIn(self, on)` (0x002087f0, cOVState_MAP_onGainTransition): when `on` is set, labels the 8 map entries (enabled flags at `self+0x9c`), selects the current location in the menu and sets the two arrow widgets.
pub fn ov_screen_session_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        'b_208ac8: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.g[S6][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
            let t = c.g[A1][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            if t {
                break 'b_208ac8;
            } else {
                c.cycles += 21;
                c.g[S3][0] = 0u64;
                call_ptr(c, 0x0014dc80, 0x0020882c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00208838)?;
                c.g[S7][0] = sx((c.g[S6][0] as u32).wrapping_add(0x9c));
                c.g[S1][0] = c.g[V0][0];
                c.g[FP][0] = 0x470000;
                call_ptr(c, 0x0014dc80, 0x00208848)?;
                c.g[A1][0] = 0xa;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x00208854)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00144bc0, 0x00208860)?;
                c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001545f8, 0x0020886c)?;
                c.g[S5][0] = c.g[V0][0];
                c.g[V0][0] = 0x470000;
                'l_208878: loop {
                    'b_20899c: {
                        'b_208960: {
                            c.cycles += 13;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1808));
                            c.g[A2][0] = c.g[S3][0];
                            call_ptr(c, 0x00417828, 0x00208888)?;
                            c.g[A0][0] = c.g[SP][0];
                            call_ptr(c, 0x00317670, 0x00208890)?;
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xd4));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x0020889c)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S5][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                            if t {
                                break 'b_208960;
                            } else {
                                c.cycles += 18;
                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[V1][0] = 0x1;
                                c.g[V0][0] =
                                    sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                                c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = 0x470000;
                                c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
                                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1818));
                                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                call_ptr(c, 0x00317670, 0x002088dc)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                c.g[A0][0] = c.g[S0][0];
                                let t = c.g[V1][0] as u32;
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, t, 0x002088ec)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A2][0] = c.g[V0][0];
                                if t {
                                    break 'b_20899c;
                                } else {
                                    c.cycles += 7;
                                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                    c.g[V0][0] = 0x4a0000;
                                    c.g[A3][0] = c.g[S4][0];
                                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2558));
                                    call_ptr(c, 0x002c27c0, 0x00208908)?;
                                    let t = c.g[S2][0] == 0u64;
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_20899c;
                                    } else {
                                        'b_208950: {
                                            'b_208934: {
                                                'b_208924: {
                                                    c.cycles += 2;
                                                    let t = c.g[S3][0] != 0u64;
                                                    c.g[V0][0] =
                                                        sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                                    if t {
                                                        break 'b_208924;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = 0x470000;
                                                        let t = true;
                                                        c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x1828));
                                                        if t {
                                                            break 'b_208934;
                                                        } else {
                                                            break 'b_208924;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[S3][0] != c.g[V0][0];
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_208950;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[S5][0] == c.g[V0][0];
                                                    c.g[A0][0] =
                                                        sx((c.g[FP][0] as u32)
                                                            .wrapping_add(0x1838));
                                                    if t {
                                                        break 'b_208950;
                                                    } else {
                                                        break 'b_208934;
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            call_ptr(c, 0x00317670, 0x0020893c)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S2][0];
                                            call_ptr(c, 0x003a1000, 0x00208948)?;
                                            let t = true;
                                            c.sw(
                                                (c.g[S2][0] as u32).wrapping_add(0x18),
                                                c.g[S4][0],
                                            );
                                            if t {
                                                break 'b_20899c;
                                            } else {
                                                break 'b_208950;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x003a0e90, 0x00208958)?;
                                        let t = true;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[S4][0]);
                                        if t {
                                            break 'b_20899c;
                                        } else {
                                            break 'b_208960;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw(c.g[V0][0] as u32, 0u64);
                        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x00208984)?;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0020899c)?;
                        break 'b_20899c;
                    }
                    c.cycles += 4;
                    c.g[S3][0] = c.g[S4][0];
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 8i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00208878);
                        }
                        continue 'l_208878;
                    } else {
                        'b_208a3c: {
                            c.cycles += 11;
                            c.g[A0][0] = 0x470000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1850));
                            call_ptr(c, 0x00317670, 0x002089b8)?;
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xd4));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x002089c4)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_208a3c;
                            } else {
                                c.cycles += 25;
                                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                call_ptr(c, t, 0x002089f8)?;
                                c.lq(3, c.g[V0][0] as u32);
                                c.g[A0][0] = 0x4d0000;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
                                c.sq(3, c.g[SP][0] as u32);
                                c.lwc1(0, c.g[SP][0] as u32);
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0026b680, 0x00208a28)?;
                                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                c.g[A0][0] = c.g[S0][0];
                                if c.g[V0][0] == 0 {
                                    c.g[A1][0] = 0u64;
                                }
                                c.g[A1][0] = c.g[A1][0] & 0xff;
                                call_ptr(c, 0x0039b960, 0x00208a3c)?;
                                break 'b_208a3c;
                            }
                        }
                        'b_208a84: {
                            c.cycles += 11;
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x00209e78, 0x00208a44)?;
                            c.g[A0][0] = 0x470000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6b8));
                            call_ptr(c, 0x00317670, 0x00208a50)?;
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xd4));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00208a5c)?;
                            c.g[S2][0] = c.g[V0][0];
                            let t = c.g[S2][0] == 0u64;
                            c.g[A1][0] = 0x4d0000;
                            if t {
                                break 'b_208a84;
                            } else {
                                c.cycles += 7;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x18), 0u64);
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74a8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00208a84)?;
                                break 'b_208a84;
                            }
                        }
                        c.cycles += 9;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c8));
                        call_ptr(c, 0x00317670, 0x00208a90)?;
                        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xd4));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00208a9c)?;
                        c.g[S2][0] = c.g[V0][0];
                        let t = c.g[S2][0] == 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_208ac8;
                        } else {
                            c.cycles += 8;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                            c.g[A1][0] = 0x4d0000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74a8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00208ac8)?;
                            break 'b_208ac8;
                        }
                    }
                }
            }
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[V0][0] = 0u64;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        return Ok(());
    })
}

/// `void OVScreenSession_UpdateConfirmPopup(OVScreenSession* self)` (0x208f88): the map screen's
/// confirm popup. With `self->+0xbc` set it shows the popup widgets, fills the
/// text and money fields (`UStr_SprintfAsciiFmt_2`, `sprintf`, the string manager's
/// lookup through the world), hides the others and shows `0x4706c8`; otherwise it
/// hides the popup and toggles the menu and list widgets.
pub fn ov_screen_session_update_confirm_popup(c: &mut Cpu) {
    guest(c, |c| {
        'b_2092e4: {
            'b_20929c: {
                c.cycles += 24;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[A0][0] = 0x470000;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
                c.g[RA][0] = sx(0x00208fb0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa20));
                call_ptr(c, 0x00317670, 0x00208fb0)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                c.g[RA][0] = sx(0x00208fbc);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00208fbc)?;
                c.g[A0][0] = 0x470000;
                c.g[S1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00208fcc);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1850));
                call_ptr(c, 0x00317670, 0x00208fcc)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                c.g[RA][0] = sx(0x00208fd8);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00208fd8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xbc));
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_20929c;
                } else {
                    'b_209004: {
                        c.cycles += 5;
                        c.g[A1][0] = 0x470000;
                        c.g[RA][0] = sx(0x00208ff4);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa60));
                        call_ptr(c, 0x0039e948, 0x00208ff4)?;
                        let t = c.g[S1][0] == 0u64;
                        c.g[A0][0] = c.g[S1][0];
                        if t {
                            break 'b_209004;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x00209004);
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x0039b960, 0x00209004)?;
                            break 'b_209004;
                        }
                    }
                    'b_209048: {
                        c.cycles += 11;
                        c.g[A1][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2638));
                        c.g[RA][0] = sx(0x00209018);
                        c.g[S3][0] = 0u64;
                        call_ptr(c, 0x0039e948, 0x00209018)?;
                        c.g[A1][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x00209028);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2640));
                        call_ptr(c, 0x0039e8b8, 0x00209028)?;
                        let t = c.g[S0][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_209048;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00209044);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00209044)?;
                            c.g[S3][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x95));
                            break 'b_209048;
                        }
                    }
                    'b_20907c: {
                        c.cycles += 2;
                        let t = c.g[S1][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_20907c;
                        } else {
                            c.cycles += 11;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00209064);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00209064)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x0020907c);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0020907c)?;
                            break 'b_20907c;
                        }
                    }
                    'b_2090f4: {
                        c.cycles += 14;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1818));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x0020909c);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x00317670, 0x0020909c)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x002090ac);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, t, 0x002090ac)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A2][0] = c.g[V0][0];
                        if t {
                            break 'b_2090f4;
                        } else {
                            c.cycles += 13;
                            c.g[A1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x26f0));
                            c.g[RA][0] = sx(0x002090c8);
                            c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            call_ptr(c, 0x002c27c0, 0x002090c8)?;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x002090d4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1890));
                            call_ptr(c, 0x00317670, 0x002090d4)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                            c.g[RA][0] = sx(0x002090e0);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x002090e0)?;
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_2090f4;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x002090f4);
                                c.g[A1][0] = c.g[SP][0];
                                call_ptr(c, 0x003a0e90, 0x002090f4)?;
                                break 'b_2090f4;
                            }
                        }
                    }
                    'b_209174: {
                        c.cycles += 28;
                        c.g[RA][0] = sx(0x002090fc);
                        call_ptr(c, 0x0014dc80, 0x002090fc)?;
                        c.g[A1][0] = 0xb;
                        c.g[RA][0] = sx(0x00209108);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dd58, 0x00209108)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00209114);
                        c.g[A0][0] = 0u64;
                        call_ptr(c, 0x0014a080, 0x00209114)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00209124);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x00150928, 0x00209124)?;
                        c.g[V1][0] = 0x4a0000;
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x26f8));
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[S3][0];
                        c.g[RA][0] = sx(0x00209140);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00417828, 0x00209140)?;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x0020914c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18a0));
                        call_ptr(c, 0x00317670, 0x0020914c)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                        c.g[RA][0] = sx(0x00209158);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00209158)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A1][0] = c.g[S3][0];
                            break 'b_209174;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00209170);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x003a0c50, 0x00209170)?;
                            c.g[A1][0] = c.g[S3][0];
                            break 'b_209174;
                        }
                    }
                    'b_2091b4: {
                        c.cycles += 12;
                        c.g[A2][0] = 0x1f4;
                        c.g[RA][0] = sx(0x00209180);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00417828, 0x00209180)?;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x0020918c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18b8));
                        call_ptr(c, 0x00317670, 0x0020918c)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                        c.g[RA][0] = sx(0x00209198);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00209198)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A0][0] = 0x470000;
                            break 'b_2091b4;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x002091b0);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x003a0c50, 0x002091b0)?;
                            c.g[A0][0] = 0x470000;
                            break 'b_2091b4;
                        }
                    }
                    'b_2091f0: {
                        c.cycles += 8;
                        c.g[RA][0] = sx(0x002091bc);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18c8));
                        call_ptr(c, 0x00317670, 0x002091bc)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                        c.g[RA][0] = sx(0x002091c8);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x002091c8)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A1][0] = 0x470000;
                            break 'b_2091f0;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x002091e0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18d8));
                            call_ptr(c, 0x00317670, 0x002091e0)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x002091ec);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x002091ec)?;
                            c.g[A1][0] = 0x470000;
                            break 'b_2091f0;
                        }
                    }
                    'b_209254: {
                        c.cycles += 19;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x002091fc);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x700));
                        call_ptr(c, 0x0039e8b8, 0x002091fc)?;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x00209208);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6b8));
                        call_ptr(c, 0x00317670, 0x00209208)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                        c.g[RA][0] = sx(0x00209214);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00209214)?;
                        c.g[S1][0] = c.g[V0][0];
                        c.g[V0][0] = 0x470000;
                        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c8));
                        c.g[RA][0] = sx(0x00209228);
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x00317670, 0x00209228)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                        c.g[RA][0] = sx(0x00209234);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00209234)?;
                        let t = c.g[S1][0] == 0u64;
                        c.g[S0][0] = c.g[V0][0];
                        if t {
                            break 'b_209254;
                        } else {
                            c.cycles += 6;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x00209248);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xaa8));
                            call_ptr(c, 0x00317670, 0x00209248)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00209254);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x00209254)?;
                            break 'b_209254;
                        }
                    }
                    'b_209288: {
                        c.cycles += 2;
                        let t = c.g[S0][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_209288;
                        } else {
                            c.cycles += 11;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00209270);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00209270)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00209288);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00209288)?;
                            break 'b_209288;
                        }
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00209294);
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x0039e948, 0x00209294)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        break 'b_2092e4;
                    } else {
                        break 'b_20929c;
                    }
                }
            }
            'b_2092c4: {
                c.cycles += 5;
                c.g[A1][0] = 0x470000;
                c.g[RA][0] = sx(0x002092a8);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa60));
                call_ptr(c, 0x0039e8b8, 0x002092a8)?;
                let t = c.g[S0][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_2092c4;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002092c4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x002092c4)?;
                    break 'b_2092c4;
                }
            }
            'b_2092e0: {
                c.cycles += 2;
                let t = c.g[S1][0] == 0u64;
                c.g[A1][0] = 0u64;
                if t {
                    break 'b_2092e0;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002092e0);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x002092e0)?;
                    break 'b_2092e0;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            break 'b_2092e4;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenSession_CreatePlayerIndicator(OVScreenSession* self)` (0x2096a8): allocate
/// the player's marker icon, place it on the map image from the rider's position and the
/// track bounds (`BEMissionInt_GetSessionInfo`), scale it 17x17, give it its texture, hide
/// it and append it to the screen's object list.
pub fn ov_screen_session_create_player_indicator(c: &mut Cpu) {
    guest(c, |c| {
        'b_209780: {
            c.cycles += 54;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            c.g[RA][0] = sx(0x002096c8);
            c.g[S3][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x002096c8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002096d4);
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0014dd58, 0x002096d4)?;
            c.g[RA][0] = sx(0x002096dc);
            c.g[S2][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x002096dc)?;
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x002096e8);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014dd58, 0x002096e8)?;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xd0));
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x4b));
            c.ldr(2, (c.g[V1][0] as u32).wrapping_add(0x44));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[SP][0] as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00209730);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            call_ptr(c, t, 0x00209730)?;
            c.lq(3, c.g[V0][0] as u32);
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00209744);
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00144bc0, 0x00209744)?;
            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00209754);
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x00154678, 0x00209754)?;
            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x3c));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
            c.sub_s(1, 1, 2);
            c.sub_s(0, 0, 2);
            c.mtc1(0, 5);
            c.div_s(4, 1, 0);
            c.c_lt_s(4, 5);
            if c.fcc() {
                c.neg_s(4, 4);
                break 'b_209780;
            } else {
                break 'b_209780;
            }
        }
        'b_2097bc: {
            c.cycles += 15;
            c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
            c.sub_s(2, 2, 3);
            c.g[AT][0] = 0x43b20000;
            c.mtc1(1, 0);
            c.sub_s(1, 1, 3);
            c.mul_s(0, 4, 0);
            c.div_s(2, 2, 1);
            c.cvt_w_s(1, 0);
            c.mfc1(17, 1);
            c.c_lt_s(2, 5);
            if c.fcc() {
                c.neg_s(2, 2);
                break 'b_2097bc;
            } else {
                break 'b_2097bc;
            }
        }
        'b_209850: {
            'b_209838: {
                c.cycles += 27;
                c.g[AT][0] = 0x43850000;
                c.mtc1(1, 0);
                c.g[A1][0] = 0x470000;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x18f0));
                c.mul_s(0, 2, 0);
                c.g[A0][0] = 0x88;
                c.cvt_w_s(1, 0);
                c.mfc1(18, 1);
                c.g[RA][0] = sx(0x002097e8);
                c.g[A2][0] = 0x20000000;
                call_ptr(c, 0x00317d70, 0x002097e8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xd4));
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x002097fc);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039fb30, 0x002097fc)?;
                c.g[V1][0] = 0x490000;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4aa8));
                c.g[RA][0] = sx(0x00209810);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                Callee {
                    addr: 0x0021ccc8,
                    f: ee::s05::UIIconFlags_ctor,
                }
                .call(c, 0x00209810)?;
                c.g[V0][0] = 0xffffffffffffffff;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xfc));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                c.sw((c.g[S3][0] as u32).wrapping_add(0xe0), c.g[S0][0]);
                let t = (c.g[A0][0] as i64) < 0;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0u64);
                if t {
                    break 'b_209838;
                } else {
                    c.cycles += 4;
                    c.mtc1(4, 1);
                    c.cvt_s_w(1, 1);
                    let t = true;

                    if t {
                        break 'b_209850;
                    } else {
                        break 'b_209838;
                    }
                }
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A0][0] as u32) >> 1);
            c.g[V0][0] = c.g[A0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            break 'b_209850;
        }
        'b_209898: {
            'b_209874: {
                c.cycles += 5;
                c.g[AT][0] = 0x40a00000;
                c.mtc1(1, 0);
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x61));
                let t = (c.g[V0][0] as i64) < 0;
                c.sub_s(3, 1, 0);
                if t {
                    break 'b_209874;
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 2);
                    c.cvt_s_w(2, 2);
                    let t = true;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                    if t {
                        break 'b_209898;
                    } else {
                        break 'b_209874;
                    }
                }
            }
            c.cycles += 9;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 2);
            c.cvt_s_w(2, 2);
            c.add_s(2, 2, 2);
            c.g[AT][0] = 0x40a00000;
            c.mtc1(1, 0);
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
            break 'b_209898;
        }
        c.cycles += 53;
        c.sub_s(0, 2, 0);
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x57));
        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
        c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
        c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
        c.g[AT][0] = 0x41880000;
        c.mtc1(1, 0);
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa8));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x58), 0u64);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xac));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x002098f4);
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        call_ptr(c, t, 0x002098f4)?;
        c.g[A0][0] = 0x470000;
        c.g[RA][0] = sx(0x00209900);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1908));
        call_ptr(c, 0x00317670, 0x00209900)?;
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
        c.g[RA][0] = sx(0x0020990c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x00399730, 0x0020990c)?;
        c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
        c.g[A1][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00209928);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00209928)?;
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
        c.g[V1][0] = 0xffffffffffffe0ff;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0xd00;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xd4));
        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
        c.g[RA][0] = sx(0x00209950);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb4));
        call_ptr(c, 0x00397910, 0x00209950)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenSession_CreateMapIndicators(OVScreenSession* self)` (0x209970): create one
/// marker icon per path marker of the track (`PathData_HasMarker`/`FindMarker`), each
/// placed on the map image from the track bounds, plus the second set of markers, and
/// append them to the screen's object list.
pub fn ov_screen_session_create_map_indicators(c: &mut Cpu) {
    guest(c, |c| {
        'b_209e40: {
            c.cycles += 40;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x120));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S5][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.g[S7][0] = 0u64;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x118));
            c.g[RA][0] = sx(0x002099b0);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x110));
            call_ptr(c, 0x0014dc80, 0x002099b0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002099bc);
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0014dd58, 0x002099bc)?;
            c.g[RA][0] = sx(0x002099c4);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x60), c.g[V0][0]);
            call_ptr(c, 0x0014dc80, 0x002099c4)?;
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x002099d0);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014dd58, 0x002099d0)?;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x64), c.g[V0][0]);
            c.g[RA][0] = sx(0x002099dc);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x00144bc0, 0x002099dc)?;
            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
            c.g[RA][0] = sx(0x002099e8);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
            call_ptr(c, 0x001545f8, 0x002099e8)?;
            c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xd0));
            c.sw((c.g[S5][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
            c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x4b));
            c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x44));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(4, c.g[SP][0] as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
            let t = (c.g[V0][0] as i64) <= 0;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            if t {
                break 'b_209e40;
            } else {
                c.cycles += 6;
                c.g[AT][0] = 0x40a00000;
                c.mtc1(1, 21);
                c.g[AT][0] = 0x41900000;
                c.mtc1(1, 20);
                c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                'l_209a28: loop {
                    'b_209e28: {
                        c.cycles += 7;
                        c.g[A0][0] = 0x4d0000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x68), c.g[A2][0]);
                        c.g[RA][0] = sx(0x00209a3c);
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x0026b598, 0x00209a3c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x68));
                        if t {
                            break 'b_209e28;
                        } else {
                            'b_209ab4: {
                                c.cycles += 28;
                                c.g[A0][0] = 0x4d0000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
                                c.g[RA][0] = sx(0x00209a54);
                                c.g[A1][0] = 0x2;
                                call_ptr(c, 0x0026b5e0, 0x00209a54)?;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x64));
                                c.ldl(8, (c.g[V0][0] as u32).wrapping_add(0xf));
                                c.ldr(8, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x17));
                                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                c.g[RA][0] = sx(0x00209a78);
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                call_ptr(c, 0x00144bc0, 0x00209a78)?;
                                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00209a88);
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
                                call_ptr(c, 0x00154678, 0x00209a88)?;
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x3c));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                                c.sub_s(1, 1, 2);
                                c.sub_s(0, 0, 2);
                                c.mtc1(0, 5);
                                c.div_s(4, 1, 0);
                                c.c_lt_s(4, 5);
                                if c.fcc() {
                                    c.neg_s(4, 4);
                                    break 'b_209ab4;
                                } else {
                                    break 'b_209ab4;
                                }
                            }
                            'b_209af0: {
                                c.cycles += 15;
                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                                c.sub_s(2, 2, 3);
                                c.g[AT][0] = 0x43b20000;
                                c.mtc1(1, 0);
                                c.sub_s(1, 1, 3);
                                c.mul_s(0, 4, 0);
                                c.div_s(2, 2, 1);
                                c.cvt_w_s(1, 0);
                                c.mfc1(20, 1);
                                c.c_lt_s(2, 5);
                                if c.fcc() {
                                    c.neg_s(2, 2);
                                    break 'b_209af0;
                                } else {
                                    break 'b_209af0;
                                }
                            }
                            'b_209b9c: {
                                'b_209b80: {
                                    c.cycles += 32;
                                    c.g[AT][0] = 0x43850000;
                                    c.mtc1(1, 0);
                                    c.g[A1][0] = 0x470000;
                                    c.g[A3][0] = 0u64;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1918));
                                    c.mul_s(0, 2, 0);
                                    c.g[A0][0] = 0x88;
                                    c.g[A2][0] = 0x20000000;
                                    c.g[S1][0] = 0x490000;
                                    c.g[S2][0] = sx((c.g[S7][0] as u32) << 2);
                                    c.g[S3][0] = sx((c.g[S5][0] as u32).wrapping_add(0xe4));
                                    c.cvt_w_s(1, 0);
                                    c.mfc1(22, 1);
                                    c.g[RA][0] = sx(0x00209b2c);
                                    c.g[FP][0] = c.g[S2][0];
                                    call_ptr(c, 0x00317d70, 0x00209b2c)?;
                                    c.g[S2][0] =
                                        sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xd4));
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x00209b44);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x0039fb30, 0x00209b44)?;
                                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0xfc));
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4aa8));
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
                                    c.g[RA][0] = sx(0x00209b58);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                                    Callee {
                                        addr: 0x0021ccc8,
                                        f: ee::s05::UIIconFlags_ctor,
                                    }
                                    .call(c, 0x00209b58)?;
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    c.g[A0][0] = c.g[S3][0];
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                                    c.sw(c.g[S2][0] as u32, c.g[S0][0]);
                                    let t = (c.g[S4][0] as i64) < 0;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0u64);
                                    if t {
                                        break 'b_209b80;
                                    } else {
                                        c.cycles += 4;
                                        c.mtc1(20, 0);
                                        c.cvt_s_w(0, 0);
                                        let t = true;
                                        c.sub_s(2, 0, 21);
                                        if t {
                                            break 'b_209b9c;
                                        } else {
                                            break 'b_209b80;
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.g[V0][0] = c.g[S4][0] & 0x1;
                                c.g[V1][0] = sx((c.g[S4][0] as u32) >> 1);
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 0, 0);
                                c.sub_s(2, 0, 21);
                                break 'b_209b9c;
                            }
                            'b_209bd0: {
                                'b_209bb8: {
                                    c.cycles += 3;
                                    c.g[S3][0] = sx((c.g[S6][0] as u32).wrapping_add(0x61));
                                    let t = (c.g[S3][0] as i64) < 0;
                                    c.g[V0][0] = c.g[S3][0] & 0x1;
                                    if t {
                                        break 'b_209bb8;
                                    } else {
                                        c.cycles += 4;
                                        c.mtc1(19, 0);
                                        c.cvt_s_w(0, 0);
                                        let t = true;
                                        c.sub_s(0, 0, 21);
                                        if t {
                                            break 'b_209bd0;
                                        } else {
                                            break 'b_209bb8;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[V1][0] = sx((c.g[S3][0] as u32) >> 1);
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 0, 0);
                                c.sub_s(0, 0, 21);
                                break 'b_209bd0;
                            }
                            'b_209d20: {
                                'b_209d04: {
                                    c.cycles += 73;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                                    c.g[S0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[FP][0] as u32));
                                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                                    c.g[S1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x104));
                                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
                                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x57));
                                    c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
                                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x4b));
                                    c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x44));
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
                                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa8));
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x54));
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x58), 0u64);
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xac));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00209c30);
                                    c.g[S2][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[FP][0] as u32));
                                    call_ptr(c, t, 0x00209c30)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x00209c3c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1930));
                                    call_ptr(c, 0x00317670, 0x00209c3c)?;
                                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                    c.g[RA][0] = sx(0x00209c48);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x00399730, 0x00209c48)?;
                                    c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                    c.g[A1][0] = 0u64;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00209c64);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00209c64)?;
                                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                    c.g[V1][0] = 0xffffffffffffe0ff;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                    c.g[V0][0] = c.g[V0][0] | 0xb00;
                                    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xd4));
                                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                    c.g[RA][0] = sx(0x00209c8c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb4));
                                    call_ptr(c, 0x00397910, 0x00209c8c)?;
                                    c.g[A1][0] = 0x470000;
                                    c.g[A3][0] = 0u64;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1940));
                                    c.g[A0][0] = 0x88;
                                    c.g[RA][0] = sx(0x00209ca4);
                                    c.g[A2][0] = 0x20000000;
                                    call_ptr(c, 0x00317d70, 0x00209ca4)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xd4));
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x00209cb8);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x0039fb30, 0x00209cb8)?;
                                    c.g[T0][0] = 0x490000;
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
                                    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4aa8));
                                    c.g[RA][0] = sx(0x00209ccc);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                    Callee {
                                        addr: 0x0021ccc8,
                                        f: ee::s05::UIIconFlags_ctor,
                                    }
                                    .call(c, 0x00209ccc)?;
                                    c.g[V0][0] = 0xc;
                                    c.sw(c.g[S2][0] as u32, c.g[S0][0]);
                                    c.mult(0, 4, 23, 2);
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V1][0]);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0u64);
                                    c.g[V0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                                    let t = (c.g[S4][0] as i64) < 0;
                                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x124));
                                    if t {
                                        break 'b_209d04;
                                    } else {
                                        c.cycles += 4;
                                        c.mtc1(20, 0);
                                        c.cvt_s_w(0, 0);
                                        let t = true;
                                        c.sub_s(2, 0, 21);
                                        if t {
                                            break 'b_209d20;
                                        } else {
                                            break 'b_209d04;
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.g[V1][0] = sx((c.g[S4][0] as u32) >> 1);
                                c.g[V0][0] = c.g[S4][0] & 0x1;
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 0, 0);
                                c.sub_s(2, 0, 21);
                                break 'b_209d20;
                            }
                            'b_209d58: {
                                'b_209d38: {
                                    c.cycles += 2;
                                    let t = (c.g[S3][0] as i64) < 0;
                                    c.g[V1][0] = 0xc;
                                    if t {
                                        break 'b_209d38;
                                    } else {
                                        c.cycles += 4;
                                        c.mtc1(19, 0);
                                        c.cvt_s_w(0, 0);
                                        let t = true;
                                        c.sub_s(1, 0, 21);
                                        if t {
                                            break 'b_209d58;
                                        } else {
                                            break 'b_209d38;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[V1][0] = sx((c.g[S3][0] as u32) >> 1);
                                c.g[V0][0] = c.g[S3][0] & 0x1;
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 0, 0);
                                c.g[V1][0] = 0xc;
                                c.sub_s(1, 0, 21);
                                break 'b_209d58;
                            }
                            c.cycles += 52;
                            c.mult(0, 2, 23, 3);
                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[FP][0] as u32));
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x58));
                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                            c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x57));
                            c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
                            c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.sdr(6, c.g[A0][0] as u32);
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                            c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x12b));
                            c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x124));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x12c));
                            c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x4b));
                            c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x44));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa8));
                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x54));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x58), 0u64);
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xac));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x00209dcc);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00209dcc)?;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x00209dd8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1958));
                            call_ptr(c, 0x00317670, 0x00209dd8)?;
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            c.g[RA][0] = sx(0x00209de4);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x00399730, 0x00209de4)?;
                            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00209e00);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00209e00)?;
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            c.g[V1][0] = 0xffffffffffffe0ff;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            c.g[V0][0] = c.g[V0][0] | 0xc00;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xd4));
                            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                            c.g[RA][0] = sx(0x00209e28);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb4));
                            call_ptr(c, 0x00397910, 0x00209e28)?;
                            break 'b_209e28;
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc0));
                    c.g[S7][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x68));
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00209a28);
                        }
                        continue 'l_209a28;
                    } else {
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                        break 'b_209e40;
                    }
                }
            }
        }
        c.cycles += 13;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x118));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x110));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
        c.pc = t;
        return Ok(());
    });
}

/// `obj->vtbl` method call of the generated bodies: `V0 = *(obj + 8)`,
/// `A0 = obj + *(short *)(V0 + 0x48)`, then `jalr *(V0 + 0x4c)` returning to
/// `ret`. The caller charges its own cycles and sets `A1`.
pub(crate) fn call_method(c: &mut Cpu, obj: usize, ret: u32) -> Flow {
    c.g[V0][0] = Addr((c.g[obj][0] as u32).wrapping_add(8)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[obj][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `OVScreenSession_Init` (0x2086a8): builds the map session's screen state
/// (the map picture, the player and map indicators) on the session `s1`.
/// Stores the picture handle at +0xd4 and the marker handle at +0xd0, clears
/// the list slots at +0xe0..+0x1a4 and runs the four setup calls at the end.
pub fn ovscreensession_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);

        // Block A: the picture handle from the table; a non-null handle gets a
        // call through the table.
        c.g[A0][0] = sx(0x0047_0000u32.wrapping_add(0x17f8));
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0020_86cc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0020_86e0)?;
        let t = c.g[V0][0] == 0;
        Addr((c.g[S1][0] as u32).wrapping_add(0xd4)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x0020_86f8);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0020_86f8)?;
        }

        // Block B: the map-image setup (`cOVStateOverlay_Init`, then the table).
        c.cycles += 22;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0020_a380,
            f: ee::s05::f_0020a380,
        }
        .call(c, 0x0020_8700)?;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x11c)).lw(c);
        call_ptr(c, 0x001a_3bf0, 0x0020_870c)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.g[A0][0] = 0x004a_0000;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5508));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x25a8));
        Addr((c.g[S1][0] as u32).wrapping_add(0xdc)).sw(c, 0);
        Addr((c.g[S1][0] as u32).wrapping_add(0xbc)).sw(c, 0);
        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x1cc));
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x1d0));
        Addr((c.g[S1][0] as u32).wrapping_add(0xc4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x0020_8738)?;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xd4)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0020_8744)?;
        c.g[A2][0] = c.g[V0][0];
        let t = c.g[A2][0] == 0;
        Addr((c.g[S1][0] as u32).wrapping_add(0xd0)).sw(c, c.g[A2][0]);
        if !t {
            c.cycles += 6;
            c.g[A1][0] = 0;
            call_method(c, A2, 0x0020_8768)?;
        }

        // The list slots: seven pointer-sized cells cleared in lock step.
        c.cycles += 8;
        Addr((c.g[S1][0] as u32).wrapping_add(0xc0)).sw(c, 0);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1a4));
        Addr((c.g[S1][0] as u32).wrapping_add(0xe0)).sw(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x184));
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x104));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xe4));
        c.g[A2][0] = 7;
        loop {
            c.cycles += 10;
            Addr(c.g[V0][0] as u32).sw(c, 0);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
            Addr(c.g[V1][0] as u32).sw(c, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            Addr(c.g[A0][0] as u32).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            Addr(c.g[A1][0] as u32).sw(c, 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            let t = (c.g[A2][0] as i64) >= 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            if !t {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0020_8788);
            }
        }

        // Done: the four setup calls, each on the session.
        c.cycles += 13;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0020_8f10,
            f: ee::s05::f_00208f10,
        }
        .call(c, 0x0020_87b8)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0020_8f88,
            f: ee::s05::f_00208f88,
        }
        .call(c, 0x0020_87c0)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0020_96a8,
            f: ee::s05::f_002096a8,
        }
        .call(c, 0x0020_87c8)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0020_9970,
            f: ee::s05::f_00209970,
        }
        .call(c, 0x0020_87d0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenSession_HideIndicators` (0x209fd0): calls the hide method of the
/// session's child at +0xe0, then the hide method of each entry of the list
/// at +0xe4 (count at +0xc0; entries from +0xe4 and +0x104).
pub fn ovscreensession_hide_indicators(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        c.g[A2][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xe0)).lw(c);
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !t {
            c.cycles += 5;
            call_method(c, A2, 0x0020_a008)?;
        }

        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc0)).lw(c);
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !t {
            c.cycles += 1;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x104));
            loop {
                // The entry before the cursor (+0xe4 at the first step).
                c.cycles += 3;
                c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x20)).lw(c);
                let t = c.g[A2][0] == 0;
                c.g[A1][0] = 0;
                if !t {
                    c.cycles += 5;
                    call_method(c, A2, 0x0020_a038)?;
                }
                // The entry at the cursor.
                c.cycles += 3;
                c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
                let t = c.g[A2][0] == 0;
                c.g[A1][0] = 0;
                if !t {
                    c.cycles += 5;
                    call_method(c, A2, 0x0020_a058)?;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc0)).lw(c);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0020_a018);
                }
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

/// `OVScreenFinalStandings_RecvChatMessage` (0x20a4e0): receives a chat line
/// on the final standings screen. While the chat's sound/popup state is set
/// (global 0x534b30) it hides the old popup, builds the new one from the
/// message text (+0x58) and the widget at +0x40, and lays it out.
pub fn ovscreen_final_standings_recv_chat_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save(c, S0, 0x80);
        c.g[A1][0] = 7;
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S6, 0x20);
        frame.save(c, S2, 0x60);
        c.g[S6][0] = 0;
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x78)).lw(c);
        call_ptr(c, 0x0014_dd58, 0x0020_a51c)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4b30)).lw(c);
        let t = c.g[V0][0] == 0;
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4b30));

        'exit: {
            'b67c: {
                if t {
                    break 'b67c;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                let t = c.g[V0][0] == 0;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                if t {
                    break 'b67c;
                }
                c.cycles += 2;
                let t = c.g[A0][0] == 0;
                c.g[V0][0] = c.g[S6][0];
                if t {
                    break 'exit;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                let t = c.g[V0][0] == 0;
                c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                if t {
                    break 'b67c;
                }
                c.cycles += 5;
                c.g[A2][0] = 0x41;
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x0025_8160, 0x0020_a558)?;
                let t = c.g[V0][0] == 0;
                c.g[A0][0] = 0x0047_0000;
                if t {
                    break 'b67c;
                }

                // The old popup is hidden (0x46e660 table entry) and its
                // widget at +0x40 is re-laid out when present.
                'b594: {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x19a0));
                    call_ptr(c, 0x0031_7670, 0x0020_a568)?;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x0020_a574)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0;
                    c.g[A1][0] = 0;
                    if t {
                        break 'b594;
                    }
                    c.cycles += 5;
                    call_method(c, A2, 0x0020_a594)?;
                }

                // The chat line's popup is hidden the same way (0x46e7f8).
                'b5d4: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x0047_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1808));
                    call_ptr(c, 0x0031_7670, 0x0020_a5a0)?;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x0020_a5ac)?;
                    c.g[A2][0] = c.g[V0][0];
                    if c.g[A2][0] == 0 {
                        c.g[A0][0] = 0x0047_0000;
                        break 'b5d4;
                    }
                    c.cycles += 7;
                    c.g[A1][0] = 0;
                    call_method(c, A2, 0x0020_a5d0)?;
                    c.g[A0][0] = 0x0047_0000;
                }

                // The new message: state flag, the text table, the layout.
                c.cycles += 42;
                c.g[S6][0] = 1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x17f8));
                call_ptr(c, 0x0031_7670, 0x0020_a5e0)?;
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0020_a5ec)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x498));
                c.g[A0][0] = c.g[SP][0];
                c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc8)).lhu(c);
                c.g[S2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc4)).lhu(c);
                c.g[S3][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc6)).lhu(c);
                c.g[S4][0] = (c.g[S1][0] < 1) as u64;
                call_ptr(c, 0x0031_82d8, 0x0020_a60c)?;
                c.g[A1][0] = 0x004a_0000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2710));
                call_ptr(c, 0x0031_8910, 0x0020_a61c)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x0031_8910, 0x0020_a628)?;
                c.g[A1][0] = Addr(c.g[SP][0] as u32).lw(c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003a_19f8, 0x0020_a634)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[T0][0] = c.g[S2][0];
                c.g[T1][0] = c.g[S3][0];
                c.g[T2][0] = c.g[S4][0];
                c.g[T3][0] = 1;
                c.g[A1][0] = 0x28;
                c.g[A2][0] = 4;
                call_ptr(c, 0x001a_83d8, 0x0020_a658)?;
                c.g[A1][0] = 1;
                call_method(c, S0, 0x0020_a670)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 2;
                call_ptr(c, 0x0031_8260, 0x0020_a67c)?;
                break 'b67c;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S6][0];
        }

        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `func_002212C8` (0x2212c8), the session overlay's one-argument constructor:
/// the base constructor with engine 0, the session vtables and fields, and a
/// countdown loop of eight steps (`a1` from 7 down to -1) with a back-edge
/// event check. Returns `this`.
pub fn ov_screen_session_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OV_STATE_BASE_CTOR, 0x0022_12e0)?;
        let this = Addr(c.g[S0][0] as u32);
        this.off(0x98).sw(c, 0);
        this.off(0x48).sw(c, 0);
        let vtable = ov_image_sym(0x4e08);
        this.off(0x8).sw(c, vtable);
        c.g[A0][0] = 0x004a_0000;
        this.off(0x4c).sw(c, 0);
        this.off(0x54).sw(c, 0);
        let session_vtable = ov_image_sym(0x3320);
        c.g[V0][0] = session_vtable;
        c.g[A2][0] = u64::MAX;
        c.g[A1][0] = 0x7;
        let halfword = Addr((c.g[A0][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[V1][0] = halfword;
        this.off(0x8).sw(c, session_vtable);
        this.off(0x58).sh(c, halfword);
        loop {
            c.cycles += 7;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            let more = c.g[A1][0] != c.g[A2][0];
            c.g[V0][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0022_1320);
                }
                continue;
            }
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    });
}

/// `cOVState_MAP::cOVState_MAP(this, engine, overlayId)` (0x221350), the
/// session overlay's three-argument constructor: the same shape as the
/// one-argument form with the overlay id from a2 and `this` kept in s1.
pub fn ov_screen_session_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        call_ptr(c, OV_STATE_BASE_CTOR, 0x0022_136c)?;
        let this = Addr(c.g[S1][0] as u32);
        let overlay_id = c.g[S0][0];
        this.off(0x98).sw(c, overlay_id);
        this.off(0x48).sw(c, 0);
        let vtable = ov_image_sym(0x4e08);
        this.off(0x8).sw(c, vtable);
        c.g[A0][0] = 0x004a_0000;
        this.off(0x4c).sw(c, 0);
        this.off(0x54).sw(c, 0);
        let session_vtable = ov_image_sym(0x3320);
        c.g[V0][0] = session_vtable;
        c.g[A2][0] = u64::MAX;
        c.g[A1][0] = 0x7;
        let halfword = Addr((c.g[A0][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[V1][0] = halfword;
        this.off(0x8).sw(c, session_vtable);
        this.off(0x58).sh(c, halfword);
        loop {
            c.cycles += 7;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            let more = c.g[A1][0] != c.g[A2][0];
            c.g[V0][0] = c.g[S1][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0022_13a8);
                }
                continue;
            }
            c.cycles += 5;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    });
}

/// `OVScreenSession_dtor` (0x2213d8): the destructor of the session overlay
/// state. Sets the base vtable and runs `UIState_dtor`.
pub fn ov_screen_session_dtor(c: &mut Cpu) {
    guest(c, |c| ov_state_dtor_body(c, 0x0022_13f4));
}

/// `OVScreenSession_OnWidgetCreated` (0x0020_87e8): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_session_on_widget_created_002087e8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVScreenSession_v13` (0x0022_1400): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_session_v13_00221400(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc767_install_fakes;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    use crate::difftest::RA as TEST_RA;

    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_method;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_3 as put_str;

    pub(crate) use crate::game::fe::feasyncfile::tests::screen_with;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::fepopupmisc::tests::setup;
    pub(crate) use crate::game::fe::festatestore::tests::world;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2 as globals;

    pub(crate) use crate::game::fe::ovstatepause::tests::WORLD;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_3;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn ov_screen_session_shutdown_matches() {
        check(
            ssx3_ee::s05::f_00208b78__generated,
            ov_screen_session_shutdown,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x31, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0xc4, (seed % 8) as u32);
                c.bus.write32(WORLD + 0x11c, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn ov_state_overlay_on_widget_action_matches() {
        check(
            ssx3_ee::s05::f_0020a6a8__generated,
            ov_state_overlay_on_widget_action,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x20a6a8, 0);
                be_library(c);
                let screen = DATA + 0x8000;
                let vt = DATA + 0x9000;
                match seed % 3 {
                    0 => {}
                    1 => {
                        world(c);
                    }
                    _ => {
                        world(c);
                        c.bus.write32(DATA + 0x6000 + 0x200, screen);
                        c.bus.write32(screen + 0x0c, vt);
                        fake_method(c, vt, 0x48);
                    }
                }
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x1400);
                c.g[A2][0] = 3;
            },
        );
    }

    /// The session colour setters: an object whose vtable slot 0x58 takes the
    /// colour, and both the on (custom colour) and off (default colour) paths.
    pub(crate) fn color_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = setup(c, seed, salt);
        let obj = DATA + 0x900;
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = obj as u64;
        c.g[A2][0] = if seed % 2 == 0 {
            0
        } else {
            r.next_u32() as u64
        };
        let vt = vtable(c, &mut r, &[0x58]);
        c.bus.write32(obj + 8, vt);
    }

    #[test]
    fn ov_screen_session_set_highlight_color_matches() {
        check(
            ssx3_ee::s05::f_002095e8__generated,
            ov_screen_session_set_highlight_color,
            120,
            &|c, seed| color_setup(c, seed, 0x2095e8),
        );
    }

    #[test]
    fn ov_screen_session_set_unlocked_color_matches() {
        check(
            ssx3_ee::s05::f_00209648__generated,
            ov_screen_session_set_unlocked_color,
            120,
            &|c, seed| color_setup(c, seed, 0x209648),
        );
    }

    #[test]
    fn ov_screen_session_on_widget_notify_matches() {
        check(
            ssx3_ee::s05::f_00208ef8__generated,
            ov_screen_session_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x208ef8, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
    }

    #[test]
    fn ov_screen_session_hide_map_pic_matches() {
        check(
            ee::s05::f_00208bb8__generated,
            ov_screen_session_hide_map_pic,
            60,
            &|c, seed| {
                machine(c, seed, 0x208b_b8, 0);
                c.g[A0][0] = sx(DATA);
                let a = if seed % 3 == 0 { 0 } else { DATA + 0x1000 };
                let b = if seed % 2 == 0 { 0 } else { DATA + 0x2000 };
                c.bus.write32(DATA + 0xd0, a);
                c.bus.write32(DATA + 0xe0, b);
                if a != 0 {
                    put_vtable(c, a, &[0x48]);
                }
                if b != 0 {
                    put_vtable(c, b, &[0x48]);
                }
            },
        );
    }

    #[test]
    fn ov_screen_session_post_input_matches() {
        check(
            ee::s05::f_00209300__generated,
            ov_screen_session_post_input,
            60,
            &|c, seed| {
                machine(c, seed, 0x2093_00, 0);
                let obj = DATA + 0x300;
                let vt = DATA + 0x400;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(obj);
                c.bus.write32(obj + 8, vt);
                put_vtable(c, vt, &[0x88, 0x90]);
                // The two status methods are answered by
                // UIListNode_IsSentinel (a real function returning 0/1), so
                // the zero-result branches are reachable.
                c.bus.write32(vt + 0x88, 0);
                c.bus.write16(vt + 0x88, 0);
                c.bus.write32(vt + 0x8c, 0x0039_7718);
                c.bus.write32(vt + 0x94, 0x0039_7718);
                if seed % 3 == 0 {
                    c.bus.write32(obj + 4, obj);
                } else {
                    c.bus.write32(obj, 0);
                    c.bus.write32(obj + 4, 0);
                }
                if seed % 3 == 2 {
                    c.bus.write16(vt + 0x90, 0x40);
                    let n = obj + 0x40;
                    c.bus.write32(n, n);
                    c.bus.write32(n + 4, n);
                }
                c.bus.write32(DATA + 0xe0, 0);
                c.bus.write32(DATA + 0xd4, 0);
                c.bus.write32(DATA + 0xc0, 0);
            },
        );
    }

    /// Run a guest helper during setup; returns its `v0`.
    pub(crate) fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = TEST_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, TEST_RA as u32, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// `StrHash` (0x317670).
    pub(crate) fn str_hash(c: &mut Cpu, s: u32) -> u32 {
        run(c, 0x0031_7670, ee::s10::f_00317670, &[s as u64]) as u32
    }

    #[test]
    fn ov_screen_session_on_activate_matches() {
        check(
            ee::s05::f_00208b00__generated,
            ov_screen_session_on_activate,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x208b00, 5);
                be_library_2(c);
                world_2(c);
                heap_init(c);
                let this = DATA;
                let screen = DATA + 0x400;
                let arr = DATA + 0x500;
                let child = DATA + 0x600;
                // `StrHash` clobbers the argument registers: hash first.
                let hash = str_hash(c, 0x0047_0a20);
                c.g[A0][0] = this as u64;
                if seed % 2 == 0 {
                    w(c, this + 0xd4, 0);
                } else {
                    w(c, this + 0xd4, screen);
                    w(c, screen + 0x38, arr);
                    w(c, screen + 0x3c, arr + 8);
                    w(c, arr, 1);
                    w(c, arr + 8, child);
                    w(c, child + 0x38, hash);
                    w(c, child + 0x8, DATA + 0x2000);
                    slot(c, DATA + 0x2000, 0x38, 0, FAKE);
                    slot(c, DATA + 0x2000, 0x48, 0, FAKE + 0x10);
                }
            },
        );
    }

    #[test]
    fn ov_screen_session_load_map_image_matches() {
        check(
            ee::s05::f_00208f10__generated,
            ov_screen_session_load_map_image,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x208f10, 4);
                be_library_3(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0xc4, r.next_u32() % 4);
                let wld = DATA + 0x1000;
                w(c, GAME_GP.wrapping_sub(0x848), wld);
                w(c, wld + 0x11c, DATA + 0x2000);
                w(c, DATA + 0x2000, 0);
                w(c, DATA + 0x30_0000, 0);
            },
        );
    }

    #[test]
    fn c_ov_state_overlay_play_object_anim_matches() {
        check(
            ee::s05::f_0020a6f0__generated,
            c_ov_state_overlay_play_object_anim,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x20a6f0, 2);
                put_str(c, DATA + 0x500, b"overlay");
                let h = str_hash(c, DATA + 0x500);
                let screen = DATA + 0x1000;
                screen_with(c, &mut r, screen, h);
                c.bus.write32(DATA + 0x10, DATA + 0x2000);
                c.bus.write32(DATA + 0x2050, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = sx(DATA + 0x600);
                c.bus
                    .write32(DATA + 0x40, if seed % 4 == 0 { 0 } else { screen });
            },
        );
    }

    pub(crate) const SESSION: u32 = DATA + 0x1_0000;

    pub(crate) const SESSION_IMG: u32 = DATA + 0x1_1000;

    #[test]
    fn ov_screen_session_update_matches() {
        check_all(
            ee::s05::f_00209370__generated,
            ov_screen_session_update,
            30,
            &|c, seed| {
                let mut r = base(c, seed, 0x2093_70, 6);
                c.g[A0][0] = sx(SESSION);
                c.bus.write32(SESSION + 0xdc, (seed % 2) as u32); // busy
                c.bus.write32(SESSION + 0xc8, ((seed / 2) % 2) as u32); // loaded
                c.bus.write32(SESSION + 0xcc, r.edge_f32()); // fade
                c.bus
                    .write32(SESSION + 0xd0, if seed % 3 == 0 { 0 } else { SESSION_IMG });
                c.bus.write32(SESSION + 0xc4, (seed % 4) as u32); // async file
                c.bus.write32(SESSION + 0x1cc, r.edge_f32()); // bob
                c.bus.write32(SESSION + 0x1d0, r.f32(1.0)); // bob velocity
                                                            // The eight icons at +0x104 stay null: the loop only advances
                                                            // the icon positions.
            },
        );
    }

    // ------------------------------------------------------ rate: image fade

    /// A callee outside the translated code: answer null and return at once.
    /// The rate test below only looks at the fade value.
    fn no_callee(c: &mut Cpu, _target: u32) {
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }

    /// `OVScreenSession_Update` (0x209370) at rate `hz` for `updates` updates
    /// with the session image loaded and the fade starting at 0, the VSync
    /// counter running from 1: the fade value afterwards.
    fn session_fade(hz: u32, updates: u32) -> f32 {
        let mut c = crate::difftest::fresh();
        let _ = base(&mut c, 0, 0x2093_70, 0);
        c.hooks.miss = no_callee;
        crate::tick::set_for_test(&mut c, hz);
        let app = c.bus.read32(crate::tick::APPMAN_PTR);
        // The fade step is the game's 0.1 at gp-0x5504.
        c.bus
            .write32((c.g[GP][0] as u32).wrapping_sub(0x5504), 0.1f32.to_bits());
        c.bus.write32(SESSION + 0xdc, 0); // not busy
        c.bus.write32(SESSION + 0xc8, 1); // image loaded
        c.bus.write32(SESSION + 0xcc, 0); // fade
        c.bus.write32(SESSION + 0xd0, SESSION_IMG);
        c.bus.write32(SESSION + 0xc4, 0); // async slot
        for i in 0..updates {
            c.bus.write32(app + 0x18, 1 + i);
            c.g[A0][0] = sx(SESSION);
            ov_screen_session_update(&mut c);
            assert_eq!(c.pc, crate::difftest::RA, "the update did not return");
        }
        f32::from_bits(c.bus.read32(SESSION + 0xcc))
    }

    /// The image fade adds one authored 60 Hz frame's worth of alpha (0.1) per
    /// 60 Hz boundary, so it takes the same wall-clock time at every rate: one
    /// update at 60 Hz, two at 120 and four at 240 give the same fade.
    #[test]
    fn ov_screen_session_fade_follows_the_rate() {
        let one = session_fade(60, 1);
        assert!((one - 0.1).abs() < 1e-6, "one 60 Hz frame: {one}");
        for (hz, mul) in [(120u32, 2u32), (240, 4)] {
            let f = session_fade(hz, mul);
            assert!((f - one).abs() < 1e-6, "{hz} Hz, one frame: {f} vs {one}");
        }
        // Five frames (0.5) and nine frames (0.9) agree the same way.
        for frames in [5u32, 9] {
            let at60 = session_fade(60, frames);
            for (hz, mul) in [(120u32, 2u32), (240, 4)] {
                let f = session_fade(hz, frames * mul);
                assert!(
                    (f - at60).abs() < 1e-5,
                    "{hz} Hz, {frames} frames: {f} vs {at60}"
                );
            }
        }
    }

    pub(crate) const MAP_IND: u32 = DATA + 0x1_0000;

    pub(crate) const MAP_SCREEN: u32 = DATA + 0x1_1000;

    pub(crate) const MAP_LIST: u32 = DATA + 0x1_2000;

    pub(crate) const MAP_ICONS: u32 = DATA + 0x1_3000;

    /// `OVScreenSession_DestroyIndicators`: each of the 17 indicator slots
    /// (`+0xe0`, `+0xe4..`, `+0x104..`) is empty or holds an object whose
    /// vtable methods (`+0x4c`, `+0xc`) are fakes; some of them unwind.
    #[test]
    fn ov_screen_session_destroy_indicators_matches() {
        check(
            ee::s05::f_0020a088__generated,
            ov_screen_session_destroy_indicators,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x20a088, 34);
                let this = DATA + 0x100;
                let vt = DATA + 0x3000;
                fake_method(c, vt, 0x48);
                fake_method(c, vt, 0x8);
                c.g[A0][0] = sx(this);
                let slots = std::iter::once(0xe0u32)
                    .chain((0..8).map(|i| 0xe4 + 4 * i))
                    .chain((0..8).map(|i| 0x104 + 4 * i));
                for (i, off) in slots.enumerate() {
                    let obj = DATA + 0x4000 + 0x100 * i as u32;
                    let present = r.next_u32() % 4 != 0;
                    w(c, this + off, if present { obj } else { 0 });
                    w(c, obj + 8, vt);
                }
            },
        );
    }

    #[test]
    fn ov_screen_session_update_indicators_matches() {
        check_all(
            ee::s05::f_00209e78__generated,
            ov_screen_session_update_indicators,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x209e_78, 12);
                c.g[A0][0] = sx(MAP_IND);
                c.bus
                    .write32(MAP_IND + 0x0, if seed % 2 == 0 { 0 } else { MAP_ICONS });
                c.bus.write32(MAP_IND + 0x4, MAP_SCREEN);
                c.bus.write32(MAP_IND + 0x8, (seed % 4) as u32); // count
                c.bus.write32(MAP_IND + 0xc, MAP_LIST); // list object
                c.bus.write32(MAP_SCREEN + 0xa0, MAP_LIST);
                c.bus
                    .write32(MAP_LIST + 0x18, ((seed % 4) as u32).wrapping_add(1));
                for i in 0..4u32 {
                    c.bus.write32(MAP_IND + 0x10 + 4 * i, MAP_ICONS + 0x100 * i);
                    c.bus
                        .write32(MAP_IND + 0x20 + 4 * i, MAP_ICONS + 0x800 + 0x100 * i);
                }
            },
        );
    }

    use crate::difftest::fakes::covered;
    use crate::difftest::fill;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC767_CTRL;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC767_MAGIC;

    /// `FE_FormatMinSec`: the option block at 0x535610 holds mode `seed % 8`
    /// in bits 22..24 of its first word (0/2: format A, 1/3: format B, 4-7:
    /// no text); `secs` is negative, zero, below and above a minute; the
    /// four callees (`cBE_getBE`, `getInterface`, `fptodp` twice, `sprintf`)
    /// are fakes and some of them unwind.
    #[test]
    fn fe_format_min_sec_matches() {
        oc767_install_fakes();
        covered(
            ee::s05::f_0020a1a8__generated,
            fe_format_min_sec,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0020_a1a8, 5);
                c.bus.write32(OC767_CTRL, OC767_MAGIC);
                fill(c, &mut r, FE_FORMAT_MIN_SEC_OPTIONS, 0x288 / 4);
                let first = (r.next_u32() & !(7 << 22)) | ((seed % 8) << 22);
                c.bus.write32(FE_FORMAT_MIN_SEC_OPTIONS, first);
                let secs = match r.next_u32() % 5 {
                    0 => 0u32,
                    1 => r.next_u32() % 60,
                    2 => r.next_u32() % 6000,
                    3 => 0u32.wrapping_sub(r.next_u32() % 500),
                    _ => r.next_u32(),
                };
                c.g[A0][0] = sx(secs);
                c.pc = 0x0020_a1a8;
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine_rt;

    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::Rng;

    pub(crate) use crate::game::fe::ovstateenterlodge::tests::oc1004_setup;

    pub(crate) use crate::game::fe::ovstatepause::tests::oc987_enc;

    pub(crate) use crate::game::fe::ovstatepause::tests::oc987_machine;

    pub(crate) use crate::game::fe::ovstatepause::tests::OC987_MBOX;
    pub(crate) use crate::game::fe::ovstatepause::tests::OC987_OBJ;

    pub(crate) use crate::game::fe::ovtemplatedialog::tests::oc983_fixture;

    pub(crate) use crate::game::fe::ovtemplatedialog::tests::oc983_paths;

    pub(crate) use crate::game::fe::ovtemplatedialog::tests::OC983_SELF;
    pub(crate) use crate::game::fe::ovtemplatedialog::tests::OC983_WIDGET;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::ov_object_setup;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// `OVScreenSession_OnWidgetAction(self, widget, event)`: a map screen's
    /// next / previous / focus events, with the session popup open or closed
    /// and the widget's misc data (`+0x18`) 0 or not; events outside 1..=7 and
    /// a null widget leave at once.
    pub(crate) fn oc983_session_setup(c: &mut Cpu, seed: u32) {
        let _r = oc983_fixture(c, seed, 0x208c28, 30);
        c.pc = 0x0020_8c28;
        c.g[A0][0] = sx(OC983_SELF);
        let widget = if seed >= 36 && seed % 7 == 0 {
            0
        } else {
            OC983_WIDGET
        };
        c.g[A1][0] = sx(widget);
        c.g[A2][0] = sx(seed % 9);
        c.bus.write32(OC983_SELF + 0xbc, (seed / 9) % 2);
        c.bus.write32(OC983_WIDGET + 0x18, (seed / 18) % 2);
    }

    #[test]
    fn ov_screen_session_on_widget_action_matches() {
        check(
            ee::s05::f_00208c28__generated,
            ov_screen_session_on_widget_action,
            300,
            &oc983_session_setup,
        );
        let n = oc983_paths(ee::s05::f_00208c28__generated, 300, &oc983_session_setup);
        assert!(n >= 8, "only {n} distinct paths");
    }

    /// Map screen labels: `on` clear / set, the entry count forced to 0..9 at
    /// the call that returns it (`n`), a game world present or not.
    #[test]
    fn ov_screen_session_transition_in_matches() {
        covered(
            ee::s05::f_002087f0__generated,
            ov_screen_session_transition_in,
            64,
            &|c, seed| {
                oc987_machine(c, seed, 0x2087f0, 60, &[0, 1, 2]);
                c.g[A1][0] = (seed % 5 != 0) as u64;
                c.bus
                    .write32(OC987_MBOX + 4 * 5, oc987_enc((seed % 10) as i32));
                if seed % 2 == 1 {
                    let world = OC987_OBJ + 0x2000;
                    c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                    c.bus.write32(world + 0x84, world + 0x400);
                    c.bus.write32(world + 0x400 + 0xc, world + 0x800);
                    c.bus.write32(world + 0x800 + 0x28, world + 0xc00);
                }
            },
        );
    }

    /// Gate word of this module's callee fakes (in the machine's own RAM).
    pub(crate) const OC988_CTRL: u32 = DATA + 0x2f_0000;

    pub(crate) const OC988_MAGIC: u32 = 0x0c98_8a5e;

    /// A real leaf that returns 0 (`jr ra; li v0, 0`).
    pub(crate) const OC988_ZERO_LEAF: u32 = 0x0014_06c0;

    /// A real leaf that returns `a0` (`jr ra; move v0, a0`).
    pub(crate) const OC988_THIS_LEAF: u32 = 0x0014_0ce0;

    /// Kinds of the gated fakes: log only, `sprintf` (writes `s<a2>`), or a
    /// result sequence (kind `OC988_K_SEQ + n`: `v0 = word[ctr & 3]` of
    /// mailbox `n` at `CTRL + 0x30 + 0x10 * n`).
    pub(crate) const OC988_K_PLAIN: u32 = 0;

    pub(crate) const OC988_K_SPRINTF: u32 = 1;

    pub(crate) const OC988_K_SEQ: u32 = 2;

    /// The logging fake of a gated callee: logs the call like every fake
    /// machine call, then (by kind) writes the name or sets `v0`.
    pub(crate) fn oc988_fake(c: &mut Cpu, addr: u32, kind: u32) {
        let a0 = c.g[A0][0] as u32;
        let a2 = c.g[A2][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match kind {
            OC988_K_PLAIN => {}
            OC988_K_SPRINTF => {
                let text = format!("s{a2}");
                for (i, b) in text.bytes().enumerate() {
                    c.bus.write8(a0 + i as u32, b);
                }
                c.bus.write8(a0 + text.len() as u32, 0);
            }
            k => {
                let n = k - OC988_K_SEQ;
                let ctr = OC988_CTRL + 0x20 + 4 * n;
                let i = c.bus.read32(ctr);
                c.bus.write32(ctr, i + 1);
                let v = c.bus.read32(OC988_CTRL + 0x30 + 0x10 * n + 4 * (i & 3));
                c.g[V0][0] = sx(v);
            }
        }
    }

    macro_rules! oc988_fakes {
        ($($name:ident $prev:ident $addr:literal $shard:ident $slot:ident $gen:ident $kind:expr;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC988_CTRL) != OC988_MAGIC {
                        return fakes::fall_back(c, &$prev, ee::$shard::$gen);
                    }
                    oc988_fake(c, $addr, $kind)
                }
            )*
            fn oc988_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, &$prev); )*
            }
        };
    }

    oc988_fakes! {
        oc988_f_create_screen OC988_P_CREATE_SCREEN 0x0039_7cc0 s13 OVERRIDE_00397CC0 f_00397cc0__generated OC988_K_SEQ;
        oc988_f_start_thread OC988_P_START_THREAD 0x0039_c870 s13 OVERRIDE_0039C870 f_0039c870__generated OC988_K_PLAIN;
        oc988_f_overlay_init OC988_P_OVERLAY_INIT 0x0020_a380 s05 OVERRIDE_0020A380 f_0020a380__generated OC988_K_PLAIN;
        oc988_f_step_slot_dir OC988_P_STEP_SLOT_DIR 0x001d_97e0 s04 OVERRIDE_001D97E0 f_001d97e0__generated OC988_K_SEQ + 1;
        oc988_f_card_error_text OC988_P_CARD_ERROR_TEXT 0x0024_1710 s06 OVERRIDE_00241710 f_00241710__generated OC988_K_PLAIN;
        oc988_f_no_cards OC988_P_NO_CARDS 0x001d_8240 s04 OVERRIDE_001D8240 f_001d8240__generated OC988_K_PLAIN;
        oc988_f_reset_slot_list OC988_P_RESET_SLOT_LIST 0x0021_1270 s05 OVERRIDE_00211270 f_00211270__generated OC988_K_PLAIN;
        oc988_f_set_wide OC988_P_SET_WIDE 0x003a_0d00 s13 OVERRIDE_003A0D00 f_003a0d00__generated OC988_K_PLAIN;
        oc988_f_set_wide_string OC988_P_SET_WIDE_STRING 0x003a_0e90 s13 OVERRIDE_003A0E90 f_003a0e90__generated OC988_K_PLAIN;
        oc988_f_sprintf OC988_P_SPRINTF 0x0041_7828 s15 OVERRIDE_00417828 f_00417828__generated OC988_K_SPRINTF;
        oc988_f_sprintf_wide OC988_P_SPRINTF_WIDE 0x002c_27c0 s08 OVERRIDE_002C27C0 f_002c27c0__generated OC988_K_PLAIN;
        oc988_f_set_string OC988_P_SET_STRING 0x003a_0c50 s13 OVERRIDE_003A0C50 f_003a0c50__generated OC988_K_PLAIN;
        oc988_f_set_text_id OC988_P_SET_TEXT_ID 0x003a_1000 s13 OVERRIDE_003A1000 f_003a1000__generated OC988_K_PLAIN;
        oc988_f_nav_buttons OC988_P_NAV_BUTTONS 0x003a_1310 s13 OVERRIDE_003A1310 f_003a1310__generated OC988_K_PLAIN;
    }

    pub(crate) fn oc988_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// The gated fakes on, with their result mailboxes (word `i` of mailbox
    /// `n`) set by the caller afterwards.
    pub(crate) fn oc988_gate(c: &mut Cpu) {
        oc988_install_fakes();
        c.bus.write32(OC988_CTRL, OC988_MAGIC);
        for i in 0..0x20 {
            c.bus.write32(OC988_CTRL + 4 + 4 * i, 0);
        }
    }

    pub(crate) fn oc988_put_str(c: &mut Cpu, addr: u32, text: &str) {
        for (i, b) in text.bytes().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + text.len() as u32, 0);
    }

    /// `StrHash(text)` run for real on the fixture machine.
    pub(crate) fn oc988_hash(c: &mut Cpu, addr: u32) -> u32 {
        c.g[A0][0] = addr as u64;
        c.g[RA][0] = TEST_RA as u64;
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(c);
        assert_eq!(c.pc, TEST_RA, "StrHash did not return");
        c.g[V0][0] as u32
    }

    /// A vtable of 0x48 slots (`{delta 0, method}` at `8 * n`); a slot's
    /// method is a logging fake, or (1 in `zero_chance`) a real leaf that
    /// returns 0.
    pub(crate) fn oc988_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, zero_chance: u32) {
        for n in 0..0x48u32 {
            c.bus.write16(vt + 8 * n, 0);
            let f = if zero_chance != 0 && r.next_u32() % zero_chance == 0 {
                OC988_ZERO_LEAF
            } else {
                FAKE + 0x10 * n
            };
            c.bus.write32(vt + 8 * n + 4, f);
        }
    }

    /// A UI screen at `screen` (needs `0x3000` bytes): the children named by
    /// `names` (`(address of the name string, its text, present)`) with the
    /// hash of their string in `+0x38` and a vtable at `+8`. Lookups by
    /// `StrHash(name)` find exactly the present ones.
    pub(crate) fn oc988_screen(
        c: &mut Cpu,
        r: &mut Rng,
        screen: u32,
        names: &[(u32, String, bool)],
    ) {
        let vt = screen + 0x1000;
        oc988_vtable(c, r, vt, 3);
        let count = screen + 0x10;
        let array = screen + 0x20;
        let mut n = 0;
        for (i, (addr, text, present)) in names.iter().enumerate() {
            oc988_put_str(c, *addr, text);
            let h = oc988_hash(c, *addr);
            if *present {
                let obj = screen + 0x2000 + 0x100 * i as u32;
                c.bus.write32(obj + 8, vt);
                c.bus.write32(obj + 0x38, h);
                c.bus.write32(obj + 0x94, r.next_u32());
                c.bus.write32(array + 4 * n, obj);
                n += 1;
            }
        }
        c.bus.write32(count, n);
        c.bus.write32(screen + 0x38, count);
        c.bus.write32(screen + 0x3c, array);
        // The list head the indicators are appended to (`screen + 0xb4`).
        c.bus.write32(screen + 0xb4 + 0xc, screen + 0xe0);
    }

    /// Names of `addrs`, present with probability 3 in 4.
    pub(crate) fn oc988_names(r: &mut Rng, addrs: &[u32], tag: &str) -> Vec<(u32, String, bool)> {
        addrs
            .iter()
            .enumerate()
            .map(|(i, &a)| (a, format!("{tag}{i}"), r.next_u32() % 4 != 0))
            .collect()
    }

    /// The first track-info record (`0x440770`: key 0 matches the key the
    /// zeroed track table gives) with random bounds floats; `+0x18` is the
    /// session-type count `BEMissionInt_GetSessionType` returns.
    pub(crate) fn oc988_bounds(c: &mut Cpu, r: &mut Rng, count: u32) {
        for off in (4..0x2c).step_by(4) {
            c.bus.write32(0x0044_0770 + off, r.f32(300.0));
        }
        c.bus.write32(0x0044_0770, 0);
        c.bus.write32(0x0044_0770 + 0x18, count);
    }

    /// `world -> +0x84 -> +0xc -> +0x28` = rider whose object at `+0x6c0` has
    /// the vtable method `vt+0x2c` (the real "return this" leaf, with a `+0x10`
    /// delta, so the position vector is read from the rider at `+0x6d0`).
    pub(crate) fn oc988_rider(c: &mut Cpu, r: &mut Rng) {
        let world = DATA + 0x2000;
        let race = DATA + 0x2100;
        let mgr = DATA + 0x2200;
        let rider = DATA + 0x2400;
        let vt = DATA + 0x2300;
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, race);
        c.bus.write32(race + 0xc, mgr);
        c.bus.write32(mgr + 0x28, rider);
        c.bus.write32(rider + 0x6c0, vt);
        c.bus.write16(vt + 0x28, 0x10);
        c.bus.write32(vt + 0x2c, OC988_THIS_LEAF);
        for i in 0..4 {
            c.bus.write32(rider + 0x6d0 + 4 * i, r.f32(300.0));
        }
    }

    /// `OVScreenSession_UpdateConfirmPopup`: with the flag at `+0xbc` set the
    /// popup widgets are shown and filled from the string manager (reached
    /// through the world; its lookup method returns 0 or a pointer), without
    /// it they are hidden. Widgets are present or missing.
    #[test]
    fn ov_screen_session_update_confirm_popup_matches() {
        check(
            ee::s05::f_00208f88__generated,
            ov_screen_session_update_confirm_popup,
            90,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x208f88, 36);
                oc988_gate(c);
                let this = DATA + 0x6000;
                let screen = DATA + 0x8000;
                let names = oc988_names(
                    &mut r,
                    &[
                        0x0047_0a20,
                        0x0047_1850,
                        0x0047_1890,
                        0x0047_18a0,
                        0x0047_18b8,
                        0x0047_18c8,
                        0x0047_18d8,
                        0x0047_06b8,
                        0x0047_06c8,
                        0x0047_0aa8,
                    ],
                    "a",
                );
                oc988_screen(c, &mut r, screen, &names);
                oc988_w(c, this + 0xd4, if seed % 11 == 0 { 0 } else { screen });
                oc988_w(c, this + 0xbc, if seed % 3 == 0 { 0 } else { 1 });
                // world -> +0x8c: the string manager and its lookup method.
                let world = DATA + 0x2000;
                let mgr = DATA + 0x2100;
                let vt = DATA + 0x2200;
                oc988_w(c, GAME_GP - 0x848, world);
                oc988_w(c, world + 0x8c, mgr);
                oc988_w(c, mgr + 4, vt);
                c.bus.write16(vt + 0x20, 0);
                oc988_w(
                    c,
                    vt + 0x24,
                    if r.next_u32() % 3 == 0 {
                        OC988_ZERO_LEAF
                    } else {
                        FAKE + 0x500
                    },
                );
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// `OVScreenSession_CreatePlayerIndicator`: the rider's map position
    /// (random bounds, negative and positive) becomes the icon's place; the
    /// interface results `S1`/`S2` are sometimes negative (the unsigned
    /// float conversions).
    #[test]
    fn ov_screen_session_create_player_indicator_matches() {
        check(
            ee::s05::f_002096a8__generated,
            ov_screen_session_create_player_indicator,
            60,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x2096a8, 12);
                oc988_gate(c);
                let this = DATA + 0x6000;
                let img = DATA + 0x6800;
                let screen = DATA + 0x8000;
                oc988_rider(c, &mut r);
                oc988_bounds(c, &mut r, 0);
                for i in 0..3 {
                    c.bus.write32(img + 0x44 + 4 * i, r.f32(400.0));
                }
                oc988_w(c, this + 0xd0, img);
                oc988_w(c, this + 0xd4, screen);
                oc988_w(c, screen + 0xb4 + 0xc, screen + 0xe0);
                // The two interface lookups are the first two fake calls.
                let neg = |bit: u32| {
                    if (seed >> bit) & 1 == 1 {
                        0xffff_ff00u32
                    } else {
                        0
                    }
                };
                c.bus.write32(LOG_RESULTS, neg(0));
                c.bus.write32(LOG_RESULTS + 4, neg(1));
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// `OVScreenSession_CreateMapIndicators`: one icon per path marker of
    /// type 2 that exists, for `count` session entries (0..=4), markers
    /// present or absent.
    #[test]
    fn ov_screen_session_create_map_indicators_matches() {
        check(
            ee::s05::f_00209970__generated,
            ov_screen_session_create_map_indicators,
            60,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x209970, 12);
                oc988_gate(c);
                let this = DATA + 0x6000;
                let img = DATA + 0x6800;
                let screen = DATA + 0x8000;
                let count = if seed % 5 == 0 { 0 } else { 1 + seed % 4 };
                oc988_bounds(c, &mut r, count);
                for i in 0..3 {
                    c.bus.write32(img + 0x44 + 4 * i, r.f32(400.0));
                }
                oc988_w(c, this + 0xd0, img);
                oc988_w(c, this + 0xd4, screen);
                oc988_w(c, screen + 0xb4 + 0xc, screen + 0xe0);
                // Path data at 0x4d33a0: `{count, entries}`, 0x28 bytes per
                // entry `{id, type, position..}`.
                let path = 0x004d_33a0;
                let entries = DATA + 0x7000;
                let n = 1 + seed % 3;
                oc988_w(c, path, n);
                oc988_w(c, path + 4, entries);
                for i in 0..n {
                    let e = entries + 0x28 * i;
                    oc988_w(c, e, 1 + i);
                    oc988_w(c, e + 4, if r.next_u32() % 4 == 0 { 3 } else { 2 });
                    for k in 0..3 {
                        oc988_w(c, e + 8 + 4 * k, r.f32(300.0));
                    }
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC992_MAGIC` (otherwise the body they replaced runs).
    pub(crate) const OC992_CTRL: u32 = DATA + 0x0030_0000;

    pub(crate) const OC992_MAGIC: u32 = 0x0992_c0de;

    /// Forced results: one word per callee address (`v + 3`, so 0 is `3`).
    pub(crate) const OC992_MAIL: u32 = DATA + 0x0031_0000;

    /// The world (`gp - 0x848` points here).
    pub(crate) const OC992_W: u32 = DATA + 0x0020_0000;

    /// Fixture objects.
    pub(crate) const OC992_O: u32 = DATA + 0x0021_0000;

    pub(crate) const OC992_P: u32 = DATA + 0x0022_0000;

    pub(crate) const OC992_Q: u32 = DATA + 0x0023_0000;

    pub(crate) const OC992_R: u32 = DATA + 0x0023_1000;

    /// The class struct the `obj->vtbl` calls read (+0x48 short, +0x4c target).
    pub(crate) const OC992_CLS: u32 = DATA + 0x0024_0000;

    /// Dynamic call targets: outside the table, so the machine's miss hook answers.
    pub(crate) const OC992_TGT: u32 = fakes::FAKE + 0x40;

    pub(crate) fn oc992_mail(addr: u32) -> u32 {
        OC992_MAIL + (addr & 0xfffc)
    }

    /// Force the fake for `addr` to return `v` (0 forces zero).
    pub(crate) fn oc992_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc992_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs like `fakes::fake_call`, then returns the forced
    /// result unless it unwound.
    pub(crate) fn oc992_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc992_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = if mail == 1 { 0 } else { (mail - 3) as u64 };
        }
    }

    macro_rules! oc992_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC992_CTRL) != OC992_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc992_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc992_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    // Direct and table callees of the eight bodies. Each slot is hooked once
    // per process; the fakes answer only in the fixtures of this module.
    oc992_fakes! {
        (oc992_fake_2018a8, OC992_PREV_2018A8, ee::s05::OVERRIDE_002018A8, ee::s05::f_002018a8__generated, 0x0020_18a8),
        (oc992_fake_202770, OC992_PREV_202770, ee::s05::OVERRIDE_00202770, ee::s05::f_00202770__generated, 0x0020_2770),
        (oc992_fake_39f190, OC992_PREV_39F190, ee::s13::OVERRIDE_0039F190, ee::s13::f_0039f190__generated, 0x0039_f190),
        (oc992_fake_317670, OC992_PREV_317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670),
        (oc992_fake_397cc0, OC992_PREV_397CC0, ee::s13::OVERRIDE_00397CC0, ee::s13::f_00397cc0__generated, 0x0039_7cc0),
        (oc992_fake_39c870, OC992_PREV_39C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039_c870),
        (oc992_fake_20a380, OC992_PREV_20A380, ee::s05::OVERRIDE_0020A380, ee::s05::f_0020a380__generated, 0x0020_a380),
        (oc992_fake_1a3bf0, OC992_PREV_1A3BF0, ee::s03::OVERRIDE_001A3BF0, ee::s03::f_001a3bf0__generated, 0x001a_3bf0),
        (oc992_fake_39d860, OC992_PREV_39D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039_d860),
        (oc992_fake_208f10, OC992_PREV_208F10, ee::s05::OVERRIDE_00208F10, ee::s05::f_00208f10__generated, 0x0020_8f10),
        (oc992_fake_208f88, OC992_PREV_208F88, ee::s05::OVERRIDE_00208F88, ee::s05::f_00208f88__generated, 0x0020_8f88),
        (oc992_fake_2096a8, OC992_PREV_2096A8, ee::s05::OVERRIDE_002096A8, ee::s05::f_002096a8__generated, 0x0020_96a8),
        (oc992_fake_209970, OC992_PREV_209970, ee::s05::OVERRIDE_00209970, ee::s05::f_00209970__generated, 0x0020_9970),
        (oc992_fake_14dd58, OC992_PREV_14DD58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated, 0x0014_dd58),
        (oc992_fake_258160, OC992_PREV_258160, ee::s06::OVERRIDE_00258160, ee::s06::f_00258160__generated, 0x0025_8160),
        (oc992_fake_3a19f8, OC992_PREV_3A19F8, ee::s13::OVERRIDE_003A19F8, ee::s13::f_003a19f8__generated, 0x003a_19f8),
        (oc992_fake_1a83d8, OC992_PREV_1A83D8, ee::s03::OVERRIDE_001A83D8, ee::s03::f_001a83d8__generated, 0x001a_83d8),
        (oc992_fake_3182d8, OC992_PREV_3182D8, ee::s10::OVERRIDE_003182D8, ee::s10::f_003182d8__generated, 0x0031_82d8),
        (oc992_fake_318910, OC992_PREV_318910, ee::s10::OVERRIDE_00318910, ee::s10::f_00318910__generated, 0x0031_8910),
        (oc992_fake_318260, OC992_PREV_318260, ee::s10::OVERRIDE_00318260, ee::s10::f_00318260__generated, 0x0031_8260),
        (oc992_fake_20e900, OC992_PREV_20E900, ee::s05::OVERRIDE_0020E900, ee::s05::f_0020e900__generated, 0x0020_e900),
        (oc992_fake_39c778, OC992_PREV_39C778, ee::s13::OVERRIDE_0039C778, ee::s13::f_0039c778__generated, 0x0039_c778),
        (oc992_fake_2c26d0, OC992_PREV_2C26D0, ee::s08::OVERRIDE_002C26D0, ee::s08::f_002c26d0__generated, 0x002c_26d0),
        (oc992_fake_3a0e90, OC992_PREV_3A0E90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated, 0x003a_0e90),
        (oc992_fake_14dc80, OC992_PREV_14DC80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated, 0x0014_dc80),
        (oc992_fake_30b8c0, OC992_PREV_30B8C0, ee::s10::OVERRIDE_0030B8C0, ee::s10::f_0030b8c0__generated, 0x0030_b8c0),
        (oc992_fake_258c00, OC992_PREV_258C00, ee::s06::OVERRIDE_00258C00, ee::s06::f_00258c00__generated, 0x0025_8c00),
        (oc992_fake_258790, OC992_PREV_258790, ee::s06::OVERRIDE_00258790, ee::s06::f_00258790__generated, 0x0025_8790),
        (oc992_fake_20ab50, OC992_PREV_20AB50, ee::s05::OVERRIDE_0020AB50, ee::s05::f_0020ab50__generated, 0x0020_ab50),
    }

    /// Machine with the gate on, the world at `gp - 0x848`, and no forced
    /// results (every fake returns its default pointer).
    pub(crate) fn oc992_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(OC992_CTRL, OC992_MAGIC);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC992_W);
        r
    }

    /// A class struct for the method calls: short +0x48 and target +0x4c.
    pub(crate) fn oc992_class(c: &mut Cpu, k: u32) {
        c.bus.write16(OC992_CLS + 0x48, (0x10 * k) as u16);
        c.bus.write32(OC992_CLS + 0x4c, OC992_TGT + 0x10 * k);
    }

    #[test]
    fn ovscreensession_init_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_002086a8__generated,
            ovscreensession_init,
            200,
            &|c, seed| {
                let mut r = oc992_base(c, seed, 0x0992_86a8);
                c.pc = 0x0020_86a8;
                c.g[A0][0] = sx(OC992_O);
                c.bus.write32(OC992_O + 0x10, OC992_P);
                // The picture handle and the widget from the table.
                oc992_force(c, 0x0039_7cc0, if seed % 2 == 0 { 0 } else { OC992_Q });
                oc992_force(c, 0x0039_d860, if seed % 3 == 0 { 0 } else { OC992_Q });
                c.bus.write32(OC992_Q + 8, OC992_CLS);
                oc992_class(c, seed % 4);
                c.bus.write32(OC992_W + 0x8c, r.next_u32());
            },
        );
    }

    #[test]
    fn ovscreensession_hide_indicators_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_00209fd0__generated,
            ovscreensession_hide_indicators,
            200,
            &|c, seed| {
                let mut r = oc992_base(c, seed, 0x0992_9fd0);
                c.pc = 0x0020_9fd0;
                c.g[A0][0] = sx(OC992_O);
                oc992_class(c, seed % 4);
                c.bus
                    .write32(OC992_O + 0xe0, if seed % 5 == 0 { 0 } else { OC992_P });
                c.bus.write32(OC992_P + 8, OC992_CLS);
                c.bus.write32(OC992_O + 0xc0, r.next_u32() % 4);
                for j in 0..12u32 {
                    let e = if r.next_u32() % 3 == 0 { 0 } else { OC992_P };
                    c.bus.write32(OC992_O + 0xe4 + 4 * j, e);
                }
            },
        );
    }

    #[test]
    fn ovscreen_final_standings_recv_chat_message_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_0020a4e0__generated,
            ovscreen_final_standings_recv_chat_message,
            200,
            &|c, seed| {
                let mut r = oc992_base(c, seed, 0x0992_a4e0);
                c.pc = 0x0020_a4e0;
                c.g[A0][0] = sx(OC992_O);
                c.bus.write32(OC992_W + 0x78, OC992_P);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 7 == 0 { 0 } else { OC992_R },
                );
                c.bus
                    .write32(OC992_R, if seed % 5 == 0 { 0 } else { OC992_P });
                c.bus
                    .write32(0x0053_4b30, if seed % 4 == 0 { 0 } else { 1 });
                c.bus
                    .write32(OC992_O + 0x40, if seed % 6 == 0 { 0 } else { OC992_Q });
                c.bus.write32(OC992_O + 0x58, r.next_u32());
                c.bus.write32(OC992_Q + 8, OC992_CLS);
                oc992_class(c, seed % 4);
                oc992_force(c, 0x0025_8160, seed % 3);
                oc992_force(c, 0x0039_d860, if seed % 4 == 1 { 0 } else { OC992_Q });
                c.bus.write32(OC992_Q + 0xc4, r.next_u32() & 0xffff);
                c.bus.write32(OC992_Q + 0xc8, r.next_u32() & 0xffff);
            },
        );
    }

    #[test]
    fn ov_screen_session_ctor_matches() {
        covered(
            ee::s05::f_002212c8__generated,
            ov_screen_session_ctor,
            240,
            &|c, seed| {
                let mut r = oc1004_setup(c, seed, 0x0022_12c8, 1);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ov_screen_session_ctor_2_matches() {
        covered(
            ee::s05::f_00221350__generated,
            ov_screen_session_ctor_2,
            240,
            &|c, seed| {
                let mut r = oc1004_setup(c, seed, 0x0022_1350, 1);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ov_screen_session_dtor_matches() {
        check(
            ee::s05::f_002213d8__generated,
            ov_screen_session_dtor,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    /// `OVScreenSession_OnWidgetCreated` (0x0020_87e8) against the generated body.
    #[test]
    fn ov_screen_session_on_widget_created_002087e8_matches() {
        check(
            ee::s05::f_002087e8__generated,
            ov_screen_session_on_widget_created_002087e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2087e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenSession_v13` (0x0022_1400) against the generated body.
    #[test]
    fn ov_screen_session_v13_00221400_matches() {
        check(
            ee::s05::f_00221400__generated,
            ov_screen_session_v13_00221400,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x221400, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
