//! `src/sound/bankmanager.cpp`: hand-written replacements (moved from the
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
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_3;
pub(crate) use crate::game::be::beintplayer::lh;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gpa;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::dirtysock::tagsunk::arg;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::gp;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at as at_3;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::p as p_3;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::p as p_8;
pub(crate) use crate::game::fe::feasyncfile::p as p_9;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p as p_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::ui::uistate::oc224_at;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_3;
pub(crate) use crate::game::wscript::wscriptman::p as p_7;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_2;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0028_b588, snd_bank_set_memory, "SndBank_SetMemory"),
    (0x0028_b650, snd_bank_free_memory, "SndBank_FreeMemory"),
    (0x0028_b6c8, snd_bank_link, "SndBank_Link"),
    (0x0028_b730, snd_bank_unlink_2, "SndBank_Unlink"),
    (
        0x0028_b788,
        snd_bank_load_from_memory,
        "SndBank_LoadFromMemory",
    ),
    (0x0028_b7e0, snd_bank_unload_memory, "SndBank_UnloadMemory"),
    (
        0x0028_b830,
        snd_bank_is_loaded_with_name,
        "SndBank_IsLoadedWithName",
    ),
    (0x0028_b878, snd_bank_release, "SndBank_Release"),
    (
        0x0028_b930,
        snd_bank_man_init_load_request,
        "SndBankMan_InitLoadRequest",
    ),
    (
        0x0028_bce8,
        snd_bank_man_unload_bank,
        "SndBankMan_UnloadBank",
    ),
    (
        0x0028_bd10,
        snd_bank_man_load_bank_async,
        "SndBankMan_LoadBankAsync",
    ),
    (
        0x0028_bda8,
        snd_bank_man_set_bank_memory,
        "SndBankMan_SetBankMemory",
    ),
    (
        0x0028_bde0,
        snd_bank_man_free_bank_memory,
        "SndBankMan_FreeBankMemory",
    ),
    (
        0x0028_be90,
        snd_bank_man_unload_memory_bank,
        "SndBankMan_UnloadMemoryBank",
    ),
    (
        0x0028_beb8,
        snd_bank_man_release_bank,
        "SndBankMan_ReleaseBank",
    ),
    (
        0x0028_bee0,
        snd_bank_man_get_bank_buffer,
        "SndBankMan_GetBankBuffer",
    ),
    (
        0x0028_bf10,
        snd_bank_man_get_bank_buffer_size,
        "SndBankMan_GetBankBufferSize",
    ),
    (
        0x0028_bf38,
        snd_bank_man_init_bank_request,
        "SndBankMan_InitBankRequest",
    ),
    (
        0x0028_cd48,
        ssx_audio_system_play_peak_transition_music,
        "SSXAudioSystem_PlayPeakTransitionMusic",
    ),
    (
        0x0028_d5a0,
        ssx_audio_system_update_music_tier,
        "SSXAudioSystem_UpdateMusicTier",
    ),
    (
        0x0028_d740,
        ssx_audio_system_confirm_music_trigger,
        "SSXAudioSystem_ConfirmMusicTrigger",
    ),
    (
        0x0028_d960,
        ssx_audio_system_is_game_music_enabled,
        "SSXAudioSystem_IsGameMusicEnabled",
    ),
    (
        0x0028_def0,
        ssx_audio_system_set_music_state,
        "SSXAudioSystem_SetMusicState",
    ),
    (
        0x0028_e888,
        ssx_audio_system_end_script_music,
        "SSXAudioSystem_EndScriptMusic",
    ),
    (
        0x0028_ef90,
        ssx_audio_system_restart_peak_music,
        "SSXAudioSystem_RestartPeakMusic",
    ),
    (
        0x0028_f200,
        ssx_audio_system_on_replay_begin,
        "SSXAudioSystem_OnReplayBegin",
    ),
    (
        0x0028_f2c0,
        ssx_audio_system_on_replay_end,
        "SSXAudioSystem_OnReplayEnd",
    ),
    (
        0x0028_f328,
        ssx_audio_system_on_music_post_event,
        "SSXAudioSystem_OnMusicPostEvent",
    ),
    (
        0x0028_f520,
        audio_stop_fe_music,
        "SSXAudioSystem_StopFEMusic",
    ),
    (
        0x0028_f558,
        ssx_audio_system_set_selected_peak,
        "SSXAudioSystem_SetSelectedPeak",
    ),
    (
        0x0028_f678,
        ssx_audio_system_on_event_select_exit,
        "SSXAudioSystem_OnEventSelectExit",
    ),
    (
        0x0028_f730,
        ssx_audio_system_unload_loading_screen_bank,
        "SSXAudioSystem_UnloadLoadingScreenBank",
    ),
    (
        0x0028_fc58,
        ssx_audio_system_play_char_select_music,
        "SSXAudioSystem_PlayCharSelectMusic",
    ),
    (
        0x002a_3f90,
        ssx_audio_system_is_track_group5,
        "SSXAudioSystem_IsTrackGroup5",
    ),
    (
        0x002a_4030,
        ssx_audio_system_is_track_id_group4,
        "SSXAudioSystem_IsTrackIdGroup4",
    ),
    (
        0x002a_40e0,
        ssx_audio_system_is_track_id_group0,
        "SSXAudioSystem_IsTrackIdGroup0",
    ),
    (
        0x002a_4158,
        ssx_audio_system_is_track_id_group1,
        "SSXAudioSystem_IsTrackIdGroup1",
    ),
    (
        0x002a_41c8,
        ssx_audio_system_is_track_id_group2,
        "SSXAudioSystem_IsTrackIdGroup2",
    ),
    (
        0x002a_4238,
        ssx_audio_system_is_track_id_group3,
        "SSXAudioSystem_IsTrackIdGroup3",
    ),
    (
        0x002a_42c8,
        ssx_audio_system_is_race_type6_to11,
        "SSXAudioSystem_IsRaceType6to11",
    ),
    (
        0x002b_4070,
        music_player_set_playlist_mask,
        "MusicPlayer_SetPlaylistMask",
    ),
    (
        0x002b_4708,
        music_player_clear_loops_active,
        "MusicPlayer_ClearLoopsActive",
    ),
    (
        0x0028_c8c8,
        ssx_audio_system_update_music_intensity,
        "SSXAudioSystem_UpdateMusicIntensity",
    ),
    (
        0x0028_e100,
        ssx_audio_system_update_peak_music,
        "SSXAudioSystem_UpdatePeakMusic",
    ),
    (
        0x002a_3fe8,
        ssx_audio_system_is_track_group4,
        "SSXAudioSystem_IsTrackGroup4",
    ),
    (
        0x0028_d988,
        ssx_audio_system_handle_music_trigger,
        "SSXAudioSystem_HandleMusicTrigger",
    ),
    (0x0028_b8d8, snd_bank_get_buffer, "SndBank_GetBuffer"),
    (0x0028_b9b8, snd_bank_man_ctor, "SndBankMan_ctor"),
    (0x0028_bc58, snd_bank_man_load_bank_2, "SndBankMan_LoadBank"),
    (
        0x0028_be60,
        snd_bank_man_load_bank_from_memory,
        "SndBankMan_LoadBankFromMemory",
    ),
    (
        0x0028_bf78,
        ssx_audio_system_set_music_mode,
        "SSXAudioSystem_SetMusicMode",
    ),
    (
        0x0028_c2d0,
        ssx_audio_system_set_request_playlist,
        "SSXAudioSystem_SetRequestPlaylist",
    ),
    (
        0x0028_cf98,
        ssx_audio_system_play_music,
        "SSXAudioSystem_PlayMusic",
    ),
    (
        0x0028_d8a0,
        ssx_audio_system_get_track_music_group,
        "SSXAudioSystem_GetTrackMusicGroup",
    ),
    (
        0x0028_f108,
        ssx_audio_system_reset_music_fade_in,
        "SSXAudioSystem_ResetMusicFadeIn",
    ),
    (
        0x0028_f140,
        ssx_audio_system_set_fe_music_screen,
        "SSXAudioSystem_SetFEMusicScreen",
    ),
    (
        0x0028_f5b8,
        ssx_audio_system_on_event_select_enter,
        "SSXAudioSystem_OnEventSelectEnter",
    ),
    (
        0x0028_f700,
        ssx_audio_system_load_loading_screen_bank,
        "SSXAudioSystem_LoadLoadingScreenBank",
    ),
    (
        0x0028_f768,
        ssx_audio_system_play_loading_screen_loop,
        "SSXAudioSystem_PlayLoadingScreenLoop",
    ),
    (
        0x0028_d488,
        ssx_audio_system_select_random_song_hand,
        "SSXAudioSystem_SelectRandomSong",
    ),
    (
        0x0028_f478,
        ssx_audio_system_on_song_changed_resumable,
        "SSXAudioSystem_OnSongChanged",
    ),
    (
        0x0028_b928,
        snd_bank_get_buffer_size,
        "SndBank_GetBufferSize",
    ),
    (
        0x0028_c8c0,
        oc817_ssx_audio_system_set_countdown_value,
        "SSXAudioSystem_SetCountdownValue",
    ),
    (
        0x0028_df08,
        ssx_audio_system_reset_music_state,
        "SSXAudioSystem_ResetMusicState",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::AUDIO_IS_GAME_MUSIC_ENABLED;

    pub(crate) use crate::game::be::beintmission::callee::MUSIC_PLAYER_STOP_SONG;

    pub(crate) use crate::game::be::beintmission::callee::SND_BANK_SET_MEMORY;

    pub(crate) use crate::game::bx::bxstringctor::callee::SND_BANK_MAN_INIT_LOAD_REQUEST;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SND_BANK_GET_BUFFER;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SND_BANK_GET_BUFFER_SIZE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SND_BANK_RELEASE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SND_BANK_UNLOAD_MEMORY;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SSX_AUDIO_HANDLE_MUSIC_EVENT;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP;

    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;

    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;

    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;

    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;

    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME as SND_BANK_IS_LOADED_WITH_NAME_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK as SND_LOAD_QUEUE_CANCEL_BANK_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;

    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_RESTART_PEAK_MUSIC;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2;

    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;

    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {

    pub(crate) use crate::game::wscript::wscriptman::table::SND_BANK_REMOVE;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {

    pub(crate) use crate::game::bx::bxstringctor::addr::BE_INTERFACE_LOOKUP;
    pub(crate) use crate::game::bx::bxstringctor::addr::CBE_GET_BE;
    pub(crate) use crate::game::bx::bxstringctor::addr::CBE_GET_INTERFACE;

    pub(crate) use crate::game::bx::bxstringctor::addr::UI_ENGINE_TEARDOWN;
}

/// `void SndBank_Link(SndBank *self, SndBank *other)` (0x28b6c8): splice
/// `self` into the doubly-linked list right after `other`, then adopt
/// `other`'s data pointers.
pub fn snd_bank_link(c: &mut Cpu) {
    let self_ = Addr(abi::arg(c, 0));
    let other = Addr(abi::arg(c, 1));
    c.cycles += 3;
    let other_next = other.off(0x18).lw(c);
    if other_next == 0 {
        other.off(0x18).sw(c, c.g[A0][0]);
        c.cycles += 3;
        other.off(0x1c).sw(c, c.g[A0][0]);
        self_.off(0x18).sw(c, c.g[A1][0]);
        self_.off(0x1c).sw(c, c.g[A1][0]);
    } else {
        c.cycles += 7;
        self_.off(0x18).sw(c, c.g[A1][0]);
        let next = other.off(0x1c).lw(c);
        self_.off(0x1c).sw(c, next);
        other.off(0x1c).sw(c, c.g[A0][0]);
        let back = self_.off(0x1c).lw(c);
        Addr(back as u32).off(0x18).sw(c, c.g[A0][0]);
    }
    c.cycles += 3;
    let data = other.off(0x8).lw(c);
    c.g[V0][0] = data;
    if data != 0 {
        c.cycles += 7;
        self_.off(0x8).sw(c, data);
        let p10 = other.off(0x10).lw(c);
        self_.off(0x10).sw(c, p10);
        let p14 = other.off(0x14).lw(c);
        c.g[V1][0] = p14;
        self_.off(0x14).sw(c, p14);
        let p0c = other.off(0xc).lw(c);
        self_.off(0xc).sw(c, p0c);
        c.g[V0][0] = p0c;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SndBank_Unlink(SndBank *bank)` (0x28b730): unlink the bank from its
/// doubly-linked list and clear its link fields.
pub fn snd_bank_unlink_2(c: &mut Cpu) {
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

// ---------------------------------------------------------------------------
// Sound bank manager.

/// `SndBankMan` methods index a 0x60-byte `SndBank` array at `self+0xacc`.
pub(crate) fn snd_bank_call(c: &mut Cpu, target: Callee, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x60;
    frame.save_ra(c, 0);
    c.mult(0, A1, A1, V0);
    let base = Addr(arg(c, 0).wrapping_add(0xacc)).lw(c);
    c.g[A0][0] = sx((base as u32).wrapping_add(c.g[A1][0] as u32));
    target.call(c, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void SndBankMan_UnloadBank(self, int bank)` (0x28bce8).
pub fn snd_bank_man_unload_bank(c: &mut Cpu) {
    guest(c, |c| {
        snd_bank_call(c, callee::SND_BANK_UNLOAD, 0x0028_bd04)
    })
}

/// `void SndBankMan_FreeBankMemory(self, int bank)` (0x28bde0).
pub fn snd_bank_man_free_bank_memory(c: &mut Cpu) {
    guest(c, |c| {
        snd_bank_call(c, callee::SND_BANK_FREE_MEMORY, 0x0028_bdfc)
    })
}

/// `void SndBankMan_UnloadMemoryBank(self, int bank)` (0x28be90).
pub fn snd_bank_man_unload_memory_bank(c: &mut Cpu) {
    guest(c, |c| {
        snd_bank_call(c, callee::SND_BANK_UNLOAD_MEMORY, 0x0028_beac)
    })
}

/// `void SndBankMan_ReleaseBank(self, int bank)` (0x28beb8).
pub fn snd_bank_man_release_bank(c: &mut Cpu) {
    guest(c, |c| {
        snd_bank_call(c, callee::SND_BANK_RELEASE, 0x0028_bed4)
    })
}

/// `int SndBankMan_GetBankBufferSize(self, int bank)` (0x28bf10).
pub fn snd_bank_man_get_bank_buffer_size(c: &mut Cpu) {
    guest(c, |c| {
        snd_bank_call(c, callee::SND_BANK_GET_BUFFER_SIZE, 0x0028_bf2c)
    })
}

/// `bool SSXAudioSystem_IsGameMusicEnabled(SSXAudioSystem* self)`
/// (0x28d960): true when the game-music state at `+0x608c` is 0, 1 or 3.
pub fn ssx_audio_system_is_game_music_enabled(c: &mut Cpu) {
    c.cycles += 4;
    let state = Addr(arg(c, 0).wrapping_add(0x608c)).lw(c);
    c.g[V1][0] = state;
    let below_two = state < 2;
    c.g[V0][0] = below_two as u64;
    c.g[V0][0] = 3;
    if !below_two {
        c.cycles += 2;
        if state != 3 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `void *SndBankMan_GetBankBuffer(SndBankMan *self, int index)` (0x28bee0):
/// the memory buffer of bank `index` (each `SndBank` is 0x60 bytes in the
/// array at `self+0xacc`).
pub fn snd_bank_man_get_bank_buffer(c: &mut Cpu) {
    /// The array of `SndBank`s.
    const BANKS: u32 = 0xacc;
    /// Size of one `SndBank`.
    const BANK_SIZE: u32 = 0x60;

    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = BANK_SIZE as u64;
        frame.save_ra(c, 0);
        c.mult(0, V1, A1, V0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(BANKS).lw(c);
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        callee::SND_BANK_GET_BUFFER.call(c, 0x0028_bf04)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SndBankMan_SetBankMemory(SndBankMan* self, int bank, void* mem,
/// uint32 flags, uint32 arg)` (0x28bda8): pass the per-bank memory block of
/// the `mem` pointer to `SndBank_SetMemory`.
pub fn snd_bank_man_set_bank_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x60;
        frame.save_ra(c, 0);
        c.mult(0, V1, A1, V0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xacc)).lw(c);
        c.g[A2][0] = c.g[A3][0];
        c.g[A3][0] = c.g[T0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        callee::SND_BANK_SET_MEMORY.call(c, 0x0028_bdd0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SSXAudioSystem_StopFEMusic(SSXAudioSystem* self)` (0x28f520):
/// stop the front-end music player, if game music is enabled.
pub fn audio_stop_fe_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::AUDIO_IS_GAME_MUSIC_ENABLED.call(c, 0x0028_f534)?;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
            callee::MUSIC_PLAYER_STOP_SONG.call(c, 0x0028_f544)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Sound.

/// `void SSXAudioSystem_EndScriptMusic(SSXAudioSystem* self)` (0x28e888):
/// post the script-music stop event and clear the handle.
pub fn ssx_audio_system_end_script_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x6254).lw(c);
        let silent = c.g[V0][0] == 0;
        c.g[A1][0] = 0x13;
        if !silent {
            c.cycles += 3;
            c.g[A2][0] = 1;
            callee::SSX_AUDIO_HANDLE_MUSIC_EVENT.call(c, 0x0028_e8ac)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).off(0x6254).sw(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_UnloadLoadingScreenBank(SSXAudioSystem* self)`
/// (0x28f730): fade out the loading-screen loop and unload its bank.
pub fn ssx_audio_system_unload_loading_screen_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.mtc1(0, 12);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP.call(c, 0x0028_f748)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        callee::SND_BANK_MAN_UNLOAD_BANK.call(c, 0x0028_f758)?;
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

/// `int SndBank_IsLoadedWithName(self, name)` (0x28b830): true when the bank
/// is in state 1 and its name (`self+0x20`) matches `name`.
pub fn snd_bank_is_loaded_with_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 1;
        let self_ = c.g[V1][0] as u32;
        c.g[S0][0] = Addr(self_).lw(c);
        let not_loaded = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = c.g[A1][0];
        if not_loaded {
            c.cycles += 1;
            c.g[V0][0] = 0;
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A1][0] = sx(self_.wrapping_add(0x20));
        call_ptr(c, 0x0041_65a8, 0x0028_b858)?;
        if c.g[V0][0] != 0 {
            c.g[S0][0] = 0;
        }
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the three race-type range checks: is the current race
/// type (the byte at 0x535c12) inside `[lo, hi]`?
pub(crate) fn race_type_in_range(c: &mut Cpu, lo: i64, hi: i64, rets: [u32; 3]) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    call_ptr(c, 0x0014_dc80, rets[0])?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, rets[1])?;
    c.g[V1][0] = 0x530000;
    c.g[V1][0] = Addr(0x0053_5c12).lb(c);
    c.g[V0][0] = ((c.g[V1][0] as i64) < lo) as u64;
    let below = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !below {
        c.cycles += 3;
        c.g[V1][0] = ((c.g[V1][0] as i64) < hi) as u64;
        let inside = c.g[V1][0] != 0;
        c.g[V0][0] = 1;
        if !inside {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `bool SSXAudioSystem_IsRaceType6to11(void)` (0x2a42c8).
pub fn ssx_audio_system_is_race_type6_to11(c: &mut Cpu) {
    guest(c, |c| {
        race_type_in_range(c, 6, 12, [0x002a_42d8, 0x002a_42e4, 0x002a_4308])
    })
}

/// `void SndBank_UnloadMemory(SndBank* self)` (0x28b7e0): detach the bank from
/// the sound system and clear the memory/state fields.
pub fn snd_bank_unload_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = u64::MAX; // -1: no bank
        Addr(this).sw(c, 0);
        c.g[A0][0] = Addr(this).off(4).lw(c);
        if c.g[A0][0] != c.g[S1][0] {
            c.cycles += 4;
            call_ptr(c, table::SND_BANK_REMOVE, 0x0028_b810)?;
            Addr(this).off(4).sw(c, c.g[S1][0]);
            Addr(this).off(0x14).sw(c, 0);
        } else {
            Addr(this).off(0x14).sw(c, 0);
        }
        c.cycles += 6;
        Addr(this).off(0x10).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SndBank_LoadFromMemory(SndBank* b, u32 a1, u32 a2)` (0x28b788): drop
/// the bank's current memory, install the new pointer/length and rebuild the
/// tag table.
pub fn snd_bank_load_from_memory(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x30);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A2][0];
    frame.save_ra(c, 0);
    c.g[S2][0] = c.g[A1][0];
    if callee::SND_BANK_UNLOAD_MEMORY_2
        .call(c, 0x0028_b7ac)
        .is_err()
    {
        return;
    }
    c.sw(p(c, S0, 0x14), c.g[S1][0]);
    c.g[A1][0] = c.g[S2][0];
    c.sw(p(c, S0, 0x10), c.g[S2][0]);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
    if call_ptr(c, 0x003b_6098, 0x0028_b7c0).is_err() {
        return;
    }
    c.g[V1][0] = 1;
    frame.restore(c, S1, 0x20);
    c.sw(p(c, S0, 0), c.g[V1][0]);
    frame.restore(c, S0, 0x30);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Sound banks.

/// `void SndBankMan_InitBankRequest(SndBankMan* man, int, int, int, int,
/// int, int)` (0x28bf38): forward a bank-load request to
/// `SndBankMan_InitLoadRequest(man->banks + arg1*0x60, ...)` with the
/// arguments shifted up by one (the byte offset is built by the `mult`).
pub fn snd_bank_man_init_bank_request(c: &mut Cpu) {
    const RET: u32 = 0x0028_bf68;
    guest(c, |c| {
        c.cycles += 15;
        c.g[V0][0] = 0x60;
        let sp = r32(c, SP).wrapping_sub(0x10);
        c.g[SP][0] = sx(sp);
        c.mult(0, 3, 5, 2);
        Addr(sp).sd(c, c.g[RA][0]);
        c.g[A1][0] = c.g[A2][0];
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = Addr(r32(c, A0).wrapping_add(0xacc)).lw(c);
        c.g[A3][0] = c.g[T0][0];
        c.g[T0][0] = c.g[T1][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[T1][0] = c.g[T2][0];
        callee::SND_BANK_MAN_INIT_LOAD_REQUEST.call(c, RET)?;
        c.g[RA][0] = Addr(sp).ld(c);
        c.g[SP][0] = sx(sp.wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_SetSelectedPeak(SSXAudioSystem* self, int id)`
/// (0x28F558): store the peak-music id and restart that peak if the audio
/// manager is already playing it.
pub fn ssx_audio_system_set_selected_peak(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(ea(c.g[S0][0], 0x6284), c.g[A1][0]);
        call_ptr(c, addr::CBE_GET_BE, 0x0028_f570)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, addr::CBE_GET_INTERFACE, 0x0028_f57c)?;
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, addr::BE_INTERFACE_LOOKUP, 0x0028_f58c)?;
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x6284));
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::SSX_AUDIO_SYSTEM_RESTART_PEAK_MUSIC.call(c, 0x0028_f5a4)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SndBank_Release(SndBank* bank)` (0x28b878): unload the bank, mark it
/// released and unlink it from its parent when it was playing.
pub fn snd_bank_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let bank = c.g[A0][0];
        c.g[S0][0] = bank;
        call_ptr(c, 0x0028_b528, 0x0028_b88c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_b2d0, 0x0028_b894)?;
        c.g[V0][0] = 0x2;
        c.g[V1][0] = Addr(at(c, S0, 0x8)).lw(c);
        let not_playing = c.g[V1][0] != c.g[V0][0];
        Addr(at(c, S0, 0x0)).sw(c, c.g[V0][0]);
        if !not_playing {
            c.cycles += 9;
            c.g[V1][0] = Addr(gp(c, 0x524)).lw(c);
            c.g[A1][0] = Addr(at(c, S0, 0xc)).lw(c);
            c.g[V0][0] = Addr(r32_2(c, V1)).off(0x1d8).lw(c);
            c.g[A2][0] = Addr(r32_2(c, V0)).lw(c);
            c.g[V1][0] = Addr(r32_2(c, A2)).lw(c);
            c.g[A0][0] = Addr(r32_2(c, V1)).off(0x30).lh(c);
            c.g[V0][0] = Addr(r32_2(c, V1)).off(0x34).lw(c);
            let target = r32_2(c, V0);
            c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
            call_ptr(c, target, 0x0028_b8c8)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_OnReplayEnd(SSXAudioSystem* self)` (0x28f2c0): stop the
/// music player backing the replay (`+0x6294`) or send the stop command
/// (`0x24`) through the bank manager vtable (`+0x5558`), then clear both
/// replay flags.
pub fn ssx_audio_system_on_replay_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let base = this as u32;
        c.g[V0][0] = lw(c, p_2(base, 0x6294));
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = sx(p_2(base, 0x118));
            call_ptr(c, 0x002b_3ac0, 0x0028_f2e4)?;
            sw(c, p_2(base, 0x6298), 0);
            c.cycles += 5;
            sw(c, p_2(base, 0x6294), 0);
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.g[V0][0] = lw(c, p_2(base, 0x6298));
        c.cycles += 2;
        let idle = c.g[V0][0] == 0;
        c.g[A2][0] = sx(p_2(base, 0x118));
        if !idle {
            c.cycles += 6;
            c.g[V1][0] = lw(c, p_2(base, 0x5558));
            c.g[A1][0] = 0x24;
            c.g[A0][0] = lh(c, p_2(c.g[V1][0] as u32, 0x20));
            c.g[V0][0] = lw(c, p_2(c.g[V1][0] as u32, 0x24));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0028_f30c)?;
        }
        c.cycles += 1;
        sw(c, p_2(base, 0x6298), 0);
        c.cycles += 5;
        sw(c, p_2(base, 0x6294), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem::RestartPeakMusic(self)` (0x28ef90): reset the peak-music
/// state, pick a random song and arm its timer.
pub fn ssx_audio_system_restart_peak_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p_3(c.g[S0][0] as u32, 0x6258)).sw(c, 0);
        Addr(p_3(c.g[S0][0] as u32, 0x577c)).sw(c, 0);
        call_ptr(c, 0x0028_df08, 0x0028_efac)?;
        Addr(p_3(c.g[S0][0] as u32, 0x578c)).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_d488, 0x0028_efb8)?;
        c.g[V1][0] = Addr(p_3(c.g[S0][0] as u32, 0x118)).lw(c);
        c.g[V0][0] = 0x4a0000;
        c.ldl(A2, 0x004a_364f);
        c.ldr(A2, 0x004a_3648);
        c.g[A1][0] = 0xa;
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0x2;
        c.g[T1][0] = 0x1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        call_ptr(c, 0x002a_dca0, 0x0028_efec)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SndBank_FreeMemory` (0x28B650): unload a sound bank and release its stream
/// and memory.
pub fn snd_bank_free_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SND_BANK_UNLOAD.call(c, 0x0028_b664)?;
        c.g[V1][0] = lw_at(c, S0, 0x8);
        let free = c.g[V1][0] == 0;
        c.g[V0][0] = 2;
        if free {
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        let streaming = c.g[V1][0] != c.g[V0][0];
        c.g[V1][0] = Addr(gp(c, 0x524)).lw(c);
        if !streaming {
            c.cycles += 10;
            c.g[A1][0] = lw_at(c, S0, 0xc);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x1d8)).lw(c);
            c.g[A3][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(r32(c, A3)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
            c.g[A2][0] = Addr(r32(c, V1).wrapping_add(0x2c)).lw(c);
            let stop = r32(c, A2);
            c.g[A0][0] = abi::addu(c.g[A3][0], c.g[A0][0]);
            call_ptr(c, stop, 0x0028_b698)?;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            sw_at(c, S0, 0xc, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[A0][0] = lw_at(c, S0, 0x10);
        call_ptr(c, addr::UI_ENGINE_TEARDOWN, 0x0028_b6a8)?;
        sw_at(c, S0, 0x8, 0);
        sw_at(c, S0, 0x10, 0);
        sw_at(c, S0, 0x14, 0);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_SetPlaylistMask(MusicPlayer* m, u32 mask)`
/// (0x002b4070): clear the playlist, then enable every song whose bit is set
/// in `mask`, finally poke the empty-playlist hook.
pub fn music_player_set_playlist_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        call_ptr(c, 0x002b_3eb0, 0x002b_4090)?; // MusicPlayer_ClearPlaylist
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3ec));
        c.g[S0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0] >> (c.g[S0][0] & 63);
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.g[V0][0] & 1;
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                let zero = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if !zero {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x002b_3fe8, 0x002b_40bc)?; // MusicPlayer_EnableSong
                }
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3ec));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[S2][0] >> (c.g[S0][0] & 63);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_40a0);
                }
            }
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x002b_4060, 0x002b_40d8)?; // MusicPlayer_EmptyPlaylistHook
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void SndBankMan_InitLoadRequest(SndBankMan* self, u32 bank, u32 async,
/// void* cb)` (0x28b930): ask the bank's slot for the loading screen, mark
/// the request pending and start it.
pub fn snd_bank_man_init_load_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        open(c, 0x50);
        let sp0 = spv(c);
        let g = gpa(c, 0x524);
        c.g[V0][0] = lw_at_2(c, g as u64, 0);
        c.sq(16, at_2(sp0, 0x40));
        c.sq(17, at_2(sp0, 0x30));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, at_2(sp0, 0x20));
        c.g[S1][0] = c.g[A2][0];
        c.sq(19, at_2(sp0, 0x10));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S3][0] = c.g[A3][0];
        c.g[V1][0] = lw_at_2(c, c.g[V0][0], 0x1d8);
        c.g[A1][0] = lw_at_2(c, c.g[V1][0], 0);
        c.g[V0][0] = lw_at_2(c, c.g[A1][0], 0);
        c.g[A0][0] = lh_at(c, c.g[V0][0], 0x38);
        c.g[V1][0] = lw_at_2(c, c.g[V0][0], 0x3c);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at_2(c.g[A1][0] as u32, c.g[A0][0] as u32));
        call_ptr(c, func, 0x0028_b978)?;
        let pending = c.g[S1][0] != 0;
        let this = c.g[S2][0];
        c.sw(at_2(this as u32, 4), c.g[S0][0]);
        if !pending {
            c.cycles += 3;
            c.g[V0][0] = 1;
            c.sw(this as u32, c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.sw(this as u32, 0);
        }
        c.cycles += 10;
        c.g[A0][0] = sx(at_2(this as u32, 0x20));
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0041_66f4, 0x0028_b99c)?;
        c.lq(16, at_2(sp0, 0x40));
        c.lq(17, at_2(sp0, 0x30));
        c.lq(18, at_2(sp0, 0x20));
        c.lq(19, at_2(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x50));
        c.pc = ra;
        Ok(())
    })
}

/// `void SSXAudioSystem_OnEventSelectExit(SSXAudioSystem* self, int resume)`
/// (0x28f678): drop the event-select audio mode, stop the loading-screen
/// loop and (when asked) start the event music.
pub fn ssx_audio_system_on_event_select_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at_2(sp0, 0x20));
        c.sq(17, at_2(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        let this = c.g[S0][0];
        c.g[V0][0] = lw_at_2(c, this, 0x6088);
        let none = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 3;
            call_ptr(c, callee::SSX_AUDIO_RESUME_SFX, 0x0028_f6a0)?;
            c.sw(at_2(this as u32, 0x6088), 0);
        }
        c.cycles += 3;
        c.g[V0][0] = lw_at_2(c, this, 0x62a8);
        if (c.g[V0][0] as i64) < 0 {
            c.lq(16, at_2(sp0, 0x20));
        } else {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S0][0];
            callee::STOP_LOADING_SCREEN_LOOP.call(c, 0x0028_f6c0)?;
            let skip = c.g[S1][0] == 0;
            if skip {
                c.lq(16, at_2(sp0, 0x20));
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::SSX_AUDIO_IS_TRACK_GROUP5, 0x0028_f6d0)?;
                let not_group5 = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !not_group5 {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = u64::MAX;
                    c.g[T0][0] = 0;
                    callee::SSX_AUDIO_PLAY_MUSIC.call(c, 0x0028_f6ec)?;
                }
                c.cycles += 1;
                c.lq(16, at_2(sp0, 0x20));
            }
        }
        c.cycles += 4;
        c.lq(17, at_2(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sound banks.

/// `void SSXAudioSystem_UpdateMusicTier(self, int tier)` (0x28d5a0): when the
/// music system is not in state 0x17, play the track for the given tier if it
/// differs from the one already playing.
pub fn ssx_audio_system_update_music_tier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        let tier = abi::arg64(c, 1);
        c.g[V0][0] = 0x17;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(self_);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw(at_3(c, S0, 0x6284));
        let idle = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = tier;
        if !idle {
            c.cycles += 10;
            c.g[A0][0] = sx(p_5(r32(c, S0), 0x118));
            call_ptr(c, callee::AUDIO_GET_MUSIC_TIER, 0x0028_d5d0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, callee::AUDIO_GET_TRACK_TYPE, 0x0028_d5e0)?;
            c.g[A3][0] = c.g[V0][0];
            c.g[V0][0] = 1;
            let one = c.g[A3][0] == c.g[V0][0];
            c.g[A0][0] = 0x65;
            if !one {
                c.cycles += 4;
                c.g[A0][0] = 0x67;
                c.g[V1][0] = 0x66;
                c.g[V0][0] = c.g[A3][0] ^ 2;
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = c.g[V1][0];
                }
            }
            c.cycles += 2;
            let same = c.g[A0][0] == c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            if !same {
                c.cycles += 4;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[T0][0] = 0;
                call_ptr(c, callee::AUDIO_PLAY_MUSIC, 0x0028_d618)?;
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

/// `SndBankMan_LoadBankAsync` (0x28bd10): if the bank is not loaded, cancel
/// any queued load of the same name and queue it again with the entry's id.
pub fn snd_bank_man_load_bank_async(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x60;
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.mult(0, 19, 17, 2);
        frame.save(c, S2, 0x30);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A3][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = Addr(p_6(this, 0xacc)).lw(c);
        c.g[RA][0] = sx(0x0028_bd54);
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S3)));
        callee::SND_BANK_IS_LOADED_WITH_NAME_2.call(c, 0x0028_bd54)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x50);
        } else {
            c.cycles += 12;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0028_bd68);
            c.g[A1][0] = c.g[S1][0];
            callee::SND_LOAD_QUEUE_CANCEL_BANK_2.call(c, 0x0028_bd68)?;
            c.g[V0][0] = Addr(p_6(r(c, S0), 0xacc)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[V0][0] = sx(r(c, S3).wrapping_add(r(c, V0)));
            c.g[T0][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x0028_bd88);
            c.g[A3][0] = Addr(p_6(r(c, V0), 8)).lw(c);
            callee::SND_LOAD_QUEUE_PUSH.call(c, 0x0028_bd88)?;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_ConfirmMusicTrigger` (0x28d740): confirm a music trigger
/// when the object's request id matches, clearing the trigger otherwise.
pub fn ssx_audio_system_confirm_music_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[V0][0] = Addr(p_6(this, 0x6240)).lw(c);
        let same = c.g[V0][0] == c.g[S2][0];
        let mut skip = false;
        if !same {
            c.cycles += 4;
            c.g[V1][0] = Addr(p_6(this, 0x62a0)).lw(c);
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            let none = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = 0;
            if !none {
                c.cycles += 9;
                c.g[V1][0] = Addr(p_6(r(c, S1), 0x6c0)).lw(c);
                c.g[A1][0] = sx(r(c, S1).wrapping_add(0x6c0));
                c.g[A0][0] = Addr(p_6(r(c, V1), 0x38)).lh(c);
                c.g[V0][0] = Addr(p_6(r(c, V1), 0x3c)).lw(c);
                let target = r(c, V0);
                c.g[RA][0] = sx(0x0028_d790);
                c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x0028_d790)?;
                c.g[V1][0] = Addr(p_6(r(c, S0), 0x62a0)).lw(c);
                c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                c.g[A0][0] = (c.g[V0][0] < 1) as u64;
            }
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                frame.restore(c, S0, 0x30);
                skip = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p_6(this, 0x623c)).lw(c);
                let different = c.g[V0][0] != c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                if different {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0028_d7c0);
                    c.g[A1][0] = c.g[S1][0];
                    callee::CLEAR_MUSIC_TRIGGER.call(c, 0x0028_d7c0)?;
                } else {
                    c.cycles += 2;
                    Addr(p_6(this, 0x6240)).sw(c, c.g[S2][0]);
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
        } else {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            skip = true;
        }
        let _ = skip;
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_OnMusicPostEvent(AudioSystem* self, int event)`
/// (0x28f328): post the event to the embedded music player, then, when the
/// current song tag came back as none, track the restart events (10, 11, 18)
/// in `+0x530` and clear it (stopping the song through the player's vtable
/// slot 0x18 for event 18).
pub fn ssx_audio_system_on_music_post_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x002b_3b88, 0x0028_f350)?; // MusicPlayer_PostEvent
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x002b_49e0, 0x0028_f358)?; // MusicPlayer_GetSongTag
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        let mut early = false;
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x30);
            early = true;
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xa));
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            let restart = c.g[V0][0] != 0;
            c.g[V1][0] = lw_3(c, gp(c, 0x530));
            if !restart {
                c.cycles += 3;
                c.g[V0][0] = 0x12;
                let other = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, S0, 0x30);
                if other {
                    early = true;
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = lw_3(c, gp(c, 0x530));
                }
            }
        }
        if !early {
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                sw_2(c, p_7(c.g[S2][0] as u32, 0x530), 0);
            } else {
                c.cycles += 7;
                c.g[V0][0] = lw_3(c, p_7(c.g[S2][0] as u32, 0x5558));
                c.g[A1][0] = 8;
                c.g[A0][0] = Addr(p_7(c.g[V0][0] as u32, 0x18)).lh(c);
                c.g[V1][0] = lw_3(c, p_7(c.g[V0][0] as u32, 0x1c));
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x0028_f3a8)?;
                sw_2(c, p_7(c.g[S2][0] as u32, 0x530), 0);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SSXAudioSystem_PlayCharSelectMusic(self)` (0x28fc58): start the
/// character-select music on the sound bank's channel 0x12d and set its
/// volume from the current rider.
pub fn ssx_audio_system_play_char_select_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = 1;
        frame.save(c, S1, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = sx(0x004a_0000);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = sx(r32(c, S2).wrapping_add(0x118));
        frame.save(c, S4, 0x10);
        c.g[S1][0] = sx(0x004a_3598);
        frame.save_ra(c, 0);
        c.g[S4][0] = Addr(p_8(r32(c, S2), 0x5558)).lw(c);
        c.g[S0][0] = Addr(p_8(r32(c, S4), 0x10)).lh(c);
        c.g[S0][0] = sx(p_8(r32(c, S3), r32(c, S0)));
        call_ptr(c, 0x0028_7968, 0x0028fc9c)?;
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = 1;
        c.g[V0][0] = Addr(p_8(r32(c, S4), 0x14)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        let play = r32(c, V0);
        c.g[A2][0] = 0xc;
        call_ptr(c, play, 0x0028fcb8)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x12d;
        call_ptr(c, 0x002b_49c0, 0x0028fcc4)?;
        c.g[V1][0] = Addr(p_8(r32(c, S2), 0x5558)).lw(c);
        c.g[A1][0] = Addr(p_8(r32(c, S2), 0x6290)).lw(c);
        c.g[A0][0] = Addr(p_8(r32(c, V1), 0x20)).lh(c);
        c.g[V0][0] = Addr(p_8(r32(c, V1), 0x24)).lw(c);
        let set_vol = r32(c, V0);
        c.g[A0][0] = sx(p_8(r32(c, S3), r32(c, A0)));
        call_ptr(c, set_vol, 0x0028fcdc)?;
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

// ---------------------------------------------------------------------------
// `SSXAudioSystem_OnMusicSetEventState` (0x28f3c8): set the bank monitor's
// event state and, when the song tag is no longer -1 and the code is one of
// 10/11/18, optionally issue the monitor's `v3(8)` call and clear `+0x530`.
//
// Not installed and without a test (no `OVERRIDES` entry); kept under
// `cfg(test)` as an unverified port.
#[cfg(test)]
#[allow(dead_code)]
pub fn ssx_audio_system_on_music_set_event_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002b3bc0, 0x0028_f3f4)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x002b49e0, 0x0028_f400)?;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        let tagged = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = c.g[S3][0];
        if tagged {
            c.g[V0][0] = c.g[S3][0];
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xa));
            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
            let interesting = c.g[V0][0] != 0;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x530));
            let mut act = true;
            if !interesting {
                c.cycles += 3;
                c.g[V0][0] = 0x12;
                let other = c.g[S0][0] != c.g[V0][0];
                c.g[V0][0] = c.g[S3][0];
                if other {
                    act = false;
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x530));
                }
            }
            if act {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                if c.g[V1][0] != c.g[V0][0] {
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x530), 0u64);
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5558));
                    c.g[A1][0] = 0x8;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0028_f450)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x530), 0u64);
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S3][0];
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_PlayPeakTransitionMusic(self)` (0x28cd48): start the
/// peak transition music on the music channel and set its volume.
pub fn ssx_audio_system_play_peak_transition_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = 0x1;
        frame.save(c, S1, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0x4a0000;
        frame.save(c, S3, 0x20);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        c.g[S3][0] = sx((this as u32).wrapping_add(0x118));
        frame.save(c, S0, 0x50);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3628));
        frame.save(c, S4, 0x10);
        c.g[S4][0] = Addr(p_9(this as u32, 0x5558)).lw(c);
        c.g[S0][0] = Addr(p_9(c.g[S4][0] as u32, 0x10)).lh(c);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
        call_ptr(c, 0x0028_7968, 0x0028_cd8c)?;
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = 0x1;
        c.g[V0][0] = Addr(p_9(c.g[S4][0] as u32, 0x14)).lw(c);
        c.g[A2][0] = 0xc;
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, target, 0x0028_cda8)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x191;
        call_ptr(c, 0x002b_49c0, 0x0028_cdb4)?;
        c.g[V1][0] = Addr(p_9(c.g[S2][0] as u32, 0x5558)).lw(c);
        c.g[A1][0] = 0x1;
        c.g[A0][0] = Addr(p_9(c.g[V1][0] as u32, 0x20)).lh(c);
        c.g[V0][0] = Addr(p_9(c.g[V1][0] as u32, 0x24)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0028_cdcc)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x7f;
        call_ptr(c, 0x002b_3c28, 0x0028_cdd8)?;
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

