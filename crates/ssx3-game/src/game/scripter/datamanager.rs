//! `src/scripter/datamanager.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_2;
pub(crate) use crate::game::dirtysock::tagsunk::vslot;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::oc432_gp_minus;
pub(crate) use crate::game::main::ssxapp::oc429_at;
pub(crate) use crate::game::movie::movieplayer::at as at_3;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::sound::bankmonitor::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_5828, data_instance_man_ctor, "DataInstanceMan_ctor"),
    (0x0027_58c0, data_instance_man_dtor, "DataInstanceMan_dtor"),
    (
        0x0027_5b08,
        data_instance_man_remove_all,
        "DataInstanceMan_RemoveAll",
    ),
    (
        0x0027_5b98,
        data_instance_man_add_big_entries,
        "DataInstanceMan_AddBigEntries",
    ),
    (
        0x0027_5cd0,
        data_instance_man_free_info,
        "DataInstanceMan_FreeInfo",
    ),
    (
        0x0027_5cf8,
        data_instance_man_alloc_info,
        "DataInstanceMan_AllocInfo",
    ),
    (0x0027_5d10, script_list_ctor, "ScriptList_ctor"),
    (0x0027_5d90, script_list_dtor, "ScriptList_dtor"),
    (0x0027_5dd8, scriptlist_add, "ScriptList_Add"),
    (
        0x0027_6998,
        script_list_get_entry_type,
        "ScriptList_GetEntryType",
    ),
    (0x0027_6b98, script_list_get_state, "ScriptList_GetState"),
    (0x0027_6cc8, script_list_update, "ScriptList_Update"),
    (0x0027_7060, script_list_can_skip, "ScriptList_CanSkip"),
    (
        0x0027_70c0,
        script_list_is_current_skippable,
        "ScriptList_IsCurrentSkippable",
    ),
    (
        0x0027_71c8,
        scriptlist_is_playing_script,
        "ScriptList_IsPlayingScript",
    ),
    (0x0027_7298, script_list_is_paused, "ScriptList_IsPaused"),
    (
        0x0027_72c0,
        script_list_on_seq_start,
        "ScriptList_OnSeqStart",
    ),
    (
        0x0027_7310,
        script_list_on_seq_advance,
        "ScriptList_OnSeqAdvance",
    ),
    (0x0027_73a0, script_list_on_seq_end, "ScriptList_OnSeqEnd"),
    (
        0x0027_7400,
        script_list_on_seq_stopped,
        "ScriptList_OnSeqStopped",
    ),
    (
        0x0027_76e0,
        script_list_check_next_ready,
        "ScriptList_CheckNextReady",
    ),
    (
        0x0027_7778,
        script_list_poll_loading,
        "ScriptList_PollLoading",
    ),
    (
        0x0027_7dd8,
        script_list_is_movie_entry,
        "ScriptList_IsMovieEntry",
    ),
    (0x0027_7f08, script_bank_slot_clear, "ScriptBankSlot_Clear"),
    (0x0027_80b8, script_engine_dtor, "ScriptEngine_dtor"),
    (
        0x0027_8308,
        script_engine_unload_master,
        "ScriptEngine_UnloadMaster",
    ),
    (0x0028_3818, script_movie_stop, "ScriptMovie_Stop"),
    (
        0x0027_6bd8,
        script_list_get_play_status,
        "ScriptList_GetPlayStatus",
    ),
    (0x0027_6388, script_list_start, "ScriptList_Start"),
    (
        0x0027_7450,
        script_list_prepare_entry,
        "ScriptList_PrepareEntry",
    ),
    (
        0x0027_7598,
        script_list_pop_entries,
        "ScriptList_PopEntries",
    ),
    (
        0x0027_5920,
        data_instance_man_add_data,
        "DataInstanceMan_AddData",
    ),
    (0x0027_5ed0, script_list_add_choice, "ScriptList_AddChoice"),
    (0x0027_6868, script_list_stop, "ScriptList_Stop"),
    (0x0027_7f20, script_engine_ctor, "ScriptEngine_ctor"),
    (
        0x0027_5a20,
        data_instance_man_remove_data,
        "DataInstanceMan_RemoveData",
    ),
    (0x0027_6270, script_list_prepare, "ScriptList_Prepare"),
    (
        0x0027_6f48,
        script_list_skip_current,
        "ScriptList_SkipCurrent",
    ),
    (0x0027_7838, script_list_next_entry, "ScriptList_NextEntry"),
    (
        0x0027_8210,
        script_engine_load_master,
        "ScriptEngine_LoadMaster",
    ),
    (
        0x0027_72b8,
        script_list_get_field_b0,
        "ScriptList_GetFieldB0",
    ),
    (
        0x0027_7800,
        script_list_try_advance,
        "ScriptList_TryAdvance",
    ),
    (0x0027_6048, script_list_add_movie, "ScriptList_AddMovie"),
    (0x0027_6198, script_list_take_from, "ScriptList_TakeFrom"),
    (
        0x0027_66b0,
        script_list_set_paused_nested,
        "ScriptList_SetPausedNested",
    ),
    (
        0x0027_6a08,
        script_list_get_entry_ext_type,
        "ScriptList_GetEntryExtType",
    ),
    (
        0x0027_6a78,
        script_list_get_entry_script_index,
        "ScriptList_GetEntryScriptIndex",
    ),
    (
        0x0027_6b08,
        script_list_get_entry_script,
        "ScriptList_GetEntryScript",
    ),
    (0x0027_6ca8, script_list_count, "ScriptList_Count"),
    (
        0x0027_7100,
        script_list_is_next_ready,
        "ScriptList_IsNextReady",
    ),
    (0x0027_71a8, script_list_is_full, "ScriptList_IsFull"),
    (
        0x0027_7248,
        script_list_is_playing_movie,
        "ScriptList_IsPlayingMovie",
    ),
    (
        0x0027_7980,
        script_list_start_transition,
        "ScriptList_StartTransition",
    ),
    (0x0027_7c08, script_list_set_paused, "ScriptList_SetPaused"),
    (
        0x0027_7de8,
        script_list_get_movie_info,
        "ScriptList_GetMovieInfo",
    ),
    (0x0027_66d0, script_list_skip_2, "ScriptList_Skip"),
    (
        0x0028_47a8,
        data_instance_man_on_data_added_002847a8,
        "DataInstanceMan_OnDataAdded",
    ),
    (
        0x0028_47b0,
        data_instance_man_on_data_removed_002847b0,
        "DataInstanceMan_OnDataRemoved",
    ),
    (
        0x0028_47b8,
        data_instance_man_v04_002847b8,
        "DataInstanceMan_v04",
    ),
    (
        0x0028_47c0,
        data_instance_man_v05_002847c0,
        "DataInstanceMan_v05",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MESSAGE_CENTER_MESSAGE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;

    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;

    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4;

    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_STOP;

    pub(crate) use crate::game::fe::fepopup::callee::C002758C0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275B98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275CF8;

    pub(crate) use crate::game::fe::fepopup::callee::C00281ED8;
    pub(crate) use crate::game::fe::fepopup::callee::C002837C8;

    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::SCRIPT_CMD_QUEUE_AT;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::SCRIPT_LIST_POLL_LOADING;

    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2;

    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;

    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
}

pub(crate) mod addr {

    pub(crate) use crate::game::bx::bxstringctor::addr::MEM_MAN_FREE;
    pub(crate) use crate::game::bx::bxstringctor::addr::MEM_MAN_FREE_NOSIZE;
}

// ---------------------------------------------------------------------------
// Data instance manager.

/// `void DataInstanceMan_FreeInfo(DataInstanceMan *man, DataInfo *info)`
/// (0x275cd0): push `info` onto the free list `man->free` (+0x8), clearing the
/// record's fields and threading the old head through `info + 0x14`.
pub fn data_instance_man_free_info(c: &mut Cpu) {
    c.cycles += 9;
    const FREE: u32 = 0x8;
    const NEXT: u32 = 0x14;
    let man = Addr(abi::arg(c, 0));
    let info = Addr(abi::arg(c, 1));
    info.off(0x8).sw(c, 0);
    info.off(0xc).sw(c, 0);
    info.off(0x10).sw(c, 0);
    info.sw(c, 0);
    info.off(0x4).sw(c, 0);
    let head = man.off(FREE).lw(c);
    c.g[V0][0] = head;
    info.off(NEXT).sw(c, head);
    man.off(FREE).sw(c, abi::arg64(c, 1));
    abi::ret(c);
}

/// `DataInfo *DataInstanceMan_AllocInfo(DataInstanceMan *man)` (0x275cf8):
/// pop the free list `man->free` (+0x8), returning the node and clearing its
/// next pointer (+0x14).
pub fn data_instance_man_alloc_info(c: &mut Cpu) {
    c.cycles += 6;
    const FREE: u32 = 0x8;
    const NEXT: u32 = 0x14;
    let man = Addr(abi::arg(c, 0));
    let info = man.off(FREE).lw(c);
    c.g[V1][0] = info;
    let next = Addr(info as u32).off(NEXT).lw(c);
    c.g[A1][0] = next;
    c.g[V0][0] = info;
    man.off(FREE).sw(c, next);
    Addr(info as u32).off(NEXT).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script lists.

/// `bool ScriptList_IsPaused(ScriptList *list)` (0x277298): true when the state
/// at +0xa4 is 3 and the pending count at +0xa8 is non-zero.
pub fn script_list_is_paused(c: &mut Cpu) {
    c.cycles += 4;
    const STATE: u32 = 0xa4;
    const PENDING: u32 = 0xa8;
    const PAUSED: u64 = 3;
    let list = Addr(abi::arg(c, 0));
    c.g[V1][0] = list.off(STATE).lw(c);
    c.g[V0][0] = PAUSED;
    let mut result = 0u64;
    if c.g[V1][0] == PAUSED {
        c.cycles += 2;
        let pending = list.off(PENDING).lw(c);
        c.g[V0][0] = pending;
        result = (0u64 < pending) as u64;
    }
    c.g[A1][0] = result;
    c.cycles += 2;
    c.g[V0][0] = result;
    abi::ret(c);
}

/// `bool ScriptList_IsMovieEntry(int *entry)` (0x277dd8): true when the signed
/// word at `entry` is at least 29 (i.e. not a movie entry when below 29).
pub fn script_list_is_movie_entry(c: &mut Cpu) {
    c.cycles += 4;
    let value = Addr(abi::arg(c, 1)).lw(c);
    c.g[V0][0] = ((value as i64) < 29) as u64;
    c.g[V0][0] ^= 0x1;
    abi::ret(c);
}

/// `void ScriptBankSlot_Clear(ScriptBankSlot *slot)` (0x277f08): reset a bank
/// slot: id -1 at +0x4, fields +0x0/+0x8/+0xc cleared.
pub fn script_bank_slot_clear(c: &mut Cpu) {
    c.cycles += 6;
    let slot = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    slot.off(0xc).sw(c, 0);
    slot.off(0x4).sw(c, c.g[V0][0]);
    slot.sw(c, 0);
    slot.off(0x8).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script engine.

/// `int ScriptList_GetState(ScriptList* self)` (0x276B98): while the list is
/// loading (state +0xA4 == 1) poll the load, then return the state.
pub fn script_list_get_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let state = c.lw_r_2(V0, r(c, S0).wrapping_add(0xa4));
        if state == c.g[V1][0] {
            c.cycles += 4;
            callee::SCRIPT_LIST_POLL_LOADING.call(c, 0x0027_6bc0)?;
            let state = c.lw_r_2(V0, r(c, S0).wrapping_add(0xa4));
            let _ = state;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptList_IsCurrentSkippable(ScriptList* self)` (0x2770C0): only a
/// running list (state 3) can be skipped, and only when the current command's
/// flags at +0x8 have bit 1 set.
pub fn script_list_is_current_skippable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 3;
        frame.save_ra(c, 0);
        let this = r(c, A0);
        let state = c.lw_r_2(V0, this.wrapping_add(0xa4));
        if state != c.g[V1][0] {
            c.g[V0][0] = 1;
        } else {
            c.cycles += 6;
            c.g[A0][0] = sx(this.wrapping_add(0xc));
            c.g[A1][0] = 0;
            callee::SCRIPT_CMD_QUEUE_AT.call(c, 0x0027_70e4)?;
            let cmd = c.lw_r_2(V0, r(c, V0).wrapping_add(8));
            let flags = sx(((cmd as u32) as i32 >> 2) as u32);
            c.g[V0][0] = flags & 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `ScriptList_dtor(self, flags)` (0x275d90): stop the script list, clear
/// its head and free it when `flags & 1`.
pub fn script_list_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        callee::SCRIPT_LIST_STOP.call(c, 0x0027_5db0)?;
        let self_ = abi::r32(c, S1);
        c.g[S0][0] &= 1;
        let keep = c.g[S0][0] == 0;
        Addr(self_).sw(c, 0);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0027_5dc4)?;
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
// Script engine.

/// `void ScriptList_OnSeqStopped(ScriptList* self, void* list, void*
/// script)` (0x277400): when `script` is still playing and its block exists,
/// notify the render-state manager with 0.
pub fn script_list_on_seq_stopped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S0][0];
        callee::SCRIPT_LIST_IS_PLAYING_SCRIPT.call(c, 0x0027_7418)?;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            callee::SCRIPT_DATA_GET_INFO.call(c, 0x0027_7428)?;
            c.g[V1][0] = Addr(r32(c, V0)).lb(c);
            let no_flag = c.g[V1][0] == 0;
            frame.restore(c, S0, 0x10);
            if !no_flag {
                c.cycles += 3;
                c.g[A0][0] = 0;
                call_ptr(c, 0x002e_f378, 0x0027_743c)?;
                frame.restore(c, S0, 0x10);
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

// ---------------------------------------------------------------------------
// Script list / data manager.

/// `void ScriptList_OnSeqStart(ScriptList* list, void* seq, ScriptObj* obj)`
/// (0x2772c0): when the sequence's script is playing and it is a named script,
/// post the "sequence started" message from `D_004A34A8`.
/// 8 (+6, +2) + 1 + 3 cycles plus the callees.
pub fn script_list_on_seq_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        let obj = c.g[A2][0];
        c.g[S0][0] = obj;
        frame.save_ra(c, 0);
        c.g[A1][0] = obj;
        callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2.call(c, 0x0027_72d8)?;
        if c.g[V0][0] == 0 {
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            callee::SCRIPT_DATA_GET_INFO_2.call(c, 0x0027_72e8)?;
            c.g[V1][0] = Addr(r32(c, V0)).lb(c);
            c.g[V0][0] = sx(0x004a_0000);
            let unnamed = c.g[V1][0] == 0;
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x34a8));
            if !unnamed {
                c.cycles += 2;
                call_ptr(c, callee::MESSAGE_CENTER_MESSAGE, 0x0027_7300)?;
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_UnloadMaster(ScriptEngine* e)` (0x278308): destroy the
/// script database, free the two object-table blocks at +0x52c and +0x534 and
/// clear them (and the +0x554 word). 10 (+3) + 7 cycles plus the callees.
pub fn script_engine_unload_master(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE.call(c, 0x0027_831c)?;
        c.g[A0][0] = c.lw(p(self_, 0x52c));
        call_ptr(c, callee::MEMORY_FREE, 0x0027_8324)?;
        let second = c.lw(p(self_, 0x534));
        c.g[A0][0] = second;
        if second == 0 {
            c.sw(p(self_, 0x554), 0);
        } else {
            c.cycles += 3;
            call_ptr(c, callee::MEMORY_FREE, 0x0027_8338)?;
            c.sw(p(self_, 0x554), 0);
        }
        c.cycles += 7;
        c.sw(p(self_, 0x52c), 0);
        c.sw(p(self_, 0x530), 0);
        c.sw(p(self_, 0x534), 0);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DataInstanceMan_dtor(DataInstanceMan* self, int flags)` (0x2758C0):
/// install the vtable, free the owned block at `+4`, and delete when bit 0 of
/// `flags` is set.
pub fn data_instance_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(ea(c.g[V0][0], 0x25a0));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(ea(c.g[S0][0], 0xc), c.g[V0][0]);
        c.g[A0][0] = c.lw(ea(c.g[S0][0], 4));
        let no_block = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_block {
            c.cycles += 2;
            call_ptr(c, addr::MEM_MAN_FREE_NOSIZE, 0x0027_58f4)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, addr::MEM_MAN_FREE, 0x0027_5908)?;
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
// Script data manager.

/// `void ScriptList_OnSeqEnd(ScriptList* self, int a1, void* obj)` (0x2773a0):
/// if the list's current command is a movie entry that is not playing, hand
/// over to the next ready script; a paused script is resumed first.
pub fn script_list_on_seq_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        call_ptr(c, 0x0027_71c8, 0x0027_73c0)?;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p_2(c.g[S0][0], 0xb8));
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = lw(c, c.g[S0][0] as u32);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0028_26d8, 0x0027_73e0)?;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0027_76e0, 0x0027_73e8)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ScriptList_CanSkip(ScriptList* list)` (0x277060): true when the
/// current command is skippable and the list has no pending delay.
pub fn script_list_can_skip(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let list = c.g[A0][0];
        c.g[S0][0] = list;
        frame.save_ra(c, 0);
        c.g[V1][0] = 0x3;
        c.g[V0][0] = Addr(at(c, S0, 0xa4)).lw(c);
        let not_script = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        if !not_script {
            'check: {
                c.cycles += 7;
                c.g[A0][0] = sx(at(c, S0, 0xc));
                c.g[A1][0] = 0;
                call_ptr(c, 0x0028_3d70, 0x0027_7090)?;
                c.g[V1][0] = Addr(r32_2(c, V0)).off(0x8).lw(c);
                c.g[V1][0] = c.g[V1][0] & 0x1;
                let not_skippable = c.g[V1][0] == 0;
                c.g[S1][0] = 0;
                if not_skippable {
                    break 'check;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(at(c, S0, 0xa8)).lw(c);
                c.g[S1][0] = (c.g[V0][0] < 0x1) as u64;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptList_GetEntryType(ScriptList* self, int i)` (0x276998): the
/// command type at_2 index `i` when it is in range and below 0x1c, else 0x1c.
pub fn script_list_get_entry_type(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = sx(at_2(c, A0, 0xc));
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0028_3d28, 0x0027_69b8)?;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        let mut result = 0x1c;
        if in_range {
            c.cycles += 6;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0028_3d70, 0x0027_69cc)?;
            c.g[V1][0] = Addr(at_2(c, V0, 0)).lw(c);
            c.g[V1][0] = ((c.g[V1][0] as i64) < 28) as u64;
            let low = c.g[V1][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if low {
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0028_3d70, 0x0027_69e4)?;
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                result = c.g[V0][0] as u32;
            } else {
                c.cycles += 1;
            }
        } else {
            c.cycles += 1;
        }
        c.g[V0][0] = sx(result);
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DataInstanceMan_RemoveAll(DataInstanceMan* m)` (0x275b08): remove
/// every live instance of the manager.
pub fn data_instance_man_remove_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        sqsp(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = Addr(p_3(r32(c, S0), 4)).lw(c);
                c.g[A1][0] = abi::addu(c.g[S2][0], c.g[V0][0]);
                c.g[V1][0] = Addr(p_3(c.g[A1][0] as u32, 0x10)).lw(c);
                if c.g[V1][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call::call_ptr(c, 0x0027_5a20, 0x0027_5b4c)?;
                }
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_5b30);
                }
            }
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `void ScriptList_ctor(self)` (0x00275d10): the script list constructor.
pub fn script_list_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[S1][0] = sx(word(c, S0).wrapping_add(0xc));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::SCRIPT_CMD_QUEUE_CTOR.call(c, 0x0027_5d30)?;
        c.g[A0][0] = c.g[S1][0];
        callee::SCRIPT_CMD_QUEUE_CLEAR.call(c, 0x0027_5d38)?;
        c.g[V0][0] = 0xffffffffffffffff;
        c.g[V1][0] = 0x1;
        Addr(at_3(c, S0, 0xbc)).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(at_3(c, S0, 0x4)).sw(c, c.g[V1][0]);
        for off in [0xa4, 0xa8, 0xc8, 0xb0, 0xac, 0xb8, 0xc0, 0xc4] {
            Addr(at_3(c, S0, off)).sw(c, 0);
        }
        Addr(at_3(c, S0, 0)).sw(c, 0);
        Addr(at_3(c, S0, 0xb4)).sw(c, 0);
        Addr(at_3(c, S0, 0x8)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script lists.

/// `int ScriptList_IsPlayingScript(self, process, script_obj_id)` (0x2771c8):
/// 0 unless the list is in state 3 and its current command is not a movie,
/// then whether `process` is the one running the command's script.
pub fn scriptlist_is_playing_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 3;
        frame.save(c, 16, 0x30);
        frame.save(c, 18, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        let state = lw_at(c, p(r32(c, S0), 0xa4));
        c.g[V1][0] = state;
        let not_ready = state != c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        if not_ready {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 9;
            c.g[A0][0] = sx(p(r32(c, S0), 0xc));
            c.g[A1][0] = 0;
            callee::SCRIPT_CMD_QUEUE_AT_2.call(c, 0x0027_71fc)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            callee::SCRIPT_LIST_IS_MOVIE_ENTRY.call(c, 0x0027_720c)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 6;
                c.g[A1][0] = lw_at(c, p(r32(c, S1), 0xc));
                c.g[A0][0] = lw_at(c, gw(c, -0x84c));
                callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2.call(c, 0x0027_7220)?;
                c.g[V0][0] ^= c.g[S2][0];
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script list.

/// `void ScriptList_PollLoading(ScriptList* self)` (0x277778): while the list
/// waits for its movie/process command (state 1), start the command at the
/// queue head; the state becomes 2 when the command is a movie entry or its
/// process reports state 2.
pub fn script_list_poll_loading(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0x1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S0), 0xa4)).lw(c);
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 9;
            c.g[A0][0] = sx(p(r32(c, S0), 0xc));
            c.g[A1][0] = 0;
            callee::SCRIPT_CMD_QUEUE_AT_3.call(c, 0x0027_77a8)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2.call(c, 0x0027_77b8)?;
            let movie = c.g[V0][0] != 0;
            c.g[V0][0] = 0x2;
            if movie {
                c.cycles += 1;
                Addr(p(r32(c, S0), 0xa4)).sw(c, c.g[V0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 7;
                c.g[A1][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                callee::SCRIPT_ENGINE_GET_PROCESS_STATE.call(c, 0x0027_77cc)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = 0x2;
                if c.g[V1][0] == c.g[V0][0] {
                    Addr(p(r32(c, S0), 0xa4)).sw(c, c.g[V1][0]);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                } else {
                    // The process is still pending: leave the list alone.
                    c.cycles += 2;
                    frame.restore(c, S0, 0x20);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script lists.

/// `void ScriptList_OnSeqAdvance(ScriptList* self, ScriptObj* obj)`
/// (0x277310): when the object is the list's playing script with no pending
/// transition and its remaining steps have run out, start the transition.
pub fn script_list_on_seq_advance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S1][0];
        callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3.call(c, 0x0027_7334)?;
        frame.restore(c, S0, 0x30);
        // Two of the four paths jump straight to the epilogue (their `lq16`
        // and `lq17` happen before the +1 cycle charge).
        let mut short_epilogue = false;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S2), 4)).lw(c);
            if c.g[V0][0] == 0 {
                short_epilogue = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x1c)).lw(c);
                if c.g[V0][0] != 0 {
                    short_epilogue = true;
                } else {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S1][0];
                    callee::SCRIPT_DATA_GET_INFO_4.call(c, 0x0027_735c)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS.call(c, 0x0027_7368)?;
                    c.g[V1][0] = Addr(p(r32(c, S0), 0x12)).lh(c);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let exhausted = c.g[V1][0] != 0;
                    frame.restore(c, S0, 0x30);
                    if !exhausted {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S2][0];
                        callee::SCRIPT_LIST_START_TRANSITION.call(c, 0x0027_7384)?;
                    }
                }
            }
        }
        if !short_epilogue {
            c.cycles += 1;
        }
        frame.restore(c, S1, 0x20);
        c.cycles += 4;
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `DataInstanceMan_ctor` (0x275828): allocate and clear a table of `count`
/// 0x18-byte data-instance records.
pub fn data_instance_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S1, 0x20);
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_add(0x25a0));
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = 0x18;
        c.mult(0, 0, 16, 4);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        Addr(this).sw(c, c.g[S0][0]);
        c.g[A1][0] = 0x0048_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_add(0x1ab8));
        c.g[A0][0] = c.lo[0];
        c.g[RA][0] = sx(0x0027_5874);
        c.g[A3][0] = 0;
        callee::OPERATOR_NEW.call(c, 0x0027_5874)?;
        Addr(p(this, 4)).sw(c, c.g[V0][0]);
        let empty = (c.g[S0][0] as i64) <= 0;
        Addr(p(this, 8)).sw(c, 0);
        if !empty {
            c.cycles += 2;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 7;
                c.g[A1][0] = Addr(p(this, 4)).lw(c);
                c.g[A0][0] = c.g[S1][0];
                c.g[S0][0] = sx(r(c, S0).wrapping_sub(1));
                c.g[RA][0] = sx(0x0027_589c);
                c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, S2)));
                callee::DATA_INSTANCE_FREE_INFO.call(c, 0x0027_589c)?;
                let more = c.g[S0][0] != 0;
                c.g[S2][0] = sx(r(c, S2).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_5888);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptList_CheckNextReady` (0x2776e0): if the next script cmd is a movie
