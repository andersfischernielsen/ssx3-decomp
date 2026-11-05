//! Staging wave 206 (oc246): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::guest::Addr;
use ssx3_ee_rt::vu0::{Src, X};

// ------------------------------------------------------- cWorldSound_Update

/// `cWorldSound_RefreshNearbyEmitters` (0x2b75d0) and
/// `cWorldSound_ProcessEmitter` (0x2b7908).
const WS_REFRESH_NEARBY: Callee = Callee {
    addr: 0x002b_75d0,
    f: ee::s08::f_002b75d0,
};
const WS_PROCESS_EMITTER: Callee = Callee {
    addr: 0x002b_7908,
    f: ee::s08::f_002b7908,
};

/// Size of one listener's nearby-emitter list inside `cWorldSound`
/// (count at +0, emitter pointers from +4, the position it was built for
/// at +0x110).
const WS_NEAR_LIST_SIZE: u64 = 0x120;

/// `void cWorldSound_Update(cWorldSound* self, Vec4* listener, int idx)`
/// (0x2b7848), every frame per listener: refresh the listener's list of
/// nearby emitters (`self + idx * 0x120`) around `listener`, then run
/// `cWorldSound_ProcessEmitter(self, listener, idx, emitter)` for each
/// non-null entry. Frame 0x80 (s0-s6); the loop keeps its state in s0-s3
/// (s3 = list, s0/s1 = entry cursor, s2 = index).
pub fn c_world_sound_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = WS_NEAR_LIST_SIZE;
        frame.save(c, S6, 0x10);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S5, 0x20);
        c.mult(0, V1, S6, V0);
        frame.save(c, S0, 0x70);
        frame.save(c, S4, 0x30);
        c.g[S5][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S2, 0x50);
        frame.save(c, S1, 0x60);
        // The listener's list: self + idx * 0x120.
        c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, S5)));
        frame.save(c, S3, 0x40);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        WS_REFRESH_NEARBY.call(c, 0x002b_788c)?;

        let count = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = count;
        c.g[S2][0] = 0;
        if (count as i64) > 0 {
            c.cycles += 4;
            c.g[S3][0] = c.g[S0][0];
            c.g[S0][0] = sx(r32(c, S3).wrapping_add(4));
            c.g[S1][0] = c.g[S0][0];
            loop {
                c.cycles += 3;
                let entry = Addr(r32(c, S0)).lw(c);
                c.g[V0][0] = entry;
                c.g[A0][0] = c.g[S5][0];
                if entry != 0 {
                    // ProcessEmitter(self, listener, idx, entry)
                    c.cycles += 4;
                    c.g[A3][0] = Addr(r32(c, S1)).lw(c);
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S6][0];
                    WS_PROCESS_EMITTER.call(c, 0x002b_78c4)?;
                }
                c.cycles += 6;
                let count = Addr(r32(c, S3)).lw(c);
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                let more = (c.g[S2][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_78a8);
                }
            }
        }

        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------ ModifierBlock_SetTransformBounds

/// `sBoundBoxNode` vtable slot 13 (`setBoundBox`): this-delta (s16) at
/// +0x68, method at +0x6c.
const MB_SET_BOUND_BOX_SLOT: u32 = 0x68;

