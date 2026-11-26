//! Staging module oc767 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc767`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0020_a1a8, fe_format_min_sec, "FE_FormatMinSec"),
    (0x002a_e4c0, path_plat_add_track, "PATH_PlatAddTrack"),
    (0x0028_a558, snd_load_queue_push, "SndLoadQueue_Push"),
    (0x003d_2350, path_add_map, "PATH_AddMap"),
    (0x002b_35a0, music_player_play_song, "MusicPlayer_PlaySong"),
];

/// `reg + off` as a 32-bit guest address.
#[inline(always)]
fn oc767_ea(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `lw` from `off(reg)` (sign-extended).
#[inline(always)]
fn oc767_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = oc767_ea(c, reg, off);
    c.lw(a)
}

/// `sw value, off(reg)`.
#[inline(always)]
fn oc767_sw(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = oc767_ea(c, reg, off);
    c.sw(a, v);
}

/// `reg = sx(reg_a + reg_b)` (`addu`).
#[inline(always)]
fn oc767_addu(c: &mut Cpu, rd: usize, a: usize, b: usize) {
    c.g[rd][0] = sx((c.g[a][0] as u32).wrapping_add(c.g[b][0] as u32));
}

/// `reg = sx(reg << sh)` (`sll`).
#[inline(always)]
fn oc767_sll(c: &mut Cpu, rd: usize, rt: usize, sh: u32) {
    c.g[rd][0] = sx((c.g[rt][0] as u32) << sh);
}

/// `rd = sx(rs + imm)` (`addiu`).
#[inline(always)]
fn oc767_addiu(c: &mut Cpu, rd: usize, rs: usize, imm: u32) {
    c.g[rd][0] = sx((c.g[rs][0] as u32).wrapping_add(imm));
}

/// Where `cBEOptionInterface_mOption` (the 0x288-byte option block) lives.
pub(crate) const FE_FORMAT_MIN_SEC_OPTIONS: u32 = 0x0053_5610;

/// `char* FE_FormatMinSec(int secs)` (0x20a1a8): formats `secs` as minutes
/// and seconds into the shared clock buffer at 0x4c6848 and returns it.
/// The format depends on the option block's mode (bits 22..24 of its first
/// word, copied to the stack first): 0/2 use the format at 0x4719f0, 1/3 the
/// one at 0x471a00, other modes leave the buffer alone.
///
/// The generated body also holds an unaligned copy loop for an option block
/// that is not 8-byte aligned. The block address is the constant 0x535610,
/// so that branch is dead and not reproduced.
pub fn fe_format_min_sec(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x2e0);
        frame.save(c, 16, 0x2d0);
        frame.save(c, 17, 0x2c0);
        frame.save(c, 18, 0x2b0);
        frame.save(c, 19, 0x2a0);
        frame.save_ra(c, 0x290);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0020_a1c8)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_dd58, 0x0020_a1d4)?; // cBE_getInterface
        c.g[V1][0] = 0x3c;
        c.g[V0][0] = 0x3c;
        c.div(0, S0, V1); // minutes = secs / 60, seconds = secs % 60

        // Copy the option block (0x280 bytes in 32-byte chunks, then 8 more
        // bytes) to the bottom of the frame.
        c.cycles += 7;
        c.g[A0][0] = sx(FE_FORMAT_MIN_SEC_OPTIONS);
        c.g[V0][0] = c.g[A0][0] & 7;
        c.g[S0][0] = c.lo[0];
        c.g[S1][0] = c.hi[0];
        c.g[V1][0] = c.g[SP][0];
        c.cycles += 2;
        oc767_addiu(c, V0, A0, 0x280);
        c.g[S3][0] = 0x4c0000;
        loop {
            c.cycles += 11;
            c.g[A1][0] = Addr(oc767_ea(c, A0, 0)).ld(c);
            c.g[A2][0] = Addr(oc767_ea(c, A0, 8)).ld(c);
            c.g[A3][0] = Addr(oc767_ea(c, A0, 0x10)).ld(c);
            c.g[T0][0] = Addr(oc767_ea(c, A0, 0x18)).ld(c);
            let (d, v) = (oc767_ea(c, V1, 0), c.g[A1][0]);
            c.sd(d, v);
            let (d, v) = (oc767_ea(c, V1, 8), c.g[A2][0]);
            c.sd(d, v);
            let (d, v) = (oc767_ea(c, V1, 0x10), c.g[A3][0]);
            c.sd(d, v);
            let (d, v) = (oc767_ea(c, V1, 0x18), c.g[T0][0]);
            c.sd(d, v);
            oc767_addiu(c, A0, A0, 0x20);
            let again = c.g[A0][0] != c.g[V0][0];
            oc767_addiu(c, V1, V1, 0x20);
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0020_a268);
            }
        }
        c.cycles += 10;
        let a = oc767_ea(c, A0, 7);
        c.ldl(2, a);
        let a = oc767_ea(c, A0, 0);
        c.ldr(2, a);
        let a = oc767_ea(c, V1, 7);
        c.sdl(2, a);
        let a = oc767_ea(c, V1, 0);
        c.sdr(2, a);
        c.g[V0][0] = oc767_lw(c, SP, 0); // option flags
        c.g[V1][0] = 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 22);
        c.g[A0][0] = c.g[V0][0] & 7;
        let mode = c.g[A0][0];

        // Mode 1 and 3 (format B), 0 and 2 (format A), others: no text.
        let mut format = 0; // 0: none, 1: A, 2: B
        if mode == c.g[V1][0] {
            format = 2;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 2;
            if mode == 0 {
                format = 1;
            } else {
                c.cycles += 2;
                let t = mode == c.g[V0][0];
                c.g[V0][0] = 3;
                if t {
                    format = 1;
                } else {
                    c.cycles += 2;
                    let t = mode == c.g[V0][0];
                    oc767_addiu(c, V0, S3, 0x6848);
                    if t {
                        format = 2;
                    } else {
                        c.cycles += 2;
                        frame.restore(c, 16, 0x2d0);
                    }
                }
            }
        }
        if format != 0 {
            let (cycles, fmt, r1, r2, r3) = if format == 1 {
                (17, 0x19f0, 0x0020_a2f4, 0x0020_a304, 0x0020_a318)
            } else {
                (16, 0x1a00, 0x0020_a338, 0x0020_a348, 0x0020_a35c)
            };
            c.cycles += cycles;
            c.mtc1(16, 12);
            c.cvt_s_w(12, 12);
            oc767_addiu(c, S2, S3, 0x6848);
            c.g[S0][0] = 0x470000;
            oc767_addiu(c, S0, S0, fmt);
            call_ptr(c, 0x0041_3af8, r1)?; // fptodp(minutes)
            c.mtc1(17, 12);
            c.cvt_s_w(12, 12);
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x0041_3af8, r2)?; // fptodp(seconds)
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[V0][0];
            call_ptr(c, 0x0041_7828, r3)?; // sprintf(buffer, fmt, min, sec)
            oc767_addiu(c, V0, S3, 0x6848);
            c.cycles += 1;
            frame.restore(c, 16, 0x2d0);
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x2c0);
        frame.restore(c, 18, 0x2b0);
        frame.restore(c, 19, 0x2a0);
        frame.restore_ra(c, 0x290);
        frame.close(c);
        Ok(())
    });
}

