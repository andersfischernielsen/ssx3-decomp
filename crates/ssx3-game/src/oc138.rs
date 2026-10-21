//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path front-end / network / sound / script helpers,
//! each with a differential test against the generated body.
//!
//! Most are short: a widget-created callback that compares the widget's name
//! hash against a table of known names, a constructor that publishes a vtable
//! and initialises a few fields, a table walk or a state-machine step. Calls
//! follow the generated code (cross-shard calls go through the function table
//! as [`call_ptr`] does, matching the generator's `call_indirect`), and loops
//! that check events use [`crate::events`].

use crate::call::{call_ptr, guest};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_d308,
        c_fe_state_select_persona_on_widget_created,
        "cFEStateSelectPersona_OnWidgetCreated",
    ),
    (
        0x001b_6bb8,
        c_fe_state_auth_dnas_error_on_widget_created,
        "cFEStateAuthDNASError_OnWidgetCreated",
    ),
    (0x001b_6288, net_config_auth_popup_ctor, "NetConfigAuthPopup_ctor"),
    (0x0027_26a8, in_seq_curve_track_update, "InSeq_CurveTrackUpdate"),
    (0x0014_b478, be_bolt_int_sum_equipped_bolt_value, "BEBoltInt_SumEquippedBoltValue"),
    (0x0035_9140, anim_combo_v52, "AnimCombo_v52"),
    (0x0026_9f18, path_find_marker_boundary, "Path_FindMarkerBoundary"),
    (0x001b_d460, c_fe_state_lobby_menu_enter_room, "cFEStateLobbyMenu_EnterRoom"),
    (0x001c_0138, c_fe_state_online_main_menu_on_leave_popup_event, "cFEStateOnlineMainMenu_OnLeavePopupEvent"),
    (0x001e_5800, c_ov_state_view_popup_on_widget_action, "cOVState_ViewPopup_OnWidgetAction"),
    (0x001e_5aa0, c_ov_state_view_popup_get_group_variant, "cOVState_ViewPopup_GetGroupVariant"),
    (0x0025_fc70, lobby_man_send_lock_chal, "LobbyMan_SendLockChal"),
    (0x002a_23d0, ssx_audio_system_dj_say_20e9, "SSXAudioSystem_DJSay20E9"),(0x0027_cea8, script_engine_add_script_obj, "ScriptEngine_AddScriptObj"),
    (0x002a_d650, bank_monitor_stop_by_bank, "BankMonitor_StopByBank"),
    (0x0027_23a8, in_seq_heap_free, "InSeq_HeapFree"),
    (0x0012_b340, ai_force_finish_riders, "AI_ForceFinishRiders"),
    (0x0024_12a0, mc_manager_begin_replay_write, "MCManager_BeginReplayWrite"),
    (0x0024_5418, c_game_load_state_init, "cGameLoadState_Init"),
    (0x0030_9ba8, w_script_man_get_hud_target_height, "WScriptMan_GetHudTargetHeight"),
    (0x0030_b9a0, w_script_man_collect_item, "WScriptMan_CollectItem"),
    (0x0024_f860, plants_menu_dtor, "PlantsMenu_dtor"),
    (0x0030_8038, w_script_process_save_state, "WScriptProcess_SaveState"),
    (0x001d_8160, mem_card_ui_try_next_slot, "MemCardUI_TryNextSlot"),(0x003a_a608, c_ps2_chunk_resolve_v15, "cPS2ChunkResolve_v15"),
    (0x0035_2f40, modifier_block_handle_event, "ModifierBlock_HandleEvent"),
    (0x0024_5b80, c_fe_load_state_init, "cFELoadState_Init"),
    (0x0019_6148, c_fe_state_audio_options_init, "cFEStateAudioOptions_Init"),
    (0x0032_7cc8, roller_modifier_copy_matrix_block, "RollerModifier_CopyMatrixBlock"),
    (0x0020_de30, ov_screen_replay_menu_on_widget_created, "OVScreenReplayMenu_OnWidgetCreated"),
    (0x0023_ef68, mc_manager_state0a_format, "MCManager_State0A_Format"),
    (0x001c_b418, c_keyboard_popup_set_text, "cKeyboardPopup_SetText"),(0x001d_0238, c_fe_state_reward_gallery_check_rewards_ready, "cFEStateRewardGallery_CheckRewardsReady"),
    (0x0019_b098, c_fe_state_char_equip_category_all_flagged, "cFEStateCharEquip_CategoryAllFlagged"),
    (0x0030_9030, w_script_man_restart, "WScriptMan_Restart"),
    (0x0011_94c0, rider_metrix_on_split_marker, "RiderMetrix_OnSplitMarker"),
    (0x001d_58b8, mem_card_state_base_shutdown, "MemCardStateBase_Shutdown"),
    (0x003a_a438, c_ps2_chunk_resolve_v06, "cPS2ChunkResolve_v06"),
    (0x0020_0d00, event_select_is_event_unlocked, "EventSelect_IsEventUnlocked"),
    (0x0039_a7f0, ui_list_box_set_item_text, "UIListBox_SetItemText"),
    (0x001a_8830, net_fe_state_v32, "NetFEState_v32"),
    (0x001a_9930, net_fe_state_show_disconnected_popup, "NetFEState_ShowDisconnectedPopup"),
    (0x001e_4758, ov_screen_message_center_update_count_text, "OVScreenMessageCenter_UpdateCountText"),
    (0x0025_98c8, lobby_man_on_disconnect, "LobbyMan_OnDisconnect"),
    (0x0025_bc48, lobby_man_parse_name_suggestions, "LobbyMan_ParseNameSuggestions"),
];

// ---------------------------------------------------------------------------
// Callees the generator emits as `call_indirect` (cross-shard or indirect).

mod addr {
    pub const STR_HASH: u32 = 0x0031_7670;
    pub const OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;
    pub const MEM_FREE: u32 = 0x0031_7e98;
    pub const MEMCPY: u32 = 0x0041_605c;
    pub const FILE_LOAD: u32 = 0x003e_18c8;
    pub const UI_TEXT_SET_ASCII: u32 = 0x003a_0c50;
    pub const UI_TEXT_SET_UNICODE: u32 = 0x003a_1000;
    pub const USTR_LENGTH: u32 = 0x003d_bca0;
}

/// `gp`-relative global address.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `a + off` (32-bit address arithmetic, sign-extended like `addu`).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// `addiu a, b, off` on a 64-bit register.
#[inline(always)]
fn sub(a: u64, off: u32) -> u64 {
    sx((a as u32).wrapping_sub(off))
}

/// `addu a, b` on 64-bit registers.
#[inline(always)]
fn addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// `lw` at `a`.
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

/// `sw v, a`.
#[inline(always)]
fn sw(c: &mut Cpu, a: u32, v: u64) {
    c.sw(a, v);
}

// ---------------------------------------------------------------------------
// Irradiance_Load (0x0038acd0).
//
// NOTE: not installed (no `OVERRIDES` entry): the test cannot bound the work,
// because `FILE_load` waits for the IOP file-request queue, which never
// completes on a test machine. The port itself is exact.

