//! Phase 2 wave 13 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small constructors, destructors, vtable forwarders, array
//! resets, string helpers and three menu measurement wrappers, each with a
//! differential test against the generated body.
//!
//! Groups:
//!
//! - **Audio** (`0x2a8fa0..0x2b3eb0`): the sound manager / stream array /
//!   speech / song / music-player singletons: allocation wrappers, the
//!   stream-array setters, the per-slot range resets and the music player's
//!   playlist clear.
//! - **Memory card and strings** (`0x2c22c8..0x2c7440`): create/destroy,
//!   request helpers (a `strcpy` plus a card state transition), the
//!   `cPS2MemCard` constructor and `UStr_Len` / `Str_ToLowerInPlace`.
//! - **Menu / stats / effects constructors** (`0x2ccd90..0x2f5998`).
//! - **Script and rider helpers** (`0x308760..0x30eca0`).
//! - **Node constructors and destructors** (`0x344fc0..0x374ca8`): the
//!   `cInstanceNode` / `cMoveNode` derived-class constructors (base ctor,
//!   then the vtable store) and the two deleting destructors.
//! - **Graphics** (`0x370888`, `0x375a08`, `0x382650`).
//!
//! Replacements that call guest code use the [`crate::call`] protocol and
//! reproduce every register the original has at the call (the differential
//! fake callees log them); loops keep the original's back-edge event checks
//! ([`crate::events`]).

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Flow, Frame, VSlot};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002a_8fa0, ps2_sound_manager_create, "PS2SoundManager_Create"),
    (0x002a_ab98, stream_array_set_stream_file, "StreamArray_SetStreamFile"),
    (0x002a_ae40, stream_array_stop, "StreamArray_Stop"),
    (0x002a_d810, bank_monitor_reset_distances, "BankMonitor_ResetDistances"),
    (0x002b_11b0, speech_system_stop_instance, "SpeechSystem_StopInstance"),
    (0x002b_25c8, song_instance_get_request_status, "SongInstance_GetRequestStatus"),
    (0x002b_3b88, music_player_post_event, "MusicPlayer_PostEvent"),
    (0x002b_3d48, music_player_fade_out, "MusicPlayer_FadeOut"),
    (0x002b_3eb0, music_player_clear_playlist, "MusicPlayer_ClearPlaylist"),
    (0x002c_22c8, c_ps2_mem_card_create, "cPS2MemCard_Create"),
    (0x002c_2300, c_ps2_mem_card_destroy, "cPS2MemCard_Destroy"),
    (0x002c_24d0, ustr_len_2, "UStr_Len_2"),
    (0x002c_2868, str_to_lower_in_place, "Str_ToLowerInPlace"),
    (0x002c_3fa8, c_ps2_mem_card_ctor, "cPS2MemCard_ctor"),
    (0x002c_41d8, c_ps2_mem_card_request_list_dir, "cPS2MemCard_RequestListDir"),
    (0x002c_6ce0, c_ps2_mem_card_request_delete, "cPS2MemCard_RequestDelete"),
    (0x002c_7440, c_ps2_console_config_create, "cPS2ConsoleConfig_Create"),
    (0x002c_cd90, menu_sys_submenu_item_measure, "MenuSysSubmenuItem_Measure"),
    (0x002c_cea8, menu_sys_back_item_measure, "MenuSysBackItem_Measure"),
    (0x002c_d2f0, menu_sys_label_item_measure, "MenuSysLabelItem_Measure"),
    (0x002d_2100, ssx_stats_man_create, "SSXStatsMan_Create"),
    (0x002e_4cb0, c_transition_man_reset_effect_pool, "cTransitionMan_ResetEffectPool"),
    (0x002e_caf0, c_bolt_on_fx_interface_clear_emitters, "cBoltOnFXInterface_ClearEmitters"),
    (0x002f_0390, ps2_ai_visual_effects_man_create, "PS2AIVisualEffectsMan_Create"),
    (0x002f_0510, ps2_visual_effects_man_create, "PS2VisualEffectsMan_Create"),
    (0x002f_5998, world_light_man_reset_regions, "WorldLightMan_ResetRegions"),
    (0x002f_6a90, rider_base_load_cheat_code, "RiderBase_LoadCheatCode"),
    (0x0030_8760, wscript_task_save_state, "WScriptTask_SaveState"),
    (0x0030_8798, wscript_task_load_state, "WScriptTask_LoadState"),
    (0x0030_9b70, wscript_man_lookup_func, "WScriptMan_LookupFunc"),
    (0x0030_c700, wscript_man_collect_set_load, "WScriptMan_CollectSetLoad"),
    (0x0030_eca0, rider_model_lock_part, "RiderModel_LockPart"),
    (0x0034_4fc0, crowd_man3d_group_save, "CrowdMan3d_GroupSave"),
    (0x0034_ae68, flag_ctor_2, "Flag_ctor_2"),
    (0x0035_0f08, hide_ctor_2, "Hide_ctor_2"),
    (0x0035_0fd8, restore_node_ctor_2, "RestoreNode_ctor_2"),
    (0x0035_3098, modifier_block_forward_v42, "ModifierBlock_ForwardV42"),
    (0x0035_6e60, object_ctor_2, "Object_ctor_2"),
    (0x0035_7918, particle_ctor, "Particle_ctor"),
    (0x0035_79c8, multi_particle_man_ctor, "MultiParticleMan_ctor"),
    (0x0035_84b8, boost_modifier_base_save, "BoostModifier_BaseSave"),
    (0x0035_da00, roller_modifier_save_body, "RollerModifier_SaveBody"),
    (0x0035_da38, roller_modifier_load_body, "RollerModifier_LoadBody"),
    (0x0036_1778, floating_v45, "Floating_v45"),
    (0x0036_1e80, intc_table_remove_vif_vu_handlers, "IntcTable_RemoveVifVuHandlers"),
    (0x0036_95d8, graphics_man_dtor, "GraphicsMan_dtor"),
    (0x0037_0888, emitter_render_at, "Emitter_RenderAt"),
    (0x0037_4ca8, bezier_man_base_dtor, "BezierManBase_dtor"),
    (0x0037_5a08, ps2_graphics_man_create, "PS2GraphicsMan_Create"),
    (0x0038_2650, renderer_vif1_dmac_handler, "Renderer_Vif1DmacHandler"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    /// The engine's named-block allocator (`cMemMan_alloc`).
    pub const EA_MEM_ALLOC: Callee = Callee { addr: 0x0031_7d70, f: ee::s10::EA_MemAllocNamed };
    pub const BUILTIN_DELETE: Callee = Callee { addr: 0x0031_7e50, f: ee::s10::__builtin_delete };

    pub const PS2_SOUND_MANAGER_CTOR: Callee =
        Callee { addr: 0x002a_8c20, f: ee::s08::PS2SoundManager_ctor };
    pub const STREAM_BUFFER_SET_FILE: Callee =
        Callee { addr: 0x002a_9e78, f: ee::s08::StreamBuffer_SetFile };
    pub const STREAM_BUFFER_STOP: Callee =
        Callee { addr: 0x002a_a408, f: ee::s08::StreamBuffer_Stop };
    pub const SPEECH_INSTANCE_STOP_STREAM: Callee =
        Callee { addr: 0x002b_0320, f: ee::s08::SpeechInstance_StopStream };
    pub const SONG_GET_STREAM_STATUS: Callee =
        Callee { addr: 0x002b_2578, f: ee::s08::SongInstance_GetStreamStatus };
    pub const SONG_POST_EVENT: Callee =
        Callee { addr: 0x002b_20c8, f: ee::s08::SongInstance_PostEvent };
    pub const SONG_FADE_OUT: Callee = Callee { addr: 0x002b_2418, f: ee::s08::SongInstance_FadeOut };

    /// `SNDSTRM_GetRequestStatus` (0x3b7c40, another shard).
    pub const SNDSTRM_GET_REQUEST_STATUS: u32 = 0x003b_7c40;

    pub const MEM_CARD_CTOR: Callee = Callee { addr: 0x002c_3fa8, f: ee::s08::cPS2MemCard_ctor };
    pub const MEM_CARD_INIT: Callee = Callee { addr: 0x002c_4050, f: ee::s08::cPS2MemCard_Init };
    pub const MEM_CARD_SET_STATE: Callee =
        Callee { addr: 0x002c_48c0, f: ee::s08::cPS2MemCard_SetState };
    pub const CONSOLE_CONFIG_CTOR: Callee =
        Callee { addr: 0x002c_7478, f: ee::s08::cPS2ConsoleConfig_ctor };

    /// `strcpy` (0x4166f4, s15).
    pub const STRCPY: Callee = Callee { addr: 0x0041_66f4, f: ee::s15::strcpy };

    pub const MENU_ITEM_MEASURE_ROW: Callee =
        Callee { addr: 0x002c_a4c8, f: ee::s09::MenuSysItem_MeasureRow };
    pub const STATS_MAN_CTOR: Callee = Callee { addr: 0x002d_21c0, f: ee::s09::SSXStatsMan_ctor };
    pub const AI_VFX_MAN_CTOR: Callee =
        Callee { addr: 0x002f_03c8, f: ee::s09::PS2AIVisualEffectsMan_ctor };
    pub const VFX_MAN_CTOR: Callee =
        Callee { addr: 0x002f_0548, f: ee::s09::PS2VisualEffectsMan_ctor };

    /// `World_GetSectionScriptFunc` (0x3a6d18, s13).
    pub const WORLD_GET_SECTION_SCRIPT_FUNC: u32 = 0x003a_6d18;

    pub const WORLD_OBJ_CTOR: Callee = Callee { addr: 0x0034_fb90, f: ee::s11::WorldObj_ctor_2 };
    pub const MOD_OBJ_CTOR: Callee = Callee { addr: 0x0035_52d8, f: ee::s11::ModObj_ctor_2 };
    pub const GET_TRANSFORM_BOUNDS: Callee =
        Callee { addr: 0x0035_2b88, f: ee::s11::ModifierBlock_GetTransformBounds };

    pub const INTC_REMOVE_HANDLER: Callee =
        Callee { addr: 0x0036_2008, f: ee::s12::IntcTable_RemoveHandler };
    pub const EMITTER_RENDER: Callee = Callee { addr: 0x0037_08c0, f: ee::s12::Emitter_Render };
    pub const GRAPHICS_ON_VIF1_DMA_END: Callee =
        Callee { addr: 0x0038_2688, f: ee::s12::PS2GraphicsMan_OnVif1DmaEnd };
    /// `PS2GraphicsMan_ctor` (0x395288, s13).
    pub const GRAPHICS_MAN_CTOR: u32 = 0x0039_5288;
}

