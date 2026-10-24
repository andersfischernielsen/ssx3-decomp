//! `src/be/beintecon.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
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
pub(crate) use crate::game::be::beintmission::lw;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_CHARACTER;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_PROFILE;
pub(crate) use crate::game::be::beintmission::EA_MEM_ALLOC_NAMED;
pub(crate) use crate::game::be::beintmission::RIDER_STRIDE;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::sound::bankmonitor::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_08d8,
        be_econ_int_get_instance,
        "BEEconInt_GetInstance",
    ),
    (0x0015_0928, be_econ_int_get_money, "BEEconInt_GetMoney"),
    (
        0x0015_0960,
        be_econ_int_get_rider_money,
        "BEEconInt_GetRiderMoney",
    ),
    (
        0x0015_09c0,
        be_econ_int_get_total_earned,
        "BEEconInt_GetTotalEarned",
    ),
    (
        0x0015_09f8,
        be_econ_int_get_rider_total_earned,
        "BEEconInt_GetRiderTotalEarned",
    ),
    (0x0015_0a90, be_econ_int_add_money, "BEEconInt_AddMoney"),
    (
        0x0015_0ae0,
        be_econ_int_add_rider_money,
        "BEEconInt_AddRiderMoney",
    ),
    (0x0015_0b48, be_econ_int_spend_money, "BEEconInt_SpendMoney"),
    (
        0x0015_0b88,
        be_set_char_state_flag_cheat,
        "BE_SetCharStateFlagCheat",
    ),
    (
        0x0015_0e50,
        be_econ_int_get_attribute_cost,
        "BEEconInt_GetAttributeCost",
    ),
    (
        0x0015_1178,
        reward_get_collect_item_points,
        "BERewardInt_GetCollectItemPoints",
    ),
    (
        0x0015_11b0,
        reward_get_mission_points,
        "BERewardInt_GetMissionPoints",
    ),
    (0x0015_1368, be_char_profile_dtor, "BE_CharProfile_dtor"),
    (
        0x0015_15b8,
        be_restore_char_profile_equip_flags,
        "BE_RestoreCharProfileEquipFlags",
    ),
    (
        0x0015_19e0,
        be_reset_char_rivalries,
        "BE_ResetCharRivalries",
    ),
    (
        0x0015_1c48,
        be_set_char_profile_bolt_owned,
        "BE_SetCharProfileBoltOwned",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_3 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2 as FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2 as FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2 as REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2 as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
}