/// entry or its process is paused, pause the list.
pub fn script_list_check_next_ready(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(p(this, 0xbc)).lw(c);
        let invalid = (c.g[A1][0] as i64) < 0;
        c.g[S2][0] = sx(this.wrapping_add(0xc));
        if !invalid {
            c.cycles += 8;
            c.g[RA][0] = sx(0x0027_770c);
            c.g[A0][0] = c.g[S2][0];
            callee::SCRIPT_CMD_QUEUE_AT_4.call(c, 0x0027_770c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0027_771c);
            c.g[A1][0] = c.g[S1][0];
            callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3.call(c, 0x0027_771c)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p(r(c, S1), 0xc)).lw(c);
                c.cycles += 1;
                c.g[V0][0] = sx(r(c, V0) >> 31);
            } else {
                c.cycles += 9;
                c.g[A1][0] = Addr(p(r(c, S0), 0xbc)).lw(c);
                c.g[RA][0] = sx(0x0027_7730);
                c.g[A0][0] = c.g[S2][0];
                callee::SCRIPT_CMD_QUEUE_AT_4.call(c, 0x0027_7730)?;
                c.g[A1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                c.g[RA][0] = sx(0x0027_773c);
                c.g[A0][0] = Addr(r(c, S0)).lw(c);
                callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2.call(c, 0x0027_773c)?;
                c.g[V0][0] = c.g[V0][0] ^ 1;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
            c.cycles += 2;
            let ready = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !ready {
                c.cycles += 3;
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x0027_7760);
                c.g[A2][0] = 1;
                callee::SCRIPT_LIST_SET_PAUSED.call(c, 0x0027_7760)?;
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

/// `void ScriptMovie_Stop(ScriptMovie *self)` (0x283818): stop the movie
/// player and release its resources.
pub fn script_movie_stop(c: &mut Cpu) {
    guest(c, |c| {
        'b_2838c8: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            if c.g[A0][0] == 0u64 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                break 'b_2838c8;
            } else {
                'b_2838c4: {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0028383c);
                    c.g[A1][0] = 0x3;
                    call_ptr(c, 0x00253418, 0x0028383c)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                    if t {
                        break 'b_2838c4;
                    } else {
                        c.cycles += 26;
                        c.g[A0][0] = 0x530000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x66e8));
                        c.g[A1][0] = 0x440000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28f0));
                        c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                        c.g[RA][0] = sx(0x00283880);
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
                        call_ptr(c, 0x0028b180, 0x00283880)?;
                        c.g[RA][0] = sx(0x00283888);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0029ce70, 0x00283888)?;
                        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
                        vslot(c, c.g[V0][0], 0x140, V1);
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x002838a4);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002838a4)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        if c.g[V0][0] == 0u64 {
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            break 'b_2838c8;
                        } else {
                            c.cycles += 5;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            c.g[RA][0] = sx(0x002838c4);
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                            call_ptr(c, 0x002ea860, 0x002838c4)?;
                            break 'b_2838c4;
                        }
                    }
                }
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                break 'b_2838c8;
            }
        }
        c.cycles += 7;
        c.sw(c.g[S0][0] as u32, 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `ScriptList_Add(self, cmd, arg)` (0x275dd8): queue a script command on
/// the list: push a command record, and when the pending list is in its
/// running state, prepare the next entry and advance it. Returns 1 when
/// queued, 0 when the command queue is full.
pub fn scriptlist_add(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        let _this = abi::arg(c, 0);
        frame.save(c, 16, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x50);
        frame.save(c, 18, 0x40);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        frame.save(c, 19, 0x30);
        c.g[S2][0] = c.g[A2][0];
        c.sd(p(sp(c), 0x20), c.g[RA][0]);
        c.g[S3][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x0027_5e08);
        c.g[A0][0] = c.g[S1][0];
        c.pc = 0x0028_3c58;
        ee::s07::ScriptCmdQueue_IsFull(c);
        if c.pc != 0x0027_5e08 {
            return Err(Unwound);
        }
        let full = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !full {
            c.cycles += 14;
            c.g[V0][0] = sx(0xffff_ffff);
            c.sw(sp(c), c.g[S3][0]);
            c.sw(p(sp(c), 8), c.g[S2][0]);
            c.g[A0][0] = c.g[S1][0];
            c.sw(p(sp(c), 0x18), c.g[V0][0]);
            c.g[A1][0] = c.g[SP][0];
            c.sw(p(sp(c), 4), c.g[V0][0]);
            c.sw(p(sp(c), 0xc), c.g[V0][0]);
            c.sw(p(sp(c), 0x10), c.g[V0][0]);
            c.g[RA][0] = sx(0x0027_5e3c);
            c.sw(p(sp(c), 0x14), c.g[V0][0]);
            c.pc = 0x0028_3c80;
            ee::s07::ScriptCmdQueue_Push(c);
            if c.pc != 0x0027_5e3c {
                return Err(Unwound);
            }
            c.g[V1][0] = lw_2(c, p(c.g[S0][0] as u32, 0xa4));
            let t = c.g[V1][0] == 0;
            c.g[V0][0] = 1;
            if !t {
                c.cycles += 3;
                c.g[V0][0] = lw_2(c, p(c.g[S0][0] as u32, 0xbc));
                let t = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = 1;
                if !t {
                    let mut direct = false;
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x0027_5e5c);
                    c.g[A0][0] = c.g[S1][0];
                    c.pc = 0x0028_3d28;
                    ee::s07::ScriptCmdQueue_Count(c);
                    if c.pc != 0x0027_5e5c {
                        return Err(Unwound);
                    }
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[RA][0] = sx(0x0027_5e6c);
                    c.g[A2][0] = 0;
                    c.pc = 0x0027_7450;
                    ee::s07::ScriptList_PrepareEntry(c);
                    if c.pc != 0x0027_5e6c {
                        return Err(Unwound);
                    }
                    c.g[A0][0] = lw_2(c, p(c.g[S0][0] as u32, 0xa4));
                    c.g[V1][0] = 3;
                    let t = c.g[A0][0] != c.g[V1][0];
                    c.sw(p(c.g[S0][0] as u32, 0xbc), c.g[V0][0]);
                    if !t {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x0027_5e88);
                        c.g[A1][0] = 0;
                        c.pc = 0x0028_3d70;
                        ee::s07::ScriptCmdQueue_At(c);
                        if c.pc != 0x0027_5e88 {
                            return Err(Unwound);
                        }
                        c.g[A1][0] = lw_2(c, p(c.g[V0][0] as u32, 0xc));
                        c.g[RA][0] = sx(0x0027_5e94);
                        c.g[A0][0] = lw_2(c, c.g[S0][0] as u32);
                        c.pc = 0x0028_2bf0;
                        ee::s07::ScriptEngine_GetProcessInSeq(c);
                        if c.pc != 0x0027_5e94 {
                            return Err(Unwound);
                        }
                        c.g[V1][0] = lw_2(c, p(c.g[V0][0] as u32, 0x20));
                        c.g[V1][0] ^= 4;
                        let t = c.g[V1][0] != 0;
                        c.g[V0][0] = 1;
                        if t {
                            direct = true;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x0027_5eac);
                            c.g[A0][0] = c.g[S0][0];
                            c.pc = 0x0027_76e0;
                            ee::s07::ScriptList_CheckNextReady(c);
                            if c.pc != 0x0027_5eac {
                                return Err(Unwound);
                            }
                        }
                    }
                    if !direct {
                        c.cycles += 1;
                        c.g[V0][0] = 1;
                    }
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        c.g[RA][0] = c.ld(p(sp(c), 0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `sDmNode* func_00275B98(void* self, char* big, sDmNode* prev)` (0x275b98):
/// append every entry of the BIG archive `big` to the data-instance manager's
/// list, recursing into entries that are themselves archives, and emit each
/// added node through vtable slot 2.
pub fn data_instance_man_add_big_entries(c: &mut Cpu) {
    let frame = Frame::open(c, 0xa0);
    'b_275ca0: {
        c.cycles += 25;
        frame.save(c, S3, 0x60);
        c.g[A3][0] = c.g[SP][0];
        frame.save(c, S4, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[A0][0];
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S7, 0x20);
        c.g[A0][0] = c.g[S5][0];
        frame.save(c, S0, 0x90);
        c.g[A1][0] = 0u64;
        frame.save(c, S1, 0x80);
        c.g[A2][0] = 0u64;
        frame.save(c, S6, 0x30);
        c.g[T0][0] = c.g[SP][0] | 0x4;
        frame.save_ra(c, 0x10);
        c.g[S7][0] = 0x1;
        sw(c, r32(c, SP), 0u64);
        c.g[RA][0] = sx(0x00275bf0);
        sw(c, at_4(c, SP, 0x4), 0u64);
        if call_ptr(c, 0x003e2768, 0x00275bf0).is_err() {
            return;
        }
        c.g[S2][0] = c.g[V0][0];
        let t = c.g[S2][0] == 0u64;
        c.g[S6][0] = 0x450000;
        if t {
            break 'b_275ca0;
        } else {
            c.cycles += 1;
            c.g[V0][0] = lw_3(c, r32(c, SP));
            'l_275c00: loop {
                'b_275c78: {
                    'b_275c34: {
                        c.cycles += 7;
                        c.g[S0][0] = lw_3(c, at_4(c, S6, 0xdd4));
                        c.g[S1][0] = sx(r32(c, S5).wrapping_add(r32(c, V0)));
                        sw(c, at_4(c, S6, 0xdd4), 0u64);
                        c.g[RA][0] = sx(0x00275c14);
                        c.g[A0][0] = c.g[S1][0];
                        if call_ptr(c, 0x003e2580, 0x00275c14).is_err() {
                            return;
                        }
                        let t = c.g[V0][0] == 0u64;
                        sw(c, at_4(c, S6, 0xdd4), c.g[S0][0]);
                        if t {
                            break 'b_275c34;
                        } else {
                            c.cycles += 6;
                            c.g[A2][0] = c.g[S3][0];
                            c.g[A1][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00275c2c);
                            c.g[A0][0] = c.g[S4][0];
                            if callee::C00275B98.call(c, 0x00275c2c).is_err() {
                                return;
                            }
                            let t = true;
                            c.g[S3][0] = c.g[V0][0];
                            if t {
                                break 'b_275c78;
                            } else {
                                break 'b_275c34;
                            }
                        }
                    }
                    c.cycles += 17;
                    c.g[RA][0] = sx(0x00275c3c);
                    c.g[A0][0] = c.g[S4][0];
                    if callee::C00275CF8.call(c, 0x00275c3c).is_err() {
                        return;
                    }
                    sw(c, at_4(c, V0, 0x8), c.g[S1][0]);
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V1][0] = lw_3(c, at_4(c, SP, 0x4));
                    sw(c, at_4(c, V0, 0xc), c.g[V1][0]);
                    sw(c, at_4(c, S3, 0x14), c.g[V0][0]);
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V1][0] = lw_3(c, at_4(c, S4, 0xc));
                    c.g[T0][0] = c.g[S3][0];
                    c.g[A3][0] = lw_3(c, at_4(c, SP, 0x4));
                    c.g[T1][0] = sx(at_4(c, S3, 0x4));
                    c.g[A0][0] = lh(c, at_4(c, V1, 0x10));
                    c.g[V0][0] = lw_3(c, at_4(c, V1, 0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00275c78);
                    c.g[A0][0] = sx(r32(c, S4).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x00275c78).is_err() {
                        return;
                    }
                    break 'b_275c78;
                }
                c.cycles += 10;
                c.g[A2][0] = c.g[S7][0];
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = 0u64;
                c.g[A3][0] = c.g[SP][0];
                c.g[T0][0] = 0u64;
                c.g[RA][0] = sx(0x00275c94);
                c.g[S7][0] = sx(at_4(c, S7, 0x1));
                if call_ptr(c, 0x003e2768, 0x00275c94).is_err() {
                    return;
                }
                c.g[S2][0] = c.g[V0][0];
                let t = c.g[S2][0] != 0u64;
                c.g[V0][0] = lw_3(c, r32(c, SP));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00275c00);
                    }
                    continue 'l_275c00;
                } else {
                    break 'b_275ca0;
                }
            }
        }
    }
    c.cycles += 12;
    c.g[V0][0] = c.g[S3][0];
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    c.g[RA][0] = ld(c, at_4(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0xa0));
    c.pc = t;
    return;
}

/// `void func_002780B8(void* self, int flags)` (0x2780b8): destroying
/// destructor of the script engine: install the base vtables, unload the
/// master script list, tear down the sound-bank, movie and script-object
/// managers, clear the per-frame state slots, and finally free the engine.
pub fn script_engine_dtor(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_278120: {
        c.cycles += 24;
        c.g[V0][0] = 0x480000;
        frame.save(c, S1, 0x50);
        c.g[V1][0] = 0x480000;
        frame.save(c, S5, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[A0][0] = 0x480000;
        frame.save(c, S2, 0x40);
        c.g[A2][0] = 0x480000;
        frame.save(c, S3, 0x30);
        c.g[V0][0] = sx(at_4(c, V0, 0x2100));
        frame.save(c, S4, 0x20);
        c.g[V1][0] = sx(at_4(c, V1, 0x21a0));
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[A0][0] = sx(at_4(c, A0, 0x2168));
        c.g[A2][0] = sx(at_4(c, A2, 0x21b8));
        c.g[S5][0] = c.g[A1][0];
        c.g[A3][0] = lw_3(c, at_4(c, S1, 0x52c));
        sw(c, at_4(c, S1, 0x2ac), c.g[V0][0]);
        sw(c, at_4(c, S1, 0x518), c.g[V1][0]);
        sw(c, at_4(c, S1, 0x528), c.g[A0][0]);
        let t = c.g[A3][0] == 0u64;
        sw(c, at_4(c, S1, 0x2a8), c.g[A2][0]);
        if t {
            break 'b_278120;
        } else {
            c.cycles += 2;
            c.g[RA][0] = sx(0x00278120);
            c.g[A0][0] = c.g[S1][0];
            if callee::C00278308.call(c, 0x00278120).is_err() {
                return;
            }
            break 'b_278120;
        }
    }
    'b_2781cc: {
        c.cycles += 31;
        c.g[A0][0] = sx(at_4(c, S1, 0xa38));
        c.g[A1][0] = 0x2;
        sw(c, at_4(c, GP, 0x3c0), 0u64);
        c.g[RA][0] = sx(0x00278134);
        c.g[S0][0] = sx(at_4(c, S1, 0x910));
        if callee::C00283518.call(c, 0x00278134).is_err() {
            return;
        }
        c.g[S2][0] = sx(at_4(c, S1, 0x6d0));
        c.g[A0][0] = sx(at_4(c, S1, 0xa28));
        c.g[RA][0] = sx(0x00278144);
        c.g[A1][0] = 0x2;
        if callee::C00275498.call(c, 0x00278144).is_err() {
            return;
        }
        c.g[S4][0] = sx(at_4(c, S1, 0x51c));
        c.g[V0][0] = 0x480000;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = sx(at_4(c, V0, 0x2298));
        c.g[V1][0] = sx(at_4(c, V1, 0x2260));
        sw(c, at_4(c, S1, 0x944), c.g[V0][0]);
        c.g[A0][0] = sx(at_4(c, S1, 0x938));
        sw(c, at_4(c, S1, 0x92c), c.g[V1][0]);
        c.g[RA][0] = sx(0x0027816c);
        c.g[A1][0] = 0u64;
        if callee::C00283228.call(c, 0x0027816c).is_err() {
            return;
        }
        c.g[S3][0] = sx(at_4(c, S1, 0x500));
        c.g[V0][0] = 0x480000;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = sx(at_4(c, V0, 0x2318));
        c.g[V1][0] = sx(at_4(c, V1, 0x22e0));
        sw(c, at_4(c, S1, 0x928), c.g[V0][0]);
        c.g[A0][0] = sx(at_4(c, S1, 0x91c));
        sw(c, at_4(c, S1, 0x910), c.g[V1][0]);
        c.g[RA][0] = sx(0x00278194);
        c.g[A1][0] = 0u64;
        if callee::C00283228.call(c, 0x00278194).is_err() {
            return;
        }
        let t = c.g[S2][0] == 0u64;
        c.g[A0][0] = c.g[S4][0];
        if t {
            break 'b_2781cc;
        } else {
            c.cycles += 2;
            let t = c.g[S2][0] == c.g[S0][0];
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x120));
            if t {
                break 'b_2781cc;
            } else {
                c.cycles += 1;
                'l_2781a8: loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = lw_3(c, at_4(c, S0, 0xc));
                    c.g[A0][0] = lh(c, at_4(c, V0, 0x8));
                    c.g[V1][0] = lw_3(c, at_4(c, V0, 0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002781c0);
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x002781c0).is_err() {
                        return;
                    }
                    let t = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x120));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002781a8);
                        }
                        continue 'l_2781a8;
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S4][0];
                        break 'b_2781cc;
                    }
                }
            }
        }
    }
    c.cycles += 17;
    c.g[RA][0] = sx(0x002781d4);
    c.g[A1][0] = 0u64;
    if callee::C002758C0.call(c, 0x002781d4).is_err() {
        return;
    }
    c.g[A0][0] = c.g[S3][0];
    c.g[RA][0] = sx(0x002781e0);
    c.g[A1][0] = 0u64;
    if callee::C002837C8.call(c, 0x002781e0).is_err() {
        return;
    }
    c.g[A0][0] = c.g[S1][0];
    c.g[RA][0] = sx(0x002781ec);
    c.g[A1][0] = c.g[S5][0];
    if callee::C00281ED8.call(c, 0x002781ec).is_err() {
        return;
    }
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0x70));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// Script list update (tick rate).

/// The callees of [`script_list_update`].
mod update_callee {
    use super::{ee, Callee};
    pub(super) const POLL_LOADING: Callee = Callee {
        addr: 0x0027_7778,
        f: ee::s07::ScriptList_PollLoading,
    };
    pub(super) const START: Callee = Callee {
        addr: 0x0027_6388,
        f: ee::s07::ScriptList_Start,
    };
    pub(super) const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::ScriptCmdQueue_At,
    };
    pub(super) const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::ScriptList_IsMovieEntry,
    };
    pub(super) const RESUME_PROCESS: Callee = Callee {
        addr: 0x0028_2720,
        f: ee::s07::ScriptEngine_ResumeProcess,
    };
    pub(super) const MOVIE_START: Callee = Callee {
        addr: 0x0028_39a8,
        f: ee::s07::ScriptMovie_Start,
    };
    pub(super) const GET_PROCESS_STATE: Callee = Callee {
        addr: 0x0028_2798,
        f: ee::s07::ScriptEngine_GetProcessState,
    };
    pub(super) const IS_PROCESS_FINISHED: Callee = Callee {
        addr: 0x0028_2838,
        f: ee::s07::ScriptEngine_IsProcessFinished,
    };
    pub(super) const SET_PAUSED: Callee = Callee {
        addr: 0x0027_7c08,
        f: ee::s07::ScriptList_SetPaused,
    };
    pub(super) const NEXT_ENTRY: Callee = Callee {
        addr: 0x0027_7838,
        f: ee::s07::ScriptList_NextEntry,
    };
    pub(super) const TRY_ADVANCE: Callee = Callee {
        addr: 0x0027_7800,
        f: ee::s07::ScriptList_TryAdvance,
    };
    pub(super) const MOVIE_GET_ID: Callee = Callee {
        addr: 0x0028_3bb8,
        f: ee::s07::ScriptMovie_GetId,
    };
    pub(super) const START_TRANSITION: Callee = Callee {
        addr: 0x0027_7980,
        f: ee::s07::ScriptList_StartTransition,
    };
}

/// `GameApp_GetPlayer(app, n)` (through the function table).
const APP_GET_PLAYER: u32 = 0x0023_0698;
/// The player's wake effect stop (through the function table).
const WAKE_FX_STOP: u32 = 0x002e_4578;

/// Script list fields.
mod slf {
    /// 1 loading, 2 ready, 3 playing.
    pub const STATE: u32 = 0xa4;
    pub const PAUSED: u32 = 0xa8;
    pub const RESUME_PENDING: u32 = 0xac;
    pub const TRANSITION: u32 = 0xb0;
    pub const START_PENDING: u32 = 0xb4;
    /// 60 Hz frames before the paused entry (script or movie) resumes:
    /// the transition's fade-in + hold (`ScriptList_StartTransition`,
    /// `ScriptList_Start`).
    pub const RESUME_WAIT: u32 = 0xb8;
    pub const NEXT: u32 = 0xbc;
    pub const NESTED_PAUSE: u32 = 0xc4;
    /// 60 Hz frames before a skipped entry advances (`ScriptList_SkipCurrent`
    /// sets 1).
    pub const SKIP_WAIT: u32 = 0xc8;
}

/// One update of a countdown that counts 60 Hz frames (`lw; beqz; addiu -1;
/// sw` in the original): at 60 it goes down by 1 when not 0, as the original;
/// above 60 only on a 60 Hz frame of its own (`tick::frame60_due`, keyed by
/// the field's address), so the wait lasts the same wall time. Returns the
/// value before (None when it was 0) and leaves `v0` = the value minus 1, as
/// the original's delay slot does.
fn countdown60(c: &mut Cpu, field: u32) -> Option<u32> {
    let v = Addr(field).lw(c);
    c.g[V0][0] = sx((v as u32).wrapping_sub(1));
    if v == 0 {
        return None;
    }
    let v = v as u32;
    let next = if crate::tick::frame60_due(c, field, v) {
        v.wrapping_sub(1)
    } else {
        v
    };
    c.cycles += 2;
    Addr(field).sw(c, sx(next));
    crate::tick::frame60_ran(c, field, next);
    Some(next)
}

/// `void ScriptList_Update(ScriptList* self)` (0x276cc8): one update of a
/// script list. While loading (state 1) it polls the loader; a pending start
/// (+0xb4) starts the list. While playing (state 3, not nested-paused):
/// during a transition (+0xb0) the resume wait (+0xb8) runs down and then
/// the paused script resumes (or the movie starts); a paused list (+0xa8)
/// waits for its next entry to be ready and moves on; else the skip wait
/// (+0xc8) runs down (ending the transition and advancing), or the list
/// advances when the current entry's script has finished or its movie ended.
///
/// Both waits count 60 Hz frames ([`countdown60`]): at another game rate they
/// last the same wall time (the fade-in + hold the transition shows).
pub fn script_list_update(c: &mut Cpu) {
    use update_callee as callee;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        let at = |off: u32| Addr(this.wrapping_add(off));
        c.g[V1][0] = at(slf::STATE).lw(c);
        let loading = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 2;
        if loading {
            c.cycles += 4;
            callee::POLL_LOADING.call(c, 0x0027_6cf8)?;
            frame.restore(c, S0, 0x30);
            return script_list_update_leave(c, frame);
        }
        c.cycles += 2;
        if c.g[V1][0] == c.g[V0][0] {
            // Ready: start the list when asked (+0xb4: 1 = from the top).
            c.cycles += 3;
            c.g[A1][0] = at(slf::START_PENDING).lw(c);
            let pending = c.g[A1][0] != 0;
            c.g[A1][0] ^= 1;
            if pending {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = (c.g[A1][0] < 1) as u64;
                c.g[A2][0] = 0;
                callee::START.call(c, 0x0027_6d24)?;
                at(slf::START_PENDING).sw(c, 0);
            }
            c.cycles += 1;
        }
        c.g[V1][0] = at(slf::STATE).lw(c);
        c.cycles += 3;
        c.g[V0][0] = 3;
        if c.g[V1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x30);
            return script_list_update_leave(c, frame);
        }
        c.cycles += 3;
        c.g[V0][0] = at(slf::NESTED_PAUSE).lw(c);
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x30);
            return script_list_update_leave(c, frame);
        }
        c.cycles += 3;
        c.g[V0][0] = at(slf::TRANSITION).lw(c);
        if c.g[V0][0] != 0 {
            // A transition: the resume wait runs down, then the paused entry
            // resumes.
            c.cycles += 12;
            c.g[A0][0] = sx(this.wrapping_add(0xc));
            c.g[A1][0] = 0;
            callee::CMD_QUEUE_AT.call(c, 0x0027_6d5c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[V0][0] = Addr(gw(c, -0x848)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x84).lw(c);
            call_ptr(c, APP_GET_PLAYER, 0x0027_6d70)?;
            c.g[S2][0] = c.g[V0][0];
            let mut charge = true;
            if countdown60(c, this.wrapping_add(slf::RESUME_WAIT)) == Some(0) {
                c.cycles += 3;
                c.g[V0][0] = at(slf::RESUME_PENDING).lw(c);
                c.g[A0][0] = c.g[S0][0];
                if c.g[V0][0] == 0 {
                    return script_list_update_advance(c, frame);
                }
                c.cycles += 4;
                c.g[A1][0] = c.g[S1][0];
                callee::IS_MOVIE_ENTRY.call(c, 0x0027_6d9c)?;
                c.g[A0][0] = at(0).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x500));
                    callee::MOVIE_START.call(c, 0x0027_6dc0)?;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = Addr(r32(c, S1)).off(0xc).lw(c);
                    callee::RESUME_PROCESS.call(c, 0x0027_6db0)?;
                    charge = false;
                }
            }
            if charge {
                c.cycles += 1;
            }
            // The player's effect done and nothing to wait for: the
            // transition is over.
            c.g[V0][0] = Addr(r32(c, S2)).off(0x44).lw(c);
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = at(slf::RESUME_WAIT).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    at(slf::RESUME_PENDING).sw(c, 0);
                    at(slf::TRANSITION).sw(c, 0);
                }
            }
        }
        c.g[V0][0] = at(slf::PAUSED).lw(c);
        c.cycles += 2;
        let paused = c.g[V0][0] != 0;
        c.g[S2][0] = sx(this.wrapping_add(0xc));
        if paused {
            // Paused: move on once the next entry is ready.
            c.cycles += 9;
            c.g[A1][0] = at(slf::NEXT).lw(c);
            c.g[A0][0] = c.g[S2][0];
            callee::CMD_QUEUE_AT.call(c, 0x0027_6df8)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            callee::IS_MOVIE_ENTRY.call(c, 0x0027_6e08)?;
            if c.g[V0][0] != 0 {
                // A movie is ready when it has an id.
                c.g[V0][0] = Addr(r32(c, S1)).off(0xc).lw(c);
                c.cycles += 2;
                c.g[V0][0] = !c.g[V0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
            } else {
                // A script when its process is ready (state 2).
                c.cycles += 9;
                c.g[A1][0] = at(slf::NEXT).lw(c);
                c.g[A0][0] = c.g[S2][0];
                callee::CMD_QUEUE_AT.call(c, 0x0027_6e1c)?;
                c.g[A1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
                c.g[A0][0] = at(0).lw(c);
                callee::GET_PROCESS_STATE.call(c, 0x0027_6e28)?;
                c.g[V0][0] ^= 2;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
            c.cycles += 2;
            let ready = c.g[V0][0] != 0;
            c.g[A1][0] = 0;
            if !ready {
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
                return script_list_update_leave(c, frame);
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 1;
            callee::SET_PAUSED.call(c, 0x0027_6e50)?;
            c.g[A0][0] = c.g[S0][0];
            callee::NEXT_ENTRY.call(c, 0x0027_6e58)?;
            frame.restore(c, S0, 0x30);
            return script_list_update_leave(c, frame);
        }
        c.cycles += 3;
        if countdown60(c, this.wrapping_add(slf::SKIP_WAIT)) == Some(0) {
            // A skip ran out: end the transition, advance.
            c.cycles += 3;
            c.g[V0][0] = at(slf::TRANSITION).lw(c);
            let transition = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(gw(c, -0x848)).lw(c);
            if transition {
                c.cycles += 8;
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr(r32(c, V0)).off(0x84).lw(c);
                call_ptr(c, APP_GET_PLAYER, 0x0027_6e8c)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, WAKE_FX_STOP, 0x0027_6e94)?;
                at(slf::RESUME_PENDING).sw(c, 0);
                at(slf::TRANSITION).sw(c, 0);
                at(slf::RESUME_WAIT).sw(c, 0);
            }
            return script_list_update_advance(c, frame);
        }
        // The current entry: advance once its script finished or its movie
        // has an id (then the transition starts).
        c.cycles += 10;
        c.g[A0][0] = sx(this.wrapping_add(0xc));
        c.g[A1][0] = 0;
        callee::CMD_QUEUE_AT.call(c, 0x0027_6ebc)?;
        c.g[S2][0] = 0;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::IS_MOVIE_ENTRY.call(c, 0x0027_6ed0)?;
        let movie = c.g[V0][0] != 0;
        c.g[A0][0] = at(0).lw(c);
        if movie {
            c.cycles += 4;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x500));
            callee::MOVIE_GET_ID.call(c, 0x0027_6f10)?;
            let id = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if id {
                c.cycles += 2;
                c.g[S2][0] = 1;
                callee::START_TRANSITION.call(c, 0x0027_6f20)?;
            }
        } else {
            c.cycles += 5;
            c.g[A1][0] = Addr(r32(c, S1)).off(0xc).lw(c);
            callee::GET_PROCESS_STATE.call(c, 0x0027_6ee0)?;
            c.g[V1][0] = 2;
            if c.g[V0][0] == c.g[V1][0] {
                c.g[S2][0] = 1;
            } else {
                c.cycles += 5;
                c.g[A1][0] = Addr(r32(c, S1)).off(0xc).lw(c);
                c.g[A0][0] = at(0).lw(c);
                callee::IS_PROCESS_FINISHED.call(c, 0x0027_6ef8)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[S2][0] = 1;
                }
            }
        }
        c.cycles += 2;
        if c.g[S2][0] == 0 {
            frame.restore(c, S0, 0x30);
            return script_list_update_leave(c, frame);
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        callee::TRY_ADVANCE.call(c, 0x0027_6f30)?;
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
        script_list_update_leave(c, frame)
    })
}

/// [`script_list_update`]: `ScriptList_TryAdvance(self)` (a0 = self) and
/// leave (0x276ea0).
fn script_list_update_advance(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    update_callee::TRY_ADVANCE.call(c, 0x0027_6ea8)?;
    frame.restore(c, S0, 0x30);
    script_list_update_leave(c, frame)
}