// ---------------------------------------------------------------------------
// Allocating constructors.

/// `new (size, name, flags)` then construct: the shape of every `Create` in
/// this module (`EA_MemAllocNamed` then the class constructor).
fn create_with<F>(c: &mut Cpu, size: u32, name: u32, flags: u32, ret: u32, ctor: F) -> Flow
where
    F: FnOnce(&mut Cpu, u32) -> Flow,
{
    c.cycles += 13;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = sx(name);
    c.g[A2][0] = sx(flags);
    c.g[A3][0] = 0;
    c.g[A0][0] = sx(size);
    callee::EA_MEM_ALLOC.call(c, ret)?;
    c.g[A0][0] = c.g[V0][0];
    ctor(c, ret.wrapping_add(8))?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void* PS2SoundManager_Create()` (0x2a8fa0): allocate the 0x320-byte sound
/// manager and construct it.
pub fn ps2_sound_manager_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 0x320, 0x0048_3168, 0, 0x002a_8fc0, |c, ret| {
            callee::PS2_SOUND_MANAGER_CTOR.call(c, ret)
        })
    })
}

// ---------------------------------------------------------------------------
// Stream array / speech / song / music player.

/// `void StreamArray_SetStreamFile(StreamArray* a, int i, const char* file)`
/// (0x2aab98): call the setter of stream slot `i` if it exists.
///
/// Leaves the slot list base in `v0`.
pub fn stream_array_set_stream_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let i = r32(c, A1) << 2;
        frame.save_ra(c, 0);
        let base = Addr(abi::arg(c, 0)).off(8).lw(c);
        c.g[V0][0] = base;
        c.g[A1][0] = sx(i.wrapping_add(base as u32));
        c.g[A0][0] = Addr(i.wrapping_add(base as u32)).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            c.g[A1][0] = c.g[A2][0];
            callee::STREAM_BUFFER_SET_FILE.call(c, 0x002a_abc0)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void StreamArray_Stop(StreamArray* a, int i)` (0x2aae40): stop stream slot
/// `i` if it exists. Leaves the slot list base in `v0`.
pub fn stream_array_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let i = r32(c, A1) << 2;
        frame.save_ra(c, 0);
        let base = Addr(abi::arg(c, 0)).off(8).lw(c);
        c.g[V0][0] = base;
        c.g[A1][0] = sx(i.wrapping_add(base as u32));
        c.g[A0][0] = Addr(i.wrapping_add(base as u32)).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            callee::STREAM_BUFFER_STOP.call(c, 0x002a_ae68)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void SpeechSystem_StopInstance(SpeechSystem* s, int i)` (0x2b11b0): stop
/// speech instance `i` if it exists. Leaves the instance list base in `v0`.
pub fn speech_system_stop_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let i = r32(c, A1) << 2;
        frame.save_ra(c, 0);
        let base = Addr(abi::arg(c, 0)).off(4).lw(c);
        c.g[V0][0] = base;
        c.g[A1][0] = sx(i.wrapping_add(base as u32));
        c.g[A0][0] = Addr(i.wrapping_add(base as u32)).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            callee::SPEECH_INSTANCE_STOP_STREAM.call(c, 0x002b_11d8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_ResetDistances(BankMonitor* bm)` (0x2ad810): set the
/// distance of all 64 voices to -1.0.
pub fn bank_monitor_reset_distances(c: &mut Cpu) {
    const HEAD: u32 = 0x002a_d820;
    c.cycles += 4;
    // `lui at, 0xbf80; mtc1 at, f0`
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 0);
    c.g[A1][0] = 0;
    c.g[V1][0] = 0x3f;
    loop {
        c.cycles += 7;
        let voices = Addr(abi::arg(c, 0)).off(0x8ec).lw(c);
        c.g[V0][0] = voices;
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(voices as u32));
        c.swc1(0, r32(c, V0).wrapping_add(0x5c));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0xc0));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void* SongInstance_GetRequestStatus(SongInstance* s, sRequestStatus* v)`
/// (0x2b25c8): fill in the song's stream status, then ask the stream driver
/// for its request status.
pub fn song_instance_get_request_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let v = c.g[A1][0];
        c.g[S0][0] = v;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx(frame.at(c, 0));
        callee::SONG_GET_STREAM_STATUS.call(c, 0x002b_25e0)?;
        c.g[A0][0] = Addr(frame.at(c, 4)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, callee::SNDSTRM_GET_REQUEST_STATUS, 0x002b_25ec)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_PostEvent(MusicPlayer* mp, int event)` (0x2b3b88): post
/// to the current song, if the player has one.
pub fn music_player_post_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let mp = abi::arg(c, 0);
        c.g[V0][0] = Addr(mp).off(0x418).lw(c);
        let playing = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if playing {
            c.cycles += 3;
            c.g[A0][0] = Addr(mp).off(0x408).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A2][0] = 0;
                callee::SONG_POST_EVENT.call(c, 0x002b_3bb0)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_FadeOut(MusicPlayer* mp, float v)` (0x2b3d48): fade the
/// current song out, if the player has one.
pub fn music_player_fade_out(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let mp = abi::arg(c, 0);
        c.g[V0][0] = Addr(mp).off(0x418).lw(c);
        let playing = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if playing {
            c.cycles += 3;
            c.g[A0][0] = Addr(mp).off(0x408).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                callee::SONG_FADE_OUT.call(c, 0x002b_3d70)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_ClearPlaylist(MusicPlayer* mp)` (0x2b3eb0): clear the