// ---------------------------------------------------------------------------
// `SSXAudioSystem_OnReplayBegin` (0x0028f200).
//
// `void SSXAudioSystem::OnReplayBegin(self)`: start the replay music unless it
// is already playing.

pub fn ssx_audio_system_on_replay_begin(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(this);
        let mon = this.wrapping_add(0x118);
        c.g[S1][0] = sx(mon);
        sw_3(c, this, 0x6294, 0);
        c.g[V0][0] = lw_4(c, this, 0x530);
        let t = c.g[V0][0] == 0;
        sw_3(c, this, 0x6298, 0);
        if !t {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x002b_49e0, 0x0028_f230)?;
            c.g[V1][0] = 0xc9;
            let not_playing = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 1;
            if not_playing {
                c.cycles += 1;
                sw_3(c, this, 0x6298, c.g[V0][0]);
                c.cycles += 5;
                frame.restore(c, 16, 0x20);
                frame.restore(c, 17, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_d960, 0x0028_f244)?;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = 1;
        if !t {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x002b_49e0, 0x0028_f254)?;
            c.g[V1][0] = 0xc9;
            if c.g[V0][0] != c.g[V1][0] {
                c.g[V0][0] = lw_4(c, this, 0x118);
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw_4(c, this, 0x62ac);
                sw_3(c, this, 0x508, c.g[V0][0]);
                c.g[V0][0] = lw_4(c, this, 0x118);
            }
            c.cycles += 13;
            c.g[V1][0] = 0x4a0000;
            c.ldl(6, 0x4a364f);
            c.ldr(6, 0x4a3648);
            c.g[A1][0] = 0xa;
            c.g[A0][0] = lw_4(c, c.g[V0][0] as u32, 0);
            c.g[A3][0] = 0;
            c.g[T0][0] = 3;
            c.g[T1][0] = 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            call_ptr(c, 0x002a_dca0, 0x0028_f29c)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 2;
        sw_3(c, this, 0x6294, c.g[V0][0]);
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SndBank_SetMemory(this, size, extra, tag)` (0x28b588): store the flags,
/// try to allocate the `extra` region from the stream allocator and record
/// the sized allocation.
pub fn snd_bank_set_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let this = abi::arg(c, 0);
        let _size = abi::arg(c, 1);
        let _extra = abi::arg(c, 2);
        let _tag = abi::arg(c, 3);
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = abi::arg64(c, 2);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = abi::arg64(c, 1);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        let nonzero = c.g[S1][0] != 0;
        c.g[S3][0] = abi::arg64(c, 3);

        if !nonzero {
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                frame.restore(c, 16, 0x40);
                c.cycles += 6;
                frame.restore(c, 17, 0x30);
                frame.restore(c, 18, 0x20);
                frame.restore(c, 19, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }

        c.cycles += 3;
        c.g[V0][0] = 1;
        let no_extra = c.g[S0][0] == 0;
        Addr(this).off(8).sw(c, c.g[V0][0]);
        if !no_extra {
            c.cycles += 12;
            c.g[V1][0] = Addr(gp(c, 0x524)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1d8).lw(c);
            c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0028_b5ec)?;
            c.g[A0][0] = c.g[V0][0];
            let failed = (c.g[A0][0] as i64) < 0;
            c.g[V0][0] = 2;
            if !failed {
                c.cycles += 3;
                Addr(this).off(0xc).sw(c, c.g[A0][0]);
                Addr(this).off(8).sw(c, c.g[V0][0]);
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx(0xffff_fffd_u32);
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = c.g[A0][0] ^ c.g[V0][0];
                if c.g[V0][0] == 0 {
                    c.g[S1][0] = c.g[V1][0];
                }
            }
        }

        c.cycles += 8;
        Addr(this).off(0x14).sw(c, c.g[S1][0]);
        c.g[A0][0] = 0x0048_0000;
        c.g[A0][0] = sx(0x0048_0000u32.wrapping_add(0x2998));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S3][0];
        callee::MEM_ALLOC_NAMED_128.call(c, 0x0028_b62c)?;
        Addr(this).off(0x10).sw(c, c.g[V0][0]);
        frame.restore(c, 16, 0x40);
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod audio_sys {

    /// The inline music player (bank monitor) at +0x118.
    pub const PLAYER: u32 = 0x118;

    /// The music bank's mixer object (its methods: +0x10 set gain, +0x20
    /// set volume curve).
    pub const MIXER: u32 = 0x5558;

    /// Peak music needs an update.
    pub const PEAK_DIRTY: u32 = 0x6274;

    /// What to do: 0 start, 2 change peak, 3/other resume, 4 stop.
    pub const PEAK_STATE: u32 = 0x6278;

    /// The pending track id (0x17 = none).
    pub const PEAK_TRACK: u32 = 0x6284;

    pub const NO_TRACK: u64 = 0x17;
}

pub(crate) mod audio_callee {
    use super::Callee;
    use ssx3_ee as ee;

    pub const IS_GAME_MUSIC_ENABLED: Callee = Callee {
        addr: 0x0028_d960,
        f: ee::s07::SSXAudioSystem_IsGameMusicEnabled,
    };

    pub const GET_BE_LIBRARY: Callee = Callee {
        addr: 0x0028_b1e8,
        f: ee::s07::SSXAudioSystem_GetBELibrary,
    };

    pub const GET_CHANNEL_GAIN_PTR: Callee = Callee {
        addr: 0x0028_7968,
        f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
    };

    pub const SELECT_RANDOM_SONG: Callee = Callee {
        addr: 0x0028_d488,
        f: ee::s07::SSXAudioSystem_SelectRandomSong,
    };

    pub const PLAY_MUSIC: Callee = Callee {
        addr: 0x0028_cf98,
        f: ee::s07::SSXAudioSystem_PlayMusic,
    };

    /// `BE_GetInterface(lib, 0)` (s01).
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;

    /// `BENewRaceInt_GetTrack` (s01): the current track record.
    pub const RACE_GET_TRACK: u32 = 0x0014_4bc0;

    /// `SSXAudioSystem_TrackToCategory(this, track)` (s08): 1, 2, 4 or 0.
    pub const TRACK_TO_CATEGORY: u32 = 0x002a_1e20;

    /// `SSXAudioSystem_IsTrackGroup14(this, track)` (s08).
    pub const IS_RANDOM_TRACK: u32 = 0x002a_4030;

    /// `MusicPlayer_SetCategory(player, n)` (s08).
    pub const PLAYER_SET_CATEGORY: u32 = 0x002b_49c0;

    /// `MusicPlayer_PathCmdA(player, n)` (s08).
    pub const PLAYER_PATH_CMD: u32 = 0x002b_3c28;
}

pub(crate) mod music {

    /// The music player (`MusicMonitor`, embedded).
    pub const MONITOR: u32 = 0x118;

    /// Its vtable pointer (event slot at +0x18).
    pub const MONITOR_VT: u32 = 0x5558;

    /// Ambience gain written for the mixer.
    pub const AMBIENCE_GAIN: u32 = 0x5830;

    /// Music gain (copy of the channel gain).
    pub const MUSIC_GAIN: u32 = 0x60a0;

    /// Speed latched when the rider left the ground.
    pub const AIR_SPEED: u32 = 0x6244;

    /// Lowest height seen during the jump.
    pub const AIR_LOW: u32 = 0x6248;

    /// Peak height expected for the jump.
    pub const AIR_HIGH: u32 = 0x624c;

    /// Frames the music stays held.
    pub const HOLD: u32 = 0x6250;

    /// Rider's air predictor (`rider + 0x788`): state, speed, heights.
    pub const RIDER_AIR: u32 = 0x788;

    pub const AIR_STATE: u32 = 0xac;

    pub const AIR_TAKEOFF_SPEED: u32 = 0x98;

    pub const AIR_PEAK: u32 = 0x9c;

    pub const AIR_HEIGHT: u32 = 0xa0;

    /// `gp` globals: "in a jump" this frame and last frame.
    pub const GP_IN_AIR: u32 = 0x52c;

    pub const GP_WAS_IN_AIR: u32 = 0x530;
}

/// `void SSXAudioSystem_UpdatePeakMusic(SSXAudioSystem* this)` (0x28e100):
/// when the peak music is dirty, by the peak state: 0 (if game music is
/// on) load the bank for the pending track (or the current race track) and
/// apply its category's gain; 2 (if game music is on and a track is
/// pending) either play a random song (random-song tracks) or load the
/// track's bank, then clear the pending track; 4 stop the music; anything
/// else (if music is on) restart it (3 with path 0x24). Clears the dirty
/// flag.
pub fn ssx_audio_system_update_peak_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S4, 0x20);
        frame.save(c, S0, 0x60);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, S4);
        c.g[V0][0] = c.lw(this.wrapping_add(audio_sys::PEAK_DIRTY));
        let dirty = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x60);
        if dirty {
            update_peak_music_body(c, this)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x60);
        }
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

/// Clear the dirty flag (0x28e4e4).
pub(crate) fn peak_clear_dirty(c: &mut Cpu, this: u32) -> crate::call::Flow {
    c.cycles += 1;
    c.sw(this.wrapping_add(audio_sys::PEAK_DIRTY), 0);
    Ok(())
}

pub(crate) fn update_peak_music_body(c: &mut Cpu, this: u32) -> crate::call::Flow {
    c.cycles += 3;
    c.g[S0][0] = c.lw(this.wrapping_add(audio_sys::PEAK_STATE));
    c.g[V0][0] = 2;
    if c.g[S0][0] == 0 {
        return peak_start(c, this);
    }
    c.cycles += 2;
    let state = c.g[S0][0];
    c.g[V0][0] = 4;
    if state == 2 {
        return peak_change(c, this);
    }
    // 0x28e488: stop (4) or restart.
    c.cycles += 2;
    c.g[A0][0] = c.g[S4][0];
    if state == 4 {
        c.cycles += 2;
        c.g[A1][0] = 0;
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S4][0];
        audio_callee::IS_GAME_MUSIC_ENABLED.call(c, 0x0028_e4a0)?;
        let enabled = c.g[V0][0] != 0;
        c.g[V0][0] = 3;
        if !enabled {
            return peak_clear_dirty(c, this);
        }
        c.cycles += 3;
        c.g[V1][0] = c.lw(this.wrapping_add(audio_sys::PEAK_STATE));
        c.g[A0][0] = c.g[S4][0];
        if c.g[V1][0] != c.g[V0][0] {
            // PlayMusic(this, 0, 0, -1, 0)
            c.cycles += 5;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = u64::MAX;
            c.g[T0][0] = 0;
            audio_callee::PLAY_MUSIC.call(c, 0x0028_e4e4)?;
            return peak_clear_dirty(c, this);
        }
        c.cycles += 1;
        c.g[A1][0] = 0x24;
    }
    // 0x28e4b8: PlayMusic(this, a1, 0, -1, 0)
    c.cycles += 6;
    c.g[A2][0] = 0;
    c.g[A3][0] = u64::MAX;
    c.g[T0][0] = 0;
    audio_callee::PLAY_MUSIC.call(c, 0x0028_e4c8)?;
    c.sw(this.wrapping_add(audio_sys::PEAK_DIRTY), 0);
    Ok(())
}

/// State 0 (0x28e134): load the bank for the pending (or current race)
/// track.
pub(crate) fn peak_start(c: &mut Cpu, this: u32) -> crate::call::Flow {
    c.cycles += 4;
    audio_callee::IS_GAME_MUSIC_ENABLED.call(c, 0x0028_e144)?;
    let enabled = c.g[V0][0] != 0;
    c.g[S0][0] = audio_sys::NO_TRACK;
    if !enabled {
        return peak_clear_dirty(c, this);
    }
    c.cycles += 3;
    c.g[A1][0] = c.lw(this.wrapping_add(audio_sys::PEAK_TRACK));
    if c.g[A1][0] == c.g[S0][0] {
        // No pending track: the race's track.
        c.cycles += 12;
        audio_callee::GET_BE_LIBRARY.call(c, 0x0028_e160)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, audio_callee::BE_GET_INTERFACE, 0x0028_e16c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, audio_callee::RACE_GET_TRACK, 0x0028_e174)?;
        c.g[A1][0] = c.lw(r32(c, V0));
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, audio_callee::TRACK_TO_CATEGORY, 0x0028_e180)?;
        c.g[V1][0] = c.g[V0][0];
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, audio_callee::TRACK_TO_CATEGORY, 0x0028_e190)?;
        let v = c.g[S0][0];
        c.sw(this.wrapping_add(audio_sys::PEAK_TRACK), v);
        c.g[V1][0] = c.g[V0][0];
    }
    // 0x28e198: by category (v1).
    c.cycles += 3;
    c.g[V0][0] = 2;
    if c.g[V1][0] == 1 {
        c.cycles += 23;
        peak_bank_gain(c, this, 1, [0x0028_e1d0, 0x0028_e1ec, 0x0028_e1f8])?;
    } else {
        c.cycles += 2;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 22;
            peak_bank_gain(c, this, 2, [0x0028_e230, 0x0028_e24c, 0x0028_e258])?;
        } else {
            c.cycles += 21;
            peak_bank_gain(c, this, 3, [0x0028_e284, 0x0028_e2a4, 0x0028_e2b0])?;
        }
    }
    c.cycles += 10;
    peak_bank_curve(c, [0x0028_e2c8, 0x0028_e2d4])?;
    c.sw(this.wrapping_add(audio_sys::PEAK_DIRTY), 0);
    Ok(())
}

/// State 2 (0x28e2dc): change to the pending peak track.
pub(crate) fn peak_change(c: &mut Cpu, this: u32) -> crate::call::Flow {
    c.cycles += 4;
    c.g[A0][0] = c.g[S4][0];
    audio_callee::IS_GAME_MUSIC_ENABLED.call(c, 0x0028_e2ec)?;
    let enabled = c.g[V0][0] != 0;
    c.g[V0][0] = audio_sys::NO_TRACK;
    if !enabled {
        return peak_clear_dirty(c, this);
    }
    c.cycles += 3;
    c.g[A1][0] = c.lw(this.wrapping_add(audio_sys::PEAK_TRACK));
    if c.g[A1][0] == c.g[V0][0] {
        c.sw(this.wrapping_add(audio_sys::PEAK_DIRTY), 0);
        return Ok(());
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S4][0];
    call_ptr(c, audio_callee::IS_RANDOM_TRACK, 0x0028_e308)?;
    if c.g[V0][0] != 0 {
        // A random song: SelectRandomSong, PlayMusic(this, 0x24, 1, -1, 0).
        c.cycles += 10;
        c.g[A0][0] = c.g[S4][0];
        audio_callee::SELECT_RANDOM_SONG.call(c, 0x0028_e318)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 0x24;
        c.g[A2][0] = 1;
        c.g[A3][0] = u64::MAX;
        c.g[T0][0] = 0;
        audio_callee::PLAY_MUSIC.call(c, 0x0028_e330)?;
        c.g[V0][0] = audio_sys::NO_TRACK;
    } else {
        // Load the track's bank (0x28e338).
        c.g[A1][0] = c.lw(this.wrapping_add(audio_sys::PEAK_TRACK));
        c.cycles += 6;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, audio_callee::TRACK_TO_CATEGORY, 0x0028_e340)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        c.g[S2][0] = sx(this.wrapping_add(audio_sys::PLAYER));
        if c.g[V1][0] == 1 {
            c.cycles += 22;
            peak_bank_gain(c, this, 1, [0x0028_e378, 0x0028_e394, 0x0028_e3a0])?;
        } else {
            c.cycles += 2;
            if c.g[V1][0] == c.g[S0][0] {
                c.cycles += 22;
                peak_bank_gain(c, this, 2, [0x0028_e3d8, 0x0028_e3f4, 0x0028_e400])?;
            } else {
                c.cycles += 21;
                peak_bank_gain(c, this, 3, [0x0028_e42c, 0x0028_e44c, 0x0028_e458])?;
            }
        }
        c.cycles += 9;
        peak_bank_curve(c, [0x0028_e470, 0x0028_e47c])?;
        c.g[V0][0] = audio_sys::NO_TRACK;
    }
    // 0x28e480: no pending track any more.
    c.cycles += 2;
    let v = c.g[V0][0];
    c.sw(this.wrapping_add(audio_sys::PEAK_TRACK), v);
    peak_clear_dirty(c, this)
}

/// Apply category `kind`'s gain: `mixer->vt[0x10](player + d, &table[kind],
/// 0xc, GetChannelGainPtr(this, 1, 0), 0)`, then
/// `MusicPlayer_SetCategory(player, kind)`; v1 = the mixer. The third
/// variant sets s5 after the first call (as compiled).
pub(crate) fn peak_bank_gain(
    c: &mut Cpu,
    this: u32,
    kind: u32,
    rets: [u32; 3],
) -> crate::call::Flow {
    c.g[S3][0] = c.lw(this.wrapping_add(audio_sys::MIXER));
    c.g[S2][0] = sx(this.wrapping_add(audio_sys::PLAYER));
    c.g[A0][0] = c.g[S4][0];
    c.g[S0][0] = c.lh(oc224_at(c, S3, 0x10));
    c.g[S1][0] = sx(0x004a_3630 + 8 * (kind - 1));
    c.g[A1][0] = 1;
    c.g[A2][0] = 0;
    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
    if kind != 3 {
        c.g[S5][0] = c.g[S2][0];
    }
    audio_callee::GET_CHANNEL_GAIN_PTR.call(c, rets[0])?;
    if kind == 3 {
        c.g[S5][0] = c.g[S2][0];
    }
    c.g[V1][0] = c.lw(oc224_at(c, S3, 0x14));
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = c.g[V0][0];
    c.g[A2][0] = 0xc;
    let t = r32(c, V1);
    c.g[T0][0] = 0;
    call_ptr(c, t, rets[1])?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = kind as u64;
    call_ptr(c, audio_callee::PLAYER_SET_CATEGORY, rets[2])?;
    c.g[V1][0] = c.lw(this.wrapping_add(audio_sys::MIXER));
    Ok(())
}

/// `mixer(v1)->vt[0x20](player(s5) + d, 0xc)`, then
/// `MusicPlayer_PathCmdA(player, 0x7f)`.
pub(crate) fn peak_bank_curve(c: &mut Cpu, rets: [u32; 2]) -> crate::call::Flow {
    c.g[A1][0] = 0xc;
    c.g[A0][0] = c.lh(oc224_at(c, V1, 0x20));
    c.g[V0][0] = c.lw(oc224_at(c, V1, 0x24));
    let t = r32(c, V0);
    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, rets[0])?;
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = 0x7f;
    call_ptr(c, audio_callee::PLAYER_PATH_CMD, rets[1])
}

/// An FPU register as a float.
#[inline]
pub(crate) fn fpr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `(int)(signed char)x`: the `sll 24; sra 24` the compiler emits for a
/// `char` argument or result.
#[inline]
pub(crate) fn sext8(x: u64) -> u64 {
    sx((((x as u32) << 24) as i32 >> 24) as u32)
}

pub(crate) const FIND_RIDER: Callee = Callee {
    addr: 0x0028_5d98,
    f: ee::s07::SSXAudioSystem_FindRider,
};

pub(crate) const GAIN_PTR: Callee = Callee {
    addr: 0x0028_7968,
    f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
};

pub(crate) const MUSIC_TRIGGER: Callee = Callee {
    addr: 0x0028_d898,
    f: ee::s07::SSXAudioSystem_GetMusicTrigger,
};

pub(crate) const IS_REPLAY: Callee = Callee {
    addr: 0x0028_8ae0,
    f: ee::s07::SSXAudioSystem_IsReplayPlaying,
};

pub(crate) const START_AMBIENCE: Callee = Callee {
    addr: 0x0029_10e0,
    f: ee::s07::SSXAudioSystem_StartAmbienceLoop,
};

pub(crate) const STOP_AMBIENCE: Callee = Callee {
    addr: 0x0029_13d8,
    f: ee::s07::SSXAudioSystem_StopAmbienceLoop,
};

pub(crate) const MUSIC_ENABLED: Callee = Callee {
    addr: 0x0028_d960,
    f: ee::s07::SSXAudioSystem_IsGameMusicEnabled,
};

/// `void SSXAudioSystem_UpdateMusicIntensity(SSXAudioSystem* self)`
/// (0x28c8c8): while the local rider is in a jump (air state 1 or 3) fast
/// enough, duck the music by how high the rider is in the jump (music and
/// ambience gains, monitor volume/percent), start the ambience loop on
/// take-off and fire monitor event 7; on landing (or replay, or too slow)
/// stop the ambience loop, fire event 8 and restore full volume.
pub fn ssx_audio_system_update_music_intensity(c: &mut Cpu) {
    use music::*;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 23, 0x68);
        frame.save_fpr(c, 22, 0x60);
        frame.save_fpr(c, 21, 0x58);
        frame.save_fpr(c, 20, 0x50);
        let sys = r32(c, S1);
        let gp = r32(c, GP);

        // Hold counter.
        let hold = Addr(sys.wrapping_add(HOLD)).lw(c);
        c.g[V0][0] = sx((hold as u32).wrapping_sub(1));
        if (hold as i64) > 0 {
            c.cycles += 1;
            let v = c.g[V0][0];
            Addr(sys.wrapping_add(HOLD)).sw(c, v);
        }

        // The speed that drives the intensity: the take-off speed while in
        // a jump (latched on the first frame), 0 otherwise.
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = u64::MAX;
        FIND_RIDER.call(c, 0x0028_c910)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0011_fe98, 0x0028_c91c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        c.g[A1][0] = 0;
        let mut have_speed = false;
        if c.g[A0][0] == 1 {
            c.cycles += 4;
            let air = Addr(r32(c, S2).wrapping_add(RIDER_AIR)).lw(c);
            c.g[V0][0] = air;
            let state = Addr((air as u32).wrapping_add(AIR_STATE)).lw(c);
            c.g[V1][0] = state;
            c.g[A2][0] = air;
            let jumping = if state == 1 {
                true
            } else {
                c.cycles += 3;
                c.g[V0][0] = 3;
                state == 3
            };
            if jumping {
                c.cycles += 1;
                c.g[A1][0] = 1;
            }
            c.cycles += 2;
            let in_air = Addr(gp.wrapping_add(GP_IN_AIR)).lw(c);
            c.g[V1][0] = in_air;
            c.cycles += 3;
            c.g[V0][0] = 1;
            if jumping {
                if in_air == 1 {
                    c.f[20] = Addr(sys.wrapping_add(AIR_SPEED)).read_u32(c);
                } else {
                    c.cycles += 4;
                    let speed = Addr((air as u32).wrapping_add(AIR_TAKEOFF_SPEED)).read_fpu(c);
                    c.f[20] = speed.0;
                    Addr(sys.wrapping_add(AIR_LOW)).sw(c, 0);
                    Addr(sys.wrapping_add(AIR_SPEED)).write_fpu(c, speed);
                }
                have_speed = true;
            } else if in_air == 1 {
                c.cycles += 2;
                c.f[20] = Addr(sys.wrapping_add(AIR_SPEED)).read_u32(c);
                have_speed = true;
            }
        }
        if !have_speed {
            c.cycles += 1;
            c.f[20] = 0;
        }
        let speed = fpr(c, 20);

        // Whether the monitor may be driven this frame (s3).
        c.cycles += 13;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        c.g[S3][0] = 0;
        GAIN_PTR.call(c, 0x0028_c99c)?;
        let threshold = Addr(gp.wrapping_sub(0x45d4)).read_fpu(c);
        c.f[0] = threshold.0;
        let gain = Addr(r32(c, V0)).read_fpu(c);
        c.f[1] = gain.0;
        let in_air = Addr(gp.wrapping_add(GP_IN_AIR)).lw(c);
        c.g[V0][0] = in_air;
        let fast = abi::c_lt_s(c, threshold, speed);
        Addr(sys.wrapping_add(MUSIC_GAIN)).write_fpu(c, gain);
        Addr(gp.wrapping_add(GP_WAS_IN_AIR)).sw(c, in_air);
        Addr(gp.wrapping_add(GP_IN_AIR)).sw(c, 0);
        if fast {
            c.cycles += 6;
            c.g[A0][0] = sx(sys.wrapping_add(MONITOR));
            call_ptr(c, 0x002b_4878, 0x0028_c9c4)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = 1;
            if c.g[S0][0] == 1 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x002a_4168, 0x0028_c9dc)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x002a_41d8, 0x0028_c9ec)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S1][0];
                        MUSIC_TRIGGER.call(c, 0x0028_c9fc)?;
                        if c.g[V0][0] == 0 {
                            c.g[S3][0] = c.g[S0][0];
                        }
                    }
                }
            }
        }

        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        IS_REPLAY.call(c, 0x0028_ca08)?;
        let replay = c.g[V0][0] != 0;
        c.g[V1][0] = Addr(gp.wrapping_add(GP_WAS_IN_AIR)).lw(c);
        if !replay {
            c.cycles += 6;
            c.g[AT][0] = 0x4000_0000;
            c.f[0] = 0x4000_0000;
            let moving = abi::c_lt_s(c, FpuF32(0x4000_0000), speed);
            c.g[V1][0] = 1;
            if moving {
                let extra = music_in_air(c, sys, gp)?;
                return music_finish(c, frame, extra);
            }
            c.cycles += 1;
            c.g[V1][0] = Addr(gp.wrapping_add(GP_WAS_IN_AIR)).lw(c);
        }
        music_landed(c, sys)?;
        music_finish(c, frame, true)
    })
}

