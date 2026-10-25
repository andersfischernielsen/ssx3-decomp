//! `src/be/beintnewrace.cpp`: hand-written replacements (moved from the
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
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::bedbreward::destroy_instance;
pub(crate) use crate::game::be::bedbreward::BUILTIN_DELETE;
pub(crate) use crate::game::be::beintnetwork::NEW_RACE_GP;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::sound::soundsys::gp_4 as gp;
pub(crate) use crate::game::sound::soundsys::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_4928, be_new_race_int_commit, "BENewRaceInt_Commit"),
    (0x0014_49e8, be_new_race_int_reset, "BENewRaceInt_Reset"),
    (
        0x0014_4aa8,
        be_new_race_int_set_num_human_players,
        "BENewRaceInt_SetNumHumanPlayers",
    ),
    (
        0x0014_4b20,
        be_new_race_int_set_track_strings,
        "BENewRaceInt_SetTrackStrings",
    ),
    (
        0x0014_4be0,
        be_new_race_int_get_track,
        "BENewRaceInt_GetTrack",
    ),
    (
        0x0014_4bf0,
        be_new_race_int_get_track_name,
        "BENewRaceInt_GetTrackName",
    ),
    (
        0x0014_4c48,
        be_new_race_int_get_track_code_by_index,
        "BENewRaceInt_GetTrackCodeByIndex",
    ),
    (
        0x0014_4c60,
        be_new_race_int_get_track_name_by_index,
        "BENewRaceInt_GetTrackNameByIndex",
    ),
    (
        0x0014_4c78,
        be_new_race_int_get_track_peak_by_index,
        "BENewRaceInt_GetTrackPeakByIndex",
    ),
    (
        0x0014_4cc0,
        benewraceint_is_station_track_by_index,
        "BENewRaceInt_IsStationTrackByIndex",
    ),
    (
        0x0014_4ce0,
        be_new_race_int_find_track_by_id,
        "BENewRaceInt_FindTrackById",
    ),
    (
        0x0014_4d18,
        benewraceint_get_track_field60,
        "BENewRaceInt_GetTrackField60",
    ),
    (
        0x0014_4d38,
        benewraceint_get_section_entry,
        "BENewRaceInt_GetSectionEntry",
    ),
    (
        0x0014_4d50,
        benewraceint_get_section_field10,
        "BENewRaceInt_GetSectionField10",
    ),
    (
        0x0014_4d70,
        be_new_race_int_set_rider_slot,
        "BENewRaceInt_SetRiderSlot",
    ),
    (
        0x0014_4d98,
        be_new_race_int_set_num_ai_riders,
        "BENewRaceInt_SetNumAIRiders",
    ),
    (
        0x0014_4df0,
        be_new_race_int_set_event_type,
        "BENewRaceInt_SetEventType",
    ),
    (
        0x0014_51e8,
        be_new_race_int_set_event,
        "BENewRaceInt_SetEvent",
    ),
    (
        0x0014_5108,
        be_new_race_int_set_game_mode,
        "BENewRaceInt_SetGameMode",
    ),
    (
        0x0014_5340,
        be_new_race_int_get_event_name,
        "BENewRaceInt_GetEventName",
    ),
    (
        0x0014_5378,
        be_new_race_int_get_game_mode_name,
        "BENewRaceInt_GetGameModeName",
    ),
    (
        0x0014_5398,
        be_new_race_int_get_event_type_name,
        "BENewRaceInt_GetEventTypeName",
    ),
    (
        0x0014_53b8,
        be_new_race_int_get_event_type_name_by_index,
        "BENewRaceInt_GetEventTypeNameByIndex",
    ),
    (
        0x0014_54f8,
        benewraceint_get_event_target,
        "BENewRaceInt_GetEventTarget",
    ),
    (
        0x0014_55d0,
        benew_race_int_get_special_event_field4,
        "BENewRaceInt_GetSpecialEventField4",
    ),
    (
        0x0014_5600,
        benew_race_int_get_special_event_field6,
        "BENewRaceInt_GetSpecialEventField6",
    ),
    (
        0x0014_5630,
        be_new_race_int_get_special_event_score,
        "BENewRaceInt_GetSpecialEventScore",
    ),
    (
        0x0014_5668,
        be_new_race_int_get_special_event_value,
        "BENewRaceInt_GetSpecialEventValue",
    ),
    (
        0x0014_56a0,
        benew_race_int_get_event_record_target,
        "BENewRaceInt_GetEventRecordTarget",
    ),
    (
        0x0014_5750,
        benew_race_int_get_peak_rival_character,
        "BENewRaceInt_GetPeakRivalCharacter",
    ),
    (
        0x0015_bb20,
        be_new_race_int_destroy_instance,
        "BENewRaceInt_DestroyInstance",
    ),
];

// ---------------------------------------------------------------------------
// Callees (the names the generated bodies call them by).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::object::debouncenode::callee::EVAL_SEGMENT;
    pub(crate) use crate::game::object::debouncenode::callee::GET_CHARACTER;
    pub(crate) use crate::game::object::debouncenode::callee::GET_HUMAN_RIDER;
    pub(crate) use crate::game::object::debouncenode::callee::GET_OBJECT_SLOT;
    pub(crate) use crate::game::object::debouncenode::callee::GET_PLAYER_INSTANCE;
    pub(crate) use crate::game::object::debouncenode::callee::GET_TRACK_PEAK;
    pub(crate) use crate::game::object::debouncenode::callee::GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::object::debouncenode::callee::HAS_OBJECT_SCRIPT1;
    pub(crate) use crate::game::object::debouncenode::callee::HAS_OBJECT_SCRIPT3;
    /// `BENewRaceInt_SetNumHumanPlayers` (0x144aa8).
    pub(crate) const SET_NUM_HUMAN_PLAYERS: Callee = Callee {
        addr: 0x0014_4aa8,
        f: ee::s01::f_00144aa8,
    };
    /// `BENewRaceInt_SetNumAIRiders` (0x144d98).
    pub(crate) const SET_NUM_AI_RIDERS: Callee = Callee {
        addr: 0x0014_4d98,
        f: ee::s01::f_00144d98,
    };
    /// `BENewRaceInt_SetNumExtraRiders` (0x144af8).
    pub(crate) const SET_NUM_EXTRA_RIDERS: Callee = Callee {
        addr: 0x0014_4af8,
        f: ee::s01::f_00144af8,
    };
    /// `BENewRaceInt_SetEvent` (0x1451e8).
    pub(crate) const SET_EVENT: Callee = Callee {
        addr: 0x0014_51e8,
        f: ee::s01::f_001451e8,
    };
    /// `BENewPlayerInt_ShareSlot0CharRecord` (0x147828).
    pub(crate) const SHARE_SLOT0_CHAR_RECORD: Callee = Callee {
        addr: 0x0014_7828,
        f: ee::s01::f_00147828,
    };
    /// `BENewPlayerInt_IsSharingCharRecord` (0x1477e8).
    pub(crate) const IS_SHARING_CHAR_RECORD: Callee = Callee {
        addr: 0x0014_77e8,
        f: ee::s01::f_001477e8,
    };
    /// `BENewPlayerInt_ClearSharedCharRecord` (0x147898).
    pub(crate) const CLEAR_SHARED_CHAR_RECORD: Callee = Callee {
        addr: 0x0014_7898,
        f: ee::s01::f_00147898,
    };
}

/// `gp`-relative globals used here.
pub(crate) mod g {
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
}

// ---------------------------------------------------------------------------
// New race.

