//! `src/be/belibrary.cpp`: hand-written replacements (moved from the
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
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::animation::rideranimbase::sw;
pub(crate) use crate::game::be::beintmission::event_check;
pub(crate) use crate::game::be::beintmission::lw;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_CHARACTER;
pub(crate) use crate::game::be::beintmission::BE_GET_RIDER_PROFILE;
pub(crate) use crate::game::be::beintmission::EA_MEM_ALLOC_NAMED;
pub(crate) use crate::game::be::beintplayer::mul32;
pub(crate) use crate::game::fe::feasyncfile::addu;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_4;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::sub;
pub(crate) use crate::game::fe::fepopup::at as at_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_5;
pub(crate) use crate::game::fe::fepopup::sq;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_2;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_dc80, be_library_get_instance, "cBE_getBE"),
    (
        0x0014_a130,
        be_find_rider_by_profile,
        "BE_FindRiderByProfile",
    ),
    (
        0x0014_ab20,
        be_score_int_get_high_score_table,
        "BEScoreInt_GetHighScoreTable",
    ),
    (
        0x0014_ab68,
        be_get_event_record_index,
        "BE_GetEventRecordIndex",
    ),
    (
        0x0014_abe0,
        be_bolt_int_get_instance,
        "BEBoltInt_GetInstance",
    ),
    (
        0x0014_ac30,
        beb_bolt_int_get_rider_character,
        "BEBoltInt_GetRiderCharacter",
    ),
    (
        0x0014_ac50,
        beb_bolt_int_get_rider_profile,
        "BEBoltInt_GetRiderProfile",
    ),
    (
        0x0014_ac70,
        beb_bolt_int_get_rider_profile_and_char,
        "BEBoltInt_GetRiderProfileAndChar",
    ),
    (
        0x0014_acb0,
        beb_bolt_int_set_bolt_states,
        "BEBoltInt_SetBoltStates",
    ),
    (
        0x0014_ad50,
        beb_bolt_int_get_rider_profile_data,
        "BEBoltInt_GetRiderProfileData",
    ),
    (
        0x0014_ae28,
        be_bolt_int_get_bolt_states,
        "BEBoltInt_GetBoltStates",
    ),
    (
        0x0014_ae60,
        beb_bolt_int_get_rider_bolt_states,
        "BEBoltInt_GetRiderBoltStates",
    ),
    (
        0x0014_aea8,
        be_bolt_int_save_equip_flags,
        "BEBoltInt_SaveEquipFlags",
    ),
    (
        0x0014_b478,
        be_bolt_int_sum_equipped_bolt_value,
        "BEBoltInt_SumEquippedBoltValue",
    ),
    (0x0014_b560, be_bolt_int_unlock_bolt, "BEBoltInt_UnlockBolt"),
    (
        0x0014_c2b0,
        bolts_index_bolts_by_char,
        "Bolts_IndexBoltsByChar",
    ),
    (
        0x0014_c320,
        bolts_index_table2_by_char,
        "Bolts_IndexTable2ByChar",
    ),
    (
        0x0014_c3c8,
        bolts_index_table3_by_char,
        "Bolts_IndexTable3ByChar",
    ),
    (
        0x0014_c488,
        bolts_index_table4_by_char,
        "Bolts_IndexTable4ByChar",
    ),
    (0x0014_c620, bolts_init_db, "Bolts_InitDB"),
    (0x0014_c658, bolts_destroy_db, "Bolts_DestroyDB"),
    (0x0014_d240, bolts_free_db, "Bolts_FreeDB"),
    (0x0014_d448, bolts_find_bolt_index, "Bolts_FindBoltIndex"),
    (0x0014_d558, bolts_find_first_child, "Bolts_FindFirstChild"),
    (0x0014_d908, bolts_is_descendant_of, "Bolts_IsDescendantOf"),
    (
        0x0014_d988,
        bolts_get_char_bolt_count,
        "Bolts_GetCharBoltCount",
    ),
    (0x0014_d9d0, bolts_get_char_bolts_2, "Bolts_GetCharBolts_2"),
    (
        0x0014_a0b0,
        be_get_rider_alt_character,
        "BE_GetRiderAltCharacter",
    ),
    (
        0x0014_b988,
        beb_bolt_int_get_bolt_texture_name,
        "BEBoltInt_GetBoltTextureName",
    ),
    (0x0014_c7a0, bolts_build_game_bolts, "Bolts_BuildGameBolts"),
    (0x0014_dc10, bolts_get_char_table3, "Bolts_GetCharTable3"),
    (0x0014_dc50, bolts_get_char_table4, "Bolts_GetCharTable4"),
    (0x0014_be70, bolts_read_file, "Bolts_ReadFile"),
    (0x0014_d068, bolts_prepare_for_game, "Bolts_PrepareForGame"),
    (
        0x0014_dc00,
        bolts_get_char_table3_count,
        "Bolts_GetCharTable3Count",
    ),
    (
        0x0014_dc40,
        bolts_get_char_table4_count,
        "Bolts_GetCharTable4Count",
    ),
    (
        0x0014_af10,
        be_bolt_int_restore_all_equip_flags_2,
        "BEBoltInt_RestoreAllEquipFlags",
    ),
    (0x0014_bd78, bolts_load_global_db, "Bolts_LoadGlobalDB"),
    (0x0014_bd98, bolts_prepare_game_db, "Bolts_PrepareGameDB"),
    (
        0x0014_a5e0,
        be_library_deserialize_save,
        "BELibrary_DeserializeSave",
    ),
    (
        0x0014_acf8,
        be_init_rider_char_profile_bolt_map,
        "BE_InitRiderCharProfileBoltMap",
    ),
    (
        0x0014_ada8,
        bolt_int_get_bolt_state,
        "BEBoltInt_GetBoltState",
    ),
    (
        0x0014_add8,
        bolt_int_get_rider_bolt_state,
        "BEBoltInt_GetRiderBoltState",
    ),
    (
        0x0014_afb0,
        bolt_int_set_bolt_equipped,
        "BEBoltInt_SetBoltEquipped",
    ),
    (
        0x0014_b1b0,
        bolt_int_char_record_call_52368,
        "BEBoltInt_CharRecordCall52368",
    ),
    (
        0x0014_b2f8,
        bolt_int_count_available_bolts,
        "BEBoltInt_CountAvailableBolts",
    ),
    (
        0x0014_b378,
        bolt_int_get_available_bolts,
        "BEBoltInt_GetAvailableBolts",
    ),
    (
        0x0014_b700,
        bolt_int_dispatch_category,
        "BEBoltInt_DispatchCategory",
    ),
    (
        0x0014_b920,
        bolt_int_collect_rider_equipped_texture_names,
        "BEBoltInt_CollectRiderEquippedTextureNames",
    ),
    (
        0x0014_bba0,
        bolt_int_get_rider_bolt_texture_name,
        "BEBoltInt_GetRiderBoltTextureName",
    ),
    (
        0x0014_bc18,
        bolt_int_collect_equipped_file_names,
        "BEBoltInt_CollectEquippedFileNames",
    ),
    (0x0014_d608, bolts_collect_children, "Bolts_CollectChildren"),
    (
        0x0014_a188,
        be_library_serialize_save,
        "BELibrary_SerializeSave",
    ),
    (
        0x0014_b7a8,
        bolt_int_collect_equipped_texture_names_2,
        "BEBoltInt_CollectEquippedTextureNames",
    ),
    (0x0014_d4c8, bolts_count_children_2, "Bolts_CountChildren"),
    (
        0x0014_d7e8,
        bolts_collect_by_group_2,
        "Bolts_CollectByGroup",
    ),
    (
        0x0014_db40,
        bolts_find_table2_entries_by_flag_2,
        "Bolts_FindTable2EntriesByFlag",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {

    pub(crate) use crate::game::fe::fepopup::callee::C0014BDB8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014D448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014D988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014D998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014DC80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014DD58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150B48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151C48;

    pub(crate) use crate::game::fe::fepopup::callee::C00150928;

    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;

    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;

    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;

    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
}

// ---------------------------------------------------------------------------
// BEBoltInt_SumEquippedBoltValue (0x0014b478).

/// The character bolt database and the equipped-bolt flags table.
pub(crate) mod bolts {

    /// `BoltsBolt` stride and the equipped flag word offset.
    pub const STRIDE: u32 = 0x38;

    pub const FLAGS: u32 = 4;

    pub const VALUE: u32 = 0xc;
}

// ---------------------------------------------------------------------------
// Bolt database.

/// `void Bolts_InitDB(BoltsDB *db)` (0x14c620): zero the eleven words of the
/// DB header (0x00..0x28); leaves `db` in v0, as the constructor stub does.
pub fn bolts_init_db(c: &mut Cpu) {
    c.cycles += 13;
    let db = Addr(c.g[A0][0] as u32);
    c.g[V0][0] = c.g[A0][0];
    for off in (0..0x2c).step_by(4) {
        db.off(off).sw(c, 0);
    }
    abi::ret(c);
}

/// `int Bolts_GetCharBoltCount(BoltsDB *db, int charIndex)` (0x14d988): read the
/// count at `db + charIndex*4 + 0x2c`.
pub fn bolts_get_char_bolt_count(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    let record = abi::arg(c, 0).wrapping_add(c.g[A1][0] as u32);
    c.g[A0][0] = sx(record);
    c.g[V0][0] = Addr(record).off(0x2c).lw(c);
    abi::ret(c);
}

/// The shared body of `Bolts_GetCharBolts`/`_2`: if the character's bolt count
/// is positive, resolve the bolt array at `record + 0xa4` through its stride
/// (0x38) and add the base read at `db + 4`; otherwise return 0.
pub(crate) fn char_bolts_pointer(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    let record = abi::arg(c, 0).wrapping_add(c.g[A1][0] as u32);
    c.g[V0][0] = sx(record);
    let count = Addr(record).off(0x2c).lw(c);
    c.g[V1][0] = c.g[V0][0];
    if (count as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[A1][0] = 0x38;
    c.g[V0][0] = Addr(record).off(0xa4).lw(c);
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x4).lw(c);
    let index = abi::mult(c, c.g[V0][0] as u32, 0x38);
    c.g[V0][0] = sx(index);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `BoltsBolt *Bolts_GetCharBolts_2(BoltsDB *db, int charIndex)` (0x14d9d0):
/// identical to `Bolts_GetCharBolts`.
pub fn bolts_get_char_bolts_2(c: &mut Cpu) {
    char_bolts_pointer(c);
}

/// The base of the per-bolt record table.
pub(crate) const BOLT_RECORDS: u32 = 0x004a_0000;

/// Offset of the per-bolt record table within its base.
pub(crate) const BOLT_RECORDS_OFF: u32 = 0x6ca8;

/// `void BEBoltInt_GetBoltStates(int a1, int a2, int a3)` (0x14ae28): read a
/// bolt word into `*a3`.
pub fn be_bolt_int_get_bolt_states(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V1][0] = 0x9b50;
    c.g[A0][0] = 0xf88;
    c.mult(0, V0, A1, V1);
    c.mult(0, A2, A2, A0);
    c.g[V1][0] = BOLT_RECORDS as u64;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(BOLT_RECORDS_OFF));
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = c.g[A0][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x28c)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
    Addr(abi::arg(c, 3)).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `BELibrary_GetInstance` (0x14dc80, `cBE_getBE`): the lazily built BE library
/// singleton at `gp-0x1f04`. Once built (the hot path) it saves `s0`/`s1`/`ra`
/// in its 0x30 frame, reloads the slot and returns it; the one-time
/// construction (null slot: allocation, constructor, sub-singletons) runs the
/// generated body from its start (nothing has been charged or written yet).
pub fn be_library_get_instance(c: &mut Cpu) {
    let slot = (c.g[GP][0] as u32).wrapping_sub(0x1f04);
    if c.lw(slot) == 0 {
        return ee::s01::f_0014dc80__generated(c);
    }
    c.cycles += 12;
    let frame = (c.g[SP][0] as u32).wrapping_sub(0x30);
    c.sq(16, frame.wrapping_add(0x20));
    c.sq(17, frame.wrapping_add(0x10));
    c.sd(frame, c.g[RA][0]);
    c.g[V0][0] = c.lw(slot);
    c.pc = c.g[RA][0] as u32;
}

// ---------------------------------------------------------------------------
// Singleton accessors: each keeps a lazily allocated singleton in a gp-relative
// slot (the address is the gp offset). They return the instance pointer.

/// `BEBoltInt *BEBoltInt_GetInstance(void)` (0x14abe0).
pub fn be_bolt_int_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1f28));
        let have = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !have {
            c.cycles += 11;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx(0x460000u32.wrapping_sub(0x5bf8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, EA_MEM_ALLOC_NAMED, 0x0014_ac08)?;
            c.g[V1][0] = 0x460000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
            c.g[V1][0] = sx(0x460000u32.wrapping_sub(0x5178));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f28), c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1f28));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bolt interface: thin forwarding wrappers around the rider character/profile
// accessors.

/// `Character *BEBoltInt_GetRiderCharacter(BEBoltInt *self, int rider)`
/// (0x14ac30): `BE_GetRiderCharacter(rider)`.
pub fn beb_bolt_int_get_rider_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        BE_GET_RIDER_CHARACTER.call(c, 0x0014_ac40)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderProfile *BEBoltInt_GetRiderProfile(BEBoltInt *self, int rider)`
/// (0x14ac50): `BE_GetRiderProfile(rider)`.
pub fn beb_bolt_int_get_rider_profile(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        BE_GET_RIDER_PROFILE.call(c, 0x0014_ac60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderProfile *BEBoltInt_GetRiderProfileAndChar(BEBoltInt *self, int rider,
/// Character **out)` (0x14ac70): store the rider's character in `*out`, then
/// return the rider's profile.
pub fn beb_bolt_int_get_rider_profile_and_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0014_ac90)?;
        c.g[A0][0] = c.g[S1][0];
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        BE_GET_RIDER_PROFILE.call(c, 0x0014_ac9c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const BOLT_GET_RIDER_PROFILE: Callee = Callee {
    addr: 0x0014_ac50,
    f: ssx3_ee::s01::f_0014ac50,
};

pub(crate) const BOLT_GET_RIDER_CHAR: Callee = Callee {
    addr: 0x0014_ac30,
    f: ssx3_ee::s01::f_0014ac30,
};

pub(crate) const GET_CHAR_PROFILE_DATA: Callee = Callee {
    addr: 0x0014_ad28,
    f: ssx3_ee::s01::f_0014ad28,
};

pub(crate) const BOLT_GET_BOLT_STATES: Callee = Callee {
    addr: 0x0014_ae28,
    f: ssx3_ee::s01::f_0014ae28,
};

pub(crate) const EA_MEM_COPY: u32 = 0x003e_6574;

/// `int BE_FindRiderByProfile(int profile)` (0x14a130): the index (0..5) of the
/// rider whose profile id matches, or -1.
pub fn be_find_rider_by_profile(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x530000;
    c.g[V0][0] = 0x530000;
    c.g[T0][0] = sx(0x530000u32.wrapping_add(0x5b20));
    c.g[A2][0] = sx(0x530000u32.wrapping_add(0x5b0));
    c.g[A1][0] = 0;
    c.g[A3][0] = 0x1c;
    loop {
        c.cycles += 7;
        c.g[V0][0] = lw(c, c.g[A2][0] as u32);
        c.mult(0, V1, V0, A3);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V1][0] = lw(c, (c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.g[V1][0] & 0x1;
        if c.g[V1][0] != c.g[A0][0] {
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 6) as u64;
            let t = c.g[V0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            if t {
                event_check(c, 0x0014_a148);
                continue;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                c.pc = c.g[RA][0] as u32;
                return;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[A1][0];
            c.pc = c.g[RA][0] as u32;
            return;
        }
    }
}

/// `void BEBoltInt_GetRiderProfileData(BEBoltInt *self, int rider, void *out)`
/// (0x14ad50): look up the rider's profile and character and copy the character
/// profile data into `out`.
pub fn beb_bolt_int_get_rider_profile_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BOLT_GET_RIDER_PROFILE.call(c, 0x0014_ad74)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        BOLT_GET_RIDER_CHAR.call(c, 0x0014_ad80)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        GET_CHAR_PROFILE_DATA.call(c, 0x0014_ad90)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BEBoltInt_GetRiderBoltStates(BEBoltInt *self, int rider, int *out)`
/// (0x14ae60): `BEBoltInt_GetBoltStates(self, profile, char, out)`.
pub fn beb_bolt_int_get_rider_bolt_states(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0];
        Callee {
            addr: 0x0014_ac70,
            f: ssx3_ee::s01::f_0014ac70,
        }
        .call(c, 0x0014_ae80)?;
        c.g[A2][0] = lw(c, c.g[SP][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        BOLT_GET_BOLT_STATES.call(c, 0x0014_ae94)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void BEBoltInt_GetRiderProfileData?` — `BEBoltInt_SetBoltStates(BEBoltInt
/// *self, int rider, void *states, ...)` (0x14acb0): `EA_MemCopy(profile*0xf88
/// + rider*0x9b50 + 0x4a6ca8 + 0x290, states, 0x834)`.
pub fn beb_bolt_int_set_bolt_states(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[V0][0] = 0xf88;
        c.g[T0][0] = 0x9b50;
        c.mult(0, V1, A2, V0);
        c.mult(0, A0, A1, T0);
        c.g[V0][0] = 0x4a0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A3][0];
        c.g[A2][0] = 0x834;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x290));
        call_ptr(c, EA_MEM_COPY, 0x0014_ace8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int* BEScoreInt_GetHighScoreTable(int a, int b)` (0x14ab20): the high
/// score table for the event record (`D_00535C18 + type*100`), null for the
/// record type 26.
pub fn be_score_int_get_high_score_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::BE_GET_EVENT_RECORD_INDEX.call(c, 0x0014_ab30)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x1a;
        let unknown = c.g[A0][0] == c.g[V0][0];
        c.g[V1][0] = 0x64;
        if !unknown {
            c.cycles += 5;
            c.g[V0][0] = 0x0053_0000;
            c.mult(0, V1, A0, V1);
            c.g[V0][0] = sx(0x0053_0000u32.wrapping_add(0x5c18));
            c.g[V0][0] = add32(c.g[V1][0], c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}

/// `void Bolts_DestroyDB(Bolts* self, u32 flags)` (0x14c658): free the bolt
/// database and, with bit 0 of `flags`, the object itself.
pub fn bolts_destroy_db(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::BOLTS_FREE_DB.call(c, 0x0014_c674)?;
        c.g[S0][0] &= 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::BUILTIN_DELETE.call(c, 0x0014_c688)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[inline(always)]
pub(crate) fn lhu(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lhu(c)
}

#[inline(always)]
pub(crate) fn sh(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sh(c, v);
}

/// `BEBoltInt_SaveEquipFlags(int a1, int a2, int self)` (0x14aea8): for each
/// bolt entry (`{u16 pad; u16 flags; u16 kind; u16 pad}` at +0x290, `a1*0x9B50
/// + a2*0xF88` into `D_004A6CA8`), set bit 2 when bit 4 is set and clear it
/// otherwise.
pub fn be_bolt_int_save_equip_flags(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = 0xf88;
    c.g[V1][0] = 0x9b50;
    c.mult(0, A2, A2, V0);
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = 0x004a_0000;
    c.g[V0][0] = 0x004a_6ca8;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x28c));
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A1][0] = sx(p(c.g[A1][0] as u32, 0x290));
    if !none {
        c.cycles += 3;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_fffb;
        loop {
            c.cycles += 4;
            c.g[V1][0] = lhu(c, p(c.g[A1][0] as u32, 2));
            c.g[V0][0] = c.g[V1][0] & 0x10;
            let set = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[V1][0] | 4;
            if !set {
                c.cycles += 1;
                c.g[V0][0] = c.g[V1][0] & c.g[A2][0];
            }
            c.cycles += 4;
            sh(c, p(c.g[A1][0] as u32, 2), c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            let more = c.g[A0][0] != 0;
            c.g[A1][0] = sx(p(c.g[A1][0] as u32, 4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_aee0);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void Bolts_IndexBoltsByChar(Bolts* self)` (0x14c2b0): count the bolts per
/// character group and remember the first index of each group.
pub fn bolts_index_bolts_by_char(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(at(c, A0, 0x1c)).lw(c);
    c.g[T1][0] = u64::MAX;
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A3][0] = 0;
    if !none {
        c.cycles += 4;
        c.g[T3][0] = sx(at(c, A0, 0x2c));
        c.g[T2][0] = sx(at(c, A0, 0xa4));
        c.g[T0][0] = 0;
        loop {
            c.cycles += 9;
            c.g[V0][0] = Addr(at(c, A0, 4)).lw(c);
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = Addr(at(c, V0, 0)).lb(c);
            c.g[A2][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V0][0] = Addr(at(c, V1, 0)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            let same = c.g[T1][0] == c.g[A1][0];
            Addr(at(c, V1, 0)).sw(c, c.g[V0][0]);
            if !same {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[T1][0] = c.g[A1][0];
                Addr(at(c, V0, 0)).sw(c, c.g[A3][0]);
            }
            c.cycles += 5;
            c.g[V0][0] = Addr(at(c, A0, 0x1c)).lw(c);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x38));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_c2d0);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `BE_GetEventRecordIndex` (0x14AB68): map a BE event id `b` (and the player
/// `a`) to a score-type index; 26 when there is no record.
pub fn be_get_event_record_index(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A0][0] as i64) < 17) as u64;
    let known_player = c.g[V0][0] != 0;
    c.g[A2][0] = low(c, A1).wrapping_sub(6);
    if !known_player {
        c.cycles += 2;
        c.g[V0][0] = 0x1a;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[A2][0] < 3) as u64;
    let low_id = c.g[V0][0] != 0;
    c.g[V0][0] = low(c, A1).wrapping_sub(9);
    let mut table = false;
    if !low_id {
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 3) as u64;
        // `t` is `(b - 9) < 3` compared against zero: the table scan is the
        // arm where it is *not* a low id (b >= 12), the other one returns b-6.
        let high_id = c.g[V0][0] != 0;
        c.g[V0][0] = 0x46_0000;
        if !high_id {
            table = true;
        }
    }
    if !table {
        c.cycles += 2;
        c.g[V0][0] = c.g[A2][0];
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V1][0] = sx(r32(c, A0) << 4);
    c.g[V0][0] = low(c, V0).wrapping_sub(0x5d08);
    c.g[V1][0] = low(c, V1).wrapping_add(low(c, V0));
    c.g[A2][0] = c.g[V1][0];
    c.g[A3][0] = 0;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
        let hit = c.g[V0][0] == c.g[A1][0];
        c.g[A3][0] = low(c, A3).wrapping_add(1);
        if hit {
            c.cycles += 2;
            c.g[V0][0] = lw_at(c, V1, 0x4);
            abi::ret(c);
            return;
        }
        c.cycles += 4;
        c.g[A2][0] = low(c, A2).wrapping_add(8);
        c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = low(c, V1).wrapping_add(8);
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0x1a;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_abb8);
        }
    }
}

/// `lh` (sign-extended).
#[inline(always)]
pub(crate) fn lh_at(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lh(c)
}

/// `int Bolts_FindBoltIndex(self, group, key)` (0x14d448): the key's index in
/// the group's bolt list — a per-group short lookup table when present, else
/// a linear scan of `self->entries[first .. first+count]` (0x38-byte records,
/// key at +4); -1 when absent.
pub fn bolts_find_bolt_index(c: &mut Cpu) {
    c.cycles += 7;
    let self_ = abi::arg(c, 0);
    let group = abi::arg(c, 1);
    let key = c.g[A2][0];
    c.g[T0][0] = self_ as u64;
    c.g[A1][0] = sx(group << 2);
    let slot = p(self_, group << 2);
    c.g[V0][0] = sx(slot);
    c.g[T1][0] = key;
    let table = lw_at_2(c, p(slot, 0x4dc));
    c.g[A2][0] = table;
    if table != 0 {
        c.cycles += 3;
        c.g[V1][0] = sx(((key as u32) << 1).wrapping_add(table as u32));
        c.g[V0][0] = lh_at(c, c.g[V1][0] as u32);
        abi::ret(c);
        return;
    }
    c.cycles += 10;
    c.g[V1][0] = sx(slot);
    c.g[A0][0] = 0x38;
    let first = lw_at_2(c, p(slot, 0xa4));
    let count = lw_at_2(c, p(slot, 0x2c));
    c.g[A3][0] = first;
    c.g[V1][0] = count;
    mul32(c, A0, A0, A3);
    let base = lw_at_2(c, p(self_, 4));
    let end = first.wrapping_add(count);
    c.g[A1][0] = base;
    c.g[T0][0] = end;
    c.g[V0][0] = ((first as i64) < (end as i64)) as u64;
    c.g[V1][0] = sx((base as u32).wrapping_add(c.g[A0][0] as u32));
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    loop {
        c.cycles += 3;
        c.g[V0][0] = lh_at(c, p(c.g[V1][0] as u32, 4));
        if c.g[V0][0] == c.g[T1][0] {
            c.cycles += 2;
            c.g[V0][0] = c.g[A3][0];
            abi::ret(c);
            return;
        }
        c.cycles += 4;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x38));
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_d490);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    abi::ret(c);
}

/// `int Bolts_IsDescendantOf(self, entry, key)` (0x14d908): 1 when `entry` or
/// one of its ancestors (walking `entry->group` through
/// [`bolts_find_bolt_index`]) is the entry whose group tag is `key`.
pub fn bolts_is_descendant_of(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        let key = c.g[S1][0];
        let mut entry = c.g[A1][0];
        let mut result: u64;
        loop {
            c.cycles += 2;
            if entry == 0 {
                result = 0;
                break;
            }
            c.cycles += 3;
            let group = lh_at(c, (entry as u32).wrapping_add(6));
            c.g[A2][0] = group;
            result = 1;
            if group == key {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            if group != u64::MAX {
                entry = Addr(entry as u32).lb(c);
                c.g[A1][0] = entry;
            } else {
                c.cycles += 2;
                result = 0;
                break;
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = entry;
            c.g[A2][0] = group;
            callee::BOLTS_FIND_BOLT_INDEX.call(c, 0x0014_d950)?;
            c.g[A0][0] = c.g[V0][0];
            let found = (c.g[A0][0] as i64) >= 0;
            c.g[V0][0] = 0x38;
            if !found {
                c.cycles += 2;
                c.g[A1][0] = 0;
                entry = 0;
                if event_due(c) {
                    deliver_event_here(c, 0x0014_d920);
                }
                continue;
            }
            c.cycles += 4;
            c.g[V1][0] = lw_at_2(c, p(self_, 4));
            mul32(c, V0, V0, A0);
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            entry = c.g[A1][0];
            if event_due(c) {
                deliver_event_here(c, 0x0014_d920);
            }
        }
        c.g[V0][0] = result;
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bolt-on scene graph.

pub(crate) const BOLTS_COUNT_ENTRY: u32 = 0x0014_d4c8;

/// The two back edges of the child walk (event resume points).
pub(crate) const BOLTS_COUNT_MATCH: u32 = 0x0014_d510;

pub(crate) const BOLTS_COUNT_NEXT: u32 = 0x0014_d514;

/// `int Bolts_CountChildren(Bolts* self, int group, short key)` (0x14d4c8):
/// with a lookup table, the cached count; otherwise walk the group's entries
/// and count the ones whose key matches.
///
/// The original is a `match pc` dispatcher re-enterable at both loop heads.
pub fn bolts_count_children(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                BOLTS_COUNT_ENTRY => {
                    c.cycles += 6;
                    c.g[T0][0] = c.g[A0][0];
                    c.g[A1][0] = sx(r32(c, A1) << 2);
                    c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, A1)));
                    c.g[V0][0] = Addr(p(r32(c, V0), 0x11c)).lw(c);
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = sx(r32(c, A2) << 1);
                        pc = 0x0014_d548;
                    } else {
                        pc = 0x0014_d4e0;
                    }
                }
                0x0014_d4e0 => {
                    c.cycles += 11;
                    c.g[V1][0] = sx(r32(c, T0).wrapping_add(r32(c, A1)));
                    c.g[A0][0] = 0x38;
                    c.g[A3][0] = Addr(p(r32(c, V1), 0xa4)).lw(c);
                    c.g[V0][0] = c.g[V1][0];
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x2c)).lw(c);
                    c.g[T1][0] = 0;
                    c.mult(0, A0, A3, A0);
                    c.g[V0][0] = Addr(p(r32(c, T0), 4)).lw(c);
                    c.g[A1][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    pc = 0x0014_d518;
                }
                0x0014_d50c => {
                    c.cycles += 1;
                    pc = BOLTS_COUNT_MATCH;
                }
                BOLTS_COUNT_MATCH => {
                    c.cycles += 1;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x38));
                    pc = BOLTS_COUNT_NEXT;
                }
                BOLTS_COUNT_NEXT => {
                    c.cycles += 1;
                    c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                    pc = 0x0014_d518;
                }
                0x0014_d518 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    pc = if c.g[V0][0] == 0 {
                        0x0014_d540
                    } else {
                        0x0014_d524
                    };
                }
                0x0014_d524 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r32(c, V1), 6)).lh(c);
                    let different = c.g[V0][0] != c.g[A2][0];
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    pc = if different { 0x0014_d538 } else { 0x0014_d530 };
                }
                0x0014_d530 => {
                    c.cycles += 2;
                    c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
                    if crate::events::back_edge_unwinds(c, BOLTS_COUNT_MATCH) {
                        return Ok(());
                    }
                    pc = BOLTS_COUNT_MATCH;
                }
                0x0014_d538 => {
                    c.cycles += 2;
                    let advance = c.g[V0][0] == 0;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x38));
                    if !advance {
                        pc = 0x0014_d540;
                    } else {
                        if crate::events::back_edge_unwinds(c, BOLTS_COUNT_NEXT) {
                            return Ok(());
                        }
                        pc = BOLTS_COUNT_NEXT;
                    }
                }
                0x0014_d540 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[T1][0];
                    abi::ret(c);
                    return Ok(());
                }
                0x0014_d548 => {
                    c.cycles += 3;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[V0][0] = Addr(r32(c, V1)).lh(c);
                    abi::ret(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, BOLTS_COUNT_ENTRY, 0x0014_d554);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Board equipment.

/// `void BEBoltInt_RestoreAllEquipFlags(void)` (0x14af10): for the three board
/// slots and their ten equipment entries, restore the "owned" flag bit 0x10
/// on entries whose bit 4 was set, and clear it elsewhere.
pub fn be_bolt_int_restore_all_equip_flags(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x004a_0000;
    c.g[A0][0] = 0;
    c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
    c.g[T5][0] = 0x9b50;
    c.g[T3][0] = 0xf88;
    c.mult(0, 9, 4, 13);
    loop {
        c.cycles += 4;
        c.g[V1][0] = 0;
        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        c.mult(0, 4, 3, 11);
        loop {
            c.cycles += 6;
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T4][0] as u32));
            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = lw_2(c, p(c.g[V0][0] as u32, 0x28c));
            let empty = (c.g[V1][0] as i64) <= 0;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
            if !empty {
                c.cycles += 2;
                c.g[A2][0] = c.g[V1][0];
                c.g[T0][0] = 0xffff_ffff_ffff_ffef;
                loop {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 2)).lhu(c);
                    c.g[A1][0] = c.g[V0][0] | 0x10;
                    c.g[V1][0] = c.g[V0][0] & c.g[T0][0];
                    c.g[V0][0] = c.g[V0][0] & 0x4;
                    let clear = c.g[V0][0] == 0;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                    if clear {
                        c.cycles += 1;
                        c.sh(p(c.g[A0][0] as u32, 2), c.g[V1][0]);
                    } else {
                        c.cycles += 2;
                        c.sh(p(c.g[A0][0] as u32, 2), c.g[A1][0]);
                    }
                    c.cycles += 2;
                    let more = c.g[A2][0] != 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014_af58);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[A3][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10) as u64;
            let more = c.g[V0][0] != 0;
            c.mult(0, 4, 3, 11);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_af38);
                }
                continue;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[T2][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
            let more_outer = c.g[V0][0] != 0;
            c.mult(0, 9, 4, 13);
            if more_outer {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_af28);
                }
                continue;
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        }
    }
}

// ---------------------------------------------------------------------------
// Table and pool helpers.

/// `void Bolts_IndexTable2ByChar(self)` (0x14c320): clear the per-character
/// group counts and record the first entry of each group.
pub fn bolts_index_table2_by_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x30);
        c.g[A2][0] = 0x78;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = sx(p(r32(c, S0), 0x20c));
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003e_6448, 0x0014c34c)?;
        c.g[S2][0] = 0x1e;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x20)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[T1][0] = sx(p(r32(c, S0), 0x284));
            c.g[T0][0] = 0;
            loop {
                c.cycles += 9;
                c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                c.g[V0][0] = sx(p(r32(c, T0), r32(c, V0)));
                c.g[A1][0] = Addr(r32(c, V0)).lb(c);
                c.g[A2][0] = sx(r32(c, A1) << 2);
                c.g[V1][0] = sx(p(r32(c, A0), r32(c, A2)));
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                let same = c.g[S2][0] == c.g[A1][0];
                Addr(r32(c, V1)).sw(c, c.g[V0][0]);
                if !same {
                    c.cycles += 3;
                    c.g[V0][0] = sx(p(r32(c, T1), r32(c, A2)));
                    c.g[S2][0] = c.g[A1][0];
                    Addr(r32(c, V0)).sw(c, c.g[A3][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x20)).lw(c);
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T0][0] = sx(p(r32(c, T0), 0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_c368);
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
    })
}

// ---------------------------------------------------------------------------
// `Bolts_FindFirstChild` (0x14d558): find the bolt entry whose 16-bit key
// matches (`key == -1` or a null lookup table falls back to a linear scan)
// and return the entry's target pointer, or null.
pub fn bolts_find_first_child(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    let no_key = c.g[A2][0] == c.g[V0][0];
    c.g[T0][0] = c.g[A0][0];
    if !no_key {
        c.cycles += 5;
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x194));
        if c.g[A0][0] != 0 {
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
            c.cycles += 5;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = c.lh(c.g[V1][0] as u32);
            let negative = (c.g[V0][0] as i64) <= 0;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            if negative {
                c.cycles += 2;
                c.g[V0][0] = 0u64;
                abi::ret(c);
                return;
            }
            c.cycles += 6;
            c.g[A1][0] = 0x38;
            c.g[V0][0] = c.lh(c.g[V1][0] as u32);
            c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
            c.mult(0, 2, 2, 5);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            abi::ret(c);
            return;
        }
    }
    // Linear scan over the group's entry range.
    c.cycles += 12;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = 0x38;
    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
    c.g[V0][0] = c.g[V1][0];
    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
    c.mult(0, 4, 7, 4);
    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
    let empty = c.g[V0][0] == 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    if !empty {
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x6));
            let hit = c.g[V0][0] == c.g[A2][0];
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            if hit {
                c.cycles += 2;
                c.g[V0][0] = c.g[V1][0];
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x38));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_d5a8);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    abi::ret(c);
}

/// `void* Bolts_FindTable2EntriesByFlag(Bolts* self, int index, int kind,
/// int want, int* out)` (0x14db40): walk the `table2[index]` entries (`kind`
/// at +2, nonzero flag at +1) and return the first entry whose kind and flag
/// match, counting every match in `*out`.
pub fn bolts_find_table2_entries_by_flag(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    Addr(c.g[T0][0] as u32).sw(c, 0);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20c));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T2][0] = Addr(c.g[V1][0] as u32).lw(c);
    let none = (c.g[T2][0] as i64) <= 0;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x284));
    if none {
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[V0][0] = 0;
        c.pc = ra;
        return;
    }
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let empty = c.g[V0][0] == 0;
    c.g[T3][0] = 0;
    if empty {
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[V0][0] = c.g[T3][0];
        c.pc = ra;
        return;
    }
    c.cycles += 3;
    c.g[T4][0] = c.g[V1][0];
    c.mult(0, 10, 9, 2);
    loop {
        c.cycles += 5;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(2).lh(c);
        if c.g[V0][0] != c.g[A2][0] {
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                let ra = c.g[RA][0] as u32;
                c.g[V0][0] = c.g[T3][0];
                c.pc = ra;
                return;
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(1).lb(c);
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            if c.g[V0][0] != c.g[A3][0] {
                c.cycles += 2;
                if c.g[T3][0] != 0 {
                    let ra = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[T3][0];
                    c.pc = ra;
                    return;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
                if c.g[T3][0] == 0 {
                    c.g[T3][0] = c.g[V1][0];
                }
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                Addr(c.g[T0][0] as u32).sw(c, c.g[V0][0]);
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            }
        }
        c.cycles += 6;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
        c.g[V1][0] = Addr(c.g[T4][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0xc));
        if !more {
            let ra = c.g[RA][0] as u32;
            c.g[V0][0] = c.g[T3][0];
            c.pc = ra;
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_db88);
        }
    }
}

