//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for the small generated functions of wave 8 — leaf getters
//! and setters, tiny field tests, and one-call forwarding wrappers. 49 of the
//! 50 assigned functions are here (see the note on `WorldState_Restore` at the
//! end).
//!
//! Most of these are the 7- or 8-cycle forwarding wrappers gcc emits for a
//! function whose body is a single call: a `0x10`-byte frame holding `ra`, a
//! few argument fix-ups, `jal`, then the epilogue; the callee's `v0` is the
//! result. [`forward`] spells that shape out and keeps the cycle counts of
//! the original blocks. The few wrappers with a conditional call or a
//! post-call step are written out in full.
//!
//! All calls go through the function table (`call_ptr`), exactly as the
//! generated bodies' `call_indirect` does.
//!
//! `WorldState_Restore` (0x0026dbd0) is *not* ported: it forwards to
//! `WorldState_Read` (0x0026dbf0), which pulls loop bounds out of a
//! serialised replay stream through its vtable; on test data those reads come
//! back as fake results and the loops run away, so it cannot be covered by a
//! cheap differential test.

use crate::abi;
use crate::call::{call_ptr, guest, Frame};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_93e0, rider_metrix_on_race_finished, "RiderMetrix_OnRaceFinished"),
    (0x0014_ebb8, be_set_net_string_534fc8, "BE_SetNetString534fc8"),
    (0x0014_ebd8, be_set_net_string_534fb8, "BE_SetNetString534fb8"),
    (0x0015_0158, bebagtint_get_trick_field8, "BEBAGTInt_GetTrickField8"),
    (0x0015_0178, bebagtint_get_trick_fielda, "BEBAGTInt_GetTrickFieldA"),
    (0x0015_dac0, game_camera_set_collision_flag4a4, "GameCamera_SetCollisionFlag4a4"),
    (0x0018_6978, cfe_state_event_select_handle_input, "cFEStateEventSelect_HandleInput"),
    (0x0019_2948, cfe_state_bragging_rights_is_row_enabled, "cFEStateBraggingRights_IsRowEnabled"),
    (0x001a_9890, netfe_state_set_selected_user, "NetFEState_SetSelectedUser"),
    (0x001c_4b78, cfe_state_stats_reset_results_page, "cFEStateStats_ResetResultsPage"),
    (0x0020_8638, ov_screen_map_handle_input, "OVScreenMap_HandleInput"),
    (0x0022_7f80, mc_manager_get, "MCManager_Get"),
    (0x0023_7cd8, game_rand_next, "GameRand_Next"),
    (0x0023_c8d0, mc_manager_has_status_error, "MCManager_HasStatusError"),
    (0x0024_1aa0, mc_manager_get_product_code, "MCManager_GetProductCode"),
    (0x0024_49f8, stats_process_update, "StatsProcess_Update"),
    (0x0024_4a18, stats_process_render, "StatsProcess_Render"),
    (0x0026_4990, c_setup_man_free_packet, "cSetupMan_FreePacket"),
    (0x0026_a070, path_read_track_field, "Path_ReadTrackField"),
    (0x0026_ba48, path_data_read_vec3, "PathData_ReadVec3"),
    (0x0026_ba68, path_data_read_int, "PathData_ReadInt"),
    (0x0026_ba88, path_data_read_float, "PathData_ReadFloat"),
    (0x0027_4c38, in_seq_get_header_extra, "InSeq_GetHeaderExtra"),
    (0x0027_9ce0, script_engine_v12, "ScriptEngine_v12"),
    (0x0027_c020, script_dummy_obj_deactivate, "ScriptDummyObj_Deactivate"),
    (0x0027_c050, script_dummy_obj_apply, "ScriptDummyObj_Apply"),
    (0x0027_d2c8, script_engine_binding_has_process, "ScriptEngine_BindingHasProcess"),
    (0x0028_b210, ssx_audio_system_get_view, "SSXAudioSystem_GetView"),
    (0x0029_4880, ssx_audio_system_set_rider_air_flag, "SSXAudioSystem_SetRiderAirFlag"),
    (0x002a_df60, audio_rand, "Audio_Rand"),
    (0x002a_a428, stream_buffer_get_state, "StreamBuffer_GetState"),
    (0x002b_2120, song_instance_path_cmd_a, "SongInstance_PathCmdA"),
    (0x002b_46e8, music_player_are_loops_active, "MusicPlayer_AreLoopsActive"),
    (0x002b_49c0, music_player_set_song_tag, "MusicPlayer_SetSongTag"),
    (0x002b_5818, world_trigger_get_info, "WorldTrigger_GetInfo"),
    (0x002b_5878, world_trigger_get_name, "WorldTrigger_GetName"),
    (0x002b_6b30, c_world_sound_on_section_unloaded, "cWorldSound_OnSectionUnloaded"),
    (0x002c_66b8, c_ps2_mem_card_v39, "cPS2MemCard_v39"),
    (0x002c_a3e8, menu_sys_item_get_input_code, "MenuSysItem_GetInputCode"),
    (0x002e_e2f8, irradiance_get_override, "Irradiance_GetOverride"),
    (0x0030_b520, wscript_man_set_pending_rider_config, "WScriptMan_SetPendingRiderConfig"),
    (0x0037_c8a0, ps2_graphics_man_set_tex_alloc_flags, "PS2GraphicsMan_SetTexAllocFlags"),
    (0x0038_6cf0, ps2_graphics_man_v118, "PS2GraphicsMan_v118"),
    (0x0039_8018, ui_engine_render_state_mgr, "UIEngine_RenderStateMgr"),
    (0x0039_e758, ui_state_add_screen, "UIState_AddScreen"),
    (0x0039_fae8, ui_list_first_wrapper, "UIList_First_Wrapper"),
    (0x003a_4868, ui_list_node_dtor_body, "UIListNode_dtorBody"),
    (0x003a_67e0, world_on_game_enter, "World_OnGameEnter"),
];

// ---------------------------------------------------------------------------
// Callees, under the names the generated bodies give them.