/// `int BENewRaceInt_GetTrackPeakByIndex(BENewRaceInt *self, int index)`
/// (0x144c78): `peaks[index].peak`, the records being 100 bytes each starting
/// at 0x43d950.
pub fn be_new_race_int_get_track_peak_by_index(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x64;
    let scaled = abi::mult(c, abi::arg(c, 1), 0x64);
    c.g[A1][0] = sx(scaled);
    let base = sx(0x44_0000u32.wrapping_sub(0x26b0).wrapping_add(scaled));
    c.g[V1][0] = base;
    c.g[V0][0] = Addr(base as u32).off(0x54).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// New race tables.

/// `char *BENewRaceInt_GetTrackNameByIndex(BENewRaceInt *self, int index)`
/// (0x144c60): `trackNames[index]`, records of 100 bytes starting at 0x43d954.
pub fn be_new_race_int_get_track_name_by_index(c: &mut Cpu) {
    c.cycles += 6;
    let index = abi::arg(c, 1);
    c.g[V1][0] = 0x64; // 100
    let low = abi::mult(c, index, 0x64);
    c.g[A1][0] = sx(low);
    let base = 0x44_0000u32.wrapping_sub(0x26ac); // 0x43d954
    c.g[V0][0] = sx(base.wrapping_add(low));
    abi::ret(c);
}

/// `void BENewRaceInt_SetRiderSlot(BENewRaceInt *self, int slot, int rider)`
/// (0x144d70): `riders[slot] = rider` and `riderSlots[rider] = slot`.
pub fn be_new_race_int_set_rider_slot(c: &mut Cpu) {
    c.cycles += 9;
    let slot = c.g[A1][0] as u32;
    let rider = c.g[A2][0] as u32;
    let base = 0x53_0000u32.wrapping_add(0x5bc8); // 0x535bc8
    c.g[A0][0] = sx(rider << 2);
    c.g[V0][0] = sx(base.wrapping_add(slot << 2));
    c.g[V1][0] = sx(base.wrapping_add(rider << 2));
    Addr(c.g[V0][0] as u32).sw(c, c.g[A2][0]);
    Addr(c.g[V1][0] as u32).off(0x18).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `bool BENewRaceInt_SetNumAIRiders(BENewRaceInt *self, int count)` (0x144d98):
/// bump a per-race running total by `count`, store it, and report whether the
/// selected entry is now past the limit `count` (1 when it is not).
pub fn be_new_race_int_set_num_ai_riders(c: &mut Cpu) {
    c.cycles += 21;
    let race = 0x53_0000u32.wrapping_add(0x5bc8); // 0x535bc8
    let num = c.g[A1][0];
    let entry = Addr(race).off(0x49).lb(c);
    let scaled = abi::mult(c, entry as u32, 0x1c);
    c.g[A2][0] = sx(scaled);
    let mut total = Addr(race).off(0x34).lw(c) as u32;
    let base = Addr(race).off(0x38).lw(c) as u32;
    let mut offset = (Addr(race).off(0x48).lb(c) as u32) << 2;
    total = total.wrapping_add(num as u32);
    Addr(race).off(0x3c).sw(c, num);
    total = total.wrapping_add(base);
    offset = offset.wrapping_add(scaled);
    Addr(race).off(0x30).sw(c, sx(total));
    let table = 0x44_0000u32.wrapping_sub(0x2708); // 0x43d8f8
    offset = offset.wrapping_add(table);
    c.g[V1][0] = sx(offset);
    let loaded = Addr(offset).lw(c);
    c.g[V0][0] = ((loaded < c.g[A1][0]) as u64) ^ 1;
    abi::ret(c);
}

/// `char *BENewRaceInt_GetEventName(BENewRaceInt *self)` (0x145340):
/// `eventNames[eventType]`, records of 84 bytes starting at 0x43e97c.
pub fn be_new_race_int_get_event_name(c: &mut Cpu) {
    c.cycles += 8;
    let event = Addr(0x53_0000u32.wrapping_add(0x5c12)).lb(c) as u32; // 0x535c12
    let low = abi::mult(c, event, 0x54);
    c.g[V1][0] = sx(low);
    let base = 0x44_0000u32.wrapping_sub(0x1684); // 0x43e97c
    c.g[V0][0] = sx(base.wrapping_add(low));
    abi::ret(c);
}

/// `char *BENewRaceInt_GetGameModeName(BENewRaceInt *self)` (0x145378):
/// `gameModeNames[gameMode]`, records of 67 bytes starting at 0x43e707.
pub fn be_new_race_int_get_game_mode_name(c: &mut Cpu) {
    c.cycles += 8;
    let mode = Addr(0x53_0000u32.wrapping_add(0x5c11)).lb(c) as u32; // 0x535c11
    let low = abi::mult(c, mode, 0x43);
    c.g[V1][0] = sx(low);
    let base = 0x44_0000u32.wrapping_sub(0x18f9); // 0x43e707
    c.g[V0][0] = sx(base.wrapping_add(low));
    abi::ret(c);
}

/// `char *BENewRaceInt_GetEventTypeName(BENewRaceInt *self)` (0x145398):
/// `eventTypeNames[eventType]`, records of 60 bytes starting at 0x43e7d0.
pub fn be_new_race_int_get_event_type_name(c: &mut Cpu) {
    c.cycles += 8;
    let kind = Addr(0x53_0000u32.wrapping_add(0x5c10)).lb(c) as u32; // 0x535c10
    let low = abi::mult(c, kind, 0x3c);
    c.g[V1][0] = sx(low);
    let base = 0x44_0000u32.wrapping_sub(0x1830); // 0x43e7d0
    c.g[V0][0] = sx(base.wrapping_add(low));
    abi::ret(c);
}

/// `char *BENewRaceInt_GetEventTypeNameByIndex(BENewRaceInt *self, int index)`
/// (0x1453b8): `eventTypeNames[index]`, records of 60 bytes at 0x43e7d0.
pub fn be_new_race_int_get_event_type_name_by_index(c: &mut Cpu) {
    c.cycles += 6;
    let index = abi::arg(c, 1);
    c.g[V1][0] = 0x3c; // 60
    let low = abi::mult(c, index, 0x3c);
    c.g[A1][0] = sx(low);
    let base = 0x44_0000u32.wrapping_sub(0x1830); // 0x43e7d0
    c.g[V0][0] = sx(base.wrapping_add(low));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// New-race / character / bolt / reward tables.

/// The base of the "new race" track table.
pub(crate) const NEW_RACE_TRACK: u32 = 0x0053_0000;

/// Offset of the selected-track pointer within that table.
pub(crate) const NEW_RACE_TRACK_PTR: u32 = 0x5c08;

/// The base of the per-track record table.
pub(crate) const TRACK_RECORDS: u32 = 0x0044_0000;

/// `BETrack* BENewRaceInt_GetTrack(void)` (0x144be0): the pointer at
/// `0x535c08`.
pub fn be_new_race_int_get_track(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = NEW_RACE_TRACK as u64;
    c.g[V0][0] = Addr(NEW_RACE_TRACK.wrapping_add(NEW_RACE_TRACK_PTR)).lw(c);
    abi::ret(c);
}

/// `char* BENewRaceInt_GetTrackName(int index)` (0x144bf0): the name of the
/// selected track plus `index*100` records into the track table.
pub fn be_new_race_int_get_track_name(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = NEW_RACE_TRACK as u64;
    c.g[A0][0] = 0x64;
    c.g[V1][0] = Addr(NEW_RACE_TRACK.wrapping_add(NEW_RACE_TRACK_PTR)).lw(c);
    c.g[V0][0] = TRACK_RECORDS as u64;
    c.mult(0, V1, V1, A0);
    c.g[V0][0] = sx(TRACK_RECORDS.wrapping_sub(0x26ac));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `int BENewRaceInt_GetTrackCodeByIndex(int index)` (0x144c48): the track
/// code of the `index`-th record.
pub fn be_new_race_int_get_track_code_by_index(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x64;
    c.g[V0][0] = TRACK_RECORDS as u64;
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x267c));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `BENewRaceInt_DestroyInstance()` (0x15bb20): as above for `gp - 0x1f3c`.
pub fn be_new_race_int_destroy_instance(c: &mut Cpu) {
    destroy_instance(c, NEW_RACE_GP, 0x0015_bb34);
}

// ---------------------------------------------------------------------------
// New race.

/// `BENewRaceInt_SetNumHumanPlayers(self, n)` (0x144aa8): set the human
/// count and the derived total at +0x34/+0x30, then clear six pairs of words
/// (the per-player slots at +0 and +0x18).
pub fn be_new_race_int_set_num_human_players(c: &mut Cpu) {
    const INFO: u32 = 0x0053_5bc8;
    c.cycles += 10;
    c.g[V0][0] = 0x0053_0000;
    c.g[A2][0] = 0;
    c.g[V0][0] = sx(0x0053_0000u32.wrapping_add(0x5bc8));
    c.g[V1][0] = Addr(INFO + 0x3c).lw(c);
    c.g[A3][0] = c.g[V0][0];
    c.g[A0][0] = Addr(INFO + 0x38).lw(c);
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(INFO + 0x34).sw(c, c.g[A1][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    Addr(INFO + 0x30).sw(c, c.g[V1][0]);
    loop {
        c.cycles += 7;
        Addr(c.g[A3][0] as u32).sw(c, c.g[A2][0]);
        Addr(c.g[A3][0] as u32 + 0x18).sw(c, c.g[A2][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[A2][0] as i64) < 6) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0014_4ad0);
            }
        } else {
            c.cycles += 2;
            return abi::ret(c);
        }
    }
}

/// `BENewRaceInt_SetTrackStrings(self, name)` (0x144b20): append the name to
/// four track-name buffers inside the new-race info block.
pub fn be_new_race_int_set_track_strings(c: &mut Cpu) {
    const INFO: u32 = 0x0053_5bc8;
    call::guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = INFO as u64;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = sx(INFO);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = 0x0044_0000;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0x64;
        c.g[S2][0] = c.g[A1][0];
        c.g[S0][0] = sx(0x0044_0000u32.wrapping_sub(0x267c));
        // 1: dst = *(info+0x40) * 0x64 + s0.
        c.g[A0][0] = Addr(INFO + 0x40).lw(c);
        c.mult(0, A0, A0, S3);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        call_ptr(c, 0x0041_66f4, 0x0014_4b60)?;
        // 2: dst = *(info+0x40) * 0x64 + s0 + 0x10.
        c.g[A0][0] = Addr(INFO + 0x40).lw(c);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[S2][0];
        c.mult(0, A0, A0, S3);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0041_66f4, 0x0014_4b78)?;
        // 3: dst = *(info+0x40) * 0x64 + s0 - 0x30.
        c.g[A0][0] = Addr(INFO + 0x40).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x30));
        c.g[A1][0] = c.g[S2][0];
        c.mult(0, A0, A0, S3);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        call_ptr(c, 0x0041_66f4, 0x0014_4b90)?;
        // 4: a fixed buffer.
        c.g[A0][0] = 0x0044_0000;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx(0x0044_0000u32.wrapping_sub(0x1914));
        call_ptr(c, 0x0041_66f4, 0x0014_4ba0)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BENewRaceInt_SetGameMode(self, mode)` (0x145108): record the mode byte,
/// then set the human/AI rider counts for modes 0, 1 and 2.
pub fn be_new_race_int_set_game_mode(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0053_0000;
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = sx(0x0053_0000u32.wrapping_add(0x5bc8));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = 1;
        let eq1 = c.g[A1][0] == c.g[V0][0];
        c.sb((c.g[S1][0] as u32).wrapping_add(0x49), c.g[A1][0]);
        let mode = c.g[A1][0];
        let case: u8;
        if eq1 {
            case = 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((mode as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                if mode == c.g[V0][0] {
                    case = 2;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x20);
                    c.cycles += 4;
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                if mode == 0 {
                    case = 0;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x20);
                    c.cycles += 4;
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
        match case {
            0 => {
                c.cycles += 7;
                c.g[A1][0] = 1;
                call_ptr(c, 0x0014_4aa8, 0x0014_5164)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_4d98, 0x0014_5170)?;
            }
            1 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 1;
                call_ptr(c, 0x0014_4aa8, 0x0014_5184)?;
                c.g[V0][0] = Addr(c.g[S1][0] as u32 + 0x3c).lw(c);
                let ge = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 6) as u64;
                if !ge {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_4d98, 0x0014_519c)?;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32 + 0x3c).lw(c);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 6) as u64;
                }
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 5;
                    call_ptr(c, 0x0014_4d98, 0x0014_51b8)?;
                }
            }
            _ => {
                c.cycles += 6;
                c.g[A1][0] = 2;
                call_ptr(c, 0x0014_4aa8, 0x0014_51c8)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_4d98, 0x0014_51d4)?;
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

/// The new-race settings block (`BENewRaceInt`'s state): +0x48 game mode,
/// +0x49 game type (1 or 2 players...), +0x4a event type.
const RACE_INFO: u32 = 0x0053_5bc8;
/// Per game type, the 0x43-byte row of modes allowed (`allowed[mode] == 1`).
const MODE_ROWS: u32 = 0x0043_e700;
/// Per event type, a 0x54-byte record whose first word is its game mode.
const EVENT_ROWS: u32 = 0x0043_e978;

/// gcc's inline block move of the first 0x40 bytes of a table row onto the
/// stack: two 32-byte iterations from register `src` to register `dst`
/// (`ld`/`sd` when the source is 8-aligned, else `ldl`/`ldr` and
/// `sdl`/`sdr`), the four doublewords passing through `regs`, `cycles` per
/// iteration and the event check at loop head `head` in between.
fn move_row_head(
    c: &mut Cpu,
    src: usize,
    dst: usize,
    regs: [usize; 4],
    cycles: u64,
    head: u32,
    aligned: bool,
) {
    for first in [true, false] {
        c.cycles += cycles;
        let s = c.g[src][0] as u32;
        let d = c.g[dst][0] as u32;
        let mut v = [0u64; 4];
        for (k, w) in v.iter_mut().enumerate() {
            let a = Addr(s.wrapping_add(8 * k as u32));
            *w = if aligned {
                a.ld(c)
            } else {
                a.read_u64_unaligned(c)
            };
        }
        for (k, &w) in v.iter().enumerate() {
            let a = Addr(d.wrapping_add(8 * k as u32));
            if aligned {
                a.sd(c, w)
            } else {
                a.write_u64_unaligned(c, w)
            }
            c.g[regs[k]][0] = w;
        }
        c.g[src][0] = sx(s.wrapping_add(0x20));
        c.g[dst][0] = sx(d.wrapping_add(0x20));
        if first && event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `this->vtable(+0xc)` slot `{+8 delta, +0xc fn}` of the new-player
/// interface in `s0` (the "commit" method), returning to `ret`.
fn player_commit(c: &mut Cpu, ret: u32) -> Flow {
    let player = c.g[S0][0] as u32;
    c.g[V0][0] = Addr(player.wrapping_add(0xc)).lw(c);
    let slot = VSlot::read(c, c.g[V0][0], 8);
    c.g[A0][0] = slot.delta;
    c.g[V1][0] = slot.func;
    c.g[A0][0] = sx(player.wrapping_add(slot.delta as u32));
    call_ptr(c, slot.func as u32, ret)
}

/// `bool BENewRaceInt_SetEventType(self, int mode)` (0x144df0,
/// `cBENewRaceInterface_setGameMode`): if the current game type's row
/// allows `mode`, switch to it and set the rider counts it needs (1 or 2
/// humans, AI riders, the mission slot); entering or leaving mode 4
/// (missions) shares or un-shares player 1's character record. Returns
/// whether the mode was allowed.
pub fn be_new_race_int_set_event_type(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S4, 0x70);
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S0, 0xb0);
        frame.save(c, S3, 0x80);
        frame.save(c, S5, 0x60);
        frame.save_ra(c, 0x50);
        c.g[S4][0] = 0x0053_0000;
        c.g[A3][0] = 0x43;
        c.g[V1][0] = sx(MODE_ROWS);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        let this = c.g[S1][0];
        let mode = c.g[S2][0];
        // The game type's row of allowed modes, copied to the stack.
        c.g[A2][0] = Addr(RACE_INFO + 0x49).lb(c);
        c.mult(0, A2, A2, A3);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(MODE_ROWS));
        let aligned = c.g[A2][0] & 7 == 0;
        c.g[A0][0] = c.g[SP][0];
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
        let allowed_at = sp(c).wrapping_add(mode as u32);
        c.g[A3][0] = sx(allowed_at);
        if aligned {
            move_row_head(c, A2, A0, [T2, T3, V1, A1], 11, 0x0014_4eac, true);
            c.cycles += 1;
        } else {
            move_row_head(c, A2, A0, [V1, A1, T0, T1], 19, 0x0014_4e50, false);
            c.cycles += 2;
        }
        c.g[A1][0] = sx(RACE_INFO);
        c.cycles += 11;
        c.g[S3][0] = 1;
        c.g[V1][0] = Addr(RACE_INFO + 0x49).lb(c);
        c.g[S5][0] = 5;
        let (src, dst) = (c.g[A2][0] as u32, c.g[A0][0] as u32);
        for (k, r) in [T0, T1, T2].into_iter().enumerate() {
            c.g[r][0] = Addr(src.wrapping_add(k as u32)).lb(c);
        }
        for (k, r) in [T0, T1, T2].into_iter().enumerate() {
            Addr(dst.wrapping_add(k as u32)).sb(c, c.g[r][0]);
        }
        c.g[V0][0] = 2;
        if c.g[V1][0] == 2 {
            // Two-player game type: 2 humans, 4 AI riders.
            c.cycles += 2;
            c.g[S3][0] = 2;
            c.g[S5][0] = 4;
        }
        let humans = c.g[S3][0];
        let ai = c.g[S5][0];

        c.cycles += 4;
        c.g[V1][0] = Addr(allowed_at).lb(c);
        // (v0 = 0 in the branch delay slot: the result when the mode is
        // not allowed, and what the first callees see.)
        let allowed = c.g[V1][0] == 1;
        c.g[V0][0] = 0;
        if allowed {
            c.cycles += 3;
            c.g[V1][0] = 4;
            if mode == 4 {
                // Entering missions: one human, no AI, the mission rider.
                c.cycles += 22;
                Addr(RACE_INFO + 0x48).sb(c, mode);
                c.g[A0][0] = this;
                c.g[A1][0] = 1;
                callee::SET_NUM_HUMAN_PLAYERS.call(c, 0x0014_4f3c)?;
                c.g[A0][0] = this;
                c.g[A1][0] = 0;
                callee::SET_NUM_AI_RIDERS.call(c, 0x0014_4f48)?;
                c.g[A0][0] = this;
                c.g[A1][0] = 1;
                callee::SET_NUM_EXTRA_RIDERS.call(c, 0x0014_4f54)?;
                callee::GET_PLAYER_INSTANCE.call(c, 0x0014_4f5c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 1;
                callee::SHARE_SLOT0_CHAR_RECORD.call(c, 0x0014_4f68)?;
                player_commit(c, 0x0014_4f7c)?;
            } else {
                c.g[V0][0] = Addr(RACE_INFO + 0x48).lb(c);
                c.cycles += 2;
                if c.g[V0][0] == 4 {
                    // Leaving missions.
                    c.cycles += 16;
                    c.g[A0][0] = this;
                    c.g[A1][0] = humans;
                    callee::SET_NUM_HUMAN_PLAYERS.call(c, 0x0014_4f98)?;
                    c.g[A0][0] = this;
                    c.g[A1][0] = 0;
                    callee::SET_NUM_AI_RIDERS.call(c, 0x0014_4fa4)?;
                    c.g[A0][0] = this;
                    c.g[A1][0] = 0;
                    callee::SET_NUM_EXTRA_RIDERS.call(c, 0x0014_4fb0)?;
                    callee::GET_PLAYER_INSTANCE.call(c, 0x0014_4fb8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 1;
                    callee::IS_SHARING_CHAR_RECORD.call(c, 0x0014_4fc4)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 9;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 1;
                        callee::CLEAR_SHARED_CHAR_RECORD.call(c, 0x0014_4fd8)?;
                        player_commit(c, 0x0014_4fec)?;
                    }
                }
            }
            c.g[V0][0] = sx(RACE_INFO);

            c.cycles += 3;
            c.g[V1][0] = 3;
            Addr(RACE_INFO + 0x48).sb(c, mode);
            let mut free_ride = mode == 3;
            if !free_ride {
                c.cycles += 3;
                c.g[V0][0] = 6;
                free_ride = mode == 5;
                if !free_ride {
                    c.cycles += 2;
                    free_ride = mode == 6;
                }
            }
            if free_ride {
                // Modes 3, 5, 6: the humans, one AI rider for event types
                // 4/5 outside two-player games, no extra rider.
                c.cycles += 9;
                c.g[A0][0] = this;
                c.g[A1][0] = humans;
                callee::SET_NUM_HUMAN_PLAYERS.call(c, 0x0014_501c)?;
                c.g[V1][0] = sx(RACE_INFO);
                let event = Addr(RACE_INFO + 0x4a).lbu(c);
                let event_45 = sx((event as u32).wrapping_sub(4)) < 2;
                c.g[V0][0] = 2;
                let mut one_ai = false;
                if event_45 {
                    c.cycles += 3;
                    c.g[V1][0] = Addr(RACE_INFO + 0x49).lb(c);
                    c.g[A0][0] = this;
                    one_ai = c.g[V1][0] != 2;
                }
                c.cycles += 4;
                if one_ai {
                    c.g[A1][0] = 1;
                    callee::SET_NUM_AI_RIDERS.call(c, 0x0014_5048)?;
                } else {
                    c.g[A0][0] = this;
                    c.g[A1][0] = 0;
                    callee::SET_NUM_AI_RIDERS.call(c, 0x0014_505c)?;
                }
                c.g[A0][0] = this;
                c.cycles += 2;
                c.g[A1][0] = 0;
                callee::SET_NUM_EXTRA_RIDERS.call(c, 0x0014_5068)?;
            }

            c.cycles += 2;
            c.g[V0][0] = 1;
            if mode == 0 {
                // Race: the humans and the game type's AI riders.
                c.cycles += 10;
                c.g[A0][0] = this;
                c.g[A1][0] = humans;
                callee::SET_NUM_HUMAN_PLAYERS.call(c, 0x0014_507c)?;
                c.g[A1][0] = ai;
                c.g[A0][0] = this;
                callee::SET_NUM_AI_RIDERS.call(c, 0x0014_5088)?;
                c.g[A0][0] = this;
                c.g[A1][0] = 0;
                callee::SET_NUM_EXTRA_RIDERS.call(c, 0x0014_5094)?;
                c.g[V0][0] = 1;
            }
            c.cycles += 2;
            c.g[V0][0] = 5;
            if mode == 1 {
                // The humans and one AI rider.
                c.cycles += 10;
                c.g[A1][0] = humans;
                c.g[A0][0] = this;
                callee::SET_NUM_HUMAN_PLAYERS.call(c, 0x0014_50ac)?;
                c.g[A0][0] = this;
                c.g[A1][0] = 1;
                callee::SET_NUM_AI_RIDERS.call(c, 0x0014_50b8)?;
                c.g[A0][0] = this;
                c.g[A1][0] = 0;
                callee::SET_NUM_EXTRA_RIDERS.call(c, 0x0014_50c4)?;
                c.g[V0][0] = 5;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            if mode == 5 {
                c.cycles += 4;
                c.g[A0][0] = this;
                c.g[A1][0] = 4;
                callee::SET_EVENT.call(c, 0x0014_50dc)?;
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `bool BENewRaceInt_SetEvent(self, int event)` (0x1451e8,
/// `cBENewRaceInterface_setGameEvent`): store the event type; event types 4
/// and 5 get one AI rider (none in two-player games). Returns whether the
/// event's record belongs to the current game mode.
pub fn be_new_race_int_set_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let event = c.g[A1][0];
        c.g[A3][0] = event;
        c.g[V0][0] = sx(EVENT_ROWS);
        c.g[A1][0] = 0x54;
        c.mult(0, A1, A3, A1);
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(EVENT_ROWS));
        c.g[V1][0] = c.g[A1][0] & 7;
        let aligned = c.g[V1][0] == 0;
        frame.save_ra(c, 0x60);
        // The event's record, copied to the stack.
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = 0x0053_0000;
        c.g[T0][0] = ((event as i64) < 6) as u64;
        if aligned {
            move_row_head(c, A1, A2, [V1, T1, T2, T3], 11, 0x0014_5284, true);
        } else {
            move_row_head(c, A1, A2, [V1, T1, T2, T3], 19, 0x0014_5224, false);
            c.cycles += 2;
        }
        c.cycles += 13;
        let (src, dst) = (c.g[A1][0] as u32, c.g[A2][0] as u32);
        c.g[V0][0] = Addr(src).read_u64_unaligned(c);
        c.g[T1][0] = Addr(src.wrapping_add(8)).read_u64_unaligned(c);
        c.g[T2][0] = Addr(src.wrapping_add(0x10)).lw(c);
        Addr(dst).write_u64_unaligned(c, c.g[V0][0]);
        Addr(dst.wrapping_add(8)).write_u64_unaligned(c, c.g[T1][0]);
        c.g[V1][0] = sx(RACE_INFO);
        Addr(dst.wrapping_add(0x10)).sw(c, c.g[T2][0]);
        Addr(RACE_INFO + 0x4a).sb(c, event);
        let record = sp(c);
        if c.g[T0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = Addr(record).lw(c);
        } else {
            c.cycles += 3;
            let below4 = (event as i64) < 4;
            c.g[V0][0] = Addr(record).lw(c);
            if !below4 {
                // Event types 4 and 5.
                c.cycles += 4;
                c.g[V1][0] = Addr(RACE_INFO + 0x49).lb(c);
                c.g[V0][0] = 2;
                if c.g[V1][0] == 2 {
                    c.cycles += 4;
                    c.g[A1][0] = 0;
                    callee::SET_NUM_AI_RIDERS.call(c, 0x0014_5308)?;
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = 1;
                    callee::SET_NUM_AI_RIDERS.call(c, 0x0014_5318)?;
                }
                c.g[V1][0] = sx(RACE_INFO);
                c.cycles += 1;
                c.g[V0][0] = Addr(record).lw(c);
            }
        }
        c.cycles += 7;
        c.g[A0][0] = Addr(RACE_INFO + 0x48).lb(c);
        c.g[V0][0] ^= c.g[A0][0];
        c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        frame.restore(c, S0, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Races.

/// `int BENewRaceInt_GetPeakRivalCharacter(BENewRaceInt* self)` (0x145750):
/// the rival character for the human rider on the race's current peak: the
/// player character id `c` (`BENewPlayerInt_GetCharacter`) is compared with
/// the character at the peak table entry
/// `*(0x535c08)` (`BENewRaceInt_GetTrackPeakByIndex`):
/// peak 1 -> `c == 7 ? 4 : 7`; peak 0 -> `c == 3 ? 5 : 3`;
/// peak 2 -> `c == 8 ? 6 : 8`; else 3.
pub fn benew_race_int_get_peak_rival_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = abi::arg64(c, 0);
        callee::GET_PLAYER_INSTANCE.call(c, 0x0014_5768)?;
        c.g[S0][0] = c.g[V0][0];
        callee::GET_PLAYER_INSTANCE.call(c, 0x0014_5770)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::GET_HUMAN_RIDER.call(c, 0x0014_577c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        callee::GET_CHARACTER.call(c, 0x0014_5788)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = 0x530000;
        c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x5c08).lw(c);
        callee::GET_TRACK_PEAK.call(c, 0x0014_579c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        let first_peak = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S0][0] ^ 0x7;
        if first_peak {
            c.cycles += 4;
            c.g[V1][0] = 7;
            c.g[V0][0] = 4;
            if c.g[A0][0] != 0 {
                c.g[V0][0] = c.g[V1][0];
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            let mut done = false;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
            } else {
                c.cycles += 2;
                let peak_zero = c.g[V1][0] == 0;
                c.g[A0][0] = c.g[S0][0] ^ 0x3;
                if !peak_zero {
                    c.cycles += 2;
                    c.g[V0][0] = 3;
                    done = true;
                }
            }
            if !done {
                if c.g[V1][0] == 0 {
                    // Peak 0: the third character of the group.
                    c.cycles += 4;
                    c.g[V1][0] = 3;
                    c.g[V0][0] = 5;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                } else {
                    c.cycles += 2;
                    let second_peak = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0] ^ 0x8;
                    if second_peak {
                        c.cycles += 3;
                        c.g[V1][0] = 8;
                        c.g[V0][0] = 6;
                        if c.g[A0][0] != 0 {
                            c.g[V0][0] = c.g[V1][0];
                        }
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 3;
                    }
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

// ---------------------------------------------------------------------------
// Fixed addresses (the original materialises these with `lui`/`addiu` pairs).

/// `D_0043D950`: the BE new-race station table (0x64-byte entries).
pub(crate) const SECTION_TABLE: u32 = 0x0043_d950;

/// `D_0043E250`: the BE new-race track table (0x18-byte entries; the getters
/// read entry + 0x14).
pub(crate) const TRACK_TABLE: u32 = 0x0043_e250;

/// `D_0043E254`: the BE new-race section-entry table (0x18-byte entries). The
/// original materialises it as `0x440000 - 0x1dac`.
pub(crate) const SECTION_ENTRY_TABLE: u32 = 0x0043_e254;

// ---------------------------------------------------------------------------
// BE race/player/BAGT table getters.

/// `int BENewRaceInt_IsStationTrackByIndex(int unused, int index)` (0x144cc0):
/// the station-track flag (field 0x58) of a section-table entry (0x64-byte
/// records in `D_0043D950`).
pub fn benewraceint_is_station_track_by_index(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x64;
    c.mult(0, A1, A1, V0); // a1 = index * 0x64
    c.g[V1][0] = sx(SECTION_TABLE.wrapping_add(c.g[A1][0] as u32));
    let entry = c.g[V1][0] as u32;
    c.g[V0][0] = c.lw(entry.wrapping_add(0x58));
    abi::ret(c);
}

/// `int BENewRaceInt_GetTrackField60(int unused, int index)` (0x144d18): field
/// 0x60 of a section-table entry (0x64-byte records in `D_0043D950`).
pub fn benewraceint_get_track_field60(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x64;
    c.mult(0, A1, A1, V0);
    c.g[V1][0] = sx(SECTION_TABLE.wrapping_add(c.g[A1][0] as u32));
    let entry = c.g[V1][0] as u32;
    c.g[V0][0] = c.lw(entry.wrapping_add(0x60));
    abi::ret(c);
}

/// `BENewRaceSection* BENewRaceInt_GetSectionEntry(int unused, int index)`
/// (0x144d38): the address of section-table entry `index` (0x18-byte records at
/// `D_0043E254`).
pub fn benewraceint_get_section_entry(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x18;
    c.g[V0][0] = 0x44_0000;
    c.mult(0, A1, A1, V1); // a1 = index * 0x18
    c.g[V0][0] = sx(SECTION_ENTRY_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[V0][0] = sx(base.wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}

/// `int BENewRaceInt_GetSectionField10(int unused, int index)` (0x144d50):
/// field 0x14 of a track-table entry (0x18-byte records in `D_0043E250`).
pub fn benewraceint_get_section_field10(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x18;
    c.mult(0, A1, A1, V0); // a1 = index * 0x18
    c.g[V1][0] = sx(TRACK_TABLE.wrapping_add(c.g[A1][0] as u32));
    let entry = c.g[V1][0] as u32;
    c.g[V0][0] = c.lw(entry.wrapping_add(0x14));
    abi::ret(c);
}

/// `unsigned short BENewRaceInt_GetSpecialEventField4(self, int event,
/// int index)` (0x1455d0): field 4 (u16) of `D_00440D18[event - 6][index]`
/// (a 3-wide, 20-byte-stride table).
pub fn benew_race_int_get_special_event_field4(c: &mut Cpu) {
    c.cycles += 11;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(6));
    c.g[A0][0] = 0x14;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[V1][0] = 0x0044_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = sx(0x0044_0000u32.wrapping_add(0xd18));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.mult(0, A2, A2, A0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[V1][0] as u32;
    c.g[V0][0] = Addr(at(entry, 4)).lhu(c);
    abi::ret(c);
}

/// `unsigned short BENewRaceInt_GetSpecialEventField6(self, int event,
/// int index)` (0x145600): field 6 (u16) of the same table entry.
pub fn benew_race_int_get_special_event_field6(c: &mut Cpu) {
    c.cycles += 11;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(6));
    c.g[A0][0] = 0x14;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[V1][0] = 0x0044_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = sx(0x0044_0000u32.wrapping_add(0xd18));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.mult(0, A2, A2, A0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[V1][0] as u32;
    c.g[V0][0] = Addr(at(entry, 6)).lhu(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// BENewRaceInt track lookups and BE_CopyProfile.

/// Base of the 23-entry race-entry table (`D_0043D950`).
pub(crate) const RACE_TABLE: u32 = 0x0043_d950;

/// Entry stride: `D_0043D950 + 0x8fc` is the one-past-the-end address.
pub(crate) const RACE_ENTRY_SIZE: u32 = 0x64;

/// Entries in the table.
pub(crate) const RACE_COUNT: u32 = 23;

/// `sRaceEntryFull* BENewRaceInt_FindTrackById(int id)` (0x144ce0): scan the
/// race table and return the entry whose `+0x5c` word equals `id`, else
/// entry 22.
pub fn be_new_race_int_find_track_by_id(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x0044_0000;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x26b0));
    let id = c.g[A1][0];
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(RACE_COUNT * RACE_ENTRY_SIZE));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
    loop {
        c.cycles += 2;
        if c.g[V0][0] != id {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(RACE_ENTRY_SIZE));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0014_4cf0);
                }
                continue;
            }
            c.cycles += 2;
            let ret = c.g[RA][0] as u32;
            c.g[V0][0] = 0x16;
            c.pc = ret;
            return;
        }
        c.cycles += 2;
        let ret = c.g[RA][0] as u32;
        c.g[V0][0] = Addr(r32(c, V1)).lw(c);
        c.pc = ret;
        return;
    }
}

