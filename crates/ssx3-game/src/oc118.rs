//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for 50 cold front-end / manager functions, each with a
//! differential test against the generated body.
//!
//! - **FE state changes** (`cFEStateTitle_Init`, `cFEStateUberTrick_OnActivate`,
//!   `cFEStateCharEquip_OnActivate`, `cFEStateChatRoom_*`): screen setup and
//!   child-event dispatch through the state's vtable.
//! - **Net FE toggles** (`NetFEState_ToggleBlockUser`,
//!   `NetFEState_ToggleMessengerBuddy`, `cFEStateChatRoom_v32`): make sure the
//!   buddy manager exists, reconnect it, then change a roster entry.
//! - **Managers** (`ScriptSoundBankMan_RemoveAll`, `ScriptAnimBankMan_RemoveAll`,
//!   `MusicPlayer_*`, `PATH_Plat*`, `MCManager_*`): iterate an array, call each
//!   entry and reset the slot.
//! - **Constructors** (`TypedProcess_ctor_2`, `MenuSysIntItem_ctor_3`,
//!   `ManualCamAlgoBase_ctor`, `cOVState_ViewPopup_ctor` callers): write the
//!   vtable, call the base constructor, bump a class counter.
//!
//! Calls follow [`crate::call`] (the same frame, registers, cycle charges and
//! unwind check as the generated body); loops deliver events at the same back
//! edges with [`crate::events`].

use crate::abi;
use crate::call::{call_ptr, guest, sp, Flow};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0019_4778, c_fe_state_title_init, "cFEStateTitle_Init"),
    (0x0019_5f70, fe_locale_v05, "FELocale_v05"),
    (
        0x0026_de58,
        world_state_notify_objects_restored,
        "WorldState_NotifyObjectsRestored",
    ),
    (
        0x0018_4be0,
        c_fe_state_uber_trick_on_activate,
        "cFEStateUberTrick_OnActivate",
    ),
    (
        0x0028_3658,
        script_sound_bank_man_remove_all,
        "ScriptSoundBankMan_RemoveAll",
    ),
    (0x0035_4788, typed_process_ctor_2, "TypedProcess_ctor_2"),
    (0x0014_de68, be_library_set_phase, "BELibrary_SetPhase"),
    (
        0x0027_5698,
        script_anim_bank_man_remove_all,
        "ScriptAnimBankMan_RemoveAll",
    ),
    (0x0015_d700, camera_base_init, "CameraBase_Init"),
    (
        0x0018_7cb8,
        fe_screen_autosave_popup_update,
        "FEScreenAutosavePopup_Update",
    ),
    (
        0x001a_bc50,
        net_fe_state_toggle_block_user,
        "NetFEState_ToggleBlockUser",
    ),
    (
        0x001a_be58,
        net_fe_state_toggle_messenger_buddy,
        "NetFEState_ToggleMessengerBuddy",
    ),
    (
        0x001b_22a0,
        c_fe_state_chat_room_v32,
        "cFEStateChatRoom_v32",
    ),
    (
        0x001b_3850,
        c_fe_state_chat_room_on_child_event,
        "cFEStateChatRoom_OnChildEvent",
    ),
    (
        0x001b_6368,
        net_config_auth_popup_dtor,
        "NetConfigAuthPopup_dtor",
    ),
    (
        0x001b_bd60,
        c_fe_state_create_duplicate_init,
        "cFEStateCreateDuplicate_Init",
    ),
    (
        0x001e_3570,
        message_center_is_new_player_flag_clear,
        "MessageCenter_IsNewPlayerFlagClear",
    ),
    (
        0x001e_50c8,
        ov_screen_message_center_open_view_popup,
        "OVScreenMessageCenter_OpenViewPopup",
    ),
    (
        0x001e_5780,
        c_ov_state_view_popup_set_body,
        "cOVState_ViewPopup_SetBody",
    ),
    (
        0x0026_60c8,
        c_shared_login_man_mc_open,
        "cSharedLoginMan_McOpen",
    ),
    (0x002a_e328, path_plat_remove_bank, "PATH_PlatRemoveBank"),
    (0x002a_f2f0, path_plat_get_status, "PATH_PlatGetStatus"),
    (
        0x002b_4070,
        music_player_set_playlist_mask,
        "MusicPlayer_SetPlaylistMask",
    ),
    (
        0x002b_4a20,
        music_player_load_song_data,
        "MusicPlayer_LoadSongData",
    ),
    (
        0x002c_d8f8,
        menu_sys_int_item_ctor_3,
        "MenuSysIntItem_ctor_3",
    ),
    (
        0x002c_ff88,
        menu_sys_color_swatch_item_measure,
        "MenuSysColorSwatchItem_Measure",
    ),
    (0x0012_9160, ai_update_rider_poses, "AI_UpdateRiderPoses"),
    (
        0x002c_4210,
        c_ps2_mem_card_do_list_dir,
        "cPS2MemCard_DoListDir",
    ),
    (
        0x002c_6848,
        c_ps2_mem_card_do_list_dir_all,
        "cPS2MemCard_DoListDirAll",
    ),
    (
        0x0015_f938,
        manual_cam_algo_base_ctor,
        "ManualCamAlgoBase_ctor",
    ),
    (
        0x0038_9098,
        ps2_graphics_man_set_clear_color,
        "PS2GraphicsMan_SetClearColor",
    ),
    (0x0038_9c38, irradiance_build_sh, "Irradiance_BuildSH"),
    (
        0x0023_e820,
        mc_manager_state04_no_card,
        "MCManager_State04_NoCard",
    ),
    (0x0023_eb80, mc_manager_state05, "MCManager_State05"),
    (0x0023_f1d8, mc_manager_state15, "MCManager_State15"),
    (
        0x0023_f4f8,
        mc_manager_state10_save,
        "MCManager_State10_Save",
    ),
    (
        0x0024_00b0,
        mc_manager_state1f_load,
        "MCManager_State1F_Load",
    ),
    (
        0x0024_1380,
        mc_manager_begin_slot_select,
        "MCManager_BeginSlotSelect",
    ),
    (
        0x0024_1d40,
        mc_manager_get_save_type_tag,
        "MCManager_GetSaveTypeTag",
    ),
    (
        0x001c_c7c8,
        c_keyboard_popup_type_current_key,
        "cKeyboardPopup_TypeCurrentKey",
    ),
    (
        0x003b_0600,
        rcmp_decoder_decode_frame,
        "RCMP_DECODER_DecodeFrame",
    ),
    (
        0x003b_1050,
        ps2_sony_codec_internal_decode_frame,
        "PS2_SONY_CODEC_INTERNAL_DecodeFrame",
    ),
    (
        0x0030_9dd0,
        w_script_man_table_has_function,
        "WScriptMan_TableHasFunction",
    ),
    (
        0x0035_59f8,
        mod_obj_add_magnet_modifier,
        "ModObj_AddMagnetModifier",
    ),
    (0x0026_adf0, path_data_parse, "PathData_Parse"),
    (
        0x0028_6e20,
        ssx_audio_system_enter_game_init,
        "SSXAudioSystem_EnterGameInit",
    ),
    (
        0x0019_93a0,
        c_fe_state_char_equip_on_activate,
        "cFEStateCharEquip_OnActivate",
    ),
    (
        0x0019_cb60,
        bolt_file_index_manager_reset,
        "BoltFileIndexManager_Reset",
    ),
    (
        0x0023_4be8,
        game_state_play_script_event,
        "GameState_PlayScriptEvent",
    ),
    (
        0x0025_36c8,
        vid_engine_player_start_playback,
        "VidEnginePlayer_StartPlayback",
    ),
];

// ---------------------------------------------------------------------------
// Small helpers mirroring the generated code's idioms.

/// A `gp`-relative global address.
#[inline(always)]
fn gpa(c: &Cpu, off: u32) -> u32 {
    abi::r32(c, GP).wrapping_add(off)
}

/// A `gp`-relative global written with a negative offset.
#[inline(always)]
fn gpn(c: &Cpu, off: u32) -> u32 {
    abi::r32(c, GP).wrapping_sub(off)
}

/// `lw` (sign-extended).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// `lhu`/`lbu`-free byte load helpers used below.
#[inline(always)]
fn lb(c: &mut Cpu, a: u32) -> u64 {
    c.lb(a)
}

/// A vtable dispatch: `lh(vt + delta); lw(vt + delta + 4); a0 = obj + delta`.
#[inline(always)]
fn vt_call(c: &mut Cpu, vt: u64, delta_off: u32, obj: u64, func_reg: usize, ret: u32) -> Flow {
    let base = (vt as u32).wrapping_add(delta_off);
    let delta = c.lh(base);
    let func = c.lw(base.wrapping_add(4));
    c.g[func_reg][0] = func;
    c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
    call_ptr(c, func as u32, ret)
}

// ---------------------------------------------------------------------------
// Front-end state changes.

/// `void cFEStateTitle_Init(cFEStateTitle* st)` (0x00194778): clear the
/// autosave counter, create the title screen from the screen engine named by
/// `st+0x10`, start it, then set the FE music screen.
pub fn c_fe_state_title_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = sx(0x004a_17c8); // "cFEStateTitle"
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0);
        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_7670, 0x0019_47a0)?; // StrHash
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0019_47b4)?; // UIEngine_CreateScreen
        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0019_47cc)?; // UIScreen_StartThread
        }
        c.cycles += 10;
        call_ptr(c, 0x0028_b180, 0x0019_47d4)?; // SSXAudioSystem_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_f140, 0x0019_47e0)?; // SSXAudioSystem_SetFEMusicScreen
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `int FELocale_v05(FELocale* loc, LocStringTable* table)` (0x00195f70): walk
/// the locale's table of string ids, resolving each until one is an entry in
/// the string table (a locale row at +8 per id), accumulating the table's
/// base offset; returns 0 when nothing resolves.
pub fn fe_locale_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        c.sq(S0, sp(c).wrapping_add(0x40));
        c.sq(S1, sp(c).wrapping_add(0x30));
        c.sq(S2, sp(c).wrapping_add(0x20));
        c.sq(S3, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
        c.g[S1][0] = 0x0100_0000;
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = 0x0100_0000;
        loop {
            c.cycles += 3;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = c.g[S1][0];
            let found = if c.g[A0][0] == 0 {
                false
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0019_5fac);
                call_ptr(c, 0x002c_6dc0, 0x0019_5fac)?; // LocStringTable_ResolveString
                if c.g[V0][0] != 0 {
                    c.lq(S0, sp(c).wrapping_add(0x40));
                    true
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S1][0];
                    false
                }
            };
            if found {
                break;
            }
            c.cycles += 5;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < 20) as u64;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S3][0] as u32));
            if c.g[V0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0019_5f98);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.lq(S0, sp(c).wrapping_add(0x40));
            break;
        }
        c.cycles += 6;
        c.lq(S1, sp(c).wrapping_add(0x30));
        c.lq(S2, sp(c).wrapping_add(0x20));
        c.lq(S3, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        abi::ret(c);
        Ok(())
    })
}

