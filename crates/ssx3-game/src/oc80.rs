//! Phase 2 wave 19 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for fifty small game functions, each with a differential test
//! against the generated body.
//!
//! Groups:
//! - **Audio**: the gain-target/duck-gain setters, sound-bank teardown, the DJ
//!   speech queries, the section speech timer, crowd-audio instance destroy,
//!   the streaming-bank resume and the PS2 memcard flush.
//! - **Script engine**: the process list forwarder, object-script runner, the
//!   list delete-all loop and the mission-start request.
//! - **Objects / world**: the forward-diff cache, crowd group load, flex rail
//!   and flag-manager updates, anim-node and modifier destructors, the
//!   interrupt table, texture loading and the light manager.
//! - **UI / streaming**: list-box entry lookup, UI node/thread destructors,
//!   the stream-manager buffer free, section table and chunk-resolve
//!   destructors, the AV player constructor.
//! - **Riders / camera / FE**: the focus box, the render-slot query, the
//!   camera algorithm setter, camera-trigger header readers, the debug menu,
//!   and the front-end state callbacks and constructors.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_87a8,
        ssx_audio_system_set_gain_target,
        "SSXAudioSystem_SetGainTarget",
    ),
    (
        0x0028_9830,
        ssx_audio_system_set_duck_gain,
        "SSXAudioSystem_SetDuckGain",
    ),
    (0x0028_b2d0, snd_bank_unload_linked, "SndBank_UnloadLinked"),
    (0x0028_b7e0, snd_bank_unload_memory, "SndBank_UnloadMemory"),
    (
        0x0029_f0d8,
        ssx_audio_system_is_dj_speech_on,
        "SSXAudioSystem_IsDJSpeechOn",
    ),
    (
        0x002a_49e8,
        ssx_audio_system_start_section_speech_timer,
        "SSXAudioSystem_StartSectionSpeechTimer",
    ),
    (
        0x002a_4e38,
        crowd_audio_destroy_instance,
        "CrowdAudio_DestroyInstance",
    ),
    (0x002a_7890, ini_parser_dtor, "IniParser_dtor"),
    (
        0x002a_cb30,
        bank_monitor_instance_resume,
        "BankMonitorInstance_Resume",
    ),
    (0x002c_6b00, c_ps2_mem_card_do_flush, "cPS2MemCard_DoFlush"),
    (0x002c_6d70, loc_string_table_dtor, "LocStringTable_dtor"),
    (0x002d_9660, avalanche_bake_group, "Avalanche_BakeGroup"),
    (
        0x0030_7a10,
        wscript_process_forward_event_to_task,
        "WScriptProcess_ForwardEventToTask",
    ),
    (
        0x0030_a610,
        wscript_man_run_obj_script0,
        "WScriptMan_RunObjScript0",
    ),
    (
        0x0030_b1b8,
        wscript_process_list_delete_all,
        "WScriptProcessList_DeleteAll",
    ),
    (
        0x0030_b540,
        wscript_man_request_mission_start,
        "WScriptMan_RequestMissionStart",
    ),
    (
        0x0033_4800,
        nash_create_fwd_diff_cache,
        "Nash_CreateFwdDiffCache",
    ),
    (0x0034_4ff8, crowd_man3d_group_load, "CrowdMan3d_GroupLoad"),
    (0x0034_8008, flex_rail_update, "FlexRail_Update"),
    (0x0034_c898, c_flag_manager_load, "cFlagManager_Load"),
    (0x0034_ee68, floating_v01, "Floating_v01"),
    (0x0035_1120, instances_man_dtor, "InstancesMan_dtor"),
    (
        0x0035_3228,
        modifier_block_reset_particles,
        "ModifierBlock_ResetParticles",
    ),
    (
        0x0035_3db8,
        flex_bridge_springs_dtor,
        "FlexBridge_SpringsDtor",
    ),
    (
        0x0036_0c98,
        modifier_list_node_dtor,
        "ModifierListNode_dtor",
    ),
    (
        0x0036_0d40,
        rail_modifier_list_node_dtor,
        "RailModifierListNode_dtor",
    ),
    (
        0x0036_2008,
        intc_table_remove_handler,
        "IntcTable_RemoveHandler",
    ),
    (0x0036_7cd0, tex_man_lock_texture, "TexMan_LockTexture"),
    (
        0x0037_c720,
        ps2_graphics_man_set_brightness_alpha,
        "PS2GraphicsMan_SetBrightnessAlpha",
    ),
    (0x0038_dee8, light_man_v09, "LightMan_v09"),
    (
        0x0039_4c98,
        light_man_init_instances,
        "LightMan_InitInstances",
    ),
    (
        0x0039_9920,
        ui_list_box_get_mode_entry,
        "UIListBox_GetModeEntry",
    ),
    (0x003a_47f8, ui_list_node_dtor, "UIListNode_dtor"),
    (0x003a_5b48, ui_thread_dtor, "UIThread_dtor"),
    (
        0x003a_6f38,
        c_stream_man_free_buffers,
        "cStreamMan_FreeBuffers",
    ),
    (0x003a_7d80, section_table_dtor, "SectionTable_dtor"),
    (
        0x003a_ad98,
        c_ps2_chunk_resolve_create,
        "cPS2ChunkResolve_Create",
    ),
    (0x003a_de08, av_player_ctor_2, "AV_Player_ctor_2"),
    (0x0011_bb40, rider_create_focus_box, "Rider_CreateFocusBox"),
    (0x0011_c0e0, rider_has_render_slot, "Rider_HasRenderSlot"),
    (
        0x0015_cf28,
        cam_controller_set_algorithm,
        "CamController_SetAlgorithm",
    ),
    (
        0x0016_c3d0,
        camera_trigger_man_read_header,
        "CameraTriggerMan_ReadHeader",
    ),
    (
        0x0017_15a0,
        camera_trigger_read_action_type0,
        "CameraTrigger_ReadActionType0",
    ),
    (0x0017_fb98, debug_menu_module_v06, "DebugMenuModule_v06"),
    (
        0x0018_3550,
        c_fe_state_rider_detail_on_render,
        "cFEStateRiderDetail_OnRender",
    ),
    (
        0x0019_4440,
        c_fe_state_bragging_rights_on_child_event_quit,
        "cFEStateBraggingRights_OnChildEventQuit",
    ),
    (
        0x0019_4988,
        c_fe_state_title_on_widget_created,
        "cFEStateTitle_OnWidgetCreated",
    ),
    (0x001a_3590, fe_async_manager_ctor, "FEAsyncManager_ctor"),
    (
        0x001a_87d0,
        net_fe_state_on_activate,
        "NetFEState_OnActivate",
    ),
    (
        0x001a_c010,
        c_fe_use_existing_account_ctor,
        "cFEUseExistingAccount_ctor",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const SET_FADE_GAIN: Callee = Callee {
        addr: 0x0028_7a10,
        f: ee::s07::SSXAudioSystem_SetFadeGain,
    };
    pub const SND_BANK_UNLOAD: Callee = Callee {
        addr: 0x0028_b528,
        f: ee::s07::SndBank_Unload,
    };
    pub const BANK_MONITOR_ADD_TIMER: Callee = Callee {
        addr: 0x002a_dca0,
        f: ee::s08::BankMonitor_AddTimer,
    };
    pub const CROWD_AUDIO_INSTANCE_STOP_SOUND: Callee = Callee {
        addr: 0x002a_6848,
        f: ee::s08::CrowdAudio_InstanceStopSound,
    };
    pub const CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES: Callee = Callee {
        addr: 0x002a_6430,
        f: ee::s08::CrowdAudio_InstanceRemoveAllSources,
    };
    pub const BANK_MONITOR_INSTANCE_POLL_FREE: Callee = Callee {
        addr: 0x002a_bb80,
        f: ee::s08::BankMonitorInstance_PollFree,
    };
    pub const BANK_MONITOR_INSTANCE_SET_PITCH: Callee = Callee {
        addr: 0x002a_be78,
        f: ee::s08::BankMonitorInstance_SetPitch,
    };
    pub const CPS2_MEMCARD_SET_STATE: Callee = Callee {
        addr: 0x002c_48c0,
        f: ee::s08::cPS2MemCard_SetState,
    };
    pub const AVALANCHE_BAKE_NODE_HISTORY: Callee = Callee {
        addr: 0x002d_66a0,
        f: ee::s09::Avalanche_BakeNodeHistory,
    };
    pub const WSCRIPT_MAN_CALL_FUNC: Callee = Callee {
        addr: 0x0030_9c88,
        f: ee::s10::WScriptMan_CallFunc,
    };
    pub const CROWD_MAN3D_GROUP_FREE_MESHES: Callee = Callee {
        addr: 0x0034_4800,
        f: ee::s11::CrowdMan3d_GroupFreeMeshes,
    };
    pub const CROWD_MAN3D_GROUP_CREATE_MESHES: Callee = Callee {
        addr: 0x0034_43a8,
        f: ee::s11::CrowdMan3d_GroupCreateMeshes,
    };
    pub const FLEX_RAIL_UPDATE_SPACE_NODE: Callee = Callee {
        addr: 0x0034_8058,
        f: ee::s11::FlexRail_UpdateSpaceNode,
    };
    pub const C_FLAG_MANAGER_GROUP_LOAD: Callee = Callee {
        addr: 0x0034_c378,
        f: ee::s11::cFlagManager_GroupLoad,
    };
    pub const FLOATING_V49: Callee = Callee {
        addr: 0x0034_f048,
        f: ee::s11::Floating_v49,
    };
    pub const MOD_OBJ_DTOR: Callee = Callee {
        addr: 0x0035_53c0,
        f: ee::s11::ModObj_dtor,
    };
    pub const MODIFIER_LIST_UNLINK: Callee = Callee {
        addr: 0x0034_5760,
        f: ee::s11::ModifierList_Unlink,
    };
    pub const RAIL_MODIFIER_LIST_UNLINK: Callee = Callee {
        addr: 0x0035_b6d0,
        f: ee::s11::RailModifier_ListUnlink,
    };
    pub const DISABLE_INTC: Callee = Callee {
        addr: 0x0042_4880,
        f: ee::s15::DisableIntc,
    };
    pub const REMOVE_INTC_HANDLER: Callee = Callee {
        addr: 0x0042_3aa0,
        f: ee::s15::RemoveIntcHandler,
    };
    pub const TEX_MAN_LOAD_TEXTURE: Callee = Callee {
        addr: 0x0036_7440,
        f: ee::s12::TexMan_LoadTexture,
    };
    pub const LIGHT_MAN_EMPTY_HOOK_A: Callee = Callee {
        addr: 0x0038_dc50,
        f: ee::s12::LightMan_EmptyHookA,
    };
    pub const UI_OBJECT_GET_STATE_DATA: Callee = Callee {
        addr: 0x0039_ff50,
        f: ee::s13::UIObject_GetStateData,
    };
    pub const UI_LIST_NODE_DTOR_BODY: Callee = Callee {
        addr: 0x003a_4868,
        f: ee::s13::UIListNode_dtorBody,
    };
    pub const CPS2_CHUNK_RESOLVE_CTOR: Callee = Callee {
        addr: 0x003a_a2f0,
        f: ee::s13::cPS2ChunkResolve_ctor,
    };
    pub const AV_PLAYER_INIT: Callee = Callee {
        addr: 0x003a_dec8,
        f: ee::s13::AV_Player_Init,
    };
    pub const FOCUS_MAN_ADD_FOCUS_BOX: Callee = Callee {
        addr: 0x0010_33b0,
        f: ee::s00::FocusMan_AddFocusBox,
    };
    pub const CAM_CONTROLLER_CLEAR_ALGORITHMS: Callee = Callee {
        addr: 0x0015_cd60,
        f: ee::s01::CamController_ClearAlgorithms,
    };
    pub const ALGORITHM_LIST_PUSH: Callee = Callee {
        addr: 0x0015_c988,
        f: ee::s01::AlgorithmList_Push,
    };
    pub const APP_MODULE_UPDATE_FADE: Callee = Callee {
        addr: 0x0023_1d18,
        f: ee::s06::AppModule_UpdateFade,
    };
    pub const DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL: Callee = Callee {
        addr: 0x0017_f2d0,
        f: ee::s02::DebugMenuModule_UpdateCursorTrail,
    };
    pub const UI_OBJECT_SHOW_CLAMP_ALPHA_HALF: Callee = Callee {
        addr: 0x0019_4498,
        f: ee::s02::UIObject_ShowClampAlphaHalf,
    };
    pub const C_SETUP_MAN_SEND_QUIT: Callee = Callee {
        addr: 0x0026_5768,
        f: ee::s07::cSetupMan_SendQuit,
    };
    pub const C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER: Callee = Callee {
        addr: 0x001a_94d8,
        f: ee::s03::cFEStateOnlineCharacterSelect_EmptyChildHandler,
    };
    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };
    pub const UI_OBJECT_SET_USER_DATA: Callee = Callee {
        addr: 0x003a_0330,
        f: ee::s13::UIObject_SetUserData,
    };
    pub const EA_FILE_MOUNT_BIG: Callee = Callee {
        addr: 0x003d_ed50,
        f: ee::s14::EAFile_MountBig,
    };
    pub const NET_FE_STATE_CTOR: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::NetFEState_ctor,
    };
    pub const EA_MEM_ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::EA_MemAllocNamed,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