/// [`script_list_update`]'s epilogue (0x276f34; s0 already restored).
fn script_list_update_leave(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// ScriptList_GetPlayStatus (0x00276bd8)

/// `ScriptCmdQueue_At` (0x00283d70).
pub(crate) const SCRIPT_CMD_QUEUE_AT_CALL: Callee = Callee {
    addr: 0x0028_3d70,
    f: ee::s07::ScriptCmdQueue_At,
};

/// `ScriptEngine_IsProcessFinished` (0x00282838).
pub(crate) const SCRIPT_ENGINE_IS_PROCESS_FINISHED_CALL: Callee = Callee {
    addr: 0x0028_2838,
    f: ee::s07::ScriptEngine_IsProcessFinished,
};

/// `int ScriptList::GetPlayStatus(this)` (0x276bd8): the play state of a script list. 0 unless the state (+0xa4) is 3 (playing); 3 when +0xc4 is set (paused); otherwise, when +0xc8 is clear, look at the first queued command: if its flag 8 is set the list waits (2 with no wait object at +0xb0, or while the count at +0xb8 is above 1), else if the script process of its id (+0xc) has not finished the status is 1. The rest is 4 or, with a negative +0xbc, 5.
pub fn script_list_get_play_status(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        'b_276c98: {
            'b_276c94: {
                c.cycles += 9;
                Frame::open(c, 0x30);
                c.g[V1][0] = 0x3;
                frame.save(c, S0, 0x20);
                frame.save(c, S1, 0x10);
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xa4));
                let t = c.g[V0][0] != c.g[V1][0];
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_276c94;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xc4));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        break 'b_276c94;
                    } else {
                        'b_276c80: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xc8));
                            if c.g[V0][0] != 0u64 {
                                c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0xbc));
                                break 'b_276c80;
                            } else {
                                'b_276c7c: {
                                    'b_276c58: {
                                        c.cycles += 8;
                                        c.g[S1][0] = sx(r32(c, S0).wrapping_add(0xc));
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.g[S1][0];
                                        SCRIPT_CMD_QUEUE_AT_CALL.call(c, 0x0027_6c24)?;
                                        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x8));
                                        c.g[V1][0] = c.g[V1][0] & 0x8;
                                        if c.g[V1][0] == 0u64 {
                                            c.g[A0][0] = c.g[S1][0];
                                            break 'b_276c58;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xb0));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_276c94;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xb8));
                                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                                                let t = (c.g[V0][0] as i64) <= 0;
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_276c7c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    frame.restore(c, S0, 0x20);
                                                    if t {
                                                        break 'b_276c98;
                                                    } else {
                                                        break 'b_276c58;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A1][0] = 0u64;
                                    SCRIPT_CMD_QUEUE_AT_CALL.call(c, 0x0027_6c60)?;
                                    c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
                                    c.g[A0][0] = c.lw(r32(c, S0));
                                    SCRIPT_ENGINE_IS_PROCESS_FINISHED_CALL.call(c, 0x0027_6c6c)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0xbc));
                                        break 'b_276c80;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_276c94;
                                        } else {
                                            break 'b_276c7c;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0xbc));
                                break 'b_276c80;
                            }
                        }
                        c.cycles += 5;
                        c.g[V1][0] = 0xffffffffffffffff;
                        c.g[A0][0] = 0x4;
                        c.g[V0][0] = 0x5;
                        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        if c.g[V1][0] != 0 {
                            c.g[V0][0] = c.g[A0][0];
                        }
                        break 'b_276c94;
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            break 'b_276c98;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptList_PopEntries(ScriptList* self, int n)` (0x277598, `func_00277598`): handle up to `n` queued script commands.
/// While `n` is not zero and the command queue (`self+0xc`) is not empty: copy the front command (0x1c bytes, id at +0xc); if its id is not negative and it is not a movie entry (`IsMovieEntry`) stop the script request `ScriptEngine(*self)->0x282540(id)`; for a movie entry play the entry's sound (`ent[6]` when the video engine at `*self+0x500` has an id, else `ent[5]`) through the audio system's virtual `+4` entry when `*self+0x504` is set, then reset the video engine. Pop the command and go on.
pub fn script_list_pop_entries(c: &mut Cpu) {
    guest(c, |c| {
        'b_2776c4: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S2][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            let t = c.g[S2][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            if t {
                break 'b_2776c4;
            } else {
                c.cycles += 1;
                c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                'l_2775c0: loop {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x00283da0, 0x002775c8)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        break 'b_2776c4;
                    } else {
                        'b_2776b0: {
                            'b_2776ac: {
                                c.cycles += 19;
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x00283d70, 0x002775d8)?;
                                c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                                c.ldr(3, c.g[V0][0] as u32);
                                c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                                c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x17));
                                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x10));
                                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                                c.sdr(3, c.g[SP][0] as u32);
                                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
                                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x17));
                                c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A2][0]);
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                let t = (c.g[V0][0] as i64) < 0;
                                c.g[A0][0] = c.g[S1][0];
                                if t {
                                    break 'b_2776ac;
                                } else {
                                    'b_277640: {
                                        c.cycles += 4;
                                        c.g[A1][0] = c.g[SP][0];
                                        call_ptr(c, 0x00277dd8, 0x00277624)?;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                            break 'b_277640;
                                        } else {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                            call_ptr(c, 0x00282540, 0x00277638)?;
                                            let t = true;
                                            c.g[A0][0] = c.g[S3][0];
                                            if t {
                                                break 'b_2776b0;
                                            } else {
                                                break 'b_277640;
                                            }
                                        }
                                    }
                                    'b_2776a4: {
                                        c.cycles += 3;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x504));
                                        if c.g[V1][0] == 0u64 {
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            break 'b_2776a4;
                                        } else {
                                            'b_277674: {
                                                c.cycles += 9;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[A1][0] = c.g[SP][0];
                                                call_ptr(c, 0x00277de8, 0x00277658)?;
                                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                                c.g[S0][0] = c.g[V0][0];
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x500));
                                                call_ptr(c, 0x00283bb8, 0x00277668)?;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                    break 'b_277674;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                                    break 'b_277674;
                                                }
                                            }
                                            c.cycles += 2;
                                            if (c.g[S0][0] as i64) < 0 {
                                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                                break 'b_2776a4;
                                            } else {
                                                c.cycles += 10;
                                                call_ptr(c, 0x0028b180, 0x00277684)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x5558));
                                                c.g[A1][0] = c.g[S0][0];
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x118));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                                c.g[A2][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                                let t = c.g[A2][0] as u32;
                                                c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x002776a0)?;
                                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                                break 'b_2776a4;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
                                    call_ptr(c, 0x00283818, 0x002776ac)?;
                                    break 'b_2776ac;
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.g[S3][0];
                            break 'b_2776b0;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x00283dc0, 0x002776b8)?;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                        let t = c.g[S2][0] != 0u64;

                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002775c0);
                            }
                            continue 'l_2775c0;
                        } else {
                            break 'b_2776c4;
                        }
                    }
                }
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `int ScriptList_Start(ScriptList* self, int fade, int notify)` (0x276388, `tNISList_Play`): start the front script (or movie) of a loaded NIS list.
/// If the list is still loading (state +0xa4 == 1) poll it; if it is then not ready (state != 2) return the state. Otherwise set the state to 3, tell the HUD state (`UIStateStack_FindState` of the string hash 0x4a34a0) to hide through its virtual +0xc0, and take the front command (`ScriptCmdQueue_At(self+0xc, 0)`). A script command plays its script (`PlayScriptInstance(engine, self+8, id, 0, flags >> 3 & 1)`); when `self+4` is set it also stops the running transition and, with `fade`, starts the transition of the script data (the in, hold and out times in frames at +0xa, +0xc and +0xe of its info: `MakeTransitionObject` twice and `cTransitionMan_StartTransition`, or a screen grab for kind 7), pauses the process when a fade ran (`+0xac`, `+0xb0` = 1) and starts the cut scene transition (a fade of at least 0.5 s); finally `notify` calls the engine virtual +0x30 (slot 6). A movie command forces the cut scene mode (when `self+4`) and starts the video (`PlayVideo`, `UpdateVideo` with `notify`). Returns the state again.
pub fn script_list_start(c: &mut Cpu) {
    guest(c, |c| {
        'b_2763dc: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.g[V0][0] = 0x1;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S1][0] = c.g[A0][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[FP][0] = c.g[A2][0];
            if t {
                break 'b_2763dc;
            } else {
                c.cycles += 3;
                call_ptr(c, 0x00277778, 0x002763d8)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                break 'b_2763dc;
            }
        }
        'b_27667c: {
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[V1][0];
            if t {
                break 'b_27667c;
            } else {
                'b_27644c: {
                    c.cycles += 19;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x00283d70, 0x002763f4)?;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V0][0] = 0x3;
                    c.g[A0][0] = 0x4a0000;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x34a0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x00317670, 0x0027641c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039f9d8, 0x00276428)?;
                    c.g[A3][0] = c.g[V0][0];
                    let t = c.g[A3][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_27644c;
                    } else {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                        c.g[A2][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0027644c)?;
                        break 'b_27644c;
                    }
                }
                'b_276644: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S6][0];
                    call_ptr(c, 0x00277dd8, 0x00276458)?;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        break 'b_276644;
                    } else {
                        'b_276618: {
                            c.cycles += 11;
                            c.g[T0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = 0u64;
                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                            c.g[T0][0] = sx(((c.g[T0][0] as u32) as i32 >> 3) as u32);
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                            c.g[T0][0] = c.g[T0][0] & 0x1;
                            call_ptr(c, 0x00279720, 0x00276480)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            if t {
                                break 'b_276618;
                            } else {
                                'b_2764b4: {
                                    c.cycles += 7;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                    call_ptr(c, 0x00230698, 0x00276498)?;
                                    c.g[S3][0] = c.g[V0][0];
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x44));
                                    if c.g[V0][0] == 0u64 {
                                        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                        break 'b_2764b4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x002e4578, 0x002764b0)?;
                                        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                        break 'b_2764b4;
                                    }
                                }
                                'b_2764e0: {
                                    c.cycles += 9;
                                    c.g[S4][0] = 0u64;
                                    c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                                    c.mtc1(0, 20);
                                    c.g[S5][0] = 0u64;
                                    call_ptr(c, 0x00282bf0, 0x002764c8)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x0027c070, 0x002764d0)?;
                                    let t = c.g[S2][0] == 0u64;
                                    c.g[S0][0] = c.g[V0][0];
                                    if t {
                                        break 'b_2764e0;
                                    } else {
                                        c.cycles += 2;
                                        c.g[S4][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xa));
                                        c.g[S5][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xc));
                                        break 'b_2764e0;
                                    }
                                }
                                'b_2765ec: {
                                    c.cycles += 6;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0xb8), 0u64);
                                    c.g[S7][0] =
                                        sx((c.g[S4][0] as u32).wrapping_add(c.g[S5][0] as u32));
                                    c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xe));
                                    c.g[V0][0] =
                                        sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[V0][0] = 0x7;
                                    if t {
                                        break 'b_2765ec;
                                    } else {
                                        'b_2765c8: {
                                            'b_2765c4: {
                                                'b_276584: {
                                                    c.cycles += 3;
                                                    c.g[A1][0] =
                                                        c.lb((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    let t = c.g[A1][0] == c.g[V0][0];

                                                    if t {
                                                        break 'b_276584;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                                        c.g[A2][0] = c
                                                            .lb((c.g[S0][0] as u32)
                                                                .wrapping_add(0x9));
                                                        call_ptr(c, 0x0027a4a0, 0x00276510)?;
                                                        c.g[S2][0] = c.g[V0][0];
                                                        if c.g[S2][0] == 0u64 {
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xb8));
                                                            break 'b_2765c8;
                                                        } else {
                                                            c.cycles += 26;
                                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                                            c.g[A1][0] = c.lb((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A2][0] = c.lb((c.g[S0][0] as u32)
                                                                .wrapping_add(0x9));
                                                            call_ptr(c, 0x0027a4a0, 0x0027652c)?;
                                                            c.g[V1][0] = c.lh((c.g[S0][0] as u32)
                                                                .wrapping_add(0xe));
                                                            c.mtc1(20, 1);
                                                            c.cvt_s_w(1, 1);
                                                            c.lwc1(
                                                                0,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x4804),
                                                            );
                                                            c.mtc1(21, 13);
                                                            c.cvt_s_w(13, 13);
                                                            c.mtc1(3, 14);
                                                            c.cvt_s_w(14, 14);
                                                            c.g[A0][0] = c.g[S3][0];
                                                            c.mul_s(1, 1, 0);
                                                            c.g[A2][0] = c.g[S2][0];
                                                            c.mul_s(13, 13, 0);
                                                            c.g[A3][0] = c.g[V0][0];
                                                            c.mul_s(14, 14, 0);
                                                            c.g[A1][0] = 0x2;
                                                            c.mov_s(12, 1);
                                                            c.g[T0][0] = 0u64;
                                                            c.g[T1][0] = 0u64;
                                                            c.mov_s(20, 1);
                                                            call_ptr(c, 0x002e4370, 0x0027657c)?;
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0xb8),
                                                                c.g[S7][0],
                                                            );
                                                            if t {
                                                                break 'b_2765c4;
                                                            } else {
                                                                break 'b_276584;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[S4][0] == 0u64;
                                                c.g[A1][0] = 0x7;
                                                if t {
                                                    break 'b_2765c4;
                                                } else {
                                                    c.cycles += 14;
                                                    c.g[A2][0] =
                                                        c.lb((c.g[S0][0] as u32).wrapping_add(0x9));
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x84c)
                                                        );
                                                    call_ptr(c, 0x0027a4a0, 0x00276598)?;
                                                    c.mtc1(20, 0);
                                                    c.cvt_s_w(0, 0);
                                                    c.lwc1(
                                                        1,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x4800),
                                                    );
                                                    c.g[A0][0] = c.g[S3][0];
                                                    c.g[A2][0] = c.g[V0][0];
                                                    c.g[A1][0] = 0x2;
                                                    c.g[A3][0] = 0u64;
                                                    c.mul_s(0, 0, 1);
                                                    c.mov_s(12, 0);
                                                    c.mov_s(20, 0);
                                                    call_ptr(c, 0x002e4540, 0x002765c4)?;
                                                    break 'b_2765c4;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xb8));
                                            break 'b_2765c8;
                                        }
                                        c.cycles += 2;
                                        let t = (c.g[V0][0] as i64) <= 0;

                                        if t {
                                            break 'b_2765ec;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            c.g[FP][0] = 0u64;
                                            c.g[A1][0] =
                                                c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                                            call_ptr(c, 0x002826d8, 0x002765e0)?;
                                            c.g[V0][0] = 0x1;
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0xb0),
                                                c.g[V0][0],
                                            );
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0xac),
                                                c.g[V0][0],
                                            );
                                            break 'b_2765ec;
                                        }
                                    }
                                }
                                'b_276608: {
                                    c.cycles += 5;
                                    c.mtc1(0, 0);
                                    c.c_le_s(20, 0);
                                    let t = !c.fcc();
                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_276608;
                                    } else {
                                        c.cycles += 2;
                                        c.g[AT][0] = 0x3f000000;
                                        c.mtc1(1, 20);
                                        break 'b_276608;
                                    }
                                }
                                c.cycles += 4;
                                c.mov_s(12, 20);
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                                call_ptr(c, 0x002ea6b8, 0x00276618)?;
                                break 'b_276618;
                            }
                        }
                        c.cycles += 2;
                        if c.g[FP][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                            break 'b_27667c;
                        } else {
                            c.cycles += 9;
                            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                            c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x2a8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0027663c)?;
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                            if t {
                                break 'b_27667c;
                            } else {
                                break 'b_276644;
                            }
                        }
                    }
                }
                'b_276658: {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_276658;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                        call_ptr(c, 0x002ea860, 0x00276658)?;
                        break 'b_276658;
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
                call_ptr(c, 0x002839a8, 0x00276664)?;
                if c.g[FP][0] == 0u64 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                    break 'b_27667c;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
                    call_ptr(c, 0x00283aa0, 0x00276678)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                    break 'b_27667c;
                }
            }
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        Ok(())
    })
}

/// `int ScriptList::PrepareEntry(this, int start, bool skip_flagged)` (0x277450,
/// `tNISList::SetupNextScript`): find the first entry of the script list's command
/// queue (`this + 0xc`) from index `start` that is ready to play and return its
/// index, or -1. Entries with flag 2 (+0x8) are skipped when `skip_flagged`; an
/// entry whose script id (+0xc) is already non-negative is ready. A script
/// entry requests its script from the engine (`*this`) by index (+0x4) or, with
/// a negative index, by category (+0x0), with the entry's parameters
/// (+0x10, +0x14, +0x18); the id it gets back is stored in +0xc. A movie entry
/// opens its video on the engine's movie player (+0x500) when that one is idle,
/// clearing the id when it opened.
pub fn script_list_prepare_entry(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x00277450);
        'b_277578: {
            'b_277574: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S2][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S4][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S1][0] = c.g[S0][0];
                Callee {
                    addr: 0x00283d28,
                    f: ee::s07::ScriptCmdQueue_Count,
                }
                .call(c, 0x00277484)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    break 'b_277574;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                    'l_277498: loop {
                        'b_277564: {
                            'b_2774b8: {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x00283d70,
                                    f: ee::s07::ScriptCmdQueue_At,
                                }
                                .call(c, 0x002774a0)?;
                                let t = c.g[S4][0] == 0u64;
                                c.g[S0][0] = c.g[V0][0];
                                if t {
                                    break 'b_2774b8;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = c.g[V0][0] & 0x2;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                        break 'b_277564;
                                    } else {
                                        break 'b_2774b8;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            let t = (c.g[V0][0] as i64) >= 0;
                            c.g[V0][0] = c.g[S1][0];
                            if t {
                                break 'b_277574;
                            } else {
                                'b_277560: {
                                    'b_277528: {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[A1][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x00277dd8,
                                            f: ee::s07::ScriptList_IsMovieEntry,
                                        }
                                        .call(c, 0x002774d0)?;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                            break 'b_277528;
                                        } else {
                                            'b_277514: {
                                                'b_2774fc: {
                                                    c.cycles += 3;
                                                    c.g[A1][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                    let t = (c.g[A1][0] as i64) < 0;
                                                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                                                    if t {
                                                        break 'b_2774fc;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A2][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[A3][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x14));
                                                        c.g[T0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                        Callee {
                                                            addr: 0x002795d0,
                                                            f: ee::s07::ScriptEngine_SpawnScript,
                                                        }
                                                        .call(c, 0x002774f4)?;
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0xc),
                                                            c.g[V0][0],
                                                        );
                                                        if t {
                                                            break 'b_277514;
                                                        } else {
                                                            break 'b_2774fc;
                                                        }
                                                    }
                                                }
                                                c.cycles += 6;
                                                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                                c.g[A2][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                c.g[A3][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                c.g[T0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                                Callee {
                                                    addr: 0x00279528,
                                                    f: ee::s07::ScriptEngine_SpawnChosenScript,
                                                }
                                                .call(c, 0x00277510)?;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xc),
                                                    c.g[V0][0],
                                                );
                                                break 'b_277514;
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                            let t = (c.g[V0][0] as i64) < 0;
                                            c.g[V0][0] = c.g[S1][0];
                                            if t {
                                                break 'b_277560;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                if t {
                                                    break 'b_277578;
                                                } else {
                                                    break 'b_277528;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x500));
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[V0][0] = c.g[S1][0];
                                    if t {
                                        break 'b_277574;
                                    } else {
                                        'b_277558: {
                                            c.cycles += 9;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x00277de8,
                                                f: ee::s07::ScriptList_GetMovieInfo,
                                            }
                                            .call(c, 0x00277540)?;
                                            c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                                            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x500));
                                            Callee {
                                                addr: 0x002838e8,
                                                f: ee::s07::ScriptMovie_Open,
                                            }
                                            .call(c, 0x00277550)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                                                break 'b_277558;
                                            } else {
                                                break 'b_277558;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = c.g[S1][0];
                                        if t {
                                            break 'b_277574;
                                        } else {
                                            break 'b_277560;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                break 'b_277564;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00277498);
                            }
                            continue 'l_277498;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0xffffffffffffffff;
                            break 'b_277574;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_277578;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    })
}

/// `void ScriptList_Stop(ScriptList* self, int resetTransition)` (0x276868):
/// stop the running script list: unpause it (`ScriptList_SetPaused`), and
/// for a list in state 3 with a cut-scene transition restore the regular
/// mode (`cCutSceneTransitionMan_ForceOff`, stop the transition manager) and
/// re-show the `hud` UI state (virtual slot 0x18); then pop all entries and
/// clear the list's state words (`+0xa4`..`+0xc8`).
#[allow(unreachable_code)]
pub fn script_list_stop(c: &mut Cpu) {
    guest(c, |c| {
        'b_276894: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa8));
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = c.g[A1][0];
            if t {
                break 'b_276894;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0x1;
                Callee {
                    addr: 0x00277c08,
                    f: ee::s07::ScriptList_SetPaused,
                }
                .call(c, 0x00276894)?;
                break 'b_276894;
            }
        }
        'b_2768b4: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc4));
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_2768b4;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                Callee {
                    addr: 0x00277c08,
                    f: ee::s07::ScriptList_SetPaused,
                }
                .call(c, 0x002768b4)?;
                break 'b_2768b4;
            }
        }
        'b_276954: {
            'b_276950: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x3;
                if t {
                    break 'b_276950;
                } else {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        break 'b_276954;
                    } else {
                        'b_276904: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            let t = c.g[S0][0] == 0u64;
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                            if t {
                                break 'b_276904;
                            } else {
                                c.cycles += 5;
                                call_ptr(c, 0x002ea820, 0x002768e4)?;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb0));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_276904;
                                } else {
                                    c.cycles += 5;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                    call_ptr(c, 0x00230698, 0x002768fc)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x002e4578, 0x00276904)?;
                                    break 'b_276904;
                                }
                            }
                        }
                        c.cycles += 13;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = 0x4a0000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x34a0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x00317670, 0x00276920)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039f9d8, 0x0027692c)?;
                        c.g[A3][0] = c.g[V0][0];
                        let t = c.g[A3][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_276950;
                        } else {
                            c.cycles += 6;
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00276950)?;
                            break 'b_276950;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
            break 'b_276954;
        }
        c.cycles += 17;
        c.g[A1][0] = 0x5;
        Callee {
            addr: 0x00277598,
            f: ee::s07::ScriptList_PopEntries,
        }
        .call(c, 0x0027695c)?;
        c.g[V0][0] = 0xffffffffffffffff;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xb4), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xa4), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc8), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xb0), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xb8), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xac), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc0), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc4), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `int ScriptList_AddChoice(ScriptList* self, int a1, int a2, int lookup,
/// int a4, int a5, int a6)` (0x275ed0): queue a script choice on the list's
/// command queue (`self + 0xc`). Returns 0 when the queue is full or the
/// script lookup (`ScriptEngine_FindScriptIndex`) fails, else 1; after the
/// push, a list in mode 3 whose head entry is a finished movie / request
/// player (state 4) calls `ScriptList_CheckNextReady`.
///
/// Has resume entries: a resumed pc (after an unwound callee) goes to the
/// generated `__entries` body.
pub fn script_list_add_choice(c: &mut Cpu) {
    if c.pc != 0x0027_5ed0 {
        return script_list_add_choice_resume(c);
    }
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S4, 0x50);
        c.g[S3][0] = c.g[T1][0];
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[T2][0];
        frame.save(c, 22, 0x30);
        c.g[S5][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[S6][0] = c.g[A3][0];
        c.g[A0][0] = sx(oc429_at(c.g[S0][0], 0xc));
        call_ptr(c, 0x0028_3c58, 0x0027_5f18)?; // ScriptCmdQueue_IsFull
        let full = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !full {
            'queued: {
                c.cycles += 9;
                c.g[V0][0] = u64::MAX;
                c.sw(oc429_at(c.g[SP][0], 8), c.g[S1][0]);
                c.sw(oc429_at(c.g[SP][0], 0xc), c.g[V0][0]);
                c.sw(sp(c), c.g[S5][0]);
                c.sw(oc429_at(c.g[SP][0], 4), c.g[V0][0]);
                c.sw(oc429_at(c.g[SP][0], 0x10), c.g[S2][0]);
                c.sw(oc429_at(c.g[SP][0], 0x14), c.g[S3][0]);
                let no_lookup = c.g[S6][0] == 0;
                c.sw(oc429_at(c.g[SP][0], 0x18), c.g[S4][0]);
                if !no_lookup {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[T0][0] = c.g[S4][0];
                    call_ptr(c, 0x0027_97a0, 0x0027_5f5c)?; // ScriptEngine_FindScriptIndex
                    if (c.g[V0][0] as i64) >= 0 {
                        c.sw(oc429_at(c.g[SP][0], 4), c.g[V0][0]);
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        break 'queued;
                    }
                }
                c.cycles += 7;
                c.g[S1][0] = sx(oc429_at(c.g[S0][0], 0xc));
                c.g[A1][0] = c.g[SP][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0028_3c80, 0x0027_5f7c)?; // ScriptCmdQueue_Push
                c.g[V1][0] = c.lw(oc429_at(c.g[S0][0], 0xa4));
                let t = c.g[V1][0] == 0;
                c.g[V0][0] = 1;
                if t {
                    break 'queued;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(oc429_at(c.g[S0][0], 0xbc));
                let t = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = 1;
                if t {
                    break 'queued;
                }
                'done: {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0028_3d28, 0x0027_5f9c)?; // ScriptCmdQueue_Count
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x0027_7450, 0x0027_5fac)?; // ScriptList_PrepareEntry
                    c.g[A0][0] = c.lw(oc429_at(c.g[S0][0], 0xa4));
                    c.g[V1][0] = 3;
                    let not_mode3 = c.g[A0][0] != c.g[V1][0];
                    c.sw(oc429_at(c.g[S0][0], 0xbc), c.g[V0][0]);
                    if not_mode3 {
                        break 'done;
                    }
                    'ready: {
                        'movie: {
                            c.cycles += 8;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = 0;
                            call_ptr(c, 0x0028_3d70, 0x0027_5fc8)?; // ScriptCmdQueue_At
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0027_7dd8, 0x0027_5fd4)?; // ScriptList_IsMovieEntry
                            if c.g[V0][0] != 0 {
                                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                break 'movie;
                            }
                            c.cycles += 10;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = 0;
                            call_ptr(c, 0x0028_3d70, 0x0027_5fe8)?; // ScriptCmdQueue_At
                            c.g[A1][0] = c.lw(oc429_at(c.g[V0][0], 0xc));
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            call_ptr(c, 0x0028_2bf0, 0x0027_5ff4)?; // ScriptEngine_GetProcessInSeq
                            c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x20));
                            c.g[V1][0] ^= 0x4;
                            c.g[V0][0] = (c.g[V1][0] < 1) as u64;
                            break 'ready;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = sx(oc429_at(c.g[A0][0], 0x500));
                        call_ptr(c, 0x0028_3bb8, 0x0027_600c)?; // ScriptMovie_GetId
                    }
                    c.cycles += 2;
                    let not_ready = c.g[V0][0] == 0;
                    c.g[V0][0] = 1;
                    if not_ready {
                        break 'queued;
                    }
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0027_76e0, 0x0027_601c)?; // ScriptList_CheckNextReady
                }
                c.cycles += 1;
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, 22, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// Native resume of `script_list_add_choice` (converted from the `entries` body): any entry
/// other than 0x00275ed0, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn script_list_add_choice_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00275ed0 => {
                    c.cycles += 20;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S1][0] = c.g[A2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S2][0] = c.g[T0][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S3][0] = c.g[T1][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S4][0] = c.g[T2][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S5][0] = c.g[A1][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.g[S6][0] = c.g[A3][0];
                    c.g[RA][0] = sx(0x00275f18);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    Callee {
                        addr: 0x00283c58,
                        f: ee::s07::ScriptCmdQueue_IsFull,
                    }
                    .call(c, 0x00275f18)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00276020;
                        continue;
                    }
                    pc = 0x00275f20;
                    continue;
                }
                0x00275f20 => {
                    c.cycles += 9;
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.sw(c.g[SP][0] as u32, c.g[S5][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S2][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[S3][0]);
                    let t = c.g[S6][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S4][0]);
                    if t {
                        pc = 0x00275f6c;
                        continue;
                    }
                    pc = 0x00275f44;
                    continue;
                }
                0x00275f44 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00275f5c);
                    c.g[T0][0] = c.g[S4][0];
                    Callee {
                        addr: 0x002797a0,
                        f: ee::s07::ScriptEngine_FindScriptIndex,
                    }
                    .call(c, 0x00275f5c)?;

                    if (c.g[V0][0] as i64) >= 0 {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        pc = 0x00275f6c;
                        continue;
                    }
                    pc = 0x00275f64;
                    continue;
                }
                0x00275f64 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00276020;
                        continue;
                    }
                    pc = 0x00275f6c;
                    continue;
                }
                0x00275f6c => {
                    c.cycles += 7;
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00275f7c);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00283c80,
                        f: ee::s07::ScriptCmdQueue_Push,
                    }
                    .call(c, 0x00275f7c)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00276020;
                        continue;
                    }
                    pc = 0x00275f88;
                    continue;
                }
                0x00275f88 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00276020;
                        continue;
                    }
                    pc = 0x00275f94;
                    continue;
                }
                0x00275f94 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x00275f9c);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x00283d28,
                        f: ee::s07::ScriptCmdQueue_Count,
                    }
                    .call(c, 0x00275f9c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[RA][0] = sx(0x00275fac);
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x00277450,
                        f: ee::s07::ScriptList_PrepareEntry,
                    }
                    .call(c, 0x00275fac)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                    c.g[V1][0] = 0x3;
                    let t = c.g[A0][0] != c.g[V1][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                    if t {
                        pc = 0x0027601c;
                        continue;
                    }
                    pc = 0x00275fbc;
                    continue;
                }
                0x00275fbc => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00275fc8);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x00283d70,
                        f: ee::s07::ScriptCmdQueue_At,
                    }
                    .call(c, 0x00275fc8)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00275fd4);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00277dd8,
                        f: ee::s07::ScriptList_IsMovieEntry,
                    }
                    .call(c, 0x00275fd4)?;

                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                        pc = 0x00276004;
                        continue;
                    }
                    pc = 0x00275fdc;
                    continue;
                }
                0x00275fdc => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00275fe8);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x00283d70,
                        f: ee::s07::ScriptCmdQueue_At,
                    }
                    .call(c, 0x00275fe8)?;
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[RA][0] = sx(0x00275ff4);
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    Callee {
                        addr: 0x00282bf0,
                        f: ee::s07::ScriptEngine_GetProcessInSeq,
                    }
                    .call(c, 0x00275ff4)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.g[V1][0] ^ 0x4;
                    let t = true;
                    c.g[V0][0] = (c.g[V1][0] < 0x1u64) as u64;
                    if t {
                        pc = 0x0027600c;
                        continue;
                    }
                    pc = 0x00276004;
                    continue;
                }
                0x00276000 => {
                    c.cycles += 1;
                    c.g[V0][0] = (c.g[V1][0] < 0x1u64) as u64;
                    pc = 0x00276004;
                    continue;
                }
                0x00276004 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0027600c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
                    Callee {
                        addr: 0x00283bb8,
                        f: ee::s07::ScriptMovie_GetId,
                    }
                    .call(c, 0x0027600c)?;
                    pc = 0x0027600c;
                    continue;
                }
                0x0027600c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00276020;
                        continue;
                    }
                    pc = 0x00276014;
                    continue;
                }
                0x00276014 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0027601c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002776e0,
                        f: ee::s07::ScriptList_CheckNextReady,
                    }
                    .call(c, 0x0027601c)?;
                    pc = 0x0027601c;
                    continue;
                }
                0x0027601c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00276020;
                    continue;
                }
                0x00276020 => {
                    c.cycles += 10;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00275ed0, 0x00276048);
                    return Ok(());
                }
            }
        }
    });
}

