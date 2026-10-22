//! Work package D4: AI, BE interfaces & race logic
//! (`docs/investigation/phase2-candidates-2.md`, structs in
//! `docs/investigation/structs/{rider,world}.md`).
//!
//! - **Back-end singletons.** The BE ("back end") layer keeps one lazily
//!   created object per interface, each behind a global in the gp area.
//!   `*_GetInstance` allocates it on first use (`EA_MemAllocNamed`),
//!   initialises it (vtable words, a constructor or a virtual init slot) and
//!   returns the global, reloaded.
//! - **Back-end tables.** The player table (0x534fe0, 0x1c-byte records), the
//!   character attribute table (0x535538: 0x46 bytes per profile, 7 per
//!   character), the character maximum table (0x5308d8, 15-byte records)
//!   and the 3-D aggression (rivalry) table at 0x4a6ca8.
//! - **The AI race manager** (`AI`, alloc "AI", 0xd0 bytes): rider array at
//!   +0x28, players at +0x40, counts at +0x78/+0x7c, rivals switch at +0x80;
//!   its race-state machine and the per-pair rivalry pass.
//! - **CPU riders** (`ComputerActorRider`): the Cruise and Wipeout thinking
//!   steps (reset request, shove bits, the behaviour pointer-to-member) and
//!   the steering look-ahead point.
//!
//! The replacements follow the call protocol of [`crate::call`]. Every loop
//! here is a structured loop in the generated code (the `event()` result is
//! ignored), so loop state lives in the guest registers the original uses
//! and is read back from them after each back-edge check.

use crate::abi::{self, addu, r32};
use crate::call::{call_ptr, guest, Callee, Flow, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::{vu0_scratch, FpuF32, Vec4, VuF32};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_48d8, be_new_race_int_get_instance, "BENewRaceInt_GetInstance"),
    (0x0014_5970, be_new_player_int_get_instance, "BENewPlayerInt_GetInstance"),
    (0x0014_7f78, be_char_int_get_instance, "BECharInt_GetInstance"),
    (0x0015_5a50, be_aggression_int_get_instance, "BEAggressionInt_GetInstance"),
    (0x0015_4948, be_score_int_get_instance, "BEScoreInt_GetInstance"),
    (0x0014_f250, be_option_int_get_instance, "BEOptionInt_GetInstance"),
    (0x0014_6e98, be_new_player_int_get_human_rider, "BENewPlayerInt_GetHumanRider"),
    (0x0014_7398, be_new_player_int_get_character, "BENewPlayerInt_GetCharacter"),
    (0x0014_7290, be_new_player_int_get_pad_index, "BENewPlayerInt_GetPadIndex"),
    (0x0014_4c98, be_new_race_int_get_track_peak, "BENewRaceInt_GetTrackPeak"),
    (0x0014_8410, be_char_int_get_attrib0, "BECharInt_GetAttrib0"),
    (0x0014_84e0, be_char_int_get_attrib1, "BECharInt_GetAttrib1"),
    (0x0014_85b0, be_char_int_get_attrib2, "BECharInt_GetAttrib2"),
    (0x0014_8680, be_char_int_get_attrib3, "BECharInt_GetAttrib3"),
    (0x0014_8750, be_char_int_get_attrib4, "BECharInt_GetAttrib4"),
    (0x0014_8820, be_char_int_get_attrib5, "BECharInt_GetAttrib5"),
    (0x0014_88f0, be_char_int_get_attrib6, "BECharInt_GetAttrib6"),
    (0x0014_8d80, be_char_int_get_attrib3_factor, "BECharInt_GetAttrib3Factor"),
    (0x0014_8e68, be_char_int_get_attrib3_factor_2, "BECharInt_GetAttrib3Factor_2"),
    (0x0014_8f50, be_char_int_get_attrib5_factor, "BECharInt_GetAttrib5Factor"),
    (0x0014_9038, be_char_int_get_attrib5_factor_2, "BECharInt_GetAttrib5Factor_2"),
    (0x0014_9120, be_char_int_get_attrib6_factor, "BECharInt_GetAttrib6Factor"),
    (0x0014_9208, be_char_int_get_attrib6_factor_2, "BECharInt_GetAttrib6Factor_2"),
    (0x0014_92f0, be_char_int_get_attrib6_factor_3, "BECharInt_GetAttrib6Factor_3"),
    (0x0014_93d8, be_char_int_get_attrib1_factor, "BECharInt_GetAttrib1Factor"),
    (0x0014_94c0, be_char_int_get_attrib0_factor, "BECharInt_GetAttrib0Factor"),
    (0x0014_95a8, be_char_int_get_attrib4_factor, "BECharInt_GetAttrib4Factor"),
    (0x0014_9690, be_char_int_get_attrib2_factor, "BECharInt_GetAttrib2Factor"),
    (0x0015_5b50, be_aggression_int_get_rival_level, "BEAggressionInt_GetRivalLevel"),
    (0x0012_a250, ai_all_players_finished, "AI_AllPlayersFinished"),
    (0x0012_a180, ai_are_all_riders_loaded, "AI_AreAllRidersLoaded"),
    (0x0012_98c8, ai_get_frame_counter, "AI_GetFrameCounter"),
    (0x0011_3c20, ai_update_race_state, "AI_UpdateRaceState"),
    (0x0010_f878, ai_update_rivals, "AI_UpdateRivals"),
    (0x0010_a960, computer_actor_rider_think_cruise, "ComputerActorRider_ThinkCruise"),
    (0x0010_b750, computer_actor_rider_think_wipeout, "ComputerActorRider_ThinkWipeout"),
    (0x0010_0610, computer_actor_rider_get_steer_target, "ComputerActorRider_GetSteerTarget"),
    (0x0010_deb0, computer_actor_rider_get_human_rider, "ComputerActorRider_GetHumanRider"),
    (0x0011_3618, air_predictor_set_launch_state, "AirPredictor_SetLaunchState"),
];

// ---------------------------------------------------------------------------
// Callees and globals.

/// `EA_MemAllocNamed(size, name, flags, extra)` (another shard: the
/// original calls it through the function table).
const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;

mod callee {
    use super::{ee, Callee};
    pub const NEW_PLAYER_INT_CTOR: Callee = Callee { addr: 0x0014_5930, f: ee::s01::BENewPlayerInt_ctor };
    pub const SCORE_INT_CTOR: Callee = Callee { addr: 0x0015_49a8, f: ee::s01::BEScoreInt_ctor };
    pub const OPTION_INT_CTOR: Callee = Callee { addr: 0x0014_f418, f: ee::s01::BEOptionInt_ctor };
    pub const LIBRARY_CTOR: Callee = Callee { addr: 0x0014_9860, f: ee::s01::BELibrary_ctor };
    pub const PRE_RACE_GET_INSTANCE: Callee = Callee { addr: 0x0015_6570, f: ee::s01::BEPreRace_GetInstance };
    pub const RACE_GET_INSTANCE: Callee = Callee { addr: 0x0015_3088, f: ee::s01::BERace_GetInstance };
    pub const POST_RACE_GET_INSTANCE: Callee = Callee { addr: 0x0015_6338, f: ee::s01::BEPostRace_GetInstance };
    pub const GET_SLOT: Callee = Callee { addr: 0x0014_7798, f: ee::s01::BENewPlayerInt_GetSlot };
    pub const GET_RIDER_PROFILE: Callee = Callee { addr: 0x0014_a0e0, f: ee::s01::BE_GetRiderProfile };
    pub const GET_RIDER_CHARACTER: Callee = Callee { addr: 0x0014_a080, f: ee::s01::BE_GetRiderCharacter };
    /// Unnamed: "is this player's attribute fixed" (the factor is 0.5).
    pub const ATTRIB_FIXED: Callee = Callee { addr: 0x0014_77e8, f: ee::s01::f_001477e8 };
    pub const NEW_RACE_INT_GET_INSTANCE: Callee =
        Callee { addr: 0x0014_48d8, f: ee::s01::BENewRaceInt_GetInstance };
    /// Unnamed: the race's featured (boss) character.
    pub const RACE_FEATURED_CHARACTER: Callee = Callee { addr: 0x0014_5750, f: ee::s01::f_00145750 };
    pub const RIDER_IS_LOADED: Callee = Callee { addr: 0x0011_d640, f: ee::s00::Rider_IsLoaded };
    pub const SHOULD_RESET_PATH: Callee =
        Callee { addr: 0x0010_d1a0, f: ee::s00::ComputerActorRider_ShouldResetPath };
    pub const FIND_ATTACK_TARGET: Callee =
        Callee { addr: 0x0010_dbf0, f: ee::s00::ComputerActorRider_FindAttackTarget };
    /// Unnamed: "shove back" decision (cooldown at CAR+0xf30, result +0xf34).
    pub const SHOVE_BACK: Callee = Callee { addr: 0x0010_da10, f: ee::s00::f_0010da10 };
    pub const FIND_HUMAN_RIDER_INDEX: Callee =
        Callee { addr: 0x0010_d8f8, f: ee::s00::ComputerActorRider_FindHumanRiderIndex };
}

/// gp offsets of the globals (`*(gp - off)`).
mod global {
    /// `SSXApp*` (`App+0x84` game module, `GameModule+0xc` the AI manager).
    pub const APP: u32 = 0x848;
    pub const BE_LIBRARY: u32 = 0x1f04;
    pub const BE_LIBRARY_READY: u32 = 0x1f00;
    pub const BE_LIBRARY_PHASE: u32 = 0x1efc;
    pub const BE_LIBRARY_IMPL: u32 = 0x1ef8;
    pub const BE_LIBRARY_ALIAS: u32 = 0x1ea0;
    pub const NEW_RACE_INT: u32 = 0x1f3c;
    pub const NEW_PLAYER_INT: u32 = 0x1f30;
    pub const CHAR_INT: u32 = 0x1f2c;
    pub const AGGRESSION_INT: u32 = 0x1ea4;
    pub const SCORE_INT: u32 = 0x1ea8;
    pub const OPTION_INT: u32 = 0x1ee4;
    /// 0.1f, the steering look-ahead blend.
    pub const STEER_BLEND: u32 = 0x7fd0;
}

