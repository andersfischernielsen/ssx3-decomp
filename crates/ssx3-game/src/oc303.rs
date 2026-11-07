//! Staging wave 212 (oc303): hand-written replacements for the hottest
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

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002a_c220, bank_monitor_instance_play, "BankMonitorInstance_Play"),
];

// ---------------------------------------------------------------------------
// BankMonitorInstance_Play (0x2ac220, streamsys.cpp).

/// Play-state stack the bank monitor plays through (`a1`): the current
/// play descriptor, a depth index and per-depth parameter arrays.
pub(crate) const BANK_PLAY_CUR_DESC: u32 = 0x04; // 24-byte descriptor
pub(crate) const BANK_PLAY_DEPTH: u32 = 0x1c;
pub(crate) const BANK_PLAY_DESCS: u32 = 0x20; // -> descriptors, 0x18 each
pub(crate) const BANK_PLAY_F24: u32 = 0x24; // inline words
pub(crate) const BANK_PLAY_F34: u32 = 0x34; // -> words
pub(crate) const BANK_PLAY_VOLUME: u32 = 0x38; // -> words (clamped to 0..127)
pub(crate) const BANK_PLAY_F3C: u32 = 0x3c; // -> words
pub(crate) const BANK_PLAY_F40: u32 = 0x40; // inline 8-byte entries
pub(crate) const BANK_PLAY_ENV: u32 = 0x70; // -> 16-byte envelopes
/// Monitor instance fields (`a0`).
pub(crate) const BANK_MON_KIND: u32 = 0x00; // 1: bank patch, 2: MIDI, 3: none
pub(crate) const BANK_MON_HANDLE: u32 = 0x04;
pub(crate) const BANK_MON_PLAYING: u32 = 0x08;
pub(crate) const BANK_MON_BANK: u32 = 0x0c;
pub(crate) const BANK_MON_PATCH: u32 = 0x10;
pub(crate) const BANK_MON_F54: u32 = 0x54;
pub(crate) const BANK_MON_F60: u32 = 0x60;

const BANK_PLAY_MEMSET: u32 = 0x0041_6210;
const BANK_PLAY_SND_LOCK: u32 = 0x003b_58a0;
const BANK_PLAY_SND_UNLOCK: u32 = 0x003b_58d8;
const BANK_PLAY_SND_PLAY_PATCH: u32 = 0x003b_6820;
const BANK_PLAY_SNDMIDI_PLAY: u32 = 0x003b_9a00;