/// `void cIrradianceDataBase_Load(cIrradianceDataBase* db)`:
/// load the irradiance data file, allocate the index table (`count * 8`) and
/// the sample array (`count * 0xa0`), copy both from the file image, then
/// release the file buffer.
pub fn irradiance_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        c.g[SP][0] = sub(c.g[SP][0], 0x60);
        c.g[A1][0] = 0x100;
        c.sq(16, p(c.g[SP][0] as u32, 0x50));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, p(c.g[SP][0] as u32, 0x40));
        c.g[A0][0] = sx(0x0049_2e68);
        c.sq(18, p(c.g[SP][0] as u32, 0x30));
        c.sq(19, p(c.g[SP][0] as u32, 0x20));
        c.sq(20, p(c.g[SP][0] as u32, 0x10));
        c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
        call_ptr(c, addr::FILE_LOAD, 0x0038_ad00)?;
        // db->file = f; count = f[0]; index/samples allocated and copied.
        sw(c, p(c.g[S0][0] as u32, 0), c.g[V0][0]);
        c.g[S2][0] = 0xa0;
        c.g[A1][0] = sx(0x0049_2e80);
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        c.g[V1][0] = lw(c, c.g[V0][0] as u32);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[S1][0] = sx((c.g[V1][0] as u32) << 3);
        c.mult(0, 18, 3, 18);
        sw(c, p(c.g[S0][0] as u32, 4), c.g[V1][0]);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, addr::OPERATOR_NEW_NAMED, 0x0038_ad34)?;
        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[A1][0] = sx(0x0049_2e90);
        c.g[A3][0] = 0;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = 0x200_0000;
        sw(c, p(c.g[S0][0] as u32, 0xc), c.g[V0][0]);
        call_ptr(c, addr::OPERATOR_NEW_NAMED, 0x0038_ad54)?;
        c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0xc));
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S1][0];
        sw(c, p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        call_ptr(c, addr::MEMCPY, 0x0038_ad68)?;
        c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 8));
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, addr::MEMCPY, 0x0038_ad78)?;
        c.g[A0][0] = lw(c, c.g[S0][0] as u32);
        let empty = c.g[A0][0] == 0;
        c.lq(16, p(c.g[SP][0] as u32, 0x50));
        if !empty {
            c.cycles += 3;
            call_ptr(c, addr::MEM_FREE, 0x0038_ad8c)?;
            c.lq(16, p(c.g[SP][0] as u32, 0x50));
        }
        c.cycles += 7;
        c.lq(17, p(c.g[SP][0] as u32, 0x40));
        c.lq(18, p(c.g[SP][0] as u32, 0x30));
        c.lq(19, p(c.g[SP][0] as u32, 0x20));
        c.lq(20, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cFEStateSelectPersona_OnWidgetCreated (0x001ad308).

/// The name-hash table of the persona screen (title, locked label, name and
/// description strings).
mod select_persona {
    pub const TITLE: u32 = 0x0046_2e00;
    pub const LOCKED: u32 = 0x0046_2e10;
    pub const NAME: u32 = 0x0046_2e20;
    pub const DESC: u32 = 0x0046_2e30;
    pub const DESC_ID: u32 = 0x0046_2e40;
    /// `cUIText.+0x38` is the widget's name hash.
    pub const HASH: u32 = 0x38;
}

/// `void cFEStateSelectPersona_OnWidgetCreated(cFEStateSelectPersona* self,
/// cUIText* text)`: fill the persona list for the title, blank the lock icon,
/// or set the name/description of the new text widget.
pub fn c_fe_state_select_persona_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1ad3d0: {
            'b_1ad358: {
                c.cycles += 15;
                c.g[SP][0] = sub(c.g[SP][0], 0x50);
                c.sq(18, p(c.g[SP][0] as u32, 0x20));
                c.sq(17, p(c.g[SP][0] as u32, 0x30));
                c.g[S2][0] = c.g[A0][0];
                c.sq(19, p(c.g[SP][0] as u32, 0x10));
                c.g[S1][0] = c.g[A1][0];
                c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
                c.g[A0][0] = sx(select_persona::TITLE);
                c.sq(16, p(c.g[SP][0] as u32, 0x40));
                c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, select_persona::HASH));
                c.g[S3][0] = lw(c, gp(c, -0xc8));
                call_ptr(c, addr::STR_HASH, 0x001a_d33c)?;
                let matched = c.g[S0][0] == c.g[V0][0];
                c.g[A0][0] = sx(0x0046_0000);
                if !matched {
                    break 'b_1ad358;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x001a_d3e8, 0x001a_d350)?;
                c.lq(16, p(c.g[SP][0] as u32, 0x40));
                break 'b_1ad3d0;
            }
            'b_1ad3cc: {
                'b_1ad378: {
                    c.cycles += 5;
                    c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, select_persona::HASH));
                    c.g[A0][0] = sx(select_persona::LOCKED);
                    call_ptr(c, addr::STR_HASH, 0x001a_d364)?;
                    let matched = c.g[S0][0] == c.g[V0][0];
                    c.g[A0][0] = sx(0x0046_0000);
                    if !matched {
                        break 'b_1ad378;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    sw(c, p(c.g[S1][0] as u32, 0x18), c.g[V0][0]);
                    break 'b_1ad3cc;
                }
                'b_1ad3a0: {
                    c.cycles += 5;
                    c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, select_persona::HASH));
                    c.g[A0][0] = sx(select_persona::NAME);
                    call_ptr(c, addr::STR_HASH, 0x001a_d384)?;
                    let matched = c.g[S0][0] == c.g[V0][0];
                    c.g[A0][0] = sx(0x0046_0000);
                    if !matched {
                        break 'b_1ad3a0;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = lw(c, p(c.g[S3][0] as u32, 0x48));
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, addr::UI_TEXT_SET_ASCII, 0x001a_d398)?;
                    c.lq(16, p(c.g[SP][0] as u32, 0x40));
                    break 'b_1ad3d0;
                }
                c.cycles += 5;
                c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, select_persona::HASH));
                c.g[A0][0] = sx(select_persona::DESC);
                call_ptr(c, addr::STR_HASH, 0x001a_d3ac)?;
                let mismatched = c.g[S0][0] != c.g[V0][0];
                c.lq(16, p(c.g[SP][0] as u32, 0x40));
                if !mismatched {
                    c.cycles += 6;
                    c.g[A0][0] = sx(select_persona::DESC_ID);
                    call_ptr(c, addr::STR_HASH, 0x001a_d3c0)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, addr::UI_TEXT_SET_UNICODE, 0x001a_d3cc)?;
                    c.cycles += 1;
                    c.lq(16, p(c.g[SP][0] as u32, 0x40));
                }
                break 'b_1ad3cc;
            }
            c.cycles += 1;
            c.lq(16, p(c.g[SP][0] as u32, 0x40));
        }
        c.cycles += 6;
        c.lq(17, p(c.g[SP][0] as u32, 0x30));
        c.lq(18, p(c.g[SP][0] as u32, 0x20));
        c.lq(19, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// NetConfigAuthPopup_ctor (0x001b6288).

/// `NetConfigAuthPopup` fields.
mod net_config_auth {
    /// Base class (`NetFEState` vtable).
    pub const VTABLE: u32 = 0x8;
    /// The popup id.
    pub const ID: u32 = 0xc;
    /// The title text buffer.
    pub const TITLE: u32 = 0x6a8;
    pub const F6D0: u32 = 0x6d0;
    pub const F6D4: u32 = 0x6d4;
    pub const F6E4: u32 = 0x6e4;
    pub const F6F4: u32 = 0x6f4;
    pub const F6F8: u32 = 0x6f8;
    /// The `cBXString`-like buffer freed and reallocated here.
    pub const BUF: u32 = 0x6fc;
}

/// `void* NetConfigAuthPopup_ctor(NetConfigAuthPopup* self, void* a1,
/// void* a2)`: initialise the online popup state, then allocate the title
/// buffer from the measured widget text and return `self`.
pub fn net_config_auth_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        c.g[SP][0] = sub(c.g[SP][0], 0x40);
        c.sq(18, p(c.g[SP][0] as u32, 0x10));
        c.sq(16, p(c.g[SP][0] as u32, 0x30));
        c.sq(17, p(c.g[SP][0] as u32, 0x20));
        c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x001a_8500, 0x001b_62a4)?;
        c.g[V0][0] = 2;
        c.g[V1][0] = sx(0x0046_89f0);
        sw(c, p(c.g[S2][0] as u32, net_config_auth::F6F4), c.g[V0][0]);
        c.g[A2][0] = 0x44;
        c.g[A1][0] = 0x1e;
        sw(c, p(c.g[S2][0] as u32, net_config_auth::ID), c.g[A2][0]);
        c.g[A0][0] = sx(0x0046_4208);
        sw(c, p(c.g[S2][0] as u32, net_config_auth::VTABLE), c.g[V1][0]);
        sw(c, p(c.g[S2][0] as u32, net_config_auth::F6F8), c.g[A1][0]);
        sw(c, p(c.g[S2][0] as u32, net_config_auth::F6D0), 0);
        sw(c, p(c.g[S2][0] as u32, net_config_auth::F6D4), 0);
        sw(c, p(c.g[S2][0] as u32, net_config_auth::F6E4), 0);
        sw(c, p(c.g[S2][0] as u32, net_config_auth::BUF), 0);
        // Measure the title string of the current UI object through its vtable.
        c.g[V0][0] = lw(c, gp(c, -0x848));
        c.g[S0][0] = lw(c, p(c.g[V0][0] as u32, 0x8c));
        c.g[S1][0] = lw(c, p(c.g[S0][0] as u32, 4));
        c.g[V0][0] = c.lh(p(c.g[S1][0] as u32, 0x20));
        c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
        call_ptr(c, addr::STR_HASH, 0x001b_62f8)?;
        c.g[V1][0] = lw(c, p(c.g[S1][0] as u32, 0x24));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        let func = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001b_6308);
        c.pc = func;
        ssx3_ee_rt::funcs::call_indirect(c, func);
        if c.pc != 0x001b_6308 {
            return Err(crate::call::Unwound);
        }
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, addr::USTR_LENGTH, 0x001b_6310)?;
        c.g[A0][0] = lw(c, p(c.g[S2][0] as u32, net_config_auth::BUF));
        let empty = c.g[A0][0] == 0;
        c.g[S0][0] = c.g[V0][0];
        if !empty {
            c.cycles += 3;
            call_ptr(c, addr::MEM_FREE, 0x001b_6324)?;
            sw(c, p(c.g[S2][0] as u32, net_config_auth::BUF), 0);
        }
        c.cycles += 16;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(5));
        c.g[A1][0] = sx(0x0046_4220);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, addr::OPERATOR_NEW_NAMED, 0x001b_6344)?;
        sw(c, p(c.g[S2][0] as u32, net_config_auth::BUF), c.g[V0][0]);
        c.g[V0][0] = c.g[S2][0];
        sw(c, p(c.g[S2][0] as u32, net_config_auth::TITLE), 0);
        c.lq(16, p(c.g[SP][0] as u32, 0x30));
        c.lq(17, p(c.g[SP][0] as u32, 0x20));
        c.lq(18, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cFEStateAuthDNASError_OnWidgetCreated (0x001b6bb8).

/// Name-hash table of the DNAS error screen.
mod dnas_error {
    pub const STATUS: u32 = 0x004a_1ba8;
    pub const STATUS2: u32 = 0x004a_1bb0;
    pub const STATUS3: u32 = 0x004a_1bb8;
    pub const RETRY: u32 = 0x0046_4250;
    pub const MESSAGE: u32 = 0x0046_42c0;
}

/// `void cFEStateAuthDNASError_OnWidgetCreated(cFEStateAuthDNASError* self,
/// cUIWidget* obj)`: keep the status widgets, press the retry button when the
/// retry widget appears, or make the message text scroll.
pub fn c_fe_state_auth_dnas_error_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b6c84: {
            'b_1b6c80: {
                'b_1b6bf4: {
                    c.cycles += 13;
                    c.g[SP][0] = sub(c.g[SP][0], 0x40);
                    c.sq(18, p(c.g[SP][0] as u32, 0x10));
                    c.sq(17, p(c.g[SP][0] as u32, 0x20));
                    c.g[S2][0] = c.g[A0][0];
                    c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, p(c.g[SP][0] as u32, 0x30));
                    c.g[A0][0] = sx(dnas_error::STATUS);
                    c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x38));
                    call_ptr(c, addr::STR_HASH, 0x001b_6be4)?;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.cycles += 2;
                        sw(c, p(c.g[S2][0] as u32, 0x6dc), c.g[S1][0]);
                        break 'b_1b6c80;
                    }
                    c.g[A0][0] = sx(dnas_error::STATUS2);
                    break 'b_1b6bf4;
                }
                'b_1b6c10: {
                    c.cycles += 5;
                    c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x38));
                    c.g[A0][0] = sx(dnas_error::STATUS2);
                    call_ptr(c, addr::STR_HASH, 0x001b_6c00)?;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.cycles += 2;
                        sw(c, p(c.g[S2][0] as u32, 0x6e0), c.g[S1][0]);
                        break 'b_1b6c80;
                    }
                    c.g[A0][0] = sx(dnas_error::STATUS3);
                    break 'b_1b6c10;
                }
                'b_1b6c2c: {
                    c.cycles += 5;
                    c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x38));
                    c.g[A0][0] = sx(dnas_error::STATUS3);
                    call_ptr(c, addr::STR_HASH, 0x001b_6c1c)?;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.cycles += 2;
                        sw(c, p(c.g[S2][0] as u32, 0x6e4), c.g[S1][0]);
                        break 'b_1b6c80;
                    }
                    c.g[A0][0] = sx(0x0046_0000);
                    break 'b_1b6c2c;
                }
                'b_1b6c60: {
                    c.cycles += 5;
                    c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x38));
                    c.g[A0][0] = sx(dnas_error::RETRY);
                    call_ptr(c, addr::STR_HASH, 0x001b_6c38)?;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.cycles += 8;
                        c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 8));
                        c.g[A1][0] = 1;
                        c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x48));
                        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
                        let func = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001b_6c58);
                        c.g[A0][0] = addu(c.g[S1][0], c.g[A0][0]);
                        c.pc = func;
                        ssx3_ee_rt::funcs::call_indirect(c, func);
                        if c.pc != 0x001b_6c58 {
                            return Err(crate::call::Unwound);
                        }
                        c.lq(16, p(c.g[SP][0] as u32, 0x30));
                        break 'b_1b6c84;
                    }
                    c.g[A0][0] = sx(0x0046_0000);
                    break 'b_1b6c60;
                }
                c.cycles += 5;
                c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x38));
                c.g[A0][0] = sx(dnas_error::MESSAGE);
                call_ptr(c, addr::STR_HASH, 0x001b_6c6c)?;
                let mismatched = c.g[S0][0] != c.g[V0][0];
                c.lq(16, p(c.g[SP][0] as u32, 0x30));
                if mismatched {
                    break 'b_1b6c84;
                }
                c.cycles += 3;
                c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x90));
                c.g[V0][0] = c.g[V0][0] | 8;
                sw(c, p(c.g[S1][0] as u32, 0x90), c.g[V0][0]);
                break 'b_1b6c80;
            }
            c.cycles += 1;
            c.lq(16, p(c.g[SP][0] as u32, 0x30));
        }
        c.cycles += 5;
        c.lq(17, p(c.g[SP][0] as u32, 0x20));
        c.lq(18, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// InSeq_CurveTrackUpdate (0x002726a8).

/// `InSeq` curve-track fields (`InSeq_CurveTrackUpdate`).
mod in_seq_curve {
    /// The current curve segment (`{u16 start, u16 end, f32 p[4]}`).
    pub const CURVE: u32 = 0x24;
    /// The float value sampled last time.
    pub const VALUE: u32 = 0x1c;
    /// The segment being played.
    pub const SEGMENT: u32 = 0x14;
    /// The key index inside that segment.
    pub const KEY: u32 = 0x18;
    /// The current value's address inside the curve.
    pub const CURSOR: u32 = 0x20;
    /// The curve track's length in keys.
    pub const LENGTH: u32 = 0x10;
}

/// `float InSeq_CurveTrackUpdate(InSeqCurveTrack* self, float t)`:
/// advance the curve track's playhead by `t` keys, evaluate the cubic through
/// the four surrounding keys and return the value; 0 when the track ended.
pub fn in_seq_curve_track_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_272778: {
            c.cycles += 8;
            c.g[SP][0] = sub(c.g[SP][0], 0x20);
            c.mov_s(1, 12);
            c.sq(16, p(c.g[SP][0] as u32, 0x10));
            c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, in_seq_curve::CURVE));
            let ended = c.g[A0][0] == 0;
            c.g[V0][0] = 0;
            if ended {
                break 'b_272778;
            }
            'b_272758: {
                c.cycles += 7;
                c.g[V0][0] = c.lhu(p(c.g[A0][0] as u32, 2));
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.c_le_s(0, 1);
                if !c.fcc() {
                    c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, in_seq_curve::SEGMENT));
                    break 'b_272758;
                }
                'b_272714: {
                    c.cycles += 9;
                    c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, in_seq_curve::KEY));
                    sw(c, p(c.g[S0][0] as u32, in_seq_curve::SEGMENT), c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    sw(c, p(c.g[S0][0] as u32, in_seq_curve::CURSOR), 0);
                    sw(c, p(c.g[S0][0] as u32, in_seq_curve::KEY), c.g[V1][0]);
                    c.g[V0][0] = c.lhu(c.g[A0][0] as u32);
                    c.g[V0][0] = (c.g[V0][0] < 0x15) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, in_seq_curve::LENGTH));
                        break 'b_272714;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                    sw(c, p(c.g[S0][0] as u32, in_seq_curve::CURSOR), c.g[V0][0]);
                    c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, in_seq_curve::LENGTH));
                    break 'b_272714;
                }
                'b_272748: {
                    c.cycles += 5;
                    c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, in_seq_curve::KEY));
                    c.g[V1][0] = lw(c, c.g[V0][0] as u32);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                    if c.g[A0][0] != c.g[V1][0] {
                        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, in_seq_curve::SEGMENT));
                        break 'b_272748;
                    }
                    c.cycles += 8;
                    c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, in_seq_curve::SEGMENT));
                    sw(c, p(c.g[S0][0] as u32, in_seq_curve::CURVE), 0);
                    c.mtc1(0, 12);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                    call_ptr(c, 0x0027_2570, 0x0027_273c)?;
                    c.swc1(0, p(c.g[S0][0] as u32, in_seq_curve::VALUE));
                    c.g[V0][0] = 0;
                    break 'b_272778;
                }
                c.cycles += 4;
                c.g[V1][0] = c.lhu(c.g[V0][0] as u32);
                c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
                sw(c, p(c.g[S0][0] as u32, in_seq_curve::CURVE), c.g[V0][0]);
                c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, in_seq_curve::SEGMENT));
                break 'b_272758;
            }
            c.cycles += 8;
            c.g[V0][0] = c.lhu(p(c.g[A0][0] as u32, 2));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            c.mtc1(2, 12);
            c.cvt_s_w(12, 12);
            c.sub_s(12, 1, 12);
            call_ptr(c, 0x0027_2570, 0x0027_2770)?;
            c.swc1(0, p(c.g[S0][0] as u32, in_seq_curve::VALUE));
            c.g[V0][0] = 1;
            break 'b_272778;
        }
        c.cycles += 4;
        c.lq(16, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// BEBoltInt_SumEquippedBoltValue (0x0014b478).

/// The character bolt database and the equipped-bolt flags table.
mod bolts {
    /// `BoltsDB`: per-character counts at `+0x2c`, bolt array index at
    /// `+0xa4`, the shared bolt array base at `+4`.
    pub const DB: u32 = 0x004a_6750;
    pub const ARRAY_BASE: u32 = 0x004a_7c30;
    /// `BoltsBolt` stride and the equipped flag word offset.
    pub const STRIDE: u32 = 0x38;
    pub const FLAGS: u32 = 4;
    pub const VALUE: u32 = 0xc;
}

/// `int BEBoltInt_SumEquippedBoltValue(cBEState*, int charIndex, int, int)`:
/// sum the `+0xc` values of the character's bolts whose equipped bit (0x10)
/// is set in the shared flags table.
pub fn be_bolt_int_sum_equipped_bolt_value(c: &mut Cpu) {
    guest(c, |c| {
        'b_14b538: {
            c.cycles += 28;
            c.g[SP][0] = sub(c.g[SP][0], 0x60);
            c.g[V0][0] = 0x9b50;
            c.sq(17, p(c.g[SP][0] as u32, 0x40));
            c.mult(0, 7, 5, 2);
            c.g[S1][0] = c.g[A2][0];
            c.sq(16, p(c.g[SP][0] as u32, 0x50));
            c.g[A2][0] = 0xf88;
            c.g[V1][0] = 0x4a_0000;
            c.mult(0, 6, 17, 6);
            c.g[V1][0] = sx(0x004a_6ca8u32.wrapping_add(0));
            c.g[S0][0] = 0x4a_0000;
            c.sq(20, p(c.g[SP][0] as u32, 0x10));
            c.g[S0][0] = sx(0x004a_6750u32.wrapping_add(0));
            c.sq(18, p(c.g[SP][0] as u32, 0x30));
            c.sq(19, p(c.g[SP][0] as u32, 0x20));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = addu(c.g[A2][0], c.g[V1][0]);
            c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[S4][0] = addu(c.g[A3][0], c.g[A2][0]);
            call_ptr(c, 0x0014_d988, 0x0014_b4cc)?;
            c.g[S2][0] = 0;
            c.g[S3][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0014_d998, 0x0014_b4e0)?;
            let none = (c.g[S3][0] as i64) <= 0;
            c.g[V1][0] = c.g[V0][0];
            if none {
                break 'b_14b538;
            }
            c.cycles += 2;
            c.g[A1][0] = lw(c, p(c.g[S4][0] as u32, 0x288));
            c.g[A0][0] = c.g[S3][0];
            'l_14b4f0: loop {
                'b_14b518: {
                    'b_14b514: {
                        c.cycles += 6;
                        c.g[V0][0] = c.lh(p(c.g[V1][0] as u32, bolts::FLAGS));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                        c.g[V0][0] = addu(c.g[V0][0], c.g[A1][0]);
                        c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                        let negative = (c.g[V0][0] as i64) < 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if !negative {
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                            c.g[V0][0] = addu(c.g[S4][0], c.g[V0][0]);
                            break 'b_14b518;
                        }
                        break 'b_14b514;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break 'b_14b518;
                }
                'b_14b530: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu(p(c.g[V0][0] as u32, 2));
                    c.g[V0][0] = c.g[V0][0] & 0x10;
                    let unequipped = c.g[V0][0] == 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    if unequipped {
                        break 'b_14b530;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.lh(p(c.g[V1][0] as u32, bolts::VALUE));
                    c.g[S2][0] = addu(c.g[S2][0], c.g[V0][0]);
                    break 'b_14b530;
                }
                c.cycles += 2;
                let more = c.g[A0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(bolts::STRIDE));
                if !more {
                    break 'b_14b538;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_b4f0);
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S2][0];
        c.lq(16, p(c.g[SP][0] as u32, 0x50));
        c.lq(17, p(c.g[SP][0] as u32, 0x40));
        c.lq(18, p(c.g[SP][0] as u32, 0x30));
        c.lq(19, p(c.g[SP][0] as u32, 0x20));
        c.lq(20, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// AnimCombo_v52 (0x00359140).

/// `cAnimCombo` fields used here.
mod anim_combo {
    /// `+0x14`/`+0x15`/`+0x17`: per-part enable bytes.
    pub const PART_A: u32 = 0x14;
    pub const PART_B: u32 = 0x15;
    pub const PART_C: u32 = 0x17;
    /// The object whose vertex arrays are blended.
    pub const MODEL: u32 = 0x60;
    /// The animation object and its per-part output matrices.
    pub const ANIM: u32 = 0x7c;
    pub const MATRICES: u32 = 0x84;
}

/// `void AnimCombo_v52(cAnimCombo* self)`: update the underlying animation
/// object and, when the combo is playing, multiply every part's matrix with
/// the animation's transform.
pub fn anim_combo_v52(c: &mut Cpu) {
    guest(c, |c| {
        'b_359204: {
            c.cycles += 14;
            c.g[SP][0] = sub(c.g[SP][0], 0x70);
            c.sq(16, p(c.g[SP][0] as u32, 0x60));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, p(c.g[SP][0] as u32, 0x50));
            c.sq(18, p(c.g[SP][0] as u32, 0x40));
            c.sq(19, p(c.g[SP][0] as u32, 0x30));
            c.sq(20, p(c.g[SP][0] as u32, 0x20));
            c.sq(21, p(c.g[SP][0] as u32, 0x10));
            c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
            call_ptr(c, 0x0034_e348, 0x0035_916c)?;
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, anim_combo::ANIM));
            if c.g[V0][0] == 0 {
                c.lq(16, p(c.g[SP][0] as u32, 0x60));
                break 'b_359204;
            }
            'b_3591a0: {
                c.cycles += 3;
                c.g[V0][0] = c.lb(p(c.g[S0][0] as u32, anim_combo::PART_A));
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, anim_combo::MODEL));
                    break 'b_3591a0;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(p(c.g[S0][0] as u32, anim_combo::PART_C));
                if c.g[V0][0] == 0 {
                    c.lq(16, p(c.g[SP][0] as u32, 0x60));
                    break 'b_359204;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(p(c.g[S0][0] as u32, anim_combo::PART_B));
                if c.g[V0][0] == 0 {
                    c.lq(16, p(c.g[SP][0] as u32, 0x60));
                    break 'b_359204;
                }
                c.cycles += 1;
                c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, anim_combo::MODEL));
                break 'b_3591a0;
            }
            'b_359200: {
                c.cycles += 6;
                c.g[S5][0] = 0;
                c.g[S2][0] = 0;
                c.g[S3][0] = lw(c, p(c.g[V1][0] as u32, 0x80));
                c.g[V0][0] = lw(c, p(c.g[S3][0] as u32, 4));
                let none = (c.g[V0][0] as i64) <= 0;
                c.g[S1][0] = lw(c, p(c.g[S3][0] as u32, 8));
                if none {
                    break 'b_359200;
                }
                c.cycles += 2;
                c.g[S4][0] = 0;
                'l_3591c0: loop {
                    'b_3591ec: {
                        c.cycles += 4;
                        c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 8));
                        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x1c));
                        let empty = c.g[V1][0] == 0;
                        c.g[V0][0] = sx((c.g[S5][0] as u32) << 6);
                        if empty {
                            break 'b_3591ec;
                        }
                        c.cycles += 7;
                        c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, anim_combo::ANIM));
                        c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, anim_combo::MATRICES));
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(1));
                        c.g[A0][0] = addu(c.g[A0][0], c.g[S4][0]);
                        c.g[A1][0] = addu(c.g[A1][0], c.g[V0][0]);
                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0xd0));
                        call_ptr(c, 0x0035_1948, 0x0035_91ec)?;
                        break 'b_3591ec;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = lw(c, p(c.g[S3][0] as u32, 4));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                    if !more {
                        break 'b_359200;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_91c0);
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, p(c.g[SP][0] as u32, 0x60));
            break 'b_359204;
        }
        c.cycles += 8;
        c.lq(17, p(c.g[SP][0] as u32, 0x50));
        c.lq(18, p(c.g[SP][0] as u32, 0x40));
        c.lq(19, p(c.g[SP][0] as u32, 0x30));
        c.lq(20, p(c.g[SP][0] as u32, 0x20));
        c.lq(21, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Path_FindMarkerBoundary (0x00269f18).

/// `Path_FindMarkerBoundary(path, rangeList, float* t)`:
/// scan the range list for the entry containing `*t` and clamp `*t` to that
/// entry's boundary (or `*t + 50` when nothing matches), then evaluate the
/// path at the clamped distance. The list is `{int count; Entry* entries}`
/// with 0x10-byte entries (`{int type; int pad; float lo; float hi;}`), and
/// the same object doubles as the path track (`+8` segment count, `+0x18`
/// segments).
pub fn path_find_marker_boundary(c: &mut Cpu) {
    guest(c, |c| {
        'b_269fc0: {
            c.cycles += 11;
            c.g[SP][0] = sub(c.g[SP][0], 0x20);
            c.g[AT][0] = 0x4248_0000;
            c.mtc1(1, 0);
            c.sq(16, p(c.g[SP][0] as u32, 0x10));
            c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = lw(c, c.g[A1][0] as u32);
            c.lwc1(1, c.g[A2][0] as u32);
            c.add_s(3, 1, 0);
            let none = (c.g[V0][0] as i64) <= 0;
            c.mov_s(2, 3);
            if none {
                break 'b_269fc0;
            }
            c.cycles += 7;
            c.g[V1][0] = lw(c, p(c.g[A1][0] as u32, 4));
            c.mov_s(4, 1);
            c.g[A0][0] = c.g[V0][0];
            c.g[T1][0] = 0x10;
            c.g[T0][0] = 0xc;
            c.g[A3][0] = 0xe;
            'l_269f60: loop {
                'b_269fb8: {
                    c.cycles += 5;
                    c.lwc1(1, p(c.g[V1][0] as u32, 8));
                    c.c_lt_s(4, 1);
                    if c.fcc() {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                        break 'b_269fb8;
                    }
                    c.cycles += 5;
                    c.lwc1(0, p(c.g[V1][0] as u32, 0xc));
                    c.c_lt_s(0, 4);
                    if c.fcc() {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                        break 'b_269fb8;
                    }
                    'b_269fb4: {
                        'b_269f9c: {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, c.g[V1][0] as u32);
                            if c.g[V0][0] != c.g[T1][0] {
                                break 'b_269f9c;
                            }
                            c.cycles += 2;
                            c.max_s(3, 0, 3);
                            break 'b_269fb4;
                        }
                        'b_269fac: {
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[T0][0] {
                                break 'b_269fac;
                            }
                            c.cycles += 2;
                            c.min_s(2, 1, 2);
                            break 'b_269fb4;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == c.g[A3][0] {
                            c.max_s(2, 0, 2);
                        }
                        break 'b_269fb4;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    break 'b_269fb8;
                }
                c.cycles += 2;
                let more = c.g[A0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                if !more {
                    break 'b_269fc0;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_9f60);
                }
            }
        }
        'b_269fd8: {
            c.cycles += 5;
            c.lwc1(0, c.g[A2][0] as u32);
            c.c_eq_s(2, 0);
            if c.fcc() {
                c.swc1(3, c.g[A2][0] as u32);
                break 'b_269fd8;
            }
            c.cycles += 1;
            c.swc1(2, c.g[A2][0] as u32);
            break 'b_269fd8;
        }
        c.cycles += 9;
        c.lwc1(12, c.g[A2][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0026_ab20, 0x0026_9fe8)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        c.lq(16, p(c.g[SP][0] as u32, 0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cFEStateLobbyMenu_EnterRoom (0x00001bd460).

/// `cFEStateLobbyMenu_EnterRoom`: open the room popup, store the lobby name (or clear it when empty) and copy the room name into the label.
pub fn c_fe_state_lobby_menu_enter_room(c: &mut Cpu) {
    guest(c, |c| {
    'b_1bd50c: {
    'b_1bd4f4: {
    'b_1bd4dc: {
    c.cycles += 26;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S2][0] = c.g[A0][0];
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S1][0] = c.g[A2][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S3][0] = c.g[A1][0];
    call_ptr(c, 0x001a9c50, 0x001bd488)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 0x460000;
    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), 0u64);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4fa8));
    call_ptr(c, 0x00317670, 0x001bd49c)?;
    c.g[V1][0] = 0x3;
    c.g[A2][0] = 0x829;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x164), 0u64);
    c.g[A1][0] = c.g[S0][0];
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
    c.g[A0][0] = c.g[S2][0];
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[A2][0]);
    call_ptr(c, 0x001a9bc0, 0x001bd4c0)?;
    if c.g[S1][0] == 0u64 {
    c.sw((c.g[S2][0] as u32).wrapping_add(0x770), 0u64);
    break 'b_1bd4dc;
    } else {
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x00416810, 0x001bd4d0)?;
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = 0x1;
    if t {
    break 'b_1bd4f4;
    } else {
    c.cycles += 1;
    c.sw((c.g[S2][0] as u32).wrapping_add(0x770), 0u64);
    break 'b_1bd4dc;
    }
    }
    }
    c.cycles += 6;
    c.g[A1][0] = c.g[S3][0];
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    c.g[A2][0] = 0u64;
    call_ptr(c, 0x0025d800, 0x001bd4ec)?;
    let t = true;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    if t {
    break 'b_1bd50c;
    } else {
    break 'b_1bd4f4;
    }
    }
    c.cycles += 6;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    c.sw((c.g[S2][0] as u32).wrapping_add(0x770), c.g[V0][0]);
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S3][0];
    call_ptr(c, 0x0025d800, 0x001bd508)?;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    break 'b_1bd50c;
    }
    c.cycles += 13;
    c.g[A1][0] = c.g[S3][0];
    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xb80));
    call_ptr(c, 0x0025f150, 0x001bd518)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x003184e8, 0x001bd524)?;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cFEStateOnlineMainMenu_OnLeavePopupEvent (0x00001c0138).