/// `lb` of `a + off`.
#[inline(always)]
pub(crate) fn lb(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lb(c)
}

// ---------------------------------------------------------------------------
// `Bolts_IndexTable3ByChar` (0x0014c3c8) / `Bolts_IndexTable4ByChar`
// (0x0014c488): clear a per-character index table and rebuild it from the
// character bytes, keeping the first occurrence of each character.

pub fn bolts_index_table3_by_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x40);
        c.g[A2][0] = 0x78;
        let this = c.g[A0][0] as u32;
        frame.save(c, 17, 0x30);
        c.g[S0][0] = sx(this);
        c.g[S1][0] = sx(this.wrapping_add(0x2fc));
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[A0][0] = sx(this.wrapping_add(0x2fc));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(this.wrapping_add(0x374));
        call_ptr(c, 0x003e_6448, 0x0014_c3fc)?;
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = sx(this.wrapping_add(0x374));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x78;
        call_ptr(c, 0x003e_6448, 0x0014_c410)?;
        c.g[V0][0] = lw_3(c, this, 0x24);
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !t {
            c.cycles += 3;
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[V1][0] = lw_3(c, this, 0xc);
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = lb(c, c.g[V0][0] as u32, 0);
                c.g[A2][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = lw_3(c, c.g[V1][0] as u32, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                let same = c.g[S3][0] == c.g[A1][0];
                sw(c, c.g[V1][0] as u32, 0, c.g[V0][0]);
                if !same {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[S3][0] = c.g[A1][0];
                    sw(c, c.g[V0][0] as u32, 0, c.g[A3][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = lw_3(c, this, 0x24);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = lw_3(c, this, 0xc);
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0014_c428);
                }
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

pub fn bolts_index_table4_by_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x40);
        c.g[A2][0] = 0x78;
        let this = c.g[A0][0] as u32;
        frame.save(c, 17, 0x30);
        c.g[S0][0] = sx(this);
        c.g[S1][0] = sx(this.wrapping_add(0x3ec));
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[A0][0] = sx(this.wrapping_add(0x3ec));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(this.wrapping_add(0x464));
        call_ptr(c, 0x003e_6448, 0x0014_c4bc)?;
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = sx(this.wrapping_add(0x464));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x78;
        call_ptr(c, 0x003e_6448, 0x0014_c4d0)?;
        c.g[V0][0] = lw_3(c, this, 0x28);
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !t {
            c.cycles += 3;
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[V1][0] = lw_3(c, this, 0x10);
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = lb(c, c.g[V0][0] as u32, 0);
                c.g[A2][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = lw_3(c, c.g[V1][0] as u32, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                let same = c.g[S3][0] == c.g[A1][0];
                sw(c, c.g[V1][0] as u32, 0, c.g[V0][0]);
                if !same {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[S3][0] = c.g[A1][0];
                    sw(c, c.g[V0][0] as u32, 0, c.g[A3][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = lw_3(c, this, 0x28);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = lw_3(c, this, 0x10);
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0014_c4e8);
                }
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

/// `int BEBoltInt_SumEquippedBoltValue(cBEState*, int charIndex, int, int)`:
/// sum the `+0xc` values of the character's bolts whose equipped bit (0x10)
/// is set in the shared flags table.
pub fn be_bolt_int_sum_equipped_bolt_value(c: &mut Cpu) {
    guest(c, |c| {
        'b_14b538: {
            c.cycles += 28;
            c.g[SP][0] = sub(c.g[SP][0], 0x60);
            c.g[V0][0] = 0x9b50;
            c.sq(17, p(c.g[SP][0] as u32, 0x40));
            c.mult(0, 7, 5, 2);
            c.g[S1][0] = c.g[A2][0];
            c.sq(16, p(c.g[SP][0] as u32, 0x50));
            c.g[A2][0] = 0xf88;
            c.g[V1][0] = 0x4a_0000;
            c.mult(0, 6, 17, 6);
            c.g[V1][0] = sx(0x004a_6ca8u32.wrapping_add(0));
            c.g[S0][0] = 0x4a_0000;
            c.sq(20, p(c.g[SP][0] as u32, 0x10));
            c.g[S0][0] = sx(0x004a_6750u32.wrapping_add(0));
            c.sq(18, p(c.g[SP][0] as u32, 0x30));
            c.sq(19, p(c.g[SP][0] as u32, 0x20));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = addu(c.g[A2][0], c.g[V1][0]);
            c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[S4][0] = addu(c.g[A3][0], c.g[A2][0]);
            call_ptr(c, 0x0014_d988, 0x0014_b4cc)?;
            c.g[S2][0] = 0;
            c.g[S3][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0014_d998, 0x0014_b4e0)?;
            let none = (c.g[S3][0] as i64) <= 0;
            c.g[V1][0] = c.g[V0][0];
            if none {
                break 'b_14b538;
            }
            c.cycles += 2;
            c.g[A1][0] = lw(c, p(c.g[S4][0] as u32, 0x288));
            c.g[A0][0] = c.g[S3][0];
            loop {
                'b_14b518: {
                    'b_14b514: {
                        c.cycles += 6;
                        c.g[V0][0] = c.lh(p(c.g[V1][0] as u32, bolts::FLAGS));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                        c.g[V0][0] = addu(c.g[V0][0], c.g[A1][0]);
                        c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                        let negative = (c.g[V0][0] as i64) < 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if !negative {
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                            c.g[V0][0] = addu(c.g[S4][0], c.g[V0][0]);
                            break 'b_14b518;
                        }
                        break 'b_14b514;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break 'b_14b518;
                }
                'b_14b530: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu(p(c.g[V0][0] as u32, 2));
                    c.g[V0][0] = c.g[V0][0] & 0x10;
                    let unequipped = c.g[V0][0] == 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    if unequipped {
                        break 'b_14b530;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.lh(p(c.g[V1][0] as u32, bolts::VALUE));
                    c.g[S2][0] = addu(c.g[S2][0], c.g[V0][0]);
                    break 'b_14b530;
                }
                c.cycles += 2;
                let more = c.g[A0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(bolts::STRIDE));
                if !more {
                    break 'b_14b538;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_b4f0);
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S2][0];
        c.lq(16, p(c.g[SP][0] as u32, 0x50));
        c.lq(17, p(c.g[SP][0] as u32, 0x40));
        c.lq(18, p(c.g[SP][0] as u32, 0x30));
        c.lq(19, p(c.g[SP][0] as u32, 0x20));
        c.lq(20, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    });
}

/// `Bolts_LoadDB(self)` (0x14c6a0): load the bolts database (read the file,
/// release the old one, index the four tables by character), then set bit 4
/// of every bolt entry's flags.
pub fn bolts_load_db(c: &mut Cpu) {
    const FREE_DB: Callee = Callee {
        addr: 0x0014_d240,
        f: ee::s01::Bolts_FreeDB,
    };
    const READ_FILE: Callee = Callee {
        addr: 0x0014_be70,
        f: ee::s01::Bolts_ReadFile,
    };
    const INDEX1: Callee = Callee {
        addr: 0x0014_c2b0,
        f: ee::s01::Bolts_IndexBoltsByChar,
    };
    const INDEX2: Callee = Callee {
        addr: 0x0014_c320,
        f: ee::s01::Bolts_IndexTable2ByChar,
    };
    const INDEX3: Callee = Callee {
        addr: 0x0014_c3c8,
        f: ee::s01::Bolts_IndexTable3ByChar,
    };
    const INDEX4: Callee = Callee {
        addr: 0x0014_c488,
        f: ee::s01::Bolts_IndexTable4ByChar,
    };

    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_4(c, p(this, 0x10));
        if c.g[V0][0] == 0 {
            c.cycles += 15;
            c.g[S0][0] = lw_4(c, p(this, 4));
            c.g[S0][0] = (c.g[S0][0] < 1) as u64;
            c.g[A0][0] = c.g[S1][0];
            FREE_DB.call(c, 0x0014_c6cc)?;
            c.g[A0][0] = c.g[S1][0];
            READ_FILE.call(c, 0x0014_c6d4)?;
            c.g[A0][0] = c.g[S1][0];
            INDEX1.call(c, 0x0014_c6dc)?;
            c.g[A0][0] = c.g[S1][0];
            INDEX2.call(c, 0x0014_c6e4)?;
            c.g[A0][0] = c.g[S1][0];
            INDEX3.call(c, 0x0014_c6ec)?;
            c.g[A0][0] = c.g[S1][0];
            INDEX4.call(c, 0x0014_c6f4)?;
            let skip = c.g[S0][0] != 0;
            if !skip {
                c.cycles += 7;
                c.g[V0][0] = 0x004a_0000;
                c.g[A0][0] = 0;
                c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                c.g[T4][0] = 0x9b50;
                c.g[T2][0] = 0xf88;
                c.mult(0, T0, A0, T4);
                'outer: loop {
                    c.cycles += 4;
                    c.g[V1][0] = 0;
                    c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                    c.mult(0, A0, V1, T2);
                    loop {
                        c.cycles += 6;
                        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = lw_4(c, p(c.g[V0][0] as u32, 0x28c));
                        let empty = (c.g[V1][0] as i64) <= 0;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                        if !empty {
                            c.cycles += 2;
                            c.g[A1][0] = c.g[V1][0];
                            c.g[A3][0] = 0xffff_ffff_ffff_ffef;
                            loop {
                                c.cycles += 4;
                                c.g[V1][0] = c.lhu(p(c.g[A0][0] as u32, 2));
                                c.g[V0][0] = c.g[V1][0] & 4;
                                let has4 = c.g[V0][0] != 0;
                                c.g[V0][0] = c.g[V1][0] | 0x10;
                                if !has4 {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.g[V1][0] & c.g[A3][0];
                                }
                                c.cycles += 4;
                                c.sh(p(c.g[A0][0] as u32, 2), c.g[V0][0]);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                                let more = c.g[A1][0] != 0;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                                if !more {
                                    break;
                                }
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014_c748);
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V1][0] = c.g[A2][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 10) as u64;
                        let more = c.g[V0][0] != 0;
                        c.mult(0, A0, V1, T2);
                        if !more {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[T1][0];
                            c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
                            if c.g[V0][0] != 0 {
                                c.mult(0, T0, A0, T4);
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014_c718);
                                }
                                continue 'outer;
                            }
                            c.cycles += 1;
                            break 'outer;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0014_c728);
                        }
                    }
                }
            }
            c.lq(16, frame.at(c, 0x20));
        } else {
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *EA_MemSet(void*, int, int)`.
pub(crate) const EA_MEMSET: u32 = 0x003e_6448;

/// `void EA_MemFree(void*)`.
pub(crate) const EA_MEM_FREE: u32 = 0x0031_7e98;

// ---------------------------------------------------------------------------
// Bolts free-db.

/// The eleven 0x78-byte sub-blocks cleared by [`bolts_free_db`].
pub(crate) const BOLT_SUB_BLOCKS: [u32; 11] = [
    0x02c, 0x0a4, 0x11c, 0x194, 0x20c, 0x284, 0x2fc, 0x374, 0x3ec, 0x464, 0x4dc,
];

/// `void Bolts_FreeDB(BoltsDB *db)` (0x14d240): free the database's two heap
/// blocks and clear the eleven per-mode sub-blocks.
pub fn bolts_free_db(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);
        let db = r32(c, S0);

        c.g[A0][0] = Addr(db).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, EA_MEM_FREE, 0x0014_d264)?;
        }
        Addr(db).sw(c, 0);

        c.cycles += 56;
        // The first block's clear also covers the small header words.
        for off in [0x4u32, 0x8, 0xc, 0x10, 0x14, 0x1c, 0x20, 0x24, 0x28] {
            Addr(db).off(off).sw(c, 0);
        }
        c.g[A1][0] = 0;
        for (i, off) in BOLT_SUB_BLOCKS.iter().enumerate() {
            c.g[A2][0] = 0x78;
            c.g[A0][0] = sx(db.wrapping_add(*off));
            call_ptr(c, EA_MEMSET, 0x0014_d29c + 0x10 * i as u32)?;
        }

        c.g[A0][0] = Addr(db).off(0x18).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, EA_MEM_FREE, 0x0014_d350)?;
        }
        Addr(db).off(0x18).sw(c, 0);

        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Bolts_CollectByGroup` (0x0014d7e8): recursively collect the bolt nodes of
/// group `g` whose `parent` equals `key` into `out`, returning how many were
/// written.  The group's node range comes from `self+0xa4+4g` (first) and
/// `self+0x2c+4g` (count) over the 0x38-byte nodes at `*(self+4)`; a matching
/// node is also descended into when `depth != 0` and it is either not
/// `0x20`-flagged or `all` is set.
pub fn bolts_collect_by_group(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S7, 0x20);
        frame.save(c, S6, 0x30);
        c.g[S7][0] = c.g[A1][0]; // g
        frame.save(c, S0, 0x90);
        c.g[S6][0] = c.g[A0][0]; // self
        frame.save(c, S1, 0x80);
        c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
        frame.save(c, S3, 0x60);
        c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save(c, S4, 0x50);
        c.g[A0][0] = 0x38; // node stride
        frame.save(c, S5, 0x40);
        c.g[V0][0] = c.g[V1][0];
        frame.save(c, FP, 0x10);
        c.g[S1][0] = c.g[A3][0]; // out
        frame.save_ra(c, 0);
        c.g[FP][0] = c.g[A2][0]; // key
        frame.save(c, S2, 0x70);
        c.g[S5][0] = c.g[T0][0]; // depth
        c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4)); // first
        c.g[S4][0] = c.g[T1][0]; // all
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c)); // count
        c.g[S3][0] = 0u64; // collected
        c.mult(0, A0, S2, A0); // first * 0x38
        c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4)); // nodes
        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32)); // end
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if !empty {
            c.cycles += 2;
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S2][0] as u32)); // remaining
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x8));
                if c.g[V0][0] != c.g[FP][0] {
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                } else {
                    c.cycles += 4;
                    c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                    let leaf = c.g[S5][0] == 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    let mut stop = false;
                    if !leaf {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                        c.g[V0][0] = c.g[V0][0] & 0x20;
                        if c.g[V0][0] == 0 {
                            c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                        } else {
                            c.cycles += 2;
                            if c.g[S4][0] == 0 {
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                                stop = true;
                            } else {
                                c.cycles += 1;
                                c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                            }
                        }
                    }
                    if !stop {
                        c.cycles += 9;
                        c.g[A3][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S6][0];
                        c.g[A1][0] = c.g[S7][0];
                        c.g[T0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                        c.g[T1][0] = c.g[S4][0];
                        call_ptr(c, 0x0014_d7e8, 0x0014_d8b8)?;
                        c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.cycles += 1;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    } else {
                        // The child was skipped: the caller's loop continues.
                    }
                }
                c.cycles += 2;
                let more = c.g[S2][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_d868);
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void func_0014B560(void* self, int rider, int chr, int key, int flag)`
/// (0x14b560): unlock bolt `key` of character `chr`: mark the entry unlocked,
/// unlock every bolt whose scale is the negative key, then either spend the
/// bolt's price or grant it for free.
pub fn be_bolt_int_unlock_bolt(c: &mut Cpu) {
    let frame = Frame::open(c, 0xb0);
    'b_14b650: {
        c.cycles += 44;
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        sq(c, 30, at_2(c, SP, 0x20));
        sw_2(c, r32(c, SP), c.g[T0][0]);
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        frame.save(c, S7, 0x30);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x0014b5a0);
        c.g[S6][0] = c.g[A1][0];
        if callee::C0014DC80.call(c, 0x0014b5a0).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0014b5ac);
        c.g[A1][0] = 0xb;
        if callee::C0014DD58.call(c, 0x0014b5ac).is_err() {
            return;
        }
        c.g[V1][0] = 0xf88;
        c.g[A0][0] = 0x9b50;
        c.mult(0, V1, S4, V1);
        c.g[FP][0] = c.g[V0][0];
        c.mult(0, A0, S6, A0);
        c.g[V0][0] = 0x4a0000;
        c.g[V0][0] = sx(at_2(c, V0, 0x6ca8));
        c.g[A2][0] = 0x1;
        c.g[A1][0] = c.g[S5][0];
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[S2][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        c.g[RA][0] = sx(0x0014b5e0);
        c.g[A0][0] = c.g[S2][0];
        if callee::C00151C48.call(c, 0x0014b5e0).is_err() {
            return;
        }
        c.g[RA][0] = sx(0x0014b5e8);
        if callee::C0014BDB8.call(c, 0x0014b5e8).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0014b5f4);
        c.g[A1][0] = c.g[S4][0];
        if callee::C0014D998.call(c, 0x0014b5f4).is_err() {
            return;
        }
        c.g[RA][0] = sx(0x0014b5fc);
        c.g[S0][0] = c.g[V0][0];
        if callee::C0014BDB8.call(c, 0x0014b5fc).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0014b608);
        c.g[A1][0] = c.g[S4][0];
        if callee::C0014D988.call(c, 0x0014b608).is_err() {
            return;
        }
        let t = (c.g[V0][0] as i64) <= 0;

        if t {
            break 'b_14b650;
        } else {
            c.cycles += 4;
            c.g[S1][0] = c.g[V0][0];
            c.g[S7][0] = 0xa;
            c.g[S3][0] = sx(0u32.wrapping_sub(r32(c, S5)));
            'l_14b620: loop {
                'b_14b62c: {
                    c.cycles += 3;
                    c.g[V0][0] = lh(c, at_2(c, S0, 0xe));
                    if (c.g[V0][0] as i64) > 0 {
                        c.mult(0, V0, V0, S7);
                        break 'b_14b62c;
                    } else {
                        break 'b_14b62c;
                    }
                }
                'b_14b648: {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x1));
                        break 'b_14b648;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = lh(c, at_2(c, S0, 0x4));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x0014b644);
                        c.g[A2][0] = 0x1;
                        if callee::C00151C48.call(c, 0x0014b644).is_err() {
                            return;
                        }
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x1));
                        break 'b_14b648;
                    }
                }
                c.cycles += 2;
                let t = c.g[S1][0] != 0u64;
                c.g[S0][0] = sx(at_2(c, S0, 0x38));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014b620);
                    }
                    continue 'l_14b620;
                } else {
                    break 'b_14b650;
                }
            }
        }
    }
    'b_14b68c: {
        'b_14b67c: {
            c.cycles += 9;
            c.g[RA][0] = sx(0x0014b658);
            if callee::C0014BDB8.call(c, 0x0014b658).is_err() {
                return;
            }
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S5][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0014b66c);
            c.g[A1][0] = c.g[S4][0];
            if callee::C0014D448.call(c, 0x0014b66c).is_err() {
                return;
            }
            let t = (c.g[V0][0] as i64) >= 0;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'b_14b67c;
            } else {
                c.cycles += 2;
                let t = true;
                c.g[S0][0] = 0u64;
                if t {
                    break 'b_14b68c;
                } else {
                    break 'b_14b67c;
                }
            }
        }
        c.cycles += 4;
        c.g[V0][0] = 0x38;
        c.g[V1][0] = lw_5(c, at_2(c, S0, 0x4));
        c.mult(0, V0, A0, V0);
        c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        break 'b_14b68c;
    }
    'b_14b6d0: {
        'b_14b6c0: {
            c.cycles += 3;
            c.g[V0][0] = lw_5(c, r32(c, SP));
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[FP][0];
            if t {
                break 'b_14b6c0;
            } else {
                'b_14b6ac: {
                    c.cycles += 3;
                    c.g[A3][0] = lh(c, at_2(c, S0, 0xe));
                    let t = (c.g[A3][0] as i64) <= 0;
                    c.g[V0][0] = 0xa;
                    if t {
                        break 'b_14b6ac;
                    } else {
                        c.cycles += 2;
                        c.mult(0, A3, A3, V0);
                        c.g[A0][0] = c.g[FP][0];
                        break 'b_14b6ac;
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x0014b6b8);
                c.g[A2][0] = c.g[S4][0];
                if callee::C00150B48.call(c, 0x0014b6b8).is_err() {
                    return;
                }
                let t = true;
                frame.restore(c, S0, 0xa0);
                if t {
                    break 'b_14b6d0;
                } else {
                    break 'b_14b6c0;
                }
            }
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[S6][0];
        c.g[RA][0] = sx(0x0014b6cc);
        c.g[A2][0] = c.g[S4][0];
        if callee::C00150928.call(c, 0x0014b6cc).is_err() {
            return;
        }
        frame.restore(c, S0, 0xa0);
        break 'b_14b6d0;
    }
    c.cycles += 11;
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    lq(c, 30, at_2(c, SP, 0x20));
    c.g[RA][0] = ld(c, at_2(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_2(c, SP, 0xb0));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// BE_GetRiderAltCharacter (0x14a0b0)

/// The rider -> character id table (`D_005305B0`, one word per rider) and
/// the character table (`D_00535B20`, 0x1c bytes each; +0x12 is the signed
/// alternate-character byte).
pub(crate) const BE_RIDER_CHARACTER_IDS: u32 = 0x0053_05b0;

pub(crate) const BE_CHARACTERS: u32 = 0x0053_5b20;

pub(crate) const BE_CHARACTER_SIZE: u64 = 0x1c;

pub(crate) const BE_CHARACTER_ALT: u32 = 0x12;

/// `int BE_GetRiderAltCharacter(int rider)` (0x14a0b0): the alternate
/// character byte (signed) of the rider's character entry.
pub fn be_get_rider_alt_character(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = sx(BE_RIDER_CHARACTER_IDS);
    c.g[A0][0] = sx(r32(c, A0) << 2);
    c.g[A2][0] = BE_CHARACTER_SIZE;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(BE_RIDER_CHARACTER_IDS));
    c.g[V1][0] = c.lw(r32(c, A0)); // character id
    c.g[A1][0] = sx(BE_CHARACTERS);
    c.mult(0, V1, V1, A2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(BE_CHARACTERS));
    c.g[V0][0] = c.lb(r32(c, V1).wrapping_add(BE_CHARACTER_ALT));
    ret(c);
}

// ---------------------------------------------------------------------------
// BEBoltInt_GetBoltTextureName (0x14b988)

/// `BELibrary_GetInstance` (0x0014dc80).
pub(crate) const BELIBRARY_GET_INSTANCE_CALL: Callee = Callee {
    addr: 0x0014_dc80,
    f: ee::s01::BELibrary_GetInstance,
};

/// `BELibrary_GetInterface` (0x0014dd58).
pub(crate) const BELIBRARY_GET_INTERFACE_CALL: Callee = Callee {
    addr: 0x0014_dd58,
    f: ee::s01::BELibrary_GetInterface,
};

/// `BE_GetCharProfileData` (0x0014ad28).
pub(crate) const BE_GET_CHAR_PROFILE_DATA_CALL: Callee = Callee {
    addr: 0x0014_ad28,
    f: ee::s01::BE_GetCharProfileData,
};

/// `Bolts_GetCharBoltCount` (0x0014d988).
pub(crate) const BOLTS_GET_CHAR_BOLT_COUNT_CALL: Callee = Callee {
    addr: 0x0014_d988,
    f: ee::s01::Bolts_GetCharBoltCount,
};

/// `Bolts_GetCharBolts` (0x0014d998).
pub(crate) const BOLTS_GET_CHAR_BOLTS_CALL: Callee = Callee {
    addr: 0x0014_d998,
    f: ee::s01::Bolts_GetCharBolts,
};

/// `int BEBoltInt::GetBoltTextureName(this, char, rider, const sBoltEntry* e0,
/// char* out)` (0x14b988): build the texture file name of a bolt. Copies
/// `e0->name` (+0x2c) to `out`; if it holds a `$` placeholder, fills the
/// placeholder characters from the names of the other bolts of the same
/// group (+3) that the rider has unlocked (profile flag 0x10). Returns 0
/// with an empty string when `e0` has no group (-1) or no name, else 1.
///
/// Mechanical port: the control flow keeps the generated block structure.
pub fn beb_bolt_int_get_bolt_texture_name(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x90 };
        'b_14bb74: {
            'b_14b9e4: {
                'b_14b9d8: {
                    c.cycles += 17;
                    Frame::open(c, 0x90);
                    c.g[V0][0] = 0xffffffffffffffff;
                    frame.save(c, S0, 0x80);
                    frame.save(c, S1, 0x70);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, S3, 0x50);
                    c.g[S1][0] = c.g[A2][0];
                    frame.save(c, S4, 0x40);
                    frame.save(c, S2, 0x60);
                    c.g[S4][0] = c.g[A3][0];
                    frame.save(c, S5, 0x30);
                    frame.save(c, S7, 0x10);
                    frame.save_ra(c, 0);
                    frame.save(c, S6, 0x20);
                    c.g[S6][0] = c.lb(r32(c, S4).wrapping_add(0x3));
                    let t = c.g[S6][0] == c.g[V0][0];
                    c.g[S3][0] = c.g[T0][0];
                    if t {
                        break 'b_14b9d8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, S4).wrapping_add(0x2c));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x4a0000;
                        if t {
                            break 'b_14b9e4;
                        } else {
                            break 'b_14b9d8;
                        }
                    }
                }
                c.cycles += 3;
                c.sb(r32(c, S3), 0u64);
                let t = true;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_14bb74;
                } else {
                    break 'b_14b9e4;
                }
            }
            c.cycles += 19;
            c.g[S7][0] = sx(r32(c, V0).wrapping_add(0x6750));
            BELIBRARY_GET_INSTANCE_CALL.call(c, 0x0014_b9ec)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x9;
            BELIBRARY_GET_INTERFACE_CALL.call(c, 0x0014_b9f8)?;
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[V0][0];
            BE_GET_CHAR_PROFILE_DATA_CALL.call(c, 0x0014_ba08)?;
            c.g[A1][0] = c.lw(r32(c, S4).wrapping_add(0x2c));
            c.g[A0][0] = c.g[S3][0];
            c.g[S2][0] = c.g[V0][0];
            call_ptr(c, 0x0041_66f4, 0x0014_ba18)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0x24;
            call_ptr(c, 0x0041_6404, 0x0014_ba24)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_14bb74;
            } else {
                'b_14bb70: {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A1][0] = c.g[S1][0];
                    BOLTS_GET_CHAR_BOLT_COUNT_CALL.call(c, 0x0014_ba3c)?;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A1][0] = c.g[S1][0];
                    BOLTS_GET_CHAR_BOLTS_CALL.call(c, 0x0014_ba4c)?;
                    c.g[T0][0] = c.g[V0][0];
                    let t = (c.g[S5][0] as i64) <= 0;
                    c.g[T3][0] = 0u64;
                    if t {
                        break 'b_14bb70;
                    } else {
                        c.cycles += 2;
                        c.g[T5][0] = 0x2e;
                        c.g[T4][0] = 0x24;
                        'l_14ba60: loop {
                            'b_14bb64: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lb(r32(c, T0).wrapping_add(0x3));
                                if c.g[V0][0] != c.g[S6][0] {
                                    c.g[T3][0] = sx(r32(c, T3).wrapping_add(0x1));
                                    break 'b_14bb64;
                                } else {
                                    c.cycles += 3;
                                    c.g[T2][0] = c.lw(r32(c, T0).wrapping_add(0x2c));
                                    if c.g[T2][0] == 0u64 {
                                        c.g[T3][0] = sx(r32(c, T3).wrapping_add(0x1));
                                        break 'b_14bb64;
                                    } else {
                                        'b_14bb60: {
                                            c.cycles += 4;
                                            c.g[A0][0] = c.lh(r32(c, T0).wrapping_add(0x4));
                                            c.g[V0][0] = c.lh(r32(c, S4).wrapping_add(0x4));
                                            let t = c.g[A0][0] == c.g[V0][0];
                                            c.g[V0][0] = sx(r32(c, A0) << 1);
                                            if t {
                                                break 'b_14bb60;
                                            } else {
                                                'b_14baac: {
                                                    'b_14baa8: {
                                                        c.cycles += 5;
                                                        c.g[V1][0] =
                                                            c.lw(r32(c, S2).wrapping_add(0x288));
                                                        c.g[V0][0] =
                                                            sx(r32(c, V0).wrapping_add(r32(c, V1)));
                                                        c.g[V0][0] = c.lh(r32(c, V0));
                                                        let t = (c.g[V0][0] as i64) < 0;
                                                        c.g[V0][0] = sx(r32(c, V0) << 2);
                                                        if t {
                                                            break 'b_14baa8;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] =
                                                                sx(r32(c, V0).wrapping_add(0x290));
                                                            let t = true;
                                                            c.g[V0][0] =
                                                                sx(r32(c, S2)
                                                                    .wrapping_add(r32(c, V0)));
                                                            if t {
                                                                break 'b_14baac;
                                                            } else {
                                                                break 'b_14baa8;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[V0][0] = 0u64;
                                                    break 'b_14baac;
                                                }
                                                c.cycles += 4;
                                                c.g[V0][0] = c.lhu(r32(c, V0).wrapping_add(0x2));
                                                c.g[V0][0] = c.g[V0][0] & 0x10;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[T3][0] = sx(r32(c, T3).wrapping_add(0x1));
                                                    break 'b_14bb64;
                                                } else {
                                                    'b_14bb14: {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c.lb(r32(c, S0));
                                                        c.g[A0][0] = c.g[S0][0];
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A2][0] =
                                                            sx(r32(c, S0).wrapping_sub(r32(c, S3)));
                                                        if t {
                                                            break 'b_14bb14;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == c.g[T5][0];

                                                            if t {
                                                                break 'b_14bb14;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A3][0] = 0x24;
                                                                c.g[T1][0] = 0x2e;
                                                                c.g[V0][0] = sx(r32(c, T2)
                                                                    .wrapping_add(r32(c, A2)));
                                                                'l_14bae0: loop {
                                                                    'b_14baf8: {
                                                                        c.cycles += 3;
                                                                        c.g[V1][0] =
                                                                            c.lb(r32(c, V0));
                                                                        let t = c.g[V1][0]
                                                                            == c.g[A3][0];
                                                                        c.g[A1][0] =
                                                                            c.lbu(r32(c, V0));
                                                                        if t {
                                                                            break 'b_14baf8;
                                                                        } else {
                                                                            c.cycles += 3;
                                                                            c.g[V0][0] =
                                                                                c.lb(r32(c, A0));
                                                                            if c.g[V0][0]
                                                                                == c.g[A3][0]
                                                                            {
                                                                                c.sb(
                                                                                    r32(c, A0),
                                                                                    c.g[A1][0],
                                                                                );
                                                                                break 'b_14baf8;
                                                                            } else {
                                                                                break 'b_14baf8;
                                                                            }
                                                                        }
                                                                    }
                                                                    'b_14bb10: {
                                                                        c.cycles += 4;
                                                                        c.g[A0][0] = sx(r32(c, A0)
                                                                            .wrapping_add(0x1));
                                                                        c.g[V0][0] =
                                                                            c.lb(r32(c, A0));
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[A2][0] = sx(r32(c, A2)
                                                                            .wrapping_add(0x1));
                                                                        if t {
                                                                            break 'b_14bb10;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = c.g[V0][0]
                                                                                != c.g[T1][0];
                                                                            c.g[V0][0] =
                                                                                sx(r32(c, T2)
                                                                                    .wrapping_add(
                                                                                        r32(c, A2),
                                                                                    ));
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0014_bae0);
                                                                                }
                                                                                continue 'l_14bae0;
                                                                            } else {
                                                                                break 'b_14bb10;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[V0][0] = c.lb(r32(c, S0));
                                                                    break 'b_14bb14;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_14bb58: {
                                                        c.cycles += 2;
                                                        if c.g[V0][0] == 0u64 {
                                                            c.g[V0][0] = c.lb(r32(c, S0));
                                                            break 'b_14bb58;
                                                        } else {
                                                            c.cycles += 2;
                                                            if c.g[V0][0] == c.g[T4][0] {
                                                                c.g[T3][0] =
                                                                    sx(r32(c, T3)
                                                                        .wrapping_add(0x1));
                                                                break 'b_14bb64;
                                                            } else {
                                                                c.cycles += 2;
                                                                if c.g[V0][0] == c.g[T5][0] {
                                                                    c.g[V0][0] = c.lb(r32(c, S0));
                                                                    break 'b_14bb58;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A0][0] = 0x24;
                                                                    c.g[V1][0] = 0x2e;
                                                                    c.g[S0][0] = sx(r32(c, S0)
                                                                        .wrapping_add(0x1));
                                                                    'l_14bb38: loop {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] =
                                                                            c.lb(r32(c, S0));
                                                                        let t = c.g[V0][0] == 0u64;

                                                                        if t {
                                                                            break 'b_14bb58;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[A0][0]
                                                                            {
                                                                                c.g[T3][0] = sx(r32(c, T3).wrapping_add(0x1));
                                                                                break 'b_14bb64;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    != c.g[V1][0]
                                                                                {
                                                                                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x1));
                                                                                    if event_due(c)
                                                                                    {
                                                                                        deliver_event_here(c, 0x0014_bb38);
                                                                                    }
                                                                                    continue 'l_14bb38;
                                                                                } else {
                                                                                    c.cycles += 1;
                                                                                    c.g[V0][0] = c
                                                                                        .lb(r32(
                                                                                            c, S0,
                                                                                        ));
                                                                                    break 'b_14bb58;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] != c.g[T4][0];
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_14bb74;
                                                    } else {
                                                        break 'b_14bb60;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[T3][0] = sx(r32(c, T3).wrapping_add(0x1));
                                        break 'b_14bb64;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[T3][0] as i64) < (c.g[S5][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x38));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014_ba60);
                                }
                                continue 'l_14ba60;
                            } else {
                                break 'b_14bb70;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'b_14bb74;
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const BGB_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_dc80,
    f: ee::s01::BELibrary_GetInstance,
};

pub(crate) const BGB_GET_INTERFACE: Callee = Callee {
    addr: 0x0014_dd58,
    f: ee::s01::BELibrary_GetInterface,
};

pub(crate) const BGB_GET_ALT_CHARACTER: Callee = Callee {
    addr: 0x0014_74c8,
    f: ee::s01::BENewPlayerInt_GetAltCharacterBySlot,
};

pub(crate) const BGB_GET_CHARACTER: Callee = Callee {
    addr: 0x0014_7488,
    f: ee::s01::BENewPlayerInt_GetCharacterBySlot,
};

pub(crate) const BGB_GET_CHAR_BOLTS: Callee = Callee {
    addr: 0x0014_d9d0,
    f: ee::s01::Bolts_GetCharBolts_2,
};

pub(crate) const BGB_GET_CHAR_BOLT_COUNT: Callee = Callee {
    addr: 0x0014_d988,
    f: ee::s01::Bolts_GetCharBoltCount,
};

pub(crate) const BGB_FREE_DB: Callee = Callee {
    addr: 0x0014_d240,
    f: ee::s01::Bolts_FreeDB,
};

pub(crate) const BGB_STRLEN: u32 = 0x0041_6810;

pub(crate) const BGB_STRCPY: u32 = 0x0041_66f4;

pub(crate) const BGB_OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;

pub(crate) const BGB_MEM_COPY: u32 = 0x003e_6574;

/// Bolt entry layout (0x38 bytes): short key at +4, a name string at +0x14
/// (kept only for flags 0x100/0x200/0x400/0x800), four strings at
/// +0x18..+0x24, strings at +0x28 and +0x2c, scratch word at +0x30, flags at
/// +0x34 (bit 7 = marked as kept).
pub(crate) const BGB_BOLT_KEY: u32 = 0x4;

pub(crate) const BGB_BOLT_FLAGS: u32 = 0x34;

/// The per-slot character records (`0x9b50` per rider slot, `0xf88` per
/// character) pass 1 consults, and the slot-independent ones of pass 2 and 3.
#[cfg(test)]
pub(crate) const BGB_SLOT_RECORDS: u32 = 0x004a_6ca8;

#[cfg(test)]
pub(crate) const BGB_PLAIN_RECORDS: u32 = 0x004b_a348;

pub(crate) fn bgb_lw(c: &mut Cpu, base: usize, off: u32) -> u64 {
    let a = (c.g[base][0] as u32).wrapping_add(off);
    c.lw(a)
}

/// `sw src, off(base)`.
pub(crate) fn bgb_sw_r(c: &mut Cpu, base: usize, off: u32, src: usize) {
    let a = (c.g[base][0] as u32).wrapping_add(off);
    let v = c.g[src][0];
    c.sw(a, v);
}

/// `sw zero, off(base)`.
pub(crate) fn bgb_sw_zero(c: &mut Cpu, base: usize, off: u32) {
    let a = (c.g[base][0] as u32).wrapping_add(off);
    c.sw(a, 0);
}

pub(crate) fn bgb_addi(c: &mut Cpu, dst: usize, src: usize, imm: u32) {
    c.g[dst][0] = sx((c.g[src][0] as u32).wrapping_add(imm));
}

/// `dst = src + 1` style bump of the string-length accumulator: `s0 = s4 + 1`,
/// `strlen(a0)`, `s4 = s0 + v0`.
pub(crate) fn bgb_add_len(c: &mut Cpu, ret: u32) -> Flow {
    bgb_addi(c, S0, S4, 1);
    call_ptr(c, BGB_STRLEN, ret)?;
    c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    Ok(())
}

/// The four-name loop (`s1` walks the strings at bolt+0x18, `s3` counts 3
/// down to 0): add every name's length plus one to `s4`.
pub(crate) fn bgb_names(c: &mut Cpu, head: u32, ret: u32) -> Flow {
    loop {
        c.cycles += 3;
        c.g[A0][0] = bgb_lw(c, S1, 0);
        if c.g[A0][0] == 0 {
            bgb_addi(c, S3, S3, 0xffff_ffff);
        } else {
            c.cycles += 4;
            bgb_add_len(c, ret)?;
            bgb_addi(c, S3, S3, 0xffff_ffff);
        }
        c.cycles += 2;
        let more = (c.g[S3][0] as i64) >= 0;
        bgb_addi(c, S1, S1, 4);
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The tail of a bolt (`s2`) tally: the +0x2c and +0x28 names, then mark the
/// bolt kept (`flags |= 0x80`) and take the new count `s5` into `fp`.
pub(crate) fn bgb_tail(c: &mut Cpu, pre: u64, ret_a: u32, ret_b: u32) -> Flow {
    c.cycles += pre;
    c.g[A0][0] = bgb_lw(c, S2, 0x2c);
    if c.g[A0][0] == 0 {
        c.g[A0][0] = bgb_lw(c, S2, 0x28);
    } else {
        c.cycles += 4;
        bgb_add_len(c, ret_a)?;
        c.g[A0][0] = bgb_lw(c, S2, 0x28);
    }
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        c.g[V0][0] = bgb_lw(c, S2, 0x34);
    } else {
        c.cycles += 4;
        bgb_add_len(c, ret_b)?;
        c.g[V0][0] = bgb_lw(c, S2, 0x34);
    }
    c.cycles += 3;
    c.g[FP][0] = c.g[S5][0];
    c.g[V0][0] |= 0x80;
    bgb_sw_r(c, S2, 0x34, V0);
    Ok(())
}

/// `s5 = fp + 1; s1 = bolt + 0x18; s3 = 3`: the setup of the name loop.
pub(crate) fn bgb_names_setup(c: &mut Cpu) {
    bgb_addi(c, S5, FP, 1);
    bgb_addi(c, S1, S2, 0x18);
    c.g[S3][0] = 3;
}

/// The key to flags lookup: `*(base + 0x288)` is a table of shorts indexed by
/// the bolt's key (negative = none); the entry is `base + 0x290 + 4 * index`
/// and `dst` receives it (0 when none).
pub(crate) fn bgb_entry(c: &mut Cpu, base: u32, dst: usize) {
    let key = bgb_lh(c, S2, BGB_BOLT_KEY);
    c.g[V0][0] = key;
    c.g[V1][0] = c.lw(base.wrapping_add(0x288));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lh(c.g[V0][0] as u32);
    let none = (c.g[V0][0] as i64) < 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    if !none {
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
        c.g[dst][0] = sx(base.wrapping_add(c.g[V0][0] as u32));
    } else {
        c.cycles += 1;
        c.g[dst][0] = 0;
    }
}

pub(crate) fn bgb_lh(c: &mut Cpu, base: usize, off: u32) -> u64 {
    let a = (c.g[base][0] as u32).wrapping_add(off);
    c.lh(a)
}

pub(crate) fn bgb_lhu_2(c: &mut Cpu, base: usize) -> u64 {
    let a = (c.g[base][0] as u32).wrapping_add(2);
    c.lhu(a)
}

/// Pass 1, character 0..9 (`s0`) of rider slot `s3`: every unmarked bolt of
/// the character whose record flags (selected by the bolt's own flags) say so
/// is tallied (names, then marked kept).
pub(crate) fn bgb_pass1_low(c: &mut Cpu) -> Flow {
    c.cycles += 21;
    c.g[A1][0] = c.g[S0][0];
    c.g[S1][0] = 0;
    BGB_GET_CHAR_BOLTS.call(c, 0x0014_c89c)?;
    bgb_addi(c, A1, S3, 1);
    c.g[A0][0] = bgb_lw(c, SP, 0);
    bgb_sw_r(c, SP, 0x40, A1);
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    BGB_GET_CHAR_BOLT_COUNT.call(c, 0x0014_c8b4)?;
    c.g[V1][0] = 0x9b50;
    c.g[A0][0] = 0x4a0000;
    bgb_sw_r(c, SP, 0xc, V0);
    c.mult(0, V1, S3, V1);
    bgb_addi(c, A0, A0, 0x6ca8);
    c.g[V0][0] = 0xf88;
    c.lo[0] = c.g[A0][0];
    c.g[A1][0] = bgb_lw(c, SP, 0xc);
    c.madd(0, V0, S0, V0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let none = (c.g[A1][0] as i64) <= 0;
    bgb_sw_r(c, SP, 0x10, V1);
    if none {
        c.cycles += 1;
        c.g[S3][0] = bgb_lw(c, SP, 0x40);
        return Ok(());
    }
    c.cycles += 1;
    loop {
        'entry: {
            c.cycles += 6;
            c.g[V0][0] = bgb_lw(c, S2, BGB_BOLT_FLAGS);
            bgb_addi(c, S7, S1, 1);
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] &= 0x80;
            let kept = c.g[V0][0] != 0;
            bgb_addi(c, S6, S2, 0x38);
            if kept {
                break 'entry;
            }
            c.cycles += 8;
            c.g[A2][0] = bgb_lw(c, SP, 0x10);
            let base = c.g[A2][0] as u32;
            bgb_entry(c, base, V1);
            // The bolt's flags (a0) choose which tests apply.
            let mut plain = false;
            c.cycles += 3;
            c.g[V0][0] = c.g[A0][0] & 0x100;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = bgb_lhu_2(c, V1);
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.g[A0][0] & 0x200;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = bgb_lhu_2(c, V1);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A0][0] & 0x400;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = bgb_lhu_2(c, V1);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[A0][0] & 0x800;
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = bgb_lhu_2(c, V1);
                            plain = true;
                        } else {
                            c.cycles += 1;
                            c.g[V1][0] = bgb_lhu_2(c, V1);
                        }
                    }
                }
            }
            if !plain {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 0x10;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = bgb_lw(c, S2, 0x14);
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[V1][0] & 0x2;
                    bgb_addi(c, S7, S1, 1);
                    let skip = c.g[V0][0] != 0;
                    bgb_addi(c, S6, S2, 0x38);
                    if skip {
                        break 'entry;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = bgb_lw(c, S2, 0x14);
                }
                c.cycles += 2;
                let no_name = c.g[A0][0] == 0;
                bgb_addi(c, S7, S1, 1);
                if !no_name {
                    c.cycles += 4;
                    bgb_add_len(c, 0x0014_c994)?;
                    bgb_addi(c, S7, S1, 1);
                }
                c.cycles += 5;
                bgb_addi(c, S5, FP, 1);
                bgb_addi(c, S6, S2, 0x38);
                bgb_addi(c, S1, S2, 0x18);
                c.g[S3][0] = 3;
                bgb_names(c, 0x0014_c9b0, 0x0014_c9c4)?;
                bgb_tail(c, 4, 0x0014_ca30, 0x0014_ca48)?;
            } else {
                c.cycles += 4;
                bgb_addi(c, S7, S1, 1);
                c.g[V0][0] &= 0x10;
                let skip = c.g[V0][0] == 0;
                bgb_addi(c, S6, S2, 0x38);
                if skip {
                    break 'entry;
                }
                c.cycles += 3;
                bgb_names_setup(c);
                bgb_names(c, 0x0014_c9f8, 0x0014_ca0c)?;
                bgb_tail(c, 3, 0x0014_ca30, 0x0014_ca48)?;
            }
        }
        c.cycles += 5;
        c.g[A3][0] = bgb_lw(c, SP, 0xc);
        c.g[S1][0] = c.g[S7][0];
        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A3][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S2][0] = c.g[S6][0];
        if !more {
            c.cycles += 2;
            c.g[S3][0] = bgb_lw(c, SP, 0x40);
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_c8e8);
        }
    }
}