mod table {
    pub const IS_REPLAY_PLAYING: u32 = 0x0028_8ae0;
    pub const GET_CHANNEL_VOLUME: u32 = 0x0028_7920;
    pub const SND_BANK_REMOVE: u32 = 0x003b_6300;
    pub const SND_MIDI_RESUME: u32 = 0x003b_9dc8;
    pub const SCE_MC_FLUSH: u32 = 0x0040_ab80;
    pub const BUILTIN_DELETE: u32 = 0x0031_7e50;
    pub const BUILTIN_VEC_DELETE: u32 = 0x0031_7e98;
    pub const MEM_FREE: u32 = 0x0025_23a8;
    pub const WORLD_GET_OBJECT_SCRIPT_FUNC: u32 = 0x003a_6b78;
    pub const WSCRIPT_PROCESS_LIST_POP_FRONT: u32 = 0x0030_b208;
    pub const STR_HASH: u32 = 0x0031_7670;
    pub const SERIALIZE_READ_U32: u32 = 0x002d_2510;
    pub const SERIALIZE_READ_U32_2: u32 = 0x002d_2558;
    pub const BXFILE_EXISTS: u32 = 0x0031_7f38;
    pub const STREAM_FILE_DESTROY: u32 = 0x003e_0a28;
    pub const UI_SCREEN_FIND_OBJECT: u32 = 0x0039_d860;
}

/// `gp`-relative global (`lw(gp + off)` / `sw(gp + off, v)`).
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

// ---------------------------------------------------------------------------
// Audio.