/// `DataNode* DataInstanceMan_AddData(DataMan* self, int name, void* data,
/// int size)` (0x275920, `cDataManager::AttachData`): take a node from the
/// free list (`AllocInfo`), fill its data / size / refcount (1), then ask
/// `BigFile_GetHeaderSize` (with the global abort flag at `0x450dd4`
/// cleared around it) whether `data` is a big file. A big file goes through
/// the owner's vtable `+0x20` slot and `AddBigEntries`, anything else
/// through the vtable `+0x10` slot. Returns the node.
pub fn data_instance_man_add_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S4, 0x30);
        frame.save(c, S3, 0x40);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S5, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, 22, 0x10);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S0, 0x70);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0x45_0000;
        call_ptr(c, 0x0027_5cf8, 0x0027_595c)?; // DataInstanceMan_AllocInfo
        c.g[S2][0] = c.g[V0][0];
        c.g[S1][0] = c.lw(oc429_at(c.g[S0][0], 0xdd4));
        c.g[V0][0] = 1;
        c.sw(oc429_at(c.g[S2][0], 8), c.g[S3][0]);
        c.sw(oc429_at(c.g[S2][0], 0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S3][0];
        c.sw(oc429_at(c.g[S2][0], 0xc), c.g[S5][0]);
        c.sw(oc429_at(c.g[S0][0], 0xdd4), 0);
        call_ptr(c, 0x003e_2580, 0x0027_5980)?; // BigFile_GetHeaderSize
        let not_big = c.g[V0][0] == 0;
        c.sw(oc429_at(c.g[S0][0], 0xdd4), c.g[S1][0]);
        if !not_big {
            c.cycles += 16;
            c.g[V0][0] = c.lw(oc429_at(c.g[S4][0], 0xc));
            c.g[A1][0] = c.g[S6][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S5][0];
            c.g[A0][0] = c.lh(oc429_at(c.g[V0][0], 0x20));
            c.g[T0][0] = c.g[S2][0];
            c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x24));
            c.g[T1][0] = sx(oc429_at(c.g[S2][0], 4));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(oc429_at(c.g[S4][0], c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_59b0)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, 0x0027_5b98, 0x0027_59c0)?; // DataInstanceMan_AddBigEntries
            c.g[V0][0] = c.g[S2][0];
        } else {
            c.cycles += 11;
            c.g[V0][0] = c.lw(oc429_at(c.g[S4][0], 0xc));
            c.g[A1][0] = c.g[S6][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S5][0];
            c.g[A0][0] = c.lh(oc429_at(c.g[V0][0], 0x10));
            c.g[T0][0] = c.g[S2][0];
            c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x14));
            c.g[T1][0] = sx(oc429_at(c.g[S2][0], 4));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(oc429_at(c.g[S4][0], c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_59f0)?;
            c.g[V0][0] = c.g[S2][0];
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptEngine* ScriptEngine_ctor(ScriptEngine* self)`: constructs the two view objects, the script/movie/data tables, the dummy, audio, anim bank and sound bank objects, clears the engine state, installs the instance pointer at `gp+0x3c0` and numbers the two view slots.
/// (`ScriptEngine_ctor`, 0x00277f20)
pub fn script_engine_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = 0x1;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6d0));
        Callee {
            addr: 0x00281e80,
            f: ee::s07::ScriptEngine_ScriptDbCtor,
        }
        .call(c, 0x00277f48)?;
        c.g[S3][0] = 0xffffffffffffffff;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x500));
        Callee {
            addr: 0x00283798,
            f: ee::s07::ScriptMovie_ctor,
        }
        .call(c, 0x00277f54)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x51c));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x100;
        Callee {
            addr: 0x00275828,
            f: ee::s07::DataInstanceMan_ctor,
        }
        .call(c, 0x00277f64)?;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = 0x480000;
        c.g[A0][0] = 0x480000;
        c.g[A1][0] = 0x480000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2100));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x21a0));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2168));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x21b8));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x2ac), c.g[V1][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x518), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x528), c.g[A0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x2a8), c.g[A1][0]);
        'l_277f98: loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            Callee {
                addr: 0x00281018,
                f: ee::s07::ScriptViewObj_ctor,
            }
            .call(c, 0x00277fa4)?;
            let t = c.g[S2][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x120));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00277f98);
                }
                continue 'l_277f98;
            } else {
                c.cycles += 33;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x910));
                c.g[S2][0] = 0x31;
                Callee {
                    addr: 0x0027bfd0,
                    f: ee::s07::ScriptDummyObj_ctor,
                }
                .call(c, 0x00277fb8)?;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x92c));
                Callee {
                    addr: 0x002805b8,
                    f: ee::s07::ScriptAudioObj_ctor,
                }
                .call(c, 0x00277fc0)?;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa28));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0xa;
                c.g[A3][0] = 0xf0;
                Callee {
                    addr: 0x002753b8,
                    f: ee::s07::ScriptAnimBankMan_ctor,
                }
                .call(c, 0x00277fd4)?;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa38));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0xa;
                c.g[A3][0] = 0x11;
                Callee {
                    addr: 0x00283450,
                    f: ee::s07::ScriptSoundBankMan_ctor,
                }
                .call(c, 0x00277fe8)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x52c), 0u64);
                c.g[V1][0] = 0xa;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x530), 0u64);
                c.g[V0][0] = 0x2;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x534), 0u64);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x61c));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x538), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x53c), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x540), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x544), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x548), 0u64);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x3c0), c.g[S1][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0xa48), c.g[V1][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x550), c.g[V0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x54c), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x554), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x620), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x624), 0u64);
                'l_278030: loop {
                    c.cycles += 7;
                    c.sw(c.g[A0][0] as u32, 0u64);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[S2][0] as i64) >= 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00278030);
                        }
                        continue 'l_278030;
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x628));
                        c.g[S2][0] = 0x9;
                        'l_278058: loop {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            Callee {
                                addr: 0x00277f08,
                                f: ee::s07::ScriptBankSlot_Clear,
                            }
                            .call(c, 0x00278064)?;
                            let t = (c.g[S2][0] as i64) >= 0;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00278058);
                                }
                                continue 'l_278058;
                            } else {
                                c.cycles += 3;
                                c.g[S2][0] = 0u64;
                                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                'l_278078: loop {
                                    c.cycles += 7;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    Callee {
                                        addr: 0x00281160,
                                        f: ee::s07::ScriptViewObj_SetViewIndex,
                                    }
                                    .call(c, 0x00278084)?;
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x120));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00278078);
                                        }
                                        continue 'l_278078;
                                    } else {
                                        c.cycles += 8;
                                        c.g[V0][0] = c.g[S1][0];
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                        let t = c.g[RA][0] as u32;
                                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.pc = t;
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

/// `void ScriptList_NextEntry(self)` (0x277838): advances the script list.
/// With a pending entry (`self + 0xbc > 0`) it pops the list entries, clears
/// the pending slot, takes the next command (`ScriptCmdQueue_At(self + 0xc,
/// 0)`) and starts it (a movie via `ScriptMovie_Start(engine + 0x500)`, else
/// `ScriptEngine_PlayScriptInstance` plus the engine's virtual method 6),
/// marks `self + 0xac` when `self + 0xb0` is set and prepares the next entry
/// into `self + 0xbc`. Without one it stops the list and, when `self + 4` is
/// set, fades via the game's transition manager (the remaining time of the
/// running transition when `self + 0xb0` was set, else 0.5s). Frame 0x30.
pub fn script_list_next_entry(c: &mut Cpu) {
    const POP_ENTRIES: Callee = Callee {
        addr: 0x0027_7598,
        f: ee::s07::f_00277598,
    };
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PLAY_SCRIPT_INSTANCE: Callee = Callee {
        addr: 0x0027_9720,
        f: ee::s07::f_00279720,
    };
    const MOVIE_START: Callee = Callee {
        addr: 0x0028_39a8,
        f: ee::s07::f_002839a8,
    };
    const PREPARE_ENTRY: Callee = Callee {
        addr: 0x0027_7450,
        f: ee::s07::f_00277450,
    };
    const LIST_STOP: Callee = Callee {
        addr: 0x0027_6868,
        f: ee::s07::f_00276868,
    };
    const PENDING: u32 = 0xbc;
    const ENABLED: u32 = 0xb0;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        c.g[A1][0] = c.lw(this.wrapping_add(PENDING));
        'ret: {
            'end: {
                if (c.g[A1][0] as i64) > 0 {
                    c.cycles += 14;
                    c.sw(this.wrapping_add(0xc8), 0);
                    POP_ENTRIES.call(c, 0x0027_7860)?;
                    c.g[V0][0] = u64::MAX;
                    c.sw(this.wrapping_add(0xc0), 0);
                    c.g[A0][0] = sx(this.wrapping_add(0xc));
                    c.g[A1][0] = 0;
                    c.sw(this.wrapping_add(PENDING), c.g[V0][0]);
                    CMD_QUEUE_AT.call(c, 0x0027_7878)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    IS_MOVIE_ENTRY.call(c, 0x0027_7888)?;
                    let movie = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lw(this);
                    if !movie {
                        let cmd = r32(c, S1);
                        c.cycles += 16;
                        c.g[T0][0] = c.lw(cmd.wrapping_add(8));
                        c.g[A3][0] = 0;
                        c.g[T0][0] = sx(((c.g[T0][0] as u32) as i32 >> 3) as u32);
                        c.g[A1][0] = c.lw(this.wrapping_add(8));
                        c.g[A2][0] = c.lw(cmd.wrapping_add(0xc));
                        c.g[T0][0] &= 1;
                        PLAY_SCRIPT_INSTANCE.call(c, 0x0027_78ac)?;
                        c.g[A2][0] = c.lw(this);
                        c.g[A1][0] = c.lw(cmd.wrapping_add(0xc));
                        c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x2a8));
                        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x30));
                        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x34));
                        let method = r32(c, V0);
                        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                        call_ptr(c, method, 0x0027_78c8)?;
                        c.g[V0][0] = c.lw(this.wrapping_add(ENABLED));
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x500));
                        MOVIE_START.call(c, 0x0027_78d8)?;
                        c.g[V0][0] = c.lw(this.wrapping_add(ENABLED));
                    }
                    c.cycles += 2;
                    let off = c.g[V0][0] == 0;
                    c.g[V0][0] = 1;
                    if !off {
                        c.cycles += 1;
                        c.sw(this.wrapping_add(0xac), c.g[V0][0]);
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 1;
                    c.g[A2][0] = 0;
                    PREPARE_ENTRY.call(c, 0x0027_78f8)?;
                    c.sw(this.wrapping_add(PENDING), c.g[V0][0]);
                    break 'end;
                }
                // Nothing pending: stop the list and fade out.
                c.g[S1][0] = c.lw(this.wrapping_add(ENABLED));
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                LIST_STOP.call(c, 0x0027_790c)?;
                c.g[V0][0] = c.lw(this.wrapping_add(4));
                let no_fade = c.g[V0][0] == 0;
                c.g[V0][0] = c.lw(oc432_gp_minus(c, 0x848));
                if no_fade {
                    break 'end;
                }
                'fade_half: {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x84));
                    c.g[S0][0] = c.lw(r32(c, A0).wrapping_add(0x64));
                    call_ptr(c, 0x0023_0698, 0x0027_7928)?;
                    let was_off = c.g[S1][0] == 0;
                    c.g[A0][0] = c.g[V0][0];
                    if was_off {
                        break 'fade_half;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x44));
                    if c.g[V0][0] == 0 {
                        break 'fade_half;
                    }
                    c.cycles += 7;
                    call_ptr(c, 0x002e_4678, 0x0027_7944)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.mov_s(12, 0);
                    call_ptr(c, 0x002e_a780, 0x0027_7950)?;
                    frame.restore(c, S0, 0x20);
                    break 'ret;
                }
                c.cycles += 4;
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(AT, 12);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002e_a780, 0x0027_7968)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptList_Prepare(ScriptList* self, int playOnReady)` (0x276270): when the
/// list is idle and has queued entries, prepare the first entry and set the
/// state (1: loading, 2: ready) from the entry's process state; returns the
/// state.
#[allow(unreachable_code)]
pub fn script_list_prepare(c: &mut Cpu) {
    guest(c, |c| {
        'b_2762a8: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = 0x1;
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
            let t = c.g[V0][0] != c.g[S3][0];
            c.g[S1][0] = c.g[A1][0];
            if t {
                break 'b_2762a8;
            } else {
                c.cycles += 3;
                Callee {
                    addr: 0x00277778,
                    f: ee::s07::ScriptList_PollLoading,
                }
                .call(c, 0x002762a4)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                break 'b_2762a8;
            }
        }
        'b_276370: {
            c.cycles += 2;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_276370;
            } else {
                'b_27636c: {
                    'b_2762d0: {
                        c.cycles += 6;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xb4), c.g[S1][0]);
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00283d28,
                            f: ee::s07::ScriptCmdQueue_Count,
                        }
                        .call(c, 0x002762c0)?;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_2762d0;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_27636c;
                            } else {
                                break 'b_2762d0;
                            }
                        }
                    }
                    'b_2762f8: {
                        c.cycles += 6;
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0u64;
                        Callee {
                            addr: 0x00277450,
                            f: ee::s07::ScriptList_PrepareEntry,
                        }
                        .call(c, 0x002762dc)?;
                        c.g[S2][0] = c.g[V0][0];
                        let t = (c.g[S2][0] as i64) >= 0;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_2762f8;
                        } else {
                            c.cycles += 4;
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x00276868,
                                f: ee::s07::ScriptList_Stop,
                            }
                            .call(c, 0x002762f0)?;
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                            if t {
                                break 'b_27636c;
                            } else {
                                break 'b_2762f8;
                            }
                        }
                    }
                    'b_276354: {
                        'b_276350: {
                            'b_276348: {
                                'b_276340: {
                                    c.cycles += 11;
                                    c.g[A1][0] = c.g[S2][0];
                                    Callee {
                                        addr: 0x00277598,
                                        f: ee::s07::ScriptList_PopEntries,
                                    }
                                    .call(c, 0x00276300)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = 0u64;
                                    Callee {
                                        addr: 0x00283d70,
                                        f: ee::s07::ScriptCmdQueue_At,
                                    }
                                    .call(c, 0x0027630c)?;
                                    c.g[S1][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x00277dd8,
                                        f: ee::s07::ScriptList_IsMovieEntry,
                                    }
                                    .call(c, 0x0027631c)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                        break 'b_276340;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                        Callee {
                                            addr: 0x00282798,
                                            f: ee::s07::ScriptEngine_GetProcessState,
                                        }
                                        .call(c, 0x00276330)?;
                                        let t = c.g[V0][0] == c.g[S3][0];
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_276348;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xa4),
                                                c.g[V0][0],
                                            );
                                            if t {
                                                break 'b_276354;
                                            } else {
                                                break 'b_276340;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = (c.g[V0][0] as i64) >= 0;
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_276350;
                                } else {
                                    break 'b_276348;
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xa4), c.g[S3][0]);
                            if t {
                                break 'b_276354;
                            } else {
                                break 'b_276350;
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
                        break 'b_276354;
                    }
                    c.cycles += 6;
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x00277450,
                        f: ee::s07::ScriptList_PrepareEntry,
                    }
                    .call(c, 0x00276364)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                    break 'b_27636c;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_276370;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `void ScriptList_SkipCurrent(ScriptList* self)` (0x276f48): when the playing
/// entry is skippable (`flags & 1`), mark the skip (`+0xc8`), re-prepare the next
/// entry (killing or stopping the pending one) and advance the list.
#[allow(unreachable_code)]
pub fn script_list_skip_current(c: &mut Cpu) {
    guest(c, |c| {
        'b_27704c: {
            'b_277048: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[V1][0] = 0x3;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                let t = c.g[V0][0] != c.g[V1][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_277048;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc8));
                    if c.g[V0][0] != 0u64 {
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_27704c;
                    } else {
                        c.cycles += 9;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x00283d70,
                            f: ee::s07::ScriptCmdQueue_At,
                        }
                        .call(c, 0x00276f8c)?;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.g[V1][0] ^ 0x1;
                        c.g[V1][0] = c.g[V1][0] & 0x1;
                        if c.g[V1][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_277048;
                        } else {
                            'b_276fc8: {
                                c.cycles += 4;
                                c.g[V0][0] = 0x1;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xbc));
                                let t = (c.g[A1][0] as i64) >= 0;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0xc8), c.g[V0][0]);
                                if t {
                                    break 'b_276fc8;
                                } else {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = 0x1;
                                    c.g[A2][0] = 0x1;
                                    Callee {
                                        addr: 0x00277450,
                                        f: ee::s07::ScriptList_PrepareEntry,
                                    }
                                    .call(c, 0x00276fc0)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_277048;
                                    } else {
                                        break 'b_276fc8;
                                    }
                                }
                            }
                            'b_277034: {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A2][0] = 0x1;
                                Callee {
                                    addr: 0x00277450,
                                    f: ee::s07::ScriptList_PrepareEntry,
                                }
                                .call(c, 0x00276fd4)?;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xbc));
                                c.g[S2][0] = c.g[V0][0];
                                let t = c.g[S2][0] == c.g[A1][0];

                                if t {
                                    break 'b_277034;
                                } else {
                                    'b_277030: {
                                        'b_27701c: {
                                            c.cycles += 8;
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x00283d70,
                                                f: ee::s07::ScriptCmdQueue_At,
                                            }
                                            .call(c, 0x00276fec)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A1][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x00277dd8,
                                                f: ee::s07::ScriptList_IsMovieEntry,
                                            }
                                            .call(c, 0x00276ffc)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                break 'b_27701c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                Callee {
                                                    addr: 0x00282540,
                                                    f: ee::s07::ScriptEngine_KillProcess,
                                                }
                                                .call(c, 0x00277010)?;
                                                c.g[V1][0] = 0xffffffffffffffff;
                                                let t = true;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xc),
                                                    c.g[V1][0],
                                                );
                                                if t {
                                                    break 'b_277030;
                                                } else {
                                                    break 'b_27701c;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if (c.g[V0][0] as i64) < 0 {
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0xbc),
                                                c.g[S2][0],
                                            );
                                            break 'b_277034;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x500));
                                            Callee {
                                                addr: 0x00283818,
                                                f: ee::s07::ScriptMovie_Stop,
                                            }
                                            .call(c, 0x00277030)?;
                                            break 'b_277030;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0xbc), c.g[S2][0]);
                                    break 'b_277034;
                                }
                            }
                            c.cycles += 2;
                            let t = (c.g[S2][0] as i64) < 0;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_277048;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x00277800,
                                    f: ee::s07::ScriptList_TryAdvance,
                                }
                                .call(c, 0x00277044)?;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                break 'b_277048;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_27704c;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `void ScriptEngine_LoadMaster(ScriptEngine* self)` (0x278210): loads the script master
/// table (file names at `gp + 0x3a4` and `gp + 0x3a8`): `+0x52c` header, `+0x530`
/// script entries (0xc bytes each, offset at +8), `+0x534` the debug base. Offsets are
/// rebased on the debug file, or cleared without one. Then sets up the BX engine
/// (`ScriptEngine_CreateScriptDatabase(self, count, 10, 0x4000)`).
pub fn script_engine_load_master(c: &mut Cpu) {
    guest(c, |c| {
        'b_2782d8: {
            'b_2782d4: {
                'b_278298: {
                    c.cycles += 15;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.g[A1][0] = 0u64;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x3a4));
                    call_ptr(c, 0x003e1908, 0x0027822c)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x3a8));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x52c), c.g[V0][0]);
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x530), c.g[V1][0]);
                    call_ptr(c, 0x003e1908, 0x00278244)?;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x534), c.g[V0][0]);
                    if t {
                        break 'b_278298;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x52c));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_2782d4;
                        } else {
                            c.cycles += 1;
                            c.g[A2][0] = 0u64;
                            'l_278260: loop {
                                c.cycles += 12;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x530));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x534));
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x52c));
                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00278260);
                                    }
                                    continue 'l_278260;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x52c));
                                    if t {
                                        break 'b_2782d8;
                                    } else {
                                        break 'b_278298;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x52c));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = 0u64;
                if t {
                    break 'b_2782d4;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = 0u64;
                    'l_2782b0: loop {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x530));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x52c));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002782b0);
                            }
                            continue 'l_2782b0;
                        } else {
                            break 'b_2782d4;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x52c));
            break 'b_2782d8;
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x554), 0u64);
        c.g[A2][0] = 0xa;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x620), 0u64);
        c.g[A3][0] = 0x4000;
        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
        Callee {
            addr: 0x00281f30,
            f: ee::s07::ScriptEngine_CreateScriptDatabase,
        }
        .call(c, 0x002782f4)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `int DataInstanceMan_RemoveData(DataInstanceMan* self, Node* entry)` (0x275a20): removes
/// an entry and its chain of linked nodes (`+0x14`): each is passed to the manager's
/// virtual `remove` (vtable slot at `+0x1c` or `+0x2c` of the manager's vtable) and
/// returned to the free list (`DataInstanceMan_FreeInfo`). Returns the key of the
/// last node (`+0x8`).
pub fn data_instance_man_remove_data(c: &mut Cpu) {
    guest(c, |c| {
        'b_275adc: {
            'b_275ab4: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S3][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                let t = c.g[A1][0] == 0u64;
                c.g[S2][0] = c.g[A0][0];
                if t {
                    break 'b_275ab4;
                } else {
                    c.cycles += 2;
                    c.g[S1][0] = c.g[A1][0];
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    'l_275a50: loop {
                        c.cycles += 15;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A3][0] = c.lw(c.g[S1][0] as u32);
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                        let t = c.g[V1][0] as u32;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        call_ptr(c, t, 0x00275a74)?;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x00275cd0,
                            f: ee::s07::DataInstanceMan_FreeInfo,
                        }
                        .call(c, 0x00275a80)?;
                        c.g[S1][0] = c.g[S0][0];
                        let t = c.g[S1][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00275a50);
                            }
                            continue 'l_275a50;
                        } else {
                            c.cycles += 10;
                            c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                            let t = c.g[V1][0] as u32;
                            c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                            call_ptr(c, t, 0x00275aac)?;
                            let t = true;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                            if t {
                                break 'b_275adc;
                            } else {
                                break 'b_275ab4;
                            }
                        }
                    }
                }
            }
            c.cycles += 10;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A3][0] = c.lw(c.g[S3][0] as u32);
            let t = c.g[V1][0] as u32;
            c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
            call_ptr(c, t, 0x00275ad8)?;
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            break 'b_275adc;
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S3][0];
        Callee {
            addr: 0x00275cd0,
            f: ee::s07::DataInstanceMan_FreeInfo,
        }
        .call(c, 0x00275ae8)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `ScriptList.pending` (non-zero: an entry is already being processed).
pub(crate) const OC437_SCRIPT_LIST_PENDING: u32 = 0xa8;

/// `ScriptList_TryAdvance(list)` (0x277800): `ScriptList_CheckNextReady(list)`;
/// if `list->pending` (+0xa8) is zero, `ScriptList_NextEntry(list)`.
pub fn script_list_try_advance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0027_76e0,
            f: ee::s07::ScriptList_CheckNextReady,
        }
        .call(c, 0x0027_7814)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(OC437_SCRIPT_LIST_PENDING)).lw(c);
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0027_7838,
                f: ee::s07::ScriptList_NextEntry,
            }
            .call(c, 0x0027_7828)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptList_GetFieldB0(ScriptList* self)` (0x2772b8): the word at +0xb0.
pub fn script_list_get_field_b0(c: &mut Cpu) {
    c.cycles += 2;
    let this = c.g[A0][0] as u32;
    c.g[V0][0] = c.lw(this.wrapping_add(0xb0));
    ret(c);
}

