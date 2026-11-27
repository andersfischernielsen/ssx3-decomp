//! Staging module oc1154 (wave wave273): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1154`.

#![allow(unused_imports)]

use crate::abi::{r32, ret, slt};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_5fd0, snd_set_config_cc, "SND_SetConfigCC"),
    (
        0x003c_6040,
        snd_remove_service_callback,
        "SND_RemoveServiceCallback",
    ),
    (0x003c_6240, snd_mem_get_free_percent, "SND_MemGetFreePercent"),
    (0x003c_65e0, snd_strm_calc_ring_size, "SNDSTRM_CalcRingSize"),
    (
        0x003c_65f0,
        snd_strm_calc_slot_mem_size,
        "SNDSTRM_CalcSlotMemSize",
    ),
    (0x003c_6620, snd_strm_alloc_slot, "SNDSTRM_AllocSlot"),
    (0x003c_6da0, snd_strm_free_slot, "SNDSTRM_FreeSlot"),
    (0x003c_7138, snd_channel_key_off, "SND_ChannelKeyOff"),
];

/// `SND_SetConfigCC(a0, a1)` (0x3c5fd0): reads the sound config block
/// (`SND_GetConfig`) onto the stack, stores `a0` at +0xcc and `a1` at +0xd0,
/// and writes the block back (`SND_SetConfig`). Returns 0.
pub fn snd_set_config_cc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0x100);
        frame.save(c, S1, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0xe0);
        c.g[A0][0] = c.g[SP][0];
        Callee {
            addr: 0x003b_4dd8,
            f: ee::s13::f_003b4dd8,
        }
        .call(c, 0x003c_5ff0)?;
        let s0 = c.g[S0][0];
        Addr(frame.at(c, 0xcc)).sw(c, s0);
        c.g[A0][0] = c.g[SP][0];
        let s1 = c.g[S1][0];
        Addr(frame.at(c, 0xd0)).sw(c, s1);
        Callee {
            addr: 0x003b_4fc0,
            f: ee::s13::f_003b4fc0,
        }
        .call(c, 0x003c_6000)?;
        frame.restore(c, S0, 0x100);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0xf0);
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    });
}

/// `SND_RemoveServiceCallback(cb)` (0x3c6040; `iSNDserverremoveclient`): removes
/// `cb` from the service-callback list at `0x50a8e8 + 0x19c` (4-byte entries;
/// the count is the byte at `+0x178`). The entries after it move down one and
/// the count drops by one. Makes no call.
pub fn snd_remove_service_callback(c: &mut Cpu) {
    c.cycles += 5;
    let mut a0 = c.g[A0][0];
    let mut a1: u64 = 0;
    let mut a2: u64 = 0x0051_0000;
    let mut a3 = c.g[A3][0];
    let mut t0 = c.g[T0][0];
    let mut t1 = c.g[T1][0];
    let mut v1: u64 = sx((a2 as u32).wrapping_sub(0x5718));
    let mut v0: u64 = Addr((v1 as u32).wrapping_add(0x178)).lb(c);
    'done: {
        if (v0 as i64) <= 0 {
            break 'done;
        }
        c.cycles += 5;
        a3 = v1;
        t1 = a2;
        t0 = sx((a3 as u32).wrapping_add(0x19c));
        a2 = sx((a1 as u32) << 2);
        'scan: loop {
            c.cycles += 4;
            v0 = sx((a2 as u32).wrapping_add(t0 as u32));
            v1 = Addr(v0 as u32).lw(c);
            if v1 != a0 {
                v0 = Addr((a3 as u32).wrapping_add(0x178)).lb(c);
                c.cycles += 4;
                a1 = sx((a1 as u32).wrapping_add(1));
                v0 = slt(a1, v0);
                let t = v0 != 0;
                a2 = sx((a1 as u32) << 2);
                if !t {
                    snd_remove_cb_exit(c, [a0, a1, a2, a3, t0, t1, v0, v1]);
                    return;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_6068);
                }
                continue 'scan;
            }
            // The callback is at index a1: count down, then shift the rest.
            c.cycles += 7;
            v0 = Addr((a3 as u32).wrapping_add(0x178)).lbu(c);
            v0 = sx((v0 as u32).wrapping_sub(1));
            v1 = sx((v0 as u32) << 24);
            v1 = sx(((v1 as u32) as i32 >> 24) as u32);
            v1 = slt(a1, v1);
            let t = v1 == 0;
            Addr((a3 as u32).wrapping_add(0x178)).sb(c, v0);
            if t {
                break 'done;
            }
            c.cycles += 3;
            a0 = sx((t1 as u32).wrapping_sub(0x5718));
            v0 = sx((a2 as u32).wrapping_add(0x19c));
            v1 = sx((v0 as u32).wrapping_add(a0 as u32));
            loop {
                c.cycles += 7;
                v0 = Addr((v1 as u32).wrapping_add(4)).lw(c);
                a1 = sx((a1 as u32).wrapping_add(1));
                Addr(v1 as u32).sw(c, v0);
                v0 = Addr((a0 as u32).wrapping_add(0x178)).lb(c);
                v0 = slt(a1, v0);
                let t = v0 != 0;
                v1 = sx((v1 as u32).wrapping_add(4));
                if !t {
                    break 'done;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_60a0);
                }
            }
        }
    }
    snd_remove_cb_exit(c, [a0, a1, a2, a3, t0, t1, v0, v1]);
}