/// The address `*(gp - off)` lives at.
#[inline(always)]
fn gp_global(c: &Cpu, off: u32) -> u32 {
    r32(c, GP).wrapping_sub(off)
}

/// The address `reg + off` (32-bit address arithmetic).
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `rd = rs + rt` (`addu`, sign-extended).
#[inline(always)]
fn add_regs(c: &mut Cpu, rd: usize, rs: usize, rt: usize) {
    c.g[rd][0] = addu(c.g[rs][0], c.g[rt][0]);
}

// ---------------------------------------------------------------------------
// Back-end singletons.

/// One lazily created BE interface object.
struct Singleton {
    /// gp offset of the global holding it.
    global: u32,
    /// `EA_MemAllocNamed` size and name.
    size: u64,
    name: u32,
    /// Return address of the allocation call.
    alloc_ret: u32,
    /// Cycles of the block that allocates and initialises.
    init_cycles: u64,
}

/// The `*_GetInstance` shape: a 16-byte frame holding `ra`; if the global is
/// 0, allocate (`EA_MemAllocNamed(size, name, 0, 0)`) and run `init` with
/// the new object in v0; return the global, reloaded. 4 + (init) + 4 cycles.
fn lazy_singleton(c: &mut Cpu, s: Singleton, init: impl FnOnce(&mut Cpu) -> Flow) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw(gp_global(c, s.global));
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += s.init_cycles;
            // Registers at the call.
            c.g[A1][0] = sx(s.name);
            c.g[A0][0] = s.size;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, EA_MEM_ALLOC_NAMED, s.alloc_ret)?;
            init(c)?;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw(gp_global(c, s.global));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Initialise a bare interface object (v0): `+8 = 0`, `+0xc = vtable`,
/// then publish it in the global.
fn init_with_vtable(c: &mut Cpu, vtable: u32, global: u32) {
    let obj = r32(c, V0);
    c.g[V1][0] = sx(vtable);
    c.sw(obj.wrapping_add(8), 0);
    c.sw(obj.wrapping_add(0xc), c.g[V1][0]);
    c.sw(gp_global(c, global), c.g[V0][0]);
}

/// `BENewRaceInt* BENewRaceInt_GetInstance(void)` (0x1448d8): the race
/// setup interface (vtable 0x45ae58).
pub fn be_new_race_int_get_instance(c: &mut Cpu) {
    let s = Singleton { global: global::NEW_RACE_INT, size: 0x10, name: 0x0045_a0f0, alloc_ret: 0x0014_4900, init_cycles: 11 };
    lazy_singleton(c, s, |c| {
        init_with_vtable(c, 0x0045_ae58, global::NEW_RACE_INT);
        Ok(())
    })
}

/// `BEAggressionInt* BEAggressionInt_GetInstance(void)` (0x155a50): the
/// rivalry interface (vtable 0x45ac48).
pub fn be_aggression_int_get_instance(c: &mut Cpu) {
    let s = Singleton { global: global::AGGRESSION_INT, size: 0x10, name: 0x0045_a898, alloc_ret: 0x0015_5a78, init_cycles: 11 };
    lazy_singleton(c, s, |c| {
        init_with_vtable(c, 0x0045_ac48, global::AGGRESSION_INT);
        Ok(())
    })
}

/// `BENewPlayerInt* BENewPlayerInt_GetInstance(void)` (0x145970): built by
/// `BENewPlayerInt_ctor`; the global holds what the constructor returns.
pub fn be_new_player_int_get_instance(c: &mut Cpu) {
    let s = Singleton { global: global::NEW_PLAYER_INT, size: 0x10, name: 0x0045_a1d8, alloc_ret: 0x0014_5998, init_cycles: 9 };
    lazy_singleton(c, s, |c| {
        c.g[A0][0] = c.g[V0][0];
        callee::NEW_PLAYER_INT_CTOR.call(c, 0x0014_59a0)?;
        c.sw(gp_global(c, global::NEW_PLAYER_INT), c.g[V0][0]);
        Ok(())
    })
}

/// `BECharInt* BECharInt_GetInstance(void)` (0x147f78): the character
/// interface. Published first, then vtable 0x45ad68 and the init method in
/// its slot 0x10 (called through the static vtable).
pub fn be_char_int_get_instance(c: &mut Cpu) {
    let s = Singleton { global: global::CHAR_INT, size: 0x10, name: 0x0045_a280, alloc_ret: 0x0014_7fa0, init_cycles: 15 };
    lazy_singleton(c, s, |c| {
        const VTABLE: u32 = 0x0045_ad68;
        let obj = c.g[V0][0];
        c.g[V1][0] = sx(VTABLE);
        c.sw(gp_global(c, global::CHAR_INT), obj);
        let init = VSlot::read(c, sx(VTABLE), 0x10);
        c.g[A1][0] = init.func;
        c.g[A0][0] = init.this(obj);
        c.sw((obj as u32).wrapping_add(0xc), c.g[V1][0]);
        c.sw((obj as u32).wrapping_add(8), 0);
        call_ptr(c, init.func as u32, 0x0014_7fc4)
    })
}

/// `BEScoreInt* BEScoreInt_GetInstance(void)` (0x154948): built by
/// `BEScoreInt_ctor`; then the score fields are reset on the constructor's
/// result (`+0x1c = +0x18 = -1`, `+0x8 = +0x10 = +0x14 = 0`).
pub fn be_score_int_get_instance(c: &mut Cpu) {
    let s = Singleton { global: global::SCORE_INT, size: 0x230, name: 0x0045_a858, alloc_ret: 0x0015_4970, init_cycles: 15 };
    lazy_singleton(c, s, |c| {
        c.g[A0][0] = c.g[V0][0];
        callee::SCORE_INT_CTOR.call(c, 0x0015_4978)?;
        let obj = r32(c, V0);
        c.sw(gp_global(c, global::SCORE_INT), c.g[V0][0]);
        c.g[V1][0] = u64::MAX;
        c.sw(obj.wrapping_add(0x1c), u64::MAX);
        c.sw(obj.wrapping_add(0x8), 0);
        c.sw(obj.wrapping_add(0x10), 0);
        c.sw(obj.wrapping_add(0x14), 0);
        c.sw(obj.wrapping_add(0x18), u64::MAX);
        Ok(())
    })
}

/// `BEOptionInt* BEOptionInt_GetInstance(void)` (0x14f250): vtable
/// 0x45adc8, published, then `BEOptionInt_ctor` (a re-entrant `GetInstance`
/// from the constructor already sees the object).
pub fn be_option_int_get_instance(c: &mut Cpu) {
    let s = Singleton { global: global::OPTION_INT, size: 0x10, name: 0x0045_a6b0, alloc_ret: 0x0014_f278, init_cycles: 13 };
    lazy_singleton(c, s, |c| {
        let obj = r32(c, V0);
        c.g[V1][0] = sx(0x0045_adc8);
        c.g[A0][0] = c.g[V0][0];
        c.sw(obj.wrapping_add(0xc), c.g[V1][0]);
        c.sw(gp_global(c, global::OPTION_INT), c.g[V0][0]);
        c.sw(obj.wrapping_add(8), 0);
        callee::OPTION_INT_CTOR.call(c, 0x0014_f294)
    })
}

/// The table of race-phase interfaces (`g_beLibrary`) that
/// `BELibrary_GetInterface` indexes: pre-race, race, post-race.
const BE_PHASE_INTERFACES: u32 = 0x0053_08a8;

/// `BELibrary* BELibrary_GetInstance(void)` (0x14dc80): on first use it
/// builds the whole back end: the library handle, the `BELibrary` object,
/// the three race-phase interfaces, selects phase 0 and calls its
/// activation method (vtable slot 0x20). 6 cycles, + 38 when building, + 6.
pub fn be_library_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.lw(gp_global(c, global::BE_LIBRARY));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 38;
            c.g[A1][0] = 0x004a_1200;
            c.g[A0][0] = 4;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[S0][0] = 0x0053_0000;
            call_ptr(c, EA_MEM_ALLOC_NAMED, 0x0014_dcb4)?;
            c.g[S1][0] = sx(BE_PHASE_INTERFACES);
            c.g[A1][0] = sx(0x0045_a638);
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[A0][0] = 1;
            c.sw(gp_global(c, global::BE_LIBRARY), c.g[V0][0]);
            call_ptr(c, EA_MEM_ALLOC_NAMED, 0x0014_dcd4)?;
            c.g[A0][0] = c.g[V0][0];
            callee::LIBRARY_CTOR.call(c, 0x0014_dcdc)?;
            c.g[A0][0] = 0;
            c.sw(gp_global(c, global::BE_LIBRARY_IMPL), c.g[V0][0]);
            callee::PRE_RACE_GET_INSTANCE.call(c, 0x0014_dce8)?;
            c.g[A0][0] = 1;
            c.sw(BE_PHASE_INTERFACES, c.g[V0][0]);
            callee::RACE_GET_INSTANCE.call(c, 0x0014_dcf4)?;
            c.g[A0][0] = 2;
            c.sw(BE_PHASE_INTERFACES + 4, c.g[V0][0]);
            callee::POST_RACE_GET_INSTANCE.call(c, 0x0014_dd00)?;
            c.sw(BE_PHASE_INTERFACES + 8, c.g[V0][0]);
            c.sw(gp_global(c, global::BE_LIBRARY_PHASE), 0);
            // Activate phase 0: its vtable is at +4.
            let iface = c.lw(BE_PHASE_INTERFACES);
            c.g[A1][0] = iface;
            c.g[V0][0] = c.lw((iface as u32).wrapping_add(4));
            let activate = VSlot::read(c, c.g[V0][0], 0x20);
            c.g[V1][0] = activate.func;
            c.g[A0][0] = activate.this(iface);
            call_ptr(c, activate.func as u32, 0x0014_dd20)?;
            c.g[A0][0] = c.lw(gp_global(c, global::BE_LIBRARY));
            c.g[V0][0] = 1;
            c.sw(gp_global(c, global::BE_LIBRARY_READY), 1);
            c.sw(gp_global(c, global::BE_LIBRARY_ALIAS), c.g[A0][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = c.lw(gp_global(c, global::BE_LIBRARY));
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Back-end tables.

/// The player table: 0x1c-byte records indexed by race slot.
pub mod player_table {
    /// Address of the record base (`0x530000 + 0x4fe0`).
    pub const BASE: u32 = 0x0053_4fe0;
    pub const RECORD_SIZE: u32 = 0x1c;
    /// Pad index word; < 0 for a CPU rider (also the "human" test).
    pub const PAD_INDEX: u32 = 0x0c;
    /// Character id (signed byte).
    pub const CHARACTER: u32 = 0x11;
}

/// Character attributes: `ATTRIBS + profile * 0x46 + character * 7 + n`.
pub const ATTRIBS: u32 = 0x0053_5538;
pub const ATTRIB_PROFILE_STRIDE: u32 = 0x46;
/// Per-character maxima: `CHARACTER_MAX + character * 15 + 8 + n`.
pub const CHARACTER_MAX: u32 = 0x0053_08d8;

/// The player-table record of player `a1`: `GetSlot(a1)` (16-byte frame,
/// one 13-cycle block before the call), then `mult` slot by the record size.
/// Leaves v1 = the table base, a0 = 0x1c; returns the record address in a
/// register named by `rec` (v0 or v1) as the two originals do.
fn player_record(c: &mut Cpu, call_ret: u32, rec: usize, field: u32, load: fn(&mut Cpu, u32) -> u64) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        callee::GET_SLOT.call(c, call_ret)?;
        c.g[A0][0] = player_table::RECORD_SIZE as u64;
        let off = abi::mult(c, r32(c, V0), player_table::RECORD_SIZE);
        c.g[V0][0] = sx(off);
        c.g[V1][0] = sx(player_table::BASE);
        frame.restore_ra(c, 0);
        let other = if rec == V0 { V1 } else { V0 };
        add_regs(c, rec, rec, other);
        c.g[V0][0] = load(c, at(c, rec, field));
        frame.close(c);
        Ok(())
    })
}

