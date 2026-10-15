//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Trivial leaf replacements. Each copies the generated body's logic, cycle
//! charging and observable register (and FPU condition) results exactly, with
//! a differential test as its gate.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0012_fc60, rider_board_press_sub_enter, "RiderBoardPressSub_Enter"),
    (0x0013_1608, rider_cruise_sub_enter, "RiderCruiseSub_Enter"),
    (0x0013_2048, rider_railslide_sub_exit, "RiderRailslideSub_Exit"),
    (0x0013_2f98, rider_handplant_sub_exit, "RiderHandplantSub_Exit"),
    (0x0014_4be0, be_new_race_int_get_track, "BENewRaceInt_GetTrack"),
    (0x0014_4bf0, be_new_race_int_get_track_name, "BENewRaceInt_GetTrackName"),
    (0x0014_4c48, be_new_race_int_get_track_code_by_index, "BENewRaceInt_GetTrackCodeByIndex"),
    (0x0014_8b78, be_char_int_get_attrib0_by_char, "BECharInt_GetAttrib0ByChar"),
    (0x0014_8bc8, be_char_int_get_attrib1_by_char, "BECharInt_GetAttrib1ByChar"),
    (0x0014_8c18, be_char_int_get_attrib2_by_char, "BECharInt_GetAttrib2ByChar"),
    (0x0014_8c68, be_char_int_get_attrib3_by_char, "BECharInt_GetAttrib3ByChar"),
    (0x0014_8cb8, be_char_int_get_attrib4_by_char, "BECharInt_GetAttrib4ByChar"),
    (0x0014_8d08, be_char_int_get_attrib5_by_char, "BECharInt_GetAttrib5ByChar"),
    (0x0014_8d58, be_char_int_get_attrib6_by_char, "BECharInt_GetAttrib6ByChar"),
    (0x0014_ad28, be_get_char_profile_data, "BE_GetCharProfileData"),
    (0x0014_ae28, be_bolt_int_get_bolt_states, "BEBoltInt_GetBoltStates"),
    (0x0014_bdb8, bolts_get_db, "Bolts_GetDB"),
    (0x0015_a398, be_reward_int_get_track_peak, "BERewardInt_GetTrackPeak"),
    (0x0015_a478, be_reward_int_get_rewards_file, "BERewardInt_GetRewardsFile"),
    (0x0015_a750, be_reward_int_get_new_reward_count, "BERewardInt_GetNewRewardCount"),
    (0x0015_a768, be_reward_int_get_new_reward_bits, "BERewardInt_GetNewRewardBits"),
    (0x0015_a778, rewards_file_ctor, "RewardsFile_ctor"),
    (0x0015_c930, algorithm_list_ctor, "AlgorithmList_ctor"),
    (0x0016_1950, chase_cam_controller_init_state, "ChaseCamController_InitState"),
    (0x0016_7d88, chase_camera_algorithm_default_heading_p, "ChaseCameraAlgorithm_DefaultHeadingP"),
    (0x0016_7d98, chase_camera_algorithm_default_heading_i, "ChaseCameraAlgorithm_DefaultHeadingI"),
    (0x0016_7da8, chase_camera_algorithm_default_heading_d, "ChaseCameraAlgorithm_DefaultHeadingD"),
    (0x0016_bd30, math_wrap_angle_clamp_half_pi, "Math_WrapAngleClampHalfPi"),
    (0x0016_c120, camera_trigger_man_add_trigger, "CameraTriggerMan_AddTrigger"),
    (0x0016_c5d0, active_camera_trigger_list_ctor, "ActiveCameraTriggerList_ctor"),
    (0x0017_1680, camera_trigger_read_action_type3, "CameraTrigger_ReadActionType3"),
    (0x0017_31c0, vec3_max_component, "Vec3_MaxComponent"),
    (0x0017_abc8, debug_menu_init_background_list, "DebugMenu_InitBackgroundList"),
    (0x0017_ac98, debug_menu_select_background, "DebugMenu_SelectBackground"),
    (0x0019_c8d0, bolt_file_index_manager_get_prev_node, "BoltFileIndexManager_GetPrevNode"),
    (0x0019_c8f8, bolt_file_index_manager_get_next_node, "BoltFileIndexManager_GetNextNode"),
    (0x0019_c920, bolt_file_index_manager_node_index, "BoltFileIndexManager_NodeIndex"),
    (0x0026_28c0, buddy_man_map_presence_state, "BuddyMan_MapPresenceState"),
    (0x0026_d628, world_state_stream_reset, "WorldStateStream_Reset"),
    (0x0028_b730, snd_bank_unlink, "SndBank_Unlink"),
    (0x002a_d980, bank_monitor_timer_enable, "BankMonitor_TimerEnable"),
    (0x002a_ec28, path_plat_mark_track_dirty, "PATH_PlatMarkTrackDirty"),
    (0x002a_f4b0, path_plat_get_track_aux_send, "PATH_PlatGetTrackAuxSend"),
    (0x002a_f8e0, speech_bank_ctor, "SpeechBank_ctor"),
    (0x002b_1220, speech_system_get_instance_stream_slot, "SpeechSystem_GetInstanceStreamSlot"),
    (0x002b_2850, song_instance_apply_delay_params, "SongInstance_ApplyDelayParams"),
    (0x002b_48b8, song_instance_get_sed_value, "SongInstance_GetSedValue"),
    (0x002b_4908, music_player_get_selected_song_sed_value, "MusicPlayer_GetSelectedSongSedValue"),
    (0x002b_4af0, music_player_is_current_song_enabled, "MusicPlayer_IsCurrentSongEnabled"),
    (0x002b_4b70, world_trigger_instance_ctor, "WorldTriggerInstance_ctor"),
];

// ---------------------------------------------------------------------------
// Rider sub-states.