mod callee {
    /// `EventSelect_HandleInput` — stub, returns 0.
    pub const EVENT_SELECT_HANDLE_INPUT: u32 = 0x0020_2768;
    /// `MCManager_GetInstance`.
    pub const MC_MANAGER_GET_INSTANCE: u32 = 0x0023_c808;
    /// `StatsProcess_UpdateStub`.
    pub const STATS_PROCESS_UPDATE_STUB: u32 = 0x002d_21b0;
    /// `StatsProcess_RenderStub`.
    pub const STATS_PROCESS_RENDER_STUB: u32 = 0x002d_21b8;
    /// `ScriptEngine_PushFreeNode`.
    pub const SCRIPT_ENGINE_PUSH_FREE_NODE: u32 = 0x0027_bd90;
    /// `ScriptObj_Deactivate`.
    pub const SCRIPT_OBJ_DEACTIVATE: u32 = 0x0028_3440;
    /// `ScriptDummyObj_ReadFields`.
    pub const SCRIPT_DUMMY_OBJ_READ_FIELDS: u32 = 0x0027_cbb0;
    /// `PathData_ReadFloat`.
    pub const PATH_DATA_READ_FLOAT: u32 = 0x0026_ba88;
    /// `EA_MemCopy`.
    pub const EA_MEM_COPY: u32 = 0x003e_6574;
    /// `Rand_NextMain`.
    pub const RAND_NEXT_MAIN: u32 = 0x0031_77f0;
    /// `Rand_Next`.
    pub const RAND_NEXT: u32 = 0x0031_7a08;
    /// `TexMan_SetAllocFlags`.
    pub const TEX_MAN_SET_ALLOC_FLAGS: u32 = 0x0036_6fe0;
    /// `GraphicsMan_EmptyHook`.
    pub const GRAPHICS_MAN_EMPTY_HOOK: u32 = 0x0036_9ff0;
    /// `UIStateMgr_RenderStub`.
    pub const UI_STATE_MGR_RENDER_STUB: u32 = 0x0039_eca8;
    /// `UIList_PushBack`.
    pub const UI_LIST_PUSH_BACK: u32 = 0x0039_7910;
    /// `UIList_First`.
    pub const UI_LIST_FIRST: u32 = 0x0039_7840;
    /// `UIListNode_Unlink`.
    pub const UI_LIST_NODE_UNLINK: u32 = 0x0039_7740;
    /// `World_ResetSections`.
    pub const WORLD_RESET_SECTIONS: u32 = 0x003a_6800;
    /// `strcpy`.
    pub const STRLEN_AND_STRCPY: u32 = 0x0041_66f4;
    /// `cBXString_assign_cstr`.
    pub const CBXSTRING_ASSIGN_CSTR: u32 = 0x0031_84e8;
    /// `LobbyMan_RequestStatsPage`.
    pub const LOBBY_MAN_REQUEST_STATS_PAGE: u32 = 0x0025_e348;
    /// `SSXApp_BootLoad`.
    pub const SSX_APP_BOOT_LOAD: u32 = 0x0022_82d0;
    /// `UStr_FromAscii_2`.
    pub const USTR_FROM_ASCII: u32 = 0x002c_2540;
    /// `PATH_SetTrackControlValue`.
    pub const PATH_SET_TRACK_CONTROL_VALUE: u32 = 0x003d_0d70;
    /// `WorldTriggerManager_GetTriggerInfo`.
    pub const WORLD_TRIGGER_GET_INFO: u32 = 0x002b_63d0;
    /// `WorldTriggerManager_GetTriggerName`.
    pub const WORLD_TRIGGER_GET_NAME: u32 = 0x002b_65b0;
    /// `StreamBuffer_GetStatus`.
    pub const STREAM_BUFFER_GET_STATUS: u32 = 0x002a_a020;
    /// `MenuSys_GetInputCode`.
    pub const MENU_SYS_GET_INPUT_CODE: u32 = 0x002c_c248;
    /// `IrradianceRef_GetEntry`.
    pub const IRRADIANCE_REF_GET_ENTRY: u32 = 0x002e_e010;
    /// `cWorldSound_UnlinkEmitters`.
    pub const C_WORLD_SOUND_UNLINK_EMITTERS: u32 = 0x002b_6c20;
    /// `RiderMetrix_BankCombo`.
    pub const RIDER_METRIX_BANK_COMBO: u32 = 0x0011_7718;
    /// `__builtin_delete`.
    pub const BUILTIN_DELETE: u32 = 0x0031_7e50;
}

// ---------------------------------------------------------------------------
// Helpers.

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// The shared shape of a one-call forwarding wrapper: charge `cycles`, open a
/// `size`-byte frame with `ra` saved at `size - 0x10`, run `adjust` (the
/// argument fix-ups the original does after the frame is open), call `target`
/// and return to `return_to`, then the epilogue. The callee's `v0` is the
/// result, as in the original.
fn forward(
    c: &mut Cpu,
    cycles: u64,
    size: u32,
    adjust: impl FnOnce(&mut Cpu),
    target: u32,
    return_to: u32,
) {
    guest(c, |c| {
        c.cycles += cycles;
        let frame = Frame::open(c, size);
        frame.save_ra(c, size - 0x10);
        adjust(c);
        call_ptr(c, target, return_to)?;
        frame.restore_ra(c, size - 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BAGT trick records: 20-byte records at `D_0045AEB8`.

/// `D_0045AEB8`: the BAGT trick table.
const BAGT_TRICKS: u32 = 0x0045_aeb8;

/// `unsigned short BEBAGTInt_GetTrickField8(int idx)` (0x00150158): the u16
/// field at `+0x8` of trick record `idx`.
pub fn bebagtint_get_trick_field8(c: &mut Cpu) {
    bagt_trick_field(c, 0x8);
}

/// `unsigned short BEBAGTInt_GetTrickFieldA(int idx)` (0x00150178): the u16
/// field at `+0xa` of trick record `idx`.
pub fn bebagtint_get_trick_fielda(c: &mut Cpu) {
    bagt_trick_field(c, 0xa);
}

/// `p = &D_0045AEB8 + idx * 0x14; return *(u16 *)(p + off)`.
fn bagt_trick_field(c: &mut Cpu, off: u32) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    // a2 = idx * 20 (the multiply leaves LO/HI set, as the original does).
    c.mult(0, A2, A2, V1);
    // `v0 = 0x460000 - 0x5148`: the low half of `&D_0045AEB8`.
    c.g[V0][0] = sx(BAGT_TRICKS);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(off));
    c.pc = ra;
}

/// `void GameCamera_SetCollisionFlag4a4(void)` (0x0015dac0): set the word at
/// `+0x4a4` of the camera control object (world → `+0x84` camera manager →
/// `+0x84` chase manager → `+0x4` controller) to 1.
pub fn game_camera_set_collision_flag4a4(c: &mut Cpu) {
    c.cycles += 7;
    let world = Addr(gp(c, -0x848)).lw(c);
    c.g[A0][0] = world;
    c.g[A1][0] = 1;
    let cameras = Addr((world as u32).wrapping_add(0x84)).lw(c);
    c.g[V0][0] = cameras;
    let chase = Addr((cameras as u32).wrapping_add(0x84)).lw(c);
    c.g[V1][0] = chase;
    let control = Addr((chase as u32).wrapping_add(4)).lw(c);
    c.g[A0][0] = control;
    c.sw((control as u32).wrapping_add(0x4a4), 1);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Small leaves.

/// `bool cFEStateBraggingRights_IsRowEnabled(this, int idx)` (0x00192948):
/// bit `idx` of the top byte of the word at `+0x2c`.
pub fn cfe_state_bragging_rights_is_row_enabled(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x0100_0000;
    let word = Addr((c.g[A0][0] as u32).wrapping_add(0x2c)).lw(c);
    c.g[V0][0] = word;
    let shift = (c.g[A1][0] as u32) & 31;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << shift);
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
    let masked = word & c.g[V1][0];
    c.g[V0][0] = masked;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = (0u64 < masked) as u64;
    c.pc = ra;
}

/// `int MCManager_HasStatusError(this)` (0x0023c8d0): whether the status word
/// at `+0x43c` has any bit outside `0x4 | 0x1e0`.
pub fn mc_manager_has_status_error(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x43c)).lw(c);
    c.g[V1][0] = 0xffff_ffff_ffff_fffb;
    c.g[A0][0] = 0xffff_ffff_ffff_fe1f;
    let v = c.g[V0][0] & c.g[V1][0];
    let v = v & c.g[A0][0];
    c.g[V0][0] = v;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = (0u64 < v) as u64;
    c.pc = ra;
}

