//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small engine, front-end, camera, replay, script and
//! audio helpers, each with a differential test against the generated body.
//!
//! - **Engine/threads**: `PS2ExecMan_VSyncThread` (the vsync thread loop),
//!   `InputPreproc_WriteString` (write a NUL-terminated string).
//! - **Collision**: `CollSphere_Transform` (transform a sphere's centre by a
//!   matrix, scale its radius), `CollSphereTree_dtor`.
//! - **Objects**: `Counter_ctor`, `DeadFade_dtor`, `ModObj_Set*Modifier`,
//!   `MultiParticleMan_Clear`, `DynamicEmitter_dtor`, `cHullPage_ctor/_dtor`,
//!   `BoaderShadow_AllocTexture`.
//! - **UI/FE**: `UIScreen_CmdGotoLabel`, `UIText_FreeStrings`, the FE state
//!   ctors/dtors and the widget handlers that hash a widget's id.
//! - **Camera**: `GameCamera_ResetAllChaseControllers`,
//!   `CameraTriggerMan_Unload`.
//! - **Replay/script/audio**: `PlaybackMan_Stop`, `Player_FreeReplayCache`,
//!   `Replay_AllocSaveBuffers`, the `ScriptEngine`/`ScriptList` helpers,
//!   `RiderScriptObj_MatchesRiderSlot`, `SSXAudioSystem_OnAnimSoundEvent`,
//!   `SndBank_LoadFromMemory`.
//!
//! The replacements keep their state in the guest registers the originals
//! use (callers, callees and event handlers see the same values), follow the
//! call protocol of [`crate::call`] (direct calls through the generated
//! aliases, `jalr` and cross-shard calls through the function table) and the
//! event rules of [`crate::events`]. VU0 macro ops are issued on `c.vu0()`
//! exactly as the originals do.

use crate::abi;
use crate::call::{call_ptr, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

#[rustfmt::skip]
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_ac08, ps2_exec_man_vsync_thread, "PS2ExecMan_VSyncThread"),
    (0x0032_6308, input_preproc_write_string, "InputPreproc_WriteString"),
    (0x0032_fd40, coll_sphere_transform, "CollSphere_Transform"),
    (0x0033_04e8, coll_sphere_tree_dtor, "CollSphereTree_dtor"),
    (0x0034_3768, counter_ctor, "Counter_ctor"),
    (0x0035_0be0, dead_fade_dtor, "DeadFade_dtor"),
    (0x0035_55a8, mod_obj_set_tex_flip_modifier, "ModObj_SetTexFlipModifier"),
    (0x0035_5600, mod_obj_set_boost_modifier, "ModObj_SetBoostModifier"),
    (0x0035_7b38, multi_particle_man_clear, "MultiParticleMan_Clear"),
    (0x0037_0c08, dynamic_emitter_dtor, "DynamicEmitter_dtor"),
    (0x0037_5990, boader_shadow_alloc_texture, "BoaderShadow_AllocTexture"),
    (0x0039_cbe0, ui_screen_cmd_goto_label, "UIScreen_CmdGotoLabel"),
    (0x003a_0b38, ui_text_free_strings, "UIText_FreeStrings"),
    (0x003a_7bd8, c_hull_page_ctor, "cHullPage_ctor"),
    (0x003a_7c30, c_hull_page_dtor, "cHullPage_dtor"),
    (0x003b_04a0, rcmp_decoder_dtor, "RCMP_DECODER_dtor"),
    (0x0010_e8b8, rider_award_bonus_boost, "Rider_AwardBonusBoost"),
    (0x0011_a110, rider_metrix_on_mission_completed, "RiderMetrix_OnMissionCompleted"),
    (0x0012_8998, ai_setup_actor_riders, "AI_SetupActorRiders"),
    (0x0014_75c0, be_new_player_int_set_slot_flag3, "BENewPlayerInt_SetSlotFlag3"),
    (0x0015_dc18, game_camera_reset_all_chase_controllers, "GameCamera_ResetAllChaseControllers"),
    (0x0016_c9b0, camera_trigger_man_unload, "CameraTriggerMan_Unload"),
    (0x0018_5a18, cfe_options_credits_ctor, "cFEOptionsCredits_ctor"),
    (0x0018_c8b8, cfe_state_options_controller_shutdown, "cFEStateOptionsController_Shutdown"),
    (0x0019_1360, cfe_state_rule_select_ctor, "cFEStateRuleSelect_ctor"),
    (0x0019_58f0, fe_locale_v01, "FELocale_v01"),
    (0x001b_77c8, cfe_state_net_account_mem_update_state02, "cFEStateNetAccountMem_UpdateState02"),
    (0x001b_ee78, cfe_state_login_dtor, "cFEStateLogin_dtor"),
    (0x001c_0a28, cfe_state_online_agreement_dtor, "cFEStateOnlineAgreement_dtor"),
    (0x001c_5310, cfe_state_net_news_on_widget_created, "cFEStateNetNews_OnWidgetCreated"),
    (0x001c_af00, cui_state_buy_popup_on_widget_action, "cUIStateBuyPopup_OnWidgetAction"),
    (0x001d_50e0, cfe_state_relogin_on_widget_created, "cFEStateRelogin_OnWidgetCreated"),
    (0x0023_2d50, start_screen_unload, "StartScreen_Unload"),
    (0x0023_e2c8, mc_manager_state35, "MCManager_State35"),
    (0x0023_e498, mc_manager_state33, "MCManager_State33"),
    (0x0024_5cd0, cfe_load_state_on_widget_created, "cFELoadState_OnWidgetCreated"),
    (0x0026_5d98, cshared_login_man_do_load, "cSharedLoginMan_DoLoad"),
    (0x0026_6240, cshared_login_man_mc_delete_path, "cSharedLoginMan_McDeletePath"),
    (0x0026_6e30, cvoip_man_activate, "cVoipMan_Activate"),
    (0x0026_cdf8, playback_man_stop, "PlaybackMan_Stop"),
    (0x0027_00d8, player_free_replay_cache, "Player_FreeReplayCache"),
    (0x0027_0f20, replay_alloc_save_buffers, "Replay_AllocSaveBuffers"),
    (0x0027_a798, script_engine_on_seq_end, "ScriptEngine_OnSeqEnd"),
    (0x0028_0560, rider_script_obj_matches_rider_slot, "RiderScriptObj_MatchesRiderSlot"),
    (0x0028_1ed8, script_engine_script_db_dtor, "ScriptEngine_ScriptDbDtor"),
    (0x0028_2338, script_engine_stop_process_seq, "ScriptEngine_StopProcessSeq"),
    (0x0028_9b18, ssx_audio_system_on_anim_sound_event, "SSXAudioSystem_OnAnimSoundEvent"),
    (0x0028_b788, snd_bank_load_from_memory, "SndBank_LoadFromMemory"),
    // Not installed (no bounded test): `OVScreenMap_OnWidgetAction`
    // (0x2085b8) calls the 3.7 KiB `EventSelect_OnWidgetAction` screen
    // machine, and `ScriptEngine_ListStop` (0x2790a0) runs the whole
    // `ScriptList_Stop` chain; both need the front-end / script-list
    // fixtures. Their ports below reproduce them exactly.
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const APP_VSYNC_CALLBACK: Callee = Callee {
        addr: 0x0031_7500,
        f: ee::s10::App_VSyncCallback,
    };
    pub const INPUT_PREPROC_WRITE: Callee = Callee {
        addr: 0x0032_6360,
        f: ee::s10::InputPreproc_Write,
    };
    pub const WORLD_OBJ_CTOR: Callee = Callee {
        addr: 0x0034_fb00,
        f: ee::s11::WorldObj_ctor,
    };
    pub const WORLD_OBJ_DTOR: Callee = Callee {
        addr: 0x0034_fbf0,
        f: ee::s11::WorldObj_dtor,
    };
    pub const MOD_OBJ_CREATE_MODIFIER_BLOCK: Callee = Callee {
        addr: 0x0035_5468,
        f: ee::s11::ModObj_CreateModifierBlock,
    };
    pub const MODIFIER_BLOCK_FREE_TEX_FLIP: Callee = Callee {
        addr: 0x0035_3150,
        f: ee::s11::ModifierBlock_FreeTexFlip,
    };
    pub const MODIFIER_BLOCK_FREE_BOOST: Callee = Callee {
        addr: 0x0035_3188,
        f: ee::s11::ModifierBlock_FreeBoost,
    };
    pub const MULTI_PARTICLE_DTOR: Callee = Callee {
        addr: 0x0035_7fa0,
        f: ee::s11::MultiParticle_dtor,
    };
    pub const DYNAMIC_EMITTER_FREE_DATA: Callee = Callee {
        addr: 0x0037_0cf8,
        f: ee::s12::DynamicEmitter_FreeData,
    };
    pub const RCMP_DECODER_RELEASE_CODEC: Callee = Callee {
        addr: 0x003b_0538,
        f: ee::s13::RCMP_DECODER_ReleaseCodec,
    };
    pub const RIDER_METRIX_AWARD_BONUS_POINTS: Callee = Callee {
        addr: 0x0011_9608,
        f: ee::s00::RiderMetrix_AwardBonusPoints,
    };
    pub const RIDER_ADD_BOOST: Callee = Callee {
        addr: 0x0010_e098,
        f: ee::s00::Rider_AddBoost,
    };
    pub const RIDER_METRIX_SHOW_HUD_NUMBER: Callee = Callee {
        addr: 0x0011_7b88,
        f: ee::s00::RiderMetrix_ShowHUDNumber,
    };
    pub const RIDER_METRIX_AWARD_POINTS: Callee = Callee {
        addr: 0x0011_9ef8,
        f: ee::s00::RiderMetrix_AwardPoints,
    };
    pub const AI_CREATE_ACTOR_RIDERS: Callee = Callee {
        addr: 0x0012_9ff8,
        f: ee::s00::AI_CreateActorRiders,
    };
    pub const BE_NEW_PLAYER_INT_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::BENewPlayerInt_GetSlot,
    };
    pub const CHASE_CAM_CONTROLLER_RESET_STATE: Callee = Callee {
        addr: 0x0016_2138,
        f: ee::s01::ChaseCamController_ResetState,
    };
    pub const ACTIVE_CAMERA_TRIGGER_LIST_CLEAR: Callee = Callee {
        addr: 0x0016_c628,
        f: ee::s02::ActiveCameraTriggerList_Clear,
    };
    pub const CAMERA_TRIGGER_STACK_CLEAR: Callee = Callee {
        addr: 0x0016_d200,
        f: ee::s02::CameraTriggerStack_Clear,
    };
    pub const CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS: Callee = Callee {
        addr: 0x0016_cbe0,
        f: ee::s02::CameraTriggerMan_RemoveAllTriggers,
    };
    pub const CAMERA_TRIGGER_MAN_CLEAR_DATA: Callee = Callee {
        addr: 0x0016_bf20,
        f: ee::s02::CameraTriggerMan_ClearData,
    };
    pub const CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION: Callee = Callee {
        addr: 0x0018_d108,
        f: ee::s02::cFEStateOptionsController_SetVibration,
    };
    pub const FE_LOCALE_UNLOAD_ALL_TABLES: Callee = Callee {
        addr: 0x0019_5948,
        f: ee::s02::FELocale_UnloadAllTables,
    };
    pub const CFE_STATE_NET_ACCOUNT_MEM_SET_STATE: Callee = Callee {
        addr: 0x001b_70d0,
        f: ee::s03::cFEStateNetAccountMem_SetState,
    };
    pub const NET_FE_STATE_DTOR: Callee = Callee {
        addr: 0x001a_85d0,
        f: ee::s03::NetFEState_dtor,
    };
    pub const CFE_LOAD_STATE_SETUP_RESULT_WIDGET: Callee = Callee {
        addr: 0x0024_7ab0,
        f: ee::s06::cFELoadState_SetupResultWidget,
    };
    pub const EVENT_SELECT_ON_WIDGET_ACTION: Callee = Callee {
        addr: 0x0020_18a8,
        f: ee::s05::EventSelect_OnWidgetAction,
    };
    pub const PLAYBACK_MAN_STOP_PLAYBACK: Callee = Callee {
        addr: 0x0026_cd20,
        f: ee::s07::PlaybackMan_StopPlayback,
    };
    pub const PLAYBACK_MAN_STOP_RECORDING: Callee = Callee {
        addr: 0x0026_cdd0,
        f: ee::s07::PlaybackMan_StopRecording,
    };
    pub const REPLAY_CACHE_DTOR: Callee = Callee {
        addr: 0x0026_d0e8,
        f: ee::s07::ReplayCache_dtor,
    };
    pub const SCRIPT_LIST_STOP: Callee = Callee {
        addr: 0x0027_6868,
        f: ee::s07::ScriptList_Stop,
    };
    pub const SCRIPT_ENGINE_EXIT_SCRIPT_MODE: Callee = Callee {
        addr: 0x0027_bf90,
        f: ee::s07::ScriptEngine_ExitScriptMode,
    };
    pub const SCRIPT_ENGINE_GET_SEQ_PROCESS_ID: Callee = Callee {
        addr: 0x0028_2be0,
        f: ee::s07::ScriptEngine_GetSeqProcessId,
    };
    pub const SCRIPT_LIST_ON_SEQ_END: Callee = Callee {
        addr: 0x0027_73a0,
        f: ee::s07::ScriptList_OnSeqEnd,
    };
    pub const SCRIPT_ENGINE_GET_TARGET_OBJECT: Callee = Callee {
        addr: 0x0027_9f18,
        f: ee::s07::ScriptEngine_GetTargetObject,
    };
    pub const SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE: Callee = Callee {
        addr: 0x0028_2020,
        f: ee::s07::ScriptEngine_DestroyScriptDatabase,
    };
    pub const SCRIPT_ENGINE_OBJ_TABLE_DTOR: Callee = Callee {
        addr: 0x0027_ce58,
        f: ee::s07::ScriptEngine_ObjTableDtor,
    };
    pub const SCRIPT_ENGINE_GET_PROCESS: Callee = Callee {
        addr: 0x0028_2cb0,
        f: ee::s07::ScriptEngine_GetProcess,
    };
    pub const IN_SEQ_STOP: Callee = Callee {
        addr: 0x0027_2b58,
        f: ee::s07::InSeq_Stop,
    };
    pub const SND_BANK_UNLOAD_MEMORY: Callee = Callee {
        addr: 0x0028_b7e0,
        f: ee::s07::SndBank_UnloadMemory,
    };
}