/// playlist pointers, the current index and the five pending entries.
pub fn music_player_clear_playlist(c: &mut Cpu) {
    const HEAD: u32 = 0x002b_3ec0;
    c.cycles += 4;
    let mp = abi::arg(c, 0);
    Addr(mp).off(0x3f8).sd(c, 0);
    c.g[V1][0] = 4;
    Addr(mp).off(0x3e8).sw(c, 0);
    c.g[V0][0] = sx(mp.wrapping_add(0x930));
    loop {
        c.cycles += 7;
        Addr(r32(c, V0)).sw(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// cPS2MemCard.

/// `void* cPS2MemCard_Create()` (0x2c22c8): allocate the 0x1500-byte memory
/// card object (`0x100` alignment) and construct it.
pub fn c_ps2_mem_card_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 0x1500, 0x0048_5ff0, 0x100, 0x002c_22e8, |c, ret| {
            callee::MEM_CARD_CTOR.call(c, ret)
        })
    })
}

/// `void cPS2MemCard_Destroy(cPS2MemCard* card)` (0x2c2300): the deleting
/// destructor through the object's vtable (slot +8, flag 3).
pub fn c_ps2_mem_card_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let obj = c.g[A0][0];
        c.g[A2][0] = obj;
        frame.save_ra(c, 0);
        if obj != 0 {
            c.cycles += 6;
            let vt = Addr(obj as u32).lw(c);
            c.g[V0][0] = vt;
            c.g[A1][0] = 3;
            let m = VSlot::read(c, vt, 8);
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(obj);
            call_ptr(c, m.func as u32, 0x002c_2328)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cPS2MemCard_ctor(cPS2MemCard* card)` (0x2c3fa8): install the class
/// vtable and initialise the card state.
pub fn c_ps2_mem_card_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx(0x0048_6f78);
        frame.save(c, S0, 0x10);
        let obj = c.g[A0][0];
        c.g[S0][0] = obj;
        frame.save_ra(c, 0);
        Addr(obj as u32).sw(c, c.g[V0][0]);
        callee::MEM_CARD_INIT.call(c, 0x002c_3fc8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_RequestListDir(cPS2MemCard* card, const char* name)`
/// (0x2c41d8): save the directory name and switch the card to state 5.
pub fn c_ps2_mem_card_request_list_dir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let obj = c.g[A0][0];
        c.g[S0][0] = obj;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((obj as u32).wrapping_add(0x139));
        callee::STRCPY.call(c, 0x002c_41f0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 5;
        callee::MEM_CARD_SET_STATE.call(c, 0x002c_41fc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_RequestDelete(cPS2MemCard* card, const char* name)`
/// (0x2c6ce0): save the name and switch the card to state 0x4d (delete).
pub fn c_ps2_mem_card_request_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let obj = c.g[A0][0];
        c.g[S0][0] = obj;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((obj as u32).wrapping_add(0xf9));
        callee::STRCPY.call(c, 0x002c_6cf8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x4d;
        callee::MEM_CARD_SET_STATE.call(c, 0x002c_6d04)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cPS2ConsoleConfig_Create()` (0x2c7440): allocate the 0x38-byte
/// console configuration and construct it.
pub fn c_ps2_console_config_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 0x38, 0x0048_6560, 0, 0x002c_7460, |c, ret| {
            callee::CONSOLE_CONFIG_CTOR.call(c, ret)
        })
    })
}

// ---------------------------------------------------------------------------
// Strings.

/// `int UStr_Len_2(const u16* s)` (0x2c24d0): the length in UTF-16 units
/// (`lhu`, two bytes per unit), counting the first unit and then while the
/// units are non-zero.
pub fn ustr_len_2(c: &mut Cpu) {
    const HEAD: u32 = 0x002c_24e0;
    c.cycles += 4;
    let mut p = abi::arg(c, 0);
    let mut ch = c.lhu(p);
    c.g[V0][0] = ch;
    let mut n = 0u32;
    c.g[V1][0] = 0;
    p = p.wrapping_add(2);
    c.g[A0][0] = sx(p);
    if ch != 0 {
        loop {
            c.cycles += 7;
            ch = c.lhu(p);
            c.g[V0][0] = ch;
            n = n.wrapping_add(1);
            c.g[V1][0] = sx(n);
            let more = ch != 0;
            p = p.wrapping_add(2);
            c.g[A0][0] = sx(p);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
                p = r32(c, A0);
                n = r32(c, V1);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

/// `void Str_ToLowerInPlace(char* s)` (0x2c2868): lower-case the ASCII
/// letters of a zero-terminated string.
///
/// The original is a dispatcher: it tests the byte, folds `A`-`Z`, stores it
/// back and loads the next one, and an unwind after the back-edge event check
/// re-enters it at the folding block with the current byte in `v1` and the
/// pointer in `a0`.
pub fn str_to_lower_in_place(c: &mut Cpu) {
    const HEAD: u32 = 0x002c_2870;
    // Re-entered at the folding block after an event asked the thread to
    // unwind (`pc` is the loop head, `a0`/`v1` hold the loop state).
    let mut resumed = c.pc == HEAD;
    if !resumed {
        c.cycles += 2;
        c.g[V0][0] = Addr(abi::arg(c, 0)).lb(c);
    }
    loop {
        if !resumed {
            // The zero test and the (unsigned) reload of the current byte.
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = Addr(r32(c, A0)).lbu(c);
            if !more {
                break;
            }
            if back_edge_unwinds(c, HEAD) {
                return;
            }
        }
        resumed = false;
        // Fold `A`-`Z` (`v1 - 'A' < 26`), leaving `v1 + 0x20` in v0.
        c.cycles += 4;
        c.g[V0][0] = sx((r32(c, V1).wrapping_sub(0x41) < 0x1a) as u32);
        let upper = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x20));
        if upper {
            c.cycles += 1;
            Addr(r32(c, A0)).sb(c, c.g[V0][0]);
        }
        // Advance and load the next byte.
        c.cycles += 2;
        let p = r32(c, A0).wrapping_add(1);
        c.g[A0][0] = sx(p);
        c.g[V0][0] = Addr(p).lb(c);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Menu item measurement.

/// `MenuSysItem* Measure(MenuSysItem* self, sItemLayout* layout)`
/// (0x2ccd90 / 0x2ccea8 / 0x2cd2f0): measure the row with the layout's row
/// count and return `self`.
fn menu_item_measure(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x20);
    c.g[A2][0] = 0;
    frame.save(c, S0, 0x10);
    c.g[T0][0] = 0;
    frame.save_ra(c, 0);
    let item = c.g[A0][0];
    c.g[S0][0] = item;
    c.g[A3][0] = Addr(abi::arg(c, 1)).off(0x14).lw(c);
    callee::MENU_ITEM_MEASURE_ROW.call(c, ret)?;
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `MenuSysSubmenuItem_Measure` (0x2ccd90).
pub fn menu_sys_submenu_item_measure(c: &mut Cpu) {
    guest(c, |c| menu_item_measure(c, 0x002c_cdb0))
}

/// `MenuSysBackItem_Measure` (0x2ccea8).
pub fn menu_sys_back_item_measure(c: &mut Cpu) {
    guest(c, |c| menu_item_measure(c, 0x002c_cec8))
}

/// `MenuSysLabelItem_Measure` (0x2cd2f0).
pub fn menu_sys_label_item_measure(c: &mut Cpu) {
    guest(c, |c| menu_item_measure(c, 0x002c_d310))
}

// ---------------------------------------------------------------------------
// Stats and visual effects constructors.

/// `void* SSXStatsMan_Create()` (0x2d2100): allocate the one-byte stats
/// manager singleton and construct it.
pub fn ssx_stats_man_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 1, 0x0048_6758, 0, 0x002d_2120, |c, ret| {
            callee::STATS_MAN_CTOR.call(c, ret)
        })
    })
}