/// `int ScriptEngine_BindingHasProcess(this, int i, int process)`
/// (0x0027d2c8): whether binding `i`'s record at `+0x58` names `process`.
pub fn script_engine_binding_has_process(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x14;
    c.mult(0, V1, A1, V0); // v1 = i * 20
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x58)).lw(c);
    let binding = c.g[V0][0] ^ c.g[A2][0];
    c.g[V0][0] = binding;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    c.pc = ra;
}

/// `void *SSXAudioSystem_GetView(int i)` (0x0028b210): rider view `i` from the
/// rider table (world → `+0x84` → `+0x84`, entries of 4 bytes at `+0x4`).
pub fn ssx_audio_system_get_view(c: &mut Cpu) {
    c.cycles += 7;
    let world = Addr(gp(c, -0x848)).lw(c);
    c.g[A1][0] = world;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    let cameras = Addr((world as u32).wrapping_add(0x84)).lw(c);
    c.g[V0][0] = cameras;
    let riders = Addr((cameras as u32).wrapping_add(0x84)).lw(c);
    c.g[V1][0] = sx((riders as u32).wrapping_add(c.g[A0][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
    c.pc = ra;
}

/// `void SSXAudioSystem_SetRiderAirFlag(this, int i, int on)` (0x00294880):
/// store `on` in the per-rider air flag at `+0x59e8 + i*4`, for `i < 6`.
pub fn ssx_audio_system_set_rider_air_flag(c: &mut Cpu) {
    c.cycles += 3;
    let in_range = (c.g[A1][0] as i64) < 6;
    c.g[V0][0] = in_range as u64;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
    if in_range {
        c.cycles += 2;
        let at = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = at;
        c.sw((at as u32).wrapping_add(0x59e8), c.g[A2][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int MusicPlayer_AreLoopsActive(this)` (0x002b46e8): the flag at `+0x4c` of
/// the active stream, or 0 when there is none.
pub fn music_player_are_loops_active(c: &mut Cpu) {
    c.cycles += 3;
    let stream = Addr((c.g[A0][0] as u32).wrapping_add(0x408)).lw(c);
    c.g[V0][0] = stream;
    c.cycles += 2;
    c.g[V0][0] = if stream == 0 {
        0
    } else {
        Addr((stream as u32).wrapping_add(0x4c)).lw(c)
    };
    abi::ret(c);
}

/// `void *InSeq_GetHeaderExtra(SeqHeader *h)` (0x00274c38): the extra header
/// at `+0xc` for format versions >= 13, else null.
pub fn in_seq_get_header_extra(c: &mut Cpu) {
    c.cycles += 4;
    let version = Addr((c.g[A0][0] as u32).wrapping_add(2)).lhu(c);
    let old = (version < 0xd) as u64;
    c.g[V0][0] = old;
    c.cycles += 2;
    if old != 0 {
        c.g[V0][0] = 0;
    } else {
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
    }
    abi::ret(c);
}

/// `void MusicPlayer_SetSongTag(this, int tag)` (0x002b49c0): store `tag` at
/// `+0x58` of the current song, when the player has one.
pub fn music_player_set_song_tag(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x418)).lw(c);
    let has_song = c.g[V0][0] != 0;
    if has_song {
        c.cycles += 3;
        let song = Addr((c.g[A0][0] as u32).wrapping_add(0x408)).lw(c);
        c.g[A0][0] = song;
        if song != 0 {
            c.sw((song as u32).wrapping_add(0x58), c.g[A1][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void WScriptMan_SetPendingRiderConfig(this, cWScriptPair *src)`
/// (0x0030b520): set the pending flag at `+0x34` and copy the pair into
/// `+0x38` (an unaligned 8-byte move).
pub fn wscript_man_set_pending_rider_config(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 1;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x34), c.g[V0][0]);
    c.ldl(2, (c.g[A1][0] as u32).wrapping_add(7));
    c.ldr(2, (c.g[A1][0] as u32));
    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x3f));
    c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x38));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Forwards into another virtual method of `this`.

/// `void cFEStateEventSelect_HandleInput(this)` (0x00186978):
/// `EventSelect_HandleInput(this + 0x48)`.
pub fn cfe_state_event_select_handle_input(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48)),
        callee::EVENT_SELECT_HANDLE_INPUT,
        0x0018_6988,
    )
}

/// `void OVScreenMap_HandleInput(this)` (0x00208638):
/// `EventSelect_HandleInput(this + 0x9c)`.
pub fn ov_screen_map_handle_input(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9c)),
        callee::EVENT_SELECT_HANDLE_INPUT,
        0x0020_8648,
    )
}

