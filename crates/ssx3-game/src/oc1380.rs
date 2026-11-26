//! Staging module oc1380 (wave memcard-ui-1): hand-written replacements for
//! the front-end save-flow drivers, each with its differential test against
//! the generated body. Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc1380`.

#![allow(unused_imports)]

use crate::abi::r32;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::game::fe::fememcardcallbacks::{
    memcard_fe, memcard_gp, MEMCARD_APP_GP, MEMCARD_FE_MESSAGE, MEMCARD_FE_POPUP_OPEN,
    MEMCARD_FE_SAVE_FROM_PAUSE, MEMCARD_MC_SLOT,
};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_59a0,
        mem_card_ui_write_save_data,
        "MemCardUI_WriteSaveData",
    ),
    (
        0x001d_64c0,
        mem_card_ui_apply_sound_mode,
        "MemCardUI_ApplySoundMode",
    ),
    (
        0x001d_5c28,
        mem_card_ui_read_save_chunk,
        "MemCardUI_ReadSaveChunk",
    ),
    (
        0x001d_5d78,
        mem_card_ui_write_replay_save_block,
        "MemCardUI_WriteReplaySaveBlock",
    ),
];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `MCManager_Get` (0x227f80): the application's card manager.
    pub const MC_MANAGER_GET: Callee = Callee {
        addr: 0x0022_7f80,
        f: ee::s05::MCManager_Get,
    };

    /// `MCManager_BeginReplayWrite` (0x2412a0).
    pub const MC_MANAGER_BEGIN_REPLAY_WRITE: Callee = Callee {
        addr: 0x0024_12a0,
        f: ee::s06::MCManager_BeginReplayWrite,
    };

    /// `Replay_BeginSave` (0x270e68).
    pub const REPLAY_BEGIN_SAVE: Callee = Callee {
        addr: 0x0027_0e68,
        f: ee::s07::Replay_BeginSave,
    };

    /// `BELibrary_GetInstance` (0x14dc80).
    pub const BELIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };

    /// `BELibrary_GetSaveDataSize` (0x14e048): the constant save size.
    pub const BELIBRARY_GET_SAVE_DATA_SIZE: Callee = Callee {
        addr: 0x0014_e048,
        f: ee::s01::BELibrary_GetSaveDataSize,
    };

    /// `BELibrary_WriteSaveData` (0x14e050).
    pub const BELIBRARY_WRITE_SAVE_DATA: Callee = Callee {
        addr: 0x0014_e050,
        f: ee::s01::BELibrary_WriteSaveData,
    };

    /// `MCManager_GetSaveTypeTag` (0x241d40).
    pub const MC_MANAGER_GET_SAVE_TYPE_TAG: Callee = Callee {
        addr: 0x0024_1d40,
        f: ee::s06::MCManager_GetSaveTypeTag,
    };

    /// `UStr_FromAscii_2` (0x2c2540).
    pub const USTR_FROM_ASCII_2: Callee = Callee {
        addr: 0x002c_2540,
        f: ee::s08::UStr_FromAscii_2,
    };

    /// `MCManager_GetProductCode` (0x241aa0).
    pub const MC_MANAGER_GET_PRODUCT_CODE: Callee = Callee {
        addr: 0x0024_1aa0,
        f: ee::s06::MCManager_GetProductCode,
    };

    /// `UStr_Sprintf` (0x2c26d0).
    pub const USTR_SPRINTF: Callee = Callee {
        addr: 0x002c_26d0,
        f: ee::s08::UStr_Sprintf,
    };

    /// `MCManager_WriteSaveByNameW` (0x241000).
    pub const MC_MANAGER_WRITE_SAVE_BY_NAME_W: Callee = Callee {
        addr: 0x0024_1000,
        f: ee::s06::MCManager_WriteSaveByNameW,
    };

    /// `BESaveInt_BuildProfileSave` (0x152950).
    pub const BESAVEINT_BUILD_PROFILE_SAVE: Callee = Callee {
        addr: 0x0015_2950,
        f: ee::s01::BESaveInt_BuildProfileSave,
    };

    /// `BESaveInt_GetProfileSaveSize` (0x152ba8).
    pub const BESAVEINT_GET_PROFILE_SAVE_SIZE: Callee = Callee {
        addr: 0x0015_2ba8,
        f: ee::s01::BESaveInt_GetProfileSaveSize,
    };

    /// `BELibrary_GetInterface` (0x14dd58).
    pub const BELIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };

    /// `BESaveInt_ReleaseSaveBuffer` (0x152728).
    pub const BESAVEINT_RELEASE_SAVE_BUFFER: Callee = Callee {
        addr: 0x0015_2728,
        f: ee::s01::BESaveInt_ReleaseSaveBuffer,
    };

    /// `MCManager_ReadSaveChunk` (0x241200).
    pub const MC_MANAGER_READ_SAVE_CHUNK: Callee = Callee {
        addr: 0x0024_1200,
        f: ee::s06::MCManager_ReadSaveChunk,
    };

    /// `BELibrary_ReadSaveData` (0x14e0c0).
    pub const BELIBRARY_READ_SAVE_DATA: Callee = Callee {
        addr: 0x0014_e0c0,
        f: ee::s01::BELibrary_ReadSaveData,
    };

    /// `MemCardStateBase_AllocReadBuffer` (0x1d9948).
    pub const MEMCARDSTATEBASE_ALLOC_READ_BUFFER: Callee = Callee {
        addr: 0x001d_9948,
        f: ee::s04::MemCardStateBase_AllocReadBuffer,
    };

    /// `operator_new_named` (0x317e30).
    pub const OPERATOR_NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::operator_new_named,
    };

    /// `EA_MemFree` (0x317e98).
    pub const EA_MEM_FREE: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::EA_MemFree,
    };

    /// `MCManager_GetStatusText` (0x241e18).
    pub const MC_MANAGER_GET_STATUS_TEXT: Callee = Callee {
        addr: 0x0024_1e18,
        f: ee::s06::MCManager_GetStatusText,
    };

    /// `Replay_WriteSaveBlock` (0x2710a8).
    pub const REPLAY_WRITE_SAVE_BLOCK: Callee = Callee {
        addr: 0x0027_10a8,
        f: ee::s07::Replay_WriteSaveBlock,
    };

    /// `MCManager_WriteSaveChunk` (0x241138).
    pub const MC_MANAGER_WRITE_SAVE_CHUNK: Callee = Callee {
        addr: 0x0024_1138,
        f: ee::s06::MCManager_WriteSaveChunk,
    };

    /// `SSXAudioSystem_GetInstance` (0x28b180).
    pub const SSXAUDIOSYSTEM_GET_INSTANCE: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::SSXAudioSystem_GetInstance,
    };

    /// `SSXAudioSystem_GetMusicVolumeLevel` (0x287670).
    pub const SSXAUDIOSYSTEM_GET_MUSIC_VOLUME_LEVEL: Callee = Callee {
        addr: 0x0028_7670,
        f: ee::s07::SSXAudioSystem_GetMusicVolumeLevel,
    };

    /// `SSXAudioSystem_GetSfxVolumeLevel` (0x2876a0).
    pub const SSXAUDIOSYSTEM_GET_SFX_VOLUME_LEVEL: Callee = Callee {
        addr: 0x0028_76a0,
        f: ee::s07::SSXAudioSystem_GetSfxVolumeLevel,
    };

    /// `SSXAudioSystem_GetSpeechVolumeLevel` (0x2876d0).
    pub const SSXAUDIOSYSTEM_GET_SPEECH_VOLUME_LEVEL: Callee = Callee {
        addr: 0x0028_76d0,
        f: ee::s07::SSXAudioSystem_GetSpeechVolumeLevel,
    };

    /// `SSXAudioSystem_UnloadFrontEndAudio` (0x286200).
    pub const SSXAUDIOSYSTEM_UNLOAD_FRONT_END_AUDIO: Callee = Callee {
        addr: 0x0028_6200,
        f: ee::s07::SSXAudioSystem_UnloadFrontEndAudio,
    };

    /// `SSXAudioSystem_Destroy` (0x284c28).
    pub const SSXAUDIOSYSTEM_DESTROY: Callee = Callee {
        addr: 0x0028_4c28,
        f: ee::s07::SSXAudioSystem_Destroy,
    };

    /// `SSXAudioSystem_Create` (0x284bb0).
    pub const SSXAUDIOSYSTEM_CREATE: Callee = Callee {
        addr: 0x0028_4bb0,
        f: ee::s07::SSXAudioSystem_Create,
    };

    /// `SSXAudioSystem_SetMusicVolume` (0x287410).
    pub const SSXAUDIOSYSTEM_SET_MUSIC_VOLUME: Callee = Callee {
        addr: 0x0028_7410,
        f: ee::s07::SSXAudioSystem_SetMusicVolume,
    };

    /// `SSXAudioSystem_SetSfxVolume` (0x287488).
    pub const SSXAUDIOSYSTEM_SET_SFX_VOLUME: Callee = Callee {
        addr: 0x0028_7488,
        f: ee::s07::SSXAudioSystem_SetSfxVolume,
    };

    /// `SSXAudioSystem_SetSpeechVolume` (0x287520).
    pub const SSXAUDIOSYSTEM_SET_SPEECH_VOLUME: Callee = Callee {
        addr: 0x0028_7520,
        f: ee::s07::SSXAudioSystem_SetSpeechVolume,
    };

    /// `SSXAudioSystem_SetMusicChannelEnabled` (0x287558).
    pub const SSXAUDIOSYSTEM_SET_MUSIC_CHANNEL_ENABLED: Callee = Callee {
        addr: 0x0028_7558,
        f: ee::s07::SSXAudioSystem_SetMusicChannelEnabled,
    };

    /// `SSXAudioSystem_SetSfxSubChannelsEnabled` (0x2875d0).
    pub const SSXAUDIOSYSTEM_SET_SFX_SUB_CHANNELS_ENABLED: Callee = Callee {
        addr: 0x0028_75d0,
        f: ee::s07::SSXAudioSystem_SetSfxSubChannelsEnabled,
    };

    /// `SSXAudioSystem_LoadFrontEndAudio` (0x285fb0).
    pub const SSXAUDIOSYSTEM_LOAD_FRONT_END_AUDIO: Callee = Callee {
        addr: 0x0028_5fb0,
        f: ee::s07::SSXAudioSystem_LoadFrontEndAudio,
    };
}

