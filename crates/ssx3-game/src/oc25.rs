//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Hand-written, typed replacements for small generated functions, each with a
//! differential test against the generated body.

use crate::abi;
use crate::call::{self, call_ptr, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_15e0, ai_anim_event_map_ctor, "AIAnimEventMap_ctor"),
    (0x0031_1870, ai_anim_event_map_create, "AIAnimEventMap_Create"),
    (0x0031_18a8, ai_anim_event_map_destroy, "AIAnimEventMap_Destroy"),
    (0x0032_7828, ai_fwd_diff_cache_dtor, "AIFwdDiffCache_dtor"),
    (0x0032_7bb0, ai_fwd_diff_cache_hash_insert, "AIFwdDiffCache_HashInsert"),
    (0x0042_3a80, add_intc_handler, "AddIntcHandler"),
    (0x0042_3a90, add_intc_handler_2, "AddIntcHandler"),
    (0x0042_3ab0, add_dmac_handler, "AddDmacHandler"),
    (0x0042_3ac0, add_dmac_handler_2, "AddDmacHandler"),
    (0x0016_c5e0, active_camera_trigger_list_dtor, "ActiveCameraTriggerList_dtor"),
    (0x0016_c628, active_camera_trigger_list_clear, "ActiveCameraTriggerList_Clear"),
    (0x0016_c678, active_camera_trigger_list_clear_flags, "ActiveCameraTriggerList_ClearFlags"),
    (0x0016_c6b0, active_camera_trigger_list_find, "ActiveCameraTriggerList_Find"),
    (0x0016_c718, active_camera_trigger_list_add, "ActiveCameraTriggerList_Add"),
    (0x0016_c778, active_camera_trigger_list_remove, "ActiveCameraTriggerList_Remove"),
    (0x0024_48e0, aivfx_process_dtor, "AIVfxProcess_dtor"),
    (0x0024_4948, aivfx_process_update, "AIVfxProcess_Update"),
    (0x0024_4978, aivfx_process_render, "AIVfxProcess_Render"),
    (0x0025_09d8, aivisual_effects_main_menu_ctor, "AIVisualEffectsMainMenu_ctor"),
    (0x0025_0a60, aivisual_effects_main_menu_on_activate, "AIVisualEffectsMainMenu_OnActivate"),
    (0x0025_0a80, aivisual_effects_main_menu_on_deactivate, "AIVisualEffectsMainMenu_OnDeactivate"),
    (0x0025_1630, aivisual_effects_main_menu_dtor, "AIVisualEffectsMainMenu_dtor"),
    (0x0025_1690, aivisual_effects_menus_global_ctors, "AIVisualEffectsMenus_GlobalCtors"),
    (0x0025_16b0, aivisual_effects_menus_global_dtors, "AIVisualEffectsMenus_GlobalDtors"),
    (0x002f_8340, aivisual_effects_man_dtor, "AIVisualEffectsMan_dtor"),
    (0x0032_7b30, ai_fwd_diff_cache_hash_remove, "AIFwdDiffCache_HashRemove"),
    (0x0017_63f8, algorithmic_script_camera_algo_dtor, "AlgorithmicScriptCameraAlgo_dtor"),
    (0x0031_10d0, anim_data_base_reset_chan_cache_table, "AnimDataBase_ResetChanCacheTable"),
    (0x003a_f300, av_skip_whitespace, "AV_SkipWhitespace"),
    (0x003a_f378, av_str_icmp, "AV_StrICmp"),
    (0x003a_e860, av_player_is_audio_done, "AV_Player_IsAudioDone"),
    (0x003a_e888, av_player_set_rate, "AV_Player_SetRate"),
    (0x003a_e938, av_player_pause, "AV_Player_Pause"),
    (0x003a_e958, av_player_resume, "AV_Player_Resume"),
    (0x003a_e978, av_player_set_volume, "AV_Player_SetVolume"),
    (0x003a_ed20, av_player_release_chunk_callback, "AV_Player_ReleaseChunkCallback"),
    (0x003a_ee98, av_audio_set_pitch, "AV_Audio_SetPitch"),
    (0x003a_eef8, av_audio_start, "AV_Audio_Start"),
    (0x003a_ef48, av_audio_set_volume, "AV_Audio_SetVolume"),
    (0x003a_efa8, av_audio_is_done, "AV_Audio_IsDone"),
    (0x003a_f258, av_find_subtitle_by_frame, "AV_FindSubtitleByFrame"),
    (0x003a_e718, av_player_get_current_subtitle, "AV_Player_GetCurrentSubtitle"),
    (0x003a_f020, av_free_subtitles, "AV_FreeSubtitles"),
    (0x0011_3aa0, air_predictor_save_state, "AirPredictor_SaveState"),
    (0x0011_3ad8, air_predictor_load_state, "AirPredictor_LoadState"),
    (0x0016_b150, algorithmic_script_camera_algo_reset, "AlgorithmicScriptCameraAlgo_Reset"),
    (0x0015_c940, algorithm_list_dtor, "AlgorithmList_dtor"),
    (0x0015_cb08, algorithm_list_clear, "AlgorithmList_Clear"),
    (0x0031_12c8, adf_data_source_dtor, "AdfDataSource_dtor"),
    (0x0035_8e50, anim_combo_dtor, "AnimCombo_dtor"),
    (0x0035_8f90, anim_combo_v42, "AnimCombo_v42"),
    (0x0035_9030, anim_combo_snapshot_matrices, "AnimCombo_SnapshotMatrices"),
    (0x0035_8db0, anim_combo_ctor_2, "AnimCombo_ctor_2"),
    (0x0035_90c8, anim_combo_kill, "AnimCombo_Kill"),
    (0x0035_90f0, anim_combo_save, "AnimCombo_Save"),
    (0x0031_1048, anim_data_base_ctor, "AnimDataBase_ctor"),
    (0x0031_1088, anim_data_base_dtor, "AnimDataBase_dtor"),
    (0x0031_1e38, anim_controller_clear_layer, "AnimController_ClearLayer"),
    (0x0031_2ab0, anim_controller_get_layer_progress, "AnimController_GetLayerProgress"),
    (0x0031_2790, anim_controller_get_current_anim_length, "AnimController_GetCurrentAnimLength"),
    (0x0031_2820, anim_controller_get_current_anim_trigger_time, "AnimController_GetCurrentAnimTriggerTime"),
    (0x0016_ad38, algorithmic_script_camera_algo_ctor, "AlgorithmicScriptCameraAlgo_ctor"),
    (0x0015_c988, algorithm_list_push, "AlgorithmList_Push"),
    (0x0031_18d0, anim_controller_ctor, "AnimController_ctor"),
    (0x0031_1958, anim_controller_dtor, "AnimController_dtor"),
];

// ---------------------------------------------------------------------------
// Direct-call targets.

const EA_MEM_ALLOC_NAMED: Callee = Callee { addr: 0x0031_7d70, f: ee::s10::EA_MemAllocNamed };
const BUILTIN_DELETE: Callee = Callee { addr: 0x0031_7e50, f: ee::s10::__builtin_delete };
const EA_MEM_FREE: Callee = Callee { addr: 0x0031_7e98, f: ee::s10::EA_MemFree };
const AI_ANIM_EVENT_MAP_CTOR: Callee = Callee { addr: 0x0031_15e0, f: ee::s10::f_003115e0 };
const ACTIVE_CAMERA_TRIGGER_LIST_CLEAR: Callee =
    Callee { addr: 0x0016_c628, f: ee::s02::f_0016c628 };
/// `__builtin_delete(0x317e50)`, cross-shard (through the function table).
const BUILTIN_DELETE_ADDR: u32 = 0x0031_7e50;
/// Cross-shard direct call emitted as `call_indirect`.
const GAME_PROCESS_DTOR: u32 = 0x0035_46c8;
const AIVISUAL_EFFECTS_MENUS_STATIC_INIT_DESTROY: Callee =
    Callee { addr: 0x0025_0cb0, f: ee::s06::f_00250cb0 };
const AV_PLAYER_SET_RATE: Callee = Callee { addr: 0x003a_e888, f: ee::s13::f_003ae888 };
const AV_AUDIO_IS_DONE: Callee = Callee { addr: 0x003a_efa8, f: ee::s13::f_003aefa8 };
const AV_AUDIO_SET_VOLUME: Callee = Callee { addr: 0x003a_ef48, f: ee::s13::f_003aef48 };
const AV_AUDIO_SET_PITCH: Callee = Callee { addr: 0x003a_ee98, f: ee::s13::f_003aee98 };
const AV_PLAYER_ON_CHUNK: Callee = Callee { addr: 0x003a_ead0, f: ee::s13::f_003aead0 };
const AV_PLAYER_RELEASE_CHUNK: Callee = Callee { addr: 0x003a_ece0, f: ee::s13::f_003aece0 };
const EATIMER_GET_TICKS: u32 = 0x003e_4e98;
const SND_LOCK: Callee = Callee { addr: 0x003b_58a0, f: ee::s13::f_003b58a0 };
const SND_UNLOCK: Callee = Callee { addr: 0x003b_58d8, f: ee::s13::f_003b58d8 };
const SNDSTRM_SET_PITCH: Callee = Callee { addr: 0x003b_7fb8, f: ee::s13::f_003b7fb8 };
const SNDSTRM_SET_PREBUFFER: Callee = Callee { addr: 0x003b_7d80, f: ee::s13::f_003b7d80 };
const SNDSTRM_SET_VOLUME: Callee = Callee { addr: 0x003b_7e70, f: ee::s13::f_003b7e70 };
const SNDSTRM_GET_STATUS: Callee = Callee { addr: 0x003b_7b48, f: ee::s13::f_003b7b48 };
const SNDSTRM_GET_REQUEST_STATUS: Callee = Callee { addr: 0x003b_7c40, f: ee::s13::f_003b7c40 };
const AV_FIND_SUBTITLE_BY_FRAME: Callee = Callee { addr: 0x003a_f258, f: ee::s13::f_003af258 };
const ALGORITHM_LIST_CLEAR: Callee = Callee { addr: 0x0015_cb08, f: ee::s01::f_0015cb08 };
const ADF_DATA_SOURCE_SET_DATA: Callee = Callee { addr: 0x0031_1290, f: ee::s10::f_00311290 };
const ANIM_OBJECT_DTOR: Callee = Callee { addr: 0x0034_1cf0, f: ee::s11::f_00341cf0 };
const MOD_OBJ_V42: Callee = Callee { addr: 0x0035_6ae0, f: ee::s11::f_00356ae0 };
const WORLD_OBJ_KILL: Callee = Callee { addr: 0x0034_fd90, f: ee::s11::f_0034fd90 };
const ANIM_OBJECT_SAVE: Callee = Callee { addr: 0x0034_2150, f: ee::s11::f_00342150 };
const ANIM_OBJECT_V52: Callee = Callee { addr: 0x0034_e348, f: ee::s11::f_0034e348 };
const ANIM_OBJECT_CTOR_2: Callee = Callee { addr: 0x0034_1c80, f: ee::s11::f_00341c80 };
const ANIM_COMBO_SNAPSHOT: Callee = Callee { addr: 0x0035_9030, f: ee::s11::f_00359030 };
const INTERSECT_INFO_MUL_MATRIX: Callee = Callee { addr: 0x0035_1948, f: ee::s11::f_00351948 };
const MEMSET_ADDR: u32 = 0x0041_6210;
const ANIM_DATA_BASE_RESET: Callee = Callee { addr: 0x0031_10d0, f: ee::s10::f_003110d0 };
const ANIM_DATA_BASE_FREE: Callee = Callee { addr: 0x0031_1110, f: ee::s10::f_00311110 };
const ANIM_SEQ_LIST_CLEAR: Callee = Callee { addr: 0x0031_4668, f: ee::s10::f_00314668 };
const ANIM_SEQ_LIST_GET_AT: Callee = Callee { addr: 0x0031_4760, f: ee::s10::f_00314760 };
const SCRIPT_CAMERA_ALGO_CTOR: Callee = Callee { addr: 0x0016_9418, f: ee::s02::f_00169418 };
const OPERATOR_NEW_NAMED_ADDR: u32 = 0x0031_7d70;
const CAMERA_WOBBLE_INIT: u32 = 0x0015_fff0;
const OPERATOR_NEW_NAMED2_ADDR: u32 = 0x0031_7e30;
const ANIM_SEQ_LIST_INIT: Callee = Callee { addr: 0x0031_42e8, f: ee::s10::f_003142e8 };
const ANIM_CONTROLLER_RESET: Callee = Callee { addr: 0x0031_1a50, f: ee::s10::f_00311a50 };
/// `EAFile_LoadDecompressedThunk`, cross-shard (through the function table).
const EA_FILE_LOAD_DECOMPRESSED_THUNK: u32 = 0x003e_22f0;
const EA_MEM_FREE_ADDR: u32 = 0x0031_7e98;
/// `__fixunssfsi` (cross-shard, through the function table).
const FIXUNSSFSI: u32 = 0x0041_39f8;

// ---------------------------------------------------------------------------
// Kernel syscall wrappers.

