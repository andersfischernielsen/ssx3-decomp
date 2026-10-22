//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for 50 small cold-path functions, each with a differential
//! test against its generated body.
//!
//! The functions fall into a few families:
//!
//! - script/music engine pokes (`ScriptEngine_OnSeqAdvance`,
//!   `MusicPlayer_EnableSong`), the replay cache (`Replay_SeekToEnd`,
//!   `Replay_CheckInstantReplay`, `AI_GetSyncChecksum`) and the dirty-sock tag
//!   parser (`TagFieldGetAddress`);
//! - PS2 memcard state-machine stubs (`cPS2MemCard_v37/v36/RequestReadSave`
//!   from the quadtree/painter module) which format a name, stash two words
//!   and hand the state number to `cPS2MemCard_SetState`;
//! - UI construction/teardown: `cUIEngine_addScreenByHashName` callers, list
//!   destructors, text-scroll and list helpers (`UIList_GetAt`,
//!   `cUIGroup_SetFocus`, `cUITextScroll_SetTextId`, `GameModule_dtor`, ...);
//! - rider metrics (`RiderMetrix_Reset`, `RiderMetrix_OnRiderReset`,
//!   `BENewPlayerInt_Get*`), object constructors/destructors
//!   (`MagnetModifier_ctor`, `BoostModifier_ctor_2`, `Debounce_ctor`,
//!   `MultiParticleMan_dtor`, `cStreamMan_dtor`) and the small FE state
//!   handlers.
//!
//! Every port keeps the generated body's cycle charges at the same points, its
//! exact register writes (including v0/v1 scratch values a caller may read) and
//! its call order. Calls use the same protocol as the original: a `Callee` for
//! the intra-shard `jal` the generator wrote as a direct call, `call_ptr` for
//! the cross-shard/virtual calls it wrote as `call_indirect`. Loop back edges
//! carry the same event checks (`deliver_event_here` for structured loops,
//! `back_edge_unwinds` for the two dispatcher-shaped functions).

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_a720, script_engine_on_seq_advance, "ScriptEngine_OnSeqAdvance"),
    (0x002b_3fe8, music_player_enable_song, "MusicPlayer_EnableSong"),
    (0x002c_e3a0, menu_sys_float_range_item_draw, "MenuSysFloatRangeItem_Draw"),
    (0x0027_0378, replay_seek_to_end, "Replay_SeekToEnd"),
    (0x0027_05b0, replay_check_instant_replay, "Replay_CheckInstantReplay"),
    (0x0012_b420, ai_get_sync_checksum, "AI_GetSyncChecksum"),
    (0x002c_45d0, cps2_mem_card_v37, "cPS2MemCard_v37"),
    (0x002c_4648, cps2_mem_card_v36, "cPS2MemCard_v36"),
    (0x002c_4788, cps2_mem_card_request_read_save, "cPS2MemCard_RequestReadSave"),
    (0x003a_4888, ui_list_dtor, "UIList_dtor"),
    (0x0037_5918, boader_shadow_dtor, "BoaderShadow_dtor"),
    (0x0030_b928, wscript_man_define_collect_set, "WScriptMan_DefineCollectSet"),
    (0x0030_d840, rider_model_alloc_model_refs, "RiderModel_AllocModelRefs"),(0x0035_72e0, magnet_modifier_ctor, "MagnetModifier_ctor"),
    (0x0035_7a00, multi_particle_man_dtor, "MultiParticleMan_dtor"),
    (0x001d_0eb8, c_fe_state_reward_gallery_update_count_text, "cFEStateRewardGallery_UpdateCountText"),
    (0x0012_7848, player_get_uber_grind_button, "Player_GetUberGrindButton"),
    (0x0030_8328, wscript_task_update, "WScriptTask_Update"),
    (0x0018_d3c0, c_fe_state_options_save_load_on_render, "cFEStateOptionsSaveLoad_OnRender"),
    (0x001f_aa78, ov_screen_be_options_on_widget_notify, "OVScreenBEOptions_OnWidgetNotify"),
    (0x0015_9170, be_reward_int_grant_medal_reward, "BERewardInt_GrantMedalReward"),
    (0x0011_7540, rider_metrix_reset, "RiderMetrix_Reset"),
    (0x0011_9368, rider_metrix_on_rider_reset, "RiderMetrix_OnRiderReset"),
    (0x0014_5f90, be_new_player_int_get_char_state_flag, "BENewPlayerInt_GetCharStateFlag"),
    (0x0014_6d98, be_new_player_int_get_char_record_field_27c, "BENewPlayerInt_GetCharRecordField27C"),
    (0x0014_7908, be_new_player_int_get_inbox, "BENewPlayerInt_GetInbox"),
    (0x0034_2808, boost_modifier_ctor_2, "BoostModifier_ctor_2"),
    (0x0023_80e8, game_mode_man_destroy, "GameModeMan_Destroy"),
    (0x0023_8d30, gm_handler_clear_standings, "GMHandler_ClearStandings"),
    (0x0023_8da8, gm_handler_clear_standings2, "GMHandler_ClearStandings2"),
    (0x0031_81e8, cbx_string_alloc_copy, "cBXString_AllocCopy"),
    (0x0031_8260, cbx_string_dtor, "cBXString_dtor"),
    (0x0031_82d8, cbx_string_ctor_cstr, "cBXString_ctor_cstr"),
    (0x0034_2d10, debounce_ctor, "Debounce_ctor"),
    (0x001d_4c60, c_fe_state_rewards_room_init, "cFEStateRewardsRoom_Init"),
    (0x003e_cff8, tag_field_get_address, "TagFieldGetAddress"),(0x003a_1f18, cui_text_scroll_set_text_id, "cUITextScroll_SetTextId"),
    (0x0039_8638, cui_group_set_focus, "cUIGroup_SetFocus"),
    (0x0035_3e08, flex_bridge_springs_init, "FlexBridge_SpringsInit"),
    (0x0020_8f10, ov_screen_session_load_map_image, "OVScreenSession_LoadMapImage"),(0x003a_6e20, c_stream_man_dtor, "cStreamMan_dtor"),
    (0x0039_7870, ui_list_get_at, "UIList_GetAt"),
    (0x0024_3838, game_module_dtor, "GameModule_dtor"),
    (0x0022_e7c8, game_view_man_reset_cameras, "GameViewMan_ResetCameras"),
    (0x0019_5540, c_fe_state_bonus_material_on_widget_created, "cFEStateBonusMaterial_OnWidgetCreated"),
    (0x0019_4588, c_fe_state_transition_init, "cFEStateTransition_Init"),
    (0x001a_36e8, fe_async_manager_request_texture, "FEAsyncManager_RequestTexture"),
];

// ---------------------------------------------------------------------------
// Helpers.

/// The low 32 bits of a register (an object pointer or a 32-bit value).
#[inline]
fn low(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// The current stack pointer.
#[inline]
fn sp(c: &Cpu) -> u32 {
    c.g[SP][0] as u32
}

/// `gp + off` for a `gp`-relative global (the game's `gp` is in `c.g[GP]`).
#[inline]
fn gpo(c: &Cpu, off: i32) -> u32 {
    if off >= 0 {
        low(c, GP).wrapping_add(off as u32)
    } else {
        low(c, GP).wrapping_sub((-off) as u32)
    }
}

/// `lw off(reg)`.
#[inline]
fn lw_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lw(c)
}

/// `lh off(reg)` (sign-extended).
#[inline]
fn lh_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lh(c)
}

/// `lb off(reg)` (sign-extended).
#[inline]
fn lb_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lb(c)
}

/// `lb off(reg)` (zero-extended).
#[inline]
fn lbu_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lbu(c)
}

/// `sw off(reg)`.
#[inline]
fn sw_r(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = low(c, reg).wrapping_add(off);
    c.sw(a, v);
}

/// `lwc1 f, off(reg)`.
#[inline]
fn lwc1_r(c: &mut Cpu, f: usize, reg: usize, off: u32) {
    let a = low(c, reg).wrapping_add(off);
    c.lwc1(f, a);
}

/// `swc1 f, off(reg)`.
#[inline]
fn swc1_r(c: &mut Cpu, f: usize, reg: usize, off: u32) {
    let a = low(c, reg).wrapping_add(off);
    c.swc1(f, a);
}

// ---------------------------------------------------------------------------
// Intra-shard callees the generator called by name (`jal`), plus the
// cross-shard ones it wrote as `call_indirect` (called with `call_ptr`).

mod callee {
    use super::{ee, Callee};

    // s00
    pub const RIDER_METRIX_RESET_STATS: Callee = Callee { addr: 0x0011_75b8, f: ee::s00::f_001175b8 };
    pub const RIDER_METRIX_CLEAR_HUD_MESSAGES: Callee =
        Callee { addr: 0x0011_73b8, f: ee::s00::f_001173b8 };
    pub const RIDER_METRIX_RESET_TRICK: Callee =
        Callee { addr: 0x0011_7838, f: ee::s00::f_00117838 };
    pub const RIDER_METRIX_RESET_COMBO: Callee =
        Callee { addr: 0x0011_75f8, f: ee::s00::f_001175f8 };
    pub const RIDER_METRIX_BANK_PENDING_SCORE: Callee =
        Callee { addr: 0x0011_a7a8, f: ee::s00::f_0011a7a8 };
    pub const RIDER_GET_SYNC_CHECKSUM: Callee =
        Callee { addr: 0x0012_5ad8, f: ee::s00::f_00125ad8 };

    // s01
    pub const CBE_GET_BE: Callee = Callee { addr: 0x0014_dc80, f: ee::s01::f_0014dc80 };
    pub const BE_GET_RIDER_PROFILE: Callee = Callee { addr: 0x0014_a0e0, f: ee::s01::f_0014a0e0 };
    pub const BE_NEW_PLAYER_INT_GET_SLOT: Callee =
        Callee { addr: 0x0014_7798, f: ee::s01::f_00147798 };
    pub const BE_REWARD_INT_GET_MEDAL_REWARD_ID: Callee =
        Callee { addr: 0x0015_9080, f: ee::s01::f_00159080 };
    pub const BE_REWARD_INT_IS_REWARD_GRANTED: Callee =
        Callee { addr: 0x0015_a2e0, f: ee::s01::f_0015a2e0 };
    pub const BE_REWARD_INT_GRANT_REWARD: Callee =
        Callee { addr: 0x0015_9cd0, f: ee::s01::f_00159cd0 };

    // s02
    pub const CAMERA_TRIGGER_MAN_UNLOAD: Callee =
        Callee { addr: 0x0016_c9b0, f: ee::s02::f_0016c9b0 };
    pub const CAMERA_TRIGGER_MAN_PARSE: Callee =
        Callee { addr: 0x0016_c228, f: ee::s02::f_0016c228 };
    pub const CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2: Callee =
        Callee { addr: 0x0016_d1b8, f: ee::s02::f_0016d1b8 };
    pub const UI_OBJECT_SHOW_CLAMP_ALPHA_HALF: Callee =
        Callee { addr: 0x0019_4498, f: ee::s02::f_00194498 };

    // s03
    pub const FE_MODULE_UNLOAD_RIDER_PREVIEWS: Callee =
        Callee { addr: 0x001a_04c8, f: ee::s03::f_001a04c8 };
    pub const FE_MODULE_RIDER_PREVIEW_DTOR: Callee =
        Callee { addr: 0x0019_e498, f: ee::s03::f_0019e498 };
    pub const FE_ASYNC_MANAGER_SET_FILE_STATUS: Callee =
        Callee { addr: 0x001a_39d8, f: ee::s03::f_001a39d8 };

    // s07
    pub const REPLAY_FIND_SNAPSHOT: Callee = Callee { addr: 0x0026_e448, f: ee::s07::f_0026e448 };
    pub const RUN_LIST_SEEK: Callee = Callee { addr: 0x0026_d4d8, f: ee::s07::f_0026d4d8 };
    pub const SCRIPT_LIST_ON_SEQ_ADVANCE: Callee =
        Callee { addr: 0x0027_7310, f: ee::s07::f_00277310 };
    pub const SCRIPT_ENGINE_GET_SEQ_PROCESS_ID: Callee =
        Callee { addr: 0x0028_2be0, f: ee::s07::f_00282be0 };
    pub const REPLAY_REQUEST_INSTANT_REPLAY: Callee =
        Callee { addr: 0x0027_0658, f: ee::s07::f_00270658 };

    // s08
    pub const USTR_TO_ASCII: Callee = Callee { addr: 0x002c_2580, f: ee::s08::f_002c2580 };
    pub const CPS2_MEM_CARD_SET_STATE: Callee =
        Callee { addr: 0x002c_48c0, f: ee::s08::f_002c48c0 };

    // s09
    pub const MENU_SYS_ITEM_DRAW_ROW: Callee =
        Callee { addr: 0x002c_a988, f: ee::s09::f_002ca988 };

    // s10
    pub const WSCRIPT_MAN_RESET_COLLECT_SET: Callee =
        Callee { addr: 0x0030_c390, f: ee::s10::f_0030c390 };
    pub const OPERATOR_NEW_NAMED: Callee = Callee { addr: 0x0031_7e30, f: ee::s10::f_00317e30 };
    pub const CBX_STRING_ALLOC_BUFFER: Callee =
        Callee { addr: 0x0031_8040, f: ee::s10::f_00318040 };
    pub const EA_MEM_FREE: Callee = Callee { addr: 0x0031_7e98, f: ee::s10::f_00317e98 };
    pub const BUILTIN_DELETE: Callee = Callee { addr: 0x0031_7e50, f: ee::s10::f_00317e50 };

    // s11
    pub const MODIFIER_BLOCK_HAS_RAIL_MODIFIER: Callee =
        Callee { addr: 0x0035_3418, f: ee::s11::f_00353418 };
    pub const RAIL_MODIFIER_CTOR: Callee = Callee { addr: 0x0035_b708, f: ee::s11::f_0035b708 };
    pub const MOD_OBJ_SET_RAIL_MODIFIER: Callee =
        Callee { addr: 0x0035_56a8, f: ee::s11::f_003556a8 };
    pub const MULTI_PARTICLE_DTOR: Callee = Callee { addr: 0x0035_7fa0, f: ee::s11::f_00357fa0 };
    pub const MOD_OBJ_CTOR_2: Callee = Callee { addr: 0x0035_52d8, f: ee::s11::f_003552d8 };
    pub const FLEX_BRIDGE_ALLOC_SPRING_NODES: Callee =
        Callee { addr: 0x0035_3f00, f: ee::s11::f_00353f00 };

