//! Staging module oc1509 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1509`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_47f8,
        script_engine_seq_listener_v05_002847f8,
        "ScriptEngine_SeqListener_v05",
    ),
    (
        0x0028_4800,
        script_engine_seq_listener_v07_00284800,
        "ScriptEngine_SeqListener_v07",
    ),
    (
        0x0028_4808,
        script_engine_seq_listener_v08_00284808,
        "ScriptEngine_SeqListener_v08",
    ),
    (
        0x0028_4810,
        script_engine_seq_listener_v11_00284810,
        "ScriptEngine_SeqListener_v11",
    ),
    (
        0x0028_4820,
        script_engine_script_db_load_bank_00284820,
        "ScriptEngine_ScriptDbLoadBank",
    ),
    (
        0x0028_4828,
        script_engine_script_db_cancel_bank_00284828,
        "ScriptEngine_ScriptDbCancelBank",
    ),
    (
        0x0028_4830,
        script_engine_script_db_unload_bank_00284830,
        "ScriptEngine_ScriptDbUnloadBank",
    ),
    (
        0x0028_4838,
        script_engine_script_db_v10_00284838,
        "ScriptEngine_ScriptDb_v10",
    ),
    (
        0x0028_4840,
        script_engine_script_db_v11_00284840,
        "ScriptEngine_ScriptDb_v11",
    ),
    (
        0x0028_4848,
        script_engine_script_db_v12_00284848,
        "ScriptEngine_ScriptDb_v12",
    ),
    (
        0x0028_4878,
        script_binding_apply_00284878,
        "ScriptBinding_Apply",
    ),
    (
        0x0028_4880,
        script_binding_tick_00284880,
        "ScriptBinding_Tick",
    ),
    (
        0x0028_4888,
        script_binding_accept_command_00284888,
        "ScriptBinding_AcceptCommand",
    ),
    (
        0x0028_4940,
        script_dummy_obj_get_binding_count_00284940,
        "ScriptDummyObj_GetBindingCount",
    ),
    (
        0x0028_4948,
        script_dummy_obj_get_binding_00284948,
        "ScriptDummyObj_GetBinding",
    ),
    (
        0x0028_49b8,
        script_audio_obj_get_binding_count_002849b8,
        "ScriptAudioObj_GetBindingCount",
    ),
    (
        0x0028_49c0,
        script_audio_obj_get_binding_002849c0,
        "ScriptAudioObj_GetBinding",
    ),
    (
        0x0028_4a88,
        script_obj_cam_get_binding_count_00284a88,
        "ScriptObjCam_GetBindingCount",
    ),
    (
        0x0028_4a90,
        script_obj_cam_get_binding_00284a90,
        "ScriptObjCam_GetBinding",
    ),
    (
        0x0028_4ad8,
        c_transition_man_script_get_binding_count_00284ad8,
        "cTransitionMan_ScriptGetBindingCount",
    ),
    (
        0x0028_4ae0,
        c_transition_man_script_get_binding_00284ae0,
        "cTransitionMan_ScriptGetBinding",
    ),
    (
        0x0028_4b50,
        rider_script_obj_get_binding_count_00284b50,
        "RiderScriptObj_GetBindingCount",
    ),
    (
        0x0028_4b58,
        rider_script_obj_get_binding_00284b58,
        "RiderScriptObj_GetBinding",
    ),
    (
        0x0029_1430,
        ssx_audio_system_update_ambience_loop_00291430,
        "SSXAudioSystem_UpdateAmbienceLoop",
    ),
    (
        0x0029_6e18,
        ssx_audio_system_update_wipeout_loop_00296e18,
        "SSXAudioSystem_UpdateWipeoutLoop",
    ),
    (
        0x0029_7430,
        ssx_audio_system_update_wipeout_loop2_00297430,
        "SSXAudioSystem_UpdateWipeoutLoop2",
    ),
    (
        0x0029_8d88,
        ssx_audio_system_update_boost_loop_00298d88,
        "SSXAudioSystem_UpdateBoostLoop",
    ),
    (
        0x002a_4a68,
        ssx_audio_system_section_speech_timer_callback_002a4a68,
        "SSXAudioSystem_SectionSpeechTimerCallback",
    ),
    (
        0x002b_83b8,
        c_world_sound_debug_printf,
        "cWorldSound_DebugPrintf",
    ),
    (
        0x002b_acf0,
        speech_host_base_get_speech_category_002bacf0,
        "SpeechHostBase_GetSpeechCategory",
    ),
    (
        0x002b_acf8,
        speech_host_base_get_speech_category_volume_002bacf8,
        "SpeechHostBase_GetSpeechCategoryVolume",
    ),
    (
        0x002b_bbf8,
        c_world_painter_man_debug_printf,
        "cWorldPainterMan_DebugPrintf",
    ),
    (
        0x002b_c500,
        t_wpigd_base_get_music_trigger08_002bc500,
        "tWPIGD_Base_GetMusicTrigger08",
    ),
    (
        0x002b_c508,
        t_wpigd_base_get_music_trigger10_002bc508,
        "tWPIGD_Base_GetMusicTrigger10",
    ),
    (
        0x002b_c510,
        t_wpigd_base_get_mix08_002bc510,
        "tWPIGD_Base_GetMix08",
    ),
    (
        0x002b_c518,
        t_wpigd_base_get_ambience08_002bc518,
        "tWPIGD_Base_GetAmbience08",
    ),
    (
        0x002b_c520,
        t_wpigd_base_get_speech08_002bc520,
        "tWPIGD_Base_GetSpeech08",
    ),
    (
        0x002b_c528,
        t_wpigd_base_get_camera08_002bc528,
        "tWPIGD_Base_GetCamera08",
    ),
    (
        0x002b_c530,
        t_wpigd_base_get_fog08_002bc530,
        "tWPIGD_Base_GetFog08",
    ),
    (
        0x002b_c538,
        t_wpigd_base_get_fog10_002bc538,
        "tWPIGD_Base_GetFog10",
    ),
    (
        0x002b_c540,
        t_wpigd_base_get_fog18_002bc540,
        "tWPIGD_Base_GetFog18",
    ),
    (
        0x002b_c548,
        t_wpigd_base_get_fog20_002bc548,
        "tWPIGD_Base_GetFog20",
    ),
    (
        0x002b_c550,
        t_wpigd_base_get_fog28_002bc550,
        "tWPIGD_Base_GetFog28",
    ),
    (
        0x002b_c558,
        t_wpigd_base_get_fog30_002bc558,
        "tWPIGD_Base_GetFog30",
    ),
    (
        0x002b_c560,
        t_wpigd_base_get_light_glow08_002bc560,
        "tWPIGD_Base_GetLightGlow08",
    ),
];