/// The `syscall` trap: `pc = ret`, invoke the syscall hook at `at`; `false`
/// when the hook unwound (a thread switch mid-syscall).
#[inline]
fn do_syscall(c: &mut Cpu, at: u32, ret: u32) -> bool {
    c.pc = ret;
    c.syscall(at);
    c.pc == ret
}

/// One `syscall` wrapper: 4 cycles, `v1 = code`, trap, a due-event check and
/// `jr ra`.
#[inline]
fn syscall_wrapper(c: &mut Cpu, code: u32, at: u32, ret: u32) {
    c.cycles += 4;
    c.g[V1][0] = sx(code);
    if !do_syscall(c, at, ret) {
        return;
    }
    if event_due(c) {
        c.event();
    }
    abi::ret(c);
}

/// `int AddIntcHandler(int intr, int (*handler)(), int arg, int flag)`
/// (0x423a80): syscall 0x10.
pub fn add_intc_handler(c: &mut Cpu) {
    syscall_wrapper(c, 0x10, 0x0042_3a84, 0x0042_3a88);
}

/// `int AddIntcHandler(...)` (0x423a90): identical syscall-0x10 wrapper.
pub fn add_intc_handler_2(c: &mut Cpu) {
    syscall_wrapper(c, 0x10, 0x0042_3a94, 0x0042_3a98);
}

/// `int AddDmacHandler(int channel, int (*handler)(), int arg, int flag)`
/// (0x423ab0): syscall 0x12.
pub fn add_dmac_handler(c: &mut Cpu) {
    syscall_wrapper(c, 0x12, 0x0042_3ab4, 0x0042_3ab8);
}

/// `int AddDmacHandler(...)` (0x423ac0): identical syscall-0x12 wrapper.
pub fn add_dmac_handler_2(c: &mut Cpu) {
    syscall_wrapper(c, 0x12, 0x0042_3ac4, 0x0042_3ac8);
}

// ---------------------------------------------------------------------------
// Active camera trigger list (a linked list of 0x10-byte nodes: id at +0,
// next at +4, flags at +8/+0xc).

/// The first word of a list holds the head node.
const LIST_HEAD: u32 = 0x0;
const NODE_NEXT: u32 = 0x4;
const NODE_ID: u32 = 0x0;
const NODE_FLAG_A: u32 = 0x8;
const NODE_FLAG_B: u32 = 0xc;

/// `void ActiveCameraTriggerList_dtor(ActiveCameraTriggerList *list, uint flags)`
/// (0x16c5e0): clear the list, then delete it when `flags & 1`.
pub fn active_camera_trigger_list_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        ACTIVE_CAMERA_TRIGGER_LIST_CLEAR.call(c, 0x0016_c5fc)?;
        c.g[S0][0] &= 1;
        let delete_list = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete_list {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, BUILTIN_DELETE_ADDR, 0x0016_c610)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ActiveCameraTriggerList_Clear(ActiveCameraTriggerList *list)`
/// (0x16c628): delete every node and set the head to null. 8 cycles + per
/// node.
pub fn active_camera_trigger_list_clear(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(LIST_HEAD).lw(c);
        if c.g[S0][0] == 0 {
            c.sw(c.g[S1][0] as u32, 0);
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 4;
                c.g[S0][0] = Addr((c.g[S0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
                call_ptr(c, BUILTIN_DELETE_ADDR, 0x0016_c658)?;
                let more = c.g[S0][0] != 0;
                c.g[A0][0] = c.g[S0][0];
                if !more {
                    c.cycles += 1;
                    c.sw(c.g[S1][0] as u32, 0);
                    break;
                }
                if event_due(c) {
                    deliver_event(c, 0x0016_c650);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ActiveCameraTriggerList_ClearFlags(ActiveCameraTriggerList *list)`
