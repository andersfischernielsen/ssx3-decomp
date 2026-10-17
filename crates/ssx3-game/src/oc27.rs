//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Hand-written, typed replacements for small generated functions, each with a
//! differential test against the generated body.

use crate::events::deliver_event_here;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_abe0, be_bolt_int_get_instance, "BEBoltInt_GetInstance"),
    (0x0014_e0e0, be_network_int_get_instance, "BENetworkInt_GetInstance"),
    (0x0015_08d8, be_econ_int_get_instance, "BEEconInt_GetInstance"),
    (0x0015_32a0, be_mission_int_get_instance, "BEMissionInt_GetInstance"),
    (0x0015_c678, be_char_int_destroy_instance, "BECharInt_DestroyInstance"),
    (0x0015_c6e8, be_bagt_int_destroy_instance, "BEBAGTInt_DestroyInstance"),
    (0x0015_c7e0, be_aggression_int_destroy_instance, "BEAggressionInt_DestroyInstance"),
    (0x0015_bcd0, be_mission_int_destroy_instance, "BEMissionInt_DestroyInstance"),
    (0x0014_ac30, beb_bolt_int_get_rider_character, "BEBoltInt_GetRiderCharacter"),
    (0x0014_ac50, beb_bolt_int_get_rider_profile, "BEBoltInt_GetRiderProfile"),
    (0x0014_ac70, beb_bolt_int_get_rider_profile_and_char, "BEBoltInt_GetRiderProfileAndChar"),
    (0x0025_15f0, avalanche_menu_on_activate, "AvalancheMenu_OnActivate"),
    (0x0025_1610, avalanche_menu_on_deactivate, "AvalancheMenu_OnDeactivate"),
    (0x002f_7e70, avalanche_destroy_all_thunk, "Avalanche_DestroyAllThunk"),
    (0x0014_ee28, be_get_rider_char_name, "BE_GetRiderCharName"),
    (0x0014_ee58, be_get_rider_display_name, "BE_GetRiderDisplayName"),
    (0x0015_4240, be_mission_int_find_mission, "BEMissionInt_FindMission"),
    (0x0015_42a0, be_mission_int_count_missions_on_track, "BEMissionInt_CountMissionsOnTrack"),
    (0x0015_4368, be_mission_int_count_missions_on_peak, "BEMissionInt_CountMissionsOnPeak"),
    (0x0015_3688, be_mission_int_count_collected_on_peak_for, "BEMissionInt_CountCollectedOnPeakFor"),
    (0x0015_3f30, be_mission_int_get_mission_track, "BEMissionInt_GetMissionTrack"),
    (0x0015_3d78, be_mission_int_is_mission_complete, "BEMissionInt_IsMissionComplete"),
    (0x0015_3dd0, be_mission_int_get_mission_flag1, "BEMissionInt_GetMissionFlag1"),
    (0x0015_3e28, be_mission_int_get_mission_flag0, "BEMissionInt_GetMissionFlag0"),
    (0x0015_3e80, be_mission_int_get_mission_flag2, "BEMissionInt_GetMissionFlag2"),
    (0x0015_3ed8, be_mission_int_get_mission_flag4, "BEMissionInt_GetMissionFlag4"),
    (0x0015_4010, be_mission_int_set_mission_flag0, "BEMissionInt_SetMissionFlag0"),
    (0x0015_4080, be_mission_int_set_mission_flag1, "BEMissionInt_SetMissionFlag1"),
    (0x0015_40f0, be_mission_int_set_mission_flag2, "BEMissionInt_SetMissionFlag2"),
    (0x0015_4160, be_mission_int_set_mission_complete, "BEMissionInt_SetMissionComplete"),
    (0x0015_41d0, be_mission_int_set_mission_flag4, "BEMissionInt_SetMissionFlag4"),
    (0x0015_45b0, be_mission_int_get_nth_mission_on_track, "BEMissionInt_GetNthMissionOnTrack"),
    (0x0015_45f8, be_mission_int_get_session_type, "BEMissionInt_GetSessionType"),
    (0x0015_4760, be_mission_int_get_session_name, "BEMissionInt_GetSessionName"),
    (0x0015_4678, be_mission_int_get_session_info, "BEMissionInt_GetSessionInfo"),
    (0x0015_3c88, be_mission_int_get_mission_text1, "BEMissionInt_GetMissionText1"),
    (0x0015_3cd8, be_mission_int_get_mission_text2, "BEMissionInt_GetMissionText2"),
    (0x0015_3d28, be_mission_int_get_mission_text3, "BEMissionInt_GetMissionText3"),
    (0x0014_83a0, be_char_int_set_attrib0, "BECharInt_SetAttrib0"),
    (0x0014_8470, be_char_int_set_attrib1, "BECharInt_SetAttrib1"),
    (0x0014_8540, be_char_int_set_attrib2, "BECharInt_SetAttrib2"),
    (0x0014_8610, be_char_int_set_attrib3, "BECharInt_SetAttrib3"),
    (0x0014_86e0, be_char_int_set_attrib4, "BECharInt_SetAttrib4"),
    (0x0014_87b0, be_char_int_set_attrib5, "BECharInt_SetAttrib5"),
    (0x0014_8880, be_char_int_set_attrib6, "BECharInt_SetAttrib6"),
    (0x0015_c8f0, be_global_ctors, "BE_GlobalCtors"),
    (0x0015_c910, be_global_dtors, "BE_GlobalDtors"),
    (0x0014_a130, be_find_rider_by_profile, "BE_FindRiderByProfile"),
    (0x0014_ad50, beb_bolt_int_get_rider_profile_data, "BEBoltInt_GetRiderProfileData"),
    (0x0014_ae60, beb_bolt_int_get_rider_bolt_states, "BEBoltInt_GetRiderBoltStates"),
    (0x0014_acb0, beb_bolt_int_set_bolt_states, "BEBoltInt_SetBoltStates"),
    (0x0015_0960, be_econ_int_get_rider_money, "BEEconInt_GetRiderMoney"),
    (0x0015_09f8, be_econ_int_get_rider_total_earned, "BEEconInt_GetRiderTotalEarned"),
    (0x0015_0ae0, be_econ_int_add_rider_money, "BEEconInt_AddRiderMoney"),
    (0x0015_5ab0, be_aggression_int_get_rival_state, "BEAggressionInt_GetRivalState"),
    (0x0015_67b8, be_reset_profile_characters, "BE_ResetProfileCharacters"),
    (0x0015_6810, be_reset_profile_rivalries, "BE_ResetProfileRivalries"),
    (0x0015_6950, be_backup_character, "BE_BackupCharacter"),
    (0x0015_6988, be_restore_character, "BE_RestoreCharacter"),
    (0x0014_e0c0, be_library_read_save_data, "BELibrary_ReadSaveData"),
    (0x0015_3498, be_mission_int_get_collected_count, "BEMissionInt_GetCollectedCount"),
    (0x0015_35c0, be_mission_int_count_collected_on_peak, "BEMissionInt_CountCollectedOnPeak"),
    (0x0015_3858, be_mission_int_get_collected_mask, "BEMissionInt_GetCollectedMask"),
    (0x0015_38e8, be_mission_int_is_collected, "BEMissionInt_IsCollected"),
    (0x0015_42e0, be_mission_int_count_completed_on_track, "BEMissionInt_CountCompletedOnTrack"),
    (0x0015_44d0, be_mission_int_count_completed_on_peak_for, "BEMissionInt_CountCompletedOnPeakFor"),
    (0x0014_e050, be_library_write_save_data, "BELibrary_WriteSaveData"),
    (0x0014_8aa8, be_char_int_set_all_attribs, "BECharInt_SetAllAttribs"),
    (0x0014_8098, be_char_int_reset, "BECharInt_Reset"),
    (0x0014_f900, be_bagt_int_get_instance, "BEBAGTInt_GetInstance"),
];

/// `lw` through the CPU load path (sign-extended 32-bit load).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

// ---------------------------------------------------------------------------
// Callees / globals.

/// `EA_MemAllocNamed` (cross-shard, called through the function table).
const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
/// `__builtin_delete`.
const BUILTIN_DELETE: u32 = 0x0031_7e50;

const BE_GET_RIDER_CHARACTER: Callee = Callee { addr: 0x0014_a080, f: ssx3_ee::s01::f_0014a080 };
const BE_GET_RIDER_PROFILE: Callee = Callee { addr: 0x0014_a0e0, f: ssx3_ee::s01::f_0014a0e0 };

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