/// `PATH_PlatFindFreeTrackSlot` (0x2ae1b8).
const OC767_PLAT_FIND_FREE_SLOT: Callee = Callee {
    addr: 0x002a_e1b8,
    f: ee::s08::f_002ae1b8,
};

/// `PATH_PlatTrackInit` (0x2ae138).
const OC767_PLAT_TRACK_INIT: Callee = Callee {
    addr: 0x002a_e138,
    f: ee::s08::f_002ae138,
};

/// Offsets of the platform track player (`tPathPlayer`) in guest memory.
pub(crate) const PLAT_TRACK_HANDLE: u32 = 0x00;
pub(crate) const PLAT_TRACK_REQUEST_COUNT: u32 = 0x66;
pub(crate) const PLAT_TRACK_IS_BANK_PLAYER: u32 = 0x64;
pub(crate) const PLAT_TRACK_REQUESTS: u32 = 0x6c;

/// `int PATH_PlatAddTrack(char* name, SNDPLAYOPTS* desc, int count,
/// int maxChunks, char* mem, int size)` (0x2ae4c0): takes a free track slot,
/// builds the player in `mem` (request array of `count` 12-byte entries, each
/// handle -1, then the rest as stream memory), copies the play options
/// (`desc`, or the defaults), formats its name, creates the stream when
/// `maxChunks > 0` (otherwise it is a bank player) and returns the slot, -13
/// when none is free, or the stream's error handle.
pub fn path_plat_add_track(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 17, 0x70);
        frame.save(c, 18, 0x60);
        c.g[S1][0] = c.g[T0][0]; // mem
        frame.save(c, 19, 0x50);
        c.g[S2][0] = c.g[A2][0]; // count
        frame.save(c, 21, 0x30);
        c.g[S3][0] = c.g[T1][0]; // size
        frame.save(c, 22, 0x20);
        c.g[S5][0] = c.g[A1][0]; // desc
        frame.save(c, 23, 0x10);
        c.g[S6][0] = c.g[A3][0]; // maxChunks
        frame.save(c, 20, 0x40);
        frame.save(c, 16, 0x80);
        frame.save_ra(c, 0);
        c.g[S7][0] = c.g[A0][0]; // name
        OC767_PLAT_FIND_FREE_SLOT.call(c, 0x002a_e504)?;
        c.g[S4][0] = c.g[V0][0];
        let none = (c.g[S4][0] as i64) < 0;
        c.g[V0][0] = 0xffff_ffff_ffff_fff3;
        if !none {
            c.cycles += 26;
            c.g[V0][0] = 0x4d0000;
            oc767_sll(c, S0, S4, 2);
            oc767_addiu(c, V0, V0, 0x3e98);
            oc767_addu(c, S0, S0, V0); // &slots[slot]
            let v = c.g[S1][0];
            oc767_sw(c, S0, 0, v);
            oc767_addiu(c, S1, S1, 0x70);
            c.g[A0][0] = oc767_lw(c, S0, 0);
            OC767_PLAT_TRACK_INIT.call(c, 0x002a_e530)?;
            c.g[V0][0] = 0xc;
            c.g[S0][0] = oc767_lw(c, S0, 0); // the player
            c.mult(0, V1, S2, V0);
            c.g[A0][0] = 0xffff_ffff_ffff_ffff;
            let v = c.g[S1][0];
            oc767_sw(c, S0, PLAT_TRACK_REQUESTS, v);
            oc767_addu(c, S1, V1, S1);
            oc767_subu(c, V1, S1, S0);
            oc767_addiu(c, A1, S1, 0x10);
            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            oc767_addiu(c, V0, V1, 0xf);
            if c.g[A0][0] != 0 {
                c.g[V0][0] = c.g[V1][0];
            }
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
            oc767_sll(c, V0, V0, 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[S0][0] as u32));
            let empty = (c.g[S2][0] as i64) <= 0;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            if !empty {
                // Every request's handle (+4) is -1.
                c.cycles += 4;
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                c.g[A0][0] = 0;
                c.g[V1][0] = c.g[S2][0];
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = oc767_lw(c, S0, PLAT_TRACK_REQUESTS);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                    oc767_addu(c, V0, A0, V0);
                    let v = c.g[A1][0];
                    oc767_sw(c, V0, 4, v);
                    let again = c.g[V1][0] != 0;
                    oc767_addiu(c, A0, A0, 0xc);
                    if !again {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002a_e588);
                    }
                }
            }
            // The play options: the caller's, or the defaults.
            c.cycles += 2;
            if c.g[S5][0] != 0 {
                c.cycles += 14;
                for (k, r) in [4usize, 5, 2].into_iter().enumerate() {
                    let o = 8 * k as u32;
                    let a = oc767_ea(c, S5, o + 7);
                    c.ldl(r, a);
                    let a = oc767_ea(c, S5, o);
                    c.ldr(r, a);
                }
                for (k, r) in [4usize, 5, 2].into_iter().enumerate() {
                    let o = 8 * k as u32;
                    let a = oc767_ea(c, S0, 0x30 + o + 7);
                    c.sdl(r, a);
                    let a = oc767_ea(c, S0, 0x30 + o);
                    c.sdr(r, a);
                }
                c.g[A1][0] = 0x4a0000;
            } else {
                c.cycles += 3;
                oc767_addiu(c, A0, S0, 0x30);
                call_ptr(c, 0x003b_a020, 0x002a_e5ec)?; // SND_InitPlayParams
                c.g[A1][0] = 0x4a0000;
            }
            // Name, then the stream.
            c.cycles += 8;
            let a = oc767_ea(c, S0, 0x35);
            c.sb(a, 0); // fxlevel0 = 0
            oc767_addiu(c, A1, A1, 0x37a8);
            c.g[A3][0] = c.g[S7][0];
            oc767_addiu(c, A0, S0, 0x1c);
            c.g[A2][0] = 0xb;
            call_ptr(c, 0x0041_7828, 0x002a_e608)?; // sprintf(player->name, "%.*s", 11, name)
            let no_stream = (c.g[S6][0] as i64) <= 0;
            c.g[A2][0] = c.g[S6][0];
            if !no_stream {
                c.cycles += 7;
                c.g[A3][0] = c.g[S1][0];
                c.g[T0][0] = c.g[S3][0];
                oc767_addiu(c, A0, S0, 0x30);
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x003b_7818, 0x002a_e624)?; // SNDSTRM_create
                let v = c.g[V0][0];
                oc767_sw(c, S0, PLAT_TRACK_HANDLE, v);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                let v = c.g[V0][0];
                let a = oc767_ea(c, S0, PLAT_TRACK_IS_BANK_PLAYER);
                c.sb(a, v);
            }
            // A stream that failed (and is no bank player): return its handle.
            c.cycles += 3;
            c.g[V1][0] = oc767_lw(c, S0, PLAT_TRACK_HANDLE);
            c.g[V0][0] = 0x7f;
            let mut failed = false;
            if (c.g[V1][0] as i64) < 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(oc767_ea(c, S0, PLAT_TRACK_IS_BANK_PLAYER)).lb(c);
                let bank = c.g[V0][0] != 0;
                c.g[V0][0] = 0x7f;
                if !bank {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V1][0];
                    failed = true;
                }
            }
            if !failed {
                c.cycles += 4;
                let a = oc767_ea(c, S0, PLAT_TRACK_REQUEST_COUNT);
                let v = c.g[S2][0];
                c.sb(a, v);
                let v = c.g[V0][0];
                oc767_sw(c, S0, 0x14, v); // dry level 0x7f
                oc767_sw(c, S0, 0x10, 0); // fx level 0
                c.g[V0][0] = c.g[S4][0];
            }
        }
        c.cycles += 11;
        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore(c, 20, 0x40);
        frame.restore(c, 21, 0x30);
        frame.restore(c, 22, 0x20);
        frame.restore(c, 23, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `sw src, off(base)` with the source taken from a register.
#[inline(always)]
fn oc767_swr(c: &mut Cpu, base: usize, off: u32, src: usize) {
    let v = c.g[src][0];
    oc767_sw(c, base, off, v);
}

/// `rd = sx(a - b)` (`subu`).
#[inline(always)]
fn oc767_subu(c: &mut Cpu, rd: usize, a: usize, b: usize) {
    c.g[rd][0] = sx((c.g[a][0] as u32).wrapping_sub(c.g[b][0] as u32));
}

/// `SndLoadQueue_StartNext` (0x28a298).
const OC767_SND_LOAD_QUEUE_START_NEXT: Callee = Callee {
    addr: 0x0028_a298,
    f: ee::s07::f_0028a298,
};

/// Offsets in the async load system (`sAsync28A558`): the next free name
/// slot, four ring queues (`size`, `data`, `head`, `tail`) for bank ids,
/// slots, memory types and memory flags, and the 64-byte name slots.
pub(crate) const SND_LOAD_SLOT: u32 = 0x08;
pub(crate) const SND_LOAD_NAMES: u32 = 0x0c;
pub(crate) const SND_LOAD_Q_BANK: u32 = 0x18c;
pub(crate) const SND_LOAD_Q_SLOT: u32 = 0x19c;
pub(crate) const SND_LOAD_Q_TYPE: u32 = 0x1ac;
pub(crate) const SND_LOAD_Q_FLAGS: u32 = 0x1bc;

/// The ring-queue push of `SndLoadQueue_Push` once `hi` holds
/// `(head + 1) % size`: when that is not the tail (room left) store `value`
/// at `data[head]` and advance `head`, wrapping to 0 at `size`. `base` holds
/// the queue's address; the generated code loads the tail into `tail_reg`,
/// the remainder into `hi_reg`, and leaves `v0`/`v1` as scratch.
fn oc767_ring_push(c: &mut Cpu, base: usize, tail_reg: usize, hi_reg: usize, value: usize) {
    c.cycles += 4;
    c.g[tail_reg][0] = oc767_lw(c, base, 0xc);
    c.g[hi_reg][0] = c.hi[0];
    if c.g[hi_reg][0] == c.g[tail_reg][0] {
        return;
    }
    c.cycles += 11;
    c.g[V0][0] = oc767_lw(c, base, 8);
    c.g[V1][0] = oc767_lw(c, base, 4);
    oc767_sll(c, V0, V0, 2);
    oc767_addu(c, V0, V0, V1);
    let v = c.g[value][0];
    oc767_sw(c, V0, 0, v);
    c.g[V1][0] = oc767_lw(c, base, 8);
    c.g[V0][0] = oc767_lw(c, base, 0);
    oc767_addiu(c, V1, V1, 1);
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let more = c.g[V0][0] != 0;
    let v = c.g[V1][0];
    oc767_sw(c, base, 8, v);
    if !more {
        c.cycles += 1;
        oc767_sw(c, base, 8, 0);
    }
    c.cycles += 1;
}

/// `void SndLoadQueue_Push(AsyncSys* self, int bankId, char* name, int
/// memType, int memFlags)` (0x28a558, `cAsyncSys::ASYNCSYS_QueueLoad`): queues
/// a bank load (bank id, memory type, memory flags and the next slot number
/// in four ring queues that drop the entry when full), copies the name into
/// the current 64-byte name slot, advances the slot (0..5), and starts the next
/// load.
pub fn snd_load_queue_push(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        oc767_addiu(c, T1, S0, SND_LOAD_Q_BANK);
        c.g[V0][0] = oc767_lw(c, S0, SND_LOAD_Q_BANK + 8);
        c.g[V1][0] = oc767_lw(c, S0, SND_LOAD_Q_BANK);
        oc767_addiu(c, V0, V0, 1);
        c.div(0, V0, V1);

        // bank id
        oc767_ring_push(c, T1, V0, A0, A1);
        c.g[V1][0] = oc767_lw(c, S0, SND_LOAD_Q_TYPE + 8);

        // memory type
        c.cycles += 6;
        oc767_addiu(c, T1, S0, SND_LOAD_Q_TYPE);
        c.g[V0][0] = oc767_lw(c, S0, SND_LOAD_Q_TYPE);
        oc767_addiu(c, V1, V1, 1);
        c.div(0, V1, V0);
        oc767_ring_push(c, T1, V1, A0, A3);
        c.g[V1][0] = oc767_lw(c, S0, SND_LOAD_Q_FLAGS + 8);

        // memory flags
        c.cycles += 6;
        oc767_addiu(c, A3, S0, SND_LOAD_Q_FLAGS);
        c.g[V0][0] = oc767_lw(c, S0, SND_LOAD_Q_FLAGS);
        oc767_addiu(c, V1, V1, 1);
        c.div(0, V1, V0);
        oc767_ring_push(c, A3, V1, A0, T0);
        c.g[A0][0] = oc767_lw(c, S0, SND_LOAD_SLOT);

        // The name goes into names[slot] (64 bytes each).
        c.cycles += 16;
        c.g[A1][0] = c.g[A2][0];
        oc767_sll(c, A0, A0, 6);
        oc767_addiu(c, A0, A0, SND_LOAD_NAMES);
        oc767_addu(c, A0, S0, A0);
        call_ptr(c, 0x0041_66f4, 0x0028_a694)?; // strcpy
        c.g[V0][0] = oc767_lw(c, S0, SND_LOAD_Q_SLOT + 8);
        oc767_addiu(c, A2, S0, SND_LOAD_Q_SLOT);
        c.g[A0][0] = oc767_lw(c, S0, SND_LOAD_Q_SLOT);
        oc767_addiu(c, V0, V0, 1);
        c.g[V1][0] = oc767_lw(c, S0, SND_LOAD_SLOT);
        c.div(0, V0, A0);
        c.g[A3][0] = c.g[V1][0];
        oc767_addiu(c, V1, V1, 1);
        oc767_swr(c, S0, SND_LOAD_SLOT, V1);
        // slot number
        oc767_ring_push(c, A2, V0, A1, A3);
        c.g[V0][0] = oc767_lw(c, S0, SND_LOAD_SLOT);

        // Six name slots: wrap.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V0][0] as i64) < 6) as u64;
        if c.g[V0][0] == 0 {
            oc767_sw(c, S0, SND_LOAD_SLOT, 0);
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        OC767_SND_LOAD_QUEUE_START_NEXT.call(c, 0x0028_a718)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `PATH_Init` (0x3d2290).
const OC767_PATH_INIT: Callee = Callee {
    addr: 0x003d_2290,
    f: ee::s14::f_003d2290,
};

/// `PATH_FindMap` (0x3d4bb0).
const OC767_PATH_FIND_MAP: Callee = Callee {
    addr: 0x003d_4bb0,
    f: ee::s14::f_003d4bb0,
};

/// PATH globals: the initialised flag (byte), the map-table busy flag, the
/// current map (set by `PATH_FindMap`) and the four 0x928-byte map slots.
#[cfg(test)]
pub(crate) const PATH_INITIALISED: u32 = 0x0044_f42c;
#[cfg(test)]
pub(crate) const PATH_BUSY: u32 = 0x0044_f430;
#[cfg(test)]
pub(crate) const PATH_MAP_SLOTS: u32 = 0x0051_7610;
#[cfg(test)]
pub(crate) const PATH_MAP_SLOT_SIZE: u32 = 0x928;

/// `int PATH_AddMap(char* data)` (0x3d2350): registers a path map file
/// (magic "xDFP" = 0x50464478, version 4 at +4, kind byte at +0xc) in the
/// first free of four map slots. Returns -18 when busy, -1 when there is no
/// slot or the header is wrong, else the map's kind. A new slot is cleared,
/// gets the data pointer (+0x34) and a flag word (+0x30) from the number of
/// earlier maps of the same kind and the kind, then `PATH_FindMap` selects
/// it and the section pointers (+0x38..+0x54) are derived from the
/// file's offset tables, the 16 words below +0x118 cleared and the defaults
/// 50, 10, 1000, 1 stored at +0xc, +0x10, +0x14, +0x18.
pub fn path_add_map(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x450000;
        frame.save(c, 16, 0x80);
        frame.save(c, 18, 0x60);
        c.g[S0][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, 20, 0x40);
        c.g[S2][0] = 4;
        c.g[V1][0] = Addr(oc767_ea(c, V0, 0u32.wrapping_sub(0xbd4))).lbu(c);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, 17, 0x70);
        frame.save(c, 19, 0x50);
        frame.save(c, 21, 0x30);
        frame.save(c, 22, 0x20);
        frame.save(c, 23, 0x10);
        let initialised = c.g[V1][0] != 0;
        frame.save_ra(c, 0);
        if !initialised {
            c.cycles += 2;
            OC767_PATH_INIT.call(c, 0x003d_2398)?;
        }
        c.cycles += 4;
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = oc767_lw(c, V1, 0u32.wrapping_sub(0xbd0));
        let idle = c.g[V0][0] == 0;
        c.g[S7][0] = c.g[V1][0];
        if !idle {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffee;
        } else {
            c.cycles += 8;
            c.g[S1][0] = 0x510000;
            c.g[S5][0] = Addr(oc767_ea(c, S4, 0xc)).lbu(c);
            oc767_addiu(c, V0, S1, 0x7610);
            c.g[S3][0] = 0;
            oc767_addiu(c, A1, V0, 0x34);
            c.g[A0][0] = 0;
            c.g[S6][0] = 0x450000;
            // Find the first free slot (s2) and count the maps of this kind (s3).
            loop {
                c.cycles += 3;
                c.g[V0][0] = oc767_lw(c, A1, 0);
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(oc767_ea(c, V0, 0xc)).lbu(c);
                    c.cycles += 3;
                    oc767_addiu(c, V1, S3, 1);
                    c.g[V0][0] ^= c.g[S5][0];
                    if c.g[V0][0] == 0 {
                        c.g[S3][0] = c.g[V1][0];
                    }
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] ^ 0x4;
                    if c.g[V0][0] == 0 {
                        c.g[S2][0] = c.g[A0][0];
                    }
                }
                c.cycles += 4;
                oc767_addiu(c, A0, A0, 1);
                c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
                let again = c.g[V0][0] != 0;
                oc767_addiu(c, A1, A1, 0x928);
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_23d0);
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 4) as u64;
            let no_slot = c.g[V0][0] == 0;
            c.g[V0][0] = 0x5046_0000;
            let mut added = false;
            if no_slot {
                // falls out below
            } else {
                c.cycles += 4;
                c.g[V1][0] = oc767_lw(c, S4, 0);
                c.g[V0][0] |= 0x4478;
                if c.g[V1][0] != c.g[V0][0] {
                    oc767_sw(c, S7, 0u32.wrapping_sub(0xbd0), 0);
                    // (no +1 cycle on this early exit)
                    return oc767_path_add_map_tail(c, &frame);
                }
                c.cycles += 4;
                c.g[V1][0] = Addr(oc767_ea(c, S4, 4)).lbu(c);
                c.g[V0][0] = 4;
                if c.g[V1][0] != c.g[V0][0] {
                    oc767_sw(c, S7, 0u32.wrapping_sub(0xbd0), 0);
                    return oc767_path_add_map_tail(c, &frame);
                }
                added = true;
            }
            if added {
                c.cycles += 80;
                c.g[S0][0] = 0x928;
                oc767_addiu(c, S1, S1, 0x7610);
                c.mult(0, S0, S2, S0);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x928;
                oc767_addu(c, A0, S0, S1);
                call_ptr(c, 0x0041_6210, 0x003d_244c)?; // memset(slot, 0, 0x928)
                c.g[A0][0] = 0x1000_0000;
                c.g[V0][0] = 0x100_0000;
                oc767_addu(c, V1, S1, S0);
                c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[S3][0] as u32) & 31));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[S5][0] as u32) & 31));
                let v = c.g[S4][0];
                oc767_sw(c, V1, 0x34, v);
                c.g[A0][0] |= c.g[V0][0];
                c.g[S1][0] = c.g[V1][0];
                let v = c.g[A0][0];
                oc767_sw(c, S1, 0x30, v);
                OC767_PATH_FIND_MAP.call(c, 0x003d_2474)?;
                oc767_path_add_map_sections(c);
                c.cycles += 1;
                oc767_sw(c, S7, 0u32.wrapping_sub(0xbd0), 0);
            } else {
                c.cycles += 1;
                oc767_sw(c, S7, 0u32.wrapping_sub(0xbd0), 0);
            }
            return oc767_path_add_map_tail(c, &frame);
        }
        c.cycles += 11;
        oc767_path_add_map_restore(c, &frame);
        Ok(())
    });
}

