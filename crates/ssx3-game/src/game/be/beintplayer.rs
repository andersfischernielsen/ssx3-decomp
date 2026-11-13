//! `src/be/beintplayer.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::bedbreward::destroy_instance;
pub(crate) use crate::game::be::beintmission::event_check;
pub(crate) use crate::game::be::beintmission::lw;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_CHARACTER;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_PROFILE;
pub(crate) use crate::game::be::beintmission::BUILTIN_DELETE;
pub(crate) use crate::game::be::beintmission::RIDER_STRIDE;
pub(crate) use crate::game::be::beintnetwork::NEW_PLAYER_GP;
pub(crate) use crate::game::be::beintreward::REWARD_BASE;
pub(crate) use crate::game::be::beintreward::REWARD_STRIDE_A;
pub(crate) use crate::game::be::beintreward::REWARD_STRIDE_B;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::render::particle::this_with_delta;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::sound::soundsys::gp_4 as gp;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_5930, be_new_player_int_ctor, "BENewPlayerInt_ctor"),
    (0x0014_59b8, benewplayerint_v01, "BENewPlayerInt_v01"),
    (
        0x0014_5a98,
        benewplayerint_save_slot_defaults,
        "BENewPlayerInt_SaveSlotDefaults",
    ),
    (
        0x0014_5b20,
        be_new_player_int_reset_slots,
        "BENewPlayerInt_ResetSlots",
    ),
    (
        0x0014_5bd8,
        be_new_player_int_restore_defaults,
        "BENewPlayerInt_RestoreDefaults",
    ),
    (
        0x0014_5c38,
        be_new_player_int_get_first_time_wc_flag,
        "BENewPlayerInt_GetFirstTimeWCFlag",
    ),
    (
        0x0014_5cb0,
        be_new_player_int_set_first_time_wc_flag,
        "BENewPlayerInt_SetFirstTimeWCFlag",
    ),
    (
        0x0014_5d38,
        benew_player_int_is_char_profile_flag_clear,
        "BENewPlayerInt_IsCharProfileFlagClear",
    ),
    (
        0x0014_5dd0,
        be_new_player_int_set_char_profile_flag,
        "BENewPlayerInt_SetCharProfileFlag",
    ),
    (
        0x0014_5ef0,
        be_new_player_int_has_event_record,
        "BENewPlayerInt_HasEventRecord",
    ),
    (
        0x0014_5f90,
        be_new_player_int_get_char_state_flag,
        "BENewPlayerInt_GetCharStateFlag",
    ),
    (
        0x0014_6008,
        be_new_player_int_get_rider_state_flag,
        "BENewPlayerInt_GetRiderStateFlag",
    ),
    (
        0x0014_64d0,
        be_new_player_int_get_peak_flag_b,
        "BENewPlayerInt_GetPeakFlagB",
    ),
    (
        0x0014_6d98,
        be_new_player_int_get_char_record_field_27c,
        "BENewPlayerInt_GetCharRecordField27C",
    ),
    (
        0x0014_6e10,
        benewplayerint_set_char_record_field27c,
        "BENewPlayerInt_SetCharRecordField27C",
    ),
    (
        0x0014_6f40,
        be_new_player_int_set_name,
        "BENewPlayerInt_SetName",
    ),
    (
        0x0014_7138,
        be_new_player_int_set_name_by_slot,
        "BENewPlayerInt_SetNameBySlot",
    ),
    (
        0x0014_7170,
        benew_player_int_get_name,
        "BENewPlayerInt_GetName",
    ),
    (
        0x0014_7220,
        be_new_player_int_set_pad_index,
        "BENewPlayerInt_SetPadIndex",
    ),
    (
        0x0014_72c8,
        be_new_player_int_set_pad_index_by_slot,
        "BENewPlayerInt_SetPadIndexBySlot",
    ),
    (
        0x0014_7318,
        be_new_player_int_get_pad_index_by_slot,
        "BENewPlayerInt_GetPadIndexBySlot",
    ),
    (
        0x0014_7338,
        be_new_player_int_set_character,
        "BENewPlayerInt_SetCharacter",
    ),
    (
        0x0014_73d0,
        benew_player_int_set_alt_character,
        "BENewPlayerInt_SetAltCharacter",
    ),
    (
        0x0014_7448,
        be_new_player_int_set_character_by_slot,
        "BENewPlayerInt_SetCharacterBySlot",
    ),
    (
        0x0014_7488,
        be_new_player_int_get_character_by_slot,
        "BENewPlayerInt_GetCharacterBySlot",
    ),
    (
        0x0014_74a8,
        benewplayerint_set_alt_character_by_slot,
        "BENewPlayerInt_SetAltCharacterBySlot",
    ),
    (
        0x0014_74c8,
        benewplayerint_get_alt_character_by_slot,
        "BENewPlayerInt_GetAltCharacterBySlot",
    ),
    (
        0x0014_74e8,
        be_new_player_int_get_vibration,
        "BENewPlayerInt_GetVibration",
    ),
    (
        0x0014_7528,
        be_new_player_int_set_vibration,
        "BENewPlayerInt_SetVibration",
    ),
    (
        0x0014_7580,
        be_new_player_int_get_slot_flag3,
        "BENewPlayerInt_GetSlotFlag3",
    ),
    (
        0x0014_75c0,
        be_new_player_int_set_slot_flag3,
        "BENewPlayerInt_SetSlotFlag3",
    ),
    (
        0x0014_7618,
        be_new_player_int_get_control_map,
        "BENewPlayerInt_GetControlMap",
    ),
    (
        0x0014_7658,
        be_new_player_int_set_control_map,
        "BENewPlayerInt_SetControlMap",
    ),
    (
        0x0014_7828,
        be_new_player_int_share_slot0_char_record,
        "BENewPlayerInt_ShareSlot0CharRecord",
    ),
    (
        0x0014_7898,
        benew_player_int_clear_shared_char_record,
        "BENewPlayerInt_ClearSharedCharRecord",
    ),
    (
        0x0014_7908,
        be_new_player_int_get_inbox,
        "BENewPlayerInt_GetInbox",
    ),
    (
        0x0014_7980,
        benew_player_int_is_message_flag_set,
        "BENewPlayerInt_IsMessageFlagSet",
    ),
    (
        0x0014_7cb8,
        be_new_player_int_get_difficulty,
        "BENewPlayerInt_GetDifficulty",
    ),
    (
        0x0014_7d20,
        benewplayerint_raise_difficulty,
        "BENewPlayerInt_RaiseDifficulty",
    ),
    (
        0x0014_7e18,
        benewplayerint_lower_difficulty,
        "BENewPlayerInt_LowerDifficulty",
    ),
    (0x0014_7fd8, be_char_int_commit, "BECharInt_Commit"),
    (0x0014_8098, be_char_int_reset, "BECharInt_Reset"),
    (0x0014_83a0, be_char_int_set_attrib0, "BECharInt_SetAttrib0"),
    (0x0014_8470, be_char_int_set_attrib1, "BECharInt_SetAttrib1"),
    (0x0014_8540, be_char_int_set_attrib2, "BECharInt_SetAttrib2"),
    (0x0014_8610, be_char_int_set_attrib3, "BECharInt_SetAttrib3"),
    (0x0014_86e0, be_char_int_set_attrib4, "BECharInt_SetAttrib4"),
    (0x0014_87b0, be_char_int_set_attrib5, "BECharInt_SetAttrib5"),
    (0x0014_8880, be_char_int_set_attrib6, "BECharInt_SetAttrib6"),
    (
        0x0014_8aa8,
        be_char_int_set_all_attribs,
        "BECharInt_SetAllAttribs",
    ),
    (
        0x0014_8b78,
        be_char_int_get_attrib0_by_char,
        "BECharInt_GetAttrib0ByChar",
    ),
    (
        0x0014_8bc8,
        be_char_int_get_attrib1_by_char,
        "BECharInt_GetAttrib1ByChar",
    ),
    (
        0x0014_8c18,
        be_char_int_get_attrib2_by_char,
        "BECharInt_GetAttrib2ByChar",
    ),
    (
        0x0014_8c68,
        be_char_int_get_attrib3_by_char,
        "BECharInt_GetAttrib3ByChar",
    ),
    (
        0x0014_8cb8,
        be_char_int_get_attrib4_by_char,
        "BECharInt_GetAttrib4ByChar",
    ),
    (
        0x0014_8d08,
        be_char_int_get_attrib5_by_char,
        "BECharInt_GetAttrib5ByChar",
    ),
    (
        0x0014_8d58,
        be_char_int_get_attrib6_by_char,
        "BECharInt_GetAttrib6ByChar",
    ),
    (
        0x0015_bc88,
        be_new_player_int_destroy_instance,
        "BENewPlayerInt_DestroyInstance",
    ),
    (
        0x0015_c678,
        be_char_int_destroy_instance,
        "BECharInt_DestroyInstance",
    ),
    (
        0x0014_77e8,
        be_new_player_int_is_sharing_char_record,
        "BENewPlayerInt_IsSharingCharRecord",
    ),
    (
        0x0014_8950,
        be_char_int_get_overall_rating,
        "BECharInt_GetOverallRating",
    ),
    (
        0x0014_5e68,
        be_new_player_int_char_profile_call523e8,
        "BENewPlayerInt_CharProfileCall523e8",
    ),
    (
        0x0014_7410,
        be_new_player_int_get_alt_character,
        "BENewPlayerInt_GetAltCharacter",
    ),
];

// ---------------------------------------------------------------------------
// Direct-call targets (the generated body's `c.pc = addr; Name(c)` calls).

pub(crate) mod callee {

    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE_3;

    pub(crate) use crate::game::camera::camera::callee::BENEWPLAYER_GET_SLOT;

    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;

    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_SLOT_3;

    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;

    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_GET_SLOT as BENEWPLAYER_GET_SLOT_2;

    pub(crate) use crate::game::fe::festaterewards::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE_5;

    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;

    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT as BE_NEW_PLAYER_INT_GET_SLOT_2;

    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;

    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
}

// ---------------------------------------------------------------------------
// Player slots. `BENewPlayerInt_GetSlot` (0x147798) computes
// `0x534fe0 + slot*28`; these functions inline that arithmetic.

/// `void BENewPlayerInt_SetPadIndexBySlot(BENewPlayerInt *self, int pad,
/// int value)` (0x1472c8): store `value` in the player's pad-index word and,
/// when it is non-negative, force bit 0 of the flag word at +0x10 to `pad & 1`
/// (clearing bits 0 and 2 first).
pub fn be_new_player_int_set_pad_index_by_slot(c: &mut Cpu) {
    c.cycles += 8;
    let pad = c.g[A1][0];
    let value = c.g[A2][0];
    c.g[A3][0] = pad;
    let scaled = abi::mult(c, pad as u32, 0x1c);
    c.g[A0][0] = sx(scaled);
    let base = 0x53_0000u32.wrapping_add(0x4fe0); // 0x534fe0
    c.g[V1][0] = sx(base);
    let player = base.wrapping_add(scaled);
    c.g[V0][0] = sx(player);
    Addr(player).off(0xc).sw(c, value);
    if (value as i64) >= 0 {
        c.cycles += 9;
        c.g[A2][0] = sx(player);
        c.g[A1][0] = 0xffff_ffff_ffff_fffe;
        let flags = Addr(player).off(0x10).lw(c);
        let bit = c.g[A3][0] & 1;
        c.g[A0][0] = 0xffff_ffff_ffff_fffb;
        let updated = ((flags & 0xffff_ffff_ffff_fffe) | bit) & 0xffff_ffff_ffff_fffb;
        c.g[V1][0] = bit;
        c.g[V0][0] = updated;
        Addr(player).off(0x10).sw(c, updated);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int BENewPlayerInt_GetPadIndexBySlot(BENewPlayerInt *self, int pad)`
/// (0x147318): the player's pad-index word (slot `pad`, record size 28).
pub fn be_new_player_int_get_pad_index_by_slot(c: &mut Cpu) {
    c.cycles += 7;
    let pad = abi::arg(c, 1);
    let scaled = abi::mult(c, pad, 0x1c);
    c.g[A1][0] = sx(scaled);
    let base = 0x53_0000u32.wrapping_add(0x4fe0); // 0x534fe0
    let player = base.wrapping_add(scaled);
    c.g[V1][0] = sx(player);
    c.g[V0][0] = Addr(player).off(0xc).lw(c);
    abi::ret(c);
}

/// `void BENewPlayerInt_SetCharacterBySlot(BENewPlayerInt *self, int pad,
/// int character)` (0x147448): if the player's character slot is unlocked
/// (bit 2 of the word at +0x10 clear) store `character` at +0x11, otherwise
/// copy the character already at the base record's +0x11.
pub fn be_new_player_int_set_character_by_slot(c: &mut Cpu) {
    c.cycles += 11;
    let pad = abi::arg(c, 1);
    let character = c.g[A2][0];
    let scaled = abi::mult(c, pad, 0x1c);
    let base = 0x53_0000u32.wrapping_add(0x4fe0); // 0x534fe0
    c.g[A0][0] = sx(base);
    c.g[V1][0] = 0x1;
    let player = base.wrapping_add(scaled);
    c.g[A1][0] = sx(player);
    let flags = Addr(player).off(0x10).lw(c);
    let locked = ((flags as u32) >> 2) & 1;
    c.g[V0][0] = locked as u64;
    if locked != 1 {
        c.cycles += 2;
        Addr(player).off(0x11).sb(c, character);
    } else {
        c.cycles += 3;
        let copied = Addr(base).off(0x11).lbu(c);
        c.g[V0][0] = copied;
        Addr(player).off(0x11).sb(c, copied);
    }
    abi::ret(c);
}

/// `int BENewPlayerInt_GetCharacterBySlot(BENewPlayerInt *self, int pad)`
/// (0x147488): the player's character byte at +0x11.
pub fn be_new_player_int_get_character_by_slot(c: &mut Cpu) {
    c.cycles += 7;
    let pad = abi::arg(c, 1);
    let scaled = abi::mult(c, pad, 0x1c);
    c.g[V1][0] = 0x1c;
    let base = 0x53_0000u32.wrapping_add(0x4fe0); // 0x534fe0
    let player = base.wrapping_add(scaled);
    c.g[V0][0] = sx(base);
    c.g[A1][0] = sx(player);
    c.g[V0][0] = Addr(player).off(0x11).lb(c);
    abi::ret(c);
}

/// The base of the character-attribute table.
pub(crate) const CHAR_ATTRIB: u32 = 0x0053_0000;

/// Offset of the character-attribute table within its base.
pub(crate) const CHAR_ATTRIB_OFF: u32 = 0x5538;

/// Shared body of the `BECharInt_GetAttribNByChar` accessors (0x148b78+2n):
/// index `char` into the attribute table (`char*70 + char2*7`) and return the
/// byte at `+field`.
pub(crate) fn be_char_int_get_attrib(c: &mut Cpu, field: u32) {
    c.cycles += 10;
    c.g[V0][0] = 0x46;
    c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
    c.mult(0, A1, A1, V0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
    c.g[V0][0] = CHAR_ATTRIB as u64;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(CHAR_ATTRIB_OFF));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(field)).lb(c);
    abi::ret(c);
}

/// `int BECharInt_GetAttrib0ByChar(int char, int char2)` (0x148b78).
pub fn be_char_int_get_attrib0_by_char(c: &mut Cpu) {
    be_char_int_get_attrib(c, 0x0);
}

/// `int BECharInt_GetAttrib1ByChar(int char, int char2)` (0x148bc8).
pub fn be_char_int_get_attrib1_by_char(c: &mut Cpu) {
    be_char_int_get_attrib(c, 0x1);
}

/// `int BECharInt_GetAttrib2ByChar(int char, int char2)` (0x148c18).
pub fn be_char_int_get_attrib2_by_char(c: &mut Cpu) {
    be_char_int_get_attrib(c, 0x2);
}

/// `int BECharInt_GetAttrib3ByChar(int char, int char2)` (0x148c68).
pub fn be_char_int_get_attrib3_by_char(c: &mut Cpu) {
    be_char_int_get_attrib(c, 0x3);
}

/// `int BECharInt_GetAttrib4ByChar(int char, int char2)` (0x148cb8).
pub fn be_char_int_get_attrib4_by_char(c: &mut Cpu) {
    be_char_int_get_attrib(c, 0x4);
}

/// `int BECharInt_GetAttrib5ByChar(int char, int char2)` (0x148d08).
pub fn be_char_int_get_attrib5_by_char(c: &mut Cpu) {
    be_char_int_get_attrib(c, 0x5);
}

/// `int BECharInt_GetAttrib6ByChar(int char, int char2)` (0x148d58).
pub fn be_char_int_get_attrib6_by_char(c: &mut Cpu) {
    be_char_int_get_attrib(c, 0x6);
}

// ---------------------------------------------------------------------------
// Singleton destructors: free the singleton and clear its gp slot.

/// `void BECharInt_DestroyInstance(void)` (0x15c678).
pub fn be_char_int_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1f2c));
        frame.save_ra(c, 0);
        call_ptr(c, BUILTIN_DELETE, 0x0015_c68c)?;
        frame.restore_ra(c, 0);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f2c), 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BECharInt attribute bytes. The character state is `char * 0x46` plus the
