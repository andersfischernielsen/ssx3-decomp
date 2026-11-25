//! `src/main/gamemode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gp;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::shl32;
pub(crate) use crate::game::fe::fepopup::at as at_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw;
pub(crate) use crate::game::fe::fepopup::sq;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::sound::bankmonitor::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0023_80e8, game_mode_man_destroy, "GameModeMan_Destroy"),
    (0x0023_8e20, gm_handler0_reset, "GMHandler0_Reset"),
    (0x0023_98e8, gm_handler0_v06, "GMHandler0_v06"),
    (
        0x0023_8510,
        game_mode_man_handler_v06,
        "GameModeMan_HandlerV06",
    ),
    (
        0x0023_8550,
        game_mode_man_handler_v07,
        "GameModeMan_HandlerV07",
    ),
    (
        0x0023_8590,
        game_mode_man_handler_v09,
        "GameModeMan_HandlerV09",
    ),
    (
        0x0023_8b70,
        game_mode_man_sort_by_score,
        "GameModeMan_SortByScore",
    ),
    (
        0x0023_8bf8,
        game_mode_man_sort_by_time,
        "GameModeMan_SortByTime",
    ),
    (0x0023_8c80, gm_handler_start, "GMHandler_Start"),
    (
        0x0023_8d30,
        gm_handler_clear_standings,
        "GMHandler_ClearStandings",
    ),
    (
        0x0023_8da8,
        gm_handler_clear_standings2,
        "GMHandler_ClearStandings2",
    ),
    (0x0023_9ce0, gm_handler0_v09, "GMHandler0_v09"),
    (0x0023_9d18, gm_handler0_save_state, "GMHandler0_SaveState"),
    (0x0023_9d50, gm_handler0_load_state, "GMHandler0_LoadState"),
    (0x0023_9d88, gm_handler9_reset, "GMHandler9_Reset"),
    (0x0023_a070, gm_handler9_v06, "GMHandler9_v06"),
    (0x0023_ac18, gm_handler1_save_state, "GMHandler1_SaveState"),
    (0x0023_ac50, gm_handler1_load_state, "GMHandler1_LoadState"),
    (
        0x0023_ae00,
        gm_handler8_on_rider_finished,
        "GMHandler8_OnRiderFinished",
    ),
    (0x0023_b038, gm_handler8_save_state, "GMHandler8_SaveState"),
    (0x0023_b070, gm_handler8_load_state, "GMHandler8_LoadState"),
    (0x0023_b268, gm_handler4_reset, "GMHandler4_Reset"),
    (
        0x0023_b468,
        gm_handler4_on_rider_finished,
        "GMHandler4_OnRiderFinished",
    ),
    (0x0023_b5f8, gm_handler4_v06, "GMHandler4_v06"),
    (0x0023_bb98, gm_handler6_reset, "GMHandler6_Reset"),
    (0x0023_c0d0, gm_handler7_reset, "GMHandler7_Reset"),
    (0x0023_c560, gm_handler7_v06, "GMHandler7_v06"),
    (0x0023_c770, game_rand_shuffle, "GameRand_Shuffle"),
    (
        0x0023_7cf8,
        game_mode_man_get_instance,
        "GameModeMan_GetInstance",
    ),
    (0x0023_a108, gm_handler1_reset, "GMHandler1_Reset"),
    (
        0x0023_8160,
        game_mode_man_start_mode,
        "GameModeMan_StartMode",
    ),
    (
        0x0023_a4f0,
        gm_handler1_pick_opponents,
        "GMHandler1_PickOpponents",
    ),
    (
        0x0023_a668,
        gm_handler1_setup_rider_slots,
        "GMHandler1_SetupRiderSlots",
    ),
    (
        0x0023_8358,
        game_mode_man_on_rider_finished,
        "GameModeMan_OnRiderFinished",
    ),
    (
        0x0023_8348,
        ov_screen_yn_dialog_copy_default_choice,
        "OVScreenYNDialog_CopyDefaultChoice",
    ),
    (
        0x0023_85d0,
        game_mode_man_save_state,
        "GameModeMan_SaveState",
    ),
    (
        0x0023_88a0,
        game_mode_man_load_state,
        "GameModeMan_LoadState",
    ),
    (
        0x0023_9230,
        gm_handler0_on_rider_finished,
        "GMHandler0_OnRiderFinished",
    ),
    (0x0023_b170, gm_handler2_reset, "GMHandler2_Reset"),
    (
        0x0023_9aa0,
        gm_handler0_init_standings,
        "GMHandler0_InitStandings",
    ),
    (
        0x0023_9ec8,
        gm_handler9_on_rider_finished,
        "GMHandler9_OnRiderFinished",
    ),
    (0x0023_ac88, gm_handler8_reset_port, "GMHandler8_Reset"),
    (0x0023_b6c0, gm_handler5_reset_port, "GMHandler5_Reset"),
    (
        0x0023_b8c8,
        gm_handler5_on_rider_finished_port,
        "GMHandler5_OnRiderFinished",
    ),
    (
        0x0023_bdb8,
        gm_handler6_on_rider_finished_port,
        "GMHandler6_OnRiderFinished",
    ),
    (
        0x0023_c2d8,
        gm_handler7_on_rider_finished,
        "GMHandler7_OnRiderFinished",
    ),
    (
        0x0023_c618,
        gm_handler_unused_reset,
        "GMHandlerUnused_Reset",
    ),
    (
        0x0023_9938,
        gm_handler0_pick_opponents,
        "GMHandler0_PickOpponents",
    ),
    (
        0x0023_a760,
        gm_handler1_on_rider_finished,
        "GMHandler1_OnRiderFinished",
    ),
    (
        0x0023_b0a8,
        gm_handler8_pick_opponents,
        "GMHandler8_PickOpponents",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::CBE_GET_INTERFACE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::GET_ALT_CHAR;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::GET_CHAR_ID;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEMSET_3 as MEMSET_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;

    pub(crate) use crate::game::fe::fepopup::callee::C00238BF8;

    // s00
    pub const RIDER_METRIX_RESET_STATS: Callee = Callee {
        addr: 0x0011_75b8,
        f: ee::s00::f_001175b8,
    };

    pub const RIDER_METRIX_CLEAR_HUD_MESSAGES: Callee = Callee {
        addr: 0x0011_73b8,
        f: ee::s00::f_001173b8,
    };

    pub const RIDER_METRIX_RESET_TRICK: Callee = Callee {
        addr: 0x0011_7838,
        f: ee::s00::f_00117838,
    };

    pub const RIDER_METRIX_RESET_COMBO: Callee = Callee {
        addr: 0x0011_75f8,
        f: ee::s00::f_001175f8,
    };

    pub const RIDER_METRIX_BANK_PENDING_SCORE: Callee = Callee {
        addr: 0x0011_a7a8,
        f: ee::s00::f_0011a7a8,
    };

    pub const RIDER_GET_SYNC_CHECKSUM: Callee = Callee {
        addr: 0x0012_5ad8,
        f: ee::s00::f_00125ad8,
    };

    // s01
    pub const CBE_GET_BE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };

    pub const BE_GET_RIDER_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::f_0014a0e0,
    };

    pub const BE_NEW_PLAYER_INT_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::f_00147798,
    };

    pub const BE_REWARD_INT_GET_MEDAL_REWARD_ID: Callee = Callee {
        addr: 0x0015_9080,
        f: ee::s01::f_00159080,
    };

    pub const BE_REWARD_INT_IS_REWARD_GRANTED: Callee = Callee {
        addr: 0x0015_a2e0,
        f: ee::s01::f_0015a2e0,
    };

    pub const BE_REWARD_INT_GRANT_REWARD: Callee = Callee {
        addr: 0x0015_9cd0,
        f: ee::s01::f_00159cd0,
    };

    // s02
    pub const CAMERA_TRIGGER_MAN_UNLOAD: Callee = Callee {
        addr: 0x0016_c9b0,
        f: ee::s02::f_0016c9b0,
    };

    pub const CAMERA_TRIGGER_MAN_PARSE: Callee = Callee {
        addr: 0x0016_c228,
        f: ee::s02::f_0016c228,
    };

    pub const CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2: Callee = Callee {
        addr: 0x0016_d1b8,
        f: ee::s02::f_0016d1b8,
    };

    pub const UI_OBJECT_SHOW_CLAMP_ALPHA_HALF: Callee = Callee {
        addr: 0x0019_4498,
        f: ee::s02::f_00194498,
    };

    // s03
    pub const FE_MODULE_UNLOAD_RIDER_PREVIEWS: Callee = Callee {
        addr: 0x001a_04c8,
        f: ee::s03::f_001a04c8,
    };

    pub const FE_MODULE_RIDER_PREVIEW_DTOR: Callee = Callee {
        addr: 0x0019_e498,
        f: ee::s03::f_0019e498,
    };

    pub const FE_ASYNC_MANAGER_SET_FILE_STATUS: Callee = Callee {
        addr: 0x001a_39d8,
        f: ee::s03::f_001a39d8,
    };

    // s07
    pub const REPLAY_FIND_SNAPSHOT: Callee = Callee {
        addr: 0x0026_e448,
        f: ee::s07::f_0026e448,
    };

    pub const RUN_LIST_SEEK: Callee = Callee {
        addr: 0x0026_d4d8,
        f: ee::s07::f_0026d4d8,
    };

    pub const SCRIPT_LIST_ON_SEQ_ADVANCE: Callee = Callee {
        addr: 0x0027_7310,
        f: ee::s07::f_00277310,
    };

    pub const SCRIPT_ENGINE_GET_SEQ_PROCESS_ID: Callee = Callee {
        addr: 0x0028_2be0,
        f: ee::s07::f_00282be0,
    };

    pub const REPLAY_REQUEST_INSTANT_REPLAY: Callee = Callee {
        addr: 0x0027_0658,
        f: ee::s07::f_00270658,
    };

    // s08
    pub const USTR_TO_ASCII: Callee = Callee {
        addr: 0x002c_2580,
        f: ee::s08::f_002c2580,
    };

    pub const CPS2_MEM_CARD_SET_STATE: Callee = Callee {
        addr: 0x002c_48c0,
        f: ee::s08::f_002c48c0,
    };

    // s09
    pub const MENU_SYS_ITEM_DRAW_ROW: Callee = Callee {
        addr: 0x002c_a988,
        f: ee::s09::f_002ca988,
    };

    // s10
    pub const WSCRIPT_MAN_RESET_COLLECT_SET: Callee = Callee {
        addr: 0x0030_c390,
        f: ee::s10::f_0030c390,
    };

    pub const OPERATOR_NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::f_00317e30,
    };

    pub const CBX_STRING_ALLOC_BUFFER: Callee = Callee {
        addr: 0x0031_8040,
        f: ee::s10::f_00318040,
    };

    pub const EA_MEM_FREE: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };

    pub const BUILTIN_DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::f_00317e50,
    };

    // s11
    pub const MODIFIER_BLOCK_HAS_RAIL_MODIFIER: Callee = Callee {
        addr: 0x0035_3418,
        f: ee::s11::f_00353418,
    };

    pub const RAIL_MODIFIER_CTOR: Callee = Callee {
        addr: 0x0035_b708,
        f: ee::s11::f_0035b708,
    };

    pub const MOD_OBJ_SET_RAIL_MODIFIER: Callee = Callee {
        addr: 0x0035_56a8,
        f: ee::s11::f_003556a8,
    };

    pub const MULTI_PARTICLE_DTOR: Callee = Callee {
        addr: 0x0035_7fa0,
        f: ee::s11::f_00357fa0,
    };

    pub const MOD_OBJ_CTOR_2: Callee = Callee {
        addr: 0x0035_52d8,
        f: ee::s11::f_003552d8,
    };

    pub const FLEX_BRIDGE_ALLOC_SPRING_NODES: Callee = Callee {
        addr: 0x0035_3f00,
        f: ee::s11::f_00353f00,
    };

    // s12
    pub const TEX_MAN_UNLOCK_VRAM: Callee = Callee {
        addr: 0x0036_7b60,
        f: ee::s12::f_00367b60,
    };

    // s13
    pub const UI_LIST_NODE_IS_SENTINEL: Callee = Callee {
        addr: 0x0039_7718,
        f: ee::s13::f_00397718,
    };

    pub const UI_LIST_FIRST: Callee = Callee {
        addr: 0x0039_7840,
        f: ee::s13::f_00397840,
    };

    pub const UI_LIST_DELETE_ALL: Callee = Callee {
        addr: 0x0039_7948,
        f: ee::s13::f_00397948,
    };

    pub const UI_LIST_ACCEPT_VISITOR: Callee = Callee {
        addr: 0x0039_79f8,
        f: ee::s13::f_003979f8,
    };

    pub const UI_LIST_NODE_DTOR_BODY: Callee = Callee {
        addr: 0x003a_4868,
        f: ee::s13::f_003a4868,
    };

    pub const CUI_TEXT_SCROLL_SET_WIDE_STRING: Callee = Callee {
        addr: 0x003a_1d30,
        f: ee::s13::f_003a1d30,
    };

    pub const CSTREAM_MAN_RESET: Callee = Callee {
        addr: 0x003a_6ed8,
        f: ee::s13::f_003a6ed8,
    };

    pub const CSTREAM_MAN_FREE_BUFFERS: Callee = Callee {
        addr: 0x003a_6f38,
        f: ee::s13::f_003a6f38,
    };

    // Cross-shard calls the generator wrote as `call_indirect`.
    pub const STR_HASH: u32 = 0x0031_7670;

    pub const SPRINTF: u32 = 0x0041_7828;

    pub const MEMSET: u32 = 0x0041_6210;

    /// The libc `memset` copy in the world/serialisation shard.
    pub const MEMSET_3E6448: u32 = 0x003e_6448;

    pub const MEMCPY: u32 = 0x0041_605c;

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
// GM save/load handlers: `save`/`load` are virtual methods on the handler
// object at `this`; each handler forwards its own state block.

/// One GM handler forwarder: `fn(this, handler, size)` calls the handler's
/// virtual method at `slot` with `(handler + delta, this + data_off, size)`.
pub(crate) fn gm_handler_forward(c: &mut Cpu, this_off: u32, save_slot: u32, size: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(this_off));
        c.g[A2][0] = size;
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(save_slot)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(save_slot + 4)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void GMHandler0_SaveState(GMHandler0* self, Handler* h)` (0x239d18).
pub fn gm_handler0_save_state(c: &mut Cpu) {
    gm_handler_forward(c, 4, 0x08, 0xa0, 0x0023_9d40);
}

/// `void GMHandler0_LoadState(GMHandler0* self, Handler* h)` (0x239d50).
pub fn gm_handler0_load_state(c: &mut Cpu) {
    gm_handler_forward(c, 4, 0x10, 0xa0, 0x0023_9d78);
}

/// `void GMHandler1_SaveState(GMHandler1* self, Handler* h)` (0x23ac18).
pub fn gm_handler1_save_state(c: &mut Cpu) {
    gm_handler_forward(c, 0, 0x08, 0xd0, 0x0023_ac40);
}

/// `void GMHandler1_LoadState(GMHandler1* self, Handler* h)` (0x23ac50).
pub fn gm_handler1_load_state(c: &mut Cpu) {
    gm_handler_forward(c, 0, 0x10, 0xd0, 0x0023_ac78);
}

/// `void GMHandler8_SaveState(GMHandler8* self, Handler* h)` (0x23b038).
pub fn gm_handler8_save_state(c: &mut Cpu) {
    gm_handler_forward(c, 0, 0x08, 0x2c, 0x0023_b060);
}

/// `void GMHandler8_LoadState(GMHandler8* self, Handler* h)` (0x23b070).
pub fn gm_handler8_load_state(c: &mut Cpu) {
    gm_handler_forward(c, 0, 0x10, 0x2c, 0x0023_b098);
}

/// The body of the `GameModeMan_HandlerV*` thunks: `obj =
/// D_00536668[*(int*)(self+4)]`, then `vt = *(void**)obj` and the vtable slot
/// at `slot` called with `a0 = obj + delta` and the other arguments as given.
pub(crate) fn game_mode_handler(c: &mut Cpu, slot: u32, ret: u32) -> Flow {
    c.cycles += 16;
    let frame = Frame::open(c, 0x10);
    c.g[V1][0] = 0x0053_6668;
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(p(abi::arg(c, 0), 4)).lw(c);
    c.g[V0][0] = sx((r32(c, V0) as u32) << 2);
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
    let index_at = r32(c, V0);
    c.g[A3][0] = Addr(index_at).lw(c);
    c.g[V1][0] = Addr(r32(c, A3)).lw(c);
    let vtable = c.g[V1][0];
    let entry = VSlot::read(c, vtable, slot);
    c.g[A0][0] = sx(entry.delta as u32);
    c.g[V0][0] = entry.func;
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = abi::addu(c.g[A3][0], entry.delta);
    let func = entry.func as u32;
    c.pc = func;
    call_indirect(c, func);
    if c.pc != ret {
        return Err(crate::call::Unwound);
    }
    frame.restore_ra(c, 0);
    leave(c, 0x10);
    Ok(())
}

/// `void GameModeMan_HandlerV06(self, a, b)` (0x238510): vtable slot 6.
pub fn game_mode_man_handler_v06(c: &mut Cpu) {
    guest(c, |c| game_mode_handler(c, 0x30, 0x0023_8544))
}

/// `void GameModeMan_HandlerV07(self, a, b)` (0x238550): vtable slot 7.
pub fn game_mode_man_handler_v07(c: &mut Cpu) {
    guest(c, |c| game_mode_handler(c, 0x38, 0x0023_8584))
}

/// `void GameModeMan_HandlerV09(self, a, b)` (0x238590): vtable slot 9.
pub fn game_mode_man_handler_v09(c: &mut Cpu) {
    guest(c, |c| game_mode_handler(c, 0x48, 0x0023_85c4))
}

/// `int GMHandler0_v09(Handler* self, int col, int row)` (0x239ce0): a
/// 0x28-byte-row table lookup `self->rows[row][col]` (word at
/// `+row*0x28 + col*4 + 4`), 0 when `row` is past the count (`gp-0x480`).
pub fn gm_handler0_v09(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = Addr(gpa(c, -0x480)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
    let out_of_range = c.g[V0][0] != 0;
    c.g[V0][0] = 0x28;
    if out_of_range {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.mult(0, 2, 6, 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    abi::ret(c);
}

/// `int GMHandler9_v06(Handler9* self, int player, int other)` (0x23a070):
/// bump `self->score[player]` and report whether it is now strictly ahead of
/// the other player; if so and the race is running, extend the deadline
/// `manager->+0x78` by `other` seconds (in game updates) unless the AI update
/// counter (world->+0x84->+0xc->+0x8) has already passed it (then 0).
pub fn gm_handler9_v06(c: &mut Cpu) {
    const ONE: u64 = 0x1;
    c.cycles += 17;
    let self_ = r32(c, A0);
    let player = r32(c, A1);
    c.g[A0][0] = sx(self_.wrapping_add(0x4)); // score array
    c.g[A3][0] = sx(player << 1);
    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[T1][0] = ONE;
    c.g[V1][0] = c.lhu(c.g[A3][0] as u32);
    c.g[V0][0] = c.g[T1][0];
    if c.g[A1][0] != 0 {
        c.g[V0][0] = 0;
    }
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.sh(c.g[A3][0] as u32, c.g[V1][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
    c.g[V0][0] = c.lh(c.g[A0][0] as u32);
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let ahead = c.g[V0][0] != 0;
    c.g[T0][0] = c.g[A2][0];
    if !ahead {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[A1][0] = Addr(gpa(c, -0x480)).lw(c);
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    let not_running = c.g[V0][0] != c.g[T1][0];
    c.g[V0][0] = Addr(gpa(c, -0x848)).lw(c);
    if not_running {
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x78)); // the deadline
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84)); // game
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc)); // AI / race
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8)); // AI update counter
    c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
    let passed = c.g[V0][0] != 0;
    // Updates per second (`li v1, 60`): the game rate.
    c.g[V1][0] = deadline_updates_per_second(c);
    if passed {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V0][0] = 0x1;
    c.mult(0, 4, 8, 3);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.sw((c.g[A1][0] as u32).wrapping_add(0x78), c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Helpers.

/// The low 32 bits of a register (an object pointer or a 32-bit value).
#[inline]
pub(crate) fn low(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// The current stack pointer.
#[inline]
pub(crate) fn sp_2(c: &Cpu) -> u32 {
    c.g[SP][0] as u32
}

/// `gp + off` for a `gp`-relative global (the game's `gp` is in `c.g[GP]`).
#[inline]
pub(crate) fn gpo(c: &Cpu, off: i32) -> u32 {
    if off >= 0 {
        low(c, GP).wrapping_add(off as u32)
    } else {
        low(c, GP).wrapping_sub((-off) as u32)
    }
}

/// `lw off(reg)`.
#[inline]
pub(crate) fn lw_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lw(c)
}

/// `lh off(reg)` (sign-extended).
#[inline]
pub(crate) fn lh_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lh(c)
}

/// `sw off(reg)`.
#[inline]
pub(crate) fn sw_r(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = low(c, reg).wrapping_add(off);
    c.sw(a, v);
}

