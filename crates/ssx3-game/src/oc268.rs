//! Staging wave 208 (oc268): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, VSlot};
use crate::guest::Addr;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `int Renderer_VBlankStartIntc(int cause, PS2GraphicsMan* man)` (0x3825c0):
/// the VBlank-start interrupt handler: run the graphics manager's vblank
/// hook (frame counter and buffer swap signal) and re-enable interrupts;
/// any other cause is a fatal assertion (`break`) in the original.
pub fn renderer_vblank_start_intc(c: &mut Cpu) {
    const VBLANK_START: u64 = 2;
    const ON_VBLANK_START: Callee = Callee {
        addr: 0x0038_25f8,
        f: ee::s12::f_003825f8,
    };
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = VBLANK_START;
        let is_vblank = c.g[A0][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if is_vblank {
            c.cycles += 8;
            c.g[A0][0] = c.g[A1][0];
            ON_VBLANK_START.call(c, 0x0038_25dc)?;
            c.ei();
            frame.restore_ra(c, 0);
            c.g[V0][0] = 1;
            frame.close(c);
        } else {
            c.cycles += 1;
            c.brk(0x0038_25d0, 0);
        }
        Ok(())
    })
}

/// `void PS2AIVisualEffectsMan_Update(PS2AIVisualEffectsMan* self)`
/// (0x2f04b0): forward the per-frame update to the platform implementation
/// object held at `self[0]` (its virtual method in vtable slot 0x10).
pub fn ps2_ai_visual_effects_man_update(c: &mut Cpu) {
    const VT_UPDATE: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let imp = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A1][0] = imp;
        let vt = Addr(imp as u32).lw(c);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, VT_UPDATE);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = m.this(imp);
        call_ptr(c, m.func as u32, 0x002f_04d0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SongInstance_SetVolumePercent(SongInstance* self, s8 percent)`
/// (0x2b23e8): when the song is playing (`+0x54` non-zero), set the music
/// tracks' volume through `PATH_SetVolumePercent(0x11000002, percent)`.
pub fn song_instance_set_volume_percent(c: &mut Cpu) {
    const PLAYING: u32 = 0x54;
    const MUSIC_TRACK_MASK: u64 = 0x1100_0002;
    const PATH_SET_VOLUME_PERCENT: Callee = Callee {
        addr: 0x003d_5508,
        f: ee::s14::f_003d5508,
    };
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(PLAYING).lw(c);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = MUSIC_TRACK_MASK;
            PATH_SET_VOLUME_PERCENT.call(c, 0x002b_240c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimSequence_FreeTriggers(AnimSequence* self)` (0x313c08): pop and
/// `delete` every node of the trigger list at `+0xac` (linked through
/// `+0xc`), leaving the list empty.
pub fn anim_sequence_free_triggers(c: &mut Cpu) {
    const TRIGGERS: u32 = 0xac;
    const NEXT: u32 = 0xc;
    const LOOP_HEAD: u32 = 0x0031_3c28;
    const BUILTIN_DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(TRIGGERS).lw(c);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 1;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(TRIGGERS).lw(c);
            loop {
                c.cycles += 6;
                let next = Addr(c.g[A0][0] as u32).off(NEXT).lw(c);
                c.g[V0][0] = next;
                Addr(c.g[S0][0] as u32).off(TRIGGERS).sw(c, next);
                BUILTIN_DELETE.call(c, 0x0031_3c34)?;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(TRIGGERS).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                    break;
                }
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(TRIGGERS).lw(c);
                if event_due(c) {
                    deliver_event_here(c, LOOP_HEAD);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_EaxaSetInput(EaxaDecoder* dec, void* data, int size, int mode)`
/// (0x3ccf60): give an idle EA-XA decoder its input (data at `+0xa0`, size
/// at `+0xc`, mode at `+0x8`) and return 0; -1 when `data` is null or the
/// decoder is still busy (mode non-zero).
pub fn snd_eaxa_set_input(c: &mut Cpu) {
    const MODE: u32 = 0x8;
    const SIZE: u32 = 0xc;
    const DATA_PTR: u32 = 0xa0;
    c.cycles += 2;
    let dec = Addr(c.g[A0][0] as u32);
    if c.g[A1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        return ret(c);
    }
    let busy = dec.off(MODE).lw(c) != 0;
    c.cycles += 2;
    if busy {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
    } else {
        c.cycles += 4;
        c.g[V0][0] = 0;
        let (data, size, mode) = (c.g[A1][0], c.g[A2][0], c.g[A3][0]);
        dec.off(DATA_PTR).sw(c, data);
        dec.off(MODE).sw(c, mode);
        dec.off(SIZE).sw(c, size);
    }
    ret(c);
}

/// `void ScriptEngine_ToggleMoviePause(ScriptEngine* self)` (0x2789c0):
/// pause or resume the script engine's movie (`ScriptMovie_TogglePause` on
/// the movie embedded at `+0x500`).
pub fn script_engine_toggle_movie_pause(c: &mut Cpu) {
    const MOVIE: u32 = 0x500;
    const SCRIPT_MOVIE_TOGGLE_PAUSE: Callee = Callee {
        addr: 0x0028_3af8,
        f: ee::s07::f_00283af8,
    };
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(MOVIE));
        SCRIPT_MOVIE_TOGGLE_PAUSE.call(c, 0x0027_89d0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptResult WScriptFn_Crowd2dControl(int argc, WScriptArg* args)`
/// (0x303ba0, hidden result pointer in a0): the world-script command that
/// adds, removes or binds a 2D crowd instance. The arguments (16 bytes
/// each: slot index, value, -, type) fill a 5-word buffer initialised from
/// a static default (`{-1, -1, 0, 0, ..}`, set up on first use); an int
/// given for a float slot (type table 0x4465d8) is converted. `buf[0]` is
/// the object handle (-1: the default object), `buf[1]` the mode (0 bind
/// frame, 1 nothing, 2 add, 3 remove) on the crowd manager (gp-0x6f0) with
/// the object's instance handle (`+0x78`) and `buf[2]`/`buf[3]`. Returns the
/// static "no value" result.
pub fn wscript_fn_crowd2d_control(c: &mut Cpu) {
    const INIT_GUARD: u32 = 0x26f0; // gp+
    const DEFAULTS: u32 = 0x004f_bc80;
    const ARG_TYPES: u32 = 0x0044_65d8;
    const HANDLE_TABLES: u32 = 0x16c8; // gp+
    const WORLD: u32 = 0xce8; // gp+
    const DEFAULT_OBJECT: u32 = 0x290;
    const INSTANCE: u32 = 0x78;
    const CROWD_MAN: u32 = 0x6f0; // gp-
    // The static "none" result is at 0x4c9098 (0x4d0000 - 0x6f68).
    const FLOAT_ARG: u64 = 2;
    const LOOP_HEAD: u32 = 0x0030_3c40;
    const MEMCPY: Callee = Callee {
        addr: 0x003e_6574,
        f: ee::s14::f_003e6574,
    };
    const BIND_INSTANCE_FRAME: Callee = Callee {
        addr: 0x0022_9820,
        f: ee::s05::f_00229820,
    };
    const CROWD_ADD: Callee = Callee {
        addr: 0x0022_9788,
        f: ee::s05::f_00229788,
    };
    const CROWD_REMOVE: Callee = Callee {
        addr: 0x0022_97d8,
        f: ee::s05::f_002297d8,
    };
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(gp.wrapping_add(INIT_GUARD)).lw(c);
        frame.save(c, S2, 0x50);
        frame.save(c, S1, 0x60);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[A2][0] = 0x0050_0000;
        if c.g[V0][0] == 0 {
            // Guarded static init of the default argument buffer.
            c.cycles += 13;
            c.g[V1][0] = sx(DEFAULTS);
            c.g[V0][0] = u64::MAX;
            let tag = Addr(DEFAULTS + 0x10).lbu(c);
            c.g[A1][0] = tag;
            Addr(DEFAULTS).sw(c, u64::MAX);
            c.g[A0][0] = u64::MAX;
            Addr(DEFAULTS + 0x10).sb(c, tag);
            c.g[V0][0] = 1;
            Addr(DEFAULTS + 4).sw(c, u64::MAX);
            Addr(gp.wrapping_add(INIT_GUARD)).sw(c, 1);
            Addr(DEFAULTS + 8).sw(c, 0);
            Addr(DEFAULTS + 0xc).sw(c, 0);
            Addr(sp(c)).sb(c, tag);
        }
        // buf = defaults (buf[0] = -1 first).
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let buf = sp(c).wrapping_add(0x10);
        c.g[S0][0] = sx(buf);
        c.g[A1][0] = sx(DEFAULTS);
        c.g[A2][0] = 0x14;
        Addr(buf).sw(c, u64::MAX);
        c.g[A0][0] = c.g[S0][0];
        MEMCPY.call(c, 0x0030_3c24)?;
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(ARG_TYPES);
            c.g[A3][0] = FLOAT_ARG;
            loop {
                c.cycles += 8;
                let arg = Addr(c.g[A2][0] as u32);
                let slot = arg.lw(c);
                c.g[V1][0] = arg.off(0xc).lw(c);
                let off = sx((slot as u32) << 2);
                c.g[A0][0] = sx((off as u32).wrapping_add(c.g[S0][0] as u32));
                let ty_addr = (off as u32).wrapping_add(c.g[T0][0] as u32);
                c.g[V0][0] = Addr(ty_addr).lw(c);
                let dst = c.g[A0][0] as u32;
                let convert = if c.g[V1][0] == c.g[V0][0] {
                    false
                } else {
                    c.cycles += 2;
                    c.g[V0][0] == c.g[A3][0]
                };
                if convert {
                    c.cycles += 4;
                    c.lwc1(0, arg.off(4).0);
                    c.cvt_s_w(0, 0);
                    c.swc1(0, dst);
                } else {
                    let v = arg.off(4).lw(c);
                    c.g[V0][0] = v;
                    c.cycles += 1;
                    Addr(dst).sw(c, v);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if c.g[A1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOOP_HEAD);
                }
            }
        }
        // Resolve the object handle.
        c.cycles += 4;
        let buf = Addr(c.g[S0][0] as u32);
        let id = buf.lw(c);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        c.g[A1][0] = Addr(gp.wrapping_add(HANDLE_TABLES)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            let world = Addr(gp.wrapping_add(WORLD)).lw(c);
            c.g[V0][0] = Addr(world as u32).off(DEFAULT_OBJECT).lw(c);
        } else {
            c.cycles += 8;
            let tables = Addr(c.g[A1][0] as u32).lw(c);
            c.g[V1][0] = tables;
            let sets = Addr(tables as u32).off(8).lw(c);
            c.g[A0][0] = sets;
            let set_addr = (((id & 0xff) as u32) << 2).wrapping_add(sets as u32);
            c.g[V0][0] = sx(set_addr);
            let set = Addr(set_addr).lw(c);
            c.g[V1][0] = set;
            c.g[V0][0] = sx((id as u32) >> 8);
            let mut found = false;
            if set != 0 {
                c.cycles += 7;
                let entries = Addr(set as u32).off(0x1c).lw(c);
                c.g[A0][0] = entries;
                let entry_addr = ((c.g[V0][0] as u32) << 2).wrapping_add(entries as u32);
                c.g[V0][0] = sx(entry_addr);
                let entry = Addr(entry_addr).lw(c);
                c.g[V1][0] = entry;
                let index = (entry as u32) >> 8;
                c.g[V0][0] = sx(index << 2);
                found = index != 0;
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 2;
        let obj = c.g[V0][0];
        c.g[V1][0] = 0x004d_0000;
        if obj != 0 {
            c.cycles += 5;
            let mode = buf.off(4).lw(c);
            c.g[A0][0] = mode;
            c.g[A3][0] = buf.off(0xc).lw(c);
            c.g[A1][0] = Addr(obj as u32).off(INSTANCE).lw(c);
            c.g[A2][0] = buf.off(8).lw(c);
            let crowd = gp.wrapping_sub(CROWD_MAN);
            if mode == 0 {
                c.cycles += 4;
                c.g[A0][0] = Addr(crowd).lw(c);
                BIND_INSTANCE_FRAME.call(c, 0x0030_3d08)?;
                c.g[V1][0] = 0x004d_0000;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                if mode == 1 {
                    c.cycles += 1;
                    c.g[V1][0] = 0x004d_0000;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 3;
                    if mode == 2 {
                        c.cycles += 4;
                        c.g[A0][0] = Addr(crowd).lw(c);
                        CROWD_ADD.call(c, 0x0030_3d2c)?;
                        c.g[V1][0] = 0x004d_0000;
                    } else {
                        c.cycles += 2;
                        c.g[V1][0] = 0x004d_0000;
                        if mode == 3 {
                            c.cycles += 2;
                            c.g[A0][0] = Addr(crowd).lw(c);
                            CROWD_REMOVE.call(c, 0x0030_3d44)?;
                            c.cycles += 1;
                            c.g[V1][0] = 0x004d_0000;
                        }
                    }
                }
            }
        }
        // *result = the static "none" result (12 bytes, unaligned copy).
        c.cycles += 15;
        let result = c.g[S3][0] as u32;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let src = c.g[A3][0] as u32;
        c.ldl(A0, src.wrapping_add(7));
        c.ldr(A0, src);
        let tail = Addr(src).off(8).lw(c);
        c.g[A1][0] = tail;
        c.sdl(A0, result.wrapping_add(7));
        c.sdr(A0, result);
        Addr(result).off(8).sw(c, tail);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_ChannelStopVoices(int channel)` (0x3c2440): silence and release
/// every voice of a sound channel. SPU voices (flag 0x8) get their key word
/// in the voice-parameter table cleared (in output mode 2 also the mirror
/// voice `+0x18`); IOP DSP voices (flag 0x100) the word at `+0x1a0` of their
/// 16-byte block (numbered from the IOP voice base); EE voices are stopped
/// with `SND_EeVoiceStop` (numbered from the IOP + EE voice bases). Each
/// voice is then given back with `SND_ChannelRelease`. Returns 0.
pub fn snd_channel_stop_voices(c: &mut Cpu) {
    const STATE: u32 = 0x0050_a8e8;
    const SYNC: u32 = 0x0050_ad00;
    const CHANNELS: u32 = 0x1f0; // STATE+: SndChannel[]
    const CHANNEL_SIZE: u64 = 0x8c;
    const EE_VOICE_BASE: u32 = 0x2f; // STATE+: u8
    const VOICE_BASE: u32 = 0x30; // STATE+: u8
    const OUTPUT_MODE: u32 = 0x49; // STATE+: u8
    const VOICE_PARAMS: u32 = 0xcb0; // SYNC+
    const CH_VOICES: u32 = 0x04; // s16[]
    const CH_VOICE_COUNT: u32 = 0x23; // u8
    const CH_FLAGS: u32 = 0x24; // u16
    const MIRROR_VOICE: u32 = 0x18;
    const SND_CHANNEL_RELEASE: Callee = Callee {
        addr: 0x003b_a550,
        f: ee::s13::SND_ChannelRelease,
    };
    const SND_EE_VOICE_STOP: Callee = Callee {
        addr: 0x003c_8468,
        f: ee::s14::f_003c8468,
    };
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = CHANNEL_SIZE;
        frame.save(c, S2, 0x40);
        c.mult(0, A0, A0, V0);
        frame.save(c, S0, 0x60);
        c.g[V1][0] = 0x0051_0000;
        frame.save(c, S1, 0x50);
        c.g[V1][0] = sx(STATE);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let chans = Addr(STATE + CHANNELS).lw(c);
        c.g[V0][0] = chans;
        c.g[S2][0] = sx((chans as u32).wrapping_add(c.g[A0][0] as u32));
        let ch = c.g[S2][0] as u32;
        let flags = Addr(ch + CH_FLAGS).lhu(c);
        c.g[A0][0] = flags;
        c.g[V0][0] = flags & 0x100;
        let voice_count = |c: &mut Cpu| Addr(c.g[S2][0] as u32).off(CH_VOICE_COUNT).lbu(c);
        if flags & 0x8 != 0 {
            // SPU voices.
            c.cycles += 3;
            c.g[V0][0] = voice_count(c);
            c.g[S1][0] = 0;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                frame.restore(c, S0, 0x60);
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x0051_0000;
                c.g[S5][0] = c.g[V1][0];
                c.g[S3][0] = sx(SYNC);
                c.g[S4][0] = 2;
                c.g[S0][0] = sx(ch.wrapping_add(CH_VOICES));
                loop {
                    c.cycles += 10;
                    let slot = Addr(c.g[S0][0] as u32);
                    let voice = slot.lh(c);
                    let params = Addr((c.g[S3][0] as u32).wrapping_add(VOICE_PARAMS)).lw(c);
                    c.g[V0][0] = sx((voice as u32) << 3);
                    c.g[V1][0] = sx((params as u32).wrapping_add(c.g[V0][0] as u32));
                    Addr(c.g[V1][0] as u32).off(0x20).sw(c, 0);
                    c.g[A0][0] = slot.lh(c);
                    SND_CHANNEL_RELEASE.call(c, 0x003c_24c4)?;
                    c.g[V0][0] = Addr(c.g[S5][0] as u32).off(OUTPUT_MODE).lbu(c);
                    if c.g[V0][0] == c.g[S4][0] {
                        c.cycles += 7;
                        let voice = Addr(c.g[S0][0] as u32).lh(c);
                        let params = Addr((c.g[S3][0] as u32).wrapping_add(VOICE_PARAMS)).lw(c);
                        let v = sx((voice as u32).wrapping_add(MIRROR_VOICE));
                        c.g[V0][0] = sx((v as u32) << 3);
                        c.g[V1][0] = sx((params as u32).wrapping_add(c.g[V0][0] as u32));
                        Addr(c.g[V1][0] as u32).off(0x20).sw(c, 0);
                    }
                    c.g[V0][0] = voice_count(c);
                    c.cycles += 4;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                    c.g[V0][0] = more as u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                    if !more {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x60);
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_24a8);
                    }
                }
            }
        } else {
            c.cycles += 2;
            let iop = c.g[V0][0] != 0;
            c.g[V0][0] = voice_count(c);
            if iop {
                // IOP DSP voices.
                c.cycles += 2;
                c.g[S1][0] = 0;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x60);
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = 0x0051_0000;
                    c.g[S4][0] = c.g[V1][0];
                    c.g[S3][0] = sx(SYNC);
                    c.g[S0][0] = sx(ch.wrapping_add(CH_VOICES));
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lh(c);
                    loop {
                        c.cycles += 14;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        let base = Addr(c.g[S4][0] as u32).off(VOICE_BASE).lbu(c);
                        c.g[A0][0] = base;
                        let params = Addr((c.g[S3][0] as u32).wrapping_add(VOICE_PARAMS)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(base as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                        c.g[V1][0] = sx((params as u32).wrapping_add(c.g[V0][0] as u32));
                        Addr(c.g[V1][0] as u32).off(0x1a0).sw(c, 0);
                        c.g[A0][0] = Addr(c.g[S0][0] as u32).lh(c);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                        SND_CHANNEL_RELEASE.call(c, 0x003c_2550)?;
                        let count = Addr(c.g[S2][0] as u32).off(CH_VOICE_COUNT).lbu(c);
                        let more = (c.g[S1][0] as i64) < (count as i64);
                        c.g[V0][0] = more as u64;
                        if !more {
                            c.cycles += 2;
                            frame.restore(c, S0, 0x60);
                            break;
                        }
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).lh(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_2528);
                        }
                    }
                }
            } else {
                // EE voices.
                c.cycles += 2;
                c.g[S1][0] = 0;
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[S3][0] = c.g[V1][0];
                    c.g[S0][0] = sx(ch.wrapping_add(CH_VOICES));
                    c.g[V0][0] = Addr(c.g[S3][0] as u32).off(EE_VOICE_BASE).lbu(c);
                    loop {
                        c.cycles += 13;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        let base = Addr(c.g[S3][0] as u32).off(VOICE_BASE).lbu(c);
                        let voice = Addr(c.g[S0][0] as u32).lh(c);
                        c.g[V1][0] = voice;
                        let first = sx((base as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = sx((voice as u32).wrapping_sub(first as u32));
                        SND_EE_VOICE_STOP.call(c, 0x003c_2598)?;
                        c.g[A0][0] = Addr(c.g[S0][0] as u32).lh(c);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                        SND_CHANNEL_RELEASE.call(c, 0x003c_25a4)?;
                        let count = Addr(c.g[S2][0] as u32).off(CH_VOICE_COUNT).lbu(c);
                        let more = (c.g[S1][0] as i64) < (count as i64);
                        c.g[V0][0] = more as u64;
                        if !more {
                            break;
                        }
                        c.g[V0][0] = Addr(c.g[S3][0] as u32).off(EE_VOICE_BASE).lbu(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_2580);
                        }
                    }
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x60);
            }
        }
        c.cycles += 9;
        c.g[V0][0] = 0;
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

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_25c0, renderer_vblank_start_intc, "Renderer_VBlankStartIntc"),
    (0x002f_04b0, ps2_ai_visual_effects_man_update, "PS2AIVisualEffectsMan_Update"),
    (0x002b_23e8, song_instance_set_volume_percent, "SongInstance_SetVolumePercent"),
    (0x0031_3c08, anim_sequence_free_triggers, "AnimSequence_FreeTriggers"),
    (0x003c_cf60, snd_eaxa_set_input, "SND_EaxaSetInput"),
    (0x0027_89c0, script_engine_toggle_movie_pause, "ScriptEngine_ToggleMoviePause"),
    (0x0030_3ba0, wscript_fn_crowd2d_control, "WScriptFn_Crowd2dControl"),
    (0x003c_2440, snd_channel_stop_voices, "SND_ChannelStopVoices"),
];

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes;
    use crate::difftest::{check, fill, put_words, scramble, Rng, DATA};

    /// Renderer_VBlankStartIntc with cause 2 (any other cause is a fatal
    /// `break`): the real vblank hook runs with swap state 4 or not and the
    /// frame counter below/at/above the wait count, so the semaphore
    /// signal (a fake syscall, unwinding on some seeds) is reached.
    #[test]
    fn renderer_vblank_start_intc_matches() {
        const SWAP_STATE: u32 = 0x5a8c;
        const WAIT_COUNT: u32 = 0x5ab8;
        const COUNTER: u32 = 0x5abc;
        const SEMA: u32 = 0x5ac8;
        fakes::covered(
            ee::s12::f_003825c0__generated,
            renderer_vblank_start_intc,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3825_c0, 1);
                let gfx = DATA + 0x100;
                let state = if seed % 4 == 3 { r.next_u32() % 8 } else { 4 };
                let count = r.next_u32() % 6;
                let wait = match seed / 4 % 3 {
                    0 => count + 1,
                    1 => r.next_u32() % 4,
                    _ => count + 3,
                };
                c.bus.write32(gfx + SWAP_STATE, state);
                c.bus.write32(gfx + COUNTER, count);
                c.bus.write32(gfx + WAIT_COUNT, wait);
                c.bus.write32(gfx + SEMA, r.next_u32() % 64);
                c.g[A0][0] = 2;
                c.g[A1][0] = sx(gfx);
                c.pc = 0x0038_25c0;
            },
        );
    }

    /// PS2AIVisualEffectsMan_Update: the implementation object at a random
    /// address, its vtable slot 0x10 a random delta and a fake method
    /// (unwinding on some seeds).
    #[test]
    fn ps2_ai_visual_effects_man_update_matches() {
        fakes::covered(
            ee::s09::f_002f04b0__generated,
            ps2_ai_visual_effects_man_update,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2f04_b0, 1);
                let imp = fakes::ptr(&mut r);
                let vt = fakes::vtable(c, &mut r, &[0x10]);
                c.bus.write32(imp, vt);
                c.bus.write32(DATA, imp);
                c.g[A0][0] = sx(DATA);
                c.pc = 0x002f_04b0;
            },
        );
    }

    /// SongInstance_SetVolumePercent: playing or not; the real
    /// PATH_SetVolumePercent takes its short exits (not initialised, or a
    /// map selected); the volume a random 64-bit value (sign-extended byte).
    #[test]
    fn song_instance_set_volume_percent_matches() {
        const PATH_INITED: u32 = 0x0044_f42c;
        const PATH_SELECTED: u32 = 0x0044_f430;
        check(
            ee::s08::f_002b23e8__generated,
            song_instance_set_volume_percent,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2b_23e8, 0);
                let song = DATA + 0x200;
                let playing = if seed % 3 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(song + 0x54, playing);
                c.bus.write8(PATH_INITED, (seed % 2) as u8);
                c.bus.write32(PATH_SELECTED, r.next_u32() | 1);
                c.g[A0][0] = sx(song);
                c.g[A1][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
                c.pc = 0x002b_23e8;
            },
        );
    }

    /// AnimSequence_FreeTriggers: a trigger list of 0..4 nodes (the
    /// allocator's mutex set up as in the other list-freeing tests).
    #[test]
    fn anim_sequence_free_triggers_matches() {
        use crate::game::ai::aianimeventmap::tests::alloc_setup;
        check(
            ee::s10::f_00313c08__generated,
            anim_sequence_free_triggers,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x31_3c08, 0);
                alloc_setup(c, &mut r);
                let seq = DATA;
                let count = seed % 5;
                let mut head = 0;
                for k in (0..count).rev() {
                    let node = DATA + 0x1000 + 0x100 * k;
                    fill(c, &mut r, node, 4);
                    c.bus.write32(node + 0xc, head);
                    head = node;
                }
                c.bus.write32(seq + 0xac, head);
                c.g[A0][0] = sx(seq);
                c.pc = 0x0031_3c08;
            },
        );
    }

    /// SND_EaxaSetInput: null or non-null data, idle or busy decoder.
    #[test]
    fn snd_eaxa_set_input_matches() {
        check(ee::s14::f_003ccf60__generated, snd_eaxa_set_input, 200, &|c, seed| {
            let mut r = Rng(0x003c_cf60 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let dec = DATA + 0x40;
            fill(c, &mut r, dec, 0x30);
            c.bus.write32(dec + 0x8, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
            c.g[A0][0] = sx(dec);
            if seed % 5 == 0 {
                c.g[A1][0] = 0;
            }
            c.pc = 0x003c_cf60;
        });
    }

    /// ScriptEngine_ToggleMoviePause: the real ScriptMovie_TogglePause on
    /// the movie at engine + 0x500 (no player, not playing, pause or resume
    /// requested, player paused or not), as in its own test.
    #[test]
    fn script_engine_toggle_movie_pause_matches() {
        check(
            ee::s07::f_002789c0__generated,
            script_engine_toggle_movie_pause,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x27_89c0, 0);
                let engine = DATA;
                let movie = engine + 0x500;
                let player = DATA + 0x1000;
                let flag = |r: &mut Rng, k: u32| {
                    if (seed >> k) & 1 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    }
                };
                c.bus.write32(movie, if seed % 9 == 8 { 0 } else { player });
                c.bus.write32(movie + 4, if seed % 7 == 6 { 0 } else { r.next_u32() | 1 });
                c.bus.write32(movie + 8, flag(&mut r, 0));
                c.bus.write32(player + 8, flag(&mut r, 1));
                c.bus.write32(player + 0x38, flag(&mut r, 2) & (seed / 4 % 2).wrapping_neg());
                c.bus.write32(player + 0x3c, 0);
                c.g[A0][0] = sx(engine);
                c.pc = 0x0027_89c0;
            },
        );
    }

    /// WScriptFn_Crowd2dControl: the static defaults initialised or not;
    /// 0..4 extra arguments over the handle and mode arguments (or none at
    /// all), int arguments for float slots converted; the handle -1 (the
    /// world's default object, present or not), an empty handle set, an
    /// empty entry, or a found object; the modes bind (the real bind finds
    /// no instance), none, add and remove (the real crowd manager has no
    /// free or matching slot) and an unknown one.
    #[test]
    fn wscript_fn_crowd2d_control_matches() {
        use crate::difftest::GAME_GP;
        const DEFAULTS: u32 = 0x004f_bc80;
        const ARG_TYPES: u32 = 0x0044_65d8;
        const RESULT_NONE: u32 = 0x004c_9098;
        check(
            ee::s10::f_00303ba0__generated,
            wscript_fn_crowd2d_control,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x30_3ba0, 0);
                let gp = GAME_GP;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // Static defaults: fresh, or already set up (handle -1).
                let inited = seed % 2;
                w(c, gp + 0x26f0, inited);
                fill(c, &mut r, DEFAULTS, 5);
                w(c, DEFAULTS, u32::MAX);
                w(c, DEFAULTS + 4, r.next_u32() % 5);
                w(c, ARG_TYPES, 0);
                w(c, ARG_TYPES + 4, 0);
                for i in 2..5 {
                    w(c, ARG_TYPES + 4 * i, r.next_u32() % 3);
                }
                fill(c, &mut r, RESULT_NONE, 3);
                // Handle tables: gp+0x16c8 -> P -> Q; Q+8 -> sets.
                let (p, q, sets) = (DATA + 0x2000, DATA + 0x2100, DATA + 0x2200);
                let (set, entries, obj) = (DATA + 0x2800, DATA + 0x2900, DATA + 0x3000);
                let world = DATA + 0x3800;
                let crowd = DATA + 0x4000;
                w(c, gp + 0x16c8, p);
                w(c, p, q);
                w(c, q + 8, sets);
                w(c, sets + 4 * 3, set);
                w(c, sets + 4 * 7, 0);
                w(c, set + 0x1c, entries);
                let found = (obj >> 2) << 8 | (r.next_u32() & 0xff);
                w(c, entries + 4 * 5, if seed / 2 % 4 == 2 { r.next_u32() & 0xff } else { found });
                w(c, gp + 0xce8, world);
                w(c, world + 0x290, if seed % 9 == 4 { 0 } else { obj });
                // The instance handle: its set is empty (bind finds nothing).
                w(c, obj + 0x78, r.next_u32() & 0xffff_ff00 | 7);
                // Crowd manager: 64 slots, none free (-1) and none matching.
                w(c, gp.wrapping_sub(0x6f0), crowd);
                for i in 0..64 {
                    w(c, crowd + 8 + 4 * i, r.next_u32() & 0x7fff_fff0 | 0x10);
                }
                let id = match seed / 2 % 4 {
                    0 => u32::MAX,
                    1 => 5 << 8 | 7,
                    _ => 5 << 8 | 3,
                };
                let mode = match seed / 8 % 5 {
                    4 => r.next_u32() % 16,
                    m => m,
                };
                // Arguments (16 bytes: slot, value, -, type).
                let args = DATA + 0x100;
                let extra = seed / 3 % 5;
                let mut n = 0;
                let mut put = |c: &mut Cpu, slot: u32, val: u32, ty: u32| {
                    let a = args + 0x10 * n;
                    c.bus.write32(a, slot);
                    c.bus.write32(a + 4, val);
                    c.bus.write32(a + 8, 0x5555);
                    c.bus.write32(a + 0xc, ty);
                    n += 1;
                };
                put(c, 0, id, 0);
                put(c, 1, mode, 0);
                for _ in 0..extra {
                    let slot = 2 + r.next_u32() % 3;
                    let val = r.next_u32();
                    let ty = r.next_u32() % 3;
                    put(c, slot, val, ty);
                }
                let argc = match seed % 11 {
                    0 => 0,
                    5 => sx(0xffff_fffe),
                    _ => n as u64,
                };
                c.g[A0][0] = sx(DATA + 0x10);
                c.g[A1][0] = argc;
                c.g[A2][0] = sx(args);
                c.pc = 0x0030_3ba0;
            },
        );
    }

    /// SND_ChannelStopVoices: SPU (output mode 2 or not), IOP DSP and EE
    /// channels with 0..4 voices; the real SND_ChannelRelease runs on a
    /// table of 10 channels, the real SND_EeVoiceStop on EE voices whose
    /// callback list holds one element (callback none or a fake that
    /// unwinds on some seeds) and SND_MemFree takes its empty-heap exit.
    #[test]
    fn snd_channel_stop_voices_matches() {
        const STATE: u32 = 0x0050_a8e8;
        const SYNC: u32 = 0x0050_ad00;
        const EE_VOICES: u32 = 0x0051_5b40;
        const CHANNEL_SIZE: u32 = 0x8c;
        check(ee::s13::f_003c2440__generated, snd_channel_stop_voices, 300, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x3c_2440, 1);
            let chans = DATA;
            let count = 10u32;
            c.bus.write32(STATE + 0x1f0, chans);
            c.bus.write16(STATE + 0x17e, count as u16);
            for i in 0..count {
                let ch = chans + i * CHANNEL_SIZE;
                fill(c, &mut r, ch, CHANNEL_SIZE / 4);
                c.bus.write8(ch + 0x26, (r.next_u32() % 3) as u8);
                c.bus.write8(ch + 0x69, (r.next_u32() % 3) as u8);
            }
            let voice_base = r.next_u32() % 4;
            let ee_base = r.next_u32() % 3;
            c.bus.write8(STATE + 0x30, voice_base as u8);
            c.bus.write8(STATE + 0x2f, ee_base as u8);
            let mode = if seed % 2 == 0 { 2 } else { r.next_u32() as u8 | 1 };
            c.bus.write8(STATE + 0x49, mode);
            c.bus.write32(SYNC + 0xcb0, DATA + 0x1_0000);
            // SND_MemFree's heap: no blocks (it returns at once).
            c.bus.write32(STATE + 0x234, DATA + 0x5000);
            c.bus.write32(DATA + 0x5000 + 0x14, 0);
            // EE voices (0x60 bytes): random gains, one callback element.
            c.bus.write32(EE_VOICES + 0x1dc, DATA + 0x2_0000);
            c.bus.write8(EE_VOICES + 5, (r.next_u32() % 3) as u8);
            for i in 0..0x50 {
                c.bus.write32(EE_VOICES + 0xe8 + 4 * i, r.f32(1.0));
            }
            for v in 0..4 {
                let voice = DATA + 0x2_0000 + 0x60 * v;
                for i in 0..0x18 {
                    c.bus.write32(voice + 4 * i, r.f32(1.0));
                }
                let elem = DATA + 0x3_0000 + 0x20 * v;
                c.bus.write32(voice + 0x40, elem);
                c.bus.write32(elem + 4, if r.next_u32() % 2 == 0 { 0 } else { fakes::FAKE });
                c.bus.write32(elem + 8, 0);
            }
            // The channel: SPU / IOP / EE by seed % 3, 0..4 voices.
            let index = r.next_u32() % 3;
            let ch = chans + index * CHANNEL_SIZE;
            let other = r.next_u32() as u16 & !0x108;
            let flags = match seed % 3 {
                0 => other | 0x8 | (r.next_u32() as u16 & 0x100),
                1 => other | 0x100,
                _ => other,
            };
            c.bus.write16(ch + 0x24, flags);
            c.bus.write8(ch + 0x23, (seed / 3 % 5) as u8);
            for i in 0..4 {
                let voice = if seed % 3 == 2 {
                    // Distinct EE voices (a stopped voice's list is empty).
                    voice_base + ee_base + (i + seed / 15) % 4
                } else {
                    r.next_u32() % count
                };
                c.bus.write16(ch + 4 + 2 * i, voice as u16);
            }
            c.g[A0][0] = index as u64;
            c.pc = 0x003c_2440;
        });
    }
}