// character's record (0x530000 + 0x5538 + 8*char - char); each attribute is one
// byte at +index.

/// The shared prologue/epilogue of `BECharInt_SetAttrib1..6` (the accumulator
/// ends in v0); `index` is the attribute byte offset (1..6).
pub(crate) fn set_attrib(c: &mut Cpu, entry: u64, index: u32, r1: u32, r2: u32) -> Flow {
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S2, 0x10);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0);
    c.cycles += entry;
    c.g[A0][0] = c.g[S1][0];
    BE_GET_RIDER_PROFILE.call(c, r1)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[S0][0] = c.g[V0][0];
    BE_GET_RIDER_CHARACTER.call(c, r2)?;
    c.g[A0][0] = 0x46;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 3);
    c.mult(0, S0, S0, A0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.g[V0][0] = 0x530000;
    frame.restore(c, S1, 0x20);
    c.g[V0][0] = sx(0x530000u32.wrapping_add(0x5538));
    frame.restore_ra(c, 0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    frame.restore(c, S0, 0x30);
    c.sb((c.g[V0][0] as u32).wrapping_add(index), c.g[S2][0]);
    frame.restore(c, S2, 0x10);
    frame.close(c);
    Ok(())
}

/// `void BECharInt_SetAttrib0(void *self, int rider, int value)` (0x1483a0).
pub fn be_char_int_set_attrib0(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.cycles += 27;
        c.g[A0][0] = c.g[S1][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0014_83c4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0014_83d0)?;
        c.g[A0][0] = 0x46;
        c.g[V1][0] = sx((c.g[V0][0] as u32) << 3);
        c.mult(0, S0, S0, A0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = 0x530000;
        frame.restore(c, S1, 0x20);
        c.g[V0][0] = sx(0x530000u32.wrapping_add(0x5538));
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore(c, S0, 0x30);
        c.sb(c.g[V1][0] as u32, c.g[S2][0]);
        frame.restore(c, S2, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void BECharInt_SetAttrib1(void *self, int rider, int value)` (0x148470).
pub fn be_char_int_set_attrib1(c: &mut Cpu) {
    guest(c, |c| set_attrib(c, 27, 1, 0x0014_8494, 0x0014_84a0))
}

/// `void BECharInt_SetAttrib2(void *self, int rider, int value)` (0x148540).
pub fn be_char_int_set_attrib2(c: &mut Cpu) {
    guest(c, |c| set_attrib(c, 27, 2, 0x0014_8564, 0x0014_8570))
}

/// `void BECharInt_SetAttrib3(void *self, int rider, int value)` (0x148610).
pub fn be_char_int_set_attrib3(c: &mut Cpu) {
    guest(c, |c| set_attrib(c, 27, 3, 0x0014_8634, 0x0014_8640))
}

/// `void BECharInt_SetAttrib4(void *self, int rider, int value)` (0x1486e0).
pub fn be_char_int_set_attrib4(c: &mut Cpu) {
    guest(c, |c| set_attrib(c, 27, 4, 0x0014_8704, 0x0014_8710))
}

/// `void BECharInt_SetAttrib5(void *self, int rider, int value)` (0x1487b0).
pub fn be_char_int_set_attrib5(c: &mut Cpu) {
    guest(c, |c| set_attrib(c, 27, 5, 0x0014_87d4, 0x0014_87e0))
}

/// `void BECharInt_SetAttrib6(void *self, int rider, int value)` (0x148880).
pub fn be_char_int_set_attrib6(c: &mut Cpu) {
    guest(c, |c| set_attrib(c, 27, 6, 0x0014_88a4, 0x0014_88b0))
}

// ---------------------------------------------------------------------------
// Character resets and bulk attribute writes.

pub(crate) const SET_ATTRIB0: Callee = Callee {
    addr: 0x0014_83a0,
    f: ssx3_ee::s01::f_001483a0,
};

pub(crate) const SET_ATTRIB1: Callee = Callee {
    addr: 0x0014_8470,
    f: ssx3_ee::s01::f_00148470,
};

pub(crate) const SET_ATTRIB2: Callee = Callee {
    addr: 0x0014_8540,
    f: ssx3_ee::s01::f_00148540,
};

pub(crate) const SET_ATTRIB3: Callee = Callee {
    addr: 0x0014_8610,
    f: ssx3_ee::s01::f_00148610,
};

pub(crate) const SET_ATTRIB4: Callee = Callee {
    addr: 0x0014_86e0,
    f: ssx3_ee::s01::f_001486e0,
};

pub(crate) const SET_ATTRIB5: Callee = Callee {
    addr: 0x0014_87b0,
    f: ssx3_ee::s01::f_001487b0,
};

pub(crate) const SET_ATTRIB6: Callee = Callee {
    addr: 0x0014_8880,
    f: ssx3_ee::s01::f_00148880,
};

/// `void BECharInt_SetAllAttribs(void *self, int rider, int value)` (0x148aa8):
/// write `value*5` to all seven attribute bytes.
pub fn be_char_int_set_all_attribs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx((c.g[A2][0] as u32) << 2);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[S2][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (callee, ret) in [
            (SET_ATTRIB1, 0x0014_8ad4u32),
            (SET_ATTRIB3, 0x0014_8ae4),
            (SET_ATTRIB0, 0x0014_8af4),
            (SET_ATTRIB4, 0x0014_8b04),
            (SET_ATTRIB5, 0x0014_8b14),
            (SET_ATTRIB6, 0x0014_8b24),
            (SET_ATTRIB2, 0x0014_8b34),
        ] {
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S0][0];
            callee.call(c, ret)?;
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BECharInt_Reset(void)` (0x148098): fill the ten 0xf88-byte character
/// records with three 7-byte constants.
pub fn be_char_int_reset(c: &mut Cpu) {
    c.cycles += 10;
    c.g[A0][0] = sx(0x530000u32.wrapping_add(0x5538));
    c.g[A1][0] = sx(0x4a0000u32.wrapping_add(0x7887));
    c.g[V1][0] = sx(0x4b0000u32.wrapping_add(0x13d7));
    c.g[V0][0] = sx(0x4c0000u32.wrapping_sub(0x50d9));
    c.g[A2][0] = 0x9;
    loop {
        c.cycles += 36;
        for (src_reg, dst_off) in [(A1, 0x0u32), (V1, 0x46u32), (V0, 0x8cu32)] {
            let src = c.g[src_reg][0] as u32;
            c.lwl(A3, src.wrapping_add(3));
            c.lwr(A3, src);
            c.g[T0][0] = c.lb(src.wrapping_add(4));
            c.g[T1][0] = c.lb(src.wrapping_add(5));
            c.g[T2][0] = c.lb(src.wrapping_add(6));
            c.swl(A3, (c.g[A0][0] as u32).wrapping_add(dst_off + 3));
            c.swr(A3, (c.g[A0][0] as u32).wrapping_add(dst_off));
            c.sb((c.g[A0][0] as u32).wrapping_add(dst_off + 4), c.g[T0][0]);
            c.sb((c.g[A0][0] as u32).wrapping_add(dst_off + 5), c.g[T1][0]);
            c.sb((c.g[A0][0] as u32).wrapping_add(dst_off + 6), c.g[T2][0]);
            if src_reg == A1 {
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
            }
        }
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(RIDER_STRIDE));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(RIDER_STRIDE));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(RIDER_STRIDE));
        let t = (c.g[A2][0] as i64) >= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(7));
        if t {
            event_check(c, 0x0014_80c0);
            continue;
        } else {
            c.cycles += 2;
            c.pc = c.g[RA][0] as u32;
            return;
        }
    }
}

/// `BENewPlayerInt_DestroyInstance()` (0x15bc88): as above for
/// `gp - 0x1f30`.
pub fn be_new_player_int_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, NEW_PLAYER_GP, 0x0015_bc9c);
}

// ---------------------------------------------------------------------------
// New-player slots.

/// `BENewPlayerInt_GetSlot` (0x147798): the slot index for a rider id.
pub(crate) const GET_SLOT: Callee = Callee {
    addr: 0x0014_7798,
    f: ssx3_ee::s01::f_00147798,
};

/// The player-slot array base (`0x530000 + 0x4fe0`), stride 0x1c.
pub(crate) const PLAYER_SLOTS: u32 = 0x0053_4fe0;

pub(crate) const PLAYER_SLOT_STRIDE: u32 = 0x1c;

/// `BENewPlayerInt_SetNameBySlot(self, rider, name)` (0x147138): copy 12 bytes
/// of the name into the rider's slot (`strncpy`).
pub fn be_new_player_int_set_name_by_slot(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = PLAYER_SLOT_STRIDE as u64;
        c.g[A0][0] = PLAYER_SLOTS as u64;
        c.mult(0, V1, A1, V0);
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0xc;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(PLAYER_SLOTS));
        call_ptr(c, 0x0041_6ce0, 0x0014_7160)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetPadIndex(self, rider, pad)` (0x147220): store the pad
/// index at slot+0xc, and (when non-negative) tag the low flag bits with its
/// parity.
pub fn be_new_player_int_set_pad_index(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        GET_SLOT.call(c, 0x0014_7238)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = PLAYER_SLOTS as u64;
        c.g[V0][0] = PLAYER_SLOT_STRIDE as u64;
        c.mult(0, A2, A0, V0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        let negative = (c.g[S0][0] as i64) < 0;
        Addr(c.g[V0][0] as u32 + 0xc).sw(c, c.g[S0][0]);
        if !negative {
            c.cycles += 9;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = 0xffff_ffff_ffff_fffe;
            c.g[V0][0] = Addr(c.g[A2][0] as u32 + 0x10).lw(c);
            c.g[V1][0] = c.g[A0][0] & 0x1;
            c.g[A0][0] = 0xffff_ffff_ffff_fffb;
            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
            Addr(c.g[A2][0] as u32 + 0x10).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetCharacter(self, rider, character)` (0x147338): store the