/// `void WorldState_NotifyObjectsRestored(WorldState*, void* arg)`
/// (0x0026de58): walk the process list at `gp+0x2898`, virtual-call each
/// process with `arg`, then hand the process back to the list iterator.
pub fn world_state_notify_objects_restored(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.g[A0][0] = sx(gpa(c, 0x2898));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0035_4ea8, 0x0026_de7c)?; // ProcessList_First
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            loop {
                c.cycles += 11;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                let target = c.g[V1][0] as u32;
                call_ptr(c, target, 0x0026_dea0)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x0035_4ef8, 0x0026_deb0)?; // ProcessList_Next
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    c.lq(S0, sp(c).wrapping_add(0x30));
                    break;
                }
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                if event_due(c) {
                    deliver_event_here(c, 0x0026_de90);
                }
            }
        } else {
            c.lq(S0, sp(c).wrapping_add(0x30));
        }
        c.cycles += 5;
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void cFEStateUberTrick_OnActivate(cFEState*, void* arg)` (0x00184be0):
/// select the left panel, notify both optional panels (`+0x4c`, `+0x50`) with
/// their own event id, then activate the base state.
pub fn c_fe_state_uber_trick_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0018_3b08, 0x0018_4bfc)?; // cFEStateUberTrick_SelectLeftPanel
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
        c.g[A1][0] = 1;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(8));
            let vt = c.g[V0][0];
            let obj = c.g[A2][0];
            vt_call(c, vt, 0x38, obj, V1, 0x0018_4c1c)?;
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[A1][0] = 0;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(8));
            let vt = c.g[V0][0];
            let obj = c.g[A2][0];
            vt_call(c, vt, 0x38, obj, V1, 0x0018_4c3c)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e4c0, 0x0018_4c48)?; // UIState_OnActivate
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void ScriptSoundBankMan_RemoveAll(ScriptSoundBankMan* m)` (0x00283658):
/// for each of the `n` banks (array at `+8`, stride 0xc) call its `+0xc`
/// virtual destructor when present.
pub fn script_sound_bank_man_remove_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
        c.g[S1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(8));
                if c.g[V1][0] != 0 {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    let vt = c.g[V0][0];
                    let obj = c.g[S0][0];
                    vt_call(c, vt, 0x10, obj, V1, 0x0028_36a8)?;
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0028_3680);
                }
            }
        }
        c.cycles += 6;
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void TypedProcess_ctor_2(TypedProcess* p, int, TypeInfo* ti)`
/// (0x00354788): build the base process, install the type descriptor from
/// `[ti]`, then bump the constructor count in the type table.
pub fn typed_process_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0035_4688, 0x0035_47a4)?; // GameProcess_ctor_2
        c.g[V1][0] = sx(0x0049_1e80);
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 4;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
        let vt = c.g[V0][0];
        let obj = c.g[S1][0];
        vt_call(c, vt, 0x10, obj, V1, 0x0035_47cc)?;
        c.g[A1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx(0x0044_aff0);
        c.g[V0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[RA][0] = c.ld(sp(c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.sw(c.g[A1][0] as u32, c.g[V1][0]);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void BELibrary_SetPhase(int phase)` (0x0014de68): leave the previous
/// phase's library (vtable `+0x18`/`+0x1c`) and enter the new one (vtable
/// `+0x20`/`+0x24`), recording the phase in `gp-0x1efc`.
pub fn be_library_set_phase(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[V0][0] = c.lw(gpn(c, 0x1efc));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        let same = c.g[V0][0] == c.g[S1][0];
        c.sd(sp(c), c.g[RA][0]);
        if !same {
            c.cycles += 19;
            c.g[S0][0] = sx(0x0053_08a8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
            let vt = c.g[V1][0];
            let obj = c.g[A2][0];
            vt_call(c, vt, 0x18, obj, A1, 0x0014_deac)?;
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
            c.sw(gpn(c, 0x1efc), c.g[S1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(4));
            let vt = c.g[V1][0];
            let obj = c.g[A1][0];
            vt_call(c, vt, 0x20, obj, V0, 0x0014_ded0)?;
        }
        c.cycles += 5;
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void ScriptAnimBankMan_RemoveAll(ScriptAnimBankMan* m)` (0x00275698):
/// same shape as [`script_sound_bank_man_remove_all`] with stride 0x14 and
/// the destructor slot at `+0x14`.
pub fn script_anim_bank_man_remove_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
        c.g[S1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(8));
                if c.g[V1][0] != 0 {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    let vt = c.g[V0][0];
                    let obj = c.g[S0][0];
                    vt_call(c, vt, 0x10, obj, V1, 0x0027_56e8)?;
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_56c0);
                }
            }
        }
        c.cycles += 6;
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void CameraBase_Init(CameraBase* cam)` (0x0015d700): clear the frame
/// (position, rotation, 1.0 scalar), load the three axes and the base
/// orientation into `+0x80`, then build the matrices.
pub fn camera_base_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.sw(sp(c), 0);
        c.g[V1][0] = c.g[A0][0];
        c.sw(sp(c).wrapping_add(4), 0);
        c.g[A1][0] = sx(0x004f_f140);
        c.sw(sp(c).wrapping_add(8), 0);
        c.sw(sp(c).wrapping_add(0xc), 0);
        c.sd(sp(c).wrapping_add(0x10), c.g[RA][0]);
        c.lq(6, sp(c));
        c.swc1(0, sp(c).wrapping_add(4));
        c.swc1(0, sp(c).wrapping_add(0xc));
        c.sw(sp(c), 0);
        c.sw(sp(c).wrapping_add(8), 0);
        c.swc1(12, c.g[V1][0] as u32);
        c.lq(2, sp(c));
        c.swc1(13, (c.g[V1][0] as u32).wrapping_add(4));
        c.swc1(14, (c.g[V1][0] as u32).wrapping_add(8));
        c.sq(6, (c.g[V1][0] as u32).wrapping_add(0x20));
        c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x30));
        c.swc1(12, (c.g[V1][0] as u32).wrapping_add(0xc));
        c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x14));
        c.lq(2, c.g[A1][0] as u32);
        c.g[RA][0] = sx(0x0015_d770);
        c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x80));
        call_ptr(c, 0x0015_d928, 0x0015_d770)?; // CameraBase_UpdateMatrices
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x10));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void FEScreenAutosavePopup_Update(FEScreenAutosavePopup* p)` (0x00187cb8):
/// tick the base state; every 240 ticks look up the autosave screen, start
/// its thread (or report failure) and close the top states.
pub fn fe_screen_autosave_popup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e510, 0x0018_7ccc)?; // UIState_Update
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = 0xf0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let tick = c.g[V0][0] != c.g[V1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
        if !tick {
            c.cycles += 10;
            c.g[A0][0] = sx(0x0045_dc60);
            c.g[RA][0] = sx(0x0018_7cec);
            call_ptr(c, 0x0031_7670, 0x0018_7cec)?; // StrHash
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_c778, 0x0018_7cf8)?; // UIScreen_LookupValue
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = 0xffff;
            let missing = c.g[A1][0] == c.g[V0][0];
            c.g[A1][0] = c.g[A1][0] & 0xffff;
            if !missing {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                c.g[A2][0] = 1;
                call_ptr(c, 0x0039_c870, 0x0018_7d14)?; // UIScreen_StartThread
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f190, 0x0018_7d24)?; // UIStateMgr_CloseTopStates
        }
        c.cycles += 4;
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void NetFEState_ToggleBlockUser(NetFEState* s, void* a, void* b)`
/// (0x001abc50): make sure the buddy manager exists and is connected, then
/// change roster entry `a` to block (kind 1).
pub fn net_fe_state_toggle_block_user(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[V0][0] = c.lw(gpa(c, 0x238));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A2][0];
        let have = c.g[V0][0] != 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x6c4), 0);
        if !have {
            c.cycles += 2;
            c.g[RA][0] = sx(0x001a_bc7c);
            call_ptr(c, 0x0026_0f80, 0x001a_bc7c)?; // BuddyMan_Create
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw(gpa(c, 0x238));
        call_ptr(c, 0x0026_1460, 0x001a_bc84)?; // BuddyMan_IsConnected
        let connected = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gpa(c, 0x238));
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gpa(c, 0x238));
            call_ptr(c, 0x0026_13c8, 0x001a_bc94)?; // BuddyMan_Reconnect
            c.g[V0][0] = c.lw(gpa(c, 0x238));
        }
        c.cycles += 2;
        let present = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw(gpa(c, 0x238));
        if !present {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gpn(c, 0xc8));
            call_ptr(c, 0x0025_b800, 0x001a_bca8)?; // LobbyMan_Disconnect
            c.g[A0][0] = c.lw(gpa(c, 0x238));
        }
        c.cycles += 9;
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = 1;
        call_ptr(c, 0x0026_1ef8, 0x001a_bcbc)?; // BuddyMan_ChangeRosterEntry
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void NetFEState_ToggleMessengerBuddy(NetFEState* s, void* a, void* b)`
/// (0x001abe58): as [`net_fe_state_toggle_block_user`] but changing the entry
/// to kind 0 (a messenger buddy).
pub fn net_fe_state_toggle_messenger_buddy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[V0][0] = c.lw(gpa(c, 0x238));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A2][0];
        let have = c.g[V0][0] != 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x6c4), 0);
        if !have {
            c.cycles += 2;
            c.g[RA][0] = sx(0x001a_be84);
            call_ptr(c, 0x0026_0f80, 0x001a_be84)?; // BuddyMan_Create
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw(gpa(c, 0x238));
        call_ptr(c, 0x0026_1460, 0x001a_be8c)?; // BuddyMan_IsConnected
        let connected = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gpa(c, 0x238));
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gpa(c, 0x238));
            call_ptr(c, 0x0026_13c8, 0x001a_be9c)?; // BuddyMan_Reconnect
            c.g[V0][0] = c.lw(gpa(c, 0x238));
        }
        c.cycles += 2;
        let present = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw(gpa(c, 0x238));
        if !present {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gpn(c, 0xc8));
            call_ptr(c, 0x0025_b800, 0x001a_beb0)?; // LobbyMan_Disconnect
            c.g[A0][0] = c.lw(gpa(c, 0x238));
        }
        c.cycles += 9;
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0026_1ef8, 0x001a_bec4)?; // BuddyMan_ChangeRosterEntry
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void cFEStateChatRoom_v32(cFEStateChatRoom* s)` (0x001b22a0): ensure the
/// buddy manager exists and is connected, then set the local presence to
/// "chat room" (kind 1/3) using the lobby's kick reason as the payload.
pub fn c_fe_state_chat_room_v32(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[V0][0] = c.lw(gpa(c, 0x238));
        c.sq(S0, sp(c).wrapping_add(0x10));
        let have = c.g[V0][0] != 0;
        c.sd(sp(c), c.g[RA][0]);
        if !have {
            c.cycles += 2;
            c.g[RA][0] = sx(0x001b_22bc);
            call_ptr(c, 0x0026_0f80, 0x001b_22bc)?; // BuddyMan_Create
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw(gpa(c, 0x238));
        call_ptr(c, 0x0026_1460, 0x001b_22c4)?; // BuddyMan_IsConnected
        let connected = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gpa(c, 0x238));
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gpa(c, 0x238));
            call_ptr(c, 0x0026_13c8, 0x001b_22d4)?; // BuddyMan_Reconnect
            c.g[V0][0] = c.lw(gpa(c, 0x238));
        }
        c.cycles += 2;
        let present = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw(gpn(c, 0xc8));
        if !present {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gpn(c, 0xc8));
            call_ptr(c, 0x0025_b800, 0x001b_22e8)?; // LobbyMan_Disconnect
            c.g[A0][0] = c.lw(gpn(c, 0xc8));
        }
        c.cycles += 13;
        c.g[RA][0] = sx(0x001b_22f4);
        c.g[S0][0] = c.lw(gpa(c, 0x238));
        call_ptr(c, 0x0025_da90, 0x001b_22f4)?; // LobbyMan_GetKickReason
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 3;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        call_ptr(c, 0x0026_2768, 0x001b_2310)?; // BuddyMan_SetPresence
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void cFEStateChatRoom_OnChildEvent(cFEStateChatRoom* s, UIEvent* ev)`
/// (0x001b3850): dispatch the event id at `ev+0x18` to the buddy-action
/// handler (0x80b), to char select (0x800/0x801) or to the base net state.
pub fn c_fe_state_chat_room_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
        c.g[V0][0] = 0x801;
        c.sd(sp(c), c.g[RA][0]);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        let is_801 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[V1][0] < 0x802) as u64;
        if is_801 {
            // `break 'b_1b38ac`: the char-select-2 handler.
            c.cycles += 4;
            c.g[RA][0] = sx(0x001b_38b4);
            call_ptr(c, 0x001b_37d8, 0x001b_38b4)?;
            c.g[RA][0] = c.ld(sp(c));
        } else {
            // Head of `'b_1b389c`/`'b_1b3880`: both arms charge it.
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                // `break 'b_1b3880`: only 0x80b reaches the buddy handler.
                c.cycles += 3;
                c.g[V0][0] = 0x80b;
                if c.g[V1][0] != c.g[V0][0] {
                    // `break 'b_1b38bc`.
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001b_38c4);
                    call_ptr(c, 0x001a_97b8, 0x001b_38c4)?;
                    c.g[RA][0] = c.ld(sp(c));
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001b_3894);
                    call_ptr(c, 0x001b_35b0, 0x001b_3894)?;
                    c.g[RA][0] = c.ld(sp(c));
                }
            } else {
                c.g[V0][0] = 0x800;
                c.cycles += 2;
                if c.g[V1][0] == c.g[V0][0] {
                    // `break 'b_1b389c`: the char-select handler.
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001b_38a4);
                    call_ptr(c, 0x001b_3760, 0x001b_38a4)?;
                    c.g[RA][0] = c.ld(sp(c));
                } else {
                    // `break 'b_1b38bc`.
                    c.cycles += 2;
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001b_38c4);
                    call_ptr(c, 0x001a_97b8, 0x001b_38c4)?;
                    c.g[RA][0] = c.ld(sp(c));
                }
            }
        }
        c.cycles += 2;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    })
}