/// `void SSXAudioSystem_SetGainTarget(sAudioFade* self, int i, int smooth,
/// float target, float time)` (0x2887a8): store the gain target and, when it
/// differs from the current gain, derive the ramp rate from the time (a
/// `smooth` of 0 leaves the rate alone). Equal target zeroes the rate.
pub fn ssx_audio_system_set_gain_target(c: &mut Cpu) {
    c.cycles += 10;
    let this = r32(c, A0);
    c.g[A3][0] = c.g[A0][0];
    // self + i*4 (the per-channel slots).
    let ch = this.wrapping_add(r32(c, A1) << 2);
    c.g[V0][0] = sx(ch);
    c.swc1(12, p(ch, 0x6440)); // target[i]
    c.g[V1][0] = sx(ch);
    c.lwc1(0, p(ch, 0x63e4)); // cur[i]
    c.c_eq_s(12, 0);
    if c.fcc() {
        Addr(p(ch, 0x6414)).sw(c, 0); // rate[i] = 0
    } else {
        c.cycles += 2;
        if c.g[A2][0] != 0 {
            c.cycles += 6;
            c.sub_s(0, 12, 0); // target - cur
            c.lwc1(1, gp(c, -0x4614)); // 1/60
            c.div_s(0, 0, 13); // / time
            c.mul_s(0, 0, 1); // * (1/60)
            c.swc1(0, p(ch, 0x6414));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SSXAudioSystem_SetDuckGain(void* self, float v)` (0x289830): set the
/// duck gain at `+0x6c5c` through the fade system when it changed.
pub fn ssx_audio_system_set_duck_gain(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        c.lwc1(0, p(this, 0x6c5c));
        c.c_eq_s(0, 20);
        if !c.fcc() {
            c.cycles += 5;
            c.mtc1(0, 13); // fade time 0.0
            c.g[A1][0] = 0x1;
            callee::SET_FADE_GAIN.call(c, 0x0028_9868)?;
            c.swc1(20, p(this, 0x6c5c));
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void SndBank_UnloadLinked(SndBank* self)` (0x28b2d0): walk the list of
/// banks linked from `+0x18` (terminated by `self`) and unload each.
pub fn snd_bank_unload_linked(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let list = c.g[A0][0];
        c.g[S1][0] = list;
        frame.save_ra(c, 0);
        let mut p = Addr(list as u32).off(0x18).lw(c);
        c.g[S0][0] = p;
        let mut only_sentinel = false;
        if p != 0 {
            c.cycles += 2;
            if p == list {
                // the head is already the sentinel: nothing to unload
                only_sentinel = true;
            } else {
                loop {
                    c.cycles += 5;
                    c.g[A0][0] = p;
                    callee::SND_BANK_UNLOAD.call(c, 0x0028_b300)?;
                    c.g[S0][0] = Addr(p as u32).off(0x18).lw(c);
                    p = c.g[S0][0];
                    if p == list {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0028_b2f8);
                    }
                }
            }
        }
        if !only_sentinel {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SndBank_UnloadMemory(SndBank* self)` (0x28b7e0): detach the bank from
/// the sound system and clear the memory/state fields.
pub fn snd_bank_unload_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = u64::MAX; // -1: no bank
        Addr(this).sw(c, 0);
        c.g[A0][0] = Addr(this).off(4).lw(c);
        if c.g[A0][0] != c.g[S1][0] {
            c.cycles += 4;
            call_ptr(c, table::SND_BANK_REMOVE, 0x0028_b810)?;
            Addr(this).off(4).sw(c, c.g[S1][0]);
            Addr(this).off(0x14).sw(c, 0);
        } else {
            Addr(this).off(0x14).sw(c, 0);
        }
        c.cycles += 6;
        Addr(this).off(0x10).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool SSXAudioSystem_IsDJSpeechOn(void* self)` (0x29f0d8): true when the
/// replay is not playing and the DJ speech channel's volume is non-zero.
pub fn ssx_audio_system_is_dj_speech_on(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, table::IS_REPLAY_PLAYING, 0x0029_f0ec)?;
        let mut on = 0u64;
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, table::GET_CHANNEL_VOLUME, 0x0029_f100)?;
            c.mtc1(0, 1); // 0.0
            c.c_eq_s(0, 1);
            if !c.fcc() {
                c.cycles += 1;
                on = 0x1;
            }
        }
        c.g[V0][0] = on;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StartSectionSpeechTimer(void* self)` (0x2a49e8):
/// mark the section speech active and arm the bank monitor's 30 s timer.
pub fn ssx_audio_system_start_section_speech_timer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x10);
        let this = r32(c, A0);
        c.g[V0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[V1][0] = 0x4a_0000; // the timer constant's page
        Addr(p(this, 0x5818)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x7530; // 30000 ms
        c.g[V0][0] = Addr(p(this, 0x118)).lw(c);
        c.g[A3][0] = 0;
        // The 8-byte timer argument (`D_004A36F0`): an unaligned `ld` pair.
        c.ldl(A2, 0x004a_36f7);
        c.ldr(A2, 0x004a_36f0);
        c.g[T0][0] = 0;
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d8));
        callee::BANK_MONITOR_ADD_TIMER.call(c, 0x002a_4a2c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdAudio_DestroyInstance(CrowdAudio* self, int flags)`
/// (0x2a4e38): stop the instance's sound, drop its sources, and delete it
/// when `flags & 1`.
pub fn crowd_audio_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::CROWD_AUDIO_INSTANCE_STOP_SOUND.call(c, 0x002a_4e54)?;
        c.mtc1(0, 12); // fade out over 0.0
        c.g[A0][0] = c.g[S1][0];
        callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES.call(c, 0x002a_4e60)?;
        c.g[S0][0] &= 1;
        let owned = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if owned {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x002a_4e74)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void IniParser_dtor(IniParser* self, int flags)` (0x2a7890): free the
/// owned buffer and, when `flags & 1`, the parser itself.
pub fn ini_parser_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(this).off(4).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, table::BUILTIN_VEC_DELETE, 0x002a_78b8)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x002a_78cc)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitorInstance_Resume(void* self)` (0x2acb30): resume a paused
/// bank monitor instance and restart its MIDI slot.
pub fn bank_monitor_instance_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::BANK_MONITOR_INSTANCE_POLL_FREE.call(c, 0x002a_cb44)?;
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x1000;
            callee::BANK_MONITOR_INSTANCE_SET_PITCH.call(c, 0x002a_cb58)?;
            c.g[V1][0] = Addr(this).lw(c);
            c.g[V0][0] = 0x2;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = Addr(this).off(4).lw(c);
                call_ptr(c, table::SND_MIDI_RESUME, 0x002a_cb70)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoFlush(void* self)` (0x2c6b00): flush the memcard and
/// move the object to the done (or error) state.
pub fn c_ps2_mem_card_do_flush(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        call_ptr(c, table::SCE_MC_FLUSH, 0x002c_6b18)?;
        let ok = (c.g[V0][0] as i64) >= 0;
        Addr(gp(c, 0x850)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        if ok {
            c.cycles += 4;
            c.g[A1][0] = 0x48;
            callee::CPS2_MEMCARD_SET_STATE.call(c, 0x002c_6b40)?;
        } else {
            c.cycles += 5;
            c.g[A1][0] = 0x1;
            callee::CPS2_MEMCARD_SET_STATE.call(c, 0x002c_6b2c)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LocStringTable_dtor(LocStringTable* self, int flags)`
/// (0x2c6d70): free the owned table and, when `flags & 1`, the object.
pub fn loc_string_table_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(this).off(4).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, table::MEM_FREE, 0x002c_6d98)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x002c_6dac)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Avalanche_BakeGroup(AvalancheGroup* self)` (0x2d9660): bake the
/// history of every node linked from `+0x4` (through `+0xf4`).
pub fn avalanche_bake_group(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = Addr(this).off(4).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                callee::AVALANCHE_BAKE_NODE_HISTORY.call(c, 0x002d_9690)?;
                c.g[S0][0] = Addr(r32(c, S0)).off(0xf4).lw(c);
                let more = c.g[S0][0] != 0;
                c.g[A0][0] = c.g[S0][0];
                if !more {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9688);
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `void WScriptProcess_ForwardEventToTask(WScriptProcList* self, int event)`
/// (0x307a10): forward an event to the task object of the process list's
/// current entry (entries are 0x44-byte records, their vtable slot at +0x40).
pub fn wscript_process_forward_event_to_task(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let this = r32(c, A0);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(p(this, 8)).lw(c);
        c.g[V0][0] = Addr(p(this, 4)).lw(c);
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let none = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 10;
            c.g[A0][0] = 0x44;
            c.g[V1][0] = Addr(p(this, 0x14)).lw(c);
            c.mult(0, A0, A2, A0);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            c.g[V0][0] = Addr(r32(c, A0)).off(0x40).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x18).lh(c);
            c.g[A2][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
            let target = r32(c, A2);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            call_ptr(c, target, 0x0030_7a54)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.pc = r32(c, RA);
        Ok(())
    })
}

/// `void WScriptMan_RunObjScript0(WScriptMan* self, int id)` (0x30a610): set
/// the running id, look the object script up in the script cache and run it
/// when it resolved.
pub fn wscript_man_run_obj_script0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p(this, 0x290)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x0030_a634);
        c.g[A0][0] = Addr(p(this, 0x28c)).lw(c);
        call_ptr(c, table::WORLD_GET_OBJECT_SCRIPT_FUNC, 0x0030_a634)?;
        c.g[V1][0] = !(0u64 | c.g[V0][0]);
        let found = c.g[V1][0] != 0;
        Addr(r32(c, SP)).sw(c, c.g[V0][0]);
        if found {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0030_a64c);
            c.g[A1][0] = c.g[SP][0];
            callee::WSCRIPT_MAN_CALL_FUNC.call(c, 0x0030_a64c)?;
        }
        c.cycles += 5;
        Addr(p(this, 0x290)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptProcessList_DeleteAll(WScriptProcessList* self)`
/// (0x30b1b8): pop processes and delete each until the list is empty. The
/// loop head (0x30b1d0) is a resume entry: an event can unwind the thread
/// with the popped process still in `a2`.
pub fn wscript_process_list_delete_all(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        let list: u32;
        let mut pending: u64;
        // 0x30b1e4 pops the next process. The entry starts there (after the
        // prologue); the loop head 0x30b1d0 is entered with a process already
        // in `a2` (also the resume point of an unwound event).
        let mut need_pop;
        if c.pc == 0x0030_b1d0 {
            list = r32(c, S0);
            pending = c.g[A2][0];
            need_pop = false;
        } else if c.pc == 0x0030_b1b8 {
            c.cycles += 5;
            c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x20));
            frame.save(c, S0, 0x10);
            frame.save_ra(c, 0);
            list = r32(c, A0);
            c.g[S0][0] = c.g[A0][0];
            pending = 0;
            need_pop = true;
        } else {
            c.jump_out(c.pc, 0x0030_b1b8, 0x0030_b208);
            return Ok(());
        }
        loop {
            if need_pop {
                // 0x30b1e4: pop the next process, then check for an event.
                c.cycles += 5;
                c.g[RA][0] = sx(0x0030_b1ec);
                c.g[A0][0] = sx(list);
                call_ptr(c, table::WSCRIPT_PROCESS_LIST_POP_FRONT, 0x0030_b1ec)?;
                c.g[A2][0] = c.g[V0][0];
                pending = c.g[V0][0];
                c.g[A1][0] = 0x3;
                if pending == 0 {
                    break;
                }
                if event_due(c) && crate::events::deliver_event(c, 0x0030_b1d0) {
                    return Ok(());
                }
                need_pop = false;
            }
            // 0x30b1d0: run the process's destructor through its vtable.
            c.cycles += 5;
            let obj = pending as u32;
            c.g[V0][0] = Addr(obj).off(0x5c).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(8).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0030_b1e4);
            c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0030_b1e4)?;
            need_pop = true;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_RequestMissionStart(WScriptMan* self)` (0x30b540): stop
/// playback and the replay, then arm the mission-start request.
pub fn wscript_man_request_mission_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, V1)).off(0x84).lw(c);
        c.g[RA][0] = sx(0x0030_b560);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x34).lw(c);
        call_ptr(c, 0x0026_cdf8, 0x0030_b560)?;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x84).lw(c);
        c.g[RA][0] = sx(0x0030_b570);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x28).lw(c);
        call_ptr(c, 0x0026_f228, 0x0030_b570)?;
        c.g[V1][0] = Addr(p(this, 0x2a0)).lw(c);
        c.g[V0][0] = 0x1;
        Addr(p(this, 0x2ac)).sw(c, c.g[V0][0]);
        Addr(p(this, 0x2b0)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Objects and world.

/// `void Nash_CreateFwdDiffCache(void* self)` (0x334800): create the rider's
/// forward-difference cache and initialise it with 0x28 entries.
pub fn nash_create_fwd_diff_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x48_e4f8; // D_0048E4F8, the allocation name
        frame.save(c, S0, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x10;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0033_4828)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0032_7810, 0x0033_4830)?;
        Addr(p(this, 0xa4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x28;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0032_7890, 0x0033_4840)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdMan3d_GroupLoad(CrowdMan3dGroup* self, void* stream)`