/// `void RiderBoardPressSub_Enter(RiderBoardPressSub *self)` (0x12fc60):
/// reset the sub-state and its rider's board-press words.
pub fn rider_board_press_sub_enter(c: &mut Cpu) {
    c.cycles += 8;
    let this = Addr(abi::arg(c, 0));
    let rider = this.off(0x14).lw(c);
    c.g[V0][0] = rider;
    this.sw(c, 0);
    Addr((rider as u32).wrapping_add(0x360)).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `void RiderCruiseSub_Enter(RiderCruiseSub *self)` (0x131608): clear the
/// two board-press words of the rider referenced at +0.
pub fn rider_cruise_sub_enter(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    let handle = this.lw(c);
    c.g[V0][0] = handle;
    Addr((handle as u32).wrapping_add(0x35c)).sw(c, 0);
    let handle = this.lw(c);
    c.g[V1][0] = handle;
    Addr((handle as u32).wrapping_add(0x360)).sw(c, 0);
    abi::ret(c);
}

/// `void RiderRailslideSub_Exit(RiderRailslideSub *self)` (0x132048): clear
/// the railslide word and restore its pitch constant.
pub fn rider_railslide_sub_exit(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    let rider = this.off(0x8).lw(c);
    c.g[V0][0] = rider;
    let pitch = Addr((c.g[GP][0] as u32).wrapping_sub(0x7594)).read_fpu(c);
    c.f[0] = pitch.0;
    Addr((rider as u32).wrapping_add(0x240)).sw(c, 0);
    Addr((rider as u32).wrapping_add(0x23c)).write_fpu(c, pitch);
    abi::ret(c);
}

/// `void RiderHandplantSub_Exit(RiderHandplantSub *self)` (0x132f98): clear
/// the handplant words and restore its pitch constant.
pub fn rider_handplant_sub_exit(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    let rider = this.off(0x14).lw(c);
    c.g[V0][0] = rider;
    let pitch = Addr((c.g[GP][0] as u32).wrapping_sub(0x754c)).read_fpu(c);
    c.f[0] = pitch.0;
    Addr((rider as u32).wrapping_add(0x32c)).sw(c, 0);
    let rider = this.off(0x14).lw(c);
    c.g[V1][0] = rider;
    Addr((rider as u32).wrapping_add(0x248)).write_fpu(c, pitch);
    Addr((rider as u32).wrapping_add(0x24c)).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// New-race / character / bolt / reward tables.

/// The base of the "new race" track table.
const NEW_RACE_TRACK: u32 = 0x0053_0000;
/// Offset of the selected-track pointer within that table.
const NEW_RACE_TRACK_PTR: u32 = 0x5c08;
/// The base of the per-track record table.
const TRACK_RECORDS: u32 = 0x0044_0000;
/// The base of the character-attribute table.
const CHAR_ATTRIB: u32 = 0x0053_0000;
/// Offset of the character-attribute table within its base.
const CHAR_ATTRIB_OFF: u32 = 0x5538;
/// The base of the per-bolt record table.
const BOLT_RECORDS: u32 = 0x004a_0000;
/// Offset of the per-bolt record table within its base.
const BOLT_RECORDS_OFF: u32 = 0x6ca8;
/// The base of the rewards file.
const REWARDS_FILE: u32 = 0x004c_0000;

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

/// Shared body of the `BECharInt_GetAttribNByChar` accessors (0x148b78+2n):
/// index `char` into the attribute table (`char*70 + char2*7`) and return the
/// byte at `+field`.
fn be_char_int_get_attrib(c: &mut Cpu, field: u32) {
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

/// `void* BE_GetCharProfileData(int char, int char2)` (0x14ad28): index into
/// the character-profile table (`char2*0xf88 + char*0x9b50`).
pub fn be_get_char_profile_data(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V1][0] = 0xf88;
    c.g[A0][0] = 0x9b50;
    c.mult(0, V0, A2, V1);
    c.mult(0, A1, A1, A0);
    c.g[V1][0] = BOLT_RECORDS as u64;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(BOLT_RECORDS_OFF));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

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

/// `void* Bolts_GetDB(void)` (0x14bdb8): the bolt database at `0x4a6750`.
pub fn bolts_get_db(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = BOLT_RECORDS as u64;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6750));
    abi::ret(c);
}

/// `int BERewardInt_GetTrackPeak(int index)` (0x15a398): the peak score of the
/// `index`-th track record.
pub fn be_reward_int_get_track_peak(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x64;
    c.g[V1][0] = TRACK_RECORDS as u64;
    c.mult(0, A1, A1, V0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x26b0));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x54)).lw(c);
    abi::ret(c);
}

/// `void* BERewardInt_GetRewardsFile(void)` (0x15a478): the rewards file at
/// `0x4c3e98`.
pub fn be_reward_int_get_rewards_file(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = REWARDS_FILE as u64;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
    abi::ret(c);
}

/// `int BERewardInt_GetNewRewardCount(int self, int reward, int sub)`
/// (0x15a750): read a byte out of the reward record.
pub fn be_reward_int_get_new_reward_count(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0xa;
    c.mult(0, V1, A1, V0);
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x6)).lb(c);
    abi::ret(c);
}

/// `int BERewardInt_GetNewRewardBits(int self, int reward)` (0x15a768): the
/// reward's bit word.
pub fn be_reward_int_get_new_reward_bits(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x174)).lw(c);
    abi::ret(c);
}

/// `RewardsFile* RewardsFile_ctor(RewardsFile *self)` (0x15a778): clear the
/// record, set the first word to -1, and return `self`.
pub fn rewards_file_ctor(c: &mut Cpu) {
    c.cycles += 22;
    let a0 = abi::arg64(c, 0);
    let this = Addr(a0 as u32);
    c.g[V0][0] = u64::MAX;
    this.off(0x4).sw(c, 0);
    this.sw(c, u64::MAX);
    c.g[V0][0] = a0;
    for off in [0x8u32, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x44, 0x48, 0x4c] {
        this.off(off).sw(c, 0);
    }
    abi::ret(c);
}

/// `AlgorithmList* AlgorithmList_ctor(AlgorithmList *self)` (0x15c930):
/// clear the list head/tail and return `self`.
pub fn algorithm_list_ctor(c: &mut Cpu) {
    c.cycles += 4;
    let a0 = abi::arg64(c, 0);
    let this = Addr(a0 as u32);
    c.g[V0][0] = a0;
    this.sw(c, 0);
    this.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `void ChaseCamController_InitState(ChaseCamController *self, int index)`
/// (0x161950): mask the flags at +0x30, set the counts at +0x1c/+0x24/+0x28/
/// +0x2c, and link in an algorithm from `gp + 0x1ce0`.
pub fn chase_cam_controller_init_state(c: &mut Cpu) {
    c.cycles += 22;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(0x30).lw(c);
    c.g[V1][0] = 0xffff_ffff_ffff_fffd;
    c.g[A3][0] = 0xffff_ffff_ffff_fffe;
    c.g[T0][0] = 0xffff_ffff_ffff_fffb;
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[A2][0] = 0x3d;
    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
    this.off(0x1c).sw(c, 0x3d);
    this.off(0x38).sw(c, 0);
    c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1ce0));
    this.off(0x30).sw(c, c.g[V0][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    this.off(0x3c).sw(c, 0);
    this.off(0x40).sw(c, 0);
    this.off(0x2c).sw(c, 0x3d);
    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
    this.off(0x28).sw(c, 0x3d);
    this.off(0x24).sw(c, c.g[V0][0]);
    this.off(0x34).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Camera algorithms and triggers.

/// `float ChaseCameraAlgorithm_DefaultHeadingP(void)` (0x167d88): the pitch
/// heading constant.
pub fn chase_camera_algorithm_default_heading_p(c: &mut Cpu) {
    c.cycles += 3;
    c.f[0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x6788)).read_fpu(c).0;
    abi::ret(c);
}

/// `float ChaseCameraAlgorithm_DefaultHeadingI(void)` (0x167d98): the yaw
/// heading constant.
pub fn chase_camera_algorithm_default_heading_i(c: &mut Cpu) {
    c.cycles += 3;
    c.f[0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x6784)).read_fpu(c).0;
    abi::ret(c);
}