/// `ScriptList_SetPausedNested` (0x2766b0): `ScriptList_SetPaused(list, a1, 0)`, a tail
/// wrapper that clears the nested flag.
pub fn script_list_set_paused_nested(c: &mut Cpu) {
    const SET_PAUSED: Callee = Callee {
        addr: 0x0027_7c08,
        f: ee::s07::f_00277c08,
    };
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        SET_PAUSED.call(c, 0x0027_66c0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptList_GetEntryExtType` (0x276a08): the extension type of queue entry `index`
/// (`a1`) of a script list, when that entry is in the queue and its type word is below 29
/// (`29` is the first non-movie type). Otherwise 0x20. The queue is the list at +0xc.
pub fn script_list_get_entry_ext_type(c: &mut Cpu) {
    const CMD_QUEUE_COUNT: Callee = Callee {
        addr: 0x0028_3d28,
        f: ee::s07::f_00283d28,
    };
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        CMD_QUEUE_COUNT.call(c, 0x0027_6a28)?;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A0][0] = c.g[S1][0];
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0x20;
        } else {
            c.cycles += 6;
            c.g[A1][0] = c.g[S0][0];
            CMD_QUEUE_AT.call(c, 0x0027_6a3c)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = ((c.g[V1][0] as i64) < 29) as u64;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = 0x20;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0027_6a58);
                c.g[A1][0] = c.g[S0][0];
                CMD_QUEUE_AT.call(c, 0x0027_6a58)?;
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptList_Count` (0x276ca8): the number of queued commands of a script list
/// (`ScriptCmdQueue_Count` of the queue at +0xc).
pub fn script_list_count(c: &mut Cpu) {
    const CMD_QUEUE_COUNT: Callee = Callee {
        addr: 0x0028_3d28,
        f: ee::s07::f_00283d28,
    };
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
        CMD_QUEUE_COUNT.call(c, 0x0027_6cb8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `bool ScriptList_IsFull(ScriptList* list)` (0x2771a8): whether the command
/// queue embedded at `list + 0xc` (a `cCircularBuffer`) is full.
pub fn script_list_is_full(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0xc));
        Callee {
            addr: 0x0028_3c58,
            f: ee::s07::f_00283c58,
        }
        .call(c, 0x0027_71b8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool ScriptList_IsPlayingMovie(ScriptList* list)` (0x277248): false unless
/// the list's state word at `+0xa4` is 3 (a movie is playing); then whether the
/// entry at the queue head (`ScriptCmdQueue_At(list + 0xc, 0)`) is a movie entry.
pub fn script_list_is_playing_movie(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 3;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xa4)).lw(c);
        let playing = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = 0;
        if playing {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = 0;
            Callee {
                addr: 0x0028_3d70,
                f: ee::s07::f_00283d70,
            }
            .call(c, 0x0027_7274)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            Callee {
                addr: 0x0027_7dd8,
                f: ee::s07::f_00277dd8,
            }
            .call(c, 0x0027_7280)?;
        } else {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The address `off(reg)`, for a register read before a call or a store.
pub(crate) fn oc538_field(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr((c.g[reg][0] as u32).wrapping_add(off))
}

/// `ScriptList_AddMovie(self, controlBits, scriptIndex, a2, a3, t0, t1)` (0x276048,
/// `tNISList_AddByScriptIndex`): queues a script command record (category 0x1c) on the
/// list's command queue (+0xc). When the list is idle (state 0), the call returns 1. When
/// it has a pending entry (`nextPending >= 0`), returns 1 too. Otherwise the entry is
/// prepared, and when the list is in state 3 the entry is checked (movie or request
/// player) and `ScriptList_CheckNextReady` decides whether it is ready. Returns 0 when the
/// queue is full.
pub fn script_list_add_movie(c: &mut Cpu) {
    const CMD_QUEUE_IS_FULL: Callee = Callee {
        addr: 0x0028_3c58,
        f: ee::s07::f_00283c58,
    };
    const CMD_QUEUE_PUSH: Callee = Callee {
        addr: 0x0028_3c80,
        f: ee::s07::f_00283c80,
    };
    const CMD_QUEUE_COUNT: Callee = Callee {
        addr: 0x0028_3d28,
        f: ee::s07::f_00283d28,
    };
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const PREPARE_ENTRY: Callee = Callee {
        addr: 0x0027_7450,
        f: ee::s07::f_00277450,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PROCESS_IN_SEQ: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::f_00282bf0,
    };
    const MOVIE_GET_ID: Callee = Callee {
        addr: 0x0028_3bb8,
        f: ee::s07::f_00283bb8,
    };
    const CHECK_NEXT_READY: Callee = Callee {
        addr: 0x0027_76e0,
        f: ee::s07::f_002776e0,
    };
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x50);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[T0][0];
        frame.save(c, S6, 0x30);
        c.g[S5][0] = c.g[T1][0];
        frame.save_ra(c, 0x20);
        c.g[S6][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S1][0];
        CMD_QUEUE_IS_FULL.call(c, 0x0027_6090)?;
        let full = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !full {
            c.cycles += 15;
            c.g[V1][0] = u64::MAX;
            c.g[V0][0] = 0x1c;
            let v1 = c.g[V1][0];
            Addr(sp(c).wrapping_add(0xc)).sw(c, v1);
            c.g[A0][0] = c.g[S1][0];
            let category = c.g[V0][0];
            Addr(sp(c)).sw(c, category);
            c.g[A1][0] = c.g[SP][0];
            let (s2, s6, s3, s4, s5) = (c.g[S2][0], c.g[S6][0], c.g[S3][0], c.g[S4][0], c.g[S5][0]);
            Addr(sp(c).wrapping_add(0x8)).sw(c, s2);
            Addr(sp(c).wrapping_add(0x4)).sw(c, s6);
            Addr(sp(c).wrapping_add(0x10)).sw(c, s3);
            Addr(sp(c).wrapping_add(0x14)).sw(c, s4);
            Addr(sp(c).wrapping_add(0x18)).sw(c, s5);
            CMD_QUEUE_PUSH.call(c, 0x0027_60c8)?;
            c.g[V1][0] = oc538_field(c, S0, 0xa4).lw(c);
            let idle = c.g[V1][0] == 0;
            c.g[V0][0] = 1;
            if !idle {
                c.cycles += 3;
                c.g[V0][0] = oc538_field(c, S0, 0xbc).lw(c);
                let pending = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = 1;
                if !pending {
                    // Prepare the next entry; state 3 checks it.
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S1][0];
                    CMD_QUEUE_COUNT.call(c, 0x0027_60e8)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[A2][0] = 0;
                    PREPARE_ENTRY.call(c, 0x0027_60f8)?;
                    c.g[A0][0] = oc538_field(c, S0, 0xa4).lw(c);
                    c.g[V1][0] = 3;
                    let not_three = c.g[A0][0] != c.g[V1][0];
                    let prepared = c.g[V0][0];
                    oc538_field(c, S0, 0xbc).sw(c, prepared);
                    if not_three {
                        c.cycles += 1;
                        c.g[V0][0] = 1;
                    } else {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = 0;
                        CMD_QUEUE_AT.call(c, 0x0027_6114)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S0][0];
                        IS_MOVIE_ENTRY.call(c, 0x0027_6120)?;
                        let movie = c.g[V0][0] != 0;
                        let mut get_id = true;
                        if movie {
                            c.g[A0][0] = oc538_field(c, S0, 0).lw(c);
                        } else {
                            c.cycles += 10;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = 0;
                            CMD_QUEUE_AT.call(c, 0x0027_6134)?;
                            c.g[A1][0] = oc538_field(c, V0, 0xc).lw(c);
                            c.g[A0][0] = oc538_field(c, S0, 0).lw(c);
                            PROCESS_IN_SEQ.call(c, 0x0027_6140)?;
                            c.g[V1][0] = oc538_field(c, V0, 0x20).lw(c);
                            c.g[V1][0] ^= 4;
                            c.g[V0][0] = (c.g[V1][0] < 1) as u64;
                            get_id = false;
                        }
                        if get_id {
                            c.cycles += 2;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
                            MOVIE_GET_ID.call(c, 0x0027_6158)?;
                        }
                        c.cycles += 2;
                        let no_request = c.g[V0][0] == 0;
                        c.g[V0][0] = 1;
                        if !no_request {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S0][0];
                            CHECK_NEXT_READY.call(c, 0x0027_6168)?;
                            c.cycles += 1;
                            c.g[V0][0] = 1;
                        }
                    }
                }
            }
        }
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
        Ok(())
    });
}

/// `ScriptList_GetEntryScriptIndex(self, i)` (0x276a78): the script index of queue entry
/// `i`, or -1 when `i` is out of range or the entry is a movie. An entry whose own index
/// (+4) is negative takes the index of its request (`ScriptEngine_GetProcessScriptId`)
/// when it has a handle (+0xc).
pub fn script_list_get_entry_script_index(c: &mut Cpu) {
    const CMD_QUEUE_COUNT: Callee = Callee {
        addr: 0x0028_3d28,
        f: ee::s07::f_00283d28,
    };
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PROCESS_SCRIPT_ID: Callee = Callee {
        addr: 0x0028_27f8,
        f: ee::s07::f_002827f8,
    };
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        CMD_QUEUE_COUNT.call(c, 0x0027_6aa0)?;
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S0][0];
        let mut script_entry = false;
        if in_range {
            c.cycles += 8;
            c.g[A1][0] = c.g[S2][0];
            CMD_QUEUE_AT.call(c, 0x0027_6ab4)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            IS_MOVIE_ENTRY.call(c, 0x0027_6ac4)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = oc538_field(c, S0, 0x4).lw(c);
                script_entry = true;
            }
        }
        if script_entry {
            c.cycles += 2;
            if (c.g[V0][0] as i64) >= 0 {
                frame.restore(c, S0, 0x30);
            } else {
                c.cycles += 3;
                c.g[A1][0] = oc538_field(c, S0, 0xc).lw(c);
                let no_handle = (c.g[A1][0] as i64) < 0;
                c.g[V0][0] = u64::MAX;
                if !no_handle {
                    c.cycles += 2;
                    c.g[A0][0] = oc538_field(c, S1, 0).lw(c);
                    PROCESS_SCRIPT_ID.call(c, 0x0027_6af0)?;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
        } else {
            // Out of range or a movie entry: -1.
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptList_GetEntryScript(self, i)` (0x276b08): the request player of queue entry `i`
/// (`ScriptEngine_GetProcessInSeq` of its handle) when `i` is in range, the entry is not a
/// movie and its handle (+0xc) is set. Otherwise 0.
pub fn script_list_get_entry_script(c: &mut Cpu) {
    const CMD_QUEUE_COUNT: Callee = Callee {
        addr: 0x0028_3d28,
        f: ee::s07::f_00283d28,
    };
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PROCESS_IN_SEQ: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::f_00282bf0,
    };
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        CMD_QUEUE_COUNT.call(c, 0x0027_6b30)?;
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S0][0];
        if !in_range {
            c.cycles += 1;
            c.g[V0][0] = 0;
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 8;
            c.g[A1][0] = c.g[S2][0];
            CMD_QUEUE_AT.call(c, 0x0027_6b44)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            IS_MOVIE_ENTRY.call(c, 0x0027_6b54)?;
            let movie = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if movie {
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            } else {
                c.cycles += 3;
                c.g[A1][0] = oc538_field(c, S0, 0xc).lw(c);
                let no_handle = (c.g[A1][0] as i64) < 0;
                frame.restore(c, S0, 0x30);
                if !no_handle {
                    c.cycles += 4;
                    c.g[A0][0] = oc538_field(c, S1, 0).lw(c);
                    PROCESS_IN_SEQ.call(c, 0x0027_6b70)?;
                    frame.restore(c, S0, 0x30);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptList_IsNextReady(self)` (0x277100): 0 while the list is in state 1, 1 when there
/// is no pending entry (`nextPending < 0`). Otherwise the pending entry is checked: a movie
/// is ready when its handle (+0xc) is non-negative, anything else when its request is in
/// state 2 (`ScriptEngine_GetProcessState`).
pub fn script_list_is_next_ready(c: &mut Cpu) {
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PROCESS_STATE: Callee = Callee {
        addr: 0x0028_2798,
        f: ee::s07::f_00282798,
    };
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = oc538_field(c, S0, 0xa4).lw(c);
        if c.g[V0][0] == c.g[V1][0] {
            // State 1: not ready.
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.g[A1][0] = oc538_field(c, S0, 0xbc).lw(c);
            c.cycles += 2;
            let pending = (c.g[A1][0] as i64) >= 0;
            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            if !pending {
                c.cycles += 2;
                c.g[V0][0] = 1;
            } else {
                c.cycles += 8;
                c.g[A0][0] = c.g[S2][0];
                CMD_QUEUE_AT.call(c, 0x0027_7148)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                IS_MOVIE_ENTRY.call(c, 0x0027_7158)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = oc538_field(c, S1, 0xc).lw(c);
                    c.cycles += 2;
                    c.g[V0][0] = !c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = oc538_field(c, S0, 0xbc).lw(c);
                    c.g[A0][0] = c.g[S2][0];
                    CMD_QUEUE_AT.call(c, 0x0027_716c)?;
                    c.g[A0][0] = oc538_field(c, S0, 0).lw(c);
                    c.g[A1][0] = oc538_field(c, V0, 0xc).lw(c);
                    PROCESS_STATE.call(c, 0x0027_7178)?;
                    c.g[V0][0] ^= 2;
                    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
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

/// `ScriptList_GetMovieInfo(list, key)` (0x277de8): the movie record for the
/// entry `key` (its type word `*key` indexes 28-byte records). It calls the BE
/// library (`cBE_getInterface_Fv(cBE_getBE(), 4)`, whose result is not used),
/// copies the 0x288-byte table at 0x535610 onto the stack, and returns
/// `&D_0048247C[*key]`, or `&D_004823DC[*key]` when the table's first word,
/// masked with 0x300000, equals 0x200000.
pub fn script_list_get_movie_info(c: &mut Cpu) {
    const BE_GET_INSTANCE: u32 = 0x0014_dc80;
    const BE_GET_INTERFACE: u32 = 0x0014_dd58;
    const TABLE: u32 = 0x0053_5610;
    const TABLE_LEN: u32 = 0x280;
    const RECORDS_A: u32 = 0x0048_247c;
    const RECORDS_B: u32 = 0x0048_23dc;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x2b0);
        frame.save(c, S0, 0x2a0);
        frame.save_ra(c, 0x290);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, BE_GET_INSTANCE, 0x0027_7dfc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        call_ptr(c, BE_GET_INTERFACE, 0x0027_7e08)?;
        // The table is 8-byte aligned (0x535610 & 7 == 0), so the generated
        // unaligned copy path (ldl/ldr) never runs: only the aligned loop does.
        c.g[V1][0] = sx(TABLE);
        c.g[A0][0] = c.g[SP][0];
        c.cycles += 1;
        c.g[V0][0] = sx(TABLE.wrapping_add(TABLE_LEN));
        // The copy: 0x20 bytes per pass from the table to the stack buffer.
        loop {
            c.cycles += 11;
            let src = c.g[V1][0] as u32;
            let dst = c.g[A0][0] as u32;
            c.g[A1][0] = Addr(src).ld(c);
            c.g[A2][0] = Addr(src.wrapping_add(8)).ld(c);
            c.g[A3][0] = Addr(src.wrapping_add(0x10)).ld(c);
            c.g[T0][0] = Addr(src.wrapping_add(0x18)).ld(c);
            let words = [c.g[A1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]];
            for (i, w) in words.into_iter().enumerate() {
                Addr(dst.wrapping_add(8 * i as u32)).sd(c, w);
            }
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_7e78);
            }
        }
        // The last 8 bytes (0x535890) go to sp+0x280.
        c.cycles += 10;
        let tail = Addr(c.g[V1][0] as u32).ld(c);
        c.g[V0][0] = tail;
        Addr(c.g[A0][0] as u32).sd(c, tail);
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[V0][0] = 0x0030_0000;
        c.g[A0][0] = 0x0020_0000;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        let t = c.g[V1][0] != c.g[A0][0];
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.cycles += if t { 4 } else { 5 };
        c.g[A0][0] = 0x1c;
        c.mult(0, V1, V1, A0);
        c.g[V0][0] = sx(if t { RECORDS_A } else { RECORDS_B });
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore(c, S0, 0x2a0);
        frame.restore_ra(c, 0x290);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptList_SetPaused(list, on, hard)` (0x277c08), the body of
/// `tNISList::DoPause`. In a playing list (state 3) a hard request sets the
/// paused flag (+0xa8) and a soft one counts the pauses (+0xc4). When nothing
/// else holds the list, the current entry is paused or resumed (a movie
/// through the movie player, a script through the engine), along with the
/// cutscene transition when the list has one (+4).
pub fn script_list_set_paused(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 3;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let list = c.g[S0][0] as u32;
        c.g[V1][0] = Addr(list).off(0xa4).lw(c);
        if c.g[V1][0] != c.g[V0][0] {
            set_paused_exit_s0(c, frame);
            return Ok(());
        }
        c.cycles += 2;
        if c.g[A2][0] != 0 {
            // Hard: the paused flag must change, and no count may be pending.
            c.g[V0][0] = Addr(list).off(0xa8).lw(c);
            c.cycles += 2;
            if c.g[V0][0] == c.g[A1][0] {
                set_paused_exit_s0(c, frame);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = Addr(list).off(0xc4).lw(c);
            let on = c.g[A1][0];
            Addr(list).off(0xa8).sw(c, on);
        } else {
            // Soft: count the pauses; a nested pause needs no action.
            c.cycles += 2;
            let on = c.g[A1][0] != 0;
            c.g[V1][0] = Addr(list).off(0xc4).lw(c);
            if on {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                let nested = c.g[V1][0] != 0;
                let count = c.g[V0][0];
                Addr(list).off(0xc4).sw(c, count);
                if nested {
                    set_paused_exit_dc4(c, frame);
                    return Ok(());
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(list).off(0xa8).lw(c);
            } else {
                c.cycles += 2;
                let none_left = (c.g[V1][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                if none_left {
                    set_paused_exit_dc4(c, frame);
                    return Ok(());
                }
                c.cycles += 2;
                let still = c.g[V0][0] != 0;
                let count = c.g[V0][0];
                Addr(list).off(0xc4).sw(c, count);
                if still {
                    set_paused_exit_dc4(c, frame);
                    return Ok(());
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(list).off(0xa8).lw(c);
            }
        }
        // The paused flag (soft) or the count (hard) must be clear to go on.
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            set_paused_exit_s0(c, frame);
            return Ok(());
        }
        c.cycles += 2;
        let on = c.g[A1][0] != 0;
        c.g[A0][0] = sx(list.wrapping_add(0xc));
        if on {
            set_paused_on(c, frame, list)?;
        } else {
            set_paused_off(c, frame, list)?;
        }
        Ok(())
    });
}

/// The pause branch of [`script_list_set_paused`] (`on` != 0). Always ends
/// with the epilogue (or the one-cycle-longer `dc4` exit).
pub(crate) fn set_paused_on(c: &mut Cpu, frame: Frame, list: u32) -> Flow {
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PROCESS_STATE: Callee = Callee {
        addr: 0x0028_2798,
        f: ee::s07::f_00282798,
    };
    const PAUSE_PROCESS: Callee = Callee {
        addr: 0x0028_26d8,
        f: ee::s07::f_002826d8,
    };
    const MOVIE_SET_PAUSED: Callee = Callee {
        addr: 0x0028_3b78,
        f: ee::s07::f_00283b78,
    };
    c.cycles += 8;
    c.g[A1][0] = 0;
    CMD_QUEUE_AT.call(c, 0x0027_7c90)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    IS_MOVIE_ENTRY.call(c, 0x0027_7ca0)?;
    let movie = c.g[V0][0] != 0;
    c.g[A0][0] = Addr(list).lw(c);
    if movie {
        c.cycles += 4;
        c.g[A1][0] = 1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
        MOVIE_SET_PAUSED.call(c, 0x0027_7ce0)?;
        c.g[V0][0] = Addr(list).off(4).lw(c);
    } else {
        c.cycles += 6;
        c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
        PROCESS_STATE.call(c, 0x0027_7cb0)?;
        let state_less_3 = sx((c.g[V0][0] as u32).wrapping_sub(3));
        c.g[V0][0] = (state_less_3 < 3) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(list).off(4).lw(c);
        } else {
            c.cycles += 5;
            c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
            c.g[A0][0] = Addr(list).lw(c);
            PAUSE_PROCESS.call(c, 0x0027_7ccc)?;
            c.g[V0][0] = Addr(list).off(4).lw(c);
        }
    }
    // The cutscene handler (+4) pauses its transition too.
    c.cycles += 2;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    if c.g[V0][0] == 0 {
        set_paused_exit_dc4(c, frame);
        return Ok(());
    }
    c.cycles += 12;
    c.g[A1][0] = 1;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x64)).lw(c);
    call_ptr(c, 0x002e_a8b8, 0x0027_7cfc)?;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A1][0] = 0;
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
    call_ptr(c, 0x0023_0698, 0x0027_7d0c)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x44).lw(c);
    let idle = c.g[V0][0] == 0;
    frame.restore(c, S0, 0x20);
    if idle {
        set_paused_epilogue(c, frame);
        return Ok(());
    }
    c.cycles += 4;
    c.g[A1][0] = 1;
    call_ptr(c, 0x002e_4638, 0x0027_7d24)?;
    frame.restore(c, S0, 0x20);
    set_paused_epilogue(c, frame);
    Ok(())
}

/// The resume branch of [`script_list_set_paused`] (`on` == 0).
pub(crate) fn set_paused_off(c: &mut Cpu, frame: Frame, list: u32) -> Flow {
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PROCESS_STATE: Callee = Callee {
        addr: 0x0028_2798,
        f: ee::s07::f_00282798,
    };
    const RESUME_PROCESS: Callee = Callee {
        addr: 0x0028_2720,
        f: ee::s07::f_00282720,
    };
    const MOVIE_SET_PAUSED: Callee = Callee {
        addr: 0x0028_3b78,
        f: ee::s07::f_00283b78,
    };
    c.cycles += 8;
    c.g[A1][0] = 0;
    CMD_QUEUE_AT.call(c, 0x0027_7d34)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    IS_MOVIE_ENTRY.call(c, 0x0027_7d44)?;
    let movie = c.g[V0][0] != 0;
    c.g[A0][0] = Addr(list).lw(c);
    if movie {
        c.cycles += 4;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
        MOVIE_SET_PAUSED.call(c, 0x0027_7d80)?;
        c.g[V0][0] = Addr(list).off(4).lw(c);
    } else {
        c.cycles += 5;
        c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
        PROCESS_STATE.call(c, 0x0027_7d54)?;
        c.g[V1][0] = 5;
        if c.g[V0][0] != c.g[V1][0] {
            c.g[V0][0] = Addr(list).off(4).lw(c);
        } else {
            c.cycles += 5;
            c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
            c.g[A0][0] = Addr(list).lw(c);
            RESUME_PROCESS.call(c, 0x0027_7d6c)?;
            c.g[V0][0] = Addr(list).off(4).lw(c);
        }
    }
    c.cycles += 2;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    if c.g[V0][0] == 0 {
        set_paused_exit_dc4(c, frame);
        return Ok(());
    }
    c.cycles += 12;
    c.g[A1][0] = 0;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x64)).lw(c);
    call_ptr(c, 0x002e_a8b8, 0x0027_7d9c)?;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A1][0] = 0;
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
    call_ptr(c, 0x0023_0698, 0x0027_7dac)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x44).lw(c);
    let idle = c.g[V0][0] == 0;
    frame.restore(c, S0, 0x20);
    if idle {
        set_paused_epilogue(c, frame);
        return Ok(());
    }
    c.cycles += 2;
    c.g[A1][0] = 0;
    call_ptr(c, 0x002e_4638, 0x0027_7dc4)?;
    set_paused_exit_dc4(c, frame);
    Ok(())
}

/// `break 'b_277dc8` after restoring s0 (the early exits of SetPaused).
pub(crate) fn set_paused_exit_s0(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, 0x20);
    set_paused_epilogue(c, frame);
}

/// `break 'b_277dc4`: one more cycle, then the s0 exit.
pub(crate) fn set_paused_exit_dc4(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    set_paused_exit_s0(c, frame);
}

/// The epilogue: restore s1 and ra, pop the frame, return.
pub(crate) fn set_paused_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `ScriptList_StartTransition(list)` (0x277980): starts the transition into
/// the list's next entry (a movie's record, or a script's process info), or
/// reports its progress. Returns the list's transition state (+0xb0) or 0/1
/// while a stop or a wait is pending.
pub fn script_list_start_transition(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let list = c.g[S1][0] as u32;
        c.g[V0][0] = Addr(list).off(0xb0).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = sx(list.wrapping_add(0xc));
        if !idle {
            c.cycles += 3;
            c.g[V0][0] = Addr(list).off(0xac).lw(c);
            let waiting = c.g[V0][0] == 0;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            if !waiting {
                // A resume is pending: stop the transition manager.
                c.cycles += 7;
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
                call_ptr(c, 0x0023_0698, 0x0027_79c0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002e_4578, 0x0027_79c8)?;
                c.g[V0][0] = 0;
                start_transition_epilogue(c, frame);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            start_transition_epilogue(c, frame);
            return Ok(());
        }
        if start_transition_main(c)? {
            c.cycles += 1;
            c.g[V0][0] = Addr(list).off(0xb0).lw(c);
        }
        start_transition_epilogue(c, frame);
        Ok(())
    });
}

/// The idle path: the next entry of the queue. `Ok(true)` ends on the normal
/// tail (`break 'b_277be8`), `Ok(false)` returns at once with v0 set.
pub(crate) fn start_transition_main(c: &mut Cpu) -> Result<bool, Unwound> {
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    c.cycles += 8;
    c.g[A1][0] = 0;
    CMD_QUEUE_AT.call(c, 0x0027_79e0)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    IS_MOVIE_ENTRY.call(c, 0x0027_79f0)?;
    let movie = c.g[V0][0] != 0;
    c.g[A1][0] = c.g[S0][0];
    if movie {
        start_transition_movie(c)
    } else {
        start_transition_script(c)
    }
}

/// A movie entry: its record (from `ScriptList_GetMovieInfo`) gives the
/// duration (+0xc, a float), the transition kind (+4) and the target (+8).
pub(crate) fn start_transition_movie(c: &mut Cpu) -> Result<bool, Unwound> {
    const GET_MOVIE_INFO: Callee = Callee {
        addr: 0x0027_7de8,
        f: ee::s07::f_00277de8,
    };
    const TRANSITION_TARGET: Callee = Callee {
        addr: 0x0027_a4a0,
        f: ee::s07::f_0027a4a0,
    };
    c.cycles += 11;
    c.g[A0][0] = c.g[S1][0];
    GET_MOVIE_INFO.call(c, 0x0027_7b48)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
    call_ptr(c, 0x0023_0698, 0x0027_7b5c)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x44).lw(c);
    if c.g[V0][0] == 0 {
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
    } else {
        c.cycles += 3;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002e_4578, 0x0027_7b74)?;
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
    }
    // The duration must be positive (`0.0 < f1`, latched in the FCC bit).
    c.cycles += 5;
    c.f[0] = 0;
    let zero = FpuF32(c.f[0]);
    let duration = FpuF32(c.f[1]);
    let positive = crate::abi::c_lt_s(c, zero, duration);
    c.g[V0][0] = 7;
    if !positive {
        return Ok(true);
    }
    c.cycles += 3;
    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
    let captured = c.g[A1][0] == c.g[V0][0];
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x84c)).lw(c);
    if captured {
        c.cycles += 10;
        c.g[A1][0] = 7;
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
        TRANSITION_TARGET.call(c, 0x0027_7bcc)?;
        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        c.g[A3][0] = 0;
        call_ptr(c, 0x002e_4540, 0x0027_7be4)?;
        Addr(c.g[S1][0] as u32).off(0xb8).sw(c, 0);
        return Ok(true);
    }
    c.cycles += 10;
    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[A2][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
    TRANSITION_TARGET.call(c, 0x0027_7ba4)?;
    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 2;
    call_ptr(c, 0x002e_44f0, 0x0027_7bb8)?;
    Addr(c.g[S1][0] as u32).off(0xb8).sw(c, 0);
    Ok(true)
}

/// A script entry: its process (`ScriptEngine_GetProcessInSeq`) gives the
/// transition info (`ScriptData_GetInfo`): three durations (+0x12, +0x14,
/// +0x16), a kind (+0x10) and a target (+0x11).
pub(crate) fn start_transition_script(c: &mut Cpu) -> Result<bool, Unwound> {
    const PROCESS_IN_SEQ: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::f_00282bf0,
    };
    const SCRIPT_DATA_INFO: Callee = Callee {
        addr: 0x0027_c070,
        f: ee::s07::f_0027c070,
    };
    const TRANSITION_TARGET: Callee = Callee {
        addr: 0x0027_a4a0,
        f: ee::s07::f_0027a4a0,
    };
    c.cycles += 14;
    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x84c)).lw(c);
    PROCESS_IN_SEQ.call(c, 0x0027_7a04)?;
    c.g[A0][0] = c.g[V0][0];
    SCRIPT_DATA_INFO.call(c, 0x0027_7a0c)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
    call_ptr(c, 0x0023_0698, 0x0027_7a20)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x44).lw(c);
    if c.g[V0][0] == 0 {
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x12).lh(c);
    } else {
        c.cycles += 3;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002e_4578, 0x0027_7a38)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x12).lh(c);
    }
    // The three durations: nothing to start when they sum to 0 or less.
    c.cycles += 6;
    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x14).lh(c);
    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x16).lh(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[V0][0] = 7;
    if none {
        return Ok(true);
    }
    c.cycles += 3;
    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x10).lb(c);
    let captured = c.g[A1][0] == c.g[V0][0];
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x84c)).lw(c);
    if captured {
        // A captured transition (kind 7).
        c.cycles += 14;
        c.g[A1][0] = 7;
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x11).lb(c);
        TRANSITION_TARGET.call(c, 0x0027_7afc)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x12).lh(c);
        c.g[A0][0] = c.g[S2][0];
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x47f8));
        c.g[A2][0] = c.g[V0][0];
        c.f[12] = FpuF32::from_i32(c.g[V1][0] as u32 as i32).0;
        c.g[A1][0] = 2;
        c.g[A3][0] = 0;
        c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[0])).0;
        call_ptr(c, 0x002e_4540, 0x0027_7b24)?;
        Addr(c.g[S1][0] as u32).off(0xb8).sw(c, 0);
    } else {
        // A transition to the first target; none means it cannot start.
        c.cycles += 6;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x11).lb(c);
        TRANSITION_TARGET.call(c, 0x0027_7a6c)?;
        c.g[S3][0] = c.g[V0][0];
        if c.g[S3][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            return Ok(false);
        }
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x10).lb(c);
        c.cycles += 28;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x11).lb(c);
        TRANSITION_TARGET.call(c, 0x0027_7a8c)?;
        c.g[A3][0] = Addr(c.g[S0][0] as u32).off(0x12).lh(c);
        c.g[A0][0] = c.g[S2][0];
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x14).lh(c);
        c.g[A2][0] = c.g[S3][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x16).lh(c);
        c.f[12] = FpuF32::from_i32(c.g[A3][0] as u32 as i32).0;
        c.f[13] = FpuF32::from_i32(c.g[V1][0] as u32 as i32).0;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x47fc));
        c.f[14] = FpuF32::from_i32(c.g[A1][0] as u32 as i32).0;
        c.g[A3][0] = c.g[V0][0];
        c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[0])).0;
        c.g[A1][0] = 2;
        c.f[13] = (FpuF32(c.f[13]) * FpuF32(c.f[0])).0;
        c.g[T0][0] = 0;
        c.f[14] = (FpuF32(c.f[14]) * FpuF32(c.f[0])).0;
        c.g[T1][0] = 0;
        call_ptr(c, 0x002e_4370, 0x0027_7adc)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x14).lh(c);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x12).lh(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let frames = c.g[V0][0];
        Addr(c.g[S1][0] as u32).off(0xb8).sw(c, frames);
    }
    // The started transition's frames: the list then waits for it.
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0xb8).lw(c);
    let no_frames = (c.g[V0][0] as i64) <= 0;
    c.g[V0][0] = 1;
    if no_frames {
        return Ok(true);
    }
    c.cycles += 3;
    Addr(c.g[S1][0] as u32).off(0xac).sw(c, 0);
    let one = c.g[V0][0];
    Addr(c.g[S1][0] as u32).off(0xb0).sw(c, one);
    Ok(true)
}