    // s12
    pub const TEX_MAN_UNLOCK_VRAM: Callee = Callee { addr: 0x0036_7b60, f: ee::s12::f_00367b60 };

    // s13
    pub const UI_LIST_NODE_IS_SENTINEL: Callee =
        Callee { addr: 0x0039_7718, f: ee::s13::f_00397718 };
    pub const UI_LIST_FIRST: Callee = Callee { addr: 0x0039_7840, f: ee::s13::f_00397840 };
    pub const UI_LIST_DELETE_ALL: Callee = Callee { addr: 0x0039_7948, f: ee::s13::f_00397948 };
    pub const UI_LIST_ACCEPT_VISITOR: Callee =
        Callee { addr: 0x0039_79f8, f: ee::s13::f_003979f8 };
    pub const UI_LIST_NODE_DTOR_BODY: Callee =
        Callee { addr: 0x003a_4868, f: ee::s13::f_003a4868 };
    pub const CUI_TEXT_SCROLL_SET_WIDE_STRING: Callee =
        Callee { addr: 0x003a_1d30, f: ee::s13::f_003a1d30 };
    pub const CSTREAM_MAN_RESET: Callee = Callee { addr: 0x003a_6ed8, f: ee::s13::f_003a6ed8 };
    pub const CSTREAM_MAN_FREE_BUFFERS: Callee =
        Callee { addr: 0x003a_6f38, f: ee::s13::f_003a6f38 };

    // Cross-shard calls the generator wrote as `call_indirect`.
    pub const STR_HASH: u32 = 0x0031_7670;
    pub const SPRINTF: u32 = 0x0041_7828;
    pub const MEMSET: u32 = 0x0041_6210;
    /// The libc `memset` copy in the world/serialisation shard.
    pub const MEMSET_3E6448: u32 = 0x003e_6448;
    pub const MEMCPY: u32 = 0x0041_605c;
    pub const STRLEN: u32 = 0x0041_6810;
    pub const STRCPY: u32 = 0x0041_66f4;
    pub const CVT_FLOAT_TO_INT: u32 = 0x0041_3af8;
    pub const C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME: u32 = 0x0039_d860;
    pub const C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME: u32 = 0x0039_7cc0;
    pub const C_UI_SCREEN_PLAY_FRAME: u32 = 0x0039_c870;
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;
    pub const FUNC_0028F140: u32 = 0x0028_f140;
    pub const C_MEM_MAN_ALLOC: u32 = 0x0031_7d70;
    pub const AI_RAND: u32 = 0x0031_7810;
}

// ---------------------------------------------------------------------------
// Script / music / replay / AI.

/// `void ScriptEngine_OnSeqAdvance(self, a1, float x, float y)` (0x27a720):
/// unless the script slot at `+0x550` is already running (id 2), advance the
/// sequence list at `+0x54c + id*0xcc` to the process the engine reports for
/// `a1`, passing both floats through in the delay slots.
pub fn script_engine_on_seq_advance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.g[V0][0] = 0x2;
        c.sq(17, sp(c).wrapping_add(0x10));
        c.swc1(21, sp(c).wrapping_add(0x38));
        c.g[S1][0] = c.g[A1][0];
        c.swc1(20, sp(c).wrapping_add(0x30));
        c.mov_s(21, 13);
        c.sq(16, sp(c).wrapping_add(0x20));
        c.sd(sp(c), c.g[RA][0]);
        c.g[V1][0] = lw_r(c, A0, 0x550);
        let running = c.g[V1][0] == c.g[V0][0];
        c.mov_s(20, 12);
        if !running {
            c.cycles += 11;
            c.g[V0][0] = 0xcc;
            c.g[S0][0] = lw_r(c, A0, 0x54c);
            c.mult(0, V0, V1, V0); // lo = id * 0xcc
            c.g[S0][0] = sx(low(c, S0).wrapping_add(low(c, V0)));
            callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID.call(c, 0x0027_a764)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.mov_s(12, 20);
            c.mov_s(13, 21);
            callee::SCRIPT_LIST_ON_SEQ_ADVANCE.call(c, 0x0027_a77c)?;
        }
        c.cycles += 7;
        c.lq(16, sp(c).wrapping_add(0x20));
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        c.lwc1(21, sp(c).wrapping_add(0x38));
        c.lwc1(20, sp(c).wrapping_add(0x30));
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void MusicPlayer_EnableSong(self, idx)` (0x2b3fe8): while fewer than 64
/// songs are enabled, set the `idx`-th bit of the 256-bit mask at `+0x3f8`,
/// bump the count and add one to each of the five per-channel totals whose
/// grid entry for that song is non-zero.
pub fn music_player_enable_song(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] = c.g[A0][0];
    c.g[A0][0] = lw_r(c, A2, 0x3e8);
    c.g[V0][0] = ((c.g[A0][0] as i64) < 64) as u64;
    let full = c.g[V0][0] == 0;
    c.g[A3][0] = c.g[A1][0];
    if !full {
        c.cycles += 13;
        c.g[A1][0] = 0x14;
        c.g[V1][0] = Addr(low(c, A2).wrapping_add(0x3f8)).ld(c);
        c.mult(0, A1, A3, A1); // lo = idx * sizeof(grid row)
        c.g[V0][0] = 0x1;
        c.g[V0][0] = c.g[V0][0] << (c.g[A3][0] & 63);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(1));
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        sw_r(c, A2, 0x3e8, c.g[A0][0]);
        Addr(low(c, A2).wrapping_add(0x3f8)).sd(c, c.g[V1][0]);
        c.g[V0][0] = sx(low(c, A2).wrapping_add(0x420));
        c.g[A1][0] = sx(low(c, A1).wrapping_add(low(c, V0)));
        c.g[A0][0] = sx(low(c, A2).wrapping_add(0x920));
        c.g[V1][0] = 0x4;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(low(c, A1)).lw(c);
            if c.g[V0][0] == 0 {
                c.g[A0][0] = sx(low(c, A0).wrapping_add(4));
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(low(c, A0)).lw(c);
                c.g[V0][0] = sx(low(c, V0).wrapping_add(1));
                sw_r(c, A0, 0, c.g[V0][0]);
                c.g[A0][0] = sx(low(c, A0).wrapping_add(4));
            }
            c.cycles += 3;
            c.g[V1][0] = sx(low(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[A1][0] = sx(low(c, A1).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_4030);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void MenuSysFloatRangeItem_Draw(self, row)` (0x2ce3a0): format the item's
/// float (`*(self+0x18)` scaled by 180/pi) into a 0x70-byte buffer and draw
/// the row.
pub fn menu_sys_float_range_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0xb0));
        c.lwc1(0, gpo(c, -0x3fb4));
        c.sq(16, sp(c).wrapping_add(0xa0));
        c.sq(17, sp(c).wrapping_add(0x90));
        c.g[S0][0] = 0x4a_0000;
        c.sq(18, sp(c).wrapping_add(0x80));
        c.g[S1][0] = c.g[A0][0];
        c.sd(sp(c).wrapping_add(0x70), c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        c.g[S0][0] = sx(low(c, S0).wrapping_add(0x39c8));
        c.g[V0][0] = lw_r(c, S1, 0x18);
        c.lwc1(12, low(c, V0));
        c.mul_s(12, 12, 0);
        call_ptr(c, callee::CVT_FLOAT_TO_INT, 0x002c_e3d8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, callee::SPRINTF, 0x002c_e3e8)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = lw_r(c, A0, 0x14);
        c.g[A3][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002c_e400)?;
        c.lq(16, sp(c).wrapping_add(0xa0));
        c.lq(17, sp(c).wrapping_add(0x90));
        c.lq(18, sp(c).wrapping_add(0x80));
        c.g[RA][0] = Addr(sp(c).wrapping_add(0x70)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0xb0));
        c.pc = t;
        Ok(())
    })
}

/// `void Replay_SeekToEnd(self)` (0x270378): find the oldest snapshot (frame
/// limit 100000000), record it and its position, then seek the two stream
/// run lists to that position.
pub fn replay_seek_to_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.g[A1][0] = 0x5f5_0000;
        c.sq(16, sp(c).wrapping_add(0x30));
        c.g[A1][0] = c.g[A1][0] | 0xe100;
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, sp(c).wrapping_add(0x20));
        c.sq(18, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = sx(low(c, S0).wrapping_add(0x48c));
        sw_r(c, S0, 0x604, 0);
        c.g[S2][0] = 0x1;
        callee::REPLAY_FIND_SNAPSHOT.call(c, 0x0027_03a8)?;
        sw_r(c, S0, 0x488, c.g[V0][0]);
        c.g[V1][0] = lw_r(c, V0, 0x30);
        sw_r(c, S0, 0x484, c.g[V1][0]);
        c.g[V0][0] = Addr(low(c, S1)).lw(c);
        loop {
            c.cycles += 4;
            c.g[S2][0] = sx(low(c, S2).wrapping_sub(1));
            c.g[S1][0] = sx(low(c, S1).wrapping_add(4));
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !none {
                c.cycles += 2;
                c.g[A1][0] = lw_r(c, S0, 0x484);
                callee::RUN_LIST_SEEK.call(c, 0x0027_03d0)?;
            }
            c.cycles += 2;
            if (c.g[S2][0] as i64) >= 0 {
                c.g[V0][0] = Addr(low(c, S1)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_03b8);
                }
                continue;
            }
            c.cycles += 6;
            c.lq(16, sp(c).wrapping_add(0x30));
            c.lq(17, sp(c).wrapping_add(0x20));
            c.lq(18, sp(c).wrapping_add(0x10));
            c.g[RA][0] = Addr(sp(c)).ld(c);
            let t = low(c, RA);
            c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
            c.pc = t;
            break;
        }
        Ok(())
    })
}

/// `void Replay_CheckInstantReplay(self, obj, alt)` (0x2705b0): when a request
/// is pending and the race manager has exactly one rider, fire an instant
/// replay once the recorded span reaches 1000 ms, otherwise clear the request.
pub fn replay_check_instant_replay(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[A3][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = lw_r(c, A3, 0x3d4);
        let idle = c.g[V0][0] == 0;
        c.g[T0][0] = c.g[A1][0];
        if idle {
            c.cycles += 1;
            c.g[RA][0] = Addr(sp(c)).ld(c);
        } else {
            c.cycles += 7;
            c.g[A0][0] = Addr(gpo(c, -0x848)).lw(c);
            c.g[A1][0] = 0x1;
            c.g[V0][0] = lw_r(c, A0, 0x84);
            c.g[V1][0] = lw_r(c, V0, 0xc);
            c.g[A0][0] = lw_r(c, V1, 0x7c);
            let multi = c.g[A0][0] != c.g[A1][0];
            c.g[RA][0] = Addr(sp(c)).ld(c);
            if !multi {
                let now;
                let start;
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    c.g[V0][0] = lw_r(c, T0, 0xa4);
                    c.cycles += 1;
                    c.g[V1][0] = lw_r(c, A3, 0x47c);
                    now = c.g[V0][0];
                    start = c.g[V1][0];
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw_r(c, T0, 0x9c);
                    c.g[V1][0] = lw_r(c, A3, 0x474);
                    now = c.g[V0][0];
                    start = c.g[V1][0];
                }
                c.cycles += 4;
                c.g[V0][0] = sx((now as u32).wrapping_sub(start as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 1000) as u64;
                if c.g[V0][0] != 0 {
                    sw_r(c, A3, 0x3d4, 0);
                    c.cycles += 1;
                    c.g[RA][0] = Addr(sp(c)).ld(c);
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[A3][0];
                    callee::REPLAY_REQUEST_INSTANT_REPLAY.call(c, 0x0027_0614)?;
                    c.g[RA][0] = Addr(sp(c)).ld(c);
                }
            }
        }
        c.cycles += 2;
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `int AI_GetSyncChecksum(self)` (0x12b420): start from the AI's random value
/// and add the sync checksum of each rider in the array at `+0x28`.
pub fn ai_get_sync_checksum(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x50));
        c.sq(19, sp(c).wrapping_add(0x10));
        c.sq(17, sp(c).wrapping_add(0x30));
        c.g[S3][0] = c.g[A0][0];
        c.sq(18, sp(c).wrapping_add(0x20));
        c.sq(16, sp(c).wrapping_add(0x40));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S2][0] = 0;
        call_ptr(c, callee::AI_RAND, 0x0012_b444)?;
        c.g[V1][0] = lw_r(c, S3, 0x78);
        let empty = (c.g[V1][0] as i64) <= 0;
        c.g[S1][0] = c.g[V0][0];
        if !empty {
            c.cycles += 2;
            c.g[S0][0] = sx(low(c, S3).wrapping_add(0x28));
            loop {
                c.cycles += 8;
                c.g[A0][0] = Addr(low(c, S0)).lw(c);
                c.g[S2][0] = sx(low(c, S2).wrapping_add(1));
                c.g[S0][0] = sx(low(c, S0).wrapping_add(4));
                callee::RIDER_GET_SYNC_CHECKSUM.call(c, 0x0012_b468)?;
                c.g[V1][0] = lw_r(c, S3, 0x78);
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                c.g[S1][0] = sx(low(c, S1).wrapping_add(low(c, V0)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_b458);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, sp(c).wrapping_add(0x40));
        c.lq(17, sp(c).wrapping_add(0x30));
        c.lq(18, sp(c).wrapping_add(0x20));
        c.lq(19, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2 memcard save-name stubs (world-painter module).

/// Shared body of the three memcard stubs: build the ASCII copy of the name at
/// `+0xf9`, then record the two payload words and hand `state` to the memcard
/// state machine.
fn mem_card_save_name(
    c: &mut Cpu,
    a2: u64,
    a3: u64,
    state: u64,
    wide: bool,
    rets: [u32; 3],
) -> Flow {
    c.cycles += 30;
    c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x50));
    c.sq(16, sp(c).wrapping_add(0x40));
    c.g[S0][0] = c.g[A0][0];
    c.sq(17, sp(c).wrapping_add(0x30));
    c.sq(18, sp(c).wrapping_add(0x20));
    c.g[S1][0] = sx(low(c, S0).wrapping_add(0xb9));
    c.sq(19, sp(c).wrapping_add(0x10));
    c.g[S2][0] = a2;
    c.sd(sp(c), c.g[RA][0]);
    c.g[S3][0] = a3;
    if wide {
        c.g[A0][0] = c.g[S1][0];
        callee::USTR_TO_ASCII.call(c, rets[0])?;
    } else {
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::STRCPY, rets[0])?;
    }
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = 0x4a_0000;
    c.g[A0][0] = sx(low(c, S0).wrapping_add(0xf9));
    c.g[A3][0] = c.g[A2][0];
    c.g[A1][0] = sx(low(c, A1).wrapping_add(0x3948));
    call_ptr(c, callee::SPRINTF, rets[1])?;
    sw_r(c, S0, 0x2c, c.g[S2][0]);
    c.g[A0][0] = c.g[S0][0];
    sw_r(c, S0, 0x20, c.g[S3][0]);
    c.g[A1][0] = state;
    callee::CPS2_MEM_CARD_SET_STATE.call(c, rets[2])?;
    c.lq(16, sp(c).wrapping_add(0x40));
    c.lq(17, sp(c).wrapping_add(0x30));
    c.lq(18, sp(c).wrapping_add(0x20));
    c.lq(19, sp(c).wrapping_add(0x10));
    c.g[RA][0] = Addr(sp(c)).ld(c);
    let t = low(c, RA);
    c.g[SP][0] = sx(low(c, SP).wrapping_add(0x50));
    c.pc = t;
    Ok(())
}