/// `void* PS2AIVisualEffectsMan_Create()` (0x2f0390): allocate the 8-byte AI
/// effects manager and construct it.
pub fn ps2_ai_visual_effects_man_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 8, 0x0048_7b88, 0, 0x002f_03b0, |c, ret| {
            callee::AI_VFX_MAN_CTOR.call(c, ret)
        })
    })
}

/// `void* PS2VisualEffectsMan_Create()` (0x2f0510): allocate the 0x48-byte
/// visual effects manager and construct it.
pub fn ps2_visual_effects_man_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 0x48, 0x0048_7ba0, 0, 0x002f_0530, |c, ret| {
            callee::VFX_MAN_CTOR.call(c, ret)
        })
    })
}

/// `void cTransitionMan_ResetEffectPool()` (0x2e4cb0): clear the four global
/// transition effect entries.
pub fn c_transition_man_reset_effect_pool(c: &mut Cpu) {
    const HEAD: u32 = 0x002e_4cc0;
    c.cycles += 4;
    c.g[V0][0] = 0x0054_0000;
    c.g[V1][0] = 3;
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x77b0));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xc));
    loop {
        c.cycles += 7;
        Addr(r32(c, V0)).sw(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void cBoltOnFXInterface_ClearEmitters(cBoltOnFXInterface* fx)`
/// (0x2ecaf0): invalidate the 15 emitter slots (`id = -1`).
pub fn c_bolt_on_fx_interface_clear_emitters(c: &mut Cpu) {
    const HEAD: u32 = 0x002e_cb00;
    c.cycles += 4;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0xb8));
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = 0xe;
    loop {
        c.cycles += 7;
        Addr(r32(c, A0)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0xc));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void WorldLightMan_ResetRegions(WorldLightMan* man)` (0x2f5998): set the
/// eight region records to -1.
pub fn world_light_man_reset_regions(c: &mut Cpu) {
    const HEAD: u32 = 0x002f_59a8;
    c.cycles += 4;
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x10));
    c.g[A1][0] = 0;
    c.g[V1][0] |= 0xffff;
    loop {
        c.cycles += 7;
        Addr(r32(c, A0)).sw(c, c.g[V1][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        c.g[V0][0] = (c.g[A1][0] < 8) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x40));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void MultiParticleMan_ctor(MultiParticleMan* man)` (0x3579c8): publish the
/// singleton and clear its four lists.
pub fn multi_particle_man_ctor(c: &mut Cpu) {
    const HEAD: u32 = 0x0035_79d8;
    c.cycles += 4;
    let man = abi::arg(c, 0);
    c.g[V0][0] = sx(man);
    c.g[A1][0] = 3;
    c.sw(r32(c, GP).wrapping_add(0xf38), c.g[V0][0]);
    c.g[V1][0] = sx(man.wrapping_add(0xc));
    loop {
        c.cycles += 7;
        Addr(r32(c, V1)).sw(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script and rider helpers.

/// `void WScriptMan_LookupFunc(WScriptMan* man, int id)` (0x309b70): resolve
/// a script function id through the man's cache, or return 0.
pub fn wscript_man_lookup_func(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        let id = c.g[A1][0];
        c.g[V0][0] = !(0u64 | id);
        let wanted = c.g[V0][0] != 0;
        c.sw(frame.at(c, 0), c.g[A1][0]);
        if wanted {
            c.cycles += 3;
            c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x28c).lw(c);
            c.g[A1][0] = sx(frame.at(c, 0));
            call_ptr(c, callee::WORLD_GET_SECTION_SCRIPT_FUNC, 0x0030_9b98)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderModel_LockPart(RiderModel* model, int i)` (0x30eca0): OR the
/// mask of part `partIndex[i]` into the model's locked-part mask.
pub fn rider_model_lock_part(c: &mut Cpu) {
    c.cycles += 13;
    let model = abi::arg(c, 0);
    let i = r32(c, A1) << 2;
    c.g[V0][0] = Addr(model).off(0x1c).lw(c);
    c.g[V1][0] = 0x58;
    c.g[A2][0] = Addr(model).off(0xc).lw(c);
    c.g[A1][0] = sx(i.wrapping_add(r32(c, V0)));
    c.g[A3][0] = Addr(model).off(0x158).ld(c);
    c.g[V0][0] = Addr(r32(c, A1)).lw(c);
    c.mult(0, V0, V0, V1);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
    c.g[V1][0] = Addr(r32(c, A2)).off(0x10).ld(c);
    c.g[A3][0] = c.g[A3][0] | c.g[V1][0];
    c.sd(model.wrapping_add(0x158), c.g[A3][0]);
    abi::ret(c);
}

/// `int Floating_v45(Floating* self)` (0x361778): the floating transform's id,
/// either from the bound modifier block or the cached word.
pub fn floating_v45(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let this = c.g[A0][0];
        c.g[V0][0] = this;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(this as u32).off(0x1c).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            callee::GET_TRANSFORM_BOUNDS.call(c, 0x0036_17a0)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(this as u32).off(0x78).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Vtable forwarders: `obj->slot(a1, a2)` where the vtable pointer is the
// first word of the argument object.

/// Forward to slot `slot` of the vtable found at the first word of `a1`.
/// `a1_off` selects `a1 = self + off` (else `a1 = self`).
fn forward_to_arg_vtable(c: &mut Cpu, ret: u32, slot: u32, a2: u32, a1_off: Option<u32>) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x10);
    let target = c.g[A1][0];
    c.g[A3][0] = target;
    frame.save_ra(c, 0);
    c.g[A1][0] = match a1_off {
        Some(off) => sx(r32(c, A0).wrapping_add(off)),
        None => c.g[A0][0],
    };
    c.g[A2][0] = sx(a2);
    let vt = Addr(target as u32).lw(c);
    c.g[V1][0] = vt;
    let m = VSlot::read(c, vt, slot);
    c.g[V0][0] = m.func;
    c.g[A0][0] = m.this(target);
    call_ptr(c, m.func as u32, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void RiderBase_LoadCheatCode(RiderBase* self, const sCheatCode* code)`
/// (0x2f6a90): read the code's 4-byte entries into `self + 4`.
pub fn rider_base_load_cheat_code(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x002f_6ab8, 0x10, 4, Some(4)))
}

/// `void WScriptTask_SaveState(WScriptTask* task, sWScriptStream* stream)`
/// (0x308760).
pub fn wscript_task_save_state(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x0030_8788, 8, 4, None))
}

/// `void WScriptTask_LoadState(WScriptTask* task, sWScriptStream* stream)`
/// (0x308798).
pub fn wscript_task_load_state(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x0030_87c0, 0x10, 4, None))
}

/// `void WScriptMan_CollectSetLoad(WScriptMan* man, sWScriptStream* stream)`
/// (0x30c700).
pub fn wscript_man_collect_set_load(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x0030_c728, 0x10, 0x10c, None))
}

/// `void CrowdMan3d_GroupSave(CrowdMan3d* man, sWScriptStream* stream)`
/// (0x344fc0).
pub fn crowd_man3d_group_save(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x0034_4fe8, 8, 0x10c, None))
}

/// `void BoostModifier_BaseSave(BoostModifier* mod, sWScriptStream* stream)`
/// (0x3584b8).
pub fn boost_modifier_base_save(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x0035_84e0, 8, 4, None))
}

/// `void RollerModifier_SaveBody(RollerModifier* mod, sWScriptStream* stream)`
/// (0x35da00).
pub fn roller_modifier_save_body(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x0035_da28, 8, 0x50, None))
}

/// `void RollerModifier_LoadBody(RollerModifier* mod, sWScriptStream* stream)`
/// (0x35da38).
pub fn roller_modifier_load_body(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x0035_da60, 0x10, 0x50, None))
}

/// `void ModifierBlock_ForwardV42(ModifierBlock* block, int arg)`
/// (0x353098): call the wrapped node's `v02(arg)`, if there is a node.
pub fn modifier_block_forward_v42(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(abi::arg(c, 0)).lw(c);
        let node = c.g[A2][0];
        frame.restore_ra(c, 0);
        if node != 0 {
            c.cycles += 6;
            let vt = Addr(node as u32).lw(c);
            c.g[V0][0] = vt;
            let m = VSlot::read(c, vt, 0xb0);
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(node);
            call_ptr(c, m.func as u32, 0x0035_30c0)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Derived-class constructors: base constructor, then the class vtable.

/// `self = BaseCtor(self); self->vptr = vtable; return self`: the shape of the
/// five node constructors (0x34ae68, 0x350f08, 0x350fd8, 0x356e60, 0x357918).
fn ctor_with_vtable(c: &mut Cpu, ret: u32, base: Callee, vtable: u32) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    let obj = c.g[A0][0];
    c.g[S0][0] = obj;
    base.call(c, ret)?;
    c.g[V1][0] = sx(vtable);
    c.g[V0][0] = c.g[S0][0];
    frame.restore_ra(c, 0);
    Addr(obj as u32).off(0xc).sw(c, c.g[V1][0]);
    frame.restore(c, S0, 0x10);
    frame.close(c);
    Ok(())
}

/// `void* Flag_ctor_2(Flag* self)` (0x34ae68).
pub fn flag_ctor_2(c: &mut Cpu) {
    guest(c, |c| ctor_with_vtable(c, 0x0034_ae7c, callee::WORLD_OBJ_CTOR, 0x0048_fc10))
}

/// `void* Hide_ctor_2(Hide* self)` (0x350f08).
pub fn hide_ctor_2(c: &mut Cpu) {
    guest(c, |c| ctor_with_vtable(c, 0x0035_0f1c, callee::WORLD_OBJ_CTOR, 0x0049_1800))
}

/// `void* RestoreNode_ctor_2(RestoreNode* self)` (0x350fd8).
pub fn restore_node_ctor_2(c: &mut Cpu) {
    guest(c, |c| ctor_with_vtable(c, 0x0035_0fec, callee::WORLD_OBJ_CTOR, 0x0049_1680))
}

/// `void* Object_ctor_2(Object* self)` (0x356e60).
pub fn object_ctor_2(c: &mut Cpu) {
    guest(c, |c| ctor_with_vtable(c, 0x0035_6e74, callee::MOD_OBJ_CTOR, 0x0049_0e80))
}

/// `void* Particle_ctor(Particle* self)` (0x357918).
pub fn particle_ctor(c: &mut Cpu) {
    guest(c, |c| ctor_with_vtable(c, 0x0035_792c, callee::MOD_OBJ_CTOR, 0x0048_ee60))
}

/// `void IntcTable_RemoveVifVuHandlers(IntcTable* table)` (0x361e80): remove
/// the VIF1 (5) and VU1 (7) interrupt handlers.
pub fn intc_table_remove_vif_vu_handlers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 5;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let table = c.g[A0][0];
        c.g[S0][0] = table;
        callee::INTC_REMOVE_HANDLER.call(c, 0x0036_1e98)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 7;
        callee::INTC_REMOVE_HANDLER.call(c, 0x0036_1ea4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The deleting-destructor prologue: install the class vtable, clear the
/// class's singleton global, then `delete this` when `flags & 1`.
fn deleting_dtor(c: &mut Cpu, ret: u32, vtable_slot: u32, vtable: u32, gp_slot: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    let obj = c.g[A0][0];
    c.g[V0][0] = sx(vtable);
    frame.save_ra(c, 0);
    Addr(obj as u32).off(vtable_slot).sw(c, c.g[V0][0]);
    c.g[A1][0] &= 1;
    let owned = c.g[A1][0] != 0;
    c.sw(r32(c, GP).wrapping_add(gp_slot), 0);
    if owned {
        c.cycles += 2;
        callee::BUILTIN_DELETE.call(c, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `GraphicsMan::~GraphicsMan()` (0x3695d8): the base graphics manager
/// destructor (`operator delete` when the low flag bit is set).
pub fn graphics_man_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0036_9600, 0x10d8, 0x0049_3950, 0x2a90))
}

/// `BezierManBase::~BezierManBase()` (0x374ca8): the bezier manager base
/// destructor (`operator delete` when the low flag bit is set).
pub fn bezier_man_base_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0037_4cd0, 4, 0x0049_3820, 0x1158))
}

// ---------------------------------------------------------------------------
// Graphics.

/// `void Emitter_RenderAt(Emitter* e, sMPBox* box, sMPVec4* center)`
/// (0x370888): copy the box and centre into the descriptor's quad slots and
/// render.
pub fn emitter_render_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let self_ = abi::arg(c, 0);
        let bbox = abi::arg(c, 1);
        let center = abi::arg(c, 2);
        c.lq(3, bbox);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = sx(self_);
        c.sq(3, self_.wrapping_add(0x160));
        c.lq(2, bbox.wrapping_add(0x10));
        c.sq(2, self_.wrapping_add(0x170));
        c.lq(3, center);
        c.sq(3, self_.wrapping_add(0xc0));
        callee::EMITTER_RENDER.call(c, 0x0037_08b0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* PS2GraphicsMan_Create()` (0x375a08): allocate the 0x75e0-byte
/// graphics manager and construct it.
pub fn ps2_graphics_man_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 0x75e0, 0x0049_2878, 0, 0x0037_5a28, |c, ret| {
            call_ptr(c, callee::GRAPHICS_MAN_CTOR, ret)
        })
    })
}

