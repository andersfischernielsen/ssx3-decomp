//! `src/sound/bankmanager.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
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
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::lw_9 as lw_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_3;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::p as p_8;
pub(crate) use crate::game::fe::feasyncfile::p as p_9;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_4;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p as p_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_4;
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

    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_PLAY_SONG;

    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME as SND_BANK_IS_LOADED_WITH_NAME_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK as SND_LOAD_QUEUE_CANCEL_BANK_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;

    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN;

    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_RESTART_PEAK_MUSIC;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;

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

/// `void SndBank_Unlink(SndBank *self)` (0x28b730): remove `self` from the
/// doubly-linked list, keeping neighbours connected, and clear its links.
pub fn snd_bank_unlink(c: &mut Cpu) {
    let self_ = Addr(abi::arg(c, 0));
    c.cycles += 3;
    let prev = self_.off(0x18).lw(c);
    c.g[V1][0] = prev;
    c.g[V0][0] = u64::MAX;
    if prev != 0 {
        c.cycles += 6;
        let next = self_.off(0x1c).lw(c);
        c.g[A1][0] = next;
        self_.off(0xc).sw(c, u64::MAX);
        self_.off(0x10).sw(c, 0);
        self_.off(0x14).sw(c, 0);
        self_.off(0x8).sw(c, 0);
        if prev != next {
            c.cycles += 4;
            Addr(prev as u32).off(0x1c).sw(c, next);
            c.g[V1][0] = self_.off(0x1c).lw(c);
            c.g[V0][0] = self_.off(0x18).lw(c);
            Addr(c.g[V1][0] as u32).off(0x18).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 4;
            Addr(prev as u32).off(0x18).sw(c, 0);
            c.g[V0][0] = self_.off(0x18).lw(c);
            Addr(c.g[V0][0] as u32).off(0x1c).sw(c, 0);
            c.g[V1][0] = prev;
        }
        c.cycles += 2;
        self_.off(0x18).sw(c, 0);
        self_.off(0x1c).sw(c, 0);
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

/// `void SndBankMan_LoadBank(SndBankMan* self, int id, void* a2, void* a3)`
/// (0x28bc58): if the bank is not loaded yet, cancel its pending load and
/// load it into the bank table entry.
pub fn snd_bank_man_load_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x60;
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.mult(0, S3, S2, V0);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A3][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr(p_4(r32(c, S0), 0xacc)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
        callee::SND_BANK_IS_LOADED_WITH_NAME.call(c, 0x0028_bc9c)?;
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            callee::SND_LOAD_QUEUE_CANCEL_BANK.call(c, 0x0028_bcb0)?;
            c.g[A0][0] = Addr(p_4(r32(c, S0), 0xacc)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S4][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
            callee::SND_BANK_LOAD.call(c, 0x0028_bcc4)?;
        }
        c.cycles += 7;
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

/// `void SSXAudioSystem_OnSongChanged(self)` (0x28f478): play the new song,
/// and if its gain is audible, push the song title to the active music state.
pub fn ssx_audio_system_on_song_changed(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'outer: {
            'inner: {
                c.cycles += 15;
                frame.save(c, S0, 0x10);
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[RA][0] = sx(0x0028_f490);
                c.g[A0][0] = sx(at_4(c.g[S0][0], 0x118));
                callee::MUSIC_PLAYER_PLAY_SONG.call(c, 0x0028_f490)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x0028_f4a0);
                c.g[A2][0] = 0;
                callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN.call(c, 0x0028_f4a0)?;
                c.mtc1(0, 1);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = lw(c, gp_2(c, -0x848));
                if t {
                    break 'inner;
                }
                c.cycles += 3;
                c.g[V0][0] = lw(c, at_4(c.g[V0][0], 0x84));
                let t = c.g[V0][0] == 0;
                frame.restore(c, S0, 0x10);
                if t {
                    break 'outer;
                }
                c.cycles += 3;
                c.g[A0][0] = lw(c, at_4(c.g[V0][0], 0x48));
                let t = c.g[A0][0] == 0;
                c.g[S0][0] = sx(at_4(c.g[A0][0], 0x18));
                if t {
                    break 'inner;
                }
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    frame.restore(c, S0, 0x10);
                    break 'outer;
                }
                c.cycles += 9;
                c.g[A0][0] = sx(0x004a_3660);
                c.g[RA][0] = sx(0x0028_f4e0);
                call_ptr(c, 0x0031_7670, 0x0028_f4e0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0028_f4ec);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_f9d8, 0x0028_f4ec)?;
                c.g[A3][0] = c.g[V0][0];
                let t = c.g[A3][0] == 0;
                c.g[A1][0] = 5;
                if !t {
                    c.cycles += 6;
                    c.g[V1][0] = lw(c, at_4(c.g[A3][0], 8));
                    c.g[A2][0] = 0;
                    c.g[A0][0] = lh(c, at_4(c.g[V1][0], 0xc0));
                    c.g[V0][0] = lw(c, at_4(c.g[V1][0], 0xc4));
                    let target = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0028_f510);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0028_f510)?;
                }
                break 'inner;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
            break 'outer;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `SSXAudioSystem_OnMusicSetEventState` (0x28f3c8): set the bank monitor's
