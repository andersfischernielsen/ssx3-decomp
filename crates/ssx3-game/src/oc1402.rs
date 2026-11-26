//! Staging module oc1402 (wave av-1): hand-written AV subtitle-list,
//! player shutdown/start/chunk replacements, each with its differential test
//! against the generated body. Moved into the source-tree units by
//! `tools/reorg_modules.py apply oc1402`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub(crate) use crate::game::world::wscriptcache::callee::RCMP_DECODER_RELEASE_CODEC;
pub(crate) use crate::game::world::wscriptcache::callee::STREAM_FILE_RELEASE_CHUNK;
pub(crate) use crate::game::world::wscriptcache::AV_PLAYER_SET_RATE;
pub(crate) use crate::game::world::wscriptcache::SUBTITLE_RECORD;

/// `void AV_SubtitleInit(AVSubtitle *self)` (0x3b0340).
pub(crate) const AV_SUBTITLE_INIT: Callee = Callee {
    addr: 0x003b_0340,
    f: ee::s13::f_003b0340,
};

/// `void AV_SubtitleSetX(AVSubtitle *self, int x)` (0x3b03a8).
pub(crate) const AV_SUBTITLE_SET_X: Callee = Callee {
    addr: 0x003b_03a8,
    f: ee::s13::f_003b03a8,
};

/// `void AV_SubtitleSetY(AVSubtitle *self, int y)` (0x3b03b0).
pub(crate) const AV_SUBTITLE_SET_Y: Callee = Callee {
    addr: 0x003b_03b0,
    f: ee::s13::f_003b03b0,
};

/// `void AV_SubtitleSetWidth(AVSubtitle *self, int width)` (0x3b03b8).
pub(crate) const AV_SUBTITLE_SET_WIDTH: Callee = Callee {
    addr: 0x003b_03b8,
    f: ee::s13::f_003b03b8,
};

/// `void AV_SubtitleSetHeight(AVSubtitle *self, int height)` (0x3b03c0).
pub(crate) const AV_SUBTITLE_SET_HEIGHT: Callee = Callee {
    addr: 0x003b_03c0,
    f: ee::s13::f_003b03c0,
};

/// `void AV_SubtitleSetStringId(AVSubtitle *self, int id)` (0x3b03c8).
pub(crate) const AV_SUBTITLE_SET_STRING_ID: Callee = Callee {
    addr: 0x003b_03c8,
    f: ee::s13::f_003b03c8,
};

/// `void AV_SubtitleSetFrame(AVSubtitle *self, int frame)` (0x3b03d0).
pub(crate) const AV_SUBTITLE_SET_FRAME: Callee = Callee {
    addr: 0x003b_03d0,
    f: ee::s13::f_003b03d0,
};

/// `void AV_Audio_dtor(AV_Audio *self, int flags)` (0x3aee30).
pub(crate) const AV_AUDIO_DTOR: Callee = Callee {
    addr: 0x003a_ee30,
    f: ee::s13::f_003aee30,
};

/// `void *AV_Audio_Init(AV_Audio *self, int a1, int a2)` (0x3aed40).
pub(crate) const AV_AUDIO_INIT: Callee = Callee {
    addr: 0x003a_ed40,
    f: ee::s13::f_003aed40,
};

/// `int AV_Audio_Start(AV_Audio *self)` (0x3aeef8).
pub(crate) const AV_AUDIO_START: Callee = Callee {
    addr: 0x003a_eef8,
    f: ee::s13::f_003aeef8,
};

/// `void RCMP_MakeCallbacks(...)` (0x3b0758).
pub(crate) const RCMP_MAKE_CALLBACKS: Callee = Callee {
    addr: 0x003b_0758,
    f: ee::s13::f_003b0758,
};

/// `RCMP_DECODER *RCMP_DECODER_ctor(RCMP_DECODER *self, void *key)`
/// (0x3b0430).
pub(crate) const RCMP_DECODER_CTOR: Callee = Callee {
    addr: 0x003b_0430,
    f: ee::s13::f_003b0430,
};

/// `int RCMP_DECODER_DecodeFrame(RCMP_DECODER *d, int a1)` (0x3b0600).
pub(crate) const RCMP_DECODER_DECODE_FRAME: Callee = Callee {
    addr: 0x003b_0600,
    f: ee::s13::f_003b0600,
};

/// `void *AV_ChunkInit(AV_Chunk *self)` (0x3b0720).
pub(crate) const AV_CHUNK_INIT: Callee = Callee {
    addr: 0x003b_0720,
    f: ee::s13::f_003b0720,
};

/// `void PS2_SONY_CODEC_INTERNAL_Create(void)` (0x3b11a0).
pub(crate) const PS2_SONY_CODEC_INTERNAL_CREATE: Callee = Callee {
    addr: 0x003b_11a0,
    f: ee::s13::f_003b11a0,
};

/// `void RCMP_DECODER_SetCodec(Decoder *d, Codec *codec, int a2)`
/// (0x3b04f8).
pub(crate) const RCMP_DECODER_SET_CODEC: Callee = Callee {
    addr: 0x003b_04f8,
    f: ee::s13::f_003b04f8,
};

/// `int RCMP_IsMpegChunkTag(uint tag)` (0x3b11f0).
pub(crate) const RCMP_IS_MPEG_CHUNK_TAG: Callee = Callee {
    addr: 0x003b_11f0,
    f: ee::s13::f_003b11f0,
};

/// `void StreamFile_SetPriority(int handle, int a, int b)` (0x3e0ad8).
pub(crate) const STREAM_FILE_SET_PRIORITY: Callee = Callee {
    addr: 0x003e_0ad8,
    f: ee::s14::f_003e0ad8,
};

/// `int AsyncFile_GetResult(int handle, void *out, int *size)` (0x3df9d8).
pub(crate) const ASYNC_FILE_GET_RESULT: Callee = Callee {
    addr: 0x003d_f9d8,
    f: ee::s14::f_003df9d8,
};

/// `int StreamFile_QueueFile(int h, int a, int b, int c)` (0x3e0d80).
pub(crate) const STREAM_FILE_QUEUE_FILE: Callee = Callee {
    addr: 0x003e_0d80,
    f: ee::s14::f_003e0d80,
};

/// `uint EATimer_GetTicks(void)` (0x3e4e98).
pub(crate) const EA_TIMER_GET_TICKS: Callee = Callee {
    addr: 0x003e_4e98,
    f: ee::s14::f_003e4e98,
};