/// (0x344ff8): let the group's stream read its chunk (0x10c bytes), then
/// rebuild the group's meshes.
pub fn crowd_man3d_group_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        let stream = r32(c, A1);
        frame.save(c, S0, 0x10);
        c.g[A3][0] = c.g[A1][0];
        c.g[A2][0] = 0x10c;
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(stream).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x10).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x14).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0034_5028)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CROWD_MAN3D_GROUP_FREE_MESHES.call(c, 0x0034_5030)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CROWD_MAN3D_GROUP_CREATE_MESHES.call(c, 0x0034_5038)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FlexRail_Update(FlexRail* self)` (0x348008): run the two per-node
/// virtual updates, then re-place the space node.
pub fn flex_rail_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(this, 0xc)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x178).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x17c).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0034_802c)?;
        c.g[V0][0] = Addr(p(this, 0xc)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x180).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x184).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0034_8040)?;
        c.g[A0][0] = c.g[S0][0];
        callee::FLEX_RAIL_UPDATE_SPACE_NODE.call(c, 0x0034_8048)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFlagManager_Load(cFlagManager* self, void* arg)` (0x34c898): load
/// the 15 flag groups that follow the header.
pub fn c_flag_manager_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let mut group = r32(c, A0).wrapping_add(0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(group);
        c.g[S1][0] = 0xe;
        c.g[S2][0] = c.g[A1][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = sx(group);
            c.g[RA][0] = sx(0x0034_c8c4);
            c.g[A1][0] = c.g[S2][0];
            callee::C_FLAG_MANAGER_GROUP_LOAD.call(c, 0x0034_c8c4)?;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            group = r32(c, S0).wrapping_add(0x188);
            c.g[S0][0] = sx(group);
            if !more {
                c.cycles += 6;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_c8b8);
            }
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Floating_v01(Floating* self, int flags)` (0x34ee68): the anim
/// node's destructor: install the vtable, run the subclass teardown, free
/// the owned block and the base object.
pub fn floating_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_f858; // D_0048F858
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        callee::FLOATING_V49.call(c, 0x0034_ee90)?;
        c.g[A0][0] = Addr(p(this, 0x74)).lw(c);
        call_ptr(c, table::BUILTIN_DELETE, 0x0034_ee98)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::MOD_OBJ_DTOR.call(c, 0x0034_eea4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InstancesMan_dtor(InstancesMan* self, int flags)` (0x351120): free
/// the instance table and, when `flags & 1`, the object.
pub fn instances_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(p(this, 0xc)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, table::BUILTIN_VEC_DELETE, 0x0035_1148)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0035_115c)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ModifierBlock_ResetParticles(ModifierBlock* self)` (0x353228):
/// call the third virtual method of every modifier link.
pub fn modifier_block_reset_particles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = Addr(r32(c, A1)).off(8).lw(c);
            loop {
                c.cycles += 8;
                c.g[S0][0] = Addr(r32(c, A1)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).off(0x18).lh(c);
                c.g[V1][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0035_325c)?;
                c.g[A1][0] = c.g[S0][0];
                if c.g[A1][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V0][0] = Addr(r32(c, A1)).off(8).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_3248);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void FlexBridge_SpringsDtor(FlexBridge* self, int flags)` (0x353db8):
/// free the spring array and, when `flags & 1`, the object.
pub fn flex_bridge_springs_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(p(this, 0x24)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, table::BUILTIN_VEC_DELETE, 0x0035_3de0)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0035_3df4)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ModifierListNode_dtor(ModifierListNode* self, int flags)`
/// (0x360c98): install the vtable, unlink the node and maybe delete it.
pub fn modifier_list_node_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_1340; // D_00491340
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        callee::MODIFIER_LIST_UNLINK.call(c, 0x0036_0cc0)?;
        c.g[S0][0] &= 1;
        let owned = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if owned {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0036_0cd4)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RailModifierListNode_dtor(RailModifierListNode* self, int flags)`
/// (0x360d40): install the vtable, unlink the node and maybe delete it.
pub fn rail_modifier_list_node_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_1200; // D_00491200
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        callee::RAIL_MODIFIER_LIST_UNLINK.call(c, 0x0036_0d68)?;
        c.g[S0][0] &= 1;
        let owned = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if owned {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0036_0d7c)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* IntcTable_RemoveHandler(IntcTable* self, int handler)`
/// (0x362008): disable the interrupt, clear its table slot and return the
/// old handler.
pub fn intc_table_remove_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        let handler = r32(c, A1);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::DISABLE_INTC.call(c, 0x0036_2028)?;
        c.g[V1][0] = sx(handler << 2);
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(this.wrapping_add(r32(c, V1)));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
        callee::REMOVE_INTC_HANDLER.call(c, 0x0036_2044)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void TexMan_LockTexture(int a0, int a1, int a2, int a3)` (0x367cd0): load
/// a texture with the renderer's default lock parameters.
pub fn tex_man_lock_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[T0][0] = c.g[A2][0];
        c.g[T2][0] = c.g[A3][0];
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = u64::MAX;
        c.g[A3][0] = c.g[A1][0];
        c.g[V0][0] = 0x3;
        c.g[A2][0] = 0x4a_4068; // D_004A4068
        frame.save_ra(c, 0x20);
        Addr(frame.at(c, 8)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0;
        Addr(frame.at(c, 0x18)).sw(c, c.g[V1][0]);
        Addr(frame.at(c, 0)).sw(c, 0);
        c.g[T1][0] = 0;
        Addr(frame.at(c, 0x10)).sw(c, 0);
        c.g[RA][0] = sx(0x0036_7d14);
        c.g[T3][0] = 0;
        callee::TEX_MAN_LOAD_TEXTURE.call(c, 0x0036_7d14)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2GraphicsMan_SetBrightnessAlpha(void* self, float a)`
/// (0x37c720): store the brightness ramp for `1 - clamp(a, 1)`, scaled to
/// 128 steps.
pub fn ps2_graphics_man_set_brightness_alpha(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.mtc1(0, 0); // f0 = 0
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.c_le_s(0, 12); // 0 <= a
        frame.save_ra(c, 0);
        c.g[AT][0] = 0x3f80_0000; // 1.0
        c.mtc1(1, 1);
        let below = !c.fcc();
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        if !below {
            c.cycles += 1;
            c.min_s(0, 12, 1);
        }
        c.cycles += 10;
        c.sub_s(0, 1, 0); // 1 - clamp
        c.g[AT][0] = 0x4300_0000; // 128.0
        c.mtc1(1, 12);
        c.g[RA][0] = sx(0x0037_c75c);
        c.mul_s(12, 0, 12);
        call_ptr(c, 0x0041_39f8, 0x0037_c75c)?;
        Addr(p(this, 0x59f8)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LightMan_v09(void* obj, int a, int arg, float f)` (0x38dee8): resolve
/// the object's light hook and forward the call.
pub fn light_man_v09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x20);
        c.mov_s(20, 12);
        c.g[A2][0] = Addr(this).lw(c);
        c.g[V1][0] = Addr(r32(c, A2)).off(0x10).lh(c);
        c.g[V0][0] = Addr(r32(c, A2)).off(0x14).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, V1)));
        call_ptr(c, target, 0x0038_df14)?;
        c.g[A1][0] = c.g[S0][0];
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[V0][0];
        callee::LIGHT_MAN_EMPTY_HOOK_A.call(c, 0x0038_df24)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void LightMan_InitInstances(LightMan* self, int n)` (0x394c98): allocate
/// the `n + 0x32` instance table.
pub fn light_man_init_instances(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[V1][0] = sx(r32(c, V0).wrapping_add(0x32));
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p(this, 0xc)).sw(c, 0);
        c.g[A1][0] = 0x49_1fa0; // D_00491FA0
        Addr(p(this, 0x10)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(r32(c, V1) << 2);
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x0039_4cd4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0039_4cd4)?;
        Addr(p(this, 4)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI and streaming.

/// `int UIListBox_GetModeEntry(UIListBox* self)` (0x399920): read the entry
/// table of the list box's state data for the current mode (2-bit index).
pub fn ui_list_box_get_mode_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_GET_STATE_DATA.call(c, 0x0039_9934)?;
        c.g[A0][0] = c.g[V0][0];
        let mut entry = 0u64;
        if c.g[A0][0] != 0 {
            c.g[V1][0] = Addr(p(this, 0x74)).lbu(c);
            c.cycles += 6;
            c.g[V0][0] = 0xc;
            c.g[A0][0] = Addr(r32(c, A0)).off(8).lw(c);
            c.g[V1][0] &= 0x3;
            c.mult(0, V1, V1, V0);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            entry = Addr(r32(c, V1)).off(0xc).lw(c);
        } else {
            c.cycles += 2;
        }
        c.g[V0][0] = entry;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIListNode_dtor(UIListNode* self, int flags)` (0x3a47f8): install