/// `int8 BENewPlayerInt_GetCharacter(this, int player)` (0x147398): the
/// player's character id. 13 cycles.
pub fn be_new_player_int_get_character(c: &mut Cpu) {
    player_record(c, 0x0014_73a8, V0, player_table::CHARACTER, |c, a| c.lb(a));
}

/// `int BENewPlayerInt_GetPadIndex(this, int player)` (0x147290): the
/// player's pad (< 0 for a CPU). 13 cycles.
pub fn be_new_player_int_get_pad_index(c: &mut Cpu) {
    player_record(c, 0x0014_72a0, V1, player_table::PAD_INDEX, |c, a| c.lw(a));
}

/// `int BENewPlayerInt_GetHumanRider(this, int second)` (0x146e98): the
/// index (0..5) of the first human player, or of the second one when
/// `second != 0`; -1 if there is none.
pub fn be_new_player_int_get_human_rider(c: &mut Cpu) {
    const HEAD: u32 = 0x0014_6ed8;
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x0053_0000;
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        // s0 = index, s1 = humans seen, s2 = second, s3 = 1, s4 = record
        // size, s5 = &table[0].pad
        c.g[S0][0] = 0;
        c.g[S1][0] = 0;
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = 1;
        c.g[S4][0] = player_table::RECORD_SIZE as u64;
        c.g[S5][0] = sx(player_table::BASE + player_table::PAD_INDEX);
        'found: {
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                callee::GET_SLOT.call(c, 0x0014_6ee0)?;
                let off = abi::mult(c, r32(c, V0), r32(c, S4));
                c.g[V1][0] = sx(off);
                add_regs(c, V0, V1, S5);
                c.g[V1][0] = c.lw(r32(c, V0));
                if (c.g[V1][0] as i64) < 0 {
                    // A CPU slot.
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    if c.g[S2][0] == 0 {
                        c.cycles += 1;
                        break 'found;
                    }
                    c.cycles += 2;
                    let is_second = c.g[S1][0] == c.g[S3][0];
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    if is_second {
                        c.cycles += 2;
                        break 'found;
                    }
                    c.cycles += 1;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                }
                c.cycles += 3;
                let more = (c.g[S0][0] as i64) < 6;
                c.g[V0][0] = u64::MAX;
                if !more {
                    c.cycles += 1;
                    break 'found;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        frame.restore(c, S0, 0x60);
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BENewRaceInt_GetTrackPeak(void)` (0x144c98): the peak (mountain)
/// of the current track, from the 100-byte track records at 0x43d950.
/// 9 cycles.
pub fn be_new_race_int_get_track_peak(c: &mut Cpu) {
    c.cycles += 9;
    c.g[A0][0] = 0x0053_0000;
    c.g[A1][0] = 0x64;
    let track = c.lw(0x0053_5c08);
    let off = abi::mult(c, track as u32, 0x64);
    c.g[V1][0] = sx(0x0043_d950u32.wrapping_add(off));
    c.g[V0][0] = c.lw(at(c, V1, 0x54));
    abi::ret(c);
}

/// How the compiler allocated the attribute-address registers (attribute
/// 0 swaps a0 and v1 against the others).
#[derive(Clone, Copy, PartialEq)]
enum Regs {
    /// Attributes 1-6.
    A,
    /// Attribute 0.
    B,
}

/// `int8 BECharInt_GetAttribN(this, int player)` (0x148410 + ...): the
/// rider's attribute `n` from its profile and character. 24 cycles.
fn get_attrib(c: &mut Cpu, start: u32, n: u32, regs: Regs) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S1][0];
        callee::GET_RIDER_PROFILE.call(c, start + 0x1c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        callee::GET_RIDER_CHARACTER.call(c, start + 0x28)?;
        let character = r32(c, V0);
        let profile_off = abi::mult(c, r32(c, S0), ATTRIB_PROFILE_STRIDE);
        let entry_off = (character << 3).wrapping_sub(character).wrapping_add(profile_off);
        let addr = ATTRIBS.wrapping_add(entry_off);
        match regs {
            Regs::A => {
                c.g[A0][0] = sx(entry_off);
                c.g[V1][0] = sx(addr);
            }
            Regs::B => {
                c.g[A0][0] = ATTRIB_PROFILE_STRIDE as u64;
                c.g[V1][0] = sx(addr);
            }
        }
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = c.lb(addr.wrapping_add(n));
        frame.close(c);
        Ok(())
    })
}

pub fn be_char_int_get_attrib0(c: &mut Cpu) {
    get_attrib(c, 0x0014_8410, 0, Regs::B)
}
pub fn be_char_int_get_attrib1(c: &mut Cpu) {
    get_attrib(c, 0x0014_84e0, 1, Regs::A)
}
pub fn be_char_int_get_attrib2(c: &mut Cpu) {
    get_attrib(c, 0x0014_85b0, 2, Regs::A)
}
pub fn be_char_int_get_attrib3(c: &mut Cpu) {
    get_attrib(c, 0x0014_8680, 3, Regs::A)
}
pub fn be_char_int_get_attrib4(c: &mut Cpu) {
    get_attrib(c, 0x0014_8750, 4, Regs::A)
}
pub fn be_char_int_get_attrib5(c: &mut Cpu) {
    get_attrib(c, 0x0014_8820, 5, Regs::A)
}
pub fn be_char_int_get_attrib6(c: &mut Cpu) {
    get_attrib(c, 0x0014_88f0, 6, Regs::A)
}

/// `float BECharInt_GetAttribNFactor(this, int player, int value)`: how far
/// the rider's attribute `n` is along the character's maximum, as
/// `value / max` (`value` defaults to `attrib / 5` when it is ≤ 0). A fixed
/// attribute (`f_001477e8`) gives 0.5. Result in f0.
fn get_attrib_factor(c: &mut Cpu, start: u32, n: u32, regs: Regs) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S0][0];
        callee::GET_RIDER_PROFILE.call(c, start + 0x28)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S3][0] = c.g[V0][0];
        callee::GET_RIDER_CHARACTER.call(c, start + 0x34)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::ATTRIB_FIXED.call(c, start + 0x40)?;
        let fixed = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x40);
        if fixed {
            c.cycles += 4;
            c.g[AT][0] = 0x3f00_0000;
            abi::set_fpr(c, 0, FpuF32(0x3f00_0000));
            frame.restore(c, S1, 0x30);
        } else {
            c.cycles += 2;
            c.g[A2][0] = c.g[S2][0];
            if (c.g[S2][0] as i64) <= 0 {
                // value = attrib / 5
                c.cycles += 9 + 6;
                let character = r32(c, S1);
                let profile_off = abi::mult(c, r32(c, S3), ATTRIB_PROFILE_STRIDE);
                let entry_off = (character << 3).wrapping_sub(character).wrapping_add(profile_off);
                let addr = ATTRIBS.wrapping_add(entry_off);
                c.g[A1][0] = 5;
                match regs {
                    Regs::A => c.g[A0][0] = sx(entry_off),
                    Regs::B => c.g[A0][0] = sx(ATTRIBS),
                }
                c.g[V1][0] = sx(addr);
                let attrib = c.lb(addr.wrapping_add(n));
                let (q, _) = abi::div(c, attrib as u32, 5);
                c.g[V0][0] = sx(q);
                c.g[A2][0] = sx(q);
            }
            c.cycles += 13;
            let character = r32(c, S1);
            let max_rec = CHARACTER_MAX.wrapping_add((character << 4).wrapping_sub(character));
            c.g[V1][0] = sx(max_rec);
            let value = FpuF32::from_i32(r32(c, A2) as i32);
            abi::set_fpr(c, 1, value);
            c.g[V0][0] = c.lb(max_rec.wrapping_add(8 + n));
            let max = FpuF32::from_i32(r32(c, V0) as i32);
            abi::set_fpr(c, 0, value.div(max));
            frame.restore(c, S0, 0x40);
            frame.restore(c, S1, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub fn be_char_int_get_attrib3_factor(c: &mut Cpu) {
    get_attrib_factor(c, 0x0014_8d80, 3, Regs::A)
}
pub fn be_char_int_get_attrib3_factor_2(c: &mut Cpu) {
    get_attrib_factor(c, 0x0014_8e68, 3, Regs::A)
}
/// Has an `__entries` twin: the stub sends other entries to it.
pub fn be_char_int_get_attrib5_factor(c: &mut Cpu) {
    get_attrib_factor(c, 0x0014_8f50, 5, Regs::A)
}
pub fn be_char_int_get_attrib5_factor_2(c: &mut Cpu) {
    get_attrib_factor(c, 0x0014_9038, 5, Regs::A)
}
pub fn be_char_int_get_attrib6_factor(c: &mut Cpu) {
    get_attrib_factor(c, 0x0014_9120, 6, Regs::A)
}
pub fn be_char_int_get_attrib6_factor_2(c: &mut Cpu) {
    get_attrib_factor(c, 0x0014_9208, 6, Regs::A)
}
pub fn be_char_int_get_attrib6_factor_3(c: &mut Cpu) {
    get_attrib_factor(c, 0x0014_92f0, 6, Regs::A)
}
pub fn be_char_int_get_attrib1_factor(c: &mut Cpu) {
    get_attrib_factor(c, 0x0014_93d8, 1, Regs::A)
}
pub fn be_char_int_get_attrib0_factor(c: &mut Cpu) {
    get_attrib_factor(c, 0x0014_94c0, 0, Regs::B)
}
pub fn be_char_int_get_attrib4_factor(c: &mut Cpu) {
    get_attrib_factor(c, 0x0014_95a8, 4, Regs::A)
}
pub fn be_char_int_get_attrib2_factor(c: &mut Cpu) {
    get_attrib_factor(c, 0x0014_9690, 2, Regs::A)
}

/// The rivalry table: `RIVALRY + profileB * 0x9b50 + characterB * 0xf88 +
/// characterA * 3` (signed bytes).
pub const RIVALRY: u32 = 0x004a_6ca8 + 0xbc2;

/// `int BEAggressionInt_GetRivalLevel(this, int a, int b)` (0x155b50): how
/// much rider `a` dislikes rider `b` (0..4; 4 = sworn rival). The race's
/// featured character is everyone's level-3 rival.
pub fn be_aggression_int_get_rival_level(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S1][0];
        callee::GET_RIDER_PROFILE.call(c, 0x0015_5b74)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
        callee::GET_RIDER_CHARACTER.call(c, 0x0015_5b80)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        callee::GET_RIDER_CHARACTER.call(c, 0x0015_5b8c)?;
        c.g[S1][0] = c.g[V0][0];
        callee::NEW_RACE_INT_GET_INSTANCE.call(c, 0x0015_5b94)?;
        c.g[A0][0] = c.g[V0][0];
        callee::RACE_FEATURED_CHARACTER.call(c, 0x0015_5b9c)?;
        let featured = c.g[V0][0] == c.g[S0][0];
        c.g[A0][0] = 0xf88;
        if featured {
            c.cycles += 1;
            c.g[V0][0] = 3;
        } else {
            c.cycles += 12;
            c.g[A1][0] = 0x9b50;
            let char_b = abi::mult(c, r32(c, S1), 0xf88);
            c.g[A0][0] = sx(char_b);
            let profile_b = abi::mult(c, r32(c, S2), 0x9b50);
            c.g[A1][0] = sx(profile_b);
            let char_a = r32(c, S0);
            let off = (char_a << 1).wrapping_add(char_a).wrapping_add(char_b).wrapping_add(profile_b);
            c.g[V0][0] = sx(off);
            let addr = 0x004a_6ca8u32.wrapping_add(off);
            c.g[V1][0] = sx(addr);
            c.g[V0][0] = c.lb(addr.wrapping_add(0xbc2));
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
// The AI race manager.

/// `AI` (alloc "AI", 0xd0 bytes; `GameModule+0xc`).
pub mod ai_mgr {
    /// `Rider*[]`, one per rider in the race.
    pub const RIDERS: u32 = 0x28;
    /// `Player*[]` (humans).
    pub const PLAYERS: u32 = 0x40;
    pub const RIDER_COUNT: u32 = 0x78;
    pub const PLAYER_COUNT: u32 = 0x7c;
    /// Rivalries enabled (> 0).
    pub const RIVALS_ENABLED: u32 = 0x80;
    /// `Race*` (frame counter at `+0xc` → `+8`).
    pub const RACE: u32 = 0x84;
}

/// Rider fields written by the rivalry pass (unmapped in rider.md so far).
pub mod rider_rival {
    /// 1 if the rider has a sworn (level 4) rival.
    pub const FLAG: u32 = 0xf0;
    /// Index of that rival (-1 none).
    pub const INDEX: u32 = 0xf8;
    /// Level towards the last rider compared.
    pub const LEVEL: u32 = 0xfc;
}

/// `bool AI_AllPlayersFinished(AI *ai)` (0x12a250): every human player
/// has a finish time (`state+0x470 ≥ 0`).
pub fn ai_all_players_finished(c: &mut Cpu) {
    const HEAD: u32 = 0x0012_a268;
    c.cycles += 3;
    c.g[V1][0] = c.lw(at(c, A0, ai_mgr::PLAYER_COUNT));
    c.g[A2][0] = 0;
    if (c.g[V1][0] as i64) > 0 {
        c.cycles += 3;
        abi::set_fpr(c, 1, FpuF32::ZERO);
        c.g[A3][0] = c.g[V1][0];
        c.g[A0][0] = sx(at(c, A0, ai_mgr::PLAYERS));
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw(r32(c, A0));
            c.g[V1][0] = c.lw(at(c, V0, 0x18));
            let finish_time = Addr(at(c, V1, 0x470)).read_fpu(c);
            abi::set_fpr(c, 0, finish_time);
            let finished = FpuF32(c.f[1]).le(finish_time);
            abi::set_fcc(c, finished);
            c.g[A1][0] = 1;
            if !finished {
                c.cycles += 1;
                c.g[A1][0] = 0;
            }
            c.cycles += 2;
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
            if c.g[A1][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            let more = (c.g[A2][0] as i64) < (c.g[A3][0] as i64);
            c.g[V0][0] = more as u64;
            c.g[A0][0] = sx(at(c, A0, 4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool AI_AreAllRidersLoaded(AI *ai)` (0x12a180): asks every rider
/// (`Rider_IsLoaded`, no early exit) and returns whether all are loaded.
pub fn ai_are_all_riders_loaded(c: &mut Cpu) {
    const HEAD: u32 = 0x0012_a1b0;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        // s1 = index, s2 = ai, s3 = all loaded, s0 = &riders[i]
        c.g[S1][0] = 0;
        c.g[S2][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(at(c, S2, ai_mgr::RIDER_COUNT));
        c.g[S3][0] = 1;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S0][0] = sx(at(c, S2, ai_mgr::RIDERS));
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.lw(r32(c, S0));
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[S0][0] = sx(at(c, S0, 4));
                callee::RIDER_IS_LOADED.call(c, 0x0012_a1c0)?;
                c.g[V1][0] = c.lw(at(c, S2, ai_mgr::RIDER_COUNT));
                let more = (c.g[S1][0] as i64) < (c.g[V1][0] as i64);
                c.g[V1][0] = more as u64;
                if c.g[V0][0] == 0 {
                    c.g[S3][0] = 0;
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AI_GetFrameCounter(void)` (0x1298c8): the race's frame counter
/// (`App → GameModule → AI → +8`). 5 cycles.
pub fn ai_get_frame_counter(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = c.lw(gp_global(c, global::APP));
    c.g[V1][0] = c.lw(at(c, V0, 0x84));
    c.g[A0][0] = c.lw(at(c, V1, 0xc));
    c.g[V0][0] = c.lw(at(c, A0, 8));
    abi::ret(c);
}

/// A gcc 2.9x virtual call on `obj` (vtable at `obj+0`): reads slot `slot`
/// (v0 = vtable, a0 = adjusted this, v1 = function) and calls it.
fn state_call(c: &mut Cpu, obj_reg: usize, slot: u32, ret: u32) -> Flow {
    let obj = c.g[obj_reg][0];
    c.g[V0][0] = c.lw(obj as u32);
    let m = VSlot::read(c, c.g[V0][0], slot);
    c.g[V1][0] = m.func;
    c.g[A0][0] = m.this(obj);
    call_ptr(c, m.func as u32, ret)
}

/// `void AI_UpdateRaceState(RaceStateMachine *sm)` (0x113c20): on a state
/// change (`+0` requested != `+4` current) exit the old state (`+0x9c`, if
/// any), enter the new one (`+0x98`), clear the old pointer and adopt the
/// id; then update the current state. Each state method gets `sm` in a1.
pub fn ai_update_race_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = c.lw(at(c, S0, 4));
        c.g[V0][0] = c.lw(r32(c, S0));
        if c.g[V1][0] == c.g[V0][0] {
            c.g[A2][0] = c.lw(at(c, S0, 0x98));
        } else {
            c.cycles += 3;
            c.g[A2][0] = c.lw(at(c, S0, 0x9c));
            c.g[A1][0] = c.g[S0][0];
            if c.g[A2][0] != 0 {
                c.cycles += 5;
                state_call(c, A2, 0x10, 0x0011_3c60)?; // old->Exit(sm)
            }
            c.cycles += 11;
            c.g[A2][0] = c.lw(at(c, S0, 0x98));
            c.g[A1][0] = c.g[S0][0];
            state_call(c, A2, 0x8, 0x0011_3c7c)?; // new->Enter(sm)
            c.g[V0][0] = c.lw(r32(c, S0));
            c.sw(at(c, S0, 0x9c), 0);
            c.sw(at(c, S0, 4), c.g[V0][0]);
            c.g[A2][0] = c.lw(at(c, S0, 0x98));
        }
        c.cycles += 10;
        c.g[A1][0] = c.g[S0][0];
        state_call(c, A2, 0x18, 0x0011_3ca4)?; // current->Update(sm)
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_UpdateRivals(AI *ai)` (0x10f878): when rivalries are on, give
/// every rider its rival: the last other rider at level 4 (`FLAG`/`INDEX`),
/// and the level towards the last rider compared (`LEVEL`).
pub fn ai_update_rivals(c: &mut Cpu) {
    const OUTER: u32 = 0x0010_f8d0;
    const INNER: u32 = 0x0010_f900;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S3, 0x60);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        // s3 = ai, s2 = rider i, s0 = rider j, s1 = &riders[i], s7 = riders,
        // s4 = i + 1, s5 = 1, s6 = 4, fp = -1
        c.g[S3][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(at(c, S3, ai_mgr::RIVALS_ENABLED));
        frame.restore(c, S0, 0x90);
        'out: {
            if (c.g[V0][0] as i64) <= 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, S3, ai_mgr::RIDER_COUNT));
            c.g[S2][0] = 0;
            if (c.g[V0][0] as i64) <= 0 {
                break 'out;
            }
            c.cycles += 4;
            c.g[S7][0] = sx(at(c, S3, ai_mgr::RIDERS));
            c.g[FP][0] = u64::MAX;
            c.g[V0][0] = sx(r32(c, S2) << 2);
            loop {
                c.cycles += 9;
                c.g[S0][0] = 0;
                add_regs(c, A0, S7, V0);
                c.g[V0][0] = c.lw(r32(c, A0));
                c.sw(at(c, V0, rider_rival::FLAG), 0);
                c.g[V1][0] = c.lw(r32(c, A0));
                c.sw(at(c, V1, rider_rival::INDEX), c.g[FP][0]);
                c.g[V0][0] = c.lw(at(c, S3, ai_mgr::RIDER_COUNT));
                c.g[S4][0] = sx(r32(c, S2).wrapping_add(1));
                if (c.g[V0][0] as i64) > 0 {
                    c.cycles += 3;
                    c.g[S1][0] = c.g[A0][0];
                    c.g[S6][0] = 4;
                    c.g[S5][0] = 1;
                    loop {
                        c.cycles += 2;
                        if c.g[S2][0] == c.g[S0][0] {
                            c.g[V0][0] = c.lw(at(c, S3, ai_mgr::RIDER_COUNT));
                        } else {
                            c.cycles += 9;
                            call_ptr(c, 0x0015_5a50, 0x0010_f910)?; // BEAggressionInt_GetInstance
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S0][0];
                            call_ptr(c, 0x0015_5b50, 0x0010_f920)?; // GetRivalLevel(i, j)
                            c.g[A0][0] = c.g[V0][0];
                            let sworn = c.g[A0][0] == c.g[S6][0];
                            c.g[V0][0] = c.lw(r32(c, S1));
                            if sworn {
                                c.cycles += 4;
                                c.sw(at(c, V0, rider_rival::FLAG), c.g[S5][0]);
                                c.g[V1][0] = c.lw(r32(c, S1));
                                c.sw(at(c, V1, rider_rival::INDEX), c.g[S0][0]);
                                c.g[V0][0] = c.lw(r32(c, S1));
                            }
                            c.cycles += 2;
                            c.sw(at(c, V0, rider_rival::LEVEL), c.g[A0][0]);
                            c.g[V0][0] = c.lw(at(c, S3, ai_mgr::RIDER_COUNT));
                        }
                        c.cycles += 4;
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        let more = (c.g[S0][0] as i64) < (c.g[V0][0] as i64);
                        c.g[V0][0] = more as u64;
                        if !more {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(at(c, S3, ai_mgr::RIDER_COUNT));
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, INNER);
                        }
                    }
                }
                c.cycles += 4;
                c.g[S2][0] = c.g[S4][0];
                let more = (c.g[S2][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = sx(r32(c, S2) << 2);
                if !more {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x90);
                    break 'out;
                }
                if event_due(c) {
                    deliver_event_here(c, OUTER);
                }
            }
        }
        c.cycles += 11;
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
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider (CPU riders).

/// ComputerActorRider fields used here (rider.md).
pub mod car {
    /// `Rider*` (the rider state block).
    pub const RIDER: u32 = 0x18;
    /// `noLateTrick`, cleared by every Cruise step.
    pub const NO_LATE_TRICK: u32 = 0xe20;
    /// `behaviourFn`: gcc 2.9x pointer to member `{i16 delta, i16 index,
    /// fn | i16 vptr offset}`.
    pub const BEHAVIOUR_FN: u32 = 0xf44;
}

/// Control-input word 0 bits set by the CPU.
pub mod control {
    /// Request a reset to the AI path.
    pub const RESET_PATH_BIT: u32 = 12;
    pub const SHOVE_LEFT: u64 = 0x40000;
    pub const SHOVE_RIGHT: u64 = 0x80000;
}

/// Bit 12 of the control word at `*ctl` = `ShouldResetPath() & 1` (v0),
/// as the originals compute it (64-bit mask in a0, result in v1).
fn merge_reset_request(c: &mut Cpu, ctl: usize) {
    c.g[V1][0] = c.lw(r32(c, ctl));
    c.g[A0][0] = !(1u64 << control::RESET_PATH_BIT);
    c.g[V0][0] &= 1;
    c.g[V1][0] &= c.g[A0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) << control::RESET_PATH_BIT);
    c.g[V1][0] |= c.g[V0][0];
}

/// A gcc 2.9x pointer to member function at `obj + off`: `{i16 delta;
/// i16 index; union { fn; i16 vptr_offset } }`. `index < 0` holds the
/// function itself; otherwise the 8-byte vtable entry `vt[index - 1]` (at
/// `*(obj + vptr_offset)`) supplies its own delta (low half) and the
/// function (high word).
fn pmf_resolve(c: &mut Cpu, obj_reg: usize, off: u32) {
    // a1 = index, v1 = delta, a2 = function, s2 = vtable entry, v0 = this
    // offset, as the original leaves them.
    c.cycles += 3;
    c.g[A1][0] = c.lh(at(c, obj_reg, off + 2));
    c.g[V1][0] = sx(r32(c, A1) << 3);
    let is_virtual = (c.g[A1][0] as i64) >= 0;
    if is_virtual {
        c.cycles += 8;
        c.g[V0][0] = c.lh(at(c, obj_reg, off + 4));
        add_regs(c, V0, obj_reg, V0);
        c.g[A0][0] = c.lw(r32(c, V0));
        add_regs(c, V1, V1, A0);
        c.g[S2][0] = c.ld(at(c, V1, 0u32.wrapping_sub(8)));
        c.g[A2][0] = ((c.g[S2][0] as i64) >> 32) as u64;
        c.g[V1][0] = c.lh(at(c, obj_reg, off));
    } else {
        c.cycles += 2;
        c.g[A2][0] = c.lw(at(c, obj_reg, off + 4));
        c.g[V1][0] = c.lh(at(c, obj_reg, off));
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[S2][0] & 0xffff;
    if is_virtual {
        c.cycles += 4;
        c.g[V0][0] = sx(((r32(c, V0) << 16) as i32 >> 16) as u32);
        add_regs(c, V0, V0, V1);
    } else {
        c.cycles += 1;
        c.g[V0][0] = c.g[V1][0];
    }
}

/// `void ComputerActorRider_ThinkCruise(CAR *this, Control *ctl)`
/// (0x10a960): the Cruise sub-state. Requests a path reset when lost,
/// decides on a shove (an attack target on the left/right sets
/// 0x40000/0x80000; shoving back sets both), then lets the current
/// behaviour (`behaviourFn`) steer.
pub fn computer_actor_rider_think_cruise(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        // s0 = this, s1 = ctl
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        callee::SHOULD_RESET_PATH.call(c, 0x0010_a980)?;
        merge_reset_request(c, S1);
        c.g[A1][0] = c.g[SP][0]; // &side (a word in the frame)
        c.g[A0][0] = c.g[S0][0];
        c.sw(r32(c, S1), c.g[V1][0]);
        c.sw(at(c, S0, car::NO_LATE_TRICK), 0);
        callee::FIND_ATTACK_TARGET.call(c, 0x0010_a9ac)?;
        let found = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(r32(c, SP));
        let shove = if found {
            c.cycles += 2;
            let right = c.g[V0][0] != 0;
            c.g[V1][0] = control::SHOVE_RIGHT;
            if right {
                c.cycles += 3;
            } else {
                c.cycles += 4;
                c.g[V1][0] = control::SHOVE_LEFT;
            }
            c.g[V0][0] = c.lw(r32(c, S1)) | c.g[V1][0];
            true
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::SHOVE_BACK.call(c, 0x0010_a9e0)?;
            let back = c.g[V0][0] != 0;
            c.g[V1][0] = control::SHOVE_RIGHT;
            if back {
                c.cycles += 4;
                c.g[A0][0] = control::SHOVE_LEFT;
                c.g[V0][0] = c.lw(r32(c, S1)) | c.g[V1][0] | c.g[A0][0];
            }
            back
        };
        if shove {
            c.cycles += 1;
            c.sw(r32(c, S1), c.g[V0][0]);
        }
        // (this->*behaviourFn)(ctl)
        pmf_resolve(c, S0, car::BEHAVIOUR_FN);
        c.cycles += 9;
        add_regs(c, A0, S0, V0);
        let func = r32(c, A2);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, func, 0x0010_aa58)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void ComputerActorRider_ThinkWipeout(CAR *this, Control *ctl)`
/// (0x10b750): only the path-reset request. 16 cycles.
pub fn computer_actor_rider_think_wipeout(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::SHOULD_RESET_PATH.call(c, 0x0010_b764)?;
        frame.restore_ra(c, 0);
        merge_reset_request(c, S0);
        c.sw(r32(c, S0), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `Vec4 *ComputerActorRider_GetSteerTarget(Vec4 *out, CAR *this)`
/// (0x100610): the point the CPU steers at, the AI-path look-ahead
/// (`rider+0x4b0`) moved by a tenth of the offset between the nearest path
/// point (`rider+0x490`) and the rider position (`rider+0x110`). 27 cycles.
/// The compiler bounces every vector through the 0x30-byte frame.
pub fn computer_actor_rider_get_steer_target(c: &mut Cpu) {
    let lanes = |a: Vec4, b: Vec4, f: fn(VuF32, VuF32) -> VuF32| Vec4 {
        x: f(a.x, b.x),
        y: f(a.y, b.y),
        z: f(a.z, b.z),
        w: f(a.w, b.w),
    };
    c.cycles += 27;
    let frame = Frame::open(c, 0x30);
    let out = r32(c, A0);
    let rider = c.lw(at(c, A1, car::RIDER)) as u32;
    c.g[V0][0] = sx(rider);
    let nearest = Addr(rider.wrapping_add(0x490)).read_vec4(c);
    let pos = Addr(rider.wrapping_add(0x110)).read_vec4(c);
    let offset = lanes(nearest, pos, |a, b| a - b);
    Addr(frame.at(c, 0x10)).write_vec4(c, offset);
    let blend = Addr(gp_global(c, global::STEER_BLEND)).read_fpu(c);
    abi::set_fpr(c, 0, blend);
    c.g[V0][0] = c.g[A0][0];
    Addr(frame.at(c, 0)).write_vec4(c, offset);
    c.g[A2][0] = abi::mfc1(blend);
    let k = VuF32(blend.0);
    let scaled = Vec4 { x: offset.x * k, y: offset.y * k, z: offset.z * k, w: offset.w * k };
    Addr(frame.at(c, 0x20)).write_vec4(c, scaled);
    c.g[A2][0] = c.lw(at(c, A1, car::RIDER));
    Addr(frame.at(c, 0)).write_vec4(c, scaled);
    Addr(frame.at(c, 0x10)).write_vec4(c, scaled);
    let ahead = Addr(at(c, A2, 0x4b0)).read_vec4(c);
    let scaled = Addr(frame.at(c, 0)).read_vec4(c);
    let target = lanes(ahead, scaled, |a, b| a + b);
    Addr(frame.at(c, 0x10)).write_vec4(c, target);
    let q = c.bus.read128(frame.at(c, 0x10));
    abi::set_gpr128(c, V1, q);
    c.bus.write128(out, q);
    vu0_scratch::set_vf(c, 3, 0b1111, ahead);
    vu0_scratch::set_vf(c, 4, 0b1111, scaled);
    vu0_scratch::set_vf(c, 5, 0b1111, target);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x30));
    c.pc = ra;
}

/// `Rider *ComputerActorRider_GetHumanRider(CAR *this)` (0x10deb0): the
/// human rider this CPU measures itself against, or null.
pub fn computer_actor_rider_get_human_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::FIND_HUMAN_RIDER_INDEX.call(c, 0x0010_dec0)?;
        let none = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = c.lw(gp_global(c, global::APP));
        if none {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 6;
            c.g[A1][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = c.lw(at(c, A0, 0x84));
            c.g[V1][0] = c.lw(at(c, V0, 0xc));
            add_regs(c, V1, V1, A1);
            c.g[V0][0] = c.lw(at(c, V1, ai_mgr::RIDERS));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AirPredictor_SetLaunchState(AirPredictor *ap, Vec4 *pos, Vec4
/// *vel)` (0x113618): start a new prediction from the take-off: phase 0,
/// rider position/velocity copies (+0x70/+0x80) and the simulation's start
/// point/velocity (+0x50/+0x60), predicted air time = elapsed (+0x98 =
/// +0xa0). 12 cycles.
pub fn air_predictor_set_launch_state(c: &mut Cpu) {
    c.cycles += 12;
    let ap = r32(c, A0);
    let (pos, vel) = (r32(c, A1), r32(c, A2));
    c.sw(ap.wrapping_add(0xac), 0);
    let elapsed = Addr(ap.wrapping_add(0xa0)).read_fpu(c);
    abi::set_fpr(c, 0, elapsed);
    c.lq(V0, pos);
    c.sq(V0, ap.wrapping_add(0x70));
    c.lq(V1, vel);
    c.sq(V1, ap.wrapping_add(0x80));
    c.lq(V0, pos);
    c.sq(V0, ap.wrapping_add(0x50));
    c.lq(V1, vel);
    Addr(ap.wrapping_add(0x98)).write_fpu(c, elapsed);
    c.sq(V1, ap.wrapping_add(0x60));
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{covered, machine, vtable, FAKE, LOG_RESULTS};
    use crate::difftest::{compare, fill, fresh, put_vec, put_words, Rng, DATA, GAME_GP};
    use std::collections::BTreeSet;

    const SEEDS: u32 = 300;
    /// Where setups note which case a seed exercises (untouched by the code
    /// under test).
    const CASE: u32 = 0x01e0_0000;

    /// Require every case `0..n` (as `classify` reads it after the
    /// generated function ran) to be taken by some seed.
    fn require_cases(gen: GuestFn, n: u32, setup: &dyn Fn(&mut Cpu, u32), classify: &dyn Fn(&mut Cpu) -> u32) {
        let mut seen = BTreeSet::new();
        for seed in 0..SEEDS {
            let mut c = fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, gen);
            seen.insert(classify(&mut c));
        }
        for i in 0..n {
            assert!(seen.contains(&i), "case {i} never taken (seen {seen:?})");
        }
    }

    /// `check_lohi`, and the caller-saved temporaries must match too (the
    /// replacements reproduce every register; only the VU0 flags stay
    /// report-only).
    fn check_lohi(gen: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(gen, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
            let soft: Vec<&String> = r.soft.iter().filter(|m| !m.starts_with("vu0 ctrl 1")).collect();
            assert!(soft.is_empty(), "seed {seed} (temporaries): {:#?}", soft);
        }
    }

    /// The case the setup noted.
    fn noted(c: &mut Cpu) -> u32 {
        c.bus.read32(CASE)
    }

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    // ------------------------------------------------------------ fixtures

    /// The back end's tables: player slots, the player table, rider
    /// records (characters mostly 0..9, some profiles -1), attributes,
    /// character maxima (some 0), the rivalry table (0..4), the track.
    fn be_tables(c: &mut Cpu, r: &mut Rng) {
        // Noise around the tables first (the slot table and rider records
        // lie inside these ranges and are written below).
        fill(c, r, ATTRIBS - 0x400, 0x400);
        fill(c, r, CHARACTER_MAX - 0x800, 0x400);
        for p in 0..8 {
            c.bus.write32(0x0053_05b0 + 4 * p, r.next_u32() % 8);
        }
        fill(c, r, player_table::BASE, 8 * 0x1c / 4);
        let all_cpu = r.next_u32() % 5 == 0;
        for s in 0..8 {
            if all_cpu || r.next_u32() % 2 == 0 {
                c.bus.write32(player_table::BASE + s * 0x1c + 0xc, r.next_u32() | 0x8000_0000);
            } else {
                c.bus.write32(player_table::BASE + s * 0x1c + 0xc, r.next_u32() % 4);
            }
        }
        fill(c, r, 0x0053_5b20, 16 * 0x1c / 4);
        for s in 0..16 {
            let rec = 0x0053_5b20 + s * 0x1c;
            if r.next_u32() % 2 == 0 {
                c.bus.write32(rec + 0xc, u32::MAX);
            }
            let ch = if r.next_u32() % 8 == 0 { r.next_u32() as u8 } else { (r.next_u32() % 10) as u8 };
            c.bus.write8(rec + 0x11, ch);
        }
        for ch in 0..10 {
            for n in 0..7 {
                if r.next_u32() % 6 == 0 {
                    c.bus.write8(CHARACTER_MAX + ch * 15 + 8 + n, 0);
                }
            }
        }
        // Rivalry levels for characters 0..9 (others stay as noise).
        for p in 0..3u32 {
            for b in 0..10u32 {
                for a in 0..10u32 {
                    c.bus.write8(RIVALRY + p * 0x9b50 + b * 0xf88 + a * 3, (r.next_u32() % 5) as u8);
                }
            }
        }
        c.bus.write32(0x0053_5c08, r.next_u32() % 40);
        fill(c, r, 0x0043_d950, 40 * 0x64 / 4);
        for t in 0..40 {
            c.bus.write32(0x0043_d950 + t * 0x64 + 0x54, r.next_u32() % 4);
        }
        // The BE singletons exist.
        for g in [global::NEW_RACE_INT, global::NEW_PLAYER_INT, global::AGGRESSION_INT, global::CHAR_INT] {
            c.bus.write32(GAME_GP - g, DATA + 0xe_0000 + g * 0x10);
        }
    }

    /// `App → GameModule → AI` with `riders` riders (pointers at
    /// `rider(i)`), the frame counter `frame`. Returns the AI address.
    const APP: u32 = DATA + 0x4_0000;
    fn app_chain(c: &mut Cpu, frame: u32) -> u32 {
        let gm = APP + 0x100;
        let ai = APP + 0x200;
        let race = APP + 0x400;
        c.bus.write32(GAME_GP - global::APP, APP);
        c.bus.write32(APP + 0x84, gm);
        c.bus.write32(gm + 0xc, ai);
        c.bus.write32(ai + 8, frame);
        c.bus.write32(race, 0);
        ai
    }

    // ------------------------------------------------------------ singletons

    /// A singleton test: hot (global set) or built (0); real allocator and
    /// constructors; the allocator's mutex syscalls are fakes that can
    /// unwind.
    fn singleton_test(gen: GuestFn, hand: GuestFn, global: u32, salt: u64) {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, 6);
            be_tables(c, &mut r);
            let built = seed % 3 != 0;
            c.bus.write32(CASE, built as u32);
            c.bus.write32(GAME_GP - global, if built { 0 } else { DATA + (r.next_u32() & 0xfff0) });
        };
        covered(gen, hand, SEEDS, &setup);
        check_lohi(gen, hand, SEEDS, &setup);
        require_cases(gen, 2, &setup, &|c| noted(c));
    }

    #[test]
    fn be_new_race_int_get_instance_matches() {
        singleton_test(ssx3_ee::s01::f_001448d8__generated, super::be_new_race_int_get_instance, global::NEW_RACE_INT, 1);
    }
    #[test]
    fn be_aggression_int_get_instance_matches() {
        singleton_test(ssx3_ee::s01::f_00155a50__generated, super::be_aggression_int_get_instance, global::AGGRESSION_INT, 2);
    }
    #[test]
    fn be_new_player_int_get_instance_matches() {
        singleton_test(ssx3_ee::s01::f_00145970__generated, super::be_new_player_int_get_instance, global::NEW_PLAYER_INT, 3);
    }
    #[test]
    fn be_char_int_get_instance_matches() {
        singleton_test(ssx3_ee::s01::f_00147f78__generated, super::be_char_int_get_instance, global::CHAR_INT, 4);
    }
    #[test]
    fn be_score_int_get_instance_matches() {
        singleton_test(ssx3_ee::s01::f_00154948__generated, super::be_score_int_get_instance, global::SCORE_INT, 5);
    }
    #[test]
    fn be_option_int_get_instance_matches() {
        singleton_test(ssx3_ee::s01::f_0014f250__generated, super::be_option_int_get_instance, global::OPTION_INT, 6);
    }
    /// Only the hot path and an unwind inside the first allocation: the
    /// rest of the build (`BELibrary_ctor` loads the character database
    /// from disc) cannot run on a test machine, so this one is not in
    /// `OVERRIDES`.
    #[test]
    fn be_library_get_instance_hot_path_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 7, 0);
            be_tables(c, &mut r);
            let built = seed % 3 != 0;
            c.bus.write32(CASE, built as u32);
            c.bus.write32(GAME_GP - global::BE_LIBRARY, if built { 0 } else { DATA + (r.next_u32() & 0xfff0) });
            if built {
                c.bus.write32(crate::difftest::fakes::LOG_UNWIND_AT, 0);
            }
        };
        covered(ssx3_ee::s01::f_0014dc80, super::be_library_get_instance, SEEDS, &setup);
        require_cases(ssx3_ee::s01::f_0014dc80, 2, &setup, &|c| noted(c));
    }

    // ------------------------------------------------------------ tables

    fn table_setup(salt: u64) -> impl Fn(&mut Cpu, u32) {
        move |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, 0);
            be_tables(c, &mut r);
            c.g[A1][0] = (r.next_u32() % 8) as u64;
            c.g[A2][0] = match r.next_u32() % 3 {
                0 => 0,
                1 => (r.next_u32() % 40) as u64,
                _ => sx(r.next_u32()),
            };
        }
    }

    #[test]
    fn be_new_player_int_table_getters_match() {
        check_lohi(ssx3_ee::s01::f_00147398__generated, super::be_new_player_int_get_character, SEEDS, &table_setup(10));
        check_lohi(ssx3_ee::s01::f_00147290__generated, super::be_new_player_int_get_pad_index, SEEDS, &table_setup(11));
        check_lohi(ssx3_ee::s01::f_00144c98__generated, super::be_new_race_int_get_track_peak, SEEDS, &table_setup(12));
    }

    #[test]
    fn be_char_int_get_attrib_matches() {
        let fns: [(GuestFn, GuestFn); 7] = [
            (ssx3_ee::s01::f_00148410__generated, super::be_char_int_get_attrib0),
            (ssx3_ee::s01::f_001484e0__generated, super::be_char_int_get_attrib1),
            (ssx3_ee::s01::f_001485b0__generated, super::be_char_int_get_attrib2),
            (ssx3_ee::s01::f_00148680__generated, super::be_char_int_get_attrib3),
            (ssx3_ee::s01::f_00148750__generated, super::be_char_int_get_attrib4),
            (ssx3_ee::s01::f_00148820__generated, super::be_char_int_get_attrib5),
            (ssx3_ee::s01::f_001488f0__generated, super::be_char_int_get_attrib6),
        ];
        for (i, (g, h)) in fns.into_iter().enumerate() {
            check_lohi(g, h, SEEDS, &table_setup(20 + i as u64));
        }
    }

    #[test]
    fn be_char_int_get_attrib_factor_matches() {
        let fns: [(u32, GuestFn, GuestFn); 11] = [
            (0x0014_8d80, ssx3_ee::s01::f_00148d80__generated, super::be_char_int_get_attrib3_factor),
            (0x0014_8e68, ssx3_ee::s01::f_00148e68__generated, super::be_char_int_get_attrib3_factor_2),
            (0x0014_8f50, ssx3_ee::s01::f_00148f50__generated, super::be_char_int_get_attrib5_factor),
            (0x0014_9038, ssx3_ee::s01::f_00149038__generated, super::be_char_int_get_attrib5_factor_2),
            (0x0014_9120, ssx3_ee::s01::f_00149120__generated, super::be_char_int_get_attrib6_factor),
            (0x0014_9208, ssx3_ee::s01::f_00149208__generated, super::be_char_int_get_attrib6_factor_2),
            (0x0014_92f0, ssx3_ee::s01::f_001492f0__generated, super::be_char_int_get_attrib6_factor_3),
            (0x0014_93d8, ssx3_ee::s01::f_001493d8__generated, super::be_char_int_get_attrib1_factor),
            (0x0014_94c0, ssx3_ee::s01::f_001494c0__generated, super::be_char_int_get_attrib0_factor),
            (0x0014_95a8, ssx3_ee::s01::f_001495a8__generated, super::be_char_int_get_attrib4_factor),
            (0x0014_9690, ssx3_ee::s01::f_00149690__generated, super::be_char_int_get_attrib2_factor),
        ];
        for (i, (start, g, h)) in fns.into_iter().enumerate() {
            let base = table_setup(30 + i as u64);
            let setup = move |c: &mut Cpu, seed: u32| {
                base(c, seed);
                c.bus.write32(CASE, ((c.g[A2][0] as i64) > 0) as u32);
                c.pc = start;
            };
            check_lohi(g, h, SEEDS, &setup);
            // fixed (0.5), value given, value from the attribute
            require_cases(g, 3, &setup, &|c| {
                if c.f[0] == 0x3f00_0000 && c.g[AT][0] == 0x3f00_0000 {
                    0
                } else {
                    2 - noted(c)
                }
            });
        }
    }

    #[test]
    fn be_aggression_int_get_rival_level_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            table_setup(40)(c, seed);
            c.g[A2][0] = (seed / 8 % 8) as u64;
        };
        check_lohi(ssx3_ee::s01::f_00155b50__generated, super::be_aggression_int_get_rival_level, SEEDS, &setup);
        // featured character (3) or the table
        require_cases(ssx3_ee::s01::f_00155b50__generated, 2, &setup, &|c| {
            (r32(c, V1).wrapping_sub(0x004a_6ca8) < 0x4_0000) as u32
        });
    }

    // ------------------------------------------------------------ AI loops

    #[test]
    fn be_new_player_int_get_human_rider_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 50, 0);
            be_tables(c, &mut r);
            c.g[A1][0] = if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() as u64 | 1 };
            c.bus.write32(CASE, (c.g[A1][0] != 0) as u32);
            schedule(c, &mut r, seed, 0x0014_6e98)
        };
        check_events(ssx3_ee::s01::f_00146e98__generated, super::be_new_player_int_get_human_rider, SEEDS, &setup);
        check_lohi(ssx3_ee::s01::f_00146e98__generated, super::be_new_player_int_get_human_rider, SEEDS, &|c, s| {
            setup(c, s);
        });
        // none, first, second
        require_cases(ssx3_ee::s01::f_00146e98__generated, 3, &|c, s| { setup(c, s); }, &|c| {
            if c.g[V0][0] == u64::MAX { 0 } else { 1 + noted(c) }
        });
    }

    fn ai_players(c: &mut Cpu, r: &mut Rng, ai: u32) {
        let n = r.next_u32() % 5;
        c.bus.write32(ai + ai_mgr::PLAYER_COUNT, if r.next_u32() % 8 == 0 { r.next_u32() | 0x8000_0000 } else { n });
        for i in 0..4 {
            let player = DATA + 0x1000 * (i + 1);
            let state = DATA + 0x8000 + 0x1000 * i;
            c.bus.write32(ai + ai_mgr::PLAYERS + 4 * i, player);
            c.bus.write32(player + 0x18, state);
            let t = match r.next_u32() % 4 {
                0 => r.edge_f32(),
                1 => f(-1.0),
                _ => r.f32(500.0) & 0x7fff_ffff,
            };
            c.bus.write32(state + 0x470, t);
        }
    }

    #[test]
    fn ai_all_players_finished_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 60, 0);
            let ai = DATA + 0x100;
            ai_players(c, &mut r, ai);
            c.g[A0][0] = ai as u64;
            let mode = schedule(c, &mut r, seed, 0x0012_a250);
            if mode != Events::Off {
                c.next_event = c.cycles + (r.next_u32() % 16) as u64;
            }
            mode
        };
        check_events(ssx3_ee::s00::f_0012a250__generated, super::ai_all_players_finished, SEEDS, &setup);
        require_cases(ssx3_ee::s00::f_0012a250__generated, 3, &|c, s| { setup(c, s); }, &|c| {
            if c.g[V0][0] == 0 { 0 } else if c.g[A2][0] == 0 { 1 } else { 2 }
        });
    }

    fn ai_riders(c: &mut Cpu, r: &mut Rng, ai: u32) {
        let n = r.next_u32() % 7;
        c.bus.write32(ai + ai_mgr::RIDER_COUNT, if r.next_u32() % 10 == 0 { r.next_u32() | 0x8000_0000 } else { n });
        for i in 0..6 {
            let rider = DATA + 0x2_0000 + 0x1000 * i;
            c.bus.write32(ai + ai_mgr::RIDERS + 4 * i, rider);
            c.bus.write32(rider + 0x880, if r.next_u32() % 4 == 0 { r.next_u32() % 9 } else { 7 });
        }
    }

    #[test]
    fn ai_are_all_riders_loaded_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 70, 0);
            let ai = DATA + 0x100;
            ai_riders(c, &mut r, ai);
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0012_a180)
        };
        check_events(ssx3_ee::s00::f_0012a180__generated, super::ai_are_all_riders_loaded, SEEDS, &setup);
        require_cases(ssx3_ee::s00::f_0012a180__generated, 2, &|c, s| { setup(c, s); }, &|c| c.g[V0][0] as u32);
    }

    #[test]
    fn ai_get_frame_counter_matches() {
        check_lohi(ssx3_ee::s00::f_001298c8__generated, super::ai_get_frame_counter, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 80, 0);
            app_chain(c, r.next_u32());
        });
    }

    #[test]
    fn ai_update_race_state_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 90, 3);
            let sm = DATA + 0x100;
            let id = r.next_u32() % 4;
            c.bus.write32(sm, id);
            c.bus.write32(sm + 4, if r.next_u32() % 2 == 0 { id } else { r.next_u32() % 4 });
            for (off, obj) in [(0x98, DATA + 0x400), (0x9c, DATA + 0x500)] {
                let vt = vtable(c, &mut r, &[0x8, 0x10, 0x18]);
                c.bus.write32(obj, vt);
                c.bus.write32(sm + off, obj);
            }
            if r.next_u32() % 3 == 0 {
                c.bus.write32(sm + 0x9c, 0);
            }
            c.g[A0][0] = sm as u64;
        };
        covered(ssx3_ee::s00::f_00113c20__generated, super::ai_update_race_state, SEEDS, &setup);
        // same state; change without / with an old state
        let no_unwind = |c: &mut Cpu, seed: u32| {
            setup(c, seed);
            c.bus.write32(crate::difftest::fakes::LOG_UNWIND_AT, u32::MAX);
        };
        // fake calls: update only; enter + update; exit + enter + update
        require_cases(ssx3_ee::s00::f_00113c20__generated, 3, &no_unwind, &|c| {
            c.bus.read32(crate::difftest::fakes::LOG_COUNT).wrapping_sub(1)
        });
    }

    #[test]
    fn ai_update_rivals_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 100, 0);
            be_tables(c, &mut r);
            let ai = DATA + 0x100;
            ai_riders(c, &mut r, ai);
            c.bus.write32(ai + ai_mgr::RIVALS_ENABLED, if r.next_u32() % 6 == 0 { 0 } else { 1 });
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0010_f878)
        };
        check_events(ssx3_ee::s00::f_0010f878__generated, super::ai_update_rivals, SEEDS, &setup);
        // some rider got a sworn rival
        require_cases(ssx3_ee::s00::f_0010f878__generated, 2, &|c, s| { setup(c, s); }, &|c| {
            (0..6).any(|i| c.bus.read32(DATA + 0x2_0000 + 0x1000 * i + 0xf0) == 1) as u32
        });
    }

    // ------------------------------------------------------------ CPU riders

    const THIS: u32 = DATA + 0x1_0000;
    const RIDER: u32 = DATA + 0x5_0000;
    const OWNER: u32 = DATA + 0x6_0000;
    const CTL: u32 = DATA + 0x200;

    /// A CPU rider whose path-reset check, attack-target search and shove
    /// decision take their different outcomes (case noted at CASE: 0 no
    /// shove, 1 target left, 2 target right, 3 shove back).
    fn car_setup(c: &mut Cpu, r: &mut Rng) {
        be_tables(c, r);
        fill(c, r, THIS, 0x1000 / 4);
        fill(c, r, RIDER, 0xb40 / 4);
        c.bus.write32(THIS + 0x18, RIDER);
        // ShouldResetPath inputs.
        c.bus.write32(RIDER + 0x1b0, if r.next_u32() % 2 == 0 { 0 } else { r.f32(10.0) });
        c.bus.write32(RIDER + 0x1b4, if r.next_u32() % 2 == 0 { 0 } else { r.f32(10.0) });
        c.bus.write32(RIDER + 0x4cc, r.f32(4.0));
        c.bus.write32(RIDER + 0x4c8, r.f32(2000.0) & 0x7fff_ffff);
        c.bus.write32(RIDER + 0x77c, OWNER);
        c.bus.write32(OWNER + 0xde0, r.next_u32() % 6);
        c.bus.write32(OWNER + 0xde4, r.next_u32() % 10);
        c.bus.write32(RIDER + 0x788, OWNER + 0x100);
        c.bus.write32(OWNER + 0x1a0, r.f32(60.0));
        c.bus.write32(THIS + 0xe70, if r.next_u32() % 2 == 0 { u32::MAX } else { 1 });
        c.bus.write32(THIS + 0xe74, r.next_u32() % 300);
        c.bus.write32(THIS + 0xe78, r.next_u32() % 100);
        // FindAttackTarget: a target only on frames divisible by 6.
        let attack = r.next_u32() % 3 == 0;
        let frame = if attack { 600 } else { 601 + r.next_u32() % 5 };
        let ai = app_chain(c, frame);
        c.bus.write32(ai + ai_mgr::RIDER_COUNT, 1);
        let vt = vtable(c, r, &[0x38]);
        c.bus.write32(RIDER + 0x6c0, vt);
        put_words(c, GAME_GP - 0x7ce0, &[f(3.14), f(1.0), f(100.0)]);
        c.bus.write32(RIDER + 8, 0);
        c.bus.write32(RIDER + 0x1b0, 0);
        c.bus.write32(RIDER + 0x1b4, 0);
        let left = r.next_u32() % 2 == 0;
        c.bus.write32(RIDER + 0xc, f(if left { 10.0 } else { -10.0 }));
        c.bus.write32(RIDER + 0xf4, 0);
        // Fake results: the rider's own index (5), then rider a = 2.
        put_words(c, LOG_RESULTS, &[5, 2]);
        for p in 0..3 {
            for b in 0..10 {
                for a in 0..10 {
                    c.bus.write8(RIVALRY + p * 0x9b50 + b * 0xf88 + a * 3, 4);
                }
            }
        }
        // ShoveBack: cooldown running (returns +0xf34), negative, or idle.
        let cooldown = match r.next_u32() % 3 {
            0 => f(1.0),
            1 => f(-1.0),
            _ if !attack => 0,
            _ => f(0.5),
        };
        c.bus.write32(THIS + 0xf30, cooldown);
        c.bus.write32(THIS + 0xf34, r.next_u32() % 2);
        // The behaviour: plain function or virtual.
        c.bus.write16(THIS + 0xf44, ((r.next_u32() % 8) * 4) as u16);
        if r.next_u32() % 2 == 0 {
            c.bus.write16(THIS + 0xf46, 0xffff);
            c.bus.write32(THIS + 0xf48, FAKE + 0x40);
        } else {
            let index = 1 + r.next_u32() % 4;
            c.bus.write16(THIS + 0xf46, index as u16);
            c.bus.write16(THIS + 0xf48, 0x20);
            let bvt = DATA + 0x7000;
            c.bus.write32(THIS + 0x20, bvt);
            c.bus.write16(bvt + (index - 1) * 8, ((r.next_u32() % 8) * 4) as u16);
            c.bus.write32(bvt + (index - 1) * 8 + 4, FAKE + 0x50);
        }
        c.bus.write32(CTL, r.next_u32() & !0x000c_0000);
        c.g[A0][0] = THIS as u64;
        c.g[A1][0] = CTL as u64;
    }

    #[test]
    fn computer_actor_rider_think_cruise_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 110, 4);
            car_setup(c, &mut r);
        };
        covered(ssx3_ee::s00::f_0010a960__generated, super::computer_actor_rider_think_cruise, SEEDS, &setup);
        check_lohi(ssx3_ee::s00::f_0010a960__generated, super::computer_actor_rider_think_cruise, SEEDS, &setup);
        let no_unwind = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 110, 0);
            car_setup(c, &mut r);
        };
        // shove bits: none, left, right, both; reset bit 0/1; virtual PMF
        require_cases(ssx3_ee::s00::f_0010a960__generated, 8, &no_unwind, &|c| {
            let w = c.bus.read32(CTL);
            ((w >> 18) & 3) | ((w >> 12) & 1) << 2
        });
        require_cases(ssx3_ee::s00::f_0010a960__generated, 2, &no_unwind, &|c| (c.bus.read16(THIS + 0xf46) as i16 >= 0) as u32);
    }

    #[test]
    fn computer_actor_rider_think_wipeout_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 120, 0);
            car_setup(c, &mut r);
        };
        check_lohi(ssx3_ee::s00::f_0010b750__generated, super::computer_actor_rider_think_wipeout, SEEDS, &setup);
        require_cases(ssx3_ee::s00::f_0010b750__generated, 2, &setup, &|c| (c.bus.read32(CTL) >> 12) & 1);
    }

    #[test]
    fn computer_actor_rider_get_human_rider_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 130, 0);
            let ai = app_chain(c, 0);
            fill(c, &mut r, ai + ai_mgr::RIDERS, 6);
            let rel = DATA + 0x3000;
            c.bus.write32(THIS + 0x18, rel);
            for i in 0..6 {
                for w in 0..2 {
                    c.bus.write32(rel + 0x24 * i + 4 * w, if r.next_u32() % 4 == 0 { 0 } else { r.next_u32() });
                }
            }
            c.g[A0][0] = THIS as u64;
            schedule(c, &mut r, seed, 0x0010_deb0)
        };
        // No loop of its own (events reach the callee's loop only).
        check_lohi(ssx3_ee::s00::f_0010deb0__generated, super::computer_actor_rider_get_human_rider, SEEDS, &|c, s| {
            setup(c, s);
        });
        require_cases(ssx3_ee::s00::f_0010deb0__generated, 2, &|c, s| { setup(c, s); }, &|c| (c.g[V0][0] != 0) as u32);
    }

    #[test]
    fn computer_actor_rider_get_steer_target_matches() {
        check_lohi(ssx3_ee::s00::f_00100610__generated, super::computer_actor_rider_get_steer_target, 1000, &|c, seed| {
            let mut r = machine(c, seed, 140, 0);
            c.bus.write32(THIS + 0x18, RIDER);
            for off in [0x110, 0x490, 0x4b0] {
                let v = if seed % 4 == 0 {
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]
                } else {
                    [r.f32(1e5), r.f32(1e5), r.f32(1e5), r.f32(2.0)]
                };
                put_vec(c, RIDER + off, v);
            }
            c.bus.write32(GAME_GP - global::STEER_BLEND, if seed % 3 == 0 { r.edge_f32() } else { 0x3dcc_cccd });
            c.g[A0][0] = (DATA + 0x800) as u64;
            c.g[A1][0] = THIS as u64;
        });
    }

    #[test]
    fn air_predictor_set_launch_state_matches() {
        check_lohi(ssx3_ee::s00::f_00113618__generated, super::air_predictor_set_launch_state, 1000, &|c, seed| {
            let mut r = machine(c, seed, 150, 0);
            let ap = DATA + 0x1000 + (r.next_u32() % 4) * 0x10;
            fill(c, &mut r, DATA + 0x1000, 0x100);
            c.g[A0][0] = ap as u64;
            // pos/vel elsewhere, or inside the predictor (overlap)
            c.g[A1][0] = (DATA + 0x1000 + (r.next_u32() % 0x100) * 4) as u64;
            c.g[A2][0] = (DATA + 0x1000 + (r.next_u32() % 0x100) * 4) as u64;
        });
    }
}