/// `void RCMP_AV_MS_TIMER_Update(void *timer)` (0x42e508).
pub(crate) const RCMP_AV_MS_TIMER_UPDATE: Callee = Callee {
    addr: 0x0042_e508,
    f: ee::s15::f_0042e508,
};

/// `uint EATimer_GetTickRate(void)` (0x3e5008).
pub(crate) const EA_TIMER_GET_TICK_RATE: Callee = Callee {
    addr: 0x003e_5008,
    f: ee::s14::f_003e5008,
};

/// `void StreamFile_Destroy(int handle)` (0x3e0a28).
pub(crate) const STREAM_FILE_DESTROY: Callee = Callee {
    addr: 0x003e_0a28,
    f: ee::s14::f_003e0a28,
};

/// `int EAFile_GetSettings(void *stream)` (0x3de928).
pub(crate) const EA_FILE_GET_SETTINGS: Callee = Callee {
    addr: 0x003d_e928,
    f: ee::s14::f_003de928,
};

/// `void *StreamFile_GetChunk(int stream)` (0x3e12e0).
pub(crate) const STREAM_FILE_GET_CHUNK: Callee = Callee {
    addr: 0x003e_12e0,
    f: ee::s14::f_003e12e0,
};

/// `int EATimer_ServiceCallbacks(void *unused)` (0x3e5928).
pub(crate) const EA_TIMER_SERVICE_CALLBACKS: Callee = Callee {
    addr: 0x003e_5928,
    f: ee::s14::f_003e5928,
};

/// `int StreamFile_IsIdle(int stream)` (0x3e1530).
pub(crate) const STREAM_FILE_IS_IDLE: Callee = Callee {
    addr: 0x003e_1530,
    f: ee::s14::f_003e1530,
};

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_f0c8, av_create_subtitles, "AV_CreateSubtitles"),
    (0x003a_e300, av_player_shutdown, "AV_Player_Shutdown"),
    (0x003a_e450, av_player_start, "AV_Player_Start"),
    (0x003a_ead0, av_player_on_chunk, "AV_Player_OnChunk"),
];