/// `void cPS2MemCard_v37(self, wname, a2, a3)` (0x2c45d0): wide-character name.
pub fn cps2_mem_card_v37(c: &mut Cpu) {
    guest(c, |c| {
        let a2 = c.g[A2][0];
        let a3 = c.g[A3][0];
        mem_card_save_name(c, a2, a3, 0x15, true, [0x002c_4600, 0x002c_4618, 0x002c_462c])
    })
}

/// `void cPS2MemCard_v36(self, name, a2, a3)` (0x2c4648): narrow name.
pub fn cps2_mem_card_v36(c: &mut Cpu) {
    guest(c, |c| {
        let a2 = c.g[A2][0];
        let a3 = c.g[A3][0];
        mem_card_save_name(c, a2, a3, 0x15, false, [0x002c_4678, 0x002c_4690, 0x002c_46a4])
    })
}

/// `void cPS2MemCard_RequestReadSave(self, name, a2, a3)` (0x2c4788): same,
/// entering the `0x1b` (read) state.
pub fn cps2_mem_card_request_read_save(c: &mut Cpu) {
    guest(c, |c| {
        let a2 = c.g[A2][0];
        let a3 = c.g[A3][0];
        mem_card_save_name(c, a2, a3, 0x1b, false, [0x002c_47b8, 0x002c_47d0, 0x002c_47e4])
    })
}

// ---------------------------------------------------------------------------
// UI construction / teardown.

/// `void UIList_dtor(self, flags)` (0x3a4888): deleting destructor of a UI
/// list — delete the entries, reset the two embedded list nodes and free the
/// object when `flags & 1`.
pub fn ui_list_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.g[V0][0] = 0x49_0000;
        c.sq(18, sp(c).wrapping_add(0x10));
        c.g[V0][0] = sx(low(c, V0).wrapping_add(0x4c98));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, sp(c).wrapping_add(0x30));
        c.sq(17, sp(c).wrapping_add(0x20));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        sw_r(c, S2, 0x18, c.g[V0][0]);
        callee::UI_LIST_DELETE_ALL.call(c, 0x003a_48b4)?;
        c.g[S0][0] = 0x49_0000;
        c.g[A0][0] = sx(low(c, S2).wrapping_add(0xc));
        c.g[S0][0] = sx(low(c, S0).wrapping_add(0x4cc0));
        sw_r(c, S2, 0x14, c.g[S0][0]);
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x003a_48c8)?;
        sw_r(c, S2, 0x8, c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x003a_48d4)?;
        c.g[S1][0] = c.g[S1][0] & 0x1;
        let keep = c.g[S1][0] == 0;
        c.lq(16, sp(c).wrapping_add(0x30));
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, callee::BUILTIN_DELETE.addr, 0x003a_48e8)?;
            c.lq(16, sp(c).wrapping_add(0x30));
        }
        c.cycles += 5;
        c.lq(17, sp(c).wrapping_add(0x20));
        c.lq(18, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void BoaderShadow_dtor(self, flags)` (0x375918): release the texture held
/// at `+0x170` (unlock the vram, then free the surface through its vtable)
/// and free the object when `flags & 1`.
pub fn boader_shadow_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(16, sp(c).wrapping_add(0x20));
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = lw_r(c, S0, 0x170);
        let none = (c.g[A1][0] as i64) < 0;
        c.g[V0][0] = lw_r(c, GP, 0x2a90);
        if !none {
            c.cycles += 11;
            c.g[A0][0] = lw_r(c, V0, 0x18f4);
            callee::TEX_MAN_UNLOCK_VRAM.call(c, 0x0037_5944)?;
            c.g[A2][0] = lw_r(c, GP, 0x2a90);
            c.g[A1][0] = lw_r(c, S0, 0x170);
            c.g[V0][0] = lw_r(c, A2, 0x10d8);
            c.g[A0][0] = lh_r(c, V0, 0x190);
            c.g[V1][0] = lw_r(c, V0, 0x194);
            let target = low(c, V1);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0037_5960)?;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            sw_r(c, S0, 0x170, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] == 0 {
            c.lq(16, sp(c).wrapping_add(0x20));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::BUILTIN_DELETE.addr, 0x0037_597c)?;
            c.lq(16, sp(c).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void WScriptMan_DefineCollectSet(self, id, arg)` (0x30b928): look through
/// the collect-set slots at `+0x2c0` (stride 0x10c) for `id`; take a free
/// (`-1`) slot by resetting it, else skip.
pub fn wscript_man_define_collect_set(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.g[A3][0] = 0xffff_ffff_ffff_0000;
        c.sq(16, sp(c).wrapping_add(0x30));
        c.g[V1][0] = 0;
        c.sq(17, sp(c).wrapping_add(0x20));
        c.g[S0][0] = sx(low(c, A0).wrapping_add(0x2c0));
        c.sq(18, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A2][0];
        c.g[A3][0] = c.g[A3][0] | 0xffff;
        'outer: loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(low(c, S0)).lw(c);
            if c.g[V0][0] == c.g[S1][0] {
                c.lq(16, sp(c).wrapping_add(0x30));
                break 'outer;
            }
            c.cycles += 2;
            let free = c.g[V0][0] == c.g[A3][0];
            c.g[V1][0] = sx(low(c, V1).wrapping_add(1));
            if free {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::WSCRIPT_MAN_RESET_COLLECT_SET.call(c, 0x0030_b974)?;
                sw_r(c, S0, 0x4, c.g[S2][0]);
                sw_r(c, S0, 0, c.g[S1][0]);
                c.cycles += 1;
                c.lq(16, sp(c).wrapping_add(0x30));
                break 'outer;
            }
            c.cycles += 2;
            let more = (c.g[V1][0] as i64) <= 0;
            c.g[S0][0] = sx(low(c, S0).wrapping_add(0x10c));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0030_b958);
                }
                continue;
            }
            c.cycles += 1;
            c.lq(16, sp(c).wrapping_add(0x30));
            break;
        }
        c.cycles += 5;
        c.lq(17, sp(c).wrapping_add(0x20));
        c.lq(18, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void RiderModel_AllocModelRefs(self, count, unused)` (0x30d840): allocate
/// `count` 0x58-byte model references, zero them and store the block.
pub fn rider_model_alloc_model_refs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.g[V0][0] = c.g[A1][0];
        c.sq(16, sp(c).wrapping_add(0x20));
        c.g[A1][0] = 0x49_0000;
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = 0x58;
        c.sd(sp(c), c.g[RA][0]);
        c.mult(0, ZERO, V0, S1); // lo = count * 0x58
        sw_r(c, S0, 0x4, c.g[A2][0]);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(low(c, A1).wrapping_sub(0x6390));
        c.g[A0][0] = c.lo[0];
        sw_r(c, S0, 0, c.g[V0][0]);
        callee::OPERATOR_NEW_NAMED.call(c, 0x0030_d880)?;
        c.g[A2][0] = lw_r(c, S0, 0);
        c.g[A0][0] = c.g[V0][0];
        sw_r(c, S0, 0xc, c.g[V0][0]);
        c.mult(0, ZERO, A2, S1); // lo = count * 0x58
        c.g[A2][0] = c.lo[0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::MEMSET_3E6448, 0x0030_d89c)?;
        sw_r(c, S0, 0x60, 0);
        sw_r(c, S0, 0x8, 0);
        c.lq(16, sp(c).wrapping_add(0x20));
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void ModObj_AddRailModifier(self, desc)` (0x355e38): unless the object's
/// modifier block already has a rail modifier for `desc+4`, allocate a
/// 0x90-byte `RailModifier` and install it.
pub fn mod_obj_add_rail_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(16, sp(c).wrapping_add(0x20));
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = lw_r(c, S0, 0x1c);
        let none = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        let mut done = false;
        if !none {
            c.cycles += 4;
            c.g[A1][0] = lw_r(c, S1, 0x4);
            callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER.call(c, 0x0035_5e60)?;
            if c.g[V0][0] != 0 {
                c.lq(16, sp(c).wrapping_add(0x20));
                done = true;
            }
        }
        if !done {
            c.cycles += 14;
            c.g[A1][0] = 0x49_0000;
            c.g[A0][0] = 0x90;
            c.g[A1][0] = sx(low(c, A1).wrapping_sub(0x1648));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::C_MEM_MAN_ALLOC, 0x0035_5e80)?;
            c.g[A2][0] = lw_r(c, S0, 0x18);
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            callee::RAIL_MODIFIER_CTOR.call(c, 0x0035_5e90)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            callee::MOD_OBJ_SET_RAIL_MODIFIER.call(c, 0x0035_5e9c)?;
            c.lq(16, sp(c).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `MagnetModifier* MagnetModifier_ctor(self, owner, mat, float a, float b)`
/// (0x3572e0): install the vtable, copy the 4x4 matrix into `+0x60` through
/// VU0, store the speed (b * 100/3.6) and the quad at `+0x10`.
pub fn magnet_modifier_ctor(c: &mut Cpu) {
    c.cycles += 30;
    c.lwc1(0, gpo(c, -0x2a84));
    c.g[V0][0] = 0x49_0000;
    c.g[V0][0] = sx(low(c, V0).wrapping_sub(0xbe0));
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.mul_s(13, 13, 0);
    c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
    sw_r(c, A0, 0, c.g[V0][0]);
    c.g[A3][0] = sx(low(c, A0).wrapping_add(0x60));
    sw_r(c, A0, 0xa0, c.g[A1][0]);
    swc1_r(c, 12, A0, 0x40);
    swc1_r(c, 13, A0, 0x48);
    sw_r(c, A0, 0x44, c.g[V1][0]);
    sw_r(c, A0, 0x4c, 0);
    sw_r(c, A0, 0x54, 0);
    c.lqc2(1, low(c, A2));
    c.lqc2(2, low(c, A2).wrapping_add(0x10));
    c.lqc2(3, low(c, A2).wrapping_add(0x20));
    c.lqc2(4, low(c, A2).wrapping_add(0x30));
    c.sqc2(1, low(c, A3));
    c.sqc2(2, low(c, A3).wrapping_add(0x10));
    c.sqc2(3, low(c, A3).wrapping_add(0x20));
    c.sqc2(4, low(c, A3).wrapping_add(0x30));
    c.lq(3, low(c, A0).wrapping_add(0x90));
    c.g[V0][0] = 0x1;
    sw_r(c, A0, 0x50, c.g[V0][0]);
    c.sq(3, low(c, A0).wrapping_add(0x10));
    c.g[V0][0] = c.g[A0][0];
    c.sq(3, sp(c));
    let t = low(c, RA);
    c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
    c.pc = t;
}

/// `void MultiParticleMan_dtor(self, flags)` (0x357a00): destroy the four
/// owned particle groups, clear the global manager pointer and free the
/// object when `flags & 1`.
pub fn multi_particle_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x50));
        c.sq(18, sp(c).wrapping_add(0x20));
        c.sq(16, sp(c).wrapping_add(0x40));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, sp(c).wrapping_add(0x30));
        c.g[S0][0] = c.g[S2][0];
        c.sq(19, sp(c).wrapping_add(0x10));
        c.g[S1][0] = 0x3;
        c.sd(sp(c), c.g[RA][0]);
        c.g[S3][0] = c.g[A1][0];
        loop {
            c.cycles += 3;
            c.g[A0][0] = Addr(low(c, S0)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = 0x3;
                callee::MULTI_PARTICLE_DTOR.call(c, 0x0035_7a3c)?;
            }
            c.g[S1][0] = sx(low(c, S1).wrapping_sub(1));
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(low(c, S0).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0035_7a28);
                }
                continue;
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[S3][0] & 0x1;
            let keep = c.g[V0][0] == 0;
            sw_r(c, GP, 0xf38, 0);
            if !keep {
                c.cycles += 2;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, callee::BUILTIN_DELETE.addr, 0x0035_7a5c)?;
            }
            c.cycles += 7;
            c.lq(16, sp(c).wrapping_add(0x40));
            c.lq(17, sp(c).wrapping_add(0x30));
            c.lq(18, sp(c).wrapping_add(0x20));
            c.lq(19, sp(c).wrapping_add(0x10));
            c.g[RA][0] = Addr(sp(c)).ld(c);
            let t = low(c, RA);
            c.g[SP][0] = sx(low(c, SP).wrapping_add(0x50));
            c.pc = t;
            break;
        }
        Ok(())
    })
}