/// (0x16c678): zero the flag words of every node.
pub fn active_camera_trigger_list_clear_flags(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).off(LIST_HEAD).lw(c);
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.sw((c.g[A0][0] as u32).wrapping_add(NODE_FLAG_B), 0);
    loop {
        c.cycles += 7;
        c.sw((c.g[A0][0] as u32).wrapping_add(NODE_FLAG_A), 0);
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
        if c.g[A0][0] == 0 {
            break;
        }
        c.sw((c.g[A0][0] as u32).wrapping_add(NODE_FLAG_B), 0);
        if event_due(c) {
            deliver_event(c, 0x0016_c688);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `Node *ActiveCameraTriggerList_Find(ActiveCameraTriggerList *list, int id)`
/// (0x16c6b0): the first node whose id equals `id`, or null.
pub fn active_camera_trigger_list_find(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).off(LIST_HEAD).lw(c);
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(NODE_ID).lw(c);
    loop {
        c.cycles += 2;
        if c.g[V0][0] == c.g[A1][0] {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(NODE_ID).lw(c);
        if event_due(c) {
            deliver_event(c, 0x0016_c6c0);
        }
    }
}

/// `Node *ActiveCameraTriggerList_Add(ActiveCameraTriggerList *list, int id)`
/// (0x16c718): allocate a 0x10-byte node, push it at the head, and return it.
pub fn active_camera_trigger_list_add(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S1, 0x10);
        c.g[A3][0] = 0;
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x46_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x10;
        c.g[A1][0] = sx(0x46_0000u32.wrapping_sub(0x4098));
        EA_MEM_ALLOC_NAMED.call(c, 0x0016_c748)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(LIST_HEAD).lw(c);
        c.g[V1][0] = 1;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.sw(c.g[V0][0] as u32, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(NODE_FLAG_A), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(NODE_NEXT), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(NODE_FLAG_B), c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `Node *ActiveCameraTriggerList_Remove(ActiveCameraTriggerList *list, Node *node)`
/// (0x16c778): unlink the node with pointer identity `node` and delete it;
/// returns the new head. A `match pc` dispatcher re-enterable at its loop
/// heads after an unwind (0x16c7b0, 0x16c7d0).
pub fn active_camera_trigger_list_remove(c: &mut Cpu) {
    const ENTRY: u32 = 0x0016_c778;
    const END: u32 = 0x0016_c800;
    const AT_LINK: u32 = 0x0016_c7b0;
    const AT_CMP: u32 = 0x0016_c7d0;
    call::guest(c, |c| {
        // Stage: 0 = entry, 1 = remove head, 2 = relink+delete, 3 = compare,
        // 4 = advance, 5..7 = transient, 8 = epilogue.
        let mut stage = match c.pc {
            ENTRY => 0u8,
            AT_LINK => 2,
            AT_CMP => 3,
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        };
        const FRAME_SIZE: u32 = 0x30;
        loop {
            match stage {
                0 => {
                    c.cycles += 8;
                    let frame = Frame::open(c, FRAME_SIZE);
                    frame.save(c, S1, 0x10);
                    frame.save(c, S0, 0x20);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(LIST_HEAD).lw(c);
                    let head_match = c.g[A1][0] == c.g[V0][0];
                    c.g[S0][0] = c.g[V0][0];
                    stage = if head_match { 1 } else { 4 };
                }
                1 => {
                    c.cycles += 6;
                    c.g[S0][0] = Addr((c.g[A1][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
                    c.g[A0][0] = c.g[A1][0];
                    call_ptr(c, BUILTIN_DELETE_ADDR, 0x0016_c7a4)?;
                    c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                    c.g[V0][0] = c.g[S0][0];
                    stage = 8;
                }
                4 => {
                    c.cycles += 3;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
                    if c.g[A0][0] == 0 {
                        c.g[V0][0] = 0;
                        stage = 8;
                    } else {
                        c.cycles += 1;
                        stage = 3;
                    }
                }
                3 => {
                    c.cycles += 2;
                    if c.g[A0][0] == c.g[A1][0] {
                        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
                        if back_edge_unwinds(c, AT_LINK) {
                            return Ok(());
                        }
                        stage = 2;
                    } else {
                        c.cycles += 4;
                        c.g[S0][0] = c.g[A0][0];
                        c.g[A0][0] =
                            Addr((c.g[A0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
                        if c.g[A0][0] != 0 {
                            if back_edge_unwinds(c, AT_CMP) {
                                return Ok(());
                            }
                            stage = 3;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                            stage = 8;
                        }
                    }
                }
                2 => {
                    c.cycles += 4;
                    c.sw((c.g[S0][0] as u32).wrapping_add(NODE_NEXT), c.g[V0][0]);
                    call_ptr(c, BUILTIN_DELETE_ADDR, 0x0016_c7b8)?;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
                    stage = 8;
                }
                _ => {
                    c.cycles += 5;
                    let frame = Frame { size: FRAME_SIZE };
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// AI animation event map.

/// `void AIAnimEventMap_ctor(AIAnimEventMap *self)` (0x3115e0): copy the
/// 0x760-byte event table at 0x489d60 into a stack frame, clear the map's
/// 437 halfword entries at `self + 0x6d2` (stepping down), then rebuild the
/// halfword table from the copied words: entry `id` gets the running index in
/// its low half and the next count in its high half. The copied table is
/// 8-byte aligned, so only the `ld`/`sd` 32-byte copy path runs.
pub fn ai_anim_event_map_ctor(c: &mut Cpu) {
    const TABLE: u32 = 0x489d60; // 0x490000 - 0x62a0
    const LEN: u32 = 0x760;
    const COPY_HEAD: u32 = 0x0031_1660;
    const CLEAR_HEAD: u32 = 0x0031_1698;
    const BUILD_HEAD: u32 = 0x0031_16c8;
    c.cycles += 7;
    c.g[V0][0] = 0x49_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(LEN));
    let this = c.g[A0][0];
    c.g[T0][0] = this;
    c.g[V1][0] = sx(0x49_0000u32.wrapping_sub(0x62a0));
    c.g[V0][0] = c.g[V1][0] & 7;
    c.g[A0][0] = c.g[SP][0];
    // The table address is fixed and 8-byte aligned, so the `ldl/ldr` copy
    // path (0x311604) is never taken; the `ld`/`sd` path below always runs.
    c.cycles += 2;
    c.g[A1][0] = sx(TABLE.wrapping_add(LEN));
    c.g[V0][0] = sx((this as u32).wrapping_add(2));
    loop {
        c.cycles += 11;
        let src = c.g[V1][0] as u32;
        c.g[A2][0] = Addr(src).ld(c);
        c.g[A3][0] = Addr(src.wrapping_add(8)).ld(c);
        c.g[T1][0] = Addr(src.wrapping_add(0x10)).ld(c);
        c.g[T2][0] = Addr(src.wrapping_add(0x18)).ld(c);
        let dst = c.g[A0][0] as u32;
        Addr(dst).sd(c, c.g[A2][0]);
        Addr(dst.wrapping_add(8)).sd(c, c.g[A3][0]);
        Addr(dst.wrapping_add(0x10)).sd(c, c.g[T1][0]);
        Addr(dst.wrapping_add(0x18)).sd(c, c.g[T2][0]);
        c.g[V1][0] = sx(src.wrapping_add(0x20));
        let more = c.g[V1][0] != c.g[A1][0];
        c.g[A0][0] = sx(dst.wrapping_add(0x20));
        if !more {
            c.cycles += 1;
            c.g[A0][0] = 0x1b4;
            break;
        }
        if event_due(c) {
            deliver_event(c, COPY_HEAD);
        }
    }
    // Clear 437 halfwords ending at self + 0.
    c.cycles += 2;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6d0));
    loop {
        c.cycles += 7;
        Addr(c.g[V1][0] as u32).sh(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        let count_down = (c.g[A0][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(4));
        if count_down {
            if event_due(c) {
                deliver_event(c, CLEAR_HEAD);
            }
            continue;
        }
        c.cycles += 5;
        // Build the halfword table from the copied words.
        c.g[T1][0] = c.g[V0][0];
        c.g[A3][0] = 0x1b6;
        c.g[A2][0] = 0;
        c.g[A1][0] = c.g[SP][0];
        loop {
            c.cycles += 3;
            let at = c.g[A1][0] as u32;
            c.g[V0][0] = Addr(at).lw(c);
            let same = c.g[V0][0] == c.g[A3][0];
            c.g[A1][0] = sx(at.wrapping_add(4));
            if !same {
                c.cycles += 4;
                c.g[A3][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
                c.g[V0][0] = sx((this as u32).wrapping_add(c.g[V0][0] as u32));
                Addr(c.g[V0][0] as u32).sh(c, c.g[A2][0]);
            }
            c.cycles += 8;
            c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = ((c.g[A2][0] as i64) < 472i64) as u64;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            let more = c.g[A0][0] != 0;
            Addr(c.g[V0][0] as u32).sh(c, c.g[V1][0]);
            if !more {
                c.cycles += 2;
                let ret = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(LEN));
                c.pc = ret;
                return;
            }
            if event_due(c) {
                deliver_event(c, BUILD_HEAD);
            }
        }
    }
}

/// `AIAnimEventMap *AIAnimEventMap_Create(void)` (0x311870): allocate 0x6d4
/// bytes, store the map in the global at `gp + 0xd0c`, and construct it.
pub fn ai_anim_event_map_create(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x49_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x6d4;
        c.g[A1][0] = sx(0x49_0000u32.wrapping_sub(0x5b30));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        EA_MEM_ALLOC_NAMED.call(c, 0x0031_1890)?;
        c.g[A0][0] = c.g[V0][0];
        c.sw((c.g[GP][0] as u32).wrapping_add(0xd0c), c.g[V0][0]);
        AI_ANIM_EVENT_MAP_CTOR.call(c, 0x0031_189c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AIAnimEventMap_Destroy(void)` (0x3118a8): delete the global map
/// (`gp + 0xd0c`) and clear the slot.
pub fn ai_anim_event_map_destroy(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xd0c)).lw(c);
        frame.save_ra(c, 0);
        BUILTIN_DELETE.call(c, 0x0031_18bc)?;
        frame.restore_ra(c, 0);
        c.sw((c.g[GP][0] as u32).wrapping_add(0xd0c), 0);
        frame.close(c);
        Ok(())
    })
}

/// `AIFwdDiffCache *AIFwdDiffCache_dtor(AIFwdDiffCache *this, uint flags)`
/// (0x327828): free the two owned blocks (`this+0` and `this+8`) and, when
/// `flags & 1`, delete `this` itself.
pub fn ai_fwd_diff_cache_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let has_first = c.g[A0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if has_first {
            c.cycles += 2;
            EA_MEM_FREE.call(c, 0x0032_7850)?;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
        let has_second = c.g[A0][0] != 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if has_second {
            c.cycles += 3;
            EA_MEM_FREE.call(c, 0x0032_7864)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            BUILTIN_DELETE.call(c, 0x0032_7878)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AIFwdDiffCache_HashInsert(AIFwdDiffCache *this, Entry *e)`
/// (0x327bb0): insert `e` into the hash bucket `e->key % this->bucket_count`
/// (`this+0xc`), which is a list ordered by descending key (`e->key` at +0,
/// the chain link at +0xc).
pub fn ai_fwd_diff_cache_hash_insert(c: &mut Cpu) {
    const BUCKET_COUNT: u32 = 0xc;
    const BUCKETS: u32 = 0x8;
    const KEY: u32 = 0x0;
    const NEXT: u32 = 0xc;
    const HEAD: u32 = 0x0032_7bd8;
    c.cycles += 5;
    let this = c.g[A0][0] as u32;
    c.g[A3][0] = c.g[A1][0];
    c.g[V0][0] = Addr(this).off(BUCKET_COUNT).lw(c);
    c.g[A2][0] = Addr(c.g[A3][0] as u32).off(KEY).lw(c);
    c.cycles += 5;
    abi::divu(c, c.g[A2][0] as u32, c.g[V0][0] as u32);
    c.g[A1][0] = Addr(this).off(BUCKETS).lw(c);
    c.g[V1][0] = sx((c.hi[0] as u32) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    loop {
        c.cycles += 3;
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        if c.g[V1][0] == 0 {
            c.sw(c.g[A1][0] as u32, c.g[A3][0]);
            break;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(KEY).lw(c);
        c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.sw(c.g[A1][0] as u32, c.g[A3][0]);
            break;
        }
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(NEXT));
        if event_due(c) {
            deliver_event(c, HEAD);
        }
    }
    c.cycles += 2;
    let ret = c.g[RA][0] as u32;
    c.sw((c.g[A3][0] as u32).wrapping_add(NEXT), c.g[V1][0]);
    c.pc = ret;
}

// ---------------------------------------------------------------------------
// AI visual-effects processes and menus.

/// `void AIVfxProcess_dtor(AIVfxProcess *this, uint flags)` (0x2448e0): set the
/// vtable at +0xc, call the sub-effect's destructor (`this+0x10 -> +4` slot
/// +8), then `GameProcess_dtor(this, flags)`.
pub fn aivfx_process_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x48_0000u32.wrapping_sub(0x34a0));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
        let has_effect = c.g[A2][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if has_effect {
            c.cycles += 6;
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x0024_4924)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, GAME_PROCESS_DTOR, 0x0024_4930)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AIVfxProcess_Update(AIVfxProcess *this)` (0x244948): call the
/// sub-effect's update (`this+0x10 -> +4` vtable slot +0x10).
pub fn aivfx_process_update(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let inner = Addr(c.g[A0][0] as u32).off(0x10).lw(c);
        c.g[A1][0] = inner;
        let eff = Addr(inner as u32).off(4).lw(c);
        c.g[V0][0] = eff;
        c.g[A0][0] = Addr(eff as u32).off(0x10).lh(c);
        let func = Addr(eff as u32).off(0x14).lw(c);
        c.g[V1][0] = func;
        c.g[A0][0] = abi::addu(inner, c.g[A0][0]);
        call_ptr(c, func as u32, 0x0024_4968)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AIVfxProcess_Render(AIVfxProcess *this)` (0x244978): call the
/// sub-effect's render (`this+0x10 -> +4` vtable slot +0x18).
pub fn aivfx_process_render(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let inner = Addr(c.g[A0][0] as u32).off(0x10).lw(c);
        c.g[A1][0] = inner;
        let eff = Addr(inner as u32).off(4).lw(c);
        c.g[V0][0] = eff;
        c.g[A0][0] = Addr(eff as u32).off(0x18).lh(c);
        let func = Addr(eff as u32).off(0x1c).lw(c);
        c.g[V1][0] = func;
        c.g[A0][0] = abi::addu(inner, c.g[A0][0]);
        call_ptr(c, func as u32, 0x0024_4998)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AIVisualEffectsMainMenu *AIVisualEffectsMainMenu_ctor(AIVisualEffectsMainMenu *this)`
/// (0x2509d8): construct the page, a label item, a spacer item and a submenu
/// item and insert the submenu into the page.
pub fn aivisual_effects_main_menu_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002c_aa58, 0x0025_09f0)?; // MenuSysPage_ctor(this)
        let s0 = c.g[S0][0];
        c.g[S1][0] = sx((s0 as u32).wrapping_add(0x164));
        c.g[V1][0] = 0x48_0000;
        c.g[A1][0] = 0x48_0000;
        c.g[V1][0] = sx(0x48_0000u32.wrapping_sub(0x18));
        c.g[A0][0] = sx((s0 as u32).wrapping_add(0x130));
        c.sw((s0 as u32).wrapping_add(0x12c), c.g[V1][0]);
        c.g[A1][0] = sx(0x48_0000u32.wrapping_sub(0x50));
        call_ptr(c, 0x002c_d2a0, 0x0025_0a10)?; // MenuSysLabelItem_ctor
        let s0 = c.g[S0][0];
        c.g[A0][0] = sx((s0 as u32).wrapping_add(0x14c));
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0025_0a1c)?; // MenuSysSpacerItem_ctor
        c.g[A1][0] = 0x4d_0000;
        c.g[A2][0] = 0x48_0000;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(0x4d_0000u32.wrapping_add(0x1c88));
        c.g[A2][0] = sx(0x48_0000u32.wrapping_sub(0x30));
        call_ptr(c, 0x002c_ccc8, 0x0025_0a34)?; // MenuSysSubmenuItem_ctor
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x0025_0a44)?; // MenuSysPage_InsertItem
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AIVisualEffectsMainMenu_OnActivate(AIVisualEffectsMainMenu *this)`
/// (0x250a60): `MenuSysPage_OnActivate(this)`.
pub fn aivisual_effects_main_menu_on_activate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002c_bf60), 0x0025_0a70)
}

/// `void AIVisualEffectsMainMenu_OnDeactivate(AIVisualEffectsMainMenu *this)`
/// (0x250a80): `MenuSysPage_OnDeactivate(this)`.
pub fn aivisual_effects_main_menu_on_deactivate(c: &mut Cpu) {
    call::tail_wrapper(c, None, call::Target::Table(0x002c_bfd0), 0x0025_0a90)
}

/// `void AIVisualEffectsMainMenu_dtor(AIVisualEffectsMainMenu *this, uint flags)`
/// (0x251630): destruct the submenu, spacer, label and page, then the base.
pub fn aivisual_effects_main_menu_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002c_a280, 0x0025_1654)?; // MenuSysItem_dtor
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002c_a280, 0x0025_1660)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002c_a280, 0x0025_166c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0025_1678)?; // MenuSysPage_dtor
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AIVisualEffectsMenus_GlobalCtors(void)` (0x251690): construct the
/// static menus (`AIVisualEffectsMenus_StaticInitDestroy(1, 0xffff)`).
pub fn aivisual_effects_menus_global_ctors(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        AIVISUAL_EFFECTS_MENUS_STATIC_INIT_DESTROY.call(c, 0x0025_16a4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AIVisualEffectsMenus_GlobalDtors(void)` (0x2516b0): destroy the
/// static menus (`AIVisualEffectsMenus_StaticInitDestroy(0, 0xffff)`).
pub fn aivisual_effects_menus_global_dtors(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        AIVISUAL_EFFECTS_MENUS_STATIC_INIT_DESTROY.call(c, 0x0025_16c4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AIVisualEffectsMan_dtor(AIVisualEffectsMan *this, uint flags)`
/// (0x2f8340): set the vtable at +4 to 0x487d78 and, when `flags & 1`, delete
/// `this`.
pub fn aivisual_effects_man_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x48_0000u32.wrapping_add(0x7d78));
        c.g[A1][0] &= 1;
        let delete_this = c.g[A1][0] != 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[V0][0]);
        if delete_this {
            c.cycles += 2;
            call_ptr(c, BUILTIN_DELETE_ADDR, 0x002f_8364)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Assorted leaves and leaf loops.

/// `void AIFwdDiffCache_HashRemove(AIFwdDiffCache *this, Entry *e)`
/// (0x327b30): remove the hash entry whose key (`e->key`) is `e` from the
/// bucket `key % this->bucket_count`. A `match pc` dispatcher re-enterable at
/// 0x327b78.
pub fn ai_fwd_diff_cache_hash_remove(c: &mut Cpu) {
    const ENTRY: u32 = 0x0032_7b30;
    const END: u32 = 0x0032_7bb0;
    const RESUME: u32 = 0x0032_7b78;
    let mut stage = match c.pc {
        ENTRY => 0u8,
        RESUME => 4,
        pc => {
            c.jump_out(pc, ENTRY, END);
            return;
        }
    };
    loop {
        match stage {
            0 => {
                c.cycles += 5;
                c.g[V0][0] = sx(0xffff_0000) | 0xffff;
                c.g[A1][0] = Addr(c.g[A1][0] as u32).lw(c);
                stage = if c.g[A1][0] == c.g[V0][0] { 8 } else { 1 };
            }
            1 => {
                c.cycles += 4;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lw(c);
                abi::divu(c, c.g[A1][0] as u32, c.g[V0][0] as u32);
                stage = 2;
            }
            2 => {
                c.cycles += 7;
                c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
                c.g[V1][0] = sx((c.hi[0] as u32) << 2);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V1][0] = c.g[V0][0];
                stage = if c.g[V0][0] == 0 { 8 } else { 3 };
            }
            3 => {
                c.cycles += 2;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                stage = 4;
            }
            4 => {
                c.cycles += 2;
                if c.g[V0][0] == c.g[A1][0] {
                    stage = 5;
                } else {
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                    stage = 6;
                }
            }
            5 => {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                stage = 8;
            }
            6 => {
                c.cycles += 3;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                let last = c.g[V1][0] == 0;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
                stage = if last { 8 } else { 7 };
            }
            7 => {
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.cycles += 3;
                c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] != 0 {
                    stage = 8;
                } else {
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                    if back_edge_unwinds(c, RESUME) {
                        return;
                    }
                    stage = 4;
                }
            }
            _ => {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
        }
    }
}

/// `void AlgorithmicScriptCameraAlgo_dtor(AlgorithmicScriptCameraAlgo *this, uint flags)`
/// (0x1763f8): set the vtable at +0x10 to 0x45bde0 and, when `flags & 1`,
/// delete `this`.
pub fn algorithmic_script_camera_algo_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x46_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x46_0000u32.wrapping_sub(0x4220));
        c.g[A1][0] &= 1;
        let delete_this = c.g[A1][0] != 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        if delete_this {
            c.cycles += 2;
            call_ptr(c, BUILTIN_DELETE_ADDR, 0x0017_641c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimDataBase_ResetChanCacheTable(void)` (0x3110d0): clear two
/// 48-entry tables at 0x4fe7a0 and 0x4fe860.
pub fn anim_data_base_reset_chan_cache_table(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x50_0000;
    c.g[V0][0] = 0x50_0000;
    c.g[V0][0] = sx(0x50_0000u32.wrapping_sub(0x17a0));
    c.g[V1][0] = sx(0x50_0000u32.wrapping_sub(0x1860));
    c.g[A0][0] = 0x2f;
    loop {
        c.cycles += 7;
        c.sw(c.g[V1][0] as u32, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        c.sw(c.g[V0][0] as u32, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
        let more = (c.g[A0][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event(c, 0x0031_10e8);
        }
    }
}

/// The character-class table `AV_SkipWhitespace`/`AV_StrICmp` index with a
/// byte (`lb`): 0x4a0000 - 0x637f. Bit 3 is "whitespace", bit 0 "uppercase".
const CTYPE: u32 = 0x0049_9c81;

/// `void AV_SkipWhitespace(const char **str, int *count)` (0x3af300): advance
/// `*str` over leading whitespace (per the ctype table) and decrease
/// `*count` accordingly.
pub fn av_skip_whitespace(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A3][0] = 0;
    let empty = c.g[A2][0] == 0;
    c.g[T0][0] = Addr(c.g[A0][0] as u32).lw(c);
    if empty {
        c.cycles += 6;
        av_skip_whitespace_tail(c);
        return;
    }
    c.cycles += 8;
    c.g[V1][0] = 0x4a_0000;
    c.g[V0][0] = Addr(c.g[T0][0] as u32).lb(c);
    c.g[T2][0] = sx(0x4a_0000u32.wrapping_sub(0x637f));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
    c.g[V1][0] &= 8;
    let first_ws = c.g[V1][0] != 0;
    c.g[T1][0] = c.g[A2][0];
    if !first_ws {
        c.cycles += 6;
        av_skip_whitespace_tail(c);
        return;
    }
    c.cycles += 2;
    c.g[A2][0] = c.g[T0][0];
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
    loop {
        c.cycles += 3;
        c.g[V0][0] = (c.g[A3][0] < c.g[T1][0]) as u64;
        let done = c.g[V0][0] == 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        if done {
            break;
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lb(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
        c.g[V1][0] &= 8;
        if c.g[V1][0] == 0 {
            break;
        }
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        if event_due(c) {
            deliver_event(c, 0x003a_f338);
        }
    }
    c.cycles += 6;
    av_skip_whitespace_tail(c);
}

/// The shared epilogue: advance `*str` by the skipped count and subtract it
/// from `*count`.
fn av_skip_whitespace_tail(c: &mut Cpu) {
    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
    let ret = c.g[RA][0] as u32;
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    c.pc = ret;
}

/// `int AV_StrICmp(const char *a, const char *b)` (0x3af378): a case-insensitive
/// compare using the ctype table's "uppercase" bit (0); returns the accumulated
/// character difference.
pub fn av_str_icmp(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T5][0] = c.g[A0][0];
    c.g[T4][0] = c.g[A1][0];
    c.g[V0][0] = Addr(c.g[T5][0] as u32).lb(c);
    c.g[T3][0] = 0;
    let a_end = c.g[V0][0] == 0;
    c.g[T2][0] = 0;
    if a_end {
        c.cycles += 6;
        av_str_icmp_tail(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[T4][0] as u32).lb(c);
    let b_end = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[T4][0];
    if b_end {
        c.cycles += 6;
        av_str_icmp_tail(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 0x4a_0000;
    c.g[T1][0] = c.g[T5][0];
    c.g[T6][0] = sx(0x4a_0000u32.wrapping_sub(0x637f));
    loop {
        c.cycles += 16;
        c.g[A3][0] = Addr(c.g[A0][0] as u32).lb(c);
        c.g[T0][0] = Addr(c.g[T1][0] as u32).lb(c);
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T6][0] as u32));
        c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
        c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T6][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lbu(c);
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
        c.g[V1][0] &= 1;
        c.g[A0][0] &= 1;
        if c.g[V1][0] == 0 {
            c.g[A2][0] = c.g[A3][0];
        }
        if c.g[A0][0] == 0 {
            c.g[V0][0] = c.g[T0][0];
        }
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let diff = c.g[T3][0] == 0;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
        if !diff {
            c.cycles += 2;
            let ret = c.g[RA][0] as u32;
            c.g[V0][0] = c.g[T3][0];
            c.pc = ret;
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[T1][0] as u32).lb(c);
        let a_end = c.g[V0][0] == 0;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
        if a_end {
            break;
        }
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[T2][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
        let b_more = c.g[V1][0] != 0;
        c.g[A0][0] = c.g[V0][0];
        if !b_more {
            break;
        }
        if event_due(c) {
            deliver_event(c, 0x003a_f3a8);
        }
    }
    c.cycles += 6;
    av_str_icmp_tail(c);
}

/// The shared epilogue: return the differing character bytes.
fn av_str_icmp_tail(c: &mut Cpu) {
    c.g[V0][0] = sx((c.g[T5][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
    let ret = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.pc = ret;
}

// ---------------------------------------------------------------------------
// AV player / audio wrappers.

/// `bool AV_Player_IsAudioDone(AV_Player *this)` (0x3ae860): true when there
/// is no audio object, else `AV_Audio_IsDone(this->audio(+0))`.
pub fn av_player_is_audio_done(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = 1;
        if !none {
            c.cycles += 2;
            AV_AUDIO_IS_DONE.call(c, 0x003a_e87c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_Player_SetRate(AV_Player *this, uint rate)` (0x3ae888): clamp the
/// rate to 0x4000, apply it to the audio object's pitch, and update the
/// stream's tick bookkeeping.
pub fn av_player_set_rate(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = (c.g[A1][0] < 0x4001) as u64;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x4000;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let no_audio = c.g[A0][0] == 0;
        if c.g[V0][0] != 0 {
            c.g[S1][0] = c.g[A1][0];
        }
        if !no_audio {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            AV_AUDIO_SET_PITCH.call(c, 0x003a_e8b8)?;
        }
        c.cycles += 7;
        let stream = Addr(c.g[S0][0] as u32).off(0x5c).lw(c);
        c.g[S0][0] = stream;
        c.g[A0][0] = stream;
        call_ptr(c, EATIMER_GET_TICKS, 0x003a_e8c0)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = Addr(stream as u32).off(0x10).lw(c);
        c.g[V0][0] = 0x1000;
        let not_default = c.g[A0][0] != c.g[V0][0];
        c.g[V0][0] = Addr(stream as u32).lw(c);
        if not_default {
            c.cycles += 10;
            c.g[V1][0] = Addr((stream as u32).wrapping_add(8)).ld(c);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            let prod = abi::multu(c, c.g[A0][0] as u32, c.g[V0][0] as u32);
            c.g[A2][0] = prod as u64;
            c.g[V0][0] = c.hi[0];
            c.g[A2][0] = (c.g[A2][0] << 32) >> 32;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] |= c.g[A2][0];
            c.g[V0][0] >>= 12;
        } else {
            c.cycles += 5;
            c.g[V1][0] = Addr((stream as u32).wrapping_add(8)).ld(c);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] = (c.g[V0][0] << 32) >> 32;
        }
        c.cycles += 10;
        c.g[V1][0] = c.g[V1][0].wrapping_add(c.g[V0][0]);
        Addr((c.g[S0][0] as u32).wrapping_add(8)).sd(c, c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.g[V0][0] = 0;
        c.sw(c.g[S0][0] as u32, c.g[A1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_Player_Pause(AV_Player *this)` (0x3ae938): `AV_Player_SetRate(this, 0)`.
pub fn av_player_pause(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        AV_PLAYER_SET_RATE.call(c, 0x003a_e948)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_Player_Resume(AV_Player *this)` (0x3ae958): `AV_Player_SetRate(this, 0x1000)`.
pub fn av_player_resume(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1000;
        AV_PLAYER_SET_RATE.call(c, 0x003a_e968)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Player_SetVolume(AV_Player *this, int volume)` (0x3ae978): 0 when
/// there is no audio object, else `AV_Audio_SetVolume(this->audio(+0), volume)`.
pub fn av_player_set_volume(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !none {
            c.cycles += 2;
            AV_AUDIO_SET_VOLUME.call(c, 0x003a_e994)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_Player_GetChunkCallback(AV_Player *this, void *user)` (0x3aecb8):
/// `AV_Player_OnChunk(*user, this)`.
pub fn av_player_get_chunk_callback(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A1][0] = c.g[V0][0];
        AV_PLAYER_ON_CHUNK.call(c, 0x003a_ecd0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_Player_ReleaseChunkCallback(void *unused, void *user, void *chunk)`
/// (0x3aed20): `AV_Player_ReleaseChunk(*user, chunk)`.
pub fn av_player_release_chunk_callback(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A1][0] = c.g[A2][0];
        AV_PLAYER_RELEASE_CHUNK.call(c, 0x003a_ed34)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Audio_SetPitch(AV_Audio *this, int pitch)` (0x3aee98): 0 when the
/// stream id at +0xc is -1, else `SNDSTRM_SetPitch` under the sound lock.
pub fn av_audio_set_pitch(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        let has_stream = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if !has_stream {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 8;
            SND_LOCK.call(c, 0x003a_eecc)?;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
            c.g[A1][0] = c.g[S1][0];
            SNDSTRM_SET_PITCH.call(c, 0x003a_eed8)?;
            c.g[S0][0] = c.g[V0][0];
            SND_UNLOCK.call(c, 0x003a_eee0)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Audio_Start(AV_Audio *this)` (0x3aeef8): 0 when the prebuffer id
/// at +0x10 is -1, else `SNDSTRM_SetPrebuffer(stream, 0)` under the lock.
pub fn av_audio_start(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
        let has_prebuf = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        if has_prebuf {
            c.cycles += 8;
            SND_LOCK.call(c, 0x003a_ef20)?;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
            c.g[A1][0] = 0;
            SNDSTRM_SET_PREBUFFER.call(c, 0x003a_ef2c)?;
            c.g[S0][0] = c.g[V0][0];
            SND_UNLOCK.call(c, 0x003a_ef34)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Audio_SetVolume(AV_Audio *this, int volume)` (0x3aef48): 0 when the
/// stream id at +0xc is -1, else `SNDSTRM_SetVolume` under the lock.
pub fn av_audio_set_volume(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        let has_stream = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if !has_stream {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 8;
            SND_LOCK.call(c, 0x003a_ef7c)?;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
            c.g[A1][0] = c.g[S1][0];
            SNDSTRM_SET_VOLUME.call(c, 0x003a_ef88)?;
            c.g[S0][0] = c.g[V0][0];
            SND_UNLOCK.call(c, 0x003a_ef90)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The subtitle-record size `AV_FindSubtitleByFrame` indexes with; the frame
/// number is at +0x14, the count at +0x4 and the record base at +0.
const SUBTITLE_RECORD: usize = 0x34;

// ---------------------------------------------------------------------------
// AnimController.

/// `AnimController *AnimController_ctor(AnimController *this)` (0x3118d0):
/// allocate six sequence lists at +0x50, register the vtable at +0x58 and
/// reset the controller.
pub fn anim_controller_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S1, 0x10);
        c.g[V0][0] = sx(0x49_0000u32.wrapping_sub(0x2828));
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x49_0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x54), 0);
        c.g[A1][0] = sx(0x49_0000u32.wrapping_sub(0x5b20));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x58), c.g[V0][0]);
        c.g[A0][0] = 0x30;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, OPERATOR_NEW_NAMED2_ADDR, 0x0031_190c)?;
        c.g[S0][0] = 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x50), c.g[V0][0]);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x50)).lw(c);
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 3);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            ANIM_SEQ_LIST_INIT.call(c, 0x0031_1928)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x50)).lw(c);
            if event_due(c) {
                deliver_event(c, 0x0031_1918);
            }
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S1][0];
        ANIM_CONTROLLER_RESET.call(c, 0x0031_193c)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimController_dtor(AnimController *this, uint flags)` (0x311958):
/// clear the six sequence lists, free them and, when `flags & 1`, delete
/// `this`.
pub fn anim_controller_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S1, 0x20);
        c.g[V0][0] = sx(0x49_0000u32.wrapping_sub(0x2828));
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x58), c.g[V0][0]);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x50)).lw(c);
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 3);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            ANIM_SEQ_LIST_CLEAR.call(c, 0x0031_1998)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x50)).lw(c);
            if event_due(c) {
                deliver_event(c, 0x0031_1988);
            }
        }
        c.cycles += 3;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x50)).lw(c);
        let no_lists = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S2][0] & 1;
        if !no_lists {
            c.cycles += 3;
            call_ptr(c, EA_MEM_FREE_ADDR, 0x0031_19b8)?;
            c.g[V0][0] = c.g[S2][0] & 1;
        }
        c.cycles += 2;
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            BUILTIN_DELETE.call(c, 0x0031_19cc)?;
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

/// `void AnimController_ClearLayer(AnimController *this, int layer)`
/// (0x311e38): clear the layer's sequence list and mark the layer slot 0x1b6.
pub fn anim_controller_clear_layer(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = sx((c.g[S0][0] as u32) << 3);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x50)).lw(c);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        ANIM_SEQ_LIST_CLEAR.call(c, 0x0031_1e60)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = 0x1b6;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        frame.restore_ra(c, 0);
        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `float AnimController_GetCurrentAnimLength(AnimController *this)`
/// (0x312790): the current anim's length in seconds: ask the vtable at
/// `this+0x58` for the layer, look the anim up in the tables at `gp+0xd8c` /
/// `gp+0xd08`, and scale the frame count by the constant at `gp-0x3230`.
pub fn anim_controller_get_current_anim_length(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x58)).lw(c);
        c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0031_27ac)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x207;
        let no_anim = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xd8c)).lw(c);
        if no_anim {
            c.cycles += 1;
            c.mtc1(0, 0);
        } else {
            c.cycles += 20;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xd08)).lw(c);
            c.g[A1][0] = 0x14;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3230));
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1030)).lw(c);
            c.g[V0][0] = c.g[V1][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 8);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let prod = abi::mult(c, c.g[V1][0] as u32, c.g[A1][0] as u32);
            c.g[V1][0] = sx(prod);
            c.g[A0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lhu(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 0, 1);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float AnimController_GetCurrentAnimTriggerTime(AnimController *this, void *unused, int trigger)`
/// (0x312820): the time of trigger `trigger` of the current anim, or -1.0 when
/// it is out of range.
pub fn anim_controller_get_current_anim_trigger_time(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x58)).lw(c);
        c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0031_2844)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x207;
        let is_anim = c.g[A0][0] != c.g[V0][0];
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xd8c)).lw(c);
        if !is_anim {
            c.cycles += 3;
            c.mtc1(0, 0);
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 17;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xd08)).lw(c);
        c.g[A2][0] = 0x14;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1030)).lw(c);
        c.g[V1][0] = c.g[A0][0] & 0xff;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[A0][0] as u32) >> 8);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let prod = abi::mult(c, c.g[A0][0] as u32, c.g[A2][0] as u32);
        c.g[A0][0] = sx(prod);
        c.g[A1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(8)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x12)).lhu(c);
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[AT][0] = sx(0xbf80_0000);
            c.mtc1(1, 0);
        } else {
            c.cycles += 11;
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lhu(c);
            c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x322c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
            c.mtc1(3, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 0, 1);
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float AnimController_GetLayerProgress(AnimController *this, int layer)`
/// (0x312ab0): the ratio `seq->elapsed(+8) / seq->length(+0x10)` of the layer's
/// current sequence.
pub fn anim_controller_get_layer_progress(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x50)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        ANIM_SEQ_LIST_GET_AT.call(c, 0x0031_2acc)?;
        let seq = c.g[V0][0] as u32;
        c.lwc1(1, seq.wrapping_add(0x10));
        c.lwc1(0, seq.wrapping_add(8));
        frame.restore_ra(c, 0);
        c.div_s(0, 0, 1);
        frame.close(c);
        Ok(())
    })
}

/// `AlgorithmicScriptCameraAlgo *AlgorithmicScriptCameraAlgo_ctor(...)`
/// (0x16ad38): construct the `ScriptCameraAlgo` base, initialise the camera
/// wobble at +0x70 and store the default state quad from 0x4ff130.
pub fn algorithmic_script_camera_algo_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        SCRIPT_CAMERA_ALGO_CTOR.call(c, 0x0016_ad4c)?;
        c.g[V1][0] = 0x46_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx(0x46_0000u32.wrapping_sub(0x38e8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        call_ptr(c, CAMERA_WOBBLE_INIT, 0x0016_ad60)?;
        c.g[V1][0] = 0x50_0000;
        c.g[A1][0] = 0x59;
        c.g[V1][0] = sx(0x50_0000u32.wrapping_sub(0xed0));
        c.g[V0][0] = c.g[S0][0];
        c.lq(4, c.g[V1][0] as u32);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
        c.sq(4, (c.g[S0][0] as u32).wrapping_add(0x30));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x44), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AlgorithmList_Push(AlgorithmList *list, void *payload, void *arg, float blend f12)`
/// (0x15c988): allocate a 0x1c-byte node, run the payload's prepare slot
/// (`payload+0x10 -> +0x20/+0x24`), link it at the head and remember the
/// per-node blend (1.0 or 0 when `blend == 1.0`).
pub fn algorithm_list_push(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        c.swc1(20, frame.at(c, 0x40));
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0015_c9c4)?;
        c.g[A1][0] = 0x46_0000;
        c.g[A0][0] = 0x1c;
        c.g[A1][0] = sx(0x46_0000u32.wrapping_sub(0x4e10));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, OPERATOR_NEW_NAMED_ADDR, 0x0015_c9dc)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = c.g[V0][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x18), c.g[S1][0]);
        c.c_eq_s(20, 0);
        c.sw(c.g[A0][0] as u32, c.g[S0][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x10), 0);
        let blend_ne_one = !c.fcc();
        c.sw((c.g[A0][0] as u32).wrapping_add(0x14), 0);
        if blend_ne_one {
            c.cycles += 1;
            c.sw((c.g[A0][0] as u32).wrapping_add(4), 0);
        } else {
            c.cycles += 2;
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(4));
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
        let empty = c.g[V0][0] == 0;
        c.swc1(20, (c.g[A0][0] as u32).wrapping_add(8));
        if !empty {
            c.cycles += 1;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
        }
        c.cycles += 13;
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(4)).lw(c);
        c.g[V1][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.sw(c.g[S2][0] as u32, c.g[A0][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(4), c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.lwc1(20, frame.at(c, 0x40));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimDataBase.

/// `AnimDataBase *AnimDataBase_ctor(AnimDataBase *this)` (0x311048): clear the
/// 0x3fc-byte object, register it in the global at `gp + 0xd08`, and reset the
/// channel-cache table.
pub fn anim_data_base_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x3fc;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[GP][0] as u32).wrapping_add(0xd08), c.g[S0][0]);
        call_ptr(c, MEMSET_ADDR, 0x0031_1068)?;
        c.g[A0][0] = c.g[S0][0];
        ANIM_DATA_BASE_RESET.call(c, 0x0031_1070)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimDataBase_dtor(AnimDataBase *this, uint flags)` (0x311088): clear
/// the global at `gp + 0xd08`, free the channel caches and, when `flags & 1`,
/// delete `this`.
pub fn anim_data_base_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.sw((c.g[GP][0] as u32).wrapping_add(0xd08), 0);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        ANIM_DATA_BASE_FREE.call(c, 0x0031_10a8)?;
        c.g[S0][0] &= 1;
        let delete_this = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete_this {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            BUILTIN_DELETE.call(c, 0x0031_10bc)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ADF data source and animation combo.

/// `AdfDataSource *AdfDataSource_ctor(AdfDataSource *this, name)` (0x311250):
/// load the file through `EAFile_LoadDecompressedThunk(name, 0)`, store it at
/// +0 and hand it to `AdfDataSource_SetData`.
pub fn adf_data_source_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = 0;
        call_ptr(c, EA_FILE_LOAD_DECOMPRESSED_THUNK, 0x0031_126c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        ADF_DATA_SOURCE_SET_DATA.call(c, 0x0031_127c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AdfDataSource_dtor(AdfDataSource *this, uint flags)` (0x3112c8): free the
/// loaded data (+0) and, when `flags & 1`, delete `this`.
pub fn adf_data_source_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let no_data = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_data {
            c.cycles += 2;
            call_ptr(c, EA_MEM_FREE_ADDR, 0x0031_12f0)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            BUILTIN_DELETE.call(c, 0x0031_1304)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AnimCombo_dtor(AnimCombo *this, uint flags)` (0x358e50): set the vtable at
/// +0x54, free the snapshot matrices (+0x84), destruct the `AnimObject` at
/// +0x18 and, when `flags & 1`, delete `this`.
pub fn anim_combo_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x49_0000u32.wrapping_sub(0x13d8));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x84)).lw(c);
        let no_snap = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_snap {
            c.cycles += 2;
            call_ptr(c, EA_MEM_FREE_ADDR, 0x0035_8e84)?;
        }
        c.cycles += 6;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = 0;
        ANIM_OBJECT_DTOR.call(c, 0x0035_8e90)?;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            BUILTIN_DELETE.call(c, 0x0035_8ea4)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AnimCombo_v42(SubObj *sub, void *out)` (0x358f90): call `ModObj_v42` on
/// `sub+0x48`, then copy the two fixed quads at 0x4ff120 to `out+0x20` and
/// `out+0x30`.
pub fn anim_combo_v42(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        MOD_OBJ_V42.call(c, 0x0035_8fa8)?;
        c.g[A0][0] = 0x50_0000;
        frame.restore_ra(c, 0);
        c.g[A0][0] = sx(0x50_0000u32.wrapping_sub(0xee0));
        c.lq(2, c.g[A0][0] as u32);
        c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.lq(3, c.g[A0][0] as u32);
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0x30));
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `AnimCombo *AnimCombo_ctor_2(AnimCombo *this, void *a1, void *a2)`
/// (0x358db0): construct the `AnimObject` at +0x18, run the vtable slot at
/// `*a2 -> +0x10/+0x14`, allocate the snapshot buffer from the bone count and
/// snapshot the matrices.
pub fn anim_combo_ctor_2(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x84));
        ANIM_OBJECT_CTOR_2.call(c, 0x0035_8dd8)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = sx(0x49_0000u32.wrapping_sub(0x13d8));
        c.g[A2][0] = 0x18;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V1][0]);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x0035_8e00)?;
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x74)).lw(c);
        c.g[A1][0] = 0x49_0000;
        c.g[A1][0] = sx(0x49_0000u32.wrapping_sub(0x15e0));
        c.g[A2][0] = 0x2000_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32) << 6);
        c.g[A3][0] = 0;
        call_ptr(c, OPERATOR_NEW_NAMED2_ADDR, 0x0035_8e1c)?;
        let empty = c.g[S0][0] == 0;
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
        if !empty {
            c.cycles += 1;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 10;
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        ANIM_COMBO_SNAPSHOT.call(c, 0x0035_8e34)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimCombo_Kill(AnimCombo *this)` (0x3590c8): kill the `WorldObj` at
/// +0x48 unless the alive flag at +0x15 is set.
pub fn anim_combo_kill(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x15)).lb(c);
        let alive = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !alive {
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
            WORLD_OBJ_KILL.call(c, 0x0035_90e4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void AnimCombo_Save(AnimCombo *this, State *out)` (0x3590f0): save the
/// `AnimObject` at +0x18 and forward to `out->save` (`out+0 -> +8/+0xc`).
pub fn anim_combo_save(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
        ANIM_OBJECT_SAVE.call(c, 0x0035_9110)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x18;
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0035_912c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimCombo_v52(AnimCombo *this)` (0x359140): forward to
/// `AnimObject_v52(this+0x34)` and, when the bones are ready, multiply every
/// bone matrix into the snapshot buffer.
pub fn anim_combo_v52(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
        ANIM_OBJECT_V52.call(c, 0x0035_916c)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x7c)).lw(c);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x60);
            anim_combo_v52_epilogue(c, &frame);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x14)).lb(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x17)).lb(c);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x60);
                anim_combo_v52_epilogue(c, &frame);
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x15)).lb(c);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x60);
                anim_combo_v52_epilogue(c, &frame);
                return Ok(());
            }
            c.cycles += 1;
        }
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x60)).lw(c);
        c.cycles += 6;
        c.g[S5][0] = 0;
        c.g[S2][0] = 0;
        c.g[S3][0] = Addr(c.g[V1][0] as u32).off(0x80).lw(c);
        c.g[V0][0] = Addr(c.g[S3][0] as u32).off(4).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = Addr(c.g[S3][0] as u32).off(8).lw(c);
        if !empty {
            c.cycles += 2;
            c.g[S4][0] = 0;
            loop {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(8).lw(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1c).lw(c);
                let skip = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[S5][0] as u32) << 6);
                if !skip {
                    c.cycles += 7;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x7c)).lw(c);
                    c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0xd0));
                    INTERSECT_INFO_MUL_MATRIX.call(c, 0x0035_91ec)?;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S3][0] as u32).off(4).lw(c);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event(c, 0x0035_91c0);
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x60);
        anim_combo_v52_epilogue(c, &frame);
        Ok(())
    })
}

