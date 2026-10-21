//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! Small script/audio/UI bookkeeping routines: script-engine object removal,
//! the script sound-bank list, the crowd sample list, the pause/loading-screen
//! audio latches, rider metrics and reset paths, and a long tail of destructor
//! thunks (`Foo_dtor(self, flags)`: run the body's destructor, then delete the
//! object when `flags & 1`), vtable-slot forwarders and tiny state setters.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `operator delete(void*)` (0x317e50).
const DELETE: Callee = Callee {
    addr: 0x0031_7e50,
    f: ee::s10::f_00317e50,
};

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0028_32d8, script_obj_unregister, "ScriptObj_Unregister"),
    (
        0x0028_3610,
        script_sound_bank_man_remove_bank,
        "ScriptSoundBankMan_RemoveBank",
    ),
    (
        0x0028_36d8,
        script_sound_bank_man_find_bank,
        "ScriptSoundBankMan_FindBank",
    ),
    (
        0x0028_9b70,
        ssx_audio_system_pause_game_audio,
        "SSXAudioSystem_PauseGameAudio",
    ),
    (
        0x0028_fa98,
        ssx_audio_system_stop_loading_screen_loop,
        "SSXAudioSystem_StopLoadingScreenLoop",
    ),
    (
        0x002a_3f90,
        ssx_audio_system_is_track_group5,
        "SSXAudioSystem_IsTrackGroup5",
    ),
    (
        0x002a_4be0,
        crowd_audio_delete_sample_list,
        "CrowdAudio_DeleteSampleList",
    ),
    (
        0x002a_e478,
        path_plat_calc_track_mem_size,
        "PATH_PlatCalcTrackMemSize",
    ),
    (
        0x002b_4978,
        music_player_play_preview,
        "MusicPlayer_PlayPreview",
    ),
    (0x002c_26d0, ustr_sprintf, "UStr_Sprintf"),
    (
        0x002d_1b58,
        world_get_rider_position,
        "World_GetRiderPosition",
    ),
    (
        0x002e_a670,
        c_cut_scene_transition_man_ctor,
        "cCutSceneTransitionMan_ctor",
    ),
    (
        0x0030_ba80,
        wscript_man_get_collected_count,
        "WScriptMan_GetCollectedCount",
    ),
    (
        0x0030_bac8,
        wscript_man_add_collect_items,
        "WScriptMan_AddCollectItems",
    ),
    (0x0031_4988, mdf_file_dtor, "MdfFile_dtor"),
    (0x0031_49d0, mdf_file_free, "MdfFile_Free"),
    (
        0x0032_6430,
        input_preproc_emit_line_marker,
        "InputPreproc_EmitLineMarker",
    ),
    (0x0034_3820, counter_update, "Counter_Update"),
    (0x0035_0b98, dead_fade_ctor, "DeadFade_ctor"),
    (
        0x0035_f788,
        uv_scroll_modifier_ctor_2,
        "UVScrollModifier_ctor_2",
    ),
    (0x0036_1a60, magnet_modifier_v18, "MagnetModifier_v18"),
    (0x0037_cbe8, ps2_graphics_man_v59, "PS2GraphicsMan_v59"),
    (0x0039_16c0, font_dtor, "Font_dtor"),
    (0x0039_b760, uimenu_reset_scroll, "UIMenu_ResetScroll"),
    (0x0039_fd38, ui_object_clear_anim, "UIObject_ClearAnim"),
    (0x003a_0000, ui_object_update_anim, "UIObject_UpdateAnim"),
    (0x003a_3d28, ui_vector_update_anim, "UIVector_UpdateAnim"),
    (0x003a_77d0, section_allocator_dtor, "SectionAllocator_dtor"),
    (
        0x003a_8fb8,
        c_world_cache_streamer_dtor,
        "cWorldCache_StreamerDtor",
    ),
    (
        0x003b_0538,
        rcmp_decoder_release_codec,
        "RCMP_DECODER_ReleaseCodec",
    ),
    (0x003b_06b0, rcmp_decoder_get_chunk, "RCMP_DECODER_GetChunk"),
    (
        0x003b_0770,
        rcmp_static_init_destroy,
        "RCMP_StaticInitDestroy",
    ),
    (
        0x003b_11a0,
        ps2_sony_codec_internal_create,
        "PS2_SONY_CODEC_INTERNAL_Create",
    ),
    (
        0x003b_2318,
        mpeg2_extra_bit_information,
        "MPEG2_extra_bit_information",
    ),
    (
        0x0011_73b8,
        rider_metrix_clear_hud_messages,
        "RiderMetrix_ClearHUDMessages",
    ),
    (0x0012_35f8, rider_force_reset, "Rider_ForceReset"),
    (
        0x0012_c078,
        rider_gate_sub_can_start,
        "RiderGateSub_CanStart",
    ),
    (
        0x0015_1c48,
        be_set_char_profile_bolt_owned,
        "BE_SetCharProfileBoltOwned",
    ),
    (
        0x0015_50e8,
        be_score_int_get_high_score_for,
        "BEScoreInt_GetHighScoreFor",
    ),
    (
        0x0016_0b70,
        relative_chase_camera_algorithm_ctor,
        "RelativeChaseCameraAlgorithm_ctor",
    ),
    (
        0x0016_c968,
        camera_trigger_man_reset_active,
        "CameraTriggerMan_ResetActive",
    ),
    (0x0019_ce20, bolt_file_cache_dtor, "BoltFile_CacheDtor"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    pub const SCRIPT_ENGINE_GET_INSTANCE: Callee = Callee {
        addr: 0x0027_d2e8,
        f: ee::s07::f_0027d2e8,
    };
    pub const SCRIPT_SOUND_BANK_MAN_FIND_BANK: Callee = Callee {
        addr: 0x0028_36d8,
        f: ee::s07::f_002836d8,
    };
    pub const SSX_AUDIO_SYSTEM_PAUSE: Callee = Callee {
        addr: 0x0029_ce28,
        f: ee::s08::f_0029ce28,
    };
    pub const BANK_MONITOR_DELETE_INSTANCE: Callee = Callee {
        addr: 0x002a_d5f0,
        f: ee::s08::f_002ad5f0,
    };
    pub const BELIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };
    pub const BELIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };
    pub const BE_NEW_RACE_INT_GET_TRACK_INFO: Callee = Callee {
        addr: 0x0014_4bc0,
        f: ee::s01::f_00144bc0,
    };
    pub const SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5: Callee = Callee {
        addr: 0x002a_3fd8,
        f: ee::s08::f_002a3fd8,
    };
    pub const CROWD_AUDIO_FREE_SAMPLE_LIST: Callee = Callee {
        addr: 0x002a_4cf8,
        f: ee::s08::f_002a4cf8,
    };
    pub const CUT_SCENE_TRANSITION_MAN_BASE_CTOR: Callee = Callee {
        addr: 0x0035_4648,
        f: ee::s11::f_00354648,
    };
    pub const CUT_SCENE_TRANSITION_MAN_INIT: Callee = Callee {
        addr: 0x002e_aa28,
        f: ee::s09::f_002eaa28,
    };
    pub const WSCRIPT_MAN_COLLECT_SET_GET_COUNT: Callee = Callee {
        addr: 0x0030_c468,
        f: ee::s10::f_0030c468,
    };
    pub const WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS: Callee = Callee {
        addr: 0x0030_c4a8,
        f: ee::s10::f_0030c4a8,
    };
    pub const EA_MEM_FREE: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };
    pub const MDF_FILE_FREE: Callee = Callee {
        addr: 0x0031_49d0,
        f: ee::s10::f_003149d0,
    };
    pub const SPRINTF: Callee = Callee {
        addr: 0x0041_7828,
        f: ee::s15::f_00417828,
    };
    pub const INPUT_PREPROC_WRITE_STRING: Callee = Callee {
        addr: 0x0032_6308,
        f: ee::s10::f_00326308,
    };
    pub const CINSTANCE_NODE_CTOR_V: Callee = Callee {
        addr: 0x0034_fb90,
        f: ee::s11::f_0034fb90,
    };
    pub const FONT_FREE: Callee = Callee {
        addr: 0x0039_19e8,
        f: ee::s12::f_003919e8,
    };
    pub const UI_LIST_COUNT: Callee = Callee {
        addr: 0x0039_79c0,
        f: ee::s13::f_003979c0,
    };
    pub const UI_ANIM_APPLY: Callee = Callee {
        addr: 0x0039_75e0,
        f: ee::s13::f_003975e0,
    };
    pub const UI_ANIM_ADVANCE: Callee = Callee {
        addr: 0x0039_74b0,
        f: ee::s13::f_003974b0,
    };
    pub const SECTION_ALLOCATOR_FREE_ALL: Callee = Callee {
        addr: 0x003a_7818,
        f: ee::s13::f_003a7818,
    };
    pub const C_WORLD_CACHE_STREAMER_DTOR_STUB: Callee = Callee {
        addr: 0x003a_9180,
        f: ee::s13::f_003a9180,
    };
    pub const RCMP_SYSTEM_CTOR: Callee = Callee {
        addr: 0x003b_0410,
        f: ee::s13::f_003b0410,
    };
    pub const PS2_SONY_CODEC_INTERNAL_CTOR: Callee = Callee {
        addr: 0x003b_0948,
        f: ee::s13::f_003b0948,
    };
    pub const MPEG2_GET_BITS1: Callee = Callee {
        addr: 0x003b_1270,
        f: ee::s13::f_003b1270,
    };
    pub const MPEG2_FLUSH_BUFFER: Callee = Callee {
        addr: 0x003b_1290,
        f: ee::s13::f_003b1290,
    };
    pub const RIDER_SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fec8,
        f: ee::s00::f_0011fec8,
    };
    pub const RIDER_SET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe78,
        f: ee::s00::f_0011fe78,
    };
    pub const BE_SCORE_INT_GET_HIGH_SCORE_TABLE: Callee = Callee {
        addr: 0x0014_ab20,
        f: ee::s01::f_0014ab20,
    };
    pub const CHASE_CAMERA_ALGORITHM_CTOR: Callee = Callee {
        addr: 0x0016_2318,
        f: ee::s01::f_00162318,
    };
    pub const ACTIVE_CAMERA_TRIGGER_LIST_CLEAR: Callee = Callee {
        addr: 0x0016_c628,
        f: ee::s02::f_0016c628,
    };
    pub const CAMERA_TRIGGER_STACK_CLEAR: Callee = Callee {
        addr: 0x0016_d200,
        f: ee::s02::f_0016d200,
    };
    pub const CAMERA_RESET_CHASE_CONTROLLER_SWITCHES: Callee = Callee {
        addr: 0x0015_dc18,
        f: ee::s01::f_0015dc18,
    };
    pub const BOLT_FILE_CACHE_SHUTDOWN: Callee = Callee {
        addr: 0x0019_cf40,
        f: ee::s03::f_0019cf40,
    };
    pub const DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::f_00317e50,
    };

    /// `call_indirect` targets the generator left as table calls.
    pub const SND_BANK_MONITOR_MEM_SIZE: u32 = 0x003b_7b00;
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// `void Foo_dtor(void* self, int flags)` (0x30 frame): run the destructor
/// `base`, then `__builtin_delete(self)` when bit 0 of `flags` is set.
///
/// The original reloads `s0` (`lq 16, 0x20(sp)`) *before* the delete branch, so
/// the delete call already sees the caller's `s0` — and so does its own frame
/// save. Reproduce that order.
fn dtor(c: &mut Cpu, base: Callee, ret_base: u32, ret_delete: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A1][0];
    base.call(c, ret_base)?;
    c.g[S0][0] &= 1;
    let keep = c.g[S0][0] == 0;
    frame.restore(c, S0, 0x20);
    if !keep {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        DELETE.call(c, ret_delete)?;
        frame.restore(c, S0, 0x20);
    }
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    c.cycles += 4;
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Script engine: object teardown and the script sound-bank list.