/// Cross-shard call targets (the generator emits `call_indirect` for them).
mod table {
    pub const DELETE: u32 = 0x0031_7e50;
    pub const EA_MEM_FREE: u32 = 0x0031_7e98;
    pub const OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;
    pub const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    pub const WAIT_SEMA: u32 = 0x0042_3de0;
    pub const WAKEUP_THREAD: u32 = 0x0042_3cd0;
    pub const SLEEP_THREAD: u32 = 0x0042_3cc0;
    pub const STR_HASH: u32 = 0x0031_7670;
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;
}

// ---------------------------------------------------------------------------
// Small register helpers.

/// The address `reg + off`.
#[inline(always)]
fn p(c: &Cpu, reg: usize, off: u32) -> u32 {
    abi::r32(c, reg).wrapping_add(off)
}

/// A `gp`-relative global address.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    abi::r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Engine and input.

/// `void PS2ExecMan_VSyncThread(PS2ExecMan* m)` (0x31ac08): the vsync thread:
/// wait on the manager's semaphore, tick the app's vsync callback, and loop
/// while the shutdown flag at `+0x20` is still clear. The event check at the
/// loop back edge re-enters the function at 0x31ac20, so this port accepts
/// that resume address too.
pub fn ps2_exec_man_vsync_thread(c: &mut Cpu) {
    loop {
        match c.pc {
            0x0031_ac08 => {
                // Entry: the prologue falls straight through to the
                // `WaitSema` block (the generator's `t` is the constant true,
                // so 0x31ac1c/0x31ac20 are *not* run before the first wait).
                c.cycles += 5;
                let frame = Frame::open(c, 0x20);
                frame.save(c, S0, 0x10);
                frame.save_ra(c, 0);
                c.g[S0][0] = c.g[A0][0];
                c.pc = 0x0031_ac28;
            }
            // Dead fall-through block of the original (`t` is always true);
            // only the trampoline could resume here, and it never does.
            0x0031_ac1c => {
                c.cycles += 1;
                c.pc = 0x0031_ac20;
            }
            0x0031_ac20 => {
                // Registers at the call: a0 = this, ra = 0x31ac28.
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                if callee::APP_VSYNC_CALLBACK.call(c, 0x0031_ac28).is_err() {
                    return;
                }
                c.pc = 0x0031_ac28;
            }
            0x0031_ac28 => {
                c.cycles += 5;
                c.g[A0][0] = Addr(p(c, S0, 0x4034)).lw(c);
                if call_ptr(c, table::WAIT_SEMA, 0x0031_ac30).is_err() {
                    return;
                }
                c.g[V1][0] = Addr(p(c, S0, 0x20)).lw(c);
                if c.g[V1][0] == 0 {
                    if back_edge_unwinds(c, 0x0031_ac20) {
                        return;
                    }
                    c.pc = 0x0031_ac20;
                } else {
                    c.pc = 0x0031_ac3c;
                }
            }
            0x0031_ac3c => {
                c.cycles += 8;
                c.g[A0][0] = Addr(p(c, S0, 0x10)).lw(c);
                if call_ptr(c, table::WAKEUP_THREAD, 0x0031_ac44).is_err() {
                    return;
                }
                if call_ptr(c, table::SLEEP_THREAD, 0x0031_ac4c).is_err() {
                    return;
                }
                let frame = Frame { size: 0x20 };
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x0031_ac08, 0x0031_ac5c);
                return;
            }
        }
    }
}