/// The epilogue of `SND_RemoveServiceCallback`: the scratch registers the
/// generated body leaves (a0..a3, t0, t1, v0, v1 in that order), the 2-cycle
/// exit charge and the return.
fn snd_remove_cb_exit(c: &mut Cpu, r: [u64; 8]) {
    c.g[A0][0] = r[0];
    c.g[A1][0] = r[1];
    c.g[A2][0] = r[2];
    c.g[A3][0] = r[3];
    c.g[T0][0] = r[4];
    c.g[T1][0] = r[5];
    c.g[V0][0] = r[6];
    c.g[V1][0] = r[7];
    c.cycles += 2;
    ret(c);
}

/// `SND_MemGetFreePercent()` (0x3c6240): `(total - used) * 100 / total` for the
/// sound heap record at `*(0x50ab1c)`, with `total` at +8 and `used` at +0x10.
pub fn snd_mem_get_free_percent(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x0051_0000;
    c.g[A1][0] = 0x64;
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0x54e4)).lw(c);
    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lw(c);
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c);
    c.cycles += 6;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.mult(0, 2, 2, 5);
    c.div(0, 2, 3);
    c.g[V0][0] = c.lo[0];
    ret(c);
}

/// `SNDSTRM_CalcRingSize(a0)` (0x3c65e0): `a0 * 32 + 0x9c`.
pub fn snd_strm_calc_ring_size(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[A0][0] as u32) << 5);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x9c));
    ret(c);
}

/// `SNDSTRM_CalcSlotMemSize(a0)` (0x3c65f0): the ring size of the slot plus the
/// platform's packet overhead (`SNDPLATFORM_packetoverhead`).
pub fn snd_strm_calc_slot_mem_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x003c_65e0,
            f: ee::s13::f_003c65e0,
        }
        .call(c, 0x003c_6604)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x003c_e198, 0x003c_660c)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `SNDSTRM_AllocSlot(a0, a1, a2, a3, t0)` (0x3c6620): takes a free stream slot