/// character at slot+0x11, unless the "character locked" flag (bit 2 of the
/// slot flags) is set, in which case keep the slot's default byte.
pub fn be_new_player_int_set_character(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        GET_SLOT.call(c, 0x0014_7350)?;
        c.g[V1][0] = PLAYER_SLOT_STRIDE as u64;
        c.g[A0][0] = PLAYER_SLOTS as u64;
        c.mult(0, V0, V0, V1);
        c.g[V1][0] = 1;
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(PLAYER_SLOTS));
        c.g[V0][0] = Addr(c.g[A1][0] as u32 + 0x10).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        if c.g[V0][0] != c.g[V1][0] {
            Addr(c.g[A1][0] as u32 + 0x11).sb(c, c.g[S0][0]);
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(PLAYER_SLOTS + 0x11).lbu(c);
            Addr(c.g[A1][0] as u32 + 0x11).sb(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_GetVibration(self, rider)` (0x1474e8): bit 1 of the slot
/// flags at +0x10.
pub fn be_new_player_int_get_vibration(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        GET_SLOT.call(c, 0x0014_74f8)?;
        c.g[A0][0] = PLAYER_SLOT_STRIDE as u64;
        c.g[V1][0] = PLAYER_SLOTS as u64;
        c.mult(0, V0, V0, A0);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(PLAYER_SLOTS));
        c.g[V0][0] = Addr(c.g[V0][0] as u32 + 0x10).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetVibration(self, rider, on)` (0x147528): write bit 1 of
/// the slot flags from `on & 1`.
pub fn be_new_player_int_set_vibration(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        GET_SLOT.call(c, 0x0014_7540)?;
        c.g[V1][0] = PLAYER_SLOT_STRIDE as u64;
        c.g[A0][0] = PLAYER_SLOTS as u64;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = 0xffff_ffff_ffff_fffd;
        c.g[S0][0] = c.g[S0][0] & 0x1;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 1);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(PLAYER_SLOTS));
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0x10).lw(c);
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V1][0] = c.g[V1][0] | c.g[S0][0];
        frame.restore(c, S0, 0x10);
        Addr(c.g[V0][0] as u32 + 0x10).sw(c, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_GetControlMap(self, rider)` (0x147618): the signed byte at
/// slot+0x13.
pub fn be_new_player_int_get_control_map(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[S0][0] = PLAYER_SLOTS as u64;
        frame.save_ra(c, 0);
        GET_SLOT.call(c, 0x0014_7634)?;
        c.g[V1][0] = PLAYER_SLOT_STRIDE as u64;
        frame.restore_ra(c, 0);
        c.mult(0, V0, V0, V1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32 + 0x13).lb(c);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetControlMap(self, rider, map)` (0x147658): store the
/// control map byte at slot+0x13.
pub fn be_new_player_int_set_control_map(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        GET_SLOT.call(c, 0x0014_7670)?;
        c.g[A0][0] = PLAYER_SLOT_STRIDE as u64;
        c.g[V1][0] = PLAYER_SLOTS as u64;
        c.mult(0, V0, V0, A0);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(PLAYER_SLOTS));
        Addr(c.g[V0][0] as u32 + 0x13).sb(c, c.g[S0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// New-player object.

/// `BENewPlayerInt_ctor(self)` (0x145930): clear the state at +8, store the
/// name literal at +0xc, `ResetSlots`, then the shared slot initialiser, and
/// return `self`.
pub fn be_new_player_int_ctor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x51d8));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw(c.g[S0][0] as u32 + 8, 0);
        c.sw(c.g[S0][0] as u32 + 0xc, c.g[V0][0]);
        call_ptr(c, 0x0014_5b20, 0x0014_5954)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_5bd8, 0x0014_595c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_ResetSlots(self)` (0x145b20): copy the 0xa0-byte template
/// at 0x535b20 over the 20 player slots at 0x534fe0 (plus the 8-byte tail),
/// with the original's alignment check.
pub fn be_new_player_int_reset_slots(c: &mut Cpu) {
    const SRC: u32 = 0x0053_5b20;
    const DST: u32 = 0x0053_4fe0;
    const END: u32 = SRC + 0xa0;
    c.cycles += 8;
    c.g[V0][0] = 0x0053_0000;
    c.g[V1][0] = 0x0053_0000;
    c.g[A0][0] = DST as u64;
    c.g[V1][0] = SRC as u64;
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] = c.g[V0][0] & 7;
    let use_ldl = c.g[V0][0] != 0;
    c.g[V0][0] = sx(END);
    if use_ldl {
        loop {
            c.cycles += 19;
            for (r, k) in [(5usize, 7u32), (6, 0xf), (7, 0x17), (8, 0x1f)] {
                c.ldl(r, (c.g[V1][0] as u32).wrapping_add(k));
                c.ldr(r, (c.g[V1][0] as u32).wrapping_add(k - 7));
            }
            for (r, k) in [(5usize, 7u32), (6, 0xf), (7, 0x17), (8, 0x1f)] {
                c.sdl(r, (c.g[A0][0] as u32).wrapping_add(k));
                c.sdr(r, (c.g[A0][0] as u32).wrapping_add(k - 7));
            }
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_5b40);
                }
            } else {
                c.cycles += 2;
                break;
            }
        }
    } else {
        loop {
            c.cycles += 11;
            c.g[A1][0] = c.ld(c.g[V1][0] as u32);
            c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(8));
            c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
            c.sd(c.g[A0][0] as u32, c.g[A1][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(8), c.g[A2][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_5b94);
            }
        }
    }
    c.cycles += 5;
    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(7));
    c.ldr(2, c.g[V1][0] as u32);
    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(7));
    c.sdr(2, c.g[A0][0] as u32);
    abi::ret(c);
}

/// `BENewPlayerInt_GetFirstTimeWCFlag(self, rider)` (0x145c38): the first-time
/// world-circuit flag for `rider` from the reward array.
pub fn be_new_player_int_get_first_time_wc_flag(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a0e0, 0x0014_5c54)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, 0x0014_7798, 0x0014_5c60)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x0053_0000;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = PLAYER_SLOTS as u64;
        c.g[V1][0] = REWARD_STRIDE_A as u64;
        c.g[A2][0] = REWARD_STRIDE_B as u64;
        c.mult(0, S1, S1, V1);
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(REWARD_BASE);
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.restore_ra(c, 0);
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0x11).lb(c);
        c.mult(0, V1, V1, A2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetFirstTimeWCFlag(self, rider, value)` (0x145cb0): store
/// the first-time world-circuit flag.
pub fn be_new_player_int_set_first_time_wc_flag(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a0e0, 0x0014_5cd4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, 0x0014_7798, 0x0014_5ce0)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x0053_0000;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = PLAYER_SLOTS as u64;
        c.g[V1][0] = REWARD_STRIDE_A as u64;
        c.g[A2][0] = REWARD_STRIDE_B as u64;
        c.mult(0, S1, S1, V1);
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(REWARD_BASE);
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.restore_ra(c, 0);
        c.g[V1][0] = Addr(c.g[V0][0] as u32 + 0x11).lb(c);
        c.mult(0, V1, V1, A2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        frame.restore(c, S1, 0x20);
        Addr(c.g[V1][0] as u32).sw(c, c.g[S2][0]);
        frame.restore(c, S2, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetName(self, rider, name)` (0x146f40): copy 12 bytes of
/// `name` into the rider's slot.
pub fn be_new_player_int_set_name(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        call_ptr(c, 0x0014_7798, 0x0014_6f58)?;
        c.g[V1][0] = PLAYER_SLOT_STRIDE as u64;
        c.g[A0][0] = PLAYER_SLOTS as u64;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0xc;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(PLAYER_SLOTS));
        call_ptr(c, 0x0041_6ce0, 0x0014_6f78)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `D_00534FE0`: the BE player table (0x1c-byte slots; the alternate
/// character byte is at +0x12).
pub(crate) const PLAYER_TABLE: u32 = 0x0053_4fe0;

/// `void BENewPlayerInt_SetAltCharacterBySlot(int unused, int slot, s8 c)`
/// (0x1474a8): the alternate-character byte of a player slot (0x1c-byte slots
/// in `D_00534FE0`, byte at +0x12).
pub fn benewplayerint_set_alt_character_by_slot(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x1c;
    c.g[V0][0] = 0x53_0000;
    c.mult(0, A1, A1, V1); // a1 = slot * 0x1c
    c.g[V0][0] = sx(PLAYER_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A1][0] = sx(base.wrapping_add(c.g[A1][0] as u32));
    let slotp = c.g[A1][0] as u32;
    let ch = c.g[A2][0];
    c.sb(slotp.wrapping_add(0x12), ch);
    abi::ret(c);
}

/// `s8 BENewPlayerInt_GetAltCharacterBySlot(int unused, int slot)` (0x1474c8):
/// the alternate-character byte of a player slot.
pub fn benewplayerint_get_alt_character_by_slot(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x1c;
    c.g[V0][0] = 0x53_0000;
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = sx(PLAYER_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A1][0] = sx(base.wrapping_add(c.g[A1][0] as u32));
    let slotp = c.g[A1][0] as u32;
    c.g[V0][0] = c.lb(slotp.wrapping_add(0x12));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// BE getters/setters.

/// `int BENewPlayerInt_GetSlotFlag3(this, int index)` (0x147580): bit 3 of
/// the 32-byte player-character record at `&D_00534FE0 + id * 0x1c + 0x10`,
/// where `id` is the slot's player id.
pub fn be_new_player_int_get_slot_flag3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        callee::BENEWPLAYER_GET_SLOT.call(c, 0x0014_7590)?;
        c.g[A0][0] = 0x1c;
        c.g[V1][0] = 0x0053_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx(0x0053_4fe0);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
        c.g[V0][0] = c.g[V0][0] & 1;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front end and BE wrappers.

/// `void BENewPlayerInt_SetAltCharacter(int index, int value)` (0x1473d0):
/// store `value` in alternates' character byte (`D_00534FE0[id].+0x12`).
pub fn benew_player_int_set_alt_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        callee::BE_NEW_PLAYER_INT_GET_SLOT.call(c, 0x0014_73e8)?;
        c.g[A0][0] = 0x1c;
        c.g[V1][0] = 0x53_0000;
        // `mult v0, v0, a0`: the per-player stride.
        c.mult(0, 2, 2, 4);
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_add(0x4fe0));
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(crate::abi::r32(c, V0).wrapping_add(crate::abi::r32(c, V1)));
        Addr(p(crate::abi::r32(c, V0), 0x12)).sb(c, c.g[S0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void BENewPlayerInt_SetSlotFlag3(BENewPlayerInt* p_2, int index, int on)`
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
    c.g[V1][0] = Addr(p_2(c, V0, 0x10)).lw(c);
    c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
    c.g[V1][0] = c.g[V1][0] | c.g[S0][0];
    frame.restore(c, S0, 0x10);
    c.sw(p_2(c, V0, 0x10), c.g[V1][0]);
    frame.close(c);
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

// ---------------------------------------------------------------------------
// Beast character controller.

/// `void BECharInt_Commit(void)` (0x147fd8): publish the ten control blocks
/// of the three beast characters from the source table at `0x535538`
/// (three rows of 0x7 bytes + 3 trailing bytes) to `0x4a7887`, `0x4b13d7`
/// and `0x4baf27` (stride 0x7 per iteration, three rows +0x0/+0x46/+0x8c).
pub fn be_char_int_commit(c: &mut Cpu) {
    c.cycles += 10;
    c.g[A0][0] = 0x4a0000;
    c.g[A1][0] = 0x530000;
    c.g[V1][0] = 0x4b0000;
    c.g[V0][0] = 0x4c0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x50d9));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x13d7));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7887));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5538));
    c.g[A2][0] = 0x9;
    loop {
        c.cycles += 36;
        let (a0, a1, v0, v1) = (r32(c, A0), r32(c, A1), r32(c, V0), r32(c, V1));
        c.lwl(7, a1.wrapping_add(0x3));
        c.lwr(7, a1);
        c.g[T0][0] = c.lb(a1.wrapping_add(0x4));
        c.g[T1][0] = c.lb(a1.wrapping_add(0x5));
        c.g[T2][0] = c.lb(a1.wrapping_add(0x6));
        c.swl(7, a0.wrapping_add(0x3));
        c.swr(7, a0);
        c.sb(a0.wrapping_add(0x4), c.g[T0][0]);
        c.sb(a0.wrapping_add(0x5), c.g[T1][0]);
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        c.sb(a0.wrapping_add(0x6), c.g[T2][0]);
        c.lwl(7, a1.wrapping_add(0x49));
        c.lwr(7, a1.wrapping_add(0x46));
        c.g[T0][0] = c.lb(a1.wrapping_add(0x4a));
        c.g[T1][0] = c.lb(a1.wrapping_add(0x4b));
        c.g[T2][0] = c.lb(a1.wrapping_add(0x4c));
        c.swl(7, v1.wrapping_add(0x3));
        c.swr(7, v1);
        c.sb(v1.wrapping_add(0x4), c.g[T0][0]);
        c.sb(v1.wrapping_add(0x5), c.g[T1][0]);
        c.sb(v1.wrapping_add(0x6), c.g[T2][0]);
        c.g[A0][0] = sx(a0.wrapping_add(0xf88));
        c.lwl(7, a1.wrapping_add(0x8f));
        c.lwr(7, a1.wrapping_add(0x8c));
        c.g[T0][0] = c.lb(a1.wrapping_add(0x90));
        c.g[T1][0] = c.lb(a1.wrapping_add(0x91));
        c.g[T2][0] = c.lb(a1.wrapping_add(0x92));
        c.swl(7, v0.wrapping_add(0x3));
        c.swr(7, v0);
        c.sb(v0.wrapping_add(0x4), c.g[T0][0]);
        c.sb(v0.wrapping_add(0x5), c.g[T1][0]);
        c.sb(v0.wrapping_add(0x6), c.g[T2][0]);
        c.g[V1][0] = sx(v1.wrapping_add(0xf88));
        c.g[V0][0] = sx(v0.wrapping_add(0xf88));
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[A1][0] = sx(a1.wrapping_add(0x7));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_8000);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

#[inline(always)]
pub(crate) fn lh(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lh(c)
}

/// `int BENewPlayerInt_GetDifficulty(int which, void* player)` (0x147cb8):
/// the rider's character row (`D_004A6CA8 + id*0xF88`) holds two shorts; take
/// `+0x284` when `which` is zero and `+0x280` otherwise.
pub fn be_new_player_int_get_difficulty(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let which = c.g[A1][0];
        c.g[S0][0] = which;
        call_ptr(c, 0x0014_a080, 0x0014_7cd0)?;
        c.g[A0][0] = c.g[V0][0];
        if which != 0 {
            c.cycles += 6;
            c.g[V1][0] = 0xf88;
            c.g[V0][0] = 0x004a_0000;
            c.mult(0, V1, A0, V1);
            c.g[V0][0] = 0x004a_6ca8;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = lh(c, p(c.g[V1][0] as u32, 0x280));
        } else {
            c.cycles += 7;
            c.g[V1][0] = 0xf88;
            c.g[V0][0] = 0x004a_0000;
            c.mult(0, V1, A0, V1);
            c.g[V0][0] = 0x004a_6ca8;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = lh(c, p(c.g[V1][0] as u32, 0x284));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BENewPlayerInt_GetRiderStateFlag(BENewPlayerInt* self, int rider,
/// int flag)` (0x146008): resolve profile and character, then read the
/// character-state bit.
pub fn be_new_player_int_get_rider_state_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x50);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[A2][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0014_6034);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a0e0, 0x0014_6034)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0014_6040);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a080, 0x0014_6040)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x0014_6054);
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x0014_5f90, 0x0014_6054)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x50);
        Ok(())
    })
}