/// `BENetworkInt *BENetworkInt_GetInstance(void)` (0x14e0e0).
pub fn be_network_int_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1eec));
        let have = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !have {
            c.cycles += 11;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx(0x460000u32.wrapping_sub(0x59b8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, EA_MEM_ALLOC_NAMED, 0x0014_e108)?;
            c.g[V1][0] = 0x460000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
            c.g[V1][0] = sx(0x460000u32.wrapping_sub(0x5268));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1eec), c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1eec));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
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

/// `BEMissionInt *BEMissionInt_GetInstance(void)` (0x1532a0).
pub fn be_mission_int_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1eb8));
        let have = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !have {
            c.cycles += 11;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x14;
            c.g[A1][0] = sx(0x460000u32.wrapping_sub(0x57b8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, EA_MEM_ALLOC_NAMED, 0x0015_32c8)?;
            c.g[V1][0] = 0x460000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
            c.g[V1][0] = sx(0x460000u32.wrapping_sub(0x5208));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1eb8), c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1eb8));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
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

/// `void BEBAGTInt_DestroyInstance(void)` (0x15c6e8).
pub fn be_bagt_int_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ee0));
        frame.save_ra(c, 0);
        call_ptr(c, BUILTIN_DELETE, 0x0015_c6fc)?;
        frame.restore_ra(c, 0);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1ee0), 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BEAggressionInt_DestroyInstance(void)` (0x15c7e0).
pub fn be_aggression_int_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ea4));
        frame.save_ra(c, 0);
        call_ptr(c, BUILTIN_DELETE, 0x0015_c7f4)?;
        frame.restore_ra(c, 0);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1ea4), 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BEMissionInt_DestroyInstance(void)` (0x15bcd0).
pub fn be_mission_int_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1eb8));
        frame.save_ra(c, 0);
        call_ptr(c, BUILTIN_DELETE, 0x0015_bce4)?;
        frame.restore_ra(c, 0);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1eb8), 0);
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

// ---------------------------------------------------------------------------
// Menu hooks and thunks.

/// `void AvalancheMenu_OnActivate(AvalancheMenu *self)` (0x2515f0):
/// `MenuSysPage_OnActivate(self)`.
pub fn avalanche_menu_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002c_bf60, 0x0025_1600)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AvalancheMenu_OnDeactivate(AvalancheMenu *self)` (0x251610):
/// `MenuSysPage_OnDeactivate(self)`.
pub fn avalanche_menu_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002c_bfd0, 0x0025_1620)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Avalanche_DestroyAllThunk(void)` (0x2f7e70): `Avalanche_DestroyAll()`.
pub fn avalanche_destroy_all_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee { addr: 0x002d_9b40, f: ssx3_ee::s09::f_002d9b40 }.call(c, 0x002f_7e80)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider names.

/// `const char *BE_GetRiderCharName(int rider)` (0x14ee28): index the rider's
/// character record by 0x88 and return the name string inside it.
pub fn be_get_rider_char_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0014_ee38)?;
        c.g[A0][0] = 0x88;
        c.g[V1][0] = 0x530000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx(0x530000u32.wrapping_add(0x970));
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.close(c);
        Ok(())
    })
}

/// `const char *BE_GetRiderDisplayName(int rider)` (0x14ee58): the rider's
/// character's display name; for indices 0xa..0x1d it comes from one table
/// (0x43fa38), otherwise from the per-rider name table at 0x530990.
pub fn be_get_rider_display_name(c: &mut Cpu) {
    guest(c, |c| {
        const NAME_TABLE: u32 = 0x0053_0990;
        const INDEX_TABLE: u32 = 0x0043_fa38;
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0014_ee74)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        Callee { addr: 0x0014_a0b0, f: ssx3_ee::s01::f_0014a0b0 }.call(c, 0x0014_ee80)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xa));
        c.g[V0][0] = (c.g[V0][0] < 0x14u64) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0x440000;
        } else {
            c.cycles += 5;
            c.g[V1][0] = 0x88;
            c.g[V0][0] = 0x530000;
            c.mult(0, V1, S1, V1);
            c.g[V0][0] = sx(0x530000u32.wrapping_add(0x990));
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        // In-range character index: name = INDEX_TABLE + (index << 3).
        c.cycles += 2;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[V0][0] = sx(0x440000u32.wrapping_sub(0x5c8));
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let _ = (NAME_TABLE, INDEX_TABLE);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Back-end mission state: 0x440000 holds two parallel record tables (mission
// records at 0x43ee10, 0x24 bytes each; peak records at 0x43d950) and the
// per-session table at 0x440770 (0x2c bytes each).

const MISSION_RECORDS: u32 = 0x0043_ee10; // 0x440000 - 0x11f0
const PEAK_RECORD_END: u32 = 0x0043_ee20; // 0x440000 - 0x11e0
const PEAK_RECORDS: u32 = 0x0043_d950; // 0x440000 - 0x26b0
const SESSION_TABLE: u32 = 0x0044_0770; // 0x440000 + 0x770
const MISSION_STRIDE: u32 = 0x24;
const SESSION_STRIDE: u32 = 0x2c;
const RIDER_STRIDE: u32 = 0xf88;
const RIDER_FLAG_TABLE: u32 = 0x004a_6ca8; // 0x4a0000 + 0x6ca8
const CHAR_STATE: u32 = 0x0053_5538; // 0x530000 + 0x5538

const FIND_MISSION: Callee = Callee { addr: 0x0015_4240, f: ssx3_ee::s01::f_00154240 };

/// The generated loop back-edge event check (`if cycles >= next_event`).
#[inline(always)]
fn event_check(c: &mut Cpu, head: u32) {
    if c.cycles >= c.next_event {
        c.pc = head;
        deliver_event_here(c, c.pc);
    }
}

/// `int BEMissionInt_FindMission(BEMissionInt *self, int id)` (0x154240): the
/// index of the mission record whose id equals `id`, or -1 after 88 records.
pub fn be_mission_int_find_mission(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x440000;
    c.g[V1][0] = 0;
    let mut rec = MISSION_RECORDS;
    c.g[A0][0] = sx(rec);
    loop {
        c.cycles += 3;
        c.g[V0][0] = lw(c, rec);
        if c.g[V0][0] != c.g[A1][0] {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 88) as u64;
            let t = c.g[V0][0] != 0;
            rec = rec.wrapping_add(MISSION_STRIDE);
            c.g[A0][0] = sx(rec);
            if t {
                event_check(c, 0x0015_4250);
                continue;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                c.pc = c.g[RA][0] as u32;
                return;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0];
            c.pc = c.g[RA][0] as u32;
            return;
        }
    }
}

/// `int BEMissionInt_CountMissionsOnTrack(BEMissionInt *self, int track)`
/// (0x1542a0): how many of the 88 mission records have this track.
pub fn be_mission_int_count_missions_on_track(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x440000;
    c.g[A0][0] = 0;
    c.g[V0][0] = sx(0x440000u32.wrapping_sub(0x11f0));
    c.g[A2][0] = 0x57;
    let mut rec = MISSION_RECORDS.wrapping_add(0x10);
    c.g[A3][0] = sx(rec);
    loop {
        c.cycles += 7;
        c.g[V0][0] = lw(c, rec);
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        rec = rec.wrapping_add(MISSION_STRIDE);
        c.g[A3][0] = sx(rec);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        c.g[V0][0] = c.g[V0][0] ^ c.g[A1][0];
        let t = (c.g[A2][0] as i64) >= 0;
        if c.g[V0][0] == 0 {
            c.g[A0][0] = c.g[V1][0];
        }
        if t {
            event_check(c, 0x0015_42b8);
            continue;
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            c.pc = c.g[RA][0] as u32;
            return;
        }
    }
}

/// `int BEMissionInt_CountMissionsOnPeak(BEMissionInt *self, int peak)`
/// (0x154368): count mission records on `peak` across the mission/peak tables.
pub fn be_mission_int_count_missions_on_peak(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V1][0] = 0x440000;
    c.g[V0][0] = 0x440000;
    c.g[T4][0] = sx(0x440000u32.wrapping_sub(0x11e0));
    c.g[T3][0] = sx(0x440000u32.wrapping_sub(0x26b0));
    c.g[A3][0] = 0;
    c.g[V1][0] = 0;
    c.g[T2][0] = 0x24;
    c.mult(0, V0, V1, T2);
    loop {
        c.cycles += 4;
        c.g[A2][0] = 0;
        c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x54));
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
        loop {
            c.cycles += 3;
            c.g[V0][0] = lw(c, c.g[A0][0] as u32);
            if c.g[V0][0] != c.g[A1][0] {
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 5;
                c.g[V0][0] = lw(c, c.g[T0][0] as u32);
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.g[V0][0] ^ c.g[A2][0];
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[V1][0];
                }
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 22) as u64;
            let t = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x64));
            if t {
                event_check(c, 0x0015_4398);
                continue;
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[T1][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 88) as u64;
            let t = c.g[V0][0] != 0;
            c.mult(0, V0, V1, T2);
            if t {
                event_check(c, 0x0015_4388);
                break;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[A3][0];
                c.pc = c.g[RA][0] as u32;
                return;
            }
        }
    }
}