/// The end of `PATH_AddMap` after the busy flag was cleared: also clears
/// the current-map pointer (+2 cycles), returns `s0` (the kind, or -1).
fn oc767_path_add_map_tail(c: &mut Cpu, frame: &Frame) -> crate::call::Flow {
    c.cycles += 2;
    c.g[V0][0] = c.g[S0][0];
    oc767_sw(c, S6, 0u32.wrapping_sub(0xbe0), 0);
    c.cycles += 11;
    oc767_path_add_map_restore(c, frame);
    Ok(())
}

/// The epilogue of `PATH_AddMap`: callee-saved registers, `ra`, `sp`.
fn oc767_path_add_map_restore(c: &mut Cpu, frame: &Frame) {
    for (k, r) in [16usize, 17, 18, 19, 20, 21, 22, 23]
        .into_iter()
        .enumerate()
    {
        frame.restore(c, r, 0x80 - 0x10 * k as u32);
    }
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// After `PATH_FindMap` selected the new map (the current-map pointer at 0x44f420): derive the
/// section pointers +0x38..+0x54 from the file's offset tables (each rounded
/// up to 4 bytes), clear the 16 words ending at +0x118 and store the defaults.
fn oc767_path_add_map_sections(c: &mut Cpu) {
    let cur = 0u32.wrapping_sub(0xbe0);
    c.g[A3][0] = oc767_lw(c, S6, cur);
    c.g[T0][0] = 0xffff_ffff_ffff_fffc;
    c.g[T2][0] = 0xf;
    c.g[A2][0] = oc767_lw(c, A3, 0x34);
    oc767_addiu(c, T1, A3, 0x118);
    oc767_addiu(c, A1, A2, 0x20);
    let v = c.g[A1][0];
    oc767_sw(c, A3, 0x38, v);
    c.g[V0][0] = Addr(oc767_ea(c, A2, 0x12)).lhu(c);
    oc767_sll(c, V0, V0, 1);
    oc767_addu(c, V0, A1, V0);
    let v = c.g[V0][0];
    oc767_sw(c, A3, 0x3c, v);
    c.g[V1][0] = Addr(oc767_ea(c, A2, 0x12)).lhu(c);
    oc767_sll(c, V1, V1, 1);
    oc767_addu(c, V1, V1, A1);
    c.g[A0][0] = Addr(oc767_ea(c, V1, 0u32.wrapping_sub(2))).lh(c);
    oc767_sll(c, A0, A0, 2);
    oc767_addu(c, A0, A2, A0);
    c.g[V0][0] = oc767_lw(c, A0, 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 10);
    c.g[V0][0] &= 0x7c;
    oc767_addu(c, A0, A0, V0);
    oc767_addiu(c, V1, A0, 0x13);
    c.g[A0][0] = c.g[V1][0] & c.g[T0][0];
    let v = c.g[A0][0];
    oc767_sw(c, A3, 0x40, v);
    c.g[V0][0] = Addr(oc767_ea(c, A2, 0xf)).lbu(c);
    oc767_sll(c, V0, V0, 1);
    oc767_addu(c, V0, A0, V0);
    oc767_addiu(c, V0, V0, 3);
    c.g[V0][0] &= c.g[T0][0];
    let v = c.g[V0][0];
    oc767_sw(c, A3, 0x44, v);
    c.g[V1][0] = Addr(oc767_ea(c, A2, 0xf)).lbu(c);
    oc767_sll(c, V1, V1, 1);
    oc767_addu(c, V1, V1, A0);
    c.g[A0][0] = Addr(oc767_ea(c, V1, 0u32.wrapping_sub(2))).lh(c);
    oc767_sll(c, A0, A0, 2);
    oc767_addu(c, V1, A2, A0);
    c.g[V0][0] = Addr(oc767_ea(c, V1, 0xe)).lhu(c);
    c.g[V0][0] &= 0x3f;
    oc767_sll(c, V0, V0, 4);
    oc767_addiu(c, V0, V0, 0x10);
    oc767_addu(c, V1, V1, V0);
    oc767_addiu(c, V1, V1, 3);
    c.g[V1][0] &= c.g[T0][0];
    let v = c.g[V1][0];
    oc767_sw(c, A3, 0x48, v);
    c.g[V0][0] = Addr(oc767_ea(c, A2, 0x10)).lbu(c);
    oc767_sll(c, V0, V0, 2);
    oc767_addu(c, V0, V0, V1);
    c.g[A0][0] = oc767_lw(c, V0, 0);
    oc767_sll(c, A0, A0, 2);
    oc767_addu(c, A0, A2, A0);
    let v = c.g[A0][0];
    oc767_sw(c, A3, 0x4c, v);
    c.g[V0][0] = Addr(oc767_ea(c, A2, 0xd)).lbu(c);
    oc767_sll(c, V0, V0, 2);
    oc767_addu(c, V0, A0, V0);
    oc767_addiu(c, V0, V0, 4);
    let v = c.g[V0][0];
    oc767_sw(c, A3, 0x50, v);
    c.g[V1][0] = Addr(oc767_ea(c, A2, 0xd)).lbu(c);
    oc767_sll(c, V1, V1, 2);
    oc767_addu(c, V1, V1, A0);
    c.g[V0][0] = oc767_lw(c, V1, 0);
    oc767_sll(c, V0, V0, 2);
    oc767_addu(c, A2, A2, V0);
    let v = c.g[A2][0];
    oc767_sw(c, A3, 0x54, v);
    loop {
        c.cycles += 7;
        oc767_sw(c, T1, 0, 0);
        oc767_addiu(c, T2, T2, 0u32.wrapping_sub(1));
        let more = (c.g[T2][0] as i64) >= 0;
        oc767_addiu(c, T1, T1, 0u32.wrapping_sub(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003d_2570);
            }
            continue;
        }
        c.cycles += 10;
        c.g[V0][0] = oc767_lw(c, S6, cur);
        c.g[A0][0] = 0x32;
        c.g[A1][0] = 0xa;
        c.g[A2][0] = 0x3e8;
        c.g[V1][0] = 1;
        let v = c.g[A0][0];
        oc767_sw(c, V0, 0xc, v);
        let v = c.g[V1][0];
        oc767_sw(c, V0, 0x18, v);
        c.g[S0][0] = c.g[S5][0];
        let v = c.g[A1][0];
        oc767_sw(c, V0, 0x10, v);
        let v = c.g[A2][0];
        oc767_sw(c, V0, 0x14, v);
        break;
    }
}