/// `void cFEStateRewardGallery_UpdateCountText(self)` (0x1d0eb8): re-render
/// the two count labels from the stored totals.
pub fn c_fe_state_reward_gallery_update_count_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.sq(16, sp(c).wrapping_add(0x30));
        c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_r(c, S0, 0x918);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = lw_r(c, S0, 0x914);
        } else {
            c.cycles += 9;
            c.g[A1][0] = 0x4a_0000;
            c.g[A2][0] = lw_r(c, S0, 0x6c);
            c.g[A1][0] = sx(low(c, A1).wrapping_add(0x12c0));
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, callee::SPRINTF, 0x001d_0ee8)?;
            c.g[A0][0] = lw_r(c, S0, 0x918);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0c50, 0x001d_0ef4)?;
            c.g[V0][0] = lw_r(c, S0, 0x914);
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lq(16, sp(c).wrapping_add(0x30));
        } else {
            c.cycles += 9;
            c.g[A1][0] = 0x4a_0000;
            c.g[A2][0] = lw_r(c, S0, 0x68);
            c.g[A1][0] = sx(low(c, A1).wrapping_add(0x12c0));
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, callee::SPRINTF, 0x001d_0f14)?;
            c.g[A0][0] = lw_r(c, S0, 0x914);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0c50, 0x001d_0f20)?;
            c.lq(16, sp(c).wrapping_add(0x30));
        }
        c.cycles += 3;
        c.g[RA][0] = Addr(sp(c).wrapping_add(0x20)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `int Player_GetUberGrindButton(self)` (0x127848): which of the four uber
/// grind buttons (0x1b..0x1e) is held, or -1.
pub fn player_get_uber_grind_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x20));
        c.g[A1][0] = 0x1b;
        c.sq(16, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = lw_r(c, S0, 0xdf0);
        call_ptr(c, 0x0032_0c48, 0x0012_7864)?;
        let mut v0;
        if c.g[V0][0] != 0 {
            v0 = 0;
        } else {
            c.cycles += 5;
            c.g[A0][0] = lw_r(c, S0, 0xdf0);
            c.g[A1][0] = 0x1c;
            call_ptr(c, 0x0032_0c48, 0x0012_7878)?;
            if c.g[V0][0] != 0 {
                v0 = 1;
            } else {
                c.cycles += 5;
                c.g[A0][0] = lw_r(c, S0, 0xdf0);
                c.g[A1][0] = 0x1d;
                call_ptr(c, 0x0032_0c48, 0x0012_788c)?;
                if c.g[V0][0] != 0 {
                    v0 = 2;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = lw_r(c, S0, 0xdf0);
                    c.g[A1][0] = 0x1e;
                    call_ptr(c, 0x0032_0c48, 0x0012_78a0)?;
                    c.g[A0][0] = 0x3;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[A0][0];
                    }
                    v0 = low(c, V1);
                }
            }
        }
        c.g[V0][0] = sx(v0);
        c.cycles += 4;
        c.lq(16, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void WScriptTask_Update(self)` (0x308328): a two-state task step — state
/// 1 arms state 2 and runs the vtable action, state 2 just runs it.
pub fn wscript_task_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[A1][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = 0x1;
        c.g[V1][0] = Addr(low(c, A1)).lw(c);
        let one = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        if !one {
            let below = c.g[V0][0] != 0;
            c.cycles += 2;
            if below {
                c.g[RA][0] = Addr(sp(c)).ld(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x2;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = lw_r(c, A1, 0x40);
                    c.cycles += 5;
                    c.g[A0][0] = lh_r(c, V0, 0x28);
                    c.g[V1][0] = lw_r(c, V0, 0x2c);
                    let target = low(c, V1);
                    c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x0030_8394)?;
                    c.g[RA][0] = Addr(sp(c)).ld(c);
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = Addr(sp(c)).ld(c);
                }
            }
        } else {
            c.cycles += 9;
            c.g[V0][0] = 0x2;
            c.g[V1][0] = lw_r(c, A1, 0x40);
            sw_r(c, A1, 0, c.g[V0][0]);
            c.g[A0][0] = lh_r(c, V1, 0x28);
            c.g[V0][0] = lw_r(c, V1, 0x2c);
            let target = low(c, V0);
            c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0030_837c)?;
            c.g[RA][0] = Addr(sp(c)).ld(c);
        }
        c.cycles += 2;
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateOptionsSaveLoad_OnRender(self)` (0x18d3c0): when the options
/// screen is live, reveal the two option widgets found by name hash.
pub fn c_fe_state_options_save_load_on_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x20));
        c.g[V0][0] = Addr(gpo(c, -0x1718)).lw(c);
        c.sq(16, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        let off = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if off {
            c.cycles += 1;
            c.lq(16, sp(c).wrapping_add(0x10));
        } else {
            c.cycles += 8;
            c.g[A0][0] = 0x46_0000;
            c.g[A0][0] = sx(low(c, A0).wrapping_sub(0x1f10));
            call_ptr(c, callee::STR_HASH, 0x0018_d3e4)?;
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
            c.g[A0][0] = sx(low(c, A0).wrapping_sub(0x1f00));
            call_ptr(c, callee::STR_HASH, 0x0018_d40c)?;
            c.g[A0][0] = lw_r(c, S0, 0x40);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME, 0x0018_d418)?;
            let none = c.g[V0][0] == 0;
            c.lq(16, sp(c).wrapping_add(0x10));
            if none {
                c.cycles += 3;
                c.g[RA][0] = Addr(sp(c)).ld(c);
                let t = low(c, RA);
                c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
                c.pc = t;
                return Ok(());
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[V0][0];
            callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF.call(c, 0x0018_d428)?;
            c.cycles += 1;
            c.lq(16, sp(c).wrapping_add(0x10));
        }
        c.cycles += 3;
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `int OVScreenBEOptions_OnWidgetNotify(self, menu, key, index)` (0x1faa78):
/// the pause-menu widget notification filter — 0x101 for the keys it does not
/// handle, 0x100 for "start"/"options" and for the pause title.
pub fn ov_screen_be_options_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x20));
        c.g[A0][0] = c.g[A1][0];
        c.sq(16, sp(c).wrapping_add(0x10));
        c.g[A1][0] = c.g[A3][0];
        c.g[V0][0] = 0x6;
        let title = c.g[A2][0] == c.g[V0][0];
        c.sd(sp(c), c.g[RA][0]);
        if title {
            c.cycles += 8;
            c.g[A0][0] = sx(low(c, A0).wrapping_add(0x74));
            call_ptr(c, 0x0039_7870, 0x001f_aac4)?;
            c.g[A0][0] = 0x4a_0000;
            c.g[S0][0] = lw_r(c, V0, 0x38);
            c.g[A0][0] = sx(low(c, A0).wrapping_add(0x24e8));
            call_ptr(c, callee::STR_HASH, 0x001f_aad4)?;
            if c.g[S0][0] != c.g[V0][0] {
                c.g[V0][0] = 0x100;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0x101;
            }
        } else {
            c.cycles += 3;
            if c.g[A2][0] < 0x6 {
                c.g[V0][0] = 0x101;
            } else {
                c.cycles += 3;
                if !(c.g[A2][0] < 0xa) {
                    c.cycles += 1;
                    c.g[V0][0] = 0x101;
                } else if c.g[A2][0] < 0x8 {
                    c.cycles += 2;
                    c.g[V0][0] = 0x101;
                } else {
                    c.cycles += 2;
                    c.cycles += 2;
                    c.g[V0][0] = 0x100;
                }
            }
        }
        c.cycles += 4;
        c.lq(16, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void BERewardInt_GrantMedalReward(self, a1, a2)` (0x159170): compute the
/// medal reward id and grant it unless it was already granted.
pub fn be_reward_int_grant_medal_reward(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x50));
        c.sq(16, sp(c).wrapping_add(0x40));
        c.sq(17, sp(c).wrapping_add(0x30));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, sp(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A2][0];
        callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID.call(c, 0x0015_9198)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        callee::BE_REWARD_INT_IS_REWARD_GRANTED.call(c, 0x0015_91b0)?;
        if c.g[V0][0] != 0 {
            c.lq(16, sp(c).wrapping_add(0x40));
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S3][0];
            callee::BE_REWARD_INT_GRANT_REWARD.call(c, 0x0015_91cc)?;
            c.lq(16, sp(c).wrapping_add(0x40));
        }
        c.cycles += 6;
        c.lq(17, sp(c).wrapping_add(0x30));
        c.lq(18, sp(c).wrapping_add(0x20));
        c.lq(19, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void RiderMetrix_Reset(self)` (0x117540): reset every metrics counter,
/// HUD message, trick and combo, and the two float/flag fields.
pub fn rider_metrix_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x20));
        c.sq(16, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        callee::RIDER_METRIX_RESET_STATS.call(c, 0x0011_7554)?;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_METRIX_CLEAR_HUD_MESSAGES.call(c, 0x0011_755c)?;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x50;
        c.g[A0][0] = sx(low(c, S0).wrapping_add(0xa8));
        call_ptr(c, callee::MEMSET_3E6448, 0x0011_756c)?;
        sw_r(c, S0, 0xf8, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_METRIX_RESET_TRICK.call(c, 0x0011_7578)?;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_METRIX_RESET_COMBO.call(c, 0x0011_7580)?;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(0x1b8));
        sw_r(c, S0, 0x1a8, 0);
        c.g[A1][0] = 0;
        swc1_r(c, 0, S0, 0xa4);
        c.g[A2][0] = 0x8;
        call_ptr(c, callee::MEMSET_3E6448, 0x0011_75a0)?;
        c.g[V0][0] = 0x1;
        c.g[RA][0] = Addr(sp(c)).ld(c);
        sw_r(c, S0, 0x1c0, c.g[V0][0]);
        c.lq(16, sp(c).wrapping_add(0x10));
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `float RiderMetrix_OnRiderReset(self, a1)` (0x119368): bank the pending
/// score, reset trick/combo, count the reset and return the penalty multiplier
/// applied to the model's speed value.
pub fn rider_metrix_on_rider_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, sp(c).wrapping_add(0x20));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        callee::RIDER_METRIX_BANK_PENDING_SCORE.call(c, 0x0011_9384)?;
        c.g[V1][0] = lw_r(c, S1, 0x1a4);
        c.g[V1][0] = sx(low(c, V1).wrapping_add(low(c, V0)));
        sw_r(c, S1, 0x1a4, c.g[V1][0]);
        c.g[A0][0] = c.g[S1][0];
        callee::RIDER_METRIX_RESET_TRICK.call(c, 0x0011_9398)?;
        c.g[A0][0] = c.g[S1][0];
        callee::RIDER_METRIX_RESET_COMBO.call(c, 0x0011_93a0)?;
        c.g[V0][0] = lw_r(c, S1, 0x120);
        c.g[V0][0] = sx(low(c, V0).wrapping_add(1));
        let first = c.g[S0][0] == 0;
        sw_r(c, S1, 0x120, c.g[V0][0]);
        if first {
            c.cycles += 2;
            c.g[V0][0] = lw_r(c, S1, 0x1ac);
            c.lwc1(1, gpo(c, -0x7a24));
        } else {
            c.cycles += 3;
            c.lwc1(1, gpo(c, -0x7a28));
            c.g[V0][0] = lw_r(c, S1, 0x1ac);
        }
        c.cycles += 7;
        lwc1_r(c, 0, V0, 0x2f8);
        c.mul_s(0, 0, 1);
        c.lq(16, sp(c).wrapping_add(0x20));
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `int BENewPlayerInt_GetCharStateFlag(a, b, c, peak)` (0x145f90): read one
/// bit of the character record's state word — bit 12 for peak 1, bit 13 for
/// any other non-zero peak, 0 for peak 0.
pub fn be_new_player_int_get_char_state_flag(c: &mut Cpu) {
    c.cycles += 2;
    let mut peak = c.g[A3][0];
    c.g[V0][0] = 0x1;
    if peak == 0 {
        c.cycles += 2;
        let t = low(c, RA);
        c.g[V0][0] = 0;
        c.pc = t;
        return;
    }
    c.cycles += 2;
    peak = c.g[A3][0];
    c.g[V1][0] = 0xf88;
    let shift = if peak != c.g[V0][0] { 19 } else { 20 };
    c.cycles += 12;
    c.g[A0][0] = 0x9b50;
    c.mult(0, V1, A2, V1); // lo = b * 0xf88
    c.mult(0, A0, A1, A0); // lo = a * 0x9b50
    c.g[V0][0] = 0x4a_0000;
    c.g[V0][0] = sx(low(c, V0).wrapping_add(0x6ca8));
    c.g[V1][0] = sx(low(c, V1).wrapping_add(low(c, A0)));
    c.g[V1][0] = sx(low(c, V1).wrapping_add(low(c, V0)));
    c.g[V0][0] = Addr(low(c, V1).wrapping_add(0x278)).ld(c);
    c.g[V0][0] = c.g[V0][0] << shift;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
    let t = low(c, RA);
    c.g[V0][0] = c.g[V0][0] & 0x1;
    c.pc = t;
}