/// `void GameModeMan_Destroy()` (0x2380e8): delete the ten mode singletons and
/// the optional extra object.
pub fn game_mode_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.g[V0][0] = 0x53_0000;
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.g[S0][0] = sx(low(c, V0).wrapping_add(0x6668));
        c.sd(sp_2(c), c.g[RA][0]);
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
            c.lq(16, sp_2(c).wrapping_add(0x20));
            if !gone {
                c.cycles += 3;
                call_ptr(c, callee::BUILTIN_DELETE.addr, 0x0023_8148)?;
                c.lq(16, sp_2(c).wrapping_add(0x20));
            }
            c.cycles += 5;
            c.lq(17, sp_2(c).wrapping_add(0x10));
            c.g[RA][0] = Addr(sp_2(c)).ld(c);
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
        c.sd(sp_2(c), c.g[RA][0]);
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
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
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
        c.sd(sp_2(c), c.g[RA][0]);
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
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// Zero ten score/time table entries at `0x536708` and `0x536730`, then sort
/// both with `qsort` (element size 4, comparator `cmp`).
pub(crate) fn game_mode_man_sort(
    c: &mut Cpu,
    head: u32,
    ret_first: u32,
    ret_second: u32,
    cmp: u32,
) -> Flow {
    c.cycles += 10;
    open(c, 0x30);
    let sp0 = spv(c);
    c.g[A0][0] = 0x530000;
    c.sq(16, at(sp0, 0x20));
    c.g[V1][0] = 0;
    c.sq(17, at(sp0, 0x10));
    c.g[S0][0] = c.g[A1][0];
    c.g[S1][0] = 0x530000;
    c.sd(sp0, c.g[RA][0]);
    c.g[A2][0] = sx(at(c.g[A0][0] as u32, 0x6708));
    c.g[A1][0] = sx(at(c.g[S1][0] as u32, 0x6730));
    loop {
        c.cycles += 7;
        let a1 = c.g[A1][0] as u32;
        let a2 = c.g[A2][0] as u32;
        c.sw(a1, c.g[V1][0]);
        c.sw(a2, c.g[V1][0]);
        c.g[A1][0] = sx(at(a1, 4));
        c.g[V1][0] = sx(at(c.g[V1][0] as u32, 1));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 10i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx(at(a2, 4));
        if !more {
            c.cycles += 17;
            c.g[A3][0] = sx(cmp);
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0x6708));
            c.g[A2][0] = 4;
            call_ptr(c, callee::QSORT, ret_first)?;
            c.g[A3][0] = 0x245ec0;
            c.g[A0][0] = sx(at(c.g[S1][0] as u32, 0x6730));
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 4;
            call_ptr(c, callee::QSORT, ret_second)?;
            c.lq(16, at(sp0, 0x20));
            c.lq(17, at(sp0, 0x10));
            c.g[RA][0] = c.ld(sp0);
            let ra = c.g[RA][0] as u32;
            c.g[SP][0] = sx(at(sp0, 0x30));
            c.pc = ra;
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void GameModeMan_SortByScore(GameModeMan* self, void* cmp)` (0x238b70).
pub fn game_mode_man_sort_by_score(c: &mut Cpu) {
    guest(c, |c| {
        game_mode_man_sort(c, 0x0023_8b98, 0x0023_8bcc, 0x0023_8be4, 0x0024_5e30)
    })
}

/// `void GameModeMan_SortByTime(GameModeMan* self, void* cmp)` (0x238bf8).
pub fn game_mode_man_sort_by_time(c: &mut Cpu) {
    guest(c, |c| {
        game_mode_man_sort(c, 0x0023_8c20, 0x0023_8c54, 0x0023_8c6c, 0x0024_5e78)
    })
}

/// `GameRand_Shuffle` (0x23c770): Fisher–Yates shuffle of 25 words using the
/// game RNG (two draws per swap).
pub fn game_rand_shuffle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        let this = r(c, A0);
        c.g[S2][0] = sx(this);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = 0x18;
        frame.save_ra(c, 0);
        loop {
            c.cycles += 5;
            c.g[RA][0] = sx(0x0023_c7a0);
            c.g[S3][0] = sx(r(c, S3).wrapping_sub(1));
            callee::GAME_RAND_NEXT.call(c, 0x0023_c7a0)?;
            c.divu(0, 2, 17);
            c.cycles += 6;
            c.g[S0][0] = c.hi[0];
            c.g[RA][0] = sx(0x0023_c7b8);
            callee::GAME_RAND_NEXT.call(c, 0x0023_c7b8)?;
            c.divu(0, 2, 17);
            c.cycles += 10;
            c.g[S0][0] = shl32(c.g[S0][0], 2);
            c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, S2)));
            c.g[A0][0] = Addr(r(c, S0)).lw(c);
            c.g[V1][0] = c.hi[0];
            c.g[V1][0] = shl32(c.g[V1][0], 2);
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, S2)));
            c.g[V0][0] = Addr(r(c, V1)).lw(c);
            Addr(r(c, S0)).sw(c, c.g[V0][0]);
            let more = (c.g[S3][0] as i64) >= 0;
            Addr(r(c, V1)).sw(c, c.g[A0][0]);
            if !more {
                c.cycles += 7;
                frame.restore(c, S0, 0x40);
                frame.restore(c, S1, 0x30);
                frame.restore(c, S2, 0x20);
                frame.restore(c, S3, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_c798);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// `GMHandler_Start` (0x238c80): reset the game-mode singleton's fields, clear
// its two 0x28-byte arrays and dispatch its entry vtable function.
pub fn gm_handler_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[S0][0] = c.g[A0][0];
        let gm = c.g[V0][0] as u32;
        c.sw(gm.wrapping_add(0x84), c.g[A1][0]);
        c.g[A0][0] = sx(gm.wrapping_add(0x18));
        c.sw(gm.wrapping_add(0x8), c.g[V1][0]);
        c.sw(gm.wrapping_add(0x4), c.g[V1][0]);
        c.g[A1][0] = 0u64;
        c.sw(gm.wrapping_add(0x68), c.g[V1][0]);
        c.g[A2][0] = 0x28;
        c.sw(gm.wrapping_add(0x6c), c.g[V1][0]);
        c.sw(gm.wrapping_add(0x98), 0u64);
        c.sw(gm, 0u64);
        c.sw(gm.wrapping_add(0x70), 0u64);
        c.sw(gm.wrapping_add(0x74), 0u64);
        c.sw(gm.wrapping_add(0x10), 0u64);
        c.sw(gm.wrapping_add(0x14), 0u64);
        c.sw(gm.wrapping_add(0x78), 0u64);
        c.sw(gm.wrapping_add(0x7c), 0u64);
        c.sw(gm.wrapping_add(0x80), 0u64);
        c.sw(gm.wrapping_add(0x88), 0u64);
        c.sw(gm.wrapping_add(0x8c), 0u64);
        c.sw(gm.wrapping_add(0x90), 0u64);
        c.sw(gm.wrapping_add(0x94), 0u64);
        call_ptr(c, 0x00416210, 0x0023_8cf4)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x28;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x00416210, 0x0023_8d08)?;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_8d1c)?;
        c.lq(16, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int GMHandler7_v06(self)` (0x23c560): the "7" GM handler: dispatch an
/// input event to the active game (or its HUD) for the 17..21 range.
pub fn gm_handler7_v06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        call_ptr(c, 0x0014_dc80, 0x0023_c578)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_c584)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0014_4be0, 0x0023_c590)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 17) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        c.g[A0][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = ((c.g[V1][0] as i64) < 22) as u64;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x530000;
        if c.g[A0][0] != 0 {
            c.cycles += 14;
            c.g[A2][0] = Addr(p(c.g[S0][0] as u32, 0xa)).lh(c);
            c.g[A1][0] = Addr(p(c.g[V0][0] as u32, 0x5c12)).lb(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A3][0] = Addr(p(c.g[S0][0] as u32, 8)).lhu(c);
            call_ptr(c, 0x0014_5668, 0x0023_c5c4)?;
            c.g[A0][0] = Addr(p(r32(c, GP), 0xffff_f7b8)).lw(c);
            c.g[V1][0] = 0x64;
            c.mult(0, 6, 2, 3);
            c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x84)).lw(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0xc)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x28)).lw(c);
            c.g[A0][0] = Addr(p(c.g[A0][0] as u32, 0x790)).lw(c);
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = Addr(p(c.g[A0][0] as u32, 0x198)).lw(c);
            if !none {
                c.cycles += 2;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                call_ptr(c, 0x0011_95d8, 0x0023_c5f0)?;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 8)).lhu(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sh(p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 0x1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GMHandler4_v06(this)` (0x23b5f8): when the special event is in the
/// 17..21 range, show the HUD message and count the seconds.
pub fn gm_handler4_v06(c: &mut Cpu) {
    'b_23b640: {
        c.cycles += 17;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0023b610);
        c.g[S0][0] = c.g[A0][0];
        if call_ptr(c, 0x0014dc80, 0x0023b610).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023b61c);
        c.g[A1][0] = 0u64;
        if call_ptr(c, 0x0014dd58, 0x0023b61c).is_err() {
            return;
        }
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023b628);
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, 0x00144be0, 0x0023b628).is_err() {
            return;
        }
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 17i64) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = 0u64;
        if t {
            break 'b_23b640;
        } else {
            c.cycles += 1;
            c.g[A0][0] = ((c.g[V1][0] as i64) < 22i64) as u64;
            break 'b_23b640;
        }
    }
    'b_23b6a4: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0x530000;
        if t {
            break 'b_23b6a4;
        } else {
            'b_23b698: {
                c.cycles += 17;
                c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xa));
                c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c12));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0023b65c);
                c.g[A3][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x8));
                if call_ptr(c, 0x00145668, 0x0023b65c).is_err() {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = c.g[V0][0];
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x514c));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.cvt_s_w(0, 0);
                c.mul_s(0, 0, 1);
                c.cvt_w_s(1, 0);
                c.mfc1(5, 1);
                let t = c.g[A0][0] == 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                if t {
                    break 'b_23b698;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[RA][0] = sx(0x0023b698);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790));
                    if call_ptr(c, 0x001195a8, 0x0023b698).is_err() {
                        return;
                    }
                    break 'b_23b698;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sh((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            break 'b_23b6a4;
        }
    }
    c.cycles += 6;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V0][0] = 0x1;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t;
    return;
}

/// `void GMHandler9_Reset(GMHandler9* self)` (0x239d88): clear the race
/// manager's player/handler counters and re-seat the two player interfaces.
pub fn gm_handler9_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 80;
        let f = Frame::open(c, 0x40);
        f.save(c, S2, 0x10);
        f.save(c, S0, 0x30);
        f.save(c, S1, 0x20);
        f.save_ra(c, 0);
        c.g[RA][0] = sx(0x0023_9da4);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_9da4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023_9db0);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_9db0)?;
        c.g[RA][0] = sx(0x0023_9db8);
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_9db8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023_9dc4);
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0023_9dc4)?;
        // The handler's state block.
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        c.g[T0][0] = u64::MAX;
        c.g[A2][0] = 3;
        c.g[A3][0] = 1;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x6c), c.g[T0][0]);
        c.g[S0][0] = c.g[V0][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x68), c.g[T0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x88), c.g[A3][0]);
        c.g[A1][0] = 3;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x74), c.g[A2][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x98), c.g[A3][0]);
        c.sw(c.g[V1][0] as u32, c.g[A2][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x70), c.g[A2][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x80), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x84), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x94), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x90), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x9c), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), 0);
        c.g[RA][0] = sx(0x0023_9e1c);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x14), 0);
        call_ptr(c, 0x0014_54f8, 0x0023_9e1c)?;
        // The deadline (+0x78): the mode time in seconds, in game updates
        // (`li v1, 60`: the game rate).
        c.g[V1][0] = deadline_updates_per_second(c);
        c.g[A3][0] = Addr(gp(c, -0x480)).lw(c);
        c.mult(0, V0, V0, V1);
        c.g[A2][0] = 0x28;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x18));
        c.sw((c.g[A3][0] as u32).wrapping_add(0x7c), 0);
        c.sw((c.g[A3][0] as u32).wrapping_add(0x8c), 0);
        c.g[A1][0] = u64::MAX;
        c.g[RA][0] = sx(0x0023_9e44);
        c.sw((c.g[A3][0] as u32).wrapping_add(0x78), c.g[V0][0]);
        call_ptr(c, 0x0041_6210, 0x0023_9e44)?;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
        let func = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0023_9e58);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, 0x0023_9e58)?;
        // The two player-ID slots (a1 = 0, then 1) and their names.
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0023_9e64);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_7488, 0x0023_9e64)?;
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[RA][0] = sx(0x0023_9e78);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        call_ptr(c, 0x0014_7488, 0x0023_9e78)?;
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0023_9e8c);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        call_ptr(c, 0x0014_74c8, 0x0023_9e8c)?;
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[RA][0] = sx(0x0023_9ea0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        call_ptr(c, 0x0014_74c8, 0x0023_9ea0)?;
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        f.restore(c, S0, 0x30);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
        c.sh((c.g[S2][0] as u32).wrapping_add(6), 0);
        c.sh((c.g[S2][0] as u32).wrapping_add(4), 0);
        f.restore(c, S1, 0x20);
        f.restore(c, S2, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `void func_0023AE00(sSelf* self, int idx)` (0x23ae00): rider `idx` finished
/// the race: store the finish time (or 360000 for a DNF), then when every
/// player is finished also fill in the estimate for the others, sort the
/// results by time and mark the handler done.
pub fn gm_handler8_on_rider_finished(c: &mut Cpu) {
    let frame = Frame::open(c, 0xb0);
    'b_23af0c: {
        'b_23ae80: {
            c.cycles += 21;
            c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
            frame.save(c, S7, 0x20);
            sq(c, 30, at_2(c, SP, 0x10));
            c.g[S7][0] = c.g[A0][0];
            frame.save(c, S0, 0x90);
            c.g[FP][0] = c.g[A1][0];
            frame.save(c, S1, 0x80);
            frame.save(c, S2, 0x70);
            frame.save(c, S3, 0x60);
            frame.save(c, S4, 0x50);
            frame.save(c, S5, 0x40);
            frame.save(c, S6, 0x30);
            sd(c, r32(c, SP), c.g[RA][0]);
            c.swc1(20, at_2(c, SP, 0xa0));
            c.g[V0][0] = lw(c, at_2(c, V1, 0x84));
            c.g[S0][0] = lw(c, at_2(c, V0, 0xc));
            c.g[RA][0] = sx(0x0023ae4c);
            c.g[A0][0] = c.g[S0][0];
            if call_ptr(c, 0x0012a250, 0x0023ae4c).is_err() {
                return;
            }
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = sx(r32(c, FP) << 2);
            if t {
                break 'b_23ae80;
            } else {
                c.cycles += 11;
                c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                c.g[S4][0] = c.g[V0][0];
                c.g[A0][0] = lw(c, at_2(c, V1, 0x28));
                c.g[A1][0] = lw(c, at_2(c, A0, 0x6c0));
                c.g[A0][0] = sx(at_2(c, A0, 0x6c0));
                c.g[V1][0] = lh(c, at_2(c, A1, 0x48));
                c.g[V0][0] = lw(c, at_2(c, A1, 0x4c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0023ae78);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                if call_ptr(c, t, 0x0023ae78).is_err() {
                    return;
                }
                let t = c.g[V0][0] != 0u64;

                if t {
                    break 'b_23af0c;
                } else {
                    break 'b_23ae80;
                }
            }
        }
        'b_23aedc: {
            c.cycles += 10;
            c.g[A1][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
            c.g[A0][0] = sx(r32(c, FP) << 2);
            c.g[S4][0] = c.g[A0][0];
            c.g[V1][0] = lw(c, at_2(c, A1, 0x84));
            c.g[V0][0] = lw(c, at_2(c, V1, 0xc));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = lw(c, at_2(c, V0, 0x28));
            c.g[A0][0] = lw(c, at_2(c, V1, 0x480));
            let t = c.g[A0][0] == 0u64;
            c.g[V1][0] = 0x530000;
            if t {
                break 'b_23aedc;
            } else {
                c.cycles += 13;
                c.g[A0][0] = 0x50000;
                c.g[A0][0] = c.g[A0][0] | 0x7e40;
                c.g[A1][0] = sx(r32(c, S7).wrapping_add(r32(c, S4)));
                c.g[V0][0] = 0x530000;
                c.g[V1][0] = sx(at_2(c, V1, 0x6640));
                sw(c, at_2(c, A1, 0x4), c.g[A0][0]);
                c.g[V0][0] = sx(at_2(c, V0, 0x66a8));
                c.g[V1][0] = sx(r32(c, S4).wrapping_add(r32(c, V1)));
                c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                c.g[A1][0] = 0x1;
                sw(c, r32(c, V1), c.g[A0][0]);
                let t = true;
                sw(c, r32(c, V0), c.g[A1][0]);
                if t {
                    break 'b_23af0c;
                } else {
                    break 'b_23aedc;
                }
            }
        }
        c.cycles += 12;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, S4)));
        c.g[A2][0] = lw(c, at_2(c, A0, 0x28));
        c.g[V0][0] = 0x530000;
        c.g[V1][0] = sx(at_2(c, V1, 0x6640));
        c.g[A0][0] = sx(r32(c, S7).wrapping_add(r32(c, S4)));
        c.g[A1][0] = lw(c, at_2(c, A2, 0x478));
        c.g[V0][0] = sx(at_2(c, V0, 0x66a8));
        c.g[V1][0] = sx(r32(c, S4).wrapping_add(r32(c, V1)));
        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        sw(c, at_2(c, A0, 0x4), c.g[A1][0]);
        sw(c, r32(c, V1), c.g[A1][0]);
        sw(c, r32(c, V0), 0u64);
        break 'b_23af0c;
    }
    'b_23b000: {
        c.cycles += 4;
        c.g[RA][0] = sx(0x0023af14);
        c.g[A0][0] = c.g[S0][0];
        if call_ptr(c, 0x0012a250, 0x0023af14).is_err() {
            return;
        }
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
        if t {
            break 'b_23b000;
        } else {
            'b_23afb4: {
                c.cycles += 4;
                c.g[V0][0] = lw(c, at_2(c, V1, 0x10));
                c.g[V0][0] = sx(at_2(c, V0, 0x2));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S1][0] = 0u64;
                if t {
                    break 'b_23afb4;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = 0x530000;
                    c.mtc1(0, 20);
                    c.g[S6][0] = sx(at_2(c, V0, 0x6640));
                    c.g[S3][0] = sx(at_2(c, S0, 0x28));
                    c.g[S5][0] = 0u64;
                    c.g[S2][0] = sx(at_2(c, S7, 0x4));
                    'l_23af48: loop {
                        'b_23af64: {
                            c.cycles += 6;
                            c.g[A0][0] = lw(c, r32(c, S3));
                            c.lwc1(0, at_2(c, A0, 0x470));
                            c.c_le_s(20, 0);
                            let t = c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_23af64;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = 0u64;
                                break 'b_23af64;
                            }
                        }
                        'b_23af84: {
                            'b_23af80: {
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S0][0] = sx(at_2(c, S7, 0x4));
                                if t {
                                    break 'b_23af80;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == c.g[FP][0];
                                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, S5)));
                                    if t {
                                        break 'b_23af84;
                                    } else {
                                        c.cycles += 3;
                                        c.g[RA][0] = sx(0x0023af7c);
                                        if call_ptr(c, 0x00122d78, 0x0023af7c).is_err() {
                                            return;
                                        }
                                        sw(c, r32(c, S2), c.g[V0][0]);
                                        break 'b_23af80;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, S5)));
                            break 'b_23af84;
                        }
                        c.cycles += 12;
                        c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
                        c.g[V1][0] = lw(c, r32(c, V0));
                        c.g[S1][0] = sx(at_2(c, S1, 0x1));
                        c.g[S5][0] = sx(at_2(c, S5, 0x4));
                        c.g[S2][0] = sx(at_2(c, S2, 0x4));
                        sw(c, r32(c, S6), c.g[V1][0]);
                        c.g[S3][0] = sx(at_2(c, S3, 0x4));
                        c.g[V0][0] = lw(c, at_2(c, A0, 0x10));
                        c.g[V0][0] = sx(at_2(c, V0, 0x2));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S6][0] = sx(at_2(c, S6, 0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0023af48);
                            }
                            continue 'l_23af48;
                        } else {
                            break 'b_23afb4;
                        }
                    }
                }
            }
            'b_23aff8: {
                c.cycles += 10;
                c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
                c.g[RA][0] = sx(0x0023afc0);
                c.g[A1][0] = 0xa;
                if callee::C00238BF8.call(c, 0x0023afc0).is_err() {
                    return;
                }
                c.g[V0][0] = 0x530000;
                c.g[V0][0] = sx(at_2(c, V0, 0x6730));
                c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                c.g[V1][0] = lw(c, r32(c, V0));
                c.g[V1][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                let t = c.g[V1][0] != 0u64;
                c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
                if t {
                    break 'b_23aff8;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                    c.g[A0][0] = lw(c, at_2(c, V1, 0x84));
                    c.g[V0][0] = lw(c, at_2(c, A0, 0xc));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
                    c.g[V1][0] = lw(c, at_2(c, V0, 0x28));
                    sw(c, at_2(c, V1, 0x100), 0u64);
                    c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
                    break 'b_23aff8;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            sw(c, at_2(c, V1, 0x9c), c.g[V0][0]);
            break 'b_23b000;
        }
    }
    c.cycles += 13;
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    lq(c, 30, at_2(c, SP, 0x10));
    c.g[RA][0] = ld(c, r32(c, SP));
    c.lwc1(20, at_2(c, SP, 0xa0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_2(c, SP, 0xb0));
    c.pc = t;
    return;
}

/// `D_004A2C70`: the current GM handler object (`lw(gp - 0x480)`).
pub(crate) const GM_HANDLER: u32 = 0x480;

pub(crate) const D_00535C04: u32 = 0x0053_5c04;

/// `lw(gp - off)`.
#[inline(always)]
pub(crate) fn gpl(c: &mut Cpu, off: u32) -> u64 {
    c.lw((c.g[GP][0] as u32).wrapping_sub(off))
}

/// `void GMHandler8_Reset(void *self)` (0x23ac88): reset the GM handler's
/// screen fields, clear the 5*8-byte table at `self+4`, run its vtable's
/// slot 5 and copy the player's character ids and alt-character ids into the
/// handler; pick opponents when the mode word was 1.
pub fn gmhandler8_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 16, 0x30);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 1;

        // player = cBE_getInterface(cBE_getBE(), 1)
        callee::CBE_GET_BE.call(c, 0x0023_aca8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        callee::CBE_GET_INTERFACE.call(c, 0x0023_acb4)?;
        c.g[S0][0] = c.g[V0][0];

        let gm = gpl(c, GM_HANDLER);
        let this = c.g[S1][0] as u32;
        c.g[V1][0] = 4;
        c.g[V0][0] = gm;
        c.g[A0][0] = 3;
        c.g[A2][0] = 2;
        c.g[A3][0] = u64::MAX;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[A1][0] = 7;
        c.g[V1][0] = gpl(c, 0x848);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[A3][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x94), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x90), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x98), c.g[S2][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x88), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[A0][0]);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
        callee::CBE_GET_INTERFACE.call(c, 0x0023_ad0c)?;

        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
        let profile_missing = c.g[V0][0] == 0;
        c.g[V0][0] = gpl(c, GM_HANDLER);
        if !profile_missing {
            c.cycles += 10;
            callee::CBE_GET_BE.call(c, 0x0023_ad24)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            callee::CBE_GET_INTERFACE.call(c, 0x0023_ad30)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[A0][0] = gpl(c, GM_HANDLER);
            c.g[V0][0] = c.lw(D_00535C04);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[V0][0] = gpl(c, GM_HANDLER);
        }

        c.cycles += 36;
        c.g[A2][0] = 0x28;
        c.g[A1][0] = 9;
        c.g[A0][0] = sx(this.wrapping_add(4));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x80), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x78), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), 0);
        callee::MEMSET_2.call(c, 0x0023_ad68)?;

        // self's vtable slot 5: {int16 delta; fn}(self + delta)
        c.g[V1][0] = c.lw(this);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let f = c.g[V0][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        crate::call::call_ptr(c, f, 0x0023_ad7c)?;

        let player = c.g[S0][0];
        for (alt, ret, slot) in [(0u64, 0x0023_ad88u32, 0x18u32), (1, 0x0023_ad9c, 0x1c)] {
            if alt == 0 {
                c.g[A0][0] = player;
                c.g[A1][0] = 0;
                callee::GET_CHAR_ID.call(c, ret)?;
            } else {
                c.g[A0][0] = player;
                c.g[A1][0] = 1;
                callee::GET_CHAR_ID.call(c, ret)?;
            }
            c.g[V1][0] = gpl(c, GM_HANDLER);
            c.sw((c.g[V1][0] as u32).wrapping_add(slot), c.g[V0][0]);
        }
        for (slot, ret) in [(0x40u32, 0x0023_adb0u32), (0x44, 0x0023_adc4)] {
            let alt = if slot == 0x40 { 0 } else { 1 };
            c.g[A0][0] = player;
            c.g[A1][0] = alt;
            callee::GET_ALT_CHAR.call(c, ret)?;
            c.g[V1][0] = gpl(c, GM_HANDLER);
            c.sw((c.g[V1][0] as u32).wrapping_add(slot), c.g[V0][0]);
        }
        c.g[V1][0] = gpl(c, GM_HANDLER);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        let pick = c.g[A0][0] != c.g[S2][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
        if !pick {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0023_addc);
            c.pc = 0x0023_b0a8;
            ee::s06::GMHandler8_PickOpponents(c);
            if c.pc != 0x0023_addc {
                return Err(crate::call::Unwound);
            }
        }

        c.cycles += 8;
        c.g[V0][0] = gpl(c, GM_HANDLER);
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Game-mode resets and deadlines (tick rate).
//
// The game-mode manager (`gp-0x480`) keeps a deadline at +0x78 that the
// handlers' `v06` callbacks compare with the AI update counter (world ->
// +0x84 -> +0xc -> +0x8, one step per game update): the resets set it to
// `seconds * 60` and `GMHandler0_v06` / `GMHandler9_v06` extend it by
// `seconds * 60`. The 60 is updates per second: at another game rate it is
// [`crate::tick::rate`] (60 at 60), so the deadline is the same wall time.

/// `cBE_getBE()`.
const BE_GET: u32 = 0x0014_dc80;
/// `cBE_getInterface(be, n)`: 0 race, 1 player, 8 score.
const BE_IFACE: u32 = 0x0014_dd58;
/// `memset(p, c, n)`.
const MEMSET: u32 = 0x0041_6210;
/// The race interface's mode time (seconds) for a mode/level.
const RACE_MODE_SECONDS: u32 = 0x0014_54f8;
/// The race interface's medal time (seconds) for a mode and medal.
const RACE_MEDAL_SECONDS: u32 = 0x0014_55d0;
/// The race interface's time limit (seconds) for a mode and medal.
const RACE_LIMIT_SECONDS: u32 = 0x0014_5600;
/// The score interface's best medal for a level and mode.
const SCORE_BEST_MEDAL: u32 = 0x0015_58f8;
/// The player interface's slot / character / name helpers.
const PLAYER_SLOT: u32 = 0x0014_6e98;
const PLAYER_CHAR_ID: u32 = 0x0014_7398;
const PLAYER_NAME: u32 = 0x0014_7410;
/// The race interface's opponent character.
const RACE_OPPONENT: u32 = 0x0014_5750;
/// The quick-race / multiplayer flag and the event mode (bytes).
const QUICK_RACE: u32 = 0x0053_5c11;
const EVENT_MODE: u32 = 0x0053_5c12;
/// The standings scratch table (`D_00536640`) and the sorted order
/// (`D_00536708`).
const STANDINGS: u32 = 0x0053_6640;
const ORDER: u32 = 0x0053_6708;

const PICK_OPPONENTS: Callee = Callee {
    addr: 0x0023_9938,
    f: ee::s06::GMHandler0_PickOpponents,
};
const INIT_STANDINGS: Callee = Callee {
    addr: 0x0023_9aa0,
    f: ee::s06::GMHandler0_InitStandings,
};
const SORT_BY_SCORE: Callee = Callee {
    addr: 0x0023_8b70,
    f: ee::s06::GameModeMan_SortByScore,
};

/// The game-mode manager (`lw gp-0x480`).
#[inline]
fn gm_man(c: &mut Cpu) -> u64 {
    Addr(gpo(c, -0x480)).lw(c)
}

/// Updates per second of the deadline arithmetic (`li 60`): the game rate.
#[inline]
fn deadline_updates_per_second(c: &mut Cpu) -> u64 {
    crate::tick::rate(c) as u64
}

/// `int GMHandler0_v06(GMHandler0* self, int player, int seconds)`
/// (0x2398e8): while the race runs (manager +0x8 == 1), extend the deadline
/// (manager +0x78) by `seconds` and return 1, or 0 once the AI update counter
/// has passed it. The deadline is in AI updates: `seconds * rate`.
pub fn gm_handler0_v06(c: &mut Cpu) {
    c.cycles += 5;
    let gm = gm_man(c);
    c.g[A3][0] = gm;
    c.g[V0][0] = 1;
    c.g[T0][0] = Addr(gm as u32).off(0x8).lw(c);
    let running = c.g[T0][0] == c.g[V0][0];
    c.g[V1][0] = c.g[A2][0];
    if running {
        c.cycles += 12;
        c.g[A2][0] = Addr(gpo(c, -0x848)).lw(c);
        c.g[A0][0] = deadline_updates_per_second(c);
        c.mult(0, A0, V1, A0);
        c.g[V0][0] = Addr(r32(c, A2)).off(0x84).lw(c);
        c.g[V1][0] = Addr(gm as u32).off(0x78).lw(c);
        c.g[A1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        // The AI update counter.
        c.g[V0][0] = Addr(r32(c, A1)).off(0x8).lw(c);
        Addr(gm as u32).off(0x78).sw(c, c.g[V1][0]);
        c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        c.g[V0][0] = c.g[T0][0];
        if c.g[V1][0] != 0 {
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void GMHandler0_Reset(GMHandler0* self)` (0x238e20): in a quick race,
/// reset the opponent count, the times table and the standings; set the
/// deadline (manager +0x78) to the mode's time; then start round
/// `manager->+0x70` (1, 2 or 3) with its standings sorted by score.
pub fn gm_handler0_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let f = Frame::open(c, 0x40);
        f.save(c, S0, 0x30);
        f.save(c, S1, 0x20);
        f.save(c, S2, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, BE_GET, 0x0023_8e3c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, BE_IFACE, 0x0023_8e48)?;
        c.g[S1][0] = c.g[V0][0]; // the race interface
        let gm = gm_man(c);
        c.g[A0][0] = gm;
        c.g[V0][0] = 0x53_0000;
        c.g[V1][0] = Addr(QUICK_RACE).lb(c);
        Addr(gm as u32).off(0x8c).sw(c, 0);
        Addr(gm as u32).off(0x7c).sw(c, 0);
        if c.g[V1][0] != 0 {
            // A quick race: five opponents, the standings start over.
            c.cycles += 5;
            c.g[A1][0] = 5;
            c.g[V1][0] = Addr(gm as u32).off(0x8).lw(c);
            c.g[V0][0] = 1;
            Addr(gm as u32).off(0x10).sw(c, 5);
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 1;
                Addr(gm as u32).off(0x14).sw(c, c.g[A1][0]);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 4;
                Addr(gm as u32).off(0x14).sw(c, 4);
            }
            c.cycles += 7;
            let gm = gm_man(c);
            c.g[A0][0] = gm;
            c.g[V0][0] = u64::MAX;
            c.g[V1][0] = 1;
            c.g[A1][0] = Addr(gm as u32).off(0x8).lw(c);
            Addr(gm as u32).off(0x6c).sw(c, u64::MAX);
            Addr(gm as u32).off(0x90).sw(c, 0);
            if c.g[A1][0] != c.g[V1][0] {
                c.cycles += 1;
                Addr(gm as u32).off(0x94).sw(c, 0);
            } else {
                c.cycles += 2;
                Addr(gm as u32).off(0x94).sw(c, c.g[A1][0]);
            }
            c.cycles += 14;
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(4));
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x78;
            call_ptr(c, MEMSET, 0x0023_8ec0)?;
            reset_vcall(c, 0x0023_8ed4)?;
            let gm = gm_man(c);
            c.g[V1][0] = gm;
            c.g[A0][0] = 1;
            c.g[V0][0] = Addr(gm as u32).off(0x84).lw(c);
            let pick = c.g[V0][0] == c.g[A0][0];
            c.g[V0][0] = 3;
            if pick {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                PICK_OPPONENTS.call(c, 0x0023_8ef0)?;
                c.g[V1][0] = gm_man(c);
                c.g[V0][0] = 3;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            Addr(r32(c, V1)).off(0x74).sw(c, c.g[V0][0]);
            Addr(r32(c, V1)).off(0x70).sw(c, c.g[V0][0]);
            INIT_STANDINGS.call(c, 0x0023_8f08)?;
        }
        // The deadline: the mode's time in seconds, in game updates.
        c.cycles += 11;
        let gm = gm_man(c);
        c.g[V0][0] = gm;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = Addr(gm as u32).lw(c);
        call_ptr(c, RACE_MODE_SECONDS, 0x0023_8f18)?;
        c.g[V1][0] = deadline_updates_per_second(c);
        let gm = gm_man(c);
        c.g[A1][0] = gm;
        c.mult(0, V0, V0, V1);
        c.g[A0][0] = 2;
        c.g[V1][0] = Addr(gm as u32).off(0x8).lw(c);
        Addr(gm as u32).off(0x78).sw(c, c.g[V0][0]);
        c.cycles += 2;
        if c.g[V1][0] != c.g[A0][0] {
            c.g[V0][0] = 1;
            Addr(gm as u32).off(0x94).sw(c, 1);
        } else {
            Addr(gm as u32).off(0x94).sw(c, 0);
        }
        // The round to start.
        c.cycles += 5;
        let gm = gm_man(c);
        let g = Addr(gm as u32);
        c.g[A1][0] = gm;
        c.g[V0][0] = 2;
        c.g[V1][0] = g.off(0x70).lw(c);
        g.sw(c, c.g[V1][0]);
        let round = c.g[V1][0];
        if round == 2 {
            gm0_round2(c, g)?;
            f.restore(c, S0, 0x30);
        } else {
            c.cycles += 3;
            let first = (round as i64) < 3;
            c.g[V0][0] = 3;
            if first {
                c.cycles += 2;
                c.g[V1][0] = 1;
                if (round as i64) >= 0 {
                    gm0_round1(c, g)?;
                }
                c.cycles += 1;
                f.restore(c, S0, 0x30);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                if round == 3 {
                    gm0_round3(c, g)?;
                    c.cycles += 1;
                    f.restore(c, S0, 0x30);
                } else {
                    c.cycles += 2;
                    f.restore(c, S0, 0x30);
                }
            }
        }
        c.cycles += 5;
        f.restore(c, S1, 0x20);
        f.restore(c, S2, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `self->vtable[4](self)` (the slot at +0x20/+0x24), returning to `ret`.
fn reset_vcall(c: &mut Cpu, ret: u32) -> Flow {
    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
    c.g[A0][0] = Addr(r32(c, V1)).off(0x20).lh(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(0x24).lw(c);
    let func = c.g[V0][0] as u32;
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, func, ret)
}

/// The standings copy loop of [`gm_handler0_reset`]: `STANDINGS[i] =
/// *src` for `i = 1..=count` (`count` = manager +0x10 read through `gm_reg`
/// each pass), `src` in `src_reg` and `dst` in `dst_reg` both stepping by 4;
/// the loop counter is t1 (starting at 1). `head` is the loop's address.
fn gm0_copy_loop(c: &mut Cpu, src_reg: usize, dst_reg: usize, gm_reg: usize, head: u32) {
    loop {
        c.cycles += 9;
        c.g[V0][0] = Addr(r32(c, src_reg)).lw(c);
        c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
        c.g[src_reg][0] = sx(r32(c, src_reg).wrapping_add(4));
        Addr(r32(c, dst_reg)).sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr(r32(c, gm_reg)).off(0x10).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[dst_reg][0] = sx(r32(c, dst_reg).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `t1 = 1; v0 = (1 < manager->count + 1)` (the copy loops' entry test,
/// manager in `gm_reg`): whether there is anything to copy.
fn gm0_count_test(c: &mut Cpu, gm_reg: usize) -> bool {
    c.g[T1][0] = 1;
    c.g[V0][0] = Addr(r32(c, gm_reg)).off(0x10).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[V0][0] = (1i64 < (c.g[V0][0] as i64)) as u64;
    c.g[V0][0] != 0
}

/// `SortByScore(manager, manager->count + 1)`, returning to `ret`.
fn gm0_sort(c: &mut Cpu, ret: u32) -> Flow {
    let gm = gm_man(c);
    c.g[A0][0] = gm;
    c.g[A1][0] = Addr(gm as u32).off(0x10).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    SORT_BY_SCORE.call(c, ret)
}

/// Round 0/1 of [`gm_handler0_reset`] (0x238f9c..0x239118): a fresh event
/// (five opponents, no times), opponents picked again when asked (+0x84),
/// the two front riders from the standings of round 3, then the round-1
/// standings.
fn gm0_round1(c: &mut Cpu, g: Addr) -> Flow {
    c.cycles += 19;
    c.g[A0][0] = 5;
    c.g[V0][0] = u64::MAX;
    g.off(0x14).sw(c, 5);
    g.off(0x6c).sw(c, u64::MAX);
    g.sw(c, c.g[V1][0]);
    g.off(0x70).sw(c, c.g[V1][0]);
    g.off(0x74).sw(c, c.g[V1][0]);
    g.off(0x10).sw(c, 5);
    g.off(0x88).sw(c, c.g[V1][0]);
    g.off(0x90).sw(c, 0);
    g.off(0x9c).sw(c, 0);
    g.off(0x98).sw(c, 0);
    g.off(0x80).sw(c, 0);
    let this = Addr(r32(c, S0));
    this.off(0x4).sw(c, 0);
    this.off(0x2c).sw(c, 0);
    this.off(0x54).sw(c, 0);
    c.g[V0][0] = g.off(0x84).lw(c);
    let pick = c.g[V0][0] == c.g[V1][0];
    c.g[S2][0] = sx(r32(c, S0).wrapping_add(4));
    if pick {
        c.cycles += 13;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x78;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, MEMSET, 0x0023_8fd8)?;
        reset_vcall(c, 0x0023_8fec)?;
        c.g[A0][0] = c.g[S0][0];
        PICK_OPPONENTS.call(c, 0x0023_8ff4)?;
        c.g[A0][0] = c.g[S0][0];
        INIT_STANDINGS.call(c, 0x0023_8ffc)?;
    }
    // Round-3 times (self+0x58..) into the standings.
    c.cycles += 9;
    c.g[V1][0] = 0x53_0000;
    c.g[A1][0] = gm_man(c);
    Addr(STANDINGS).sw(c, 0);
    let any = gm0_count_test(c, A1);
    c.g[S1][0] = 0x53_0000;
    if any {
        c.cycles += 4;
        c.g[V0][0] = sx(STANDINGS);
        c.g[V1][0] = sx(r32(c, S0).wrapping_add(0x58));
        c.g[A0][0] = sx(STANDINGS + 4);
        gm0_copy_loop(c, V1, A0, A1, 0x0023_9030);
    }
    // Sort; the two front riders' characters into the race.
    c.cycles += 28;
    gm0_sort(c, 0x0023_9064)?;
    c.g[T2][0] = gm_man(c);
    c.g[T0][0] = sx(ORDER);
    c.g[A3][0] = Addr(gpo(c, -0x848)).lw(c);
    c.g[T1][0] = 1;
    c.g[V0][0] = Addr(ORDER).lw(c);
    c.g[A1][0] = sx(r32(c, T2).wrapping_add(0x18));
    c.g[A2][0] = Addr(r32(c, A3)).off(0x84).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
    c.g[V1][0] = Addr(r32(c, A2)).off(0xc).lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).lw(c);
    Addr(r32(c, V1)).off(0x20).sw(c, c.g[A0][0]);
    c.g[V0][0] = Addr(ORDER + 4).lw(c);
    c.g[A2][0] = Addr(r32(c, A3)).off(0x84).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
    c.g[A0][0] = Addr(r32(c, A2)).off(0xc).lw(c);
    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
    Addr(r32(c, A0)).off(0x24).sw(c, c.g[V1][0]);
    c.g[V0][0] = Addr(r32(c, T2)).off(0x10).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[V0][0] = (1i64 < (c.g[V0][0] as i64)) as u64;
    let any = c.g[V0][0] != 0;
    c.g[V0][0] = 0x53_0000;
    if any {
        // Round-1 times (self+0x8..) into the standings.
        c.cycles += 5;
        c.g[A1][0] = c.g[T2][0];
        c.g[V0][0] = sx(STANDINGS);
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(4));
        c.g[V1][0] = sx(STANDINGS + 4);
        gm0_copy_loop(c, A0, V1, A1, 0x0023_90d8);
    }
    c.cycles += 7;
    gm0_sort(c, 0x0023_910c)?;
    c.g[V0][0] = gm_man(c);
    Addr(r32(c, V0)).off(0x84).sw(c, 0);
    Ok(())
}

/// Round 2 of [`gm_handler0_reset`] (0x239118): the standings are the sums
/// of the round-1 and round-2 times.
fn gm0_round2(c: &mut Cpu, g: Addr) -> Flow {
    c.cycles += 13;
    c.g[V0][0] = 1;
    g.off(0x9c).sw(c, 0);
    g.off(0x74).sw(c, 1);
    c.g[V1][0] = 0x53_0000;
    g.off(0x70).sw(c, 1);
    c.g[T1][0] = 1;
    g.off(0x98).sw(c, 0);
    Addr(STANDINGS).sw(c, 0);
    c.g[V0][0] = g.off(0x10).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[V0][0] = (1i64 < (c.g[V0][0] as i64)) as u64;
    let any = c.g[V0][0] != 0;
    c.g[V0][0] = sx(STANDINGS);
    if any {
        c.cycles += 3;
        c.g[A2][0] = c.g[A1][0];
        c.g[A1][0] = sx(STANDINGS + 4);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(8));
        loop {
            c.cycles += 11;
            c.g[V0][0] = Addr(r32(c, A0)).off(0x28).lw(c);
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
            c.g[V1][0] = Addr(r32(c, A0)).lw(c);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            Addr(r32(c, A1)).sw(c, c.g[V0][0]);
            c.g[V0][0] = Addr(r32(c, A2)).off(0x10).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_9158);
            }
        }
    }
    c.cycles += 6;
    gm0_sort(c, 0x0023_9194)
}

/// Round 3 of [`gm_handler0_reset`] (0x23919c): the final, standings from
/// the round-3 times (self+0x58..). Entered with v1 = 3, v0 = 1.
fn gm0_round3(c: &mut Cpu, g: Addr) -> Flow {
    c.cycles += 14;
    g.off(0x74).sw(c, c.g[V1][0]);
    g.off(0x88).sw(c, c.g[V0][0]);
    c.g[A0][0] = 0x53_0000;
    g.off(0x98).sw(c, c.g[V0][0]);
    c.g[T1][0] = 1;
    g.off(0x70).sw(c, c.g[V1][0]);
    g.off(0x9c).sw(c, 0);
    g.off(0x84).sw(c, 0);
    Addr(STANDINGS).sw(c, 0);
    c.g[V0][0] = g.off(0x10).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[V0][0] = (1i64 < (c.g[V0][0] as i64)) as u64;
    let any = c.g[V0][0] != 0;
    c.g[V0][0] = sx(STANDINGS);
    if any {
        c.cycles += 3;
        c.g[V1][0] = sx(r32(c, S0).wrapping_add(0x58));
        c.g[A0][0] = sx(STANDINGS + 4);
        gm0_copy_loop(c, V1, A0, A1, 0x0023_91e0);
    }
    c.cycles += 4;
    gm0_sort(c, 0x0023_9214)
}

/// The score interface's best medal for the event (a 0..3 result) as the
/// handler's medal level at self+0xa (0x23b34c / 0x23c1a4): `level(result)`
/// gives the stored level, or `None` to leave 0. Charges like the original.
fn best_medal_level(c: &mut Cpu, ret: u32, two: fn(u64) -> bool) -> Flow {
    c.g[A1][0] = 0;
    c.g[A3][0] = Addr(r32(c, V0)).off(0x8).lw(c);
    c.g[A2][0] = Addr(r32(c, V0)).off(0x68).lw(c);
    call_ptr(c, SCORE_BEST_MEDAL, ret)?;
    c.g[V1][0] = c.g[V0][0];
    let level2 = two(c.g[V1][0]);
    c.g[V0][0] = level2 as u64;
    Addr(r32(c, S1)).off(0xa).sh(c, 0);
    let level = if level2 {
        c.cycles += 2;
        c.g[V0][0] = 2;
        Some(2)
    } else {
        c.cycles += 3;
        c.g[V0][0] = 3;
        let three = c.g[V1][0] == 3;
        c.g[V0][0] = gm_man(c);
        if three {
            c.cycles += 1;
            c.g[V0][0] = 1;
            Some(1)
        } else {
            None
        }
    };
    if level.is_some() {
        c.cycles += 2;
        Addr(r32(c, S1)).off(0xa).sh(c, c.g[V0][0]);
        c.g[V0][0] = gm_man(c);
    }
    Ok(())
}

/// The three `cBE_getInterface` calls of the timed-event resets: race in s0,
/// player in s2, score in v0 (returns at `rets`).
fn race_player_score(c: &mut Cpu, rets: [u32; 6]) -> Flow {
    call_ptr(c, BE_GET, rets[0])?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, BE_IFACE, rets[1])?;
    c.g[S0][0] = c.g[V0][0];
    call_ptr(c, BE_GET, rets[2])?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    call_ptr(c, BE_IFACE, rets[3])?;
    c.g[S2][0] = c.g[V0][0];
    call_ptr(c, BE_GET, rets[4])?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 8;
    call_ptr(c, BE_IFACE, rets[5])
}

/// The player's character into the manager's rider slot (`a18[slot] =
/// charID(player, slot)`), after clearing the slots to -1 (0x23b3e8 /
/// 0x23c248 ..): v0 = the character, s0 = the slot.
fn seat_player(c: &mut Cpu, rets: [u32; 4]) -> Flow {
    let gm = gm_man(c);
    c.g[A0][0] = gm;
    c.g[A2][0] = 0x28;
    c.g[A1][0] = u64::MAX;
    c.g[A0][0] = sx((gm as u32).wrapping_add(0x18));
    call_ptr(c, MEMSET, rets[0])?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 0;
    call_ptr(c, PLAYER_SLOT, rets[1])?;
    c.g[A0][0] = c.g[S2][0];
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, PLAYER_SLOT, rets[2])?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, PLAYER_CHAR_ID, rets[3])
}

/// `void GMHandler4_Reset(GMHandler4* self)` (0x23b268): a timed event
/// (event modes 6..8): the medal level from the best score, the event's
/// time limit `self->+0x4` = the medal time in seconds, as game updates (it
/// is compared with the rider's finish time, counted in updates), copied to
/// the deadline (+0x78) and +0x7c; the player seated alone.
pub fn gm_handler4_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let f = Frame::open(c, 0x40);
        f.save(c, S1, 0x20);
        f.save(c, S0, 0x30);
        f.save(c, S2, 0x10);
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        race_player_score(
            c,
            [
                0x0023_b284,
                0x0023_b290,
                0x0023_b298,
                0x0023_b2a4,
                0x0023_b2ac,
                0x0023_b2b8,
            ],
        )?;
        let gm = gm_man(c);
        let g = Addr(gm as u32);
        c.g[A1][0] = gm;
        c.g[A2][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        c.g[V1][0] = 0x53_0000;
        g.sw(c, 1);
        c.g[A0][0] = 7;
        c.g[V1][0] = Addr(EVENT_MODE).lb(c);
        g.off(0x10).sw(c, 0);
        g.off(0x8).sw(c, c.g[V1][0]);
        g.off(0x14).sw(c, 0);
        g.off(0x8c).sw(c, 0);
        let mode = c.g[V1][0];
        // The level/medal table entries for modes 6, 7 and 8.
        let entries = if mode == 7 {
            c.cycles += 3;
            Some((0xf, 3))
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((mode as i64) < 8) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 6;
                c.cycles += 3;
                c.g[V0][0] = 8;
                let eight = mode == 8;
                c.g[V0][0] = gm_man(c);
                if eight {
                    c.cycles += 2;
                    Some((0x10, 4))
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[A2][0];
                    None
                }
            } else {
                c.g[V0][0] = 6;
                c.cycles += 2;
                let six = mode == 6;
                c.g[V0][0] = gm_man(c);
                if six {
                    c.cycles += 4;
                    c.g[V0][0] = 0xe;
                    g.off(0x6c).sw(c, 0);
                    g.off(0x68).sw(c, 0xe);
                    c.cycles += 2;
                    c.g[V0][0] = gm_man(c);
                    c.g[A0][0] = c.g[A2][0];
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[A2][0];
                }
                None
            }
        };
        if let Some((level, medal)) = entries {
            c.g[V1][0] = level;
            c.g[V0][0] = medal;
            c.cycles += 2;
            g.off(0x6c).sw(c, medal);
            g.off(0x68).sw(c, level);
            c.cycles += 2;
            c.g[V0][0] = gm_man(c);
            c.g[A0][0] = c.g[A2][0];
        }
        c.cycles += 9;
        best_medal_level(c, 0x0023_b35c, |r| sx((r as u32).wrapping_sub(1)) < 2)?;
        // The time limit: the medal time in seconds, in game updates.
        c.cycles += 54;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = Addr(r32(c, S1)).off(0xa).lh(c);
        c.g[A1][0] = Addr(r32(c, V0)).off(0x8).lw(c);
        call_ptr(c, RACE_MEDAL_SECONDS, 0x0023_b3a0)?;
        c.g[A0][0] = deadline_updates_per_second(c);
        let gm = gm_man(c);
        let g = Addr(gm as u32);
        c.g[V1][0] = gm;
        c.mult(0, V0, V0, A0);
        c.g[A1][0] = 1;
        c.g[A2][0] = 2;
        let this = Addr(r32(c, S1));
        this.off(0x4).sw(c, c.g[V0][0]);
        g.off(0x88).sw(c, 1);
        c.g[V0][0] = this.off(0x4).lw(c);
        g.off(0x78).sw(c, c.g[V0][0]);
        c.g[A0][0] = this.off(0x4).lw(c);
        g.off(0x74).sw(c, 2);
        g.off(0x70).sw(c, 2);
        g.off(0x7c).sw(c, c.g[A0][0]);
        // self->vtable[5](self).
        c.g[V0][0] = this.lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x28).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x2c).lw(c);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x0023_b3e8)?;
        seat_player(c, [0x0023_b3fc, 0x0023_b408, 0x0023_b418, 0x0023_b424])?;
        let gm = gm_man(c);
        let g = Addr(gm as u32);
        c.g[V1][0] = gm;
        c.g[S0][0] = sx(r32(c, S0) << 2);
        f.restore(c, S2, 0x10);
        c.g[S0][0] = sx((gm as u32).wrapping_add(r32(c, S0)));
        f.restore_ra(c, 0);
        Addr(r32(c, S0)).off(0x18).sw(c, c.g[V0][0]);
        for off in [0x9c, 0x80, 0x84, 0x90, 0x94, 0x98] {
            g.off(off).sw(c, 0);
        }
        Addr(r32(c, S1)).off(0x8).sh(c, 0);
        f.restore(c, S0, 0x30);
        f.restore(c, S1, 0x20);
        f.close(c);
        Ok(())
    })
}