/// Pass 1, character 10..29 (the alternate characters), or any other: the
/// slot counter is saved; for 10..29 every unmarked bolt is tallied.
pub(crate) fn bgb_pass1_high(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xa));
    bgb_addi(c, S3, S3, 1);
    c.g[V0][0] = (c.g[V0][0] < 0x14) as u64;
    let other = c.g[V0][0] == 0;
    bgb_sw_r(c, SP, 0x40, S3);
    if !other {
        c.cycles += 10;
        c.g[A0][0] = bgb_lw(c, SP, 0);
        c.g[A1][0] = c.g[S0][0];
        BGB_GET_CHAR_BOLTS.call(c, 0x0014_ca98)?;
        c.g[A0][0] = bgb_lw(c, SP, 0);
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        BGB_GET_CHAR_BOLT_COUNT.call(c, 0x0014_caa8)?;
        bgb_sw_r(c, SP, 0x14, V0);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if !none {
            c.cycles += 1;
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lbu(bgb_a(c, S2, BGB_BOLT_FLAGS));
                bgb_addi(c, S6, V1, 1);
                c.g[V0][0] >>= 7;
                let kept = c.g[V0][0] != 0;
                bgb_addi(c, S7, S2, 0x38);
                if !kept {
                    c.cycles += 3;
                    bgb_names_setup(c);
                    bgb_names(c, 0x0014_cad8, 0x0014_caec)?;
                    bgb_tail(c, 3, 0x0014_cb10, 0x0014_cb28)?;
                }
                c.cycles += 5;
                c.g[A0][0] = bgb_lw(c, SP, 0x14);
                c.g[V1][0] = c.g[S6][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = c.g[S7][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_cab8);
                }
            }
        }
    }
    c.cycles += 1;
    c.g[S3][0] = bgb_lw(c, SP, 0x40);
    Ok(())
}

pub(crate) fn bgb_a(c: &Cpu, base: usize, off: u32) -> u32 {
    (c.g[base][0] as u32).wrapping_add(off)
}

/// Pass 2: characters 0..9 (`s0`) of the slot-independent records: the
/// unmarked bolts whose record flags have bit 4 are tallied.
pub(crate) fn bgb_pass2(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[S0][0] = 0;
    c.g[V0][0] = 0xf88;
    loop {
        c.cycles += 17;
        c.g[A1][0] = 0x4c0000;
        c.mult(0, V0, S0, V0);
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5cb8));
        c.g[A0][0] = bgb_lw(c, SP, 0);
        bgb_addi(c, A2, S0, 1);
        c.g[A1][0] = c.g[S0][0];
        bgb_sw_r(c, SP, 0x38, A2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        bgb_sw_r(c, SP, 0x18, V0);
        BGB_GET_CHAR_BOLTS.call(c, 0x0014_cb90)?;
        c.g[A0][0] = bgb_lw(c, SP, 0);
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        BGB_GET_CHAR_BOLT_COUNT.call(c, 0x0014_cba0)?;
        bgb_sw_r(c, SP, 0x1c, V0);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = 0;
        if !none {
            c.cycles += 1;
            loop {
                'entry: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lbu(bgb_a(c, S2, BGB_BOLT_FLAGS));
                    bgb_addi(c, S7, A0, 1);
                    c.g[V0][0] >>= 7;
                    let kept = c.g[V0][0] != 0;
                    bgb_addi(c, S6, S2, 0x38);
                    if kept {
                        break 'entry;
                    }
                    c.cycles += 8;
                    c.g[A3][0] = bgb_lw(c, SP, 0x18);
                    let base = c.g[A3][0] as u32;
                    bgb_entry(c, base, V0);
                    c.cycles += 5;
                    c.g[V0][0] = bgb_lhu_2(c, V0);
                    bgb_addi(c, S7, A0, 1);
                    c.g[V0][0] &= 0x10;
                    let skip = c.g[V0][0] == 0;
                    bgb_addi(c, S6, S2, 0x38);
                    if skip {
                        break 'entry;
                    }
                    c.cycles += 4;
                    bgb_names_setup(c);
                    bgb_names(c, 0x0014_cc18, 0x0014_cc2c)?;
                    bgb_tail(c, 3, 0x0014_cc50, 0x0014_cc68)?;
                }
                c.cycles += 5;
                c.g[V1][0] = bgb_lw(c, SP, 0x1c);
                c.g[A0][0] = c.g[S7][0];
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = c.g[S6][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_cbb0);
                }
            }
        }
        c.cycles += 4;
        c.g[S0][0] = bgb_lw(c, SP, 0x38);
        c.g[V0][0] = ((c.g[S0][0] as i64) < 10) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = 0xf88;
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_cb68);
        }
    }
}

/// Pass 3: characters 10..29: every unmarked bolt is tallied.
pub(crate) fn bgb_pass3(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[S0][0] = 0xa;
    c.g[A0][0] = bgb_lw(c, SP, 0);
    loop {
        c.cycles += 11;
        c.g[A1][0] = c.g[S0][0];
        BGB_GET_CHAR_BOLTS.call(c, 0x0014_ccb0)?;
        bgb_addi(c, A0, S0, 1);
        c.g[S2][0] = c.g[V0][0];
        bgb_sw_r(c, SP, 0x2c, A0);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = bgb_lw(c, SP, 0);
        BGB_GET_CHAR_BOLT_COUNT.call(c, 0x0014_ccc8)?;
        bgb_sw_r(c, SP, 0x20, V0);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if !none {
            c.cycles += 1;
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lbu(bgb_a(c, S2, BGB_BOLT_FLAGS));
                bgb_addi(c, S6, V1, 1);
                c.g[V0][0] >>= 7;
                let kept = c.g[V0][0] != 0;
                bgb_addi(c, S7, S2, 0x38);
                if !kept {
                    c.cycles += 3;
                    bgb_names_setup(c);
                    bgb_names(c, 0x0014_ccf8, 0x0014_cd0c)?;
                    bgb_tail(c, 3, 0x0014_cd30, 0x0014_cd48)?;
                }
                c.cycles += 5;
                c.g[A1][0] = bgb_lw(c, SP, 0x20);
                c.g[V1][0] = c.g[S6][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = c.g[S7][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_ccd8);
                }
            }
        }
        c.cycles += 4;
        c.g[S0][0] = bgb_lw(c, SP, 0x2c);
        c.g[V0][0] = ((c.g[S0][0] as i64) < 30) as u64;
        if c.g[V0][0] == 0 {
            return Ok(());
        }
        c.g[A0][0] = bgb_lw(c, SP, 0);
        if event_due(c) {
            deliver_event_here(c, 0x0014_cca8);
        }
    }
}

/// `strcpy` the string `a1` into the block at `s2` and advance `s2` past it
/// (`s2 += strlen(src) + 1`); `a0 = s2` going in, as the original passes it.
pub(crate) fn bgb_copy_string_call(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, BGB_STRCPY, ret)
}

/// Pass 4: allocate the new block, copy every kept bolt and its strings, copy
/// the second table behind it, free the old block and install the new one.
pub(crate) fn bgb_pass4(c: &mut Cpu) -> Flow {
    c.cycles += 23;
    c.g[S0][0] = 0x38;
    c.g[V0][0] = bgb_lw(c, SP, 0x3c);
    c.mult(0, S0, FP, S0);
    c.g[A1][0] = 0x460000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x59d8));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    c.g[S6][0] = 0;
    c.g[S1][0] = 0;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S4][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    call_ptr(c, BGB_OPERATOR_NEW_NAMED, 0x0014_cdb0)?;
    c.g[V1][0] = bgb_lw(c, SP, 0);
    c.g[S5][0] = c.g[V0][0];
    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
    c.g[A0][0] = bgb_lw(c, SP, 0x3c);
    c.g[V0][0] = bgb_lw(c, V1, 0x1c);
    bgb_sw_r(c, SP, 0x24, S0);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[S2][0] = c.g[A0][0];
    c.g[S3][0] = bgb_lw(c, V1, 0x4);
    let none = (c.g[V0][0] as i64) <= 0;
    bgb_sw_r(c, SP, 0x28, A0);
    if !none {
        c.cycles += 1;
        loop {
            'entry: {
                c.cycles += 6;
                c.g[V0][0] = c.lbu(bgb_a(c, S3, BGB_BOLT_FLAGS));
                bgb_addi(c, A1, S1, 1);
                bgb_sw_r(c, SP, 0x34, A1);
                c.g[V0][0] >>= 7;
                let skip = c.g[V0][0] == 0;
                bgb_addi(c, S7, S3, 0x38);
                if skip {
                    break 'entry;
                }
                c.cycles += 39;
                // a0 = new bolt (new block + 0x38 * s6); copy the 0x38 bytes.
                c.g[V0][0] = 0x38;
                c.mult(0, V0, S6, V0);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                let (src, dst) = (c.g[S3][0] as u32, c.g[A0][0] as u32);
                c.ldl(3, src.wrapping_add(0x7));
                c.ldr(3, src);
                c.ldl(5, src.wrapping_add(0xf));
                c.ldr(5, src.wrapping_add(0x8));
                c.ldl(6, src.wrapping_add(0x17));
                c.ldr(6, src.wrapping_add(0x10));
                c.ldl(7, src.wrapping_add(0x1f));
                c.ldr(7, src.wrapping_add(0x18));
                c.sdl(3, dst.wrapping_add(0x7));
                c.sdr(3, dst);
                c.sdl(5, dst.wrapping_add(0xf));
                c.sdr(5, dst.wrapping_add(0x8));
                c.sdl(6, dst.wrapping_add(0x17));
                c.sdr(6, dst.wrapping_add(0x10));
                c.sdl(7, dst.wrapping_add(0x1f));
                c.sdr(7, dst.wrapping_add(0x18));
                c.ldl(3, src.wrapping_add(0x27));
                c.ldr(3, src.wrapping_add(0x20));
                c.ldl(5, src.wrapping_add(0x2f));
                c.ldr(5, src.wrapping_add(0x28));
                c.ldl(6, src.wrapping_add(0x37));
                c.ldr(6, src.wrapping_add(0x30));
                c.sdl(3, dst.wrapping_add(0x27));
                c.sdr(3, dst.wrapping_add(0x20));
                c.sdl(5, dst.wrapping_add(0x2f));
                c.sdr(5, dst.wrapping_add(0x28));
                c.sdl(6, dst.wrapping_add(0x37));
                c.g[V0][0] = 0xffff_ffff_ffff_ff7f;
                c.sdr(6, dst.wrapping_add(0x30));
                // The copy is not marked kept.
                c.g[V1][0] = bgb_lw(c, A0, BGB_BOLT_FLAGS);
                c.g[V1][0] &= c.g[V0][0];
                bgb_sw_r(c, A0, BGB_BOLT_FLAGS, V1);
                c.g[V1][0] = bgb_lw(c, S3, BGB_BOLT_FLAGS);
                // The +0x14 name is copied only for the flags 0x100..0x800.
                let mut have_name = true;
                c.g[V0][0] = c.g[V1][0] & 0x100;
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = bgb_lw(c, S3, 0x14);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] & 0x200;
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = bgb_lw(c, S3, 0x14);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V1][0] & 0x400;
                        if c.g[V0][0] != 0 {
                            c.g[A1][0] = bgb_lw(c, S3, 0x14);
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[V1][0] & 0x800;
                            if c.g[V0][0] == 0 {
                                bgb_sw_zero(c, A0, 0x14);
                                have_name = false;
                                c.cycles += 1;
                                c.g[V0][0] = 0x38;
                            } else {
                                c.cycles += 1;
                                c.g[A1][0] = bgb_lw(c, S3, 0x14);
                            }
                        }
                    }
                }
                if have_name {
                    c.cycles += 2;
                    let empty = c.g[A1][0] == 0;
                    c.g[V0][0] = 0x38;
                    if !empty {
                        c.cycles += 10;
                        bgb_copy_string_call(c, 0x0014_cecc)?;
                        c.g[V0][0] = 0x38;
                        c.mult(0, V0, S6, V0);
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                        bgb_sw_r(c, V1, 0x14, S2);
                        c.g[A0][0] = bgb_lw(c, S3, 0x14);
                        call_ptr(c, BGB_STRLEN, 0x0014_cee4)?;
                        bgb_addi(c, V0, V0, 1);
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.cycles += 1;
                        c.g[V0][0] = 0x38;
                    }
                }
                c.cycles += 10;
                bgb_addi(c, S1, S1, 1);
                c.mult(0, V0, S6, V0);
                bgb_addi(c, A0, S6, 1);
                bgb_addi(c, V1, S5, 0x18);
                bgb_sw_r(c, SP, 0x34, S1);
                bgb_sw_r(c, SP, 0x30, A0);
                bgb_addi(c, S7, S3, 0x38);
                bgb_addi(c, S0, S3, 0x18);
                c.g[S4][0] = 3;
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                loop {
                    c.cycles += 3;
                    c.g[A1][0] = bgb_lw(c, S0, 0);
                    if c.g[A1][0] == 0 {
                        bgb_addi(c, S1, S1, 4);
                    } else {
                        c.cycles += 8;
                        bgb_copy_string_call(c, 0x0014_cf2c)?;
                        bgb_sw_r(c, S1, 0, S2);
                        c.g[A0][0] = bgb_lw(c, S0, 0);
                        call_ptr(c, BGB_STRLEN, 0x0014_cf38)?;
                        bgb_addi(c, V0, V0, 1);
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        bgb_addi(c, S1, S1, 4);
                    }
                    c.cycles += 3;
                    bgb_addi(c, S4, S4, 0xffff_ffff);
                    let more = (c.g[S4][0] as i64) >= 0;
                    bgb_addi(c, S0, S0, 4);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0014_cf18);
                    }
                }
                // The +0x2c and +0x28 names.
                c.cycles += 3;
                c.g[A1][0] = bgb_lw(c, S3, 0x2c);
                if c.g[A1][0] == 0 {
                    c.g[A1][0] = bgb_lw(c, S3, 0x28);
                } else {
                    c.cycles += 11;
                    bgb_copy_string_call(c, 0x0014_cf64)?;
                    c.g[V0][0] = 0x38;
                    c.mult(0, V0, S6, V0);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    bgb_sw_r(c, V1, 0x2c, S2);
                    c.g[A0][0] = bgb_lw(c, S3, 0x2c);
                    call_ptr(c, BGB_STRLEN, 0x0014_cf7c)?;
                    bgb_addi(c, V0, V0, 1);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = bgb_lw(c, S3, 0x28);
                }
                c.cycles += 2;
                let none28 = c.g[A1][0] == 0;
                c.g[V1][0] = 0x38;
                if !none28 {
                    c.cycles += 11;
                    bgb_copy_string_call(c, 0x0014_cf98)?;
                    c.g[V0][0] = 0x38;
                    c.mult(0, V0, S6, V0);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    bgb_sw_r(c, V1, 0x28, S2);
                    c.g[A0][0] = bgb_lw(c, S3, 0x28);
                    call_ptr(c, BGB_STRLEN, 0x0014_cfb0)?;
                    bgb_addi(c, V0, V0, 1);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = 0x38;
                }
                c.cycles += 4;
                c.mult(0, V1, S6, V1);
                c.g[S6][0] = bgb_lw(c, SP, 0x30);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                bgb_sw_zero(c, V0, 0x30);
            }
            c.cycles += 6;
            c.g[A0][0] = bgb_lw(c, SP, 0);
            c.g[S1][0] = bgb_lw(c, SP, 0x34);
            c.g[V0][0] = bgb_lw(c, A0, 0x1c);
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S3][0] = c.g[S7][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_cde0);
            }
        }
    }
    // Copy the second table behind the bolts.
    c.cycles += 3;
    c.g[A1][0] = bgb_lw(c, SP, 0x8);
    let no_table = (c.g[A1][0] as i64) <= 0;
    c.g[A3][0] = bgb_lw(c, SP, 0);
    if !no_table {
        c.cycles += 5;
        c.g[A0][0] = bgb_lw(c, SP, 0x24);
        c.g[A2][0] = bgb_lw(c, A3, 0x24);
        c.g[A1][0] = bgb_lw(c, A3, 0xc);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
        call_ptr(c, BGB_MEM_COPY, 0x0014_d004)?;
    }
    c.cycles += 24;
    c.g[A0][0] = bgb_lw(c, SP, 0);
    BGB_FREE_DB.call(c, 0x0014_d00c)?;
    Ok(())
}