/// `int BENewPlayerInt_GetCharRecordField27C(index)` (0x146d98): the character
/// record's word at `+0x27c`, indexed by profile and character id.
pub fn be_new_player_int_get_char_record_field_27c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(16, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(17, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = c.g[S0][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_6db4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        callee::BE_NEW_PLAYER_INT_GET_SLOT.call(c, 0x0014_6dc0)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x53_0000;
        c.mult(0, V0, V0, V1); // lo = slot * 0x1c
        c.g[A1][0] = sx(low(c, A1).wrapping_add(0x4fe0));
        c.g[V1][0] = 0x9b50;
        c.g[A2][0] = 0xf88;
        c.mult(0, S1, S1, V1); // lo = profile * 0x9b50
        c.g[A0][0] = 0x4a_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6ca8));
        c.lq(16, sp(c).wrapping_add(0x20));
        c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, A1)));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        c.g[V1][0] = lb_r(c, V0, 0x11);
        c.mult(0, V1, V1, A2); // lo = charid * 0xf88
        c.g[V1][0] = sx(low(c, V1).wrapping_add(low(c, S1)));
        c.g[V1][0] = sx(low(c, V1).wrapping_add(low(c, A0)));
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[V0][0] = lw_r(c, V1, 0x27c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void* BENewPlayerInt_GetInbox(index)` (0x147908): the character record
/// base at `+0xe38` for the profile/character indexed by `index`.
pub fn be_new_player_int_get_inbox(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(16, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(17, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = c.g[S0][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_7924)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        callee::BE_NEW_PLAYER_INT_GET_SLOT.call(c, 0x0014_7930)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x53_0000;
        c.mult(0, V0, V0, V1); // lo = slot * 0x1c
        c.g[A1][0] = sx(low(c, A1).wrapping_add(0x4fe0));
        c.g[V1][0] = 0x9b50;
        c.g[A2][0] = 0xf88;
        c.mult(0, S1, S1, V1); // lo = profile * 0x9b50
        c.g[A0][0] = 0x4a_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6ca8));
        c.lq(16, sp(c).wrapping_add(0x20));
        c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, A1)));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        c.g[V1][0] = lb_r(c, V0, 0x11);
        c.mult(0, V1, V1, A2); // lo = charid * 0xf88
        c.g[V1][0] = sx(low(c, V1).wrapping_add(low(c, A0)));
        c.g[S1][0] = sx(low(c, S1).wrapping_add(low(c, V1)));
        c.g[V0][0] = sx(low(c, S1).wrapping_add(0xe38));
        c.lq(17, sp(c).wrapping_add(0x10));
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Object constructors / destructors and small managers.

/// `void* BoostModifier_ctor_2(self, stream)` (0x342808): install the vtable,
/// clear the header word and read two 4-word blocks through the stream's
/// reader table.
pub fn boost_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.g[V1][0] = 0x49_0000;
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x898));
        c.sq(16, sp(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = 0x1;
        sw_r(c, S1, 0x4, c.g[V1][0]);
        c.g[S0][0] = c.g[A1][0];
        sw_r(c, S1, 0, c.g[V0][0]);
        c.g[A1][0] = sx(low(c, S1).wrapping_add(0x10));
        c.g[A2][0] = 0x30;
        c.g[V0][0] = Addr(low(c, S0)).lw(c);
        c.g[A0][0] = lh_r(c, V0, 0x10);
        c.g[A3][0] = lw_r(c, V0, 0x14);
        let target = low(c, A3);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0034_2850)?;
        c.g[V1][0] = Addr(low(c, S0)).lw(c);
        c.g[A0][0] = lh_r(c, V1, 0x18);
        c.g[V0][0] = lw_r(c, V1, 0x1c);
        let target = low(c, V0);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0034_2864)?;
        sw_r(c, S1, 0x40, c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, sp(c).wrapping_add(0x20));
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void GameModeMan_Destroy()` (0x2380e8): delete the ten mode singletons and
/// the optional extra object.
pub fn game_mode_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.g[V0][0] = 0x53_0000;
        c.sq(16, sp(c).wrapping_add(0x20));
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[S0][0] = sx(low(c, V0).wrapping_add(0x6668));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = 0x9;
        loop {
            c.cycles += 3;
            c.g[A2][0] = Addr(low(c, S0)).lw(c);
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 0x3;
            if !none {
                c.cycles += 5;
                c.g[V0][0] = Addr(low(c, A2)).lw(c);
                c.g[A0][0] = lh_r(c, V0, 0x8);
                c.g[V1][0] = lw_r(c, V0, 0xc);
                let target = low(c, V1);
                c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
                call_ptr(c, target, 0x0023_8128)?;
            }
            c.cycles += 3;
            c.g[S1][0] = sx(low(c, S1).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(low(c, S0).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0023_8108);
                }
                continue;
            }
            c.cycles += 3;
            c.g[A0][0] = Addr(gpo(c, -0x484)).lw(c);
            let gone = c.g[A0][0] == 0;
            c.lq(16, sp(c).wrapping_add(0x20));
            if !gone {
                c.cycles += 3;
                call_ptr(c, callee::BUILTIN_DELETE.addr, 0x0023_8148)?;
                c.lq(16, sp(c).wrapping_add(0x20));
            }
            c.cycles += 5;
            c.lq(17, sp(c).wrapping_add(0x10));
            c.g[RA][0] = Addr(sp(c)).ld(c);
            sw_r(c, GP, (-0x484i32) as u32, 0);
            let t = low(c, RA);
            c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
            c.pc = t;
            break;
        }
        Ok(())
    })
}

/// `void GMHandler_ClearStandings()` (0x238d30): zero the five standing tables.
pub fn gm_handler_clear_standings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[A0][0] = 0x53_0000;
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6730));
        c.g[A1][0] = 0x9;
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8d4c)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6708));
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8d60)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6640));
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8d74)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x66a8));
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8d88)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x66d0));
        c.g[A2][0] = 0x18;
        call_ptr(c, callee::MEMSET, 0x0023_8d9c)?;
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void GMHandler_ClearStandings2()` (0x238da8): same, but the third table is
/// filled with `0x09`.
pub fn gm_handler_clear_standings2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[A0][0] = 0x53_0000;
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6730));
        c.g[A1][0] = 0x9;
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8dc4)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6708));
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8dd8)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0x9;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6640));
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8dec)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x66a8));
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8e00)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x66d0));
        c.g[A2][0] = 0x18;
        call_ptr(c, callee::MEMSET, 0x0023_8e14)?;
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void cBXString_AllocCopy(dst, src, count, start, extra)` (0x3181e8):
/// allocate `count + extra` characters and copy them from `src->chars +
/// start`; a zero total aliases the shared empty string.
pub fn cbx_string_alloc_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x50));
        c.sq(17, sp(c).wrapping_add(0x30));
        c.sq(16, sp(c).wrapping_add(0x40));
        c.g[S1][0] = c.g[A2][0];
        c.sq(18, sp(c).wrapping_add(0x20));
        c.g[V0][0] = sx(low(c, S1).wrapping_add(low(c, T0)));
        c.sq(19, sp(c).wrapping_add(0x10));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        let empty = c.g[V0][0] != 0;
        c.g[S3][0] = c.g[A3][0];
        if empty {
            c.cycles += 8;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            callee::CBX_STRING_ALLOC_BUFFER.call(c, 0x0031_8230)?;
            c.g[A1][0] = Addr(low(c, S2)).lw(c);
            c.g[A2][0] = c.g[S1][0];
            c.g[A0][0] = Addr(low(c, S0)).lw(c);
            c.g[A1][0] = sx(low(c, A1).wrapping_add(low(c, S3)));
            call_ptr(c, callee::MEMCPY, 0x0031_8244)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(gpo(c, 0xda0)).lw(c);
            sw_r(c, S0, 0, c.g[V0][0]);
        }
        c.cycles += 7;
        c.lq(16, sp(c).wrapping_add(0x40));
        c.lq(17, sp(c).wrapping_add(0x30));
        c.lq(18, sp(c).wrapping_add(0x20));
        c.lq(19, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void cBXString_dtor(self, flags)` (0x318260): drop the character buffer's
/// reference count, free it at zero and delete the object when `flags & 1`.
pub fn cbx_string_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.g[V0][0] = 0x49_0000;
        c.sq(16, sp(c).wrapping_add(0x20));
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(0x232c));
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V1][0] = Addr(low(c, S0)).lw(c);
        let shared = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        let mut extra = true;
        let mut result;
        if shared {
            result = low(c, S1) & 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(low(c, V1).wrapping_sub(0xc)).lw(c);
            c.g[V0][0] = sx(low(c, V0).wrapping_sub(1));
            let alive = (c.g[V0][0] as i64) > 0;
            sw_r(c, V1, 0xffff_fff4, c.g[V0][0]);
            if alive {
                result = low(c, S1) & 1;
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(low(c, S0)).lw(c);
                c.g[A0][0] = sx(low(c, V0).wrapping_sub(0xc));
                let null = c.g[A0][0] == 0;
                result = low(c, S1) & 1;
                c.g[V0][0] = c.g[S1][0] & 0x1;
                if null {
                    extra = false;
                } else {
                    c.cycles += 2;
                    callee::EA_MEM_FREE.call(c, 0x0031_82b0)?;
                }
            }
        }
        if extra {
            c.cycles += 1;
        }
        c.g[V0][0] = result as u64;
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lq(16, sp(c).wrapping_add(0x20));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::BUILTIN_DELETE.call(c, 0x0031_82c4)?;
            c.lq(16, sp(c).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `cBXString* cBXString_ctor_cstr(self, str)` (0x3182d8): point at the shared
/// empty string, then copy `strlen(str)` characters when `str` is non-empty.
pub fn cbx_string_ctor_cstr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.g[V0][0] = Addr(gpo(c, 0xda0)).lw(c);
        c.sq(18, sp(c).wrapping_add(0x10));
        c.sq(17, sp(c).wrapping_add(0x20));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, sp(c).wrapping_add(0x30));
        c.g[S1][0] = c.g[A1][0];
        c.sd(sp(c), c.g[RA][0]);
        let null = c.g[S1][0] == 0;
        sw_r(c, S2, 0, c.g[V0][0]);
        if null {
            c.cycles += 1;
            c.g[S0][0] = 0;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::STRLEN, 0x0031_8308)?;
            c.g[S0][0] = c.g[V0][0];
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S2][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A1][0] = c.g[S0][0];
            callee::CBX_STRING_ALLOC_BUFFER.call(c, 0x0031_8324)?;
            c.g[A0][0] = Addr(low(c, S2)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, callee::MEMCPY, 0x0031_8334)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        c.lq(16, sp(c).wrapping_add(0x30));
        c.lq(17, sp(c).wrapping_add(0x20));
        c.lq(18, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void* Debounce_ctor(self, unused, stream)` (0x342d10): base move-node
/// constructor, install the debounce vtable and read the two 4-byte debounce
/// fields through the stream reader.
pub fn debounce_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(16, sp(c).wrapping_add(0x20));
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A2][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        callee::MOD_OBJ_CTOR_2.call(c, 0x0034_2d2c)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A1][0] = sx(low(c, S1).wrapping_add(0x2c));
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x6f0));
        c.g[A2][0] = 0x4;
        sw_r(c, S1, 0xc, c.g[V1][0]);
        c.g[V0][0] = Addr(low(c, S0)).lw(c);
        c.g[A0][0] = lh_r(c, V0, 0x10);
        c.g[V1][0] = lw_r(c, V0, 0x14);
        let target = low(c, V1);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0034_2d54)?;
        c.g[A3][0] = Addr(low(c, S0)).lw(c);
        c.g[A1][0] = sx(low(c, S1).wrapping_add(0x30));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = lh_r(c, A3, 0x10);
        c.g[V0][0] = lw_r(c, A3, 0x14);
        let target = low(c, V0);
        c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
        call_ptr(c, target, 0x0034_2d70)?;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, sp(c).wrapping_add(0x20));
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateRewardsRoom_Init(self)` (0x1d4c60): create the rewards-room
/// screen by name hash, play its first frame and tell the audio system the FE
/// state is 6.
pub fn c_fe_state_rewards_room_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(16, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x7738));
        c.g[S1][0] = lw_r(c, S0, 0x10);
        call_ptr(c, callee::STR_HASH, 0x001d_4c84)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME, 0x001d_4c98)?;
        let failed = c.g[V0][0] == 0;
        sw_r(c, S0, 0x40, c.g[V0][0]);
        if !failed {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, callee::C_UI_SCREEN_PLAY_FRAME, 0x001d_4cb0)?;
        }
        c.cycles += 10;
        call_ptr(c, callee::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x001d_4cb8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x6;
        call_ptr(c, callee::FUNC_0028F140, 0x001d_4cc4)?;
        c.lq(16, sp(c).wrapping_add(0x20));
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `int TagFieldGetAddress(const char* tag, int defval)` (0x3ecff8): parse a
/// dotted IPv4 text address; digits shift in decimal, a '.' shifts the
/// accumulator left by eight, any other character ends the scan. Returns
/// `defval` for a null tag.
pub fn tag_field_get_address(c: &mut Cpu) {
    loop {
        match c.pc {
            0x003e_cff8 => {
                c.cycles += 4;
                c.g[A2][0] = c.g[A0][0];
                c.g[V0][0] = c.g[A1][0];
                let null = c.g[A2][0] == 0;
                c.g[V1][0] = 0;
                c.pc = if null { 0x003e_d068 } else { 0x003e_d008 };
            }
            0x003e_d008 => {
                c.cycles += 5;
                c.g[A1][0] = 0xffff_ffff_ffff_0000;
                c.g[T0][0] = 0xa;
                c.g[A1][0] = c.g[A1][0] | 0xff00;
                c.g[A3][0] = 0x2e;
                c.pc = 0x003e_d024;
            }
            0x003e_d020 => {
                c.cycles += 1;
                c.g[A2][0] = sx(low(c, A2).wrapping_add(1));
                c.pc = 0x003e_d024;
            }
            0x003e_d024 => {
                c.cycles += 5;
                c.g[A0][0] = lbu_r(c, A2, 0);
                c.g[V0][0] = sx(low(c, A0).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                let digit = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[V1][0] & 0xff;
                c.pc = if digit { 0x003e_d038 } else { 0x003e_d050 };
            }
            0x003e_d038 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[A0][0] & 0xf;
                c.mult(0, V0, V0, T0); // lo = (accum & 0xff) * 10
                c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, A0)));
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                if back_edge_unwinds(c, 0x003e_d020) {
                    return;
                }
                c.pc = 0x003e_d020;
            }
            0x003e_d050 => {
                c.cycles += 4;
                c.g[V0][0] = sx(low(c, A0) << 24);
                c.g[V0][0] = sx(((low(c, V0) as i32) >> 24) as u32);
                let dot = c.g[V0][0] != c.g[A3][0];
                c.g[V0][0] = c.g[V1][0];
                c.pc = if dot { 0x003e_d068 } else { 0x003e_d060 };
            }
            0x003e_d060 => {
                c.cycles += 2;
                c.g[V1][0] = sx(low(c, V1) << 8);
                if back_edge_unwinds(c, 0x003e_d020) {
                    return;
                }
                c.pc = 0x003e_d020;
            }
            0x003e_d068 => {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x003e_cff8, 0x003e_d070);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// FE module teardown, text scroll and group focus.

/// `void FEModule_DestroyRiderPreviews(self, flags)` (0x1a0420): unload the
/// previews, destroy the two rider-preview objects at `+0xce0`/`+0x19c0` and
/// free the module when `flags & 1`.
pub fn fe_module_destroy_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.sq(17, sp(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, sp(c).wrapping_add(0x10));
        c.sq(16, sp(c).wrapping_add(0x30));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS.call(c, 0x001a_0440)?;
        let null = c.g[S1][0] == 0;
        c.g[S0][0] = sx(low(c, S1).wrapping_add(0x19c0));
        let mut extra = true;
        let mut result = low(c, S2) & 1;
        if !null {
            c.cycles += 2;
            let last = c.g[S1][0] == c.g[S0][0];
            result = low(c, S2) & 1;
            if !last {
                c.cycles += 2;
                c.g[S0][0] = sx(low(c, S0).wrapping_sub(0xce0));
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    callee::FE_MODULE_RIDER_PREVIEW_DTOR.call(c, 0x001a_0464)?;
                    let more = c.g[S1][0] != c.g[S0][0];
                    c.g[S0][0] = sx(low(c, S0).wrapping_sub(0xce0));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x001a_0458);
                    }
                }
            } else {
                extra = false;
            }
        }
        if extra {
            c.cycles += 1;
        }
        c.g[V0][0] = result as u64;
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lq(16, sp(c).wrapping_add(0x30));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE.addr, 0x001a_0480)?;
            c.lq(16, sp(c).wrapping_add(0x30));
        }
        c.cycles += 5;
        c.lq(17, sp(c).wrapping_add(0x20));
        c.lq(18, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void cUITextScroll_SetTextId(self, id)` (0x3a1f18): store the text id, look