/// `cFEStateOnlineMainMenu_OnLeavePopupEvent`: forward a popup event; when the "join" popup (id 0xf) closes, queue the lobby entry and tear the old session down.
pub fn c_fe_state_online_main_menu_on_leave_popup_event(c: &mut Cpu) {
    guest(c, |c| {
    'b_1c020c: {
    'b_1c0208: {
    'b_1c0200: {
    c.cycles += 6;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[V0][0] = 0xf;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    let t = c.g[A2][0] != c.g[V0][0];
    c.g[S0][0] = c.g[A0][0];
    if t {
    break 'b_1c0200;
    } else {
    c.cycles += 18;
    c.g[A2][0] = 0xf;
    call_ptr(c, 0x001a97b8, 0x001c0158)?;
    c.g[V0][0] = 0x1;
    c.g[A1][0] = 0x460000;
    c.g[A3][0] = 0u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xba8));
    c.g[A0][0] = 0x58;
    c.g[A2][0] = 0u64;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x6b0), c.g[V0][0]);
    call_ptr(c, 0x00317d70, 0x001c0178)?;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x00194a60, 0x001c0184)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x001a8a40, 0x001c0190)?;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1c0208;
    } else {
    'b_1c01a8: {
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;

    if t {
    break 'b_1c01a8;
    } else {
    c.cycles += 2;
    call_ptr(c, 0x00260f80, 0x001c01a8)?;
    break 'b_1c01a8;
    }
    }
    'b_1c01c4: {
    c.cycles += 4;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x00261460, 0x001c01b0)?;
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1c01c4;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x002613c8, 0x001c01c0)?;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    break 'b_1c01c4;
    }
    }
    'b_1c01d4: {
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;

    if t {
    break 'b_1c01d4;
    } else {
    c.cycles += 2;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    call_ptr(c, 0x0025b800, 0x001c01d4)?;
    break 'b_1c01d4;
    }
    }
    c.cycles += 11;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x00261408, 0x001c01dc)?;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    call_ptr(c, 0x0025b800, 0x001c01e4)?;
    call_ptr(c, 0x002591b8, 0x001c01ec)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x002636f0, 0x001c01f8)?;
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    if t {
    break 'b_1c020c;
    } else {
    break 'b_1c0200;
    }
    }
    }
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x001a97b8, 0x001c0208)?;
    break 'b_1c0208;
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    break 'b_1c020c;
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cOVState_ViewPopup_OnWidgetAction (0x00001e5800).

/// `cOVState_ViewPopup_OnWidgetAction`: handle a popup widget action: close the message (and remove it from the list) or forward the close to the owner.
pub fn c_ov_state_view_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
    'b_1e58d4: {
    'b_1e58b0: {
    c.cycles += 6;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[V0][0] = 0x6;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    let t = c.g[A2][0] == c.g[V0][0];
    c.g[S0][0] = c.g[A0][0];
    if t {
    break 'b_1e58b0;
    } else {
    'b_1e588c: {
    'b_1e5834: {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = 0x5;
    if t {
    break 'b_1e5834;
    } else {
    c.cycles += 2;
    if c.g[A2][0] == c.g[V0][0] {
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
    break 'b_1e588c;
    } else {
    c.cycles += 2;
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    if t {
    break 'b_1e58d4;
    } else {
    break 'b_1e5834;
    }
    }
    }
    }
    c.cycles += 3;
    c.g[V0][0] = 0x7;
    if c.g[A2][0] != c.g[V0][0] {
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    break 'b_1e58d4;
    } else {
    c.cycles += 16;
    c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
    c.g[V1][0] = 0x18;
    c.g[A1][0] = 0x4c0000;
    c.g[A3][0] = 0x14;
    c.mult(0, 3, 8, 3);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c08));
    c.g[A0][0] = 0x440000;
    c.g[A2][0] = 0x4;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1630));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    c.mult(0, 2, 2, 7);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
    if c.g[V1][0] == c.g[A2][0] {
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    break 'b_1e58d4;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.g[T0][0];
    call_ptr(c, 0x001e3268, 0x001e5888)?;
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
    break 'b_1e588c;
    }
    }
    }
    c.cycles += 9;
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = 0x14;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x001e58a8);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001e58a8 { return Err(crate::call::Unwound); }
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    if t {
    break 'b_1e58d4;
    } else {
    break 'b_1e58b0;
    }
    }
    }
    c.cycles += 9;
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = 0x14;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x001e58d0);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001e58d0 { return Err(crate::call::Unwound); }
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    break 'b_1e58d4;
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cOVState_ViewPopup_GetGroupVariant (0x00001e5aa0).

/// `cOVState_ViewPopup_GetGroupVariant`: pick the popup group variant (a small modulo of the entry value) used to lay out the message list.
pub fn c_ov_state_view_popup_get_group_variant(c: &mut Cpu) {
    guest(c, |c| {
    'b_1e5b70: {
    'b_1e5ac0: {
    c.cycles += 6;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x102c));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    let t = c.g[V0][0] != 0u64;
    c.g[S0][0] = c.g[A1][0];
    if t {
    break 'b_1e5ac0;
    } else {
    c.cycles += 2;
    let t = true;
    c.g[V0][0] = 0xff;
    if t {
    break 'b_1e5b70;
    } else {
    break 'b_1e5ac0;
    }
    }
    }
    'b_1e5b60: {
    'b_1e5b5c: {
    'b_1e5b58: {
    c.cycles += 18;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x001e2dc0, 0x001e5ac8)?;
    c.g[V1][0] = 0x18;
    c.g[A1][0] = 0x4c0000;
    c.mult(0, 3, 16, 3);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c08));
    c.g[T1][0] = c.g[V0][0];
    c.g[A3][0] = 0x14;
    c.g[A0][0] = 0x440000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1630));
    c.g[A2][0] = 0x4;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    c.mult(0, 2, 2, 7);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    let t = c.g[V1][0] == c.g[A2][0];
    c.g[T0][0] = 0x1;
    if t {
    break 'b_1e5b58;
    } else {
    'b_1e5b50: {
    'b_1e5b40: {
    'b_1e5b24: {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V1][0] as i64) < 5i64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = 0x3;
    if t {
    break 'b_1e5b24;
    } else {
    c.cycles += 2;
    let t = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = sx((c.g[T1][0] as u32) >> 31);
    if t {
    break 'b_1e5b40;
    } else {
    c.cycles += 2;
    let t = true;
    c.div(0, 9, 8);
    if t {
    break 'b_1e5b60;
    } else {
    break 'b_1e5b24;
    }
    }
    }
    }
    c.cycles += 3;
    c.g[V0][0] = 0x5;
    let t = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = 0x8;
    if t {
    break 'b_1e5b50;
    } else {
    c.cycles += 2;
    if c.g[V1][0] == c.g[V0][0] {
    c.g[T0][0] = 0x4;
    break 'b_1e5b5c;
    } else {
    c.cycles += 2;
    let t = true;
    c.div(0, 9, 8);
    if t {
    break 'b_1e5b60;
    } else {
    break 'b_1e5b40;
    }
    }
    }
    }
    c.cycles += 4;
    c.g[T0][0] = 0xa;
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    let t = true;
    c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
    if t {
    break 'b_1e5b5c;
    } else {
    break 'b_1e5b50;
    }
    }
    c.cycles += 2;
    let t = true;
    c.g[T0][0] = 0xb;
    if t {
    break 'b_1e5b5c;
    } else {
    break 'b_1e5b58;
    }
    }
    }
    c.cycles += 1;
    c.g[T0][0] = 0xa;
    break 'b_1e5b5c;
    }
    c.cycles += 1;
    c.div(0, 9, 8);
    break 'b_1e5b60;
    }
    'b_1e5b68: {
    c.cycles += 2;
    if c.g[T0][0] == 0u64 {
    break 'b_1e5b68;
    } else {
    break 'b_1e5b68;
    }
    }
    c.cycles += 2;
    c.g[V0][0] = c.hi[0];
    c.g[V0][0] = c.g[V0][0] & 0xff;
    break 'b_1e5b70;
    }
    c.cycles += 4;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// LobbyMan_SendLockChal (0x000025fc70).

/// `LobbyMan_SendLockChal`: queue a "lock challenge" message on the lobby manager and start its 20-frame hold timer.
pub fn lobby_man_send_lock_chal(c: &mut Cpu) {
    guest(c, |c| {
    'b_25fcfc: {
    c.cycles += 35;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S0][0] = c.g[A2][0];
    c.g[S2][0] = c.g[A0][0];
    c.g[A2][0] = 0x480000;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x910));
    call_ptr(c, 0x0025fb80, 0x0025fc98)?;
    c.g[S1][0] = 0x110;
    c.sw((c.g[S2][0] as u32), c.g[S0][0]);
    c.g[A1][0] = 0x480000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
    c.g[A0][0] = 0x8;
    c.g[A2][0] = 0x20000000;
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00317d70, 0x0025fcb8)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf0));
    c.g[V0][0] = 0x480000;
    c.sw((c.g[S0][0] as u32), c.g[S1][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
    c.g[A1][0] = 0x480000;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x20000000;
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00317d70, 0x0025fcec)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
    if c.g[V0][0] != 0u64 {
    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
    break 'b_25fcfc;
    } else {
    break 'b_25fcfc;
    }
    }
    c.cycles += 21;
    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[AT][0] = 0x41a00000;
    c.mtc1(1, 0);
    c.sw((c.g[A1][0] as u32), c.g[A0][0]);
    c.g[AT][0] = 0x41a80000;
    c.mtc1(1, 1);
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
    c.sw((c.g[V1][0] as u32), c.g[A1][0]);
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0xe0));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sw((c.g[SP][0] as u32), c.g[A1][0]);
    c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x2064));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_DJSay20E9 (0x00002a23d0).

/// `SSXAudioSystem_DJSay20E9`: make the DJ say the given line (event 0x20e9) unless the DJ is already talking.
pub fn ssx_audio_system_dj_say_20e9(c: &mut Cpu) {
    guest(c, |c| {
    'b_2a249c: {
    'b_2a2498: {
    c.cycles += 10;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = c.g[A0][0];
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S1][0] = c.g[A1][0];
    call_ptr(c, 0x0029f0d8, 0x002a23f0)?;
    let t = c.g[V0][0] == 0u64;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
    if t {
    break 'b_2a2498;
    } else {
    c.cycles += 10;
    c.mtc1(0, 12);
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0x20e9;
    c.g[A3][0] = 0u64;
    c.g[T0][0] = 0xa;
    c.g[T1][0] = 0u64;
    c.g[T2][0] = 0u64;
    call_ptr(c, 0x002b1458, 0x002a2418)?;
    if c.g[V0][0] == 0u64 {
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    break 'b_2a249c;
    } else {
    'b_2a2464: {
    'b_2a2460: {
    c.cycles += 6;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0029f198, 0x002a2428)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V0][0] = 0xffffffffffffffff;
    let t = c.g[S1][0] != c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    if t {
    break 'b_2a2460;
    } else {
    c.cycles += 10;
    call_ptr(c, 0x0028b1e8, 0x002a2440)?;
    c.g[A1][0] = 0u64;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014dd58, 0x002a244c)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x00144bc0, 0x002a2454)?;
    c.g[A1][0] = c.lw((c.g[V0][0] as u32));
    let t = true;
    c.g[A0][0] = c.g[S0][0];
    if t {
    break 'b_2a2464;
    } else {
    break 'b_2a2460;
    }
    }
    }
    c.cycles += 1;
    c.g[A1][0] = c.g[S1][0];
    break 'b_2a2464;
    }
    c.cycles += 13;
    call_ptr(c, 0x002a1e20, 0x002a246c)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 0x1;
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0x20e9;
    call_ptr(c, 0x003d8008, 0x002a2480)?;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x173c));
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.g[S0][0];
    let t = c.g[V1][0] as u32;
    c.g[A1][0] = 0x2;
    c.g[RA][0] = sx(0x002a2498);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x002a2498 { return Err(crate::call::Unwound); }
    break 'b_2a2498;
    }
    }
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    break 'b_2a249c;
    }
    c.cycles += 5;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// PS2SoundManager_Init (0x00002a8cb0).

/// `PS2SoundManager_Init`: allocate the voice buffer, register it with the PS2 sound library and initialise the 32 voice slots.
pub fn ps2_sound_manager_init(c: &mut Cpu) {
    guest(c, |c| {
    c.cycles += 34;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.g[A2][0] = 0u64;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[A3][0] = 0u64;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = c.g[A1][0];
    c.g[S1][0] = c.g[A0][0];
    c.g[A1][0] = 0x480000;
    c.g[A0][0] = 0x30000;
    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
    c.g[A0][0] = c.g[A0][0] | 0x2000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3158));
    call_ptr(c, 0x00317e30, 0x002a8ce4)?;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    call_ptr(c, 0x003b5478, 0x002a8cec)?;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x002a8d04);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x002a8d04 { return Err(crate::call::Unwound); }
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
    c.g[A1][0] = 0x30000;
    c.g[A2][0] = 0x80000;
    c.g[A1][0] = c.g[A1][0] | 0x2000;
    c.g[A2][0] = c.g[A2][0] | 0x303;
    call_ptr(c, 0x003b5158, 0x002a8d1c)?;
    c.g[V1][0] = 0xffffffffffffffff;
    c.g[V0][0] = 0x1f;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x318), c.g[V1][0]);
    c.g[A1][0] = 0xffffffffffffffff;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x310), c.g[V0][0]);
    c.g[A0][0] = 0x1f;
    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x308));
    'l_2a8d38: loop {
    c.cycles += 10;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x14), c.g[A0][0]);
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x10), c.g[A1][0]);
    c.g[A0][0] = c.g[V0][0];
    c.sw((c.g[V1][0] as u32).wrapping_sub(0xc), c.g[A1][0]);
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x8), 0u64);
    c.sw((c.g[V1][0] as u32).wrapping_sub(0x4), 0u64);
    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
    let t = (c.g[A0][0] as i64) >= 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x18));
    if t {
    if event_due(c) { deliver_event_here(c, 0x002a8d38); } continue 'l_2a8d38;
    } else {
    c.cycles += 12;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x31c), 0u64);
    c.g[A0][0] = c.g[SP][0];
    c.sw((c.g[SP][0] as u32), 0u64);
    call_ptr(c, 0x003b5b68, 0x002a8d70)?;
    c.g[V1][0] = c.lw((c.g[SP][0] as u32));
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
    c.sw((c.g[S1][0] as u32).wrapping_add(0x30c), c.g[V1][0]);
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
    }
    }
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// ScriptEngine_AddScriptObj (0x000027cea8).

/// `ScriptEngine_AddScriptObj`: register a script owner in the first free of 20 slots and add its bindings.
pub fn script_engine_add_script_obj(c: &mut Cpu) {
    guest(c, |c| {
    'b_27cf04: {
    'b_27cee0: {
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S1][0] = c.g[A1][0];
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[S4][0] = c.g[A0][0];
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[V0][0] = c.lw((c.g[S4][0] as u32));
    let t = c.g[V0][0] != 0u64;
    c.g[S3][0] = 0u64;
    if t {
    break 'b_27cee0;
    } else {
    c.cycles += 2;
    let t = true;
    c.sw((c.g[S4][0] as u32), c.g[S1][0]);
    if t {
    break 'b_27cf04;
    } else {
    break 'b_27cee0;
    }
    }
    }
    c.cycles += 1;
    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
    'l_27cee4: loop {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[S3][0] as i64) < 20i64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
    if t {
    break 'b_27cf04;
    } else {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
    if c.g[V1][0] != 0u64 {
    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
    if event_due(c) { deliver_event_here(c, 0x0027cee4); } continue 'l_27cee4;
    } else {
    c.cycles += 1;
    c.sw((c.g[V0][0] as u32), c.g[S1][0]);
    break 'b_27cf04;
    }
    }
    }
    }
    'b_27cf68: {
    c.cycles += 9;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
    c.g[S0][0] = 0u64;
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x0027cf1c);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x0027cf1c { return Err(crate::call::Unwound); }
    c.g[S2][0] = c.g[V0][0];
    let t = (c.g[S2][0] as i64) <= 0;
    c.g[V0][0] = c.g[S3][0];
    if t {
    break 'b_27cf68;
    } else {
    c.cycles += 2;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
    'l_27cf30: loop {
    c.cycles += 13;
    c.g[A1][0] = c.g[S0][0];
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x0027cf48);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x0027cf48 { return Err(crate::call::Unwound); }
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S3][0];
    call_ptr(c, 0x0027d330, 0x0027cf58)?;
    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
    if c.g[V1][0] != 0u64 {
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
    if event_due(c) { deliver_event_here(c, 0x0027cf30); } continue 'l_27cf30;
    } else {
    c.cycles += 1;
    c.g[V0][0] = c.g[S3][0];
    break 'b_27cf68;
    }
    }
    }
    }
    c.cycles += 8;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// BankMonitor_StopByBank (0x00002ad650).

/// `BankMonitor_StopByBank`: walk the 32 sound bank slots and fade/stop the instances that belong to the given bank.
pub fn bank_monitor_stop_by_bank(c: &mut Cpu) {
    guest(c, |c| {
    c.cycles += 18;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S0][0] = 0u64;
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S1][0] = c.g[A0][0];
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S2][0] = 0x3f;
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S3][0] = c.g[A1][0];
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[S4][0] = 0x1;
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.mov_s(20, 12);
    c.mtc1(0, 21);
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8ec));
    'l_2ad698: loop {
    'b_2ad700: {
    c.cycles += 5;
    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
    call_ptr(c, 0x002abb80, 0x002ad6a4)?;
    if c.g[V0][0] != 0u64 {
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
    break 'b_2ad700;
    } else {
    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8ec));
    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[V1][0] = c.lw((c.g[A3][0] as u32));
    if c.g[V1][0] != c.g[S4][0] {
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
    break 'b_2ad700;
    } else {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
    c.c_eq_s(20, 21);
    if c.g[V0][0] != c.g[S3][0] {
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
    break 'b_2ad700;
    } else {
    'b_2ad6f4: {
    c.cycles += 5;
    c.g[A0][0] = c.g[A3][0];
    c.mov_s(12, 20);
    c.g[A1][0] = 0xffffffffffffffff;
    let t = c.fcc();
    c.g[A2][0] = 0x1;
    if t {
    break 'b_2ad6f4;
    } else {
    c.cycles += 4;
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
    call_ptr(c, 0x002acb80, 0x002ad6ec)?;
    let t = true;

    if t {
    break 'b_2ad700;
    } else {
    break 'b_2ad6f4;
    }
    }
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[A3][0];
    call_ptr(c, 0x002abb38, 0x002ad6fc)?;
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
    break 'b_2ad700;
    }
    }
    }
    }
    c.cycles += 2;
    if (c.g[S2][0] as i64) >= 0 {
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8ec));
    if event_due(c) { deliver_event_here(c, 0x002ad698); } continue 'l_2ad698;
    } else {
    c.cycles += 10;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    c.pc = t; return Ok(());
    }
    }
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// InSeq_HeapFree (0x00002723a8).