/// `void NetConfigAuthPopup_dtor(NetConfigAuthPopup* p, int flags)`
/// (0x001b6368): install the class vtable, free the auth buffers, then free
/// the popup's own buffer and run the base destructor.
pub fn net_config_auth_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[V0][0] = sx(0x0046_89f0);
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d0));
        let no_auth = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        let mut charged = false;
        if !no_auth {
            c.cycles += 5;
            call_ptr(c, 0x0025_63a8, 0x001b_639c)?; // DNASAuth_FreeBuffers
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d0));
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                c.g[A1][0] = 3;
                call_ptr(c, 0x0025_61e0, 0x001b_63b0)?; // DNASAuth_dtor
                charged = true;
            }
            if !charged {
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6fc));
            }
        }
        if no_auth || charged {
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6fc));
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x001b_63c4)?; // EA_MemFree
            c.sw((c.g[S0][0] as u32).wrapping_add(0x6fc), 0);
        }
        c.g[A0][0] = c.g[S0][0];
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_85d0, 0x001b_63d4)?; // NetFEState_dtor
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void cFEStateCreateDuplicate_Init(cFEStateCreateDuplicate* s)`
/// (0x001bbd60): hash the screen name, create the screen through the engine,
/// start its thread, then fill in the name suggestions and activate.
pub fn c_fe_state_create_duplicate_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[V0][0] = c.lw(gpn(c, 0xc8));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x260));
        c.g[A0][0] = sx(0x0046_4c50);
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x001b_bd90);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x6d8), c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x001b_bd90)?; // StrHash
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001b_bda4)?; // UIEngine_CreateScreen
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001b_bdbc)?; // UIScreen_StartThread
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001b_c400, 0x001b_bdc4)?; // cFEStateCreateDuplicate_FillNameSuggestions
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001a_8768, 0x001b_bdcc)?; // NetFEState_Init
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `bool MessageCenter_IsNewPlayerFlagClear(void)` (0x001e3570): false when
/// the new-player marker (`0x535bc8+0x49`) is set or the save type is 6/9;
/// otherwise the slot-0 new-player flag is clear.
pub fn message_center_is_new_player_flag_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
        c.sd(sp(c), c.g[RA][0]);
        call_ptr(c, 0x0014_dc80, 0x001e_3580)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x001e_358c)?; // BELibrary_GetInterface
        c.g[V1][0] = sx(0x0053_5bc8);
        c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
        let marked = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !marked {
            c.cycles += 4;
            c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x4a));
            c.g[V0][0] = 6;
            let six = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 9;
            if six {
                c.cycles += 2;
                c.g[V0][0] = 0;
            } else if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 2;
                c.cycles += 9;
                call_ptr(c, 0x0014_dc80, 0x001e_35c8)?; // BELibrary_GetInstance
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                call_ptr(c, 0x0014_dd58, 0x001e_35d4)?; // BELibrary_GetInterface
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_7580, 0x001e_35e0)?; // BENewPlayerInt_GetSlotFlag3
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            } else {
                c.cycles += 2;
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    })
}

/// `void OVScreenMessageCenter_OpenViewPopup(OVScreenMessageCenter* s,
/// void* a, void* b, void* c)` (0x001e50c8): allocate a 0x70-byte view popup,
/// construct it in place and push it on the state manager's popup stack.
pub fn ov_screen_message_center_open_view_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.g[A3][0] = 0;
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.g[S2][0] = c.g[A1][0];
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = sx(0x0046_e240);
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = 0x70;
        c.g[A2][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x001e_5100)?; // EA_MemAllocNamed
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[T0][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x001e_51d0, 0x001e_5118)?; // cOVState_ViewPopup_ctor
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x17c));
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f290, 0x001e_5130)?; // UIStateMgr_PushPopup
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void cOVState_ViewPopup_SetBody(cOVState_ViewPopup* s, ViewPopupData* d)`
/// (0x001e5780): when the body index is >= 2, flag the text as dirty, set its
/// id and refresh the object; otherwise leave the view unchanged.
pub fn c_ov_state_view_popup_set_body(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V1][0] = 0x18;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.mult(0, 5, 5, 3);
        c.g[V0][0] = sx(0x004c_6c08);
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sd(sp(c), c.g[RA][0]);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(8));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        let ge2 = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if ge2 {
            c.cycles += 1;
            c.lq(S0, sp(c).wrapping_add(0x10));
        } else {
            c.cycles += 2;
            if (c.g[V1][0] as i64) < 0 {
                c.lq(S0, sp(c).wrapping_add(0x10));
            } else {
                c.cycles += 14;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.g[V0][0] | 0x80;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                call_ptr(c, 0x003a_1000, 0x001e_57d4)?; // UIText_SetTextId
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[A1][0] = 1;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(8));
                let vt = c.g[V0][0];
                let obj = c.g[A2][0];
                vt_call(c, vt, 0x48, obj, V1, 0x001e_57f0)?;
                c.cycles += 1;
                c.lq(S0, sp(c).wrapping_add(0x10));
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `int cSharedLoginMan_McOpen(cSharedLoginMan* m, void* a, char* buf,
/// int slot)` (0x002660c8): format the slot's name into a stack buffer, then
/// ask the memory card manager to open it; returns 4.
pub fn c_shared_login_man_mc_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0xb0));
        c.sq(S1, sp(c).wrapping_add(0x90));
        c.sq(S2, sp(c).wrapping_add(0x80));
        c.g[S1][0] = c.g[A1][0];
        c.sq(S3, sp(c).wrapping_add(0x70));
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[SP][0];
        c.sq(S0, sp(c).wrapping_add(0xa0));
        c.sd(sp(c).wrapping_add(0x60), c.g[RA][0]);
        c.g[S3][0] = c.g[A3][0];
        c.g[A0][0] = c.g[A2][0];
        call_ptr(c, 0x003f_a1d0, 0x0026_60f8)?; // SLoginMc_GetPortSlot
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = sx(0x004a_33e8);
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_7828, 0x0026_6114)?; // sprintf
        c.g[A0][0] = c.lw(gpa(c, 0x2e8));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        call_ptr(c, 0x002c_6968, 0x0026_6124)?; // cPS2MemCard_RequestOpen
        c.g[V0][0] = 4;
        c.lq(S0, sp(c).wrapping_add(0xa0));
        c.lq(S1, sp(c).wrapping_add(0x90));
        c.lq(S2, sp(c).wrapping_add(0x80));
        c.lq(S3, sp(c).wrapping_add(0x70));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x60));
        c.sw(gpa(c, 0x2e4), c.g[V0][0]);
        c.g[SP][0] = sx(sp(c).wrapping_add(0xb0));
        abi::ret(c);
        Ok(())
    })
}

/// `void PATH_PlatRemoveBank(int slot)` (0x002ae328): find the platform's
/// bank slot; when it is live, remove the SND bank and clear the slot's
/// fields; returns 0 (or -1 when the slot is not usable).
pub fn path_plat_remove_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = sx(0xffff_ffff);
        call_ptr(c, 0x002a_e230, 0x002a_e340)?; // PATH_PlatFindBankSlot
        c.g[A0][0] = c.g[V0][0];
        let live = (c.g[A0][0] as i64) >= 0;
        c.g[V1][0] = 0x28;
        if live {
            c.cycles += 7;
            c.g[V0][0] = sx(0x004d_3ef8);
            c.mult(0, 3, 4, 3);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
            let empty = (c.g[A0][0] as i64) < 0;
            c.g[V0][0] = 0;
            if !empty {
                c.cycles += 9;
                c.g[A0][0] = sx(0xffff_ffff);
                call_ptr(c, 0x003b_6300, 0x002a_e378)?; // SND_BankRemove
                c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
                c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[S1][0]);
                c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0);
                c.g[V0][0] = 0;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(0xffff_ffff);
        }
        c.cycles += 5;
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `int PATH_PlatGetStatus(int plat, int* out_a, int* out_b)` (0x002af2f0):
/// look the platform up in the bank table, refresh its status if requested,
/// copy the two status words out and return the bank's main status.
pub fn path_plat_get_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.g[V0][0] = sx(0x004d_3e98);
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sd(sp(c), c.g[RA][0]);
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.lw(c.g[A0][0] as u32);
        c.g[S2][0] = c.g[A2][0];
        let absent = c.g[S0][0] == 0;
        c.g[V0][0] = sx(0xffff_fff8);
        if !absent {
            c.cycles += 3;
            c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x65));
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = 0;
                call_ptr(c, 0x002a_efa8, 0x002a_f33c)?; // PATH_PlatRefreshStatus
            }
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            c.cycles += 6;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
            c.sw(c.g[S2][0] as u32, c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            }
        }
        c.cycles += 6;
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void MusicPlayer_SetPlaylistMask(MusicPlayer* m, u32 mask)`
/// (0x002b4070): clear the playlist, then enable every song whose bit is set
/// in `mask`, finally poke the empty-playlist hook.
pub fn music_player_set_playlist_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        call_ptr(c, 0x002b_3eb0, 0x002b_4090)?; // MusicPlayer_ClearPlaylist
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3ec));
        c.g[S0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0] >> (c.g[S0][0] & 63);
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.g[V0][0] & 1;
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                let zero = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if !zero {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x002b_3fe8, 0x002b_40bc)?; // MusicPlayer_EnableSong
                }
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3ec));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[S2][0] >> (c.g[S0][0] & 63);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_40a0);
                }
            }
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x002b_4060, 0x002b_40d8)?; // MusicPlayer_EmptyPlaylistHook
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void MusicPlayer_LoadSongData(MusicPlayer* m, void* file, int mode)`
/// (0x002b4a20): cancel any pending load, then either load the buffer into
/// the async file handle (mode 0) or into the engine file (mode != 0).
pub fn music_player_load_song_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A2][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5438));
        let none = c.g[A0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !none {
            c.cycles += 6;
            call_ptr(c, 0x003d_faf0, 0x002b_4a50)?; // AsyncFile_Cancel
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5438));
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x003d_f9d8, 0x002b_4a60)?; // AsyncFile_GetResult
        }
        c.cycles += 2;
        let direct = c.g[S1][0] != 0;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5434));
        if !direct {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = 0x6400;
            call_ptr(c, 0x003d_f748, 0x002b_4a74)?; // AsyncFile_LoadInto
            c.sw((c.g[S0][0] as u32).wrapping_add(0x5438), c.g[V0][0]);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = 0x6400;
            call_ptr(c, 0x003e_1b68, 0x002b_4a88)?; // EAFile_LoadFileIntoEx
        }
        c.cycles += 6;
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `MenuSysIntItem* MenuSysIntItem_ctor_3(MenuSysIntItem* p, int a1, int a2,
/// int a3, int t0, int t1)` (0x002cd8f8): base menu item, class vtable, then
/// the extra fields.
pub fn menu_sys_int_item_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x60));
        c.sq(S0, sp(c).wrapping_add(0x50));
        c.g[S0][0] = c.g[A0][0];
        c.sq(S1, sp(c).wrapping_add(0x40));
        c.sq(S2, sp(c).wrapping_add(0x30));
        c.g[S1][0] = c.g[T1][0];
        c.sq(S3, sp(c).wrapping_add(0x20));
        c.g[S2][0] = c.g[A2][0];
        c.sq(S4, sp(c).wrapping_add(0x10));
        c.g[S3][0] = c.g[A3][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S4][0] = c.g[T0][0];
        call_ptr(c, 0x002c_a258, 0x002c_d92c)?; // MenuSysItem_ctor
        c.g[V1][0] = sx(0x0048_6bb0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S3][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S4][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0);
        c.lq(S0, sp(c).wrapping_add(0x50));
        c.lq(S1, sp(c).wrapping_add(0x40));
        c.lq(S2, sp(c).wrapping_add(0x30));
        c.lq(S3, sp(c).wrapping_add(0x20));
        c.lq(S4, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x60));
        abi::ret(c);
        Ok(())
    })
}