/// The epilogue of [`script_list_start_transition`].
pub(crate) fn start_transition_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 7;
    frame.restore(c, S0, 0x40);
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `ScriptList_TakeFrom` (0x00276198): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn script_list_take_from(c: &mut Cpu) {
    if c.pc != 0x00276198 {
        return script_list_take_from_resume(c);
    }
    guest(c, |c| {
        c.cycles += 15;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S0][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[RA][0] = sx(0x002761c0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        Callee {
            addr: 0x00283d28,
            f: ee::s07::ScriptCmdQueue_Count,
        }
        .call(c, 0x002761c0)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
        c.g[V0][0] = 0x3;
        let t = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        'out: {
            if !t {
                // 0x2761d4: the source list is in state 3: nothing to take.
                c.cycles += 2;
                c.g[V0][0] = 0u64;
                break 'out;
            }
            // 0x2761dc
            c.cycles += 4;
            c.g[RA][0] = sx(0x002761e4);
            c.g[S2][0] = c.g[A0][0];
            Callee {
                addr: 0x00283d28,
                f: ee::s07::ScriptCmdQueue_Count,
            }
            .call(c, 0x002761e4)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[S2][0];
            if t {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'out;
            }
            // 0x2761ec (always continues at 0x276204)
            c.cycles += 2;
            loop {
                // 0x276204
                c.cycles += 4;
                c.g[RA][0] = sx(0x0027620c);
                c.g[A1][0] = c.g[SP][0];
                Callee {
                    addr: 0x00283dc0,
                    f: ee::s07::ScriptCmdQueue_Pop,
                }
                .call(c, 0x0027620c)?;
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                if !t {
                    break;
                }
                if back_edge_unwinds(c, 0x002761f8) {
                    return Ok(());
                }
                // 0x2761f8
                c.cycles += 3;
                c.g[RA][0] = sx(0x00276200);
                c.g[A1][0] = c.g[SP][0];
                Callee {
                    addr: 0x00283c80,
                    f: ee::s07::ScriptCmdQueue_Push,
                }
                .call(c, 0x00276200)?;
                c.g[A0][0] = c.g[S2][0];
            }
            // 0x276214
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00276220);
            c.g[A1][0] = 0x1;
            Callee {
                addr: 0x00276868,
                f: ee::s07::ScriptList_Stop,
            }
            .call(c, 0x00276220)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'out;
            }
            // 0x27622c
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
            let t = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = 0x1;
            if t {
                break 'out;
            }
            // 0x276238
            c.cycles += 5;
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00276248);
            c.g[A2][0] = 0u64;
            Callee {
                addr: 0x00277450,
                f: ee::s07::ScriptList_PrepareEntry,
            }
            .call(c, 0x00276248)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
            // 0x27624c
            c.cycles += 1;
            c.g[V0][0] = 0x1;
        }
        // 0x276250
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}

/// Native resume of `script_list_take_from` (converted from the `generated` body): any entry
/// other than 0x00276198, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn script_list_take_from_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00276198 => {
                    c.cycles += 15;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.g[RA][0] = sx(0x002761c0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    Callee {
                        addr: 0x00283d28,
                        f: ee::s07::ScriptCmdQueue_Count,
                    }
                    .call(c, 0x002761c0)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                    c.g[V0][0] = 0x3;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                    if t {
                        pc = 0x002761dc;
                        continue;
                    }
                    pc = 0x002761d4;
                    continue;
                }
                0x002761d4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00276250;
                        continue;
                    }
                    pc = 0x002761dc;
                    continue;
                }
                0x002761dc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002761e4);
                    c.g[S2][0] = c.g[A0][0];
                    Callee {
                        addr: 0x00283d28,
                        f: ee::s07::ScriptCmdQueue_Count,
                    }
                    .call(c, 0x002761e4)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        pc = 0x0027624c;
                        continue;
                    }
                    pc = 0x002761ec;
                    continue;
                }
                0x002761ec => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x00276204;
                        continue;
                    }
                    pc = 0x002761f4;
                    continue;
                }
                0x002761f4 => {
                    c.cycles += 1;
                    pc = 0x002761f8;
                    continue;
                }
                0x002761f8 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00276200);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x00283c80,
                        f: ee::s07::ScriptCmdQueue_Push,
                    }
                    .call(c, 0x00276200)?;
                    c.g[A0][0] = c.g[S2][0];
                    pc = 0x00276204;
                    continue;
                }
                0x00276204 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0027620c);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x00283dc0,
                        f: ee::s07::ScriptCmdQueue_Pop,
                    }
                    .call(c, 0x0027620c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    if t {
                        if back_edge_unwinds(c, 0x002761f8) {
                            return Ok(());
                        }
                        pc = 0x002761f8;
                        continue;
                    }
                    pc = 0x00276214;
                    continue;
                }
                0x00276214 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00276220);
                    c.g[A1][0] = 0x1;
                    Callee {
                        addr: 0x00276868,
                        f: ee::s07::ScriptList_Stop,
                    }
                    .call(c, 0x00276220)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00276250;
                        continue;
                    }
                    pc = 0x0027622c;
                    continue;
                }
                0x0027622c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00276250;
                        continue;
                    }
                    pc = 0x00276238;
                    continue;
                }
                0x00276238 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00276248);
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x00277450,
                        f: ee::s07::ScriptList_PrepareEntry,
                    }
                    .call(c, 0x00276248)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                    pc = 0x0027624c;
                    continue;
                }
                0x0027624c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00276250;
                    continue;
                }
                0x00276250 => {
                    c.cycles += 7;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00276198, 0x0027626c);
                    return Ok(());
                }
            }
        }
    });
}

/// `ScriptCmdQueue_Count` (0x283d28).
pub(crate) const SLK_CMDQ_COUNT: Callee = Callee {
    addr: 0x0028_3d28,
    f: ee::s07::f_00283d28,
};

/// `ScriptList_PopEntries` (0x277598).
pub(crate) const SLK_POP_ENTRIES: Callee = Callee {
    addr: 0x0027_7598,
    f: ee::s07::f_00277598,
};

/// `ScriptList_Prepare` (0x276270).
pub(crate) const SLK_LIST_PREPARE: Callee = Callee {
    addr: 0x0027_6270,
    f: ee::s07::f_00276270,
};

/// `ScriptCmdQueue_At` (0x283d70).
pub(crate) const SLK_CMDQ_AT: Callee = Callee {
    addr: 0x0028_3d70,
    f: ee::s07::f_00283d70,
};

/// `ScriptList_IsMovieEntry` (0x277dd8).
pub(crate) const SLK_IS_MOVIE: Callee = Callee {
    addr: 0x0027_7dd8,
    f: ee::s07::f_00277dd8,
};

/// `ScriptEngine_KillProcess` (0x282540).
pub(crate) const SLK_KILL_PROCESS: Callee = Callee {
    addr: 0x0028_2540,
    f: ee::s07::f_00282540,
};

/// `ScriptMovie_Stop` (0x283818).
pub(crate) const SLK_MOVIE_STOP: Callee = Callee {
    addr: 0x0028_3818,
    f: ee::s07::f_00283818,
};

/// `ScriptList_SetPaused` (0x277c08).
pub(crate) const SLK_SET_PAUSED: Callee = Callee {
    addr: 0x0027_7c08,
    f: ee::s07::f_00277c08,
};

/// `ScriptList_PrepareEntry` (0x277450).
pub(crate) const SLK_PREPARE_ENTRY: Callee = Callee {
    addr: 0x0027_7450,
    f: ee::s07::f_00277450,
};

/// `ScriptList_Stop` (0x276868).
pub(crate) const SLK_LIST_STOP: Callee = Callee {
    addr: 0x0027_6868,
    f: ee::s07::f_00276868,
};

/// `ScriptList_StartTransition` (0x277980).
pub(crate) const SLK_START_TRANSITION: Callee = Callee {
    addr: 0x0027_7980,
    f: ee::s07::f_00277980,
};

/// `ScriptList_TryAdvance` (0x277800).
pub(crate) const SLK_TRY_ADVANCE: Callee = Callee {
    addr: 0x0027_7800,
    f: ee::s07::f_00277800,
};

/// `void ScriptList_Skip(list, delta, force)` (0x2766d0): moves the list's
/// skip position by `delta` and advances the list to the new position: the
/// queue entries before it are removed, the next pending script is set up,
/// and the transition or the play-on-available path runs. Returns the state.
/// Frame 0x50: saves ra and s0-s3.
pub fn script_list_skip_2(c: &mut Cpu) {
    guest(c, slk_body)
}