/// The shared address arithmetic of the two economy getters: `db` (0x4a6ca8)
/// plus the rider slot (stride 0x9b50) and character slot (stride 0xf88).
pub(crate) fn econ_rider_base(c: &mut Cpu) -> u32 {
    c.cycles += 9;
    c.g[A0][0] = 0x9b50;
    let char_off = abi::mult(c, abi::arg(c, 2), 0xf88);
    c.g[V1][0] = sx(char_off);
    let rider_off = abi::mult(c, abi::arg(c, 1), 0x9b50);
    c.g[A0][0] = sx(rider_off);
    c.g[V0][0] = sx(0x4a_0000u32.wrapping_add(0x6ca8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] as u32
}

/// `int BEEconInt_GetMoney(BEEconInt *self, int rider, int character)`
/// (0x150928): 0 for rider slots >= 2, else the money word at +0xac4.
pub fn be_econ_int_get_money(c: &mut Cpu) {
    c.cycles += 3;
    let rider = abi::arg64(c, 1);
    let under_two = (rider as i64) < 2;
    c.g[V0][0] = under_two as u64;
    c.g[V1][0] = 0xf88;
    if !under_two {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    let base = econ_rider_base(c);
    c.g[V0][0] = Addr(base).off(0xac4).lw(c);
    abi::ret(c);
}

/// `int BEEconInt_GetTotalEarned(BEEconInt *self, int rider, int character)`
/// (0x1509c0): as `GetMoney`, but the word at +0xac8.
pub fn be_econ_int_get_total_earned(c: &mut Cpu) {
    c.cycles += 3;
    let rider = abi::arg64(c, 1);
    let under_two = (rider as i64) < 2;
    c.g[V0][0] = under_two as u64;
    c.g[V1][0] = 0xf88;
    if !under_two {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    let base = econ_rider_base(c);
    c.g[V0][0] = Addr(base).off(0xac8).lw(c);
    abi::ret(c);
}

/// `int BEEconInt_AddMoney(BEEconInt *self, int rider, int character, int amount)`
/// (0x150a90): add `amount` to both money (+0xac4) and total earned (+0xac8);
/// returns the new money value.
pub fn be_econ_int_add_money(c: &mut Cpu) {
    c.cycles += 3;
    let rider = abi::arg64(c, 1);
    let under_two = (rider as i64) < 2;
    c.g[V0][0] = under_two as u64;
    c.g[V1][0] = 0xf88;
    if !under_two {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 15;
    c.g[A0][0] = 0x9b50;
    let rider_off = abi::mult(c, abi::arg(c, 1), 0x9b50);
    c.g[A0][0] = sx(rider_off);
    let char_off = abi::mult(c, abi::arg(c, 2), 0xf88);
    c.g[V1][0] = sx(char_off);
    c.g[V0][0] = sx(0x4a_0000u32.wrapping_add(0x6ca8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let money = Addr(c.g[V1][0] as u32).off(0xac4).lw(c);
    let earned = Addr(c.g[V1][0] as u32).off(0xac8).lw(c);
    c.g[A1][0] = sx((money as u32).wrapping_add(abi::arg(c, 3)));
    c.g[A0][0] = sx((earned as u32).wrapping_add(abi::arg(c, 3)));
    c.g[V0][0] = c.g[A1][0];
    Addr(c.g[V1][0] as u32).off(0xac8).sw(c, c.g[A0][0]);
    Addr(c.g[V1][0] as u32).off(0xac4).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `int BEEconInt_SpendMoney(BEEconInt *self, int rider, int character, int amount)`
/// (0x150b48): subtract `amount` from money (+0xac4); returns the new value.
pub fn be_econ_int_spend_money(c: &mut Cpu) {
    c.cycles += 3;
    let rider = abi::arg64(c, 1);
    let under_two = (rider as i64) < 2;
    c.g[V0][0] = under_two as u64;
    c.g[V1][0] = 0xf88;
    if !under_two {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 11;
    c.g[A0][0] = 0x9b50;
    let char_off = abi::mult(c, abi::arg(c, 2), 0xf88);
    c.g[V1][0] = sx(char_off);
    let rider_off = abi::mult(c, abi::arg(c, 1), 0x9b50);
    c.g[A0][0] = sx(rider_off);
    c.g[V0][0] = sx(0x4a_0000u32.wrapping_add(0x6ca8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xac4).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(abi::arg(c, 3)));
    Addr(c.g[V1][0] as u32).off(0xac4).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `int BEEconInt_GetAttributeCost(BEEconInt *self, int attribute)`
/// (0x150e50): the cost at `attribute*8 + 0x440554`.
pub fn be_econ_int_get_attribute_cost(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x44_0000;
    c.g[A1][0] = sx(abi::arg(c, 1) << 3);
    c.g[V1][0] = sx(0x44_0000u32.wrapping_add(0x550));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4).lw(c);
    abi::ret(c);
}

/// `BEEconInt *BEEconInt_GetInstance(void)` (0x1508d8).
pub fn be_econ_int_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1edc));
        let have = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !have {
            c.cycles += 11;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx(0x460000u32.wrapping_sub(0x58e8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, EA_MEM_ALLOC_NAMED, 0x0015_0900)?;
            c.g[V1][0] = 0x460000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
            c.g[V1][0] = sx(0x460000u32.wrapping_sub(0x5388));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1edc), c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1edc));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const ECON_ADD_MONEY: Callee = Callee {
    addr: 0x0015_0a90,
    f: ssx3_ee::s01::f_00150a90,
};

/// `int BEEconInt_GetRiderMoney(BEEconInt *self, int rider)` (0x150960): the
/// rider's money at `0x4a6ca8 + char*0xf88 + profile*0x9b50 + 0xac4`.
pub fn be_econ_int_get_rider_money(c: &mut Cpu) {
    guest(c, |c| econ_rider_field(c, 0x0015_097c, 0x0015_0988, 0x0ac4))
}

/// `int BEEconInt_GetRiderTotalEarned(BEEconInt *self, int rider)` (0x1509f8):
/// the rider's total earned at +0xac8.
pub fn be_econ_int_get_rider_total_earned(c: &mut Cpu) {
    guest(c, |c| econ_rider_field(c, 0x0015_0a14, 0x0015_0a20, 0x0ac8))
}

/// Shared body of the economy getters: `<char,profile> -> field`.
pub(crate) fn econ_rider_field(c: &mut Cpu, r_prof: u32, r_char: u32, field: u32) -> Flow {
    c.cycles += 24;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[S1][0];
    BE_GET_RIDER_PROFILE.call(c, r_prof)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[S0][0] = c.g[V0][0];
    BE_GET_RIDER_CHARACTER.call(c, r_char)?;
    c.g[V1][0] = RIDER_STRIDE as u64;
    c.g[A0][0] = 0x9b50;
    c.mult(0, V0, V0, V1);
    c.mult(0, S0, S0, A0);
    c.g[V1][0] = 0x4a0000;
    frame.restore(c, S1, 0x10);
    c.g[V1][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
    frame.restore_ra(c, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    frame.restore(c, S0, 0x20);
    c.g[V0][0] = lw(c, (c.g[V0][0] as u32).wrapping_add(field));
    frame.close(c);
    Ok(())
}

/// `void BEEconInt_AddRiderMoney(BEEconInt *self, int rider, int amount)`
/// (0x150ae0): add `amount` to the rider's money and total earned.
pub fn be_econ_int_add_rider_money(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_0b0c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_0b18)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[A2][0] = c.g[V0][0];
        ECON_ADD_MONEY.call(c, 0x0015_0b2c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE rewards.

/// `int BERewardInt_GetCollectItemPoints(BERewardInt* self, int level)`
/// (0x151178): 1000 points for level 1, 500 below that, 2000 for level 2.
pub fn reward_get_collect_item_points(c: &mut Cpu) {
    c.cycles += 3;
    let level = c.g[A1][0];
    c.g[V0][0] = 1;
    let is_one = level == c.g[V0][0];
    c.g[V0][0] = ((level as i64) < 2) as u64;
    if is_one {
        c.cycles += 2;
        c.g[V0][0] = 0x3e8;
        crate::abi::ret(c);
        return;
    }
    c.cycles += 2;
    let below_two = c.g[V0][0] != 0;
    c.g[V0][0] = 2;
    if !below_two {
        c.cycles += 2;
        if level == c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = 0x7d0;
            crate::abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0x1f4;
    crate::abi::ret(c);
}

/// `int BERewardInt_GetMissionPoints(BERewardInt* self, int level)`
/// (0x1511b0): 4000 for level 2, 2000 below that, 6000 for level 3.
pub fn reward_get_mission_points(c: &mut Cpu) {
    c.cycles += 3;
    let level = c.g[A1][0];
    c.g[V0][0] = 2;
    let is_two = level == c.g[V0][0];
    c.g[V0][0] = ((level as i64) < 3) as u64;
    if is_two {
        c.cycles += 2;
        c.g[V0][0] = 0xfa0;
        crate::abi::ret(c);
        return;
    }
    c.cycles += 2;
    let below_three = c.g[V0][0] != 0;
    c.g[V0][0] = 3;
    if !below_three {
        c.cycles += 2;
        if level == c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = 0x1770;
            crate::abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0x7d0;
    crate::abi::ret(c);
}

/// `void BE_RestoreCharProfileEquipFlags(Profile* self)` (0x1515b8): for each
/// item (flags halfword at `+2`, `self+0x290`, stride 4), set bit 4 when bit
/// 2 is set, clear it otherwise.
pub fn be_restore_char_profile_equip_flags(c: &mut Cpu) {
    c.cycles += 3;
    let base = abi::arg(c, 0);
    c.g[V0][0] = Addr(base).off(0x28c).lw(c);
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A0][0] = sx(p(base, 0x290));
    if none {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = 0xffff_ffff_ffff_ffef;
    loop {
        c.cycles += 4;
        c.g[V1][0] = Addr(r32(c, A0)).off(2).lhu(c);
        c.g[V0][0] = c.g[V1][0] & 4;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.g[V1][0] | 0x10;
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.g[V1][0] & c.g[A2][0];
        }
        c.cycles += 4;
        c.sh(p(r32(c, A0), 2), c.g[V0][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0015_15d0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void BE_SetCharProfileBoltOwned(Profile* p, int id, bool on)` (0x151c48):
/// look `id` up in the u16 index table at `+0x288`; a non-negative index
/// selects the item at `+0x290 + idx*4`, whose flags word (`+2`) gets bit 1
/// set or cleared. A negative index falls back to address 2, as the original
/// does.
pub fn be_set_char_profile_bolt_owned(c: &mut Cpu) {
    c.cycles += 6;
    let prof = abi::arg(c, 0);
    let id = abi::arg(c, 1);
    let lookup = Addr(prof.wrapping_add(0x288)).lw(c) as u32;
    let idx = Addr(lookup.wrapping_add(id << 1)).lh(c);
    c.g[V0][0] = sx((idx as u32) << 2);
    let item = if (idx as i64) < 0 {
        c.cycles += 1;
        0
    } else {
        c.cycles += 3;
        let a = prof.wrapping_add((idx as u32) << 2).wrapping_add(0x290);
        c.g[A0][0] = sx(a);
        a
    };
    c.cycles += 2;
    let on = abi::arg(c, 2);
    let flags = Addr(item.wrapping_add(2)).lhu(c) as u32;
    if on == 0 {
        c.cycles += 3;
        c.g[V0][0] = sx(flags & 0xfffd);
    } else {
        c.cycles += 3;
        c.g[V0][0] = sx(flags | 2);
    }
    Addr(item.wrapping_add(2)).sh(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// BE character profile.

/// `void BE_CharProfile_dtor(BE_CharProfile* p, int flags)` (0x151368):
/// free the owned block at +0x288 and, when bit 0 of `flags` is set, delete
/// the object. 8 (+2) + 3 (+2) + 5 cycles plus the callees.
pub fn be_char_profile_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let owned = c.lw(p(self_, 0x288));
        c.g[A0][0] = owned;
        c.g[S1][0] = c.g[A1][0];
        if owned != 0 {
            c.cycles += 2;
            call_ptr(c, callee::EA_MEM_FREE, 0x0015_1390)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        c.sw(p(self_, 0x288), 0);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::OPERATOR_DELETE, 0x0015_13a4)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BE_SetCharStateFlagCheat` (0x150b88): set or clear character-state flag
/// 12/13 for player 0 or 1 (cheat helper), returning 1 when handled.
pub fn be_set_char_state_flag_cheat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < 2i64) as u64;
        let player_ok = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[T0][0];
        if player_ok {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let one = c.g[A3][0] != c.g[V0][0];
            c.g[A1][0] = c.g[A1][0] ^ 1;
            if !one {
                c.cycles += 10;
                c.g[V1][0] = 0xf88;
                c.g[V0][0] = 0x9b50;
                c.mult(0, 2, 4, 2);
                c.mult(0, 3, 6, 3);
                c.g[A0][0] = 0x004a_0000;
                c.g[A1][0] = c.g[A1][0] & 1;
                c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x6ca8));
                c.g[A2][0] = 0xffff_ffff_ffff_efff;
                c.g[A1][0] = c.g[A1][0] << 12;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                let two = c.g[A3][0] != c.g[V0][0];
                c.g[V1][0] = 0xf88;
                if two {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    abi::ret(c);
                    return Ok(());
                }
                c.cycles += 8;
                c.g[V0][0] = 0x9b50;
                c.mult(0, 2, 4, 2);
                c.mult(0, 3, 6, 3);
                c.g[A0][0] = 0x004a_0000;
                c.g[A1][0] = c.g[A1][0] & 1;
                c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x6ca8));
                c.g[A2][0] = 0xffff_ffff_ffff_dfff;
                c.g[A1][0] = c.g[A1][0] << 13;
            }
            c.cycles += 6;
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
            c.g[V0][0] = c.ld(p(r(c, V1), 0x278));
            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
            c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
            c.sd(p(r(c, V1), 0x278), c.g[V0][0]);
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
        }
        Ok(())
    })
}

/// `void BE_ResetCharRivalries(p, index)` (0x1519e0): write the three record
/// bytes for `index` into the rivalry table (10 entries of 3 bytes).
pub fn be_reset_char_rivalries(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        c.cycles += 20;
        c.g[V1][0] = 0xc;
        c.mult(0, 5, 5, 3);
        c.g[V0][0] = sx(0x0044_0000);
        c.g[V1][0] = sx(0x004a_0000);
        c.g[V0][0] = sx(0x0044_06fc);
        c.g[T2][0] = sx(0x004a_1218);
        c.g[A3][0] = Addr(c.g[T2][0] as u32).lb(c);
        c.g[T0][0] = Addr(at(c.g[T2][0], 1)).lb(c);
        c.g[T1][0] = Addr(at(c.g[T2][0], 2)).lb(c);
        Addr(frame.at(c, 0)).sb(c, c.g[A3][0]);
        Addr(frame.at(c, 1)).sb(c, c.g[T0][0]);
        Addr(frame.at(c, 2)).sb(c, c.g[T1][0]);
        c.g[A2][0] = sx(at(c.g[V0][0], 4));
        c.g[A0][0] = sx(at(c.g[A0][0], 0xbc2));
        c.g[V1][0] = 0;
        c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[T0][0] = 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A2][0] = 2;
        loop {
            'next: {
                'emit: {
                    'zero_or_match: {
                        c.cycles += 9;
                        c.g[T3][0] = Addr(frame.at(c, 0)).lb(c);
                        c.g[V0][0] = Addr(frame.at(c, 1)).lb(c);
                        c.g[T1][0] = Addr(frame.at(c, 2)).lb(c);
                        Addr(at(c.g[A0][0], 0xffff_ffff)).sb(c, c.g[T3][0]);
                        Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);
                        Addr(at(c.g[A0][0], 1)).sb(c, c.g[T1][0]);
                        c.g[V0][0] = lw(c, c.g[A3][0] as u32);
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = lw(c, c.g[A1][0] as u32);
                            break 'zero_or_match;
                        }
                        c.cycles += 3;
                        Addr(at(c.g[A0][0], 0xffff_ffff)).sb(c, 0);
                        Addr(c.g[A0][0] as u32).sb(c, 0);
                        break 'emit;
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V1][0] = sx(at(c.g[V1][0], 1));
                        break 'next;
                    }
                    c.cycles += 2;
                    Addr(at(c.g[A0][0], 0xffff_ffff)).sb(c, c.g[T0][0]);
                    Addr(c.g[A0][0] as u32).sb(c, c.g[A2][0]);
                    break 'emit;
                }
                c.cycles += 1;
                c.g[V1][0] = sx(at(c.g[V1][0], 1));
                break 'next;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10i64) as u64;
            let t = c.g[V0][0] != 0;
            c.g[A0][0] = sx(at(c.g[A0][0], 3));
            if !t {
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0015_1a30);
            }
        }
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::be::beintmission::tests::instance_setup;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    use ssx3_ee_rt::funcs::GuestFn;

    /// Address arithmetic shared by the economy tests.
    pub(crate) fn econ_base(rider: u32, ch: u32) -> u32 {
        0x4a6ca8u32
            .wrapping_add(rider.wrapping_mul(0x9b50))
            .wrapping_add(ch.wrapping_mul(0xf88))
    }

    #[test]
    fn be_econ_int_get_money_matches() {
        check_lohi(
            ssx3_ee::s01::f_00150928,
            be_econ_int_get_money,
            300,
            &|c, seed| {
                let mut r = Rng(0x5000_0006 ^ seed as u64);
                scramble(c, &mut r);
                let rider = seed % 3;
                let ch = seed % 4;
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = ch as u64;
                c.bus.write32(econ_base(rider, ch) + 0xac4, r.next_u32());
            },
        );
    }

    #[test]
    fn be_econ_int_get_total_earned_matches() {
        check_lohi(
            ssx3_ee::s01::f_001509c0,
            be_econ_int_get_total_earned,
            300,
            &|c, seed| {
                let mut r = Rng(0x6000_0007 ^ seed as u64);
                scramble(c, &mut r);
                let rider = seed % 3;
                let ch = seed % 4;
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = ch as u64;
                c.bus.write32(econ_base(rider, ch) + 0xac8, r.next_u32());
            },
        );
    }

    #[test]
    fn be_econ_int_add_money_matches() {
        check_lohi(
            ssx3_ee::s01::f_00150a90,
            be_econ_int_add_money,
            300,
            &|c, seed| {
                let mut r = Rng(0x7000_0008 ^ seed as u64);
                scramble(c, &mut r);
                let rider = seed % 3;
                let ch = seed % 4;
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = ch as u64;
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                let base = econ_base(rider, ch);
                c.bus.write32(base + 0xac4, r.next_u32());
                c.bus.write32(base + 0xac8, r.next_u32());
            },
        );
    }

    #[test]
    fn be_econ_int_spend_money_matches() {
        check_lohi(
            ssx3_ee::s01::f_00150b48,
            be_econ_int_spend_money,
            300,
            &|c, seed| {
                let mut r = Rng(0x8000_0009 ^ seed as u64);
                scramble(c, &mut r);
                let rider = seed % 3;
                let ch = seed % 4;
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = ch as u64;
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(econ_base(rider, ch) + 0xac4, r.next_u32());
            },
        );
    }

    #[test]
    fn be_econ_int_get_attribute_cost_matches() {
        check(
            ssx3_ee::s01::f_00150e50,
            be_econ_int_get_attribute_cost,
            100,
            &|c, seed| {
                let mut r = Rng(0x9000_000a ^ seed as u64);
                scramble(c, &mut r);
                let idx = seed % 4;
                c.g[A1][0] = idx as u64;
                c.bus.write32(0x440550 + idx * 8 + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn be_econ_int_get_instance_matches() {
        check(
            ssx3_ee::s01::f_001508d8,
            be_econ_int_get_instance,
            90,
            &|c, seed| {
                instance_setup(c, seed, 0x1edc, 0xec0);
            },
        );
    }

    #[test]
    fn be_econ_int_getters_match() {
        for (gen, hand, salt) in [
            (
                ssx3_ee::s01::f_00150960 as fn(&mut Cpu),
                be_econ_int_get_rider_money as fn(&mut Cpu),
                0x0960,
            ),
            (
                ssx3_ee::s01::f_001509f8,
                be_econ_int_get_rider_total_earned,
                0x09f8,
            ),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A1][0] = (seed % 8) as u64;
                fill(c, &mut r, 0x530000, 0x800);
            });
        }
    }

    #[test]
    fn be_econ_int_add_rider_money_matches() {
        check(
            ssx3_ee::s01::f_00150ae0,
            be_econ_int_add_rider_money,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0ae0, 0);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A0][0] = (DATA + 0x400) as u64;
                fill(c, &mut r, 0x530000, 0x800);
            },
        );
    }

    // ------------------------------------------------------------------
    // BE rewards / missions.

    #[test]
    fn reward_get_collect_item_points_matches() {
        check(
            ssx3_ee::s01::f_00151178__generated,
            reward_get_collect_item_points,
            SEEDS,
            &|c, seed| {
                let levels: [u64; 8] = [0, 1, 2, 3, 9, u64::MAX, 0xffff_ffff, 0x8000_0000];
                c.g[A1][0] = levels[seed as usize % levels.len()];
            },
        );
    }

    #[test]
    fn reward_get_mission_points_matches() {
        check(
            ssx3_ee::s01::f_001511b0__generated,
            reward_get_mission_points,
            SEEDS,
            &|c, seed| {
                let levels: [u64; 8] = [0, 1, 2, 3, 4, u64::MAX, 0xffff_ffff, 0x8000_0000];
                c.g[A1][0] = levels[seed as usize % levels.len()];
            },
        );
    }

    #[test]
    fn be_restore_char_profile_equip_flags_matches() {
        check(
            ee::s01::f_001515b8__generated,
            be_restore_char_profile_equip_flags,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1515b8, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x28c, (seed % 4) as u32);
                for k in 0..3u32 {
                    let flags = match (seed + k) % 4 {
                        0 => 0x0004u16,
                        1 => 0x0010,
                        2 => 0x0014,
                        _ => 0x00ff,
                    };
                    c.bus.write16(self_at + 0x290 + 4 * k + 2, flags);
                }
            },
        );
    }

    #[test]
    fn be_set_char_profile_bolt_owned_matches() {
        check(
            ee::s01::f_00151c48__generated,
            be_set_char_profile_bolt_owned,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15_1c48, 0);
                let prof = DATA + 0x1000;
                let table = DATA + 0x2000;
                c.bus.write32(prof + 0x288, table);
                let id = (r.next_u32() % 8) as u32;
                for i in 0..8u32 {
                    let idx: i32 = match (seed as usize + i as usize) % 4 {
                        0 => -1,
                        1 => 0,
                        2 => (i as i32) * 3,
                        _ => 0x7fff,
                    };
                    c.bus.write16(table + 2 * i, idx as u16);
                }
                c.g[A0][0] = sx(prof);
                c.g[A1][0] = sx(id);
                c.g[A2][0] = (seed % 3) as u64;
            },
        );
    }

    // ---------------------------------------------------------- BE profile

    #[test]
    fn be_char_profile_dtor_matches() {
        check(
            ee::s01::f_00151368__generated,
            be_char_profile_dtor,
            60,
            &|c, seed| {
                base(c, seed, 0x1513);
                let flags = if seed % 4 == 0 { 1u32 } else { 0 };
                let obj = if flags == 1 { 0x100 } else { DATA };
                let owned = if seed % 3 == 0 {
                    0
                } else {
                    0x40 + 8 * (seed % 8)
                };
                c.bus.write32(obj + 0x288, owned);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(flags);
            },
        );
    }

    #[test]
    fn be_set_char_state_flag_cheat_matches() {
        check_lohi(
            ee::s01::f_00150b88__generated,
            be_set_char_state_flag_cheat,
            SEEDS,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0b88, 1);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A3][0] = (seed / 4 % 4) as u64;
                c.g[T0][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn be_reset_char_rivalries_matches() {
        check(
            ee::s01::f_001519e0__generated,
            be_reset_char_rivalries,
            SEEDS_2,
            &|c, seed| {
                let _ = base_3(c, seed, 0x1519_e0, 4);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 10) as u64;
                // The character record the loop compares against (`v0` at
                // `0x4406fc + i*0xc`) and the second pointer it reads
                // (`0x440700 + i*0xc`), which decides where the loop stops.
                for i in 0..10u32 {
                    w(c, 0x0044_06fc + 0xc * i, 0);
                    w(c, 0x0044_0700 + 0xc * i, (seed % 9) + 1);
                }
                for i in 0..0x40u32 {
                    w(c, obj + 0xbc2 + 4 * i, 0);
                }
                wb(c, 0x004a_1218, 0);
                wb(c, 0x004a_1219, 1);
                wb(c, 0x004a_121a, 2);
            },
        );
    }
}