/// `void AV_CreateSubtitles(SubtitleList *list, int count, int x, int y,
/// int width, int height)` (0x3af0c8): allocate `count` 0x34-byte subtitle
/// records, run `AV_SubtitleInit` over each, then stamp every record's
/// geometry (`x`, `y`, `width`, `height`) and its string id (its index).
/// Record 0's frame is cleared as well. `width` arrives in `t0` (spilled to
/// the stack on entry), `height` in `t1`.
pub fn av_create_subtitles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0xb0);
        c.g[V1][0] = 0x51_0000;
        frame.save(c, 20, 0x60);
        c.g[T2][0] = sx(r32(c, V1).wrapping_sub(0x6bd0));
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, 16, 0xa0);
        c.g[A1][0] = 0x34;
        frame.save(c, 17, 0x90);
        let rec_bytes = abi::mult(c, r32(c, S4), r32(c, A1));
        c.g[A1][0] = sx(rec_bytes);
        frame.save(c, 18, 0x80);
        frame.save(c, 19, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 22, 0x40);
        c.g[A0][0] = 0x49_0000;
        frame.save(c, 23, 0x30);
        c.g[S6][0] = c.g[T1][0];
        frame.save(c, 30, 0x20);
        c.g[S7][0] = c.g[A3][0];
        frame.save(c, 21, 0x50);
        c.g[FP][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5260));
        c.sw(sp(c), c.g[T0][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x10));
        c.g[V0][0] = Addr(r32(c, V1).wrapping_sub(0x6bd0)).lw(c);
        c.g[A2][0] = 0;
        c.g[T0][0] = Addr(r32(c, T2).wrapping_add(8)).lw(c);
        c.g[A3][0] = 0;
        let alloc = r32(c, V0);
        c.g[RA][0] = sx(0x003a_f140);
        c.g[S1][0] = sx(r32(c, S4).wrapping_sub(1));
        call_ptr(c, alloc, 0x003a_f140)?;
        c.g[S3][0] = sx(r32(c, V0).wrapping_add(0x10));
        c.sw(r32(c, V0), c.g[S4][0]);
        let no_records = c.g[S4][0] == 0;
        c.g[S0][0] = c.g[S3][0];
        if !no_records {
            c.cycles += 2;
            c.g[S5][0] = u64::MAX;
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003a_f164);
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                AV_SUBTITLE_INIT.call(c, 0x003a_f164)?;
                let more = c.g[S1][0] != c.g[S5][0];
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(SUBTITLE_RECORD as u32));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_f158);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 3;
        c.sw(r32(c, S2), c.g[S3][0]);
        let empty = c.g[S4][0] == 0;
        c.sw(r32(c, S2).wrapping_add(4), c.g[S4][0]);
        if !empty {
            c.cycles += 3;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x003a_f184);
            c.g[A1][0] = 0;
            AV_SUBTITLE_SET_FRAME.call(c, 0x003a_f184)?;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(4)).lw(c);
        let none = c.g[V0][0] == 0;
        c.g[S0][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[S1][0] = 0;
            loop {
                c.cycles += 25;
                c.g[A0][0] = Addr(r32(c, S2)).lw(c);
                c.g[A1][0] = c.g[FP][0];
                c.g[RA][0] = sx(0x003a_f1a8);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                AV_SUBTITLE_SET_X.call(c, 0x003a_f1a8)?;
                c.g[A0][0] = Addr(r32(c, S2)).lw(c);
                c.g[A1][0] = c.g[S7][0];
                c.g[RA][0] = sx(0x003a_f1b8);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                AV_SUBTITLE_SET_Y.call(c, 0x003a_f1b8)?;
                c.g[A0][0] = Addr(r32(c, S2)).lw(c);
                c.g[A1][0] = c.lw(sp(c));
                c.g[RA][0] = sx(0x003a_f1c8);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                AV_SUBTITLE_SET_WIDTH.call(c, 0x003a_f1c8)?;
                c.g[A0][0] = Addr(r32(c, S2)).lw(c);
                c.g[A1][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x003a_f1d8);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                AV_SUBTITLE_SET_HEIGHT.call(c, 0x003a_f1d8)?;
                c.g[A0][0] = Addr(r32(c, S2)).lw(c);
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[RA][0] = sx(0x003a_f1ec);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                AV_SUBTITLE_SET_STRING_ID.call(c, 0x003a_f1ec)?;
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(4)).lw(c);
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(SUBTITLE_RECORD as u32));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_f198);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 12;
        frame.restore(c, 16, 0xa0);
        frame.restore(c, 17, 0x90);
        frame.restore(c, 18, 0x80);
        frame.restore(c, 19, 0x70);
        frame.restore(c, 20, 0x60);
        frame.restore(c, 21, 0x50);
        frame.restore(c, 22, 0x40);
        frame.restore(c, 23, 0x30);
        frame.restore(c, 30, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void AV_Player_Shutdown(AV_Player *this, int flags)` (0x3ae300): tear
/// down a player: release the decoder through its vtable, destroy the audio
/// object, free the timer, destroy both stream handles, apply the pending
/// chunk callbacks, and free the stream/timer slots. `flags & 1` frees the
/// player itself through the heap free slot.
pub fn av_player_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0x20);
        c.g[A2][0] = Addr(r32(c, S0).wrapping_add(0x64)).lw(c);
        let no_decoder = c.g[A2][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !no_decoder {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x18)).lw(c);
            c.g[A1][0] = 3;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(8)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
            let release = r32(c, V1);
            c.g[RA][0] = sx(0x003a_e33c);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, release, 0x003a_e33c)?;
            c.sw(r32(c, S0).wrapping_add(0x64), 0);
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x5c)).lw(c);
        } else {
            c.cycles += 4;
            c.g[RA][0] = sx(0x003a_e354);
            c.g[A1][0] = 3;
            AV_AUDIO_DTOR.call(c, 0x003a_e354)?;
            c.sw(r32(c, S0), 0);
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x5c)).lw(c);
        }
        c.cycles += 2;
        let no_timer = c.g[A0][0] == 0;
        c.g[V1][0] = 0x51_0000;
        if !no_timer {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, V1).wrapping_sub(0x6bcc)).lw(c);
            let free = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e370);
            call_ptr(c, free, 0x003a_e370)?;
            c.sw(r32(c, S0).wrapping_add(0x5c), 0);
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x003a_e37c);
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x30)).lw(c);
        STREAM_FILE_DESTROY.call(c, 0x003a_e37c)?;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x14)).lw(c);
        let one_handle = c.g[V0][0] == 0;
        c.sw(r32(c, S0).wrapping_add(0x30), 0);
        if !one_handle {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003a_e390);
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x34)).lw(c);
            STREAM_FILE_DESTROY.call(c, 0x003a_e390)?;
            c.sw(r32(c, S0).wrapping_add(0x34), 0);
        }
        c.cycles += 3;
        c.g[S1][0] = Addr(r32(c, S0).wrapping_add(0x20)).lw(c);
        let no_extra = c.g[S1][0] == 0;
        c.g[V0][0] = 0x20;
        if !no_extra {
            c.cycles += 7;
            c.g[A0][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x003a_e3ac);
            c.sw(sp(c), c.g[V0][0]);
            EA_FILE_GET_SETTINGS.call(c, 0x003a_e3ac)?;
            c.g[V0][0] = Addr(sp(c).wrapping_add(8)).lw(c);
            let apply = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e3b8);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, apply, 0x003a_e3b8)?;
            c.sw(r32(c, S0).wrapping_add(0x20), 0);
        }
        c.cycles += 3;
        c.g[S1][0] = Addr(r32(c, S0).wrapping_add(0x28)).lw(c);
        let no_extra2 = c.g[S1][0] == 0;
        c.g[V0][0] = 0x20;
        if !no_extra2 {
            c.cycles += 7;
            c.g[A0][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x003a_e3d4);
            c.sw(sp(c), c.g[V0][0]);
            EA_FILE_GET_SETTINGS.call(c, 0x003a_e3d4)?;
            c.g[V0][0] = Addr(sp(c).wrapping_add(8)).lw(c);
            let apply = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e3e0);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, apply, 0x003a_e3e0)?;
            c.sw(r32(c, S0).wrapping_add(0x28), 0);
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
        let no_free = c.g[A0][0] == 0;
        c.g[V1][0] = 0x51_0000;
        if !no_free {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, V1).wrapping_sub(0x6bcc)).lw(c);
            let free = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e3fc);
            call_ptr(c, free, 0x003a_e3fc)?;
            c.sw(r32(c, S0).wrapping_add(4), 0);
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
        let no_free2 = c.g[A0][0] == 0;
        c.g[V1][0] = 0x51_0000;
        if !no_free2 {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, V1).wrapping_sub(0x6bcc)).lw(c);
            let free = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e418);
            call_ptr(c, free, 0x003a_e418)?;
            c.sw(r32(c, S0).wrapping_add(8), 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 1;
        let keep = c.g[V0][0] == 0;
        c.g[V1][0] = 0x51_0000;
        if !keep {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, V1).wrapping_sub(0x6bcc)).lw(c);
            let free = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e434);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, free, 0x003a_e434)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Player_Start(AV_Player *this, int a1, int rate)` (0x3ae450):
/// (re)open the two stream handles, resolve the pending async reads, bring
/// up the audio object and the decoder, start the millisecond timer and set
/// the playback rate. Returns the first decoded frame number (`+0x74`).
pub fn av_player_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S4, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S2, 0x60);
        c.g[A1][0] = 0x97;
        frame.save(c, S3, 0x50);
        c.g[A2][0] = 0x33;
        frame.save_ra(c, 0x20);
        c.g[RA][0] = sx(0x003a_e48c);
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x34)).lw(c);
        STREAM_FILE_SET_PRIORITY.call(c, 0x003a_e48c)?;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x30)).lw(c);
        c.g[A1][0] = 0x97;
        c.g[RA][0] = sx(0x003a_e49c);
        c.g[A2][0] = 0x33;
        STREAM_FILE_SET_PRIORITY.call(c, 0x003a_e49c)?;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x24)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x2c)).lw(c);
        } else {
            c.cycles += 10;
            c.g[A2][0] = sx(sp(c).wrapping_add(0x10));
            c.g[RA][0] = sx(0x003a_e4b4);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x20));
            ASYNC_FILE_GET_RESULT.call(c, 0x003a_e4b4)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x30)).lw(c);
            c.g[A3][0] = 0;
            c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x20)).lw(c);
            c.g[RA][0] = sx(0x003a_e4c8);
            c.g[A2][0] = Addr(sp(c).wrapping_add(0x10)).lw(c);
            STREAM_FILE_QUEUE_FILE.call(c, 0x003a_e4c8)?;
            c.sw(r32(c, S1).wrapping_add(0x38), c.g[V0][0]);
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x2c)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x14)).lw(c);
        } else {
            c.cycles += 10;
            c.g[A2][0] = sx(sp(c).wrapping_add(0x14));
            c.g[RA][0] = sx(0x003a_e4e4);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x28));
            ASYNC_FILE_GET_RESULT.call(c, 0x003a_e4e4)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x34)).lw(c);
            c.g[A3][0] = 0;
            c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x28)).lw(c);
            c.g[RA][0] = sx(0x003a_e4f8);
            c.g[A2][0] = Addr(sp(c).wrapping_add(0x14)).lw(c);
            STREAM_FILE_QUEUE_FILE.call(c, 0x003a_e4f8)?;
            c.sw(r32(c, S1).wrapping_add(0x3c), c.g[V0][0]);
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x14)).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x1c)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
            c.sw(r32(c, S1).wrapping_add(0x3c), c.g[V0][0]);
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x1c)).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.sw(r32(c, S1).wrapping_add(0x54), 0);
            c.cycles += 2;
            c.g[S2][0] = 0x51_0000;
            c.g[S3][0] = 0x49_0000;
        } else {
            c.cycles += 22;
            c.g[T1][0] = 0x51_0000;
            c.g[S0][0] = 0x49_0000;
            c.g[V1][0] = sx(r32(c, T1).wrapping_sub(0x6bd0));
            c.g[V0][0] = Addr(r32(c, T1).wrapping_sub(0x6bd0)).lw(c);
            c.g[T0][0] = Addr(r32(c, V1).wrapping_add(8)).lw(c);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x5210));
            c.g[A1][0] = 0x14;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            let alloc = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e548);
            c.g[S2][0] = c.g[T1][0];
            call_ptr(c, alloc, 0x003a_e548)?;
            c.g[S3][0] = c.g[S0][0];
            c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x34)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003a_e55c);
            c.g[A2][0] = Addr(r32(c, S1).wrapping_add(0x3c)).lw(c);
            AV_AUDIO_INIT.call(c, 0x003a_e55c)?;
            c.sw(r32(c, S1), c.g[V0][0]);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
            c.g[V1][0] = !(0u64 | c.g[V1][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 31);
            c.sw(r32(c, S1).wrapping_add(0x54), c.g[V1][0]);
        }
        c.cycles += 5;
        c.sw(r32(c, S1).wrapping_add(0x64), 0);
        c.g[S0][0] = sx(r32(c, S2).wrapping_sub(0x6bd0));
        c.g[V0][0] = Addr(r32(c, S2).wrapping_sub(0x6bd0)).lw(c);
        let no_heap = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !no_heap {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        let no_callbacks = c.g[V1][0] == 0;
        c.g[A2][0] = 0x3b_0000;
        if !no_callbacks {
            c.cycles += 19;
            c.sw(r32(c, S1).wrapping_add(0x60), c.g[S1][0]);
            c.g[A3][0] = 0x3b_0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0x1348));
            c.g[A3][0] = sx(r32(c, A3).wrapping_sub(0x12e0));
            c.g[T0][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x003a_e5c0);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x60));
            RCMP_MAKE_CALLBACKS.call(c, 0x003a_e5c0)?;
            c.g[T0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
            c.g[A0][0] = sx(r32(c, S3).wrapping_add(0x5210));
            c.g[V0][0] = Addr(r32(c, S2).wrapping_sub(0x6bd0)).lw(c);
            c.g[A1][0] = 0x1c;
            c.g[A2][0] = 0;
            let alloc = r32(c, V0);
            c.g[RA][0] = sx(0x003a_e5dc);
            c.g[A3][0] = 0;
            call_ptr(c, alloc, 0x003a_e5dc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003a_e5e8);
            c.g[A1][0] = c.g[SP][0];
            RCMP_DECODER_CTOR.call(c, 0x003a_e5e8)?;
            c.sw(r32(c, S1).wrapping_add(0x64), c.g[V0][0]);
        }
        c.cycles += 25;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x64)).lw(c);
        c.g[RA][0] = sx(0x003a_e5f8);
        c.g[A1][0] = 0;
        RCMP_DECODER_DECODE_FRAME.call(c, 0x003a_e5f8)?;
        c.sw(r32(c, S1).wrapping_add(0x74), c.g[V0][0]);
        c.g[A1][0] = 0x18;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
        c.g[V0][0] = Addr(r32(c, S2).wrapping_sub(0x6bd0)).lw(c);
        let timer_alloc = r32(c, V0);
        c.g[RA][0] = sx(0x003a_e618);
        c.g[A0][0] = sx(r32(c, S3).wrapping_add(0x5210));
        call_ptr(c, timer_alloc, 0x003a_e618)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003a_e624);
        c.sd(r32(c, S0).wrapping_add(8), 0);
        EA_TIMER_GET_TICKS.call(c, 0x003a_e624)?;
        c.g[V1][0] = 0x1000;
        c.sw(r32(c, S0), c.g[V0][0]);
        c.sw(r32(c, S0).wrapping_add(0x10), c.g[V1][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x003a_e63c);
        c.sw(r32(c, S1).wrapping_add(0x5c), c.g[S0][0]);
        RCMP_AV_MS_TIMER_UPDATE.call(c, 0x003a_e63c)?;
        c.g[RA][0] = sx(0x003a_e644);
        EA_TIMER_GET_TICK_RATE.call(c, 0x003a_e644)?;
        c.g[A0][0] = c.ld(r32(c, S0).wrapping_add(8));
        // Either way the timer block ends here: the original branches to the
        // same block exit whether the tick rate came back zero or not.
        c.cycles += 14;
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        c.g[V1][0] = c.g[A0][0] << 5;
        c.sw(r32(c, S1).wrapping_add(0x40), c.g[S5][0]);
        c.g[V1][0] = c.g[V1][0].wrapping_sub(c.g[A0][0]);
        c.sw(r32(c, S1).wrapping_add(0x50), 0);
        c.g[V1][0] = c.g[V1][0] << 2;
        c.sw(r32(c, S1).wrapping_add(0x58), 0);
        c.g[V1][0] = c.g[V1][0].wrapping_add(c.g[A0][0]);
        c.g[V1][0] = c.g[V1][0] << 35;
        c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
        c.divu(0, 3, 2);
        c.g[V1][0] = c.lo[0];
        let no_audio = c.g[A1][0] == 0;
        c.sw(r32(c, S1).wrapping_add(0x4c), c.g[V1][0]);
        if !no_audio {
            c.cycles += 2;
            c.g[RA][0] = sx(0x003a_e690);
            c.g[A0][0] = c.g[A1][0];
            AV_AUDIO_START.call(c, 0x003a_e690)?;
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x003a_e69c);
        c.g[A1][0] = 0x1000;
        AV_PLAYER_SET_RATE.call(c, 0x003a_e69c)?;
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x74)).lw(c);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `int AV_Player_OnChunk(AV_Player *this, void *decoder, void **out)`
/// (0x3aead0): poll the stream for the next chunk and hand it to the codec:
/// fetch a chunk, tag it (MPEG chunks get a codec object created and set),
/// release it, and repeat while the stream is idle. Returns through the save
/// slots; the out pointer receives the working chunk buffer.
pub fn av_player_on_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        c.g[S3][0] = 0;
        frame.save(c, S5, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S7, 0x30);
        c.g[S6][0] = 0x51_0000;
        frame.save(c, FP, 0x20);
        c.g[S7][0] = 0x3;
        frame.save(c, S0, 0xa0);
        c.g[FP][0] = 0x2;
        frame.save(c, S1, 0x90);
        frame.save_ra(c, 0x10);
        frame.save(c, S2, 0x80);
        c.g[S2][0] = Addr(r32(c, A0).wrapping_add(0x30)).lw(c);
        c.sw(r32(c, S5), 0);
        'poll: loop {
            'done: {
                'check: {
                    'chunk: {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x003a_eb28);
                        c.g[A0][0] = c.g[S2][0];
                        STREAM_FILE_GET_CHUNK.call(c, 0x003a_eb28)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x003a_eb34);
                        c.g[A0][0] = 0;
                        EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003a_eb34)?;
                        if c.g[S0][0] == 0 {
                            break 'chunk;
                        }
                        'filter: {
                            c.cycles += 15;
                            c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
                            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(1)).lbu(c);
                            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(2)).lbu(c);
                            c.g[V0][0] = sx(r32(c, V0) << 24);
                            c.g[V1][0] = sx(r32(c, V1) << 16);
                            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(3)).lbu(c);
                            c.g[A0][0] = sx(r32(c, A0) << 8);
                            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                            c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                            c.g[S1][0] = c.g[V0][0] | c.g[A1][0];
                            c.g[RA][0] = sx(0x003a_eb6c);
                            c.g[A0][0] = c.g[S1][0];
                            RCMP_IS_MPEG_CHUNK_TAG.call(c, 0x003a_eb6c)?;
                            if c.g[V0][0] != 0 {
                                c.g[S3][0] = c.g[S7][0];
                            }
                            let plain = c.g[S3][0] == 0;
                            c.g[V0][0] = 0x51_0000;
                            if plain {
                                break 'filter;
                            }
                            'init: {
                                c.cycles += 15;
                                c.g[A1][0] = 0xc;
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x6bd0));
                                c.g[A2][0] = 0;
                                c.g[T0][0] = Addr(r32(c, V0).wrapping_add(8)).lw(c);
                                c.g[V0][0] = 0x49_0000;
                                c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x5210));
                                c.g[V0][0] = Addr(r32(c, S6).wrapping_sub(0x6bd0)).lw(c);
                                let alloc = r32(c, V0);
                                c.g[RA][0] = sx(0x003a_eb9c);
                                c.g[A3][0] = 0;
                                call_ptr(c, alloc, 0x003a_eb9c)?;
                                c.g[RA][0] = sx(0x003a_eba4);
                                c.g[A0][0] = c.g[V0][0];
                                AV_CHUNK_INIT.call(c, 0x003a_eba4)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.sw(r32(c, S5), c.g[S1][0]);
                                let tag_mismatch = c.g[S3][0] != c.g[FP][0];
                                // Delay slot of the tag test: link the chunk
                                // on both paths.
                                c.sw(r32(c, S1), c.g[S0][0]);
                                if tag_mismatch {
                                    break 'init;
                                }
                                // Unreachable: S3 is 0 or 3 here (cleared on
                                // entry, set to 3 on an MPEG tag) while FP is
                                // 2, so the original's else arm never runs.
                                c.cycles += 13;
                                c.g[V1][0] = sx(r32(c, S0).wrapping_add(4));
                                c.g[A2][0] = Addr(r32(c, S0).wrapping_add(4)).lbu(c);
                                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(3)).lbu(c);
                                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(2)).lbu(c);
                                c.g[A1][0] = Addr(r32(c, V1).wrapping_add(1)).lbu(c);
                                c.g[V0][0] = sx(r32(c, V0) << 24);
                                c.g[A0][0] = sx(r32(c, A0) << 16);
                                c.sw(r32(c, S1).wrapping_add(4), c.g[S0][0]);
                                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                                c.g[A1][0] = sx(r32(c, A1) << 8);
                                c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                            }
                            c.cycles += 13;
                            c.g[V1][0] = sx(r32(c, S0).wrapping_add(4));
                            c.g[A3][0] = Addr(r32(c, S0).wrapping_add(4)).lbu(c);
                            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(3)).lbu(c);
                            c.g[A2][0] = sx(r32(c, S0).wrapping_add(8));
                            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(2)).lbu(c);
                            c.g[A1][0] = Addr(r32(c, V1).wrapping_add(1)).lbu(c);
                            c.g[V0][0] = sx(r32(c, V0) << 24);
                            c.g[A0][0] = sx(r32(c, A0) << 16);
                            c.sw(r32(c, S1).wrapping_add(4), c.g[A2][0]);
                            c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                            c.g[A1][0] = sx(r32(c, A1) << 8);
                            c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                            c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                            c.cycles += 4;
                            c.sw(r32(c, S1).wrapping_add(8), c.g[V0][0]);
                            c.g[V0][0] = Addr(r32(c, S4).wrapping_add(0x14)).lw(c);
                            let encoded = c.g[V0][0] != 0;
                            frame.restore(c, S0, 0xa0);
                            if encoded {
                                break 'check;
                            }
                            c.cycles += 2;
                            if c.g[S3][0] != c.g[S7][0] {
                                frame.restore(c, S1, 0x90);
                                break 'done;
                            }
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x003a_ec3c);
                            PS2_SONY_CODEC_INTERNAL_CREATE.call(c, 0x003a_ec3c)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x003a_ec4c);
                            c.g[A0][0] = c.g[S4][0];
                            RCMP_DECODER_SET_CODEC.call(c, 0x003a_ec4c)?;
                            frame.restore(c, S0, 0xa0);
                            if c.g[V0][0] == c.g[FP][0] {
                                break 'check;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x003a_ec5c);
                            c.g[A0][0] = c.g[S4][0];
                            RCMP_DECODER_RELEASE_CODEC.call(c, 0x003a_ec5c)?;
                            frame.restore(c, S0, 0xa0);
                            break 'check;
                        }
                        c.cycles += 5;
                        c.sw(sp(c), c.g[S1][0]);
                        c.g[A1][0] = c.g[S0][0];
                        c.sb(sp(c).wrapping_add(4), 0);
                        c.g[RA][0] = sx(0x003a_ec78);
                        c.g[A0][0] = c.g[S2][0];
                        STREAM_FILE_RELEASE_CHUNK.call(c, 0x003a_ec78)?;
                        break 'chunk;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003a_ec80);
                    c.g[A0][0] = c.g[S2][0];
                    STREAM_FILE_IS_IDLE.call(c, 0x003a_ec80)?;
                    let more = c.g[V0][0] == 0;
                    frame.restore(c, S0, 0xa0);
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_eb20);
                        }
                        continue 'poll;
                    }
                    break 'check;
                }
                c.cycles += 1;
                frame.restore(c, S1, 0x90);
            }
            break 'poll;
        }
        c.cycles += 10;
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, check_paths, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Fresh machine with the BELibrary/world singletons, the SND-not-ready
    /// flag (`lb(0x50aa5c)`) and the assert-not-busy word clear, the tick
    /// globals scribbled, and the wsAlloc/free slots (`gp - 0x6bd0/0x6bcc`)
    /// pointed at the test fake. Returns the RNG for fixture garbage.
    pub(crate) fn av_test_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x30_0000);
        c.bus.write32(GAME_GP - 0x848, DATA + 0x31_0000);
        c.bus.write8(0x0050_aa5c, 0);
        c.bus.write32(0x0045_0da4, 0);
        c.bus.write32(0x0045_0dcc, 0x1111_1111);
        c.bus.write32(0x0045_0dc8, 0);
        c.bus.write32(0x0050_9430, FAKE);
        c.bus.write32(0x0050_9434, FAKE);
        c.bus.write32(0x0050_9438, 0);
        r
    }

    /// Fill `words` words at `addr` with nonzero garbage.
    pub(crate) fn av_fill_words(c: &mut Cpu, r: &mut Rng, addr: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(addr + 4 * i, r.next_u32() | 0x0101_0101);
        }
    }

    /// Gate for the stream fakes below: they answer only while this word holds
    /// `AVSTREAM_CTRL_KEY` (otherwise the slot body they replaced runs).
    /// `AVSTREAM_CHUNKVAL` is what the queued-chunk fake returns: 0 for the
    /// empty arm, the forged chunk for the chunk arms. The stream-queue
    /// internals (`Validate`/mutex/list walk) stay on the other side of this
    /// boundary; everything downstream (tag dispatch, codec paths, release)
    /// runs real.
    pub(crate) const AVSTREAM_CTRL: u32 = DATA + 0x3100;
    pub(crate) const AVSTREAM_CTRL_KEY: u32 = 0x5354_5245;
    pub(crate) const AVSTREAM_CHUNKVAL: u32 = DATA + 0x3104;

    static AVSTREAM_PREV_ISIDLE: AtomicUsize = AtomicUsize::new(0);
    static AVSTREAM_PREV_GETCHUNK: AtomicUsize = AtomicUsize::new(0);

    fn avstream_gated(c: &mut Cpu) -> bool {
        c.bus.read32(AVSTREAM_CTRL) == AVSTREAM_CTRL_KEY
    }

    /// `StreamFile_IsIdle`: the stream is idle, so the chunk loop exits.
    /// `StreamFile_GetChunk` stays real: the fixtures forge a draining
    /// stream and the test proves it drains.
    fn avstream_fake_isidle(c: &mut Cpu) {
        if !avstream_gated(c) {
            return fakes::fall_back(c, &AVSTREAM_PREV_ISIDLE, ee::s14::f_003e1530__generated);
        }
        fakes::fake_call(c, 0x003e_1530);
        c.g[V0][0] = 1;
    }

    fn avstream_install_isidle_fake() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E1530),
            avstream_fake_isidle,
            &AVSTREAM_PREV_ISIDLE,
        );
    }

    /// `StreamFile_GetChunk`: the fixture-forged queue head.
    fn avstream_fake_getchunk(c: &mut Cpu) {
        if !avstream_gated(c) {
            return fakes::fall_back(c, &AVSTREAM_PREV_GETCHUNK, ee::s14::f_003e12e0__generated);
        }
        fakes::fake_call(c, 0x003e_12e0);
        c.g[V0][0] = c.bus.read32(AVSTREAM_CHUNKVAL) as u64;
    }

    fn avstream_install_getchunk_fake() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E12E0),
            avstream_fake_getchunk,
            &AVSTREAM_PREV_GETCHUNK,
        );
    }

    #[test]
    fn av_create_subtitles_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = av_test_base(c, seed, 0xaf0c8);
            let list = DATA + 0x1410;
            let count = seed % 4;
            av_fill_words(c, &mut r, list, 2);
            c.pc = 0x003a_f0c8;
            c.g[A0][0] = list as u64;
            c.g[A1][0] = count as u64;
            c.g[A2][0] = (0x111 + (seed / 4) % 5) as u64;
            c.g[A3][0] = (0x222 + (seed / 8) % 7) as u64;
            c.g[T0][0] = (0x333 + (seed / 16) % 9) as u64;
            c.g[T1][0] = (0x444 + (seed / 32) % 9) as u64;
        };
        check_paths(
            ee::s13::f_003af0c8__generated,
            av_create_subtitles,
            16,
            &[56, 135, 199, 269],
            &setup,
        );
        // The built list holds `count` records stamped with the geometry
        // and the record index as string id (record 0's frame is cleared).
        // The records land at the fake-alloc address, which could in theory
        // overlap the list head; `check` above already proves those seeds
        // equal, so only the shape asserts skip an overlapped seed.
        for seed in 0..8u32 {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            av_create_subtitles(&mut c);
            let list = DATA + 0x1410;
            let count = seed % 4;
            assert_eq!(c.bus.read32(list + 4), count, "seed {seed}");
            let base = c.bus.read32(list);
            if count == 0 {
                continue;
            }
            if base >= list.wrapping_sub(3 * 0x34) && base < list + 8 {
                continue;
            }
            let x = 0x111 + (seed / 4) % 5;
            let y = 0x222 + (seed / 8) % 7;
            let width = 0x333 + (seed / 16) % 9;
            let height = 0x444 + (seed / 32) % 9;
            for i in 0..count {
                let rec = base + i * 0x34;
                assert_eq!(c.bus.read32(rec), x, "seed {seed} rec {i} x");
                assert_eq!(c.bus.read32(rec + 4), y, "seed {seed} rec {i} y");
                assert_eq!(c.bus.read32(rec + 8), width, "seed {seed} rec {i} w");
                assert_eq!(c.bus.read32(rec + 0xc), height, "seed {seed} rec {i} h");
                assert_eq!(c.bus.read32(rec + 0x10), i, "seed {seed} rec {i} id");
                assert_eq!(
                    c.bus.read32(rec + 0x14),
                    if i == 0 { 0 } else { 0xffff_ffff },
                    "seed {seed} rec {i} frame"
                );
            }
        }
    }

    fn av_shutdown_setup(c: &mut Cpu, seed: u32) {
        let mut r = av_test_base(c, seed, 0xae300);
        let this = DATA + 0x1410;
        let arm = seed % 4;
        av_fill_words(c, &mut r, this, 0x1e);
        let decoder = DATA + 0x1510;
        let vtab = DATA + 0x1600;
        av_fill_words(c, &mut r, decoder, 8);
        av_fill_words(c, &mut r, vtab, 4);
        c.bus.write32(vtab + 8, 0);
        c.bus.write32(vtab + 0xc, FAKE);
        c.bus.write32(decoder + 0x18, vtab);
        let audio = DATA + 0x1700;
        av_fill_words(c, &mut r, audio, 4);
        c.bus.write32(audio + 4, 0);
        c.pc = 0x003a_e300;
        c.g[A0][0] = this as u64;
        c.g[A1][0] = (arm >= 2) as u64;
        c.bus
            .write32(this + 0x64, if arm >= 2 { decoder } else { 0 });
        c.bus.write32(this, if arm >= 2 { audio } else { 0 });
        c.bus.write32(this + 0x5c, if arm >= 2 { 0x77 } else { 0 });
        c.bus.write32(this + 0x30, 0);
        c.bus.write32(this + 0x34, 0);
        c.bus.write32(this + 0x14, if arm >= 1 { 0x99 } else { 0 });
        c.bus.write32(this + 0x20, if arm >= 2 { FAKE } else { 0 });
        c.bus.write32(this + 0x28, if arm >= 3 { FAKE } else { 0 });
        c.bus.write32(this + 0x4, if arm >= 2 { 0x55 } else { 0 });
        c.bus.write32(this + 0x8, if arm >= 3 { 0x66 } else { 0 });
        // No stack setup needed: each GetSettings block stores 0x20 at
        // sp first (so the length is always 32), and the settings copy
        // zeroes sp+8, so the chunk callback always goes through the
        // miss-hook fake at target 0.
    }

    #[test]
    fn av_player_shutdown_matches() {
        check_paths(
            ee::s13::f_003ae300__generated,
            av_player_shutdown,
            32,
            &[56, 75, 241, 345],
            &av_shutdown_setup,
        );
    }

    fn av_start_setup(c: &mut Cpu, seed: u32) {
        let mut r = av_test_base(c, seed, 0xae450);
        avstream_install_isidle_fake();
        c.bus.write32(AVSTREAM_CTRL, AVSTREAM_CTRL_KEY);
        let this = DATA + 0x1410;
        let arm = seed % 6;
        av_fill_words(c, &mut r, this, 0x1e);
        let stream = DATA + 0x1900;
        av_fill_words(c, &mut r, stream, 5);
        c.bus.write32(stream + 0x10, 0x1000);
        let hp1 = DATA + 0x1a10;
        let hp2 = DATA + 0x1a20;
        c.bus.write32(hp1, hp2);
        c.bus.write32(hp2, 0x1234_5678);
        let bad = arm == 1;
        c.pc = 0x003a_e450;
        c.g[A0][0] = this as u64;
        c.g[A1][0] = 0x700;
        c.g[A2][0] = 0x800;
        c.bus.write32(this + 0x34, if bad { hp1 } else { 0 });
        c.bus.write32(this + 0x30, if bad { hp1 } else { 0 });
        c.bus.write32(
            this + 0x24,
            match arm {
                2 => 5,
                _ => 0,
            },
        );
        c.bus.write32(
            this + 0x2c,
            match arm {
                3 => 9,
                _ => 0,
            },
        );
        c.bus.write32(this + 0x14, if arm == 5 { 0 } else { 1 });
        c.bus.write32(this + 0x1c, if arm >= 4 { 0 } else { 1 });
        c.bus.write32(this + 0x5c, stream);
        c.bus.write32(this, 0);
        c.bus.write32(0x0050_9430, if arm >= 4 { FAKE } else { 0 });
    }

    #[test]
    fn av_player_start_matches() {
        check_paths(
            ee::s13::f_003ae450__generated,
            av_player_start,
            36,
            &[236, 244, 383, 398, 760, 762],
            &av_start_setup,
        );
        // The two async blocks land in different fields: arm C resolves
        // handle 1 into +0x38, arm C2 resolves handle 2 into +0x3c. The
        // queue call returns slot 0 for the forged handles (not -1).
        for seed in [2u32, 8, 14, 20, 26, 32] {
            let mut c = crate::difftest::fresh();
            av_start_setup(&mut c, seed);
            assert_eq!(seed % 6, 2);
            av_player_start(&mut c);
            assert_eq!(c.bus.read32(DATA + 0x1410 + 0x38), 0);
        }
        for seed in [3u32, 9, 15, 21, 27, 33] {
            let mut c = crate::difftest::fresh();
            av_start_setup(&mut c, seed);
            assert_eq!(seed % 6, 3);
            av_player_start(&mut c);
            assert_eq!(c.bus.read32(DATA + 0x1410 + 0x3c), 0);
        }
        // Arms 4 and 5 take different sub-paths at the +0x14 test (arm 5
        // stores [+0x38] into +0x3c with a +3 charge; arm 4 skips it) and
        // the audio path downstream reads +0x3c, so their totals differ
        // (760 vs 762) even though the +0x3c word itself is clobbered
        // downstream on both arms.
    }

    /// Value-boundary fixtures (arm 5, which runs the audio-init path):
    /// a heap word at the wsAlloc free slot (so `no_heap` is false and the
    /// V0==0 boundary is hit) and a nonzero audio word (so `!(0&V1)`
    /// differs). Outputs are prefilled nonzero and checked by the RAM
    /// comparison. (Timer ticks stay 0: TIMER_UPDATE zeroes the fresh
    /// timer's +8 before it is read, so the shift mutants need a
    /// ticking timer, documented as a gap.)
    pub(crate) fn av_start_values_setup(c: &mut Cpu, seed: u32) {
        av_start_setup(c, seed);
        // Heap with a zero word at +4 (V0==0 boundary for the `<` test).
        c.bus.write32(0x0050_a430, DATA + 0x4000);
        c.bus.write32(DATA + 0x4000 + 4, 0);
        // Nonzero audio word with bit 31 set (arm-5 AudioInit returns a
        // stable address; bit 31 survives the >>31 sign test below).
        c.bus.write32(0x0083_a7d0 + 0xc, 0x8abc_def0);
    }

    #[test]
    fn av_player_start_values_matches() {
        check(
            ee::s13::f_003ae450__generated,
            av_player_start,
            36,
            &av_start_values_setup,
        );
    }

    fn av_onchunk_setup(c: &mut Cpu, seed: u32) {
        let mut r = av_test_base(c, seed, 0xaead0);
        avstream_install_isidle_fake();
        avstream_install_getchunk_fake();
        c.bus.write32(AVSTREAM_CTRL, AVSTREAM_CTRL_KEY);
        // A: empty stream; B1: plain chunk; B2: MPEG chunk, decoder
        // busy; B3: MPEG chunk, codec created and set.
        let arm = seed % 4;
        let player = DATA + 0x1410;
        let decoder = DATA + 0x1420;
        let out = DATA + 0x1430;
        let stream = DATA + 0x1440;
        let head = DATA + 0x1450;
        let chunk = DATA + 0x1460;
        av_fill_words(c, &mut r, player, 0x10);
        av_fill_words(c, &mut r, decoder, 0x10);
        av_fill_words(c, &mut r, stream, 5);
        av_fill_words(c, &mut r, head, 0x10);
        av_fill_words(c, &mut r, chunk, 8);
        c.bus.write32(player + 0x30, stream);
        c.bus.write32(stream, head);
        c.bus.write32(stream + 8, if arm == 0 { 0 } else { 0x10 });
        c.bus.write32(stream + 0xc, chunk);
        c.bus.write32(head, 0x4d52_5453);
        c.bus.write32(head + 4, 0);
        c.bus.write32(head + 0xc, 0);
        c.bus.write32(head + 0x30, 0);
        c.bus.write32(head + 0x34, 8);
        c.bus.write32(head + 0x38, 0);
        c.bus.write32(chunk + 4, if arm == 0 { 0 } else { 0x10 });
        for i in 0..4u32 {
            c.bus
                .write8(chunk + i, if arm == 1 { (i == 0) as u8 } else { 0 });
        }
        c.bus.write32(
            decoder + 0x14,
            match arm {
                2 => 0x888,
                3 => 0,
                _ => 0x99,
            },
        );
        let sp = crate::difftest::SP - 0xb0;
        c.bus.write32(sp + 4, DATA + 0x1500);
        c.bus.write32(DATA + 0x1508, 0);
        c.bus
            .write32(AVSTREAM_CHUNKVAL, if arm == 0 { 0 } else { chunk });
        c.pc = 0x003a_ead0;
        c.g[A0][0] = player as u64;
        c.g[A1][0] = decoder as u64;
        c.g[A2][0] = out as u64;
    }

    #[test]
    fn av_player_on_chunk_matches() {
        check_paths(
            ee::s13::f_003aead0__generated,
            av_player_on_chunk,
            32,
            &[162, 227, 224, 328],
            &av_onchunk_setup,
        );
    }

    /// Second-tag bytes nonzero: chunk bytes 5-7 feed the codec-path tag
    /// assembly (`V0 = b7<<24|b6<<16|b5<<8|b4`), so the shift/operator
    /// mutants there change the stored tag. Bytes 0-3 keep the real MPCh
    /// tag so dispatch is unchanged (MPEG arms).
    pub(crate) fn av_onchunk_bytes_setup(c: &mut Cpu, seed: u32) {
        av_onchunk_setup(c, seed);
        let chunk = DATA + 0x1460;
        // Nonzero, seed-varying; byte 5 gets the high bit on some seeds.
        c.bus.write8(
            chunk + 5,
            (0x20 + seed * 37) as u8 | if seed % 3 == 0 { 0x80 } else { 0 },
        );
        c.bus.write8(chunk + 6, (0x30 + seed * 53) as u8);
        c.bus.write8(chunk + 7, (0x40 + seed * 71) as u8);
    }

    #[test]
    fn av_player_on_chunk_bytes_matches() {
        check(
            ee::s13::f_003aead0__generated,
            av_player_on_chunk,
            32,
            &av_onchunk_bytes_setup,
        );
    }

    /// The chunk loop's bare event check delivers without unwinding, so both
    /// sides stay in lockstep through the driver. `GetChunk`/`IsIdle` are
    /// gated fakes, `EATimer` runs real (bare check only); everything
    /// downstream of a queued chunk runs real.
    #[test]
    fn av_player_on_chunk_events() {
        driven!(av_onchunk_gen_driven, ee::s13::f_003aead0__generated);
        driven!(av_onchunk_hand_driven, av_player_on_chunk);
        check_events(
            av_onchunk_gen_driven,
            av_onchunk_hand_driven,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0xaead_00, 0);
                av_onchunk_setup(c, seed % 32);
                c.pc = 0x003a_ead0;
                schedule(c, &mut r, seed, 0x003a_ead0)
            },
        );
    }

    /// Both subtitle-list build loops deliver their bare event checks without
    /// unwinding. The allocator and record writes run real; the list head
    /// is prefilled nonzero.
    #[test]
    fn av_create_subtitles_events() {
        driven!(av_create_gen_driven, ee::s13::f_003af0c8__generated);
        driven!(av_create_hand_driven, av_create_subtitles);
        check_events(
            av_create_gen_driven,
            av_create_hand_driven,
            96,
            &|c, seed| {
                let mut r = av_test_base(c, seed, 0xaf0c_80);
                let list = DATA + 0x1410;
                av_fill_words(c, &mut r, list, 2);
                c.pc = 0x003a_f0c8;
                c.g[A0][0] = list as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = 0x111 as u64;
                c.g[A3][0] = 0x222 as u64;
                c.g[T0][0] = 0x333 as u64;
                c.g[T1][0] = 0x444 as u64;
                schedule(c, &mut r, seed, 0x003a_f0c8)
            },
        );
    }
}