// ---------------------------------------------------------------------------
// Shared field offsets on the FE card state.

/// The save operation selector (`+0x1bc`): 2 starts a replay write, 1 writes
/// the replay save, anything else builds and writes the profile save.
pub(crate) const MEMCARD_UI_SAVE_OP: u32 = 0x1bc;

/// The replay-save flag (`+0x1c8`) the replay arm records its mode into.
pub(crate) const MEMCARD_UI_REPLAY_SAVING: u32 = 0x1c8;

/// The profile-save argument (`+0x1cc`) the profile arm passes to
/// `BESaveInt_BuildProfileSave`.
pub(crate) const MEMCARD_UI_PROFILE_ARG: u32 = 0x1cc;

/// The read cursor and buffer (`+0x1d0`, `+0x1d4`) the chunk reader advances.
pub(crate) const MEMCARD_UI_READ_CURSOR: u32 = 0x1d0;
pub(crate) const MEMCARD_UI_READ_BUFFER: u32 = 0x1d4;

/// The pending-read flag (`+0x200`) the chunk reader clears first.
pub(crate) const MEMCARD_UI_READ_PENDING: u32 = 0x200;

/// The read-size word (`+0x228`) `AllocReadBuffer` maintains.
pub(crate) const MEMCARD_UI_READ_SIZE: u32 = 0x228;