/// In a jump: scale music and ambience by the height ratio. Returns whether
/// the exit takes the extra cycle (`s0` reloaded at the common tail).
pub(crate) fn music_in_air(c: &mut Cpu, sys: u32, gp: u32) -> Result<bool, crate::call::Unwound> {
    use music::*;
    c.cycles += 7;
    let air = Addr(r32(c, S2).wrapping_add(RIDER_AIR)).lw(c);
    c.g[V0][0] = air;
    let low_seen = Addr(sys.wrapping_add(AIR_LOW)).read_fpu(c);
    c.f[0] = low_seen.0;
    let height = Addr((air as u32).wrapping_add(AIR_HEIGHT)).read_fpu(c);
    let lower = abi::c_lt_s(c, height, low_seen);
    let v1 = c.g[V1][0];
    Addr(gp.wrapping_add(GP_IN_AIR)).sw(c, v1);
    let mut low = height;
    if lower {
        c.cycles += 1;
        low = low_seen;
    }
    c.f[4] = low.0;
    c.cycles += 3;
    let was_in_air = Addr(gp.wrapping_add(GP_WAS_IN_AIR)).lw(c);
    c.g[V0][0] = was_in_air;
    Addr(sys.wrapping_add(AIR_LOW)).write_fpu(c, low);
    let high = if was_in_air == 0 {
        // First frame of the jump: expect half the remaining rise above
        // the lowest point (at least 0.5).
        c.cycles += 11;
        let air = Addr(r32(c, S2).wrapping_add(RIDER_AIR)).lw(c);
        c.g[V0][0] = air;
        c.g[AT][0] = 0x3f00_0000;
        let half = FpuF32(0x3f00_0000);
        c.f[2] = half.0;
        let h = Addr((air as u32).wrapping_add(AIR_HEIGHT)).read_fpu(c);
        c.f[1] = h.0;
        let peak = Addr((air as u32).wrapping_add(AIR_PEAK)).read_fpu(c);
        let rise = peak - h;
        c.f[0] = rise.0;
        let mut high = rise * half;
        if abi::c_lt_s(c, high, half) {
            high = half;
        }
        c.cycles += 3;
        let high = high + low;
        c.f[3] = high.0;
        Addr(sys.wrapping_add(AIR_HIGH)).write_fpu(c, high);
        high
    } else {
        c.cycles += 1;
        let high = Addr(sys.wrapping_add(AIR_HIGH)).read_fpu(c);
        c.f[3] = high.0;
        high
    };

    // Volume floor from speed: clamp((1 - speed * k * 0.5) * 127, 20, 127).
    c.cycles += 18;
    let k = Addr(gp.wrapping_sub(0x45d0)).read_fpu(c);
    let below = abi::c_lt_s(c, low, high);
    let half = FpuF32(0x3f00_0000);
    let one = FpuF32(0x3f80_0000);
    let vmax = FpuF32(0x42fe_0000);
    let vmin = FpuF32(0x41a0_0000);
    let speed = fpr(c, 20);
    let scaled = (speed * k) * half;
    c.f[0] = scaled.0;
    c.f[2] = half.0;
    let inv = Addr(gp.wrapping_sub(0x45cc)).read_fpu(c);
    c.f[23] = inv.0;
    let rest = one - scaled;
    c.f[1] = rest.0;
    c.g[AT][0] = 0x41a0_0000;
    c.f[21] = vmax.0;
    c.f[5] = vmin.0;
    let floor = (rest * vmax).min(vmax).max(vmin);
    c.f[20] = floor.0;
    let play = c.g[S3][0] != 0;
    let mon = sx(sys.wrapping_add(MONITOR));

    if below {
        c.cycles += 13;
        let ratio = low.div(high);
        c.f[22] = ratio.0;
        c.g[AT][0] = 0x42d6_0000;
        c.g[S2][0] = 0x7f;
        let t = vmax - ratio * FpuF32(0x42d6_0000);
        c.f[0] = t.0;
        let v = t.min(vmax).max(vmin);
        c.f[1] = v.0;
        c.f[0] = v.to_i32() as u32;
        c.g[V0][0] = sx((c.f[0]) << 24);
        c.g[S0][0] = sext8(c.f[0] as u64);
        if play {
            c.cycles += 21;
            c.g[A1][0] = 1;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0;
            GAIN_PTR.call(c, 0x0028_cb1c)?;
            let k = Addr(r32(c, V0)).read_fpu(c);
            c.g[A0][0] = mon;
            c.g[V0][0] = sx(r32(c, S0).wrapping_sub(0x14));
            c.g[AT][0] = 0x42c8_0000;
            c.g[V0][0] = sx(0x7fu32.wrapping_sub(r32(c, V0)));
            let k = k * inv;
            c.f[1] = k.0;
            let g = FpuF32::from_i32(r32(c, V0) as i32) * k;
            c.f[0] = g.0;
            let pct = g * FpuF32(0x42c8_0000);
            c.f[2] = pct.0;
            Addr(sys.wrapping_add(MUSIC_GAIN)).write_fpu(c, g);
            c.f[0] = pct.to_i32() as u32;
            c.g[A1][0] = sext8(c.f[0] as u64);
            call_ptr(c, 0x002b_3d10, 0x0028_cb60)?;
        }
        c.cycles += 15;
        let floor = fpr(c, 20);
        let ratio = fpr(c, 22);
        let d = (vmax - floor) * ratio;
        c.f[1] = d.0;
        let v = (vmax - d).min(vmax).max(floor);
        c.f[0] = v.0;
        c.f[1] = v.to_i32() as u32;
        c.g[V0][0] = sx(c.f[1] << 24);
        c.g[S0][0] = sext8(c.f[1] as u64);
        c.g[V1][0] = sx(0x7fu32.wrapping_sub(r32(c, S0)));
        let amb = FpuF32::from_i32(r32(c, V1) as i32) * inv;
        c.f[0] = amb.0;
        Addr(sys.wrapping_add(AMBIENCE_GAIN)).write_fpu(c, amb);
        if play {
            c.cycles += 5;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = mon;
            call_ptr(c, 0x002b_3c98, 0x0028_cba8)?;
            c.g[A0][0] = c.g[S1][0];
        } else {
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
        }
    } else {
        c.cycles += 4;
        let amb = (vmax - floor) * inv;
        c.f[0] = amb.0;
        Addr(sys.wrapping_add(AMBIENCE_GAIN)).write_fpu(c, amb);
        if play {
            c.cycles += 22;
            c.f[0] = floor.to_i32() as u32;
            c.g[S0][0] = mon;
            c.g[A0][0] = mon;
            c.g[A1][0] = sext8(c.f[0] as u64);
            call_ptr(c, 0x002b_3c98, 0x0028_cbdc)?;
            c.g[A1][0] = 1;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0;
            GAIN_PTR.call(c, 0x0028_cbec)?;
            let g = Addr(r32(c, V0)).read_fpu(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[AT][0] = 0x42c8_0000;
            c.f[1] = 0x42c8_0000;
            Addr(sys.wrapping_add(MUSIC_GAIN)).write_fpu(c, g);
            let pct = g * FpuF32(0x42c8_0000);
            c.f[0] = pct.0;
            c.f[1] = pct.to_i32() as u32;
            c.g[A1][0] = sext8(c.f[1] as u64);
            call_ptr(c, 0x002b_3d10, 0x0028_cc18)?;
        }
        c.cycles += 1;
        c.g[A0][0] = c.g[S1][0];
    }

    // Ambience gain times the master ambience channel.
    c.cycles += 9;
    c.g[A1][0] = 5;
    c.g[A2][0] = 0;
    GAIN_PTR.call(c, 0x0028_cc28)?;
    let master = Addr(r32(c, V0)).read_fpu(c);
    c.f[0] = master.0;
    let amb = Addr(sys.wrapping_add(AMBIENCE_GAIN)).read_fpu(c) * master;
    let was_in_air = Addr(gp.wrapping_add(GP_WAS_IN_AIR)).lw(c);
    c.g[V0][0] = was_in_air;
    c.f[1] = amb.0;
    Addr(sys.wrapping_add(AMBIENCE_GAIN)).write_fpu(c, amb);
    if was_in_air != 0 {
        return Ok(true);
    }
    // Take-off: start the ambience loop, fire event 7.
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    START_AMBIENCE.call(c, 0x0028_cc48)?;
    c.g[S0][0] = mon;
    if c.g[S3][0] == 0 {
        return Ok(true);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002b_4620, 0x0028_cc58)?;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    MUSIC_ENABLED.call(c, 0x0028_cc68)?;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    c.cycles += 8;
    monitor_event(c, sys, 7, 0x0028_cc88)?;
    Ok(false)
}

/// `monitor->vt->event(monitor + delta, n)` (slot +0x18 of the vtable at
/// `sys + 0x5558`), with the monitor in s0.
pub(crate) fn monitor_event(c: &mut Cpu, sys: u32, event: u64, ret: u32) -> Flow {
    let vt = Addr(sys.wrapping_add(music::MONITOR_VT)).lw(c);
    c.g[V0][0] = vt;
    c.g[A1][0] = event;
    let delta = Addr((vt as u32).wrapping_add(0x18)).lh(c);
    c.g[A0][0] = delta;
    let func = Addr((vt as u32).wrapping_add(0x1c)).lw(c);
    c.g[V1][0] = func;
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(delta as u32));
    call_ptr(c, func as u32, ret)
}

/// Landed (or replay / too slow): if the jump just ended stop the ambience
/// loop and fire event 8; then full volume and the stored music gain.
pub(crate) fn music_landed(c: &mut Cpu, sys: u32) -> Flow {
    use music::*;
    c.cycles += 3;
    c.g[V0][0] = 1;
    let mon = sx(sys.wrapping_add(MONITOR));
    c.g[S0][0] = mon;
    if c.g[V1][0] == 1 {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        STOP_AMBIENCE.call(c, 0x0028_cca8)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002b_46e8, 0x0028_ccb0)?;
        c.g[A1][0] = 0x7f;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            MUSIC_ENABLED.call(c, 0x0028_ccc0)?;
            if c.g[V0][0] != 0 {
                c.cycles += 6;
                monitor_event(c, sys, 8, 0x0028_cce0)?;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002b_4708, 0x0028_cce8)?;
            c.cycles += 1;
            c.g[A1][0] = 0x7f;
        }
    } else {
        c.cycles += 1;
        c.g[A1][0] = 0x7f;
    }
    c.cycles += 12;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002b_3c98, 0x0028_ccf4)?;
    c.g[AT][0] = 0x42c8_0000;
    c.f[1] = 0x42c8_0000;
    c.g[A0][0] = c.g[S0][0];
    let g = Addr(sys.wrapping_add(MUSIC_GAIN)).read_fpu(c);
    let pct = g * FpuF32(0x42c8_0000);
    c.f[0] = pct.0;
    c.f[1] = pct.to_i32() as u32;
    c.g[A1][0] = sext8(c.f[1] as u64);
    call_ptr(c, 0x002b_3d10, 0x0028_cd1c)
}

/// Common tail: `extra` is the path through the 1-cycle `lq s0` block.
pub(crate) fn music_finish(c: &mut Cpu, frame: Frame, extra: bool) -> Flow {
    if extra {
        c.cycles += 1;
    }
    frame.restore(c, S0, 0x40);
    c.cycles += 10;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 23, 0x68);
    frame.restore_fpr(c, 22, 0x60);
    frame.restore_fpr(c, 21, 0x58);
    frame.restore_fpr(c, 20, 0x50);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_IsTrackGroup4 (0x2a3fe8)

/// The current track index (`BENewRaceInt`, 0x535c08) and the track info
/// table it indexes (0x64-byte records from 0x43d950, track id first).
#[cfg(test)]
pub(crate) const TRACK_GROUP4_TRACK: u32 = 0x0053_5c08;

#[cfg(test)]
pub(crate) const TRACK_GROUP4_INFOS: u32 = 0x0043_d950;

/// `bool SSXAudioSystem_IsTrackGroup4(SSXAudioSystem* self)` (0x2a3fe8):
/// whether the current race's track id (`BENewRaceInt_GetTrackInfo` of the
/// BE library's interface 0) is in the group 0xe..0x10
/// (`SSXAudioSystem_IsTrackIdGroup4`). Frame 0x20 (s0 = self).
pub fn ssx_audio_system_is_track_group4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x002a_3ffc)?; // BELibrary_GetInstance
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x002a_4008)?; // BELibrary_GetInterface
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x002a_4010)?; // BENewRaceInt_GetTrackInfo
        c.g[A1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x002a_4030,
            f: ee::s08::SSXAudioSystem_IsTrackIdGroup4,
        }
        .call(c, 0x002a_401c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_HandleMusicTrigger (0x28d988)

/// Trigger object: vtable pointer.
pub(crate) const HMT_TRIGGER_VT: u32 = 0x4;

/// Trigger vtable slot: `int* GetState()`.
pub(crate) const HMT_SLOT_STATE: u32 = 0x8;

/// Trigger vtable slot: `int* GetValue()` (the music cue).
pub(crate) const HMT_SLOT_VALUE: u32 = 0x10;

/// Audio system: custom-music flag (0 plays the default music for cue 0).
pub(crate) const HMT_CUSTOM_MUSIC: u32 = 0x530;

/// Audio system: music player sub-object; its vtable pointer is at 0x5558.
pub(crate) const HMT_MUSIC: u32 = 0x118;

pub(crate) const HMT_MUSIC_VT: u32 = 0x5558;

/// Audio system: a queued cue (flags and cue number).
pub(crate) const HMT_PENDING_ON: u32 = 0x5740;

pub(crate) const HMT_PENDING_FLAG2: u32 = 0x5750;

pub(crate) const HMT_PENDING_SET: u32 = 0x5758;

pub(crate) const HMT_PENDING_CUE: u32 = 0x575c;

/// Audio system: fade the cue (BankMonitor timer flag).
pub(crate) const HMT_FADE_MODE: u32 = 0x5790;

/// Audio system: last trigger state / value seen.
pub(crate) const HMT_LAST_STATE: u32 = 0x6264;

pub(crate) const HMT_LAST_VALUE: u32 = 0x6268;

/// Audio system: music may switch on the next cue.
pub(crate) const HMT_ARMED: u32 = 0x6270;

pub(crate) const HMT_PAUSED: u32 = 0x62b8;

pub(crate) const HMT_GET_INSTANCE: Callee = Callee {
    addr: 0x0028_b180,
    f: ee::s07::f_0028b180,
};

pub(crate) const HMT_CLEAR_TRIGGER: Callee = Callee {
    addr: 0x0028_d7d8,
    f: ee::s07::f_0028d7d8,
};

pub(crate) const HMT_FIND_RIDER: Callee = Callee {
    addr: 0x0028_5d98,
    f: ee::s07::f_00285d98,
};

pub(crate) const HMT_PLAY_MUSIC: Callee = Callee {
    addr: 0x0028_cf98,
    f: ee::s07::f_0028cf98,
};

pub(crate) const HMT_SET_MUSIC_STATE: Callee = Callee {
    addr: 0x0028_def0,
    f: ee::s07::f_0028def0,
};

pub(crate) const HMT_START_NEXT_SONG: Callee = Callee {
    addr: 0x0028_df18,
    f: ee::s07::f_0028df18,
};

pub(crate) const HMT_START_TRIGGER: Callee = Callee {
    addr: 0x0028_d630,
    f: ee::s07::f_0028d630,
};

pub(crate) const HMT_CONFIRM_TRIGGER: Callee = Callee {
    addr: 0x0028_d740,
    f: ee::s07::f_0028d740,
};

/// `SSXAudioSystem_IsRaceType6to11` (cross-shard: through the table).
pub(crate) const HMT_IS_RACE_TYPE_6_TO_11: u32 = 0x002a_42c8;

/// `SSXAudioSystem_IsEventMode4`.
pub(crate) const HMT_IS_EVENT_MODE_4: u32 = 0x002a_4040;

/// `BankMonitor_AddTimer`.
pub(crate) const HMT_ADD_TIMER: u32 = 0x002a_dca0;

/// The audio system (s0).
pub(crate) fn hmt_self(c: &Cpu) -> Addr {
    Addr(c.g[S0][0] as u32)
}

/// self + 0x118 (the music player), as `addiu` leaves it.
pub(crate) fn hmt_music(c: &Cpu) -> u64 {
    sx((c.g[S0][0] as u32).wrapping_add(HMT_MUSIC))
}

/// Call the trigger object's (s1) vtable method at `slot` (`{delta, fn}`),
/// loading the vtable into `vt` (v1, or a1 at the end), returning to `ret`.
pub(crate) fn hmt_trigger_call(c: &mut Cpu, vt: usize, slot: u32, ret: u32) -> crate::call::Flow {
    let obj = c.g[S1][0] as u32;
    c.g[vt][0] = Addr(obj).off(HMT_TRIGGER_VT).lw(c);
    let table = c.g[vt][0] as u32;
    c.g[A0][0] = Addr(table).off(slot).lh(c);
    c.g[V0][0] = Addr(table).off(slot + 4).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// Call the music player's method at `slot` of the vtable in v1 (a2 =
/// self+0x118 and a1 already set).
pub(crate) fn hmt_music_call(c: &mut Cpu, slot: u32, ret: u32) -> crate::call::Flow {
    let table = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(table).off(slot).lh(c);
    c.g[V0][0] = Addr(table).off(slot + 4).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `BankMonitor_AddTimer(*music + 0x1d8, 1500, k(0x4a3650), 0, fade, 0, 0,
/// 0)` with v0 = the music pointer (self+0x118, already loaded), then
/// disarm.
pub(crate) fn hmt_fade_timer(c: &mut Cpu, fade: u64, ret: u32) -> crate::call::Flow {
    c.cycles += 14;
    c.g[V1][0] = 0x4a_0000;
    c.ldl(A2, 0x004a_3657);
    c.ldr(A2, 0x004a_3650);
    c.g[A1][0] = 0x5dc;
    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[A3][0] = 0;
    c.g[T0][0] = fade;
    c.g[T1][0] = 0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
    c.g[T2][0] = 0;
    c.g[T3][0] = 0;
    call_ptr(c, HMT_ADD_TIMER, ret)?;
    hmt_self(c).off(HMT_ARMED).sw(c, 0);
    Ok(())
}

/// How the trigger compares with the last state/value seen.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum HmtCmp {
    StateDiffers,
    ValueDiffers,
    Same,
}

/// Compare the trigger with what was seen last: loads the state (call
/// returning to `ret_state`) and compares it with self+0x6264, then (only if
/// equal; 9 more cycles) the value with self+0x6268. Leaves v0/v1 as the
/// last compare does.
pub(crate) fn hmt_compare_last(
    c: &mut Cpu,
    ret_state: u32,
    ret_value: u32,
) -> Result<HmtCmp, crate::call::Unwound> {
    hmt_trigger_call(c, V1, HMT_SLOT_STATE, ret_state)?;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = hmt_self(c).off(HMT_LAST_STATE).lw(c);
    if c.g[V0][0] != c.g[V1][0] {
        return Ok(HmtCmp::StateDiffers);
    }
    c.cycles += 9;
    hmt_trigger_call(c, V1, HMT_SLOT_VALUE, ret_value)?;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = hmt_self(c).off(HMT_LAST_VALUE).lw(c);
    Ok(if c.g[V0][0] == c.g[V1][0] {
        HmtCmp::Same
    } else {
        HmtCmp::ValueDiffers
    })
}

/// The epilogue (`extra`: the 1-cycle block before the `lq s0` on the
/// normal path).
pub(crate) fn hmt_exit(c: &mut Cpu, frame: Frame, extra: u64) -> crate::call::Flow {
    c.cycles += extra;
    frame.restore(c, S0, 0x40);
    c.cycles += 6;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void SSXAudioSystem_HandleMusicTrigger(SSXAudioSystem* self, Rider* rider,
/// MusicTrigger* trigger)` (0x28d988): react to a course music trigger.
///
/// The trigger's state (vtable getter at 0x8) selects: 1 starts a music
/// trigger, 2 confirms one, -1 clears it (when the last state was 1 or 2) and
/// re-arms the music; 0 (only for the rider the audio follows) applies the
/// trigger's music cue (getter at 0x10): 0 plays the default music or sends
/// cue 0, 0xb (race end) and 0x12 (event mode 4) switch the music state and
/// start the next song or schedule timers, 0xd and 0xe..0x11 switch state
/// and queue the cue, 0x2a sends a one-shot cue once, others go straight to
/// the music player. Unless the clear call returned 0, the trigger's state
/// and value are remembered at 0x6264/0x6268. Frame 0x50 (ra at 0, s3..s0
/// at 0x10..0x40).
pub fn ssx_audio_system_handle_music_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        if c.g[S1][0] == 0 {
            return hmt_exit(c, frame, 1);
        }
        'done: {
            c.cycles += 9;
            c.g[S3][0] = 1;
            hmt_trigger_call(c, V1, HMT_SLOT_STATE, 0x0028_d9c8)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let state = c.g[V1][0];
            if state != 0 {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                if (state as i64) > 0 {
                    c.cycles += 2;
                    c.g[V0][0] = 2;
                    if state == c.g[S3][0] {
                        // State 1: start the trigger's cue.
                        c.cycles += 11;
                        hmt_trigger_call(c, V1, HMT_SLOT_VALUE, 0x0028_de5c)?;
                        c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                        c.g[A2][0] = c.g[S2][0];
                        c.g[A0][0] = c.g[S0][0];
                        HMT_START_TRIGGER.call(c, 0x0028_de6c)?;
                        break 'done;
                    }
                    c.cycles += 2;
                    if state != 2 {
                        c.cycles += 2;
                        break 'done;
                    }
                    // State 2: confirm it.
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).off(HMT_TRIGGER_VT).lw(c);
                    c.cycles += 8;
                    hmt_trigger_call(c, V1, HMT_SLOT_VALUE, 0x0028_de84)?;
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S0][0];
                    HMT_CONFIRM_TRIGGER.call(c, 0x0028_de94)?;
                    break 'done;
                }
                c.cycles += 2;
                if state != u64::MAX {
                    c.cycles += 2;
                    break 'done;
                }
                // State -1: clear a started/confirmed trigger, re-arm.
                c.g[V0][0] = hmt_self(c).off(HMT_LAST_STATE).lw(c);
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                let was_active = c.g[V0][0] < 2;
                c.g[V0][0] = 1;
                if was_active {
                    c.cycles += 7;
                    HMT_GET_INSTANCE.call(c, 0x0028_da1c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S2][0];
                    HMT_CLEAR_TRIGGER.call(c, 0x0028_da28)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V0][0] = 1;
                }
                c.cycles += 2;
                hmt_self(c).off(HMT_ARMED).sw(c, c.g[V0][0]);
                break 'done;
            }
            // State 0: only for the rider the audio follows.
            c.g[A0][0] = c.g[S0][0];
            c.cycles += 4;
            c.g[A1][0] = u64::MAX;
            HMT_FIND_RIDER.call(c, 0x0028_da40)?;
            if c.g[S2][0] != c.g[V0][0] {
                return hmt_exit(c, frame, 0);
            }
            c.cycles += 8;
            hmt_trigger_call(c, V1, HMT_SLOT_VALUE, 0x0028_da5c)?;
            c.g[S2][0] = Addr(c.g[V0][0] as u32).lw(c);
            let cue = c.g[S2][0];
            c.g[V0][0] = 0xb;
            // Ends in the music player's slot 0x20 call (0x28de30; v1 =
            // vtable, a2 = player, a1 = cue).
            'music: {
                if cue == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = hmt_self(c).off(HMT_CUSTOM_MUSIC).lw(c);
                    c.g[A2][0] = hmt_music(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 0;
                        c.g[A3][0] = u64::MAX;
                        c.g[T0][0] = 0;
                        HMT_PLAY_MUSIC.call(c, 0x0028_da8c)?;
                        break 'done;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = hmt_self(c).off(HMT_MUSIC_VT).lw(c);
                    c.g[A1][0] = 0;
                    break 'music;
                }
                c.cycles += 2;
                c.g[V0][0] = 0xd;
                // Ends in the state switch shared by cues 0xb and 0x12
                // (0x28dd4c; a0 = self, a1 = cue).
                'switch: {
                    if cue == 0xb {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, HMT_IS_RACE_TYPE_6_TO_11, 0x0028_dab0)?;
                        let race = c.g[V0][0] != 0;
                        c.g[V0][0] = hmt_self(c).off(HMT_ARMED).lw(c);
                        if race {
                            c.cycles += 2;
                            c.g[A1][0] = 0xb;
                            if c.g[V0][0] == 0 {
                                break 'done;
                            }
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S0][0];
                            break 'switch;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            break 'done;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0xb;
                        HMT_SET_MUSIC_STATE.call(c, 0x0028_dacc)?;
                        let ok = c.g[V0][0] != 0;
                        c.g[A2][0] = hmt_music(c);
                        if !ok {
                            break 'done;
                        }
                        c.cycles += 30;
                        c.g[V1][0] = hmt_self(c).off(HMT_MUSIC_VT).lw(c);
                        c.g[A1][0] = 0xb;
                        hmt_music_call(c, 0x20, 0x0028_daec)?;
                        c.g[V1][0] = hmt_self(c).off(HMT_MUSIC).lw(c);
                        c.g[V0][0] = 0x4a_0000;
                        c.g[T0][0] = hmt_self(c).off(HMT_PAUSED).lw(c);
                        c.g[T1][0] = 0;
                        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                        c.g[A1][0] = 0x7d0;
                        if c.g[T0][0] != 0 {
                            c.g[T1][0] = c.g[S3][0];
                        }
                        c.ldl(A2, 0x004a_364f);
                        c.ldr(A2, 0x004a_3648);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                        c.g[A3][0] = 0;
                        c.g[T0][0] = 0;
                        c.g[T2][0] = 0;
                        c.g[T3][0] = 0;
                        call_ptr(c, HMT_ADD_TIMER, 0x0028_db28)?;
                        match hmt_compare_last(c, 0x0028_db3c, 0x0028_db60)? {
                            HmtCmp::Same => {
                                hmt_self(c).off(HMT_ARMED).sw(c, 0);
                                break 'done;
                            }
                            HmtCmp::ValueDiffers => c.cycles += 1,
                            HmtCmp::StateDiffers => {}
                        }
                        // 0x28db74: a 1.5 s timer, fading or not.
                        let fade = hmt_self(c).off(HMT_FADE_MODE).lw(c);
                        c.g[V0][0] = fade;
                        c.cycles += 2;
                        c.g[V0][0] = hmt_self(c).off(HMT_MUSIC).lw(c);
                        if fade == 0 {
                            hmt_fade_timer(c, 0, 0x0028_dbac)?;
                        } else {
                            hmt_fade_timer(c, 1, 0x0028_dbe4)?;
                        }
                        break 'done;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((cue as u32).wrapping_sub(0xe));
                    if cue == 0xd {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0xd;
                        HMT_SET_MUSIC_STATE.call(c, 0x0028_dc10)?;
                        if c.g[V0][0] == 0 {
                            break 'done;
                        }
                        c.cycles += 9;
                        if hmt_compare_last(c, 0x0028_dc2c, 0x0028_dc50)? != HmtCmp::Same {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, HMT_IS_RACE_TYPE_6_TO_11, 0x0028_dc68)?;
                            if c.g[V0][0] == 0 {
                                hmt_self(c).off(HMT_PENDING_FLAG2).sw(c, c.g[S3][0]);
                            }
                            c.cycles += 1;
                        }
                        c.g[V1][0] = hmt_self(c).off(HMT_MUSIC_VT).lw(c);
                        c.cycles += 3;
                        c.g[A2][0] = hmt_music(c);
                        c.g[A1][0] = 0xd;
                        break 'music;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                    let mut queued = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !queued {
                        c.cycles += 4;
                        c.g[V0][0] = sx((cue as u32).wrapping_sub(0x10));
                        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                        queued = c.g[V0][0] != 0;
                        c.g[V0][0] = 0x12;
                    }
                    if queued {
                        // Cues 0xe..0x11: switch state and queue the cue.
                        c.cycles += 4;
                        c.g[A1][0] = c.g[S2][0];
                        HMT_SET_MUSIC_STATE.call(c, 0x0028_dca4)?;
                        if c.g[V0][0] == 0 {
                            break 'done;
                        }
                        c.cycles += 9;
                        if hmt_compare_last(c, 0x0028_dcc0, 0x0028_dce4)? == HmtCmp::Same {
                            break 'done;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, HMT_IS_RACE_TYPE_6_TO_11, 0x0028_dcfc)?;
                        if c.g[V0][0] != 0 {
                            break 'done;
                        }
                        c.cycles += 4;
                        hmt_self(c).off(HMT_PENDING_CUE).sw(c, c.g[S2][0]);
                        hmt_self(c).off(HMT_PENDING_SET).sw(c, c.g[S3][0]);
                        hmt_self(c).off(HMT_PENDING_ON).sw(c, c.g[S3][0]);
                        break 'done;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0x2a;
                    if cue == 0x12 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, HMT_IS_EVENT_MODE_4, 0x0028_dd24)?;
                        if c.g[V0][0] == 0 {
                            break 'done;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, HMT_IS_RACE_TYPE_6_TO_11, 0x0028_dd34)?;
                        if c.g[V0][0] != 0 {
                            break 'done;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = hmt_self(c).off(HMT_ARMED).lw(c);
                        c.g[A1][0] = 0x12;
                        if c.g[V0][0] == 0 {
                            break 'done;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S0][0];
                        break 'switch;
                    }
                    c.cycles += 2;
                    if cue != 0x2a {
                        c.g[V1][0] = hmt_self(c).off(HMT_MUSIC_VT).lw(c);
                        c.cycles += 2;
                        c.g[A2][0] = hmt_music(c);
                        c.g[A1][0] = c.g[S2][0];
                        break 'music;
                    }
                    // Cue 0x2a: a one-shot cue (slot 0x18) when it changed.
                    c.cycles += 9;
                    match hmt_compare_last(c, 0x0028_ddd0, 0x0028_ddf4)? {
                        HmtCmp::Same => break 'done,
                        HmtCmp::ValueDiffers => c.cycles += 1,
                        HmtCmp::StateDiffers => {}
                    }
                    c.g[V1][0] = hmt_self(c).off(HMT_MUSIC_VT).lw(c);
                    c.cycles += 8;
                    c.g[A2][0] = hmt_music(c);
                    c.g[A1][0] = 0x2a;
                    hmt_music_call(c, 0x18, 0x0028_de20)?;
                    break 'done;
                }
                // 0x28dd4c: switch the music state; start the next song if
                // the trigger changed; disarm.
                c.cycles += 4;
                HMT_SET_MUSIC_STATE.call(c, 0x0028_dd54)?;
                if c.g[V0][0] == 0 {
                    break 'done;
                }
                c.cycles += 9;
                let same = hmt_compare_last(c, 0x0028_dd70, 0x0028_dd94)? == HmtCmp::Same;
                c.g[A0][0] = c.g[S0][0];
                if !same {
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    HMT_START_NEXT_SONG.call(c, 0x0028_ddac)?;
                }
                c.cycles += 2;
                hmt_self(c).off(HMT_ARMED).sw(c, 0);
                break 'done;
            }
            // 0x28de30: the music player's slot 0x20 with the cue.
            c.cycles += 6;
            hmt_music_call(c, 0x20, 0x0028_de40)?;
        }
        // 0x28de94: remember the trigger's state and value.
        c.cycles += 2;
        if c.g[S3][0] == 0 {
            return hmt_exit(c, frame, 0);
        }
        c.cycles += 14;
        hmt_trigger_call(c, V1, HMT_SLOT_STATE, 0x0028_deb0)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        hmt_self(c).off(HMT_LAST_STATE).sw(c, c.g[V1][0]);
        hmt_trigger_call(c, A1, HMT_SLOT_VALUE, 0x0028_decc)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        hmt_self(c).off(HMT_LAST_VALUE).sw(c, c.g[V1][0]);
        hmt_exit(c, frame, 1)
    })
}