/// `int BEMissionInt_CountCollectedOnPeakFor(BEMissionInt *self, int track,
/// int peak, int mission)` (0x153688): sum the "collected" flags of the mission
/// matching `track`, `peak`, `mission`.
pub fn be_mission_int_count_collected_on_peak_for(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0xf88;
    c.g[V1][0] = 0x9b50;
    c.mult(0, T4, A2, V0);
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = 0x440000;
    c.g[V1][0] = 0x4a0000;
    c.g[T5][0] = sx(0x440000u32.wrapping_sub(0x26b0));
    c.g[V1][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
    c.g[T1][0] = 0;
    c.g[A0][0] = 0;
    c.g[T3][0] = 0xc;
    c.mult(0, A2, A0, T3);
    loop {
        c.cycles += 6;
        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        c.g[T0][0] = 0x15;
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T4][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A2][0] = sx((c.g[T5][0] as u32).wrapping_add(0x54));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        loop {
            c.cycles += 3;
            c.g[V0][0] = lw(c, c.g[A2][0] as u32);
            if c.g[V0][0] != c.g[A3][0] {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lb((c.g[A0][0] as u32).wrapping_add(4));
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
            }
            c.cycles += 3;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
            let t = (c.g[T0][0] as i64) >= 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x64));
            if t {
                event_check(c, 0x0015_36d0);
                continue;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[T2][0];
            let t = (c.g[A0][0] as i64) <= 0;
            c.mult(0, A2, A0, T3);
            if t {
                event_check(c, 0x0015_36b8);
                break;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[T1][0];
                c.pc = c.g[RA][0] as u32;
                return;
            }
        }
    }
}

/// `int BEMissionInt_GetMissionTrack(BEMissionInt *self, int id)` (0x153f30):
/// the track of the mission record with this id.
pub fn be_mission_int_get_mission_track(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        FIND_MISSION.call(c, 0x0015_3f40)?;
        c.g[A0][0] = MISSION_STRIDE as u64;
        c.g[V1][0] = 0x440000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx(0x440000u32.wrapping_sub(0x11f0));
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x10));
        frame.close(c);
        Ok(())
    })
}

/// Common tail of the per-flag mission getters: after `FindMission` the flag
/// word is `*((char * 0xf88) + 0x4a6ca8 + index*4 + 0x118)`, shifted right by
/// `shift` and masked to its low bit.
fn mission_flag(c: &mut Cpu, entry: u64, find_ret: u32, char_ret: u32, shift: u32) -> Flow {
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.cycles += entry as u64;
    FIND_MISSION.call(c, find_ret)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 0;
    BE_GET_RIDER_CHARACTER.call(c, char_ret)?;
    c.g[A0][0] = RIDER_STRIDE as u64;
    c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
    c.mult(0, V0, V0, A0);
    c.g[V1][0] = RIDER_FLAG_TABLE as u64;
    frame.restore_ra(c, 0);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = lw(c, (c.g[S0][0] as u32).wrapping_add(0x118));
    frame.restore(c, S0, 0x10);
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> shift);
    c.g[V0][0] = c.g[V0][0] & 0x1;
    frame.close(c);
    Ok(())
}

/// `int BEMissionInt_IsMissionComplete(BEMissionInt *self, int id)` (0x153d78).
pub fn be_mission_int_is_mission_complete(c: &mut Cpu) {
    guest(c, |c| mission_flag(c, 22, 0x0015_3d8c, 0x0015_3d98, 3))
}

/// `int BEMissionInt_GetMissionFlag1(BEMissionInt *self, int id)` (0x153dd0).
pub fn be_mission_int_get_mission_flag1(c: &mut Cpu) {
    guest(c, |c| mission_flag(c, 22, 0x0015_3de4, 0x0015_3df0, 1))
}

/// `int BEMissionInt_GetMissionFlag0(BEMissionInt *self, int id)` (0x153e28).
pub fn be_mission_int_get_mission_flag0(c: &mut Cpu) {
    guest(c, |c| mission_flag(c, 21, 0x0015_3e3c, 0x0015_3e48, 0))
}

/// `int BEMissionInt_GetMissionFlag2(BEMissionInt *self, int id)` (0x153e80).
pub fn be_mission_int_get_mission_flag2(c: &mut Cpu) {
    guest(c, |c| mission_flag(c, 22, 0x0015_3e94, 0x0015_3ea0, 2))
}

/// `int BEMissionInt_GetMissionFlag4(BEMissionInt *self, int id)` (0x153ed8).
pub fn be_mission_int_get_mission_flag4(c: &mut Cpu) {
    guest(c, |c| mission_flag(c, 22, 0x0015_3eec, 0x0015_3ef8, 4))
}

/// Common tail of the per-flag mission setters: after `FindMission`, replace
/// bit `shift` of the flag word with the low bit of `a2`.
fn set_mission_flag(c: &mut Cpu, entry: u64, find_ret: u32, char_ret: u32, shift: u32, mask: u64) -> Flow {
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0);
    c.cycles += entry as u64;
    c.g[S1][0] = c.g[A2][0];
    FIND_MISSION.call(c, find_ret)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 0;
    BE_GET_RIDER_CHARACTER.call(c, char_ret)?;
    c.g[A0][0] = RIDER_STRIDE as u64;
    c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
    c.mult(0, V0, V0, A0);
    c.g[V1][0] = RIDER_FLAG_TABLE as u64;
    c.g[A0][0] = mask;
    c.g[S1][0] = c.g[S1][0] & 1;
    frame.restore_ra(c, 0);
    c.g[S1][0] = sx((c.g[S1][0] as u32) << shift);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = lw(c, (c.g[S0][0] as u32).wrapping_add(0x118));
    c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S1][0];
    c.sw((c.g[S0][0] as u32).wrapping_add(0x118), c.g[V0][0]);
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.close(c);
    Ok(())
}

/// `void BEMissionInt_SetMissionFlag0(BEMissionInt *self, int id, int v)`
/// (0x154010).
pub fn be_mission_int_set_mission_flag0(c: &mut Cpu) {
    guest(c, |c| set_mission_flag(c, 27, 0x0015_4028, 0x0015_4034, 0, 0xffff_ffff_ffff_fffe))
}

/// `void BEMissionInt_SetMissionFlag1(BEMissionInt *self, int id, int v)`
/// (0x154080).
pub fn be_mission_int_set_mission_flag1(c: &mut Cpu) {
    guest(c, |c| set_mission_flag(c, 28, 0x0015_4098, 0x0015_40a4, 1, 0xffff_ffff_ffff_fffd))
}

/// `void BEMissionInt_SetMissionFlag2(BEMissionInt *self, int id, int v)`
/// (0x1540f0).
pub fn be_mission_int_set_mission_flag2(c: &mut Cpu) {
    guest(c, |c| set_mission_flag(c, 28, 0x0015_4108, 0x0015_4114, 2, 0xffff_ffff_ffff_fffb))
}

/// `void BEMissionInt_SetMissionComplete(BEMissionInt *self, int id, int v)`
/// (0x154160).
pub fn be_mission_int_set_mission_complete(c: &mut Cpu) {
    guest(c, |c| set_mission_flag(c, 28, 0x0015_4178, 0x0015_4184, 3, 0xffff_ffff_ffff_fff7))
}

/// `void BEMissionInt_SetMissionFlag4(BEMissionInt *self, int id, int v)`
/// (0x1541d0).
pub fn be_mission_int_set_mission_flag4(c: &mut Cpu) {
    guest(c, |c| set_mission_flag(c, 28, 0x0015_41e8, 0x0015_41f4, 4, 0xffff_ffff_ffff_ffef))
}

/// `void *BEMissionInt_GetNthMissionOnTrack(BEMissionInt *self, int n,
/// int track)` (0x1545b0): the `n`-th record on `track`, or 0.
pub fn be_mission_int_get_nth_mission_on_track(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x440000;
    c.g[A0][0] = 0;
    c.g[V1][0] = sx(0x440000u32.wrapping_sub(0x11f0));
    c.g[A3][0] = sx(MISSION_RECORDS.wrapping_add(0xc60));
    c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x10));
    loop {
        c.cycles += 2;
        if c.g[V0][0] != c.g[A2][0] {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(MISSION_STRIDE));
        } else {
            c.cycles += 2;
            let t = c.g[A0][0] != c.g[A1][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            if t {
                c.cycles += 1;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(MISSION_STRIDE));
            } else {
                c.cycles += 2;
                c.g[V0][0] = lw(c, c.g[V1][0] as u32);
                c.pc = c.g[RA][0] as u32;
                return;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x10));
            event_check(c, 0x0015_45c8);
            continue;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.pc = c.g[RA][0] as u32;
            return;
        }
    }
}