/// The per-player "out of time / DNF" flags (`D_005366A8`).
const OUT_FLAGS: u32 = 0x0053_66a8;
/// The per-player medal won (`D_00536730`): 0 gold, 1 silver, 2 bronze.
const MEDALS: u32 = 0x0053_6730;

/// `void GMHandler4_OnRiderFinished(GMHandler4* self, int idx)` (0x23b468):
/// a timed event's rider `idx` finished. Its finish time (rider +0x478, in
/// game updates) goes to the standings; a DNF (rider +0x480) stores 360000
/// and the out flag; over the handler's limit (`self->+0x4`, in updates:
/// see [`gm_handler4_reset`]) only the out flag; otherwise the medal from
/// the event's medal times (seconds, `* 60` updates per second: the game
/// rate).
pub fn gm_handler4_on_rider_finished(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let f = Frame::open(c, 0x70);
        f.save(c, S0, 0x60);
        f.save(c, S2, 0x40);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = sx(r32(c, A1) << 2);
        f.save(c, S3, 0x30);
        f.save(c, S4, 0x20);
        f.save(c, S5, 0x10);
        f.save(c, S1, 0x50);
        f.save_ra(c, 0);
        c.g[S4][0] = 1;
        call_ptr(c, BE_GET, 0x0023_b498)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, BE_IFACE, 0x0023_b4a4)?;
        // The rider: world -> +0x84 -> +0xc -> +0x28 + 4 * idx.
        c.g[A2][0] = Addr(gpo(c, -0x848)).lw(c);
        c.g[S5][0] = c.g[V0][0];
        c.g[A3][0] = gm_man(c);
        c.g[A0][0] = Addr(r32(c, A2)).off(0x84).lw(c);
        c.g[V1][0] = sx(STANDINGS);
        c.g[S3][0] = sx(r32(c, S2).wrapping_add(STANDINGS));
        c.g[V0][0] = Addr(r32(c, A0)).off(0xc).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
        c.g[V1][0] = Addr(r32(c, V0)).off(0x28).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x478).lw(c);
        Addr(r32(c, S3)).sw(c, c.g[A0][0]);
        Addr(r32(c, A3)).off(0x9c).sw(c, c.g[S4][0]);
        c.g[V1][0] = Addr(r32(c, A2)).off(0x84).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
        c.g[A1][0] = Addr(r32(c, V0)).off(0x28).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).off(0x480).lw(c);
        c.g[V0][0] = 0x5_0000;
        if c.g[V1][0] != 0 {
            // DNF: 360000 and out.
            c.cycles += 13;
            c.g[V0][0] = 0x5_7e40;
            Addr(r32(c, S3)).sw(c, c.g[V0][0]);
            c.g[A0][0] = sx(r32(c, S2).wrapping_add(OUT_FLAGS));
            c.g[V1][0] = Addr(r32(c, A2)).off(0x84).lw(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
            c.g[V1][0] = Addr(r32(c, V0)).off(0x28).lw(c);
            Addr(r32(c, V1)).off(0x100).sw(c, 0);
            Addr(r32(c, A3)).off(0x70).sw(c, c.g[S4][0]);
            Addr(r32(c, A0)).sw(c, c.g[S4][0]);
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, S0)).off(0x4).lw(c);
            c.g[V1][0] = Addr(r32(c, S3)).lw(c);
            let over = c.g[V0][0] < c.g[V1][0];
            c.g[V0][0] = 0x53_0000;
            if over {
                // Over the time limit: out.
                c.cycles += 4;
                Addr(r32(c, A1)).off(0x100).sw(c, 0);
                c.g[V0][0] = sx(OUT_FLAGS);
                Addr(r32(c, A3)).off(0x70).sw(c, c.g[S4][0]);
            } else {
                c.cycles += 18;
                Addr(r32(c, A1)).off(0x100).sw(c, c.g[S4][0]);
                c.g[A0][0] = c.g[S5][0];
                c.g[A2][0] = 2;
                Addr(r32(c, A3)).off(0x84).sw(c, c.g[S4][0]);
                c.g[A1][0] = Addr(r32(c, A3)).off(0x8).lw(c);
                // `li s1, 60`: updates per second (the medal times are in
                // seconds, the finish time in game updates).
                c.g[S1][0] = crate::tick::rate(c) as u64;
                call_ptr(c, RACE_MEDAL_SECONDS, 0x0023_b568)?;
                c.g[V1][0] = gm_man(c);
                c.mult(0, S0, V0, S1);
                c.g[A0][0] = c.g[S5][0];
                c.g[A2][0] = 1;
                c.g[A1][0] = Addr(r32(c, V1)).off(0x8).lw(c);
                call_ptr(c, RACE_MEDAL_SECONDS, 0x0023_b580)?;
                c.mult(0, V1, V0, S1);
                c.g[V0][0] = Addr(r32(c, S3)).lw(c);
                c.g[S0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                let past_gold = c.g[S0][0] != 0;
                c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                if !past_gold {
                    // Gold: 0.
                    c.cycles += 2;
                    c.g[V0][0] = sx(MEDALS);
                } else {
                    c.cycles += 2;
                    let past_silver = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x53_0000;
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, S2).wrapping_add(MEDALS));
                    if past_silver {
                        c.g[V1][0] = 2;
                        Addr(r32(c, V0)).sw(c, c.g[V1][0]);
                    } else {
                        Addr(r32(c, V0)).sw(c, c.g[S4][0]);
                    }
                    return gm4_finished_leave(c, f);
                }
            }
            // The flag (out) or the medal (gold) at `v0 + idx * 4`: 0.
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
            Addr(r32(c, V0)).sw(c, 0);
        }
        gm4_finished_leave(c, f)
    })
}

/// [`gm_handler4_on_rider_finished`]'s epilogue.
fn gm4_finished_leave(c: &mut Cpu, f: Frame) -> Flow {
    c.cycles += 9;
    f.restore(c, S0, 0x60);
    f.restore(c, S1, 0x50);
    f.restore(c, S2, 0x40);
    f.restore(c, S3, 0x30);
    f.restore(c, S4, 0x20);
    f.restore(c, S5, 0x10);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `void GMHandler6_Reset(GMHandler6* self)` (0x23bb98): one round; the
/// deadline (+0x78) is the mode time in seconds, in game updates; then the
/// riders: in a two-player race (`QUICK_RACE` == 2) both players with their
/// names, else the player and the race's opponent.
pub fn gm_handler6_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        let f = Frame::open(c, 0x60);
        f.save(c, S1, 0x40);
        f.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        f.save(c, S2, 0x30);
        f.save(c, S4, 0x10);
        f.save(c, S3, 0x20);
        f.save_ra(c, 0);
        c.g[S0][0] = 1;
        call_ptr(c, BE_GET, 0x0023_bbc0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, BE_IFACE, 0x0023_bbcc)?;
        c.g[S4][0] = c.g[V0][0]; // the player interface
        call_ptr(c, BE_GET, 0x0023_bbd4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, BE_IFACE, 0x0023_bbe0)?;
        c.g[S2][0] = c.g[V0][0]; // the race interface
        c.g[V1][0] = u64::MAX;
        let gm = gm_man(c);
        let g = Addr(gm as u32);
        c.g[V0][0] = gm;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 1;
        g.off(0x6c).sw(c, u64::MAX);
        g.off(0x68).sw(c, u64::MAX);
        g.off(0x74).sw(c, 1);
        g.sw(c, 1);
        g.off(0x10).sw(c, 1);
        g.off(0x14).sw(c, 0);
        g.off(0x70).sw(c, 1);
        call_ptr(c, RACE_MODE_SECONDS, 0x0023_bc14)?;
        // The deadline: the mode time in seconds, in game updates.
        c.g[A0][0] = deadline_updates_per_second(c);
        let gm = gm_man(c);
        let g = Addr(gm as u32);
        c.g[V1][0] = gm;
        c.mult(0, V0, V0, A0);
        g.off(0x98).sw(c, 1);
        g.off(0x9c).sw(c, 0);
        g.off(0x7c).sw(c, 0);
        g.off(0x78).sw(c, c.g[V0][0]);
        g.off(0x80).sw(c, 0);
        g.off(0x84).sw(c, 0);
        g.off(0x88).sw(c, 1);
        g.off(0x8c).sw(c, 0);
        g.off(0x90).sw(c, 0);
        g.off(0x94).sw(c, 0);
        // self->vtable[4](self).
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x20).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x24).lw(c);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x0023_bc5c)?;
        c.g[A0][0] = 0x53_0000;
        c.g[V1][0] = 2;
        c.g[V0][0] = Addr(QUICK_RACE).lb(c);
        let two_players = c.g[V0][0] == c.g[V1][0];
        c.g[A0][0] = gm_man(c);
        if two_players {
            c.cycles += 37;
            c.g[V0][0] = gm_man(c);
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0;
            Addr(r32(c, V0)).off(0x10).sw(c, 0);
            call_ptr(c, PLAYER_SLOT, 0x0023_bc84)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 1;
            c.g[S2][0] = sx(r32(c, S0) << 2);
            call_ptr(c, PLAYER_SLOT, 0x0023_bc98)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[S3][0] = sx(r32(c, S1) << 2);
            call_ptr(c, PLAYER_CHAR_ID, 0x0023_bcac)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S4][0];
            store_at_slot(c, S2, 0x18);
            call_ptr(c, PLAYER_NAME, 0x0023_bcc4)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S1][0];
            store_at_slot(c, S2, 0x40);
            call_ptr(c, PLAYER_CHAR_ID, 0x0023_bcdc)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S1][0];
            store_at_slot(c, S3, 0x18);
            call_ptr(c, PLAYER_NAME, 0x0023_bcf4)?;
            store_at_slot(c, S3, 0x40);
        } else {
            c.cycles += 37;
            c.g[A2][0] = 0x28;
            c.g[A1][0] = u64::MAX;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
            call_ptr(c, MEMSET, 0x0023_bd14)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0;
            call_ptr(c, PLAYER_SLOT, 0x0023_bd20)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0;
            call_ptr(c, PLAYER_SLOT, 0x0023_bd30)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, PLAYER_CHAR_ID, 0x0023_bd3c)?;
            c.g[S0][0] = sx(r32(c, S0) << 2);
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S4][0];
            store_at_slot(c, S0, 0x18);
            call_ptr(c, PLAYER_SLOT, 0x0023_bd58)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, RACE_OPPONENT, 0x0023_bd64)?;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[S0][0] = sx(r32(c, S0) << 2);
            c.g[A0][0] = c.g[S4][0];
            store_at_slot(c, S0, 0x18);
            c.g[A1][0] = 0;
            call_ptr(c, PLAYER_SLOT, 0x0023_bd84)?;
            c.g[V1][0] = gm_man(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            Addr(r32(c, V1)).off(0x40).sw(c, 0);
        }
        c.cycles += 8;
        f.restore(c, S0, 0x50);
        f.restore(c, S1, 0x40);
        f.restore(c, S2, 0x30);
        f.restore(c, S3, 0x20);
        f.restore(c, S4, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `v1 = manager + slot_reg; sw v0, field(v1)` (a per-rider table entry).
fn store_at_slot(c: &mut Cpu, slot: usize, field: u32) {
    c.g[V1][0] = gm_man(c);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, slot)));
    Addr(r32(c, V1)).off(field).sw(c, c.g[V0][0]);
}

/// `void GMHandler7_Reset(GMHandler7* self)` (0x23c0d0): a timed event
/// (modes 9..11): the medal level from the best score, the target time
/// `self->+0x4` = +0x7c = the medal time in 1/100 s (compared with finish
/// times in 1/100 s, left alone), and the deadline (+0x78) = the time limit
/// in seconds, in game updates; the player seated alone.
pub fn gm_handler7_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let f = Frame::open(c, 0x40);
        f.save(c, S1, 0x20);
        f.save(c, S0, 0x30);
        f.save(c, S2, 0x10);
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        race_player_score(
            c,
            [
                0x0023_c0ec,
                0x0023_c0f8,
                0x0023_c100,
                0x0023_c10c,
                0x0023_c114,
                0x0023_c120,
            ],
        )?;
        let gm = gm_man(c);
        let g = Addr(gm as u32);
        c.g[A1][0] = gm;
        c.g[A2][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        c.g[V1][0] = 0x53_0000;
        g.sw(c, 1);
        c.g[A0][0] = 9;
        c.g[V1][0] = Addr(EVENT_MODE).lb(c);
        g.off(0x10).sw(c, 0);
        g.off(0x8).sw(c, c.g[V1][0]);
        g.off(0x14).sw(c, 0);
        let mode = c.g[V1][0];
        let entries = if mode == 9 {
            c.cycles += 3;
            Some((0xe, 5))
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((mode as i64) < 9) as u64;
            let below = c.g[V0][0] != 0;
            c.g[V0][0] = gm_man(c);
            if below {
                None
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0xa;
                let ten = mode == 0xa;
                c.g[V0][0] = 0xb;
                if ten {
                    c.cycles += 3;
                    Some((0xf, 6))
                } else {
                    c.cycles += 2;
                    let eleven = mode == 0xb;
                    c.g[V0][0] = gm_man(c);
                    if eleven {
                        c.cycles += 2;
                        Some((0x10, 4))
                    } else {
                        // No table entries; straight to the score lookup.
                        c.cycles += 2;
                        c.g[A0][0] = c.g[A2][0];
                        c.cycles += 8;
                        return gm7_after_entries(c, f);
                    }
                }
            }
        };
        if let Some((level, medal)) = entries {
            c.g[V1][0] = level;
            c.g[V0][0] = medal;
            c.cycles += 3;
            g.off(0x6c).sw(c, medal);
            g.off(0x68).sw(c, level);
            c.g[V0][0] = gm_man(c);
        }
        c.cycles += 1;
        c.g[A0][0] = c.g[A2][0];
        c.cycles += 8;
        gm7_after_entries(c, f)
    })
}

/// The rest of [`gm_handler7_reset`] from the score lookup (0x23c1a4).
fn gm7_after_entries(c: &mut Cpu, f: Frame) -> Flow {
    best_medal_level(c, 0x0023_c1b4, |r| r < 3)?;
    c.cycles += 60;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = Addr(r32(c, S1)).off(0xa).lh(c);
    c.g[A1][0] = Addr(r32(c, V0)).off(0x8).lw(c);
    call_ptr(c, RACE_MEDAL_SECONDS, 0x0023_c1f4)?;
    // The target time in 1/100 s (finish times are in 1/100 s).
    c.g[V1][0] = 0x64;
    let gm = gm_man(c);
    c.g[A2][0] = gm;
    c.mult(0, V0, V0, V1);
    c.g[A0][0] = c.g[S0][0];
    let this = Addr(r32(c, S1));
    this.off(0x4).sw(c, c.g[V0][0]);
    Addr(gm as u32).off(0x7c).sw(c, c.g[V0][0]);
    c.g[A1][0] = Addr(gm as u32).off(0x8).lw(c);
    c.g[A2][0] = this.off(0xa).lh(c);
    call_ptr(c, RACE_LIMIT_SECONDS, 0x0023_c218)?;
    // The deadline: the time limit in seconds, in game updates.
    c.g[A1][0] = deadline_updates_per_second(c);
    let gm = gm_man(c);
    let g = Addr(gm as u32);
    c.g[V1][0] = gm;
    c.mult(0, V0, V0, A1);
    c.g[A0][0] = 2;
    g.off(0x74).sw(c, 2);
    g.off(0x70).sw(c, 2);
    g.off(0x78).sw(c, c.g[V0][0]);
    // self->vtable[4](self).
    c.g[V0][0] = this.lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).off(0x20).lh(c);
    c.g[V1][0] = Addr(r32(c, V0)).off(0x24).lw(c);
    let func = c.g[V1][0] as u32;
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    call_ptr(c, func, 0x0023_c248)?;
    seat_player(c, [0x0023_c25c, 0x0023_c268, 0x0023_c278, 0x0023_c284])?;
    let gm = gm_man(c);
    let g = Addr(gm as u32);
    c.g[V1][0] = gm;
    c.g[S0][0] = sx(r32(c, S0) << 2);
    c.g[A0][0] = 1;
    f.restore(c, S2, 0x10);
    c.g[S0][0] = sx((gm as u32).wrapping_add(r32(c, S0)));
    f.restore_ra(c, 0);
    Addr(r32(c, S0)).off(0x18).sw(c, c.g[V0][0]);
    g.off(0x8c).sw(c, 1);
    g.off(0x9c).sw(c, 0);
    g.off(0x80).sw(c, 0);
    g.off(0x84).sw(c, 0);
    g.off(0x88).sw(c, 1);
    g.off(0x90).sw(c, 0);
    g.off(0x94).sw(c, 0);
    g.off(0x98).sw(c, 0);
    this.off(0x8).sh(c, 0);
    f.restore(c, S0, 0x30);
    f.restore(c, S1, 0x20);
    f.close(c);
    Ok(())
}

/// `cGameModeMan* cGameModeMan::getGM()` (0x237cf8): the game-mode manager singleton, created on first use.
/// On the first call: allocate the 0xa0-byte `GameModeMan` (instance at `gp-0x484`),
/// clear the 10-entry handler table at 0x536668 and fill its empty entries with the mode
/// handlers (a `new` of 0x7c, 0xc or 0x8 bytes is tried up to three times; 0xd0, 4 and 0x2c
/// byte ones once), each with its vtable (0x47cfa0, 0x47cf38, 0x47ce00, 0x47cd30, 0x47cd98,
/// 0x47ccc8, 0x47cc60, 0x47ced0, 0x47ce68). Then publish the manager (`gp-0x480`) and set
/// `iCurrentGameEvent` (+8) to -1 and +0, +4, +0x80 to 0. Returns the instance.
pub fn game_mode_man_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        'b_2380c4: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x484));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            let t = c.g[V0][0] != 0u64;
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            if t {
                break 'b_2380c4;
            } else {
                'b_237dc8: {
                    c.cycles += 16;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0xa0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f58));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[S0][0] = 0x530000;
                    call_ptr(c, 0x00317d70, 0x00237d34)?;
                    c.g[S3][0] = c.g[S0][0];
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x484), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6668));
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x28;
                    call_ptr(c, 0x00416210, 0x00237d4c)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6668));
                    let t = c.g[V1][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                    if t {
                        break 'b_237dc8;
                    } else {
                        'b_237dc4: {
                            c.cycles += 12;
                            c.g[V0][0] = 0x480000;
                            c.g[A0][0] = 0x7c;
                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3f48));
                            c.g[A2][0] = 0u64;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x00237d74)?;
                            c.g[V1][0] = 0x480000;
                            c.sw((c.g[S3][0] as u32).wrapping_add(0x6668), c.g[V0][0]);
                            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3060));
                            let t = c.g[V0][0] != 0u64;
                            c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                            if t {
                                break 'b_237dc4;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0x7c;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x00237d9c)?;
                                c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                                let t = c.g[V0][0] != 0u64;
                                c.sw((c.g[S3][0] as u32).wrapping_add(0x6668), c.g[V0][0]);
                                if t {
                                    break 'b_237dc4;
                                } else {
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A0][0] = 0x7c;
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x00237dbc)?;
                                    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x6668), c.g[V0][0]);
                                    break 'b_237dc4;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                        break 'b_237dc8;
                    }
                }
                'b_237e00: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        break 'b_237e00;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0xd0;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00237dec)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30c8));
                        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        break 'b_237e00;
                    }
                }
                'b_237e34: {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        break 'b_237e34;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00237e20)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3200));
                        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        break 'b_237e34;
                    }
                }
                'b_237e68: {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        break 'b_237e68;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00237e54)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x32d0));
                        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        break 'b_237e68;
                    }
                }
                'b_237ee0: {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                    if t {
                        break 'b_237ee0;
                    } else {
                        'b_237edc: {
                            c.cycles += 12;
                            c.g[V0][0] = 0x480000;
                            c.g[A0][0] = 0xc;
                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3f48));
                            c.g[A2][0] = 0u64;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x00237e8c)?;
                            c.g[V1][0] = 0x480000;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3268));
                            let t = c.g[V0][0] != 0u64;
                            c.sw(c.g[V0][0] as u32, c.g[S2][0]);
                            if t {
                                break 'b_237edc;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0xc;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x00237eb4)?;
                                c.sw(c.g[V0][0] as u32, c.g[S2][0]);
                                let t = c.g[V0][0] != 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                if t {
                                    break 'b_237edc;
                                } else {
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A0][0] = 0xc;
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x00237ed4)?;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                    c.sw(c.g[V0][0] as u32, c.g[S2][0]);
                                    break 'b_237edc;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                        break 'b_237ee0;
                    }
                }
                'b_237f18: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                        break 'b_237f18;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00237f04)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3338));
                        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                        break 'b_237f18;
                    }
                }
                'b_237f90: {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                        break 'b_237f90;
                    } else {
                        'b_237f8c: {
                            c.cycles += 12;
                            c.g[V0][0] = 0x480000;
                            c.g[A0][0] = 0xc;
                            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3f48));
                            c.g[A2][0] = 0u64;
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x00237f3c)?;
                            c.g[V1][0] = 0x480000;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x33a0));
                            let t = c.g[V0][0] != 0u64;
                            c.sw(c.g[V0][0] as u32, c.g[S1][0]);
                            if t {
                                break 'b_237f8c;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0xc;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x00237f64)?;
                                c.sw(c.g[V0][0] as u32, c.g[S1][0]);
                                let t = c.g[V0][0] != 0u64;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                                if t {
                                    break 'b_237f8c;
                                } else {
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A0][0] = 0xc;
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x00237f84)?;
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                                    c.sw(c.g[V0][0] as u32, c.g[S1][0]);
                                    break 'b_237f8c;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                        break 'b_237f90;
                    }
                }
                'b_237fc8: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                        break 'b_237fc8;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x2c;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00237fb4)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3130));
                        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                        break 'b_237fc8;
                    }
                }
                'b_238040: {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                    if t {
                        break 'b_238040;
                    } else {
                        'b_23803c: {
                            c.cycles += 12;
                            c.g[V0][0] = 0x480000;
                            c.g[A0][0] = 0x8;
                            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3f48));
                            c.g[A2][0] = 0u64;
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x00237fec)?;
                            c.g[V1][0] = 0x480000;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3198));
                            let t = c.g[V0][0] != 0u64;
                            c.sw(c.g[V0][0] as u32, c.g[S1][0]);
                            if t {
                                break 'b_23803c;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0x8;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x00238014)?;
                                c.sw(c.g[V0][0] as u32, c.g[S1][0]);
                                let t = c.g[V0][0] != 0u64;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                if t {
                                    break 'b_23803c;
                                } else {
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A0][0] = 0x8;
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x00238034)?;
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                    c.sw(c.g[V0][0] as u32, c.g[S1][0]);
                                    break 'b_23803c;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6668));
                        break 'b_238040;
                    }
                }
                'b_238078: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        break 'b_238078;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00238064)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x32d0));
                        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        break 'b_238078;
                    }
                }
                'b_2380ac: {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x484));
                    if t {
                        break 'b_2380ac;
                    } else {
                        c.cycles += 11;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f48));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00238098)?;
                        c.g[V1][0] = 0x480000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3338));
                        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x484));
                        break 'b_2380ac;
                    }
                }
                c.cycles += 6;
                c.g[V1][0] = 0xffffffffffffffff;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x480), c.g[V0][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x80), 0u64);
                c.sw(c.g[V0][0] as u32, 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                break 'b_2380c4;
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x484));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
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

/// `void GMHandler1::Reset(this)` (0x23a108): reset the single-player game mode
/// handler for the heat in `iNextHeat`: unlock the results, set the heat number
/// (with the quick-start flag forcing heat 3) and per heat (1: first heat,
/// 2: second, 3: final) set the AI counts, clear or rebuild the rider tables
/// (`func_0023A4F0`, `func_0023A668`, memsets of the heat tables to 9 or 0),
/// store the next location/heat and clear the event flags.
pub fn gm_handler1_reset(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x0023a108);
        'b_23a198: {
            'b_23a194: {
                c.cycles += 29;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S0][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), 0u64);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                call_ptr(c, 0x0014dc80, 0x0023a148)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x0023a154)?;
                c.g[S2][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x0023a15c)?;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x0023a168)?;
                c.g[S4][0] = c.g[V0][0];
                c.g[V0][0] = 0x530000;
                c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                let t = c.g[A1][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                if t {
                    break 'b_23a194;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = 0x3;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    if c.g[V0][0] == c.g[A0][0] {
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        break 'b_23a198;
                    } else {
                        c.cycles += 2;
                        c.sw(c.g[V1][0] as u32, c.g[A0][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                        break 'b_23a194;
                    }
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            break 'b_23a198;
        }
        'b_23a4c8: {
            'b_23a4c4: {
                'b_23a4c0: {
                    'b_23a360: {
                        'b_23a254: {
                            c.cycles += 5;
                            c.g[V0][0] = 0x2;
                            c.g[S1][0] = c.lw(c.g[A0][0] as u32);
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x7c), 0u64);
                            let t = c.g[S1][0] == c.g[V0][0];
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x8c), 0u64);
                            if t {
                                break 'b_23a254;
                            } else {
                                'b_23a1c8: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S1][0] as i64) < 3i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_23a1c8;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x5;
                                        if t {
                                            break 'b_23a360;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                            if t {
                                                break 'b_23a4c8;
                                            } else {
                                                break 'b_23a1c8;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = (c.g[S1][0] as i64) < 0;
                                c.g[S1][0] = 0x1;
                                if t {
                                    break 'b_23a4c4;
                                } else {
                                    'b_23a1fc: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x5;
                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                        c.sw((c.g[A0][0] as u32).wrapping_add(0x98), 0u64);
                                        c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                        c.sw(c.g[A0][0] as u32, c.g[S1][0]);
                                        let t = c.g[V1][0] != c.g[S1][0];
                                        c.sw((c.g[A0][0] as u32).wrapping_add(0x14), 0u64);
                                        if t {
                                            break 'b_23a1fc;
                                        } else {
                                            c.cycles += 4;
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x0023a4f0,
                                                f: ee::s06::GMHandler1_PickOpponents,
                                            }
                                            .call(c, 0x0023a1f4)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x0023a668,
                                                f: ee::s06::GMHandler1_SetupRiderSlots,
                                            }
                                            .call(c, 0x0023a1fc)?;
                                            break 'b_23a1fc;
                                        }
                                    }
                                    c.cycles += 22;
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                                    c.g[A1][0] = 0x9;
                                    c.g[A2][0] = 0x78;
                                    call_ptr(c, 0x00416210, 0x0023a20c)?;
                                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                    let t = c.g[A1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0023a220)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x90), c.g[S1][0]);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x94), 0u64);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[S1][0]);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x78), 0u64);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x80), 0u64);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x84), 0u64);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x88), 0u64);
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                                    if t {
                                        break 'b_23a4c0;
                                    } else {
                                        break 'b_23a254;
                                    }
                                }
                            }
                        }
                        'b_23a2ec: {
                            c.cycles += 9;
                            c.g[V0][0] = 0x5;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x14), 0u64);
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                            c.g[S6][0] = 0x530000;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x98), 0u64);
                            c.g[S7][0] = 0x530000;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            let t = c.g[V0][0] != 0u64;
                            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x80));
                            if t {
                                break 'b_23a2ec;
                            } else {
                                c.cycles += 12;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x00146e98, 0x0023a284)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, 0x00147398, 0x0023a294)?;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c));
                                c.g[V0][0] = 0x530000;
                                c.g[A2][0] = 0x2;
                                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6708));
                                'l_23a2a8: loop {
                                    'b_23a2d8: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                        let t = c.g[V0][0] == c.g[S1][0];
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        if t {
                                            break 'b_23a2d8;
                                        } else {
                                            'b_23a2d4: {
                                                c.cycles += 5;
                                                c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                                                if c.g[V0][0] != 0u64 {
                                                    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                                                    break 'b_23a2d4;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_add(c.g[V1][0] as u32));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x18));
                                                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                                    break 'b_23a2d4;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                            break 'b_23a2d8;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                    let t = (c.g[A2][0] as i64) >= 0;
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0023a2a8);
                                        }
                                        continue 'l_23a2a8;
                                    } else {
                                        c.cycles += 2;
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x0023a668,
                                            f: ee::s06::GMHandler1_SetupRiderSlots,
                                        }
                                        .call(c, 0x0023a2ec)?;
                                        break 'b_23a2ec;
                                    }
                                }
                            }
                        }
                        c.cycles += 29;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[A3][0] = 0x2;
                        c.g[V1][0] = 0xffffffffffffffff;
                        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6640));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
                        c.g[A1][0] = 0x9;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[A3][0]);
                        c.g[A2][0] = 0x28;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[A3][0]);
                        call_ptr(c, 0x00416210, 0x0023a314)?;
                        c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x6730));
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0x28;
                        call_ptr(c, 0x00416210, 0x0023a324)?;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = 0x9;
                        c.g[A2][0] = 0x28;
                        call_ptr(c, 0x00416210, 0x0023a334)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[V0][0] = 0x1;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x94), 0u64);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x78), 0u64);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x80), 0u64);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x84), 0u64);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x88), 0u64);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x90), 0u64);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                        if t {
                            break 'b_23a4c4;
                        } else {
                            break 'b_23a360;
                        }
                    }
                    'b_23a458: {
                        'b_23a454: {
                            c.cycles += 9;
                            c.g[S3][0] = 0x1;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                            c.g[S6][0] = 0x530000;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x98), c.g[S3][0]);
                            c.g[S7][0] = 0x530000;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x14), 0u64);
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V0][0] != 0u64;
                            c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa8));
                            if t {
                                break 'b_23a454;
                            } else {
                                'b_23a3d8: {
                                    c.cycles += 2;
                                    let t = c.g[A1][0] == 0u64;
                                    c.g[A0][0] = c.g[S2][0];
                                    if t {
                                        break 'b_23a3d8;
                                    } else {
                                        c.cycles += 19;
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x0023a4f0,
                                            f: ee::s06::GMHandler1_PickOpponents,
                                        }
                                        .call(c, 0x0023a394)?;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.sw(c.g[V0][0] as u32, c.g[S3][0]);
                                        Callee {
                                            addr: 0x0023a668,
                                            f: ee::s06::GMHandler1_SetupRiderSlots,
                                        }
                                        .call(c, 0x0023a3a4)?;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                                        c.g[A1][0] = 0x9;
                                        c.g[A2][0] = 0x78;
                                        c.sw(c.g[V0][0] as u32, c.g[S1][0]);
                                        call_ptr(c, 0x00416210, 0x0023a3bc)?;
                                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0023a3d0)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        if t {
                                            break 'b_23a458;
                                        } else {
                                            break 'b_23a3d8;
                                        }
                                    }
                                }
                                c.cycles += 14;
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x00146e98, 0x0023a3e0)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, 0x00147398, 0x0023a3f0)?;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                                c.g[A0][0] = c.g[S4][0];
                                call_ptr(c, 0x00145750, 0x0023a3fc)?;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48));
                                c.g[V0][0] = 0x530000;
                                c.g[A2][0] = 0x2;
                                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6708));
                                'l_23a410: loop {
                                    'b_23a440: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                        let t = c.g[V0][0] == c.g[S1][0];
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        if t {
                                            break 'b_23a440;
                                        } else {
                                            'b_23a43c: {
                                                c.cycles += 5;
                                                c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                                                if c.g[V0][0] != 0u64 {
                                                    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                                                    break 'b_23a43c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_add(c.g[V1][0] as u32));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x18));
                                                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                                    break 'b_23a43c;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                            break 'b_23a440;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                    let t = (c.g[A2][0] as i64) >= 0;
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0023a410);
                                        }
                                        continue 'l_23a410;
                                    } else {
                                        c.cycles += 2;
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x0023a668,
                                            f: ee::s06::GMHandler1_SetupRiderSlots,
                                        }
                                        .call(c, 0x0023a454)?;
                                        break 'b_23a454;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        break 'b_23a458;
                    }
                    c.cycles += 26;
                    c.g[A3][0] = 0x3;
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6640));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
                    c.g[A1][0] = 0x9;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[A3][0]);
                    c.g[A2][0] = 0x28;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[A3][0]);
                    call_ptr(c, 0x00416210, 0x0023a47c)?;
                    c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x6730));
                    c.g[A1][0] = 0x9;
                    c.g[A2][0] = 0x28;
                    call_ptr(c, 0x00416210, 0x0023a48c)?;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = 0x9;
                    c.g[A2][0] = 0x28;
                    call_ptr(c, 0x00416210, 0x0023a49c)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x94), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x78), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x80), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x84), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x88), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x90), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    break 'b_23a4c0;
                }
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                break 'b_23a4c4;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_23a4c8;
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    })
}