/// `SndBank_IsLoadedWithName` (0x28b830).
pub(crate) const OC762_BANK_IS_LOADED: Callee = Callee {
    addr: 0x0028_b830,
    f: ee::s07::f_0028b830,
};

/// `SndLoadQueue_CancelBank` (0x28a728).
pub(crate) const OC762_CANCEL_BANK: Callee = Callee {
    addr: 0x0028_a728,
    f: ee::s07::f_0028a728,
};

/// `SndBank_Load` (0x28b320).
pub(crate) const OC762_BANK_LOAD: Callee = Callee {
    addr: 0x0028_b320,
    f: ee::s07::f_0028b320,
};

/// `void SndBankMan_LoadBank(self, i, id, a3)` (0x28bc58,
/// `cBankManager::BANK_SyncLoad`): when the bank `id` is not loaded yet on
/// bank slot `i` (`self->banks + i * 0x60`), cancels its pending queue entry
/// and loads it synchronously.
pub fn snd_bank_man_load_bank_2(c: &mut Cpu) {
    guest(c, |c| {
        'b_28bcc8: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[V0][0] = 0x60;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S2][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.mult(0, 19, 18, 2);
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A2][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S4][0] = c.g[A3][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xacc));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
            OC762_BANK_IS_LOADED.call(c, 0x0028bc9c)?;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_28bcc8;
            } else {
                c.cycles += 9;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                OC762_CANCEL_BANK.call(c, 0x0028bcb0)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xacc));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S4][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                OC762_BANK_LOAD.call(c, 0x0028bcc4)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_28bcc8;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `void SSXAudioSystem_PlayMusic(AudioSystem* self, int startEvent, int
/// requestLine, int type, int skipPeakTheme)` (0x28cf98): start the music for
/// the current location: the peak's ambience bank in radio mode (`self+0x608c`
/// == 2), the peak theme (for locations from 17 up, unless skipped, requested
/// or a race type 6 to 11), else the normal music bank, then the fade-in
/// reset. Does nothing when a replay plays and `self+0x6294` is clear.
pub fn ssx_audio_system_play_music(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xe0 };
        'b_28d464: {
            'b_28cfe4: {
                c.cycles += 16;
                Frame::open(c, 0xe0);
                frame.save(c, S4, 0x90);
                frame.save(c, S0, 0xd0);
                c.g[S4][0] = c.g[A0][0];
                frame.save(c, S1, 0xc0);
                c.g[S0][0] = c.g[A3][0];
                frame.save(c, S2, 0xb0);
                c.g[S1][0] = c.g[A2][0];
                frame.save(c, S3, 0xa0);
                c.g[S2][0] = c.g[A1][0];
                frame.save(c, S5, 0x80);
                frame.save_ra(c, 0x70);
                c.g[S3][0] = c.g[T0][0];
                call_ptr(c, 0x00288ae0, 0x0028cfd0)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = 0xffffffffffffffff;
                if t {
                    break 'b_28cfe4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6294));
                    if c.g[V0][0] == 0u64 {
                        frame.restore(c, S0, 0xd0);
                        break 'b_28d464;
                    } else {
                        break 'b_28cfe4;
                    }
                }
            }
            'b_28d3c8: {
                'b_28d3b8: {
                    'b_28d170: {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x608c));
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x6280), 0u64);
                        c.g[V0][0] = 0x2;
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x62a0), c.g[A0][0]);
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x623c), 0u64);
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x6240), 0u64);
                        if t {
                            break 'b_28d170;
                        } else {
                            'b_28d034: {
                                c.cycles += 2;
                                let t = c.g[S0][0] != c.g[A0][0];
                                c.g[A3][0] = c.g[S0][0];
                                if t {
                                    break 'b_28d034;
                                } else {
                                    c.cycles += 11;
                                    call_ptr(c, 0x0028b1e8, 0x0028d010)?;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014dd58, 0x0028d01c)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x00144bc0, 0x0028d024)?;
                                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[A0][0] = c.g[S4][0];
                                    call_ptr(c, 0x002a1e20, 0x0028d030)?;
                                    c.g[A3][0] = c.g[V0][0];
                                    break 'b_28d034;
                                }
                            }
                            'b_28d14c: {
                                'b_28d09c: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[A3][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x2;
                                    if t {
                                        break 'b_28d09c;
                                    } else {
                                        c.cycles += 23;
                                        c.g[S3][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                        c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                                        c.g[S1][0] = 0x480000;
                                        c.g[A0][0] = c.g[S4][0];
                                        c.g[S0][0] = c.lh((c.g[S3][0] as u32).wrapping_add(0x10));
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2a38));
                                        c.g[A1][0] = 0x8;
                                        c.g[A2][0] = 0u64;
                                        c.g[S0][0] =
                                            sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                        c.g[S5][0] = c.g[S2][0];
                                        call_ptr(c, 0x00287968, 0x0028d06c)?;
                                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[A3][0] = c.g[V0][0];
                                        c.g[A2][0] = 0xc;
                                        let t = c.g[V1][0] as u32;
                                        c.g[T0][0] = 0u64;
                                        call_ptr(c, t, 0x0028d088)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[A1][0] = 0x65;
                                        call_ptr(c, 0x002b49c0, 0x0028d094)?;
                                        let t = true;
                                        if t {
                                            break 'b_28d14c;
                                        } else {
                                            break 'b_28d09c;
                                        }
                                    }
                                }
                                'b_28d0fc: {
                                    c.cycles += 2;
                                    let t = c.g[A3][0] != c.g[V0][0];
                                    c.g[S3][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                    if t {
                                        break 'b_28d0fc;
                                    } else {
                                        c.cycles += 22;
                                        c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                                        c.g[S1][0] = 0x480000;
                                        c.g[A0][0] = c.g[S4][0];
                                        c.g[S0][0] = c.lh((c.g[S3][0] as u32).wrapping_add(0x10));
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2a48));
                                        c.g[A1][0] = 0x8;
                                        c.g[A2][0] = 0u64;
                                        c.g[S0][0] =
                                            sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                        c.g[S5][0] = c.g[S2][0];
                                        call_ptr(c, 0x00287968, 0x0028d0cc)?;
                                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[A3][0] = c.g[V0][0];
                                        c.g[A2][0] = 0xc;
                                        let t = c.g[V1][0] as u32;
                                        c.g[T0][0] = 0u64;
                                        call_ptr(c, t, 0x0028d0e8)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[A1][0] = 0x66;
                                        call_ptr(c, 0x002b49c0, 0x0028d0f4)?;
                                        let t = true;
                                        if t {
                                            break 'b_28d14c;
                                        } else {
                                            break 'b_28d0fc;
                                        }
                                    }
                                }
                                c.cycles += 20;
                                c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                                c.g[S1][0] = 0x480000;
                                c.g[A0][0] = c.g[S4][0];
                                c.g[S0][0] = c.lh((c.g[S3][0] as u32).wrapping_add(0x10));
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2a58));
                                c.g[A1][0] = 0x8;
                                c.g[A2][0] = 0u64;
                                c.g[S0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                call_ptr(c, 0x00287968, 0x0028d120)?;
                                c.g[S5][0] = c.g[S2][0];
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A3][0] = c.g[V0][0];
                                c.g[A2][0] = 0xc;
                                let t = c.g[V1][0] as u32;
                                c.g[T0][0] = 0u64;
                                call_ptr(c, t, 0x0028d140)?;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = 0x67;
                                call_ptr(c, 0x002b49c0, 0x0028d14c)?;
                                break 'b_28d14c;
                            }
                            c.cycles += 9;
                            c.g[A0][0] = c.g[S5][0];
                            call_ptr(c, 0x002b46d0, 0x0028d154)?;
                            c.g[V0][0] = 0x1;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[A1][0] = 0u64;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x6280), c.g[V0][0]);
                            call_ptr(c, 0x002b3cc0, 0x0028d168)?;
                            let t = true;
                            c.g[A0][0] = c.g[S5][0];
                            if t {
                                break 'b_28d3b8;
                            } else {
                                break 'b_28d170;
                            }
                        }
                    }
                    'b_28d22c: {
                        'b_28d1f8: {
                            c.cycles += 11;
                            call_ptr(c, 0x0014dc80, 0x0028d178)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x0014dd58, 0x0028d184)?;
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x00144bc0, 0x0028d18c)?;
                            c.g[V1][0] = c.g[V0][0];
                            c.g[V0][0] = c.g[V1][0] & 0x7;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[SP][0];
                            if t {
                                break 'b_28d1f8;
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                                c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                                'l_28d1a4: loop {
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
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0028d1a4);
                                        }
                                        continue 'l_28d1a4;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        if t {
                                            break 'b_28d22c;
                                        } else {
                                            break 'b_28d1f8;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                        c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                        'l_28d200: loop {
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
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0028d200);
                                }
                                continue 'l_28d200;
                            } else {
                                break 'b_28d22c;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 17i64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                        break 'b_28d3c8;
                    } else {
                        c.cycles += 2;
                        if c.g[S1][0] != 0u64 {
                            c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                            break 'b_28d3c8;
                        } else {
                            c.cycles += 2;
                            if c.g[S3][0] != 0u64 {
                                c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                break 'b_28d3c8;
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S4][0];
                                call_ptr(c, 0x002a42c8, 0x0028d25c)?;
                                if c.g[V0][0] != 0u64 {
                                    c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                    break 'b_28d3c8;
                                } else {
                                    'b_28d394: {
                                        'b_28d2f0: {
                                            c.cycles += 14;
                                            call_ptr(c, 0x0028b1e8, 0x0028d26c)?;
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x0014dd58, 0x0028d278)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x00144bc0, 0x0028d280)?;
                                            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[A0][0] = c.g[S4][0];
                                            call_ptr(c, 0x002a1e20, 0x0028d28c)?;
                                            c.g[V1][0] = c.g[V0][0];
                                            c.g[V0][0] = 0x1;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_28d2f0;
                                            } else {
                                                c.cycles += 21;
                                                c.g[S2][0] =
                                                    c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                                c.g[S1][0] = 0x4a0000;
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x3630));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[S0][0] =
                                                    c.lh((c.g[S2][0] as u32).wrapping_add(0x10));
                                                c.g[A1][0] = 0x1;
                                                c.g[A2][0] = 0u64;
                                                c.g[S0][0] = sx((c.g[S5][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                                call_ptr(c, 0x00287968, 0x0028d2c0)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] = c.g[S1][0];
                                                c.g[A3][0] = c.g[V0][0];
                                                c.g[A2][0] = 0xc;
                                                let t = c.g[V1][0] as u32;
                                                c.g[T0][0] = 0u64;
                                                call_ptr(c, t, 0x0028d2dc)?;
                                                c.g[A0][0] = c.g[S5][0];
                                                c.g[A1][0] = 0x1;
                                                call_ptr(c, 0x002b49c0, 0x0028d2e8)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S5][0];
                                                if t {
                                                    break 'b_28d394;
                                                } else {
                                                    break 'b_28d2f0;
                                                }
                                            }
                                        }
                                        'b_28d348: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[S2][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                            if t {
                                                break 'b_28d348;
                                            } else {
                                                c.cycles += 20;
                                                c.g[S1][0] = 0x4a0000;
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x3638));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[S0][0] =
                                                    c.lh((c.g[S2][0] as u32).wrapping_add(0x10));
                                                c.g[A1][0] = 0x1;
                                                c.g[A2][0] = 0u64;
                                                c.g[S0][0] = sx((c.g[S5][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                                call_ptr(c, 0x00287968, 0x0028d318)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] = c.g[S1][0];
                                                c.g[A3][0] = c.g[V0][0];
                                                c.g[A2][0] = 0xc;
                                                let t = c.g[V1][0] as u32;
                                                c.g[T0][0] = 0u64;
                                                call_ptr(c, t, 0x0028d334)?;
                                                c.g[A0][0] = c.g[S5][0];
                                                c.g[A1][0] = 0x2;
                                                call_ptr(c, 0x002b49c0, 0x0028d340)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S5][0];
                                                if t {
                                                    break 'b_28d394;
                                                } else {
                                                    break 'b_28d348;
                                                }
                                            }
                                        }
                                        c.cycles += 19;
                                        c.g[S1][0] = 0x4a0000;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3640));
                                        c.g[A0][0] = c.g[S4][0];
                                        c.g[S0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = 0x1;
                                        c.g[A2][0] = 0u64;
                                        c.g[S0][0] =
                                            sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                        call_ptr(c, 0x00287968, 0x0028d368)?;
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[A3][0] = c.g[V0][0];
                                        c.g[A2][0] = 0xc;
                                        let t = c.g[V1][0] as u32;
                                        c.g[T0][0] = 0u64;
                                        call_ptr(c, t, 0x0028d384)?;
                                        c.g[A0][0] = c.g[S5][0];
                                        c.g[A1][0] = 0x3;
                                        call_ptr(c, 0x002b49c0, 0x0028d390)?;
                                        c.g[A0][0] = c.g[S5][0];
                                        break 'b_28d394;
                                    }
                                    c.cycles += 9;
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x002b3cc0, 0x0028d39c)?;
                                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                    c.g[A1][0] = 0xc;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0028d3b4)?;
                                    c.g[A0][0] = c.g[S5][0];
                                    break 'b_28d3b8;
                                }
                            }
                        }
                    }
                }
                c.cycles += 4;
                c.g[A1][0] = 0x7f;
                call_ptr(c, 0x002b3c28, 0x0028d3c0)?;
                let t = true;
                frame.restore(c, S0, 0xd0);
                if t {
                    break 'b_28d464;
                } else {
                    break 'b_28d3c8;
                }
            }
            'b_28d430: {
                c.cycles += 23;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0u64;
                c.g[S0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
                call_ptr(c, 0x00287968, 0x0028d3e0)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                c.g[A3][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 0xc;
                let t = c.g[V1][0] as u32;
                c.g[T0][0] = 0u64;
                call_ptr(c, t, 0x0028d3fc)?;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x508));
                c.g[A0][0] = c.g[S5][0];
                c.sw((c.g[S4][0] as u32).wrapping_add(0x62a4), c.g[V0][0]);
                call_ptr(c, 0x002b4908, 0x0028d40c)?;
                c.sw((c.g[S4][0] as u32).wrapping_add(0x5778), c.g[V0][0]);
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x002b3cc0, 0x0028d41c)?;
                let t = c.g[S3][0] == 0u64;
                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                if t {
                    break 'b_28d430;
                } else {
                    c.cycles += 3;
                    c.g[V1][0] = 0x24;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 17i64) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S2][0] = c.g[V1][0];
                    }
                    break 'b_28d430;
                }
            }
            c.cycles += 13;
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0028d448)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0xffffffffffffffff;
            call_ptr(c, 0x00285d98, 0x0028d454)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0028f108, 0x0028d460)?;
            frame.restore(c, S0, 0xd0);
            break 'b_28d464;
        }
        c.cycles += 8;
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        return Ok(());
    });
}

/// `void SSXAudioSystem_SetMusicMode(SSXAudioSystem* self, int mode)` (0x28bf78): switches the radio mode (0 default map, 1 and 3 radio playlist, 2 off). Stores the mode at +0x608c (no-op when unchanged), applies the music-player playlist mask, copies the BE option block to the stack to read the DJ-speech flag (bit 17) for `SetMusicChannelEnabled`, and, when in game with a song playing and the player state allows it, starts a new random song with `PlayMusic`.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_labels
)]
pub fn ssx_audio_system_set_music_mode(c: &mut Cpu) {
    'b_28c2bc: {
        'b_28c2b8: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2d0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2b0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2a0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x290), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x608c));
            let t = c.g[V0][0] == c.g[A1][0];
            c.g[V0][0] = 0x1;
            if t {
                break 'b_28c2b8;
            } else {
                'b_28c264: {
                    'b_28c23c: {
                        'b_28c0fc: {
                            'b_28c0f8: {
                                c.cycles += 2;
                                let t = c.g[A1][0] == c.g[V0][0];
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x608c), c.g[A1][0]);
                                if t {
                                    break 'b_28c0f8;
                                } else {
                                    'b_28bfd8: {
                                        'b_28bfc0: {
                                            c.cycles += 3;
                                            c.g[V0][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V0][0] = 0x2;
                                                break 'b_28bfc0;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A1][0] == 0u64;
                                                c.g[V0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x118));
                                                if t {
                                                    break 'b_28bfd8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x2c0),
                                                    );
                                                    if t {
                                                        break 'b_28c2bc;
                                                    } else {
                                                        break 'b_28bfc0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[A1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x3;
                                        if t {
                                            break 'b_28c264;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A1][0] == c.g[V0][0];
                                            c.g[V0][0] =
                                                sx((c.g[S0][0] as u32).wrapping_add(0x118));
                                            if t {
                                                break 'b_28c0fc;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                                if t {
                                                    break 'b_28c2bc;
                                                } else {
                                                    break 'b_28bfd8;
                                                }
                                            }
                                        }
                                    }
                                    'b_28c098: {
                                        'b_28c068: {
                                            c.cycles += 14;
                                            c.g[A1][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x518));
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x0028bfe8);
                                            c.g[S2][0] = c.g[V0][0];
                                            c.pc = 0x002b4070;
                                            call_indirect(c, 0x002b4070);
                                            if c.pc != 0x0028bfe8 {
                                                return;
                                            }
                                            c.g[RA][0] = sx(0x0028bff0);
                                            c.pc = 0x0014dc80;
                                            call_indirect(c, 0x0014dc80);
                                            if c.pc != 0x0028bff0 {
                                                return;
                                            }
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x0028bffc);
                                            c.g[A1][0] = 0x4;
                                            c.pc = 0x0014dd58;
                                            call_indirect(c, 0x0014dd58);
                                            if c.pc != 0x0028bffc {
                                                return;
                                            }
                                            c.g[V1][0] = 0x530000;
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                                            c.g[V0][0] = c.g[V1][0] & 0x7;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A0][0] = c.g[SP][0];
                                            if t {
                                                break 'b_28c068;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x280));
                                                'l_28c014: loop {
                                                    c.cycles += 19;
                                                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.ldr(5, (c.g[V1][0] as u32));
                                                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.ldl(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.ldr(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.ldl(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.ldr(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                                                    c.sdr(5, (c.g[A0][0] as u32));
                                                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                                                    c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        7,
                                                        (c.g[A0][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.sdr(
                                                        7,
                                                        (c.g[A0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sdl(
                                                        8,
                                                        (c.g[A0][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.sdr(
                                                        8,
                                                        (c.g[A0][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if c.cycles >= c.next_event {
                                                            c.pc = 0x0028c014;
                                                            c.event();
                                                        }
                                                        continue 'l_28c014;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_28c098;
                                                        } else {
                                                            break 'b_28c068;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                                        'l_28c06c: loop {
                                            c.cycles += 11;
                                            c.g[A1][0] = c.ld((c.g[V1][0] as u32));
                                            c.g[A2][0] =
                                                c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                                            c.g[A3][0] =
                                                c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                                            c.g[T0][0] =
                                                c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                                            c.sd((c.g[A0][0] as u32), c.g[A1][0]);
                                            c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                                            c.sd(
                                                (c.g[A0][0] as u32).wrapping_add(0x10),
                                                c.g[A3][0],
                                            );
                                            c.sd(
                                                (c.g[A0][0] as u32).wrapping_add(0x18),
                                                c.g[T0][0],
                                            );
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if c.cycles >= c.next_event {
                                                    c.pc = 0x0028c06c;
                                                    c.event();
                                                }
                                                continue 'l_28c06c;
                                            } else {
                                                break 'b_28c098;
                                            }
                                        }
                                    }
                                    c.cycles += 13;
                                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                                    c.ldr(2, (c.g[V1][0] as u32));
                                    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
                                    c.sdr(2, (c.g[A0][0] as u32));
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
                                    c.g[RA][0] = sx(0x0028c0bc);
                                    c.g[A1][0] = c.g[A1][0] & 0x1;
                                    c.pc = 0x00287558;
                                    call_indirect(c, 0x00287558);
                                    if c.pc != 0x0028c0bc {
                                        return;
                                    }
                                    c.g[RA][0] = sx(0x0028c0c4);
                                    c.pc = 0x0028b1b0;
                                    call_indirect(c, 0x0028b1b0);
                                    if c.pc != 0x0028c0c4 {
                                        return;
                                    }
                                    if c.g[V0][0] == 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                        break 'b_28c2bc;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x530));
                                        if c.g[V0][0] == 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                            break 'b_28c2bc;
                                        } else {
                                            c.cycles += 6;
                                            c.g[RA][0] = sx(0x0028c0e0);
                                            c.g[A0][0] = c.g[S2][0];
                                            c.pc = 0x002b49e0;
                                            call_indirect(c, 0x002b49e0);
                                            if c.pc != 0x0028c0e0 {
                                                return;
                                            }
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x65));
                                            c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                                            if c.g[V0][0] == 0u64 {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                                break 'b_28c2bc;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_28c23c;
                                                } else {
                                                    break 'b_28c0f8;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                            break 'b_28c0fc;
                        }
                        'b_28c1bc: {
                            'b_28c18c: {
                                c.cycles += 14;
                                c.g[A1][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x6098));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x0028c10c);
                                c.g[S2][0] = c.g[V0][0];
                                c.pc = 0x002b4070;
                                call_indirect(c, 0x002b4070);
                                if c.pc != 0x0028c10c {
                                    return;
                                }
                                c.g[RA][0] = sx(0x0028c114);
                                c.pc = 0x0014dc80;
                                call_indirect(c, 0x0014dc80);
                                if c.pc != 0x0028c114 {
                                    return;
                                }
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x0028c120);
                                c.g[A1][0] = 0x4;
                                c.pc = 0x0014dd58;
                                call_indirect(c, 0x0014dd58);
                                if c.pc != 0x0028c120 {
                                    return;
                                }
                                c.g[V1][0] = 0x530000;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                                c.g[V0][0] = c.g[V1][0] & 0x7;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.g[SP][0];
                                if t {
                                    break 'b_28c18c;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                                    'l_28c138: loop {
                                        c.cycles += 19;
                                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                                        c.ldr(8, (c.g[V1][0] as u32));
                                        c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0xf));
                                        c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                                        c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                                        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                                        c.sdr(8, (c.g[A0][0] as u32));
                                        c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0xf));
                                        c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                                        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
                                        c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
                                        c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                        c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x18));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                        if t {
                                            if c.cycles >= c.next_event {
                                                c.pc = 0x0028c138;
                                                c.event();
                                            }
                                            continue 'l_28c138;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_28c1bc;
                                            } else {
                                                break 'b_28c18c;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                            'l_28c190: loop {
                                c.cycles += 11;
                                c.g[T0][0] = c.ld((c.g[V1][0] as u32));
                                c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                                c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                                c.sd((c.g[A0][0] as u32), c.g[T0][0]);
                                c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                                c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                if t {
                                    if c.cycles >= c.next_event {
                                        c.pc = 0x0028c190;
                                        c.event();
                                    }
                                    continue 'l_28c190;
                                } else {
                                    break 'b_28c1bc;
                                }
                            }
                        }
                        c.cycles += 12;
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(8, (c.g[V1][0] as u32));
                        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                        c.sdr(8, (c.g[A0][0] as u32));
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
                        c.g[RA][0] = sx(0x0028c1e0);
                        c.g[A1][0] = c.g[A1][0] & 0x1;
                        c.pc = 0x00287558;
                        call_indirect(c, 0x00287558);
                        if c.pc != 0x0028c1e0 {
                            return;
                        }
                        c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x6098));
                        if c.g[V0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                            break 'b_28c2bc;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x0028c1f4);
                            c.pc = 0x0028b1b0;
                            call_indirect(c, 0x0028b1b0);
                            if c.pc != 0x0028c1f4 {
                                return;
                            }
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                break 'b_28c2bc;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x530));
                                if c.g[V0][0] == 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                    break 'b_28c2bc;
                                } else {
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x0028c210);
                                    c.g[A0][0] = c.g[S2][0];
                                    c.pc = 0x002b49e0;
                                    call_indirect(c, 0x002b49e0);
                                    if c.pc != 0x0028c210 {
                                        return;
                                    }
                                    c.g[S1][0] = c.g[V0][0];
                                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                        break 'b_28c2bc;
                                    } else {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x0028c22c);
                                        c.g[A0][0] = c.g[S2][0];
                                        c.pc = 0x002b4af0;
                                        call_indirect(c, 0x002b4af0);
                                        if c.pc != 0x0028c22c {
                                            return;
                                        }
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x12d;
                                        if t {
                                            break 'b_28c23c;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[S1][0] != c.g[V0][0] {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                                break 'b_28c2bc;
                                            } else {
                                                break 'b_28c23c;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x0028c244);
                    c.g[A0][0] = c.g[S0][0];
                    c.pc = 0x0028d488;
                    call_indirect(c, 0x0028d488);
                    if c.pc != 0x0028c244 {
                        return;
                    }
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0xffffffffffffffff;
                    c.g[RA][0] = sx(0x0028c25c);
                    c.g[T0][0] = 0u64;
                    c.pc = 0x0028cf98;
                    call_indirect(c, 0x0028cf98);
                    if c.pc != 0x0028c25c {
                        return;
                    }
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                    if t {
                        break 'b_28c2bc;
                    } else {
                        break 'b_28c264;
                    }
                }
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0028c270);
                c.g[A1][0] = 0u64;
                c.pc = 0x00287558;
                call_indirect(c, 0x00287558);
                if c.pc != 0x0028c270 {
                    return;
                }
                c.g[RA][0] = sx(0x0028c278);
                c.pc = 0x0028b1b0;
                call_indirect(c, 0x0028b1b0);
                if c.pc != 0x0028c278 {
                    return;
                }
                if c.g[V0][0] == 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                    break 'b_28c2bc;
                } else {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x0028c288);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                    c.pc = 0x002b49e0;
                    call_indirect(c, 0x002b49e0);
                    if c.pc != 0x0028c288 {
                        return;
                    }
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x65));
                    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                        break 'b_28c2bc;
                    } else {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0xffffffffffffffff;
                        c.g[RA][0] = sx(0x0028c2b0);
                        c.g[T0][0] = 0u64;
                        c.pc = 0x0028cf98;
                        call_indirect(c, 0x0028cf98);
                        if c.pc != 0x0028c2b0 {
                            return;
                        }
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x5fd0), 0u64);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x5fd4), 0u64);
                        break 'b_28c2b8;
                    }
                }
            }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        break 'b_28c2bc;
    }
    c.cycles += 5;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2b0));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2a0));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x290));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2d0));
    c.pc = t;
    return;
}

