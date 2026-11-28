//! `src/fe/festateoptions.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::returned;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
#[cfg(test)]
use crate::difftest::fakes;
#[cfg(test)]
use crate::difftest::Rng;
#[cfg(test)]
use crate::difftest::DATA;
#[cfg(test)]
use crate::difftest::GAME_GP;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::rider::world;
pub(crate) use crate::game::dirtysock::tagsunk::const_reg;
pub(crate) use crate::game::dirtysock::tagsunk::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tagsunk::p_2;
pub(crate) use crate::game::dirtysock::tagsunk::sw;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_4;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festatenethelppopup::fe_state_screen_init;
pub(crate) use crate::game::fe::festateprofile::off842;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::low as low_2;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_3;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
#[cfg(test)]
use std::ptr::addr_of_mut;
#[cfg(test)]
use std::sync::atomic::AtomicUsize;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0018_8800, c_fe_state_options_init, "cFEStateOptions_Init"),
    (
        0x0018_8c58,
        cfe_state_options_handle_input,
        "cFEStateOptions_HandleInput",
    ),
    (
        0x0018_8e58,
        c_fe_state_options_open_reset_popup,
        "cFEStateOptions_OpenResetPopup",
    ),
    (
        0x0018_8ee8,
        c_fe_state_options_game_ctor,
        "cFEStateOptionsGame_ctor",
    ),
    (
        0x0018_9958,
        cfe_state_options_game_on_widget_notify,
        "cFEStateOptionsGame_OnWidgetNotify",
    ),
    (
        0x0018_9970,
        cfe_state_options_game_v21,
        "cFEStateOptionsGame_v21",
    ),
    (
        0x0018_a258,
        c_fe_state_options_sound_ctor,
        "cFEStateOptionsSound_ctor",
    ),
    (
        0x0018_a890,
        cfe_state_options_sound_on_widget_notify,
        "cFEStateOptionsSound_OnWidgetNotify",
    ),
    (
        0x0018_a8c0,
        cfe_state_options_sound_v21,
        "cFEStateOptionsSound_v21",
    ),
    (
        0x0018_c0e8,
        c_fe_state_options_hud_ctor,
        "cFEStateOptionsHUD_ctor",
    ),
    (
        0x0018_c128,
        c_fe_state_options_hud_init,
        "cFEStateOptionsHUD_Init",
    ),
    (
        0x0018_c1b8,
        fe_state_options_hud_on_child_event,
        "cFEStateOptionsHUD_OnChildEvent",
    ),
    (
        0x0018_c270,
        cfe_state_options_hud_on_widget_notify,
        "cFEStateOptionsHUD_OnWidgetNotify",
    ),
    (
        0x0018_c658,
        fe_state_options_controller_ctor,
        "cFEStateOptionsController_ctor",
    ),
    (
        0x0018_c8b8,
        cfe_state_options_controller_shutdown,
        "cFEStateOptionsController_Shutdown",
    ),
    (
        0x0018_cdb0,
        cfe_state_options_controller_v21,
        "cFEStateOptionsController_v21",
    ),
    (
        0x0018_d058,
        cfe_state_options_controller_update,
        "cFEStateOptionsController_Update",
    ),
    (
        0x0018_d240,
        c_fe_state_options_save_load_ctor,
        "cFEStateOptionsSaveLoad_ctor",
    ),
    (
        0x0018_d280,
        c_fe_state_options_save_load_init,
        "cFEStateOptionsSaveLoad_Init",
    ),
    (
        0x0018_d3c0,
        c_fe_state_options_save_load_on_render,
        "cFEStateOptionsSaveLoad_OnRender",
    ),
    (
        0x0018_d438,
        cfe_state_options_save_load_on_widget_notify,
        "cFEStateOptionsSaveLoad_OnWidgetNotify",
    ),
    (
        0x0018_d7f0,
        c_fe_state_options_device_select_ctor,
        "cFEStateOptionsDeviceSelect_ctor",
    ),
    (
        0x0018_ddd0,
        cfe_state_options_device_select_on_widget_notify,
        "cFEStateOptionsDeviceSelect_OnWidgetNotify",
    ),
    (
        0x0018_e178,
        c_fe_state_options_device_select_v37,
        "cFEStateOptionsDeviceSelect_v37",
    ),
    (
        0x0018_e2c0,
        cfe_state_options_device_select_v38,
        "cFEStateOptionsDeviceSelect_v38",
    ),
    (
        0x0018_ca10,
        fe_options_controller_on_widget_action,
        "cFEStateOptionsController_OnWidgetAction",
    ),
    (
        0x001d_dee8,
        cfe_state_options_ctor_2,
        "cFEStateOptions_ctor_2",
    ),
    (0x001d_df20, cfe_state_options_dtor, "cFEStateOptions_dtor"),
    (
        0x001d_df48,
        cfe_state_options_game_ctor_2,
        "cFEStateOptionsGame_ctor_2",
    ),
    (
        0x001d_df80,
        cfe_state_options_game_dtor,
        "cFEStateOptionsGame_dtor",
    ),
    (
        0x001d_dfb0,
        cfe_state_options_sound_ctor_2,
        "cFEStateOptionsSound_ctor_2",
    ),
    (
        0x001d_dfe8,
        cfe_state_options_sound_dtor,
        "cFEStateOptionsSound_dtor",
    ),
    (
        0x001d_e018,
        cfe_state_options_hud_ctor_2,
        "cFEStateOptionsHUD_ctor_2",
    ),
    (
        0x001d_e050,
        cfe_state_options_hud_dtor,
        "cFEStateOptionsHUD_dtor",
    ),
    (
        0x001d_e080,
        cfe_state_options_controller_ctor_2,
        "cFEStateOptionsController_ctor_2",
    ),
    (
        0x001d_e0c0,
        cfe_state_options_controller_dtor,
        "cFEStateOptionsController_dtor",
    ),
    (
        0x001d_e0f0,
        cfe_state_options_save_load_ctor_2,
        "cFEStateOptionsSaveLoad_ctor_2",
    ),
    (
        0x001d_e128,
        cfe_state_options_save_load_dtor,
        "cFEStateOptionsSaveLoad_dtor",
    ),
    (
        0x001d_e150,
        cfe_state_options_device_select_dtor,
        "cFEStateOptionsDeviceSelect_dtor",
    ),
    (
        0x001d_e178,
        cfe_state_options_device_select_ctor_2,
        "cFEStateOptionsDeviceSelect_ctor_2",
    ),
    (
        0x0018_8870,
        cfe_state_options_on_widget_created,
        "cFEStateOptions_OnWidgetCreated",
    ),
    (
        0x0018_8980,
        cfe_state_options_on_widget_action,
        "cFEStateOptions_OnWidgetAction",
    ),
    (
        0x0018_8c68,
        cfe_state_options_on_widget_notify,
        "cFEStateOptions_OnWidgetNotify",
    ),
    (
        0x0018_8d08,
        cfe_state_options_on_child_event,
        "cFEStateOptions_OnChildEvent",
    ),
    (
        0x0018_8f28,
        oc833_fe_state_options_game_init,
        "cFEStateOptionsGame_Init",
    ),
    (
        0x0018_95f0,
        cfe_state_options_game_on_activate,
        "cFEStateOptionsGame_OnActivate",
    ),
    (
        0x0018_9610,
        cfe_state_options_game_on_child_event,
        "cFEStateOptionsGame_OnChildEvent",
    ),
    (
        0x0018_9980,
        cfe_state_options_game_on_widget_action,
        "cFEStateOptionsGame_OnWidgetAction",
    ),
    (
        0x0018_9d60,
        cfe_state_options_game_refresh_list_boxes,
        "cFEStateOptionsGame_RefreshListBoxes",
    ),
    (
        0x0018_a298,
        c_fe_state_options_sound_init,
        "cFEStateOptionsSound_Init",
    ),
    (
        0x0018_a3d8,
        c_fe_state_options_sound_on_child_event,
        "cFEStateOptionsSound_OnChildEvent",
    ),
    (
        0x0018_a870,
        c_fe_state_options_sound_on_activate,
        "cFEStateOptionsSound_OnActivate",
    ),
    (
        0x0018_a8d0,
        cfe_state_options_sound_on_widget_action,
        "cFEStateOptionsSound_OnWidgetAction",
    ),
    (
        0x0018_b380,
        cfe_state_options_sound_refresh_controls,
        "cFEStateOptionsSound_RefreshControls",
    ),
    (
        0x0018_bef8,
        c_fe_state_options_sound_set_speaker_mode,
        "cFEStateOptionsSound_SetSpeakerMode",
    ),
    (
        0x0018_c198,
        c_fe_state_options_hud_on_activate,
        "cFEStateOptionsHUD_OnActivate",
    ),
    (
        0x0018_c2a0,
        c_fe_state_options_hud_on_widget_action,
        "cFEStateOptionsHUD_OnWidgetAction",
    ),
    (
        0x0018_c478,
        oc838_options_hud_refresh_selection,
        "cFEStateOptionsHUD_RefreshSelection",
    ),
    (
        0x0018_c6a0,
        oc838_options_controller_init,
        "cFEStateOptionsController_Init",
    ),
    (
        0x0018_c910,
        oc838_options_controller_on_child_event,
        "cFEStateOptionsController_OnChildEvent",
    ),
    (
        0x0018_cce0,
        oc838_options_controller_on_widget_notify,
        "cFEStateOptionsController_OnWidgetNotify",
    ),
    (
        0x0018_cdc0,
        oc838_options_controller_refresh_list_boxes,
        "cFEStateOptionsController_RefreshListBoxes",
    ),
    (
        0x0018_cf50,
        oc838_options_controller_show_config_text,
        "cFEStateOptionsController_ShowConfigText",
    ),
    (
        0x0018_d108,
        c_fe_state_options_controller_set_vibration,
        "cFEStateOptionsController_SetVibration",
    ),
    (
        0x0018_d2e8,
        c_fe_state_options_save_load_on_widget_created,
        "cFEStateOptionsSaveLoad_OnWidgetCreated",
    ),
    (
        0x0018_d460,
        c_fe_state_options_save_load_on_child_event,
        "cFEStateOptionsSaveLoad_OnChildEvent",
    ),
    (
        0x0018_d648,
        c_fe_state_options_save_load_on_widget_action,
        "cFEStateOptionsSaveLoad_OnWidgetAction",
    ),
    (
        0x0018_d868,
        c_fe_state_options_device_select_init,
        "cFEStateOptionsDeviceSelect_Init",
    ),
    (
        0x0018_dea0,
        c_fe_state_options_device_select_on_widget_action,
        "cFEStateOptionsDeviceSelect_OnWidgetAction",
    ),
    (
        0x0018_e368,
        c_fe_state_options_device_select_v39,
        "cFEStateOptionsDeviceSelect_v39",
    ),
    (
        0x0018_e478,
        cfe_state_options_device_select_v33_2,
        "cFEStateOptionsDeviceSelect_v33",
    ),
    (
        0x0018_e510,
        oc841_device_select_display_status,
        "cFEStateOptionsDeviceSelect_v26",
    ),
    (
        0x0018_ea98,
        fe_screen_select_profile_ctor,
        "FEScreenSelectProfile_ctor",
    ),
    (
        0x001d_dfa8,
        c_fe_state_options_game_on_deactivate_001ddfa8,
        "cFEStateOptionsGame_OnDeactivate",
    ),
    (
        0x001d_e010,
        c_fe_state_options_sound_on_deactivate_001de010,
        "cFEStateOptionsSound_OnDeactivate",
    ),
    (
        0x001d_e078,
        c_fe_state_options_hud_on_deactivate_001de078,
        "cFEStateOptionsHUD_OnDeactivate",
    ),
    (
        0x001d_e0e8,
        c_fe_state_options_controller_on_deactivate_001de0e8,
        "cFEStateOptionsController_OnDeactivate",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::bx::bxstringctor::callee::OPTIONS_SET_VIBRATION;

    pub(crate) use crate::game::camera::camera::callee::UI_STATE_CTOR;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR_3;

    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;

    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;

    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;

    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR as UI_STATE_CTOR_2;

    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;

    pub(crate) use crate::game::main::gamemode::callee::STR_HASH as STR_HASH_2;

    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_BASE_CTOR;

    pub(crate) const SET_VIBRATION: Callee = Callee {
        addr: 0x0018_d108,
        f: ee::s02::f_0018d108,
    };

    pub(crate) const SHOW_CONFIG_TEXT: Callee = Callee {
        addr: 0x0018_cf50,
        f: ee::s02::f_0018cf50,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_ADD_OBJECT;
}

// ---------------------------------------------------------------------------
// cFEState constructors.

/// `cFEStateOptionsGame* ctor(this)` (0x188ee8): base `UIState_ctor`, then the
/// state vtable at `+0x8` (`D_0046C6F0`) and id 0x1c at `+0xc`.
pub fn c_fe_state_options_game_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::UI_STATE_CTOR.call(c, 0x0018_8efc)?;
        c.g[V0][0] = 0x1c;
        c.g[V1][0] = 0x0047_0000;
        let id = c.g[V0][0];
        c.sw((this as u32).wrapping_add(0xc), id);
        c.g[V1][0] = sx(0x0046_c6f0);
        let vt = c.g[V1][0];
        c.sw((this as u32).wrapping_add(0x8), vt);
        c.g[V0][0] = this;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end state constructors.

/// `void* cFEStateOptionsSound_ctor(void* self)` (0x18a258): UI state with
/// class id 0x1d, its vtable, and the global "sound changed" flag cleared.
pub fn c_fe_state_options_sound_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_STATE_CTOR_2.call(c, 0x0018_a26c)?;
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = 0x1d;
        c.g[V1][0] = 0x47_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_sub(0x39e0));
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(crate::abi::r32(c, GP).wrapping_sub(0x1c1c)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEStateOptionsHUD_ctor(void* self)` (0x18c0e8): class id 0x1e.
pub fn c_fe_state_options_hud_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_STATE_CTOR_2.call(c, 0x0018_c0fc)?;
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = 0x1e;
        c.g[V1][0] = 0x47_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_sub(0x3ab0));
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(crate::abi::r32(c, GP).wrapping_sub(0x1c18)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEStateOptionsSaveLoad_ctor(void* self)` (0x18d240): class id
/// 0x20, `+0x48` (the "scanning" flag) cleared.
pub fn c_fe_state_options_save_load_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_STATE_CTOR_2.call(c, 0x0018_d254)?;
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = 0x20;
        c.g[V1][0] = 0x47_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_sub(0x3c50));
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(this, 0x48)).sb(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOptionsController* cFEStateOptionsController_ctor(self)`
/// (0x18c658): UI state constructor, then the options-controller vtable and
/// two cleared fields; clears the "options open" global.
pub fn fe_state_options_controller_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_CTOR_3, 0x0018_c66c)?;
        let this = r32(c, S0);
        c.g[V0][0] = 0x1f;
        const_reg(c, V1, 0x47_0000, -0x3b80);
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sb(p_2(this, 0x50), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        sw(c, gp(c, -0x1c14), 0, 0);
        leave(c, 0x20);
        Ok(())
    });
}

/// `void cFEStateOptionsController_Shutdown(cFEStateOptionsController* s)`
/// (0x18c8b8): turn both vibration channels off and deactivate the state.
pub fn cfe_state_options_controller_shutdown(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x30);
    c.g[A1][0] = 0;
    frame.save_fpr(c, 20, 0x20);
    frame.save(c, S0, 0x10);
    c.mtc1(0, 20);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.mov_s(12, 20);
    c.mov_s(13, 20);
    if callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION
        .call(c, 0x0018_c8e0)
        .is_err()
    {
        return;
    }
    c.mov_s(12, 20);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 1;
    c.mov_s(13, 12);
    if callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION
        .call(c, 0x0018_c8f4)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    if call_ptr(c, 0x0039_e4a0, 0x0018_c8fc).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x20);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Small helpers.

/// A `gp`-relative global address.
#[inline(always)]
pub(crate) fn gpa(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Front-end option screens (fe/feoptions.cpp).

/// `int cFEStateOptions_HandleInput(cFEStateOptions* self, FEUIInput* input)`
/// (0x188c58): latch the raw pad byte at `input+0x4` into the global input
/// word (`gp+0x394`) and return 0.
pub fn cfe_state_options_handle_input(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x4)).lbu(c);
    c.g[V0][0] = 0;
    c.sw(gpa(c, 0x394), c.g[V1][0]);
    abi::ret(c);
}

/// `int cFEStateOptionsGame_OnWidgetNotify(void*, int, int id)`
/// (0x189958): the game-options page handles the "back" widget (9) with
/// 0x100 and everything else with 0x101.
pub fn cfe_state_options_game_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] ^= 0x9;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `int cFEStateOptionsGame_v21(void*, int, int id)` (0x189970): 0x100 when
/// an id was passed, 0 otherwise.
pub fn cfe_state_options_game_v21(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x100;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `int cFEStateOptionsSound_OnWidgetNotify(void*, int, int id)`
/// (0x18a890): id 6 is ignored (0), id 9 is handled (0x100), anything else
/// is not for this page (0x101).
pub fn cfe_state_options_sound_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x6;
    let is6 = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 0x9;
    if is6 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let not_back = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = 0x100;
    if not_back {
        c.cycles += 2;
        c.g[V0][0] = 0x101;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cFEStateOptionsSound_v21(void*, int, int id)` (0x18a8c0): 0x100 when
/// an id was passed, 0 otherwise.
pub fn cfe_state_options_sound_v21(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x100;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `int cFEStateOptionsHUD_OnWidgetNotify(void*, int, int id)` (0x18c270):
/// as [`cfe_state_options_sound_on_widget_notify`] for the HUD page.
pub fn cfe_state_options_hud_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x6;
    let is6 = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 0x9;
    if is6 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let not_back = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = 0x100;
    if not_back {
        c.cycles += 2;
        c.g[V0][0] = 0x101;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cFEStateOptionsController_v21(void*, int, int id)` (0x18cdb0):
/// 0x100 when an id was passed, 0 otherwise.
pub fn cfe_state_options_controller_v21(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x100;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `int cFEStateOptionsSaveLoad_OnWidgetNotify(void*, int, int key)`
/// (0x18d438): keys 8 and 9 are handled (0x100), everything else ignored
/// (0x101); the two comparisons are done as unsigned `< 10` / `< 8`.
pub fn cfe_state_options_save_load_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = (c.g[A2][0] < 0xau64) as u64;
    let out_of_range = c.g[V1][0] == 0;
    c.g[V0][0] = 0x101;
    if out_of_range {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[A0][0] = (c.g[A2][0] < 0x8u64) as u64;
    c.g[V1][0] = 0x101;
    c.g[V0][0] = 0x100;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `cFEStateOptionsSaveLoad_Init(cFEStateOptionsSaveLoad* self)` (0x18d280).
pub fn c_fe_state_options_save_load_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0045_e0c0, [0x0018_d2a4, 0x0018_d2b8, 0x0018_d2d0])
    })
}

/// `cFEStateOptionsHUD::Init(self)` (0x18c128): create the options HUD screen,
/// start it and refresh the selected row.
pub fn c_fe_state_options_hud_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fc8));
        c.g[S0][0] = Addr(p(c.g[S1][0] as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0018_c14c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_c160)?;
        let missing = c.g[V0][0] == 0;
        Addr(p(c.g[S1][0] as u32, 0x40)).sw(c, c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_c178)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0018_c478, 0x0018_c180)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOptions_Init(cFEStateOptions* s)` (0x188800): create the
/// options screen from the hash name and start its frame.
pub fn c_fe_state_options_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 17, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sqsp(c, 16, 0x20);
        c.g[A0][0] = 0x0045_dd30;
        c.g[S0][0] = Addr(p_3(r32(c, S1), 0x10)).lw(c);
        call::call_ptr(c, 0x0031_7670, 0x0018_8824)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call::call_ptr(c, 0x0039_7cc0, 0x0018_8838)?;
        let no_screen = c.g[V0][0] == 0;
        Addr(p_3(r32(c, S1), 0x40)).sw(c, c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call::call_ptr(c, 0x0039_c870, 0x0018_8850)?;
        }
        c.cycles += 8;
        c.g[V0][0] = 1;
        Addr(p_3(r32(c, S1), 0x48)).sw(c, 0);
        Addr(p_3(r32(c, S1), 0x4c)).sw(c, c.g[V0][0]);
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `cFEStateOptionsDeviceSelect_ctor` (0x18D7F0): construct the device-select
/// options state, remembering its two handlers and clearing the slot state.
pub fn c_fe_state_options_device_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A3][0];
        callee::MEM_CARD_BASE_CTOR.call(c, 0x0018_d814)?;
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = 0x21;
        c.g[V1][0] = low(c, V1).wrapping_sub(0x3dd8);
        sw_at(c, S2, 0x230, c.g[S0][0]);
        sw_at(c, S2, 0x8, c.g[V1][0]);
        sw_at(c, S2, 0xc, c.g[V0][0]);
        sw_at(c, S2, 0x22c, c.g[S1][0]);
        sw_at(c, S2, 0x234, 0);
        let no_handler = c.g[S1][0] == 0;
        sw_at(c, S2, 0x238, 0);
        if no_handler {
            c.cycles += 3;
            c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
            call_ptr(c, addr::WORLD_GET, 0x0018_d844)?;
            sw_at(c, V0, 0x11c, 0);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOptionsSaveLoad_OnRender(self)` (0x18d3c0): when the options
/// screen is live, reveal the two option widgets found by name hash.
pub fn c_fe_state_options_save_load_on_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x20));
        c.g[V0][0] = Addr(gpo(c, -0x1718)).lw(c);
        c.sq(16, sp_2(c).wrapping_add(0x10));
        c.sd(sp_2(c), c.g[RA][0]);
        let off = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if off {
            c.cycles += 1;
            c.lq(16, sp_2(c).wrapping_add(0x10));
        } else {
            c.cycles += 8;
            c.g[A0][0] = 0x46_0000;
            c.g[A0][0] = sx(low_2(c, A0).wrapping_sub(0x1f10));
            call_ptr(c, callee::STR_HASH_2, 0x0018_d3e4)?;
            c.g[A0][0] = lw_r(c, S0, 0x40);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME, 0x0018_d3f0)?;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[V0][0];
                callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF.call(c, 0x0018_d400)?;
            }
            c.g[A0][0] = 0x46_0000;
            c.cycles += 7;
            c.g[A0][0] = sx(low_2(c, A0).wrapping_sub(0x1f00));
            call_ptr(c, callee::STR_HASH_2, 0x0018_d40c)?;
            c.g[A0][0] = lw_r(c, S0, 0x40);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME, 0x0018_d418)?;
            let none = c.g[V0][0] == 0;
            c.lq(16, sp_2(c).wrapping_add(0x10));
            if none {
                c.cycles += 3;
                c.g[RA][0] = Addr(sp_2(c)).ld(c);
                let t = low_2(c, RA);
                c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x20));
                c.pc = t;
                return Ok(());
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[V0][0];
            callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF.call(c, 0x0018_d428)?;
            c.cycles += 1;
            c.lq(16, sp_2(c).wrapping_add(0x10));
        }
        c.cycles += 3;
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low_2(c, RA);
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine teardown.

/// `void cFEStateOptions_OpenResetPopup(self)` (0x188e58): allocate and build
/// the reset-options popup screen object, add it to the engine's screen list
/// and arm the global "reset" flags.
pub fn c_fe_state_options_open_reset_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        c.g[A1][0] = 0x0046_0000;
        frame.save(c, S1, 0x10);
        c.g[A3][0] = 0;
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x2298));
        c.g[A0][0] = 0x4c;
        c.g[A2][0] = 0x100;
        call_ptr(c, callee::MEM_ALLOC, 0x0018_8e84)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(at(c, S1, 0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, callee::UI_OBJECT_CTOR_2, 0x0018_8e98)?;
        c.g[A0][0] = c.lw(at(c, S1, 0x10));
        c.g[V0][0] = 0x0047_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4c30));
        c.g[A1][0] = c.g[S0][0];
        c.sw(at(c, S0, 8), c.g[V0][0]);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
        c.sb(at(c, S0, 0x48), 0);
        call_ptr(c, callee::UI_OBJECT_ADD_CHILD, 0x0018_8eb8)?;
        c.g[V0][0] = 1;
        c.sw(gp(c, -0x1c18), 0);
        c.sw(gp(c, -0x1c10), c.g[V0][0]);
        c.sw(gp(c, -0x1c14), 0);
        c.sw(gp(c, -0x1c1c), 0);
        c.sw(gp(c, -0x1c20), 0);
        c.sw(at(c, S1, 0x4c), 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOptionsDeviceSelect_v33(self)` (0x18e478): open the device
/// select popup once, with the mode selected from the four option flags.
pub fn cfe_state_options_device_select_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_2(c, S0, 0x1ac);
        let done = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !done {
            c.cycles += 21;
            c.g[V1][0] = lw_2(c, S0, 0x10);
            c.g[T2][0] = 2;
            c.g[A2][0] = lw_2(c, S0, 0x1bc);
            c.g[A1][0] = c.g[S0][0];
            c.g[T1][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[V0][0] = lw_2(c, S0, 0x1e4);
            c.g[A2][0] = c.g[A2][0] ^ 1;
            c.g[T0][0] = lw_2(c, T1, 0x4);
            c.g[A2][0] = (c.g[A2][0] < 1) as u64;
            c.g[V0][0] = c.g[V0][0] ^ 1;
            c.g[V1][0] = lw_2(c, S0, 0x1b4);
            if c.g[V0][0] == 0 {
                c.g[A2][0] = 0;
            }
            c.g[A3][0] = lw_2(c, S0, 0x230);
            c.g[A0][0] = Addr(c.g[T0][0] as u32).off(0x28).lh(c);
            if c.g[V1][0] == 0 {
                c.g[A2][0] = 0;
            }
            c.g[V0][0] = lw_2(c, T0, 0x2c);
            if c.g[A3][0] != 0 {
                c.g[A2][0] = c.g[T2][0];
            }
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0018_e4e0)?;
            let created = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[V0][0];
            if !created {
                c.cycles += 3;
                c.g[A0][0] = lw_2(c, S0, 0x10);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, table_callee::UI_SCREEN_ADD_OBJECT, 0x0018_e4f4)?;
            }
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        Addr(c.g[S0][0] as u32).off(0x1ac).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateOptions_OnWidgetNotify(cFEState*, void* widget, int event,
/// int index)` (0x188c68): map the widget events 6/7 onto the page's flag
/// word, or return 0x100/0x101 for the other ids.
pub fn c_fe_state_options_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        c.g[V0][0] = (c.g[S1][0] < 0xa) as u64;
        let big = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        let mut done = false;
        if !big {
            c.cycles += 3;
            c.g[V0][0] = (c.g[S1][0] < 8) as u64;
            let low = c.g[V0][0] != 0;
            c.g[V0][0] = 6;
            if !low {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
                done = true;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 6;
        }
        if !done {
            c.cycles += 2;
            let not_six = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 7;
            let mut hit = false;
            if !not_six {
                c.cycles += 9;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
                c.g[A1][0] = c.g[A3][0];
                call_ptr(c, 0x0039_7870, 0x0018_8cb8)?; // UIList_GetAt
                c.g[A0][0] = 0x004a_0000;
                c.g[S0][0] = lw_3(c, p(c.g[V0][0] as u32, 0x38));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1530));
                call_ptr(c, 0x0031_7670, 0x0018_8cc8)?; // StrHash
                if c.g[S0][0] == c.g[V0][0] {
                    c.g[A0][0] = lw_3(c, p(c.g[S2][0] as u32, 0x4c));
                    hit = true;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 7;
                }
            }
            if hit {
                c.cycles += 3;
                c.g[V1][0] = 0x101;
                c.g[V0][0] = 1;
                if lw_3(c, c.g[A0][0] as u32) != 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
            } else {
                c.cycles += 2;
                if c.g[S1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0x101;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = lw_3(c, p(c.g[S2][0] as u32, 0x4c));
                    c.cycles += 3;
                    c.g[V1][0] = 0x101;
                    c.g[V0][0] = 1;
                    if lw_3(c, c.g[A0][0] as u32) != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
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

/// `void cFEStateOptionsDeviceSelect_v38(self, flag)` (0x18e2c0): if the
/// state's object exists, tell both device-select items the new selected
/// flag and store the inverted flag.
pub fn cfe_state_options_device_select_v38(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        c.cycles += 9;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, at_3(c.g[S1][0], 0x22c));
        let t = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !t {
            c.cycles += 25;
            c.g[A0][0] = sx(0x004a_0000);
            c.g[S0][0] = c.g[S2][0] ^ 1;
            c.g[RA][0] = sx(0x0018_e2f4);
            c.g[A0][0] = sx(0x004a_1458);
            call_ptr(c, 0x0031_7670, 0x0018_e2f4)?;
            c.g[A0][0] = lw(c, at_3(c.g[S1][0], 0x40));
            c.g[RA][0] = sx(0x0018_e300);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_e300)?;
            c.g[V1][0] = lw(c, at_3(c.g[V0][0], 8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = lh(c, at_3(c.g[V1][0], 0x48));
            c.g[A2][0] = lw(c, at_3(c.g[V1][0], 0x4c));
            let target = c.g[A2][0] as u32;
            c.g[RA][0] = sx(0x0018_e318);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_e318)?;
            c.g[A0][0] = sx(0x004a_0000);
            c.g[RA][0] = sx(0x0018_e324);
            c.g[A0][0] = sx(0x004a_1460);
            call_ptr(c, 0x0031_7670, 0x0018_e324)?;
            c.g[A0][0] = lw(c, at_3(c.g[S1][0], 0x40));
            c.g[RA][0] = sx(0x0018_e330);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_e330)?;
            c.g[V1][0] = lw(c, at_3(c.g[V0][0], 8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = lh(c, at_3(c.g[V1][0], 0x48));
            c.g[A2][0] = lw(c, at_3(c.g[V1][0], 0x4c));
            let target = c.g[A2][0] as u32;
            c.g[RA][0] = sx(0x0018_e348);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_e348)?;
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0] ^ 1;
        frame.restore(c, S0, 0x30);
        Addr(at_3(c.g[S1][0], 0x220)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateOptionsController_Update` (0x18d058): update the screen, then tick
// down the two vibration timers; an expired timer clears its bit and tells
// the controller to stop vibrating that motor. The returned bits select the
// motors.
pub fn cfe_state_options_controller_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039e510, 0x0018_d06c)?;
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.g[V1][0] & 0x1;
        let no_first = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[V1][0] & 0x2;
        if !no_first {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            let expired = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            if !expired {
                c.cycles += 2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
            } else {
                c.cycles += 8;
                c.mtc1(0, 12);
                c.g[V0][0] = c.g[V1][0] & 0xfe;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.g[A0][0] = c.g[S0][0];
                c.sb((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                c.g[A1][0] = 0u64;
                c.mov_s(13, 12);
                callee::OPTIONS_SET_VIBRATION.call(c, 0x0018_d0b0)?;
            }
            c.cycles += 2;
            c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.g[V1][0] & 0x2;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lq(16, frame.at(c, 0x10));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
            let expired = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            if !expired {
                c.cycles += 2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
            } else {
                c.cycles += 8;
                c.mtc1(0, 12);
                c.g[V0][0] = c.g[V1][0] & 0xfd;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[A0][0] = c.g[S0][0];
                c.sb((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                c.g[A1][0] = 0x1;
                c.mov_s(13, 12);
                callee::OPTIONS_SET_VIBRATION.call(c, 0x0018_d0f4)?;
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x10));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOptionsHUD_OnChildEvent(cFEStateOptionsHUD* s, void* child,
/// int event)` (0x18c1b8): apply the option row's change when the HUD's child
/// screen reports the selection event.
pub fn fe_state_options_hud_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        'outer: {
            'done: {
                frame.save(c, S0, 0x40);
                frame.save(c, S1, 0x30);
                let event = r(c, A2);
                c.g[S0][0] = sx(event);
                frame.save(c, S2, 0x20);
                c.g[S1][0] = c.g[A1][0];
                let this = r(c, A0);
                frame.save(c, S3, 0x10);
                frame.save_ra(c, 0);
                c.g[S3][0] = sx(this);
                call_ptr(c, 0x0014_dc80, 0x0018_c1e0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0018_c1ec);
                c.g[A1][0] = 4;
                call_ptr(c, callee::BE_GET_INTERFACE, 0x0018_c1ec)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = 0x15;
                let open = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 0x16;
                if open {
                    break 'done;
                }
                c.cycles += 2;
                let not_close = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, 16, 0x40);
                if not_close {
                    break 'outer;
                }
                c.cycles += 6;
                c.g[A0][0] = sx(0x0046_0000);
                c.g[S0][0] = Addr(p(r(c, S1), 0xc)).lw(c);
                c.g[RA][0] = sx(0x0018_c214);
                c.g[A0][0] = sx(0x0045_dd20);
                call_ptr(c, callee::FE_STRING_HASH, 0x0018_c214)?;
                let differs = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, 16, 0x40);
                if differs {
                    break 'outer;
                }
                c.cycles += 8;
                c.g[V1][0] = Addr(p(r(c, S1), 8)).lw(c);
                c.g[A1][0] = 2;
                c.g[A0][0] = Addr(p(r(c, V1), 0xb8)).lh(c);
                c.g[V0][0] = Addr(p(r(c, V1), 0xbc)).lw(c);
                let target = r(c, V0);
                c.g[RA][0] = sx(0x0018_c234);
                c.g[A0][0] = sx(p(r(c, S1), r(c, A0)));
                call_ptr(c, target, 0x0018_c234)?;
                let empty = c.g[V0][0] == 0;
                frame.restore(c, 16, 0x40);
                if empty {
                    break 'outer;
                }
                c.cycles += 6;
                c.g[RA][0] = sx(0x0018_c244);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0014_f600, 0x0018_c244)?;
                c.g[RA][0] = sx(0x0018_c24c);
                c.g[A0][0] = c.g[S3][0];
                callee::OPTIONS_HUD_REFRESH_SELECTION.call(c, 0x0018_c24c)?;
                c.g[V0][0] = 1;
                Addr(gp_2(c, -0x1c18)).sw(c, c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateOptionsDeviceSelect_OnWidgetNotify(self, void *widget,
/// int id)` (0x18ddd0): map a widget id to the menu's return code (0x100 for
/// "previous", 0x101 otherwise); for ids 6/8 it also notifies the selected
/// device entry.
pub fn cfe_state_options_device_select_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = sx(world(c));
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        call_ptr(c, 0x0022_7f80, 0x0018_ddf4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = 0x8;
        let is8 = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[S0][0] < 9) as u64;
        if is8 {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_4(c.g[S2][0], 0x22c)).lw(c);
            c.g[A2][0] = Addr(p_4(c.g[S1][0], 0x434)).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 8;
                c.g[A1][0] = Addr(p_4(c.g[S1][0], 0x428)).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(p_4(c.g[V1][0], 0x1b0)).lh(c);
                c.g[V0][0] = Addr(p_4(c.g[V1][0], 0x1b4)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0018_de54)?;
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = 0x101;
                if nonzero {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x30);
                    c.cycles += 5;
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 1;
                c.g[A2][0] = Addr(p_4(c.g[S1][0], 0x434)).lw(c);
            } else {
                c.g[A2][0] = Addr(p_4(c.g[S1][0], 0x434)).lw(c);
            }
            // The shared device-entry notification (label b_18de60).
            c.cycles += 8;
            c.g[A1][0] = Addr(p_4(c.g[S1][0], 0x428)).lw(c);
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(p_4(c.g[V1][0], 0x1b0)).lh(c);
            c.g[V0][0] = Addr(p_4(c.g[V1][0], 0x1b4)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_de78)?;
            let nonzero = c.g[V0][0] != 0;
            c.g[V0][0] = 0x101;
            if nonzero {
                c.cycles += 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
            }
            frame.restore(c, S0, 0x30);
        } else if c.g[S0][0] >= 9 {
            c.cycles += 2;
            c.cycles += 3;
            if c.g[S0][0] == 9 {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x101;
            }
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 2;
            if c.g[S0][0] == 6 {
                c.cycles += 2;
                c.cycles += 1;
                c.g[A2][0] = Addr(p_4(c.g[S1][0], 0x434)).lw(c);
                c.cycles += 8;
                c.g[A1][0] = Addr(p_4(c.g[S1][0], 0x428)).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(p_4(c.g[V1][0], 0x1b0)).lh(c);
                c.g[V0][0] = Addr(p_4(c.g[V1][0], 0x1b4)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0018_de78)?;
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = 0x101;
                if nonzero {
                    c.cycles += 1;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x100;
                }
                frame.restore(c, S0, 0x30);
            } else {
                c.cycles += 2;
                c.cycles += 2;
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Screen-object helpers.
//
// The front-end states look widgets up by hashed name and then call a method
// on the found object through its vtable (`lh vt+0x48` delta, `lw vt+0x4c`
// function).

/// `int StrHash(const char*)`.
pub(crate) const STR_HASH: u32 = 0x0031_7670;

/// `UIObject *UIScreen_FindObject(UIScreen*, int hash)`.
pub(crate) const UI_SCREEN_FIND_OBJECT: u32 = 0x0039_d860;

/// `StrHash(name)`, then `UIScreen_FindObject(*(this+0x40), hash)`; the found
/// object is left in `v0`.
pub(crate) fn screen_object(
    c: &mut Cpu,
    this: u32,
    name: u32,
    ret_hash: u32,
    ret_find: u32,
) -> Flow {
    c.g[A0][0] = name as u64;
    call_ptr(c, STR_HASH, ret_hash)?;
    c.g[A0][0] = Addr(this).off(0x40).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, UI_SCREEN_FIND_OBJECT, ret_find)
}

/// Call the method at vtable slot `+0x4c` of `*(obj+8)` with `a1 = arg`
/// (the function pointer is fetched into `a2`, as the original does).
pub(crate) fn call_object_a2(c: &mut Cpu, obj: u64, arg: u64, ret: u32) -> Flow {
    c.g[V1][0] = Addr(obj as u32).off(8).lw(c);
    c.g[A1][0] = arg;
    c.g[A0][0] = Addr(r32(c, V1)).off(0x48).lh(c);
    c.g[A2][0] = Addr(r32(c, V1)).off(0x4c).lw(c);
    let func = c.g[A2][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}

// ---------------------------------------------------------------------------
// cFEStateOptionsDeviceSelect_v37 / cFEStateProfileLoad_v37.

/// The UIState's frame (0x40, `s0`/`s1`/`s2` at +0x30/+0x20/+0x10).
pub(crate) fn ui_state_frame(c: &mut Cpu) -> Frame {
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x30);
    frame.save(c, S2, 0x10);
    frame.save(c, S1, 0x20);
    frame.save_ra(c, 0);
    frame
}

pub(crate) fn ui_state_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cFEStateOptionsDeviceSelect_v37(FEState *s, int selected)` (0x18e178):
/// re-apply the state `selected` to the three device/controller widgets and
/// record it in `+0x234`.
pub fn c_fe_state_options_device_select_v37(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = ui_state_frame(c);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[S2][0] = abi::arg64(c, 1);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x234).lw(c);
        let same = c.g[V0][0] == c.g[S2][0];
        c.g[S1][0] = c.g[S2][0] ^ 1;
        let this = r32(c, S0);
        let arg = c.g[S1][0];
        if !same {
            c.cycles += 15;
            screen_object(c, this, 0x004a_15e0, 0x0018_e1ac, 0x0018_e1b8)?;
            let obj = c.g[V0][0];
            call_object_a2(c, obj, arg, 0x0018_e1d0)?;

            c.g[V0][0] = Addr(this).off(0x22c).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 12;
                screen_object(c, this, 0x004a_15d8, 0x0018_e27c, 0x0018_e288)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_e2a0)?;
                Addr(this).off(0x234).sw(c, c.g[S2][0]);
            } else {
                c.cycles += 38;
                screen_object(c, this, 0x004a_15d0, 0x0018_e1e8, 0x0018_e1f4)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_e20c)?;
                screen_object(c, this, 0x004a_1458, 0x0018_e218, 0x0018_e224)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_e23c)?;
                screen_object(c, this, 0x004a_1460, 0x0018_e248, 0x0018_e254)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_e26c)?;
                Addr(this).off(0x234).sw(c, c.g[S2][0]);
            }
        }
        ui_state_epilogue(c, frame);
        Ok(())
    })
}

/// `jalr` through a vtable entry (the target is data).
#[inline]
pub(crate) fn call_vtable(c: &mut Cpu, target: u32, ret: u32) -> Flow {
    c.g[RA][0] = sx(ret);
    c.pc = target;
    call_indirect(c, target);
    returned(c, ret)
}

/// The vibration timer a checked option turns on: one second at 60 Hz.
const VIBRATION_FRAMES: u32 = 60;

/// `void cFEStateOptionsController_OnWidgetAction(self, widget, event)`
/// (0x18ca10): the options screen's widget actions. Event 7 closes a popup,
/// event 6 opens the config popup, event 9 toggles vibration (ids
/// `D_004A1568`/`D_004A1570`, timers `+0x48`/`+0x4c`, flags `+0x50` bits 0/1)
/// and switches the config text (`D_004A15C8`/`D_004A1578`).
pub fn fe_options_controller_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_18cccc: {
            'b_18ccc8: {
                'b_18ccc4: {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S0][0] = c.g[A2][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S2][0] = c.g[A0][0];
                    let t = c.g[S1][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_18ccc4;
                    } else {
                        'b_18cb34: {
                            'b_18cadc: {
                                'b_18ca88: {
                                    c.cycles += 9;
                                    c.g[RA][0] = sx(0x0018ca40);
                                    call_vtable(c, 0x0014dc80, 0x0018ca40)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x0018ca4c);
                                    c.g[A1][0] = 0x1;
                                    call_vtable(c, 0x0014dd58, 0x0018ca4c)?;
                                    c.g[S3][0] = c.g[V0][0];
                                    c.g[V0][0] = 0x7;
                                    let t = c.g[S0][0] == c.g[V0][0];
                                    c.g[V0][0] = (c.g[S0][0] < 0x8u64) as u64;
                                    if t {
                                        break 'b_18ca88;
                                    } else {
                                        'b_18ca74: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x6;
                                            if t {
                                                break 'b_18ca74;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_18cadc;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_18cccc;
                                                    } else {
                                                        break 'b_18ca74;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x9;
                                        let t = c.g[S0][0] == c.g[V0][0];
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        if t {
                                            break 'b_18cb34;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_18cccc;
                                            } else {
                                                break 'b_18ca88;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 21;
                                c.g[A1][0] = 0x460000;
                                c.g[A0][0] = 0x4c;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2298));
                                c.g[A2][0] = 0x100;
                                c.g[RA][0] = sx(0x0018caa0);
                                c.g[A3][0] = 0u64;
                                call_vtable(c, 0x00317d70, 0x0018caa0)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0018cab4);
                                c.g[A2][0] = c.g[S2][0];
                                call_vtable(c, 0x0039e318, 0x0018cab4)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = 0x470000;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4c30));
                                c.g[A1][0] = c.g[S0][0];
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                c.g[RA][0] = sx(0x0018cad4);
                                c.sb((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                                call_vtable(c, 0x0039f290, 0x0018cad4)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_18ccc8;
                                } else {
                                    break 'b_18cadc;
                                }
                            }
                            c.cycles += 16;
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x0018caf0);
                            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_vtable(c, t, 0x0018caf0)?;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0018cb14);
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_vtable(c, t, 0x0018cb14)?;
                            let t = c.g[V0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_18ccc8;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x0018cb2c);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_vtable(c, 0x0039f400, 0x0018cb2c)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_18ccc8;
                                } else {
                                    break 'b_18cb34;
                                }
                            }
                        }
                        'b_18ccc0: {
                            'b_18ccbc: {
                                'b_18cc34: {
                                    'b_18cc24: {
                                        'b_18cbb0: {
                                            c.cycles += 6;
                                            c.g[A0][0] = 0x4a0000;
                                            c.g[S0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                            c.g[RA][0] = sx(0x0018cb44);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x1568));
                                            call_vtable(c, 0x00317670, 0x0018cb44)?;
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[A0][0] = 0x4a0000;
                                            if t {
                                                break 'b_18cbb0;
                                            } else {
                                                'b_18cba4: {
                                                    c.cycles += 11;
                                                    c.g[RA][0] = sx(0x0018cb54);
                                                    c.g[A0][0] = c.g[S1][0];
                                                    call_vtable(c, 0x0039a738, 0x0018cb54)?;
                                                    c.g[A0][0] = c.g[S3][0];
                                                    c.g[A1][0] = 0u64;
                                                    c.g[RA][0] = sx(0x0018cb64);
                                                    c.g[A2][0] = (0u64 < c.g[V0][0]) as u64;
                                                    call_vtable(c, 0x00147528, 0x0018cb64)?;
                                                    c.g[A0][0] = c.g[S3][0];
                                                    c.g[RA][0] = sx(0x0018cb70);
                                                    c.g[A1][0] = 0u64;
                                                    call_vtable(c, 0x001474e8, 0x0018cb70)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    if t {
                                                        break 'b_18cba4;
                                                    } else {
                                                        c.cycles += 11;
                                                        c.g[AT][0] = 0x41200000;
                                                        c.mtc1(1, 12);
                                                        c.g[A1][0] = 0u64;
                                                        c.g[RA][0] = sx(0x0018cb8c);
                                                        c.mov_s(13, 12);
                                                        callee::SET_VIBRATION
                                                            .call(c, 0x0018cb8c)?;
                                                        c.g[V0][0] = c.lbu(
                                                            (c.g[S2][0] as u32).wrapping_add(0x50),
                                                        );
                                                        // The vibration lasts 60 frames (1 s): ticks of the VSync clock.
                                                        c.g[V1][0] =
                                                            crate::tick::ticks_from_frames60(
                                                                c,
                                                                VIBRATION_FRAMES,
                                                            )
                                                                as u64;
                                                        c.sw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x48),
                                                            c.g[V1][0],
                                                        );
                                                        c.g[V0][0] = c.g[V0][0] | 0x1;
                                                        let t = true;
                                                        c.sb(
                                                            (c.g[S2][0] as u32).wrapping_add(0x50),
                                                            c.g[V0][0],
                                                        );
                                                        if t {
                                                            break 'b_18ccbc;
                                                        } else {
                                                            break 'b_18cba4;
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.mtc1(0, 12);
                                                let t = true;
                                                c.g[A1][0] = 0u64;
                                                if t {
                                                    break 'b_18cc24;
                                                } else {
                                                    break 'b_18cbb0;
                                                }
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                        c.g[RA][0] = sx(0x0018cbbc);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1570));
                                        call_vtable(c, 0x00317670, 0x0018cbbc)?;
                                        let t = c.g[S0][0] != c.g[V0][0];
                                        c.g[A0][0] = 0x4a0000;
                                        if t {
                                            break 'b_18cc34;
                                        } else {
                                            'b_18cc1c: {
                                                c.cycles += 11;
                                                c.g[RA][0] = sx(0x0018cbcc);
                                                c.g[A0][0] = c.g[S1][0];
                                                call_vtable(c, 0x0039a738, 0x0018cbcc)?;
                                                c.g[A0][0] = c.g[S3][0];
                                                c.g[A1][0] = 0x1;
                                                c.g[RA][0] = sx(0x0018cbdc);
                                                c.g[A2][0] = (0u64 < c.g[V0][0]) as u64;
                                                call_vtable(c, 0x00147528, 0x0018cbdc)?;
                                                c.g[A0][0] = c.g[S3][0];
                                                c.g[RA][0] = sx(0x0018cbe8);
                                                c.g[A1][0] = 0x1;
                                                call_vtable(c, 0x001474e8, 0x0018cbe8)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = c.g[S2][0];
                                                if t {
                                                    break 'b_18cc1c;
                                                } else {
                                                    c.cycles += 11;
                                                    c.g[AT][0] = 0x41200000;
                                                    c.mtc1(1, 12);
                                                    c.g[A1][0] = 0x1;
                                                    c.g[RA][0] = sx(0x0018cc04);
                                                    c.mov_s(13, 12);
                                                    callee::SET_VIBRATION.call(c, 0x0018cc04)?;
                                                    c.g[V0][0] = c.lbu(
                                                        (c.g[S2][0] as u32).wrapping_add(0x50),
                                                    );
                                                    // One second, in ticks of the VSync clock.
                                                    c.g[V1][0] = crate::tick::ticks_from_frames60(
                                                        c,
                                                        VIBRATION_FRAMES,
                                                    )
                                                        as u64;
                                                    c.sw(
                                                        (c.g[S2][0] as u32).wrapping_add(0x4c),
                                                        c.g[V1][0],
                                                    );
                                                    c.g[V0][0] = c.g[V0][0] | 0x2;
                                                    let t = true;
                                                    c.sb(
                                                        (c.g[S2][0] as u32).wrapping_add(0x50),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_18ccbc;
                                                    } else {
                                                        break 'b_18cc1c;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.mtc1(0, 12);
                                            c.g[A1][0] = 0x1;
                                            break 'b_18cc24;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x0018cc2c);
                                    c.mov_s(13, 12);
                                    callee::SET_VIBRATION.call(c, 0x0018cc2c)?;
                                    let t = true;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_18ccc0;
                                    } else {
                                        break 'b_18cc34;
                                    }
                                }
                                'b_18cc7c: {
                                    c.cycles += 5;
                                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                    c.g[RA][0] = sx(0x0018cc40);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15c8));
                                    call_vtable(c, 0x00317670, 0x0018cc40)?;
                                    let t = c.g[S0][0] != c.g[V0][0];
                                    c.g[A0][0] = 0x4a0000;
                                    if t {
                                        break 'b_18cc7c;
                                    } else {
                                        c.cycles += 13;
                                        c.g[RA][0] = sx(0x0018cc50);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_vtable(c, 0x0039a738, 0x0018cc50)?;
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.g[S3][0];
                                        c.g[RA][0] = sx(0x0018cc60);
                                        c.g[A2][0] = c.g[V0][0];
                                        call_vtable(c, 0x00147658, 0x0018cc60)?;
                                        c.g[RA][0] = sx(0x0018cc68);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_vtable(c, 0x0039a738, 0x0018cc68)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x0018cc74);
                                        c.g[A1][0] = (c.g[V0][0] < 0x1u64) as u64;
                                        callee::SHOW_CONFIG_TEXT.call(c, 0x0018cc74)?;
                                        let t = true;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_18ccc0;
                                        } else {
                                            break 'b_18cc7c;
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                c.g[RA][0] = sx(0x0018cc88);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1578));
                                call_vtable(c, 0x00317670, 0x0018cc88)?;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_18ccc0;
                                } else {
                                    c.cycles += 11;
                                    c.g[RA][0] = sx(0x0018cc98);
                                    c.g[A0][0] = c.g[S1][0];
                                    call_vtable(c, 0x0039a738, 0x0018cc98)?;
                                    c.g[A1][0] = 0x1;
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[RA][0] = sx(0x0018cca8);
                                    c.g[A2][0] = c.g[V0][0];
                                    call_vtable(c, 0x00147658, 0x0018cca8)?;
                                    c.g[RA][0] = sx(0x0018ccb0);
                                    c.g[A0][0] = c.g[S1][0];
                                    call_vtable(c, 0x0039a738, 0x0018ccb0)?;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x0018ccbc);
                                    c.g[A1][0] = (c.g[V0][0] < 0x1u64) as u64;
                                    callee::SHOW_CONFIG_TEXT.call(c, 0x0018ccbc)?;
                                    break 'b_18ccbc;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0x1;
                            break 'b_18ccc0;
                        }
                        c.cycles += 1;
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c14), c.g[V0][0]);
                        break 'b_18ccc4;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_18ccc8;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_18cccc;
        }
        c.cycles += 5;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `UIState` constructor (`UIState_ctor`, 0x39e2a0).
pub(crate) const FE_UI_STATE_CTOR: u32 = 0x0039_e2a0;

/// `UIState_ctor_2` (0x39e318), the constructor with a2 as an argument.
pub(crate) const FE_UI_STATE_CTOR_2: u32 = 0x0039_e318;

/// `UIState` destructor (0x39e390).
pub(crate) const FE_UI_STATE_DTOR: u32 = 0x0039_e390;

/// `MemCardStateBase_ctor` (0x1d53b0).
pub(crate) const FE_MEMCARD_STATE_CTOR: u32 = 0x001d_53b0;

/// `MemCardStateBase_dtor` (0x1d5428).
pub(crate) const FE_MEMCARD_STATE_DTOR: u32 = 0x001d_5428;

/// `cKeyboardPopup_SetKeyEnabled` (0x1ce3c8).
pub(crate) const FE_KEYBOARD_SET_KEY_ENABLED: u32 = 0x001c_e3c8;

/// `strlen` (0x416810).
pub(crate) const FE_STRLEN: u32 = 0x0041_6810;

/// The vtable pointer a constructor stores: `0x470000 - sub`, sign-extended
/// (the `lui`/`addiu` pair that builds the vtable address).
pub(crate) fn fe_vptr(sub: u32) -> u64 {
    sx(0x0047_0000u32.wrapping_sub(sub))
}

/// The 56-byte `cFEStateOptions*` constructors: the `UIState` base constructor
/// runs on `this` with a1 = 0, then `this->vptr` (+8) is stored.
pub(crate) fn fe_options_ui_ctor(c: &mut Cpu, call_ret: u32, vptr_sub: u32) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = 0;
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, FE_UI_STATE_CTOR, call_ret)?;
    c.g[V1][0] = fe_vptr(vptr_sub);
    c.g[V0][0] = c.g[S0][0];
    frame.restore_ra(c, 0);
    Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
    frame.restore(c, S0, 0x10);
    frame.close(c);
    Ok(())
}

/// The 40-byte destructors: store `this->vptr` (+8), then call the base
/// destructor `target` on the unchanged `this` (a0), returning to `ret`.
pub(crate) fn fe_options_dtor(c: &mut Cpu, target: u32, ret: u32, vptr_sub: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = fe_vptr(vptr_sub);
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
    call_ptr(c, target, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEStateOptions::cFEStateOptions` (0x1ddee8, the `_ctor_2` alias): the
/// `UIState` base constructor, then the `cFEStateOptions` vtable.
pub fn cfe_state_options_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_df00, 0x3840))
}

/// `cFEStateOptions::~cFEStateOptions` (0x1ddf20): the `cFEState` vtable,
/// then the `UIState` destructor.
pub fn cfe_state_options_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_df3c, 0x2f30)
    })
}

/// `cFEStateOptionsGame::cFEStateOptionsGame` (0x1ddf48, `_ctor_2`).
pub fn cfe_state_options_game_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_df60, 0x3910))
}

/// `cFEStateOptionsGame::~cFEStateOptionsGame` (0x1ddf80).
pub fn cfe_state_options_game_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_df9c, 0x2f30)
    })
}

/// `cFEStateOptionsSound::cFEStateOptionsSound` (0x1ddfb0, `_ctor_2`).
pub fn cfe_state_options_sound_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_dfc8, 0x39e0))
}

/// `cFEStateOptionsSound::~cFEStateOptionsSound` (0x1ddfe8).
pub fn cfe_state_options_sound_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_e004, 0x2f30)
    })
}

/// `cFEStateOptionsHUD::cFEStateOptionsHUD` (0x1de018, `_ctor_2`).
pub fn cfe_state_options_hud_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_e030, 0x3ab0))
}

/// `cFEStateOptionsHUD::~cFEStateOptionsHUD` (0x1de050).
pub fn cfe_state_options_hud_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_e06c, 0x2f30)
    })
}

/// `cFEStateOptionsController::cFEStateOptionsController` (0x1de080, 60
/// bytes): as the 56-byte constructors, and it also clears the byte at +0x50
/// (`this->flag`) before the vptr store.
pub fn cfe_state_options_controller_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, FE_UI_STATE_CTOR, 0x001d_e098)?;
        c.g[V1][0] = fe_vptr(0x3b80);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x50)).sb(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOptionsController::~cFEStateOptionsController` (0x1de0c0).
pub fn cfe_state_options_controller_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_e0dc, 0x2f30)
    })
}

/// `cFEStateOptionsSaveLoad::cFEStateOptionsSaveLoad` (0x1de0f0, `_ctor_2`).
pub fn cfe_state_options_save_load_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_options_ui_ctor(c, 0x001d_e108, 0x3c50))
}

/// `cFEStateOptionsSaveLoad::~cFEStateOptionsSaveLoad` (0x1de128).
pub fn cfe_state_options_save_load_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_options_dtor(c, FE_UI_STATE_DTOR, 0x001d_e144, 0x2f30)
    })
}

/// `cFEStateOptionsDeviceSelect::~cFEStateOptionsDeviceSelect` (0x1de150):
/// the vptr, then the `MemCardStateBase` destructor.
pub fn cfe_state_options_device_select_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_options_dtor(c, FE_MEMCARD_STATE_DTOR, 0x001d_e16c, 0x3dd8)
    })
}

/// `cFEStateOptionsDeviceSelect::cFEStateOptionsDeviceSelect` (0x1de178,
/// `_ctor_2`, 60 bytes): the `MemCardStateBase` constructor runs with a1 = a2
/// = 0, then the vptr is stored.
pub fn cfe_state_options_device_select_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, FE_MEMCARD_STATE_CTOR, 0x001d_e194)?;
        c.g[V1][0] = fe_vptr(0x3dd8);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Gate word for this module's fakes, and its magic value.
#[cfg(test)]
pub(crate) const FE_GATE: u32 = DATA + 0xf_0000;

#[cfg(test)]
pub(crate) const FE_GATE_MAGIC: u32 = 0x0509_0ce5;

/// Seeds per test.
#[cfg(test)]
pub(crate) const FE_SEEDS: u32 = 200;

/// Object pointers land in `DATA..DATA + 0x80000` (away from the gate word).
#[cfg(test)]
pub(crate) const FE_OBJ_MASK: u32 = 0x7_fff0;

#[cfg(test)]
pub(crate) static FE_PREV_UI_CTOR: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) static FE_PREV_UI_CTOR_2: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) static FE_PREV_UI_DTOR: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) static FE_PREV_MEMCARD_CTOR: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) static FE_PREV_MEMCARD_DTOR: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) static FE_PREV_KEYBOARD_SET_KEY: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
pub(crate) static FE_PREV_STRLEN: AtomicUsize = AtomicUsize::new(0);

/// Logs the call (registers, cycles, target) and returns to `ra`; with the
/// gate off, runs the body the slot held before.
#[cfg(test)]
pub(crate) fn fe_gated(c: &mut Cpu, target: u32, prev: &AtomicUsize, generated: GuestFn) {
    if c.bus.read32(FE_GATE) != FE_GATE_MAGIC {
        return fakes::fall_back(c, prev, generated);
    }
    let ra = c.g[RA][0] as u32;
    fakes::fake(c, target, ra);
}

#[cfg(test)]
pub(crate) fn fe_fake_ui_ctor(c: &mut Cpu) {
    fe_gated(
        c,
        FE_UI_STATE_CTOR,
        &FE_PREV_UI_CTOR,
        ee::s13::f_0039e2a0__generated,
    )
}

#[cfg(test)]
pub(crate) fn fe_fake_ui_ctor_2(c: &mut Cpu) {
    fe_gated(
        c,
        FE_UI_STATE_CTOR_2,
        &FE_PREV_UI_CTOR_2,
        ee::s13::f_0039e318__generated,
    )
}

#[cfg(test)]
pub(crate) fn fe_fake_ui_dtor(c: &mut Cpu) {
    fe_gated(
        c,
        FE_UI_STATE_DTOR,
        &FE_PREV_UI_DTOR,
        ee::s13::f_0039e390__generated,
    )
}

#[cfg(test)]
pub(crate) fn fe_fake_memcard_ctor(c: &mut Cpu) {
    fe_gated(
        c,
        FE_MEMCARD_STATE_CTOR,
        &FE_PREV_MEMCARD_CTOR,
        ee::s04::f_001d53b0__generated,
    )
}

#[cfg(test)]
pub(crate) fn fe_fake_memcard_dtor(c: &mut Cpu) {
    fe_gated(
        c,
        FE_MEMCARD_STATE_DTOR,
        &FE_PREV_MEMCARD_DTOR,
        ee::s04::f_001d5428__generated,
    )
}

#[cfg(test)]
pub(crate) fn fe_fake_keyboard_set_key(c: &mut Cpu) {
    fe_gated(
        c,
        FE_KEYBOARD_SET_KEY_ENABLED,
        &FE_PREV_KEYBOARD_SET_KEY,
        ee::s04::f_001ce3c8__generated,
    )
}

#[cfg(test)]
pub(crate) fn fe_fake_strlen(c: &mut Cpu) {
    fe_gated(
        c,
        FE_STRLEN,
        &FE_PREV_STRLEN,
        ee::s15::f_00416810__generated,
    )
}

/// Hooks the fakes into the override slots of their callees (once per process).
#[cfg(test)]
pub(crate) fn fe_install_fakes() {
    fakes::hook(
        addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
        fe_fake_ui_ctor,
        &FE_PREV_UI_CTOR,
    );
    fakes::hook(
        addr_of_mut!(ee::s13::OVERRIDE_0039E318),
        fe_fake_ui_ctor_2,
        &FE_PREV_UI_CTOR_2,
    );
    fakes::hook(
        addr_of_mut!(ee::s13::OVERRIDE_0039E390),
        fe_fake_ui_dtor,
        &FE_PREV_UI_DTOR,
    );
    fakes::hook(
        addr_of_mut!(ee::s04::OVERRIDE_001D53B0),
        fe_fake_memcard_ctor,
        &FE_PREV_MEMCARD_CTOR,
    );
    fakes::hook(
        addr_of_mut!(ee::s04::OVERRIDE_001D5428),
        fe_fake_memcard_dtor,
        &FE_PREV_MEMCARD_DTOR,
    );
    fakes::hook(
        addr_of_mut!(ee::s04::OVERRIDE_001CE3C8),
        fe_fake_keyboard_set_key,
        &FE_PREV_KEYBOARD_SET_KEY,
    );
    fakes::hook(
        addr_of_mut!(ee::s15::OVERRIDE_00416810),
        fe_fake_strlen,
        &FE_PREV_STRLEN,
    );
}

/// The machine for one seed: scrambled registers, the gate word set, a
/// random object pointer in a0 and a game-like gp word. Each callee is
/// called once, and that call may unwind.
#[cfg(test)]
pub(crate) fn fe_setup(c: &mut Cpu, seed: u32, entry: u32, salt: u64) -> Rng {
    fe_install_fakes();
    let mut r = fakes::machine(c, seed, salt, 1);
    c.bus.write32(FE_GATE, FE_GATE_MAGIC);
    c.bus.write32(GAME_GP + 0xda0, r.next_u32());
    c.pc = entry;
    let obj = DATA + (r.next_u32() & FE_OBJ_MASK);
    c.g[A0][0] = sx(obj);
    r
}

/// `lw off(reg)` (register `reg` holds the base).
#[inline(always)]
pub(crate) fn oc832_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.lw(a)
}

/// `lh off(reg)`.
#[inline(always)]
pub(crate) fn oc832_lh(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.lh(a)
}

/// `sw value, off(reg)`.
#[inline(always)]
pub(crate) fn oc832_sw(c: &mut Cpu, reg: usize, off: u32, value: u64) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.sw(a, value);
}

/// `lw -off(gp)`.
#[inline(always)]
pub(crate) fn oc832_gp_lw(c: &mut Cpu, off: u32) -> u64 {
    let a = (c.g[GP][0] as u32).wrapping_sub(off);
    c.lw(a)
}

/// `sw value, -off(gp)`.
#[inline(always)]
pub(crate) fn oc832_gp_sw(c: &mut Cpu, off: u32, value: u64) {
    let a = (c.g[GP][0] as u32).wrapping_sub(off);
    c.sw(a, value);
}

/// The option widgets `cFEStateOptions_OnWidgetCreated` knows: (offset of the
/// name's string from 0x4a0000, value stored at `self+0x18`, return address of
/// the hash call).
pub(crate) const OC832_OPTION_WIDGETS: [(u32, u64, u32); 8] = [
    (0x14f8, 0x1c, 0x0018_8894),
    (0x1500, 0x1d, 0x0018_88b0),
    (0x1508, 0x1f, 0x0018_88cc),
    (0x1510, 0x1e, 0x0018_88e8),
    (0x1518, 0x20, 0x0018_8904),
    (0x1520, 0x24, 0x0018_8920),
    (0x1528, 0x24, 0x0018_893c),
    (0x1530, 0x22, 0x0018_8958),
];

/// `void cFEStateOptions_OnWidgetCreated(self, widget)` (0x188870): compare the
/// widget's hash name (`+0x38`) with the eight option-widget names in turn and
/// store the matching kind (0x1c, 0x1d, 0x1f, 0x1e, 0x20, 0x24, 0x24, 0x22) at
/// `self+0x18`.
pub fn cfe_state_options_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x004a_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(OC832_OPTION_WIDGETS[0].0));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = oc832_lw(c, S1, 0x38);
        call_ptr(c, 0x0031_7670, OC832_OPTION_WIDGETS[0].2)?;
        let mut kind = None;
        let first_matches = c.g[S0][0] == c.g[V0][0];
        c.g[A0][0] = 0x004a_0000;
        if first_matches {
            c.cycles += 2;
            kind = Some(OC832_OPTION_WIDGETS[0].1);
        } else {
            for (i, &(off, value, ret)) in OC832_OPTION_WIDGETS.iter().enumerate().skip(1) {
                c.cycles += 5;
                c.g[S0][0] = oc832_lw(c, S1, 0x38);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(off));
                call_ptr(c, 0x0031_7670, ret)?;
                if i == OC832_OPTION_WIDGETS.len() - 1 {
                    let no_match = c.g[S0][0] != c.g[V0][0];
                    frame.restore(c, S0, 0x20);
                    if !no_match {
                        c.cycles += 1;
                        kind = Some(value);
                    }
                } else if c.g[S0][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x004a_0000;
                } else {
                    c.cycles += 2;
                    kind = Some(value);
                    break;
                }
            }
        }
        if let Some(value) = kind {
            c.g[V0][0] = value;
            c.cycles += 2;
            oc832_sw(c, S1, 0x18, c.g[V0][0]);
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared "unsaved options?" test of `cFEStateOptions_OnWidgetAction`
/// (cases 5 and 6): `true` if the popup must be opened instead (no pending
/// `gp-0x1c10` flag and one of the four change flags `gp-0x1c20`, `-0x1c1c`,
/// `-0x1c14`, `-0x1c18` set). On `false`, `v0` holds `self+0x1c`.
/// The case-5 copy (`case5`) spends one more cycle on its fall-through.
pub(crate) fn oc832_options_changed(c: &mut Cpu, case5: bool) -> bool {
    c.cycles += 3;
    c.g[V0][0] = oc832_gp_lw(c, 0x1c10);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = oc832_lw(c, S1, 0x1c);
        return false;
    }
    c.cycles += 3;
    c.g[V0][0] = oc832_gp_lw(c, 0x1c20);
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = oc832_gp_lw(c, 0x1c1c);
    if t {
        return true;
    }
    c.cycles += 2;
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = oc832_gp_lw(c, 0x1c14);
    if t {
        return true;
    }
    c.cycles += 2;
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = oc832_gp_lw(c, 0x1c18);
    if t {
        return true;
    }
    c.cycles += 2;
    if case5 {
        // The case-5 copy tests the last flag into a register first.
        let t = c.g[V0][0] != 0;
        if t {
            return true;
        }
        c.cycles += 1;
        c.g[V0][0] = oc832_lw(c, S1, 0x1c);
        return false;
    }
    if c.g[V0][0] == 0 {
        c.g[V0][0] = oc832_lw(c, S1, 0x1c);
        false
    } else {
        true
    }
}

/// The "transition out" of the options screen: clear `gp-0x1c10`, set bit 0x80
/// of `self+0x1c`, look up the "TransitionOut" frame label of the screen
/// (`self+0x40`). Returns whether the screen has such a frame (the caller plays it).
/// `rets` are the return addresses of the hash call and the label call.
pub(crate) fn oc832_options_find_transition(c: &mut Cpu, rets: [u32; 2]) -> Result<bool, Unwound> {
    c.cycles += 13;
    c.g[A0][0] = 0x0046_0000;
    oc832_gp_sw(c, 0x1c10, 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x23a0));
    c.g[V0][0] |= 0x80;
    oc832_sw(c, S1, 0x1c, c.g[V0][0]);
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A0][0] = oc832_lw(c, S1, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_c778, rets[1])?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = 0xffff;
    let none = c.g[A1][0] == c.g[V0][0];
    c.g[A1][0] &= 0xffff;
    Ok(!none)
}

/// `void cFEStateOptions_OnWidgetAction(self, widget, event)` (0x188980): case 5
/// (button): the keyboard widget opens a cKeyboardPopup; the back widget asks
/// to save or plays "TransitionOut" and otherwise asks the engine's factory
/// (vtable slot 4) for a screen to push. Case 6 (back): the same save prompt or
/// transition. Case 13 (keyboard closed): apply the cheat code typed and play
/// the success (6) or failure (4) sound.
///
/// Returns through the common tail: `exit` 0 runs all of it, 1 skips the first
/// `cycles += 1` + `lq s0`, 2 only the epilogue.
pub fn cfe_state_options_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x6;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        let is_back = c.g[A2][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let exit: u8;
        if is_back {
            exit = oc832_options_back(c, &frame)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
            let high = c.g[V0][0] == 0;
            c.g[V0][0] = 0x5;
            if !high {
                c.cycles += 2;
                let is_button = c.g[A2][0] == c.g[V0][0];
                frame.restore(c, S0, 0x30);
                if is_button {
                    exit = oc832_options_button(c, &frame)?;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x20);
                    exit = 2;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0xd;
                let is_closed = c.g[A2][0] == c.g[V0][0];
                frame.restore(c, S0, 0x30);
                if is_closed {
                    exit = oc832_options_keyboard_closed(c)?;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x20);
                    exit = 2;
                }
            }
        }
        if exit == 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        if exit <= 1 {
            c.cycles += 1;
            frame.restore(c, S1, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Opens the "save options?" popup (`0x188e58`); returns the tail level 1.
pub(crate) fn oc832_options_open_prompt(c: &mut Cpu, frame: &Frame) -> Result<u8, Unwound> {
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0018_8e58, 0x0018_8b8c)?;
    frame.restore(c, S0, 0x30);
    Ok(1)
}

/// Event 6 of `cFEStateOptions_OnWidgetAction`: back out of the options screen.
pub(crate) fn oc832_options_back(c: &mut Cpu, frame: &Frame) -> Result<u8, Unwound> {
    if oc832_options_changed(c, false) {
        return oc832_options_open_prompt(c, frame);
    }
    if !oc832_options_find_transition(c, [0x0018_8bac, 0x0018_8bb8])? {
        return Ok(0);
    }
    c.cycles += 5;
    c.g[A0][0] = oc832_lw(c, S1, 0x40);
    c.g[A2][0] = 1;
    call_ptr(c, 0x0039_c870, 0x0018_8bd4)?;
    frame.restore(c, S0, 0x30);
    Ok(1)
}

/// Event 5 of `cFEStateOptions_OnWidgetAction`: a button was pressed.
pub(crate) fn oc832_options_button(c: &mut Cpu, frame: &Frame) -> Result<u8, Unwound> {
    c.cycles += 6;
    c.g[A0][0] = 0x004a_0000;
    c.g[S0][0] = oc832_lw(c, S2, 0x38);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1520));
    call_ptr(c, 0x0031_7670, 0x0018_89e4)?;
    if c.g[S0][0] == c.g[V0][0] {
        // The keyboard widget: open a keyboard popup for the name.
        c.cycles += 32;
        c.g[A1][0] = 0x0046_0000;
        c.g[A0][0] = 0x444;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2728));
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0018_8a04)?;
        c.g[T0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x15));
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = oc832_lw(c, S1, 0x10);
        c.g[A3][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001c_b030, 0x0018_8a1c)?;
        c.g[S0][0] = 0x004a_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1408));
        oc832_sw(c, S1, 0x50, c.g[V0][0]);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001c_b418, 0x0018_8a34)?;
        c.g[A0][0] = oc832_lw(c, S1, 0x50);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001c_cf00, 0x0018_8a40)?;
        c.g[A0][0] = oc832_lw(c, S1, 0x50);
        c.g[A1][0] = 0x4b;
        c.g[A2][0] = 1;
        c.g[A3][0] = 2;
        call_ptr(c, 0x001c_e3c8, 0x0018_8a54)?;
        c.g[A0][0] = oc832_lw(c, S1, 0x10);
        c.g[A1][0] = oc832_lw(c, S1, 0x50);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f290, 0x0018_8a64)?;
        frame.restore(c, S0, 0x30);
        return Ok(1);
    }
    c.g[A0][0] = 0x004a_0000;
    c.cycles += 5;
    c.g[S0][0] = oc832_lw(c, S2, 0x38);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1530));
    call_ptr(c, 0x0031_7670, 0x0018_8a78)?;
    if c.g[S0][0] != c.g[V0][0] {
        c.g[V0][0] = oc832_lw(c, S1, 0x10);
    } else {
        // The back widget.
        if oc832_options_changed(c, true) {
            return oc832_options_open_prompt(c, frame);
        }
        if oc832_options_find_transition(c, [0x0018_8acc, 0x0018_8ad8])? {
            c.cycles += 3;
            c.g[A0][0] = oc832_lw(c, S1, 0x40);
            c.g[A2][0] = 1;
            call_ptr(c, 0x0039_c870, 0x0018_8af4)?;
        }
        c.cycles += 3;
        c.g[V0][0] = oc832_lw(c, S1, 0x48);
        let no_save_screen = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        if no_save_screen {
            return Ok(1);
        }
        c.cycles += 4;
        c.g[V0][0] = oc832_lw(c, S1, 0x10);
        c.g[V1][0] = oc832_lw(c, V0, 0);
        let no_custom = c.g[V1][0] == 0;
        c.g[V0][0] = 0x18;
        if !no_custom {
            c.cycles += 1;
            oc832_sw(c, V1, 0xc, c.g[V0][0]);
        }
        c.cycles += 1;
        c.g[V0][0] = oc832_lw(c, S1, 0x10);
    }
    // Ask the engine's factory (vtable slot 4) for the next screen and push it.
    c.cycles += 10;
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = oc832_lw(c, S2, 0x18);
    c.g[A3][0] = oc832_lw(c, V0, 0);
    c.g[V1][0] = oc832_lw(c, A3, 0x4);
    c.g[A0][0] = oc832_lh(c, V1, 0x20);
    c.g[V0][0] = oc832_lw(c, V1, 0x24);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0018_8b38)?;
    let no_screen = c.g[V0][0] == 0;
    c.g[A1][0] = c.g[V0][0];
    if no_screen {
        return Ok(0);
    }
    c.cycles += 5;
    c.g[A0][0] = oc832_lw(c, S1, 0x10);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    call_ptr(c, 0x0039_f400, 0x0018_8b4c)?;
    frame.restore(c, S0, 0x30);
    Ok(1)
}

/// Event 13 of `cFEStateOptions_OnWidgetAction`: the keyboard popup closed;
/// apply the cheat code (`0x187d38` on the popup's text), play sound 6 if it
/// worked, else 4, on the engine's sound object (or the null sound at
/// `gp+0x2968`), and forget the popup.
pub(crate) fn oc832_options_keyboard_closed(c: &mut Cpu) -> Result<u8, Unwound> {
    c.cycles += 10;
    c.g[A0][0] = oc832_lw(c, S1, 0x50);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
    call_ptr(c, 0x0018_7d38, 0x0018_8be8)?;
    c.g[V1][0] = oc832_lw(c, S1, 0x10);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
    c.g[V0][0] = oc832_lw(c, V1, 0x14);
    if c.g[V0][0] != 0 {
        c.g[A2][0] = c.g[V0][0];
    }
    let no_sound = c.g[A2][0] == 0;
    c.g[V0][0] = 1;
    if !no_sound {
        c.cycles += 2;
        let failed = c.g[A0][0] != c.g[V0][0];
        c.g[V0][0] = oc832_lw(c, A2, 0);
        if !failed {
            c.cycles += 7;
            c.g[A1][0] = 6;
            c.g[A0][0] = oc832_lh(c, V0, 0x10);
            c.g[V1][0] = oc832_lw(c, V0, 0x14);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_8c20)?;
            oc832_sw(c, S1, 0x50, 0);
            return Ok(0);
        }
        c.cycles += 5;
        c.g[A1][0] = 4;
        c.g[A0][0] = oc832_lh(c, V0, 0x10);
        c.g[V1][0] = oc832_lw(c, V0, 0x14);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_8c3c)?;
    }
    c.cycles += 1;
    oc832_sw(c, S1, 0x50, 0);
    Ok(0)
}

/// Address of the options record the three setting rows copy (`getOptions`).
#[cfg(test)]
pub(crate) const OC833_OPTIONS: u32 = 0x0053_5610;

/// `lw 0x40(s1)`, then the call `UIScreen_FindObject(screen, v0)` (the hash
/// the previous call returned) returning to `ret`.
pub(crate) fn oc833_find(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret)
}

/// Row setup: the string-table entry of the row's slot, hashed, and its list widget found.
pub(crate) fn oc833_slot_lookup(c: &mut Cpu, ret_hash: u32, ret_find: u32) -> Flow {
    c.cycles += 13;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
    c.g[S2][0] = 0x440000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[S3][0] = 0x440000;
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    call_ptr(c, 0x0031_7670, ret_hash)?;
    oc833_find(c, ret_find)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 0x460000;
    Ok(())
}

/// One `addEntryByStringID(list, hash(str), index)`: the string pointer is
/// `0x460000 - sub` in `a0`.
pub(crate) fn oc833_add_entry(c: &mut Cpu, sub: u32, idx: u64, r_hash: u32, r_add: u32) -> Flow {
    c.g[A0][0] = 0x460000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(sub));
    call_ptr(c, 0x0031_7670, r_hash)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = idx;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0039_a568, r_add)
}

/// The inlined `getOptions()` copy (0x288 bytes from 0x535610 to `sp+0x10`)
/// in its aligned form (the source address is a constant with `& 7 == 0`, so
/// the unaligned word loop of the generated body is never taken). `regs`
/// are the registers loaded from +0/+8/+0x10/+0x18 of each 32-byte block,
/// `head` the loop head address.
pub(crate) fn oc833_copy_options(c: &mut Cpu, regs: [usize; 4], head: u32) {
    c.g[V1][0] = 0x530000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
    c.g[V0][0] = c.g[V1][0] & 0x7;
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.cycles += 1;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
    loop {
        c.cycles += 11;
        let v1 = c.g[V1][0] as u32;
        let a0 = c.g[A0][0] as u32;
        c.g[regs[0]][0] = c.ld(v1);
        c.g[regs[1]][0] = c.ld(v1.wrapping_add(0x8));
        c.g[regs[2]][0] = c.ld(v1.wrapping_add(0x10));
        c.g[regs[3]][0] = c.ld(v1.wrapping_add(0x18));
        c.sd(a0, c.g[regs[0]][0]);
        c.sd(a0.wrapping_add(0x8), c.g[regs[1]][0]);
        c.sd(a0.wrapping_add(0x10), c.g[regs[2]][0]);
        c.sd(a0.wrapping_add(0x18), c.g[regs[3]][0]);
        c.g[V1][0] = sx(v1.wrapping_add(0x20));
        let again = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = sx(a0.wrapping_add(0x20));
        if !again {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The last 8 bytes of the options copy (unaligned form).
pub(crate) fn oc833_copy_tail(c: &mut Cpu, tail: usize) {
    let v1 = c.g[V1][0] as u32;
    let a0 = c.g[A0][0] as u32;
    c.ldl(tail, v1.wrapping_add(0x7));
    c.ldr(tail, v1);
    c.sdl(tail, a0.wrapping_add(0x7));
    c.sdr(tail, a0);
}

/// Label row: `setUnicodeStringByID(find(table[slot]), hash(string))` for the
/// two tables (`s2`/`s3` hold 0x440000).
#[allow(clippy::too_many_arguments)]
pub(crate) fn oc833_label(
    c: &mut Cpu,
    off: u32,
    base: usize,
    imm: u32,
    r1: u32,
    r2: u32,
    imm2: u32,
    r3: u32,
    r4: u32,
) -> Flow {
    c.cycles += 12;
    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(off));
    c.g[V1][0] = sx((c.g[base][0] as u32).wrapping_add(imm));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
    call_ptr(c, 0x0031_7670, r1)?;
    oc833_find(c, r2)?;
    c.g[S0][0] = c.g[V0][0];
    let missing = c.g[S0][0] == 0u64;
    c.g[V0][0] = 0x440000;
    if !missing {
        c.cycles += 5;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(imm2));
        call_ptr(c, 0x0031_7670, r3)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, r4)?;
    }
    Ok(())
}

/// `cFEStateOptionsGame_Init(self)` (`onCreateScreen`): creates the "game
/// options" screen, hides two objects, selects menu entry 1, and fills the
/// three setting rows (language: skipped, its slot is -1; units: mph/kph
/// list; widescreen: three entries) with their list entries, current
/// selection from the options record at 0x535610, and the two label texts.
/// The row slots at `self+0x48..0x4a` are stored by the function itself and
/// re-read after the calls, so every row is ported (the language row is
/// reached only if a callee changed the slot).
pub fn oc833_fe_state_options_game_init(c: &mut Cpu) {
    if c.pc != 0x0018_8f28 {
        return oc833_fe_state_options_game_init_resume(c);
    }
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2f0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
        c.g[A0][0] = 0x460000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x2a0), c.g[RA][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2130));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_7670, 0x0018_8f54)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x0039_7cc0, 0x0018_8f68)?;
        let none = c.g[V0][0] == 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if !none {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0u64;
            call_ptr(c, 0x0039_c870, 0x0018_8f80)?;
        }
        c.cycles += 44;
        call_ptr(c, 0x0014_dc80, 0x0018_8f88)?;
        c.g[A1][0] = 0x4;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0018_8f94)?;
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[A1][0] = 0x2;
        c.g[V0][0] = 0x1;
        c.g[A0][0] = 0x4a0000;
        c.sb((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14f8));
        c.sb((c.g[S1][0] as u32).wrapping_add(0x4a), c.g[A1][0]);
        c.sb((c.g[S1][0] as u32).wrapping_add(0x49), c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x0018_8fb8)?;
        oc833_find(c, 0x0018_8fc4)?;
        c.g[A0][0] = 0x4a0000;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1500));
        call_ptr(c, 0x0031_7670, 0x0018_8fd4)?;
        oc833_find(c, 0x0018_8fe0)?;
        // uiObj->setVisible(0); uiObj->setStatic(1) (vtable slots 0x48 and 0x40)
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0018_8ff8)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0018_9010)?;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
        call_ptr(c, 0x0031_7670, 0x0018_901c)?;
        oc833_find(c, 0x0018_9028)?;
        let none = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[V0][0];
        if !none {
            c.cycles += 2;
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0039_b960, 0x0018_9038)?;
        }

        // Row 0: language list (slot -1 as stored above).
        c.cycles += 3;
        c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x48));
        let neg = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0x440000;
        if !neg {
            oc833_slot_lookup(c, 0x0018_9060, 0x0018_906c)?;
            if c.g[S0][0] != 0u64 {
                c.cycles += 32;
                oc833_add_entry(c, 0x2120, 0, 0x0018_9080, 0x0018_9090)?;
                oc833_add_entry(c, 0x2110, 1, 0x0018_909c, 0x0018_90ac)?;
                oc833_add_entry(c, 0x2100, 2, 0x0018_90b8, 0x0018_90c8)?;
                oc833_add_entry(c, 0x20f0, 3, 0x0018_90d4, 0x0018_90e4)?;
                oc833_copy_options(c, [A1, A2, A3, T0], 0x0018_9154);
                c.cycles += 12;
                oc833_copy_tail(c, V0);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 22);
                c.g[A1][0] &= 0x7;
                call_ptr(c, 0x0039_a768, 0x0018_91a8)?;
                let missing = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lbu(c.g[SP][0] as u32);
                if !missing {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039_a7a8, 0x0018_91b8)?;
                }
            }
            oc833_label(
                c,
                0x48,
                S2,
                0xfb8,
                0x0018_91d0,
                0x0018_91dc,
                0xfc8,
                0x0018_91f0,
                0x0018_91fc,
            )?;
            oc833_label(
                c,
                0x48,
                S3,
                0xff8,
                0x0018_9214,
                0x0018_9220,
                0xfe8,
                0x0018_9234,
                0x0018_9240,
            )?;
        }

        // Row 1: units (mph / kph).
        c.cycles += 3;
        c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
        let neg = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0x440000;
        if !neg {
            oc833_slot_lookup(c, 0x0018_9268, 0x0018_9274)?;
            if c.g[S0][0] != 0u64 {
                c.cycles += 18;
                oc833_add_entry(c, 0x20e0, 0, 0x0018_9288, 0x0018_9298)?;
                oc833_add_entry(c, 0x20d0, 0, 0x0018_92a4, 0x0018_92b4)?;
                oc833_copy_options(c, [T0, A1, A2, A3], 0x0018_9324);
                c.cycles += 9;
                oc833_copy_tail(c, T0);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 19);
                c.g[A1][0] &= 0x1;
                call_ptr(c, 0x0039_a7a8, 0x0018_9374)?;
            }
            oc833_label(
                c,
                0x49,
                S2,
                0xfb8,
                0x0018_938c,
                0x0018_9398,
                0xfcc,
                0x0018_93ac,
                0x0018_93b8,
            )?;
            oc833_label(
                c,
                0x49,
                S3,
                0xff8,
                0x0018_93d0,
                0x0018_93dc,
                0xfec,
                0x0018_93f0,
                0x0018_93fc,
            )?;
        }

        // Row 2: widescreen (off / 16:9 / animorphic).
        c.cycles += 3;
        c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x4a));
        let neg = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0x440000;
        if !neg {
            oc833_slot_lookup(c, 0x0018_9424, 0x0018_9430)?;
            if c.g[S0][0] != 0u64 {
                c.cycles += 25;
                oc833_add_entry(c, 0x20c0, 0, 0x0018_9444, 0x0018_9454)?;
                oc833_add_entry(c, 0x20b0, 0, 0x0018_9460, 0x0018_9470)?;
                oc833_add_entry(c, 0x2098, 0, 0x0018_947c, 0x0018_948c)?;
                oc833_copy_options(c, [A3, T0, A1, A2], 0x0018_94fc);
                c.cycles += 9;
                oc833_copy_tail(c, A3);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 20);
                c.g[A1][0] &= 0x3;
                call_ptr(c, 0x0039_a7a8, 0x0018_954c)?;
            }
            oc833_label(
                c,
                0x4a,
                S2,
                0xfb8,
                0x0018_9564,
                0x0018_9570,
                0xfd0,
                0x0018_9584,
                0x0018_9590,
            )?;
            oc833_label(
                c,
                0x4a,
                S3,
                0xff8,
                0x0018_95a8,
                0x0018_95b4,
                0xff0,
                0x0018_95c8,
                0x0018_95d4,
            )?;
        }

        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x2a0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2f0));
        c.pc = t;
        Ok(())
    });
}

/// Native resume of `oc833_fe_state_options_game_init` (converted from the `entries` body): any entry
/// other than 0x00188f28, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn oc833_fe_state_options_game_init_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00188f28 => {
                    c.cycles += 18;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2f0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
                    c.g[A0][0] = 0x460000;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x2a0), c.g[RA][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2130));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
                    c.g[RA][0] = sx(0x00188f54);
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317670, 0x00188f54)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00188f68);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00397cc0, 0x00188f68)?;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    if t {
                        pc = 0x00188f80;
                        continue;
                    }
                    pc = 0x00188f70;
                    continue;
                }
                0x00188f70 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00188f80);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039c870, 0x00188f80)?;
                    pc = 0x00188f80;
                    continue;
                }
                0x00188f80 => {
                    c.cycles += 44;
                    c.g[RA][0] = sx(0x00188f88);
                    call_ptr(c, 0x0014dc80, 0x00188f88)?;
                    c.g[A1][0] = 0x4;
                    c.g[RA][0] = sx(0x00188f94);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dd58, 0x00188f94)?;
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[A1][0] = 0x2;
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = 0x4a0000;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14f8));
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x4a), c.g[A1][0]);
                    c.g[RA][0] = sx(0x00188fb8);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x49), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x00188fb8)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00188fc4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00188fc4)?;
                    c.g[A0][0] = 0x4a0000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00188fd4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1500));
                    call_ptr(c, 0x00317670, 0x00188fd4)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00188fe0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00188fe0)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00188ff8);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00188ff8)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00189010);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00189010)?;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x0018901c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
                    call_ptr(c, 0x00317670, 0x0018901c)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00189028);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00189028)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[V0][0];
                    if t {
                        pc = 0x00189038;
                        continue;
                    }
                    pc = 0x00189030;
                    continue;
                }
                0x00189030 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00189038);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0039b960, 0x00189038)?;
                    pc = 0x00189038;
                    continue;
                }
                0x00189038 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x48));
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x00189240;
                        continue;
                    }
                    pc = 0x00189044;
                    continue;
                }
                0x00189044 => {
                    c.cycles += 13;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                    c.g[S2][0] = 0x440000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[S3][0] = 0x440000;
                    c.g[RA][0] = sx(0x00189060);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    call_ptr(c, 0x00317670, 0x00189060)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x0018906c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x0018906c)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001891b8;
                        continue;
                    }
                    pc = 0x00189078;
                    continue;
                }
                0x00189078 => {
                    c.cycles += 32;
                    c.g[RA][0] = sx(0x00189080);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2120));
                    call_ptr(c, 0x00317670, 0x00189080)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00189090);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x00189090)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x0018909c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2110));
                    call_ptr(c, 0x00317670, 0x0018909c)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0x1;
                    c.g[RA][0] = sx(0x001890ac);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x001890ac)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x001890b8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2100));
                    call_ptr(c, 0x00317670, 0x001890b8)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0x2;
                    c.g[RA][0] = sx(0x001890c8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x001890c8)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x001890d4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20f0));
                    call_ptr(c, 0x00317670, 0x001890d4)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001890e4);
                    c.g[A2][0] = 0x3;
                    call_ptr(c, 0x0039a568, 0x001890e4)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x00189150;
                        continue;
                    }
                    pc = 0x001890f8;
                    continue;
                }
                0x001890f8 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                    pc = 0x001890fc;
                    continue;
                }
                0x001890fc => {
                    c.cycles += 19;
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(5, (c.g[V1][0] as u32));
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(5, (c.g[A0][0] as u32));
                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001890fc) {
                            return Ok(());
                        }
                        pc = 0x001890fc;
                        continue;
                    }
                    pc = 0x00189148;
                    continue;
                }
                0x00189148 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x00189180;
                        continue;
                    }
                    pc = 0x00189150;
                    continue;
                }
                0x00189150 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                    pc = 0x00189154;
                    continue;
                }
                0x00189154 => {
                    c.cycles += 11;
                    c.g[A1][0] = c.ld((c.g[V1][0] as u32));
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd((c.g[A0][0] as u32), c.g[A1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00189154) {
                            return Ok(());
                        }
                        pc = 0x00189154;
                        continue;
                    }
                    pc = 0x00189180;
                    continue;
                }
                0x00189180 => {
                    c.cycles += 12;
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, (c.g[V1][0] as u32));
                    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(2, (c.g[A0][0] as u32));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 22);
                    c.g[RA][0] = sx(0x001891a8);
                    c.g[A1][0] = c.g[A1][0] & 0x7;
                    call_ptr(c, 0x0039a768, 0x001891a8)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.lbu((c.g[SP][0] as u32));
                    if t {
                        pc = 0x001891b8;
                        continue;
                    }
                    pc = 0x001891b0;
                    continue;
                }
                0x001891b0 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001891b8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a7a8, 0x001891b8)?;
                    pc = 0x001891b8;
                    continue;
                }
                0x001891b8 => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xfb8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[RA][0] = sx(0x001891d0);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x001891d0)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001891dc);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001891dc)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x001891fc;
                        continue;
                    }
                    pc = 0x001891e8;
                    continue;
                }
                0x001891e8 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x001891f0);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xfc8));
                    call_ptr(c, 0x00317670, 0x001891f0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001891fc);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001891fc)?;
                    pc = 0x001891fc;
                    continue;
                }
                0x001891fc => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xff8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[RA][0] = sx(0x00189214);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x00189214)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00189220);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00189220)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x00189240;
                        continue;
                    }
                    pc = 0x0018922c;
                    continue;
                }
                0x0018922c => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00189234);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xfe8));
                    call_ptr(c, 0x00317670, 0x00189234)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00189240);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00189240)?;
                    pc = 0x00189240;
                    continue;
                }
                0x00189240 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x001893fc;
                        continue;
                    }
                    pc = 0x0018924c;
                    continue;
                }
                0x0018924c => {
                    c.cycles += 13;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                    c.g[S2][0] = 0x440000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[S3][0] = 0x440000;
                    c.g[RA][0] = sx(0x00189268);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    call_ptr(c, 0x00317670, 0x00189268)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00189274);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00189274)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x00189374;
                        continue;
                    }
                    pc = 0x00189280;
                    continue;
                }
                0x00189280 => {
                    c.cycles += 18;
                    c.g[RA][0] = sx(0x00189288);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20e0));
                    call_ptr(c, 0x00317670, 0x00189288)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00189298);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x00189298)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x001892a4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20d0));
                    call_ptr(c, 0x00317670, 0x001892a4)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001892b4);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039a568, 0x001892b4)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x00189320;
                        continue;
                    }
                    pc = 0x001892c8;
                    continue;
                }
                0x001892c8 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                    pc = 0x001892cc;
                    continue;
                }
                0x001892cc => {
                    c.cycles += 19;
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(8, (c.g[V1][0] as u32));
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(8, (c.g[A0][0] as u32));
                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001892cc) {
                            return Ok(());
                        }
                        pc = 0x001892cc;
                        continue;
                    }
                    pc = 0x00189318;
                    continue;
                }
                0x00189318 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x00189350;
                        continue;
                    }
                    pc = 0x00189320;
                    continue;
                }
                0x00189320 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                    pc = 0x00189324;
                    continue;
                }
                0x00189324 => {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32));
                    c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd((c.g[A0][0] as u32), c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00189324) {
                            return Ok(());
                        }
                        pc = 0x00189324;
                        continue;
                    }
                    pc = 0x00189350;
                    continue;
                }
                0x00189350 => {
                    c.cycles += 9;
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(8, (c.g[V1][0] as u32));
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(8, (c.g[A0][0] as u32));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 19);
                    c.g[RA][0] = sx(0x00189374);
                    c.g[A1][0] = c.g[A1][0] & 0x1;
                    call_ptr(c, 0x0039a7a8, 0x00189374)?;
                    pc = 0x00189374;
                    continue;
                }
                0x00189374 => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xfb8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[RA][0] = sx(0x0018938c);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x0018938c)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00189398);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00189398)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x001893b8;
                        continue;
                    }
                    pc = 0x001893a4;
                    continue;
                }
                0x001893a4 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x001893ac);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xfcc));
                    call_ptr(c, 0x00317670, 0x001893ac)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001893b8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001893b8)?;
                    pc = 0x001893b8;
                    continue;
                }
                0x001893b8 => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xff8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[RA][0] = sx(0x001893d0);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x001893d0)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001893dc);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001893dc)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x001893fc;
                        continue;
                    }
                    pc = 0x001893e8;
                    continue;
                }
                0x001893e8 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x001893f0);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xfec));
                    call_ptr(c, 0x00317670, 0x001893f0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001893fc);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001893fc)?;
                    pc = 0x001893fc;
                    continue;
                }
                0x001893fc => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x4a));
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x001895d4;
                        continue;
                    }
                    pc = 0x00189408;
                    continue;
                }
                0x00189408 => {
                    c.cycles += 13;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                    c.g[S2][0] = 0x440000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[S3][0] = 0x440000;
                    c.g[RA][0] = sx(0x00189424);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    call_ptr(c, 0x00317670, 0x00189424)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00189430);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00189430)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x0018954c;
                        continue;
                    }
                    pc = 0x0018943c;
                    continue;
                }
                0x0018943c => {
                    c.cycles += 25;
                    c.g[RA][0] = sx(0x00189444);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20c0));
                    call_ptr(c, 0x00317670, 0x00189444)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00189454);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x00189454)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00189460);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20b0));
                    call_ptr(c, 0x00317670, 0x00189460)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00189470);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x00189470)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x0018947c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2098));
                    call_ptr(c, 0x00317670, 0x0018947c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0018948c);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039a568, 0x0018948c)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x001894f8;
                        continue;
                    }
                    pc = 0x001894a0;
                    continue;
                }
                0x001894a0 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                    pc = 0x001894a4;
                    continue;
                }
                0x001894a4 => {
                    c.cycles += 19;
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(7, (c.g[V1][0] as u32));
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(7, (c.g[A0][0] as u32));
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001894a4) {
                            return Ok(());
                        }
                        pc = 0x001894a4;
                        continue;
                    }
                    pc = 0x001894f0;
                    continue;
                }
                0x001894f0 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x00189528;
                        continue;
                    }
                    pc = 0x001894f8;
                    continue;
                }
                0x001894f8 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                    pc = 0x001894fc;
                    continue;
                }
                0x001894fc => {
                    c.cycles += 11;
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32));
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd((c.g[A0][0] as u32), c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A2][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001894fc) {
                            return Ok(());
                        }
                        pc = 0x001894fc;
                        continue;
                    }
                    pc = 0x00189528;
                    continue;
                }
                0x00189528 => {
                    c.cycles += 9;
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(7, (c.g[V1][0] as u32));
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(7, (c.g[A0][0] as u32));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 20);
                    c.g[RA][0] = sx(0x0018954c);
                    c.g[A1][0] = c.g[A1][0] & 0x3;
                    call_ptr(c, 0x0039a7a8, 0x0018954c)?;
                    pc = 0x0018954c;
                    continue;
                }
                0x0018954c => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x4a));
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xfb8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[RA][0] = sx(0x00189564);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x00189564)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00189570);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00189570)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x00189590;
                        continue;
                    }
                    pc = 0x0018957c;
                    continue;
                }
                0x0018957c => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00189584);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xfd0));
                    call_ptr(c, 0x00317670, 0x00189584)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00189590);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00189590)?;
                    pc = 0x00189590;
                    continue;
                }
                0x00189590 => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x4a));
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xff8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[RA][0] = sx(0x001895a8);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x001895a8)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001895b4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001895b4)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x001895d4;
                        continue;
                    }
                    pc = 0x001895c0;
                    continue;
                }
                0x001895c0 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x001895c8);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xff0));
                    call_ptr(c, 0x00317670, 0x001895c8)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001895d4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001895d4)?;
                    pc = 0x001895d4;
                    continue;
                }
                0x001895d4 => {
                    c.cycles += 7;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x2a0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2f0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00188f28, 0x001895f0);
                    return Ok(());
                }
            }
        }
    });
}

/// `int cFEStateOptions_OnWidgetNotify(self, menu, key, index)` (0x188c68): the
/// options screen's answer to a widget notification. Keys 8 and 9 answer 0x100;
/// key 6 (when the list item at `index` carries the hash of the screen's name
/// string `0x4a1530`) and key 7 answer 1, or 0x101 when the child flag at
/// `+0x4c` is set; every other case answers 0x101.
pub fn cfe_state_options_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        'b_188cec: {
            'b_188ca4: {
                'b_188ca0: {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A2][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A0][0];
                    c.g[V0][0] = (c.g[S1][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_188ca0;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[S1][0] < 0x8u64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x6;
                        if t {
                            break 'b_188ca4;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x100;
                            if t {
                                break 'b_188cec;
                            } else {
                                break 'b_188ca0;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x6;
                break 'b_188ca4;
            }
            'b_188ce0: {
                'b_188cd4: {
                    c.cycles += 2;
                    let t = c.g[S1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x7;
                    if t {
                        break 'b_188cd4;
                    } else {
                        c.cycles += 9;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
                        c.g[A1][0] = c.g[A3][0];
                        call_ptr(c, 0x00397870, 0x00188cb8)?;
                        c.g[A0][0] = 0x4a0000;
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1530));
                        call_ptr(c, 0x00317670, 0x00188cc8)?;
                        if c.g[S0][0] == c.g[V0][0] {
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                            break 'b_188ce0;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0x7;
                            break 'b_188cd4;
                        }
                    }
                }
                c.cycles += 2;
                if c.g[S1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0x101;
                    break 'b_188cec;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                    break 'b_188ce0;
                }
            }
            c.cycles += 3;
            c.g[V1][0] = 0x101;
            c.g[V0][0] = 0x1;
            if c.g[A0][0] != 0 {
                c.g[V0][0] = c.g[V1][0];
            }
            break 'b_188cec;
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    })
}

/// `cFEStateOptions_OnChildEvent(self, popup, event)` (0x188d08): the reset
/// popup's events. 0x15 sets the popup text; 0x16 (when the popup's name is the
/// reset popup) forwards the answer to the options widget (`onWidgetEvent`
/// 5 or 6) and sets the save-options flags at `+0x48`/`+0x4c`.
pub fn cfe_state_options_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_188e44: {
            'b_188e40: {
                'b_188d88: {
                    'b_188d40: {
                        c.cycles += 9;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                        c.g[V0][0] = 0x15;
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S1][0] = c.g[A0][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S2][0] = c.g[A1][0];
                        let t = c.g[A2][0] == c.g[V0][0];
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        if t {
                            break 'b_188d40;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x16;
                            let t = c.g[A2][0] == c.g[V0][0];
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_188d88;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_188e44;
                                } else {
                                    break 'b_188d40;
                                }
                            }
                        }
                    }
                    c.cycles += 10;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x22c0));
                    call_ptr(c, 0x00317670, 0x00188d50)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00188d5c)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        break 'b_188e40;
                    } else {
                        c.cycles += 8;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x22b0));
                        call_ptr(c, 0x00317670, 0x00188d74)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x00188d80)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_188e40;
                        } else {
                            break 'b_188d88;
                        }
                    }
                }
                c.cycles += 6;
                c.g[A0][0] = 0x460000;
                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x22e0));
                call_ptr(c, 0x00317670, 0x00188d98)?;
                let t = c.g[S0][0] != c.g[V0][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_188e40;
                } else {
                    'b_188e3c: {
                        'b_188e10: {
                            c.cycles += 15;
                            c.g[A0][0] = 0x4a0000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1530));
                            call_ptr(c, 0x00317670, 0x00188dac)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00188db8)?;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00188dd4)?;
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_188e10;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S0][0] == 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_188e3c;
                                } else {
                                    c.cycles += 11;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                                    c.g[A1][0] = c.g[S0][0];
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                                    c.g[A2][0] = 0x5;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00188e08)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_188e40;
                                    } else {
                                        break 'b_188e10;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S0][0] == 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_188e3c;
                        } else {
                            c.cycles += 9;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                            c.g[A1][0] = c.g[S0][0];
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = 0x6;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00188e3c)?;
                            break 'b_188e3c;
                        }
                    }
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    break 'b_188e40;
                }
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_188e44;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    })
}

/// `cFEStateOptionsGame_OnActivate(self)` (0x1895f0): the base screen's
/// activation (0x39e4c0).
pub fn cfe_state_options_game_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        call_ptr(c, 0x0039e4c0, 0x00189600)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    })
}

/// `cFEStateOptionsGame_OnChildEvent(self, popup, event)` (0x189610): the
/// popups of the game options screen. 0x16 applies the reset-defaults, screen
/// position (restore or store the offsets) and video calibration answers, sets
/// the options-changed flag at `gp-0x1c20`, refreshes the list boxes and calls
/// the option interface's slot 1.
pub fn cfe_state_options_game_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_18993c: {
            'b_189938: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x570));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x560));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x550));
                c.g[S0][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x530));
                c.g[S1][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x540));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x520), c.g[RA][0]);
                c.g[S3][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x00189638)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x4;
                call_ptr(c, 0x0014dd58, 0x00189644)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = 0x15;
                let t = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 0x16;
                if t {
                    break 'b_189938;
                } else {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x560));
                    if t {
                        break 'b_18993c;
                    } else {
                        'b_189928: {
                            'b_1896c8: {
                                c.cycles += 6;
                                c.g[A0][0] = 0x460000;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x22e0));
                                call_ptr(c, 0x00317670, 0x0018966c)?;
                                if c.g[S0][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1896c8;
                                } else {
                                    c.cycles += 8;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0x2;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018968c)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                        break 'b_189928;
                                    } else {
                                        c.cycles += 13;
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x0014f4f8, 0x0018969c)?;
                                        c.g[V0][0] = 0x1;
                                        c.g[A0][0] = c.g[S3][0];
                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c20), c.g[V0][0]);
                                        Callee {
                                            addr: 0x00189d60,
                                            f: ee::s02::f_00189d60,
                                        }
                                        .call(c, 0x001896ac)?;
                                        call_ptr(c, 0x0028b180, 0x001896b4)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x0029ed90, 0x001896c0)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_189928;
                                        } else {
                                            break 'b_1896c8;
                                        }
                                    }
                                }
                            }
                            'b_189924: {
                                'b_1898e0: {
                                    c.cycles += 5;
                                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2228));
                                    call_ptr(c, 0x00317670, 0x001896d4)?;
                                    if c.g[S0][0] != c.g[V0][0] {
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                        break 'b_1898e0;
                                    } else {
                                        'b_189748: {
                                            c.cycles += 8;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0x2;
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                                            let t = c.g[V0][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001896f4)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0u64;
                                            if t {
                                                break 'b_189748;
                                            } else {
                                                c.cycles += 19;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                                                let t = c.g[V0][0] as u32;
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00189710)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                c.g[S0][0] = c.g[V0][0];
                                                c.g[A1][0] = 0x1;
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                                                let t = c.g[V0][0] as u32;
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x0018972c)?;
                                                c.g[V1][0] = 0x530000;
                                                c.g[A0][0] = 0x1;
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                                                c.sw(
                                                    (c.g[GP][0] as u32).wrapping_sub(0x1c20),
                                                    c.g[A0][0],
                                                );
                                                c.sb(
                                                    (c.g[V1][0] as u32).wrapping_add(0x5),
                                                    c.g[V0][0],
                                                );
                                                let t = true;
                                                c.sb(
                                                    (c.g[V1][0] as u32).wrapping_add(0x6),
                                                    c.g[S0][0],
                                                );
                                                if t {
                                                    break 'b_189924;
                                                } else {
                                                    break 'b_189748;
                                                }
                                            }
                                        }
                                        'b_189804: {
                                            'b_1897d0: {
                                                c.cycles += 11;
                                                c.g[A1][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                                c.g[V0][0] = 0x530000;
                                                c.g[A3][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x5610));
                                                c.g[T2][0] = c.g[V0][0];
                                                c.g[V1][0] =
                                                    c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                                                c.g[A2][0] = c.g[SP][0];
                                                c.g[A0][0] = c.g[A3][0] & 0x7;
                                                c.g[V0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x60));
                                                c.g[T1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x64));
                                                let t = c.g[A0][0] == 0u64;
                                                c.g[T0][0] = sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                if t {
                                                    break 'b_1897d0;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[V1][0] =
                                                        sx((c.g[A3][0] as u32).wrapping_add(0x280));
                                                    c.g[V0][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x290));
                                                    'l_18977c: loop {
                                                        c.cycles += 19;
                                                        c.ldl(
                                                            4,
                                                            (c.g[A3][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.ldr(4, c.g[A3][0] as u32);
                                                        c.ldl(
                                                            5,
                                                            (c.g[A3][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.ldr(
                                                            5,
                                                            (c.g[A3][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.ldl(
                                                            11,
                                                            (c.g[A3][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.ldr(
                                                            11,
                                                            (c.g[A3][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.ldl(
                                                            12,
                                                            (c.g[A3][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.ldr(
                                                            12,
                                                            (c.g[A3][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.sdl(
                                                            4,
                                                            (c.g[A2][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.sdr(4, c.g[A2][0] as u32);
                                                        c.sdl(
                                                            5,
                                                            (c.g[A2][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.sdr(
                                                            5,
                                                            (c.g[A2][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sdl(
                                                            11,
                                                            (c.g[A2][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.sdr(
                                                            11,
                                                            (c.g[A2][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sdl(
                                                            12,
                                                            (c.g[A2][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.sdr(
                                                            12,
                                                            (c.g[A2][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.g[A3][0] =
                                                            sx((c.g[A3][0] as u32)
                                                                .wrapping_add(0x20));
                                                        let t = c.g[A3][0] != c.g[V1][0];
                                                        c.g[A2][0] =
                                                            sx((c.g[A2][0] as u32)
                                                                .wrapping_add(0x20));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x0018977c);
                                                            }
                                                            continue 'l_18977c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;

                                                            if t {
                                                                break 'b_189804;
                                                            } else {
                                                                break 'b_1897d0;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[V1][0] =
                                                sx((c.g[A3][0] as u32).wrapping_add(0x280));
                                            c.g[V0][0] =
                                                sx((c.g[SP][0] as u32).wrapping_add(0x290));
                                            'l_1897d8: loop {
                                                c.cycles += 11;
                                                c.g[A0][0] = c.ld(c.g[A3][0] as u32);
                                                c.g[A1][0] =
                                                    c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[T3][0] =
                                                    c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                                                c.g[T4][0] =
                                                    c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                                                c.sd(c.g[A2][0] as u32, c.g[A0][0]);
                                                c.sd(
                                                    (c.g[A2][0] as u32).wrapping_add(0x8),
                                                    c.g[A1][0],
                                                );
                                                c.sd(
                                                    (c.g[A2][0] as u32).wrapping_add(0x10),
                                                    c.g[T3][0],
                                                );
                                                c.sd(
                                                    (c.g[A2][0] as u32).wrapping_add(0x18),
                                                    c.g[T4][0],
                                                );
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x20));
                                                let t = c.g[A3][0] != c.g[V1][0];
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x001897d8);
                                                    }
                                                    continue 'l_1897d8;
                                                } else {
                                                    break 'b_189804;
                                                }
                                            }
                                        }
                                        'b_1898b8: {
                                            'b_189888: {
                                                c.cycles += 11;
                                                c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                                                c.ldr(3, c.g[A3][0] as u32);
                                                c.sdl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                                                c.sdr(3, c.g[A2][0] as u32);
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[A2][0] =
                                                    sx((c.g[T2][0] as u32).wrapping_add(0x5610));
                                                c.g[V0][0] =
                                                    c.lbu((c.g[SP][0] as u32).wrapping_add(0x6));
                                                c.g[V1][0] = c.g[A2][0] & 0x7;
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                                                let t = c.g[V1][0] == 0u64;
                                                c.g[A1][0] =
                                                    sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                                if t {
                                                    break 'b_189888;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V1][0] =
                                                        sx((c.g[A2][0] as u32).wrapping_add(0x280));
                                                    'l_189834: loop {
                                                        c.cycles += 19;
                                                        c.ldl(
                                                            12,
                                                            (c.g[A2][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.ldr(12, c.g[A2][0] as u32);
                                                        c.ldl(
                                                            2,
                                                            (c.g[A2][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.ldr(
                                                            2,
                                                            (c.g[A2][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.ldl(
                                                            7,
                                                            (c.g[A2][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.ldr(
                                                            7,
                                                            (c.g[A2][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.ldl(
                                                            10,
                                                            (c.g[A2][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.ldr(
                                                            10,
                                                            (c.g[A2][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.sdl(
                                                            12,
                                                            (c.g[A0][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.sdr(12, c.g[A0][0] as u32);
                                                        c.sdl(
                                                            2,
                                                            (c.g[A0][0] as u32).wrapping_add(0xf),
                                                        );
                                                        c.sdr(
                                                            2,
                                                            (c.g[A0][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sdl(
                                                            7,
                                                            (c.g[A0][0] as u32).wrapping_add(0x17),
                                                        );
                                                        c.sdr(
                                                            7,
                                                            (c.g[A0][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sdl(
                                                            10,
                                                            (c.g[A0][0] as u32).wrapping_add(0x1f),
                                                        );
                                                        c.sdr(
                                                            10,
                                                            (c.g[A0][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.g[A2][0] =
                                                            sx((c.g[A2][0] as u32)
                                                                .wrapping_add(0x20));
                                                        let t = c.g[A2][0] != c.g[V1][0];
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x20));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x00189834);
                                                            }
                                                            continue 'l_189834;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;

                                                            if t {
                                                                break 'b_1898b8;
                                                            } else {
                                                                break 'b_189888;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V1][0] =
                                                sx((c.g[A2][0] as u32).wrapping_add(0x280));
                                            'l_18988c: loop {
                                                c.cycles += 11;
                                                c.g[T3][0] = c.ld(c.g[A2][0] as u32);
                                                c.g[T4][0] =
                                                    c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                                                c.g[V0][0] =
                                                    c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                                                c.g[A3][0] =
                                                    c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                                                c.sd(c.g[A0][0] as u32, c.g[T3][0]);
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x8),
                                                    c.g[T4][0],
                                                );
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x10),
                                                    c.g[V0][0],
                                                );
                                                c.sd(
                                                    (c.g[A0][0] as u32).wrapping_add(0x18),
                                                    c.g[A3][0],
                                                );
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x20));
                                                let t = c.g[A2][0] != c.g[V1][0];
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0018988c);
                                                    }
                                                    continue 'l_18988c;
                                                } else {
                                                    break 'b_1898b8;
                                                }
                                            }
                                        }
                                        c.cycles += 10;
                                        c.ldl(10, (c.g[A2][0] as u32).wrapping_add(0x7));
                                        c.ldr(10, c.g[A2][0] as u32);
                                        c.sdl(10, (c.g[A0][0] as u32).wrapping_add(0x7));
                                        c.sdr(10, c.g[A0][0] as u32);
                                        c.g[V0][0] = c.lw(c.g[T1][0] as u32);
                                        c.g[A0][0] = c.g[T0][0];
                                        let t = c.g[V0][0] as u32;
                                        c.g[A2][0] = c.lb((c.g[SP][0] as u32).wrapping_add(0x295));
                                        call_ptr(c, t, 0x001898d8)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_189928;
                                        } else {
                                            break 'b_1898e0;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x1b;
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                    break 'b_189928;
                                } else {
                                    c.cycles += 8;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0x2;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00189904)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                        break 'b_189928;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x0014f4f8, 0x00189914)?;
                                        c.g[V0][0] = 0x1;
                                        c.g[A0][0] = c.g[S3][0];
                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c20), c.g[V0][0]);
                                        Callee {
                                            addr: 0x00189d60,
                                            f: ee::s02::f_00189d60,
                                        }
                                        .call(c, 0x00189924)?;
                                        break 'b_189924;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                            break 'b_189928;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x00189938)?;
                        break 'b_189938;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x560));
            break 'b_18993c;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x550));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x540));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x530));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x520));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x570));
        c.pc = t;
        return Ok(());
    })
}

/// `cFEStateOptionsGame_OnWidgetAction(self, widget, event)` (0x189980):
/// widget events of the game options screen: 7 and 5 push the reset, screen
/// position and calibration popups; 6 activates the options, refreshes and
/// pushes the next state; 9 stores the language, metric units and widescreen
/// choices. A null widget returns at once.
pub fn cfe_state_options_game_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_189d4c: {
            'b_189d48: {
                'b_189d44: {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S3][0] = c.g[A1][0];
                    let t = c.g[S3][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_189d44;
                    } else {
                        'b_189bd8: {
                            'b_189b34: {
                                c.cycles += 9;
                                call_ptr(c, 0x0014dc80, 0x001899b0)?;
                                c.g[A1][0] = 0x4;
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x0014dd58, 0x001899bc)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = 0x6;
                                let t = c.g[S0][0] == c.g[V0][0];
                                c.g[V0][0] = (c.g[S0][0] < 0x7u64) as u64;
                                if t {
                                    break 'b_189b34;
                                } else {
                                    'b_189a54: {
                                        'b_1899e4: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x5;
                                            if t {
                                                break 'b_1899e4;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_189a54;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_189d4c;
                                                    } else {
                                                        break 'b_1899e4;
                                                    }
                                                }
                                            }
                                        }
                                        'b_189a00: {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x7;
                                            let t = c.g[S0][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x9;
                                            if t {
                                                break 'b_189a00;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_189bd8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_189d4c;
                                                    } else {
                                                        break 'b_189a00;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 21;
                                        c.g[A1][0] = 0x460000;
                                        c.g[A0][0] = 0x4c;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2298));
                                        c.g[A2][0] = 0x100;
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x00189a18)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A2][0] = c.g[S2][0];
                                        call_ptr(c, 0x0039e318, 0x00189a2c)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[V0][0] = 0x470000;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4c30));
                                        c.g[A1][0] = c.g[S0][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        c.sb((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                                        call_ptr(c, 0x0039f290, 0x00189a4c)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        if t {
                                            break 'b_189d48;
                                        } else {
                                            break 'b_189a54;
                                        }
                                    }
                                    'b_189ac8: {
                                        c.cycles += 6;
                                        c.g[A0][0] = 0x4a0000;
                                        c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1520));
                                        call_ptr(c, 0x00317670, 0x00189a64)?;
                                        let t = c.g[S0][0] != c.g[V0][0];
                                        c.g[A0][0] = 0x4a0000;
                                        if t {
                                            break 'b_189ac8;
                                        } else {
                                            c.cycles += 23;
                                            c.g[A1][0] = 0x460000;
                                            c.g[A0][0] = 0x54;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_sub(0x2078));
                                            c.g[A2][0] = 0x100;
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x00189a84)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            call_ptr(c, 0x0039e318, 0x00189a98)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                            c.g[V0][0] = 0x470000;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0x4dd0));
                                            c.g[A1][0] = c.g[S0][0];
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                                            c.sb((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
                                            call_ptr(c, 0x0039f290, 0x00189ac0)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            if t {
                                                break 'b_189d48;
                                            } else {
                                                break 'b_189ac8;
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1528));
                                    call_ptr(c, 0x00317670, 0x00189ad4)?;
                                    let t = c.g[S0][0] != c.g[V0][0];
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                    if t {
                                        break 'b_189d48;
                                    } else {
                                        c.cycles += 22;
                                        c.g[A1][0] = 0x460000;
                                        c.g[A0][0] = 0x4c;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2060));
                                        c.g[A2][0] = 0x100;
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x00189af4)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A2][0] = c.g[S2][0];
                                        call_ptr(c, 0x0039e318, 0x00189b08)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[V0][0] = 0x470000;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4d00));
                                        c.g[V1][0] = 0x1b;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                                        c.g[A1][0] = c.g[S0][0];
                                        call_ptr(c, 0x0039f290, 0x00189b2c)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        if t {
                                            break 'b_189d48;
                                        } else {
                                            break 'b_189b34;
                                        }
                                    }
                                }
                            }
                            c.cycles += 36;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00189b48)?;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            call_ptr(c, 0x00228c08, 0x00189b50)?;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[V0][0] = 0x530000;
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5610));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 22);
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[A1][0] & 0x7;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00189b7c)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x48));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            let t = c.g[A1][0] as u32;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            call_ptr(c, t, 0x00189b98)?;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00189bbc)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_189d44;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f400, 0x00189bd0)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_189d48;
                                } else {
                                    break 'b_189bd8;
                                }
                            }
                        }
                        'b_189c94: {
                            'b_189c90: {
                                c.cycles += 3;
                                c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x48));
                                let t = (c.g[V1][0] as i64) < 0;
                                c.g[V0][0] = 0x440000;
                                if t {
                                    break 'b_189c90;
                                } else {
                                    c.cycles += 8;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    call_ptr(c, 0x00317670, 0x00189bfc)?;
                                    if c.g[S0][0] != c.g[V0][0] {
                                        c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x49));
                                        break 'b_189c94;
                                    } else {
                                        c.cycles += 35;
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x0039a738, 0x00189c0c)?;
                                        c.g[A3][0] = 0x530000;
                                        c.g[V1][0] = 0xfffffffffe3f0000;
                                        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x5610));
                                        c.g[V1][0] = c.g[V1][0] | 0xffff;
                                        c.g[V0][0] = c.g[V0][0] & 0x7;
                                        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 22);
                                        c.g[A1][0] = c.g[A1][0] & c.g[V1][0];
                                        c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                                        c.g[A0][0] = c.g[S3][0];
                                        c.sw((c.g[A3][0] as u32).wrapping_add(0x5610), c.g[A1][0]);
                                        c.g[S0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8c));
                                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x18));
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        call_ptr(c, 0x0039a738, 0x00189c4c)?;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, t, 0x00189c60)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x48));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                                        c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
                                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                        let t = c.g[A1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                        call_ptr(c, t, 0x00189c7c)?;
                                        call_ptr(c, 0x0028b180, 0x00189c84)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x0029ed90, 0x00189c90)?;
                                        break 'b_189c90;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x49));
                            break 'b_189c94;
                        }
                        'b_189ce8: {
                            'b_189ce4: {
                                c.cycles += 2;
                                let t = (c.g[V1][0] as i64) < 0;
                                c.g[V0][0] = 0x440000;
                                if t {
                                    break 'b_189ce4;
                                } else {
                                    c.cycles += 8;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    call_ptr(c, 0x00317670, 0x00189cb4)?;
                                    if c.g[S0][0] != c.g[V0][0] {
                                        c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x4a));
                                        break 'b_189ce8;
                                    } else {
                                        c.cycles += 10;
                                        c.g[A0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(0x319));
                                        c.g[A1][0] = 0x530000;
                                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x5610));
                                        c.g[V0][0] = 0xfffffffffff70000;
                                        c.g[A0][0] = (0u64 < c.g[A0][0]) as u64;
                                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                                        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 19);
                                        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                                        c.sw((c.g[A1][0] as u32).wrapping_add(0x5610), c.g[V1][0]);
                                        break 'b_189ce4;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x4a));
                            break 'b_189ce8;
                        }
                        'b_189d34: {
                            c.cycles += 2;
                            let t = (c.g[V1][0] as i64) < 0;
                            c.g[V0][0] = 0x440000;
                            if t {
                                break 'b_189d34;
                            } else {
                                c.cycles += 8;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                                c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00189d08)?;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.g[A1][0] = 0x530000;
                                if t {
                                    break 'b_189d34;
                                } else {
                                    c.cycles += 9;
                                    c.g[A0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(0x319));
                                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x5610));
                                    c.g[V0][0] = 0xffffffffffcf0000;
                                    c.g[A0][0] = c.g[A0][0] & 0x3;
                                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                                    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 20);
                                    c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x5610), c.g[V1][0]);
                                    break 'b_189d34;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        call_ptr(c, 0x00228c08, 0x00189d3c)?;
                        c.g[V0][0] = 0x1;
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c20), c.g[V0][0]);
                        break 'b_189d44;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_189d48;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_189d4c;
        }
        c.cycles += 5;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    })
}

/// `cFEStateOptionsGame_RefreshListBoxes(self)` (0x189d60): set the language,
/// units and widescreen list boxes from the option interface, tell the locale
/// and the graphics manager, and activate the options.
pub fn cfe_state_options_game_refresh_list_boxes(c: &mut Cpu) {
    guest(c, |c| {
        'b_189edc: {
            'b_189ed8: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x580));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x550));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x540));
                c.g[S2][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x530));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x570));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x560));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x520), c.g[RA][0]);
                c.g[S4][0] = 0x530000;
                call_ptr(c, 0x0014dc80, 0x00189d88)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x4;
                call_ptr(c, 0x0014dd58, 0x00189d94)?;
                c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x48));
                let t = (c.g[V1][0] as i64) < 0;
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x290));
                if t {
                    break 'b_189ed8;
                } else {
                    c.cycles += 12;
                    c.g[V0][0] = 0x440000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x00317670, 0x00189db8)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00189dc4)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0u64 {
                        c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x49));
                        break 'b_189edc;
                    } else {
                        'b_189e68: {
                            'b_189e38: {
                                c.cycles += 4;
                                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5610));
                                c.g[V0][0] = c.g[A0][0] & 0x7;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V1][0] = c.g[SP][0];
                                if t {
                                    break 'b_189e38;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x280));
                                    'l_189de4: loop {
                                        c.cycles += 19;
                                        c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                                        c.ldr(5, c.g[A0][0] as u32);
                                        c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                                        c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                                        c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                                        c.ldr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                                        c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                        c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                                        c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                                        c.sdr(5, c.g[V1][0] as u32);
                                        c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                                        c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                                        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                                        c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                        c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                        let t = c.g[A0][0] != c.g[V0][0];
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00189de4);
                                            }
                                            continue 'l_189de4;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_189e68;
                                            } else {
                                                break 'b_189e38;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x280));
                            'l_189e3c: loop {
                                c.cycles += 11;
                                c.g[T1][0] = c.ld(c.g[A0][0] as u32);
                                c.g[T2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.g[T3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                                c.sd(c.g[V1][0] as u32, c.g[T1][0]);
                                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T3][0]);
                                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                let t = c.g[A0][0] != c.g[V0][0];
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00189e3c);
                                    }
                                    continue 'l_189e3c;
                                } else {
                                    break 'b_189e68;
                                }
                            }
                        }
                        c.cycles += 28;
                        c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                        c.ldr(6, c.g[A0][0] as u32);
                        c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.sdr(6, c.g[V1][0] as u32);
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                        c.g[A1][0] = sx((c.g[A1][0] as u32) >> 22);
                        c.g[A1][0] = c.g[A1][0] & 0x7;
                        call_ptr(c, 0x0039a7a8, 0x00189e8c)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x18));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x0039a738, 0x00189ea8)?;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        call_ptr(c, t, 0x00189ebc)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x48));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        let t = c.g[A1][0] as u32;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        call_ptr(c, t, 0x00189ed8)?;
                        break 'b_189ed8;
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x49));
            break 'b_189edc;
        }
        'b_189fc4: {
            c.cycles += 2;
            let t = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = 0x440000;
            if t {
                break 'b_189fc4;
            } else {
                c.cycles += 11;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                call_ptr(c, 0x00317670, 0x00189ef8)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00189f04)?;
                c.g[A0][0] = c.g[V0][0];
                let t = c.g[A0][0] == 0u64;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5610));
                if t {
                    break 'b_189fc4;
                } else {
                    'b_189fa4: {
                        'b_189f74: {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[A1][0] & 0x7;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = c.g[SP][0];
                            if t {
                                break 'b_189f74;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                'l_189f20: loop {
                                    c.cycles += 19;
                                    c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0x7));
                                    c.ldr(10, c.g[A1][0] as u32);
                                    c.ldl(11, (c.g[A1][0] as u32).wrapping_add(0xf));
                                    c.ldr(11, (c.g[A1][0] as u32).wrapping_add(0x8));
                                    c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x17));
                                    c.ldr(6, (c.g[A1][0] as u32).wrapping_add(0x10));
                                    c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x1f));
                                    c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x18));
                                    c.sdl(10, (c.g[V1][0] as u32).wrapping_add(0x7));
                                    c.sdr(10, c.g[V1][0] as u32);
                                    c.sdl(11, (c.g[V1][0] as u32).wrapping_add(0xf));
                                    c.sdr(11, (c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                                    c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                    let t = c.g[A1][0] != c.g[V0][0];
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00189f20);
                                        }
                                        continue 'l_189f20;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_189fa4;
                                        } else {
                                            break 'b_189f74;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        'l_189f78: loop {
                            c.cycles += 11;
                            c.g[T0][0] = c.ld(c.g[A1][0] as u32);
                            c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[T2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                            c.g[T3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sd(c.g[V1][0] as u32, c.g[T0][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00189f78);
                                }
                                continue 'l_189f78;
                            } else {
                                break 'b_189fa4;
                            }
                        }
                    }
                    c.cycles += 8;
                    c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[A1][0] as u32);
                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.sdr(2, c.g[V1][0] as u32);
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 19);
                    c.g[A1][0] = c.g[A1][0] & 0x1;
                    call_ptr(c, 0x0039a7a8, 0x00189fc4)?;
                    break 'b_189fc4;
                }
            }
        }
        'b_18a0b0: {
            c.cycles += 3;
            c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x4a));
            let t = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = 0x440000;
            if t {
                break 'b_18a0b0;
            } else {
                c.cycles += 11;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                call_ptr(c, 0x00317670, 0x00189fe4)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00189ff0)?;
                c.g[A0][0] = c.g[V0][0];
                let t = c.g[A0][0] == 0u64;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5610));
                if t {
                    break 'b_18a0b0;
                } else {
                    'b_18a090: {
                        'b_18a060: {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[A1][0] & 0x7;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = c.g[SP][0];
                            if t {
                                break 'b_18a060;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                'l_18a00c: loop {
                                    c.cycles += 19;
                                    c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                                    c.ldr(9, c.g[A1][0] as u32);
                                    c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0xf));
                                    c.ldr(10, (c.g[A1][0] as u32).wrapping_add(0x8));
                                    c.ldl(11, (c.g[A1][0] as u32).wrapping_add(0x17));
                                    c.ldr(11, (c.g[A1][0] as u32).wrapping_add(0x10));
                                    c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x1f));
                                    c.ldr(6, (c.g[A1][0] as u32).wrapping_add(0x18));
                                    c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
                                    c.sdr(9, c.g[V1][0] as u32);
                                    c.sdl(10, (c.g[V1][0] as u32).wrapping_add(0xf));
                                    c.sdr(10, (c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.sdl(11, (c.g[V1][0] as u32).wrapping_add(0x17));
                                    c.sdr(11, (c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                    c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                    let t = c.g[A1][0] != c.g[V0][0];
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0018a00c);
                                        }
                                        continue 'l_18a00c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_18a090;
                                        } else {
                                            break 'b_18a060;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        'l_18a064: loop {
                            c.cycles += 11;
                            c.g[A3][0] = c.ld(c.g[A1][0] as u32);
                            c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                            c.g[T2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sd(c.g[V1][0] as u32, c.g[A3][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018a064);
                                }
                                continue 'l_18a064;
                            } else {
                                break 'b_18a090;
                            }
                        }
                    }
                    c.cycles += 8;
                    c.ldl(11, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(11, c.g[A1][0] as u32);
                    c.sdl(11, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.sdr(11, c.g[V1][0] as u32);
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 20);
                    c.g[A1][0] = c.g[A1][0] & 0x3;
                    call_ptr(c, 0x0039a7a8, 0x0018a0b0)?;
                    break 'b_18a0b0;
                }
            }
        }
        'b_18a15c: {
            'b_18a12c: {
                c.cycles += 9;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5610));
                c.g[A2][0] = c.g[SP][0];
                c.g[A1][0] = c.g[A3][0] & 0x7;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10d8));
                c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x60));
                c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x64));
                let t = c.g[A1][0] == 0u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                if t {
                    break 'b_18a12c;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x280));
                    'l_18a0d8: loop {
                        c.cycles += 19;
                        c.ldl(9, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.ldr(9, c.g[A3][0] as u32);
                        c.ldl(10, (c.g[A3][0] as u32).wrapping_add(0xf));
                        c.ldr(10, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.ldl(11, (c.g[A3][0] as u32).wrapping_add(0x17));
                        c.ldr(11, (c.g[A3][0] as u32).wrapping_add(0x10));
                        c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x1f));
                        c.ldr(3, (c.g[A3][0] as u32).wrapping_add(0x18));
                        c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.sdr(9, c.g[A2][0] as u32);
                        c.sdl(10, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.sdr(10, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.sdl(11, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.sdr(11, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.sdl(3, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.sdr(3, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        let t = c.g[A3][0] != c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0018a0d8);
                            }
                            continue 'l_18a0d8;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_18a15c;
                            } else {
                                break 'b_18a12c;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x280));
            'l_18a130: loop {
                c.cycles += 11;
                c.g[A1][0] = c.ld(c.g[A3][0] as u32);
                c.g[T1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A2][0] as u32, c.g[A1][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                let t = c.g[A3][0] != c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0018a130);
                    }
                    continue 'l_18a130;
                } else {
                    break 'b_18a15c;
                }
            }
        }
        'b_18a214: {
            'b_18a1e4: {
                c.cycles += 12;
                c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[A3][0] as u32);
                c.sdl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[A2][0] as u32);
                c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x6));
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5610));
                c.g[V1][0] = c.g[A2][0] | c.g[A3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[V1][0] = c.g[V1][0] & 0x7;
                let t = c.g[V1][0] == 0u64;
                c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                if t {
                    break 'b_18a1e4;
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x280));
                    'l_18a190: loop {
                        c.cycles += 19;
                        c.ldl(10, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.ldr(10, c.g[A3][0] as u32);
                        c.ldl(11, (c.g[A3][0] as u32).wrapping_add(0xf));
                        c.ldr(11, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0x17));
                        c.ldr(2, (c.g[A3][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[A3][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[A3][0] as u32).wrapping_add(0x18));
                        c.sdl(10, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.sdr(10, c.g[A2][0] as u32);
                        c.sdl(11, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.sdr(11, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.sdl(2, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.sdr(2, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        let t = c.g[A3][0] != c.g[V1][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0018a190);
                            }
                            continue 'l_18a190;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_18a214;
                            } else {
                                break 'b_18a1e4;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x280));
            'l_18a1e8: loop {
                c.cycles += 11;
                c.g[T2][0] = c.ld(c.g[A3][0] as u32);
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A2][0] as u32, c.g[T2][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T3][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                let t = c.g[A3][0] != c.g[V1][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0018a1e8);
                    }
                    continue 'l_18a1e8;
                } else {
                    break 'b_18a214;
                }
            }
        }
        c.cycles += 17;
        c.ldl(10, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(10, c.g[A3][0] as u32);
        c.sdl(10, (c.g[A2][0] as u32).wrapping_add(0x7));
        c.sdr(10, c.g[A2][0] as u32);
        c.g[V0][0] = c.lw(c.g[T0][0] as u32);
        let t = c.g[V0][0] as u32;
        c.g[A2][0] = c.lb((c.g[SP][0] as u32).wrapping_add(0x295));
        call_ptr(c, t, 0x0018a230)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        call_ptr(c, 0x00228c08, 0x0018a238)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x570));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x560));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x550));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x540));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x530));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x520));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x580));
        c.pc = t;
        return Ok(());
    })
}

/// The back-end option block (`cBEOptionInterface` mOption).
#[cfg(test)]
pub(crate) const SOUND_OPTION_BLOCK: u32 = 0x0053_5610;

/// Word `gp - 0x1680` (`D_004A1A70`): the "7lb" list box only counts when it is 1.
pub(crate) const SOUND_FLAG_GP: u32 = 0x1680;

/// Word `gp - 0x1c1c` (`D_004A14D4`): "options changed" flag.
pub(crate) const SOUND_CHANGED_GP: u32 = 0x1c1c;

#[inline(always)]
pub(crate) fn sound_r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// The shared tail of `OnWidgetAction`: the three label levels of the
/// generated epilogue (`level` 0 = after `cycles += 1; lq s0`, 1 = after
/// `cycles += 1; lq s1`, 2 = the final restore of `s2`, `ra` and `sp`).
pub(crate) fn sound_widget_exit(c: &mut Cpu, frame: Frame, level: u8) {
    if level == 0 {
        c.cycles += 1;
        frame.restore(c, 16, 0x2c0);
    }
    if level <= 1 {
        c.cycles += 1;
        frame.restore(c, 17, 0x2b0);
    }
    c.cycles += 4;
    frame.restore(c, 18, 0x2a0);
    frame.restore_ra(c, 0x290);
    frame.close(c);
}

/// The compiled `Cfg()` copy of the option block (0x280 bytes in four-dword
/// steps from `src` to the stack copy at `v1`). `src` always holds
/// `0x535610` here (8-aligned), so only the dword loop of the compiler's
/// aligned/unaligned pair is reachable. `regs` are the scratch registers the
/// compiler picked for this copy, `head` the loop's address (event resume).
pub(crate) fn sound_cfg_copy(c: &mut Cpu, src: usize, regs: [usize; 4], head: u32) {
    c.cycles += 1;
    c.g[V0][0] = sx(sound_r(c, src).wrapping_add(0x280));
    loop {
        c.cycles += 11;
        for (i, &reg) in regs.iter().enumerate() {
            let a = sound_r(c, src).wrapping_add(8 * i as u32);
            c.g[reg][0] = c.ld(a);
        }
        for (i, &reg) in regs.iter().enumerate() {
            let a = sound_r(c, V1).wrapping_add(8 * i as u32);
            let v = c.g[reg][0];
            c.sd(a, v);
        }
        c.g[src][0] = sx(sound_r(c, src).wrapping_add(0x20));
        let more = c.g[src][0] != c.g[V0][0];
        c.g[V1][0] = sx(sound_r(c, V1).wrapping_add(0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, head);
            }
            continue;
        }
        break;
    }
}

/// The last eight bytes of the option block copy (`ldl/ldr` + `sdl/sdr` pair
/// through scratch register `reg`).
pub(crate) fn sound_cfg_tail(c: &mut Cpu, src: usize, reg: usize) {
    let s = sound_r(c, src);
    c.ldl(reg, s.wrapping_add(0x7));
    c.ldr(reg, s);
    let d = sound_r(c, V1);
    c.sdl(reg, d.wrapping_add(0x7));
    c.sdr(reg, d);
}

/// `Is(widget->mHashName, str)`: hash the string at `0x4a0000 + str_off`
/// (`0x317670`) and compare with the widget's name hash (`s0` after the call
/// is the widget hash). `cycles` is the block charge before the call.
pub(crate) fn sound_hash_is(
    c: &mut Cpu,
    cycles: u64,
    str_off: u32,
    ret_addr: u32,
) -> Result<bool, crate::call::Unwound> {
    c.cycles += cycles;
    c.g[A0][0] = 0x4a_0000;
    let widget = sound_r(c, S1);
    c.g[S0][0] = c.lw(widget.wrapping_add(0x38));
    c.g[A0][0] = sx(sound_r(c, A0).wrapping_add(str_off));
    call_ptr(c, 0x0031_7670, ret_addr)?;
    Ok(c.g[S0][0] == c.g[V0][0])
}

/// `v0 = D_004A1A70` (the `lw v0, -0x1680(gp)` the chain leaves behind).
pub(crate) fn sound_load_flag(c: &mut Cpu) {
    c.g[V0][0] = c.lw(sound_r(c, GP).wrapping_sub(SOUND_FLAG_GP));
}

/// Store a volume slider's low nibble (shifted into place) into the option
/// block, then `bxAudio()` and snapshot the block: the common start of the
/// three slider cases. Leaves `a0 = audio`, `v1 = sp`, `s0 = block`, `v0 = 0`.
pub(crate) fn sound_slider_store(
    c: &mut Cpu,
    cycles: u64,
    mask: u64,
    shift: u32,
    ret_addr: u32,
) -> Flow {
    c.cycles += cycles;
    c.g[A0][0] = 0x53_0000;
    let widget = sound_r(c, S1);
    c.g[V1][0] = c.lbu(widget.wrapping_add(0x74));
    let block = sound_r(c, A0).wrapping_add(0x5610);
    c.g[V0][0] = c.lw(block);
    c.g[A1][0] = mask;
    c.g[V1][0] &= 0xf;
    c.g[S0][0] = sx(block);
    if shift != 0 {
        c.g[V1][0] = sx(sound_r(c, V1) << shift);
    }
    c.g[V0][0] &= c.g[A1][0];
    c.g[V0][0] |= c.g[V1][0];
    c.sw(block, c.g[V0][0]);
    call_ptr(c, 0x0028_b180, ret_addr)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = c.g[S0][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];
    Ok(())
}

/// Store a list box's boolean into one bit of the option block (`sw` of
/// `(block & !bit) | (getCurrentEntryData(widget) > 0) << pos`) and
/// start the block snapshot: the two DJ-speech / arcade-SFX cases.
pub(crate) fn sound_flag_store(
    c: &mut Cpu,
    mask_top: u64,
    pos: u32,
    entry_ret: u32,
    ret_addr: u32,
) -> Flow {
    c.cycles += 17;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0039_a738, entry_ret)?;
    c.g[A1][0] = 0x53_0000;
    c.g[A0][0] = mask_top;
    let block = sound_r(c, A1).wrapping_add(0x5610);
    c.g[V1][0] = c.lw(block);
    c.g[A0][0] |= 0xffff;
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    c.g[S0][0] = sx(block);
    c.g[V1][0] &= c.g[A0][0];
    c.g[V0][0] = sx(sound_r(c, V0) << pos);
    c.g[V1][0] |= c.g[V0][0];
    c.sw(block, c.g[V1][0]);
    call_ptr(c, 0x0028_b180, ret_addr)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = c.g[S0][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];
    Ok(())
}

/// The `case 9` (value changed) widget-name chain: which control moved, store
/// it in the option block and push it to the audio engine. Ends with
/// `v0 = D_004A1A70` (the generated `b_18b30c` join).
pub(crate) fn sound_value_dispatch(c: &mut Cpu) -> Flow {
    // 1lb: speaker mode list box.
    let hit = sound_hash_is(c, 6, 0x1568, 0x0018_adc4)?;
    c.g[A0][0] = 0x4a_0000;
    if hit {
        c.cycles += 17;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0039_a738, 0x0018_add4)?;
        c.g[A2][0] = 0x53_0000;
        c.g[A1][0] = 0xffff_ffff_ffff_cfff;
        let block = sound_r(c, A2).wrapping_add(0x5610);
        c.g[V1][0] = c.lw(block);
        c.g[V0][0] &= 0x3;
        c.g[V0][0] = sx(sound_r(c, V0) << 12);
        c.g[A0][0] = c.g[S1][0];
        c.g[V1][0] &= c.g[A1][0];
        c.g[V1][0] |= c.g[V0][0];
        c.sw(block, c.g[V1][0]);
        call_ptr(c, 0x0039_a738, 0x0018_adfc)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        Callee {
            addr: 0x0018_bef8,
            f: ee::s02::f_0018bef8,
        }
        .call(c, 0x0018_ae08)?;
        sound_load_flag(c);
        return Ok(());
    }

    // 2sl: sfx volume slider.
    let hit = sound_hash_is(c, 5, 0x1590, 0x0018_ae1c)?;
    c.g[A0][0] = 0x4a_0000;
    if hit {
        sound_slider_store(c, 14, 0xffff_ffff_ffff_fff0, 0, 0x0018_ae4c)?;
        sound_cfg_copy(c, S0, [T0, T1, A1, A2], 0x0018_aeb8);
        c.cycles += 9;
        sound_cfg_tail(c, S0, A3);
        c.g[A1][0] = c.lw(sound_r(c, SP));
        c.g[A1][0] &= 0xf;
        call_ptr(c, 0x0028_7410, 0x0018_af00)?;
        sound_load_flag(c);
        return Ok(());
    }

    // 3sl: speech volume slider.
    let hit = sound_hash_is(c, 5, 0x1598, 0x0018_af14)?;
    c.g[A0][0] = 0x4a_0000;
    if hit {
        sound_slider_store(c, 15, 0xffff_ffff_ffff_f0ff, 8, 0x0018_af48)?;
        sound_cfg_copy(c, S0, [T1, A1, A2, A3], 0x0018_afb4);
        c.cycles += 10;
        sound_cfg_tail(c, S0, T0);
        c.g[A1][0] = c.lw(sound_r(c, SP));
        c.g[A1][0] = sx(sound_r(c, A1) >> 8);
        c.g[A1][0] &= 0xf;
        call_ptr(c, 0x0028_7488, 0x0018_b000)?;
        sound_load_flag(c);
        return Ok(());
    }

    // 4sl: music volume slider.
    let hit = sound_hash_is(c, 5, 0x15a0, 0x0018_b014)?;
    c.g[A0][0] = 0x4a_0000;
    if hit {
        sound_slider_store(c, 15, 0xffff_ffff_ffff_ff0f, 4, 0x0018_b048)?;
        sound_cfg_copy(c, S0, [A1, A2, A3, T0], 0x0018_b0b4);
        c.cycles += 9;
        sound_cfg_tail(c, S0, T1);
        c.g[A1][0] = c.lbu(sound_r(c, SP));
        c.g[A1][0] >>= 4;
        call_ptr(c, 0x0028_7520, 0x0018_b0fc)?;
        sound_load_flag(c);
        return Ok(());
    }

    // 5lb: DJ speech on/off.
    let hit = sound_hash_is(c, 5, 0x15a8, 0x0018_b110)?;
    c.g[A0][0] = 0x4a_0000;
    if hit {
        sound_flag_store(c, 0xffff_ffff_fffd_0000, 17, 0x0018_b120, 0x0018_b14c)?;
        sound_cfg_copy(c, S0, [A2, A3, T0, T1], 0x0018_b1b8);
        c.cycles += 10;
        sound_cfg_tail(c, S0, V0);
        c.g[A1][0] = c.lw(sound_r(c, SP));
        c.g[A1][0] = sx(((sound_r(c, A1)) as i32 >> 17) as u32);
        c.g[A1][0] &= 0x1;
        call_ptr(c, 0x0028_7558, 0x0018_b204)?;
        sound_load_flag(c);
        return Ok(());
    }

    // 6lb: arcade SFX on/off (the compare's delay slot loads the flag).
    let hit = sound_hash_is(c, 5, 0x15b0, 0x0018_b218)?;
    sound_load_flag(c);
    if hit {
        sound_flag_store(c, 0xffff_ffff_fffe_0000, 16, 0x0018_b228, 0x0018_b254)?;
        sound_cfg_copy(c, S0, [A3, T0, T1, A1], 0x0018_b2c0);
        c.cycles += 8;
        sound_cfg_tail(c, S0, A2);
        c.g[A1][0] = c.lh(sound_r(c, SP).wrapping_add(2));
        c.g[A1][0] &= 0x1;
        call_ptr(c, 0x0028_75d0, 0x0018_b308)?;
        sound_load_flag(c);
    }
    Ok(())
}

/// `case 9` of the widget action: a control's value changed. After the name
/// chain, "7lb" (only while `D_004A1A70 == 1`) stores its boolean into bit 18
/// of the option block, and the options-changed flag is set.
pub(crate) fn sound_value_changed(c: &mut Cpu, frame: Frame) -> Flow {
    sound_value_dispatch(c)?;
    c.cycles += 3;
    c.g[V0][0] ^= 0x1;
    let skip = c.g[V0][0] != 0;
    c.g[V0][0] = 0x1;
    if !skip {
        let hit = sound_hash_is(c, 6, 0x15b8, 0x0018_b328)?;
        c.g[V0][0] = 0x1;
        if hit {
            c.cycles += 12;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0039_a738, 0x0018_b338)?;
            c.g[A1][0] = 0x53_0000;
            c.g[A0][0] = 0xffff_ffff_fffb_0000;
            let block = sound_r(c, A1).wrapping_add(0x5610);
            c.g[V1][0] = c.lw(block);
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[A0][0] |= 0xffff;
            c.g[V0][0] = sx(sound_r(c, V0) << 18);
            c.g[V1][0] &= c.g[A0][0];
            c.g[V1][0] |= c.g[V0][0];
            c.sw(block, c.g[V1][0]);
            c.g[V0][0] = 0x1;
        }
    }
    c.cycles += 1;
    let changed = sound_r(c, GP).wrapping_sub(SOUND_CHANGED_GP);
    c.sw(changed, c.g[V0][0]);
    sound_widget_exit(c, frame, 0);
    Ok(())
}

/// `case 6` of the widget action (the "accept" button): `iface->submit()`, push
/// each option-block field (volumes, DJ speech, arcade SFX) to the audio
/// engine through five fresh `Cfg()` snapshots, then ask the engine's state
/// factory for the next state (vtable slot at `+0x2c`) and push it.
pub(crate) fn sound_accept(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 14;
    // iface->submit(): `a1` is the interface from 0x14dd58.
    c.g[V0][0] = c.lw(sound_r(c, A1).wrapping_add(0xc));
    c.g[A0][0] = c.lh(sound_r(c, V0).wrapping_add(0x8));
    c.g[V1][0] = c.lw(sound_r(c, V0).wrapping_add(0xc));
    let target = sound_r(c, V1);
    c.g[A0][0] = sx(sound_r(c, A1).wrapping_add(sound_r(c, A0)));
    call_ptr(c, target, 0x0018_a9a4)?;
    call_ptr(c, 0x0028_b180, 0x0018_a9ac)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = 0x53_0000;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = sx(sound_r(c, V0).wrapping_add(0x5610));
    c.g[V0][0] = c.g[A1][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];

    // Snapshot 1: sfx volume (low nibble of word 0).
    sound_cfg_copy(c, A1, [A2, A3, T0, T1], 0x0018_aa24);
    c.cycles += 14;
    sound_cfg_tail(c, A1, V0);
    c.g[A1][0] = c.lw(sound_r(c, SP));
    c.g[A1][0] &= 0xf;
    call_ptr(c, 0x0028_7410, 0x0018_aa6c)?;
    call_ptr(c, 0x0028_b180, 0x0018_aa74)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(sound_r(c, S0).wrapping_add(0x5610));
    c.g[V0][0] = c.g[A1][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];

    // Snapshot 2: speech volume (bits 8..12).
    sound_cfg_copy(c, A1, [T1, A2, A3, T0], 0x0018_aae4);
    c.cycles += 15;
    sound_cfg_tail(c, A1, T1);
    c.g[A1][0] = c.lw(sound_r(c, SP));
    c.g[A1][0] = sx(sound_r(c, A1) >> 8);
    c.g[A1][0] &= 0xf;
    call_ptr(c, 0x0028_7488, 0x0018_ab30)?;
    call_ptr(c, 0x0028_b180, 0x0018_ab38)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(sound_r(c, S0).wrapping_add(0x5610));
    c.g[V0][0] = c.g[A1][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];

    // Snapshot 3: music volume (bits 4..8 of byte 0).
    sound_cfg_copy(c, A1, [T0, T1, A2, A3], 0x0018_aba8);
    c.cycles += 14;
    sound_cfg_tail(c, A1, T0);
    c.g[A1][0] = c.lbu(sound_r(c, SP));
    c.g[A1][0] >>= 4;
    call_ptr(c, 0x0028_7520, 0x0018_abf0)?;
    call_ptr(c, 0x0028_b180, 0x0018_abf8)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(sound_r(c, S0).wrapping_add(0x5610));
    c.g[V0][0] = c.g[A1][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];

    // Snapshot 4: DJ speech (bit 17).
    sound_cfg_copy(c, A1, [A3, T0, T1, A2], 0x0018_ac68);
    c.cycles += 15;
    sound_cfg_tail(c, A1, A3);
    c.g[A1][0] = c.lw(sound_r(c, SP));
    c.g[A1][0] = sx(((sound_r(c, A1)) as i32 >> 17) as u32);
    c.g[A1][0] &= 0x1;
    call_ptr(c, 0x0028_7558, 0x0018_acb4)?;
    call_ptr(c, 0x0028_b180, 0x0018_acbc)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(sound_r(c, S0).wrapping_add(0x5610));
    c.g[V0][0] = c.g[A1][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];

    // Snapshot 5: arcade SFX (bit 16 = bit 0 of the high half-word).
    sound_cfg_copy(c, A1, [A2, A3, T0, T1], 0x0018_ad2c);
    c.cycles += 18;
    sound_cfg_tail(c, A1, V0);
    c.g[A1][0] = c.lh(sound_r(c, SP).wrapping_add(0x2));
    c.g[A1][0] &= 0x1;
    call_ptr(c, 0x0028_75d0, 0x0018_ad74)?;

    // next = engine->vtable[+0x2c](engine + delta, self, widget->mMiscData)
    c.g[V0][0] = c.lw(sound_r(c, S2).wrapping_add(0x10));
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.lw(sound_r(c, S1).wrapping_add(0x18));
    c.g[A3][0] = c.lw(sound_r(c, V0));
    c.g[V1][0] = c.lw(sound_r(c, A3).wrapping_add(0x4));
    c.g[A0][0] = c.lh(sound_r(c, V1).wrapping_add(0x28));
    c.g[V0][0] = c.lw(sound_r(c, V1).wrapping_add(0x2c));
    let factory = sound_r(c, V0);
    c.g[A0][0] = sx(sound_r(c, A3).wrapping_add(sound_r(c, A0)));
    call_ptr(c, factory, 0x0018_ad98)?;
    let no_state = c.g[V0][0] == 0;
    c.g[A1][0] = c.g[V0][0];
    if no_state {
        sound_widget_exit(c, frame, 0);
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.lw(sound_r(c, S2).wrapping_add(0x10));
    c.g[A0][0] = sx(sound_r(c, A0).wrapping_add(0x18));
    call_ptr(c, 0x0039_f400, 0x0018_adac)?;
    frame.restore(c, 16, 0x2c0);
    sound_widget_exit(c, frame, 1);
    Ok(())
}

/// `cFEStateOptionsSound::OnWidgetAction(self = a0, widget = a1, event = a2)`
/// (0x18a8d0): the sound options screen's widget handler. A null widget does
/// nothing; event 6 (accept) applies every option and pushes the next state;
/// event 7 pushes the "reset options" popup; event 9 (value changed) stores
/// the moved control (speaker mode, three volumes, DJ speech, arcade SFX,
/// one more list box) into the option block and updates the audio engine and
/// the changed flag; other events do nothing.
///
/// The compiler's unaligned alternative of each option-block snapshot copy is
/// unreachable (the block address is the constant `0x535610`), so only the
/// aligned loops are ported.
pub fn cfe_state_options_sound_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x2d0);
        frame.save(c, 16, 0x2c0);
        frame.save(c, 17, 0x2b0);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, 18, 0x2a0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x290);
        let no_widget = c.g[S1][0] == 0;
        c.g[S2][0] = c.g[A0][0];
        if no_widget {
            sound_widget_exit(c, frame, 0);
            return Ok(());
        }
        c.cycles += 9;
        call_ptr(c, 0x0014_dc80, 0x0018_a8fc)?;
        c.g[A1][0] = 0x4;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0018_a908)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = 0x6;
        let accept = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[S0][0] < 0x7u64) as u64;
        if accept {
            return sound_accept(c, frame);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            frame.restore(c, 16, 0x2c0);
            sound_widget_exit(c, frame, 1);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 0x7;
        let popup = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = 0x9;
        if !popup {
            c.cycles += 2;
            let changed = c.g[S0][0] == c.g[V0][0];
            frame.restore(c, 16, 0x2c0);
            if changed {
                return sound_value_changed(c, frame);
            }
            c.cycles += 2;
            frame.restore(c, 17, 0x2b0);
            sound_widget_exit(c, frame, 2);
            return Ok(());
        }
        // Event 7: new cFEPopupOptionReset(76 bytes), parent = self's engine.
        c.cycles += 21;
        c.g[A1][0] = 0x46_0000;
        c.g[A0][0] = 0x4c;
        c.g[A1][0] = sx(sound_r(c, A1).wrapping_sub(0x2298));
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0018_a954)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(sound_r(c, S2).wrapping_add(0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, 0x0039_e318, 0x0018_a968)?;
        c.g[A0][0] = c.lw(sound_r(c, S2).wrapping_add(0x10));
        c.g[V0][0] = 0x47_0000;
        c.g[V0][0] = sx(sound_r(c, V0).wrapping_sub(0x4c30));
        c.g[A1][0] = c.g[S0][0];
        c.sw(sound_r(c, S0).wrapping_add(0x8), c.g[V0][0]);
        c.g[A0][0] = sx(sound_r(c, A0).wrapping_add(0x18));
        c.sb(sound_r(c, S0).wrapping_add(0x48), 0);
        call_ptr(c, 0x0039_f290, 0x0018_a988)?;
        frame.restore(c, 16, 0x2c0);
        sound_widget_exit(c, frame, 1);
        Ok(())
    })
}

/// `cFEStateOptionsSound_RefreshControls(self)` (0x18b380,
/// `cFEStateOptionsSound::updateWidget`): fills the sound options screen from
/// the saved options. Seven widget lookups (`Find` = 0x39d860 on
/// `self->f40` with a string hash from 0x317670):
/// - speaker mode list (4 entries: stereo, surround, mono, DTS), selecting the
///   entry of the current sound mode (`0x289dc0(0x28b180())`);
/// - SFX, speech and music volume sliders (`w+0x78 = 12`, `w+0x74` = the
///   saved volume, applied to the audio system with 0x287410 / 0x287488 /
///   0x287520);
/// - DJ speech and arcade SFX off/on lists, selecting the saved flag and
///   applying it (0x287558 / 0x2875d0);
/// - only when `D_004A1A70 == 1`: a last off/on list (option bit 18).
///
/// Each saved value is read from the stack copy of the BE option struct at
/// `0x535610` (`Cfg_18B380()` by value: a 0x288-byte inline copy per use;
/// the source address is a constant, so only the aligned `ld`/`sd` copy
/// loop runs, the `ldl`/`ldr` loop is kept for fidelity). The generated block
/// structure is kept: every copy loop, its register use and event check, and
/// every delay slot are as in the generated body.
pub fn cfe_state_options_sound_refresh_controls(c: &mut Cpu) {
    guest(c, cfe_state_options_sound_refresh_controls_body);
}

#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    clippy::all
)]
pub(crate) fn cfe_state_options_sound_refresh_controls_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x0018b380);
    'b_18b488: {
        c.cycles += 23;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x330));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x310));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x320));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x300));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2f0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x2e0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x2d0));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x2b0), c.g[RA][0]);
        c.g[RA][0] = sx(0x0018b3ac);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0014dc80, 0x0018b3ac)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0018b3b8);
        c.g[A1][0] = 0x4;
        call_ptr(c, 0x0014dd58, 0x0018b3b8)?;
        c.g[A0][0] = 0x4a0000;
        c.g[RA][0] = sx(0x0018b3c4);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1568));
        call_ptr(c, 0x00317670, 0x0018b3c4)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x0018b3d0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x0018b3d0)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0u64 {
            c.g[A0][0] = 0x4a0000;
            break 'b_18b488;
        } else {
            'b_18b484: {
                c.cycles += 40;
                c.g[RA][0] = sx(0x0018b3e4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039a670, 0x0018b3e4)?;
                c.g[A0][0] = 0x460000;
                c.g[RA][0] = sx(0x0018b3f0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2030));
                call_ptr(c, 0x00317670, 0x0018b3f0)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0018b400);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039a568, 0x0018b400)?;
                c.g[A0][0] = 0x460000;
                c.g[RA][0] = sx(0x0018b40c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2018));
                call_ptr(c, 0x00317670, 0x0018b40c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = 0x1;
                c.g[RA][0] = sx(0x0018b41c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039a568, 0x0018b41c)?;
                c.g[A0][0] = 0x460000;
                c.g[RA][0] = sx(0x0018b428);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2000));
                call_ptr(c, 0x00317670, 0x0018b428)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = 0x2;
                c.g[RA][0] = sx(0x0018b438);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039a568, 0x0018b438)?;
                c.g[A0][0] = 0x460000;
                c.g[RA][0] = sx(0x0018b444);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fe8));
                call_ptr(c, 0x00317670, 0x0018b444)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = 0x3;
                c.g[RA][0] = sx(0x0018b454);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039a568, 0x0018b454)?;
                c.g[RA][0] = sx(0x0018b45c);
                call_ptr(c, 0x0028b180, 0x0018b45c)?;
                c.g[RA][0] = sx(0x0018b464);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00289dc0, 0x0018b464)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0018b474);
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, 0x0039a768, 0x0018b474)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lbu(c.g[SP][0] as u32);
                if t {
                    break 'b_18b484;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0018b484);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a7a8, 0x0018b484)?;
                    break 'b_18b484;
                }
            }
            c.cycles += 1;
            c.g[A0][0] = 0x4a0000;
            break 'b_18b488;
        }
    }
    'b_18b638: {
        c.cycles += 12;
        c.g[S0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1590));
        c.g[RA][0] = sx(0x0018b498);
        c.g[S2][0] = 0x4a0000;
        call_ptr(c, 0x00317670, 0x0018b498)?;
        c.g[S3][0] = 0x4a0000;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0018b4ac);
        c.g[S6][0] = 0x4a0000;
        call_ptr(c, 0x0039d860, 0x0018b4ac)?;
        c.g[A2][0] = c.g[V0][0];
        let t = c.g[A2][0] == 0u64;
        c.g[V1][0] = 0x530000;
        if t {
            break 'b_18b638;
        } else {
            'b_18b560: {
                'b_18b530: {
                    c.cycles += 8;
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                    c.g[S5][0] = c.g[V1][0];
                    c.g[V0][0] = c.g[A1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = c.g[A0][0];
                    if t {
                        break 'b_18b530;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        'l_18b4dc: loop {
                            c.cycles += 19;
                            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.ldr(3, c.g[A1][0] as u32);
                            c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.sdr(3, c.g[A0][0] as u32);
                            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0xf));
                            c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x17));
                            c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x10));
                            c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x1f));
                            c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018b4dc);
                                }
                                continue 'l_18b4dc;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18b560;
                                } else {
                                    break 'b_18b530;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                'l_18b534: loop {
                    c.cycles += 11;
                    c.g[V1][0] = c.ld(c.g[A1][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[V1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018b534);
                        }
                        continue 'l_18b534;
                    } else {
                        break 'b_18b560;
                    }
                }
            }
            'b_18b61c: {
                'b_18b5ec: {
                    c.cycles += 13;
                    c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[A1][0] as u32);
                    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(2, c.g[A0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.g[RA][0] = sx(0x0018b580);
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                    call_ptr(c, 0x0028b180, 0x0018b580)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5610));
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t {
                        break 'b_18b5ec;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                        'l_18b598: loop {
                            c.cycles += 19;
                            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.ldr(9, c.g[V1][0] as u32);
                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                            c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.sdr(9, c.g[A1][0] as u32);
                            c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.sdr(6, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018b598);
                                }
                                continue 'l_18b598;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18b61c;
                                } else {
                                    break 'b_18b5ec;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                'l_18b5f0: loop {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018b5f0);
                        }
                        continue 'l_18b5f0;
                    } else {
                        break 'b_18b61c;
                    }
                }
            }
            c.cycles += 7;
            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(9, c.g[V1][0] as u32);
            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(9, c.g[A1][0] as u32);
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x0018b638);
            c.g[A1][0] = c.g[A1][0] & 0xf;
            call_ptr(c, 0x00287410, 0x0018b638)?;
            break 'b_18b638;
        }
    }
    'b_18b7e0: {
        c.cycles += 8;
        c.g[RA][0] = sx(0x0018b640);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1598));
        call_ptr(c, 0x00317670, 0x0018b640)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x0018b64c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x0018b64c)?;
        c.g[A2][0] = c.g[V0][0];
        let t = c.g[A2][0] == 0u64;
        c.g[V1][0] = 0x530000;
        if t {
            break 'b_18b7e0;
        } else {
            'b_18b700: {
                'b_18b6d0: {
                    c.cycles += 8;
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                    c.g[S5][0] = c.g[V1][0];
                    c.g[V0][0] = c.g[A1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = c.g[A0][0];
                    if t {
                        break 'b_18b6d0;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        'l_18b67c: loop {
                            c.cycles += 19;
                            c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.ldr(8, c.g[A1][0] as u32);
                            c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.sdr(8, c.g[A0][0] as u32);
                            c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0xf));
                            c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0x17));
                            c.sdr(3, (c.g[A0][0] as u32).wrapping_add(0x10));
                            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x1f));
                            c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018b67c);
                                }
                                continue 'l_18b67c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18b700;
                                } else {
                                    break 'b_18b6d0;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                'l_18b6d4: loop {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[A1][0] as u32);
                    c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018b6d4);
                        }
                        continue 'l_18b6d4;
                    } else {
                        break 'b_18b700;
                    }
                }
            }
            'b_18b7c0: {
                'b_18b790: {
                    c.cycles += 14;
                    c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(8, c.g[A1][0] as u32);
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(8, c.g[A0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.g[RA][0] = sx(0x0018b724);
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                    call_ptr(c, 0x0028b180, 0x0018b724)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5610));
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t {
                        break 'b_18b790;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                        'l_18b73c: loop {
                            c.cycles += 19;
                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.ldr(6, c.g[V1][0] as u32);
                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                            c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                            c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.sdr(6, c.g[A1][0] as u32);
                            c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018b73c);
                                }
                                continue 'l_18b73c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18b7c0;
                                } else {
                                    break 'b_18b790;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                'l_18b794: loop {
                    c.cycles += 11;
                    c.g[A2][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018b794);
                        }
                        continue 'l_18b794;
                    } else {
                        break 'b_18b7c0;
                    }
                }
            }
            c.cycles += 8;
            c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(2, c.g[V1][0] as u32);
            c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[A1][0] as u32);
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 8);
            c.g[RA][0] = sx(0x0018b7e0);
            c.g[A1][0] = c.g[A1][0] & 0xf;
            call_ptr(c, 0x00287488, 0x0018b7e0)?;
            break 'b_18b7e0;
        }
    }
    'b_18b980: {
        c.cycles += 8;
        c.g[RA][0] = sx(0x0018b7e8);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x15a0));
        call_ptr(c, 0x00317670, 0x0018b7e8)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x0018b7f4);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x0018b7f4)?;
        c.g[A2][0] = c.g[V0][0];
        let t = c.g[A2][0] == 0u64;
        c.g[V1][0] = 0x530000;
        if t {
            break 'b_18b980;
        } else {
            'b_18b8a8: {
                'b_18b878: {
                    c.cycles += 8;
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                    c.g[S5][0] = c.g[V1][0];
                    c.g[V0][0] = c.g[A1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = c.g[A0][0];
                    if t {
                        break 'b_18b878;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        'l_18b824: loop {
                            c.cycles += 19;
                            c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.ldr(9, c.g[A1][0] as u32);
                            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.sdr(9, c.g[A0][0] as u32);
                            c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0xf));
                            c.sdr(3, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                            c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                            c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                            c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018b824);
                                }
                                continue 'l_18b824;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18b8a8;
                                } else {
                                    break 'b_18b878;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                'l_18b87c: loop {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A1][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018b87c);
                        }
                        continue 'l_18b87c;
                    } else {
                        break 'b_18b8a8;
                    }
                }
            }
            'b_18b964: {
                'b_18b934: {
                    c.cycles += 13;
                    c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(9, c.g[A1][0] as u32);
                    c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(9, c.g[A0][0] as u32);
                    c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.g[V0][0] >> 4;
                    c.g[RA][0] = sx(0x0018b8c8);
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                    call_ptr(c, 0x0028b180, 0x0018b8c8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5610));
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t {
                        break 'b_18b934;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                        'l_18b8e0: loop {
                            c.cycles += 19;
                            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.ldr(8, c.g[V1][0] as u32);
                            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
                            c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.sdr(8, c.g[A1][0] as u32);
                            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.sdr(6, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018b8e0);
                                }
                                continue 'l_18b8e0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18b964;
                                } else {
                                    break 'b_18b934;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                'l_18b938: loop {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[V1][0] as u32);
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018b938);
                        }
                        continue 'l_18b938;
                    } else {
                        break 'b_18b964;
                    }
                }
            }
            c.cycles += 7;
            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(8, c.g[V1][0] as u32);
            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(8, c.g[A1][0] as u32);
            c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x0018b980);
            c.g[A1][0] = c.g[A1][0] >> 4;
            call_ptr(c, 0x00287520, 0x0018b980)?;
            break 'b_18b980;
        }
    }
    'b_18bb84: {
        c.cycles += 8;
        c.g[RA][0] = sx(0x0018b988);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x15a8));
        call_ptr(c, 0x00317670, 0x0018b988)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x0018b994);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x0018b994)?;
        c.g[S0][0] = c.g[V0][0];
        let t = c.g[S0][0] == 0u64;
        if t {
            break 'b_18bb84;
        } else {
            'b_18ba8c: {
                'b_18ba58: {
                    c.cycles += 23;
                    c.g[RA][0] = sx(0x0018b9a8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a670, 0x0018b9a8)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x0018b9b4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20c0));
                    call_ptr(c, 0x00317670, 0x0018b9b4)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0018b9c4);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x0018b9c4)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x0018b9d0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fd8));
                    call_ptr(c, 0x00317670, 0x0018b9d0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0018b9e0);
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039a568, 0x0018b9e0)?;
                    c.g[V1][0] = 0x530000;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                    c.g[S5][0] = c.g[V1][0];
                    c.g[V0][0] = c.g[A1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = c.g[A0][0];
                    if t {
                        break 'b_18ba58;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2a0));
                        'l_18ba04: loop {
                            c.cycles += 19;
                            c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.ldr(7, c.g[A1][0] as u32);
                            c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.sdr(7, c.g[A0][0] as u32);
                            c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xf));
                            c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x17));
                            c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x10));
                            c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0x1f));
                            c.sdr(3, (c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018ba04);
                                }
                                continue 'l_18ba04;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18ba8c;
                                } else {
                                    break 'b_18ba58;
                                }
                            }
                        }
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2a0));
                'l_18ba60: loop {
                    c.cycles += 11;
                    c.g[A3][0] = c.ld(c.g[A1][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018ba60);
                        }
                        continue 'l_18ba60;
                    } else {
                        break 'b_18ba8c;
                    }
                }
            }
            'b_18bac0: {
                c.cycles += 11;
                c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(7, c.g[A1][0] as u32);
                c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                c.sdr(7, c.g[A0][0] as u32);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 17);
                c.g[RA][0] = sx(0x0018bab0);
                c.g[A1][0] = c.g[A1][0] & 0x1;
                call_ptr(c, 0x0039a768, 0x0018bab0)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x2a0));
                if t {
                    break 'b_18bac0;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0018bac0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a7a8, 0x0018bac0)?;
                    break 'b_18bac0;
                }
            }
            'b_18bb64: {
                'b_18bb34: {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x0018bac8);
                    call_ptr(c, 0x0028b180, 0x0018bac8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5610));
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t {
                        break 'b_18bb34;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                        'l_18bae0: loop {
                            c.cycles += 19;
                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.ldr(6, c.g[V1][0] as u32);
                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                            c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                            c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.sdr(6, c.g[A1][0] as u32);
                            c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018bae0);
                                }
                                continue 'l_18bae0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18bb64;
                                } else {
                                    break 'b_18bb34;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                'l_18bb38: loop {
                    c.cycles += 11;
                    c.g[A2][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018bb38);
                        }
                        continue 'l_18bb38;
                    } else {
                        break 'b_18bb64;
                    }
                }
            }
            c.cycles += 8;
            c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(2, c.g[V1][0] as u32);
            c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[A1][0] as u32);
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
            c.g[RA][0] = sx(0x0018bb84);
            c.g[A1][0] = c.g[A1][0] & 0x1;
            call_ptr(c, 0x00287558, 0x0018bb84)?;
            break 'b_18bb84;
        }
    }
    'b_18bd84: {
        c.cycles += 8;
        c.g[RA][0] = sx(0x0018bb8c);
        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x15b0));
        call_ptr(c, 0x00317670, 0x0018bb8c)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x0018bb98);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x0018bb98)?;
        c.g[S0][0] = c.g[V0][0];
        let t = c.g[S0][0] == 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
        if t {
            break 'b_18bd84;
        } else {
            'b_18bc90: {
                'b_18bc5c: {
                    c.cycles += 23;
                    c.g[RA][0] = sx(0x0018bbac);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a670, 0x0018bbac)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x0018bbb8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20c0));
                    call_ptr(c, 0x00317670, 0x0018bbb8)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0018bbc8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x0018bbc8)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x0018bbd4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fd8));
                    call_ptr(c, 0x00317670, 0x0018bbd4)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0018bbe4);
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039a568, 0x0018bbe4)?;
                    c.g[V1][0] = 0x530000;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                    c.g[S5][0] = c.g[V1][0];
                    c.g[V0][0] = c.g[A1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = c.g[A0][0];
                    if t {
                        break 'b_18bc5c;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2a1));
                        'l_18bc08: loop {
                            c.cycles += 19;
                            c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.ldr(9, c.g[A1][0] as u32);
                            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.sdr(9, c.g[A0][0] as u32);
                            c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0xf));
                            c.sdr(3, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                            c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                            c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                            c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018bc08);
                                }
                                continue 'l_18bc08;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18bc90;
                                } else {
                                    break 'b_18bc5c;
                                }
                            }
                        }
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2a1));
                'l_18bc64: loop {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A1][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018bc64);
                        }
                        continue 'l_18bc64;
                    } else {
                        break 'b_18bc90;
                    }
                }
            }
            'b_18bcc0: {
                c.cycles += 10;
                c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(9, c.g[A1][0] as u32);
                c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                c.sdr(9, c.g[A0][0] as u32);
                c.g[A1][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x12));
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0018bcb0);
                c.g[A1][0] = c.g[A1][0] & 0x1;
                call_ptr(c, 0x0039a768, 0x0018bcb0)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x2a1));
                if t {
                    break 'b_18bcc0;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0018bcc0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a7a8, 0x0018bcc0)?;
                    break 'b_18bcc0;
                }
            }
            'b_18bd64: {
                'b_18bd34: {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x0018bcc8);
                    call_ptr(c, 0x0028b180, 0x0018bcc8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5610));
                    c.g[V0][0] = c.g[V1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t {
                        break 'b_18bd34;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                        'l_18bce0: loop {
                            c.cycles += 19;
                            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.ldr(8, c.g[V1][0] as u32);
                            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
                            c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                            c.sdr(8, c.g[A1][0] as u32);
                            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0xf));
                            c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x17));
                            c.sdr(6, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0x1f));
                            c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x18));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018bce0);
                                }
                                continue 'l_18bce0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                if t {
                                    break 'b_18bd64;
                                } else {
                                    break 'b_18bd34;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                'l_18bd38: loop {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[V1][0] as u32);
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018bd38);
                        }
                        continue 'l_18bd38;
                    } else {
                        break 'b_18bd64;
                    }
                }
            }
            c.cycles += 8;
            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(8, c.g[V1][0] as u32);
            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(8, c.g[A1][0] as u32);
            c.g[A1][0] = c.lh((c.g[SP][0] as u32).wrapping_add(0x12));
            c.g[RA][0] = sx(0x0018bd80);
            c.g[A1][0] = c.g[A1][0] & 0x1;
            call_ptr(c, 0x002875d0, 0x0018bd80)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
            break 'b_18bd84;
        }
    }
    'b_18bed0: {
        c.cycles += 3;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        let t = c.g[V0][0] != 0u64;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x320));
        if t {
            break 'b_18bed0;
        } else {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x0018bd9c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15b8));
            call_ptr(c, 0x00317670, 0x0018bd9c)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x0018bda8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x0018bda8)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x320));
                break 'b_18bed0;
            } else {
                'b_18be98: {
                    'b_18be64: {
                        c.cycles += 21;
                        c.g[RA][0] = sx(0x0018bdbc);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0039a670, 0x0018bdbc)?;
                        c.g[A0][0] = 0x460000;
                        c.g[RA][0] = sx(0x0018bdc8);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20c0));
                        call_ptr(c, 0x00317670, 0x0018bdc8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A2][0] = 0u64;
                        c.g[RA][0] = sx(0x0018bdd8);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0039a568, 0x0018bdd8)?;
                        c.g[A0][0] = 0x460000;
                        c.g[RA][0] = sx(0x0018bde4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fd8));
                        call_ptr(c, 0x00317670, 0x0018bde4)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x0018bdf4);
                        c.g[A2][0] = 0x1;
                        call_ptr(c, 0x0039a568, 0x0018bdf4)?;
                        c.g[V1][0] = 0x530000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                        c.g[V0][0] = c.g[V1][0] & 0x7;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_18be64;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2a2));
                            'l_18be10: loop {
                                c.cycles += 19;
                                c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                c.ldr(7, c.g[V1][0] as u32);
                                c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                                c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                                c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x17));
                                c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x10));
                                c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x18));
                                c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                                c.sdr(7, c.g[A0][0] as u32);
                                c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xf));
                                c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                                c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x17));
                                c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x10));
                                c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x18));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0018be10);
                                    }
                                    continue 'l_18be10;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    if t {
                                        break 'b_18be98;
                                    } else {
                                        break 'b_18be64;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2a2));
                    'l_18be6c: loop {
                        c.cycles += 11;
                        c.g[A3][0] = c.ld(c.g[V1][0] as u32);
                        c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sd(c.g[A0][0] as u32, c.g[A3][0]);
                        c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                        c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                        c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0018be6c);
                            }
                            continue 'l_18be6c;
                        } else {
                            break 'b_18be98;
                        }
                    }
                }
                'b_18becc: {
                    c.cycles += 11;
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(7, c.g[V1][0] as u32);
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(7, c.g[A0][0] as u32);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 18);
                    c.g[RA][0] = sx(0x0018bebc);
                    c.g[A1][0] = c.g[A1][0] & 0x1;
                    call_ptr(c, 0x0039a768, 0x0018bebc)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x2a2));
                    if t {
                        break 'b_18becc;
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x0018becc);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0039a7a8, 0x0018becc)?;
                        break 'b_18becc;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x320));
                break 'b_18bed0;
            }
        }
    }
    c.cycles += 9;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x310));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x300));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2f0));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x2e0));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x2d0));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x2c0));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x2b0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x330));
    c.pc = t;
    return Ok(());
}

/// `cFEStateOptionsSound_OnActivate(self)` (0x18a870): hand the activation to
/// the base state (`UIState_OnActivate`).
pub fn c_fe_state_options_sound_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_e4c0, 0x0018_a880)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOptionsHUD_OnActivate(self)` (0x18c198): hand the activation to
/// the base state (`UIState_OnActivate`).
pub fn c_fe_state_options_hud_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_e4c0, 0x0018_c1a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// The bodies below keep the generated block structure (labelled blocks and the
// struct-copy loops, whose unaligned halves are never taken at the fixed
// config address 0x535610).

/// `cFEStateOptionsSound_Init(self)` (0x18a298): create the advanced sound settings screen, start it, refresh the controls and show or hide the on/off groups and the extra item from the surround flag.
pub fn c_fe_state_options_sound_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_18a2ec: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2040));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x0018a2c0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x0018a2d4)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_18a2ec;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x0018a2ec)?;
                break 'b_18a2ec;
            }
        }
        'b_18a338: {
            c.cycles += 14;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0018b380, 0x0018a2f4)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1580));
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            c.g[S2][0] = (c.g[V0][0] < 0x1u64) as u64;
            call_ptr(c, 0x00317670, 0x0018a30c)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x0018a318)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S2][0];
            if t {
                break 'b_18a338;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018a338)?;
                break 'b_18a338;
            }
        }
        'b_18a370: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1588));
            call_ptr(c, 0x00317670, 0x0018a344)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x0018a350)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S2][0] ^ 0x1;
            if t {
                break 'b_18a370;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018a370)?;
                break 'b_18a370;
            }
        }
        'b_18a3c0: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1528));
            call_ptr(c, 0x00317670, 0x0018a37c)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x0018a388)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = c.g[S2][0];
            if t {
                break 'b_18a3c0;
            } else {
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018a3a8)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[S2][0] ^ 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018a3c0)?;
                break 'b_18a3c0;
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
    })
}

/// `cFEStateOptionsSound_OnChildEvent(self, popup, event)` (0x18a3d8): on the reset-options popup answer, restore the default sound options (speaker mode, volumes, DJ speech, arcade SFX).
pub fn c_fe_state_options_sound_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_18a858: {
            'b_18a854: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2e0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2d0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                c.g[S0][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2a0));
                c.g[S1][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2b0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x290), c.g[RA][0]);
                c.g[S3][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x0018a400)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x4;
                call_ptr(c, 0x0014dd58, 0x0018a40c)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = 0x15;
                let t = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 0x16;
                if t {
                    break 'b_18a854;
                } else {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2d0));
                    if t {
                        break 'b_18a858;
                    } else {
                        c.cycles += 6;
                        c.g[A0][0] = 0x460000;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x22e0));
                        call_ptr(c, 0x00317670, 0x0018a434)?;
                        let t = c.g[S0][0] != c.g[V0][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2d0));
                        if t {
                            break 'b_18a858;
                        } else {
                            'b_18a850: {
                                c.cycles += 8;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xbc));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0018a454)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_18a850;
                                } else {
                                    'b_18a524: {
                                        'b_18a4f4: {
                                            c.cycles += 16;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[S0][0] = 0x530000;
                                            call_ptr(c, 0x0014f458, 0x0018a468)?;
                                            c.g[S1][0] = c.g[S0][0];
                                            c.g[A1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x5610));
                                            c.g[A0][0] = c.g[S3][0];
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32).wrapping_add(0x5610));
                                            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 12);
                                            c.g[A1][0] = c.g[A1][0] & 0x3;
                                            call_ptr(c, 0x0018bef8, 0x0018a484)?;
                                            call_ptr(c, 0x0028b180, 0x0018a48c)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[V0][0] = c.g[S0][0] & 0x7;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] = c.g[SP][0];
                                            if t {
                                                break 'b_18a4f4;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x280));
                                                'l_18a4a0: loop {
                                                    c.cycles += 19;
                                                    c.ldl(5, (c.g[S0][0] as u32).wrapping_add(0x7));
                                                    c.ldr(5, c.g[S0][0] as u32);
                                                    c.ldl(6, (c.g[S0][0] as u32).wrapping_add(0xf));
                                                    c.ldr(6, (c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.ldl(
                                                        7,
                                                        (c.g[S0][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.ldr(
                                                        7,
                                                        (c.g[S0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.ldl(
                                                        8,
                                                        (c.g[S0][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.ldr(
                                                        8,
                                                        (c.g[S0][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.sdr(5, c.g[V1][0] as u32);
                                                    c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                    c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.sdr(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sdl(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.sdr(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.g[S0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0018a4a0);
                                                        }
                                                        continue 'l_18a4a0;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_18a524;
                                                        } else {
                                                            break 'b_18a4f4;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x280));
                                        'l_18a4f8: loop {
                                            c.cycles += 11;
                                            c.g[T1][0] = c.ld(c.g[S0][0] as u32);
                                            c.g[A1][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A2][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x10));
                                            c.g[A3][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x18));
                                            c.sd(c.g[V1][0] as u32, c.g[T1][0]);
                                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[A2][0],
                                            );
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x18),
                                                c.g[A3][0],
                                            );
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0018a4f8);
                                                }
                                                continue 'l_18a4f8;
                                            } else {
                                                break 'b_18a524;
                                            }
                                        }
                                    }
                                    'b_18a5e4: {
                                        'b_18a5b4: {
                                            c.cycles += 14;
                                            c.ldl(8, (c.g[S0][0] as u32).wrapping_add(0x7));
                                            c.ldr(8, c.g[S0][0] as u32);
                                            c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                                            c.sdr(8, c.g[V1][0] as u32);
                                            c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                                            c.g[A1][0] = c.g[A1][0] & 0xf;
                                            call_ptr(c, 0x00287410, 0x0018a540)?;
                                            call_ptr(c, 0x0028b180, 0x0018a548)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x5610));
                                            c.g[V0][0] = c.g[A1][0] & 0x7;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] = c.g[SP][0];
                                            if t {
                                                break 'b_18a5b4;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                                'l_18a560: loop {
                                                    c.cycles += 19;
                                                    c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                                                    c.ldr(6, c.g[A1][0] as u32);
                                                    c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                                                    c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                                                    c.ldl(
                                                        8,
                                                        (c.g[A1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.ldr(
                                                        8,
                                                        (c.g[A1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.ldl(
                                                        9,
                                                        (c.g[A1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.ldr(
                                                        9,
                                                        (c.g[A1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.sdr(6, c.g[V1][0] as u32);
                                                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.sdr(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sdl(
                                                        9,
                                                        (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.sdr(
                                                        9,
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[A1][0] != c.g[V0][0];
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0018a560);
                                                        }
                                                        continue 'l_18a560;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_18a5e4;
                                                        } else {
                                                            break 'b_18a5b4;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                        'l_18a5b8: loop {
                                            c.cycles += 11;
                                            c.g[A2][0] = c.ld(c.g[A1][0] as u32);
                                            c.g[A3][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                                            c.g[T0][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                                            c.g[T1][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                                            c.sd(c.g[V1][0] as u32, c.g[A2][0]);
                                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[T0][0],
                                            );
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x18),
                                                c.g[T1][0],
                                            );
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                            let t = c.g[A1][0] != c.g[V0][0];
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0018a5b8);
                                                }
                                                continue 'l_18a5b8;
                                            } else {
                                                break 'b_18a5e4;
                                            }
                                        }
                                    }
                                    'b_18a6a8: {
                                        'b_18a678: {
                                            c.cycles += 15;
                                            c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                                            c.ldr(2, c.g[A1][0] as u32);
                                            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                                            c.sdr(2, c.g[V1][0] as u32);
                                            c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 8);
                                            c.g[A1][0] = c.g[A1][0] & 0xf;
                                            call_ptr(c, 0x00287488, 0x0018a604)?;
                                            call_ptr(c, 0x0028b180, 0x0018a60c)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x5610));
                                            c.g[V0][0] = c.g[A1][0] & 0x7;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] = c.g[SP][0];
                                            if t {
                                                break 'b_18a678;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                                'l_18a624: loop {
                                                    c.cycles += 19;
                                                    c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                                                    c.ldr(9, c.g[A1][0] as u32);
                                                    c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0xf));
                                                    c.ldr(6, (c.g[A1][0] as u32).wrapping_add(0x8));
                                                    c.ldl(
                                                        7,
                                                        (c.g[A1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.ldr(
                                                        7,
                                                        (c.g[A1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.ldl(
                                                        8,
                                                        (c.g[A1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.ldr(
                                                        8,
                                                        (c.g[A1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.sdr(9, c.g[V1][0] as u32);
                                                    c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                    c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.sdr(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sdl(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.sdr(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[A1][0] != c.g[V0][0];
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0018a624);
                                                        }
                                                        continue 'l_18a624;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_18a6a8;
                                                        } else {
                                                            break 'b_18a678;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                        'l_18a67c: loop {
                                            c.cycles += 11;
                                            c.g[T1][0] = c.ld(c.g[A1][0] as u32);
                                            c.g[A2][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                                            c.g[A3][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                                            c.g[T0][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                                            c.sd(c.g[V1][0] as u32, c.g[T1][0]);
                                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[A3][0],
                                            );
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x18),
                                                c.g[T0][0],
                                            );
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                            let t = c.g[A1][0] != c.g[V0][0];
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0018a67c);
                                                }
                                                continue 'l_18a67c;
                                            } else {
                                                break 'b_18a6a8;
                                            }
                                        }
                                    }
                                    'b_18a768: {
                                        'b_18a738: {
                                            c.cycles += 14;
                                            c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                                            c.ldr(9, c.g[A1][0] as u32);
                                            c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
                                            c.sdr(9, c.g[V1][0] as u32);
                                            c.g[A1][0] = c.lbu(c.g[SP][0] as u32);
                                            c.g[A1][0] = c.g[A1][0] >> 4;
                                            call_ptr(c, 0x00287520, 0x0018a6c4)?;
                                            call_ptr(c, 0x0028b180, 0x0018a6cc)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x5610));
                                            c.g[V0][0] = c.g[A1][0] & 0x7;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] = c.g[SP][0];
                                            if t {
                                                break 'b_18a738;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                                'l_18a6e4: loop {
                                                    c.cycles += 19;
                                                    c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                                                    c.ldr(8, c.g[A1][0] as u32);
                                                    c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0xf));
                                                    c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x8));
                                                    c.ldl(
                                                        6,
                                                        (c.g[A1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.ldr(
                                                        6,
                                                        (c.g[A1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.ldl(
                                                        7,
                                                        (c.g[A1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.ldr(
                                                        7,
                                                        (c.g[A1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.sdr(8, c.g[V1][0] as u32);
                                                    c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                    c.sdr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        6,
                                                        (c.g[V1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.sdr(
                                                        6,
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sdl(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.sdr(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[A1][0] != c.g[V0][0];
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0018a6e4);
                                                        }
                                                        continue 'l_18a6e4;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_18a768;
                                                        } else {
                                                            break 'b_18a738;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
                                        'l_18a73c: loop {
                                            c.cycles += 11;
                                            c.g[T0][0] = c.ld(c.g[A1][0] as u32);
                                            c.g[T1][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                                            c.g[A2][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                                            c.g[A3][0] =
                                                c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                                            c.sd(c.g[V1][0] as u32, c.g[T0][0]);
                                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[A2][0],
                                            );
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x18),
                                                c.g[A3][0],
                                            );
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                            let t = c.g[A1][0] != c.g[V0][0];
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0018a73c);
                                                }
                                                continue 'l_18a73c;
                                            } else {
                                                break 'b_18a768;
                                            }
                                        }
                                    }
                                    'b_18a828: {
                                        'b_18a7f8: {
                                            c.cycles += 14;
                                            c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                                            c.ldr(8, c.g[A1][0] as u32);
                                            c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                                            c.sdr(8, c.g[V1][0] as u32);
                                            c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                                            c.g[A1][0] =
                                                sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
                                            c.g[A1][0] = c.g[A1][0] & 0x1;
                                            call_ptr(c, 0x00287558, 0x0018a788)?;
                                            c.g[S0][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x5610));
                                            call_ptr(c, 0x0028b180, 0x0018a790)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[V0][0] = c.g[S0][0] & 0x7;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] = c.g[SP][0];
                                            if t {
                                                break 'b_18a7f8;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x280));
                                                'l_18a7a4: loop {
                                                    c.cycles += 19;
                                                    c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x7));
                                                    c.ldr(7, c.g[S0][0] as u32);
                                                    c.ldl(8, (c.g[S0][0] as u32).wrapping_add(0xf));
                                                    c.ldr(8, (c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.ldl(
                                                        9,
                                                        (c.g[S0][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.ldr(
                                                        9,
                                                        (c.g[S0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.ldl(
                                                        5,
                                                        (c.g[S0][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.ldr(
                                                        5,
                                                        (c.g[S0][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.sdr(7, c.g[V1][0] as u32);
                                                    c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                    c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        9,
                                                        (c.g[V1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.sdr(
                                                        9,
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sdl(
                                                        5,
                                                        (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.sdr(
                                                        5,
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.g[S0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0018a7a4);
                                                        }
                                                        continue 'l_18a7a4;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_18a828;
                                                        } else {
                                                            break 'b_18a7f8;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x280));
                                        'l_18a7fc: loop {
                                            c.cycles += 11;
                                            c.g[A2][0] = c.ld(c.g[S0][0] as u32);
                                            c.g[A3][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[T0][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x10));
                                            c.g[T1][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x18));
                                            c.sd(c.g[V1][0] as u32, c.g[A2][0]);
                                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[T0][0],
                                            );
                                            c.sd(
                                                (c.g[V1][0] as u32).wrapping_add(0x18),
                                                c.g[T1][0],
                                            );
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0018a7fc);
                                                }
                                                continue 'l_18a7fc;
                                            } else {
                                                break 'b_18a828;
                                            }
                                        }
                                    }
                                    c.cycles += 10;
                                    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x7));
                                    c.ldr(2, c.g[S0][0] as u32);
                                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                                    c.sdr(2, c.g[V1][0] as u32);
                                    c.g[A1][0] = c.lh((c.g[SP][0] as u32).wrapping_add(0x2));
                                    c.g[A1][0] = c.g[A1][0] & 0x1;
                                    call_ptr(c, 0x002875d0, 0x0018a844)?;
                                    c.g[A0][0] = c.g[S3][0];
                                    call_ptr(c, 0x0018b380, 0x0018a84c)?;
                                    c.g[V0][0] = 0x1;
                                    break 'b_18a850;
                                }
                            }
                            c.cycles += 1;
                            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c1c), c.g[V0][0]);
                            break 'b_18a854;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2d0));
            break 'b_18a858;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2b0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2a0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x290));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2e0));
        c.pc = t;
        Ok(())
    })
}

/// `cFEStateOptionsSound_SetSpeakerMode(self, mode)` (0x18bef8): map the speaker mode (0 and default 1, 1 -> 2, 2 -> 0, 3 -> 3) to the audio mode; when the music system accepts it, restart the audio engine with that mode and restore the volumes and flags.
pub fn c_fe_state_options_sound_set_speaker_mode(c: &mut Cpu) {
    guest(c, |c| {
        'b_18bf60: {
            'b_18bf5c: {
                'b_18bf54: {
                    'b_18bf4c: {
                        'b_18bf44: {
                            c.cycles += 11;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                            c.g[V0][0] = 0x1;
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x58));
                            let t = c.g[A1][0] == c.g[V0][0];
                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                            if t {
                                break 'b_18bf44;
                            } else {
                                c.cycles += 2;
                                let t = c.g[A1][0] == 0u64;
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_18bf5c;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_18bf4c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[A1][0] == c.g[V0][0];
                                        c.g[S3][0] = 0x1;
                                        if t {
                                            break 'b_18bf54;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_18bf60;
                                            } else {
                                                break 'b_18bf44;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = true;
                        c.g[S3][0] = 0x2;
                        if t {
                            break 'b_18bf60;
                        } else {
                            break 'b_18bf4c;
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[S3][0] = 0u64;
                    if t {
                        break 'b_18bf60;
                    } else {
                        break 'b_18bf54;
                    }
                }
                c.cycles += 2;
                let t = true;
                c.g[S3][0] = 0x3;
                if t {
                    break 'b_18bf60;
                } else {
                    break 'b_18bf5c;
                }
            }
            c.cycles += 1;
            c.g[S3][0] = 0x1;
            break 'b_18bf60;
        }
        'b_18c0c0: {
            c.cycles += 14;
            call_ptr(c, 0x0028b180, 0x0018bf68)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = c.g[S3][0];
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018bf8c)?;
            c.g[V1][0] = 0x1;
            let t = c.g[V0][0] != c.g[V1][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            if t {
                break 'b_18c0c0;
            } else {
                c.cycles += 74;
                call_ptr(c, 0x0028b180, 0x0018bfa0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00287670, 0x0018bfa8)?;
                c.mtc1(2, 20);
                c.cvt_s_w(20, 20);
                call_ptr(c, 0x0028b180, 0x0018bfb8)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002876a0, 0x0018bfc0)?;
                c.mtc1(2, 21);
                c.cvt_s_w(21, 21);
                call_ptr(c, 0x0028b180, 0x0018bfd0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002876d0, 0x0018bfd8)?;
                c.mtc1(2, 22);
                c.cvt_s_w(22, 22);
                call_ptr(c, 0x0028b180, 0x0018bfe8)?;
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x62b8));
                call_ptr(c, 0x0028b180, 0x0018bff0)?;
                c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x62b4));
                call_ptr(c, 0x0028b180, 0x0018bff8)?;
                c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x534));
                call_ptr(c, 0x0028b180, 0x0018c000)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00286200, 0x0018c008)?;
                call_ptr(c, 0x00284c28, 0x0018c010)?;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x00284bb0, 0x0018c018)?;
                call_ptr(c, 0x0028b180, 0x0018c020)?;
                c.cvt_w_s(0, 20);
                c.mfc1(5, 0);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00287410, 0x0018c030)?;
                call_ptr(c, 0x0028b180, 0x0018c038)?;
                c.cvt_w_s(0, 21);
                c.mfc1(5, 0);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00287488, 0x0018c048)?;
                call_ptr(c, 0x0028b180, 0x0018c050)?;
                c.cvt_w_s(0, 22);
                c.mfc1(5, 0);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00287520, 0x0018c060)?;
                call_ptr(c, 0x0028b180, 0x0018c068)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00287558, 0x0018c074)?;
                call_ptr(c, 0x0028b180, 0x0018c07c)?;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002875d0, 0x0018c088)?;
                call_ptr(c, 0x0028b180, 0x0018c090)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00285fb0, 0x0018c098)?;
                call_ptr(c, 0x0028b180, 0x0018c0a0)?;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5558));
                c.g[A1][0] = c.g[S2][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                let t = c.g[A2][0] as u32;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018c0bc)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_18c0c0;
            }
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x58));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `cFEStateOptionsHUD_OnWidgetAction(self, widget, action)` (0x18c2a0): handle the HUD options widget actions: open the reset popup (7), go to the previous state (6), show the widget (1), hide it for the three listed names (2).
pub fn c_fe_state_options_hud_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_18c460: {
            'b_18c45c: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A2][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A1][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                let t = c.g[S1][0] == 0u64;
                c.g[S2][0] = c.g[A0][0];
                if t {
                    break 'b_18c45c;
                } else {
                    'b_18c3fc: {
                        c.cycles += 9;
                        call_ptr(c, 0x0014dc80, 0x0018c2cc)?;
                        c.g[A1][0] = 0x4;
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dd58, 0x0018c2d8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0x2;
                        let t = c.g[S0][0] == c.g[V0][0];
                        c.g[V0][0] = (c.g[S0][0] < 0x3u64) as u64;
                        if t {
                            break 'b_18c3fc;
                        } else {
                            'b_18c3bc: {
                                'b_18c300: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_18c300;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[S0][0] == c.g[V0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            break 'b_18c3bc;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_18c460;
                                            } else {
                                                break 'b_18c300;
                                            }
                                        }
                                    }
                                }
                                'b_18c368: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x6;
                                    let t = c.g[S0][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x7;
                                    if t {
                                        break 'b_18c368;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S0][0] != c.g[V0][0];
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_18c460;
                                        } else {
                                            c.cycles += 21;
                                            c.g[A1][0] = 0x460000;
                                            c.g[A0][0] = 0x4c;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_sub(0x2298));
                                            c.g[A2][0] = 0x100;
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x0018c32c)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            call_ptr(c, 0x0039e318, 0x0018c340)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                            c.g[V0][0] = 0x470000;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0x4c30));
                                            c.g[A1][0] = c.g[S0][0];
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            c.sb((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                                            call_ptr(c, 0x0039f290, 0x0018c360)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_18c460;
                                            } else {
                                                break 'b_18c368;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 16;
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0018c37c)?;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0018c3a0)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = c.g[V0][0];
                                if t {
                                    break 'b_18c45c;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f400, 0x0018c3b4)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_18c460;
                                    } else {
                                        break 'b_18c3bc;
                                    }
                                }
                            }
                            c.cycles += 16;
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018c3d0)?;
                            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c18), c.g[S0][0]);
                            c.g[A1][0] = 0x530000;
                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x5610));
                            c.g[V0][0] = 0x3fff0000;
                            c.g[V1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x18));
                            c.g[V0][0] = c.g[V0][0] | 0xffff;
                            c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 30);
                            c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
                            let t = true;
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x5610), c.g[A0][0]);
                            if t {
                                break 'b_18c45c;
                            } else {
                                break 'b_18c3fc;
                            }
                        }
                    }
                    'b_18c448: {
                        c.cycles += 6;
                        c.g[A0][0] = 0x4a0000;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14f8));
                        call_ptr(c, 0x00317670, 0x0018c40c)?;
                        if c.g[S0][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            break 'b_18c448;
                        } else {
                            c.cycles += 6;
                            c.g[A0][0] = 0x4a0000;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1500));
                            call_ptr(c, 0x00317670, 0x0018c424)?;
                            if c.g[S0][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                break 'b_18c448;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = 0x4a0000;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1508));
                                call_ptr(c, 0x00317670, 0x0018c43c)?;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_18c460;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_18c448;
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018c45c)?;
                    break 'b_18c45c;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_18c460;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `cFEStateOptionsController_SetVibration` (0x18d108, not ported).
pub(crate) const OC838_SET_VIBRATION: Callee = Callee {
    addr: 0x0018_d108,
    f: ee::s02::f_0018d108,
};

/// `cFEStateOptionsController_RefreshListBoxes` (0x18cdc0).
pub(crate) const OC838_REFRESH_LIST_BOXES: Callee = Callee {
    addr: 0x0018_cdc0,
    f: ee::s02::f_0018cdc0,
};

/// `cFEStateOptionsController_ShowConfigText` (0x18cf50).
pub(crate) const OC838_SHOW_CONFIG_TEXT: Callee = Callee {
    addr: 0x0018_cf50,
    f: ee::s02::f_0018cf50,
};

/// `lw off(reg)` / `lh off(reg)` with the register's low 32 bits.
macro_rules! lwr {
    ($c:expr, $reg:expr, $off:expr) => {{
        let base = $c.g[$reg][0] as u32;
        $c.lw(base.wrapping_add($off))
    }};
}

macro_rules! lhr {
    ($c:expr, $reg:expr, $off:expr) => {{
        let base = $c.g[$reg][0] as u32;
        $c.lh(base.wrapping_add($off))
    }};
}

/// `sx(base + off)` (`addiu`).
pub(crate) fn add_at(base: u64, off: u32) -> u64 {
    sx((base as u32).wrapping_add(off))
}

/// `SetVibration(self, pad, 0.0f, 0.0f)` for pad 0 and pad 1 (the generated
/// pair of calls with `f20 = 0`); `self` is in `self_reg`, the first call
/// returns to `ret0`, the second to `ret1` (f13 taken from f12 the second
/// time, as the generated body does).
pub(crate) fn oc838_vibrate_both(c: &mut Cpu, self_reg: usize, ret0: u32, ret1: u32) -> Flow {
    c.mtc1(0, 20);
    c.g[A0][0] = c.g[self_reg][0];
    c.g[A1][0] = 0;
    c.mov_s(12, 20);
    c.mov_s(13, 20);
    OC838_SET_VIBRATION.call(c, ret0)?;
    c.mov_s(12, 20);
    c.g[A0][0] = c.g[self_reg][0];
    c.g[A1][0] = 1;
    c.mov_s(13, 12);
    OC838_SET_VIBRATION.call(c, ret1)
}

/// `GetHashValue32(a0 + add)` then `UIScreen_FindObject(self->mpScreen, hash)`
/// (screen at `s1 + 0x40`); the object lands in `s0`. True when it exists.
pub(crate) fn oc838_find_widget(
    c: &mut Cpu,
    pre: u64,
    add: u32,
    ret_hash: u32,
    ret_find: u32,
) -> Result<bool, Unwound> {
    c.cycles += pre;
    c.g[A0][0] = add_at(c.g[A0][0], add);
    call_ptr(c, 0x0031_7670, ret_hash)?;
    c.g[A0][0] = lwr!(c, S1, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret_find)?;
    c.g[S0][0] = c.g[V0][0];
    Ok(c.g[S0][0] != 0)
}

/// Add the two entries (`ListBox_AddEntryByStringID(s0, hash(str), 0 / 1)`)
/// whose strings are at `0x460000 - lo1` and `0x460000 - lo2`.
pub(crate) fn oc838_add_two_entries(
    c: &mut Cpu,
    pre: u64,
    lo1: u32,
    lo2: u32,
    rets: [u32; 4],
) -> Flow {
    c.cycles += pre;
    c.g[A0][0] = 0x46_0000;
    c.g[A0][0] = add_at(c.g[A0][0], 0u32.wrapping_sub(lo1));
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0039_a568, rets[1])?;
    c.g[A0][0] = 0x46_0000;
    c.g[A0][0] = add_at(c.g[A0][0], 0u32.wrapping_sub(lo2));
    call_ptr(c, 0x0031_7670, rets[2])?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = 1;
    call_ptr(c, 0x0039_a568, rets[3])
}

/// A widget's virtual `setVisible`-style call: `vt = *(obj + 8)`, call
/// `vt[+0x4c]` with `this = obj + *(short*)(vt + 0x48)` and `a1` as set.
pub(crate) fn oc838_widget_vcall(c: &mut Cpu, ret: u32) -> Flow {
    c.g[V0][0] = lwr!(c, A2, 8);
    c.g[A0][0] = lhr!(c, V0, 0x48);
    c.g[V1][0] = lwr!(c, V0, 0x4c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = add_at(c.g[A2][0], c.g[A0][0] as u32);
    call_ptr(c, target, ret)
}

/// `void cFEStateOptionsHUD_RefreshSelection(cFEStateOptionsHUD* self)`
/// (0x18c478): for the three rows "1x", "2x", "3x" of the HUD's "Menu"
/// widget, store the row index in the item, then highlight the item whose
/// index equals the current option mode (the top two bits of the first word
/// of the option block at 0x535610; that block is copied to the stack by an
/// inlined struct copy) and select it in the menu; the others get the
/// normal (0) state. The menu's "busy" bit (`+0x14 & 1`) is cleared at the end.
pub fn oc838_options_hud_refresh_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x300);
        frame.save(c, S0, 0x2f0);
        frame.save(c, S3, 0x2c0);
        frame.save(c, S1, 0x2e0);
        frame.save(c, S2, 0x2d0);
        frame.save(c, S4, 0x2b0);
        frame.save_ra(c, 0x2a0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_c49c)?;
        c.g[A1][0] = 4;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0018_c4a8)?;
        c.g[V1][0] = 0x4a_0000;
        c.g[A0][0] = 0x4a_0000;
        c.g[A3][0] = add_at(c.g[V1][0], 0x14f8);
        let name = c.g[A3][0] as u32;
        c.g[V0][0] = c.lb(name);
        c.g[A1][0] = c.lb(name.wrapping_add(1));
        let s = frame.at(c, 0);
        c.sb(s, c.g[V0][0]);
        c.sb(s.wrapping_add(1), c.g[A1][0]);
        c.g[A0][0] = add_at(c.g[A0][0], 0x1398);
        call_ptr(c, 0x0031_7670, 0x0018_c4cc)?;
        c.g[A0][0] = lwr!(c, S0, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_c4d8)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[S2][0] = 0;
        if c.g[S3][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = add_at(c.g[S2][0], 0x31);
            loop {
                c.cycles += 14;
                c.g[A0][0] = c.g[SP][0];
                let s = frame.at(c, 0);
                c.sb(s, c.g[V0][0]);
                c.g[S4][0] = add_at(c.g[S2][0], 1);
                c.g[S1][0] = lwr!(c, S3, 8);
                c.g[S0][0] = lhr!(c, S1, 0x70);
                c.g[S0][0] = add_at(c.g[S3][0], c.g[S0][0] as u32);
                call_ptr(c, 0x0031_7670, 0x0018_c504)?;
                c.g[V1][0] = lwr!(c, S1, 0x74);
                c.g[A0][0] = c.g[S0][0];
                let target = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, target, 0x0018_c514)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[V0][0] = 0x53_0000;
                if c.g[A2][0] == 0 {
                    c.cycles += 1;
                    c.g[S2][0] = c.g[S4][0];
                } else {
                    c.cycles += 5;
                    let item = c.g[A2][0] as u32;
                    c.sw(item.wrapping_add(0x18), c.g[S2][0]);
                    c.g[A0][0] = add_at(c.g[V0][0], 0x5610);
                    c.g[V0][0] = c.g[A0][0] & 7;
                    c.g[V1][0] = add_at(c.g[SP][0], 0x10);
                    if c.g[V0][0] != 0 {
                        // Unaligned copy (never taken: the source is constant).
                        c.cycles += 1;
                        c.g[V0][0] = add_at(c.g[A0][0], 0x280);
                        loop {
                            c.cycles += 19;
                            let a0 = c.g[A0][0] as u32;
                            let v1 = c.g[V1][0] as u32;
                            c.ldl(8, a0.wrapping_add(7));
                            c.ldr(8, a0);
                            c.ldl(9, a0.wrapping_add(0xf));
                            c.ldr(9, a0.wrapping_add(8));
                            c.ldl(5, a0.wrapping_add(0x17));
                            c.ldr(5, a0.wrapping_add(0x10));
                            c.ldl(7, a0.wrapping_add(0x1f));
                            c.ldr(7, a0.wrapping_add(0x18));
                            c.sdl(8, v1.wrapping_add(7));
                            c.sdr(8, v1);
                            c.sdl(9, v1.wrapping_add(0xf));
                            c.sdr(9, v1.wrapping_add(8));
                            c.sdl(5, v1.wrapping_add(0x17));
                            c.sdr(5, v1.wrapping_add(0x10));
                            c.sdl(7, v1.wrapping_add(0x1f));
                            c.sdr(7, v1.wrapping_add(0x18));
                            c.g[A0][0] = add_at(c.g[A0][0], 0x20);
                            let more = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = add_at(c.g[V1][0], 0x20);
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018_c538);
                                }
                                continue;
                            }
                            c.cycles += 2;
                            break;
                        }
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = add_at(c.g[A0][0], 0x280);
                        loop {
                            c.cycles += 11;
                            let a0 = c.g[A0][0] as u32;
                            let v1 = c.g[V1][0] as u32;
                            c.g[T0][0] = c.ld(a0);
                            c.g[T1][0] = c.ld(a0.wrapping_add(8));
                            c.g[A1][0] = c.ld(a0.wrapping_add(0x10));
                            c.g[A3][0] = c.ld(a0.wrapping_add(0x18));
                            c.sd(v1, c.g[T0][0]);
                            c.sd(v1.wrapping_add(8), c.g[T1][0]);
                            c.sd(v1.wrapping_add(0x10), c.g[A1][0]);
                            c.sd(v1.wrapping_add(0x18), c.g[A3][0]);
                            c.g[A0][0] = add_at(c.g[A0][0], 0x20);
                            let more = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = add_at(c.g[V1][0], 0x20);
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018_c590);
                                }
                                continue;
                            }
                            break;
                        }
                    }
                    // The last eight bytes of the option block.
                    c.cycles += 8;
                    let a0 = c.g[A0][0] as u32;
                    let v1 = c.g[V1][0] as u32;
                    c.ldl(8, a0.wrapping_add(7));
                    c.ldr(8, a0);
                    c.sdl(8, v1.wrapping_add(7));
                    c.sdr(8, v1);
                    let copy = frame.at(c, 0x10);
                    c.g[V0][0] = c.lw(copy);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 30);
                    let other = c.g[S2][0] != c.g[V0][0];
                    c.g[V0][0] = lwr!(c, A2, 8);
                    if other {
                        c.cycles += 5;
                        c.g[A1][0] = 0;
                        c.g[A0][0] = lhr!(c, V0, 0x48);
                        c.g[V1][0] = lwr!(c, V0, 0x4c);
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = add_at(c.g[A2][0], c.g[A0][0] as u32);
                        call_ptr(c, target, 0x0018_c618)?;
                        c.cycles += 1;
                        c.g[S2][0] = c.g[S4][0];
                    } else {
                        c.cycles += 10;
                        c.g[A1][0] = 1;
                        c.g[A0][0] = lhr!(c, V0, 0x48);
                        c.g[V1][0] = lwr!(c, V0, 0x4c);
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = add_at(c.g[A2][0], c.g[A0][0] as u32);
                        call_ptr(c, target, 0x0018_c5f0)?;
                        c.g[A1][0] = c.g[S2][0] & 0xff;
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x0039_b960, 0x0018_c5fc)?;
                        c.g[S2][0] = c.g[S4][0];
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 3) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = add_at(c.g[S2][0], 0x31);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_c4e8);
                    }
                    continue;
                }
                c.cycles += 4;
                c.g[V0][0] = lwr!(c, S3, 0x14);
                c.g[V1][0] = 0xffff_ffff_ffff_fffe;
                c.g[V0][0] &= c.g[V1][0];
                let flags = (c.g[S3][0] as u32).wrapping_add(0x14);
                c.sw(flags, c.g[V0][0]);
                break;
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x2f0);
        frame.restore(c, S1, 0x2e0);
        frame.restore(c, S2, 0x2d0);
        frame.restore(c, S3, 0x2c0);
        frame.restore(c, S4, 0x2b0);
        frame.restore_ra(c, 0x2a0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOptionsController_Init(self)` (0x18c6a0): create the
/// "22control" screen, clear the two vibration timers, fill the four list
/// boxes ("1lb"/"2lb": Off/On, "3lb"/"4lb": Default/Pro), switch both pads'
/// vibration off and refresh the list boxes from the current settings.
pub fn oc838_options_controller_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x30);
        c.g[A0][0] = 0x46_0000;
        frame.save(c, S0, 0x20);
        c.g[A0][0] = add_at(c.g[A0][0], 0u32.wrapping_sub(0x1f98));
        c.g[S0][0] = lwr!(c, S1, 0x10);
        call_ptr(c, 0x0031_7670, 0x0018_c6c8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_c6dc)?;
        let created = c.g[V0][0] != 0;
        let this = (c.g[S1][0] as u32).wrapping_add(0x40);
        c.sw(this, c.g[V0][0]);
        if created {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_c6f4)?;
        }
        // "1lb"
        c.cycles += 11;
        let this = c.g[S1][0] as u32;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[A0][0] = 0x4a_0000;
        c.sw(this.wrapping_add(0x48), 0);
        if oc838_find_widget(c, 0, 0x1568, 0x0018_c708, 0x0018_c714)? {
            oc838_add_two_entries(
                c,
                15,
                0x20c0,
                0x1fd8,
                [0x0018_c72c, 0x0018_c73c, 0x0018_c748, 0x0018_c758],
            )?;
        }
        c.g[A0][0] = 0x4a_0000;
        // "2lb"
        if oc838_find_widget(c, 8, 0x1570, 0x0018_c764, 0x0018_c770)? {
            oc838_add_two_entries(
                c,
                15,
                0x20c0,
                0x1fd8,
                [0x0018_c788, 0x0018_c798, 0x0018_c7a4, 0x0018_c7b4],
            )?;
        }
        c.g[A0][0] = 0x4a_0000;
        // "3lb"
        if oc838_find_widget(c, 8, 0x15c8, 0x0018_c7c0, 0x0018_c7cc)? {
            oc838_add_two_entries(
                c,
                15,
                0x1f88,
                0x1f70,
                [0x0018_c7e4, 0x0018_c7f4, 0x0018_c800, 0x0018_c810],
            )?;
        }
        c.g[A0][0] = 0x4a_0000;
        // "4lb"
        if oc838_find_widget(c, 8, 0x1578, 0x0018_c81c, 0x0018_c828)? {
            oc838_add_two_entries(
                c,
                14,
                0x1f88,
                0x1f70,
                [0x0018_c840, 0x0018_c850, 0x0018_c85c, 0x0018_c86c],
            )?;
        }
        c.cycles += 19;
        oc838_vibrate_both(c, S1, 0x0018_c884, 0x0018_c898)?;
        c.g[A0][0] = c.g[S1][0];
        OC838_REFRESH_LIST_BOXES.call(c, 0x0018_c8a0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOptionsController_OnChildEvent(self, child, int event)`
/// (0x18c910): event 0x15 switches both pads' vibration off; event 0x16 on
/// the "resetopts" child with a non-null `getData(2)` resets the controller
/// mapping to the defaults, refreshes the list boxes and flags the change
/// (`gp-0x1c14 = 1`).
pub fn oc838_options_controller_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x50);
        call_ptr(c, 0x0014_dc80, 0x0018_c93c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0018_c948)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[V0][0] = 0x15;
        let is_15 = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = 0x16;
        'done: {
            if is_15 {
                // Event 0x15: both pads' vibration off.
                c.cycles += 13;
                oc838_vibrate_both(c, S2, 0x0018_c980, 0x0018_c994)?;
                frame.restore(c, S0, 0x40);
                break 'done;
            }
            c.cycles += 2;
            let is_16 = c.g[S0][0] == c.g[V0][0];
            frame.restore(c, S0, 0x40);
            if !is_16 {
                c.cycles += 2;
                frame.restore(c, S1, 0x30);
                c.cycles += 6;
                frame.restore(c, S2, 0x20);
                frame.restore(c, S3, 0x10);
                frame.restore_ra(c, 0);
                frame.restore_fpr(c, 20, 0x50);
                frame.close(c);
                return Ok(());
            }
            // Event 0x16.
            c.cycles += 6;
            c.g[A0][0] = 0x46_0000;
            c.g[S0][0] = lwr!(c, S1, 0xc);
            c.g[A0][0] = add_at(c.g[A0][0], 0u32.wrapping_sub(0x22e0));
            call_ptr(c, 0x0031_7670, 0x0018_c9ac)?;
            let differs = c.g[S0][0] != c.g[V0][0];
            frame.restore(c, S0, 0x40);
            if differs {
                break 'done;
            }
            c.cycles += 8;
            c.g[V1][0] = lwr!(c, S1, 8);
            c.g[A1][0] = 2;
            c.g[A0][0] = lhr!(c, V1, 0xb8);
            c.g[V0][0] = lwr!(c, V1, 0xbc);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = add_at(c.g[S1][0], c.g[A0][0] as u32);
            call_ptr(c, target, 0x0018_c9cc)?;
            let empty = c.g[V0][0] == 0;
            frame.restore(c, S0, 0x40);
            if empty {
                break 'done;
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0014_5bd8, 0x0018_c9dc)?;
            c.g[A0][0] = c.g[S2][0];
            OC838_REFRESH_LIST_BOXES.call(c, 0x0018_c9e4)?;
            c.g[V0][0] = 1;
            let flag = (c.g[GP][0] as u32).wrapping_sub(0x1c14);
            c.sw(flag, c.g[V0][0]);
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 1;
        frame.restore(c, S1, 0x30);
        c.cycles += 6;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateOptionsController_OnWidgetNotify(self, menu, key)`
/// (0x18cce0): key 6 returns 0, key 9 returns 0x100, other keys 0x101; key 5
/// on the menu row 2 or 3 also shows the config text for pad 0 or 1 (default
/// text when that pad's controller mapping is 0).
pub fn oc838_options_controller_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_cd04)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0018_cd10)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 6;
        let key = c.g[S0][0];
        let is_6 = key == c.g[V0][0];
        c.g[V0][0] = (key < 7) as u64;
        // The shared exits of the generated body: `b_18cd98` (+1 cycle,
        // restore s0) and the epilogue behind it (+5 cycles).
        let restore_s0 = 'decide: {
            if is_6 {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'decide true;
            }
            c.cycles += 2;
            let high = c.g[V0][0] == 0;
            c.g[V0][0] = 5;
            if high {
                // key >= 7: 9 returns 0x100, the rest 0x101.
                c.cycles += 3;
                c.g[V0][0] = 9;
                let not_9 = c.g[S0][0] != c.g[V0][0];
                c.g[V0][0] = 0x101;
                if !not_9 {
                    c.cycles += 2;
                    c.g[V0][0] = 0x100;
                }
                break 'decide true;
            }
            c.cycles += 2;
            let is_5 = c.g[S0][0] == c.g[V0][0];
            c.g[V0][0] = 0x101;
            if !is_5 {
                c.cycles += 2;
                break 'decide false;
            }
            // Key 5: which pad, from the menu's current row.
            c.cycles += 4;
            c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x95));
            c.g[V0][0] = 2;
            let row2 = c.g[A1][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if row2 {
                c.cycles += 2;
                c.g[A1][0] = 0;
            } else {
                c.cycles += 2;
                let row3 = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 0x101;
                if !row3 {
                    c.cycles += 2;
                    break 'decide false;
                }
                c.cycles += 1;
                c.g[A1][0] = 1;
            }
            c.cycles += 7;
            call_ptr(c, 0x0014_7618, 0x0018_cd80)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = (c.g[V0][0] < 1) as u64;
            OC838_SHOW_CONFIG_TEXT.call(c, 0x0018_cd8c)?;
            c.g[V0][0] = 0x101;
            true
        };
        if restore_s0 {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOptionsController_RefreshListBoxes(self)` (0x18cdc0): set
/// each of the four list boxes' current entry from the player interface:
/// "1lb"/"2lb" the dual-shock flag of pad 0/1, "3lb"/"4lb" the controller
/// mapping of pad 0/1 (and the config text for pad 0's mapping).
pub fn oc838_options_controller_refresh_list_boxes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_cddc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0018_cde8)?;
        c.g[A0][0] = 0x4a_0000;
        c.g[S2][0] = c.g[V0][0];
        // "1lb"
        if oc838_find_widget(c, 0, 0x1568, 0x0018_cdf8, 0x0018_ce04)? {
            c.cycles += 11;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_74e8, 0x0018_ce1c)?;
            oc838_set_current_entry(c, 0, 0x0018_ce2c, 0x0018_ce38)?;
        }
        c.g[A0][0] = 0x4a_0000;
        // "2lb"
        if oc838_find_widget(c, 8, 0x1570, 0x0018_ce44, 0x0018_ce50)? {
            c.cycles += 11;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0014_74e8, 0x0018_ce68)?;
            oc838_set_current_entry(c, 1, 0x0018_ce78, 0x0018_ce84)?;
        }
        c.g[A0][0] = 0x4a_0000;
        // "3lb"
        if oc838_find_widget(c, 8, 0x15c8, 0x0018_ce90, 0x0018_ce9c)? {
            c.cycles += 17;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_7618, 0x0018_ceb4)?;
            oc838_set_current_entry(c, 2, 0x0018_cec4, 0x0018_ced0)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_7618, 0x0018_cedc)?;
            c.g[A1][0] = (c.g[V0][0] < 1) as u64;
            c.g[A0][0] = c.g[S1][0];
            OC838_SHOW_CONFIG_TEXT.call(c, 0x0018_cee8)?;
        }
        c.g[A0][0] = 0x4a_0000;
        // "4lb"
        if oc838_find_widget(c, 8, 0x1578, 0x0018_cef4, 0x0018_cf00)? {
            c.cycles += 11;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0014_7618, 0x0018_cf18)?;
            oc838_set_current_entry(c, 3, 0x0018_cf28, 0x0018_cf34)?;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `ListBox_GetEntryByData(s0, a1 = v0, &vals[i])` then
/// `ListBox_SetCurrentEntryByIndex(s0, vals[i])`; `vals` is the four bytes at
/// the stack pointer.
pub(crate) fn oc838_set_current_entry(c: &mut Cpu, index: u32, ret_get: u32, ret_set: u32) -> Flow {
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[SP][0] | index as u64;
    call_ptr(c, 0x0039_a768, ret_get)?;
    let slot = (c.g[SP][0] as u32).wrapping_add(index);
    c.g[A1][0] = c.lbu(slot);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0039_a7a8, ret_set)
}

/// `void cFEStateOptionsController_ShowConfigText(self, int on)` (0x18cf50):
/// show the "default" picture and text when `on`, the "pro" ones when not
/// (the four widgets are looked up by hash in the screen at `self + 0x40`;
/// the first two names come from `gp-0x1b34` and `gp-0x1b30`).
pub fn oc838_options_controller_show_config_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b34));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        oc838_set_visible(c, false, [0x0018_cf70, 0x0018_cf7c, 0x0018_cf9c])?;
        c.cycles += 8;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b30));
        oc838_set_visible(c, true, [0x0018_cfa4, 0x0018_cfb0, 0x0018_cfd0])?;
        c.cycles += 9;
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = add_at(c.g[A0][0], 0u32.wrapping_sub(0x1f60));
        oc838_set_visible(c, false, [0x0018_cfdc, 0x0018_cfe8, 0x0018_d008])?;
        c.cycles += 9;
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = add_at(c.g[A0][0], 0u32.wrapping_sub(0x1f50));
        oc838_set_visible(c, true, [0x0018_d014, 0x0018_d020, 0x0018_d040])?;
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One widget of `ShowConfigText`: hash the name in `a0`, find the object in
/// the screen at `s0 + 0x40`, and when it exists call its virtual
/// `setVisible(on ^ invert)` (`a1` is `s1`, or `s1 ^ 1`, in the delay slot
/// of the null test either way).
pub(crate) fn oc838_set_visible(c: &mut Cpu, invert: bool, rets: [u32; 3]) -> Flow {
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A0][0] = lwr!(c, S0, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, rets[1])?;
    c.g[A2][0] = c.g[V0][0];
    let missing = c.g[A2][0] == 0;
    c.g[A1][0] = c.g[S1][0] ^ invert as u64;
    if !missing {
        c.cycles += 5;
        oc838_widget_vcall(c, rets[2])?;
    }
    Ok(())
}

/// `StrHash(const char*)`.
pub(crate) const OC839_STR_HASH: u32 = 0x0031_7670;

/// `UIScreen_FindObject(screen, hash)`.
pub(crate) const OC839_FIND_OBJECT: u32 = 0x0039_d860;

/// `cUIText_setUnicodeString(text, str)`.
pub(crate) const OC839_SET_UNICODE: u32 = 0x003a_0e90;

/// `cUIText_setAsciiString(text, str)`.
pub(crate) const OC839_SET_ASCII: u32 = 0x003a_0c50;

/// `sprintf(buf, fmt, ...)`.
pub(crate) const OC839_SPRINTF: u32 = 0x0041_7828;

/// `StrHash(name)`, returning to `ret`.
pub(crate) fn oc839_hash(c: &mut Cpu, name: u32, ret: u32) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, OC839_STR_HASH, ret)
}

/// `UIScreen_FindObject(*(this+0x40), v0)`, returning to `ret`.
pub(crate) fn oc839_find(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, OC839_FIND_OBJECT, ret)
}

/// `StrHash(name)` then the find; the object is left in `v0`.
pub(crate) fn oc839_lookup(c: &mut Cpu, name: u32, ret_hash: u32, ret_find: u32) -> Flow {
    oc839_hash(c, name, ret_hash)?;
    oc839_find(c, ret_find)
}

/// Call the method at `*(*(s0+8) + off + 4)` on `s0` (`this` adjusted by the
/// halfword at `off`), `a1` already set.
pub(crate) fn oc839_vcall_s0(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(off));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(off + 4));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// The same call on the object in `v0` (the method pointer goes through
/// `a2`), `a1 = arg`.
pub(crate) fn oc839_vcall_v0(c: &mut Cpu, arg: u64, ret: u32) -> Flow {
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[A1][0] = arg;
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
    let t = c.g[A2][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// The world's string-table lookup: `v0` = world; `s0 = world->+0x8c`,
/// `s1 = s0->+4`, `s0 += s1->+0x20`, `v0 = StrHash(name)`, then
/// `s1->+0x24(s0, v0)`; the result ends up in `v0`.
pub(crate) fn oc839_world_text(c: &mut Cpu, name: u32, ret_hash: u32, ret_call: u32) -> Flow {
    c.g[A0][0] = sx(name);
    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    call_ptr(c, OC839_STR_HASH, ret_hash)?;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
    c.g[A1][0] = c.g[V0][0];
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, t, ret_call)
}

/// The shared tail of the type-1 and save branches: call the `+0x48` method
/// of the object in `s0` (`a1` set by the branch), `a0 = 0x460000`.
pub(crate) fn oc839_seven(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    oc839_vcall_s0(c, 0x48, 0x0018_dad4)?;
    c.g[A0][0] = 0x46_0000;
    Ok(())
}

/// `cFEStateOptionsDeviceSelect_Init(self)` (0x18d868): builds the memory
/// card device-select screen ("120MCselect"): hides some texts, sets the
/// title (load replay / save / load options), then fills the four memory
/// card slot texts ("%dmemory_card", "%d_b") with the "empty" string.
pub fn c_fe_state_options_device_select_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S2, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        c.g[A0][0] = 0x46_0000;
        frame.save(c, S4, 0x50);
        c.g[A0][0] = sx(0x46_0000u32.wrapping_sub(0x1ec0));
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save_ra(c, 0x20);
        frame.save(c, S0, 0x90);
        c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        call_ptr(c, OC839_STR_HASH, 0x0018_d8a0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_d8b4)?;
        let none = c.g[V0][0] == 0;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if none {
            oc839_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 36;
        c.g[A2][0] = 0;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0039_c870, 0x0018_d8cc)?;
        c.g[V1][0] = 0x4a_0000;
        c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x15d0));
        let name = c.g[S4][0] as u32;
        oc839_hash(c, name, 0x0018_d8dc)?;
        oc839_find(c, 0x0018_d8e8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        oc839_vcall_s0(c, 0x48, 0x0018_d904)?;
        c.g[V0][0] = 0x4a_0000;
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x15d8));
        let name = c.g[S3][0] as u32;
        oc839_hash(c, name, 0x0018_d914)?;
        oc839_find(c, 0x0018_d920)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        oc839_vcall_s0(c, 0x48, 0x0018_d93c)?;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
        c.g[V0][0] = 1;
        if c.g[A0][0] != c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x22c));
            c.cycles += 2;
            let load = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if !load {
                // Save screen.
                c.cycles += 32;
                oc839_world_text(c, 0x46_0000 - 0x1e88, 0x0018_da5c, 0x0018_da6c)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, OC839_STR_HASH, 0x0018_da78)?;
                oc839_find(c, 0x0018_da84)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                oc839_vcall_s0(c, 0x48, 0x0018_daa0)?;
                oc839_lookup(c, 0x46_0000 - 0x1e70, 0x0018_daac, 0x0018_dab8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                oc839_seven(c)?;
            } else {
                // Load screen.
                c.cycles += 63;
                oc839_world_text(c, 0x46_0000 - 0x1e60, 0x0018_daf8, 0x0018_db08)?;
                c.g[S1][0] = c.g[V0][0];
                let name = c.g[S3][0] as u32;
                oc839_hash(c, name, 0x0018_db14)?;
                oc839_find(c, 0x0018_db20)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                oc839_vcall_s0(c, 0x48, 0x0018_db3c)?;
                oc839_lookup(c, 0x46_0000 - 0x1e48, 0x0018_db48, 0x0018_db54)?;
                c.g[S0][0] = c.g[V0][0];
                oc839_hash(c, 0x4a_1460, 0x0018_db64)?;
                oc839_find(c, 0x0018_db70)?;
                oc839_vcall_v0(c, 0, 0x0018_db88)?;
                c.g[A1][0] = 1;
                oc839_vcall_s0(c, 0x48, 0x0018_dba0)?;
                oc839_lookup(c, 0x4a_1458, 0x0018_dbac, 0x0018_dbb8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                oc839_vcall_s0(c, 0x48, 0x0018_dbd4)?;
                c.g[A0][0] = 0x46_0000;
            }
        } else {
            // Replay-load screen.
            c.cycles += 59;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            oc839_world_text(c, 0x46_0000 - 0x1eb0, 0x0018_d96c, 0x0018_d97c)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, OC839_STR_HASH, 0x0018_d988)?;
            oc839_find(c, 0x0018_d994)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            oc839_vcall_s0(c, 0x48, 0x0018_d9b0)?;
            oc839_lookup(c, 0x46_0000 - 0x1e98, 0x0018_d9bc, 0x0018_d9c8)?;
            c.g[S0][0] = c.g[V0][0];
            oc839_hash(c, 0x4a_1460, 0x0018_d9d8)?;
            oc839_find(c, 0x0018_d9e4)?;
            oc839_vcall_v0(c, 0, 0x0018_d9fc)?;
            c.g[A1][0] = 1;
            oc839_vcall_s0(c, 0x48, 0x0018_da14)?;
            oc839_lookup(c, 0x4a_1458, 0x0018_da20, 0x0018_da2c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            oc839_seven(c)?;
        }
        // Title and slot texts.
        c.cycles += 46;
        c.g[S3][0] = 1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1e38));
        c.g[S6][0] = 0x46_0000;
        call_ptr(c, OC839_STR_HASH, 0x0018_dbe8)?;
        c.g[S5][0] = 0x4c_0000;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        c.g[S4][0] = 0x4a_0000;
        call_ptr(c, OC839_FIND_OBJECT, 0x0018_dbfc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, OC839_SET_UNICODE, 0x0018_dc08)?;
        oc839_lookup(c, 0x46_0000 - 0x1e28, 0x0018_dc14, 0x0018_dc20)?;
        c.g[S0][0] = 0x4a_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1408));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, OC839_SET_ASCII, 0x0018_dc34)?;
        oc839_lookup(c, 0x46_0000 - 0x1e18, 0x0018_dc40, 0x0018_dc4c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, OC839_SET_ASCII, 0x0018_dc58)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        oc839_world_text(c, 0x46_0000 - 0x1e00, 0x0018_dc78, 0x0018_dc88)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[SP][0];
        loop {
            c.cycles += 33;
            c.g[A2][0] = c.g[S3][0];
            c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1df0));
            call_ptr(c, OC839_SPRINTF, 0x0018_dc9c)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, OC839_STR_HASH, 0x0018_dca4)?;
            oc839_find(c, 0x0018_dcb0)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            call_ptr(c, OC839_SET_UNICODE, 0x0018_dcc8)?;
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x66c8));
            c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.ldr(2, c.g[A1][0] as u32);
            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
            c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
            c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x8f));
            c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x88));
            c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x97));
            c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x90));
            c.g[A1][0] = 1;
            oc839_vcall_s0(c, 0xc8, 0x0018_dd04)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
            c.g[V0][0] = 1;
            let replay = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = ((c.g[S3][0] as i64) < 2) as u64;
            if replay {
                c.cycles += 1;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 31;
                    c.g[A1][0] = 1;
                    oc839_vcall_s0(c, 0x40, 0x0018_dd34)?;
                    c.g[A1][0] = 0;
                    oc839_vcall_s0(c, 0x48, 0x0018_dd4c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x12c0));
                    call_ptr(c, OC839_SPRINTF, 0x0018_dd5c)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, OC839_STR_HASH, 0x0018_dd64)?;
                    oc839_find(c, 0x0018_dd70)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    oc839_vcall_s0(c, 0x48, 0x0018_dd8c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, OC839_SET_ASCII, 0x0018_dd98)?;
                    c.cycles += 1;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 5) as u64;
            let again = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[SP][0];
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0018_dc90);
            }
        }
        oc839_epilogue(c, frame);
        Ok(())
    })
}

pub(crate) fn oc839_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 10;
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
}

/// `void cFEStateOptionsController_SetVibration(self, int pad, float x,
/// float y)` (0x18d108): sets the two rumble motors of pad `pad` from the
/// options sliders. `x` (f12) and `y` (f13) are rescaled and clamped; if the
/// pad's device reports two actuators (`+0x18/+0x1c` vtable slot) the large
/// (`setActuator(0, ..)`) and small (`setActuator(1, ..)`) motors are set
/// from the larger of two linear ramps (never below 0). f20-f22 are saved
/// and restored like the original.
pub fn c_fe_state_options_controller_set_vibration(c: &mut Cpu) {
    guest(c, |c| {
        'b_18d154: {
            c.cycles += 18;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x59e8));
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x59e4));
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
            c.mul_s(12, 12, 0);
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x28));
            c.mtc1(0, 21);
            c.g[V0][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sub_s(12, 12, 1);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.mov_s(20, 13);
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.c_le_s(21, 12);
            let t = !c.fcc();
            c.mov_s(22, 21);
            if t {
                break 'b_18d154;
            } else {
                c.cycles += 1;
                c.mov_s(22, 12);
                break 'b_18d154;
            }
        }
        'b_18d174: {
            c.cycles += 5;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x59e0));
            c.g[S0][0] = 0u64;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x850));
            let t = c.g[V1][0] == 0u64;
            c.mul_s(20, 20, 0);
            if t {
                break 'b_18d174;
            } else {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2eec));
                break 'b_18d174;
            }
        }
        'b_18d228: {
            c.cycles += 8;
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018_d188)?;
            c.g[V1][0] = 0x2;
            if c.g[V0][0] != c.g[V1][0] {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_18d228;
            } else {
                c.cycles += 37;
                c.g[AT][0] = 0x4000_0000;
                c.mtc1(1, 0);
                c.g[A1][0] = 0u64;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x59dc));
                c.sub_s(0, 22, 0);
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 12);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x59d8));
                c.sub_s(12, 20, 12);
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.div_s(0, 0, 2);
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                c.mul_s(12, 12, 1);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.max_s(12, 12, 0);
                let t = c.g[V1][0] as u32;
                c.max_s(12, 12, 21);
                call_ptr(c, t, 0x0018_d1dc)?;
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 0);
                c.g[A1][0] = 0x1;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x59d4));
                c.sub_s(0, 22, 0);
                c.g[AT][0] = 0x428c_0000;
                c.mtc1(1, 12);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x59d0));
                c.sub_s(12, 20, 12);
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.div_s(0, 0, 2);
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                c.mul_s(12, 12, 1);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.max_s(12, 12, 0);
                let t = c.g[V1][0] as u32;
                c.max_s(12, 12, 21);
                call_ptr(c, t, 0x0018_d224)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_18d228;
            }
        }
        c.cycles += 6;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateOptionsSaveLoad_OnWidgetCreated(self, widget)` (0x18d2e8):
/// tags a freshly created menu widget (`+0x38` name hash) with its menu-item
/// code in `+0x18` (`1saveoptions` 0, `2loadoptions` 2, `3loadreplay` 1,
/// `4new_game` 3, two more ids 0x300 and 0x200); other widgets are left alone.
pub fn c_fe_state_options_save_load_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_18d3ac: {
            'b_18d3a8: {
                'b_18d31c: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[A0][0] = 0x460000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f30));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x0031_7670, 0x0018_d30c)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        break 'b_18d31c;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                        if t {
                            break 'b_18d3a8;
                        } else {
                            break 'b_18d31c;
                        }
                    }
                }
                'b_18d3a4: {
                    'b_18d338: {
                        c.cycles += 5;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f20));
                        call_ptr(c, 0x0031_7670, 0x0018_d328)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_18d338;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x2;
                            if t {
                                break 'b_18d3a4;
                            } else {
                                break 'b_18d338;
                            }
                        }
                    }
                    'b_18d354: {
                        c.cycles += 5;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f10));
                        call_ptr(c, 0x0031_7670, 0x0018_d344)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_18d354;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_18d3a4;
                            } else {
                                break 'b_18d354;
                            }
                        }
                    }
                    'b_18d370: {
                        c.cycles += 5;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f00));
                        call_ptr(c, 0x0031_7670, 0x0018_d360)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x4a0000;
                            break 'b_18d370;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x3;
                            if t {
                                break 'b_18d3a4;
                            } else {
                                break 'b_18d370;
                            }
                        }
                    }
                    'b_18d38c: {
                        c.cycles += 5;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1508));
                        call_ptr(c, 0x0031_7670, 0x0018_d37c)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x4a0000;
                            break 'b_18d38c;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x300;
                            if t {
                                break 'b_18d3a4;
                            } else {
                                break 'b_18d38c;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1500));
                    call_ptr(c, 0x0031_7670, 0x0018_d398)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_18d3ac;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0x200;
                        break 'b_18d3a4;
                    }
                }
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_18d3a8;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_18d3ac;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateOptionsSaveLoad_OnChildEvent(self, widget, event)`
/// (0x18d460): event 0xf / 0x10 store 1 / 0 in `widget+0x18` and pop the
/// state; event 0x16 (with `widget+0x18 != 0`) starts a new profile: names
/// player `mUserNum` (`+0x44`, clamped to 0) empty, resets both front-end
/// riders, asks the custom interface (`engine+0` vtable `+0x20/+0x24`) for the
/// next state (`1 << self+0x48 | 0x100`), pushes it and plays the
/// `TransitionOut` frame of the screen (`self+0x40`). Other events do
/// nothing.
pub fn c_fe_state_options_save_load_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_18d638: {
            'b_18d634: {
                'b_18d630: {
                    'b_18d4e8: {
                        'b_18d4d4: {
                            'b_18d4cc: {
                                c.cycles += 9;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                                c.g[V1][0] = c.g[A1][0];
                                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = 0x10;
                                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[S2][0] = c.g[A0][0];
                                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                let t = c.g[A2][0] == c.g[V0][0];
                                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                                if t {
                                    break 'b_18d4cc;
                                } else {
                                    'b_18d4bc: {
                                        'b_18d4a0: {
                                            c.cycles += 3;
                                            c.g[V0][0] = (c.g[A2][0] < 0x11u64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0xf;
                                            if t {
                                                break 'b_18d4a0;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_18d4bc;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    if t {
                                                        break 'b_18d638;
                                                    } else {
                                                        break 'b_18d4a0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x15;
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x16;
                                        if t {
                                            break 'b_18d630;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A2][0] == c.g[V0][0];
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_18d4e8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_18d638;
                                                } else {
                                                    break 'b_18d4bc;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = 0x1;
                                    c.g[A1][0] = 0x1;
                                    let t = true;
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                    if t {
                                        break 'b_18d4d4;
                                    } else {
                                        break 'b_18d4cc;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x18), 0u64);
                            c.g[A1][0] = 0x1;
                            break 'b_18d4d4;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039_f190, 0x0018_d4e0)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_18d634;
                        } else {
                            break 'b_18d4e8;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_18d630;
                    } else {
                        'b_18d5fc: {
                            c.cycles += 63;
                            c.g[S1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                c.g[S1][0] = 0u64;
                            }
                            call_ptr(c, 0x0014_dc80, 0x0018_d504)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x0014_dd58, 0x0018_d510)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A2][0] = 0x4a0000;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1408));
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0014_7138, 0x0018_d528)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018_d53c)?;
                            call_ptr(c, 0x0014_dc80, 0x0018_d544)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x0014_de28, 0x0018_d550)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                            call_ptr(c, 0x001a_0548, 0x0018_d564)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0019_eba0, 0x0018_d570)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = 0x4;
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x0019_e960, 0x0018_d584)?;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                            call_ptr(c, 0x001a_0548, 0x0018_d598)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0019_eba0, 0x0018_d5a4)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = 0u64;
                            c.g[A1][0] = 0x1;
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x0019_e960, 0x0018_d5b8)?;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[A2][0] = 0x1;
                            c.g[V0][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x48));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A3][0] = c.lw(c.g[V1][0] as u32);
                            c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[V0][0] as u32) & 31));
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A2][0] = c.g[A2][0] | 0x100;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018_d5e8)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_18d5fc;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039_f400, 0x0018_d5fc)?;
                                break 'b_18d5fc;
                            }
                        }
                        c.cycles += 10;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x23a0));
                        call_ptr(c, 0x0031_7670, 0x0018_d608)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x0018_d614)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let t = c.g[A1][0] == c.g[V0][0];
                        c.g[A1][0] = c.g[A1][0] & 0xffff;
                        if t {
                            break 'b_18d630;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x0039_c870, 0x0018_d630)?;
                            break 'b_18d630;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_18d634;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_18d638;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateOptionsSaveLoad_OnWidgetAction(self, widget, event)`
/// (0x18d648): event 5 (select) switches on the widget's menu-item code
/// (`widget+0x18`, set to 3 on the way): the "new game" item (3) opens the
/// create-new-file confirm popup, others push the next state the custom
/// interface (`engine+0` vtable `+0x20/+0x24`) returns for `item | 1 <<
/// self+0x14`; event 6 (back) asks the previous state (vtable `+0x28/+0x2c`)
/// for the widget's item and pushes it. A null widget does nothing.
pub fn c_fe_state_options_save_load_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_18d7e0: {
            'b_18d7dc: {
                'b_18d7d8: {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[V1][0] = c.g[A1][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[V1][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    if t {
                        break 'b_18d7d8;
                    } else {
                        'b_18d7a0: {
                            'b_18d684: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x5;
                                let t = c.g[A2][0] == c.g[V0][0];
                                c.g[V0][0] = 0x6;
                                if t {
                                    break 'b_18d684;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                    if t {
                                        break 'b_18d7a0;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_18d7e0;
                                        } else {
                                            break 'b_18d684;
                                        }
                                    }
                                }
                            }
                            'b_18d748: {
                                c.cycles += 4;
                                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                c.g[V0][0] = 0x3;
                                let t = c.g[A2][0] != c.g[V0][0];
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                if t {
                                    break 'b_18d748;
                                } else {
                                    c.cycles += 45;
                                    c.g[V1][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x14));
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1ef0));
                                    c.sb((c.g[S2][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    call_ptr(c, 0x0031_7670, 0x0018_d6bc)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x0018_d6cc)?;
                                    c.g[A1][0] = 0x460000;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1ed0));
                                    c.g[A0][0] = 0xbfc;
                                    c.g[A2][0] = 0x100;
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x0031_7d70, 0x0018_d6e8)?;
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A3][0] = c.g[S0][0];
                                    c.g[T0][0] = 0u64;
                                    c.g[T1][0] = 0u64;
                                    c.g[T2][0] = 0u64;
                                    c.g[T3][0] = 0u64;
                                    c.sw(c.g[SP][0] as u32, 0u64);
                                    c.g[A2][0] = c.g[S2][0];
                                    call_ptr(c, 0x001d_99c8, 0x0018_d710)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018_d72c)?;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039_f290, 0x0018_d740)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                    if t {
                                        break 'b_18d7dc;
                                    } else {
                                        break 'b_18d748;
                                    }
                                }
                            }
                            'b_18d768: {
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[A2][0] < 0x4u64) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    break 'b_18d768;
                                } else {
                                    c.cycles += 5;
                                    c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x14));
                                    c.g[V0][0] = 0x1;
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32) << ((c.g[V1][0] as u32) & 31));
                                    c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    break 'b_18d768;
                                }
                            }
                            c.cycles += 9;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018_d784)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_18d7d8;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039_f400, 0x0018_d798)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_18d7dc;
                                } else {
                                    break 'b_18d7a0;
                                }
                            }
                        }
                        c.cycles += 11;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0018_d7c4)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.g[V0][0];
                        if t {
                            break 'b_18d7d8;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039_f400, 0x0018_d7d8)?;
                            break 'b_18d7d8;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_18d7dc;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_18d7e0;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateOptionsDeviceSelect_OnWidgetAction(self, widget, event)`
/// (0x18dea0): the memory-card device screen's button handler. Event 5
/// (select) and event 7 (X button) pick the widget's slot (`widget+0x18`
/// into `self+0x1c4` and the overlay manager `+0xf8`) and start a save/load
/// or open a status popup depending on `self+0x1bc/+0x1c0/+0x1a8`; event 6
/// (back/cancel) closes the screen (`self+0x1c0 = 6`).
pub fn c_fe_state_options_device_select_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_18e154: {
            'b_18e150: {
                'b_18e094: {
                    'b_18e028: {
                        c.cycles += 16;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[S0][0] = c.g[A0][0];
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S1][0] = c.g[A2][0];
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[S3][0] = c.g[A1][0];
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[S4][0] = 0x6;
                        call_ptr(c, 0x0022_7f80, 0x0018_ded8)?;
                        let t = c.g[S1][0] == c.g[S4][0];
                        c.g[S2][0] = c.g[V0][0];
                        if t {
                            break 'b_18e028;
                        } else {
                            'b_18df10: {
                                'b_18defc: {
                                    c.cycles += 3;
                                    c.g[V0][0] = (c.g[S1][0] < 0x7u64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x5;
                                    if t {
                                        break 'b_18defc;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == c.g[V0][0];

                                        if t {
                                            break 'b_18df10;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            if t {
                                                break 'b_18e154;
                                            } else {
                                                break 'b_18defc;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x7;
                                if c.g[S1][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                                    break 'b_18e094;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                    if t {
                                        break 'b_18e154;
                                    } else {
                                        break 'b_18df10;
                                    }
                                }
                            }
                            c.cycles += 2;
                            if c.g[S3][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                break 'b_18e154;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                                if c.g[V0][0] != 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                    break 'b_18e154;
                                } else {
                                    c.cycles += 5;
                                    c.g[S1][0] = 0x1;
                                    c.g[S4][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x214));
                                    c.g[V0][0] = 0x3;
                                    let t = c.g[S4][0] != c.g[V0][0];
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x238), c.g[S1][0]);
                                    if t {
                                        break 'b_18e150;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x224));
                                        if c.g[V0][0] == 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            break 'b_18e154;
                                        } else {
                                            'b_18e018: {
                                                'b_18e010: {
                                                    'b_18e00c: {
                                                        'b_18df8c: {
                                                            c.cycles += 7;
                                                            c.g[V0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x18));
                                                            c.g[A0][0] = 0x2;
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x1c4),
                                                                c.g[V0][0],
                                                            );
                                                            c.sw(
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0xf8),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1bc));
                                                            let t = c.g[V1][0] != c.g[A0][0];
                                                            c.g[V0][0] = 0x1;
                                                            if t {
                                                                break 'b_18df8c;
                                                            } else {
                                                                'b_18df88: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x22c));
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    if t {
                                                                        break 'b_18df88;
                                                                    } else {
                                                                        c.cycles += 7;
                                                                        c.g[A1][0] = 0x2;
                                                                        c.g[A2][0] =
                                                                            0xffffffffffffffff;
                                                                        call_ptr(
                                                                            c,
                                                                            0x0023_cae8,
                                                                            0x0018_df78,
                                                                        )?;
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1a8,
                                                                                ),
                                                                            c.g[S1][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1c0,
                                                                                ),
                                                                            c.g[S4][0],
                                                                        );
                                                                        let t = true;
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1dc,
                                                                                ),
                                                                            c.g[S1][0],
                                                                        );
                                                                        if t {
                                                                            break 'b_18e00c;
                                                                        } else {
                                                                            break 'b_18df88;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V0][0] = 0x1;
                                                                break 'b_18df8c;
                                                            }
                                                        }
                                                        'b_18dfd0: {
                                                            c.cycles += 2;
                                                            let t = c.g[V1][0] != c.g[V0][0];
                                                            c.g[A2][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x434));
                                                            if t {
                                                                break 'b_18dfd0;
                                                            } else {
                                                                c.cycles += 8;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x1c4));
                                                                c.g[V1][0] =
                                                                    c.lw(c.g[A2][0] as u32);
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x218));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x21c));
                                                                let t = c.g[V0][0] as u32;
                                                                c.g[A0][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                call_ptr(c, t, 0x0018_dfac)?;
                                                                if c.g[V0][0] == 0u64 {
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_18e010;
                                                                } else {
                                                                    c.cycles += 7;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    c.g[A1][0] = 0u64;
                                                                    call_ptr(
                                                                        c,
                                                                        0x0023_c8f0,
                                                                        0x0018_dfc0,
                                                                    )?;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    c.g[A0][0] = c
                                                                        .lh((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x160));
                                                                    let t = true;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x164));
                                                                    if t {
                                                                        break 'b_18e018;
                                                                    } else {
                                                                        break 'b_18dfd0;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 8;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1c4));
                                                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x218));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x21c));
                                                        let t = c.g[V0][0] as u32;
                                                        c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        call_ptr(c, t, 0x0018_dfe8)?;
                                                        if c.g[V0][0] == 0u64 {
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            break 'b_18e010;
                                                        } else {
                                                            c.cycles += 7;
                                                            c.g[A0][0] = c.g[S2][0];
                                                            c.g[A1][0] = 0x2;
                                                            call_ptr(c, 0x0023_c8f0, 0x0018_dffc)?;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x160));
                                                            let t = true;
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x164));
                                                            if t {
                                                                break 'b_18e018;
                                                            } else {
                                                                break 'b_18e00c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    break 'b_18e010;
                                                }
                                                c.cycles += 2;
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x178));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x17c));
                                                break 'b_18e018;
                                            }
                                            c.cycles += 4;
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0018_e020)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            if t {
                                                break 'b_18e154;
                                            } else {
                                                break 'b_18e028;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x238), 0u64);
                    call_ptr(c, 0x0022_7f80, 0x0018_e034)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                    if c.g[V0][0] == c.g[A0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        break 'b_18e154;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                        if c.g[V1][0] == c.g[S4][0] {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            break 'b_18e154;
                        } else {
                            'b_18e08c: {
                                'b_18e074: {
                                    'b_18e070: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == 0u64;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1e4), c.g[A0][0]);
                                        if t {
                                            break 'b_18e070;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x2;
                                            if c.g[V1][0] == c.g[V0][0] {
                                                c.g[V0][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x748));
                                                break 'b_18e074;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != c.g[A0][0];
                                                c.g[V0][0] = 0x6;
                                                if t {
                                                    break 'b_18e08c;
                                                } else {
                                                    break 'b_18e070;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x748));
                                    break 'b_18e074;
                                }
                                c.cycles += 6;
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0018_e088)?;
                                c.g[V0][0] = 0x6;
                                break 'b_18e08c;
                            }
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                            if t {
                                break 'b_18e150;
                            } else {
                                break 'b_18e094;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[S5][0] = 0x1;
                if c.g[V0][0] == c.g[S5][0] {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    break 'b_18e154;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                    if c.g[V0][0] == c.g[S4][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        break 'b_18e154;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x22c));
                        if c.g[V0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            break 'b_18e154;
                        } else {
                            c.cycles += 12;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            call_ptr(c, 0x0022_7f80, 0x0018_e0c0)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018_e0e0)?;
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                break 'b_18e154;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c4), c.g[V0][0]);
                                c.sw((c.g[S1][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
                                c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                                if c.g[S2][0] != c.g[S5][0] {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                    break 'b_18e154;
                                } else {
                                    c.cycles += 9;
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c4));
                                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018_e11c)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x4;
                                    if t {
                                        break 'b_18e150;
                                    } else {
                                        c.cycles += 11;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[S2][0]);
                                        c.g[A0][0] = c.g[S1][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                                        c.g[A1][0] = 0u64;
                                        call_ptr(c, 0x0023_d570, 0x0018_e138)?;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x748));
                                        c.g[A1][0] = 0x18;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0018_e150)?;
                                        break 'b_18e150;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_18e154;
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateOptionsDeviceSelect_v39(self, int a1, int a2)` (0x18e368):
/// switches the device screen's highlight. Looks up a UI object of the
/// screen (`self+0x40`) by the hash of `0x4a15e0`; with `self+0x22c != 0` it
/// calls vtable `+0x48/+0x4c` on the `0x4a15d0` object with `a2 ^ 1`,
/// otherwise on the `0x4a15d8` object with `a1 ^ 1`; the value is stored in
/// `self+0x224` and also sent to the first object.
pub fn c_fe_state_options_device_select_v39(c: &mut Cpu) {
    guest(c, |c| {
        'b_18e45c: {
            'b_18e408: {
                c.cycles += 19;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S1][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = 0x4a0000;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S3][0] = c.g[A1][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = c.g[A2][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15e0));
                call_ptr(c, 0x0031_7670, 0x0018_e398)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0018_e3a4)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x22c));
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = c.g[S0][0] ^ 0x1;
                if t {
                    break 'b_18e408;
                } else {
                    c.cycles += 21;
                    c.g[A0][0] = 0x4a0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15d0));
                    call_ptr(c, 0x0031_7670, 0x0018_e3c0)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x0018_e3cc)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[A2][0] as u32;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018_e3e4)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x224), c.g[S0][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018_e400)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_18e45c;
                    } else {
                        break 'b_18e408;
                    }
                }
            }
            c.cycles += 21;
            c.g[A0][0] = 0x4a0000;
            c.g[S0][0] = c.g[S3][0] ^ 0x1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15d8));
            call_ptr(c, 0x0031_7670, 0x0018_e418)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_e424)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            let t = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018_e43c)?;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x224), c.g[S0][0]);
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018_e458)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_18e45c;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// Virtual call `obj->vtable[slot]()` the way gcc 2.9x emits it: `a0 =
/// lh(vt + doff)` (the `this` delta), `fr = lw(vt + foff)` (the method), `ra`,
/// `a0 = obj + a0`, `jalr`. `obj` and `vt` are register numbers.
#[allow(clippy::too_many_arguments)]
pub(crate) fn oc841_vcall(
    c: &mut Cpu,
    obj: usize,
    vt: usize,
    doff: u32,
    fr: usize,
    foff: u32,
    ret: u32,
) -> Flow {
    c.g[A0][0] = c.lh((c.g[vt][0] as u32).wrapping_add(doff));
    c.g[fr][0] = c.lw((c.g[vt][0] as u32).wrapping_add(foff));
    let t = c.g[fr][0] as u32;
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = sx((c.g[obj][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// `0x460000 - off`: the address of a string literal (`lui` + `addiu`).
pub(crate) fn oc841_str(off: u32) -> u64 {
    sx(0x0046_0000u32.wrapping_sub(off))
}

/// Shared tail of the per-slot loop: call the text's method (`v1`, `this =
/// s0 + a0`) and count the slot (`s4 += 1`).
pub(crate) fn oc841_slot_tail(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    let t = c.g[V1][0] as u32;
    c.g[RA][0] = sx(0x0018_e86c);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x0018_e86c)?;
    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
    Ok(())
}

/// The slot is neither the save target nor a blank one: grey it out or leave it
/// live depending on the device (the `DEVI 54` / `66` branches).
pub(crate) fn oc841_slot_device_flags(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
    oc841_vcall(c, A2, V1, 0x1b0, V0, 0x1b4, 0x0018_e7f4)?;
    let t = c.g[V0][0] == 0;
    c.g[A1][0] = c.g[S3][0];
    if t {
        c.cycles += 13;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A1][0] = 1;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0018_e88c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0018_e88c)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A1][0] = 1;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0018_e8a4);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0018_e8a4)?;
        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
        return Ok(());
    }
    'b_85c: {
        c.cycles += 8;
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
        oc841_vcall(c, A2, V1, 0x210, V0, 0x214, 0x0018_e814)?;
        let t = c.g[V0][0] == 0;
        c.g[A1][0] = 1;
        if !t {
            c.cycles += 8;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0018_e830);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018_e830)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
            c.g[A1][0] = 1;
            break 'b_85c;
        }
        c.cycles += 8;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A1][0] = 0;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0018_e854);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0018_e854)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A1][0] = 0;
    }
    c.cycles += 2;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
    oc841_slot_tail(c)
}

/// One slot's text object (`s0`) is set up: the device texts, then static /
/// static-colour flags. Ends with `s4 += 1` on every path.
pub(crate) fn oc841_slot_flags(c: &mut Cpu) -> Flow {
    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
    c.g[A1][0] = c.g[S3][0];
    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
    oc841_vcall(c, A2, V1, 0x218, V0, 0x21c, 0x0018_e6e8)?;
    let mut to_static = c.g[V0][0] != 0;
    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
    if !to_static {
        c.cycles += 8;
        c.g[A1][0] = c.g[S3][0];
        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
        oc841_vcall(c, A2, V1, 0x210, V0, 0x214, 0x0018_e708)?;
        if c.g[V0][0] != 0 {
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
            to_static = true;
        } else {
            c.cycles += 8;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            oc841_vcall(c, A1, V1, 0x1b8, V0, 0x1bc, 0x0018_e728)?;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                to_static = true;
            }
        }
    }
    if to_static {
        return oc841_slot_device_flags(c);
    }
    // Blank slot of a writable card: type 1 (all slots) greys it, otherwise
    // only the first one stays live.
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
    let t = c.g[V0][0] != c.g[S7][0];
    c.g[A1][0] = 1;
    if !t {
        c.cycles += 10;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0018_e750);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0018_e750)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A1][0] = 1;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        return oc841_slot_tail(c);
    }
    c.cycles += 2;
    if c.g[FP][0] != 0 {
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x22c));
        let t = c.g[V0][0] == 0;
        c.g[A1][0] = 0;
        if !t {
            c.cycles += 14;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
            c.g[FP][0] = 1;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0018_e790);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018_e790)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
            c.g[A1][0] = 0;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xcc));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0018_e7a8);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018_e7a8)?;
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
            return Ok(());
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
    }
    c.cycles += 10;
    c.g[A1][0] = 1;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
    let t = c.g[V1][0] as u32;
    c.g[RA][0] = sx(0x0018_e7c8);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x0018_e7c8)?;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
    c.g[A1][0] = 1;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
    oc841_slot_tail(c)
}

/// An arrow text: `FindObject(screen, StrHash(name)).setVisible(vis)`
/// (virtual `+0x48`); `rets` are the three return addresses.
pub(crate) fn oc841_arrow(c: &mut Cpu, name_off: u32, rets: [u32; 3], vis: u64) -> Flow {
    c.g[A0][0] = oc841_str(name_off);
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, rets[1])?;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(8));
    c.g[A1][0] = vis;
    oc841_vcall(c, V0, V1, 0x48, A2, 0x4c, rets[2])
}

/// `cFEStateOptionsDeviceSelect::displayStatus(self)` (0x18e510): the memory
/// card device picker. Keeps the screen state in step with the card (no cards:
/// the no-cards callback; otherwise state 3), then fills the device name text,
/// the four "memory card N" slot texts (live or greyed per slot) and the two
/// arrows; finally greys the slots again unless the screen state is 3.
pub fn oc841_device_select_display_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x770);
        frame.save(c, S2, 0x740);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x750);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S0, 0x760);
        frame.save(c, S3, 0x730);
        frame.save(c, S4, 0x720);
        frame.save(c, S5, 0x710);
        frame.save(c, S6, 0x700);
        frame.save(c, S7, 0x6f0);
        frame.save(c, FP, 0x6e0);
        frame.save_ra(c, 0x6d0);
        call_ptr(c, 0x0022_7f80, 0x0018_e54c)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(8));
        c.g[S1][0] = c.g[V0][0];
        oc841_vcall(c, S2, V1, 0x150, V0, 0x154, 0x0018_e564)?;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(8));
        } else {
            c.cycles += 8;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            oc841_vcall(c, A1, V1, 0x1a8, V0, 0x1ac, 0x0018_e584)?;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x214));
                c.cycles += 3;
                c.g[V0][0] = 3;
                let same = c.g[V1][0] == c.g[V0][0];
                c.g[A1][0] = 3;
                if !same {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(8));
                    oc841_vcall(c, S2, V0, 0x118, V1, 0x11c, 0x0018_e5bc)?;
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(8));
            } else {
                c.cycles += 4;
                call_ptr(c, 0x001d_8240, 0x0018_e594)?;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(8));
            }
        }
        // Second displayFileOK: the real work.
        c.cycles += 6;
        oc841_vcall(c, S2, V1, 0x150, V0, 0x154, 0x0018_e5d0)?;
        let skip = c.g[V0][0] == 0;
        c.g[FP][0] = 0;
        if skip {
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x214));
        } else {
            c.cycles += 40;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(8));
            c.g[S4][0] = 1;
            c.g[S7][0] = 1;
            c.g[S3][0] = 0;
            oc841_vcall(c, S2, V0, 0x158, V1, 0x15c, 0x0018_e5f8)?;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = 0x320;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0024_1710, 0x0018_e60c)?;
            c.g[A0][0] = oc841_str(0x1e18);
            call_ptr(c, 0x0031_7670, 0x0018_e618)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_e624)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0d00, 0x0018_e630)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
            c.g[A0][0] = oc841_str(0x1e28);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x21c), c.g[V0][0]);
            call_ptr(c, 0x0031_7670, 0x0018_e644)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_e650)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
            oc841_vcall(c, S0, V0, 0x48, V1, 0x4c, 0x0018_e66c)?;
            c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x690));
            c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
            c.g[V0][0] = 0x0046_0000;
            // The four "memory card N" slot texts.
            'l_18e678: loop {
                c.cycles += 30;
                c.g[A2][0] = c.g[S4][0];
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1df0));
                c.g[A0][0] = c.g[S6][0];
                call_ptr(c, 0x0041_7828, 0x0018_e688)?;
                c.g[A0][0] = c.g[S6][0];
                call_ptr(c, 0x0031_7670, 0x0018_e690)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0018_e69c)?;
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                c.g[S0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S5][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                oc841_vcall(c, A3, V0, 0x70, V1, 0x74, 0x0018_e6c0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x003a_0e90, 0x0018_e6cc)?;
                oc841_slot_flags(c)?;
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S4][0] as i64) < 5) as u64;
                let done = c.g[V0][0] == 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                if !done {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
                    let again = c.g[V0][0] == c.g[S7][0];
                    c.g[V0][0] = 0x0046_0000;
                    if again {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018_e678);
                        }
                        continue 'l_18e678;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S4][0] as i64) < 2) as u64;
                    let again = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x0046_0000;
                    if again {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018_e678);
                        }
                        continue 'l_18e678;
                    }
                }
                break;
            }
            // Left arrow: shown when a next device exists.
            c.cycles += 8;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 1;
            call_ptr(c, 0x001d_97e0, 0x0018_e8e0)?;
            c.g[V1][0] = u64::MAX;
            let found = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 0x0046_0000;
            if found {
                c.cycles += 12;
                oc841_arrow(c, 0x1de0, [0x0018_e928, 0x0018_e934, 0x0018_e94c], 1)?;
            } else {
                c.cycles += 13;
                oc841_arrow(c, 0x1de0, [0x0018_e8f4, 0x0018_e900, 0x0018_e918], 0)?;
            }
            c.g[A1][0] = c.g[S1][0];
            // Right arrow.
            c.cycles += 7;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
            c.g[A3][0] = 0;
            call_ptr(c, 0x001d_97e0, 0x0018_e960)?;
            c.g[V1][0] = u64::MAX;
            let found = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 0x0046_0000;
            if found {
                c.cycles += 11;
                oc841_arrow(c, 0x1dd0, [0x0018_e9a8, 0x0018_e9b4, 0x0018_e9cc], 1)?;
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x214));
            } else {
                c.cycles += 13;
                oc841_arrow(c, 0x1dd0, [0x0018_e974, 0x0018_e980, 0x0018_e998], 0)?;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x214));
            }
        }
        // Not in state 3: grey every slot text again.
        c.cycles += 3;
        c.g[V0][0] = 3;
        let same = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = 1;
        if same {
            c.cycles += 1;
            frame.restore(c, S0, 0x760);
        } else {
            c.cycles += 3;
            c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
            c.g[S4][0] = 1;
            c.g[V0][0] = 0x0046_0000;
            loop {
                c.cycles += 26;
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1df0));
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0041_7828, 0x0018_e9f8)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0031_7670, 0x0018_ea04)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0018_ea10)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                oc841_vcall(c, S0, V0, 0xc8, V1, 0xcc, 0x0018_ea2c)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                c.g[A1][0] = 1;
                oc841_vcall(c, S0, V0, 0x40, V1, 0x44, 0x0018_ea44)?;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 5) as u64;
                if c.g[V0][0] == 0 {
                    frame.restore(c, S0, 0x760);
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
                let again = c.g[V0][0] == c.g[S4][0];
                c.g[V0][0] = 0x0046_0000;
                if again {
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_e9e8);
                    }
                    continue;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 2) as u64;
                let again = c.g[V0][0] != 0;
                c.g[V0][0] = 0x0046_0000;
                if again {
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_e9e8);
                    }
                    continue;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x760);
                break;
            }
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x750);
        frame.restore(c, S2, 0x740);
        frame.restore(c, S3, 0x730);
        frame.restore(c, S4, 0x720);
        frame.restore(c, S5, 0x710);
        frame.restore(c, S6, 0x700);
        frame.restore(c, S7, 0x6f0);
        frame.restore(c, FP, 0x6e0);
        frame.restore_ra(c, 0x6d0);
        frame.close(c);
        Ok(())
    })
}

/// `UIStateMgr_SwitchStateClosePopups` (0x39f4c0).
pub(crate) const OC842_SWITCH_CLOSE_POPUPS: u32 = 0x0039_f4c0;

/// `int cFEStateOptionsDeviceSelect_v33(this)` (0x18e478): once per screen
/// (guard word at `+0x1ac`), ask the owner's engine for the next state through
/// its vtable (slot `+0x28/+0x2c`, with a flag argument in a2: set when the
/// word at `+0x1bc` is 0, `+0x1e4` is non-zero and `+0x1b4` is non-zero, or
/// forced to 2 when `+0x230` is non-zero), and if it returns one, switch to it
/// with the owner's state manager (`owner + 0x18`). Returns 1; the guard is
/// stored as 1.
pub fn cfe_state_options_device_select_v33_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(off842(c.g[S0][0], 0x1ac));
        let done = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !done {
            c.cycles += 21;
            c.g[V1][0] = c.lw(off842(c.g[S0][0], 0x10));
            c.g[T2][0] = 2;
            c.g[A2][0] = c.lw(off842(c.g[S0][0], 0x1bc));
            c.g[A1][0] = c.g[S0][0];
            c.g[T1][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = c.lw(off842(c.g[S0][0], 0x1e4));
            c.g[A2][0] ^= 1;
            c.g[T0][0] = c.lw(off842(c.g[T1][0], 4));
            c.g[A2][0] = (c.g[A2][0] < 1) as u64;
            c.g[V0][0] ^= 1;
            c.g[V1][0] = c.lw(off842(c.g[S0][0], 0x1b4));
            if c.g[V0][0] == 0 {
                c.g[A2][0] = 0;
            }
            c.g[A3][0] = c.lw(off842(c.g[S0][0], 0x230));
            c.g[A0][0] = c.lh(off842(c.g[T0][0], 0x28));
            if c.g[V1][0] == 0 {
                c.g[A2][0] = 0;
            }
            c.g[V0][0] = c.lw(off842(c.g[T0][0], 0x2c));
            if c.g[A3][0] != 0 {
                c.g[A2][0] = c.g[T2][0];
            }
            let method = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, 0x0018_e4e0)?;
            let none = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[V0][0];
            if !none {
                c.cycles += 3;
                c.g[A0][0] = c.lw(off842(c.g[S0][0], 0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, OC842_SWITCH_CLOSE_POPUPS, 0x0018_e4f4)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        let guard = c.g[V0][0];
        c.sw(off842(c.g[S0][0], 0x1ac), guard);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `FEScreenSelectProfile::FEScreenSelectProfile(this)` (0x18ea98): the
/// `UIState` base constructor, then `this->+0xc = 8` and the screen's vtable
/// at `+8`. Returns `this`.
pub fn fe_screen_select_profile_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x0018_eaac)?;
        c.g[V0][0] = 8;
        c.g[V1][0] = 0x0047_0000;
        c.sw(off842(c.g[S0][0], 0xc), c.g[V0][0]);
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x3ea8));
        c.sw(off842(c.g[S0][0], 0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateOptionsGame_OnDeactivate` (0x001d_dfa8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_options_game_on_deactivate_001ddfa8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateOptionsSound_OnDeactivate` (0x001d_e010): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_options_sound_on_deactivate_001de010(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateOptionsHUD_OnDeactivate` (0x001d_e078): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_options_hud_on_deactivate_001de078(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateOptionsController_OnDeactivate` (0x001d_e0e8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_options_controller_on_deactivate_001de0e8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::bx::bxstringctor::tests::base_3;
    pub(crate) use crate::game::render::particle::tests::gp_at;

    pub(crate) use crate::game::camera::cameracontroller::tests::pool as pool_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2 as be_library;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::world_2 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_5;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui_init_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_5;

    pub(crate) use crate::game::fe::festatebonusmaterial::tests::str_hash;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup as setup_2;

    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2 as belib;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_2;
    use ssx3_ee as ee;

    // -----------------------------------------------------------------------
    // cFEState constructors.

    pub(crate) fn fe_state_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        for i in 0..0x120u32 {
            c.bus.write32(DATA + i * 4, r.next_u32());
        }
        // A zeroed class descriptor (the base ctor installs a real one): the
        // refresh helpers then read a null vtable slot and reach the fake.
        c.bus.write32(DATA + 0x8, 0);
        c.bus.write32(
            DATA + 0x214,
            match seed % 3 {
                0 => 3,
                1 => r.next_u32(),
                _ => 2,
            },
        );
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
        c.bus.write32(DATA + 0x1000 + 0x84, DATA + 0x2000);
        c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x3000);
    }

    #[test]
    fn c_fe_state_options_game_ctor_matches() {
        check(
            ssx3_ee::s02::f_00188ee8__generated,
            c_fe_state_options_game_ctor,
            24,
            &|c, seed| {
                fe_state_setup(c, seed, 0x188e_e8);
            },
        );
    }

    // --------------------------------------------------------- state ctors

    #[test]
    fn c_fe_state_options_sound_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018a258__generated,
            c_fe_state_options_sound_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x13, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
            },
        );
    }

    #[test]
    fn c_fe_state_options_hud_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018c0e8__generated,
            c_fe_state_options_hud_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x14, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
            },
        );
    }

    #[test]
    fn c_fe_state_options_save_load_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018d240__generated,
            c_fe_state_options_save_load_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x15, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
            },
        );
    }

    #[test]
    fn fe_state_options_controller_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018c658__generated,
            fe_state_options_controller_ctor,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x18c658, 0);
                be_library(c);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn cfe_state_options_controller_shutdown_matches() {
        check(
            ssx3_ee::s02::f_0018c8b8__generated,
            cfe_state_options_controller_shutdown,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x18c8b8, 3);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.bus.write32(DATA + 0x400, seed % 4);
            },
        );
    }

    /// Scratch objects, tables and hit records.
    pub(crate) const AUX: u32 = DATA + 0x1000;

    pub(crate) fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 8) ^ salt)
    }

    /// Scrambled registers (so incidental leftovers are compared) and the
    /// game's `gp`.
    pub(crate) fn setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = rng(seed, salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    // ------------------------------------------------------------------
    // Front-end option screens.

    #[test]
    fn cfe_state_options_handle_input_matches() {
        check(
            ee::s02::f_00188c58__generated,
            cfe_state_options_handle_input,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 23);
                c.g[A1][0] = AUX as u64;
                c.bus.write8(AUX + 4, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn cfe_state_options_game_on_widget_notify_matches() {
        check(
            ee::s02::f_00189958__generated,
            cfe_state_options_game_on_widget_notify,
            200,
            &|c, seed| {
                setup(c, seed, 24);
                c.g[A2][0] = [9u64, 0, 6, 1][(seed % 4) as usize];
            },
        );
    }

    #[test]
    fn cfe_state_options_game_v21_matches() {
        check(
            ee::s02::f_00189970__generated,
            cfe_state_options_game_v21,
            200,
            &|c, seed| {
                setup(c, seed, 25);
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { seed as u64 };
            },
        );
    }

    #[test]
    fn cfe_state_options_sound_on_widget_notify_matches() {
        check(
            ee::s02::f_0018a890__generated,
            cfe_state_options_sound_on_widget_notify,
            200,
            &|c, seed| {
                setup(c, seed, 26);
                c.g[A2][0] = [6u64, 9, 0, 3][(seed % 4) as usize];
            },
        );
    }

    #[test]
    fn cfe_state_options_sound_v21_matches() {
        check(
            ee::s02::f_0018a8c0__generated,
            cfe_state_options_sound_v21,
            200,
            &|c, seed| {
                setup(c, seed, 27);
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { seed as u64 };
            },
        );
    }

    #[test]
    fn cfe_state_options_hud_on_widget_notify_matches() {
        check(
            ee::s02::f_0018c270__generated,
            cfe_state_options_hud_on_widget_notify,
            200,
            &|c, seed| {
                setup(c, seed, 28);
                c.g[A2][0] = [6u64, 9, 0, 3][(seed % 4) as usize];
            },
        );
    }

    #[test]
    fn cfe_state_options_controller_v21_matches() {
        check(
            ee::s02::f_0018cdb0__generated,
            cfe_state_options_controller_v21,
            200,
            &|c, seed| {
                setup(c, seed, 29);
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { seed as u64 };
            },
        );
    }

    #[test]
    fn cfe_state_options_save_load_on_widget_notify_matches() {
        check(
            ee::s02::f_0018d438__generated,
            cfe_state_options_save_load_on_widget_notify,
            200,
            &|c, seed| {
                setup(c, seed, 30);
                c.g[A2][0] = [0u64, 7, 8, 9, 10, 0xffff_ffff][(seed % 6) as usize];
            },
        );
    }

    // ---------------------------------------------------------- FE state screens

    #[test]
    fn c_fe_state_options_save_load_init_matches() {
        check(
            ssx3_ee::s02::f_0018d280__generated,
            c_fe_state_options_save_load_init,
            40,
            &|c, seed| {
                let _r = setup_2(c, seed, 0x18d280);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn c_fe_state_options_hud_init_matches() {
        check(
            ee::s02::f_0018c128__generated,
            c_fe_state_options_hud_init,
            4,
            &|c, seed| {
                ui_init_setup(c, seed, 0x0018_c128);
            },
        );
    }

    #[test]
    fn c_fe_state_options_init_matches() {
        check(
            ssx3_ee::s02::f_00188800__generated,
            c_fe_state_options_init,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x188800, 0);
                pool_2(c);
                let s = DATA + 0x1000;
                let engine = DATA + 0x2000;
                c.g[A0][0] = s as u64;
                w32(c, s + 0x10, engine);
                w32(c, engine + 4, 0);
                w32(c, engine + 0x38, 0);
                w32(c, engine + 0x3c, 0);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_fe_state_options_device_select_ctor_matches() {
        check(
            ee::s02::f_0018d7f0__generated,
            c_fe_state_options_device_select_ctor,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x18_d7f0, 3);
                belib(c);
                world(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2200) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = if seed % 2 == 0 {
                    0
                } else {
                    (DATA + 0x2400) as u64
                };
                c.g[T0][0] = (seed % 7) as u64;
                c.pc = 0x0018_d7f0;
            },
        );
    }

    #[test]
    fn c_fe_state_options_save_load_on_render_matches() {
        check(
            ee::s02::f_0018d3c0__generated,
            c_fe_state_options_save_load_on_render,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x18d3c0, 2);
                be_library_2(c);
                let h0 = str_hash(c, 0x0045_e0f0);
                let h1 = str_hash(c, 0x0045_e100);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                w(
                    c,
                    GAME_GP.wrapping_sub(0x1718),
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                let screen = DATA + 0x1000;
                let arr = DATA + 0x1100;
                w(c, obj + 0x40, screen);
                let count = if seed % 3 == 0 { 0 } else { 2 };
                w(c, screen + 0x3c, count);
                w(c, screen + 0x38, arr);
                for i in 0..2u32 {
                    let present = i < count;
                    let p = DATA + 0x1200 + 0x80 * i;
                    w(c, arr + 4 * i, if present { p } else { 0 });
                    if present {
                        let hash = match (seed + i) % 3 {
                            0 => h0,
                            1 => h1,
                            _ => r.next_u32(),
                        };
                        w(c, p + 0x38, hash);
                        w(c, p + 8, 0);
                    }
                }
            },
        );
    }

    #[test]
    fn c_fe_state_options_open_reset_popup_matches() {
        check(
            ee::s02::f_00188e58__generated,
            c_fe_state_options_open_reset_popup,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x188e, 0);
                env(c);
                let this = DATA + 0x400;
                w32(
                    c,
                    this + 0x10,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------------------- device select

    #[test]
    fn cfe_state_options_device_select_v38_matches() {
        check(
            ee::s02::f_0018e2c0__generated,
            cfe_state_options_device_select_v38,
            SEEDS_4,
            &|c, seed| {
                let _ = base_2(c, seed, 0x18e2_c0, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, obj + 0x22c, if seed % 3 == 0 { 0 } else { obj + 0x2000 });
                let screen = obj + 0x3000;
                w(c, obj + 0x40, screen);
                w(c, screen + 0x3c, 1);
                w(c, screen + 0x38, screen + 0x400);
                let entry = screen + 0x500;
                w(c, screen + 0x400, entry);
                let hash = run_2(c, 0x0031_7670, ee::s10::f_00317670, &[0x004a_1458u64]);
                w(c, entry + 0x38, hash as u32);
                w(c, entry + 8, screen + 0x600);
                slot_2(c, screen + 0x600, 0x48, 0);
                slot_2(c, screen + 0x600, 0x4c, 1);
            },
        );
    }

    #[test]
    fn cfe_state_options_controller_update_matches() {
        check(
            ee::s02::f_0018d058__generated,
            cfe_state_options_controller_update,
            80,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x18d0);
                c.g[A0][0] = sx(DATA);
                c.bus.write8(DATA + 0x50, [0, 1, 2, 3][(seed % 4) as usize]);
                c.bus
                    .write32(DATA + 0x48, if seed % 3 == 0 { 0 } else { 2 });
                c.bus
                    .write32(DATA + 0x4c, if seed % 5 == 0 { 0 } else { 1 });
                fill(c, &mut r, DATA + 0x10, 0x40);
            },
        );
    }

    #[test]
    fn fe_state_options_hud_on_child_event_matches() {
        check(
            ee::s02::f_0018c1b8__generated,
            fe_state_options_hud_on_child_event,
            SEEDS_5,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x18c1, 3);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 0x15,
                    1 => 0x16,
                    _ => 0x17,
                };
                w(c, DATA + 0x800 + 0xc, r.next_u32());
                w(c, DATA + 0x800 + 8, DATA + 0x900);
                w(c, DATA + 0x900 + 0xb8, 0);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn cfe_state_options_device_select_on_widget_notify_matches() {
        check(
            ee::s02::f_0018ddd0__generated,
            cfe_state_options_device_select_on_widget_notify,
            80,
            &|c, seed| {
                let _r = base_5(c, seed, 0x18dd);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.bus.write32(DATA + 0x1000, seed);
                c.g[A2][0] = [8u64, 9, 6, 0x101][(seed % 4) as usize];
                // A prepared manager so `MCManager_Get` does not construct one.
                c.bus.write32(gp_at(-0x47c), DATA + 0x2000);
            },
        );
    }

    pub(crate) const RET: u32 = 0x0012_3450;

    /// `int StrHash(const char *s)` run during setup (to build hash-keyed
    /// screens).
    pub(crate) fn str_hash_2(c: &mut Cpu, s: u32) -> u64 {
        c.g[A0][0] = sx(s);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(c);
        assert_eq!(c.pc, RET, "StrHash did not return");
        c.g[V0][0]
    }

    /// A one-object screen at `screen`: `UIScreen_FindObject` returns the
    /// object for `hash`, whose `+8` vtable sends slot `+0x4c` to the fake
    /// callee.
    pub(crate) fn screen_fixture(c: &mut Cpu, screen: u32, hash: u64) {
        let array = screen + 0x100;
        let obj = screen + 0x200;
        let vt = screen + 0x300;
        w(c, screen + 0x3c, 1);
        w(c, screen + 0x38, array);
        w(c, array, obj);
        w(c, obj + 0x38, hash as u32);
        w(c, obj + 8, vt);
        c.bus.write16(vt + 0x48, 0);
        w(c, vt + 0x4c, FAKE);
    }

    /// The shared fixture of the two `_v37` state switches: `this` at DATA,
    /// the `+0x234`/`+0x22c` fields and an optional widget screen.
    pub(crate) fn state_switch_setup(c: &mut Cpu, seed: u32, salt: u64, field: u32, calls: u32) {
        let _ = machine(c, seed, salt, calls);
        belib(c);
        let screen = DATA + 0x40_000;
        w(
            c,
            DATA + 0x234,
            if seed % 2 == 0 {
                seed % 2
            } else {
                (seed + 1) % 2
            },
        );
        w(c, DATA + field, if seed % 3 == 0 { 0 } else { 1 });
        let found = seed % 4 >= 2;
        if found {
            let hash = str_hash_2(c, 0x004a_15e0);
            screen_fixture(c, screen, hash);
            w(c, DATA + 0x40, screen);
        }
        // (Registers last: building the fixture calls guest code.)
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (seed % 2) as u64;
    }

    #[test]
    fn c_fe_state_options_device_select_v37_matches() {
        check(
            ee::s02::f_0018e178__generated,
            c_fe_state_options_device_select_v37,
            40,
            &|c, seed| state_switch_setup(c, seed, 0x18_e178, 0x22c, 8),
        );
    }

    /// `StrHash` of a `D_004A...` id string, by the generated hasher (the
    /// test machine holds the game's `.rodata`).
    pub(crate) fn hash_of(addr: u32) -> u32 {
        let mut c = crate::difftest::fresh();
        c.g[A0][0] = addr as u64;
        c.g[RA][0] = 0x0012_3450;
        c.g[GP][0] = crate::difftest::GAME_GP as u64;
        c.pc = 0x0031_7670;
        (ee::s10::f_00317670)(&mut c);
        c.g[V0][0] as u32
    }

    pub(crate) const ID_VIBRATION_0: u32 = 0x004a_1568;

    pub(crate) const ID_VIBRATION_1: u32 = 0x004a_1570;

    pub(crate) const ID_CONFIG_V0: u32 = 0x004a_15c8;

    pub(crate) const ID_CONFIG_V1: u32 = 0x004a_1578;

    pub(crate) fn fixture(c: &mut Cpu, seed: u32) {
        let _r = crate::game::fe::feasyncfile::tests::base_3(c, seed, 0x18ca10, 0);
        c.pc = 0x0018_ca10;
        let self_ = DATA;
        let widget = DATA + 0x800;
        c.g[A0][0] = self_ as u64;
        c.g[A1][0] = if seed % 6 == 0 { 0 } else { widget as u64 };
        c.g[A2][0] = (seed % 12) as u64;
        c.bus.write32(self_ + 0x10, DATA + 0x1000);
        c.bus
            .write32(self_ + 0x48, if seed % 3 == 0 { 3 } else { 0 });
        c.bus
            .write32(self_ + 0x4c, if seed % 5 == 0 { 4 } else { 0 });
        c.bus.write8(self_ + 0x50, (seed % 4) as u8);
        let id = match seed % 4 {
            0 => ID_VIBRATION_0,
            1 => ID_VIBRATION_1,
            2 => ID_CONFIG_V0,
            _ => ID_CONFIG_V1,
        };
        let h = hash_of(id);
        c.bus
            .write32(widget + 0x38, if seed % 7 == 3 { 0 } else { h });
        c.bus.write32(widget + 0x18, 0);
    }

    /// The vibration timer an option turns on is one second of the VSync
    /// clock: 60 ticks at 60 Hz, 120 at 120, 240 at 240.
    #[test]
    fn widget_action_vibration_timer_follows_the_rate() {
        for hz in [60u32, 120, 240] {
            let mut c = crate::difftest::fresh();
            fixture(&mut c, 9); // event 9, the first vibration id
                                // The option is checked (`WIDGET_CHECKED`: count 0 < 1 and value
                                // `widget+0xa4`), and the player-int record `Vibration_Apply`
                                // reads has its bit 1 set (slot 0 is player 0; the test machine's
                                // id strings hash to 0, so the first vibration id is the branch).
            c.bus.write8(DATA + 0x800 + 0x318, 1);
            c.bus.write8(DATA + 0x800 + 0x319, 0);
            c.bus.write32(DATA + 0x800 + 0xa4, 1);
            c.bus.write32(0x0053_05b0, 0);
            c.bus.write32(0x0053_4fe0 + 0x10, 2);
            crate::tick::set_for_test(&mut c, hz);
            fe_options_controller_on_widget_action(&mut c);
            assert_eq!(c.pc, crate::difftest::RA, "{hz}: did not return");
            let t = c.bus.read32(DATA + 0x48);
            assert_eq!(t, hz, "{hz} Hz: vibration timer");
        }
    }

    #[test]
    fn on_widget_action_matches() {
        check(
            ee::s02::f_0018ca10__generated,
            fe_options_controller_on_widget_action,
            16,
            &|c, seed| {
                fixture(c, seed);
            },
        );
    }

    use crate::difftest::fakes::covered;

    #[test]
    fn cfe_state_options_ctor_2_matches() {
        covered(
            ee::s04::f_001ddee8__generated,
            cfe_state_options_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_dee8, 1);
            },
        );
    }

    #[test]
    fn cfe_state_options_dtor_matches() {
        covered(
            ee::s04::f_001ddf20__generated,
            cfe_state_options_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_df20, 2);
            },
        );
    }

    #[test]
    fn cfe_state_options_game_ctor_2_matches() {
        covered(
            ee::s04::f_001ddf48__generated,
            cfe_state_options_game_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_df48, 3);
            },
        );
    }

    #[test]
    fn cfe_state_options_game_dtor_matches() {
        covered(
            ee::s04::f_001ddf80__generated,
            cfe_state_options_game_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_df80, 4);
            },
        );
    }

    #[test]
    fn cfe_state_options_sound_ctor_2_matches() {
        covered(
            ee::s04::f_001ddfb0__generated,
            cfe_state_options_sound_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_dfb0, 5);
            },
        );
    }

    #[test]
    fn cfe_state_options_sound_dtor_matches() {
        covered(
            ee::s04::f_001ddfe8__generated,
            cfe_state_options_sound_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_dfe8, 6);
            },
        );
    }

    #[test]
    fn cfe_state_options_hud_ctor_2_matches() {
        covered(
            ee::s04::f_001de018__generated,
            cfe_state_options_hud_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e018, 7);
            },
        );
    }

    #[test]
    fn cfe_state_options_hud_dtor_matches() {
        covered(
            ee::s04::f_001de050__generated,
            cfe_state_options_hud_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e050, 8);
            },
        );
    }

    #[test]
    fn cfe_state_options_controller_ctor_2_matches() {
        covered(
            ee::s04::f_001de080__generated,
            cfe_state_options_controller_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e080, 9);
            },
        );
    }

    #[test]
    fn cfe_state_options_controller_dtor_matches() {
        covered(
            ee::s04::f_001de0c0__generated,
            cfe_state_options_controller_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e0c0, 10);
            },
        );
    }

    #[test]
    fn cfe_state_options_save_load_ctor_2_matches() {
        covered(
            ee::s04::f_001de0f0__generated,
            cfe_state_options_save_load_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e0f0, 11);
            },
        );
    }

    #[test]
    fn cfe_state_options_save_load_dtor_matches() {
        covered(
            ee::s04::f_001de128__generated,
            cfe_state_options_save_load_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e128, 12);
            },
        );
    }

    #[test]
    fn cfe_state_options_device_select_dtor_matches() {
        covered(
            ee::s04::f_001de150__generated,
            cfe_state_options_device_select_dtor,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e150, 13);
            },
        );
    }

    #[test]
    fn cfe_state_options_device_select_ctor_2_matches() {
        covered(
            ee::s04::f_001de178__generated,
            cfe_state_options_device_select_ctor_2,
            FE_SEEDS,
            &|c, seed| {
                fe_setup(c, seed, 0x001d_e178, 14);
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::SP;
    use crate::game::fe::feasyncfile::tests::run_8;

    pub(crate) use crate::game::fe::festateprofile::tests::oc842_machine;
    pub(crate) use crate::game::fe::festateprofile::tests::slot842;
    pub(crate) use crate::game::fe::festateprofile::tests::w842;

    pub(crate) use crate::game::fe::festateprofile::tests::OC842_RET_ZERO;
    use crate::game::main::loadscreens::tests::oc736_fake_call;
    use crate::game::main::loadscreens::tests::oc736_force_zero;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate word of the fakes (a machine without it runs the real callees).
    pub(crate) const OC832_CTRL: u32 = DATA + 0x30_0000;

    pub(crate) const OC832_MAGIC: u32 = 0x4f43_8320;

    /// Per-callee forced results: one word per fake at `OC832_FORCE + (addr & 0xffffc)`.
    pub(crate) const OC832_FORCE: u32 = DATA + 0x30_1000;

    pub(crate) const OC832_OBJ: u32 = DATA + 0x8_0000;

    pub(crate) const OC832_WIDGET: u32 = DATA + 0x8_1000;

    pub(crate) const OC832_ENGINE: u32 = DATA + 0x8_2000;

    pub(crate) const OC832_CUSTOM: u32 = DATA + 0x8_3000;

    pub(crate) const OC832_VT: u32 = DATA + 0x8_4000;

    pub(crate) const OC832_KEYBOARD: u32 = DATA + 0x8_5000;

    pub(crate) const OC832_SOUND: u32 = DATA + 0x8_6000;

    pub(crate) const OC832_SOUND_VT: u32 = DATA + 0x8_7000;

    pub(crate) const OC832_WORLD: u32 = DATA + 0x8_8000;

    /// The hash every widget name test compares against.
    pub(crate) const OC832_HASH: u32 = 0x1234;

    pub(crate) fn oc832_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Make the fake for `addr` return `v` (stored +3).
    pub(crate) fn oc832_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus
            .write32(OC832_FORCE + (addr & 0xf_fffc), v.wrapping_add(3));
    }

    /// The fake callee: logs like `fakes::fake_call`, then returns the forced
    /// result of its mailbox (0 = the fake's pointer) unless it unwound.
    pub(crate) fn oc832_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC832_FORCE + (addr & 0xf_fffc));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = if mail == 1 { 0 } else { (mail - 3) as u64 };
        }
    }

    macro_rules! oc832_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC832_CTRL) != OC832_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc832_fake_call(c, $addr);
                }
            )*
            fn oc832_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc832_fakes!(
        (
            oc832_fake_1d8700,
            OC832_PREV_1D8700,
            ee::s04::OVERRIDE_001D8700,
            ee::s04::f_001d8700__generated,
            0x001d_8700
        ),
        (
            oc832_fake_227f80,
            OC832_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80
        ),
        (
            oc832_fake_23fb18,
            OC832_PREV_23FB18,
            ee::s06::OVERRIDE_0023FB18,
            ee::s06::f_0023fb18__generated,
            0x0023_fb18
        ),
        (
            oc832_fake_23fae0,
            OC832_PREV_23FAE0,
            ee::s06::OVERRIDE_0023FAE0,
            ee::s06::f_0023fae0__generated,
            0x0023_fae0
        ),
        (
            oc832_fake_1877b0,
            OC832_PREV_1877B0,
            ee::s02::OVERRIDE_001877B0,
            ee::s02::f_001877b0__generated,
            0x0018_77b0
        ),
        (
            oc832_fake_187318,
            OC832_PREV_187318,
            ee::s02::OVERRIDE_00187318,
            ee::s02::f_00187318__generated,
            0x0018_7318
        ),
        (
            oc832_fake_187270,
            OC832_PREV_187270,
            ee::s02::OVERRIDE_00187270,
            ee::s02::f_00187270__generated,
            0x0018_7270
        ),
        (
            oc832_fake_23d618,
            OC832_PREV_23D618,
            ee::s06::OVERRIDE_0023D618,
            ee::s06::f_0023d618__generated,
            0x0023_d618
        ),
        (
            oc832_fake_14dc80,
            OC832_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc832_fake_14dd58,
            OC832_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc832_fake_152948,
            OC832_PREV_152948,
            ee::s01::OVERRIDE_00152948,
            ee::s01::f_00152948__generated,
            0x0015_2948
        ),
        (
            oc832_fake_23caa0,
            OC832_PREV_23CAA0,
            ee::s06::OVERRIDE_0023CAA0,
            ee::s06::f_0023caa0__generated,
            0x0023_caa0
        ),
        (
            oc832_fake_23c8f0,
            OC832_PREV_23C8F0,
            ee::s06::OVERRIDE_0023C8F0,
            ee::s06::f_0023c8f0__generated,
            0x0023_c8f0
        ),
        (
            oc832_fake_317670,
            OC832_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc832_fake_39d860,
            OC832_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc832_fake_397140,
            OC832_PREV_397140,
            ee::s13::OVERRIDE_00397140,
            ee::s13::f_00397140__generated,
            0x0039_7140
        ),
        (
            oc832_fake_39fcc8,
            OC832_PREV_39FCC8,
            ee::s13::OVERRIDE_0039FCC8,
            ee::s13::f_0039fcc8__generated,
            0x0039_fcc8
        ),
        (
            oc832_fake_1d9308,
            OC832_PREV_1D9308,
            ee::s04::OVERRIDE_001D9308,
            ee::s04::f_001d9308__generated,
            0x001d_9308
        ),
        (
            oc832_fake_39e318,
            OC832_PREV_39E318,
            ee::s13::OVERRIDE_0039E318,
            ee::s13::f_0039e318__generated,
            0x0039_e318
        ),
        (
            oc832_fake_397cc0,
            OC832_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc832_fake_39c870,
            OC832_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc832_fake_1cb418,
            OC832_PREV_1CB418,
            ee::s04::OVERRIDE_001CB418,
            ee::s04::f_001cb418__generated,
            0x001c_b418
        ),
        (
            oc832_fake_1ccf00,
            OC832_PREV_1CCF00,
            ee::s04::OVERRIDE_001CCF00,
            ee::s04::f_001ccf00__generated,
            0x001c_cf00
        ),
        (
            oc832_fake_1ce3c8,
            OC832_PREV_1CE3C8,
            ee::s04::OVERRIDE_001CE3C8,
            ee::s04::f_001ce3c8__generated,
            0x001c_e3c8
        ),
        (
            oc832_fake_188e58,
            OC832_PREV_188E58,
            ee::s02::OVERRIDE_00188E58,
            ee::s02::f_00188e58__generated,
            0x0018_8e58
        ),
        (
            oc832_fake_39f400,
            OC832_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc832_fake_317d70,
            OC832_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc832_fake_39c778,
            OC832_PREV_39C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778
        ),
        (
            oc832_fake_3180b8,
            OC832_PREV_3180B8,
            ee::s10::OVERRIDE_003180B8,
            ee::s10::f_003180b8__generated,
            0x0031_80b8
        ),
        (
            oc832_fake_4166f4,
            OC832_PREV_4166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4
        ),
        (
            oc832_fake_2c2868,
            OC832_PREV_2C2868,
            ee::s08::OVERRIDE_002C2868,
            ee::s08::f_002c2868__generated,
            0x002c_2868
        ),
        (
            oc832_fake_158618,
            OC832_PREV_158618,
            ee::s01::OVERRIDE_00158618,
            ee::s01::f_00158618__generated,
            0x0015_8618
        ),
    );

    /// Scrambled machine with the fakes' gate and the world at `gp-0x848`.
    pub(crate) fn oc832_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        oc832_install_fakes();
        let _ = machine(c, seed, salt, calls);
        c.bus.write32(OC832_CTRL, OC832_MAGIC);
        oc832_w(c, GAME_GP.wrapping_sub(0x848), OC832_WORLD);
    }

    /// `cFEStateOptions_OnWidgetCreated`: the widget name matches each of the
    /// eight option names in turn, or none.
    #[test]
    fn cfe_state_options_on_widget_created_matches() {
        check(
            ee::s02::f_00188870__generated,
            cfe_state_options_on_widget_created,
            18,
            &|c, seed| {
                oc832_machine(c, seed, 0x188870, 8);
                c.pc = 0x0018_8870;
                c.g[A1][0] = sx(OC832_WIDGET);
                oc832_w(c, OC832_WIDGET + 0x38, OC832_HASH);
                oc832_w(c, OC832_OBJ + 0x18, 0x77);
                let m = seed % 9;
                if m < 8 {
                    oc832_w(c, LOG_RESULTS + 4 * m, OC832_HASH);
                }
            },
        );
    }

    /// The screen, widget and engine `cFEStateOptions_OnWidgetAction` reads.
    pub(crate) fn oc832_action_fixture(c: &mut Cpu, key: u32) {
        c.pc = 0x0018_8980;
        c.g[A0][0] = sx(OC832_OBJ);
        c.g[A1][0] = sx(OC832_WIDGET);
        c.g[A2][0] = key as u64;
        oc832_w(c, OC832_OBJ + 0x10, OC832_ENGINE);
        oc832_w(c, OC832_OBJ + 0x1c, 3);
        oc832_w(c, OC832_OBJ + 0x40, 0x7777);
        oc832_w(c, OC832_WIDGET + 0x38, OC832_HASH);
        oc832_w(c, OC832_WIDGET + 0x18, 0x55);
        oc832_w(c, OC832_ENGINE, OC832_CUSTOM);
        oc832_w(c, OC832_CUSTOM + 4, OC832_VT);
        c.bus.write16(OC832_VT + 0x20, 0x10);
        oc832_w(c, OC832_VT + 0x24, FAKE + 0x20);
    }

    /// Set the "options changed" flag number `which` (1: `gp-0x1c10`, 2..5: the
    /// four change flags).
    pub(crate) fn oc832_option_flag(c: &mut Cpu, which: u32) {
        let off = [0, 0x1c10, 0x1c20, 0x1c1c, 0x1c14, 0x1c18][which as usize];
        if off != 0 {
            oc832_w(c, GAME_GP.wrapping_sub(off), 1);
        }
    }

    /// `cFEStateOptions_OnWidgetAction`, event 6 (back).
    #[test]
    fn cfe_state_options_on_widget_action_back_matches() {
        check(
            ee::s02::f_00188980__generated,
            cfe_state_options_on_widget_action,
            24,
            &|c, seed| {
                oc832_machine(c, seed, 0x188980, 4);
                oc832_action_fixture(c, 6);
                oc832_option_flag(c, seed % 6);
                if seed / 6 % 2 == 1 {
                    oc832_w(c, LOG_RESULTS + 4, 0xffff);
                }
            },
        );
    }

    /// `cFEStateOptions_OnWidgetAction`, events that do nothing.
    #[test]
    fn cfe_state_options_on_widget_action_other_matches() {
        check(
            ee::s02::f_00188980__generated,
            cfe_state_options_on_widget_action,
            10,
            &|c, seed| {
                oc832_machine(c, seed, 0x188980, 1);
                let key = [0u32, 1, 2, 3, 4, 7, 8, 0xc, 0xe, 0x20][(seed % 10) as usize];
                oc832_action_fixture(c, key);
            },
        );
    }

    /// `cFEStateOptions_OnWidgetAction`, event 5 on the back widget or on a
    /// widget that asks the engine's factory for a screen.
    #[test]
    fn cfe_state_options_on_widget_action_button_matches() {
        check(
            ee::s02::f_00188980__generated,
            cfe_state_options_on_widget_action,
            192,
            &|c, seed| {
                oc832_machine(c, seed, 0x188981, 6);
                oc832_action_fixture(c, 5);
                let back_widget = seed % 2 == 1;
                oc832_option_flag(c, seed / 2 % 6);
                if seed / 12 % 2 == 1 {
                    oc832_w(c, OC832_OBJ + 0x48, 1);
                }
                if seed / 24 % 2 == 1 {
                    oc832_w(c, OC832_ENGINE, 0);
                }
                if seed / 48 % 2 == 1 {
                    // A real leaf returning 0: the factory has no screen.
                    oc832_w(c, OC832_VT + 0x24, 0x0014_06c0);
                }
                if seed / 96 % 2 == 1 {
                    oc832_w(c, LOG_RESULTS + 12, 0xffff);
                }
                if back_widget {
                    oc832_w(c, LOG_RESULTS + 4, OC832_HASH);
                }
            },
        );
    }

    /// `cFEStateOptions_OnWidgetAction`, event 5 on the keyboard widget. Its
    /// callees `cKeyboardPopup_cKeyboardPopup` (0x1cb030) and
    /// `cUIStateStack_pushPopup` (0x39f290) have no override slot, so they run
    /// for real.
    #[test]
    fn cfe_state_options_on_widget_action_keyboard_matches() {
        check(
            ee::s02::f_00188980__generated,
            cfe_state_options_on_widget_action,
            4,
            &|c, seed| {
                oc832_machine(c, seed, 0x188982, 8);
                oc832_action_fixture(c, 5);
                oc832_w(c, OC832_OBJ + 0x14, 0x0000_ab00);
                oc832_w(c, LOG_RESULTS, OC832_HASH);
            },
        );
    }

    /// `cFEStateOptions_OnWidgetAction`, event 13: the typed cheat code worked
    /// (the cheat hash is forced) or not; the sound object is the engine's or
    /// the null sound.
    #[test]
    fn cfe_state_options_on_widget_action_keyboard_closed_matches() {
        check(
            ee::s02::f_00188980__generated,
            cfe_state_options_on_widget_action,
            8,
            &|c, seed| {
                oc832_machine(c, seed, 0x188983, 24);
                oc832_action_fixture(c, 13);
                oc832_w(c, OC832_OBJ + 0x50, OC832_KEYBOARD);
                if seed & 1 != 0 {
                    oc832_force(c, 0x0031_7670, 0x7_e68d);
                }
                oc832_w(c, GAME_GP.wrapping_add(0x2968), OC832_SOUND_VT);
                c.bus.write16(OC832_SOUND_VT + 0x10, 8);
                oc832_w(c, OC832_SOUND_VT + 0x14, FAKE + 0x30);
                if seed & 2 != 0 {
                    oc832_w(c, OC832_ENGINE + 0x14, OC832_SOUND);
                    oc832_w(c, OC832_SOUND, OC832_SOUND_VT);
                }
            },
        );
    }

    /// Gate word and magic of this module's fakes (clear of the fake log at
    /// DATA+0x100000).
    pub(crate) const OC833_CTRL: u32 = DATA + 0x2f_1000;

    pub(crate) const OC833_MAGIC: u32 = 0x4f43_3833;

    /// Slot bytes (`self+0x48..0x4a`) the string-hash fake stores, one word.
    pub(crate) const OC833_SLOTS: u32 = DATA + 0x2f_1010;

    /// Count of `UIScreen_FindObject` calls and the results table (16 words).
    pub(crate) const OC833_FINDN: u32 = DATA + 0x2f_1020;

    pub(crate) const OC833_FIND: u32 = DATA + 0x2f_1040;

    /// Bit i set: the i-th `UIListBox_FindItemByValue` call answers 0.
    pub(crate) const OC833_MISS: u32 = DATA + 0x2f_1030;

    /// The standard fake, then the answers this test steers: the slot bytes
    /// (after each string hash, so the function's own stores are replaced
    /// before its re-reads), the found widgets, the list item lookup.
    pub(crate) fn oc833_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        match addr {
            0x0031_7670 => {
                let s1 = c.g[S1][0] as u32;
                let w = c.bus.read32(OC833_SLOTS);
                if w != 0 {
                    for i in 0..3u32 {
                        let b = (w >> (8 * i)) as u8;
                        c.bus.write8(s1.wrapping_add(0x48 + i), b);
                    }
                }
            }
            0x0039_d860 => {
                let k = c.bus.read32(OC833_FINDN);
                c.bus.write32(OC833_FINDN, k + 1);
                c.g[V0][0] = c.bus.read32(OC833_FIND + 4 * (k % 16)) as u64;
            }
            0x0039_a768 => {
                if c.bus.read32(OC833_MISS) >> (n % 32) & 1 != 0 {
                    c.g[V0][0] = 0;
                }
            }
            _ => {}
        }
    }

    macro_rules! oc833_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC833_CTRL) != OC833_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc833_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc833_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc833_fakes!(
        (
            oc833_fake_317670,
            OC833_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc833_fake_397cc0,
            OC833_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc833_fake_39c870,
            OC833_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc833_fake_14dc80,
            OC833_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc833_fake_14dd58,
            OC833_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc833_fake_39d860,
            OC833_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc833_fake_39b960,
            OC833_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
        (
            oc833_fake_39a768,
            OC833_PREV_39A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039_a768
        ),
        (
            oc833_fake_39a7a8,
            OC833_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039_a7a8
        ),
        (
            oc833_fake_39a568,
            OC833_PREV_39A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039_a568
        ),
        (
            oc833_fake_3a1000,
            OC833_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
    );

    /// `cFEStateOptionsGame_Init`: screen created or not, the object and
    /// widget lookups finding or missing widgets, the row slots as the
    /// function stores them or replaced (so the language row and every
    /// negative-slot skip run), list-item lookup hits and misses, and callee
    /// unwinds at random calls. The options record holds random data.
    #[test]
    fn fe_state_options_game_init_matches() {
        oc833_install_fakes();
        check(
            ee::s02::f_00188f28__generated,
            oc833_fe_state_options_game_init,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8331, 60);
                c.bus.write32(OC833_CTRL, OC833_MAGIC);
                let slots = match seed % 8 {
                    0..=3 => 0,
                    4 => 0x0001_0200,
                    5 => 0x00ff_ff01,
                    6 => 0x0002_0100,
                    _ => 0x00ff_0201,
                };
                c.bus.write32(OC833_SLOTS, slots);
                c.bus.write32(OC833_MISS, r.next_u32());
                let obj = fakes::ptr(&mut r);
                let vt = fakes::vtable(c, &mut r, &[0x48, 0x40]);
                c.bus.write32(obj + 8, vt);
                c.bus.write32(OC833_FIND, obj);
                for k in 1..16u32 {
                    let v = if (seed >> (k % 5 + 3)) & 1 == 1 && k % 3 == 0 {
                        0
                    } else {
                        fakes::ptr(&mut r)
                    };
                    c.bus.write32(OC833_FIND + 4 * k, v);
                }
                for i in 0..(0x290 / 8) {
                    c.bus.write64(
                        OC833_OPTIONS + 8 * i,
                        ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
                    );
                }
                let this = DATA + 0x30_0000 + (r.next_u32() & 0xf_fff0);
                c.g[A0][0] = sx(this);
                c.g[RA][0] = sx(0x0020_0000 + 0x10 * seed);
                c.pc = 0x0018_8f28;
            },
        );
    }

    /// Fixture objects, all above DATA+0x200000: clear of the fake log
    /// (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC834_CTRL: u32 = DATA + 0x20_0100;

    pub(crate) const OC834_MAGIC: u32 = 0x4f43_8340;

    /// The shared fake vtable: every slot `{delta 0, FAKE + offset}`.
    pub(crate) const OC834_VT: u32 = DATA + 0x22_0000;

    /// Four 0x1000-byte objects; the last one is the interface object.
    pub(crate) const OC834_OBJ: u32 = DATA + 0x23_0000;

    pub(crate) const OC834_IFACE: u32 = OC834_OBJ + 3 * 0x1000;

    /// Forced result meaning "v0 = 0" (a fake cannot return 0 by itself).
    pub(crate) const OC834_ZERO: u32 = 0xffff_ffff;

    /// Values that fields and forced results are drawn from, so that
    /// comparisons between the two come out equal now and then.
    pub(crate) const OC834_POOL: [u32; 8] = [
        0,
        0x1234,
        0x5678,
        OC834_OBJ,
        OC834_OBJ + 0x1000,
        OC834_IFACE,
        0x0001_0203,
        0xff01_0200,
    ];

    /// The fake callee: logs the call, returns a pointer or the forced result
    /// of this call index (`OC834_ZERO`: 0).
    pub(crate) fn oc834_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let forced = if n < 8 {
            c.bus.read32(fakes::LOG_RESULTS + 4 * n)
        } else {
            0
        };
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc == ra && forced == OC834_ZERO {
            c.g[V0][0] = 0;
        }
    }

    pub(crate) fn oc834_miss(c: &mut Cpu, target: u32) {
        oc834_call(c, target);
    }

    macro_rules! oc834_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC834_CTRL) != OC834_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc834_call(c, $addr);
                }
            )*
            fn oc834_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc834_fakes!(
        (
            oc834_fake_14dc80,
            OC834_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc834_fake_14dd58,
            OC834_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc834_fake_14f4f8,
            OC834_PREV_14F4F8,
            ee::s01::OVERRIDE_0014F4F8,
            ee::s01::f_0014f4f8__generated,
            0x0014_f4f8
        ),
        (
            oc834_fake_228c08,
            OC834_PREV_228C08,
            ee::s05::OVERRIDE_00228C08,
            ee::s05::f_00228c08__generated,
            0x0022_8c08
        ),
        (
            oc834_fake_28b180,
            OC834_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc834_fake_29ed90,
            OC834_PREV_29ED90,
            ee::s08::OVERRIDE_0029ED90,
            ee::s08::f_0029ed90__generated,
            0x0029_ed90
        ),
        (
            oc834_fake_317670,
            OC834_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc834_fake_317d70,
            OC834_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc834_fake_397870,
            OC834_PREV_397870,
            ee::s13::OVERRIDE_00397870,
            ee::s13::f_00397870__generated,
            0x0039_7870
        ),
        (
            oc834_fake_397840,
            OC834_PREV_397840,
            ee::s13::OVERRIDE_00397840,
            ee::s13::f_00397840__generated,
            0x0039_7840
        ),
        (
            oc834_fake_3978f0,
            OC834_PREV_3978F0,
            ee::s13::OVERRIDE_003978F0,
            ee::s13::f_003978f0__generated,
            0x0039_78f0
        ),
        (
            oc834_fake_39a738,
            OC834_PREV_39A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738
        ),
        (
            oc834_fake_39a7a8,
            OC834_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039_a7a8
        ),
        (
            oc834_fake_39d860,
            OC834_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc834_fake_39e318,
            OC834_PREV_39E318,
            ee::s13::OVERRIDE_0039E318,
            ee::s13::f_0039e318__generated,
            0x0039_e318
        ),
        (
            oc834_fake_39e4c0,
            OC834_PREV_39E4C0,
            ee::s13::OVERRIDE_0039E4C0,
            ee::s13::f_0039e4c0__generated,
            0x0039_e4c0
        ),
        (
            oc834_fake_3a1000,
            OC834_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
    );

    pub(crate) fn oc834_pick(r: &mut Rng) -> u32 {
        OC834_POOL[(r.next_u32() % 8) as usize]
    }

    /// A machine with the gated fakes on, the vtable and objects built and
    /// the forced results of the first calls drawn from the pool.
    pub(crate) fn oc834_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc834_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.hooks.miss = oc834_miss;
        c.bus.write32(OC834_CTRL, OC834_MAGIC);
        for off in (0..0x400u32).step_by(8) {
            c.bus.write16(OC834_VT + off, 0);
            c.bus.write32(OC834_VT + off + 4, fakes::FAKE + off);
        }
        for i in 0..4u32 {
            let base = OC834_OBJ + i * 0x1000;
            for off in (0..0x400u32).step_by(4) {
                let v = oc834_pick(&mut r);
                c.bus.write32(base + off, v);
            }
            c.bus.write32(base + 8, OC834_VT);
        }
        c.bus.write32(OC834_IFACE + 0xc, OC834_VT);
        for n in 0..8u32 {
            let v = oc834_pick(&mut r);
            c.bus.write32(
                fakes::LOG_RESULTS + 4 * n,
                if v == 0 { OC834_ZERO } else { v },
            );
        }
        r
    }

    /// Menu notifications: keys 6/7 compare the list item's name hash.
    #[test]
    fn cfe_state_options_on_widget_notify_matches() {
        check(
            ee::s02::f_00188c68__generated,
            cfe_state_options_on_widget_notify,
            400,
            &|c, seed| {
                let mut r = oc834_setup(c, seed, 0x188c68, 3);
                c.g[A0][0] = OC834_OBJ as u64;
                c.g[A1][0] = (OC834_OBJ + 0x1000) as u64;
                c.g[A2][0] = [5u64, 6, 7, 8, 9, 10, 0x20, 6, 7][(r.next_u32() % 9) as usize];
                c.g[A3][0] = (r.next_u32() % 4) as u64;
            },
        );
    }

    /// Popup child events 0x15 (set the texts) and 0x16 (answer).
    #[test]
    fn cfe_state_options_on_child_event_matches() {
        check(
            ee::s02::f_00188d08__generated,
            cfe_state_options_on_child_event,
            400,
            &|c, seed| {
                let mut r = oc834_setup(c, seed, 0x188d08, 8);
                c.g[A0][0] = OC834_OBJ as u64;
                c.g[A1][0] = (OC834_OBJ + 0x1000) as u64;
                c.g[A2][0] = [0x15u64, 0x16, 0x16, 0x15, 0x17][(r.next_u32() % 5) as usize];
            },
        );
    }

    /// OnActivate is a single call to the base activation.
    #[test]
    fn cfe_state_options_game_on_activate_matches() {
        check(
            ee::s02::f_001895f0__generated,
            cfe_state_options_game_on_activate,
            12,
            &|c, seed| {
                oc834_setup(c, seed, 0x1895f0, 1);
                c.g[A0][0] = OC834_OBJ as u64;
            },
        );
    }

    /// Game options popup events.
    #[test]
    fn cfe_state_options_game_on_child_event_matches() {
        check(
            ee::s02::f_00189610__generated,
            cfe_state_options_game_on_child_event,
            600,
            &|c, seed| {
                let mut r = oc834_setup(c, seed, 0x189610, 14);
                c.g[A0][0] = OC834_OBJ as u64;
                c.g[A1][0] = (OC834_OBJ + 0x1000) as u64;
                c.g[A2][0] = [0x15u64, 0x16, 0x16, 0x16, 0x17][(r.next_u32() % 5) as usize];
            },
        );
    }

    /// Widget actions: popups (events 5, 7), language and unit options (9),
    /// the next state (6); a null widget returns at once.
    #[test]
    fn cfe_state_options_game_on_widget_action_matches() {
        check(
            ee::s02::f_00189980__generated,
            cfe_state_options_game_on_widget_action,
            600,
            &|c, seed| {
                let mut r = oc834_setup(c, seed, 0x189980, 14);
                c.g[A0][0] = OC834_OBJ as u64;
                c.g[A1][0] = if r.next_u32() % 8 == 0 {
                    0
                } else {
                    (OC834_OBJ + 0x1000) as u64
                };
                c.g[A2][0] = [5u64, 6, 7, 9, 9, 5, 8][(r.next_u32() % 7) as usize];
            },
        );
    }

    /// Refresh the list boxes of the game options screen.
    #[test]
    fn cfe_state_options_game_refresh_list_boxes_matches() {
        check(
            ee::s02::f_00189d60__generated,
            cfe_state_options_game_refresh_list_boxes,
            600,
            &|c, seed| {
                oc834_setup(c, seed, 0x189d60, 14);
                c.g[A0][0] = OC834_OBJ as u64;
            },
        );
    }

    // Fixture objects, in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC835_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC835_MAGIC: u32 = 0x4f43_3835;

    pub(crate) const OC835_SELF: u32 = DATA + 0x21_0000;

    pub(crate) const OC835_ENGINE: u32 = DATA + 0x22_0000;

    pub(crate) const OC835_ENG: u32 = DATA + 0x23_0000;

    pub(crate) const OC835_ENGVT: u32 = DATA + 0x24_0000;

    pub(crate) const OC835_WIDGET: u32 = DATA + 0x25_0000;

    pub(crate) const OC835_IFACE: u32 = DATA + 0x26_0000;

    pub(crate) const OC835_IFVT: u32 = DATA + 0x27_0000;

    pub(crate) const OC835_FORCE: u32 = DATA + 0x30_0000;

    pub(crate) fn oc835_mail(addr: u32) -> u32 {
        OC835_FORCE + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3; 1 = zero).
    pub(crate) fn oc835_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc835_mail(addr), v.wrapping_add(3));
    }

    pub(crate) fn oc835_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc835_mail(addr), 1);
    }

    /// The fake callee: logs the call, then returns its mailbox value; the
    /// string hash (`0x317670`) echoes its argument (the string address).
    pub(crate) fn oc835_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc835_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if addr == 0x0031_7670 {
            c.g[V0][0] = a0;
        } else if mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc835_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC835_CTRL) != OC835_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc835_fake_call(c, $addr);
                }
            )*
            fn oc835_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc835_fakes!(
        (
            oc835_fake_14dc80,
            OC835_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc835_fake_14dd58,
            OC835_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc835_fake_317d70,
            OC835_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc835_fake_317670,
            OC835_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc835_fake_39e318,
            OC835_PREV_39E318,
            ee::s13::OVERRIDE_0039E318,
            ee::s13::f_0039e318__generated,
            0x0039_e318
        ),
        (
            oc835_fake_39f400,
            OC835_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc835_fake_39a738,
            OC835_PREV_39A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738
        ),
        (
            oc835_fake_397840,
            OC835_PREV_397840,
            ee::s13::OVERRIDE_00397840,
            ee::s13::f_00397840__generated,
            0x0039_7840
        ),
        (
            oc835_fake_3978f0,
            OC835_PREV_3978F0,
            ee::s13::OVERRIDE_003978F0,
            ee::s13::f_003978f0__generated,
            0x0039_78f0
        ),
        (
            oc835_fake_28b180,
            OC835_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc835_fake_287410,
            OC835_PREV_287410,
            ee::s07::OVERRIDE_00287410,
            ee::s07::f_00287410__generated,
            0x0028_7410
        ),
        (
            oc835_fake_287488,
            OC835_PREV_287488,
            ee::s07::OVERRIDE_00287488,
            ee::s07::f_00287488__generated,
            0x0028_7488
        ),
        (
            oc835_fake_287520,
            OC835_PREV_287520,
            ee::s07::OVERRIDE_00287520,
            ee::s07::f_00287520__generated,
            0x0028_7520
        ),
        (
            oc835_fake_287558,
            OC835_PREV_287558,
            ee::s07::OVERRIDE_00287558,
            ee::s07::f_00287558__generated,
            0x0028_7558
        ),
        (
            oc835_fake_2875d0,
            OC835_PREV_2875D0,
            ee::s07::OVERRIDE_002875D0,
            ee::s07::f_002875d0__generated,
            0x0028_75d0
        ),
    );

    /// Name-hash string offsets of the nine `Is()` tests in order of the
    /// chain (1lb, 2sl, 3sl, 4sl, 5lb, 6lb, 7lb) plus "none".
    pub(crate) const OC835_STR_OFFS: [u32; 8] = [
        0x1568, 0x1590, 0x1598, 0x15a0, 0x15a8, 0x15b0, 0x15b8, 0x0001,
    ];

    /// `OnWidgetAction`: null widget, events 5/6/7/9/others; event 9 over
    /// every control (and none), the flag word at gp-0x1680 at 1 and not,
    /// the engine's state factory returning 0 (real leaf) or a state, the
    /// list box returning 0/1/2/3/5. The 0x39f290 push and its `UIList`
    /// callees run real on the fixture engine; the compiler's unaligned
    /// snapshot loops are unreachable.
    #[test]
    fn cfe_state_options_sound_on_widget_action_matches() {
        oc835_install_fakes();
        check(
            ee::s02::f_0018a8d0__generated,
            cfe_state_options_sound_on_widget_action,
            320,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0835_0001, 14);
                c.bus.write32(OC835_CTRL, OC835_MAGIC);
                for i in 1..=0x300u32 {
                    let w = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(SP - 4 * i, w);
                }
                // The option block the snapshots copy.
                fill(c, &mut r, SOUND_OPTION_BLOCK, 0xa2);
                let gp = c.g[GP][0] as u32;
                let flag = match (seed / 7) % 3 {
                    0 => 1,
                    1 => 0,
                    _ => 2,
                };
                c.bus.write32(gp.wrapping_sub(SOUND_FLAG_GP), flag);
                c.bus
                    .write32(gp.wrapping_sub(SOUND_CHANGED_GP), r.next_u32());
                // The back-end interface (0x14dd58) and its vtable slot.
                oc835_force(c, 0x0014_dd58, OC835_IFACE);
                c.bus.write32(OC835_IFACE + 0xc, OC835_IFVT);
                c.bus.write16(OC835_IFVT + 8, 0x10 * (seed % 3) as u16);
                c.bus.write32(OC835_IFVT + 0xc, FAKE + 0x20);
                // The engine, its vtable and the state factory.
                c.bus.write32(OC835_SELF + 0x10, OC835_ENGINE);
                c.bus.write32(OC835_ENGINE, OC835_ENG);
                c.bus.write32(OC835_ENG + 4, OC835_ENGVT);
                c.bus
                    .write16(OC835_ENGVT + 0x28, 0x10 * ((seed / 3) % 3) as u16);
                let factory = if seed % 2 == 0 {
                    0x0014_06c0
                } else {
                    FAKE + 0x40
                };
                c.bus.write32(OC835_ENGVT + 0x2c, factory);
                // The widget.
                fill(c, &mut r, OC835_WIDGET, 0x40);
                let k = seed % 16;
                let sub = ((seed / 16) % 8) as usize;
                let hash = 0x4a_0000 + OC835_STR_OFFS[sub];
                c.bus
                    .write32(OC835_WIDGET + 0x38, if sub == 7 { 0x1_2345 } else { hash });
                c.bus.write8(OC835_WIDGET + 0x74, r.next_u32() as u8);
                let value = [0u32, 1, 2, 3, 5, 0x7fff_ffff][((seed / 5) % 6) as usize];
                oc835_force(c, 0x0039_a738, value);
                if (seed / 3) % 2 == 0 {
                    oc835_force_zero(c, 0x0039_7840);
                }
                let events: [u32; 16] = [0, 6, 9, 9, 9, 7, 9, 9, 9, 5, 0, 8, 10, 0xffff_ffff, 6, 9];
                c.g[A0][0] = sx(OC835_SELF);
                c.g[A1][0] = if k == 0 { 0 } else { sx(OC835_WIDGET) };
                c.g[A2][0] = if k == 0 {
                    sx(r.next_u32() % 12)
                } else {
                    sx(events[k as usize])
                };
                c.pc = 0x0018_a8d0;
            },
        );
    }

    /// Gate word of the fakes (own area, clear of the fake log at DATA+0x100000).
    pub(crate) const OC836_CTRL: u32 = DATA + 0x2a_0000;

    pub(crate) const OC836_MAGIC: u32 = 0x4f43_3836;

    /// Call counter and "absent" bit mask of the `Find` fake (0x39d860).
    pub(crate) const OC836_FIND_N: u32 = OC836_CTRL + 4;

    pub(crate) const OC836_FIND_MASK: u32 = OC836_CTRL + 8;

    /// Same for the `getEntryByData` fake (0x39a768).
    pub(crate) const OC836_ENTRY_N: u32 = OC836_CTRL + 0xc;

    pub(crate) const OC836_ENTRY_MASK: u32 = OC836_CTRL + 0x10;

    /// The screen object (`self`), the BE option struct (`Cfg_18B380()` source)
    /// and the `D_004A1A70` flag (read as `gp - 0x1680`).
    pub(crate) const OC836_SELF: u32 = DATA + 0x2a_1000;

    pub(crate) const OC836_BEOPT: u32 = 0x0053_5610;

    pub(crate) const OC836_FLAG: u32 = GAME_GP - 0x1680;

    /// Logs the call like `fakes::fake_call`; then the `Find` and
    /// `getEntryByData` fakes answer 0 on the calls whose bit is set in their
    /// mask (the other calls keep the fake's pointer).
    pub(crate) fn oc836_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let (n_at, mask_at) = match addr {
            0x0039_d860 => (OC836_FIND_N, OC836_FIND_MASK),
            0x0039_a768 => (OC836_ENTRY_N, OC836_ENTRY_MASK),
            _ => return,
        };
        let n = c.bus.read32(n_at);
        c.bus.write32(n_at, n + 1);
        if (c.bus.read32(mask_at) >> (n % 32)) & 1 != 0 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc836_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC836_CTRL) != OC836_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc836_fake_call(c, $addr);
                }
            )*
            fn oc836_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc836_fakes!(
        (
            oc836_fake_14dc80,
            OC836_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc836_fake_14dd58,
            OC836_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc836_fake_317670,
            OC836_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc836_fake_39d860,
            OC836_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc836_fake_39a670,
            OC836_PREV_39A670,
            ee::s13::OVERRIDE_0039A670,
            ee::s13::f_0039a670__generated,
            0x0039_a670
        ),
        (
            oc836_fake_39a568,
            OC836_PREV_39A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039_a568
        ),
        (
            oc836_fake_28b180,
            OC836_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc836_fake_39a768,
            OC836_PREV_39A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039_a768
        ),
        (
            oc836_fake_39a7a8,
            OC836_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039_a7a8
        ),
        (
            oc836_fake_287410,
            OC836_PREV_287410,
            ee::s07::OVERRIDE_00287410,
            ee::s07::f_00287410__generated,
            0x0028_7410
        ),
        (
            oc836_fake_287488,
            OC836_PREV_287488,
            ee::s07::OVERRIDE_00287488,
            ee::s07::f_00287488__generated,
            0x0028_7488
        ),
        (
            oc836_fake_287520,
            OC836_PREV_287520,
            ee::s07::OVERRIDE_00287520,
            ee::s07::f_00287520__generated,
            0x0028_7520
        ),
        (
            oc836_fake_287558,
            OC836_PREV_287558,
            ee::s07::OVERRIDE_00287558,
            ee::s07::f_00287558__generated,
            0x0028_7558
        ),
        (
            oc836_fake_2875d0,
            OC836_PREV_2875D0,
            ee::s07::OVERRIDE_002875D0,
            ee::s07::f_002875d0__generated,
            0x0028_75d0
        ),
    );

    /// `cFEStateOptionsSound_RefreshControls`: widgets present or absent per
    /// section (bit mask over the seven `Find` calls), entries found or not
    /// (mask over the four `getEntryByData` calls), the optional seventh list
    /// on or off (`D_004A1A70`), random option struct, and unwinds in the
    /// callees. The 0x289dc0 / 0x289d60 sound-mode getters run for real on
    /// the audio object the fake returns.
    #[test]
    fn cfe_state_options_sound_refresh_controls_matches() {
        oc836_install_fakes();
        check(
            ee::s02::f_0018b380__generated,
            cfe_state_options_sound_refresh_controls,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x18b380, 55);
                c.bus.write32(OC836_CTRL, OC836_MAGIC);
                c.bus.write32(OC836_FIND_N, 0);
                c.bus.write32(OC836_ENTRY_N, 0);
                // Mostly present widgets, a few seeds with absent ones.
                let absent = match seed % 4 {
                    0 => 0,
                    1 => r.next_u32() & r.next_u32(),
                    2 => r.next_u32(),
                    _ => 0x7f & r.next_u32(),
                };
                c.bus.write32(OC836_FIND_MASK, absent);
                c.bus.write32(OC836_ENTRY_MASK, r.next_u32());
                c.bus.write32(OC836_FLAG, if seed % 3 == 0 { 0 } else { 1 });
                fill(c, &mut r, OC836_BEOPT, 0xa2);
                for i in 0..0x300u32 {
                    c.bus
                        .write32(crate::difftest::SP - 4 * (i + 1), r.next_u32());
                }
                fill(c, &mut r, OC836_SELF, 0x20);
                c.g[A0][0] = sx(OC836_SELF);
                c.pc = 0x0018_b380;
            },
        );
    }

    /// Gate word of this module's fakes, and its magic value.
    pub(crate) const CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const MAGIC: u32 = 0x4f43_8337;

    /// Per-callee result queues: 0x40 bytes per callee, `[count, e0, e1, ..]`.
    pub(crate) const MAIL: u32 = DATA + 0x30_0000;

    /// Fixture objects (above the range of the fakes' result pointers).
    pub(crate) const FIX: u32 = DATA + 0x8_0000;

    /// A faked function used as a virtual method with a controllable result.
    pub(crate) const VMETHOD: u32 = 0x0014_f600;

    /// The saved options struct the sound handler copies (0x290 bytes).
    pub(crate) const CFG: u32 = 0x0053_5610;

    /// Queue entries: 0 = the fake's own pointer, 1 = null, 2 = a0, `val(v)` = v.
    pub(crate) const NULL: u32 = 1;

    pub(crate) fn val(v: u32) -> u32 {
        v.wrapping_add(3)
    }

    /// The faked callees, in queue order.
    pub(crate) const ADDRS: &[u32] = &[
        0x00317670, 0x00317d70, 0x00397cc0, 0x0039c870, 0x0039d860, 0x0039e318, 0x0039f400,
        0x0039e4c0, 0x0014dc80, 0x0014dd58, 0x0014f458, 0x0014f600, 0x0028b180, 0x00287410,
        0x00287488, 0x00287520, 0x00287558, 0x002875d0, 0x00287670, 0x002876a0, 0x002876d0,
        0x00286200, 0x00284c28, 0x00284bb0, 0x00285fb0,
    ];

    pub(crate) fn mail(addr: u32) -> u32 {
        MAIL + 0x40 * ADDRS.iter().position(|&a| a == addr).unwrap() as u32
    }

    /// Queue the results of the next calls of the faked function `addr`.
    pub(crate) fn queue(c: &mut Cpu, addr: u32, entries: &[u32]) {
        for (i, &e) in entries.iter().enumerate() {
            c.bus.write32(mail(addr) + 4 + 4 * i as u32, e);
        }
    }

    pub(crate) fn w_2(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    pub(crate) fn w16(c: &mut Cpu, addr: u32, v: u16) {
        c.bus.write16(addr, v);
    }

    /// A gated fake callee: logs the call like `fakes::fake_call`, then
    /// returns the next queued result. `RefreshControls` (0x18b380, not
    /// replaced here) finds none of its screen objects.
    pub(crate) fn oc837_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let refresh = addr == 0x0039_d860 && (0x0018_b380..0x0018_bef8).contains(&ra);
        let mut entry = 0;
        if !refresh {
            let q = mail(addr);
            let n = c.bus.read32(q);
            c.bus.write32(q, n.wrapping_add(1));
            if n < 15 {
                entry = c.bus.read32(q + 4 + 4 * n);
            }
        }
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if refresh {
            c.g[V0][0] = 0;
            return;
        }
        match entry {
            0 => {}
            1 => c.g[V0][0] = 0,
            2 => c.g[V0][0] = a0,
            m => c.g[V0][0] = m.wrapping_sub(3) as u64,
        }
    }

    macro_rules! oc837_fake {
        ($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(CTRL) != MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                oc837_fake_call(c, $addr);
            }
        };
    }

    oc837_fake!(
        oc837_fake_317670,
        OC837_PREV_317670,
        ee::s10::OVERRIDE_00317670,
        ee::s10::f_00317670__generated,
        0x00317670
    );

    oc837_fake!(
        oc837_fake_317d70,
        OC837_PREV_317D70,
        ee::s10::OVERRIDE_00317D70,
        ee::s10::f_00317d70__generated,
        0x00317d70
    );

    oc837_fake!(
        oc837_fake_397cc0,
        OC837_PREV_397CC0,
        ee::s13::OVERRIDE_00397CC0,
        ee::s13::f_00397cc0__generated,
        0x00397cc0
    );

    oc837_fake!(
        oc837_fake_39c870,
        OC837_PREV_39C870,
        ee::s13::OVERRIDE_0039C870,
        ee::s13::f_0039c870__generated,
        0x0039c870
    );

    oc837_fake!(
        oc837_fake_39d860,
        OC837_PREV_39D860,
        ee::s13::OVERRIDE_0039D860,
        ee::s13::f_0039d860__generated,
        0x0039d860
    );

    oc837_fake!(
        oc837_fake_39e318,
        OC837_PREV_39E318,
        ee::s13::OVERRIDE_0039E318,
        ee::s13::f_0039e318__generated,
        0x0039e318
    );

    oc837_fake!(
        oc837_fake_39f400,
        OC837_PREV_39F400,
        ee::s13::OVERRIDE_0039F400,
        ee::s13::f_0039f400__generated,
        0x0039f400
    );

    oc837_fake!(
        oc837_fake_39e4c0,
        OC837_PREV_39E4C0,
        ee::s13::OVERRIDE_0039E4C0,
        ee::s13::f_0039e4c0__generated,
        0x0039e4c0
    );

    oc837_fake!(
        oc837_fake_14dc80,
        OC837_PREV_14DC80,
        ee::s01::OVERRIDE_0014DC80,
        ee::s01::f_0014dc80__generated,
        0x0014dc80
    );

    oc837_fake!(
        oc837_fake_14dd58,
        OC837_PREV_14DD58,
        ee::s01::OVERRIDE_0014DD58,
        ee::s01::f_0014dd58__generated,
        0x0014dd58
    );

    oc837_fake!(
        oc837_fake_14f458,
        OC837_PREV_14F458,
        ee::s01::OVERRIDE_0014F458,
        ee::s01::f_0014f458__generated,
        0x0014f458
    );

    oc837_fake!(
        oc837_fake_14f600,
        OC837_PREV_14F600,
        ee::s01::OVERRIDE_0014F600,
        ee::s01::f_0014f600__generated,
        0x0014f600
    );

    oc837_fake!(
        oc837_fake_28b180,
        OC837_PREV_28B180,
        ee::s07::OVERRIDE_0028B180,
        ee::s07::f_0028b180__generated,
        0x0028b180
    );

    oc837_fake!(
        oc837_fake_287410,
        OC837_PREV_287410,
        ee::s07::OVERRIDE_00287410,
        ee::s07::f_00287410__generated,
        0x00287410
    );

    oc837_fake!(
        oc837_fake_287488,
        OC837_PREV_287488,
        ee::s07::OVERRIDE_00287488,
        ee::s07::f_00287488__generated,
        0x00287488
    );

    oc837_fake!(
        oc837_fake_287520,
        OC837_PREV_287520,
        ee::s07::OVERRIDE_00287520,
        ee::s07::f_00287520__generated,
        0x00287520
    );

    oc837_fake!(
        oc837_fake_287558,
        OC837_PREV_287558,
        ee::s07::OVERRIDE_00287558,
        ee::s07::f_00287558__generated,
        0x00287558
    );

    oc837_fake!(
        oc837_fake_2875d0,
        OC837_PREV_2875D0,
        ee::s07::OVERRIDE_002875D0,
        ee::s07::f_002875d0__generated,
        0x002875d0
    );

    oc837_fake!(
        oc837_fake_287670,
        OC837_PREV_287670,
        ee::s07::OVERRIDE_00287670,
        ee::s07::f_00287670__generated,
        0x00287670
    );

    oc837_fake!(
        oc837_fake_2876a0,
        OC837_PREV_2876A0,
        ee::s07::OVERRIDE_002876A0,
        ee::s07::f_002876a0__generated,
        0x002876a0
    );

    oc837_fake!(
        oc837_fake_2876d0,
        OC837_PREV_2876D0,
        ee::s07::OVERRIDE_002876D0,
        ee::s07::f_002876d0__generated,
        0x002876d0
    );

    oc837_fake!(
        oc837_fake_286200,
        OC837_PREV_286200,
        ee::s07::OVERRIDE_00286200,
        ee::s07::f_00286200__generated,
        0x00286200
    );

    oc837_fake!(
        oc837_fake_284c28,
        OC837_PREV_284C28,
        ee::s07::OVERRIDE_00284C28,
        ee::s07::f_00284c28__generated,
        0x00284c28
    );

    oc837_fake!(
        oc837_fake_284bb0,
        OC837_PREV_284BB0,
        ee::s07::OVERRIDE_00284BB0,
        ee::s07::f_00284bb0__generated,
        0x00284bb0
    );

    oc837_fake!(
        oc837_fake_285fb0,
        OC837_PREV_285FB0,
        ee::s07::OVERRIDE_00285FB0,
        ee::s07::f_00285fb0__generated,
        0x00285fb0
    );

    pub(crate) fn oc837_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc837_fake_317670,
            &OC837_PREV_317670,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc837_fake_317d70,
            &OC837_PREV_317D70,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397CC0),
            oc837_fake_397cc0,
            &OC837_PREV_397CC0,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc837_fake_39c870,
            &OC837_PREV_39C870,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc837_fake_39d860,
            &OC837_PREV_39D860,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E318),
            oc837_fake_39e318,
            &OC837_PREV_39E318,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039F400),
            oc837_fake_39f400,
            &OC837_PREV_39F400,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E4C0),
            oc837_fake_39e4c0,
            &OC837_PREV_39E4C0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc837_fake_14dc80,
            &OC837_PREV_14DC80,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc837_fake_14dd58,
            &OC837_PREV_14DD58,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014F458),
            oc837_fake_14f458,
            &OC837_PREV_14F458,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014F600),
            oc837_fake_14f600,
            &OC837_PREV_14F600,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            oc837_fake_28b180,
            &OC837_PREV_28B180,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287410),
            oc837_fake_287410,
            &OC837_PREV_287410,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287488),
            oc837_fake_287488,
            &OC837_PREV_287488,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287520),
            oc837_fake_287520,
            &OC837_PREV_287520,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287558),
            oc837_fake_287558,
            &OC837_PREV_287558,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002875D0),
            oc837_fake_2875d0,
            &OC837_PREV_2875D0,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287670),
            oc837_fake_287670,
            &OC837_PREV_287670,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002876A0),
            oc837_fake_2876a0,
            &OC837_PREV_2876A0,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002876D0),
            oc837_fake_2876d0,
            &OC837_PREV_2876D0,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00286200),
            oc837_fake_286200,
            &OC837_PREV_286200,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00284C28),
            oc837_fake_284c28,
            &OC837_PREV_284C28,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00284BB0),
            oc837_fake_284bb0,
            &OC837_PREV_284BB0,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00285FB0),
            oc837_fake_285fb0,
            &OC837_PREV_285FB0,
        );
    }

    /// The machine for one seed: scrambled registers, the fakes hooked and
    /// gated on.
    pub(crate) fn setup_3(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc837_install_fakes();
        let r = machine(c, seed, salt, calls);
        w_2(c, CTRL, MAGIC);
        r
    }

    /// An object whose vtable (at `+8`) has its slots at `+0x40` and `+0x48`
    /// set to harmless fake methods.
    pub(crate) fn vobject(c: &mut Cpu, obj: u32, vt: u32) {
        w_2(c, obj + 8, vt);
        w16(c, vt + 0x40, 0);
        w_2(c, vt + 0x44, FAKE);
        w16(c, vt + 0x48, 0);
        w_2(c, vt + 0x4c, FAKE + 0x10);
    }

    #[test]
    fn c_fe_state_options_sound_init_matches() {
        check(
            ee::s02::f_0018a298__generated,
            c_fe_state_options_sound_init,
            48,
            &|c, seed| {
                let _r = setup_3(c, seed, 0x18a298, 6);
                let this = FIX;
                let obj = FIX + 0x800;
                c.pc = 0x0018_a298;
                c.g[A0][0] = this as u64;
                w_2(c, this + 0x10, FIX + 0x400);
                w_2(
                    c,
                    GAME_GP.wrapping_sub(0x1680),
                    [0, 1, 2][(seed / 3 % 3) as usize],
                );
                vobject(c, obj, FIX + 0xc00);
                // The screen may be missing; each of the three groups too.
                queue(c, 0x0039_7cc0, &[if seed % 3 == 0 { NULL } else { 0 }]);
                let pick = |bit: u32| {
                    if (seed >> bit) & 1 == 0 {
                        NULL
                    } else {
                        val(obj)
                    }
                };
                queue(c, 0x0039_d860, &[pick(2), pick(3), pick(4)]);
            },
        );
    }

    #[test]
    fn c_fe_state_options_sound_on_child_event_matches() {
        check(
            ee::s02::f_0018a3d8__generated,
            c_fe_state_options_sound_on_child_event,
            96,
            &|c, seed| {
                let mut r = setup_3(c, seed, 0x18a3d8, 8);
                let this = FIX;
                let popup = FIX + 0x400;
                let pvt = FIX + 0x800;
                let hv = r.next_u32() | 0x1000_0000;
                c.pc = 0x0018_a3d8;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = popup as u64;
                c.g[A2][0] = match seed % 4 {
                    0 => 0x15,
                    1 => 0x17,
                    _ => 0x16,
                };
                w_2(c, popup + 0xc, hv);
                w_2(c, popup + 8, pvt);
                w16(c, pvt + 0xb8, 0);
                w_2(c, pvt + 0xbc, VMETHOD);
                // The popup's name hashes to the "reset options" id or not.
                let hit = (seed / 4) % 3 != 0;
                queue(c, 0x0031_7670, &[if hit { val(hv) } else { 0 }]);
                // The popup's answer method: answered "no" (0) or "yes".
                queue(
                    c,
                    VMETHOD,
                    &[if (seed / 12) % 2 == 0 { NULL } else { val(1) }],
                );
                for i in 0..0xa4 {
                    w_2(c, CFG + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn c_fe_state_options_sound_on_activate_matches() {
        check(
            ee::s02::f_0018a870__generated,
            c_fe_state_options_sound_on_activate,
            12,
            &|c, seed| {
                let _r = setup_3(c, seed, 0x18a870, 1);
                c.pc = 0x0018_a870;
                c.g[A0][0] = FIX as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_options_sound_set_speaker_mode_matches() {
        check(
            ee::s02::f_0018bef8__generated,
            c_fe_state_options_sound_set_speaker_mode,
            60,
            &|c, seed| {
                let _r = setup_3(c, seed, 0x18bef8, 6);
                c.pc = 0x0018_bef8;
                c.g[A0][0] = FIX as u64;
                c.g[A1][0] = [0u64, 1, 2, 3, 7, u64::MAX][(seed % 6) as usize];
                // 28b180 returns the audio object; its music system's method
                // (slot 2 of the object at +0x1d8) answers "accepted" or not.
                let (p, vt1, o1, o2, v) = (
                    FIX + 0x400,
                    FIX + 0x800,
                    FIX + 0xc00,
                    FIX + 0x1000,
                    FIX + 0x1400,
                );
                w_2(c, p + 0x118, vt1);
                w_2(c, vt1, o1);
                w_2(c, o1 + 0x1d8, o2);
                w_2(c, o2 + 4, v);
                w16(c, v + 0x10, 0);
                w_2(c, v + 0x14, VMETHOD);
                queue(c, 0x0028_b180, &[val(p)]);
                queue(
                    c,
                    VMETHOD,
                    &[if (seed / 6) % 2 == 0 { val(1) } else { NULL }],
                );
            },
        );
    }

    #[test]
    fn c_fe_state_options_hud_on_activate_matches() {
        check(
            ee::s02::f_0018c198__generated,
            c_fe_state_options_hud_on_activate,
            12,
            &|c, seed| {
                let _r = setup_3(c, seed, 0x18c198, 1);
                c.pc = 0x0018_c198;
                c.g[A0][0] = FIX as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_options_hud_on_widget_action_matches() {
        check(
            ee::s02::f_0018c2a0__generated,
            c_fe_state_options_hud_on_widget_action,
            120,
            &|c, seed| {
                let mut r = setup_3(c, seed, 0x18c2a0, 8);
                let this = FIX;
                let (eng, eng_vt, v1) = (FIX + 0x400, FIX + 0x800, FIX + 0xc00);
                let (widget, wvt) = (FIX + 0x1000, FIX + 0x1400);
                let (iface, ivt) = (FIX + 0x1800, FIX + 0x1c00);
                let hv = r.next_u32() | 0x1000_0000;
                let sub = seed / 10;
                c.pc = 0x0018_c2a0;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = if seed % 25 == 24 { 0 } else { widget as u64 };
                c.g[A2][0] = [2u64, 2, 2, 2, 6, 6, 7, 1, 0, 3][(seed % 10) as usize];
                // The engine's custom interface: its "previous state" method.
                w_2(c, this + 0x10, eng);
                w_2(c, eng, eng_vt);
                w_2(c, eng_vt + 4, v1);
                w16(c, v1 + 0x28, 0);
                w_2(c, v1 + 0x2c, VMETHOD);
                // The widget: show/hide slot, name hash, misc byte.
                w_2(c, widget + 8, wvt);
                w16(c, wvt + 0x48, 0);
                w_2(c, wvt + 0x4c, FAKE + 0x10);
                w_2(c, widget + 0x18, r.next_u32());
                w_2(c, widget + 0x38, hv);
                // The BE interface: a submit slot.
                w_2(c, iface + 0xc, ivt);
                w16(c, ivt + 8, 0);
                w_2(c, ivt + 0xc, FAKE + 0x20);
                queue(c, 0x0014_dd58, &[val(iface)]);
                // Which of the three name compares matches (or none).
                let mut h = [0u32; 3];
                if sub % 4 < 3 {
                    h[(sub % 4) as usize] = val(hv);
                }
                queue(c, 0x0031_7670, &h);
                queue(c, VMETHOD, &[if sub % 2 == 0 { NULL } else { val(1) }]);
                w_2(c, CFG, r.next_u32());
            },
        );
    }

    /// The gate word of this module's callee fakes (clear of the fake log at
    /// `DATA + 0x100000` and of the loadscreens fakes' gate).
    pub(crate) const CTRL_2: u32 = DATA + 0x2a_0000;

    pub(crate) const MAGIC_2: u32 = 0x4f43_3838;

    macro_rules! oc838_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(CTRL_2) != MAGIC_2 {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc736_fake_call(c, $addr);
                }
            )*
            fn oc838_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc838_fakes!(
        (
            oc838_fk_14dc80,
            OC838_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc838_fk_14dd58,
            OC838_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc838_fk_147618,
            OC838_PREV_147618,
            ee::s01::OVERRIDE_00147618,
            ee::s01::f_00147618__generated,
            0x0014_7618
        ),
        (
            oc838_fk_1474e8,
            OC838_PREV_1474E8,
            ee::s01::OVERRIDE_001474E8,
            ee::s01::f_001474e8__generated,
            0x0014_74e8
        ),
        (
            oc838_fk_145bd8,
            OC838_PREV_145BD8,
            ee::s01::OVERRIDE_00145BD8,
            ee::s01::f_00145bd8__generated,
            0x0014_5bd8
        ),
        (
            oc838_fk_397cc0,
            OC838_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc838_fk_39c870,
            OC838_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc838_fk_39a568,
            OC838_PREV_39A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039_a568
        ),
        (
            oc838_fk_39a768,
            OC838_PREV_39A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039_a768
        ),
        (
            oc838_fk_39a7a8,
            OC838_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039_a7a8
        ),
        (
            oc838_fk_39b960,
            OC838_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
    );

    pub(crate) static OC838_PREV_39D860: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake of `UIScreen_FindObject` (0x39d860): logs the call like the
    /// other fakes, then returns the fixture screen's object whose hash
    /// (`+0x38`) equals `a1`, or 0 (the real loop runs away on test data).
    pub(crate) fn oc838_fk_39d860(c: &mut Cpu) {
        if c.bus.read32(CTRL_2) != MAGIC_2 {
            return fakes::fall_back(c, &OC838_PREV_39D860, ee::s13::f_0039d860__generated);
        }
        let screen = c.g[A0][0] as u32;
        let hash = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        let count = c.bus.read32(screen.wrapping_add(0x3c)).min(16);
        let array = c.bus.read32(screen.wrapping_add(0x38));
        let mut found = 0u32;
        for i in 0..count {
            let obj = c.bus.read32(array.wrapping_add(4 * i));
            if obj != 0 && c.bus.read32(obj.wrapping_add(0x38)) == hash {
                found = obj;
                break;
            }
        }
        fakes::fake_call(c, 0x0039_d860);
        if c.pc == ra {
            c.g[V0][0] = found as u64;
        }
    }

    /// The screen of the controller / HUD under test.
    pub(crate) const SCR: u32 = DATA + 0x4_0000;

    /// The state object under test.
    pub(crate) const THIS: u32 = DATA + 0x100;

    /// A widget with a vtable whose methods are fakes (the "item" of the HUD).
    pub(crate) const ITEM: u32 = DATA + 0x6_0000;

    /// A real leaf that returns 0.
    pub(crate) const ZERO_LEAF: u32 = 0x0014_06c0;

    /// The four list box names, and the names `ShowConfigText` looks up.
    pub(crate) const LB_NAMES: [u32; 4] = [0x004a_1568, 0x004a_1570, 0x004a_15c8, 0x004a_1578];

    pub(crate) const CFG_NAMES: [u32; 4] = [0x004a_15bc, 0x004a_15c0, 0x0045_e0a0, 0x0045_e0b0];

    pub(crate) fn str_hash_3(c: &mut Cpu, s: u32) -> u32 {
        run_8(c, 0x0031_7670, ee::s10::f_00317670, &[s as u64]) as u32
    }

    /// Machine with the callee fakes gated on, and the fixture's per-call
    /// unwind index.
    pub(crate) fn fixture_2(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc838_install_fakes();
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc838_fk_39d860,
            &OC838_PREV_39D860,
        );
        let r = machine(c, seed, salt, calls);
        c.bus.write32(CTRL_2, MAGIC_2);
        r
    }

    /// A screen at `SCR` with one object per hash (count at `+0x3c`, pointer
    /// array at `+0x38`, hash at `object + 0x38`), all sharing one vtable whose
    /// methods at `+0x4c`, `+0xbc` and `+0x74` are `fns` (deltas 0).
    pub(crate) fn screen(c: &mut Cpu, hashes: &[u32], fns: [u32; 3]) {
        let array = SCR + 0x100;
        let vt = SCR + 0x1000;
        c.bus.write32(SCR + 0x3c, hashes.len() as u32);
        c.bus.write32(SCR + 0x38, array);
        for (i, &h) in hashes.iter().enumerate() {
            let obj = SCR + 0x200 + 0x100 * i as u32;
            c.bus.write32(array + 4 * i as u32, obj);
            c.bus.write32(obj + 0x38, h);
            c.bus.write32(obj + 8, vt);
            c.bus.write32(obj + 0x14, 0x5);
            c.bus.write8(obj + 0x95, i as u8);
        }
        for (off, f) in [(0x4cu32, fns[0]), (0xbc, fns[1]), (0x74, fns[2])] {
            c.bus.write16(vt + off - 4, 0);
            c.bus.write32(vt + off, f);
        }
        c.bus.write16(vt + 0x70, 0);
    }

    /// The names of `mask`'s set bits, hashed by the real hash function.
    pub(crate) fn present(c: &mut Cpu, names: &[u32], mask: u32) -> Vec<u32> {
        let mut v = Vec::new();
        for (i, &n) in names.iter().enumerate() {
            if mask >> i & 1 != 0 {
                v.push(str_hash_3(c, n));
            }
        }
        v
    }

    /// `gp-0x1b34` / `gp-0x1b30`: the pointers to the picture names.
    pub(crate) fn config_names(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1b34), CFG_NAMES[0]);
        c.bus.write32(GAME_GP.wrapping_sub(0x1b30), CFG_NAMES[1]);
    }

    /// Which vtable method a seed uses: a fake, or a real leaf returning 0.
    pub(crate) fn vfn(seed: u32, shift: u32) -> u32 {
        if (seed >> shift) % 3 == 0 {
            ZERO_LEAF
        } else {
            FAKE + 0x10
        }
    }

    /// An item (the HUD row) the fakes' forced results hand out.
    pub(crate) fn item(c: &mut Cpu) {
        c.bus.write32(ITEM + 8, ITEM + 0x400);
        c.bus.write16(ITEM + 0x400 + 0x48, 0);
        c.bus.write32(ITEM + 0x400 + 0x4c, FAKE + 0x20);
        for n in 2..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * n, ITEM);
        }
    }

    /// `cFEStateOptionsHUD_RefreshSelection`: the "Menu" widget missing or
    /// present, every option mode 0..2 (the row index matches for one row), the
    /// row lookup returning a fake item or nothing (the real leaf).
    #[test]
    fn options_hud_refresh_selection_matches() {
        check(
            ee::s02::f_0018c478__generated,
            oc838_options_hud_refresh_selection,
            90,
            &|c, seed| {
                let mut r = fixture_2(c, seed, 0x18c478, 8);
                item(c);
                let menu = str_hash_3(c, 0x004a_1398);
                let hashes = match seed % 5 {
                    0 => vec![],
                    1 => vec![menu ^ 1],
                    _ => vec![r.next_u32(), menu],
                };
                screen(c, &hashes, [FAKE + 0x10, FAKE + 0x20, vfn(seed, 2)]);
                c.bus.write32(THIS + 0x40, SCR);
                // The option block: mode in the top two bits of its first word.
                for i in 0..0xa4u32 {
                    c.bus.write32(0x0053_5610 + 4 * i, r.next_u32());
                }
                c.bus
                    .write32(0x0053_5610, (seed % 3) << 30 | r.next_u32() >> 2);
                c.pc = 0x0018_c478;
                c.g[A0][0] = THIS as u64;
            },
        );
    }

    /// `cFEStateOptionsController_Init`: the screen creation failing or not
    /// (a forced fake result), each subset of the four list boxes found.
    #[test]
    fn options_controller_init_matches() {
        check(
            ee::s02::f_0018c6a0__generated,
            oc838_options_controller_init,
            160,
            &|c, seed| {
                let mut r = fixture_2(c, seed, 0x18c6a0, 14);
                if seed % 7 == 0 {
                    oc736_force_zero(c, 0x0039_7cc0);
                } else {
                    crate::game::main::loadscreens::tests::oc736_force(c, 0x0039_7cc0, SCR);
                }
                if seed % 3 == 1 {
                    oc736_force_zero(c, 0x0014_7618);
                }
                let hashes = present(c, &LB_NAMES, seed >> 1);
                screen(c, &hashes, [FAKE + 0x10, FAKE + 0x20, FAKE + 0x30]);
                c.bus.write32(THIS + 0x10, DATA + 0x200);
                c.bus.write32(THIS + 0x48, r.next_u32());
                c.bus.write32(THIS + 0x4c, r.next_u32());
                c.pc = 0x0018_c6a0;
                c.g[A0][0] = THIS as u64;
            },
        );
    }

    /// `cFEStateOptionsController_OnChildEvent`: events 0x15, 0x16 (the
    /// "resetopts" child or another, `getData(2)` zero or not) and others.
    #[test]
    fn options_controller_on_child_event_matches() {
        check(
            ee::s02::f_0018c910__generated,
            oc838_options_controller_on_child_event,
            120,
            &|c, seed| {
                let mut r = fixture_2(c, seed, 0x18c910, 6);
                let reset = str_hash_3(c, 0x0045_dd20);
                let hashes = present(c, &LB_NAMES, seed >> 3);
                screen(c, &hashes, [FAKE + 0x10, FAKE + 0x20, FAKE + 0x30]);
                c.bus.write32(THIS + 0x40, SCR);
                let child = DATA + 0x800;
                let vt = DATA + 0x900;
                c.bus.write32(child + 8, vt);
                c.bus.write16(vt + 0xb8, 0);
                c.bus.write32(
                    vt + 0xbc,
                    if seed % 4 == 0 {
                        ZERO_LEAF
                    } else {
                        FAKE + 0x40
                    },
                );
                c.bus.write32(
                    child + 0xc,
                    if seed % 3 == 0 { r.next_u32() } else { reset },
                );
                let event = [0x15u32, 0x16, 0x16, 0x16, 0x17, 0x0][(seed % 6) as usize];
                c.pc = 0x0018_c910;
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = child as u64;
                c.g[A2][0] = event as u64;
            },
        );
    }

    /// `cFEStateOptionsController_OnWidgetNotify`: keys 5 (menu rows 2, 3 and
    /// others; the controller mapping zero or not), 6, 9, the small keys and
    /// the large ones.
    #[test]
    fn options_controller_on_widget_notify_matches() {
        check(
            ee::s02::f_0018cce0__generated,
            oc838_options_controller_on_widget_notify,
            160,
            &|c, seed| {
                let _r = fixture_2(c, seed, 0x18cce0, 6);
                if seed % 2 == 1 {
                    oc736_force_zero(c, 0x0014_7618);
                }
                config_names(c);
                let hashes = present(c, &CFG_NAMES, seed >> 2);
                screen(c, &hashes, [FAKE + 0x10, FAKE + 0x20, FAKE + 0x30]);
                c.bus.write32(THIS + 0x40, SCR);
                let menu = DATA + 0x800;
                c.bus
                    .write8(menu + 0x95, [2u8, 3, 0, 4, 3, 2][((seed / 2) % 6) as usize]);
                let key = [5u32, 5, 5, 6, 9, 0, 1, 3, 4, 7, 8, 0x101][(seed % 12) as usize];
                c.pc = 0x0018_cce0;
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = menu as u64;
                c.g[A2][0] = key as u64;
            },
        );
    }

    /// `cFEStateOptionsController_RefreshListBoxes`: each subset of the four
    /// list boxes present, the pad mapping zero or not.
    #[test]
    fn options_controller_refresh_list_boxes_matches() {
        check(
            ee::s02::f_0018cdc0__generated,
            oc838_options_controller_refresh_list_boxes,
            160,
            &|c, seed| {
                let _r = fixture_2(c, seed, 0x18cdc0, 10);
                if seed % 3 == 1 {
                    oc736_force_zero(c, 0x0014_7618);
                }
                config_names(c);
                let mut hashes = present(c, &LB_NAMES, seed);
                hashes.extend(present(c, &CFG_NAMES, seed >> 4));
                screen(c, &hashes, [FAKE + 0x10, FAKE + 0x20, FAKE + 0x30]);
                c.bus.write32(THIS + 0x40, SCR);
                c.pc = 0x0018_cdc0;
                c.g[A0][0] = THIS as u64;
            },
        );
    }

    /// `cFEStateOptionsController_ShowConfigText`: each subset of the four
    /// widgets present, `on` 0, 1 and other values.
    #[test]
    fn options_controller_show_config_text_matches() {
        check(
            ee::s02::f_0018cf50__generated,
            oc838_options_controller_show_config_text,
            160,
            &|c, seed| {
                let mut r = fixture_2(c, seed, 0x18cf50, 5);
                config_names(c);
                let hashes = present(c, &CFG_NAMES, seed);
                screen(c, &hashes, [vfn(seed, 4), FAKE + 0x20, FAKE + 0x30]);
                c.bus.write32(THIS + 0x40, SCR);
                let on = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32(),
                };
                c.pc = 0x0018_cf50;
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = on as u64;
            },
        );
    }

    /// Gate word: the fakes below log and return only when it holds the magic.
    pub(crate) const OC839_CTRL: u32 = DATA + 0xe_0000;

    pub(crate) const OC839_MAGIC: u32 = 0x0c83_9a11;

    /// Word after the gate: what the screen-creation fake returns.
    pub(crate) const OC839_SCREEN: u32 = OC839_CTRL + 4;

    pub(crate) static OC839_PREV_ADD: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC839_PREV_PLAY: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC839_PREV_FIND: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC839_PREV_ASCII: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC839_PREV_UNICODE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC839_PREV_SPRINTF: AtomicUsize = AtomicUsize::new(0);

    /// The object every `UIScreen_FindObject` returns, with a vtable whose
    /// method slots point at the call fake.
    pub(crate) const OC839_OBJ: u32 = DATA + 0x3000;

    pub(crate) const OC839_VT: u32 = DATA + 0x3100;

    pub(crate) fn oc839_gated(
        c: &mut Cpu,
        target: u32,
        prev: &AtomicUsize,
        generated: GuestFn,
    ) -> bool {
        if c.bus.read32(OC839_CTRL) != OC839_MAGIC {
            fakes::fall_back(c, prev, generated);
            return false;
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        true
    }

    pub(crate) fn oc839_fake_add(c: &mut Cpu) {
        if oc839_gated(
            c,
            0x0039_7cc0,
            &OC839_PREV_ADD,
            ee::s13::f_00397cc0__generated,
        ) && c.pc == c.g[RA][0] as u32
        {
            c.g[V0][0] = c.bus.read32(OC839_SCREEN) as u64;
        }
    }

    pub(crate) fn oc839_fake_play(c: &mut Cpu) {
        oc839_gated(
            c,
            0x0039_c870,
            &OC839_PREV_PLAY,
            ee::s13::f_0039c870__generated,
        );
    }

    pub(crate) fn oc839_fake_find(c: &mut Cpu) {
        let ra = c.g[RA][0] as u32;
        if oc839_gated(
            c,
            0x0039_d860,
            &OC839_PREV_FIND,
            ee::s13::f_0039d860__generated,
        ) && c.pc == ra
        {
            c.g[V0][0] = OC839_OBJ as u64;
        }
    }

    pub(crate) fn oc839_fake_ascii(c: &mut Cpu) {
        oc839_gated(
            c,
            0x003a_0c50,
            &OC839_PREV_ASCII,
            ee::s13::f_003a0c50__generated,
        );
    }

    pub(crate) fn oc839_fake_unicode(c: &mut Cpu) {
        oc839_gated(
            c,
            0x003a_0e90,
            &OC839_PREV_UNICODE,
            ee::s13::f_003a0e90__generated,
        );
    }

    pub(crate) fn oc839_fake_sprintf(c: &mut Cpu) {
        oc839_gated(
            c,
            0x0041_7828,
            &OC839_PREV_SPRINTF,
            ee::s15::f_00417828__generated,
        );
    }

    pub(crate) fn oc839_install() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397CC0),
            oc839_fake_add,
            &OC839_PREV_ADD,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc839_fake_play,
            &OC839_PREV_PLAY,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc839_fake_find,
            &OC839_PREV_FIND,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C50),
            oc839_fake_ascii,
            &OC839_PREV_ASCII,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0E90),
            oc839_fake_unicode,
            &OC839_PREV_UNICODE,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc839_fake_sprintf,
            &OC839_PREV_SPRINTF,
        );
    }

    /// The memory-card device-select screen: `self` at DATA+0x400 with the
    /// type (+0x1bc: 1 = load replay) and save flag (+0x22c); the world's
    /// string table (`gp-0x848` -> +0x8c -> +4 -> method at +0x24 = fake) and
    /// a UI object whose vtable slots +0x44/+0x4c/+0xcc are fakes. Seeds pick
    /// the null-screen exit and the three branches.
    #[test]
    fn c_fe_state_options_device_select_init_matches() {
        check(
            ee::s02::f_0018d868__generated,
            c_fe_state_options_device_select_init,
            90,
            &|c, seed| {
                oc839_install();
                let _ = machine(c, seed, 0x18_d868, 40);
                c.bus.write32(OC839_CTRL, OC839_MAGIC);
                c.bus
                    .write32(OC839_SCREEN, if seed % 5 == 0 { 0 } else { DATA + 0x5000 });
                let this = DATA + 0x400;
                c.bus.write32(this + 0x10, DATA + 0x1000);
                let kind = match seed % 3 {
                    0 => 1,
                    1 => 0,
                    _ => 2,
                };
                c.bus.write32(this + 0x1bc, kind);
                c.bus
                    .write32(this + 0x22c, if (seed / 3) % 2 == 0 { 0 } else { 1 });
                // World -> string table -> entry whose method is the fake.
                let world = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x8c, DATA + 0x2100);
                c.bus.write32(DATA + 0x2104, DATA + 0x2200);
                c.bus.write16(DATA + 0x2220, 0x10);
                c.bus.write32(DATA + 0x2224, fakes::FAKE);
                // The UI object.
                c.bus.write32(OC839_OBJ + 8, OC839_VT);
                for off in [0x40u32, 0x48, 0xc8] {
                    c.bus.write16(OC839_VT + off, 0);
                    c.bus.write32(OC839_VT + off + 4, fakes::FAKE);
                }
                // An empty sprintf buffer on the stack (the fake writes nothing).
                for i in 0..8 {
                    c.bus.write32(SP - 0xa0 + 4 * i, 0);
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// Gate word of this module's fakes (a machine without it runs the
    /// callees' real bodies) and its magic.
    pub(crate) const OC840_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC840_MAGIC: u32 = 0x0c84_0a49;

    /// A real leaf that returns 0 (a virtual call that answers 0).
    pub(crate) const OC840_LEAF_ZERO: u32 = 0x0014_06c0;

    // Fixture layout (above the area the fakes' result pointers cover).
    pub(crate) const OC840_SELF: u32 = DATA + 0x4_0000;

    pub(crate) const OC840_WIDGET: u32 = DATA + 0x4_1000;

    pub(crate) const OC840_ENGINE: u32 = DATA + 0x4_2000;

    pub(crate) const OC840_CUSTOM: u32 = DATA + 0x4_3000;

    pub(crate) const OC840_VT_CUSTOM: u32 = DATA + 0x4_4000;

    pub(crate) const OC840_WORLD: u32 = DATA + 0x4_5000;

    pub(crate) const OC840_SUB: u32 = DATA + 0x4_6000;

    pub(crate) const OC840_VT_SUB: u32 = DATA + 0x4_7000;

    pub(crate) const OC840_MGR: u32 = DATA + 0x4_8000;

    pub(crate) const OC840_OBJ434: u32 = DATA + 0x4_9000;

    pub(crate) const OC840_VT434: u32 = DATA + 0x4_a000;

    pub(crate) const OC840_OBJ748: u32 = DATA + 0x4_b000;

    pub(crate) const OC840_VT_SELF8: u32 = DATA + 0x4_c000;

    pub(crate) const OC840_OBJ: u32 = DATA + 0x4_d000;

    pub(crate) const OC840_VT_OBJ: u32 = DATA + 0x4_e000;

    pub(crate) const OC840_VT_X: u32 = DATA + 0x4_f000;

    pub(crate) const OC840_PAD: u32 = DATA + 0x5_0000;

    pub(crate) const OC840_DEV: u32 = DATA + 0x5_8000;

    pub(crate) const OC840_VT_DEV: u32 = DATA + 0x5_9000;

    /// Mailbox of the fake for `addr`: 0 = the fake's own pointer, else the
    /// call returns `mail - 1`.
    pub(crate) fn oc840_mail(addr: u32) -> u32 {
        OC840_CTRL + 0x100 + (addr & 0x3f_fffc)
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// the mailbox value (minus 1) unless the call unwound.
    pub(crate) fn oc840_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc840_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 1) as u64;
        }
    }

    macro_rules! oc840_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC840_CTRL) != OC840_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc840_fake_call(c, $addr);
                }
            )*
            fn oc840_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc840_fakes!(
        (
            oc840_fake_14dc80,
            OC840_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc840_fake_14dd58,
            OC840_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc840_fake_147138,
            OC840_PREV_147138,
            ee::s01::OVERRIDE_00147138,
            ee::s01::f_00147138__generated,
            0x0014_7138
        ),
        (
            oc840_fake_14de28,
            OC840_PREV_14DE28,
            ee::s01::OVERRIDE_0014DE28,
            ee::s01::f_0014de28__generated,
            0x0014_de28
        ),
        (
            oc840_fake_1a0548,
            OC840_PREV_1A0548,
            ee::s03::OVERRIDE_001A0548,
            ee::s03::f_001a0548__generated,
            0x001a_0548
        ),
        (
            oc840_fake_19eba0,
            OC840_PREV_19EBA0,
            ee::s03::OVERRIDE_0019EBA0,
            ee::s03::f_0019eba0__generated,
            0x0019_eba0
        ),
        (
            oc840_fake_19e960,
            OC840_PREV_19E960,
            ee::s03::OVERRIDE_0019E960,
            ee::s03::f_0019e960__generated,
            0x0019_e960
        ),
        (
            oc840_fake_39f190,
            OC840_PREV_39F190,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039_f190
        ),
        (
            oc840_fake_39f400,
            OC840_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc840_fake_39c778,
            OC840_PREV_39C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778
        ),
        (
            oc840_fake_39c870,
            OC840_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc840_fake_39d860,
            OC840_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc840_fake_397840,
            OC840_PREV_397840,
            ee::s13::OVERRIDE_00397840,
            ee::s13::f_00397840__generated,
            0x0039_7840
        ),
        (
            oc840_fake_3978f0,
            OC840_PREV_3978F0,
            ee::s13::OVERRIDE_003978F0,
            ee::s13::f_003978f0__generated,
            0x0039_78f0
        ),
        (
            oc840_fake_317d70,
            OC840_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc840_fake_1d99c8,
            OC840_PREV_1D99C8,
            ee::s04::OVERRIDE_001D99C8,
            ee::s04::f_001d99c8__generated,
            0x001d_99c8
        ),
        (
            oc840_fake_227f80,
            OC840_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80
        ),
        (
            oc840_fake_23cae8,
            OC840_PREV_23CAE8,
            ee::s06::OVERRIDE_0023CAE8,
            ee::s06::f_0023cae8__generated,
            0x0023_cae8
        ),
        (
            oc840_fake_23c8f0,
            OC840_PREV_23C8F0,
            ee::s06::OVERRIDE_0023C8F0,
            ee::s06::f_0023c8f0__generated,
            0x0023_c8f0
        ),
        (
            oc840_fake_23d570,
            OC840_PREV_23D570,
            ee::s06::OVERRIDE_0023D570,
            ee::s06::f_0023d570__generated,
            0x0023_d570
        ),
    );

    pub(crate) fn oc840_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    pub(crate) fn oc840_gp(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// A scrambled machine with this module's fakes installed and switched on.
    pub(crate) fn oc840_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc840_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC840_CTRL, OC840_MAGIC);
        r
    }

    /// A vtable entry `{ delta, func }` at `vt + off`: a call through it
    /// reaches fake number `k`, or (`leaf`) a real leaf that returns 0.
    pub(crate) fn oc840_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, k: u32, leaf: bool) {
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt + off, delta as u16);
        let func = if leaf {
            OC840_LEAF_ZERO
        } else {
            FAKE + 0x10 * k
        };
        c.bus.write32(vt + off + 4, func);
    }

    /// `StrHash` of the string at `addr`, by the generated hasher.
    pub(crate) fn oc840_hash_of(addr: u32) -> u32 {
        let mut c = crate::difftest::fresh();
        c.g[A0][0] = addr as u64;
        c.g[RA][0] = 0x0012_3450;
        c.g[GP][0] = GAME_GP as u64;
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(&mut c);
        c.g[V0][0] as u32
    }

    /// The options-state object `OC840_SELF`: engine and custom interface
    /// (vtable slots `+0x20/+0x24` next state, `+0x28/+0x2c` previous
    /// state), the screen/rider bytes and the world.
    pub(crate) fn oc840_state_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, leaf: bool) {
        oc840_w(c, OC840_SELF + 0x10, OC840_ENGINE);
        oc840_w(c, OC840_ENGINE, OC840_CUSTOM);
        oc840_w(c, OC840_CUSTOM + 4, OC840_VT_CUSTOM);
        oc840_slot(c, r, OC840_VT_CUSTOM, 0x20, 1, leaf);
        oc840_slot(c, r, OC840_VT_CUSTOM, 0x28, 2, leaf);
        oc840_w(c, OC840_SELF + 0x40, r.next_u32());
        c.bus
            .write8(OC840_SELF + 0x44, [0u8, 2, 0xff, 0x80][(seed % 4) as usize]);
        c.bus.write8(OC840_SELF + 0x48, (seed % 4) as u8);
        c.bus.write8(OC840_SELF + 0x14, ((seed / 4) % 4) as u8);
        oc840_w(c, oc840_gp(-0x848), OC840_WORLD);
        oc840_w(c, OC840_WORLD + 0x7c, r.next_u32());
        oc840_w(c, OC840_WORLD + 0x8c, OC840_SUB);
        oc840_w(c, OC840_SUB + 4, OC840_VT_SUB);
        oc840_slot(c, r, OC840_VT_SUB, 0x20, 3, false);
    }

    /// A UI object `OC840_OBJ` that the fakes of `0x14dd58`, `0x1d99c8` and
    /// `0x39d860` hand out: virtual slots `+0x48`, `+0xc8` and an embedded
    /// `{delta, func}` at `+0xc -> +8`.
    pub(crate) fn oc840_obj_fixture(c: &mut Cpu, r: &mut Rng) {
        for addr in [0x0014_dd58u32, 0x001d_99c8, 0x0039_d860] {
            oc840_w(c, oc840_mail(addr), OC840_OBJ + 1);
        }
        oc840_w(c, OC840_OBJ + 8, OC840_VT_OBJ);
        oc840_w(c, OC840_OBJ + 0xc, OC840_VT_X);
        oc840_slot(c, r, OC840_VT_OBJ, 0x48, 4, false);
        oc840_slot(c, r, OC840_VT_OBJ, 0xc8, 5, false);
        oc840_slot(c, r, OC840_VT_X, 8, 6, false);
    }

    /// Float bits of the sliders `SetVibration` rescales.
    #[test]
    fn c_fe_state_options_controller_set_vibration_matches() {
        check(
            ee::s02::f_0018d108__generated,
            c_fe_state_options_controller_set_vibration,
            160,
            &|c, seed| {
                let mut r = oc840_base(c, seed, 0x18_d108, 3);
                let idx = [0u64, 1, 2, 3, 0xfe][(seed % 5) as usize];
                c.g[A0][0] = OC840_SELF as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 8) | idx;
                let xs = [0.0f32, 5.0, 20.0, 50.0, 100.0, 255.0, 1000.0, -3.0];
                let ys = [0.0f32, 0.5, 10.0, 60.0, 70.0, 100.0, 200.0];
                c.f[12] = xs[(seed % 8) as usize].to_bits();
                c.f[13] = ys[((seed / 8) % 7) as usize].to_bits();
                let no_pads = seed % 9 == 8;
                oc840_w(c, oc840_gp(-0x850), if no_pads { 0 } else { OC840_PAD });
                if no_pads {
                    // A null pad cache: the device read at address 0 is a
                    // zero vtable.
                    for a in (0..0x40u32).step_by(4) {
                        oc840_w(c, a, 0);
                    }
                }
                for i in -2i32..=3 {
                    oc840_w(c, OC840_PAD + 0x2eec + (i * 4) as u32, OC840_DEV);
                }
                oc840_w(c, OC840_DEV, OC840_VT_DEV);
                oc840_slot(c, &mut r, OC840_VT_DEV, 0x18, 0, false);
                oc840_slot(c, &mut r, OC840_VT_DEV, 0x20, 1, false);
                // numActuators(): 2 (both motors) or something else.
                c.bus
                    .write32(LOG_RESULTS, if seed % 3 == 2 { 3 } else { 2 });
            },
        );
    }

    #[test]
    fn c_fe_state_options_save_load_on_widget_created_matches() {
        check(
            ee::s02::f_0018d2e8__generated,
            c_fe_state_options_save_load_on_widget_created,
            64,
            &|c, seed| {
                let mut r = oc840_base(c, seed, 0x18_d2e8, 0);
                let ids = [
                    0x0045_e0d0u32,
                    0x0045_e0e0,
                    0x0045_e0f0,
                    0x0045_e100,
                    0x004a_1508,
                    0x004a_1500,
                ];
                let k = (seed % 8) as usize;
                let hash = if k < 6 {
                    oc840_hash_of(ids[k])
                } else {
                    r.next_u32()
                };
                c.g[A0][0] = OC840_SELF as u64;
                c.g[A1][0] = OC840_WIDGET as u64;
                oc840_w(c, OC840_WIDGET + 0x38, hash);
                oc840_w(c, OC840_WIDGET + 0x18, r.next_u32());
            },
        );
    }

    /// Event 0xf / 0x10 pop the state, 0x16 starts the new-profile chain.
    #[test]
    fn c_fe_state_options_save_load_on_child_event_matches() {
        check(
            ee::s02::f_0018d460__generated,
            c_fe_state_options_save_load_on_child_event,
            240,
            &|c, seed| {
                let mut r = oc840_base(c, seed, 0x18_d460, 20);
                oc840_state_fixture(c, &mut r, seed, seed % 5 == 4);
                oc840_obj_fixture(c, &mut r);
                let ev = [0xfu64, 0x10, 0x15, 0x16, 0x16, 0x16, 0x14, 0x30][(seed % 8) as usize];
                c.g[A0][0] = OC840_SELF as u64;
                c.g[A1][0] = OC840_WIDGET as u64;
                c.g[A2][0] = ev;
                let flag = if seed % 3 == 0 { 0 } else { r.next_u32() | 1 };
                oc840_w(c, OC840_WIDGET + 0x18, flag);
                // GetFrameByLabel: "no such frame" (0xffff) or a frame.
                if seed % 2 == 0 {
                    oc840_w(c, oc840_mail(0x0039_c778), 0x1_0000);
                }
            },
        );
    }

    /// Select (5) on each menu item, back (6), a null widget, other events.
    #[test]
    fn c_fe_state_options_save_load_on_widget_action_matches() {
        check(
            ee::s02::f_0018d648__generated,
            c_fe_state_options_save_load_on_widget_action,
            300,
            &|c, seed| {
                let mut r = oc840_base(c, seed, 0x18_d648, 14);
                oc840_state_fixture(c, &mut r, seed, seed % 5 == 4);
                oc840_obj_fixture(c, &mut r);
                let ev = [5u64, 5, 5, 5, 6, 6, 0, 5][(seed % 8) as usize];
                let item = [3u32, 3, 0, 1, 2, 4, 5, 0x300, 0x200][((seed / 8) % 9) as usize];
                c.g[A0][0] = OC840_SELF as u64;
                c.g[A1][0] = if seed % 13 == 12 {
                    0
                } else {
                    OC840_WIDGET as u64
                };
                c.g[A2][0] = ev;
                oc840_w(c, OC840_WIDGET + 0x18, item);
                // The popup push: the list has a head (UIList_First != 0) or not.
                if seed % 2 == 1 {
                    oc840_w(c, oc840_mail(0x0039_7840), 1);
                }
            },
        );
    }

    /// The memory-card device screen's buttons: select (5), back (6), X (7).
    #[test]
    fn c_fe_state_options_device_select_on_widget_action_matches() {
        check(
            ee::s02::f_0018dea0__generated,
            c_fe_state_options_device_select_on_widget_action,
            400,
            &|c, seed| {
                let mut r = oc840_base(c, seed, 0x18_dea0, 8);
                let pick = |r: &mut Rng, n: u32| r.next_u32() % n;
                let s = OC840_SELF;
                oc840_w(c, s + 8, OC840_VT_SELF8);
                oc840_slot(c, &mut r, OC840_VT_SELF8, 0x160, 1, false);
                oc840_slot(c, &mut r, OC840_VT_SELF8, 0x178, 2, false);
                let v = [0u32, 0, 1, 2][pick(&mut r, 4) as usize];
                oc840_w(c, s + 0x1a8, v);
                let v = [2u32, 2, 0, 1][pick(&mut r, 4) as usize];
                oc840_w(c, s + 0x1bc, v);
                let v = [0u32, 1, 2, 3, 6, 4][pick(&mut r, 6) as usize];
                oc840_w(c, s + 0x1c0, v);
                let v = [3u32, 3, 3, 1][pick(&mut r, 4) as usize];
                oc840_w(c, s + 0x214, v);
                let v = [0u32, 1, 1, 1][pick(&mut r, 4) as usize];
                oc840_w(c, s + 0x224, v);
                let v = [0u32, 1, 1][pick(&mut r, 3) as usize];
                oc840_w(c, s + 0x22c, v);
                oc840_w(c, OC840_WIDGET + 0x18, r.next_u32() & 7);
                // The memory-card overlay manager every call returns.
                oc840_w(c, oc840_gp(-0x848), OC840_WORLD);
                oc840_w(c, oc840_mail(0x0022_7f80), OC840_MGR + 1);
                oc840_w(c, OC840_MGR + 0x434, OC840_OBJ434);
                oc840_w(c, OC840_MGR + 0x428, r.next_u32());
                oc840_w(c, OC840_OBJ434, OC840_VT434);
                let leaf = seed % 2 == 0;
                oc840_slot(c, &mut r, OC840_VT434, 0x218, 3, leaf);
                let leaf = seed % 3 == 0;
                oc840_slot(c, &mut r, OC840_VT434, 0x1b0, 4, leaf);
                oc840_w(c, OC840_MGR + 0x748, OC840_OBJ748);
                oc840_slot(c, &mut r, OC840_OBJ748, 8, 5, false);
                let ev = [5u64, 5, 5, 5, 6, 7, 7, 7, 3][(seed % 9) as usize];
                c.g[A0][0] = s as u64;
                c.g[A1][0] = if ev == 5 && seed % 11 == 0 {
                    0
                } else {
                    OC840_WIDGET as u64
                };
                c.g[A2][0] = ev;
            },
        );
    }

    #[test]
    fn c_fe_state_options_device_select_v39_matches() {
        check(
            ee::s02::f_0018e368__generated,
            c_fe_state_options_device_select_v39,
            64,
            &|c, seed| {
                let mut r = oc840_base(c, seed, 0x18_e368, 3);
                oc840_obj_fixture(c, &mut r);
                c.g[A0][0] = OC840_SELF as u64;
                c.g[A1][0] = [0u64, 1, 5][(seed % 3) as usize];
                c.g[A2][0] = [1u64, 0, 7, 0][((seed / 3) % 4) as usize];
                oc840_w(c, OC840_SELF + 0x40, r.next_u32());
                oc840_w(c, OC840_SELF + 0x22c, (seed % 2) as u32);
                oc840_w(c, OC840_SELF + 0x224, r.next_u32());
            },
        );
    }

    /// Gate word and magic of this module's fakes; the seed word and the two
    /// "no next device" flags (left, right) follow it.
    pub(crate) const OC841_CTRL: u32 = DATA + 0x2e_0000;

    pub(crate) const OC841_MAGIC: u32 = 0x4f43_3834;

    /// The screen, the memory card manager, the card object and one text.
    pub(crate) const OC841_SELF: u32 = DATA + 0x1000;

    pub(crate) const OC841_MC: u32 = DATA + 0x2000;

    pub(crate) const OC841_CARD: u32 = DATA + 0x3000;

    pub(crate) const OC841_TEXT: u32 = DATA + 0x4000;

    pub(crate) const OC841_VT_SELF: u32 = DATA + 0x5000;

    pub(crate) const OC841_VT_CARD: u32 = DATA + 0x6000;

    pub(crate) const OC841_VT_TEXT: u32 = DATA + 0x7000;

    macro_rules! oc841_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $post:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC841_CTRL) != OC841_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    let a3 = c.g[A3][0];
                    let ra = c.g[RA][0] as u32;
                    fakes::fake_call(c, $addr);
                    if c.pc == ra {
                        let post: fn(&mut Cpu, u64) = $post;
                        post(c, a3);
                    }
                }
            )*
            pub(crate) fn oc841_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    pub(crate) fn oc841_none(_: &mut Cpu, _: u64) {}

    pub(crate) fn oc841_post_mc(c: &mut Cpu, _: u64) {
        c.g[V0][0] = OC841_MC as u64;
    }

    pub(crate) fn oc841_post_text(c: &mut Cpu, _: u64) {
        c.g[V0][0] = OC841_TEXT as u64;
    }

    /// `checkNextAvailableDevice(.., a3)`: -1 (none) when the seed says so.
    pub(crate) fn oc841_post_next(c: &mut Cpu, a3: u64) {
        if c.bus.read32(OC841_CTRL + 8 + 4 * (a3 as u32 & 1)) != 0 {
            c.g[V0][0] = u64::MAX;
        }
    }

    oc841_fakes!(
        (
            oc841_fake_227f80,
            OC841_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80,
            oc841_post_mc
        ),
        (
            oc841_fake_1d8240,
            OC841_PREV_1D8240,
            ee::s04::OVERRIDE_001D8240,
            ee::s04::f_001d8240__generated,
            0x001d_8240,
            oc841_none
        ),
        (
            oc841_fake_241710,
            OC841_PREV_241710,
            ee::s06::OVERRIDE_00241710,
            ee::s06::f_00241710__generated,
            0x0024_1710,
            oc841_none
        ),
        (
            oc841_fake_317670,
            OC841_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            oc841_none
        ),
        (
            oc841_fake_39d860,
            OC841_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            oc841_post_text
        ),
        (
            oc841_fake_3a0d00,
            OC841_PREV_3A0D00,
            ee::s13::OVERRIDE_003A0D00,
            ee::s13::f_003a0d00__generated,
            0x003a_0d00,
            oc841_none
        ),
        (
            oc841_fake_417828,
            OC841_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828,
            oc841_none
        ),
        (
            oc841_fake_3a0e90,
            OC841_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90,
            oc841_none
        ),
        (
            oc841_fake_1d97e0,
            OC841_PREV_1D97E0,
            ee::s04::OVERRIDE_001D97E0,
            ee::s04::f_001d97e0__generated,
            0x001d_97e0,
            oc841_post_next
        ),
    );

    /// Virtual methods (fake addresses): the answers of the "is it so"
    /// methods come from a hash of the call, so each seed walks other branches.
    pub(crate) const OC841_DISPLAY_FILE_OK: u32 = fakes::FAKE + 0x10;

    pub(crate) const OC841_IS_AVAILABLE: u32 = fakes::FAKE + 0x40;

    pub(crate) fn oc841_miss(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT) as u64;
        let seed = c.bus.read32(OC841_CTRL + 4) as u64;
        let h = fakes::mix(target as u64 ^ c.g[A1][0] << 8 ^ seed << 32 ^ n << 20);
        fakes::fake_call(c, target);
        let mostly = target == OC841_DISPLAY_FILE_OK || target == OC841_IS_AVAILABLE;
        let yes = if mostly { h & 7 != 0 } else { h & 1 != 0 };
        c.g[V0][0] = yes as u64;
    }

    pub(crate) fn oc841_slot(c: &mut Cpu, r: &mut crate::difftest::Rng, vt: u32, off: u32, k: u32) {
        let delta = (r.next_u32() % 8) * 4;
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * k);
    }

    /// Reference: `cFEStateOptionsDeviceSelect::displayStatus`: both
    /// `displayFileOK` answers, a card present or not, screen state 3 or not,
    /// screen types 1 / 0 / 2 (all four slots or just the first), the slot
    /// flag branches (device flags and save screen), arrows with and without a
    /// next device, and callee unwinds at random calls.
    #[test]
    fn device_select_display_status_matches() {
        oc841_install_fakes();
        check(
            |c| {
                c.pc = 0x0018_e510;
                ee::s02::f_0018e510__generated(c)
            },
            oc841_device_select_display_status,
            600,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8410, 40);
                c.hooks.miss = oc841_miss;
                c.bus.write32(OC841_CTRL, OC841_MAGIC);
                c.bus.write32(OC841_CTRL + 4, r.next_u32());
                c.bus.write32(OC841_CTRL + 8, r.next_u32() & 1);
                c.bus.write32(OC841_CTRL + 12, r.next_u32() & 1);
                c.bus.write32(OC841_SELF + 8, OC841_VT_SELF);
                c.bus.write32(OC841_SELF + 0x40, DATA + 0x8000);
                c.bus
                    .write32(OC841_SELF + 0x1bc, [1, 1, 0, 2][(seed % 4) as usize]);
                c.bus
                    .write32(OC841_SELF + 0x214, [3, 0, 1, 3, 2][(seed / 4 % 5) as usize]);
                c.bus.write32(OC841_SELF + 0x22c, r.next_u32() & 1);
                c.bus.write32(OC841_MC + 0x428, r.next_u32() % 4);
                c.bus.write32(OC841_MC + 0x434, OC841_CARD);
                c.bus.write32(OC841_CARD, OC841_VT_CARD);
                c.bus.write32(OC841_TEXT + 8, OC841_VT_TEXT);
                for (off, k) in [(0x118, 0), (0x150, 1), (0x158, 2)] {
                    oc841_slot(c, &mut r, OC841_VT_SELF, off, k);
                }
                for (off, k) in [
                    (0x70, 3),
                    (0x1a8, 4),
                    (0x1b0, 5),
                    (0x1b8, 6),
                    (0x210, 7),
                    (0x218, 8),
                ] {
                    oc841_slot(c, &mut r, OC841_VT_CARD, off, k);
                }
                for (off, k) in [(0x40, 9), (0x48, 10), (0xc8, 11)] {
                    oc841_slot(c, &mut r, OC841_VT_TEXT, off, k);
                }
                c.g[A0][0] = sx(OC841_SELF);
                c.g[RA][0] = sx(0x0020_0000 + 0x10 * seed);
                c.pc = 0x0018_e510;
            },
        );
    }

    #[test]
    fn cfe_state_options_device_select_v33_matches() {
        check(
            ssx3_ee::s02::f_0018e478__generated,
            cfe_state_options_device_select_v33_2,
            80,
            &|c, seed| {
                let mut r = oc842_machine(c, seed, 0x0018_e478, 0x18e478, 3);
                let this = DATA + 0x2000;
                let owner = DATA + 0x3000;
                let engine = DATA + 0x4000;
                let vt = DATA + 0x5000;
                // The guard is already set on every fifth seed.
                w842(
                    c,
                    this + 0x1ac,
                    if seed % 5 == 0 { r.next_u32() | 1 } else { 0 },
                );
                w842(c, this + 0x10, owner);
                // The a2 flag inputs: each of the four words zero or not.
                for (i, off) in [0x1bcu32, 0x1e4, 0x1b4, 0x230].into_iter().enumerate() {
                    let v = if (seed >> (i + 1)) & 1 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    };
                    w842(c, this + off, v);
                }
                if seed % 7 == 0 {
                    w842(c, this + 0x1bc, 1);
                }
                w842(c, owner, engine);
                w842(c, engine + 4, vt);
                slot842(
                    c,
                    vt,
                    0x28,
                    (r.next_u32() as i16 as i32 & !3) as u16,
                    if seed % 4 == 1 { OC842_RET_ZERO } else { FAKE },
                );
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn fe_screen_select_profile_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018ea98__generated,
            fe_screen_select_profile_ctor,
            60,
            &|c, seed| {
                let mut r = oc842_machine(c, seed, 0x0018_ea98, 0x18ea98, 1);
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0x7_fff0));
            },
        );
    }

    /// `cFEStateOptionsGame_OnDeactivate` (0x001d_dfa8) against the generated body.
    #[test]
    fn c_fe_state_options_game_on_deactivate_001ddfa8_matches() {
        check(
            ee::s04::f_001ddfa8__generated,
            c_fe_state_options_game_on_deactivate_001ddfa8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1ddfa8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateOptionsSound_OnDeactivate` (0x001d_e010) against the generated body.
    #[test]
    fn c_fe_state_options_sound_on_deactivate_001de010_matches() {
        check(
            ee::s04::f_001de010__generated,
            c_fe_state_options_sound_on_deactivate_001de010,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de010, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateOptionsHUD_OnDeactivate` (0x001d_e078) against the generated body.
    #[test]
    fn c_fe_state_options_hud_on_deactivate_001de078_matches() {
        check(
            ee::s04::f_001de078__generated,
            c_fe_state_options_hud_on_deactivate_001de078,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de078, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateOptionsController_OnDeactivate` (0x001d_e0e8) against the generated body.
    #[test]
    fn c_fe_state_options_controller_on_deactivate_001de0e8_matches() {
        check(
            ee::s04::f_001de0e8__generated,
            c_fe_state_options_controller_on_deactivate_001de0e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de0e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