/// `void GMHandler1_PickOpponents(self)` (0x23a4f0): picks the opponents of
/// a single-player event. Asks the BE for the player (interface 1) and race
/// (interface 0) objects, the human rider's character (`charA`) and the peak
/// rival's character (`charB`), then builds `arr[10]` on the stack:
/// `arr[0] = GMrand() % 7 + 10`, `arr[1] = GMrand() % 6 + 10` (+1 when not
/// below `arr[0]`), `arr[2..]` = the characters 0..9 except `charA`/`charB`
/// (while there is room), shuffles the array (`GameRand_Shuffle(arr, 10)`)
/// and writes charA and the array's entries into the handler's slots
/// (+0x10, +0x14..+0x24, +0x34..+0x3c, +0x50, +0x54). Frame 0x70 (s0-s2).
pub fn gm_handler1_pick_opponents(c: &mut Cpu) {
    const GET_BE: u32 = 0x0014_dc80;
    const GET_INTERFACE: u32 = 0x0014_dd58;
    const GET_HUMAN_RIDER: u32 = 0x0014_6e98;
    const GET_CHARACTER: u32 = 0x0014_7398;
    const GET_PEAK_RIVAL: u32 = 0x0014_5750;
    const RAND: Callee = Callee {
        addr: 0x0023_7cd8,
        f: ee::s06::f_00237cd8,
    };
    const SHUFFLE: Callee = Callee {
        addr: 0x0023_c770,
        f: ee::s06::f_0023c770,
    };
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0x30);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, GET_BE, 0x0023_a50c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, GET_INTERFACE, 0x0023_a518)?;
        c.g[S0][0] = c.g[V0][0]; // player interface
        call_ptr(c, GET_BE, 0x0023_a520)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, GET_INTERFACE, 0x0023_a52c)?;
        c.g[S1][0] = c.g[V0][0]; // race interface
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, GET_HUMAN_RIDER, 0x0023_a53c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, GET_CHARACTER, 0x0023_a548)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0]; // charA
        call_ptr(c, GET_PEAK_RIVAL, 0x0023_a554)?;
        c.g[S1][0] = c.g[V0][0]; // charB

        RAND.call(c, 0x0023_a55c)?;
        c.g[V1][0] = 7;
        c.divu(0, V0, V1);
        c.cycles += 8;
        c.g[A0][0] = c.hi[0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xa));
        let arr = r32(c, SP);
        c.sw(arr, c.g[A0][0]);
        RAND.call(c, 0x0023_a57c)?;
        c.g[V1][0] = 6;
        c.divu(0, V0, V1);

        c.cycles += 6;
        c.g[V0][0] = c.lw(arr);
        c.g[A0][0] = c.hi[0];
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(0xa));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let below = c.g[V0][0] != 0;
        c.sw(arr.wrapping_add(4), c.g[V1][0]);
        if !below {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(0xb));
            c.sw(arr.wrapping_add(4), c.g[V0][0]);
        }

        // arr[n + 2] = i for every character i in 0..10 other than charA and
        // charB, while n + 2 < 10.
        c.cycles += 3;
        c.g[A1][0] = 0;
        c.g[V1][0] = 0;
        c.g[A0][0] = sx(arr.wrapping_add(8));
        loop {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10) as u64;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = c.g[SP][0];
                break;
            }
            c.cycles += 2;
            let is_a = c.g[V1][0] == c.g[S0][0];
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(2));
            let mut still_room_known = false;
            if is_a {
                // skipped: fall to the common "v0 = (a1 + 2) < 10" below
            } else {
                c.cycles += 2;
                let is_b = c.g[V1][0] == c.g[S1][0];
                c.g[V0][0] = ((c.g[V0][0] as i64) < 10) as u64;
                if is_b {
                    still_room_known = true;
                } else {
                    c.cycles += 4;
                    c.sw(r32(c, A0), c.g[V1][0]);
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(2));
                }
            }
            if !still_room_known {
                c.cycles += 1;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 10) as u64;
            }
            c.cycles += 2;
            let room = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if room {
                if event_due(c) {
                    deliver_event_here(c, 0x0023_a5b8);
                }
                continue;
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[SP][0];
            break;
        }

        c.cycles += 29;
        c.g[A1][0] = 10;
        SHUFFLE.call(c, 0x0023_a5fc)?;
        let arr = r32(c, SP);
        c.g[V1][0] = c.lw(arr);
        c.g[A0][0] = c.lw(arr.wrapping_add(0x4));
        c.g[A1][0] = c.lw(arr.wrapping_add(0x8));
        c.g[A2][0] = c.lw(arr.wrapping_add(0xc));
        c.g[A3][0] = c.lw(arr.wrapping_add(0x10));
        c.g[T0][0] = c.lw(arr.wrapping_add(0x14));
        c.g[T1][0] = c.lw(arr.wrapping_add(0x18));
        c.g[T2][0] = c.lw(arr.wrapping_add(0x1c));
        c.g[T3][0] = c.lw(arr.wrapping_add(0x20));
        c.g[V0][0] = c.lw(arr.wrapping_add(0x24));
        let this = r32(c, S2);
        c.sw(this.wrapping_add(0x10), c.g[S0][0]);
        c.sw(this.wrapping_add(0x54), c.g[V0][0]);
        c.sw(this.wrapping_add(0x14), c.g[V1][0]);
        c.sw(this.wrapping_add(0x18), c.g[A0][0]);
        c.sw(this.wrapping_add(0x1c), c.g[A1][0]);
        c.sw(this.wrapping_add(0x20), c.g[A2][0]);
        c.sw(this.wrapping_add(0x24), c.g[A3][0]);
        c.sw(this.wrapping_add(0x34), c.g[T0][0]);
        c.sw(this.wrapping_add(0x38), c.g[T1][0]);
        c.sw(this.wrapping_add(0x3c), c.g[T2][0]);
        c.sw(this.wrapping_add(0x50), c.g[T3][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void GameModeMan_StartMode(self, int type)` (0x238160): switches the
/// game mode manager to a new event type. Asks the BE for interface 0 (result
/// unused), calls the current mode handler's virtual method 3 (handler table
/// `0x536668[self->iCurrModeHandler]`), stores `type` at `self + 8` and the
/// network-path byte (0x535c11) at `self + 0xc`, picks the new handler index
/// from `type` (a `jr` table at 0x47c0d0 for the network path 2: types 0..5;
/// otherwise a table at 0x47c0f0: types 0..12, out of range keeps the old
/// handler), stores it at `self + 4`, sets `self + 0x84 = 1` and calls
/// `GameModeMan_ResetMode(self)`. Frame 0x30 (s0, s1).
pub fn game_mode_man_start_mode(c: &mut Cpu) {
    const RESET_MODE: Callee = Callee {
        addr: 0x0023_82d8,
        f: ee::s06::f_002382d8,
    };
    const ENTRY: u32 = 0x0023_8160;
    const NET_PATH_NOT_2: u32 = 0x0023_8228;
    const NET_PATH_2: u32 = 0x0023_81cc;
    const NET_TABLE_JUMP: u32 = 0x0023_81d8;
    const OFFLINE_TABLE_JUMP: u32 = 0x0023_8234;
    const NET_OUT_OF_RANGE: u32 = 0x0023_8220;
    const KEEP_HANDLER: u32 = 0x0023_824c;
    const STORE_HANDLER: u32 = 0x0023_8290;
    const FINISH: u32 = 0x0023_8294;
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 27;
                    let frame = Frame::open(c, 0x30);
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A1][0];
                    call_ptr(c, 0x0014_dc80, 0x0023_817c)?;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014_dd58, 0x0023_8188)?;
                    // handler = HANDLERS[self->iCurrModeHandler]; handler->vt[3]()
                    let this = r32(c, S0);
                    c.g[V1][0] = c.lw(this.wrapping_add(4));
                    c.g[V0][0] = 0x53_0000;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6668));
                    c.g[V1][0] = sx(r32(c, V1) << 2);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[A1][0] = c.lw(r32(c, V1));
                    c.g[V0][0] = c.lw(r32(c, A1));
                    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x18));
                    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1c));
                    let method = r32(c, V1);
                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    call_ptr(c, method, 0x0023_81b4)?;
                    let this = r32(c, S0);
                    c.sw(this.wrapping_add(8), c.g[S1][0]);
                    c.g[V1][0] = 0x53_0000;
                    c.g[A0][0] = 2;
                    c.g[V0][0] = c.lb(r32(c, V1).wrapping_add(0x5c11));
                    let not_net2 = c.g[V0][0] != c.g[A0][0];
                    c.sw(this.wrapping_add(0xc), c.g[V0][0]);
                    pc = if not_net2 { NET_PATH_NOT_2 } else { NET_PATH_2 };
                }
                NET_PATH_2 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < 6) as u64;
                    let out_of_range = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x48_0000;
                    pc = if out_of_range {
                        NET_OUT_OF_RANGE
                    } else {
                        NET_TABLE_JUMP
                    };
                }
                NET_TABLE_JUMP | OFFLINE_TABLE_JUMP => {
                    c.cycles += 6;
                    let table_hi = if pc == NET_TABLE_JUMP { 0x3f30 } else { 0x3f10 };
                    c.g[V1][0] = sx(r32(c, S1) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(table_hi));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[A0][0] = c.lw(r32(c, V1));
                    pc = r32(c, A0);
                }
                // The switch cases: each stores a handler index (or, for
                // KEEP_HANDLER, clears `self + 4`).
                0x0023_81f0 | 0x0023_81f8 | 0x0023_8200 | 0x0023_8208 | 0x0023_8210
                | 0x0023_8218 | NET_OUT_OF_RANGE | 0x0023_8254 | 0x0023_825c | 0x0023_8264
                | 0x0023_826c | 0x0023_8274 | 0x0023_827c | 0x0023_8284 => {
                    let v = match pc {
                        0x0023_81f0 => 8,
                        0x0023_81f8 | 0x0023_8200 | 0x0023_8208 => 9,
                        0x0023_8210 => 5,
                        0x0023_8218 => 6,
                        NET_OUT_OF_RANGE => 8,
                        0x0023_8254 => 1,
                        0x0023_825c => 2,
                        0x0023_8264 => 5,
                        0x0023_826c | 0x0023_8274 | 0x0023_827c => 4,
                        _ => 6,
                    };
                    c.cycles += 2;
                    c.g[V0][0] = v;
                    pc = STORE_HANDLER;
                }
                0x0023_828c => {
                    c.cycles += 1;
                    c.g[V0][0] = 7;
                    pc = STORE_HANDLER;
                }
                NET_PATH_NOT_2 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < 0xd) as u64;
                    let out_of_range = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x48_0000;
                    pc = if out_of_range {
                        FINISH
                    } else {
                        OFFLINE_TABLE_JUMP
                    };
                }
                KEEP_HANDLER => {
                    c.cycles += 2;
                    let this = r32(c, S0);
                    c.sw(this.wrapping_add(4), 0);
                    pc = FINISH;
                }
                STORE_HANDLER => {
                    c.cycles += 1;
                    let this = r32(c, S0);
                    c.sw(this.wrapping_add(4), c.g[V0][0]);
                    pc = FINISH;
                }
                FINISH => {
                    c.cycles += 9;
                    c.g[V0][0] = 1;
                    c.g[A0][0] = c.g[S0][0];
                    let this = r32(c, S0);
                    c.sw(this.wrapping_add(0x84), c.g[V0][0]);
                    RESET_MODE.call(c, 0x0023_82a4)?;
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0023_8160, 0x0023_82b8);
                    return Ok(());
                }
            }
        }
    })
}

/// `void GMHandler1_SetupRiderSlots(GMHandler1* self)` (0x23a668): fills the game mode
/// manager's six character slots (`+0x18`, ids from the heat's row at `self + 0x10 +
/// 0x18 * heat`) and cheat character slots (`+0x40`): ids of 10 or more become the
/// human rider's character, the cheat slot keeping the id; the human's own slot gets
/// the alternate character.
pub fn gm_handler1_setup_rider_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0014dc80, 0x0023a688)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014dd58, 0x0023a694)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00146e98, 0x0023a6a4)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x00147398, 0x0023a6b4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x00147410, 0x0023a6c4)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[T1][0] = c.g[V0][0];
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[A1][0] = 0x18;
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        c.g[T0][0] = 0u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[A3][0] = 0u64;
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0u64;
        }
        c.mult(0, 2, 2, 5);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        'l_23a700: loop {
            'b_23a718: {
                c.cycles += 4;
                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                c.g[V0][0] = ((c.g[V1][0] as i64) < 10i64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sw(c.g[A0][0] as u32, 0u64);
                    break 'b_23a718;
                } else {
                    c.cycles += 2;
                    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                    c.g[V1][0] = c.g[S3][0];
                    break 'b_23a718;
                }
            }
            'b_23a72c: {
                c.cycles += 4;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                let t = c.g[T0][0] != c.g[S2][0];
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                if t {
                    break 'b_23a72c;
                } else {
                    c.cycles += 1;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x40), c.g[T1][0]);
                    break 'b_23a72c;
                }
            }
            c.cycles += 6;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = ((c.g[T0][0] as i64) < 6i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0023a700);
                }
                continue 'l_23a700;
            } else {
                c.cycles += 7;
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
    })
}

/// `GameModeMan_OnRiderFinished` (0x00238358): `cGameModeMan::raceFinish`: tells the mode handler of the current mode that a rider finished, and for a normal (non-replay) race on a known course submits the race result, the mountain-conquered award and the placement to the score interface. `(self, rider, stats)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_mode_man_on_rider_finished(c: &mut Cpu) {
    guest(c, |c| {
        'b_2384e8: {
            'b_2384e4: {
                c.cycles += 35;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.g[V1][0] = 0x530000;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6668));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S1][0] = c.g[A2][0];
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S7][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002383b8)?; // call_indirect
                call_ptr(c, 0x0014dc80, 0x002383c0)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x002383cc)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00144bc0, 0x002383d4)?; // call_indirect
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = ((c.g[V1][0] as i64) < 17i64) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_2384e4;
                } else {
                    'b_2383fc: {
                        c.cycles += 5;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[A0][0] = c.lw(c.g[A0][0] as u32);
                        let t = c.g[A0][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_2383fc;
                        } else {
                            c.cycles += 1;
                            c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                            break 'b_2383fc;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[A1][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    if t {
                        break 'b_2384e8;
                    } else {
                        'b_2384c0: {
                            c.cycles += 18;
                            c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                            call_ptr(c, 0x0014dc80, 0x0023840c)?; // call_indirect
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0x8;
                            call_ptr(c, 0x0014dd58, 0x00238418)?; // call_indirect
                            c.g[S6][0] = c.g[S0][0];
                            c.g[S5][0] = c.g[V0][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[V0][0] = 0x530000;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[S4][0] = c.lw(c.g[S0][0] as u32);
                            c.g[A3][0] = c.g[S4][0];
                            call_ptr(c, 0x00154ab8, 0x00238444)?; // call_indirect
                            if c.g[S2][0] != 0u64 {
                                c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x9c));
                                break 'b_2384c0;
                            } else {
                                'b_2384bc: {
                                    c.cycles += 17;
                                    c.g[A0][0] = 0u64;
                                    call_ptr(c, 0x0014a0e0, 0x00238454)?; // call_indirect
                                    c.g[A0][0] = 0u64;
                                    c.g[S1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014a080, 0x00238460)?; // call_indirect
                                    c.g[S3][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014dc80, 0x00238468)?; // call_indirect
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A1][0] = 0xd;
                                    call_ptr(c, 0x0014dd58, 0x00238474)?; // call_indirect
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A2][0] = c.g[S3][0];
                                    call_ptr(c, 0x001577e0, 0x00238488)?; // call_indirect
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        break 'b_2384bc;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[A2][0] = c.g[S1][0];
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x0015a2e0, 0x002384a0)?; // call_indirect
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S7][0] as u32).wrapping_add(0x9c));
                                            break 'b_2384c0;
                                        } else {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[A2][0] = c.g[S3][0];
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00159cd0, 0x002384bc)?; // call_indirect
                                            break 'b_2384bc;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x9c));
                                break 'b_2384c0;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x530000;
                        if t {
                            break 'b_2384e4;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A3][0] = c.g[S4][0];
                            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                            call_ptr(c, 0x00154ee8, 0x002384e4)?; // call_indirect
                            break 'b_2384e4;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_2384e8;
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        Ok(())
    })
}

/// `void GMHandler0_OnRiderFinished(GMHandler0* self, int idx)` (0x239230,
/// `cGMBigAirHandler_raceFinish`): rider `idx` finished the race. Its heat
/// total (the score, or 0 for a did-not-finish, with the out flag
/// `D_005366A8[idx]`) goes to `self->t[heat - 1][idx]`. When the race is
/// over (`cAI_isRaceFinished`), the sorted standings pick the next heat
/// (`iNextHeat`, `iRestoreHeat`) and the rider's continue flag (`unk_100`),
/// and the quick-race flag (`0x535c11`) decides whether the player gets the
/// result screen (`func_00147D20` win, `func_00147E18` lose). Finally the
/// human player's boost bar is set from the place: 0.7 first, 0.35 second,
/// else 0.
///
/// The body is the generated control flow with its calls through
/// `call_ptr` and its back-edge events through `event_due`; the register
/// and cycle accounting (including the scratch registers the original leaves)
/// is the generated one.
#[allow(
    unreachable_code,
    unused_labels,
    unused_assignments,
    unused_variables,
    unused_mut
)]
pub fn gm_handler0_on_rider_finished(c: &mut Cpu) {
    guest(c, |c| {
        'b_239374: {
            'b_239334: {
                c.cycles += 40;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[S0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.g[S4][0] = c.g[A0][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
                c.g[RA][0] = sx(0x0023926c);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x150));
                call_ptr(c, 0x0014dc80, 0x0023926c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00239278);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00239278)?;
                c.g[RA][0] = sx(0x00239280);
                c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
                call_ptr(c, 0x0014dc80, 0x00239280)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0023928c);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x0023928c)?;
                c.g[S6][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[V0][0] = 0x530000;
                c.g[A0][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x002392a4);
                c.g[S5][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                call_ptr(c, 0x00146e98, 0x002392a4)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x790));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xfc));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.g[SP][0];
                if t {
                    break 'b_239334;
                } else {
                    c.cycles += 4;
                    c.g[S2][0] = c.g[S0][0];
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19c));
                    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = 0x530000;
                    'l_2392e0: loop {
                        c.cycles += 19;
                        c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(4, c.g[V1][0] as u32);
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(4, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.sdr(4, c.g[A1][0] as u32);
                        c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.sdr(6, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002392e0);
                            }
                            continue 'l_2392e0;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            if t {
                                break 'b_239374;
                            } else {
                                break 'b_239334;
                            }
                        }
                    }
                }
            }
            c.cycles += 4;
            c.g[S2][0] = c.g[S0][0];
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19c));
            c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
            c.g[A3][0] = 0x530000;
            'l_239344: loop {
                c.cycles += 11;
                c.g[A0][0] = c.ld(c.g[V1][0] as u32);
                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00239344);
                    }
                    continue 'l_239344;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    break 'b_239374;
                }
            }
        }
        'b_239404: {
            'b_2393d8: {
                c.cycles += 13;
                c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[V1][0] as u32);
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[A1][0] as u32);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x480));
                let t = c.g[A0][0] == 0u64;
                c.g[A1][0] = 0x28;
                if t {
                    break 'b_2393d8;
                } else {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x66a8));
                    c.g[A2][0] = 0x1;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.mult(0, 4, 2, 5);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0u64);
                    let t = true;
                    c.sw(c.g[V1][0] as u32, c.g[A2][0]);
                    if t {
                        break 'b_239404;
                    } else {
                        break 'b_2393d8;
                    }
                }
            }
            c.cycles += 11;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x9c));
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x66a8));
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.mult(0, 4, 2, 5);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[A2][0]);
            c.sw(c.g[V1][0] as u32, 0u64);
            break 'b_239404;
        }
        'b_2398b0: {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[RA][0] = sx(0x00239414);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x0012a250, 0x00239414)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            if t {
                break 'b_2398b0;
            } else {
                'b_239858: {
                    'b_239854: {
                        'b_239828: {
                            'b_239804: {
                                'b_2396d8: {
                                    'b_2395ac: {
                                        c.cycles += 4;
                                        c.g[V0][0] = 0x2;
                                        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                                        if t {
                                            break 'b_2395ac;
                                        } else {
                                            'b_239458: {
                                                'b_239444: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_239444;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[S7][0] = sx((c.g[S1][0] as u32) << 2);
                                                        if t {
                                                            break 'b_239458;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[FP][0] = 0x530000;
                                                            if t {
                                                                break 'b_239854;
                                                            } else {
                                                                break 'b_239444;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x3;
                                                let t = c.g[V1][0] == c.g[V0][0];
                                                c.g[S7][0] = sx((c.g[S1][0] as u32) << 2);
                                                if t {
                                                    break 'b_2396d8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[FP][0] = 0x530000;
                                                    if t {
                                                        break 'b_239854;
                                                    } else {
                                                        break 'b_239458;
                                                    }
                                                }
                                            }
                                            'b_2394b8: {
                                                c.cycles += 7;
                                                c.g[V1][0] =
                                                    c.lw((c.g[A1][0] as u32).wrapping_add(0x70));
                                                c.g[V0][0] =
                                                    c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                                                c.g[A2][0] = 0u64;
                                                c.sw(
                                                    (c.g[A1][0] as u32).wrapping_add(0x74),
                                                    c.g[V1][0],
                                                );
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                let t = (c.g[V0][0] as i64) <= 0;
                                                c.g[FP][0] = 0x530000;
                                                if t {
                                                    break 'b_2394b8;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[V0][0] = 0x530000;
                                                    c.g[T0][0] = c.g[A1][0];
                                                    c.g[A3][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x6640));
                                                    c.g[A0][0] =
                                                        sx((c.g[S4][0] as u32).wrapping_add(0x2c));
                                                    c.g[A1][0] = c.g[S0][0];
                                                    'l_239488: loop {
                                                        c.cycles += 12;
                                                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                                                        c.g[A2][0] =
                                                            sx((c.g[A2][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                                                        c.g[A1][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(c.g[V1][0] as u32));
                                                        c.sw(c.g[A3][0] as u32, c.g[V0][0]);
                                                        c.g[V0][0] = c
                                                            .lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[V0][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] = ((c.g[A2][0] as i64)
                                                            < (c.g[V0][0] as i64))
                                                            as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[A3][0] =
                                                            sx((c.g[A3][0] as u32)
                                                                .wrapping_add(0x4));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x00239488);
                                                            }
                                                            continue 'l_239488;
                                                        } else {
                                                            break 'b_2394b8;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_2394f0: {
                                                'b_2394e8: {
                                                    c.cycles += 10;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x480)
                                                        );
                                                    c.g[A1][0] = c
                                                        .lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                                    c.g[RA][0] = sx(0x002394c8);
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                    call_ptr(c, 0x00238b70, 0x002394c8)?;
                                                    c.g[V0][0] =
                                                        sx((c.g[FP][0] as u32)
                                                            .wrapping_add(0x6730));
                                                    c.g[V0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add(c.g[V0][0] as u32));
                                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                    c.g[V1][0] =
                                                        ((c.g[V1][0] as i64) < 3i64) as u64;
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x480)
                                                        );
                                                    if t {
                                                        break 'b_2394e8;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[V0][0] = 0x3;
                                                        if t {
                                                            break 'b_2394f0;
                                                        } else {
                                                            break 'b_2394e8;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                break 'b_2394f0;
                                            }
                                            'b_23953c: {
                                                c.cycles += 6;
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x70),
                                                    c.g[V0][0],
                                                );
                                                c.g[A2][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                                c.g[V0][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                let t = (c.g[V0][0] as i64) <= 0;
                                                c.g[A0][0] = 0u64;
                                                if t {
                                                    break 'b_23953c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = 0x530000;
                                                    c.g[V1][0] = c.g[S0][0];
                                                    c.g[A1][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x6640));
                                                    'l_239518: loop {
                                                        c.cycles += 9;
                                                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V1][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[V0][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] = ((c.g[A0][0] as i64)
                                                            < (c.g[V0][0] as i64))
                                                            as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[A1][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x4));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x00239518);
                                                            }
                                                            continue 'l_239518;
                                                        } else {
                                                            break 'b_23953c;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_239590: {
                                                c.cycles += 6;
                                                c.g[V1][0] =
                                                    sx((c.g[FP][0] as u32).wrapping_add(0x6730));
                                                c.g[S1][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                                c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                if t {
                                                    break 'b_239590;
                                                } else {
                                                    c.cycles += 8;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x480)
                                                        );
                                                    c.g[A1][0] = c
                                                        .lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                                    c.g[RA][0] = sx(0x00239564);
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                    call_ptr(c, 0x00238b70, 0x00239564)?;
                                                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                                    c.g[V0][0] =
                                                        ((c.g[V0][0] as i64) < 3i64) as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    if t {
                                                        break 'b_239590;
                                                    } else {
                                                        c.cycles += 7;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[A0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x84));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(c.g[S2][0] as u32));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x28));
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x100),
                                                            0u64,
                                                        );
                                                        if t {
                                                            break 'b_239854;
                                                        } else {
                                                            break 'b_239590;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[A1][0] = 0x1;
                                            c.g[V1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S2][0] as u32));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                            let t = true;
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x100),
                                                c.g[A1][0],
                                            );
                                            if t {
                                                break 'b_239854;
                                            } else {
                                                break 'b_2395ac;
                                            }
                                        }
                                    }
                                    'b_239608: {
                                        c.cycles += 6;
                                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                                        c.g[S7][0] = sx((c.g[S1][0] as u32) << 2);
                                        c.g[A2][0] = 0u64;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[FP][0] = 0x530000;
                                        if t {
                                            break 'b_239608;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] = 0x530000;
                                            c.g[T0][0] = c.g[A1][0];
                                            c.g[A3][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x2c));
                                            'l_2395d8: loop {
                                                c.cycles += 12;
                                                c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                c.sw(c.g[A3][0] as u32, c.g[V0][0]);
                                                c.g[V0][0] =
                                                    c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                c.g[V0][0] = ((c.g[A2][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002395d8);
                                                    }
                                                    continue 'l_2395d8;
                                                } else {
                                                    break 'b_239608;
                                                }
                                            }
                                        }
                                    }
                                    'b_23965c: {
                                        c.cycles += 9;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = sx(0x00239618);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        call_ptr(c, 0x00238b70, 0x00239618)?;
                                        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[A0][0] = 0u64;
                                        if t {
                                            break 'b_23965c;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x530000;
                                            c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x2c));
                                            c.g[A1][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                                            'l_239638: loop {
                                                c.cycles += 9;
                                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                                c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                                                c.g[V0][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                c.g[V0][0] = ((c.g[A0][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x00239638);
                                                    }
                                                    continue 'l_239638;
                                                } else {
                                                    break 'b_23965c;
                                                }
                                            }
                                        }
                                    }
                                    'b_2396c4: {
                                        c.cycles += 9;
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6730));
                                        c.g[A2][0] =
                                            sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x70));
                                        c.sw((c.g[A1][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                                        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                        c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        if t {
                                            break 'b_2396c4;
                                        } else {
                                            c.cycles += 9;
                                            c.g[V0][0] = 0x3;
                                            c.sw(
                                                (c.g[A1][0] as u32).wrapping_add(0x70),
                                                c.g[V0][0],
                                            );
                                            c.g[A1][0] = 0x1;
                                            c.g[V1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S2][0] as u32));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                            let t = c.g[S5][0] != 0u64;
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x100),
                                                c.g[A1][0],
                                            );
                                            if t {
                                                break 'b_239854;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] =
                                                    sx((c.g[FP][0] as u32).wrapping_add(0x6730));
                                                if t {
                                                    break 'b_239858;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[A0][0] = c.g[S6][0];
                                                    c.g[RA][0] = sx(0x002396bc);
                                                    c.g[A1][0] = 0u64;
                                                    call_ptr(c, 0x00147d20, 0x002396bc)?;
                                                    let t = true;
                                                    c.g[V0][0] =
                                                        sx((c.g[FP][0] as u32)
                                                            .wrapping_add(0x6730));
                                                    if t {
                                                        break 'b_239858;
                                                    } else {
                                                        break 'b_2396c4;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[V0][0] = 0x1;
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                    if t {
                                        break 'b_239804;
                                    } else {
                                        break 'b_2396d8;
                                    }
                                }
                                'b_239760: {
                                    c.cycles += 10;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[V0][0] = 0x1;
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
                                    c.g[S0][0] = 0u64;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x80));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                    let t = (c.g[A0][0] as i64) <= 0;
                                    c.g[FP][0] = 0x530000;
                                    if t {
                                        break 'b_239760;
                                    } else {
                                        c.cycles += 4;
                                        c.mtc1(0, 20);
                                        c.g[S3][0] = c.g[A0][0];
                                        c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x54));
                                        'l_239710: loop {
                                            'b_239740: {
                                                c.cycles += 11;
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                                                c.g[V1][0] =
                                                    c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A1][0] as u32));
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x470));
                                                c.c_le_s(20, 0);
                                                let t = c.fcc();
                                                c.g[A2][0] = 0x1;
                                                if t {
                                                    break 'b_239740;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[A2][0] = 0u64;
                                                    break 'b_239740;
                                                }
                                            }
                                            'b_239754: {
                                                c.cycles += 2;
                                                let t = c.g[A2][0] != 0u64;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_239754;
                                                } else {
                                                    c.cycles += 3;
                                                    call_ptr(c, 0x00122e50, 0x00239750)?;
                                                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                                                    break 'b_239754;
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                ((c.g[S0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00239710);
                                                }
                                                continue 'l_239710;
                                            } else {
                                                break 'b_239760;
                                            }
                                        }
                                    }
                                }
                                'b_2397a4: {
                                    c.cycles += 5;
                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[A0][0] = 0u64;
                                    if t {
                                        break 'b_2397a4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x530000;
                                        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x54));
                                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                                        'l_239780: loop {
                                            c.cycles += 9;
                                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                            c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            c.g[V0][0] =
                                                ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00239780);
                                                }
                                                continue 'l_239780;
                                            } else {
                                                break 'b_2397a4;
                                            }
                                        }
                                    }
                                }
                                'b_2397f4: {
                                    'b_2397f0: {
                                        'b_2397ec: {
                                            c.cycles += 13;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                            c.g[A1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                            c.g[RA][0] = sx(0x002397b4);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            call_ptr(c, 0x00238b70, 0x002397b4)?;
                                            c.g[A1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                            c.g[V1][0] =
                                                sx((c.g[FP][0] as u32).wrapping_add(0x6730));
                                            c.g[V1][0] =
                                                sx((c.g[S2][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.g[A0][0] =
                                                c.lw((c.g[A1][0] as u32).wrapping_add(0x70));
                                            c.sw(
                                                (c.g[A1][0] as u32).wrapping_add(0x74),
                                                c.g[A0][0],
                                            );
                                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                            c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                                            let t = c.g[S5][0] != 0u64;
                                            c.g[V1][0] = c.g[V0][0] ^ 0x1;
                                            if t {
                                                break 'b_2397ec;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != 0u64;
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_2397f0;
                                                } else {
                                                    c.cycles += 3;
                                                    c.sw(
                                                        (c.g[A1][0] as u32).wrapping_add(0x74),
                                                        0u64,
                                                    );
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[A1][0] as u32).wrapping_add(0x70),
                                                        0u64,
                                                    );
                                                    if t {
                                                        break 'b_2397f4;
                                                    } else {
                                                        break 'b_2397ec;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = 0x3;
                                        break 'b_2397f0;
                                    }
                                    c.cycles += 1;
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                                    break 'b_2397f4;
                                }
                                c.cycles += 2;
                                let t = c.g[V1][0] == 0u64;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_239828;
                                } else {
                                    c.cycles += 2;
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                    break 'b_239804;
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                            let t = c.g[S5][0] != 0u64;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x100), 0u64);
                            if t {
                                break 'b_239854;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S6][0];
                                c.g[RA][0] = sx(0x00239820);
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x00147e18, 0x00239820)?;
                                let t = true;
                                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6730));
                                if t {
                                    break 'b_239858;
                                } else {
                                    break 'b_239828;
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = 0x1;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                        let t = c.g[S5][0] != 0u64;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x100), c.g[A1][0]);
                        if t {
                            break 'b_239854;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S6][0];
                            c.g[RA][0] = sx(0x00239854);
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x00147d20, 0x00239854)?;
                            break 'b_239854;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6730));
                    break 'b_239858;
                }
                'b_23989c: {
                    'b_239884: {
                        'b_239874: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            let t = c.g[V1][0] != 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_239874;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x515c));
                                let t = true;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_239884;
                                } else {
                                    break 'b_239874;
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if t {
                            break 'b_23989c;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5158));
                            break 'b_239884;
                        }
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = true;
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2f8));
                    if t {
                        break 'b_2398b0;
                    } else {
                        break 'b_23989c;
                    }
                }
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x2f8), 0u64);
                break 'b_2398b0;
            }
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x150));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
        c.pc = t;
        return Ok(());
    })
}

/// `void OVScreenYNDialog_CopyDefaultChoice(OVScreenYNDialog* d (a0))`
/// (0x238348): the field at +0x70 takes the value of +0x74.
pub fn ov_screen_yn_dialog_copy_default_choice(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x74).lw(c);
    let v0 = c.g[V0][0];
    Addr(c.g[A0][0] as u32).off(0x70).sw(c, v0);
    ret(c);
}

/// Return addresses of the 22 stream transfers and of the handler call, in
/// `GameModeMan_SaveState` (writes) and `GameModeMan_LoadState` (reads).
pub(crate) const GMM_SAVE_RETS: [u32; 22] = [
    0x0023_8604,
    0x0023_8620,
    0x0023_863c,
    0x0023_8658,
    0x0023_8674,
    0x0023_8690,
    0x0023_86ac,
    0x0023_86c8,
    0x0023_86e4,
    0x0023_8704,
    0x0023_8724,
    0x0023_8740,
    0x0023_875c,
    0x0023_8778,
    0x0023_8798,
    0x0023_87b4,
    0x0023_87d0,
    0x0023_87ec,
    0x0023_8808,
    0x0023_8824,
    0x0023_8840,
    0x0023_885c,
];

pub(crate) const GMM_LOAD_RETS: [u32; 22] = [
    0x0023_88d4,
    0x0023_88f0,
    0x0023_890c,
    0x0023_8928,
    0x0023_8944,
    0x0023_8960,
    0x0023_897c,
    0x0023_8998,
    0x0023_89b4,
    0x0023_89d4,
    0x0023_89f4,
    0x0023_8a10,
    0x0023_8a2c,
    0x0023_8a48,
    0x0023_8a68,
    0x0023_8a84,
    0x0023_8aa0,
    0x0023_8abc,
    0x0023_8ad8,
    0x0023_8af4,
    0x0023_8b10,
    0x0023_8b2c,
];

/// One stream transfer of the state passes: the virtual call through the
/// stream object in `s0`, slot `slot` (`this` adjusted by the short at
/// `adj`), with the compiled code's register writes.
pub(crate) fn game_mode_man_stream_call(
    c: &mut Cpu,
    ptr: u64,
    n: u64,
    ret: u32,
    adj: u32,
    slot: u32,
) -> Flow {
    c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[A1][0] = ptr;
    c.g[A2][0] = n;
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(adj).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(slot).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// The body shared by `GameModeMan_SaveState` (`load == false`, stream
/// writes) and `GameModeMan_LoadState` (`load == true`, stream reads): 22
/// fields of the manager in order (current handler, game event, AI counts,
/// the character and rank tables, location and heat, the limits, the lock
/// flags), then the current mode handler saves or loads its own state with
/// the same stream. `s0` is the stream, `s1` the manager.
pub(crate) fn game_mode_man_stream_pass(c: &mut Cpu, load: bool) -> Flow {
    let (rets, adj, slot, handler_adj, handler_slot, handler_ret) = if load {
        (&GMM_LOAD_RETS, 0x10, 0x14, 0x58, 0x5c, 0x0023_8b5c)
    } else {
        (&GMM_SAVE_RETS, 0x8, 0xc, 0x50, 0x54, 0x0023_888c)
    };
    let s1_reg = c.g[S1][0];
    let s1 = s1_reg as u32;
    // (field address, byte count) in transfer order; the handler-table
    // entries and the rank tables are global (gp-free) addresses.
    let fields: [(u64, u64); 21] = [
        (sx(s1.wrapping_add(0x4)), 4),
        (s1_reg, 4),
        (sx(s1.wrapping_add(0x8)), 4),
        (sx(s1.wrapping_add(0x10)), 4),
        (sx(s1.wrapping_add(0x14)), 4),
        (sx(s1.wrapping_add(0x18)), 0x28),
        (sx(s1.wrapping_add(0x68)), 4),
        (sx(s1.wrapping_add(0x6c)), 4),
        (sx(s1.wrapping_add(0x70)), 4),
        (0x0053_6730, 0x28), // cGameModeMan_iRanking
        (0x0053_6708, 0x28), // cGameModeMan_iCharRank
        (sx(s1.wrapping_add(0x78)), 4),
        (sx(s1.wrapping_add(0x7c)), 4),
        (sx(s1.wrapping_add(0x80)), 4),
        (0x0053_6640, 0x28), // D_00536640
        (sx(s1.wrapping_add(0x84)), 4),
        (sx(s1.wrapping_add(0x88)), 4),
        (sx(s1.wrapping_add(0x8c)), 4),
        (sx(s1.wrapping_add(0x90)), 4),
        (sx(s1.wrapping_add(0x94)), 4),
        (sx(s1.wrapping_add(0x98)), 4),
    ];
    for (i, (ptr, n)) in fields.into_iter().enumerate() {
        game_mode_man_stream_call(c, ptr, n, rets[i], adj, slot)?;
    }
    // The last field goes through a3 (the stream's vtable), not v1.
    c.g[A3][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[A1][0] = sx(s1.wrapping_add(0x9c));
    c.g[A2][0] = 0x4;
    c.g[A0][0] = Addr(c.g[A3][0] as u32).off(adj).lh(c);
    c.g[V1][0] = Addr(c.g[A3][0] as u32).off(slot).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[RA][0] = sx(rets[21]);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, rets[21])?;
    // The current mode handler (cGameModeMan_aHandler[iCurrModeHandler]).
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
    c.g[V1][0] = 0x0053_0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6668));
    c.g[A1][0] = c.g[S0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(handler_adj).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(handler_slot).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[RA][0] = sx(handler_ret);
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, handler_ret)?;
    Ok(())
}