/// `int BEMissionInt_GetSessionType(BEMissionInt *self, int id)` (0x1545f8):
/// the session type at +0x18 of the matching session record, or 0.
pub fn be_mission_int_get_session_type(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x440000;
    c.g[V1][0] = sx(SESSION_TABLE);
    c.g[A0][0] = sx(SESSION_TABLE.wrapping_add(0x3c8));
    c.g[V0][0] = lw(c, SESSION_TABLE);
    loop {
        c.cycles += 2;
        if c.g[V0][0] != c.g[A1][0] {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(SESSION_STRIDE));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = lw(c, c.g[V1][0] as u32);
                event_check(c, 0x0015_4608);
                continue;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.pc = c.g[RA][0] as u32;
                return;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x18));
            c.pc = c.g[RA][0] as u32;
            return;
        }
    }
}

/// `const char *BEMissionInt_GetSessionName(BEMissionInt *self, int id)`
/// (0x154760): the name field (+4) of the matching session record, or 0.
pub fn be_mission_int_get_session_name(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = sx(SESSION_TABLE);
    c.g[V1][0] = c.g[V0][0];
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
    c.g[A2][0] = sx(SESSION_TABLE.wrapping_add(0x3c8));
    loop {
        c.cycles += 3;
        c.g[V0][0] = lw(c, c.g[V1][0] as u32);
        let t = c.g[V0][0] != c.g[A1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(SESSION_STRIDE));
        if t {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            let t = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(SESSION_STRIDE));
            if t {
                event_check(c, 0x0015_4778);
                continue;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.pc = c.g[RA][0] as u32;
                return;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            c.pc = c.g[RA][0] as u32;
            return;
        }
    }
}