/// `int Renderer_Vif1DmacHandler(int cause, PS2GraphicsMan* man)` (0x382650):
/// the VIF1 DMA-end interrupt handler: finish the DMA transfer and re-enable
/// interrupts; any other cause is a fatal assertion in the original.
pub fn renderer_vif1_dmac_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        let dma_end = c.g[A0][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if dma_end {
            c.cycles += 8;
            c.g[A0][0] = c.g[A1][0];
            callee::GRAPHICS_ON_VIF1_DMA_END.call(c, 0x0038_266c)?;
            c.ei();
            frame.restore_ra(c, 0);
            c.g[V0][0] = 1;
            frame.close(c);
        } else {
            c.cycles += 1;
            c.brk(0x0038_2660, 0);
        }
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{machine, FAKE};
    // `RA`/`SP` from `reg` are the register *indices* used all over the
    // tests; the difftest sentinel return address and stack top get their own
    // names.
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP, RA as SENTINEL_RA, SP as STACK_TOP};

    /// The engine's size-class pools (`MEM_PoolAlloc`): allocations up to
    /// 0x200 bytes are popped from per-class free lists hanging off
    /// `gp + 0x2a60..`, but only when the "pools ready" flag at `gp + 0xdd8`
    /// is set; otherwise `MEM_PoolAlloc` forwards to `MEM_AllocInternal`, which
    /// asks the pools again — an endless recursion on a fresh test machine.
    fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1); // pools ready
        // (pool pointer global, free-list head offset within the pool object,
        // block size of the class)
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),     // 8-byte blocks
            (0x2a60, 0x2000, 0x10), // 16
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes;
            // 16 free nodes per class, spaced by the block size (the head is
            // updated on pop, so only an unpopped node's next pointer matters).
            for n in 0..16u32 {
                let node = base + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    /// The main heap's dlmalloc arena over 256 KiB of test RAM, as
    /// `containers.rs`/`oc59` set up for the allocator tests: allocations
    /// larger than 0x200 bytes come from here.
    const MEM_HEAP: u32 = 0x004f_f250;

    fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    fn heap_init(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000],
        );
    }

    /// A real block of `size` bytes from the main heap (for the deleting
    /// destructors, whose `operator delete` must see a valid chunk).
    fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0032_0058,
            ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    /// A machine for functions that allocate: fake callees for the misses,
    /// the pool free lists and a real heap arena.
    fn alloc_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        pool_allocator(c);
        heap_init(c);
        r
    }

    /// A vtable at a fixed test address with `{delta, fake}` at each slot.
    fn put_vtable(c: &mut Cpu, r: &mut Rng, at: u32, slots: &[u32]) -> u32 {
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(at + off, delta as u16);
            c.bus.write32(at + off + 4, FAKE + 0x10 * i as u32);
        }
        at
    }

    // ------------------------------------------------------------- audio

    #[test]
    fn ps2_sound_manager_create_matches() {
        check(ee::s08::f_002a8fa0__generated, ps2_sound_manager_create, 40, &|c, seed| {
            alloc_machine(c, seed, 0x2a8f_a0);
        });
    }

    #[test]
    fn stream_array_set_stream_file_matches() {
        check(ee::s08::f_002aab98__generated, stream_array_set_stream_file, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x2aab_98, 0);
            let arr = DATA + 0x100;
            c.g[A0][0] = sx(arr);
            c.g[A1][0] = sx(seed % 16);
            c.g[A2][0] = sx(DATA + 0x300);
            // The slot list lives at arr + 8; the slots are arr + 0x200 + i*4.
            c.bus.write32(arr + 8, DATA + 0x200);
            let slot = DATA + 0x200 + 4 * (seed % 16);
            let entry = if seed % 3 == 0 { 0 } else { DATA + 0x320 };
            c.bus.write32(slot, entry);
            fill(c, &mut r, DATA + 0x300, 8);
        });
    }

    #[test]
    fn stream_array_stop_matches() {
        check(ee::s08::f_002aae40__generated, stream_array_stop, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x2aae_40, 0);
            let arr = DATA + 0x100;
            c.g[A0][0] = sx(arr);
            c.g[A1][0] = sx(seed % 16);
            c.bus.write32(arr + 8, DATA + 0x200);
            let slot = DATA + 0x200 + 4 * (seed % 16);
            let entry = if seed % 3 == 0 { 0 } else { DATA + 0x340 };
            c.bus.write32(slot, entry);
            fill(c, &mut r, DATA + 0x340, 8);
        });
    }

    #[test]
    fn bank_monitor_reset_distances_matches() {
        check(ee::s08::f_002ad810__generated, bank_monitor_reset_distances, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2ad8_10, 0);
            let voices = DATA + 0x1000;
            c.g[A0][0] = sx(DATA + 0x100);
            c.bus.write32(DATA + 0x100 + 0x8ec, voices);
            fill(c, &mut r, voices, 64 * 0x30);
        });
    }

    #[test]
    fn speech_system_stop_instance_matches() {
        check(ee::s08::f_002b11b0__generated, speech_system_stop_instance, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x2b11_b0, 0);
            let sys = DATA + 0x100;
            c.g[A0][0] = sx(sys);
            c.g[A1][0] = sx(seed % 16);
            c.bus.write32(sys + 4, DATA + 0x200);
            let slot = DATA + 0x200 + 4 * (seed % 16);
            let entry = if seed % 3 == 0 { 0 } else { DATA + 0x360 };
            c.bus.write32(slot, entry);
            fill(c, &mut r, DATA + 0x360, 8);
        });
    }

    #[test]
    fn song_instance_get_request_status_matches() {
        check(ee::s08::f_002b25c8__generated, song_instance_get_request_status, 80, &|c, seed| {
            let _r = machine(c, seed, 0x2b25_c8, 0);
            c.g[A0][0] = sx(DATA + 0x100);
            c.g[A1][0] = sx(DATA + 0x400);
            // The stream id the wrapper reloads from its frame (`lw(sp + 4)`):
            // negative means "no stream" to SNDSTRM_GetRequestStatus.
            c.bus.write32(STACK_TOP - 0x30 + 4, 0xffff_ffff);
            let _ = seed;
        });
    }

    #[test]
    fn music_player_post_event_matches() {
        check(ee::s08::f_002b3b88__generated, music_player_post_event, 120, &|c, seed| {
            let _r = machine(c, seed, 0x2b3b_88, 0);
            let mp = DATA + 0x100;
            let song = DATA + 0x300;
            c.g[A0][0] = sx(mp);
            c.g[A1][0] = sx(seed * 3);
            let has_song = seed % 3 != 0;
            let active = seed % 2 == 0;
            c.bus.write32(mp + 0x418, if active { 1 } else { 0 });
            c.bus.write32(mp + 0x408, if has_song { song } else { 0 });
            // A song with no pending events and no queue: the post returns
            // after its first test.
            for i in 0..8u32 {
                c.bus.write32(song + 4 * i, 0);
            }
        });
    }

    #[test]
    fn music_player_fade_out_matches() {
        check(ee::s08::f_002b3d48__generated, music_player_fade_out, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x2b3d_48, 0);
            let mp = DATA + 0x100;
            let song = DATA + 0x300;
            c.g[A0][0] = sx(mp);
            c.g[A1][0] = sx(r.edge_f32());
            let has_song = seed % 3 != 0;
            let active = seed % 2 == 0;
            c.bus.write32(mp + 0x418, if active { 1 } else { 0 });
            c.bus.write32(mp + 0x408, if has_song { song } else { 0 });
            for i in 0..8u32 {
                c.bus.write32(song + 4 * i, 0);
            }
        });
    }

    #[test]
    fn music_player_clear_playlist_matches() {
        check(ee::s08::f_002b3eb0__generated, music_player_clear_playlist, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2b3e_b0, 0);
            let mp = DATA + 0x100;
            c.g[A0][0] = sx(mp);
            fill(c, &mut r, mp + 0x3e0, 0x160);
        });
    }

    // -------------------------------------------------------- memory card

    #[test]
    fn c_ps2_mem_card_create_matches() {
        check(ee::s08::f_002c22c8__generated, c_ps2_mem_card_create, 40, &|c, seed| {
            alloc_machine(c, seed, 0x2c22_c8);
        });
    }

    #[test]
    fn c_ps2_mem_card_destroy_matches() {
        check(ee::s08::f_002c2300__generated, c_ps2_mem_card_destroy, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2c23_00, 0);
            let obj = DATA + 0x100;
            let vt = put_vtable(c, &mut r, DATA + 0x800, &[8]);
            c.bus.write32(obj, vt);
            c.g[A0][0] = sx(if seed % 4 == 0 { 0 } else { obj });
        });
    }

    #[test]
    fn c_ps2_mem_card_ctor_matches() {
        check(ee::s08::f_002c3fa8__generated, c_ps2_mem_card_ctor, 40, &|c, seed| {
            alloc_machine(c, seed, 0x2c3f_a8);
            c.g[A0][0] = sx(DATA + 0x1000);
        });
    }

    #[test]
    fn c_ps2_mem_card_request_list_dir_matches() {
        check(ee::s08::f_002c41d8__generated, c_ps2_mem_card_request_list_dir, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2c41_d8, 0);
            c.g[A0][0] = sx(DATA + 0x1000);
            c.g[A1][0] = sx(DATA + 0x2000);
            c.bus.write8(DATA + 0x2000, 0);
            let len = (seed % 16) as u32;
            for i in 0..len {
                c.bus.write8(DATA + 0x2000 + i, b'a' + (r.next_u32() % 26) as u8);
            }
            c.bus.write8(DATA + 0x2000 + len, 0);
        });
    }

    #[test]
    fn c_ps2_mem_card_request_delete_matches() {
        check(ee::s08::f_002c6ce0__generated, c_ps2_mem_card_request_delete, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2c6c_e0, 0);
            c.g[A0][0] = sx(DATA + 0x1000);
            c.g[A1][0] = sx(DATA + 0x2000);
            let len = (seed % 16) as u32;
            for i in 0..len {
                c.bus.write8(DATA + 0x2000 + i, b'A' + (r.next_u32() % 26) as u8);
            }
            c.bus.write8(DATA + 0x2000 + len, 0);
        });
    }

    #[test]
    fn c_ps2_console_config_create_matches() {
        check(ee::s08::f_002c7440__generated, c_ps2_console_config_create, 60, &|c, seed| {
            alloc_machine(c, seed, 0x2c74_40);
        });
    }

    // ---------------------------------------------------------- menu/stats

    #[test]
    fn menu_sys_submenu_item_measure_matches() {
        check(ee::s09::f_002ccd90__generated, menu_sys_submenu_item_measure, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2ccd_90, 0);
            c.g[A0][0] = sx(DATA + 0x100);
            c.g[A1][0] = sx(DATA + 0x200);
            c.bus.write32(DATA + 0x200 + 0x14, (seed % 8) as u32);
            fill(c, &mut r, DATA + 0x100, 0x40);
        });
    }

    #[test]
    fn menu_sys_back_item_measure_matches() {
        check(ee::s09::f_002ccea8__generated, menu_sys_back_item_measure, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2cce_a8, 0);
            c.g[A0][0] = sx(DATA + 0x100);
            c.g[A1][0] = sx(DATA + 0x200);
            c.bus.write32(DATA + 0x200 + 0x14, (seed % 8) as u32);
            fill(c, &mut r, DATA + 0x100, 0x40);
        });
    }

    #[test]
    fn menu_sys_label_item_measure_matches() {
        check(ee::s09::f_002cd2f0__generated, menu_sys_label_item_measure, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2cd2_f0, 0);
            c.g[A0][0] = sx(DATA + 0x100);
            c.g[A1][0] = sx(DATA + 0x200);
            c.bus.write32(DATA + 0x200 + 0x14, (seed % 8) as u32);
            fill(c, &mut r, DATA + 0x100, 0x40);
        });
    }

    #[test]
    fn ssx_stats_man_create_matches() {
        check(ee::s09::f_002d2100__generated, ssx_stats_man_create, 60, &|c, seed| {
            alloc_machine(c, seed, 0x2d21_00);
        });
    }

    #[test]
    fn c_transition_man_reset_effect_pool_matches() {
        check(ee::s09::f_002e4cb0__generated, c_transition_man_reset_effect_pool, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2e4c_b0, 0);
            fill(c, &mut r, 0x0053_8850, 16);
        });
    }

    #[test]
    fn c_bolt_on_fx_interface_clear_emitters_matches() {
        check(ee::s09::f_002ecaf0__generated, c_bolt_on_fx_interface_clear_emitters, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2eca_f0, 0);
            let fx = DATA + 0x100;
            c.g[A0][0] = sx(fx);
            fill(c, &mut r, fx, 0x100);
        });
    }

    #[test]
    fn ps2_ai_visual_effects_man_create_matches() {
        check(ee::s09::f_002f0390__generated, ps2_ai_visual_effects_man_create, 40, &|c, seed| {
            alloc_machine(c, seed, 0x2f03_90);
        });
    }

    #[test]
    fn ps2_visual_effects_man_create_matches() {
        check(ee::s09::f_002f0510__generated, ps2_visual_effects_man_create, 40, &|c, seed| {
            alloc_machine(c, seed, 0x2f05_10);
        });
    }

    #[test]
    fn world_light_man_reset_regions_matches() {
        check(ee::s09::f_002f5998__generated, world_light_man_reset_regions, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2f59_98, 0);
            let man = DATA + 0x100;
            c.g[A0][0] = sx(man);
            fill(c, &mut r, man, 0x240);
        });
    }

    // -------------------------------------------------------- strings

    driven!(gen_ustr_len_2, ee::s08::f_002c24d0__generated);
    driven!(hand_ustr_len_2, ustr_len_2);

    #[test]
    fn ustr_len_2_matches() {
        check_events(gen_ustr_len_2, hand_ustr_len_2, 200, &|c, seed| {
            let mut r = Rng(0x7573_7472_6c65_6e32 ^ (seed as u64) << 20);
            crate::difftest::scramble(c, &mut r);
            let len = (seed % 24) as u32;
            for i in 0..len {
                let b = match r.next_u32() % 5 {
                    0 => [1u16, 0x7f, 0x80, 0xff, 0xfe][(r.next_u32() % 5) as usize],
                    _ => (r.next_u32() as u16).max(1),
                };
                c.bus.write16(DATA + 2 * i, b);
            }
            c.bus.write16(DATA + 2 * len, 0);
            c.g[A0][0] = sx(DATA);
            schedule(c, &mut r, seed, 0x002c_24d0)
        });
    }

    driven!(gen_str_to_lower, ee::s08::f_002c2868__generated);
    driven!(hand_str_to_lower, str_to_lower_in_place);

    #[test]
    fn str_to_lower_in_place_matches() {
        check_events(gen_str_to_lower, hand_str_to_lower, 200, &|c, seed| {
            let mut r = Rng(0x6c6f_7765_72 ^ (seed as u64) << 20);
            crate::difftest::scramble(c, &mut r);
            let len = (seed % 24) as u32;
            for i in 0..len {
                let b = match r.next_u32() % 6 {
                    0 => b'A' + (r.next_u32() % 26) as u8,
                    1 => [b'Z', b'a', b'z', b'@', b'[', 0x80, 0xff][(r.next_u32() % 7) as usize],
                    _ => (r.next_u32() as u8).max(1),
                };
                c.bus.write8(DATA + i, b);
            }
            c.bus.write8(DATA + len, 0);
            c.g[A0][0] = sx(DATA);
            schedule(c, &mut r, seed, 0x002c_2868)
        });
    }

    // ------------------------------------------------- script and rider

    #[test]
    fn wscript_man_lookup_func_matches() {
        check(ee::s10::f_00309b70__generated, wscript_man_lookup_func, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x309b_70, 0);
            c.g[A0][0] = sx(DATA + 0x100);
            c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(r.next_u32()) };
            c.bus.write32(DATA + 0x100 + 0x28c, DATA + 0x400);
            fill(c, &mut r, DATA + 0x400, 8);
            // The lookup's table base: zero keeps the (fake-free) lookup in
            // low memory instead of following a random pointer.
            c.bus.write32(DATA + 0x404, 0);
        });
    }

    #[test]
    fn rider_model_lock_part_matches() {
        check(ee::s10::f_0030eca0__generated, rider_model_lock_part, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x30ec_a0, 0);
            let model = DATA + 0x100;
            let parts = DATA + 0x1000;
            let indices = DATA + 0x2000;
            c.g[A0][0] = sx(model);
            c.g[A1][0] = sx(seed % 8);
            c.bus.write32(model + 0x1c, parts);
            c.bus.write32(model + 0xc, DATA + 0x3000);
            c.bus.write64(model + 0x158, r.next_u32() as u64);
            for i in 0..8u32 {
                c.bus.write32(indices + 4 * i, r.next_u32() % 6);
            }
            fill(c, &mut r, parts, 6 * 0x58 / 4);
        });
    }

    // -------------------------------------------------- vtable forwarders

    /// The wrappers whose vtable comes from the argument object.
    fn forward_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32) {
        let mut r = machine(c, seed, salt, 0);
        let obj = DATA + 0x100;
        let arg = DATA + 0x300;
        let vt = put_vtable(c, &mut r, DATA + 0x800, &[slot]);
        c.bus.write32(arg, vt);
        c.g[A0][0] = sx(obj);
        c.g[A1][0] = sx(arg);
        c.g[A2][0] = sx(r.next_u32());
        c.g[A3][0] = sx(r.next_u32());
        fill(c, &mut r, obj, 8);
    }

    #[test]
    fn rider_base_load_cheat_code_matches() {
        check(ee::s09::f_002f6a90__generated, rider_base_load_cheat_code, 120, &|c, seed| {
            forward_setup(c, seed, 0x2f6a_90, 0x10);
        });
    }

    #[test]
    fn wscript_task_save_state_matches() {
        check(ee::s10::f_00308760__generated, wscript_task_save_state, 120, &|c, seed| {
            forward_setup(c, seed, 0x3087_60, 8);
        });
    }

    #[test]
    fn wscript_task_load_state_matches() {
        check(ee::s10::f_00308798__generated, wscript_task_load_state, 120, &|c, seed| {
            forward_setup(c, seed, 0x3087_98, 0x10);
        });
    }

    #[test]
    fn wscript_man_collect_set_load_matches() {
        check(ee::s10::f_0030c700__generated, wscript_man_collect_set_load, 120, &|c, seed| {
            forward_setup(c, seed, 0x30c7_00, 0x10);
        });
    }

    #[test]
    fn crowd_man3d_group_save_matches() {
        check(ee::s11::f_00344fc0__generated, crowd_man3d_group_save, 120, &|c, seed| {
            forward_setup(c, seed, 0x344f_c0, 8);
        });
    }

    #[test]
    fn boost_modifier_base_save_matches() {
        check(ee::s11::f_003584b8__generated, boost_modifier_base_save, 120, &|c, seed| {
            forward_setup(c, seed, 0x3584_b8, 8);
        });
    }

    #[test]
    fn roller_modifier_save_body_matches() {
        check(ee::s11::f_0035da00__generated, roller_modifier_save_body, 120, &|c, seed| {
            forward_setup(c, seed, 0x35da_00, 8);
        });
    }

    #[test]
    fn roller_modifier_load_body_matches() {
        check(ee::s11::f_0035da38__generated, roller_modifier_load_body, 120, &|c, seed| {
            forward_setup(c, seed, 0x35da_38, 0x10);
        });
    }

    #[test]
    fn modifier_block_forward_v42_matches() {
        check(ee::s11::f_00353098__generated, modifier_block_forward_v42, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x3530_98, 0);
            let block = DATA + 0x100;
            let node = DATA + 0x300;
            let vt = put_vtable(c, &mut r, DATA + 0x800, &[0xb0]);
            c.bus.write32(block, if seed % 4 == 0 { 0 } else { node });
            c.bus.write32(node, vt);
            c.g[A0][0] = sx(block);
            c.g[A1][0] = sx(r.next_u32());
            fill(c, &mut r, node, 8);
        });
    }

    // ---------------------------------------------- node constructors

    fn ctor_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32, src: u32) {
        let mut r = machine(c, seed, salt, 0);
        let vt = put_vtable(c, &mut r, DATA + 0x800, &[slot]);
        c.bus.write32(src, vt);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = sx(r.next_u32());
        c.g[A2][0] = sx(src);
        fill(c, &mut r, DATA + 0x1000, 0x100);
        // ModObj's ctor reads a word the fake read leaves alone; keep it zero
        // so the modifier-block path is skipped.
        c.bus.write32(STACK_TOP - 0x60, 0);
    }

    #[test]
    fn flag_ctor_2_matches() {
        check(ee::s11::f_0034ae68__generated, flag_ctor_2, 80, &|c, seed| {
            ctor_setup(c, seed, 0x34ae_68, 0x20, DATA + 0x300);
        });
    }

    #[test]
    fn hide_ctor_2_matches() {
        check(ee::s11::f_00350f08__generated, hide_ctor_2, 80, &|c, seed| {
            ctor_setup(c, seed, 0x350f_08, 0x20, DATA + 0x300);
        });
    }

    #[test]
    fn restore_node_ctor_2_matches() {
        check(ee::s11::f_00350fd8__generated, restore_node_ctor_2, 80, &|c, seed| {
            ctor_setup(c, seed, 0x350f_d8, 0x20, DATA + 0x300);
        });
    }

    #[test]
    fn object_ctor_2_matches() {
        check(ee::s11::f_00356e60__generated, object_ctor_2, 80, &|c, seed| {
            ctor_setup(c, seed, 0x356e_60, 0x10, DATA + 0x300);
        });
    }

    #[test]
    fn particle_ctor_matches() {
        check(ee::s11::f_00357918__generated, particle_ctor, 80, &|c, seed| {
            ctor_setup(c, seed, 0x3579_18, 0x10, DATA + 0x300);
        });
    }

    #[test]
    fn multi_particle_man_ctor_matches() {
        check(ee::s11::f_003579c8__generated, multi_particle_man_ctor, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3579_c8, 0);
            c.g[A0][0] = sx(DATA + 0x100);
            c.bus.write32(GAME_GP + 0xf38, r.next_u32());
            fill(c, &mut r, DATA + 0x100, 0x10);
        });
    }

    #[test]
    fn floating_v45_matches() {
        check(ee::s12::f_00361778__generated, floating_v45, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x3617_78, 0);
            c.g[A0][0] = sx(DATA + 0x100);
            let bound = seed % 3 != 0;
            c.bus.write32(DATA + 0x100 + 0x1c, if bound { DATA + 0x300 } else { 0 });
            c.bus.write32(DATA + 0x100 + 0x78, r.next_u32());
            fill(c, &mut r, DATA + 0x300, 0x40);
            // The bound block's first word is its wrapped object: no object,
            // no virtual call.
            c.bus.write32(DATA + 0x300, 0);
        });
    }

    #[test]
    fn intc_table_remove_vif_vu_handlers_matches() {
        check(ee::s12::f_00361e80__generated, intc_table_remove_vif_vu_handlers, 80, &|c, seed| {
            let mut r = machine(c, seed, 0x361e_80, 0);
            c.g[A0][0] = sx(DATA + 0x100);
            fill(c, &mut r, DATA + 0x100, 0x40);
        });
    }

    // ------------------------------------------------------ destructors

    #[test]
    fn graphics_man_dtor_matches() {
        check(ee::s12::f_003695d8__generated, graphics_man_dtor, 80, &|c, seed| {
            let mut r = machine(c, seed, 0x3695_d8, 0);
            pool_allocator(c);
            heap_init(c);
            // A real block: with the low flag bit set the destructor frees it.
            let obj = heap_block(c, 0x1200);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx((seed & 3) as u32);
            c.bus.write32(GAME_GP + 0x2a90, r.next_u32());
        });
    }

    #[test]
    fn bezier_man_base_dtor_matches() {
        check(ee::s12::f_00374ca8__generated, bezier_man_base_dtor, 80, &|c, seed| {
            let mut r = machine(c, seed, 0x374c_a8, 0);
            pool_allocator(c);
            heap_init(c);
            let obj = heap_block(c, 0x80);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx((seed & 3) as u32);
            c.bus.write32(GAME_GP + 0x1158, r.next_u32());
        });
    }

    // --------------------------------------------------------- graphics

    #[test]
    fn emitter_render_at_matches() {
        check(ee::s12::f_00370888__generated, emitter_render_at, 80, &|c, seed| {
            let mut r = machine(c, seed, 0x3708_88, 0);
            let desc = DATA + 0x2000;
            c.g[A0][0] = sx(desc);
            c.g[A1][0] = sx(DATA + 0x400);
            c.g[A2][0] = sx(DATA + 0x440);
            crate::difftest::put_vec(c, DATA + 0x400, [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
            crate::difftest::put_vec(c, DATA + 0x410, [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
            crate::difftest::put_vec(c, DATA + 0x440, [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
            fill(c, &mut r, desc, 0x200);
            // Nothing pending on the descriptor: Emitter_Render returns early.
            c.bus.write32(desc + 0xc, 0);
        });
    }

    #[test]
    fn ps2_graphics_man_create_matches() {
        check(ee::s12::f_00375a08__generated, ps2_graphics_man_create, 30, &|c, seed| {
            alloc_machine(c, seed, 0x375a_08);
        });
    }

    #[test]
    fn renderer_vif1_dmac_handler_matches() {
        check(ee::s12::f_00382650__generated, renderer_vif1_dmac_handler, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3826_50, 0);
            // Only cause 1 is testable: any other cause is a `break` (fatal).
            c.g[A0][0] = 1;
            c.g[A1][0] = sx(DATA + 0x1000);
            c.bus.write32(DATA + 0x1000 + 0x5a8c, (seed % 4) as u32);
            fill(c, &mut r, DATA + 0x1000, 0x40);
        });
    }
}