/// `void GameModeMan_SaveState(GameModeMan* self (a0), cGmStream* data (a1))`
/// (0x2385d0): writes the manager's state to `data` (see
/// [`game_mode_man_stream_pass`]).
pub fn game_mode_man_save_state(c: &mut Cpu) {
    game_mode_man_state_io(c, false)
}

/// `void GameModeMan_LoadState(GameModeMan* self (a0), cGmStream* data (a1))`
/// (0x2388a0): reads the manager's state from `data` (see
/// [`game_mode_man_stream_pass`]).
pub fn game_mode_man_load_state(c: &mut Cpu) {
    game_mode_man_state_io(c, true)
}

/// The frame shared by the two state functions.
pub(crate) fn game_mode_man_state_io(c: &mut Cpu, load: bool) {
    guest(c, |c| {
        c.cycles += 180;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x4;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        game_mode_man_stream_pass(c, load)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameModeHandler2::Reset(GameModeHandler2* self (a0))` (0x23b170;
/// `GMHandler2_Reset`): the manager starts a new single-heat event. Sets the
/// heat and the AI counts, clears the character table (-1) and gives the
/// human rider its character, resets the rank and points (through the
/// handler's virtual `cleanRiderRanksPoints`), then the limits and flags.
pub fn gm_handler2_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 62;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0023b194);
        c.g[S2][0] = 0x1;
        call_ptr(c, 0x0014dc80, 0x0023b194)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023b1a0);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014dd58, 0x0023b1a0)?; // BELibrary_GetInterface
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x480)).lw(c);
        c.g[A2][0] = 0x28;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x18));
        let one = c.g[S2][0];
        let gm = c.g[V1][0] as u32;
        Addr(gm).sw(c, one); // iHeatNum
        Addr(gm).off(0x98).sw(c, one); // bIsLastHeat
        Addr(gm).off(0x10).sw(c, 0); // iNumberAI
        c.g[RA][0] = sx(0x0023b1c8);
        Addr(gm).off(0x14).sw(c, 0); // iSimAI
        call_ptr(c, 0x00416210, 0x0023b1c8)?; // memset(iCharacters, -1, 40)
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0023b1d4);
        c.g[A1][0] = 0;
        call_ptr(c, 0x00146e98, 0x0023b1d4)?; // BENewPlayerInt_GetHumanRider
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023b1e4);
        c.g[A1][0] = 0;
        call_ptr(c, 0x00146e98, 0x0023b1e4)?; // BENewPlayerInt_GetHumanRider
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0023b1f0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x00147398, 0x0023b1f0)?; // BENewPlayerInt_GetCharacter
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x480)).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        let char_id = c.g[V0][0];
        Addr(c.g[V1][0] as u32).off(0x18).sw(c, char_id); // iCharacters[humanRider]
        c.g[V0][0] = Addr(c.g[S3][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0023b214);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023b214)?; // cleanRiderRanksPoints (virtual)
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x480)).lw(c);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        frame.restore(c, S0, 0x40);
        let gm2 = c.g[V0][0] as u32;
        Addr(gm2).off(0x9c).sw(c, one); // iNextHeat
        Addr(gm2).off(0x70).sw(c, one); // iRestoreHeat
        Addr(gm2).off(0x74).sw(c, one); // bAreResultsLocked
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        let minus_one = c.g[V1][0];
        Addr(gm2).off(0x6c).sw(c, minus_one); // iNextLocation
        Addr(gm2).off(0x78).sw(c, 0); // uiTimeLimit
        Addr(gm2).off(0x7c).sw(c, 0); // uiPointTarget
        Addr(gm2).off(0x80).sw(c, 0); // unk_80
        Addr(gm2).off(0x84).sw(c, 0); // bIsEventDone
        Addr(gm2).off(0x88).sw(c, 0); // bIsTimedEvent
        Addr(gm2).off(0x8c).sw(c, 0); // bHasPointTarget
        Addr(gm2).off(0x90).sw(c, 0); // unk_90
        Addr(gm2).off(0x94).sw(c, 0); // unk_94
        frame.close(c);
        Ok(())
    })
}

/// `gp-0x480`: the game-mode manager pointer (`cGameModeHandler_pManager`).
pub(crate) const GM_MANAGER_GP: u32 = 0x480;

/// `gp-0x848`: the world pointer; its `+0x84` word is the race.
pub(crate) const WORLD_GP: u32 = 0x848;

/// The race-path byte of `cBENewRaceInterface_newRace` (0x535c11).
pub(crate) const NEW_RACE_PATH_BYTE: u32 = 0x0053_5c11;

/// `cGMBigAirHandler_fillScoreTable` (0x00239aa0, `GMHandler0_InitStandings`):
/// fills the standings table of the big-air event. The non-simulated riders
/// (`iNumberAI - iSimAI`, plus the human) take rows 0.. with rank `5 - i` and
/// zero scores in all three heats. Each simulated rider then gets its three
/// heat thresholds (heat 1 is repeated when the race path asks for it). The
/// score table is sorted with `GameModeMan_SortByScore` at the end.
pub fn gm_handler0_init_standings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);

        c.g[S0][0] = c.g[A0][0]; // self
                                 // race = BE interface 0, player = BE interface 1
        call_ptr(c, 0x0014_dc80, 0x0023_9ad4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_9ae0)?;
        let s = sp(c);
        let race = c.g[V0][0];
        c.sw(s, race);
        call_ptr(c, 0x0014_dc80, 0x0023_9ae8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0023_9af4)?;

        // nB = iSimAI simulated riders; numNonSimulated = iNumberAI - nB.
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(GM_MANAGER_GP));
        c.g[T0][0] = 0; // i of the ranking loop
        let s = sp(c);
        let player = c.g[V0][0];
        c.sw(s.wrapping_add(4), player);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14)); // iSimAI
        let s = sp(c);
        let sim = c.g[V0][0];
        c.sw(s.wrapping_add(8), sim);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10)); // iNumberAI
        let s = sp(c);
        c.g[V1][0] = c.lw(s.wrapping_add(8));
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(1)); // rows of the first loop
        c.g[A3][0] = 0; // k: the next free row

        // The human and non-simulated rows: rank 5 - i, zero scores.
        if (c.g[A0][0] as i64) > 0 {
            c.cycles += 8;
            c.g[V0][0] = 0x53_0000;
            c.g[V1][0] = 0x53_0000;
            c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
            c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
            c.g[T1][0] = c.g[A0][0];
            c.g[T2][0] = 5;
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(4)); // &self->t[0][0]
            'rows: loop {
                c.cycles += 10;
                c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                c.g[A2][0] = c.g[V0][0];
                c.g[S5][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S3][0] as u32));
                let rank = c.g[V1][0];
                let rank_slot = c.g[V0][0] as u32;
                c.sw(rank_slot, rank);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                c.g[V0][0] = 2;
                // Heats 2, 1, 0 of this row: zero.
                'heats0: loop {
                    c.cycles += 7;
                    let cell = c.g[A0][0] as u32;
                    c.sw(cell, 0);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    let more = (c.g[V0][0] as i64) >= 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x28));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_9b68);
                        }
                        continue 'heats0;
                    }
                    break;
                }
                c.cycles += 8;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[T0][0] = c.g[V1][0];
                c.g[A0][0] = c.lw(c.g[V0][0] as u32); // self->t[0][i]
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T3][0] as u32));
                c.g[A3][0] = c.g[S5][0];
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                let score_slot = c.g[V1][0] as u32;
                c.sw(score_slot, c.g[A0][0]);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_9b40);
                    }
                    continue 'rows;
                }
                break;
            }
        }

        // The simulated riders: three heat thresholds each.
        c.cycles += 5;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1)); // nA = numNonSimulated + 1
        c.g[A0][0] = c.lw(sp(c).wrapping_add(8)); // nB
        let s = sp(c);
        let n_a = c.g[A1][0];
        c.sw(s.wrapping_add(0xc), n_a);
        c.g[S4][0] = 0; // simRider
        if (c.g[A0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            'sim: loop {
                c.cycles += 8;
                c.g[S2][0] = sx((c.g[A3][0] as u32) << 2); // k * 4
                c.g[S1][0] = 0; // heat
                c.g[S7][0] = c.g[S2][0];
                c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[S5][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.g[V0][0] = 0x53_0000;
                'heat: loop {
                    // Heat 0 and heats with the path byte clear: a fresh
                    // threshold. Else: a repeat of heat 0's score.
                    c.cycles += 7;
                    c.g[V0][0] = 0x53_5bc8;
                    c.g[V1][0] = c.lb(NEW_RACE_PATH_BYTE);
                    c.g[V0][0] = 1;
                    if c.g[S1][0] == 0 {
                        c.g[V0][0] = 0;
                    }
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = 0;
                    }
                    let fresh = c.g[V0][0] == 0;
                    c.g[V1][0] = 0x28;
                    if fresh {
                        c.cycles += 14;
                        let s = sp(c);
                        c.g[A0][0] = c.lw(s.wrapping_add(4)); // player
                        c.g[A1][0] = 0;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        call_ptr(c, 0x0014_7cb8, 0x0023_9c28)?;
                        let s = sp(c);
                        c.g[A0][0] = c.lw(s); // race
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S4][0];
                        call_ptr(c, 0x0014_53d0, 0x0023_9c3c)?;
                        c.g[A0][0] = 0x28;
                        c.mult(0, 4, 17, 4);
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        let slot = c.g[V1][0] as u32;
                        c.sw(slot, c.g[V0][0]);
                    } else {
                        c.cycles += 7;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        c.mult(0, 3, 17, 3);
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S7][0] as u32));
                        c.g[V1][0] = c.lw(c.g[S6][0] as u32);
                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        let slot = c.g[V0][0] as u32;
                        c.sw(slot, c.g[V1][0]);
                    }

                    c.cycles += 4;
                    c.g[S1][0] = c.g[S0][0];
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 3) as u64;
                    let more_heats = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x53_0000;
                    if more_heats {
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_9be0);
                        }
                        continue 'heat;
                    }

                    // The rider's score slot: the heat-0 score goes to the
                    // table and the simRider ranking is stored.
                    c.cycles += 15;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[A0][0] = 0x53_0000;
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6730));
                    c.g[V0][0] = 0x53_0000;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                    let rank_slot = c.g[V1][0] as u32;
                    c.sw(rank_slot, c.g[S4][0]);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S4][0] = c.g[FP][0];
                    let score_slot = c.g[A0][0] as u32;
                    c.sw(score_slot, c.g[A1][0]);
                    c.g[V1][0] = c.lw(sp(c).wrapping_add(8));
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more_sims = c.g[V0][0] != 0;
                    c.g[A3][0] = c.g[S5][0];
                    if more_sims {
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_9bc0);
                        }
                        continue 'sim;
                    }
                    break 'sim;
                }
            }
        }

        // Sort the score table: (nA + nB) rows.
        c.cycles += 17;
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0xc));
        c.g[V1][0] = c.lw(sp(c).wrapping_add(8));
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(GM_MANAGER_GP));
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, 0x0023_8b70, 0x0023_9cb0)?;

        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cGMMPSlopeHandler_raceFinish` (0x00239ec8, `GMHandler9_OnRiderFinished`):
/// a rider (`idx`) has finished. Copies the rider's totals (0xa8 bytes at
/// `rider->metrix + 0xfc`) to the frame, records its score (0 when it did
/// not finish, which also sets its did-not-finish flag) and, once the race is
/// over, locks the event and sorts the table.
pub fn gm_handler9_on_rider_finished(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save_ra(c, 0xb0);

        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(WORLD_GP));
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2); // idx * 4
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84)); // race
        c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc)); // riders
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28)); // rider
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790)); // metrix
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xfc)); // totals source
        c.g[V0][0] = c.g[V1][0] & 0x7;
        let aligned = c.g[V0][0] == 0;
        c.g[A2][0] = c.g[SP][0]; // the copy's destination cursor

        // Both copy loops share this set-up.
        c.cycles += 4;
        c.g[S0][0] = c.g[A1][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19c)); // copy end
        c.g[A1][0] = 0x53_0000;
        c.g[A3][0] = 0x53_0000;
        if aligned {
            'copy8: loop {
                c.cycles += 11;
                c.g[A0][0] = c.ld(c.g[V1][0] as u32);
                c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[T3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A2][0] as u32, c.g[A0][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let more = c.g[V1][0] != c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_9f74);
                    }
                    continue 'copy8;
                }
                c.cycles += 1;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(WORLD_GP));
                break;
            }
        } else {
            'copy_unaligned: loop {
                c.cycles += 19;
                c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(4, c.g[V1][0] as u32);
                c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.ldl(10, (c.g[V1][0] as u32).wrapping_add(0x17));
                c.ldr(10, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.ldl(11, (c.g[V1][0] as u32).wrapping_add(0x1f));
                c.ldr(11, (c.g[V1][0] as u32).wrapping_add(0x18));
                c.sdl(4, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.sdr(4, c.g[A2][0] as u32);
                c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.sdr(9, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sdl(10, (c.g[A2][0] as u32).wrapping_add(0x17));
                c.sdr(10, (c.g[A2][0] as u32).wrapping_add(0x10));
                c.sdl(11, (c.g[A2][0] as u32).wrapping_add(0x1f));
                c.sdr(11, (c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let more = c.g[V1][0] != c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_9f10);
                    }
                    continue 'copy_unaligned;
                }
                c.cycles += 2;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(WORLD_GP));
                break;
            }
        }

        // The copy's tail: 8 more bytes at the end of the frame copy.
        c.cycles += 13;
        c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[V1][0] as u32);
        c.g[T1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
        c.sdr(2, c.g[A2][0] as u32);
        c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T1][0]);
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84)); // race
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28)); // rider
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x480)); // did not finish
        let finished = c.g[A0][0] == 0;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6640)); // D_00536640
        if finished {
            c.cycles += 6;
            let s = sp(c);
            c.g[A0][0] = c.lw(s.wrapping_add(0x9c)); // st.TotalPoints
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x66a8));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let score_slot = c.g[V0][0] as u32;
            c.sw(score_slot, c.g[A0][0]);
            let dnf_slot = c.g[V1][0] as u32;
            c.sw(dnf_slot, 0);
        } else {
            c.cycles += 7;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x66a8));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = 1;
            let score_slot = c.g[V0][0] as u32;
            c.sw(score_slot, 0);
            let dnf_slot = c.g[V1][0] as u32;
            c.sw(dnf_slot, c.g[A0][0]);
        }

        // The race is over when cAI_isRaceFinished says so: lock the event
        // and sort the table; the rider that ranks first keeps no flag.
        c.cycles += 4;
        c.g[A0][0] = c.g[T0][0];
        call_ptr(c, 0x0012_a250, 0x0023_a014)?;
        let over = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(GM_MANAGER_GP));
        if over {
            c.cycles += 12;
            c.g[V1][0] = 1;
            c.g[A1][0] = 2;
            let flag_slot = (c.g[V0][0] as u32).wrapping_add(0x9c);
            c.sw(flag_slot, c.g[V1][0]); // bIsEventDone
            c.g[A0][0] = c.g[V0][0];
            let locked_slot = (c.g[V0][0] as u32).wrapping_add(0x84);
            c.sw(locked_slot, c.g[V1][0]); // bAreResultsLocked
            call_ptr(c, 0x0023_8b70, 0x0023_a034)?;
            c.g[V0][0] = 0x53_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32); // iRanking[idx]
            let ranked = (c.g[V1][0] as i64) > 0;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(WORLD_GP));
            if ranked {
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                let unk_slot = (c.g[V1][0] as u32).wrapping_add(0x100);
                c.sw(unk_slot, 0);
            }
        }

        c.cycles += 4;
        frame.restore(c, S0, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.close(c);
        Ok(())
    })
}

// Callees of the game-mode handlers (generated addresses).
/// `cBE_getBE()`.
pub(crate) const BE_GET_BE: u32 = 0x0014_dc80;

/// `cBE_getInterface(be, n)`: 0 race, 1 player, 8 score.
pub(crate) const BE_GET_INTERFACE: u32 = 0x0014_dd58;

/// `cBENewPlayerInterface_getHumanRider(player, n)`.
pub(crate) const BE_GET_HUMAN_RIDER: u32 = 0x0014_6e98;

/// `cBENewRaceInterface_getRival(race, n)`.
pub(crate) const BE_GET_RIVAL: u32 = 0x0014_5750;

/// Player character id (`GET_CHAR_ID`): `(player, alt)`.
pub(crate) const BE_GET_CHAR_ID: u32 = 0x0014_7488;

/// Player alternate character id (`GET_ALT_CHAR`): `(player, alt)`.
pub(crate) const BE_GET_ALT_CHAR: u32 = 0x0014_74c8;

/// `cBENewPlayerInterface_getRiderCheatCharID(player, n)`.
pub(crate) const BE_GET_RIDER_CHEAT_ID: u32 = 0x0014_7410;

/// The player's rider-slot call with `(player, n)` (generated `0x147398`).
pub(crate) const BE_PLAYER_SLOT: u32 = 0x0014_7398;

/// `cAI_isRaceFinished(ai)`.
pub(crate) const AI_IS_RACE_FINISHED: u32 = 0x0012_a250;

/// `cRider_estimateFinishTick(rider)`.
pub(crate) const RIDER_ESTIMATE_TICK: u32 = 0x0012_2d78;

/// `cRider_estimateFinishScore(rider)`.
pub(crate) const RIDER_ESTIMATE_SCORE: u32 = 0x0012_2e50;

/// `GameModeMan_SortByTime(handler, n)`.
pub(crate) const SORT_BY_TIME: u32 = 0x0023_8bf8;

/// `GameModeMan_SortByScore(handler, n)`.
pub(crate) const SORT_BY_SCORE_2: u32 = 0x0023_8b70;

/// `GMHandler8_PickOpponents(self)`.
pub(crate) const PICK_OPPONENTS_2: u32 = 0x0023_b0a8;

/// `gp-0x480`: the game-mode handler (the current GM object).
#[cfg(test)]
pub(crate) const GP_GM: u32 = 0x480;

/// `void GMHandler8_Reset(GMHandler8* self)` (0x23ac88): reset the GM
/// handler's screen fields, clear the 5*8-byte table at `self+4`, run its
/// vtable's slot 5, copy the player's character ids and alt-character ids
/// into the handler, and pick opponents when the mode word was 1.
pub fn gm_handler8_reset_port(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 16, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 1;

        // player = cBE_getInterface(cBE_getBE(), 1)
        call_ptr(c, BE_GET_BE, 0x0023_aca8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, BE_GET_INTERFACE, 0x0023_acb4)?;
        c.g[S0][0] = c.g[V0][0];

        c.g[V1][0] = 4;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[A0][0] = 3;
        c.g[A2][0] = 2;
        c.g[A3][0] = u64::MAX;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[A1][0] = 7;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[A3][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x94), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x90), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x98), c.g[S2][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x88), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[A0][0]);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
        call_ptr(c, BE_GET_INTERFACE, 0x0023_ad0c)?;

        // A profile word in the BE library's static data selects the copy.
        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
        let no_profile = c.g[V0][0] == 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        if !no_profile {
            c.cycles += 10;
            call_ptr(c, BE_GET_BE, 0x0023_ad24)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, BE_GET_INTERFACE, 0x0023_ad30)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c04));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        }

        // memset(self + 4, 0, 0x28), then zero the handler's timers.
        c.cycles += 36;
        c.g[A2][0] = 0x28;
        c.g[A1][0] = 9;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x80), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x78), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), 0);
        call_ptr(c, MEMSET, 0x0023_ad68)?;

        // self's vtable slot 5: {int16 delta; fn}, called with self + delta.
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let slot = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, slot, 0x0023_ad7c)?;

        // The character ids (+0x18, +0x1c) and alt ids (+0x40, +0x44).
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, BE_GET_CHAR_ID, 0x0023_ad88)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        call_ptr(c, BE_GET_CHAR_ID, 0x0023_ad9c)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        call_ptr(c, BE_GET_ALT_CHAR, 0x0023_adb0)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        call_ptr(c, BE_GET_ALT_CHAR, 0x0023_adc4)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        let mode_not_one = c.g[A0][0] != c.g[S2][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
        if !mode_not_one {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: PICK_OPPONENTS_2,
                f: ee::s06::GMHandler8_PickOpponents,
            }
            .call(c, 0x0023_addc)?;
        }

        c.cycles += 8;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GMHandler5_Reset(GMHandler5* self)` (0x23b6c0): the rival time
/// handler's reset. The byte at `0x535c11` picks the branch: when it is 2 the
/// player's per-rider tables are filled from the riders and the function
/// returns; otherwise the rival tables are cleared and filled from the race.
pub fn gm_handler5_reset_port(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        'b_23b8a8: {
            'b_23b814: {
                c.cycles += 48;
                let frame = Frame::open(c, 0x60);
                frame.save(c, 16, 0x50);
                frame.save(c, 17, 0x40);
                frame.save(c, 20, 0x10);
                frame.save(c, 18, 0x30);
                frame.save(c, 19, 0x20);
                frame.save_ra(c, 0);
                c.g[S0][0] = c.g[A0][0];
                call_ptr(c, BE_GET_BE, 0x0023_b6e4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                call_ptr(c, BE_GET_INTERFACE, 0x0023_b6f0)?;
                c.g[S4][0] = c.g[V0][0];
                call_ptr(c, BE_GET_BE, 0x0023_b6f8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, BE_GET_INTERFACE, 0x0023_b704)?;
                c.g[S1][0] = c.g[V0][0];

                c.g[V1][0] = 1;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[A0][0] = u64::MAX;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[A0][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x98), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), 0);
                c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x68), c.g[A0][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x90, 0x94] {
                    c.sw((c.g[V0][0] as u32).wrapping_add(off), 0);
                }

                // self's vtable slot 5 (delta + fn), called with self + delta.
                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let slot = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, slot, 0x0023_b76c)?;

                c.g[V1][0] = 0x0053_0000;
                c.g[A0][0] = 2;
                c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                let flag_not_two = c.g[V0][0] != c.g[A0][0];
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                if flag_not_two {
                    break 'b_23b814;
                }

                // Flag == 2: one table per rider from the player's riders.
                c.cycles += 37;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = 0;
                c.g[RA][0] = sx(0x0023_b794);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), 0);
                call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_b794)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = 1;
                c.g[S2][0] = sx((c.g[S0][0] as u32) << 2);
                call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_b7a8)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[S3][0] = sx((c.g[S1][0] as u32) << 2);
                call_ptr(c, BE_PLAYER_SLOT, 0x0023_b7bc)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                call_ptr(c, BE_GET_RIDER_CHEAT_ID, 0x0023_b7d4)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                call_ptr(c, BE_PLAYER_SLOT, 0x0023_b7ec)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                call_ptr(c, BE_GET_RIDER_CHEAT_ID, 0x0023_b804)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                break 'b_23b8a8;
            }

            // Flag != 2: the rival tables.
            c.cycles += 37;
            c.g[A2][0] = 0x28;
            c.g[A1][0] = u64::MAX;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, MEMSET, 0x0023_b824)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0;
            call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_b830)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0;
            call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_b840)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, BE_PLAYER_SLOT, 0x0023_b84c)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S4][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_b868)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, BE_GET_RIVAL, 0x0023_b874)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[A0][0] = c.g[S4][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = 0;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_b894)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x40), 0);
        }

        c.cycles += 8;
        frame_epilogue_b6c0(c);
        Ok(())
    })
}

/// The epilogue of `GMHandler5_Reset` (0x23b6c0): reload s0.. and `ra`
/// from its frame, pop it and return.
pub(crate) fn frame_epilogue_b6c0(c: &mut Cpu) {
    let frame = Frame { size: 0x60 };
    frame.restore(c, 16, 0x50);
    frame.restore(c, 17, 0x40);
    frame.restore(c, 18, 0x30);
    frame.restore(c, 19, 0x20);
    frame.restore(c, 20, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void GMHandler5_OnRiderFinished(GMHandler5* self, int idx)` (0x23b8c8):