/// the list-node vtable, run the body and maybe delete the node.
pub fn ui_list_node_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_4cc0; // D_00494CC0
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x003a_4820)?;
        c.g[S0][0] &= 1;
        let owned = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if owned {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x003a_4834)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIThread_dtor(UIThread* self, int flags)` (0x3a5b48): the thread
/// node shares the list-node body and vtable.
pub fn ui_thread_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_4cc0; // D_00494CC0
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x003a_5b70)?;
        c.g[S0][0] &= 1;
        let owned = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if owned {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x003a_5b84)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cStreamMan_FreeBuffers(cStreamMan* self)` (0x3a6f38): destroy the
/// stream file and free the two buffer blocks.
pub fn c_stream_man_free_buffers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p(this, 0x88)).lw(c);
        call_ptr(c, table::STREAM_FILE_DESTROY, 0x003a_6f50)?;
        c.g[A0][0] = Addr(p(this, 0x84)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, table::BUILTIN_VEC_DELETE, 0x003a_6f64)?;
        }
        c.g[A0][0] = Addr(p(this, 0x8c)).lw(c);
        c.cycles += 2;
        let none = c.g[A0][0] == 0;
        frame.restore(c, S0, 0x10);
        if !none {
            c.cycles += 3;
            call_ptr(c, table::BUILTIN_VEC_DELETE, 0x003a_6f78)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SectionTable_dtor(SectionTable* self, int flags)` (0x3a7d80): free
/// the owned table and, when `flags & 1`, the object.
pub fn section_table_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(this).lw(c);
        let none = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 2;
            call_ptr(c, table::BUILTIN_VEC_DELETE, 0x003a_7da8)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x003a_7dbc)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cPS2ChunkResolve_Create(void)` (0x3aad98): allocate and construct a
/// chunk resolver.
pub fn c_ps2_chunk_resolve_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x49_4f70; // D_00494F70
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x003a_adbc);
        c.g[A0][0] = 0x14;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x003a_adbc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::CPS2_CHUNK_RESOLVE_CTOR.call(c, 0x003a_adc8)?;
        c.g[V1][0] = 0x49_5090; // D_00495090
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        Addr(r32(c, S0)).off(8).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void* AV_Player_ctor_2(AV_Player* self, int a1, int a2, int a3, int t0,
/// int t1)` (0x3ade08): clear the player's state and forward the constructor
/// arguments to `AV_Player_Init`.
pub fn av_player_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.g[A3][0];
        frame.save(c, S0, 0x20);
        c.g[V1][0] = c.g[T0][0];
        frame.save_ra(c, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        Addr(frame.at(c, 0)).sw(c, c.g[T2][0]);
        c.g[A3][0] = c.g[A2][0];
        c.g[T3][0] = c.g[T1][0];
        Addr(p(this, 0x60)).sw(c, 0);
        c.g[T0][0] = c.g[V0][0];
        c.g[T2][0] = c.g[V1][0];
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x003a_de44);
        c.g[T1][0] = 0;
        callee::AV_PLAYER_INIT.call(c, 0x003a_de44)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Riders, camera and the front end.

/// `void Rider_CreateFocusBox(Rider* self)` (0x11bb40): allocate the rider's
/// focus box and register it with the focus manager.
pub fn rider_create_focus_box(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x45_7960; // D_00457960
        frame.save(c, S0, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0011_bb68)?;
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        Addr(p(this, 0x78c)).sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr(r32(c, A0)).off(0x84).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0xa4).lw(c);
        callee::FOCUS_MAN_ADD_FOCUS_BOX.call(c, 0x0011_bb84)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool Rider_HasRenderSlot(Rider* self)` (0x11c0e0): whether the rider's
/// safe render tick is older than the renderer's current tick (no tick `-1`
/// means never).
pub fn rider_has_render_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(this, 0x888)).lw(c);
        let has_tick = c.g[V0][0] != c.g[V1][0];
        c.g[A1][0] = Addr(gp(c, -0x854)).lw(c);
        let mut slot = 0u64;
        if has_tick {
            c.cycles += 7;
            c.g[V1][0] = Addr(r32(c, A1)).off(0x10d8).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x390).lh(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x394).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0011_c11c)?;
            c.g[V1][0] = Addr(p(this, 0x888)).lw(c);
            slot = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        } else {
            c.cycles += 2;
        }
        c.g[V0][0] = slot;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CamController_SetAlgorithm(CamController* self, void* algo,
/// void* arg)` (0x15cf28): replace the controller's algorithm list with one
/// algorithm.
pub fn cam_controller_set_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        let this = r32(c, A0);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        callee::CAM_CONTROLLER_CLEAR_ALGORITHMS.call(c, 0x0015_cf4c)?;
        c.g[A0][0] = Addr(p(this, 8)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[AT][0] = 0x3f80_0000; // 1.0
        c.mtc1(1, 12);
        c.g[A2][0] = c.g[S2][0];
        callee::ALGORITHM_LIST_PUSH.call(c, 0x0015_cf64)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cCameraTriggerMan_ReadHeader(cCameraTriggerMan* self, void* buffer)`
/// (0x16c3d0): read three words of a trigger list header.
pub fn camera_trigger_man_read_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0]; // the serialised buffer
        c.g[S1][0] = c.g[A0][0]; // the header object
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x10));
        call_ptr(c, table::SERIALIZE_READ_U32_2, 0x0016_c3f4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(4));
        call_ptr(c, table::SERIALIZE_READ_U32_2, 0x0016_c400)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(8));
        call_ptr(c, table::SERIALIZE_READ_U32_2, 0x0016_c40c)?;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int CameraTrigger_ReadActionType0(void* reader, void* dst)` (0x1715a0):
/// read the action's float and uint payloads and return the bytes consumed.
pub fn camera_trigger_read_action_type0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0]; // the action record
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0]; // the reader
        frame.save_ra(c, 0);
        Addr(r32(c, S0)).sw(c, 0);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0xc));
        call_ptr(c, table::SERIALIZE_READ_U32, 0x0017_15c8)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(8));
        call_ptr(c, table::SERIALIZE_READ_U32_2, 0x0017_15d8)?;
        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DebugMenuModule_v06(DebugMenuModule* self)` (0x17fb98): poll the
/// debug menu's child module (when no world is up) and update the cursor
/// trail.
pub fn debug_menu_module_v06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::APP_MODULE_UPDATE_FADE.call(c, 0x0017_fbac)?;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(4).lw(c);
        if c.g[V1][0] == 0 {
            c.cycles += 6;
            c.g[A1][0] = Addr(p(this, 0x14)).lw(c);
            c.g[V0][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x10).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x14).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0017_fbd4)?;
        }
        c.cycles += 6;
        c.g[A0][0] = sx(this.wrapping_add(0x24));
        callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL.call(c, 0x0017_fbdc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRiderDetail_OnRender(cFEStateRiderDetail* self)`