/// `void BENewPlayerInt_ShareSlot0CharRecord(int index)` (0x147828): copy
/// rider 0's character id into the player slot's record and set its shared
/// flag.
pub fn be_new_player_int_share_slot0_char_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x1c;
        call_ptr(c, 0x0014_7798, 0x0014_7848)?;
        c.mult(0, 2, 2, 18);
        c.g[S0][0] = 0x534f_e0;
        c.g[A0][0] = c.g[S1][0];
        c.g[V1][0] = Addr(at(c, S0, 0x11)).lbu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        Addr(at(c, V0, 0x11)).sb(c, c.g[V1][0]);
        call_ptr(c, 0x0014_7798, 0x0014_7868)?;
        c.mult(0, 3, 2, 18);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V1][0] = Addr(at(c, V0, 0x10)).lw(c);
        frame.restore(c, S0, 0x30);
        c.g[V1][0] = c.g[V1][0] | 4;
        Addr(at(c, V0, 0x10)).sw(c, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void BENewPlayerInt_ClearSharedCharRecord(int idx, int player)`
/// (0x147898): clear the character id and the "custom" flag of the player's
/// shared record.
pub fn benew_player_int_clear_shared_char_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 17, 0x20);
        c.g[S1][0] = c.g[A1][0];
        sqsp(c, 16, 0x30);
        sqsp(c, 18, 0x10);
        c.g[A0][0] = c.g[S1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x1c;
        call::call_ptr(c, 0x0014_7798, 0x0014_78bc)?;
        c.mult(0, V0, V0, S2);
        c.g[S0][0] = 0x0053_4fe0;
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = abi::addu(c.g[V0][0], c.g[S0][0]);
        Addr(p_3(c.g[V0][0] as u32, 0x11)).sb(c, 0);
        call::call_ptr(c, 0x0014_7798, 0x0014_78d8)?;
        c.mult(0, V1, V0, S2);
        c.g[A0][0] = 0xffff_ffff_ffff_fffb;
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.g[V0][0] = abi::addu(c.g[V1][0], c.g[S0][0]);
        c.g[V1][0] = Addr(p_3(c.g[V0][0] as u32, 0x10)).lw(c);
        lqsp(c, 16, 0x30);
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        Addr(p_3(c.g[V0][0] as u32, 0x10)).sw(c, c.g[V1][0]);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `lb off(reg)` (sign-extended).
#[inline]
pub(crate) fn lb_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lb(c)
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
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[A0][0] = c.g[S0][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_6db4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        callee::BE_NEW_PLAYER_INT_GET_SLOT_2.call(c, 0x0014_6dc0)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x53_0000;
        c.mult(0, V0, V0, V1); // lo = slot * 0x1c
        c.g[A1][0] = sx(low(c, A1).wrapping_add(0x4fe0));
        c.g[V1][0] = 0x9b50;
        c.g[A2][0] = 0xf88;
        c.mult(0, S1, S1, V1); // lo = profile * 0x9b50
        c.g[A0][0] = 0x4a_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6ca8));
        c.lq(16, sp_2(c).wrapping_add(0x20));
        c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, A1)));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        c.g[V1][0] = lb_r(c, V0, 0x11);
        c.mult(0, V1, V1, A2); // lo = charid * 0xf88
        c.g[V1][0] = sx(low(c, V1).wrapping_add(low(c, S1)));
        c.g[V1][0] = sx(low(c, V1).wrapping_add(low(c, A0)));
        c.lq(17, sp_2(c).wrapping_add(0x10));
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
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[A0][0] = c.g[S0][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_7924)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        callee::BE_NEW_PLAYER_INT_GET_SLOT_2.call(c, 0x0014_7930)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x53_0000;
        c.mult(0, V0, V0, V1); // lo = slot * 0x1c
        c.g[A1][0] = sx(low(c, A1).wrapping_add(0x4fe0));
        c.g[V1][0] = 0x9b50;
        c.g[A2][0] = 0xf88;
        c.mult(0, S1, S1, V1); // lo = profile * 0x9b50
        c.g[A0][0] = 0x4a_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6ca8));
        c.lq(16, sp_2(c).wrapping_add(0x20));
        c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, A1)));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        c.g[V1][0] = lb_r(c, V0, 0x11);
        c.mult(0, V1, V1, A2); // lo = charid * 0xf88
        c.g[V1][0] = sx(low(c, V1).wrapping_add(low(c, A0)));
        c.g[S1][0] = sx(low(c, S1).wrapping_add(low(c, V1)));
        c.g[V0][0] = sx(low(c, S1).wrapping_add(0xe38));
        c.lq(17, sp_2(c).wrapping_add(0x10));
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `mult rd, rs, rt` (LO/HI of pipeline 0, `rd = lo`).
#[inline(always)]
pub(crate) fn mul32(c: &mut Cpu, rd: usize, rs: usize, rt: usize) {
    c.mult(0, rd, rs, rt);
}

/// `lq reg, off(sp)` at the current `sp`.
#[inline(always)]
pub(crate) fn frame_restore_at(c: &mut Cpu, reg: usize, off: u32) {
    c.lq(reg, p(sp(c), off));
}

/// `void BENewPlayerInt_SaveSlotDefaults()` (0x145a98): copy the twelve
/// character-record slots into the save slots, preserving the "new" flag.
pub fn benewplayerint_save_slot_defaults(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x530000;
        frame.save(c, 16, 0x40);
        c.g[V1][0] = 0x530000;
        frame.save(c, 17, 0x30);
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4fe0));
        frame.save(c, 18, 0x20);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b20));
        frame.save(c, 19, 0x10);
        c.g[S2][0] = 0xffff_ffff_ffff_fffd;
        frame.save_ra(c, 0);
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa8));
        loop {
            c.cycles += 14;
            c.g[V0][0] = lw_at(c, p(r32(c, S1), 0x10));
            c.g[A0][0] = c.g[S1][0];
            c.g[V1][0] = lw_at(c, p(r32(c, S0), 0x10));
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] &= c.g[S2][0];
            c.g[A2][0] = 0xc;
            c.g[V1][0] &= 2;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] |= c.g[V1][0];
            c.sw(p(r32(c, S1), 0x10), c.g[V0][0]);
            call_ptr(c, callee::STR_N_COPY, 0x0014_5af4)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S3][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_5ac8);
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BENewPlayerInt_SetCharRecordField27C(player, value)` (0x146e10):
/// store `value` in the rider's character record at `+0x27c`.
pub fn benewplayerint_set_char_record_field27c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::BE_GET_RIDER_PROFILE_2.addr, 0x0014_6e34)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, callee::BE_NEW_PLAYER_INT_GET_SLOT.addr, 0x0014_6e40)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x530000;
        mul32(c, V0, V0, V1);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4fe0));
        c.g[V1][0] = 0x9b50;
        c.g[A2][0] = 0xf88;
        mul32(c, S1, S1, V1);
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
        frame.restore(c, 16, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.restore_ra(c, 0);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x11)).lb(c);
        mul32(c, V1, V1, A2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        frame_restore_at(c, 17, 0x20);
        c.sw(p(c.g[V1][0] as u32, 0x27c), c.g[S2][0]);
        frame_restore_at(c, 18, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `bool BENewPlayerInt_IsCharProfileFlagClear(void* be, int index, int bit)`
/// (0x145d38): whether the character-profile flag `bit` is clear.
pub fn benew_player_int_is_char_profile_flag_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::BE_GET_RIDER_PROFILE_3.call(c, 0x0014_5d5c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        callee::BENEW_PLAYER_INT_GET_SLOT.call(c, 0x0014_5d68)?;
        c.g[V1][0] = 0x1c;
        c.g[A2][0] = 0x530000;
        c.mult(0, V0, V0, V1);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4fe0));
        c.g[V1][0] = 0x9b50;
        c.g[A3][0] = 0xf88;
        c.mult(0, S0, S0, V1);
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
        c.g[A1][0] = 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32) << ((c.g[S2][0] as u32) & 31));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x11).lb(c);
        frame.restore(c, S1, 0x20);
        c.mult(0, V1, V1, A3);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xacc).lw(c);
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `BENewPlayerInt_SetCharProfileFlag` (0x145dd0): set or clear one player's
/// character-profile flag (a2 = flag bit, a3 selects set/clear).
pub fn be_new_player_int_set_char_profile_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0014_5dfc);
        c.g[A0][0] = c.g[S0][0];
        callee::BE_GET_RIDER_PROFILE_4.call(c, 0x0014_5dfc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0014_5e08);
        c.g[S1][0] = c.g[V0][0];
        callee::BENEW_PLAYER_INT_GET_SLOT_2.call(c, 0x0014_5e08)?;
        c.g[V1][0] = 0x1c;
        c.g[A0][0] = 0x0053_0000;
        c.mult(0, 2, 2, 3);
        c.g[A0][0] = sx(0x0053_0000u32.wrapping_add(0x4fe0));
        c.g[T0][0] = 0xf88;
        c.g[V1][0] = 0x9b50;
        c.mult(0, 17, 17, 3);
        c.g[A3][0] = 0x004a_0000;
        c.g[A3][0] = sx(0x004a_0000u32.wrapping_add(0x6ca8));
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
        c.g[A2][0] = c.g[S3][0] ^ 1;
        c.g[A0][0] = Addr(p(r(c, V0), 0x11)).lb(c);
        c.mult(0, 4, 4, 8);
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A3)));
        c.g[RA][0] = sx(0x0014_5e4c);
        c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, A0)));
        callee::BE_SET_CHAR_PROFILE_FLAG.call(c, 0x0014_5e4c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool BENewPlayerInt_HasEventRecord(rider, a, b, c)` (0x145ef0): look the
/// rider's character profile up in the score-stat table and report whether
/// the event's record byte is set.
pub fn be_new_player_int_has_event_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_5f1c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        callee::BE_NEW_PLAYER_INT_GET_SLOT_3.call(c, 0x0014_5f28)?;
        c.g[V1][0] = 0x1c;
        c.g[A0][0] = 0x0053_0000;
        c.mult(0, V0, V0, V1);
        c.g[A0][0] = 0x0053_4fe0;
        c.g[T0][0] = 0xf88;
        c.g[V1][0] = 0x9b50;
        c.mult(0, S1, S1, V1);
        c.g[A3][0] = 0x004a_6ca8;
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A2][0] = c.g[S3][0];
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x11).lb(c);
        c.mult(0, A0, A0, T0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        callee::BE_GET_CHAR_PROFILE_EVENT_RECORD.call(c, 0x0014_5f6c)?;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lb(c);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `BENewPlayerInt_GetName` (0x147170): the player-name interface: if the
// fixed 28-byte name slot for `id` is empty, format the profile's name into it
// and copy it to ASCII; either way return the slot.
pub fn benew_player_int_get_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x120);
        c.g[V1][0] = 0x1c;
        frame.save(c, S3, 0xe0);
        c.g[V0][0] = 0x530000;
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0xf0);
        c.mult(0, 3, 19, 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4fe0));
        frame.save(c, S0, 0x110);
        frame.save(c, S1, 0x100);
        frame.save_ra(c, 0xd0);
        let name = (c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32);
        c.g[S2][0] = sx(name);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x00416810, 0x0014_71a8)?;
        let has_name = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S2][0];
        if !has_name {
            c.cycles += 20;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5d98));
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, 0x00317670, 0x0014_71d0)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
            c.g[A0][0] = c.g[S0][0];
            let target = c.g[V1][0] as u32;
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, target, 0x0014_71e0)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            call_ptr(c, 0x002c26d0, 0x0014_71f0)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x002c2580, 0x0014_71fc)?;
            c.g[V0][0] = c.g[S2][0];
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x110);
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// `int BENewPlayerInt_IsMessageFlagSet(self, int index, int bit)` (0x147980):
/// test one bit of the per-profile/per-character message-flag bitmap.
pub fn benew_player_int_is_message_flag_set(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        let index = c.g[A1][0];
        c.g[S1][0] = index;
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = index;
        call_ptr(c, 0x0014_a0e0, 0x0014_79a4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_7798, 0x0014_79b0)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x530000;
        c.mult(0, 2, 2, 3);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4fe0));
        c.g[A0][0] = 0x9b50;
        c.g[A2][0] = 0xf88;
        c.mult(0, 16, 16, 4);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S2][0] as i64)) as u64;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1f));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.g[S2][0];
        }
        c.g[A1][0] = Addr(p(c.g[V0][0] as u32, 0x11)).lb(c);
        c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 5) as u32);
        c.g[V0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.mult(0, 5, 5, 6);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
        c.g[V1][0] = 0x1;
        frame.restore(c, S1, 0x20);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S2][0] as u32) & 31));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        frame.restore(c, S2, 0x10);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0xf08)).lw(c);
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ret;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// New player defaults.

/// `void BENewPlayerInt_v01(NewPlayer* self, int a1)` (0x1459b8): copy the
/// default player block into the global slot, or save the defaults when there
/// is no global slot.
pub fn benewplayerint_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x10));
        c.g[V1][0] = Addr(gp(c, -0x1f30)).lw(c);
        c.sd(r32(c, SP), c.g[RA][0]);
        c.g[V0][0] = Addr(p(r32(c, V1), 8)).lw(c);
        let save = c.g[V0][0] == 0;
        c.g[V0][0] = 0x530000;
        if save {
            c.cycles += 2;
            call_ptr(c, 0x0014_5a98, 0x0014_5a88)?;
        } else {
            c.cycles += 7;
            c.g[V1][0] = 0x530000;
            c.g[A0][0] = sx(p(r32(c, V0), 0x5b20));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x4fe0));
            c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
            c.g[V0][0] = c.g[V0][0] & 7;
            let aligned = c.g[V0][0] == 0;
            c.g[V0][0] = sx(p(r32(c, V1), 0xa0));
            if !aligned {
                loop {
                    c.cycles += 19;
                    c.ldl(5, p(r32(c, V1), 7));
                    c.ldr(5, r32(c, V1));
                    c.ldl(6, p(r32(c, V1), 0xf));
                    c.ldr(6, p(r32(c, V1), 8));
                    c.ldl(7, p(r32(c, V1), 0x17));
                    c.ldr(7, p(r32(c, V1), 0x10));
                    c.ldl(8, p(r32(c, V1), 0x1f));
                    c.ldr(8, p(r32(c, V1), 0x18));
                    c.sdl(5, p(r32(c, A0), 7));
                    c.sdr(5, r32(c, A0));
                    c.sdl(6, p(r32(c, A0), 0xf));
                    c.sdr(6, p(r32(c, A0), 8));
                    c.sdl(7, p(r32(c, A0), 0x17));
                    c.sdr(7, p(r32(c, A0), 0x10));
                    c.sdl(8, p(r32(c, A0), 0x1f));
                    c.sdr(8, p(r32(c, A0), 0x18));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x20));
                    let more = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x20));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0014_59ec);
                    }
                }
            }
            loop {
                c.cycles += 11;
                c.g[A1][0] = c.ld(r32(c, V1));
                c.g[A2][0] = c.ld(p(r32(c, V1), 8));
                c.g[A3][0] = c.ld(p(r32(c, V1), 0x10));
                c.g[T0][0] = c.ld(p(r32(c, V1), 0x18));
                c.sd(r32(c, A0), c.g[A1][0]);
                c.sd(p(r32(c, A0), 8), c.g[A2][0]);
                c.sd(p(r32(c, A0), 0x10), c.g[A3][0]);
                c.sd(p(r32(c, A0), 0x18), c.g[T0][0]);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x20));
                let more = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_5a40);
                }
            }
            c.cycles += 5;
            c.ldl(2, p(r32(c, V1), 7));
            c.ldr(2, r32(c, V1));
            c.sdl(2, p(r32(c, A0), 7));
            c.sdr(2, r32(c, A0));
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(r32(c, SP));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE new-player flags.

/// `void BENewPlayerInt_SetMessageFlag(char* self, void* rider, int idx,
/// int set)` (0x147a30): set (or clear) the "new player message" bit for the
/// rider's slot in the profile's flags table.
pub fn benewplayer_int_set_message_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0);

        c.g[A0][0] = c.g[S1][0];
        callee::BE_GET_RIDER_PROFILE_5.call(c, 0x0014_7a5c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::BENEWPLAYER_GET_SLOT_2.call(c, 0x0014_7a68)?;

        // entry = D_004A6CA8 + profile * 0x9b50 + slot * 0xf88
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x530000;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = sx(r(c, A1).wrapping_add(0x4fe0));
        c.g[A0][0] = 0x9b50;
        c.g[A2][0] = 0xf88;
        c.mult(0, S0, S0, A0);
        c.g[V1][0] = u64::MAX;
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S2][0] as i64)) as u64;
        c.g[A0][0] = sx(p(r(c, S2), 0x1f));
        c.g[V0][0] = sx(p(r(c, V0), r(c, A1)));
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.g[S2][0];
        }
        c.g[A1][0] = c.lb(p(r(c, V0), 0x11));
        c.g[A0][0] = sx((r(c, A0) as i32 >> 5) as u32);
        c.g[V0][0] = 0x4a0000;
        c.g[V1][0] = sx(r(c, A0) << 5);
        c.mult(0, A1, A1, A2);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(0x6ca8));
        c.g[S2][0] = sx(r(c, S2).wrapping_sub(r(c, V1)));
        c.g[S0][0] = sx(p(r(c, S0), r(c, A1)));
        c.g[S0][0] = sx(p(r(c, S0), r(c, V0)));
        let clear = c.g[S3][0] == 0;
        c.g[S0][0] = sx(p(r(c, S0), 0xf08));
        c.cycles += 7;
        c.g[A0][0] = sx(r(c, A0) << 2);
        c.g[V1][0] = 1;
        c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
        c.g[V1][0] = sx(r(c, V1) << (r(c, S2) & 31));
        c.g[V0][0] = Addr(r(c, A0)).lw(c);
        if clear {
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        } else {
            c.g[V1][0] = !(0u64 | c.g[V1][0]);
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        }
        c.cycles += 8;
        c.sw(r(c, A0), c.g[V0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00147D20(void* unused, int which)` (0x147d20): step the current
/// rider's difficulty up: `a` (a 16-bit counter) increments and wraps to 0 at
/// 2, in which case `b` (bounded to 2) follows; the `which` pair of the
/// character record at `D_004A6CA8 + profile*0x9b50 + character*0xf88` is
/// updated.
pub fn benewplayerint_raise_difficulty(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0;
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0014_7d40);
        c.g[S1][0] = 0; // profile
        c.pc = 0x0014_a080;
        ee::s01::BE_GetRiderCharacter(c);
        if c.pc != 0x0014_7d40 {
            return Err(Unwound);
        }
        let second = c.g[S0][0] == 0;
        c.g[A2][0] = c.g[V0][0]; // character id
        if second {
            c.cycles += 8;
            c.g[V0][0] = 0xf88;
            c.g[V1][0] = 0x4a_0000;
            c.mult(0, 2, 6, 2);
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
            c.g[T1][0] = c.g[V1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A1][0] = c.lh(p(c.g[V0][0] as u32, 0x286)); // b1
            c.g[A3][0] = c.lh(p(c.g[V0][0] as u32, 0x284)); // a1
        } else {
            c.cycles += 9;
            c.g[V0][0] = 0xf88;
            c.g[V1][0] = 0x4a_0000;
            c.mult(0, 2, 6, 2);
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
            c.g[T1][0] = c.g[V1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A1][0] = c.lh(p(c.g[V0][0] as u32, 0x282)); // b0
            c.g[A3][0] = c.lh(p(c.g[V0][0] as u32, 0x280)); // a0
        }
        // a++; at 2 wrap to 0 and step b (up to 2)
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
        c.g[V1][0] = ((c.g[A1][0] as i64) < 2) as u64;
        let a_wrapped = c.g[V1][0] == 0;
        c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
        if a_wrapped {
            c.cycles += 2;
            let b_at_max = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            if !b_at_max {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
            }
            c.cycles += 1;
            c.g[A1][0] = 0;
        }
        // write the pair back
        c.cycles += 2;
        let second_store = c.g[S0][0] == 0;
        c.g[V0][0] = 0xf88;
        if second_store {
            c.cycles += 8;
            c.g[V1][0] = 0x9b50;
            c.mult(0, 2, 6, 2);
            c.mult(0, 3, 17, 3);
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x6ca8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sh(p(c.g[V0][0] as u32, 0x286), c.g[A1][0]);
            c.sh(p(c.g[V0][0] as u32, 0x284), c.g[A3][0]);
        } else {
            c.cycles += 9;
            c.g[V1][0] = 0x9b50;
            c.mult(0, 2, 6, 2);
            c.mult(0, 3, 17, 3);
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x6ca8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sh(p(c.g[V0][0] as u32, 0x282), c.g[A1][0]);
            c.sh(p(c.g[V0][0] as u32, 0x280), c.g[A3][0]);
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00147E18(void* unused, int which)` (0x147e18): the mirror of
/// [`benewplayerint_raise_difficulty`]: `a` counts down and wraps to 0 below
/// -1, in which case `b` (bounded to 0) follows.
pub fn benewplayerint_lower_difficulty(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0;
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0014_7e38);
        c.g[S1][0] = 0; // profile
        c.pc = 0x0014_a080;
        ee::s01::BE_GetRiderCharacter(c);
        if c.pc != 0x0014_7e38 {
            return Err(Unwound);
        }
        let second = c.g[S0][0] == 0;
        c.g[A2][0] = c.g[V0][0];
        if second {
            c.cycles += 8;
            c.g[V0][0] = 0xf88;
            c.g[V1][0] = 0x4a_0000;
            c.mult(0, 2, 6, 2);
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
            c.g[T1][0] = c.g[V1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A1][0] = c.lh(p(c.g[V0][0] as u32, 0x286));
            c.g[A3][0] = c.lh(p(c.g[V0][0] as u32, 0x284));
        } else {
            c.cycles += 9;
            c.g[V0][0] = 0xf88;
            c.g[V1][0] = 0x4a_0000;
            c.mult(0, 2, 6, 2);
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
            c.g[T1][0] = c.g[V1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A1][0] = c.lh(p(c.g[V0][0] as u32, 0x282));
            c.g[A3][0] = c.lh(p(c.g[V0][0] as u32, 0x280));
        }
        // a--; below -1 wrap to 0 and step b (down to 0)
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
        c.g[V1][0] = ((c.g[A1][0] as i64) < -1) as u64;
        let a_under = c.g[V1][0] != 0;
        if a_under {
            c.cycles += 2;
            let b_at_zero = c.g[A3][0] == 0;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
            if !b_at_zero {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
            }
            c.cycles += 1;
            c.g[A1][0] = 0;
        }
        // write the pair back
        c.cycles += 2;
        let second_store = c.g[S0][0] == 0;
        c.g[V0][0] = 0xf88;
        if second_store {
            c.cycles += 8;
            c.g[V1][0] = 0x9b50;
            c.mult(0, 2, 6, 2);
            c.mult(0, 3, 17, 3);
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x6ca8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sh(p(c.g[V0][0] as u32, 0x286), c.g[A1][0]);
            c.sh(p(c.g[V0][0] as u32, 0x284), c.g[A3][0]);
        } else {
            c.cycles += 9;
            c.g[V1][0] = 0x9b50;
            c.mult(0, 2, 6, 2);
            c.mult(0, 3, 17, 3);
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x6ca8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sh(p(c.g[V0][0] as u32, 0x282), c.g[A1][0]);
            c.sh(p(c.g[V0][0] as u32, 0x280), c.g[A3][0]);
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BENewPlayerInt_GetPeakFlagB(void* self, int index, int c, int kind,
/// int flag)` (0x1464d0): one bit of a character record's flags word
/// (`D_004A6CA8 + profile*0x9b50 + c*0xf88 + 0x278`), bit 6/7/8 of `kind`
/// 0/1/2 when `flag`, bits 9/10/11 otherwise. The profile index comes from
/// `cBELibrary_getProfileIndex(index)`.
pub fn be_new_player_int_get_peak_flag_b(c: &mut Cpu) {
    'b_146684: {
        'b_1465c4: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[A0][0] = c.g[A1][0];
            c.sq(16, c.pa(SP, 0x30));
            c.sq(17, c.pa(SP, 0x20));
            c.g[S0][0] = c.g[T0][0];
            c.sq(18, c.pa(SP, 0x10));
            c.g[S1][0] = c.g[A3][0];
            c.std(SP, 0, c.g[RA][0]);
            c.g[RA][0] = sx(0x001464f8);
            c.g[S2][0] = c.g[A2][0];
            c.pc = 0x0014a0e0;
            call_indirect(c, 0x0014a0e0);
            if c.pc != 0x001464f8 {
                return;
            }
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = c.g[V0][0];
            if t {
                break 'b_1465c4;
            } else {
                'b_146594: {
                    'b_146564: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[S1][0] == c.g[V0][0];
                        c.g[V1][0] = 0xf88;
                        if t {
                            break 'b_146564;
                        } else {
                            'b_146538: {
                                'b_146528: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = 0x2;
                                        break 'b_146528;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == 0u64;
                                        c.g[A0][0] = 0x9b50;
                                        if t {
                                            break 'b_146538;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_146684;
                                            } else {
                                                break 'b_146528;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S1][0] == c.g[V0][0];
                                c.g[V1][0] = 0xf88;
                                if t {
                                    break 'b_146594;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_146684;
                                    } else {
                                        break 'b_146538;
                                    }
                                }
                            }
                            c.cycles += 11;
                            c.mult(0, 3, 18, 3);
                            c.mult(0, 4, 5, 4);
                            c.g[V0][0] = 0x4a0000;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V0][0] = c.ldd(V1, 0x278);
                            c.g[V0][0] = c.g[V0][0] << 26;
                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                            let t = true;
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            if t {
                                break 'b_146684;
                            } else {
                                break 'b_146564;
                            }
                        }
                    }
                    c.cycles += 12;
                    c.g[A0][0] = 0x9b50;
                    c.mult(0, 3, 18, 3);
                    c.mult(0, 4, 5, 4);
                    c.g[V0][0] = 0x4a0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.ldd(V1, 0x278);
                    c.g[V0][0] = c.g[V0][0] << 25;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    if t {
                        break 'b_146684;
                    } else {
                        break 'b_146594;
                    }
                }
                c.cycles += 12;
                c.g[A0][0] = 0x9b50;
                c.mult(0, 3, 18, 3);
                c.mult(0, 4, 5, 4);
                c.g[V0][0] = 0x4a0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = c.ldd(V1, 0x278);
                c.g[V0][0] = c.g[V0][0] << 24;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                let t = true;
                c.g[V0][0] = c.g[V0][0] & 0x1;
                if t {
                    break 'b_146684;
                } else {
                    break 'b_1465c4;
                }
            }
        }
        'b_146658: {
            'b_146628: {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let t = c.g[S1][0] == c.g[V0][0];
                c.g[V1][0] = 0xf88;
                if t {
                    break 'b_146628;
                } else {
                    'b_1465fc: {
                        'b_1465ec: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = 0x2;
                                break 'b_1465ec;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S1][0] == 0u64;
                                c.g[A0][0] = 0x9b50;
                                if t {
                                    break 'b_1465fc;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_146684;
                                    } else {
                                        break 'b_1465ec;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S1][0] == c.g[V0][0];
                        c.g[V1][0] = 0xf88;
                        if t {
                            break 'b_146658;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_146684;
                            } else {
                                break 'b_1465fc;
                            }
                        }
                    }
                    c.cycles += 11;
                    c.mult(0, 3, 18, 3);
                    c.mult(0, 4, 5, 4);
                    c.g[V0][0] = 0x4a0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.ldd(V1, 0x278);
                    c.g[V0][0] = c.g[V0][0] << 23;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    if t {
                        break 'b_146684;
                    } else {
                        break 'b_146628;
                    }
                }
            }
            c.cycles += 12;
            c.g[A0][0] = 0x9b50;
            c.mult(0, 3, 18, 3);
            c.mult(0, 4, 5, 4);
            c.g[V0][0] = 0x4a0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = c.ldd(V1, 0x278);
            c.g[V0][0] = c.g[V0][0] << 22;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            let t = true;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            if t {
                break 'b_146684;
            } else {
                break 'b_146658;
            }
        }
        c.cycles += 11;
        c.g[A0][0] = 0x9b50;
        c.mult(0, 3, 18, 3);
        c.mult(0, 4, 5, 4);
        c.g[V0][0] = 0x4a0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = c.ldd(V1, 0x278);
        c.g[V0][0] = c.g[V0][0] << 21;
        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        break 'b_146684;
    }
    c.cycles += 6;
    c.lq(16, c.pa(SP, 0x30));
    c.lq(17, c.pa(SP, 0x20));
    c.lq(18, c.pa(SP, 0x10));
    c.g[RA][0] = c.ldd(SP, 0);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    return;
}