/// `InSeq_HeapFree`: return a block to an InSeq heap, coalescing with the neighbouring free blocks.
pub fn in_seq_heap_free(c: &mut Cpu) {
    guest(c, |c| {
    'b_272480: {
    c.cycles += 2;
    let t = c.g[A1][0] == 0u64;

    if t {
    break 'b_272480;
    } else {
    'b_2723bc: {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    let t = c.g[V0][0] != 0u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x8));
    if t {
    c.cycles += 3;
    c.g[A2][0] = c.g[V0][0];
    c.g[T0][0] = 0u64;
    c.g[V0][0] = (c.g[A1][0] < c.g[A2][0]) as u64;
    'l_2723d0: loop {
    'b_272478: {
    'b_272470: {
    'b_27241c: {
    c.cycles += 2;
    if c.g[V0][0] == 0u64 {
    c.g[V0][0] = c.lw((c.g[A2][0] as u32));
    break 'b_27241c;
    } else {
    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    let t = c.g[V1][0] != c.g[A2][0];

    if t {
    c.cycles += 2;
    if c.g[T0][0] == 0u64 {
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    break 'b_272470;
    } else {
    c.cycles += 2;
    let t = true;
    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
    if t {
    break 'b_272478;
    } else {
    break 'b_27241c;
    }
    }
    } else {
    c.cycles += 6;
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[A2][0] as u32));
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    let t = c.g[T0][0] == 0u64;
    c.sw((c.g[A1][0] as u32), c.g[V0][0]);
    if t {
    break 'b_2723bc;
    } else {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    c.pc = t; return Ok(());
    }
    }
    }
    }
    'b_27245c: {
    'b_272458: {
    c.cycles += 4;
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    if c.g[V1][0] != c.g[A1][0] {
    c.g[T0][0] = c.g[A2][0];
    break 'b_272458;
    } else {
    'b_272450: {
    c.cycles += 7;
    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
    c.g[A1][0] = c.g[A2][0];
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[A2][0] as u32), c.g[V0][0]);
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    let t = c.g[T0][0] != 0u64;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), 0u64);
    if t {
    break 'b_272450;
    } else {
    c.cycles += 2;
    let t = true;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
    if t {
    break 'b_27245c;
    } else {
    break 'b_272450;
    }
    }
    }
    c.cycles += 2;
    let t = true;
    c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
    if t {
    break 'b_27245c;
    } else {
    break 'b_272458;
    }
    }
    }
    c.cycles += 1;
    c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
    break 'b_27245c;
    }
    c.cycles += 2;
    let t = c.g[A2][0] != 0u64;
    c.g[V0][0] = (c.g[A1][0] < c.g[A2][0]) as u64;
    if t {
    if event_due(c) { deliver_event_here(c, 0x002723d0); } continue 'l_2723d0;
    } else {
    c.cycles += 2;
    if c.g[T0][0] != 0u64 {
    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
    break 'b_272478;
    } else {
    c.cycles += 1;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    break 'b_272470;
    }
    }
    }
    c.cycles += 2;
    let t = true;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    if t {
    break 'b_2723bc;
    } else {
    break 'b_272478;
    }
    }
    c.cycles += 2;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    break 'b_272480;
    }
    } else {
    break 'b_2723bc;
    }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
    c.pc = t; return Ok(());
    }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// AI_ForceFinishRiders (0x000012b340).

/// `AI_ForceFinishRiders`: when the AI race state is 5, force every rider that has not finished to the end of the course.
pub fn ai_force_finish_riders(c: &mut Cpu) {
    guest(c, |c| {
    'b_12b400: {
    c.cycles += 13;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V1][0] = 0x5;
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[S3][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[V0][0] = c.lw((c.g[S3][0] as u32));
    let t = c.g[V0][0] != c.g[V1][0];
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    if t {
    break 'b_12b400;
    } else {
    'b_12b3fc: {
    c.cycles += 6;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
    c.g[S2][0] = 0u64;
    call_ptr(c, 0x00258950, 0x0012b380)?;
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x7c));
    let t = (c.g[V0][0] as i64) <= 0;
    c.g[S4][0] = 0x1;
    if t {
    break 'b_12b3fc;
    } else {
    c.cycles += 3;
    c.g[AT][0] = 0x41200000;
    c.mtc1(1, 20);
    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x40));
    'l_12b398: loop {
    'b_12b3ec: {
    c.cycles += 5;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32));
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x87c));
    if c.g[V0][0] != 0u64 {
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x7c));
    break 'b_12b3ec;
    } else {
    'b_12b3c4: {
    c.cycles += 5;
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x470));
    c.c_le_s(20, 0);
    let t = c.fcc();
    c.g[V0][0] = 0x1;
    if t {
    break 'b_12b3c4;
    } else {
    c.cycles += 1;
    c.g[V0][0] = 0u64;
    break 'b_12b3c4;
    }
    }
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    if t {
    break 'b_12b3fc;
    } else {
    c.cycles += 8;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
    call_ptr(c, 0x0026fa50, 0x0012b3d8)?;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x480), c.g[S4][0]);
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
    call_ptr(c, 0x00125108, 0x0012b3e8)?;
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x7c));
    break 'b_12b3ec;
    }
    }
    }
    c.cycles += 4;
    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let t = c.g[V0][0] != 0u64;
    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
    if t {
    if event_due(c) { deliver_event_here(c, 0x0012b398); } continue 'l_12b398;
    } else {
    break 'b_12b3fc;
    }
    }
    }
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    break 'b_12b400;
    }
    }
    c.cycles += 8;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// MCManager_BeginReplayWrite (0x00002412a0).

/// `MCManager_BeginReplayWrite`: compose the "please insert a memory card" text and start the replay write flow.
pub fn mc_manager_begin_replay_write(c: &mut Cpu) {
    guest(c, |c| {
    c.cycles += 56;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x590));
    c.g[V0][0] = 0x1;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x570));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x580));
    c.g[S1][0] = c.g[A0][0];
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x560));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x550));
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x540));
    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x530));
    c.sd((c.g[SP][0] as u32).wrapping_add(0x520), c.g[RA][0]);
    c.sw((c.g[GP][0] as u32).wrapping_sub(0x478), c.g[V0][0]);
    call_ptr(c, 0x0014dc80, 0x002412d0)?;
    c.g[A1][0] = 0x5;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014dd58, 0x002412dc)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x00152758, 0x002412e4)?;
    c.g[S5][0] = c.g[V0][0];
    call_ptr(c, 0x00152948, 0x002412ec)?;
    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x320));
    c.g[A1][0] = 0x480000;
    c.g[S4][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3e88));
    call_ptr(c, 0x002c2540, 0x00241304)?;
    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.g[A3][0] = 0x82;
    c.g[A2][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x00241d40, 0x0024131c)?;
    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
    c.g[A1][0] = 0x200;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x00241aa0, 0x0024132c)?;
    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf8));
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = c.g[S2][0];
    c.g[A0][0] = c.g[SP][0];
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
    call_ptr(c, 0x002c26d0, 0x00241348)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[A3][0] = c.g[S4][0];
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, 0x00241000, 0x0024135c)?;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x580));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x570));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x560));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x550));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x540));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x530));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x520));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x590));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cGameLoadState_Init (0x0000245418).

/// `cGameLoadState_Init`: initialise the game loader state: load the level file and set up its script and world.
pub fn c_game_load_state_init(c: &mut Cpu) {
    guest(c, |c| {
    'b_2454d0: {
    'b_2454cc: {
    'b_245494: {
    c.cycles += 14;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S1][0] = c.g[A0][0];
    call_ptr(c, 0x0014dc80, 0x00245430)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x0014dd58, 0x0024543c)?;
    c.g[V1][0] = 0x530000;
    c.g[A0][0] = 0x2;
    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
    if c.g[V0][0] != c.g[A0][0] {
    c.g[A0][0] = 0x480000;
    break 'b_245494;
    } else {
    c.cycles += 11;
    c.g[A0][0] = 0x480000;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x45d8));
    call_ptr(c, 0x00317670, 0x00245460)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00397cc0, 0x00245474)?;
    let t = c.g[V0][0] == 0u64;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
    if t {
    break 'b_2454cc;
    } else {
    c.cycles += 6;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0u64;
    call_ptr(c, 0x0039c870, 0x0024548c)?;
    let t = true;
    c.g[A0][0] = c.g[S1][0];
    if t {
    break 'b_2454d0;
    } else {
    break 'b_245494;
    }
    }
    }
    }
    c.cycles += 10;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x45c8));
    call_ptr(c, 0x00317670, 0x002454a0)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00397cc0, 0x002454b4)?;
    let t = c.g[V0][0] == 0u64;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
    if t {
    break 'b_2454cc;
    } else {
    c.cycles += 4;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0u64;
    call_ptr(c, 0x0039c870, 0x002454cc)?;
    break 'b_2454cc;
    }
    }
    c.cycles += 1;
    c.g[A0][0] = c.g[S1][0];
    break 'b_2454d0;
    }
    c.cycles += 10;
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x00246cf8, 0x002454d8)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0x1;
    call_ptr(c, 0x00246cf8, 0x002454e4)?;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// WScriptMan_GetHudTargetHeight (0x0000309ba8).

/// `WScriptMan_GetHudTargetHeight`: return the HUD target height for the current script target.
pub fn w_script_man_get_hud_target_height(c: &mut Cpu) {
    guest(c, |c| {
    'b_309c7c: {
    'b_309c74: {
    c.cycles += 7;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
    c.g[A1][0] = c.g[A0][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28));
    let t = c.g[V0][0] == 0u64;

    if t {
    break 'b_309c74;
    } else {
    'b_309c18: {
    'b_309bf4: {
    c.cycles += 10;
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28c));
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
    if c.g[V1][0] != 0u64 {
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
    break 'b_309bf4;
    } else {
    c.cycles += 2;
    let t = true;
    c.g[S0][0] = 0u64;
    if t {
    break 'b_309c18;
    } else {
    break 'b_309bf4;
    }
    }
    }
    'b_309c14: {
    c.cycles += 7;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    if t {
    break 'b_309c14;
    } else {
    c.cycles += 1;
    c.g[V0][0] = 0u64;
    break 'b_309c14;
    }
    }
    c.cycles += 1;
    c.g[S0][0] = c.g[V0][0];
    break 'b_309c18;
    }
    c.cycles += 23;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x30));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[RA][0] = sx(0x00309c4c);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x00309c4c { return Err(crate::call::Unwound); }
    c.lq(3, (c.g[S0][0] as u32).wrapping_add(0x40));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.sq(3, (c.g[SP][0] as u32));
    c.mtc1(0, 2);
    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.sub_s(0, 0, 1);
    c.c_le_s(2, 0);
    let t = c.fcc();
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    if t {
    break 'b_309c7c;
    } else {
    break 'b_309c74;
    }
    }
    }
    c.cycles += 2;
    c.mtc1(0, 0);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    break 'b_309c7c;
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// WScriptMan_CollectItem (0x000030b9a0).

/// `WScriptMan_CollectItem`: collect a world script item: add it to the inventory and fire its events.
pub fn w_script_man_collect_item(c: &mut Cpu) {
    guest(c, |c| {
    c.cycles += 8;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c0));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V1][0] = 0u64;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S2][0] = sx((c.g[A1][0] as u32) << 2);
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    'l_30b9c0: loop {
    'b_30ba6c: {
    'b_30ba60: {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
    let t = c.g[V0][0] != c.g[A2][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    if t {
    break 'b_30ba60;
    } else {
    c.cycles += 37;
    c.g[A1][0] = c.g[A3][0];
    call_ptr(c, 0x0030c3e0, 0x0030b9d4)?;
    call_ptr(c, 0x0028b180, 0x0030b9dc)?;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 12);
    c.g[A2][0] = 0u64;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x5;
    call_ptr(c, 0x0029ced8, 0x0030b9f4)?;
    call_ptr(c, 0x0014dc80, 0x0030b9fc)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x0014dd58, 0x0030ba08)?;
    c.g[S0][0] = c.g[V0][0];
    call_ptr(c, 0x0014dc80, 0x0030ba10)?;
    c.g[A1][0] = 0xb;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014dd58, 0x0030ba1c)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[S1][0] = c.g[V0][0];
    call_ptr(c, 0x00144c98, 0x0030ba28)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, 0x00151178, 0x0030ba38)?;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
    call_ptr(c, 0x0010f338, 0x0030ba58)?;
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    if t {
    break 'b_30ba6c;
    } else {
    break 'b_30ba60;
    }
    }
    }
    c.cycles += 2;
    let t = (c.g[V1][0] as i64) <= 0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10c));
    if t {
    if event_due(c) { deliver_event_here(c, 0x0030b9c0); } continue 'l_30b9c0;
    } else {
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    break 'b_30ba6c;
    }
    }
    c.cycles += 5;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
    }
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// PlantsMenu_dtor (0x000024f860).

/// `PlantsMenu_dtor`: destroy the plants menu: free its child objects and the base screen.
pub fn plants_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
    c.cycles += 56;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x398));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f884)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x364));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f890)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x330));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f89c)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fc));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8a8)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c8));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8b4)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x298));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8c0)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x264));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8cc)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x230));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8d8)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1fc));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8e4)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8f0)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f8fc)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f908)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f914)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002ca280, 0x0024f920)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x002caa80, 0x0024f92c)?;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// WScriptProcess_SaveState (0x0000308038).

/// `WScriptProcess_SaveState`: walk the process list and save each running script's state.
pub fn w_script_process_save_state(c: &mut Cpu) {
    guest(c, |c| {
    'b_3080c8: {
    c.cycles += 33;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[A2][0] = 0x10;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S2][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S3][0] = c.g[A1][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
    c.g[A1][0] = c.g[S2][0];
    c.g[V1][0] = c.lw((c.g[S3][0] as u32));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x00308074);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x00308074 { return Err(crate::call::Unwound); }
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    call_ptr(c, 0x003aca38, 0x00308084)?;
    c.g[A3][0] = c.lw((c.g[S3][0] as u32));
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
    c.g[A1][0] = c.g[SP][0];
    c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x8));
    c.g[A2][0] = 0x4;
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x003080b0);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x003080b0 { return Err(crate::call::Unwound); }
    c.g[V0][0] = c.lw((c.g[SP][0] as u32));
    let t = c.g[V0][0] == 0u64;
    c.g[A2][0] = c.g[S0][0];
    if t {
    break 'b_3080c8;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
    c.g[A1][0] = c.g[S3][0];
    call_ptr(c, 0x002257e0, 0x003080c8)?;
    break 'b_3080c8;
    }
    }
    'b_3080fc: {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
    let t = (c.g[V0][0] as i64) <= 0;
    c.g[S0][0] = 0u64;
    if t {
    break 'b_3080fc;
    } else {
    c.cycles += 1;
    c.g[S1][0] = 0u64;
    'l_3080d8: loop {
    c.cycles += 9;
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
    c.g[A1][0] = c.g[S3][0];
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
    call_ptr(c, 0x00308760, 0x003080ec)?;
    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let t = c.g[V0][0] != 0u64;
    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x44));
    if t {
    if event_due(c) { deliver_event_here(c, 0x003080d8); } continue 'l_3080d8;
    } else {
    break 'b_3080fc;
    }
    }
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
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// MemCardUI_TryNextSlot (0x00001d8160).

/// `MemCardUI_TryNextSlot`: advance the memory-card screen to the next usable slot.
pub fn mem_card_ui_try_next_slot(c: &mut Cpu) {
    guest(c, |c| {
    'b_1d8234: {
    'b_1d8210: {
    'b_1d8208: {
    'b_1d81f0: {
    'b_1d81a8: {
    c.cycles += 13;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    call_ptr(c, 0x00227f80, 0x001d8178)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x428));
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x424));
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V0][0] == 0u64 {
    c.g[A0][0] = c.g[S0][0];
    break 'b_1d81a8;
    } else {
    c.cycles += 5;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0023fb18, 0x001d819c)?;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
    let t = true;
    c.g[A1][0] = 0x31;
    if t {
    break 'b_1d81f0;
    } else {
    break 'b_1d81a8;
    }
    }
    }
    c.cycles += 11;
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x0023fb18, 0x001d81b0)?;
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
    c.g[A1][0] = 0u64;
    c.g[V1][0] = c.lw((c.g[A2][0] as u32));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a0));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1a4));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x001d81cc);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001d81cc { return Err(crate::call::Unwound); }
    let t = c.g[V0][0] != 0u64;

    if t {
    break 'b_1d8210;
    } else {
    c.cycles += 4;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x208));
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0x6;
    if t {
    break 'b_1d8208;
    } else {
    c.cycles += 3;
    c.g[A1][0] = 0x2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1c0), c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
    break 'b_1d81f0;
    }
    }
    }
    c.cycles += 6;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x001d8200);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001d8200 { return Err(crate::call::Unwound); }
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    if t {
    break 'b_1d8234;
    } else {
    break 'b_1d8208;
    }
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x00241400, 0x001d8210)?;
    break 'b_1d8210;
    }
    c.cycles += 9;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0023fae0, 0x001d8218)?;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
    c.g[A1][0] = 0x36;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x001d8230);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001d8230 { return Err(crate::call::Unwound); }
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    break 'b_1d8234;
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cBXString_ConcatInPlace (0x0000318830).

/// `cBXString_ConcatInPlace`: append a C string to a cBXString in place, growing the buffer when needed.
pub fn c_bx_string_concat_in_place(c: &mut Cpu) {
    guest(c, |c| {
    'b_3188f8: {
    'b_3188c4: {
    'b_318880: {
    c.cycles += 14;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S1][0] = c.g[A0][0];
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S2][0] = c.g[A1][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[S3][0] = c.g[A2][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
    c.g[A3][0] = c.lw((c.g[S1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0xc));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xc));
    if t {
    break 'b_318880;
    } else {
    c.cycles += 6;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x8));
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V1][0] == 0u64 {
    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    break 'b_3188c4;
    } else {
    break 'b_318880;
    }
    }
    }
    c.cycles += 17;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x00317fe8, 0x0031888c)?;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32));
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003180b8, 0x00318898)?;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x8));
    c.g[A0][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S2][0];
    c.g[T0][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S0][0];
    call_ptr(c, 0x00318540, 0x003188b0)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x00318260, 0x003188bc)?;
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    if t {
    break 'b_3188f8;
    } else {
    break 'b_3188c4;
    }
    }
    c.cycles += 13;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    call_ptr(c, 0x0041605c, 0x003188d4)?;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
    c.sw((c.g[A0][0] as u32).wrapping_sub(0x8), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sb((c.g[V1][0] as u32), 0u64);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    break 'b_3188f8;
    }
    c.cycles += 6;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cPS2ChunkResolve_v15 (0x00003aa608).

/// `cPS2ChunkResolve_v15`: resolve a PlayStation 2 chunk descriptor (version 15) to data pointers.
pub fn c_ps2_chunk_resolve_v15(c: &mut Cpu) {
    guest(c, |c| {
    'b_3aa6c0: {
    c.cycles += 13;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S1][0] = c.g[A0][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[S2][0] = c.g[A1][0];
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
    let t = (c.g[V1][0] as i64) <= 0;
    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
    if t {
    break 'b_3aa6c0;
    } else {
    c.cycles += 5;
    c.g[S4][0] = 0xffffffffffff0000;
    c.g[V0][0] = 0x500000;
    c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
    c.g[S3][0] = c.g[V1][0];
    c.g[S4][0] = c.g[S4][0] | 0xffff;
    'l_3aa650: loop {
    'b_3aa674: {
    c.cycles += 3;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    let t = c.g[A1][0] == 0u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
    if t {
    break 'b_3aa674;
    } else {
    c.cycles += 6;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x80));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x003aa674);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x003aa674 { return Err(crate::call::Unwound); }
    break 'b_3aa674;
    }
    }
    'b_3aa68c: {
    'b_3aa688: {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
    let t = c.g[V0][0] != c.g[S4][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
    if t {
    break 'b_3aa688;
    } else {
    c.cycles += 2;
    let t = true;
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[S5][0]);
    if t {
    break 'b_3aa68c;
    } else {
    break 'b_3aa688;
    }
    }
    }
    c.cycles += 1;
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
    break 'b_3aa68c;
    }
    'b_3aa6b4: {
    c.cycles += 3;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    let t = c.g[A1][0] == 0u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
    if t {
    break 'b_3aa6b4;
    } else {
    c.cycles += 7;
    c.g[A2][0] = 0u64;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x003aa6b4);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x003aa6b4 { return Err(crate::call::Unwound); }
    break 'b_3aa6b4;
    }
    }
    c.cycles += 3;
    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
    let t = c.g[S3][0] != 0u64;
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
    if t {
    if event_due(c) { deliver_event_here(c, 0x003aa650); } continue 'l_3aa650;
    } else {
    break 'b_3aa6c0;
    }
    }
    }
    }
    c.cycles += 10;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[V0][0] = 0x1;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// ModifierBlock_HandleEvent (0x0000352f40).