/// `void Bolts_BuildGameBolts(Bolts* self)` (0x14c7a0): rebuild the bolt
/// database for the game. Pass 1 clears every bolt's 0x80 mark, then for the
/// two rider slots counts the bolts of the character (and its alternate) that
/// qualify; pass 2 and 3 count the bolts of characters 0..9 and 10..29 that
/// qualify; every kept bolt adds its name strings. One block (`0x38 * kept +
/// strings + 8 * table2`) is allocated, the marked bolts and their strings are
/// copied into it, the second table follows, the old block is freed and the
/// database points at the new one.
pub fn bolts_build_game_bolts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0xf0);
        let s = sp(c);
        c.sw(s, c.g[A0][0]);
        frame.save(c, 20, 0xa0);
        frame.save(c, 30, 0x60);
        c.g[S4][0] = 0;
        frame.save(c, 16, 0xe0);
        frame.save(c, 17, 0xd0);
        frame.save(c, 18, 0xc0);
        frame.save(c, 19, 0xb0);
        frame.save(c, 21, 0x90);
        frame.save(c, 22, 0x80);
        frame.save(c, 23, 0x70);
        c.sd(s.wrapping_add(0x50), c.g[RA][0]);
        c.g[FP][0] = 0;
        BGB_GET_INSTANCE.call(c, 0x0014_c7dc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        BGB_GET_INTERFACE.call(c, 0x0014_c7e8)?;
        c.sw(s.wrapping_add(0x4), c.g[V0][0]);
        c.g[V0][0] = bgb_lw(c, SP, 0);
        c.g[V1][0] = bgb_lw(c, SP, 0);
        c.g[V0][0] = bgb_lw(c, V0, 0x24);
        bgb_sw_r(c, SP, 0x8, V0);
        c.g[V0][0] = bgb_lw(c, V1, 0x1c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = 0;
        if !empty {
            // Clear the 0x80 mark of every bolt.
            c.cycles += 2;
            c.g[A3][0] = 0xffff_ffff_ffff_ff7f;
            c.g[A1][0] = 0;
            loop {
                c.cycles += 12;
                c.g[V0][0] = bgb_lw(c, SP, 0);
                bgb_addi(c, A2, A2, 1);
                c.g[A0][0] = bgb_lw(c, V0, 0x4);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = bgb_lw(c, A0, BGB_BOLT_FLAGS);
                c.g[V1][0] &= c.g[A3][0];
                bgb_sw_r(c, A0, BGB_BOLT_FLAGS, V1);
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = bgb_lw(c, V1, 0x1c);
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                bgb_addi(c, A1, A1, 0x38);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_c810);
                }
            }
        }

        c.cycles += 6;
        c.g[A0][0] = bgb_lw(c, SP, 0x8);
        c.g[S3][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
        bgb_sw_r(c, SP, 0x3c, A0);
        c.g[A0][0] = bgb_lw(c, SP, 0x4);

        // Pass 1: the two rider slots (s3 = slot).
        loop {
            c.cycles += 7;
            c.g[A1][0] = c.g[S3][0];
            BGB_GET_ALT_CHARACTER.call(c, 0x0014_c860)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xa));
            c.g[V0][0] = (c.g[V0][0] < 0x14) as u64;
            let alt_ok = c.g[V0][0] != 0;
            c.g[V0][0] = (c.g[S0][0] < 0xa) as u64;
            if !alt_ok {
                c.cycles += 5;
                c.g[A0][0] = bgb_lw(c, SP, 0x4);
                c.g[A1][0] = c.g[S3][0];
                BGB_GET_CHARACTER.call(c, 0x0014_c880)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[S0][0] < 0xa) as u64;
            }
            c.cycles += 2;
            let not_low = c.g[V0][0] == 0;
            c.g[A0][0] = bgb_lw(c, SP, 0);
            if !not_low {
                bgb_pass1_low(c)?;
            } else {
                bgb_pass1_high(c)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[A0][0] = bgb_lw(c, SP, 0x4);
            if event_due(c) {
                deliver_event_here(c, 0x0014_c858);
            }
        }
        bgb_pass2(c)?;
        bgb_pass3(c)?;
        bgb_pass4(c)?;

        c.g[V1][0] = bgb_lw(c, SP, 0x8);
        c.g[V0][0] = bgb_lw(c, SP, 0);
        frame.restore(c, 16, 0xe0);
        bgb_sw_r(c, V0, 0x4, S5);
        bgb_sw_r(c, V0, 0x1c, FP);
        bgb_sw_r(c, V0, 0x24, V1);
        frame.restore(c, 17, 0xd0);
        c.g[A0][0] = bgb_lw(c, SP, 0x28);
        frame.restore(c, 18, 0xc0);
        bgb_sw_r(c, V0, 0x14, A0);
        frame.restore(c, 19, 0xb0);
        c.g[A1][0] = bgb_lw(c, SP, 0x24);
        bgb_sw_r(c, V0, 0, S5);
        frame.restore(c, 20, 0xa0);
        frame.restore(c, 21, 0x90);
        frame.restore(c, 22, 0x80);
        frame.restore(c, 23, 0x70);
        frame.restore(c, 30, 0x60);
        frame.restore_ra(c, 0x50);
        bgb_sw_r(c, V0, 0xc, A1);
        frame.close(c);
        Ok(())
    })
}

/// The two `Bolts` character tables (`Bolts_GetCharTable3` / `4`): entry `i`
/// of the table of `self` (`count[i]` at `self + 4*i + count`, the first
/// entry `index[i]` at `self + 4*i + index`, the table at `self + table`):
/// the address of `table[index[i]]` (8-byte entries), or 0 when `count[i]`
/// is not positive.
pub(crate) fn bolts_get_char_table(c: &mut Cpu, count: u32, index: u32, table: u32) {
    c.cycles += 5;
    c.g[A1][0] = sx(r32(c, A1) << 2);
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(count));
    let positive = (c.g[V1][0] as i64) > 0;
    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    if positive {
        c.cycles += 5;
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(index));
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(table));
        c.g[V0][0] = sx(r32(c, V0) << 3);
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    ret(c);
}

/// `sBoltEntry* Bolts_GetCharTable3(Bolts* self, int i)` (0x14dc10): the
/// table at `+0xc` (counts at `+0x2fc`, indexes at `+0x374`).
pub fn bolts_get_char_table3(c: &mut Cpu) {
    bolts_get_char_table(c, 0x2fc, 0x374, 0xc);
}

/// `sBoltEntry* Bolts_GetCharTable4(Bolts* self, int i)` (0x14dc50): the
/// table at `+0x10` (counts at `+0x3ec`, indexes at `+0x464`).
pub fn bolts_get_char_table4(c: &mut Cpu) {
    bolts_get_char_table(c, 0x3ec, 0x464, 0x10);
}

/// `int Bolts_GetCharTable4Count(BoltsLib* self, int character)`
/// (0x0014dc40): the number of rows of the character's table 4, the word at
/// `self + character * 4 + 0x3ec`.
pub fn bolts_get_char_table4_count(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(BOLTS_CHAR_TABLE4_COUNTS));
    c.pc = ra;
}

/// Offset of the per-character table 4 row counts in the bolts library.
pub(crate) const BOLTS_CHAR_TABLE4_COUNTS: u32 = 0x3ec;

/// `int Bolts_GetCharTable3Count(BoltsLib* self, int character)`
/// (0x0014dc00): the number of rows of the character's table 3, the word at
/// `self + character * 4 + 0x2fc`.
pub fn bolts_get_char_table3_count(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(BOLTS_CHAR_TABLE3_COUNTS));
    c.pc = ra;
}

/// Offset of the per-character table 3 row counts in the bolts library.
pub(crate) const BOLTS_CHAR_TABLE3_COUNTS: u32 = 0x2fc;

/// `void Bolts_ReadFile(BoltsLib* self)`: loads the bolts data file (`BXFILE_sizez`, `new`, `FILE_loadatz` on the path at `gp-0x1f24`) into a new buffer and points the library's tables into it: 0x38-byte entries (n0 at +0x18/+0x1c), then three tables (12-, 8- and 8-byte rows) and the string pool at +0x14. The global flag word at `gp-0x1f08` gets the file's first word (+1 when odd). Every entry's string offsets (name, +0x28/+0x2c/+0x30 and the 4 sub-names) are relocated to pointers into the pool.
/// (`Bolts_ReadFile`, 0x0014be70)
pub fn bolts_read_file(c: &mut Cpu) {
    guest(c, |c| {
        'b_14bf64: {
            c.cycles += 59;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f24));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            call_ptr(c, 0x00317f60, 0x0014be90)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 0x460000;
            c.g[A3][0] = 0u64;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5a00));
            c.g[A2][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0014beac)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f24));
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            call_ptr(c, 0x003e1bb8, 0x0014bec0)?;
            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
            c.g[V0][0] = 0x38;
            c.g[A3][0] = 0xc;
            c.g[T2][0] = c.lw(c.g[A0][0] as u32);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f08), c.g[T2][0]);
            c.g[T0][0] = c.g[T2][0] & 0x1;
            c.g[T1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
            c.mult(0, 2, 9, 2);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[T1][0]);
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.mult(0, 3, 6, 7);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[A2][0]);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A2][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = sx((c.g[A2][0] as u32) << 3);
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x24), c.g[A2][0]);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A2][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = sx((c.g[A2][0] as u32) << 3);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x28), c.g[A2][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let t = c.g[T0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
            if t {
                break 'b_14bf64;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f08), c.g[V0][0]);
                break 'b_14bf64;
            }
        }
        'b_14c050: {
            c.cycles += 2;
            let t = (c.g[T1][0] as i64) <= 0;
            c.g[A2][0] = 0u64;
            if t {
                break 'b_14c050;
            } else {
                c.cycles += 3;
                c.g[T1][0] = 0x38;
                c.g[T0][0] = 0x38;
                c.mult(0, 5, 6, 9);
                'l_14bf78: loop {
                    'b_14bfa0: {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                        let t = c.g[V1][0] == 0u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_14bfa0;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            break 'b_14bfa0;
                        }
                    }
                    'b_14bfb8: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
                        let t = c.g[V1][0] == 0u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_14bfb8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                            break 'b_14bfb8;
                        }
                    }
                    'b_14bfe0: {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V1][0] == 0u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_14bfe0;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            break 'b_14bfe0;
                        }
                    }
                    'b_14bff8: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x30));
                        let t = c.g[V1][0] == 0u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_14bff8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                            break 'b_14bff8;
                        }
                    }
                    c.cycles += 4;
                    c.mult(0, 7, 6, 8);
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    'l_14c008: loop {
                        'b_14c030: {
                            c.cycles += 7;
                            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                            let t = c.g[V1][0] == 0u64;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                            if t {
                                break 'b_14c030;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                                break 'b_14c030;
                            }
                        }
                        c.cycles += 4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 4i64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            if event_due(c) {
                                deliver_event_here(c, 0x0014c008);
                            }
                            continue 'l_14c008;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.mult(0, 5, 6, 9);
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014bf78);
                                }
                                continue 'l_14bf78;
                            } else {
                                break 'b_14c050;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void Bolts_PrepareForGame(BoltsLib* self)`: for each of the 3 profiles and 10 characters marks the profile's bolt entries (flag 0x10) that are unlocked (flag 4) or required by a bolt rule, then rebuilds the DB and the character indexes.
/// (`Bolts_PrepareForGame`, 0x0014d068)
pub fn bolts_prepare_for_game(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V1][0] = 0u64;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = c.g[A0][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S5][0] = 0x4a0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S6][0] = 0x9b50;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.mult(0, 19, 3, 22);
        'l_14d0a0: loop {
            c.cycles += 2;
            c.g[S0][0] = 0u64;
            c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            'l_14d0a8: loop {
                'b_14d110: {
                    c.cycles += 17;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0014dc10,
                        f: ee::s01::Bolts_GetCharTable3,
                    }
                    .call(c, 0x0014d0b4)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[S1][0] = c.g[V0][0];
                    Callee {
                        addr: 0x0014dc00,
                        f: ee::s01::Bolts_GetCharTable3Count,
                    }
                    .call(c, 0x0014d0c4)?;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x6ca8));
                    c.g[V0][0] = 0xf88;
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.mult(0, 2, 16, 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28c));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x290));
                    if t {
                        break 'b_14d110;
                    } else {
                        c.cycles += 1;
                        c.g[A2][0] = c.g[V0][0];
                        'l_14d0f0: loop {
                            'b_14d104: {
                                c.cycles += 4;
                                c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = c.g[V1][0] & 0x4;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.g[V1][0] | 0x10;
                                if t {
                                    break 'b_14d104;
                                } else {
                                    c.cycles += 1;
                                    c.sh((c.g[A0][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                                    break 'b_14d104;
                                }
                            }
                            c.cycles += 3;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                            let t = c.g[A2][0] != 0u64;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014d0f0);
                                }
                                continue 'l_14d0f0;
                            } else {
                                break 'b_14d110;
                            }
                        }
                    }
                }
                'b_14d1a8: {
                    c.cycles += 2;
                    if (c.g[A3][0] as i64) <= 0 {
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x288));
                        break 'b_14d1a8;
                    } else {
                        c.cycles += 2;
                        c.g[A2][0] = c.g[A3][0];
                        'l_14d120: loop {
                            'b_14d14c: {
                                'b_14d148: {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x288));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                    let t = (c.g[V0][0] as i64) < 0;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    if t {
                                        break 'b_14d148;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                        let t = true;
                                        c.g[A0][0] =
                                            sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        if t {
                                            break 'b_14d14c;
                                        } else {
                                            break 'b_14d148;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[A0][0] = 0u64;
                                break 'b_14d14c;
                            }
                            'b_14d19c: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = c.g[V0][0] & 0x4;
                                if c.g[V0][0] == 0u64 {
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                    break 'b_14d19c;
                                } else {
                                    'b_14d198: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x6));
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        if t {
                                            break 'b_14d198;
                                        } else {
                                            'b_14d18c: {
                                                'b_14d188: {
                                                    c.cycles += 5;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[A1][0] as u32).wrapping_add(0x288)
                                                        );
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[V1][0] as u32));
                                                    c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                                    let t = (c.g[V0][0] as i64) < 0;
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    if t {
                                                        break 'b_14d188;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x290));
                                                        let t = true;
                                                        c.g[V1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(c.g[V0][0] as u32));
                                                        if t {
                                                            break 'b_14d18c;
                                                        } else {
                                                            break 'b_14d188;
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[V1][0] = 0u64;
                                                break 'b_14d18c;
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                                            c.g[V0][0] = c.g[V0][0] | 0x10;
                                            c.sh((c.g[V1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                                            break 'b_14d198;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                    break 'b_14d19c;
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[A2][0] != 0u64;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014d120);
                                }
                                continue 'l_14d120;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x288));
                                break 'b_14d1a8;
                            }
                        }
                    }
                }
                'b_14d1c4: {
                    'b_14d1c0: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                        let t = (c.g[V0][0] as i64) < 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if t {
                            break 'b_14d1c0;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                            let t = true;
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            if t {
                                break 'b_14d1c4;
                            } else {
                                break 'b_14d1c0;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = 0u64;
                    break 'b_14d1c4;
                }
                c.cycles += 6;
                c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x2));
                c.g[S0][0] = c.g[T0][0];
                c.g[V1][0] = ((c.g[S0][0] as i64) < 10i64) as u64;
                c.g[V0][0] = c.g[V0][0] | 0x10;
                let t = c.g[V1][0] != 0u64;
                c.sh((c.g[A0][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014d0a8);
                    }
                    continue 'l_14d0a8;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[S4][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.mult(0, 19, 3, 22);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014d0a0);
                        }
                        continue 'l_14d0a0;
                    } else {
                        c.cycles += 20;
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0014c7a0,
                            f: ee::s01::Bolts_BuildGameBolts,
                        }
                        .call(c, 0x0014d1f4)?;
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0014c2b0,
                            f: ee::s01::Bolts_IndexBoltsByChar,
                        }
                        .call(c, 0x0014d1fc)?;
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0014c320,
                            f: ee::s01::Bolts_IndexTable2ByChar,
                        }
                        .call(c, 0x0014d204)?;
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0014c3c8,
                            f: ee::s01::Bolts_IndexTable3ByChar,
                        }
                        .call(c, 0x0014d20c)?;
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0014c488,
                            f: ee::s01::Bolts_IndexTable4ByChar,
                        }
                        .call(c, 0x0014d214)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `void BEBoltInt_RestoreAllEquipFlags(void)` (0x14af10): for each of the 3
/// profiles (0x9b50 bytes apart from 0x4a6ca8) and 10 characters (0xf88
/// each), every flag entry (count at `+0x28c`, 4-byte entries from `+0x290`,
/// flags halfword at `+2`) gets bit 0x10 set when bit 4 is set, else
/// cleared. The loop counters live in a0 (profile), v1 (character), a2 (entry).
pub fn be_bolt_int_restore_all_equip_flags_2(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x4a_0000;
    c.g[A0][0] = 0;
    c.g[T4][0] = sx(r32(c, V0).wrapping_add(0x6ca8));
    c.g[T5][0] = 0x9b50;
    c.g[T3][0] = 0xf88;
    c.mult(0, 9, 4, 13);
    'profiles: loop {
        c.cycles += 4;
        c.g[V1][0] = 0;
        c.g[T2][0] = sx(r32(c, A0).wrapping_add(1));
        c.mult(0, 4, 3, 11);
        'characters: loop {
            c.cycles += 6;
            c.g[A3][0] = sx(r32(c, V1).wrapping_add(1));
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, T4)));
            c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, V0)));
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x28c)).lw(c);
            let no_flags = (c.g[V1][0] as i64) <= 0;
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x290));
            if !no_flags {
                c.cycles += 2;
                c.g[A2][0] = c.g[V1][0];
                c.g[T0][0] = 0xffff_ffff_ffff_ffef;
                'entries: loop {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(2)).lhu(c);
                    c.g[A1][0] = c.g[V0][0] | 0x10;
                    c.g[V1][0] = c.g[V0][0] & c.g[T0][0];
                    c.g[V0][0] &= 4;
                    let clear = c.g[V0][0] == 0;
                    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
                    if clear {
                        c.cycles += 1;
                        Addr(r32(c, A0).wrapping_add(2)).sh(c, c.g[V1][0]);
                    } else {
                        c.cycles += 2;
                        Addr(r32(c, A0).wrapping_add(2)).sh(c, c.g[A1][0]);
                    }
                    c.cycles += 2;
                    let more = c.g[A2][0] != 0;
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014_af58);
                        }
                        continue 'entries;
                    }
                    break 'entries;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[A3][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10) as u64;
            let more_characters = c.g[V0][0] != 0;
            c.mult(0, 4, 3, 11);
            if more_characters {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_af38);
                }
                continue 'characters;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[T2][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
            let more_profiles = c.g[V0][0] != 0;
            c.mult(0, 9, 4, 13);
            if more_profiles {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_af28);
                }
                continue 'profiles;
            }
            c.cycles += 2;
            ret(c);
            return;
        }
    }
}