/// `void SSXAudioSystem_PlayLoadingScreenLoop(SSXAudioSystem* self)` (0x28f768): pushes the loading-screen loop voice (id 0xd) onto the bank manager's tables, clamps its base volume to 127, sets the volume channel and stores the pushed-sound handle at +0x62a8.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_labels
)]
pub fn ssx_audio_system_play_loading_screen_loop(c: &mut Cpu) {
    'b_28fa38: {
        'b_28fa0c: {
            c.cycles += 163;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[A1][0] = 0u64;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A2][0] = 0x8;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S2][0] = 0x18;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S3][0] = 0xd;
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S4][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = c.lw((c.g[S4][0] as u32));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.mult(0, 3, 3, 18);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
            c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
            c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
            c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
            c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.sdr(2, (c.g[V1][0] as u32));
            c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
            c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
            c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
            c.mult(0, 4, 4, 18);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
            c.g[V1][0] = c.lb((c.g[A0][0] as u32));
            c.sw((c.g[A3][0] as u32), c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[RA][0] = sx(0x0028f860);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
            c.pc = 0x00416210;
            call_indirect(c, 0x00416210);
            if c.pc != 0x0028f860 {
                return;
            }
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = 0x64;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
            c.g[A1][0] = 0x5a;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[A2][0] = 0x32;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A3][0] = 0x7f;
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[T0][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[A0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[A3][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), c.g[T0][0]);
            c.g[A0][0] = c.lw((c.g[S4][0] as u32));
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[S3][0]);
            c.g[V1][0] = c.lw((c.g[S4][0] as u32));
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[A3][0]);
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
            c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
            c.mult(0, 4, 4, 18);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            if t {
                break 'b_28fa0c;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                let t = (c.g[V1][0] as i64) < 0;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_28fa38;
                } else {
                    break 'b_28fa0c;
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
        if c.g[V1][0] == 0u64 {
            c.g[V0][0] = 0x7f;
            break 'b_28fa38;
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
            break 'b_28fa38;
        }
    }
    c.cycles += 23;
    c.sb((c.g[A0][0] as u32), c.g[V0][0]);
    c.g[A1][0] = 0x1;
    c.g[A0][0] = c.g[S4][0];
    c.g[A2][0] = 0u64;
    c.g[RA][0] = sx(0x0028fa50);
    c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
    c.pc = 0x00287968;
    call_indirect(c, 0x00287968);
    if c.pc != 0x0028fa50 {
        return;
    }
    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
    c.g[A0][0] = c.g[S4][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.g[RA][0] = sx(0x0028fa70);
    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
    c.pc = 0x002906b8;
    call_indirect(c, 0x002906b8);
    if c.pc != 0x0028fa70 {
        return;
    }
    c.sw((c.g[S4][0] as u32).wrapping_add(0x62a8), c.g[V0][0]);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t;
    return;
}

/// `void SSXAudioSystem_SetFEMusicScreen(SSXAudioSystem* sys, int screen)` (0x28f140).
///
/// Remembers the front-end screen (`+0x6290`) and, when it changed, maps it
/// through a jump table (0x482a70, screens 0..=9) to a music bank (screen 0
/// picks bank 9 or 0 by the word at `+0x534`) and tells the music system
/// (`sys + 0x118`, vtable at `+0x5558`, slot `+0x20`) to switch to it.
pub fn ssx_audio_system_set_fe_music_screen(c: &mut Cpu) {
    if c.pc != 0x0028_f140 {
        return ee::s07::f_0028f140__entries(c);
    }
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(FE_MUSIC_SCREEN)).lw(c);
        let same = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[A1][0] < 0xa) as u64;
        if same {
            c.cycles += 1;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        let in_range = c.g[V0][0] != 0;
        c.sw(
            (c.g[A2][0] as u32).wrapping_add(FE_MUSIC_SCREEN),
            c.g[A1][0],
        );
        if in_range {
            c.cycles += 7;
            c.g[V0][0] = 0x480000;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2a70));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            // Each case of the jump table: the bank it selects.
            let bank: u64 = match target {
                0x0028_f17c => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x534)).lw(c);
                    c.g[A1][0] = 0x9;
                    c.g[V0][0] = !(0u64 | c.g[V0][0]);
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = 0;
                    }
                    c.g[A1][0]
                }
                0x0028_f190 => {
                    c.cycles += 2;
                    1
                }
                0x0028_f198 => {
                    c.cycles += 2;
                    3
                }
                0x0028_f1a0 => {
                    c.cycles += 2;
                    4
                }
                0x0028_f1a8 => {
                    c.cycles += 2;
                    5
                }
                0x0028_f1b0 => {
                    c.cycles += 2;
                    6
                }
                0x0028_f1b8 => {
                    c.cycles += 2;
                    7
                }
                0x0028_f1c0 => {
                    c.cycles += 2;
                    8
                }
                0x0028_f1c8 => {
                    c.cycles += 2;
                    2
                }
                _ => {
                    c.jump_out(target, 0x0028_f140, 0x0028_f200);
                    return Ok(());
                }
            };
            c.g[A1][0] = bank;
        } else {
            c.cycles += 1;
            c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 2;
        let no_bank = (c.g[A1][0] as i64) < 0;
        frame.restore_ra(c, 0);
        if !no_bank {
            c.cycles += 6;
            c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(FE_MUSIC_SYS_VTABLE)).lw(c);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x118));
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
            let method = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, 0x0028_f1f4)?;
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem` fields of `ssx_audio_system_set_fe_music_screen`.
pub(crate) const FE_MUSIC_SCREEN: u32 = 0x6290;

pub(crate) const FE_MUSIC_SYS_VTABLE: u32 = 0x5558;

/// `void SSXAudioSystem_SetRequestPlaylist(SSXAudioSystem* sys, long key)`
/// (0x28c2d0, frame 0x40): stores the requested radio playlist (+0x6098).
/// When the radio mode (+0x608c) is 1 or 3 and the playlist changed, first
/// calls `func_002B4070` on the music system at +0x118; then (modes 1 and 3
/// only, a changed playlist or not) while in a game with a music system
/// (+0x530) whose state is not 1..3 and (state 0x12d or no pending
/// request) restarts playback with `SelectRandomSong` and `PlayMusic`.
pub fn ssx_audio_system_set_request_playlist(c: &mut Cpu) {
    guest(c, |c| {
        'b_28c418: {
            'b_28c390: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x6098));
                let t = c.g[V0][0] == c.g[A1][0];
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_28c390;
                } else {
                    'b_28c30c: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x608c));
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.sd((c.g[S0][0] as u32).wrapping_add(0x6098), c.g[A1][0]);
                        if t {
                            break 'b_28c30c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x3;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                break 'b_28c418;
                            } else {
                                break 'b_28c30c;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                    c.g[RA][0] = sx(0x0028c318);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x002b4070, 0x0028c318)?;
                    c.g[RA][0] = sx(0x0028c320);
                    Callee {
                        addr: 0x0028b1b0,
                        f: ee::s07::SSXAudioSystem_IsInGame,
                    }
                    .call(c, 0x0028c320)?;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        break 'b_28c418;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x530));
                        if c.g[V0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_28c418;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x0028c33c);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x002b49e0, 0x0028c33c)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                break 'b_28c418;
                            } else {
                                'b_28c368: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x0028c358);
                                    c.g[A0][0] = c.g[S1][0];
                                    call_ptr(c, 0x002b4af0, 0x0028c358)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x12d;
                                    if t {
                                        break 'b_28c368;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[S2][0] != c.g[V0][0] {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            break 'b_28c418;
                                        } else {
                                            break 'b_28c368;
                                        }
                                    }
                                }
                                c.cycles += 10;
                                c.g[RA][0] = sx(0x0028c370);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x0028d488,
                                    f: ee::s07::SSXAudioSystem_SelectRandomSong,
                                }
                                .call(c, 0x0028c370)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0xffffffffffffffff;
                                c.g[RA][0] = sx(0x0028c388);
                                c.g[T0][0] = 0u64;
                                Callee {
                                    addr: 0x0028cf98,
                                    f: ee::s07::SSXAudioSystem_PlayMusic,
                                }
                                .call(c, 0x0028c388)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_28c418;
                                } else {
                                    break 'b_28c390;
                                }
                            }
                        }
                    }
                }
            }
            'b_28c3a4: {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x608c));
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x3;
                if t {
                    break 'b_28c3a4;
                } else {
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        break 'b_28c418;
                    } else {
                        break 'b_28c3a4;
                    }
                }
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x0028c3ac);
            Callee {
                addr: 0x0028b1b0,
                f: ee::s07::SSXAudioSystem_IsInGame,
            }
            .call(c, 0x0028c3ac)?;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_28c418;
            } else {
                'b_28c414: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x530));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                    if t {
                        break 'b_28c414;
                    } else {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x0028c3c8);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x002b49e0, 0x0028c3c8)?;
                        c.g[S2][0] = c.g[V0][0];
                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_28c418;
                        } else {
                            'b_28c3f4: {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x0028c3e4);
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x002b4af0, 0x0028c3e4)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x12d;
                                if t {
                                    break 'b_28c3f4;
                                } else {
                                    c.cycles += 2;
                                    if c.g[S2][0] != c.g[V0][0] {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        break 'b_28c418;
                                    } else {
                                        break 'b_28c3f4;
                                    }
                                }
                            }
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x0028c3fc);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x0028d488,
                                f: ee::s07::SSXAudioSystem_SelectRandomSong,
                            }
                            .call(c, 0x0028c3fc)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0xffffffffffffffff;
                            c.g[RA][0] = sx(0x0028c414);
                            c.g[T0][0] = 0u64;
                            Callee {
                                addr: 0x0028cf98,
                                f: ee::s07::SSXAudioSystem_PlayMusic,
                            }
                            .call(c, 0x0028c414)?;
                            break 'b_28c414;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_28c418;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `cBankManager* SndBankMan_ctor(cBankManager* this, int embedded, int count, heap)`
/// (0x28b9b8, frame 0x90): with `embedded` set, builds the sound system
/// sub-object at +0xad0 (and points +0x1d8 at it); runs the load queue and
/// bank monitor ctors, sets the vtables (0x483ab8, the copy to the stack when
/// not embedded), stores `count` at +0xac8, allocates `count` 0x60 byte
/// `BankInstances` banks (constructing each) and publishes `this` in the
/// global at gp+0x524. Returns `this`.
pub fn snd_bank_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_28ba00: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S0][0] = c.g[A3][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S1][0] = c.g[A1][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S2][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = c.g[A2][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            let t = c.g[S1][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            if t {
                break 'b_28ba00;
            } else {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xad0));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0028ba00);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x1d8), c.g[V0][0]);
                call_ptr(c, 0x002ade88, 0x0028ba00)?;
                break 'b_28ba00;
            }
        }
        'b_28ba6c: {
            c.cycles += 11;
            c.g[RA][0] = sx(0x0028ba08);
            c.g[A0][0] = c.g[S2][0];
            Callee {
                addr: 0x0028a058,
                f: ee::s07::SndLoadQueue_ctor,
            }
            .call(c, 0x0028ba08)?;
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1d8));
            c.g[RA][0] = sx(0x0028ba18);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x002acd08, 0x0028ba18)?;
            c.g[V1][0] = 0x480000;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1d8));
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3ab8));
            let t = c.g[S1][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
            if t {
                break 'b_28ba6c;
            } else {
                c.cycles += 16;
                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x3ab8));
                c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1d8));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd(c.g[SP][0] as u32, c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1d8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xad0));
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                break 'b_28ba6c;
            }
        }
        'b_28badc: {
            c.cycles += 21;
            c.g[A0][0] = 0x60;
            c.g[V0][0] = 0x480000;
            c.mult(0, 4, 20, 4);
            c.g[V1][0] = 0x480000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3b48));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3ad8));
            c.sw((c.g[S2][0] as u32).wrapping_add(0xac8), c.g[S4][0]);
            c.g[A1][0] = 0x480000;
            c.sw((c.g[S2][0] as u32).wrapping_add(0xab0), c.g[V0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x29b8));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x1d4), c.g[V1][0]);
            c.g[A0][0] = c.g[A0][0] | 0x10;
            c.g[A2][0] = 0u64;
            c.g[RA][0] = sx(0x0028baa8);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0028baa8)?;
            c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(0xacc));
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
            c.sw(c.g[V0][0] as u32, c.g[S4][0]);
            let t = c.g[S4][0] == 0u64;
            c.g[S0][0] = c.g[S3][0];
            if t {
                break 'b_28badc;
            } else {
                c.cycles += 2;
                c.g[S4][0] = 0xffffffffffffffff;
                'l_28bac8: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0028bad4);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    Callee {
                        addr: 0x0028b248,
                        f: ee::s07::SndBank_ctor,
                    }
                    .call(c, 0x0028bad4)?;
                    let t = c.g[S1][0] != c.g[S4][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x60));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0028bac8);
                        }
                        continue 'l_28bac8;
                    } else {
                        break 'b_28badc;
                    }
                }
            }
        }
        c.cycles += 12;
        c.sw(c.g[S5][0] as u32, c.g[S3][0]);
        c.g[V0][0] = c.g[S2][0];
        c.sw((c.g[GP][0] as u32).wrapping_add(0x524), c.g[S2][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

// 0x002f5298 WorldLightMan_AllocOverlapList: already ported (game/visualfx/worldlightman.rs), skipped.

/// `void* SndBank_GetBuffer(SndBank* bank, int size)` (0x28b8d8).
///
/// Lazily allocates the bank's 128-aligned buffer (`MEM_AllocNamed128`, name
/// at 0x4829a8) when `bank+8` is zero, remembering buffer (+0x10) and size
/// (+0x14); returns the buffer.
pub fn snd_bank_get_buffer(c: &mut Cpu) {
    guest(c, |c| {
        'b_28b90c: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            let t = c.g[V0][0] != 0u64;
            c.g[S1][0] = c.g[A1][0];
            if t {
                break 'b_28b90c;
            } else {
                c.cycles += 5;
                c.g[A0][0] = 0x480000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x29a8));
                call_ptr(c, 0x00252fa0, 0x0028b904)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
                break 'b_28b90c;
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        ret(c);
        return Ok(());
    })
}

/// `bool SSXAudioSystem_ResetMusicFadeIn(SSXAudioSystem* sys)` (0x28f108).
///
/// Clears the music fade-in word at `sys+0x628c`; returns whether the rider
/// rating tier is non-zero.
pub fn ssx_audio_system_reset_music_fade_in(c: &mut Cpu) {
    guest(c, |c| {
        'b_28f12c: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            call_ptr(c, 0x00289c18, 0x0028f11c)?;
            c.g[V1][0] = c.g[V0][0];
            let t = c.g[V1][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_28f12c;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'b_28f12c;
            }
        }
        c.cycles += 5;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x628c), c.g[A0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        ret(c);
        return Ok(());
    })
}

/// `void SndBankMan_LoadBankFromMemory(SndBankMan* self, int index, a2, a3)` (0x28be60):
/// `SndBank_LoadFromMemory(self->banks + index * 0x60, a2, a3)`, with the bank array
/// pointer at `self+0xacc`.
pub fn snd_bank_man_load_bank_from_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = 0x60;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.mult(0, 3, 5, 2);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xacc));
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Callee {
            addr: 0x0028b788,
            f: ee::s07::SndBank_LoadFromMemory,
        }
        .call(c, 0x0028be84)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    });
}

/// `SSXAudioSystem_OnEventSelectEnter(self)` (0x0028f5b8): clears the
/// "paused SFX for map screen" flag (`+0x6088`); when no audio pause is
/// active (`+0x5fb4 == 0`) and the game is in-game: pause the SFX unless
/// `+0x5818` is set (and remember it in `+0x6088`), play menu sound 9, and
/// unless the radio mode (`+0x608c`) is 2 start the loading-screen loop on
/// the music system (`self + 0x118`: volume 1.0, loop, vtable call, 0x7f).
pub fn ssx_audio_system_on_event_select_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x6088), 0);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5fb4));
        let paused = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x20);
        'out: {
            if paused {
                break 'out;
            }
            c.cycles += 4;
            Callee {
                addr: 0x0028_b1b0,
                f: ee::s07::SSXAudioSystem_IsInGame,
            }
            .call(c, 0x0028_f5e4)?;
            let not_in_game = c.g[V0][0] == 0;
            frame.restore(c, S0, 0x20);
            if not_in_game {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5818));
            let skip_pause = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if !skip_pause {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0029_ce28, 0x0028_f600)?;
                c.g[V0][0] = 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6088), c.g[V0][0]);
                c.g[A0][0] = c.g[S1][0];
            }
            c.cycles += 6;
            c.g[A1][0] = 9;
            call_ptr(c, 0x0029_4f78, 0x0028_f614)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x608c));
            c.g[V0][0] = 2;
            let radio2 = c.g[V1][0] == c.g[V0][0];
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x118));
            if !radio2 {
                c.cycles += 15;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 12);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002b_3d48, 0x0028_f634)?;
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0028_f768,
                    f: ee::s07::SSXAudioSystem_PlayLoadingScreenLoop,
                }
                .call(c, 0x0028_f63c)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5558));
                c.g[A1][0] = 0;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0028_f654)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x7f;
                call_ptr(c, 0x002b_3c28, 0x0028_f660)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_GetTrackMusicGroup(self)` (0x0028d8a0): classify the
/// current track of the location's music: asks five predicates
/// (`0x2a4318`, `0x2a4368`, `0x2a40e0`, `0x2a4158`, `0x2a41c8`; the first
/// that holds gives 0, 1, 0, 1, 2) and otherwise `0x2a4238` gives 3 or 4.
pub fn ssx_audio_system_get_track_music_group(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0028_d8b8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0028_d8c4)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x0028_d8cc)?;
        c.g[S1][0] = c.lw(c.g[V0][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_4318, 0x0028_d8d8)?;
        'done: {
            let hit = c.g[V0][0] != 0;
            c.g[A0][0] = 0;
            if hit {
                break 'done;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_4368, 0x0028_d8e8)?;
            let hit = c.g[V0][0] != 0;
            c.g[A0][0] = 1;
            if hit {
                break 'done;
            }
            // (predicate, group of the hit)
            for (target, ret, group) in [
                (0x002a_40e0u32, 0x0028_d8fcu32, 0u64),
                (0x002a_4158, 0x0028_d910, 1),
                (0x002a_41c8, 0x0028_d924, 2),
            ] {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, target, ret)?;
                let hit = c.g[V0][0] != 0;
                // The delay slot of each test loads the group of its hit.
                c.g[A0][0] = group;
                if hit {
                    break 'done;
                }
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x002a_4238, 0x0028_d938)?;
            c.g[A0][0] = 4;
            c.g[V1][0] = 3;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = c.g[V1][0];
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem::LoadLoadingScreenBank(this)` (0x28f700): loads sound bank
/// 13 (the loading-screen bank) through the bank manager `*(this + 0x118)`,
/// with the bank name at `gp + 0x534` (`SndBankMan_LoadBank(mgr, 13, name, 0)`).
pub fn ssx_audio_system_load_loading_screen_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x534));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A1][0] = 0xd;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x118));
        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
        Callee {
            addr: 0x0028_bc58,
            f: ee::s07::f_0028bc58,
        }
        .call(c, 0x0028_f720)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `SSXAudioSystem_SelectRandomSong(self)` (0x0028d488): pick a random song
/// index in `[0, tracks)`, store it and make the music player switch to it.
///
/// Same body as the unwired ports in `bankmanager.rs`; this one is installed.
pub fn ssx_audio_system_select_random_song_hand(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0x7fff;
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S2][0];
        let player = c.lw((this as u32).wrapping_add(0x118));
        c.g[S1][0] = c.lw((this as u32).wrapping_add(0x504));
        c.g[S3][0] = c.lw((this as u32).wrapping_add(0x62a4));
        c.g[V1][0] = player;
        c.g[V0][0] = c.lw((player as u32).wrapping_add(0));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
        call_ptr(c, 0x002a_df60, 0x0028_d4c0)?;
        c.g[V0][0] = c.g[V0][0] & 0x7fff;

        // Random fraction of the (scaled) track count.
        c.cycles += 14;
        c.mult(0, V0, V0, S1);
        c.g[V1][0] = c.lw((this as u32).wrapping_add(0x118));
        c.divu(0, V0, S0);
        c.g[V0][0] = c.lo[0];
        c.sw((this as u32).wrapping_add(0x508), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
        call_ptr(c, 0x002a_df60, 0x0028_d4ec)?;
        c.g[V1][0] = 0xa;
        c.g[V0][0] = c.g[V0][0] & 0x7fff;
        c.mult(0, V0, V0, V1);
        c.g[A0][0] = 0x7fff;

        // Skip up to ten songs to find a playable one.
        c.cycles += 5;
        c.divu(0, V0, S0);
        c.g[V1][0] = c.lo[0];
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[V0][0];
        if !empty {
            c.cycles += 2;
            c.g[A0][0] = this;
            loop {
                c.cycles += 8;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                Callee {
                    addr: 0x0028_d8a0,
                    f: ee::s07::SSXAudioSystem_GetTrackMusicGroup,
                }
                .call(c, 0x0028_d528)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = sx((this as u32).wrapping_add(0x118));
                c.g[A1][0] = 1;
                call_ptr(c, 0x002b_42b8, 0x0028_d538)?;
                let more = c.g[S0][0] != 0;
                c.g[A0][0] = this;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0028_d520);
                }
            }
        }

        // Restart the music player on the new selection.
        c.cycles += 3;
        c.g[V0][0] = c.lw((this as u32).wrapping_add(0x508));
        let same = c.g[S3][0] == c.g[V0][0];
        c.g[S0][0] = sx((this as u32).wrapping_add(0x118));
        if !same {
            c.cycles += 1;
        } else {
            c.cycles += 9;
            c.g[A0][0] = this;
            Callee {
                addr: 0x0028_d8a0,
                f: ee::s07::SSXAudioSystem_GetTrackMusicGroup,
            }
            .call(c, 0x0028_d554)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x002b_42b8, 0x0028_d564)?;
            c.g[V0][0] = c.lw((this as u32).wrapping_add(0x508));
            if c.g[S3][0] == c.g[V0][0] {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                call_ptr(c, 0x002b_42b8, 0x0028_d580)?;
                c.cycles += 1;
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_OnSongChanged` (0x0028f478): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ssx_audio_system_on_song_changed_resumable(c: &mut Cpu) {
    if c.pc != 0x0028f478 {
        return ee::s07::f_0028f478__generated(c);
    }
    guest(c, |c| {
        'b_28f514: {
            'b_28f510: {
                c.cycles += 15;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[RA][0] = sx(0x0028f490);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                call_ptr(c, 0x002b35a0, 0x0028f490)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1;
                c.g[RA][0] = sx(0x0028f4a0);
                c.g[A2][0] = 0u64;
                Callee {
                    addr: 0x00287930,
                    f: ee::s07::SSXAudioSystem_GetChannelGain,
                }
                .call(c, 0x0028f4a0)?;
                c.mtc1(0, 1);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_28f510;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_28f514;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                        let t = c.g[A0][0] == 0u64;
                        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        if t {
                            break 'b_28f510;
                        } else {
                            c.cycles += 2;
                            if c.g[S0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                break 'b_28f514;
                            } else {
                                c.cycles += 9;
                                c.g[A0][0] = 0x4a0000;
                                c.g[RA][0] = sx(0x0028f4e0);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3660));
                                call_ptr(c, 0x00317670, 0x0028f4e0)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0028f4ec);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039f9d8, 0x0028f4ec)?;
                                c.g[A3][0] = c.g[V0][0];
                                let t = c.g[A3][0] == 0u64;
                                c.g[A1][0] = 0x5;
                                if t {
                                    break 'b_28f510;
                                } else {
                                    c.cycles += 6;
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.g[A2][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x0028f510);
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0028f510)?;
                                    break 'b_28f510;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_28f514;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `SndBank_GetBufferSize` (0x28b928): returns `bank->buffer_size` (+0x14).
pub fn snd_bank_get_buffer_size(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
    c.pc = t;
}

/// `SSXAudioSystem_ResetMusicState` (0x28df08): `audio->current_song (+0x629c) = -1`.
pub fn ssx_audio_system_reset_music_state(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x629c), c.g[V0][0]);
    c.pc = t;
}

