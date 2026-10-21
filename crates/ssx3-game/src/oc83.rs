//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small sound, menu, string, world-modifier, graphics and
//! front-end helpers, each with a differential test against the generated
//! body.
//!
//! - **Sound** (`s08`): the INI comment skipper, SPU-block freeing, the
//!   bank-monitor pause loop, song pause/resume/event posting, the next-beat
//!   timer, the duplicate-trigger query and the five memory-card requests.
//! - **Menus** (`s09`): the bool item's draw, three menu-item constructors,
//!   the cut-scene "force on" transition and the irradiance-name fallback.
//! - **Strings / app** (`s10`): `cBXString` copy/assign/append and the app
//!   object's two virtual calls.
//! - **World** (`s11`): crowd-section removal, flex-bridge save, the dead
//!   fade, the Ava spline modifier and two transform-modifier constructors.
//! - **Graphics** (`s12`): anim-teeter teardown, the floating-element lookup,
//!   display-mode selection, texture loading, the model-matrix emitter, the
//!   VIF1 DMA end handler and the plant sway matrix.
//! - **UI** (`s13`): `UIList` construction, background-state insertion and the
//!   chunk-resolve destructor.
//! - **Riders, camera, front end** (`s00`-`s02`): mission/item callbacks, the
//!   board-press cancel, player defaults, camera controller destructor and
//!   default algorithm, the direction-chase-camera constructor, the debug
//!   track filter, the lobby occupant popup, the keyboard cursor and the
//!   character-select transition.
//!
//! The ports keep their state in the guest registers the originals use, follow
//! the call protocol of [`crate::call`] and the event rules of
//! [`crate::events`]; the two VU0 users issue the original's COP2 macro ops.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    // Sound.
    (0x002a_79c8, ini_parser_skip_comment, "IniParser_SkipComment"),
    (0x002a_92f8, ps2_sound_manager_free_spu_block, "PS2SoundManager_FreeSpuBlock"),
    (0x002a_d2a8, bank_monitor_pause_all, "BankMonitor_PauseAll"),
    (0x002b_2018, song_instance_pause, "SongInstance_Pause"),
    (0x002b_2070, song_instance_resume, "SongInstance_Resume"),
    (0x002b_20c8, song_instance_post_event, "SongInstance_PostEvent"),
    (0x002b_4920, music_player_get_ms_to_next_beat, "MusicPlayer_GetMsToNextBeat"),
    (0x002b_66e8, world_trigger_manager_is_duplicate_active, "WorldTriggerManager_IsDuplicateActive"),
    (0x002c_6a88, c_ps2_mem_card_do_seek, "cPS2MemCard_DoSeek"),
    (0x002c_6b78, c_ps2_mem_card_do_read, "cPS2MemCard_DoRead"),
    (0x002c_6bf8, c_ps2_mem_card_do_write, "cPS2MemCard_DoWrite"),
    (0x002c_6c88, c_ps2_mem_card_do_mkdir, "cPS2MemCard_DoMkdir"),
    (0x002c_6d18, c_ps2_mem_card_do_delete, "cPS2MemCard_DoDelete"),
    // Menus.
    (0x002c_c758, menu_sys_bool_item_draw, "MenuSysBoolItem_Draw"),
    (0x002c_ccc8, menu_sys_submenu_item_ctor, "MenuSysSubmenuItem_ctor"),
    (0x002c_ed20, menu_sys_color_channel_item_ctor, "MenuSysColorChannelItem_ctor"),
    (0x002c_ff28, menu_sys_color_swatch_item_ctor, "MenuSysColorSwatchItem_ctor"),
    (0x002e_a860, c_cut_scene_transition_man_force_on, "cCutSceneTransitionMan_ForceOn"),
    (0x002e_f198, world_painter_get_irradiance_name2, "WorldPainter_GetIrradianceName2"),
    // App and strings.
    (0x0031_7400, app_v02, "App_v02"),
    (0x0031_7fe8, c_bx_string_ctor_copy, "cBXString_ctor_copy"),
    (0x0031_84e8, c_bx_string_assign_cstr, "cBXString_assign_cstr"),
    (0x0031_8910, c_bx_string_append_cstr, "cBXString_AppendCStr"),
    // World and modifiers.
    (0x0034_3c08, crowd_man3d_remove_section, "CrowdMan3d_RemoveSection"),
    (0x0034_7d38, flex_bridge_save, "FlexBridge_Save"),
    (0x0035_0c38, dead_fade_update, "DeadFade_Update"),
    (0x0035_5a78, mod_obj_add_ava_spline_modifier, "ModObj_AddAvaSplineModifier"),
    (0x0035_6eb8, position_modifier_ctor, "PositionModifier_ctor"),
    (0x0035_7038, parent_modifier_ctor, "ParentModifier_ctor"),
    // Graphics.
    (0x0036_1400, anim_teeter_dtor, "AnimTeeter_dtor"),
    (0x0036_1800, floating_v29, "Floating_v29"),
    (0x0037_6468, ps2_graphics_man_select_matching_display_mode, "PS2GraphicsMan_SelectMatchingDisplayMode"),
    (0x0037_caf8, ps2_graphics_man_load_texture, "PS2GraphicsMan_LoadTexture"),
    (0x0037_e040, ps2_graphics_man_emit_model_matrix2, "PS2GraphicsMan_EmitModelMatrix2"),
    (0x0038_2688, ps2_graphics_man_on_vif1_dma_end, "PS2GraphicsMan_OnVif1DmaEnd"),
    (0x0038_f2a8, vfx_plants_get_sway_matrix, "VfxPlants_GetSwayMatrix"),
    // UI.
    (0x0039_77e8, ui_list_ctor, "UIList_ctor"),
    (0x0039_f238, ui_state_mgr_add_background_state, "UIStateMgr_AddBackgroundState"),
    (0x003a_ade8, c_ps2_chunk_resolve_dtor, "cPS2ChunkResolve_dtor"),
    // Riders, camera and front end.
    (0x0010_f2d8, rider_on_mission_completed, "Rider_OnMissionCompleted"),
    (0x0010_f338, rider_on_item_collected, "Rider_OnItemCollected"),
    (0x0013_1348, rider_board_press_sub_cancel, "RiderBoardPressSub_Cancel"),
    (0x0014_5bd8, be_new_player_int_restore_defaults, "BENewPlayerInt_RestoreDefaults"),
    (0x0015_cc10, cam_controller_dtor, "CamController_dtor"),
    (0x0016_1ef0, chase_cam_controller_set_default_algorithm, "ChaseCamController_SetDefaultAlgorithm"),
    (0x0017_74c0, direction_ne_chase_camera_algorithm_ctor, "DirectionNeChaseCameraAlgorithm_ctor"),
    (0x0017_9fc8, debug_menu_is_track_selectable, "DebugMenu_IsTrackSelectable"),
    (0x0017_e3b8, lobby_menu_show_occupant_detail, "LobbyMenu_ShowOccupantDetail"),
    (0x0018_0768, keyboard_screen_cursor_down, "KeyboardScreen_CursorDown"),
    (0x0018_12a8, c_fe_state_char_select_transition_in, "cFEStateCharSelect_TransitionIn"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const INI_PARSER_SKIP_TO_EOL: Callee =
        Callee { addr: 0x002a_7a90, f: ee::s08::IniParser_SkipToEol };
    pub const BANK_MONITOR_INSTANCE_PAUSE: Callee =
        Callee { addr: 0x002a_cac8, f: ee::s08::BankMonitorInstance_Pause };
    pub const SONG_INSTANCE_GET_STREAM_HANDLE: Callee =
        Callee { addr: 0x002b_2488, f: ee::s08::SongInstance_GetStreamHandle };
    pub const SONG_INSTANCE_GET_MS_TO_NEXT_BEAT: Callee =
        Callee { addr: 0x002b_2718, f: ee::s08::SongInstance_GetMsToNextBeat };
    pub const CPS2_MEM_CARD_SET_STATE: Callee =
        Callee { addr: 0x002c_48c0, f: ee::s08::cPS2MemCard_SetState };
    pub const MENU_SYS_ITEM_CTOR: Callee = Callee { addr: 0x002c_a258, f: ee::s09::MenuSysItem_ctor };
    pub const MENU_SYS_ITEM_DRAW_ROW: Callee =
        Callee { addr: 0x002c_a988, f: ee::s09::MenuSysItem_DrawRow };
    pub const CUT_SCENE_TRANSITION_SAVE_VIEWPORT: Callee =
        Callee { addr: 0x002e_aa28, f: ee::s09::cCutSceneTransitionMan_SaveViewport };
    pub const EA_MEM_ALLOC_NAMED: Callee = Callee { addr: 0x0031_7d70, f: ee::s10::EA_MemAllocNamed };
    pub const AVA_SPLINE_MODIFIER_CTOR: Callee =
        Callee { addr: 0x0035_7750, f: ee::s11::AvaSplineModifier_ctor };
    pub const POSITION_MODIFIER_CTOR: Callee =
        Callee { addr: 0x0035_6eb8, f: ee::s11::PositionModifier_ctor };
    pub const MOD_OBJ_SET_TRANSFORM_MODIFIER: Callee =
        Callee { addr: 0x0035_54b0, f: ee::s11::ModObj_SetTransformModifier };
    pub const ANIM_OBJECT_BASE_DTOR: Callee =
        Callee { addr: 0x0034_dba8, f: ee::s11::AnimObject_BaseDtor };
    pub const BUILTIN_DELETE: Callee = Callee { addr: 0x0031_7e50, f: ee::s10::__builtin_delete };
    pub const TEX_MAN_LOAD_TEXTURE: Callee =
        Callee { addr: 0x0036_7440, f: ee::s12::TexMan_LoadTexture };
    pub const UI_LIST_RESET: Callee = Callee { addr: 0x0039_7930, f: ee::s13::UIList_Reset };
    pub const UI_LIST_PUSH_BACK: Callee = Callee { addr: 0x0039_7910, f: ee::s13::UIList_PushBack };
    pub const CPS2_CHUNK_RESOLVE_DTOR_STUB: Callee =
        Callee { addr: 0x003a_ae60, f: ee::s13::cPS2ChunkResolve_DtorStub };
    pub const RIDER_METRIX_ON_MISSION_COMPLETED: Callee =
        Callee { addr: 0x0011_a110, f: ee::s00::RiderMetrix_OnMissionCompleted };
    pub const RIDER_METRIX_AWARD_POINTS: Callee =
        Callee { addr: 0x0011_9ef8, f: ee::s00::RiderMetrix_AwardPoints };
    pub const RIDER_BOARD_PRESS_SUB_SETTLE_SLEW: Callee =
        Callee { addr: 0x0013_13a8, f: ee::s00::RiderBoardPressSub_SettleSlew };
    pub const RIDER_BOARD_PRESS_SUB_FINISH_PRESS: Callee =
        Callee { addr: 0x0013_1428, f: ee::s00::RiderBoardPressSub_FinishPress };
    pub const RIDER_ADD_BOOST: Callee = Callee { addr: 0x0010_e098, f: ee::s00::Rider_AddBoost };
    pub const RIDER_METRIX_FINISH_BOARD_PRESS: Callee =
        Callee { addr: 0x0011_9a38, f: ee::s00::RiderMetrix_FinishBoardPress };
    pub const ALGORITHM_LIST_DTOR: Callee =
        Callee { addr: 0x0015_c940, f: ee::s01::AlgorithmList_dtor };
    pub const CHASE_CAM_SET_ALGORITHM_IMMEDIATE: Callee =
        Callee { addr: 0x0015_d050, f: ee::s01::ChaseCamController_SetAlgorithmImmediate };
    pub const CHASE_CAMERA_ALGORITHM_CTOR: Callee =
        Callee { addr: 0x0016_2318, f: ee::s01::ChaseCameraAlgorithm_ctor };
    pub const CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS: Callee =
        Callee { addr: 0x0018_1bd0, f: ee::s02::cFEStateCharSelect_UpdateAttributeBars };
    pub const BXSTRING_ASSIGN_CSTR: Callee =
        Callee { addr: 0x0031_84e8, f: ee::s10::cBXString_assign_cstr };
    pub const BXSTRING_ASSIGN_COPY: Callee =
        Callee { addr: 0x0031_83d8, f: ee::s10::cBXString_AssignCopy };
    pub const BXSTRING_CONCAT_IN_PLACE: Callee =
        Callee { addr: 0x0031_8830, f: ee::s10::cBXString_ConcatInPlace };
    pub const CROWD_MAN3D_GROUP_REMOVE_SECTION: Callee =
        Callee { addr: 0x0034_4898, f: ee::s11::CrowdMan3d_GroupRemoveSection };
    pub const WORLD_OBJ_SAVE: Callee = Callee { addr: 0x0034_fe90, f: ee::s11::WorldObj_Save };
    pub const FLEX_BRIDGE_SPRINGS_SAVE: Callee =
        Callee { addr: 0x0035_45d8, f: ee::s11::FlexBridge_SpringsSave };

    /// Library entry points reached through the function table.
    pub const BELIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BELIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const BEREWARD_ON_MISSION_COMPLETED: u32 = 0x0015_9b08;
    pub const BEREWARD_ON_COLLECTIBLE_COLLECTED: u32 = 0x0015_99a0;
    pub const BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT: u32 = 0x0014_7488;
    pub const BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX: u32 = 0x0014_4cc0;
    pub const SND_LOCK: u32 = 0x003b_58a0;
    pub const SND_UNLOCK: u32 = 0x003b_58d8;
    pub const SND_SPU_MEM_SET_BASE: u32 = 0x003b_5b20;
    pub const LOBBY_OCCUPANT_POP: u32 = 0x002c_c460;
    pub const LOBBY_OCCUPANT_SELECT: u32 = 0x002c_c3b8;
    pub const SNDSTRM_SET_PITCH: u32 = 0x003b_7fb8;
    pub const PATH_SET_PAUSED: u32 = 0x003d_1038;
    pub const PATH_POST_EVENT: u32 = 0x003d_16f0;
    pub const SCE_MC_SEEK: u32 = 0x0040_9f28;
    pub const SCE_MC_READ: u32 = 0x0040_a090;
    pub const SCE_MC_WRITE: u32 = 0x0040_a1a8;
    pub const SCE_MC_MKDIR: u32 = 0x0040_9e38;
    pub const SCE_MC_DELETE: u32 = 0x0040_aa68;
    pub const ISIGNAL_SEMA: u32 = 0x0042_3dd0;
    pub const STRLEN: u32 = 0x0041_6810;
    pub const WORLD_CREATE_DEAD_NODE: u32 = 0x002d_1cc0;
}