/// `void MenuSysColorSwatchItem_Measure(MenuSysColorSwatchItem* item,
/// MenuSysItem* row)` (0x002cff88): measure the base row, pad its height by 9
/// and floor it at 29, then store the 16-byte extent into the item.
pub fn menu_sys_color_swatch_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[A3][0] = 0;
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[T0][0] = 0;
        c.sd(sp(c).wrapping_add(0x10), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x002c_a4c8, 0x002c_ffac)?; // MenuSysItem_MeasureRow
        c.lwc1(2, sp(c).wrapping_add(8));
        c.g[V0][0] = c.g[S0][0];
        c.g[AT][0] = 0x4210_0000;
        c.mtc1(1, 3);
        c.lwc1(1, sp(c).wrapping_add(0xc));
        c.add_s(2, 2, 3);
        c.g[AT][0] = 0x41e8_0000;
        c.mtc1(1, 0);
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x10));
        c.max_s(1, 1, 0);
        c.swc1(2, sp(c).wrapping_add(8));
        c.swc1(1, sp(c).wrapping_add(0xc));
        c.ldl(3, sp(c).wrapping_add(7));
        c.ldr(3, sp(c));
        c.ldl(4, sp(c).wrapping_add(0xf));
        c.ldr(4, sp(c).wrapping_add(8));
        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(7));
        c.sdr(3, c.g[S0][0] as u32);
        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(8));
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void AI_UpdateRiderPoses(World* w)` (0x00129160): tick, update the
/// skeleton and rebuild the bone matrices for the world's first `+0x78`
/// riders (the rider pointers start at `+0x28`).
pub fn ai_update_rider_poses(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x78));
        c.g[S1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            loop {
                c.cycles += 14;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 12);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, 0x0011_eb60, 0x0012_91a0)?; // Rider_TickAnimation
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                call_ptr(c, 0x0011_eb98, 0x0012_91a8)?; // Rider_UpdateSkeleton
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x780));
                call_ptr(c, 0x0031_03f0, 0x0012_91b8)?; // RiderModel_BuildBoneMatrices
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x78));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0012_9190);
                }
            }
        }
        c.cycles += 6;
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoListDir(cPS2MemCard* m)` (0x002c4210): clear the
/// 0x1000-byte directory buffer and ask the IOP for the entries of the
/// current directory; state 1 on failure, state 6 on success.
pub fn c_ps2_mem_card_do_list_dir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[A1][0] = 0;
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x1000;
        c.g[S1][0] = c.g[A0][0];
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c0));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6448, 0x002c_4238)?; // EA_MemSet
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[T1][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x139));
        c.g[A3][0] = 0;
        c.g[T0][0] = 6;
        call_ptr(c, 0x0040_a688, 0x002c_4254)?; // sceMcGetDir
        let failed = (c.g[V0][0] as i64) < 0;
        c.sw(gpa(c, 0x850), c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x002c_48c0, 0x002c_4268)?; // cPS2MemCard_SetState
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 6;
            call_ptr(c, 0x002c_48c0, 0x002c_427c)?; // cPS2MemCard_SetState
        }
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.cycles += 4;
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoListDirAll(cPS2MemCard* m)` (0x002c6848): as
/// [`c_ps2_mem_card_do_list_dir`] but listing every directory (`t0 = 0x40`)
/// and entering state 0x3d on success.
pub fn c_ps2_mem_card_do_list_dir_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[A1][0] = 0;
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x1000;
        c.g[S1][0] = c.g[A0][0];
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c0));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6448, 0x002c_6870)?; // EA_MemSet
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[T1][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x139));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0x40;
        call_ptr(c, 0x0040_a688, 0x002c_688c)?; // sceMcGetDir
        let failed = (c.g[V0][0] as i64) < 0;
        c.sw(gpa(c, 0x850), c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x002c_48c0, 0x002c_68a0)?; // cPS2MemCard_SetState
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x3d;
            call_ptr(c, 0x002c_48c0, 0x002c_68b4)?; // cPS2MemCard_SetState
        }
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.cycles += 4;
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void ManualCamAlgoBase_ctor(ManualCamAlgoBase* a)` (0x0015f938): install
/// the class vtable, default pitch/yaw/fov, the axis quad and the zeroed
/// state words.
pub fn manual_cam_algo_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[V0][0] = sx(0x0045_bde0);
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        call_ptr(c, 0x0016_7e30, 0x0015_f958)?; // CameraAlgorithm_DefaultFov
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 1);
        c.g[V1][0] = sx(0x0045_bd78);
        c.lwc1(2, gpn(c, 0x6c48));
        c.g[V0][0] = 0x4e;
        c.swc1(0, c.g[S0][0] as u32);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[A0][0] = sx(0x004f_f130);
        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(4));
        c.swc1(2, (c.g[S0][0] as u32).wrapping_add(8));
        c.g[A1][0] = 0x4f;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x30), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x34), 0);
        c.g[RA][0] = c.ld(sp(c));
        c.lq(3, c.g[A0][0] as u32);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void PS2GraphicsMan_SetClearColor(PS2GraphicsMan* g, float* rgb)`
/// (0x00389098): scale each channel by 255, truncate to an integer and pack
/// them into the clear colour word (`+0x6ae0`).
pub fn ps2_graphics_man_set_clear_color(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.swc1(20, sp(c).wrapping_add(0x40));
        c.g[AT][0] = 0x437f_0000;
        c.mtc1(1, 20);
        c.lwc1(12, c.g[S1][0] as u32);
        c.mul_s(12, 12, 20);
        call_ptr(c, 0x0041_39f8, 0x0038_90cc)?; // __fixunssfsi
        c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(4));
        c.g[S0][0] = c.g[V0][0];
        c.mul_s(12, 12, 20);
        call_ptr(c, 0x0041_39f8, 0x0038_90dc)?; // __fixunssfsi
        c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(8));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
        c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
        c.mul_s(12, 12, 20);
        call_ptr(c, 0x0041_39f8, 0x0038_90f0)?; // __fixunssfsi
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
        c.g[RA][0] = c.ld(sp(c));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x6ae0), c.g[S0][0]);
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.lwc1(20, sp(c).wrapping_add(0x40));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        abi::ret(c);
        Ok(())
    })
}

/// `void Irradiance_BuildSH(Irradiance* irr, Light* sun)` (0x00389c38): clear
/// the coefficients, add the sky's ambient entry scaled by 1.0, then add the
/// sun's camera-relative light when there is one.
pub fn irradiance_build_sh(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0038_9260, 0x0038_9c54)?; // Irradiance_Clear
        c.g[V0][0] = sx(0x0050_0000);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x47f8));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(4));
        if (c.g[V1][0] as i64) > 0 {
            c.cycles += 9;
            c.g[A1][0] = c.lw(gpa(c, 0x12d4));
            call_ptr(c, 0x0038_abf8, 0x0038_9c70)?; // Irradiance_GetEntry
            c.g[A2][0] = sx(0x0050_4810);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0038_9590, 0x0038_9c8c)?; // Irradiance_AddScaled
        }
        c.cycles += 2;
        let no_sun = c.g[S1][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_sun {
            c.cycles += 4;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0038_9cb8, 0x0038_9ca4)?; // Irradiance_AddCameraRelativeLight
        }
        c.cycles += 5;
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void MCManager_State04_NoCard(MCManager* m)` (0x0023e820): when the card
/// operation is idle, end it (or report it done).
pub fn mc_manager_state04_no_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0023_e848)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0023_e85c)?;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !idle {
                c.cycles += 4;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0023_ea30, 0x0023_e86c)?; // MCManager_EndOperation
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x0023_e880)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0023_ea90, 0x0023_e890)?; // MCManager_OnOperationDone
                }
            }
        }
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void MCManager_State05(MCManager* m)` (0x0023eb80): as state 4 but ends
/// the operation with error 1.
pub fn mc_manager_state05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0023_eba8)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0023_ebbc)?;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !idle {
                c.cycles += 4;
                c.g[A1][0] = 1;
                call_ptr(c, 0x0023_ea30, 0x0023_ebcc)?; // MCManager_EndOperation
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x0023_ebe0)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0023_ea90, 0x0023_ebf0)?; // MCManager_OnOperationDone
                }
            }
        }
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void MCManager_State15(MCManager* m)` (0x0023f1d8): as state 4 but ending
/// the operation with error 0.
pub fn mc_manager_state15(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0023_f200)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0023_f214)?;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !idle {
                c.cycles += 4;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0023_ea30, 0x0023_f224)?; // MCManager_EndOperation
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x0023_f238)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0023_ea90, 0x0023_f248)?; // MCManager_OnOperationDone
                }
            }
        }
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void MCManager_State10_Save(MCManager* m)` (0x0023f4f8): update the save
/// progress fraction, then finish the progress bar and kick the save (or
/// report that it finished).
pub fn mc_manager_state10_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xe4));
        c.cvt_s_w(0, 0);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xf0));
        c.cvt_s_w(1, 1);
        c.div_s(0, 0, 1);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x10c));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x248));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0023_f538)?;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0024_1180, 0x0023_f54c)?; // MCManager_EndProgress
        } else {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
            c.cycles += 6;
            c.g[A1][0] = 0x11;
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            let target = c.g[V1][0] as u32;
            call_ptr(c, target, 0x0023_f568)?;
        }
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void MCManager_State1F_Load(MCManager* m)` (0x002400b0): when the card is
/// ready, start the load and the progress bar; otherwise finish it.
pub fn mc_manager_state1f_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x248));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0024_00d8)?;
        let not_ready = c.g[V0][0] == 0;
        c.g[A1][0] = 0x24;
        if not_ready {
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
            c.g[A1][0] = 0x20;
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            let target = c.g[V1][0] as u32;
            call_ptr(c, target, 0x0024_0120)?;
        } else {
            c.cycles += 10;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            let target = c.g[V1][0] as u32;
            call_ptr(c, target, 0x0024_00f4)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0024_0100)?;
        }
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void MCManager_BeginSlotSelect(MCManager* m)` (0x00241380): reset the
/// slot-selection state, select slot 0 and enter the "choose a slot" state.
pub fn mc_manager_begin_slot_select(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[V0][0] = sx(0xffff_ffff);
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[A1][0] = 0;
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = 1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x340), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xdc), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x42c), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xf8), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x444), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x430), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xfc), c.g[V0][0]);
        call_ptr(c, 0x0023_fb18, 0x0024_13c0)?; // MCManager_SetSlot
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023_fae0, 0x0024_13c8)?; // MCManager_RefreshSlot
        c.g[V0][0] = 2;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x338), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x130), c.g[V0][0]);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        let target = c.g[V1][0] as u32;
        call_ptr(c, target, 0x0024_13ec)?;
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `char* MCManager_GetSaveTypeTag(MCManager* m, int type, UStr* out)`
/// (0x00241d40): the English tag for a save type ("new"/"current"/"other"),
/// built into `out`.
pub fn mc_manager_get_save_type_tag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
        c.g[A0][0] = c.g[A2][0];
        c.g[V0][0] = 1;
        let one = c.g[A1][0] == c.g[V0][0];
        c.sd(sp(c), c.g[RA][0]);
        if one {
            // `break 'b_241d94`: the "current" tag.
            c.cycles += 5;
            c.g[A1][0] = sx(0x004a_2cb0);
            call_ptr(c, 0x002c_2540, 0x0024_1da0)?; // UStr_FromAscii_2
            c.g[RA][0] = c.ld(sp(c));
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 2;
                let two = c.g[A1][0] == c.g[V0][0];
                c.g[RA][0] = c.ld(sp(c));
                if two {
                    // `break 'b_241da8`: the "other" tag.
                    c.cycles += 4;
                    c.g[A1][0] = sx(0x004a_2cb8);
                    call_ptr(c, 0x002c_2540, 0x0024_1db4)?; // UStr_FromAscii_2
                    c.g[RA][0] = c.ld(sp(c));
                } else {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 2;
                let zero = c.g[A1][0] == 0;
                c.g[RA][0] = c.ld(sp(c));
                if zero {
                    // `break 'b_241d80`: the "new" tag.
                    c.cycles += 5;
                    c.g[A1][0] = sx(0x004a_2ca8);
                    call_ptr(c, 0x002c_2540, 0x0024_1d8c)?; // UStr_FromAscii_2
                    c.g[RA][0] = c.ld(sp(c));
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.cycles += 2;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    })
}