/// `void MemCardUI_WriteSaveData(void)` (0x1d59a0): write the pending save
/// for the card state: start a replay write in save mode 2, write the replay
/// save in mode 1, else build and write the profile save; then offer the
/// status-text popup unless a save is already pending.
pub fn mem_card_ui_write_save_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x7a0);
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        frame.save(c, S6, 0x730);
        frame.save(c, S0, 0x790);
        frame.save(c, S1, 0x780);
        frame.save(c, S2, 0x770);
        frame.save(c, S3, 0x760);
        frame.save(c, S4, 0x750);
        frame.save(c, S5, 0x740);
        frame.save_ra(c, 0x720);
        callee::MC_MANAGER_GET.call(c, 0x001d_59d0)?;
        c.g[S6][0] = c.g[V0][0];
        let app = c.g[S6][0];
        let fe = memcard_fe(c) as u64;
        c.g[A2][0] = fe;
        c.g[V0][0] = 0x2;
        c.g[A1][0] = Addr(fe as u32).off(MEMCARD_UI_SAVE_OP).lw(c);
        let mode = c.g[A1][0];
        c.g[V0][0] = 0x1;
        if mode == 2 {
            c.cycles += 4;
            c.g[A0][0] = app;
            callee::MC_MANAGER_BEGIN_REPLAY_WRITE.call(c, 0x001d_59f0)?;
            c.g[V1][0] = memcard_fe(c) as u64;
        } else {
            c.g[V0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
            if mode == 1 {
                c.cycles += 2;
                c.cycles += 48;
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lw(c);
                Addr(fe as u32).off(MEMCARD_UI_REPLAY_SAVING).sw(c, mode);
                callee::REPLAY_BEGIN_SAVE.call(c, 0x001d_5a10)?;
                // A0 still addresses the replay object; the call's result is unused.
                callee::BELIBRARY_GET_INSTANCE.call(c, 0x001d_5a18)?;
                c.g[A0][0] = c.g[V0][0];
                callee::BELIBRARY_GET_SAVE_DATA_SIZE.call(c, 0x001d_5a20)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[A0][0] = fe;
                c.g[A1][0] = c.g[S3][0];
                callee::MEMCARDSTATEBASE_ALLOC_READ_BUFFER.call(c, 0x001d_5a30)?;
                // A0 still addresses the card state; the call's result is unused here.
                callee::BELIBRARY_GET_INSTANCE.call(c, 0x001d_5a38)?;
                c.g[V1][0] = memcard_fe(c) as u64;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = Addr(fe as u32).off(MEMCARD_UI_READ_BUFFER).lw(c);
                callee::BELIBRARY_WRITE_SAVE_DATA.call(c, 0x001d_5a48)?;
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = 0x41;
                c.g[A1][0] = 0;
                c.g[A0][0] = app;
                callee::MC_MANAGER_GET_SAVE_TYPE_TAG.call(c, 0x001d_5a5c)?;
                c.g[S2][0] = sx(frame.at(c, 0x320));
                c.g[A1][0] = 0x460000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x77a8));
                c.g[A0][0] = c.g[S2][0];
                callee::USTR_FROM_ASCII_2.call(c, 0x001d_5a70)?;
                c.g[S0][0] = sx(frame.at(c, 0x120));
                c.g[A1][0] = 0x100;
                c.g[A0][0] = c.g[S0][0];
                callee::MC_MANAGER_GET_PRODUCT_CODE.call(c, 0x001d_5a80)?;
                c.g[T0][0] = Addr(app as u32).off(MEMCARD_MC_SLOT).lw(c);
                c.g[S1][0] = sx(frame.at(c, 0x90));
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[A3][0] = c.g[SP][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                callee::USTR_SPRINTF.call(c, 0x001d_5aa0)?;
                c.g[V0][0] = memcard_fe(c) as u64;
                c.g[A1][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[A0][0] = app;
                c.g[A2][0] = Addr(c.g[V0][0] as u32).off(MEMCARD_UI_READ_BUFFER).lw(c);
                callee::MC_MANAGER_WRITE_SAVE_BY_NAME_W.call(c, 0x001d_5ab8)?;
                c.g[V1][0] = memcard_fe(c) as u64;
            } else {
                c.cycles += 2;
                c.cycles += 42;
                c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
                callee::MC_MANAGER_GET.call(c, 0x001d_5ac8)?;
                c.g[S3][0] = c.g[V0][0];
                // A0 keeps the manager call's leftover, on both sides alike.
                callee::BELIBRARY_GET_INSTANCE.call(c, 0x001d_5ad0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x5;
                callee::BELIBRARY_GET_INTERFACE.call(c, 0x001d_5adc)?;
                c.g[V1][0] = memcard_fe(c) as u64;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = Addr(c.g[V1][0] as u32).off(MEMCARD_UI_PROFILE_ARG).lw(c);
                callee::BESAVEINT_BUILD_PROFILE_SAVE.call(c, 0x001d_5aec)?;
                c.g[S5][0] = c.g[V0][0];
                callee::BESAVEINT_GET_PROFILE_SAVE_SIZE.call(c, 0x001d_5af4)?;
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = 0x41;
                c.g[S4][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.g[S3][0];
                callee::MC_MANAGER_GET_SAVE_TYPE_TAG.call(c, 0x001d_5b0c)?;
                c.g[S2][0] = sx(frame.at(c, 0x520));
                c.g[A1][0] = 0x460000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x77a8));
                c.g[A0][0] = c.g[S2][0];
                callee::USTR_FROM_ASCII_2.call(c, 0x001d_5b20)?;
                c.g[S0][0] = sx(frame.at(c, 0x120));
                c.g[A1][0] = 0x100;
                c.g[A0][0] = c.g[S0][0];
                callee::MC_MANAGER_GET_PRODUCT_CODE.call(c, 0x001d_5b30)?;
                c.g[T0][0] = Addr(c.g[S3][0] as u32).off(MEMCARD_MC_SLOT).lw(c);
                c.g[S1][0] = sx(frame.at(c, 0x90));
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[A3][0] = c.g[SP][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                callee::USTR_SPRINTF.call(c, 0x001d_5b50)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S5][0];
                c.g[A3][0] = c.g[S4][0];
                callee::MC_MANAGER_WRITE_SAVE_BY_NAME_W.call(c, 0x001d_5b64)?;
                c.g[V1][0] = memcard_fe(c) as u64;
            }
        }
        // The shared status-text tail: offer the popup unless a save is
        // already pending, freeing the status buffer afterwards.
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(MEMCARD_FE_POPUP_OPEN).lw(c);
        frame.restore(c, S0, 0x790);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[V1][0] as u32)
                .off(MEMCARD_FE_SAVE_FROM_PAUSE)
                .lw(c);
            frame.restore(c, S1, 0x780);
            if c.g[V0][0] == 0 {
                c.cycles += 29;
                c.g[V0][0] = 0x16;
                c.g[A1][0] = 0x460000;
                Addr(c.g[V1][0] as u32)
                    .off(MEMCARD_FE_MESSAGE)
                    .sw(c, c.g[V0][0]);
                c.g[A3][0] = 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1d60));
                c.g[A0][0] = 0x640;
                c.g[A2][0] = 0x100;
                callee::OPERATOR_NEW_NAMED.call(c, 0x001d_5ba0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = app;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x320;
                callee::MC_MANAGER_GET_STATUS_TEXT.call(c, 0x001d_5bb4)?;
                c.g[T3][0] = memcard_fe(c) as u64;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x1;
                c.g[A3][0] = 0x1;
                c.g[V1][0] = Addr(c.g[T3][0] as u32).off(8).lw(c);
                c.g[T0][0] = 0;
                c.g[T1][0] = 0;
                c.g[T2][0] = 0x1;
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x120).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x124).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001d_5be4)?;
                c.g[V1][0] = memcard_fe(c) as u64;
                c.g[V0][0] = 0x1;
                let buf = c.g[S0][0];
                Addr(c.g[V1][0] as u32).off(MEMCARD_FE_POPUP_OPEN).sw(c, 1);
                if buf != 0 {
                    c.cycles += 2;
                    c.g[A0][0] = buf;
                    callee::EA_MEM_FREE.call(c, 0x001d_5bfc)?;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x790);
                c.cycles += 1;
                frame.restore(c, S1, 0x780);
            }
        } else {
            c.cycles += 1;
            frame.restore(c, S1, 0x780);
        }
        c.cycles += 8;
        frame.restore(c, S2, 0x770);
        frame.restore(c, S3, 0x760);
        frame.restore(c, S4, 0x750);
        frame.restore(c, S5, 0x740);
        frame.restore(c, S6, 0x730);
        frame.restore_ra(c, 0x720);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_ApplySoundMode(unsigned int mode)` (0x1d64c0): apply the
/// audio language for the sound mode (1 keeps 2, 2 keeps 0, 3 keeps 3,
/// anything else keeps 1); when the language holder takes the new language
/// (its check returns 1), rebuild the audio system with the saved music, sfx
/// and speech levels.
pub fn mem_card_ui_apply_sound_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x1;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x60);
        frame.save_fpr(c, 21, 0x58);
        let mode = c.g[A0][0];
        frame.save_fpr(c, 20, 0x50);
        if mode == 1 {
            c.cycles += 2;
            c.g[S3][0] = 0x2;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x2;
            if mode == 0 {
                c.cycles += 1;
                c.g[S3][0] = 0x1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x3;
                if mode == 2 {
                    c.cycles += 2;
                    c.g[S3][0] = 0;
                } else {
                    c.cycles += 2;
                    c.g[S3][0] = 0x1;
                    if mode == 3 {
                        c.cycles += 2;
                        c.g[S3][0] = 0x3;
                    } else {
                        c.cycles += 2;
                        c.g[S3][0] = 0x1;
                    }
                }
            }
        }
        c.cycles += 14;
        callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_6530)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x118).lw(c);
        c.g[A1][0] = c.g[S3][0];
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[A2][0] = Addr(c.g[V0][0] as u32).off(0x1d8).lw(c);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_6554)?;
        c.g[V1][0] = 0x1;
        frame.restore(c, S0, 0x40);
        // The holder took the language: rebuild the audio system around it.
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 74;
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_6568)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SSXAUDIOSYSTEM_GET_MUSIC_VOLUME_LEVEL.call(c, 0x001d_6570)?;
            c.mtc1(2, 20);
            c.cvt_s_w(20, 20);
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_6580)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SSXAUDIOSYSTEM_GET_SFX_VOLUME_LEVEL.call(c, 0x001d_6588)?;
            c.mtc1(2, 21);
            c.cvt_s_w(21, 21);
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_6598)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SSXAUDIOSYSTEM_GET_SPEECH_VOLUME_LEVEL.call(c, 0x001d_65a0)?;
            c.mtc1(2, 22);
            c.cvt_s_w(22, 22);
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_65b0)?;
            c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x62b8).lw(c);
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_65b8)?;
            c.g[S1][0] = Addr(c.g[V0][0] as u32).off(0x62b4).lw(c);
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_65c0)?;
            c.g[S2][0] = Addr(c.g[V0][0] as u32).off(0x534).lw(c);
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_65c8)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SSXAUDIOSYSTEM_UNLOAD_FRONT_END_AUDIO.call(c, 0x001d_65d0)?;
            callee::SSXAUDIOSYSTEM_DESTROY.call(c, 0x001d_65d8)?;
            c.g[A0][0] = c.g[S3][0];
            callee::SSXAUDIOSYSTEM_CREATE.call(c, 0x001d_65e0)?;
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_65e8)?;
            c.cvt_w_s(0, 20);
            c.mfc1(5, 0);
            c.g[A0][0] = c.g[V0][0];
            callee::SSXAUDIOSYSTEM_SET_MUSIC_VOLUME.call(c, 0x001d_65f8)?;
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_6600)?;
            c.cvt_w_s(0, 21);
            c.mfc1(5, 0);
            c.g[A0][0] = c.g[V0][0];
            callee::SSXAUDIOSYSTEM_SET_SFX_VOLUME.call(c, 0x001d_6610)?;
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_6618)?;
            c.cvt_w_s(0, 22);
            c.mfc1(5, 0);
            c.g[A0][0] = c.g[V0][0];
            callee::SSXAUDIOSYSTEM_SET_SPEECH_VOLUME.call(c, 0x001d_6628)?;
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_6630)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            callee::SSXAUDIOSYSTEM_SET_MUSIC_CHANNEL_ENABLED.call(c, 0x001d_663c)?;
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_6644)?;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[V0][0];
            callee::SSXAUDIOSYSTEM_SET_SFX_SUB_CHANNELS_ENABLED.call(c, 0x001d_6650)?;
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_6658)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SSXAUDIOSYSTEM_LOAD_FRONT_END_AUDIO.call(c, 0x001d_6660)?;
            callee::SSXAUDIOSYSTEM_GET_INSTANCE.call(c, 0x001d_6668)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x5558).lw(c);
            c.g[A1][0] = c.g[S2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
            c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001d_6684)?;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x60);
        frame.restore_fpr(c, 21, 0x58);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `int MemCardUI_ReadSaveChunk(int size)` (0x1d5c28): when the card state's
/// save mode is 1, validate the pending read buffer's CRC and read a save
/// chunk of `size` bytes (clamped to 0x4000) into it, advancing the cursor;
/// 0 on a CRC mismatch, 1 otherwise.
pub fn mem_card_ui_read_save_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x1;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = memcard_fe(c) as u64;
        frame.save(c, S2, 0x30);
        c.g[S2][0] = Addr(c.g[S0][0] as u32).off(MEMCARD_UI_SAVE_OP).lw(c);
        let mode = c.g[S2][0];
        c.g[S1][0] = c.g[A0][0];
        if mode == 1 {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(MEMCARD_UI_READ_PENDING).lw(c);
            // The size block below runs unless the CRC mismatches.
            let mut chunk = true;
            if c.g[V0][0] != 0 {
                c.cycles += 29;
                Addr(c.g[S0][0] as u32)
                    .off(MEMCARD_UI_READ_PENDING)
                    .sw(c, 0);
                c.g[S3][0] = Addr(c.g[S0][0] as u32).off(MEMCARD_UI_READ_CURSOR).lw(c);
                callee::BELIBRARY_GET_INSTANCE.call(c, 0x001d_5c6c)?;
                c.g[A0][0] = c.g[V0][0];
                callee::BELIBRARY_GET_SAVE_DATA_SIZE.call(c, 0x001d_5c74)?;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(MEMCARD_UI_READ_CURSOR).lw(c);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                c.g[A2][0] = 0xfbea;
                call_ptr(c, 0x003e_62d0, 0x001d_5c84)?;
                c.g[S0][0] = c.g[V0][0];
                callee::BELIBRARY_GET_INSTANCE.call(c, 0x001d_5c8c)?;
                c.g[A0][0] = c.g[V0][0];
                callee::BELIBRARY_GET_SAVE_DATA_SIZE.call(c, 0x001d_5c94)?;
                c.g[A0][0] = memcard_fe(c) as u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                c.g[V1][0] = Addr(c.g[A0][0] as u32).off(MEMCARD_UI_READ_CURSOR).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                Addr(c.g[A0][0] as u32)
                    .off(MEMCARD_UI_READ_CURSOR)
                    .sw(c, c.g[V1][0]);
                c.sw(frame.at(c, 0), 0u64);
                c.lwl(2, (c.g[V1][0] as u32).wrapping_add(0x3));
                c.lwr(2, c.g[V1][0] as u32);
                c.swl(2, (c.g[SP][0] as u32).wrapping_add(0x3));
                c.swr(2, c.g[SP][0] as u32);
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(MEMCARD_UI_READ_CURSOR).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                Addr(c.g[A0][0] as u32)
                    .off(MEMCARD_UI_READ_CURSOR)
                    .sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr(c.g[SP][0] as u32).lw(c);
                if c.g[S0][0] == c.g[V1][0] {
                    c.cycles += 5;
                    c.sw(memcard_gp(c, 0x1738u32.wrapping_neg()), c.g[S2][0]);
                    callee::BELIBRARY_GET_INSTANCE.call(c, 0x001d_5d10)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S3][0];
                    callee::BELIBRARY_READ_SAVE_DATA.call(c, 0x001d_5d1c)?;
                } else {
                    c.cycles += 13;
                    callee::BELIBRARY_GET_INSTANCE.call(c, 0x001d_5cdc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x5;
                    callee::BELIBRARY_GET_INTERFACE.call(c, 0x001d_5ce8)?;
                    c.g[A0][0] = c.g[V0][0];
                    callee::BESAVEINT_RELEASE_SAVE_BUFFER.call(c, 0x001d_5cf0)?;
                    c.g[V1][0] = memcard_fe(c) as u64;
                    c.g[V0][0] = 0;
                    Addr(c.g[V1][0] as u32).off(MEMCARD_UI_READ_SIZE).sw(c, 0);
                    Addr(c.g[V1][0] as u32).off(MEMCARD_UI_READ_BUFFER).sw(c, 0);
                    Addr(c.g[V1][0] as u32).off(MEMCARD_UI_READ_CURSOR).sw(c, 0);
                    chunk = false;
                }
            }
            if chunk {
                c.cycles += 2;
                c.g[V1][0] = ((c.g[S1][0] as i64) < 16385) as u64;
                if (c.g[S1][0] as i64) > 0 {
                    c.cycles += 13;
                    c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c) as u64;
                    c.g[V0][0] = 0x4000;
                    if c.g[V1][0] == 0 {
                        c.g[S1][0] = c.g[V0][0];
                    }
                    callee::MC_MANAGER_GET.call(c, 0x001d_5d34)?;
                    c.g[V1][0] = memcard_fe(c) as u64;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A1][0] = Addr(c.g[V1][0] as u32).off(MEMCARD_UI_READ_CURSOR).lw(c);
                    callee::MC_MANAGER_READ_SAVE_CHUNK.call(c, 0x001d_5d48)?;
                    c.g[V1][0] = memcard_fe(c) as u64;
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(MEMCARD_UI_READ_CURSOR).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    Addr(c.g[V1][0] as u32)
                        .off(MEMCARD_UI_READ_CURSOR)
                        .sw(c, c.g[V0][0]);
                }
                c.cycles += 1;
                c.g[V0][0] = 0x1;
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int MemCardUI_WriteReplaySaveBlock(void)` (0x1d5d78): in save mode 1,
/// write one replay save block through the replay writer into the card
/// manager's current chunk; 1 when the block writer ran, 0 otherwise.
pub fn mem_card_ui_write_replay_save_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = memcard_fe(c) as u64;
        frame.save(c, S0, 0x20);
        c.g[A1][0] = 0x1;
        frame.save_ra(c, 0x10);
        c.sw(frame.at(c, 4), c.g[A0][0]);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(MEMCARD_UI_SAVE_OP).lw(c);
        let mode = c.g[V1][0];
        c.g[V0][0] = 0;
        if mode == 1 {
            c.cycles += 11;
            c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c) as u64;
            callee::MC_MANAGER_GET.call(c, 0x001d_5da4)?;
            c.g[V1][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c) as u64;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = c.g[SP][0] | 0x4;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x28).lw(c);
            callee::REPLAY_WRITE_SAVE_BLOCK.call(c, 0x001d_5dc0)?;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = Addr(c.g[SP][0] as u32).lw(c);
                callee::MC_MANAGER_WRITE_SAVE_CHUNK.call(c, 0x001d_5dd4)?;
                c.g[V0][0] = 0x1;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, check_paths, fill, require_paths, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the callee fakes below: they log the call and return the
    /// fixture's word only when this magic word is in the machine's RAM.
    pub(crate) const OC1380_GATE: u32 = DATA + 0xf_e000;
    pub(crate) const OC1380_MAGIC: u32 = 0x0c38_1380;

    /// Fixture cells: forced `v0` results, one per faked callee.
    pub(crate) const R_MC_MANAGER_GET: u32 = DATA + 0xf_e100;
    pub(crate) const R_REPLAY_BEGIN_SAVE: u32 = DATA + 0xf_e104;
    pub(crate) const R_GET_INSTANCE: u32 = DATA + 0xf_e108;
    pub(crate) const R_GET_SAVE_DATA_SIZE: u32 = DATA + 0xf_e10c;
    pub(crate) const R_WRITE_SAVE_DATA: u32 = DATA + 0xf_e110;
    pub(crate) const R_BUILD_PROFILE_SAVE: u32 = DATA + 0xf_e114;
    pub(crate) const R_GET_PROFILE_SAVE_SIZE: u32 = DATA + 0xf_e118;
    pub(crate) const R_GET_INTERFACE: u32 = DATA + 0xf_e11c;
    pub(crate) const R_OPERATOR_NEW: u32 = DATA + 0xf_e120;
    pub(crate) const R_REPLAY_WRITE_SAVE_BLOCK: u32 = DATA + 0xf_e124;
    pub(crate) const R_AUDIO_INSTANCE: u32 = DATA + 0xf_e128;
    pub(crate) const R_MUSIC_VOLUME: u32 = DATA + 0xf_e12c;
    pub(crate) const R_SFX_VOLUME: u32 = DATA + 0xf_e130;
    pub(crate) const R_SPEECH_VOLUME: u32 = DATA + 0xf_e134;

    /// A gated fake returning the fixture's word in `v0`.
    macro_rules! gated_cell {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr, $cell:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1380_GATE) != OC1380_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
                c.g[V0][0] = c.bus.read32($cell) as u64;
            }
        };
    }

    gated_cell!(
        oc1380_fake_mc_manager_get,
        OC1380_PREV_MC_MANAGER_GET,
        ee::s05::f_00227f80__generated,
        0x0022_7f80,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_begin_replay_write,
        OC1380_PREV_BEGIN_REPLAY_WRITE,
        ee::s06::f_002412a0__generated,
        0x0024_12a0,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_replay_begin_save,
        OC1380_PREV_REPLAY_BEGIN_SAVE,
        ee::s07::f_00270e68__generated,
        0x0027_0e68,
        R_REPLAY_BEGIN_SAVE
    );
    gated_cell!(
        oc1380_fake_get_instance,
        OC1380_PREV_GET_INSTANCE,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80,
        R_GET_INSTANCE
    );
    gated_cell!(
        oc1380_fake_get_save_data_size,
        OC1380_PREV_GET_SAVE_DATA_SIZE,
        ee::s01::f_0014e048__generated,
        0x0014_e048,
        R_GET_SAVE_DATA_SIZE
    );
    gated_cell!(
        oc1380_fake_write_save_data,
        OC1380_PREV_WRITE_SAVE_DATA,
        ee::s01::f_0014e050__generated,
        0x0014_e050,
        R_WRITE_SAVE_DATA
    );
    gated_cell!(
        oc1380_fake_get_save_type_tag,
        OC1380_PREV_GET_SAVE_TYPE_TAG,
        ee::s06::f_00241d40__generated,
        0x0024_1d40,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_from_ascii,
        OC1380_PREV_FROM_ASCII,
        ee::s08::f_002c2540__generated,
        0x002c_2540,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_get_product_code,
        OC1380_PREV_GET_PRODUCT_CODE,
        ee::s06::f_00241aa0__generated,
        0x0024_1aa0,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_sprintf,
        OC1380_PREV_SPRINTF,
        ee::s08::f_002c26d0__generated,
        0x002c_26d0,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_write_save_by_name,
        OC1380_PREV_WRITE_SAVE_BY_NAME,
        ee::s06::f_00241000__generated,
        0x0024_1000,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_build_profile_save,
        OC1380_PREV_BUILD_PROFILE_SAVE,
        ee::s01::f_00152950__generated,
        0x0015_2950,
        R_BUILD_PROFILE_SAVE
    );
    gated_cell!(
        oc1380_fake_get_profile_save_size,
        OC1380_PREV_GET_PROFILE_SAVE_SIZE,
        ee::s01::f_00152ba8__generated,
        0x0015_2ba8,
        R_GET_PROFILE_SAVE_SIZE
    );
    gated_cell!(
        oc1380_fake_get_interface,
        OC1380_PREV_GET_INTERFACE,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58,
        R_GET_INTERFACE
    );
    gated_cell!(
        oc1380_fake_release_save_buffer,
        OC1380_PREV_RELEASE_SAVE_BUFFER,
        ee::s01::f_00152728__generated,
        0x0015_2728,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_read_save_chunk,
        OC1380_PREV_READ_SAVE_CHUNK,
        ee::s06::f_00241200__generated,
        0x0024_1200,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_read_save_data,
        OC1380_PREV_READ_SAVE_DATA,
        ee::s01::f_0014e0c0__generated,
        0x0014_e0c0,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_alloc_read_buffer,
        OC1380_PREV_ALLOC_READ_BUFFER,
        ee::s04::f_001d9948__generated,
        0x001d_9948,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_operator_new,
        OC1380_PREV_OPERATOR_NEW,
        ee::s10::f_00317e30__generated,
        0x0031_7e30,
        R_OPERATOR_NEW
    );
    gated_cell!(
        oc1380_fake_mem_free,
        OC1380_PREV_MEM_FREE,
        ee::s10::f_00317e98__generated,
        0x0031_7e98,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_get_status_text,
        OC1380_PREV_GET_STATUS_TEXT,
        ee::s06::f_00241e18__generated,
        0x0024_1e18,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_write_save_chunk,
        OC1380_PREV_WRITE_SAVE_CHUNK,
        ee::s06::f_00241138__generated,
        0x0024_1138,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_audio_get_instance,
        OC1380_PREV_AUDIO_GET_INSTANCE,
        ee::s07::f_0028b180__generated,
        0x0028_b180,
        R_AUDIO_INSTANCE
    );
    gated_cell!(
        oc1380_fake_get_music_volume,
        OC1380_PREV_GET_MUSIC_VOLUME,
        ee::s07::f_00287670__generated,
        0x0028_7670,
        R_MUSIC_VOLUME
    );
    gated_cell!(
        oc1380_fake_get_sfx_volume,
        OC1380_PREV_GET_SFX_VOLUME,
        ee::s07::f_002876a0__generated,
        0x0028_76a0,
        R_SFX_VOLUME
    );
    gated_cell!(
        oc1380_fake_get_speech_volume,
        OC1380_PREV_GET_SPEECH_VOLUME,
        ee::s07::f_002876d0__generated,
        0x0028_76d0,
        R_SPEECH_VOLUME
    );
    gated_cell!(
        oc1380_fake_unload_front_end_audio,
        OC1380_PREV_UNLOAD_FRONT_END_AUDIO,
        ee::s07::f_00286200__generated,
        0x0028_6200,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_audio_destroy,
        OC1380_PREV_AUDIO_DESTROY,
        ee::s07::f_00284c28__generated,
        0x0028_4c28,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_audio_create,
        OC1380_PREV_AUDIO_CREATE,
        ee::s07::f_00284bb0__generated,
        0x0028_4bb0,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_set_music_volume,
        OC1380_PREV_SET_MUSIC_VOLUME,
        ee::s07::f_00287410__generated,
        0x0028_7410,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_set_sfx_volume,
        OC1380_PREV_SET_SFX_VOLUME,
        ee::s07::f_00287488__generated,
        0x0028_7488,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_set_speech_volume,
        OC1380_PREV_SET_SPEECH_VOLUME,
        ee::s07::f_00287520__generated,
        0x0028_7520,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_set_music_channel,
        OC1380_PREV_SET_MUSIC_CHANNEL,
        ee::s07::f_00287558__generated,
        0x0028_7558,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_set_sfx_sub_channels,
        OC1380_PREV_SET_SFX_SUB_CHANNELS,
        ee::s07::f_002875d0__generated,
        0x0028_75d0,
        R_MC_MANAGER_GET
    );
    gated_cell!(
        oc1380_fake_load_front_end_audio,
        OC1380_PREV_LOAD_FRONT_END_AUDIO,
        ee::s07::f_00285fb0__generated,
        0x0028_5fb0,
        R_MC_MANAGER_GET
    );

    /// `Replay_WriteSaveBlock` (0x2710a8): log the call, write the block's
    /// output words through the caller's stack slots, return the fixture's word.
    static OC1380_PREV_WRITE_SAVE_BLOCK: AtomicUsize = AtomicUsize::new(0);
    fn oc1380_fake_replay_write_save_block(c: &mut Cpu) {
        if c.bus.read32(OC1380_GATE) != OC1380_MAGIC {
            return fakes::fall_back(
                c,
                &OC1380_PREV_WRITE_SAVE_BLOCK,
                ee::s07::f_002710a8__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0027_10a8, ra);
        c.bus.write32(c.g[A1][0] as u32, 0x0b1e_0001);
        c.bus.write32(c.g[A2][0] as u32, 0x0b1e_0002);
        c.g[V0][0] = c.bus.read32(R_REPLAY_WRITE_SAVE_BLOCK) as u64;
    }

    /// Hook every callee slot this module's tests fake (once per process).
    fn install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00227F80),
            oc1380_fake_mc_manager_get,
            &OC1380_PREV_MC_MANAGER_GET,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002412A0),
            oc1380_fake_begin_replay_write,
            &OC1380_PREV_BEGIN_REPLAY_WRITE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00270E68),
            oc1380_fake_replay_begin_save,
            &OC1380_PREV_REPLAY_BEGIN_SAVE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc1380_fake_get_instance,
            &OC1380_PREV_GET_INSTANCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014E048),
            oc1380_fake_get_save_data_size,
            &OC1380_PREV_GET_SAVE_DATA_SIZE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014E050),
            oc1380_fake_write_save_data,
            &OC1380_PREV_WRITE_SAVE_DATA,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241D40),
            oc1380_fake_get_save_type_tag,
            &OC1380_PREV_GET_SAVE_TYPE_TAG,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2540),
            oc1380_fake_from_ascii,
            &OC1380_PREV_FROM_ASCII,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241AA0),
            oc1380_fake_get_product_code,
            &OC1380_PREV_GET_PRODUCT_CODE,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C26D0),
            oc1380_fake_sprintf,
            &OC1380_PREV_SPRINTF,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241000),
            oc1380_fake_write_save_by_name,
            &OC1380_PREV_WRITE_SAVE_BY_NAME,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00152950),
            oc1380_fake_build_profile_save,
            &OC1380_PREV_BUILD_PROFILE_SAVE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00152BA8),
            oc1380_fake_get_profile_save_size,
            &OC1380_PREV_GET_PROFILE_SAVE_SIZE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc1380_fake_get_interface,
            &OC1380_PREV_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00152728),
            oc1380_fake_release_save_buffer,
            &OC1380_PREV_RELEASE_SAVE_BUFFER,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241200),
            oc1380_fake_read_save_chunk,
            &OC1380_PREV_READ_SAVE_CHUNK,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014E0C0),
            oc1380_fake_read_save_data,
            &OC1380_PREV_READ_SAVE_DATA,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D9948),
            oc1380_fake_alloc_read_buffer,
            &OC1380_PREV_ALLOC_READ_BUFFER,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1380_fake_operator_new,
            &OC1380_PREV_OPERATOR_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1380_fake_mem_free,
            &OC1380_PREV_MEM_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241E18),
            oc1380_fake_get_status_text,
            &OC1380_PREV_GET_STATUS_TEXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241138),
            oc1380_fake_write_save_chunk,
            &OC1380_PREV_WRITE_SAVE_CHUNK,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002710A8),
            oc1380_fake_replay_write_save_block,
            &OC1380_PREV_WRITE_SAVE_BLOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            oc1380_fake_audio_get_instance,
            &OC1380_PREV_AUDIO_GET_INSTANCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287670),
            oc1380_fake_get_music_volume,
            &OC1380_PREV_GET_MUSIC_VOLUME,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002876A0),
            oc1380_fake_get_sfx_volume,
            &OC1380_PREV_GET_SFX_VOLUME,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002876D0),
            oc1380_fake_get_speech_volume,
            &OC1380_PREV_GET_SPEECH_VOLUME,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00286200),
            oc1380_fake_unload_front_end_audio,
            &OC1380_PREV_UNLOAD_FRONT_END_AUDIO,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00284C28),
            oc1380_fake_audio_destroy,
            &OC1380_PREV_AUDIO_DESTROY,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00284BB0),
            oc1380_fake_audio_create,
            &OC1380_PREV_AUDIO_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287410),
            oc1380_fake_set_music_volume,
            &OC1380_PREV_SET_MUSIC_VOLUME,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287488),
            oc1380_fake_set_sfx_volume,
            &OC1380_PREV_SET_SFX_VOLUME,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287520),
            oc1380_fake_set_speech_volume,
            &OC1380_PREV_SET_SPEECH_VOLUME,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287558),
            oc1380_fake_set_music_channel,
            &OC1380_PREV_SET_MUSIC_CHANNEL,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002875D0),
            oc1380_fake_set_sfx_sub_channels,
            &OC1380_PREV_SET_SFX_SUB_CHANNELS,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00285FB0),
            oc1380_fake_load_front_end_audio,
            &OC1380_PREV_LOAD_FRONT_END_AUDIO,
        );
    }

    /// Fixture areas for the save-flow tests.
    pub(crate) const OC1380_FE: u32 = DATA + 0x100;
    pub(crate) const OC1380_WORLD: u32 = DATA + 0x2000;
    pub(crate) const OC1380_REPLAY_OBJ: u32 = DATA + 0x2100;
    pub(crate) const OC1380_REPLAY: u32 = DATA + 0x2200;
    pub(crate) const OC1380_APP: u32 = DATA + 0x3000;
    pub(crate) const OC1380_VT: u32 = DATA + 0x5000;
    pub(crate) const OC1380_RBUF: u32 = DATA + 0x6000;
    pub(crate) const OC1380_PBUF: u32 = DATA + 0x7000;
    pub(crate) const OC1380_BBUF: u32 = DATA + 0x4000;
    pub(crate) const OC1380_AUDIO: u32 = DATA + 0x8000;

    /// Point `gp` at the card state and the world, with scratch globals.
    fn oc1380_fe_world(c: &mut Cpu, r: &mut Rng, fe: u32, world: u32) {
        c.bus.write32(GAME_GP.wrapping_sub(0x10c8), fe);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(GAME_GP.wrapping_sub(0x1738), 0x6d75_6d70);
        fill(c, r, fe, 0x100);
        fill(c, r, world, 0x40);
        c.bus.write32(world + 0x84, OC1380_REPLAY_OBJ);
        c.bus.write32(OC1380_REPLAY_OBJ + 0x28, OC1380_REPLAY);
        fill(c, r, OC1380_REPLAY_OBJ, 4);
        fill(c, r, OC1380_REPLAY, 4);
    }

    /// The popup vtable the status tail calls: slot `0x120` is `{0, FAKE}`
    /// (its result is ignored by every caller here).
    fn oc1380_popup_vtable(c: &mut Cpu, fe: u32) {
        c.bus.write32(fe + 8, OC1380_VT);
        c.bus.write16(OC1380_VT + 0x120, 0);
        c.bus.write32(OC1380_VT + 0x124, fakes::FAKE);
    }

    // -----------------------------------------------------------------------
    // 0x1d59a0 MemCardUI_WriteSaveData.

    #[test]
    fn mem_card_ui_write_save_data_matches() {
        check_paths(
            ee::s04::f_001d59a0__generated,
            mem_card_ui_write_save_data,
            64,
            &[
                43, 45, 47, 49, 51, 53, 91, 103, 104, 114, 127, 129, 135, 145, 147, 151, 153, 156,
                157, 163, 165, 181, 195, 196, 206, 211, 225,
            ],
            &|c, seed| {
                install_fakes();
                let mut r = machine(c, seed, 0x1d59_a0, 0);
                let mode = [2u64, 1, 0, 3][seed as usize % 4];
                oc1380_fe_world(c, &mut r, OC1380_FE, OC1380_WORLD);
                fill(c, &mut r, OC1380_RBUF, 0x20);
                fill(c, &mut r, OC1380_PBUF, 0x20);
                fill(c, &mut r, OC1380_BBUF, 0x20);
                c.bus.write32(OC1380_FE + MEMCARD_UI_SAVE_OP, mode as u32);
                c.bus
                    .write32(OC1380_FE + MEMCARD_UI_READ_BUFFER, OC1380_RBUF);
                let no_pending = seed / 4 % 2 == 0;
                c.bus.write32(
                    OC1380_FE + MEMCARD_FE_POPUP_OPEN,
                    if no_pending { 0 } else { 0x21 },
                );
                c.bus.write32(
                    OC1380_FE + MEMCARD_FE_SAVE_FROM_PAUSE,
                    if no_pending && seed / 8 % 2 == 0 {
                        0
                    } else {
                        0x22
                    },
                );
                oc1380_popup_vtable(c, OC1380_FE);
                c.bus.write32(OC1380_APP + MEMCARD_MC_SLOT, 0x10 + seed);
                c.bus.write32(OC1380_GATE, OC1380_MAGIC);
                c.bus.write32(R_MC_MANAGER_GET, OC1380_APP);
                c.bus
                    .write32(R_REPLAY_BEGIN_SAVE, [0, 0x31][seed as usize / 16 % 2]);
                c.bus.write32(R_GET_INSTANCE, OC1380_APP + 0x100);
                c.bus.write32(R_GET_SAVE_DATA_SIZE, 0x14);
                c.bus.write32(R_WRITE_SAVE_DATA, OC1380_PBUF);
                c.bus.write32(R_BUILD_PROFILE_SAVE, 0x50 + seed);
                c.bus.write32(R_GET_PROFILE_SAVE_SIZE, 0x60 + seed);
                c.bus.write32(R_GET_INTERFACE, OC1380_APP + 0x200);
                c.bus.write32(
                    R_OPERATOR_NEW,
                    if no_pending && seed / 8 % 2 == 0 && seed / 32 % 2 == 0 {
                        0
                    } else {
                        OC1380_BBUF
                    },
                );
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x1d5c28 MemCardUI_ReadSaveChunk.

    #[test]
    fn mem_card_ui_read_save_chunk_matches() {
        check_paths(
            ee::s04::f_001d5c28__generated,
            mem_card_ui_read_save_chunk,
            64,
            &[
                18, 24, 46, 54, 104, 105, 107, 118, 120, 122, 126, 133, 135, 201, 207, 209, 217,
                220,
            ],
            &|c, seed| {
                install_fakes();
                let mut r = machine(c, seed, 0x1d5c_28, 0);
                let mode = [1u64, 1, 0, 2][seed as usize % 4];
                oc1380_fe_world(c, &mut r, OC1380_FE, OC1380_WORLD);
                c.bus.write32(OC1380_FE + MEMCARD_UI_SAVE_OP, mode as u32);
                // The read cursor and its first words; the CRC fixture below
                // overwrites the stored word for its own arm.
                let cur = OC1380_RBUF;
                fill(c, &mut r, cur, 0x20);
                c.bus.write32(OC1380_FE + MEMCARD_UI_READ_CURSOR, cur);
                let pending = seed / 4 % 3;
                c.bus.write32(
                    OC1380_FE + MEMCARD_UI_READ_PENDING,
                    [0x44, 0, 0x45][pending as usize % 3],
                );
                let small_crc = seed / 16 % 2 == 0;
                let size = if small_crc { 4u32 } else { 12 };
                c.bus.write32(
                    cur,
                    if seed / 32 % 2 == 0 {
                        0xfbea
                    } else {
                        0x1234_5678
                    },
                );
                // A0 is the chunk size the tail clamps: negative, zero, small,
                // exactly 0x4000, over.
                c.g[A0][0] = [(-8i64) as u64, 0, 100, 0x4000, 0x5000][seed as usize % 5];
                c.g[A1][0] = 0x99aa_bbcc + seed as u64;
                c.bus.write32(OC1380_GATE, OC1380_MAGIC);
                c.bus.write32(R_GET_INSTANCE, OC1380_APP + 0x100);
                c.bus.write32(R_GET_SAVE_DATA_SIZE, size);
                c.bus.write32(R_GET_INTERFACE, OC1380_APP + 0x200);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x1d5d78 MemCardUI_WriteReplaySaveBlock.

    #[test]
    fn mem_card_ui_write_replay_save_block_matches() {
        check_paths(
            ee::s04::f_001d5d78__generated,
            mem_card_ui_write_replay_save_block,
            24,
            &[13, 32, 39, 40, 41, 42],
            &|c, seed| {
                install_fakes();
                let mut r = machine(c, seed, 0x1d5d_78, 0);
                let mode = [1u64, 0, 2][seed as usize % 3];
                oc1380_fe_world(c, &mut r, OC1380_FE, OC1380_WORLD);
                c.bus.write32(OC1380_FE + MEMCARD_UI_SAVE_OP, mode as u32);
                c.bus.write32(OC1380_GATE, OC1380_MAGIC);
                c.bus.write32(R_MC_MANAGER_GET, OC1380_APP);
                c.bus.write32(
                    R_REPLAY_WRITE_SAVE_BLOCK,
                    if seed / 4 % 2 == 0 { 0 } else { 0x77 },
                );
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x1d64c0 MemCardUI_ApplySoundMode.

    #[test]
    fn mem_card_ui_apply_sound_mode_matches() {
        check_paths(
            ee::s04::f_001d64c0__generated,
            mem_card_ui_apply_sound_mode,
            40,
            &[
                45, 46, 50, 52, 53, 54, 55, 56, 59, 268, 289, 290, 291, 293, 294, 295, 296, 299,
                300, 301, 302, 303, 304, 305, 307, 308, 313, 326,
            ],
            &|c, seed| {
                install_fakes();
                let mut r = machine(c, seed, 0x1d64_c0, 0);
                c.g[A0][0] = [0u64, 1, 2, 3, 4][seed as usize % 5];
                let inst = OC1380_AUDIO;
                fill(c, &mut r, inst, 0x40);
                let holder = OC1380_AUDIO + 0x1000;
                let lang = OC1380_AUDIO + 0x1100;
                let lang_obj = OC1380_AUDIO + 0x1200;
                let tail = OC1380_AUDIO + 0x1300;
                c.bus.write32(inst + 0x118, holder);
                c.bus.write32(holder, lang);
                c.bus.write32(lang + 0x1d8, lang_obj);
                c.bus.write32(lang_obj + 4, lang_obj);
                c.bus.write16(lang_obj + 0x10, 0);
                c.bus.write32(lang_obj + 0x14, fakes::FAKE);
                c.bus.write32(inst + 0x62b8, 0x40 + seed);
                c.bus.write32(inst + 0x62b4, 0x41 + seed);
                c.bus.write32(inst + 0x534, 0x42 + seed);
                c.bus.write32(inst + 0x5558, tail);
                c.bus.write16(tail + 0x20, 0);
                c.bus.write32(tail + 0x24, fakes::FAKE);
                // The language-holder check: 1 rebuilds the audio system,
                // anything else keeps the current mode.
                c.bus.write32(
                    fakes::LOG_RESULTS + 4,
                    if seed / 8 % 2 == 0 { 1 } else { 2 },
                );
                c.bus.write32(OC1380_GATE, OC1380_MAGIC);
                c.bus.write32(R_AUDIO_INSTANCE, inst);
                c.bus.write32(R_MUSIC_VOLUME, 0x40);
                c.bus.write32(R_SFX_VOLUME, 0x41);
                c.bus.write32(R_SPEECH_VOLUME, 0x42);
            },
        );
    }
}
