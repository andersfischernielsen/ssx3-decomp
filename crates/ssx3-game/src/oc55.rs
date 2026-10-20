//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for fifty leaf getters/setters and small helpers, each with a
//! differential test against the generated body.
//!
//! Groups:
//! - **Save-image sizes** (`BELibrary_GetSaveDataSize`,
//!   `BESaveInt_GetProfileSaveSize`): the fixed sizes of the serialized save
//!   blobs.
//! - **Script engine singleton** (`ScriptEngine_GetInstance`) and
//!   **`ssx_ClearTrayOpened`**, both one-word globals.
//! - **Audio track-id group predicates** (`SSXAudioSystem_IsTrackIdGroup0..5`):
//!   the id ranges that name a music track group (0..4, 5..7, 8..10, 11..13,
//!   14..16 and 17..21).
//! - **`InputParser_BuiltinOperand`**: the built-in operand block 0x40 bytes
//!   into the parser.
//! - **`CameraAlgorithm_DefaultFov`**: the 45° (π/4) default from `.rodata`.
//! - **`Vu0_ReadVI1`**: `cfc2` of VU0's VI1.
//! - **`RiderMetrix_Clear/UpdatePeakA4`**: the "peak air" metric at +0xa4
//!   (cleared to −1, then raised by `max`).
//! - **Keyboard/frame/texture setters** (`KeyboardScreen_ToggleCaps`,
//!   `VD_TextureSink_SetFrameSize`).
//! - **Audio-system and bank one-word accessors** (`SSXAudioSystem_*`,
//!   `SpeechBank_HasId`, `MusicPlayer_*`, `CrowdAudio_*`).
//! - **`WorldPainter_SetScriptSurfaceMode`** and the shared-login finish flag.
//! - **`FEModule_` pad-mask byte array** (sign-extended slot index).
//! - **Table getters**: `MCManager_GetSlotFreeSpace` (0xe0-byte slots),
//!   `Replay_GetMaxHighlights`, `World_GetSphereTreeAt` (0x140-byte spheres),
//!   `BENewRaceInt_*` (`D_0043D950` / `D_0043E250`),
//!   `BENewPlayerInt_*` (`D_00534FE0`, 0x1c-byte slots) and `BEBAGTInt_*`
//!   (`D_0045AEB8`, 0x14-byte trick entries).
//! - **`Emitter_ResetDescAccel`**, **`UIMenuFlags_ctor`**, **`Flags64_TestSecond`**
//!   and **`BezierManBase_ctor`**.
//!
//! Every function reproduces the generated body's cycles, the registers it
//! leaves behind (`v0`/`v1` included) and its memory traffic; the table
//! getters that use `mult` also keep LO/HI, so their tests use the
//! `difftest::check_lohi` comparison.

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_e048, be_library_get_save_data_size, "BELibrary_GetSaveDataSize"),
    (0x0015_2ba8, be_save_int_get_profile_save_size, "BESaveInt_GetProfileSaveSize"),
    (0x0027_d2e8, script_engine_get_instance, "ScriptEngine_GetInstance"),
    (0x002a_40e0, ssx_audio_system_is_track_id_group0, "SSXAudioSystem_IsTrackIdGroup0"),
    (0x0031_ad18, ssx_clear_tray_opened, "ssx_ClearTrayOpened"),
    (0x0032_1ad0, input_parser_builtin_operand, "InputParser_BuiltinOperand"),
    (0x0016_7e30, camera_algorithm_default_fov, "CameraAlgorithm_DefaultFov"),
    (0x002a_3fd8, ssx_audio_system_is_track_id_group5, "SSXAudioSystem_IsTrackIdGroup5"),
    (0x002a_4030, ssx_audio_system_is_track_id_group4, "SSXAudioSystem_IsTrackIdGroup4"),
    (0x002a_4158, ssx_audio_system_is_track_id_group1, "SSXAudioSystem_IsTrackIdGroup1"),
    (0x002a_41c8, ssx_audio_system_is_track_id_group2, "SSXAudioSystem_IsTrackIdGroup2"),
    (0x002a_4238, ssx_audio_system_is_track_id_group3, "SSXAudioSystem_IsTrackIdGroup3"),
    (0x0039_14f8, vu0_read_vi1, "Vu0_ReadVI1"),
    (0x0011_76f8, rider_metrix_clear_peak_a4, "RiderMetrix_ClearPeakA4"),
    (0x0011_7708, rider_metrix_update_peak_a4, "RiderMetrix_UpdatePeakA4"),
    (0x0018_07c8, keyboard_screen_toggle_caps, "KeyboardScreen_ToggleCaps"),
    (0x0025_43f0, vd_texture_sink_set_frame_size, "VD_TextureSink_SetFrameSize"),
    (0x0028_2ef0, script_obj_cam_has_binding, "ScriptObjCam_HasBinding"),
    (0x002a_fa58, speech_bank_has_id, "SpeechBank_HasId"),
    (0x002e_f368, world_painter_set_script_surface_mode, "WorldPainter_SetScriptSurfaceMode"),
    (0x001a_1cb8, fe_module_set_player_pad_mask, "FEModule_SetPlayerPadMask"),
    (0x001a_1cd0, fe_module_get_player_pad_mask, "FEModule_GetPlayerPadMask"),
    (0x0024_19d8, mc_manager_get_slot_free_space, "MCManager_GetSlotFreeSpace"),
    (0x0026_5f00, c_shared_login_man_do_finish, "cSharedLoginMan_DoFinish"),
    (0x0028_5be0, ssx_audio_system_set_mode, "SSXAudioSystem_SetMode"),
    (0x0028_def0, ssx_audio_system_set_music_state, "SSXAudioSystem_SetMusicState"),
    (0x0029_e578, ssx_audio_system_reset_wipeout_sfx, "SSXAudioSystem_ResetWipeoutSfx"),
    (0x002b_3cc0, music_player_set_song_field54, "MusicPlayer_SetSongField54"),
    (0x002b_4708, music_player_clear_loops_active, "MusicPlayer_ClearLoopsActive"),
    (0x002b_4a08, music_player_set_song_field5c, "MusicPlayer_SetSongField5c"),
    (0x0036_d008, emitter_reset_desc_accel, "Emitter_ResetDescAccel"),
    (0x003a_5338, ui_menu_flags_ctor, "UIMenuFlags_ctor"),
    (0x003a_6ce8, world_get_sphere_tree_at, "World_GetSphereTreeAt"),
    (0x0012_5958, rider_set_boost_enabled, "Rider_SetBoostEnabled"),
    (0x0014_46b8, flags64_test_second, "Flags64_TestSecond"),
    (0x0014_4d38, benewraceint_get_section_entry, "BENewRaceInt_GetSectionEntry"),
    (0x0027_0c40, replay_get_max_highlights, "Replay_GetMaxHighlights"),
    (0x002a_67f0, crowd_audio_instance_start_anticipate, "CrowdAudio_InstanceStartAnticipate"),
    (0x002b_46d0, music_player_set_song_field50, "MusicPlayer_SetSongField50"),
    (0x002c_53c8, cps2_mem_card_get_port_changed_flag, "cPS2MemCard_GetPortChangedFlag"),
    (0x0037_4c90, bezier_man_base_ctor, "BezierManBase_ctor"),
    (0x0014_4cc0, benewraceint_is_station_track_by_index, "BENewRaceInt_IsStationTrackByIndex"),
    (0x0014_4d18, benewraceint_get_track_field60, "BENewRaceInt_GetTrackField60"),
    (0x0014_4d50, benewraceint_get_section_field10, "BENewRaceInt_GetSectionField10"),
    (0x0014_74a8, benewplayerint_set_alt_character_by_slot, "BENewPlayerInt_SetAltCharacterBySlot"),
    (0x0014_74c8, benewplayerint_get_alt_character_by_slot, "BENewPlayerInt_GetAltCharacterBySlot"),
    (0x0015_00d8, bebagtint_get_trick_field0, "BEBAGTInt_GetTrickField0"),
    (0x0015_00f8, bebagtint_get_trick_field2, "BEBAGTInt_GetTrickField2"),
    (0x0015_0118, bebagtint_get_trick_field4, "BEBAGTInt_GetTrickField4"),
    (0x0015_0138, bebagtint_get_trick_field6, "BEBAGTInt_GetTrickField6"),
];