/// The shared epilogue of [`anim_combo_v52`].
fn anim_combo_v52_epilogue(c: &mut Cpu, frame: &Frame) {
    c.cycles += 8;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void AnimCombo_SnapshotMatrices(AnimCombo *this)` (0x359030): copy each
/// bone's four quads (stepping 0xd0) into the snapshot buffer; VU0 `lqc2`/
/// `sqc2` set VF1-VF4 (the last copied bone is left in them).
pub fn anim_combo_snapshot_matrices(c: &mut Cpu) {
    c.cycles += 4;
    let t0 = c.g[A0][0];
    c.g[T0][0] = t0;
    c.g[V0][0] = Addr(t0 as u32).off(0x74).lw(c);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[T2][0] = 0;
    if empty {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = Addr(t0 as u32).off(0x60).lw(c);
    c.g[A2][0] = 0;
    c.g[A3][0] = Addr(c.g[V1][0] as u32).off(0x80).lw(c);
    c.g[V0][0] = Addr(c.g[A3][0] as u32).off(4).lw(c);
    let empty2 = (c.g[V0][0] as i64) <= 0;
    c.g[A1][0] = Addr(c.g[A3][0] as u32).off(8).lw(c);
    if empty2 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[T3][0] = sx((t0 as u32).wrapping_add(0x34));
    c.g[T1][0] = 0;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(8).lw(c);
        let skip = c.g[V0][0] == 0;
        c.g[V1][0] = sx((c.g[T2][0] as u32) << 6);
        if !skip {
            c.cycles += 15;
            c.g[V0][0] = Addr(c.g[T3][0] as u32).off(0x48).lw(c);
            c.g[A0][0] = Addr(t0 as u32).off(0x84).lw(c);
            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x90));
            let src = c.g[V0][0] as u32;
            let dst = c.g[V1][0] as u32;
            c.lqc2(1, src);
            c.lqc2(2, src.wrapping_add(0x10));
            c.lqc2(3, src.wrapping_add(0x20));
            c.lqc2(4, src.wrapping_add(0x30));
            c.sqc2(1, dst);
            c.sqc2(2, dst.wrapping_add(0x10));
            c.sqc2(3, dst.wrapping_add(0x20));
            c.sqc2(4, dst.wrapping_add(0x30));
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0xd0));
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
        }
        c.cycles += 5;
        c.g[V0][0] = Addr(c.g[A3][0] as u32).off(4).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event(c, 0x0035_9060);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Air predictor and algorithm lists.

/// `void AirPredictor_SaveState(AirPredictor *this, State *out)` (0x113aa0):
/// call `out->save` (`out+0 -> +8/+0xc`).
pub fn air_predictor_save_state(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = 0xb0;
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0011_3ac8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AirPredictor_LoadState(AirPredictor *this, State *in)` (0x113ad8):
/// call `in->load` (`in+0 -> +0x10/+0x14`).
pub fn air_predictor_load_state(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = 0xb0;
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lw(c);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0011_3b00)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AlgorithmicScriptCameraAlgo_Reset(AlgorithmicScriptCameraAlgo *this)`
/// (0x16b150): call the vtable slot at `this+0x10 -> +0x18/+0x1c` with
/// `arg = 0`.
pub fn algorithmic_script_camera_algo_reset(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        let vt_holder = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[V0][0] = vt_holder;
        c.g[V1][0] = Addr((vt_holder as u32).wrapping_add(0x18)).lh(c);
        c.g[A2][0] = Addr((vt_holder as u32).wrapping_add(0x1c)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, c.g[A2][0] as u32, 0x0016_b170)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// A node in an `AlgorithmList`: payload at +0, next at +0x14, flags at +0x18.
const ALG_NODE_PAYLOAD: u32 = 0x0;
const ALG_NODE_NEXT: u32 = 0x14;
const ALG_NODE_FLAGS: u32 = 0x18;

/// `void AlgorithmList_dtor(AlgorithmList *list, uint flags)` (0x15c940):
/// clear the list, then delete it when `flags & 1`.
pub fn algorithm_list_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        ALGORITHM_LIST_CLEAR.call(c, 0x0015_c95c)?;
        c.g[S0][0] &= 1;
        let delete_list = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete_list {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, BUILTIN_DELETE_ADDR, 0x0015_c970)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AlgorithmList_Clear(AlgorithmList *list)` (0x15cb08): delete every
/// node (running its payload destructor first) and null the head and count.
pub fn algorithm_list_clear(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = Addr(c.g[S2][0] as u32).lw(c);
        if c.g[S0][0] == 0 {
            c.sw(c.g[S2][0] as u32, 0);
        } else {
            c.cycles += 1;
            loop {
                c.cycles += 2;
                c.g[S1][0] =
                    Addr((c.g[S0][0] as u32).wrapping_add(ALG_NODE_NEXT)).lw(c);
                c.cycles += 3;
                c.g[A2][0] =
                    Addr((c.g[S0][0] as u32).wrapping_add(ALG_NODE_PAYLOAD)).lw(c);
                if c.g[A2][0] == 0 {
                    c.sw(c.g[S0][0] as u32, 0);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(ALG_NODE_FLAGS)).lw(c);
                    let no_flags = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x3;
                    if !no_flags {
                        c.cycles += 5;
                        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x10)).lw(c);
                        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lh(c);
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, c.g[V1][0] as u32, 0x0015_cb64)?;
                    }
                    c.cycles += 1;
                    c.sw(c.g[S0][0] as u32, 0);
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(ALG_NODE_FLAGS), 0);
                call_ptr(c, BUILTIN_DELETE_ADDR, 0x0015_cb74)?;
                c.cycles += 3;
                c.g[S0][0] = c.g[S1][0];
                let more = c.g[S0][0] != 0;
                if !more {
                    c.cycles += 1;
                    c.sw(c.g[S2][0] as u32, 0);
                    break;
                }
                if event_due(c) {
                    deliver_event(c, 0x0015_cb30);
                }
            }
        }
        c.cycles += 7;
        c.sw((c.g[S2][0] as u32).wrapping_add(4), 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_FreeSubtitles(SubtitleList *list, uint flags)` (0x3af020): free
/// the block holding the subtitle records (`list->base(+0) - 0x10`, sized by
/// `count(+4) * 0x34`) and clear the list; when `flags & 1` also delete it.
pub fn av_free_subtitles(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        let no_base = c.g[A1][0] == 0;
        c.g[V0][0] = SUBTITLE_RECORD as u64;
        if !no_base {
            c.cycles += 6;
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_sub(0x10)).lw(c);
            c.g[A0][0] = c.g[A1][0];
            let prod = abi::mult(c, c.g[V1][0] as u32, c.g[V0][0] as u32);
            c.g[A2][0] = sx(prod);
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            let aligned = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = c.g[A0][0];
            if !aligned {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(SUBTITLE_RECORD as u32));
                loop {
                    c.cycles += 7;
                    let more = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] =
                        sx((c.g[V0][0] as u32).wrapping_sub(SUBTITLE_RECORD as u32));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event(c, 0x003a_f060);
                    }
                }
            }
            c.cycles += 6;
            c.g[V1][0] = 0x51_0000;
            c.g[V1][0] = sx(0x51_0000u32.wrapping_sub(0x6bd0));
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, c.g[V0][0] as u32, 0x003a_f090)?;
            c.sw(c.g[S0][0] as u32, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        let delete_list = c.g[V0][0] != 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(4), 0);
        if delete_list {
            c.cycles += 4;
            c.g[V1][0] = 0x51_0000;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x6bcc)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, c.g[V0][0] as u32, 0x003a_f0b0)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Subtitle *AV_FindSubtitleByFrame(SubtitleList *list, uint frame)`
/// (0x3af258): binary-search the sorted records and return the nearest record
/// at or before `frame` (null for an empty list). FPU-free but uses `mult`.
pub fn av_find_subtitle_by_frame(c: &mut Cpu) {
    c.cycles += 4;
    let list = c.g[A0][0];
    c.g[V1][0] = list;
    c.g[V0][0] = Addr(list as u32).off(4).lw(c);
    let empty = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[V0][0];
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[T2][0] = Addr(list as u32).lw(c);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
    c.g[T1][0] = c.g[A0][0];
    c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
    c.g[A3][0] = u64::MAX;
    // count is non-zero here, so the `count < 1` exit below is unreachable.
    c.cycles += 2;
    c.g[T0][0] = SUBTITLE_RECORD as u64;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    loop {
        c.cycles += 9;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        let prod = abi::mult(c, c.g[A2][0] as u32, c.g[T0][0] as u32);
        c.g[V0][0] = sx(prod);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lw(c);
        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
        if c.g[V0][0] != 0 {
            c.g[A3][0] = c.g[A2][0];
        } else {
            c.cycles += 1;
            c.g[A0][0] = c.g[A2][0];
        }
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
        if c.g[V0][0] != 0 {
            break;
        }
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        if event_due(c) {
            deliver_event(c, 0x003a_f290);
        }
    }
    c.cycles += 11;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
    c.g[V1][0] = u64::MAX;
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(1));
    if c.g[V1][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[A1][0] = SUBTITLE_RECORD as u64;
    c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[V0][0] = c.g[A0][0];
    }
    let prod = abi::mult(c, c.g[V0][0] as u32, c.g[A1][0] as u32);
    c.g[V1][0] = sx(prod);
    let ret = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.pc = ret;
}