/// `void ModifierBlock_SetTransformBounds(tModifierBlock* self,
/// BoundBox* box)` (0x352bc0): forward the box to the block's bound node
/// (`self->node`, at +0) through its virtual `setBoundBox`, if it has one.
/// Frame 0x10 (ra only; reloaded before the null test as the original does).
pub fn modifier_block_set_transform_bounds(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let node = Addr(r32(c, A0)).lw(c);
        c.g[A2][0] = node;
        frame.restore_ra(c, 0);
        if node != 0 {
            c.cycles += 6;
            let node = node as u32;
            let vt = Addr(node).lw(c);
            c.g[V0][0] = vt;
            let delta = Addr((vt as u32).wrapping_add(MB_SET_BOUND_BOX_SLOT)).lh(c);
            let method = Addr((vt as u32).wrapping_add(MB_SET_BOUND_BOX_SLOT + 4)).lw(c);
            c.g[V1][0] = method;
            // node->setBoundBox(box): a1 passes through.
            c.g[A0][0] = sx(node.wrapping_add(delta as u32));
            call_ptr(c, method as u32, 0x0035_2be8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------- SSXAudioSystem_UpdateUberSpeech

/// `SSXAudioSystem` fields: the rider id the uber-speech state belongs to
/// (+0x5820) and the rider's last seen trick/boost state (+0x581c).
const UBER_RIDER_ID: u32 = 0x5820;
const UBER_LAST_STATE: u32 = 0x581c;
/// Rider fields: player id (+0x870) and the state watched for "uber" (10).
const UBER_RIDER_PLAYER_ID: u32 = 0x870;
const UBER_RIDER_STATE: u32 = 0x2f4;
/// Game module: game mode (+0x214); uber speech only in mode 4.
const UBER_MODULE_MODE: u32 = 0x214;

/// `SSXAudioSystem_ArcadeSay2133` (0x2a3de0).
const UBER_ARCADE_SAY: Callee = Callee {
    addr: 0x002a_3de0,
    f: ee::s08::f_002a3de0,
};

/// `void SSXAudioSystem_UpdateUberSpeech(SSXAudioSystem* self)` (0x29b738),
/// every frame: outside replays, find the followed rider
/// (`FindRider(self, -1)`); when it is the rider tracked at +0x5820 and its
/// state (+0x2f4) just became 10 (uber) in game mode 4, say line 0x2133
/// (`ArcadeSay2133(self, rider, 2)`); a different rider is tracked from now
/// on. The rider's state is remembered at +0x581c. Frame 0x30 (s0, s1).
pub fn ssx_audio_update_uber_speech(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        // SSXAudioSystem_IsReplayPlaying(self)
        call_ptr(c, 0x0028_8ae0, 0x0029_b750)?;
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = u64::MAX;
            // SSXAudioSystem_FindRider(self, -1)
            call_ptr(c, 0x0028_5d98, 0x0029_b764)?;
            c.g[S1][0] = c.g[V0][0];
            let this = r32(c, S0);
            let rider = r32(c, S1);
            let tracked = Addr(this.wrapping_add(UBER_RIDER_ID)).lw(c);
            c.g[V0][0] = tracked;
            let id = Addr(rider.wrapping_add(UBER_RIDER_PLAYER_ID)).lw(c);
            c.g[V1][0] = id;
            if id != tracked {
                // Another rider: track it.
                Addr(this.wrapping_add(UBER_RIDER_ID)).sw(c, id);
                c.cycles += 1;
            } else {
                c.cycles += 4;
                let state = Addr(rider.wrapping_add(UBER_RIDER_STATE)).lw(c);
                c.g[V1][0] = state;
                c.g[V0][0] = 10;
                if state == 10 {
                    c.cycles += 3;
                    let last = Addr(this.wrapping_add(UBER_LAST_STATE)).lw(c);
                    c.g[V0][0] = last;
                    if last != state {
                        c.cycles += 6;
                        // SSXAudioSystem_GetGameModule()
                        call_ptr(c, 0x0028_b1c8, 0x0029_b79c)?;
                        let mode = Addr(r32(c, V0).wrapping_add(UBER_MODULE_MODE)).lw(c);
                        c.g[V1][0] = mode;
                        c.g[V0][0] = 4;
                        if mode == 4 {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = 2;
                            UBER_ARCADE_SAY.call(c, 0x0029_b7bc)?;
                        }
                    }
                }
            }
            let state = Addr(r32(c, S1).wrapping_add(UBER_RIDER_STATE)).lw(c);
            c.g[V0][0] = state;
            c.cycles += 2;
            Addr(r32(c, S0).wrapping_add(UBER_LAST_STATE)).sw(c, state);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------- ScriptEngine_UpdateProcesses

/// `ScriptEngine` layout: the sequence player (+0x2b8) and sixteen 0x24-byte
/// process slots from +0x2c0 (type at +0x4, state at +0x20; 1 = pending).
const SE_SEQ_PLAYER: u32 = 0x2b8;
const SE_SLOTS: u32 = 0x2c0;
const SE_SLOT_SIZE: u64 = 0x24;
const SE_SLOT_TYPE: u32 = 0x4;
const SE_SLOT_STATE: u32 = 0x20;
/// The slot-type switch's jump table (six entries, `.rodata`).
const SE_TYPE_TABLE: u32 = 0x0048_1f70;

/// `InSeq_UpdateAll` (0x2728f8) and `ScriptEngine_LaunchPendingProcess`
/// (0x282908).
const SE_IN_SEQ_UPDATE_ALL: Callee = Callee {
    addr: 0x0027_28f8,
    f: ee::s07::f_002728f8,
};
const SE_LAUNCH_PENDING: Callee = Callee {
    addr: 0x0028_2908,
    f: ee::s07::f_00282908,
};

/// `void ScriptEngine_UpdateProcesses(ScriptEngine* self)` (0x2820b0), every
/// frame: update all running sequences, then launch each pending process
/// slot (state 1) of type 1 (`LaunchPendingProcess(self, slot)`; the other
/// types are no-ops). The type switch goes through the original's jump
/// table: its targets are the launch (0x282120), the skip (0x282128) and,
/// as the generated code allows, a restart at the top; anything else leaves
/// through `jump_out`. Frame 0x30 (s0 = slot index, s1 = self). Other
/// entries (resume points) go to the generated `__entries` twin.
pub fn script_engine_update_processes(c: &mut Cpu) {
    guest(c, |c| {
        'restart: loop {
            c.cycles += 10;
            let frame = Frame::open(c, 0x30);
            frame.save(c, S0, 0x20);
            frame.save(c, S1, 0x10);
            c.g[S0][0] = 0;
            frame.save_ra(c, 0);
            c.g[S1][0] = c.g[A0][0];
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(SE_SEQ_PLAYER)).lw(c);
            SE_IN_SEQ_UPDATE_ALL.call(c, 0x0028_20d0)?;
            c.g[V0][0] = SE_SLOT_SIZE;
            loop {
                c.cycles += 6;
                c.mult(0, V0, S0, V0);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(SE_SLOTS));
                let slot = r32(c, S1).wrapping_add(r32(c, V0));
                c.g[A1][0] = sx(slot);
                let state = Addr(slot.wrapping_add(SE_SLOT_STATE)).lw(c);
                c.g[V1][0] = state;
                c.g[V0][0] = 1;
                // Whether the slot ends at the shared `s0++` (1 cycle).
                let mut next = true;
                if state != 0 {
                    c.cycles += 2;
                    if state != 1 {
                        next = false;
                    } else {
                        c.cycles += 4;
                        let ty = Addr(slot.wrapping_add(SE_SLOT_TYPE)).lw(c);
                        c.g[V1][0] = ty;
                        let known = ty < 6;
                        c.g[V0][0] = known as u64;
                        c.g[V0][0] = 0x0048_0000;
                        if known {
                            c.cycles += 6;
                            c.g[V1][0] = sx((ty as u32) << 2);
                            c.g[V0][0] = sx(SE_TYPE_TABLE);
                            let entry = SE_TYPE_TABLE.wrapping_add(r32(c, V1));
                            c.g[V1][0] = sx(entry);
                            let target = Addr(entry).lw(c);
                            c.g[A0][0] = target;
                            match target as u32 {
                                0x0028_20b0 => {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0028_20b0);
                                    }
                                    continue 'restart;
                                }
                                0x0028_2120 => {
                                    c.cycles += 2;
                                    c.g[A0][0] = c.g[S1][0];
                                    SE_LAUNCH_PENDING.call(c, 0x0028_2128)?;
                                }
                                0x0028_2128 => {}
                                t => {
                                    c.jump_out(t, 0x0028_20b0, 0x0028_214c);
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
                if next {
                    c.cycles += 1;
                }
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.cycles += 3;
                let more = (c.g[S0][0] as i64) < 16;
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                c.g[V0][0] = SE_SLOT_SIZE;
                if event_due(c) {
                    deliver_event_here(c, 0x0028_20d8);
                }
            }
            c.cycles += 5;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

// ---------------------------------------------------------- SNDSTRM_ParseData

/// Stream decoder state (`self + 0x134`): byte-swap flag (+0x25, set for
/// big-endian data), "block pushed" flag (+0x24), the sample clock (+0x1c)
/// and the block tag (+0xc).
const SS_STATE: u32 = 0x134;
const SS_SWAP: u32 = 0x25;
const SS_PUSHED: u32 = 0x24;
const SS_CLOCK: u32 = 0x1c;
const SS_TAG: u32 = 0xc;
/// Stream: file handle (+0), stream index (+0x8), channel count (+0x16).
const SS_CHANNELS: u32 = 0x16;
/// Data chunk: sample count (+0x8), per-channel offsets from +0xc.
const SS_CHUNK_SAMPLES: u32 = 0x8;
const SS_CHUNK_OFFSETS: u32 = 0xc;

/// `SNDSTRM_PushBlock` (0x3c6ab8).
const SS_PUSH_BLOCK: Callee = Callee {
    addr: 0x003c_6ab8,
    f: ee::s13::f_003c6ab8,
};

/// `void SNDSTRM_ParseData(SndStream* self, Chunk* chunk)` (0x3b6ea8): turn
/// an `SCDl` data chunk into a block descriptor on the stack (a 64-bit word
/// at sp+0: low 32 bits kept, bits 32..62 the sample count, bit 63 "first
/// block pushed"; then a pointer per channel at sp+0xc..): the sample count
/// and each channel's offset (byte-swapped when the stream is big-endian)
/// relative to the end of the offset table. An empty chunk is released
/// back to the file (`StreamFile_ReleaseChunk`); otherwise the chunk is
/// remembered just before the first channel's data, its first word gets the
/// stream's tag, the clock advances by the sample count and the block goes
/// to `SNDSTRM_PushBlock(stream index, &block)`. Frame 0x60 (s0, ra at
/// +0x40).
pub fn sndstrm_parse_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        c.g[T4][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        c.g[T3][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        let this = r32(c, T4);
        let chunk = r32(c, T3);
        let state = Addr(this.wrapping_add(SS_STATE)).lw(c);
        c.g[S0][0] = state;
        let state = state as u32;
        let swap = Addr(state.wrapping_add(SS_SWAP)).lbu(c);
        c.g[V0][0] = swap;
        let s = sp(c);
        c.g[A0][0] = sx(s.wrapping_add(0x34));
        // The low 31 bits / the rest of the block descriptor word.
        const LOW31: u64 = 0x7fff_ffff;
        const KEEP: u64 = 0x8000_0000_ffff_ffff;
        if swap != 0 {
            // Byte-swap the sample count through sp+0x34 -> sp+0x30.
            c.cycles += 26;
            let n = Addr(chunk.wrapping_add(SS_CHUNK_SAMPLES)).lw(c);
            c.g[V0][0] = n;
            c.g[A1][0] = sx(s.wrapping_add(0x30));
            c.g[A3][0] = LOW31;
            Addr(s.wrapping_add(0x34)).sw(c, n);
            c.g[A2][0] = KEEP;
            let b3 = Addr(s.wrapping_add(0x37)).lbu(c);
            Addr(s.wrapping_add(0x30)).sb(c, b3);
            let b2 = Addr(s.wrapping_add(0x36)).lbu(c);
            c.g[V1][0] = b2;
            Addr(s.wrapping_add(0x31)).sb(c, b2);
            let b1 = Addr(s.wrapping_add(0x35)).lbu(c);
            c.g[V0][0] = b1;
            Addr(s.wrapping_add(0x32)).sb(c, b1);
            let b0 = Addr(s.wrapping_add(0x34)).lbu(c);
            c.g[V1][0] = b0;
            Addr(s.wrapping_add(0x33)).sb(c, b0);
            let count = Addr(s.wrapping_add(0x30)).lwu(c) & LOW31;
            let word = Addr(s).ld(c) & KEEP;
            c.g[V0][0] = count << 32;
            let word = word | count << 32;
            c.g[V1][0] = word;
            Addr(s).sd(c, word);
        } else {
            c.cycles += 14;
            let count = Addr(chunk.wrapping_add(SS_CHUNK_SAMPLES)).lwu(c) & LOW31;
            c.g[A1][0] = LOW31;
            c.g[A0][0] = KEEP;
            c.g[V1][0] = count << 32;
            let word = (Addr(s).ld(c) & KEEP) | count << 32;
            c.g[V0][0] = word;
            Addr(s).sd(c, word);
        }

        // Channel pointers: data start (after the offset table) + offset.
        c.cycles += 5;
        let channels = Addr(this.wrapping_add(SS_CHANNELS)).lbu(c);
        c.g[V1][0] = channels;
        let offsets = chunk.wrapping_add(SS_CHUNK_OFFSETS);
        c.g[A0][0] = sx(offsets);
        c.g[V0][0] = sx((channels as u32) << 2);
        let data = offsets.wrapping_add((channels as u32) << 2);
        c.g[T2][0] = sx(data);
        if channels != 0 {
            c.cycles += 6;
            c.g[A2][0] = sx(offsets);
            c.g[T1][0] = channels;
            c.g[T0][0] = sx(s.wrapping_add(0x30));
            c.g[A3][0] = sx(s.wrapping_add(0x38));
            c.g[A1][0] = sx(s.wrapping_add(0xc));
            loop {
                c.cycles += 3;
                let s = sp(c);
                let swap = Addr(r32(c, S0).wrapping_add(SS_SWAP)).lbu(c);
                let mut off = Addr(r32(c, A2)).lw(c);
                c.g[V0][0] = off;
                if swap != 0 {
                    // Byte-swap through sp+0x38 -> sp+0x30.
                    c.cycles += 10;
                    Addr(s.wrapping_add(0x38)).sw(c, off);
                    let b3 = Addr(s.wrapping_add(0x3b)).lbu(c);
                    Addr(s.wrapping_add(0x30)).sb(c, b3);
                    let b2 = Addr(s.wrapping_add(0x3a)).lbu(c);
                    c.g[V0][0] = b2;
                    Addr(s.wrapping_add(0x31)).sb(c, b2);
                    let b1 = Addr(s.wrapping_add(0x39)).lbu(c);
                    c.g[V1][0] = b1;
                    Addr(s.wrapping_add(0x32)).sb(c, b1);
                    let b0 = Addr(s.wrapping_add(0x38)).lbu(c);
                    c.g[A0][0] = b0;
                    Addr(s.wrapping_add(0x33)).sb(c, b0);
                    off = Addr(s.wrapping_add(0x30)).lw(c);
                }
                c.cycles += 6;
                let ptr = sx(r32(c, T2).wrapping_add(off as u32));
                c.g[V0][0] = ptr;
                Addr(r32(c, A1)).sw(c, ptr);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
                c.g[T1][0] = sx(r32(c, T1).wrapping_sub(1));
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
                if c.g[T1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_6f98);
                }
            }
        }

        c.cycles += 6;
        let s = sp(c);
        let count = Addr(s.wrapping_add(4)).lw(c) & LOW31;
        c.g[V0][0] = count;
        c.g[A2][0] = LOW31;
        let first = Addr(s.wrapping_add(0xc)).lw(c);
        c.g[A0][0] = first;
        if count == 0 {
            // StreamFile_ReleaseChunk(file, chunk)
            c.cycles += 5;
            c.g[A0][0] = Addr(this).lw(c);
            c.g[A1][0] = c.g[T3][0];
            call_ptr(c, 0x003e_13e8, 0x003b_7008)?;
        } else {
            c.cycles += 24;
            c.g[A3][0] = u64::MAX >> 1;
            c.g[A1][0] = c.g[SP][0];
            // The chunk, just before the first channel's data.
            let chunk_reg = c.g[T3][0];
            Addr((first as u32).wrapping_sub(4)).sw(c, chunk_reg);
            let tag = Addr(state.wrapping_add(SS_TAG)).lw(c);
            c.g[V1][0] = tag;
            let count = Addr(s.wrapping_add(4)).lwu(c);
            Addr(chunk).sw(c, tag);
            let count = (((count << 32) as i64) >> 32) as u64 & LOW31;
            c.g[V0][0] = count;
            let clock = Addr(state.wrapping_add(SS_CLOCK)).lw(c);
            let pushed = Addr(state.wrapping_add(SS_PUSHED)).lbu(c);
            let clock = sx((clock as u32).wrapping_add(count as u32));
            c.g[V1][0] = clock;
            Addr(state.wrapping_add(SS_CLOCK)).sw(c, clock);
            c.g[A2][0] = pushed << 63;
            let word = (Addr(s).ld(c) & (u64::MAX >> 1)) | pushed << 63;
            c.g[V0][0] = word;
            c.g[A0][0] = Addr(this.wrapping_add(0x8)).lw(c);
            Addr(s).sd(c, word);
            SS_PUSH_BLOCK.call(c, 0x003b_7064)?;
            c.g[V1][0] = 1;
            Addr(r32(c, S0).wrapping_add(SS_PUSHED)).sb(c, 1);
        }
        frame.restore(c, S0, 0x50);
        c.cycles += 3;
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------------------- Rider_GetTrickSkillFactor

/// Rider fields: AI (player) index (+0x86c) and stats index (+0xb34).
const RIDER_AI_INDEX: u32 = 0x86c;
const RIDER_STATS_INDEX: u32 = 0xb34;
/// `.lit4` scale of the trick attribute factor (0.29988...).
const GP_TRICK_SKILL_SCALE: u32 = 0x7960;

/// `float Rider_GetTrickSkillFactor(Rider* self)` (0x120038): the rider's
/// trick attribute as a factor, `1 + 0.2999 * attrib2Factor`, where the
/// attribute factor comes from the back end's character interface
/// (`cBE_getInterface(cBE_getBE(), 3)`,
/// `BECharInt_GetAttrib2Factor(iface, aiIndex, statsIndex)`). Frame 0x20
/// (s0).
pub fn rider_get_trick_skill_factor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        // cBE_getBE()
        call_ptr(c, 0x0014_dc80, 0x0012_004c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        // cBE_getInterface(be, 3)
        call_ptr(c, 0x0014_dd58, 0x0012_0058)?;
        let rider = r32(c, S0);
        c.g[A2][0] = Addr(rider.wrapping_add(RIDER_STATS_INDEX)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(rider.wrapping_add(RIDER_AI_INDEX)).lw(c);
        // BECharInt_GetAttrib2Factor(iface, aiIndex, statsIndex)
        call_ptr(c, 0x0014_9690, 0x0012_0068)?;
        let gp = r32(c, GP);
        c.lwc1(1, gp.wrapping_sub(GP_TRICK_SKILL_SCALE));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        c.mul_s(0, 0, 1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.add_s(0, 0, 2);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------- GameModule_RenderFogParticles

/// `GameModule`: the fog particle manager (+0x18; vtable at +0x4, slot 2
/// `renderFog` at +0x10/+0x14).
const GM_FOG_MAN: u32 = 0x18;
const FOG_VTABLE: u32 = 0x4;
const FOG_RENDER_SLOT: u32 = 0x10;
/// World: the two fog volume lists (count, then the volumes): kind 0 at
/// +0x78b4/+0x78b8, kind 2 at +0x7abc/+0x7ac0.
const WORLD_FOG_LISTS: [(u32, u32, u64, u32); 2] =
    [(0x78b4, 0x78b8, 0, 0x0022_c748), (0x7abc, 0x7ac0, 2, 0x0022_c770)];

/// `void GameModule_RenderFogParticles(GameModule* self, World* world)`
/// (0x22c708): hand each non-empty fog volume list of the world to the fog
/// particle manager (`fogMan->renderFog(volumes, count, kind)`, kinds 0 and
/// 2), then let it finish the frame (`cPS2FogParticleMan` 0x2dbf98).
/// Frame 0x30 (s0 = world, s1 = self).
pub fn game_module_render_fog_particles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        for (i, &(count_off, list_off, kind, ret)) in WORLD_FOG_LISTS.iter().enumerate() {
            if i > 0 {
                c.cycles += 3;
            }
            let world = r32(c, S0);
            let count = Addr(world.wrapping_add(count_off)).lw(c);
            c.g[A2][0] = count;
            c.g[A1][0] = sx(world.wrapping_add(list_off));
            if count != 0 {
                c.cycles += 7;
                let man = Addr(r32(c, S1).wrapping_add(GM_FOG_MAN)).lw(c);
                c.g[T0][0] = man;
                c.g[A3][0] = kind;
                let vt = Addr((man as u32).wrapping_add(FOG_VTABLE)).lw(c);
                c.g[V0][0] = vt;
                let delta = Addr((vt as u32).wrapping_add(FOG_RENDER_SLOT)).lh(c);
                let method = Addr((vt as u32).wrapping_add(FOG_RENDER_SLOT + 4)).lw(c);
                c.g[V1][0] = method;
                c.g[A0][0] = sx((man as u32).wrapping_add(delta as u32));
                call_ptr(c, method as u32, ret)?;
            }
        }
        c.cycles += 7;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(GM_FOG_MAN)).lw(c);
        call_ptr(c, 0x002d_bf98, 0x0022_c778)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------- RiderSphereTree_CollideBox

/// Collision sphere: centre (+0x10), radius (+0x20), sub-sphere mask
/// (+0x28) and count (+0x2c).
const RST_CENTRE: u32 = 0x10;
const RST_RADIUS: u32 = 0x20;
const RST_MASK: u32 = 0x28;
const RST_COUNT: u32 = 0x2c;

/// `Collide_SphereVsBoxEdges` (0x329590).
const RST_SPHERE_VS_BOX: Callee = Callee {
    addr: 0x0032_9590,
    f: ee::s10::f_00329590,
};

/// `-1.0f`, the "no hit yet" distance.
const RST_NO_HIT: u64 = 0xffff_ffff_bf80_0000;

/// `bool RiderSphereTree_CollideBox(Sphere* self, Vec4* boxMin,
/// Vec4* boxMax, Vec4* outPos, Vec4* outNormal)` (0x32b2b8): sweep the
/// sphere against the box (`Collide_SphereVsBoxEdges(min, max, &centre,
/// &normal, &dir, &t, radius)`, results on the stack: normal at sp+0,
/// dir at sp+0x10, t at sp+0x40). Without a sub-sphere mask the first hit
/// is the answer (`outPos = dir * t`, `outNormal = normal`). Otherwise the
/// sweep is repeated once per set mask bit below the count and the hit with
/// the largest t is kept (f20 = best, from -1); true if any. The scaling is
/// a VU0 `vmul.xyzw` by the broadcast t. Frame 0x100 (s0-s7, fp, f20).
pub fn rider_sphere_tree_collide_box(c: &mut Cpu) {
    /// `outPos = dir * t; outNormal = normal` (also dir * t at sp+0x20 when
    /// `copy`), through `t_reg` (mfc1) and the given GPR pair.
    fn store_hit(c: &mut Cpu, t_reg: usize, pos_reg: usize, nrm_reg: usize, copy: bool) {
        let s = sp(c);
        c.mfc1(t_reg, 0);
        c.lqc2(4, s.wrapping_add(0x10));
        c.qmtc2(t_reg, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s.wrapping_add(0x30));
        c.lq(pos_reg, s.wrapping_add(0x30));
        c.lq(nrm_reg, s);
        let out_pos = r32(c, S4);
        let out_nrm = r32(c, S5);
        c.sq(pos_reg, out_pos);
        c.sq(nrm_reg, out_nrm);
        if copy {
            c.sq(pos_reg, s.wrapping_add(0x20));
        }
    }

    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x100);
        let s = sp(c);
        frame.save(c, S1, 0xd0);
        c.g[T1][0] = sx(s.wrapping_add(0x40));
        frame.save(c, S2, 0xc0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0xb0);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0xa0);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S5, 0x90);
        c.g[S4][0] = c.g[A3][0];
        frame.save(c, S7, 0x70);
        c.g[S5][0] = c.g[T0][0];
        frame.save(c, FP, 0x60);
        c.g[T0][0] = sx(s.wrapping_add(0x10));
        frame.save(c, S0, 0xe0);
        c.g[FP][0] = c.g[T0][0];
        frame.save(c, S6, 0x80);
        c.g[S7][0] = c.g[T1][0];
        frame.save_ra(c, 0x50);
        c.g[A0][0] = c.g[S3][0];
        frame.save_fpr(c, 20, 0xf0);
        c.g[A1][0] = c.g[S2][0];
        let this = r32(c, S1);
        c.g[A2][0] = sx(this.wrapping_add(RST_CENTRE));
        c.g[A3][0] = sx(s);
        c.lwc1(12, this.wrapping_add(RST_RADIUS));
        RST_SPHERE_VS_BOX.call(c, 0x0032_b324)?;

        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            let this = r32(c, S1);
            let mask = Addr(this.wrapping_add(RST_MASK)).lw(c);
            c.g[V0][0] = mask;
            c.cycles += 2;
            if mask == 0 {
                // The single sphere: its hit.
                c.cycles += 13;
                c.lwc1(0, sp(c).wrapping_add(0x40));
                c.g[V0][0] = 1;
                store_hit(c, V1, A0, V1, true);
            } else {
                let count = Addr(this.wrapping_add(RST_COUNT)).lw(c);
                c.g[V0][0] = count;
                c.cycles += 4;
                c.g[AT][0] = RST_NO_HIT;
                c.mtc1(AT, 20);
                c.g[S0][0] = 0;
                if (count as i64) > 0 {
                    c.cycles += 2;
                    c.g[S6][0] = 1;
                    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(RST_MASK)).lw(c);
                    loop {
                        c.cycles += 4;
                        let bit = sx(r32(c, S6) << (r32(c, S0) & 31));
                        c.g[V1][0] = bit;
                        c.g[V0][0] &= bit;
                        c.g[A0][0] = c.g[S3][0];
                        // Whether the iteration ends through the 1-cycle
                        // `v0 = count` block.
                        let mut tail = true;
                        if c.g[V0][0] != 0 {
                            c.cycles += 9;
                            let this = r32(c, S1);
                            c.lwc1(12, this.wrapping_add(RST_RADIUS));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = sx(this.wrapping_add(RST_CENTRE));
                            c.g[A3][0] = c.g[SP][0];
                            c.g[T0][0] = c.g[FP][0];
                            c.g[T1][0] = c.g[S7][0];
                            RST_SPHERE_VS_BOX.call(c, 0x0032_b3b4)?;
                            let hit = c.g[V0][0] != 0;
                            c.lwc1(0, sp(c).wrapping_add(0x40));
                            if hit {
                                c.cycles += 4;
                                c.c_lt_s(20, 0);
                                if c.fcc() {
                                    // Farther than the best so far.
                                    c.cycles += 11;
                                    store_hit(c, V0, V1, V0, true);
                                    c.lwc1(20, sp(c).wrapping_add(0x40));
                                } else {
                                    tail = false;
                                }
                            }
                        }
                        if tail {
                            c.cycles += 1;
                        }
                        let count = Addr(r32(c, S1).wrapping_add(RST_COUNT)).lw(c);
                        c.cycles += 4;
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        let more = (c.g[S0][0] as i64) < (count as i64);
                        c.g[V0][0] = more as u64;
                        if !more {
                            break;
                        }
                        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(RST_MASK)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0032_b388);
                        }
                    }
                }
                // Any hit: best != -1.
                c.cycles += 6;
                c.g[AT][0] = RST_NO_HIT;
                c.mtc1(AT, 0);
                c.c_eq_s(20, 0);
                c.g[V0][0] = 1;
                if c.fcc() {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
            }
        }

        c.cycles += 13;
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------------------- SND_RunServiceCallbacks

/// Sound driver globals (0x50a8e8): service callbacks enabled (s8, +0x174),
/// their count (s8, +0x178) and the function table (+0x19c).
const SND_GLOBALS: u32 = 0x0050_a8e8;
const SND_SERVICE_ON: u32 = 0x174;
const SND_SERVICE_COUNT: u32 = 0x178;
const SND_SERVICE_FNS: u32 = 0x19c;

/// `void SND_RunServiceCallbacks(void)` (0x3c60e0), from the sound service
/// tick: when enabled, call each registered service callback in order (the
/// count is re-read after every call). Frame 0x40 (s0 = index, s1 = table
/// cursor, s2 = globals); s0 is reloaded once right at the start (the
/// original's delay slot) and again at the end.
pub fn snd_run_service_callbacks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0051_0000;
        frame.save(c, S0, 0x30);
        c.g[A0][0] = sx(SND_GLOBALS);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let on = Addr(SND_GLOBALS + SND_SERVICE_ON).lb(c);
        c.g[V1][0] = on;
        frame.restore(c, S0, 0x30);
        if on != 0 {
            c.cycles += 3;
            let count = Addr(SND_GLOBALS + SND_SERVICE_COUNT).lb(c);
            c.g[V0][0] = count;
            c.g[S0][0] = 0;
            if (count as i64) > 0 {
                c.cycles += 3;
                c.g[S2][0] = c.g[A0][0];
                c.g[S1][0] = sx(SND_GLOBALS + SND_SERVICE_FNS);
                c.g[V1][0] = Addr(r32(c, S1)).lw(c);
                loop {
                    c.cycles += 7;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    let callback = r32(c, V1);
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                    call_ptr(c, callback, 0x003c_612c)?;
                    let count = Addr(r32(c, S2).wrapping_add(SND_SERVICE_COUNT)).lb(c);
                    let more = (c.g[S0][0] as i64) < (count as i64);
                    c.g[V0][0] = more as u64;
                    if !more {
                        break;
                    }
                    c.g[V1][0] = Addr(r32(c, S1)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_6120);
                    }
                }
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
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_7848, c_world_sound_update, "cWorldSound_Update"),
    (
        0x0035_2bc0,
        modifier_block_set_transform_bounds,
        "ModifierBlock_SetTransformBounds",
    ),
    (
        0x0029_b738,
        ssx_audio_update_uber_speech,
        "SSXAudioSystem_UpdateUberSpeech",
    ),
    (
        0x0028_20b0,
        script_engine_update_processes,
        "ScriptEngine_UpdateProcesses",
    ),
    (0x003b_6ea8, sndstrm_parse_data, "SNDSTRM_ParseData"),
    (
        0x0012_0038,
        rider_get_trick_skill_factor,
        "Rider_GetTrickSkillFactor",
    ),
    (
        0x0022_c708,
        game_module_render_fog_particles,
        "GameModule_RenderFogParticles",
    ),
    (
        0x0032_b2b8,
        rider_sphere_tree_collide_box,
        "RiderSphereTree_CollideBox",
    ),
    (
        0x003c_60e0,
        snd_run_service_callbacks,
        "SND_RunServiceCallbacks",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, fill, put_vec, put_words, scramble, Rng, DATA, GAME_GP};

    /// cWorldSound_Update: listener lists 0..2 with 0..5 entries (and a
    /// negative count), null and non-null entries; the real
    /// RefreshNearbyEmitters either keeps the list (listener moved less than
    /// 10) or rebuilds it from an empty tree (count 0), and the real
    /// ProcessEmitter runs on emitters whose data blocks have no records.
    #[test]
    fn c_world_sound_update_matches() {
        const SELF: u32 = DATA;
        const LISTENER: u32 = DATA + 0x800;
        const SOURCES: u32 = DATA + 0x1000;
        const WORLD: u32 = DATA + 0x2000;
        const MODULE: u32 = DATA + 0x2100;
        const TREE: u32 = DATA + 0x2200;
        check(ee::s08::f_002b7848__generated, c_world_sound_update, 300, &|c, seed| {
            let mut r = Rng(0x2b78_4800 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            c.g[GP][0] = sx(GAME_GP);
            c.pc = 0x002b_7848;
            // World -> sound module -> emitter tree with no subtrees.
            c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
            c.bus.write32(WORLD + 0x84, MODULE);
            c.bus.write32(MODULE + 0x20, TREE);
            c.bus.write32(GAME_GP.wrapping_sub(0x452c), r.f32(2.0));
            for k in 0..8 {
                put_words(c, TREE + 0x14 * k, &[1, 0, 0, 0, 0]);
            }
            let idx = seed % 3;
            let list = SELF + 0x120 * idx;
            let p = [r.f32(20.0), r.f32(20.0), r.f32(20.0), r.f32(1.0)];
            put_vec(c, LISTENER, p);
            // Built for a position 3 (kept) or 12 (rebuilt) away; flag 0
            // rebuilds every time.
            let d = if seed % 5 == 0 { 12.0 } else { 3.0 };
            put_vec(
                c,
                list + 0x110,
                [(f32::from_bits(p[0]) + d).to_bits(), p[1], p[2], r.next_u32()],
            );
            c.bus.write32(list + 0x11c, if seed % 11 == 0 { 0 } else { 0x3f80_0000 });
            let count: u32 = match seed % 9 {
                0 => 0,
                1 => 0xffff_fffe,
                _ => 1 + seed % 5,
            };
            c.bus.write32(list, count);
            for i in 0..5 {
                let src = SOURCES + 0x80 * i;
                let entry = if r.next_u32() % 4 == 0 { 0 } else { src };
                c.bus.write32(list + 4 + 4 * i, entry);
                // Source: object at +0xc, data block at +0x10 (no records).
                let obj = src + 0x20;
                let block = src + 0x60;
                c.bus.write32(src + 0xc, obj);
                c.bus.write32(src + 0x10, block);
                for k in 0..3 {
                    c.bus.write32(obj + 0x40 + 4 * k, r.f32(10.0));
                }
                c.bus.write32(block, r.next_u32() % 3);
                c.bus.write32(block + 4, if r.next_u32() % 2 == 0 { 0 } else { 0xffff_ffff });
            }
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(LISTENER);
            c.g[A2][0] = idx as u64;
        });
    }

    /// ModifierBlock_SetTransformBounds: no node, or a node whose vtable
    /// slot 13 is a fake method (random this-delta) that may unwind.
    #[test]
    fn modifier_block_set_transform_bounds_matches() {
        covered(
            ee::s11::f_00352bc0__generated,
            modifier_block_set_transform_bounds,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35_2bc0, 1);
                c.pc = 0x0035_2bc0;
                let block = DATA + 0x2_0000;
                let node = if seed % 5 == 0 {
                    0
                } else {
                    let vt = crate::difftest::fakes::vtable(c, &mut r, &[0x68]);
                    let n = DATA + 0x3_0000 + (r.next_u32() & 0xff0);
                    c.bus.write32(n, vt);
                    n
                };
                c.bus.write32(block, node);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = sx(DATA + 0x4_0000);
            },
        );
    }

    /// SSXAudioSystem_UpdateUberSpeech: the real IsReplayPlaying (no game
    /// module, no replay, replay state 0 / playing), the real FindRider over
    /// up to three race-manager riders, the tracked id matching or not, the
    /// rider state 10 or not, the last state 10 or not, game mode 4 or not,
    /// and the real ArcadeSay2133 on its short paths (arcade speech gain 0,
    /// or the rider's air value not negative).
    #[test]
    fn ssx_audio_update_uber_speech_matches() {
        const SYS: u32 = DATA;
        const GAME: u32 = DATA + 0x1_0000;
        const MODULE: u32 = DATA + 0x1_0100;
        const RACE_MAN: u32 = DATA + 0x1_0400;
        const REPLAY: u32 = DATA + 0x1_0800;
        const RIDERS: u32 = DATA + 0x2_0000;
        check(ee::s08::f_0029b738__generated, ssx_audio_update_uber_speech, 300, &|c, seed| {
            let mut r = Rng(0x29_b738 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            c.g[GP][0] = sx(GAME_GP);
            c.pc = 0x0029_b738;
            c.bus.write32(GAME_GP.wrapping_sub(0x848), GAME);
            c.bus.write32(GAME + 0x84, if seed % 17 == 0 { 0 } else { MODULE });
            c.bus.write32(MODULE + 0xc, RACE_MAN);
            c.bus.write32(MODULE + 0x28, if seed % 4 == 0 { REPLAY } else { 0 });
            c.bus.write32(REPLAY, [0, 0, 3, 12][(r.next_u32() % 4) as usize]);
            c.bus.write32(REPLAY + 0x18, RIDERS);
            c.bus.write32(RACE_MAN + 0x40, REPLAY);
            c.bus.write32(MODULE + UBER_MODULE_MODE, [4, 4, 0, 2][(r.next_u32() % 4) as usize]);
            c.bus.write32(RACE_MAN + 0x7c, r.next_u32() % 4);
            c.bus.write32(RACE_MAN + 0x78, r.next_u32() % 4);
            let id = r.next_u32() % 2;
            for i in 0..3 {
                let rd = RIDERS + 0x1000 * i;
                c.bus.write32(RACE_MAN + 0x28 + 4 * i, rd);
                c.bus.write32(rd + 0x874, r.next_u32() % 2);
                c.bus.write32(rd + 0x87c, r.next_u32() % 2);
                c.bus.write32(rd + 0xec, (r.next_u32() % 8).wrapping_sub(1));
                c.bus.write32(rd + UBER_RIDER_PLAYER_ID, if r.next_u32() % 4 == 0 { 1 - id } else { id });
                c.bus.write32(rd + UBER_RIDER_STATE, if r.next_u32() % 3 == 0 { 3 } else { 10 });
                // Air value: not negative, so ArcadeSay2133 stops early.
                c.bus.write32(rd + 0x470, r.f32(4.0) & 0x7fff_ffff);
            }
            c.bus.write32(SYS + UBER_RIDER_ID, id);
            c.bus.write32(SYS + UBER_LAST_STATE, if r.next_u32() % 3 == 0 { 10 } else { 0 });
            // Arcade speech gain (channel 10): off or on.
            c.bus.write32(SYS + 0x635c, if r.next_u32() % 2 == 0 { 0 } else { 0x3f80_0000 });
            c.g[A0][0] = sx(SYS);
        });
    }

    /// ScriptEngine_UpdateProcesses: the real InSeq_UpdateAll with no
    /// running sequences; sixteen slots that are free, running (state 2+),
    /// or pending with types 0..7 (the jump table copied from the ELF); the
    /// real LaunchPendingProcess for type 1 on its short path (no script
    /// registered at the slot's index).
    #[test]
    fn script_engine_update_processes_matches() {
        const ENGINE: u32 = DATA;
        const SEQ: u32 = DATA + 0x1000;
        const SCRIPTS: u32 = DATA + 0x2000;
        check(ee::s07::f_002820b0__generated, script_engine_update_processes, 300, &|c, seed| {
            let mut r = Rng(0x28_20b0 ^ (seed as u64) << 24);
            scramble(c, &mut r);
            c.pc = 0x0028_20b0;
            put_words(
                c,
                SE_TYPE_TABLE,
                &[0x28_2128, 0x28_2120, 0x28_2128, 0x28_2128, 0x28_2128, 0x28_2128],
            );
            c.bus.write32(ENGINE + SE_SEQ_PLAYER, SEQ);
            c.bus.write32(SEQ + 0x1c, 0);
            c.bus.write32(ENGINE + 0x2b0, SCRIPTS);
            for i in 0..16 {
                let slot = ENGINE + SE_SLOTS + 0x24 * i;
                fill(c, &mut r, slot, 9);
                let state = match r.next_u32() % 4 {
                    0 => 0,
                    1 => 2 + r.next_u32() % 3,
                    _ => 1,
                };
                c.bus.write32(slot + SE_SLOT_STATE, state);
                let ty = if r.next_u32() % 2 == 0 { 1 } else { r.next_u32() % 8 };
                c.bus.write32(slot + SE_SLOT_TYPE, ty);
                c.bus.write32(slot + 0x14, r.next_u32() % 32);
            }
            for i in 0..32 {
                c.bus.write32(SCRIPTS + 8 * i, 0);
            }
            c.g[A0][0] = sx(ENGINE);
        });
    }

    /// SNDSTRM_ParseData: little- and big-endian streams (byte-swapped sample
    /// counts and offsets), 0..4 channels, empty chunks (count 0, or only
    /// the top bit set) released through the real StreamFile_ReleaseChunk
    /// (null file handle: the validation fails at once), non-empty ones
    /// pushed through the real SNDSTRM_PushBlock (no queued blocks), with
    /// the "pushed" flag set or clear.
    #[test]
    fn sndstrm_parse_data_matches() {
        const STREAM: u32 = DATA;
        const STATE: u32 = DATA + 0x200;
        const CHUNK: u32 = DATA + 0x400;
        const BLOCKS: u32 = DATA + 0x800;
        check(ee::s13::f_003b6ea8__generated, sndstrm_parse_data, 300, &|c, seed| {
            // Scrambled registers, game gp, random bytes around the frame.
            let mut r = machine(c, seed, 0x3b_6ea8, 0);
            c.pc = 0x003b_6ea8;
            let swap = seed % 2 == 1;
            let bytes = |v: u32| if swap { v.swap_bytes() } else { v };
            let count = match seed % 7 {
                0 => 0,
                1 => 0x8000_0000,
                _ => 1 + r.next_u32() % 0x4000 | (r.next_u32() & 0x8000_0000),
            };
            let channels = if count & 0x7fff_ffff == 0 && seed % 3 == 0 {
                0
            } else {
                1 + r.next_u32() % 4
            };
            let idx = r.next_u32() % 4;
            fill(c, &mut r, STREAM, 0x10);
            c.bus.write32(STREAM, 0);
            c.bus.write32(STREAM + 0x8, idx);
            c.bus.write8(STREAM + SS_CHANNELS, channels as u8);
            c.bus.write32(STREAM + SS_STATE, STATE);
            fill(c, &mut r, STATE, 0x10);
            c.bus.write8(STATE + SS_SWAP, swap as u8);
            c.bus.write8(STATE + SS_PUSHED, (r.next_u32() % 2) as u8);
            fill(c, &mut r, CHUNK, 0x10);
            c.bus.write32(CHUNK + SS_CHUNK_SAMPLES, bytes(count));
            for i in 0..channels {
                let off = 4 + 0x40 * i + (r.next_u32() % 0x10) * 4;
                c.bus.write32(CHUNK + SS_CHUNK_OFFSETS + 4 * i, bytes(off));
            }
            // SNDSTRM_PushBlock's stream table and stream (no queue).
            for i in 0..4 {
                let b = BLOCKS + 0x100 * i;
                c.bus.write32(0x0051_57b8 + 0x304 + 4 * i, b);
                fill(c, &mut r, b, 0x40);
                c.bus.write8(b + 0x62, 0);
                c.bus.write16(b + 0x46, (r.next_u32() % 4) as u16);
                c.bus.write16(b + 0x40, 1 + (r.next_u32() % 4) as u16);
            }
            c.g[A0][0] = sx(STREAM);
            c.g[A1][0] = sx(CHUNK);
        });
    }

    /// Rider_GetTrickSkillFactor: the game image (attribute tables, the
    /// `.lit4` scale), a BE singleton, a library whose getInterface slot is
    /// a fake (may unwind), and the real attribute-2 factor on random
    /// profile slots, character records and stats indices (zero, small,
    /// random: the fixed 0.5, the table value or the given value).
    #[test]
    fn rider_get_trick_skill_factor_matches() {
        const RIDER: u32 = DATA + 0x7_0000;
        covered(ee::s00::f_00120038__generated, rider_get_trick_skill_factor, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x12_0038, 1);
            crate::difftest::elf_image(c);
            c.pc = 0x0012_0038;
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0x6_0000);
            c.bus.write32(GAME_GP - 0x1efc, 0);
            // The lazily built BE interface singletons exist (new race,
            // new player, char, aggression).
            for g in [0x1f3c, 0x1f30, 0x1f2c, 0x1ea4] {
                c.bus.write32(GAME_GP - g, DATA + 0xe_0000 + g * 0x10);
            }
            let lib = DATA + 0x5_0000;
            c.bus.write32(0x0053_08a8, lib);
            let vt = crate::difftest::fakes::vtable(c, &mut r, &[8]);
            c.bus.write32(lib + 4, vt);
            for p in 0..8 {
                c.bus.write32(0x0053_05b0 + 4 * p, r.next_u32() % 8);
            }
            for s in 0..8 {
                let rec = 0x0053_5b20 + s * 0x1c;
                if r.next_u32() % 3 == 0 {
                    c.bus.write32(rec + 0xc, u32::MAX);
                }
                c.bus.write8(rec + 0x11, (r.next_u32() % 10) as u8);
            }
            c.bus.write32(RIDER + RIDER_AI_INDEX, r.next_u32() % 8);
            let stats = match r.next_u32() % 3 {
                0 => 0,
                1 => r.next_u32() % 40,
                _ => r.next_u32(),
            };
            c.bus.write32(RIDER + RIDER_STATS_INDEX, stats);
            c.g[A0][0] = sx(RIDER);
        });
    }

    /// GameModule_RenderFogParticles: both, one or neither fog list
    /// non-empty; the manager's renderFog slot is a fake (random this-delta,
    /// may unwind); the real frame finisher on its short path (no particles:
    /// first word 0).
    #[test]
    fn game_module_render_fog_particles_matches() {
        const MODULE: u32 = DATA + 0x1_0000;
        const WORLD: u32 = DATA + 0x2_0000;
        const MAN: u32 = DATA + 0x3_0000;
        covered(
            ee::s05::f_0022c708__generated,
            game_module_render_fog_particles,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22_c708, 2);
                c.pc = 0x0022_c708;
                let vt = crate::difftest::fakes::vtable(c, &mut r, &[FOG_RENDER_SLOT]);
                c.bus.write32(MAN, 0);
                c.bus.write32(MAN + FOG_VTABLE, vt);
                c.bus.write32(MODULE + GM_FOG_MAN, MAN);
                for (k, &(count_off, _, _, _)) in WORLD_FOG_LISTS.iter().enumerate() {
                    let n = if (seed >> k) % 2 == 0 { 0 } else { 1 + r.next_u32() % 20 };
                    c.bus.write32(WORLD + count_off, n);
                }
                c.g[A0][0] = sx(MODULE);
                c.g[A1][0] = sx(WORLD);
            },
        );
    }

    /// RiderSphereTree_CollideBox: boxes around the origin, sphere centres
    /// mostly inside (hits) and sometimes outside (misses), radii 0..5; no
    /// sub-sphere mask (first hit), or masks with counts -1..6 (the best of
    /// the repeated sweeps, or none); the real Collide_SphereVsBoxEdges with
    /// its axis vectors (.bss) and 1e30 limits (.lit4).
    #[test]
    fn rider_sphere_tree_collide_box_matches() {
        const BOX: u32 = DATA + 0x2000;
        const SPHERE: u32 = DATA + 0x3000;
        const OUT_POS: u32 = DATA + 0x4000;
        const OUT_NORMAL: u32 = DATA + 0x4100;
        check(ee::s10::f_0032b2b8__generated, rider_sphere_tree_collide_box, 300, &|c, seed| {
            let mut r = Rng(0x32_b2b8 ^ (seed as u64) << 24);
            scramble(c, &mut r);
            c.g[GP][0] = sx(GAME_GP);
            c.pc = 0x0032_b2b8;
            put_words(c, GAME_GP - 0x2df0, &[0x7149_f2ca, 0x7149_f2ca]);
            let one = 1.0f32.to_bits();
            put_vec(c, 0x004f_f140, [one, 0, 0, 0]);
            put_vec(c, 0x004f_f150, [0, one, 0, 0]);
            put_vec(c, 0x004f_f160, [0, 0, one, 0]);
            let coord = |r: &mut Rng, lo: f32, hi: f32| {
                (lo + (r.next_u32() % 1000) as f32 * (hi - lo) / 1000.0).to_bits()
            };
            let outside = seed % 7 == 0;
            for i in 0..3 {
                c.bus.write32(BOX + 4 * i, coord(&mut r, -10.0, 0.0));
                c.bus.write32(BOX + 0x10 + 4 * i, coord(&mut r, 0.0, 10.0));
                let centre = if outside {
                    coord(&mut r, 20.0, 30.0)
                } else {
                    coord(&mut r, -9.0, 9.0)
                };
                c.bus.write32(SPHERE + RST_CENTRE + 4 * i, centre);
            }
            c.bus.write32(SPHERE + RST_CENTRE + 0xc, one);
            c.bus.write32(SPHERE + RST_RADIUS, coord(&mut r, 0.0, 5.0));
            let (mask, count) = if seed % 3 == 0 {
                (0, r.next_u32())
            } else {
                let count = (r.next_u32() % 8).wrapping_sub(1);
                (r.next_u32() & 0x7f, count)
            };
            c.bus.write32(SPHERE + RST_MASK, mask);
            c.bus.write32(SPHERE + RST_COUNT, count);
            fill(c, &mut r, OUT_POS, 4);
            fill(c, &mut r, OUT_NORMAL, 4);
            c.g[A0][0] = sx(SPHERE);
            c.g[A1][0] = sx(BOX);
            c.g[A2][0] = sx(BOX + 0x10);
            c.g[A3][0] = sx(OUT_POS);
            c.g[T0][0] = sx(OUT_NORMAL);
        });
    }

    /// SND_RunServiceCallbacks: disabled, enabled with a count of 0, a
    /// negative count, or 1..5 fake callbacks (each may unwind).
    #[test]
    fn snd_run_service_callbacks_matches() {
        covered(ee::s13::f_003c60e0__generated, snd_run_service_callbacks, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3c_60e0, 5);
            c.pc = 0x003c_60e0;
            let on = if seed % 5 == 0 { 0 } else { 1 + r.next_u32() % 0x7f };
            c.bus.write8(SND_GLOBALS + SND_SERVICE_ON, on as u8);
            let count: i8 = match seed % 7 {
                0 => 0,
                1 => -3,
                _ => (1 + r.next_u32() % 5) as i8,
            };
            c.bus.write8(SND_GLOBALS + SND_SERVICE_COUNT, count as u8);
            for i in 0..6 {
                c.bus.write32(SND_GLOBALS + SND_SERVICE_FNS + 4 * i, FAKE + 0x10 * i);
            }
        });
    }
}