// ---------------------------------------------------------------------------
// Fixed addresses (the original materialises these with `lui`/`addiu` pairs).

/// `D_0043D950`: the BE new-race station table (0x64-byte entries).
const SECTION_TABLE: u32 = 0x0043_d950;
/// `D_0043E250`: the BE new-race track table (0x18-byte entries; the getters
/// read entry + 0x14).
const TRACK_TABLE: u32 = 0x0043_e250;
/// `D_0043E254`: the BE new-race section-entry table (0x18-byte entries). The
/// original materialises it as `0x440000 - 0x1dac`.
const SECTION_ENTRY_TABLE: u32 = 0x0043_e254;
/// `D_00534FE0`: the BE player table (0x1c-byte slots; the alternate
/// character byte is at +0x12).
const PLAYER_TABLE: u32 = 0x0053_4fe0;
/// `D_0045AEB8`: the BE BAGT trick table (0x14-byte entries).
const TRICK_TABLE: u32 = 0x0045_aeb8;
/// `D_00493820`: the vtable `BezierManBase`'s members point at.
const BEZIER_MAN_VTABLE: u32 = 0x0049_3820;
/// `D_004FF120`: the constant vector (0, 0, 0, 1) an emitter's acceleration is
/// reset to.
const ZERO_ACCEL: u32 = 0x004f_f120;

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Save-image sizes and singletons.

/// `int BELibrary_GetSaveDataSize(BELibrary* be)` (0x14e048): the size in
/// bytes of one serialized profile save.
pub fn be_library_get_save_data_size(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xbb78;
    abi::ret(c);
}

/// `int BESaveInt_GetProfileSaveSize(void)` (0x152ba8): the size in bytes of
/// the "save int" (profile settings) blob.
pub fn be_save_int_get_profile_save_size(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x9b61;
    abi::ret(c);
}

/// `ScriptEngine* ScriptEngine_GetInstance(void)` (0x27d2e8): the singleton
/// pointer at `D_004A34DC`.
pub fn script_engine_get_instance(c: &mut Cpu) {
    c.cycles += 2;
    let slot = gp(c, 0x3ec);
    c.g[V0][0] = c.lw(slot);
    abi::ret(c);
}

/// `void ssx_ClearTrayOpened(void)` (0x31ad18): clears the disc-tray-opened
/// latch at `D_004A3ED0`.
pub fn ssx_clear_tray_opened(c: &mut Cpu) {
    c.cycles += 2;
    let slot = gp(c, 0xde0);
    c.sw(slot, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Music track-id group predicates. Each returns whether the id lies in one
// contiguous run, and leaves `v0` as the sign-extended `id - first` too.

/// `bool SSXAudioSystem_IsTrackIdGroup0(int self, int id)` (0x2a40e0): ids
/// 0..4.
pub fn ssx_audio_system_is_track_id_group0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = (c.g[A1][0] < 0x5) as u64;
    abi::ret(c);
}

/// `bool SSXAudioSystem_IsTrackIdGroup5(int self, int id)` (0x2a3fd8): ids
/// 0x11..0x15.
pub fn ssx_audio_system_is_track_id_group5(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(0x11));
    c.g[V0][0] = (c.g[V0][0] < 0x5) as u64;
    abi::ret(c);
}

/// `bool SSXAudioSystem_IsTrackIdGroup4(int self, int id)` (0x2a4030): ids
/// 0xe..0x10.
pub fn ssx_audio_system_is_track_id_group4(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(0xe));
    c.g[V0][0] = (c.g[V0][0] < 0x3) as u64;
    abi::ret(c);
}

/// `bool SSXAudioSystem_IsTrackIdGroup1(int self, int id)` (0x2a4158): ids
/// 5..7.
pub fn ssx_audio_system_is_track_id_group1(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(5));
    c.g[V0][0] = (c.g[V0][0] < 0x3) as u64;
    abi::ret(c);
}

/// `bool SSXAudioSystem_IsTrackIdGroup2(int self, int id)` (0x2a41c8): ids
/// 8..0xa.
pub fn ssx_audio_system_is_track_id_group2(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(8));
    c.g[V0][0] = (c.g[V0][0] < 0x3) as u64;
    abi::ret(c);
}

/// `bool SSXAudioSystem_IsTrackIdGroup3(int self, int id)` (0x2a4238): ids
/// 0xb..0xd.
pub fn ssx_audio_system_is_track_id_group3(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(0xb));
    c.g[V0][0] = (c.g[V0][0] < 0x3) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Misc small accessors.

/// `char* InputParser_BuiltinOperand(InputParser* p, int index)` (0x321ad0):
/// the built-in operand block for a pad, 0x40 bytes into the parser.
pub fn input_parser_builtin_operand(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_add(0x40));
    abi::ret(c);
}