/// the widget up through `+0x5c/+0xd0/+0x10` and, when present, ask it for its
/// wide string and set it, then re-wrap.
pub fn cui_text_scroll_set_text_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x20));
        c.sq(16, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        sw_r(c, S0, 0xc0, c.g[A1][0]);
        c.g[V0][0] = lw_r(c, S0, 0x5c);
        c.g[V1][0] = lw_r(c, V0, 0xd0);
        c.g[A0][0] = lw_r(c, V1, 0x10);
        c.g[A2][0] = lw_r(c, A0, 0x10);
        if c.g[A2][0] == 0 {
            c.lq(16, sp(c).wrapping_add(0x10));
        } else {
            c.cycles += 7;
            c.g[V1][0] = lw_r(c, A2, 0x4);
            c.g[A0][0] = lh_r(c, V1, 0x20);
            c.g[V0][0] = lw_r(c, V1, 0x24);
            let target = low(c, V0);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x003a_1f58)?;
            let empty = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[V0][0];
            if !empty {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                callee::CUI_TEXT_SCROLL_SET_WIDE_STRING.call(c, 0x003a_1f68)?;
            }
            c.cycles += 7;
            c.g[V0][0] = lw_r(c, S0, 0x8);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = lh_r(c, V0, 0x28);
            c.g[V1][0] = lw_r(c, V0, 0x2c);
            let target = low(c, V1);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x003a_1f80)?;
            c.lq(16, sp(c).wrapping_add(0x10));
        }
        c.cycles += 3;
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void cUIGroup_SetFocus(self, on)` (0x398638): run the accept-visitor over
/// the group's list with a small functor, then set the group's focus flag.
pub fn cui_group_set_focus(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x50));
        c.g[V0][0] = 0x49_0000;
        c.sq(16, sp(c).wrapping_add(0x40));
        c.g[V0][0] = sx(low(c, V0).wrapping_add(0x4b98));
        c.sq(17, sp(c).wrapping_add(0x30));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, sp(c).wrapping_add(0x20));
        c.g[S1][0] = 0x49_0000;
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp(c).wrapping_add(0x10), c.g[RA][0]);
        c.g[S1][0] = sx(low(c, S1).wrapping_add(0x4ce8));
        sw_r(c, SP, 0x4, c.g[S0][0]);
        c.g[A0][0] = sx(low(c, S2).wrapping_add(0x74));
        sw_r(c, SP, 0, c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        callee::UI_LIST_ACCEPT_VISITOR.call(c, 0x0039_8678)?;
        c.g[V1][0] = lw_r(c, S2, 0x14);
        c.g[V0][0] = 0xffff_ffff_ffff_ffef;
        c.g[S0][0] = (0u64 < c.g[S0][0]) as u64;
        sw_r(c, SP, 0, c.g[S1][0]);
        c.g[S0][0] = sx(low(c, S0) << 4);
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[V1][0] = c.g[V1][0] | c.g[S0][0];
        c.lq(17, sp(c).wrapping_add(0x30));
        sw_r(c, S2, 0x14, c.g[V1][0]);
        c.lq(16, sp(c).wrapping_add(0x40));
        c.lq(18, sp(c).wrapping_add(0x20));
        c.g[RA][0] = Addr(sp(c).wrapping_add(0x10)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void FlexBridge_SpringsInit(self, desc)` (0x353e08): copy the five spring
/// parameter floats, scale the sixth, compute the segment length and build the
/// spring nodes.
pub fn flex_bridge_springs_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[V1][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.lwc1(2, gpo(c, -0x2a88));
        c.g[V0][0] = lw_r(c, A1, 0x1c);
        sw_r(c, V1, 0, c.g[V0][0]);
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(1));
        c.lwc1(0, low(c, A1));
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        swc1_r(c, 0, V1, 0x4);
        c.div_s(12, 12, 1);
        c.lwc1(0, low(c, A1).wrapping_add(0x4));
        swc1_r(c, 0, V1, 0x8);
        c.lwc1(1, low(c, A1).wrapping_add(0x8));
        swc1_r(c, 1, V1, 0xc);
        c.lwc1(0, low(c, A1).wrapping_add(0xc));
        swc1_r(c, 0, V1, 0x10);
        c.lwc1(1, low(c, A1).wrapping_add(0x10));
        swc1_r(c, 1, V1, 0x14);
        c.lwc1(0, low(c, A1).wrapping_add(0x14));
        c.mul_s(0, 0, 2);
        swc1_r(c, 0, V1, 0x18);
        c.lwc1(1, low(c, A1).wrapping_add(0x18));
        swc1_r(c, 12, V1, 0x20);
        swc1_r(c, 1, V1, 0x1c);
        callee::FLEX_BRIDGE_ALLOC_SPRING_NODES.call(c, 0x0035_3e74)?;
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void OVScreenSession_LoadMapImage(self)` (0x208f10): unload the previous
/// map image through the async manager, then start loading this screen's.
pub fn ov_screen_session_load_map_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.sq(17, sp(c).wrapping_add(0x20));
        c.sq(16, sp(c).wrapping_add(0x30));
        c.sq(18, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
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
        c.lq(16, sp(c).wrapping_add(0x30));
        c.lq(17, sp(c).wrapping_add(0x20));
        c.lq(18, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void CameraTriggerMan_streamIn(self, stream)` (0x16ce30): clean up, parse
/// the trigger list from the stream, clear each trigger's fired flag and
/// refresh the in-game mask.
pub fn camera_trigger_man_reload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(17, sp(c).wrapping_add(0x10));
        c.sq(16, sp(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        callee::CAMERA_TRIGGER_MAN_UNLOAD.call(c, 0x0016_ce4c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::CAMERA_TRIGGER_MAN_PARSE.call(c, 0x0016_ce58)?;
        c.g[V1][0] = lw_r(c, S1, 0x4);
        let empty = (c.g[V1][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[A0][0] = Addr(low(c, S1)).lw(c);
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx(low(c, A1) << 2);
                c.g[A1][0] = sx(low(c, A1).wrapping_add(1));
                c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, A0)));
                c.g[V1][0] = Addr(low(c, V0)).lw(c);
                sw_r(c, V1, 0x20, 0);
                c.g[V0][0] = lw_r(c, S1, 0x4);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(low(c, S1)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0016_ce68);
                }
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2.call(c, 0x0016_ce94)?;
        c.lq(16, sp(c).wrapping_add(0x20));
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void cStreamMan_dtor(self, flags)` (0x3a6e20): free the stream buffer when
/// the state says so, reset and free the buffers, and delete the object when
/// `flags & 1`.
pub fn c_stream_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(16, sp(c).wrapping_add(0x20));
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = lw_r(c, S0, 0xcc);
        let none = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 3;
            c.g[V0][0] = lw_r(c, S0, 0xc0);
            let zero = c.g[V0][0] == 0;
            c.g[V0][0] = 0xff;
            if !zero {
                c.cycles += 3;
                c.g[V1][0] = lbu_r(c, S0, 0xbc);
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 2;
                    callee::EA_MEM_FREE.call(c, 0x003a_6e60)?;
                }
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        callee::CSTREAM_MAN_RESET.call(c, 0x003a_6e68)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CSTREAM_MAN_FREE_BUFFERS.call(c, 0x003a_6e70)?;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] == 0 {
            c.lq(16, sp(c).wrapping_add(0x20));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::BUILTIN_DELETE.addr, 0x003a_6e84)?;
            c.lq(16, sp(c).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void* UIList_GetAt(list, index)` (0x397870): the element `index` steps from
/// the head (0 = first), or null past the end/sentinel.
pub fn ui_list_get_at(c: &mut Cpu) {
    guest(c, |c| {
        loop {
            match c.pc {
                0x0039_7870 => {
                    c.cycles += 8;
                    c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
                    c.sq(17, sp(c).wrapping_add(0x10));
                    c.sq(16, sp(c).wrapping_add(0x20));
                    c.sd(sp(c), c.g[RA][0]);
                    c.g[S1][0] = c.g[A1][0];
                    callee::UI_LIST_FIRST.call(c, 0x0039_7888)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.pc = 0x0039_7894;
                }
                0x0039_7890 => {
                    c.cycles += 1;
                    c.g[S1][0] = sx(low(c, S1).wrapping_sub(1));
                    c.pc = 0x0039_7894;
                }
                0x0039_7894 => {
                    c.cycles += 2;
                    c.pc = if (c.g[S1][0] as i64) <= 0 {
                        0x0039_78b4
                    } else {
                        0x0039_789c
                    };
                }
                0x0039_789c => {
                    c.cycles += 2;
                    let empty = c.g[S0][0] == 0;
                    c.g[V0][0] = 0;
                    c.pc = if empty { 0x0039_78d4 } else { 0x0039_78a4 };
                }
                0x0039_78a4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_78ac)?;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = lw_r(c, S0, 0x4);
                        if back_edge_unwinds(c, 0x0039_7890) {
                            return Ok(());
                        }
                        c.pc = 0x0039_7890;
                    } else {
                        c.pc = 0x0039_78b4;
                    }
                }
                0x0039_78b4 => {
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        c.g[V0][0] = 0;
                        c.pc = 0x0039_78d4;
                    } else {
                        c.pc = 0x0039_78bc;
                    }
                }
                0x0039_78bc => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    callee::UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_78c4)?;
                    c.g[V1][0] = c.g[V0][0];
                    let sentinel = c.g[V1][0] == 0;
                    c.g[V0][0] = c.g[S0][0];
                    if sentinel {
                        c.pc = 0x0039_78d4;
                    } else {
                        c.pc = 0x0039_78d0;
                    }
                }
                0x0039_78d0 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    c.pc = 0x0039_78d4;
                }
                0x0039_78d4 => {
                    c.cycles += 5;
                    c.lq(16, sp(c).wrapping_add(0x20));
                    c.lq(17, sp(c).wrapping_add(0x10));
                    c.g[RA][0] = Addr(sp(c)).ld(c);
                    let t = low(c, RA);
                    c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, 0x0039_7870, 0x0039_78e8);
                    return Ok(());
                }
            }
        }
    })
}

/// `void GameModule_dtor(self, flags)` (0x243838): install the module and its
/// embedded object vtables and free the object when `flags & 1`.
pub fn game_module_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[V0][0] = 0x48_0000;
        c.g[V1][0] = 0x47_0000;
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(0x2918));
        c.g[V1][0] = sx(low(c, V1).wrapping_sub(0x2690));
        sw_r(c, A0, 0xbc, c.g[V0][0]);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        sw_r(c, A0, 0, c.g[V1][0]);
        for off in [
            0x1f8u32, 0x1e8, 0x1c4, 0x1a8, 0x198, 0x184, 0x164, 0x154, 0x144, 0x134, 0x124, 0x10c,
            0xfc, 0xec, 0xdc,
        ] {
            sw_r(c, A0, off, c.g[V0][0]);
        }
        if c.g[A1][0] == 0 {
            c.cycles += 3;
        } else {
            c.cycles += 2;
            call_ptr(c, callee::BUILTIN_DELETE.addr, 0x0024_38a4)?;
            c.cycles += 3;
        }
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void GameViewMan_ResetCameras(self)` (0x22e7c8): call the reset method of
/// each active view.
pub fn game_view_man_reset_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.sq(17, sp(c).wrapping_add(0x20));
        c.sq(18, sp(c).wrapping_add(0x10));
        c.sq(16, sp(c).wrapping_add(0x30));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = lw_r(c, S2, 0x10);
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 3;
            c.g[S0][0] = sx(low(c, S2).wrapping_add(0x4));
            c.g[V1][0] = Addr(low(c, S0)).lw(c);
            loop {
                c.cycles += 12;
                c.g[S1][0] = sx(low(c, S1).wrapping_add(1));
                c.g[S0][0] = sx(low(c, S0).wrapping_add(4));
                c.g[A1][0] = lw_r(c, V1, 0xa0);
                c.g[V0][0] = lw_r(c, A1, 0x14);
                c.g[A0][0] = lh_r(c, V0, 0x20);
                c.g[V1][0] = lw_r(c, V0, 0x24);
                let target = low(c, V1);
                c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
                call_ptr(c, target, 0x0022_e818)?;
                c.g[V0][0] = lw_r(c, S2, 0x10);
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(low(c, S0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0022_e7f8);
                }
            }
        }
        c.cycles += 6;
        c.lq(16, sp(c).wrapping_add(0x30));
        c.lq(17, sp(c).wrapping_add(0x20));
        c.lq(18, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateBonusMaterial_OnWidgetCreated(self, menu)` (0x195540): map