/// `lw ra_, depth(st); lw rb, field(st); sll ra_, shift; addu ra_, rb;
/// sw val, extra(ra_)`: one per-depth array store, in the original's
/// registers.
fn bank_play_slot_store(c: &mut Cpu, ra_: usize, rb: usize, field: u32, shift: u32, extra: u32, val: u64) {
    let st = c.g[S1][0] as u32;
    c.g[ra_][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[rb][0] = c.lw(st.wrapping_add(field));
    c.g[ra_][0] = sx((c.g[ra_][0] as u32) << shift);
    c.g[ra_][0] = sx((c.g[ra_][0] as u32).wrapping_add(c.g[rb][0] as u32));
    c.sw((c.g[ra_][0] as u32).wrapping_add(extra), val);
}

/// `v0 = &volume[depth]` (`lw v0, depth; sll 2; addu a2`).
fn bank_play_volume_at(c: &mut Cpu) -> u32 {
    let st = c.g[S1][0] as u32;
    c.g[V0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] as u32
}

/// The shared head of both play kinds: push a new depth (copy of the
/// current descriptor), reset its per-depth state, clear its 8-byte entry
/// (memset, returning to `memset_ret`), set the default envelope (100, 90,
/// 50), volume 127, flag 1, take the instance's +0x60 word, and clamp the
/// pushed descriptor's first byte to 0..127 from `volume[depth]`. Leaves
/// `v0` = the clamped byte and `a0` = its address (the `sb` happens in the
/// caller's next block).
fn bank_play_push_depth(c: &mut Cpu, memset_ret: u32) -> crate::call::Flow {
    c.cycles += 142;
    let st = c.g[S1][0] as u32;
    // ++depth; descs[depth] = cur (unaligned 24-byte copy).
    c.g[V1][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[S0][0] = 0x18;
    c.g[A0][0] = c.lw(st.wrapping_add(BANK_PLAY_DESCS));
    c.g[A1][0] = 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.g[A2][0] = 8;
    c.sw(st.wrapping_add(BANK_PLAY_DEPTH), c.g[V1][0]);
    c.g[V0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.mult(0, V0, V0, S0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let src = st.wrapping_add(BANK_PLAY_CUR_DESC);
    c.ldl(V1, src.wrapping_add(7));
    c.ldr(V1, src);
    c.ldl(A0, src.wrapping_add(0xf));
    c.ldr(A0, src.wrapping_add(8));
    c.ldl(A3, src.wrapping_add(0x17));
    c.ldr(A3, src.wrapping_add(0x10));
    let dst = c.g[V0][0] as u32;
    c.sdl(V1, dst.wrapping_add(7));
    c.sdr(V1, dst);
    c.sdl(A0, dst.wrapping_add(0xf));
    c.sdr(A0, dst.wrapping_add(8));
    c.sdl(A3, dst.wrapping_add(0x17));
    c.sdr(A3, dst.wrapping_add(0x10));
    // f24[depth] (inline) = 0.
    c.g[V1][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx(st.wrapping_add(c.g[V1][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(BANK_PLAY_F24), 0);
    bank_play_slot_store(c, V0, V1, BANK_PLAY_F34, 2, 0, 0);
    // volume[depth] = descs[depth].c[0] (signed byte).
    c.g[A3][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[A0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[T0][0] = c.lw(st.wrapping_add(BANK_PLAY_DESCS));
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, A0, A0, S0);
    c.g[V0][0] = c.lw(st.wrapping_add(BANK_PLAY_VOLUME));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
    c.sw(c.g[A3][0] as u32, c.g[V1][0]);
    bank_play_slot_store(c, V0, V1, BANK_PLAY_F3C, 2, 0, 0);
    // memset(&f40[depth], 0, 8).
    c.g[A0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx(st.wrapping_add(c.g[A0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(BANK_PLAY_F40));
    call_ptr(c, BANK_PLAY_MEMSET, memset_ret)?;

    // Per-depth defaults.
    c.g[A0][0] = 0x64;
    c.g[A1][0] = 0x5a;
    c.g[A2][0] = 0x32;
    c.g[A3][0] = 0x7f;
    bank_play_slot_store(c, V1, V0, 0x60, 2, 0, 0);
    bank_play_slot_store(c, V0, V1, 0x64, 2, 0, 0);
    bank_play_slot_store(c, V1, V0, 0x68, 2, 0, 0);
    bank_play_slot_store(c, V0, V1, 0x6c, 2, 0, 0);
    bank_play_slot_store(c, V1, V0, BANK_PLAY_ENV, 4, 4, 0);
    let (a0, a1, a2, a3, s3) = (c.g[A0][0], c.g[A1][0], c.g[A2][0], c.g[A3][0], c.g[S3][0]);
    bank_play_slot_store(c, V0, V1, BANK_PLAY_ENV, 4, 0, a0);
    bank_play_slot_store(c, V1, V0, BANK_PLAY_ENV, 4, 8, a1);
    bank_play_slot_store(c, V0, V1, BANK_PLAY_ENV, 4, 0xc, a2);
    bank_play_slot_store(c, V1, V0, 0x74, 2, 0, 0);
    bank_play_slot_store(c, V0, V1, 0x80, 2, 0, 0);
    bank_play_slot_store(c, V1, V0, 0x84, 2, 0, 0);
    bank_play_slot_store(c, V0, V1, 0x78, 2, 0, a3);
    bank_play_slot_store(c, V1, V0, 0x7c, 2, 0, s3);
    // f3c[depth] = instance->f60; volume[depth] = 0.
    let inst = c.g[S2][0] as u32;
    let f60 = c.lw(inst.wrapping_add(BANK_MON_F60));
    // (the original loads depth and the pointer before this word; loads only)
    let st = c.g[S1][0] as u32;
    c.g[A0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[V0][0] = c.lw(st.wrapping_add(BANK_PLAY_F3C));
    c.g[V1][0] = f60;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
    bank_play_slot_store(c, V0, V1, BANK_PLAY_VOLUME, 2, 0, 0);

    // descs[depth].c[0] = clamp(volume[depth], 0, 127).
    c.g[A0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[V1][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[A2][0] = c.lw(st.wrapping_add(BANK_PLAY_VOLUME));
    c.mult(0, A0, A0, S0);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A1][0] = c.lw(st.wrapping_add(BANK_PLAY_DESCS));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    let below_max = c.g[V0][0] != 0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    if below_max {
        c.cycles += 6;
        let at = bank_play_volume_at(c);
        c.g[V1][0] = c.lw(at);
        let negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0;
        if negative {
            return Ok(());
        }
    }
    c.cycles += 7;
    let at = bank_play_volume_at(c);
    c.g[V1][0] = c.lw(at);
    c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0x7f;
    } else {
        c.cycles += 4;
        let at = bank_play_volume_at(c);
        c.g[V0][0] = c.lbu(at);
    }
    Ok(())
}

/// `void BankMonitorInstance_Play(BankMonitorInstance* self, PlayStack* st)`
/// (0x2ac220): mark the instance playing, then by its kind push a play
/// depth with default parameters and start it under the sound lock: kind 1
/// plays a bank patch (`SND_PlayPatch(bank, patch, desc)`), kind 2 a MIDI
/// stream (`SNDMIDI_Play(desc, bank, patch, -1)`); the handle goes to
/// `self+4` and the depth is popped again. Other kinds do nothing.
pub fn bank_monitor_instance_play(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 2;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = 1;
        frame.save_ra(c, 0);
        let inst = c.g[S2][0] as u32;
        c.sw(inst.wrapping_add(BANK_MON_PLAYING), c.g[S3][0]);
        c.g[V1][0] = c.lw(inst.wrapping_add(BANK_MON_KIND));
        c.sw(inst.wrapping_add(BANK_MON_F54), 0);
        let kind = c.g[V1][0];

        if kind == 2 {
            // MIDI stream: SNDMIDI_Play(&descs[depth], bank, patch, -1).
            bank_play_push_depth(c, 0x002a_c5e4)?;
            c.cycles += 11;
            c.sb(c.g[A0][0] as u32, c.g[V0][0]);
            call_ptr(c, BANK_PLAY_SND_LOCK, 0x002a_c79c)?;
            let st = c.g[S1][0] as u32;
            let inst = c.g[S2][0] as u32;
            c.g[A0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
            c.g[V0][0] = 0x18;
            c.g[V1][0] = c.lw(st.wrapping_add(BANK_PLAY_DESCS));
            c.g[A3][0] = u64::MAX;
            c.mult(0, A0, A0, V0);
            c.g[A1][0] = c.lw(inst.wrapping_add(BANK_MON_BANK));
            c.g[A2][0] = c.lw(inst.wrapping_add(BANK_MON_PATCH));
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, BANK_PLAY_SNDMIDI_PLAY, 0x002a_c7c0)?;
            bank_play_finish(c, frame)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((kind as i64) < 3) as u64;
            frame.restore(c, S0, 0x40);
            if c.g[V0][0] == 0 {
                // Kind 3 and up: nothing to play.
                c.cycles += 1;
                frame.restore(c, S1, 0x30);
            } else {
                c.cycles += 2;
                if kind != c.g[S3][0] {
                    frame.restore(c, S1, 0x30);
                } else {
                    // Bank patch: SND_PlayPatch(bank, patch, &descs[depth]).
                    bank_play_push_depth(c, 0x002a_c338)?;
                    c.cycles += 12;
                    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                    call_ptr(c, BANK_PLAY_SND_LOCK, 0x002a_c4f0)?;
                    let st = c.g[S1][0] as u32;
                    let inst = c.g[S2][0] as u32;
                    c.g[A2][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
                    c.g[V0][0] = 0x18;
                    c.g[V1][0] = c.lw(st.wrapping_add(BANK_PLAY_DESCS));
                    c.mult(0, A2, A2, V0);
                    c.g[A0][0] = c.lw(inst.wrapping_add(BANK_MON_BANK));
                    c.g[A1][0] = c.lw(inst.wrapping_add(BANK_MON_PATCH));
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    call_ptr(c, BANK_PLAY_SND_PLAY_PATCH, 0x002a_c510)?;
                    bank_play_finish(c, frame)?;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Common tail of both play kinds: store the handle, `SND_Unlock`, pop
/// the depth, restore s0/s1.
fn bank_play_finish(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 6;
    let inst = c.g[S2][0] as u32;
    c.sw(inst.wrapping_add(BANK_MON_HANDLE), c.g[V0][0]);
    call_ptr(c, BANK_PLAY_SND_UNLOCK, 0x002a_c7c8)?;
    let st = c.g[S1][0] as u32;
    c.g[V0][0] = c.lw(st.wrapping_add(BANK_PLAY_DEPTH));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    c.sw(st.wrapping_add(BANK_PLAY_DEPTH), c.g[V0][0]);
    frame.restore(c, S0, 0x40);
    c.cycles += 1;
    frame.restore(c, S1, 0x30);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // BankMonitorInstance_Play.

    /// A monitor instance at DATA and its play stack at DATA+0x1000. Kinds
    /// 1 (bank patch) and 2 (MIDI) push a depth and play under the sound
    /// lock (`SND_Lock`/`SND_Unlock` call through pointers we aim at
    /// fakes, which log and sometimes unwind); 0, 3, -1 and 7 do nothing.
    /// Every other seed of the playing kinds aliases the stack's pointers
    /// so the clamp reads a chosen word (above 127, negative, in range):
    /// `f3c` points at the volume pointer, the instance's +0x60 word (stored
    /// there) points the zeroing store at the depth, and the clamp then
    /// reads a word of the current descriptor.
    fn bank_monitor_play_setup(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{fill, DATA};
        let mut r = machine(c, seed, 0x2a_c220, 2);
        c.pc = 0x002a_c220;
        let inst = DATA;
        let st = DATA + 0x1000;
        let patch = DATA + 0x2000;
        c.g[A0][0] = sx(inst);
        c.g[A1][0] = sx(st);
        // SND_Lock / SND_Unlock interrupt hooks.
        c.bus.write32(0x0044_c46c, FAKE);
        c.bus.write32(0x0044_c470, FAKE + 0x10);

        let kinds = [1u32, 2, 1, 2, 0, 3, 0xffff_ffff, 7];
        let kind = kinds[(seed % 8) as usize];
        let alias = (seed / 8) % 2 == 1;
        fill(c, &mut r, inst, 0x20);
        c.bus.write32(inst + BANK_MON_KIND, kind);
        let bank = if r.next_u32() % 4 == 0 { 0xffff_fff0 } else { r.next_u32() % 8 };
        c.bus.write32(inst + BANK_MON_BANK, bank);
        c.bus.write32(inst + BANK_MON_PATCH, patch);
        fill(c, &mut r, patch, 8);
        if r.next_u32() % 2 == 0 {
            c.bus.write16(patch, 0x444d);
        }

        fill(c, &mut r, st, 0x22);
        let depth = (r.next_u32() % 3).wrapping_sub(1); // -1..1, pushed to 0..2
        c.bus.write32(st + BANK_PLAY_DEPTH, depth);
        c.bus.write32(st + BANK_PLAY_DESCS, DATA + 0x3000);
        fill(c, &mut r, DATA + 0x3000, 0x20);
        c.bus.write32(st + BANK_PLAY_F34, DATA + 0x4000);
        c.bus.write32(st + BANK_PLAY_VOLUME, DATA + 0x4100);
        c.bus.write32(st + BANK_PLAY_F3C, DATA + 0x4200);
        for (i, off) in [0x60u32, 0x64, 0x68, 0x6c, 0x74, 0x78, 0x7c, 0x80, 0x84]
            .into_iter()
            .enumerate()
        {
            c.bus.write32(st + off, DATA + 0x4300 + 0x100 * i as u32);
        }
        c.bus.write32(st + BANK_PLAY_ENV, DATA + 0x5000);
        if alias && kind <= 2 {
            let d = 1 + r.next_u32() % 2; // pushed depth 1 or 2
            c.bus.write32(st + BANK_PLAY_DEPTH, d - 1);
            c.bus.write32(st + BANK_PLAY_F3C, st + BANK_PLAY_VOLUME - 4 * d);
            c.bus.write32(inst + BANK_MON_F60, st + BANK_PLAY_DEPTH - 4 * d);
            let words = [200u32, 0xffff_fffb, 50, 127, 126, 0x8000_0000, 0x7f, 0x1234_5605];
            let w = words[(seed / 16 % 8) as usize];
            c.bus.write32(st + BANK_PLAY_DEPTH - 4 * d, w);
        }
    }

    /// Paths (kind 1 / 2 / none) and the clamp branches, with the sound
    /// lock fakes unwinding at either call.
    #[test]
    fn bank_monitor_instance_play_matches() {
        crate::difftest::fakes::covered(
            ee::s08::f_002ac220__generated,
            bank_monitor_instance_play,
            256,
            &bank_monitor_play_setup,
        );
        crate::difftest::check_lohi(
            ee::s08::f_002ac220__generated,
            bank_monitor_instance_play,
            128,
            &bank_monitor_play_setup,
        );
    }
}