/// `float CameraAlgorithm_DefaultFov(void)` (0x167e30): the default field of
/// view, π/4 radians, from the `.rodata` literal at `gp-0x677c`.
pub fn camera_algorithm_default_fov(c: &mut Cpu) {
    c.cycles += 3;
    let literal = gp(c, -0x677c);
    c.lwc1(0, literal);
    abi::ret(c);
}

/// `int Vu0_ReadVI1(void)` (0x3914f8): VU0's VI1 register (`cfc2 $vi1`).
pub fn vu0_read_vi1(c: &mut Cpu) {
    c.cycles += 3;
    c.cfc2(V0, 1);
    abi::ret(c);
}

/// `void RiderMetrix_ClearPeakA4(RiderMetrix* m)` (0x1176f8): resets the peak
/// air metric at +0xa4 to −1 (an unset marker) and leaves −1.0f in f0.
pub fn rider_metrix_clear_peak_a4(c: &mut Cpu) {
    c.cycles += 4;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 0); // f0 = -1.0f
    let selfp = abi::arg(c, 0);
    c.swc1(0, selfp.wrapping_add(0xa4));
    abi::ret(c);
}

/// `void RiderMetrix_UpdatePeakA4(RiderMetrix* m, float seconds)` (0x117708):
/// raises the peak air metric at +0xa4 to the new value.
pub fn rider_metrix_update_peak_a4(c: &mut Cpu) {
    c.cycles += 4;
    let selfp = abi::arg(c, 0);
    let peak = selfp.wrapping_add(0xa4);
    c.lwc1(0, peak);
    c.max_s(0, 0, 12); // max(current, seconds)
    c.swc1(0, peak);
    abi::ret(c);
}

/// `int KeyboardScreen_ToggleCaps(KeyboardScreen* s)` (0x1807c8): flips the
/// caps-lock bit of the screen's flag word at +0xe4 and returns the new word.
pub fn keyboard_screen_toggle_caps(c: &mut Cpu) {
    c.cycles += 4;
    let flags = Addr(abi::arg(c, 0)).off(0xe4);
    let v = flags.lw(c) ^ 1;
    c.g[V0][0] = v;
    flags.sw(c, v);
    abi::ret(c);
}

/// `void VD_TextureSink_SetFrameSize(VD_TextureSink* s, float a, float b,
/// float c)` (0x2543f0): the three frame dimensions at +0x40..0x48.
pub fn vd_texture_sink_set_frame_size(c: &mut Cpu) {
    c.cycles += 4;
    let selfp = abi::arg(c, 0);
    c.swc1(14, selfp.wrapping_add(0x48));
    c.swc1(12, selfp.wrapping_add(0x40));
    c.swc1(13, selfp.wrapping_add(0x44));
    abi::ret(c);
}