// ---------------------------------------------------------------------------
// BENewPlayerInt_IsSharingCharRecord (0x1477e8, beintnewrace.cpp).

/// `BENewPlayerInt_GetSlot(player)` (same shard).
pub(crate) const BE_NEW_PLAYER_INT_GET_SLOT: Callee = Callee {
    addr: 0x0014_7798,
    f: ee::s01::BENewPlayerInt_GetSlot,
};

/// Per-slot character records (0x1c bytes each); flags word at +0x10, bit 2
/// "uses slot 0's character record".
pub(crate) const CHAR_RECORDS: u32 = 0x0053_4fe0;

pub(crate) const CHAR_RECORD_SIZE: u32 = 0x1c;

pub(crate) const CHAR_RECORD_FLAGS: u32 = 0x10;

/// `bool BENewPlayerInt_IsSharingCharRecord(int player)` (0x1477e8): whether
/// the character record of the player's race slot is shared with slot 0
/// (flags bit 2). 15 cycles plus the slot lookup; LO/HI keep the record
/// offset (`mult`), v1 the table base.
pub fn be_new_player_int_is_sharing_char_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        BE_NEW_PLAYER_INT_GET_SLOT.call(c, 0x0014_77f8)?;
        c.g[A0][0] = CHAR_RECORD_SIZE as u64;
        c.g[V1][0] = sx(CHAR_RECORDS);
        c.mult(0, V0, V0, A0); // v0 = slot * 0x1c
        frame.restore_ra(c, 0);
        let record = (c.g[V0][0] as u32).wrapping_add(CHAR_RECORDS);
        let flags = c.lw(record.wrapping_add(CHAR_RECORD_FLAGS)) as u32;
        c.g[V0][0] = ((flags >> 2) & 1) as u64;
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------- BECharInt_GetOverallRating