/// `ModifierBlock_HandleEvent`: dispatch an event to the modifier block's handlers.
pub fn modifier_block_handle_event(c: &mut Cpu) {
    guest(c, |c| {
    'b_352f7c: {
    c.cycles += 10;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S0][0] = c.g[A0][0];
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S1][0] = c.g[A1][0];
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[A2][0] = c.lw((c.g[S0][0] as u32));
    let t = c.g[A2][0] == 0u64;
    c.mov_s(20, 12);
    if t {
    break 'b_352f7c;
    } else {
    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[A2][0] as u32));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x00352f7c);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x00352f7c { return Err(crate::call::Unwound); }
    break 'b_352f7c;
    }
    }
    'b_352f90: {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    let t = c.g[A0][0] == 0u64;
    c.g[A1][0] = c.g[S1][0];
    if t {
    break 'b_352f90;
    } else {
    c.cycles += 2;
    c.mov_s(12, 20);
    call_ptr(c, 0x0035fb30, 0x00352f90)?;
    break 'b_352f90;
    }
    }
    'b_352fa4: {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    let t = c.g[A0][0] == 0u64;
    c.g[A1][0] = c.g[S1][0];
    if t {
    break 'b_352fa4;
    } else {
    c.cycles += 2;
    c.mov_s(12, 20);
    call_ptr(c, 0x0035f598, 0x00352fa4)?;
    break 'b_352fa4;
    }
    }
    'b_352fc8: {
    c.cycles += 3;
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
    let t = c.g[A2][0] == 0u64;
    c.g[A1][0] = c.g[S1][0];
    if t {
    break 'b_352fc8;
    } else {
    c.cycles += 6;
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
    c.mov_s(12, 20);
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x00352fc8);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x00352fc8 { return Err(crate::call::Unwound); }
    break 'b_352fc8;
    }
    }
    'b_35300c: {
    c.cycles += 3;
    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
    if c.g[S0][0] == 0u64 {
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    break 'b_35300c;
    } else {
    c.cycles += 3;
    c.g[A2][0] = c.g[S0][0];
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
    'l_352fe0: loop {
    c.cycles += 10;
    c.g[A1][0] = c.g[S1][0];
    c.g[S0][0] = c.lw((c.g[A2][0] as u32));
    c.mov_s(12, 20);
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x00352ffc);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x00352ffc { return Err(crate::call::Unwound); }
    c.g[A2][0] = c.g[S0][0];
    if c.g[A2][0] != 0u64 {
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
    if event_due(c) { deliver_event_here(c, 0x00352fe0); } continue 'l_352fe0;
    } else {
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    break 'b_35300c;
    }
    }
    }
    }
    c.cycles += 5;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cFELoadState_Init (0x0000245b80).

/// `cFELoadState_Init`: initialise the front-end load state: kick off the load and set the progress bar.
pub fn c_fe_load_state_init(c: &mut Cpu) {
    guest(c, |c| {
    'b_245c50: {
    'b_245c4c: {
    'b_245c14: {
    c.cycles += 14;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S1][0] = c.g[A0][0];
    call_ptr(c, 0x0014dc80, 0x00245b98)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x0014dd58, 0x00245ba4)?;
    c.g[V1][0] = 0x530000;
    c.g[A0][0] = 0x2;
    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
    if c.g[V0][0] != c.g[A0][0] {
    c.g[A0][0] = 0x480000;
    break 'b_245c14;
    } else {
    c.cycles += 11;
    c.g[A0][0] = 0x480000;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x41c0));
    call_ptr(c, 0x00317670, 0x00245bc8)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00397cc0, 0x00245bdc)?;
    let t = c.g[V0][0] == 0u64;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
    if t {
    break 'b_245c4c;
    } else {
    c.cycles += 12;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0u64;
    call_ptr(c, 0x0039c870, 0x00245bf4)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x002472c0, 0x00245c00)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0x1;
    call_ptr(c, 0x002472c0, 0x00245c0c)?;
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    if t {
    break 'b_245c50;
    } else {
    break 'b_245c14;
    }
    }
    }
    }
    c.cycles += 10;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x41b0));
    call_ptr(c, 0x00317670, 0x00245c20)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00397cc0, 0x00245c34)?;
    let t = c.g[V0][0] == 0u64;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
    if t {
    break 'b_245c4c;
    } else {
    c.cycles += 4;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0u64;
    call_ptr(c, 0x0039c870, 0x00245c4c)?;
    break 'b_245c4c;
    }
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    break 'b_245c50;
    }
    c.cycles += 4;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cFEStateAudioOptions_Init (0x0000196148).

/// `cFEStateAudioOptions_Init`: initialise the audio options screen: build its sliders and read the current mix levels.
pub fn c_fe_state_audio_options_init(c: &mut Cpu) {
    guest(c, |c| {
    'b_1961dc: {
    'b_1961d8: {
    'b_1961c0: {
    c.cycles += 9;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
    let t = c.g[V0][0] == 0u64;
    c.g[S1][0] = c.g[A0][0];
    if t {
    break 'b_1961c0;
    } else {
    c.cycles += 13;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = 0x460000;
    c.g[A0][0] = 0x4a0000;
    c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x400));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18a8));
    call_ptr(c, 0x00317670, 0x00196188)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039f9d8, 0x00196194)?;
    c.g[A3][0] = c.g[V0][0];
    let t = c.g[A3][0] == 0u64;
    c.g[A1][0] = 0x3;
    if t {
    break 'b_1961d8;
    } else {
    c.cycles += 8;
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.g[A2][0] = 0u64;
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x001961b8);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001961b8 { return Err(crate::call::Unwound); }
    let t = true;
    c.g[A0][0] = c.g[S2][0];
    if t {
    break 'b_1961dc;
    } else {
    break 'b_1961c0;
    }
    }
    }
    }
    c.cycles += 6;
    c.g[V0][0] = 0x460000;
    c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x410));
    call_ptr(c, 0x0028b180, 0x001961cc)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x4;
    call_ptr(c, 0x0028f140, 0x001961d8)?;
    break 'b_1961d8;
    }
    c.cycles += 1;
    c.g[A0][0] = c.g[S2][0];
    break 'b_1961dc;
    }
    'b_196210: {
    c.cycles += 9;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
    call_ptr(c, 0x00317670, 0x001961e4)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00397cc0, 0x001961f8)?;
    let t = c.g[V0][0] == 0u64;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
    if t {
    break 'b_196210;
    } else {
    c.cycles += 4;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0u64;
    call_ptr(c, 0x0039c870, 0x00196210)?;
    break 'b_196210;
    }
    }
    c.cycles += 6;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// RollerModifier_CopyMatrixBlock (0x0000327cc8).

/// `RollerModifier_CopyMatrixBlock`: copy eight quadwords plus a 0xc0-byte matrix block, using unaligned load/store pairs when needed.
pub fn roller_modifier_copy_matrix_block(c: &mut Cpu) {
    guest(c, |c| {
    c.cycles += 4;
    c.g[A2][0] = c.g[A0][0];
    c.g[V1][0] = c.g[A1][0];
    c.g[A3][0] = 0x7;
    'l_327cd8: loop {
    c.cycles += 7;
    c.lq(2, (c.g[V1][0] as u32));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
    c.sq(2, (c.g[A2][0] as u32));
    let t = (c.g[A3][0] as i64) >= 0;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
    if t {
    if event_due(c) { deliver_event_here(c, 0x00327cd8); } continue 'l_327cd8;
    } else {
    'b_327da0: {
    'b_327d6c: {
    c.cycles += 8;
    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc0));
    c.lq(3, (c.g[A1][0] as u32).wrapping_add(0x80));
    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc0));
    c.g[T0][0] = c.g[A3][0];
    c.g[V0][0] = c.g[A2][0] | c.g[A3][0];
    c.g[V0][0] = c.g[V0][0] & 0x7;
    let t = c.g[V0][0] == 0u64;
    c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x80));
    if t {
    break 'b_327d6c;
    } else {
    c.cycles += 1;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x140));
    'l_327d18: loop {
    c.cycles += 19;
    c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
    c.ldr(2, (c.g[A2][0] as u32));
    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0xf));
    c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.ldl(9, (c.g[A2][0] as u32).wrapping_add(0x17));
    c.ldr(9, (c.g[A2][0] as u32).wrapping_add(0x10));
    c.ldl(10, (c.g[A2][0] as u32).wrapping_add(0x1f));
    c.ldr(10, (c.g[A2][0] as u32).wrapping_add(0x18));
    c.sdl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
    c.sdr(2, (c.g[A3][0] as u32));
    c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0xf));
    c.sdr(3, (c.g[A3][0] as u32).wrapping_add(0x8));
    c.sdl(9, (c.g[A3][0] as u32).wrapping_add(0x17));
    c.sdr(9, (c.g[A3][0] as u32).wrapping_add(0x10));
    c.sdl(10, (c.g[A3][0] as u32).wrapping_add(0x1f));
    c.sdr(10, (c.g[A3][0] as u32).wrapping_add(0x18));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
    let t = c.g[A2][0] != c.g[A1][0];
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
    if t {
    if event_due(c) { deliver_event_here(c, 0x00327d18); } continue 'l_327d18;
    } else {
    c.cycles += 2;
    let t = true;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x98), c.g[T0][0]);
    if t {
    break 'b_327da0;
    } else {
    break 'b_327d6c;
    }
    }
    }
    }
    }
    c.cycles += 1;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x140));
    'l_327d70: loop {
    c.cycles += 11;
    c.g[V0][0] = c.ld((c.g[A2][0] as u32));
    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
    c.g[T2][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
    c.sd((c.g[A3][0] as u32), c.g[V0][0]);
    c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[T1][0]);
    c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[T2][0]);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
    let t = c.g[A2][0] != c.g[A1][0];
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
    if t {
    if event_due(c) { deliver_event_here(c, 0x00327d70); } continue 'l_327d70;
    } else {
    c.cycles += 1;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x98), c.g[T0][0]);
    break 'b_327da0;
    }
    }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A0][0];
    c.pc = t; return Ok(());
    }
    }
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// OVScreenReplayMenu_OnWidgetCreated (0x000020de30).

/// `OVScreenReplayMenu_OnWidgetCreated`: wire the replay menu widgets by name.
pub fn ov_screen_replay_menu_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
    'b_20df00: {
    'b_20defc: {
    'b_20dec4: {
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
    c.g[A0][0] = 0x470000;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1bf0));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S1][0] = c.g[A1][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
    call_ptr(c, 0x00317670, 0x0020de54)?;
    let t = c.g[S0][0] != c.g[V0][0];
    c.g[A0][0] = 0x470000;
    if t {
    break 'b_20dec4;
    } else {
    c.cycles += 7;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x608));
    let t = c.g[V0][0] == 0u64;
    c.g[A1][0] = 0u64;
    if t {
    break 'b_20defc;
    } else {
    c.cycles += 19;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x0020de8c);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x0020de8c { return Err(crate::call::Unwound); }
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
    c.g[A1][0] = 0x1;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x0020dea4);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x0020dea4 { return Err(crate::call::Unwound); }
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
    c.g[A1][0] = 0u64;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x0020debc);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x0020debc { return Err(crate::call::Unwound); }
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    if t {
    break 'b_20df00;
    } else {
    break 'b_20dec4;
    }
    }
    }
    }
    'b_20def8: {
    'b_20dee0: {
    c.cycles += 5;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c08));
    call_ptr(c, 0x00317670, 0x0020ded0)?;
    if c.g[S0][0] != c.g[V0][0] {
    c.g[A0][0] = 0x470000;
    break 'b_20dee0;
    } else {
    c.cycles += 2;
    let t = true;
    c.g[V0][0] = 0x1;
    if t {
    break 'b_20def8;
    } else {
    break 'b_20dee0;
    }
    }
    }
    c.cycles += 5;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c20));
    call_ptr(c, 0x00317670, 0x0020deec)?;
    let t = c.g[S0][0] != c.g[V0][0];
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    if t {
    break 'b_20df00;
    } else {
    c.cycles += 1;
    c.g[V0][0] = 0x2;
    break 'b_20def8;
    }
    }
    c.cycles += 1;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
    break 'b_20defc;
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    break 'b_20df00;
    }
    c.cycles += 4;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// MCManager_State0A_Format (0x000023ef68).

/// `MCManager_State0A_Format`: memory-card manager state 0xa: format the card and report the result.
pub fn mc_manager_state0a_format(c: &mut Cpu) {
    guest(c, |c| {
    'b_23f040: {
    'b_23f004: {
    c.cycles += 7;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S0][0] = c.g[A0][0];
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x338));
    if c.g[V0][0] != 0u64 {
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
    break 'b_23f004;
    } else {
    'b_23f000: {
    'b_23efe8: {
    'b_23efb0: {
    c.cycles += 8;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x0023ef9c);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x0023ef9c { return Err(crate::call::Unwound); }
    if c.g[V0][0] != 0u64 {
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
    break 'b_23efb0;
    } else {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
    let t = true;
    c.g[A1][0] = 0x4;
    if t {
    break 'b_23efe8;
    } else {
    break 'b_23efb0;
    }
    }
    }
    'b_23efd4: {
    c.cycles += 7;
    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x0023efc4);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x0023efc4 { return Err(crate::call::Unwound); }
    let t = c.g[V0][0] == 0u64;
    c.g[A1][0] = 0xc;
    if t {
    break 'b_23efd4;
    } else {
    c.cycles += 2;
    let t = true;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
    if t {
    break 'b_23efe8;
    } else {
    break 'b_23efd4;
    }
    }
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc4));
    let t = c.g[V0][0] == 0u64;
    c.g[A1][0] = 0xb;
    if t {
    break 'b_23f000;
    } else {
    c.cycles += 2;
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
    break 'b_23efe8;
    }
    }
    c.cycles += 6;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x0023eff8);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x0023eff8 { return Err(crate::call::Unwound); }
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    if t {
    break 'b_23f040;
    } else {
    break 'b_23f000;
    }
    }
    c.cycles += 1;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
    break 'b_23f004;
    }
    }
    'b_23f024: {
    c.cycles += 4;
    let t = c.g[V0][0] as u32;
    c.g[RA][0] = sx(0x0023f00c);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x0023f00c { return Err(crate::call::Unwound); }
    let t = c.g[V0][0] == 0u64;
    c.g[A0][0] = c.g[S0][0];
    if t {
    break 'b_23f024;
    } else {
    c.cycles += 4;
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x0023ea30, 0x0023f01c)?;
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    if t {
    break 'b_23f040;
    } else {
    break 'b_23f024;
    }
    }
    }
    'b_23f03c: {
    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
    let t = c.g[V0][0] as u32;
    c.g[RA][0] = sx(0x0023f030);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x0023f030 { return Err(crate::call::Unwound); }
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = 0x1;
    if t {
    break 'b_23f03c;
    } else {
    c.cycles += 1;
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
    break 'b_23f03c;
    }
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    break 'b_23f040;
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t; return Ok(());
        Ok(())
    });
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
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
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
    if c.g[V0][0] != 0 { c.g[A2][0] = c.g[S2][0]; }
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
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// PS2_SONY_CODEC_INTERNAL_InitFrame (0x00003b07f8).

/// `PS2_SONY_CODEC_INTERNAL_InitFrame`: initialise one Sony codec frame descriptor.
pub fn ps2_sony_codec_internal_init_frame(c: &mut Cpu) {
    guest(c, |c| {
    'b_3b08ac: {
    'b_3b08a8: {
    c.cycles += 41;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.g[V1][0] = 0x3;
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = 0x2;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S2][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[A3][0] = 0x510000;
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S0][0] = c.g[A1][0];
    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V1][0]);
    c.g[S1][0] = c.g[A2][0];
    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[A0][0] = c.g[S0][0];
    c.sw((c.g[S2][0] as u32), c.g[V0][0]);
    c.g[A1][0] = c.g[S1][0];
    c.sw((c.g[S2][0] as u32).wrapping_add(0x10), 0u64);
    c.g[A2][0] = 0x20;
    c.g[T0][0] = 0u64;
    c.g[T2][0] = 0u64;
    c.g[T1][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x6bc8));
    c.g[T3][0] = 0xf0;
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x003b2a68, 0x003b0858)?;
    c.g[A0][0] = c.g[V0][0];
    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    call_ptr(c, 0x003b3d40, 0x003b0864)?;
    c.g[A0][0] = 0x32;
    c.g[A1][0] = 0x34;
    c.g[V1][0] = 0x30;
    c.sb((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
    c.sb((c.g[V0][0] as u32).wrapping_add(0x6), c.g[V1][0]);
    c.g[S0][0] = sx((c.g[S0][0] as u32) >> 4);
    c.sb((c.g[V0][0] as u32).wrapping_add(0x5), c.g[A1][0]);
    c.mult(0, 16, 16, 17);
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    c.g[S0][0] = sx((c.g[S0][0] as u32) >> 4);
    c.g[V0][0] = c.g[V0][0] & 0x1000;
    let t = c.g[V0][0] == 0u64;
    c.g[A2][0] = sx((c.g[S0][0] as u32) << 10);
    if t {
    break 'b_3b08a8;
    } else {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
    let t = true;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    if t {
    break 'b_3b08ac;
    } else {
    break 'b_3b08a8;
    }
    }
    }
    c.cycles += 1;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    break 'b_3b08ac;
    }
    c.cycles += 12;
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x003e6448, 0x003b08b4)?;
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[S2][0];
    c.g[V1][0] = 0x5;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.sb((c.g[A0][0] as u32), c.g[V1][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cFEStateRewardGallery_CheckRewardsReady (0x00001d0238).

/// `cFEStateRewardGallery_CheckRewardsReady`: check whether all reward icons in the gallery have finished loading.
pub fn c_fe_state_reward_gallery_check_rewards_ready(c: &mut Cpu) {
    guest(c, |c| {
    'b_1d0310: {
    c.cycles += 9;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S0][0] = c.g[A0][0];
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
    c.g[V0][0] = c.g[V0][0] & 0x1;
    if c.g[V0][0] != 0u64 {
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    break 'b_1d0310;
    } else {
    'b_1d030c: {
    c.cycles += 9;
    call_ptr(c, 0x0014dc80, 0x001d0264)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xd;
    call_ptr(c, 0x0014dd58, 0x001d0270)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x00156a38, 0x001d0278)?;
    let t = c.g[V0][0] == 0u64;
    c.g[V1][0] = 0xfffffffffffffffe;
    if t {
    break 'b_1d030c;
    } else {
    'b_1d02b4: {
    c.cycles += 13;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] | 0x2;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
    c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0xe0));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe4));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x001d02a4);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001d02a4 { return Err(crate::call::Unwound); }
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
    if c.g[A0][0] == 0u64 {
    break 'b_1d02b4;
    } else {
    break 'b_1d02b4;
    }
    }
    c.cycles += 9;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
    c.div(0, 2, 4);
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
    c.g[V1][0] = c.g[V1][0] & 0x1;
    c.g[V0][0] = c.lo[0];
    let t = c.g[V1][0] == 0u64;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[V0][0]);
    if t {
    break 'b_1d030c;
    } else {
    c.cycles += 13;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x924));
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x8fc), c.g[V0][0]);
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x140));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x144));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x001d02f4);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001d02f4 { return Err(crate::call::Unwound); }
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    c.g[A1][0] = 0u64;
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x108));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x001d030c);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001d030c { return Err(crate::call::Unwound); }
    break 'b_1d030c;
    }
    }
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    break 'b_1d0310;
    }
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cFEStateCharEquip_CategoryAllFlagged (0x000019b098).