/// `void NetFEState_SetSelectedUser(this, const char *name)` (0x001a9890):
/// assign the name to the `cBXString` at `+0x6c0`.
pub fn netfe_state_set_selected_user(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0)),
        callee::CBXSTRING_ASSIGN_CSTR,
        0x001a_98a0,
    )
}

/// `void *MCManager_Get(this)` (0x00227f80): the `cMCOverlayManager`
/// singleton (`MCManager_GetInstance`).
pub fn mc_manager_get(c: &mut Cpu) {
    forward(c, 7, 0x10, |_| {}, callee::MC_MANAGER_GET_INSTANCE, 0x0022_7f90)
}

/// `void StatsProcess_Update(this)` (0x002449f8): forward to the stub on the
/// owned processor at `+0x10`.
pub fn stats_process_update(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c),
        callee::STATS_PROCESS_UPDATE_STUB,
        0x0024_4a08,
    )
}

/// `void StatsProcess_Render(this)` (0x00244a18): forward to the stub on the
/// owned processor at `+0x10`.
pub fn stats_process_render(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c),
        callee::STATS_PROCESS_RENDER_STUB,
        0x0024_4a28,
    )
}

/// `void cSetupMan_FreePacket(this, void *packet)` (0x00264990):
/// `operator delete(packet)`.
pub fn c_setup_man_free_packet(c: &mut Cpu) {
    forward(c, 7, 0x10, |c| c.g[A0][0] = c.g[A1][0], callee::BUILTIN_DELETE, 0x0026_49a0)
}

/// `void ScriptEngine_v12(this, unused, Node *node)` (0x00279ce0):
/// `ScriptEngine_PushFreeNode(this, node)`.
pub fn script_engine_v12(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A1][0] = c.g[A2][0],
        callee::SCRIPT_ENGINE_PUSH_FREE_NODE,
        0x0027_9cf0,
    )
}

/// `void ScriptDummyObj_Deactivate(this)` (0x0027c020):
/// `ScriptObj_Deactivate(&this->obj)` (`+0xc`).
pub fn script_dummy_obj_deactivate(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc)),
        callee::SCRIPT_OBJ_DEACTIVATE,
        0x0027_c030,
    )
}

/// `unsigned int Audio_Rand(Rng *unused)` (0x002adf60): the global sound RNG's
/// next value (`Rand_NextMain`).
pub fn audio_rand(c: &mut Cpu) {
    forward(c, 7, 0x10, |_| {}, callee::RAND_NEXT_MAIN, 0x002a_df70)
}

/// `void PS2GraphicsMan_SetTexAllocFlags(this, int flags)` (0x0037c8a0):
/// forward to the texture manager at `+0x18f4`.
pub fn ps2_graphics_man_set_tex_alloc_flags(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x18f4)).lw(c),
        callee::TEX_MAN_SET_ALLOC_FLAGS,
        0x0037_c8b0,
    )
}

/// `void PS2GraphicsMan_v118(this)` (0x00386cf0): `GraphicsMan_EmptyHook`.
pub fn ps2_graphics_man_v118(c: &mut Cpu) {
    forward(c, 7, 0x10, |_| {}, callee::GRAPHICS_MAN_EMPTY_HOOK, 0x0038_6d00)
}

/// `void UIEngine_RenderStateMgr(this)` (0x00398018): render the state
/// manager at `+0x18`.
pub fn ui_engine_render_state_mgr(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18)),
        callee::UI_STATE_MGR_RENDER_STUB,
        0x0039_8028,
    )
}

/// `void UIState_AddScreen(this, cListNode *node)` (0x0039e758): push the
/// node onto the screen list at `+0x24`.
pub fn ui_state_add_screen(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24)),
        callee::UI_LIST_PUSH_BACK,
        0x0039_e768,
    )
}

/// `void *UIList_First_Wrapper(cList *list)` (0x0039fae8): the list's first
/// node.
pub fn ui_list_first_wrapper(c: &mut Cpu) {
    forward(c, 7, 0x10, |_| {}, callee::UI_LIST_FIRST, 0x0039_faf8)
}

/// `void UIListNode_dtorBody(cListNode *node)` (0x003a4868): unlink the node.
pub fn ui_list_node_dtor_body(c: &mut Cpu) {
    forward(c, 7, 0x10, |_| {}, callee::UI_LIST_NODE_UNLINK, 0x003a_4878)
}

/// `void World_OnGameEnter(this)` (0x003a67e0): reset the world's section
/// state.
pub fn world_on_game_enter(c: &mut Cpu) {
    forward(c, 7, 0x10, |_| {}, callee::WORLD_RESET_SECTIONS, 0x003a_67f0)
}

// ---------------------------------------------------------------------------
// Forwards with extra arguments or a result the wrapper re-reads.

/// `float RiderMetrix_OnRaceFinished(this)` (0x001193e0): bank the combo, then
/// return 0.0 (the float is materialised with `mtc1`).
pub fn rider_metrix_on_race_finished(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, callee::RIDER_METRIX_BANK_COMBO, 0x0011_93f0)?;
        c.mtc1(0, 0);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_SetNetString534fc8(const char *src)` (0x0014ebb8):
/// `strcpy(D_00534FC8, src)`.
pub fn be_set_net_string_534fc8(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = sx(0x53_0000u32.wrapping_add(0x4fc8)),
        callee::STRLEN_AND_STRCPY,
        0x0014_ebcc,
    )
}

/// `void BE_SetNetString534fb8(const char *src)` (0x0014ebd8):
/// `strcpy(D_00534FB8, src)`.
pub fn be_set_net_string_534fb8(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = sx(0x53_0000u32.wrapping_add(0x4fb8)),
        callee::STRLEN_AND_STRCPY,
        0x0014_ebec,
    )
}

/// `void cFEStateStats_ResetResultsPage(this, int slot)` (0x001c4b78): clear
/// the page field at `+0x6e8`, then request the stats page from the lobby
/// manager (`gp - 0xc8`).
pub fn cfe_state_stats_reset_results_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x6e8), 0);
        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        call_ptr(c, callee::LOBBY_MAN_REQUEST_STATS_PAGE, 0x001c_4b8c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InitLoadScreen_Enter(void)` (0x00232568): run the boot load for the
/// world (`gp - 0x848`).
pub fn init_load_screen_enter(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = Addr(gp(c, -0x848)).lw(c),
        callee::SSX_APP_BOOT_LOAD,
        0x0023_257c,
    )
}