/// the rival time handler's rider-finish step. When the race is finished and
/// the rider `idx` has finished (its state at +0x6c0), it stores the rider's
/// finish tick in the rival table and sorts the times; the player's own
/// entry is recorded at the end.
pub fn gm_handler5_on_rider_finished_port(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        'b_23b9b0: {
            'b_23b934: {
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                frame.save(c, 16, 0x30);
                frame.save(c, 17, 0x20);
                c.g[S0][0] = c.g[A1][0];
                frame.save(c, 18, 0x10);
                frame.save_ra(c, 0);
                frame.save_fpr(c, 20, 0x40);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                call_ptr(c, AI_IS_RACE_FINISHED, 0x0023_b8f4)?;
                let not_finished = c.g[V0][0] == 0;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if not_finished {
                    break 'b_23b934;
                }
                c.cycles += 14;
                c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[S2][0] = c.g[A0][0];
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                let vfn = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, vfn, 0x0023_b92c)?;
                let entry_done = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if entry_done {
                    break 'b_23b9b0;
                }
                break 'b_23b934;
            }

            'b_23b9ac: {
                'b_23b988: {
                    c.cycles += 10;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[S2][0] = c.g[A0][0];
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x480));
                    let no_rider_block = c.g[V1][0] == 0;
                    c.g[V1][0] = 0x0053_0000;
                    if no_rider_block {
                        break 'b_23b988;
                    }
                    // The rider has a block: store its finish flag in the
                    // rival table and the point table.
                    c.cycles += 11;
                    c.g[A0][0] = 0x0053_0000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                    c.g[V0][0] = 0x0005_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x66a8));
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] |= 0x7e40;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A1][0] = 1;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                    break 'b_23b9ac;
                }
                c.cycles += 9;
                c.g[V0][0] = 0x0053_0000;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x478));
                c.g[V1][0] = 0x0053_0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66a8));
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                c.sw(c.g[V1][0] as u32, 0);
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        }

        // 'b_23bb7c: the player's own time, once the race is finished.
        'b_23bb7c: {
            'b_23bb78: {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, AI_IS_RACE_FINISHED, 0x0023_b9bc)?;
                let not_finished = c.g[V0][0] == 0;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if not_finished {
                    break 'b_23bb78;
                }
                c.cycles += 12;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                let vfn = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, vfn, 0x0023_b9ec)?;
                if c.g[V0][0] == 0 {
                    frame.restore(c, 16, 0x30);
                    break 'b_23bb7c;
                }

                'b_23bb68: {
                    'b_23ba5c: {
                        c.cycles += 10;
                        call_ptr(c, BE_GET_BE, 0x0023_b9fc)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        call_ptr(c, BE_GET_INTERFACE, 0x0023_ba08)?;
                        c.g[V1][0] = 0x0053_0000;
                        c.g[A0][0] = 2;
                        c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                        let flag_not_two = c.g[V0][0] != c.g[A0][0];
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        if flag_not_two {
                            break 'b_23ba5c;
                        }
                        c.cycles += 14;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[A1][0] = 0xa;
                        call_ptr(c, SORT_BY_TIME, 0x0023_ba28)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[A0][0] = 1;
                        c.g[A1][0] = 0x0053_0000;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), c.g[A0][0]);
                        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6640));
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x6640));
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[A0][0] = 0;
                        }
                        let is_player = c.g[S0][0] == c.g[A0][0];
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if is_player {
                            break 'b_23bb78;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        break 'b_23bb68;
                    }

                    'b_23bad8: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                        let none = (c.g[V0][0] as i64) <= 0;
                        c.g[S0][0] = 0;
                        if none {
                            break 'b_23bad8;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x0053_0000;
                        c.mtc1(0, 20);
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                        'l_23ba78: loop {
                            'b_23baa8: {
                                c.cycles += 11;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x470));
                                c.c_le_s(20, 0);
                                let finish_le = c.fcc();
                                c.g[A2][0] = 1;
                                if finish_le {
                                    break 'b_23baa8;
                                }
                                c.cycles += 1;
                                c.g[A2][0] = 0;
                            }
                            'b_23bac0: {
                                c.cycles += 2;
                                let not_done = c.g[A2][0] != 0;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                if not_done {
                                    break 'b_23bac0;
                                }
                                c.cycles += 4;
                                call_ptr(c, RIDER_ESTIMATE_TICK, 0x0023_bab8)?;
                                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                            }
                            c.cycles += 6;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let more = c.g[V0][0] != 0;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0023_ba78);
                                }
                                continue 'l_23ba78;
                            }
                            break 'b_23bad8;
                        }
                    }

                    'b_23bb64: {
                        c.cycles += 21;
                        c.g[RA][0] = sx(0x0023_bae0);
                        c.g[S1][0] = 1;
                        call_ptr(c, BE_GET_BE, 0x0023_bae0)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 1;
                        call_ptr(c, BE_GET_INTERFACE, 0x0023_baec)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_baf8)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = 0xa;
                        c.g[A0][0] = c.g[V1][0];
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x9c), c.g[S1][0]);
                        call_ptr(c, SORT_BY_TIME, 0x0023_bb10)?;
                        c.g[V0][0] = 0x0053_0000;
                        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                        let slot_taken = c.g[V1][0] != 0;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if slot_taken {
                            break 'b_23bb64;
                        }
                        c.cycles += 7;
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x480));
                        let rider_taken = c.g[V1][0] != 0;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if rider_taken {
                            break 'b_23bb64;
                        }
                        c.cycles += 7;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x100), c.g[S1][0]);
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), c.g[S1][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), 0);
                        break 'b_23bb78;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x100), 0);
                break 'b_23bb78;
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
            break 'b_23bb7c;
        }

        c.cycles += 6;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void GMHandler6_OnRiderFinished(GMHandler6* self, int idx)` (0x23bdb8):
/// the point-score handler's rider-finish step. Copies a 0xa0-byte block of
/// the rider's data into the frame (`ldl`/`ldr` when the source is unaligned),
/// then the rider's point entry is recorded; when the race is finished the
/// scores are sorted and the player's own entry is recorded.
pub fn gm_handler6_on_rider_finished_port(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 16;
        let frame = Frame::open(c, 0x100);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, 16, 0xe0);
        frame.save(c, 17, 0xd0);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0xc0);
        c.g[A2][0] = sx((c.g[S0][0] as u32) << 2);
        frame.save_ra(c, 0xb0);
        frame.save_fpr(c, 20, 0xf0);
        'b_23bf08: {
            'b_23be20: {
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x480));
                let has_block = c.g[V1][0] == 0;
                c.g[V1][0] = 0x0053_0000;
                if has_block {
                    break 'b_23be20;
                }
                c.cycles += 10;
                c.g[A1][0] = 0x0053_0000;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6640));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66a8));
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = 1;
                c.sw(c.g[V0][0] as u32, 0);
                c.sw(c.g[V1][0] as u32, c.g[A0][0]);
                c.g[S1][0] = c.g[A1][0];
                break 'b_23bf08;
            }

            // The copy of the 32-byte result block into the stack frame.
            'b_23bed4: {
                'b_23be98: {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790));
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xfc));
                    c.g[V0][0] = c.g[V1][0] & 7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[SP][0];
                    if aligned {
                        break 'b_23be98;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19c));
                    c.g[S2][0] = c.g[A2][0];
                    c.g[S1][0] = 0x0053_0000;
                    c.g[V0][0] = 0x0053_0000;
                    'l_23be44: loop {
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
                        let more = c.g[V1][0] != c.g[A0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0023_be44);
                            }
                            continue 'l_23be44;
                        }
                        c.cycles += 2;
                        break 'b_23bed4;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19c));
                c.g[S2][0] = c.g[A2][0];
                c.g[S1][0] = 0x0053_0000;
                c.g[V0][0] = 0x0053_0000;
                'l_23bea8: loop {
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
                    let more = c.g[V1][0] != c.g[A0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_bea8);
                        }
                        continue 'l_23bea8;
                    }
                    break 'b_23bed4;
                }
            }

            c.cycles += 13;
            c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(4, c.g[V1][0] as u32);
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.sdl(4, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(4, c.g[A1][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66a8));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x9c));
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6640));
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0);
            c.sw(c.g[V1][0] as u32, c.g[A0][0]);
            break 'b_23bf08;
        }

        // 'b_23c0b4: the player's score once the race is finished.
        'b_23c0b4: {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            call_ptr(c, AI_IS_RACE_FINISHED, 0x0023_bf18)?;
            if c.g[V0][0] == 0 {
                frame.restore(c, 16, 0xe0);
                break 'b_23c0b4;
            }

            'b_23c0b0: {
                'b_23c0a8: {
                    'b_23bf90: {
                        c.cycles += 10;
                        call_ptr(c, BE_GET_BE, 0x0023_bf28)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        call_ptr(c, BE_GET_INTERFACE, 0x0023_bf34)?;
                        c.g[V1][0] = 0x0053_0000;
                        c.g[A0][0] = 2;
                        c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                        let flag_not_two = c.g[V0][0] != c.g[A0][0];
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        if flag_not_two {
                            break 'b_23bf90;
                        }
                        c.cycles += 13;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[A1][0] = 2;
                        call_ptr(c, SORT_BY_SCORE_2, 0x0023_bf54)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[V1][0] = 1;
                        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6640));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6640));
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[V1][0] = 0;
                        }
                        let is_player = c.g[S0][0] == c.g[V1][0];
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if is_player {
                            break 'b_23c0b0;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        break 'b_23c0a8;
                    }

                    // The rider slot is free: the score table (+0x1..) walk.
                    'b_23c010: {
                        c.cycles += 5;
                        c.g[S2][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                        let none = (c.g[V0][0] as i64) <= 0;
                        c.g[S0][0] = 0;
                        if none {
                            break 'b_23c010;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x0053_0000;
                        c.mtc1(0, 20);
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6640));
                        'l_23bfb0: loop {
                            'b_23bfe0: {
                                c.cycles += 11;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x470));
                                c.c_le_s(20, 0);
                                let finish_le = c.fcc();
                                c.g[A2][0] = 1;
                                if finish_le {
                                    break 'b_23bfe0;
                                }
                                c.cycles += 1;
                                c.g[A2][0] = 0;
                            }
                            'b_23bff8: {
                                c.cycles += 2;
                                let not_done = c.g[A2][0] != 0;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                if not_done {
                                    break 'b_23bff8;
                                }
                                c.cycles += 4;
                                call_ptr(c, RIDER_ESTIMATE_SCORE, 0x0023_bff0)?;
                                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                            }
                            c.cycles += 6;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let more = c.g[V0][0] != 0;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0023_bfb0);
                                }
                                continue 'l_23bfb0;
                            }
                            break 'b_23c010;
                        }
                    }

                    // 'b_23c09c: the race finished: sort the scores, record.
                    'b_23c09c: {
                        c.cycles += 21;
                        c.g[S1][0] = 1;
                        call_ptr(c, BE_GET_BE, 0x0023_c018)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 1;
                        call_ptr(c, BE_GET_INTERFACE, 0x0023_c024)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        call_ptr(c, BE_GET_HUMAN_RIDER, 0x0023_c030)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = 2;
                        c.g[A0][0] = c.g[V1][0];
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x9c), c.g[S1][0]);
                        call_ptr(c, SORT_BY_SCORE_2, 0x0023_c048)?;
                        c.g[V0][0] = 0x0053_0000;
                        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                        let slot_taken = c.g[V1][0] != 0;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if slot_taken {
                            break 'b_23c09c;
                        }
                        c.cycles += 7;
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x480));
                        let rider_taken = c.g[V1][0] != 0;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if rider_taken {
                            break 'b_23c09c;
                        }
                        c.cycles += 7;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x100), c.g[S1][0]);
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), c.g[S1][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), 0);
                        break 'b_23c0b0;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    break 'b_23c0a8;
                }
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x100), 0);
            }
            c.cycles += 1;
            frame.restore(c, 16, 0xe0);
        }

        c.cycles += 6;
        frame.restore(c, 17, 0xd0);
        frame.restore(c, 18, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// `GMHandler7_OnRiderFinished(game, idx)` (0x23c2d8): rider `idx` (a1) has
/// finished the race. The rider's 0xac-byte results block (the rider's
/// `+0x790` pointer plus `0xfc`) is copied to the stack first (8-byte
/// copies, or ldl/ldr pairs when the block is misaligned), then:
/// - a DNF clears the rider's point entry, sets the next heat and flags the
///   rider as not finished;
/// - a total below the point target stores the points and the next heat;
/// - otherwise the rider is ranked 0, 1 or 2 by the two peak goals, and the
///   results are locked.
pub fn gm_handler7_on_rider_finished(c: &mut Cpu) {
    guest(c, rider_finished)
}

pub(crate) fn rider_finished(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, 0x110);
    c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
    c.g[A1][0] = sx(r32(c, A1) << 2);
    frame.save(c, S0, 0x100);
    frame.save(c, S1, 0xf0);
    frame.save(c, S2, 0xe0);
    frame.save(c, S3, 0xd0);
    frame.save(c, S4, 0xc0);
    frame.save_ra(c, 0xb0);
    let game = r32(c, A0);
    c.g[V1][0] = c.lw(game.wrapping_add(0x84));
    let ai = r32(c, V1);
    c.g[V0][0] = c.lw(ai.wrapping_add(0xc));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x28));
    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x790));
    c.g[V1][0] = sx(r32(c, A0).wrapping_add(0xfc));
    c.g[V0][0] = c.g[V1][0] & 0x7;
    let aligned = c.g[V0][0] == 0;
    c.g[A2][0] = c.g[SP][0];
    c.cycles += 3;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x19c));
    c.g[S2][0] = c.g[A1][0];
    c.g[S4][0] = 0x53_0000;
    if aligned {
        rider_copy_aligned(c);
    } else {
        rider_copy_unaligned(c);
    }

    // The copy's last 12 bytes (8 by ldl/ldr, a word) end the stack block.
    c.cycles += 19;
    let src = r32(c, V1);
    c.ldl(2, src.wrapping_add(0x7));
    c.ldr(2, src);
    c.g[A0][0] = c.lw(src.wrapping_add(0x8));
    let dst = r32(c, A2);
    c.sdl(2, dst.wrapping_add(0x7));
    c.sdr(2, dst);
    let word = c.g[A0][0];
    c.sw(dst.wrapping_add(0x8), word);

    call_ptr(c, 0x0014_dc80, 0x0023_c3d4)?; // cBE_getBE()
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0023_c3e0)?; // cBE_getInterface_Fv(be, 0): the race
    c.g[A2][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
    c.g[S3][0] = c.g[V0][0];
    c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x84));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S2)));
    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x28)); // the rider
    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x480)); // mDidNotFinish
    let dnf = c.g[V0][0] != 0;
    c.g[V0][0] = sx(r32(c, S4).wrapping_add(0x6640));
    if dnf {
        rider_dnf(c);
    } else {
        rider_ranked(c)?;
    }

    c.cycles += 11;
    c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[V0][0] = 1;
    frame.restore(c, S0, 0x100);
    frame.restore(c, S1, 0xf0);
    frame.restore(c, S2, 0xe0);
    frame.restore(c, S3, 0xd0);
    frame.restore(c, S4, 0xc0);
    frame.restore_ra(c, 0xb0);
    let gm = r32(c, V1).wrapping_add(0x9c);
    let locked = c.g[V0][0];
    c.sw(gm, locked);
    frame.close(c);
    Ok(())
}

/// The aligned pass of the results copy: 32 bytes (four doublewords) a pass
/// from `V1` to `A2`, until `V1` reaches `V0`.
pub(crate) fn rider_copy_aligned(c: &mut Cpu) {
    loop {
        c.cycles += 11;
        let src = r32(c, V1);
        let dst = r32(c, A2);
        c.g[A0][0] = c.ld(src);
        c.g[A1][0] = c.ld(src.wrapping_add(0x8));
        c.g[A3][0] = c.ld(src.wrapping_add(0x10));
        c.g[T0][0] = c.ld(src.wrapping_add(0x18));
        let (a0, a1, a3, t0) = (c.g[A0][0], c.g[A1][0], c.g[A3][0], c.g[T0][0]);
        c.sd(dst, a0);
        c.sd(dst.wrapping_add(0x8), a1);
        c.sd(dst.wrapping_add(0x10), a3);
        c.sd(dst.wrapping_add(0x18), t0);
        c.g[V1][0] = sx(src.wrapping_add(0x20));
        let more = c.g[V1][0] != c.g[V0][0];
        c.g[A2][0] = sx(dst.wrapping_add(0x20));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0023_c38c);
        }
    }
}

/// The misaligned pass of the results copy: the same 32 bytes a pass, moved
/// with ldl/ldr and sdl/sdr pairs (registers a0, a1, a3, t0 carry the
/// halves the generated loop leaves behind).
pub(crate) fn rider_copy_unaligned(c: &mut Cpu) {
    loop {
        c.cycles += 19;
        let src = r32(c, V1);
        c.ldl(4, src.wrapping_add(0x7));
        c.ldr(4, src);
        c.ldl(5, src.wrapping_add(0xf));
        c.ldr(5, src.wrapping_add(0x8));
        c.ldl(7, src.wrapping_add(0x17));
        c.ldr(7, src.wrapping_add(0x10));
        c.ldl(8, src.wrapping_add(0x1f));
        c.ldr(8, src.wrapping_add(0x18));
        let dst = r32(c, A2);
        c.sdl(4, dst.wrapping_add(0x7));
        c.sdr(4, dst);
        c.sdl(5, dst.wrapping_add(0xf));
        c.sdr(5, dst.wrapping_add(0x8));
        c.sdl(7, dst.wrapping_add(0x17));
        c.sdr(7, dst.wrapping_add(0x10));
        c.sdl(8, dst.wrapping_add(0x1f));
        c.sdr(8, dst.wrapping_add(0x18));
        c.g[V1][0] = sx(src.wrapping_add(0x20));
        let more = c.g[V1][0] != c.g[V0][0];
        c.g[A2][0] = sx(dst.wrapping_add(0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0023_c32c);
            }
            continue;
        }
        c.cycles += 2;
        return;
    }
}

/// The DNF branch of [`gm_handler7_on_rider_finished`] (no calls): the
/// rider's point entry is cleared, the next heat is set and the rider's
/// `+0x100` finish flag is zeroed.
pub(crate) fn rider_dnf(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
    c.g[A1][0] = 1;
    let points = r32(c, V0);
    c.sw(points, 0);
    c.g[A0][0] = 0x53_0000;
    let manager = r32(c, V1);
    let one = c.g[A1][0];
    c.sw(manager.wrapping_add(0x70), one);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x66a8));
    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
    c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x84));
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xc));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x28));
    let rider = r32(c, V1);
    c.sw(rider.wrapping_add(0x100), 0);
    let finished = r32(c, A0);
    let one = c.g[A1][0];
    c.sw(finished, one);
}

/// The not-DNF branch: below the point target the points are kept; at or
/// above it the rider goes on to the tier ranking.
pub(crate) fn rider_ranked(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A2][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x9c));
    c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x7c));
    let below = (c.g[V0][0] as i64) < (c.g[V1][0] as i64);
    c.g[V0][0] = below as u64;
    c.g[V1][0] = sx(r32(c, S4).wrapping_add(0x6640));
    if below {
        c.cycles += 11;
        let rider_flag = r32(c, A0).wrapping_add(0x100);
        c.sw(rider_flag, 0);
        c.g[V1][0] = sx(r32(c, S2).wrapping_add(r32(c, V1)));
        c.g[V0][0] = 0x53_0000;
        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x9c));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x66a8));
        c.g[A0][0] = 1;
        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
        let points = r32(c, V1);
        let total = c.g[A1][0];
        c.sw(points, total);
        let heat = r32(c, A2).wrapping_add(0x70);
        let one = c.g[A0][0];
        c.sw(heat, one);
        let not_done = r32(c, V0);
        c.sw(not_done, 0);
        return Ok(());
    }
    rider_tiers(c)
}

/// The tier ranking: the two peak goals of the event times 100 are the tier
/// targets; the total at or above the second tier is rank 0.
pub(crate) fn rider_tiers(c: &mut Cpu) -> Flow {
    c.cycles += 29;
    c.g[A1][0] = c.lw(r32(c, A2).wrapping_add(0x8));
    c.g[A0][0] = c.g[S3][0];
    c.g[A2][0] = 2;
    c.g[S1][0] = 0x64;
    call_ptr(c, 0x0014_55d0, 0x0023_c498)?; // getPeakGoal(race, event, 2)
    c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.mult(0, 16, 2, 17); // s0 = tier 2 target
    c.g[A0][0] = c.g[S3][0];
    c.g[A2][0] = 1;
    c.g[A1][0] = c.lw(r32(c, V1).wrapping_add(0x8));
    call_ptr(c, 0x0014_55d0, 0x0023_c4b0)?; // getPeakGoal(race, event, 1)
    c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
    c.mult(0, 8, 2, 17); // t0 = tier 1 target
    c.g[A2][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[A3][0] = 1;
    c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x84));
    c.g[A0][0] = sx(r32(c, S4).wrapping_add(0x6640));
    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xc));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x28));
    let rider_flag = r32(c, V1).wrapping_add(0x100);
    let one = c.g[A3][0];
    c.sw(rider_flag, one);
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x9c));
    let points = r32(c, A0);
    let total = c.g[V0][0];
    c.sw(points, total);
    let done = r32(c, A2).wrapping_add(0x84);
    c.sw(done, one);
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x9c));
    c.g[S0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
    let below_t2 = c.g[S0][0] != 0;
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T0][0] as i64)) as u64;
    if below_t2 {
        rider_rank_low(c);
        return Ok(());
    }
    c.cycles += 5;
    c.g[V0][0] = 0x53_0000;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6730));
    c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
    let rank = r32(c, V0);
    c.sw(rank, 0);
    Ok(())
}

/// Rank 1 or 2 (below the second tier): 2 when the total is also below the
/// first tier (`v0` from the compare), else 1 (`a3`).
pub(crate) fn rider_rank_low(c: &mut Cpu) {
    c.cycles += 2;
    let below_t1 = c.g[V0][0] != 0;
    c.g[V0][0] = 0x53_0000;
    if below_t1 {
        c.cycles += 4;
        c.g[V1][0] = 2;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6730));
        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
        let rank = r32(c, V0);
        let two = c.g[V1][0];
        c.sw(rank, two);
    } else {
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6730));
        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
        let rank = r32(c, V0);
        let one = c.g[A3][0];
        c.sw(rank, one);
    }
}

/// `cGMDebugHandler_init(self)` (0x23c618, GMHandlerUnused_Reset): the debug
/// game mode's reset. One heat (the last), the AI count from the new race,
/// the ten seats filled from a shuffled list of the riders with the human's
/// character kept in the human's seat, the cleared flags, and the handler's
/// virtual `cleanRiderRanksPoints` at the end.
pub fn gm_handler_unused_reset(c: &mut Cpu) {
    guest(c, unused_reset)
}

pub(crate) fn unused_reset(c: &mut Cpu) -> Flow {
    c.cycles += 34;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S2, 0x40);
    frame.save(c, S0, 0x60);
    frame.save(c, S1, 0x50);
    frame.save_ra(c, 0x30);
    c.g[S2][0] = c.g[A0][0];
    call_ptr(c, 0x0014_dc80, 0x0023_c634)?; // cBE_getBE()
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0023_c640)?; // cBE_getInterface_Fv(be, 0)
    call_ptr(c, 0x0014_dc80, 0x0023_c648)?; // cBE_getBE() again
    c.g[A1][0] = 1;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dd58, 0x0023_c654)?; // cBE_getInterface_Fv(be, 1): the player
    c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[A1][0] = 1;
    c.g[S0][0] = c.g[V0][0];
    c.g[A2][0] = 0x53_0000;
    let manager = r32(c, V1);
    let one = c.g[A1][0];
    c.sw(manager.wrapping_add(0x98), one);
    c.g[A0][0] = c.g[S0][0];
    c.sw(manager, one);
    c.g[A1][0] = 0;
    c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x5c04));
    c.sw(manager.wrapping_add(0x14), 0);
    let ai = c.g[V0][0];
    c.sw(manager.wrapping_add(0x10), ai);
    call_ptr(c, 0x0014_6e98, 0x0023_c684)?; // getHumanRider(player, 0)
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x0014_7398, 0x0023_c694)?; // getRiderCharID(player, human)
    c.g[S0][0] = c.g[V0][0];

    // The rider ids 9..0 into the stack array (arr[i] = i).
    c.g[V1][0] = 9;
    c.g[V0][0] = sx(sp(c).wrapping_add(0x24));
    loop {
        c.cycles += 7;
        let at = r32(c, V0);
        let id = c.g[V1][0];
        c.sw(at, id);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0023_c6a0);
        }
    }
    c.cycles += 7;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = 0xa;
    call_ptr(c, 0x0023_c770, 0x0023_c6c8)?; // GameRand_Shuffle(arr, 10)

    // Seat the riders: iCharacters[j] from the shuffled list, skipping the
    // human's character wherever it comes up.
    c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[A1][0] = 0;
    c.g[V1][0] = c.g[SP][0];
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x18));
    loop {
        c.cycles += 2;
        if c.g[A1][0] == c.g[S1][0] {
            let seat = r32(c, A0);
            let ch = c.g[S0][0];
            c.sw(seat, ch);
        } else {
            c.cycles += 3;
            let src = r32(c, V1);
            c.g[V0][0] = c.lw(src);
            if c.g[V0][0] != c.g[S0][0] {
                let seat = r32(c, A0);
                let id = c.g[V0][0];
                c.sw(seat, id);
            } else {
                c.cycles += 3;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                c.g[V0][0] = c.lw(r32(c, V1));
                let seat = r32(c, A0);
                let id = c.g[V0][0];
                c.sw(seat, id);
            }
            c.cycles += 1;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
        }
        c.cycles += 4;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        c.g[V0][0] = ((c.g[A1][0] as i64) < 10) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0023_c6d8);
        }
    }

    c.cycles += 25;
    c.g[V0][0] = c.lw(r32(c, S2));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x20));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x24));
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
    call_ptr(c, method, 0x0023_c720)?; // self->cleanRiderRanksPoints()
    c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x480));
    c.g[A0][0] = 1;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore_ra(c, 0x30);
    let gm = r32(c, V0);
    let neg = c.g[V1][0];
    c.sw(gm.wrapping_add(0x6c), neg);
    let one = c.g[A0][0];
    c.sw(gm.wrapping_add(0x74), one);
    c.sw(gm.wrapping_add(0x94), 0);
    c.sw(gm.wrapping_add(0x70), one);
    for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x90] {
        c.sw(gm.wrapping_add(off), 0);
    }
    frame.close(c);
    Ok(())
}

/// `void GMHandler0_PickOpponents(void)` (0x239938): build the list of characters 0..9 without the human rider's and the rival's, shuffle it, and fill the game mode manager's character and cheat-character tables (human, rival, then the shuffled rest); on a game path the rival slot takes the last shuffled one.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn gm_handler0_pick_opponents(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.g[RA][0] = sx(0x0023995c);
        c.g[S3][0] = 0u64;
        call_ptr(c, 0x0014dc80, 0x0023995c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00239968);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014dd58, 0x00239968)?;
        c.g[RA][0] = sx(0x00239970);
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0014dc80, 0x00239970)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023997c);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0014dd58, 0x0023997c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0023998c);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x00146e98, 0x0023998c)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0023999c);
        c.g[A1][0] = c.g[S4][0];
        call_ptr(c, 0x00147398, 0x0023999c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x002399a8);
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, 0x00145750, 0x002399a8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 0u64;
        c.g[A0][0] = c.g[SP][0];
        'l_2399b8: loop {
            'b_2399d8: {
                c.cycles += 2;
                if c.g[V1][0] == c.g[S1][0] {
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    break 'b_2399d8;
                } else {
                    c.cycles += 2;
                    if c.g[V1][0] == c.g[S0][0] {
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        break 'b_2399d8;
                    } else {
                        c.cycles += 4;
                        c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        break 'b_2399d8;
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10i64) as u64;
            let t = c.g[V0][0] != 0u64;

            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002399b8);
                }
                continue 'l_2399b8;
            } else {
                'b_239a54: {
                    c.cycles += 14;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x002399f0);
                    c.g[A1][0] = c.g[S3][0];
                    Callee {
                        addr: 0x0023c770,
                        f: ee::s06::GameRand_Shuffle,
                    }
                    .call(c, 0x002399f0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00239a04);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[S1][0]);
                    call_ptr(c, 0x00147410, 0x00239a04)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[S0][0]);
                    let t = (c.g[S3][0] as i64) <= 0;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x44), 0u64);
                    if t {
                        break 'b_239a54;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x48));
                        c.g[A2][0] = c.g[SP][0];
                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                        'l_239a28: loop {
                            c.cycles += 7;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            c.sw(c.g[A0][0] as u32, 0u64);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                            c.sw((c.g[A0][0] as u32).wrapping_sub(0x28), c.g[V1][0]);
                            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            if t {
                                break 'b_239a54;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = ((c.g[V0][0] as i64) < 10i64) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00239a28);
                                    }
                                    continue 'l_239a28;
                                } else {
                                    break 'b_239a54;
                                }
                            }
                        }
                    }
                }
                'b_239a7c: {
                    c.cycles += 5;
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    if t {
                        break 'b_239a7c;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                        break 'b_239a7c;
                    }
                }
                c.cycles += 8;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void GMHandler1_OnRiderFinished(self, idx)` (0x23a760, `cGMRaceHandler::raceFinish`): record the finish time (or 6000s for a rider that did not finish) in the per-heat table, and once the race and the rider are finished estimate the others' times, sort them, decide the next heat from the ranking (the heat unlock calls, the boost bar of the player's rider by place) and lock the results after heat 3.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn gm_handler1_on_rider_finished(c: &mut Cpu) {
    guest(c, |c| {
        'b_23a8b4: {
            'b_23a82c: {
                c.cycles += 37;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[S7][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x0023a79c);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
                call_ptr(c, 0x0014dc80, 0x0023a79c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0023a7a8);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x0023a7a8)?;
                c.g[RA][0] = sx(0x0023a7b0);
                c.g[FP][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x0023a7b0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0023a7bc);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x0023a7bc)?;
                c.g[A0][0] = c.g[FP][0];
                c.g[RA][0] = sx(0x0023a7c8);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00146e98, 0x0023a7c8)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                c.g[S0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[RA][0] = sx(0x0023a7ec);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0012a250, 0x0023a7ec)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_23a82c;
                } else {
                    c.cycles += 14;
                    c.g[A0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[S6][0] = c.g[A0][0];
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0023a824);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x0023a824)?;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_23a8b4;
                    } else {
                        break 'b_23a82c;
                    }
                }
            }
            'b_23a890: {
                c.cycles += 12;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[S6][0] = c.g[A0][0];
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x480));
                let t = c.g[A0][0] == 0u64;
                c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x478));
                if t {
                    break 'b_23a890;
                } else {
                    c.cycles += 13;
                    c.g[V1][0] = 0x28;
                    c.g[A0][0] = 0x50000;
                    c.mult(0, 2, 17, 3);
                    c.g[A0][0] = c.g[A0][0] | 0x7e40;
                    c.g[A1][0] = 0x1;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    c.g[V0][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66a8));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                    if t {
                        break 'b_23a8b4;
                    } else {
                        break 'b_23a890;
                    }
                }
            }
            c.cycles += 9;
            c.g[V0][0] = 0x28;
            c.g[V1][0] = 0x530000;
            c.mult(0, 4, 17, 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66a8));
            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S6][0] as u32));
            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
            c.sw(c.g[V1][0] as u32, 0u64);
            break 'b_23a8b4;
        }
        'b_23abdc: {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0023a8bc);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0012a250, 0x0023a8bc)?;
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_23abdc;
            } else {
                c.cycles += 12;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0023a8ec);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, t, 0x0023a8ec)?;
                let t = c.g[V0][0] == 0u64;
                c.g[S3][0] = sx((c.g[S3][0] as u32) << 2);
                if t {
                    break 'b_23abdc;
                } else {
                    'b_23a994: {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                        c.sw(c.g[SP][0] as u32, c.g[S3][0]);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[S4][0] = 0u64;
                        if t {
                            break 'b_23a994;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = 0x28;
                            c.g[V1][0] = 0x530000;
                            c.mult(0, 17, 17, 2);
                            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x58));
                            c.mtc1(0, 20);
                            c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x28));
                            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            'l_23a930: loop {
                                'b_23a94c: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                                    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x470));
                                    c.c_le_s(20, 0);
                                    let t = c.fcc();
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_23a94c;
                                    } else {
                                        c.cycles += 1;
                                        c.g[V0][0] = 0u64;
                                        break 'b_23a94c;
                                    }
                                }
                                'b_23a960: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x58));
                                    if t {
                                        break 'b_23a960;
                                    } else {
                                        c.cycles += 3;
                                        c.g[RA][0] = sx(0x0023a95c);
                                        call_ptr(c, 0x00122d78, 0x0023a95c)?;
                                        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                                        break 'b_23a960;
                                    }
                                }
                                c.cycles += 13;
                                c.g[V0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                                c.sw(c.g[S5][0] as u32, c.g[V1][0]);
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0023a930);
                                    }
                                    continue 'l_23a930;
                                } else {
                                    break 'b_23a994;
                                }
                            }
                        }
                    }
                    'b_23ab7c: {
                        'b_23ab64: {
                            'b_23ab60: {
                                'b_23aad0: {
                                    c.cycles += 14;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                    c.g[RA][0] = sx(0x0023a9a0);
                                    c.g[A1][0] = 0xa;
                                    Callee {
                                        addr: 0x00238bf8,
                                        f: ee::s06::GameModeMan_SortByTime,
                                    }
                                    .call(c, 0x0023a9a0)?;
                                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                    c.g[T0][0] = 0x530000;
                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x6730));
                                    c.g[A3][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x70));
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = 0x1;
                                    if t {
                                        break 'b_23aad0;
                                    } else {
                                        'b_23aa08: {
                                            c.cycles += 13;
                                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[T0][0] = 0x3;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            c.sw(
                                                (c.g[A2][0] as u32).wrapping_add(0x70),
                                                c.g[V0][0],
                                            );
                                            c.g[V1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S6][0] as u32));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x100),
                                                c.g[A1][0],
                                            );
                                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                            let t = c.g[V0][0] != c.g[T0][0];
                                            c.g[A0][0] = 0x530000;
                                            if t {
                                                break 'b_23aa08;
                                            } else {
                                                c.cycles += 2;
                                                c.sw(
                                                    (c.g[A2][0] as u32).wrapping_add(0x84),
                                                    c.g[A1][0],
                                                );
                                                c.sw(
                                                    (c.g[A2][0] as u32).wrapping_add(0x70),
                                                    c.g[T0][0],
                                                );
                                                break 'b_23aa08;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                                        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                                        let t = c.g[V1][0] != 0u64;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                        if t {
                                            break 'b_23ab64;
                                        } else {
                                            'b_23aab8: {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V1][0] = 0x3;
                                                if t {
                                                    break 'b_23aab8;
                                                } else {
                                                    'b_23aab4: {
                                                        'b_23aaa8: {
                                                            'b_23aa54: {
                                                                c.cycles += 5;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x480));
                                                                c.g[A2][0] =
                                                                    c.lw(c.g[V0][0] as u32);
                                                                c.g[V1][0] = ((c.g[A2][0] as i64)
                                                                    < 2i64)
                                                                    as u64;
                                                                let t = c.g[V1][0] == 0u64;
                                                                c.g[V1][0] = 0x530000;
                                                                if t {
                                                                    break 'b_23aa54;
                                                                } else {
                                                                    c.cycles += 7;
                                                                    c.g[A0][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x6640));
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x6640));
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_sub(
                                                                            c.g[A1][0] as u32,
                                                                        ));
                                                                    c.g[V0][0] = (c.g[V0][0]
                                                                        < 0x259u64)
                                                                        as u64;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = c.g[FP][0];
                                                                    if t {
                                                                        break 'b_23aaa8;
                                                                    } else {
                                                                        break 'b_23aa54;
                                                                    }
                                                                }
                                                            }
                                                            'b_23aa7c: {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = 0x2;
                                                                let t = c.g[A2][0] != c.g[V0][0];
                                                                c.g[V1][0] = 0x530000;
                                                                if t {
                                                                    break 'b_23aa7c;
                                                                } else {
                                                                    c.cycles += 7;
                                                                    c.g[A0][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x6640));
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x6640));
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_sub(
                                                                            c.g[A1][0] as u32,
                                                                        ));
                                                                    c.g[V0][0] = (c.g[V0][0]
                                                                        < 0x12du64)
                                                                        as u64;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = c.g[FP][0];
                                                                    if t {
                                                                        break 'b_23aaa8;
                                                                    } else {
                                                                        break 'b_23aa7c;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A2][0] != c.g[T0][0];
                                                            c.g[V1][0] = 0x530000;
                                                            if t {
                                                                break 'b_23ab7c;
                                                            } else {
                                                                c.cycles += 8;
                                                                c.g[V1][0] = 0x530000;
                                                                c.g[A0][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x6640));
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x6640));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_sub(
                                                                        c.g[A1][0] as u32,
                                                                    ));
                                                                c.g[V0][0] =
                                                                    (c.g[V0][0] < 0xb5u64) as u64;
                                                                let t = c.g[V0][0] != 0u64;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x480));
                                                                if t {
                                                                    break 'b_23aab4;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.g[A0][0] = c.g[FP][0];
                                                                    break 'b_23aaa8;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.g[RA][0] = sx(0x0023aab0);
                                                        c.g[A1][0] = 0x1;
                                                        call_ptr(c, 0x00147d20, 0x0023aab0)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x480));
                                                        break 'b_23aab4;
                                                    }
                                                    c.cycles += 1;
                                                    c.g[V1][0] = 0x3;
                                                    break 'b_23aab8;
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                            let t = c.g[V0][0] != c.g[V1][0];
                                            c.g[V1][0] = 0x530000;
                                            if t {
                                                break 'b_23ab7c;
                                            } else {
                                                c.cycles += 3;
                                                c.sw((c.g[A0][0] as u32).wrapping_add(0x74), 0u64);
                                                let t = true;
                                                c.sw((c.g[A0][0] as u32).wrapping_add(0x70), 0u64);
                                                if t {
                                                    break 'b_23ab60;
                                                } else {
                                                    break 'b_23aad0;
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 16;
                                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                c.g[V0][0] = 0x1;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[T0][0] = 0x530000;
                                c.sw((c.g[A2][0] as u32).wrapping_add(0x70), c.g[V1][0]);
                                c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5bc8));
                                c.sw((c.g[S7][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                                c.sw((c.g[S7][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x100), 0u64);
                                c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x49));
                                if c.g[V0][0] != 0u64 {
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                                    break 'b_23ab64;
                                } else {
                                    'b_23ab58: {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A0][0] = c.g[FP][0];
                                            break 'b_23ab58;
                                        } else {
                                            'b_23ab54: {
                                                'b_23ab3c: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x2;
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[V0][0] = 0x3;
                                                    if t {
                                                        break 'b_23ab3c;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                                        c.g[V0][0] =
                                                            ((c.g[V0][0] as i64) < 4i64) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x3;
                                                        if t {
                                                            break 'b_23ab54;
                                                        } else {
                                                            break 'b_23ab3c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != c.g[V0][0];
                                                c.g[V1][0] = 0x530000;
                                                if t {
                                                    break 'b_23ab7c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                                                    c.g[V0][0] = 0x5;
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x480)
                                                        );
                                                    if t {
                                                        break 'b_23ab64;
                                                    } else {
                                                        break 'b_23ab54;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A0][0] = c.g[FP][0];
                                            break 'b_23ab58;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[RA][0] = sx(0x0023ab60);
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x00147e18, 0x0023ab60)?;
                                    break 'b_23ab60;
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                            break 'b_23ab64;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = 0x3;
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[V1][0] = 0x530000;
                        if t {
                            break 'b_23ab7c;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = 0x1;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
                            break 'b_23ab7c;
                        }
                    }
                    'b_23abc8: {
                        'b_23abb0: {
                            'b_23aba0: {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6730));
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                let t = c.g[V1][0] != 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_23aba0;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5154));
                                    let t = true;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_23abb0;
                                    } else {
                                        break 'b_23aba0;
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            if t {
                                break 'b_23abc8;
                            } else {
                                c.cycles += 2;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5150));
                                break 'b_23abb0;
                            }
                        }
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                        let t = true;
                        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2f8));
                        if t {
                            break 'b_23abdc;
                        } else {
                            break 'b_23abc8;
                        }
                    }
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x2f8), 0u64);
                    break 'b_23abdc;
                }
            }
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