/// `void InputPreproc_WriteString(InputPreproc* in, const char* s)` (0x326308):
/// call `InputPreproc_Write(in, *s++)` for every character up to the NUL. The
/// back edge re-enters at 0x326328 after a thread switch.
pub fn input_preproc_write_string(c: &mut Cpu) {
    loop {
        match c.pc {
            0x0032_6308 => {
                c.cycles += 7;
                let frame = Frame::open(c, 0x30);
                frame.save(c, S0, 0x20);
                frame.save(c, S1, 0x10);
                c.g[S0][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                c.g[S1][0] = c.g[A0][0];
                c.pc = 0x0032_633c;
            }
            0x0032_6324 => {
                c.cycles += 1;
                c.pc = 0x0032_6328;
            }
            0x0032_6328 => {
                c.cycles += 5;
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                if callee::INPUT_PREPROC_WRITE.call(c, 0x0032_633c).is_err() {
                    return;
                }
                c.pc = 0x0032_633c;
            }
            0x0032_633c => {
                // Registers at the call were set by the previous block.
                c.cycles += 3;
                c.g[V0][0] = Addr(p(c, S0, 0)).lb(c);
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = Addr(p(c, S0, 0)).lbu(c);
                if more {
                    if back_edge_unwinds(c, 0x0032_6328) {
                        return;
                    }
                    c.pc = 0x0032_6328;
                } else {
                    c.pc = 0x0032_6348;
                }
            }
            0x0032_6348 => {
                c.cycles += 5;
                let frame = Frame { size: 0x30 };
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x0032_6308, 0x0032_635c);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Collision.

/// `void CollSphere_Transform(CollSphere* s, const Matrix* m, float scale)`
/// (0x32fd40): transform the sphere centre at `+0x80` by `m` with the VU0
/// macro pipeline, store it at `+0x60`, set `+0x4 = 1` and scale the radius
/// (`+0x90` into `+0x70`).
pub fn coll_sphere_transform(c: &mut Cpu) {
    c.cycles += 21;
    let frame = Frame::open(c, 0x20);
    c.g[V1][0] = 1;
    c.lqc2(8, p(c, A0, 0x80));
    c.lqc2(4, abi::r32(c, A1));
    c.lqc2(5, p(c, A1, 0x10));
    c.lqc2(6, p(c, A1, 0x20));
    c.lqc2(7, p(c, A1, 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, frame.at(c, 0x10));
    c.lwc1(0, p(c, A0, 0x90));
    c.lq(2, frame.at(c, 0x10));
    c.mul_s(12, 12, 0);
    c.sw(p(c, A0, 4), c.g[V1][0]);
    c.sq(2, p(c, A0, 0x60));
    c.sq(2, frame.at(c, 0));
    c.swc1(12, p(c, A0, 0x70));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(abi::r32(c, SP).wrapping_add(0x20));
    c.pc = ra;
}

/// `void CollSphereTree_dtor(CollSphereTree* t, int flags)` (0x3304e8): set
/// the vtable, delete the child array at `+0x68`, and `delete this` when bit
/// 0 of `flags` is set.
pub fn coll_sphere_tree_dtor(c: &mut Cpu) {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0048_e5f0);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.sw(p(c, S1, 0x50), c.g[V0][0]);
    c.g[A0][0] = Addr(p(c, S1, 0x68)).lw(c);
    if call_ptr(c, table::DELETE, 0x0033_0514).is_err() {
        return;
    }
    c.g[S0][0] &= 1;
    let keep = c.g[S0][0] != 0;
    frame.restore(c, S0, 0x20);
    if keep {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, table::DELETE, 0x0033_0528).is_err() {
            return;
        }
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// World objects and particles.

/// `Counter* Counter_ctor(Counter* self, void* a1, void* a2, void* a3)`
/// (0x343768): `WorldObj_ctor(self, a1, 7, a2)`, then the Counter vtable,
/// the config word from `a3+4`, and a cleared counter at `+0x20`.
pub fn counter_ctor(c: &mut Cpu) {
    c.cycles += 21;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    c.g[S1][0] = c.g[A3][0];
    c.g[S0][0] = c.g[A0][0];
    c.g[A3][0] = c.g[A2][0];
    frame.save_ra(c, 0);
    c.g[A2][0] = 7;
    if callee::WORLD_OBJ_CTOR.call(c, 0x0034_378c).is_err() {
        return;
    }
    c.g[V1][0] = sx(0x0049_03f0);
    c.g[V0][0] = c.g[S0][0];
    frame.restore_ra(c, 0);
    c.sw(p(c, S0, 0xc), c.g[V1][0]);
    c.g[A0][0] = Addr(p(c, S1, 4)).lh(c);
    c.sw(p(c, S0, 0x20), 0);
    c.sw(p(c, S0, 0x1c), c.g[A0][0]);
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.close(c);
}

/// `void DeadFade_dtor(DeadFade* f, int flags)` (0x350be0): set the vtable,
/// free the fade node at `+0x38` and run the WorldObj destructor.
pub fn dead_fade_dtor(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0049_1980);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.sw(p(c, S0, 0xc), c.g[V0][0]);
    c.g[A0][0] = Addr(p(c, S0, 0x38)).lw(c);
    let null = c.g[A0][0] == 0;
    c.g[S1][0] = c.g[A1][0];
    if !null {
        c.cycles += 2;
        if call_ptr(c, table::EA_MEM_FREE, 0x0035_0c14).is_err() {
            return;
        }
    }
    c.cycles += 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    if callee::WORLD_OBJ_DTOR.call(c, 0x0035_0c20).is_err() {
        return;
    }
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void ModObj_SetTexFlipModifier(ModObj* self, TexFlipModifier* v)`
/// (0x3555a8): make sure the modifier block exists, clear the old tex-flip
/// modifier and install `v` in slot `+0x8`. Returns the block in v0.
pub fn mod_obj_set_tex_flip_modifier(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = Addr(p(c, S0, 0x1c)).lw(c);
    let have = c.g[A0][0] != 0;
    c.g[S1][0] = c.g[A1][0];
    if have {
        c.cycles += 3;
        if callee::MODIFIER_BLOCK_FREE_TEX_FLIP
            .call(c, 0x0035_55e0)
            .is_err()
        {
            return;
        }
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        if callee::MOD_OBJ_CREATE_MODIFIER_BLOCK
            .call(c, 0x0035_55d0)
            .is_err()
        {
            return;
        }
    }
    c.g[V0][0] = Addr(p(c, S0, 0x1c)).lw(c);
    c.cycles += 6;
    frame.restore(c, S0, 0x20);
    c.sw(abi::r32(c, V0).wrapping_add(8), c.g[S1][0]);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void ModObj_SetBoostModifier(ModObj* self, BoostModifier* v)` (0x355600):
/// the same shape as the tex-flip setter, in slot `+0xc`.
pub fn mod_obj_set_boost_modifier(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = Addr(p(c, S0, 0x1c)).lw(c);
    let have = c.g[A0][0] != 0;
    c.g[S1][0] = c.g[A1][0];
    if have {
        c.cycles += 3;
        if callee::MODIFIER_BLOCK_FREE_BOOST
            .call(c, 0x0035_5638)
            .is_err()
        {
            return;
        }
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        if callee::MOD_OBJ_CREATE_MODIFIER_BLOCK
            .call(c, 0x0035_5628)
            .is_err()
        {
            return;
        }
    }
    c.g[V0][0] = Addr(p(c, S0, 0x1c)).lw(c);
    c.cycles += 6;
    frame.restore(c, S0, 0x20);
    c.sw(abi::r32(c, V0).wrapping_add(0xc), c.g[S1][0]);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void MultiParticleMan_Clear(MultiParticle** list)` (0x357b38): destroy and
/// clear the first four particle slots.
pub fn multi_particle_man_clear(c: &mut Cpu) {
    c.cycles += 6;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = 3;
    loop {
        c.cycles += 5;
        c.g[V0][0] = Addr(p(c, S0, 0)).lw(c);
        c.g[A1][0] = 3;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
        let null = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[V0][0];
        if !null {
            c.cycles += 2;
            if callee::MULTI_PARTICLE_DTOR.call(c, 0x0035_7b6c).is_err() {
                return;
            }
        }
        c.cycles += 3;
        c.sw(p(c, S0, 0), 0);
        let again = (c.g[S1][0] as i64) >= 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        if !again {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0035_7b50);
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void DynamicEmitter_dtor(DynamicEmitter* e, int flags)` (0x370c08): set
/// the vtable, free the emitter's data and `delete this` for bit 0.
pub fn dynamic_emitter_dtor(c: &mut Cpu) {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0049_3118);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.sw(p(c, S1, 0x1f8), c.g[V0][0]);
    c.g[A1][0] = 1;
    if callee::DYNAMIC_EMITTER_FREE_DATA
        .call(c, 0x0037_0c34)
        .is_err()
    {
        return;
    }
    c.g[S0][0] &= 1;
    let keep = c.g[S0][0] != 0;
    frame.restore(c, S0, 0x20);
    if keep {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, table::DELETE, 0x0037_0c48).is_err() {
            return;
        }
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void BoaderShadow_AllocTexture(BoaderShadow* s)` (0x375990): if the
/// texture handle at `+0x170` is negative (not yet allocated), allocate it
/// through the graphics manager's `v52` with the stored size/format.
pub fn boader_shadow_alloc_texture(c: &mut Cpu) {
    c.cycles += 7;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[V0][0] = Addr(p(c, S0, 0x170)).lw(c);
    if (c.g[V0][0] as i64) < 0 {
        c.cycles += 11;
        c.g[T0][0] = Addr(gp(c, 0x2a90)).lw(c);
        c.g[A3][0] = 2;
        c.g[A1][0] = Addr(p(c, S0, 0x174)).lw(c);
        c.g[V1][0] = Addr(p(c, T0, 0x10d8)).lw(c);
        c.g[A2][0] = Addr(p(c, S0, 0x178)).lw(c);
        c.g[A0][0] = Addr(p(c, V1, 0x1a0)).lh(c);
        c.g[V0][0] = Addr(p(c, V1, 0x1a4)).lw(c);
        let target = abi::r32(c, V0);
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if call_ptr(c, target, 0x0037_59d0).is_err() {
            return;
        }
        c.sw(p(c, S0, 0x170), c.g[V0][0]);
    }
    c.cycles += 3;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// UI widgets.

/// `void UIScreen_CmdGotoLabel(UIScreen* s, void* key, void* out)` (0x39cbe0):
/// walk the screen's label list (count at `+0`, entries `{label, stride,
/// value}` at `+4`) and store the matching entry's `u16` at `out+0xc`.
pub fn ui_screen_cmd_goto_label(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T1][0] = c.g[A0][0];
    c.g[T0][0] = 0;
    c.g[V1][0] = Addr(p(c, T1, 0x34)).lw(c);
    c.g[V0][0] = Addr(p(c, V1, 0)).lw(c);
    let empty = c.g[V0][0] == 0;
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
    if !empty {
        loop {
            c.cycles += 4;
            c.g[V1][0] = Addr(p(c, A1, 0)).lw(c);
            c.g[V0][0] = Addr(p(c, A2, 4)).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A0][0] = Addr(p(c, T1, 0x34)).lw(c);
                c.cycles += 6;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                c.g[V1][0] = Addr(p(c, A1, 4)).lw(c);
                c.g[V0][0] = Addr(p(c, A0, 0)).lw(c);
                c.g[V0][0] = (c.g[T0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_cbf8);
                }
                continue;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c, A1, 8)).lhu(c);
            c.sh(p(c, A3, 0xc), c.g[V0][0]);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void UIText_FreeStrings(UIText* t)` (0x3a0b38): clear the string count and
/// free the two string buffers at `+0xac` and `+0xb4`.
pub fn ui_text_free_strings(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.sw(p(c, S0, 0xa8), 0);
    c.g[A0][0] = Addr(p(c, S0, 0xac)).lw(c);
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        if call_ptr(c, table::EA_MEM_FREE, 0x003a_0b60).is_err() {
            return;
        }
        c.sw(p(c, S0, 0xac), 0);
    }
    c.g[A0][0] = Addr(p(c, S0, 0xb4)).lw(c);
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        if call_ptr(c, table::EA_MEM_FREE, 0x003a_0b78).is_err() {
            return;
        }
        c.sw(p(c, S0, 0xb4), 0);
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `cHullPage* cHullPage_ctor(cHullPage* p, const u16* data)` (0x3a7bd8): keep
/// the vertex count from `*data`, allocate `4 * count` bytes of index space.
pub fn c_hull_page_ctor(c: &mut Cpu) {
    c.cycles += 21;
    let frame = Frame::open(c, 0x20);
    c.g[V1][0] = c.g[A1][0];
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.sw(p(c, S0, 0), c.g[V1][0]);
    c.g[A1][0] = sx(0x0049_4e00);
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[V0][0] = Addr(p(c, V1, 0)).lhu(c);
    c.sw(p(c, S0, 8), 0);
    c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
    frame.save_ra(c, 0);
    c.sw(p(c, S0, 4), c.g[V0][0]);
    if call_ptr(c, table::OPERATOR_NEW_NAMED, 0x003a_7c14).is_err() {
        return;
    }
    c.sw(p(c, S0, 0xc), c.g[V0][0]);
    c.g[V0][0] = c.g[S0][0];
    frame.restore_ra(c, 0);
    frame.restore(c, S0, 0x10);
    frame.close(c);
}

/// `void cHullPage_dtor(cHullPage* p, int flags)` (0x3a7c30): clear the count,
/// free the index array and `delete this` for bit 0.
pub fn c_hull_page_dtor(c: &mut Cpu) {
    c.cycles += 9;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.sw(p(c, S0, 4), 0);
    c.g[A0][0] = Addr(p(c, S0, 0xc)).lw(c);
    let null = c.g[A0][0] == 0;
    c.g[S1][0] = c.g[A1][0];
    if !null {
        c.cycles += 2;
        if call_ptr(c, table::EA_MEM_FREE, 0x003a_7c5c).is_err() {
            return;
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[S1][0] & 1;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        if call_ptr(c, table::DELETE, 0x003a_7c70).is_err() {
            return;
        }
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void RCMP_DECODER_dtor(RCMP_DECODER* d, int flags)` (0x3b04a0): set the
/// vtable, release the codec and call the global destructor for bit 0.
pub fn rcmp_decoder_dtor(c: &mut Cpu) {
    c.cycles += 13;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0045_68a8);
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A1][0];
    c.sw(p(c, S1, 0x18), c.g[V0][0]);
    if callee::RCMP_DECODER_RELEASE_CODEC
        .call(c, 0x003b_04c8)
        .is_err()
    {
        return;
    }
    c.g[S0][0] &= 1;
    let keep = c.g[S0][0] != 0;
    c.g[V1][0] = 0x0051_0000;
    if keep {
        c.cycles += 3;
        c.g[V0][0] = Addr(p(c, V1, 0u32.wrapping_sub(0x6bcc))).lw(c);
        let target = abi::r32(c, V0);
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, target, 0x003b_04e0).is_err() {
            return;
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Riders.

/// `void Rider_AwardBonusBoost(Rider* r)` (0x10e8b8): award the bonus points
/// for the metrix at `+0x790`, add 0x10 boost, and play the bonus sound on
/// the audio system (`3, r, 1.0f`).
pub fn rider_award_bonus_boost(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = Addr(p(c, S0, 0x790)).lw(c);
    if callee::RIDER_METRIX_AWARD_BONUS_POINTS
        .call(c, 0x0010_e8d0)
        .is_err()
    {
        return;
    }
    c.mov_s(12, 0);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0x10;
    if callee::RIDER_ADD_BOOST.call(c, 0x0010_e8e0).is_err() {
        return;
    }
    if call_ptr(c, table::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0010_e8e8).is_err() {
        return;
    }
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 12);
    c.g[A2][0] = c.g[S0][0];
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 3;
    if call_ptr(c, 0x0029_ced8, 0x0010_e900).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void RiderMetrix_OnMissionCompleted(RiderMetrix* m, int n)` (0x11a110):
/// show the HUD number (1.5f), award 4 points and return 0.0f.
pub fn rider_metrix_on_mission_completed(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x30);
    c.g[AT][0] = 0x3fc0_0000;
    c.mtc1(1, 12);
    frame.save(c, S0, 0x20);
    c.g[A3][0] = 0;
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = 0x1b;
    if callee::RIDER_METRIX_SHOW_HUD_NUMBER
        .call(c, 0x0011_a140)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = 4;
    if callee::RIDER_METRIX_AWARD_POINTS
        .call(c, 0x0011_a150)
        .is_err()
    {
        return;
    }
    c.mtc1(0, 0);
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void AI_SetupActorRiders(AI* ai, int n)` (0x128998): clear the BE
/// interface, and unless the "computer players" flag is set, store `n` and
/// spawn the computer actor riders.
pub fn ai_setup_actor_riders(c: &mut Cpu) {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    if call_ptr(c, 0x0014_dc80, 0x0012_89b4).is_err() {
        return;
    }
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    if call_ptr(c, 0x0014_dd58, 0x0012_89c0).is_err() {
        return;
    }
    c.g[V1][0] = 0x0053_0000;
    c.g[V0][0] = Addr(p(c, V1, 0x5c11)).lb(c);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.sw(p(c, S0, 0x94), c.g[S1][0]);
        c.g[A0][0] = c.g[S0][0];
        if callee::AI_CREATE_ACTOR_RIDERS.call(c, 0x0012_89dc).is_err() {
            return;
        }
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void BENewPlayerInt_SetSlotFlag3(BENewPlayerInt* p, int index, int on)`
/// (0x1475c0): set bit 3 of the flag word of player `index`'s character entry.
pub fn be_new_player_int_set_slot_flag3(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    c.g[A0][0] = c.g[A1][0];
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A2][0];
    if callee::BE_NEW_PLAYER_INT_GET_SLOT
        .call(c, 0x0014_75d8)
        .is_err()
    {
        return;
    }
    c.g[V1][0] = 0x1c;
    c.g[A0][0] = 0x0053_0000;
    c.mult(0, V0, V0, V1);
    c.g[A0][0] = sx(0x0053_0000u32.wrapping_add(0x4fe0));
    c.g[A1][0] = 0xffff_ffff_ffff_fff7;
    c.g[S0][0] &= 1;
    c.g[S0][0] = sx((c.g[S0][0] as u32) << 3);
    frame.restore_ra(c, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(p(c, V0, 0x10)).lw(c);
    c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
    c.g[V1][0] = c.g[V1][0] | c.g[S0][0];
    frame.restore(c, S0, 0x10);
    c.sw(p(c, V0, 0x10), c.g[V1][0]);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Camera.

/// `void GameCamera_ResetAllChaseControllers(void)` (0x15dc18): reset the
/// primary chase controller and, when the camera has two, the secondary one.
pub fn game_camera_reset_all_chase_controllers(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x20);
    c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(p(c, V0, 0x84)).lw(c);
    c.g[S0][0] = Addr(p(c, V1, 0x84)).lw(c);
    if c.g[S0][0] != 0 {
        c.cycles += 7;
        c.g[V0][0] = Addr(p(c, S0, 4)).lw(c);
        c.g[A0][0] = Addr(p(c, V0, 0xa8)).lw(c);
        if callee::CHASE_CAM_CONTROLLER_RESET_STATE
            .call(c, 0x0015_dc44)
            .is_err()
        {
            return;
        }
        c.g[V0][0] = Addr(p(c, S0, 0)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(p(c, S0, 8)).lw(c);
            c.g[A0][0] = Addr(p(c, V0, 0xa8)).lw(c);
            if callee::CHASE_CAM_CONTROLLER_RESET_STATE
                .call(c, 0x0015_dc60)
                .is_err()
            {
                return;
            }
        }
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void CameraTriggerMan_Unload(CameraTriggerMan* m)` (0x16c9b0): purge both
/// active lists and both stacks, remove every trigger, clear the data and
/// reset the chase controllers.
pub fn camera_trigger_man_unload(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
    if callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR
        .call(c, 0x0016_c9c8)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
    if callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR
        .call(c, 0x0016_c9d0)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
    if callee::CAMERA_TRIGGER_STACK_CLEAR
        .call(c, 0x0016_c9d8)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3c));
    if callee::CAMERA_TRIGGER_STACK_CLEAR
        .call(c, 0x0016_c9e0)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    if callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS
        .call(c, 0x0016_c9e8)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    if callee::CAMERA_TRIGGER_MAN_CLEAR_DATA
        .call(c, 0x0016_c9f0)
        .is_err()
    {
        return;
    }
    if call_ptr(c, 0x0015_dc18, 0x0016_c9f8).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Front end.

/// `cFEOptionsCredits* cFEOptionsCredits_ctor(void* self)` (0x185a18): the
/// credits screen: `UIObject_ctor`, vtable, and the default field values.
pub fn cfe_options_credits_ctor(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    if call_ptr(c, 0x0039_e2a0, 0x0018_5a2c).is_err() {
        return;
    }
    c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
    c.g[V1][0] = sx(0x0046_bdf8);
    c.sw(p(c, S0, 0x60), c.g[V0][0]);
    c.g[A0][0] = 1;
    c.g[A1][0] = 0x24;
    c.sw(p(c, S0, 8), c.g[V1][0]);
    c.g[V0][0] = c.g[S0][0];
    c.sw(p(c, S0, 0x64), c.g[A0][0]);
    c.sw(p(c, S0, 0xc), c.g[A1][0]);
    c.sw(p(c, S0, 0x58), 0);
    c.sw(p(c, S0, 0x5c), 0);
    c.sw(p(c, S0, 0x48), 0);
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
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

/// `cFEStateRuleSelect* cFEStateRuleSelect_ctor(void* self)` (0x191360): the
/// rule-select state: `UIState_ctor`, vtable, the string-type byte from the
/// first UI layer, and the state id 0x14.
pub fn cfe_state_rule_select_ctor(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    if call_ptr(c, 0x0039_e2a0, 0x0019_1374).is_err() {
        return;
    }
    c.g[V1][0] = sx(0x0046_b090);
    c.sw(p(c, S0, 0x48), 0);
    c.sb(p(c, S0, 0x44), 0);
    c.sw(p(c, S0, 8), c.g[V1][0]);
    c.g[A1][0] = 0;
    c.g[V0][0] = Addr(p(c, S0, 0x10)).lw(c);
    c.g[A0][0] = Addr(p(c, V0, 0)).lw(c);
    if call_ptr(c, 0x001a_1cd0, 0x0019_1398).is_err() {
        return;
    }
    c.sb(p(c, S0, 0x15), c.g[V0][0]);
    c.g[V0][0] = 0x14;
    frame.restore_ra(c, 0);
    c.sw(p(c, S0, 0xc), c.g[V0][0]);
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S0, 0x10);
    frame.close(c);
}

/// `void* FELocale_v01(FELocale* l, int flags)` (0x1958f0): the deleting
/// destructor of the locale object: set the base vtable, unload every table,
/// switch to the base vtable and `delete` for bit 0.
pub fn fe_locale_v01(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0046_bf98);
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A1][0];
    c.sw(p(c, S1, 4), c.g[V0][0]);
    if callee::FE_LOCALE_UNLOAD_ALL_TABLES
        .call(c, 0x0019_5918)
        .is_err()
    {
        return;
    }
    c.g[V0][0] = sx(0x0046_d848);
    c.g[S0][0] &= 1;
    let keep = c.g[S0][0] != 0;
    c.sw(p(c, S1, 4), c.g[V0][0]);
    if keep {
        c.cycles += 2;
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, table::DELETE, 0x0019_5934).is_err() {
            return;
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cFEStateNetAccountMem_UpdateState02(cFEStateNetAccountMem* s)`
/// (0x1b77c8): leave the sub-state machine of the "account memory" screen:
/// state 4 is accepted directly, state 5 depending on `+0x6d4`, anything else
/// is ignored.
pub fn cfe_state_net_account_mem_update_state02(c: &mut Cpu) {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    c.g[V1][0] = Addr(gp(c, 0x2e0)).lw(c);
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(p(c, V1, 0)).lw(c);
    let mode = abi::r32(c, V1);
    c.g[V0][0] = c.g[V1][0] ^ 0x4;
    let is4 = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[V1][0] ^ 0x5;
    // 0: nothing, 1: state 4, 2: state 3.
    let mut which = 0u8;
    if is4 {
        which = 1;
    } else {
        c.cycles += 2;
        let not5 = (mode ^ 5) != 0;
        frame.restore_ra(c, 0);
        if not5 {
            // Return without touching the state.
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(abi::arg(c, 0)).off(0x6d4).lw(c);
            let field = abi::r32(c, V1);
            c.g[V0][0] = 1;
            which = if field != 1 { 2 } else { 1 };
        }
    }
    if which == 1 {
        c.cycles += 4;
        c.g[RA][0] = sx(0x001b_7804);
        c.g[A1][0] = 4;
        if callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE
            .call(c, 0x001b_7804)
            .is_err()
        {
            return;
        }
        frame.restore_ra(c, 0);
    } else if which == 2 {
        c.cycles += 3;
        c.g[RA][0] = sx(0x001b_7814);
        c.g[A1][0] = 3;
        if callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE
            .call(c, 0x001b_7814)
            .is_err()
        {
            return;
        }
        frame.restore_ra(c, 0);
    }
    c.cycles += 2;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cFEStateLogin_dtor(cFEStateLogin* s, int flags)` (0x1bee78): set the
/// vtable, free the buffer at `+0x6d0`, then run the NetFEState destructor.
pub fn cfe_state_login_dtor(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0046_8388);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.sw(p(c, S0, 8), c.g[V0][0]);
    c.g[A0][0] = Addr(p(c, S0, 0x6d0)).lw(c);
    let null = c.g[A0][0] == 0;
    c.g[S1][0] = c.g[A1][0];
    if !null {
        c.cycles += 3;
        if call_ptr(c, table::EA_MEM_FREE, 0x001b_eeac).is_err() {
            return;
        }
        c.sw(p(c, S0, 0x6d0), 0);
    }
    c.cycles += 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    if callee::NET_FE_STATE_DTOR.call(c, 0x001b_eebc).is_err() {
        return;
    }
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cFEStateOnlineAgreement_dtor(cFEStateOnlineAgreement* s, int flags)`
/// (0x1c0a28): the same shape as the login-state destructor.
pub fn cfe_state_online_agreement_dtor(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0046_ac38);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.sw(p(c, S0, 8), c.g[V0][0]);
    c.g[A0][0] = Addr(p(c, S0, 0x6d0)).lw(c);
    let null = c.g[A0][0] == 0;
    c.g[S1][0] = c.g[A1][0];
    if !null {
        c.cycles += 3;
        if call_ptr(c, table::EA_MEM_FREE, 0x001c_0a5c).is_err() {
            return;
        }
        c.sw(p(c, S0, 0x6d0), 0);
    }
    c.cycles += 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    if callee::NET_FE_STATE_DTOR.call(c, 0x001c_0a6c).is_err() {
        return;
    }
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cFEStateNetNews_OnWidgetCreated(cFEStateNetNews* s, UIWidget* w)`
/// (0x1c5310): when the created widget is the news list, enable its second
/// behaviour.
pub fn cfe_state_net_news_on_widget_created(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    c.g[A0][0] = sx(0x0046_5f48);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S0, 0x20);
    c.g[S0][0] = Addr(p(c, S1, 0x38)).lw(c);
    if call_ptr(c, table::STR_HASH, 0x001c_5334).is_err() {
        return;
    }
    let same = c.g[S0][0] == c.g[V0][0];
    frame.restore(c, S0, 0x20);
    if same {
        c.cycles += 7;
        c.g[V0][0] = Addr(p(c, S1, 8)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(p(c, V0, 0x38)).lh(c);
        c.g[V1][0] = Addr(p(c, V0, 0x3c)).lw(c);
        let target = abi::r32(c, V1);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if call_ptr(c, target, 0x001c_5354).is_err() {
            return;
        }
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cUIStateBuyPopup_OnWidgetAction(cUIStateBuyPopup* s, UIWidget* w,
/// int event)` (0x1caf00): when the "purchase" button is released, set the
/// confirmation flag at `+0x6c`.
pub fn cui_state_buy_popup_on_widget_action(c: &mut Cpu) {
    c.cycles += 6;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    let mut skip_one = false;
    if c.g[A1][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = 5;
        let not_purchase = c.g[A2][0] != c.g[V0][0];
        frame.restore(c, S0, 0x20);
        if not_purchase {
            skip_one = true;
        } else {
            c.cycles += 6;
            c.g[A0][0] = sx(0x004a_14e8);
            c.g[S0][0] = Addr(p(c, A1, 0x38)).lw(c);
            if call_ptr(c, table::STR_HASH, 0x001c_af34).is_err() {
                return;
            }
            if c.g[S0][0] != c.g[V0][0] {
                c.sw(p(c, S1, 0x6c), 0);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                c.sw(p(c, S1, 0x6c), c.g[V0][0]);
            }
        }
    }
    if !skip_one {
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cFEStateRelogin_OnWidgetCreated(cFEStateRelogin* s, UIWidget* w)`
/// (0x1d50e0): when the created widget is the retry button, give it the
/// "please wait" label.
pub fn cfe_state_relogin_on_widget_created(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    c.g[A0][0] = sx(0x0046_7790);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S0, 0x20);
    c.g[S0][0] = Addr(p(c, S1, 0x38)).lw(c);
    if call_ptr(c, table::STR_HASH, 0x001d_5104).is_err() {
        return;
    }
    let same = c.g[S0][0] == c.g[V0][0];
    frame.restore(c, S0, 0x20);
    if same {
        c.cycles += 7;
        c.g[A0][0] = sx(0x0046_1cc0);
        if call_ptr(c, table::STR_HASH, 0x001d_5118).is_err() {
            return;
        }
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        if call_ptr(c, 0x003a_1000, 0x001d_5124).is_err() {
            return;
        }
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void OVScreenMap_OnWidgetAction(OVScreenMap* s, ...)` (0x2085b8): forward
/// to the event-select widget and, when it accepted, show the map legend.
pub fn ov_screen_map_on_widget_action(c: &mut Cpu) {
    c.cycles += 9;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
    if callee::EVENT_SELECT_ON_WIDGET_ACTION
        .call(c, 0x0020_85d0)
        .is_err()
    {
        return;
    }
    c.g[V1][0] = 1;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 10;
        c.g[A0][0] = Addr(p(c, S0, 0x10)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        if call_ptr(c, 0x0039_f190, 0x0020_85ec).is_err() {
            return;
        }
        c.g[V1][0] = Addr(p(c, S0, 0x1c)).lw(c);
        c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[V1][0] = c.g[V1][0] | 0x780;
        c.sw(p(c, S0, 0x1c), c.g[V1][0]);
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Start screen, movie manager.

/// `void StartScreen_Unload(StartScreen* s)` (0x232d50): free the world's
/// loaded screen data, release the screen texture and unload the world.
pub fn start_screen_unload(c: &mut Cpu) {
    c.cycles += 13;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = Addr(gp(c, -0x854)).lw(c);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[V0][0] = Addr(p(c, A1, 0x10d8)).lw(c);
    c.g[A0][0] = Addr(p(c, V0, 0x98)).lh(c);
    c.g[V1][0] = Addr(p(c, V0, 0x9c)).lw(c);
    let target = abi::r32(c, V1);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    if call_ptr(c, target, 0x0023_2d78).is_err() {
        return;
    }
    c.g[A0][0] = Addr(p(c, S0, 0xc)).lw(c);
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.g[A1][0] = 3;
        if call_ptr(c, 0x0039_7b70, 0x0023_2d8c).is_err() {
            return;
        }
    }
    c.sw(p(c, S0, 0xc), 0);
    c.cycles += 6;
    c.g[A0][0] = Addr(gp(c, -0x854)).lw(c);
    if call_ptr(c, 0x0036_a020, 0x0023_2d98).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void MCManager_State35(MCManager* m)` (0x23e2c8): when the movie is
/// finished, tell the 3D crowd manager to stop (message 0x33).
pub fn mc_manager_state35(c: &mut Cpu) {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A1][0] = Addr(p(c, S0, 0x434)).lw(c);
    c.g[V1][0] = Addr(p(c, A1, 0)).lw(c);
    c.g[A0][0] = Addr(p(c, V1, 0x58)).lh(c);
    c.g[V0][0] = Addr(p(c, V1, 0x5c)).lw(c);
    let target = abi::r32(c, V0);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    if call_ptr(c, target, 0x0023_e2f0).is_err() {
        return;
    }
    if c.g[V0][0] == 0 {
        c.cycles += 7;
        c.g[V0][0] = Addr(p(c, S0, 0x748)).lw(c);
        c.g[A1][0] = 0x33;
        c.g[A0][0] = Addr(p(c, V0, 8)).lh(c);
        c.g[V1][0] = Addr(p(c, V0, 0xc)).lw(c);
        let target = abi::r32(c, V1);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if call_ptr(c, target, 0x0023_e310).is_err() {
            return;
        }
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void MCManager_State33(MCManager* m)` (0x23e498): the same "movie
/// finished" step, with message 0x34.
pub fn mc_manager_state33(c: &mut Cpu) {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A1][0] = Addr(p(c, S0, 0x434)).lw(c);
    c.g[V1][0] = Addr(p(c, A1, 0)).lw(c);
    c.g[A0][0] = Addr(p(c, V1, 0x58)).lh(c);
    c.g[V0][0] = Addr(p(c, V1, 0x5c)).lw(c);
    let target = abi::r32(c, V0);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    if call_ptr(c, target, 0x0023_e4c0).is_err() {
        return;
    }
    if c.g[V0][0] == 0 {
        c.cycles += 7;
        c.g[V0][0] = Addr(p(c, S0, 0x748)).lw(c);
        c.g[A1][0] = 0x34;
        c.g[A0][0] = Addr(p(c, V0, 8)).lh(c);
        c.g[V1][0] = Addr(p(c, V0, 0xc)).lw(c);
        let target = abi::r32(c, V1);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if call_ptr(c, target, 0x0023_e4e0).is_err() {
            return;
        }
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cFELoadState_OnWidgetCreated(cFELoadState* s, UIWidget* w)`
/// (0x245cd0): set up the result widget unless the load is already running
/// in the "computer players" mode.
pub fn cfe_load_state_on_widget_created(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    if call_ptr(c, 0x0014_dc80, 0x0024_5cec).is_err() {
        return;
    }
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    if call_ptr(c, 0x0014_dd58, 0x0024_5cf8).is_err() {
        return;
    }
    c.g[V1][0] = 0x0053_0000;
    c.g[A0][0] = 2;
    c.g[V0][0] = Addr(p(c, V1, 0x5c11)).lb(c);
    let mode = c.g[V0][0] == c.g[A0][0];
    c.g[A0][0] = c.g[S0][0];
    if !mode {
        c.cycles += 2;
        c.g[A1][0] = c.g[S1][0];
        if callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET
            .call(c, 0x0024_5d14)
            .is_err()
        {
            return;
        }
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 5;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Shared login / voip.

/// `void cSharedLoginMan_DoLoad(cSharedLoginMan* m)` (0x265d98): clear the
/// pending request, reset the shared buffer and mark the manager loaded.
pub fn cshared_login_man_do_load(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x30);
    c.g[A1][0] = 0;
    frame.save(c, S0, 0x20);
    c.g[A2][0] = 0x24;
    frame.save(c, S1, 0x10);
    c.g[S0][0] = sx(0x0053_6760);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.sw(p(c, S1, 4), 0);
    c.g[A0][0] = c.g[S0][0];
    c.sw(gp(c, 0x2f0), 0);
    if call_ptr(c, 0x003e_6448, 0x0026_5dcc).is_err() {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    if call_ptr(c, 0x003f_8000, 0x0026_5dd4).is_err() {
        return;
    }
    c.g[V1][0] = 3;
    frame.restore(c, S0, 0x20);
    c.sw(p(c, S1, 0), c.g[V1][0]);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cSharedLoginMan_McDeletePath(void* name, int code)` (0x266240):
/// format the memory-card error, copy the path into a local buffer, show the
/// error and set the login state to 0xd.
pub fn cshared_login_man_mc_delete_path(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x90);
    frame.save(c, S1, 0x70);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x80);
    c.g[A0][0] = c.g[A1][0];
    frame.save_ra(c, 0x60);
    c.g[A1][0] = c.g[SP][0];
    if call_ptr(c, 0x003f_a1d0, 0x0026_6260).is_err() {
        return;
    }
    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = c.g[S0][0];
    if call_ptr(c, 0x0041_66f4, 0x0026_6270).is_err() {
        return;
    }
    c.g[A0][0] = Addr(gp(c, 0x2e8)).lw(c);
    c.g[A1][0] = c.g[S0][0];
    if call_ptr(c, 0x002c_6ce0, 0x0026_627c).is_err() {
        return;
    }
    c.g[V0][0] = 0xd;
    frame.restore(c, S0, 0x80);
    frame.restore(c, S1, 0x70);
    c.g[RA][0] = Addr(p(c, SP, 0x60)).ld(c);
    c.sw(gp(c, 0x2e4), c.g[V0][0]);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = ra;
}

/// `void cVoipMan_Activate(cVoipMan* v)` (0x266e30): start the voice service
/// once it is configured and at least 10 units of time have passed.
pub fn cvoip_man_activate(c: &mut Cpu) {
    c.cycles += 7;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[V0][0] = Addr(p(c, S0, 0)).lw(c);
    let mut started = false;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(p(c, S0, 4)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(gp(c, 0x308)).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < 10) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 5;
                if call_ptr(c, 0x003f_6b20, 0x0026_6e70).is_err() {
                    return;
                }
                c.g[V0][0] = 1;
                c.sw(p(c, S0, 4), c.g[V0][0]);
                started = true;
            }
        }
    }
    let _ = started;
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Replay.

/// `void PlaybackMan_Stop(PlaybackMan* p)` (0x26cdf8): stop playback (state 1)
/// or recording (state 2); any other state is left alone.
pub fn playback_man_stop(c: &mut Cpu) {
    c.cycles += 6;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 1;
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(p(c, A0, 0x70)).lw(c);
    let is1 = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
    if is1 {
        c.cycles += 4;
        if callee::PLAYBACK_MAN_STOP_PLAYBACK
            .call(c, 0x0026_ce34)
            .is_err()
        {
            return;
        }
    } else {
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            // States below 2 other than 1: nothing to do.
        } else {
            c.cycles += 3;
            c.g[V0][0] = 2;
            let is2 = c.g[V1][0] == c.g[V0][0];
            if is2 {
                c.cycles += 3;
                if callee::PLAYBACK_MAN_STOP_RECORDING
                    .call(c, 0x0026_ce44)
                    .is_err()
                {
                    return;
                }
            } else {
                c.cycles += 2;
            }
        }
    }
    c.cycles += 2;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void Player_FreeReplayCache(Player* p, void* cache)` (0x2700d8): find
/// `cache` in the player's two replay slots, destroy it and clear the slot.
pub fn player_free_replay_cache(c: &mut Cpu) {
    c.cycles += 4;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    let found_at = c.g[A1][0];
    frame.save_ra(c, 0);
    if found_at != 0 {
        c.cycles += 2;
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48c));
        c.g[V1][0] = 0;
        loop {
            c.cycles += 3;
            c.g[A0][0] = Addr(p(c, S0, 0)).lw(c);
            if c.g[A0][0] != c.g[A1][0] {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.sw(p(c, S0, 0), 0);
                    break;
                }
                c.cycles += 4;
                c.g[A1][0] = 3;
                if callee::REPLAY_CACHE_DTOR.call(c, 0x0027_010c).is_err() {
                    return;
                }
                c.sw(p(c, S0, 0), 0);
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_00f0);
            }
        }
    }
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void Replay_AllocSaveBuffers(Replay* r)` (0x270f20): allocate the 16 KiB
/// replay buffer and the 50 KiB save buffer.
pub fn replay_alloc_save_buffers(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = sx(0x0048_1800);
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = 0x4000;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    if call_ptr(c, table::EA_MEM_ALLOC_NAMED, 0x0027_0f48).is_err() {
        return;
    }
    c.g[A1][0] = sx(0x0048_1810);
    c.sw(p(c, S0, 0x63c), c.g[V0][0]);
    c.g[A0][0] = 0xc800;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    if call_ptr(c, table::OPERATOR_NEW_NAMED, 0x0027_0f64).is_err() {
        return;
    }
    c.sw(p(c, S0, 0x640), c.g[V0][0]);
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_ListStop(ScriptEngine* e, int i)` (0x2790a0): stop
/// script list `i`; when it is the active list, leave script mode.
pub fn script_engine_list_stop(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = 0xcc;
    frame.save(c, S0, 0x20);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S1, 0x10);
    c.mult(0, V0, S0, V0);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.g[A1][0] = 1;
    c.g[A0][0] = Addr(p(c, S1, 0x54c)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    if callee::SCRIPT_LIST_STOP.call(c, 0x0027_90d0).is_err() {
        return;
    }
    c.g[V0][0] = Addr(p(c, S1, 0x550)).lw(c);
    let not_active = c.g[V0][0] != c.g[S0][0];
    frame.restore(c, S0, 0x20);
    if !not_active {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        if callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE
            .call(c, 0x0027_90e4)
            .is_err()
        {
            return;
        }
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void ScriptEngine_OnSeqEnd(ScriptEngine* e, void* seq)` (0x27a798): tell
/// the active script list that the sequence ended.
pub fn script_engine_on_seq_end(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = 2;
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(p(c, A0, 0x550)).lw(c);
    let idle = c.g[V1][0] == c.g[V0][0];
    c.g[S1][0] = c.g[A1][0];
    if !idle {
        c.cycles += 9;
        c.g[V0][0] = 0xcc;
        c.g[S0][0] = Addr(p(c, A0, 0x54c)).lw(c);
        c.mult(0, V0, V1, V0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID
            .call(c, 0x0027_a7cc)
            .is_err()
        {
            return;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S1][0];
        if callee::SCRIPT_LIST_ON_SEQ_END.call(c, 0x0027_a7dc).is_err() {
            return;
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `bool RiderScriptObj_MatchesRiderSlot(void* self, void* rider, int id,
/// int a3)` (0x280560): true for slot 0, otherwise compare the target object
/// of rider `id - 1` with `rider`.
pub fn rider_script_obj_matches_rider_slot(c: &mut Cpu) {
    c.cycles += 5;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    let id = c.g[A2][0];
    c.g[S0][0] = c.g[A1][0];
    if id == 0 {
        c.cycles += 2;
        c.g[V0][0] = 1;
    } else {
        c.cycles += 2;
        let negative = (id as i64) < 0;
        c.g[V0][0] = ((id as i64) < 22) as u64;
        if negative {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            let too_big = c.g[V0][0] == 0;
            c.g[A1][0] = sx((id as u32).wrapping_sub(1));
            if too_big {
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 6;
                c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
                c.g[A2][0] = c.g[A3][0];
                if callee::SCRIPT_ENGINE_GET_TARGET_OBJECT
                    .call(c, 0x0028_0590)
                    .is_err()
                {
                    return;
                }
                c.g[V0][0] = c.g[V0][0] ^ c.g[S0][0];
                c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
            }
        }
    }
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void ScriptEngine_ScriptDbDtor(ScriptEngine* e, int flags)` (0x281ed8):
/// the script database destructor: destroy the database, clear the global
/// engine pointer and destroy the object table.
pub fn script_engine_script_db_dtor(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0048_2450);
    frame.save(c, S0, 0x20);
    c.g[V1][0] = sx(0x0048_24b8);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    c.sw(p(c, S0, 0x2ac), c.g[V0][0]);
    c.sw(p(c, S0, 0x2a8), c.g[V1][0]);
    if callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE
        .call(c, 0x0028_1f0c)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    c.sw(gp(c, 0x3f8), 0);
    if callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR
        .call(c, 0x0028_1f1c)
        .is_err()
    {
        return;
    }
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void ScriptEngine_StopProcessSeq(ScriptEngine* e, int a1)` (0x282338):
/// stop the sequence process bound to the process returned by
/// `ScriptEngine_GetProcess`.
pub fn script_engine_stop_process_seq(c: &mut Cpu) {
    c.cycles += 9;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    if callee::SCRIPT_ENGINE_GET_PROCESS
        .call(c, 0x0028_234c)
        .is_err()
    {
        return;
    }
    c.g[V1][0] = c.g[V0][0];
    c.g[A1][0] = Addr(p(c, V1, 0)).lw(c);
    if c.g[A1][0] == 0 {
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(p(c, A1, 0x20)).lw(c);
    let skip_tail = c.g[V0][0] == 0;
    c.g[V0][0] = 1;
    if !skip_tail {
        c.cycles += 3;
        c.g[V1][0] = Addr(p(c, V1, 0x20)).lw(c);
        if c.g[V1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p(c, S0, 0x2b8)).lw(c);
        c.g[A1][0] = Addr(p(c, A1, 0)).lw(c);
        if callee::IN_SEQ_STOP.call(c, 0x0028_2380).is_err() {
            return;
        }
    }
    // Both exits of the inner block charge this one cycle.
    c.cycles += 1;
    let _ = skip_tail;
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Audio.

/// `void SSXAudioSystem_OnAnimSoundEvent(void* a0, void* a1, int type)`
/// (0x289b18): dispatch an animation sound event (0x50/0x51/0x52).
pub fn ssx_audio_system_on_anim_sound_event(c: &mut Cpu) {
    c.cycles += 4;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x50;
    let not_50 = c.g[A2][0] != c.g[V0][0];
    frame.save_ra(c, 0);
    if not_50 {
        c.cycles += 3;
        c.g[V0][0] = 0x51;
        let not_51 = c.g[A2][0] != c.g[V0][0];
        c.g[V0][0] = 0x52;
        if not_51 {
            c.cycles += 2;
            let not_52 = c.g[A2][0] != c.g[V0][0];
            frame.restore_ra(c, 0);
            if !not_52 {
                c.cycles += 3;
                if call_ptr(c, 0x0029_c088, 0x0028_9b64).is_err() {
                    return;
                }
                frame.restore_ra(c, 0);
            }
        } else {
            c.cycles += 4;
            if call_ptr(c, 0x0029_b968, 0x0028_9b4c).is_err() {
                return;
            }
            frame.restore_ra(c, 0);
        }
    } else {
        c.cycles += 4;
        if call_ptr(c, 0x0029_bcf8, 0x0028_9b30).is_err() {
            return;
        }
        frame.restore_ra(c, 0);
    }
    c.cycles += 2;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void SndBank_LoadFromMemory(SndBank* b, u32 a1, u32 a2)` (0x28b788): drop
/// the bank's current memory, install the new pointer/length and rebuild the
/// tag table.
pub fn snd_bank_load_from_memory(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x30);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A2][0];
    frame.save_ra(c, 0);
    c.g[S2][0] = c.g[A1][0];
    if callee::SND_BANK_UNLOAD_MEMORY.call(c, 0x0028_b7ac).is_err() {
        return;
    }
    c.sw(p(c, S0, 0x14), c.g[S1][0]);
    c.g[A1][0] = c.g[S2][0];
    c.sw(p(c, S0, 0x10), c.g[S2][0]);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
    if call_ptr(c, 0x003b_6098, 0x0028_b7c0).is_err() {
        return;
    }
    c.g[V1][0] = 1;
    frame.restore(c, S1, 0x20);
    c.sw(p(c, S0, 0), c.g[V1][0]);
    frame.restore(c, S0, 0x30);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable, FAKE, LOG_UNWIND_AT};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};

    /// The main heap's struct and a 1 MiB arena (below the test data area and
    /// clear of the fake-call log at 0x900000).
    const MEM_HEAP: u32 = 0x004f_f250;
    const ARENA: u32 = 0x0010_0000;
    const ARENA_SIZE: u32 = 0x0010_0000;

    /// Runs a generated function to completion as a setup step.
    fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }

    /// A fresh dlmalloc arena for the main heap, so allocations and frees in
    /// the callees are real (and identical on both machines).
    fn heap_init(c: &mut Cpu) {
        run(
            c,
            0x0031_ff60,
            ssx3_ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, ARENA as u64, ARENA_SIZE as u64],
        );
    }

    fn alloc(c: &mut Cpu, size: u64) -> u32 {
        run(
            c,
            0x0032_0058,
            ssx3_ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size],
        ) as u32
    }

    /// The BELibrary singleton (gp-0x1f04) already exists; its interface slot
    /// 0 (`gp-0x1efc`) is an object whose vtable slot at `+8` is a fake.
    fn belibrary(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x1000);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        let vt = vtable(c, r, &[8]);
        c.bus.write32(0x0053_08a8, DATA + 0x1100);
        c.bus.write32(DATA + 0x1104, vt);
    }

    // ------------------------------------------------------------- engine

    #[test]
    fn ps2_exec_man_vsync_thread_matches() {
        check(
            ssx3_ee::s10::f_0031ac08__generated,
            ps2_exec_man_vsync_thread,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0xac08, 0);
                let m = DATA;
                // The app's vsync clock. The tick loop must run exactly once:
                // (int)v24 - (int)v28 - 1 == -1, so v24 = 1.0 and v28 = 0.
                let clock = DATA + 0x100;
                c.bus.write32(GAME_GP + 0x2a74, clock);
                c.bus.write32(clock + 0x18, r.next_u32() % 1000);
                c.bus.write32(clock + 0x24, 0x3f80_0000);
                c.bus.write32(clock + 0x28, 0);
                let ticks = vtable(c, &mut r, &[0x38]);
                c.bus.write32(clock + 0x5c, ticks);
                let owner = DATA + 0x200;
                let handler = vtable(c, &mut r, &[0x28]);
                c.bus.write32(clock + 8, owner);
                c.bus.write32(owner + 8, handler);
                c.bus.write32(m + 0x4034, 0x1234);
                c.bus.write32(m + 0x10, 0x5678);
                // Three shapes of the thread body:
                //  0: shutdown flag set -> wait, wake the app thread, sleep,
                //     return (the whole exit path).
                //  1: flag clear, but the *first* wait unwinds: proves no
                //     vsync callback runs before the first `WaitSema`.
                //  2: flag clear and the wait succeeds, so one full iteration
                //     runs (wait -> flag -> App_VSyncCallback) and the
                //     callback's own vtable call unwinds inside it.
                let unwind = match seed % 3 {
                    0 => {
                        c.bus.write32(m + 0x20, 1);
                        u32::MAX
                    }
                    1 => {
                        c.bus.write32(m + 0x20, 0);
                        0
                    }
                    _ => {
                        c.bus.write32(m + 0x20, 0);
                        1
                    }
                };
                c.bus.write32(LOG_UNWIND_AT, unwind);
                c.g[A0][0] = m as u64;
                // The thread body is entered at its own address.
                c.pc = 0x0031_ac08;
            },
        );
    }

    #[test]
    fn input_preproc_write_string_matches() {
        check(
            ssx3_ee::s10::f_00326308__generated,
            input_preproc_write_string,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x6308, 0);
                heap_init(c);
                let inp = DATA + 0x400;
                let s = DATA + 0x500;
                let n = (seed % 8) as u32;
                for i in 0..n {
                    c.bus.write8(s + i, b'A' + (i as u8 % 26));
                }
                c.bus.write8(s + n, 0);
                if seed % 3 == 0 {
                    // Empty buffer: the write grows it (operator_new_named).
                    c.bus.write32(inp, 0);
                    c.bus.write32(inp + 4, 0);
                    c.bus.write32(inp + 8, 0);
                } else {
                    let count = r.next_u32() % 4;
                    c.bus.write32(inp, count);
                    c.bus.write32(inp + 4, count + 1 + r.next_u32() % 4);
                    c.bus.write32(inp + 8, DATA + 0x600);
                }
                c.g[A0][0] = inp as u64;
                c.g[A1][0] = s as u64;
                // The loop is entered at its own address (it re-enters at
                // 0x326328 after a thread switch).
                c.pc = 0x0032_6308;
            },
        );
    }

    // ---------------------------------------------------------- collision

    #[test]
    fn coll_sphere_transform_matches() {
        check(
            ssx3_ee::s11::f_0032fd40__generated,
            coll_sphere_transform,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0xfd40, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                fill(c, &mut r, DATA, 0x40);
                for i in 0..16 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.f32(4.0));
                }
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn coll_sphere_tree_dtor_matches() {
        check(
            ssx3_ee::s11::f_003304e8__generated,
            coll_sphere_tree_dtor,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x3304e8, 0);
                heap_init(c);
                let this = alloc(c, 0x100);
                let child = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
                c.bus.write32(this + 0x68, child);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    // ------------------------------------------------------------ objects

    #[test]
    fn counter_ctor_matches() {
        check(
            ssx3_ee::s11::f_00343768__generated,
            counter_ctor,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x343768, 0);
                let this = DATA;
                let host = DATA + 0x200;
                let cfg = DATA + 0x300;
                // +8 bit 0x100 takes the short path of WorldObj_ctor.
                c.bus.write32(host + 8, 0x100 | (seed & 0xff));
                c.bus.write32(cfg + 4, 0x7fff_0000 | (seed & 0xff));
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = host as u64;
                c.g[A3][0] = cfg as u64;
            },
        );
    }

    #[test]
    fn dead_fade_dtor_matches() {
        check(
            ssx3_ee::s11::f_00350be0__generated,
            dead_fade_dtor,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x350be0, 0);
                heap_init(c);
                let this = alloc(c, 0x100);
                let node = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
                c.bus.write32(this + 0x38, node);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    /// The modifier block of a `ModObj`: `+0x1c` is the block, whose slots
    /// `+8` (tex flip) and `+0xc` (boost) are the installed modifiers.
    fn mod_obj(c: &mut Cpu, seed: u32) -> u32 {
        let this = DATA;
        let block = DATA + 0x200;
        match seed % 3 {
            0 => c.bus.write32(this + 0x1c, 0),
            1 => {
                c.bus.write32(this + 0x1c, block);
                c.bus.write32(block + 8, 0);
                c.bus.write32(block + 0xc, 0);
            }
            _ => {
                // A live tex-flip modifier to free, and a boost modifier.
                let flip = alloc(c, 0x20);
                let boost = alloc(c, 0x20);
                c.bus.write32(this + 0x1c, block);
                c.bus.write32(block + 8, flip);
                c.bus.write32(block + 0xc, boost);
            }
        }
        this
    }

    #[test]
    fn mod_obj_set_tex_flip_modifier_matches() {
        check(
            ssx3_ee::s11::f_003555a8__generated,
            mod_obj_set_tex_flip_modifier,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x555a8, 0);
                heap_init(c);
                let this = mod_obj(c, seed);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
            },
        );
    }

    #[test]
    fn mod_obj_set_boost_modifier_matches() {
        check(
            ssx3_ee::s11::f_00355600__generated,
            mod_obj_set_boost_modifier,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x55600, 0);
                heap_init(c);
                let this = mod_obj(c, seed);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
            },
        );
    }

    #[test]
    fn multi_particle_man_clear_matches() {
        check(
            ssx3_ee::s11::f_00357b38__generated,
            multi_particle_man_clear,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x57b38, 0);
                heap_init(c);
                let list = DATA;
                for i in 0..4u32 {
                    let p = if (seed >> i) & 1 != 0 {
                        alloc(c, 0x80)
                    } else {
                        0
                    };
                    c.bus.write32(list + 4 * i, p);
                }
                c.g[A0][0] = list as u64;
            },
        );
    }

    #[test]
    fn dynamic_emitter_dtor_matches() {
        check(
            ssx3_ee::s12::f_00370c08__generated,
            dynamic_emitter_dtor,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x370c08, 0);
                heap_init(c);
                let this = alloc(c, 0x400);
                let a = if seed % 3 == 0 { alloc(c, 0x40) } else { 0 };
                let b = if seed % 5 == 0 { alloc(c, 0x40) } else { 0 };
                c.bus.write32(this + 0x1a0, a);
                c.bus.write32(this + 0x1a4, b);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn boader_shadow_alloc_texture_matches() {
        check(
            ssx3_ee::s12::f_00375990__generated,
            boader_shadow_alloc_texture,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x75990, 1);
                let mgr = DATA + 0x100;
                let shadow = DATA + 0x200;
                c.bus.write32(GAME_GP + 0x2a90, mgr);
                let vt = vtable(c, &mut r, &[0x1a0]);
                c.bus.write32(mgr + 0x10d8, vt);
                c.bus.write32(
                    shadow + 0x170,
                    if seed % 2 == 0 { 0xffff_ffff } else { 0x1234 },
                );
                c.bus.write32(shadow + 0x174, r.next_u32() % 0x400);
                c.bus.write32(shadow + 0x178, r.next_u32() % 4);
                c.g[A0][0] = shadow as u64;
            },
        );
    }

    // ---------------------------------------------------------------- UI

    #[test]
    fn ui_screen_cmd_goto_label_matches() {
        check(
            ssx3_ee::s13::f_0039cbe0__generated,
            ui_screen_cmd_goto_label,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x39cbe0, 0);
                let screen = DATA;
                let list = DATA + 0x100;
                let key = DATA + 0x200;
                let out = DATA + 0x300;
                let n = (seed % 4) as usize;
                c.bus.write32(screen + 0x34, list);
                c.bus.write32(list, n as u32);
                let mut labels = [0u32; 4];
                for i in 0..n {
                    labels[i] = 0x1000 + 0x11 * i as u32;
                    let e = list + 4 + 0x20 * i as u32;
                    c.bus.write32(e, labels[i]);
                    c.bus.write32(e + 4, 0x20);
                    c.bus.write16(e + 8, 0x200 + 3 * i as u16);
                }
                let k = if n > 0 && seed % 2 == 0 {
                    labels[(seed as usize / 2) % n]
                } else {
                    0xdead_0000
                };
                c.bus.write32(key + 4, k);
                c.g[A0][0] = screen as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = key as u64;
                c.g[A3][0] = out as u64;
            },
        );
    }

    #[test]
    fn ui_text_free_strings_matches() {
        check(
            ssx3_ee::s13::f_003a0b38__generated,
            ui_text_free_strings,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0xa0b38, 0);
                heap_init(c);
                let this = DATA;
                if seed % 2 == 0 {
                    let a = alloc(c, 0x20);
                    c.bus.write32(this + 0xac, a);
                }
                if (seed >> 1) % 2 == 0 {
                    let b = alloc(c, 0x20);
                    c.bus.write32(this + 0xb4, b);
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn c_hull_page_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a7bd8__generated,
            c_hull_page_ctor,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0xa7bd8, 0);
                heap_init(c);
                let n = 0x81 + seed % 0x700;
                c.bus.write16(DATA, n as u16);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn c_hull_page_dtor_matches() {
        check(
            ssx3_ee::s13::f_003a7c30__generated,
            c_hull_page_dtor,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0xa7c30, 0);
                heap_init(c);
                let this = alloc(c, 0x40);
                let idx = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
                c.bus.write32(this + 0xc, idx);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    /// The global "RCMP decoder delete" function pointer at 0x509434.
    const RCMP_DELETE: u32 = 0x0050_9434;

    #[test]
    fn rcmp_decoder_dtor_matches() {
        check(
            ssx3_ee::s13::f_003b04a0__generated,
            rcmp_decoder_dtor,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0xb04a0, 1);
                heap_init(c);
                c.bus.write32(RCMP_DELETE, FAKE);
                let this = alloc(c, 0x100);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    // ------------------------------------------------------------- riders

    #[test]
    fn rider_award_bonus_boost_matches() {
        check(
            ssx3_ee::s00::f_0010e8b8__generated,
            rider_award_bonus_boost,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10e8b8, 4);
                belibrary(c, &mut r);
                let rider = DATA;
                let mtx = DATA + 0x500;
                // `RiderMetrix_AwardBonusPoints` returns early for a non-4 mode.
                c.bus.write8(0x0053_5c48, (seed % 3) as u8);
                c.bus.write32(rider + 0x790, mtx);
                c.bus.write32(mtx + 0x1c4, r.f32(1.0));
                // The multiplier sfx path returns early with a null sfx table.
                c.bus.write32(rider + 0x874, 0);
                c.g[A0][0] = rider as u64;
            },
        );
    }

    #[test]
    fn rider_metrix_on_mission_completed_matches() {
        check(
            ssx3_ee::s00::f_0011a110__generated,
            rider_metrix_on_mission_completed,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11a110, 2);
                belibrary(c, &mut r);
                let mtx = DATA + 0x500;
                // The HUD slot array is empty: `ShowHUDNumber` returns at once.
                c.bus.write32(mtx + 0x1b0, 0);
                c.g[A0][0] = mtx as u64;
                // A zero amount makes `RiderMetrix_AwardPoints` return at once.
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn ai_setup_actor_riders_matches() {
        check(
            ssx3_ee::s00::f_00128998__generated,
            ai_setup_actor_riders,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x128998, 2);
                belibrary(c, &mut r);
                c.bus.write8(0x0053_5c11, (seed % 2) as u8);
                let ai = DATA;
                // No new riders to spawn: `AI_CreateActorRiders` breaks at once.
                c.bus.write32(ai + 0x84, 0x10);
                c.bus.write32(ai + 0x94, 0x10);
                c.bus.write32(ai + 0x78, 0);
                c.g[A0][0] = ai as u64;
                c.g[A1][0] = 0x10;
            },
        );
    }

    #[test]
    fn be_new_player_int_set_slot_flag3_matches() {
        check(
            ssx3_ee::s01::f_001475c0__generated,
            be_new_player_int_set_slot_flag3,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x1475c0, 1);
                // The player slot table.
                for i in 0..8u32 {
                    c.bus.write32(0x0053_05b0 + 4 * i, i);
                    c.bus
                        .write32(0x0053_4fe0 + 0x1c * i + 0x10, 0xffff_0000 | i);
                }
                c.g[A0][0] = 0;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 2) as u64;
            },
        );
    }

    // ------------------------------------------------------------- camera

    fn camera_chain(c: &mut Cpu, seed: u32) {
        let world = DATA + 0x100;
        let cam = DATA + 0x200;
        let chase = DATA + 0x300;
        let (o1, ctrl1) = (DATA + 0x400, DATA + 0x500);
        let (o2, ctrl2) = (DATA + 0x600, DATA + 0x700);
        let have = seed % 4 != 0;
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, cam);
        c.bus.write32(cam + 0x84, if have { chase } else { 0 });
        c.bus.write32(chase + 4, o1);
        c.bus.write32(o1 + 0xa8, ctrl1);
        c.bus.write32(chase + 8, o2);
        c.bus.write32(o2 + 0xa8, ctrl2);
        c.bus.write32(
            chase,
            match seed % 3 {
                0 => 1,
                1 => 2,
                _ => 0,
            },
        );
    }

    #[test]
    fn game_camera_reset_all_chase_controllers_matches() {
        check(
            ssx3_ee::s01::f_0015dc18__generated,
            game_camera_reset_all_chase_controllers,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x15dc18, 2);
                camera_chain(c, seed);
            },
        );
    }

    #[test]
    fn camera_trigger_man_unload_matches() {
        check(
            ssx3_ee::s02::f_0016c9b0__generated,
            camera_trigger_man_unload,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x16c9b0, 6);
                // The camera chain is set up so the chase reset really runs;
                // an empty trigger list makes the purge loops skip.
                camera_chain(c, seed);
                let m = DATA;
                c.bus.write32(m + 4, 0);
                c.g[A0][0] = m as u64;
            },
        );
    }

    // ---------------------------------------------------------- front end

    #[test]
    fn cfe_options_credits_ctor_matches() {
        check(
            ssx3_ee::s02::f_00185a18__generated,
            cfe_options_credits_ctor,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x185a18, 1);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = seed as u64;
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

    #[test]
    fn cfe_state_rule_select_ctor_matches() {
        check(
            ssx3_ee::s02::f_00191360__generated,
            cfe_state_rule_select_ctor,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x191360, 2);
                let this = DATA + 0x400;
                c.bus.write32(this + 0x10, DATA + 0x500);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn fe_locale_v01_matches() {
        check(
            ssx3_ee::s02::f_001958f0__generated,
            fe_locale_v01,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1958f0, 0);
                heap_init(c);
                let this = alloc(c, 0x100);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn cfe_state_net_account_mem_update_state02_matches() {
        check(
            ssx3_ee::s03::f_001b77c8__generated,
            cfe_state_net_account_mem_update_state02,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1b77c8, 0);
                // The manager's mode. Only the "leave the state alone" paths are
                // exercised: states 4 and 5 re-enter the full screen state
                // machine, which needs the FE fixtures.
                let modes = [0u32, 1, 3, 6, 0x10, 0xffff_ffff];
                c.bus.write32(0x004a_33d0, DATA + 0x100);
                c.bus.write32(DATA + 0x100, modes[(seed % 6) as usize]);
                c.bus.write32(DATA + 0x6d4, (seed % 2) as u32);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn cfe_state_login_dtor_matches() {
        check(
            ssx3_ee::s03::f_001bee78__generated,
            cfe_state_login_dtor,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1bee78, 0);
                heap_init(c);
                let this = alloc(c, 0x800);
                let buf = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
                c.bus.write32(this + 0x6d0, buf);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn cfe_state_online_agreement_dtor_matches() {
        check(
            ssx3_ee::s03::f_001c0a28__generated,
            cfe_state_online_agreement_dtor,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1c0a28, 0);
                heap_init(c);
                let this = alloc(c, 0x800);
                let buf = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
                c.bus.write32(this + 0x6d0, buf);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn cfe_state_net_news_on_widget_created_matches() {
        check(
            ssx3_ee::s03::f_001c5310__generated,
            cfe_state_net_news_on_widget_created,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c5310, 2);
                let this = DATA + 0x400;
                let widget = DATA + 0x500;
                // `GetHashValue32` is deterministic: compute the target hash with
                // the same function so both the match and the miss are covered.
                let target = run(c, 0x0031_7670, ssx3_ee::s10::StrHash, &[0x0046_5f48]);
                c.bus.write32(
                    widget + 0x38,
                    if seed % 2 == 0 {
                        target as u32
                    } else {
                        r.next_u32()
                    },
                );
                let vt = vtable(c, &mut r, &[0x38]);
                c.bus.write32(widget + 8, vt);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
            },
        );
    }

    #[test]
    fn cui_state_buy_popup_on_widget_action_matches() {
        check(
            ssx3_ee::s04::f_001caf00__generated,
            cui_state_buy_popup_on_widget_action,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x1caf00, 1);
                let this = DATA + 0x400;
                let widget = DATA + 0x500;
                let target = run(c, 0x0031_7670, ssx3_ee::s10::StrHash, &[0x004a_14e8]);
                let events = [0u32, 5, 5, 4, 6, 5];
                match seed % 5 {
                    0 => c.g[A1][0] = 0,
                    _ => {
                        c.bus.write32(
                            widget + 0x38,
                            if seed % 2 == 0 {
                                target as u32
                            } else {
                                0x1234_5678
                            },
                        );
                        c.g[A1][0] = widget as u64;
                    }
                }
                c.g[A0][0] = this as u64;
                c.g[A2][0] = events[(seed % 5) as usize] as u64;
            },
        );
    }

    #[test]
    fn cfe_state_relogin_on_widget_created_matches() {
        check(
            ssx3_ee::s04::f_001d50e0__generated,
            cfe_state_relogin_on_widget_created,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d50e0, 3);
                let this = DATA + 0x400;
                let widget = DATA + 0x500;
                let target = run(c, 0x0031_7670, ssx3_ee::s10::StrHash, &[0x0046_7790]);
                c.bus.write32(
                    widget + 0x38,
                    if seed % 2 == 0 {
                        target as u32
                    } else {
                        r.next_u32()
                    },
                );
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(widget + 8, vt);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
            },
        );
    }

    #[test]
    fn start_screen_unload_matches() {
        check(
            ssx3_ee::s06::f_00232d50__generated,
            start_screen_unload,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x232d50, 3);
                heap_init(c);
                let world = DATA + 0x100;
                let screen = DATA + 0x200;
                c.bus.write32(GAME_GP - 0x854, world);
                // The graphics manager vtable: `v19` at 0x98 and
                // `FreeAllTextures` at 0x3c0.
                let vt = vtable(c, &mut r, &[0x98, 0x3c0]);
                c.bus.write32(world + 0x10d8, vt);
                let tex = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
                c.bus.write32(screen + 0xc, tex);
                c.g[A0][0] = screen as u64;
            },
        );
    }

    #[test]
    fn mc_manager_state_matches() {
        for (gen, hand, salt) in [
            (
                ssx3_ee::s06::f_0023e2c8__generated as fn(&mut Cpu),
                mc_manager_state35 as fn(&mut Cpu),
                0x23e2c8u64,
            ),
            (
                ssx3_ee::s06::f_0023e498__generated,
                mc_manager_state33,
                0x23e498u64,
            ),
        ] {
            check(gen, hand, 150, &|c, seed| {
                let mut r = machine(c, seed, salt, 2);
                let mgr = DATA;
                let movie = DATA + 0x100;
                let ctl = vtable(c, &mut r, &[0x58]);
                c.bus.write32(mgr + 0x434, movie);
                c.bus.write32(movie, ctl);
                // The 3D crowd manager's method table lives at the object
                // itself: `{delta, fn}` at +8 / +0xc.
                let crowd = vtable(c, &mut r, &[8]);
                c.bus.write32(mgr + 0x748, crowd);
                c.g[A0][0] = mgr as u64;
            });
        }
    }

    #[test]
    fn cfe_load_state_on_widget_created_matches() {
        check(
            ssx3_ee::s06::f_00245cd0__generated,
            cfe_load_state_on_widget_created,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x245cd0, 2);
                belibrary(c, &mut r);
                // Mode 2 is the "computer players" load: the result widget setup
                // (which needs the whole FE screen) is skipped.
                c.bus.write8(0x0053_5c11, 2);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (DATA + 0x500) as u64;
            },
        );
    }

    // -------------------------------------------------------- login / voip

    #[test]
    fn cshared_login_man_do_load_matches() {
        check(
            ssx3_ee::s07::f_00265d98__generated,
            cshared_login_man_do_load,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x265d98, 2);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.bus.write32(DATA + 0x400, seed);
            },
        );
    }

    #[test]
    fn cshared_login_man_mc_delete_path_matches() {
        check(
            ssx3_ee::s07::f_00266240__generated,
            cshared_login_man_mc_delete_path,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x266240, 3);
                let path = DATA + 0x400;
                let text = b"/mem/0/ssx3\0";
                for (i, b) in text.iter().enumerate() {
                    c.bus.write8(path + i as u32, *b);
                }
                c.g[A0][0] = path as u64;
                c.g[A1][0] = (seed % 0x10) as u64;
            },
        );
    }

    #[test]
    fn cvoip_man_activate_matches() {
        check(
            ssx3_ee::s07::f_00266e30__generated,
            cvoip_man_activate,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x266e30, 1);
                let v = DATA + 0x400;
                c.bus.write32(v, (seed % 3 != 0) as u32);
                c.bus.write32(v + 4, ((seed / 3) % 2) as u32);
                c.bus.write32(GAME_GP + 0x308, (seed % 13) as u32);
                c.g[A0][0] = v as u64;
            },
        );
    }

    // ------------------------------------------------------------- replay

    #[test]
    fn playback_man_stop_matches() {
        check(
            ssx3_ee::s07::f_0026cdf8__generated,
            playback_man_stop,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x26cdf8, 2);
                let p = DATA + 0x400;
                // States 0 and 3 do nothing; 2 stops recording (whose world-state
                // restore is a no-op with a null pointer at +0x10). State 1 needs
                // the replay stream itself and is not exercised.
                let states = [0u32, 2, 2, 3, 5, 0xffff_ffff];
                c.bus.write32(p + 0x70, states[(seed % 6) as usize]);
                c.bus.write32(p + 0x10, 0);
                c.g[A0][0] = p as u64;
            },
        );
    }

    #[test]
    fn player_free_replay_cache_matches() {
        check(
            ssx3_ee::s07::f_002700d8__generated,
            player_free_replay_cache,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x2700d8, 1);
                heap_init(c);
                let player = DATA;
                let cache = alloc(c, 0x40);
                c.bus.write32(cache + 0xc, 0);
                let slot1 = if seed % 2 == 0 { cache } else { DATA + 0x300 };
                c.bus.write32(player + 0x48c, slot1);
                c.bus
                    .write32(player + 0x490, if seed % 3 == 0 { cache } else { 0 });
                c.g[A0][0] = player as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => cache as u64,
                    _ => (DATA + 0x300) as u64,
                };
            },
        );
    }

    #[test]
    fn replay_alloc_save_buffers_matches() {
        check(
            ssx3_ee::s07::f_00270f20__generated,
            replay_alloc_save_buffers,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x270f20, 2);
                heap_init(c);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.bus.write32(DATA + 0x400, seed);
            },
        );
    }

    // ------------------------------------------------------------- script

    #[test]
    fn script_engine_on_seq_end_matches() {
        check(
            ssx3_ee::s07::f_0027a798__generated,
            script_engine_on_seq_end,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x27a798, 2);
                let engine = DATA;
                let list = DATA + 0x1000;
                c.bus.write32(engine + 0x54c, list);
                c.bus.write32(
                    engine + 0x550,
                    match seed % 3 {
                        0 => 2,
                        1 => 0,
                        _ => 1,
                    },
                );
                let entry = list + 0xcc;
                c.bus.write32(entry + 0xc, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x1c, seed);
                c.g[A0][0] = engine as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
            },
        );
    }

    #[test]
    fn rider_script_obj_matches_rider_slot_matches() {
        check(
            ssx3_ee::s07::f_00280560__generated,
            rider_script_obj_matches_rider_slot,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x280560, 0);
                let rider = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = rider as u64;
                // 0 (slot 0), a negative id, too large an id, or 1..21 (the
                // target-object lookup, which needs the script object table).
                c.g[A2][0] = match seed % 5 {
                    0 => 0,
                    1 => 0xffff_ffff_ffff_ffff,
                    2 => 22,
                    3 => 0x7fff_ffff,
                    _ => 0x8000_0000,
                };
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn script_engine_stop_process_seq_matches() {
        check(
            ssx3_ee::s07::f_00282338__generated,
            script_engine_stop_process_seq,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x282338, 2);
                // `ScriptEngine_GetProcess` returns 0 for a zeroed engine, so the
                // process pointer is null and the sequence stop is skipped.
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    // -------------------------------------------------------------- audio

    #[test]
    fn script_engine_script_db_dtor_matches() {
        check(
            ssx3_ee::s07::f_00281ed8__generated,
            script_engine_script_db_dtor,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x281ed8, 1);
                heap_init(c);
                let this = alloc(c, 0x300);
                // No live database: `DestroyScriptDatabase` returns at once, and
                // the object table below the engine is empty.
                c.bus.write32(this + 0x2b0, 0);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_anim_sound_event_matches() {
        check(
            ssx3_ee::s07::f_00289b18__generated,
            ssx_audio_system_on_anim_sound_event,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x289b18, 3);
                let a0 = DATA + 0x400;
                let a1 = DATA + 0x800;
                // A null sfx table at `+0x874` makes every handler return early.
                c.bus.write32(a1 + 0x874, 0);
                c.g[A0][0] = a0 as u64;
                c.g[A1][0] = a1 as u64;
                c.g[A2][0] = [0x50u64, 0x51, 0x52, 0x53, 0][(seed % 5) as usize];
            },
        );
    }

    #[test]
    fn snd_bank_load_from_memory_matches() {
        check(
            ssx3_ee::s07::f_0028b788__generated,
            snd_bank_load_from_memory,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x28b788, 2);
                let bank = DATA + 0x400;
                c.bus.write32(bank, seed);
                c.bus.write32(bank + 4, 0xffff_ffff);
                c.g[A0][0] = bank as u64;
                c.g[A1][0] = (DATA + 0x500) as u64;
                c.g[A2][0] = (seed * 0x10 + 4) as u64;
            },
        );
    }
}