/// `SSXAudioSystem_SetCountdownValue(self, value)`: `self->countdown = value`
/// (word at +0x6250).
pub fn oc817_ssx_audio_system_set_countdown_value(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x6250), c.g[A1][0]);
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    pub(crate) use crate::game::fe::fepopupmisc::tests::oc782_install_fakes;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc790_install_fakes;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc796_install_fakes;
    pub(crate) use crate::game::sound::soundsys::tests::oc807_install_fakes;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;

    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot;
    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::bx::bxstringctor::tests::W;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_6 as be_library_3;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::put_str8;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::rng_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::zero;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SCRATCH;
    pub(crate) use crate::game::fe::debugfe::tests::w as w_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;

    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_3 as put_str;
    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_4 as put_str_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w32;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_7;

    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_4;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;

    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn snd_bank_link_matches() {
        check(ssx3_ee::s07::f_0028b6c8, snd_bank_link, 300, &|c, seed| {
            let mut r = Rng(0x1111_000a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            let other = DATA + 0x100;
            let prev = if seed % 2 == 0 { 0 } else { DATA + 0x200 };
            c.bus.write32(other + 0x18, prev);
            c.bus.write32(other + 0x1c, DATA + 0x300);
            c.bus
                .write32(other + 0x8, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(other + 0x10, r.next_u32());
            c.bus.write32(other + 0x14, r.next_u32());
            c.bus.write32(other + 0xc, r.next_u32());
            for i in 0..8 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
            c.bus.write32(DATA + 0x200 + 0x18, r.next_u32());
            c.bus.write32(DATA + 0x300 + 0x18, r.next_u32());
            c.bus.write32(DATA + 0x300 + 0x1c, r.next_u32());
        });
    }

    #[test]
    fn snd_bank_unlink_matches_2() {
        check(
            ssx3_ee::s07::f_0028b730,
            snd_bank_unlink_2,
            300,
            &|c, seed| {
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
            },
        );
    }

    pub(crate) const SEEDS: u32 = 120;

    /// A per-function RNG (the address keeps the functions' seeds apart).
    pub(crate) fn rng(tag: u32, seed: u32) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((tag as u64) << 20) ^ seed as u64)
    }

    /// A 64-bit value whose low half is small (indices, ids) and whose upper
    /// half is junk on odd seeds, so 64-bit compares are exercised.
    pub(crate) fn mixed(r: &mut Rng, seed: u32, low: u32) -> u64 {
        if seed % 2 == 0 {
            low as u64
        } else {
            ((r.next_u32() as u64) << 32) | low as u64
        }
    }

    /// An id inside or just outside `[base, base + len)`, with junk in the
    /// upper half on odd seeds.
    pub(crate) fn track_id(c: &mut Cpu, r: &mut Rng, seed: u32, base: u32, len: u32) {
        let id = match seed % 3 {
            0 => base.wrapping_add(seed % len),
            1 => base.wrapping_sub(1 + seed % 4),
            _ => r.next_u32() % 0x40,
        };
        c.g[A1][0] = mixed(r, seed, id);
    }

    #[test]
    fn ssx_audio_system_is_track_id_group0_matches() {
        check(
            ee::s08::f_002a40e0__generated,
            ssx_audio_system_is_track_id_group0,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002a_40e0, seed);
                scramble(c, &mut r);
                track_id(c, &mut r, seed, 0, 5);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_id_group4_matches() {
        check(
            ee::s08::f_002a4030__generated,
            ssx_audio_system_is_track_id_group4,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002a_4030, seed);
                scramble(c, &mut r);
                track_id(c, &mut r, seed, 0xe, 3);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_id_group1_matches() {
        check(
            ee::s08::f_002a4158__generated,
            ssx_audio_system_is_track_id_group1,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002a_4158, seed);
                scramble(c, &mut r);
                track_id(c, &mut r, seed, 5, 3);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_id_group2_matches() {
        check(
            ee::s08::f_002a41c8__generated,
            ssx_audio_system_is_track_id_group2,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002a_41c8, seed);
                scramble(c, &mut r);
                track_id(c, &mut r, seed, 8, 3);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_id_group3_matches() {
        check(
            ee::s08::f_002a4238__generated,
            ssx_audio_system_is_track_id_group3,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002a_4238, seed);
                scramble(c, &mut r);
                track_id(c, &mut r, seed, 0xb, 3);
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_music_state_matches() {
        check(
            ee::s07::f_0028def0__generated,
            ssx_audio_system_set_music_state,
            SEEDS,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn music_player_clear_loops_active_matches() {
        check(
            ee::s08::f_002b4708__generated,
            music_player_clear_loops_active,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002b_4708, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0x408, 0);
                } else {
                    c.bus.write32(DATA + 0x408, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2000 + 0x4c, r.next_u32());
                }
            },
        );
    }

    // ---------------------------------------------------------------- sound

    /// A sound bank manager with four zeroed 0x60-byte banks. The banks are
    /// arranged so `SndBank_Unload`/`_UnloadMemory`/`_Release` take their
    /// short paths and the frees see a null payload.
    pub(crate) fn snd_bank_man_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let manager = DATA + 0x4000;
        let banks = DATA + 0x5000;
        c.bus.write32(manager + 0xacc, banks);
        for i in 0..4u32 {
            let bank = banks + i * 0x60;
            c.bus.write32(bank, (seed % 2) * 2);
            c.bus.write32(bank + 0x4, u32::MAX);
            c.bus.write32(bank + 0x8, (seed % 2) * 3);
            c.bus.write32(bank + 0x10, 0);
            c.bus.write32(bank + 0x14, r.next_u32());
            c.bus.write32(bank + 0x18, 0);
        }
        c.g[A0][0] = sx(manager);
        c.g[A1][0] = (seed % 4) as u64;
    }

    #[test]
    fn snd_bank_man_unload_bank_matches() {
        check_lohi(
            ssx3_ee::s07::f_0028bce8__generated,
            snd_bank_man_unload_bank,
            100,
            &|c, seed| snd_bank_man_setup(c, seed, 0x3b),
        );
    }

    #[test]
    fn snd_bank_man_free_bank_memory_matches() {
        check_lohi(
            ssx3_ee::s07::f_0028bde0__generated,
            snd_bank_man_free_bank_memory,
            100,
            &|c, seed| snd_bank_man_setup(c, seed, 0x3c),
        );
    }

    #[test]
    fn snd_bank_man_unload_memory_bank_matches() {
        check_lohi(
            ssx3_ee::s07::f_0028be90__generated,
            snd_bank_man_unload_memory_bank,
            100,
            &|c, seed| snd_bank_man_setup(c, seed, 0x3d),
        );
    }

    #[test]
    fn snd_bank_man_release_bank_matches() {
        check_lohi(
            ssx3_ee::s07::f_0028beb8__generated,
            snd_bank_man_release_bank,
            100,
            &|c, seed| snd_bank_man_setup(c, seed, 0x3e),
        );
    }

    #[test]
    fn snd_bank_man_get_bank_buffer_size_matches() {
        check_lohi(
            ssx3_ee::s07::f_0028bf10__generated,
            snd_bank_man_get_bank_buffer_size,
            100,
            &|c, seed| snd_bank_man_setup(c, seed, 0x3f),
        );
    }

    #[test]
    fn ssx_audio_system_is_game_music_enabled_matches() {
        check(
            ssx3_ee::s07::f_0028d960__generated,
            ssx_audio_system_is_game_music_enabled,
            300,
            &|c, seed| {
                let mut r = rng_2(seed, 0x40);
                scramble(c, &mut r);
                let state = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 3,
                    3 => 2,
                    4 => r.next_u32(),
                    _ => 4,
                };
                c.bus.write32(DATA + 0x608c, state);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn snd_bank_man_get_bank_buffer_matches() {
        check(
            ee::s07::f_0028bee0__generated,
            snd_bank_man_get_bank_buffer,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28bee0, 3);
                let banks = SCRATCH + 0x1000;
                let index = seed % 4;
                let bank = banks + 0x60 * index;
                // A loaded bank (`+8` non-zero) just hands back its buffer at
                // `+0x10`; an empty one allocates through the sound heap, whose
                // arena must be set up first (see `MEM_AllocNamed128`'s test).
                let loaded = seed % 2 == 0;
                if !loaded {
                    heap_init(c);
                }
                c.g[A0][0] = SCRATCH as u64;
                c.g[A1][0] = index as u64;
                // `a2` becomes the allocation size and `a3` its flags (both are
                // forwarded to `MEM_AllocNamed128`), so keep the size modest.
                c.g[A2][0] = (0x40 + seed % 0x100) as u64;
                c.g[A3][0] = (r.next_u32() & 1) as u64;
                c.bus.write32(SCRATCH + 0xacc, banks);
                c.bus.write32(bank + 8, loaded as u32);
                c.bus.write32(bank + 0x10, r.next_u32());
            },
        );
    }

    /// Set up the sound heap the allocator inside `SndBank_GetBuffer` uses.
    pub(crate) fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = crate::difftest::GAME_GP as u64;
        load(
            c,
            0x0031_ff60,
            ssx3_ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }

    /// Call a setup helper and check that it returned.
    pub(crate) fn load(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
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

    #[test]
    fn snd_bank_man_set_bank_memory_matches() {
        check(
            ssx3_ee::s07::f_0028bda8__generated,
            snd_bank_man_set_bank_memory,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28bda8, 0);
                let bank_man = DATA + 0x100;
                c.bus.write32(bank_man + 0xacc, DATA + 0x2000);
                fill(c, &mut r, DATA + 0x2000, 0x100);
                c.g[A0][0] = sx(bank_man);
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                // Zero length and pointer: SndBank_SetMemory takes its early exit.
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
            },
        );
    }

    #[test]
    fn audio_stop_fe_music_matches() {
        check(
            ssx3_ee::s07::f_0028f520__generated,
            audio_stop_fe_music,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28f520, 1);
                let sys = DATA + 0x100;
                fill(c, &mut r, sys, 0x100);
                // +0x608c: 0/1/2 "music enabled", 3 also enabled, else disabled.
                c.bus
                    .write32(sys + 0x608c, [0u32, 2, 3, 4][seed as usize % 4]);
                // No current song instance (+0x118 + 0x408): the player only
                // touches its own fields.
                c.bus.write32(sys + 0x520, 0);
                c.bus.write32(sys + 0x118, 0);
                c.g[A0][0] = sx(sys);
            },
        );
    }

    // ---------------------------------------------------------------- sound

    #[test]
    fn ssx_audio_system_end_script_music_matches() {
        check(
            ee::s07::f_0028e888__generated,
            ssx_audio_system_end_script_music,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x28e8_88, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x6300);
                c.g[A0][0] = sx(self_);
                c.bus
                    .write32(self_ + 0x6254, if seed % 2 == 0 { 0 } else { 1 });
                // HandleMusicEvent exits early when +0x627c is clear.
                c.bus.write32(self_ + 0x627c, 0);
            },
        );
    }

    #[test]
    fn ssx_audio_system_unload_loading_screen_bank_matches() {
        check(
            ee::s07::f_0028f730__generated,
            ssx_audio_system_unload_loading_screen_bank,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x28f7_30, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x6300);
                c.g[A0][0] = sx(self_);
                // Negative handle: StopLoadingScreenLoop returns at once.
                c.bus.write32(self_ + 0x62a8, 0xffff_ffff);
                c.bus.write32(self_ + 0x118, DATA + 0x600);
                c.bus.write32(DATA + 0x600, DATA + 0x700);
                zero(c, DATA + 0x700, 0x100);
            },
        );
    }

    /// The BE library singleton, so `BELibrary_GetInstance` never builds it.
    pub(crate) fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
    }

    /// [`be_library`], plus the current phase interface (`gp-0x1efc` selects
    /// one of the three) whose slot-2 method is the fake callee.
    pub(crate) fn be_library_interface(c: &mut Cpu, r: &mut crate::difftest::Rng) {
        be_library(c);
        let index = (r.next_u32() % 8) as u32;
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), index);
        let lib = ptr(r);
        c.bus.write32(0x0053_08a8 + 4 * index, lib);
        let vt = vtable(c, r, &[8]);
        c.bus.write32(lib + 4, vt);
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
    fn snd_bank_is_loaded_with_name_matches() {
        check(
            ee::s07::f_0028b830__generated,
            snd_bank_is_loaded_with_name,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28b8_30, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x300);
                c.bus.write32(
                    this,
                    match seed % 4 {
                        0 => 0,
                        1 => 2,
                        _ => 1,
                    },
                );
                let name = if seed % 2 == 0 {
                    &b"bank_a\0"[..]
                } else {
                    &b"bank_b\0"[..]
                };
                put_str8(c, DATA + 0x300, name);
                let local = if seed % 4 == 1 {
                    &b"bank_a\0"[..]
                } else {
                    &b"other\0"[..]
                };
                put_str8(c, this + 0x20, local);
                c.bus.write32(this + 0x24, r.next_u32());
            },
        );
    }

    /// The BE library singleton: a zeroed dummy so `cBE_getBE` does not build
    /// the real one from disc.
    pub(crate) fn be_library_2(c: &mut Cpu) {
        c.bus.write32(gp_at(-0x1f04), DATA + 0x3f000);
    }

    #[test]
    fn ssx_audio_system_is_race_type6_to11_matches() {
        race_type_test(
            ee::s08::f_002a42c8__generated,
            ssx_audio_system_is_race_type6_to11,
            0x2a42_c8,
        );
    }

    /// The three race-type queries: the BE library dummy, an empty BE table
    /// (the interface call lands in the fake hook) and every race type.
    pub(crate) fn race_type_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, SEEDS_3, &move |c, seed| {
            let _ = machine(c, seed, salt, 0);
            be_library_2(c);
            clear(c, 0x0053_08a8, 0x20);
            c.bus.write8(0x0053_5c12, (seed % 20) as u8);
        });
    }

    #[test]
    fn snd_bank_unload_memory_matches() {
        check(
            ee::s07::f_0028b7e0__generated,
            snd_bank_unload_memory,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28b7_e0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                let bank = match seed % 3 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    _ => seed % 4,
                };
                c.bus.write32(this + 4, bank);
                c.bus.write32(this + 0x10, r.next_u32());
                c.bus.write32(this + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn snd_bank_load_from_memory_matches() {
        check(
            ssx3_ee::s07::f_0028b788__generated,
            snd_bank_load_from_memory,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x28b788, 2);
                let bank = DATA + 0x400;
                c.bus.write32(bank, seed);
                c.bus.write32(bank + 4, 0xffff_ffff);
                c.g[A0][0] = bank as u64;
                c.g[A1][0] = (DATA + 0x500) as u64;
                c.g[A2][0] = (seed * 0x10 + 4) as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // Sound banks.

    #[test]
    fn snd_bank_man_init_bank_request_matches() {
        covered(
            ee::s07::f_0028bf38__generated,
            snd_bank_man_init_bank_request,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 22, 3);
                c.g[A0][0] = W as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = if seed % 2 == 0 { 1 } else { 0 };
                c.g[A3][0] = (AUX + 0x400) as u64;
                c.g[T0][0] = (r.next_u32() % 4) as u64;
                c.g[T1][0] = (r.next_u32() % 4) as u64;
                c.g[T2][0] = (r.next_u32() % 4) as u64;
                put(c, W + 0xacc, AUX + 0x800);
                // `SndBankMan_InitLoadRequest` walks
                // gp+0x524 -> +0x1d8 -> [0] -> [0] and calls the function
                // pointer at +0x3c (an unregistered address, so the test
                // machine's fake records the call).
                gpf(c, 0x524, AUX + 0x900);
                put(c, AUX + 0x900 + 0x1d8, AUX + 0x920);
                put(c, AUX + 0x920, AUX + 0x940);
                put(c, AUX + 0x940, AUX + 0x960);
                c.bus.write16(AUX + 0x960 + 0x38, 0);
                put(c, AUX + 0x960 + 0x3c, FAKE);
                c.bus.write8(AUX + 0x400, 0); // the strcpy source is empty
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_selected_peak_matches() {
        check(
            ee::s07::f_0028f558__generated,
            ssx_audio_system_set_selected_peak,
            64,
            &|c, seed| {
                machine(c, seed, 0x0028_f558, 3);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                w(c, DATA + 0x6284, (seed % 8) as u32);
                // `RestartPeakMusic` reaches the music player's round-robin slot
                // picker, which spins until the occupancy mask names a free slot:
                // give it a two-slot bank whose second slot is free.
                w(c, DATA + 0x4fc, 2); // slot count
                w(c, DATA + 0x508, 0); // current
                c.bus.write64(DATA + 0x510, 2); // slot mask (slot 1 free)
            },
        );
    }

    #[test]
    fn snd_bank_release_matches() {
        check(
            ssx3_ee::s07::f_0028b878__generated,
            snd_bank_release,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x28b878, 0);
                w_2(c, OBJ + 0x8, (seed % 3) as u32);
                w_2(c, OBJ + 0xc, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    // ------------------------------------------------------------- audio/triggers

    #[test]
    fn ssx_audio_system_on_replay_end_matches() {
        check(
            ssx3_ee::s07::f_0028f2c0__generated,
            ssx_audio_system_on_replay_end,
            90,
            &|c, seed| {
                let mut r = setup(c, seed, 0x28f2c0);
                c.g[A0][0] = DATA as u64;
                let mode = seed % 3;
                c.bus.write32(DATA + 0x6294, u32::from(mode == 1));
                c.bus.write32(DATA + 0x6298, u32::from(mode == 2));
                if mode == 1 {
                    // A song player with no song: the timer walks are empty.
                    c.bus.write32(DATA + 0x118, DATA + 0x500);
                    c.bus.write32(DATA + 0x500 + 0x408, 0);
                }
                if mode == 2 {
                    let vt = vtable(c, &mut r, &[0x20]);
                    c.bus.write32(DATA + 0x5558, vt);
                }
            },
        );
    }

    #[test]
    fn ssx_audio_system_restart_peak_music_matches() {
        check(
            ee::s07::f_0028ef90__generated,
            ssx_audio_system_restart_peak_music,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x0028_ef90, 4);
                c.g[A0][0] = OBJ_2 as u64;
                w(c, OBJ_2 + 0x118, DATA + 0x3000);
                w(c, DATA + 0x3000, DATA + 0x3200);
                w(c, DATA + 0x3200 + 0x1d8, 0);
                // `RestartPeakMusic` reaches the music player's round-robin
                // slot picker (`MusicPlayer_SelectNextSong`, 0x2b42b8), which
                // spins until the occupancy mask names a slot. The bank
                // manager is inline at `self + 0x118`: slot count at +0x3e4,
                // current slot at +0x3f0, 64-bit mask at +0x3f8. A two-slot
                // bank with slot 1 occupied and current 0 exits after one
                // step (same fixture as oc90's SetSelectedPeak test).
                w(c, OBJ_2 + 0x4fc, 2); // slot count
                w(c, OBJ_2 + 0x508, 0); // current
                c.bus.write64(OBJ_2 + 0x510, 2); // occupancy mask (slot 1)
            },
        );
    }

    #[test]
    fn snd_bank_free_memory_matches() {
        check(
            ee::s07::f_0028b650__generated,
            snd_bank_free_memory,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x28_b650, 2);
                heap_setup(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                w(c, DATA + 0x2000 + 0x8, [0u32, 1, 2][(seed % 3) as usize]);
                w(c, DATA + 0x2000 + 0xc, 0x11);
                w(c, DATA + 0x2000 + 0x10, DATA + 0x2600);
                w(c, DATA + 0x2000 + 0x14, 0x22);
                w(c, GAME_GP + 0x524, DATA + 0x3000);
                w(c, DATA + 0x3000 + 0x1d8, DATA + 0x3200);
                w(c, DATA + 0x3200, DATA + 0x3400);
                slot(c, DATA + 0x3400, 0x28, 0, FAKE);
                c.pc = 0x0028_b650;
            },
        );
    }

    #[test]
    fn music_player_set_playlist_mask_matches() {
        check(
            ee::s08::f_002b4070__generated,
            music_player_set_playlist_mask,
            80,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x2b40, 3);
                let m = DATA;
                fill(c, &mut r, m + 0x3e0, 3);
                fill(c, &mut r, m + 0x920, 8);
                w(c, m + 0x3ec, (seed % 5) as u32);
                c.g[A0][0] = sx(m);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn snd_bank_man_init_load_request_matches() {
        check(
            ee::s07::f_0028b930__generated,
            snd_bank_man_init_load_request,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x28b930, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                c.g[A3][0] = sx(DATA + 0x500);
                put_str(c, DATA + 0x500, b"bank");
                let m = DATA + 0x1000;
                c.bus.write32(GAME_GP + 0x524, m);
                c.bus.write32(m + 0x1d8, DATA + 0x1010);
                c.bus.write32(DATA + 0x1010, DATA + 0x1020);
                c.bus.write32(DATA + 0x1020, DATA + 0x1030);
                c.bus.write16(DATA + 0x1030 + 0x38, 0);
                c.bus.write32(DATA + 0x1030 + 0x3c, FAKE);
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_event_select_exit_matches() {
        check(
            ee::s07::f_0028f678__generated,
            ssx_audio_system_on_event_select_exit,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28f678, 3);
                be_library_3(c, &mut r);
                c.bus.write32(LOG_RESULTS, DATA + 0x2100);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(DATA + 0x6088, (seed % 3 == 0) as u32);
                // `+0x62a8` < 0 takes the short path; otherwise the loading-screen
                // loop runs (and its own guard is the same field).
                c.bus
                    .write32(DATA + 0x62a8, if seed % 4 == 0 { 0xffff_ffff } else { 0 });
                c.bus.write32(DATA + 0x118, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, 0);
            },
        );
    }

    #[test]
    fn ssx_audio_system_update_music_tier_matches() {
        check(
            ee::s07::f_0028d5a0__generated,
            ssx_audio_system_update_music_tier,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x28d5, 0);
                env(c);
                let this = DATA + 0x400;
                w32(c, this + 0x6284, if seed % 4 == 0 { 0x17 } else { 0 });
                w32(c, gpat(0x410), 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn snd_bank_man_load_bank_async_matches() {
        check(
            ee::s07::f_0028bd10__generated,
            snd_bank_man_load_bank_async,
            SEEDS_6,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(c, this + 0xacc, DATA + 0x1000);
                for i in 0..8 {
                    w(c, DATA + 0x1000 + 4 * i, 0);
                }
                c.g[S3][0] = 0;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x2100) as u64;
                c.g[A3][0] = seed as u64;
                put_str_2(c, DATA + 0x2000, b"music");
                put_str_2(c, DATA + 0x2100, b"bank");
            }),
        );
    }

    #[test]
    fn ssx_audio_system_confirm_music_trigger_matches() {
        check(
            ee::s07::f_0028d740__generated,
            ssx_audio_system_confirm_music_trigger,
            SEEDS_6,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                let obj = DATA + 0x800;
                let vt = vtable(c, r, &[0x38]);
                w(c, obj + 0x6c0, vt);
                match seed % 3 {
                    0 => {
                        w(c, this + 0x6240, 7);
                        w(c, this + 0x62a0, 7);
                    }
                    1 => {
                        w(c, this + 0x6240, 7);
                        w(c, this + 0x62a0, 0xffff_ffff);
                    }
                    _ => {
                        w(c, this + 0x6240, 7);
                        w(c, this + 0x62a0, 9);
                        w(c, this + 0x623c, (seed & 2) as u32);
                    }
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 7;
                c.g[A2][0] = obj as u64;
            }),
        );
    }

    #[test]
    fn ssx_audio_system_on_music_post_event_matches() {
        check(
            ssx3_ee::s07::f_0028f328__generated,
            ssx_audio_system_on_music_post_event,
            16,
            &|c, seed| {
                let _ = base_4(c, seed, 0x0028_f328);
                let audio = DATA + 0x100;
                let player = audio + 0x118;
                let song = DATA + 0x3000;
                let queue = DATA + 0x4000;
                clear_2(c, audio, 0x6000);
                clear_2(c, song, 0x80);
                clear_2(c, queue, 0x40);
                // A live song object whose tag is "none" (-1).
                w(c, player + 0x418, 1);
                w(c, player + 0x408, song);
                w(c, song + 0x58, 0xffff_ffff);
                w(c, audio + 0x5558, queue);
                w(c, queue + 0x1c, 0);
                let event = match seed % 5 {
                    0 => 10,
                    1 => 11,
                    2 => 18,
                    3 => 9,
                    _ => (seed % 32) as u32,
                };
                w(c, GAME_GP.wrapping_add(0x530), (seed % 3) as u32);
                c.g[A0][0] = sx(audio);
                c.g[A1][0] = event as u64;
            },
        );
    }

    pub(crate) const SYS: u32 = DATA + 0x18000;

    #[test]
    fn ssx_audio_system_play_char_select_music_matches() {
        check(
            ee::s07::f_0028fc58__generated,
            ssx_audio_system_play_char_select_music,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x28fc58, 3);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x118, 0x1000);
                w(c, SYS + 0x5558, OBJ2);
                w(c, SYS + 0x6290, (seed % 4) as u32);
                w(c, OBJ2 + 0x10, (seed % 5) as u32);
                let vt0 = vtable(c, &mut r, &[0x20, 0x24]);
                w(c, OBJ2 + 0x14, vt0);
                let vt1 = vtable(c, &mut r, &[0x20, 0x24]);
                w(c, OBJ2 + 0x24, vt1);
            },
        );
    }

    // ---------------------------------------------------------- audio and race

    #[test]
    fn ssx_audio_system_play_peak_transition_music_matches() {
        check(
            ee::s07::f_0028cd48__generated,
            ssx_audio_system_play_peak_transition_music,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x0028_cd48, 3);
                c.g[A0][0] = OBJ_3 as u64;
                let vt = vtable(c, &mut r, &[0x10, 0x20]);
                w(c, OBJ_3 + 0x5558, vt);
                w(c, OBJ_3 + 0x118, OBJ2);
                w(c, 0x004a_3628, 0);
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_replay_begin_matches() {
        check(
            ee::s07::f_0028f200__generated,
            ssx_audio_system_on_replay_begin,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28f2, 6);
                elf_image(c);
                let this = DATA;
                c.bus.write32(this + 0x530, [0u32, 1][(seed % 2) as usize]);
                c.bus.write32(this + 0x118, DATA + 0x2000);
                c.bus.write32(this + 0x62ac, r.next_u32());
                c.bus.write32(this + 0x508, r.next_u32());
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn snd_bank_set_memory_matches() {
        check(
            ee::s07::f_0028b588__generated,
            snd_bank_set_memory,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28b5, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = ((seed % 3) as u64) * 0x100;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    ((seed % 4) as u64) * 0x40
                };
                c.g[A3][0] = (DATA + 0x2000) as u64;
                let obj = DATA + 0x2400;
                c.bus.write32(GAME_GP + 0x524, obj);
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(obj + 0x1d8, DATA + 0x2800);
                c.bus.write32(DATA + 0x2800, vt);
            },
        );
    }

    pub(crate) use crate::game::fe::ovstatehud::tests::gpa;

    // ------------------------------------------ SSXAudioSystem_UpdatePeakMusic

    pub(crate) const AUDIO: u32 = DATA;

    pub(crate) const AUDIO_WORLD: u32 = DATA + 0xc000;

    pub(crate) const AUDIO_MIXER: u32 = DATA + 0xc100;

    pub(crate) const AUDIO_IFACE: u32 = DATA + 0xc200;

    /// `SSXAudioSystem_TrackToCategory`'s jump table (.rodata 0x483010):
    /// the 22 tracks' categories (1, 2 or 4).
    pub(crate) const TRACK_CATEGORY_JUMPS: [u32; 22] = {
        const A: u32 = 0x002a_1e44;
        const B: u32 = 0x002a_1e4c;
        const C: u32 = 0x002a_1e54;
        [
            A, A, B, B, C, A, B, C, A, B, C, A, B, C, A, B, C, A, A, B, B, C,
        ]
    };

    /// The audio system at `AUDIO` with every peak state, music on/off and
    /// pending tracks (none, plain, random-song); the world (`gp-0x848`)
    /// says a replay is playing so `PlayMusic` returns at once; the race's
    /// track (via the BE interface, a fake) at 0x43d950; a mixer of fakes;
    /// the music player fixture `SelectRandomSong` needs (a two-slot bank
    /// with slot 1 occupied, as in `RestartPeakMusic`'s test).
    pub(crate) fn update_peak_music_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0028_e100, 4);
        let a = AUDIO;
        c.bus.write32(gpa(-0x1f04), DATA + 0xc300);
        c.bus.write32(gpa(-0x848), AUDIO_WORLD);
        c.bus.write32(AUDIO_WORLD + 0x78, DATA + 0xc300);
        c.bus.write32(AUDIO_WORLD + 0x84, AUDIO_WORLD + 0x40);
        c.bus.write32(AUDIO_WORLD + 0x40 + 0x28, AUDIO_WORLD + 0x80);
        c.bus.write32(AUDIO_WORLD + 0x80, 1);
        // BE_GetInterface(lib, 0): gp-0x1efc indexes the table at 0x5308a8.
        c.bus.write32(gpa(-0x1efc), 0);
        c.bus.write32(0x0053_08a8, AUDIO_IFACE);
        c.bus.write32(AUDIO_IFACE + 4, AUDIO_IFACE + 0x10);
        c.bus.write16(AUDIO_IFACE + 0x10 + 8, 0);
        c.bus.write32(AUDIO_IFACE + 0x10 + 0xc, FAKE + 0x100);
        c.bus.write32(0x0053_5c08, 0);
        c.bus.write32(0x0043_d950, r.next_u32() % 0x18);
        crate::difftest::put_words(c, 0x0048_3010, &TRACK_CATEGORY_JUMPS);
        // The mixer (methods +0x10, +0x20).
        c.bus.write32(a + 0x5558, AUDIO_MIXER);
        for (i, off) in [0x10u32, 0x20].into_iter().enumerate() {
            c.bus
                .write16(AUDIO_MIXER + off, ((r.next_u32() % 16) * 4) as u16);
            c.bus.write32(AUDIO_MIXER + off + 4, FAKE + 0x10 * i as u32);
        }
        // The music player (inline at +0x118).
        c.bus.write32(a + 0x118, DATA + 0xa000);
        c.bus.write32(DATA + 0xa000, DATA + 0xa100);
        c.bus.write32(DATA + 0xa100 + 0x1d8, 0);
        c.bus.write32(a + 0x504, 2);
        c.bus.write32(a + 0x4fc, 2);
        c.bus.write32(a + 0x508, 0);
        c.bus.write64(a + 0x510, 2);
        c.bus.write32(a + 0x62a4, r.next_u32() % 2);
        // The peak music request.
        c.bus.write32(a + 0x6274, if seed % 7 == 0 { 0 } else { 1 });
        c.bus.write32(
            a + 0x6278,
            [0u32, 2, 3, 4, 1, 0, 2][(seed / 7) as usize % 7],
        );
        c.bus
            .write32(a + 0x608c, [3u32, 0, 2, 3, 4][(seed / 2) as usize % 5]);
        let track = match r.next_u32() % 4 {
            0 => 0x17,
            1 => 0xe + r.next_u32() % 3,
            _ => r.next_u32() % 0x18,
        };
        c.bus.write32(a + 0x6284, track);
        c.bus.write32(a + 0x6294, 0);
        c.g[A0][0] = a as u64;
        c.pc = 0x0028_e100;
    }

    #[test]
    fn ssx_audio_system_update_peak_music_matches() {
        covered(
            ee::s07::f_0028e100__generated,
            ssx_audio_system_update_peak_music,
            300,
            &update_peak_music_setup,
        );
    }

    /// A float in [0, 2 * half).
    pub(crate) fn pos(r: &mut Rng, half: f32) -> u32 {
        (f32::from_bits(r.f32(half)) + half).to_bits()
    }

    // --- SSXAudioSystem_UpdateMusicIntensity -------------------------------

    pub(crate) const SYS_2: u32 = DATA;

    pub(crate) const RIDER_M: u32 = DATA + 0x8000;

    pub(crate) const RIDER_STATE: u32 = DATA + 0x9000;

    pub(crate) const AIR: u32 = DATA + 0x9800;

    pub(crate) const WORLD_M: u32 = DATA + 0xa000;

    pub(crate) const GAME_M: u32 = DATA + 0xa100;

    pub(crate) const REPLAY: u32 = DATA + 0xa200;

    pub(crate) const RACE: u32 = DATA + 0xa300;

    pub(crate) const RACE_REPLAY: u32 = DATA + 0xa400;

    pub(crate) const SONG: u32 = DATA + 0xb000;

    pub(crate) const SONG_VT: u32 = DATA + 0xb100;

    pub(crate) const AMB_OWNER: u32 = DATA + 0xc000;

    pub(crate) const AMB_VT: u32 = DATA + 0xc100;

    pub(crate) const MON_VT: u32 = DATA + 0xc200;

    pub(crate) const BELIB: u32 = DATA + 0xd000;

    pub(crate) const BE_STATE: u32 = DATA + 0xd100;

    pub(crate) const BE_VT: u32 = DATA + 0xd200;

    /// The audio system, the local rider in or out of a jump (FindRider
    /// through the race manager, or the replay rider), the air predictor's
    /// heights, the music monitor (song present or not, ducking, playing
    /// state), the track id the race-music checks read (through the
    /// BELibrary state vtable, a fake), and the monitor/ambience vtables
    /// (fakes).
    pub(crate) fn music_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x28c8c8, 4);
        let gp = GAME_GP;
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        // World -> game module -> race manager -> riders.
        w(c, gp - 0x848, WORLD_M);
        w(c, WORLD_M + 0x84, GAME_M);
        let replay = seed % 7 == 0;
        w(c, GAME_M + 0x28, if replay { REPLAY } else { 0 });
        w(c, REPLAY, 1 + seed % 9);
        w(c, GAME_M + 0xc, RACE);
        w(c, RACE + 0x7c, 1);
        w(c, RACE + 0x28, RIDER_M);
        w(c, RACE + 0x40, RACE_REPLAY);
        w(c, RACE_REPLAY + 0x18, RIDER_M);
        w(c, RIDER_M + 0x874, 0);
        // Rider: playing state (1 = racing) and air predictor.
        w(c, RIDER_M + 0x77c, RIDER_STATE);
        w(c, RIDER_STATE + 0xde0, if seed % 11 == 0 { 0 } else { 1 });
        w(c, RIDER_M + 0x788, AIR);
        w(c, AIR + 0xac, [1, 3, 0, 2, 1][(seed / 2 % 5) as usize]);
        let speed = pos(&mut r, 3.0);
        w(c, AIR + 0x98, speed);
        w(c, AIR + 0x9c, pos(&mut r, 8.0));
        w(c, AIR + 0xa0, pos(&mut r, 4.0));
        // Audio system state.
        w(c, SYS_2 + 0x6250, (r.next_u32() % 3).wrapping_sub(1));
        w(c, SYS_2 + 0x6244, pos(&mut r, 3.0));
        w(c, SYS_2 + 0x6248, pos(&mut r, 4.0));
        w(c, SYS_2 + 0x624c, pos(&mut r, 6.0));
        w(c, SYS_2 + 0x62d8, pos(&mut r, 0.5)); // music channel gain
        w(c, SYS_2 + 0x6318, pos(&mut r, 0.5)); // ambience channel gain
        w(c, SYS_2 + 0x5830, r.next_u32());
        w(c, SYS_2 + 0x5834, 1); // ambience loop already running: short path
        w(c, SYS_2 + 0x623c, (seed / 3 % 4 == 0) as u32); // music trigger
        w(c, SYS_2 + 0x608c, seed / 5 % 5); // game-music state
        w(c, gp + 0x52c, (seed % 3 != 0) as u32);
        w(c, gp + 0x530, r.next_u32() % 2);
        w(c, gp - 0x45d4, 3.999_498_6_f32.to_bits());
        w(c, gp - 0x45d0, 0.842_519_7_f32.to_bits());
        w(c, gp - 0x45cc, (1.0f32 / 127.0).to_bits());
        // Music monitor: song (or none), ducking flag.
        let mon = SYS_2 + 0x118;
        let song = if seed % 13 == 0 { 0 } else { SONG };
        w(c, mon, AMB_OWNER);
        w(c, mon + 0x408, song);
        w(c, mon + 0x418, (seed % 4 == 1) as u32);
        w(c, SONG, SONG_VT);
        w(c, SONG_VT + 0x94, (seed % 8 == 1) as u32);
        w(c, SONG + 0x38, [2, 2, 0][(seed / 4 % 3) as usize]);
        w(c, SONG + 0x4c, (seed / 6 % 2) as u32);
        w(c, SONG + 0x54, 0);
        // Vtables of the stop-ambience owner and the monitor: fakes.
        w(c, AMB_OWNER + 0xab0, AMB_VT);
        c.bus.write16(AMB_VT + 0x18, 0x10);
        w(c, AMB_VT + 0x1c, FAKE);
        w(c, SYS_2 + 0x5558, MON_VT);
        c.bus.write16(MON_VT + 0x18, 0x20);
        w(c, MON_VT + 0x1c, FAKE + 0x10);
        // BELibrary: current state's track (race-music checks).
        w(c, gp - 0x1f04, BELIB);
        w(c, gp - 0x1efc, 0);
        w(c, 0x0053_08a8, BE_STATE);
        w(c, BE_STATE + 4, BE_VT);
        c.bus.write16(BE_VT + 8, 0);
        w(c, BE_VT + 0xc, FAKE + 0x20);
        w(c, 0x0053_5c08, 0);
        w(c, 0x0043_d950, [0, 9, 12, 3][(seed / 7 % 4) as usize]);
        c.bus
            .write8(0x0053_5c10, [0, 2, 3, 0, 0][(seed / 9 % 5) as usize]);
        c.g[A0][0] = SYS_2 as u64;
        c.pc = 0x0028_c8c8;
    }

    #[test]
    fn update_music_intensity_matches() {
        covered(
            ee::s07::f_0028c8c8__generated,
            ssx_audio_system_update_music_intensity,
            300,
            &|c, seed| music_fixture(c, seed),
        );
    }

    /// `SSXAudioSystem_IsTrackGroup4`: the real BE library lookups with the
    /// interface's vtable slot on a fake (unwinding on some seeds), track
    /// ids around the 0xe..0x10 group.
    #[test]
    fn ssx_audio_system_is_track_group4_matches() {
        use crate::difftest::fakes::FAKE;
        use crate::difftest::GAME_GP;
        covered(
            ee::s08::f_002a3fe8__generated,
            ssx_audio_system_is_track_group4,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_3fe8, 1);
                c.pc = 0x002a_3fe8;
                let idx = seed % 4;
                fill(c, &mut r, DATA + 0x400, 0x40);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x400);
                c.bus.write32(GAME_GP.wrapping_sub(0x1efc), idx);
                c.bus.write32(0x0053_08a8 + 4 * idx, DATA + 0x600);
                c.bus.write32(DATA + 0x600 + 4, DATA + 0x700);
                c.bus
                    .write16(DATA + 0x700 + 8, (r.next_u32() as u16) & 0xfc);
                c.bus.write32(DATA + 0x700 + 0xc, FAKE);
                let track = r.next_u32() % 12;
                c.bus.write32(TRACK_GROUP4_TRACK, track);
                c.bus
                    .write32(TRACK_GROUP4_INFOS + 0x64 * track, 0xb + seed % 8);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the `HandleMusicTrigger` test fakes (in each test machine's
    /// own RAM), followed by the result each faked callee returns.
    pub(crate) const HMT_TEST_CTRL: u32 = DATA + 0x0f_3550;

    pub(crate) const HMT_TEST_MAGIC: u32 = 0x484d_5452;

    pub(crate) const HMT_TEST_RESULTS: u32 = HMT_TEST_CTRL + 4;

    /// Words the trigger's getters return pointers to.
    pub(crate) const HMT_TEST_STATE: u32 = DATA + 0x500;

    pub(crate) const HMT_TEST_VALUE: u32 = DATA + 0x510;

    /// Fake methods: the trigger's getters, the music player, the rider.
    pub(crate) const HMT_TEST_GET_STATE: u32 = FAKE + 0x100;

    pub(crate) const HMT_TEST_GET_VALUE: u32 = FAKE + 0x110;

    /// A gated fake for a callee of `HandleMusicTrigger`: logs the call and
    /// returns the result the fixture chose (`id`), or unwinds.
    macro_rules! hmt_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path, $id:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(HMT_TEST_CTRL) != HMT_TEST_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fake(c, $addr, ra);
                if c.pc == ra {
                    c.g[V0][0] = sx(c.bus.read32(HMT_TEST_RESULTS + 4 * $id));
                }
            }
        };
    }

    hmt_fake!(
        hmt_fake_set_state,
        HMT_PREV_SET_STATE,
        0x0028_def0,
        ee::s07::f_0028def0__generated,
        0
    );

    hmt_fake!(
        hmt_fake_confirm,
        HMT_PREV_CONFIRM,
        0x0028_d740,
        ee::s07::f_0028d740__generated,
        1
    );

    hmt_fake!(
        hmt_fake_race_6_11,
        HMT_PREV_RACE_6_11,
        0x002a_42c8,
        ee::s08::f_002a42c8__generated,
        2
    );

    hmt_fake!(
        hmt_fake_event_4,
        HMT_PREV_EVENT_4,
        0x002a_4040,
        ee::s08::f_002a4040__generated,
        3
    );

    hmt_fake!(
        hmt_fake_add_timer,
        HMT_PREV_ADD_TIMER,
        0x002a_dca0,
        ee::s08::f_002adca0__generated,
        4
    );

    hmt_fake!(
        hmt_fake_2a10c0,
        HMT_PREV_2A10C0,
        0x002a_10c0,
        ee::s08::f_002a10c0__generated,
        5
    );

    /// The `miss` hook: the trigger getters return the fixture's words.
    pub(crate) fn hmt_test_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fake(c, target, ra);
        if c.pc == ra {
            match target {
                HMT_TEST_GET_STATE => c.g[V0][0] = sx(HMT_TEST_STATE),
                HMT_TEST_GET_VALUE => c.g[V0][0] = sx(HMT_TEST_VALUE),
                _ => {}
            }
        }
    }

    /// A vtable at `vt` with `{delta, fn}` slots.
    pub(crate) fn hmt_test_vtable(
        c: &mut Cpu,
        r: &mut crate::difftest::Rng,
        vt: u32,
        slots: &[(u32, u32)],
    ) {
        for &(off, f) in slots {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, f);
        }
    }

    /// `HandleMusicTrigger` with the music-state callees faked and the
    /// small ones real (GetInstance, FindRider, ClearMusicTrigger,
    /// StartMusicTrigger, PlayMusic and StartNextSong on short paths): no
    /// trigger, every trigger state (1, 2, -1 after 1/2 or not, other
    /// positive and negative), and for state 0 another rider or the followed
    /// one with every cue (0 with and without custom music, 0xb in and out of
    /// race types 6-11, 0xd, 0xe..0x11, 0x12, 0x2a, others), the results of
    /// the state switch and mode checks, unchanged and changed triggers, and
    /// unwinds at every faked call.
    #[test]
    fn ssx_audio_system_handle_music_trigger_matches() {
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028DEF0),
            hmt_fake_set_state,
            &HMT_PREV_SET_STATE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028D740),
            hmt_fake_confirm,
            &HMT_PREV_CONFIRM,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A42C8),
            hmt_fake_race_6_11,
            &HMT_PREV_RACE_6_11,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A4040),
            hmt_fake_event_4,
            &HMT_PREV_EVENT_4,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADCA0),
            hmt_fake_add_timer,
            &HMT_PREV_ADD_TIMER,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A10C0),
            hmt_fake_2a10c0,
            &HMT_PREV_2A10C0,
        );
        covered(
            ee::s07::f_0028d988__generated,
            ssx_audio_system_handle_music_trigger,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_d988, 6);
                c.hooks.miss = hmt_test_miss;
                c.pc = 0x0028_d988;
                c.bus.write32(HMT_TEST_CTRL, HMT_TEST_MAGIC);
                let pick = |r: &mut crate::difftest::Rng, set: &[u32]| {
                    set[(r.next_u32() as usize) % set.len()]
                };

                let audio = DATA + 0x10000;
                let trigger = DATA + 0x1000;
                let trigger_vt = DATA + 0x1100;
                let music_vt = DATA + 0x1200;
                let rider_vt = DATA + 0x1300;
                let rider = if seed % 8 == 7 { 0 } else { DATA + 0x2000 };
                let music = DATA + 0x3000;
                let world = DATA + 0x4000;
                let module = DATA + 0x4100;
                let race_man = DATA + 0x4200;
                let replay = DATA + 0x4300;
                let followed = DATA + 0x4400;

                c.g[A0][0] = sx(audio);
                c.g[A1][0] = sx(rider);
                c.g[A2][0] = if seed % 16 == 15 { 0 } else { sx(trigger) };

                // The trigger: state and value getters.
                c.bus.write32(trigger + HMT_TRIGGER_VT, trigger_vt);
                hmt_test_vtable(
                    c,
                    &mut r,
                    trigger_vt,
                    &[
                        (HMT_SLOT_STATE, HMT_TEST_GET_STATE),
                        (HMT_SLOT_VALUE, HMT_TEST_GET_VALUE),
                    ],
                );
                let state = match seed % 8 {
                    0..=3 => 0,
                    4 => pick(&mut r, &[1, 1, 2, 2, 3, 7]),
                    _ => pick(&mut r, &[u32::MAX, u32::MAX, u32::MAX, (-2i32) as u32]),
                };
                let cue = pick(
                    &mut r,
                    &[
                        0, 0, 0xb, 0xb, 0xb, 0xd, 0xd, 0xe, 0xf, 0x10, 0x11, 0x12, 0x12, 0x12,
                        0x2a, 0x2a, 5, 0x13,
                    ],
                );
                c.bus.write32(HMT_TEST_STATE, state);
                c.bus.write32(HMT_TEST_VALUE, cue);
                let last_state = match r.next_u32() % 4 {
                    0 | 1 => state,
                    2 => pick(&mut r, &[1, 2]),
                    _ => pick(&mut r, &[0, 3, u32::MAX]),
                };
                let last_value = if r.next_u32() % 2 == 0 {
                    cue
                } else {
                    cue ^ 0x40
                };
                c.bus.write32(audio + HMT_LAST_STATE, last_state);
                c.bus.write32(audio + HMT_LAST_VALUE, last_value);

                // The audio system.
                let flag = |r: &mut crate::difftest::Rng| if r.next_u32() % 2 == 0 { 0 } else { 1 };
                c.bus.write32(GAME_GP + 0x410, audio);
                c.bus.write32(audio + HMT_MUSIC, music);
                c.bus.write32(music, r.next_u32());
                c.bus.write32(audio + HMT_MUSIC_VT, music_vt);
                hmt_test_vtable(
                    c,
                    &mut r,
                    music_vt,
                    &[(0x18, FAKE + 0x200), (0x20, FAKE + 0x210)],
                );
                for off in [
                    HMT_CUSTOM_MUSIC,
                    HMT_ARMED,
                    HMT_FADE_MODE,
                    HMT_PAUSED,
                    0x5fd4,
                    0x5fd8,
                ] {
                    let v = flag(&mut r);
                    c.bus.write32(audio + off, v);
                }
                // StartMusicTrigger / ClearMusicTrigger: current cue, active rider.
                let current = pick(&mut r, &[0, 3, cue]);
                c.bus.write32(audio + 0x623c, current);
                let other = r.next_u32();
                let active = pick(&mut r, &[u32::MAX, u32::MAX, other]);
                c.bus.write32(audio + 0x62a0, active);
                c.bus.write32(audio + 0x6294, 0);
                if rider != 0 {
                    c.bus.write32(rider + 0x6c0, rider_vt);
                    hmt_test_vtable(c, &mut r, rider_vt, &[(0x38, FAKE + 0x300)]);
                }

                // FindRider: a replay in progress, followed rider = this one or not.
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, module);
                c.bus.write32(module + 0x28, module + 0x80);
                c.bus.write32(module + 0x80, 1 + r.next_u32() % 9);
                c.bus.write32(module + 0xc, race_man);
                c.bus.write32(race_man + 0x40, replay);
                let found = if r.next_u32() % 4 == 0 {
                    followed
                } else {
                    rider
                };
                c.bus.write32(replay + 0x18, found);

                // Faked results: SetMusicState, ConfirmMusicTrigger,
                // IsRaceType6to11, IsEventMode4, AddTimer, 0x2a10c0.
                let set_state = if r.next_u32() % 4 == 0 { 0 } else { 1 };
                let race = flag(&mut r);
                let event4 = if r.next_u32() % 4 == 0 { 0 } else { 1 };
                let results = [set_state, r.next_u32(), race, event4, r.next_u32(), 1];
                for (i, v) in results.into_iter().enumerate() {
                    c.bus.write32(HMT_TEST_RESULTS + 4 * i as u32, v);
                }
            },
        );
    }

    use crate::difftest::put_words;

    pub(crate) use crate::game::fe::feridermanager::tests::oc768_force;
    pub(crate) use crate::game::fe::festatecharselect::tests::oc762_force;
    pub(crate) use crate::game::fe::festatecharselect::tests::oc762_id;
    pub(crate) use crate::game::fe::festatecharselect::tests::FX_SELF_2 as FX_SELF;
    pub(crate) use crate::game::fe::festatecharselect::tests::OC762_FORCE;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc768_machine;

    pub(crate) use crate::game::sound::bankmonitor::tests::oc790_arm;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc796_arm;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc796_force_zero;

    pub(crate) use crate::game::sound::bankmonitor::tests::OC790_OBJ;

    pub(crate) use crate::game::sound::bankmonitor::tests::OC796_OBJ;
    use crate::game::sound::soundsys::tests::oc749_fake_body;
    use crate::game::sound::soundsys::tests::oc749_gate;
    use crate::game::sound::soundsys::tests::oc749_install;
    use crate::game::sound::soundsys::tests::oc749_mgr;
    use crate::game::sound::soundsys::tests::oc749_res;
    use crate::game::sound::soundsys::tests::oc749_unwind;
    pub(crate) use crate::game::sound::soundsys::tests::oc788_arm;
    pub(crate) use crate::game::sound::soundsys::tests::oc788_force;
    pub(crate) use crate::game::sound::soundsys::tests::oc788_mail;
    pub(crate) use crate::game::sound::soundsys::tests::OC788_CTRL;
    pub(crate) use crate::game::sound::soundsys::tests::OC788_MAGIC;
    pub(crate) use crate::game::worldpainter::worldpainterman::tests::oc762_machine;
    pub(crate) use crate::game::worldpainter::worldpainterman::tests::OC762_CTRL;
    pub(crate) use crate::game::worldpainter::worldpainterman::tests::OC762_MAGIC;

    /// Call counters, one word per callee.
    pub(crate) const OC762_COUNT: u32 = DATA + 0x80_0000;

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// the forced result of this call (if any) unless it unwound.
    pub(crate) fn oc762_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let id = oc762_id(addr);
        let nth = c.bus.read32(OC762_COUNT + id * 4);
        c.bus.write32(OC762_COUNT + id * 4, nth + 1);
        let mail = if nth < 16 {
            c.bus.read32(OC762_FORCE + id * 0x40 + nth * 4)
        } else {
            0
        };
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = mail.wrapping_sub(3) as u64;
        }
    }

    macro_rules! oc762_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC762_CTRL) != OC762_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc762_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc762_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc762_fakes!(
        (
            oc762_fake_00317e30,
            OC762_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc762_fake_00317670,
            OC762_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc762_fake_00317e98,
            OC762_PREV_00317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc762_fake_00417828,
            OC762_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc762_fake_00416210,
            OC762_PREV_00416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
        (
            oc762_fake_00413af8,
            OC762_PREV_00413AF8,
            ee::s15::OVERRIDE_00413AF8,
            ee::s15::f_00413af8__generated,
            0x0041_3af8
        ),
        (
            oc762_fake_0014dc80,
            OC762_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc762_fake_0014dd58,
            OC762_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc762_fake_00148bc8,
            OC762_PREV_00148BC8,
            ee::s01::OVERRIDE_00148BC8,
            ee::s01::f_00148bc8__generated,
            0x0014_8bc8
        ),
        (
            oc762_fake_00148c68,
            OC762_PREV_00148C68,
            ee::s01::OVERRIDE_00148C68,
            ee::s01::f_00148c68__generated,
            0x0014_8c68
        ),
        (
            oc762_fake_00148b78,
            OC762_PREV_00148B78,
            ee::s01::OVERRIDE_00148B78,
            ee::s01::f_00148b78__generated,
            0x0014_8b78
        ),
        (
            oc762_fake_00148cb8,
            OC762_PREV_00148CB8,
            ee::s01::OVERRIDE_00148CB8,
            ee::s01::f_00148cb8__generated,
            0x0014_8cb8
        ),
        (
            oc762_fake_00148d58,
            OC762_PREV_00148D58,
            ee::s01::OVERRIDE_00148D58,
            ee::s01::f_00148d58__generated,
            0x0014_8d58
        ),
        (
            oc762_fake_00148d08,
            OC762_PREV_00148D08,
            ee::s01::OVERRIDE_00148D08,
            ee::s01::f_00148d08__generated,
            0x0014_8d08
        ),
        (
            oc762_fake_00148c18,
            OC762_PREV_00148C18,
            ee::s01::OVERRIDE_00148C18,
            ee::s01::f_00148c18__generated,
            0x0014_8c18
        ),
        (
            oc762_fake_0039d860,
            OC762_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc762_fake_0039c428,
            OC762_PREV_0039C428,
            ee::s13::OVERRIDE_0039C428,
            ee::s13::f_0039c428__generated,
            0x0039_c428
        ),
        (
            oc762_fake_003a0c70,
            OC762_PREV_003A0C70,
            ee::s13::OVERRIDE_003A0C70,
            ee::s13::f_003a0c70__generated,
            0x003a_0c70
        ),
        (
            oc762_fake_00398798,
            OC762_PREV_00398798,
            ee::s13::OVERRIDE_00398798,
            ee::s13::f_00398798__generated,
            0x0039_8798
        ),
        (
            oc762_fake_001a0508,
            OC762_PREV_001A0508,
            ee::s03::OVERRIDE_001A0508,
            ee::s03::f_001a0508__generated,
            0x001a_0508
        ),
        (
            oc762_fake_003e6448,
            OC762_PREV_003E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc762_fake_003e6574,
            OC762_PREV_003E6574,
            ee::s14::OVERRIDE_003E6574,
            ee::s14::f_003e6574__generated,
            0x003e_6574
        ),
        (
            oc762_fake_002b4a20,
            OC762_PREV_002B4A20,
            ee::s08::OVERRIDE_002B4A20,
            ee::s08::f_002b4a20__generated,
            0x002b_4a20
        ),
        (
            oc762_fake_002b21e0,
            OC762_PREV_002B21E0,
            ee::s08::OVERRIDE_002B21E0,
            ee::s08::f_002b21e0__generated,
            0x002b_21e0
        ),
        (
            oc762_fake_002c03a8,
            OC762_PREV_002C03A8,
            ee::s08::OVERRIDE_002C03A8,
            ee::s08::f_002c03a8__generated,
            0x002c_03a8
        ),
        (
            oc762_fake_0028b830,
            OC762_PREV_0028B830,
            ee::s07::OVERRIDE_0028B830,
            ee::s07::f_0028b830__generated,
            0x0028_b830
        ),
        (
            oc762_fake_0028a728,
            OC762_PREV_0028A728,
            ee::s07::OVERRIDE_0028A728,
            ee::s07::f_0028a728__generated,
            0x0028_a728
        ),
        (
            oc762_fake_0028b320,
            OC762_PREV_0028B320,
            ee::s07::OVERRIDE_0028B320,
            ee::s07::f_0028b320__generated,
            0x0028_b320
        ),
    );

    /// `SndBankMan_LoadBank`: the bank is already loaded (forced `v0` nonzero)
    /// or not (0, so the cancel and the load calls follow); fakes unwind at
    /// any of the three calls.
    #[test]
    fn snd_bank_man_load_bank_matches() {
        oc762_install_fakes();
        covered(
            ssx3_ee::s07::f_0028bc58__generated,
            snd_bank_man_load_bank_2,
            200,
            &|c, seed| {
                oc762_machine(c, seed, 0x0028_bc58, 3);
                oc762_force(c, 0x0028_b830, 0, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(FX_SELF);
                c.bus
                    .write32(FX_SELF + 0xacc, DATA + 0x21_6000 + (seed % 5) * 0x10);
            },
        );
    }

    // -- SSXAudioSystem_PlayMusic ---------------------------------------------

    pub(crate) const PLAY_THIS: u32 = DATA + 0x10000;

    pub(crate) const PLAY_VT: u32 = DATA + 0x20000;

    pub(crate) const PLAY_LOC: u32 = DATA + 0x30000;

    /// Replay playing or not, radio mode or not, locations 0..21 (peak theme
    /// from 17 up), request line / skip flag / race-type query combinations,
    /// the three peak categories (the real `TrackToCategory` with the game
    /// image) and both copy loops (aligned and unaligned location data). The
    /// sound queue of the real `SetTrackedSoundExpiry`-like callees stays zero.
    #[test]
    fn ssx_audio_system_play_music_matches() {
        covered(
            ee::s07::f_0028cf98__generated,
            ssx_audio_system_play_music,
            240,
            &|c, seed| {
                let mut r = oc768_machine(c, seed, 0x0028_cf98, 30);
                elf_image(c);
                let bits = r.next_u32() ^ seed.wrapping_mul(0x9e37_79b1);
                let this = PLAY_THIS;
                oc768_force(c, 0x0028_8ae0, (seed % 3 == 0) as u32);
                c.bus.write32(this + 0x6294, (bits >> 3) & 1);
                c.bus.write32(
                    this + 0x608c,
                    if (bits >> 4) % 3 == 0 {
                        2
                    } else {
                        (bits >> 8) & 1
                    },
                );
                c.bus.write32(this + 0x508, r.next_u32());
                c.bus.write32(this + 0x5558, PLAY_VT);
                c.bus
                    .write16(PLAY_VT + 0x10, (r.next_u32() & 0xfffc) as u16);
                c.bus.write32(PLAY_VT + 0x14, fakes::FAKE + 0x10);
                c.bus
                    .write16(PLAY_VT + 0x20, (r.next_u32() & 0xfffc) as u16);
                c.bus.write32(PLAY_VT + 0x24, fakes::FAKE + 0x20);
                let loc = PLAY_LOC + if (bits >> 12) & 1 == 1 { 4 } else { 0 };
                fill(c, &mut r, PLAY_LOC, 0x20);
                c.bus.write32(loc, r.next_u32() % 22);
                oc768_force(c, 0x0014_4bc0, loc);
                oc768_force(c, 0x002a_42c8, (bits >> 13) & 1);
                oc768_force(c, 0x0028_9c18, (bits >> 14) % 3);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (r.next_u32() % 0x40) as u64;
                c.g[A2][0] = ((bits >> 16) & 1) as u64;
                c.g[A3][0] = match (bits >> 17) & 3 {
                    0 => u64::MAX,
                    n => n as u64,
                };
                c.g[T0][0] = ((bits >> 19) & 1) as u64;
            },
        );
    }

    /// Gate word of this module's callee fakes (the machine's own RAM, away
    /// from the pool and the fakes' call log at DATA+0x100000). Results and
    /// the unwind index come from the `oc749` control block.
    pub(crate) const OC772_CTRL: u32 = DATA + 0x22_0000;

    pub(crate) const OC772_MAGIC: u32 = 0x0772_0772;

    macro_rules! oc772_fakes {
        ($($fake:ident = $addr:expr, $shard:ident :: $slot:ident / $gen:ident;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC772_CTRL) != OC772_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::$shard::$gen);
                    }
                    if oc749_unwind(c) {
                        return;
                    }
                    oc749_fake_body(c, $addr);
                    ret(c);
                }
            )*
            pub(crate) fn oc772_install() {
                oc749_install();
                $(
                    fakes::hook(
                        addr_of_mut!(ee::$shard::$slot),
                        $fake,
                        fakes::prev_cell($fake),
                    );
                )*
            }
        };
    }

    oc772_fakes! {
        oc772_fake_287558 = 0x287558, s07::OVERRIDE_00287558 / f_00287558__generated;
        oc772_fake_28b1b0 = 0x28b1b0, s07::OVERRIDE_0028B1B0 / f_0028b1b0__generated;
        oc772_fake_2b4070 = 0x2b4070, s08::OVERRIDE_002B4070 / f_002b4070__generated;
        oc772_fake_2b49e0 = 0x2b49e0, s08::OVERRIDE_002B49E0 / f_002b49e0__generated;
        oc772_fake_2b4af0 = 0x2b4af0, s08::OVERRIDE_002B4AF0 / f_002b4af0__generated;
        oc772_fake_295028 = 0x295028, s08::OVERRIDE_00295028 / f_00295028__generated;
        oc772_fake_295628 = 0x295628, s08::OVERRIDE_00295628 / f_00295628__generated;
    }

    /// Both gates on (and maybe one fake unwinds).
    pub(crate) fn oc772_gate(c: &mut Cpu, r: &mut Rng) {
        oc749_gate(c, r);
        c.bus.write32(OC772_CTRL, OC772_MAGIC);
    }

    /// A sound manager fixture shared by the voice-pushing functions: the
    /// audio system at `this` (`+0`/`+4`/`+8` the manager and its tables,
    /// `+0x118` the music player whose first word is a second manager).
    pub(crate) fn oc772_voice_system(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        let this = DATA + 0x100;
        let p = DATA + 0x400;
        let mgr = DATA + 0x1000;
        let q = DATA + 0x2000;
        c.g[A0][0] = sx(this);
        c.bus.write32(this, mgr);
        c.bus.write32(this + 4, DATA + 0x3000);
        c.bus.write32(this + 8, DATA + 0x3100);
        c.bus.write32(this + 0x118, p);
        c.bus.write32(p, q);
        oc749_mgr(c, r, mgr, DATA + 0x10000, seed % 2);
        oc749_mgr(c, r, q, DATA + 0x14000, (seed / 2) % 3);
        this
    }

    /// `SSXAudioSystem_PlayLoadingScreenLoop`: pushes the loop voice (the
    /// volume clamp always sees the 127 it just stored), with fakes that
    /// unwind at some call.
    #[test]
    fn ssx_audio_system_play_loading_screen_loop_matches() {
        oc772_install();
        check_lohi(
            ee::s07::f_0028f768__generated,
            ssx_audio_system_play_loading_screen_loop,
            300,
            &|c, seed| {
                let mut r = Rng(0x0028_f768 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                oc772_gate(c, &mut r);
                oc772_voice_system(c, &mut r, seed);
                oc749_res(c, 0x287968, DATA + 0x3200);
                oc749_res(c, 0x2906b8, r.next_u32());
            },
        );
    }

    /// The audio system for `SetMusicMode`: the cached mode (+0x608c), the
    /// radio playlist (+0x6098), the default map (+0x518), the song-playing
    /// flag (+0x530), and what the music-player fakes report.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn set_music_mode_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        this: u32,
        mode: u32,
        cached: u32,
        in_game: bool,
        playing: bool,
        playlist: bool,
        state: u32,
        busy: u32,
    ) {
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(mode);
        c.bus.write32(this + 0x608c, cached);
        c.bus.write64(
            this + 0x6098,
            if playlist { DATA as u64 + 0xe000 } else { 0 },
        );
        c.bus.write64(this + 0x518, DATA as u64 + 0xe100);
        c.bus.write32(this + 0x530, playing as u32);
        c.bus.write32(this + 0x5fd0, r.next_u32());
        c.bus.write32(this + 0x5fd4, r.next_u32());
        // The BE option block the function copies to its stack.
        fill(c, r, 0x0053_5610, 0xa2);
        oc749_res(c, 0x28b1b0, in_game as u32);
        oc749_res(c, 0x2b49e0, state);
        oc749_res(c, 0x2b4af0, busy);
        oc749_res(c, 0x14dc80, DATA + 0x500);
        oc749_res(c, 0x14dd58, DATA + 0x510);
    }

    /// Would the mode change reach `SelectRandomSong` / `PlayMusic`?
    pub(crate) fn set_music_mode_plays(
        mode: u32,
        in_game: bool,
        playing: bool,
        playlist: bool,
        state: u32,
        busy: u32,
    ) -> bool {
        match mode {
            0 => in_game && playing && state.wrapping_sub(101) < 3,
            1 | 3 => {
                playlist
                    && in_game
                    && playing
                    && state.wrapping_sub(1) >= 3
                    && (busy == 0 || state == 0x12d)
            }
            2 => in_game && state.wrapping_sub(101) >= 3,
            _ => false,
        }
    }

    pub(crate) const MUSIC_STATES: [u32; 13] =
        [0, 1, 2, 3, 4, 100, 101, 102, 103, 104, 300, 301, 302];

    /// `SSXAudioSystem_SetMusicMode`: every mode (also an unknown one and the
    /// unchanged-mode early exit), in game or not, song playing or not, and
    /// the music-player states the checks look at. The two real callees
    /// without an override slot (`SelectRandomSong`, `PlayMusic`) are kept
    /// off the path here (see the next test).
    #[test]
    fn ssx_audio_system_set_music_mode_matches() {
        oc772_install();
        check(
            ee::s07::f_0028bf78__generated,
            ssx_audio_system_set_music_mode,
            400,
            &|c, seed| {
                let mut r = Rng(0x0028_bf78 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                oc772_gate(c, &mut r);
                let mode = [0xffff_ffffu32, 0, 1, 2, 3, 4, 7][seed as usize % 7];
                let cached = if seed % 5 == 0 {
                    mode
                } else {
                    mode.wrapping_add(1 + r.next_u32() % 3)
                };
                let mut in_game = r.next_u32() % 4 != 0;
                let playing = r.next_u32() % 3 != 0;
                let playlist = r.next_u32() % 3 != 0;
                let state = MUSIC_STATES[r.next_u32() as usize % 13];
                let busy = r.next_u32() % 2;
                if cached != mode
                    && set_music_mode_plays(mode, in_game, playing, playlist, state, busy)
                {
                    in_game = false;
                }
                set_music_mode_fixture(
                    c,
                    &mut r,
                    DATA + 0x100,
                    mode,
                    cached,
                    in_game,
                    playing,
                    playlist,
                    state,
                    busy,
                );
            },
        );
    }

    /// `SSXAudioSystem_SetMusicMode` down to `SelectRandomSong` and
    /// `PlayMusic` (modes 0, 1, 2 and 3 with every condition met), on the
    /// music fixture of `SSXAudioSystem_UpdatePeakMusic` (a replay is
    /// playing, so `PlayMusic` returns at once).
    #[test]
    fn ssx_audio_system_set_music_mode_plays_matches() {
        oc772_install();
        check(
            ee::s07::f_0028bf78__generated,
            ssx_audio_system_set_music_mode,
            120,
            &|c, seed| {
                update_peak_music_setup(c, seed);
                let mut r = Rng(0x0028_bf78 ^ 0x5eed ^ (seed as u64) << 12);
                c.bus.write32(OC772_CTRL, OC772_MAGIC);
                let mode = [0u32, 1, 2, 3][seed as usize % 4];
                let (state, busy) = match mode {
                    0 => (101 + seed % 3, 0),
                    2 => (seed % 3, 0),
                    _ if seed % 8 < 4 => (5 + seed % 4, 0),
                    _ => (0x12d, 1),
                };
                set_music_mode_fixture(
                    c,
                    &mut r,
                    DATA,
                    mode,
                    mode.wrapping_add(2),
                    true,
                    true,
                    true,
                    state,
                    busy,
                );
                c.pc = 0x0028_bf78;
            },
        );
    }

    /// `SSXAudioSystem_SetFEMusicScreen`: screens -1..=11 (out of range, all
    /// ten table cases), unchanged screen, music-system flag word, unwind in
    /// the bank call, and a corrupt table entry (jump out).
    #[test]
    fn ssx_audio_system_set_fe_music_screen_matches() {
        let table: [u32; 10] = [
            0x28f17c, 0x28f190, 0x28f1c8, 0x28f198, 0x28f1a0, 0x28f1a8, 0x28f1b0, 0x28f1b8,
            0x28f1c0, 0x28f1c8,
        ];
        fakes::covered(
            ee::s07::f_0028f140__generated,
            ssx_audio_system_set_fe_music_screen,
            420,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7761, 1);
                c.pc = 0x0028_f140;
                put_words(c, 0x0048_2a70, &table);
                let sys = DATA + 0x1_0000;
                let vt = DATA + 0x3_0000;
                c.g[A0][0] = sx(sys);
                let screen = (seed % 13).wrapping_sub(1);
                c.g[A1][0] = sx(screen);
                let old = if (seed / 13) % 4 == 0 {
                    screen
                } else {
                    r.next_u32()
                };
                c.bus.write32(sys + FE_MUSIC_SCREEN, old);
                let flag = if (seed / 13) % 2 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32()
                };
                c.bus.write32(sys + 0x534, flag);
                c.bus.write32(sys + FE_MUSIC_SYS_VTABLE, vt);
                c.bus
                    .write16(vt + 0x20, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x24, fakes::FAKE);
                if seed % 37 == 0 && screen < 10 {
                    c.bus.write32(0x0048_2a70 + 4 * screen, 0x0028_f150);
                }
            },
        );
    }

    /// Gate word of this module's fakes (clear of the fakes' call log at
    /// `DATA + 0x10_0000` and pointer range `DATA .. DATA + 0x4_0000`); the
    /// word after it is the per-seed mask that picks which result-altering
    /// fake calls return 0 / -1.
    pub(crate) const OC782_CTRL: u32 = DATA + 0x2c_0000;

    pub(crate) const OC782_MAGIC: u32 = 0x4f43_3738 + 2;

    pub(crate) const OC782_OBJ: u32 = DATA + 0x2e_0000;

    /// Scrambled machine with the fakes on, the gate word and per-seed mask,
    /// the BELibrary dummy (gp-0x1f04) and a world (gp-0x848) with the
    /// in-game flag word (+0x84) and a random +0x7c.
    pub(crate) fn oc782_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc782_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC782_CTRL, OC782_MAGIC);
        c.bus.write32(OC782_CTRL + 4, r.next_u32());
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), OC782_OBJ + 0x8000);
        c.bus
            .write32(GAME_GP.wrapping_sub(0x848), OC782_OBJ + 0x9000);
        c.bus
            .write32(OC782_OBJ + 0x9000 + 0x7c, r.next_u32() & 0xfff0);
        c.bus
            .write32(OC782_OBJ + 0x9000 + 0x84, [0, 1, 2][(seed % 3) as usize]);
        r
    }

    /// Forces the result of the `n`th fake call.
    pub(crate) fn oc782_force(c: &mut Cpu, n: u32, value: u32) {
        c.bus.write32(fakes::LOG_RESULTS + 4 * n, value);
    }

    /// `SSXAudioSystem_SetRequestPlaylist`: same / changed playlist, radio
    /// modes 0..5, in game or not, with or without the music system, state
    /// callee results in the stop range or not. The branch into
    /// `SelectRandomSong`/`PlayMusic` (unported, deep callees) is not
    /// reached: the faked state results never ask for it.
    #[test]
    fn ssx_audio_system_set_request_playlist_matches() {
        fakes::covered(
            ee::s07::f_0028c2d0__generated,
            ssx_audio_system_set_request_playlist,
            128,
            &|c, seed| {
                let mut r = oc782_fixture(c, seed, 0x782c, 3);
                let sys = OC782_OBJ + 0x1_0000;
                let key = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
                c.bus
                    .write64(sys + 0x6098, if seed % 2 == 0 { key } else { !key });
                c.bus.write32(
                    sys + 0x608c,
                    [1, 3, 0, 2, 5, 1, 3, 4][(seed / 2 % 8) as usize],
                );
                c.bus.write32(
                    sys + 0x530,
                    if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(sys + 0x118, r.next_u32());
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = key;
                // The state call (first or second fake call) is in 1..3 for some seeds.
                let idx = if seed % 2 == 0 { 0 } else { 1 };
                if seed % 3 == 0 {
                    oc782_force(c, idx, 2);
                }
            },
        );
    }

    /// `SndBankMan_ctor`: embedded or not, 0..4 banks. The load queue, bank
    /// monitor and bank ctors run for real; the allocator and the sound
    /// manager reference ctor are gated fakes (some unwind).
    #[test]
    fn snd_bank_man_ctor_matches() {
        fakes::covered(
            ee::s07::f_0028b9b8__generated,
            snd_bank_man_ctor,
            64,
            &|c, seed| {
                let mut r = oc782_fixture(c, seed, 0x782f, 8);
                let this = OC782_OBJ + 0x1_0000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = [0u64, 1, 2, 4][(seed / 2 % 4) as usize];
                c.g[A3][0] = sx(r.next_u32() & 0xff);
            },
        );
    }

    /// Per-machine mask word: bit n set forces the n-th `0x145f90`/`0x146008`
    /// call of `event_select_init_peak_locks` to return 0.
    pub(crate) const OC786_ZERO_MASK: u32 = DATA + 0x20_0000;

    pub(crate) const OC786_ZERO_COUNT: u32 = DATA + 0x20_0004;

    /// The word that arms oc786's callee fakes in a machine's RAM.
    pub(crate) const OC786_CTRL: u32 = DATA + 0x30_0000;

    pub(crate) const OC786_MAGIC: u32 = 0x0786_f4ce;

    /// Gated fake body: log like `fakes::fake_call`; the lock queries
    /// (0x145f90, 0x146008) may then return 0 per the machine's mask.
    pub(crate) fn oc786_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc == ra && (addr == 0x0014_5f90 || addr == 0x0014_6008) {
            let k = c.bus.read32(OC786_ZERO_COUNT);
            c.bus.write32(OC786_ZERO_COUNT, k + 1);
            if (c.bus.read32(OC786_ZERO_MASK) >> (k & 31)) & 1 != 0 {
                c.g[V0][0] = 0;
            }
        }
    }

    macro_rules! oc786_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC786_CTRL) != OC786_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc786_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc786_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc786_fakes!(
        (
            oc786_fk_14dc80,
            OC786_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc786_fk_14dd58,
            OC786_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc786_fk_147488,
            OC786_PREV_147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488
        ),
        (
            oc786_fk_145f90,
            OC786_PREV_145F90,
            ee::s01::OVERRIDE_00145F90,
            ee::s01::f_00145f90__generated,
            0x0014_5f90
        ),
        (
            oc786_fk_146008,
            OC786_PREV_146008,
            ee::s01::OVERRIDE_00146008,
            ee::s01::f_00146008__generated,
            0x0014_6008
        ),
        (
            oc786_fk_1d53b0,
            OC786_PREV_1D53B0,
            ee::s04::OVERRIDE_001D53B0,
            ee::s04::f_001d53b0__generated,
            0x001d_53b0
        ),
        (
            oc786_fk_227f80,
            OC786_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80
        ),
        (
            oc786_fk_369890,
            OC786_PREV_369890,
            ee::s12::OVERRIDE_00369890,
            ee::s12::f_00369890__generated,
            0x0036_9890
        ),
        (
            oc786_fk_369690,
            OC786_PREV_369690,
            ee::s12::OVERRIDE_00369690,
            ee::s12::f_00369690__generated,
            0x0036_9690
        ),
        (
            oc786_fk_30dbd0,
            OC786_PREV_30DBD0,
            ee::s10::OVERRIDE_0030DBD0,
            ee::s10::f_0030dbd0__generated,
            0x0030_dbd0
        ),
        (
            oc786_fk_317d70,
            OC786_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc786_fk_3118d0,
            OC786_PREV_3118D0,
            ee::s10::OVERRIDE_003118D0,
            ee::s10::f_003118d0__generated,
            0x0031_18d0
        ),
        (
            oc786_fk_104d50,
            OC786_PREV_104D50,
            ee::s00::OVERRIDE_00104D50,
            ee::s00::f_00104d50__generated,
            0x0010_4d50
        ),
        (
            oc786_fk_3128e8,
            OC786_PREV_3128E8,
            ee::s10::OVERRIDE_003128E8,
            ee::s10::f_003128e8__generated,
            0x0031_28e8
        ),
        (
            oc786_fk_312598,
            OC786_PREV_312598,
            ee::s10::OVERRIDE_00312598,
            ee::s10::f_00312598__generated,
            0x0031_2598
        ),
        (
            oc786_fk_310200,
            OC786_PREV_310200,
            ee::s10::OVERRIDE_00310200,
            ee::s10::f_00310200__generated,
            0x0031_0200
        ),
        (
            oc786_fk_19e680,
            OC786_PREV_19E680,
            ee::s03::OVERRIDE_0019E680,
            ee::s03::f_0019e680__generated,
            0x0019_e680
        ),
    );

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc788_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc788_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = sx(mail.wrapping_sub(3));
        }
    }

    macro_rules! oc788_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC788_CTRL) != OC788_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc788_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc788_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc788_fakes!(
        (
            oc788_fk_14dc80,
            OC788_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc788_fk_14dd58,
            OC788_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc788_fk_14bdb8,
            OC788_PREV_14BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8
        ),
        (
            oc788_fk_14ad28,
            OC788_PREV_14AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014_ad28
        ),
        (
            oc788_fk_14d448,
            OC788_PREV_14D448,
            ee::s01::OVERRIDE_0014D448,
            ee::s01::f_0014d448__generated,
            0x0014_d448
        ),
        (
            oc788_fk_147488,
            OC788_PREV_147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488
        ),
        (
            oc788_fk_30eb60,
            OC788_PREV_30EB60,
            ee::s10::OVERRIDE_0030EB60,
            ee::s10::f_0030eb60__generated,
            0x0030_eb60
        ),
        (
            oc788_fk_30ec18,
            OC788_PREV_30EC18,
            ee::s10::OVERRIDE_0030EC18,
            ee::s10::f_0030ec18__generated,
            0x0030_ec18
        ),
        (
            oc788_fk_317670,
            OC788_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc788_fk_39d860,
            OC788_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc788_fk_39b960,
            OC788_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
        (
            oc788_fk_39e4c0,
            OC788_PREV_39E4C0,
            ee::s13::OVERRIDE_0039E4C0,
            ee::s13::f_0039e4c0__generated,
            0x0039_e4c0
        ),
        (
            oc788_fk_1a0508,
            OC788_PREV_1A0508,
            ee::s03::OVERRIDE_001A0508,
            ee::s03::f_001a0508__generated,
            0x001a_0508
        ),
        (
            oc788_fk_1a0548,
            OC788_PREV_1A0548,
            ee::s03::OVERRIDE_001A0548,
            ee::s03::f_001a0548__generated,
            0x001a_0548
        ),
        (
            oc788_fk_181ef0,
            OC788_PREV_181EF0,
            ee::s02::OVERRIDE_00181EF0,
            ee::s02::f_00181ef0__generated,
            0x0018_1ef0
        ),
        (
            oc788_fk_29f128,
            OC788_PREV_29F128,
            ee::s08::OVERRIDE_0029F128,
            ee::s08::f_0029f128__generated,
            0x0029_f128
        ),
        (
            oc788_fk_2a4040,
            OC788_PREV_2A4040,
            ee::s08::OVERRIDE_002A4040,
            ee::s08::f_002a4040__generated,
            0x002a_4040
        ),
        (
            oc788_fk_295028,
            OC788_PREV_295028,
            ee::s08::OVERRIDE_00295028,
            ee::s08::f_00295028__generated,
            0x0029_5028
        ),
        (
            oc788_fk_2b1458,
            OC788_PREV_2B1458,
            ee::s08::OVERRIDE_002B1458,
            ee::s08::f_002b1458__generated,
            0x002b_1458
        ),
        (
            oc788_fk_2a3f90,
            OC788_PREV_2A3F90,
            ee::s08::OVERRIDE_002A3F90,
            ee::s08::f_002a3f90__generated,
            0x002a_3f90
        ),
        (
            oc788_fk_2adca0,
            OC788_PREV_2ADCA0,
            ee::s08::OVERRIDE_002ADCA0,
            ee::s08::f_002adca0__generated,
            0x002a_dca0
        ),
        (
            oc788_fk_2b4908,
            OC788_PREV_2B4908,
            ee::s08::OVERRIDE_002B4908,
            ee::s08::f_002b4908__generated,
            0x002b_4908
        ),
        (
            oc788_fk_3d8008,
            OC788_PREV_3D8008,
            ee::s14::OVERRIDE_003D8008,
            ee::s14::f_003d8008__generated,
            0x003d_8008
        ),
        (
            oc788_fk_28def0,
            OC788_PREV_28DEF0,
            ee::s07::OVERRIDE_0028DEF0,
            ee::s07::f_0028def0__generated,
            0x0028_def0
        ),
        (
            oc788_fk_289c18,
            OC788_PREV_289C18,
            ee::s07::OVERRIDE_00289C18,
            ee::s07::f_00289c18__generated,
            0x0028_9c18
        ),
        (
            oc788_fk_252fa0,
            OC788_PREV_252FA0,
            ee::s06::OVERRIDE_00252FA0,
            ee::s06::f_00252fa0__generated,
            0x0025_2fa0
        ),
    );

    /// `SndBank_GetBuffer`: buffer already there (no call) and not (allocator
    /// call, stores buffer and size), and the unwind of the call.
    #[test]
    fn snd_bank_get_buffer_matches() {
        oc788_install_fakes();
        fakes::covered(
            ee::s07::f_0028b8d8__generated,
            snd_bank_get_buffer,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x788a, 1);
                oc788_arm(c);
                c.pc = 0x0028_b8d8;
                let bank = DATA + 0x1000;
                c.g[A0][0] = sx(bank);
                c.g[A1][0] = sx(r.next_u32());
                let has = if seed % 2 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(bank + 8, has);
                c.bus.write32(bank + 0x10, r.next_u32());
                c.bus.write32(bank + 0x14, r.next_u32());
            },
        );
    }

    /// `SSXAudioSystem_ResetMusicFadeIn`: rating tier zero and non-zero, and
    /// the unwind of the tier call.
    #[test]
    fn ssx_audio_system_reset_music_fade_in_matches() {
        oc788_install_fakes();
        fakes::covered(
            ee::s07::f_0028f108__generated,
            ssx_audio_system_reset_music_fade_in,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x788b, 1);
                oc788_arm(c);
                c.pc = 0x0028_f108;
                let sys = DATA + 0x1000;
                c.g[A0][0] = sx(sys);
                let tier = if seed % 3 == 0 { 0 } else { 1 + seed % 5 };
                oc788_force(c, 0x0028_9c18, tier);
                c.bus.write32(sys + 0x628c, r.next_u32());
            },
        );
    }

    /// `SndBankMan_LoadBankFromMemory`: random bank index, array pointer and arguments; the
    /// callee `SndBank_LoadFromMemory` is a gated fake that sometimes unwinds.
    #[test]
    fn snd_bank_man_load_bank_from_memory_matches() {
        oc790_install_fakes();
        fakes::covered(
            ee::s07::f_0028be60__generated,
            snd_bank_man_load_bank_from_memory,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x790d, 1);
                oc790_arm(c);
                c.pc = 0x0028_be60;
                c.g[A0][0] = sx(OC790_OBJ);
                c.g[A1][0] = sx(r.next_u32() % 40);
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32());
                c.bus.write32(OC790_OBJ + 0xacc, r.next_u32());
            },
        );
    }

    /// A game-like object area well above the fakes' pointer results.
    pub(crate) const OC791_OBJ: u32 = DATA + 0x8_0000;

    /// The world / game-module record `gp-0x848` points at.
    pub(crate) const OC791_WORLD: u32 = DATA + 0x9_0000;

    /// Per-machine mask word: bit n (n < 31) forces the n-th fake call to
    /// return 0.
    pub(crate) const OC791_MASK: u32 = DATA + 0x20_0000;

    /// Gate word of the oc791 fakes (a magic value in the machine's RAM).
    pub(crate) const OC791_CTRL: u32 = DATA + 0x21_0000;

    pub(crate) const OC791_MAGIC: u32 = 0x4f43_3739;

    /// The standard fake call, then optionally a zero result.
    pub(crate) fn oc791_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc == ra {
            let n = c.bus.read32(fakes::LOG_COUNT).wrapping_sub(1);
            let m = c.bus.read32(OC791_MASK);
            if n < 31 && (m >> n) & 1 != 0 {
                c.g[V0][0] = 0;
            }
        }
    }

    macro_rules! oc791_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC791_CTRL) != OC791_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc791_miss(c, $addr);
                }
            )*
            /// Hook the gated fakes on the callee slots (once per process)
            /// and open the gate in this machine.
            pub(crate) fn oc791_install_fakes(c: &mut Cpu) {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
                c.bus.write32(OC791_CTRL, OC791_MAGIC);
            }
        };
    }

    oc791_fakes!(
        (
            oc791_f_14dc80,
            OC791_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc791_f_14dd58,
            OC791_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc791_f_144bc0,
            OC791_PREV_144BC0,
            ee::s01::OVERRIDE_00144BC0,
            ee::s01::f_00144bc0__generated,
            0x0014_4bc0
        ),
        (
            oc791_f_317670,
            OC791_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc791_f_317d70,
            OC791_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc791_f_397cc0,
            OC791_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc791_f_39c870,
            OC791_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc791_f_2a4318,
            OC791_PREV_2A4318,
            ee::s08::OVERRIDE_002A4318,
            ee::s08::f_002a4318__generated,
            0x002a_4318
        ),
        (
            oc791_f_2a4368,
            OC791_PREV_2A4368,
            ee::s08::OVERRIDE_002A4368,
            ee::s08::f_002a4368__generated,
            0x002a_4368
        ),
        (
            oc791_f_2a40e0,
            OC791_PREV_2A40E0,
            ee::s08::OVERRIDE_002A40E0,
            ee::s08::f_002a40e0__generated,
            0x002a_40e0
        ),
        (
            oc791_f_2a4158,
            OC791_PREV_2A4158,
            ee::s08::OVERRIDE_002A4158,
            ee::s08::f_002a4158__generated,
            0x002a_4158
        ),
        (
            oc791_f_2a41c8,
            OC791_PREV_2A41C8,
            ee::s08::OVERRIDE_002A41C8,
            ee::s08::f_002a41c8__generated,
            0x002a_41c8
        ),
        (
            oc791_f_2a4238,
            OC791_PREV_2A4238,
            ee::s08::OVERRIDE_002A4238,
            ee::s08::f_002a4238__generated,
            0x002a_4238
        ),
        (
            oc791_f_3de670,
            OC791_PREV_3DE670,
            ee::s14::OVERRIDE_003DE670,
            ee::s14::f_003de670__generated,
            0x003d_e670
        ),
    );

    /// `SSXAudioSystem_OnEventSelectEnter`: paused, not in game, SFX
    /// already paused, radio mode 2 or not, and unwinds in each call.
    #[test]
    fn ssx_audio_system_on_event_select_enter_matches() {
        fakes::covered(
            ee::s07::f_0028f5b8__generated,
            ssx_audio_system_on_event_select_enter,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28f5b8, 8);
                c.hooks.miss = oc791_miss;
                c.pc = 0x0028_f5b8;
                c.g[A0][0] = sx(OC791_OBJ);
                c.bus.write32(OC791_MASK, 0);
                c.bus.write32(GAME_GP - 0x848, OC791_WORLD);
                c.bus
                    .write32(OC791_WORLD + 0x84, if seed % 7 < 2 { 0 } else { 1 });
                c.bus.write32(OC791_OBJ, OC791_OBJ + 0x1_0000);
                c.bus.write32(OC791_OBJ + 0x5fb4, (seed % 5 == 0) as u32);
                c.bus.write32(OC791_OBJ + 0x5818, (seed % 3 == 0) as u32);
                c.bus.write32(
                    OC791_OBJ + 0x608c,
                    if seed % 4 == 0 { 2 } else { r.next_u32() % 3 },
                );
                c.bus.write32(OC791_OBJ + 0x6088, r.next_u32());
                // The music system's vtable record: {delta, fake method}.
                let vt = OC791_OBJ + 0x7000;
                c.bus.write32(OC791_OBJ + 0x5558, vt);
                c.bus.write16(vt + 0x20, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(vt + 0x24, fakes::FAKE);
            },
        );
    }

    /// `SSXAudioSystem_GetTrackMusicGroup`: the first predicate that holds
    /// (0..5 of the 5 return 0 first), and unwinds.
    #[test]
    fn ssx_audio_system_get_track_music_group_matches() {
        check(
            ee::s07::f_0028d8a0__generated,
            ssx_audio_system_get_track_music_group,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x28d8a0, 8);
                c.hooks.miss = oc791_miss;
                oc791_install_fakes(c);
                c.pc = 0x0028_d8a0;
                c.g[A0][0] = sx(OC791_OBJ);
                let stop = seed % 6;
                c.bus.write32(OC791_MASK, ((1u32 << stop) - 1) << 3);
            },
        );
    }

    /// `SSXAudioSystem_LoadLoadingScreenBank`: the real `SndBankMan_LoadBank`
    /// over faked bank calls (already loaded or not, with unwinds).
    #[test]
    fn ssx_audio_system_load_loading_screen_bank_matches() {
        oc796_install_fakes();
        fakes::covered(
            ee::s07::f_0028f700__generated,
            ssx_audio_system_load_loading_screen_bank,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x796e, 4);
                oc796_arm(c);
                c.pc = 0x0028_f700;
                c.g[A0][0] = sx(OC796_OBJ);
                c.bus.write32(OC796_OBJ + 0x118, OC796_OBJ + 0x1000);
                c.bus.write32(OC796_OBJ + 0x1000, OC796_OBJ + 0x2000);
                c.bus
                    .write32(OC796_OBJ + 0x2000 + 0xacc, OC796_OBJ + 0x3000);
                c.bus.write32(GAME_GP_BANK_NAME, r.next_u32());
                if seed % 2 == 0 {
                    oc796_force_zero(c, 0x0028_b830);
                }
            },
        );
    }

    /// `gp + 0x534` (the loading-screen bank name pointer) with the test gp.
    pub(crate) const GAME_GP_BANK_NAME: u32 = crate::difftest::GAME_GP + 0x534;

    pub(crate) use crate::game::sound::soundsys::tests::oc807_fixture;
    pub(crate) use crate::game::sound::soundsys::tests::oc807_queue;
    pub(crate) use crate::game::sound::soundsys::tests::oc807_script;

    pub(crate) use crate::game::sound::soundsys::tests::OC807_SELF;
    pub(crate) use crate::game::sound::soundsys::tests::OC807_VT;
    pub(crate) use crate::game::sound::ssxaudio::tests::oc806_machine;

    pub(crate) const FX_AUDIO_RAND: u32 = 11;

    /// Random song: the first pick equals the previous one (second pick,
    /// then stop) or differs, 1..=7 songs, 1..=10 skips.
    #[test]
    fn ssx_audio_system_select_random_song_matches() {
        fakes::covered(
            ee::s07::f_0028d488__generated,
            ssx_audio_system_select_random_song_hand,
            240,
            &|c, seed| {
                let rand_policy = if seed % 3 == 1 { 0 } else { 9 };
                oc806_machine(
                    c,
                    seed,
                    0x806_0004,
                    8,
                    0x0028_d488,
                    &[(FX_AUDIO_RAND, rand_policy)],
                );
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x118, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x1d8, 0x1234_5678);
                c.bus.write32(this + 0x504, 1 + seed % 7);
                c.bus
                    .write32(this + 0x62a4, if seed % 3 == 2 { 1 } else { 0 });
                c.bus.write32(this + 0x508, 0xdead_beef);
            },
        );
    }

    pub(crate) const OC807_WORLD: u32 = DATA + 0x27_0000;

    pub(crate) const OC807_UI: u32 = DATA + 0x27_1000;

    pub(crate) const OC807_OBJ: u32 = DATA + 0x27_2000;

    pub(crate) const OC807_STATE: u32 = DATA + 0x27_3000;

    /// Make the fake for `addr` return `bits` in `f0`.
    pub(crate) fn oc807_script_f0(c: &mut Cpu, addr: u32, bits: u32) {
        c.bus.write32(oc807_queue(addr) + 8, bits);
    }

    /// `SSXAudioSystem_OnSongChanged(self)`: starts the song player on
    /// `self + 0x118`, and when channel 1's gain is positive and the UI state
    /// stack has a current state, asks the state found by the hash `0x4a3660`
    /// to handle message 5 through its vtable (`+0xc0` delta, `+0xc4` method).
    /// The gain (`f0`), the world and stack pointers, the state's presence and
    /// the vtable delta all vary with the seed.
    #[test]
    fn ssx_audio_system_on_song_changed_matches() {
        oc807_install_fakes();
        covered(
            ee::s07::f_0028f478__generated,
            ssx_audio_system_on_song_changed_resumable,
            240,
            &|c, seed| {
                let _ = oc807_fixture(c, seed, 0x0028_f478, 5);
                let gain = if seed % 4 == 0 {
                    0xbf80_0000
                } else {
                    0x3f80_0000
                };
                oc807_script_f0(c, 0x0028_7930, gain);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), OC807_WORLD);
                c.bus
                    .write32(OC807_WORLD + 0x84, if seed % 7 == 0 { 0 } else { OC807_UI });
                let top = match (seed / 7) % 4 {
                    0 => 0,
                    1 => 0xffff_ffe8, // self + 0x18 == 0
                    _ => OC807_OBJ,
                };
                c.bus.write32(OC807_UI + 0x48, top);
                let found = if (seed / 3) % 3 == 0 { 0 } else { OC807_STATE };
                oc807_script(c, 0x0039_f9d8, &[found]);
                c.bus.write32(OC807_STATE + 8, OC807_VT);
                c.bus.write16(OC807_VT + 0xc0, 0x10 * (seed % 3) as u16);
                c.bus.write32(OC807_VT + 0xc4, fakes::FAKE + 0x10);
                c.g[A0][0] = sx(OC807_SELF);
                c.pc = 0x0028_f478;
            },
        );
    }

    /// Random bytes at `a` (words), so loads see varied data.
    pub(crate) fn oc814_fill(c: &mut Cpu, r: &mut crate::difftest::Rng, a: u32, words: u32) {
        for i in 0..words {
            let v = r.next_u32();
            c.bus.write32(a + 4 * i, v);
        }
    }

    /// A pointer in the low half of the data area (offsets up to 0x6300 stay
    /// clear of the fakes' call log at DATA+0x100000).
    pub(crate) fn oc814_base(c: &mut Cpu, r: &mut crate::difftest::Rng) -> u32 {
        let a = DATA + (r.next_u32() & 0x7_fff0);
        oc814_fill(c, r, a, 8);
        a
    }

    #[test]
    fn snd_bank_get_buffer_size_matches() {
        check(
            ee::s07::f_0028b928__generated,
            snd_bank_get_buffer_size,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28b9_28, 0);
                let a = oc814_base(c, &mut r);
                oc814_fill(c, &mut r, a + 0x10, 2);
                c.g[A0][0] = a as u64;
                c.pc = 0x0028_b928;
            },
        );
    }

    #[test]
    fn ssx_audio_system_reset_music_state_matches() {
        check(
            ee::s07::f_0028df08__generated,
            ssx_audio_system_reset_music_state,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28df_08, 0);
                let a = oc814_base(c, &mut r);
                oc814_fill(c, &mut r, a + 0x6298, 3);
                c.g[A0][0] = a as u64;
                c.pc = 0x0028_df08;
            },
        );
    }

    pub(crate) fn oc817_leaf_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x817, 0);
        c.g[A0][0] = sx(fakes::ptr(&mut r));
        c.g[RA][0] = sx(0x0020_0000 + 0x10 * seed);
    }

    /// `SSXAudioSystem_SetCountdownValue(self, value)` stores the value at +0x6250.
    #[test]
    fn ssx_audio_system_set_countdown_value_matches() {
        check(
            ee::s07::f_0028c8c0__generated,
            oc817_ssx_audio_system_set_countdown_value,
            50,
            &|c, seed| {
                oc817_leaf_setup(c, seed);
                c.g[A1][0] = sx(0x1000 + seed * 0x77);
            },
        );
    }
}