/// `GMHandler8_PickOpponents` (0x0023b0a8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn gm_handler8_pick_opponents(c: &mut Cpu) {
    if c.pc != 0x0023b0a8 {
        return ee::s06::f_0023b0a8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0023b0a8;
        loop {
            match pc {
                0x0023b0a8 => {
                    c.cycles += 4;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[A2][0] = 0x9;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x24));
                    pc = 0x0023b0b8;
                    continue;
                }
                0x0023b0b8 => {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[A2][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0023b0b8) {
                            return Ok(());
                        }
                        pc = 0x0023b0b8;
                        continue;
                    }
                    pc = 0x0023b0d4;
                    continue;
                }
                0x0023b0d4 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0023b0e0);
                    c.g[A1][0] = 0xa;
                    Callee {
                        addr: 0x0023c770,
                        f: ee::s06::GameRand_Shuffle,
                    }
                    .call(c, 0x0023b0e0)?;
                    c.g[T1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
                    c.g[T0][0] = 0x2;
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.g[T1][0];
                    c.g[A3][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    if t {
                        pc = 0x0023b12c;
                        continue;
                    }
                    pc = 0x0023b104;
                    continue;
                }
                0x0023b104 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));

                    if c.g[A1][0] == c.g[V0][0] {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        pc = 0x0023b130;
                        continue;
                    }
                    pc = 0x0023b110;
                    continue;
                }
                0x0023b110 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                    pc = 0x0023b118;
                    continue;
                }
                0x0023b118 => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[A1][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x40), 0u64);
                    pc = 0x0023b12c;
                    continue;
                }
                0x0023b12c => {
                    c.cycles += 1;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    pc = 0x0023b130;
                    continue;
                }
                0x0023b130 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 10i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x0023b164;
                        continue;
                    }
                    pc = 0x0023b13c;
                    continue;
                }
                0x0023b13c => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[T1][0];
                    c.g[A1][0] = c.lw(c.g[A3][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    if t {
                        if back_edge_unwinds(c, 0x0023b12c) {
                            return Ok(());
                        }
                        pc = 0x0023b12c;
                        continue;
                    }
                    pc = 0x0023b150;
                    continue;
                }
                0x0023b150 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[T0][0] as i64) < 10i64) as u64;
                    if t {
                        if back_edge_unwinds(c, 0x0023b12c) {
                            return Ok(());
                        }
                        pc = 0x0023b12c;
                        continue;
                    }
                    pc = 0x0023b15c;
                    continue;
                }
                0x0023b15c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
                    if t {
                        if back_edge_unwinds(c, 0x0023b118) {
                            return Ok(());
                        }
                        pc = 0x0023b118;
                        continue;
                    }
                    pc = 0x0023b164;
                    continue;
                }
                0x0023b164 => {
                    c.cycles += 3;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0023b0a8, 0x0023b170);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::game::main::game::tests::fake_get_be;
    use crate::game::main::game::tests::PREV_GET_BE;
    use crate::game::main::game::tests::{fake_be_get_interface, PREV_BE_GET_INTERFACE};

    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::game::ai::control::handplantcontrol::tests::oc425_install;

    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base_2;
    pub(crate) use crate::game::bx::bxstringctor::tests::slot_2;
    pub(crate) use crate::game::render::particle::tests::gp_at;

    pub(crate) use crate::game::bx::bxstringctor::tests::W;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_9 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;

    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_6 as WORLD;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_5;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all_2;

    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2;

    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base_14 as base_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::check3;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::LEAF;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD_4 as WORLD_3;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    pub(crate) const SEEDS: u32 = 200;

    // ------------------------------------------------------------------
    // GM save/load handlers.

    pub(crate) fn gm_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32) {
        let mut r = machine(c, seed, salt, 1);
        let h = DATA + 0x500;
        let vt = DATA + 0x600;
        c.bus.write32(h, vt);
        c.bus.write16(vt + slot, ((r.next_u32() % 0x40) * 4) as u16);
        c.bus.write32(vt + slot + 4, FAKE);
        c.g[A0][0] = sx(DATA + 0x300);
        c.g[A1][0] = sx(h);
    }

    #[test]
    fn gm_handler0_save_state_matches() {
        check(
            ssx3_ee::s06::f_00239d18__generated,
            gm_handler0_save_state,
            SEEDS,
            &|c, seed| {
                gm_setup(c, seed, 0x39d18, 0x08);
            },
        );
    }

    #[test]
    fn gm_handler0_load_state_matches() {
        check(
            ssx3_ee::s06::f_00239d50__generated,
            gm_handler0_load_state,
            SEEDS,
            &|c, seed| {
                gm_setup(c, seed, 0x39d50, 0x10);
            },
        );
    }

    #[test]
    fn gm_handler1_save_state_matches() {
        check(
            ssx3_ee::s06::f_0023ac18__generated,
            gm_handler1_save_state,
            SEEDS,
            &|c, seed| {
                gm_setup(c, seed, 0x3ac18, 0x08);
            },
        );
    }

    #[test]
    fn gm_handler1_load_state_matches() {
        check(
            ssx3_ee::s06::f_0023ac50__generated,
            gm_handler1_load_state,
            SEEDS,
            &|c, seed| {
                gm_setup(c, seed, 0x3ac50, 0x10);
            },
        );
    }

    #[test]
    fn gm_handler8_save_state_matches() {
        check(
            ssx3_ee::s06::f_0023b038__generated,
            gm_handler8_save_state,
            SEEDS,
            &|c, seed| {
                gm_setup(c, seed, 0x3b038, 0x08);
            },
        );
    }

    #[test]
    fn gm_handler8_load_state_matches() {
        check(
            ssx3_ee::s06::f_0023b070__generated,
            gm_handler8_load_state,
            SEEDS,
            &|c, seed| {
                gm_setup(c, seed, 0x3b070, 0x10);
            },
        );
    }

    pub(crate) fn handler_test(generated: GuestFn, hand: GuestFn, salt: u64, slot: u32) {
        check(generated, hand, SEEDS, &move |c, seed| {
            let mut r = machine(c, seed, salt, 0);
            let obj = DATA + 0x100;
            let vt = vtable(c, &mut r, &[slot]);
            let index = seed % 4;
            c.bus.write32(0x0053_6668 + 4 * index, obj);
            c.bus.write32(obj, vt);
            c.bus.write32(obj + 4, r.next_u32());
            c.g[A0][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA + 0x204, index);
        });
    }

    #[test]
    fn game_mode_man_handler_v06_matches() {
        handler_test(
            ee::s06::f_00238510__generated,
            game_mode_man_handler_v06,
            0x238510,
            0x30,
        );
    }

    #[test]
    fn game_mode_man_handler_v07_matches() {
        handler_test(
            ee::s06::f_00238550__generated,
            game_mode_man_handler_v07,
            0x238550,
            0x38,
        );
    }

    #[test]
    fn game_mode_man_handler_v09_matches() {
        handler_test(
            ee::s06::f_00238590__generated,
            game_mode_man_handler_v09,
            0x238590,
            0x48,
        );
    }

    /// The world fixture: `gp-0x848 -> world -> +0x84 -> manager -> +0xc ->
    /// race`, `race+8` the race's frame limit.
    pub(crate) fn world(c: &mut Cpu, frame_limit: u32) {
        put(c, AUX + 0x100 + 0x84, AUX + 0x200);
        put(c, AUX + 0x200 + 0xc, AUX + 0x300);
        put(c, AUX + 0x300 + 8, frame_limit);
        gpf(c, -0x848, AUX + 0x100);
    }

    #[test]
    fn gm_handler0_v09_matches() {
        check_lohi(
            ee::s06::f_00239ce0__generated,
            gm_handler0_v09,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 45);
                c.g[A0][0] = AUX as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                put(c, AUX + 0x600, 3); // the row count
                gpf(c, -0x480, AUX + 0x600);
                for w in 0..(3u32 * 0x28 / 4) {
                    put(c, AUX + 4 * w, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn gm_handler9_v06_matches() {
        check_lohi(
            ee::s06::f_0023a070__generated,
            gm_handler9_v06,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 46);
                world(c, r.next_u32() % 400);
                c.g[A0][0] = W as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = ((seed % 5) * 3) as u64;
                c.bus.write16(
                    W + 4,
                    if seed % 3 == 0 {
                        5
                    } else {
                        (r.next_u32() % 20) as u16
                    },
                );
                c.bus.write16(W + 6, (r.next_u32() % 20) as u16);
                gpf(c, -0x480, AUX + 0x600);
                put(c, AUX + 0x600 + 8, if seed % 2 == 0 { 1 } else { 0 });
                put(c, AUX + 0x600 + 0x78, r.next_u32() % 400);
            },
        );
    }

    pub(crate) const SEEDS_2: u32 = 24;

    pub(crate) fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    pub(crate) fn h(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }

    /// The zeroed BE-library singleton at `gp-0x1f04` (else the FE helpers walk
    /// into the lazy disc load).
    pub(crate) fn be_library(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }

    /// `{delta, fn}` at `vt + off`.
    pub(crate) fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16, f: u32) {
        h(c, vt + off, delta as u16);
        w(c, vt + off + 4, f);
    }

    #[test]
    fn game_mode_man_destroy_matches() {
        check(
            ee::s06::f_002380e8__generated,
            game_mode_man_destroy,
            SEEDS_2,
            &|c, seed| {
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
                w(
                    c,
                    GAME_GP.wrapping_sub(0x484),
                    if seed % 2 == 0 { 0 } else { DATA + 0x4000 },
                );
            },
        );
    }

    #[test]
    fn gm_handler_clear_standings_matches() {
        check(
            ee::s06::f_00238d30__generated,
            gm_handler_clear_standings,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x238d30, 5);
                be_library(c);
                for i in 0..0x40u32 {
                    w(c, 0x0053_6640 + 4 * i, seed ^ i);
                }
            },
        );
    }

    #[test]
    fn gm_handler_clear_standings2_matches() {
        check(
            ee::s06::f_00238da8__generated,
            gm_handler_clear_standings2,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x238da8, 5);
                be_library(c);
                for i in 0..0x40u32 {
                    w(c, 0x0053_6640 + 4 * i, seed ^ i);
                }
            },
        );
    }

    #[test]
    fn game_mode_man_sort_by_score_matches() {
        check(
            ee::s06::f_00238b70__generated,
            game_mode_man_sort_by_score,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x238b70, 2);
                c.g[A1][0] = (1 + seed % 5) as u64; // the entry count handed to qsort
                                                    // The score table lives in RAM at fixed addresses; give it values.
                for i in 0..10u32 {
                    c.bus.write32(0x0053_6708 + 4 * i, (seed * 7 + i) % 13);
                    c.bus.write32(0x0053_6730 + 4 * i, (seed * 5 + i) % 11);
                }
            },
        );
    }

    #[test]
    fn game_mode_man_sort_by_time_matches() {
        check(
            ee::s06::f_00238bf8__generated,
            game_mode_man_sort_by_time,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x238bf8, 2);
                c.g[A1][0] = (1 + seed % 5) as u64; // the entry count handed to qsort
                for i in 0..10u32 {
                    c.bus.write32(0x0053_6708 + 4 * i, (seed * 3 + i) % 17);
                    c.bus.write32(0x0053_6730 + 4 * i, (seed * 11 + i) % 19);
                }
            },
        );
    }

    #[test]
    fn game_rand_shuffle_matches() {
        check_lohi(
            ee::s06::f_0023c770__generated,
            game_rand_shuffle,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x3c770, 2);
                let array = DATA;
                for i in 0..25 {
                    w(c, array + 4 * i, (seed + i) as u32);
                }
                c.g[A0][0] = array as u64;
                c.g[A1][0] = 25;
            },
        );
    }

    #[test]
    fn gm_handler_start_matches() {
        check(
            ee::s06::f_00238c80__generated,
            gm_handler_start,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x238c);
                let gm = DATA + 0x2000;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(gp_at(-0x480), gm);
                c.bus.write32(gm, DATA + 0x3000);
                slot_2(c, DATA + 0x3000, 0x28, 1);
                fill(c, &mut r, gm, 0x100);
            },
        );
    }

    #[test]
    fn gm_handler7_v06_matches() {
        check(
            ee::s06::f_0023c560__generated,
            gm_handler7_v06,
            6,
            &|c, seed| {
                let _ = base_3(c, seed, 0x0023_c560, 2);
                c.g[A0][0] = OBJ as u64;
                c.bus.write16(OBJ + 8, (seed as u16) & 0x1f);
                c.bus.write16(OBJ + 0xa, (seed as u16) & 7);
                let game = WORLD + 0x1000;
                let race = WORLD + 0x2000;
                w(c, race + 0x28, OBJ2);
                w(c, OBJ2 + 0x790, OBJ3);
                w(c, OBJ3 + 0x198, (seed % 1000) as u32);
                w(c, game + 0x100, 0);
            },
        );
    }

    #[test]
    fn gm_handler4_v06_matches() {
        check_all(
            ee::s06::f_0023b5f8__generated,
            gm_handler4_v06,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23b5, 3);
                game_globals(c);
                be_library_2(c, &mut r);
                c.bus.write32(GAME_GP - 0x514c, (1.0f32 / 60.0).to_bits());
                c.bus.write32(0x0053_0000 + 0x5c12, (seed % 3) as u32);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write16(this + 0xa, seed as u16);
                c.bus.write16(this + 8, seed as u16);
                c.bus.write32(RMAN + 0x28, DATA + 0x150000);
                c.bus.write32(DATA + 0x150000 + 0x790, DATA + 0x160000);
            },
        );
    }

    #[test]
    fn gm_handler9_reset_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = base_4(c, seed, 0x239d88);
            // The BE interfaces (else `cBE_getInterface` calls a null
            // pointer), the handler object with a vtable, and the global
            // state block.
            be_interfaces(c);
            let h = DATA;
            let vt = DATA + 0x100;
            c.bus.write32(h, vt);
            c.bus.write32(h + 4, r.next_u32());
            c.bus.write16(vt + 0x20, 0);
            c.bus.write32(vt + 0x24, LEAF);
            c.bus.write32(GAME_GP - 0x480, DATA + 0x800);
            c.bus.write32(DATA + 0x800 + 0x78, DEADLINE_UNSET);
            c.g[A0][0] = sx(h);
        };
        check3(
            ee::s06::f_00239d88__generated,
            gm_handler9_reset,
            40,
            &setup,
        );
        deadline_stored(gm_handler9_reset, &setup, 40);
    }

    pub(crate) const HANDLER: u32 = DATA + 0x1_0000;

    pub(crate) const RIDERS_3: u32 = DATA + 0x1_1000;

    #[test]
    fn gm_handler8_on_rider_finished_matches() {
        check_all_2(
            ee::s06::f_0023ae00__generated,
            gm_handler8_on_rider_finished,
            20,
            &|c, seed| {
                let _ = base_5(c, seed, 0x23ae_00, 14);
                c.g[A0][0] = sx(HANDLER);
                c.g[A1][0] = sx(seed % 2);
                // The race and its riders come from the world singleton chain:
                // keep the rider list short by leaving the count at_2 1.
                c.bus.write32(RACE + 0xc, RMAN_2);
                c.bus.write32(RMAN_2 + 0x7c, 1);
                for i in 0..2u32 {
                    c.bus
                        .write32(RIDERS_3 + 4 * i, DATA + 0x1_2000 + 0x1000 * i);
                    c.bus.write32(DATA + 0x1_3000 + 4 * i, 0);
                }
            },
        );
    }

    // ------------------------------------------------------------------
    // Game-mode resets and deadlines (tick rate).

    /// The game-mode manager the reset fixtures use.
    const GM: u32 = DATA + 0x800;
    /// The handler object and its vtable (slots 4 and 5 are leaves).
    const HANDLER_OBJ: u32 = DATA;

    /// What the reset fixtures leave in the deadline (manager +0x78), so a
    /// run that stored it can be told apart.
    const DEADLINE_UNSET: u32 = 0x7777_7777;

    /// The BE library's interfaces: `cBE_getInterface(be, n)` asks the
    /// current BE state (`D_005308A8[gp-0x1efc]`) through its vtable slot
    /// +0x8/+0xc. The test image has no state (a call through a null
    /// pointer), so the state here answers through a fake callee: a zeroed
    /// interface object. (The interface functions the resets call read
    /// globals, not the object.)
    fn be_interfaces(c: &mut Cpu) {
        const IFACE: u32 = DATA + 0x6_0000;
        const STATE: u32 = DATA + 0x6_1000;
        const VT: u32 = DATA + 0x6_1100;
        c.hooks.miss = fake_call;
        c.hooks.jump_out = crate::difftest::fakes::fake_jump_out;
        c.bus
            .write32(crate::difftest::fakes::LOG_UNWIND_AT, u32::MAX);
        for n in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * n, IFACE);
        }
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(0x0053_08a8, STATE);
        c.bus.write32(STATE + 4, VT);
        c.bus.write16(VT + 8, 0);
        c.bus.write32(VT + 0xc, FAKE);
    }

    /// At least a quarter of the `seeds` fixtures reach the deadline store
    /// (run through the replacement; the differential test proves the
    /// original does the same).
    fn deadline_stored(f: GuestFn, setup: &dyn Fn(&mut Cpu, u32), seeds: u32) {
        let stored = (0..seeds)
            .filter(|&seed| {
                let mut c = crate::difftest::fresh();
                setup(&mut c, seed);
                f(&mut c);
                c.bus.read32(GM + 0x78) != DEADLINE_UNSET
            })
            .count() as u32;
        assert!(
            stored * 4 >= seeds,
            "only {stored} of {seeds} seeds stored the deadline"
        );
    }

    /// A scrambled machine with the game image, the BE interfaces
    /// ([`be_interfaces`]), the world (game -> AI with its update counter)
    /// and the manager at `gp-0x480` (deadline [`DEADLINE_UNSET`]); the
    /// handler's vtable slots 4 and 5 are a leaf.
    fn gm_reset_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = base_4(c, seed, salt);
        be_interfaces(c);
        let vt = DATA + 0x100;
        c.bus.write32(HANDLER_OBJ, vt);
        for slot in [0x20u32, 0x28] {
            c.bus.write16(vt + slot, 0);
            c.bus.write32(vt + slot + 4, LEAF);
        }
        // The handler's time tables (+0x4.., three rows of ten).
        for i in 0..0x20u32 {
            c.bus.write32(HANDLER_OBJ + 4 + 4 * i, r.next_u32() % 5000);
        }
        c.bus.write32(GAME_GP - 0x480, GM);
        for i in 0..0x28u32 {
            c.bus.write32(GM + 4 * i, 0);
        }
        c.bus.write32(GM + 0x78, DEADLINE_UNSET);
        // The rider character slots (+0x18..).
        for i in 0..10u32 {
            c.bus.write32(GM + 0x18 + 4 * i, r.next_u32() % 10);
        }
        let game = WORLD_3 + 0x1000;
        let ai = WORLD_3 + 0x2000;
        c.bus.write32(WORLD_3 + 0x84, game);
        c.bus.write32(game + 0xc, ai);
        c.bus.write32(ai + 0x8, r.next_u32() % 4000);
        c.g[A0][0] = sx(HANDLER_OBJ);
        r
    }

    fn gm0_reset_setup(c: &mut Cpu, seed: u32) {
        let _ = gm_reset_base(c, seed, 0x238e20);
        // Quick race (0x535c11) on some seeds.
        c.bus.write8(QUICK_RACE, if seed % 4 == 1 { 1 } else { 0 });
        c.bus.write32(GM + 0x8, [1, 2, 3][seed as usize % 3]);
        c.bus.write32(GM + 0x10, seed % 6);
        // The round to start: 0..3 and two out-of-range values.
        c.bus.write32(
            GM + 0x70,
            [0, 1, 2, 3, u32::MAX, 4][(seed / 3) as usize % 6],
        );
        c.bus.write32(GM + 0x84, (seed / 2) % 2);
    }

    #[test]
    fn gm_handler0_reset_matches() {
        check3(
            ee::s06::f_00238e20__generated,
            gm_handler0_reset,
            60,
            &gm0_reset_setup,
        );
        deadline_stored(gm_handler0_reset, &gm0_reset_setup, 60);
    }

    fn gm0_v06_setup(c: &mut Cpu, seed: u32) {
        let mut r = gm_reset_base(c, seed, 0x2398e8);
        c.bus.write32(GM + 0x8, if seed % 3 == 0 { 0 } else { 1 });
        let now = c.bus.read32(WORLD_3 + 0x2000 + 0x8);
        // The deadline before or after the counter.
        c.bus.write32(
            GM + 0x78,
            if seed % 2 == 0 {
                now + r.next_u32() % 900
            } else {
                now / 2
            },
        );
        c.g[A1][0] = (seed % 2) as u64;
        c.g[A2][0] = sx(r.next_u32() % 300);
    }

    #[test]
    fn gm_handler0_v06_matches() {
        check3(
            ee::s06::f_002398e8__generated,
            gm_handler0_v06,
            100,
            &gm0_v06_setup,
        );
    }

    fn gm4_reset_setup(c: &mut Cpu, seed: u32) {
        let _ = gm_reset_base(c, seed, 0x23b268);
        // The event mode (0x535c12): 6..8 have table entries.
        c.bus.write8(EVENT_MODE, [6, 7, 8, 9, 5][seed as usize % 5]);
        // The score lookup answers -1 (the profile database is closed).
        c.bus.write8(0x0053_05f9, 1);
    }

    #[test]
    fn gm_handler4_reset_matches() {
        check3(
            ee::s06::f_0023b268__generated,
            gm_handler4_reset,
            40,
            &gm4_reset_setup,
        );
        deadline_stored(gm_handler4_reset, &gm4_reset_setup, 40);
    }

    /// The rider [`gm4_finished_setup`] finishes (world -> game -> AI
    /// +0x28 + 4 * idx).
    const GM4_RIDER: u32 = DATA + 0x3_0000;
    /// What the fixture leaves in the rider's +0x100, the player's medal and
    /// out flag, so the stores can be told apart.
    const GM4_UNSET: u32 = 0x7777_7777;

    /// The event's medal time (seconds) for `mode` (6..8) and medal `level`
    /// (2 gold, 1 silver), from the image's table (`D_00440D18`).
    fn gm4_medal_seconds(c: &mut Cpu, mode: u32, level: u32) -> u32 {
        c.bus
            .read16(0x0044_0d18 + ((mode - 6) * 3 + level) * 20 + 4) as u32
    }

    /// Case `(seed / 4) % 5`: 0 DNF, 1 over the limit, 2 gold, 3 silver,
    /// 4 bronze; player `seed % 4`; event mode 6..8. The finish time and the
    /// handler's limit are in 60 Hz updates.
    fn gm4_finished_setup(c: &mut Cpu, seed: u32) {
        let mut r = gm_reset_base(c, seed, 0x23b468);
        let slot = seed % 4;
        let mode = 6 + (seed / 20) % 3;
        c.bus.write32(GM + 0x8, mode);
        let ai = WORLD_3 + 0x2000;
        c.bus.write32(ai + 0x28 + 4 * slot, GM4_RIDER);
        c.bus.write32(GM4_RIDER + 0x100, GM4_UNSET);
        c.bus.write32(MEDALS + 4 * slot, GM4_UNSET);
        c.bus.write32(OUT_FLAGS + 4 * slot, GM4_UNSET);
        let gold = gm4_medal_seconds(c, mode, 2) * 60;
        let silver = gm4_medal_seconds(c, mode, 1) * 60;
        let case = (seed / 4) % 5;
        let time = match case {
            1 => 1000 + r.next_u32() % 5000,
            2 => r.next_u32() % (gold + 1),
            3 => gold + 1 + r.next_u32() % silver.saturating_sub(gold).max(1),
            _ => silver + 1 + r.next_u32() % 600,
        };
        let limit = if case == 1 {
            time - 1 - r.next_u32() % 500
        } else {
            time + r.next_u32() % 1000
        };
        c.bus.write32(GM4_RIDER + 0x478, time);
        c.bus.write32(
            GM4_RIDER + 0x480,
            if case == 0 { r.next_u32() | 1 } else { 0 },
        );
        c.bus.write32(HANDLER_OBJ + 4, limit);
        c.g[A0][0] = sx(HANDLER_OBJ);
        c.g[A1][0] = slot as u64;
    }

    #[test]
    fn gm_handler4_on_rider_finished_matches() {
        check3(
            ee::s06::f_0023b468__generated,
            gm_handler4_on_rider_finished,
            60,
            &gm4_finished_setup,
        );
        // Most seeds reach the medal compare (rider +0x100 = 1), and every
        // medal is won.
        let mut medals = [0u32; 3];
        let mut compared = 0;
        for seed in 0..60 {
            let mut c = crate::difftest::fresh();
            gm4_finished_setup(&mut c, seed);
            gm_handler4_on_rider_finished(&mut c);
            if c.bus.read32(GM4_RIDER + 0x100) == 1 {
                compared += 1;
                medals[c.bus.read32(MEDALS + 4 * (seed % 4)) as usize] += 1;
            }
        }
        assert!(
            compared * 2 >= 60,
            "only {compared} of 60 seeds reached the medal compare"
        );
        assert!(medals.iter().all(|&n| n > 0), "medals won: {medals:?}");
    }

    /// The game rate `hz` in the app manager (or a test one).
    fn at_rate(c: &mut Cpu, hz: u32) {
        let app = c.bus.read32(crate::tick::APPMAN_PTR);
        if app == 0 {
            crate::tick::set_for_test(c, hz);
        } else {
            c.bus.write32(app + crate::tick::APPMAN_RATE, hz);
            c.bus
                .write32(app + crate::tick::APPMAN_STEP, crate::tick::step_bits(hz));
        }
    }

    /// The medal is the same at any rate for the same wall time: the finish
    /// time and the limit are `rate / 60` times the 60 Hz counts, the medal
    /// times (seconds) are scaled by the rate. Includes the exact gold and
    /// silver boundaries.
    #[test]
    fn gm_handler4_on_rider_finished_medal_follows_the_rate() {
        let medal = |seed: u32, hz: u32, time60: Option<u32>| {
            let mut c = crate::difftest::fresh();
            gm4_finished_setup(&mut c, seed);
            if let Some(t) = time60 {
                c.bus.write32(GM4_RIDER + 0x478, t);
                c.bus.write32(HANDLER_OBJ + 4, t + 100);
            }
            let k = hz / 60;
            for off in [GM4_RIDER + 0x478, HANDLER_OBJ + 4] {
                let v = c.bus.read32(off);
                c.bus.write32(off, v * k);
            }
            at_rate(&mut c, hz);
            gm_handler4_on_rider_finished(&mut c);
            assert_eq!(
                c.bus.read32(GM4_RIDER + 0x100),
                1,
                "seed {seed}, {hz} Hz: no medal compare"
            );
            c.bus.read32(MEDALS + 4 * (seed % 4))
        };
        let mut seen = [false; 3];
        // Cases 2..4 (gold, silver, bronze) on a few players and modes.
        for seed in [8u32, 13, 16, 29, 34, 39, 48, 53, 58] {
            let m60 = medal(seed, 60, None);
            seen[m60 as usize] = true;
            for hz in [120u32, 240] {
                assert_eq!(medal(seed, hz, None), m60, "seed {seed}, {hz} Hz");
            }
        }
        assert!(seen.iter().all(|&s| s), "medals at 60 Hz: {seen:?}");
        // On the boundaries: exactly the gold time is gold, one 60 Hz update later
        // silver; exactly the silver time silver, one later bronze.
        for seed in [8u32, 28, 48] {
            let mode = 6 + (seed / 20) % 3;
            let mut c = crate::difftest::fresh();
            gm4_finished_setup(&mut c, seed);
            let gold = gm4_medal_seconds(&mut c, mode, 2) * 60;
            let silver = gm4_medal_seconds(&mut c, mode, 1) * 60;
            for hz in [60u32, 120, 240] {
                assert_eq!(medal(seed, hz, Some(gold)), 0, "seed {seed}, {hz} Hz: gold");
                if silver > gold {
                    assert_eq!(
                        medal(seed, hz, Some(gold + 1)),
                        1,
                        "seed {seed}, {hz} Hz: past gold"
                    );
                    assert_eq!(
                        medal(seed, hz, Some(silver)),
                        1,
                        "seed {seed}, {hz} Hz: silver"
                    );
                }
                assert_eq!(
                    medal(seed, hz, Some(silver + 1)),
                    2,
                    "seed {seed}, {hz} Hz: past silver"
                );
            }
        }
    }

    fn gm6_reset_setup(c: &mut Cpu, seed: u32) {
        let _ = gm_reset_base(c, seed, 0x23bb98);
        // Two players (0x535c11 == 2) or one.
        c.bus.write8(QUICK_RACE, [2, 0, 1][seed as usize % 3]);
    }

    #[test]
    fn gm_handler6_reset_matches() {
        check3(
            ee::s06::f_0023bb98__generated,
            gm_handler6_reset,
            40,
            &gm6_reset_setup,
        );
        deadline_stored(gm_handler6_reset, &gm6_reset_setup, 40);
    }

    fn gm7_reset_setup(c: &mut Cpu, seed: u32) {
        let _ = gm_reset_base(c, seed, 0x23c0d0);
        c.bus
            .write8(EVENT_MODE, [9, 10, 11, 12, 8][seed as usize % 5]);
        c.bus.write8(0x0053_05f9, 1);
    }

    #[test]
    fn gm_handler7_reset_matches() {
        check3(
            ee::s06::f_0023c0d0__generated,
            gm_handler7_reset,
            40,
            &gm7_reset_setup,
        );
        deadline_stored(gm_handler7_reset, &gm7_reset_setup, 40);
    }

    /// The deadline (manager +0x78) one call of `f` leaves at game rate `hz`
    /// (with the manager's deadline before the call in `.1`).
    fn deadline_at(hz: u32, f: GuestFn, setup: &dyn Fn(&mut Cpu, u32), seed: u32) -> (i32, i32) {
        let mut c = crate::difftest::fresh();
        setup(&mut c, seed);
        // The rate lives in the app manager; the fixture may already have
        // an object there (the pointer is a gp global it also uses).
        let app = c.bus.read32(crate::tick::APPMAN_PTR);
        if app == 0 {
            crate::tick::set_for_test(&mut c, hz);
        } else {
            c.bus.write32(app + crate::tick::APPMAN_RATE, hz);
            c.bus
                .write32(app + crate::tick::APPMAN_STEP, crate::tick::step_bits(hz));
        }
        let before = c.bus.read32(GM + 0x78) as i32;
        f(&mut c);
        (c.bus.read32(GM + 0x78) as i32, before)
    }

    /// A reset's deadline is the mode time in game updates: twice the 60 Hz
    /// count at 120 Hz, four times at 240 (the same wall time).
    fn reset_deadline_follows_the_rate(f: GuestFn, setup: &dyn Fn(&mut Cpu, u32), seeds: &[u32]) {
        let mut timed = false;
        for &seed in seeds {
            let (d60, before) = deadline_at(60, f, setup, seed);
            timed |= d60 != before;
            for hz in [120u32, 240] {
                let (d, _) = deadline_at(hz, f, setup, seed);
                assert_eq!(d, d60.wrapping_mul(hz as i32 / 60), "seed {seed}, {hz} Hz");
            }
        }
        assert!(timed, "no seed set a deadline");
    }

    #[test]
    fn gm_handler0_reset_deadline_follows_the_rate() {
        // Seed 0: no quick race, round 0; seed 4: round 1 with picks.
        reset_deadline_follows_the_rate(gm_handler0_reset, &gm0_reset_setup, &[0, 4, 13]);
    }

    #[test]
    fn gm_handler4_reset_deadline_follows_the_rate() {
        // Modes 6, 7, 8 (time limits from the event table).
        reset_deadline_follows_the_rate(gm_handler4_reset, &gm4_reset_setup, &[0, 1, 2]);
        // The handler's own limit (+0x4, compared with finish times in
        // updates) is the same count.
        let mut c = crate::difftest::fresh();
        gm4_reset_setup(&mut c, 0);
        crate::tick::set_for_test(&mut c, 120);
        gm_handler4_reset(&mut c);
        assert_eq!(c.bus.read32(HANDLER_OBJ + 4), c.bus.read32(GM + 0x78));
    }

    #[test]
    fn gm_handler6_reset_deadline_follows_the_rate() {
        reset_deadline_follows_the_rate(gm_handler6_reset, &gm6_reset_setup, &[0, 1]);
    }

    #[test]
    fn gm_handler7_reset_deadline_follows_the_rate() {
        reset_deadline_follows_the_rate(gm_handler7_reset, &gm7_reset_setup, &[0, 1, 2]);
        // The target time (+0x7c) stays in 1/100 s.
        let target = |hz: u32| {
            let mut c = crate::difftest::fresh();
            gm7_reset_setup(&mut c, 1);
            crate::tick::set_for_test(&mut c, hz);
            gm_handler7_reset(&mut c);
            c.bus.read32(GM + 0x7c)
        };
        assert_eq!(target(240), target(60));
    }

    #[test]
    fn gm_handler9_reset_deadline_follows_the_rate() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = base_4(c, seed, 0x239d88);
            be_interfaces(c);
            let h = DATA;
            let vt = DATA + 0x100;
            c.bus.write32(h, vt);
            c.bus.write32(h + 4, r.next_u32());
            c.bus.write16(vt + 0x20, 0);
            c.bus.write32(vt + 0x24, LEAF);
            c.bus.write32(GAME_GP - 0x480, GM);
            c.bus.write32(GM + 0x78, DEADLINE_UNSET);
            c.g[A0][0] = sx(h);
        };
        reset_deadline_follows_the_rate(gm_handler9_reset, &setup, &[0, 1]);
    }

    /// The `v06` extensions add the seconds in game updates: the deadline
    /// moves by `rate` per second.
    fn extension_follows_the_rate(f: GuestFn, setup: &dyn Fn(&mut Cpu, u32), seed: u32) {
        let (a60, b60) = deadline_at(60, f, setup, seed);
        assert_ne!(a60, b60, "seed {seed}: not extended");
        for hz in [120u32, 240] {
            let (a, b) = deadline_at(hz, f, setup, seed);
            assert_eq!(
                a.wrapping_sub(b),
                a60.wrapping_sub(b60).wrapping_mul(hz as i32 / 60),
                "{hz} Hz"
            );
        }
    }

    #[test]
    fn gm_handler0_v06_extension_follows_the_rate() {
        // Seed 2: running, the deadline still ahead of the counter.
        extension_follows_the_rate(
            gm_handler0_v06,
            &|c, seed| {
                gm0_v06_setup(c, seed);
                c.g[A2][0] = 7;
            },
            2,
        );
    }

    #[test]
    fn gm_handler9_v06_extension_follows_the_rate() {
        extension_follows_the_rate(
            gm_handler9_v06,
            &|c, seed| {
                let _ = gm_reset_base(c, seed, 0x23a070);
                // Player 0 one point behind, then ahead after the bump.
                c.bus.write16(DATA + 4, 3);
                c.bus.write16(DATA + 6, 3);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0;
                c.g[A2][0] = 9;
                c.bus.write32(GM + 0x8, 1);
                let now = c.bus.read32(WORLD_3 + 0x2000 + 0x8);
                c.bus.write32(GM + 0x78, now + 100);
            },
            0,
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;

    use crate::difftest::put_words;

    pub(crate) use crate::game::ai::ai::tests::oc425_machine;
    pub(crate) use crate::game::ai::rider::tests::oc425_force;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_gp;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_queue;

    /// `cGameModeMan::getGM`: the instance exists already (a seventh of the
    /// seeds), else every handler slot empty or taken, and allocations that
    /// fail (the retries of the 0x7c, 0xc and 0x8 byte handlers). Allocator
    /// and `memset` are fakes (some unwinding).
    #[test]
    fn game_mode_man_get_instance_matches() {
        oc425_install();
        covered(
            ee::s06::f_00237cf8__generated,
            game_mode_man_get_instance,
            300,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0023_7cf8, 14);
                c.pc = 0x0023_7cf8;
                let gp = c.g[GP][0] as u32;
                let have = seed % 7 == 6;
                c.bus
                    .write32(gp - 0x484, if have { DATA + 0x5000 } else { 0 });
                c.bus.write32(gp - 0x480, r.next_u32());
                for slot in [0u32, 1, 2, 4, 5, 6, 7, 8, 9] {
                    let taken = r.next_u32() % 3 == 0;
                    c.bus.write32(
                        0x0053_6668 + 4 * slot,
                        if taken {
                            DATA + 0x6000 + 0x10 * slot
                        } else {
                            0
                        },
                    );
                }
                for k in 0..30 {
                    if r.next_u32() % 4 == 0 {
                        oc425_force(c, k, Some(0), None);
                    }
                }
            },
        );
    }

    /// A vtable at `vt` with `{delta, FAKE}` at `slot`.
    pub(crate) fn oc427_vtable(c: &mut Cpu, vt: u32, slot: u32, delta: u16) {
        c.bus.write16(vt + slot, delta);
        c.bus.write32(vt + slot + 4, fakes::FAKE);
    }

    /// `GMHandler1_Reset`: every heat (-1..4; heats 0 and 1 are the first
    /// heat), the quick-start flag only with heat 3, the rebuild paths of
    /// `PickOpponents`/`SetupRiderSlots` skipped by their guards (they are
    /// real, unbounded callees): the +8/+0xc latches non-zero and the event
    /// flag at +0x84 not 1. The first-heat vtable call is a fake method.
    #[test]
    fn gm_handler1_reset_matches() {
        covered(
            ee::s06::f_0023a108__generated,
            gm_handler1_reset,
            200,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x23a108, 14, 0x0023_a108);
                let this = DATA + 0x1000;
                let vt = DATA + 0x1400;
                let gm = DATA + 0x3000;
                let heat = [0xffff_ffffu32, 0, 1, 2, 3, 4][r.next_u32() as usize % 6];
                c.bus.write32(oc427_gp(-0x480), gm);
                c.bus.write32(gm + 0x70, heat);
                c.bus.write32(gm, r.next_u32() % 5);
                c.bus
                    .write32(gm + 0x84, [0, 2, 3][r.next_u32() as usize % 3]);
                c.bus.write8(
                    0x0053_5c11,
                    if heat == 3 {
                        (r.next_u32() % 2) as u8
                    } else {
                        0
                    },
                );
                c.bus.write32(this, vt);
                oc427_vtable(c, vt, 0x28, (r.next_u32() as u16) & !3);
                c.bus.write32(this + 8, 1);
                c.bus.write32(this + 0xc, 1);
                // character table lookups (faked interfaces; ids 0..2)
                oc427_queue(c, 0x0014_6e98, &[(r.next_u32() % 3) as u64]);
                oc427_queue(c, 0x0014_7398, &[(r.next_u32() % 4) as u64]);
                oc427_queue(c, 0x0014_5750, &[(r.next_u32() % 4) as u64]);
                put_words(c, 0x0053_6708, &[0, 1, 2]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    pub(crate) use crate::game::ai::ai::tests::oc432_fake;
    pub(crate) use crate::game::ai::ai::tests::oc434_force;
    pub(crate) use crate::game::ai::ai::tests::oc434_setup;
    pub(crate) use crate::game::ai::ai::tests::oc434_w;
    pub(crate) use crate::game::ai::ai::tests::OC432_CTRL;
    pub(crate) use crate::game::ai::ai::tests::OC432_MAGIC;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    oc432_fake!(
        fake_get_human_rider,
        PREV_GET_HUMAN_RIDER,
        ee::s01::f_00146e98__generated,
        0x0014_6e98
    );

    oc432_fake!(
        fake_get_character,
        PREV_GET_CHARACTER,
        ee::s01::f_00147398__generated,
        0x0014_7398
    );

    oc432_fake!(
        fake_get_peak_rival,
        PREV_GET_PEAK_RIVAL,
        ee::s01::f_00145750__generated,
        0x0014_5750
    );

    oc432_fake!(
        fake_game_rand_next,
        PREV_GAME_RAND_NEXT,
        ee::s06::f_00237cd8__generated,
        0x0023_7cd8
    );

    oc432_fake!(
        fake_game_rand_shuffle,
        PREV_GAME_RAND_SHUFFLE,
        ee::s06::f_0023c770__generated,
        0x0023_c770
    );

    /// `GMHandler1_PickOpponents`: the BE getters, the rand and the shuffle
    /// are logging fakes; the two characters come from forced results (some
    /// equal to loop indices, so the skip branches run, some out of range).
    #[test]
    fn gm_handler1_pick_opponents_matches() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            fake_get_be,
            &PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            fake_be_get_interface,
            &PREV_BE_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146E98),
            fake_get_human_rider,
            &PREV_GET_HUMAN_RIDER,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147398),
            fake_get_character,
            &PREV_GET_CHARACTER,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00145750),
            fake_get_peak_rival,
            &PREV_GET_PEAK_RIVAL,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00237CD8),
            fake_game_rand_next,
            &PREV_GAME_RAND_NEXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0023C770),
            fake_game_rand_shuffle,
            &PREV_GAME_RAND_SHUFFLE,
        );
        fakes::covered(
            ee::s06::f_0023a4f0__generated,
            gm_handler1_pick_opponents,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23a4f0, 10);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.g[A0][0] = DATA as u64;
                let pick =
                    |r: &mut Rng| [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 20][(r.next_u32() % 10) as usize];
                c.bus.write32(fakes::LOG_RESULTS + 4 * 5, pick(&mut r));
                c.bus.write32(fakes::LOG_RESULTS + 4 * 6, pick(&mut r));
                c.bus.write32(fakes::LOG_RESULTS + 4 * 7, r.next_u32() | 1);
                let _ = seed;
            },
        );
    }

    oc432_fake!(
        fake_game_mode_reset,
        PREV_GAME_MODE_RESET,
        ee::s06::f_002382d8__generated,
        0x0023_82d8
    );

    /// `GameModeMan_StartMode`: every event type 0..14 on both network paths
    /// (byte at 0x535c11 equal to 2 or not); the current handler's virtual
    /// method is a fake, reset mode is a fake.
    #[test]
    fn game_mode_man_start_mode_matches() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            fake_get_be,
            &PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            fake_be_get_interface,
            &PREV_BE_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002382D8),
            fake_game_mode_reset,
            &PREV_GAME_MODE_RESET,
        );
        fakes::covered(
            ee::s06::f_00238160__generated,
            game_mode_man_start_mode,
            200,
            &|c, seed| {
                crate::difftest::elf_image(c);
                let mut r = machine(c, seed, 0x238160, 4);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.pc = 0x0023_8160;
                let this = DATA + 0x5_0000;
                let handler = DATA + 0x5_1000;
                let vt = DATA + 0x5_2000;
                let cur = r.next_u32() % 10;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 15) as u64;
                c.bus.write32(this + 4, cur);
                for i in 0..10u32 {
                    c.bus.write32(0x0053_6668 + 4 * i, handler);
                }
                c.bus.write32(handler, vt);
                c.bus.write16(vt + 0x18, (r.next_u32() % 32) as u16);
                c.bus.write32(vt + 0x1c, fakes::FAKE);
                c.bus
                    .write8(0x0053_5c11, if r.next_u32() % 2 == 0 { 2 } else { 1 });
            },
        );
    }

    /// `GMHandler1_SetupRiderSlots`: heat 0..3 (rows of ids below and above 10), the
    /// human rider in slot 1..3 or past the last slot.
    #[test]
    fn gm_handler1_setup_rider_slots_matches() {
        fakes::covered(
            ee::s06::f_0023a668__generated,
            gm_handler1_setup_rider_slots,
            24,
            &|c, seed| {
                let mut r = oc434_setup(c, seed, 0x23a668, 5, 0x0023_a668);
                let (this, mgr) = (DATA + 0x100, DATA + 0x2000);
                c.g[A0][0] = this as u64;
                oc434_w(c, GAME_GP.wrapping_sub(0x480), mgr);
                oc434_w(c, mgr, seed % 5);
                for i in 0..24 {
                    oc434_w(c, this + 0x10 + 4 * i, r.next_u32() % 20);
                }
                oc434_force(c, 0, DATA + 0x3000);
                oc434_force(c, 1, DATA + 0x3100);
                oc434_force(c, 2, [1, 2, 3, 6][(seed % 4) as usize]);
                oc434_force(c, 3, 5 + seed % 4);
                oc434_force(c, 4, 20 + seed % 3);
            },
        );
    }

    pub(crate) use crate::game::fe::festateloadscreen::tests::oc444_run;

    pub(crate) use crate::game::fe::festateloadscreen::tests::OC444_OBJ;
    pub(crate) use crate::game::fe::festateloadscreen::tests::OC444_OBJ2;

    /// `GameModeMan_OnRiderFinished(self, rider, stats)`.
    #[test]
    fn game_mode_man_on_rider_finished_matches() {
        oc444_run(
            ee::s06::f_00238358__generated,
            game_mode_man_on_rider_finished,
            0x0023_8358,
            0x23_8358,
            150,
            &|c, r, seed| {
                c.g[A0][0] = OC444_OBJ as u64;
                c.bus.write32(OC444_OBJ + 4, r.next_u32() % 4);
                c.bus.write32(OC444_OBJ + 0x9c, (seed % 3 != 0) as u32);
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = (OC444_OBJ2 + 4 * (seed % 2 * 2)) as u64;
                c.bus.write32(0x0053_5c08, r.next_u32() % 4);
            },
        );
    }

    use crate::game::scripter::ssxscriptengine::tests::base_14;
    use crate::game::scripter::ssxscriptengine::tests::WORLD_4;

    /// The handler object (`self->t` at +4, three rows of ten words).
    pub(crate) const SELF: u32 = DATA + 0x6_0000;

    /// The game-mode manager (`cGameModeHandler_pManager`, `gp-0x480`).
    pub(crate) const MGR: u32 = DATA + 0x6_1000;

    /// The BE interface object and the BE state with its vtable.
    pub(crate) const IFACE: u32 = DATA + 0x6_2000;

    pub(crate) const BE_STATE: u32 = DATA + 0x6_3000;

    pub(crate) const BE_VT: u32 = DATA + 0x6_3100;

    /// The game (`world +0x84`) and the AI (`game +0xc`) with its rider list
    /// at +0x28 and the last computer index at +0x80.
    pub(crate) const GAME: u32 = WORLD_4 + 0x1000;

    pub(crate) const AI: u32 = WORLD_4 + 0x2000;

    /// The riders (`ai +0x28 + 4 * i`), their metrics and the player chain.
    pub(crate) const RIDER_BASE: u32 = DATA + 0x3_0000;

    pub(crate) const METRIX_BASE: u32 = DATA + 0x4_0000;

    pub(crate) const CHAIN_BASE: u32 = DATA + 0x5_0000;

    /// Two riders: the human and one computer.
    pub(crate) const RIDERS: u32 = 2;

    pub(crate) fn rider(i: u32) -> u32 {
        RIDER_BASE + 0x1000 * i
    }

    /// The BE interfaces answer through a fake vtable slot (the state object
    /// the BE library keeps is not in the test image).
    pub(crate) fn be_interfaces_2(c: &mut Cpu) {
        c.hooks.miss = fake_call;
        c.hooks.jump_out = crate::difftest::fakes::fake_jump_out;
        c.bus
            .write32(crate::difftest::fakes::LOG_UNWIND_AT, u32::MAX);
        for n in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * n, IFACE);
        }
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(0x0053_08a8, BE_STATE);
        c.bus.write32(BE_STATE + 4, BE_VT);
        c.bus.write16(BE_VT + 8, 0);
        c.bus.write32(BE_VT + 0xc, FAKE);
    }

    /// A scrambled machine with the BE library, the world chain and the
    /// manager; the finishing rider is `seed % 2`, the heat `1..=3`.
    pub(crate) fn gm0_finish_setup(c: &mut Cpu, seed: u32) {
        let mut r = base_14(c, seed, 0x0023_9230);
        be_interfaces_2(c);
        c.pc = 0x0023_9230;
        c.bus
            .write8(0x0053_5c11, [2u32, 0, 1][seed as usize % 3] as u8);
        // The manager.
        c.bus.write32(GAME_GP - 0x480, MGR);
        c.bus.write32(MGR, 1 + seed % 3);
        c.bus.write32(MGR + 0x10, RIDERS - 1);
        c.bus.write32(MGR + 0x70, 0);
        c.bus.write32(MGR + 0x74, 0);
        c.bus.write32(MGR + 0x9c, 0);
        // The world chain: world +0x84 -> game, game +0xc -> AI.
        c.bus.write32(WORLD_4 + 0x84, GAME);
        c.bus.write32(GAME + 0xc, AI);
        c.bus.write32(AI + 0x8, r.next_u32() % 4000);
        c.bus.write32(AI + 0x40, CHAIN_BASE);
        c.bus.write32(CHAIN_BASE + 0x18, CHAIN_BASE + 0x100);
        c.bus.write32(AI + 0x80, RIDERS - 1);
        for i in 0..RIDERS {
            let rd = rider(i);
            let metrix = METRIX_BASE + 0x1000 * i;
            c.bus.write32(AI + 0x28 + 4 * i, rd);
            c.bus.write32(rd + 0x790, metrix);
            c.bus.write32(rd + 0x40, CHAIN_BASE);
            c.bus.write32(rd + 0x100, 0);
            c.bus.write32(rd + 0x470, r.next_u32());
            let dnf = seed % 4 == 1 && i == seed % 2;
            c.bus
                .write32(rd + 0x480, if dnf { r.next_u32() | 1 } else { 0 });
            for w in 0..0x30u32 {
                c.bus.write32(metrix + 0xfc + 4 * w, r.next_u32() % 3000);
            }
            c.bus.write32(0x0053_6730 + 4 * i, r.next_u32() % 4);
        }
        c.g[A0][0] = sx(SELF);
        c.g[A1][0] = sx(seed % 2);
    }

    #[test]
    fn gm_handler0_on_rider_finished_matches() {
        check3(
            ee::s06::f_00239230__generated,
            gm_handler0_on_rider_finished,
            60,
            &gm0_finish_setup,
        );
    }

    /// Write one 32-bit word (the fixtures' shorthand).
    pub(crate) fn oc520_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    #[test]
    fn ov_screen_yn_dialog_copy_default_choice_matches() {
        check(
            ssx3_ee::s06::f_00238348__generated,
            ov_screen_yn_dialog_copy_default_choice,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x238348, 0);
                let d = DATA + 0x100;
                oc520_w(c, d + 0x74, seed.wrapping_mul(0x9e37_79b9));
                oc520_w(c, d + 0x70, seed ^ 0x55);
                c.g[A0][0] = d as u64;
                c.pc = 0x0023_8348;
            },
        );
    }

    /// A stream and a handler for the state functions: every transfer goes to
    /// the fake (both vtables' read and write slots), with random manager fields.
    pub(crate) fn game_mode_man_state_setup(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::FAKE;
        use crate::difftest::fill;
        let mut r = machine(c, seed, 0x2385d0, 0);
        let me = DATA + 0x100;
        let stream = DATA + 0x1000;
        let stream_vt = DATA + 0x2000;
        let handler = DATA + 0x3000;
        let handler_vt = DATA + 0x3100;
        fill(c, &mut r, me, 0x40);
        oc520_w(c, stream, stream_vt);
        oc520_w(c, stream_vt + 0xc, FAKE);
        oc520_w(c, stream_vt + 0x14, FAKE);
        oc520_w(c, me + 0x4, seed % 4); // iCurrModeHandler
        for i in 0..4 {
            oc520_w(c, 0x0053_6668 + 4 * i, handler);
        }
        oc520_w(c, handler, handler_vt);
        oc520_w(c, handler_vt + 0x54, FAKE);
        oc520_w(c, handler_vt + 0x5c, FAKE);
        c.g[A0][0] = me as u64;
        c.g[A1][0] = stream as u64;
    }

    #[test]
    fn game_mode_man_save_state_matches() {
        check(
            ssx3_ee::s06::f_002385d0__generated,
            game_mode_man_save_state,
            60,
            &|c, seed| {
                game_mode_man_state_setup(c, seed);
                c.pc = 0x0023_85d0;
            },
        );
    }

    #[test]
    fn game_mode_man_load_state_matches() {
        check(
            ssx3_ee::s06::f_002388a0__generated,
            game_mode_man_load_state,
            60,
            &|c, seed| {
                game_mode_man_state_setup(c, seed);
                c.pc = 0x0023_88a0;
            },
        );
    }

    #[test]
    fn gm_handler2_reset_matches() {
        use crate::difftest::fakes::FAKE;
        use crate::difftest::{fill, GAME_GP};
        check(
            ssx3_ee::s06::f_0023b170__generated,
            gm_handler2_reset,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23b170, 0);
                // The back end's singleton (gp-0x1f04) is a zeroed dummy.
                oc520_w(c, GAME_GP - 0x1f04, DATA + 0x6_0000);
                oc520_w(c, GAME_GP - 0x1efc, 0);
                let gm = DATA + 0x400;
                let me = DATA + 0x800;
                let vt = DATA + 0x900;
                fill(c, &mut r, gm, 0x30);
                oc520_w(c, GAME_GP - 0x480, gm);
                oc520_w(c, me, vt);
                oc520_w(c, vt + 0x24, FAKE); // cleanRiderRanksPoints: a fake
                fill(c, &mut r, me + 4, 4);
                c.g[A0][0] = me as u64;
                c.pc = 0x0023_b170;
            },
        );
    }

    use crate::difftest::scramble;

    /// The BE library singleton (`gp-0x1f04`), its interface table at
    /// `0x5308a8`, the interface and its method (returns 0).
    pub(crate) const STANDINGS_BE_LIB: u32 = DATA + 0x3000;

    pub(crate) const STANDINGS_IFACE: u32 = DATA + 0x3100;

    pub(crate) const STANDINGS_IFACE_VT: u32 = DATA + 0x3200;

    pub(crate) const STANDINGS_BE_IFACES: u32 = 0x0053_08a8;

    pub(crate) const STANDINGS_RETURN_ZERO: u32 = 0x003a_5ab0;

    /// The game-mode manager.
    pub(crate) const STANDINGS_MANAGER: u32 = DATA + 0x1_0000;

    /// Game state for `gm_handler0_init_standings`: the BE singleton and
    /// interfaces (race and player read as 0), the manager's counts, and the
    /// race-path byte.
    pub(crate) fn standings_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0023_9aa0 ^ (seed as u64) << 20 | 1);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP - 0x1f04, STANDINGS_BE_LIB);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(STANDINGS_BE_IFACES, STANDINGS_IFACE);
        c.bus.write32(STANDINGS_IFACE + 4, STANDINGS_IFACE_VT);
        c.bus.write16(STANDINGS_IFACE_VT + 8, 0);
        c.bus
            .write32(STANDINGS_IFACE_VT + 0xc, STANDINGS_RETURN_ZERO);
        c.bus.write32(GAME_GP - GM_MANAGER_GP, STANDINGS_MANAGER);
        // iNumberAI 0..5, iSimAI 0..3: numNonSimulated can be negative.
        c.bus.write32(STANDINGS_MANAGER + 0x10, seed % 6);
        c.bus.write32(STANDINGS_MANAGER + 0x14, (seed / 6) % 4);
        // The race path: repeats of heat 1 or fresh thresholds.
        c.bus
            .write8(NEW_RACE_PATH_BYTE, ((seed / 3) % 3 != 0) as u8);
    }

    #[test]
    fn gm_handler0_init_standings_matches() {
        check(
            ee::s06::f_00239aa0__generated,
            gm_handler0_init_standings,
            240,
            &|c, seed| {
                standings_fixture(c, seed);
                c.pc = 0x0023_9aa0;
            },
        );
    }

    /// Rider-finished fixture: the world, race, riders table and one rider
    /// per slot (did-not-finish flag by seed), each with a metrix whose
    /// totals sit at an aligned or unaligned address by seed.
    pub(crate) const FINISH_WORLD: u32 = DATA + 0x4000;

    pub(crate) const FINISH_RACE: u32 = DATA + 0x4100;

    pub(crate) const FINISH_RIDERS: u32 = DATA + 0x4200;

    pub(crate) const FINISH_RIDER0: u32 = DATA + 0x4400;

    pub(crate) const FINISH_METRIX0: u32 = DATA + 0x2_0000;

    pub(crate) const FINISH_MANAGER: u32 = DATA + 0x1_0000;

    pub(crate) fn finish_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0023_9ec8 ^ (seed as u64) << 20 | 1);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        let idx = seed % 4;
        c.g[A1][0] = idx as u64;
        c.bus.write32(GAME_GP - WORLD_GP, FINISH_WORLD);
        c.bus.write32(FINISH_WORLD + 0x84, FINISH_RACE);
        c.bus.write32(FINISH_RACE + 0xc, FINISH_RIDERS);
        for i in 0..4u32 {
            let rider = FINISH_RIDER0 + i * 0x800;
            let metrix = FINISH_METRIX0 + i * 0x200 + (seed % 2) * 4;
            c.bus.write32(FINISH_RIDERS + 4 * i + 0x28, rider);
            c.bus.write32(rider + 0x790, metrix);
            c.bus.write32(rider + 0x480, (seed >> i) & 1);
            for k in 0..0x2c {
                c.bus.write32(metrix + 0xfc + 4 * k, r.next_u32());
            }
        }
        c.bus.write32(GAME_GP - GM_MANAGER_GP, FINISH_MANAGER);
        c.bus.write32(FINISH_MANAGER + 0x10, 3);
        c.bus.write32(FINISH_MANAGER + 0x14, 2);
        // Rank of this rider: first, second, or out of the points.
        // cGameModeMan_iRanking (0x536730)
        c.bus
            .write32(0x0053_6730 + 4 * idx, (seed % 3).wrapping_sub(1));
    }

    #[test]
    fn gm_handler9_on_rider_finished_matches() {
        check(
            ee::s06::f_00239ec8__generated,
            gm_handler9_on_rider_finished,
            240,
            &|c, seed| {
                finish_fixture(c, seed);
                c.pc = 0x0023_9ec8;
            },
        );
    }

    pub(crate) const SEEDS_3: u32 = 40;

    /// The game-mode handler object (`gp-0x480`).
    pub(crate) const GM_2: u32 = DATA + 0x2_2000;

    /// The `self` of the reset functions and its vtable (slot 0x28/0x2c).
    pub(crate) const SELF_2: u32 = DATA + 0x2_3000;

    pub(crate) const SELF_VT: u32 = DATA + 0x2_3800;

    /// Four riders, each 0x800 bytes apart; the state pointer at +0x6c0.
    pub(crate) const RIDERS_2: u32 = DATA + 0x2_8000;

    /// The copy source of a rider's result block (aligned or not, by seed).
    pub(crate) const SRC: u32 = DATA + 0x3_0000;

    /// A rider block that the point and time tables read.
    pub(crate) const BLOCK: u32 = DATA + 0x2_f000;

    /// The AI's rider table is at `ai + 0x28`.
    pub(crate) const AI_2: u32 = WORLD_4 + 0x2000;

    /// The BE library's interfaces: the real `cBE_getInterface` lands in a
    /// fake that returns `IFACE`, as in the gamemode tests.
    pub(crate) fn oc648_be(c: &mut Cpu) {
        const IFACE: u32 = DATA + 0x6_0000;
        const BE_STATE: u32 = DATA + 0x6_1000;
        const BE_VT: u32 = DATA + 0x6_1100;
        c.hooks.miss = fakes::fake_call;
        c.hooks.jump_out = fakes::fake_jump_out;
        c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
        for n in 0..8u32 {
            c.bus.write32(fakes::LOG_RESULTS + 4 * n, IFACE);
        }
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(0x0053_08a8, BE_STATE);
        c.bus.write32(BE_STATE + 4, BE_VT);
        c.bus.write16(BE_VT + 8, 0);
        c.bus.write32(BE_VT + 0xc, fakes::FAKE);
    }

    /// Scrambled machine with the world chain (world -> game -> AI) and the
    /// game-mode handler at `gp-0x480`.
    pub(crate) fn oc648_world(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = base_14(c, seed, salt);
        oc648_be(c);
        let game = WORLD_4 + 0x1000;
        c.bus.write32(WORLD_4 + 0x84, game);
        c.bus.write32(game + 0xc, AI_2);
        c.bus.write32(AI_2 + 0x8, r.next_u32() % 4000);
        c.bus.write32(GAME_GP - GP_GM, GM_2);
        for i in 0..0x40u32 {
            c.bus.write32(GM_2 + 4 * i, 0);
        }
        c.bus.write32(GM_2 + 0x84, (seed % 2) as u32);
        r
    }

    /// The four riders of the AI's table, with their state objects (a leaf
    /// method) and the copy sources and blocks the handlers read.
    pub(crate) fn oc648_riders(c: &mut Cpu, seed: u32) {
        for i in 0..4u32 {
            let rider = RIDERS_2 + 0x800 * i;
            let state = rider + 0x400;
            c.bus.write32(AI_2 + 0x28 + 4 * i, rider);
            c.bus.write32(rider + 0x6c0, state);
            c.bus.write32(state + 0x4c, LEAF);
            c.bus.write16(state + 0x48, 0);
            let aligned = (seed + i) % 2 == 0;
            c.bus
                .write32(rider + 0x790, if aligned { SRC } else { SRC + 4 });
            c.bus
                .write32(rider + 0x480, if (seed + i) % 3 == 0 { 0 } else { BLOCK });
        }
    }

    /// `self` for the reset functions: a vtable whose slot 5 is the leaf.
    pub(crate) fn oc648_self(c: &mut Cpu) {
        c.bus.write32(SELF_2, SELF_VT);
        c.bus.write16(SELF_VT + 0x28, 0);
        c.bus.write32(SELF_VT + 0x2c, LEAF);
    }

    #[test]
    fn gm_handler8_reset_port_matches() {
        check3(
            ee::s06::f_0023ac88__generated,
            gm_handler8_reset_port,
            SEEDS_3,
            &|c, seed| {
                let _ = oc648_world(c, seed, 0x23ac88);
                oc648_self(c);
                oc648_riders(c, seed);
                c.g[A0][0] = sx(SELF_2);
                c.pc = 0x0023_ac88;
            },
        );
    }

    #[test]
    fn gm_handler5_reset_port_matches() {
        check3(
            ee::s06::f_0023b6c0__generated,
            gm_handler5_reset_port,
            SEEDS_3,
            &|c, seed| {
                let _ = oc648_world(c, seed, 0x23b6c0);
                oc648_self(c);
                oc648_riders(c, seed);
                // The flag byte at 0x535c11 picks the early-out (2) or the
                // rival tables.
                c.bus.write8(0x0053_5c11, if seed % 2 == 0 { 2 } else { 0 });
                c.g[A0][0] = sx(SELF_2);
                c.pc = 0x0023_b6c0;
            },
        );
    }

    #[test]
    fn gm_handler5_on_rider_finished_port_matches() {
        check3(
            ee::s06::f_0023b8c8__generated,
            gm_handler5_on_rider_finished_port,
            SEEDS_3,
            &|c, seed| {
                let _ = oc648_world(c, seed, 0x23b8c8);
                oc648_riders(c, seed);
                c.g[A0][0] = sx(GM_2);
                c.g[A1][0] = (seed % 4) as u64;
                c.pc = 0x0023_b8c8;
            },
        );
    }

    #[test]
    fn gm_handler6_on_rider_finished_port_matches() {
        check3(
            ee::s06::f_0023bdb8__generated,
            gm_handler6_on_rider_finished_port,
            SEEDS_3,
            &|c, seed| {
                let _ = oc648_world(c, seed, 0x23bdb8);
                oc648_riders(c, seed);
                c.g[A0][0] = sx(GM_2);
                c.g[A1][0] = (seed % 4) as u64;
                c.pc = 0x0023_bdb8;
            },
        );
    }

    /// Gate word in the machine's own RAM: the fakes below log only when it
    /// is set, so other tests keep the generated callees.
    pub(crate) const CTRL: u32 = DATA + 0x20_f000;

    pub(crate) const MAGIC: u32 = 0x6f63_3634;

    /// Scratch area of the fixtures (the machine is fresh per run).
    pub(crate) const BASE: u32 = DATA + 0x20_0000;

    macro_rules! gated {
        ($fake:ident, $prev:ident, $target:literal, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(CTRL) != MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    gated!(
        oc649_fake_be,
        OC649_PREV_BE,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated
    );

    gated!(
        oc649_fake_iface,
        OC649_PREV_IFACE,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );

    gated!(
        oc649_fake_peak,
        OC649_PREV_PEAK,
        0x0014_55d0,
        ee::s01::f_001455d0__generated
    );

    gated!(
        oc649_fake_human,
        OC649_PREV_HUMAN,
        0x0014_6e98,
        ee::s01::f_00146e98__generated
    );

    gated!(
        oc649_fake_char,
        OC649_PREV_CHAR,
        0x0014_7398,
        ee::s01::f_00147398__generated
    );

    gated!(
        oc649_fake_all_view,
        OC649_PREV_ALL_VIEW,
        0x0022_e840,
        ee::s05::f_0022e840__generated
    );

    gated!(
        oc649_fake_sm_update,
        OC649_PREV_SM_UPDATE,
        0x0022_d8d8,
        ee::s05::f_0022d8d8__generated
    );

    gated!(
        oc649_fake_focus,
        OC649_PREV_FOCUS,
        0x0022_e8b8,
        ee::s05::f_0022e8b8__generated
    );

    gated!(
        oc649_fake_update_state,
        OC649_PREV_UPDATE_STATE,
        0x0023_0f40,
        ee::s06::f_00230f40__generated
    );

    gated!(
        oc649_fake_world_update,
        OC649_PREV_WORLD_UPDATE,
        0x003a_6928,
        ee::s13::f_003a6928__generated
    );

    gated!(
        oc649_fake_flush,
        OC649_PREV_FLUSH,
        0x003a_88a8,
        ee::s13::f_003a88a8__generated
    );

    pub(crate) static OC649_PREV_READY: AtomicUsize = AtomicUsize::new(0);

    /// `cGameViewMan_isStreamReady`: the answer is the gate's word 4.
    pub(crate) fn oc649_fake_ready(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &OC649_PREV_READY, ee::s05::f_0022e920__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0022_e920, ra);
        if c.pc == ra {
            c.g[V0][0] = c.bus.read32(CTRL + 4) as u64;
        }
    }

    pub(crate) fn oc649_install() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc649_fake_be,
            &OC649_PREV_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc649_fake_iface,
            &OC649_PREV_IFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001455D0),
            oc649_fake_peak,
            &OC649_PREV_PEAK,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146E98),
            oc649_fake_human,
            &OC649_PREV_HUMAN,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147398),
            oc649_fake_char,
            &OC649_PREV_CHAR,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022E840),
            oc649_fake_all_view,
            &OC649_PREV_ALL_VIEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D8D8),
            oc649_fake_sm_update,
            &OC649_PREV_SM_UPDATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022E8B8),
            oc649_fake_focus,
            &OC649_PREV_FOCUS,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022E920),
            oc649_fake_ready,
            &OC649_PREV_READY,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00230F40),
            oc649_fake_update_state,
            &OC649_PREV_UPDATE_STATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A6928),
            oc649_fake_world_update,
            &OC649_PREV_WORLD_UPDATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A88A8),
            oc649_fake_flush,
            &OC649_PREV_FLUSH,
        );
    }

    /// A rider finishing: the rider `idx = seed % 4`, its DNF flag on every
    /// fifth seed, the point total below or above the target, the peak goals
    /// (fake calls 2 and 3) and both copy alignments (the stats block at
    /// `rider + 0x800`, shifted by `seed & 4`).
    pub(crate) fn oc649_rider_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0023_c2d8, 0);
        c.pc = 0x0023_c2d8;
        c.bus.write32(CTRL, MAGIC);
        let idx = seed % 4;
        c.g[A1][0] = idx as u64;
        let game = BASE;
        let ai = BASE + 0x1000;
        let riders = BASE + 0x2000;
        let gm = BASE + 0x1_0000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), game);
        c.bus.write32(GAME_GP.wrapping_sub(0x480), gm);
        c.bus.write32(game + 0x84, ai);
        c.bus.write32(ai + 0xc, riders);
        for k in 0..4u32 {
            let rider = BASE + 0x3000 + k * 0x1000;
            let stats = rider + 0x800 + (seed & 4);
            c.bus.write32(riders + 0x28 + 4 * k, rider);
            c.bus.write32(rider + 0x790, stats);
            c.bus
                .write32(rider + 0x480, (k == idx && seed % 5 == 0) as u32);
            fill(c, &mut r, stats, 0x1b0 / 4);
        }
        let rider = BASE + 0x3000 + idx * 0x1000;
        let stats = rider + 0x800 + (seed & 4);
        c.bus.write32(stats + 0x198, r.next_u32() % 700);
        c.bus.write32(gm + 0x8, seed % 3);
        c.bus.write32(gm + 0x7c, 200 + 100 * (seed % 3));
        c.bus.write32(fakes::LOG_RESULTS + 8, 1 + r.next_u32() % 6);
        c.bus.write32(fakes::LOG_RESULTS + 12, 1 + r.next_u32() % 6);
    }

    /// `GMHandlerUnused_Reset`: the human's seat and the character are chosen
    /// by the fixture (fake calls 4 and 5), the vtable's cleanup is a fake.
    pub(crate) fn oc649_seat_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0023_c618, 0);
        c.pc = 0x0023_c618;
        c.bus.write32(CTRL, MAGIC);
        let this = BASE + 0x7000;
        let vt = BASE + 0x7100;
        let gm = BASE + 0x1_0000;
        c.g[A0][0] = this as u64;
        c.bus.write32(this, vt);
        c.bus.write16(vt + 0x20, (0x10 * (seed % 2)) as u16);
        c.bus.write32(vt + 0x24, fakes::FAKE);
        c.bus.write32(GAME_GP.wrapping_sub(0x480), gm);
        c.bus.write32(0x0053_5c04, 3 + seed % 5);
        fill(c, &mut r, gm, 0x40);
        c.bus.write32(fakes::LOG_RESULTS + 16, 1 + seed % 9);
        c.bus.write32(fakes::LOG_RESULTS + 20, 1 + (seed / 9) % 9);
    }

    #[test]
    fn gm_handler7_on_rider_finished_matches() {
        oc649_install();
        check(
            ee::s06::f_0023c2d8__generated,
            gm_handler7_on_rider_finished,
            300,
            &|c, seed| oc649_rider_setup(c, seed),
        );
    }

    #[test]
    fn gm_handler_unused_reset_matches() {
        oc649_install();
        check(
            ee::s06::f_0023c618__generated,
            gm_handler_unused_reset,
            300,
            &|c, seed| oc649_seat_setup(c, seed),
        );
    }

    pub(crate) use crate::game::main::gamestate::tests::OC682_CTRL;
    pub(crate) use crate::game::main::gamestate::tests::OC682_SEEDS;

    pub(crate) use crate::game::main::gamestate::tests::OC682_OBJ;
    pub(crate) use crate::game::main::gamestate::tests::OC682_SMALL;
    pub(crate) use crate::game::main::gamestate::tests::OC682_SOUP_WORDS;

    pub(crate) use crate::game::main::gamestate::tests::oc682_force;

    pub(crate) use crate::game::main::gamestate::tests::oc682_base;
    pub(crate) use crate::game::main::gamestate::tests::oc682_child;
    pub(crate) use crate::game::main::gamestate::tests::oc682_gp;
    pub(crate) use crate::game::main::gamestate::tests::oc682_pick;

    /// `GMHandler0_PickOpponents` against the generated body: the human and
    /// rival characters inside and outside 0..9 (equal or not), the game path
    /// flag; the shuffle is a fake.
    #[test]
    fn gm_handler0_pick_opponents_matches() {
        if !oc682_child(module_path!(), "gm_handler0_pick_opponents_matches") {
            return;
        }
        covered(
            ee::s06::f_00239938__generated,
            gm_handler0_pick_opponents,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x23_9938, 10, 0x0023_9938);
                oc682_gp(c, -0x480, OC682_OBJ + 0x2000);
                let chars = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12];
                let a = oc682_pick(&mut r, &chars);
                let b = if seed % 5 == 0 {
                    a
                } else {
                    oc682_pick(&mut r, &chars)
                };
                oc682_force(c, 0, 0x0014_7398, a, a, 0);
                oc682_force(c, 1, 0x0014_5750, b, b, 0);
                oc682_force(c, 2, 0x0014_6e98, seed % 4, seed % 4, 0);
                c.bus.write8(0x0053_5c11, (seed & 1) as u8);
            },
        );
    }

    /// `GMHandler1_OnRiderFinished` against the generated body: heats 0..4,
    /// 0..9 AI riders, race and rider finished or not, every place and time
    /// gap of the heat logic, the did-not-finish case, the boost bar of the
    /// player's rider; the sort is a fake.
    #[test]
    fn gm_handler1_on_rider_finished_matches() {
        if !oc682_child(module_path!(), "gm_handler1_on_rider_finished_matches") {
            return;
        }
        covered(
            ee::s06::f_0023a760__generated,
            gm_handler1_on_rider_finished,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x23_a760, 14, 0x0023_a760);
                let bits = r.next_u32();
                // The game mode manager (gp-0x480): heat, AI count, next heats.
                let man = OC682_OBJ + 0x2000;
                oc682_gp(c, -0x480, man);
                for i in 0..0x40 {
                    let v = oc682_pick(&mut r, &OC682_SMALL);
                    c.bus.write32(man + 4 * i, v);
                }
                c.bus
                    .write32(man, [1u32, 2, 3, 0, 4][((bits >> 4) % 5) as usize]);
                c.bus.write32(
                    man + 0x10,
                    [0u32, 1, 2, 3, 9, 0xffff_ffff][(seed % 6) as usize],
                );
                // The riders: gp-0x848 -> +0x84 -> +0xc -> riders at +0x28.
                let (world, game, riders) =
                    (OC682_OBJ + 0x2100, OC682_OBJ + 0x2200, OC682_OBJ + 0x2300);
                oc682_gp(c, -0x848, world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0xc, riders);
                for i in 0..12 {
                    let rider = DATA + (r.next_u32() % OC682_SOUP_WORDS) * 4;
                    c.bus.write32(riders + 0x28 + 4 * i, rider);
                }
                // The race tables: times, did-not-finish flags and places.
                let base = r.next_u32() % 600;
                c.bus.write32(0x0053_6640, base);
                let gap = oc682_pick(&mut r, &[100, 179, 181, 299, 301, 599, 601, 700]);
                c.bus.write32(0x0053_6644, base + gap);
                for i in 2..10 {
                    c.bus.write32(0x0053_6640 + 4 * i, r.next_u32() % 900);
                }
                for i in 0..16 {
                    c.bus.write32(0x0053_66a8 + 4 * i, r.next_u32() & 1);
                    let place = r.next_u32() % 7;
                    c.bus.write32(0x0053_6730 + 4 * i, place);
                }
                c.bus.write8(0x0053_5c11, ((bits >> 8) & 1) as u8);
                oc682_gp(
                    c,
                    -0x5154,
                    [0x3f33_3333u32, 0x3eb3_3333, 0][((bits >> 10) % 3) as usize],
                );
                oc682_gp(
                    c,
                    -0x5150,
                    [0x3f33_3333u32, 0x3eb3_3333, 0][((bits >> 12) % 3) as usize],
                );
                // Race finished (first and later calls), the human rider.
                let a = (bits >> 14) & 1;
                let b = (bits >> 15) & 1;
                oc682_force(c, 0, 0x0012_a250, a, b, bits >> 16 & 0xff);
                oc682_force(c, 1, 0x0014_6e98, (bits >> 24) % 4, (bits >> 24) % 4, 0);
                c.bus.write32(OC682_CTRL + 4, 1);
                c.g[A0][0] = (OC682_OBJ + 0x4000) as u64;
                c.g[A1][0] = ((bits >> 26) % 4) as u64;
            },
        );
    }

    #[test]
    fn gm_handler8_pick_opponents_matches() {
        check(
            ee::s06::f_0023b0a8__generated,
            gm_handler8_pick_opponents,
            200,
            &|c, seed| {
                machine(c, seed, 0xb0a8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0023b0a8;
            },
        );
    }
}