// ---------------------------------------------------------------------------
// Small register helpers.

/// A 32-bit address sum.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `addu reg, delta`: a vtable's adjusted `this`.
#[inline(always)]
fn this_with_delta(obj: u64, delta: u64) -> u64 {
    abi::addu(obj, delta)
}

// ---------------------------------------------------------------------------
// Sound (s08).

/// `int IniParser_SkipComment(IniParser* self)` (0x2a79c8): 1 at end of input,
/// otherwise skip a `#`/`;` comment line and report whether text followed.
pub fn ini_parser_skip_comment(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0)).off(0x8).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).lb(c);
        let at_end = c.g[V1][0] == 0;
        c.g[V0][0] = b'#' as u64;
        if at_end {
            c.cycles += 1;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            let hash = c.g[V1][0] == c.g[V0][0];
            if !hash {
                c.cycles += 3;
                c.g[V0][0] = b';' as u64;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0;
                    c.cycles += 3;
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
            c.cycles += 5;
            callee::INI_PARSER_SKIP_TO_EOL.call(c, 0x002a_79fc)?;
            c.g[V1][0] = c.g[V0][0];
            let skipped = c.g[V1][0] != 0;
            c.g[V0][0] = 1;
            if !skipped {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2SoundManager_FreeSpuBlock(PS2SoundManager* self, SpuBlock* e)`
/// (0x2a92f8): push a block back onto the free list and, before the SPU base
/// has been set, hand the block's voice id to the driver.
pub fn ps2_sound_manager_free_spu_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = u64::MAX;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A1)).off(0x8).lw(c);
        Addr(r32(c, A0)).off(0x30c).sw(c, c.g[V0][0]);
        Addr(r32(c, A1)).off(0x8).sw(c, c.g[V1][0]);
        Addr(r32(c, A1)).off(0x4).sw(c, c.g[V1][0]);
        Addr(r32(c, A1)).off(0xc).sw(c, 0);
        Addr(r32(c, A1)).off(0x10).sw(c, 0);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = Addr(r32(c, A0)).off(0x310).lw(c);
        Addr(r32(c, A1)).off(0x14).sw(c, c.g[V0][0]);
        Addr(r32(c, A0)).off(0x310).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(r32(c, A0)).off(0x31c).lw(c);
        let spu_base_set = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !spu_base_set {
            c.cycles += 4;
            c.g[A1][0] = Addr(r32(c, A0)).off(0x30c).lw(c);
            c.g[A0][0] = u64::MAX;
            call_ptr(c, callee::SND_SPU_MEM_SET_BASE, 0x002a_9344)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_PauseAll(BankMonitor* self)` (0x2ad2a8): pause all 64
/// instrument slots.
pub fn bank_monitor_pause_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 1;
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0x3f;
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        Addr(r32(c, S2)).off(0x8e8).sw(c, c.g[V0][0]);
        loop {
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, S2)).off(0x8ec).lw(c);
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
            c.g[RA][0] = sx(0x002a_d2e0);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
            callee::BANK_MONITOR_INSTANCE_PAUSE.call(c, 0x002a_d2e0)?;
            let more = (c.g[S0][0] as i64) >= 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0xc0));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_d2d0);
            }
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

/// `void SongInstance_Pause(SongInstance* self)` (0x2b2018): stop the song's
/// stream (after the driver lock dance), if it has one.
pub fn song_instance_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002b_202c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::SND_LOCK, 0x002b_202c)?;
        c.g[RA][0] = sx(0x002b_2034);
        c.g[A0][0] = c.g[S0][0];
        callee::SONG_INSTANCE_GET_STREAM_HANDLE.call(c, 0x002b_2034)?;
        if (c.g[V0][0] as i64) < 0 {
            // no stream: only the unlock/final notification below
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, callee::SNDSTRM_SET_PITCH, 0x002b_2048)?;
        }
        c.cycles += 10;
        call_ptr(c, callee::SND_UNLOCK, 0x002b_2050)?;
        c.g[A0][0] = 0x1100_0000;
        c.g[A1][0] = 1;
        c.g[A0][0] |= 0x2;
        call_ptr(c, callee::PATH_SET_PAUSED, 0x002b_2060)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SongInstance_Resume(SongInstance* self)` (0x2b2070): the reverse of
/// [`song_instance_pause`], restoring the pitch and unpausing the path.
pub fn song_instance_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002b_2084);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::SND_LOCK, 0x002b_2084)?;
        c.g[RA][0] = sx(0x002b_208c);
        c.g[A0][0] = c.g[S0][0];
        callee::SONG_INSTANCE_GET_STREAM_HANDLE.call(c, 0x002b_208c)?;
        if (c.g[V0][0] as i64) < 0 {
            // no stream
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1000;
            call_ptr(c, callee::SNDSTRM_SET_PITCH, 0x002b_20a0)?;
        }
        c.cycles += 10;
        call_ptr(c, callee::SND_UNLOCK, 0x002b_20a8)?;
        c.g[A0][0] = 0x1100_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] |= 0x2;
        call_ptr(c, callee::PATH_SET_PAUSED, 0x002b_20b8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SongInstance_PostEvent(SongInstance* self, int code, int force)`
/// (0x2b20c8): queue a music event for the currently playing path, or record
/// its code until the path starts.
pub fn song_instance_post_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V0][0] = Addr(this).off(0x5c).lw(c);
        let pending;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            if r32(c, A2) == 0 {
                frame.restore_ra(c, 0);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
            pending = Addr(this).off(0x2c).lw(c);
        } else {
            pending = Addr(this).off(0x2c).lw(c);
        }
        c.cycles += 2;
        if pending == 0 {
            c.g[V1][0] = Addr(this).off(0x30).lw(c);
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            if c.g[V1][0] == c.g[V0][0] {
                Addr(r32(c, A0)).off(0x30).sw(c, c.g[A1][0]);
            }
            c.cycles += 1;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
        } else {
            c.cycles += 6;
            c.g[A1][0] = sx(r32(c, A1) << 16);
            c.g[A0][0] = u64::MAX;
            c.g[A1][0] = sx((r32(c, A1) as i32 >> 16) as u32);
            call_ptr(c, callee::PATH_POST_EVENT, 0x002b_2100)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
        }
        Ok(())
    })
}