/// the created menu's name hash to the bonus-material sub-page index.
pub fn c_fe_state_bonus_material_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.g[A0][0] = 0x4a_0000;
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x14f8));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.sq(16, sp(c).wrapping_add(0x20));
        c.g[S0][0] = lw_r(c, S1, 0x38);
        call_ptr(c, callee::STR_HASH, 0x0019_5564)?;
        if c.g[S0][0] == c.g[V0][0] {
            sw_r(c, S1, 0x18, 0);
        }
        c.cycles += 6;
        c.g[A0][0] = 0x4a_0000;
        c.g[S0][0] = lw_r(c, S1, 0x38);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x1500));
        call_ptr(c, callee::STR_HASH, 0x0019_557c)?;
        let differs = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x4a_0000;
        if !differs {
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            sw_r(c, S1, 0x18, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[S0][0] = lw_r(c, S1, 0x38);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x1508));
        call_ptr(c, callee::STR_HASH, 0x0019_5598)?;
        let differs = c.g[S0][0] != c.g[V0][0];
        c.lq(16, sp(c).wrapping_add(0x20));
        if !differs {
            c.cycles += 2;
            c.g[V0][0] = 0x2;
            sw_r(c, S1, 0x18, c.g[V0][0]);
        }
        c.cycles += 4;
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateTransition_onCreateScreen(self)` (0x194588): create the two
/// transition screens by name hash.
pub fn c_fe_state_transition_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.sq(16, sp(c).wrapping_add(0x20));
        c.g[A0][0] = 0x46_0000;
        c.g[S0][0] = lw_r(c, S1, 0x10);
        c.g[A0][0] = sx(low(c, A0).wrapping_sub(0x18));
        call_ptr(c, callee::STR_HASH, 0x0019_45ac)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = 0;
        call_ptr(c, callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME, 0x0019_45c0)?;
        c.g[A0][0] = 0x46_0000;
        c.g[S0][0] = lw_r(c, S1, 0x10);
        sw_r(c, S1, 0x48, c.g[V0][0]);
        c.g[A0][0] = sx(low(c, A0).wrapping_sub(0x8));
        call_ptr(c, callee::STR_HASH, 0x0019_45d4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = 0;
        call_ptr(c, callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME, 0x0019_45e8)?;
        sw_r(c, S1, 0x4c, c.g[V0][0]);
        c.lq(16, sp(c).wrapping_add(0x20));
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEAsyncManager_Load3PeakPic(self, slot, param)` (0x1a36e8): name the
/// slot's 0x11c-byte file entry, store the parameter and mark it loading.
pub fn fe_async_manager_request_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x50));
        c.g[A3][0] = 0x4a_0000;
        c.sq(16, sp(c).wrapping_add(0x40));
        c.g[A3][0] = sx(low(c, A3).wrapping_add(0x1a00));
        c.sq(18, sp(c).wrapping_add(0x20));
        c.g[S0][0] = 0x11c;
        c.g[S2][0] = c.g[A1][0];
        c.sq(19, sp(c).wrapping_add(0x10));
        c.mult(0, S0, S2, S0); // lo = slot * 0x11c
        c.sq(17, sp(c).wrapping_add(0x30));
        c.g[S3][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = Addr(gpo(c, -0x1700)).lw(c);
        c.g[A1][0] = 0x4a_0000;
        c.sd(sp(c), c.g[RA][0]);
        c.g[A1][0] = sx(low(c, A1).wrapping_add(0x19f8));
        c.g[A0][0] = sx(low(c, S1).wrapping_add(low(c, S0)));
        call_ptr(c, callee::SPRINTF, 0x001a_3730)?;
        c.g[S0][0] = sx(low(c, S1).wrapping_add(low(c, S0)));
        c.g[A0][0] = c.g[S1][0];
        sw_r(c, S0, 0x108, c.g[S3][0]);
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0x1;
        callee::FE_ASYNC_MANAGER_SET_FILE_STATUS.call(c, 0x001a_3748)?;
        c.lq(16, sp(c).wrapping_add(0x40));
        c.lq(17, sp(c).wrapping_add(0x30));
        c.lq(18, sp(c).wrapping_add(0x20));
        c.lq(19, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Differential tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, check_lohi, Rng, DATA, GAME_GP, RA as TEST_RA};

    const SEEDS: u32 = 24;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }
    fn h(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }
    fn b(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    /// Zero the words `machine` filled with junk below the stack pointer, for
    /// tests where a callee writes a value that decides control flow into a
    /// stack slot (the fakes do not).
    fn clear_stack(c: &mut Cpu) {
        for i in 0..0x50u32 {
            w(c, crate::difftest::SP - 0x140 + 4 * i, 0);
        }
    }

    /// The zeroed BE-library singleton at `gp-0x1f04` (else the FE helpers walk
    /// into the lazy disc load).
    fn be_library(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }

    /// The dummy world at `gp-0x848` with the race-manager chain, so rider
    /// loops terminate.
    fn world(c: &mut Cpu) {
        let wld = DATA + 0x1000;
        w(c, GAME_GP.wrapping_sub(0x848), wld);
        w(c, wld + 0x84, DATA + 0x1200);
        w(c, DATA + 0x1200 + 0xc, DATA + 0x1400);
        w(c, DATA + 0x1400 + 0x7c, 0);
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, 0x40000)`: the game heap over test
    /// memory, for the paths that allocate.
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = TEST_RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, TEST_RA as u32, "DLHeap_InitArena did not return");
    }

    /// A NUL-terminated string at `a`.
    fn string(c: &mut Cpu, a: u32, s: &str) {
        for (i, ch) in s.bytes().enumerate() {
            b(c, a + i as u32, ch);
        }
        b(c, a + s.len() as u32, 0);
    }

    /// A NUL-terminated UTF-16 string at `a` (for the wide-char memcard name).
    fn wide_string(c: &mut Cpu, a: u32, s: &str) {
        for (i, ch) in s.encode_utf16().enumerate() {
            h(c, a + 2 * i as u32, ch);
        }
        h(c, a + 2 * s.encode_utf16().count() as u32, 0);
    }

    /// `{delta, fn}` at `vt + off`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16, f: u32) {
        h(c, vt + off, delta as u16);
        w(c, vt + off + 4, f);
    }

    /// Run a guest helper during setup; returns its `v0`.
    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
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
    fn str_hash(c: &mut Cpu, s: u32) -> u32 {
        run(c, 0x0031_7670, ee::s10::f_00317670, &[s as u64]) as u32
    }

    #[test]
    fn script_engine_on_seq_advance_matches() {
        check(ee::s07::f_0027a720__generated, script_engine_on_seq_advance, SEEDS, &|c, seed| {
            machine(c, seed, 0x27a720, 0);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 3) as u64;
            w(c, obj + 0x550, if seed % 3 == 0 { 2 } else { 1 });
            w(c, obj + 0x54c, DATA + 0x1000);
        });
    }

    #[test]
    fn music_player_enable_song_matches() {
        check(ee::s08::f_002b3fe8__generated, music_player_enable_song, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2b3fe8, 0);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 40) as u64;
            w(c, obj + 0x3e8, seed % 70);
            w(c, obj + 0x3f8, r.next_u32());
            w(c, obj + 0x3fc, r.next_u32());
            for i in 0..5u32 {
                w(c, obj + 0x420 + 4 * i, if (seed + i) % 3 == 0 { 0 } else { 1 + i });
            }
            for i in 0..5u32 {
                w(c, obj + 0x920 + 4 * i, r.next_u32() % 7);
            }
        });
    }

    #[test]
    fn menu_sys_float_range_item_draw_matches() {
        check(ee::s09::f_002ce3a0__generated, menu_sys_float_range_item_draw, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2ce3a0, 3);
            be_library(c);
            let item = DATA;
            let row = DATA + 0x800;
            c.g[A0][0] = item as u64;
            c.g[A1][0] = row as u64;
            w(c, item, 0);
            w(c, item + 0x18, DATA + 0x100);
            w(c, DATA + 0x100, r.edge_f32());
            w(c, item + 0x14, r.next_u32());
        });
    }

    #[test]
    fn replay_seek_to_end_matches() {
        check(ee::s07::f_00270378__generated, replay_seek_to_end, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x270378, 2);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            w(c, obj + 0x48c, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
            w(c, obj + 0x490, if seed % 4 == 0 { 0 } else { DATA + 0x2000 });
            w(c, obj + 0x3b0, 0);
            w(c, obj + 0x484, r.next_u32() % 0x1000);
        });
    }

    #[test]
    fn replay_check_instant_replay_matches() {
        check(ee::s07::f_002705b0__generated, replay_check_instant_replay, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2705b0, 1);
            be_library(c);
            let obj = DATA;
            let alt = DATA + 0x800;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = alt as u64;
            c.g[A2][0] = (seed % 2) as u64;
            w(c, obj + 0x3d4, if seed % 3 == 0 { 0 } else { 1 });
            let wld = DATA + 0x1000;
            let race = DATA + 0x1200;
            let mgr = DATA + 0x1400;
            w(c, GAME_GP.wrapping_sub(0x848), wld);
            w(c, wld + 0x84, race);
            w(c, race + 0xc, mgr);
            w(c, mgr + 0x7c, if seed % 4 == 0 { 2 } else { 1 });
            w(c, alt + 0x9c, r.next_u32() % 0x1000);
            w(c, alt + 0xa4, r.next_u32() % 0x1000);
            w(c, obj + 0x474, r.next_u32() % 0x1000);
            w(c, obj + 0x47c, r.next_u32() % 0x1000);
        });
    }

    #[test]
    fn ai_get_sync_checksum_matches() {
        check(ee::s00::f_0012b420__generated, ai_get_sync_checksum, SEEDS, &|c, seed| {
            machine(c, seed, 0x12b420, 0);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            let n = seed % 4;
            w(c, obj + 0x78, n);
            for i in 0..n {
                w(c, obj + 0x28 + 4 * i, DATA + 0x1000 + 0x40 * i);
                w(c, DATA + 0x1000 + 0x40 * i + 4, seed + i);
            }
        });
    }

    #[test]
    fn cps2_mem_card_v37_matches() {
        check(ee::s08::f_002c45d0__generated, cps2_mem_card_v37, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2c45d0, 3);
            be_library(c);
            let obj = DATA;
            let name = DATA + 0x800;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = name as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
            wide_string(c, name, if seed % 2 == 0 { "SAVE01" } else { "" });
        });
    }

    #[test]
    fn cps2_mem_card_v36_matches() {
        check(ee::s08::f_002c4648__generated, cps2_mem_card_v36, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2c4648, 3);
            be_library(c);
            let obj = DATA;
            let name = DATA + 0x800;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = name as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
            string(c, name, if seed % 2 == 0 { "SAVE01" } else { "" });
        });
    }

    #[test]
    fn cps2_mem_card_request_read_save_matches() {
        check(ee::s08::f_002c4788__generated, cps2_mem_card_request_read_save, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2c4788, 3);
            be_library(c);
            let obj = DATA;
            let name = DATA + 0x800;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = name as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
            string(c, name, if seed % 2 == 0 { "SLOT-A" } else { "" });
        });
    }

    #[test]
    fn ui_list_dtor_matches() {
        check(ee::s13::f_003a4888__generated, ui_list_dtor, SEEDS, &|c, seed| {
            machine(c, seed, 0x3a4888, 4);
            be_library(c);
            let list = DATA;
            c.g[A0][0] = list as u64;
            c.g[A1][0] = (seed % 2) as u64;
            let s = DATA + 0x2000;
            if seed % 3 == 0 {
                w(c, list + 4, s);
                w(c, s + 4, s);
                w(c, s, s);
            } else {
                // head -> a -> terminator (self-linked at +4)
                let a = DATA + 0x2100;
                w(c, list + 4, a);
                w(c, a + 4, s);
                w(c, a, 0);
                w(c, s + 4, s);
                w(c, s, 0);
                let vt = DATA + 0x3000;
                w(c, a + 8, vt);
                slot(c, vt, 8, 0x20, FAKE);
            }
        });
    }

    #[test]
    fn boader_shadow_dtor_matches() {
        check(ee::s12::f_00375918__generated, boader_shadow_dtor, SEEDS, &|c, seed| {
            machine(c, seed, 0x375918, 2);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            w(c, obj + 0x170, if seed % 3 == 0 { 0xffff_ffff } else { 5 });
            let mgr = DATA + 0x2000;
            w(c, GAME_GP.wrapping_add(0x2a90), mgr);
            w(c, mgr + 0x18f4, DATA + 0x2100);
            let vt = DATA + 0x2200;
            w(c, mgr + 0x10d8, vt);
            slot(c, vt, 0x190, 0x30, FAKE);
        });
    }

    #[test]
    fn wscript_man_define_collect_set_matches() {
        check(ee::s10::f_0030b928__generated, wscript_man_define_collect_set, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x30b928, 1);
            be_library(c);
            let obj = DATA;
            let slotaddr = obj + 0x2c0;
            let mut id = r.next_u32() % 8;
            if seed % 3 == 1 {
                id = 0xffff_ffff;
            }
            if seed % 3 == 2 {
                id = 0x55;
            }
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = if seed % 3 == 0 { id as u64 } else { (r.next_u32() % 8) as u64 };
            c.g[A2][0] = r.next_u32() as u64;
            w(c, slotaddr, id);
            w(c, slotaddr + 4, 0);
        });
    }

    #[test]
    fn rider_model_alloc_model_refs_matches() {
        check_lohi(ee::s10::f_0030d840__generated, rider_model_alloc_model_refs, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x30d840, 2);
            be_library(c);
            heap_init(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

        #[test]
    fn magnet_modifier_ctor_matches() {
        check(ee::s11::f_003572e0__generated, magnet_modifier_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3572e0, 0);
            be_library(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            c.g[A2][0] = (DATA + 0x1000) as u64;
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            for i in 0..16u32 {
                w(c, DATA + 0x1000 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn multi_particle_man_dtor_matches() {
        check(ee::s11::f_00357a00__generated, multi_particle_man_dtor, SEEDS, &|c, seed| {
            machine(c, seed, 0x357a00, 1);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            for i in 0..4u32 {
                let p = DATA + 0x1000 + 0x100 * i;
                let present = (seed >> i) & 1 != 0;
                w(c, obj + 4 * i, if present { p } else { 0 });
                if present {
                    w(c, p, 0);
                }
            }
        });
    }

    #[test]
    fn c_fe_state_reward_gallery_update_count_text_matches() {
        check(ee::s04::f_001d0eb8__generated, c_fe_state_reward_gallery_update_count_text, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1d0eb8, 4);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            w(c, obj + 0x918, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
            w(c, obj + 0x914, if seed % 4 < 2 { 0 } else { DATA + 0x1100 });
            w(c, obj + 0x6c, r.next_u32());
            w(c, obj + 0x68, r.next_u32());
            for a in [DATA + 0x1000, DATA + 0x1100] {
                w(c, a, 0);
                w(c, a + 8, 0);
            }
        });
    }

    #[test]
    fn player_get_uber_grind_button_matches() {
        check(ee::s00::f_00127848__generated, player_get_uber_grind_button, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x127848, 4);
            be_library(c);
            let obj = DATA;
            let map = DATA + 0x1000;
            c.g[A0][0] = obj as u64;
            w(c, obj + 0xdf0, map);
            w(c, map, 0);
            w(c, map + 4, 0);
            w(c, map + 0x14, r.next_u32() % 4);
        });
    }

    #[test]
    fn wscript_task_update_matches() {
        check(ee::s10::f_00308328__generated, wscript_task_update, SEEDS, &|c, seed| {
            machine(c, seed, 0x308328, 1);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            w(c, obj, (seed % 4) as u32);
            let vt = DATA + 0x3000;
            w(c, obj + 0x40, vt);
            slot(c, vt, 0x28, 0x40, FAKE);
        });
    }

    #[test]
    fn c_fe_state_options_save_load_on_render_matches() {
        check(ee::s02::f_0018d3c0__generated, c_fe_state_options_save_load_on_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x18d3c0, 2);
            be_library(c);
            let h0 = str_hash(c, 0x0045_e0f0);
            let h1 = str_hash(c, 0x0045_e100);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            w(c, GAME_GP.wrapping_sub(0x1718), if seed % 2 == 0 { 0 } else { 1 });
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
        });
    }

    #[test]
    fn ov_screen_be_options_on_widget_notify_matches() {
        check(ee::s04::f_001faa78__generated, ov_screen_be_options_on_widget_notify, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1faa78, 2);
            be_library(c);
            let hash = str_hash(c, 0x004a_24e8);
            let menu = DATA;
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = menu as u64;
            c.g[A2][0] = (seed % 12) as u64;
            c.g[A3][0] = (seed % 4) as u64;
            let s = DATA + 0x2000;
            let a = DATA + 0x2100;
            w(c, menu + 0x74 + 4, a);
            w(c, a + 4, s);
            w(c, a, 0);
            w(c, s + 4, s);
            w(c, s, 0);
            w(c, a + 0x38, if seed % 3 == 0 { hash } else { r.next_u32() });
        });
    }

    #[test]
    fn be_reward_int_grant_medal_reward_matches() {
        check(ee::s01::f_00159170__generated, be_reward_int_grant_medal_reward, SEEDS, &|c, seed| {
            machine(c, seed, 0x159170, 3);
            be_library(c);
            world(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 8) as u64;
            c.g[A2][0] = (seed % 5) as u64;
            c.g[A3][0] = (seed % 3) as u64;
            for i in 0..0x400u32 {
                w(c, 0x004a_6ca8 + 4 * i, (seed.wrapping_mul(2654435761) ^ i) & 0x3);
            }
        });
    }

    #[test]
    fn rider_metrix_reset_matches() {
        check(ee::s00::f_00117540__generated, rider_metrix_reset, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x117540, 6);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            w(c, obj + 0x114, 0);
            w(c, obj + 0x118, 0);
            for i in 0..16u32 {
                w(c, obj + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn rider_metrix_on_rider_reset_matches() {
        check(ee::s00::f_00119368__generated, rider_metrix_on_rider_reset, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x119368, 3);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            w(c, obj + 0x1ac, DATA + 0x1000);
            w(c, DATA + 0x1000 + 0x2f8, r.edge_f32());
        });
    }

    #[test]
    fn be_new_player_int_get_char_state_flag_matches() {
        check(ee::s01::f_00145f90__generated, be_new_player_int_get_char_state_flag, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x145f90, 0);
            be_library(c);
            let a = (seed % 4) as u32;
            let bb = (seed % 3) as u32;
            let peak = (seed % 3) as u64;
            c.g[A1][0] = a as u64;
            c.g[A2][0] = bb as u64;
            c.g[A3][0] = peak;
            let addr = 0x004a_6ca8u32
                .wrapping_add(bb.wrapping_mul(0xf88))
                .wrapping_add(a.wrapping_mul(0x9b50))
                .wrapping_add(0x278);
            w(c, addr, r.next_u32());
            w(c, addr + 4, r.next_u32());
        });
    }

    #[test]
    fn be_new_player_int_get_char_record_field_27c_matches() {
        check(ee::s01::f_00146d98__generated, be_new_player_int_get_char_record_field_27c, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x146d98, 2);
            be_library(c);
            c.g[A1][0] = (seed % 3) as u64;
            for i in 0..4u32 {
                w(c, 0x0053_05b0 + 4 * i, r.next_u32() % 4);
                w(c, 0x0053_5b20 + 4 * i, 0);
            }
            for i in 0..0x200u32 {
                w(c, 0x004a_6ca8 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn be_new_player_int_get_inbox_matches() {
        check(ee::s01::f_00147908__generated, be_new_player_int_get_inbox, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x147908, 2);
            be_library(c);
            c.g[A1][0] = (seed % 3) as u64;
            for i in 0..4u32 {
                w(c, 0x0053_05b0 + 4 * i, r.next_u32() % 4);
                w(c, 0x0053_5b20 + 4 * i, 0);
            }
        });
    }

    #[test]
    fn boost_modifier_ctor_2_matches() {
        check(ee::s11::f_00342808__generated, boost_modifier_ctor_2, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x342808, 2);
            be_library(c);
            let obj = DATA;
            let stream = DATA + 0x800;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = stream as u64;
            let vt = DATA + 0x3000;
            w(c, stream, vt);
            slot(c, vt, 0x10, 0x10, FAKE);
            slot(c, vt, 0x18, 0x18, FAKE + 0x40);
            w(c, obj + 0x40, r.next_u32());
        });
    }

    #[test]
    fn game_mode_man_destroy_matches() {
        check(ee::s06::f_002380e8__generated, game_mode_man_destroy, SEEDS, &|c, seed| {
            machine(c, seed, 0x2380e8, 10);
            be_library(c);
            for i in 0..10u32 {
                let p = DATA + 0x1000 + 0x100 * i;
                let present = (seed >> i) & 1 != 0;
                w(c, 0x0053_6668 + 4 * i, if present { p } else { 0 });
                if present {
                    let vt = DATA + 0x3000;
                    w(c, p, vt);
                    slot(c, vt, 8, 0x20, FAKE);
                }
            }
            w(c, GAME_GP.wrapping_sub(0x484), if seed % 2 == 0 { 0 } else { DATA + 0x4000 });
        });
    }

    #[test]
    fn gm_handler_clear_standings_matches() {
        check(ee::s06::f_00238d30__generated, gm_handler_clear_standings, SEEDS, &|c, seed| {
            machine(c, seed, 0x238d30, 5);
            be_library(c);
            for i in 0..0x40u32 {
                w(c, 0x0053_6640 + 4 * i, seed ^ i);
            }
        });
    }

    #[test]
    fn gm_handler_clear_standings2_matches() {
        check(ee::s06::f_00238da8__generated, gm_handler_clear_standings2, SEEDS, &|c, seed| {
            machine(c, seed, 0x238da8, 5);
            be_library(c);
            for i in 0..0x40u32 {
                w(c, 0x0053_6640 + 4 * i, seed ^ i);
            }
        });
    }

    #[test]
    fn cbx_string_alloc_copy_matches() {
        check(ee::s10::f_003181e8__generated, cbx_string_alloc_copy, SEEDS, &|c, seed| {
            machine(c, seed, 0x3181e8, 2);
            be_library(c);
            heap_init(c);
            let dst = DATA;
            let src = DATA + 0x800;
            c.g[A0][0] = dst as u64;
            c.g[A1][0] = src as u64;
            c.g[A2][0] = (seed % 5) as u64;
            c.g[A3][0] = (seed % 3) as u64;
            c.g[T0][0] = if seed % 4 == 0 { 0 } else { 1 };
            w(c, src, DATA + 0x1000);
            string(c, DATA + 0x1000, "abcdefgh");
        });
    }

    #[test]
    fn cbx_string_dtor_matches() {
        check(ee::s10::f_00318260__generated, cbx_string_dtor, SEEDS, &|c, seed| {
            machine(c, seed, 0x318260, 2);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            if seed % 4 == 0 {
                w(c, obj, 0x0048_dcd4);
            } else {
                let buf = DATA + 0x1000;
                w(c, obj, buf);
                w(c, buf - 0xc, if seed % 4 == 1 { 1 } else { 2 + seed % 3 });
            }
        });
    }

    #[test]
    fn cbx_string_ctor_cstr_matches() {
        check(ee::s10::f_003182d8__generated, cbx_string_ctor_cstr, SEEDS, &|c, seed| {
            machine(c, seed, 0x3182d8, 2);
            be_library(c);
            heap_init(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { (DATA + 0x1000) as u64 };
            string(c, DATA + 0x1000, if seed % 2 == 0 { "hello" } else { "" });
        });
    }

    #[test]
    fn debounce_ctor_matches() {
        check(ee::s11::f_00342d10__generated, debounce_ctor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x342d10, 3);
            be_library(c);
            clear_stack(c);
            let obj = DATA;
            let stream = DATA + 0x800;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = stream as u64;
            let vt = DATA + 0x3000;
            w(c, stream, vt);
            slot(c, vt, 0x10, 0x10, FAKE);
        });
    }

    #[test]
    fn c_fe_state_rewards_room_init_matches() {
        check(ee::s04::f_001d4c60__generated, c_fe_state_rewards_room_init, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0x1d4c60, 4);
            be_library(c);
            heap_init(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            w(c, obj + 0x10, DATA + 0x1000);
            w(c, DATA + 0x1000, 0);
        });
    }

    #[test]
    fn tag_field_get_address_matches() {
        check(ee::s14::f_003ecff8__generated, tag_field_get_address, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3ecff8, 0);
            be_library(c);
            c.pc = 0x003e_cff8;
            c.g[A1][0] = r.next_u32() as u64;
            let s = DATA + 0x200;
            if seed % 4 == 0 {
                c.g[A0][0] = 0;
            } else {
                c.g[A0][0] = s as u64;
                let n = 1 + seed % 6;
                let chars = *b"0129.x ";
                for i in 0..n {
                    b(c, s + i, chars[((seed + i) % 7) as usize]);
                }
                b(c, s + n, 0);
            }
        });
    }

        #[test]
    fn cui_text_scroll_set_text_id_matches() {
        check(ee::s13::f_003a1f18__generated, cui_text_scroll_set_text_id, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3a1f18, 3);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = r.next_u32() as u64;
            w(c, obj + 0x5c, DATA + 0x1000);
            w(c, DATA + 0x1000 + 0xd0, DATA + 0x1100);
            w(c, DATA + 0x1100 + 0x10, DATA + 0x1200);
            let present = seed % 2 == 0;
            w(c, DATA + 0x1200 + 0x10, if present { DATA + 0x1300 } else { 0 });
            let vt = DATA + 0x3000;
            w(c, DATA + 0x1300 + 4, vt);
            slot(c, vt, 0x20, 0x30, FAKE);
            w(c, obj + 8, vt);
            slot(c, vt, 0x28, 0x40, FAKE + 0x40);
        });
    }

    #[test]
    fn cui_group_set_focus_matches() {
        check(ee::s13::f_00398638__generated, cui_group_set_focus, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x398638, 1);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            if seed % 2 == 0 {
                w(c, obj + 0x74 + 4, 0);
            } else {
                let s = DATA + 0x2000;
                let a = DATA + 0x2100;
                w(c, obj + 0x74 + 4, a);
                w(c, a + 4, s);
                w(c, a, 0);
                w(c, s + 4, s);
                w(c, s, 0);
            }
            w(c, obj + 0x14, r.next_u32());
        });
    }

    #[test]
    fn flex_bridge_springs_init_matches() {
        check(ee::s11::f_00353e08__generated, flex_bridge_springs_init, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x353e08, 1);
            be_library(c);
            heap_init(c);
            let desc = DATA + 0x800;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = desc as u64;
            c.f[12] = r.edge_f32();
            for i in 0..7u32 {
                w(c, desc + 4 * i, r.edge_f32());
            }
            w(c, desc + 0x1c, (2 + seed % 4) as u32);
        });
    }

    #[test]
    fn ov_screen_session_load_map_image_matches() {
        check(ee::s05::f_00208f10__generated, ov_screen_session_load_map_image, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x208f10, 4);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            w(c, obj + 0xc4, r.next_u32() % 4);
            let wld = DATA + 0x1000;
            w(c, GAME_GP.wrapping_sub(0x848), wld);
            w(c, wld + 0x11c, DATA + 0x2000);
            w(c, DATA + 0x2000, 0);
            w(c, DATA + 0x30_0000, 0);
        });
    }

        #[test]
    fn c_stream_man_dtor_matches() {
        check(ee::s13::f_003a6e20__generated, c_stream_man_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3a6e20, 4);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            w(c, obj + 0xcc, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
            w(c, obj + 0xc0, if seed % 3 == 0 { 0 } else { 1 });
            b(c, obj + 0xbc, if seed % 4 == 0 { 0xff } else { r.next_u32() as u8 & 0xfe });
        });
    }

    #[test]
    fn ui_list_get_at_matches() {
        check(ee::s13::f_00397870__generated, ui_list_get_at, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0x397870, 2);
            be_library(c);
            c.pc = 0x0039_7870;
            let list = DATA;
            c.g[A0][0] = list as u64;
            c.g[A1][0] = (seed % 5) as u64;
            let s = DATA + 0x2000;
            if seed % 4 == 0 {
                w(c, list + 4, s);
                w(c, s + 4, s);
                w(c, s, s);
            } else {
                // head -> a -> b -> terminator (self-linked at +4)
                let a = DATA + 0x2100;
                let bb = DATA + 0x2200;
                w(c, list + 4, a);
                w(c, a + 4, bb);
                w(c, a, 0);
                w(c, bb + 4, s);
                w(c, bb, 0);
                w(c, s + 4, s);
                w(c, s, 0);
            }
        });
    }

    #[test]
    fn game_module_dtor_matches() {
        check(ee::s06::f_00243838__generated, game_module_dtor, SEEDS, &|c, seed| {
            machine(c, seed, 0x243838, 2);
            be_library(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn game_view_man_reset_cameras_matches() {
        check(ee::s05::f_0022e7c8__generated, game_view_man_reset_cameras, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x22e7c8, 1);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            let n = seed % 3;
            w(c, obj + 0x10, n);
            for i in 0..n {
                let v = DATA + 0x1000 + 0x100 * i;
                let o = DATA + 0x1400 + 0x100 * i;
                w(c, obj + 4 + 4 * i, v);
                w(c, v + 0xa0, o);
                w(c, o + 0x14, DATA + 0x3000);
                slot(c, DATA + 0x3000, 0x20, 0x30, FAKE + 0x40 * i);
                w(c, v + 4, r.next_u32());
            }
        });
    }

    #[test]
    fn c_fe_state_bonus_material_on_widget_created_matches() {
        check(ee::s02::f_00195540__generated, c_fe_state_bonus_material_on_widget_created, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x195540, 3);
            be_library(c);
            let h0 = str_hash(c, 0x004a_14f8);
            let h1 = str_hash(c, 0x004a_1500);
            let h2 = str_hash(c, 0x004a_1508);
            let menu = DATA + 0x800;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = menu as u64;
            let hash = match seed % 4 {
                0 => h0,
                1 => h1,
                2 => h2,
                _ => r.next_u32(),
            };
            w(c, menu + 0x38, hash);
            w(c, menu + 0x18, r.next_u32());
        });
    }

    #[test]
    fn c_fe_state_transition_init_matches() {
        check(ee::s02::f_00194588__generated, c_fe_state_transition_init, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0x194588, 2);
            be_library(c);
            heap_init(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            w(c, obj + 0x10, DATA + 0x1000);
            w(c, DATA + 0x1000, 0);
        });
    }

    #[test]
    fn fe_async_manager_request_texture_matches() {
        check(ee::s03::f_001a36e8__generated, fe_async_manager_request_texture, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1a36e8, 2);
            be_library(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = r.next_u32() as u64;
            w(c, GAME_GP.wrapping_sub(0x1700), DATA + 0x2000);
            string(c, DATA + 0x2000, "pic");
        });
    }
}