/// `int BENewRaceInt_GetSpecialEventScore(BENewRaceInt* r, int track, int i)`
/// (0x145630): `D_00440D18[i + (track - 6) * 3].+0x12 * 100`.
pub fn be_new_race_int_get_special_event_score(c: &mut Cpu) {
    c.cycles += 13;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6));
    c.g[A3][0] = 0x14;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[V1][0] = 0x0044_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xd18));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A0][0] = 0x64;
    c.mult(0, 6, 6, 7);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x12)).lhu(c);
    let ret = c.g[RA][0] as u32;
    c.mult(0, 2, 2, 4);
    c.pc = ret;
}

/// `int BENewRaceInt_GetSpecialEventValue(BENewRaceInt* r, int track, int i,
/// int k)` (0x145668): `D_00440D18[i + (track - 6) * 3].+0x8[k]`.
pub fn be_new_race_int_get_special_event_value(c: &mut Cpu) {
    c.cycles += 13;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6));
    c.g[A0][0] = 0x14;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V1][0] = 0x0044_0000;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xd18));
    c.mult(0, 6, 6, 4);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    let ret = c.g[RA][0] as u32;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lhu(c);
    c.pc = ret;
}

/// `void BENewRaceInt_Commit(void)` (0x00144928): copy the working race
/// settings (0x535bc8) over the live copy (0x5305b0).
pub fn be_new_race_int_commit(c: &mut Cpu) {
    copy_race_settings(c, 0x0053_5bc8, 0x0053_05b0, 0x0014_4948, 0x0014_499c);
}