pub(crate) fn slk_body(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x50);
    frame.save(c, 16, 0x40);
    frame.save(c, 17, 0x30);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, 19, 0x10);
    frame.save(c, 18, 0x20);
    frame.save_ra(c, 0);
    'b_27684c: {
        'b_276848: {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
            c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let t = c.g[S1][0] == c.g[V0][0];
            c.g[S3][0] = c.g[A2][0];
            if t {
                break 'b_276848;
            }
            'b_276840: {
                'b_27683c: {
                    'b_276764: {
                        c.cycles += 11;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[S1][0] = c.g[V1][0];
                        }
                        SLK_CMDQ_COUNT.call(c, 0x0027_6714)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                        c.g[V0][0] = 0x3;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        if t {
                            break 'b_276764;
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), 0u64);
                        if t {
                            break 'b_27683c;
                        }
                        c.cycles += 8;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S0][0];
                        SLK_POP_ENTRIES.call(c, 0x0027_673c)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                        let t = c.g[V1][0] == 0u64;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xa4), 0u64);
                        if t {
                            break 'b_276848;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb4));
                        c.g[A0][0] = c.g[S0][0];
                        SLK_LIST_PREPARE.call(c, 0x0027_675c)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                        break 'b_27684c;
                    }
                    'b_276800: {
                        'b_2767fc: {
                            c.cycles += 3;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            let t = (c.g[A1][0] as i64) <= 0;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[S1][0]);
                            if t {
                                break 'b_2767fc;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa8));
                                break 'b_276800;
                            }
                            'b_2767c8: {
                                'b_2767b0: {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[S2][0];
                                    SLK_CMDQ_AT.call(c, 0x0027_6784)?;
                                    c.g[S2][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    SLK_IS_MOVIE.call(c, 0x0027_6794)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                        break 'b_2767b0;
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                    SLK_KILL_PROCESS.call(c, 0x0027_67a8)?;
                                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                                    break 'b_2767c8;
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                                if t {
                                    break 'b_2767c8;
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
                                SLK_MOVIE_STOP.call(c, 0x0027_67c4)?;
                                c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                            }
                            'b_2767e8: {
                                c.cycles += 5;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V1][0]);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa8));
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_2767e8;
                                }
                                c.cycles += 3;
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0x1;
                                SLK_SET_PAUSED.call(c, 0x0027_67e8)?;
                            }
                            c.cycles += 5;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = 0u64;
                            SLK_PREPARE_ENTRY.call(c, 0x0027_67f8)?;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa8));
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                        break 'b_27684c;
                    }
                    'b_276824: {
                        c.cycles += 2;
                        if c.g[S3][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            break 'b_276824;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        SLK_START_TRANSITION.call(c, 0x0027_6818)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                            break 'b_27684c;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    }
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_276840;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    SLK_TRY_ADVANCE.call(c, 0x0027_6834)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                    break 'b_27684c;
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
                break 'b_276840;
            }
            c.cycles += 2;
            c.g[A1][0] = 0x1;
            SLK_LIST_STOP.call(c, 0x0027_6848)?;
            break 'b_276848;
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
        break 'b_27684c;
    }
    c.cycles += 7;
    frame.restore(c, 16, 0x40);
    frame.restore(c, 17, 0x30);
    frame.restore(c, 18, 0x20);
    frame.restore(c, 19, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `DataInstanceMan_OnDataAdded` (0x0028_47a8): straight-line leaf. Mechanical port of the generated body.
pub fn data_instance_man_on_data_added_002847a8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DataInstanceMan_OnDataRemoved` (0x0028_47b0): straight-line leaf. Mechanical port of the generated body.
pub fn data_instance_man_on_data_removed_002847b0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DataInstanceMan_v04` (0x0028_47b8): straight-line leaf. Mechanical port of the generated body.
pub fn data_instance_man_v04_002847b8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DataInstanceMan_v05` (0x0028_47c0): straight-line leaf. Mechanical port of the generated body.
pub fn data_instance_man_v05_002847c0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::game::main::gamestate::tests::{fake_script_list_stop, PREV_SCRIPT_LIST_STOP};
    use crate::game::scripter::ssxscriptengine::tests::oc428_install_fakes;

    use super::*;
    use crate::difftest::check;
    use crate::game::ai::control::handplantcontrol::tests::oc425_install;

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;

    use crate::difftest::fill;

    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;

    pub(crate) use crate::game::ai::ai::tests::strict as strict_2;
    pub(crate) use crate::game::ai::ai::tests::SEEDS as SEEDS_8;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_9;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;
    pub(crate) use crate::game::fe::debugfe::tests::w as w_3;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_7;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_10;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_4;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_5;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS;

    use ssx3_ee as ee;

    #[test]
    fn data_instance_man_free_info_matches() {
        check(
            ssx3_ee::s07::f_00275cd0,
            data_instance_man_free_info,
            200,
            &|c, seed| {
                let mut r = Rng(0x275c_d0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.bus.write32(DATA + 0x8, r.next_u32());
                for i in 0..6 {
                    c.bus.write32(DATA + 0x100 + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn data_instance_man_alloc_info_matches() {
        check(
            ssx3_ee::s07::f_00275cf8,
            data_instance_man_alloc_info,
            200,
            &|c, seed| {
                let mut r = Rng(0x275c_f8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let head = DATA + 0x100;
                c.bus
                    .write32(DATA + 0x8, if seed % 5 == 0 { 0 } else { head });
                c.bus
                    .write32(head + 0x14, if seed % 4 == 0 { 0 } else { DATA + 0x140 });
            },
        );
    }

    #[test]
    fn script_list_is_paused_matches() {
        check(
            ssx3_ee::s07::f_00277298,
            script_list_is_paused,
            200,
            &|c, seed| {
                let mut r = Rng(0x2772_98 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 0xa4, if seed % 3 == 0 { 3 } else { r.next_u32() });
                c.bus
                    .write32(DATA + 0xa8, if seed % 4 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn script_list_is_movie_entry_matches() {
        check(
            ssx3_ee::s07::f_00277dd8,
            script_list_is_movie_entry,
            200,
            &|c, seed| {
                let mut r = Rng(0x277d_d8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = DATA as u64;
                let value = match seed % 4 {
                    0 => 28,
                    1 => 29,
                    2 => 0x8000_0000,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA, value);
            },
        );
    }

    #[test]
    fn script_bank_slot_clear_matches() {
        check(
            ssx3_ee::s07::f_00277f08,
            script_bank_slot_clear,
            100,
            &|c, seed| {
                let mut r = Rng(0x277f_08 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..4 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    // ------------------------------------------------------ script engine

    #[test]
    fn script_list_get_state_matches() {
        check(
            ssx3_ee::s07::f_00276b98__generated,
            script_list_get_state,
            60,
            &|c, seed| {
                machine(c, seed, 0x276b_98, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0xa4, if seed % 2 == 0 { 1 } else { 2 });
            },
        );
    }

    #[test]
    fn script_list_is_current_skippable_matches() {
        check(
            ssx3_ee::s07::f_002770c0__generated,
            script_list_is_current_skippable,
            60,
            &|c, seed| {
                machine(c, seed, 0x2770_c0, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0xa4, if seed % 2 == 0 { 3 } else { 0 });
            },
        );
    }

    #[test]
    fn script_list_dtor_matches() {
        check(
            ee::s07::f_00275d90__generated,
            script_list_dtor,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x275d_90, 2);
                let flags = seed % 2;
                // The free branch runs on a null list (a bogus pointer would be
                // handed to the heap).
                let this = if flags != 0 { 0 } else { DATA + 0x100 };
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(flags);
                c.bus.write32(this + 0x10, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------------------
    // Script engine.

    #[test]
    fn script_list_on_seq_stopped_matches() {
        check(
            ee::s07::f_00277400__generated,
            script_list_on_seq_stopped,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2774_00, 2);
                let list = DATA + 0x100;
                clear(c, list, 0x100);
                // Playing only when the list sits in play mode 3.
                c.bus
                    .write32(list + 0xa4, if seed % 2 == 0 { 0 } else { 3 });
                clear(c, DATA + 0x400, 0x40);
                c.bus.write32(gp_at(-0x84c), 0);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = 0;
            },
        );
    }

    // ----------------------------------------------------------- script list

    #[test]
    fn script_list_on_seq_start_matches() {
        check(
            ee::s07::f_002772c0__generated,
            script_list_on_seq_start,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x2772);
                let list = DATA;
                let obj = DATA + 0x1000;
                c.bus
                    .write32(list + 0xa4, if seed % 3 == 0 { 0 } else { 3 });
                c.bus.write32(obj + 0x14, DATA + 0x2000);
                c.bus
                    .write8(DATA + 0x2000, if seed % 4 == 0 { 0 } else { b'x' });
                c.bus.write32(DATA + 0x2004, r.next_u32());
                c.bus.write8(0x004a_34a8, 0);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = sx(obj);
            },
        );
    }

    // --------------------------------------------------------- script engine

    #[test]
    fn script_engine_unload_master_matches() {
        check(
            ee::s07::f_00278308__generated,
            script_engine_unload_master,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x2783);
                let e = DATA;
                // The database destroy is a no-op without a live database.
                c.bus.write32(e + 0x2b0, 0);
                fill(c, &mut r, e + 0x400, 0x100);
                c.bus
                    .write32(e + 0x52c, if seed % 3 == 0 { 0 } else { 0x40 });
                c.bus
                    .write32(e + 0x534, if seed % 2 == 0 { 0 } else { 0x80 });
                c.g[A0][0] = sx(e);
            },
        );
    }

    #[test]
    fn data_instance_man_dtor_matches() {
        check(
            ee::s07::f_002758c0__generated,
            data_instance_man_dtor,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x0027_58c0, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 3) as u64;
                w(c, DATA + 4, if seed % 2 == 0 { 0 } else { DATA + 0x800 });
            },
        );
    }

    // ------------------------------------------------------------------
    // Script.

    #[test]
    fn script_list_on_seq_end_matches() {
        check(
            ee::s07::f_002773a0__generated,
            script_list_on_seq_end,
            SEEDS_3,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x2773_a0);
                let list = DATA + 0x100;
                clear(c, list, 0x100);
                w_2(c, list + 0xa4, if seed % 2 == 0 { 0 } else { 3 });
                w_2(c, list + 0xb8, if seed % 4 < 2 { 0 } else { 1 });
                w_2(c, gp_at(-0x84c), 0);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn script_list_can_skip_matches() {
        check(
            ssx3_ee::s07::f_00277060__generated,
            script_list_can_skip,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x277060, 0);
                w_3(c, OBJ + 0xa4, if seed % 3 == 0 { 3 } else { seed % 4 });
                w_3(c, OBJ + 0x8c, seed);
                w_3(c, OBJ + 0xa8, if seed % 2 == 0 { 0 } else { seed % 4 });
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn script_list_get_entry_type_matches() {
        check(
            ee::s07::f_00276998__generated,
            script_list_get_entry_type,
            80,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x2769_98);
                let list = DATA + 0xc;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 7) as u64;
                c.bus
                    .write32(list + 0x94, if seed % 4 == 0 { 1 } else { 0 });
                c.bus.write32(list + 0x8c, (seed % 5) as u32);
                c.bus.write32(list + 0x90, 5);
                for slot in 0..5u32 {
                    let v = if seed % 3 == 0 {
                        r.next_u32() % 64
                    } else {
                        (r.next_u32() % 30) + 2
                    };
                    c.bus.write32(list + slot * 0x1c, v);
                }
            },
        );
    }

    #[test]
    fn data_instance_man_remove_all_matches() {
        check(
            ssx3_ee::s07::f_00275b08__generated,
            data_instance_man_remove_all,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x275b08, 0);
                let m = DATA + 0x1000;
                let entries = DATA + 0x2000;
                let count = (seed % 5) as u32;
                c.g[A0][0] = m as u64;
                w32(c, m, count);
                w32(c, m + 4, entries);
                for i in 0..4u32 {
                    w32(c, entries + i * 0x18 + 0x10, 0);
                    w32(c, entries + i * 0x18, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn script_list_ctor_matches() {
        check(
            ee::s07::f_00275d10__generated,
            script_list_ctor,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x275d_10, 4);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    // -----------------------------------------------------------------------
    // Script lists.

    #[test]
    fn scriptlist_is_playing_script_matches() {
        check(
            ee::s07::f_002771c8__generated,
            scriptlist_is_playing_script,
            SEEDS_5,
            &|c, seed| {
                base_5(c, seed, 0x71c8, 0);
                let engine = DATA + 0x8800;
                let s0 = DATA + 0x100;
                c.g[A0][0] = sx(s0);
                c.g[A1][0] = (seed % 3) as u64;
                c.bus.write32(GAME_GP - 0x84c, engine);
                c.bus.write32(s0 + 0xa4, if seed % 4 == 0 { 0 } else { 3 });
                c.bus.write32(s0 + 0xc + 0x8c, 0);
                c.bus.write32(s0 + 0xc, if seed % 2 == 0 { 5 } else { 40 });
                c.bus.write32(s0 + 0xc + 0xc, (seed / 4 % 3) as u32);
                for id in 0..3u32 {
                    let p = DATA + 0x8900 + 0x40 * id;
                    c.bus.write32(engine + 0x2c0 + 0x24 * id + 0x20, p);
                    c.bus.write32(p, id * 7 + 1);
                }
            },
        );
    }

    // ------------------------------------------------------------------
    // Script list.

    #[test]
    fn script_list_poll_loading_matches() {
        check(
            ssx3_ee::s07::f_00277778__generated,
            script_list_poll_loading,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2777_78, 4);
                let list = DATA + 0x100;
                let engine = DATA + 0x400;
                w(c, list, engine);
                let state = if seed % 4 == 0 { 0 } else { 1 };
                w(c, list + 0xa4, state);
                // ScriptCmdQueue_At(list+0xc, 0): read cursor at +0x8c, 28-byte slots.
                let read = r.next_u32() % 5;
                w(c, list + 0xc + 0x8c, read);
                let entry = list + 0xc + read * 0x1c;
                // ScriptList_IsMovieEntry reads the first word of the entry.
                let movie = seed % 5 == 0;
                w(
                    c,
                    entry,
                    if movie {
                        29 + r.next_u32() % 4
                    } else {
                        r.next_u32() % 29
                    },
                );
                // ScriptEngine_GetProcessState(engine, entry[+0xc]) indexes a
                // 0x24-byte process record at engine+0x2c0; present at +0x20.
                let id = (r.next_u32() % 3) as u32;
                w(c, entry + 0xc, id);
                let proc = engine + 0x2c0 + id * 0x24;
                w(c, proc + 0x20, 1);
                w(c, proc + 4, if seed % 2 == 0 { 2 } else { 3 });
            },
        );
    }

    #[test]
    fn script_list_on_seq_advance_matches() {
        check(
            ee::s07::f_00277310__generated,
            script_list_on_seq_advance,
            120,
            &|c, seed| {
                let _ = base_6(c, seed, 0x27_7310);
                globals(c);
                // self+0x4 gates the whole thing; obj+0x1c is the pending
                // transition.
                w(c, DATA + 0x4, if seed % 2 == 0 { 0 } else { 1 });
                w(c, DATA + 0xb0, 0);
                w(c, DATA + 0xac, 0);
                w(c, DATA + 0x2000 + 0x1c, [0, 0, 1][seed as usize % 3]);
                w(c, DATA + 0x2000 + 0x12, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x2000);
            },
        );
    }

    #[test]
    fn data_instance_man_ctor_matches() {
        check(
            ee::s07::f_00275828__generated,
            data_instance_man_ctor,
            SEEDS_7,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (seed % 4) as u64;
            }),
        );
    }

    #[test]
    fn script_list_check_next_ready_matches() {
        check(
            ee::s07::f_002776e0__generated,
            script_list_check_next_ready,
            SEEDS_7,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(
                    c,
                    this + 0xbc,
                    if seed & 1 == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 5) as u32
                    },
                );
                w(c, this + 0x8c, 0);
                w(c, this + 0x90, 0);
                w(c, this, DATA + 0x1100);
                let cmd = DATA + 0x1200;
                for i in 0..8 {
                    w(c, cmd + 4 * i, if i == 0 { 0x10 } else { 0 });
                }
                w(c, this + 0xc, cmd);
                c.g[A0][0] = this as u64;
            }),
        );
    }

    #[test]
    fn script_movie_stop_matches() {
        strict(
            ee::s07::f_00283818__generated,
            script_movie_stop,
            40,
            &|c, seed| {
                let _r = base_8(c, seed, 0x2838);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
                c.bus.write32(DATA + 4, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x10, seed as u32);
                c.bus.write32(DATA + 0x14, (seed % 4 == 0) as u32);
                c.bus.write32(gp_at(-0x69c), (seed % 4) as u32);
                c.bus.write32(gp_at(-0x6a0), 0);
            },
        );
    }

    #[test]
    fn scriptlist_add_matches() {
        strict_2(
            ee::s07::f_00275dd8__generated,
            scriptlist_add,
            SEEDS_8,
            &|c, seed| {
                let mut r = base_9(c, seed, 0x5dd8, 0);
                let this = DATA + 0x100;
                // no pending list state: the call queues and returns 1
                w(c, this + 0xa4, 0);
                w(c, this + 0xbc, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    pub(crate) const DMAN: u32 = DATA + 0x1_0000;

    pub(crate) const BIG: u32 = DATA + 0x1_1000;

    #[test]
    fn data_instance_man_add_big_entries_matches() {
        check_all(
            ee::s07::f_00275b98__generated,
            data_instance_man_add_big_entries,
            20,
            &|c, seed| {
                let mut r = base_10(c, seed, 0x275b_98, 10);
                c.g[A0][0] = sx(DMAN);
                c.g[A1][0] = sx(BIG);
                c.g[A2][0] = 0;
                // The archive walker: an empty archive stops immediately; one
                // seed gets a single non-archive entry.
                if seed % 2 == 0 {
                    put_words(c, BIG, &[0, 0, 0, 0]);
                } else {
                    put_words(c, BIG, &[1, 0, 0x20, 0]);
                }
                let vt5 = vtable(c, &mut r, &[0x10, 0x14]);
                c.bus.write32(DMAN + 0xc, vt5);
            },
        );
    }

    pub(crate) const ENGINE: u32 = DATA + 0x1_0000;

    #[test]
    fn script_engine_dtor_matches() {
        check_all(
            ee::s07::f_002780b8__generated,
            script_engine_dtor,
            20,
            &|c, seed| {
                let _ = base_10(c, seed, 0x2780_b8, 16);
                c.g[A0][0] = sx(ENGINE);
                c.g[A1][0] = sx(seed % 4); // destructor flags
                c.bus.write32(ENGINE + 0x52c, (seed % 2) as u32);
                // The per-frame state slots at_4 +0x6d0..+0x910: no vtables, so the
                // slot destructor loop only decrements.
                c.bus.write32(ENGINE + 0x910, 0);
            },
        );
    }

    // ------------------------------------------------------------------
    // Script list update.

    const SL_LIST: u32 = DATA + 0x100;
    const SL_ENGINE: u32 = DATA + 0x1000;
    const SL_INSEQ: u32 = DATA + 0x3000;
    const SL_MOVIE: u32 = DATA + 0x3800;
    const SL_WORLD: u32 = DATA + 0x5000;
    const SL_APP: u32 = DATA + 0x5100;
    const SL_PLAYER: u32 = DATA + 0x5200;
    const SL_TABLE: u32 = DATA + 0x5400;

    /// The shape of a [`script_list_fixture`].
    #[derive(Clone, Copy)]
    struct SlShape {
        state: u32,
        start_pending: u32,
        nested: u32,
        transition: u32,
        resume_wait: u32,
        resume_pending: u32,
        paused: u32,
        next: u32,
        skip_wait: u32,
        /// The engine's movie playing (`*p` = an id, active): every callee
        /// that would stop it (`ScriptList_NextEntry`) is kept off by a
        /// pausing next entry (+0xbc = 1, a movie without an id).
        movie_playing: bool,
        /// The player's effect (+0x44) running.
        player_fx: u32,
        /// The current entry is a movie (else a script).
        head_movie: bool,
        head_movie_id: u32,
        /// The script process (id -1, the record at engine+0x29c): present,
        /// its state (never 1, which launches) and its sequence's state.
        proc_present: u32,
        proc_state: u32,
        inseq_state: u32,
    }

    /// A script list at [`SL_LIST`] with a three-entry queue, its engine
    /// (one script process record, id -1, and the movie player at +0x500),
    /// the world's app with the player and an empty UI state table. Scripts
    /// in the queue have id -1, so popping them stops nothing, and an idle
    /// movie player (`p` = 0) makes starting and stopping movies no-ops.
    fn script_list_fixture(c: &mut Cpu, r: &mut Rng, s: SlShape) {
        clear(c, SL_LIST, 0x40);
        clear(c, SL_ENGINE, 0x180);
        clear(c, SL_INSEQ, 0x20);
        clear(c, SL_MOVIE, 0x10);
        clear(c, SL_WORLD, 0x200);
        clear(c, SL_TABLE, 0x20);
        w(c, GAME_GP.wrapping_sub(0x848), SL_WORLD);
        w(c, GAME_GP.wrapping_sub(0x84c), SL_ENGINE);
        w(c, SL_WORLD + 0x84, SL_APP);
        w(c, SL_APP + 0x5c, SL_PLAYER);
        w(c, SL_APP + 0x48, SL_TABLE);
        w(c, SL_PLAYER + 0x44, s.player_fx);
        w(c, SL_PLAYER + 0x6c, u32::MAX);
        // The process record of id -1.
        let rec = SL_ENGINE + 0x29c;
        w(c, rec, SL_INSEQ);
        w(c, rec + 4, s.proc_state);
        w(c, rec + 0x20, s.proc_present);
        w(c, SL_INSEQ + 0x20, s.inseq_state);
        w(c, SL_INSEQ + 0x2c, r.next_u32() % 3);
        // The movie player.
        if s.movie_playing {
            w(c, SL_ENGINE + 0x500, SL_MOVIE);
            w(c, SL_ENGINE + 0x504, 1);
            w(c, SL_MOVIE, 7);
        }
        // The list.
        let l = SL_LIST;
        w(c, l, SL_ENGINE);
        w(c, l + 8, r.next_u32() % 4);
        let fields = [
            (0xa4, s.state),
            (0xa8, s.paused),
            (0xac, s.resume_pending),
            (0xb0, s.transition),
            (0xb4, s.start_pending),
            (0xb8, s.resume_wait),
            (0xbc, s.next),
            (0xc4, s.nested),
            (0xc8, s.skip_wait),
        ];
        for (off, v) in fields {
            w(c, l + off, v);
        }
        // The queue: three entries from a random read slot; the second is a
        // movie without an id (it pauses the list when it is next), the third
        // a movie with one.
        let q = l + 0xc;
        let read = r.next_u32() % 5;
        w(c, q + 0x8c, read);
        w(c, q + 0x90, (read + 3) % 5);
        w(c, q + 0x94, 0);
        let entries = [
            if s.head_movie {
                (29 + r.next_u32() % 4, s.head_movie_id)
            } else {
                (r.next_u32() % 29, u32::MAX)
            },
            (30, u32::MAX),
            (31, 2),
        ];
        for (i, (kind, id)) in entries.into_iter().enumerate() {
            let e = q + ((read + i as u32) % 5) * 0x1c;
            w(c, e, kind);
            w(c, e + 8, r.next_u32() % 16);
            w(c, e + 0xc, id);
        }
        c.g[A0][0] = sx(l);
    }

    /// The shape for `seed`: every branch of `ScriptList_Update` with callees
    /// that stay on short paths.
    fn script_list_shape(seed: u32, r: &mut Rng) -> SlShape {
        let k = seed / 6;
        let mut s = SlShape {
            state: 3,
            start_pending: 0,
            nested: 0,
            transition: 0,
            resume_wait: 0,
            resume_pending: 0,
            paused: 0,
            next: u32::MAX,
            skip_wait: 0,
            movie_playing: false,
            player_fx: r.next_u32() % 2,
            head_movie: k % 2 == 0,
            head_movie_id: [0, 1, u32::MAX][(k / 2 % 3) as usize],
            proc_present: (k % 7 != 0) as u32,
            proc_state: [0, 2, 3, 4, 5][(k % 5) as usize],
            inseq_state: [0, 3, 4][(k % 3) as usize],
        };
        match seed % 6 {
            // Loading.
            0 => s.state = 1,
            // Ready, maybe started (the current entry is a movie, so the
            // start only switches the state).
            1 => {
                s.state = 2;
                s.start_pending = k % 3;
                s.head_movie = true;
            }
            // Not playing, or nested-paused.
            2 => match k % 3 {
                0 => s.state = 0,
                1 => s.state = 4,
                _ => s.nested = 1,
            },
            // Playing.
            _ => {
                let m = seed / 3;
                s.movie_playing = m % 3 == 0;
                s.transition = if s.movie_playing { 1 } else { (m / 3) % 2 };
                s.resume_wait = [0, 1, 1, 2, 5][(m % 5) as usize];
                s.resume_pending = (m / 5) % 2;
                s.skip_wait = [0, 1, 3][(m / 2 % 3) as usize];
                if s.movie_playing {
                    s.player_fx = 1;
                    s.next = 1;
                } else {
                    s.paused = (m / 7) % 2;
                    s.next = if s.paused != 0 {
                        0
                    } else {
                        [u32::MAX, 0][(m % 2) as usize]
                    };
                }
            }
        }
        s
    }

    #[test]
    fn script_list_update_matches() {
        check(
            ee::s07::f_00276cc8__generated,
            script_list_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x276cc8, 0);
                let s = script_list_shape(seed, &mut r);
                script_list_fixture(c, &mut r, s);
            },
        );
    }

    /// Updates of a playing list until the countdown at `field` (+0xb8 the
    /// resume wait, +0xc8 the skip wait) of `frames` 60 Hz frames runs out,
    /// at game rate `hz`.
    fn script_list_wait_updates(hz: u32, field: u32, frames: u32) -> u32 {
        let mut c = crate::difftest::fresh();
        let c = &mut *c;
        let mut r = machine(c, 3, 0x276cc8, 0);
        let s = SlShape {
            state: 3,
            start_pending: 0,
            nested: 0,
            transition: (field == 0xb8) as u32,
            resume_wait: if field == 0xb8 { frames } else { 0 },
            resume_pending: 1,
            paused: 0,
            next: u32::MAX,
            skip_wait: if field == 0xc8 { frames } else { 0 },
            movie_playing: false,
            player_fx: 1,
            head_movie: false,
            head_movie_id: 0,
            proc_present: 1,
            proc_state: 3,
            inseq_state: 0,
        };
        script_list_fixture(c, &mut r, s);
        crate::tick::set_for_test(c, hz);
        let mut updates = 0;
        while c.bus.read32(SL_LIST + field) != 0 {
            updates += 1;
            assert!(updates < 1000);
            c.g[A0][0] = sx(SL_LIST);
            script_list_update(c);
            assert_eq!(c.pc, TEST_RA, "{hz} Hz: update {updates} did not return");
        }
        updates
    }

    /// The resume and skip waits count 60 Hz frames: a 30-frame wait runs
    /// out after 30 updates at 60 Hz and after the same time at 120 / 240 Hz
    /// (its first frame on the first update, then one per `rate/60`).
    #[test]
    fn script_list_update_waits_follow_the_rate() {
        for field in [0xb8u32, 0xc8] {
            assert_eq!(script_list_wait_updates(60, field, 30), 30, "{field:#x}");
            for hz in [120u32, 240] {
                let q = hz / 60;
                assert_eq!(
                    script_list_wait_updates(hz, field, 30),
                    30 * q - (q - 1),
                    "{field:#x} at {hz} Hz"
                );
            }
        }
    }

    pub(crate) use crate::game::ai::rider::tests::w32_2;
    pub(crate) use crate::game::world::worldview::tests::oc410_machine;
    pub(crate) use crate::game::world::worldview::tests::oc410_result;

    /// `ScriptList_GetPlayStatus`: every state of the play status chain.
    #[test]
    fn script_list_get_play_status_matches() {
        const LIST: u32 = DATA + 0x1000;
        const CMD: u32 = DATA + 0x2000;
        check(
            ee::s07::f_00276bd8__generated,
            script_list_get_play_status,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0027_6bd8, 2);
                c.pc = 0x0027_6bd8;
                c.g[A0][0] = LIST as u64;
                w32_2(c, LIST, DATA + 0x600);
                w32_2(
                    c,
                    LIST + 0xa4,
                    if seed % 6 == 0 { r.next_u32() % 3 } else { 3 },
                );
                w32_2(c, LIST + 0xc4, (seed % 5 == 1) as u32);
                w32_2(c, LIST + 0xc8, (seed % 4 == 2) as u32);
                w32_2(c, LIST + 0xb0, (seed % 3 == 0) as u32);
                w32_2(c, LIST + 0xb8, r.next_u32() % 4);
                let bc = if r.next_u32() & 1 == 0 {
                    r.next_u32() | 0x8000_0000
                } else {
                    r.next_u32() & 0x7fff_ffff
                };
                w32_2(c, LIST + 0xbc, bc);
                w32_2(c, oc410_result(17), CMD);
                w32_2(c, CMD + 8, if (seed / 3) % 2 == 0 { 8 } else { 0 });
                w32_2(c, CMD + 0xc, r.next_u32());
                w32_2(c, oc410_result(18), (seed / 7) % 2);
            },
        );
    }

    use crate::difftest::fakes::covered;

    pub(crate) use crate::game::ai::ai::tests::oc425_machine;
    pub(crate) use crate::game::object::splinemodifier::tests::oc425_force_target;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_queue;

    /// `ScriptList_PopEntries`: queues of 1..3 commands (empty queue, movie and
    /// non-movie commands, negative ids), with and without the video engine's
    /// sound (negative and positive sound ids, the audio vtable call). Queue,
    /// movie and audio helpers are fakes (some unwinding); `KillProcess` and
    /// `GetMovieInfo` run for real on an empty request slot and a dummy BE.
    #[test]
    fn script_list_pop_entries_matches() {
        oc425_install();
        covered(
            ee::s07::f_00277598__generated,
            script_list_pop_entries,
            300,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0027_7598, 10);
                crate::game::ai::ai::tests::belibrary(c, &mut r);
                c.pc = 0x0027_7598;
                let me = DATA + 0x10000;
                let engine = DATA + 0x11000;
                let cmd = DATA + 0x12000;
                let audio = DATA + 0x13000;
                c.bus.write32(me, engine);
                c.bus.write32(engine + 0x504, (seed % 3 != 0) as u32);
                // The front command: id at +0xc, key (first word) below 16.
                c.bus.write32(cmd, r.next_u32() % 16);
                c.bus.write32(
                    cmd + 0xc,
                    if seed % 6 == 5 {
                        u32::MAX
                    } else {
                        r.next_u32() % 8
                    },
                );
                for w in 1..7 {
                    if w != 3 {
                        c.bus.write32(cmd + 4 * w, r.next_u32());
                    }
                }
                // The movie table entries (both option variants): sound ids.
                crate::difftest::fill(c, &mut r, 0x0048_23dc, 0x100);
                // The audio system: a vtable with a (fake) method at +0x24.
                let vt = crate::difftest::fakes::vtable(c, &mut r, &[0x20]);
                c.bus.write32(audio + 0x5558, vt);
                oc425_force_target(c, 0x0028_3da0, Some((seed % 5 == 0) as u32), None);
                oc425_force_target(c, 0x0028_3d70, Some(cmd), None);
                oc425_force_target(c, 0x0027_7dd8, Some((seed % 2) as u32), None);
                oc425_force_target(c, 0x0028_3bb8, Some((seed % 4 < 2) as u32), None);
                oc425_force_target(c, 0x0028_b180, Some(audio), None);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(1 + seed % 3);
            },
        );
    }

    /// `ScriptList_Start`: loading, ready and other list states; script and
    /// movie commands; with and without the transition (`self+4`), the fade
    /// flag and the notify flag; every transition kind (0..12, with the
    /// `MakeTransitionObject` of the engine running for real over faked
    /// allocation and constructors) and in/hold/out times (zero, positive,
    /// negative). Everything else it calls is a fake (some unwinding).
    #[test]
    fn script_list_start_matches() {
        oc425_install();
        covered(
            ee::s07::f_00276388__generated,
            script_list_start,
            300,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0027_6388, 20);
                c.pc = 0x0027_6388;
                let gp = c.g[GP][0] as u32;
                let me = DATA + 0x1000;
                let engine = DATA + 0x2000;
                let item = DATA + 0x3000;
                let world = DATA + 0x4000;
                let game = DATA + 0x5000;
                let hud = DATA + 0x6000;
                let tman = DATA + 0x7000;
                let proc = DATA + 0x8000;
                let info = DATA + 0x9000;
                c.bus.write32(gp - 0x848, world);
                c.bus.write32(gp - 0x84c, engine);
                c.bus.write32(gp - 0x4804, 0x3c88_8889);
                c.bus.write32(gp - 0x4800, 0x3c88_8889);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0x48, r.next_u32());
                c.bus.write32(game + 0x64, r.next_u32());
                c.bus.write32(me, engine);
                c.bus.write32(me + 4, (seed % 3 != 0) as u32);
                c.bus.write32(me + 8, r.next_u32());
                c.bus
                    .write32(me + 0xa4, [0, 1, 2, 2, 2, 2, 2, 3][(seed % 8) as usize]);
                let vt = crate::difftest::fakes::vtable(c, &mut r, &[0x30]);
                c.bus.write32(engine + 0x2a8, vt);
                // The video engine: no player (returns 0) or already active (1).
                c.bus.write32(engine + 0x500, (seed % 4 < 2) as u32);
                c.bus.write32(engine + 0x504, r.next_u32() | 1);
                c.bus.write32(item + 8, r.next_u32());
                c.bus.write32(item + 0xc, r.next_u32());
                let hud_vt = crate::difftest::fakes::vtable(c, &mut r, &[0xc0]);
                c.bus.write32(hud + 8, hud_vt);
                c.bus.write32(tman + 0x44, (seed % 5 < 2) as u32);
                c.bus.write32(proc + 0x14, info);
                let times = [0u16, 6, 30, 0xfffc];
                c.bus.write8(info + 8, (r.next_u32() % 13) as u8);
                c.bus.write8(info + 9, (r.next_u32() % 3) as u8);
                c.bus
                    .write16(info + 0xa, times[(r.next_u32() % 4) as usize]);
                c.bus
                    .write16(info + 0xc, times[(r.next_u32() % 4) as usize]);
                c.bus
                    .write16(info + 0xe, times[(r.next_u32() % 4) as usize]);
                oc425_force_target(
                    c,
                    0x0039_f9d8,
                    Some(if seed % 3 == 0 { 0 } else { hud }),
                    None,
                );
                oc425_force_target(c, 0x0028_3d70, Some(item), None);
                oc425_force_target(c, 0x0027_7dd8, Some((seed % 4 == 3) as u32), None);
                oc425_force_target(c, 0x0023_0698, Some(tman), None);
                oc425_force_target(c, 0x0028_2bf0, Some(proc), None);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = if seed % 7 < 3 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
                c.g[A2][0] = if seed % 5 < 2 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
            },
        );
    }

    /// `ScriptList_PrepareEntry`: random queues of entries (categories and
    /// indices, flag 2, ready/unready ids), random movie entries and spawn
    /// results (negative or valid ids), an idle or busy movie player, and
    /// callees that unwind.
    #[test]
    fn script_list_prepare_entry_matches() {
        covered(
            ee::s07::f_00277450__generated,
            script_list_prepare_entry,
            160,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x277450, 24, 0x0027_7450);
                let this = DATA + 0x1000;
                let engine = DATA + 0x2000;
                let entries = DATA + 0x3000;
                c.bus.write32(this, engine);
                // the movie player (+0x500) is idle in two of three seeds
                c.bus
                    .write32(engine + 0x500, if r.next_u32() % 3 == 0 { 1 } else { 0 });
                for k in 0..4u32 {
                    let e = entries + 0x20 * k;
                    c.bus.write32(e, r.next_u32() % 4);
                    c.bus
                        .write32(e + 4, [0xffff_ffff, 2, 5][r.next_u32() as usize % 3]);
                    c.bus
                        .write32(e + 8, if r.next_u32() % 3 == 0 { 2 } else { 0 });
                    c.bus.write32(
                        e + 0xc,
                        [0xffff_ffff, 0xffff_ffff, 7][r.next_u32() as usize % 3],
                    );
                    for w in 0..3 {
                        c.bus.write32(e + 0x10 + 4 * w, r.next_u32());
                    }
                }
                let mut at = [0u64; 12];
                let mut movie = [0u64; 12];
                let mut spawn = [0u64; 12];
                let mut open = [0u64; 12];
                for k in 0..12 {
                    at[k] = (entries + 0x20 * (r.next_u32() % 4)) as u64;
                    movie[k] = (r.next_u32() % 3 == 0) as u64;
                    spawn[k] = [u64::MAX, 0, 3, 9][r.next_u32() as usize % 4];
                    open[k] = (r.next_u32() % 2) as u64;
                }
                oc427_queue(c, 0x0028_3d28, &[1 + (r.next_u32() % 4) as u64]);
                oc427_queue(c, 0x0028_3d70, &at);
                oc427_queue(c, 0x0027_7dd8, &movie);
                oc427_queue(c, 0x0027_95d0, &spawn);
                oc427_queue(c, 0x0027_9528, &spawn);
                oc427_queue(c, 0x0028_38e8, &open);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (r.next_u32() % 3) as u64;
                c.g[A2][0] = (r.next_u32() % 2) as u64;
            },
        );
    }

    use crate::difftest::fakes::FAKE;

    pub(crate) use crate::game::ai::ai::tests::oc428_force;

    pub(crate) use crate::game::be::belibrary::tests::oc431_install;
    pub(crate) use crate::game::be::belibrary::tests::oc431_machine;

    pub(crate) use crate::game::be::belibrary::tests::OC431_BASE;

    pub(crate) use crate::game::main::ssxapp::tests::oc429_force;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_machine;

    pub(crate) use crate::game::main::ssxapp::tests::oc429_w;

    pub(crate) use crate::game::main::ssxapp::tests::OC429_FX;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc428_gate;

    /// `ScriptList_Stop` against the generated body, six situations by seed:
    /// paused/pending-unpause lists (state not 3, so the real
    /// `ScriptList_SetPaused` returns at once), and lists in state 3 with a
    /// cut-scene transition (reset requested or not, with and without the
    /// transition manager's stop, `hud` state found or missing); the other
    /// callees are fakes.
    #[test]
    fn script_list_stop_matches() {
        oc428_install_fakes();
        fakes::covered(
            ee::s07::f_00276868__generated,
            script_list_stop,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x276868, 7);
                oc428_gate(c);
                let (list, app, game, ui) =
                    (DATA + 0x1000, DATA + 0x2000, DATA + 0x2100, DATA + 0x2200);
                let (hud, hud_vt) = (DATA + 0x3000, DATA + 0x3100);
                fill(c, &mut r, list, 0x40);
                let kind = seed % 6;
                let (state, unpause, pending, active) = match kind {
                    0 => (1, 1, 1, 1),
                    1..=3 => (3, 0, 0, 1),
                    4 => (3, 0, 0, 0),
                    _ => (2, 1, 0, 1),
                };
                c.bus.write32(list + 4, active);
                c.bus.write32(list + 0xa4, state);
                c.bus.write32(list + 0xa8, unpause);
                c.bus.write32(list + 0xc4, pending);
                c.bus.write32(list + 0xb0, (kind != 2) as u32);
                c.bus.write32(GAME_GP - 0x848, app);
                c.bus.write32(app + 0x84, game);
                c.bus.write32(game + 0x64, fakes::ptr(&mut r));
                c.bus.write32(game + 0x48, ui);
                c.bus.write32(hud + 8, hud_vt);
                c.bus.write16(hud_vt + 0xc0, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(hud_vt + 0xc4, FAKE + 0x20);
                oc428_force(c, 0x0039_f9d8, if (seed / 6) % 2 == 0 { hud } else { 0 });
                c.g[A0][0] = sx(list);
                c.g[A1][0] = (kind != 3) as u64;
            },
        );
    }

    /// `ScriptList_AddChoice`: queue full, lookup failing / succeeding / off,
    /// every list mode, the "next script already set up" early exit, and the
    /// mode-3 movie / request-player readiness check.
    #[test]
    fn script_list_add_choice_matches() {
        covered(
            ee::s07::f_00275ed0__generated,
            script_list_add_choice,
            200,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x275ed0, 14, 0x0027_5ed0);
                let this = OC429_FX + 0x400;
                let proc_ = OC429_FX + 0x1000;
                // Queue at +0xc: head +0x98, tail +0x9c, full flag +0xa0.
                oc429_w(c, this + 0x98, r.next_u32() % 5);
                oc429_w(c, this + 0x9c, r.next_u32() % 5);
                oc429_w(c, this + 0xa0, (seed % 7 == 0) as u32);
                oc429_w(c, this + 0xa4, [0, 3, 3, 1][(seed % 4) as usize]);
                oc429_w(c, this + 0xbc, if seed % 3 == 0 { 0xffff_fffe } else { 2 });
                oc429_w(c, this, OC429_FX + 0x800);
                oc429_w(c, proc_ + 0x20, if seed % 5 < 2 { 4 } else { r.next_u32() });
                oc429_force(c, 0x0027_97a0, &[if seed % 5 == 0 { u64::MAX } else { 3 }]);
                oc429_force(c, 0x0028_3d28, &[1 + (seed % 4) as u64]);
                oc429_force(c, 0x0027_7dd8, &[(seed % 2) as u64]);
                oc429_force(c, 0x0028_2bf0, &[proc_ as u64]);
                oc429_force(c, 0x0028_3bb8, &[((seed / 2) % 2) as u64]);
                c.g[A0][0] = this as u64;
                for reg in [A1, A2, T0, T1, T2] {
                    c.g[reg][0] = r.next_u32() as u64;
                }
                c.g[A3][0] = (seed % 2) as u64 * (1 + (r.next_u32() as u64 % 3));
            },
        );
    }

    /// `DataInstanceMan_AddData`: both vtable paths (big file or not) and
    /// unwinds in each of the calls; the abort flag is cleared around the
    /// header-size query and restored.
    #[test]
    fn data_instance_man_add_data_matches() {
        covered(
            ee::s07::f_00275920__generated,
            data_instance_man_add_data,
            80,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x275920, 4, 0x0027_5920);
                let this = OC429_FX + 0x400;
                let vt = OC429_FX + 0x600;
                oc429_w(c, this + 0xc, vt);
                for (slot, fk) in [(0x10u32, 0u32), (0x20, 1)] {
                    c.bus.write16(vt + slot, (r.next_u32() as u16) & !3);
                    oc429_w(c, vt + slot + 4, fakes::FAKE + 0x10 * fk);
                }
                oc429_w(c, 0x0045_0dd4, r.next_u32());
                oc429_force(c, 0x0027_5cf8, &[(OC429_FX + 0x800) as u64]);
                oc429_force(c, 0x003e_2580, &[if seed % 2 == 0 { 0 } else { 0x40 }]);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// `ScriptEngine_ctor`: both view objects (real `ScriptViewObj_ctor`),
    /// the tables, the anim bank manager (real, with a fake allocation), the
    /// bank slots and the view indices (real).
    #[test]
    fn script_engine_ctor_matches() {
        oc431_install();
        covered(
            ee::s07::f_00277f20__generated,
            script_engine_ctor,
            100,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x277f20, 40, 0x0027_7f20);
                c.g[A0][0] = (OC431_BASE + 0x100) as u64;
                fill(c, &mut r, OC431_BASE + 0x100, 0x2c0);
            },
        );
    }

    pub(crate) use crate::game::ai::ai::tests::oc432_fake;
    pub(crate) use crate::game::ai::ai::tests::oc434_setup;
    pub(crate) use crate::game::ai::ai::tests::oc434_w;
    pub(crate) use crate::game::ai::ai::tests::OC432_CTRL;
    pub(crate) use crate::game::ai::ai::tests::OC432_MAGIC;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_force;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_run;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_w;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC433_FIX;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    oc432_fake!(
        fake_script_pop_entries,
        PREV_SCRIPT_POP_ENTRIES,
        ee::s07::f_00277598__generated,
        0x0027_7598
    );

    oc432_fake!(
        fake_script_cmd_queue_at,
        PREV_SCRIPT_CMD_QUEUE_AT,
        ee::s07::f_00283d70__generated,
        0x0028_3d70
    );

    oc432_fake!(
        fake_script_is_movie_entry,
        PREV_SCRIPT_IS_MOVIE_ENTRY,
        ee::s07::f_00277dd8__generated,
        0x0027_7dd8,
        |c: &mut Cpu| {
            c.g[V0][0] = c.bus.read32(OC432_CTRL + 0x20) as u64;
        }
    );

    oc432_fake!(
        fake_script_play_instance,
        PREV_SCRIPT_PLAY_INSTANCE,
        ee::s07::f_00279720__generated,
        0x0027_9720
    );

    oc432_fake!(
        fake_script_prepare_entry,
        PREV_SCRIPT_PREPARE_ENTRY,
        ee::s07::f_00277450__generated,
        0x0027_7450
    );

    oc432_fake!(
        fake_get_transition_man,
        PREV_GET_TRANSITION_MAN,
        ee::s06::f_00230698__generated,
        0x0023_0698,
        |c: &mut Cpu| {
            c.g[V0][0] = (DATA + 0x5_6000) as u64;
        }
    );

    oc432_fake!(
        fake_transition_time_remaining,
        PREV_TRANSITION_TIME_REMAINING,
        ee::s09::f_002e4678__generated,
        0x002e_4678
    );

    oc432_fake!(
        fake_transition_start_out,
        PREV_TRANSITION_START_OUT,
        ee::s09::f_002ea780__generated,
        0x002e_a780
    );

    /// `ScriptList_NextEntry`: pending entry (> 0 or not), movie or script
    /// command (the movie start runs real and returns early on a null
    /// movie), `self + 0xb0` on/off, fade on/off, transition running or not.
    #[test]
    fn script_list_next_entry_matches() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277598),
            fake_script_pop_entries,
            &PREV_SCRIPT_POP_ENTRIES,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283D70),
            fake_script_cmd_queue_at,
            &PREV_SCRIPT_CMD_QUEUE_AT,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277DD8),
            fake_script_is_movie_entry,
            &PREV_SCRIPT_IS_MOVIE_ENTRY,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00279720),
            fake_script_play_instance,
            &PREV_SCRIPT_PLAY_INSTANCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277450),
            fake_script_prepare_entry,
            &PREV_SCRIPT_PREPARE_ENTRY,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00276868),
            fake_script_list_stop,
            &PREV_SCRIPT_LIST_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00230698),
            fake_get_transition_man,
            &PREV_GET_TRANSITION_MAN,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E4678),
            fake_transition_time_remaining,
            &PREV_TRANSITION_TIME_REMAINING,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002EA780),
            fake_transition_start_out,
            &PREV_TRANSITION_START_OUT,
        );
        fakes::covered(
            ee::s07::f_00277838__generated,
            script_list_next_entry,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x277838, 9);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.bus
                    .write32(OC432_CTRL + 0x20, (r.next_u32() % 3 == 0) as u32);
                let this = DATA + 0x5_0000;
                let engine = DATA + 0x5_1000;
                let vt = DATA + 0x5_2000;
                let world = DATA + 0x5_3000;
                let game = DATA + 0x5_4000;
                let fade = DATA + 0x5_5000;
                let tm = DATA + 0x5_6000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this, engine);
                c.bus.write32(engine + 0x2a8, vt);
                c.bus.write16(vt + 0x30, (r.next_u32() % 32) as u16);
                c.bus.write32(vt + 0x34, fakes::FAKE);
                c.bus.write32(
                    this + 0xbc,
                    [0xffff_ffffu32, 0, 1, 5][(r.next_u32() % 4) as usize],
                );
                c.bus.write32(this + 0xb0, (r.next_u32() % 2) as u32);
                c.bus.write32(this + 0x4, (r.next_u32() % 3 != 0) as u32);
                c.bus
                    .write32(crate::difftest::GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0x64, fade);
                c.bus.write32(tm + 0x44, (r.next_u32() % 2) as u32);
            },
        );
    }

    /// `ScriptList_Prepare` against the generated body: list state 0, 1 or 2;
    /// empty or filled queue; the first and second entry preparation failing
    /// or not; movie and script entries (loading or ready).
    #[test]
    fn script_list_prepare_matches() {
        oc433_run(
            ee::s07::f_00276270__generated,
            script_list_prepare,
            0x0027_6270,
            9,
            240,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (this, engine, entry) = (f + 0x1000, f + 0x1100, f + 0x1800);
                oc433_w(c, this, engine);
                oc433_w(c, this + 0xa4, [0u32, 0, 0, 0, 1, 2][(seed % 6) as usize]);
                oc433_w(
                    c,
                    entry + 0xc,
                    if (seed / 6) % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 9
                    },
                );
                oc433_force(c, 0x0028_3d28, &[((seed / 6) % 5 != 1) as i32]);
                oc433_force(
                    c,
                    0x0027_7450,
                    &[
                        if (seed / 30) % 4 == 0 { -1 } else { 3 },
                        if (seed / 7) % 2 == 0 { -1 } else { 4 },
                    ],
                );
                oc433_force(c, 0x0028_3d70, &[entry as i32]);
                oc433_force(c, 0x0027_7dd8, &[(seed / 2 % 2) as i32]);
                oc433_force(c, 0x0028_2798, &[(seed / 3 % 2) as i32]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    /// `ScriptList_SkipCurrent` against the generated body: list state 3 or
    /// not, skip already requested or not, the playing entry skippable or not,
    /// no / same / different pending entry, movie and script pending entries.
    #[test]
    fn script_list_skip_current_matches() {
        oc433_run(
            ee::s07::f_00276f48__generated,
            script_list_skip_current,
            0x0027_6f48,
            6,
            240,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (this, engine, e1, e2) = (f + 0x1000, f + 0x1100, f + 0x1800, f + 0x1900);
                oc433_w(c, this, engine);
                oc433_w(c, this + 0xa4, if seed % 11 == 0 { 2 } else { 3 });
                oc433_w(c, this + 0xc8, (seed % 13 == 0) as u32);
                oc433_w(c, this + 0xa8, 1);
                let pending = [u32::MAX, 0, 2, 5][(seed / 3 % 4) as usize];
                oc433_w(c, this + 0xbc, pending);
                oc433_w(
                    c,
                    e1 + 8,
                    if seed % 7 == 0 {
                        0
                    } else {
                        1 | (r.next_u32() & 0xe)
                    },
                );
                oc433_w(
                    c,
                    e2 + 0xc,
                    if (seed / 5) % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 9
                    },
                );
                oc433_force(c, 0x0028_3d70, &[e1 as i32, e2 as i32]);
                let prepared = match (seed / 12) % 4 {
                    0 => pending as i32,
                    1 => -1,
                    _ => 4,
                };
                oc433_force(c, 0x0027_7450, &[prepared]);
                oc433_force(c, 0x0027_7dd8, &[(seed / 2 % 2) as i32]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `ScriptEngine_LoadMaster`: the master table file fails to load (the file queue
    /// fake returns 0), so the header is the words at address 0: its count of script
    /// entries (0..3) is cleared (no debug base), then the real
    /// `ScriptEngine_CreateScriptDatabase` runs.
    #[test]
    fn script_engine_load_master_matches() {
        fakes::covered(
            ee::s07::f_00278210__generated,
            script_engine_load_master,
            16,
            &|c, seed| {
                let _ = oc434_setup(c, seed, 0x278210, 8, 0x0027_8210);
                c.g[A0][0] = (DATA + 0x100) as u64;
                oc434_w(c, GAME_GP.wrapping_add(0x3a4), DATA + 0x2000);
                oc434_w(c, GAME_GP.wrapping_add(0x3a8), DATA + 0x2100);
                oc434_w(c, 0, seed % 4);
                for i in 0..4 {
                    oc434_w(c, 4 + 12 * i + 8, 0x1111_1111);
                }
            },
        );
    }

    /// `DataInstanceMan_RemoveData`: an entry alone, or with a chain of 1..3 linked nodes.
    #[test]
    fn data_instance_man_remove_data_matches() {
        fakes::covered(
            ee::s07::f_00275a20__generated,
            data_instance_man_remove_data,
            32,
            &|c, seed| {
                let mut r = oc434_setup(c, seed, 0x275a20, 10, 0x0027_5a20);
                let (this, entry, vt) = (DATA + 0x100, DATA + 0x200, DATA + 0x300);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = entry as u64;
                oc434_w(c, this + 0xc, vt);
                c.bus.write16(vt + 0x18, 0x20);
                oc434_w(c, vt + 0x1c, FAKE);
                c.bus.write16(vt + 0x28, 0x30);
                oc434_w(c, vt + 0x2c, FAKE + 0x10);
                let chain = seed % 4;
                let node = |i: u32| DATA + 0x400 + 0x20 * i;
                oc434_w(c, entry + 0x14, if chain > 0 { node(0) } else { 0 });
                for i in 0..chain {
                    oc434_w(
                        c,
                        node(i) + 0x14,
                        if i + 1 < chain { node(i + 1) } else { 0 },
                    );
                }
                for f in [0, 4, 8, 0xc] {
                    oc434_w(c, entry + f, r.next_u32());
                    for i in 0..3 {
                        oc434_w(c, node(i) + f, r.next_u32());
                    }
                }
            },
        );
    }

    pub(crate) use crate::game::camera::cameracontroller::tests::oc437_checked;

    /// `pending` non-zero (no `NextEntry`). The real `CheckNextReady` takes its
    /// early exit with the current index (+0xbc) negative. The `NextEntry`
    /// branch is not tested: its real callee chain (`PopEntries`, `Stop`, ...)
    /// runs away on fixture data and the gated fake is not reached.
    #[test]
    fn script_list_try_advance_matches() {
        oc437_checked(
            ee::s07::f_00277800__generated,
            script_list_try_advance,
            0x0027_7800,
            &|c, r, _seed| {
                let obj = c.g[A0][0] as u32;
                c.bus
                    .write32(obj + OC437_SCRIPT_LIST_PENDING, r.next_u32() | 1);
                c.bus
                    .write32(obj + 0xbc, 0xffff_fff0 | (r.next_u32() & 0xf));
            },
        );
    }

    /// `ScriptList_GetFieldB0`: loads the +0xb0 word sign-extended.
    #[test]
    fn script_list_get_field_b0_matches() {
        check(
            ee::s07::f_002772b8__generated,
            script_list_get_field_b0,
            40,
            &|c, seed| {
                let mut r = Rng(0x2772_b800 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0027_72b8;
                let this = DATA + 0x100 * (seed % 8);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0xb0, r.next_u32());
            },
        );
    }

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::fixture;

    #[test]
    fn script_list_set_paused_nested_matches() {
        check(
            ee::s07::f_002766b0__generated,
            script_list_set_paused_nested,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2766b0, 0);
                let list = DATA + 0x700;
                c.g[A0][0] = sx(list);
                c.g[A1][0] = r.next_u32() as u64;
                // ScriptList_SetPaused's early exit: a state word other than 3.
                let states: [u32; 4] = [0, 1, 2, 4];
                c.bus.write32(list + 0xa4, states[(seed % 4) as usize]);
            },
        );
    }

    #[test]
    fn script_list_get_entry_ext_type_matches() {
        check(
            ee::s07::f_00276a08__generated,
            script_list_get_entry_ext_type,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x276a08, 0);
                let list = DATA + 0x200;
                let queue = list + 0xc;
                c.g[A0][0] = sx(list);
                c.g[A1][0] = (seed % 6) as u64;
                c.bus
                    .write32(queue + 0x94, if seed % 5 == 0 { 1 } else { 0 });
                c.bus.write32(queue + 0x8c, r.next_u32() % 6);
                c.bus.write32(queue + 0x90, r.next_u32() % 6);
                // Ring entries (28 bytes each): type words below and above 29.
                for k in 0..5 {
                    let v = if (seed + k) % 2 == 0 {
                        r.next_u32() % 29
                    } else {
                        29 + r.next_u32() % 100
                    };
                    c.bus.write32(queue + k * 28, v);
                }
            },
        );
    }

    #[test]
    fn script_list_count_matches() {
        check(
            ee::s07::f_00276ca8__generated,
            script_list_count,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x276ca8, 0);
                let list = DATA + 0x200;
                let queue = list + 0xc;
                c.g[A0][0] = sx(list);
                c.bus
                    .write32(queue + 0x94, if seed % 4 == 0 { 1 } else { 0 });
                c.bus.write32(queue + 0x8c, r.next_u32() % 6);
                c.bus.write32(queue + 0x90, r.next_u32() % 6);
            },
        );
    }

    #[test]
    fn script_list_is_full_matches() {
        check(
            ee::s07::f_002771a8__generated,
            script_list_is_full,
            200,
            &|c, seed| {
                let mut r = fixture(c, seed, 1);
                fill(c, &mut r, DATA, 8);
            },
        );
    }

    #[test]
    fn script_list_is_playing_movie_matches() {
        check(
            ee::s07::f_00277248__generated,
            script_list_is_playing_movie,
            200,
            &|c, seed| {
                fixture(c, seed, 2);
                c.bus
                    .write32(DATA + 0xa4, if seed % 3 == 0 { 0 } else { 3 });
            },
        );
    }

    /// Gate word in the machine's RAM: the callee fakes below act only when it holds
    /// `OC538_MAGIC` (the second word is the start state the instance fake writes).
    pub(crate) const OC538_GATE: u32 = DATA + 0x2f_0538;

    pub(crate) const OC538_MAGIC: u32 = 0x0538_0001;

    /// A gated fake: falls back to the previous body unless the gate is set, else it logs
    /// a call to `$addr` and returns through the logged return address.
    macro_rules! gated_fake {
        ($fake:ident, $prev:ident, $addr:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC538_GATE) != OC538_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    gated_fake!(
        oc538_fake_release,
        OC538_PREV_RELEASE,
        0x0027_46f8,
        ee::s07::f_002746f8__generated
    );

    gated_fake!(
        oc538_fake_delete,
        OC538_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );

    gated_fake!(
        oc538_fake_pop,
        OC538_PREV_POP,
        0x0027_7598,
        ee::s07::f_00277598__generated
    );

    gated_fake!(
        oc538_fake_prepare,
        OC538_PREV_PREPARE,
        0x0027_6270,
        ee::s07::f_00276270__generated
    );

    gated_fake!(
        oc538_fake_stop,
        OC538_PREV_STOP,
        0x0027_6868,
        ee::s07::f_00276868__generated
    );

    gated_fake!(
        oc538_fake_prepare_entry,
        OC538_PREV_PREPARE_ENTRY,
        0x0027_7450,
        ee::s07::f_00277450__generated
    );

    gated_fake!(
        oc538_fake_kill,
        OC538_PREV_KILL,
        0x0028_2540,
        ee::s07::f_00282540__generated
    );

    gated_fake!(
        oc538_fake_movie_stop,
        OC538_PREV_MOVIE_STOP,
        0x0028_3818,
        ee::s07::f_00283818__generated
    );

    gated_fake!(
        oc538_fake_try_advance,
        OC538_PREV_TRY_ADVANCE,
        0x0027_7800,
        ee::s07::f_00277800__generated
    );

    pub(crate) static OC538_PREV_START: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC538_PREV_UPDATE: AtomicUsize = AtomicUsize::new(0);

    /// `InSeq_InstanceStart` fake: the instance's state word (+0x20) takes the gate's
    /// second word, so the run-to-end test covers both the idle and the looping path.
    pub(crate) fn oc538_fake_start(c: &mut Cpu) {
        if c.bus.read32(OC538_GATE) != OC538_MAGIC {
            return fakes::fall_back(c, &OC538_PREV_START, ee::s07::f_00274830__generated);
        }
        let inst = c.g[A0][0] as u32;
        let state = c.bus.read32(OC538_GATE + 4);
        c.bus.write32(inst + 0x20, state);
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0027_4830, ra);
    }

    /// `InSeq_InstanceUpdate` fake: the instance stops at once (state word 0).
    pub(crate) fn oc538_fake_update(c: &mut Cpu) {
        if c.bus.read32(OC538_GATE) != OC538_MAGIC {
            return fakes::fall_back(c, &OC538_PREV_UPDATE, ee::s07::f_00274a30__generated);
        }
        let inst = c.g[A0][0] as u32;
        c.bus.write32(inst + 0x20, 0);
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0027_4a30, ra);
    }

    /// Installs the fakes (once per process; each one acts only behind the gate).
    pub(crate) fn install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002746F8),
            oc538_fake_release,
            &OC538_PREV_RELEASE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc538_fake_delete,
            &OC538_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00274830),
            oc538_fake_start,
            &OC538_PREV_START,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00274A30),
            oc538_fake_update,
            &OC538_PREV_UPDATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277598),
            oc538_fake_pop,
            &OC538_PREV_POP,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00276270),
            oc538_fake_prepare,
            &OC538_PREV_PREPARE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00276868),
            oc538_fake_stop,
            &OC538_PREV_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277450),
            oc538_fake_prepare_entry,
            &OC538_PREV_PREPARE_ENTRY,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00282540),
            oc538_fake_kill,
            &OC538_PREV_KILL,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283818),
            oc538_fake_movie_stop,
            &OC538_PREV_MOVIE_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277800),
            oc538_fake_try_advance,
            &OC538_PREV_TRY_ADVANCE,
        );
    }

    /// Scrambled machine with the callee gate on (start state from the seed).
    pub(crate) fn gated(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(OC538_GATE, OC538_MAGIC);
        c.bus.write32(OC538_GATE + 4, seed % 2);
        r
    }

    /// A script list at `list`: engine pointer at +0, command queue at +0xc (full flag at
    /// +0x94, head/tail at +0x8c/+0x90, five 28-byte entries with a type word and a
    /// handle at +0xc).
    pub(crate) fn queue_fixture(c: &mut Cpu, r: &mut Rng, list: u32, seed: u32) {
        let queue = list + 0xc;
        c.bus.write32(list, DATA + 0x1000);
        c.bus
            .write32(queue + 0x94, if seed % 5 == 0 { 1 } else { 0 });
        c.bus.write32(queue + 0x8c, r.next_u32() % 6);
        c.bus.write32(queue + 0x90, r.next_u32() % 6);
        for k in 0..5 {
            let v = if (seed + k) % 2 == 0 {
                r.next_u32() % 29
            } else {
                29 + r.next_u32() % 100
            };
            c.bus.write32(queue + k * 28, v);
            let handle = if (seed + k) % 3 == 0 {
                u32::MAX
            } else {
                r.next_u32() % 4
            };
            c.bus.write32(queue + k * 28 + 0xc, handle);
        }
    }

    #[test]
    fn script_list_add_movie_matches() {
        install_fakes();
        check(
            ee::s07::f_00276048__generated,
            script_list_add_movie,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x276048);
                let list = DATA + 0x200;
                queue_fixture(c, &mut r, list, seed);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = r.next_u32() as u64;
                // States 0 (idle) to 3; pending entry -1 to 3.
                let states = [0u32, 1, 2, 3];
                c.bus.write32(list + 0xa4, states[(seed % 4) as usize]);
                c.bus.write32(list + 0xbc, (seed % 5).wrapping_sub(1));
                c.pc = 0x0027_6048;
            },
        );
    }

    #[test]
    fn script_list_get_entry_script_index_matches() {
        install_fakes();
        check(
            ee::s07::f_00276a78__generated,
            script_list_get_entry_script_index,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x276a78);
                let list = DATA + 0x200;
                queue_fixture(c, &mut r, list, seed);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = (seed % 6) as u64;
                c.pc = 0x0027_6a78;
            },
        );
    }

    #[test]
    fn script_list_get_entry_script_matches() {
        install_fakes();
        check(
            ee::s07::f_00276b08__generated,
            script_list_get_entry_script,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x276b08);
                let list = DATA + 0x200;
                queue_fixture(c, &mut r, list, seed);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = (seed % 6) as u64;
                c.pc = 0x0027_6b08;
            },
        );
    }

    #[test]
    fn script_list_is_next_ready_matches() {
        install_fakes();
        check(
            ee::s07::f_00277100__generated,
            script_list_is_next_ready,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x277100);
                let list = DATA + 0x200;
                queue_fixture(c, &mut r, list, seed);
                c.g[A0][0] = sx(list);
                let states = [0u32, 1, 2, 3];
                c.bus.write32(list + 0xa4, states[(seed % 4) as usize]);
                c.bus.write32(list + 0xbc, (seed % 5).wrapping_sub(1));
                c.pc = 0x0027_7100;
            },
        );
    }

    /// Gate word: the callee fakes act only in machines that hold it.
    pub(crate) const OC544_GATE: u32 = DATA + 0xe_0000;

    pub(crate) const OC544_MAGIC: u32 = 0x0544_0001;

    /// The fake's answers: the target (+0x10 of the gate) and the process state (+0x14).
    pub(crate) const OC544_TARGET_AT: u32 = OC544_GATE + 0x10;

    pub(crate) const OC544_STATE_AT: u32 = OC544_GATE + 0x14;

    /// The table `ScriptList_GetMovieInfo` copies (game RAM at 0x535610).
    pub(crate) const OC544_TABLE: u32 = 0x0053_5610;

    pub(crate) const OC544_WORLD: u32 = DATA + 0x1000;

    pub(crate) const OC544_ENGINE: u32 = DATA + 0x2000;

    pub(crate) const OC544_GAME: u32 = DATA + 0x3000;

    pub(crate) const OC544_TMAN: u32 = DATA + 0x4000;

    pub(crate) const OC544_CUTMAN: u32 = DATA + 0x5000;

    pub(crate) const OC544_LIST: u32 = DATA + 0x6000;

    pub(crate) const OC544_INFO: u32 = DATA + 0x7000;

    pub(crate) const OC544_PROC: u32 = DATA + 0x8000;

    pub(crate) const OC544_BE: u32 = DATA + 0x9000;

    pub(crate) const OC544_KEY: u32 = DATA + 0xa000;

    pub(crate) const OC544_TARGET: u32 = DATA + 0xb000;

    /// Gated fake: records the call (`fakes::fake`), then sets the result.
    macro_rules! oc544_gated {
        ($fake:ident, $prev:ident, $gen:path, $target:expr, |$c:ident| $body:block) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake($c: &mut Cpu) {
                if $c.bus.read32(OC544_GATE) != OC544_MAGIC {
                    return fakes::fall_back($c, &$prev, $gen);
                }
                let ra = $c.g[RA][0] as u32;
                fakes::fake($c, $target, ra);
                $body
            }
        };
    }

    oc544_gated!(
        oc544_fake_get_be,
        OC544_PREV_GET_BE,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80,
        |c| {
            c.g[V0][0] = sx(OC544_BE);
        }
    );

    oc544_gated!(
        oc544_fake_get_iface,
        OC544_PREV_GET_IFACE,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58,
        |c| {}
    );

    oc544_gated!(
        oc544_fake_proc_in_seq,
        OC544_PREV_PROC_IN_SEQ,
        ee::s07::f_00282bf0__generated,
        0x0028_2bf0,
        |c| {
            c.g[V0][0] = sx(OC544_PROC);
        }
    );

    oc544_gated!(
        oc544_fake_script_info,
        OC544_PREV_SCRIPT_INFO,
        ee::s07::f_0027c070__generated,
        0x0027_c070,
        |c| {
            c.g[V0][0] = sx(OC544_INFO);
        }
    );

    oc544_gated!(
        oc544_fake_transition_man,
        OC544_PREV_TRANSITION_MAN,
        ee::s06::f_00230698__generated,
        0x0023_0698,
        |c| {
            c.g[V0][0] = sx(OC544_TMAN);
        }
    );

    oc544_gated!(
        oc544_fake_tman_stop,
        OC544_PREV_TMAN_STOP,
        ee::s09::f_002e4578__generated,
        0x002e_4578,
        |c| {}
    );

    oc544_gated!(
        oc544_fake_target,
        OC544_PREV_TARGET,
        ee::s07::f_0027a4a0__generated,
        0x0027_a4a0,
        |c| {
            c.g[V0][0] = sx(c.bus.read32(OC544_TARGET_AT));
        }
    );

    oc544_gated!(
        oc544_fake_captured,
        OC544_PREV_CAPTURED,
        ee::s09::f_002e4540__generated,
        0x002e_4540,
        |c| {}
    );

    oc544_gated!(
        oc544_fake_start_out,
        OC544_PREV_START_OUT,
        ee::s09::f_002e44f0__generated,
        0x002e_44f0,
        |c| {}
    );

    oc544_gated!(
        oc544_fake_process_state,
        OC544_PREV_PROCESS_STATE,
        ee::s07::f_00282798__generated,
        0x0028_2798,
        |c| {
            c.g[V0][0] = sx(c.bus.read32(OC544_STATE_AT));
        }
    );

    oc544_gated!(
        oc544_fake_pause_process,
        OC544_PREV_PAUSE_PROCESS,
        ee::s07::f_002826d8__generated,
        0x0028_26d8,
        |c| {}
    );

    oc544_gated!(
        oc544_fake_resume_process,
        OC544_PREV_RESUME_PROCESS,
        ee::s07::f_00282720__generated,
        0x0028_2720,
        |c| {}
    );

    oc544_gated!(
        oc544_fake_movie_paused,
        OC544_PREV_MOVIE_PAUSED,
        ee::s07::f_00283b78__generated,
        0x0028_3b78,
        |c| {}
    );

    oc544_gated!(
        oc544_fake_cutscene_pause,
        OC544_PREV_CUTSCENE_PAUSE,
        ee::s09::f_002ea8b8__generated,
        0x002e_a8b8,
        |c| {}
    );

    oc544_gated!(
        oc544_fake_pause_transition,
        OC544_PREV_PAUSE_TRANSITION,
        ee::s09::f_002e4638__generated,
        0x002e_4638,
        |c| {}
    );

    /// Puts every fake in its override slot (once per process).
    pub(crate) fn oc544_install() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc544_fake_get_be,
            &OC544_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc544_fake_get_iface,
            &OC544_PREV_GET_IFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00282BF0),
            oc544_fake_proc_in_seq,
            &OC544_PREV_PROC_IN_SEQ,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027C070),
            oc544_fake_script_info,
            &OC544_PREV_SCRIPT_INFO,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00230698),
            oc544_fake_transition_man,
            &OC544_PREV_TRANSITION_MAN,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E4578),
            oc544_fake_tman_stop,
            &OC544_PREV_TMAN_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027A4A0),
            oc544_fake_target,
            &OC544_PREV_TARGET,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E4540),
            oc544_fake_captured,
            &OC544_PREV_CAPTURED,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E44F0),
            oc544_fake_start_out,
            &OC544_PREV_START_OUT,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00282798),
            oc544_fake_process_state,
            &OC544_PREV_PROCESS_STATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002826D8),
            oc544_fake_pause_process,
            &OC544_PREV_PAUSE_PROCESS,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00282720),
            oc544_fake_resume_process,
            &OC544_PREV_RESUME_PROCESS,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283B78),
            oc544_fake_movie_paused,
            &OC544_PREV_MOVIE_PAUSED,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002EA8B8),
            oc544_fake_cutscene_pause,
            &OC544_PREV_CUTSCENE_PAUSE,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E4638),
            oc544_fake_pause_transition,
            &OC544_PREV_PAUSE_TRANSITION,
        );
    }

    /// The gate, the world (gp-0x848), the engine (gp-0x84c), the game and the
    /// transition manager, and the two transition floats (gp-0x47fc, -0x47f8).
    pub(crate) fn oc544_world(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(OC544_GATE, OC544_MAGIC);
        let gp = c.g[GP][0] as u32;
        c.bus.write32(gp.wrapping_sub(0x848), OC544_WORLD);
        c.bus.write32(gp.wrapping_sub(0x84c), OC544_ENGINE);
        c.bus.write32(gp.wrapping_sub(0x47fc), 0x3c88_8889);
        c.bus.write32(gp.wrapping_sub(0x47f8), 0x3c88_8889);
        c.bus.write32(OC544_WORLD + 0x84, OC544_GAME);
        c.bus.write32(OC544_GAME + 0x64, OC544_CUTMAN);
        c.bus.write32(OC544_TMAN + 0x44, r.next_u32() % 2);
    }

    /// The command queue of a list at `list` (ring of five 28-byte entries
    /// whose type words are all movies (>= 29) or all scripts).
    pub(crate) fn oc544_queue(c: &mut Cpu, r: &mut Rng, list: u32, movie: bool) {
        let queue = list + 0xc;
        c.bus.write32(queue + 0x8c, r.next_u32() % 6);
        for k in 0..5 {
            c.bus
                .write32(queue + k * 28, if movie { 29 + k } else { k % 29 });
        }
    }

    /// The copied table (0x535610) with random words; its first word's flags
    /// (masked 0x300000) pick the record table.
    pub(crate) fn oc544_table(c: &mut Cpu, r: &mut Rng, seed: u32) {
        for i in 0..(0x288 / 4) {
            c.bus.write32(OC544_TABLE + 4 * i, r.next_u32());
        }
        let w0 = if seed % 2 == 0 {
            (r.next_u32() & !0x0030_0000) | 0x0020_0000
        } else {
            r.next_u32() | 0x0030_0000
        };
        c.bus.write32(OC544_TABLE, w0);
    }

    #[test]
    fn script_list_get_movie_info_matches() {
        oc544_install();
        check(
            ee::s07::f_00277de8__generated,
            script_list_get_movie_info,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x277de8, 0);
                c.bus.write32(OC544_GATE, OC544_MAGIC);
                c.g[A1][0] = sx(OC544_KEY);
                c.bus.write32(OC544_KEY, r.next_u32() % 40);
                oc544_table(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn script_list_set_paused_matches() {
        oc544_install();
        check(
            ee::s07::f_00277c08__generated,
            script_list_set_paused,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x277c08, 0);
                oc544_world(c, &mut r);
                let list = OC544_LIST;
                c.g[A0][0] = sx(list);
                c.g[A1][0] = ((seed / 2) % 2) as u64;
                c.g[A2][0] = (seed % 3 == 0) as u64;
                c.bus.write32(list, OC544_ENGINE);
                c.bus.write32(list + 0x4, (seed % 5 < 2) as u32);
                c.bus
                    .write32(list + 0xa4, if seed % 6 == 0 { 2 } else { 3 });
                c.bus.write32(list + 0xa8, (seed % 4 == 1) as u32);
                c.bus.write32(list + 0xc4, r.next_u32() % 3);
                oc544_queue(c, &mut r, list, seed % 3 == 1);
                c.bus.write32(OC544_STATE_AT, seed % 7);
            },
        );
    }

    #[test]
    fn script_list_start_transition_matches() {
        oc544_install();
        check(
            ee::s07::f_00277980__generated,
            script_list_start_transition,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x277980, 0);
                oc544_world(c, &mut r);
                let list = OC544_LIST;
                c.g[A0][0] = sx(list);
                c.bus.write32(list, OC544_ENGINE);
                // Idle (the queue's next entry), waiting, or a pending stop.
                c.bus.write32(list + 0xb0, (seed % 2) as u32);
                c.bus
                    .write32(list + 0xac, if seed % 4 == 1 { 0 } else { 1 });
                c.bus.write32(list + 0xb8, r.next_u32());
                let movie = seed % 3 == 0;
                oc544_queue(c, &mut r, list, movie);
                // Movie records (both tables) for the five entries' keys.
                for key in 29..34 {
                    for base in [0x0048_247c_u32, 0x0048_23dc] {
                        let rec = base + key * 28;
                        c.bus.write32(rec + 4, if seed % 2 == 0 { 7 } else { 3 });
                        c.bus.write32(rec + 8, r.next_u32());
                        let duration = if seed % 4 == 0 {
                            -1.0_f32
                        } else {
                            0.25 + (seed % 3) as f32
                        };
                        c.bus.write32(rec + 0xc, duration.to_bits());
                    }
                }
                oc544_table(c, &mut r, seed);
                // Script info: kind 7 or not, durations that sum to <= 0 or not.
                c.bus
                    .write8(OC544_INFO + 0x10, if seed % 5 == 0 { 7 } else { 2 });
                c.bus.write8(OC544_INFO + 0x11, 3);
                let h12: i16 = if seed % 6 == 0 { -3 } else { 2 };
                c.bus.write16(OC544_INFO + 0x12, h12 as u16);
                c.bus.write16(OC544_INFO + 0x14, 1);
                c.bus
                    .write16(OC544_INFO + 0x16, if seed % 3 == 0 { 3 } else { 0 });
                // The target: none (the script cannot start) or a pointer.
                c.bus.write32(
                    OC544_TARGET_AT,
                    if seed % 7 == 0 { 0 } else { OC544_TARGET },
                );
            },
        );
    }

    #[test]
    fn script_list_take_from_matches() {
        check(
            ee::s07::f_00276198__generated,
            script_list_take_from,
            200,
            &|c, seed| {
                machine(c, seed, 0x6198, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00276198;
            },
        );
    }

    pub(crate) use crate::game::wscript::wscriptman::tests::OC655_MAN;

    pub(crate) use crate::game::wscript::wscriptman::tests::oc655_gates;
    pub(crate) use crate::game::wscript::wscriptman::tests::oc655_hook_all;

    #[test]
    fn script_list_skip_matches() {
        oc655_hook_all();
        covered(
            ee::s07::f_002766d0__generated,
            script_list_skip_2,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_66d0, 3);
                oc655_gates(c, &mut r);
                let list = OC655_MAN;
                c.bus.write32(list + 0xc0, r.next_u32() % 4);
                c.bus
                    .write32(list + 0xbc, (r.next_u32() % 6).wrapping_sub(1));
                c.bus.write32(list + 0xa4, r.next_u32() % 4);
                c.bus.write32(list + 0xa8, r.next_u32() % 2);
                c.bus.write32(list + 0xb4, r.next_u32() % 2);
                c.g[A0][0] = list as u64;
                c.g[A1][0] = (((r.next_u32() % 5) as i32) - 2) as u32 as u64;
                c.g[A2][0] = (r.next_u32() % 2) as u64;
            },
        );
    }

    /// `DataInstanceMan_OnDataAdded` (0x0028_47a8) against the generated body.
    #[test]
    fn data_instance_man_on_data_added_002847a8_matches() {
        check(
            ee::s07::f_002847a8__generated,
            data_instance_man_on_data_added_002847a8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2847a8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DataInstanceMan_OnDataRemoved` (0x0028_47b0) against the generated body.
    #[test]
    fn data_instance_man_on_data_removed_002847b0_matches() {
        check(
            ee::s07::f_002847b0__generated,
            data_instance_man_on_data_removed_002847b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2847b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DataInstanceMan_v04` (0x0028_47b8) against the generated body.
    #[test]
    fn data_instance_man_v04_002847b8_matches() {
        check(
            ee::s07::f_002847b8__generated,
            data_instance_man_v04_002847b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2847b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DataInstanceMan_v05` (0x0028_47c0) against the generated body.
    #[test]
    fn data_instance_man_v05_002847c0_matches() {
        check(
            ee::s07::f_002847c0__generated,
            data_instance_man_v05_002847c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2847c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
