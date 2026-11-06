//! Staging wave 210 (oc282): hand-written replacements for the hottest
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

/// `void cWorldCache::ReleaseResource(int cell, int kind, sHandle h)`
/// (0x3a8e20): drops one reference of a cached world resource. The handle's
/// low byte picks the table (0xff: the shared table at +4, else the slot's
/// table from the array at +8), `kind` the table entry, and `h >> 8` the
/// reference word in the entry's array (low byte: count, upper 24 bits: the
/// resource address / 4). When the count reaches 0 the address bits are
/// cleared and the owner object (+0x3e8) is told through its vtable slot
/// +0x20 (`release(kind, resource or 0 for the sentinel at gp+0x16e0, h)`);
/// a shared (0xff) resource it hands back is freed with `EA_MemFree`.
pub fn cworld_cache_release_resource(c: &mut Cpu) {
    const SHARED_TABLE: u32 = 0x4;
    const SLOT_TABLES: u32 = 0x8;
    const OWNER: u32 = 0x3e8;
    const SHARED: u64 = 0xff;
    guest(c, |c| {
        c.cycles += 10;
        let handle = c.g[A3][0];
        c.g[T1][0] = handle;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let cache = c.g[A0][0] as u32;
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let slot = handle & 0xff;
        c.g[S0][0] = slot;
        let index = sx((handle as u32) >> 8);
        c.g[V0][0] = 0xff;
        let kind = c.g[A2][0];
        c.g[A1][0] = kind;
        let table = if slot == SHARED {
            c.cycles += 2;
            c.lw(cache.wrapping_add(SHARED_TABLE))
        } else {
            c.cycles += 4;
            let tables = c.lw(cache.wrapping_add(SLOT_TABLES));
            let p = sx(((slot as u32) << 2).wrapping_add(tables as u32));
            c.g[V0][0] = p;
            c.lw(p as u32)
        };
        // The entry's reference array and the handle's reference word.
        c.cycles += 10;
        let entry = sx((table as u32).wrapping_add((kind as u32) << 3));
        let refs = c.lw((entry as u32).wrapping_add(4));
        let word_at = sx(((index as u32) << 2).wrapping_add(refs as u32));
        let count = sx((c.lbu(word_at as u32) as u32).wrapping_sub(1));
        c.sb(word_at as u32, count);
        let resource = if count & 0xff != 0 {
            c.cycles += 1;
            c.g[V0][0] = count;
            c.g[V1][0] = count & 0xff;
            0
        } else {
            c.cycles += 6;
            let word = c.lw(word_at as u32);
            c.g[V1][0] = word & 0xff;
            let addr_bits = sx((word as u32) >> 8);
            c.g[V0][0] = addr_bits;
            c.sw(word_at as u32, word & 0xff);
            sx((addr_bits as u32) << 2)
        };
        c.g[A0][0] = resource;
        c.cycles += 2;
        if resource == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            let sentinel = sx((c.g[GP][0] as u32).wrapping_add(0x16e0));
            c.g[V0][0] = sentinel;
            let owner = c.lw(cache.wrapping_add(OWNER));
            c.g[T0][0] = owner;
            c.g[A2][0] = c.g[T1][0];
            if resource != sentinel {
                c.cycles += 2;
                c.g[A3][0] = resource;
            } else {
                c.cycles += 3;
                c.g[A3][0] = 0;
            }
            // owner->release(kind, resource, handle) through vtable +0x20.
            c.cycles += 8;
            let vt = c.lw((owner as u32).wrapping_add(8));
            c.g[V1][0] = vt;
            let delta = c.lh((vt as u32).wrapping_add(0x20));
            let method = c.lw((vt as u32).wrapping_add(0x24));
            c.g[V0][0] = method;
            c.g[A0][0] = sx((owner as u32).wrapping_add(delta as u32));
            call_ptr(c, method as u32, 0x003a_8ee0)?;
            let freed = c.g[V0][0];
            c.g[A0][0] = freed;
            c.g[V0][0] = 0xff;
            if freed == 0 {
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 2;
                let shared = c.g[S0][0] == SHARED;
                frame.restore(c, S0, 0x10);
                if shared {
                    c.cycles += 2;
                    call_ptr(c, 0x0031_7e98, 0x003a_8efc)?; // EA_MemFree
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MultiSplineModifier::Save(cStream* s)` (0x35b5a8): writes the
/// modifier's 0x34-byte header (+4: count first) with `s->Write` (vtable
/// +8), its path (+0x48, `SplineModifier_SavePath`), the pointer at +0x40
/// with `s->WritePtr` (vtable +0x28), then for each of the `count` splines
/// (array at +0x44) the word at +0x78.
pub fn multi_spline_modifier_save(c: &mut Cpu) {
    const COUNT: u32 = 0x4;
    const OWNER: u32 = 0x40;
    const SPLINES: u32 = 0x44;
    const PATH: u32 = 0x48;
    const SPLINE_WORD: u32 = 0x78;
    const SAVE_PATH: Callee = Callee { addr: 0x0034_5538, f: ee::s11::SplineModifier_SavePath };
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0x34;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        // s->Write(&self->count, 0x34)
        let me = c.g[S2][0] as u32;
        let stream = c.g[S1][0] as u32;
        c.g[A1][0] = sx(me.wrapping_add(COUNT));
        let vt = c.lw(stream);
        c.g[V1][0] = vt;
        let delta = c.lh((vt as u32).wrapping_add(0x8));
        let write = c.lw((vt as u32).wrapping_add(0xc));
        c.g[V0][0] = write;
        c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
        call_ptr(c, write as u32, 0x0035_b5e4)?;
        // SplineModifier_SavePath(&self->path, s)
        let me = c.g[S2][0] as u32;
        let stream = c.g[S1][0];
        c.g[A0][0] = sx(me.wrapping_add(PATH));
        c.g[A1][0] = stream;
        SAVE_PATH.call(c, 0x0035_b5f0)?;
        // s->WritePtr(self->owner)
        let me = c.g[S2][0] as u32;
        let stream = c.g[S1][0] as u32;
        let vt = c.lw(stream);
        c.g[V0][0] = vt;
        c.g[A1][0] = c.lw(me.wrapping_add(OWNER));
        let delta = c.lh((vt as u32).wrapping_add(0x28));
        let write_ptr = c.lw((vt as u32).wrapping_add(0x2c));
        c.g[V1][0] = write_ptr;
        c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
        call_ptr(c, write_ptr as u32, 0x0035_b608)?;
        let count = c.lw((c.g[S2][0] as u32).wrapping_add(COUNT));
        c.g[V0][0] = count;
        if count as i64 > 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(SPLINES));
            loop {
                // s->Write(&splines[i]->word78, 4)
                c.cycles += 15;
                let i = c.g[S0][0] as u32;
                let stream = c.g[S1][0] as u32;
                let at = sx((i << 2).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = at;
                let vt = c.lw(stream);
                c.g[A3][0] = vt;
                c.g[A2][0] = 0x4;
                c.g[S0][0] = sx(i.wrapping_add(1));
                let spline = c.lw(at as u32);
                let delta = c.lh((vt as u32).wrapping_add(0x8));
                let write = c.lw((vt as u32).wrapping_add(0xc));
                c.g[V1][0] = write;
                c.g[A1][0] = sx((spline as u32).wrapping_add(SPLINE_WORD));
                c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
                call_ptr(c, write as u32, 0x0035_b644)?;
                let count = c.lw((c.g[S2][0] as u32).wrapping_add(COUNT));
                let more = ((c.g[S0][0] as i64) < (count as i64)) as u64;
                c.g[V0][0] = more;
                if more == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(SPLINES));
                if event_due(c) {
                    deliver_event_here(c, 0x0035_b618);
                }
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldObj::Kill()` (0x34fd90): in the instance node (+0x18) flags
/// word (+8) the low half is replaced by the high half and bit 2 (dead) is
/// set; then `this->SetState(3)` through vtable (+0xc) slot +8.
pub fn world_obj_kill(c: &mut Cpu) {
    const NODE: u32 = 0x18;
    const FLAGS: u32 = 0x8;
    const VTABLE: u32 = 0xc;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        let me = c.g[A0][0];
        c.g[A2][0] = me;
        frame.save_ra(c, 0);
        let mask = 0xffff_ffff_ffff_0000u64;
        let node = c.lw((me as u32).wrapping_add(NODE));
        let flags = c.lw((node as u32).wrapping_add(FLAGS)) & mask;
        c.sw((node as u32).wrapping_add(FLAGS), flags);
        let node = c.lw((me as u32).wrapping_add(NODE));
        c.g[A0][0] = node;
        let flags = c.lw((node as u32).wrapping_add(FLAGS));
        let high = sx(((flags as u32) as i32 >> 16) as u32);
        c.sw((node as u32).wrapping_add(FLAGS), flags | high);
        let node = c.lw((me as u32).wrapping_add(NODE));
        c.g[V1][0] = node;
        let flags = c.lw((node as u32).wrapping_add(FLAGS)) | 0x2;
        c.g[V0][0] = flags;
        c.sw((node as u32).wrapping_add(FLAGS), flags);
        if me != 0 {
            c.cycles += 6;
            let vt = c.lw((me as u32).wrapping_add(VTABLE));
            c.g[V0][0] = vt;
            c.g[A1][0] = 0x3;
            let delta = c.lh((vt as u32).wrapping_add(0x8));
            let method = c.lw((vt as u32).wrapping_add(0xc));
            c.g[V1][0] = method;
            c.g[A0][0] = sx((me as u32).wrapping_add(delta as u32));
            call_ptr(c, method as u32, 0x0034_fdf0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One field of the voice record a play request pushes: reads the write
/// cursor (`bank + 0x1f4`, s0) into `idx_reg` and the array pointer at
/// `params + table` (s1) into `base_reg`; returns (and leaves in `idx_reg`)
/// `array + (cursor << shift)`.
fn grab_trick_voice_slot(c: &mut Cpu, idx_reg: usize, base_reg: usize, table: u32, shift: u32) -> u32 {
    let bank = c.g[S0][0] as u32;
    let params = c.g[S1][0] as u32;
    c.g[idx_reg][0] = c.lw(bank.wrapping_add(0x1f4));
    c.g[base_reg][0] = c.lw(params.wrapping_add(table));
    let at = ((c.g[idx_reg][0] as u32) << shift).wrapping_add(c.g[base_reg][0] as u32);
    c.g[idx_reg][0] = sx(at);
    at
}

/// `void SSXAudioSystem::PlayGrabTrickSound(Rider* rider)` (0x29a530): when
/// the rider has a grab sound pending (+0x874 and +0x87c > 0), pushes a play
/// record for sound 0x77 on channel 9 into the MIDI bank (`*this`): bumps
/// the cursor (+0x1f4), copies the 24-byte template (+0x1dc) into the
/// record array, clears/sets the record's fields (volume 0x64, pan 0x5a,
/// 0x32, 0x7f, 1), stores the channel gain pointer
/// (`GetChannelGainPtr(this, 9, 0)`) and plays it (`Audio_PlayPushedSound`).
pub fn ssx_audio_system_play_grab_trick_sound(c: &mut Cpu) {
    const PENDING: u32 = 0x874;
    const PENDING_COUNT: u32 = 0x87c;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0;
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        let rider = c.g[A1][0] as u32;
        let pending = c.lw(rider.wrapping_add(PENDING));
        c.g[V0][0] = pending;
        c.g[S3][0] = c.g[A0][0];
        if pending != 0 {
            c.cycles += 2;
            let count = c.lw(rider.wrapping_add(PENDING_COUNT));
            c.g[V0][0] = count;
            c.g[V1][0] = (0 < count) as u64;
        }
        c.cycles += 2;
        c.g[T1][0] = 0x18;
        if c.g[V1][0] != 0 {
            c.cycles += 148;
            let me = c.g[S3][0] as u32;
            let bank = c.lw(me) as u32;
            c.g[S0][0] = sx(bank);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x8;
            let cursor = c.lw(bank.wrapping_add(0x1f4));
            c.g[S1][0] = sx(bank.wrapping_add(0x1d8));
            c.g[S2][0] = 0x77;
            let cursor = sx((cursor as u32).wrapping_add(1));
            c.g[V0][0] = cursor;
            c.sw(bank.wrapping_add(0x1f4), cursor);
            // The record (cursor * 0x18 into params+0x20) gets the template.
            let params = c.g[S1][0] as u32;
            c.g[V1][0] = c.lw(bank.wrapping_add(0x1f4));
            c.g[V0][0] = c.lw(params.wrapping_add(0x20));
            c.mult(0, V1, V1, T1);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.ldl(V0, bank.wrapping_add(0x1e3));
            c.ldr(V0, bank.wrapping_add(0x1dc));
            c.ldl(A0, bank.wrapping_add(0x1eb));
            c.ldr(A0, bank.wrapping_add(0x1e4));
            c.ldl(A3, bank.wrapping_add(0x1f3));
            c.ldr(A3, bank.wrapping_add(0x1ec));
            let record = c.g[V1][0] as u32;
            c.sdl(V0, record.wrapping_add(0x7));
            c.sdr(V0, record);
            c.sdl(A0, record.wrapping_add(0xf));
            c.sdr(A0, record.wrapping_add(0x8));
            c.sdl(A3, record.wrapping_add(0x17));
            c.sdr(A3, record.wrapping_add(0x10));
            let cur = c.lw(bank.wrapping_add(0x1f4));
            let at = sx(((cur as u32) << 2).wrapping_add(bank));
            c.g[V0][0] = at;
            c.sw((at as u32).wrapping_add(0x1fc), 0);
            let at = grab_trick_voice_slot(c, V1, V0, 0x34, 2);
            c.sw(at, 0);
            // field 0x38[cursor] = (s8) record_bytes[cursor * 0x18]
            c.g[A3][0] = c.lw(bank.wrapping_add(0x1f4));
            c.g[A0][0] = c.lw(bank.wrapping_add(0x1f4));
            c.g[T0][0] = c.lw(params.wrapping_add(0x20));
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
            c.mult(0, A0, A0, T1);
            c.g[V0][0] = c.lw(params.wrapping_add(0x38));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            let byte = c.lb(c.g[A0][0] as u32);
            c.g[V1][0] = byte;
            c.sw(c.g[A3][0] as u32, byte);
            let at = grab_trick_voice_slot(c, V0, V1, 0x3c, 2);
            c.sw(at, 0);
            // memset(&params->names[cursor] (+0x40, 8 bytes each), 0, 8)
            let cur = c.lw(bank.wrapping_add(0x1f4));
            c.g[A0][0] = sx(params.wrapping_add((cur as u32) << 3).wrapping_add(0x40));
            call_ptr(c, 0x0041_6210, 0x0029_a63c)?;
            let at = grab_trick_voice_slot(c, V1, V0, 0x60, 2);
            c.g[A3][0] = 0x64;
            c.g[T0][0] = 0x5a;
            c.g[T1][0] = 0x32;
            c.g[T2][0] = 0x7f;
            c.sw(at, 0);
            c.g[T3][0] = 0x1;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0x9;
            c.g[A2][0] = 0;
            let at = grab_trick_voice_slot(c, V1, V0, 0x64, 2);
            c.sw(at, 0);
            let at = grab_trick_voice_slot(c, V0, V1, 0x68, 2);
            c.sw(at, 0);
            let at = grab_trick_voice_slot(c, V1, V0, 0x6c, 2);
            c.sw(at, 0);
            // The 16-byte envelope at +0x70: 0, 0x64, 0x5a, 0x32.
            let at = grab_trick_voice_slot(c, V0, V1, 0x70, 4);
            c.sw(at.wrapping_add(0x4), 0);
            let at = grab_trick_voice_slot(c, V1, V0, 0x70, 4);
            c.sw(at, c.g[A3][0]);
            let at = grab_trick_voice_slot(c, V0, V1, 0x70, 4);
            c.sw(at.wrapping_add(0x8), c.g[T0][0]);
            let at = grab_trick_voice_slot(c, V1, V0, 0x70, 4);
            c.sw(at.wrapping_add(0xc), c.g[T1][0]);
            let at = grab_trick_voice_slot(c, V0, V1, 0x74, 2);
            c.sw(at, 0);
            let at = grab_trick_voice_slot(c, V1, V0, 0x80, 2);
            c.sw(at, 0);
            let at = grab_trick_voice_slot(c, V0, V1, 0x84, 2);
            c.sw(at, 0);
            let at = grab_trick_voice_slot(c, V1, V0, 0x78, 2);
            c.sw(at, c.g[T2][0]);
            let at = grab_trick_voice_slot(c, V0, V1, 0x7c, 2);
            c.sw(at, c.g[T3][0]);
            // this->+4[cursor] = 0, this->+8[cursor] = 0x77
            for (table, value) in [(0x4u32, 0u64), (0x8, 0x77)] {
                let me = c.g[S3][0] as u32;
                c.g[V1][0] = c.lw(me);
                c.g[A3][0] = c.lw(me.wrapping_add(table));
                let cur = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                let at = sx(((cur as u32) << 2).wrapping_add(c.g[A3][0] as u32));
                c.g[V0][0] = at;
                let value = if table == 0x8 { c.g[S2][0] } else { value };
                c.sw(at as u32, value);
            }
            // gain = GetChannelGainPtr(this, 9, 0); channels->gain[cursor] = gain
            let me = c.g[S3][0] as u32;
            c.g[S0][0] = c.lw(me.wrapping_add(0x118));
            call_ptr(c, 0x0028_7968, 0x0029_a798)?;
            let channels = c.lw(c.g[S0][0] as u32);
            c.g[A1][0] = channels;
            c.g[A0][0] = c.g[S3][0];
            let cur = c.lw((channels as u32).wrapping_add(0x1f4));
            c.g[A2][0] = c.lw((channels as u32).wrapping_add(0x214));
            let at = sx(((cur as u32) << 2).wrapping_add(c.g[A2][0] as u32));
            c.g[V1][0] = at;
            c.sw(at as u32, c.g[V0][0]);
            call_ptr(c, 0x0029_06b8, 0x0029_a7b8)?; // Audio_PlayPushedSound
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

/// The shape of the small `SaveState` methods: `s->Write(this + off, size)`
/// through the stream's vtable (+8), from a 0x10 frame returning to `ret`.
/// 13 cycles.
fn stream_write_member(c: &mut Cpu, off: u32, size: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        let stream = c.g[A1][0];
        c.g[A3][0] = stream;
        frame.save_ra(c, 0);
        c.g[A1][0] = if off == 0 {
            c.g[A0][0]
        } else {
            sx((c.g[A0][0] as u32).wrapping_add(off))
        };
        c.g[A2][0] = size;
        let vt = c.lw(stream as u32);
        c.g[V1][0] = vt;
        let delta = c.lh((vt as u32).wrapping_add(0x8));
        let write = c.lw((vt as u32).wrapping_add(0xc));
        c.g[V0][0] = write;
        c.g[A0][0] = sx((stream as u32).wrapping_add(delta as u32));
        call_ptr(c, write as u32, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderBase::SaveCheatCode(cStream* s)` (0x2f6a58): writes the cheat
/// code word (+4).
pub fn rider_base_save_cheat_code(c: &mut Cpu) {
    stream_write_member(c, 0x4, 0x4, 0x002f_6a80);
}

/// `void RiderHandplantSub::SaveState(cStream* s)` (0x1330b8): writes the
/// 0x14-byte state.
pub fn rider_handplant_sub_save_state(c: &mut Cpu) {
    stream_write_member(c, 0, 0x14, 0x0013_30e0);
}

/// `void RiderMetrix::SaveState(cStream* s)` (0x1174b0): writes the
/// 0x1ac-byte metrics block.
pub fn rider_metrix_save_state(c: &mut Cpu) {
    stream_write_member(c, 0, 0x1ac, 0x0011_74d8);
}

/// `ModObj* ModObj::ModObj(list, type, node)` (0x355280): the `WorldObj`
/// constructor, then the ModObj vtable (0x491028), scale 1.0 (+0x24), flag
/// bit 0 of the half-word at +0x12, and the fields +0x1c, +0x20, +0x28
/// cleared. Returns `this`.
pub fn mod_obj_ctor(c: &mut Cpu) {
    const WORLD_OBJ_CTOR: Callee = Callee { addr: 0x0034_fb00, f: ee::s11::WorldObj_ctor };
    const VTABLE: u32 = 0x0049_1028;
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        WORLD_OBJ_CTOR.call(c, 0x0035_5294)?;
        let me = c.g[S0][0] as u32;
        let flags = c.lhu(me.wrapping_add(0x12)) | 0x1;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[V0][0] = sx(VTABLE);
        c.sw(me.wrapping_add(0xc), sx(VTABLE));
        c.g[V1][0] = flags;
        c.swc1(0, me.wrapping_add(0x24));
        c.g[V0][0] = c.g[S0][0];
        c.sh(me.wrapping_add(0x12), flags);
        c.sw(me.wrapping_add(0x1c), 0);
        c.sw(me.wrapping_add(0x20), 0);
        c.sw(me.wrapping_add(0x28), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BEAggressionInt::SaveState(cStream* s)` (0x1560a0): writes the
/// rider count (0x5305e0) through a stack copy, then for each rider the ten
/// 3-byte aggression entries of its profile and character in the table at
/// 0x4a6ca8 (`[profile * 0x9b50 + character * 0xf88 + 0xbc1 + 3 * j]`), all
/// with `s->Write` (vtable +8).
pub fn be_aggression_int_save_state(c: &mut Cpu) {
    const GET_PROFILE: Callee = Callee { addr: 0x0014_a0e0, f: ee::s01::BE_GetRiderProfile };
    const GET_CHARACTER: Callee = Callee { addr: 0x0014_a080, f: ee::s01::BE_GetRiderCharacter };
    const RIDER_COUNT: u32 = 0x0053_05e0;
    const TABLE: u32 = 0x004a_6ca8;
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = 0x0053_0000;
        frame.save(c, S1, 0x90);
        c.g[A2][0] = 0x4;
        frame.save(c, S3, 0x70);
        c.g[S1][0] = 0;
        frame.save(c, S0, 0xa0);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x80);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        // n = rider count; s->Write(&n, 4)
        let n = c.lw(RIDER_COUNT);
        c.g[T0][0] = n;
        let stream = c.g[S3][0] as u32;
        let vt = c.lw(stream);
        c.g[A3][0] = vt;
        let delta = c.lh((vt as u32).wrapping_add(0x8));
        let write = c.lw((vt as u32).wrapping_add(0xc));
        c.g[V1][0] = write;
        c.sw(frame.at(c, 0), n);
        c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
        call_ptr(c, write as u32, 0x0015_60fc)?;
        let count = c.lw(frame.at(c, 0));
        c.g[V0][0] = 0x004a_0000;
        if count as i64 > 0 {
            c.cycles += 4;
            c.g[FP][0] = 0x9b50;
            c.g[S7][0] = sx(TABLE);
            c.g[S6][0] = 0xf88;
            c.g[A0][0] = c.g[S1][0];
            loop {
                // profile row of rider s1
                c.cycles += 6;
                c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                GET_PROFILE.call(c, 0x0015_6120)?;
                c.g[S2][0] = 0xbc1;
                c.mult(0, S4, V0, FP);
                c.g[S0][0] = 0x9;
                c.g[A0][0] = c.g[S1][0];
                loop {
                    c.cycles += 15;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                    GET_CHARACTER.call(c, 0x0015_6138)?;
                    c.mult(0, V1, V0, S6);
                    c.g[A2][0] = 0x3;
                    let row = (c.g[V1][0] as u32).wrapping_add(c.g[S7][0] as u32);
                    let stream = c.g[S3][0] as u32;
                    let vt = c.lw(stream);
                    c.g[V1][0] = vt;
                    let row = (c.g[S4][0] as u32).wrapping_add(row);
                    c.g[V0][0] = sx(row);
                    c.g[A1][0] = sx(row.wrapping_add(c.g[S2][0] as u32));
                    let delta = c.lh((vt as u32).wrapping_add(0x8));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(3));
                    let write = c.lw((vt as u32).wrapping_add(0xc));
                    c.g[A3][0] = write;
                    c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
                    call_ptr(c, write as u32, 0x0015_6164)?;
                    if (c.g[S0][0] as i64) < 0 {
                        break;
                    }
                    c.g[A0][0] = c.g[S1][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_6130);
                    }
                }
                c.cycles += 5;
                let count = c.lw(frame.at(c, 0));
                c.g[S1][0] = c.g[S5][0];
                let more = ((c.g[S1][0] as i64) < (count as i64)) as u64;
                c.g[V0][0] = more;
                if more == 0 {
                    break;
                }
                c.g[A0][0] = c.g[S1][0];
                if event_due(c) {
                    deliver_event_here(c, 0x0015_6118);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
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

/// `void WorldState::WriteDeadObjects(cPbStream* s)` (0x26da88): counts the
/// processes in list 8 of the process manager (gp+0x2898), writes the count
/// as an 8-byte record (`s->Write(&rec, 8)`, vtable +8), then for each of
/// those processes the word at +0x78 of its object (node +0x18).
pub fn world_state_write_dead_objects(c: &mut Cpu) {
    const PROCESS_LIST_FIRST: u32 = 0x0035_4ea8;
    const PROCESS_LIST_NEXT: u32 = 0x0035_4ef8;
    const DEAD_LIST: u64 = 0x8;
    const OBJECT: u32 = 0x18;
    const SAVED_WORD: u32 = 0x78;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        c.g[A0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2898));
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = DEAD_LIST;
        // rec = { count, 0 } on the stack
        c.sw(frame.at(c, 0), 0);
        c.sw(frame.at(c, 4), 0);
        call_ptr(c, PROCESS_LIST_FIRST, 0x0026_dab8)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = c.lw(frame.at(c, 0));
            loop {
                c.cycles += 9;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = DEAD_LIST;
                let count = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.g[V0][0] = count;
                c.sw(frame.at(c, 0), count);
                call_ptr(c, PROCESS_LIST_NEXT, 0x0026_dae0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = c.lw(frame.at(c, 0));
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_dac8);
                }
            }
        }
        // s->Write(&rec, 8)
        let vt = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = vt;
        c.cycles += 9;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x8;
        let delta = c.lh((vt as u32).wrapping_add(0x8));
        let write = c.lw((vt as u32).wrapping_add(0xc));
        c.g[V1][0] = write;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(delta as u32));
        call_ptr(c, write as u32, 0x0026_db08)?;
        let count = c.lw(frame.at(c, 0));
        c.g[V0][0] = count;
        c.g[A0][0] = c.g[S2][0];
        let mut tail = true;
        if count as i64 > 0 {
            c.cycles += 5;
            c.g[A1][0] = DEAD_LIST;
            call_ptr(c, PROCESS_LIST_FIRST, 0x0026_db1c)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                frame.restore(c, S0, 0x50);
                tail = false;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(OBJECT));
                loop {
                    // s->Write(&object->word78 (copied to the stack), 4)
                    c.cycles += 16;
                    c.g[A1][0] = sx(frame.at(c, 0x10));
                    let vt = c.lw(c.g[S1][0] as u32);
                    c.g[V1][0] = vt;
                    c.g[A2][0] = 0x4;
                    let word = c.lw((c.g[A0][0] as u32).wrapping_add(SAVED_WORD));
                    c.sw(frame.at(c, 0x10), word);
                    let delta = c.lh((vt as u32).wrapping_add(0x8));
                    let write = c.lw((vt as u32).wrapping_add(0xc));
                    c.g[V0][0] = write;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(delta as u32));
                    call_ptr(c, write as u32, 0x0026_db54)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = DEAD_LIST;
                    call_ptr(c, PROCESS_LIST_NEXT, 0x0026_db64)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0 {
                        break;
                    }
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(OBJECT));
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_db30);
                    }
                }
            }
        }
        if tail {
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_8e20, cworld_cache_release_resource, "cWorldCache_ReleaseResource"),
    (0x0035_b5a8, multi_spline_modifier_save, "MultiSplineModifier_Save"),
    (0x0034_fd90, world_obj_kill, "WorldObj_Kill"),
    (0x0029_a530, ssx_audio_system_play_grab_trick_sound, "SSXAudioSystem_PlayGrabTrickSound"),
    (0x002f_6a58, rider_base_save_cheat_code, "RiderBase_SaveCheatCode"),
    (0x0013_30b8, rider_handplant_sub_save_state, "RiderHandplantSub_SaveState"),
    (0x0011_74b0, rider_metrix_save_state, "RiderMetrix_SaveState"),
    (0x0035_5280, mod_obj_ctor, "ModObj_ctor"),
    (0x0015_60a0, be_aggression_int_save_state, "BEAggressionInt_SaveState"),
    (0x0026_da88, world_state_write_dead_objects, "WorldState_WriteDeadObjects"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{DATA, GAME_GP};

    /// `cWorldCache_ReleaseResource`: a cache whose shared table (+4) and
    /// four slot tables (+8 array) each hold four kind entries with a
    /// 16-word reference array; the handle picks shared (1 in 3) or a slot,
    /// an index and a kind. The count is 1 (released) on 3 of 4 seeds; the
    /// resource is null, the gp+0x16e0 sentinel or a pointer. The owner's
    /// vtable +0x20 is a fake returning a small pool pointer (EA_MemFree
    /// runs for real on it), or on 1 in 5 seeds a real leaf returning 0.
    #[test]
    fn cworld_cache_release_resource_matches() {
        const CACHE: u32 = DATA;
        const SLOT_TABLES: u32 = DATA + 0x100;
        const TABLES: u32 = DATA + 0x400;
        const REFS: u32 = DATA + 0x1000;
        const OWNER: u32 = DATA + 0x3000;
        const VT: u32 = DATA + 0x3100;
        covered(ee::s13::f_003a8e20__generated, cworld_cache_release_resource, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3a8e20, 4);
            c.pc = 0x003a_8e20;
            // Tables 0..3 for the slots, table 4 the shared one.
            for t in 0..5u32 {
                let table = TABLES + 0x40 * t;
                for k in 0..4u32 {
                    c.bus.write32(table + 8 * k, r.next_u32());
                    c.bus.write32(table + 8 * k + 4, REFS + 0x400 * t + 0x40 * k);
                }
                if t < 4 {
                    c.bus.write32(SLOT_TABLES + 4 * t, table);
                }
            }
            c.bus.write32(CACHE + 4, TABLES + 0x40 * 4);
            c.bus.write32(CACHE + 8, SLOT_TABLES);
            let slot = if seed % 3 == 0 { 0xff } else { r.next_u32() % 4 };
            let t = if slot == 0xff { 4 } else { slot };
            let kind = r.next_u32() % 4;
            let index = r.next_u32() % 16;
            let count = if seed % 4 == 0 { 2 + r.next_u32() % 254 } else { 1 };
            let resource = match r.next_u32() % 5 {
                0 => 0,
                1 => GAME_GP + 0x16e0,
                _ => DATA + 0x8000 + (r.next_u32() & 0xfffc),
            };
            c.bus.write32(REFS + 0x400 * t + 0x40 * kind + 4 * index, (resource >> 2) << 8 | count);
            // The owner and its release method.
            c.bus.write32(CACHE + 0x3e8, OWNER);
            c.bus.write32(OWNER + 8, VT);
            c.bus.write16(VT + 0x20, (r.next_u32() % 0x40) as u16 & !3);
            c.bus.write32(VT + 0x24, if seed % 5 == 0 { 0x0014_06c0 } else { FAKE });
            c.bus.write32(LOG_RESULTS, 0x10 + 8 * (r.next_u32() % 0x80));
            c.g[A0][0] = sx(CACHE);
            c.g[A2][0] = kind as u64;
            let handle = index << 8 | slot;
            c.g[A3][0] = if seed % 7 == 0 { (r.next_u32() as u64) << 32 | handle as u64 } else { handle as u64 };
        });
    }

    /// `MultiSplineModifier_Save`: a stream whose vtable `Write` (+8) and
    /// `WritePtr` (+0x28) are fakes (logging every register, sometimes
    /// unwinding); `SplineModifier_SavePath` runs for real. The spline
    /// count is -1..5 (no loop for <= 0), the spline array holds random
    /// pointers.
    #[test]
    fn multi_spline_modifier_save_matches() {
        const ME: u32 = DATA;
        const STREAM: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        const SPLINES: u32 = DATA + 0x400;
        covered(ee::s11::f_0035b5a8__generated, multi_spline_modifier_save, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x35b5a8, 6);
            c.pc = 0x0035_b5a8;
            let count = (seed % 7) as i32 - 1;
            c.bus.write32(ME + 4, count as u32);
            c.bus.write32(ME + 0x40, r.next_u32());
            c.bus.write32(ME + 0x44, SPLINES);
            for i in 0..6 {
                c.bus.write32(SPLINES + 4 * i, DATA + 0x1000 + (r.next_u32() & 0xfff0));
            }
            c.bus.write32(STREAM, VT);
            for (k, off) in [8u32, 0x28].into_iter().enumerate() {
                c.bus.write16(VT + off, (r.next_u32() % 0x20) as u16 & !3);
                c.bus.write32(VT + off + 4, FAKE + 0x10 * k as u32);
            }
            c.g[A0][0] = sx(ME);
            c.g[A1][0] = sx(STREAM);
        });
    }

    /// `WorldObj_Kill`: an object with a node whose flags word is random
    /// (high half sign bit set or not) and a vtable whose slot +8 is a fake;
    /// on 1 in 8 seeds the object pointer is null (the flag writes go
    /// through low RAM and no call is made).
    #[test]
    fn world_obj_kill_matches() {
        const ME: u32 = DATA;
        const NODE: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        covered(ee::s11::f_0034fd90__generated, world_obj_kill, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x34fd90, 1);
            c.pc = 0x0034_fd90;
            c.bus.write32(ME + 0x18, NODE);
            c.bus.write32(NODE + 8, r.next_u32());
            c.bus.write32(ME + 0xc, VT);
            c.bus.write16(VT + 8, (r.next_u32() % 0x20) as u16 & !3);
            c.bus.write32(VT + 0xc, FAKE);
            c.bus.write32(0x18, if seed % 2 == 0 { 0 } else { NODE + 0x40 });
            c.g[A0][0] = if seed % 8 == 0 { 0 } else { sx(ME) };
        });
    }

    /// `SSXAudioSystem_PlayGrabTrickSound`: the MIDI bank fixture of the
    /// rail-change sibling (`midi_bank`: record arrays, cursor 0..3, the
    /// channel state at this+0x118); the rider's pending flag (+0x874) and
    /// count (+0x87c) cover "nothing pending", "count 0" and "play" (memset,
    /// GetChannelGainPtr and Audio_PlayPushedSound run for real).
    #[test]
    fn ssx_audio_system_play_grab_trick_sound_matches() {
        use crate::difftest::{check, scramble, Rng};
        use crate::game::sound::soundsys::tests::midi_bank;
        check(ee::s08::f_0029a530__generated, ssx_audio_system_play_grab_trick_sound, 200, &|c, seed| {
            let mut r = Rng(0x0029_a530 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0029_a530;
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = (DATA + 0x3000) as u64; // rider
            midi_bank(c, &mut r, seed);
            c.bus.write32(DATA + 0x3000 + 0x874, (seed % 3) as u32);
            c.bus.write32(DATA + 0x3000 + 0x87c, (seed % 5) as u32);
        });
    }

    /// A stream whose vtable `Write` (+8) is a fake (logs the registers,
    /// sometimes unwinds); `this` comes with junk upper bits on some seeds.
    fn stream_save_setup(c: &mut Cpu, seed: u32, entry: u32) {
        const ME: u32 = DATA;
        const STREAM: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        let mut r = machine(c, seed, entry as u64, 1);
        c.pc = entry;
        c.bus.write32(STREAM, VT);
        c.bus.write16(VT + 8, (r.next_u32() % 0x20) as u16 & !3);
        c.bus.write32(VT + 0xc, FAKE);
        c.g[A0][0] = if seed % 4 == 0 { (r.next_u32() as u64) << 32 | ME as u64 } else { sx(ME) };
        c.g[A1][0] = sx(STREAM);
    }

    /// `RiderBase_SaveCheatCode`.
    #[test]
    fn rider_base_save_cheat_code_matches() {
        covered(ee::s09::f_002f6a58__generated, rider_base_save_cheat_code, 200, &|c, seed| {
            stream_save_setup(c, seed, 0x002f_6a58)
        });
    }

    /// `RiderHandplantSub_SaveState`.
    #[test]
    fn rider_handplant_sub_save_state_matches() {
        covered(ee::s01::f_001330b8__generated, rider_handplant_sub_save_state, 200, &|c, seed| {
            stream_save_setup(c, seed, 0x0013_30b8)
        });
    }

    /// `RiderMetrix_SaveState`.
    #[test]
    fn rider_metrix_save_state_matches() {
        covered(ee::s00::f_001174b0__generated, rider_metrix_save_state, 200, &|c, seed| {
            stream_save_setup(c, seed, 0x0011_74b0)
        });
    }

    /// `ModObj_ctor`: the `WorldObj_ctor` fixture (its test in
    /// game/object/floatingnode.rs): process lists, the world -> focus
    /// manager chain, node flags with or without 0x100, kinds 6/0x10/0x16 or
    /// random; the WorldObj constructor runs for real.
    #[test]
    fn mod_obj_ctor_matches() {
        use crate::difftest::{check, fill};
        const SELF: u32 = DATA + 0x100;
        const NODE: u32 = DATA + 0x200;
        const PROCS: u32 = DATA + 0x1000;
        const TAILS: u32 = DATA + 0x1400;
        const WORLD: u32 = DATA + 0x2000;
        const FOCUS: u32 = DATA + 0x3000;
        check(ee::s11::f_00355280__generated, mod_obj_ctor, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x35_5280, 0);
            c.pc = 0x0035_5280;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            fill(c, &mut r, SELF, 0x10);
            fill(c, &mut r, NODE, 0x10);
            let flags = r.next_u32();
            w(c, NODE + 0x8, if seed % 4 == 0 { flags | 0x100 } else { flags & !0x100 });
            let list = seed % 4;
            w(c, GAME_GP + 0x2898 + 4, PROCS);
            for i in 0..4 {
                w(c, PROCS + 0x44 * i, r.next_u32());
                w(c, PROCS + 0x44 * i + 8, TAILS + 0x40 * i);
                w(c, PROCS + 0x44 * i + 0x28, TAILS + 0x40 * i + 0x20);
            }
            let kind = match (seed / 4) % 6 {
                0 => 6,
                1 => 0x10,
                2 => 0x16,
                _ => r.next_u32() % 0x20,
            };
            fill(c, &mut r, 0x0044_aff0, 0x20);
            // World -> game -> race -> focus manager.
            w(c, GAME_GP - 0x848, WORLD);
            w(c, WORLD + 0x84, WORLD + 0x100);
            w(c, WORLD + 0x100 + 0xc, WORLD + 0x200);
            w(c, WORLD + 0x200 + 0xa4, FOCUS);
            w(c, FOCUS + 0xd4, if seed % 3 == 0 { 0 } else { r.next_u32() });
            w(c, FOCUS + 0xd8, r.next_u32() % 8);
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(list);
            c.g[A2][0] = sx(kind);
            c.g[A3][0] = sx(NODE);
        });
    }

    /// `BEAggressionInt_SaveState`: rider count -1..4 at 0x5305e0, the
    /// rider index table (0x5305b0) pointing at rider records (0x535b20,
    /// 0x1c each) with profile -1 / random flags and a character byte;
    /// the stream's `Write` (vtable +8) is a fake (logs the registers,
    /// sometimes unwinds). BE_GetRiderProfile/Character run for real.
    #[test]
    fn be_aggression_int_save_state_matches() {
        const STREAM: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        covered(ee::s01::f_001560a0__generated, be_aggression_int_save_state, 200, &|c, seed| {
            let n = (seed % 6) as i32 - 1;
            let calls = 1 + 10 * n.max(0) as u32;
            let mut r = machine(c, seed, 0x1560a0, calls);
            c.pc = 0x0015_60a0;
            c.bus.write32(0x0053_05e0, n as u32);
            for i in 0..4u32 {
                c.bus.write32(0x0053_05b0 + 4 * i, r.next_u32() % 8);
            }
            for k in 0..8u32 {
                let rec = 0x0053_5b20 + 0x1c * k;
                c.bus.write32(rec + 0xc, if r.next_u32() % 3 == 0 { u32::MAX } else { r.next_u32() });
                c.bus.write32(rec + 0x10, r.next_u32());
            }
            c.bus.write32(STREAM, VT);
            c.bus.write16(VT + 8, (r.next_u32() % 0x20) as u16 & !3);
            c.bus.write32(VT + 0xc, FAKE);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(STREAM);
        });
    }

    /// `WorldState_WriteDeadObjects`: the process manager at gp+0x2898
    /// points at list headers (0x44 each); list 8 holds 0..4 nodes (empty:
    /// first == the sentinel at header+0x14), each with an object (+0x18)
    /// whose word +0x78 is random. ProcessList_First/Next run for real; the
    /// stream's `Write` (vtable +8) is a fake (logs the registers, sometimes
    /// unwinds).
    #[test]
    fn world_state_write_dead_objects_matches() {
        const STREAM: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        const PROCS: u32 = DATA + 0x1000;
        const NODES: u32 = DATA + 0x2000;
        const OBJECTS: u32 = DATA + 0x3000;
        covered(ee::s07::f_0026da88__generated, world_state_write_dead_objects, 240, &|c, seed| {
            let nodes = seed % 5;
            let mut r = machine(c, seed, 0x26da88, 1 + nodes);
            c.pc = 0x0026_da88;
            c.bus.write32(GAME_GP + 0x2898 + 4, PROCS);
            let head = PROCS + 8 * 0x44;
            let sentinel = head + 0x14;
            let node = |i: u32| if i < nodes { NODES + 0x40 * i } else { sentinel };
            c.bus.write32(head + 8, node(0));
            for i in 0..nodes {
                c.bus.write32(node(i) + 4, node(i + 1));
                c.bus.write32(node(i) + 0x18, OBJECTS + 0x100 * i);
                c.bus.write32(OBJECTS + 0x100 * i + 0x78, r.next_u32());
            }
            c.bus.write32(STREAM, VT);
            c.bus.write16(VT + 8, (r.next_u32() % 0x20) as u16 & !3);
            c.bus.write32(VT + 0xc, FAKE);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(STREAM);
        });
    }
}