/// (0x183550): resolve the "rider detail" panel and fade it in.
pub fn c_fe_state_rider_detail_on_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(this, 0x50)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = 0x45_d898; // D_0045D898
            call_ptr(c, table::STR_HASH, 0x0018_3578)?;
            c.g[A0][0] = Addr(p(this, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, table::UI_SCREEN_FIND_OBJECT, 0x0018_3584)?;
            let none = c.g[V0][0] == 0;
            frame.restore(c, S0, 0x10);
            if !none {
                c.cycles += 3;
                c.g[A0][0] = c.g[V0][0];
                callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF.call(c, 0x0018_3594)?;
            }
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBraggingRights_OnChildEventQuit(cFEState* self, int a,
/// int event)` (0x194440): the quit child event: tell the setup manager to
/// quit and empty the child, otherwise forward the event.
pub fn c_fe_state_bragging_rights_on_child_event_quit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let quit = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if !quit {
            c.cycles += 3;
            c.g[A0][0] = Addr(gp(c, 0x2dc)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                c.g[A1][0] = 0xf9;
                callee::C_SETUP_MAN_SEND_QUIT.call(c, 0x0019_446c)?;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER.call(c, 0x0019_4474)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x0019_4484)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateTitle_OnWidgetCreated(cFEStateTitle* self, UIObject* item)`
/// (0x194988): when the widget is the title screen's, clear its user data.
pub fn c_fe_state_title_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x4a_17d0; // D_004A17D0
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let item = r32(c, A1);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = Addr(p(item, 0x38)).lw(c);
        call_ptr(c, table::STR_HASH, 0x0019_49ac)?;
        let other = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, S0, 0x20);
        if !other {
            c.cycles += 6;
            Addr(p(item, 0x18)).sw(c, 0);
            c.g[A0][0] = sx(item);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::UI_OBJECT_SET_USER_DATA.call(c, 0x0019_49c8)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* FEAsyncManager_ctor(FEAsyncManager* self)` (0x1a3590): mount the
/// front-end's big file when it exists.
pub fn fe_async_manager_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x46_1418; // D_00461418
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[V0][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, table::BXFILE_EXISTS, 0x001a_35b4)?;
        let missing = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !missing {
            c.cycles += 4;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x64;
            c.g[A3][0] = sx(this.wrapping_add(0x58c));
            callee::EA_FILE_MOUNT_BIG.call(c, 0x001a_35cc)?;
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void NetFEState_OnActivate(NetFEState* self, int arg)` (0x1a87d0): run
/// the state's activation hook and mark it active.
pub fn net_fe_state_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(this, 8)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x100).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x104).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x001a_87fc)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e4c0, 0x001a_8808)?;
        c.g[V0][0] = 0x1;
        frame.restore(c, S1, 0x10);
        Addr(p(this, 0x48)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEUseExistingAccount_ctor(cFEUseExistingAccount* self, int a1)`
/// (0x1ac010): construct the account screen's net state and install its
/// fields.
pub fn c_fe_use_existing_account_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR.call(c, 0x001a_c028)?;
        c.g[V0][0] = 0x46_9068; // D_00469068
        c.g[A0][0] = Addr(gp(c, 0xda0)).lw(c);
        c.g[V1][0] = 0x35;
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(this, 0x6dc)).sw(c, c.g[A0][0]);
        Addr(p(this, 0xc)).sw(c, c.g[V1][0]);
        Addr(p(this, 0x6d0)).sw(c, 0);
        Addr(p(this, 0x6e0)).sw(c, 0);
        Addr(p(this, 0x6a8)).sw(c, 0);
        frame.restore(c, S0, 0x10);
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
    use crate::difftest::fakes::{machine, vtable, FAKE};
    use crate::difftest::{check, put_words, DATA, GAME_GP, RA as TEST_RA};

    const SEEDS: u32 = 120;

    /// A `gp`-relative global address.
    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// Call a generated helper as a setup step (arguments in a0..t1).
    fn load(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = TEST_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, TEST_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// Zero `words` words at `at`.
    fn clear(c: &mut Cpu, at: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(at + 4 * i, 0);
        }
    }

    /// The main heap's arena over 256 KiB of test RAM, so constructors and
    /// destructors that allocate or free take their normal path.
    fn alloc_ready(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = GAME_GP as u64;
        load(
            c,
            0x0031_ff60,
            ssx3_ee::s10::f_0031ff60,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
        c.bus.write32(GAME_GP + 0xdd8, 1); // size-class pools ready
    }

    /// A block from the test heap (`alloc_ready` must have run first).
    fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        const MEM_HEAP: u32 = 0x004f_f250;
        load(
            c,
            0x0032_0058,
            ssx3_ee::s10::f_00320058,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    /// The dummy world at `world_at`: `+0x84` the game module, its `+0xc` the
    /// race manager. Returns the module address.
    fn dummy_world(c: &mut Cpu, world_at: u32) -> u32 {
        let module = DATA + 0x3000;
        let race = DATA + 0x3400;
        clear(c, world_at, 0x40);
        clear(c, module, 0x100);
        clear(c, race, 0x100);
        c.bus.write32(world_at + 0x84, module);
        c.bus.write32(module + 0xc, race);
        c.bus.write32(gp_at(-0x848), world_at);
        module
    }

    // ------------------------------------------------------------------
    // Audio.

    #[test]
    fn ssx_audio_system_set_gain_target_matches() {
        check(
            ee::s07::f_002887a8__generated,
            ssx_audio_system_set_gain_target,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2887_a8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let ch = (seed % 5) as u32;
                let target = r.edge_f32();
                c.g[A0][0] = sx(this);
                c.g[A1][0] = ch as u64;
                c.g[A2][0] = (seed % 2) as u64;
                c.f[12] = target;
                c.f[13] = if seed % 7 == 0 { 0 } else { r.edge_f32() };
                c.bus.write32(gp_at(-0x4614), 0x3c88_8893); // 1/60
                let cur = if seed % 3 == 0 { target } else { r.edge_f32() };
                c.bus.write32(this + 0x63e4 + 4 * ch, cur);
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_duck_gain_matches() {
        check(
            ee::s07::f_00289830__generated,
            ssx_audio_system_set_duck_gain,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2898_30, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let v = r.edge_f32();
                c.g[A0][0] = sx(this);
                c.f[12] = v;
                c.bus.write32(gp_at(-0x4620), 0x3c88_8893);
                let cur = if seed % 2 == 0 { v } else { r.edge_f32() };
                c.bus.write32(this + 0x6c5c, cur);
            },
        );
    }

    #[test]
    fn snd_bank_unload_linked_matches() {
        check(
            ee::s07::f_0028b2d0__generated,
            snd_bank_unload_linked,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x28b2_d0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                let nodes = (seed % 4) as u32;
                let mut next = this;
                for i in (0..nodes).rev() {
                    let node = DATA + 0x200 + 0x100 * i;
                    clear(c, node, 0x40);
                    c.bus.write32(node + 0x18, next);
                    next = node;
                }
                let head = if seed % 5 == 0 { this } else { next };
                c.bus.write32(this + 0x18, head);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn snd_bank_unload_memory_matches() {
        check(
            ee::s07::f_0028b7e0__generated,
            snd_bank_unload_memory,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28b7_e0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                let bank = match seed % 3 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    _ => seed % 4,
                };
                c.bus.write32(this + 4, bank);
                c.bus.write32(this + 0x10, r.next_u32());
                c.bus.write32(this + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_dj_speech_on_matches() {
        check(
            ee::s08::f_0029f0d8__generated,
            ssx_audio_system_is_dj_speech_on,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29f0_d8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x100);
                let module = dummy_world(c, DATA + 0x2000);
                match seed % 3 {
                    0 => c.bus.write32(module + 0x28, 0),
                    1 => {
                        c.bus.write32(module + 0x28, DATA + 0x300);
                        c.bus.write32(DATA + 0x300, 5);
                    }
                    _ => {
                        c.bus.write32(module + 0x28, DATA + 0x304);
                        c.bus.write32(DATA + 0x304, 20);
                    }
                }
                c.g[A0][0] = sx(this);
                let vol = if seed % 4 == 0 { 0 } else { r.edge_f32() };
                c.bus.write32(this + 0x62bc + 2 * 16, vol);
            },
        );
    }

    #[test]
    fn ssx_audio_system_start_section_speech_timer_matches() {
        check(
            ee::s08::f_002a49e8__generated,
            ssx_audio_system_start_section_speech_timer,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2a49_e8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let mon = DATA + 0x2000;
                clear(c, this, 0x40);
                clear(c, mon, 0x800);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x118, mon);
                put_words(c, 0x004a_36f0, &[0xffff_0000, 0x002a_4a68]);
            },
        );
    }

    #[test]
    fn crowd_audio_destroy_instance_matches() {
        check(
            ee::s08::f_002a4e38__generated,
            crowd_audio_destroy_instance,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a4e_38, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let mon = DATA + 0x6000;
                clear(c, this, 0x200);
                clear(c, mon, 0x200);
                c.bus.write32(mon, mon);
                c.bus.write32(gp_at(0x608), mon);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
                let src = if seed % 3 == 0 { u32::MAX } else { seed % 4 };
                c.bus.write32(this + 0x174, src);
                c.bus.write32(this + 0x188, r.edge_f32());
            },
        );
    }

    #[test]
    fn ini_parser_dtor_matches() {
        check(ee::s08::f_002a7890__generated, ini_parser_dtor, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x2a78_90, 0);
            alloc_ready(c);
            let this = DATA + 0x100;
            clear(c, this, 0x20);
            let owned = if seed % 2 == 0 {
                0
            } else {
                heap_block(c, 0x20)
            };
            c.bus.write32(this + 4, owned);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 3 == 0) as u64;
        });
    }

    #[test]
    fn bank_monitor_instance_resume_matches() {
        check(
            ee::s08::f_002acb30__generated,
            bank_monitor_instance_resume,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2acb_30, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x120);
                c.g[A0][0] = sx(this);
                // `*(self)` non-zero takes the "resume" path in PollFree.
                c.bus.write32(this, (seed % 3) as u32);
                c.bus.write32(this + 0x9c, 0);
                c.bus.write32(
                    this + 4,
                    match seed % 4 {
                        0 => u32::MAX,
                        n => n,
                    },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_flush_matches() {
        check(
            ee::s08::f_002c6b00__generated,
            c_ps2_mem_card_do_flush,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2c6b_00, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let card = DATA + 0x2000;
                clear(c, this, 0x80);
                clear(c, card, 0x100);
                c.bus.write32(this + 0x18, card);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn loc_string_table_dtor_matches() {
        check(
            ee::s08::f_002c6d70__generated,
            loc_string_table_dtor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2c6d_70, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                let owned = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                c.bus.write32(this + 4, owned);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }

    #[test]
    fn avalanche_bake_group_matches() {
        check(
            ee::s09::f_002d9660__generated,
            avalanche_bake_group,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2d96_60, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                dummy_world(c, DATA + 0x2000);
                let nodes = (seed % 3) as u32;
                let mut next = 0u32;
                for i in (0..nodes).rev() {
                    let node = DATA + 0x6000 + 0x200 * i;
                    clear(c, node, 0x80);
                    c.bus.write32(node + 0xf4, next);
                    next = node;
                }
                c.bus.write32(this + 4, next);
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------------------------------
    // Script engine.

    #[test]
    fn wscript_process_forward_event_to_task_matches() {
        check(
            ee::s10::f_00307a10__generated,
            wscript_process_forward_event_to_task,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x307a_10, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let base = DATA + 0x800;
                clear(c, this, 0x20);
                clear(c, base, 0x400);
                let count = (seed % 3) as u32;
                let cur = (seed % 4) as u32;
                c.bus.write32(this + 4, count);
                c.bus.write32(this + 8, cur);
                c.bus.write32(this + 0x14, base);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                if cur < count {
                    let entry = base + cur * 0x44;
                    let vt = vtable(c, &mut r, &[0x18]);
                    c.bus.write32(entry + 0x40, vt);
                }
            },
        );
    }

    #[test]
    fn wscript_man_run_obj_script0_matches() {
        check(
            ee::s10::f_0030a610__generated,
            wscript_man_run_obj_script0,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x30a6_10, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let cache = DATA + 0x6000;
                clear(c, this, 0x400);
                clear(c, cache, 0x40);
                c.bus.write32(this + 0x28c, cache);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn wscript_process_list_delete_all_matches() {
        check(
            ee::s10::f_0030b1b8__generated,
            wscript_process_list_delete_all,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b1_b8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                let nodes = (seed % 3) as u32;
                let mut next = 0u32;
                for i in (0..nodes).rev() {
                    let node = DATA + 0x800 + 0x100 * i;
                    clear(c, node, 0x80);
                    c.bus.write32(node + 0x18, next);
                    let vt = vtable(c, &mut r, &[8]);
                    c.bus.write32(node + 0x5c, vt);
                    next = node;
                }
                c.bus.write32(this, next);
                c.g[A0][0] = sx(this);
                c.pc = 0x0030_b1b8;
            },
        );
    }

    #[test]
    fn wscript_man_request_mission_start_matches() {
        check(
            ee::s10::f_0030b540__generated,
            wscript_man_request_mission_start,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b5_40, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let module = dummy_world(c, DATA + 0x2000);
                let playback = DATA + 0x4000;
                let replay = DATA + 0x4400;
                clear(c, playback, 0x40);
                clear(c, replay, 0x40);
                c.bus.write32(module + 0x34, playback);
                c.bus.write32(module + 0x28, replay);
                // The replay manager's state 14 makes `Replay_Reset` return at
                // once (its reset is not what this test exercises).
                c.bus.write32(replay, 14);
                c.bus.write32(this + 0x2a0, r.next_u32());
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------------------------------
    // Objects and world.

    #[test]
    fn nash_create_fwd_diff_cache_matches() {
        check(
            ee::s11::f_00334800__generated,
            nash_create_fwd_diff_cache,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3348_00, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x120);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn crowd_man3d_group_load_matches() {
        check(
            ee::s11::f_00344ff8__generated,
            crowd_man3d_group_load,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x344f_f8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let stream = DATA + 0x800;
                clear(c, this, 0x40);
                clear(c, stream, 0x100);
                let vt = vtable(c, &mut r, &[0x10]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    #[test]
    fn flex_rail_update_matches() {
        check(ee::s11::f_00348008__generated, flex_rail_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3480_08, 0);
            alloc_ready(c);
            let this = DATA + 0x100;
            let vt = DATA + 0x800;
            clear(c, this, 0x40);
            clear(c, vt, 0x200);
            c.bus.write32(this + 0xc, vt);
            c.bus
                .write32(vt + 0x178, (r.next_u32() as i16 as i32 & !3) as u32);
            c.bus.write32(vt + 0x17c, FAKE);
            c.bus
                .write32(vt + 0x180, (r.next_u32() as i16 as i32 & !3) as u32);
            c.bus.write32(vt + 0x184, FAKE + 0x10);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn c_flag_manager_load_matches() {
        check(
            ee::s11::f_0034c898__generated,
            c_flag_manager_load,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34c8_98, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x2000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn floating_v01_matches() {
        check(ee::s11::f_0034ee68__generated, floating_v01, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x34ee_68, 0);
            alloc_ready(c);
            let this = DATA + 0x100;
            clear(c, this, 0x200);
            let owned = if seed % 2 == 0 {
                0
            } else {
                heap_block(c, 0x30)
            };
            c.bus.write32(this + 0x74, owned);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 3 == 0) as u64;
        });
    }

    #[test]
    fn instances_man_dtor_matches() {
        check(
            ee::s11::f_00351120__generated,
            instances_man_dtor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3511_20, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                let owned = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                c.bus.write32(this + 0xc, owned);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }

    #[test]
    fn modifier_block_reset_particles_matches() {
        check(
            ee::s11::f_00353228__generated,
            modifier_block_reset_particles,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3532_28, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                let nodes = (seed % 3) as u32;
                let mut next = 0u32;
                for i in (0..nodes).rev() {
                    let node = DATA + 0x800 + 0x100 * i;
                    clear(c, node, 0x80);
                    c.bus.write32(node, next);
                    let vt = vtable(c, &mut r, &[0x18]);
                    c.bus.write32(node + 8, vt);
                    next = node;
                }
                c.bus.write32(this + 0x10, next);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn flex_bridge_springs_dtor_matches() {
        check(
            ee::s11::f_00353db8__generated,
            flex_bridge_springs_dtor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x353d_b8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x60);
                let owned = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                c.bus.write32(this + 0x24, owned);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }

    #[test]
    fn modifier_list_node_dtor_matches() {
        check(
            ee::s12::f_00360c98__generated,
            modifier_list_node_dtor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x360c_98, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn rail_modifier_list_node_dtor_matches() {
        check(
            ee::s12::f_00360d40__generated,
            rail_modifier_list_node_dtor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x360d_40, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn intc_table_remove_handler_matches() {
        check(
            ee::s12::f_00362008__generated,
            intc_table_remove_handler,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3620_08, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                for i in 0..8 {
                    c.bus.write32(this + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn tex_man_lock_texture_matches() {
        check(
            ee::s12::f_00367cd0__generated,
            tex_man_lock_texture,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x367c_d0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x100);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_brightness_alpha_matches() {
        check(
            ee::s12::f_0037c720__generated,
            ps2_graphics_man_set_brightness_alpha,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37c7_20, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x4000);
                c.g[A0][0] = sx(this);
                c.f[12] = match seed % 4 {
                    0 => 0xbf80_0000, // -1
                    1 => 0x3f80_0000, // 1
                    2 => 0x4000_0000, // 2
                    _ => r.edge_f32(),
                };
            },
        );
    }

    #[test]
    fn light_man_v09_matches() {
        check(ee::s12::f_0038dee8__generated, light_man_v09, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x38de_e8, 0);
            alloc_ready(c);
            let this = DATA + 0x100;
            let vt = vtable(c, &mut r, &[0x10]);
            clear(c, this, 0x40);
            c.bus.write32(this, vt);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.f[12] = r.edge_f32();
        });
    }

    #[test]
    fn light_man_init_instances_matches() {
        check(
            ee::s13::f_00394c98__generated,
            light_man_init_instances,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x394c_98, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 16) as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // UI and streaming.

    #[test]
    fn ui_list_box_get_mode_entry_matches() {
        check(
            ee::s13::f_00399920__generated,
            ui_list_box_get_mode_entry,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3999_20, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let p1 = DATA + 0x800;
                let p2 = DATA + 0x820;
                let p3 = DATA + 0x840;
                let table = DATA + 0x900;
                clear(c, this, 0x100);
                clear(c, p1, 0x200);
                clear(c, table, 0x80);
                c.g[A0][0] = sx(this);
                c.bus.write8(this + 0x74, (seed % 4) as u8);
                if seed % 2 == 0 {
                    c.bus.write32(this + 0x5c, p1);
                    c.bus.write32(p1 + 0xd0, p2);
                    c.bus.write32(p2 + 0x10, p3);
                    c.bus.write32(p3 + 8, table);
                    for i in 0..4u32 {
                        c.bus.write32(table + i * 0xc + 0xc, r.next_u32());
                    }
                }
            },
        );
    }

    #[test]
    fn ui_list_node_dtor_matches() {
        check(ee::s13::f_003a47f8__generated, ui_list_node_dtor, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x3a47_f8, 0);
            alloc_ready(c);
            let this = DATA + 0x100;
            clear(c, this, 0x40);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn ui_thread_dtor_matches() {
        check(ee::s13::f_003a5b48__generated, ui_thread_dtor, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x3a5b_48, 0);
            alloc_ready(c);
            let this = DATA + 0x100;
            clear(c, this, 0x40);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn c_stream_man_free_buffers_matches() {
        check(
            ee::s13::f_003a6f38__generated,
            c_stream_man_free_buffers,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3a6f_38, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let file = DATA + 0x6000;
                clear(c, this, 0x100);
                clear(c, file, 0x40);
                // Not a stream-file magic, so `StreamFile_Destroy` returns at
                // once instead of waiting for the I/O thread.
                c.bus.write32(this + 0x88, file);
                let a = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                let b = if seed % 3 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                c.bus.write32(this + 0x84, a);
                c.bus.write32(this + 0x8c, b);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn section_table_dtor_matches() {
        check(
            ee::s13::f_003a7d80__generated,
            section_table_dtor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3a7d_80, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                let owned = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                c.bus.write32(this, owned);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }

    #[test]
    fn c_ps2_chunk_resolve_create_matches() {
        check(
            ee::s13::f_003aad98__generated,
            c_ps2_chunk_resolve_create,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3aad_98, 0);
                alloc_ready(c);
            },
        );
    }

    #[test]
    fn av_player_ctor_2_matches() {
        check(ee::s13::f_003ade08__generated, av_player_ctor_2, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3ade_08, 0);
            alloc_ready(c);
            let this = DATA + 0x100;
            clear(c, this, 0x100);
            c.g[A0][0] = sx(this);
            for i in 1..6 {
                c.g[[A1, A2, A3, T0, T1][i - 1]][0] = r.next_u32() as u64;
            }
        });
    }

    // ------------------------------------------------------------------
    // Riders, camera and the front end.

    #[test]
    fn rider_create_focus_box_matches() {
        check(
            ee::s00::f_0011bb40__generated,
            rider_create_focus_box,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x11bb_40, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let world = DATA + 0x2000;
                clear(c, this, 0x800);
                let module = dummy_world(c, world);
                let race = DATA + 0x3400;
                let focus = DATA + 0x4000;
                clear(c, focus, 0x100);
                c.bus.write32(module + 0xc, race);
                c.bus.write32(race + 0xa4, focus);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn rider_has_render_slot_matches() {
        check(
            ee::s00::f_0011c0e0__generated,
            rider_has_render_slot,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11c0_e0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let obj = DATA + 0x800;
                let vt = DATA + 0x1000;
                clear(c, this, 0x1000);
                clear(c, obj, 0x2000);
                clear(c, vt, 0x400);
                c.bus.write32(gp_at(-0x854), obj);
                c.bus.write32(obj + 0x10d8, vt);
                c.bus
                    .write32(vt + 0x390, (r.next_u32() as i16 as i32 & !3) as u32);
                c.bus.write32(vt + 0x394, FAKE);
                let tick = if seed % 3 == 0 {
                    u32::MAX
                } else {
                    r.next_u32()
                };
                c.bus.write32(this + 0x888, tick);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn cam_controller_set_algorithm_matches() {
        check(
            ee::s01::f_0015cf28__generated,
            cam_controller_set_algorithm,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15cf_28, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let list = DATA + 0x800;
                clear(c, this, 0x40);
                clear(c, list, 0x100);
                c.bus.write32(this + 8, list);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn camera_trigger_man_read_header_matches() {
        check(
            ee::s02::f_0016c3d0__generated,
            camera_trigger_man_read_header,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16c3_d0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let reader = DATA + 0x800;
                let bytes = DATA + 0x900;
                clear(c, this, 0x40);
                clear(c, reader, 0x20);
                for i in 0..0x40 {
                    c.bus.write32(bytes + 4 * i, r.next_u32());
                }
                c.bus.write32(reader, bytes);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(reader);
            },
        );
    }

    #[test]
    fn camera_trigger_read_action_type0_matches() {
        check(
            ee::s02::f_001715a0__generated,
            camera_trigger_read_action_type0,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1715_a0, 0);
                alloc_ready(c);
                let reader = DATA + 0x800;
                let dst = DATA + 0x100;
                let bytes = DATA + 0x900;
                clear(c, dst, 0x40);
                clear(c, reader, 0x20);
                for i in 0..0x40 {
                    c.bus.write32(bytes + 4 * i, r.next_u32());
                }
                c.bus.write32(reader, bytes);
                c.g[A0][0] = sx(reader);
                c.g[A1][0] = sx(dst);
            },
        );
    }

    #[test]
    fn debug_menu_module_v06_matches() {
        check(
            ee::s02::f_0017fb98__generated,
            debug_menu_module_v06,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17fb_98, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let world = DATA + 0x2000;
                let obj = DATA + 0x800;
                let vt = vtable(c, &mut r, &[0x10]);
                clear(c, this, 0x100);
                clear(c, obj, 0x100);
                c.bus.write32(this + 0x14, obj);
                c.bus.write32(obj, vt);
                dummy_world(c, world);
                if seed % 2 == 0 {
                    c.bus.write32(world + 4, 0);
                } else {
                    c.bus.write32(world + 4, 1);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_fe_state_rider_detail_on_render_matches() {
        check(
            ee::s02::f_00183550__generated,
            c_fe_state_rider_detail_on_render,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1835_50, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let screen = DATA + 0x800;
                clear(c, this, 0x100);
                clear(c, screen, 0x200);
                c.bus.write32(this + 0x40, screen);
                c.bus
                    .write32(this + 0x50, if seed % 2 == 0 { 0 } else { 1 });
                put_words(c, 0x0045_d898, &[0x5449_544c, 0]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_fe_state_bragging_rights_on_child_event_quit_matches() {
        check(
            ee::s02::f_00194440__generated,
            c_fe_state_bragging_rights_on_child_event_quit,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1944_40, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let setup = DATA + 0x800;
                clear(c, this, 0x100);
                clear(c, setup, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (seed % 16) as u64
                };
                c.bus
                    .write32(gp_at(0x2dc), if seed % 3 == 0 { 0 } else { setup });
            },
        );
    }

    #[test]
    fn c_fe_state_title_on_widget_created_matches() {
        check(
            ee::s02::f_00194988__generated,
            c_fe_state_title_on_widget_created,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1949_88, 0);
                alloc_ready(c);
                let item = DATA + 0x100;
                clear(c, item, 0x100);
                put_words(c, 0x004a_17d0, &[0x5449_544c, 0]);
                let hash = load(c, 0x0031_7670, ee::s10::f_00317670, &[0x004a_17d0]) as u32;
                c.bus
                    .write32(item + 0x38, if seed % 2 == 0 { hash } else { hash ^ 1 });
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(item);
            },
        );
    }

    #[test]
    fn fe_async_manager_ctor_matches() {
        check(
            ee::s03::f_001a3590__generated,
            fe_async_manager_ctor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1a35_90, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x800);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn net_fe_state_on_activate_matches() {
        check(
            ee::s03::f_001a87d0__generated,
            net_fe_state_on_activate,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a87_d0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let vt = vtable(c, &mut r, &[0x100]);
                clear(c, this, 0x100);
                c.bus.write32(this + 8, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn c_fe_use_existing_account_ctor_matches() {
        check(
            ee::s03::f_001ac010__generated,
            c_fe_use_existing_account_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1ac0_10, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x800);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(gp_at(0xda0), r.next_u32());
            },
        );
    }
}