// event state and, when the song tag is no longer -1 and the code is one of
// 10/11/18, optionally issue the monitor's `v3(8)` call and clear `+0x530`.
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

/// `SSXAudioSystem_SelectRandomSong(self)` (0x0028d488): pick a random song
/// index in `[0, tracks)`, store it and make the music player switch to it.
pub fn ssx_audio_system_select_random_song(c: &mut Cpu) {
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
        let player = lw_5(c, this, 0x118);
        c.g[S1][0] = lw_5(c, this, 0x504);
        c.g[S3][0] = lw_5(c, this, 0x62a4);
        c.g[V1][0] = player;
        c.g[V0][0] = lw_5(c, player, 0);
        c.g[A0][0] = lw_5(c, c.g[V0][0], 0x1d8);
        call_ptr(c, 0x002a_df60, 0x0028_d4c0)?;
        c.g[V0][0] = c.g[V0][0] & 0x7fff;

        // Random fraction of the (scaled) track count.
        c.cycles += 14;
        c.mult(0, V0, V0, S1);
        c.g[V1][0] = lw_5(c, this, 0x118);
        c.divu(0, V0, S0);
        c.g[V0][0] = c.lo[0];
        sw_4(c, this, 0x508, c.g[V0][0]);
        c.g[V0][0] = lw_5(c, c.g[V1][0], 0);
        c.g[A0][0] = lw_5(c, c.g[V0][0], 0x1d8);
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
        c.g[V0][0] = lw_5(c, this, 0x508);
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
            c.g[V0][0] = lw_5(c, this, 0x508);
            if c.g[S3][0] == c.g[V0][0] {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                call_ptr(c, 0x002b_42b8, 0x0028_d580)?;
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;

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
    fn snd_bank_unlink_matches() {
        check(
            ssx3_ee::s07::f_0028b730,
            snd_bank_unlink,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_000b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                // 0: no previous; 1: prev == next; 2: prev != next.
                let (prev, next, write_next) = match seed % 3 {
                    0 => (0, DATA + 0x200, false),
                    1 => (DATA + 0x100, DATA + 0x100, false),
                    _ => (DATA + 0x100, DATA + 0x200, true),
                };
                c.bus.write32(DATA + 0x18, prev);
                c.bus.write32(DATA + 0x1c, next);
                for off in [0x8u32, 0xc, 0x10, 0x14] {
                    c.bus.write32(DATA + off, r.next_u32());
                }
                if prev != 0 {
                    c.bus.write32(prev + 0x18, r.next_u32());
                    c.bus.write32(prev + 0x1c, r.next_u32());
                }
                if write_next {
                    c.bus.write32(DATA + 0x200 + 0x18, r.next_u32());
                }
            },
        );
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
}