/// `cFEStateCharEquip_CategoryAllFlagged`: report whether every item of a character-equipment category has the given flag.
pub fn c_fe_state_char_equip_category_all_flagged(c: &mut Cpu) {
    guest(c, |c| {
    'b_19b168: {
    'b_19b164: {
    c.cycles += 16;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x870));
    c.g[T0][0] = 0xffffffffffffffff;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x850));
    c.g[T1][0] = 0x1;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x860));
    c.g[S1][0] = c.g[A0][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x840), c.g[RA][0]);
    c.g[A0][0] = c.g[A1][0];
    c.g[S0][0] = c.g[A2][0];
    c.g[A2][0] = c.g[A3][0];
    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xbc));
    c.g[A3][0] = c.g[SP][0];
    call_ptr(c, 0x0014d7e8, 0x0019b0cc)?;
    c.g[A3][0] = c.g[V0][0];
    let t = (c.g[A3][0] as i64) <= 0;
    c.g[A2][0] = 0u64;
    if t {
    break 'b_19b164;
    } else {
    c.cycles += 2;
    c.g[A1][0] = c.g[SP][0];
    'l_19b0e0: loop {
    'b_19b158: {
    c.cycles += 5;
    c.g[A0][0] = c.lw((c.g[A1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
    c.g[V0][0] = c.g[V0][0] & 0x4;
    if c.g[V0][0] == 0u64 {
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
    break 'b_19b158;
    } else {
    'b_19b120: {
    'b_19b11c: {
    c.cycles += 7;
    c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x288));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[V0][0] = c.lh((c.g[V0][0] as u32));
    let t = (c.g[V0][0] as i64) < 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    if t {
    break 'b_19b11c;
    } else {
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
    let t = true;
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    if t {
    break 'b_19b120;
    } else {
    break 'b_19b11c;
    }
    }
    }
    c.cycles += 1;
    c.g[V0][0] = 0u64;
    break 'b_19b120;
    }
    c.cycles += 4;
    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
    c.g[V0][0] = c.g[V0][0] & 0x2;
    if c.g[V0][0] != 0u64 {
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
    break 'b_19b158;
    } else {
    c.cycles += 4;
    c.g[V1][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x2));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xcc));
    if c.g[V1][0] != c.g[V0][0] {
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
    break 'b_19b158;
    } else {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
    c.g[V0][0] = c.g[V0][0] & 0x20;
    if c.g[V0][0] != 0u64 {
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
    break 'b_19b158;
    } else {
    c.cycles += 2;
    let t = true;
    c.g[V0][0] = 0u64;
    if t {
    break 'b_19b168;
    } else {
    break 'b_19b158;
    }
    }
    }
    }
    }
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
    let t = c.g[V0][0] != 0u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
    if t {
    if event_due(c) { deliver_event_here(c, 0x0019b0e0); } continue 'l_19b0e0;
    } else {
    break 'b_19b164;
    }
    }
    }
    }
    c.cycles += 1;
    c.g[V0][0] = 0x1;
    break 'b_19b168;
    }
    c.cycles += 5;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x860));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x850));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x840));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x870));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// WScriptMan_Restart (0x0000309030).

/// `WScriptMan_Restart`: restart the world script manager: reset the script slots and load the startup scripts.
pub fn w_script_man_restart(c: &mut Cpu) {
    guest(c, |c| {
    'b_3090f4: {
    c.cycles += 24;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[S0][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b8));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S1][0] = 0u64;
    call_ptr(c, 0x0030b1b8, 0x0030905c)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2bc));
    call_ptr(c, 0x0030b1b8, 0x00309064)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b4));
    call_ptr(c, 0x0030b1b8, 0x0030906c)?;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x00308988, 0x00309074)?;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28c));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    call_ptr(c, 0x003a9800, 0x00309084)?;
    c.g[S2][0] = c.g[V0][0];
    let t = (c.g[S2][0] as i64) <= 0;
    c.g[S4][0] = 0x1;
    if t {
    break 'b_3090f4;
    } else {
    c.cycles += 2;
    c.g[S3][0] = 0u64;
    'l_309098: loop {
    'b_3090c4: {
    c.cycles += 8;
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28c));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[V0][0] = c.lh((c.g[V1][0] as u32));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
    let t = c.g[V0][0] != 0u64;
    c.g[A1][0] = 0u64;
    if t {
    break 'b_3090c4;
    } else {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[A1][0] = c.g[S4][0];
    if c.g[V0][0] == 0 { c.g[A1][0] = 0u64; }
    break 'b_3090c4;
    }
    }
    'b_3090e8: {
    'b_3090dc: {
    c.cycles += 2;
    let t = c.g[A1][0] == 0u64;
    c.g[A0][0] = c.g[S0][0];
    if t {
    break 'b_3090dc;
    } else {
    c.cycles += 4;
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x00308be0, 0x003090d4)?;
    let t = true;
    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
    if t {
    break 'b_3090e8;
    } else {
    break 'b_3090dc;
    }
    }
    }
    c.cycles += 3;
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x00308f70, 0x003090e4)?;
    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
    break 'b_3090e8;
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
    let t = c.g[V0][0] != 0u64;
    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0xc));
    if t {
    if event_due(c) { deliver_event_here(c, 0x00309098); } continue 'l_309098;
    } else {
    break 'b_3090f4;
    }
    }
    }
    }
    c.cycles += 8;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// RiderMetrix_OnSplitMarker (0x00001194c0).

/// `RiderMetrix_OnSplitMarker`: record a split time for the rider and update the split HUD.
pub fn rider_metrix_on_split_marker(c: &mut Cpu) {
    guest(c, |c| {
    'b_119588: {
    c.cycles += 16;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S1][0] = c.g[A0][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S2][0] = c.g[A1][0];
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[RA][0] = sx(0x001194f8);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001194f8 { return Err(crate::call::Unwound); }
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    if t {
    break 'b_119588;
    } else {
    c.cycles += 14;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac));
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x38));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x3c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[RA][0] = sx(0x00119520);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x00119520 { return Err(crate::call::Unwound); }
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, 0x00238510, 0x00119530)?;
    let t = c.g[V0][0] == 0u64;
    c.g[A0][0] = c.g[S1][0];
    if t {
    break 'b_119588;
    } else {
    'b_119568: {
    c.cycles += 10;
    c.g[AT][0] = 0x40200000;
    c.mtc1(1, 12);
    c.g[A2][0] = c.g[S2][0];
    c.g[A1][0] = 0x29;
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00117b88, 0x00119550)?;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x874));
    let t = c.g[V0][0] == 0u64;
    c.g[A0][0] = 0u64;
    if t {
    break 'b_119568;
    } else {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x87c));
    c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
    break 'b_119568;
    }
    }
    c.cycles += 2;
    let t = c.g[A0][0] == 0u64;

    if t {
    break 'b_119588;
    } else {
    c.cycles += 6;
    call_ptr(c, 0x0028b180, 0x00119578)?;
    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac));
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = 0x1;
    call_ptr(c, 0x002a3ce8, 0x00119588)?;
    break 'b_119588;
    }
    }
    }
    }
    c.cycles += 7;
    c.mtc1(0, 0);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// MemCardStateBase_Shutdown (0x00001d58b8).

/// `MemCardStateBase_Shutdown`: shut the memory-card state base down: close the dialogs and free the state.
pub fn mem_card_state_base_shutdown(c: &mut Cpu) {
    guest(c, |c| {
    'b_1d591c: {
    c.cycles += 14;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S1][0] = c.g[A0][0];
    call_ptr(c, 0x0014dc80, 0x001d58d0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x5;
    call_ptr(c, 0x0014dd58, 0x001d58dc)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1bc));
    c.g[V0][0] = 0x1;
    let t = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = 0x2;
    if t {
    break 'b_1d591c;
    } else {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x228));
    let t = c.g[V1][0] != c.g[V0][0];

    if t {
    break 'b_1d591c;
    } else {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b4));
    let t = c.g[V0][0] != 0u64;

    if t {
    break 'b_1d591c;
    } else {
    c.cycles += 5;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x00152728, 0x001d5910)?;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x1d4), 0u64);
    c.sw((c.g[S1][0] as u32).wrapping_add(0x1d0), 0u64);
    c.sw((c.g[S1][0] as u32).wrapping_add(0x228), 0u64);
    break 'b_1d591c;
    }
    }
    }
    }
    'b_1d5958: {
    c.cycles += 8;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x00152700, 0x001d5924)?;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    call_ptr(c, 0x00227f80, 0x001d592c)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
    if c.g[V0][0] == 0u64 {
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x228));
    break 'b_1d5958;
    } else {
    c.cycles += 7;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002410a0, 0x001d5944)?;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0023d5e8, 0x001d594c)?;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0023c860, 0x001d5954)?;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x228));
    break 'b_1d5958;
    }
    }
    'b_1d5980: {
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    let t = c.g[V1][0] != c.g[V0][0];

    if t {
    break 'b_1d5980;
    } else {
    'b_1d5978: {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1d4));
    let t = c.g[A0][0] == 0u64;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x228), 0u64);
    if t {
    break 'b_1d5978;
    } else {
    c.cycles += 2;
    call_ptr(c, 0x00317e98, 0x001d5978)?;
    break 'b_1d5978;
    }
    }
    c.cycles += 2;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x1d4), 0u64);
    c.sw((c.g[S1][0] as u32).wrapping_add(0x1d0), 0u64);
    break 'b_1d5980;
    }
    }
    c.cycles += 7;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0020a430, 0x001d5988)?;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cPS2ChunkResolve_v06 (0x00003aa438).

/// `cPS2ChunkResolve_v06`: resolve a PlayStation 2 chunk descriptor (version 6) to data pointers.
pub fn c_ps2_chunk_resolve_v06(c: &mut Cpu) {
    guest(c, |c| {
    'b_3aa4f4: {
    c.cycles += 13;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S1][0] = c.g[A1][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[S2][0] = c.g[A0][0];
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
    let t = (c.g[V1][0] as i64) <= 0;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
    if t {
    break 'b_3aa4f4;
    } else {
    c.cycles += 5;
    c.g[S4][0] = 0xffffffffffff0000;
    c.g[V0][0] = 0x500000;
    c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
    c.g[S3][0] = c.g[V1][0];
    c.g[S4][0] = c.g[S4][0] | 0xffff;
    'l_3aa480: loop {
    'b_3aa4a8: {
    c.cycles += 3;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    let t = c.g[A1][0] == 0u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
    if t {
    break 'b_3aa4a8;
    } else {
    c.cycles += 7;
    c.g[A2][0] = c.g[S1][0];
    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x003aa4a8);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x003aa4a8 { return Err(crate::call::Unwound); }
    break 'b_3aa4a8;
    }
    }
    'b_3aa4c0: {
    'b_3aa4bc: {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
    let t = c.g[V0][0] != c.g[S4][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
    if t {
    break 'b_3aa4bc;
    } else {
    c.cycles += 2;
    let t = true;
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[S5][0]);
    if t {
    break 'b_3aa4c0;
    } else {
    break 'b_3aa4bc;
    }
    }
    }
    c.cycles += 1;
    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
    break 'b_3aa4c0;
    }
    'b_3aa4e8: {
    c.cycles += 3;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    let t = c.g[A1][0] == 0u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
    if t {
    break 'b_3aa4e8;
    } else {
    c.cycles += 7;
    c.g[A2][0] = c.g[S1][0];
    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x003aa4e8);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x003aa4e8 { return Err(crate::call::Unwound); }
    break 'b_3aa4e8;
    }
    }
    c.cycles += 3;
    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
    let t = c.g[S3][0] != 0u64;
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
    if t {
    if event_due(c) { deliver_event_here(c, 0x003aa480); } continue 'l_3aa480;
    } else {
    break 'b_3aa4f4;
    }
    }
    }
    }
    c.cycles += 10;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[V0][0] = 0x1;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// EventSelect_IsEventUnlocked (0x0000200d00).

/// `EventSelect_IsEventUnlocked`: report whether the event-select entry is unlocked.
pub fn event_select_is_event_unlocked(c: &mut Cpu) {
    guest(c, |c| {
    'b_200dd0: {
    'b_200dcc: {
    c.cycles += 16;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S0][0] = c.g[A1][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S2][0] = c.g[A2][0];
    call_ptr(c, 0x0014dc80, 0x00200d20)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x1;
    call_ptr(c, 0x0014dd58, 0x00200d2c)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = 0u64;
    if t {
    break 'b_200dcc;
    } else {
    'b_200d74: {
    c.cycles += 3;
    c.g[V0][0] = 0x4;
    let t = c.g[S0][0] != c.g[V0][0];
    c.g[V0][0] = 0x5;
    if t {
    break 'b_200d74;
    } else {
    c.cycles += 10;
    c.g[A0][0] = 0u64;
    call_ptr(c, 0x0014a080, 0x00200d54)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A3][0] = c.g[S2][0];
    c.g[A1][0] = 0u64;
    c.g[T0][0] = 0x1;
    call_ptr(c, 0x001464d0, 0x00200d6c)?;
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    if t {
    break 'b_200dd0;
    } else {
    break 'b_200d74;
    }
    }
    }
    'b_200da4: {
    c.cycles += 2;
    let t = c.g[S0][0] != c.g[V0][0];
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6));
    if t {
    break 'b_200da4;
    } else {
    c.cycles += 10;
    c.g[A0][0] = 0u64;
    call_ptr(c, 0x0014a080, 0x00200d84)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A3][0] = c.g[S2][0];
    c.g[A1][0] = 0u64;
    c.g[T0][0] = 0u64;
    call_ptr(c, 0x001464d0, 0x00200d9c)?;
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    if t {
    break 'b_200dd0;
    } else {
    break 'b_200da4;
    }
    }
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = 0u64;
    if t {
    break 'b_200dcc;
    } else {
    c.cycles += 7;
    c.g[A0][0] = 0u64;
    call_ptr(c, 0x0014a080, 0x00200db8)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A3][0] = c.g[S0][0];
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x00146a70, 0x00200dcc)?;
    break 'b_200dcc;
    }
    }
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    break 'b_200dd0;
    }
    c.cycles += 5;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// UIListBox_SetItemText (0x000039a7f0).

/// `UIListBox_SetItemText`: set the text of a list box item and resize the box.
pub fn ui_list_box_set_item_text(c: &mut Cpu) {
    guest(c, |c| {
    'b_39a888: {
    'b_39a86c: {
    c.cycles += 21;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S1][0] = c.g[A0][0];
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S2][0] = c.g[A2][0];
    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S4][0] = c.g[A1][0] & 0xff;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[S5][0] = c.g[A3][0];
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x00416810, 0x0039a828)?;
    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.g[V0][0] = 0x14;
    c.mult(0, 2, 20, 2);
    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
    let t = c.g[S0][0] == 0u64;
    c.g[A1][0] = 0x490000;
    if t {
    break 'b_39a86c;
    } else {
    'b_39a860: {
    c.cycles += 4;
    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0xa8));
    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let t = c.g[V0][0] == 0u64;

    if t {
    break 'b_39a860;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x00317e98, 0x0039a85c)?;
    c.g[S0][0] = 0u64;
    break 'b_39a860;
    }
    }
    c.cycles += 2;
    let t = c.g[S0][0] != 0u64;
    c.g[A0][0] = c.g[S0][0];
    if t {
    break 'b_39a888;
    } else {
    c.cycles += 1;
    c.g[A1][0] = 0x490000;
    break 'b_39a86c;
    }
    }
    }
    c.cycles += 7;
    c.g[A0][0] = sx((c.g[S3][0] as u32) << 1);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3df0));
    c.g[A2][0] = 0x100;
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00317e30, 0x0039a880)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    break 'b_39a888;
    }
    c.cycles += 19;
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, 0x00397ad8, 0x0039a890)?;
    c.g[V0][0] = 0x14;
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.mult(0, 2, 20, 2);
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x9c), c.g[S0][0]);
    c.g[A0][0] = c.g[V1][0];
    c.sw((c.g[A0][0] as u32).wrapping_add(0xa0), c.g[S2][0]);
    c.g[V0][0] = c.g[V1][0];
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa4), c.g[S5][0]);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// NetFEState_v32 (0x00001a8830).

/// `NetFEState_v32`: net front-end state helper: refresh the connection status and popup.
pub fn net_fe_state_v32(c: &mut Cpu) {
    guest(c, |c| {
    'b_1a8910: {
    'b_1a88b4: {
    c.cycles += 9;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xe8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xec));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[RA][0] = sx(0x001a884c);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001a884c { return Err(crate::call::Unwound); }
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1a88b4;
    } else {
    'b_1a8864: {
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;

    if t {
    break 'b_1a8864;
    } else {
    c.cycles += 2;
    call_ptr(c, 0x00260f80, 0x001a8864)?;
    break 'b_1a8864;
    }
    }
    'b_1a8880: {
    c.cycles += 4;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x00261460, 0x001a886c)?;
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1a8880;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x002613c8, 0x001a887c)?;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    break 'b_1a8880;
    }
    }
    'b_1a8894: {
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1a8894;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    call_ptr(c, 0x0025b800, 0x001a8890)?;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    break 'b_1a8894;
    }
    }
    c.cycles += 8;
    c.g[A1][0] = 0x1;
    c.g[A2][0] = 0u64;
    c.g[A3][0] = 0u64;
    c.g[T0][0] = 0u64;
    c.g[T1][0] = 0u64;
    call_ptr(c, 0x00262768, 0x001a88ac)?;
    let t = true;
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    if t {
    break 'b_1a8910;
    } else {
    break 'b_1a88b4;
    }
    }
    }
    'b_1a88c4: {
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;

    if t {
    break 'b_1a88c4;
    } else {
    c.cycles += 2;
    call_ptr(c, 0x00260f80, 0x001a88c4)?;
    break 'b_1a88c4;
    }
    }
    'b_1a88e0: {
    c.cycles += 4;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x00261460, 0x001a88cc)?;
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1a88e0;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    call_ptr(c, 0x002613c8, 0x001a88dc)?;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    break 'b_1a88e0;
    }
    }
    'b_1a88f4: {
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    if t {
    break 'b_1a88f4;
    } else {
    c.cycles += 3;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    call_ptr(c, 0x0025b800, 0x001a88f0)?;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
    break 'b_1a88f4;
    }
    }
    c.cycles += 7;
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0x1;
    c.g[A3][0] = 0u64;
    c.g[T0][0] = 0u64;
    c.g[T1][0] = 0u64;
    call_ptr(c, 0x00262768, 0x001a890c)?;
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    break 'b_1a8910;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// NetFEState_ShowDisconnectedPopup (0x00001a9930).