/// `float MusicPlayer_GetMsToNextBeat(MusicPlayer* self)` (0x2b4920): the
/// millisecond distance to the next beat of the playing song, 0 when idle.
pub fn music_player_get_ms_to_next_beat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x20);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x418).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.mtc1(0, 0);
        } else {
            c.cycles += 8;
            call_ptr(c, callee::SND_LOCK, 0x002b_4948)?;
            c.g[A0][0] = Addr(r32(c, S0)).off(0x408).lw(c);
            callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT.call(c, 0x002b_4950)?;
            c.mov_s(20, 0);
            call_ptr(c, callee::SND_UNLOCK, 0x002b_4958)?;
            c.mov_s(0, 20);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `int WorldTriggerManager_IsDuplicateActive(Trigger* triggers, int a1, int
/// a2, Trigger* except)` (0x2b66e8): index of the first of the manager's 40
/// active triggers with the same `(a1, a2)` that is not `except`, or 0.
pub fn world_trigger_manager_is_duplicate_active(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = Addr(r32(c, A0)).lw(c);
    c.g[V1][0] = 0;
    c.g[T0][0] = 1;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        // `compare_done` marks the path that already computed v0 at the
        // compare; every other path charges one more cycle and recomputes it.
        let mut compare_done = false;
        if c.g[V0][0] != c.g[T0][0] {
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        } else {
            c.cycles += 2;
            let is_except = c.g[A0][0] == c.g[A3][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if !is_except {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A0)).off(0x4).lw(c);
                if c.g[V0][0] == c.g[A1][0] {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, A0)).off(0x8).lw(c);
                    let other_key = c.g[V0][0] != c.g[A2][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 40) as u64;
                    if other_key {
                        compare_done = true;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 1;
                        abi::ret(c);
                        return;
                    }
                }
            }
        }
        if !compare_done {
            c.cycles += 1;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 40) as u64;
        }
        c.cycles += 2;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x70));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002b_66f8);
        }
    }
}

/// `void cPS2MemCard_DoSeek(MemCard* self)` (0x2c6a88): start the seek phase,
/// then advance the state machine to the result.
pub fn c_ps2_mem_card_do_seek(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        c.g[A1][0] = Addr(this).off(0x14e4).lw(c);
        c.g[A2][0] = Addr(this).off(0x14e8).lw(c);
        c.g[RA][0] = sx(0x002c_6aa8);
        call_ptr(c, callee::SCE_MC_SEEK, 0x002c_6aa8)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6abc);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6abc)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x42;
            c.g[RA][0] = sx(0x002c_6ad0);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6ad0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoRead(MemCard* self)` (0x2c6b78): start the read phase.
pub fn c_ps2_mem_card_do_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        c.g[A1][0] = Addr(this).off(0x2c).lw(c);
        c.g[A2][0] = Addr(this).off(0x24).lw(c);
        c.g[RA][0] = sx(0x002c_6b98);
        call_ptr(c, callee::SCE_MC_READ, 0x002c_6b98)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6bac);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6bac)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x44;
            c.g[RA][0] = sx(0x002c_6bc0);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6bc0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoWrite(MemCard* self)` (0x2c6bf8): start the write phase.
pub fn c_ps2_mem_card_do_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        c.g[A1][0] = Addr(this).off(0x2c).lw(c);
        c.g[A2][0] = Addr(this).off(0x24).lw(c);
        c.g[RA][0] = sx(0x002c_6c18);
        call_ptr(c, callee::SCE_MC_WRITE, 0x002c_6c18)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6c2c);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6c2c)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x46;
            c.g[RA][0] = sx(0x002c_6c40);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6c40)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoMkdir(MemCard* self)` (0x2c6c88): start creating the
/// save directory.
pub fn c_ps2_mem_card_do_mkdir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A2][0] = sx(this.wrapping_add(0xf9));
        c.g[A0][0] = Addr(this).off(0xc).lw(c);
        c.g[RA][0] = sx(0x002c_6ca8);
        call_ptr(c, callee::SCE_MC_MKDIR, 0x002c_6ca8)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6cbc);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6cbc)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x4c;
            c.g[RA][0] = sx(0x002c_6cd0);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6cd0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoDelete(MemCard* self)` (0x2c6d18): start deleting the
/// save directory.
pub fn c_ps2_mem_card_do_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A2][0] = sx(this.wrapping_add(0xf9));
        c.g[A0][0] = Addr(this).off(0xc).lw(c);
        c.g[RA][0] = sx(0x002c_6d38);
        call_ptr(c, callee::SCE_MC_DELETE, 0x002c_6d38)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6d4c);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6d4c)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x4e;
            c.g[RA][0] = sx(0x002c_6d60);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6d60)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menus (s09).

/// `void MenuSysBoolItem_Draw(MenuSysBoolItem* self, int a1)` (0x2cc758):
/// draw a two-state menu row, picking the on/off variant from the item's
/// image and state fields.
pub fn menu_sys_bool_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[T1][0] = Addr(this).off(0x18).lw(c);
        if c.g[T1][0] == 0 {
            c.g[A3][0] = Addr(this).off(0x14).lw(c);
        } else {
            c.cycles += 3;
            c.g[T0][0] = Addr(this).off(0x1c).lw(c);
            let no_state = c.g[T0][0] == 0;
            c.g[A3][0] = 0;
            if !no_state {
                c.cycles += 7;
                c.g[V1][0] = Addr(this).off(0x20).lw(c);
                c.g[A2][0] = Addr(this).off(0x14).lw(c);
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[T0][0] = c.g[T1][0];
                }
                callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002c_c78c)?;
                frame.restore_ra(c, 0);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
            c.g[A3][0] = Addr(this).off(0x14).lw(c);
        }
        c.cycles += 4;
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x002c_c7a4);
        c.g[T0][0] = 0;
        callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002c_c7a4)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// A menu item constructor: `MenuSysItem_ctor(this, -1)`, then the class
/// vtable and its two data words. Shared by the three below.
fn menu_item_ctor_body(
    c: &mut Cpu,
    this: u64,
    vtable_delta: u32,
    first: u64,
    second: u64,
    ret: u32,
) -> crate::call::Flow {
    callee::MENU_SYS_ITEM_CTOR.call(c, ret)?;
    c.g[V1][0] = 0x0048_0000;
    Addr(this as u32).off(0x14).sw(c, first);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(vtable_delta));
    Addr(this as u32).off(0x18).sw(c, second);
    Addr(this as u32).off(0x10).sw(c, c.g[V1][0]);
    c.g[V0][0] = this;
    Ok(())
}

/// `MenuSysSubmenuItem* MenuSysSubmenuItem_ctor(this, a1, a2)` (0x2cccc8).
pub fn menu_sys_submenu_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        let (this, first, second) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
        menu_item_ctor_body(c, this, 0x6de0, first, second, 0x002c_ccf0)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysColorChannelItem* MenuSysColorChannelItem_ctor(this, a1, a2)`
/// (0x2ced20).
pub fn menu_sys_color_channel_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        let (this, first, second) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
        menu_item_ctor_body(c, this, 0x6a70, first, second, 0x002c_ed48)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysColorSwatchItem* MenuSysColorSwatchItem_ctor(this, a1, a2)`
/// (0x2cff28).
pub fn menu_sys_color_swatch_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        let (this, first, second) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
        menu_item_ctor_body(c, this, 0x6968, first, second, 0x002c_ff50)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cCutSceneTransitionMan_ForceOn(Manager* self)` (0x2ea860): force the
/// transition manager on, saving the viewport once and telling the app's
/// viewport slot to take over.
pub fn c_cut_scene_transition_man_force_on(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(this).off(0x10).lw(c);
        let already_on = c.g[V0][0] != 0;
        c.g[A2][0] = Addr(gp(c, 0x2a90)).lw(c);
        if !already_on {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002e_a884);
            callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT.call(c, 0x002e_a884)?;
            c.g[A2][0] = Addr(gp(c, 0x2a90)).lw(c);
        }
        c.cycles += 12;
        c.g[V0][0] = 1;
        Addr(this).off(0x10).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(this.wrapping_add(0x34));
        c.g[V0][0] = Addr(r32(c, A2)).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x148).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x14c).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x002e_a8a8);
        c.g[A0][0] = this_with_delta(c.g[A2][0], c.g[A0][0]);
        call_ptr(c, target, 0x002e_a8a8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `const char* WorldPainter_GetIrradianceName2(void)` (0x2ef198): the static
/// fallback name when no world painter is live, else the painter's name.
pub fn world_painter_get_irradiance_name2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp(c, 0xa74)).lw(c);
        let live = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !live {
            c.cycles += 14;
            c.g[V1][0] = 0xf0;
            c.g[V0][0] = 0x0050_0000;
            c.mult(0, V1, A0, V1);
            c.g[V0][0] = sx(0x004f_a370);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
            c.g[A1][0] = Addr(r32(c, A0)).lw(c);
            c.g[V1][0] = Addr(r32(c, A1)).off(0x4).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x138).lh(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x13c).lw(c);
            let target = r32(c, V0);
            c.g[RA][0] = sx(0x002e_f1d8);
            c.g[A0][0] = this_with_delta(c.g[A1][0], c.g[A0][0]);
            call_ptr(c, target, 0x002e_f1d8)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        c.g[V0][0] = sx(gp(c, 0x24c0));
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// App and strings (s10).

/// `void App_v02(App* self)` (0x317400): let the app object's screen update,
/// then, if it is still there, tell it the app finished.
pub fn app_v02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(this).off(0x8).lw(c);
        c.g[V0][0] = Addr(r32(c, A1)).off(0x8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x18).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0031_7428);
        c.g[A0][0] = this_with_delta(c.g[A1][0], c.g[A0][0]);
        call_ptr(c, target, 0x0031_7428)?;
        c.g[A2][0] = Addr(this).off(0x8).lw(c);
        let gone = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !gone {
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, A2)).off(0x8).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x8).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0031_7448);
            c.g[A0][0] = this_with_delta(c.g[A2][0], c.g[A0][0]);
            call_ptr(c, target, 0x0031_7448)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_ctor_copy(cBXString* self, const cBXString* other)`