/// `float ChaseCameraAlgorithm_DefaultHeadingD(void)` (0x167da8): the roll
/// heading constant.
pub fn chase_camera_algorithm_default_heading_d(c: &mut Cpu) {
    c.cycles += 3;
    c.f[0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x6780)).read_fpu(c).0;
    abi::ret(c);
}

/// `float Math_WrapAngleClampHalfPi(float angle)` (0x16bd30): wrap an angle
/// into [-pi/2, pi/2) using a rounding step and a final clamp.
pub fn math_wrap_angle_clamp_half_pi(c: &mut Cpu) {
    let angle = Addr(abi::arg(c, 0));
    let half = FpuF32(0x3f00_0000);
    c.cycles += 11;
    let scale = Addr((c.g[GP][0] as u32).wrapping_sub(0x6470)).read_fpu(c);
    let input = angle.read_fpu(c);
    c.f[4] = input.0;
    c.g[AT][0] = 0x3f00_0000;
    c.f[1] = half.0;
    let f0 = scale * input + half;
    let n = f0.to_i32();
    let mut f2 = FpuF32::from_i32(n);
    c.f[2] = f2.0;
    let lt = f0.lt(f2);
    abi::set_fcc(c, lt);
    if lt {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = 0x3f80_0000;
        f2 = f2 - FpuF32::ONE;
        c.f[2] = f2.0;
    }
    c.cycles += 10;
    let a = Addr((c.g[GP][0] as u32).wrapping_sub(0x646c)).read_fpu(c);
    c.f[0] = a.0;
    let b = Addr((c.g[GP][0] as u32).wrapping_sub(0x6468)).read_fpu(c);
    c.f[1] = b.0;
    let mut f0 = a * f2;
    let cc = Addr((c.g[GP][0] as u32).wrapping_sub(0x6464)).read_fpu(c);
    c.f[2] = cc.0;
    c.f[3] = b.0;
    f0 = input - f0;
    c.f[0] = f0.0;
    let le = b.le(f0);
    abi::set_fcc(c, le);
    angle.write_fpu(c, f0);
    if le {
        c.cycles += 1;
        c.f[3] = f0.min(cc).0;
    }
    c.cycles += 2;
    angle.write_fpu(c, FpuF32(c.f[3]));
    abi::ret(c);
}