/// The seven attribute getters `BECharInt_GetOverallRating` averages, in
/// call order, with their return addresses.
pub(crate) const OVERALL_RATING_ATTRIBS: [(Callee, u32); 7] = [
    // BECharInt_GetAttrib1
    (
        Callee {
            addr: 0x0014_84e0,
            f: ee::s01::f_001484e0,
        },
        0x0014_897c,
    ),
    // BECharInt_GetAttrib3
    (
        Callee {
            addr: 0x0014_8680,
            f: ee::s01::f_00148680,
        },
        0x0014_899c,
    ),
    // BECharInt_GetAttrib0
    (
        Callee {
            addr: 0x0014_8410,
            f: ee::s01::f_00148410,
        },
        0x0014_89c0,
    ),
    // BECharInt_GetAttrib4
    (
        Callee {
            addr: 0x0014_8750,
            f: ee::s01::f_00148750,
        },
        0x0014_89e4,
    ),
    // BECharInt_GetAttrib5
    (
        Callee {
            addr: 0x0014_8820,
            f: ee::s01::f_00148820,
        },
        0x0014_8a08,
    ),
    // BECharInt_GetAttrib6
    (
        Callee {
            addr: 0x0014_88f0,
            f: ee::s01::f_001488f0,
        },
        0x0014_8a2c,
    ),
    // BECharInt_GetAttrib2
    (
        Callee {
            addr: 0x0014_85b0,
            f: ee::s01::f_001485b0,
        },
        0x0014_8a50,
    ),
];