/// `void cKeyboardPopup_TypeCurrentKey(cKeyboardPopup* p)` (0x001cc7c8):
/// insert the current keyboard character (the shifted row when the key is
/// shifted) into the popup's text field.
pub fn c_keyboard_popup_type_current_key(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
        c.sd(sp(c), c.g[RA][0]);
        let base = c.g[A0][0] as u32;
        c.g[V0][0] = c.lw(base.wrapping_add(0x68));
        c.g[V1][0] = c.lw(base.wrapping_add(0x6c));
        let no_shift = c.g[V1][0] == 0;
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        if no_shift {
            c.cycles += 1;
            c.g[V1][0] = (0 < c.g[V0][0]) as u64;
        } else {
            c.cycles += 2;
            c.g[V1][0] = c.g[V0][0] ^ 1;
        }
        c.cycles += 5;
        c.g[A1][0] = c.lw(base.wrapping_add(0x140));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf));
        c.g[V0][0] = (c.g[V0][0] < 0xc) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(base.wrapping_add(0x6c));
            c.g[V1][0] = c.g[V0][0] ^ 1;
        }
        c.cycles += 2;
        let shifted = c.g[V1][0] == 0;
        c.g[V0][0] = sx(0x0044_0000);
        if !shifted {
            c.cycles += 6;
            c.g[V0][0] = sx(0x0044_1390);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = c.lb(c.g[V0][0] as u32);
            call_ptr(c, 0x001c_c620, 0x001c_c820)?; // cKeyboardPopup_InsertChar
        } else {
            c.cycles += 6;
            c.g[V0][0] = sx(0x0044_13e8);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = c.lb(c.g[V0][0] as u32);
            call_ptr(c, 0x001c_c620, 0x001c_c83c)?; // cKeyboardPopup_InsertChar
        }
        c.g[RA][0] = c.ld(sp(c));
        c.cycles += 2;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    })
}

/// `void RCMP_DECODER_DecodeFrame(RCMP_DECODER* d, void* arg)` (0x003b0600):
/// make sure a picture is queued (starting the decoder when needed), then
/// pass the frame to the current output's `+0x18` virtual method.
pub fn rcmp_decoder_decode_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        let queued = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !queued {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
                c.g[A2][0] = c.g[S0][0];
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x003b_063c)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            }
            c.cycles += 2;
            let no_picture = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if no_picture {
                c.cycles += 1;
                c.lq(S0, sp(c).wrapping_add(0x20));
                c.cycles += 4;
                c.lq(S1, sp(c).wrapping_add(0x10));
                c.g[RA][0] = c.ld(sp(c));
                c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
                abi::ret(c);
                return Ok(());
            }
        }
        c.cycles += 9;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x003b_0664)?;
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.cycles += 4;
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void PS2_SONY_CODEC_INTERNAL_DecodeFrame(Codec* c, int limit)`
/// (0x003b1050): while the MPEG stream is not at its end and pictures keep
/// arriving, decode and deliver them to the caller's `+0x30` method.
pub fn ps2_sony_codec_internal_decode_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A1][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0;
        loop {
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                let target = c.g[V1][0] as u32;
                call_ptr(c, target, 0x003b_108c)?;
            }
            c.cycles += 4;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
            call_ptr(c, 0x0040_2b38, 0x003b_1094)?; // sceMpegIsEnd
            let ended = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if ended {
                c.cycles += 1;
                c.lq(S0, sp(c).wrapping_add(0x20));
                c.cycles += 4;
                c.lq(S1, sp(c).wrapping_add(0x10));
                c.g[RA][0] = c.ld(sp(c));
                c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
                abi::ret(c);
                return Ok(());
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003b_0fb8, 0x003b_10a4)?; // PS2_SONY_CODEC_INTERNAL_GetPicture
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = (c.g[V1][0] < c.g[S1][0]) as u64;
            c.g[A1][0] = c.g[V0][0];
            if c.g[V1][0] == 0 {
                c.cycles += 2;
                c.lq(S0, sp(c).wrapping_add(0x20));
                c.cycles += 4;
                c.lq(S1, sp(c).wrapping_add(0x10));
                c.g[RA][0] = c.ld(sp(c));
                c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
                abi::ret(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_1070);
            }
        }
    })
}

/// `bool WScriptMan_TableHasFunction(WScriptMan* m, int table, UStr* name)`
/// (0x00309dd0): a script table slot with a function, whose registered name
/// is not the "special" marker.
pub fn w_script_man_table_has_function(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3cc));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.sd(sp(c).wrapping_add(0x10), c.g[RA][0]);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        let have = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A2][0];
        if !have {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 15;
            c.g[V0][0] = 3;
            c.g[A0][0] = c.g[SP][0];
            c.sw(sp(c).wrapping_add(8), c.g[V0][0]);
            call_ptr(c, 0x0022_6620, 0x0030_9e10)?; // LunoVMRegister_AsId
            c.sw(c.g[V0][0] as u32, c.g[S1][0]);
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            call_ptr(c, 0x0022_5248, 0x0030_9e20)?; // cLunoTable_Find
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(8));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 2;
            c.g[S0][0] = c.g[S0][0] ^ 5;
            c.g[S0][0] = (c.g[S0][0] < 1) as u64;
            call_ptr(c, 0x0022_5b90, 0x0030_9e38)?; // LunoVMRegister_dtor
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x10));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void ModObj_AddMagnetModifier(ModObj* o, float* v, void* target)`
/// (0x003559f8): allocate a 0xb0-byte magnet modifier, construct it with the
/// two float parameters and install it as the object's transform modifier.
pub fn mod_obj_add_magnet_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.g[A3][0] = 0;
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x2000_0000;
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A1][0];
        c.swc1(21, sp(c).wrapping_add(0x38));
        c.g[S0][0] = c.g[A0][0];
        c.swc1(20, sp(c).wrapping_add(0x30));
        c.g[A1][0] = sx(0x0048_e918);
        c.sd(sp(c), c.g[RA][0]);
        c.mov_s(20, 12);
        c.mov_s(21, 13);
        c.g[A0][0] = 0xb0;
        call_ptr(c, 0x0031_7d70, 0x0035_5a38)?; // EA_MemAllocNamed
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0035_72e0, 0x0035_5a50)?; // MagnetModifier_ctor
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0035_54b0, 0x0035_5a5c)?; // ModObj_SetTransformModifier
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.lwc1(21, sp(c).wrapping_add(0x38));
        c.lwc1(20, sp(c).wrapping_add(0x30));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void PathData_Parse(PathData* p)` (0x0026adf0): read the path header and
/// each of the four sections in turn, advancing the cursor by each section's
/// returned size.
pub fn path_data_parse(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.sd(sp(c).wrapping_add(0x10), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.g[A0][0] = c.g[SP][0];
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
        c.sw((c.g[S1][0] as u32).wrapping_add(8), 0);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), 0);
        c.g[A1][0] = c.g[S0][0];
        c.sw(c.g[S1][0] as u32, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        call_ptr(c, 0x0026_ba68, 0x0026_ae24)?; // PathData_ReadInt
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0026_b410, 0x0026_ae30)?; // PathData_ReadAIPaths
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0026_b310, 0x0026_ae40)?; // PathData_ReadTrackPaths
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0026_b7d8, 0x0026_ae50)?; // PathData_ReadFloatIntPairs
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0026_b880, 0x0026_ae5c)?; // PathData_ReadMarkers
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x10));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_EnterGameInit(SSXAudioSystem* a)` (0x00286e20): enter
/// game mode (6), reset the world triggers, restart the board loops and store
/// the game module's id; in mode 1 say the "20BB" line.
pub fn ssx_audio_system_enter_game_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[A1][0] = 6;
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0028_5be0, 0x0028_6e38)?; // SSXAudioSystem_SetMode
        c.g[A0][0] = c.lw(gpa(c, 0x21e4));
        call_ptr(c, 0x002b_6900, 0x0028_6e40)?; // WorldTriggerManager_ResetPlayedList
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0029_29d8, 0x0028_6e48)?; // SSXAudioSystem_RestartAllBoardLoops
        call_ptr(c, 0x0028_b1c8, 0x0028_6e50)?; // SSXAudioSystem_GetGameModule
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x582c), c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x0028_6e60)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0028_6e6c)?; // BELibrary_GetInterface
        c.g[V1][0] = sx(0x0053_5c11);
        c.g[V0][0] = 1;
        c.g[V1][0] = c.lb(c.g[V1][0] as u32);
        let one = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 2;
        let run = if one {
            true
        } else {
            c.cycles += 2;
            c.g[V1][0] == c.g[V0][0]
        };
        if run {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_39e0, 0x0028_6e90)?; // SSXAudioSystem_PASay20BB
        }
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_OnActivate(cFEStateCharEquip* s, void* arg)`
/// (0x001993a0): show buy mode when the equip flag is set (else equip mode),
/// then refresh the stat bars and the item list and activate the base state.
pub fn c_fe_state_char_equip_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
        c.g[V0][0] = c.g[V0][0] ^ 1;
        let equip = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if equip {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0019_a238, 0x0019_93dc)?; // cFEStateCharEquip_ShowEquipMode
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0019_a308, 0x0019_93cc)?; // cFEStateCharEquip_ShowBuyMode
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0019_bc90, 0x0019_93e4)?; // cFEStateCharEquip_UpdateStatBar
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0019_bd48, 0x0019_93f0)?; // cFEStateCharEquip_UpdateStatPreviewBar
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 1;
        c.g[A1][0] = 1;
        call_ptr(c, 0x0019_b618, 0x0019_9400)?; // cFEStateCharEquip_RefreshItemList
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0018_6518, 0x0019_940c)?; // cFEState_OnActivate
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void BoltFileIndexManager_Reset(BoltFileIndexManager* m)` (0x0019cb60):
/// clear the index and the tail table, point every one of the 400 entries at
/// the list head and mark the table's end with -1.
pub fn bolt_file_index_manager_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[A1][0] = sx(0xffff_ffff);
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[A2][0] = 0x1900;
        call_ptr(c, 0x003e_6448, 0x0019_cb80)?; // EA_MemSet
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1900));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x640;
        call_ptr(c, 0x003e_6448, 0x0019_cb90)?; // EA_MemSet
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f40), c.g[S0][0]);
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        loop {
            c.cycles += 10;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.sh((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[A1][0] = c.g[V1][0];
            c.sh((c.g[A0][0] as u32).wrapping_add(0xe), c.g[V1][0]);
            c.g[V0][0] = ((c.g[A1][0] as i64) < 400) as u64;
            c.sh((c.g[A0][0] as u32).wrapping_add(4), 0);
            c.sh((c.g[A0][0] as u32).wrapping_add(6), 0);
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            if !more {
                c.cycles += 6;
                c.g[V0][0] = sx(0xffff_ffff);
                c.g[RA][0] = c.ld(sp(c));
                c.sh((c.g[S0][0] as u32).wrapping_add(0x18fe), c.g[V0][0]);
                c.lq(S0, sp(c).wrapping_add(0x10));
                c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
                abi::ret(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_cba0);
            }
        }
    })
}