/// `unsigned int GameRand_Next(void)` (0x00237cd8): next value of the global
/// game RNG at `D_004C9548`.
pub fn game_rand_next(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = sx(0x4d_0000u32.wrapping_sub(0x6ab8)),
        callee::RAND_NEXT,
        0x0023_7cec,
    )
}

/// `void MCManager_GetProductCode(this)` (0x00241aa0): decode the product
/// code string at `D_0047C3A8` into `this`.
pub fn mc_manager_get_product_code(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A1][0] = sx(0x48_0000u32.wrapping_sub(0x3c58)),
        callee::USTR_FROM_ASCII,
        0x0024_1ab4,
    )
}

/// `void SongInstance_PathCmdA(void)` (0x002b2120): set PATH control value
/// `0x11000001`.
pub fn song_instance_path_cmd_a(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = sx(0x1100_0000u32 | 1),
        callee::PATH_SET_TRACK_CONTROL_VALUE,
        0x002b_2134,
    )
}

/// `int WorldTrigger_GetInfo(int idx, int *type, int *a, int *b, int c, int d)`
/// (0x002b5818): the singleton trigger manager's info query.
pub fn world_trigger_get_info(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = Addr(gp(c, 0x21e4)).lw(c),
        callee::WORLD_TRIGGER_GET_INFO,
        0x002b_582c,
    )
}

/// `int WorldTrigger_GetName(int idx, char *dst)` (0x002b5878): the singleton
/// trigger manager's name query.
pub fn world_trigger_get_name(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = Addr(gp(c, 0x21e4)).lw(c),
        callee::WORLD_TRIGGER_GET_NAME,
        0x002b_588c,
    )
}

/// `void ScriptDummyObj_Apply(this, void *args)` (0x0027c050): read the
/// scripted fields (`args`) into a stack buffer via `ScriptDummyObj_ReadFields`.
pub fn script_dummy_obj_apply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx(frame.at(c, 0));
        call_ptr(c, callee::SCRIPT_DUMMY_OBJ_READ_FIELDS, 0x0027_c064)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int StreamBuffer_GetState(StreamBuffer *b)` (0x002aa428): fill a stack
/// status block and return its first word.
pub fn stream_buffer_get_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx(frame.at(c, 0));
        call_ptr(c, callee::STREAM_BUFFER_GET_STATUS, 0x002a_a438)?;
        let block = frame.at(c, 0);
        c.g[V0][0] = Addr(block).lw(c);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int MenuSysItem_GetInputCode(MenuSysItem **this)` (0x002ca3e8): the input