/// (0x317fe8): share `other`'s buffer when it is already shared, else copy.
pub fn c_bx_string_ctor_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0xffff_fff4).lw(c);
        let shared = (c.g[V0][0] as i64) < 0;
        c.g[S0][0] = c.g[A0][0];
        if shared {
            c.cycles += 5;
            c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            Addr(r32(c, S0)).sw(c, c.g[V0][0]);
            c.g[A1][0] = Addr(r32(c, A1)).lw(c);
            c.g[RA][0] = sx(0x0031_802c);
            callee::BXSTRING_ASSIGN_CSTR.call(c, 0x0031_802c)?;
        } else {
            c.cycles += 5;
            Addr(r32(c, S0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(r32(c, V1)).off(0xffff_fff4).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            Addr(r32(c, V1)).off(0xffff_fff4).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_assign_cstr(cBXString* self, const char* str)`
/// (0x3184e8): replace the contents with a C string.
pub fn c_bx_string_assign_cstr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let empty = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        let len;
        if empty {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.g[A1][0] = c.g[V0][0];
            len = c.g[A1][0];
        } else {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0031_850c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::STRLEN, 0x0031_850c)?;
            c.g[A1][0] = c.g[V0][0];
            len = c.g[A1][0];
        }
        c.cycles += 9;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = len;
        c.g[RA][0] = sx(0x0031_8528);
        callee::BXSTRING_ASSIGN_COPY.call(c, 0x0031_8528)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_AppendCStr(cBXString* self, const char* str)`
/// (0x318910): append a C string in place.
pub fn c_bx_string_append_cstr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let empty = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        let len;
        if empty {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.g[A1][0] = c.g[V0][0];
            len = c.g[A1][0];
        } else {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0031_8934);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::STRLEN, 0x0031_8934)?;
            c.g[A1][0] = c.g[V0][0];
            len = c.g[A1][0];
        }
        c.cycles += 9;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = len;
        c.g[RA][0] = sx(0x0031_8950);
        callee::BXSTRING_CONCAT_IN_PLACE.call(c, 0x0031_8950)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World and modifiers (s11).

/// `void CrowdMan3d_RemoveSection(CrowdMan3d* self, int section)` (0x343c08):
/// remove a section from each of the manager's five groups.
pub fn crowd_man3d_remove_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx(r32(c, S0).wrapping_add(0x9c4));
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S0][0];
        loop {
            c.cycles += 6;
            c.g[RA][0] = sx(0x0034_3c38);
            c.g[A1][0] = c.g[S2][0];
            callee::CROWD_MAN3D_GROUP_REMOVE_SECTION.call(c, 0x0034_3c38)?;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x1f4));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_3c30);
            }
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

/// `void FlexBridge_Save(FlexBridge* self, Stream* stream)` (0x347d38): save
/// the world object, the bridge springs and the 0x30 bytes at +0x20.
pub fn flex_bridge_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0034_7d54);
        c.g[S1][0] = c.g[A1][0];
        callee::WORLD_OBJ_SAVE.call(c, 0x0034_7d54)?;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x50));
        c.g[RA][0] = sx(0x0034_7d60);
        c.g[A1][0] = c.g[S1][0];
        callee::FLEX_BRIDGE_SPRINGS_SAVE.call(c, 0x0034_7d60)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0x20));
        c.g[A2][0] = 0x30;
        c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0034_7d7c);
        c.g[A0][0] = this_with_delta(c.g[S1][0], c.g[A0][0]);
        call_ptr(c, target, 0x0034_7d7c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DeadFade_Update(DeadFade* self)` (0x350c38): fade the alpha down by
/// its step until it reaches zero, then spawn the dead node.
pub fn dead_fade_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.mtc1(0, 0);
        let a1 = r32(c, A1);
        c.lwc1(1, p(a1, 0x30));
        c.lwc1(2, p(a1, 0x34));
        c.lwc1(3, p(a1, 0x28));
        c.add_s(1, 1, 2);
        c.c_lt_s(0, 3);
        let expired = !c.fcc();
        c.swc1(1, p(a1, 0x30));
        if expired {
            c.cycles += 3;
            c.g[A0][0] = Addr(r32(c, A1)).off(0x18).lw(c);
            c.g[RA][0] = sx(0x0035_0c84);
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1c));
            call_ptr(c, callee::WORLD_CREATE_DEAD_NODE, 0x0035_0c84)?;
        } else {
            c.cycles += 4;
            c.lwc1(0, p(a1, 0x2c));
            c.sub_s(0, 3, 0);
            c.swc1(0, p(a1, 0x28));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ModObj_AddAvaSplineModifier(ModObj* self)` (0x355a78): create the
/// Ava spline transform modifier once and install it.
pub fn mod_obj_add_ava_spline_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, 0x9d4)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let installed = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if !installed {
            c.cycles += 12;
            c.g[A1][0] = 0x0049_0000;
            c.g[A0][0] = 0x90;
            c.g[A1][0] = sx(0x0048_e928);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0035_5aa8)?;
            c.g[A1][0] = Addr(r32(c, S0)).off(0x18).lw(c);
            c.g[RA][0] = sx(0x0035_5ab4);
            c.g[A0][0] = c.g[V0][0];
            callee::AVA_SPLINE_MODIFIER_CTOR.call(c, 0x0035_5ab4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0035_5ac0);
            c.g[A1][0] = c.g[V0][0];
            callee::MOD_OBJ_SET_TRANSFORM_MODIFIER.call(c, 0x0035_5ac0)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PositionModifier* PositionModifier_ctor(PositionModifier* self, const
/// Quad* pos)` (0x356eb8): vtable, copied home position, identity transform
/// and the initial matrix.
pub fn position_modifier_ctor(c: &mut Cpu) {
    c.cycles += 22;
    c.g[V0][0] = 0x0049_0000;
    c.g[V1][0] = 0x0050_0000;
    c.g[V0][0] = sx(0x0048_f5f0);
    c.g[V1][0] = sx(0x004f_f1a0);
    c.sw(r32(c, A0), c.g[V0][0]);
    c.g[A2][0] = sx(r32(c, A0).wrapping_add(0x50));
    c.lq(2, r32(c, A1));
    c.sq(2, p(r32(c, A0), 0x30));
    c.lqc2(1, r32(c, V1));
    c.lqc2(2, p(r32(c, V1), 0x10));
    c.lqc2(3, p(r32(c, V1), 0x20));
    c.lqc2(4, p(r32(c, V1), 0x30));
    c.sqc2(1, r32(c, A2));
    c.sqc2(2, p(r32(c, A2), 0x10));
    c.sqc2(3, p(r32(c, A2), 0x20));
    c.sqc2(4, p(r32(c, A2), 0x30));
    c.g[V0][0] = 1;
    c.lq(5, p(r32(c, A0), 0x30));
    c.sw(p(r32(c, A0), 0x44), c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    c.sq(5, p(r32(c, A0), 0x80));
    abi::ret(c);
}

/// `ParentModifier* ParentModifier_ctor(self, a1, a2, pos)` (0x357038): the
/// position modifier with a different vtable and two extra fields.
pub fn parent_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A3][0];
        c.g[RA][0] = sx(0x0035_7060);
        callee::POSITION_MODIFIER_CTOR.call(c, 0x0035_7060)?;
        c.g[V1][0] = 0x0049_0000;
        Addr(r32(c, S0)).off(0xa0).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx(0x0048_f508);
        Addr(r32(c, S0)).off(0x90).sw(c, c.g[S2][0]);
        Addr(r32(c, S0)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Graphics (s12).

/// `void AnimTeeter_dtor(AnimTeeter* self, int flags)` (0x361400): tear down
/// the anim object and free the block when `flags & 1`.
pub fn anim_teeter_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_08f8);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        Addr(r32(c, S1)).off(0x50).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x30));
        c.g[RA][0] = sx(0x0036_1430);
        c.g[A1][0] = 0;
        callee::ANIM_OBJECT_BASE_DTOR.call(c, 0x0036_1430)?;
        c.g[S0][0] &= 0x1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0036_1444);
            c.g[A0][0] = c.g[S1][0];
            callee::BUILTIN_DELETE.call(c, 0x0036_1444)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Elem* Floating_v29(Floating* self, int index)` (0x361800): wake the
/// element through its virtual if the float flag is set, and return
/// `&self->elems[index]`.
pub fn floating_v29(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x12).lhu(c);
        c.g[V0][0] &= 0x1;
        let idle = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !idle {
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, S0)).off(0xc).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x198).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x19c).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0036_1838);
            c.g[A0][0] = this_with_delta(c.g[S0][0], c.g[A0][0]);
            call_ptr(c, target, 0x0036_1838)?;
        }
        c.cycles += 8;
        c.g[V1][0] = Addr(r32(c, S0)).off(0x74).lw(c);
        c.g[V0][0] = sx(r32(c, S1) << 6);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* PS2GraphicsMan_SelectMatchingDisplayMode(PS2GraphicsMan* self)`
/// (0x376468): pick the first display mode the platform accepts, else 0.
pub fn ps2_graphics_man_select_matching_display_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = Addr(this).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x3d8).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x3dc).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0037_648c);
        c.g[A0][0] = this_with_delta(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, target, 0x0037_648c)?;
        if (c.g[V0][0] as i64) < 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.g[V1][0] = Addr(r32(c, S0)).off(0x10d8).lw(c);
            c.cycles += 5;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = Addr(r32(c, V1)).off(0x20).lh(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x24).lw(c);
            let target = r32(c, V0);
            c.g[RA][0] = sx(0x0037_64b0);
            c.g[A0][0] = this_with_delta(c.g[S0][0], c.g[A0][0]);
            call_ptr(c, target, 0x0037_64b0)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PS2GraphicsMan_LoadTexture(self, a1, a2, a3, a4, a5, a6, a7)`
/// (0x37caf8): forward to the texture manager and mark the returned slot used.
pub fn ps2_graphics_man_load_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = Addr(frame.at(c, 0x48)).lw(c);
        c.g[V0][0] = Addr(frame.at(c, 0x40)).lw(c);
        c.g[T4][0] = Addr(frame.at(c, 0x50)).lw(c);
        c.g[T5][0] = Addr(frame.at(c, 0x58)).lw(c);
        c.g[A0][0] = Addr(r32(c, S0)).off(0x18f4).lw(c);
        Addr(frame.at(c, 0x8)).sw(c, c.g[V1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[V0][0]);
        Addr(frame.at(c, 0x10)).sw(c, c.g[T4][0]);
        c.g[RA][0] = sx(0x0037_cb30);
        Addr(frame.at(c, 0x18)).sw(c, c.g[T5][0]);
        callee::TEX_MAN_LOAD_TEXTURE.call(c, 0x0037_cb30)?;
        c.g[V1][0] = sx(r32(c, V0) << 2);
        c.g[A0][0] = 1;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
        frame.restore_ra(c, 0x20);
        Addr(r32(c, S0)).off(0x3838).sw(c, c.g[A0][0]);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2GraphicsMan_EmitModelMatrix2(PS2GraphicsMan* self, Quad** pp,
/// const Matrix* m)` (0x37e040): append a model-matrix packet to the
/// graphics packet stream.
pub fn ps2_graphics_man_emit_model_matrix2(c: &mut Cpu) {
    c.cycles += 22;
    c.g[V0][0] = Addr(r32(c, A1)).lw(c);
    c.por(3, 0, 0);
    c.g[V1][0] = 0x1000_0000;
    c.g[V1][0] |= 0x5;
    c.g[A2][0] = 0x6c04_0000;
    c.g[A2][0] |= 0x8005;
    c.g[A2][0] <<= 32;
    c.sq(3, r32(c, V0));
    c.g[A3][0] = sx(r32(c, V0).wrapping_add(0x20));
    c.sd(p(r32(c, V0), 0x18), c.g[A2][0]);
    c.sd(p(r32(c, V0), 0x10), 0);
    c.lqc2(1, r32(c, A0));
    c.lqc2(2, p(r32(c, A0), 0x10));
    c.lqc2(3, p(r32(c, A0), 0x20));
    c.lqc2(4, p(r32(c, A0), 0x30));
    c.sqc2(1, r32(c, A3));
    c.sqc2(2, p(r32(c, A3), 0x10));
    c.sqc2(3, p(r32(c, A3), 0x20));
    c.sqc2(4, p(r32(c, A3), 0x30));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x60));
    c.sw(r32(c, A1), c.g[V0][0]);
    abi::ret(c);
}