/// `void GameState_PlayScriptEvent(GameState* g, UStr* name)`
/// (0x00234be8): resolve the current track's world section and call the
/// script function it names, when the table has it.
pub fn game_state_play_script_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0014_dc80, 0x0023_4c00)?; // BELibrary_GetInstance
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0023_4c0c)?; // BELibrary_GetInterface
        c.g[V1][0] = c.lw(gpn(c, 0x848));
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        call_ptr(c, 0x0014_4bc0, 0x0023_4c20)?; // BENewRaceInt_GetTrackInfo
        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0022_e098, 0x0023_4c2c)?; // SectionMan_GetEventWorldSection
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.lw(gpa(c, 0xce8));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0030_9dd0, 0x0023_4c40)?; // WScriptMan_TableHasFunction
        let missing = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw(gpa(c, 0xce8));
        if !missing {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0030_9e50, 0x0023_4c54)?; // WScriptMan_CallTableFunction
        }
        c.cycles += 5;
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_StartPlayback(VidEnginePlayer* p)` (0x002536c8):
/// restart the AV player on the player's current frame, pause it, create the
/// texture sink for the decoded frames and resume.
pub fn vid_engine_player_start_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[A1][0] = 1;
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x21;
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sw(c.g[S0][0] as u32, 0);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        c.sw((c.g[S0][0] as u32).wrapping_add(4), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0);
        call_ptr(c, 0x003a_e450, 0x0025_36f4)?; // AV_Player_Start
        c.g[V1][0] = 1;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x003a_e938, 0x0025_3708)?; // AV_Player_Pause
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A2][0] = c.lw(gpa(c, 0x2a90));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
        call_ptr(c, 0x0025_4368, 0x0025_3720)?; // VD_CreateTextureSink
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
        call_ptr(c, 0x003a_e958, 0x0025_372c)?; // AV_Player_Resume
        c.g[V1][0] = c.lw(gpn(c, 0x254));
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[RA][0] = c.ld(sp(c));
        c.sw(gpn(c, 0x254), c.g[V1][0]);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};
    use ssx3_ee as ee;

    /// The zeroed BELibrary dummy.
    const BE: u32 = DATA + 0x3_0000;
    /// The dummy world (`world -> +0x84 -> game -> +0xc -> race`).
    const WORLD: u32 = DATA + 0x8_0000;
    /// A real function that returns 0 when its `+4` word is 0
    /// (`LocStringTable_ResolveString` over a zeroed object): used as a vtable
    /// slot when a zero result drives a branch.
    const RET0: u32 = 0x002c_6dc0;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    fn wh(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }

    fn wb(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    /// A vtable slot `{delta = 0, fn}`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, func: u32) {
        wh(c, vt + off, 0);
        w(c, vt + off + 4, func);
    }

    /// The BELibrary singleton, the world/race globals and the `gp` slots the
    /// front-end code keeps singletons in.
    fn globals(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        let game = WORLD + 0x1000;
        w(c, WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    /// The engine's size-class pool allocator (`gp + 0xdd8` ready plus eight
    /// free nodes per size class; a zeroed heap walks garbage).
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
        for (i, (slot_off, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let region = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, region + head, base);
            w(c, GAME_GP + slot_off, region);
        }
    }

    /// Scrambled state, the fake call hooks, a game-like `gp`, the standard
    /// fixtures and a prepared UIEngine phase table (slot 0/1/2/3 -> fake
    /// objects whose `+4` is a vtable that calls the fake).
    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        globals(c);
        pool(c);
        for i in 0..4u32 {
            let obj = DATA + 0x10_000 + 0x40 * i;
            let vt = DATA + 0x11_000 + 0x40 * i;
            w(c, 0x0053_08a8 + 4 * i, obj);
            w(c, obj + 4, vt);
            slot(c, vt, 0x18, FAKE);
            slot(c, vt, 0x20, FAKE);
            slot(c, vt, 0x08, FAKE);
        }
        r
    }

    #[test]
    fn c_fe_state_title_init_matches() {
        check(
            ee::s02::f_00194778__generated,
            c_fe_state_title_init,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1947, 3);
                w(
                    c,
                    DATA + 0x10,
                    if seed % 3 == 0 { 0 } else { DATA + 0x1000 },
                );
                fill(c, &mut r, DATA + 0x40, 4);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn fe_locale_v05_matches() {
        check(ee::s02::f_00195f70__generated, fe_locale_v05, 80, &|c, seed| {
            let mut r = base(c, seed, 0x195f, 3);
            // `locale+8` is a table of string ids; each id's `+4` is the
            // LocString body. With a zeroed body the resolve returns 0, so the
            // walk runs its full 20 rounds; one seed makes the first entry
            // resolve (a count word above the table pointer) for the early exit.
            let found = seed % 3 == 1;
            w(c, DATA + 0x804, DATA + 0x900);
            w(c, DATA + 0x900 + 0xc, if found { 0xffff_ffff } else { 0 });
            for i in 0..20u32 {
                w(c, DATA + 8 + 4 * i, if i == 0 { DATA + 0x800 } else { 0 });
            }
            let _ = &mut r;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x200);
        });
    }

    #[test]
    fn world_state_notify_objects_restored_matches() {
        check(
            ee::s07::f_0026de58__generated,
            world_state_notify_objects_restored,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x26de, 1);
                let list = GAME_GP.wrapping_add(0x2898);
                let arg = DATA + 0x300;
                let proc = DATA + 0x400;
                let proc2 = DATA + 0x600;
                // `ProcessList_First(list, arg)`: `A1 = arg + lw(list+4)*0x44`,
                // `V0 = lw(A1+8)`; it is 0 when that equals `A1+0x14`.
                let n = (seed % 3) as u32; // 0: empty list, 1: one process
                w(c, list + 4, n);
                if n == 0 {
                    w(c, arg + 8, arg.wrapping_add(0x14));
                } else {
                    w(c, arg + 0x44 + 8, proc);
                    w(c, proc + 0xc, DATA + 0x500);
                    w(c, DATA + 0x500 + 0x18, 0);
                    w(c, DATA + 0x500 + 0x1c, FAKE);
                    // `ProcessList_Next` returns `lw(proc+4)` unless it equals
                    // `arg + n*0x44 + 0x14`; 0 ends the walk.
                    w(c, proc + 4, if seed % 2 == 0 { 0 } else { proc2 });
                    w(c, proc2 + 0xc, DATA + 0x520);
                    w(c, DATA + 0x520 + 0x18, 0);
                    w(c, DATA + 0x520 + 0x1c, FAKE);
                }
                w(c, arg, 1);
                c.g[A1][0] = sx(arg);
            },
        );
    }

    #[test]
    fn c_fe_state_uber_trick_on_activate_matches() {
        check(
            ee::s02::f_00184be0__generated,
            c_fe_state_uber_trick_on_activate,
            60,
            &|c, seed| {
                let _ = base(c, seed, 0x184b, 3);
                let st = DATA;
                let panel_a = if seed % 3 == 0 { 0 } else { DATA + 0x600 };
                let panel_b = if seed % 4 == 0 { 0 } else { DATA + 0x700 };
                w(c, st + 0x4c, panel_a);
                w(c, st + 0x50, panel_b);
                for (p, vt) in [(panel_a, DATA + 0x680), (panel_b, DATA + 0x780)] {
                    if p != 0 {
                        w(c, p + 8, vt);
                        slot(c, vt, 0x38, FAKE);
                    }
                }
                c.g[A0][0] = sx(st);
                c.g[A1][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn script_sound_bank_man_remove_all_matches() {
        check(
            ee::s07::f_00283658__generated,
            script_sound_bank_man_remove_all,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x2836, 3);
                let m = DATA;
                let n = (seed % 4) as u32;
                w(c, m + 4, n);
                w(c, m + 8, DATA + 0x1000);
                w(c, m + 0xc, DATA + 0x1800);
                slot(c, DATA + 0x1800, 0x10, FAKE);
                for i in 0..4u32 {
                    w(
                        c,
                        DATA + 0x1000 + 0xc * i + 8,
                        if (seed >> i) & 1 == 1 { 1 } else { 0 },
                    );
                }
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn typed_process_ctor_2_matches() {
        check(ee::s11::f_00354788__generated, typed_process_ctor_2, 80, &|c, seed| {
            let _ = base(c, seed, 0x3547, 2);
            let proc = DATA;
            let ti = DATA + 0x600;
            w(c, ti, DATA + 0x700);
            slot(c, DATA + 0x700, 0x10, FAKE);
            wh(c, proc + 0x10, (seed % 8) as u16);
            c.g[A0][0] = sx(proc);
            c.g[A1][0] = seed as u64;
            c.g[A2][0] = sx(ti);
        });
    }

    #[test]
    fn be_library_set_phase_matches() {
        check(ee::s01::f_0014de68__generated, be_library_set_phase, 80, &|c, seed| {
            let _ = base(c, seed, 0x14de, 2);
            let old = (seed % 4) as u32;
            let new = if seed % 2 == 0 { old } else { (seed / 4) % 4 };
            w(c, GAME_GP.wrapping_sub(0x1efc), old);
            c.g[A0][0] = new as u64;
        });
    }

    #[test]
    fn script_anim_bank_man_remove_all_matches() {
        check(
            ee::s07::f_00275698__generated,
            script_anim_bank_man_remove_all,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x2756, 3);
                let m = DATA;
                let n = (seed % 4) as u32;
                w(c, m + 4, n);
                w(c, m + 8, DATA + 0x1000);
                w(c, m + 0xc, DATA + 0x1800);
                slot(c, DATA + 0x1800, 0x10, FAKE);
                for i in 0..4u32 {
                    w(
                        c,
                        DATA + 0x1000 + 0x14 * i + 8,
                        if (seed >> i) & 1 == 1 { 1 } else { 0 },
                    );
                }
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn camera_base_init_matches() {
        check(ee::s01::f_0015d700__generated, camera_base_init, 80, &|c, seed| {
            let mut r = base(c, seed, 0x15d7, 1);
            fill(c, &mut r, DATA, 0x90);
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            c.f[14] = r.edge_f32();
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn fe_screen_autosave_popup_update_matches() {
        check(
            ee::s02::f_00187cb8__generated,
            fe_screen_autosave_popup_update,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x187c, 3);
                let p = DATA;
                w(c, p + 0x48, if seed % 4 == 0 { 0xef } else { r.next_u32() });
                w(c, p + 0x40, if seed % 3 == 0 { 0 } else { DATA + 0x600 });
                w(c, p + 0x10, DATA + 0x700);
                fill(c, &mut r, p + 0x20, 8);
                c.g[A0][0] = sx(p);
            },
        );
    }

    #[test]
    fn net_fe_state_toggle_block_user_matches() {
        check(
            ee::s03::f_001abc50__generated,
            net_fe_state_toggle_block_user,
            60,
            &|c, seed| {
                let _ = base(c, seed, 0x1abc, 4);
                let s = DATA;
                let buddy = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                w(c, GAME_GP.wrapping_add(0x238), buddy);
                w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x500);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(DATA + 0x600);
                c.g[A2][0] = sx(DATA + 0x700);
            },
        );
    }

    #[test]
    fn net_fe_state_toggle_messenger_buddy_matches() {
        check(
            ee::s03::f_001abe58__generated,
            net_fe_state_toggle_messenger_buddy,
            60,
            &|c, seed| {
                let _ = base(c, seed, 0x1abe, 4);
                let s = DATA;
                let buddy = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                w(c, GAME_GP.wrapping_add(0x238), buddy);
                w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x500);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(DATA + 0x600);
                c.g[A2][0] = sx(DATA + 0x700);
            },
        );
    }

    #[test]
    fn c_fe_state_chat_room_v32_matches() {
        check(
            ee::s03::f_001b22a0__generated,
            c_fe_state_chat_room_v32,
            60,
            &|c, seed| {
                let _ = base(c, seed, 0x1b22, 4);
                let s = DATA;
                let buddy = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                w(c, GAME_GP.wrapping_add(0x238), buddy);
                w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x500);
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn c_fe_state_chat_room_on_child_event_matches() {
        check(
            ee::s03::f_001b3850__generated,
            c_fe_state_chat_room_on_child_event,
            100,
            &|c, seed| {
                let _ = base(c, seed, 0x1b38, 2);
                let id = match seed % 5 {
                    0 => 0x800,
                    1 => 0x801,
                    2 => 0x80b,
                    3 => 0x7ff,
                    _ => 0x810,
                };
                w(c, DATA + 0x200 + 0x18, id);
                // The handlers see the caller's `a0` as their state.
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn net_config_auth_popup_dtor_matches() {
        check(
            ee::s03::f_001b6368__generated,
            net_config_auth_popup_dtor,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1b63, 3);
                let p = DATA;
                let auth = if seed % 3 == 0 { 0 } else { DATA + 0x800 };
                w(c, p + 0x6d0, auth);
                w(c, p + 0x6fc, if seed % 2 == 0 { 0 } else { DATA + 0x900 });
                c.g[A0][0] = sx(p);
                c.g[A1][0] = sx(DATA + 0x400);
            },
        );
    }

    #[test]
    fn c_fe_state_create_duplicate_init_matches() {
        check(
            ee::s03::f_001bbd60__generated,
            c_fe_state_create_duplicate_init,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1bbd, 3);
                let s = DATA;
                w(c, s + 0x10, if seed % 4 == 0 { 0 } else { DATA + 0x1000 });
                w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x1200);
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn message_center_is_new_player_flag_clear_matches() {
        check(
            ee::s04::f_001e3570__generated,
            message_center_is_new_player_flag_clear,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x1e35, 3);
                // The interface singleton the front-end reads at 0x535bc8.
                let flag = 0x0053_5bc8;
                wb(c, flag + 0x49, (seed % 3 == 0) as u8);
                wb(
                    c,
                    flag + 0x4a,
                    match seed % 4 {
                        0 => 6,
                        1 => 9,
                        _ => (seed % 0x20) as u8,
                    },
                );
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn ov_screen_message_center_open_view_popup_matches() {
        check(
            ee::s04::f_001e50c8__generated,
            ov_screen_message_center_open_view_popup,
            50,
            &|c, seed| {
                let mut r = base(c, seed, 0x1e50, 3);
                let s = DATA;
                w(c, s + 0x10, DATA + 0x200);
                if seed % 4 == 0 {
                    w(c, s + 0x17c, 0);
                } else {
                    w(c, s + 0x17c, DATA + 0x500);
                    w(c, DATA + 0x5d0, DATA + 0x520);
                    w(c, DATA + 0x530, DATA + 0x540);
                }
                fill(c, &mut r, DATA + 0x500, 3);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(DATA + 0x300);
                c.g[A2][0] = sx(DATA + 0x400);
                c.g[A3][0] = sx(DATA + 0x600);
            },
        );
    }

    #[test]
    fn c_ov_state_view_popup_set_body_matches() {
        check(
            ee::s04::f_001e5780__generated,
            c_ov_state_view_popup_set_body,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x1e57, 2);
                let s = DATA;
                let d = DATA + 0x200;
                // `d + 0x4c6c08` is the view popup's data record; +8 is the body
                // index (0/1 select it, >= 2 leaves the view, negative skips).
                let at = d.wrapping_add(0x4c_6c08);
                w(
                    c,
                    at + 8,
                    match seed % 4 {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        _ => 0xffff_ffff,
                    },
                );
                w(c, at + 0x14, seed);
                w(c, s + 0x58, DATA + 0x600);
                w(c, DATA + 0x608, DATA + 0x680);
                slot(c, DATA + 0x680, 0x48, FAKE);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(d);
            },
        );
    }

    #[test]
    fn c_shared_login_man_mc_open_matches() {
        check(
            ee::s07::f_002660c8__generated,
            c_shared_login_man_mc_open,
            50,
            &|c, seed| {
                let _ = base(c, seed, 0x2660, 3);
                w(
                    c,
                    GAME_GP.wrapping_add(0x2e8),
                    if seed % 3 == 0 { 0 } else { DATA + 0x400 },
                );
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                c.g[A2][0] = sx(DATA + 0x300);
                c.g[A3][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn path_plat_remove_bank_matches() {
        check(
            ee::s08::f_002ae328__generated,
            path_plat_remove_bank,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x2ae3, 2);
                // 24 bank slots: +8 the platform id, +4 the SND bank handle.
                for i in 0..24u32 {
                    let e = 0x004d_3ef8 + 0x28 * i;
                    w(c, e + 8, i % 4);
                    w(
                        c,
                        e + 4,
                        if (seed / 4 + i) % 2 == 0 {
                            0
                        } else {
                            0xffff_ffff
                        },
                    );
                    w(c, e + 0x24, 0x1111_1111);
                }
                c.g[A0][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn path_plat_get_status_matches() {
        check(ee::s08::f_002af2f0__generated, path_plat_get_status, 50, &|c, seed| {
            let _ = base(c, seed, 0x2af2, 2);
            for i in 0..4u32 {
                let e = 0x004d_3e98 + 4 * i;
                if (seed / 4 + i) % 3 == 0 {
                    w(c, e, 0);
                } else {
                    let bank = DATA + 0x400 + 0x80 * i;
                    w(c, e, bank);
                    wb(c, bank + 0x65, ((seed + i) % 3 == 0) as u8);
                    w(c, bank + 0x48, 0x10 + i);
                    w(c, bank + 0x4c, 0x20 + i);
                    w(c, bank + 0x5c, if seed % 2 == 0 { 0 } else { 0x30 + i });
                    w(c, bank + 0x60, if seed % 3 == 0 { 0x40 + i } else { 0 });
                }
            }
            c.g[A0][0] = (seed % 4) as u64;
            c.g[A1][0] = sx(DATA + 0x200);
            c.g[A2][0] = sx(DATA + 0x300);
        });
    }

    #[test]
    fn music_player_set_playlist_mask_matches() {
        check(
            ee::s08::f_002b4070__generated,
            music_player_set_playlist_mask,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x2b40, 3);
                let m = DATA;
                fill(c, &mut r, m + 0x3e0, 3);
                fill(c, &mut r, m + 0x920, 8);
                w(c, m + 0x3ec, (seed % 5) as u32);
                c.g[A0][0] = sx(m);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn music_player_load_song_data_matches() {
        check(
            ee::s08::f_002b4a20__generated,
            music_player_load_song_data,
            50,
            &|c, seed| {
                let _ = base(c, seed, 0x2b4a, 3);
                let m = DATA;
                w(c, m + 0x5438, if seed % 3 == 0 { 0 } else { DATA + 0x800 });
                w(c, m + 0x5434, if seed % 4 == 0 { 0 } else { DATA + 0x900 });
                c.g[A0][0] = sx(m);
                c.g[A1][0] = sx(DATA + 0x200);
                // `a2 == 0`: the async loader. The EAFile path (a2 != 0) queues a
                // disc read and runs away on a test machine (0x3de768).
                c.g[A2][0] = 0;
            },
        );
    }

    #[test]
    fn menu_sys_int_item_ctor_3_matches() {
        check(
            ee::s09::f_002cd8f8__generated,
            menu_sys_int_item_ctor_3,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x2cd8, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = (seed as u64) << 1;
                c.g[A3][0] = (seed as u64) << 2;
                c.g[T0][0] = (seed as u64) << 3;
                c.g[T1][0] = (seed as u64) << 4;
            },
        );
    }

    #[test]
    fn menu_sys_color_swatch_item_measure_matches() {
        check(
            ee::s09::f_002cff88__generated,
            menu_sys_color_swatch_item_measure,
            50,
            &|c, seed| {
                let mut r = base(c, seed, 0x2cff, 1);
                fill(c, &mut r, DATA, 0x40);
                w(c, DATA + 0x200 + 0x18, (seed % 4) as u32);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn ai_update_rider_poses_matches() {
        check(
            ee::s00::f_00129160__generated,
            ai_update_rider_poses,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1291, 3);
                let wld = DATA;
                let n = (seed % 3) as u32;
                w(c, wld + 0x78, n);
                for i in 0..3u32 {
                    w(
                        c,
                        wld + 0x28 + 4 * i,
                        if i < n { DATA + 0x1000 + 0x800 * i } else { 0 },
                    );
                }
                fill(c, &mut r, DATA + 0x1000, 0x40);
                fill(c, &mut r, DATA + 0x1800, 0x40);
                c.g[A0][0] = sx(wld);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_list_dir_matches() {
        check(
            ee::s08::f_002c4210__generated,
            c_ps2_mem_card_do_list_dir,
            40,
            &|c, seed| {
                let _ = base(c, seed, 0x2c42, 3);
                let m = DATA;
                w(c, m + 0xc, seed);
                w(c, m + 0x139, 0x4142);
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_list_dir_all_matches() {
        check(
            ee::s08::f_002c6848__generated,
            c_ps2_mem_card_do_list_dir_all,
            40,
            &|c, seed| {
                let _ = base(c, seed, 0x2c68, 3);
                let m = DATA;
                w(c, m + 0xc, seed);
                w(c, m + 0x139, 0x4142);
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn manual_cam_algo_base_ctor_matches() {
        check(
            ee::s01::f_0015f938__generated,
            manual_cam_algo_base_ctor,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x15f9, 1);
                fill(c, &mut r, DATA, 0x40);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_clear_color_matches() {
        check(
            ee::s12::f_00389098__generated,
            ps2_graphics_man_set_clear_color,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x3890, 2);
                for k in 0..3 {
                    let v = match seed % 4 {
                        0 => r.edge_f32(),
                        1 => ((seed % 2) as f32).to_bits(),
                        _ => r.f32(1.5),
                    };
                    w(c, DATA + 0x200 + 4 * k, v);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn irradiance_build_sh_matches() {
        check(ee::s12::f_00389c38__generated, irradiance_build_sh, 50, &|c, seed| {
            let mut r = base(c, seed, 0x389c, 3);
            fill(c, &mut r, DATA, 0x400);
            // The sky's ambient entry (a fixed global) exists for some seeds.
            w(c, 0x0050_47fc, if seed % 3 == 0 { 0 } else { 1 });
            w(c, GAME_GP.wrapping_add(0x12d4), DATA + 0x200);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x300) };
        });
    }

    #[test]
    fn mc_manager_state04_no_card_matches() {
        check(
            ee::s06::f_0023e820__generated,
            mc_manager_state04_no_card,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x23e8, 4);
                let m = DATA;
                w(c, m + 0x434, DATA + 0x400);
                // The card object's vtable: real "return 0" for the idle branch.
                w(c, DATA + 0x400, DATA + 0x500);
                slot(
                    c,
                    DATA + 0x500,
                    0x58,
                    if seed % 2 == 0 { RET0 } else { FAKE },
                );
                slot(c, DATA + 0x500, 0x18, FAKE);
                w(c, m + 0x6c, if seed % 2 == 0 { 0 } else { RET0 });
                w(c, m + 0x84, if seed % 3 == 0 { 0 } else { RET0 });
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn mc_manager_state05_matches() {
        check(ee::s06::f_0023eb80__generated, mc_manager_state05, 80, &|c, seed| {
            let _ = base(c, seed, 0x23eb, 4);
            let m = DATA;
            w(c, m + 0x434, DATA + 0x400);
            w(c, DATA + 0x400, DATA + 0x500);
            slot(
                c,
                DATA + 0x500,
                0x58,
                if seed % 2 == 0 { RET0 } else { FAKE },
            );
            slot(c, DATA + 0x500, 0x18, FAKE);
            w(c, m + 0x6c, if seed % 2 == 0 { 0 } else { RET0 });
            w(c, m + 0x84, if seed % 3 == 0 { 0 } else { RET0 });
            c.g[A0][0] = sx(m);
        });
    }

    #[test]
    fn mc_manager_state15_matches() {
        check(ee::s06::f_0023f1d8__generated, mc_manager_state15, 80, &|c, seed| {
            let _ = base(c, seed, 0x23f1, 4);
            let m = DATA;
            w(c, m + 0x434, DATA + 0x400);
            w(c, DATA + 0x400, DATA + 0x500);
            slot(
                c,
                DATA + 0x500,
                0x58,
                if seed % 2 == 0 { RET0 } else { FAKE },
            );
            slot(c, DATA + 0x500, 0x18, FAKE);
            w(c, m + 0x6c, if seed % 2 == 0 { 0 } else { RET0 });
            w(c, m + 0x84, if seed % 3 == 0 { 0 } else { RET0 });
            c.g[A0][0] = sx(m);
        });
    }

    #[test]
    fn mc_manager_state10_save_matches() {
        check(
            ee::s06::f_0023f4f8__generated,
            mc_manager_state10_save,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x23f4, 3);
                let m = DATA;
                w(c, m + 0xe4, (seed % 100) as u32);
                w(c, m + 0xf0, 1 + (seed % 7) as u32);
                w(c, m + 0x434, DATA + 0x400);
                if seed % 4 != 0 {
                    w(c, m + 0x748, DATA + 0x600);
                    w(c, DATA + 0x600 + 8, 0);
                    w(c, DATA + 0x600 + 0xc, RET0);
                }
                w(c, DATA + 0x400, DATA + 0x500);
                slot(
                    c,
                    DATA + 0x500,
                    0x248,
                    if seed % 2 == 0 { RET0 } else { FAKE },
                );
                let _ = &mut r;
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn mc_manager_state1f_load_matches() {
        check(
            ee::s06::f_002400b0__generated,
            mc_manager_state1f_load,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x2400, 3);
                let m = DATA;
                w(c, m + 0x434, DATA + 0x400);
                if seed % 4 != 0 {
                    w(c, m + 0x748, DATA + 0x600);
                    w(c, DATA + 0x600 + 8, 0);
                    w(c, DATA + 0x600 + 0xc, RET0);
                }
                w(c, m + 0x2c, RET0);
                w(c, DATA + 0x400, DATA + 0x500);
                slot(
                    c,
                    DATA + 0x500,
                    0x248,
                    if seed % 2 == 0 { RET0 } else { FAKE },
                );
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn mc_manager_begin_slot_select_matches() {
        check(
            ee::s06::f_00241380__generated,
            mc_manager_begin_slot_select,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x2413, 3);
                let m = DATA;
                w(c, m + 0x434, DATA + 0x400);
                w(c, DATA + 0x400, DATA + 0x500);
                slot(c, DATA + 0x500, 0x18, FAKE);
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn mc_manager_get_save_type_tag_matches() {
        check(
            ee::s06::f_00241d40__generated,
            mc_manager_get_save_type_tag,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x241d, 1);
                let ty: u32 = match seed % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    _ => u32::MAX,
                };
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = ty as u64;
                c.g[A2][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn c_keyboard_popup_type_current_key_matches() {
        check(
            ee::s04::f_001cc7c8__generated,
            c_keyboard_popup_type_current_key,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x1cc7, 2);
                let p = DATA;
                fill(c, &mut r, p + 0x60, 8);
                w(c, p + 0x68, if seed % 3 == 0 { 0 } else { 1 });
                w(c, p + 0x6c, if seed % 2 == 0 { 0 } else { 1 });
                w(
                    c,
                    p + 0x140,
                    match seed % 4 {
                        0 => 0xf,
                        1 => 0x12,
                        2 => 0x1a,
                        _ => 0x20,
                    },
                );
                w(c, p + 0x134, (seed % 8) as u32);
                w(c, p + 0x60, 0x10);
                c.g[A0][0] = sx(p);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn rcmp_decoder_decode_frame_matches() {
        check(
            ee::s13::f_003b0600__generated,
            rcmp_decoder_decode_frame,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x3b06, 2);
                let d = DATA;
                w(c, d + 0x14, if seed % 3 == 0 { 0 } else { DATA + 0x400 });
                w(c, d, (seed % 2) as u32);
                w(c, d + 4, seed);
                w(c, d + 8, if seed % 4 == 0 { 0 } else { RET0 });
                w(c, DATA + 0x400, DATA + 0x500);
                slot(c, DATA + 0x500, 0x18, FAKE);
                c.g[A0][0] = sx(d);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn ps2_sony_codec_internal_decode_frame_matches() {
        check(
            ee::s13::f_003b1050__generated,
            ps2_sony_codec_internal_decode_frame,
            40,
            &|c, seed| {
                let _ = base(c, seed, 0x3b10, 2);
                let codec = DATA;
                // `sceMpegIsEnd` reads `lw(lw(codec+0x40))`; GetPicture bumps
                // `codec+0x10`, so the loop ends after `limit` pictures.
                w(
                    c,
                    codec + 0x40,
                    if seed % 4 == 0 { 0 } else { DATA + 0x500 },
                );
                w(c, DATA + 0x500, if seed % 8 == 0 { 1 } else { 0 });
                w(c, codec + 0x10, 0);
                w(c, codec + 8, 0);
                w(c, codec + 0xc, 0);
                w(c, codec + 0x24, 0);
                c.g[A0][0] = sx(codec);
                // Keep the loop short: at most two decoded pictures.
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn w_script_man_table_has_function_matches() {
        check(
            ee::s10::f_00309dd0__generated,
            w_script_man_table_has_function,
            60,
            &|c, seed| {
                let m = DATA;
                let idx = (seed % 4) as u32;
                w(
                    c,
                    m + 0x3cc + 4 * idx,
                    if seed % 3 == 0 { 0 } else { DATA + 0x400 },
                );
                w(c, DATA + 0x400 + 8, 3);
                c.g[A0][0] = sx(m);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn mod_obj_add_magnet_modifier_matches() {
        check(
            ee::s11::f_003559f8__generated,
            mod_obj_add_magnet_modifier,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x3559, 3);
                let o = DATA;
                w(c, o + 0x18, seed);
                fill(c, &mut r, DATA + 0x200, 0x40);
                fill(c, &mut r, DATA + 0x300, 0x40);
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.g[A0][0] = sx(o);
                c.g[A1][0] = sx(DATA + 0x200);
                c.g[A2][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn path_data_parse_matches() {
        check(ee::s07::f_0026adf0__generated, path_data_parse, 60, &|c, seed| {
            let _ = base(c, seed, 0x26ad, 3);
            w(c, DATA + 0x48, DATA + 0x1000);
            // The section buffer is zeroed, so every reader stops at once
            // (a random count walks billions of entries). Half the seeds give
            // the AI-path section one entry to run its loop once.
            w(c, DATA + 0x1004, (seed % 2) as u32);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn ssx_audio_system_enter_game_init_matches() {
        check(
            ee::s07::f_00286e20__generated,
            ssx_audio_system_enter_game_init,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x286e, 3);
                w(
                    c,
                    GAME_GP.wrapping_add(0x21e4),
                    if seed % 3 == 0 { 0 } else { DATA + 0x400 },
                );
                wb(
                    c,
                    0x0053_5c11,
                    match seed % 4 {
                        0 => 1,
                        1 => 2,
                        _ => 0,
                    },
                );
                fill(c, &mut r, DATA, 0x40);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn c_fe_state_char_equip_on_activate_matches() {
        check(
            ee::s03::f_001993a0__generated,
            c_fe_state_char_equip_on_activate,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1993, 3);
                let s = DATA;
                w(c, s + 0xc8, (seed % 3) as u32);
                fill(c, &mut r, s + 0x20, 0x40);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn bolt_file_index_manager_reset_matches() {
        check(
            ee::s03::f_0019cb60__generated,
            bolt_file_index_manager_reset,
            60,
            &|c, seed| {
                let _ = base(c, seed, 0x19cb, 1);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn game_state_play_script_event_matches() {
        check(
            ee::s06::f_00234be8__generated,
            game_state_play_script_event,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x234b, 3);
                let m = DATA + 0x1000;
                let idx = (seed % 4) as u32;
                // The script table slot decides whether the event is called.
                w(
                    c,
                    m + 0x3cc + 4 * idx,
                    if seed % 2 == 0 { 0 } else { DATA + 0x2000 },
                );
                w(c, DATA + 0x2000 + 8, if seed % 3 == 0 { 5 } else { 0 });
                w(
                    c,
                    GAME_GP.wrapping_add(0xce8),
                    if seed % 2 == 0 { 0 } else { m },
                );
                fill(c, &mut r, DATA + 0x2000, 8);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn vid_engine_player_start_playback_matches() {
        check(
            ee::s06::f_002536c8__generated,
            vid_engine_player_start_playback,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x2536, 3);
                let p = DATA;
                w(c, p + 0x38, DATA + 0x400);
                w(c, p + 0x30, seed);
                fill(c, &mut r, DATA + 0x400, 0x40);
                c.g[A0][0] = sx(p);
            },
        );
    }
}