/// `Subtitle *AV_Player_GetCurrentSubtitle(AV_Player *this)` (0x3ae718): the
/// subtitle for the decoder's current frame (or the frame derived from the
/// rate at +0x48), null when the player has no subtitles.
pub fn av_player_get_current_subtitle(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x70).lw(c);
        let no_list = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if no_list {
            c.cycles += 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x74).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x44).lw(c);
                AV_FIND_SUBTITLE_BY_FRAME.call(c, 0x003a_e748)?;
            } else {
                c.cycles += 7;
                c.f[12] = Addr((c.g[S0][0] as u32).wrapping_add(0x48)).read_u32(c);
                call_ptr(c, FIXUNSSFSI, 0x003a_e758)?;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x70).lw(c);
                c.g[A1][0] = c.g[V0][0];
                AV_FIND_SUBTITLE_BY_FRAME.call(c, 0x003a_e764)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Audio_IsDone(AV_Audio *this)` (0x3aefa8): under the sound lock, ask
/// the stream for its status and current request; 0 while either is pending,
/// else true when the finished count at `sp+8` is below 1.
pub fn av_audio_is_done(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        SND_LOCK.call(c, 0x003a_efbc)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        c.g[A1][0] = sx(frame.at(c, 0x10));
        SNDSTRM_GET_STATUS.call(c, 0x003a_efc8)?;
        let mut early = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = Addr(frame.at(c, 0x14)).lw(c);
        if !early {
            c.cycles += 4;
            c.g[A1][0] = c.g[SP][0];
            SNDSTRM_GET_REQUEST_STATUS.call(c, 0x003a_efd8)?;
            early = (c.g[V0][0] as i64) < 0;
        }
        if early {
            c.cycles += 3;
            SND_UNLOCK.call(c, 0x003a_effc)?;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 5;
            SND_UNLOCK.call(c, 0x003a_efe8)?;
            c.g[V0][0] = Addr(frame.at(c, 8)).lw(c);
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, ptr, vtable, LOG_RESULTS};
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 300;

    /// The allocator mutex (`gp`-relative globals); marked as held by "this
    /// thread" (fake `GetThreadId` result 0x42) so the real allocator entry
    /// points take their fast path.
    const MEM_MUTEX: u32 = 0x0053_8b00;

    fn alloc_setup(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(MEM_MUTEX + 4, 0x42);
        c.bus.write32(LOG_RESULTS, 0x42);
        c.bus.write32(MEM_MUTEX + 8, r.next_u32() % 4);
    }

    #[test]
    fn ai_anim_event_map_ctor_matches() {
        check(ee::s10::f_003115e0, ai_anim_event_map_ctor, SEEDS, &|c, seed| {
            let mut r = Rng(0x0031_15e0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            // The event table copied onto the stack: values are small ids so
            // the stores land in RAM; some equal 0x1b6 to hit the skip branch.
            for i in 0..472u32 {
                let v = if (i + seed) % 7 == 0 { 0x1b6 } else { (r.next_u32() % 0x1b6) };
                c.bus.write32(0x489d60 + i * 4, v);
            }
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn ai_anim_event_map_create_matches() {
        check(ee::s10::f_00311870, ai_anim_event_map_create, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1870, 0);
            alloc_setup(c, &mut r);
        });
    }

    #[test]
    fn ai_anim_event_map_destroy_matches() {
        check(ee::s10::f_003118a8, ai_anim_event_map_destroy, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x18a8, 0);
            alloc_setup(c, &mut r);
            // Non-null so `__builtin_delete` really runs.
            c.bus.write32(GAME_GP.wrapping_add(0xd0c), if seed % 4 == 0 { 0 } else { DATA + 0x100 });
        });
    }

    #[test]
    fn ai_fwd_diff_cache_dtor_matches() {
        check(ee::s10::f_00327828, ai_fwd_diff_cache_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x7828, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            // Cover null and non-null for both owned blocks.
            c.bus.write32(DATA, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
            c.bus.write32(DATA + 8, if seed % 5 == 0 { 0 } else { DATA + 0x200 });
        });
    }

    // ------------------------------------------------------- syscall wrappers

    #[test]
    fn add_intc_and_dmac_handler_match() {
        for (gen, hand) in [
            (ee::s15::f_00423a80 as fn(&mut Cpu), add_intc_handler as fn(&mut Cpu)),
            (ee::s15::f_00423a90, add_intc_handler_2),
            (ee::s15::f_00423ab0, add_dmac_handler),
            (ee::s15::f_00423ac0, add_dmac_handler_2),
        ] {
            covered(gen, hand, SEEDS, &|c, seed| {
                let _ = machine(c, seed, 0x423a80, 1);
            });
        }
    }

    // ------------------------------------ AI visual-effect processes / menus

    /// A `this` whose sub-effect pointer (`+0x10`) is a fake vtable entry.
    fn fx_process(c: &mut Cpu, r: &mut Rng, sub: bool, slot: u32) -> u32 {
        let obj = ptr(r);
        c.g[A0][0] = obj as u64;
        if sub {
            let inner = ptr(r);
            c.bus.write32(obj + 0x10, inner);
            let vt = vtable(c, r, &[slot]);
            c.bus.write32(inner + 4, vt);
        } else {
            c.bus.write32(obj + 0x10, 0);
        }
        obj
    }

    #[test]
    fn aivfx_process_dtor_matches() {
        covered(ee::s06::f_002448e0, aivfx_process_dtor, 150, &|c, seed| {
            let mut r = machine(c, seed, 0x48e0, 1);
            c.g[A1][0] = (seed % 2) as u64;
            fx_process(c, &mut r, seed % 3 != 0, 8);
        });
    }

    #[test]
    fn aivfx_process_update_render_match() {
        for (gen, hand, slot) in [
            (ee::s06::f_00244948 as fn(&mut Cpu), aivfx_process_update as fn(&mut Cpu), 0x10u32),
            (ee::s06::f_00244978, aivfx_process_render, 0x18),
        ] {
            covered(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, 0x4948, 1);
                fx_process(c, &mut r, true, slot);
            });
        }
    }

    #[test]
    fn aivisual_effects_main_menu_ctor_matches() {
        check(ee::s06::f_002509d8, aivisual_effects_main_menu_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x09d8, 0);
            c.g[A0][0] = DATA as u64;
            crate::difftest::fill(c, &mut r, DATA, 0x200 / 4);
        });
    }

    #[test]
    fn aivisual_effects_main_menu_dtor_matches() {
        check(ee::s06::f_00251630, aivisual_effects_main_menu_dtor, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x1630, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            crate::difftest::fill(c, &mut r, DATA, 0x200 / 4);
        });
    }

    #[test]
    fn aivisual_effects_main_menu_activate_match() {
        for (gen, hand, salt) in [
            (ee::s06::f_00250a60 as fn(&mut Cpu), aivisual_effects_main_menu_on_activate as fn(&mut Cpu), 0x0a60u64),
            (ee::s06::f_00250a80, aivisual_effects_main_menu_on_deactivate, 0x0a80),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A0][0] = DATA as u64;
                crate::difftest::fill(c, &mut r, DATA, 0x100 / 4);
            });
        }
    }

    #[test]
    fn aivisual_effects_menus_globals_match() {
        for (gen, hand, salt) in [
            (ee::s06::f_00251690 as fn(&mut Cpu), aivisual_effects_menus_global_ctors as fn(&mut Cpu), 0x1690u64),
            (ee::s06::f_002516b0, aivisual_effects_menus_global_dtors, 0x16b0),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                let _ = machine(c, seed, salt, 0);
            });
        }
    }

    #[test]
    fn aivisual_effects_man_dtor_matches() {
        check(ee::s09::f_002f8340, aivisual_effects_man_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x8340, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    // ---------------------------------------------- active camera trigger list

    /// Build a linked list of `nodes` 0x10-byte nodes at DATA+0x1000 and store
    /// its head at `list`; ids are small so `Find`/`Remove` can match them.
    fn build_list(c: &mut Cpu, r: &mut Rng, list: u32, nodes: u32) -> u32 {
        if nodes == 0 {
            c.bus.write32(list, 0);
            return 0;
        }
        let base = DATA + 0x1000;
        for i in 0..nodes {
            let next = if i + 1 < nodes { base + (i + 1) * 0x10 } else { 0 };
            c.bus.write32(base + i * 0x10, r.next_u32() % 8);
            c.bus.write32(base + i * 0x10 + 4, next);
            c.bus.write32(base + i * 0x10 + 8, r.next_u32());
            c.bus.write32(base + i * 0x10 + 0xc, r.next_u32());
        }
        c.bus.write32(list, base);
        base
    }

    #[test]
    fn active_camera_trigger_list_dtor_matches() {
        check(ee::s02::f_0016c5e0, active_camera_trigger_list_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc5e0, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 3) as u64;
            build_list(c, &mut r, DATA, seed % 2);
        });
    }

    #[test]
    fn active_camera_trigger_list_clear_matches() {
        check(ee::s02::f_0016c628, active_camera_trigger_list_clear, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc628, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            build_list(c, &mut r, DATA, seed % 2);
        });
    }

    #[test]
    fn active_camera_trigger_list_clear_flags_matches() {
        check(ee::s02::f_0016c678, active_camera_trigger_list_clear_flags, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc678, 0);
            c.g[A0][0] = DATA as u64;
            build_list(c, &mut r, DATA, seed % 5);
        });
    }

    #[test]
    fn active_camera_trigger_list_find_matches() {
        check(ee::s02::f_0016c6b0, active_camera_trigger_list_find, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc6b0, 0);
            let nodes = seed % 4;
            let base = build_list(c, &mut r, DATA, nodes);
            c.g[A0][0] = DATA as u64;
            let id = if nodes > 0 && seed % 3 != 0 {
                c.bus.read32(base + (seed % nodes) * 0x10)
            } else {
                r.next_u32()
            };
            c.g[A1][0] = sx(id);
        });
    }

    #[test]
    fn active_camera_trigger_list_add_matches() {
        check(ee::s02::f_0016c718, active_camera_trigger_list_add, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc718, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = sx(r.next_u32());
            c.bus.write32(DATA, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
        });
    }

    #[test]
    fn active_camera_trigger_list_remove_matches() {
        check(ee::s02::f_0016c778, active_camera_trigger_list_remove, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc778, 0);
            alloc_setup(c, &mut r);
            let nodes = (seed % 4) + 1;
            let base = build_list(c, &mut r, DATA, nodes);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = sx(match seed % 4 {
                0 => base,
                1 => base + ((seed as u32 % nodes) * 0x10),
                2 => base + (nodes - 1) * 0x10,
                _ => base + 0x100,
            });
        });
    }

    // ------------------------------------------------------ AV player/audio

    /// An `AV_Player` at DATA with a stream at DATA+0x100; `audio` adds an
    /// audio object at DATA+0x200 pointing at the stream.
    fn av_player(c: &mut Cpu, r: &mut Rng, audio: bool) -> u32 {
        let this = DATA;
        let stream = DATA + 0x100;
        c.g[A0][0] = this as u64;
        c.bus.write32(this + 0x5c, stream);
        c.bus.write32(stream, r.next_u32());
        c.bus.write32(
            stream + 0x10,
            if r.next_u32() % 2 == 0 { 0x1000 } else { r.next_u32() % 0x4000 },
        );
        c.bus.write64(stream + 8, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
        c.bus.write32(this + 0x50, r.next_u32());
        c.bus.write32(this + 0x58, r.next_u32());
        if audio {
            let a = DATA + 0x200;
            c.bus.write32(this, a);
            c.bus.write32(a + 0xc, stream);
            c.bus.write32(a + 0x10, r.next_u32());
            c.bus.write64(a + 8, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
        } else {
            c.bus.write32(this, 0);
        }
        this
    }

    #[test]
    fn av_player_rate_wrappers_match() {
        for (gen, hand, salt) in [
            (ee::s13::f_003ae938 as fn(&mut Cpu), av_player_pause as fn(&mut Cpu), 0xe938u64),
            (ee::s13::f_003ae958, av_player_resume, 0xe958),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                av_player(c, &mut r, seed % 2 == 0);
            });
        }
    }

    #[test]
    fn av_player_set_rate_matches() {
        check(ee::s13::f_003ae888, av_player_set_rate, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xe888, 0);
            av_player(c, &mut r, seed % 2 == 0);
            c.g[A1][0] = if seed % 3 == 0 { 0x4000 } else { r.next_u32() as u64 };
        });
    }

    #[test]
    fn av_player_is_audio_done_matches() {
        check(ee::s13::f_003ae860, av_player_is_audio_done, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xe860, 0);
            av_player(c, &mut r, seed % 3 != 0);
        });
    }

    #[test]
    fn av_player_set_volume_matches() {
        check(ee::s13::f_003ae978, av_player_set_volume, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xe978, 0);
            av_player(c, &mut r, seed % 3 != 0);
            c.g[A1][0] = sx(r.next_u32());
        });
    }

    /// An `AV_Audio` at DATA with a stream handle at `field` (or -1).
    fn av_audio(c: &mut Cpu, r: &mut Rng, field: u32, neg: bool) {
        let audio = DATA;
        c.g[A0][0] = audio as u64;
        c.bus.write32(audio + field, if neg { 0xffff_ffff } else { DATA + 0x100 });
        crate::difftest::fill(c, r, DATA + 0x100, 0x40);
    }

    #[test]
    fn av_audio_pitch_volume_match() {
        for (gen, hand, field, salt) in [
            (ee::s13::f_003aee98 as fn(&mut Cpu), av_audio_set_pitch as fn(&mut Cpu), 0xcu32, 0xee98u64),
            (ee::s13::f_003aef48, av_audio_set_volume, 0xc, 0xef48),
        ] {
            check(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                av_audio(c, &mut r, field, seed % 3 == 0);
                c.g[A1][0] = sx(r.next_u32());
            });
        }
    }

    #[test]
    fn av_audio_start_matches() {
        check(ee::s13::f_003aeef8, av_audio_start, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xeef8, 0);
            av_audio(c, &mut r, 0x10, seed % 3 == 0);
        });
    }

    #[test]
    fn av_audio_is_done_matches() {
        check(ee::s13::f_003aefa8, av_audio_is_done, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xefa8, 0);
            av_audio(c, &mut r, 0xc, false);
        });
    }

    /// Subtitle records (0x34 bytes, frame at +0x14) at DATA+0x200, list head
    /// at DATA. Returns the list.
    fn subtitle_list(c: &mut Cpu, r: &mut Rng, n: u32) -> u32 {
        let list = DATA;
        let base = DATA + 0x200;
        c.bus.write32(list, base);
        c.bus.write32(list + 4, n);
        for i in 0..n {
            c.bus.write32(base + i * 0x34 + 0x14, i * 10);
            c.bus.write32(base + i * 0x34 + 0x18, r.next_u32());
        }
        list
    }

    #[test]
    fn av_free_subtitles_matches() {
        covered(ee::s13::f_003af020, av_free_subtitles, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xaf020, 1);
            let obj = DATA;
            let base = DATA + 0x100;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(base.wrapping_sub(0x10), seed % 4);
            c.bus.write32(obj, if seed % 5 == 0 { 0 } else { base });
            c.bus.write32(obj + 4, r.next_u32());
        });
    }

    #[test]
    fn av_find_subtitle_by_frame_matches() {
        check_lohi(ee::s13::f_003af258, av_find_subtitle_by_frame, SEEDS, &|c, seed| {
            let mut r = Rng(0x3af258 ^ seed as u64);
            scramble(c, &mut r);
            let n = seed % 6;
            subtitle_list(c, &mut r, n);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = sx(r.next_u32() % 60);
        });
    }

    #[test]
    fn av_player_get_current_subtitle_matches() {
        check(ee::s13::f_003ae718, av_player_get_current_subtitle, 150, &|c, seed| {
            let mut r = machine(c, seed, 0xe718, 0);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let list = if seed % 4 == 0 { 0 } else { subtitle_list(c, &mut r, seed % 5) };
            c.bus.write32(this + 0x70, list);
            c.bus.write32(this + 0x74, if seed % 2 == 0 { 0 } else { r.next_u32() % 50 });
            c.bus.write32(this + 0x44, r.next_u32() % 50);
            c.bus.write32(this + 0x48, r.edge_f32());
        });
    }

    #[test]
    fn av_player_release_chunk_callback_matches() {
        check(ee::s13::f_003aed20, av_player_release_chunk_callback, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xed20, 0);
            let player = DATA;
            let chunk = DATA + 0x300;
            let user = DATA + 0x500;
            c.bus.write32(user, player);
            c.bus.write32(player + 0x30, DATA + 0x100);
            c.bus.write32(chunk, r.next_u32());
            c.g[A1][0] = user as u64;
            c.g[A2][0] = if seed % 3 == 0 { 0 } else { chunk as u64 };
        });
    }

    #[test]
    fn anim_combo_ctor_2_matches() {
        check(ee::s11::f_00358db0, anim_combo_ctor_2, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x8db0, 0);
            alloc_setup(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            let vt = vtable(c, &mut r, &[0x10]);
            c.bus.write32(DATA + 0x200, vt);
            c.bus.write32(this + 0x74, if seed % 3 == 0 { 0 } else { seed % 3 });
            crate::difftest::fill(c, &mut r, this + 0x18, 0x80);
            crate::difftest::fill(c, &mut r, DATA + 0x2000, 0x400 / 4);
        });
    }

    #[test]
    fn anim_combo_kill_matches() {
        check(ee::s11::f_003590c8, anim_combo_kill, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x90c8, 0);
            c.g[A0][0] = DATA as u64;
            c.bus.write8(DATA + 0x15, if seed % 2 == 0 { 0 } else { 1 });
            crate::difftest::fill(c, &mut r, DATA + 0x48, 0x40);
        });
    }

    #[test]
    fn anim_combo_save_matches() {
        check(ee::s11::f_003590f0, anim_combo_save, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x90f0, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            let vt = vtable(c, &mut r, &[8]);
            c.bus.write32(DATA + 0x200, vt);
            crate::difftest::fill(c, &mut r, DATA + 0x18, 0x40);
        });
    }

    #[test]
    fn anim_controller_get_current_anim_length_matches() {
        check(ee::s10::f_00312790, anim_controller_get_current_anim_length, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2790, 1);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let vt = vtable(c, &mut r, &[0x20]);
            c.bus.write32(this + 0x58, vt);
            let index = if seed % 4 == 0 { 0x207 } else { (seed % 5) + 1 };
            c.bus.write32(crate::difftest::fakes::LOG_RESULTS, index);
            // Anim table at gp+0xd8c: entry +0x1030 holds the packed index.
            let table = DATA + 0x200;
            c.bus.write32(GAME_GP + 0xd8c, table);
            c.bus.write32(table + index * 4 + 0x1030, 0x203);
            // Channel-cache table at gp+0xd08.
            c.bus.write32(GAME_GP + 0xd08, DATA + 0x400);
            c.bus.write32(DATA + 0x400 + 0x0c, DATA + 0x600);
            c.bus.write32(DATA + 0x608, 0x100);
            c.bus.write16(DATA + 0x134, 5);
            c.bus.write32(GAME_GP - 0x3230, r.edge_f32());
        });
    }

    #[test]
    fn anim_controller_get_current_anim_trigger_time_matches() {
        check(ee::s10::f_00312820, anim_controller_get_current_anim_trigger_time, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2820, 1);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let vt = vtable(c, &mut r, &[0x20]);
            c.bus.write32(this + 0x58, vt);
            let index = if seed % 4 == 0 { 0x207 } else { (seed % 5) + 1 };
            c.bus.write32(crate::difftest::fakes::LOG_RESULTS, index);
            let table = DATA + 0x200;
            c.bus.write32(GAME_GP + 0xd8c, table);
            c.bus.write32(table + index * 4 + 0x1030, 0x203);
            c.bus.write32(GAME_GP + 0xd08, DATA + 0x400);
            c.bus.write32(DATA + 0x400 + 0x0c, DATA + 0x600);
            c.bus.write32(DATA + 0x608, 0x100);
            c.bus.write16(DATA + 0x710, 0);
            c.bus.write16(DATA + 0x712, 10);
            c.bus.write32(DATA + 0x610, DATA + 0x800);
            for i in 0..12u32 {
                c.bus.write16(DATA + 0x800 + i * 4, r.next_u32() as u16);
            }
            c.g[A2][0] = (seed % 12) as u64;
            c.bus.write32(GAME_GP - 0x322c, r.edge_f32());
        });
    }

    #[test]
    fn anim_controller_ctor_matches() {
        check(ee::s10::f_003118d0, anim_controller_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x18d0, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn anim_controller_dtor_matches() {
        check(ee::s10::f_00311958, anim_controller_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1958, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(DATA + 0x50, 0);
        });
    }

    #[test]
    fn algorithmic_script_camera_algo_ctor_matches() {
        check(ee::s02::f_0016ad38, algorithmic_script_camera_algo_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xad38, 0);
            c.g[A0][0] = DATA as u64;
            crate::difftest::fill(c, &mut r, DATA, 0x180 / 4);
        });
    }

    #[test]
    fn algorithm_list_push_matches() {
        check(ee::s01::f_0015c988, algorithm_list_push, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc988, 0);
            alloc_setup(c, &mut r);
            let list = DATA;
            let obj = DATA + 0x100;
            c.g[A0][0] = list as u64;
            c.g[A1][0] = obj as u64;
            c.g[A2][0] = sx(r.next_u32());
            c.bus.write32(list, if seed % 3 == 0 { 0 } else { DATA + 0x300 });
            c.bus.write32(list + 4, r.next_u32());
            let vt = vtable(c, &mut r, &[0x20]);
            c.bus.write32(obj + 0x10, vt);
            c.f[12] = if seed % 2 == 0 { 0x3f80_0000 } else { r.edge_f32() };
        });
    }

    #[test]
    fn anim_controller_clear_layer_matches() {
        check(ee::s10::f_00311e38, anim_controller_clear_layer, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1e38, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 4) as u64;
            // A layer's sequence list: head at +4 (empty keeps the free path
            // short and safe).
            c.bus.write32(DATA + 0x50, DATA + 0x100);
            c.bus.write32(DATA + 0x100 + 4, 0);
        });
    }

    #[test]
    fn anim_controller_get_layer_progress_matches() {
        check(ee::s10::f_00312ab0, anim_controller_get_layer_progress, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2ab0, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 0;
            c.bus.write32(DATA + 0x50, DATA + 0x100);
            c.bus.write32(DATA + 0x100 + 4, DATA + 0x300);
            c.bus.write32(DATA + 0x300 + 8, r.edge_f32());
            c.bus.write32(DATA + 0x300 + 0x10, r.edge_f32());
        });
    }

    #[test]
    fn anim_data_base_ctor_matches() {
        check(ee::s10::f_00311048, anim_data_base_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1048, 0);
            c.g[A0][0] = DATA as u64;
            crate::difftest::fill(c, &mut r, DATA, 0x3fc / 4);
        });
    }

    #[test]
    fn anim_data_base_dtor_matches() {
        check(ee::s10::f_00311088, anim_data_base_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1088, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    // -------------------------------- ADF data source / animation combo

    #[test]
    fn adf_data_source_dtor_matches() {
        check(ee::s10::f_003112c8, adf_data_source_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x12c8, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(DATA, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
        });
    }

    #[test]
    fn anim_combo_dtor_matches() {
        check(ee::s11::f_00358e50, anim_combo_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x8e50, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(DATA + 0x84, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
        });
    }

    #[test]
    fn anim_combo_v42_matches() {
        check(ee::s11::f_00358f90, anim_combo_v42, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x8f90, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            crate::difftest::fill(c, &mut r, DATA + 0x48, 0x40);
            crate::difftest::fill(c, &mut r, 0x4ff120, 8);
        });
    }

    #[test]
    fn anim_combo_snapshot_matrices_matches() {
        check(ee::s11::f_00359030, anim_combo_snapshot_matrices, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x9030, 0);
            let this = DATA;
            c.g[A0][0] = this as u64;
            c.bus.write32(this + 0x74, if seed % 5 == 0 { 0 } else { 1 + (seed % 3) });
            c.bus.write32(this + 0x60, DATA + 0x100);
            c.bus.write32(DATA + 0x100 + 0x80, DATA + 0x300);
            let n = seed % 4;
            c.bus.write32(DATA + 0x300 + 4, n);
            c.bus.write32(DATA + 0x300 + 8, DATA + 0x400);
            for i in 0..n {
                c.bus.write32(DATA + 0x400 + i * 0x10 + 8, if (seed >> i) & 1 != 0 { 1 } else { 0 });
                c.bus.write32(DATA + 0x400 + i * 0x10 + 0xc, r.next_u32());
            }
            c.bus.write32(this + 0x7c, DATA + 0x1000);
            c.bus.write32(this + 0x84, DATA + 0x2000);
            crate::difftest::fill(c, &mut r, DATA + 0x1000, 0x400 / 4);
            crate::difftest::fill(c, &mut r, DATA + 0x2000, 0x400 / 4);
        });
    }

    // ------------------------------------- air predictor / algorithm lists

    #[test]
    fn air_predictor_state_match() {
        for (gen, hand, slot, salt) in [
            (ee::s00::f_00113aa0 as fn(&mut Cpu), air_predictor_save_state as fn(&mut Cpu), 8u32, 0x3aa0u64),
            (ee::s00::f_00113ad8, air_predictor_load_state, 0x10, 0x3ad8),
        ] {
            covered(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, salt, 1);
                let obj = ptr(&mut r);
                let vt = vtable(c, &mut r, &[slot]);
                c.bus.write32(obj, vt);
                c.g[A0][0] = ptr(&mut r) as u64;
                c.g[A1][0] = obj as u64;
            });
        }
    }

    #[test]
    fn algorithmic_script_camera_algo_reset_matches() {
        covered(ee::s02::f_0016b150, algorithmic_script_camera_algo_reset, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xb150, 1);
            let this = ptr(&mut r);
            let vt = vtable(c, &mut r, &[0x18]);
            c.bus.write32(this + 0x10, vt);
            c.g[A0][0] = this as u64;
        });
    }

    #[test]
    fn algorithm_list_dtor_matches() {
        check(ee::s01::f_0015c940, algorithm_list_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc940, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(DATA, 0);
        });
    }

    #[test]
    fn algorithm_list_clear_matches() {
        check(ee::s01::f_0015cb08, algorithm_list_clear, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xcb08, 0);
            alloc_setup(c, &mut r);
            let list = DATA;
            c.g[A0][0] = list as u64;
            if seed % 2 == 0 {
                c.bus.write32(list, 0);
                c.bus.write32(list + 4, 0);
            } else {
                let node = DATA + 0x100;
                c.bus.write32(list, node);
                c.bus.write32(list + 4, 1);
                c.bus.write32(node + 0x14, 0);
                if seed % 4 == 0 {
                    c.bus.write32(node, 0);
                } else {
                    let payload = DATA + 0x200;
                    c.bus.write32(node, payload);
                    let vt = vtable(c, &mut r, &[8]);
                    c.bus.write32(payload + 0x10, vt);
                }
                c.bus.write32(node + 0x18, if seed % 6 == 0 { 0 } else { 1 });
            }
        });
    }

    // --------------------------------------------------- assorted leaves

    #[test]
    fn ai_fwd_diff_cache_hash_remove_matches() {
        check(ee::s10::f_00327b30, ai_fwd_diff_cache_hash_remove, SEEDS, &|c, seed| {
            let mut r = Rng(0x327b30 ^ seed as u64);
            scramble(c, &mut r);
            let this = DATA;
            let node = DATA + 0x100;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = node as u64;
            let count = (seed % 4) + 1;
            c.bus.write32(this + 0xc, count);
            let buckets = DATA + 0x400;
            c.bus.write32(this + 8, buckets);
            let key = if seed % 6 == 0 { 0xffff_ffff } else { r.next_u32() };
            c.bus.write32(node, key);
            // A short chain in the selected bucket; one node matches `key`
            // when seed % 3 != 0 so the removal path runs.
            let bucket = buckets + (key % count) * 4;
            let n = (seed % 3) + 1;
            let mut cur = bucket;
            for i in 0..n {
                let e = DATA + 0x800 + i * 0x10;
                c.bus.write32(cur, e);
                let k = if seed % 3 != 0 && i == n - 1 { key } else { r.next_u32() };
                c.bus.write32(e, k);
                cur = e + 0xc;
            }
            c.bus.write32(cur, 0);
        });
    }

    #[test]
    fn algorithmic_script_camera_algo_dtor_matches() {
        check(ee::s02::f_001763f8, algorithmic_script_camera_algo_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x63f8, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn anim_data_base_reset_chan_cache_table_matches() {
        check(ee::s10::f_003110d0, anim_data_base_reset_chan_cache_table, SEEDS, &|c, seed| {
            let mut r = Rng(0x3110d0 ^ seed as u64);
            scramble(c, &mut r);
            c.bus.write32(0x4fe7a0, r.next_u32());
            c.bus.write32(0x4fe860, r.next_u32());
        });
    }

    #[test]
    fn av_skip_whitespace_matches() {
        check(ee::s13::f_003af300, av_skip_whitespace, SEEDS, &|c, seed| {
            let mut r = Rng(0x3af300 ^ seed as u64);
            scramble(c, &mut r);
            for ch in [0x20u32, 0x09, 0x0a, 0x0d, 0x0b, 0x0c] {
                c.bus.write8(CTYPE + ch, 8);
            }
            let s = DATA + 0x1000;
            let ws = seed % 6;
            for i in 0..8u32 {
                let ch = if i < ws {
                    [0x20u8, 0x09, 0x0a, 0x0d][((seed as usize + i as usize) % 4)]
                } else {
                    b'a' + (i as u8 % 26)
                };
                c.bus.write8(s + i, ch);
            }
            let count = if seed % 5 == 0 { r.next_u32() % 8 } else { seed % 8 };
            c.bus.write32(DATA, s);
            c.bus.write32(DATA + 4, count);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 4) as u64;
        });
    }

    #[test]
    fn av_str_icmp_matches() {
        check(ee::s13::f_003af378, av_str_icmp, SEEDS, &|c, seed| {
            let mut r = Rng(0x3af378 ^ seed as u64);
            scramble(c, &mut r);
            for ch in b'A'..=b'Z' {
                c.bus.write8(CTYPE + ch as u32, 1);
            }
            let a = DATA + 0x1000;
            let b = DATA + 0x2000;
            let n = (seed % 5) as usize;
            for i in 0..n {
                let ch = [b'a', b'A', b'b', b'B', b'1', b'0'][(r.next_u32() as usize) % 6];
                c.bus.write8(a + i as u32, ch);
            }
            c.bus.write8(a + n as u32, 0);
            for i in 0..n {
                // Often mirror `a` with a case flip so the compare hits 0.
                let ch = if seed % 3 == 0 {
                    let x = c.bus.read8(a + i as u32);
                    if x.is_ascii_lowercase() { x - 32 } else { x }
                } else {
                    [b'a', b'A', b'b', b'B', b'1', b'0'][(r.next_u32() as usize) % 6]
                };
                c.bus.write8(b + i as u32, ch);
            }
            c.bus.write8(b + n as u32, 0);
            c.g[A0][0] = a as u64;
            c.g[A1][0] = b as u64;
        });
    }

    #[test]
    fn ai_fwd_diff_cache_hash_insert_matches() {
        check(ee::s10::f_00327bb0, ai_fwd_diff_cache_hash_insert, SEEDS, &|c, seed| {
            let mut r = Rng(0x0032_7bb0 ^ seed as u64);
            scramble(c, &mut r);
            let this = DATA;
            let buckets = DATA + 0x200;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            let count = (seed % 4) + 1;
            c.bus.write32(this + 0xc, count);
            c.bus.write32(this + 8, buckets);
            c.bus.write32(DATA + 0x100, r.next_u32());
            // Build a short descending chain in the selected bucket.
            let bucket = buckets + (c.bus.read32(DATA + 0x100) % count) * 4;
            let n = seed % 3;
            let mut cur = bucket;
            for _ in 0..n {
                let node = DATA + 0x400 + (r.next_u32() & 0xf0);
                c.bus.write32(cur, node);
                c.bus.write32(node, r.next_u32());
                cur = node + 0xc;
            }
            c.bus.write32(cur, 0);
        });
    }
}