/// `void BENewRaceInt_Reset(void)` (0x001449e8): copy the live race
/// settings (0x5305b0) back over the working ones (0x535bc8).
pub fn be_new_race_int_reset(c: &mut Cpu) {
    copy_race_settings(c, 0x0053_05b0, 0x0053_5bc8, 0x0014_4a08, 0x0014_4a5c);
}

/// The `sRaceSettings` block copy behind both `BENewRaceInt` entry points:
/// two 0x20 halves (unaligned `ldl`/`ldr` pairs when the addresses are not
/// 8-byte aligned, plain `ld`/`sd` otherwise) and a 12-byte tail.
pub(crate) fn copy_race_settings(
    c: &mut Cpu,
    src: u32,
    dst: u32,
    head_unaligned: u32,
    head_aligned: u32,
) {
    c.cycles += 8;
    c.g[V0][0] = sx(src & 0xffff_0000);
    c.g[V1][0] = sx(src);
    c.g[A0][0] = sx(dst);
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] = c.g[V0][0] & 0x7;
    let aligned = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
    if aligned {
        loop {
            c.cycles += 11;
            c.g[A1][0] = c.ld(c.g[V1][0] as u32);
            c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
            c.sd(c.g[A0][0] as u32, c.g[A1][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head_aligned);
            }
        }
    } else {
        loop {
            c.cycles += 19;
            c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(5, c.g[V1][0] as u32);
            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
            c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
            c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
            c.sdr(5, c.g[A0][0] as u32);
            c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
            c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
            c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
            c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
            c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if !more {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head_unaligned);
            }
        }
    }
    c.cycles += 7;
    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
    c.ldr(2, c.g[V1][0] as u32);
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
    c.sdr(2, c.g[A0][0] as u32);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// `BENewRaceInt_GetEventRecordTarget` (0x1456a0): map a (row, mode) pair to a