/// `void Bolts_PrepareGameDB(void)` (0x14bd98): `Bolts_PrepareForGame(0x4a6750)`.
pub fn bolts_prepare_game_db(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x4a_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6750));
        Callee {
            addr: 0x0014_d068,
            f: ee::s01::Bolts_PrepareForGame,
        }
        .call(c, 0x0014_bdac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Bolts_LoadGlobalDB(void)` (0x14bd78): `Bolts_LoadDB(0x4a6750)`.
pub fn bolts_load_global_db(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x4a_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6750));
        Callee {
            addr: 0x0014_c6a0,
            f: ee::s01::Bolts_LoadDB,
        }
        .call(c, 0x0014_bd8c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The interface getters the load calls, and their generated shard functions.
pub(crate) const DESER_RACE_GET: Callee = Callee {
    addr: 0x0014_48d8,
    f: ee::s01::f_001448d8,
};

pub(crate) const DESER_PLAYER_GET: Callee = Callee {
    addr: 0x0014_5970,
    f: ee::s01::f_00145970,
};

pub(crate) const DESER_BAGT_GET: Callee = Callee {
    addr: 0x0014_f900,
    f: ee::s01::f_0014f900,
};

pub(crate) const DESER_MISSION_GET: Callee = Callee {
    addr: 0x0015_32a0,
    f: ee::s01::f_001532a0,
};

pub(crate) const DESER_SCORE_GET: Callee = Callee {
    addr: 0x0015_4948,
    f: ee::s01::f_00154948,
};

pub(crate) const DESER_CHAR_GET: Callee = Callee {
    addr: 0x0014_7f78,
    f: ee::s01::f_00147f78,
};

/// One block-copy loop of the generated body. Each step moves 0x20 bytes
/// (four doublewords) from `src` to `dst` through the `temps` registers; the
/// loop runs while `src != end`, with an event check at its back edge (`head`).
/// `per_step` is the cycle charge of one step; `unaligned` picks `ldl`/`ldr`
/// (the source or destination is not 8-aligned) over `ld`/`sd`.
#[derive(Clone, Copy)]
pub(crate) struct DeserCopy {
    head: u32,
    src: usize,
    dst: usize,
    end: usize,
    temps: [usize; 4],
    per_step: u64,
    unaligned: bool,
}

/// The 0x40-byte head of the race block (`cBELibrary_raceDB`), `V1` -> `A2`.
pub(crate) const DESER_RACE_ALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a69c,
    src: V1,
    dst: A2,
    end: V0,
    temps: [T1, T2, A0, A1],
    per_step: 11,
    unaligned: false,
};

pub(crate) const DESER_RACE_UNALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a630,
    src: V1,
    dst: A2,
    end: V0,
    temps: [A0, A1, A3, T0],
    per_step: 19,
    unaligned: true,
};

/// The 0xa0-byte body of the player block (`cBELibrary_playerDB`).
pub(crate) const DESER_PLAYER_ALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a77c,
    src: V1,
    dst: A2,
    end: V0,
    temps: [T1, T2, A0, A1],
    per_step: 11,
    unaligned: false,
};

pub(crate) const DESER_PLAYER_UNALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a720,
    src: V1,
    dst: A2,
    end: V0,
    temps: [A0, A1, A3, T0],
    per_step: 19,
    unaligned: true,
};

/// The 0xf80-byte body of the first profile block, `A2` -> `V1`.
pub(crate) const DESER_PROFILE0_ALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a860,
    src: A2,
    dst: V1,
    end: V0,
    temps: [T2, A0, A1, T0],
    per_step: 11,
    unaligned: false,
};

pub(crate) const DESER_PROFILE0_UNALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a804,
    src: A2,
    dst: V1,
    end: V0,
    temps: [A0, A1, T0, T1],
    per_step: 19,
    unaligned: true,
};

/// The 0xf80-byte body of the second profile block (`D_004B07F8`).
pub(crate) const DESER_PROFILE1_ALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a934,
    src: A2,
    dst: V1,
    end: V0,
    temps: [T0, T1, T2, A0],
    per_step: 11,
    unaligned: false,
};

pub(crate) const DESER_PROFILE1_UNALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a8d8,
    src: A2,
    dst: V1,
    end: V0,
    temps: [T1, T2, A0, A1],
    per_step: 19,
    unaligned: true,
};

/// The 0x9b40-byte body of the rules block (`D_004BA348`), `V1` -> `A1`.
pub(crate) const DESER_RULES_ALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a9e8,
    src: V1,
    dst: A1,
    end: V0,
    temps: [T2, A0, A2, T0],
    per_step: 11,
    unaligned: false,
};

pub(crate) const DESER_RULES_UNALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a98c,
    src: V1,
    dst: A1,
    end: V0,
    temps: [A0, A2, T0, T1],
    per_step: 19,
    unaligned: true,
};

/// One 0x20-byte step with unaligned accesses: four `ldl`/`ldr` pairs into
/// `t`, then four `sdl`/`sdr` pairs, in the generated order.
pub(crate) fn deser_step_unaligned(c: &mut Cpu, from: u32, to: u32, t: [usize; 4]) {
    for (i, &r) in t.iter().enumerate() {
        let o = 8 * i as u32;
        c.ldl(r, from.wrapping_add(o + 7));
        c.ldr(r, from.wrapping_add(o));
    }
    for (i, &r) in t.iter().enumerate() {
        let o = 8 * i as u32;
        c.sdl(r, to.wrapping_add(o + 7));
        c.sdr(r, to.wrapping_add(o));
    }
}

/// One 0x20-byte step with aligned doublewords: four `ld` into `t`, then four
/// `sd`.
pub(crate) fn deser_step_aligned(c: &mut Cpu, from: u32, to: u32, t: [usize; 4]) {
    for (i, &r) in t.iter().enumerate() {
        c.g[r][0] = c.ld(from.wrapping_add(8 * i as u32));
    }
    for (i, &r) in t.iter().enumerate() {
        let v = c.g[r][0];
        c.sd(to.wrapping_add(8 * i as u32), v);
    }
}

/// Runs one block-copy loop to its exit (the caller charges the exit cycles).
pub(crate) fn deser_run_copy(c: &mut Cpu, k: DeserCopy) {
    loop {
        c.cycles += k.per_step;
        let from = c.g[k.src][0] as u32;
        let to = c.g[k.dst][0] as u32;
        if k.unaligned {
            deser_step_unaligned(c, from, to, k.temps);
        } else {
            deser_step_aligned(c, from, to, k.temps);
        }
        c.g[k.src][0] = sx(from.wrapping_add(0x20));
        let more = c.g[k.src][0] != c.g[k.end][0];
        c.g[k.dst][0] = sx(to.wrapping_add(0x20));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, k.head);
        }
    }
}

/// `obj->onLoad()` with the interface object in `v0` (the race and player
/// interfaces): `v1 = vtable`, `a0 = this delta`, `a1 = method`, then the call
/// with `a0 = obj + delta`.
pub(crate) fn deser_on_load_v0(c: &mut Cpu, ret_addr: u32) -> Flow {
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
    let target = c.g[A1][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_addr)
}

/// The same call with the interface object copied to `a1` first (the BAGT,
/// mission, score and character interfaces): `v0 = vtable`, `v1 = method`.
pub(crate) fn deser_on_load_a1(c: &mut Cpu, ret_addr: u32) -> Flow {
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_addr)
}

/// `BELibrary_DeserializeSave` (0x14a5e0), the body of `cBELibrary_readFromData`:
/// loads the save's game data at `data` (`a0`). Layout: the race block (0x4c
/// bytes) into `cBELibrary_raceDB`, then the race interface's `onLoad`; the
/// player block (0xa8) into `cBELibrary_playerDB`, then the player interface's
/// `onLoad`; one 0xf88-byte profile block per player into
/// `cBELibrary_profileLib[cCharacterID]` (the second one only when its id is
/// not negative, into `D_004B07F8`); the rules block (0x9b50 bytes, then 0x20
/// bytes of rule options) into `D_004BA348` and `cBERules_mRules`; then
/// `onLoad` on the BAGT, mission, score and character interfaces. Every block
/// copy is an aligned `ld`/`sd` loop or an unaligned `ldl`/`ldr` one, chosen
/// by `(src | dst) & 7`, as the generated body does.
pub fn be_library_deserialize_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[V0][0] = 0x530000;
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b0));
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = c.g[A1][0] | c.g[A2][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        c.g[V0][0] &= 0x7;
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        let aligned = c.g[V0][0] == 0;
        frame.save_ra(c, 0);

        // Race block: 0x40 bytes by the loop, then 12 more.
        c.cycles += 6;
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4c));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = 0x530000;
        c.g[S2][0] = 0x4a0000;
        c.g[S3][0] = 0x4c0000;
        c.g[S4][0] = 0x530000;
        if aligned {
            deser_run_copy(c, DESER_RACE_ALIGNED);
        } else {
            deser_run_copy(c, DESER_RACE_UNALIGNED);
            c.cycles += 2;
        }
        c.cycles += 20;
        let (src, dst) = (c.g[V1][0] as u32, c.g[A2][0] as u32);
        c.ldl(7, src.wrapping_add(0x7));
        c.ldr(7, src);
        c.lwl(8, src.wrapping_add(0xb));
        c.lwr(8, src.wrapping_add(0x8));
        c.sdl(7, dst.wrapping_add(0x7));
        c.sdr(7, dst);
        c.swl(8, dst.wrapping_add(0xb));
        c.swr(8, dst.wrapping_add(0x8));
        DESER_RACE_GET.call(c, 0x0014_a6ec)?;
        deser_on_load_v0(c, 0x0014_a700)?;

        // Player block: 0xa8 bytes at data + 0x4c; 0xa0 by the loop, 8 more.
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5b20));
        c.g[V0][0] = c.g[A1][0] | c.g[A2][0];
        c.g[V0][0] &= 0x7;
        let aligned = c.g[V0][0] == 0;
        c.g[V1][0] = c.g[A1][0];
        c.cycles += 2;
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa8));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa0));
        if aligned {
            deser_run_copy(c, DESER_PLAYER_ALIGNED);
        } else {
            deser_run_copy(c, DESER_PLAYER_UNALIGNED);
            c.cycles += 2;
        }
        c.cycles += 21;
        let (src, dst) = (c.g[V1][0] as u32, c.g[A2][0] as u32);
        c.ldl(7, src.wrapping_add(0x7));
        c.ldr(7, src);
        c.sdl(7, dst.wrapping_add(0x7));
        c.sdr(7, dst);
        DESER_PLAYER_GET.call(c, 0x0014_a7bc)?;
        deser_on_load_v0(c, 0x0014_a7d0)?;

        // First profile block: the slot of playerDB[0].cCharacterID.
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5b20));
        c.g[V1][0] = 0xf88;
        c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x11));
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6ca8));
        c.g[A1][0] = c.g[S0][0];
        c.mult(0, 3, 2, 3);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = c.g[A1][0] | c.g[V1][0];
        c.g[V0][0] &= 0x7;
        let aligned = c.g[V0][0] == 0;
        c.g[A2][0] = c.g[A1][0];
        c.cycles += 2;
        c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf88));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf80));
        if aligned {
            deser_run_copy(c, DESER_PROFILE0_ALIGNED);
            c.cycles += 1;
        } else {
            deser_run_copy(c, DESER_PROFILE0_UNALIGNED);
            c.cycles += 2;
        }
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5b20));

        // The first profile block's last 8 bytes, then the second profile
        // block when playerDB[1].cCharacterID is not negative.
        c.cycles += 7;
        c.g[A1][0] = c.g[A3][0];
        c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x2d));
        let (src, dst) = (c.g[A2][0] as u32, c.g[V1][0] as u32);
        c.ldl(9, src.wrapping_add(0x7));
        c.ldr(9, src);
        c.sdl(9, dst.wrapping_add(0x7));
        let no_second = (c.g[V0][0] as i64) < 0;
        c.sdr(9, dst);
        if !no_second {
            c.cycles += 9;
            c.g[A0][0] = 0xf88;
            c.g[V1][0] = 0x4b0000;
            c.mult(0, 4, 2, 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7f8));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = c.g[A1][0] | c.g[V1][0];
            c.g[V0][0] &= 0x7;
            let aligned = c.g[V0][0] == 0;
            c.g[A2][0] = c.g[A1][0];
            c.cycles += 2;
            c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf88));
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf80));
            if aligned {
                deser_run_copy(c, DESER_PROFILE1_ALIGNED);
            } else {
                deser_run_copy(c, DESER_PROFILE1_UNALIGNED);
                c.cycles += 2;
            }
            c.cycles += 4;
            let (src, dst) = (c.g[A2][0] as u32, c.g[V1][0] as u32);
            c.ldl(5, src.wrapping_add(0x7));
            c.ldr(5, src);
            c.sdl(5, dst.wrapping_add(0x7));
            c.sdr(5, dst);
        }

        // Rules block: 0x9b40 bytes by the loop, 16 more, then 32 bytes of
        // rule options from the end of the block.
        c.cycles += 5;
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5cb8));
        c.g[V0][0] = c.g[A3][0] | c.g[A1][0];
        c.g[V0][0] &= 0x7;
        let aligned = c.g[V0][0] == 0;
        c.g[V1][0] = c.g[A3][0];
        c.cycles += 2;
        c.g[V0][0] = 0x9b40;
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if aligned {
            deser_run_copy(c, DESER_RULES_ALIGNED);
        } else {
            deser_run_copy(c, DESER_RULES_UNALIGNED);
            c.cycles += 2;
        }
        c.cycles += 67;
        let (src, dst) = (c.g[V1][0] as u32, c.g[A1][0] as u32);
        c.ldl(9, src.wrapping_add(0x7));
        c.ldr(9, src);
        c.ldl(10, src.wrapping_add(0xf));
        c.ldr(10, src.wrapping_add(0x8));
        c.sdl(9, dst.wrapping_add(0x7));
        c.sdr(9, dst);
        c.sdl(10, dst.wrapping_add(0xf));
        c.g[V0][0] = 0x9b50;
        c.sdr(10, dst.wrapping_add(0x8));
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[T2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x8b8));
        let (src, dst) = (c.g[A1][0] as u32, c.g[T2][0] as u32);
        c.ldl(7, src.wrapping_add(0x7));
        c.ldr(7, src);
        c.ldl(8, src.wrapping_add(0xf));
        c.ldr(8, src.wrapping_add(0x8));
        c.ldl(9, src.wrapping_add(0x17));
        c.ldr(9, src.wrapping_add(0x10));
        c.sdl(7, dst.wrapping_add(0x7));
        c.sdr(7, dst);
        c.sdl(8, dst.wrapping_add(0xf));
        c.sdr(8, dst.wrapping_add(0x8));
        c.sdl(9, dst.wrapping_add(0x17));
        c.sdr(9, dst.wrapping_add(0x10));
        c.ldl(7, src.wrapping_add(0x1f));
        c.ldr(7, src.wrapping_add(0x18));
        c.sdl(7, dst.wrapping_add(0x1f));
        c.sdr(7, dst.wrapping_add(0x18));

        // The interfaces' onLoad, in the generated order.
        DESER_BAGT_GET.call(c, 0x0014_aa88)?;
        deser_on_load_a1(c, 0x0014_aaa0)?;
        DESER_MISSION_GET.call(c, 0x0014_aaa8)?;
        deser_on_load_a1(c, 0x0014_aac0)?;
        DESER_SCORE_GET.call(c, 0x0014_aac8)?;
        deser_on_load_a1(c, 0x0014_aae0)?;
        DESER_CHAR_GET.call(c, 0x0014_aae8)?;
        deser_on_load_a1(c, 0x0014_ab00)?;

        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BE_GetCharProfileData` (0x14ad28), called by `BE_InitRiderCharProfileBoltMap`.
pub(crate) const INIT_BOLT_MAP_PROFILE_DATA: Callee = Callee {
    addr: 0x0014_ad28,
    f: ssx3_ee::s01::f_0014ad28,
};

/// `BE_InitCharProfileBoltMap` (0x151a88).
pub(crate) const INIT_BOLT_MAP_INIT: Callee = Callee {
    addr: 0x0015_1a88,
    f: ssx3_ee::s01::f_00151a88,
};

/// `void BE_InitRiderCharProfileBoltMap(a0, a1, int char)` (0x14acf8): get
/// the character's profile data (the caller's arguments pass through) and
/// initialise its bolt map with the character in `s0`.
pub fn be_init_rider_char_profile_bolt_map(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        INIT_BOLT_MAP_PROFILE_DATA.call(c, 0x0014_ad0c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        INIT_BOLT_MAP_INIT.call(c, 0x0014_ad18)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The bolt database (`Bolts_GetDB`), the profile library (the character
/// records of every rider profile), and their strides.
pub(crate) const BOLT_DB_BASE: u32 = 0x004a_6750;

pub(crate) const BOLT_PROFILE_LIB: u32 = 0x004a_6ca8;

/// Bytes per profile and per character record.
pub(crate) const BOLT_PROFILE_STRIDE: u32 = 0x9b50;

pub(crate) const BOLT_CHARACTER_STRIDE: u32 = 0x0f88;

/// Character record fields: the bolt lookup pointer, the bolt table.
pub(crate) const BOLT_LOOKUP_PTR: u32 = 0x288;

pub(crate) const BOLT_TABLE: u32 = 0x290;

/// The 525-entry bolt table, 4 bytes each.
pub(crate) const BOLT_TABLE_BYTES: u32 = 0x834;

/// `BEBoltInt_GetRiderProfileAndChar` (0x14ac70).
pub(crate) const BOLT_CALL_RIDER_PROFILE_AND_CHAR: Callee = Callee {
    addr: 0x0014_ac70,
    f: ee::s01::BEBoltInt_GetRiderProfileAndChar,
};

/// `BEBoltInt_GetBoltState` (0x14ada8).
pub(crate) const BOLT_CALL_GET_BOLT_STATE: Callee = Callee {
    addr: 0x0014_ada8,
    f: ee::s01::BEBoltInt_GetBoltState,
};

/// `BE_SetCharProfileBoltEquipped` (0x151c90).
pub(crate) const BOLT_CALL_SET_CHAR_EQUIPPED: Callee = Callee {
    addr: 0x0015_1c90,
    f: ee::s01::BE_SetCharProfileBoltEquipped,
};

/// `BE_EnsureDefaultBoltsEquipped` (0x1521f0).
pub(crate) const BOLT_CALL_ENSURE_DEFAULTS: Callee = Callee {
    addr: 0x0015_21f0,
    f: ee::s01::BE_EnsureDefaultBoltsEquipped,
};

/// `BE_ValidateExclusiveBolts` (0x1520e8).
pub(crate) const BOLT_CALL_VALIDATE_EXCLUSIVE: Callee = Callee {
    addr: 0x0015_20e8,
    f: ee::s01::BE_ValidateExclusiveBolts,
};

/// `BE_TestCharProfileBoltFlags` (0x152368).
pub(crate) const BOLT_CALL_TEST_FLAGS: Callee = Callee {
    addr: 0x0015_2368,
    f: ee::s01::BE_TestCharProfileBoltFlags,
};

/// `Bolts_GetCharBoltCount(db, kind)` (0x14d988).
pub(crate) const BOLT_CALL_CHAR_BOLT_COUNT: Callee = Callee {
    addr: 0x0014_d988,
    f: ee::s01::Bolts_GetCharBoltCount,
};

/// `Bolts_GetCharBolts(db, kind)` (0x14d998).
pub(crate) const BOLT_CALL_CHAR_BOLTS: Callee = Callee {
    addr: 0x0014_d998,
    f: ee::s01::Bolts_GetCharBolts,
};

/// `BEBoltInt_GetRiderProfile` (0x14ac50).
pub(crate) const BOLT_CALL_RIDER_PROFILE: Callee = Callee {
    addr: 0x0014_ac50,
    f: ee::s01::BEBoltInt_GetRiderProfile,
};

/// `BEBoltInt_GetRiderCharacter` (0x14ac30).
pub(crate) const BOLT_CALL_RIDER_CHARACTER: Callee = Callee {
    addr: 0x0014_ac30,
    f: ee::s01::BEBoltInt_GetRiderCharacter,
};

/// `BEBoltInt_CollectEquippedTextureNames` (0x14b7a8).
pub(crate) const BOLT_CALL_COLLECT_TEXTURES: Callee = Callee {
    addr: 0x0014_b7a8,
    f: ee::s01::BEBoltInt_CollectEquippedTextureNames,
};

/// `BEBoltInt_GetBoltTextureName` (0x14b988).
pub(crate) const BOLT_CALL_TEXTURE_NAME: Callee = Callee {
    addr: 0x0014_b988,
    f: ee::s01::BEBoltInt_GetBoltTextureName,
};

/// `memcpy` (0x3e6574), reached through the function table.
pub(crate) const BOLT_MEMCPY: u32 = 0x003e_6574;

/// `BEBoltInt_GetBoltState(self, profile, character, slot)` (0x14ada8): the
/// address of a slot's word in a character record,
/// `profile lib + profile * 0x9b50 + character * 0xf88 + 0x290 + slot * 4`.
/// `self` is unused.
pub fn bolt_int_get_bolt_state(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x9b50;
    c.g[A0][0] = 0xf88;
    let profile = c.g[A1][0] as u32;
    c.g[A1][0] = sx(abi::mult(c, profile, BOLT_PROFILE_STRIDE));
    let character = c.g[A2][0] as u32;
    c.g[A2][0] = sx(abi::mult(c, character, BOLT_CHARACTER_STRIDE));
    c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
    c.g[V1][0] = sx(BOLT_PROFILE_LIB);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(BOLT_TABLE));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `BEBoltInt_GetRiderBoltState(self, rider, slot)` (0x14add8): the bolt
/// state address of a rider's current profile and character.
pub fn bolt_int_get_rider_bolt_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A2][0] = c.g[SP][0];
        Addr(sp(c)).sw(c, 0);
        BOLT_CALL_RIDER_PROFILE_AND_CHAR.call(c, 0x0014_adfc)?;
        c.g[A2][0] = Addr(sp(c)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        BOLT_CALL_GET_BOLT_STATE.call(c, 0x0014_ae10)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `BEBoltInt_SetBoltEquipped(self, profile, character, slot, on)`
/// (0x14afb0): set or clear the equipped flag (0x10) of a bolt in a
/// character record, validate the bolt set, and restore the saved bolt table
/// when the change is refused. Returns 1 when the change stuck.
pub fn bolt_int_set_bolt_equipped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[V0][0] = 0xf88;
        c.g[V1][0] = 0x9b50;
        let character = c.g[A2][0] as u32;
        c.g[A2][0] = sx(abi::mult(c, character, BOLT_CHARACTER_STRIDE));
        let profile = c.g[A1][0] as u32;
        c.g[A1][0] = sx(abi::mult(c, profile, BOLT_PROFILE_STRIDE));
        let frame = Frame::open(c, 0x8a0);
        c.g[V0][0] = sx(0x004a_0000u32.wrapping_add(0x6ca8));
        frame.save(c, S1, 0x880);
        frame.save(c, S0, 0x890);
        c.g[S1][0] = c.g[A3][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save(c, S2, 0x870);
        frame.save(c, S3, 0x860);
        c.g[S2][0] = c.g[T0][0];
        frame.save_ra(c, 0x850);
        c.g[V1][0] = c.g[SP][0];
        c.g[A3][0] = 0x20c;
        c.g[A0][0] = u64::MAX;
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = u64::MAX;
        // The local table of 525 (id, flags) pairs starts as all unused.
        loop {
            c.cycles += 7;
            let entry = c.g[V1][0] as u32;
            let unused_id = c.g[A0][0];
            Addr(entry).sh(c, unused_id);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
            Addr(entry.wrapping_add(2)).sh(c, 0);
            let more = c.g[A3][0] != c.g[V0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_b000);
            }
        }
        // Save the character's bolt table, apply the change, validate.
        let applied: bool = 'applied: {
            c.cycles += 11;
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(BOLT_TABLE));
            c.g[A0][0] = c.g[SP][0];
            c.g[S3][0] = c.g[A1][0];
            c.g[A2][0] = BOLT_TABLE_BYTES as u64;
            call_ptr(c, BOLT_MEMCPY, 0x0014_b030)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            BOLT_CALL_SET_CHAR_EQUIPPED.call(c, 0x0014_b040)?;
            let set_ok = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 1);
            if !set_ok {
                break 'applied false;
            }
            // The table slot of the bolt: the lookup gives its index (or -1).
            c.cycles += 5;
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(BOLT_LOOKUP_PTR)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lh(c);
            let unknown = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if unknown {
                c.cycles += 1;
                c.g[A1][0] = 0;
            } else {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(BOLT_TABLE));
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            }
            // Set or clear the equipped flag (0x10) of the entry.
            c.cycles += 2;
            let clear = c.g[S2][0] == 0;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(2)).lhu(c);
            if clear {
                c.cycles += 1;
                c.g[V0][0] &= 0xffef;
            } else {
                c.cycles += 2;
                c.g[V0][0] |= 0x10;
            }
            c.cycles += 5;
            let flags = c.g[V0][0];
            Addr((c.g[A1][0] as u32).wrapping_add(2)).sh(c, flags);
            c.g[A0][0] = c.g[S0][0];
            BOLT_CALL_ENSURE_DEFAULTS.call(c, 0x0014_b08c)?;
            let ensured = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !ensured {
                break 'applied false;
            }
            c.cycles += 5;
            Addr(sp(c).wrapping_add(0x840)).sw(c, 0);
            c.g[A1][0] = sx(sp(c).wrapping_add(0x840));
            BOLT_CALL_VALIDATE_EXCLUSIVE.call(c, 0x0014_b0a0)?;
            let validated = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(sp(c).wrapping_add(0x840)).lw(c);
            if !validated {
                break 'applied false;
            }
            c.cycles += 2;
            let no_count = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if no_count {
                break 'applied true;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            BOLT_CALL_ENSURE_DEFAULTS.call(c, 0x0014_b0b8)?;
            let ensured_again = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            ensured_again
        };
        if applied {
            c.g[V0][0] = 1;
        } else {
            // Refused: put the saved bolt table back and report 0.
            c.cycles += 5;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = BOLT_TABLE_BYTES as u64;
            call_ptr(c, BOLT_MEMCPY, 0x0014_b0d0)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x880);
        frame.restore(c, S0, 0x890);
        frame.restore(c, S2, 0x870);
        frame.restore(c, S3, 0x860);
        frame.restore_ra(c, 0x850);
        frame.close(c);
        Ok(())
    });
}

/// `BEBoltInt_CharRecordCall52368(self, rider, a2, a3)` (0x14b1b0): the rider's
/// character record, tested for bolt flags (`BE_TestCharProfileBoltFlags`).
pub fn bolt_int_char_record_call_52368(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A3][0];
        c.g[A2][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        Addr(sp(c)).sw(c, 0);
        BOLT_CALL_RIDER_PROFILE_AND_CHAR.call(c, 0x0014_b1d4)?;
        c.g[A0][0] = Addr(sp(c)).lw(c);
        c.g[A1][0] = 0xf88;
        c.g[A2][0] = 0x9b50;
        c.g[V1][0] = 0x004a_0000;
        let character = c.g[A0][0] as u32;
        c.g[A0][0] = sx(abi::mult(c, character, 0xf88));
        let profile = c.g[V0][0] as u32;
        c.g[V0][0] = sx(abi::mult(c, profile, BOLT_PROFILE_STRIDE));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        BOLT_CALL_TEST_FLAGS.call(c, 0x0014_b204)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `BEBoltInt_CountAvailableBolts(kind)` (0x14b2f8): how many of a
/// character's bolts (of `kind`) are not yet owned (byte +1 of the 0x38-byte
/// record is zero).
pub fn bolt_int_count_available_bolts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = BOLT_DB_BASE as u64;
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S0][0];
        BOLT_CALL_CHAR_BOLT_COUNT.call(c, 0x0014_b320)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        BOLT_CALL_CHAR_BOLTS.call(c, 0x0014_b330)?;
        c.g[A2][0] = c.g[V0][0];
        let count_positive = (c.g[S2][0] as i64) > 0;
        c.g[A0][0] = 0;
        if count_positive {
            c.cycles += 1;
            c.g[A1][0] = c.g[S2][0];
            loop {
                c.cycles += 7;
                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(1)).lb(c);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x38));
                let more = c.g[A1][0] != 0;
                if c.g[V1][0] == 0 {
                    c.g[A0][0] = c.g[V0][0];
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_b340);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[A0][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Copy one 0x38-byte bolt record (56 bytes, seven doublewords) from `src`
/// to `dst`, in the generated order. Returns the last doubleword of each
/// lane (lane 1: +0x18, lane 2: +0x20, +0x28, +0x30), which the generated
/// code leaves in `v0`, `a2`, `a3` and `t0`.
pub(crate) fn bolt_copy_record(c: &mut Cpu, src: u32, dst: u32) -> [u64; 4] {
    let first = [
        Addr(src).read_u64_unaligned(c),
        Addr(src.wrapping_add(8)).read_u64_unaligned(c),
        Addr(src.wrapping_add(0x10)).read_u64_unaligned(c),
        Addr(src.wrapping_add(0x18)).read_u64_unaligned(c),
    ];
    Addr(dst).write_u64_unaligned(c, first[0]);
    Addr(dst.wrapping_add(8)).write_u64_unaligned(c, first[1]);
    Addr(dst.wrapping_add(0x10)).write_u64_unaligned(c, first[2]);
    Addr(dst.wrapping_add(0x18)).write_u64_unaligned(c, first[3]);
    let second = [
        Addr(src.wrapping_add(0x20)).read_u64_unaligned(c),
        Addr(src.wrapping_add(0x28)).read_u64_unaligned(c),
        Addr(src.wrapping_add(0x30)).read_u64_unaligned(c),
    ];
    Addr(dst.wrapping_add(0x20)).write_u64_unaligned(c, second[0]);
    Addr(dst.wrapping_add(0x28)).write_u64_unaligned(c, second[1]);
    Addr(dst.wrapping_add(0x30)).write_u64_unaligned(c, second[2]);
    [second[0], second[1], second[2], first[3]]
}

/// `BEBoltInt_GetAvailableBolts(kind, out)` (0x14b378): copy the unowned
/// bolt records of `kind` to `out` (0x38 bytes each). Returns how many were
/// copied. The records are walked from the last one back to the first.
pub fn bolt_int_get_available_bolts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = BOLT_DB_BASE as u64;
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S0][0];
        BOLT_CALL_CHAR_BOLT_COUNT.call(c, 0x0014_b3a8)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        BOLT_CALL_CHAR_BOLTS.call(c, 0x0014_b3b8)?;
        c.g[V1][0] = c.g[V0][0];
        let count_positive = (c.g[S3][0] as i64) > 0;
        c.g[A1][0] = 0;
        if count_positive {
            c.cycles += 1;
            c.g[A0][0] = c.g[S3][0];
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(1)).lb(c);
                let unowned = c.g[V0][0] == 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                if unowned {
                    c.cycles += 30;
                    let src = c.g[V1][0] as u32;
                    let dst = c.g[S2][0] as u32;
                    let regs = bolt_copy_record(c, src, dst);
                    c.g[V0][0] = regs[0];
                    c.g[A2][0] = regs[1];
                    c.g[A3][0] = regs[2];
                    c.g[T0][0] = regs[3];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x38));
                }
                c.cycles += 2;
                let more = c.g[A0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x38));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_b3c8);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[A1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `BEBoltInt_DispatchCategory(category)` (0x14b700): the name of a bolt
/// category (0..9), from a jump table at 0x45a5c0 of handler addresses.
/// Categories of 10 or more return 0. An unknown handler leaves through
/// `jump_out`, as the generated code does.
pub fn bolt_int_dispatch_category(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A1][0] < 0xau64) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x5a40));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    let handler = c.g[A0][0] as u32;
    let name_offset: u32 = match handler {
        0x0014_b724 => 0x5be8,
        0x0014_b730 => 0x5bc0,
        0x0014_b73c => 0x5b98,
        0x0014_b748 => 0x5b68,
        0x0014_b754 => 0x5b40,
        0x0014_b760 => 0x5b18,
        0x0014_b76c => 0x5ae8,
        0x0014_b778 => 0x5ac0,
        0x0014_b784 => 0x5a98,
        0x0014_b790 => 0x5a70,
        _ => {
            c.jump_out(handler, 0x0014_b700, 0x0014_b7a4);
            return;
        }
    };
    c.cycles += 3;
    c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(name_offset));
    abi::ret(c);
}

/// `BEBoltInt_CollectRiderEquippedTextureNames(self, rider, a2, a3)`
/// (0x14b920): the equipped bolt textures of a rider's character, through
/// `BEBoltInt_CollectEquippedTextureNames`.
pub fn bolt_int_collect_rider_equipped_texture_names(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BOLT_CALL_RIDER_PROFILE.call(c, 0x0014_b94c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        BOLT_CALL_RIDER_CHARACTER.call(c, 0x0014_b958)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[A2][0] = c.g[V0][0];
        BOLT_CALL_COLLECT_TEXTURES.call(c, 0x0014_b96c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `BEBoltInt_GetRiderBoltTextureName(self, rider, a2, a3, t0)` (0x14bba0):
/// the texture name of a rider's bolt, through `BEBoltInt_GetBoltTextureName`.
pub fn bolt_int_get_rider_bolt_texture_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A2][0];
        c.g[S4][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        BOLT_CALL_RIDER_PROFILE.call(c, 0x0014_bbd4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        BOLT_CALL_RIDER_CHARACTER.call(c, 0x0014_bbe0)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[T0][0] = c.g[S4][0];
        c.g[A2][0] = c.g[V0][0];
        BOLT_CALL_TEXTURE_NAME.call(c, 0x0014_bbf8)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `Bolts_GetDB()` (0x14bdb8): the bolt database.
pub(crate) const COLLECT_DB_CALL: Callee = Callee {
    addr: 0x0014_bdb8,
    f: ee::s01::Bolts_GetDB,
};

/// `BE_GetCharProfileData(self, profile, ch)` (0x14ad28).
pub(crate) const COLLECT_PROFILE_CALL: Callee = Callee {
    addr: 0x0014_ad28,
    f: ee::s01::BE_GetCharProfileData,
};

/// `Bolts_GetCharBoltCount(db, ch)` (0x14d988).
pub(crate) const COLLECT_COUNT_CALL: Callee = Callee {
    addr: 0x0014_d988,
    f: ee::s01::Bolts_GetCharBoltCount,
};

/// `Bolts_GetCharBolts(db, ch)` (0x14d998).
pub(crate) const COLLECT_BOLTS_CALL: Callee = Callee {
    addr: 0x0014_d998,
    f: ee::s01::Bolts_GetCharBolts,
};

/// `BEBoltInt_GetBoltTextureName(self, profile, ch, def, out)` (0x14b988).
pub(crate) const COLLECT_TEXTURE_CALL: Callee = Callee {
    addr: 0x0014_b988,
    f: ee::s01::BEBoltInt_GetBoltTextureName,
};

/// `Bolts_FindFirstChild(self, group, key)` (0x14d558).
pub(crate) const COLLECT_FIRST_CHILD_CALL: Callee = Callee {
    addr: 0x0014_d558,
    f: ee::s01::Bolts_FindFirstChild,
};

/// `Bolts_CollectChildren` (0x14d608), through its generated name (the
/// override slot once installed).
pub(crate) const COLLECT_CHILDREN_CALL: Callee = Callee {
    addr: 0x0014_d608,
    f: ee::s01::f_0014d608,
};

/// `strcmp` (0x4165a8) and `strcpy` (0x4166f4), through the function table.
pub(crate) const COLLECT_STRCMP: u32 = 0x0041_65a8;

pub(crate) const COLLECT_STRCPY: u32 = 0x0041_66f4;

/// The address `reg + off`.
pub(crate) fn ea_reg(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr((c.g[reg][0] as u32).wrapping_add(off))
}

/// `getEntry(prof, def->slot)->flags`, inlined the way the generated body
/// does it: the slot (`def+4`) indexes the keytab at `prof+0x288` (halfwords),
/// the index selects a 4-byte entry at `prof+0x290`, and its flags are the
/// halfword at +2. Leaves `v0` = the flags and `v1` = the keytab pointer.
pub(crate) fn collect_entry_flags(c: &mut Cpu, def: u32, prof: u32) -> u64 {
    c.cycles += 7;
    c.g[V0][0] = Addr(def.wrapping_add(4)).lh(c);
    c.g[V1][0] = Addr(prof.wrapping_add(0x288)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = Addr(c.g[V0][0] as u32).lh(c);
    let negative = (c.g[V0][0] as i64) < 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    if negative {
        c.cycles += 1;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
        c.g[V0][0] = sx(prof.wrapping_add(c.g[V0][0] as u32));
    }
    c.cycles += 5;
    c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(2)).lhu(c);
    c.g[V0][0]
}

/// `int BEBoltInt_CollectEquippedFileNames(self, profile, ch, out)`
/// (0x14bc18). Like the texture-name collector, but copies each bolt's own
/// name (+0x28) into the next 256-byte slot of `out` for every bolt with flag
/// 0x10 whose name is not yet there (strcmp over the slots). Returns the
/// number of slots used.
pub fn bolt_int_collect_equipped_file_names(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[FP][0] = c.g[A3][0];
        c.g[S4][0] = 0;
        COLLECT_DB_CALL.call(c, 0x0014_bc5c)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        COLLECT_PROFILE_CALL.call(c, 0x0014_bc70)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[S6][0] = c.g[V0][0];
        COLLECT_COUNT_CALL.call(c, 0x0014_bc80)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[S7][0] = c.g[V0][0];
        COLLECT_BOLTS_CALL.call(c, 0x0014_bc90)?;
        c.g[A0][0] = c.g[V0][0];
        let any = (c.g[S7][0] as i64) > 0;
        c.g[A1][0] = 0;
        if any {
            c.cycles += 1;
            c.g[S5][0] = c.g[FP][0];
            loop {
                c.cycles += 4;
                c.g[S1][0] = ea_reg(c, A0, 0x28).lw(c);
                c.g[S3][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                let named = c.g[S1][0] != 0;
                c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
                let mut trailing = true;
                if named {
                    let def = c.g[A0][0] as u32;
                    let prof = c.g[S6][0] as u32;
                    let flags = collect_entry_flags(c, def, prof);
                    c.g[V0][0] = flags & 0x10;
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_sub(1));
                        let mut found = false;
                        loop {
                            c.cycles += 2;
                            let done = (c.g[S0][0] as i64) < 0;
                            c.g[A1][0] = sx((c.g[S0][0] as u32) << 8);
                            if done {
                                break;
                            }
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            call_ptr(c, COLLECT_STRCMP, 0x0014_bd0c)?;
                            let differs = c.g[V0][0] != 0;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                            if differs {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014_bcf8);
                                }
                                continue;
                            }
                            c.cycles += 2;
                            c.g[A1][0] = c.g[S3][0];
                            found = true;
                            break;
                        }
                        if found {
                            trailing = false;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, COLLECT_STRCPY, 0x0014_bd28)?;
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x100));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                        }
                    }
                }
                if trailing {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S3][0];
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S7][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S2][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_bca0);
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The scan of `Bolts_CollectChildren` over the group's own list (the
/// generated `key == -1`, or no lookup table, branch). Ends with `v0` = the
/// count (`s3`).
pub(crate) fn collect_children_linear_628(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    c.g[V0][0] = sx((c.g[FP][0] as u32) << 2);
    c.g[A0][0] = 0x38;
    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[S2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xa4)).lw(c);
    c.g[V0][0] = c.g[V1][0];
    c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2c)).lw(c);
    c.mult(0, A0, S2, A0);
    c.g[V1][0] = Addr((c.g[S5][0] as u32).wrapping_add(4)).lw(c);
    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let end = c.g[A1][0];
    Addr(sp(c)).sw(c, end);
    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[A1][0] as i64)) as u64;
    let any = c.g[V0][0] != 0;
    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    if !any {
        c.cycles += 1;
        c.g[V0][0] = c.g[S3][0];
        return Ok(());
    }
    c.cycles += 1;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(6)).lh(c);
        let mismatch = c.g[V0][0] != c.g[S4][0];
        c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let mut tail = true;
        if mismatch {
            c.cycles += 2;
            let below = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S3][0];
            if below {
                return Ok(());
            }
        } else {
            c.cycles += 4;
            let bolt = c.g[S0][0];
            Addr(c.g[S1][0] as u32).sw(c, bolt);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            let depth_zero = c.g[S7][0] == 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            if !depth_zero {
                c.cycles += 4;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x34)).lw(c);
                c.g[V0][0] &= 0x20;
                let mut recurse = true;
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lh(c);
                } else {
                    c.cycles += 2;
                    let no_all = c.g[S6][0] == 0;
                    c.g[V1][0] = Addr(sp(c)).lw(c);
                    if no_all {
                        recurse = false;
                        tail = false;
                    } else {
                        c.cycles += 1;
                        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lh(c);
                    }
                }
                if recurse {
                    c.cycles += 10;
                    c.g[A3][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = c.g[FP][0];
                    c.g[T0][0] = sx((c.g[S7][0] as u32).wrapping_sub(1));
                    c.g[T1][0] = c.g[S6][0];
                    COLLECT_CHILDREN_CALL.call(c, 0x0014_d6f8)?;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                }
            }
        }
        if tail {
            c.cycles += 1;
            c.g[V1][0] = Addr(sp(c)).lw(c);
        }
        c.cycles += 4;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = c.g[S3][0];
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_d6a8);
        }
    }
}

/// The lookup branch of `Bolts_CollectChildren`: the run of bolts that the
/// group's lookup table gives for `key` (the first one from
/// `Bolts_FindFirstChild`). Ends with `v0` = the count (`s3`).
pub(crate) fn collect_children_lookup_628(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    c.g[A1][0] = c.g[FP][0];
    c.g[A2][0] = c.g[S4][0];
    COLLECT_FIRST_CHILD_CALL.call(c, 0x0014_d738)?;
    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[V1][0] = sx((c.g[S4][0] as u32) << 1);
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lh(c);
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[S2][0] = c.g[V0][0];
    if none {
        c.cycles += 1;
        c.g[V0][0] = c.g[S3][0];
        return Ok(());
    }
    c.cycles += 1;
    loop {
        c.cycles += 4;
        let bolt = c.g[S0][0];
        Addr(c.g[S1][0] as u32).sw(c, bolt);
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
        let depth_zero = c.g[S7][0] == 0;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        let mut tail = true;
        if !depth_zero {
            c.cycles += 4;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x34)).lw(c);
            c.g[V0][0] &= 0x20;
            let mut recurse = true;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lh(c);
            } else {
                c.cycles += 2;
                if c.g[S6][0] == 0 {
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    recurse = false;
                    tail = false;
                } else {
                    c.cycles += 1;
                    c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lh(c);
                }
            }
            if recurse {
                c.cycles += 9;
                c.g[A3][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = c.g[FP][0];
                c.g[T0][0] = sx((c.g[S7][0] as u32).wrapping_sub(1));
                c.g[T1][0] = c.g[S6][0];
                COLLECT_CHILDREN_CALL.call(c, 0x0014_d79c)?;
                c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
            }
        }
        if tail {
            c.cycles += 1;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
        }
        c.cycles += 2;
        let more = c.g[S2][0] != 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_d758);
        }
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[S3][0];
    Ok(())
}

/// `int Bolts_CollectChildren(self, group, key, out, depth, all)` (0x14d608):
/// writes the pointers of the bolts of `group` under `key` into `out` (one
/// word each), and for each bolt with children (flag 0x20 clear, or `all`)
/// recurses one level less deep. With `key` -1 or no lookup table for the
/// group, it scans the group's list; otherwise the lookup table gives the run
/// of bolts for `key`. Returns the count.
pub fn bolts_collect_children(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S4, 0x60);
        c.g[S3][0] = 0;
        frame.save(c, S5, 0x50);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S7, 0x30);
        c.g[S6][0] = c.g[T1][0];
        frame.save(c, FP, 0x20);
        c.g[S7][0] = c.g[T0][0];
        c.g[FP][0] = c.g[A1][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        frame.save_ra(c, 0x10);
        let lookup = if c.g[S4][0] == c.g[V0][0] {
            false
        } else {
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[FP][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x11c));
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            let table = c.g[A0][0] != 0;
            c.g[A0][0] = c.g[S5][0];
            table
        };
        if lookup {
            collect_children_lookup_628(c)?;
        } else {
            collect_children_linear_628(c)?;
        }
        c.cycles += 12;
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
    });
}