/// `bool ScriptObjCam_HasBinding(ScriptObjCam* cam, int id)` (0x282ef0): true
/// when the binding id at +0x14 equals `id`.
pub fn script_obj_cam_has_binding(c: &mut Cpu) {
    c.cycles += 4;
    let selfp = abi::arg(c, 0);
    let id = c.g[A1][0];
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x14)) ^ id;
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `bool SpeechBank_HasId(SpeechBank* b, int id)` (0x2afa58): true when the
/// bank's own id (its first word) equals `id`.
pub fn speech_bank_has_id(c: &mut Cpu) {
    c.cycles += 4;
    let selfp = abi::arg(c, 0);
    let id = c.g[A1][0];
    c.g[V1][0] = c.lw(selfp);
    c.g[V0][0] = id ^ c.g[V1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `void WorldPainter_SetScriptSurfaceMode(WorldPainter* p)` (0x2ef368):
/// remembers the painter (`D_004A3B5C`) and whether it is non-null
/// (`D_004A4444`, as the 0/1 `v0`).
pub fn world_painter_set_script_surface_mode(c: &mut Cpu) {
    c.cycles += 4;
    let painter = c.g[A0][0];
    c.g[V0][0] = (0 < painter) as u64;
    c.sw(gp(c, 0xa6c), painter);
    c.sw(gp(c, 0x1354), c.g[V0][0]);
    abi::ret(c);
}

/// `void FEModule_SetPlayerPadMask(FEModule* m, int slot, u8 mask)` (0x1a1cb8):
/// the per-player pad mask bytes at +8, indexed by the sign-extended `slot`.
pub fn fe_module_set_player_pad_mask(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    let slot = abi::arg(c, 1) as u8 as i8 as i32;
    c.g[A1][0] = sx(slot as u32);
    c.g[A0][0] = sx(selfp.wrapping_add(slot as u32));
    let mask = c.g[A2][0];
    Addr(c.g[A0][0] as u32).off(8).sb(c, mask);
    abi::ret(c);
}

/// `u8 FEModule_GetPlayerPadMask(FEModule* m, int slot)` (0x1a1cd0): the
/// per-player pad mask byte at +8.
pub fn fe_module_get_player_pad_mask(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    let slot = abi::arg(c, 1) as u8 as i8 as i32;
    c.g[A1][0] = sx(slot as u32);
    c.g[A0][0] = sx(selfp.wrapping_add(slot as u32));
    let addr = (c.g[A0][0] as u32).wrapping_add(8);
    c.g[V0][0] = c.lbu(addr);
    abi::ret(c);
}

/// `int MCManager_GetSlotFreeSpace(MCManager* m, int slot)` (0x2419d8): the
/// free-space word of a memory-card slot (0xe0-byte slots, field at +0x354).
pub fn mc_manager_get_slot_free_space(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0xe0;
    c.mult(0, V1, A1, V0); // v1 = slot * 0xe0
    let selfp = c.g[A0][0] as u32;
    c.g[A0][0] = sx(selfp.wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x354));
    abi::ret(c);
}

/// `void cSharedLoginMan_DoFinish(cSharedLoginMan* m)` (0x265f00): marks the
/// login done (state 0x13 at +0), clears the pending flag at +4 and the
/// shared-login pointer `D_004A33E0`.
pub fn c_shared_login_man_do_finish(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    c.sw(selfp.wrapping_add(4), 0);
    c.g[V0][0] = 0x13;
    c.sw(gp(c, 0x2f0), 0);
    c.sw(selfp, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Audio system / music player / bank accessors.

/// `void SSXAudioSystem_SetMode(SSXAudioSystem* a, int mode)` (0x285be0): the
/// system mode at +0x6474, stored only when it changed.
pub fn ssx_audio_system_set_mode(c: &mut Cpu) {
    c.cycles += 3;
    let selfp = abi::arg(c, 0);
    let mode = c.g[A1][0];
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x6474));
    if c.g[V0][0] != mode {
        c.sw(selfp.wrapping_add(0x6474), mode);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool SSXAudioSystem_SetMusicState(SSXAudioSystem* a, int state)`
/// (0x28def0): stores the new music state at +0x629c and returns whether it
/// differed from the old one.
pub fn ssx_audio_system_set_music_state(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    let state = c.g[A1][0];
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x629c));
    c.sw(selfp.wrapping_add(0x629c), state);
    c.g[V0][0] ^= state;
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `void SSXAudioSystem_ResetWipeoutSfx(SSXAudioSystem* a, Wipeout* w)`
/// (0x29e578): clears the wipe-out active word of the voice indexed by
/// `w->+0x870` (4-byte entries at +0x6080).
pub fn ssx_audio_system_reset_wipeout_sfx(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    c.g[V0][0] = c.lw(src.wrapping_add(0x870));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[A0][0] = sx(selfp.wrapping_add(c.g[V0][0] as u32));
    let dst = (c.g[A0][0] as u32).wrapping_add(0x6080);
    c.sw(dst, 0);
    abi::ret(c);
}

/// `void MusicPlayer_SetSongField54(MusicPlayer* p, int v)` (0x2b3cc0): stores
/// `v` at +0x54 of the current song (`+0x408`), if there is one.
pub fn music_player_set_song_field54(c: &mut Cpu) {
    c.cycles += 3;
    let selfp = abi::arg(c, 0);
    let v = c.g[A1][0];
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x408));
    if c.g[V0][0] != 0 {
        let song = c.g[V0][0] as u32;
        c.sw(song.wrapping_add(0x54), v);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void MusicPlayer_ClearLoopsActive(MusicPlayer* p)` (0x2b4708): clears the
/// loops-active word at +0x4c of the current song, if there is one.
pub fn music_player_clear_loops_active(c: &mut Cpu) {
    c.cycles += 3;
    let selfp = abi::arg(c, 0);
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x408));
    if c.g[V0][0] != 0 {
        let song = c.g[V0][0] as u32;
        c.sw(song.wrapping_add(0x4c), 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void MusicPlayer_SetSongField5c(MusicPlayer* p, int v)` (0x2b4a08): stores
/// `v` at +0x5c of the current song, if there is one.
pub fn music_player_set_song_field5c(c: &mut Cpu) {
    c.cycles += 3;
    let selfp = abi::arg(c, 0);
    let v = c.g[A1][0];
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x408));
    if c.g[V0][0] != 0 {
        let song = c.g[V0][0] as u32;
        c.sw(song.wrapping_add(0x5c), v);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int MusicPlayer_SetSongField50(MusicPlayer* p)` (0x2b46d0): sets the
/// current song's +0x50 word to 1 (when there is a song) and returns 1.
pub fn music_player_set_song_field50(c: &mut Cpu) {
    c.cycles += 3;
    let selfp = abi::arg(c, 0);
    c.g[V1][0] = c.lw(selfp.wrapping_add(0x408));
    let none = c.g[V1][0] == 0;
    c.g[V0][0] = 1;
    if !none {
        c.cycles += 1;
        let song = c.g[V1][0] as u32;
        c.sw(song.wrapping_add(0x50), c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void CrowdAudio_InstanceStartAnticipate(CrowdAudio* a)` (0x2a67f0): sets
/// the "anticipating" flag at +0x180 the first time it is called and returns
/// 1.
pub fn crowd_audio_instance_start_anticipate(c: &mut Cpu) {
    c.cycles += 3;
    let selfp = abi::arg(c, 0);
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x180));
    let already = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if !already {
        c.cycles += 1;
        c.sw(selfp.wrapping_add(0x180), c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Render, UI and world accessors.

/// `void Emitter_ResetDescAccel(Emitter* e)` (0x36d008): copies the constant
/// (0, 0, 0, 1) vector `D_004FF120` into the descriptor's acceleration at
/// +0xe0.
pub fn emitter_reset_desc_accel(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0x50_0000;
    c.g[V0][0] = sx(ZERO_ACCEL);
    let src = c.g[V0][0] as u32;
    c.lq(3, src);
    let selfp = abi::arg(c, 0);
    c.sq(3, selfp.wrapping_add(0xe0));
    abi::ret(c);
}

/// `UIMenuFlags* UIMenuFlags_ctor(UIMenuFlags* f)` (0x3a5338): clears the low
/// nibble of the flag byte and returns the object.
pub fn ui_menu_flags_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    c.g[V1][0] = c.lbu(selfp);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] &= 0xf0;
    c.sb(selfp, c.g[V1][0]);
    abi::ret(c);
}

/// `SphereTree* World_GetSphereTreeAt(World* w, int index)` (0x3a6ce8): the
/// index-th sphere tree (0x140-byte records) of the world's tree array.
pub fn world_get_sphere_tree_at(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    c.g[V1][0] = 0x140;
    c.g[V0][0] = c.lw(selfp);
    c.mult(0, A1, A1, V1); // a1 = index * 0x140
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}

/// `void Rider_SetBoostEnabled(Rider* r, Boost* boost)` (0x125958): stores the
/// boost pointer at +0xb2c; clearing it also zeroes the two boost state words
/// at +0x2f0/+0x2f4.
pub fn rider_set_boost_enabled(c: &mut Cpu) {
    c.cycles += 2;
    let r = abi::arg(c, 0);
    let boost = c.g[A1][0];
    c.sw(r.wrapping_add(0xb2c), boost);
    if boost == 0 {
        c.cycles += 2;
        c.sw(r.wrapping_add(0x2f0), 0);
        c.sw(r.wrapping_add(0x2f4), 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool Flags64_TestSecond(u64* flags, int bit)` (0x1446b8): bit `bit` of the
/// 64-bit flag word at +8, non-zero as a 0/1 (and `v1` keeps the mask).
pub fn flags64_test_second(c: &mut Cpu) {
    c.cycles += 6;
    let selfp = abi::arg(c, 0);
    c.g[V0][0] = c.ld(selfp.wrapping_add(8));
    c.g[V1][0] = 0x1;
    c.g[V1][0] = c.g[V1][0] << (c.g[A1][0] & 63);
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `int Replay_GetMaxHighlights(Replay* r, int unused)` (0x270c40): the
/// highlight count at +0xc, or 1 when there is no replay.
pub fn replay_get_max_highlights(c: &mut Cpu) {
    c.cycles += 2;
    let unused = c.g[A1][0];
    c.g[V0][0] = 0x1;
    if unused == 0 {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        let selfp = abi::arg(c, 0);
        c.g[V0][0] = c.lw(selfp.wrapping_add(0xc));
        abi::ret(c);
    }
}

/// `int cPS2MemCard_GetPortChangedFlag(cPS2MemCard* m, int port)` (0x2c53c8):
/// whether a card port (0x14-byte slots, flag at +0x18c) changed.
pub fn cps2_mem_card_get_port_changed_flag(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x14;
    c.mult(0, V1, A1, V0); // v1 = port * 0x14
    let selfp = c.g[A0][0] as u32;
    c.g[A0][0] = sx(selfp.wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18c));
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `BezierManBase* BezierManBase_ctor(BezierManBase* b)` (0x374c90): installs
/// the vtable at +4, publishes the object at `D_004A4248` and returns it.
pub fn bezier_man_base_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x49_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(BEZIER_MAN_VTABLE);
    let selfp = c.g[A0][0] as u32;
    c.sw(selfp.wrapping_add(4), c.g[V1][0]);
    c.sw(gp(c, 0x1158), c.g[A0][0]);
    abi::ret(c);
}

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

/// `u16 BEBAGTInt_GetTrickField0(int unused, int unused, int index)`
/// (0x1500d8): the first halfword of a BAGT trick entry (0x14-byte records in
/// `D_0045AEB8`).
pub fn bebagtint_get_trick_field0(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    c.mult(0, A2, A2, V1); // a2 = index * 0x14
    c.g[V0][0] = sx(TRICK_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A2][0] = sx(base.wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[A2][0] as u32;
    c.g[V0][0] = c.lhu(entry);
    abi::ret(c);
}

/// `u16 BEBAGTInt_GetTrickField2(...)` (0x1500f8): halfword +2 of a trick
/// entry.
pub fn bebagtint_get_trick_field2(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    c.mult(0, A2, A2, V1);
    c.g[V0][0] = sx(TRICK_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A2][0] = sx(base.wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[A2][0] as u32;
    c.g[V0][0] = c.lhu(entry.wrapping_add(2));
    abi::ret(c);
}

/// `u16 BEBAGTInt_GetTrickField4(...)` (0x150118): halfword +4 of a trick
/// entry.
pub fn bebagtint_get_trick_field4(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    c.mult(0, A2, A2, V1);
    c.g[V0][0] = sx(TRICK_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A2][0] = sx(base.wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[A2][0] as u32;
    c.g[V0][0] = c.lhu(entry.wrapping_add(4));
    abi::ret(c);
}

/// `u16 BEBAGTInt_GetTrickField6(...)` (0x150138): halfword +6 of a trick
/// entry.
pub fn bebagtint_get_trick_field6(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x14;
    c.g[V0][0] = 0x46_0000;
    c.mult(0, A2, A2, V1);
    c.g[V0][0] = sx(TRICK_TABLE);
    let base = c.g[V0][0] as u32;
    c.g[A2][0] = sx(base.wrapping_add(c.g[A2][0] as u32));
    let entry = c.g[A2][0] as u32;
    c.g[V0][0] = c.lhu(entry.wrapping_add(6));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Differential tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, put_vec, scramble, Rng, DATA, GAME_GP};
    use ssx3_ee as ee;

    const SEEDS: u32 = 120;

    /// A per-function RNG (the address keeps the functions' seeds apart).
    fn rng(tag: u32, seed: u32) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((tag as u64) << 20) ^ seed as u64)
    }

    /// A 64-bit value whose low half is small (indices, ids) and whose upper
    /// half is junk on odd seeds, so 64-bit compares are exercised.
    fn mixed(r: &mut Rng, seed: u32, low: u32) -> u64 {
        if seed % 2 == 0 {
            low as u64
        } else {
            ((r.next_u32() as u64) << 32) | low as u64
        }
    }

    /// An id inside or just outside `[base, base + len)`, with junk in the
    /// upper half on odd seeds.
    fn track_id(c: &mut Cpu, r: &mut Rng, seed: u32, base: u32, len: u32) {
        let id = match seed % 3 {
            0 => base.wrapping_add(seed % len),
            1 => base.wrapping_sub(1 + seed % 4),
            _ => r.next_u32() % 0x40,
        };
        c.g[A1][0] = mixed(r, seed, id);
    }

    #[test]
    fn be_library_get_save_data_size_matches() {
        check(ee::s01::f_0014e048__generated, be_library_get_save_data_size, SEEDS, &|c, seed| {
            let mut r = rng(0x0014_e048, seed);
            scramble(c, &mut r);
        });
    }

    #[test]
    fn be_save_int_get_profile_save_size_matches() {
        check(ee::s01::f_00152ba8__generated, be_save_int_get_profile_save_size, SEEDS, &|c, seed| {
            let mut r = rng(0x0015_2ba8, seed);
            scramble(c, &mut r);
        });
    }

    #[test]
    fn script_engine_get_instance_matches() {
        check(ee::s07::f_0027d2e8__generated, script_engine_get_instance, SEEDS, &|c, seed| {
            let mut r = rng(0x0027_d2e8, seed);
            scramble(c, &mut r);
            c.g[GP][0] = sx(GAME_GP);
            if seed % 4 != 0 {
                c.bus.write32(0x004a_34dc, r.next_u32());
            }
        });
    }

    #[test]
    fn ssx_clear_tray_opened_matches() {
        check(ee::s10::f_0031ad18__generated, ssx_clear_tray_opened, SEEDS, &|c, seed| {
            let mut r = rng(0x0031_ad18, seed);
            scramble(c, &mut r);
            c.g[GP][0] = sx(GAME_GP);
            c.bus.write32(0x004a_3ed0, if seed % 2 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn input_parser_builtin_operand_matches() {
        check(ee::s10::f_00321ad0__generated, input_parser_builtin_operand, SEEDS, &|c, seed| {
            let mut r = rng(0x0032_1ad0, seed);
            scramble(c, &mut r);
            let v = r.next_u32();
            c.g[A1][0] = mixed(&mut r, seed, v);
        });
    }

    #[test]
    fn camera_algorithm_default_fov_matches() {
        check(ee::s02::f_00167e30__generated, camera_algorithm_default_fov, SEEDS, &|c, seed| {
            let mut r = rng(0x0016_7e30, seed);
            scramble(c, &mut r);
            c.g[GP][0] = sx(GAME_GP);
            let fov = if seed % 2 == 0 { 0x3f49_0fdb } else { r.edge_f32() };
            c.bus.write32(0x0049_c974, fov);
        });
    }

    #[test]
    fn ssx_audio_system_is_track_id_group0_matches() {
        check(ee::s08::f_002a40e0__generated, ssx_audio_system_is_track_id_group0, SEEDS, &|c, seed| {
            let mut r = rng(0x002a_40e0, seed);
            scramble(c, &mut r);
            track_id(c, &mut r, seed, 0, 5);
        });
    }

    #[test]
    fn ssx_audio_system_is_track_id_group5_matches() {
        check(ee::s08::f_002a3fd8__generated, ssx_audio_system_is_track_id_group5, SEEDS, &|c, seed| {
            let mut r = rng(0x002a_3fd8, seed);
            scramble(c, &mut r);
            track_id(c, &mut r, seed, 0x11, 5);
        });
    }

    #[test]
    fn ssx_audio_system_is_track_id_group4_matches() {
        check(ee::s08::f_002a4030__generated, ssx_audio_system_is_track_id_group4, SEEDS, &|c, seed| {
            let mut r = rng(0x002a_4030, seed);
            scramble(c, &mut r);
            track_id(c, &mut r, seed, 0xe, 3);
        });
    }

    #[test]
    fn ssx_audio_system_is_track_id_group1_matches() {
        check(ee::s08::f_002a4158__generated, ssx_audio_system_is_track_id_group1, SEEDS, &|c, seed| {
            let mut r = rng(0x002a_4158, seed);
            scramble(c, &mut r);
            track_id(c, &mut r, seed, 5, 3);
        });
    }

    #[test]
    fn ssx_audio_system_is_track_id_group2_matches() {
        check(ee::s08::f_002a41c8__generated, ssx_audio_system_is_track_id_group2, SEEDS, &|c, seed| {
            let mut r = rng(0x002a_41c8, seed);
            scramble(c, &mut r);
            track_id(c, &mut r, seed, 8, 3);
        });
    }

    #[test]
    fn ssx_audio_system_is_track_id_group3_matches() {
        check(ee::s08::f_002a4238__generated, ssx_audio_system_is_track_id_group3, SEEDS, &|c, seed| {
            let mut r = rng(0x002a_4238, seed);
            scramble(c, &mut r);
            track_id(c, &mut r, seed, 0xb, 3);
        });
    }

    #[test]
    fn vu0_read_vi1_matches() {
        check(ee::s12::f_003914f8__generated, vu0_read_vi1, SEEDS, &|c, seed| {
            let mut r = rng(0x0039_14f8, seed);
            scramble(c, &mut r);
            c.bus.plat.write_ctrl(1, r.next_u32());
        });
    }

    #[test]
    fn rider_metrix_clear_peak_a4_matches() {
        check(ee::s00::f_001176f8__generated, rider_metrix_clear_peak_a4, SEEDS, &|c, seed| {
            let mut r = rng(0x0011_76f8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA + 0xa4, r.next_u32());
        });
    }

    #[test]
    fn rider_metrix_update_peak_a4_matches() {
        check(ee::s00::f_00117708__generated, rider_metrix_update_peak_a4, SEEDS, &|c, seed| {
            let mut r = rng(0x0011_7708, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            // Half the seeds take a larger "seconds", half a smaller one.
            let (cur, new) = if seed % 2 == 0 {
                (r.f32(10.0), r.f32(100.0))
            } else {
                (r.f32(100.0), r.f32(10.0))
            };
            c.bus.write32(DATA + 0xa4, cur);
            c.f[12] = new;
        });
    }

    #[test]
    fn keyboard_screen_toggle_caps_matches() {
        check(ee::s02::f_001807c8__generated, keyboard_screen_toggle_caps, SEEDS, &|c, seed| {
            let mut r = rng(0x0018_07c8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let flags = match seed % 3 {
                0 => 0,
                1 => 1,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + 0xe4, flags);
        });
    }

    #[test]
    fn vd_texture_sink_set_frame_size_matches() {
        check(ee::s06::f_002543f0__generated, vd_texture_sink_set_frame_size, SEEDS, &|c, seed| {
            let mut r = rng(0x0025_43f0, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.f[12] = r.next_u32();
            c.f[13] = r.edge_f32();
            c.f[14] = r.next_u32();
        });
    }

    #[test]
    fn script_obj_cam_has_binding_matches() {
        check(ee::s07::f_00282ef0__generated, script_obj_cam_has_binding, SEEDS, &|c, seed| {
            let mut r = rng(0x0028_2ef0, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let id = r.next_u32();
            c.bus.write32(DATA + 0x14, id);
            c.g[A1][0] = match seed % 3 {
                0 => mixed(&mut r, seed, id),
                1 => id as u64,
                _ => r.next_u32() as u64,
            };
        });
    }

    #[test]
    fn speech_bank_has_id_matches() {
        check(ee::s08::f_002afa58__generated, speech_bank_has_id, SEEDS, &|c, seed| {
            let mut r = rng(0x002a_fa58, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let id = r.next_u32();
            c.bus.write32(DATA, id);
            c.g[A1][0] = match seed % 3 {
                0 => mixed(&mut r, seed, id),
                1 => id as u64,
                _ => r.next_u32() as u64,
            };
        });
    }

    #[test]
    fn world_painter_set_script_surface_mode_matches() {
        check(ee::s09::f_002ef368__generated, world_painter_set_script_surface_mode, SEEDS, &|c, seed| {
            let mut r = rng(0x002e_f368, seed);
            scramble(c, &mut r);
            c.g[GP][0] = sx(GAME_GP);
            c.bus.write32(0x004a_3b5c, r.next_u32());
            c.bus.write32(0x004a_4444, r.next_u32());
            c.g[A0][0] = match seed % 3 {
                0 => 0,
                1 => sx(DATA),
                _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
            };
        });
    }

    #[test]
    fn fe_module_set_player_pad_mask_matches() {
        check(ee::s03::f_001a1cb8__generated, fe_module_set_player_pad_mask, SEEDS, &|c, seed| {
            let mut r = rng(0x001a_1cb8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x100);
            let slot = (seed % 9) as i32 - 4;
            // Odd seeds pass a slot with the sign extension hidden in a wider
            // value, which the original re-sign-extends from the low byte.
            let raw = if seed % 2 == 0 { slot } else { 0x100 + slot };
            c.g[A1][0] = sx(raw as u32);
            c.g[A2][0] = match seed % 4 {
                0 => 0,
                _ => r.next_u32() as u64,
            };
        });
    }

    #[test]
    fn fe_module_get_player_pad_mask_matches() {
        check(ee::s03::f_001a1cd0__generated, fe_module_get_player_pad_mask, SEEDS, &|c, seed| {
            let mut r = rng(0x001a_1cd0, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x100);
            let slot = (seed % 9) as i32 - 4;
            c.g[A1][0] = sx(slot as u32);
            let at = (DATA + 0x100) as i32 + slot + 8;
            c.bus.write8(at as u32, r.next_u32() as u8);
        });
    }

    #[test]
    fn mc_manager_get_slot_free_space_matches() {
        check_lohi(ee::s06::f_002419d8__generated, mc_manager_get_slot_free_space, SEEDS, &|c, seed| {
            let mut r = rng(0x0024_19d8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let slot = (seed % 8) as u32;
            c.g[A1][0] = mixed(&mut r, seed, slot);
            c.bus.write32(DATA + slot * 0xe0 + 0x354, r.next_u32());
        });
    }

    #[test]
    fn c_shared_login_man_do_finish_matches() {
        check(ee::s07::f_00265f00__generated, c_shared_login_man_do_finish, SEEDS, &|c, seed| {
            let mut r = rng(0x0026_5f00, seed);
            scramble(c, &mut r);
            c.g[GP][0] = sx(GAME_GP);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
            c.bus.write32(0x004a_33e0, r.next_u32());
        });
    }

    #[test]
    fn ssx_audio_system_set_mode_matches() {
        check(ee::s07::f_00285be0__generated, ssx_audio_system_set_mode, SEEDS, &|c, seed| {
            let mut r = rng(0x0028_5be0, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let old = r.next_u32();
            c.bus.write32(DATA + 0x6474, old);
            c.g[A1][0] = match seed % 3 {
                0 => old as u64,
                1 => mixed(&mut r, seed, old),
                _ => r.next_u32() as u64,
            };
        });
    }

    #[test]
    fn ssx_audio_system_set_music_state_matches() {
        check(ee::s07::f_0028def0__generated, ssx_audio_system_set_music_state, SEEDS, &|c, seed| {
            let mut r = rng(0x0028_def0, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let old = r.next_u32();
            c.bus.write32(DATA + 0x629c, old);
            c.g[A1][0] = match seed % 3 {
                0 => old as u64,
                _ => {
                    let v = r.next_u32();
                    mixed(&mut r, seed, v)
                }
            };
        });
    }

    #[test]
    fn ssx_audio_system_reset_wipeout_sfx_matches() {
        check(ee::s08::f_0029e578__generated, ssx_audio_system_reset_wipeout_sfx, SEEDS, &|c, seed| {
            let mut r = rng(0x0029_e578, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            let voice = (seed % 8) as u32;
            c.bus.write32(DATA + 0x1000 + 0x870, voice);
            c.bus.write32(DATA + voice * 4 + 0x6080, r.next_u32());
        });
    }

    #[test]
    fn music_player_set_song_field54_matches() {
        check(ee::s08::f_002b3cc0__generated, music_player_set_song_field54, SEEDS, &|c, seed| {
            let mut r = rng(0x002b_3cc0, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            if seed % 3 == 0 {
                c.bus.write32(DATA + 0x408, 0);
            } else {
                c.bus.write32(DATA + 0x408, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x54, r.next_u32());
            }
            let v = r.next_u32();
            c.g[A1][0] = mixed(&mut r, seed, v);
        });
    }

    #[test]
    fn music_player_clear_loops_active_matches() {
        check(ee::s08::f_002b4708__generated, music_player_clear_loops_active, SEEDS, &|c, seed| {
            let mut r = rng(0x002b_4708, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            if seed % 3 == 0 {
                c.bus.write32(DATA + 0x408, 0);
            } else {
                c.bus.write32(DATA + 0x408, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x4c, r.next_u32());
            }
        });
    }

    #[test]
    fn music_player_set_song_field5c_matches() {
        check(ee::s08::f_002b4a08__generated, music_player_set_song_field5c, SEEDS, &|c, seed| {
            let mut r = rng(0x002b_4a08, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            if seed % 3 == 0 {
                c.bus.write32(DATA + 0x408, 0);
            } else {
                c.bus.write32(DATA + 0x408, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x5c, r.next_u32());
            }
            let v = r.next_u32();
            c.g[A1][0] = mixed(&mut r, seed, v);
        });
    }

    #[test]
    fn music_player_set_song_field50_matches() {
        check(ee::s08::f_002b46d0__generated, music_player_set_song_field50, SEEDS, &|c, seed| {
            let mut r = rng(0x002b_46d0, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            if seed % 3 == 0 {
                c.bus.write32(DATA + 0x408, 0);
            } else {
                c.bus.write32(DATA + 0x408, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x50, r.next_u32());
            }
        });
    }

    #[test]
    fn crowd_audio_instance_start_anticipate_matches() {
        check(ee::s08::f_002a67f0__generated, crowd_audio_instance_start_anticipate, SEEDS, &|c, seed| {
            let mut r = rng(0x002a_67f0, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA + 0x180, if seed % 3 == 0 { 0 } else { 1 });
        });
    }

    #[test]
    fn emitter_reset_desc_accel_matches() {
        check(ee::s12::f_0036d008__generated, emitter_reset_desc_accel, SEEDS, &|c, seed| {
            let mut r = rng(0x0036_d008, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            put_vec(c, 0x004f_f120, [0, 0, 0, 0x3f80_0000]);
            c.bus.write32(DATA + 0xe0, r.next_u32());
        });
    }

    #[test]
    fn ui_menu_flags_ctor_matches() {
        check(ee::s13::f_003a5338__generated, ui_menu_flags_ctor, SEEDS, &|c, seed| {
            let mut r = rng(0x003a_5338, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write8(DATA, r.next_u32() as u8);
        });
    }

    #[test]
    fn world_get_sphere_tree_at_matches() {
        check_lohi(ee::s13::f_003a6ce8__generated, world_get_sphere_tree_at, SEEDS, &|c, seed| {
            let mut r = rng(0x003a_6ce8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA, DATA + 0x2000);
            c.g[A1][0] = mixed(&mut r, seed, (seed % 8) as u32);
        });
    }

    #[test]
    fn rider_set_boost_enabled_matches() {
        check(ee::s00::f_00125958__generated, rider_set_boost_enabled, SEEDS, &|c, seed| {
            let mut r = rng(0x0012_5958, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { mixed(&mut r, seed, DATA + 0x100) };
            c.bus.write32(DATA + 0xb2c, r.next_u32());
            c.bus.write32(DATA + 0x2f0, r.next_u32());
            c.bus.write32(DATA + 0x2f4, r.next_u32());
        });
    }

    #[test]
    fn flags64_test_second_matches() {
        check(ee::s01::f_001446b8__generated, flags64_test_second, SEEDS, &|c, seed| {
            let mut r = rng(0x0014_46b8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let flags = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.bus.write64(DATA + 8, flags);
            c.g[A1][0] = sx((seed % 70) as u32);
        });
    }

    #[test]
    fn benewraceint_get_section_entry_matches() {
        check_lohi(ee::s01::f_00144d38__generated, benewraceint_get_section_entry, SEEDS, &|c, seed| {
            let mut r = rng(0x0014_4d38, seed);
            scramble(c, &mut r);
            c.g[A1][0] = mixed(&mut r, seed, (seed % 8) as u32);
        });
    }

    #[test]
    fn replay_get_max_highlights_matches() {
        check(ee::s07::f_00270c40__generated, replay_get_max_highlights, SEEDS, &|c, seed| {
            let mut r = rng(0x0027_0c40, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let v = r.next_u32();
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { mixed(&mut r, seed, v) };
            c.bus.write32(DATA + 0xc, r.next_u32());
        });
    }

    #[test]
    fn cps2_mem_card_get_port_changed_flag_matches() {
        check_lohi(ee::s08::f_002c53c8__generated, cps2_mem_card_get_port_changed_flag, SEEDS, &|c, seed| {
            let mut r = rng(0x002c_53c8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let port = (seed % 8) as u32;
            c.g[A1][0] = mixed(&mut r, seed, port);
            c.bus.write32(DATA + port * 0x14 + 0x18c, r.next_u32());
        });
    }

    #[test]
    fn bezier_man_base_ctor_matches() {
        check(ee::s12::f_00374c90__generated, bezier_man_base_ctor, SEEDS, &|c, seed| {
            let mut r = rng(0x0037_4c90, seed);
            scramble(c, &mut r);
            c.g[GP][0] = sx(GAME_GP);
            c.g[A0][0] = sx(DATA + 0x100);
            c.bus.write32(DATA + 0x104, r.next_u32());
            c.bus.write32(0x004a_4248, r.next_u32());
        });
    }

    #[test]
    fn benewraceint_is_station_track_by_index_matches() {
        check_lohi(ee::s01::f_00144cc0__generated, benewraceint_is_station_track_by_index, SEEDS, &|c, seed| {
            let mut r = rng(0x0014_4cc0, seed);
            scramble(c, &mut r);
            let index = (seed % 8) as u32;
            c.g[A1][0] = mixed(&mut r, seed, index);
            c.bus.write32(SECTION_TABLE + index * 0x64 + 0x58, r.next_u32());
        });
    }

    #[test]
    fn benewraceint_get_track_field60_matches() {
        check_lohi(ee::s01::f_00144d18__generated, benewraceint_get_track_field60, SEEDS, &|c, seed| {
            let mut r = rng(0x0014_4d18, seed);
            scramble(c, &mut r);
            let index = (seed % 8) as u32;
            c.g[A1][0] = mixed(&mut r, seed, index);
            c.bus.write32(SECTION_TABLE + index * 0x64 + 0x60, r.next_u32());
        });
    }

    #[test]
    fn benewraceint_get_section_field10_matches() {
        check_lohi(ee::s01::f_00144d50__generated, benewraceint_get_section_field10, SEEDS, &|c, seed| {
            let mut r = rng(0x0014_4d50, seed);
            scramble(c, &mut r);
            let index = (seed % 8) as u32;
            c.g[A1][0] = mixed(&mut r, seed, index);
            c.bus.write32(TRACK_TABLE + index * 0x18 + 0x14, r.next_u32());
        });
    }

    #[test]
    fn benewplayerint_set_alt_character_by_slot_matches() {
        check_lohi(ee::s01::f_001474a8__generated, benewplayerint_set_alt_character_by_slot, SEEDS, &|c, seed| {
            let mut r = rng(0x0014_74a8, seed);
            scramble(c, &mut r);
            let slot = (seed % 8) as u32;
            c.g[A1][0] = mixed(&mut r, seed, slot);
            c.g[A2][0] = match seed % 4 {
                0 => 0,
                1 => 0xff,
                _ => r.next_u32() as u64,
            };
            c.bus.write8(PLAYER_TABLE + slot * 0x1c + 0x12, r.next_u32() as u8);
        });
    }

    #[test]
    fn benewplayerint_get_alt_character_by_slot_matches() {
        check_lohi(ee::s01::f_001474c8__generated, benewplayerint_get_alt_character_by_slot, SEEDS, &|c, seed| {
            let mut r = rng(0x0014_74c8, seed);
            scramble(c, &mut r);
            let slot = (seed % 8) as u32;
            c.g[A1][0] = mixed(&mut r, seed, slot);
            c.bus.write8(PLAYER_TABLE + slot * 0x1c + 0x12, r.next_u32() as u8);
        });
    }

    #[test]
    fn bebagtint_get_trick_field0_matches() {
        check_lohi(ee::s01::f_001500d8__generated, bebagtint_get_trick_field0, SEEDS, &|c, seed| {
            let mut r = rng(0x0015_00d8, seed);
            scramble(c, &mut r);
            let index = (seed % 8) as u32;
            c.g[A2][0] = mixed(&mut r, seed, index);
            c.bus.write16(TRICK_TABLE + index * 0x14, r.next_u32() as u16);
        });
    }

    #[test]
    fn bebagtint_get_trick_field2_matches() {
        check_lohi(ee::s01::f_001500f8__generated, bebagtint_get_trick_field2, SEEDS, &|c, seed| {
            let mut r = rng(0x0015_00f8, seed);
            scramble(c, &mut r);
            let index = (seed % 8) as u32;
            c.g[A2][0] = mixed(&mut r, seed, index);
            c.bus.write16(TRICK_TABLE + index * 0x14 + 2, r.next_u32() as u16);
        });
    }

    #[test]
    fn bebagtint_get_trick_field4_matches() {
        check_lohi(ee::s01::f_00150118__generated, bebagtint_get_trick_field4, SEEDS, &|c, seed| {
            let mut r = rng(0x0015_0118, seed);
            scramble(c, &mut r);
            let index = (seed % 8) as u32;
            c.g[A2][0] = mixed(&mut r, seed, index);
            c.bus.write16(TRICK_TABLE + index * 0x14 + 4, r.next_u32() as u16);
        });
    }

    #[test]
    fn bebagtint_get_trick_field6_matches() {
        check_lohi(ee::s01::f_00150138__generated, bebagtint_get_trick_field6, SEEDS, &|c, seed| {
            let mut r = rng(0x0015_0138, seed);
            scramble(c, &mut r);
            let index = (seed % 8) as u32;
            c.g[A2][0] = mixed(&mut r, seed, index);
            c.bus.write16(TRICK_TABLE + index * 0x14 + 6, r.next_u32() as u16);
        });
    }
}