/// `int BECharInt_GetOverallRating(BECharInt* self, int rider)` (0x148950):
/// a rider's overall rating, the mean of its seven attribute ratings each
/// divided by 5 (integer divisions: `sum(attr / 5) / 7`), at least 1.
/// Frame 0x60 (s0-s4).
pub fn be_char_int_get_overall_rating(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        let this = c.g[A0][0];
        let rider = c.g[A1][0];
        c.g[S2][0] = this;
        c.g[S3][0] = rider;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 5;
        let (getter, ret) = OVERALL_RATING_ATTRIBS[0];
        getter.call(c, ret)?;
        c.g[S4][0] = 0;
        // attr / 5 (the divisor check `teq s0, s4` never traps).
        c.div(0, V0, S0);

        c.cycles += 10;
        c.g[A0][0] = this;
        c.g[S1][0] = c.lo[0];
        c.g[A1][0] = rider;
        let (getter, ret) = OVERALL_RATING_ATTRIBS[1];
        getter.call(c, ret)?;
        c.div(0, V0, S0);
        c.g[V1][0] = 5;
        c.g[A0][0] = this;
        c.g[A1][0] = rider;

        for &(getter, ret) in &OVERALL_RATING_ATTRIBS[2..] {
            c.cycles += 9;
            c.g[V0][0] = c.lo[0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            getter.call(c, ret)?;
            c.div(0, V0, S0);
            c.g[V1][0] = 5;
            c.g[A0][0] = this;
            c.g[A1][0] = rider;
        }
        // The last block loads 7 (the mean's divisor), 1 (the minimum).
        c.g[V1][0] = 7;
        c.g[A1][0] = 1;
        c.g[A0][0] = 5;

        c.cycles += 15;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.lo[0];
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.div(0, S1, V1);
        let mean = c.lo[0];
        c.g[S1][0] = mean;
        c.g[V0][0] = if 0 < mean as i64 { mean } else { 1 };
        c.g[S4][0] = (0 < mean as i64) as u64;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S4, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int BENewPlayerInt_CharProfileCall523e8(self, int player, int flag)`
/// (0x145e68, beintplayer.cpp): tests a character-profile flag of a
/// player's character: `BE_TestCharProfileFlag(&profiles[profile][char],
/// flag)` with the rider profile index (`BE_GetRiderProfile`), the player's
/// slot (`BENewPlayerInt_GetSlot`) and the slot's character id (signed byte
/// +0x11 of the 0x1c-byte player record at 0x534fe0); the profile table is
/// at 0x4a6ca8 (0x9b50 bytes per profile, 0xf88 per character).
pub fn be_new_player_int_char_profile_call523e8(c: &mut Cpu) {
    const PLAYERS: u32 = 0x0053_4fe0;
    const PROFILES: u32 = 0x004a_6ca8;
    const GET_RIDER_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::f_0014a0e0,
    };
    const GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::f_00147798,
    };
    const TEST_FLAG: Callee = Callee {
        addr: 0x0015_23e8,
        f: ee::s01::f_001523e8,
    };
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        GET_RIDER_PROFILE.call(c, 0x0014_5e8c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = c.g[V0][0];
        GET_SLOT.call(c, 0x0014_5e98)?;
        c.g[V1][0] = 0x1c;
        let slot = r32(c, V0);
        let record = abi::mult(c, slot, 0x1c);
        c.g[V0][0] = sx(record);
        c.g[A3][0] = 0xf88;
        c.g[V1][0] = 0x9b50;
        let profile_index = r32(c, S1);
        let profile = abi::mult(c, profile_index, 0x9b50);
        c.g[S1][0] = sx(profile);
        c.g[A2][0] = sx(PROFILES);
        c.g[A1][0] = c.g[S2][0];
        let record = record.wrapping_add(PLAYERS);
        c.g[V0][0] = sx(record);
        let character = Addr(record).off(0x11).lb(c);
        let character = abi::mult(c, character as u32, 0xf88);
        c.g[A0][0] = sx(profile.wrapping_add(character.wrapping_add(PROFILES)));
        TEST_FLAG.call(c, 0x0014_5ed8)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BENewPlayerInt_GetAltCharacter (0x147410)

/// The player records (0x1c bytes each) and the alternate-character byte.
pub(crate) const BENP_PLAYERS: u32 = 0x0053_4fe0;

pub(crate) const BENP_PLAYER_SIZE: u32 = 0x1c;

pub(crate) const BENP_PLAYER_ALT_CHARACTER: u32 = 0x12;

/// The slot table `BENewPlayerInt_GetSlot` reads (`0x5305b0[index]`).
#[cfg(test)]
pub(crate) const BENP_SLOTS: u32 = 0x0053_05b0;

pub(crate) const BENP_GET_SLOT: Callee = Callee {
    addr: 0x0014_7798,
    f: ee::s01::BENewPlayerInt_GetSlot,
};

/// `s8 BENewPlayerInt_GetAltCharacter(BENewPlayerInt* self, int index)`
/// (0x147410): the alternate character (signed byte +0x12) of the player
/// record the player in slot `index` uses.
pub fn be_new_player_int_get_alt_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        BENP_GET_SLOT.call(c, 0x0014_7420)?;
        c.g[A0][0] = BENP_PLAYER_SIZE as u64;
        c.g[V1][0] = sx(BENP_PLAYERS);
        let slot = c.g[V0][0] as u32;
        let off = abi::mult(c, slot, BENP_PLAYER_SIZE);
        frame.restore_ra(c, 0);
        let rec = off.wrapping_add(BENP_PLAYERS);
        c.g[V0][0] = Addr(rec.wrapping_add(BENP_PLAYER_ALT_CHARACTER)).lb(c);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::generic;

    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::be::beintaggression::be_aggression_int_destroy_instance;
    pub(crate) use crate::game::be::beintbagt::be_bagt_int_destroy_instance;
    pub(crate) use crate::game::be::beintmission::be_mission_int_destroy_instance;
    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base_7;
    pub(crate) use crate::game::bx::bxstringctor::tests::slot_2 as slot_4;

    pub(crate) use crate::game::bx::bxstringctor::tests::VTS;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_5;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_6;

    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_3;

    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup as setup_2;

    pub(crate) use crate::game::main::gamemode::tests::be_library;
    pub(crate) use crate::game::main::gamemode::tests::w;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_4;
    pub(crate) use crate::game::main::gamestate::tests::base_3;

    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::common;

    pub(crate) use crate::game::render::particle::tests::vtable_2;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;

    pub(crate) use crate::game::sound::soundsys::tests::base_9;

    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    pub(crate) const PLAYERS: u32 = 0x53_4fe0;

    #[test]
    fn be_new_player_int_set_pad_index_by_slot_matches() {
        check_lohi(
            ssx3_ee::s01::f_001472c8,
            be_new_player_int_set_pad_index_by_slot,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0009 ^ seed as u64);
                scramble(c, &mut r);
                let slot = seed % 4;
                c.g[A1][0] = slot as u64;
                let mut value = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                if seed % 2 == 0 {
                    value |= 0x8000_0000_0000_0000; // negative: skip the flag update
                } else {
                    value &= 0x7fff_ffff_ffff_ffff;
                }
                c.g[A2][0] = value;
                c.bus.write32(PLAYERS + slot * 28 + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_player_int_get_pad_index_by_slot_matches() {
        check_lohi(
            ssx3_ee::s01::f_00147318,
            be_new_player_int_get_pad_index_by_slot,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_000a ^ seed as u64);
                scramble(c, &mut r);
                let slot = seed % 4;
                c.g[A1][0] = slot as u64;
                c.bus.write32(PLAYERS + slot * 28 + 0xc, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_player_int_set_character_by_slot_matches() {
        check_lohi(
            ssx3_ee::s01::f_00147448,
            be_new_player_int_set_character_by_slot,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_000b ^ seed as u64);
                scramble(c, &mut r);
                let slot = seed % 4;
                c.g[A1][0] = slot as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                let player = PLAYERS + slot * 28;
                // Cover both: bit 2 set (locked -> copy) and clear (store).
                let mut flags = r.next_u32();
                flags = (flags & !0x4) | if seed % 2 == 0 { 0x4 } else { 0 };
                c.bus.write32(player + 0x10, flags);
                c.bus.write8(PLAYERS + 0x11, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn be_new_player_int_get_character_by_slot_matches() {
        check_lohi(
            ssx3_ee::s01::f_00147488,
            be_new_player_int_get_character_by_slot,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_000c ^ seed as u64);
                scramble(c, &mut r);
                let slot = seed % 4;
                c.g[A1][0] = slot as u64;
                c.bus.write8(PLAYERS + slot * 28 + 0x11, r.next_u32() as u8);
            },
        );
    }

    /// Shared setup for the seven attribute accessors: index into the
    /// character-attribute table with small indices and seed the bytes read.
    pub(crate) fn char_attrib_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x148b ^ seed as u64);
        scramble(c, &mut r);
        let a1 = (r.next_u32() % 0x200) as u32;
        let a2 = (r.next_u32() % 8) as u32;
        c.g[A1][0] = a1 as u64;
        c.g[A2][0] = a2 as u64;
        let addr = CHAR_ATTRIB
            .wrapping_add(CHAR_ATTRIB_OFF)
            .wrapping_add(a1.wrapping_mul(70))
            .wrapping_add(a2.wrapping_mul(7));
        for k in 0..8 {
            c.bus.write8(addr + k, r.next_u32() as u8);
        }
    }

    #[test]
    fn be_char_int_get_attrib0_by_char_matches() {
        check_lohi(
            ssx3_ee::s01::f_00148b78,
            be_char_int_get_attrib0_by_char,
            200,
            &char_attrib_setup,
        );
    }

    #[test]
    fn be_char_int_get_attrib1_by_char_matches() {
        check_lohi(
            ssx3_ee::s01::f_00148bc8,
            be_char_int_get_attrib1_by_char,
            200,
            &char_attrib_setup,
        );
    }

    #[test]
    fn be_char_int_get_attrib2_by_char_matches() {
        check_lohi(
            ssx3_ee::s01::f_00148c18,
            be_char_int_get_attrib2_by_char,
            200,
            &char_attrib_setup,
        );
    }

    #[test]
    fn be_char_int_get_attrib3_by_char_matches() {
        check_lohi(
            ssx3_ee::s01::f_00148c68,
            be_char_int_get_attrib3_by_char,
            200,
            &char_attrib_setup,
        );
    }

    #[test]
    fn be_char_int_get_attrib4_by_char_matches() {
        check_lohi(
            ssx3_ee::s01::f_00148cb8,
            be_char_int_get_attrib4_by_char,
            200,
            &char_attrib_setup,
        );
    }

    #[test]
    fn be_char_int_get_attrib5_by_char_matches() {
        check_lohi(
            ssx3_ee::s01::f_00148d08,
            be_char_int_get_attrib5_by_char,
            200,
            &char_attrib_setup,
        );
    }

    #[test]
    fn be_char_int_get_attrib6_by_char_matches() {
        check_lohi(
            ssx3_ee::s01::f_00148d58,
            be_char_int_get_attrib6_by_char,
            200,
            &char_attrib_setup,
        );
    }

    #[test]
    fn be_destroy_instances_match() {
        for (gen, hand, slot, salt) in [
            (
                ssx3_ee::s01::f_0015c678 as fn(&mut Cpu),
                be_char_int_destroy_instance as fn(&mut Cpu),
                0x1f2c,
                0xc67,
            ),
            (
                ssx3_ee::s01::f_0015c6e8,
                be_bagt_int_destroy_instance,
                0x1ee0,
                0xc6e,
            ),
            (
                ssx3_ee::s01::f_0015c7e0,
                be_aggression_int_destroy_instance,
                0x1ea4,
                0xc7e,
            ),
            (
                ssx3_ee::s01::f_0015bcd0,
                be_mission_int_destroy_instance,
                0x1eb8,
                0xbcd,
            ),
        ] {
            check(gen, hand, 80, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                let p = if seed % 2 == 0 {
                    DATA + (seed as u32 % 4) * 0x100
                } else {
                    0
                };
                c.bus.write32(GAME_GP.wrapping_sub(slot), p);
                let _ = &mut r;
            });
        }
    }

    #[test]
    fn be_char_int_set_attrib_matches() {
        for (gen, hand, salt) in [
            (
                ssx3_ee::s01::f_001483a0 as fn(&mut Cpu),
                be_char_int_set_attrib0 as fn(&mut Cpu),
                0x83a0,
            ),
            (ssx3_ee::s01::f_00148470, be_char_int_set_attrib1, 0x8470),
            (ssx3_ee::s01::f_00148540, be_char_int_set_attrib2, 0x8540),
            (ssx3_ee::s01::f_00148610, be_char_int_set_attrib3, 0x8610),
            (ssx3_ee::s01::f_001486e0, be_char_int_set_attrib4, 0x86e0),
            (ssx3_ee::s01::f_001487b0, be_char_int_set_attrib5, 0x87b0),
            (ssx3_ee::s01::f_00148880, be_char_int_set_attrib6, 0x8880),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                // Give the char state region some content.
                fill(c, &mut r, 0x530000, 0x800);
            });
        }
    }

    #[test]
    fn be_char_int_set_all_attribs_matches() {
        check(
            ssx3_ee::s01::f_00148aa8,
            be_char_int_set_all_attribs,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8aa8, 0);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (r.next_u32() as u64 & 0xff) | 0x100;
                fill(c, &mut r, 0x530000, 0x800);
            },
        );
    }

    #[test]
    fn be_char_int_reset_matches() {
        check(
            ssx3_ee::s01::f_00148098,
            be_char_int_reset,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8098, 0);
                fill(c, &mut r, 0x4a0000, 0x28000 / 4);
            },
        );
    }

    /// Point a rider id at a slot index and fill that slot.
    pub(crate) fn slot_setup(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        scramble(c, r);
        c.g[GP][0] = GAME_GP as u64;
        let rider = seed % 4;
        let idx = (seed / 4) % 3;
        c.bus.write32(0x0053_5b0 + rider * 4, idx);
        c.g[A1][0] = rider as u64;
        let base = PLAYER_SLOTS + idx * PLAYER_SLOT_STRIDE;
        for i in 0..8 {
            c.bus.write32(base + i * 4, r.next_u32());
        }
        base
    }

    #[test]
    fn be_new_player_slot_accessors_match() {
        check(
            ssx3_ee::s01::f_00147138,
            be_new_player_int_set_name_by_slot,
            120,
            &|c, seed| {
                let mut r = Rng(0x7138_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let idx = seed % 3;
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = 0x0080_0200u64;
                for i in 0..4 {
                    c.bus.write32(0x0080_0200 + i * 4, r.next_u32());
                }
                for i in 0..8 {
                    c.bus.write32(
                        PLAYER_SLOTS + idx * PLAYER_SLOT_STRIDE + i * 4,
                        r.next_u32(),
                    );
                }
            },
        );
        check(
            ssx3_ee::s01::f_00147220,
            be_new_player_int_set_pad_index,
            200,
            &|c, seed| {
                let mut r = Rng(0x7220_0000 ^ seed as u64);
                slot_setup(c, &mut r, seed);
                c.g[A2][0] = if seed % 2 == 0 {
                    r.next_u32() as u64
                } else {
                    ((r.next_u32() as u64) << 32) | r.next_u32() as u64
                };
            },
        );
        check(
            ssx3_ee::s01::f_00147338,
            be_new_player_int_set_character,
            200,
            &|c, seed| {
                let mut r = Rng(0x7338_0000 ^ seed as u64);
                slot_setup(c, &mut r, seed);
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
        check(
            ssx3_ee::s01::f_001474e8,
            be_new_player_int_get_vibration,
            150,
            &|c, seed| {
                let mut r = Rng(0x74e8_0000 ^ seed as u64);
                slot_setup(c, &mut r, seed);
            },
        );
        check(
            ssx3_ee::s01::f_00147528,
            be_new_player_int_set_vibration,
            150,
            &|c, seed| {
                let mut r = Rng(0x7528_0000 ^ seed as u64);
                slot_setup(c, &mut r, seed);
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
        check(
            ssx3_ee::s01::f_00147618,
            be_new_player_int_get_control_map,
            150,
            &|c, seed| {
                let mut r = Rng(0x7618_0000 ^ seed as u64);
                slot_setup(c, &mut r, seed);
            },
        );
        check(
            ssx3_ee::s01::f_00147658,
            be_new_player_int_set_control_map,
            150,
            &|c, seed| {
                let mut r = Rng(0x7658_0000 ^ seed as u64);
                slot_setup(c, &mut r, seed);
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn be_new_player_object_match() {
        covered(
            ssx3_ee::s01::f_00145930,
            be_new_player_int_ctor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x5930, 1);
                c.g[A0][0] = DATA as u64;
                for i in 0..0x40 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
                // The template ResetSlots copies from.
                for i in 0..0x30 {
                    c.bus.write32(0x0053_5b20 + i * 4, r.next_u32());
                }
            },
        );
        check(
            ssx3_ee::s01::f_00145b20,
            be_new_player_int_reset_slots,
            80,
            &|c, seed| {
                let mut r = Rng(0x5b20_0000 ^ seed as u64);
                scramble(c, &mut r);
                for i in 0..0x30 {
                    c.bus.write32(0x0053_5b20 + i * 4, r.next_u32());
                }
                for i in 0..0x30 {
                    c.bus.write32(0x0053_4fe0 + i * 4, r.next_u32());
                }
            },
        );
        check(
            ssx3_ee::s01::f_00146f40,
            be_new_player_int_set_name,
            80,
            &|c, seed| {
                let mut r = Rng(0x6f40_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                c.bus
                    .write32(0x0053_5b0 + (seed % 4) * 4, (seed % 3) as u32);
                for i in 0..4 {
                    c.bus.write32(DATA + 0x200 + i * 4, r.next_u32());
                }
                for i in 0..0x30 {
                    c.bus.write32(PLAYER_SLOTS + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn be_new_player_first_time_wc_match() {
        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = Rng(0x5c38_0000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A1][0] = 0;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(0x0053_5b0, 0);
            c.bus.write32(PLAYER_SLOTS + 0xc, 0xffff_ffff);
            c.bus.write8(PLAYER_SLOTS + 0x11, 0);
            // profile 2 -> +2*0x9b50.
            for i in 0..8 {
                c.bus
                    .write32(REWARD_BASE + 2 * REWARD_STRIDE_A + i * 4, r.next_u32());
            }
        }
        check(
            ssx3_ee::s01::f_00145c38,
            be_new_player_int_get_first_time_wc_flag,
            80,
            &|c, seed| setup(c, seed),
        );
        check(
            ssx3_ee::s01::f_00145cb0,
            be_new_player_int_set_first_time_wc_flag,
            80,
            &|c, seed| setup(c, seed),
        );
    }

    #[test]
    fn benewplayerint_set_alt_character_by_slot_matches() {
        check_lohi(
            ee::s01::f_001474a8__generated,
            benewplayerint_set_alt_character_by_slot,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0014_74a8, seed);
                scramble(c, &mut r);
                let slot = (seed % 8) as u32;
                c.g[A1][0] = mixed(&mut r, seed, slot);
                c.g[A2][0] = match seed % 4 {
                    0 => 0,
                    1 => 0xff,
                    _ => r.next_u32() as u64,
                };
                c.bus
                    .write8(PLAYER_TABLE + slot * 0x1c + 0x12, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn benewplayerint_get_alt_character_by_slot_matches() {
        check_lohi(
            ee::s01::f_001474c8__generated,
            benewplayerint_get_alt_character_by_slot,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0014_74c8, seed);
                scramble(c, &mut r);
                let slot = (seed % 8) as u32;
                c.g[A1][0] = mixed(&mut r, seed, slot);
                c.bus
                    .write8(PLAYER_TABLE + slot * 0x1c + 0x12, r.next_u32() as u8);
            },
        );
    }

    // -----------------------------------------------------------------------
    // BE getters/setters.

    #[test]
    fn be_new_player_int_get_slot_flag3_matches() {
        check(
            ssx3_ee::s01::f_00147580__generated,
            be_new_player_int_get_slot_flag3,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1475_80, 0);
                let slot = r.next_u32() % 4;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = slot as u64;
                let id = r.next_u32() % 4;
                c.bus.write32(0x0053_05b0 + slot * 4, id);
                for i in 0..4u32 {
                    c.bus.write32(
                        0x0053_4fe0 + i * 0x1c + 0x10,
                        if seed % 3 == 0 { 0 } else { r.next_u32() },
                    );
                }
            },
        );
    }

    // ----------------------------------------------------------- front end

    #[test]
    fn benew_player_int_set_alt_character_matches() {
        check(
            ssx3_ee::s01::f_001473d0__generated,
            benew_player_int_set_alt_character,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0c, 1);
                c.g[A0][0] = 0;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (r.next_u32() & 0xff) as u64;
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

    #[test]
    fn be_new_player_int_restore_defaults_matches() {
        check(
            ee::s01::f_00145bd8__generated,
            be_new_player_int_restore_defaults,
            SEEDS_2,
            &|c, seed| {
                let _ = common(c, seed, 0x145bd8, 1);
                let this = DATA + 0x100;
                let vt = DATA + 0x300;
                clear(c, this, 0x100);
                clear(c, vt, 0x200);
                let _ = put_words(c, 0x0053_5b20, &[0; 6 * 7]);
                c.bus.write32(this + 0xc, vt);
                vtable_2(c, vt, &[(0x10, 0, FAKE)]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `n` random bytes at `a`.
    pub(crate) fn fillb(c: &mut Cpu, r: &mut Rng, a: u32, n: u32) {
        for i in 0..n {
            c.bus.write8(a + i, r.next_u32() as u8);
        }
    }

    // ------------------------------------------------------------------
    // Beast character controller.

    #[test]
    fn be_char_int_commit_matches() {
        check(
            ee::s01::f_00147fd8__generated,
            be_char_int_commit,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 40);
                fillb(c, &mut r, 0x0053_5538, 0x100);
                for i in 0..10u32 {
                    fillb(c, &mut r, 0x004a_7887 + i * 0xf88, 8);
                    fillb(c, &mut r, 0x004b_13d7 + i * 0xf88, 8);
                    fillb(c, &mut r, 0x004b_af27 + i * 0xf88, 8);
                }
            },
        );
    }

    #[test]
    fn be_new_player_int_get_difficulty_matches() {
        check(
            ssx3_ee::s01::f_00147cb8__generated,
            be_new_player_int_get_difficulty,
            80,
            &|c, seed| {
                let _r = setup_2(c, seed, 0x147cb8);
                c.g[A0][0] = 0;
                c.g[A1][0] = u64::from(seed % 2 != 0);
                // A rider character row with both difficulty shorts.
                c.bus.write16(0x004a_6ca8 + 0x280, 1 + (seed % 5) as u16);
                c.bus.write16(0x004a_6ca8 + 0x284, 1 + (seed % 3) as u16);
            },
        );
    }

    #[test]
    fn be_new_player_int_get_rider_state_flag_matches() {
        check(
            ee::s01::f_00146008__generated,
            be_new_player_int_get_rider_state_flag,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x14_6008);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = (seed % 4) as u64;
                // The profile table stores offsets from 0x5305b20, not pointers.
                for i in 0..3u32 {
                    let rec = DATA + 0x100 + 0x40 * i;
                    c.bus
                        .write32(0x0053_05b0 + 4 * i, rec.wrapping_sub(0x0530_5b20));
                    c.bus
                        .write32(rec + 0xc, if (seed + i) % 2 == 0 { 0xffff_ffff } else { 0 });
                    c.bus
                        .write32(rec + 0x10, if (seed + i) % 3 == 0 { 1 } else { 0 });
                    c.bus.write8(rec + 0x11, (r.next_u32() & 7) as u8);
                }
            },
        );
    }

    #[test]
    fn be_new_player_int_share_slot0_char_record_matches() {
        check(
            ee::s01::f_00147828__generated,
            be_new_player_int_share_slot0_char_record,
            80,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1478_28);
                let index = seed % 4;
                let slot = (seed / 4) % 3;
                c.g[A0][0] = sx(index);
                c.bus.write32(0x5305_b0 + index * 4, slot);
                let entry = 0x534f_e0 + slot * 0x1c;
                c.bus.write32(0x534f_e0 + 0x11, r.next_u32());
                c.bus.write32(entry + 0x10, r.next_u32());
                c.bus.write8(entry + 0x11, r.next_u32() as u8);
            },
        );
    }

    pub(crate) fn w8(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    #[test]
    fn benew_player_int_clear_shared_char_record_matches() {
        check(
            ssx3_ee::s01::f_00147898__generated,
            benew_player_int_clear_shared_char_record,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x147898, 0);
                let idx = (seed % 4) as u32;
                c.g[A1][0] = idx as u64;
                for i in 0..4u32 {
                    w32(
                        c,
                        0x0053_55b0 + 4 * i,
                        if i == idx { (seed % 4) as u32 } else { 0 },
                    );
                }
                for slot in 0..4u32 {
                    let base = 0x0053_4fe0 + slot * 0x1c;
                    w32(c, base + 0x10, r.next_u32());
                    w8(c, base + 0x11, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn be_new_player_int_get_char_state_flag_matches() {
        check(
            ee::s01::f_00145f90__generated,
            be_new_player_int_get_char_state_flag,
            SEEDS_4,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn be_new_player_int_get_char_record_field_27c_matches() {
        check(
            ee::s01::f_00146d98__generated,
            be_new_player_int_get_char_record_field_27c,
            SEEDS_4,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn be_new_player_int_get_inbox_matches() {
        check(
            ee::s01::f_00147908__generated,
            be_new_player_int_get_inbox,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x147908, 2);
                be_library(c);
                c.g[A1][0] = (seed % 3) as u64;
                for i in 0..4u32 {
                    w(c, 0x0053_05b0 + 4 * i, r.next_u32() % 4);
                    w(c, 0x0053_5b20 + 4 * i, 0);
                }
            },
        );
    }

    pub(crate) fn r_u32(seed: u32, salt: u32) -> u32 {
        let mut x = seed.wrapping_mul(0x9e37_79b9) ^ salt;
        x ^= x >> 13;
        x.wrapping_mul(0x85eb_ca6b)
    }

    #[test]
    fn benewplayerint_save_slot_defaults_matches() {
        check(
            ee::s01::f_00145a98__generated,
            benewplayerint_save_slot_defaults,
            SEEDS_5,
            &|c, seed| {
                base_3(c, seed, 0x45a98, 0);
                for i in 0..12u32 {
                    let src = 0x0053_4fe0 + 0x1c * i;
                    let dst = 0x0053_5b20 + 0x1c * i;
                    c.bus.write32(src + 0x10, r_u32(seed, i));
                    c.bus.write32(dst + 0x10, r_u32(seed, i + 32));
                    for k in 0..7u32 {
                        c.bus.write32(src + 4 * k, r_u32(seed, i * 8 + k));
                        c.bus.write32(dst + 4 * k, r_u32(seed, i * 8 + k + 64));
                    }
                }
            },
        );
    }

    #[test]
    fn benewplayerint_set_char_record_field27c_matches() {
        check(
            ee::s01::f_00146e10__generated,
            benewplayerint_set_char_record_field27c,
            SEEDS_5,
            &|c, seed| {
                base_3(c, seed, 0x46e10, 0);
                let player = (seed % 4) as u32;
                let profile = ((seed / 4) % 4) as u32;
                let slot_2 = ((seed / 16) % 4) as u32;
                c.g[A1][0] = player as u64;
                c.g[A2][0] = r_u32(seed, 9) as u64;
                c.bus.write32(0x0053_05b0 + 4 * player, profile);
                let rec = 0x0053_5b20 + 0x1c * profile;
                c.bus
                    .write32(rec + 0xc, if seed % 2 == 0 { 0xffff_ffff } else { 0 });
                c.bus.write32(rec + 0x10, seed & 1);
                let slot_base = 0x0053_4fe0 + 0x1c * slot_2;
                for k in 0..7u32 {
                    c.bus.write32(slot_base + 4 * k, r_u32(seed, k));
                }
                c.bus.write8(slot_base + 0x11, (seed % 8) as u8);
                c.bus.write32(0x004a_6ca8 + 0x27c, 0);
            },
        );
    }

    #[test]
    fn benew_player_int_is_char_profile_flag_clear_matches() {
        check_lohi(
            ee::s01::f_00145d38__generated,
            benew_player_int_is_char_profile_flag_clear,
            60,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x145d38, 2);
                let index = (seed % 4) as u32;
                let player = (seed / 4 % 4) as u32;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = index as u64;
                c.g[A2][0] = (seed % 32) as u64;
                // `BE_GetRiderProfile` and `BENewPlayerInt_GetSlot` both read
                // the player id from `0x5305B0 + index*4`; the profile record is
                // at_2 `0x5305B20 + pid*0x1C` and the character table at_2
                // `0x534FE0 + pid*0x1C`.
                w(c, 0x0053_05b0 + 4 * index, player);
                w(
                    c,
                    0x0053_0520 + 0x1c * player + 0xc,
                    if seed % 3 == 0 { 0xffff_ffff } else { 0 },
                );
                w(c, 0x0053_0520 + 0x1c * player + 0x10, (seed / 3 % 2) as u32);
                w8(c, 0x0053_4fe0 + 0x1c * player + 0x11, (seed % 5) as u8);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn be_new_player_int_set_char_profile_flag_matches() {
        check(
            ee::s01::f_00145dd0__generated,
            be_new_player_int_set_char_profile_flag,
            SEEDS_6,
            &|c, seed| {
                let _ = base_5(c, seed, 0x5dd0, 3);
                let player = seed % 4;
                w(c, 0x0053_05b0 + 4 * player, 0);
                c.g[A0][0] = player as u64;
                c.g[A1][0] = (seed & 2) as u64;
                c.g[A2][0] = player as u64;
                c.g[A3][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn be_new_player_int_has_event_record_matches() {
        check_lohi(
            ssx3_ee::s01::f_00145ef0__generated,
            be_new_player_int_has_event_record,
            20,
            &|c, seed| {
                let _r = base_6(c, seed, 0x145ef0, 3);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((seed % 8) + 1) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = ((seed / 5) % 3) as u64;
                for i in 0..1024u32 {
                    c.bus.write8(0x004a_6ca8 + i, seed.wrapping_add(i) as u8);
                }
            },
        );
    }

    #[test]
    fn benew_player_int_get_name_matches() {
        check(
            ee::s01::f_00147170__generated,
            benew_player_int_get_name,
            60,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x1471);
                let idx = (seed % 4) as u32;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = idx as u64;
                let name = 0x0053_4fe0 + idx * 0x1c;
                if seed % 3 == 0 {
                    c.bus.write8(name, 0);
                } else {
                    c.bus.write8(name, b'A');
                    c.bus.write8(name + 1, 0);
                }
                slot_4(c, VTS, 0x20, 0);
                fill(c, &mut r, DATA + 0x800, 0x40);
            },
        );
    }

    #[test]
    fn benew_player_int_is_message_flag_set_matches() {
        // The word index (a0 = bit / 32 * 4) is a scratch register to
        // `check`, and the bitmap word is rarely seeded: compare the scratch
        // registers too (a `li -1` without sign extension went unnoticed).
        for seed in 0..80 {
            let r = crate::difftest::compare(
                ee::s01::f_00147980__generated,
                benew_player_int_is_message_flag_set,
                &|c| {
                    let _ = base_8(c, seed, 0x0014_7980, 2);
                    c.g[A0][0] = OBJ as u64;
                    c.g[A1][0] = (seed % 4) as u64;
                    c.g[A2][0] = (seed % 40) as u64;
                    for i in 0..8u32 {
                        w(
                            c,
                            0x004a_6ca8 + i * 4,
                            seed.wrapping_mul(0x0101_0101).wrapping_add(i),
                        );
                    }
                },
            );
            assert!(
                r.hard.is_empty() && r.soft.is_empty(),
                "seed {seed}: {:#?} {:#?}",
                r.hard,
                r.soft
            );
        }
    }

    #[test]
    fn benewplayerint_v01_matches() {
        check(
            ee::s01::f_001459b8__generated,
            benewplayerint_v01,
            8,
            &|c, seed| {
                let _ = base_9(c, seed, 0x1459b8, 2);
                let this = DATA;
                clear_2(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                w(c, GAME_GP.wrapping_sub(0x1f30), this + 0x200);
                w(c, this + 0x208, if seed % 2 == 0 { 0 } else { 1 });
                clear_2(c, DATA + 0x535b20u32.wrapping_sub(DATA), 0x200);
            },
        );
    }

    pub(crate) const SEEDS_7: u32 = 200;

    /// Every compared state must match: hard, caller-saved (`soft`) and LO/HI.
    pub(crate) fn strict(g: GuestFn, h: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(g, h, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard={:#?} soft={:#?} lohi={:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    /// The `BELibrary` singleton (`gp - 0x1f04`) and the world (`gp - 0x848`).
    pub(crate) fn globals_5(c: &mut Cpu) {
        let be = DATA + 0x3_0000;
        let world = DATA + 0x8_0000;
        w(c, GAME_GP.wrapping_sub(0x1f04), be);
        w(c, GAME_GP.wrapping_sub(0x848), world);
        let game = world + 0x1000;
        w(c, world + 0x84, game);
        let race = world + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    /// Scrambled state, the fake call hooks, `gp` and the globals_5.
    pub(crate) fn base_10(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        globals_5(c);
        r
    }

    /// The rider character record `BENewPlayerInt_*Difficulty` reads:
    /// `BE_GetRiderCharacter(0)` resolves a byte of it through two tables.
    pub(crate) fn difficulty_fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = base_10(c, seed, salt, 0);
        let character = (seed % 4) as u32;
        w(c, 0x0053_05b0, character);
        c.bus
            .write8(0x0053_5b30 + character * 0x1c + 1, character as u8);
        let rec = 0x004a_6ca8 + character * 0xf88;
        let edge = [0u16, 1, 2, 3, 0xffff];
        let a = edge[((seed / 4) % 5) as usize];
        let b = edge[((seed / 20) % 5) as usize];
        c.bus.write16(rec + 0x280, a);
        c.bus.write16(rec + 0x284, a);
        c.bus.write16(rec + 0x282, b);
        c.bus.write16(rec + 0x286, b);
        r
    }

    #[test]
    fn benewplayerint_raise_difficulty_matches() {
        strict(
            ee::s01::f_00147d20__generated,
            benewplayerint_raise_difficulty,
            SEEDS_7,
            &|c, seed| {
                let _ = difficulty_fixture(c, seed, 0x7d20);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn benewplayerint_lower_difficulty_matches() {
        strict(
            ee::s01::f_00147e18__generated,
            benewplayerint_lower_difficulty,
            SEEDS_7,
            &|c, seed| {
                let _ = difficulty_fixture(c, seed, 0x7e18);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn be_new_player_int_get_peak_flag_b_matches() {
        same(
            ssx3_ee::s01::f_001464d0__generated,
            be_new_player_int_get_peak_flag_b,
            200,
            &|c, seed| {
                let mut r = Rng(0x1464d0 ^ seed as u64);
                generic(c, seed, 0x20);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                c.g[A3][0] = (seed % 4) as u64;
                c.g[T0][0] = ((seed / 4) % 2) as u64;
                // Profile table: index -> profile.
                for i in 0..4u32 {
                    c.bus.write32(0x0053_05b0 + 4 * i, (i % 4) as u32);
                }
                for profile in 0..4u32 {
                    for ch in 0..3u32 {
                        let p = 0x004a_6ca8 + profile * 0x9b50 + ch * 0xf88 + 0x278;
                        c.bus.write32(p, r.next_u32());
                        c.bus.write32(p + 4, r.next_u32());
                    }
                }
            },
        );
    }

    // ------------------------------------------------------------------
    // BENewPlayerInt_IsSharingCharRecord (0x1477e8).

    /// Players 0-3 (slot table 0x5305b0), random slots 0-3 and record flags.
    #[test]
    fn be_new_player_int_is_sharing_char_record_matches() {
        use crate::difftest::{scramble, Rng};
        crate::difftest::check_lohi(
            ee::s01::f_001477e8__generated,
            be_new_player_int_is_sharing_char_record,
            200,
            &|c, seed| {
                let mut r = Rng(0x1477_e8 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                let player = seed % 4;
                for p in 0..4 {
                    c.bus.write32(0x0053_05b0 + 4 * p, r.next_u32() % 4);
                }
                for slot in 0..4 {
                    c.bus.write32(
                        CHAR_RECORDS + CHAR_RECORD_SIZE * slot + CHAR_RECORD_FLAGS,
                        r.next_u32(),
                    );
                }
                c.g[A0][0] = player as u64;
                c.pc = 0x0014_77e8;
            },
        );
    }

    /// BECharInt_GetOverallRating: the real attribute getters on a rider
    /// table (0x5305b0: rider -> profile slot), profiles (0x535b20, stride
    /// 0x1c: character byte at +0x11, the -1 / odd-flag words at +0xc /
    /// +0x10) and a random signed attribute table (0x535538), so ratings
    /// come out positive, zero and negative (the minimum of 1).
    #[test]
    fn be_char_int_get_overall_rating_matches() {
        check(
            ee::s01::f_00148950__generated,
            be_char_int_get_overall_rating,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14_8950, 0);
                c.pc = 0x0014_8950;
                for i in 0..8u32 {
                    c.bus.write32(0x0053_05b0 + 4 * i, r.next_u32() % 8);
                }
                for i in 0..8u32 {
                    let p = 0x0053_5b20 + 0x1c * i;
                    fill(c, &mut r, p, 7);
                    if r.next_u32() % 3 == 0 {
                        c.bus.write32(p + 0xc, 0xffff_ffff);
                    }
                    c.bus.write8(p + 0x11, (r.next_u32() % 10) as u8);
                }
                // Mostly positive attributes; some seeds all negative.
                let neg = seed % 5 == 0;
                for i in 0..0x100u32 {
                    let v = r.next_u32() as u8;
                    let v = if neg {
                        v | 0x80
                    } else if seed % 5 < 3 {
                        v & 0x7f
                    } else {
                        v
                    };
                    c.bus.write8(0x0053_5538 + i, v);
                }
                c.g[A0][0] = (DATA + (r.next_u32() & 0xfff0)) as u64;
                c.g[A1][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    /// `BENewPlayerInt_CharProfileCall523e8`: player 0..7 whose slot (table
    /// 0x5305b0) is 0..3; the slot's rider record (0x535b20) is unused
    /// (+0xc = -1: profile 2) or used (profile = +0x10 & 1); the character
    /// byte 0..9; flag index 0..2 into the mask table at 0x45a7c0; random
    /// profile flag words. All three callees are real leaves.
    #[test]
    fn be_new_player_int_char_profile_call523e8_matches() {
        use crate::difftest::{check, scramble};
        check(
            ssx3_ee::s01::f_00145e68__generated,
            be_new_player_int_char_profile_call523e8,
            300,
            &|c, seed| {
                let mut r = Rng(0x145e_6800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0014_5e68;
                let player = r.next_u32() % 8;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = player as u64;
                c.g[A2][0] = (seed % 3) as u64;
                for i in 0..8 {
                    c.bus.write32(0x0053_05b0 + 4 * i, r.next_u32() % 4);
                }
                for slot in 0..4 {
                    let rec = 0x0053_5b20 + 0x1c * slot;
                    c.bus.write32(
                        rec + 0xc,
                        if r.next_u32() % 3 == 0 {
                            u32::MAX
                        } else {
                            r.next_u32() >> 1
                        },
                    );
                    c.bus.write32(rec + 0x10, r.next_u32());
                    c.bus
                        .write8(0x0053_4fe0 + 0x1c * slot + 0x11, (r.next_u32() % 10) as u8);
                }
                for i in 0..3 {
                    c.bus.write32(0x0045_a7c0 + 4 * i, 1 << (r.next_u32() % 32));
                }
                for profile in 0..3 {
                    for ch in 0..10 {
                        c.bus.write32(
                            0x004a_6ca8 + 0x9b50 * profile + 0xf88 * ch + 0xacc,
                            r.next_u32(),
                        );
                    }
                }
            },
        );
    }

    /// BENewPlayerInt_GetAltCharacter: slot indices 0..7 whose players use
    /// records 0..15 (alt characters of either sign), through the real
    /// BENewPlayerInt_GetSlot.
    #[test]
    fn be_new_player_int_get_alt_character_matches() {
        crate::difftest::check_lohi(
            ee::s01::f_00147410__generated,
            be_new_player_int_get_alt_character,
            200,
            &|c, seed| {
                let mut r = Rng(0x0014_7410 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0014_7410;
                for i in 0..8 {
                    c.bus.write32(BENP_SLOTS + 4 * i, r.next_u32() % 16);
                }
                fill(c, &mut r, BENP_PLAYERS, 16 * BENP_PLAYER_SIZE / 4);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (r.next_u32() % 8) as u64 | (r.next_u32() as u64) << 32;
            },
        );
    }
}