/// `NetFEState_ShowDisconnectedPopup`: show the "disconnected" popup and stop the network session.
pub fn net_fe_state_show_disconnected_popup(c: &mut Cpu) {
    guest(c, |c| {
    'b_1a9a00: {
    c.cycles += 10;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S3][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6ac));
    let t = c.g[V0][0] != 0u64;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    if t {
    break 'b_1a9a00;
    } else {
    'b_1a99dc: {
    'b_1a99d0: {
    c.cycles += 25;
    c.g[S1][0] = 0x1;
    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
    c.sw((c.g[S3][0] as u32).wrapping_add(0x6ac), c.g[S1][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    call_ptr(c, 0x0039f698, 0x001a996c)?;
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x001a9c50, 0x001a9974)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A0][0] = 0x460000;
    c.sw((c.g[S2][0] as u32).wrapping_add(0xbc), c.g[S1][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xbc));
    call_ptr(c, 0x00317670, 0x001a998c)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x001c57c0, 0x001a999c)?;
    c.g[V0][0] = 0x6;
    c.sw((c.g[S2][0] as u32).wrapping_add(0x170), c.g[S1][0]);
    c.sw((c.g[S2][0] as u32).wrapping_add(0x164), 0u64);
    c.sw((c.g[S2][0] as u32).wrapping_add(0x14c), c.g[V0][0]);
    call_ptr(c, 0x002591b8, 0x001a99b0)?;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
    let t = c.g[V1][0] != 0u64;
    c.g[V0][0] = 0x83d;
    if t {
    break 'b_1a99d0;
    } else {
    c.cycles += 5;
    c.g[V0][0] = 0x83e;
    c.g[A0][0] = 0x460000;
    c.sw((c.g[S2][0] as u32).wrapping_add(0x150), c.g[V0][0]);
    let t = true;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c40));
    if t {
    break 'b_1a99dc;
    } else {
    break 'b_1a99d0;
    }
    }
    }
    c.cycles += 3;
    c.g[A0][0] = 0x460000;
    c.sw((c.g[S2][0] as u32).wrapping_add(0x150), c.g[V0][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d28));
    break 'b_1a99dc;
    }
    c.cycles += 9;
    call_ptr(c, 0x00317670, 0x001a99e4)?;
    c.g[V1][0] = 0x3;
    c.sw((c.g[S2][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
    c.sw((c.g[S2][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, 0x001a9bc0, 0x001a99fc)?;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    break 'b_1a9a00;
    }
    }
    c.cycles += 6;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// OVScreenMessageCenter_UpdateCountText (0x00001e4758).

/// `OVScreenMessageCenter_UpdateCountText`: refresh the message-centre counter text.
pub fn ov_screen_message_center_update_count_text(c: &mut Cpu) {
    guest(c, |c| {
    'b_1e47cc: {
    c.cycles += 9;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.g[S2][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x13c));
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    if t {
    break 'b_1e47cc;
    } else {
    c.cycles += 20;
    c.g[A0][0] = 0x470000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f58));
    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    call_ptr(c, 0x00317670, 0x001e4798)?;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
    c.g[A0][0] = c.g[S0][0];
    let t = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[V0][0];
    c.g[RA][0] = sx(0x001e47a8);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001e47a8 { return Err(crate::call::Unwound); }
    c.g[S0][0] = c.g[V0][0];
    call_ptr(c, 0x001e33e8, 0x001e47b0)?;
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = c.g[V0][0];
    call_ptr(c, 0x002c26d0, 0x001e47c0)?;
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x13c));
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, 0x003a0e90, 0x001e47cc)?;
    break 'b_1e47cc;
    }
    }
    'b_1e4828: {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    if t {
    break 'b_1e4828;
    } else {
    c.cycles += 20;
    c.g[A0][0] = 0x470000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f40));
    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    call_ptr(c, 0x00317670, 0x001e47f4)?;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
    c.g[A0][0] = c.g[S0][0];
    let t = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[V0][0];
    c.g[RA][0] = sx(0x001e4804);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x001e4804 { return Err(crate::call::Unwound); }
    c.g[S0][0] = c.g[V0][0];
    call_ptr(c, 0x001e3408, 0x001e480c)?;
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = c.g[V0][0];
    call_ptr(c, 0x002c26d0, 0x001e481c)?;
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, 0x003a0e90, 0x001e4828)?;
    break 'b_1e4828;
    }
    }
    c.cycles += 6;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// LobbyMan_OnDisconnect (0x00002598c8).

/// `LobbyMan_OnDisconnect`: handle losing the network connection in the lobby.
pub fn lobby_man_on_disconnect(c: &mut Cpu) {
    guest(c, |c| {
    'b_2599a0: {
    'b_25999c: {
    c.cycles += 9;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.g[V0][0] = 0x64690000;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[V0][0] = c.g[V0][0] | 0x7363;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    let t = c.g[V1][0] != c.g[V0][0];
    c.g[S1][0] = c.g[A0][0];
    if t {
    break 'b_25999c;
    } else {
    'b_25994c: {
    c.cycles += 24;
    c.g[A1][0] = 0x480000;
    c.g[A2][0] = 0x20000000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
    c.g[A0][0] = 0x8;
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00317d70, 0x00259904)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf0));
    c.g[V0][0] = 0x480000;
    c.g[V1][0] = 0x115;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
    c.sw((c.g[S0][0] as u32), c.g[V1][0]);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[A1][0] = 0x480000;
    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    c.g[A2][0] = 0x20000000;
    c.g[A0][0] = 0xc;
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00317d70, 0x0025993c)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
    if c.g[V0][0] != 0u64 {
    c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[S0][0]);
    break 'b_25994c;
    } else {
    break 'b_25994c;
    }
    }
    c.cycles += 15;
    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = 0x1;
    c.sw((c.g[A2][0] as u32), c.g[A0][0]);
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.sw((c.g[V1][0] as u32), c.g[A2][0]);
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
    c.sw((c.g[SP][0] as u32), c.g[A2][0]);
    call_ptr(c, 0x002591d0, 0x0025997c)?;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
    let t = c.g[V1][0] == 0u64;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    if t {
    break 'b_2599a0;
    } else {
    c.cycles += 5;
    call_ptr(c, 0x002591d0, 0x00259990)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x00263ba8, 0x0025999c)?;
    break 'b_25999c;
    }
    }
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    break 'b_2599a0;
    }
    c.cycles += 4;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// LobbyMan_ParseNameSuggestions (0x000025bc48).