/// `void CameraTriggerMan_AddTrigger(CameraTriggerMan *man, int value, int index)`
/// (0x16c120): append `value` at `index` (or at the running count when `index`
/// is negative and there is room).
pub fn camera_trigger_man_add_trigger(c: &mut Cpu) {
    let man = Addr(abi::arg(c, 0));
    let value = abi::arg64(c, 1);
    let index = c.g[A2][0];
    c.cycles += 2;
    if (index as i64) < 0 {
        c.cycles += 4;
        let count = man.off(0x4).lw(c);
        c.g[V1][0] = count;
        let room = (count as i64) < 120;
        c.g[V0][0] = room as u64;
        if room {
            let base = man.lw(c);
            c.g[V0][0] = base;
            c.cycles += 7;
            let slot = (count as u32).wrapping_shl(2).wrapping_add(base as u32);
            c.g[V1][0] = sx(slot);
            Addr(slot).sw(c, value);
            c.g[V0][0] = man.off(0x4).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            man.off(0x4).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.g[V1][0] = man.lw(c);
    c.cycles += 3;
    c.g[V0][0] = sx((index as u32) << 2);
    let slot = (c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32);
    c.g[V0][0] = sx(slot);
    Addr(slot).sw(c, value);
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `ActiveCameraTriggerList* ActiveCameraTriggerList_ctor(ActiveCameraTriggerList *self)`
/// (0x16c5d0): clear the head and return `self`.
pub fn active_camera_trigger_list_ctor(c: &mut Cpu) {
    c.cycles += 3;
    let a0 = abi::arg64(c, 0);
    c.g[V0][0] = a0;
    Addr(a0 as u32).sw(c, 0);
    abi::ret(c);
}

/// `int CameraTrigger_ReadActionType3(CameraTrigger *self, int *out)`
/// (0x171680): store the action type 3 and return 0.
pub fn camera_trigger_read_action_type3(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x3;
    c.g[V0][0] = 0;
    Addr(abi::arg(c, 1)).sw(c, 0x3);
    abi::ret(c);
}

/// `float Vec3_MaxComponent(const Vec3 *v)` (0x1731c0): the largest of the
/// three components.
pub fn vec3_max_component(c: &mut Cpu) {
    let v = Addr(abi::arg(c, 0));
    c.cycles += 6;
    let f2 = v.read_fpu(c);
    c.f[2] = f2.0;
    let f1 = v.off(0x4).read_fpu(c);
    c.f[1] = f1.0;
    let y_lt_x = f1.lt(f2);
    abi::set_fcc(c, y_lt_x);
    let f0 = v.off(0x8).read_fpu(c);
    c.f[0] = f0.0;
    if !y_lt_x {
        c.cycles += 4;
        let z_lt_y = f0.lt(f1);
        abi::set_fcc(c, z_lt_y);
        if z_lt_y {
            c.f[0] = f1.0;
        }
    } else {
        c.cycles += 3;
        let z_lt_x = f0.lt(f2);
        abi::set_fcc(c, z_lt_x);
        if z_lt_x {
            c.cycles += 2;
            c.f[0] = f2.0;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `DebugMenuBackgroundList* DebugMenu_InitBackgroundList(DebugMenuBackgroundList *self)`
/// (0x17abc8): clear the list and set its cursor to -1.
pub fn debug_menu_init_background_list(c: &mut Cpu) {
    c.cycles += 6;
    let a0 = abi::arg64(c, 0);
    let this = Addr(a0 as u32);
    c.g[V0][0] = u64::MAX;
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, u64::MAX);
    c.g[V0][0] = a0;
    this.sw(c, 0);
    abi::ret(c);
}

/// `int DebugMenu_SelectBackground(DebugMenuBackgroundList *self, int index)`
/// (0x17ac98): store `index` at +0x8 when it is in `[0, count)`; otherwise
/// store -1. Returns 1.
pub fn debug_menu_select_background(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    let index = c.g[A1][0];
    c.cycles += 2;
    let mut select = true;
    let mut stored = u64::MAX;
    'outer: {
        if (index as i64) < 0 {
            stored = u64::MAX;
            break 'outer;
        }
        c.cycles += 4;
        let limit = this.off(0x4).lw(c);
        c.g[V0][0] = limit;
        let in_range = (index as i64) < (limit as i64);
        c.g[V0][0] = in_range as u64;
        if !in_range {
            stored = u64::MAX;
            break 'outer;
        }
        c.cycles += 2;
        this.off(0x8).sw(c, index);
        select = false;
    }
    if select {
        c.cycles += 1;
        c.g[V0][0] = stored;
        this.off(0x8).sw(c, stored);
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Bolt file index manager.

/// Shared body of `BoltFileIndexManager_GetPrev/NextNode` (0x19c8d0/0x19c8f8):
/// follow the 16-bit node link at `node + link` and return `base + index*16`.
fn bolt_file_index_manager_get_node(c: &mut Cpu, link: u32) {
    c.cycles += 2;
    let node = abi::arg64(c, 1);
    if node != 0 {
        c.cycles += 3;
        let index = Addr(node as u32).off(link).lh(c);
        c.g[V0][0] = index;
        let negative = (index as i64) < 0;
        let scaled = sx((index as u32) << 4);
        c.g[V0][0] = scaled;
        if !negative {
            c.cycles += 2;
            c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(scaled as u32));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `Node* BoltFileIndexManager_GetPrevNode(Node *base, Node *node)` (0x19c8d0).
pub fn bolt_file_index_manager_get_prev_node(c: &mut Cpu) {
    bolt_file_index_manager_get_node(c, 0xc);
}

/// `Node* BoltFileIndexManager_GetNextNode(Node *base, Node *node)` (0x19c8f8).
pub fn bolt_file_index_manager_get_next_node(c: &mut Cpu) {
    bolt_file_index_manager_get_node(c, 0xe);
}

/// `int BoltFileIndexManager_NodeIndex(Node *base, Node *node)` (0x19c920):
/// `(node - base) >> 4`, or -1 for a null node.
pub fn bolt_file_index_manager_node_index(c: &mut Cpu) {
    c.cycles += 2;
    let node = abi::arg64(c, 1);
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(abi::arg(c, 0)));
    if node != 0 {
        c.cycles += 2;
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// Buddies / world state / sound banks.

/// `int BuddyMan_MapPresenceState(BuddyMan *man, int state)` (0x2628c0): map a
/// buddy state to a presence state, or -1 when the manager has no presence
/// object.
pub fn buddy_man_map_presence_state(c: &mut Cpu) {
    c.cycles += 3;
    let man = Addr(abi::arg(c, 0));
    let presence = man.off(0xc8).lw(c);
    c.g[V0][0] = 0x1;
    if presence == 0 {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let state = c.g[A1][0];
    let is_one = state == c.g[V0][0];
    c.g[V0][0] = ((state as i64) < 2) as u64;
    if is_one {
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        abi::ret(c);
        return;
    }
    'presence_low: {
        'presence_zero: {
            'presence_ge2: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0;
                c.g[V0][0] = 0x2;
                if t {
                    break 'presence_ge2;
                } else {
                    c.cycles += 2;
                    let t = state == 0;
                    c.g[V0][0] = u64::MAX;
                    if t {
                        break 'presence_zero;
                    } else {
                        c.cycles += 2;
                        break 'presence_low;
                    }
                }
            }
            c.cycles += 2;
            let t = state == c.g[V0][0];
            c.g[V0][0] = 0x3;
            if t {
                c.cycles += 2;
                c.g[V0][0] = 0x2;
                abi::ret(c);
                return;
            } else {
                c.cycles += 2;
                let t = state == c.g[V0][0];
                c.g[V0][0] = u64::MAX;
                if t {
                    c.cycles += 2;
                    c.g[V0][0] = 0x3;
                    abi::ret(c);
                    return;
                } else {
                    c.cycles += 2;
                    break 'presence_low;
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `WorldStateStream* WorldStateStream_Reset(WorldStateStream *self)`
/// (0x26d628): reset the stream's counters and the 12-byte field at +0x1c.
pub fn world_state_stream_reset(c: &mut Cpu) {
    c.cycles += 20;
    let stream = Addr(abi::arg(c, 0));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = u64::MAX;
    let sp = c.g[SP][0] as u32;
    Addr(sp).sh(c, 0);
    Addr(sp.wrapping_add(0x2)).sb(c, 0);
    Addr(sp.wrapping_add(0x3)).sb(c, 0);
    stream.off(0x30).sw(c, 0);
    stream.off(0x34).sw(c, u64::MAX);
    c.lwl(V1, sp.wrapping_add(0x3));
    c.lwr(V1, sp);
    c.swl(V1, stream.0.wrapping_add(0x1f));
    c.swr(V1, stream.0.wrapping_add(0x1c));
    stream.off(0x2c).sw(c, u64::MAX);
    stream.off(0x10).sw(c, 0);
    stream.off(0x20).sw(c, u64::MAX);
    stream.off(0x24).sw(c, u64::MAX);
    stream.off(0x28).sw(c, u64::MAX);
    stream.off(0x8).sw(c, 0);
    stream.off(0xc).sw(c, 0);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

/// `void SndBank_Unlink(SndBank *bank)` (0x28b730): unlink the bank from its
/// doubly-linked list and clear its link fields.
pub fn snd_bank_unlink(c: &mut Cpu) {
    let bank = Addr(abi::arg(c, 0));
    'outer: {
        c.cycles += 3;
        let next = bank.off(0x18).lw(c);
        c.g[V1][0] = next;
        c.g[V0][0] = u64::MAX;
        if next == 0 {
            break 'outer;
        }
        'prev_link: {
            'forward_link: {
                c.cycles += 6;
                c.g[A1][0] = bank.off(0x1c).lw(c);
                bank.off(0xc).sw(c, u64::MAX);
                bank.off(0x10).sw(c, 0);
                bank.off(0x14).sw(c, 0);
                bank.off(0x8).sw(c, 0);
                if next != c.g[A1][0] {
                    break 'forward_link;
                }
                c.cycles += 4;
                Addr(next as u32).off(0x18).sw(c, 0);
                c.g[V0][0] = bank.off(0x18).lw(c);
                Addr(c.g[V0][0] as u32).off(0x1c).sw(c, 0);
                break 'prev_link;
            }
            c.cycles += 4;
            Addr(next as u32).off(0x1c).sw(c, c.g[A1][0]);
            c.g[V1][0] = bank.off(0x1c).lw(c);
            c.g[V0][0] = bank.off(0x18).lw(c);
            Addr(c.g[V1][0] as u32).off(0x18).sw(c, c.g[V0][0]);
            break 'prev_link;
        }
        c.cycles += 2;
        bank.off(0x18).sw(c, 0);
        bank.off(0x1c).sw(c, 0);
        break 'outer;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void BankMonitor_TimerEnable(BankMonitorTimer *t)` (0x2ad980): set the
/// timer's enable flag.
pub fn bank_monitor_timer_enable(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    Addr(abi::arg(c, 0)).sw(c, 0x1);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Path platform table.

/// The base of the path platform pointer table.
const PLATFORM_TABLE: u32 = 0x004d_3e98;

/// `bool PATH_PlatMarkTrackDirty(int index)` (0x2aec28): set the dirty byte of
/// the platform at `index`, or report an empty slot (-8).
pub fn path_plat_mark_track_dirty(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = (PLATFORM_TABLE & 0xffff_0000) as u64;
    c.g[A0][0] = sx(abi::arg(c, 0) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(PLATFORM_TABLE & 0xffff));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V1][0] = 0x1;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_fff8;
        abi::ret(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        Addr((c.g[A0][0] as u32).wrapping_add(0x65)).sb(c, 0x1);
        abi::ret(c);
    }
}

/// `int PATH_PlatGetTrackAuxSend(int index)` (0x2af4b0): the aux-send word of
/// the platform at `index`, or -8 for an empty slot.
pub fn path_plat_get_track_aux_send(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = (PLATFORM_TABLE & 0xffff_0000) as u64;
    c.g[A0][0] = sx(abi::arg(c, 0) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(PLATFORM_TABLE & 0xffff));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = 0xffff_ffff_ffff_fff8;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c);
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// Speech / music / world trigger.

/// `SpeechBank* SpeechBank_ctor(SpeechBank *self)` (0x2af8e0): clear the bank
/// and return `self`.
pub fn speech_bank_ctor(c: &mut Cpu) {
    c.cycles += 7;
    let a0 = abi::arg64(c, 0);
    let this = Addr(a0 as u32);
    c.g[V0][0] = u64::MAX;
    this.off(0x48).sw(c, 0);
    this.sw(c, u64::MAX);
    c.g[V0][0] = a0;
    this.off(0x44).sw(c, 0);
    this.off(0x4).sb(c, 0);
    abi::ret(c);
}

/// `void* SpeechSystem_GetInstanceStreamSlot(SpeechSystem *sys, int slot)`
/// (0x2b1220): the stream at `sys->slots[slot] + 4`, or -1.
pub fn speech_system_get_instance_stream_slot(c: &mut Cpu) {
    c.cycles += 6;
    let sys = Addr(abi::arg(c, 0));
    c.g[V0][0] = sys.off(0x4).lw(c);
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = u64::MAX;
    if c.g[A1][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x4)).lw(c);
        abi::ret(c);
    }
}

/// `void SongInstance_ApplyDelayParams(SongInstance *player, SongInstance *a, int b)`
/// (0x2b2850): copy three delay-parameter fields of `a`/`b` into the
/// player's DSP delay table.
pub fn song_instance_apply_delay_params(c: &mut Cpu) {
    c.cycles += 28;
    let p = abi::arg(c, 0);
    let q = abi::arg(c, 1);
    // First delay row: +0x4.
    let v0 = Addr(q).lw(c);
    let v1 = Addr(p).lw(c);
    let a2 = Addr(v0 as u32).lw(c);
    let a3 = Addr((v1 as u32).wrapping_add(0x78)).lw(c);
    let v0 = Addr((a2 as u32).wrapping_add(0x1f4)).lw(c);
    let v1 = Addr((a2 as u32).wrapping_add(0x248)).lw(c);
    let v0 = sx((v0 as u32) << 4);
    let v0 = sx((v0 as u32).wrapping_add(v1 as u32));
    Addr((v0 as u32).wrapping_add(0x4)).sw(c, a3);
    // Second delay row: +0x8.
    let v1 = Addr(q).lw(c);
    let a2 = Addr(p).lw(c);
    let a3 = Addr(v1 as u32).lw(c);
    let t0 = Addr((a2 as u32).wrapping_add(0x7c)).lw(c);
    let v0 = Addr((a3 as u32).wrapping_add(0x1f4)).lw(c);
    let v1 = Addr((a3 as u32).wrapping_add(0x248)).lw(c);
    let v0 = sx((v0 as u32) << 4);
    let v0 = sx((v0 as u32).wrapping_add(v1 as u32));
    Addr((v0 as u32).wrapping_add(0x8)).sw(c, t0);
    // Third delay row: +0x0.
    let v1 = Addr(q).lw(c);
    let a1 = Addr(p).lw(c);
    let a0 = Addr(v1 as u32).lw(c);
    let a2 = Addr((a1 as u32).wrapping_add(0x80)).lw(c);
    let v0 = Addr((a0 as u32).wrapping_add(0x1f4)).lw(c);
    let v1 = Addr((a0 as u32).wrapping_add(0x248)).lw(c);
    let v0 = sx((v0 as u32) << 4);
    let v0 = sx((v0 as u32).wrapping_add(v1 as u32));
    Addr(v0 as u32).sw(c, a2);
    c.g[V0][0] = v0;
    c.g[V1][0] = v1;
    c.g[A0][0] = a0;
    c.g[A1][0] = a1;
    c.g[A2][0] = a2;
    c.g[A3][0] = a3;
    c.g[T0][0] = t0;
    abi::ret(c);
}

/// `int SongInstance_GetSedValue(SongInstance *s)` (0x2b48b8): read the sed
/// value at +0x98 of the owning song.
pub fn song_instance_get_sed_value(c: &mut Cpu) {
    c.cycles += 3;
    let owner = Addr(abi::arg(c, 0)).lw(c);
    c.g[V1][0] = owner;
    c.g[V0][0] = Addr((owner as u32).wrapping_add(0x98)).lw(c);
    abi::ret(c);
}

/// `int MusicPlayer_GetSelectedSongSedValue(MusicPlayer *mp)` (0x2b4908):
/// the sed value of the currently selected song.
pub fn music_player_get_selected_song_sed_value(c: &mut Cpu) {
    c.cycles += 6;
    let mp = Addr(abi::arg(c, 0));
    c.g[V0][0] = mp.off(0x3f0).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[A0][0] = sx(mp.0.wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x98)).lw(c);
    abi::ret(c);
}

/// `bool MusicPlayer_IsCurrentSongEnabled(MusicPlayer *mp)` (0x2b4af0):
/// whether the bit for the current song is set in the 64-bit enable mask at
/// +0x3f8.
pub fn music_player_is_current_song_enabled(c: &mut Cpu) {
    c.cycles += 5;
    let mp = Addr(abi::arg(c, 0));
    c.g[V1][0] = mp.off(0x3f0).lw(c);
    c.g[V0][0] = mp.off(0x3f8).ld(c);
    c.g[V0][0] = c.g[V0][0] >> (c.g[V1][0] & 63);
    c.g[V0][0] = c.g[V0][0] & 0x1;
    abi::ret(c);
}

/// `WorldTriggerInstance* WorldTriggerInstance_ctor(WorldTriggerInstance *self)`
/// (0x2b4b70): clear the instance, arm it from the global trigger flag and
/// set its timer to -1.0.
pub fn world_trigger_instance_ctor(c: &mut Cpu) {
    c.cycles += 27;
    let this = Addr(abi::arg(c, 0));
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = 0x3f80_0000;
    c.g[V0][0] = u64::MAX;
    this.off(0x30).sw(c, u64::MAX);
    c.g[A1][0] = 0x004a_0000;
    this.off(0x2c).write_fpu(c, FpuF32(0x3f80_0000));
    c.g[V0][0] = abi::arg64(c, 0);
    this.sw(c, 0);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    this.off(0x20).sw(c, 0);
    this.off(0x24).sw(c, 0);
    this.off(0x28).sw(c, 0);
    this.off(0x34).sw(c, 0);
    this.off(0x38).sw(c, 0);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.f[0] = 0xbf80_0000;
    c.g[V1][0] = Addr(0x004a_0000 + 0x35a8).lbu(c);
    this.off(0x68).write_fpu(c, FpuF32(0xbf80_0000));
    this.off(0x40).sb(c, c.g[V1][0]);
    this.off(0x60).sw(c, 0);
    this.off(0x64).sw(c, 0);
    this.off(0x6c).sw(c, 0);
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn rider_board_press_sub_enter_matches() {
        check(ssx3_ee::s00::f_0012fc60, rider_board_press_sub_enter, 200, &|c, seed| {
            let mut r = Rng(0x1001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x14, DATA + 0x400);
        });
    }

    #[test]
    fn rider_cruise_sub_enter_matches() {
        check(ssx3_ee::s00::f_00131608, rider_cruise_sub_enter, 200, &|c, seed| {
            let mut r = Rng(0x1002 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, DATA + 0x400);
        });
    }

    #[test]
    fn rider_railslide_sub_exit_matches() {
        check(ssx3_ee::s01::f_00132048, rider_railslide_sub_exit, 200, &|c, seed| {
            let mut r = Rng(0x1003 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x7594), r.next_u32());
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x8, DATA + 0x400);
        });
    }

    #[test]
    fn rider_handplant_sub_exit_matches() {
        check(ssx3_ee::s01::f_00132f98, rider_handplant_sub_exit, 200, &|c, seed| {
            let mut r = Rng(0x1004 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x754c), r.next_u32());
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x14, DATA + 0x400);
        });
    }

    #[test]
    fn be_new_race_int_get_track_matches() {
        check(ssx3_ee::s01::f_00144be0, be_new_race_int_get_track, 100, &|c, seed| {
            let mut r = Rng(0x1005 ^ seed as u64);
            scramble(c, &mut r);
            c.bus.write32(NEW_RACE_TRACK + NEW_RACE_TRACK_PTR, r.next_u32());
        });
    }

    #[test]
    fn be_new_race_int_get_track_name_matches() {
        check_lohi(ssx3_ee::s01::f_00144bf0, be_new_race_int_get_track_name, 100, &|c, seed| {
            let mut r = Rng(0x1006 ^ seed as u64);
            scramble(c, &mut r);
            c.bus.write32(NEW_RACE_TRACK + NEW_RACE_TRACK_PTR, r.next_u32());
        });
    }

    #[test]
    fn be_new_race_int_get_track_code_by_index_matches() {
        check_lohi(ssx3_ee::s01::f_00144c48, be_new_race_int_get_track_code_by_index, 200, &|c, seed| {
            let mut r = Rng(0x1007 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = (r.next_u32() % 16) as u64;
        });
    }

    /// Shared setup for the seven attribute accessors: index into the
    /// character-attribute table with small indices and seed the bytes read.
    fn char_attrib_setup(c: &mut Cpu, seed: u32) {
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
        check_lohi(ssx3_ee::s01::f_00148b78, be_char_int_get_attrib0_by_char, 200, &char_attrib_setup);
    }

    #[test]
    fn be_char_int_get_attrib1_by_char_matches() {
        check_lohi(ssx3_ee::s01::f_00148bc8, be_char_int_get_attrib1_by_char, 200, &char_attrib_setup);
    }

    #[test]
    fn be_char_int_get_attrib2_by_char_matches() {
        check_lohi(ssx3_ee::s01::f_00148c18, be_char_int_get_attrib2_by_char, 200, &char_attrib_setup);
    }

    #[test]
    fn be_char_int_get_attrib3_by_char_matches() {
        check_lohi(ssx3_ee::s01::f_00148c68, be_char_int_get_attrib3_by_char, 200, &char_attrib_setup);
    }

    #[test]
    fn be_char_int_get_attrib4_by_char_matches() {
        check_lohi(ssx3_ee::s01::f_00148cb8, be_char_int_get_attrib4_by_char, 200, &char_attrib_setup);
    }

    #[test]
    fn be_char_int_get_attrib5_by_char_matches() {
        check_lohi(ssx3_ee::s01::f_00148d08, be_char_int_get_attrib5_by_char, 200, &char_attrib_setup);
    }

    #[test]
    fn be_char_int_get_attrib6_by_char_matches() {
        check_lohi(ssx3_ee::s01::f_00148d58, be_char_int_get_attrib6_by_char, 200, &char_attrib_setup);
    }

    #[test]
    fn be_get_char_profile_data_matches() {
        check_lohi(ssx3_ee::s01::f_0014ad28, be_get_char_profile_data, 200, &|c, seed| {
            let mut r = Rng(0x14ad ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = (r.next_u32() % 16) as u64;
            c.g[A2][0] = (r.next_u32() % 16) as u64;
        });
    }

    #[test]
    fn be_bolt_int_get_bolt_states_matches() {
        check_lohi(ssx3_ee::s01::f_0014ae28, be_bolt_int_get_bolt_states, 200, &|c, seed| {
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
        });
    }

    #[test]
    fn bolts_get_db_matches() {
        check(ssx3_ee::s01::f_0014bdb8, bolts_get_db, 32, &|c, seed| {
            let mut r = Rng(0x14bd ^ seed as u64);
            scramble(c, &mut r);
        });
    }

    #[test]
    fn be_reward_int_get_track_peak_matches() {
        check_lohi(ssx3_ee::s01::f_0015a398, be_reward_int_get_track_peak, 200, &|c, seed| {
            let mut r = Rng(0x15a3 ^ seed as u64);
            scramble(c, &mut r);
            let index = (r.next_u32() % 8) as u32;
            c.g[A1][0] = index as u64;
            c.bus.write32(TRACK_RECORDS.wrapping_sub(0x26b0).wrapping_add(index.wrapping_mul(100)).wrapping_add(0x54), r.next_u32());
        });
    }

    #[test]
    fn be_reward_int_get_rewards_file_matches() {
        check(ssx3_ee::s01::f_0015a478, be_reward_int_get_rewards_file, 32, &|c, seed| {
            let mut r = Rng(0x15a4 ^ seed as u64);
            scramble(c, &mut r);
        });
    }

    #[test]
    fn be_reward_int_get_new_reward_count_matches() {
        check_lohi(ssx3_ee::s01::f_0015a750, be_reward_int_get_new_reward_count, 200, &|c, seed| {
            let mut r = Rng(0x15a7 ^ seed as u64);
            scramble(c, &mut r);
            let a1 = (r.next_u32() % 8) as u32;
            let a2 = (r.next_u32() % 8) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = a1 as u64;
            c.g[A2][0] = a2 as u64;
            for k in 0..0x40 {
                c.bus.write8(DATA + k, r.next_u32() as u8);
            }
        });
    }

    #[test]
    fn be_reward_int_get_new_reward_bits_matches() {
        check(ssx3_ee::s01::f_0015a768, be_reward_int_get_new_reward_bits, 200, &|c, seed| {
            let mut r = Rng(0x15a8 ^ seed as u64);
            scramble(c, &mut r);
            let a1 = (r.next_u32() % 8) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = a1 as u64;
            for k in 0..0x10 {
                c.bus.write32(DATA + 0x170 + k * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn rewards_file_ctor_matches() {
        check(ssx3_ee::s01::f_0015a778, rewards_file_ctor, 100, &|c, seed| {
            let mut r = Rng(0x15a9 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn algorithm_list_ctor_matches() {
        check(ssx3_ee::s01::f_0015c930, algorithm_list_ctor, 100, &|c, seed| {
            let mut r = Rng(0x15c9 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn chase_cam_controller_init_state_matches() {
        check(ssx3_ee::s01::f_00161950, chase_cam_controller_init_state, 200, &|c, seed| {
            let mut r = Rng(0x1619 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 8) as u64;
            c.bus.write32(DATA + 0x30, r.next_u32());
            for k in 0..8u32 {
                c.bus.write32(GAME_GP.wrapping_add(0x1ce0 + k * 4), r.next_u32());
            }
        });
    }

    #[test]
    fn chase_camera_algorithm_default_heading_p_matches() {
        check(ssx3_ee::s02::f_00167d88, chase_camera_algorithm_default_heading_p, 100, &|c, seed| {
            let mut r = Rng(0x167d ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x6788), r.edge_f32());
        });
    }

    #[test]
    fn chase_camera_algorithm_default_heading_i_matches() {
        check(ssx3_ee::s02::f_00167d98, chase_camera_algorithm_default_heading_i, 100, &|c, seed| {
            let mut r = Rng(0x167e ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x6784), r.edge_f32());
        });
    }

    #[test]
    fn chase_camera_algorithm_default_heading_d_matches() {
        check(ssx3_ee::s02::f_00167da8, chase_camera_algorithm_default_heading_d, 100, &|c, seed| {
            let mut r = Rng(0x167f ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x6780), r.edge_f32());
        });
    }

    #[test]
    fn math_wrap_angle_clamp_half_pi_matches() {
        check(ssx3_ee::s02::f_0016bd30, math_wrap_angle_clamp_half_pi, 300, &|c, seed| {
            let mut r = Rng(0x16bd ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x6470), r.edge_f32());
            c.bus.write32(GAME_GP.wrapping_sub(0x646c), r.edge_f32());
            c.bus.write32(GAME_GP.wrapping_sub(0x6468), r.edge_f32());
            c.bus.write32(GAME_GP.wrapping_sub(0x6464), r.edge_f32());
            c.bus.write32(DATA, r.edge_f32());
        });
    }

    #[test]
    fn camera_trigger_man_add_trigger_matches() {
        check(ssx3_ee::s02::f_0016c120, camera_trigger_man_add_trigger, 300, &|c, seed| {
            let mut r = Rng(0x16c1 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = match seed % 3 {
                0 => 0xffff_ffff_ffff_ffff,
                1 => (r.next_u32() % 30) as u64,
                _ => (r.next_u32() % 120) as u64,
            };
            c.bus.write32(DATA, DATA + 0x400);
            c.bus.write32(DATA + 4, if seed % 5 == 0 { 200 } else { r.next_u32() % 100 });
        });
    }

    #[test]
    fn active_camera_trigger_list_ctor_matches() {
        check(ssx3_ee::s02::f_0016c5d0, active_camera_trigger_list_ctor, 64, &|c, seed| {
            let mut r = Rng(0x16c5 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn camera_trigger_read_action_type3_matches() {
        check(ssx3_ee::s02::f_00171680, camera_trigger_read_action_type3, 64, &|c, seed| {
            let mut r = Rng(0x1716 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = (DATA + 0x200) as u64;
        });
    }

    #[test]
    fn vec3_max_component_matches() {
        check(ssx3_ee::s02::f_001731c0, vec3_max_component, 300, &|c, seed| {
            let mut r = Rng(0x1731 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for k in 0..3 {
                c.bus.write32(DATA + k * 4, r.edge_f32());
            }
        });
    }

    #[test]
    fn debug_menu_init_background_list_matches() {
        check(ssx3_ee::s02::f_0017abc8, debug_menu_init_background_list, 64, &|c, seed| {
            let mut r = Rng(0x17ab ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn debug_menu_select_background_matches() {
        check(ssx3_ee::s02::f_0017ac98, debug_menu_select_background, 300, &|c, seed| {
            let mut r = Rng(0x17ac ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = match seed % 3 {
                0 => 0xffff_ffff_ffff_ffff,
                _ => (r.next_u32() % 30) as u64,
            };
            c.bus.write32(DATA + 4, r.next_u32() % 30);
        });
    }

    #[test]
    fn bolt_file_index_manager_get_prev_node_matches() {
        check(ssx3_ee::s03::f_0019c8d0, bolt_file_index_manager_get_prev_node, 200, &|c, seed| {
            let mut r = Rng(0x19c8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let node = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
            c.g[A1][0] = node as u64;
            if node != 0 {
                let val = if seed % 2 == 0 { r.next_u32() & 0x7fff } else { 0x8000 | (r.next_u32() & 0x7fff) };
                c.bus.write16(node + 0xc, val as u16);
            }
        });
    }

    #[test]
    fn bolt_file_index_manager_get_next_node_matches() {
        check(ssx3_ee::s03::f_0019c8f8, bolt_file_index_manager_get_next_node, 200, &|c, seed| {
            let mut r = Rng(0x19c9 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let node = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
            c.g[A1][0] = node as u64;
            if node != 0 {
                let val = if seed % 2 == 0 { r.next_u32() & 0x7fff } else { 0x8000 | (r.next_u32() & 0x7fff) };
                c.bus.write16(node + 0xe, val as u16);
            }
        });
    }

    #[test]
    fn bolt_file_index_manager_node_index_matches() {
        check(ssx3_ee::s03::f_0019c920, bolt_file_index_manager_node_index, 200, &|c, seed| {
            let mut r = Rng(0x19ca ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { (DATA + (r.next_u32() % 0x100) * 16) as u64 };
        });
    }

    #[test]
    fn buddy_man_map_presence_state_matches() {
        check(ssx3_ee::s07::f_002628c0, buddy_man_map_presence_state, 300, &|c, seed| {
            let mut r = Rng(0x2628 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = match seed % 7 {
                0 => 0xffff_ffff_ffff_ffff,
                n => (n - 1) as u64,
            };
            c.bus.write32(DATA + 0xc8, if seed % 4 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn world_state_stream_reset_matches() {
        check(ssx3_ee::s07::f_0026d628, world_state_stream_reset, 64, &|c, seed| {
            let mut r = Rng(0x26d6 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn snd_bank_unlink_matches() {
        check(ssx3_ee::s07::f_0028b730, snd_bank_unlink, 300, &|c, seed| {
            let mut r = Rng(0x28b7 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let (next, prev) = match seed % 3 {
                0 => (0u32, r.next_u32()),
                1 => {
                    let v = DATA + 0x400;
                    (v, v)
                }
                _ => (DATA + 0x400, DATA + 0x500),
            };
            c.bus.write32(DATA + 0x18, next);
            c.bus.write32(DATA + 0x1c, prev);
            c.bus.write32(DATA + 0x418, r.next_u32());
            c.bus.write32(DATA + 0x41c, r.next_u32());
            c.bus.write32(DATA + 0x518, r.next_u32());
        });
    }

    #[test]
    fn bank_monitor_timer_enable_matches() {
        check(ssx3_ee::s08::f_002ad980, bank_monitor_timer_enable, 64, &|c, seed| {
            let mut r = Rng(0x2ad9 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn path_plat_mark_track_dirty_matches() {
        check(ssx3_ee::s08::f_002aec28, path_plat_mark_track_dirty, 300, &|c, seed| {
            let mut r = Rng(0x2aec ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 8) as u32;
            c.g[A0][0] = index as u64;
            let plat = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
            c.bus.write32(PLATFORM_TABLE + index * 4, plat);
            c.bus.write8(DATA + 0x465, 0);
        });
    }

    #[test]
    fn path_plat_get_track_aux_send_matches() {
        check(ssx3_ee::s08::f_002af4b0, path_plat_get_track_aux_send, 300, &|c, seed| {
            let mut r = Rng(0x2af4 ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 8) as u32;
            c.g[A0][0] = index as u64;
            let plat = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
            c.bus.write32(PLATFORM_TABLE + index * 4, plat);
            if plat != 0 {
                c.bus.write32(plat + 0x10, r.next_u32());
            }
        });
    }

    #[test]
    fn speech_bank_ctor_matches() {
        check(ssx3_ee::s08::f_002af8e0, speech_bank_ctor, 64, &|c, seed| {
            let mut r = Rng(0x2af8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn speech_system_get_instance_stream_slot_matches() {
        check(ssx3_ee::s08::f_002b1220, speech_system_get_instance_stream_slot, 200, &|c, seed| {
            let mut r = Rng(0x2b12 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let slot = (r.next_u32() % 8) as u32;
            c.g[A1][0] = slot as u64;
            let stream = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
            c.bus.write32(DATA + 4, DATA + 0x200);
            c.bus.write32(DATA + 0x200 + slot * 4, stream);
            if stream != 0 {
                c.bus.write32(stream + 4, r.next_u32());
            }
        });
    }

    #[test]
    fn song_instance_apply_delay_params_matches() {
        check(ssx3_ee::s08::f_002b2850, song_instance_apply_delay_params, 100, &|c, seed| {
            let mut r = Rng(0x2b28 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA + 0x100, DATA + 0x1000);
            c.bus.write32(DATA, DATA + 0x2000);
            c.bus.write32(DATA + 0x1000, DATA + 0x3000);
            c.bus.write32(DATA + 0x2078, r.next_u32());
            c.bus.write32(DATA + 0x31f4, 0);
            c.bus.write32(DATA + 0x3248, 0);
            c.bus.write32(DATA + 0x207c, r.next_u32());
            c.bus.write32(DATA + 0x2080, r.next_u32() & 0x3ff);
        });
    }

    #[test]
    fn song_instance_get_sed_value_matches() {
        check(ssx3_ee::s08::f_002b48b8, song_instance_get_sed_value, 100, &|c, seed| {
            let mut r = Rng(0x2b48 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, DATA + 0x400);
            c.bus.write32(DATA + 0x498, r.next_u32());
        });
    }

    #[test]
    fn music_player_get_selected_song_sed_value_matches() {
        check(ssx3_ee::s08::f_002b4908, music_player_get_selected_song_sed_value, 100, &|c, seed| {
            let mut r = Rng(0x2b49 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let idx = (r.next_u32() % 8) as u32;
            c.bus.write32(DATA + 0x3f0, idx);
            c.bus.write32(DATA + idx * 4 + 4, DATA + 0x400);
            c.bus.write32(DATA + 0x498, r.next_u32());
        });
    }

    #[test]
    fn music_player_is_current_song_enabled_matches() {
        check(ssx3_ee::s08::f_002b4af0, music_player_is_current_song_enabled, 200, &|c, seed| {
            let mut r = Rng(0x2b4a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x3f0, if seed % 2 == 0 { r.next_u32() % 64 } else { r.next_u32() });
            c.bus.write32(DATA + 0x3f8, r.next_u32());
            c.bus.write32(DATA + 0x3fc, r.next_u32());
        });
    }

    #[test]
    fn world_trigger_instance_ctor_matches() {
        check(ssx3_ee::s08::f_002b4b70, world_trigger_instance_ctor, 200, &|c, seed| {
            let mut r = Rng(0x2b4b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write8(0x004a_35a8, r.next_u32() as u8);
        });
    }
}
