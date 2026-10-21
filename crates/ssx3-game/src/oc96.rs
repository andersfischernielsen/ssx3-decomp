//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements ordered by runtime hotness, each with a differential test
//! against the generated body.
//!
//! Most of this wave is the "stub" layer of the game: empty rider/replay
//! hooks, one-field setters, table lookups, kernel syscall wrappers and the
//! character→voice/speech mask tables. A handful are jump-table dispatchers
//! (`Nash_ClassifyEffect`, the two audio mask lookups, the rider wipeout
//! animation selectors) whose `__entries` twins can be re-entered at any
//! block, so those ports are written as stage machines keyed on `pc`, like
//! the generated dispatchers.
//!
//! Calls follow [`crate::call`]: the same registers at each call, the same
//! return address, the unwind check after it, and the same cycle charge per
//! block. Registers the originals leave behind (`v0`, `v1`, `a0`..) are
//! reproduced, including scratch values.

use crate::abi;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002f_6908, rider_base_render_cheat_code, "RiderBase_RenderCheatCode"),
    (0x0012_5ad0, rider_empty_progress_hook, "Rider_EmptyProgressHook"),
    (0x0011_25b8, rider_return_zero_25b8, "Rider_ReturnZero_25b8"),
    (0x0042_4160, sce_sif_set_dma, "sceSifSetDma"),
    (0x0042_4190, isce_sif_set_dchain, "isceSifSetDChain"),
    (0x0042_4020, flush_cache, "FlushCache"),
    (0x0022_a270, nash_classify_effect, "Nash_ClassifyEffect"),
    (0x0042_3df0, poll_sema, "PollSema"),
    (0x003d_d310, ea_file_release_request, "EAFile_ReleaseRequest"),
    (0x003c_2438, snd_tick_stub, "SND_TickStub"),
    (0x003c_d190, snd_eaxa_set_history, "SND_EaxaSetHistory"),(0x0036_73d0, tex_man_end_frame_stub, "TexMan_EndFrameStub"),
    (0x0036_1f90, intc_table_end_frame_stub, "IntcTable_EndFrameStub"),
    (0x002d_21b8, stats_process_render_stub, "StatsProcess_RenderStub"),
    (0x002d_21b0, stats_process_update_stub, "StatsProcess_UpdateStub"),
    (0x0028_b240, audio_get_frame_rate, "Audio_GetFrameRate"),
    (0x0022_9f80, nash_empty_hook, "Nash_EmptyHook"),(0x003d_b040, spch_notify_part_callback, "SPCH_NotifyPartCallback"),
    (0x0012_da88, rider_wipeout_sub_select_tumble_anim, "RiderWipeoutSub_SelectTumbleAnim"),
    (0x0022_6600, luno_vm_register_set_raw, "LunoVMRegister_SetRaw"),(0x0022_24a8, luno_vm_pass_through, "LunoVM_PassThrough"),
    (0x003a_6d00, world_get_table_entry, "World_GetTableEntry"),
    (0x0022_6618, luno_vm_register_as_float, "LunoVMRegister_AsFloat"),
    (0x003a_6cc8, world_get_object_section_data, "World_GetObjectSectionData"),
    (0x0029_f2b0, character_to_speech_mask, "SSXAudioSystem_CharacterToSpeechMask"),
    (0x0042_4140, sce_sif_dma_stat, "sceSifDmaStat"),
    (0x0012_df48, rider_wipeout_sub_play_end_anim_a, "RiderWipeoutSub_PlayEndAnimA"),
    (0x002a_bce8, sound_request_set_bank_sample, "SoundRequest_SetBankSample"),
    (0x002a_1da0, character_to_voice_mask, "SSXAudioSystem_CharacterToVoiceMask"),
    (0x002a_d990, bank_monitor_timer_disable, "BankMonitor_TimerDisable"),
    (0x003c_b538, snd_dsp_resample_set_rate, "SND_DspResample_SetRate"),
    (0x002f_7a68, board_spark_emitter_ctor, "BoardSparkEmitter_ctor"),
    (0x0011_3998, air_predictor_clear_air_time, "AirPredictor_ClearAirTime"),
    (0x0026_ed88, replay_snapshot_empty_hook_a, "Replay_SnapshotEmptyHookA"),
    (0x0013_9a18, rider_air_state_exit, "RiderAirState_Exit"),
    (0x002a_9f30, stream_buffer_set_base_volume, "StreamBuffer_SetBaseVolume"),
    (0x0012_e9b0, rider_prewind_sub_exit, "RiderPrewindSub_Exit"),
    (0x0011_6930, rider_empty_stance_hook, "Rider_EmptyStanceHook"),
    (0x0035_aae0, multi_spline_modifier_release, "MultiSplineModifier_Release"),
    (0x0022_6610, luno_vm_register_as_int, "LunoVMRegister_AsInt"),
    (0x0011_7900, rider_metrix_get_trick_points, "RiderMetrix_GetTrickPoints"),
    (0x003c_e0d8, snd_utk_set_mode, "SND_UtkSetMode"),
    (0x003c_6ce0, sndstrm_stop_slot_unsupported, "SNDSTRM_StopSlotUnsupported"),
    (0x0027_05a8, replay_empty_rider_hook, "Replay_EmptyRiderHook"),
    (0x0013_6f28, rider_wipeout_state_exit, "RiderWipeoutState_Exit"),
    (0x0010_3918, rider_anim_v03, "RiderAnim_v03"),
    (0x0011_1380, rider_base_enter_main_state, "RiderBase_EnterMainState"),
    (0x0011_12f8, rider_base_exit_main_state, "RiderBase_ExitMainState"),
    (
        0x0012_dd98,
        rider_wipeout_sub_switch_to_anim_set_a,
        "RiderWipeoutSub_SwitchToAnimSetA",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const EA_FILE_FREE_FILE_HANDLE: Callee =
        Callee { addr: 0x003d_e8c0, f: ee::s14::EAFile_FreeFileHandle };
    pub const EA_FILE_FREE_REQUEST: Callee =
        Callee { addr: 0x003d_e7b0, f: ee::s14::EAFile_FreeRequest };
    pub const SPCH_FIND_BANK_TYPE: Callee =
        Callee { addr: 0x003d_8330, f: ee::s14::SPCH_FindBankType };
    pub const RIDER_WIPEOUT_UPDATE_ANIM_SPEED: Callee =
        Callee { addr: 0x0012_e528, f: ee::s00::RiderWipeoutSub_UpdateAnimSpeed };
    pub const RIDER_ANIM_SEQ_END_TYPE00: Callee =
        Callee { addr: 0x0010_4ca0, f: ee::s00::RiderAnim_SeqEndType00 };
    pub const RIDER_ANIM_SEQ_END_TYPE01: Callee =
        Callee { addr: 0x0010_4a40, f: ee::s00::RiderAnim_SeqEndType01 };
    pub const RIDER_ANIM_SEQ_END_TYPE02: Callee =
        Callee { addr: 0x0010_4a60, f: ee::s00::RiderAnim_SeqEndType02 };
    pub const RIDER_ANIM_SEQ_END_TYPE03: Callee =
        Callee { addr: 0x0010_4b78, f: ee::s00::RiderAnim_SeqEndType03 };
    pub const RIDER_ANIM_SEQ_END_TYPE04: Callee =
        Callee { addr: 0x0010_4c18, f: ee::s00::RiderAnim_SeqEndType04 };
    pub const RIDER_ANIM_SEQ_END_TYPE05: Callee =
        Callee { addr: 0x0010_4b48, f: ee::s00::RiderAnim_SeqEndType05 };
    pub const RIDER_ANIM_SEQ_END_TYPE06: Callee =
        Callee { addr: 0x0010_4c38, f: ee::s00::RiderAnim_SeqEndType06 };
    pub const RIDER_ANIM_SEQ_END_TYPE07: Callee =
        Callee { addr: 0x0010_4b98, f: ee::s00::RiderAnim_SeqEndType07 };
    pub const RIDER_ANIM_SEQ_END_TYPE08: Callee =
        Callee { addr: 0x0010_4bb8, f: ee::s00::RiderAnim_SeqEndType08 };
    pub const RIDER_ANIM_SEQ_END_TYPE09: Callee =
        Callee { addr: 0x0010_4bd8, f: ee::s00::RiderAnim_SeqEndType09 };
    pub const RIDER_ANIM_SEQ_END_TYPE0A: Callee =
        Callee { addr: 0x0010_4c80, f: ee::s00::RiderAnim_SeqEndType0A };
}

/// `call_indirect` targets the generated code reaches through the function
/// table (cross-shard `jalr`/`call_indirect`).
const ANIM_TABLE_GET_ENTRY_ADDR: u32 = 0x0031_15c8;
const ANIM_PLAYER_GET_ANIM_FIELD: u32 = 0x0031_2aa0;
const ANIM_CONTROLLER_SET_ANIM: u32 = 0x0031_28e8;
/// Rider state enter/exit methods (cross-shard `call_indirect`).
const RIDER_GROUND_ENTER: u32 = 0x0013_c7a8;
const RIDER_AIR_ENTER: u32 = 0x0013_99e0;
const RIDER_WIPEOUT_ENTER: u32 = 0x0013_6c40;
const RIDER_RAILSLIDE_ENTER: u32 = 0x0013_ad20;
const RIDER_HANDPLANT_ENTER: u32 = 0x0013_8b48;
const RIDER_GROUND_EXIT: u32 = 0x0013_f410;
const RIDER_AIR_EXIT: u32 = 0x0013_9a18;
const RIDER_WIPEOUT_EXIT: u32 = 0x0013_6f28;
const RIDER_RAILSLIDE_EXIT: u32 = 0x0013_c5a0;
const RIDER_HANDPLANT_EXIT: u32 = 0x0013_9178;

// ---------------------------------------------------------------------------
// Helpers.

/// `jalr` through the function table (the generated `call_indirect` form).
#[inline]
fn call_table(c: &mut Cpu, target: u32, ret: u32) -> crate::call::Flow {
    call_ptr(c, target, ret)
}

/// The `syscall` trap: `pc = ret`, invoke the hook at `at`; `false` when the
/// hook unwound (a thread switch mid-syscall).
#[inline]
fn do_syscall(c: &mut Cpu, at: u32, ret: u32) -> bool {
    c.pc = ret;
    c.syscall(at);
    c.pc == ret
}

/// The final due-event check of a syscall wrapper (result ignored, as in the
/// compiled code).
#[inline]
fn event_if_due(c: &mut Cpu) {
    if event_due(c) {
        deliver_event_here(c, c.pc);
    }
}