/// `LobbyMan_ParseNameSuggestions`: parse the name-suggestion list the server sent.
pub fn lobby_man_parse_name_suggestions(c: &mut Cpu) {
    guest(c, |c| {
    'b_25bd1c: {
    'b_25bd18: {
    c.cycles += 20;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
    c.g[V0][0] = 0x4a0000;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
    c.g[S1][0] = c.g[A0][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3090));
    call_ptr(c, 0x003eb6d8, 0x0025bc70)?;
    c.g[A3][0] = 0x4a0000;
    c.g[A0][0] = c.g[V0][0];
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f40));
    c.g[A1][0] = c.g[SP][0];
    c.g[A2][0] = 0x100;
    call_ptr(c, 0x003ed378, 0x0025bc88)?;
    c.g[V1][0] = c.lb((c.g[SP][0] as u32));
    c.g[S0][0] = c.g[SP][0];
    let t = c.g[V1][0] == 0u64;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x284), 0u64);
    if t {
    break 'b_25bd18;
    } else {
    c.cycles += 2;
    c.g[S2][0] = 0x2c;
    c.g[V0][0] = c.lb((c.g[S0][0] as u32));
    'l_25bca0: loop {
    'b_25bce4: {
    'b_25bce0: {
    c.cycles += 2;
    let t = c.g[V0][0] == 0u64;
    c.g[A1][0] = c.g[S0][0];
    if t {
    break 'b_25bce0;
    } else {
    'b_25bccc: {
    c.cycles += 2;
    let t = c.g[V0][0] == c.g[S2][0];
    c.g[V1][0] = 0x2c;
    if t {
    break 'b_25bccc;
    } else {
    c.cycles += 2;
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
    'l_25bcb8: loop {
    c.cycles += 3;
    c.g[V0][0] = c.lb((c.g[S0][0] as u32));
    if c.g[V0][0] == 0u64 {
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x284));
    break 'b_25bce4;
    } else {
    c.cycles += 2;
    if c.g[V0][0] != c.g[V1][0] {
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
    if event_due(c) { deliver_event_here(c, 0x0025bcb8); } continue 'l_25bcb8;
    } else {
    break 'b_25bccc;
    }
    }
    }
    }
    }
    c.cycles += 3;
    c.g[V0][0] = c.lb((c.g[S0][0] as u32));
    if c.g[V0][0] == 0u64 {
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x284));
    break 'b_25bce4;
    } else {
    c.cycles += 2;
    c.sb((c.g[S0][0] as u32), 0u64);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
    break 'b_25bce0;
    }
    }
    }
    c.cycles += 1;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x284));
    break 'b_25bce4;
    }
    c.cycles += 9;
    c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x274));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.sw((c.g[S1][0] as u32).wrapping_add(0x284), c.g[V0][0]);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    call_ptr(c, 0x003184e8, 0x0025bcfc)?;
    c.g[V1][0] = c.lb((c.g[S0][0] as u32));
    if c.g[V1][0] == 0u64 {
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
    break 'b_25bd1c;
    } else {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x284));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
    if c.g[V0][0] != 0u64 {
    c.g[V0][0] = c.lb((c.g[S0][0] as u32));
    if event_due(c) { deliver_event_here(c, 0x0025bca0); } continue 'l_25bca0;
    } else {
    break 'b_25bd18;
    }
    }
    }
    }
    }
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
    break 'b_25bd1c;
    }
    c.cycles += 5;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
    c.pc = t; return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{compare, fill, seed_count, Rng, DATA, GAME_GP};

    /// A scratch object away from `DATA` (where pointers land).
    const OBJ: u32 = DATA + 0x1000;
    /// The zeroed BELibrary dummy.
    const BE: u32 = DATA + 0x3_0000;
    /// The dummy world.
    const WORLD: u32 = DATA + 0x8_0000;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    fn put_str(c: &mut Cpu, a: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(a + i as u32, *b);
        }
        c.bus.write8(a + s.len() as u32, 0);
    }

    /// The BELibrary singleton (`gp - 0x1f04`, a zeroed dummy so nothing reads
    /// the disc) and the world (`gp - 0x848`, with no riders).
    fn globals(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        let game = WORLD + 0x1000;
        w(c, WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    /// The UI manager singleton (`gp - 0xc8`, `D_004A3028`).
    fn ui(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0xc8), OBJ);
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena.
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u32 = 0xdead_0100;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET, "DLHeap_InitArena did not return");
    }

    /// The engine's size-class pool allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class.
    fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pl = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, pl + head, base);
            w(c, GAME_GP.wrapping_add(slot), pl);
        }
    }

    /// `StrHash(s)` (0x317670) on the string at `a`, to build name-hash
    /// fixtures.
    fn str_hash(c: &mut Cpu, a: u32) -> u32 {
        const RET: u32 = 0xdead_0200;
        c.g[A0][0] = sx(a);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ee::s10::StrHash(c);
        assert_eq!(c.pc, RET, "StrUHash did not return");
        c.g[V0][0] as u32
    }

    /// Assert no differences at all (hard, caller-saved and LO/HI).
    fn diff(gen: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seed_count(seeds) {
            let r = compare(gen, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} (caller-saved): {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
        }
    }

        #[test]
    fn net_config_auth_popup_ctor_matches() {
        diff(ee::s03::f_001b6288__generated, net_config_auth_popup_ctor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1b_6288, 0);
            globals(c);
            ui(c);
            heap_init(c);
            pool(c);
            // `USTR_length` goes through the current UI object's vtable; a zero
            // vtable pointer makes that call a fake.
            w(c, WORLD + 0x8c, DATA + 0x3000);
            w(c, DATA + 0x3004, 0);
            put_str(c, DATA + 0x300, b"config");
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(r.next_u32());
            c.g[A2][0] = sx(r.next_u32());
            w(c, DATA + 0x6fc, 0);
            w(c, DATA + 0x6dc, r.next_u32());
        });
    }

    #[test]
    fn auth_dnas_error_on_widget_created_matches() {
        diff(
            ee::s03::f_001b6bb8__generated,
            c_fe_state_auth_dnas_error_on_widget_created,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b_6bb8, 0);
                globals(c);
                put_str(c, dnas_error::STATUS, b"status1");
                put_str(c, dnas_error::STATUS2, b"status2");
                put_str(c, dnas_error::STATUS3, b"status3");
                put_str(c, dnas_error::RETRY, b"retry");
                put_str(c, dnas_error::MESSAGE, b"message");
                let obj = DATA + 0x2000;
                let hash = match seed % 6 {
                    0 => str_hash(c, dnas_error::STATUS),
                    1 => str_hash(c, dnas_error::STATUS2),
                    2 => str_hash(c, dnas_error::STATUS3),
                    3 => str_hash(c, dnas_error::RETRY),
                    4 => str_hash(c, dnas_error::MESSAGE),
                    _ => r.next_u32(),
                };
                w(c, obj + 0x38, hash);
                w(c, obj + 8, 0);
                w(c, obj + 0x90, r.next_u32());
                w(c, DATA + 0x6dc, 0);
                w(c, DATA + 0x6e0, 0);
                w(c, DATA + 0x6e4, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(obj);
            },
        );
    }
    /// A float bit pattern in [-range, range).
    fn uf(r: &mut Rng, range: f32) -> u32 {
        r.f32(range)
    }

    #[test]
    fn in_seq_curve_track_update_matches() {
        diff(ee::s07::f_002726a8__generated, in_seq_curve_track_update, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x27_26a8, 0);
            globals(c);
            let mode = seed % 4;
            let curve = if mode == 0 { 0 } else { DATA + 0x2000 };
            c.g[A0][0] = sx(DATA);
            c.f[12] = uf(&mut r, 10.0);
            w(c, DATA + in_seq_curve::CURVE, curve);
            w(c, DATA + in_seq_curve::VALUE, r.next_u32());
            w(c, DATA + in_seq_curve::LENGTH, DATA + 0x2100);
            w(c, DATA + in_seq_curve::SEGMENT, if mode == 3 { 0 } else { DATA + 0x2200 });
            w(c, DATA + in_seq_curve::KEY, (seed % 3) as u32);
            if curve != 0 {
                c.bus.write16(curve, (seed % 0x30) as u16);
                c.bus.write16(curve + 2, if mode == 1 { 2 } else { 0x20 });
                fill(c, &mut r, curve + 4, 4);
            }
            w(c, DATA + 0x2100, (seed % 3) as u32);
            c.bus.write16(DATA + 0x2200, (seed % 0x30) as u16);
            c.bus.write16(DATA + 0x2202, (seed % 0x30) as u16);
            for i in 0..4 {
                w(c, DATA + 0x2204 + 4 * i, uf(&mut r, 2.0));
            }
        });
    }

    #[test]
    fn sum_equipped_bolt_value_matches() {
        diff(
            ee::s01::f_0014b478__generated,
            be_bolt_int_sum_equipped_bolt_value,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14_b478, 0);
                globals(c);
                let idx = seed % 4;
                let count = (seed % 5) as u32;
                w(c, 0x004a_6750 + idx * 4 + 0x2c, count);
                w(c, 0x004a_6750 + 4, DATA);
                w(c, 0x004a_6750 + idx * 4 + 0xa4, 0);
                w(c, 0x004a_7eb8, DATA + 0x3000);
                for i in 0..count {
                    let bolt = DATA + i * bolts::STRIDE;
                    c.bus.write16(bolt + bolts::FLAGS, (seed as u16 % 5) as u16);
                    c.bus.write16(bolt + bolts::VALUE, r.next_u32() as u16);
                    c.bus.write16(
                        DATA + 0x3000 + 2 * (seed as u32 % 5),
                        (if seed % 3 == 0 { 0x8000 } else { 0x10 }) as u16,
                    );
                }
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = idx as u64;
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn anim_combo_v52_matches() {
        diff(ee::s11::f_00359140__generated, anim_combo_v52, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x35_9140, 0);
            globals(c);
            let anim = DATA + 0x400;
            c.g[A0][0] = sx(DATA);
            // The animation object: no flags (early exit of AnimObject_v52).
            w(c, anim + 0x48, 0);
            w(c, DATA + 0x34, anim);
            w(c, DATA + anim_combo::ANIM, if seed % 5 == 0 { 0 } else { DATA + 0x4000 });
            c.bus.write8(DATA + anim_combo::PART_A, (seed % 2) as u8);
            c.bus.write8(DATA + anim_combo::PART_B, ((seed >> 1) % 2) as u8);
            c.bus.write8(DATA + anim_combo::PART_C, ((seed >> 2) % 2) as u8);
            w(c, DATA + anim_combo::MODEL, DATA + 0x6000);
            w(c, DATA + anim_combo::MATRICES, DATA + 0x5000);
            // The animation controller's part list.
            let ctrl = DATA + 0x6000;
            let count = (seed % 4) as u32;
            w(c, ctrl + 0x80, DATA + 0x7000);
            w(c, DATA + 0x7004, count);
            w(c, DATA + 0x7008, DATA + 0x7100);
            for i in 0..count {
                let part = DATA + 0x7100 + 0x10 * i;
                let obj = DATA + 0x7200 + 0x40 * i;
                w(c, part + 8, obj);
                w(c, obj + 0x1c, if seed % 3 == 0 { 0 } else { r.next_u32() });
            }
            for i in 0..8 {
                w(c, DATA + 0x4000 + 4 * i, r.next_u32());
                w(c, DATA + 0x5000 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn path_find_marker_boundary_matches() {
        diff(ee::s07::f_00269f18__generated, path_find_marker_boundary, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x26_9f18, 0);
            globals(c);
            let list = DATA + 0x1000;
            let entries = DATA + 0x2000;
            let count = (seed % 4) as u32;
            w(c, list, count);
            w(c, list + 4, entries);
            w(c, list + 8, (seed / 4 % 4) as u32);
            for i in 0..4 {
                w(c, list + 0xc + 4 * i, uf(&mut r, 20.0));
            }
            let segs = DATA + 0x2500;
            w(c, list + 0x18, segs);
            for i in 0..4 {
                for j in 0..3 {
                    w(c, segs + 0x10 * i + 4 * j, uf(&mut r, 2.0));
                }
                w(c, segs + 0x10 * i + 0xc, 10.0f32.to_bits() + (r.next_u32() % 0x40));
            }
            for i in 0..count {
                let e = entries + 0x10 * i;
                let kind = [12u32, 16, 14, 7][(seed as usize + i as usize) % 4];
                w(c, e, kind);
                w(c, e + 8, uf(&mut r, 50.0));
                w(c, e + 0xc, uf(&mut r, 50.0));
            }
            w(c, DATA + 0x3000, uf(&mut r, 60.0));
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(list);
            c.g[A2][0] = sx(DATA + 0x3000);
        });
    }

    /// A scratch world/UI object graph used by the front-end tests.
    fn fe_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        globals(c);
        ui(c);
        heap_init(c);
        pool(c);
        r
    }

    #[test]
    fn lobby_menu_enter_room_matches() {
        diff(ee::s03::f_001bd460__generated, c_fe_state_lobby_menu_enter_room, 40, &|c, seed| {
            let _r = fe_base(c, seed, 0x1b_d460);
            let s = if seed % 3 == 0 { 0 } else { DATA + 0x300 };
            if s != 0 {
                put_str(c, s, b"room name");
            }
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = sx(s);
            w(c, DATA + 0x770, 0);
            w(c, DATA + 0xb80, 0);
        });
    }

    #[test]
    fn online_main_menu_on_leave_popup_event_matches() {
        diff(
            ee::s03::f_001c0138__generated,
            c_fe_state_online_main_menu_on_leave_popup_event,
            40,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1c_0138);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = if seed % 5 == 0 { 0xf } else { (seed % 0x10) as u64 };
                w(c, DATA + 0x10, r.next_u32());
                w(c, DATA + 0x6b0, 0);
                w(c, DATA + 0x2000, r.next_u32());
                w(c, GAME_GP.wrapping_add(0x238), if seed % 3 == 0 { 0 } else { DATA + 0x3000 });
            },
        );
    }

    #[test]
    fn view_popup_on_widget_action_matches() {
        diff(
            ee::s04::f_001e5800__generated,
            c_ov_state_view_popup_on_widget_action,
            40,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1e_5800);
                let id = (seed % 4) as u32;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = [5u64, 6, 7, 8][seed as usize % 4];
                w(c, DATA + 0x20, 0);
                w(c, DATA + 0x68, id);
                w(c, 0x004c_6c08 + id * 0x18 + 4, (seed % 3) as u32);
                w(c, 0x0044_1630 + (seed % 3) * 0x14, [4u32, 0, 2][seed as usize % 3]);
                w(c, 0x004c_6c08 + id * 0x18 + 8, r.next_u32());
            },
        );
    }

        #[test]
    fn lobby_man_send_lock_chal_matches() {
        diff(ee::s06::f_0025fc70__generated, lobby_man_send_lock_chal, 40, &|c, seed| {
            let mut r = fe_base(c, seed, 0x25_fc70);
            put_str(c, DATA + 0x300, b"chal");
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 2 == 0 { sx(DATA + 0x300) } else { 1 };
            c.g[A2][0] = (seed % 4) as u64;
            w(c, DATA + 0xf0, 0);
            w(c, DATA + 0xe0, r.next_u32());
        });
    }

    #[test]
    fn dj_say_20e9_matches() {
        diff(ee::s08::f_002a23d0__generated, ssx_audio_system_dj_say_20e9, 40, &|c, seed| {
            let mut r = fe_base(c, seed, 0x2a_23d0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 2 == 0 { 0xffff_ffff_ffff_ffff } else { (seed % 4) as u64 };
            w(c, DATA + 0x5560, 0);
            w(c, DATA + 0x5578, 0);
            w(c, GAME_GP.wrapping_add(0x173c), 0x002c_c098);
            w(c, DATA + 0x3000, 0);
        });
    }

        #[test]
    fn script_engine_add_script_obj_matches() {
        diff(ee::s07::f_0027cea8__generated, script_engine_add_script_obj, 40, &|c, seed| {
            let _r = fe_base(c, seed, 0x27_cea8);
            let obj = DATA + 0x2000;
            let vt = DATA + 0x3000;
            // The object's "binding count" virtual returns null: MenuSys_IsActive
            // on a zeroed first word.
            w(c, vt + 0x30, 0);
            w(c, vt + 0x34, 0x002c_c098);
            w(c, obj, 0);
            w(c, obj + 0xc, vt);
            for i in 0..20 {
                w(c, DATA + 4 * i, if i == 0 { 0 } else { 0 });
            }
            if seed % 2 == 0 {
                w(c, DATA, 0);
            } else {
                w(c, DATA, DATA + 0x5000);
            }
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(obj);
        });
    }

    #[test]
    fn bank_monitor_stop_by_bank_matches() {
        diff(ee::s08::f_002ad650__generated, bank_monitor_stop_by_bank, 30, &|c, seed| {
            let mut r = fe_base(c, seed, 0x2a_d650);
            let bank = (seed % 3) as u32;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = bank as u64;
            c.f[12] = if seed % 2 == 0 { 0.0f32.to_bits() } else { uf(&mut r, 4.0) };
            w(c, DATA + 0x8ec, DATA + 0x9000);
            for i in 0..64u32 {
                let inst = DATA + 0x9000 + i * 0xc0;
                w(c, inst, if seed % 3 == 0 && i % 3 == 0 { 1 } else { 0 });
                w(c, inst + 0xc, bank);
                w(c, inst + 0x10, r.next_u32());
            }
        });
    }

    #[test]
    fn in_seq_heap_free_matches() {
        diff(ee::s07::f_002723a8__generated, in_seq_heap_free, 40, &|c, seed| {
            let mut r = fe_base(c, seed, 0x27_23a8);
            c.g[A0][0] = sx(DATA);
            w(c, DATA + 8, if seed % 2 == 0 { 0 } else { 0 });
            w(c, DATA, r.next_u32());
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x1008) };
            w(c, DATA + 0x1000, 0x20); // the block's size header (payload at +8)
            w(c, DATA + 0x1008, 0x30);
            w(c, DATA + 0x100c, 0);
        });
    }

    #[test]
    fn ai_force_finish_riders_matches() {
        diff(ee::s00::f_0012b340__generated, ai_force_finish_riders, 40, &|c, seed| {
            let mut r = fe_base(c, seed, 0x12_b340);
            c.g[A0][0] = sx(DATA);
            w(c, DATA, if seed % 5 == 0 { r.next_u32() } else { 5 });
            w(c, DATA + 0x7c, 2);
            for i in 0..2u32 {
                w(c, DATA + 0x40 + 4 * i, DATA + 0x1000 + 0x100 * i);
            }
            for i in 0..2u32 {
                let rider = DATA + 0x1000 + 0x100 * i;
                let r_ = DATA + 0x2000 + 0x100 * i;
                w(c, rider + 0x18, r_);
                w(c, r_ + 0x87c, if seed % 2 == 0 { 0 } else { 1 });
                w(c, r_ + 0x470, if i == 0 { 0.0f32.to_bits() } else { uf(&mut r, 100.0) });
                w(c, r_ + 0x480, 0);
            }
            w(c, GAME_GP.wrapping_sub(0x204), DATA + 0x3000);
        });
    }

    #[test]
    fn mc_manager_begin_replay_write_matches() {
        diff(ee::s06::f_002412a0__generated, mc_manager_begin_replay_write, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x24_12a0);
            let _ = r.next_u32();
            c.g[A0][0] = sx(DATA);
            w(c, DATA + 0xf8, (seed % 3) as u32);
            w(c, GAME_GP.wrapping_sub(0x478), 0);
        });
    }

    #[test]
    fn c_game_load_state_init_matches() {
        diff(ee::s06::f_00245418__generated, c_game_load_state_init, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x24_5418);
            let _ = r.next_u32();
            c.g[A0][0] = sx(DATA);
            w(c, DATA + 0x10, r.next_u32());
        });
    }

    #[test]
    fn w_script_man_get_hud_target_height_matches() {
        diff(ee::s10::f_00309ba8__generated, w_script_man_get_hud_target_height, 40, &|c, seed| {
            let mut r = fe_base(c, seed, 0x30_9ba8);
            c.g[A0][0] = sx(DATA);
            w(c, DATA + 0x28, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
            w(c, DATA + 0x2c, r.next_u32());
            w(c, DATA + 0x30, r.next_u32());
            let t = DATA + 0x2000;
            c.f[12] = uf(&mut r, 100.0);
            w(c, t + 0x28c, r.next_u32());
            w(c, t + 0x6c0, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
        });
    }

    #[test]
    fn w_script_man_collect_item_matches() {
        diff(ee::s10::f_0030b9a0__generated, w_script_man_collect_item, 30, &|c, seed| {
            let mut r = fe_base(c, seed, 0x30_b9a0);
            let item = (seed % 4) as u32;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 3) as u64;
            c.g[A2][0] = item as u64;
            c.g[A3][0] = sx(DATA + 0x2000);
            w(c, DATA + 0x2c0, if seed % 2 == 0 { item } else { item ^ 1 });
            for i in 0..4 {
                w(c, DATA + 0x2c0 + 0x10c * i, r.next_u32());
            }
        });
    }

    #[test]
    fn plants_menu_dtor_matches() {
        diff(ee::s06::f_0024f860__generated, plants_menu_dtor, 30, &|c, seed| {
            let _r = fe_base(c, seed, 0x24_f860);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 3) as u64;
            for off in (0x130..0x3a0).step_by(4) {
                w(c, DATA + off, 0);
            }
        });
    }

    #[test]
    fn w_script_process_save_state_matches() {
        diff(ee::s10::f_00308038__generated, w_script_process_save_state, 30, &|c, seed| {
            let mut r = fe_base(c, seed, 0x30_8038);
            let obj = DATA + 0x2000;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(obj);
            w(c, obj, 0); // no vtable: the two virtual calls are fakes
            w(c, DATA + 4, (seed % 3) as u32);
            w(c, DATA + 0x14, DATA + 0x3000);
            w(c, DATA + 0x1c, if seed % 2 == 0 { 0 } else { 1 });
            w(c, DATA + 0x24, r.next_u32());
            w(c, GAME_GP.wrapping_add(0x16c8), DATA + 0x4000);
            w(c, DATA + 0x4004, r.next_u32());
            w(c, DATA + 0x3000, r.next_u32());
        });
    }

    #[test]
    fn mem_card_ui_try_next_slot_matches() {
        diff(ee::s04::f_001d8160__generated, mem_card_ui_try_next_slot, 30, &|c, seed| {
            let mut r = fe_base(c, seed, 0x1d_8160);
            c.g[A0][0] = sx(DATA);
            w(c, DATA + 0x424, (seed % 4) as u32);
            w(c, DATA + 0x428, (seed % 3) as u32);
            w(c, DATA + 0x434, r.next_u32());
            w(c, DATA + 0x1a0, r.next_u32());
            w(c, DATA + 0x1a4, r.next_u32());
            w(c, DATA + 0x1c0, r.next_u32());
        });
    }

        #[test]
    fn c_ps2_chunk_resolve_v15_matches() {
        diff(ee::s13::f_003aa608__generated, c_ps2_chunk_resolve_v15, 30, &|c, seed| {
            let mut r = fe_base(c, seed, 0x3a_a608);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x2000);
            w(c, DATA + 0x2004, if seed % 3 == 0 { 0 } else { 1 });
            w(c, DATA + 0x2008, DATA + 0x3000);
            w(c, DATA + 0x3000, r.next_u32());
            w(c, DATA + 0x3004, if seed % 2 == 0 { 0 } else { 0x10 });
            w(c, DATA + 0x3008, r.next_u32());
            w(c, DATA + 0x300c, 0xffff_ffff);
            w(c, DATA + 8, 0); // no vtable on the chunk: calls are fakes
        });
    }

    #[test]
    fn modifier_block_handle_event_matches() {
        diff(ee::s11::f_00352f40__generated, modifier_block_handle_event, 30, &|c, seed| {
            let mut r = fe_base(c, seed, 0x35_2f40);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x2000);
            c.f[12] = uf(&mut r, 10.0);
            w(c, DATA, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
            w(c, DATA + 0x1000, 0); // the handler's vtable is null
            w(c, DATA + 4, if seed % 3 == 0 { 0 } else { DATA + 0x1100 });
            w(c, DATA + 8, if seed % 3 == 0 { DATA + 0x1200 } else { 0 });
            w(c, DATA + 0xc, if seed % 5 == 0 { DATA + 0x1300 } else { 0 });
            w(c, DATA + 0x1100, r.next_u32());
        });
    }

    #[test]
    fn c_fe_load_state_init_matches() {
        diff(ee::s06::f_00245b80__generated, c_fe_load_state_init, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x24_5b80);
            c.g[A0][0] = sx(DATA);
            w(c, DATA + 0x10, r.next_u32());
            w(c, DATA + 0x20, r.next_u32());
        });
    }

    #[test]
    fn c_fe_state_audio_options_init_matches() {
        diff(ee::s02::f_00196148__generated, c_fe_state_audio_options_init, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x19_6148);
            c.g[A0][0] = sx(DATA);
            let game = WORLD + 0x1000;
            w(c, game + 0x28, DATA + 0x2000);
            w(c, DATA + 0x2000, r.next_u32());
            w(c, DATA + 0x2004, r.next_u32());
            w(c, DATA + 0x80, r.next_u32());
        });
    }

    #[test]
    fn roller_modifier_copy_matrix_block_matches() {
        diff(ee::s10::f_00327cc8__generated, roller_modifier_copy_matrix_block, 30, &|c, seed| {
            let mut r = fe_base(c, seed, 0x32_7cc8);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(if seed % 2 == 0 { DATA + 0x2000 } else { DATA + 0x2004 });
            for i in 0..0x40 {
                w(c, DATA + 0x2000 + 4 * i, r.next_u32());
            }
            for i in 0..0x40 {
                w(c, DATA + 0x2200 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn ov_screen_replay_menu_on_widget_created_matches() {
        diff(
            ee::s05::f_0020de30__generated,
            ov_screen_replay_menu_on_widget_created,
            30,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x20_de30);
                let widget = DATA + 0x2000;
                put_str(c, 0x0047_1bf0, b"replay");
                let h = str_hash(c, 0x0047_1bf0);
                w(c, widget + 0x38, if seed % 3 == 0 { r.next_u32() } else { h });
                w(c, widget + 8, 0);
                w(c, widget + 0x18, r.next_u32());
                let game = WORLD + 0x1000;
                w(c, game + 0x28, DATA + 0x3000);
                w(c, DATA + 0x3000 + 0x608, if seed % 2 == 0 { 0 } else { DATA + 0x3100 });
                w(c, DATA + 0x3100, r.next_u32());
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(widget);
            },
        );
    }

    #[test]
    fn mc_manager_state0a_format_matches() {
        diff(ee::s06::f_0023ef68__generated, mc_manager_state0a_format, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x23_ef68);
            c.g[A0][0] = sx(DATA);
            w(c, DATA + 0x338, if seed % 3 == 0 { 0 } else { r.next_u32() });
            w(c, DATA + 0x434, DATA + 0x2000);
            w(c, DATA + 0x2000, 0);
            w(c, DATA + 0x6c, (seed % 4) as u32);
            w(c, DATA + 0x748, r.next_u32());
        });
    }

    #[test]
    fn c_keyboard_popup_set_text_matches() {
        diff(ee::s04::f_001cb418__generated, c_keyboard_popup_set_text, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x1c_b418);
            let hb = DATA + 0x1000;
            w(c, DATA + 0x74, hb + 0xc);
            w(c, hb, 1);
            w(c, hb + 4, 2);
            w(c, hb + 8, 0x40);
            c.bus.write8(hb + 0xc, b'h');
            c.bus.write8(hb + 0xd, b'i');
            c.bus.write8(hb + 0xe, 0);
            w(c, DATA + 0x58, if seed % 2 == 0 { 0 } else { r.next_u32() });
            w(c, DATA + 0x64, r.next_u32());
            c.g[A0][0] = sx(DATA);
        });
    }

        #[test]
    fn reward_gallery_check_rewards_ready_matches() {
        diff(
            ee::s04::f_001d0238__generated,
            c_fe_state_reward_gallery_check_rewards_ready,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1d_0238);
                c.g[A0][0] = sx(DATA);
                w(c, DATA + 0x48, r.next_u32());
                w(c, DATA + 8, 0);
                w(c, DATA + 0x5c, r.next_u32());
                w(c, DATA + 0x68, r.next_u32());
            },
        );
    }

    #[test]
    fn char_equip_category_all_flagged_matches() {
        diff(
            ee::s03::f_0019b098__generated,
            c_fe_state_char_equip_category_all_flagged,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x19_b098);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = 0;
                c.g[A3][0] = (seed % 4) as u64;
                w(c, DATA + 0xbc, DATA + 0x3000);
                w(c, DATA + 0x3000, 0);
                w(c, DATA + 0x3004, r.next_u32());
            },
        );
    }

    #[test]
    fn w_script_man_restart_matches() {
        diff(ee::s10::f_00309030__generated, w_script_man_restart, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x30_9030);
            c.g[A0][0] = sx(DATA);
            w(c, DATA + 0x28c, DATA + 0x2000);
            w(c, DATA + 0x2000, DATA + 0x2100);
            w(c, DATA + 0x2004, DATA + 0x2200);
            w(c, DATA + 0x2104, DATA + 0x2300);
            w(c, DATA + 0x2110, 0);
            w(c, DATA + 0x2204, DATA + 0x2300);
            w(c, DATA + 0x2300, r.next_u32());
            w(c, DATA + 0x2308, r.next_u32());
        });
    }

    #[test]
    fn rider_metrix_on_split_marker_matches() {
        diff(ee::s00::f_001194c0__generated, rider_metrix_on_split_marker, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x11_94c0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 4) as u64;
            w(c, DATA + 0x1ac, DATA + 0x2000);
            w(c, DATA + 0x2000 + 0x6c0, 0);
            w(c, DATA + 0x2000 + 0x6c4, r.next_u32());
            w(c, DATA + 0x2000 + 0x6c8, r.next_u32());
            w(c, DATA + 0x2000 + 0x6cc, r.next_u32());
        });
    }

    #[test]
    fn mem_card_state_base_shutdown_matches() {
        diff(ee::s04::f_001d58b8__generated, mem_card_state_base_shutdown, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x1d_58b8);
            c.g[A0][0] = sx(DATA);
            w(c, DATA + 0x1bc, (seed % 4) as u32);
            w(c, DATA + 0x1c0, r.next_u32());
            w(c, DATA + 0x20, r.next_u32());
        });
    }

    #[test]
    fn c_ps2_chunk_resolve_v06_matches() {
        diff(ee::s13::f_003aa438__generated, c_ps2_chunk_resolve_v06, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x3a_a438);
            c.g[A0][0] = sx(DATA + 0x2000);
            c.g[A1][0] = sx(DATA);
            w(c, DATA + 0x2004, if seed % 3 == 0 { 0 } else { 1 });
            w(c, DATA + 0x2008, DATA + 0x3000);
            w(c, DATA + 0x3000, r.next_u32());
            w(c, DATA + 0x3004, if seed % 2 == 0 { 0 } else { 0x10 });
            w(c, DATA + 0x3008, r.next_u32());
            w(c, DATA + 8, 0);
        });
    }

    #[test]
    fn event_select_is_event_unlocked_matches() {
        diff(ee::s05::f_00200d00__generated, event_select_is_event_unlocked, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x20_0d00);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x2000);
            c.g[A2][0] = (seed % 5) as u64;
            w(c, GAME_GP.wrapping_sub(0xb5c), if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
            w(c, DATA + 0x2000, r.next_u32());
            w(c, DATA + 0x3000, r.next_u32());
        });
    }

    #[test]
    fn ui_list_box_set_item_text_matches() {
        diff(ee::s13::f_0039a7f0__generated, ui_list_box_set_item_text, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x39_a7f0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = sx(DATA + 0x2000);
            c.g[A3][0] = (seed % 3) as u64;
            w(c, DATA + 0x2000, r.next_u32());
            w(c, DATA + 0x2004, r.next_u32());
            w(c, DATA + 0x2008, 0);
        });
    }

    #[test]
    fn net_fe_state_v32_matches() {
        diff(ee::s03::f_001a8830__generated, net_fe_state_v32, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x1a_8830);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 3) as u64;
            w(c, DATA + 8, 0);
            w(c, DATA + 0x3000, r.next_u32());
            w(c, GAME_GP.wrapping_add(0x238), if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
        });
    }

    #[test]
    fn net_fe_state_show_disconnected_popup_matches() {
        diff(
            ee::s03::f_001a9930__generated,
            net_fe_state_show_disconnected_popup,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1a_9930);
                c.g[A0][0] = sx(DATA);
                w(c, DATA + 0x6ac, if seed % 2 == 0 { 0 } else { (r.next_u32() | 1) });
                w(c, DATA + 0x6b0, 0);
                w(c, DATA + 0x6dc, 0);
            },
        );
    }

    #[test]
    fn ov_screen_message_center_update_count_text_matches() {
        diff(
            ee::s04::f_001e4758__generated,
            ov_screen_message_center_update_count_text,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1e_4758);
                c.g[A0][0] = sx(DATA);
                w(c, DATA + 0x13c, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                w(c, DATA + 0x2000 + 0x68, (seed % 4) as u32);
                w(c, DATA + 0x2000 + 0x20, 0);
                w(c, DATA + 0x2000 + 0x8, r.next_u32());
            },
        );
    }

    #[test]
    fn lobby_man_on_disconnect_matches() {
        diff(ee::s06::f_002598c8__generated, lobby_man_on_disconnect, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x25_98c8);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x2000);
            w(c, DATA + 0x2004, if seed % 2 == 0 { 0 } else { 0x6469_7363 });
            w(c, DATA + 8, r.next_u32());
            w(c, DATA + 0xf0, 0);
        });
    }

    #[test]
    fn lobby_man_parse_name_suggestions_matches() {
        diff(ee::s06::f_0025bc48__generated, lobby_man_parse_name_suggestions, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x25_bc48);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x2000);
            w(c, DATA + 0x200c, DATA + 0x200);
            put_str(c, DATA + 0x200, b"a,b,c");
            w(c, DATA + 0x204, r.next_u32());
            for i in 0..8 {
                w(c, DATA + 0x220 + 4 * i, r.next_u32());
            }
        });
    }

}