// score/record table value; modes 6..11 are invalid (-1). Rows below 14 use
// the first table, others the second; modes 4/5 use the neighbouring entries
// and mode 4 the base value while 5 scales by 100.
pub fn benew_race_int_get_event_record_target(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x9));
    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
    let invalid = c.g[V0][0] != 0;
    c.g[V0][0] = 0x6;
    // -1 result shared by every early exit.
    let reject = |c: &mut Cpu| {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        abi::ret(c);
    };
    if invalid {
        reject(c);
        return;
    }
    c.cycles += 2;
    let is6 = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 0x7;
    if is6 {
        reject(c);
        return;
    }
    c.cycles += 2;
    let is7 = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 0x8;
    if is7 {
        reject(c);
        return;
    }
    c.cycles += 2;
    let not8 = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = ((c.g[A1][0] as i64) < 14i64) as u64;
    if !not8 {
        reject(c);
        return;
    }
    c.cycles += 2;
    let row_below_14 = c.g[V0][0] != 0;
    let mut base_path = false;
    if !row_below_14 {
        // Row >= 14: only modes 4 and 5 are valid.
        c.g[V0][0] = 0x4;
        c.cycles += 2;
        let is4 = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 16);
        if is4 {
            base_path = true;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            let not5 = c.g[A2][0] != c.g[V0][0];
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 16);
            if not5 {
                reject(c);
                return;
            }
            c.cycles += 8;
            c.g[V0][0] = 0x440000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe80));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 14) as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = 0x64;
            c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xe));
            c.mult(0, 2, 2, 4);
            abi::ret(c);
            return;
        }
    } else {
        // Row < 14: mode 0 is the base value, anything else scales by 100.
        c.cycles += 2;
        let mode_not0 = c.g[A2][0] != 0u64;
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 16);
        if !mode_not0 {
            base_path = true;
        } else {
            c.cycles += 8;
            c.g[V1][0] = 0x440000;
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 14) as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xe80));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = 0x64;
            c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2));
            c.mult(0, 2, 2, 4);
            abi::ret(c);
            return;
        }
    }
    if base_path {
        c.cycles += 6;
        c.g[V1][0] = 0x440000;
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 14) as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xe80));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2));
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// Front end: race event target.