/// `void PS2GraphicsMan_OnVif1DmaEnd(PS2GraphicsMan* self)` (0x382688): run
/// the DMA-end state machine (start, finish or idle).
pub fn ps2_graphics_man_on_vif1_dma_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = 1;
        c.g[V0][0] = Addr(r32(c, A1)).off(0x5a8c).lw(c);
        let idle = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 2;
        if !idle {
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, A1)).off(0x5ac8).lw(c);
            Addr(r32(c, A1)).off(0x5a8c).sw(c, c.g[V0][0]);
            c.g[RA][0] = sx(0x0038_26b4);
            call_ptr(c, callee::ISIGNAL_SEMA, 0x0038_26b4)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(r32(c, A1)).off(0x5a8c).lw(c);
        c.g[V0][0] = 3;
        let not_running = c.g[V1][0] != c.g[V0][0];
        frame.restore_ra(c, 0);
        if !not_running {
            c.cycles += 3;
            c.g[V0][0] = 4;
            Addr(r32(c, A1)).off(0x5a8c).sw(c, c.g[V0][0]);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `Matrix* VfxPlants_GetSwayMatrix(const Vec2* pos)` (0x38f2a8): one of eight
/// sway matrices chosen from the plant's distance to the origin.
pub fn vfx_plants_get_sway_matrix(c: &mut Cpu) {
    c.cycles += 22;
    let a0 = r32(c, A0);
    c.lwc1(1, p(a0, 0x4));
    c.g[A1][0] = u64::MAX;
    c.lwc1(0, p(a0, 0x0));
    c.mul_s(1, 1, 1);
    c.lwc1(2, gp(c, 0x1368));
    c.mul_s(0, 0, 0);
    c.g[A0][0] = 0x0050_0000;
    c.g[A0][0] = sx(0x0050_4d80);
    c.add_s(0, 0, 1);
    c.sqrt_s(0, 0);
    c.mul_s(2, 2, 0);
    c.cvt_w_s(0, 2);
    c.mfc1(2, 0);
    c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[V1][0] = sx(r32(c, V0).wrapping_add(7));
    if c.g[A1][0] != 0 {
        c.g[V1][0] = c.g[V0][0];
    }
    c.g[V1][0] = sx(((r32(c, V1) as i32) >> 3) as u32);
    c.g[V1][0] = sx(r32(c, V1) << 3);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
    c.g[V0][0] = sx(r32(c, V0) << 6);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI (s13).

/// `UIList* UIList_ctor(UIList* self)` (0x3977e8): empty circular list with
/// its two vtables.
pub fn ui_list_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = 0x0049_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0xc));
        c.g[V1][0] = sx(0x0049_4cc0);
        c.g[V0][0] = sx(0x0049_4c98);
        let s = r32(c, S0);
        Addr(s).off(0x4).sw(c, c.g[S0][0]);
        Addr(s).sw(c, c.g[S0][0]);
        Addr(s).off(0x18).sw(c, c.g[V0][0]);
        Addr(s).off(0x14).sw(c, c.g[V1][0]);
        Addr(s).off(0xc).sw(c, c.g[A1][0]);
        Addr(s).off(0x8).sw(c, c.g[V1][0]);
        c.g[RA][0] = sx(0x0039_782c);
        Addr(s).off(0x10).sw(c, c.g[A1][0]);
        callee::UI_LIST_RESET.call(c, 0x0039_782c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void UIStateMgr_AddBackgroundState(UIStateMgr* self, State* state)`
/// (0x39f238): mark the state as a background state and append it to the
/// background list.
pub fn ui_state_mgr_add_background_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x1c).lw(c);
        c.g[V1][0] = Addr(r32(c, S0)).off(0x8).lw(c);
        c.g[V0][0] |= 0x60;
        Addr(r32(c, S0)).off(0x1c).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x20).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x24).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0039_f270);
        c.g[A0][0] = this_with_delta(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, target, 0x0039_f270)?;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x1c));
        c.g[RA][0] = sx(0x0039_f27c);
        c.g[A1][0] = c.g[S0][0];
        callee::UI_LIST_PUSH_BACK.call(c, 0x0039_f27c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2ChunkResolve_dtor(cPS2ChunkResolve* self, int flags)` (0x3aade8):
/// two-stage teardown with an optional free.
pub fn c_ps2_chunk_resolve_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x10);
        c.g[V0][0] = sx(0x0049_5090);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x003a_ae10);
        Addr(r32(c, S1)).off(0x8).sw(c, c.g[V0][0]);
        callee::CPS2_CHUNK_RESOLVE_DTOR_STUB.call(c, 0x003a_ae10)?;
        c.g[V0][0] = 0x0049_0000;
        c.g[S0][0] &= 0x1;
        c.g[V0][0] = sx(0x0049_5150);
        let keep = c.g[S0][0] == 0;
        Addr(r32(c, S1)).off(0x8).sw(c, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[RA][0] = sx(0x003a_ae2c);
            c.g[A0][0] = c.g[S1][0];
            callee::BUILTIN_DELETE.call(c, 0x003a_ae2c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Riders, camera and front end (s00-s02).

/// `void Rider_OnMissionCompleted(Rider* self, void* a1, int a2)` (0x10f2d8):
/// forward the mission result to the metric, then to the reward interface.
pub fn rider_on_mission_completed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[RA][0] = sx(0x0010_f2fc);
        c.g[A0][0] = Addr(r32(c, S0)).off(0x790).lw(c);
        callee::RIDER_METRIX_ON_MISSION_COMPLETED.call(c, 0x0010_f2fc)?;
        c.g[RA][0] = sx(0x0010_f304);
        call_ptr(c, callee::BELIBRARY_GET_INSTANCE, 0x0010_f304)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0010_f310);
        c.g[A1][0] = 0xd;
        call_ptr(c, callee::BELIBRARY_GET_INTERFACE, 0x0010_f310)?;
        c.g[A1][0] = Addr(r32(c, S0)).off(0x86c).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0010_f320);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::BEREWARD_ON_MISSION_COMPLETED, 0x0010_f320)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_OnItemCollected(Rider* self, void* a1)` (0x10f338): award the
/// collectible points and notify the reward interface.
pub fn rider_on_item_collected(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 3;
        c.g[RA][0] = sx(0x0010_f35c);
        c.g[A0][0] = Addr(r32(c, S0)).off(0x790).lw(c);
        callee::RIDER_METRIX_AWARD_POINTS.call(c, 0x0010_f35c)?;
        c.g[RA][0] = sx(0x0010_f364);
        call_ptr(c, callee::BELIBRARY_GET_INSTANCE, 0x0010_f364)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0010_f370);
        c.g[A1][0] = 0xd;
        call_ptr(c, callee::BELIBRARY_GET_INTERFACE, 0x0010_f370)?;
        c.g[A1][0] = Addr(r32(c, S0)).off(0x86c).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0010_f380);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::BEREWARD_ON_COLLECTIBLE_COLLECTED, 0x0010_f380)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderBoardPressSub_Cancel(RiderBoardPress* self)` (0x131348): cancel
/// a board press, finishing the metric and granting the boost when the press
/// had been held for one or two frames.
pub fn rider_board_press_sub_cancel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0013_135c);
        c.g[S0][0] = c.g[A0][0];
        callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW.call(c, 0x0013_135c)?;
        c.g[RA][0] = sx(0x0013_1364);
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS.call(c, 0x0013_1364)?;
        c.g[V1][0] = Addr(r32(c, S0)).off(0x14).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x330).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            c.g[RA][0] = sx(0x0013_1384);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x790).lw(c);
            callee::RIDER_METRIX_FINISH_BOARD_PRESS.call(c, 0x0013_1384)?;
            c.g[A0][0] = Addr(r32(c, S0)).off(0x14).lw(c);
            c.mov_s(12, 0);
            c.g[RA][0] = sx(0x0013_1394);
            c.g[A1][0] = 1;
            callee::RIDER_ADD_BOOST.call(c, 0x0013_1394)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BENewPlayerInt_RestoreDefaults(BENewPlayerInt* self)` (0x145bd8):
/// reset the six control entries and let the player object re-read its map.
pub fn be_new_player_int_restore_defaults(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0053_0000;
        c.g[A1][0] = c.g[A0][0];
        c.g[V1][0] = sx(0x0053_5b20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0053_5bc8);
        c.sb(p(r32(c, V1), 0x13), 0);
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, V1)).off(0x10).lw(c);
            c.g[V0][0] |= 0x2;
            Addr(r32(c, V1)).off(0x10).sw(c, c.g[V0][0]);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x1c));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.sb(p(r32(c, V1), 0x13), 0);
            if event_due(c) {
                deliver_event_here(c, 0x0014_5bf8);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = Addr(r32(c, A1)).off(0xc).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x10).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x14).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0014_5c28);
        c.g[A0][0] = this_with_delta(c.g[A1][0], c.g[A0][0]);
        call_ptr(c, target, 0x0014_5c28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CamController_dtor(CamController* self, int flags)` (0x15cc10):