/// `MusicPlayer_StartSong` (0x2b3838).
const OC767_MUSIC_PLAYER_START_SONG: Callee = Callee {
    addr: 0x002b_3838,
    f: ee::s08::f_002b3838,
};

/// `void MusicPlayer_PlaySong(MusicPlayer* self, char* song, int bankIndex,
/// float* volumeControl, int syncLoad)` (0x2b35a0, `func_002B35A0`): pushes a
/// new instance of the bank monitor's stack (`BankMonitor.mInst` at +0x1f4 of
/// the bank manager `**self`), initialises it from the defaults (the 24-byte
/// play options at +0x1dc, zero position, base volume from the options, zero
/// pointers, delay 0/100/90/50, volume mapping 0x7f, close on end), stores
/// `volumeControl` in the instance's slot, calls `MusicPlayer_StartSong`
/// and pops the instance.
pub fn music_player_play_song(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 165;
        let frame = Frame::open(c, 0x80);
        c.g[T1][0] = 0x18;
        frame.save(c, 17, 0x60);
        frame.save(c, 18, 0x50);
        frame.save(c, 19, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 20, 0x30);
        c.g[S3][0] = c.g[T0][0];
        frame.save(c, 21, 0x20);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, 22, 0x10);
        c.g[S5][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S6][0] = c.g[A3][0];
        frame.save(c, 16, 0x70);
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        c.g[V0][0] = oc767_lw(c, S2, 0);
        c.g[S0][0] = oc767_lw(c, V0, 0); // the bank manager
        c.g[V0][0] = oc767_lw(c, S0, 0x1f4);
        oc767_addiu(c, S1, S0, 0x1d8); // its BankMonitor
        oc767_addiu(c, V0, V0, 1);
        oc767_swr(c, S0, 0x1f4, V0);
        c.g[V1][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V0][0] = oc767_lw(c, S1, 0x20);
        c.mult(0, V1, V1, T1);
        oc767_addu(c, V1, V1, V0);
        for (k, r) in [2usize, 4, 7].into_iter().enumerate() {
            let o = 8 * k as u32;
            let a = oc767_ea(c, S0, 0x1e3 + o);
            c.ldl(r, a);
            let a = oc767_ea(c, S0, 0x1dc + o);
            c.ldr(r, a);
        }
        for (k, r) in [2usize, 4, 7].into_iter().enumerate() {
            let o = 8 * k as u32;
            let a = oc767_ea(c, V1, 7 + o);
            c.sdl(r, a);
            let a = oc767_ea(c, V1, o);
            c.sdr(r, a);
        }
        // mPosition[inst] = 0
        c.g[V0][0] = oc767_lw(c, S0, 0x1f4);
        oc767_sll(c, V0, V0, 2);
        oc767_addu(c, V0, S0, V0);
        oc767_sw(c, V0, 0x1fc, 0);
        c.g[V1][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V0][0] = oc767_lw(c, S1, 0x34);
        oc767_sll(c, V1, V1, 2);
        oc767_addu(c, V1, V1, V0);
        oc767_sw(c, V1, 0, 0);
        // mBaseVol[inst] = mCurrent[inst].vol
        c.g[A3][0] = oc767_lw(c, S0, 0x1f4);
        c.g[A0][0] = oc767_lw(c, S0, 0x1f4);
        c.g[T0][0] = oc767_lw(c, S1, 0x20);
        oc767_sll(c, A3, A3, 2);
        c.mult(0, A0, A0, T1);
        c.g[V0][0] = oc767_lw(c, S1, 0x38);
        oc767_addu(c, A3, A3, V0);
        oc767_addu(c, A0, A0, T0);
        c.g[V1][0] = Addr(oc767_ea(c, A0, 0)).lb(c);
        oc767_swr(c, A3, 0, V1);
        // mVolControl[inst] = 0
        c.g[V0][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V1][0] = oc767_lw(c, S1, 0x3c);
        oc767_sll(c, V0, V0, 2);
        oc767_addu(c, V0, V0, V1);
        oc767_sw(c, V0, 0, 0);
        // memset(&mem[inst * 8 + 0x40], 0, 8)
        c.g[A0][0] = oc767_lw(c, S0, 0x1f4);
        oc767_sll(c, A0, A0, 3);
        oc767_addu(c, A0, S1, A0);
        oc767_addiu(c, A0, A0, 0x40);
        call_ptr(c, 0x0041_6210, 0x002b_36b0)?;
        // Per-instance defaults.
        c.g[V1][0] = oc767_lw(c, S0, 0x1f4);
        c.g[T0][0] = 0x64;
        c.g[V0][0] = oc767_lw(c, S1, 0x60);
        c.g[T1][0] = 0x5a;
        oc767_sll(c, V1, V1, 2);
        c.g[T2][0] = 0x32;
        oc767_addu(c, V1, V1, V0);
        c.g[T3][0] = 0x7f;
        oc767_sw(c, V1, 0, 0);
        c.g[T4][0] = 1;
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[V1][0] = oc767_lw(c, S0, 0x1f4);
        c.g[A3][0] = c.g[S3][0];
        c.g[V0][0] = oc767_lw(c, S1, 0x64);
        c.g[A0][0] = c.g[S2][0];
        oc767_sll(c, V1, V1, 2);
        oc767_addu(c, V1, V1, V0);
        oc767_sw(c, V1, 0, 0);
        c.g[V0][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V1][0] = oc767_lw(c, S1, 0x68);
        oc767_sll(c, V0, V0, 2);
        oc767_addu(c, V0, V0, V1);
        oc767_sw(c, V0, 0, 0);
        c.g[V1][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V0][0] = oc767_lw(c, S1, 0x6c);
        oc767_sll(c, V1, V1, 2);
        oc767_addu(c, V1, V1, V0);
        oc767_sw(c, V1, 0, 0);
        // mDelay[inst] = { 0, 100, 90, 50 }
        c.g[V0][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V1][0] = oc767_lw(c, S1, 0x70);
        oc767_sll(c, V0, V0, 4);
        oc767_addu(c, V0, V0, V1);
        oc767_sw(c, V0, 4, 0);
        c.g[V1][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V0][0] = oc767_lw(c, S1, 0x70);
        oc767_sll(c, V1, V1, 4);
        oc767_addu(c, V1, V1, V0);
        oc767_swr(c, V1, 0, T0);
        c.g[V0][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V1][0] = oc767_lw(c, S1, 0x70);
        oc767_sll(c, V0, V0, 4);
        oc767_addu(c, V0, V0, V1);
        oc767_swr(c, V0, 8, T1);
        c.g[V1][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V0][0] = oc767_lw(c, S1, 0x70);
        oc767_sll(c, V1, V1, 4);
        oc767_addu(c, V1, V1, V0);
        oc767_swr(c, V1, 0xc, T2);
        c.g[V0][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V1][0] = oc767_lw(c, S1, 0x74);
        oc767_sll(c, V0, V0, 2);
        oc767_addu(c, V0, V0, V1);
        oc767_sw(c, V0, 0, 0);
        c.g[V1][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V0][0] = oc767_lw(c, S1, 0x80);
        oc767_sll(c, V1, V1, 2);
        oc767_addu(c, V1, V1, V0);
        oc767_sw(c, V1, 0, 0);
        c.g[V0][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V1][0] = oc767_lw(c, S1, 0x84);
        oc767_sll(c, V0, V0, 2);
        oc767_addu(c, V0, V0, V1);
        oc767_sw(c, V0, 0, 0);
        c.g[V1][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V0][0] = oc767_lw(c, S1, 0x78);
        oc767_sll(c, V1, V1, 2);
        oc767_addu(c, V1, V1, V0);
        oc767_swr(c, V1, 0, T3);
        c.g[V0][0] = oc767_lw(c, S0, 0x1f4);
        c.g[V1][0] = oc767_lw(c, S1, 0x7c);
        oc767_sll(c, V0, V0, 2);
        oc767_addu(c, V0, V0, V1);
        oc767_swr(c, V0, 0, T4);
        // mVolControl[inst] = volumeControl, via the (re-read) bank manager.
        c.g[V1][0] = oc767_lw(c, S2, 0);
        c.g[T0][0] = oc767_lw(c, V1, 0);
        c.g[V0][0] = oc767_lw(c, T0, 0x1f4);
        c.g[V1][0] = oc767_lw(c, T0, 0x214);
        oc767_sll(c, V0, V0, 2);
        oc767_addu(c, V0, V0, V1);
        oc767_swr(c, V0, 0, S6);
        OC767_MUSIC_PLAYER_START_SONG.call(c, 0x002b_37f8)?;
        // Pop the instance.
        c.g[V1][0] = oc767_lw(c, S2, 0);
        frame.restore(c, 16, 0x70);
        c.g[A0][0] = oc767_lw(c, V1, 0);
        frame.restore(c, 17, 0x60);
        c.g[V0][0] = oc767_lw(c, A0, 0x1f4);
        frame.restore(c, 18, 0x50);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore(c, 21, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        oc767_swr(c, A0, 0x1f4, V0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, fill, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Control word of this module's gated fakes (they only act when the
    /// machine's RAM holds the magic here).
    pub(crate) const OC767_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC767_MAGIC: u32 = 0x4f43_3767;

    /// Per-callee forced results: one word per fake.
    const OC767_FORCE: u32 = DATA + 0x30_0000;

    fn oc767_mail(addr: u32) -> u32 {
        OC767_FORCE + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (sign-extended; stored +3).
    pub(crate) fn oc767_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc767_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// the forced result of its mailbox (0 = the fake's own pointer) unless
    /// it unwound.
    fn oc767_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc767_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = sx(mail.wrapping_sub(3));
        }
    }

    macro_rules! oc767_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC767_CTRL) != OC767_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc767_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc767_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc767_fakes!(
        (
            oc767_fake_14dc80,
            OC767_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc767_fake_14dd58,
            OC767_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc767_fake_413af8,
            OC767_PREV_413AF8,
            ee::s15::OVERRIDE_00413AF8,
            ee::s15::f_00413af8__generated,
            0x0041_3af8
        ),
        (
            oc767_fake_417828,
            OC767_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc767_fake_2ae1b8,
            OC767_PREV_2AE1B8,
            ee::s08::OVERRIDE_002AE1B8,
            ee::s08::f_002ae1b8__generated,
            0x002a_e1b8
        ),
        (
            oc767_fake_2ae138,
            OC767_PREV_2AE138,
            ee::s08::OVERRIDE_002AE138,
            ee::s08::f_002ae138__generated,
            0x002a_e138
        ),
        (
            oc767_fake_3ba020,
            OC767_PREV_3BA020,
            ee::s13::OVERRIDE_003BA020,
            ee::s13::f_003ba020__generated,
            0x003b_a020
        ),
        (
            oc767_fake_4166f4,
            OC767_PREV_4166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4
        ),
        (
            oc767_fake_28a298,
            OC767_PREV_28A298,
            ee::s07::OVERRIDE_0028A298,
            ee::s07::f_0028a298__generated,
            0x0028_a298
        ),
        (
            oc767_fake_416210,
            OC767_PREV_416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
        (
            oc767_fake_2b3ac0,
            OC767_PREV_2B3AC0,
            ee::s08::OVERRIDE_002B3AC0,
            ee::s08::f_002b3ac0__generated,
            0x002b_3ac0
        ),
    );

    /// `FE_FormatMinSec`: the option block at 0x535610 holds mode `seed % 8`
    /// in bits 22..24 of its first word (0/2: format A, 1/3: format B, 4-7:
    /// no text); `secs` is negative, zero, below and above a minute; the
    /// four callees (`cBE_getBE`, `getInterface`, `fptodp` twice, `sprintf`)
    /// are fakes and some of them unwind.
    #[test]
    fn fe_format_min_sec_matches() {
        oc767_install_fakes();
        covered(
            ee::s05::f_0020a1a8__generated,
            fe_format_min_sec,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0020_a1a8, 5);
                c.bus.write32(OC767_CTRL, OC767_MAGIC);
                fill(c, &mut r, FE_FORMAT_MIN_SEC_OPTIONS, 0x288 / 4);
                let first = (r.next_u32() & !(7 << 22)) | ((seed % 8) << 22);
                c.bus.write32(FE_FORMAT_MIN_SEC_OPTIONS, first);
                let secs = match r.next_u32() % 5 {
                    0 => 0u32,
                    1 => r.next_u32() % 60,
                    2 => r.next_u32() % 6000,
                    3 => 0u32.wrapping_sub(r.next_u32() % 500),
                    _ => r.next_u32(),
                };
                c.g[A0][0] = sx(secs);
                c.pc = 0x0020_a1a8;
            },
        );
    }

    /// Fixture objects of `PATH_PlatAddTrack`.
    const OC767_TRACK_NAME: u32 = DATA + 0x200;
    const OC767_TRACK_DESC: u32 = DATA + 0x600;
    const OC767_TRACK_MEM: u32 = DATA + 0x4000;
    /// `SNDSTRM_Open`'s stream count byte and slot table: all slots taken, so
    /// the real `SNDSTRM_create` fails at once with -9.
    const OC767_STREAM_COUNT: u32 = 0x0050_a92e;
    const OC767_STREAM_SLOTS: u32 = 0x0050_a0a0;

    /// `PATH_PlatAddTrack`: free slot 0-3 or none (-13); explicit or default
    /// play options at an unaligned address; request counts -1..5; no stream
    /// (bank player) or a stream whose creation fails (the real
    /// `SNDSTRM_create` returns -9 with every stream slot taken), the player's
    /// bank-player byte set or clear. The successful stream tail needs the
    /// real `SNDSTRM_Open` to succeed and is not reached here.
    #[test]
    fn path_plat_add_track_matches() {
        oc767_install_fakes();
        covered(
            ee::s08::f_002ae4c0__generated,
            path_plat_add_track,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_e4c0, 5);
                c.bus.write32(OC767_CTRL, OC767_MAGIC);
                let slot = if seed % 5 == 0 {
                    0xffff_ffffu32
                } else {
                    (seed / 5) % 4
                };
                oc767_force(c, 0x002a_e1b8, slot);
                c.bus.write8(OC767_STREAM_COUNT, 2);
                c.bus.write32(OC767_STREAM_SLOTS, 1);
                c.bus.write32(OC767_STREAM_SLOTS + 4, 1);
                fill(c, &mut r, OC767_TRACK_DESC, 0x20);
                fill(c, &mut r, OC767_TRACK_MEM, 0x40);
                let mem = OC767_TRACK_MEM + r.next_u32() % 16;
                // The stored handle and bank-player byte of the track.
                if seed % 2 == 0 {
                    c.bus.write8(mem + PLAT_TRACK_IS_BANK_PLAYER, 0);
                }
                c.g[A0][0] = sx(OC767_TRACK_NAME);
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    sx(OC767_TRACK_DESC + r.next_u32() % 8)
                };
                let count = (r.next_u32() % 7) as i32 - 1;
                c.g[A2][0] = sx(count as u32);
                let chunks = [0xffff_ffffu32, 0, 1, 3][((seed / 7) % 4) as usize];
                c.g[A3][0] = sx(chunks);
                c.g[T0][0] = sx(mem);
                c.g[T1][0] = sx(r.next_u32() % 0x10000);
                c.pc = 0x002a_e4c0;
            },
        );
    }

    const OC767_LOAD_OBJ: u32 = DATA + 0x1000;
    const OC767_LOAD_RINGS: u32 = DATA + 0x3000;

    /// `SndLoadQueue_Push`: four ring queues of size 1-5 with random head
    /// and tail (full queues drop the entry, size-1 queues are always full),
    /// the name slot 0-7 (wraps at 6); the `strcpy` and `StartNext` callees
    /// are fakes, some of which unwind.
    #[test]
    fn snd_load_queue_push_matches() {
        oc767_install_fakes();
        covered(
            ee::s07::f_0028a558__generated,
            snd_load_queue_push,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_a558, 2);
                c.bus.write32(OC767_CTRL, OC767_MAGIC);
                fill(c, &mut r, OC767_LOAD_OBJ, 0x1e0 / 4);
                fill(c, &mut r, OC767_LOAD_RINGS, 0x100 / 4);
                for (k, q) in [
                    SND_LOAD_Q_BANK,
                    SND_LOAD_Q_SLOT,
                    SND_LOAD_Q_TYPE,
                    SND_LOAD_Q_FLAGS,
                ]
                .into_iter()
                .enumerate()
                {
                    let size = 1 + r.next_u32() % 5;
                    c.bus.write32(OC767_LOAD_OBJ + q, size);
                    c.bus
                        .write32(OC767_LOAD_OBJ + q + 4, OC767_LOAD_RINGS + 0x40 * k as u32);
                    c.bus.write32(OC767_LOAD_OBJ + q + 8, r.next_u32() % size);
                    c.bus.write32(OC767_LOAD_OBJ + q + 0xc, r.next_u32() % size);
                }
                c.bus
                    .write32(OC767_LOAD_OBJ + SND_LOAD_SLOT, r.next_u32() % 8);
                c.g[A0][0] = sx(OC767_LOAD_OBJ);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(DATA + 0x800);
                c.g[A3][0] = sx(r.next_u32());
                c.g[T0][0] = sx(r.next_u32());
                c.pc = 0x0028_a558;
            },
        );
    }

    const OC767_MAP_BLOCKS: u32 = DATA + 0x1000;

    /// `PATH_AddMap`: initialised or not (the real `PATH_Init` runs), busy
    /// or not, between none and four earlier maps of random kinds, a header
    /// that is right or has a bad magic or version; the success path runs the
    /// real `PATH_FindMap` (`memset` is a fake) and the section tables are
    /// derived from small random offset tables.
    #[test]
    fn path_add_map_matches() {
        oc767_install_fakes();
        check(
            ee::s14::f_003d2350__generated,
            path_add_map,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003d_2350, 1);
                c.bus.write32(OC767_CTRL, OC767_MAGIC);
                // Four earlier map blocks and the new one, filled with small words.
                for k in 0..5u32 {
                    let base = OC767_MAP_BLOCKS + 0x400 * k;
                    for w in 0..0x100u32 {
                        c.bus.write32(base + 4 * w, r.next_u32() % 64);
                    }
                    c.bus.write32(base + 0xc, r.next_u32() % 3);
                }
                for i in 0..4u32 {
                    let slot = PATH_MAP_SLOTS + i * PATH_MAP_SLOT_SIZE;
                    if r.next_u32() % 3 != 0 {
                        c.bus.write32(slot + 0x34, OC767_MAP_BLOCKS + 0x400 * i);
                        c.bus.write32(slot + 0x30, r.next_u32());
                    }
                }
                let new = OC767_MAP_BLOCKS + 0x400 * 4;
                let magic = if seed % 9 == 0 {
                    r.next_u32()
                } else {
                    0x5046_4478
                };
                c.bus.write32(new, magic);
                let version = if seed % 11 == 0 { r.next_u32() % 8 } else { 4 };
                c.bus.write32(new + 4, version);
                c.bus.write8(PATH_INITIALISED, (seed % 7 != 0) as u8);
                c.bus.write32(PATH_BUSY, (seed % 6 == 0) as u32);
                c.g[A0][0] = sx(new);
                c.pc = 0x003d_2350;
            },
        );
    }

    const OC767_SONG_SELF: u32 = DATA + 0x1000;
    const OC767_SONG_PTR: u32 = DATA + 0x1100;
    const OC767_SONG_BANK: u32 = DATA + 0x2000;
    const OC767_SONG_TABLES: u32 = DATA + 0x8000;

    /// `MusicPlayer_PlaySong`: the bank manager's per-instance arrays (all
    /// the BankMonitor pointers into 0x400-byte tables, instance 0-3) are
    /// initialised for the new instance; the `memset` and the real
    /// `StartSong`'s first callee, `StopSong`, are fakes, and the fake after
    /// `memset` always unwinds (a seed in three also unwinds the `memset`).
    /// The real `StartSong` chain below it (instance creation and loading)
    /// is not bounded on test data, so the pop of the instance after
    /// `StartSong` returns is not exercised.
    #[test]
    fn music_player_play_song_matches() {
        oc767_install_fakes();
        covered(
            ee::s08::f_002b35a0__generated,
            music_player_play_song,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_35a0, 2);
                c.bus.write32(OC767_CTRL, OC767_MAGIC);
                c.bus
                    .write32(fakes::LOG_UNWIND_AT, if seed % 3 == 0 { 0 } else { 1 });
                fill(c, &mut r, OC767_SONG_BANK + 0x1d8, 0x60 / 4);
                fill(c, &mut r, OC767_SONG_TABLES, 0x2000);
                c.bus.write32(OC767_SONG_SELF, OC767_SONG_PTR);
                c.bus.write32(OC767_SONG_PTR, OC767_SONG_BANK);
                c.bus.write32(OC767_SONG_BANK + 0x1f4, r.next_u32() % 4);
                for (k, off) in [
                    0x20u32, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c,
                    0x80, 0x84,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(
                        OC767_SONG_BANK + 0x1d8 + off,
                        OC767_SONG_TABLES + 0x400 * k as u32,
                    );
                }
                c.g[A0][0] = sx(OC767_SONG_SELF);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32());
                c.g[T0][0] = sx(r.next_u32());
                c.pc = 0x002b_35a0;
            },
        );
    }
}