/// `int BENewRaceInt_GetEventTarget(BENewRaceInt* self, int kind, int a2)`
/// (0x1454f8): special-field lookup by mode, else scan the 30-entry event
/// table for the current key and kind.
pub fn benewraceint_get_event_target(c: &mut Cpu) {
    let mut pc = c.pc;
    loop {
        match pc {
            0x0014_54f8 => {
                c.cycles += 10;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x10));
                c.g[V0][0] = 0x530000;
                c.sd(r32(c, SP), c.g[RA][0]);
                c.g[A3][0] = sx(r32(c, V0).wrapping_add(0x5bc8));
                c.g[A2][0] = c.g[A1][0];
                c.g[A1][0] = Addr(p(r32(c, A3), 0x4a)).lbu(c);
                c.g[V0][0] = sx(r32(c, A1).wrapping_sub(6));
                c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                let special = c.g[V0][0] != 0;
                c.g[A1][0] = sx(r32(c, A1) << 24);
                if special {
                    pc = 0x0014_5520;
                } else {
                    pc = 0x0014_5538;
                }
            }
            0x0014_5520 => {
                c.cycles += 4;
                c.g[A1][0] = sx(((r32(c, A1) as i32) >> 24) as u32);
                if call_ptr(c, 0x0014_5600, 0x0014_5528).is_err() {
                    return;
                }
                c.g[RA][0] = c.ld(r32(c, SP));
                pc = 0x0014_55c8;
            }
            0x0014_5530 => {
                c.cycles += 2;
                c.g[V0][0] = Addr(p(r32(c, V1), 0xe)).lh(c);
                pc = 0x0014_55c4;
            }
            0x0014_5538 => {
                c.cycles += 8;
                c.g[V1][0] = 0x440000;
                c.g[V0][0] = 1;
                c.g[T0][0] = sx(r32(c, V1).wrapping_add(0xb38));
                c.g[A3][0] = Addr(p(r32(c, A3), 0x40)).lw(c);
                if c.g[A2][0] == 0 {
                    c.g[A2][0] = c.g[V0][0];
                }
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[T0][0];
                pc = 0x0014_5558;
            }
            0x0014_5558 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A0)).lh(c);
                let key_ok = c.g[V0][0] == c.g[A3][0];
                c.g[V1][0] = sx(r32(c, A1) << 4);
                if key_ok {
                    pc = 0x0014_5564;
                } else {
                    pc = 0x0014_5570;
                }
            }
            0x0014_5564 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r32(c, A0), 2)).lh(c);
                if c.g[V0][0] == c.g[A2][0] {
                    c.g[V1][0] = sx(p(r32(c, T0), r32(c, V1)));
                    if back_edge_unwinds(c, 0x0014_5530) {
                        return;
                    }
                    pc = 0x0014_5530;
                } else {
                    pc = 0x0014_5570;
                }
            }
            0x0014_5570 => {
                c.cycles += 4;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V0][0] = ((c.g[A1][0] as i64) < 30i64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(p(r32(c, A0), 0x10));
                if more {
                    if back_edge_unwinds(c, 0x0014_5558) {
                        return;
                    }
                    pc = 0x0014_5558;
                } else {
                    pc = 0x0014_5580;
                }
            }
            0x0014_5580 => {
                c.cycles += 6;
                c.g[V0][0] = 0x440000;
                c.g[A3][0] = Addr(gp(c, -0x1f34)).lw(c);
                c.g[T0][0] = sx(r32(c, V0).wrapping_add(0xb38));
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[T0][0];
                pc = 0x0014_5598;
            }
            0x0014_5598 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A0)).lh(c);
                let key_ok = c.g[V0][0] == c.g[A3][0];
                c.g[V1][0] = sx(r32(c, A1) << 4);
                if key_ok {
                    pc = 0x0014_55a4;
                } else {
                    pc = 0x0014_55b0;
                }
            }
            0x0014_55a4 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r32(c, A0), 2)).lh(c);
                if c.g[V0][0] == c.g[A2][0] {
                    c.g[V1][0] = sx(p(r32(c, T0), r32(c, V1)));
                    if back_edge_unwinds(c, 0x0014_5530) {
                        return;
                    }
                    pc = 0x0014_5530;
                } else {
                    pc = 0x0014_55b0;
                }
            }
            0x0014_55b0 => {
                c.cycles += 4;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V0][0] = ((c.g[A1][0] as i64) < 30i64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(p(r32(c, A0), 0x10));
                if more {
                    if back_edge_unwinds(c, 0x0014_5598) {
                        return;
                    }
                    pc = 0x0014_5598;
                } else {
                    pc = 0x0014_55c0;
                }
            }
            0x0014_55c0 => {
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                pc = 0x0014_55c4;
            }
            0x0014_55c4 => {
                c.cycles += 1;
                c.g[RA][0] = c.ld(r32(c, SP));
                pc = 0x0014_55c8;
            }
            0x0014_55c8 => {
                c.cycles += 2;
                let ra = r32(c, RA);
                c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x0014_54f8, 0x0014_55d0);
                return;
            }
        }
    }
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
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_2;
    pub(crate) use crate::game::bx::bxstring2::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W;
    pub(crate) use crate::game::object::debouncenode::tests::base;
    pub(crate) use crate::game::object::debouncenode::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_3;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool;
    pub(crate) use crate::game::sound::soundsys::tests::w;
    pub(crate) use crate::game::sound::soundsys::tests::BE as BE_2;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD;
    pub(crate) use crate::game::world::worldview::tests::word;
    use ssx3_ee as ee;
    use ssx3_ee::s00;
    use ssx3_ee::s01;
    use ssx3_ee::s02;
    use ssx3_ee::s03;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;

    #[test]
    fn be_new_race_int_get_track_peak_by_index_matches() {
        check(
            ssx3_ee::s01::f_00144c78__generated,
            be_new_race_int_get_track_peak_by_index,
            100,
            &|c, seed| {
                let mut r = Rng(0x9191_9191 ^ seed as u64);
                scramble(c, &mut r);
                let index = seed % 8;
                c.g[A1][0] = index as u64;
                let base = 0x43d950u32.wrapping_add(index * 0x64);
                c.bus.write32(base + 0x54, r.next_u32());
            },
        );
    }

    /// The `BENewRaceInt` singleton and the player-record base the generated
    /// `GetSlot` computes (`0x534fe0`).
    pub(crate) const RACE: u32 = 0x53_5bc8;

    #[test]
    fn be_new_race_int_get_track_name_by_index_matches() {
        check_lohi(
            ssx3_ee::s01::f_00144c60,
            be_new_race_int_get_track_name_by_index,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0001 ^ seed as u64);
                scramble(c, &mut r);
                let index = match seed % 3 {
                    0 => seed % 8,
                    1 => 0xffff_ffff, // -1
                    _ => r.next_u32(),
                };
                c.g[A1][0] = index as u64;
            },
        );
    }

    #[test]
    fn be_new_race_int_set_rider_slot_matches() {
        check_lohi(
            ssx3_ee::s01::f_00144d70,
            be_new_race_int_set_rider_slot,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    #[test]
    fn be_new_race_int_set_num_ai_riders_matches() {
        check_lohi(
            ssx3_ee::s01::f_00144d98,
            be_new_race_int_set_num_ai_riders,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0003 ^ seed as u64);
                scramble(c, &mut r);
                let entry = (seed % 8) as u8;
                let count = (seed % 4) as u8;
                c.bus.write8(RACE + 0x48, entry);
                c.bus.write8(RACE + 0x49, count);
                c.bus.write32(RACE + 0x34, r.next_u32());
                c.bus.write32(RACE + 0x38, r.next_u32());
                c.g[A1][0] = match seed % 4 {
                    0 => r.next_u32() as u64,
                    1 => 0xffff_ffff_8000_0000,
                    2 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    _ => 0x1_0000_0000u64 | r.next_u32() as u64,
                };
                let at = 0x43_d8f8u32
                    .wrapping_add((entry as u32) << 2)
                    .wrapping_add((count as u32) * 0x1c);
                c.bus.write32(at, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_race_int_get_event_name_matches() {
        check_lohi(
            ssx3_ee::s01::f_00145340,
            be_new_race_int_get_event_name,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0005 ^ seed as u64);
                scramble(c, &mut r);
                let b = match seed % 4 {
                    0 => 0xff,
                    1 => 0x7f,
                    _ => (r.next_u32() % 8) as u8,
                };
                c.bus.write8(0x53_5c12, b);
            },
        );
    }

    #[test]
    fn be_new_race_int_get_game_mode_name_matches() {
        check_lohi(
            ssx3_ee::s01::f_00145378,
            be_new_race_int_get_game_mode_name,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0006 ^ seed as u64);
                scramble(c, &mut r);
                let b = match seed % 4 {
                    0 => 0xff,
                    1 => 0x7f,
                    _ => (r.next_u32() % 8) as u8,
                };
                c.bus.write8(0x53_5c11, b);
            },
        );
    }

    #[test]
    fn be_new_race_int_get_event_type_name_matches() {
        check_lohi(
            ssx3_ee::s01::f_00145398,
            be_new_race_int_get_event_type_name,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0007 ^ seed as u64);
                scramble(c, &mut r);
                let b = match seed % 4 {
                    0 => 0xff,
                    1 => 0x7f,
                    _ => (r.next_u32() % 8) as u8,
                };
                c.bus.write8(0x53_5c10, b);
            },
        );
    }

    #[test]
    fn be_new_race_int_get_event_type_name_by_index_matches() {
        check_lohi(
            ssx3_ee::s01::f_001453b8,
            be_new_race_int_get_event_type_name_by_index,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0008 ^ seed as u64);
                scramble(c, &mut r);
                let index = match seed % 3 {
                    0 => seed % 8,
                    1 => 0xffff_ffff, // -1
                    _ => r.next_u32(),
                };
                c.g[A1][0] = index as u64;
            },
        );
    }

    #[test]
    fn be_new_race_int_get_track_matches() {
        check(
            ssx3_ee::s01::f_00144be0,
            be_new_race_int_get_track,
            100,
            &|c, seed| {
                let mut r = Rng(0x1005 ^ seed as u64);
                scramble(c, &mut r);
                c.bus
                    .write32(NEW_RACE_TRACK + NEW_RACE_TRACK_PTR, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_race_int_get_track_name_matches() {
        check_lohi(
            ssx3_ee::s01::f_00144bf0,
            be_new_race_int_get_track_name,
            100,
            &|c, seed| {
                let mut r = Rng(0x1006 ^ seed as u64);
                scramble(c, &mut r);
                c.bus
                    .write32(NEW_RACE_TRACK + NEW_RACE_TRACK_PTR, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_race_int_get_track_code_by_index_matches() {
        check_lohi(
            ssx3_ee::s01::f_00144c48,
            be_new_race_int_get_track_code_by_index,
            200,
            &|c, seed| {
                let mut r = Rng(0x1007 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = (r.next_u32() % 16) as u64;
            },
        );
    }

    #[test]
    fn be_new_race_match() {
        check(
            ssx3_ee::s01::f_00144aa8,
            be_new_race_int_set_num_human_players,
            120,
            &|c, seed| {
                let mut r = Rng(0x44aa_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | (seed % 8) as u64;
                for i in 0..0x80 {
                    c.bus.write32(0x0053_5bc8 + i * 4, r.next_u32());
                }
            },
        );
        check(
            ssx3_ee::s01::f_00144b20,
            be_new_race_int_set_track_strings,
            60,
            &|c, seed| {
                let mut r = Rng(0x44b2_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = (DATA + 0x200) as u64;
                for i in 0..8 {
                    c.bus.write8(DATA + 0x200 + i, (r.next_u32() | 1) as u8);
                }
                c.bus.write8(DATA + 0x208, 0);
                c.bus.write32(0x0053_5bc8 + 0x40, 0);
                for i in 0..0x60 {
                    c.bus.write32(0x0043_d900 + i * 4, 0);
                }
            },
        );
    }

    #[test]
    fn be_new_race_game_mode_match() {
        check(
            ssx3_ee::s01::f_00145108,
            be_new_race_int_set_game_mode,
            100,
            &|c, seed| {
                let mut r = Rng(0x5108_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x800) as u64;
                c.g[A1][0] = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 0xffff_ffff_ffff_ffff,
                    _ => 0x10,
                };
                for i in 0..0x30 {
                    c.bus.write32(0x0053_5bc8 + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn benew_race_int_get_peak_rival_character_matches() {
        check(
            ee::s01::f_00145750__generated,
            benew_race_int_get_peak_rival_character,
            SEEDS,
            &|c, seed| {
                let _r = base(c, seed, 0x1457);
                // `BENewPlayerInt_GetInstance` singleton (no allocation).
                c.bus
                    .write32((GAME_GP as u32).wrapping_sub(0x1f30), DATA + 0x800);
                // `GetSlot(0)` is the slot number the character table is indexed
                // by; 0 -> the byte at 0x534ff1.
                c.bus.write32(0x5305b0, 0);
                c.bus.write8(0x534ff1, (seed % 10) as u8);
                // The peak the race asks for, and the peak record it comes from.
                c.bus.write32(0x535c08, 0);
                c.bus.write32(0x43d950 + 0x54, (seed % 5) as u32);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn benewraceint_get_section_entry_matches() {
        check_lohi(
            ee::s01::f_00144d38__generated,
            benewraceint_get_section_entry,
            SEEDS_2,
            &|c, seed| {
                let mut r = rng(0x0014_4d38, seed);
                scramble(c, &mut r);
                c.g[A1][0] = mixed(&mut r, seed, (seed % 8) as u32);
            },
        );
    }

    #[test]
    fn benewraceint_is_station_track_by_index_matches() {
        check_lohi(
            ee::s01::f_00144cc0__generated,
            benewraceint_is_station_track_by_index,
            SEEDS_2,
            &|c, seed| {
                let mut r = rng(0x0014_4cc0, seed);
                scramble(c, &mut r);
                let index = (seed % 8) as u32;
                c.g[A1][0] = mixed(&mut r, seed, index);
                c.bus
                    .write32(SECTION_TABLE + index * 0x64 + 0x58, r.next_u32());
            },
        );
    }

    #[test]
    fn benewraceint_get_track_field60_matches() {
        check_lohi(
            ee::s01::f_00144d18__generated,
            benewraceint_get_track_field60,
            SEEDS_2,
            &|c, seed| {
                let mut r = rng(0x0014_4d18, seed);
                scramble(c, &mut r);
                let index = (seed % 8) as u32;
                c.g[A1][0] = mixed(&mut r, seed, index);
                c.bus
                    .write32(SECTION_TABLE + index * 0x64 + 0x60, r.next_u32());
            },
        );
    }

    #[test]
    fn benewraceint_get_section_field10_matches() {
        check_lohi(
            ee::s01::f_00144d50__generated,
            benewraceint_get_section_field10,
            SEEDS_2,
            &|c, seed| {
                let mut r = rng(0x0014_4d50, seed);
                scramble(c, &mut r);
                let index = (seed % 8) as u32;
                c.g[A1][0] = mixed(&mut r, seed, index);
                c.bus
                    .write32(TRACK_TABLE + index * 0x18 + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn benew_race_int_get_special_event_field4_matches() {
        check_lohi(
            s01::f_001455d0__generated,
            benew_race_int_get_special_event_field4,
            200,
            &|c, seed| {
                let mut r = Rng(0x1455_d0aa ^ seed as u64);
                scramble(c, &mut r);
                let event = 6 + (seed % 4) as i32;
                let index = (seed % 3) as i32;
                c.g[A1][0] = sx(event as u32);
                c.g[A2][0] = sx(index as u32);
                // 12 table entries (4 events x 3 indices) of 20 bytes at 0x440d18.
                let base = 0x0044_0d18u32;
                for i in 0..36u32 {
                    c.bus.write32(base + 0x14 * i, r.next_u32());
                    c.bus.write16(base + 0x14 * i + 4, r.next_u32() as u16);
                    c.bus.write16(base + 0x14 * i + 6, r.next_u32() as u16);
                }
            },
        );
    }

    #[test]
    fn benew_race_int_get_special_event_field6_matches() {
        check_lohi(
            s01::f_00145600__generated,
            benew_race_int_get_special_event_field6,
            200,
            &|c, seed| {
                let mut r = Rng(0x1456_00aa ^ seed as u64);
                scramble(c, &mut r);
                let event = 6 + (seed % 4) as i32;
                let index = (seed % 3) as i32;
                c.g[A1][0] = sx(event as u32);
                c.g[A2][0] = sx(index as u32);
                let base = 0x0044_0d18u32;
                for i in 0..36u32 {
                    c.bus.write32(base + 0x14 * i, r.next_u32());
                    c.bus.write16(base + 0x14 * i + 4, r.next_u32() as u16);
                    c.bus.write16(base + 0x14 * i + 6, r.next_u32() as u16);
                }
            },
        );
    }

    #[test]
    fn be_new_race_int_find_track_by_id_matches() {
        check(
            ee::s01::f_00144ce0__generated,
            be_new_race_int_find_track_by_id,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x144c_e0, 1);
                be_library(c);
                const WANTED: u32 = 0x555;
                for i in 0..RACE_COUNT {
                    let e = RACE_TABLE + RACE_ENTRY_SIZE * i;
                    c.bus.write32(e, r.next_u32());
                    // Ids in 0x1000..0x1fff can never collide with the probe.
                    let id = if i == 7 {
                        WANTED
                    } else {
                        0x1000 + r.next_u32() % 0x1000
                    };
                    c.bus.write32(e + 0x5c, id);
                }
                c.g[A1][0] = (if seed % 2 == 0 { WANTED } else { 0x777 }) as u64;
            },
        );
    }

    #[test]
    fn be_new_race_int_get_special_event_score_matches() {
        check(
            ee::s01::f_00145630__generated,
            be_new_race_int_get_special_event_score,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1456_30, 1);
                be_library(c);
                // The `D_00440D18` records: stride 0x14, the score at +0x12.
                for i in 0..0x40u32 {
                    c.bus.write32(0x0044_0d18 + 4 * i, r.next_u32());
                }
                c.g[A1][0] = if seed % 4 == 0 {
                    6 + (r.next_u32() % 4) as u64
                } else {
                    (r.next_u32() % 9) as u64
                };
                c.g[A2][0] = (r.next_u32() % 4) as u64;
            },
        );
    }

    #[test]
    fn be_new_race_int_get_special_event_value_matches() {
        check(
            ee::s01::f_00145668__generated,
            be_new_race_int_get_special_event_value,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1456_68, 1);
                be_library(c);
                for i in 0..0x40u32 {
                    c.bus.write32(0x0044_0d18 + 4 * i, r.next_u32());
                }
                c.g[A1][0] = if seed % 4 == 0 {
                    6 + (r.next_u32() % 4) as u64
                } else {
                    (r.next_u32() % 9) as u64
                };
                c.g[A2][0] = (r.next_u32() % 4) as u64;
                c.g[A3][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    // ------------------------------------------------------------- race settings

    #[test]
    fn be_new_race_int_copies_match() {
        check(
            ssx3_ee::s01::f_00144928__generated,
            be_new_race_int_commit,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x144928, 0);
                for k in 0..0x14 {
                    word(c, 0x0053_05b0 + 4 * k, r.next_u32());
                    word(c, 0x0053_5bc8 + 4 * k, r.next_u32());
                }
            },
        );
        check(
            ssx3_ee::s01::f_001449e8__generated,
            be_new_race_int_reset,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1449e8, 0);
                for k in 0..0x14 {
                    word(c, 0x0053_05b0 + 4 * k, r.next_u32());
                    word(c, 0x0053_5bc8 + 4 * k, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn be_new_race_int_copies_deliver_events() {
        use crate::difftest::events::schedule;
        check(
            ssx3_ee::s01::f_00144928__generated,
            be_new_race_int_commit,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x244928, 0);
                for k in 0..0x14 {
                    word(c, 0x0053_05b0 + 4 * k, r.next_u32());
                    word(c, 0x0053_5bc8 + 4 * k, r.next_u32());
                }
                schedule(c, &mut r, seed, 0x0014_4928);
            },
        );
        check(
            ssx3_ee::s01::f_001449e8__generated,
            be_new_race_int_reset,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2449e8, 0);
                for k in 0..0x14 {
                    word(c, 0x0053_05b0 + 4 * k, r.next_u32());
                    word(c, 0x0053_5bc8 + 4 * k, r.next_u32());
                }
                schedule(c, &mut r, seed, 0x0014_49e8);
            },
        );
    }

    #[test]
    fn benew_race_int_get_event_record_target_matches() {
        check(
            ee::s01::f_001456a0__generated,
            benew_race_int_get_event_record_target,
            150,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1456);
                c.g[A1][0] = (seed % 20) as u64;
                c.g[A2][0] = (seed / 20 % 13) as u64;
                for i in 0..24 {
                    c.bus
                        .write16(0x0044_0e80 + 2 * i, (r.next_u32() & 0x7fff) as u16);
                }
            },
        );
    }

    #[test]
    fn benewraceint_get_event_target_matches() {
        check(
            ee::s01::f_001454f8__generated,
            benewraceint_get_event_target,
            16,
            &|c, seed| {
                let _ = base_3(c, seed, 0x1454f8, 2);
                clear(c, DATA, 0x400);
                c.pc = 0x0014_54f8;
                c.g[A0][0] = DATA as u64;
                let mut kind = (seed % 3) as u64;
                let mode = if seed % 2 == 0 {
                    0
                } else {
                    6 + (seed % 6) as u32
                };
                c.g[A1][0] = kind;
                c.g[A2][0] = (seed % 2) as u64;
                w(c, 0x0053_5bc8 + 0x40, 0);
                w(c, 0x0053_5bc8 + 0x4a, mode);
                w(c, GAME_GP.wrapping_sub(0x1f34), 0);
                if kind == 0 {
                    kind = 1;
                }
                w(c, 0x0044_0b38, 0);
                c.bus.write16(0x0044_0b38 + 2, kind as u16);
                c.bus.write16(0x0044_0b38 + 0xe, 0x1234);
                for i in 1..30u32 {
                    w(c, 0x0044_0b38 + 0x10 * i, 0xffff_ffff);
                }
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x144df0 BENewRaceInt_SetEventType, 0x1451e8 BENewRaceInt_SetEvent.

    /// Random new-race state: game type `gt`, current mode, event type; the
    /// player-slot table small (`BENewPlayerInt_GetSlot`).
    fn new_race_state(c: &mut Cpu, r: &mut Rng, gt: u8, mode: u8, event: u8) {
        for i in 0..0x14 {
            c.bus.write32(RACE_INFO + i * 4, r.next_u32());
        }
        c.bus.write8(RACE_INFO + 0x48, mode);
        c.bus.write8(RACE_INFO + 0x49, gt);
        c.bus.write8(RACE_INFO + 0x4a, event);
        for i in 0..8 {
            c.bus.write32(0x0053_05b0 + i * 4, r.next_u32() % 6);
        }
        for i in 0..8 {
            c.bus.write32(0x0053_4fe0 + i * 0x1c + 0x10, r.next_u32());
        }
    }

    #[test]
    fn be_new_race_int_set_event_type_matches() {
        use crate::difftest::events::schedule;
        check(
            ssx3_ee::s01::f_00144df0__generated,
            be_new_race_int_set_event_type,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_4df0, 1);
                let this = DATA + 0x800;
                let mode = seed % 8;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = mode as u64;
                // Game types: aligned rows (0, 8), unaligned (1, 2), random.
                let gt = match (seed / 8) % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 8,
                    _ => r.next_u32() as u8,
                };
                let current = if (seed / 40) % 2 == 0 {
                    4
                } else {
                    (r.next_u32() % 8) as u8
                };
                let event = match (seed / 80) % 3 {
                    0 => 4,
                    1 => 5,
                    _ => r.next_u32() as u8,
                };
                new_race_state(c, &mut r, gt, current, event);
                // The game type's row: mostly "allowed".
                let row = MODE_ROWS.wrapping_add((gt as i8 as i32 * 0x43) as u32);
                for i in 0..0x43 {
                    let b = if r.next_u32() % 6 == 0 {
                        r.next_u32() as u8
                    } else {
                        1
                    };
                    c.bus.write8(row + i, b);
                }
                // `BENewPlayerInt_GetInstance` singleton with its commit slot.
                let player = DATA + 0x1000;
                let vt = DATA + 0x1100;
                c.bus.write32(GAME_GP.wrapping_sub(0x1f30), player);
                c.bus.write32(player + 0xc, vt);
                c.bus.write16(vt + 8, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + 0xc, FAKE);
                // Event records for `SetEvent(4)` (mode 5).
                for i in 0..0x15 {
                    c.bus.write32(EVENT_ROWS + 4 * 0x54 + i * 4, r.next_u32());
                }
                if seed % 7 == 3 {
                    c.g[RA][0] = SENTINEL as u64;
                    schedule(c, &mut r, seed / 7, 0x0014_4df0);
                }
            },
        );
    }

    #[test]
    fn be_new_race_int_set_event_matches() {
        use crate::difftest::events::schedule;
        check(
            ssx3_ee::s01::f_001451e8__generated,
            be_new_race_int_set_event,
            320,
            &|c, seed| {
                let mut r = Rng(0x0014_51e8 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                // Events 0..7: below 4, 4/5 (AI riders), 6+; odd ones are
                // unaligned rows.
                let event = seed % 8;
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = event as u64;
                let mode = r.next_u32() as u8;
                let gt = if (seed / 8) % 2 == 0 {
                    2
                } else {
                    r.next_u32() as u8
                };
                let event_type = r.next_u32() as u8;
                new_race_state(c, &mut r, gt, mode, event_type);
                let row = EVENT_ROWS + event * 0x54;
                for i in 0..0x15 {
                    c.bus.write32(row + i * 4, r.next_u32());
                }
                if (seed / 16) % 2 == 0 {
                    // The record's mode is the current one.
                    c.bus.write32(row, mode as i8 as i32 as u32);
                }
                if seed % 5 == 4 {
                    c.g[RA][0] = SENTINEL as u64;
                    schedule(c, &mut r, seed / 5, 0x0014_51e8);
                }
            },
        );
    }
}