/// destroy the algorithm list and optionally free the object.
pub fn cam_controller_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0045_b938);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(r32(c, S0)).off(0x14).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(r32(c, S0)).off(0x8).lw(c);
        let no_list = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_list {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0015_cc44);
            c.g[A1][0] = 3;
            callee::ALGORITHM_LIST_DTOR.call(c, 0x0015_cc44)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0015_cc58);
            c.g[A0][0] = c.g[S0][0];
            callee::BUILTIN_DELETE.call(c, 0x0015_cc58)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ChaseCamController_SetDefaultAlgorithm(ChaseCamController* self,
/// void* algorithm)` (0x161ef0): publish the default algorithm and apply it
/// immediately when no blend is in progress.
pub fn chase_cam_controller_set_default_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A1][0];
        Addr(r32(c, A2)).off(0x24).sw(c, c.g[A3][0]);
        c.g[A1][0] = sx(gp(c, 0x1ce0));
        c.g[V1][0] = Addr(r32(c, A2)).off(0x20).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x4).lw(c);
        c.g[V0][0] = Addr(r32(c, A0)).off(0x870).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
        c.sw(r32(c, V0), c.g[A3][0]);
        c.g[V1][0] = Addr(r32(c, A2)).off(0x30).lw(c);
        c.g[V1][0] &= 0x7;
        let blending = c.g[V1][0] != 0;
        frame.restore_ra(c, 0);
        if !blending {
            c.cycles += 5;
            c.g[A0][0] = c.g[A2][0];
            c.g[A1][0] = c.g[A3][0];
            c.g[A2][0] = 0;
            c.g[RA][0] = sx(0x0016_1f40);
            callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE.call(c, 0x0016_1f40)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `DirectionNeChaseCameraAlgorithm* DirectionNeChaseCameraAlgorithm_ctor(
/// self, int n)` (0x1774c0): chase-camera algorithm scaled by `n`.
pub fn direction_ne_chase_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0017_74dc);
        c.g[S0][0] = c.g[A1][0];
        callee::CHASE_CAMERA_ALGORITHM_CTOR.call(c, 0x0017_74dc)?;
        c.mtc1(16, 0);
        c.cvt_s_w(0, 0);
        c.lwc1(1, gp(c, -0x5f4c));
        c.g[V0][0] = 0x0046_0000;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x12));
        c.g[V0][0] = sx(0x0045_cb28);
        Addr(r32(c, S1)).off(0xc).sw(c, c.g[S0][0]);
        c.mul_s(0, 0, 1);
        Addr(r32(c, S1)).off(0x10).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        c.swc1(0, p(r32(c, S1), 0x390));
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int DebugMenu_IsTrackSelectable(int a0, int track_id)` (0x179fc8): 1 when
/// the debug track is the current station track or one of the three special
/// ids.
pub fn debug_menu_is_track_selectable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0017_9fdc);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, callee::BELIBRARY_GET_INSTANCE, 0x0017_9fdc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0017_9fe8);
        c.g[A1][0] = 0;
        call_ptr(c, callee::BELIBRARY_GET_INTERFACE, 0x0017_9fe8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0017_9ff4);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX, 0x0017_9ff4)?;
        c.g[A0][0] = 1;
        let match_current = c.g[V0][0] == c.g[A0][0];
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(0xe));
        if match_current {
            c.cycles += 1;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0;
            c.g[V1][0] = (c.g[V1][0] < 3) as u64;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[A0][0];
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_ShowOccupantDetail(LobbyMenu* self)` (0x17e3b8): drain the
/// occupant list and open the detail panel for the selected occupant.
pub fn lobby_menu_show_occupant_detail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(this).off(0x8).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx(this.wrapping_add(0x8));
        if !empty {
            loop {
                c.cycles += 5;
                c.g[RA][0] = sx(0x0017_e3e0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::LOBBY_OCCUPANT_POP, 0x0017_e3e0)?;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e3d8);
                }
            }
        }
        c.cycles += 10;
        c.g[V0][0] = 0xe;
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x1534));
        Addr(r32(c, S1)).off(0x4).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x0017_e400);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x8));
        call_ptr(c, callee::LOBBY_OCCUPANT_SELECT, 0x0017_e400)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void KeyboardScreen_CursorDown(KeyboardScreen* self)` (0x180768): move
/// the cursor to the next row, wrapping through the hidden rows.
pub fn keyboard_screen_cursor_down(c: &mut Cpu) {
    c.cycles += 5;
    let a0 = r32(c, A0);
    c.g[V1][0] = Addr(a0).off(0x170).lw(c);
    c.g[A1][0] = sx(r32(c, V1).wrapping_sub(0x1e));
    c.g[V0][0] = (c.g[A1][0] < 0xa) as u64;
    let in_normal_range = c.g[V0][0] != 0;
    c.g[V0][0] = 0x1c;
    if !in_normal_range {
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(a0).off(0x170).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(a0).off(0xe0).lw(c);
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(a0).off(0x170).lw(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x27;
                Addr(a0).off(0x170).sw(c, c.g[V0][0]);
                abi::ret(c);
                return;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xa));
        Addr(a0).off(0x170).sw(c, c.g[V0][0]);
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(a0).off(0xd8).lw(c);
    let no_shift = c.g[V0][0] == 0;
    c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x14));
    c.cycles += 2;
    if no_shift {
        Addr(a0).off(0x170).sw(c, c.g[V0][0]);
    } else {
        Addr(a0).off(0x170).sw(c, c.g[A1][0]);
    }
    abi::ret(c);
}

/// `int cFEStateCharSelect_TransitionIn(cFEStateCharSelect* self, void* on)`
/// (0x1812a8): refresh the selected character's attribute bars on entry.
pub fn c_fe_state_char_select_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let inactive = r32(c, A1) == 0;
        c.g[S0][0] = c.g[A0][0];
        if !inactive {
            c.cycles += 13;
            c.g[RA][0] = sx(0x0018_12c4);
            call_ptr(c, callee::BELIBRARY_GET_INSTANCE, 0x0018_12c4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0018_12d0);
            c.g[A1][0] = 1;
            call_ptr(c, callee::BELIBRARY_GET_INTERFACE, 0x0018_12d0)?;
            c.g[A1][0] = Addr(r32(c, S0)).off(0x44).lb(c);
            c.g[RA][0] = sx(0x0018_12dc);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT, 0x0018_12dc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = sx(r32(c, V0) << 24);
            c.g[A1][0] = Addr(r32(c, A0)).off(0x44).lb(c);
            c.g[A2][0] = sx((r32(c, V0) as i32 >> 24) as u32);
            c.g[RA][0] = sx(0x0018_12f0);
            callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS.call(c, 0x0018_12f0)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, put_words, scramble, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 100;

    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    fn clear(c: &mut Cpu, at: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(at + 4 * i, 0);
        }
    }

    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// Scrambled state, fake hooks, a zeroed BELibrary dummy at `gp-0x1f04`.
    fn common(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        clear(c, DATA + 0xb000, 0x100);
        c.bus.write32(gp_at(-0x1f04), DATA + 0xb000);
        r
    }

    fn call_setup(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        const RET: u64 = 0x0012_3450;
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = RET;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, RET as u32, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// The main heap's arena over 256 KiB of test RAM.
    fn alloc_ready(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = GAME_GP as u64;
        call_setup(
            c,
            0x0031_ff60,
            ssx3_ee::s10::f_0031ff60,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
        c.bus.write32(GAME_GP + 0xdd8, 1);
    }

    fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        const MEM_HEAP: u32 = 0x004f_f250;
        call_setup(
            c,
            0x0032_0058,
            ssx3_ee::s10::f_00320058,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    /// A vtable whose chosen slots are `{delta, FAKE+k}` entries.
    fn vtable(c: &mut Cpu, at: u32, slots: &[(u32, i16, u32)]) {
        for &(off, delta, target) in slots {
            c.bus.write16(at + off, delta as u16);
            c.bus.write32(at + off + 4, target);
        }
    }

    // ------------------------------------------------------------------
    // Sound.

    #[test]
    fn ini_parser_skip_comment_matches() {
        check(ee::s08::f_002a79c8__generated, ini_parser_skip_comment, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x2a79c8, 1);
            let text = DATA + 0x1000;
            clear(c, text, 0x40);
            match seed % 4 {
                0 => {}
                1 => {
                    c.bus.write8(text, b'#');
                    c.bus.write8(text + 1, b'a');
                    c.bus.write8(text + 2, b'\n');
                }
                2 => {
                    c.bus.write8(text, b';');
                    c.bus.write8(text + 1, b'b');
                    c.bus.write8(text + 2, b'\r');
                }
                _ => {
                    c.bus.write8(text, b'x');
                }
            }
            c.bus.write32(DATA + 8, text);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn ps2_sound_manager_free_spu_block_matches() {
        check(ee::s08::f_002a92f8__generated, ps2_sound_manager_free_spu_block, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x2a92f8, 1);
            clear(c, DATA, 0x100);
            clear(c, DATA + 0x200, 0x40);
            c.bus.write32(DATA + 0x200, r.next_u32());
            c.bus.write32(DATA + 0x208, r.next_u32());
            c.bus.write32(DATA + 0x30c, r.next_u32());
            c.bus.write32(DATA + 0x310, r.next_u32());
            c.bus.write32(DATA + 0x31c, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x200);
        });
    }

    #[test]
    fn bank_monitor_pause_all_matches() {
        check(ee::s08::f_002ad2a8__generated, bank_monitor_pause_all, 20, &|c, seed| {
            let _ = common(c, seed, 0x2ad2a8, 1);
            let this = DATA + 0x100;
            let pool = DATA + 0x2000;
            clear(c, this, 0x300);
            clear(c, pool, 64 * 0xc0 / 4);
            c.bus.write32(this + 0x8ec, pool);
            c.bus.write32(pool + 0x0, seed % 3);
            c.bus.write32(pool + 0x54, (seed / 3) % 2);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn song_instance_pause_matches() {
        check(ee::s08::f_002b2018__generated, song_instance_pause, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x2b2018, 4);
            let this = DATA + 0x100;
            clear(c, this, 0x200);
            c.bus.write32(this + 0x408, DATA + 0x2000);
            c.g[A0][0] = sx(this);
            let _ = seed;
        });
    }

    #[test]
    fn song_instance_resume_matches() {
        check(ee::s08::f_002b2070__generated, song_instance_resume, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x2b2070, 4);
            let this = DATA + 0x100;
            clear(c, this, 0x200);
            c.bus.write32(this + 0x408, DATA + 0x2000);
            c.g[A0][0] = sx(this);
            let _ = seed;
        });
    }

    #[test]
    fn song_instance_post_event_matches() {
        check(ee::s08::f_002b20c8__generated, song_instance_post_event, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x2b20c8, 1);
            let this = DATA + 0x100;
            clear(c, this, 0x200);
            c.bus.write32(this + 0x5c, (seed % 2) as u32);
            c.bus.write32(this + 0x2c, ((seed / 2) % 2) as u32);
            c.bus.write32(this + 0x30, if seed % 3 == 0 { u32::MAX } else { r.next_u32() });
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = ((seed / 4) % 2) as u64;
        });
    }

    #[test]
    fn music_player_get_ms_to_next_beat_matches() {
        check(ee::s08::f_002b4920__generated, music_player_get_ms_to_next_beat, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x2b4920, 4);
            let this = DATA + 0x100;
            clear(c, this, 0x300);
            c.bus.write32(this + 0x418, (seed % 2) as u32);
            c.bus.write32(this + 0x408, DATA + 0x2000);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn world_trigger_manager_is_duplicate_active_matches() {
        check(ee::s08::f_002b66e8__generated, world_trigger_manager_is_duplicate_active, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x2b66e8);
            let list = DATA + 0x1000;
            clear(c, list, 0x40 * 0x1c);
            let except = list + 0x70 * ((seed % 5) as u32);
            let a1 = r.next_u32() as u64;
            let a2 = r.next_u32() as u64;
            for i in 0..8u32 {
                let e = list + i * 0x70;
                // A few active entries with the queried keys, one of them the
                // exception, so both the found and the not-found exits run.
                c.bus.write32(e, if i % 3 == 0 { 1 } else { 0 });
                c.bus.write32(e + 4, a1 as u32);
                c.bus.write32(e + 8, if i % 2 == 0 { a2 as u32 } else { r.next_u32() });
            }
            c.g[A0][0] = sx(DATA + 0x900);
            c.bus.write32(DATA + 0x900, list);
            c.g[A1][0] = a1;
            c.g[A2][0] = a2;
            c.g[A3][0] = sx(except);
        });
    }

    #[test]
    fn c_ps2_mem_card_do_seek_matches() {
        check(ee::s08::f_002c6a88__generated, c_ps2_mem_card_do_seek, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x2c6a88, 1);
            let this = DATA + 0x100;
            clear(c, this, 0x400);
            c.bus.write32(this + 0x18, r.next_u32());
            c.bus.write32(this + 0x14e4, r.next_u32());
            c.bus.write32(this + 0x14e8, r.next_u32());
            c.g[A0][0] = sx(this);
            let _ = seed;
        });
    }

    #[test]
    fn c_ps2_mem_card_do_read_matches() {
        check(ee::s08::f_002c6b78__generated, c_ps2_mem_card_do_read, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x2c6b78, 1);
            let this = DATA + 0x100;
            clear(c, this, 0x400);
            c.bus.write32(this + 0x18, r.next_u32());
            c.bus.write32(this + 0x2c, DATA + 0x3000);
            c.bus.write32(this + 0x24, 0x200);
            c.g[A0][0] = sx(this);
            let _ = seed;
        });
    }

    #[test]
    fn c_ps2_mem_card_do_write_matches() {
        check(ee::s08::f_002c6bf8__generated, c_ps2_mem_card_do_write, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x2c6bf8, 1);
            let this = DATA + 0x100;
            clear(c, this, 0x400);
            c.bus.write32(this + 0x18, r.next_u32());
            c.bus.write32(this + 0x2c, DATA + 0x3000);
            c.bus.write32(this + 0x24, 0x200);
            c.g[A0][0] = sx(this);
            let _ = seed;
        });
    }

    #[test]
    fn c_ps2_mem_card_do_mkdir_matches() {
        check(ee::s08::f_002c6c88__generated, c_ps2_mem_card_do_mkdir, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x2c6c88, 1);
            let this = DATA + 0x100;
            clear(c, this, 0x400);
            c.bus.write32(this + 0xc, r.next_u32());
            c.bus.write8(this + 0xf9, b'B');
            c.g[A0][0] = sx(this);
            let _ = seed;
        });
    }

    #[test]
    fn c_ps2_mem_card_do_delete_matches() {
        check(ee::s08::f_002c6d18__generated, c_ps2_mem_card_do_delete, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x2c6d18, 1);
            let this = DATA + 0x100;
            clear(c, this, 0x400);
            c.bus.write32(this + 0xc, r.next_u32());
            c.bus.write8(this + 0xf9, b'B');
            c.g[A0][0] = sx(this);
            let _ = seed;
        });
    }

    // ------------------------------------------------------------------
    // Menus.

    #[test]
    fn menu_sys_bool_item_draw_matches() {
        check(ee::s09::f_002cc758__generated, menu_sys_bool_item_draw, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x2cc758, 1);
            let this = DATA + 0x100;
            clear(c, this, 0x200);
            c.bus.write32(this + 0x18, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(this + 0x1c, ((seed / 3) % 2) as u32);
            c.bus.write32(this + 0x20, DATA + 0x200);
            c.bus.write32(this + 0x14, r.next_u32());
            c.bus.write32(DATA + 0x200, ((seed / 6) % 2) as u32);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn menu_sys_submenu_item_ctor_matches() {
        check(ee::s09::f_002cccc8__generated, menu_sys_submenu_item_ctor, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x2cccc8);
            let this = DATA + 0x100;
            clear(c, this, 0x200);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn menu_sys_color_channel_item_ctor_matches() {
        check(ee::s09::f_002ced20__generated, menu_sys_color_channel_item_ctor, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x2ced20);
            let this = DATA + 0x100;
            clear(c, this, 0x200);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn menu_sys_color_swatch_item_ctor_matches() {
        check(ee::s09::f_002cff28__generated, menu_sys_color_swatch_item_ctor, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x2cff28);
            let this = DATA + 0x100;
            clear(c, this, 0x200);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn c_cut_scene_transition_man_force_on_matches() {
        check(ee::s09::f_002ea860__generated, c_cut_scene_transition_man_force_on, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x2ea860, 1);
            let this = DATA + 0x100;
            let app = DATA + 0x200;
            clear(c, this, 0x100);
            clear(c, app, 0x1200);
            c.bus.write32(this + 0x10, (seed % 2) as u32);
            c.bus.write32(app + 0x10d8, DATA + 0x300);
            vtable(c, DATA + 0x300, &[(0x148, 0, FAKE)]);
            c.bus.write32(gp_at(0x2a90), app);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn world_painter_get_irradiance_name2_matches() {
        check(ee::s09::f_002ef198__generated, world_painter_get_irradiance_name2, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x2ef198, 1);
            let painter = DATA + 0x100;
            let obj = DATA + 0x200;
            let vt = DATA + 0x300;
            clear(c, painter, 0x200);
            clear(c, obj, 0x200);
            clear(c, vt, 0x200);
            c.bus.write32(0x004f_a47c, painter);
            c.bus.write32(painter + 0x1c, obj);
            c.bus.write32(obj + 4, vt);
            vtable(c, vt, &[(0x138, 0, FAKE)]);
            c.bus.write32(gp_at(0xa74), ((seed % 2) as u32) * 1);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    // ------------------------------------------------------------------
    // App and strings.

    #[test]
    fn app_v02_matches() {
        check(ee::s10::f_00317400__generated, app_v02, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x317400, 2);
            let this = DATA + 0x100;
            let obj = DATA + 0x200;
            let vt = DATA + 0x300;
            clear(c, this, 0x100);
            clear(c, obj, 0x200);
            clear(c, vt, 0xf00);
            c.bus.write32(this + 8, if seed % 3 == 0 { 0 } else { obj });
            c.bus.write32(obj + 8, vt);
            vtable(c, vt, &[(0x18, 0, FAKE), (0x8, 0, FAKE + 0x10)]);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn c_bx_string_ctor_copy_matches() {
        check(ee::s10::f_00317fe8__generated, c_bx_string_ctor_copy, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x317fe8, 1);
            alloc_ready(c);
            let this = DATA + 0x100;
            let other = DATA + 0x200;
            let data = DATA + 0x300;
            clear(c, this, 0x40);
            clear(c, other, 0x40);
            clear(c, data - 0x10, 0x40);
            c.bus.write32(other, data);
            c.bus.write32(data - 0xc, if seed % 3 == 0 { 1 } else { 0xffff_ffff });
            c.bus.write8(data, b'a');
            c.bus.write8(data + 1, b'b');
            c.bus.write32(gp_at(0xda0), data + 0x20);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(other);
        });
    }

    #[test]
    fn c_bx_string_assign_cstr_matches() {
        check(ee::s10::f_003184e8__generated, c_bx_string_assign_cstr, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x3184e8, 2);
            alloc_ready(c);
            let this = DATA + 0x100;
            let data = DATA + 0x300;
            let text = DATA + 0x400;
            clear(c, this, 0x40);
            clear(c, data - 0x10, 0x40);
            clear(c, text, 0x40);
            c.bus.write32(this, data);
            c.bus.write32(data - 0xc, 1);
            c.bus.write32(data - 4, 0x40);
            c.bus.write8(text, b'h');
            c.bus.write8(text + 1, b'i');
            c.g[A0][0] = sx(this);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(text) };
        });
    }

    #[test]
    fn c_bx_string_append_cstr_matches() {
        check(ee::s10::f_00318910__generated, c_bx_string_append_cstr, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x318910, 2);
            alloc_ready(c);
            let this = DATA + 0x100;
            let data = DATA + 0x300;
            let text = DATA + 0x400;
            clear(c, this, 0x40);
            clear(c, data - 0x10, 0x80);
            clear(c, text, 0x40);
            c.bus.write32(this, data);
            c.bus.write32(data - 0xc, 1);
            c.bus.write32(data - 4, 0x20);
            c.bus.write8(data, b'a');
            c.bus.write8(text, b'h');
            c.bus.write8(text + 1, b'i');
            c.g[A0][0] = sx(this);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(text) };
        });
    }

    // ------------------------------------------------------------------
    // World and modifiers.

    #[test]
    fn crowd_man3d_remove_section_matches() {
        check(ee::s11::f_00343c08__generated, crowd_man3d_remove_section, 20, &|c, seed| {
            let mut r = common(c, seed, 0x343c08, 5);
            let this = DATA + 0x100;
            clear(c, this, 0x9c4 / 4);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
            let _ = seed;
        });
    }

    #[test]
    fn flex_bridge_save_matches() {
        check(ee::s11::f_00347d38__generated, flex_bridge_save, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x347d38, 3);
            let this = DATA + 0x100;
            let stream = DATA + 0x200;
            let vt = DATA + 0x300;
            clear(c, this, 0x200);
            clear(c, stream, 0x100);
            clear(c, vt, 0x200);
            c.bus.write32(stream, vt);
            vtable(c, vt, &[(0x8, 0, FAKE)]);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(stream);
            let _ = &mut r;
        });
    }

    #[test]
    fn dead_fade_update_matches() {
        check(ee::s11::f_00350c38__generated, dead_fade_update, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x350c38, 1);
            let this = DATA + 0x100;
            clear(c, this, 0x40);
            for off in [0x28u32, 0x2c, 0x30, 0x34] {
                let v = if (seed as u32 + off) % 3 == 0 { 0 } else { r.edge_f32() };
                c.bus.write32(this + off, v);
            }
            c.bus.write32(this + 0x18, DATA + 0x200);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn mod_obj_add_ava_spline_modifier_matches() {
        check(ee::s11::f_00355a78__generated, mod_obj_add_ava_spline_modifier, 20, &|c, seed| {
            let _ = common(c, seed, 0x355a78, 3);
            alloc_ready(c);
            let this = DATA + 0x100;
            clear(c, this, 0x200);
            c.bus.write32(this + 0x18, DATA + 0x2000);
            c.bus.write32(gp_at(0x9d4), if seed % 3 == 0 { 1 } else { 0 });
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn position_modifier_ctor_matches() {
        check(ee::s11::f_00356eb8__generated, position_modifier_ctor, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x356eb8);
            let this = DATA + 0x100;
            let pos = DATA + 0x200;
            clear(c, this, 0x100);
            put_words(c, pos, &[r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(pos);
        });
    }

    #[test]
    fn parent_modifier_ctor_matches() {
        check(ee::s11::f_00357038__generated, parent_modifier_ctor, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x357038);
            let this = DATA + 0x100;
            let pos = DATA + 0x200;
            clear(c, this, 0x100);
            put_words(c, pos, &[r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = sx(pos);
        });
    }

    // ------------------------------------------------------------------
    // Graphics.

    #[test]
    fn anim_teeter_dtor_matches() {
        check(ee::s12::f_00361400__generated, anim_teeter_dtor, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x361400, 2);
            alloc_ready(c);
            let this = heap_block(c, 0x100);
            clear(c, this, 0x40);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn floating_v29_matches() {
        check(ee::s12::f_00361800__generated, floating_v29, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x361800, 1);
            let this = DATA + 0x100;
            let vt = DATA + 0x300;
            clear(c, this, 0x100);
            clear(c, vt, 0x200);
            c.bus.write16(this + 0x12, (seed % 2) as u16);
            c.bus.write32(this + 0xc, vt);
            vtable(c, vt, &[(0x198, 0, FAKE)]);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64 & 0x3f;
        });
    }

    #[test]
    fn ps2_graphics_man_select_matching_display_mode_matches() {
        check(
            ee::s12::f_00376468__generated,
            ps2_graphics_man_select_matching_display_mode,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x376468, 2);
                let this = DATA + 0x100;
                let vt = DATA + 0x300;
                clear(c, this, 0x200);
                clear(c, vt, 0x500);
                c.bus.write32(this + 0x10d8, vt);
                vtable(c, vt, &[(0x3d8, 0, FAKE), (0x20, 0, FAKE + 0x10)]);
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn ps2_graphics_man_load_texture_matches() {
        check(ee::s12::f_0037caf8__generated, ps2_graphics_man_load_texture, 20, &|c, seed| {
            let _ = common(c, seed, 0x37caf8, 1);
            let this = DATA + 0x100;
            clear(c, this, 0x400);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 2) as u64;
            c.g[A2][0] = (seed % 3) as u64;
            c.g[A3][0] = (seed % 5) as u64;
        });
    }

    #[test]
    fn ps2_graphics_man_emit_model_matrix2_matches() {
        check(ee::s12::f_0037e040__generated, ps2_graphics_man_emit_model_matrix2, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x37e040);
            let m = DATA + 0x200;
            let pp = DATA + 0x180;
            clear(c, DATA, 0x400);
            put_words(c, m, &[r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]);
            c.bus.write32(pp, DATA + 0x300);
            c.g[A0][0] = sx(m);
            c.g[A1][0] = sx(pp);
        });
    }

    #[test]
    fn ps2_graphics_man_on_vif1_dma_end_matches() {
        check(ee::s12::f_00382688__generated, ps2_graphics_man_on_vif1_dma_end, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x382688, 1);
            let this = DATA + 0x100;
            clear(c, this, 0x5b00 / 4);
            c.bus.write32(this + 0x5a8c, (seed % 5) as u32);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn vfx_plants_get_sway_matrix_matches() {
        check(ee::s12::f_0038f2a8__generated, vfx_plants_get_sway_matrix, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x38f2a8);
            let pos = DATA + 0x100;
            clear(c, DATA, 0x200);
            c.bus.write32(pos, r.edge_f32());
            c.bus.write32(pos + 4, r.edge_f32());
            c.bus.write32(gp_at(0x1368), r.f32(100.0));
            c.g[A0][0] = sx(pos);
        });
    }

    // ------------------------------------------------------------------
    // UI.

    #[test]
    fn ui_list_ctor_matches() {
        check(ee::s13::f_003977e8__generated, ui_list_ctor, SEEDS, &|c, seed| {
            let _ = base(c, seed, 0x3977e8);
            let this = DATA + 0x100;
            clear(c, this, 0x40);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn ui_state_mgr_add_background_state_matches() {
        check(ee::s13::f_0039f238__generated, ui_state_mgr_add_background_state, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x39f238, 2);
            let this = DATA + 0x100;
            let state = DATA + 0x200;
            let vt = DATA + 0x300;
            clear(c, this, 0x100);
            clear(c, state, 0x100);
            clear(c, vt, 0x200);
            c.bus.write32(state + 8, vt);
            vtable(c, vt, &[(0x20, 0, FAKE)]);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(state);
        });
    }

    #[test]
    fn c_ps2_chunk_resolve_dtor_matches() {
        check(ee::s13::f_003aade8__generated, c_ps2_chunk_resolve_dtor, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x3aade8, 2);
            alloc_ready(c);
            let this = heap_block(c, 0x100);
            clear(c, this, 0x40);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    // ------------------------------------------------------------------
    // Riders, camera and front end.

    #[test]
    fn rider_on_mission_completed_matches() {
        check(ee::s00::f_0010f2d8__generated, rider_on_mission_completed, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x10f2d8, 4);
            let this = DATA + 0x100;
            clear(c, this, 0x900 / 4);
            c.bus.write32(this + 0x790, DATA + 0x2000);
            c.bus.write32(this + 0x86c, DATA + 0x3000);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn rider_on_item_collected_matches() {
        check(ee::s00::f_0010f338__generated, rider_on_item_collected, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x10f338, 4);
            let this = DATA + 0x100;
            clear(c, this, 0x900 / 4);
            c.bus.write32(this + 0x790, DATA + 0x2000);
            c.bus.write32(this + 0x86c, DATA + 0x3000);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn rider_board_press_sub_cancel_matches() {
        check(ee::s00::f_00131348__generated, rider_board_press_sub_cancel, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x131348, 4);
            let this = DATA + 0x100;
            let rider = DATA + 0x2000;
            clear(c, this, 0x100);
            clear(c, rider, 0x800 / 4);
            c.bus.write32(this + 0x14, rider);
            c.bus.write32(rider + 0x330, (seed % 4) as u32);
            c.bus.write32(rider + 0x790, DATA + 0x3000);
            c.g[A0][0] = sx(this);
            let _ = &mut r;
        });
    }

    #[test]
    fn be_new_player_int_restore_defaults_matches() {
        check(ee::s01::f_00145bd8__generated, be_new_player_int_restore_defaults, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x145bd8, 1);
            let this = DATA + 0x100;
            let vt = DATA + 0x300;
            clear(c, this, 0x100);
            clear(c, vt, 0x200);
            let _ = put_words(c, 0x0053_5b20, &[0; 6 * 7]);
            c.bus.write32(this + 0xc, vt);
            vtable(c, vt, &[(0x10, 0, FAKE)]);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn cam_controller_dtor_matches() {
        check(ee::s01::f_0015cc10__generated, cam_controller_dtor, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x15cc10, 2);
            alloc_ready(c);
            let this = heap_block(c, 0x100);
            clear(c, this, 0x40);
            c.bus.write32(this + 8, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn chase_cam_controller_set_default_algorithm_matches() {
        check(
            ee::s01::f_00161ef0__generated,
            chase_cam_controller_set_default_algorithm,
            SEEDS,
            &|c, seed| {
                let mut r = common(c, seed, 0x161ef0, 1);
                let this = DATA + 0x100;
                let cam = DATA + 0x200;
                let owner = DATA + 0x300;
                clear(c, this, 0x100);
                clear(c, cam, 0x100);
                clear(c, owner, 0x900 / 4);
                c.bus.write32(this + 0x20, cam);
                c.bus.write32(cam + 4, owner);
                c.bus.write32(this + 0x30, (seed % 9) as u32);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64 & !3;
            },
        );
    }

    #[test]
    fn direction_ne_chase_camera_algorithm_ctor_matches() {
        check(
            ee::s02::f_001774c0__generated,
            direction_ne_chase_camera_algorithm_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = common(c, seed, 0x1774c0, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (r.next_u32() % 64) as u64;
            },
        );
    }

    #[test]
    fn debug_menu_is_track_selectable_matches() {
        check(ee::s02::f_00179fc8__generated, debug_menu_is_track_selectable, SEEDS, &|c, seed| {
            let mut r = common(c, seed, 0x179fc8, 3);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = if seed % 2 == 0 {
                (r.next_u32() % 24) as u64
            } else {
                r.next_u32() as u64
            };
        });
    }

    #[test]
    fn lobby_menu_show_occupant_detail_matches() {
        check(ee::s02::f_0017e3b8__generated, lobby_menu_show_occupant_detail, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x17e3b8, 2);
            let this = DATA + 0x100;
            clear(c, this, 0x1600 / 4);
            c.bus.write32(this + 8, (seed % 3) as u32);
            c.g[A0][0] = sx(this);
            let _ = seed;
        });
    }

    #[test]
    fn keyboard_screen_cursor_down_matches() {
        check(ee::s02::f_00180768__generated, keyboard_screen_cursor_down, SEEDS, &|c, seed| {
            let _ = base(c, seed, 0x180768);
            let this = DATA + 0x100;
            clear(c, this, 0x200);
            c.bus.write32(this + 0x170, (seed % 48) as u32);
            c.bus.write32(this + 0xd8, ((seed / 48) % 2) as u32);
            c.bus.write32(this + 0xe0, ((seed / 96) % 2) as u32);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn c_fe_state_char_select_transition_in_matches() {
        check(ee::s02::f_001812a8__generated, c_fe_state_char_select_transition_in, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x1812a8, 3);
            let this = DATA + 0x100;
            clear(c, this, 0x100);
            c.bus.write8(this + 0x44, (seed % 12) as u8);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { seed as u64 + 1 };
        });
    }
}