/// One `syscall` wrapper: 4 cycles, `v1 = code`, trap at `at`, return to
/// `ret`, then a due-event check and `jr ra`.
#[inline]
fn syscall_wrapper(c: &mut Cpu, code: u32, at: u32, ret: u32) {
    c.cycles += 4;
    c.g[V1][0] = sx(code);
    if !do_syscall(c, at, ret) {
        return;
    }
    event_if_due(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Leaves: empty hooks.

/// `void RiderBase_RenderCheatCode(RiderBase* r)` (0x2f6908): render hook with
/// no cheat code to draw.
pub fn rider_base_render_cheat_code(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void Rider_EmptyProgressHook(Rider* r)` (0x125ad0): per-frame progress
/// hook, not installed.
pub fn rider_empty_progress_hook(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `int Rider_ReturnZero_25b8(void* self)` (0x1125b8): a virtual method that
/// always returns 0.
pub fn rider_return_zero_25b8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void SND_TickStub(void)` (0x3c2438): the audio system's per-frame tick
/// when no sequencer is installed.
pub fn snd_tick_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void TexMan_EndFrameStub(void)` (0x3673d0): texture manager end-of-frame
/// hook, not installed.
pub fn tex_man_end_frame_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void IntcTable_EndFrameStub(void)` (0x361f90): interrupt table end-of-frame
/// hook, not installed.
pub fn intc_table_end_frame_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void StatsProcess_RenderStub(void)` (0x2d21b8): stats render hook, not
/// installed.
pub fn stats_process_render_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void StatsProcess_UpdateStub(void)` (0x2d21b0): stats update hook, not
/// installed.
pub fn stats_process_update_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void Nash_EmptyHook(void)` (0x229f80): the empty Nash (effect) callback.
pub fn nash_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void Replay_SnapshotEmptyHookA(void)` (0x26ed88): replay snapshot hook, not
/// installed.
pub fn replay_snapshot_empty_hook_a(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void RiderAirState_Exit(RiderAirState* s)` (0x139a18): the air state has
/// nothing to tear down.
pub fn rider_air_state_exit(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void RiderPrewindSub_Exit(RiderPrewindSub* s)` (0x12e9b0): nothing to tear
/// down.
pub fn rider_prewind_sub_exit(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void Rider_EmptyStanceHook(Rider* r)` (0x116930): stance hook, not
/// installed.
pub fn rider_empty_stance_hook(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void Replay_EmptyRiderHook(Rider* r)` (0x2705a8): replay rider hook, not
/// installed.
pub fn replay_empty_rider_hook(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void RiderWipeoutState_Exit(RiderWipeoutState* s)` (0x136f28): nothing to
/// tear down.
pub fn rider_wipeout_state_exit(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Leaves: constants and field setters.

/// `int Audio_GetFrameRate(void)` (0x28b240): the audio system's fixed 60 Hz
/// frame rate (`0x3c`).
pub fn audio_get_frame_rate(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x3c;
    abi::ret(c);
}

/// `void* LunoVMRegister_SetRaw(Register* r, void* value, void* tag)`
/// (0x226600): store the value and its tag in the register object.
pub fn luno_vm_register_set_raw(c: &mut Cpu) {
    c.cycles += 3;
    let r = abi::arg(c, 0);
    Addr(r.wrapping_add(0x4)).sw(c, c.g[A2][0]);
    let v = c.g[A1][0];
    Addr(r).sw(c, v);
    abi::ret(c);
}

/// `void* LunoVM_PassThrough(void* value)` (0x2224a8): the identity converter.
pub fn luno_vm_pass_through(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `void* LunoVMRegister_AsFloat(Register* r)` (0x226618): the float view of
/// a register (the same word).
pub fn luno_vm_register_as_float(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `void* LunoVMRegister_AsInt(Register* r)` (0x226610): the integer view of
/// a register (the same word).
pub fn luno_vm_register_as_int(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `void* BoardSparkEmitter_ctor(BoardSparkEmitter* e)` (0x2f7a68): the
/// constructor returns its argument (the base member is initialised inline).
pub fn board_spark_emitter_ctor(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `void* World_GetTableEntry(void** table, int index)` (0x3a6d00): entry
/// `index` of the pointer table.
pub fn world_get_table_entry(c: &mut Cpu) {
    c.cycles += 5;
    let table = abi::arg(c, 0);
    c.g[V0][0] = Addr(table).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let entry = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = entry;
    abi::ret(c);
}

/// `void* World_GetObjectSectionData(World* w, WorldObject* obj)`
/// (0x3a6cc8): the section data of the object's current model: `obj->+0x88`
/// → `[+0xc]` → `[+8]`.
pub fn world_get_object_section_data(c: &mut Cpu) {
    c.cycles += 4;
    let obj = Addr(abi::arg(c, 1));
    c.g[V0][0] = obj.off(0x88).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
    let v = Addr(c.g[V1][0] as u32).off(0x8).lw(c);
    c.g[V0][0] = v;
    abi::ret(c);
}

/// `void AirPredictor_ClearAirTime(AirPredictor* p)` (0x113998): reset the
/// accumulated air time.
pub fn air_predictor_clear_air_time(c: &mut Cpu) {
    c.cycles += 2;
    let p = abi::arg(c, 0);
    Addr(p.wrapping_add(0x98)).sw(c, 0);
    abi::ret(c);
}

/// `void StreamBuffer_SetBaseVolume(StreamBuffer* b, u8 volume)` (0x2a9f30):
/// store the volume byte at +0xc.
pub fn stream_buffer_set_base_volume(c: &mut Cpu) {
    c.cycles += 2;
    let b = abi::arg(c, 0);
    let v = c.g[A1][0];
    Addr(b.wrapping_add(0xc)).sb(c, v);
    abi::ret(c);
}

/// `void BankMonitor_TimerDisable(BankMonitor* m)` (0x2ad990): clear the timer
/// word.
pub fn bank_monitor_timer_disable(c: &mut Cpu) {
    c.cycles += 2;
    let m = abi::arg(c, 0);
    Addr(m).sw(c, 0);
    abi::ret(c);
}

/// `void SND_DspResample_SetRate(SND_DspResample* r, int rate)` (0x3cb538):
/// store the resample rate at +0x1c.
pub fn snd_dsp_resample_set_rate(c: &mut Cpu) {
    c.cycles += 2;
    let r = abi::arg(c, 0);
    let rate = c.g[A1][0];
    Addr(r.wrapping_add(0x1c)).sw(c, rate);
    abi::ret(c);
}

/// `void SND_UtkSetMode(SND_Utk* u, int mode)` (0x3ce0d8): store the mode word
/// at +0xd64.
pub fn snd_utk_set_mode(c: &mut Cpu) {
    c.cycles += 2;
    let u = abi::arg(c, 0);
    let mode = c.g[A1][0];
    Addr(u.wrapping_add(0xd64)).sw(c, mode);
    abi::ret(c);
}

/// `int SNDSTRM_StopSlotUnsupported(void)` (0x3c6ce0): -15, the "not supported"
/// error.
pub fn sndstrm_stop_slot_unsupported(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((-15i32) as u32);
    abi::ret(c);
}

/// `void SND_EaxaSetHistory(SND_Eaxa* e, float* history)` (0x3cd190): copy the
/// first two history floats into the EAXA object.
pub fn snd_eaxa_set_history(c: &mut Cpu) {
    c.cycles += 5;
    let e = abi::arg(c, 0);
    let h = abi::arg(c, 1);
    c.lwc1(0, h);
    c.swc1(0, e.wrapping_add(0x98));
    c.lwc1(1, h.wrapping_add(0x4));
    let a = e.wrapping_add(0x9c);
    c.swc1(1, a);
    abi::ret(c);
}

/// `void SoundRequest_SetBankSample(SoundRequest* r, int bank, int sample)`
/// (0x2abce8): store the bank at +0xc and the sample at +0x10.
pub fn sound_request_set_bank_sample(c: &mut Cpu) {
    c.cycles += 3;
    let r = abi::arg(c, 0);
    let sample = c.g[A2][0];
    Addr(r.wrapping_add(0x10)).sw(c, sample);
    let bank = c.g[A1][0];
    Addr(r.wrapping_add(0xc)).sw(c, bank);
    abi::ret(c);
}

/// `void MultiSplineModifier_Release(MultiSplineModifier* m)` (0x35aae0):
/// decrement the reference count and store it back.
pub fn multi_spline_modifier_release(c: &mut Cpu) {
    c.cycles += 4;
    let m = abi::arg(c, 0);
    c.g[V0][0] = Addr(m.wrapping_add(0x34)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
    let count = c.g[V0][0];
    Addr(m.wrapping_add(0x34)).sw(c, count);
    abi::ret(c);
}

/// `float RiderMetrix_GetTrickPoints(RiderMetrix* m)` (0x117900): the trick
/// point total held at +0x14.
pub fn rider_metrix_get_trick_points(c: &mut Cpu) {
    c.cycles += 2;
    let m = abi::arg(c, 0);
    c.lwc1(0, m.wrapping_add(0x14));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Kernel syscall wrappers.

/// `int sceSifSetDma(SifDmaTransfer* t, int len)` (0x424160): syscall 0x77.
pub fn sce_sif_set_dma(c: &mut Cpu) {
    syscall_wrapper(c, 0x77, 0x0042_4164, 0x0042_4168);
}

/// `int isceSifSetDChain(void)` (0x424190): syscall -0x78 (0xffffff88).
pub fn isce_sif_set_dchain(c: &mut Cpu) {
    syscall_wrapper(c, (-120i32) as u32, 0x0042_4194, 0x0042_4198);
}

/// `void FlushCache(int mode)` (0x424020): syscall 0x64.
pub fn flush_cache(c: &mut Cpu) {
    syscall_wrapper(c, 0x64, 0x0042_4024, 0x0042_4028);
}

/// `int PollSema(int sema)` (0x423df0): syscall 0x45.
pub fn poll_sema(c: &mut Cpu) {
    syscall_wrapper(c, 0x45, 0x0042_3df4, 0x0042_3df8);
}

/// `int sceSifDmaStat(unsigned int id)` (0x424140): syscall 0x76.
pub fn sce_sif_dma_stat(c: &mut Cpu) {
    syscall_wrapper(c, 0x76, 0x0042_4144, 0x0042_4148);
}

// ---------------------------------------------------------------------------
// Nash_ClassifyEffect: a jump-table dispatcher (VU0 setup, then three
// per-effect registration lists).
//
// The port follows the generated `__entries` twin block for block, so a
// thread switch that resumes it at any of the table's case labels behaves
// like the generated function.

/// The two registration lists `Nash_ClassifyEffect` appends to (word at
/// `self + 0x7ab8` is the head; a count at `+0x78b4`/`+0x7abc` indexes the
/// inline arrays at `+0x78b8`/`+0x7ac0`). The effect type also selects one of
/// three lists in the table at `.rodata 0x47b380`.
pub fn nash_classify_effect(c: &mut Cpu) {
    let mut pc = c.pc as u32;
    loop {
        match pc {
            0x0022_a270 => {
                c.cycles += 3;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                let null_node = c.g[A1][0] == 0;
                c.g[A3][0] = c.g[A0][0];
                pc = if null_node { 0x0022_a2e4 } else { 0x0022_a27c };
            }
            0x0022_a27c => {
                c.cycles += 26;
                let node = c.g[A1][0] as u32;
                let sp = c.g[SP][0] as u32;
                c.lwc1(0, node.wrapping_add(0x68));
                c.lwc1(1, node.wrapping_add(0x6c));
                c.lwc1(2, node.wrapping_add(0x70));
                c.lwc1(3, node.wrapping_add(0x74));
                c.lwc1(4, node.wrapping_add(0x78));
                c.lwc1(5, node.wrapping_add(0x7c));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 6);
                c.swc1(0, sp);
                c.swc1(1, sp.wrapping_add(0x4));
                c.swc1(2, sp.wrapping_add(0x8));
                c.swc1(3, sp.wrapping_add(0x10));
                c.swc1(4, sp.wrapping_add(0x14));
                c.swc1(5, sp.wrapping_add(0x18));
                c.swc1(6, sp.wrapping_add(0x1c));
                c.swc1(6, sp.wrapping_add(0xc));
                c.cfc2(A0, 1);
                c.g[V1][0] = sx(sp.wrapping_add(0x10));
                c.g[V0][0] = sx(node.wrapping_add(0x50));
                c.lqc2(20, sp);
                c.lqc2(21, c.g[V1][0] as u32);
                c.lqc2(22, c.g[V0][0] as u32);
                c.ctc2(6, 14);
                c.vcallms(0xdb8);
                c.g[V0][0] = (c.g[A0][0] < 0x5u64) as u64;
                pc = 0x0022_a2ec;
            }
            0x0022_a2e4 => {
                c.cycles += 2;
                c.cfc2(A0, 1);
                c.g[V0][0] = (c.g[A0][0] < 0x5u64) as u64;
                pc = 0x0022_a2ec;
            }
            0x0022_a2ec => {
                c.cycles += 2;
                let out_of_range = c.g[V0][0] == 0;
                c.g[V0][0] = 0x48_0000;
                pc = if out_of_range { 0x0022_a358 } else { 0x0022_a2f4 };
            }
            0x0022_a2f4 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4c80));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let target = c.g[A0][0] as u32;
                pc = match target {
                    0x0022_a30c => 0x0022_a30c,
                    0x0022_a334 => 0x0022_a334,
                    0x0022_a358 => 0x0022_a358,
                    _ => {
                        c.jump_out(target, 0x0022_a270, 0x0022_a364);
                        return;
                    }
                };
            }
            0x0022_a30c => {
                c.cycles += 3;
                let this = c.g[A3][0] as u32;
                c.g[A0][0] = Addr(this.wrapping_add(0x7ab8)).lw(c);
                if c.g[A0][0] == 0 {
                    let v = c.g[A1][0];
                    Addr(this.wrapping_add(0x7ab8)).sw(c, v);
                    pc = 0x0022_a35c;
                } else {
                    pc = 0x0022_a318;
                }
            }
            0x0022_a318 => {
                c.cycles += 7;
                let head = c.g[A3][0] as u32;
                c.g[V1][0] = Addr(head.wrapping_add(0x78b4)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(head));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                let slot = c.g[V0][0] as u32;
                let prev = c.g[A0][0];
                Addr(slot.wrapping_add(0x78b8)).sw(c, prev);
                let count = c.g[V1][0];
                Addr(head.wrapping_add(0x78b4)).sw(c, count);
                pc = 0x0022_a358;
            }
            0x0022_a334 => {
                c.cycles += 3;
                let this = c.g[A3][0] as u32;
                c.g[A0][0] = Addr(this.wrapping_add(0x7ab8)).lw(c);
                if c.g[A0][0] == 0 {
                    let v = c.g[A1][0];
                    Addr(this.wrapping_add(0x7ab8)).sw(c, v);
                    pc = 0x0022_a35c;
                } else {
                    pc = 0x0022_a340;
                }
            }
            0x0022_a340 => {
                c.cycles += 6;
                let head = c.g[A3][0] as u32;
                c.g[V1][0] = Addr(head.wrapping_add(0x7abc)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(head));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                let slot = c.g[V0][0] as u32;
                let prev = c.g[A0][0];
                Addr(slot.wrapping_add(0x7ac0)).sw(c, prev);
                let count = c.g[V1][0];
                Addr(head.wrapping_add(0x7abc)).sw(c, count);
                pc = 0x0022_a358;
            }
            0x0022_a358 => {
                c.cycles += 1;
                let this = c.g[A3][0] as u32;
                let node = c.g[A1][0];
                Addr(this.wrapping_add(0x7ab8)).sw(c, node);
                pc = 0x0022_a35c;
            }
            0x0022_a35c => {
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x0022_a270, 0x0022_a364);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Audio character masks: two jump-table lookups in `.rodata`.

/// `u32 SSXAudioSystem_CharacterToSpeechMask(int character, int unused)`
/// (0x29f2b0): one speech bit per character id 0..9 (1,2,4,8,0x10,0x20,0x80,
/// 0x40,0x100,0x200); 0 for an unknown id or a non-null second argument.
pub fn character_to_speech_mask(c: &mut Cpu) {
    let mut pc = c.pc as u32;
    loop {
        match pc {
            0x0029_f2b0 => {
                c.cycles += 2;
                let skip = c.g[A1][0] != 0;
                c.g[V0][0] = (c.g[A0][0] < 0xau64) as u64;
                pc = if skip { 0x0029_f328 } else { 0x0029_f2b8 };
            }
            0x0029_f2b8 => {
                c.cycles += 2;
                let skip = c.g[V0][0] == 0;
                c.g[V0][0] = 0x48_0000;
                pc = if skip { 0x0029_f328 } else { 0x0029_f2c0 };
            }
            0x0029_f2c0 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2e60));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let target = c.g[A0][0] as u32;
                pc = match target {
                    0x0029_f2d8 => 0x0029_f2d8,
                    0x0029_f2e0 => 0x0029_f2e0,
                    0x0029_f2e8 => 0x0029_f2e8,
                    0x0029_f2f0 => 0x0029_f2f0,
                    0x0029_f2f8 => 0x0029_f2f8,
                    0x0029_f300 => 0x0029_f300,
                    0x0029_f308 => 0x0029_f308,
                    0x0029_f310 => 0x0029_f310,
                    0x0029_f318 => 0x0029_f318,
                    0x0029_f320 => 0x0029_f320,
                    _ => {
                        c.jump_out(target, 0x0029_f2b0, 0x0029_f330);
                        return;
                    }
                };
            }
            0x0029_f2d8 | 0x0029_f2e0 | 0x0029_f2e8 | 0x0029_f2f0 | 0x0029_f2f8
            | 0x0029_f300 | 0x0029_f308 | 0x0029_f310 | 0x0029_f318 | 0x0029_f320 => {
                c.cycles += 2;
                let mask = match pc {
                    0x0029_f2d8 => 0x1,
                    0x0029_f2e0 => 0x2,
                    0x0029_f2e8 => 0x4,
                    0x0029_f2f0 => 0x8,
                    0x0029_f2f8 => 0x10,
                    0x0029_f300 => 0x20,
                    0x0029_f308 => 0x80,
                    0x0029_f310 => 0x40,
                    0x0029_f318 => 0x100,
                    _ => 0x200,
                };
                c.g[V0][0] = mask;
                abi::ret(c);
                return;
            }
            0x0029_f328 => {
                c.cycles += 2;
                c.g[V0][0] = 0;
                abi::ret(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x0029_f2b0, 0x0029_f330);
                return;
            }
        }
    }
}

/// `u32 SSXAudioSystem_CharacterToVoiceMask(int character, int unused)`
/// (0x2a1da0): one voice bit per character id 0..9 (1,2,4,8,0x10,0x20,0x40,
/// 0x80,0x100,0x200); 0x1000 for an unknown id or a non-null third argument.
pub fn character_to_voice_mask(c: &mut Cpu) {
    let mut pc = c.pc as u32;
    loop {
        match pc {
            0x002a_1da0 => {
                c.cycles += 2;
                let skip = c.g[A2][0] != 0;
                c.g[V0][0] = (c.g[A1][0] < 0xau64) as u64;
                pc = if skip { 0x002a_1e18 } else { 0x002a_1da8 };
            }
            0x002a_1da8 => {
                c.cycles += 2;
                let skip = c.g[V0][0] == 0;
                c.g[V0][0] = 0x48_0000;
                pc = if skip { 0x002a_1e18 } else { 0x002a_1db0 };
            }
            0x002a_1db0 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2fe0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let target = c.g[A0][0] as u32;
                pc = match target {
                    0x002a_1dc8 => 0x002a_1dc8,
                    0x002a_1dd0 => 0x002a_1dd0,
                    0x002a_1dd8 => 0x002a_1dd8,
                    0x002a_1de0 => 0x002a_1de0,
                    0x002a_1de8 => 0x002a_1de8,
                    0x002a_1df0 => 0x002a_1df0,
                    0x002a_1df8 => 0x002a_1df8,
                    0x002a_1e00 => 0x002a_1e00,
                    0x002a_1e08 => 0x002a_1e08,
                    0x002a_1e10 => 0x002a_1e10,
                    _ => {
                        c.jump_out(target, 0x002a_1da0, 0x002a_1e20);
                        return;
                    }
                };
            }
            0x002a_1dc8 | 0x002a_1dd0 | 0x002a_1dd8 | 0x002a_1de0 | 0x002a_1de8
            | 0x002a_1df0 | 0x002a_1df8 | 0x002a_1e00 | 0x002a_1e08 | 0x002a_1e10 => {
                c.cycles += 2;
                let mask = match pc {
                    0x002a_1dc8 => 0x1,
                    0x002a_1dd0 => 0x2,
                    0x002a_1dd8 => 0x4,
                    0x002a_1de0 => 0x8,
                    0x002a_1de8 => 0x10,
                    0x002a_1df0 => 0x20,
                    0x002a_1df8 => 0x40,
                    0x002a_1e00 => 0x80,
                    0x002a_1e08 => 0x100,
                    _ => 0x200,
                };
                c.g[V0][0] = mask;
                abi::ret(c);
                return;
            }
            0x002a_1e18 => {
                c.cycles += 2;
                c.g[V0][0] = 0x1000;
                abi::ret(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x002a_1da0, 0x002a_1e20);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// RiderBase state transitions: five virtual enter/exit methods selected by the
// rider's current main-state id (`rider + 0xde0`, 0..5) through a `.rodata`
// table. The ports accept every `__entries` resume label.

/// `void RiderBase_EnterMainState(RiderBase* r)` (0x111380): enter the method
/// of the rider's current state (ground, air, wipeout, railslide, handplant or
/// the base state).
pub fn rider_base_enter_main_state(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0011_1380 => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x10);
                    c.g[A2][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xde0).lw(c);
                    c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                    let out_of_range = c.g[V0][0] == 0;
                    // The generated code reloads the table base into v0
                    // before branching; 0x11139c adds 0x6b30 to it.
                    c.g[V0][0] = 0x45_0000;
                    pc = if out_of_range { 0x0011_13fc } else { 0x0011_139c };
                }
                0x0011_139c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6b30));
                    c.g[V1][0] =
                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match target {
                        0x0011_13b4 => 0x0011_13b4,
                        0x0011_13c4 => 0x0011_13c4,
                        0x0011_13d4 => 0x0011_13d4,
                        0x0011_13e4 => 0x0011_13e4,
                        0x0011_13f4 => 0x0011_13f4,
                        0x0011_13fc => 0x0011_13fc,
                        _ => {
                            c.jump_out(target, 0x0011_1380, 0x0011_1408);
                            return Ok(());
                        }
                    };
                }
                0x0011_13b4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[A2][0];
                    call_table(c, RIDER_GROUND_ENTER, 0x0011_13bc)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1400;
                }
                0x0011_13c4 => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    call_table(c, RIDER_AIR_ENTER, 0x0011_13cc)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1400;
                }
                0x0011_13d4 => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30));
                    call_table(c, RIDER_WIPEOUT_ENTER, 0x0011_13dc)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1400;
                }
                0x0011_13e4 => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xb0));
                    call_table(c, RIDER_RAILSLIDE_ENTER, 0x0011_13ec)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1400;
                }
                0x0011_13f4 => {
                    c.cycles += 2;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x110));
                    call_table(c, RIDER_HANDPLANT_ENTER, 0x0011_13fc)?;
                    pc = 0x0011_13fc;
                }
                0x0011_13fc => {
                    c.cycles += 1;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1400;
                }
                0x0011_1400 => {
                    c.cycles += 2;
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0011_1380, 0x0011_1408);
                    return Ok(());
                }
            }
        }
    })
}

/// `void RiderBase_ExitMainState(RiderBase* r)` (0x1112f8): leave the method
/// of the rider's current state (same table as the enter side).
pub fn rider_base_exit_main_state(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0011_12f8 => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x10);
                    c.g[A2][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xde0).lw(c);
                    c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                    let out_of_range = c.g[V0][0] == 0;
                    // The generated code reloads the table base into v0
                    // before branching; 0x111314 adds 0x6b10 to it.
                    c.g[V0][0] = 0x45_0000;
                    pc = if out_of_range { 0x0011_1374 } else { 0x0011_1314 };
                }
                0x0011_1314 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6b10));
                    c.g[V1][0] =
                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match target {
                        0x0011_132c => 0x0011_132c,
                        0x0011_133c => 0x0011_133c,
                        0x0011_134c => 0x0011_134c,
                        0x0011_135c => 0x0011_135c,
                        0x0011_136c => 0x0011_136c,
                        0x0011_1374 => 0x0011_1374,
                        _ => {
                            c.jump_out(target, 0x0011_12f8, 0x0011_1380);
                            return Ok(());
                        }
                    };
                }
                0x0011_132c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[A2][0];
                    call_table(c, RIDER_GROUND_EXIT, 0x0011_1334)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1378;
                }
                0x0011_133c => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    call_table(c, RIDER_AIR_EXIT, 0x0011_1344)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1378;
                }
                0x0011_134c => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30));
                    call_table(c, RIDER_WIPEOUT_EXIT, 0x0011_1354)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1378;
                }
                0x0011_135c => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xb0));
                    call_table(c, RIDER_RAILSLIDE_EXIT, 0x0011_1364)?;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1378;
                }
                0x0011_136c => {
                    c.cycles += 2;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x110));
                    call_table(c, RIDER_HANDPLANT_EXIT, 0x0011_1374)?;
                    pc = 0x0011_1374;
                }
                0x0011_1374 => {
                    c.cycles += 1;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1378;
                }
                0x0011_1378 => {
                    c.cycles += 2;
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0011_12f8, 0x0011_1380);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// EAFile_ReleaseRequest: a release path chosen from flags in the request and
// the request-state table at `.rodata 0x495d40`.

/// `int EAFile_ReleaseRequest(EAFileRequest* req)` (0x3dd310): clear the
/// request and, depending on its state and channel type, free the associated
/// file handle first. The request is found through a 256-entry hash of its
/// address's top byte (table pointer at 0x51ad98, 0x30-byte entries).
pub fn ea_file_release_request(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x003d_d310 => {
                    c.cycles += 15;
                    let frame = Frame::open(c, 0x40);
                    c.g[A1][0] = 0x30;
                    let req = c.g[A0][0];
                    let sp0 = frame.at(c, 0);
                    Addr(sp0).sw(c, req);
                    c.g[A0][0] = 0x52_0000;
                    frame.save(c, S0, 0x30);
                    c.g[V0][0] = Addr(frame.at(c, 0x3)).lbu(c);
                    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_sub(0x6528)).lw(c);
                    c.mult(0, V0, V0, A1);
                    frame.save(c, S1, 0x20);
                    frame.save_ra(c, 0x10);
                    c.g[S1][0] = 0;
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
                    pc = if c.g[V1][0] == 0 { 0x003d_d414 } else { 0x003d_d34c };
                }
                0x003d_d34c => {
                    c.cycles += 2;
                    let positive = (c.g[V1][0] as i64) > 0;
                    c.g[V0][0] = 1;
                    pc = if positive { 0x003d_d368 } else { 0x003d_d354 };
                }
                0x003d_d354 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < -2i64) as u64;
                    pc = if c.g[V0][0] != 0 { 0x003d_d414 } else { 0x003d_d360 };
                }
                0x003d_d360 => {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    pc = 0x003d_d3f0;
                }
                0x003d_d368 => {
                    c.cycles += 2;
                    pc = if c.g[V1][0] != c.g[V0][0] { 0x003d_d414 } else { 0x003d_d370 };
                }
                0x003d_d370 => {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 20);
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
                    c.g[V1][0] = (c.g[A0][0] < 0x9u64) as u64;
                    let out = c.g[V1][0] == 0;
                    c.g[V0][0] = 0x49_0000;
                    pc = if out { 0x003d_d414 } else { 0x003d_d38c };
                }
                0x003d_d38c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d40));
                    c.g[V1][0] =
                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match target {
                        0x003d_d3a4 => 0x003d_d3a4,
                        0x003d_d3ac => 0x003d_d3ac,
                        0x003d_d3b8 => 0x003d_d3b8,
                        0x003d_d3c0 => 0x003d_d3c0,
                        0x003d_d3c8 => 0x003d_d3c8,
                        0x003d_d3e8 => 0x003d_d3e8,
                        0x003d_d3f0 => 0x003d_d3f0,
                        _ => {
                            c.jump_out(target, 0x003d_d310, 0x003d_d434);
                            return Ok(());
                        }
                    };
                }
                0x003d_d3a4 => {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x24).lw(c);
                    pc = 0x003d_d414;
                }
                0x003d_d3ac => {
                    c.cycles += 3;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
                    pc = 0x003d_d414;
                }
                0x003d_d3b8 => {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x18).lw(c);
                    pc = 0x003d_d414;
                }
                0x003d_d3c0 => {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
                    pc = 0x003d_d414;
                }
                0x003d_d3c8 => {
                    c.cycles += 8;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x24).lw(c);
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x18).lw(c);
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        let a = c.g[A0][0] as u32;
                        let v = c.g[V1][0];
                        Addr(a.wrapping_add(0x4)).sw(c, v);
                        pc = 0x003d_d414;
                    } else {
                        pc = 0x003d_d3e8;
                    }
                }
                0x003d_d3e8 => {
                    c.cycles += 2;
                    pc = 0x003d_d414;
                }
                0x003d_d3f0 => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 20);
                    c.g[A0][0] = c.g[V0][0] & 0xf;
                    let is2 = c.g[A0][0] == c.g[V1][0];
                    c.g[V0][0] = 0x9;
                    pc = if is2 { 0x003d_d40c } else { 0x003d_d404 };
                }
                0x003d_d404 => {
                    c.cycles += 2;
                    pc = if c.g[A0][0] != c.g[V0][0] {
                        0x003d_d414
                    } else {
                        0x003d_d40c
                    };
                }
                0x003d_d40c => {
                    c.cycles += 2;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x24).lw(c);
                    callee::EA_FILE_FREE_FILE_HANDLE.call(c, 0x003d_d414)?;
                    pc = 0x003d_d414;
                }
                0x003d_d414 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    callee::EA_FILE_FREE_REQUEST.call(c, 0x003d_d41c)?;
                    c.g[V0][0] = c.g[S1][0];
                    let frame = Frame { size: 0x40 };
                    frame.restore(c, 16, 0x30);
                    frame.restore(c, 17, 0x20);
                    frame.restore_ra(c, 0x10);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d_d310, 0x003d_d434);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// RiderAnim_v03: the rider animation sequencer's per-sequence-end step. The
// sequence-end kind (0..10) selects one of ten handlers through a `.rodata`
// table, then the handler's return value is stored into the rider's animation
// slot array.

/// The ten sequence-end handlers and the return address each call resumes at.
const RIDER_ANIM_END_HANDLERS: [(u32, u32, Callee); 10] = [
    (0x0010_3978, 0x0010_3988, callee::RIDER_ANIM_SEQ_END_TYPE00),
    (0x0010_3990, 0x0010_39a0, callee::RIDER_ANIM_SEQ_END_TYPE01),
    (0x0010_39a8, 0x0010_39b8, callee::RIDER_ANIM_SEQ_END_TYPE02),
    (0x0010_39c0, 0x0010_39d0, callee::RIDER_ANIM_SEQ_END_TYPE03),
    (0x0010_39d8, 0x0010_39e8, callee::RIDER_ANIM_SEQ_END_TYPE04),
    (0x0010_39f0, 0x0010_3a00, callee::RIDER_ANIM_SEQ_END_TYPE05),
    (0x0010_3a08, 0x0010_3a18, callee::RIDER_ANIM_SEQ_END_TYPE06),
    (0x0010_3a20, 0x0010_3a30, callee::RIDER_ANIM_SEQ_END_TYPE07),
    (0x0010_3a38, 0x0010_3a48, callee::RIDER_ANIM_SEQ_END_TYPE08),
    (0x0010_3a50, 0x0010_3a60, callee::RIDER_ANIM_SEQ_END_TYPE09),
];

/// `void RiderAnim_v03(RiderAnim* anim, int seqEnd, int* out)`
/// (0x103918): on a sequence end, look the sequence up in the animation table
/// (0x446990, 28-byte entries), run that end kind's handler and write its
/// result to `out[0]`. The port accepts all `__entries` resume labels; the
/// sequence-end table entry that points back at the function loops.
pub fn rider_anim_v03(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0010_3918 => {
                    c.cycles += 18;
                    let frame = Frame::open(c, 0x50);
                    frame.save(c, 16, 0x40);
                    frame.save(c, 17, 0x30);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, 18, 0x20);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, 19, 0x10);
                    c.g[S2][0] = c.g[A2][0];
                    frame.save_ra(c, 0);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x50).lw(c);
                    c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[S3][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                    call_ptr(c, ANIM_TABLE_GET_ENTRY_ADDR, 0x0010_3950)?;
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
                    c.g[V0][0] = (c.g[V1][0] < 0xbu64) as u64;
                    let out_of_range = c.g[V0][0] == 0;
                    // The generated code loads the table base into v0 before
                    // branching (the dispatch block below adds 0x6990 to it).
                    c.g[V0][0] = 0x45_0000;
                    pc = if out_of_range { 0x0010_3a84 } else { 0x0010_3960 };
                }
                0x0010_3960 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6990));
                    c.g[V1][0] =
                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    if target == 0x0010_3918 {
                        if back_edge_unwinds(c, 0x0010_3918) {
                            return Ok(());
                        }
                        pc = 0x0010_3918;
                    } else if let Some(k) = RIDER_ANIM_END_HANDLERS.iter().position(|h| h.0 == target) {
                        pc = RIDER_ANIM_END_HANDLERS[k].0;
                    } else if target == 0x0010_3a68 {
                        pc = 0x0010_3a68;
                    } else {
                        c.jump_out(target, 0x0010_3918, 0x0010_3aa0);
                        return Ok(());
                    }
                }
                0x0010_3978 | 0x0010_3990 | 0x0010_39a8 | 0x0010_39c0 | 0x0010_39d8
                | 0x0010_39f0 | 0x0010_3a08 | 0x0010_3a20 | 0x0010_3a38 | 0x0010_3a50 => {
                    c.cycles += 6;
                    let k = RIDER_ANIM_END_HANDLERS
                        .iter()
                        .position(|h| h.0 == pc)
                        .unwrap();
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S1][0];
                    let (_, ret, f) = RIDER_ANIM_END_HANDLERS[k];
                    f.call(c, ret)?;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
                    pc = 0x0010_3a7c;
                }
                0x0010_3a68 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S1][0];
                    callee::RIDER_ANIM_SEQ_END_TYPE0A.call(c, 0x0010_3a78)?;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
                    pc = 0x0010_3a7c;
                }
                0x0010_3a7c => {
                    c.cycles += 2;
                    c.g[V1][0] =
                        sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let a = c.g[V1][0] as u32;
                    let v = c.g[V0][0];
                    Addr(a).sw(c, v);
                    pc = 0x0010_3a84;
                }
                0x0010_3a84 => {
                    c.cycles += 7;
                    let frame = Frame { size: 0x50 };
                    frame.restore(c, 16, 0x40);
                    frame.restore(c, 17, 0x30);
                    frame.restore(c, 18, 0x20);
                    frame.restore(c, 19, 0x10);
                    frame.restore_ra(c, 0);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0010_3918, 0x0010_3aa0);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutSub animation selectors: look the current/next animation id up
// in a `.rodata` table and, when it actually changes, restart it through the
// rider's anim controller and rescale its speed.

/// The `RiderWipeoutSub` animation-id table (`.rodata 0x4580a0`): case block to
/// the id it selects.
const SWITCH_TO_ANIM_SET_A_IDS: [(u32, u64); 10] = [
    (0x0012_ddf8, 0x16a),
    (0x0012_de00, 0x16b),
    (0x0012_de08, 0x16c),
    (0x0012_de10, 0x16d),
    (0x0012_de18, 0x16e),
    (0x0012_de20, 0x16f),
    (0x0012_de28, 0x170),
    (0x0012_de30, 0x171),
    (0x0012_de38, 0x172),
    (0x0012_de40, 0x173),
];

/// `void RiderWipeoutSub_SwitchToAnimSetA(RiderWipeoutSub* w)` (0x12dd98):
/// pick animation set A's id for the rider's current animation and, if that is
/// not what is playing, restart it at speed -1 and update the anim speed.
pub fn rider_wipeout_sub_switch_to_anim_set_a(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0012_dd98 => {
                    c.cycles += 14;
                    let frame = Frame::open(c, 0x20);
                    c.g[A1][0] = 0x2;
                    frame.save(c, 16, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_PLAYER_GET_ANIM_FIELD, 0x0012_ddb8)?;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x150).lw(c);
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let same = c.g[V0][0] == 0;
                    c.g[A2][0] = c.g[A1][0];
                    pc = if same { 0x0012_de48 } else { 0x0012_ddd0 };
                }
                0x0012_ddd0 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x175));
                    c.g[V0][0] = (c.g[V1][0] < 0xfu64) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x46_0000;
                    pc = if out { 0x0012_de4c } else { 0x0012_dde0 };
                }
                0x0012_dde0 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f60));
                    c.g[V1][0] =
                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match SWITCH_TO_ANIM_SET_A_IDS.iter().find(|e| e.0 == target) {
                        Some(e) => {
                            c.cycles += 2;
                            c.g[A1][0] = e.1;
                            0x0012_de4c
                        }
                        // The table's 0x17f entry keeps the id unchanged.
                        None if target == 0x0012_de4c => 0x0012_de4c,
                        None => {
                            c.jump_out(target, 0x0012_dd98, 0x0012_de80);
                            return Ok(());
                        }
                    };
                }
                0x0012_ddf8 | 0x0012_de00 | 0x0012_de08 | 0x0012_de10 | 0x0012_de18
                | 0x0012_de20 | 0x0012_de28 | 0x0012_de30 | 0x0012_de38 | 0x0012_de40 => {
                    c.cycles += 2;
                    let id = SWITCH_TO_ANIM_SET_A_IDS
                        .iter()
                        .find(|e| e.0 == pc)
                        .unwrap()
                        .1;
                    c.g[A1][0] = id;
                    pc = 0x0012_de4c;
                }
                0x0012_de48 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x174;
                    pc = 0x0012_de4c;
                }
                0x0012_de4c => {
                    c.cycles += 2;
                    let same = c.g[A1][0] == c.g[A2][0];
                    c.g[A2][0] = 0;
                    pc = if same { 0x0012_de70 } else { 0x0012_de54 };
                }
                0x0012_de54 => {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 12);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_CONTROLLER_SET_ANIM, 0x0012_de68)?;
                    c.g[A0][0] = c.g[S0][0];
                    callee::RIDER_WIPEOUT_UPDATE_ANIM_SPEED.call(c, 0x0012_de70)?;
                    pc = 0x0012_de70;
                }
                0x0012_de70 => {
                    c.cycles += 4;
                    let frame = Frame { size: 0x20 };
                    frame.restore(c, 16, 0x10);
                    frame.restore_ra(c, 0);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0012_dd98, 0x0012_de80);
                    return Ok(());
                }
            }
        }
    })
}

/// `RiderWipeoutSub_PlayEndAnimA` (0x12df48): pick the end-animation id for the
/// rider's current animation (table `.rodata 0x458110`) and, if it changes,
/// restart it at speed -1.
pub fn rider_wipeout_sub_play_end_anim_a(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0012_df48 => {
                    c.cycles += 13;
                    let frame = Frame::open(c, 0x20);
                    c.g[A1][0] = 0x2;
                    frame.save(c, 16, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_PLAYER_GET_ANIM_FIELD, 0x0012_df68)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16a));
                    c.g[V0][0] = (c.g[V1][0] < 0x1au64) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[A2][0] = c.g[A1][0];
                    pc = if out { 0x0012_dfe4 } else { 0x0012_df7c };
                }
                0x0012_df7c => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x46_0000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ef0));
                    c.g[V1][0] =
                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match target {
                        0x0012_df98 => 0x0012_df98,
                        0x0012_dfa0 => 0x0012_dfa0,
                        0x0012_dfa8 => 0x0012_dfa8,
                        0x0012_dfb0 => 0x0012_dfb0,
                        0x0012_dfb8 => 0x0012_dfb8,
                        0x0012_dfc0 => 0x0012_dfc0,
                        0x0012_dfc8 => 0x0012_dfc8,
                        0x0012_dfd0 => 0x0012_dfd0,
                        0x0012_dfd8 => 0x0012_dfd8,
                        0x0012_dfe0 => 0x0012_dfe0,
                        0x0012_dfe4 => 0x0012_dfe4,
                        _ => {
                            c.jump_out(target, 0x0012_df48, 0x0012_e010);
                            return Ok(());
                        }
                    };
                }
                0x0012_df98 | 0x0012_dfa0 | 0x0012_dfa8 | 0x0012_dfb0 | 0x0012_dfb8
                | 0x0012_dfc0 | 0x0012_dfc8 | 0x0012_dfd0 | 0x0012_dfd8 => {
                    c.cycles += 2;
                    let id = match pc {
                        0x0012_df98 => 0x185,
                        0x0012_dfa0 => 0x186,
                        0x0012_dfa8 => 0x187,
                        0x0012_dfb0 => 0x188,
                        0x0012_dfb8 => 0x189,
                        0x0012_dfc0 => 0x18a,
                        0x0012_dfc8 => 0x18b,
                        0x0012_dfd0 => 0x18c,
                        _ => 0x18d,
                    };
                    c.g[A1][0] = id;
                    pc = 0x0012_dfe4;
                }
                0x0012_dfe0 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x18e;
                    pc = 0x0012_dfe4;
                }
                0x0012_dfe4 => {
                    c.cycles += 2;
                    let same = c.g[A1][0] == c.g[A2][0];
                    c.g[A2][0] = 0;
                    pc = if same { 0x0012_e000 } else { 0x0012_dfec };
                }
                0x0012_dfec => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 12);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_CONTROLLER_SET_ANIM, 0x0012_e000)?;
                    pc = 0x0012_e000;
                }
                0x0012_e000 => {
                    c.cycles += 4;
                    let frame = Frame { size: 0x20 };
                    frame.restore(c, 16, 0x10);
                    frame.restore_ra(c, 0);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0012_df48, 0x0012_e010);
                    return Ok(());
                }
            }
        }
    })
}

/// The tumble-animation table (`.rodata 0x457fa0`): case block, animation id,
/// its cycle charge and the alternate table entry the ground flag selects.
const TUMBLE_ANIM_IDS: [(u32, u64, u64, u64); 33] = [
    (0x0012_dae8, 0x16a, 3, 0x175),
    (0x0012_daf4, 0x16b, 3, 0x176),
    (0x0012_db00, 0x16c, 3, 0x177),
    (0x0012_db0c, 0x170, 3, 0x17b),
    (0x0012_db18, 0x16e, 3, 0x179),
    (0x0012_db24, 0x16f, 3, 0x17a),
    (0x0012_db30, 0x172, 3, 0x17d),
    (0x0012_db3c, 0x173, 3, 0x17e),
    (0x0012_db48, 0x16d, 3, 0x178),
    (0x0012_db54, 0x171, 3, 0x17c),
    (0x0012_db60, 0x16a, 3, 0x175),
    (0x0012_db6c, 0x16b, 3, 0x176),
    (0x0012_db78, 0x16c, 3, 0x177),
    (0x0012_db84, 0x170, 3, 0x17b),
    (0x0012_db90, 0x16e, 3, 0x179),
    (0x0012_db9c, 0x173, 3, 0x17e),
    (0x0012_dba8, 0x171, 3, 0x17c),
    (0x0012_dbb4, 0x16d, 3, 0x178),
    (0x0012_dbc0, 0x16f, 3, 0x17a),
    (0x0012_dbcc, 0x172, 3, 0x17d),
    (0x0012_dbd8, 0x170, 3, 0x17b),
    (0x0012_dbe4, 0x171, 3, 0x17c),
    (0x0012_dbf0, 0x16c, 3, 0x177),
    (0x0012_dbfc, 0x16e, 3, 0x179),
    (0x0012_dc08, 0x173, 3, 0x17e),
    (0x0012_dc14, 0x170, 3, 0x17b),
    (0x0012_dc20, 0x170, 3, 0x17b),
    (0x0012_dc2c, 0x172, 3, 0x17d),
    (0x0012_dc38, 0x16b, 3, 0x176),
    (0x0012_dc44, 0x16a, 3, 0x175),
    (0x0012_dc50, 0x170, 3, 0x17b),
    (0x0012_dc5c, 0x172, 3, 0x17d),
    (0x0012_dc68, 0x174, 2, 0x17f),
];

/// `RiderWipeoutSub_SelectTumbleAnim` (0x12da88): choose the tumble animation
/// for the rider's current animation (table `.rodata 0x457fa0`) and, when it
/// changes, restart it at speed -1 and update the anim speed. `s0` records
/// whether the rider was on the ground (animation index 0), which selects the
/// second table column.
pub fn rider_wipeout_sub_select_tumble_anim(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0012_da88 => {
                    c.cycles += 17;
                    let frame = Frame::open(c, 0x30);
                    c.g[A1][0] = 0x2;
                    frame.save(c, 16, 0x20);
                    frame.save(c, 17, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A0][0];
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x80).lw(c);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x784).lw(c);
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    c.g[S0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    call_ptr(c, ANIM_PLAYER_GET_ANIM_FIELD, 0x0012_dab8)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x148));
                    c.g[V0][0] = (c.g[V1][0] < 0x22u64) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[A2][0] = c.g[A1][0];
                    pc = if out { 0x0012_dc74 } else { 0x0012_dacc };
                }
                0x0012_dacc => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x45_0000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7fa0));
                    c.g[V1][0] =
                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    if let Some(e) = TUMBLE_ANIM_IDS.iter().find(|e| e.0 == target) {
                        c.cycles += e.2;
                        c.g[A1][0] = e.1;
                        c.g[V0][0] = e.3;
                        pc = 0x0012_dc70;
                    } else {
                        c.jump_out(target, 0x0012_da88, 0x0012_dcac);
                        return Ok(());
                    }
                }
                0x0012_dae8 | 0x0012_daf4 | 0x0012_db00 | 0x0012_db0c | 0x0012_db18
                | 0x0012_db24 | 0x0012_db30 | 0x0012_db3c | 0x0012_db48 | 0x0012_db54
                | 0x0012_db60 | 0x0012_db6c | 0x0012_db78 | 0x0012_db84 | 0x0012_db90
                | 0x0012_db9c | 0x0012_dba8 | 0x0012_dbb4 | 0x0012_dbc0 | 0x0012_dbcc
                | 0x0012_dbd8 | 0x0012_dbe4 | 0x0012_dbf0 | 0x0012_dbfc | 0x0012_dc08
                | 0x0012_dc14 | 0x0012_dc20 | 0x0012_dc2c | 0x0012_dc38 | 0x0012_dc44
                | 0x0012_dc50 | 0x0012_dc5c | 0x0012_dc68 => {
                    let e = TUMBLE_ANIM_IDS
                        .iter()
                        .find(|e| e.0 == pc)
                        .unwrap();
                    c.cycles += e.2;
                    c.g[A1][0] = e.1;
                    c.g[V0][0] = e.3;
                    pc = 0x0012_dc70;
                }
                0x0012_dc70 => {
                    c.cycles += 1;
                    if c.g[S0][0] != 0 {
                        c.g[A1][0] = c.g[V0][0];
                    }
                    pc = 0x0012_dc74;
                }
                0x0012_dc74 => {
                    c.cycles += 2;
                    let same = c.g[A1][0] == c.g[A2][0];
                    c.g[A2][0] = 0;
                    pc = if same { 0x0012_dc98 } else { 0x0012_dc7c };
                }
                0x0012_dc7c => {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x80).lw(c);
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 12);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_CONTROLLER_SET_ANIM, 0x0012_dc90)?;
                    c.g[A0][0] = c.g[S1][0];
                    callee::RIDER_WIPEOUT_UPDATE_ANIM_SPEED.call(c, 0x0012_dc98)?;
                    pc = 0x0012_dc98;
                }
                0x0012_dc98 => {
                    c.cycles += 5;
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, 16, 0x20);
                    frame.restore(c, 17, 0x10);
                    frame.restore_ra(c, 0);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0012_da88, 0x0012_dcac);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// SPCH_NotifyPartCallback: walk a bank's part list, find the instrument that
// matches, and hand it to the bank's callback.

/// `void SPCH_NotifyPartCallback(void* self, SPCH_Bank* bank, int index, void* callbackTable)`
/// (0x3db040): the bank descriptor at `bank` lists `bank[7] & 0xf` parts
/// starting at `bank + ((bank[6] << 1) + 3 & 0x3fc) + 0xc`; for each part whose
/// instrument matches, call the bank callback (`callbackTable[instrument]`)
/// with the part's data. The port accepts all `__entries` resume labels and
/// delivers loop events at the same heads.
pub fn spch_notify_part_callback(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x003d_b040 => {
                    c.cycles += 18;
                    let frame = Frame::open(c, 0xa0);
                    c.g[V0][0] = 0x45_0000;
                    frame.save(c, 22, 0x80);
                    frame.save(c, 21, 0x70);
                    c.g[S6][0] = c.g[V0][0];
                    frame.save(c, 19, 0x50);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x70));
                    frame.save(c, 17, 0x30);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, 16, 0x20);
                    c.g[S1][0] = c.g[A2][0];
                    frame.save(c, 31, 0x90);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, 20, 0x60);
                    frame.save(c, 18, 0x40);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
                    c.g[S5][0] = c.g[A3][0];
                    pc = if c.g[V1][0] == 0 { 0x003d_b1dc } else { 0x003d_b088 };
                }
                0x003d_b088 => {
                    c.cycles += 17;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    callee::SPCH_FIND_BANK_TYPE.call(c, 0x003d_b090)?;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x6).lbu(c);
                    c.g[V0][0] = sx((c.g[S1][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x7).lbu(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0xc).lhu(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3));
                    c.g[S2][0] = c.g[A1][0] & 0xf;
                    c.g[V1][0] = c.g[V1][0] & 0x3fc;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let done = c.g[S2][0] == 0;
                    c.g[A2][0] = 0;
                    pc = if done { 0x003d_b1dc } else { 0x003d_b0cc };
                }
                0x003d_b0cc => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 1);
                    pc = 0x003d_b0d0;
                }
                0x003d_b0d0 => {
                    c.cycles += 11;
                    c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x2).lbu(c);
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                    c.g[T1][0] = Addr(c.g[V0][0] as u32).off(0x1).lbu(c);
                    c.g[V1][0] = (c.g[A0][0] < 0x6u64) as u64;
                    let sp0 = c.g[SP][0] as u32;
                    Addr(sp0).sw(c, c.g[A1][0]);
                    Addr(sp0.wrapping_add(0x4)).sw(c, c.g[T1][0]);
                    let skip = c.g[V1][0] == 0;
                    Addr(sp0.wrapping_add(0x8)).sw(c, c.g[A0][0]);
                    pc = if skip { 0x003d_b1cc } else { 0x003d_b0fc };
                }
                0x003d_b0fc => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x49_0000;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5ba0));
                    c.g[V1][0] =
                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match target {
                        0x003d_b118 => 0x003d_b118,
                        0x003d_b1cc => 0x003d_b1cc,
                        _ => {
                            c.jump_out(target, 0x003d_b040, 0x003d_b204);
                            return Ok(());
                        }
                    };
                }
                0x003d_b118 => {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x4).lbu(c);
                    c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = 0;
                    c.g[T2][0] = sx((c.g[V0][0] as u32) >> 2);
                    let none = c.g[T2][0] == 0;
                    c.g[T0][0] = 0;
                    pc = if none { 0x003d_b1a8 } else { 0x003d_b130 };
                }
                0x003d_b130 => {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x8).lbu(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A2][0] =
                        sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x2).lbu(c);
                    let found = c.g[V1][0] == c.g[T1][0];
                    pc = if found {
                        c.g[T0][0] = 1;
                        0x003d_b1a8
                    } else {
                        0x003d_b148
                    };
                }
                0x003d_b148 => {
                    c.cycles += 2;
                    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    pc = 0x003d_b150;
                }
                0x003d_b150 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x8).lbu(c);
                    let hit = c.g[V0][0] == c.g[T1][0];
                    c.g[A0][0] = 0;
                    pc = if hit { 0x003d_b178 } else { 0x003d_b15c };
                }
                0x003d_b15c => {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    pc = 0x003d_b160;
                }
                0x003d_b160 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 4i64) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[V0][0] =
                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    pc = if out { 0x003d_b17c } else { 0x003d_b16c };
                }
                0x003d_b16c => {
                    c.cycles += 3;
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lbu(c);
                    let more = c.g[V1][0] != c.g[T1][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if more {
                        if back_edge_unwinds(c, 0x003d_b160) {
                            return Ok(());
                        }
                        pc = 0x003d_b160;
                    } else {
                        pc = 0x003d_b178;
                    }
                }
                0x003d_b178 => {
                    c.cycles += 1;
                    c.g[T0][0] = 1;
                    pc = 0x003d_b17c;
                }
                0x003d_b17c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T2][0] as i64)) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[A3][0];
                    pc = if out { 0x003d_b1a8 } else { 0x003d_b188 };
                }
                0x003d_b188 => {
                    c.cycles += 7;
                    c.g[V0][0] =
                        sx((c.g[S1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lbu(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A2][0] =
                        sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x2).lbu(c);
                    let more = c.g[V0][0] != c.g[T1][0];
                    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if more {
                        if back_edge_unwinds(c, 0x003d_b150) {
                            return Ok(());
                        }
                        pc = 0x003d_b150;
                    } else {
                        pc = 0x003d_b1a4;
                    }
                }
                0x003d_b1a4 => {
                    c.cycles += 1;
                    c.g[T0][0] = 1;
                    pc = 0x003d_b1a8;
                }
                0x003d_b1a8 => {
                    c.cycles += 2;
                    let skip = c.g[T0][0] == 0;
                    c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
                    pc = if skip { 0x003d_b1cc } else { 0x003d_b1b0 };
                }
                0x003d_b1b0 => {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x70));
                    c.g[V0][0] =
                        sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[T0][0] = Addr(c.g[V1][0] as u32).off(0x8).lw(c);
                    c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A0][0] = c.g[S3][0];
                    let target = c.g[T0][0] as u32;
                    let sp0 = c.g[SP][0] as u32;
                    c.g[A3][0] = Addr(sp0).off(0x10).lw(c);
                    call_ptr(c, target, 0x003d_b1cc)?;
                    pc = 0x003d_b1cc;
                }
                0x003d_b1cc => {
                    c.cycles += 4;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 1);
                    if more {
                        if back_edge_unwinds(c, 0x003d_b0d0) {
                            return Ok(());
                        }
                        pc = 0x003d_b0d0;
                    } else {
                        pc = 0x003d_b1dc;
                    }
                }
                0x003d_b1dc => {
                    c.cycles += 10;
                    let frame = Frame { size: 0xa0 };
                    frame.restore(c, 31, 0x90);
                    frame.restore(c, 22, 0x80);
                    frame.restore(c, 21, 0x70);
                    frame.restore(c, 20, 0x60);
                    frame.restore(c, 19, 0x50);
                    frame.restore(c, 18, 0x40);
                    frame.restore(c, 17, 0x30);
                    frame.restore(c, 16, 0x20);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d_b040, 0x003d_b204);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, vtable, LOG_UNWIND_AT};
    use crate::difftest::{check, elf_image, put_words, scramble, Rng, DATA, GAME_GP, SP as STACK};

    const SEEDS: u32 = 200;

    fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 8) ^ salt)
    }

    /// Scrambled registers, game `gp`, a zeroed data area, and four pointers
    /// into it. Enough for the pure leaves (loads/stores within `DATA`).
    fn leaf_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = rng(seed, salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        for i in 0..0x400 {
            c.bus.write32(DATA + i * 4, r.next_u32());
        }
        for i in 0..4 {
            c.g[[A0, A1, A2, A3][i]][0] = (DATA + (r.next_u32() & 0x3f0)) as u64;
        }
    }

    fn leaf(gen: GuestFn, hand: GuestFn, salt: u64) {
        check(gen, hand, SEEDS, &|c, seed| leaf_setup(c, seed, salt));
    }

    /// A test machine with fake call/jump-out/syscall hooks.
    fn fake_check(gen: GuestFn, hand: GuestFn, salt: u64, setup: &dyn Fn(&mut Cpu, u32, &mut Rng)) {
        check(gen, hand, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            setup(c, seed, &mut r);
        });
    }

    /// [`check`] plus soft state and LO/HI: the replacement reproduces every
    /// register write of the generated body, so the caller-saved temporaries
    /// and the multiply pipeline must match too.
    fn strict_check(gen: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = crate::difftest::compare(gen, hand, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard {:#?} soft {:#?} lohi {:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    fn syscall_test(gen: GuestFn, hand: GuestFn, salt: u64) {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, 1);
            for i in 0..4 {
                c.g[[A0, A1, A2, A3][i]][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
            }
        };
        covered(gen, hand, SEEDS, &setup);
    }

    // ------------------------------------------------------------- leaves

    #[test]
    fn rider_base_render_cheat_code_matches() {
        leaf(ssx3_ee::s09::f_002f6908__generated, rider_base_render_cheat_code, 1);
    }

    #[test]
    fn rider_empty_progress_hook_matches() {
        leaf(ssx3_ee::s00::f_00125ad0__generated, rider_empty_progress_hook, 2);
    }

    #[test]
    fn rider_return_zero_25b8_matches() {
        leaf(ssx3_ee::s00::f_001125b8__generated, rider_return_zero_25b8, 3);
    }

    #[test]
    fn snd_tick_stub_matches() {
        leaf(ssx3_ee::s13::f_003c2438__generated, snd_tick_stub, 4);
    }

    #[test]
    fn tex_man_end_frame_stub_matches() {
        leaf(ssx3_ee::s12::f_003673d0__generated, tex_man_end_frame_stub, 5);
    }

    #[test]
    fn intc_table_end_frame_stub_matches() {
        leaf(ssx3_ee::s12::f_00361f90__generated, intc_table_end_frame_stub, 6);
    }

    #[test]
    fn stats_process_render_stub_matches() {
        leaf(ssx3_ee::s09::f_002d21b8__generated, stats_process_render_stub, 7);
    }

    #[test]
    fn stats_process_update_stub_matches() {
        leaf(ssx3_ee::s09::f_002d21b0__generated, stats_process_update_stub, 8);
    }

    #[test]
    fn nash_empty_hook_matches() {
        leaf(ssx3_ee::s05::f_00229f80__generated, nash_empty_hook, 9);
    }

    #[test]
    fn replay_snapshot_empty_hook_a_matches() {
        leaf(ssx3_ee::s07::f_0026ed88__generated, replay_snapshot_empty_hook_a, 10);
    }

    #[test]
    fn rider_air_state_exit_matches() {
        leaf(ssx3_ee::s01::f_00139a18__generated, rider_air_state_exit, 11);
    }

    #[test]
    fn rider_prewind_sub_exit_matches() {
        leaf(ssx3_ee::s00::f_0012e9b0__generated, rider_prewind_sub_exit, 12);
    }

    #[test]
    fn rider_empty_stance_hook_matches() {
        leaf(ssx3_ee::s00::f_00116930__generated, rider_empty_stance_hook, 13);
    }

    #[test]
    fn replay_empty_rider_hook_matches() {
        leaf(ssx3_ee::s07::f_002705a8__generated, replay_empty_rider_hook, 14);
    }

    #[test]
    fn rider_wipeout_state_exit_matches() {
        leaf(ssx3_ee::s01::f_00136f28__generated, rider_wipeout_state_exit, 15);
    }

    #[test]
    fn audio_get_frame_rate_matches() {
        leaf(ssx3_ee::s07::f_0028b240__generated, audio_get_frame_rate, 16);
    }

    #[test]
    fn luno_vm_register_set_raw_matches() {
        leaf(ssx3_ee::s05::f_00226600__generated, luno_vm_register_set_raw, 17);
    }

    #[test]
    fn luno_vm_pass_through_matches() {
        leaf(ssx3_ee::s05::f_002224a8__generated, luno_vm_pass_through, 18);
    }

    #[test]
    fn luno_vm_register_as_float_matches() {
        leaf(ssx3_ee::s05::f_00226618__generated, luno_vm_register_as_float, 19);
    }

    #[test]
    fn luno_vm_register_as_int_matches() {
        leaf(ssx3_ee::s05::f_00226610__generated, luno_vm_register_as_int, 20);
    }

    #[test]
    fn board_spark_emitter_ctor_matches() {
        leaf(ssx3_ee::s09::f_002f7a68__generated, board_spark_emitter_ctor, 21);
    }

    #[test]
    fn air_predictor_clear_air_time_matches() {
        leaf(ssx3_ee::s00::f_00113998__generated, air_predictor_clear_air_time, 22);
    }

    #[test]
    fn stream_buffer_set_base_volume_matches() {
        leaf(ssx3_ee::s08::f_002a9f30__generated, stream_buffer_set_base_volume, 23);
    }

    #[test]
    fn bank_monitor_timer_disable_matches() {
        leaf(ssx3_ee::s08::f_002ad990__generated, bank_monitor_timer_disable, 24);
    }

    #[test]
    fn snd_dsp_resample_set_rate_matches() {
        leaf(ssx3_ee::s14::f_003cb538__generated, snd_dsp_resample_set_rate, 25);
    }

    #[test]
    fn snd_utk_set_mode_matches() {
        leaf(ssx3_ee::s14::f_003ce0d8__generated, snd_utk_set_mode, 26);
    }

    #[test]
    fn sndstrm_stop_slot_unsupported_matches() {
        leaf(ssx3_ee::s13::f_003c6ce0__generated, sndstrm_stop_slot_unsupported, 27);
    }

    #[test]
    fn snd_eaxa_set_history_matches() {
        leaf(ssx3_ee::s14::f_003cd190__generated, snd_eaxa_set_history, 28);
    }

    #[test]
    fn sound_request_set_bank_sample_matches() {
        leaf(ssx3_ee::s08::f_002abce8__generated, sound_request_set_bank_sample, 29);
    }

    #[test]
    fn multi_spline_modifier_release_matches() {
        leaf(ssx3_ee::s11::f_0035aae0__generated, multi_spline_modifier_release, 30);
    }

    #[test]
    fn rider_metrix_get_trick_points_matches() {
        leaf(ssx3_ee::s00::f_00117900__generated, rider_metrix_get_trick_points, 31);
    }

    #[test]
    fn world_get_table_entry_matches() {
        check(ssx3_ee::s13::f_003a6d00__generated, world_get_table_entry, SEEDS, &|c, seed| {
            let mut r = rng(seed, 32);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(DATA, DATA + 0x800);
            for i in 0..0x400 {
                c.bus.write32(DATA + 0x800 + i * 4, r.next_u32());
            }
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() & 0x3ff) as u64;
        });
    }

    #[test]
    fn world_get_object_section_data_matches() {
        check(ssx3_ee::s13::f_003a6cc8__generated, world_get_object_section_data, SEEDS, &|c, seed| {
            let mut r = rng(seed, 33);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(DATA + 0x88, DATA + 0x200);
            c.bus.write32(DATA + 0x20c, DATA + 0x300);
            c.bus.write32(DATA + 0x308, r.next_u32());
            c.g[A1][0] = DATA as u64;
        });
    }

    // ------------------------------------------------------------ syscalls

    #[test]
    fn sce_sif_set_dma_matches() {
        syscall_test(ssx3_ee::s15::f_00424160__generated, sce_sif_set_dma, 34);
    }

    #[test]
    fn isce_sif_set_dchain_matches() {
        syscall_test(ssx3_ee::s15::f_00424190__generated, isce_sif_set_dchain, 35);
    }

    #[test]
    fn flush_cache_matches() {
        syscall_test(ssx3_ee::s15::f_00424020__generated, flush_cache, 36);
    }

    #[test]
    fn poll_sema_matches() {
        syscall_test(ssx3_ee::s15::f_00423df0__generated, poll_sema, 37);
    }

    #[test]
    fn sce_sif_dma_stat_matches() {
        syscall_test(ssx3_ee::s15::f_00424140__generated, sce_sif_dma_stat, 38);
    }

    // ---------------------------------------------------- Nash_ClassifyEffect

    #[test]
    fn nash_classify_effect_matches() {
        fake_check(ssx3_ee::s05::f_0022a270__generated, nash_classify_effect, 39, &|c, seed, r| {
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { (DATA + 0x100) as u64 };
            // VI1 selects the effect-kind table entry (the code moves it into
            // a0 with cfc2).
            c.bus.plat.write_ctrl(1, seed % 5);
            for i in 0..0x400 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
            // The effect-kind table (`.rodata 0x47b380`).
            put_words(
                c,
                0x0047_b380,
                &[0x0022_a30c, 0x0022_a358, 0x0022_a358, 0x0022_a30c, 0x0022_a334],
            );
            // Registration heads/counts for the list-append branches.
            c.bus.write32(DATA + 0x7ab8, if seed % 2 == 0 { 0 } else { DATA });
            c.bus.write32(DATA + 0x78b4, r.next_u32() & 3);
            c.bus.write32(DATA + 0x7abc, r.next_u32() & 3);
        });
    }

    // ------------------------------------------------------ audio mask tables

    #[test]
    fn character_to_speech_mask_matches() {
        fake_check(ssx3_ee::s08::f_0029f2b0__generated, character_to_speech_mask, 40, &|c, seed, r| {
            put_words(
                c,
                0x0048_2e60,
                &[
                    0x0029_f2e8, 0x0029_f2f8, 0x0029_f320, 0x0029_f2e0, 0x0029_f300,
                    0x0029_f318, 0x0029_f2d8, 0x0029_f310, 0x0029_f2f0, 0x0029_f308,
                ],
            );
            let idx = (r.next_u32() % 10) as u32;
            match seed % 5 {
                0 => {
                    c.g[A0][0] = idx as u64;
                    c.g[A1][0] = (r.next_u32() | 1) as u64;
                }
                1 => {
                    c.g[A0][0] = (10 + r.next_u32() % 10) as u64;
                    c.g[A1][0] = 0;
                }
                2 => {
                    c.g[A0][0] = idx as u64;
                    c.g[A1][0] = 0;
                    c.bus.write32(0x0048_2e60 + 4 * idx, 0xdead_beef);
                }
                _ => {
                    c.g[A0][0] = idx as u64;
                    c.g[A1][0] = 0;
                }
            }
        });
    }

    #[test]
    fn character_to_voice_mask_matches() {
        fake_check(ssx3_ee::s08::f_002a1da0__generated, character_to_voice_mask, 41, &|c, seed, r| {
            put_words(
                c,
                0x0048_2fe0,
                &[
                    0x002a_1dd8, 0x002a_1de8, 0x002a_1e10, 0x002a_1dd0, 0x002a_1df0,
                    0x002a_1e08, 0x002a_1dc8, 0x002a_1e00, 0x002a_1de0, 0x002a_1df8,
                ],
            );
            let idx = (r.next_u32() % 10) as u32;
            match seed % 5 {
                0 => {
                    c.g[A1][0] = idx as u64;
                    c.g[A2][0] = (r.next_u32() | 1) as u64;
                }
                1 => {
                    c.g[A1][0] = (10 + r.next_u32() % 10) as u64;
                    c.g[A2][0] = 0;
                }
                2 => {
                    c.g[A1][0] = idx as u64;
                    c.g[A2][0] = 0;
                    c.bus.write32(0x0048_2fe0 + 4 * idx, 0xdead_beef);
                }
                _ => {
                    c.g[A1][0] = idx as u64;
                    c.g[A2][0] = 0;
                }
            }
        });
    }

    // ------------------------------------------- RiderBase enter/exit methods

    /// `RiderBase_EnterMainState`/`ExitMainState` fixture: the real dispatch
    /// tables (`.rodata 0x456b30`/`0x456b10`, loaded from the ELF image) pick
    /// the arm from the state word at `owner + 0xde0`; a zeroed owner plus a
    /// zeroed world and back-end singleton keep the real state methods on
    /// their short paths. States 0..5 are all six table entries (in the real
    /// order 0 ground, 1 air, 2 wipeout, 3 base, 4 railslide, 5 handplant);
    /// the others are the `state >= 6` fall-through to the base arm.
    fn state_fixture(c: &mut Cpu, seed: u32, salt: u64, table: u32) {
        machine(c, seed, salt, 1);
        elf_image(c);
        c.g[SP][0] = (STACK - 0x200) as u64;
        for i in 0..0x400 {
            c.bus.write32(DATA + i * 4, 0);
        }
        c.bus.write32(GAME_GP - 0x848, DATA + 0x800); // world
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x900); // BELibrary singleton
        c.bus.write32(GAME_GP - 0x1f28, DATA + 0xa00);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        let state = match seed % 10 {
            0..=5 => seed % 10,
            6 => 6,
            7 => 0x10,
            8 => 0xffff_ffff,
            _ => 0x100,
        };
        c.g[A0][0] = DATA as u64;
        c.bus.write32(DATA + 0xde0, state);
        // The fixture really selects an arm from the game's table, not zero
        // RAM: every in-range state has a non-zero case label there.
        if state < 6 {
            assert_ne!(c.bus.read32(table + state * 4), 0, "state table {table:#x} not loaded");
        }
    }

    #[test]
    fn rider_base_enter_main_state_matches() {
        check(
            ssx3_ee::s00::f_00111380__generated,
            rider_base_enter_main_state,
            30,
            &|c, seed| state_fixture(c, seed, 45, 0x0045_6b30),
        );
    }

    #[test]
    fn rider_base_exit_main_state_matches() {
        check(
            ssx3_ee::s00::f_001112f8__generated,
            rider_base_exit_main_state,
            30,
            &|c, seed| state_fixture(c, seed, 46, 0x0045_6b10),
        );
    }

    // -------------------------------------------------- EAFile_ReleaseRequest

    #[test]
    fn ea_file_release_request_matches() {
        fake_check(ssx3_ee::s14::f_003dd310__generated, ea_file_release_request, 44, &|c, _seed, r| {
            c.g[A0][0] = DATA as u64; // its top byte is 0
            c.g[SP][0] = (STACK - 0x100) as u64;
            // The hash table entry for top byte 0 points at the request.
            c.bus.write32(0x0051_ad98, DATA);
            // A positive state that equals itself in the compare.
            c.bus.write32(DATA + 8, 1);
            // Request flags: channel class 2..10, so the dispatch is taken.
            let lane = 2 + r.next_u32() % 9;
            c.bus.write32(DATA, (lane << 20) | (r.next_u32() & 0x000f_ffff));
            // The state table (0x495d40) stays zero: default jump_out.
        });
    }

    // -------------------------------------------------------------- RiderAnim

    /// `RiderAnim_v03` fixture. The real descriptor table at `.rodata
    /// 0x446990` (28-byte entries, loaded from the ELF image) supplies the
    /// sequence-end kind at `entry + 8`; `*out` is the entry index. These
    /// indices have kinds 0..10 and one whose kind is >= 0xb (the epilogue
    /// fall-through). The kind indexes the real handler table at 0x456990.
    const ANIM_ENTRY_FOR_KIND: [u32; 12] = [0, 27, 269, 21, 2, 71, 49, 4, 220, 23, 435, 437];

    fn anim_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 1);
        elf_image(c);
        c.g[SP][0] = (STACK - 0x200) as u64;
        for i in 0..0x100 {
            c.bus.write32(DATA + i * 4, 0);
        }
        c.g[A0][0] = DATA as u64; // RiderAnim
        c.g[A2][0] = (DATA + 0x100) as u64; // out: *out = descriptor index
        c.bus.write32(DATA + 0x50, 0x1000); // first sequence-end address
        let k = (seed % 12) as usize;
        let idx = ANIM_ENTRY_FOR_KIND[k];
        c.bus.write32(DATA + 0x100, idx);
        // The real descriptor really has the kind that selects this arm (and
        // the last one is out of the handler table's range).
        let kind = c.bus.read32(0x0044_6990 + idx * 0x1c + 8);
        assert!(
            if k < 11 { kind == k as u32 } else { kind >= 0xb },
            "descriptor {idx} has kind {kind}, wanted {k}"
        );
        // Sequence-end address: the slot written is (seqEnd - 0x1000) >> 3.
        c.g[A1][0] = (0x1000 + (seed % 4) * 8) as u64;
    }

    #[test]
    fn rider_anim_v03_matches() {
        check(
            ssx3_ee::s00::f_00103918__generated,
            rider_anim_v03,
            36,
            &|c, seed| anim_fixture(c, seed, 47),
        );
    }

    // --------------------------------------------------------- SPCH callback

    #[test]
    fn spch_notify_part_callback_matches() {
        fake_check(ssx3_ee::s14::f_003db040__generated, spch_notify_part_callback, 46, &|c, seed, r| {
            c.g[SP][0] = (STACK - 0x100) as u64;
            c.g[A0][0] = (DATA + 0x200) as u64;
            c.g[A1][0] = DATA as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = (DATA + 0x2000) as u64;
            // A function pointer at the callback slot (0x44ff98).
            c.bus.write32(0x0044_ff98, 0x4000);
            // The instrument-class table (`.rodata 0x495ba0`).
            put_words(
                c,
                0x0049_5ba0,
                &[0x003d_b118, 0x003d_b1cc, 0x003d_b1cc, 0x003d_b118, 0x003d_b1cc, 0x003d_b1cc],
            );
            // Bank descriptor at DATA.
            c.bus.write8(DATA + 0x6, 0);
            c.bus.write8(DATA + 0x7, 1);
            c.bus.write8(DATA + 0x2, 0);
            c.bus.write8(DATA + 0x4, if seed % 2 == 0 { 0 } else { 4 });
            c.bus.write8(DATA + 0x8, 0);
            c.bus.write16(DATA + 0xc, 0);
            c.bus.write8(DATA + 0xc, 0);
            c.bus.write8(DATA + 0xd, 0);
            c.bus.write8(DATA + 0xe, (r.next_u32() % 6) as u8);
            c.bus.write32(DATA + 0x2000, 0x1234);
        });
    }

    // ------------------------------------------ RiderWipeoutSub anim selectors

    fn anim_object(c: &mut Cpu, id: u32) {
        c.g[SP][0] = (STACK - 0x100) as u64;
        c.bus.write32(DATA + 0x80, DATA + 0x200);
        c.bus.write32(DATA + 0x984, DATA + 0x300);
        c.bus.write32(DATA + 0x308, id);
        c.bus.write32(DATA, 0);
    }

        #[test]
    fn rider_wipeout_sub_play_end_anim_a_matches() {
        fake_check(ssx3_ee::s00::f_0012df48__generated, rider_wipeout_sub_play_end_anim_a, 48, &|c, seed, _r| {
            let arms = [
                0x0012_df98, 0x0012_dfa0, 0x0012_dfa8, 0x0012_dfb0, 0x0012_dfb8,
                0x0012_dfc0, 0x0012_dfc8, 0x0012_dfd0, 0x0012_dfd8, 0x0012_dfe0,
            ];
            let ids = [
                0x185, 0x186, 0x187, 0x188, 0x189, 0x18a, 0x18b, 0x18c, 0x18d, 0x18e,
            ];
            match seed % 3 {
                0 => {
                    // In range; the (empty) end-anim table sends it to jump_out.
                    anim_object(c, 0x16a);
                    c.g[A0][0] = DATA as u64;
                }
                1 => {
                    // Out of table range and equal ids: straight to epilogue.
                    anim_object(c, 0);
                    c.g[A0][0] = DATA as u64;
                }
                _ => {
                    // Resume at a case block with the same id already playing.
                    let k = (seed as usize / 3) % arms.len();
                    c.pc = arms[k];
                    c.g[A2][0] = ids[k];
                }
            }
        });
    }

    #[test]
    fn rider_wipeout_sub_select_tumble_anim_matches() {
        fake_check(ssx3_ee::s00::f_0012da88__generated, rider_wipeout_sub_select_tumble_anim, 49, &|c, seed, _r| {
            let arm = [
                0x0012_dae8, 0x0012_daf4, 0x0012_db00, 0x0012_db0c, 0x0012_db18,
                0x0012_db24, 0x0012_db30, 0x0012_db3c, 0x0012_db48, 0x0012_db54,
                0x0012_db60, 0x0012_db6c, 0x0012_db78, 0x0012_db84, 0x0012_db90,
                0x0012_db9c, 0x0012_dba8, 0x0012_dbb4, 0x0012_dbc0, 0x0012_dbcc,
                0x0012_dbd8, 0x0012_dbe4, 0x0012_dbf0, 0x0012_dbfc, 0x0012_dc08,
                0x0012_dc14, 0x0012_dc20, 0x0012_dc2c, 0x0012_dc38, 0x0012_dc44,
                0x0012_dc50, 0x0012_dc5c, 0x0012_dc68,
            ];
            let a1 = [
                0x16a, 0x16b, 0x16c, 0x170, 0x16e, 0x16f, 0x172, 0x173, 0x16d, 0x171,
                0x16a, 0x16b, 0x16c, 0x170, 0x16e, 0x173, 0x171, 0x16d, 0x16f, 0x172,
                0x170, 0x171, 0x16c, 0x16e, 0x173, 0x170, 0x170, 0x172, 0x16b, 0x16a,
                0x170, 0x172, 0x174,
            ];
            match seed % 3 {
                0 => {
                    // Anim id 0 has a non-empty alternate column: dispatch to
                    // the (empty) table -> jump_out.
                    anim_object(c, 0x148);
                    c.g[A0][0] = DATA as u64;
                }
                1 => {
                    // Out of table range and equal ids: straight to epilogue.
                    anim_object(c, 0);
                    c.g[A0][0] = DATA as u64;
                }
                _ => {
                    let k = (seed as usize / 3) % arm.len();
                    c.pc = arm[k];
                    c.g[S0][0] = 0; // keep the primary id
                    c.g[A2][0] = a1[k];
                }
            }
        });
    }

    #[test]
    fn rider_wipeout_sub_switch_to_anim_set_a_matches() {
        strict_check(ssx3_ee::s00::f_0012dd98__generated, rider_wipeout_sub_switch_to_anim_set_a, 100, &|c, seed| {
            let mut r = machine(c, seed, 51, 1);
            c.pc = 0x0012_dd98;
            // `w + 0x80` -> anim player, its `+0x784` the layer controller and
            // its `+0x150` the "this animation is already playing" flag.
            c.bus.write32(DATA + 0x350, if seed % 3 == 0 { 1 } else { 0 });
            let id = match seed % 4 {
                0 => 0x174,
                1 => 0x175 + (seed as u32 % 0xf),
                2 => 0x148,
                _ => r.next_u32() % 0x200,
            };
            anim_object(c, id);
            c.g[A0][0] = sx(DATA);
            // The animation-id table the dispatch reads (`.rodata 0x4580a0`):
            // ten id blocks, then the "keep the id" entry for 0x17f and the
            // duplicate blocks for 0x180..0x183.
            put_words(
                c,
                0x0045_80a0,
                &[
                    0x0012_ddf8, 0x0012_de00, 0x0012_de08, 0x0012_de10, 0x0012_de18,
                    0x0012_de20, 0x0012_de28, 0x0012_de30, 0x0012_de38, 0x0012_de40,
                    0x0012_de4c, 0x0012_de28, 0x0012_de30, 0x0012_de38, 0x0012_de40,
                ],
            );
            // A restart calls AnimController_PlayAnim, whose first act is the
            // layer getter through the controller's vtable (a fake). Unwinding
            // there keeps the animation cache and the DLHeap out of the test,
            // so a free/consolidate can never run on fixture data.
            let vt = vtable(c, &mut r, &[0x20]);
            c.bus.write32(DATA + 0x300 + 0x58, vt);
            c.bus.write32(LOG_UNWIND_AT, 0);
        });
    }
}