/// `ScriptEngine_SeqListener_v05` (0x0028_47f8): straight-line leaf. Mechanical port of the generated body.
pub fn script_engine_seq_listener_v05_002847f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `ScriptEngine_SeqListener_v07` (0x0028_4800): straight-line leaf. Mechanical port of the generated body.
pub fn script_engine_seq_listener_v07_00284800(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ScriptEngine_SeqListener_v08` (0x0028_4808): straight-line leaf. Mechanical port of the generated body.
pub fn script_engine_seq_listener_v08_00284808(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ScriptEngine_SeqListener_v11` (0x0028_4810): straight-line leaf. Mechanical port of the generated body.
pub fn script_engine_seq_listener_v11_00284810(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `ScriptEngine_ScriptDbLoadBank` (0x0028_4820): straight-line leaf. Mechanical port of the generated body.
pub fn script_engine_script_db_load_bank_00284820(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `ScriptEngine_ScriptDbCancelBank` (0x0028_4828): straight-line leaf. Mechanical port of the generated body.
pub fn script_engine_script_db_cancel_bank_00284828(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ScriptEngine_ScriptDbUnloadBank` (0x0028_4830): straight-line leaf. Mechanical port of the generated body.
pub fn script_engine_script_db_unload_bank_00284830(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ScriptEngine_ScriptDb_v10` (0x0028_4838): straight-line leaf. Mechanical port of the generated body.
pub fn script_engine_script_db_v10_00284838(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ScriptEngine_ScriptDb_v11` (0x0028_4840): straight-line leaf. Mechanical port of the generated body.
pub fn script_engine_script_db_v11_00284840(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ScriptEngine_ScriptDb_v12` (0x0028_4848): straight-line leaf. Mechanical port of the generated body.
pub fn script_engine_script_db_v12_00284848(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ScriptBinding_Apply` (0x0028_4878): straight-line leaf. Mechanical port of the generated body.
pub fn script_binding_apply_00284878(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ScriptBinding_Tick` (0x0028_4880): straight-line leaf. Mechanical port of the generated body.
pub fn script_binding_tick_00284880(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ScriptBinding_AcceptCommand` (0x0028_4888): straight-line leaf. Mechanical port of the generated body.
pub fn script_binding_accept_command_00284888(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `ScriptDummyObj_GetBindingCount` (0x0028_4940): straight-line leaf. Mechanical port of the generated body.
pub fn script_dummy_obj_get_binding_count_00284940(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `ScriptDummyObj_GetBinding` (0x0028_4948): straight-line leaf. Mechanical port of the generated body.
pub fn script_dummy_obj_get_binding_00284948(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `ScriptAudioObj_GetBindingCount` (0x0028_49b8): straight-line leaf. Mechanical port of the generated body.
pub fn script_audio_obj_get_binding_count_002849b8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `ScriptAudioObj_GetBinding` (0x0028_49c0): straight-line leaf. Mechanical port of the generated body.
pub fn script_audio_obj_get_binding_002849c0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `ScriptObjCam_GetBindingCount` (0x0028_4a88): straight-line leaf. Mechanical port of the generated body.
pub fn script_obj_cam_get_binding_count_00284a88(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `ScriptObjCam_GetBinding` (0x0028_4a90): straight-line leaf. Mechanical port of the generated body.
pub fn script_obj_cam_get_binding_00284a90(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cTransitionMan_ScriptGetBindingCount` (0x0028_4ad8): straight-line leaf. Mechanical port of the generated body.
pub fn c_transition_man_script_get_binding_count_00284ad8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cTransitionMan_ScriptGetBinding` (0x0028_4ae0): straight-line leaf. Mechanical port of the generated body.
pub fn c_transition_man_script_get_binding_00284ae0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `RiderScriptObj_GetBindingCount` (0x0028_4b50): straight-line leaf. Mechanical port of the generated body.
pub fn rider_script_obj_get_binding_count_00284b50(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `RiderScriptObj_GetBinding` (0x0028_4b58): straight-line leaf. Mechanical port of the generated body.
pub fn rider_script_obj_get_binding_00284b58(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `SSXAudioSystem_UpdateAmbienceLoop` (0x0029_1430): straight-line leaf. Mechanical port of the generated body.
pub fn ssx_audio_system_update_ambience_loop_00291430(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `SSXAudioSystem_UpdateWipeoutLoop` (0x0029_6e18): straight-line leaf. Mechanical port of the generated body.
pub fn ssx_audio_system_update_wipeout_loop_00296e18(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `SSXAudioSystem_UpdateWipeoutLoop2` (0x0029_7430): straight-line leaf. Mechanical port of the generated body.
pub fn ssx_audio_system_update_wipeout_loop2_00297430(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `SSXAudioSystem_UpdateBoostLoop` (0x0029_8d88): straight-line leaf. Mechanical port of the generated body.
pub fn ssx_audio_system_update_boost_loop_00298d88(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `SSXAudioSystem_SectionSpeechTimerCallback` (0x002a_4a68): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ssx_audio_system_section_speech_timer_callback_002a4a68(c: &mut Cpu) {
    c.cycles += 3;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x5818), 0u64);
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cWorldSound_DebugPrintf` (0x002b_83b8): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn c_world_sound_debug_printf(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.g[V0][0] = c.g[A0][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x44));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
    ret(c);
}

/// `SpeechHostBase_GetSpeechCategory` (0x002b_acf0): straight-line leaf. Mechanical port of the generated body.
pub fn speech_host_base_get_speech_category_002bacf0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xffffffffffffffff;
    ret(c);
}

/// `SpeechHostBase_GetSpeechCategoryVolume` (0x002b_acf8): straight-line leaf. Mechanical port of the generated body.
pub fn speech_host_base_get_speech_category_volume_002bacf8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cWorldPainterMan_DebugPrintf` (0x002b_bbf8): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn c_world_painter_man_debug_printf(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.g[V0][0] = c.g[A0][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x44));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
    ret(c);
}

/// `tWPIGD_Base_GetMusicTrigger08` (0x002b_c500): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_music_trigger08_002bc500(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetMusicTrigger10` (0x002b_c508): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_music_trigger10_002bc508(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetMix08` (0x002b_c510): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_mix08_002bc510(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetAmbience08` (0x002b_c518): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_ambience08_002bc518(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSpeech08` (0x002b_c520): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_speech08_002bc520(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetCamera08` (0x002b_c528): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_camera08_002bc528(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetFog08` (0x002b_c530): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_fog08_002bc530(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetFog10` (0x002b_c538): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_fog10_002bc538(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetFog18` (0x002b_c540): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_fog18_002bc540(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetFog20` (0x002b_c548): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_fog20_002bc548(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetFog28` (0x002b_c550): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_fog28_002bc550(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetFog30` (0x002b_c558): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_fog30_002bc558(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLightGlow08` (0x002b_c560): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow08_002bc560(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, check_lohi, Rng};

    /// An edge-case 64-bit register value (0, 1, -1, small, random).
    #[allow(dead_code)]
    fn leaf_val(r: &mut Rng) -> u64 {
        match r.next_u32() % 8 {
            0 => 0,
            1 => 1,
            2 => u64::MAX,
            3 => (r.next_u32() % 64) as u64,
            4 => sx(r.next_u32()),
            5 => r.next_u32() as u64,
            6 => 0x8000_0000,
            _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        }
    }

    /// Fill `size` bytes at `start` with zero / small / random words, or, when
    /// `chase` is set, with pointers (16-byte aligned) into `ptr_lo..ptr_lo + 0x200`.
    #[allow(dead_code)]
    fn leaf_fill(c: &mut Cpu, r: &mut Rng, start: u32, size: u32, chase: bool, ptr_lo: u32) {
        for i in 0..size / 4 {
            let w = if chase {
                ptr_lo + 16 * (r.next_u32() % 32)
            } else {
                match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.next_u32() % 16,
                    _ => r.next_u32(),
                }
            };
            c.bus.write32(start + 4 * i, w);
        }
    }

    /// `ScriptEngine_SeqListener_v05` (0x0028_47f8) against the generated body.
    #[test]
    fn script_engine_seq_listener_v05_002847f8_matches() {
        check(
            ee::s07::f_002847f8__generated,
            script_engine_seq_listener_v05_002847f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2847f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptEngine_SeqListener_v07` (0x0028_4800) against the generated body.
    #[test]
    fn script_engine_seq_listener_v07_00284800_matches() {
        check(
            ee::s07::f_00284800__generated,
            script_engine_seq_listener_v07_00284800,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284800, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptEngine_SeqListener_v08` (0x0028_4808) against the generated body.
    #[test]
    fn script_engine_seq_listener_v08_00284808_matches() {
        check(
            ee::s07::f_00284808__generated,
            script_engine_seq_listener_v08_00284808,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284808, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptEngine_SeqListener_v11` (0x0028_4810) against the generated body.
    #[test]
    fn script_engine_seq_listener_v11_00284810_matches() {
        check(
            ee::s07::f_00284810__generated,
            script_engine_seq_listener_v11_00284810,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284810, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptEngine_ScriptDbLoadBank` (0x0028_4820) against the generated body.
    #[test]
    fn script_engine_script_db_load_bank_00284820_matches() {
        check(
            ee::s07::f_00284820__generated,
            script_engine_script_db_load_bank_00284820,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284820, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptEngine_ScriptDbCancelBank` (0x0028_4828) against the generated body.
    #[test]
    fn script_engine_script_db_cancel_bank_00284828_matches() {
        check(
            ee::s07::f_00284828__generated,
            script_engine_script_db_cancel_bank_00284828,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284828, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptEngine_ScriptDbUnloadBank` (0x0028_4830) against the generated body.
    #[test]
    fn script_engine_script_db_unload_bank_00284830_matches() {
        check(
            ee::s07::f_00284830__generated,
            script_engine_script_db_unload_bank_00284830,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284830, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptEngine_ScriptDb_v10` (0x0028_4838) against the generated body.
    #[test]
    fn script_engine_script_db_v10_00284838_matches() {
        check(
            ee::s07::f_00284838__generated,
            script_engine_script_db_v10_00284838,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284838, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptEngine_ScriptDb_v11` (0x0028_4840) against the generated body.
    #[test]
    fn script_engine_script_db_v11_00284840_matches() {
        check(
            ee::s07::f_00284840__generated,
            script_engine_script_db_v11_00284840,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284840, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptEngine_ScriptDb_v12` (0x0028_4848) against the generated body.
    #[test]
    fn script_engine_script_db_v12_00284848_matches() {
        check(
            ee::s07::f_00284848__generated,
            script_engine_script_db_v12_00284848,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284848, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptBinding_Apply` (0x0028_4878) against the generated body.
    #[test]
    fn script_binding_apply_00284878_matches() {
        check(
            ee::s07::f_00284878__generated,
            script_binding_apply_00284878,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284878, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptBinding_Tick` (0x0028_4880) against the generated body.
    #[test]
    fn script_binding_tick_00284880_matches() {
        check(
            ee::s07::f_00284880__generated,
            script_binding_tick_00284880,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284880, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptBinding_AcceptCommand` (0x0028_4888) against the generated body.
    #[test]
    fn script_binding_accept_command_00284888_matches() {
        check(
            ee::s07::f_00284888__generated,
            script_binding_accept_command_00284888,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284888, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptDummyObj_GetBindingCount` (0x0028_4940) against the generated body.
    #[test]
    fn script_dummy_obj_get_binding_count_00284940_matches() {
        check(
            ee::s07::f_00284940__generated,
            script_dummy_obj_get_binding_count_00284940,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284940, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptDummyObj_GetBinding` (0x0028_4948) against the generated body.
    #[test]
    fn script_dummy_obj_get_binding_00284948_matches() {
        check(
            ee::s07::f_00284948__generated,
            script_dummy_obj_get_binding_00284948,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x284948, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptAudioObj_GetBindingCount` (0x0028_49b8) against the generated body.
    #[test]
    fn script_audio_obj_get_binding_count_002849b8_matches() {
        check(
            ee::s07::f_002849b8__generated,
            script_audio_obj_get_binding_count_002849b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2849b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptAudioObj_GetBinding` (0x0028_49c0) against the generated body.
    #[test]
    fn script_audio_obj_get_binding_002849c0_matches() {
        check(
            ee::s07::f_002849c0__generated,
            script_audio_obj_get_binding_002849c0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2849c0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptObjCam_GetBindingCount` (0x0028_4a88) against the generated body.
    #[test]
    fn script_obj_cam_get_binding_count_00284a88_matches() {
        check(
            ee::s07::f_00284a88__generated,
            script_obj_cam_get_binding_count_00284a88,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284a88, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptObjCam_GetBinding` (0x0028_4a90) against the generated body.
    #[test]
    fn script_obj_cam_get_binding_00284a90_matches() {
        check(
            ee::s07::f_00284a90__generated,
            script_obj_cam_get_binding_00284a90,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284a90, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cTransitionMan_ScriptGetBindingCount` (0x0028_4ad8) against the generated body.
    #[test]
    fn c_transition_man_script_get_binding_count_00284ad8_matches() {
        check(
            ee::s07::f_00284ad8__generated,
            c_transition_man_script_get_binding_count_00284ad8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284ad8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cTransitionMan_ScriptGetBinding` (0x0028_4ae0) against the generated body.
    #[test]
    fn c_transition_man_script_get_binding_00284ae0_matches() {
        check(
            ee::s07::f_00284ae0__generated,
            c_transition_man_script_get_binding_00284ae0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284ae0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderScriptObj_GetBindingCount` (0x0028_4b50) against the generated body.
    #[test]
    fn rider_script_obj_get_binding_count_00284b50_matches() {
        check(
            ee::s07::f_00284b50__generated,
            rider_script_obj_get_binding_count_00284b50,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x284b50, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderScriptObj_GetBinding` (0x0028_4b58) against the generated body.
    #[test]
    fn rider_script_obj_get_binding_00284b58_matches() {
        check(
            ee::s07::f_00284b58__generated,
            rider_script_obj_get_binding_00284b58,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x284b58, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SSXAudioSystem_UpdateAmbienceLoop` (0x0029_1430) against the generated body.
    #[test]
    fn ssx_audio_system_update_ambience_loop_00291430_matches() {
        check(
            ee::s07::f_00291430__generated,
            ssx_audio_system_update_ambience_loop_00291430,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x291430, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SSXAudioSystem_UpdateWipeoutLoop` (0x0029_6e18) against the generated body.
    #[test]
    fn ssx_audio_system_update_wipeout_loop_00296e18_matches() {
        check(
            ee::s08::f_00296e18__generated,
            ssx_audio_system_update_wipeout_loop_00296e18,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x296e18, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SSXAudioSystem_UpdateWipeoutLoop2` (0x0029_7430) against the generated body.
    #[test]
    fn ssx_audio_system_update_wipeout_loop2_00297430_matches() {
        check(
            ee::s08::f_00297430__generated,
            ssx_audio_system_update_wipeout_loop2_00297430,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x297430, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SSXAudioSystem_UpdateBoostLoop` (0x0029_8d88) against the generated body.
    #[test]
    fn ssx_audio_system_update_boost_loop_00298d88_matches() {
        check(
            ee::s08::f_00298d88__generated,
            ssx_audio_system_update_boost_loop_00298d88,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x298d88, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SSXAudioSystem_SectionSpeechTimerCallback` (0x002a_4a68) against the generated body.
    #[test]
    fn ssx_audio_system_section_speech_timer_callback_002a4a68_matches() {
        check(
            ee::s08::f_002a4a68__generated,
            ssx_audio_system_section_speech_timer_callback_002a4a68,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a4a68, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x7000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cWorldSound_DebugPrintf` (0x002b_83b8) against the generated body.
    #[test]
    fn c_world_sound_debug_printf_matches() {
        check(
            ee::s08::f_002b83b8__generated,
            c_world_sound_debug_printf,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b83b8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[A1][0] = leaf_val(&mut r);
                c.g[A2][0] = leaf_val(&mut r);
                c.g[A3][0] = leaf_val(&mut r);
                c.g[T0][0] = leaf_val(&mut r);
                c.g[T1][0] = leaf_val(&mut r);
                c.g[T2][0] = leaf_val(&mut r);
                c.g[T3][0] = leaf_val(&mut r);
                c.f[12] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.f[16] = r.edge_f32();
                c.f[18] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SpeechHostBase_GetSpeechCategory` (0x002b_acf0) against the generated body.
    #[test]
    fn speech_host_base_get_speech_category_002bacf0_matches() {
        check(
            ee::s08::f_002bacf0__generated,
            speech_host_base_get_speech_category_002bacf0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bacf0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SpeechHostBase_GetSpeechCategoryVolume` (0x002b_acf8) against the generated body.
    #[test]
    fn speech_host_base_get_speech_category_volume_002bacf8_matches() {
        check(
            ee::s08::f_002bacf8__generated,
            speech_host_base_get_speech_category_volume_002bacf8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bacf8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cWorldPainterMan_DebugPrintf` (0x002b_bbf8) against the generated body.
    #[test]
    fn c_world_painter_man_debug_printf_matches() {
        check(
            ee::s08::f_002bbbf8__generated,
            c_world_painter_man_debug_printf,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2bbbf8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[A1][0] = leaf_val(&mut r);
                c.g[A2][0] = leaf_val(&mut r);
                c.g[A3][0] = leaf_val(&mut r);
                c.g[T0][0] = leaf_val(&mut r);
                c.g[T1][0] = leaf_val(&mut r);
                c.g[T2][0] = leaf_val(&mut r);
                c.g[T3][0] = leaf_val(&mut r);
                c.f[12] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.f[16] = r.edge_f32();
                c.f[18] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetMusicTrigger08` (0x002b_c500) against the generated body.
    #[test]
    fn t_wpigd_base_get_music_trigger08_002bc500_matches() {
        check(
            ee::s08::f_002bc500__generated,
            t_wpigd_base_get_music_trigger08_002bc500,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc500, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetMusicTrigger10` (0x002b_c508) against the generated body.
    #[test]
    fn t_wpigd_base_get_music_trigger10_002bc508_matches() {
        check(
            ee::s08::f_002bc508__generated,
            t_wpigd_base_get_music_trigger10_002bc508,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc508, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetMix08` (0x002b_c510) against the generated body.
    #[test]
    fn t_wpigd_base_get_mix08_002bc510_matches() {
        check(
            ee::s08::f_002bc510__generated,
            t_wpigd_base_get_mix08_002bc510,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc510, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetAmbience08` (0x002b_c518) against the generated body.
    #[test]
    fn t_wpigd_base_get_ambience08_002bc518_matches() {
        check(
            ee::s08::f_002bc518__generated,
            t_wpigd_base_get_ambience08_002bc518,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc518, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSpeech08` (0x002b_c520) against the generated body.
    #[test]
    fn t_wpigd_base_get_speech08_002bc520_matches() {
        check(
            ee::s08::f_002bc520__generated,
            t_wpigd_base_get_speech08_002bc520,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc520, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetCamera08` (0x002b_c528) against the generated body.
    #[test]
    fn t_wpigd_base_get_camera08_002bc528_matches() {
        check(
            ee::s08::f_002bc528__generated,
            t_wpigd_base_get_camera08_002bc528,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc528, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetFog08` (0x002b_c530) against the generated body.
    #[test]
    fn t_wpigd_base_get_fog08_002bc530_matches() {
        check(
            ee::s08::f_002bc530__generated,
            t_wpigd_base_get_fog08_002bc530,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc530, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetFog10` (0x002b_c538) against the generated body.
    #[test]
    fn t_wpigd_base_get_fog10_002bc538_matches() {
        check(
            ee::s08::f_002bc538__generated,
            t_wpigd_base_get_fog10_002bc538,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc538, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetFog18` (0x002b_c540) against the generated body.
    #[test]
    fn t_wpigd_base_get_fog18_002bc540_matches() {
        check(
            ee::s08::f_002bc540__generated,
            t_wpigd_base_get_fog18_002bc540,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc540, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetFog20` (0x002b_c548) against the generated body.
    #[test]
    fn t_wpigd_base_get_fog20_002bc548_matches() {
        check(
            ee::s08::f_002bc548__generated,
            t_wpigd_base_get_fog20_002bc548,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc548, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetFog28` (0x002b_c550) against the generated body.
    #[test]
    fn t_wpigd_base_get_fog28_002bc550_matches() {
        check(
            ee::s08::f_002bc550__generated,
            t_wpigd_base_get_fog28_002bc550,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc550, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetFog30` (0x002b_c558) against the generated body.
    #[test]
    fn t_wpigd_base_get_fog30_002bc558_matches() {
        check(
            ee::s08::f_002bc558__generated,
            t_wpigd_base_get_fog30_002bc558,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc558, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLightGlow08` (0x002b_c560) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow08_002bc560_matches() {
        check(
            ee::s08::f_002bc560__generated,
            t_wpigd_base_get_light_glow08_002bc560,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc560, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