/// `Econ* Bolts_FindTable2EntriesByFlag(Bolts* self, int index, int kind,
/// int want, int* out)` (0x14db40): walk the `table2[index]` entries (12 bytes
/// each: flag byte at +1, kind halfword at +2) and return the first one whose
/// kind equals `kind` and whose flag (`> 0`) equals `want`, counting every
/// match in `*out` (the list is sorted by kind: a larger kind ends the walk).
pub fn bolts_find_table2_entries_by_flag_2(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    Addr(c.g[T0][0] as u32).sw(c, 0);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20c));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T2][0] = Addr(c.g[V1][0] as u32).lw(c);
    let has_entries = (c.g[T2][0] as i64) > 0;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x284));
    if !has_entries {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let empty = c.g[V0][0] == 0;
    c.g[T3][0] = 0;
    'exit: {
        if empty {
            break 'exit;
        }
        c.cycles += 3;
        c.g[V0][0] = 0xc;
        c.g[T4][0] = c.g[V1][0];
        c.mult(0, 10, 9, 2);
        loop {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
            c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(2).lh(c);
            if c.g[V0][0] != c.g[A2][0] {
                // Other kind: past ours (the list is sorted) ends the walk.
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    break 'exit;
                }
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(1).lb(c);
                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                if c.g[V0][0] != c.g[A3][0] {
                    c.cycles += 2;
                    if c.g[T3][0] != 0 {
                        break 'exit;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
                    if c.g[T3][0] == 0 {
                        c.g[T3][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    let count = c.g[V0][0];
                    Addr(c.g[T0][0] as u32).sw(c, count);
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                }
            }
            c.cycles += 6;
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
            c.g[V1][0] = Addr(c.g[T4][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0xc));
            if !more {
                break 'exit;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_db88);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T3][0];
    ret(c);
}

/// `void BELibrary_SerializeSave(char* data)` (0x14a188): copies the race DB,
/// the player DB, the profile blocks of rider 0 and (if present) rider 1, the
/// 0x9b50-byte block and the rules into the save buffer, one after another
/// (8-byte-aligned fast path or unaligned `ldl/ldr` path, per block).
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn be_library_serialize_save(c: &mut Cpu) {
    guest(c, |c| {
        'b_14a258: {
            'b_14a210: {
                c.cycles += 7;
                c.g[V0][0] = 0x530000;
                c.g[A1][0] = c.g[A0][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b0));
                c.g[V0][0] = c.g[A1][0] | c.g[T0][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[A3][0] = c.g[A1][0];
                if t {
                    break 'b_14a210;
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4c));
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
                    c.g[T1][0] = 0x530000;
                    c.g[T2][0] = 0x4a0000;
                    c.g[T3][0] = 0x4c0000;
                    c.g[T4][0] = 0x530000;
                    'l_14a1bc: loop {
                        c.cycles += 19;
                        c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[T0][0] as u32);
                        c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
                        c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
                        c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0x17));
                        c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x10));
                        c.ldl(13, (c.g[T0][0] as u32).wrapping_add(0x1f));
                        c.ldr(13, (c.g[T0][0] as u32).wrapping_add(0x18));
                        c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.sdr(3, c.g[A3][0] as u32);
                        c.sdl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                        c.sdr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.sdl(6, (c.g[A3][0] as u32).wrapping_add(0x17));
                        c.sdr(6, (c.g[A3][0] as u32).wrapping_add(0x10));
                        c.sdl(13, (c.g[A3][0] as u32).wrapping_add(0x1f));
                        c.sdr(13, (c.g[A3][0] as u32).wrapping_add(0x18));
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                        let t = c.g[T0][0] != c.g[V0][0];
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014a1bc);
                            }
                            continue 'l_14a1bc;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[A1][0] = c.g[A0][0];
                            if t {
                                break 'b_14a258;
                            } else {
                                break 'b_14a210;
                            }
                        }
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4c));
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
            c.g[T1][0] = 0x530000;
            c.g[T2][0] = 0x4a0000;
            c.g[T3][0] = 0x4c0000;
            c.g[T4][0] = 0x530000;
            'l_14a228: loop {
                c.cycles += 11;
                c.g[T6][0] = c.ld(c.g[T0][0] as u32);
                c.g[T7][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A3][0] as u32, c.g[T6][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[T7][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                let t = c.g[T0][0] != c.g[V0][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014a228);
                    }
                    continue 'l_14a228;
                } else {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[A0][0];
                    break 'b_14a258;
                }
            }
        }
        'b_14a31c: {
            'b_14a2e8: {
                c.cycles += 13;
                c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x5b20));
                c.g[V0][0] = c.g[A1][0] | c.g[A2][0];
                c.ldl(13, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.ldr(13, c.g[T0][0] as u32);
                c.lwl(14, (c.g[T0][0] as u32).wrapping_add(0xb));
                c.lwr(14, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.sdl(13, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.sdr(13, c.g[A3][0] as u32);
                c.swl(14, (c.g[A3][0] as u32).wrapping_add(0xb));
                c.g[V1][0] = c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.swr(14, (c.g[A3][0] as u32).wrapping_add(0x8));
                if t {
                    break 'b_14a2e8;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa8));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    'l_14a294: loop {
                        c.cycles += 19;
                        c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.ldr(7, c.g[A2][0] as u32);
                        c.ldl(8, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.ldr(8, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.ldl(13, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.ldr(13, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.ldl(14, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.ldr(14, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.sdr(7, c.g[V1][0] as u32);
                        c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.sdl(13, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.sdr(13, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.sdl(14, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.sdr(14, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        let t = c.g[A2][0] != c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014a294);
                            }
                            continue 'l_14a294;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_14a31c;
                            } else {
                                break 'b_14a2e8;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa8));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
            'l_14a2f0: loop {
                c.cycles += 11;
                c.g[T7][0] = c.ld(c.g[A2][0] as u32);
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.sd(c.g[V1][0] as u32, c.g[T7][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014a2f0);
                    }
                    continue 'l_14a2f0;
                } else {
                    break 'b_14a31c;
                }
            }
        }
        'b_14a3e4: {
            'b_14a3b0: {
                c.cycles += 14;
                c.ldl(13, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(13, c.g[A2][0] as u32);
                c.sdl(13, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x6ca8));
                c.sdr(13, c.g[V1][0] as u32);
                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x5b20));
                c.g[V1][0] = 0xf88;
                c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x11));
                c.mult(0, 3, 2, 3);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = c.g[V1][0] | c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[A2][0] = c.g[A1][0];
                if t {
                    break 'b_14a3b0;
                } else {
                    c.cycles += 2;
                    c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf88));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf80));
                    'l_14a35c: loop {
                        c.cycles += 19;
                        c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(4, c.g[V1][0] as u32);
                        c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(10, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(10, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(4, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.sdr(4, c.g[A2][0] as u32);
                        c.sdl(5, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.sdr(5, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.sdl(10, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.sdr(10, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014a35c);
                            }
                            continue 'l_14a35c;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_14a3e4;
                            } else {
                                break 'b_14a3b0;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf88));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf80));
            'l_14a3b8: loop {
                c.cycles += 11;
                c.g[T5][0] = c.ld(c.g[V1][0] as u32);
                c.g[T6][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T7][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[A2][0] as u32, c.g[T5][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T6][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[T7][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014a3b8);
                    }
                    continue 'l_14a3b8;
                } else {
                    break 'b_14a3e4;
                }
            }
        }
        'b_14a4c8: {
            c.cycles += 8;
            c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(5, c.g[V1][0] as u32);
            c.sdl(5, (c.g[A2][0] as u32).wrapping_add(0x7));
            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x5b20));
            c.sdr(5, c.g[A2][0] as u32);
            c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x2d));
            let t = (c.g[V0][0] as i64) < 0;
            c.g[A1][0] = c.g[A3][0];
            if t {
                break 'b_14a4c8;
            } else {
                'b_14a4b8: {
                    'b_14a484: {
                        c.cycles += 9;
                        c.g[A0][0] = 0xf88;
                        c.g[V1][0] = 0x4b0000;
                        c.mult(0, 4, 2, 4);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7f8));
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = c.g[V1][0] | c.g[A1][0];
                        c.g[V0][0] = c.g[V0][0] & 0x7;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A2][0] = c.g[A1][0];
                        if t {
                            break 'b_14a484;
                        } else {
                            c.cycles += 2;
                            c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf88));
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf80));
                            'l_14a430: loop {
                                c.cycles += 19;
                                c.ldl(14, (c.g[V1][0] as u32).wrapping_add(0x7));
                                c.ldr(14, c.g[V1][0] as u32);
                                c.ldl(15, (c.g[V1][0] as u32).wrapping_add(0xf));
                                c.ldr(15, (c.g[V1][0] as u32).wrapping_add(0x8));
                                c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x17));
                                c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x10));
                                c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x18));
                                c.sdl(14, (c.g[A2][0] as u32).wrapping_add(0x7));
                                c.sdr(14, c.g[A2][0] as u32);
                                c.sdl(15, (c.g[A2][0] as u32).wrapping_add(0xf));
                                c.sdr(15, (c.g[A2][0] as u32).wrapping_add(0x8));
                                c.sdl(4, (c.g[A2][0] as u32).wrapping_add(0x17));
                                c.sdr(4, (c.g[A2][0] as u32).wrapping_add(0x10));
                                c.sdl(5, (c.g[A2][0] as u32).wrapping_add(0x1f));
                                c.sdr(5, (c.g[A2][0] as u32).wrapping_add(0x18));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0014a430);
                                    }
                                    continue 'l_14a430;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_14a4b8;
                                    } else {
                                        break 'b_14a484;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf88));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf80));
                    'l_14a48c: loop {
                        c.cycles += 11;
                        c.g[T0][0] = c.ld(c.g[V1][0] as u32);
                        c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[T2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[T5][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sd(c.g[A2][0] as u32, c.g[T0][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                        c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T5][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014a48c);
                            }
                            continue 'l_14a48c;
                        } else {
                            break 'b_14a4b8;
                        }
                    }
                }
                c.cycles += 4;
                c.ldl(14, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(14, c.g[V1][0] as u32);
                c.sdl(14, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.sdr(14, c.g[A2][0] as u32);
                break 'b_14a4c8;
            }
        }
        'b_14a56c: {
            'b_14a538: {
                c.cycles += 5;
                c.g[A1][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x5cb8));
                c.g[V0][0] = c.g[A3][0] | c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.g[A3][0];
                if t {
                    break 'b_14a538;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x9b40;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    'l_14a4e4: loop {
                        c.cycles += 19;
                        c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.ldr(6, c.g[A1][0] as u32);
                        c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x17));
                        c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0x1f));
                        c.ldr(10, (c.g[A1][0] as u32).wrapping_add(0x18));
                        c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.sdr(6, c.g[V1][0] as u32);
                        c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.sdr(9, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.sdl(10, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.sdr(10, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014a4e4);
                            }
                            continue 'l_14a4e4;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_14a56c;
                            } else {
                                break 'b_14a538;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x9b40;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            'l_14a540: loop {
                c.cycles += 11;
                c.g[T5][0] = c.ld(c.g[A1][0] as u32);
                c.g[T6][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[T7][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                c.sd(c.g[V1][0] as u32, c.g[T5][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T6][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T7][0]);
                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014a540);
                    }
                    continue 'l_14a540;
                } else {
                    break 'b_14a56c;
                }
            }
        }
        c.cycles += 29;
        c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(6, c.g[A1][0] as u32);
        c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(6, c.g[V1][0] as u32);
        c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.g[V0][0] = 0x9b50;
        c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add(0x8b8));
        c.ldl(13, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.ldr(13, c.g[V0][0] as u32);
        c.ldl(14, (c.g[V0][0] as u32).wrapping_add(0xf));
        c.ldr(14, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.ldl(15, (c.g[V0][0] as u32).wrapping_add(0x17));
        c.ldr(15, (c.g[V0][0] as u32).wrapping_add(0x10));
        c.sdl(13, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.sdr(13, c.g[A1][0] as u32);
        c.sdl(14, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.sdr(14, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(15, (c.g[A1][0] as u32).wrapping_add(0x17));
        c.sdr(15, (c.g[A1][0] as u32).wrapping_add(0x10));
        c.ldl(13, (c.g[V0][0] as u32).wrapping_add(0x1f));
        c.ldr(13, (c.g[V0][0] as u32).wrapping_add(0x18));
        c.sdl(13, (c.g[A1][0] as u32).wrapping_add(0x1f));
        c.sdr(13, (c.g[A1][0] as u32).wrapping_add(0x18));
        let t = c.g[RA][0] as u32;
        c.pc = t;
        return Ok(());
    });
}

/// `int BEBoltInt_CollectEquippedTextureNames(self, profile, ch, out)` (0x14b7a8): for
/// each bolt of character `ch` that is active and whose profile entry has flag
/// 0x10, stores the bolt's texture file name (`BEBoltInt_GetBoltTextureName`)
/// in the next 256-byte slot of `out` unless an earlier slot already holds it
/// (`strcmp`). Returns the number of slots used.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn bolt_int_collect_equipped_texture_names_2(c: &mut Cpu) {
    guest(c, |c| {
        'b_14b8e8: {
            c.cycles += 33;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S6][0] = c.g[A3][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S7][0] = c.g[A2][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sw(c.g[SP][0] as u32, c.g[A0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[RA][0] = sx(0x0014b7ec);
            c.g[S3][0] = 0u64;
            call_ptr(c, 0x0014bdb8, 0x0014b7ec)?;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[RA][0] = sx(0x0014b800);
            c.g[A2][0] = c.g[S7][0];
            call_ptr(c, 0x0014ad28, 0x0014b800)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S7][0];
            c.g[RA][0] = sx(0x0014b810);
            c.g[FP][0] = c.g[V0][0];
            call_ptr(c, 0x0014d998, 0x0014b810)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0014b820);
            c.g[A1][0] = c.g[S7][0];
            call_ptr(c, 0x0014d988, 0x0014b820)?;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_14b8e8;
            } else {
                c.cycles += 1;
                'l_14b830: loop {
                    'b_14b8d4: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
                        c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        let t = c.g[V0][0] == 0u64;
                        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                        if t {
                            break 'b_14b8d4;
                        } else {
                            'b_14b86c: {
                                'b_14b868: {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x288));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                    let t = (c.g[V0][0] as i64) < 0;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    if t {
                                        break 'b_14b868;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                        let t = true;
                                        c.g[V0][0] =
                                            sx((c.g[FP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        if t {
                                            break 'b_14b86c;
                                        } else {
                                            break 'b_14b868;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[V0][0] = 0u64;
                                break 'b_14b86c;
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                            c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = c.g[V0][0] & 0x10;
                            let t = c.g[V0][0] == 0u64;
                            c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                            if t {
                                break 'b_14b8d4;
                            } else {
                                c.cycles += 10;
                                c.g[V0][0] = sx((c.g[S3][0] as u32) << 8);
                                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                c.g[S2][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[A3][0] = c.g[S1][0];
                                c.g[A2][0] = c.g[S7][0];
                                c.g[RA][0] = sx(0x0014b8a0);
                                c.g[T0][0] = c.g[S2][0];
                                call_ptr(c, 0x0014b988, 0x0014b8a0)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                if t {
                                    break 'b_14b8d4;
                                } else {
                                    c.cycles += 2;
                                    c.g[S0][0] = c.g[S1][0];
                                    'l_14b8b0: loop {
                                        'b_14b8d0: {
                                            c.cycles += 2;
                                            let t = (c.g[S0][0] as i64) < 0;
                                            c.g[A0][0] = c.g[S2][0];
                                            if t {
                                                break 'b_14b8d0;
                                            } else {
                                                c.cycles += 5;
                                                c.g[A1][0] = sx((c.g[S0][0] as u32) << 8);
                                                c.g[RA][0] = sx(0x0014b8c4);
                                                c.g[A1][0] = sx((c.g[S6][0] as u32)
                                                    .wrapping_add(c.g[A1][0] as u32));
                                                call_ptr(c, 0x004165a8, 0x0014b8c4)?;
                                                if c.g[V0][0] != 0u64 {
                                                    c.g[S0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0014b8b0);
                                                    }
                                                    continue 'l_14b8b0;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[S3][0] = c.g[S1][0];
                                                    break 'b_14b8d0;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                        break 'b_14b8d4;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.g[S5][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.g[S4][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014b830);
                        }
                        continue 'l_14b830;
                    } else {
                        break 'b_14b8e8;
                    }
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S3][0];
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
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `int Bolts_CollectByGroup(self, g, key, out, depth, all)` (0x14d7e8): collects the
/// bolt nodes of group `g` whose parent is `key` into `out` and descends into
/// each match (recursively, `depth - 1`, unless it is 0x20-flagged and `all` is
/// clear). Returns how many pointers were written.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn bolts_collect_by_group_2(c: &mut Cpu) {
    guest(c, |c| {
        'b_14d8d0: {
            c.cycles += 30;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S7][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S6][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = 0x38;
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.g[V1][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A3][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[FP][0] = c.g[A2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S5][0] = c.g[T0][0];
            c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
            c.g[S4][0] = c.g[T1][0];
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
            c.g[S3][0] = 0u64;
            c.mult(0, 4, 18, 4);
            c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            if t {
                break 'b_14d8d0;
            } else {
                c.cycles += 2;
                c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                'l_14d868: loop {
                    'b_14d8c8: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x8));
                        if c.g[V0][0] != c.g[FP][0] {
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            break 'b_14d8c8;
                        } else {
                            'b_14d8c4: {
                                c.cycles += 4;
                                c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                let t = c.g[S5][0] == 0u64;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_14d8c4;
                                } else {
                                    'b_14d8a0: {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                                        c.g[V0][0] = c.g[V0][0] & 0x20;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[A2][0] =
                                                c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                            break 'b_14d8a0;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[S4][0] == 0u64 {
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_14d8c8;
                                            } else {
                                                c.cycles += 1;
                                                c.g[A2][0] =
                                                    c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                                break 'b_14d8a0;
                                            }
                                        }
                                    }
                                    c.cycles += 9;
                                    c.g[A3][0] = c.g[S1][0];
                                    c.g[A0][0] = c.g[S6][0];
                                    c.g[A1][0] = c.g[S7][0];
                                    c.g[T0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                    c.g[RA][0] = sx(0x0014d8b8);
                                    c.g[T1][0] = c.g[S4][0];
                                    call_ptr(c, 0x0014d7e8, 0x0014d8b8)?;
                                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[S3][0] =
                                        sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[S1][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    break 'b_14d8c4;
                                }
                            }
                            c.cycles += 1;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            break 'b_14d8c8;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S2][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014d868);
                        }
                        continue 'l_14d868;
                    } else {
                        break 'b_14d8d0;
                    }
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S3][0];
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
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `Bolts_CountChildren` (0x0014d4c8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn bolts_count_children_2(c: &mut Cpu) {
    if c.pc != 0x0014d4c8 {
        return ee::s01::f_0014d4c8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 6;
        c.g[T0][0] = c.g[A0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));

        if c.g[V0][0] != 0u64 {
            // direct table (0x0014d548): halfword at table + a2 * 2
            c.g[V1][0] = sx((c.g[A2][0] as u32) << 1);
            c.cycles += 3;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = c.lh(c.g[V1][0] as u32);
            c.pc = t;
            return Ok(());
        }
        // scan the 0x38-byte records (0x0014d4e0)
        c.cycles += 11;
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A0][0] = 0x38;
        c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
        c.g[V0][0] = c.g[V1][0];
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
        c.g[T1][0] = 0u64;
        c.mult(0, 4, 7, 4);
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        loop {
            // index < end? (0x0014d518)
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V0][0] == 0u64 {
                break;
            }
            // compare the record's halfword at +6 with a2 (0x0014d524)
            c.cycles += 3;
            c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x6));
            let ne = c.g[V0][0] != c.g[A2][0];
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if ne {
                // not equal (0x0014d538): stop when a2 < value
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x38));
                if !t {
                    break;
                }
                if back_edge_unwinds(c, 0x0014d514) {
                    return Ok(());
                }
                // next index (0x0014d514)
                c.cycles += 1;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            } else {
                // equal: count it (0x0014d530)
                c.cycles += 2;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                if back_edge_unwinds(c, 0x0014d510) {
                    return Ok(());
                }
                // next record (0x0014d510), next index (0x0014d514)
                c.cycles += 1;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x38));
                c.cycles += 1;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            }
        }
        // return the count (0x0014d540)
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = c.g[T1][0];
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;

    use crate::difftest::fill;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::be::beintmission::tests::instance_setup;
    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base_4;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_5;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::main::debugmenu::tests::alloc_setup;

    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_2;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_2;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn bolts_init_db_matches() {
        check(ssx3_ee::s01::f_0014c620, bolts_init_db, 50, &|c, seed| {
            let mut r = Rng(0x1000_0001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for off in (0..0x2c).step_by(4) {
                c.bus.write32(DATA + off, r.next_u32());
            }
        });
    }

    #[test]
    fn bolts_get_char_bolt_count_matches() {
        check(
            ssx3_ee::s01::f_0014d988,
            bolts_get_char_bolt_count,
            100,
            &|c, seed| {
                let mut r = Rng(0x2000_0002 ^ seed as u64);
                scramble(c, &mut r);
                let idx = seed % 4;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = idx as u64;
                c.bus.write32(DATA + idx * 4 + 0x2c, r.next_u32());
            },
        );
    }

    #[test]
    fn bolts_get_char_bolts_2_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014d9d0,
            bolts_get_char_bolts_2,
            300,
            &|c, seed| {
                let mut r = Rng(0x3000_0004 ^ seed as u64);
                scramble(c, &mut r);
                let idx = seed % 4;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = idx as u64;
                let record = DATA + idx * 4;
                let count = if seed % 2 == 0 {
                    r.next_u32() | 0x8000_0000
                } else {
                    (r.next_u32() & 0x7fff_ffff) | 1
                };
                c.bus.write32(record + 0x2c, count);
                c.bus.write32(record + 0xa4, r.next_u32());
                c.bus.write32(DATA + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn be_bolt_int_get_bolt_states_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014ae28,
            be_bolt_int_get_bolt_states,
            200,
            &|c, seed| {
                let mut r = Rng(0x14ae ^ seed as u64);
                scramble(c, &mut r);
                let a1 = (r.next_u32() % 4) as u32;
                let a2 = (r.next_u32() % 4) as u32;
                c.g[A1][0] = a1 as u64;
                c.g[A2][0] = a2 as u64;
                c.g[A3][0] = (DATA + 0x800) as u64;
                let addr = BOLT_RECORDS
                    .wrapping_add(BOLT_RECORDS_OFF)
                    .wrapping_add(a1.wrapping_mul(0x9b50))
                    .wrapping_add(a2.wrapping_mul(0xf88))
                    .wrapping_add(0x28c);
                c.bus.write32(addr, r.next_u32());
            },
        );
    }

    #[test]
    fn be_bolt_int_get_instance_matches() {
        check(
            ssx3_ee::s01::f_0014abe0,
            be_bolt_int_get_instance,
            90,
            &|c, seed| {
                instance_setup(c, seed, 0x1f28, 0xb01);
            },
        );
    }

    #[test]
    fn beb_bolt_int_getters_match() {
        for (gen, hand, salt) in [
            (
                ssx3_ee::s01::f_0014ac30 as fn(&mut Cpu),
                beb_bolt_int_get_rider_character as fn(&mut Cpu),
                0xac30,
            ),
            (
                ssx3_ee::s01::f_0014ac50,
                beb_bolt_int_get_rider_profile,
                0xac50,
            ),
        ] {
            check(gen, hand, 60, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A1][0] = (seed % 12) as u64;
                let _ = &mut r;
            });
        }
    }

    #[test]
    fn beb_bolt_int_get_rider_profile_and_char_matches() {
        check(
            ssx3_ee::s01::f_0014ac70,
            beb_bolt_int_get_rider_profile_and_char,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0xac70, 0);
                c.g[A1][0] = (seed % 12) as u64;
                // Sometimes `out` aliases the input pointer.
                c.g[A2][0] = (if seed % 4 == 0 {
                    DATA + 0x300
                } else {
                    DATA + 0x400
                }) as u64;
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn be_find_rider_by_profile_matches() {
        check(
            ssx3_ee::s01::f_0014a130,
            be_find_rider_by_profile,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xa130, 0);
                // All six rider profile slots index entry 0 of the table.
                for i in 0..6u32 {
                    c.bus.write32(0x5305b0 + i * 4, 0);
                }
                fill(c, &mut r, 0x535b20, 0x200);
                let bit = c.bus.read32(0x535b30) & 1;
                c.g[A0][0] = if seed % 2 == 0 {
                    bit as u64
                } else {
                    (bit ^ 1) as u64
                };
            },
        );
    }

    #[test]
    fn beb_bolt_int_profile_state_matches() {
        for (gen, hand, salt, ptr_arg) in [
            (
                ssx3_ee::s01::f_0014ad50 as fn(&mut Cpu),
                beb_bolt_int_get_rider_profile_data as fn(&mut Cpu),
                0xad50,
                true,
            ),
            (
                ssx3_ee::s01::f_0014ae60,
                beb_bolt_int_get_rider_bolt_states,
                0xae60,
                true,
            ),
            (
                ssx3_ee::s01::f_0014acb0,
                beb_bolt_int_set_bolt_states,
                0xacb0,
                true,
            ),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A1][0] = (seed % 8) as u64;
                if ptr_arg {
                    c.g[A2][0] = (DATA + 0x2000) as u64;
                }
                c.g[A3][0] = (DATA + 0x3000) as u64;
                c.g[A0][0] = (DATA + 0x4000) as u64;
                fill(c, &mut r, DATA, 0x4000);
            });
        }
    }

    /// Call a generated helper as a setup step (arguments in a0..t1).
    pub(crate) fn load(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
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

    /// The main heap's arena over 256 KiB of test RAM, so constructors and
    /// destructors that allocate or free take their normal path.
    pub(crate) fn alloc_ready(c: &mut Cpu) {
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

    #[test]
    fn be_score_int_get_high_score_table_matches() {
        check_lohi(
            ee::s01::f_0014ab20__generated,
            be_score_int_get_high_score_table,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x14ab20, 0);
                let (a, b) = match seed % 4 {
                    0 => (0u64, 6u64),
                    1 => (0, 7),
                    2 => (16, 7),
                    _ => (0, 12),
                };
                c.g[A0][0] = a;
                c.g[A1][0] = b;
            },
        );
    }

    #[test]
    fn bolts_destroy_db_matches() {
        check(
            ee::s01::f_0014c658__generated,
            bolts_destroy_db,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x14c658, 0);
                alloc_ready(c);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn be_bolt_int_save_equip_flags_matches() {
        check(
            ssx3_ee::s01::f_0014aea8__generated,
            be_bolt_int_save_equip_flags,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x14aea8);
                c.g[A0][0] = 0;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                let n = seed % 5;
                c.bus.write32(0x004a_6ca8 + 0x28c, n);
                for i in 0..n {
                    let e = 0x004a_6ca8 + 0x290 + 4 * i;
                    c.bus.write16(e, r.next_u32() as u16);
                    c.bus.write16(
                        e + 2,
                        if seed % 2 == 0 {
                            (r.next_u32() | 0x10) as u16
                        } else {
                            (r.next_u32() & !0x10) as u16
                        },
                    );
                }
            },
        );
    }

    #[test]
    fn bolts_index_bolts_by_char_matches() {
        check(
            ee::s01::f_0014c2b0__generated,
            bolts_index_bolts_by_char,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x14c2_b0);
                let count = seed % 4;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x1c, count);
                c.bus.write32(DATA + 4, DATA + 0x1000);
                for i in 0..count {
                    c.bus
                        .write8(DATA + 0x1000 + i * 0x38, (r.next_u32() % 3) as u8);
                }
                fill(c, &mut r, DATA + 0x2c, 4);
                fill(c, &mut r, DATA + 0xa4, 4);
            },
        );
    }

    #[test]
    fn be_get_event_record_index_matches() {
        check(
            ee::s01::f_0014ab68__generated,
            be_get_event_record_index,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x14_ab68, 0);
                let a = (seed % 20) as u32;
                let b = [6u32, 7, 8, 9, 10, 11, 12, 3][(seed % 8) as usize];
                c.g[A0][0] = a as u64;
                c.g[A1][0] = b as u64;
                let row = DATA + 0x2200 + 0x20 * (a % 5);
                if a < 17 {
                    w(c, 0x0045_a2f8 + 16 * a, row);
                }
                w(c, row, b);
                w(c, row + 4, 3 + seed % 4);
                w(c, row + 8, 0xffff_ffff);
                w(c, row + 0xc, 0x1a);
                c.pc = 0x0014_ab68;
            },
        );
    }

    #[test]
    fn bolts_find_bolt_index_matches() {
        check(
            ee::s01::f_0014d448__generated,
            bolts_find_bolt_index,
            SEEDS_3,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x4d448, 0);
                let self_ = DATA + 0x100;
                let group = (seed % 4) as u32;
                let key = ((seed / 4) % 8) as u32;
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(group);
                c.g[A2][0] = sx(key);
                if seed % 3 == 0 {
                    let tab = DATA + 0x3000;
                    c.bus.write32(self_ + 0x4dc + 4 * group, tab);
                    for k in 0..8u32 {
                        c.bus.write16(tab + 2 * k, (k as u32 + seed) as u16);
                    }
                } else {
                    c.bus.write32(self_ + 0x4dc + 4 * group, 0);
                    let first = ((seed / 3) % 4) as u32;
                    let count = ((seed / 12) % 4) as u32;
                    c.bus.write32(self_ + 0xa4 + 4 * group, first);
                    c.bus.write32(self_ + 0x2c + 4 * group, count);
                    let base_2 = DATA + 0x1000;
                    c.bus.write32(self_ + 4, base_2);
                    for i in 0..4u32 {
                        c.bus
                            .write16(base_2 + i * 0x38 + 4, ((i + seed) % 8) as u16);
                    }
                    let _ = &mut r;
                }
            },
        );
    }

    #[test]
    fn bolts_is_descendant_of_matches() {
        check(
            ee::s01::f_0014d908__generated,
            bolts_is_descendant_of,
            SEEDS_3,
            &|c, seed| {
                base_2(c, seed, 0x4d908, 0);
                let self_ = DATA + 0x100;
                let base_ = DATA + 0x1000;
                c.g[A0][0] = sx(self_);
                c.bus.write32(self_ + 4, base_);
                for g in 0..4u32 {
                    c.bus.write32(self_ + 0x4dc + 4 * g, 0);
                    c.bus.write32(self_ + 0xa4 + 4 * g, 0);
                    c.bus.write32(self_ + 0x2c + 4 * g, 4);
                }
                // Group 0: entries 0..3.
                //  e0: group tag 0, own key 1, parent key 7
                //  e1: group tag 0, own key 7, parent key -1
                //  e2: group tag 0, own key 7, parent key 5
                let keys = [(1u16, 7i16), (7, -1), (7, 5), (2, -1)];
                for (i, (own, parent)) in keys.iter().enumerate() {
                    let e = base_ + 0x38 * i as u32;
                    c.bus.write8(e, 0);
                    c.bus.write16(e + 4, *own);
                    c.bus.write16(e + 6, *parent as u16);
                }
                let start = (seed % 5) as u32;
                let key: u16 = match seed % 4 {
                    0 => 1,
                    1 => 7,
                    2 => 0xffff,
                    _ => 5,
                };
                let entry = if start >= 4 { 0 } else { base_ + 0x38 * start };
                c.g[A1][0] = sx(entry);
                c.g[A2][0] = sx(key as u32);
            },
        );
    }

    pub(crate) const SRC: u32 = DATA + 0x4000;

    // ---------------------------------------------------------------- tables

    #[test]
    fn bolts_index_table2_by_char_matches() {
        check(
            ee::s01::f_0014c320__generated,
            bolts_index_table2_by_char,
            6,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x14c320, 2);
                c.g[A0][0] = OBJ as u64;
                let n = (seed % 4) as u32;
                w(c, OBJ + 0x20, n);
                w(c, OBJ + 8, SRC);
                fill(c, &mut r, SRC, 4 * 4);
                for i in 0..n {
                    c.bus.write8(SRC + 0xc * i, ((i * 3) % 30) as u8);
                }
            },
        );
    }

    #[test]
    fn bolts_find_first_child_matches() {
        check(
            ee::s01::f_0014d558__generated,
            bolts_find_first_child,
            150,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x14d5);
                let g = (seed % 3) as u32;
                let key = [0xffff_ffffu64, 7, 0x41][(seed / 3 % 3) as usize];
                let variant = seed / 9 % 4;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = g as u64;
                c.g[A2][0] = key;
                c.bus.write32(
                    DATA + g * 4 + 0x194,
                    if variant == 0 { 0 } else { DATA + 0x1000 },
                );
                c.bus.write32(DATA + g * 4 + 0x11c, DATA + 0x1200);
                c.bus.write32(DATA + g * 4 + 0x2c, 3);
                c.bus.write32(DATA + g * 4 + 0xa4, 0);
                c.bus.write32(DATA + 4, DATA + 0x2000);
                c.bus.write16(
                    DATA + 0x1200 + (key as u32 & 0x7f) * 2,
                    if variant >= 2 { 1 } else { 0 },
                );
                c.bus.write16(DATA + 0x1000 + (key as u32 & 0x7f) * 2, 2);
                for i in 0..4u32 {
                    let e = DATA + 0x2000 + 0x38 * i;
                    for k in 0..0x38 / 4 {
                        c.bus.write32(e + 4 * k, r.next_u32());
                    }
                    c.bus.write16(e + 6, [0x41u16, 7, 9, 3][i as usize]);
                }
            },
        );
    }

    #[test]
    fn bolts_index_table3_by_char_matches() {
        check(
            ee::s01::f_0014c3c8__generated,
            bolts_index_table3_by_char,
            40,
            &|c, seed| {
                let mut r = Rng(0x0014_c3c8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let this = DATA;
                c.bus.write32(this + 0x24, (seed % 4) as u32);
                c.bus.write32(this + 0xc, DATA + 0x1000);
                for i in 0..4 {
                    c.bus.write32(DATA + 0x1000 + 8 * i, r.next_u32());
                    c.bus
                        .write8(DATA + 0x1000 + 8 * i, (r.next_u32() % 5) as u8);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn bolts_index_table4_by_char_matches() {
        check(
            ee::s01::f_0014c488__generated,
            bolts_index_table4_by_char,
            40,
            &|c, seed| {
                let mut r = Rng(0x0014_c488 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let this = DATA;
                c.bus.write32(this + 0x28, (seed % 4) as u32);
                c.bus.write32(this + 0x10, DATA + 0x1000);
                for i in 0..4 {
                    c.bus.write32(DATA + 0x1000 + 8 * i, r.next_u32());
                    c.bus
                        .write8(DATA + 0x1000 + 8 * i, (r.next_u32() % 5) as u8);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn sum_equipped_bolt_value_matches() {
        diff(
            ee::s01::f_0014b478__generated,
            be_bolt_int_sum_equipped_bolt_value,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14_b478, 0);
                globals(c);
                let idx = seed % 4;
                let count = (seed % 5) as u32;
                w(c, 0x004a_6750 + idx * 4 + 0x2c, count);
                w(c, 0x004a_6750 + 4, DATA);
                w(c, 0x004a_6750 + idx * 4 + 0xa4, 0);
                w(c, 0x004a_7eb8, DATA + 0x3000);
                for i in 0..count {
                    let bolt = DATA + i * bolts::STRIDE;
                    c.bus.write16(bolt + bolts::FLAGS, (seed as u16 % 5) as u16);
                    c.bus.write16(bolt + bolts::VALUE, r.next_u32() as u16);
                    c.bus.write16(
                        DATA + 0x3000 + 2 * (seed as u32 % 5),
                        (if seed % 3 == 0 { 0x8000 } else { 0x10 }) as u16,
                    );
                }
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = idx as u64;
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn bolts_free_db_matches() {
        check(
            ee::s01::f_0014d240__generated,
            bolts_free_db,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14_d240, 11);
                alloc_setup(c, &mut r);
                c.g[A0][0] = DATA as u64;
                // Cover both the free and the already-free path.
                w(c, DATA, if seed % 2 == 0 { 0 } else { ptr(&mut r) });
                w(c, DATA + 0x18, if seed % 3 == 0 { 0 } else { ptr(&mut r) });
            },
        );
    }

    pub(crate) const BOLT_SELF: u32 = DATA + 0x1_0000;

    #[test]
    fn be_bolt_int_unlock_bolt_matches() {
        check_all(
            ee::s01::f_0014b560__generated,
            be_bolt_int_unlock_bolt,
            20,
            &|c, seed| {
                let _ = base_5(c, seed, 0x14b5_60, 12);
                c.g[A0][0] = sx(BOLT_SELF);
                c.g[A1][0] = sx(seed % 3); // rider slot
                c.g[A2][0] = sx(seed % 4); // character
                c.g[A3][0] = sx((seed / 4) % 6); // bolt key
                c.g[T0][0] = sx(seed % 2); // free
            },
        );
    }

    /// BE_GetRiderAltCharacter: riders 0..15 with character ids 0..39 and
    /// alternate bytes of both signs.
    #[test]
    fn be_get_rider_alt_character_matches() {
        use crate::difftest::{check, scramble, Rng};
        check(
            ee::s01::f_0014a0b0__generated,
            be_get_rider_alt_character,
            200,
            &|c, seed| {
                let mut r = Rng(0x0014_a0b0 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x0014_a0b0;
                let rider = r.next_u32() % 16;
                let id = r.next_u32() % 40;
                c.bus.write32(BE_RIDER_CHARACTER_IDS + 4 * rider, id);
                c.bus.write8(
                    BE_CHARACTERS + 0x1c * id + BE_CHARACTER_ALT,
                    r.next_u32() as u8,
                );
                c.g[A0][0] = if seed % 2 == 0 {
                    rider as u64
                } else {
                    ((r.next_u32() as u64) << 32) | rider as u64
                };
            },
        );
    }

    pub(crate) use crate::game::world::worldview::tests::oc410_machine;
    pub(crate) use crate::game::world::worldview::tests::oc410_result;

    /// Test layout for `BEBoltInt_GetBoltTextureName`.
    pub(crate) const TEX_E0: u32 = DATA + 0x1000;

    pub(crate) const TEX_OUT: u32 = DATA + 0x7000;

    pub(crate) const TEX_BOLTS: u32 = DATA + 0x6000;

    pub(crate) const TEX_PROFILE: u32 = DATA + 0x9000;

    pub(crate) const TEX_KEYTAB: u32 = DATA + 0xa000;

    pub(crate) const TEX_STRINGS: u32 = DATA + 0xb000;

    /// A short name over the alphabet `a b $ .` (NUL-terminated) at `at`.
    pub(crate) fn tex_name(c: &mut Cpu, r: &mut Rng, at: u32, with_dollar: bool) {
        let len = 3 + r.next_u32() % 7;
        for i in 0..len {
            let ch = match r.next_u32() % 5 {
                0 => b'$',
                1 => b'.',
                2 => b'a',
                _ => b'b',
            };
            c.bus.write8(at + i, ch);
        }
        if with_dollar {
            c.bus.write8(at + r.next_u32() % len, b'$');
        }
        c.bus.write8(at + len, 0);
    }

    /// `GetBoltTextureName`: early outs (no group / no name / no `$`), then
    /// the `$` substitution over the rider's other unlocked bolts.
    #[test]
    fn beb_bolt_int_get_bolt_texture_name_matches() {
        check(
            ee::s01::f_0014b988__generated,
            beb_bolt_int_get_bolt_texture_name,
            400,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0014_b988, 0);
                c.pc = 0x0014_b988;
                c.g[A0][0] = (DATA + 0x500) as u64;
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = TEX_E0 as u64;
                c.g[T0][0] = TEX_OUT as u64;
                let group = match seed % 11 {
                    0 => 0xff,
                    n => n % 3,
                };
                c.bus.write8(TEX_E0 + 3, group as u8);
                let key = r.next_u32() % 6;
                c.bus.write16(TEX_E0 + 4, key as u16);
                let name = if seed % 13 == 1 { 0 } else { TEX_STRINGS };
                c.bus.write32(TEX_E0 + 0x2c, name);
                tex_name(c, &mut r, TEX_STRINGS, seed % 4 != 3);
                // The rider's other bolts.
                let count = 1 + seed % 5;
                for i in 0..count {
                    let e = TEX_BOLTS + 0x38 * i;
                    c.bus.write8(e + 3, (r.next_u32() % 3) as u8);
                    c.bus.write16(e + 4, (r.next_u32() % 6) as u16);
                    let s = TEX_STRINGS + 0x40 * (i + 1);
                    tex_name(c, &mut r, s, true);
                    c.bus
                        .write32(e + 0x2c, if r.next_u32() % 7 == 0 { 0 } else { s });
                }
                // The rider's profile: key -> entry index -> flags.
                c.bus.write32(TEX_PROFILE + 0x288, TEX_KEYTAB);
                for k in 0..6u32 {
                    let idx = if r.next_u32() % 4 == 0 {
                        0xffff
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write16(TEX_KEYTAB + 2 * k, idx as u16);
                }
                for i in 0..4u32 {
                    c.bus.write16(
                        TEX_PROFILE + 0x290 + 4 * i + 2,
                        (r.next_u32() & 0x10 | r.next_u32() & 1) as u16,
                    );
                }
                c.bus.write32(oc410_result(8), TEX_PROFILE);
                c.bus.write32(oc410_result(9), count);
                c.bus.write32(oc410_result(10), TEX_BOLTS);
            },
        );
    }

    pub(crate) const DB: u32 = DATA + 0x1000;

    pub(crate) const BOLTS: u32 = DATA + 0x2000;

    pub(crate) const LIB: u32 = DATA + 0x3000;

    pub(crate) const IFACE: u32 = DATA + 0x3100;

    pub(crate) const IFACE_VT: u32 = DATA + 0x3200;

    pub(crate) const STRINGS: u32 = DATA + 0x4000;

    pub(crate) const TABLE2: u32 = DATA + 0x6000;

    pub(crate) const KEY_TABLE: u32 = DATA + 0x7000;

    /// `g_beLibrary[]` (the interface list) and `BENewPlayerInt`'s slot table.
    pub(crate) const BE_LIBRARY: u32 = 0x0053_08a8;

    pub(crate) const PLAYER_SLOTS: u32 = 0x0053_4fe0;

    pub(crate) const RETURN_ZERO: u32 = 0x003a_5ab0;

    /// A record table (`+0x288` = the key table, four entries at `+0x290`
    /// whose halfword at +2 holds the flags) with bit 0x10 / 0x2 at random.
    pub(crate) fn record(c: &mut Cpu, r: &mut Rng, base: u32) {
        c.bus.write32(base + 0x288, KEY_TABLE);
        for i in 0..4 {
            let mut flags = r.next_u32() as u16 & 0xff00;
            if r.next_u32() % 2 == 0 {
                flags |= 0x10;
            }
            if r.next_u32() % 2 == 0 {
                flags |= 0x2;
            }
            c.bus.write16(base + 0x290 + 4 * i + 2, flags);
        }
    }

    /// `Bolts_BuildGameBolts`: per-character bolt lists with random keys,
    /// flags (the 0x80 mark, the 0x100..0x800 name classes) and name
    /// strings; the two rider slots pick a low character, an alternate, none
    /// or an out-of-range one; a second table of 0..3 entries. Real
    /// callees, allocator included.
    #[test]
    fn bolts_build_game_bolts_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014c7a0__generated,
            bolts_build_game_bolts,
            80,
            &|c, seed| {
                let mut r = Rng(0x0014_c7a0 ^ (seed as u64) << 20 | 1);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                crate::game::ai::computer::tests::rbc_test_allocator(c);

                // The library singleton, interface 0 and its method.
                c.bus.write32(GAME_GP - 0x1f04, LIB);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(BE_LIBRARY, IFACE);
                c.bus.write32(IFACE + 4, IFACE_VT);
                c.bus.write16(IFACE_VT + 8, 0);
                c.bus.write32(IFACE_VT + 0xc, RETURN_ZERO);

                // Names: eight strings, 0..12 characters.
                for k in 0..8u32 {
                    let len = r.next_u32() % 13;
                    for i in 0..len {
                        c.bus
                            .write8(STRINGS + 0x20 * k + i, b'a' + (r.next_u32() % 26) as u8);
                    }
                    c.bus.write8(STRINGS + 0x20 * k + len, 0);
                }
                // Key table: key -> entry index (negative = none).
                for k in 0..8u32 {
                    let v = [-1i16, 0, 1, 2, 3, 0, -1, 2][k as usize];
                    c.bus.write16(KEY_TABLE + 2 * k, v as u16);
                }
                // Record tables of both rider slots and of passes 2 and 3.
                for slot in 0..2u32 {
                    for ch in 0..10u32 {
                        record(c, &mut r, BGB_SLOT_RECORDS + slot * 0x9b50 + ch * 0xf88);
                    }
                }
                for ch in 0..10u32 {
                    record(c, &mut r, BGB_PLAIN_RECORDS + ch * 0xf88);
                }
                // The rider slots' characters: mode picks low / alternate /
                // none / out of range.
                for slot in 0..2u32 {
                    let mode = (seed >> (2 * slot)) % 4;
                    let (ch, alt): (i8, i8) = match mode {
                        0 => (r.next_u32() as i8 % 10, 10 + (r.next_u32() % 20) as i8),
                        1 => ((r.next_u32() % 10) as i8, -1),
                        2 => (10 + (r.next_u32() % 20) as i8, -1),
                        _ => (if r.next_u32() % 2 == 0 { 40 } else { -1 }, 35),
                    };
                    c.bus.write8(PLAYER_SLOTS + 0x1c * slot + 0x11, ch as u8);
                    c.bus.write8(PLAYER_SLOTS + 0x1c * slot + 0x12, alt as u8);
                }
                // The bolts: per character 0..2 contiguous, then spares.
                let mut n = 0u32;
                for ch in 0..30u32 {
                    let cnt = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32() % 3
                    };
                    c.bus.write32(DB + 0xa4 + 4 * ch, n);
                    c.bus.write32(DB + 0x2c + 4 * ch, cnt);
                    n += cnt;
                }
                n += r.next_u32() % 3;
                let name = |r: &mut Rng| -> u32 {
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        STRINGS + 0x20 * (r.next_u32() % 8)
                    }
                };
                for i in 0..n {
                    let b = BOLTS + 0x38 * i;
                    c.bus.write16(b + BGB_BOLT_KEY, (r.next_u32() % 8) as u16);
                    let mut flags = r.next_u32() & 0x7f;
                    if r.next_u32() % 3 == 0 {
                        flags |= 0x80;
                    }
                    flags |= [0, 0, 0x100, 0x200, 0x400, 0x800, 0x300][(r.next_u32() % 7) as usize];
                    c.bus.write32(b + BGB_BOLT_FLAGS, flags);
                    for off in [0x14u32, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
                        let p = name(&mut r);
                        c.bus.write32(b + off, p);
                    }
                    c.bus.write32(b + 0x30, r.next_u32());
                    c.bus.write32(b, r.next_u32());
                    c.bus.write32(b + 8, r.next_u32());
                    c.bus.write32(b + 0xc, r.next_u32());
                }
                let n2 = [0u32, 1, 2, 3][(seed % 4) as usize];
                for i in 0..n2 * 2 {
                    c.bus.write32(TABLE2 + 4 * i, r.next_u32());
                }
                // The header: no allocation to free, the arrays, the counts.
                c.bus.write32(DB, 0);
                c.bus.write32(DB + 0x4, BOLTS);
                c.bus.write32(DB + 0xc, TABLE2);
                c.bus.write32(DB + 0x14, 0);
                c.bus.write32(DB + 0x18, 0);
                c.bus.write32(DB + 0x1c, n);
                c.bus.write32(DB + 0x24, n2);

                c.g[A0][0] = DB as u64;
                c.pc = 0x0014_c7a0;
            },
        );
    }

    /// One of the `Bolts` character table getters: index below 8 (with junk
    /// in the high half of a1), counts zero, positive and negative.
    pub(crate) fn oc425_bolts_table_check(
        generated: GuestFn,
        hand: GuestFn,
        entry: u32,
        count: u32,
        index: u32,
        table: u32,
    ) {
        check(generated, hand, 300, &|c, seed| {
            let mut r = Rng(0xb017_0000 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = entry;
            let me = DATA + 0x1000;
            let i = r.next_u32() % 8;
            c.g[A0][0] = sx(me);
            c.g[A1][0] = (r.next_u32() as u64) << 32 | i as u64;
            let n = match seed % 4 {
                0 => 0,
                1 => 1 + r.next_u32() % 5,
                2 => -(1 + (r.next_u32() % 5) as i32) as u32,
                _ => r.next_u32(),
            };
            c.bus.write32(me + 4 * i + count, n);
            c.bus.write32(me + 4 * i + index, r.next_u32() % 100);
            c.bus
                .write32(me + table, DATA + 0x2000 + 8 * (r.next_u32() % 16));
        });
    }

    /// `Bolts_GetCharTable3`.
    #[test]
    fn bolts_get_char_table3_matches() {
        oc425_bolts_table_check(
            ee::s01::f_0014dc10__generated,
            bolts_get_char_table3,
            0x0014_dc10,
            0x2fc,
            0x374,
            0xc,
        );
    }

    /// `Bolts_GetCharTable4`.
    #[test]
    fn bolts_get_char_table4_matches() {
        oc425_bolts_table_check(
            ee::s01::f_0014dc50__generated,
            bolts_get_char_table4,
            0x0014_dc50,
            0x3ec,
            0x464,
            0x10,
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;

    use crate::difftest::fakes::LOG_COUNT;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the fakes below: they act only in a machine whose RAM holds
    /// `OC431_MAGIC` here (other units' tests run the real callees).
    pub(crate) const OC431_CTRL: u32 = DATA + 0xf_f000;

    pub(crate) const OC431_MAGIC: u32 = 0x4f43_3431;

    /// Fixture objects live above the area the fakes' default results point into
    /// (`DATA..DATA + 0x40000`).
    pub(crate) const OC431_BASE: u32 = DATA + 0x4_0000;

    /// Make fake call `n` (counted from 0 over all fakes of the run) return `v`
    /// in v0 (also 0 and negative values, which `LOG_RESULTS` cannot do).
    pub(crate) fn oc431_force(c: &mut Cpu, n: u32, v: u32) {
        c.bus.write32(OC431_CTRL + 0x20 + 8 * n, 1);
        c.bus.write32(OC431_CTRL + 0x24 + 8 * n, v);
    }

    /// Effects some fakes have on the caller's data (what the real callee
    /// would change that the caller then reads).
    pub(crate) fn oc431_post(c: &mut Cpu, addr: u32, a0: u32) {
        match addr {
            // InputParser_NextToken: the parser's "more input" word (+0x98).
            0x0032_1af0 => {
                let v = c.bus.read32(OC431_CTRL + 4);
                c.bus.write32(a0.wrapping_add(0x98), v);
            }
            // InputParser_ParseStatement consumes the input.
            0x0032_30e0 => c.bus.write32(a0.wrapping_add(0x98), 0),
            // EAFile_QueueOpen: the file does not open.
            0x003d_d4e8 => c.g[V0][0] = 0,
            _ => {}
        }
    }

    /// A fake callee: log the call like `fakes::fake`, then apply forced
    /// results and the fake's data effects (unless the call unwound).
    pub(crate) fn oc431_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let a0 = c.g[A0][0] as u32;
        let n = c.bus.read32(LOG_COUNT);
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        if c.bus.read32(OC431_CTRL + 0x20 + 8 * n) != 0 {
            c.g[V0][0] = sx(c.bus.read32(OC431_CTRL + 0x24 + 8 * n));
        }
        oc431_post(c, addr, a0);
    }

    /// Calls to addresses outside the function table (vtable slots pointing
    /// at `FAKE`) are fake callees too.
    pub(crate) fn oc431_miss(c: &mut Cpu, target: u32) {
        oc431_fake_call(c, target)
    }

    macro_rules! oc431_fake {
        ($fake:ident, $prev:ident, $addr:expr, $generated:path) => {
            pub(crate) static $prev: AtomicUsize = AtomicUsize::new(0);
            pub(crate) fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC431_CTRL) != OC431_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                oc431_fake_call(c, $addr)
            }
        };
    }

    oc431_fake!(
        oc431_fake_00317d70,
        OC431_PREV_00317D70,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );

    oc431_fake!(
        oc431_fake_00317e30,
        OC431_PREV_00317E30,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );

    oc431_fake!(
        oc431_fake_00317e98,
        OC431_PREV_00317E98,
        0x0031_7e98,
        ee::s10::f_00317e98__generated
    );

    oc431_fake!(
        oc431_fake_001172e0,
        OC431_PREV_001172E0,
        0x0011_72e0,
        ee::s00::f_001172e0__generated
    );

    oc431_fake!(
        oc431_fake_00117540,
        OC431_PREV_00117540,
        0x0011_7540,
        ee::s00::f_00117540__generated
    );

    oc431_fake!(
        oc431_fake_001033f8,
        OC431_PREV_001033F8,
        0x0010_33f8,
        ee::s00::f_001033f8__generated
    );

    oc431_fake!(
        oc431_fake_00314aa0,
        OC431_PREV_00314AA0,
        0x0031_4aa0,
        ee::s10::f_00314aa0__generated
    );

    oc431_fake!(
        oc431_fake_00321af0,
        OC431_PREV_00321AF0,
        0x0032_1af0,
        ee::s10::f_00321af0__generated
    );

    oc431_fake!(
        oc431_fake_003230e0,
        OC431_PREV_003230E0,
        0x0032_30e0,
        ee::s10::f_003230e0__generated
    );

    oc431_fake!(
        oc431_fake_00224da0,
        OC431_PREV_00224DA0,
        0x0022_4da0,
        ee::s05::f_00224da0__generated
    );

    oc431_fake!(
        oc431_fake_00224df0,
        OC431_PREV_00224DF0,
        0x0022_4df0,
        ee::s05::f_00224df0__generated
    );

    oc431_fake!(
        oc431_fake_0022d478,
        OC431_PREV_0022D478,
        0x0022_d478,
        ee::s05::f_0022d478__generated
    );

    oc431_fake!(
        oc431_fake_0022d598,
        OC431_PREV_0022D598,
        0x0022_d598,
        ee::s05::f_0022d598__generated
    );

    oc431_fake!(
        oc431_fake_0038ac50,
        OC431_PREV_0038AC50,
        0x0038_ac50,
        ee::s12::f_0038ac50__generated
    );

    oc431_fake!(
        oc431_fake_0015cd80,
        OC431_PREV_0015CD80,
        0x0015_cd80,
        ee::s01::f_0015cd80__generated
    );

    oc431_fake!(
        oc431_fake_0015d698,
        OC431_PREV_0015D698,
        0x0015_d698,
        ee::s01::f_0015d698__generated
    );

    oc431_fake!(
        oc431_fake_001619a8,
        OC431_PREV_001619A8,
        0x0016_19a8,
        ee::s01::f_001619a8__generated
    );

    oc431_fake!(
        oc431_fake_0015dfd8,
        OC431_PREV_0015DFD8,
        0x0015_dfd8,
        ee::s01::f_0015dfd8__generated
    );

    oc431_fake!(
        oc431_fake_00168fe8,
        OC431_PREV_00168FE8,
        0x0016_8fe8,
        ee::s02::f_00168fe8__generated
    );

    oc431_fake!(
        oc431_fake_00176d68,
        OC431_PREV_00176D68,
        0x0017_6d68,
        ee::s02::f_00176d68__generated
    );

    oc431_fake!(
        oc431_fake_0027d2e8,
        OC431_PREV_0027D2E8,
        0x0027_d2e8,
        ee::s07::f_0027d2e8__generated
    );

    oc431_fake!(
        oc431_fake_0027d170,
        OC431_PREV_0027D170,
        0x0027_d170,
        ee::s07::f_0027d170__generated
    );

    oc431_fake!(
        oc431_fake_0027d688,
        OC431_PREV_0027D688,
        0x0027_d688,
        ee::s07::f_0027d688__generated
    );

    oc431_fake!(
        oc431_fake_00281a50,
        OC431_PREV_00281A50,
        0x0028_1a50,
        ee::s07::f_00281a50__generated
    );

    oc431_fake!(
        oc431_fake_00283440,
        OC431_PREV_00283440,
        0x0028_3440,
        ee::s07::f_00283440__generated
    );

    oc431_fake!(
        oc431_fake_00283200,
        OC431_PREV_00283200,
        0x0028_3200,
        ee::s07::f_00283200__generated
    );

    oc431_fake!(
        oc431_fake_00283088,
        OC431_PREV_00283088,
        0x0028_3088,
        ee::s07::f_00283088__generated
    );

    oc431_fake!(
        oc431_fake_00281e80,
        OC431_PREV_00281E80,
        0x0028_1e80,
        ee::s07::f_00281e80__generated
    );

    oc431_fake!(
        oc431_fake_00283798,
        OC431_PREV_00283798,
        0x0028_3798,
        ee::s07::f_00283798__generated
    );

    oc431_fake!(
        oc431_fake_00275828,
        OC431_PREV_00275828,
        0x0027_5828,
        ee::s07::f_00275828__generated
    );

    oc431_fake!(
        oc431_fake_0027bfd0,
        OC431_PREV_0027BFD0,
        0x0027_bfd0,
        ee::s07::f_0027bfd0__generated
    );

    oc431_fake!(
        oc431_fake_002805b8,
        OC431_PREV_002805B8,
        0x0028_05b8,
        ee::s07::f_002805b8__generated
    );

    oc431_fake!(
        oc431_fake_00283450,
        OC431_PREV_00283450,
        0x0028_3450,
        ee::s07::f_00283450__generated
    );

    oc431_fake!(
        oc431_fake_00277f08,
        OC431_PREV_00277F08,
        0x0027_7f08,
        ee::s07::f_00277f08__generated
    );

    oc431_fake!(
        oc431_fake_0016c098,
        OC431_PREV_0016C098,
        0x0016_c098,
        ee::s02::f_0016c098__generated
    );

    oc431_fake!(
        oc431_fake_0016c628,
        OC431_PREV_0016C628,
        0x0016_c628,
        ee::s02::f_0016c628__generated
    );

    oc431_fake!(
        oc431_fake_0015dc18,
        OC431_PREV_0015DC18,
        0x0015_dc18,
        ee::s01::f_0015dc18__generated
    );

    oc431_fake!(
        oc431_fake_00328f28,
        OC431_PREV_00328F28,
        0x0032_8f28,
        ee::s10::f_00328f28__generated
    );

    oc431_fake!(
        oc431_fake_003284b8,
        OC431_PREV_003284B8,
        0x0032_84b8,
        ee::s10::f_003284b8__generated
    );

    oc431_fake!(
        oc431_fake_00317e50,
        OC431_PREV_00317E50,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );

    oc431_fake!(
        oc431_fake_0014c2b0,
        OC431_PREV_0014C2B0,
        0x0014_c2b0,
        ee::s01::f_0014c2b0__generated
    );

    oc431_fake!(
        oc431_fake_0014c320,
        OC431_PREV_0014C320,
        0x0014_c320,
        ee::s01::f_0014c320__generated
    );

    oc431_fake!(
        oc431_fake_0014c3c8,
        OC431_PREV_0014C3C8,
        0x0014_c3c8,
        ee::s01::f_0014c3c8__generated
    );

    oc431_fake!(
        oc431_fake_0014c488,
        OC431_PREV_0014C488,
        0x0014_c488,
        ee::s01::f_0014c488__generated
    );

    oc431_fake!(
        oc431_fake_0014c7a0,
        OC431_PREV_0014C7A0,
        0x0014_c7a0,
        ee::s01::f_0014c7a0__generated
    );

    oc431_fake!(
        oc431_fake_003e3208,
        OC431_PREV_003E3208,
        0x003e_3208,
        ee::s14::f_003e3208__generated
    );

    oc431_fake!(
        oc431_fake_003e32f8,
        OC431_PREV_003E32F8,
        0x003e_32f8,
        ee::s14::f_003e32f8__generated
    );

    oc431_fake!(
        oc431_fake_003e34e8,
        OC431_PREV_003E34E8,
        0x003e_34e8,
        ee::s14::f_003e34e8__generated
    );

    oc431_fake!(
        oc431_fake_003ddfa8,
        OC431_PREV_003DDFA8,
        0x003d_dfa8,
        ee::s14::f_003ddfa8__generated
    );

    oc431_fake!(
        oc431_fake_003dd4e8,
        OC431_PREV_003DD4E8,
        0x003d_d4e8,
        ee::s14::f_003dd4e8__generated
    );

    oc431_fake!(
        oc431_fake_003a6e98,
        OC431_PREV_003A6E98,
        0x003a_6e98,
        ee::s13::f_003a6e98__generated
    );

    oc431_fake!(
        oc431_fake_003a7a20,
        OC431_PREV_003A7A20,
        0x003a_7a20,
        ee::s13::f_003a7a20__generated
    );

    oc431_fake!(
        oc431_fake_003a7c88,
        OC431_PREV_003A7C88,
        0x003a_7c88,
        ee::s13::f_003a7c88__generated
    );

    oc431_fake!(
        oc431_fake_003a82c8,
        OC431_PREV_003A82C8,
        0x003a_82c8,
        ee::s13::f_003a82c8__generated
    );

    oc431_fake!(
        oc431_fake_003aad98,
        OC431_PREV_003AAD98,
        0x003a_ad98,
        ee::s13::f_003aad98__generated
    );

    oc431_fake!(
        oc431_fake_00318260,
        OC431_PREV_00318260,
        0x0031_8260,
        ee::s10::f_00318260__generated
    );

    oc431_fake!(
        oc431_fake_003182d8,
        OC431_PREV_003182D8,
        0x0031_82d8,
        ee::s10::f_003182d8__generated
    );

    oc431_fake!(
        oc431_fake_00318630,
        OC431_PREV_00318630,
        0x0031_8630,
        ee::s10::f_00318630__generated
    );

    oc431_fake!(
        oc431_fake_003186d0,
        OC431_PREV_003186D0,
        0x0031_86d0,
        ee::s10::f_003186d0__generated
    );

    oc431_fake!(
        oc431_fake_00326150,
        OC431_PREV_00326150,
        0x0032_6150,
        ee::s10::f_00326150__generated
    );

    oc431_fake!(
        oc431_fake_00326430,
        OC431_PREV_00326430,
        0x0032_6430,
        ee::s10::f_00326430__generated
    );

    oc431_fake!(
        oc431_fake_00326478,
        OC431_PREV_00326478,
        0x0032_6478,
        ee::s10::f_00326478__generated
    );

    oc431_fake!(
        oc431_fake_00417828,
        OC431_PREV_00417828,
        0x0041_7828,
        ee::s15::f_00417828__generated
    );

    pub(crate) fn oc431_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc431_fake_00317d70,
            &OC431_PREV_00317D70,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc431_fake_00317e30,
            &OC431_PREV_00317E30,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc431_fake_00317e98,
            &OC431_PREV_00317E98,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_001172E0),
            oc431_fake_001172e0,
            &OC431_PREV_001172E0,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_00117540),
            oc431_fake_00117540,
            &OC431_PREV_00117540,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_001033F8),
            oc431_fake_001033f8,
            &OC431_PREV_001033F8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00314AA0),
            oc431_fake_00314aa0,
            &OC431_PREV_00314AA0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00321AF0),
            oc431_fake_00321af0,
            &OC431_PREV_00321AF0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003230E0),
            oc431_fake_003230e0,
            &OC431_PREV_003230E0,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00224DA0),
            oc431_fake_00224da0,
            &OC431_PREV_00224DA0,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00224DF0),
            oc431_fake_00224df0,
            &OC431_PREV_00224DF0,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D478),
            oc431_fake_0022d478,
            &OC431_PREV_0022D478,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D598),
            oc431_fake_0022d598,
            &OC431_PREV_0022D598,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038AC50),
            oc431_fake_0038ac50,
            &OC431_PREV_0038AC50,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015CD80),
            oc431_fake_0015cd80,
            &OC431_PREV_0015CD80,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015D698),
            oc431_fake_0015d698,
            &OC431_PREV_0015D698,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001619A8),
            oc431_fake_001619a8,
            &OC431_PREV_001619A8,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015DFD8),
            oc431_fake_0015dfd8,
            &OC431_PREV_0015DFD8,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00168FE8),
            oc431_fake_00168fe8,
            &OC431_PREV_00168FE8,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00176D68),
            oc431_fake_00176d68,
            &OC431_PREV_00176D68,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027D2E8),
            oc431_fake_0027d2e8,
            &OC431_PREV_0027D2E8,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027D170),
            oc431_fake_0027d170,
            &OC431_PREV_0027D170,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027D688),
            oc431_fake_0027d688,
            &OC431_PREV_0027D688,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00281A50),
            oc431_fake_00281a50,
            &OC431_PREV_00281A50,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283440),
            oc431_fake_00283440,
            &OC431_PREV_00283440,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283200),
            oc431_fake_00283200,
            &OC431_PREV_00283200,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283088),
            oc431_fake_00283088,
            &OC431_PREV_00283088,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00281E80),
            oc431_fake_00281e80,
            &OC431_PREV_00281E80,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283798),
            oc431_fake_00283798,
            &OC431_PREV_00283798,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00275828),
            oc431_fake_00275828,
            &OC431_PREV_00275828,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027BFD0),
            oc431_fake_0027bfd0,
            &OC431_PREV_0027BFD0,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002805B8),
            oc431_fake_002805b8,
            &OC431_PREV_002805B8,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283450),
            oc431_fake_00283450,
            &OC431_PREV_00283450,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277F08),
            oc431_fake_00277f08,
            &OC431_PREV_00277F08,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C098),
            oc431_fake_0016c098,
            &OC431_PREV_0016C098,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C628),
            oc431_fake_0016c628,
            &OC431_PREV_0016C628,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015DC18),
            oc431_fake_0015dc18,
            &OC431_PREV_0015DC18,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328F28),
            oc431_fake_00328f28,
            &OC431_PREV_00328F28,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003284B8),
            oc431_fake_003284b8,
            &OC431_PREV_003284B8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc431_fake_00317e50,
            &OC431_PREV_00317E50,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C2B0),
            oc431_fake_0014c2b0,
            &OC431_PREV_0014C2B0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C320),
            oc431_fake_0014c320,
            &OC431_PREV_0014C320,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C3C8),
            oc431_fake_0014c3c8,
            &OC431_PREV_0014C3C8,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C488),
            oc431_fake_0014c488,
            &OC431_PREV_0014C488,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C7A0),
            oc431_fake_0014c7a0,
            &OC431_PREV_0014C7A0,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E3208),
            oc431_fake_003e3208,
            &OC431_PREV_003E3208,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E32F8),
            oc431_fake_003e32f8,
            &OC431_PREV_003E32F8,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E34E8),
            oc431_fake_003e34e8,
            &OC431_PREV_003E34E8,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DDFA8),
            oc431_fake_003ddfa8,
            &OC431_PREV_003DDFA8,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD4E8),
            oc431_fake_003dd4e8,
            &OC431_PREV_003DD4E8,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A6E98),
            oc431_fake_003a6e98,
            &OC431_PREV_003A6E98,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A7A20),
            oc431_fake_003a7a20,
            &OC431_PREV_003A7A20,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A7C88),
            oc431_fake_003a7c88,
            &OC431_PREV_003A7C88,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A82C8),
            oc431_fake_003a82c8,
            &OC431_PREV_003A82C8,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003AAD98),
            oc431_fake_003aad98,
            &OC431_PREV_003AAD98,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            oc431_fake_00318260,
            &OC431_PREV_00318260,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003182D8),
            oc431_fake_003182d8,
            &OC431_PREV_003182D8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318630),
            oc431_fake_00318630,
            &OC431_PREV_00318630,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003186D0),
            oc431_fake_003186d0,
            &OC431_PREV_003186D0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00326150),
            oc431_fake_00326150,
            &OC431_PREV_00326150,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00326430),
            oc431_fake_00326430,
            &OC431_PREV_00326430,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00326478),
            oc431_fake_00326478,
            &OC431_PREV_00326478,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc431_fake_00417828,
            &OC431_PREV_00417828,
        );
    }

    /// A test machine: scrambled registers, game-like gp, fake callees
    /// armed, `pc` at the function's entry, unwinding at one of `calls` fake
    /// calls in a third of the seeds.
    pub(crate) fn oc431_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc431_install();
        let r = machine(c, seed, salt, calls);
        c.hooks.miss = oc431_miss;
        c.pc = entry;
        c.bus.write32(OC431_CTRL, OC431_MAGIC);
        r
    }

    // -----------------------------------------------------------------------
    // Leaves.

    /// Table counts: the word at `self + character * 4 + offset`, for
    /// characters from -4 to 11 over a random library.
    pub(crate) fn oc431_count_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = machine(c, seed, entry as u64, 0);
        fill(c, &mut r, OC431_BASE, 0x400);
        c.g[A0][0] = OC431_BASE as u64 + 0x40;
        c.g[A1][0] = ((seed % 16) as i64 - 4) as u64;
        c.pc = entry;
    }

    #[test]
    fn bolts_get_char_table4_count_matches() {
        check(
            ee::s01::f_0014dc40__generated,
            bolts_get_char_table4_count,
            200,
            &|c, seed| oc431_count_setup(c, seed, 0x0014_dc40),
        );
    }

    #[test]
    fn bolts_get_char_table3_count_matches() {
        check(
            ee::s01::f_0014dc00__generated,
            bolts_get_char_table3_count,
            200,
            &|c, seed| oc431_count_setup(c, seed, 0x0014_dc00),
        );
    }

    /// First profile record of the bolts library (stride 0x9b50 per profile,
    /// 0xf88 per character).
    pub(crate) const OC431_PROFILES: u32 = 0x004a_6ca8;

    /// `Bolts_PrepareForGame`: three profiles x ten characters; per character
    /// the flag word (+2 of each 4-byte entry) of unlocked entries (bit 2)
    /// gains bit 4, as do the entries a bolt rule requires (rule rows give an
    /// entry index and an optional second one, -1 for none). The index
    /// rebuilds at the end are fakes.
    #[test]
    fn bolts_prepare_for_game_matches() {
        oc431_install();
        covered(
            ee::s01::f_0014d068__generated,
            bolts_prepare_for_game,
            100,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x14d068, 5, 0x0014_d068);
                let (lib, rules) = (OC431_BASE + 0x100, OC431_BASE + 0x800);
                c.g[A0][0] = lib as u64;
                c.bus.write32(lib + 0xc, rules);
                for ch in 0..10u32 {
                    c.bus.write32(lib + 0x2fc + 4 * ch, r.next_u32() % 4);
                    c.bus.write32(lib + 0x374 + 4 * ch, r.next_u32() % 3);
                }
                for k in 0..3u32 {
                    let a = (r.next_u32() % 6) as i32 - 1;
                    let b = (r.next_u32() % 6) as i32 - 1;
                    c.bus.write16(rules + 8 * k + 4, a as u16);
                    c.bus.write16(rules + 8 * k + 6, b as u16);
                }
                for p in 0..3u32 {
                    for ch in 0..10u32 {
                        let rec = OC431_PROFILES + p * 0x9b50 + ch * 0xf88;
                        let ids = OC431_BASE + 0x1000 + (p * 10 + ch) * 0x40;
                        c.bus.write32(rec + 0x28c, r.next_u32() % 5);
                        c.bus.write32(rec + 0x288, ids);
                        for e in 0..8u32 {
                            c.bus.write32(rec + 0x290 + 4 * e, r.next_u32());
                        }
                        for k in 0..16u32 {
                            let v = (r.next_u32() % 7) as i32 - 1;
                            c.bus.write16(ids + 2 * k, v as u16);
                        }
                    }
                }
            },
        );
    }

    /// Fixture of the file loaders: the file queue never opens the file
    /// (`EAFile_QueueOpen` fake returns 0), so every load returns 0 and its
    /// buffer is the fixture's.
    pub(crate) const OC431_PATH: u32 = OC431_BASE + 0x100;

    /// `Bolts_ReadFile`: the size query (real path parsing over the platform
    /// file fakes), the buffer (fake allocation forced to a fixture file image
    /// holding 0 to 2 entries and the three tables), and the pointer
    /// relocation of every entry's strings. The load itself does nothing.
    #[test]
    fn bolts_read_file_matches() {
        oc431_install();
        covered(
            ee::s01::f_0014be70__generated,
            bolts_read_file,
            100,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x14be70, 8, 0x0014_be70);
                let this = OC431_BASE + 0x200;
                let image = OC431_BASE + 0x1000;
                c.g[A0][0] = this as u64;
                c.bus.write32(GAME_GP - 0x1f24, OC431_PATH);
                for (i, b) in b"data.bin\0".iter().enumerate() {
                    c.bus.write8(OC431_PATH + i as u32, *b);
                }
                // The size query makes 3 fake calls (open, size, close), then
                // the allocation is call 3.
                oc431_force(c, 3, image);
                // File image: flags, n0, n0 x 0x38-byte entries, then three
                // (count, rows) tables of 12-, 8- and 8-byte rows, a 4-byte word.
                let n0 = r.next_u32() % 3;
                let mut at = image;
                let flags = r.next_u32();
                c.bus.write32(at, flags);
                c.bus.write32(at + 4, n0);
                at += 8;
                for _ in 0..n0 {
                    for k in 0..14u32 {
                        let v = if r.next_u32() % 3 == 0 {
                            0
                        } else {
                            1 + r.next_u32() % 0x40
                        };
                        c.bus.write32(at + 4 * k, v);
                    }
                    at += 0x38;
                }
                for row in [12u32, 8, 8] {
                    let n = r.next_u32() % 3;
                    c.bus.write32(at, n);
                    at += 4;
                    for k in 0..n * row / 4 {
                        c.bus.write32(at + 4 * k, r.next_u32());
                    }
                    at += n * row;
                }
                c.bus.write32(at, r.next_u32());
            },
        );
    }

    pub(crate) use crate::game::world::world::tests::w_2;

    /// `BEBoltInt_RestoreAllEquipFlags`: 3 profiles x 10 characters, each with
    /// a count (0, negative or a few) of flag entries.
    #[test]
    fn be_bolt_int_restore_all_equip_flags_matches() {
        check(
            ee::s01::f_0014af10__generated,
            be_bolt_int_restore_all_equip_flags_2,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14af10, 0);
                for p in 0..3u32 {
                    for ch in 0..10u32 {
                        let rec = 0x4a_6ca8 + p * 0x9b50 + ch * 0xf88;
                        let count = match r.next_u32() % 8 {
                            0 => 0,
                            1 => 0xffff_fffe,
                            n => n % 5,
                        };
                        w_2(c, rec + 0x28c, count);
                        for i in 0..5u32 {
                            let flags = r.next_u32();
                            w_2(c, rec + 0x290 + 4 * i, flags);
                        }
                    }
                }
            },
        );
    }

    pub(crate) use crate::game::camera::trigger::cameratriggerman::tests::oc438_install_fakes;
    pub(crate) use crate::game::camera::trigger::cameratriggerman::tests::oc438_machine;

    /// `Bolts_PrepareGameDB`: forwards to `Bolts_PrepareForGame` (faked).
    #[test]
    fn bolts_prepare_game_db_matches() {
        oc438_install_fakes();
        covered(
            ee::s01::f_0014bd98__generated,
            bolts_prepare_game_db,
            200,
            &|c, seed| {
                oc438_machine(c, seed, 0x0014_bd98, 1);
            },
        );
    }

    /// `Bolts_LoadGlobalDB`: forwards to `Bolts_LoadDB` (faked).
    #[test]
    fn bolts_load_global_db_matches() {
        oc438_install_fakes();
        covered(
            ee::s01::f_0014bd78__generated,
            bolts_load_global_db,
            200,
            &|c, seed| {
                oc438_machine(c, seed, 0x0014_bd78, 1);
            },
        );
    }

    /// Gate word for the getter fakes, and the interface objects they return
    /// (one slot per getter at `+0x10 + 4k`). Above the test's data.
    pub(crate) const DESER_TEST_CTRL: u32 = DATA + 0x30_0000;

    pub(crate) const DESER_TEST_MAGIC: u32 = 0x4f43_4534;

    /// A getter fake: while the gate word is set it logs the call and returns
    /// the interface object of slot `k`; otherwise it runs the body it replaced.
    pub(crate) fn deser_getter(
        c: &mut Cpu,
        target: u32,
        k: u32,
        prev: &AtomicUsize,
        generated: fn(&mut Cpu),
    ) {
        if c.bus.read32(DESER_TEST_CTRL) != DESER_TEST_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        let ret_addr = c.g[RA][0] as u32;
        fakes::fake(c, target, ret_addr);
        c.g[V0][0] = c.bus.read32(DESER_TEST_CTRL + 0x10 + 4 * k) as u64;
    }

    pub(crate) static DESER_PREV_RACE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static DESER_PREV_PLAYER: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static DESER_PREV_BAGT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static DESER_PREV_MISSION: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static DESER_PREV_SCORE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static DESER_PREV_CHAR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn deser_fake_race(c: &mut Cpu) {
        deser_getter(
            c,
            0x0014_48d8,
            0,
            &DESER_PREV_RACE,
            ee::s01::f_001448d8__generated,
        )
    }

    pub(crate) fn deser_fake_player(c: &mut Cpu) {
        deser_getter(
            c,
            0x0014_5970,
            1,
            &DESER_PREV_PLAYER,
            ee::s01::f_00145970__generated,
        )
    }

    pub(crate) fn deser_fake_bagt(c: &mut Cpu) {
        deser_getter(
            c,
            0x0014_f900,
            2,
            &DESER_PREV_BAGT,
            ee::s01::f_0014f900__generated,
        )
    }

    pub(crate) fn deser_fake_mission(c: &mut Cpu) {
        deser_getter(
            c,
            0x0015_32a0,
            3,
            &DESER_PREV_MISSION,
            ee::s01::f_001532a0__generated,
        )
    }

    pub(crate) fn deser_fake_score(c: &mut Cpu) {
        deser_getter(
            c,
            0x0015_4948,
            4,
            &DESER_PREV_SCORE,
            ee::s01::f_00154948__generated,
        )
    }

    pub(crate) fn deser_fake_char(c: &mut Cpu) {
        deser_getter(
            c,
            0x0014_7f78,
            5,
            &DESER_PREV_CHAR,
            ee::s01::f_00147f78__generated,
        )
    }

    pub(crate) fn deser_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001448D8),
            deser_fake_race,
            &DESER_PREV_RACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00145970),
            deser_fake_player,
            &DESER_PREV_PLAYER,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014F900),
            deser_fake_bagt,
            &DESER_PREV_BAGT,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001532A0),
            deser_fake_mission,
            &DESER_PREV_MISSION,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00154948),
            deser_fake_score,
            &DESER_PREV_SCORE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147F78),
            deser_fake_char,
            &DESER_PREV_CHAR,
        );
    }

    /// A save image of 0xbc00 bytes at `DATA + 0x1000` (+4 on odd seeds, so
    /// the copies alternate between aligned and unaligned), with the two
    /// character ids the load reads (playerDB[0] in 0..3, playerDB[1] in
    /// -1..3, the negative one skipping the second profile block), and six
    /// interface objects whose vtables point at fake methods.
    pub(crate) fn deser_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let buf = DATA + 0x1000 + if seed & 1 == 1 { 4 } else { 0 };
        fill(c, r, buf, 0x2f00);
        c.bus.write8(buf + 0x4c + 0x11, (seed % 4) as u8);
        c.bus
            .write8(buf + 0x4c + 0x2d, (((seed / 4) % 5) as i32 - 1) as u8);
        c.bus.write32(DESER_TEST_CTRL, DESER_TEST_MAGIC);
        for k in 0..6u32 {
            let obj = DATA + 0x30_1000 + 0x100 * k;
            let vt = DATA + 0x30_8000 + 0x40 * k;
            c.bus.write32(obj + 0xc, vt);
            c.bus.write16(vt + 0x10, (r.next_u32() as i16 & !3) as u16);
            c.bus.write32(vt + 0x14, fakes::FAKE + 0x10 * k);
            c.bus.write32(DESER_TEST_CTRL + 0x10 + 4 * k, obj);
        }
        c.g[A0][0] = sx(buf);
    }

    /// `BELibrary_DeserializeSave`: aligned and unaligned copies of every
    /// block, both second-profile cases, LO/HI too; the getters and the
    /// vtable methods are fake, the copies and the register dance are real.
    #[test]
    fn be_library_deserialize_save_matches() {
        deser_install_fakes();
        check_lohi(
            ee::s01::f_0014a5e0__generated,
            be_library_deserialize_save,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_a5e0, 12);
                deser_fixture(c, &mut r, seed);
                c.pc = 0x0014_a5e0;
            },
        );
    }

    #[test]
    fn be_init_rider_char_profile_bolt_map_matches() {
        check(
            ssx3_ee::s01::f_0014acf8__generated,
            be_init_rider_char_profile_bolt_map,
            100,
            &|c, seed| {
                let mut r = Rng(0x14ac_f800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A2][0] = (seed % 4) as u64;
            },
        );
    }

    /// Gate for the fake of `BE_SetCharProfileBoltEquipped` (the installed
    /// slot): the fake runs only while this word holds `SET_MAGIC`; the next
    /// word is its result (bit 0).
    pub(crate) const SET_GATE: u32 = DATA + 0x20_0000;

    pub(crate) const SET_MAGIC: u32 = 0x5e7b_0457;

    /// The fake of `BE_SetCharProfileBoltEquipped`: a forced result, gate off
    /// runs the body the slot held.
    pub(crate) fn oc457_set_equipped_fake(c: &mut Cpu) {
        if c.bus.read32(SET_GATE) != SET_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(oc457_set_equipped_fake),
                ee::s01::f_00151c90__generated,
            );
        }
        c.g[V0][0] = (c.bus.read32(SET_GATE + 4) & 1) as u64;
        c.cycles += 9;
        abi::ret(c);
    }

    /// Hooks the fake into the installed slot (once per process).
    pub(crate) fn oc457_install_fakes() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_00151C90),
            oc457_set_equipped_fake,
            crate::difftest::fakes::prev_cell(oc457_set_equipped_fake),
        );
    }

    /// The character record of (profile, character) in the profile library.
    pub(crate) fn oc457_record(profile: u32, character: u32) -> u32 {
        BOLT_PROFILE_LIB + profile * BOLT_PROFILE_STRIDE + character * BOLT_CHARACTER_STRIDE
    }

    /// A bolt database entry for `kind` with `count` records at a fixed list
    /// (the list's flag bytes at +1 are random).
    pub(crate) fn oc457_bolt_db(c: &mut Cpu, r: &mut Rng, kind: u32, count: u32) {
        let entry = BOLT_DB_BASE + kind * 4;
        let list = DATA + 0x3000;
        c.bus.write32(BOLT_DB_BASE + 4, list);
        c.bus.write32(entry + 0x2c, count);
        c.bus.write32(entry + 0xa4, 0);
        for i in 0..count {
            c.bus.write8(list + 0x38 * i + 1, (r.next_u32() & 1) as u8);
        }
    }

    #[test]
    fn bolt_int_get_bolt_state_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014ada8__generated,
            bolt_int_get_bolt_state,
            200,
            &|c, seed| {
                let mut r = Rng(0x1ada_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = if seed % 3 == 0 {
                    r.next_u32() as u64
                } else {
                    (seed % 4) as u64
                };
                c.g[A2][0] = if seed % 4 == 0 {
                    ((r.next_u32() as u64) << 32) | r.next_u32() as u64
                } else {
                    (seed % 3) as u64
                };
                c.g[A3][0] = (seed % 9) as u64;
            },
        );
    }

    #[test]
    fn bolt_int_get_rider_bolt_state_matches() {
        check(
            ssx3_ee::s01::f_0014add8__generated,
            bolt_int_get_rider_bolt_state,
            80,
            &|c, seed| {
                let _ = machine(c, seed, 0xadd8, 0);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 9) as u64;
            },
        );
    }

    #[test]
    fn bolt_int_set_bolt_equipped_matches() {
        oc457_install_fakes();
        check_lohi(
            ssx3_ee::s01::f_0014afb0__generated,
            bolt_int_set_bolt_equipped,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0xafb0, 0);
                let profile = seed % 2;
                let character = seed % 3;
                let slot = seed % 5;
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = profile as u64;
                c.g[A2][0] = character as u64;
                c.g[A3][0] = slot as u64;
                c.g[T0][0] = (seed % 2) as u64;
                c.bus.write32(SET_GATE, SET_MAGIC);
                c.bus
                    .write32(SET_GATE + 4, if seed % 4 == 0 { 0 } else { 1 });
                let rec = oc457_record(profile, character);
                c.bus.write32(rec + BOLT_LOOKUP_PTR, DATA + 0x2000);
                let index: i16 = if seed % 7 == 0 { -1 } else { (seed % 4) as i16 };
                c.bus.write16(DATA + 0x2000 + 2 * slot, index as u16);
                fill(c, &mut r, rec + BOLT_TABLE, 0x40);
            },
        );
    }

    #[test]
    fn bolt_int_char_record_call_52368_matches() {
        check(
            ssx3_ee::s01::f_0014b1b0__generated,
            bolt_int_char_record_call_52368,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xb1b0, 0);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn bolt_int_count_available_bolts_matches() {
        check(
            ssx3_ee::s01::f_0014b2f8__generated,
            bolt_int_count_available_bolts,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0xb2f8, 0);
                let kind = seed % 3;
                c.g[A1][0] = kind as u64;
                fill(c, &mut r, DATA + 0x3000, 0x40);
                oc457_bolt_db(c, &mut r, kind, seed % 5);
            },
        );
    }

    #[test]
    fn bolt_int_get_available_bolts_matches() {
        check(
            ssx3_ee::s01::f_0014b378__generated,
            bolt_int_get_available_bolts,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0xb378, 0);
                let kind = seed % 3;
                c.g[A1][0] = kind as u64;
                c.g[A2][0] = (DATA + 0x5000) as u64;
                fill(c, &mut r, DATA + 0x3000, 0x40);
                fill(c, &mut r, DATA + 0x5000, 0x80);
                oc457_bolt_db(c, &mut r, kind, seed % 5);
            },
        );
    }

    #[test]
    fn bolt_int_dispatch_category_matches() {
        const HANDLERS: [u32; 10] = [
            0x0014_b724,
            0x0014_b730,
            0x0014_b73c,
            0x0014_b748,
            0x0014_b754,
            0x0014_b760,
            0x0014_b76c,
            0x0014_b778,
            0x0014_b784,
            0x0014_b790,
        ];
        check(
            ssx3_ee::s01::f_0014b700__generated,
            bolt_int_dispatch_category,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0xb700, 0);
                // The generated body checks `pc` for its resume entries.
                c.pc = 0x0014_b700;
                c.g[A1][0] = if seed % 13 == 0 {
                    u64::MAX
                } else {
                    (seed % 12) as u64
                };
                for (i, h) in HANDLERS.iter().enumerate() {
                    c.bus.write32(0x0045_a5c0 + 4 * i as u32, *h);
                }
                if seed % 5 == 0 {
                    c.bus.write32(0x0045_a5c0 + 4 * (seed % 10), 0x0014_b7a4);
                }
            },
        );
    }

    #[test]
    fn bolt_int_collect_rider_equipped_texture_names_matches() {
        check(
            ssx3_ee::s01::f_0014b920__generated,
            bolt_int_collect_rider_equipped_texture_names,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0xb920, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn bolt_int_get_rider_bolt_texture_name_matches() {
        check(
            ssx3_ee::s01::f_0014bba0__generated,
            bolt_int_get_rider_bolt_texture_name,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0xbba0, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
            },
        );
    }

    /// Bolt-list fixture for the two equipped-name collectors: the bolt
    /// database (`Bolts_GetDB`, 0x4a6750) with `count` bolts for character
    /// `ch`, the profile record of (`profile`, `ch`) with its keytab and
    /// flags, and a few shared name strings (some bolts repeat a name).
    pub(crate) fn oc628_bolt_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        seed: u32,
        profile: u32,
        ch: u32,
        count: u32,
    ) {
        const DB: u32 = 0x004a_6750;
        const PROFILES: u32 = 0x004a_6ca8;
        let list = DATA + 0x3000;
        let keytab = DATA + 0x1000;
        let names = DATA + 0x4000;
        let prof = PROFILES + profile * 0x9b50 + ch * 0x0f88;
        c.bus.write32(DB + ch * 4 + 0x2c, count);
        c.bus.write32(DB + ch * 4 + 0xa4, 0);
        c.bus.write32(DB + 4, list);
        c.bus.write32(prof + 0x288, keytab);
        for k in 0..8u32 {
            let idx = if r.next_u32() % 4 == 0 {
                0xffff
            } else {
                r.next_u32() % 4
            };
            c.bus.write16(keytab + 2 * k, idx as u16);
        }
        for idx in 0..4u32 {
            c.bus
                .write16(prof + 0x290 + 4 * idx + 2, (r.next_u32() & 0x13) as u16);
        }
        for i in 0..count {
            let e = list + 0x38 * i;
            c.bus.write8(e + 3, if seed % 7 == 0 { 0xff } else { 0 });
            c.bus.write16(e + 4, ((seed + i) % 8) as u16);
            c.bus.write32(e + 0x34, r.next_u32());
            let name = names + 0x10 * ((seed + i) % 3);
            c.bus.write32(e + 0x28, name);
            c.bus
                .write32(e + 0x2c, if (seed + i) % 5 == 0 { 0 } else { name });
        }
        for k in 0..3u32 {
            let s = names + 0x10 * k;
            c.bus.write8(s, b'b');
            c.bus.write8(s + 1, b'0' + k as u8);
            c.bus.write8(s + 2, 0);
        }
    }

    #[test]
    fn bolt_int_collect_equipped_file_names_matches() {
        check(
            ee::s01::f_0014bc18__generated,
            bolt_int_collect_equipped_file_names,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_bc18, 0);
                let profile = seed % 3;
                let ch = seed / 3 % 4;
                let count = seed / 12 % 4;
                oc628_bolt_fixture(c, &mut r, seed, profile, ch, count);
                c.pc = 0x0014_bc18;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = profile as u64;
                c.g[A2][0] = ch as u64;
                c.g[A3][0] = (DATA + 0x6000) as u64;
            },
        );
    }

    #[test]
    fn bolts_collect_children_matches() {
        check(
            ee::s01::f_0014d608__generated,
            bolts_collect_children,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_d608, 0);
                let group = seed % 3;
                let key = [u64::MAX, 7, 0x41][(seed / 3 % 3) as usize];
                let variant = seed / 9 % 4;
                c.pc = 0x0014_d608;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = group as u64;
                c.g[A2][0] = key;
                c.g[A3][0] = (DATA + 0x6000) as u64;
                c.g[T0][0] = (seed % 3) as u64;
                c.g[T1][0] = (seed / 27 % 2) as u64;
                c.bus.write32(DATA + 4, DATA + 0x2000);
                for g in 0..3u32 {
                    c.bus.write32(DATA + g * 4 + 0x2c, 3);
                    c.bus.write32(DATA + g * 4 + 0xa4, 0);
                    c.bus.write32(
                        DATA + g * 4 + 0x11c,
                        if variant == 0 { 0 } else { DATA + 0x1200 },
                    );
                    c.bus.write32(DATA + g * 4 + 0x194, DATA + 0x1000);
                }
                for k in 0..0x80u32 {
                    c.bus
                        .write16(DATA + 0x1200 + 2 * k, (r.next_u32() % 3) as u16);
                    c.bus
                        .write16(DATA + 0x1000 + 2 * k, (r.next_u32() % 3) as u16);
                }
                for i in 0..3u32 {
                    let e = DATA + 0x2000 + 0x38 * i;
                    for w in 0..0x38 / 4 {
                        c.bus.write32(e + 4 * w, r.next_u32());
                    }
                    c.bus.write16(e + 4, (r.next_u32() % 0x80) as u16);
                    c.bus.write16(e + 6, [0x41u16, 7, 3][i as usize]);
                }
            },
        );
    }

    pub(crate) use crate::game::ai::ai::tests::oc675_begin;
    pub(crate) use crate::game::be::beintnetwork::tests::oc675_fill_databases;

    /// `Bolts_FindTable2EntriesByFlag` against the generated body: empty and
    /// negative counts, empty ranges, kinds below / equal / above the wanted
    /// one, matching and non-matching flags, the match count at `*out`.
    #[test]
    fn bolts_find_table2_entries_by_flag_matches() {
        check(
            ee::s01::f_0014db40__generated,
            bolts_find_table2_entries_by_flag_2,
            300,
            &|c, seed| {
                let mut r = Rng(0x1234_5678 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0014_db40;
                let bolts = DATA + 0x6_0000;
                let entries = DATA + 0x6_1000;
                let out = DATA + 0x6_0800;
                let index = r.next_u32() % 4;
                c.g[A0][0] = sx(bolts);
                c.g[A1][0] = sx(index);
                c.g[A2][0] = sx(r.next_u32() % 4);
                c.g[A3][0] = sx(r.next_u32() % 2);
                c.g[T0][0] = sx(out);
                c.bus.write32(out, r.next_u32());
                c.bus.write32(bolts + 8, entries);
                let count = match r.next_u32() % 6 {
                    0 => 0,
                    1 => -2i32 as u32,
                    n => n,
                };
                c.bus.write32(bolts + 0x20c + 4 * index, count);
                let start = r.next_u32() % 3;
                c.bus.write32(bolts + 0x284 + 4 * index, start);
                for i in 0..12 {
                    let e = entries + 12 * i;
                    c.bus.write16(e + 2, (r.next_u32() % 4) as u16);
                    c.bus
                        .write8(e + 1, [0, 1, 1, 0x80, 5][(r.next_u32() % 5) as usize]);
                }
            },
        );
    }

    /// `BELibrary_SerializeSave`: the destination's alignment decides between
    /// the doubleword path and the unaligned `ldl/ldr` path of each block;
    /// rider 1 present or absent.
    #[test]
    fn be_library_serialize_save_matches() {
        check(
            ee::s01::f_0014a188__generated,
            be_library_serialize_save,
            96,
            &|c, seed| {
                let mut r = oc675_begin(c, seed, 0x0014_a188, 0, &[]);
                oc675_fill_databases(c, &mut r, seed);
                let off = [0u32, 4, 8, 1, 2, 16][(seed / 16 % 6) as usize];
                fill(c, &mut r, DATA + 0x10000, 0x6000);
                c.g[A0][0] = (DATA + 0x10000 + off) as u64;
            },
        );
    }

    /// `BEBoltInt_CollectEquippedTextureNames`: the bolt database fixture of
    /// the file-name collector, with the texture-name callee faked.
    #[test]
    fn bolt_int_collect_equipped_texture_names_matches() {
        check(
            ee::s01::f_0014b7a8__generated,
            bolt_int_collect_equipped_texture_names_2,
            200,
            &|c, seed| {
                let mut r = oc675_begin(c, seed, 0x0014_b7a8, 0, &[]);
                let profile = seed % 3;
                let ch = seed / 3 % 4;
                let count = seed / 12 % 4;
                crate::game::be::belibrary::tests::oc628_bolt_fixture(
                    c, &mut r, seed, profile, ch, count,
                );
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = profile as u64;
                c.g[A2][0] = ch as u64;
                c.g[A3][0] = (DATA + 0x6000) as u64;
            },
        );
    }

    /// `Bolts_CollectByGroup`: nodes of three groups with parent keys and
    /// 0x20 flags, depth 0..2 and the `all` flag.
    #[test]
    fn bolts_collect_by_group_matches() {
        check(
            ee::s01::f_0014d7e8__generated,
            bolts_collect_by_group_2,
            300,
            &|c, seed| {
                let mut r = oc675_begin(c, seed, 0x0014_d7e8, 0, &[]);
                let group = seed % 3;
                let key = [u64::MAX, 7, 0x41][(seed / 3 % 3) as usize];
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = group as u64;
                c.g[A2][0] = key;
                c.g[A3][0] = (DATA + 0x6000) as u64;
                c.g[T0][0] = (seed / 9 % 3) as u64;
                c.g[T1][0] = (seed / 27 % 2) as u64;
                c.bus.write32(DATA + 4, DATA + 0x2000);
                for g in 0..3u32 {
                    c.bus.write32(DATA + g * 4 + 0x2c, 3);
                    c.bus.write32(DATA + g * 4 + 0xa4, 0);
                }
                for i in 0..3u32 {
                    let e = DATA + 0x2000 + 0x38 * i;
                    for w in 0..0x38 / 4 {
                        c.bus.write32(e + 4 * w, r.next_u32());
                    }
                    c.bus
                        .write16(e + 4, [7u16, 0x41, 3][(r.next_u32() % 3) as usize]);
                    c.bus
                        .write16(e + 8, [0xffffu16, 7, 0x41][(r.next_u32() % 3) as usize]);
                    let flag = if r.next_u32() % 2 == 0 { 0x20 } else { 0 };
                    c.bus.write32(e + 0x34, (r.next_u32() & !0x20) | flag);
                }
            },
        );
    }

    #[test]
    fn bolts_count_children_matches() {
        check(
            ee::s01::f_0014d4c8__generated,
            bolts_count_children_2,
            200,
            &|c, seed| {
                machine(c, seed, 0xd4c8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0014d4c8;
            },
        );
    }

    /// `BELibrary_GetInstance` with the singleton built (the hot path): any
    /// instance pointer, random stack and return address. (The one-time
    /// construction runs the generated body itself.)
    #[test]
    fn be_library_get_instance_matches() {
        check(
            ee::s01::f_0014dc80__generated,
            be_library_get_instance,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14dc80, 0);
                c.bus.write32(GAME_GP - 0x1f04, r.next_u32() | 1);
                c.pc = 0x0014_dc80;
            },
        );
    }
}