/// `void ScriptObj_Unregister(ScriptObj* obj)` (0x2832d8): remove `obj` from
/// the script engine's object list (`engine+0x2a8` holds a vtable-like table
/// whose slot 3 is `remove(this, obj->id)`), then mark the object dead with
/// `obj+4 = -1`.
pub fn script_obj_unregister(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_INSTANCE.call(c, 0x0028_32ec)?;
        let engine = r32(c, V0);
        c.g[V1][0] = c.lw(engine.wrapping_add(0x2a8));
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(4));
        let table = r32(c, V1);
        c.g[A0][0] = c.lh(table.wrapping_add(0x18));
        c.g[A2][0] = c.lw(table.wrapping_add(0x1c));
        let method = r32(c, A2);
        c.g[A0][0] = sx(engine.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0028_3304)?;
        c.g[V1][0] = u64::MAX;
        c.sw(r32(c, S0).wrapping_add(4), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptSoundBankMan_RemoveBank(SoundBankMan* m, int id)` (0x283610):
/// drop the bank `id` from the list: its slot-2 method takes the entry.
pub fn script_sound_bank_man_remove_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK.call(c, 0x0028_3624)?;
        let entry = c.g[V0][0];
        c.g[A1][0] = entry;
        if entry != 0 {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xc));
            let vt = r32(c, V0);
            c.g[A0][0] = c.lh(vt.wrapping_add(0x10));
            c.g[V1][0] = c.lw(vt.wrapping_add(0x14));
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0028_3644)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SndEntry* ScriptSoundBankMan_FindBank(SndList* list, int id)` (0x2836d8):
/// scan `list->count` 0xc-byte entries for the live one with `id`; returns
/// it, or 0.
pub fn script_sound_bank_man_find_bank(c: &mut Cpu) {
    c.cycles += 3;
    let list = abi::arg(c, 0);
    let id = c.g[A1][0];
    c.g[A2][0] = c.lw(list.wrapping_add(4));
    let none = (c.g[A2][0] as i64) <= 0;
    c.g[V1][0] = 0;
    if !none {
        c.cycles += 1;
        c.g[A0][0] = c.lw(list.wrapping_add(8));
        loop {
            c.cycles += 3;
            let data = c.lw(r32(c, A0).wrapping_add(8));
            c.g[V0][0] = data;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            let more;
            if data == 0 {
                c.cycles += 1;
                more = (c.g[V1][0] as i64) < (c.g[A2][0] as i64);
                c.g[V0][0] = more as u64;
            } else {
                c.cycles += 3;
                let entry_id = c.lw(r32(c, A0));
                c.g[V0][0] = entry_id;
                let hit = entry_id == id;
                more = (c.g[V1][0] as i64) < (c.g[A2][0] as i64);
                c.g[V0][0] = more as u64;
                if hit {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A0][0];
                    abi::ret(c);
                    return;
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_36e8);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// SSX audio system: pause latches.

/// `void SSXAudioSystem_PauseGameAudio(SSXAudioSystem* s)` (0x289b70): once
/// per session (`s+0x5fb4`), pause the bank monitor and the music stream.
pub fn ssx_audio_system_pause_game_audio(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x5fb4));
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            callee::SSX_AUDIO_SYSTEM_PAUSE.call(c, 0x0028_9b94)?;
            let stream = r32(c, S0).wrapping_add(0x118);
            c.g[A0][0] = sx(stream);
            call_ptr(c, 0x002b_3a70, 0x0028_9b9c)?;
            c.g[V0][0] = 1;
            c.sw(r32(c, S0).wrapping_add(0x5fb4), c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StopLoadingScreenLoop(SSXAudioSystem* s)` (0x28fa98):
/// when a loading-screen loop is playing (`s+0x62a8` >= 0), fade it out and
/// clear the index.
pub fn ssx_audio_system_stop_loading_screen_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x62a8));
        let idle = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = 1;
        if !idle {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x118));
            c.g[A0][0] = c.lw(r32(c, V0));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d8));
            callee::BANK_MONITOR_DELETE_INSTANCE.call(c, 0x0028_fac4)?;
            c.g[V0][0] = u64::MAX;
            c.sw(r32(c, S0).wrapping_add(0x62a8), c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool SSXAudioSystem_IsTrackGroup5(SSXAudioSystem* s)` (0x2a3f90): is the
/// current track's id in the group-5 range (0x11..0x15)? Reads the id through
/// the BE library's current phase interface.
pub fn ssx_audio_system_is_track_group5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::BELIBRARY_GET_INSTANCE.call(c, 0x002a_3fa4)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        callee::BELIBRARY_GET_INTERFACE.call(c, 0x002a_3fb0)?;
        c.g[A0][0] = c.g[V0][0];
        callee::BE_NEW_RACE_INT_GET_TRACK_INFO.call(c, 0x002a_3fb8)?;
        c.g[A1][0] = c.lw(r32(c, V0));
        c.g[A0][0] = c.g[S0][0];
        callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5.call(c, 0x002a_3fc4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdAudio_DeleteSampleList(CrowdAudio* self, int flags)` (0x2a4be0):
/// free the crowd sample list and, when `flags & 1`, the object.
pub fn crowd_audio_delete_sample_list(c: &mut Cpu) {
    guest(c, |c| {
        dtor(
            c,
            callee::CROWD_AUDIO_FREE_SAMPLE_LIST,
            0x002a_4bfc,
            0x002a_4c10,
        )
    })
}

// ---------------------------------------------------------------------------
// Music player.

/// `void MusicPlayer_PlayPreview(MusicPlayer* p, int slot)` (0x2b4978): when
/// the slot's stream (`p[slot+1]->+0xa0`) is loaded, tell the music stream
/// (`p+0x5440`) about it through its slot-3 method.
pub fn music_player_play_preview(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let slot = abi::arg(c, 1);
        c.g[A1][0] = sx(slot << 2);
        c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(4));
        c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0xa0));
        let unloaded = (c.g[A1][0] as i64) < 0;
        if !unloaded {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x5440));
            let vt = r32(c, V0);
            c.g[A0][0] = c.lh(vt.wrapping_add(0x18));
            c.g[V1][0] = c.lw(vt.wrapping_add(0x1c));
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x002b_49b0)?;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider metrics and state.

/// `void RiderMetrix_ClearHUDMessages(RiderMetrix* m)` (0x1173b8): mark every
/// HUD message (`m+0x1b0`, `m+0x1b4` entries of 0x9c bytes) as state 0x34.
pub fn rider_metrix_clear_hud_messages(c: &mut Cpu) {
    c.cycles += 3;
    let m = r32(c, A0);
    c.g[V0][0] = c.lw(m.wrapping_add(0x1b0));
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = c.lw(m.wrapping_add(0x1b4));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A2][0] = 0x34;
            c.g[A1][0] = 0;
            loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(m.wrapping_add(0x1b0));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.sw(r32(c, V0), c.g[A2][0]);
                c.g[V0][0] = c.lw(m.wrapping_add(0x1b4));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x9c));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0011_73d8);
                }
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void Rider_ForceReset(Rider* r)` (0x1235f8): hard-reset a rider: set the
/// handler's reset latch (`+0x350`), go to sub-state 9 / main state 3 and
/// clear the handler's `+0x290`.
pub fn rider_force_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 9;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x77c));
        c.sw(r32(c, V0).wrapping_add(0x350), c.g[V1][0]);
        callee::RIDER_SET_SUB_STATE.call(c, 0x0012_361c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 3;
        callee::RIDER_SET_MAIN_STATE.call(c, 0x0012_3628)?;
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x77c));
        c.sw(r32(c, V0).wrapping_add(0x290), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool RiderGateSub_CanStart(GateSub* g)` (0x12c078): false only when the
/// race is running (mode != 5) and the gate's timer (`g+0x14 -> +0x470`) is
/// still negative.
pub fn rider_gate_sub_can_start(c: &mut Cpu) {
    c.cycles += 8;
    let world = c.lw(gp(c, -0x848)) as u32;
    c.g[A3][0] = c.g[A0][0];
    c.g[A2][0] = 5;
    c.g[V0][0] = c.lw(world.wrapping_add(0x84));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
    c.g[A0][0] = c.lw(r32(c, V1));
    let running = c.g[A0][0] != c.g[A2][0];
    c.g[V0][0] = 1;
    if running {
        c.cycles += 6;
        c.g[V1][0] = c.lw(r32(c, A3).wrapping_add(0x14));
        c.f[1] = 0;
        let timer = Addr(r32(c, V1).wrapping_add(0x470)).read_fpu(c);
        c.f[0] = timer.0;
        let ready = FpuF32::ZERO.le(timer);
        abi::set_fcc(c, ready);
        if !ready {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
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

/// `int BEScoreInt_GetHighScoreFor(score, int i, int a, int b)` (0x1550e8):
/// entry `i` (0x14 bytes) of the high-score table for `(a, b)`, or 0.
pub fn be_score_int_get_high_score_for(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A2][0];
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A3][0];
        callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE.call(c, 0x0015_5104)?;
        let table = c.g[V0][0];
        c.g[V1][0] = 0x14;
        if table != 0 {
            c.cycles += 3;
            c.mult(0, A0, S0, V1);
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
            c.g[V0][0] = c.lw(r32(c, V1));
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Chunk (camera/misc) updates.

/// `void* RelativeChaseCameraAlgorithm_ctor(Alg* self, float f)` (0x160b70):
/// the chase-camera algorithm with the relative chase variant: base
/// constructor, its vtable, `+0x390 = f` and `+0x394 = 0`. Returns `self`.
pub fn relative_chase_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x20);
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        callee::CHASE_CAMERA_ALGORITHM_CTOR.call(c, 0x0016_0b8c)?;
        c.g[V1][0] = 0x460000;
        c.swc1(20, r32(c, S0).wrapping_add(0x390));
        c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x4620));
        c.g[V0][0] = c.g[S0][0];
        c.sw(r32(c, S0).wrapping_add(0x10), c.g[V1][0]);
        c.sw(r32(c, S0).wrapping_add(0x394), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerMan_ResetActive(CameraTriggerMan* m)` (0x16c968): purge
/// both active-trigger lists, re-init both trigger stacks and reset the chase
/// controller switches.
pub fn camera_trigger_man_reset_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x14));
        callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR.call(c, 0x0016_c980)?;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x18));
        callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR.call(c, 0x0016_c988)?;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x1c));
        callee::CAMERA_TRIGGER_STACK_CLEAR.call(c, 0x0016_c990)?;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x3c));
        callee::CAMERA_TRIGGER_STACK_CLEAR.call(c, 0x0016_c998)?;
        callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES.call(c, 0x0016_c9a0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Counter_Update(DebounceNode* self)` (0x343820): when the debounce
/// timer (`+0x1c`) has not started, set it to -1 and tell the node
/// (`vtable` slot 0x110) that it was updated.
pub fn counter_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x1c));
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            c.g[V0][0] = u64::MAX;
            c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0xc));
            c.sw(r32(c, A2).wrapping_add(0x1c), u64::MAX);
            let vt = r32(c, V1);
            c.g[A1][0] = 1;
            c.g[A0][0] = c.lh(vt.wrapping_add(0x110));
            c.g[V0][0] = c.lw(vt.wrapping_add(0x114));
            let method = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0034_3858)?;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* DeadFade_ctor(DeadFade* self)` (0x350b98): a dead-fade node: base
/// instance node, vtable 0x491980, `+0x1c = -1`, `+0x38 = 0`. Returns `self`.
pub fn dead_fade_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CINSTANCE_NODE_CTOR_V.call(c, 0x0035_0bac)?;
        c.g[V1][0] = 0x490000;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_add(0x1980));
        c.g[A0][0] |= 0xffff;
        c.sw(r32(c, S0).wrapping_add(0xc), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(r32(c, S0).wrapping_add(0x1c), c.g[A0][0]);
        c.sw(r32(c, S0).wrapping_add(0x38), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIMenu_ResetScroll(UIMenu* m, int count)` (0x39b760): clear the
/// scroll cursor, remember the item count, flag the widget dirty (`+0x90 |=
/// 4`) and reset the first visible row.
pub fn uimenu_reset_scroll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sb(r32(c, S0).wrapping_add(0x95), 0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x74));
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x90));
        c.sb(r32(c, S0).wrapping_add(0x96), c.g[A1][0]);
        c.g[V0][0] |= 4;
        c.sw(r32(c, S0).wrapping_add(0x90), c.g[V0][0]);
        callee::UI_LIST_COUNT.call(c, 0x0039_b78c)?;
        c.sb(r32(c, S0).wrapping_add(0x97), c.g[V0][0]);
        c.sb(r32(c, S0).wrapping_add(0x98), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Destructor thunks and vtable forwarders.

/// `void MdfFile_dtor(MdfFile* f, int flags)` (0x314988): free the file and,
/// when `flags & 1`, the object.
pub fn mdf_file_dtor(c: &mut Cpu) {
    guest(c, |c| {
        dtor(c, callee::MDF_FILE_FREE, 0x0031_49a4, 0x0031_49b8)
    })
}

/// `void MdfFile_Free(MdfFile* f)` (0x3149d0): release the buffer (`+0`) and
/// zero the four list fields.
pub fn mdf_file_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(r32(c, S0));
        if c.g[A0][0] == 0 {
            c.sw(r32(c, S0).wrapping_add(0xc), 0);
        } else {
            c.cycles += 3;
            callee::EA_MEM_FREE.call(c, 0x0031_49f4)?;
            c.sw(r32(c, S0).wrapping_add(0xc), 0);
        }
        c.cycles += 7;
        c.sw(r32(c, S0), 0);
        c.sw(r32(c, S0).wrapping_add(4), 0);
        c.sw(r32(c, S0).wrapping_add(8), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Font_dtor(Font* f, int flags)` (0x3916c0): free the font and, when
/// `flags & 1`, the object.
pub fn font_dtor(c: &mut Cpu) {
    guest(c, |c| dtor(c, callee::FONT_FREE, 0x0039_16dc, 0x0039_16f0))
}

/// `void SectionAllocator_dtor(SectionAllocator* a, int flags)` (0x3a77d0):
/// free every section and, when `flags & 1`, the allocator.
pub fn section_allocator_dtor(c: &mut Cpu) {
    guest(c, |c| {
        dtor(
            c,
            callee::SECTION_ALLOCATOR_FREE_ALL,
            0x003a_77ec,
            0x003a_7800,
        )
    })
}

/// `void cWorldCache_StreamerDtor(void* p, int flags)` (0x3a8fb8): the streamer
/// destructor and, when `flags & 1`, the object.
pub fn c_world_cache_streamer_dtor(c: &mut Cpu) {
    guest(c, |c| {
        dtor(
            c,
            callee::C_WORLD_CACHE_STREAMER_DTOR_STUB,
            0x003a_8fd4,
            0x003a_8fe8,
        )
    })
}

/// `void BoltFile_CacheDtor(BoltFileCache* c, int flags)` (0x19ce20): clear the
/// global bolt-file cache slot, shut the cache down and, when `flags & 1`,
/// delete the object.
pub fn bolt_file_cache_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.sw(gp(c, -0x1784), 0);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::BOLT_FILE_CACHE_SHUTDOWN.call(c, 0x0019_ce40)?;
        c.g[S0][0] &= 1;
        let keep = c.g[S0][0] == 0;
        // The original reloads s0 before the delete branch (see `dtor`).
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::DELETE.call(c, 0x0019_ce54)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* UVScrollModifier_ctor_2(UVScrollModifier* m, MbStream* s)` (0x35f788):
/// run the stream's slot-2 constructor on the object with size 0x50, return
/// the object.
pub fn uv_scroll_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x50;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = c.lw(r32(c, A3));
        let vt = r32(c, V1);
        c.g[A0][0] = c.lh(vt.wrapping_add(0x10));
        c.g[V0][0] = c.lw(vt.wrapping_add(0x14));
        let ctor = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
        call_ptr(c, ctor, 0x0035_f7b8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Vec4* MagnetModifier_v18(MagnetModifier* m)` (0x361a60): update the
/// modifier when it has a target (`+0x50`) and return its magnet vector
/// (`+0x60`).
pub fn magnet_modifier_v18(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x50));
        let idle = c.g[V0][0] == 0;
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x60));
        if !idle {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, S0));
            let vt = r32(c, V0);
            c.g[A0][0] = c.lh(vt.wrapping_add(0x18));
            c.g[V1][0] = c.lw(vt.wrapping_add(0x1c));
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0036_1a90)?;
            c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x60));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2GraphicsMan_v59(void* self, int* out, int a2, int a3)` (0x37cbe8):
/// ask slot 0x68 of the particle manager at `+0x10d8` for `(a2, a3, 0)` and
/// store the result in `out[1]`.
pub fn ps2_graphics_man_v59(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[T0][0] = c.lw(r32(c, A0).wrapping_add(0x10d8));
        c.g[A2][0] = c.g[A3][0];
        c.g[A3][0] = 0;
        let vt = r32(c, T0);
        c.g[V1][0] = c.lh(vt.wrapping_add(0x1a0));
        c.g[V0][0] = c.lw(vt.wrapping_add(0x1a4));
        let method = r32(c, V0);
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(r32(c, V1)));
        call_ptr(c, method, 0x0037_cc18)?;
        c.sw(r32(c, S0).wrapping_add(4), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIObject_ClearAnim(UIObject* o)` (0x39fd38): detach (`this, 3`) and
/// drop the animation object at `+0xc`.
pub fn ui_object_clear_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(0xc));
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x10));
            let vt = r32(c, V0);
            c.g[A0][0] = c.lh(vt.wrapping_add(8));
            c.g[V1][0] = c.lw(vt.wrapping_add(0xc));
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0039_fd68)?;
            c.sw(r32(c, S0).wrapping_add(0xc), 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RCMP_DECODER_ReleaseCodec(RCMP_DECODER* d)` (0x3b0538): detach
/// (`this, 3`) and drop the codec at `+0x14`.
pub fn rcmp_decoder_release_codec(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(0x14));
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, A2));
            let vt = r32(c, V0);
            c.g[A0][0] = c.lh(vt.wrapping_add(8));
            c.g[V1][0] = c.lw(vt.wrapping_add(0xc));
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x003b_0568)?;
            c.sw(r32(c, S0).wrapping_add(0x14), 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int RCMP_DECODER_GetChunk(RCMP_DECODER* d)` (0x3b06b0): return the pending
/// word if set, else call the decoder's `fn(self, arg, &out)` and return the
/// word it wrote.
pub fn rcmp_decoder_get_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        let this = r32(c, V0);
        c.g[V1][0] = c.lw(this);
        if c.g[V1][0] != 0 {
            c.sw(this, 0);
            c.cycles += 2;
            c.sw(sp(c), c.g[V1][0]);
            c.g[V0][0] = c.g[V1][0];
        } else {
            c.cycles += 6;
            c.g[A2][0] = sx(sp(c));
            c.g[V0][0] = c.lw(this.wrapping_add(8));
            let method = r32(c, V0);
            c.g[A1][0] = c.lw(this.wrapping_add(4));
            call_ptr(c, method, 0x003b_06d8)?;
            c.g[V0][0] = c.lw(sp(c));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void* RCMP_StaticInitDestroy(int init, int prio)` (0x3b0770): the one-time
/// RCMP static setup: with priority 0xffff, build the global RCMP system when
/// `init` is set, else point its slot at the destroy routine.
pub fn rcmp_static_init_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xffff;
        let not_static = c.g[A1][0] != c.g[V0][0];
        frame.save_ra(c, 0);
        if not_static {
            c.cycles += 1;
        } else {
            c.cycles += 2;
            let no_init = c.g[A0][0] == 0;
            c.g[A0][0] = 0x0051_0000;
            if no_init {
                c.cycles += 4;
                c.g[V0][0] = 0x0045_0000;
                c.g[V1][0] = 0x0051_0000;
                c.g[V0][0] = sx(0x0045_0000u32.wrapping_add(0x6890));
                let init_slot = 0x0051_0000u32.wrapping_sub(0x6bc4);
                c.sw(init_slot, c.g[V0][0]);
                c.cycles += 1;
            } else {
                c.cycles += 4;
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x6bd0));
                callee::RCMP_SYSTEM_CTOR.call(c, 0x003b_0790)?;
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* PS2_SONY_CODEC_INTERNAL_Create(void)` (0x3b11a0): allocate 0x80
/// bytes from the RCMP allocator, then run the codec constructor.
pub fn ps2_sony_codec_internal_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x0051_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x0051_0000u32.wrapping_sub(0x6bd0));
        c.g[V0][0] = c.lw(0x0050_9430);
        c.g[A0][0] = 0x0049_0000;
        c.g[T0][0] = c.lw(0x0050_9438);
        c.g[A2][0] = 0;
        c.g[A1][0] = 0x80;
        c.g[A3][0] = 0;
        let alloc = r32(c, V0);
        c.g[A0][0] = sx(0x0049_0000u32.wrapping_add(0x53e8));
        call_ptr(c, alloc, 0x003b_11d0)?;
        c.g[A0][0] = c.g[V0][0];
        callee::PS2_SONY_CODEC_INTERNAL_CTOR.call(c, 0x003b_11d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void World_GetRiderPosition(int i)` (0x2d1b58): call the rider slot `i`
/// object's (`race->riders[i]`) slot-5 method with its `+0x6c0` sub-object.
pub fn world_get_rider_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw(gp(c, -0x848));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(abi::arg(c, 0) << 2);
        c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x84));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xc));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x28));
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x6c0));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
        c.g[A1][0] = c.lh(r32(c, V1).wrapping_add(0x28));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x2c));
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        call_ptr(c, method, 0x002d_1b90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cCutSceneTransitionMan_ctor(cCutSceneTransitionMan* m)` (0x2ea670):
/// the cut-scene transition manager: base process ctor, vtable 0x487f30,
/// `+0x10 = +0x1c = 0`, then `SaveViewport`. Returns `m`.
pub fn c_cut_scene_transition_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CUT_SCENE_TRANSITION_MAN_BASE_CTOR.call(c, 0x002e_a684)?;
        c.g[V1][0] = 0x480000;
        c.sw(r32(c, S0).wrapping_add(0x10), 0);
        c.g[V1][0] = sx(0x0048_0000u32.wrapping_add(0x7f30));
        c.sw(r32(c, S0).wrapping_add(0x1c), 0);
        c.sw(r32(c, S0).wrapping_add(0xc), c.g[V1][0]);
        c.g[A0][0] = c.g[S0][0];
        callee::CUT_SCENE_TRANSITION_MAN_INIT.call(c, 0x002e_a6a0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScript collect-set lookups (single-entry tables).

/// `int WScriptMan_GetCollectedCount(WScriptMan* m, uint id)` (0x30ba80): the
/// collected count of the single collect-set entry, or 0 when the id does not
/// match.
pub fn wscript_man_get_collected_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x2c0));
        frame.save_ra(c, 0);
        c.g[V1][0] = 0;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, A0));
            let miss = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if !miss {
                c.cycles += 4;
                callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT.call(c, 0x0030_baa4)?;
                break;
            }
            c.cycles += 2;
            let exhausted = (c.g[V1][0] as i64) <= 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10c));
            if exhausted {
                if event_due(c) {
                    deliver_event_here(c, 0x0030_ba90);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            break;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_AddCollectItems(WScriptMan* m, uint id, int a1, int a2)`
/// (0x30bac8): add items to the single collect-set entry when the id matches.
pub fn wscript_man_add_collect_items(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x2c0));
        frame.save_ra(c, 0);
        c.g[V1][0] = 0;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, A0));
            let miss = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if !miss {
                c.cycles += 5;
                c.g[A1][0] = c.g[A2][0];
                c.g[A2][0] = c.g[A3][0];
                callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS.call(c, 0x0030_baf0)?;
                break;
            }
            c.cycles += 2;
            let exhausted = (c.g[V1][0] as i64) <= 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10c));
            if exhausted {
                if event_due(c) {
                    deliver_event_here(c, 0x0030_bad8);
                }
                continue;
            }
            c.cycles += 1;
            break;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InputPreproc_EmitLineMarker(InputPreproc* p, int line, char* text)`
/// (0x326430): append `# <line> "<text>"` to the preprocessed output.
pub fn input_preproc_emit_line_marker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xd0);
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S0, 0xc0);
        c.g[A2][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = 0x490000;
        frame.save_ra(c, 0xb0);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x1c38));
        callee::SPRINTF.call(c, 0x0032_6458)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        callee::INPUT_PREPROC_WRITE_STRING.call(c, 0x0032_6464)?;
        frame.restore(c, S0, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatCalcTrackMemSize(int n, int m)` (0x2ae478): the memory a track
/// needs: `n * 0xc + 0x70`, plus the stream size for `m` slots, rounded up to
/// a 16-byte boundary.
pub fn path_plat_calc_track_mem_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xc;
        c.mult(0, V0, A0, V0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(r32(c, V0).wrapping_add(0x70));
        if (c.g[A1][0] as i64) > 0 {
            c.cycles += 3;
            call_ptr(c, callee::SND_BANK_MONITOR_MEM_SIZE, 0x002a_e49c)?;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        }
        c.cycles += 7;
        c.g[V1][0] = c.g[S0][0] & 0xf;
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x10));
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Formatting and the MPEG2 bit reader.

/// `int UStr_Sprintf(wchar_t* dst, const wchar_t* fmt, ...)` (0x2c26d0): the
/// varargs shim: spill `a2..a3`, `t0..t3` (at `+0x50`) and `f12`, `f14`,
/// `f16`, `f18` (at `+0x40`) into the frame, then `USTR_vsprintf(dst, fmt,
/// &a2)`.
pub fn ustr_sprintf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
        c.sd(p(sp(c), 0x50), c.g[A2][0]);
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(p(sp(c), 0x50));
        c.sd(p(sp(c), 0x58), c.g[A3][0]);
        c.sd(p(sp(c), 0x60), c.g[T0][0]);
        c.sd(p(sp(c), 0x68), c.g[T1][0]);
        c.sd(p(sp(c), 0x70), c.g[T2][0]);
        c.sd(p(sp(c), 0x78), c.g[T3][0]);
        c.swc1(12, p(sp(c), 0x40));
        c.swc1(14, p(sp(c), 0x44));
        c.swc1(16, p(sp(c), 0x48));
        c.swc1(18, p(sp(c), 0x4c));
        call_ptr(c, 0x003d_c450, 0x002c_2708)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MPEG2_extra_bit_information(void)` (0x3b2318): count and discard the
/// extra-bit marker bytes (`while (Get_Bits1()) { Flush_Buffer(8); n++ }`).
/// Resumable: the loop head 0x3b2330 is an event resume entry.
pub fn mpeg2_extra_bit_information(c: &mut Cpu) {
    let entry = c.pc;
    guest(c, |c| mpeg2_extra_bits(c, entry))
}

fn mpeg2_extra_bits(c: &mut Cpu, mut pc: u32) -> Flow {
    const ENTRY: u32 = 0x003b_2318;
    const CHECK: u32 = 0x003b_2330;
    const PEEK: u32 = 0x003b_2338;
    const DONE: u32 = 0x003b_2348;
    let frame = Frame { size: 0x20 };
    loop {
        match pc {
            ENTRY => {
                c.cycles += 5;
                c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
                c.sq(S0, p(sp(c), 0x10));
                c.sd(sp(c), c.g[RA][0]);
                c.g[S0][0] = 0;
                // The prologue's degenerate branch goes straight to the peek;
                // 0x3b232c is only a resume entry.
                pc = PEEK;
            }
            0x003b_232c => {
                c.cycles += 1;
                pc = CHECK;
            }
            CHECK => {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                callee::MPEG2_FLUSH_BUFFER.call(c, PEEK)?;
                pc = PEEK;
            }
            PEEK => {
                c.cycles += 4;
                callee::MPEG2_GET_BITS1.call(c, 0x003b_2340)?;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = 8;
                if more {
                    if event_due(c) {
                        c.pc = CHECK;
                        if c.event() {
                            return Ok(());
                        }
                    }
                    pc = CHECK;
                } else {
                    pc = DONE;
                }
            }
            DONE => {
                c.cycles += 5;
                c.g[V0][0] = c.g[S0][0];
                frame.restore_ra(c, 0);
                frame.restore(c, S0, 0x10);
                frame.close(c);
                return Ok(());
            }
            _ => {
                c.jump_out(pc, ENTRY, 0x003b_235c);
                return Ok(());
            }
        }
    }
}

// ---------------------------------------------------------------------------
// UI animation updates.

/// Shared body of `UIObject_UpdateAnim` / `UIVector_UpdateAnim`: apply the
/// event to the animation at `+0xc` (only when present), then advance it.
fn ui_anim_update(c: &mut Cpu, apply_ret: u32, advance_ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0xc));
    let none = c.g[A0][0] == 0;
    c.g[A3][0] = c.g[A1][0] & 0xffff;
    if !none {
        c.cycles += 6;
        c.g[A2][0] = c.lbu(r32(c, S0).wrapping_add(0x10));
        c.g[A1][0] = c.g[S0][0];
        callee::UI_ANIM_APPLY.call(c, apply_ret)?;
        c.g[A1][0] = c.lbu(r32(c, S0).wrapping_add(0x10));
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0xc));
        callee::UI_ANIM_ADVANCE.call(c, advance_ret)?;
    }
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void UIObject_UpdateAnim(UIWidget* w, ushort event)` (0x3a0000): feed the
/// event to the widget's animation and advance it.
pub fn ui_object_update_anim(c: &mut Cpu) {
    guest(c, |c| ui_anim_update(c, 0x003a_0028, 0x003a_0034))
}

/// `void UIVector_UpdateAnim(UIVector* v, ushort event)` (0x3a3d28): the same
/// for a vector widget.
pub fn ui_vector_update_anim(c: &mut Cpu) {
    guest(c, |c| ui_anim_update(c, 0x003a_3d50, 0x003a_3d5c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, ptr, vtable, FAKE};
    use crate::difftest::{check, DATA, GAME_GP};

    /// The BE library singleton, so `BELibrary_GetInstance` never builds it.
    fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
    }

    /// [`be_library`], plus the current phase interface (`gp-0x1efc` selects
    /// one of the three) whose slot-2 method is the fake callee.
    fn be_library_interface(c: &mut Cpu, r: &mut crate::difftest::Rng) {
        be_library(c);
        let index = (r.next_u32() % 8) as u32;
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), index);
        let lib = ptr(r);
        c.bus.write32(0x0053_08a8 + 4 * index, lib);
        let vt = vtable(c, r, &[8]);
        c.bus.write32(lib + 4, vt);
    }

    #[test]
    fn script_obj_unregister_matches() {
        covered(
            ee::s07::f_002832d8__generated,
            script_obj_unregister,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_32d8, 1);
                let obj = DATA + 0x1000;
                let engine = DATA + 0x2000;
                // `ScriptEngine_GetInstance` reads the singleton at gp+0x3ec.
                c.bus.write32(GAME_GP + 0x3ec, engine);
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(engine + 0x2a8, vt);
                c.bus.write32(obj + 4, r.next_u32());
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    /// A one-entry sound bank list: `+4` count, `+8` entries (0xc bytes each),
    /// `+0xc` the manager vtable, entry 0 live.
    fn sound_bank_list(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) -> u32 {
        let list = DATA + 0x1000;
        let entries = DATA + 0x2000;
        let count = (seed % 4) as u32;
        c.bus.write32(list + 4, count);
        c.bus.write32(list + 8, entries);
        for i in 0..4u32 {
            let e = entries + 0xc * i;
            c.bus
                .write32(e, if i == 0 { 0 } else { 1 + r.next_u32() % 3 });
            c.bus
                .write32(e + 8, if i < count && i == 0 { 1 } else { 0 });
        }
        let vt = vtable(c, r, &[0x10]);
        c.bus.write32(list + 0xc, vt);
        list
    }

    #[test]
    fn script_sound_bank_man_remove_bank_matches() {
        covered(
            ee::s07::f_00283610__generated,
            script_sound_bank_man_remove_bank,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_3610, 1);
                let list = sound_bank_list(c, &mut r, seed);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn script_sound_bank_man_find_bank_matches() {
        check(
            ee::s07::f_002836d8__generated,
            script_sound_bank_man_find_bank,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_36d8, 0);
                let list = sound_bank_list(c, &mut r, seed);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = (r.next_u32() % 4) as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_pause_game_audio_matches() {
        check(
            ee::s07::f_00289b70__generated,
            ssx_audio_system_pause_game_audio,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_9b70, 0);
                let obj = DATA + 0x1000;
                c.bus
                    .write32(obj + 0x5fb4, if seed % 2 == 0 { 1 } else { 0 });
                c.bus.write32(obj + 0x118, DATA + 0x2000);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_loading_screen_loop_matches() {
        check(
            ee::s07::f_0028fa98__generated,
            ssx_audio_system_stop_loading_screen_loop,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_fa98, 0);
                let obj = DATA + 0x1000;
                c.bus.write32(
                    obj + 0x62a8,
                    match seed % 3 {
                        0 => u32::MAX,
                        1 => 3,
                        _ => 0,
                    },
                );
                c.bus.write32(obj + 0x118, DATA + 0x2000);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_group5_matches() {
        covered(
            ee::s08::f_002a3f90__generated,
            ssx_audio_system_is_track_group5,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_3f90, 1);
                be_library_interface(c, &mut r);
                // `BENewRaceInt_GetTrackInfo` indexes the 0x64-byte station table
                // by the live track index.
                let index = (r.next_u32() % 4) as u32;
                c.bus.write32(0x0053_5c08, index);
                let id = match seed % 4 {
                    0 => 0x11,
                    1 => 0x15,
                    2 => 0x10,
                    _ => r.next_u32() % 0x20,
                };
                c.bus.write32(0x0043_d950 + 0x64 * index, id);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn crowd_audio_delete_sample_list_matches() {
        check(
            ee::s08::f_002a4be0__generated,
            crowd_audio_delete_sample_list,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_4be0, 0);
                let obj = DATA + 0x1000;
                let count = match seed % 6 {
                    0 => -1i32,
                    1 => 0,
                    n => n as i32 - 2,
                };
                c.bus.write32(obj, count as u32);
                for i in 0..5u32 {
                    c.bus.write32(obj + 4 + 4 * i, 0);
                }
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (seed % 4 == 0) as u64;
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn music_player_play_preview_matches() {
        covered(
            ee::s08::f_002b4978__generated,
            music_player_play_preview,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_4978, 1);
                let p = DATA + 0x1000;
                let slot = (r.next_u32() % 4) as u32;
                let entry = DATA + 0x2000;
                c.bus.write32(p + 4 + 4 * slot, entry);
                c.bus
                    .write32(entry + 0xa0, if seed % 2 == 0 { u32::MAX } else { 1 });
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(p + 0x5440, vt);
                c.g[A0][0] = sx(p);
                c.g[A1][0] = slot as u64;
            },
        );
    }

    #[test]
    fn rider_metrix_clear_hud_messages_matches() {
        check(
            ee::s00::f_001173b8__generated,
            rider_metrix_clear_hud_messages,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_73b8, 0);
                let m = DATA + 0x1000;
                let entries = DATA + 0x2000;
                match seed % 4 {
                    0 => {}
                    1 => c.bus.write32(m + 0x1b0, entries),
                    2 => {
                        c.bus.write32(m + 0x1b0, entries);
                        c.bus.write32(m + 0x1b4, u32::MAX);
                    }
                    _ => {
                        c.bus.write32(m + 0x1b0, entries);
                        c.bus.write32(m + 0x1b4, 1 + r.next_u32() % 6);
                    }
                }
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn rider_force_reset_matches() {
        check(ee::s00::f_001235f8__generated, rider_force_reset, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x12_35f8, 0);
            let rider = DATA + 0x1000;
            c.bus.write32(rider + 0x77c, DATA + 0x2000);
            c.g[A0][0] = sx(rider);
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn rider_gate_sub_can_start_matches() {
        check(
            ee::s00::f_0012c078__generated,
            rider_gate_sub_can_start,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12_c078, 0);
                let world = DATA + 0x4000;
                let mgr = DATA + 0x5000;
                let race = DATA + 0x6000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, mgr);
                c.bus.write32(mgr + 0xc, race);
                c.bus
                    .write32(race, if seed % 3 == 0 { 5 } else { r.next_u32() % 8 });
                let gate = DATA + 0x1000;
                let timer = DATA + 0x2000;
                c.bus.write32(gate + 0x14, timer);
                c.bus.write32(
                    timer + 0x470,
                    match seed % 4 {
                        0 => 0xbf80_0000,
                        1 => 0,
                        _ => r.next_u32(),
                    },
                );
                c.g[A0][0] = sx(gate);
                c.g[A1][0] = r.next_u32() as u64;
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

    #[test]
    fn be_score_int_get_high_score_for_matches() {
        check(
            ee::s01::f_001550e8__generated,
            be_score_int_get_high_score_for,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15_50e8, 0);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = (r.next_u32() % 0x20) as u64;
                c.g[A2][0] = (seed % 0x1a) as u64;
                c.g[A3][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    #[test]
    fn relative_chase_camera_algorithm_ctor_matches() {
        check(
            ee::s01::f_00160b70__generated,
            relative_chase_camera_algorithm_ctor,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16_0b70, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.f[12] = if seed % 5 == 0 {
                    0xbf80_0000
                } else {
                    r.edge_f32()
                };
            },
        );
    }

    #[test]
    fn camera_trigger_man_reset_active_matches() {
        check(
            ee::s02::f_0016c968__generated,
            camera_trigger_man_reset_active,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16_c968, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn counter_update_matches() {
        covered(ee::s11::f_00343820__generated, counter_update, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x34_3820, 1);
            let obj = DATA + 0x1000;
            c.bus
                .write32(obj + 0x1c, if seed % 2 == 0 { 0 } else { r.next_u32() });
            let vt = vtable(c, &mut r, &[0x110]);
            c.bus.write32(obj + 0xc, vt);
            c.g[A0][0] = sx(obj);
        });
    }

    #[test]
    fn dead_fade_ctor_matches() {
        covered(ee::s11::f_00350b98__generated, dead_fade_ctor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x35_0b98, 1);
            let obj = DATA + 0x1000;
            let owner = DATA + 0x3000;
            let vt = vtable(c, &mut r, &[0x10, 0x20]);
            c.bus.write32(owner, vt);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = 0;
            c.g[A2][0] = sx(owner);
        });
    }

    #[test]
    fn uimenu_reset_scroll_matches() {
        check(ee::s13::f_0039b760__generated, uimenu_reset_scroll, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x39_b760, 0);
            let m = DATA + 0x1000;
            c.bus.write32(m + 0x90, r.next_u32());
            c.g[A0][0] = sx(m);
            c.g[A1][0] = (r.next_u32() % 0x100) as u64;
        });
    }

    #[test]
    fn mdf_file_dtor_matches() {
        check(ee::s10::f_00314988__generated, mdf_file_dtor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x31_4988, 0);
            let obj = DATA + 0x1000;
            c.bus
                .write32(obj, if seed % 3 == 0 { DATA + 0x2000 } else { 0 });
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (seed % 4 == 0) as u64;
            let _ = r.next_u32();
        });
    }

    #[test]
    fn mdf_file_free_matches() {
        check(ee::s10::f_003149d0__generated, mdf_file_free, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x31_49d0, 0);
            let obj = DATA + 0x1000;
            c.bus.write32(
                obj,
                match seed % 3 {
                    0 => 0,
                    1 => DATA + 0x2000,
                    _ => 4,
                },
            );
            c.bus.write32(obj + 0xc, r.next_u32());
            c.g[A0][0] = sx(obj);
        });
    }

    #[test]
    fn font_dtor_matches() {
        check(ee::s12::f_003916c0__generated, font_dtor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x39_16c0, 0);
            let obj = DATA + 0x1000;
            c.bus
                .write32(obj + 4, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
            c.bus.write32(obj + 0x60, r.next_u32());
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (seed % 4 == 0) as u64;
        });
    }

    #[test]
    fn section_allocator_dtor_matches() {
        check(
            ee::s13::f_003a77d0__generated,
            section_allocator_dtor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_77d0, 0);
                let obj = DATA + 0x1000;
                let n1 = DATA + 0x2000;
                let n2 = DATA + 0x2100;
                match seed % 3 {
                    0 => {}
                    1 => c.bus.write32(obj + 0xc, n1),
                    _ => {
                        c.bus.write32(obj + 0xc, n1);
                        c.bus.write32(n1 + 0x14, n2);
                    }
                }
                c.bus.write32(obj + 8, 2);
                c.bus.write32(n1, DATA + 0x3000);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (seed % 4 == 0) as u64;
            },
        );
    }

    #[test]
    fn c_world_cache_streamer_dtor_matches() {
        check(
            ee::s13::f_003a8fb8__generated,
            c_world_cache_streamer_dtor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_8fb8, 0);
                let obj = DATA + 0x1000;
                c.bus.write32(obj, r.next_u32());
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn bolt_file_cache_dtor_matches() {
        // The cache teardown frees 10 * 256 slots for real, so keep the seed
        // count small: the body is a straight loop, not data dependent.
        check(ee::s03::f_0019ce20__generated, bolt_file_cache_dtor, 8, &|c, seed| {
            let mut r = machine(c, seed, 0x19_ce20, 0);
            let obj = DATA + 0x1000;
            c.bus.write32(obj, r.next_u32());
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (seed % 4 == 0) as u64;
        });
    }

    #[test]
    fn uv_scroll_modifier_ctor_2_matches() {
        covered(
            ee::s12::f_0035f788__generated,
            uv_scroll_modifier_ctor_2,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_f788, 1);
                let obj = DATA + 0x1000;
                let stream = DATA + 0x2000;
                let vt = vtable(c, &mut r, &[0x10]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    #[test]
    fn magnet_modifier_v18_matches() {
        covered(ee::s12::f_00361a60__generated, magnet_modifier_v18, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x36_1a60, 1);
            let obj = DATA + 0x1000;
            c.bus
                .write32(obj + 0x50, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
            let vt = vtable(c, &mut r, &[0x18]);
            c.bus.write32(obj, vt);
            c.g[A0][0] = sx(obj);
        });
    }

    #[test]
    fn ps2_graphics_man_v59_matches() {
        covered(ee::s12::f_0037cbe8__generated, ps2_graphics_man_v59, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x37_cbe8, 1);
            let self_ = DATA + 0x1000;
            let out = DATA + 0x2000;
            let vt = vtable(c, &mut r, &[0x1a0]);
            c.bus.write32(self_ + 0x10d8, vt);
            c.g[A0][0] = sx(self_);
            c.g[A1][0] = sx(out);
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn ui_object_clear_anim_matches() {
        covered(ee::s13::f_0039fd38__generated, ui_object_clear_anim, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x39_fd38, 1);
            let obj = DATA + 0x1000;
            let anim = DATA + 0x2000;
            if seed % 2 != 0 {
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(anim + 0x10, vt);
                c.bus.write32(obj + 0xc, anim);
            }
            c.g[A0][0] = sx(obj);
            let _ = r.next_u32();
        });
    }

    #[test]
    fn rcmp_decoder_release_codec_matches() {
        covered(
            ee::s13::f_003b0538__generated,
            rcmp_decoder_release_codec,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_0538, 1);
                let obj = DATA + 0x1000;
                let codec = DATA + 0x2000;
                if seed % 2 != 0 {
                    let vt = vtable(c, &mut r, &[8]);
                    c.bus.write32(codec, vt);
                    c.bus.write32(obj + 0x14, codec);
                }
                c.g[A0][0] = sx(obj);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn rcmp_decoder_get_chunk_matches() {
        covered(
            ee::s13::f_003b06b0__generated,
            rcmp_decoder_get_chunk,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_06b0, 1);
                let obj = DATA + 0x1000;
                c.bus
                    .write32(obj, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(obj + 4, r.next_u32());
                c.bus.write32(obj + 8, FAKE);
                c.g[A0][0] = sx(obj);
            },
        );
    }

    #[test]
    fn rcmp_static_init_destroy_matches() {
        check(
            ee::s13::f_003b0770__generated,
            rcmp_static_init_destroy,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_0770, 0);
                c.g[A0][0] = (seed % 3) as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0xffff
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    #[test]
    fn ps2_sony_codec_internal_create_matches() {
        covered(
            ee::s13::f_003b11a0__generated,
            ps2_sony_codec_internal_create,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_11a0, 1);
                c.bus.write32(0x0050_9430, FAKE);
                c.bus.write32(0x0050_9438, r.next_u32());
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn world_get_rider_position_matches() {
        covered(
            ee::s09::f_002d1b58__generated,
            world_get_rider_position,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d_1b58, 1);
                let world = DATA + 0x4000;
                let mgr = DATA + 0x5000;
                let race = DATA + 0x6000;
                let rider = DATA + 0x7000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, mgr);
                c.bus.write32(mgr + 0xc, race);
                let i = (r.next_u32() % 4) as u32;
                c.bus.write32(race + 4 * i + 0x28, rider);
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(rider + 0x6c0, vt);
                c.g[A0][0] = i as u64;
            },
        );
    }

    #[test]
    fn c_cut_scene_transition_man_ctor_matches() {
        covered(
            ee::s09::f_002ea670__generated,
            c_cut_scene_transition_man_ctor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e_a670, 1);
                let obj = DATA + 0x1000;
                let world = DATA + 0x8000;
                c.bus.write32(GAME_GP + 0x2a90, world);
                let vt = vtable(c, &mut r, &[0x150]);
                c.bus.write32(world + 0x10d8, vt);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn wscript_man_get_collected_count_matches() {
        check(
            ee::s10::f_0030ba80__generated,
            wscript_man_get_collected_count,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_ba80, 0);
                // Zeroed BE library: no disc load when the script manager
                // reaches the behaviour-library singleton.
                c.bus.write32(crate::difftest::GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
                let m = DATA + 0x1000;
                let id = (r.next_u32() % 3) as u32;
                c.bus
                    .write32(m + 0x2c0, if seed % 2 == 0 { id } else { id ^ 1 });
                c.bus.write32(m + 0x2c0 + 0x10c, r.next_u32());
                c.g[A0][0] = sx(m);
                c.g[A1][0] = id as u64;
            },
        );
    }

    #[test]
    fn wscript_man_add_collect_items_matches() {
        covered(
            ee::s10::f_0030bac8__generated,
            wscript_man_add_collect_items,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_bac8, 1);
                be_library_interface(c, &mut r);
                let m = DATA + 0x1000;
                let id = (r.next_u32() % 3) as u32;
                c.bus
                    .write32(m + 0x2c0, if seed % 2 == 0 { id } else { id ^ 1 });
                c.g[A0][0] = sx(m);
                c.g[A1][0] = id as u64;
                c.g[A2][0] = r.next_u32() as u64;
                // A non-positive item count makes the collect-set routine bail out
                // before touching the (absent) item table.
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn input_preproc_emit_line_marker_matches() {
        check(
            ee::s10::f_00326430__generated,
            input_preproc_emit_line_marker,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_6430, 0);
                // The line-marker format lives in the game's data image:
                // `# %d "%s"\n` as UTF-16.
                let fmt = [
                    0x23u16, 0x20, 0x25, 0x64, 0x20, 0x22, 0x25, 0x73, 0x22, 0x0a, 0,
                ];
                for (i, w) in fmt.iter().enumerate() {
                    c.bus.write16(0x0048_e3c8 + 2 * i as u32, *w);
                }
                let text = DATA + 0x2000;
                for (i, b) in b"hello world\0".iter().enumerate() {
                    c.bus.write8(text + i as u32, *b);
                }
                let p = DATA + 0x1000;
                c.bus.write32(p, 0);
                c.bus.write32(p + 4, 0x1000);
                c.bus.write32(p + 8, DATA + 0x4000);
                c.g[A0][0] = sx(p);
                c.g[A1][0] = (r.next_u32() % 1000) as u64;
                c.g[A2][0] = sx(text);
            },
        );
    }

    #[test]
    fn ustr_sprintf_matches() {
        check(ee::s08::f_002c26d0__generated, ustr_sprintf, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x2c_26d0, 0);
            let dst = DATA + 0x1000;
            let fmt = DATA + 0x2000;
            let spec = [0x25u16, 0x64, 0x20, 0x25, 0x69, 0x0a, 0];
            for (i, w) in spec.iter().enumerate() {
                c.bus.write16(fmt + 2 * i as u32, *w);
            }
            c.g[A0][0] = sx(dst);
            c.g[A1][0] = sx(fmt);
            c.g[A2][0] = (r.next_u32() % 0x1000) as u64;
            c.g[A3][0] = (r.next_u32() % 0x1000) as u64;
            c.f[12] = r.edge_f32();
        });
    }

    #[test]
    fn mpeg2_extra_bit_information_matches() {
        check(
            ee::s13::f_003b2318__generated,
            mpeg2_extra_bit_information,
            20,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_2318, 0);
                // The global bit reader: an all-zero reservoir, so no extra bit is
                // ever present and the loop body (flush) is not entered.
                let reader = DATA + 0x9000;
                c.bus.write32(0x0050_a088, reader);
                c.bus.write32(reader, DATA + 0xa000);
                c.bus.write32(reader + 4, 0);
                c.bus.write32(reader + 8, 24);
                c.pc = 0x003b_2318;
                c.g[A0][0] = 0;
                let _ = r.next_u32();
            },
        );
    }

    /// A widget at `DATA+0x1000` with (sometimes) an all-zero animation object
    /// at `+0xc`; the animation's `+0` stays null so the evaluator is skipped.
    fn ui_anim_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let obj = DATA + 0x1000;
        let anim = DATA + 0x2000;
        c.bus.write8(obj + 0x10, (r.next_u32() % 0x40) as u8);
        if seed % 2 == 0 {
            c.bus.write32(obj + 0xc, anim);
            c.bus.write16(anim + 0xa, (r.next_u32() % 0x40) as u16);
            c.bus.write32(anim + 8, 0);
        }
        c.g[A0][0] = sx(obj);
        c.g[A1][0] = (r.next_u32() & 0xffff) as u64;
    }

    #[test]
    fn ui_object_update_anim_matches() {
        check(
            ee::s13::f_003a0000__generated,
            ui_object_update_anim,
            40,
            &|c, seed| {
                ui_anim_setup(c, seed, 0x3a_0000);
            },
        );
    }

    #[test]
    fn ui_vector_update_anim_matches() {
        check(
            ee::s13::f_003a3d28__generated,
            ui_vector_update_anim,
            40,
            &|c, seed| {
                ui_anim_setup(c, seed, 0x3a_3d28);
            },
        );
    }

    #[test]
    fn path_plat_calc_track_mem_size_matches() {
        check(
            ee::s08::f_002ae478__generated,
            path_plat_calc_track_mem_size,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_e478, 0);
                c.g[A0][0] = (r.next_u32() % 0x1000) as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => u32::MAX as u64,
                    2 => 1,
                    _ => (r.next_u32() % 0x40) as u64,
                };
            },
        );
    }
}