/// code of the menu system at `(*this) + 0x124`.
pub fn menu_sys_item_get_input_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x124)).lw(c);
        call_ptr(c, callee::MENU_SYS_GET_INPUT_CODE, 0x002c_a3fc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Irradiance_GetOverride(void)` (0x002ee2f8): look up the global lens
/// FX irradiance entry at `gp + 0x24c0`.
pub fn irradiance_get_override(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = sx(gp(c, 0x24c0)),
        callee::IRRADIANCE_REF_GET_ENTRY,
        0x002e_e30c,
    )
}

// ---------------------------------------------------------------------------
// Copies of a field, conditional on the arguments.

/// `void Path_ReadTrackField(Path *self, void *src, int already_read)`
/// (0x0026a070): read a float into `self + 0x38` unless a value was supplied.
pub fn path_read_track_field(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let supplied = c.g[A2][0] != 0;
        frame.save_ra(c, 0);
        if !supplied {
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
            call_ptr(c, callee::PATH_DATA_READ_FLOAT, 0x0026_a084)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PathData_ReadVec3(Vec3 *dst, const Vec3 *src)` (0x0026ba48): copy
/// three floats unless the two are the same object.
pub fn path_data_read_vec3(c: &mut Cpu) {
    path_data_copy(c, 0xc, 0x0026_ba5c)
}

/// `void PathData_ReadInt(int *dst, const int *src)` (0x0026ba68): copy the
/// word unless the two are the same object.
pub fn path_data_read_int(c: &mut Cpu) {
    path_data_copy(c, 4, 0x0026_ba7c)
}

/// `void PathData_ReadFloat(float *dst, const float *src)` (0x0026ba88): copy
/// the float unless the two are the same object.
pub fn path_data_read_float(c: &mut Cpu) {
    path_data_copy(c, 4, 0x0026_ba9c)
}

/// Shared body of the three [`PathData_Read*`] copies.
fn path_data_copy(c: &mut Cpu, size: u32, return_to: u32) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let same = c.g[A0][0] == c.g[A1][0];
        frame.save_ra(c, 0);
        if !same {
            c.cycles += 2;
            c.g[A2][0] = size as u64;
            call_ptr(c, callee::EA_MEM_COPY, return_to)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWorldSound_OnSectionUnloaded(this, void *section)` (0x002b6b30):
/// drop the section's emitters.
pub fn c_world_sound_on_section_unloaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let no_section = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if !no_section {
            c.cycles += 2;
            call_ptr(c, callee::C_WORLD_SOUND_UNLINK_EMITTERS, 0x002b_6b44)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_v39(this, const char *name, int flag)` (0x002c66b8):
/// cache `name` at `+0x14c0` when `flag == 0`.
pub fn c_ps2_mem_card_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let named = c.g[A2][0] != 0;
        frame.save_ra(c, 0);
        if !named {
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14c0));
            call_ptr(c, callee::STRLEN_AND_STRCPY, 0x002c_66cc)?;
        }
        c.cycles += 3;
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
    use crate::difftest::fakes::{machine, ptr};
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    /// NUL-terminated `s` at `addr`.
    fn put_str(c: &mut Cpu, addr: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// `world` at `gp - 0x848`, camera manager at `world + 0x84`, rider table
    /// at `camera + 0x84`.
    fn world_chain(c: &mut Cpu, world: u32, cameras: u32, riders: u32) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, cameras);
        c.bus.write32(cameras + 0x84, riders);
    }

    /// The lazily built BE singleton (`gp - 0x1f04`) pointed at a zeroed dummy:
    /// with it zero some callees run to the file system looking for disc data.
    fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_0000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
    }

    /// The allocator mutex (`0x538b00`) marked as held by the fake
    /// `GetThreadId` result, so allocations take their fast path.
    fn allocator(c: &mut Cpu) {
        c.bus.write32(0x0053_8b00 + 4, 0x42);
        c.bus.write32(crate::difftest::fakes::LOG_RESULTS, 0x42);
    }

    // -----------------------------------------------------------------------
    // BAGT trick records.

    fn trick_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1501_58 ^ seed as u64);
        scramble(c, &mut r);
        let idx = (r.next_u32() % 40) as i32 - 4;
        c.g[A2][0] = sx(idx as u32);
        let base = 0x0045_aeb8u32.wrapping_add((idx as u32).wrapping_mul(0x14));
        for k in 0..10u32 {
            c.bus.write16(base.wrapping_add(2 * k), r.next_u32() as u16);
        }
    }

    #[test]
    fn bebagtint_get_trick_field8_matches() {
        check_lohi(ee::s01::f_00150158__generated, bebagtint_get_trick_field8, 200, &trick_setup);
    }

    #[test]
    fn bebagtint_get_trick_fielda_matches() {
        check_lohi(ee::s01::f_00150178__generated, bebagtint_get_trick_fielda, 200, &trick_setup);
    }

    // -----------------------------------------------------------------------

    #[test]
    fn game_camera_set_collision_flag4a4_matches() {
        check(ee::s01::f_0015dac0__generated, game_camera_set_collision_flag4a4, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x15_dac0, 0);
            c.g[A0][0] = r.next_u32() as u64;
            world_chain(c, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
            c.bus.write32(DATA + 0x3000 + 4, DATA + 0x4000);
            c.bus.write32(DATA + 0x4000 + 0x4a4, r.next_u32());
        });
    }

    #[test]
    fn cfe_state_event_select_handle_input_matches() {
        check(ee::s02::f_00186978__generated, cfe_state_event_select_handle_input, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x18_6978, 0);
            c.g[A0][0] = ptr(&mut r) as u64;
        });
    }

    #[test]
    fn cfe_state_bragging_rights_is_row_enabled_matches() {
        check(ee::s02::f_00192948__generated, cfe_state_bragging_rights_is_row_enabled, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x19_2948, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 48) as u64;
            c.bus.write32(DATA + 0x2c, r.next_u32());
        });
    }

    #[test]
    fn netfe_state_set_selected_user_matches() {
        check(ee::s03::f_001a9890__generated, netfe_state_set_selected_user, 200, &|c, seed| {
            machine(c, seed, 0x1a_9890, 0);
            let src = DATA + 0x800;
            match seed % 3 {
                0 => put_str(c, src, ""),
                1 => put_str(c, src, "abc"),
                _ => put_str(c, src, "selected user"),
            }
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = src as u64;
        });
    }

    #[test]
    fn ov_screen_map_handle_input_matches() {
        check(ee::s05::f_00208638__generated, ov_screen_map_handle_input, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x20_8638, 0);
            c.g[A0][0] = ptr(&mut r) as u64;
        });
    }

    #[test]
    fn mc_manager_get_matches() {
        check(ee::s05::f_00227f80__generated, mc_manager_get, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x22_7f80, 0);
            // The overlay manager already exists: take the cheap path.
            c.bus.write32(GAME_GP.wrapping_sub(0x47c), DATA + 0x500);
            c.g[A0][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn mc_manager_has_status_error_matches() {
        check(ee::s06::f_0023c8d0__generated, mc_manager_has_status_error, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x23_c8d0, 0);
            c.g[A0][0] = DATA as u64;
            let v = if seed % 3 == 0 { 0 } else { r.next_u32() };
            c.bus.write32(DATA + 0x43c, v);
        });
    }

    /// The stats stubs ignore their argument, so only the load matters.
    fn stats_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x2449_f8, 0);
        c.g[A0][0] = DATA as u64;
        c.bus.write32(DATA + 0x10, ptr(&mut r));
    }

    #[test]
    fn stats_process_update_matches() {
        check(ee::s06::f_002449f8__generated, stats_process_update, 200, &stats_setup);
    }

    #[test]
    fn stats_process_render_matches() {
        check(ee::s06::f_00244a18__generated, stats_process_render, 200, &stats_setup);
    }

    #[test]
    fn c_setup_man_free_packet_matches() {
        check(ee::s07::f_00264990__generated, c_setup_man_free_packet, 100, &|c, seed| {
            machine(c, seed, 0x26_4990, 0);
            // `delete(NULL)` is the fast path through the allocator.
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 0;
        });
    }

    

    fn rand_setup(c: &mut Cpu, seed: u32, state: u32) {
        let mut r = machine(c, seed, 0x237c_d8, 0);
        for i in 0..8u32 {
            let v = if seed % 4 == 0 { 0 } else { r.next_u32() };
            c.bus.write32(state + 4 * i, v);
        }
    }

    #[test]
    fn game_rand_next_matches() {
        check(ee::s06::f_00237cd8__generated, game_rand_next, 200, &|c, seed| {
            rand_setup(c, seed, 0x004c_9548);
        });
    }

    #[test]
    fn mc_manager_get_product_code_matches() {
        check(ee::s06::f_00241aa0__generated, mc_manager_get_product_code, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x24_1aa0, 0);
            c.g[A0][0] = DATA as u64;
            let code = DATA + 0x800;
            match seed % 3 {
                0 => put_str(c, code, ""),
                1 => put_str(c, code, "SLUS-20772"),
                _ => {
                    c.bus.write32(code, r.next_u32());
                    c.bus.write8(code + 4, 0);
                }
            }
            // `D_0047C3A8`: the product-code string pointer.
            c.bus.write32(0x0047_c3a8, code);
        });
    }

    #[test]
    fn path_read_track_field_matches() {
        check(ee::s07::f_0026a070__generated, path_read_track_field, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x26_a070, 0);
            c.g[A0][0] = DATA as u64;
            // Half the seeds use the very same object, skipping the copy.
            c.g[A1][0] = if seed % 2 == 0 { (DATA + 0x38) as u64 } else { (DATA + 0x800) as u64 };
            c.g[A2][0] = if seed % 3 == 0 { 1 } else { 0 };
            c.bus.write32(DATA + 0x800, r.next_u32());
        });
    }

    fn path_copy_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = if seed % 2 == 0 { DATA as u64 } else { (DATA + 0x800) as u64 };
        c.g[A2][0] = r.next_u32() as u64;
        for i in 0..4u32 {
            c.bus.write32(DATA + 0x800 + 4 * i, r.next_u32());
        }
    }

    #[test]
    fn path_data_read_vec3_matches() {
        check(ee::s07::f_0026ba48__generated, path_data_read_vec3, 200, &|c, seed| path_copy_setup(c, seed, 0x26_ba48));
    }

    #[test]
    fn path_data_read_int_matches() {
        check(ee::s07::f_0026ba68__generated, path_data_read_int, 200, &|c, seed| path_copy_setup(c, seed, 0x26_ba68));
    }

    #[test]
    fn path_data_read_float_matches() {
        check(ee::s07::f_0026ba88__generated, path_data_read_float, 200, &|c, seed| path_copy_setup(c, seed, 0x26_ba88));
    }

    #[test]
    fn in_seq_get_header_extra_matches() {
        check(ee::s07::f_00274c38__generated, in_seq_get_header_extra, 200, &|c, seed| {
            machine(c, seed, 0x27_4c38, 0);
            c.g[A0][0] = DATA as u64;
            let version = (seed % 20) as u16;
            c.bus.write16(DATA + 2, version);
        });
    }

    #[test]
    fn script_engine_v12_matches() {
        check(ee::s07::f_00279ce0__generated, script_engine_v12, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x27_9ce0, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = (DATA + 0x900) as u64;
        });
    }

    #[test]
    fn script_dummy_obj_deactivate_matches() {
        check(ee::s07::f_0027c020__generated, script_dummy_obj_deactivate, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x27_c020, 0);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x14, r.next_u32());
        });
    }

    #[test]
    fn script_dummy_obj_apply_matches() {
        check(ee::s07::f_0027c050__generated, script_dummy_obj_apply, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x27_c050, 0);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
        });
    }

    #[test]
    fn script_engine_binding_has_process_matches() {
        check_lohi(ee::s07::f_0027d2c8__generated, script_engine_binding_has_process, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x27_d2c8, 0);
            let i = r.next_u32() % 16;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = i as u64;
            let at = DATA + 0x58 + i * 0x14;
            let v = r.next_u32();
            c.bus.write32(at, v);
            c.g[A2][0] = if seed % 2 == 0 {
                v as u64
            } else {
                (v ^ (1 + r.next_u32() % 8)) as u64
            };
        });
    }

    #[test]
    fn ssx_audio_system_get_view_matches() {
        check(ee::s07::f_0028b210__generated, ssx_audio_system_get_view, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x28_b210, 0);
            world_chain(c, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
            c.g[A0][0] = (r.next_u32() % 8) as u64;
            for i in 0..8u32 {
                c.bus.write32(DATA + 0x3000 + 4 + 4 * i, ptr(&mut r));
            }
        });
    }

    #[test]
    fn ssx_audio_system_set_rider_air_flag_matches() {
        check(ee::s07::f_00294880__generated, ssx_audio_system_set_rider_air_flag, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x29_4880, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 12) as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn audio_rand_matches() {
        check(ee::s08::f_002adf60__generated, audio_rand, 200, &|c, seed| {
            rand_setup(c, seed, 0x004f_f018);
        });
    }

    #[test]
    fn stream_buffer_get_state_matches() {
        check(ee::s08::f_002aa428__generated, stream_buffer_get_state, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x2a_a428, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            for i in 0..8u32 {
                c.bus.write32(DATA + 0x1000 + 4 * i, r.next_u32() % 4);
            }
        });
    }

    #[test]
    fn song_instance_path_cmd_a_matches() {
        check(ee::s08::f_002b2120__generated, song_instance_path_cmd_a, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2b_2120, 0);
            c.g[A0][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn music_player_are_loops_active_matches() {
        check(ee::s08::f_002b46e8__generated, music_player_are_loops_active, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2b_46e8, 0);
            c.g[A0][0] = DATA as u64;
            if seed % 2 == 0 {
                c.bus.write32(DATA + 0x408, 0);
            } else {
                c.bus.write32(DATA + 0x408, DATA + 0x500);
                c.bus.write32(DATA + 0x500 + 0x4c, r.next_u32());
            }
        });
    }

    #[test]
    fn music_player_set_song_tag_matches() {
        check(ee::s08::f_002b49c0__generated, music_player_set_song_tag, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2b_49c0, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64;
            match seed % 3 {
                0 => c.bus.write32(DATA + 0x418, 0),
                1 => {
                    c.bus.write32(DATA + 0x418, 1);
                    c.bus.write32(DATA + 0x408, 0);
                }
                _ => {
                    c.bus.write32(DATA + 0x418, 1);
                    c.bus.write32(DATA + 0x408, DATA + 0x500);
                }
            }
        });
    }

    /// A zeroed singleton trigger manager: every query is out of range.
    fn trigger_setup(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_add(0x21e4), DATA + 0x1000);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = 0;
        c.g[A2][0] = (DATA + 0x800) as u64;
        c.g[A3][0] = (DATA + 0x900) as u64;
    }

    #[test]
    fn world_trigger_get_info_matches() {
        check(ee::s08::f_002b5818__generated, world_trigger_get_info, 100, &|c, seed| {
            trigger_setup(c, seed, 0x2b_5818)
        });
    }

    #[test]
    fn world_trigger_get_name_matches() {
        check(ee::s08::f_002b5878__generated, world_trigger_get_name, 100, &|c, seed| {
            trigger_setup(c, seed, 0x2b_5878)
        });
    }

    #[test]
    fn c_world_sound_on_section_unloaded_matches() {
        check(ee::s08::f_002b6b30__generated, c_world_sound_on_section_unloaded, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2b_6b30, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { (DATA + 0x800) as u64 };
            c.bus.write32(DATA + 0x800 + 0xc, r.next_u32() % 2);
        });
    }

    #[test]
    fn c_ps2_mem_card_v39_matches() {
        check(ee::s08::f_002c66b8__generated, c_ps2_mem_card_v39, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2c_66b8, 0);
            c.g[A0][0] = DATA as u64;
            let name = DATA + 0x800;
            put_str(c, name, if seed % 2 == 0 { "slot1" } else { "" });
            c.g[A1][0] = name as u64;
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { r.next_u32() as u64 };
        });
    }

    #[test]
    fn menu_sys_item_get_input_code_matches() {
        check(ee::s09::f_002ca3e8__generated, menu_sys_item_get_input_code, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2c_a3e8, 0);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.bus.write32(DATA + 0x100 + 0x124, r.next_u32());
        });
    }

    #[test]
    fn irradiance_get_override_matches() {
        check(ee::s09::f_002ee2f8__generated, irradiance_get_override, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2e_e2f8, 0);
            let entry = GAME_GP.wrapping_add(0x24c0);
            match seed % 3 {
                0 => {}
                1 => c.bus.write32(entry + 4, 0x0012_3400),
                _ => {
                    c.bus.write32(entry + 4, 0x0012_3400);
                    c.bus.write8(entry, 1);
                    c.bus.write32(entry, 0x0100_0001);
                }
            }
            c.bus.write32(entry + 8, r.next_u32() % 0x100);
        });
    }

    #[test]
    fn wscript_man_set_pending_rider_config_matches() {
        check(ee::s10::f_0030b520__generated, wscript_man_set_pending_rider_config, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x30_b520, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x801) as u64; // deliberately unaligned
            for i in 0..3u32 {
                c.bus.write32(DATA + 0x800 + 4 * i, r.next_u32());
            }
            c.bus.write32(DATA + 0x34, r.next_u32());
        });
    }

    #[test]
    fn ps2_graphics_man_set_tex_alloc_flags_matches() {
        check(ee::s12::f_0037c8a0__generated, ps2_graphics_man_set_tex_alloc_flags, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x37_c8a0, 0);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x18f4, DATA + 0x600);
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn ps2_graphics_man_v118_matches() {
        check(ee::s12::f_00386cf0__generated, ps2_graphics_man_v118, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x38_6cf0, 0);
            c.g[A0][0] = ptr(&mut r) as u64;
        });
    }

    #[test]
    fn ui_engine_render_state_mgr_matches() {
        check(ee::s13::f_00398018__generated, ui_engine_render_state_mgr, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x39_8018, 0);
            c.g[A0][0] = ptr(&mut r) as u64;
        });
    }

    #[test]
    fn ui_state_add_screen_matches() {
        check(ee::s13::f_0039e758__generated, ui_state_add_screen, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x39_e758, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            match seed % 3 {
                0 => c.bus.write32(DATA + 0x1000 + 0x24 + 0xc, 0),
                1 => c.bus.write32(DATA + 0x1000 + 0x24 + 0xc, DATA + 0x900),
                _ => {
                    c.bus.write32(DATA + 0x1000 + 0x24 + 0xc, DATA + 0x900);
                    c.bus.write32(DATA + 0x900 + 4, DATA + 0xa00);
                }
            }
            c.bus.write32(DATA + 0x800 + 4, r.next_u32());
        });
    }

    #[test]
    fn ui_list_first_wrapper_matches() {
        check(ee::s13::f_0039fae8__generated, ui_list_first_wrapper, 200, &|c, seed| {
            machine(c, seed, 0x39_fae8, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            match seed % 3 {
                0 => c.bus.write32(DATA + 0x1000 + 4, 0),
                1 => {
                    let node = DATA + 0x800;
                    c.bus.write32(DATA + 0x1000 + 4, node);
                    c.bus.write32(node + 4, node);
                }
                _ => {
                    let node = DATA + 0x800;
                    let last = DATA + 0x900;
                    c.bus.write32(DATA + 0x1000 + 4, node);
                    c.bus.write32(node + 4, last);
                }
            }
        });
    }

    #[test]
    fn ui_list_node_dtor_body_matches() {
        check(ee::s13::f_003a4868__generated, ui_list_node_dtor_body, 200, &|c, seed| {
            machine(c, seed, 0x3a_4868, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            match seed % 3 {
                // Head of the list, no neighbours.
                0 => {}
                // Linked between two other nodes.
                1 => {
                    c.bus.write32(DATA + 0x1000, DATA + 0x800);
                    c.bus.write32(DATA + 0x1000 + 4, DATA + 0x900);
                }
                // Only a successor.
                _ => {
                    c.bus.write32(DATA + 0x1000, 0);
                    c.bus.write32(DATA + 0x1000 + 4, DATA + 0x1000);
                }
            }
        });
    }

    #[test]
    fn world_on_game_enter_matches() {
        check(ee::s13::f_003a67e0__generated, world_on_game_enter, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x3a_67e0, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            match seed % 3 {
                0 => {}
                1 => {
                    c.bus.write32(DATA + 0x1000, DATA + 0x2000);
                    c.bus.write32(DATA + 0x1000 + 0x10 + 4, 0);
                }
                _ => {
                    c.bus.write32(DATA + 0x1000, DATA + 0x2000);
                    c.bus.write32(DATA + 0x1000 + 0x10 + 4, DATA + 0x3000);
                    c.bus.write32(DATA + 0x3000 + 8, r.next_u32() % 8);
                }
            }
        });
    }

    #[test]
    fn rider_metrix_on_race_finished_matches() {
        check(ee::s00::f_001193e0__generated, rider_metrix_on_race_finished, 200, &|c, seed| {
            machine(c, seed, 0x11_93e0, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            // Both exits of `RiderMetrix_BankCombo` are early: no combo banked
            // (trick count < 2) or no points.
            if seed % 2 == 0 {
                c.bus.write32(DATA + 0x1000 + 0x9c, 0);
            } else {
                c.bus.write32(DATA + 0x1000 + 0x9c, 5);
                c.bus.write32(DATA + 0x1000 + 0xa0, 0);
            }
        });
    }

    fn net_string_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x14_ebb8, 0);
        let src = DATA + 0x800;
        match seed % 3 {
            0 => put_str(c, src, ""),
            1 => put_str(c, src, "ssx3"),
            _ => put_str(c, src, "a longer network string"),
        }
        c.g[A1][0] = src as u64;
    }

    #[test]
    fn be_set_net_string_534fc8_matches() {
        check(ee::s01::f_0014ebb8__generated, be_set_net_string_534fc8, 200, &net_string_setup);
    }

    #[test]
    fn be_set_net_string_534fb8_matches() {
        check(ee::s01::f_0014ebd8__generated, be_set_net_string_534fb8, 200, &net_string_setup);
    }

    #[test]
    fn cfe_state_stats_reset_results_page_matches() {
        check(ee::s03::f_001c4b78__generated, cfe_state_stats_reset_results_page, 50, &|c, seed| {
            let mut r = machine(c, seed, 0x1c_4b78, 0);
            be_library(c);
            allocator(c);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = (r.next_u32() % 4) as u64;
            c.bus.write32(DATA + 0x1000 + 0x6e8, r.next_u32());
        });
    }

}