/// `void *BEMissionInt_GetSessionInfo(BEMissionInt *self, int id, void *out)`
/// (0x154678): copy the matching 0x2c-byte session record to `out`, or the
/// first record when there is no match.
pub fn be_mission_int_get_session_info(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = 0x440000;
    c.g[V1][0] = sx(SESSION_TABLE);
    c.g[A1][0] = sx(SESSION_TABLE.wrapping_add(0x3c8));
    c.g[V0][0] = lw(c, SESSION_TABLE);
    loop {
        c.cycles += 2;
        if c.g[V0][0] != c.g[A2][0] {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(SESSION_STRIDE));
        } else {
            c.cycles += 23;
            copy_session(c, c.g[V1][0] as u32, c.g[A0][0] as u32);
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = lw(c, c.g[V1][0] as u32);
            event_check(c, 0x0015_4688);
            continue;
        } else {
            c.cycles += 23;
            let first = SESSION_TABLE;
            // This copy uses r2/r3/r5, so it leaves v1 holding the last
            // doubleword of the record (r3 is reloaded for it).
            let q4 = copy_session(c, first, c.g[A0][0] as u32);
            c.g[V1][0] = q4;
            c.g[A2][0] = sx(first);
            break;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    c.pc = c.g[RA][0] as u32;
}

/// Copy the 0x2c-byte session record at `src` to `dst` (two unaligned 8-byte
/// chunks, an 8-byte chunk, a 4-byte word, and the trailing word).
fn copy_session(c: &mut Cpu, src: u32, dst: u32) -> u64 {
    // The original reads four doublewords before writing any of them.
    let q0 = Addr(src).read_u64_unaligned(c);
    let q1 = Addr(src + 8).read_u64_unaligned(c);
    let q2 = Addr(src + 0x10).read_u64_unaligned(c);
    let q3 = Addr(src + 0x18).read_u64_unaligned(c);
    Addr(dst).write_u64_unaligned(c, q0);
    Addr(dst + 8).write_u64_unaligned(c, q1);
    Addr(dst + 0x10).write_u64_unaligned(c, q2);
    Addr(dst + 0x18).write_u64_unaligned(c, q3);
    let q4 = Addr(src + 0x20).read_u64_unaligned(c);
    let tail = lw(c, src.wrapping_add(0x28));
    Addr(dst + 0x20).write_u64_unaligned(c, q4);
    c.g[A1][0] = tail;
    c.sw(dst.wrapping_add(0x28), tail);
    q4
}

// ---------------------------------------------------------------------------
// Mission text getters: a session-string accessor through a virtual method.

/// Common body of the `BEMissionInt_GetMissionTextN` getters (0x153c88/0x153cd8/
/// 0x153d28): look the mission up, then call the per-rider table entry's
/// `+off` string getter.
fn get_mission_text(c: &mut Cpu, off: u32, find_ret: u32, call_ret: u32) -> Flow {
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.cycles += 20;
    FIND_MISSION.call(c, find_ret)?;
    c.g[A0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[V1][0] = MISSION_STRIDE as u64;
    c.mult(0, V0, V0, V1);
    c.g[A2][0] = 0x440000;
    c.g[A3][0] = lw(c, (c.g[A0][0] as u32).wrapping_add(0x8c));
    c.g[A2][0] = sx(0x440000u32.wrapping_sub(0x11f0));
    c.g[V1][0] = lw(c, (c.g[A3][0] as u32).wrapping_add(4));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
    c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x24));
    c.g[A1][0] = lw(c, (c.g[A2][0] as u32).wrapping_add(off));
    let t = c.g[V0][0] as u32;
    call_ptr(c, t, call_ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `const char *BEMissionInt_GetMissionText1(BEMissionInt *self, int id)`
/// (0x153c88).
pub fn be_mission_int_get_mission_text1(c: &mut Cpu) {
    guest(c, |c| get_mission_text(c, 4, 0x0015_3c98, 0x0015_3ccc))
}

/// `const char *BEMissionInt_GetMissionText2(BEMissionInt *self, int id)`
/// (0x153cd8).
pub fn be_mission_int_get_mission_text2(c: &mut Cpu) {
    guest(c, |c| get_mission_text(c, 8, 0x0015_3ce8, 0x0015_3d1c))
}

/// `const char *BEMissionInt_GetMissionText3(BEMissionInt *self, int id)`
/// (0x153d28).
pub fn be_mission_int_get_mission_text3(c: &mut Cpu) {
    guest(c, |c| get_mission_text(c, 0xc, 0x0015_3d38, 0x0015_3d6c))
}

// ---------------------------------------------------------------------------
// BECharInt attribute bytes. The character state is `char * 0x46` plus the
// character's record (0x530000 + 0x5538 + 8*char - char); each attribute is one
// byte at +index.

/// The shared prologue/epilogue of `BECharInt_SetAttrib1..6` (the accumulator
/// ends in v0); `index` is the attribute byte offset (1..6).
fn set_attrib(c: &mut Cpu, entry: u64, index: u32, r1: u32, r2: u32) -> Flow {
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
// More back-end singletons, character profiles and economy.

const BE_STATIC_INIT_DESTROY: Callee = Callee { addr: 0x0015_b028, f: ssx3_ee::s01::f_0015b028 };
const BOLT_GET_RIDER_PROFILE: Callee = Callee { addr: 0x0014_ac50, f: ssx3_ee::s01::f_0014ac50 };
const BOLT_GET_RIDER_CHAR: Callee = Callee { addr: 0x0014_ac30, f: ssx3_ee::s01::f_0014ac30 };
const GET_CHAR_PROFILE_DATA: Callee = Callee { addr: 0x0014_ad28, f: ssx3_ee::s01::f_0014ad28 };
const BOLT_GET_BOLT_STATES: Callee = Callee { addr: 0x0014_ae28, f: ssx3_ee::s01::f_0014ae28 };
const EA_MEM_COPY: u32 = 0x003e_6574;
const NEW_RACE_GET_INSTANCE: Callee = Callee { addr: 0x0014_48d8, f: ssx3_ee::s01::f_001448d8 };
const F_00145750: Callee = Callee { addr: 0x0014_5750, f: ssx3_ee::s01::f_00145750 };
const F_00151600: Callee = Callee { addr: 0x0015_1600, f: ssx3_ee::s01::f_00151600 };
const F_001519E0: Callee = Callee { addr: 0x0015_19e0, f: ssx3_ee::s01::f_001519e0 };
const ECON_ADD_MONEY: Callee = Callee { addr: 0x0015_0a90, f: ssx3_ee::s01::f_00150a90 };

/// `void BE_GlobalCtors(void)` (0x15c8f0): `BE_StaticInitDestroy(1, 0xffff)`.
pub fn be_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        BE_STATIC_INIT_DESTROY.call(c, 0x0015_c904)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_GlobalDtors(void)` (0x15c910): `BE_StaticInitDestroy(0, 0xffff)`.
pub fn be_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        BE_STATIC_INIT_DESTROY.call(c, 0x0015_c924)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

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
        Callee { addr: 0x0014_ac70, f: ssx3_ee::s01::f_0014ac70 }.call(c, 0x0014_ae80)?;
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
fn econ_rider_field(c: &mut Cpu, r_prof: u32, r_char: u32, field: u32) -> Flow {
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

/// `int BEAggressionInt_GetRivalState(BEAggressionInt *self, int rider,
/// int rival)` (0x155ab0): compare the rival's current race position with the
/// rider's and return 2/… from the aggression table, or 2 when they are equal.
pub fn be_aggression_int_get_rival_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_5ad4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_5ae0)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_5aec)?;
        c.g[S1][0] = c.g[V0][0];
        NEW_RACE_GET_INSTANCE.call(c, 0x0015_5af4)?;
        c.g[A0][0] = c.g[V0][0];
        F_00145750.call(c, 0x0015_5afc)?;
        let t = c.g[V0][0] == c.g[S0][0];
        c.g[A0][0] = RIDER_STRIDE as u64;
        if t {
            c.cycles += 1;
            c.g[V0][0] = 0x2;
        } else {
            c.cycles += 12;
            c.g[A1][0] = 0x9b50;
            c.mult(0, A0, S1, A0);
            c.mult(0, A1, S2, A1);
            c.g[V0][0] = sx((c.g[S0][0] as u32) << 1);
            c.g[V1][0] = 0x4a0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[V1][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0xbc1));
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

/// `void BE_ResetProfileCharacters(void *self, int arg)` (0x1567b8): reset the
/// ten characters (`f_00151600(self + i*0xf88, i, arg)`).
pub fn be_reset_profile_characters(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            F_00151600.call(c, 0x0015_67ec)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 10) as u64;
            let t = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(RIDER_STRIDE));
            if t {
                event_check(c, 0x0015_67d8);
                continue;
            } else {
                c.cycles += 6;
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `void BE_ResetProfileRivalries(void *self)` (0x156810): reset the ten
/// rivalries (`f_001519e0(self + i*0xf88, i)`).
pub fn be_reset_profile_rivalries(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            F_001519E0.call(c, 0x0015_6834)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 10) as u64;
            let t = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(RIDER_STRIDE));
            if t {
                event_check(c, 0x0015_6828);
                continue;
            } else {
                c.cycles += 5;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `void BE_BackupCharacter(int self, int rider)` (0x156950): copy the rider's
/// 0xf88-byte character record to 0x4c4090 and remember the rider index.
pub fn be_backup_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[V0][0] = RIDER_STRIDE as u64;
        let frame = Frame::open(c, 0x10);
        c.mult(0, V1, A1, V0);
        c.sb((c.g[GP][0] as u32).wrapping_sub(0x1e94), c.g[A1][0]);
        frame.save_ra(c, 0);
        c.g[A2][0] = RIDER_STRIDE as u64;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = 0x4c0000;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = sx(0x4c0000u32.wrapping_add(0x4090));
        call_ptr(c, EA_MEM_COPY, 0x0015_697c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BE_RestoreCharacter(int self)` (0x156988): copy 0x4c4090 back to the
/// remembered rider's character record.
pub fn be_restore_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = c.lb((c.g[GP][0] as u32).wrapping_sub(0x1e94));
        c.g[V1][0] = RIDER_STRIDE as u64;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x4c0000;
        c.mult(0, A3, V0, V1);
        frame.save_ra(c, 0);
        c.g[A2][0] = RIDER_STRIDE as u64;
        c.g[A1][0] = sx(0x4c0000u32.wrapping_add(0x4090));
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, EA_MEM_COPY, 0x0015_69b0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Collected-mission bit fields and the save round-trip.

const BELIBRARY_DESERIALIZE_SAVE: Callee = Callee { addr: 0x0014_a5e0, f: ssx3_ee::s01::f_0014a5e0 };

/// `bool BELibrary_ReadSaveData(BELibrary *self, const void *data)` (0x14e0c0):
/// `BELibrary_DeserializeSave(data)`.
pub fn be_library_read_save_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        BELIBRARY_DESERIALIZE_SAVE.call(c, 0x0014_e0d0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `char BEMissionInt_GetCollectedCount(BEMissionInt *self, int rider, int idx)`
/// (0x153498): the per-rider collected count byte.
pub fn be_mission_int_get_collected_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_34bc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S2][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_34c8)?;
        c.g[A0][0] = 0x530000;
        c.g[A2][0] = 0xc;
        c.g[A1][0] = lw(c, (c.g[A0][0] as u32).wrapping_add(0x5f0));
        c.g[V1][0] = RIDER_STRIDE as u64;
        c.mult(0, V0, V0, V1);
        c.g[A0][0] = 0x9b50;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.mult(0, S2, S2, A0);
        c.mult(0, S0, S0, A2);
        c.g[V1][0] = 0x4a0000;
        c.g[V1][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(4));
        frame.restore(c, S2, 0x10);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `int BEMissionInt_CountCollectedOnPeak(BEMissionInt *self, int rider, int
/// peak)` (0x1535c0): sum the collected bits of missions on this peak.
pub fn be_mission_int_count_collected_on_peak(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_35e4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_35f0)?;
        c.g[V1][0] = RIDER_STRIDE as u64;
        c.g[T3][0] = 0xc;
        c.mult(0, A1, V0, V1);
        c.g[A3][0] = 0;
        c.g[V0][0] = 0x9b50;
        c.g[V1][0] = 0x440000;
        c.mult(0, S1, S1, V0);
        c.g[V1][0] = sx(0x440000u32.wrapping_sub(0x26b0));
        c.g[V0][0] = 0x4a0000;
        c.g[A0][0] = 0;
        c.g[T2][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
        c.mult(0, A1, A0, T3);
        loop {
            c.cycles += 6;
            c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            c.g[A2][0] = 0x15;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x54));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
            loop {
                c.cycles += 3;
                c.g[V0][0] = lw(c, c.g[A1][0] as u32);
                if c.g[V0][0] != c.g[S2][0] {
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb((c.g[A0][0] as u32).wrapping_add(4));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                }
                c.cycles += 3;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                let t = (c.g[A2][0] as i64) >= 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x64));
                if t {
                    event_check(c, 0x0015_3638);
                    continue;
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[T0][0];
                if (c.g[A0][0] as i64) <= 0 {
                    c.mult(0, A1, A0, T3);
                    event_check(c, 0x0015_3620);
                    break;
                } else {
                    c.cycles += 7;
                    frame.restore(c, S0, 0x30);
                    c.g[V0][0] = c.g[A3][0];
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
    })
}

/// `int BEMissionInt_GetCollectedMask(BEMissionInt *self, int rider, int peak)`
/// (0x153858): pointer into the per-rider collected bit table for this peak.
pub fn be_mission_int_get_collected_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_387c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_3888)?;
        c.g[V1][0] = 0x530000;
        c.g[A0][0] = RIDER_STRIDE as u64;
        c.g[A1][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x5f0));
        c.mult(0, V0, V0, A0);
        c.g[A2][0] = 0xc;
        c.g[V1][0] = 0x9b50;
        c.mult(0, S0, S0, V1);
        c.mult(0, A1, A1, A2);
        c.g[V1][0] = 0x4a0000;
        frame.restore(c, S1, 0x20);
        c.mult(0, S2, S2, A2);
        c.g[V1][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        frame.restore(c, S2, 0x10);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `bool BEMissionInt_IsCollected(BEMissionInt *self, int rider, int peak,
/// int idx)` (0x1538e8): whether collected bit `idx` is set for this rider/peak.
pub fn be_mission_int_is_collected(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S2][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_3914)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_3920)?;
        c.g[A0][0] = 0x530000;
        c.g[V1][0] = RIDER_STRIDE as u64;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = lw(c, (c.g[A0][0] as u32).wrapping_add(0x5f0));
        c.g[A2][0] = 0x9b50;
        c.g[V1][0] = 0xc;
        c.mult(0, S0, S0, V1);
        c.mult(0, A1, A1, A2);
        c.g[A0][0] = 0xffff_ffff_ffff_ffff;
        frame.restore(c, S2, 0x20);
        c.mult(0, S1, S1, V1);
        c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[S3][0] as i64)) as u64;
        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1f));
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if c.g[A0][0] != 0 {
            c.g[V1][0] = c.g[S3][0];
        }
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = 0x4a0000;
        c.g[V0][0] = sx(0x4a0000u32.wrapping_add(0x6cac));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A0][0] = 0x1;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[S3][0] as u32) & 31));
        c.g[V0][0] = lw(c, (c.g[S1][0] as u32).wrapping_add(4));
        frame.restore(c, S0, 0x40);
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S3, 0x10);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Completed-mission counts and save writing.