/// from the table at `0x515abc` (the first `flag` entries, the flag byte at
/// `0x50a92e`), creates its platform packet (`SNDPLATFORM_packetplaycreate`)
/// and fills the slot record at `a3`. Returns the slot index; -6 when the
/// create failed, -9 when no slot is free. The sound lock is released on
/// every path that took it.
pub fn snd_strm_alloc_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, S2, 0x70);
        frame.save(c, S0, 0x90);
        frame.save(c, S3, 0x60);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A3][0];
        c.g[S4][0] = c.g[T0][0];
        c.g[S5][0] = c.g[A0][0];
        c.g[S6][0] = c.g[A1][0];
        c.g[S7][0] = c.g[A2][0];
        c.g[S2][0] = 0;
        Callee {
            addr: 0x003b_58a0,
            f: ee::s13::f_003b58a0,
        }
        .call(c, 0x003c_6668)?;
        c.g[V0][0] = 0x0051_0000;
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0x56d2)).lbu(c);
        let no_flag = c.g[V1][0] == 0;
        c.g[V0][0] = 0x0051_0000;
        'out: {
            'b_66c0: {
                'b_66b0: {
                    if no_flag {
                        break 'b_66b0;
                    }
                    c.cycles += 6;
                    c.g[A1][0] = c.g[V1][0];
                    c.g[FP][0] = c.g[V0][0];
                    c.g[S3][0] = 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x57b8));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x304));
                    // Scan the table for a free entry (zero) within the count.
                    'scan: loop {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                        let t = c.g[V0][0] == 0;
                        c.g[V1][0] = c.g[S3][0];
                        if t {
                            break 'b_66c0;
                        }
                        c.cycles += 5;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                        c.g[V0][0] = slt(c.g[S2][0], c.g[A1][0]);
                        let t = c.g[V0][0] != 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                        if !t {
                            break 'b_66b0;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_6690);
                        }
                    }
                }
                // No free slot (or no flag): release the lock, return -9.
                c.cycles += 4;
                Callee {
                    addr: 0x003b_58d8,
                    f: ee::s13::f_003b58d8,
                }
                .call(c, 0x003c_66b8)?;
                c.g[V0][0] = 0xffff_ffff_ffff_fff7;
                break 'out;
            }
            // A free entry at index s2 (the 'b_3c6724 block).
            'b_6724: {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003c_e1a0, 0x003c_66cc)?;
                if (c.g[V0][0] as i64) < 0 {
                    break 'b_6724;
                }
                c.cycles += 20;
                c.g[S0][0] = c.g[S1][0];
                call_ptr(c, 0x003c_e198, 0x003c_66dc)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x003c_e198, 0x003c_66e4)?;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A0][0] = 0xffff_ffff_ffff_ffff;
                c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x9c));
                c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x57b8));
                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                let rec = c.g[S1][0] as u32;
                let v = c.g[V0][0];
                Addr(rec.wrapping_add(0x40)).sh(c, v);
                let s0 = c.g[S0][0];
                Addr(rec.wrapping_add(0x50)).sw(c, s0);
                let s5 = c.g[S5][0];
                Addr(rec.wrapping_add(0x54)).sw(c, s5);
                let s6 = c.g[S6][0];
                Addr(rec.wrapping_add(0x58)).sw(c, s6);
                let s7 = c.g[S7][0];
                Addr(rec.wrapping_add(0x5c)).sw(c, s7);
                let a0 = c.g[A0][0];
                Addr(rec).sw(c, a0);
                let table = (c.g[V1][0] as u32).wrapping_add(0x304);
                let slot = c.g[S1][0];
                Addr(table).sw(c, slot);
                Callee {
                    addr: 0x003b_58d8,
                    f: ee::s13::f_003b58d8,
                }
                .call(c, 0x003c_671c)?;
                c.g[V0][0] = c.g[S2][0];
                break 'out;
            }
            // The create failed: release the lock, return -6.
            c.cycles += 3;
            Callee {
                addr: 0x003b_58d8,
                f: ee::s13::f_003b58d8,
            }
            .call(c, 0x003c_672c)?;
            c.g[V0][0] = 0xffff_ffff_ffff_fffa;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SNDSTRM_FreeSlot(a0)` (0x3c6da0): destroys the platform packet of slot `a0`
/// (`SNDPLATFORM_packetplaydestroy`) and clears its entry in the slot table at
/// `0x515abc`.
pub fn snd_strm_free_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x003c_e1a8, 0x003c_6db4)?;
        c.g[V1][0] = 0x0051_0000;
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x57b8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V0][0] = 0;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr((c.g[V1][0] as u32).wrapping_add(0x304)).sw(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SND_ChannelKeyOff(a1)` (0x3c7138): finds the channel of handle `a0` and,
/// for each voice of it that is keyed on (`SND_NextHandleChannel`), stops the
/// voices (`SND_ChannelStopVoices`) or updates the voice record. Returns the
/// handle. A `match pc` body: the converted normal entry, with the generated
/// body run for any resume point.
pub fn snd_channel_key_off(c: &mut Cpu) {
    if c.pc != 0x003c7138 {
        return ee::s13::f_003c7138__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003c7138;
    loop {
        match pc {
            0x003c7138 => {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x003c7154);
                Callee { addr: 0x003ba6e8, f: ee::s13::SND_HandleToChannel }.call(c, 0x003c7154)?;
                c.g[S0][0] = c.g[V0][0];
                let t = (c.g[S0][0] as i64) < 0;
                c.g[V1][0] = 0xffffffffffffffff;
                if t { pc = 0x003c71fc; continue; }
                pc = 0x003c7160; continue;
            }
            0x003c7160 => {
                c.cycles += 5;
                c.g[V0][0] = 0x510000;
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
                c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                let t = true;
                c.g[S1][0] = 0x8c;
                if t { pc = 0x003c71e8; continue; }
                pc = 0x003c7174; continue;
            }
            0x003c7174 => {
                c.cycles += 1;
                pc = 0x003c7178; continue;
            }
            0x003c7178 => {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f0));
                c.mult(0, 2, 5, 17);
                c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x5e));
                let t = (c.g[A0][0] as i64) >= 0;
                c.g[V1][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0x5e));
                if t { pc = 0x003c71a0; continue; }
                pc = 0x003c7190; continue;
            }
            0x003c7190 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003c7198);
                c.g[A0][0] = c.g[A1][0];
                Callee { addr: 0x003c2440, f: ee::s13::SND_ChannelStopVoices }.call(c, 0x003c7198)?;
                let t = true;
                c.g[A0][0] = c.g[S0][0];
                if t { pc = 0x003c71ec; continue; }
                pc = 0x003c71a0; continue;
            }
            0x003c71a0 => {
                c.cycles += 4;
                c.g[V0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x5d));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                if t { pc = 0x003c71e8; continue; }
                pc = 0x003c71b0; continue;
            }
            0x003c71b0 => {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x6c));
                c.sb((c.g[A2][0] as u32).wrapping_add(0x5d), c.g[V1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                c.sw((c.g[A2][0] as u32).wrapping_add(0x44), c.g[V1][0]);
                pc = 0x003c71d0; continue;
            }
            0x003c71d0 => {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[A1][0] as u32)));
                c.div(0, 2, 3);
                c.g[V0][0] = c.lo[0];
                c.sw((c.g[A2][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                pc = 0x003c71e8; continue;
            }
            0x003c71e8 => {
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
                pc = 0x003c71ec; continue;
            }
            0x003c71ec => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003c71f4);
                c.g[A1][0] = c.g[SP][0];
                Callee { addr: 0x003baa00, f: ee::s13::SND_NextHandleChannel }.call(c, 0x003c71f4)?;
                if c.g[V0][0] != 0u64 {
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                    if back_edge_unwinds(c, 0x003c7178) { return Ok(()); }
                    pc = 0x003c7178; continue;
                }
                pc = 0x003c71fc; continue;
            }
            0x003c71fc => {
                c.cycles += 7;
                c.g[V0][0] = c.g[S0][0];
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003c7138, 0x003c7218); return Ok(()); }
        }
    }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (in the machine's RAM): the
    /// fakes answer only when it holds [`OC1154_MAGIC`]. Its mailboxes: +4 the
    /// packet-create result, +8 the chunks `SND_NextHandleChannel` still
    /// returns, +0xc the `SND_HandleToChannel` result (0: the fake's own).
    pub(crate) const OC1154_CTRL: u32 = DATA + 0x21_0000;
    pub(crate) const OC1154_MAGIC: u32 = 0x1154_0c1a;

    /// A gated fake of `$target`: logs the call and returns to `ra`, or runs
    /// the body it replaced (`$generated`) while the gate is off.
    macro_rules! oc1154_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1154_CTRL) != OC1154_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    oc1154_fake!(
        oc1154_fake_get_config,
        OC1154_PREV_GET_CONFIG,
        0x003b_4dd8,
        ee::s13::f_003b4dd8__generated
    );
    oc1154_fake!(
        oc1154_fake_set_config,
        OC1154_PREV_SET_CONFIG,
        0x003b_4fc0,
        ee::s13::f_003b4fc0__generated
    );
    oc1154_fake!(
        oc1154_fake_lock,
        OC1154_PREV_LOCK,
        0x003b_58a0,
        ee::s13::f_003b58a0__generated
    );
    oc1154_fake!(
        oc1154_fake_unlock,
        OC1154_PREV_UNLOCK,
        0x003b_58d8,
        ee::s13::f_003b58d8__generated
    );
    oc1154_fake!(
        oc1154_fake_overhead,
        OC1154_PREV_OVERHEAD,
        0x003c_e198,
        ee::s14::f_003ce198__generated
    );
    oc1154_fake!(
        oc1154_fake_destroy,
        OC1154_PREV_DESTROY,
        0x003c_e1a8,
        ee::s14::f_003ce1a8__generated
    );
    oc1154_fake!(
        oc1154_fake_stop_voices,
        OC1154_PREV_STOP_VOICES,
        0x003c_2440,
        ee::s13::f_003c2440__generated
    );

    static OC1154_PREV_CREATE: AtomicUsize = AtomicUsize::new(0);
    static OC1154_PREV_HANDLE: AtomicUsize = AtomicUsize::new(0);
    static OC1154_PREV_NEXT: AtomicUsize = AtomicUsize::new(0);

    /// `SNDPLATFORM_packetplaycreate`: a non-zero mailbox word at +4 is its
    /// result (a negative one takes the failure path).
    fn oc1154_fake_create(c: &mut Cpu) {
        if c.bus.read32(OC1154_CTRL) != OC1154_MAGIC {
            return fakes::fall_back(c, &OC1154_PREV_CREATE, ee::s14::f_003ce1a0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003c_e1a0, ra);
        let forced = c.bus.read32(OC1154_CTRL + 4);
        if c.pc == ra && forced != 0 {
            c.g[V0][0] = sx(forced);
        }
    }

    /// `SND_HandleToChannel`: a non-zero mailbox word at +0xc is its result.
    fn oc1154_fake_handle(c: &mut Cpu) {
        if c.bus.read32(OC1154_CTRL) != OC1154_MAGIC {
            return fakes::fall_back(c, &OC1154_PREV_HANDLE, ee::s13::f_003ba6e8__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003b_a6e8, ra);
        let forced = c.bus.read32(OC1154_CTRL + 0xc);
        if c.pc == ra && forced != 0 {
            c.g[V0][0] = sx(forced);
        }
    }

    /// `SND_NextHandleChannel(a0, out)`: writes the next index to `*out` and
    /// returns 1 while the chunk count at +8 is left, then 0.
    fn oc1154_fake_next(c: &mut Cpu) {
        if c.bus.read32(OC1154_CTRL) != OC1154_MAGIC {
            return fakes::fall_back(c, &OC1154_PREV_NEXT, ee::s13::f_003baa00__generated);
        }
        let ra = c.g[RA][0] as u32;
        let out = c.g[A1][0] as u32;
        let left = c.bus.read32(OC1154_CTRL + 8);
        fakes::fake(c, 0x003b_aa00, ra);
        if c.pc != ra {
            return;
        }
        if left == 0 {
            c.g[V0][0] = 0;
        } else {
            c.bus.write32(OC1154_CTRL + 8, left - 1);
            c.bus.write32(out, left);
            c.g[V0][0] = 1;
        }
    }

    /// Installs this module's fakes into the override slots (once per process).
    pub(crate) fn oc1154_hook() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B4DD8),
            oc1154_fake_get_config,
            &OC1154_PREV_GET_CONFIG,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B4FC0),
            oc1154_fake_set_config,
            &OC1154_PREV_SET_CONFIG,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58A0),
            oc1154_fake_lock,
            &OC1154_PREV_LOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58D8),
            oc1154_fake_unlock,
            &OC1154_PREV_UNLOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003CE198),
            oc1154_fake_overhead,
            &OC1154_PREV_OVERHEAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003CE1A0),
            oc1154_fake_create,
            &OC1154_PREV_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003CE1A8),
            oc1154_fake_destroy,
            &OC1154_PREV_DESTROY,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003BA6E8),
            oc1154_fake_handle,
            &OC1154_PREV_HANDLE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003BAA00),
            oc1154_fake_next,
            &OC1154_PREV_NEXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C2440),
            oc1154_fake_stop_voices,
            &OC1154_PREV_STOP_VOICES,
        );
    }

    /// Turns this module's gate on for the machine `c`.
    pub(crate) fn oc1154_gate(c: &mut Cpu) {
        c.bus.write32(OC1154_CTRL, OC1154_MAGIC);
    }

    /// The sound state block (`0x510000 - 0x5718`) and the service table in it.
    const OC1154_STATE: u32 = 0x0050_a8e8;

    /// Voice records (0x8c bytes each) and the pool their `+0x6c` points at.
    const OC1154_VOICES: u32 = DATA + 0x2_0000;
    const OC1154_POOL: u32 = DATA + 0x2_8000;

    #[test]
    fn snd_set_config_cc_matches() {
        oc1154_hook();
        check(
            ee::s13::f_003c5fd0__generated,
            snd_set_config_cc,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x3c_5fd0, 3);
                oc1154_gate(c);
            },
        );
    }

    #[test]
    fn snd_remove_service_callback_matches() {
        check(
            ee::s13::f_003c6040__generated,
            snd_remove_service_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_6040, 0);
                // Count -1 (early exit), 0 (early exit), 1..4 (search and shift).
                let count = (seed % 6) as i32 - 1;
                c.bus.write8(OC1154_STATE + 0x178, count as u8);
                let mut table = [0u32; 8];
                for (i, entry) in table.iter_mut().enumerate() {
                    *entry = r.next_u32();
                    c.bus.write32(OC1154_STATE + 0x19c + 4 * i as u32, *entry);
                }
                let target = if count > 0 && seed % 4 != 3 {
                    table[(seed as usize / 6) % count as usize]
                } else {
                    r.next_u32()
                };
                c.g[A0][0] = sx(target);
            },
        );
    }

    #[test]
    fn snd_mem_get_free_percent_matches() {
        check(
            ee::s13::f_003c6240__generated,
            snd_mem_get_free_percent,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_6240, 0);
                let block = DATA + 0x1000;
                c.bus.write32(0x0050_ab1c, block);
                // The divisor is never zero (the division is not under test).
                c.bus.write32(block + 8, r.next_u32() | 1);
                c.bus.write32(block + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn snd_strm_calc_ring_size_matches() {
        check(
            ee::s13::f_003c65e0__generated,
            snd_strm_calc_ring_size,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x3c_65e0, 0);
            },
        );
    }

    #[test]
    fn snd_strm_calc_slot_mem_size_matches() {
        oc1154_hook();
        check(
            ee::s13::f_003c65f0__generated,
            snd_strm_calc_slot_mem_size,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x3c_65f0, 1);
                oc1154_gate(c);
            },
        );
    }

    #[test]
    fn snd_strm_alloc_slot_matches() {
        oc1154_hook();
        check(
            ee::s13::f_003c6620__generated,
            snd_strm_alloc_slot,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_6620, 4);
                oc1154_gate(c);
                // The flag byte: 0 takes the early exit, else the entry count.
                let flag = if seed % 4 == 0 { 0 } else { 1 + (seed % 3) as u8 };
                c.bus.write8(0x0050_a92e, flag);
                // The slot table: zero entries are free.
                for i in 0..4u32 {
                    let v = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x3_0000 + 4 * i
                    };
                    c.bus.write32(0x0051_5abc + 4 * i, v);
                }
                // Create result: negative on a third of the seeds.
                c.bus.write32(
                    OC1154_CTRL + 4,
                    if seed % 3 == 0 { 0xffff_ffff } else { 0 },
                );
                c.g[A3][0] = sx(DATA + 0x3000);
            },
        );
    }

    #[test]
    fn snd_strm_free_slot_matches() {
        oc1154_hook();
        check(
            ee::s13::f_003c6da0__generated,
            snd_strm_free_slot,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x3c_6da0, 1);
                oc1154_gate(c);
                // The slot index selects a table entry.
                c.g[A0][0] = (seed % 16) as u64;
            },
        );
    }

    #[test]
    fn snd_channel_key_off_matches() {
        oc1154_hook();
        check(
            ee::s13::f_003c7138__generated,
            snd_channel_key_off,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_7138, 4);
                oc1154_gate(c);
                c.pc = 0x003c_7138;
                // Voice records: the flag byte (-1..2), the count byte (0..3),
                // the pool pointer and a word.
                c.bus.write32(0x0050_aad8, OC1154_VOICES);
                for idx in 0..4u32 {
                    let rec = OC1154_VOICES + 0x8c * idx;
                    c.bus.write8(rec + 0x5e, ((r.next_u32() % 4).wrapping_sub(1)) as u8);
                    c.bus.write8(rec + 0x5d, (r.next_u32() % 4) as u8);
                    c.bus.write32(rec + 0x6c, OC1154_POOL);
                    c.bus.write32(rec + 0x40, r.next_u32());
                }
                for k in 0..16u32 {
                    c.bus.write32(OC1154_POOL + 4 * k, r.next_u32() | 1);
                }
                // Chunks left for SND_NextHandleChannel, and the handle result.
                c.bus.write32(OC1154_CTRL + 8, seed % 4);
                c.bus
                    .write32(OC1154_CTRL + 0xc, if seed % 5 == 0 { 0xffff_ffff } else { 0 });
            },
        );
    }
}