const BELIBRARY_SERIALIZE_SAVE: Callee = Callee { addr: 0x0014_a188, f: ssx3_ee::s01::f_0014a188 };
const BELIBRARY_GET_SAVE_SIZE: Callee = Callee { addr: 0x0014_e048, f: ssx3_ee::s01::f_0014e048 };
const COUNT_COMPLETED_ON_PEAK: Callee = Callee { addr: 0x0015_43e0, f: ssx3_ee::s01::f_001543e0 };

/// `int BEMissionInt_CountCompletedOnTrack(BEMissionInt *self, int rider,
/// int track)` (0x1542e0): number of completed missions on this track.
pub fn be_mission_int_count_completed_on_track(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x440000;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(0x440000u32.wrapping_sub(0x11f0));
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = sx(MISSION_RECORDS.wrapping_add(0xc60));
        c.g[V0][0] = lw(c, (c.g[S0][0] as u32).wrapping_add(0x10));
        loop {
            c.cycles += 2;
            if c.g[V0][0] != c.g[S2][0] {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(MISSION_STRIDE));
            } else {
                c.cycles += 6;
                c.g[A1][0] = lw(c, c.g[S0][0] as u32);
                c.g[A0][0] = c.g[S3][0];
                Callee { addr: 0x0015_3d78, f: ssx3_ee::s01::f_00153d78 }.call(c, 0x0015_432c)?;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = c.g[V1][0];
                }
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(MISSION_STRIDE));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = lw(c, (c.g[S0][0] as u32).wrapping_add(0x10));
                event_check(c, 0x0015_4318);
                continue;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.g[S1][0];
                frame.restore(c, S0, 0x50);
                frame.restore(c, S1, 0x40);
                frame.restore(c, S2, 0x30);
                frame.restore(c, S3, 0x20);
                frame.restore(c, S4, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `int BEMissionInt_CountCompletedOnPeakFor(BEMissionInt *self, int rider,
/// int peak, int track)` (0x1544d0): count completed missions of `rider` on
/// `peak` and `track`.
pub fn be_mission_int_count_completed_on_peak_for(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x84));
        let t = c.g[V0][0] == 0;
        c.g[T1][0] = c.g[A2][0];
        if !t {
            c.cycles += 4;
            c.g[A1][0] = c.g[A3][0];
            COUNT_COMPLETED_ON_PEAK.call(c, 0x0015_44f0)?;
            c.cycles += 2;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 18;
        c.g[V1][0] = 0x9b50;
        c.g[A0][0] = RIDER_STRIDE as u64;
        c.mult(0, V1, A1, V1);
        c.mult(0, A0, T1, A0);
        c.g[A1][0] = 0x4a0000;
        c.g[A2][0] = 0x440000;
        c.g[A1][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
        c.g[V0][0] = 0x440000;
        c.g[V0][0] = sx(0x440000u32.wrapping_sub(0x11f0));
        c.g[T2][0] = sx(0x440000u32.wrapping_sub(0x265c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x118));
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[T1][0] = 0x64;
        c.g[A1][0] = 0x57;
        loop {
            c.cycles += 6;
            c.g[V0][0] = lw(c, c.g[T0][0] as u32);
            c.mult(0, V1, V0, T1);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
            c.g[V1][0] = lw(c, c.g[V0][0] as u32);
            if c.g[V1][0] != c.g[A3][0] {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            } else {
                c.cycles += 5;
                c.g[V0][0] = lw(c, c.g[A0][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
                c.g[V0][0] = c.g[V0][0] & 1;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            }
            c.cycles += 3;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            let t = (c.g[A1][0] as i64) >= 0;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(MISSION_STRIDE));
            if t {
                event_check(c, 0x0015_4540);
                continue;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[A2][0];
                c.cycles += 2;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `void BELibrary_WriteSaveData(BELibrary *self, void *out)` (0x14e050):
/// serialise the state, clear the header, and append a trailer.
pub fn be_library_write_save_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[S0][0];
        BELIBRARY_SERIALIZE_SAVE.call(c, 0x0014_e070)?;
        c.g[A0][0] = c.g[S1][0];
        BELIBRARY_GET_SAVE_SIZE.call(c, 0x0014_e078)?;
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        c.g[A2][0] = 0xfbea;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_62d0, 0x0014_e088)?;
        c.g[A0][0] = c.g[S1][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        BELIBRARY_GET_SAVE_SIZE.call(c, 0x0014_e094)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        frame.restore(c, S1, 0x20);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore_ra(c, 0x10);
        c.lwl(V0, (c.g[SP][0] as u32).wrapping_add(3));
        c.lwr(V0, c.g[SP][0] as u32);
        c.swl(V0, (c.g[S0][0] as u32).wrapping_add(3));
        c.swr(V0, c.g[S0][0] as u32);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Character resets and bulk attribute writes.

const SET_ATTRIB0: Callee = Callee { addr: 0x0014_83a0, f: ssx3_ee::s01::f_001483a0 };
const SET_ATTRIB1: Callee = Callee { addr: 0x0014_8470, f: ssx3_ee::s01::f_00148470 };
const SET_ATTRIB2: Callee = Callee { addr: 0x0014_8540, f: ssx3_ee::s01::f_00148540 };
const SET_ATTRIB3: Callee = Callee { addr: 0x0014_8610, f: ssx3_ee::s01::f_00148610 };
const SET_ATTRIB4: Callee = Callee { addr: 0x0014_86e0, f: ssx3_ee::s01::f_001486e0 };
const SET_ATTRIB5: Callee = Callee { addr: 0x0014_87b0, f: ssx3_ee::s01::f_001487b0 };
const SET_ATTRIB6: Callee = Callee { addr: 0x0014_8880, f: ssx3_ee::s01::f_00148880 };

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

// ---------------------------------------------------------------------------
// BAGT singleton.

/// `BEBAGTInt *BEBAGTInt_GetInstance(void)` (0x14f900): lazily allocate the
/// singleton, install its vtable and run the vtable's constructor slot (+0x10).
pub fn be_bagt_int_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ee0));
        let have = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !have {
            c.cycles += 16;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx(0x460000u32.wrapping_sub(0x5928));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, EA_MEM_ALLOC_NAMED, 0x0014_f928)?;
            c.g[V1][0] = 0x460000;
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1ee0), c.g[V0][0]);
            c.g[V1][0] = sx(0x460000u32.wrapping_sub(0x5328));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            let target = c.g[A1][0] as u32;
            call_ptr(c, target, 0x0014_f948)?;
            c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ee0));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
        }
        c.cycles += 4;
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1ee0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// @@FUNCS@@

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    /// Common setup for the singleton accessors: scrambled machine, gp set,
    /// the gp-relative singleton slot mostly empty (so the allocation path is
    /// taken) or pre-filled.
    fn instance_setup(c: &mut Cpu, seed: u32, slot: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(slot), if seed % 3 == 0 { 0 } else { DATA + 0x100 });
        r
    }

    #[test]
    fn be_bolt_int_get_instance_matches() {
        check(ssx3_ee::s01::f_0014abe0, be_bolt_int_get_instance, 90, &|c, seed| {
            instance_setup(c, seed, 0x1f28, 0xb01);
        });
    }

    #[test]
    fn be_network_int_get_instance_matches() {
        check(ssx3_ee::s01::f_0014e0e0, be_network_int_get_instance, 90, &|c, seed| {
            instance_setup(c, seed, 0x1eec, 0xbe7);
        });
    }

    #[test]
    fn be_econ_int_get_instance_matches() {
        check(ssx3_ee::s01::f_001508d8, be_econ_int_get_instance, 90, &|c, seed| {
            instance_setup(c, seed, 0x1edc, 0xec0);
        });
    }

    #[test]
    fn be_mission_int_get_instance_matches() {
        check(ssx3_ee::s01::f_001532a0, be_mission_int_get_instance, 90, &|c, seed| {
            instance_setup(c, seed, 0x1eb8, 0x315);
        });
    }

    #[test]
    fn be_destroy_instances_match() {
        for (gen, hand, slot, salt) in [
            (ssx3_ee::s01::f_0015c678 as fn(&mut Cpu), be_char_int_destroy_instance as fn(&mut Cpu), 0x1f2c, 0xc67),
            (ssx3_ee::s01::f_0015c6e8, be_bagt_int_destroy_instance, 0x1ee0, 0xc6e),
            (ssx3_ee::s01::f_0015c7e0, be_aggression_int_destroy_instance, 0x1ea4, 0xc7e),
            (ssx3_ee::s01::f_0015bcd0, be_mission_int_destroy_instance, 0x1eb8, 0xbcd),
        ] {
            check(gen, hand, 80, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                let p = if seed % 2 == 0 { DATA + (seed as u32 % 4) * 0x100 } else { 0 };
                c.bus.write32(GAME_GP.wrapping_sub(slot), p);
                let _ = &mut r;
            });
        }
    }

    #[test]
    fn beb_bolt_int_getters_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_0014ac30 as fn(&mut Cpu), beb_bolt_int_get_rider_character as fn(&mut Cpu), 0xac30),
            (ssx3_ee::s01::f_0014ac50, beb_bolt_int_get_rider_profile, 0xac50),
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
        check(ssx3_ee::s01::f_0014ac70, beb_bolt_int_get_rider_profile_and_char, 60, &|c, seed| {
            let mut r = machine(c, seed, 0xac70, 0);
            c.g[A1][0] = (seed % 12) as u64;
            // Sometimes `out` aliases the input pointer.
            c.g[A2][0] = (if seed % 4 == 0 { DATA + 0x300 } else { DATA + 0x400 }) as u64;
            let _ = &mut r;
        });
    }

    #[test]
    fn avalanche_menu_hooks_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s06::f_002515f0 as fn(&mut Cpu), avalanche_menu_on_activate as fn(&mut Cpu), 0x515),
            (ssx3_ee::s06::f_00251610, avalanche_menu_on_deactivate, 0x610),
        ] {
            check(gen, hand, 60, &|c, seed| {
                let mut r = machine(c, seed, salt, 1);
                c.g[A0][0] = DATA as u64;
                let _ = &mut r;
            });
        }
    }

    #[test]
    fn avalanche_destroy_all_thunk_matches() {
        check(ssx3_ee::s09::f_002f7e70, avalanche_destroy_all_thunk, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x7e70, 0);
            // Kill the singleton so the callee takes its short path.
            c.bus.write32(GAME_GP.wrapping_add(0x9c0), 0);
            let _ = &mut r;
        });
    }

    #[test]
    fn be_get_rider_char_name_matches() {
        check(ssx3_ee::s01::f_0014ee28, be_get_rider_char_name, 60, &|c, seed| {
            let mut r = machine(c, seed, 0xee28, 0);
            c.g[A1][0] = (seed % 12) as u64;
            let _ = &mut r;
        });
    }

    #[test]
    fn be_get_rider_display_name_matches() {
        check(ssx3_ee::s01::f_0014ee58, be_get_rider_display_name, 80, &|c, seed| {
            let mut r = machine(c, seed, 0xee58, 0);
            c.g[A1][0] = (seed % 12) as u64;
            let _ = &mut r;
        });
    }

    use crate::difftest::fill;

    /// Fill the mission/peak record tables with a mix of ids and tracks.
    fn fill_missions(c: &mut Cpu, r: &mut Rng, tracks: &[u32]) {
        for i in 0..88u32 {
            c.bus.write32(MISSION_RECORDS + i * 0x24, 0x1000 + i);
            c.bus.write32(MISSION_RECORDS + i * 0x24 + 0x10, tracks[(i as usize) % tracks.len()]);
        }
        fill(c, r, 0x43d000, 0xc00);
    }

    #[test]
    fn be_mission_int_find_mission_matches() {
        check(ssx3_ee::s01::f_00154240, be_mission_int_find_mission, 60, &|c, seed| {
            let mut r = machine(c, seed, 0xf00d, 0);
            fill_missions(c, &mut r, &[1, 2, 3]);
            let idx = seed as u32 % 89;
            c.g[A1][0] = if idx < 88 { (0x1000 + idx) as u64 } else { 0x9999 };
        });
    }

    #[test]
    fn be_mission_int_count_missions_on_track_matches() {
        check(ssx3_ee::s01::f_001542a0, be_mission_int_count_missions_on_track, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x42a0, 0);
            fill_missions(c, &mut r, &[0, 1, 2]);
            c.g[A1][0] = (seed % 3) as u64;
        });
    }

    #[test]
    fn be_mission_int_count_missions_on_peak_matches() {
        check(ssx3_ee::s01::f_00154368, be_mission_int_count_missions_on_peak, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x4368, 0);
            fill_missions(c, &mut r, &[0, 1, 2]);
            // Make the first peak-table entry equal a1 on even seeds (match
            // branch), otherwise use a value that never appears.
            c.g[A1][0] = if seed % 2 == 0 {
                c.bus.read32(0x43d9a4) as u64
            } else {
                0xdead_beef
            };
        });
    }

    #[test]
    fn be_mission_int_count_collected_on_peak_for_matches() {
        check(ssx3_ee::s01::f_00153688, be_mission_int_count_collected_on_peak_for, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x3688, 0);
            fill_missions(c, &mut r, &[0, 1, 2]);
            c.g[A1][0] = (seed % 3) as u64; // track
            c.g[A2][0] = if seed % 2 == 0 {
                c.bus.read32(0x43d9a4) as u64
            } else {
                0xdead_beef
            };
            c.g[A3][0] = (seed % 5) as u64; // mission
        });
    }

    /// Shared setup for the mission-flag getters/setters: a found mission id
    /// (or not) in `a1`, plus the flag word region.
    fn flag_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        fill_missions(c, &mut r, &[0, 1, 2]);
        let idx = seed as u32 % 90;
        c.g[A1][0] = if idx < 88 { (0x1000 + idx) as u64 } else { 0x9999 };
        c.g[A2][0] = (r.next_u32() | 1) as u64;
        fill(c, &mut r, RIDER_FLAG_TABLE, 0x200);
        r
    }

    #[test]
    fn be_mission_int_flag_getters_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_00153d78 as fn(&mut Cpu), be_mission_int_is_mission_complete as fn(&mut Cpu), 0x3d78),
            (ssx3_ee::s01::f_00153dd0, be_mission_int_get_mission_flag1, 0x3dd0),
            (ssx3_ee::s01::f_00153e28, be_mission_int_get_mission_flag0, 0x3e28),
            (ssx3_ee::s01::f_00153e80, be_mission_int_get_mission_flag2, 0x3e80),
            (ssx3_ee::s01::f_00153ed8, be_mission_int_get_mission_flag4, 0x3ed8),
        ] {
            check(gen, hand, 40, &|c, seed| {
                flag_setup(c, seed, salt);
            });
        }
    }

    #[test]
    fn be_mission_int_flag_setters_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_00154010 as fn(&mut Cpu), be_mission_int_set_mission_flag0 as fn(&mut Cpu), 0x4010),
            (ssx3_ee::s01::f_00154080, be_mission_int_set_mission_flag1, 0x4080),
            (ssx3_ee::s01::f_001540f0, be_mission_int_set_mission_flag2, 0x40f0),
            (ssx3_ee::s01::f_00154160, be_mission_int_set_mission_complete, 0x4160),
            (ssx3_ee::s01::f_001541d0, be_mission_int_set_mission_flag4, 0x41d0),
        ] {
            check(gen, hand, 40, &|c, seed| {
                flag_setup(c, seed, salt);
            });
        }
    }

    #[test]
    fn be_mission_int_get_mission_track_matches() {
        check(ssx3_ee::s01::f_00153f30, be_mission_int_get_mission_track, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x3f30, 0);
            fill_missions(c, &mut r, &[0, 1, 2]);
            let idx = seed as u32 % 90;
            c.g[A1][0] = if idx < 88 { (0x1000 + idx) as u64 } else { 0x9999 };
        });
    }

    #[test]
    fn be_mission_int_get_nth_mission_on_track_matches() {
        check(ssx3_ee::s01::f_001545b0, be_mission_int_get_nth_mission_on_track, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x45b0, 0);
            fill_missions(c, &mut r, &[0, 1, 2]);
            c.g[A1][0] = (seed % 5) as u64; // n-th on track
            c.g[A2][0] = (seed % 3) as u64; // track
        });
    }

    #[test]
    fn be_mission_int_session_getters_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_001545f8 as fn(&mut Cpu), be_mission_int_get_session_type as fn(&mut Cpu), 0x45f8),
            (ssx3_ee::s01::f_00154760, be_mission_int_get_session_name, 0x4760),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                for i in 0..22u32 {
                    c.bus.write32(SESSION_TABLE + i * SESSION_STRIDE, 0x2000 + i);
                    fill(c, &mut r, SESSION_TABLE + i * SESSION_STRIDE + 4, 0xa);
                }
                let idx = seed as u32 % 23;
                c.g[A1][0] = if idx < 22 { (0x2000 + idx) as u64 } else { 0x9999 };
            });
        }
    }

    #[test]
    fn be_mission_int_get_session_info_matches() {
        check(ssx3_ee::s01::f_00154678, be_mission_int_get_session_info, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x4678, 0);
            for i in 0..22u32 {
                c.bus.write32(SESSION_TABLE + i * SESSION_STRIDE, 0x2000 + i);
                fill(c, &mut r, SESSION_TABLE + i * SESSION_STRIDE + 4, 0xa);
            }
            let idx = seed as u32 % 23;
            c.g[A2][0] = if idx < 22 { (0x2000 + idx) as u64 } else { 0x9999 };
            c.g[A0][0] = (DATA + 0x1000) as u64;
        });
    }

    #[test]
    fn be_mission_int_get_mission_text_matches() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_00153c88 as fn(&mut Cpu), be_mission_int_get_mission_text1 as fn(&mut Cpu), 0x3c88),
            (ssx3_ee::s01::f_00153cd8, be_mission_int_get_mission_text2, 0x3cd8),
            (ssx3_ee::s01::f_00153d28, be_mission_int_get_mission_text3, 0x3d28),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 1);
                fill_missions(c, &mut r, &[0, 1, 2]);
                let idx = seed as u32 % 90;
                c.g[A1][0] = if idx < 88 { (0x1000 + idx) as u64 } else { 0x9999 };
            });
        }
    }

    #[test]
    fn be_char_int_set_attrib_matches() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_001483a0 as fn(&mut Cpu), be_char_int_set_attrib0 as fn(&mut Cpu), 0x83a0),
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
    fn be_global_ctors_dtors_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_0015c8f0 as fn(&mut Cpu), be_global_ctors as fn(&mut Cpu), 0xc8f0),
            (ssx3_ee::s01::f_0015c910, be_global_dtors, 0xc910),
        ] {
            check(gen, hand, 30, &|c, seed| {
                let _ = machine(c, seed, salt, 0);
            });
        }
    }

    #[test]
    fn be_find_rider_by_profile_matches() {
        check(ssx3_ee::s01::f_0014a130, be_find_rider_by_profile, 40, &|c, seed| {
            let mut r = machine(c, seed, 0xa130, 0);
            // All six rider profile slots index entry 0 of the table.
            for i in 0..6u32 {
                c.bus.write32(0x5305b0 + i * 4, 0);
            }
            fill(c, &mut r, 0x535b20, 0x200);
            let bit = c.bus.read32(0x535b30) & 1;
            c.g[A0][0] = if seed % 2 == 0 { bit as u64 } else { (bit ^ 1) as u64 };
        });
    }

    #[test]
    fn beb_bolt_int_profile_state_matches() {
        for (gen, hand, salt, ptr_arg) in [
            (ssx3_ee::s01::f_0014ad50 as fn(&mut Cpu), beb_bolt_int_get_rider_profile_data as fn(&mut Cpu), 0xad50, true),
            (ssx3_ee::s01::f_0014ae60, beb_bolt_int_get_rider_bolt_states, 0xae60, true),
            (ssx3_ee::s01::f_0014acb0, beb_bolt_int_set_bolt_states, 0xacb0, true),
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

    #[test]
    fn be_econ_int_getters_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_00150960 as fn(&mut Cpu), be_econ_int_get_rider_money as fn(&mut Cpu), 0x0960),
            (ssx3_ee::s01::f_001509f8, be_econ_int_get_rider_total_earned, 0x09f8),
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
        check(ssx3_ee::s01::f_00150ae0, be_econ_int_add_rider_money, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x0ae0, 0);
            c.g[A1][0] = (seed % 8) as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A0][0] = (DATA + 0x400) as u64;
            fill(c, &mut r, 0x530000, 0x800);
        });
    }

    #[test]
    fn be_aggression_int_get_rival_state_matches() {
        check(ssx3_ee::s01::f_00155ab0, be_aggression_int_get_rival_state, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x5ab0, 0);
            c.g[A1][0] = (seed % 8) as u64;
            c.g[A2][0] = (seed % 8) as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x400);
            fill(c, &mut r, DATA, 0x2000);
        });
    }

    #[test]
    fn be_reset_profile_matches() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_001567b8 as fn(&mut Cpu), be_reset_profile_characters as fn(&mut Cpu), 0x67b8),
            (ssx3_ee::s01::f_00156810, be_reset_profile_rivalries, 0x6810),
        ] {
            check(gen, hand, 30, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                fill(c, &mut r, DATA, 0x8000);
            });
        }
    }

    #[test]
    fn be_backup_restore_character_matches() {
        check(ssx3_ee::s01::f_00156950, be_backup_character, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x6950, 0);
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = (seed % 8) as u64;
            fill(c, &mut r, DATA, 0x4000);
        });
        check(ssx3_ee::s01::f_00156988, be_restore_character, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x6988, 0);
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.bus.write8(GAME_GP.wrapping_sub(0x1e94), (seed % 8) as u8);
            fill(c, &mut r, DATA, 0x4000);
        });
    }

    #[test]
    fn be_library_read_save_data_matches() {
        check(ssx3_ee::s01::f_0014e0c0, be_library_read_save_data, 30, &|c, seed| {
            let mut r = machine(c, seed, 0xe0c0, 0);
            c.g[A1][0] = (DATA + 0x400) as u64;
            fill(c, &mut r, DATA, 0x1000);
        });
    }

    #[test]
    fn be_mission_int_collected_matches() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_00153498 as fn(&mut Cpu), be_mission_int_get_collected_count as fn(&mut Cpu), 0x3498),
            (ssx3_ee::s01::f_001535c0, be_mission_int_count_collected_on_peak, 0x35c0),
            (ssx3_ee::s01::f_00153858, be_mission_int_get_collected_mask, 0x3858),
            (ssx3_ee::s01::f_001538e8, be_mission_int_is_collected, 0x38e8),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.g[A3][0] = (seed % 20) as u64;
                c.g[T1][0] = 0;
                fill(c, &mut r, 0x530000, 0x1000);
                fill(c, &mut r, 0x43d000, 0x1000);
            });
        }
    }

    #[test]
    fn be_mission_int_count_completed_matches() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_001542e0 as fn(&mut Cpu), be_mission_int_count_completed_on_track as fn(&mut Cpu), 0x42e0),
            (ssx3_ee::s01::f_001544d0, be_mission_int_count_completed_on_peak_for, 0x44d0),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.g[A3][0] = (seed % 4) as u64;
                // World chain so `world->+0x84` is sometimes present.
                let world = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                fill_missions(c, &mut r, &[0, 1, 2, 3]);
                fill(c, &mut r, 0x530000, 0x1000);
            });
        }
    }

    #[test]
    fn be_library_write_save_data_matches() {
        check(ssx3_ee::s01::f_0014e050, be_library_write_save_data, 30, &|c, seed| {
            let mut r = machine(c, seed, 0xe050, 0);
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            fill(c, &mut r, DATA, 0x4000);
        });
    }

    #[test]
    fn be_char_int_set_all_attribs_matches() {
        check(ssx3_ee::s01::f_00148aa8, be_char_int_set_all_attribs, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x8aa8, 0);
            c.g[A1][0] = (seed % 8) as u64;
            c.g[A2][0] = (r.next_u32() as u64 & 0xff) | 0x100;
            fill(c, &mut r, 0x530000, 0x800);
        });
    }

    #[test]
    fn be_char_int_reset_matches() {
        check(ssx3_ee::s01::f_00148098, be_char_int_reset, 12, &|c, seed| {
            let mut r = machine(c, seed, 0x8098, 0);
            fill(c, &mut r, 0x4a0000, 0x28000 / 4);
        });
    }

    #[test]
    fn be_bagt_int_get_instance_matches() {
        check(ssx3_ee::s01::f_0014f900, be_bagt_int_get_instance, 60, &|c, seed| {
            let mut r = machine(c, seed, 0xf900, 0);
            c.bus.write32(GAME_GP.wrapping_sub(0x1ee0), if seed % 3 == 0 { 0 } else { DATA + 0x100 });
            let _ = &mut r;
        });
    }

    // @@TESTS@@
}
