//! Staging wave 209 (oc273): hand-written replacements for the hottest
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
    (0x0035_1a80, intersect_info_find_key_segment, "IntersectInfo_FindKeySegment"),
    (0x003f_f390, libpad_send_act_dma, "libpad_SendActDma"),
    (0x003d_b5c0, spch_random_no_repeat, "SPCH_RandomNoRepeat"),
    (0x0030_a3a0, wscript_man_focus_enter, "WScriptMan_FocusEnter"),
    (0x0040_0190, sce_pad_set_act_direct, "scePadSetActDirect"),
    (0x0010_4d68, rider_anim_save_state, "RiderAnim_SaveState"),
    (0x0035_67e0, mod_obj_v49, "ModObj_v49"),
    (0x003d_b4d0, spch_rand_next, "SPCH_RandNext"),
    (0x0037_0aa8, emitter_save, "Emitter_Save"),
];


// ---------------------------------------------------------------------------
// IntersectInfo_FindKeySegment (0x351a80)

/// Range table: `count` at +0, `entries` pointer at +4; each 0x18-byte entry
/// holds its key range `[lo, hi)` at +0x10/+0x14.
const IKS_COUNT: u32 = 0x0;
const IKS_ENTRIES: u32 = 0x4;
const IKS_ENTRY_SIZE: u32 = 0x18;
const IKS_LO: u32 = 0x10;
const IKS_HI: u32 = 0x14;

/// `sImRange* IntersectInfo_FindKeySegment(sImRangeTable* t, int* cur, float x)`
/// (0x351a80): find the key segment containing `x`, trying the one after the
/// cached cursor first (`lo <= x < hi`), else scanning from the start for the
/// first entry with `x < hi` (the last entry when none matches). Updates the
/// cursor and returns the entry.
pub fn intersect_info_find_key_segment(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    let table = Addr(r32(c, A0));
    let cur = Addr(r32(c, A1));
    let x = FpuF32(c.f[12]);

    c.cycles += 7;
    let next = sx((cur.lw(c) as u32).wrapping_add(1));
    c.g[A2][0] = next;
    cur.sw(c, next);
    let count = table.off(IKS_COUNT).lw(c);
    let in_table = (next as i64) < (count as i64);
    c.g[V0][0] = IKS_ENTRY_SIZE as u64;
    'scan: {
        if in_table {
            // Fast path: the entry after the cursor.
            c.cycles += 8;
            c.g[V1][0] = table.off(IKS_ENTRIES).lw(c);
            c.mult(0, V0, A2, V0);
            c.g[A2][0] = crate::abi::addu(c.g[V1][0], c.g[V0][0]);
            let e = Addr(r32(c, A2));
            let lo = e.off(IKS_LO).read_fpu(c);
            c.f[0] = lo.0;
            let above_lo = lo.le(x);
            crate::abi::set_fcc(c, above_lo);
            c.g[A3][0] = 0;
            if above_lo {
                c.cycles += 5;
                let hi = e.off(IKS_HI).read_fpu(c);
                c.f[0] = hi.0;
                if crate::abi::c_lt_s(c, x, hi) {
                    c.g[A3][0] = 1;
                }
            }
            c.cycles += 2;
            if c.g[A3][0] != 0 {
                break 'scan;
            }
            c.g[A2][0] = table.off(IKS_ENTRIES).lw(c);
        } else {
            c.cycles += 1;
            c.g[A2][0] = table.off(IKS_ENTRIES).lw(c);
        }

        // Scan from the start.
        c.cycles += 5;
        cur.sw(c, 0);
        c.g[V0][0] = sx((table.off(IKS_COUNT).lw(c) as u32).wrapping_sub(1));
        if (c.g[V0][0] as i64) <= 0 {
            break 'scan;
        }
        loop {
            c.cycles += 5;
            let hi = Addr(r32(c, A2)).off(IKS_HI).read_fpu(c);
            c.f[0] = hi.0;
            let below = crate::abi::c_lt_s(c, x, hi);
            c.g[V0][0] = 1;
            if !below {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 2;
            if below {
                break 'scan;
            }
            c.cycles += 8;
            let i = sx((cur.lw(c) as u32).wrapping_add(1));
            cur.sw(c, i);
            let last = sx((table.off(IKS_COUNT).lw(c) as u32).wrapping_sub(1));
            c.g[V0][0] = last;
            let more = (i as i64) < (last as i64);
            c.g[V1][0] = more as u64;
            c.g[A2][0] = crate::abi::addu(c.g[A2][0], IKS_ENTRY_SIZE as u64);
            if !more {
                break 'scan;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_1af8);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    ret(c);
}

// ---------------------------------------------------------------------------
// libpad_SendActDma (0x3ff390)

/// libpad's per-port/slot state table (0x526250): 0x70 bytes per port,
/// 0x1c per slot.
const PAD_SLOTS: u32 = 0x0052_6250;
const PAD_PORT_STRIDE: u32 = 0x70;
const PAD_SLOT_STRIDE: u32 = 0x1c;
/// Slot fields: actuator packet buffer (+4), IOP destination (+8), last
/// DMA id (+0xc).
const PAD_ACT_BUF: u32 = 0x4;
const PAD_IOP_DEST: u32 = 0x8;
const PAD_DMA_ID: u32 = 0xc;
/// libpad's debug-print flag (0x4515cc) and the "busy" message (0x497888).
const PAD_DEBUG_BASE: u32 = 0x0045_0000;
const PAD_DEBUG_FLAG: u32 = 0x15cc;
const PAD_BUSY_MSG: u32 = 0x0049_7888;

/// `int libpad_SendActDma(int port, int slot)` (0x3ff390): send the slot's
/// actuator (vibration) packet to the IOP. If the previous DMA is still
/// running (`sceSifDmaStat >= 0`) it gives up (printing a message when
/// libpad debugging is on) and returns 0; otherwise it bumps the packet
/// counter, writes back the buffer, DMAs 0x20 bytes into the IOP's
/// double-buffered destination and records the DMA id (returns 1; 0 and
/// the message when `sceSifSetDma` fails).
pub fn libpad_send_act_dma(c: &mut Cpu) {
    use crate::guest::Addr;
    const DMA_STAT: Callee = Callee { addr: 0x0042_4140, f: ee::s15::sceSifDmaStat };
    const SYNC_DCACHE: Callee = Callee { addr: 0x0042_4698, f: ee::s15::SyncDCache };
    const SET_DMA: Callee = Callee { addr: 0x0042_4160, f: ee::s15::sceSifSetDma };
    const PRINTF: Callee = Callee { addr: 0x0042_5c60, f: ee::s15::scePrintf };
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x150);
        c.g[V1][0] = PAD_SLOT_STRIDE as u64;
        c.sd(frame.at(c, 0x120), c.g[S2][0]);
        c.sd(frame.at(c, 0x100), c.g[S0][0]);
        c.g[S2][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[A0][0] = PAD_PORT_STRIDE as u64;
        c.mult(1, A0, S2, A0);
        c.mult(0, V1, S0, V1);
        c.sd(frame.at(c, 0x130), c.g[S3][0]);
        c.g[S3][0] = 0x0052_0000;
        c.sd(frame.at(c, 0x140), c.g[RA][0]);
        c.sd(frame.at(c, 0x110), c.g[S1][0]);
        c.g[V0][0] = sx(PAD_SLOTS);
        c.g[V1][0] = crate::abi::addu(c.g[V1][0], c.g[A0][0]);
        let slot = PAD_SLOTS.wrapping_add(r32(c, V1));
        c.g[A1][0] = sx(slot);
        c.g[V0][0] = sx(slot);
        c.g[A0][0] = Addr(slot).off(PAD_DMA_ID).lw(c);
        c.g[S1][0] = Addr(slot).off(PAD_ACT_BUF).lw(c);

        let mut busy = false;
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            DMA_STAT.call(c, 0x003f_f3e8)?;
            busy = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = sx(PAD_DEBUG_BASE);
        }
        if !busy {
            c.cycles += 28;
            c.g[A3][0] = PAD_SLOT_STRIDE as u64;
            c.g[V1][0] = PAD_PORT_STRIDE as u64;
            c.mult(0, A3, S0, A3);
            c.mult(1, V1, S2, V1);
            let buf = r32(c, S1);
            let counter = sx((Addr(buf).lw(c) as u32).wrapping_add(1));
            c.g[A2][0] = counter;
            c.g[S3][0] = sx(PAD_SLOTS);
            c.g[A0][0] = sx(buf);
            c.g[A1][0] = sx(buf.wrapping_add(0x1f));
            c.g[S2][0] = crate::abi::addu(c.g[A3][0], c.g[V1][0]);
            let slot = PAD_SLOTS.wrapping_add(r32(c, S2));
            c.g[V1][0] = sx(slot);
            let half = (counter as u32 & 1) << 5;
            c.g[V0][0] = half as u64;
            let dest = (Addr(slot).off(PAD_IOP_DEST).lw(c) as u32).wrapping_add(half);
            Addr(buf).sw(c, counter);
            c.g[S0][0] = sx(dest);
            SYNC_DCACHE.call(c, 0x003f_f434)?;

            // One sceSifDmaData {src, dest, size 0x20, attr 0} on the stack.
            c.g[V0][0] = 0x20;
            let sp0 = frame.at(c, 0);
            let (dest, src) = (c.g[S0][0], c.g[S1][0]);
            Addr(sp0).off(4).sw(c, dest);
            Addr(sp0).sw(c, src);
            c.g[A0][0] = c.g[SP][0];
            Addr(sp0).off(8).sw(c, 0x20);
            c.g[A1][0] = 1;
            Addr(sp0).off(0xc).sw(c, 0);
            SET_DMA.call(c, 0x003f_f454)?;
            c.g[A0][0] = c.g[V0][0];
            let slot = r32(c, S3).wrapping_add(r32(c, S2));
            c.g[V1][0] = sx(slot);
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = 1;
                let id = c.g[A0][0];
                Addr(slot).off(PAD_DMA_ID).sw(c, id);
                return pad_send_act_dma_epilogue(c, frame);
            }
            c.cycles += 1;
            c.g[V0][0] = sx(PAD_DEBUG_BASE);
        }

        // Busy (or failed): print when debugging, return 0.
        c.cycles += 3;
        c.g[V1][0] = Addr(PAD_DEBUG_BASE).off(PAD_DEBUG_FLAG).lw(c);
        c.g[A0][0] = 0x0049_0000;
        if c.g[V1][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = sx(PAD_BUSY_MSG);
            PRINTF.call(c, 0x003f_f478)?;
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        pad_send_act_dma_epilogue(c, frame)
    })
}

/// libpad_SendActDma's epilogue: reload ra, s3..s0 (`ld`), pop the frame.
fn pad_send_act_dma_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 7;
    frame.restore_ra(c, 0x140);
    c.g[S3][0] = c.ld(frame.at(c, 0x130));
    c.g[S2][0] = c.ld(frame.at(c, 0x120));
    c.g[S1][0] = c.ld(frame.at(c, 0x110));
    c.g[S0][0] = c.ld(frame.at(c, 0x100));
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// SPCH_RandomNoRepeat (0x3db5c0)

/// The speech picker's history ring: 32 entries of {u16 value, u16 key} at
/// 0x450b88, the newest at index `gp+0x1768` (0x4a4858).
const SPCH_HISTORY: u32 = 0x0045_0b88;
const SPCH_HISTORY_KEY: u32 = 0x0045_0b8a;
const SPCH_HISTORY_POS: u32 = 0x1768;
const SPCH_RAND_NEXT: Callee = Callee { addr: 0x003d_b4d0, f: ee::s14::SPCH_RandNext };

/// `int SPCH_RandomNoRepeat(int range, int key)` (0x3db5c0): a random
/// value in `[0, range)` (`(SPCH_RandNext() >> 16) * range >> 16`) that
/// avoids the values recently picked for `key`. With `key == -1` the first
/// draw is returned as is. Otherwise up to 32 draws are made; each is
/// looked up among the last `min(range / 2 clamped to 10, 32)` history
/// entries with the same key (newest first): an unseen value is taken at
/// once, else the one seen longest ago wins. The pick is pushed into the
/// history ring (value, key); the return value is the last draw.
///
/// Register roles, as the original keeps them: s4 range, s5 key (masked to
/// 16 bits), s3 how far back to look, s0 tries, s1 done, s2 pick, fp the
/// oldest hit so far; inner loop t0 limit, t1 same-key entries seen, t2
/// draw, t3 steps, t4 hit position, a1 ring index, a2 stop.
pub fn spch_random_no_repeat(c: &mut Cpu) {
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0xa0);
        for (reg, off) in [
            (S4, 0x40),
            (S3, 0x30),
            (S5, 0x50),
            (FP, 0x80),
            (S2, 0x20),
            (S1, 0x10),
            (S0, 0x00),
            (RA, 0x90),
            (S7, 0x70),
            (S6, 0x60),
        ] {
            frame.save(c, reg, off);
        }
        c.g[V1][0] = 0xa;
        c.g[S4][0] = c.g[A0][0];
        let range = r32(c, S4);
        // half = range / 2 (rounded toward zero), at most 10.
        let half = ((range.wrapping_add(range >> 31)) as i32 >> 1) as u32;
        c.g[S3][0] = sx(half);
        c.g[V0][0] = ((half as i32) < 11) as u64;
        c.g[S5][0] = c.g[A1][0];
        if c.g[V0][0] == 0 {
            c.g[S3][0] = 0xa;
        }
        c.g[S0][0] = 0;
        c.g[S1][0] = 0;
        SPCH_RAND_NEXT.call(c, 0x003d_b618)?;
        c.g[FP][0] = u64::MAX;
        c.g[V0][0] = sx(r32(c, V0) >> 16);
        c.mult(0, V0, V0, S4);
        c.g[S2][0] = sx(r32(c, V0) >> 16);
        if c.g[S5][0] == u64::MAX {
            c.cycles += 2;
            c.g[V0][0] = c.g[S2][0];
            return spch_random_no_repeat_epilogue(c, frame);
        }

        c.cycles += 3;
        c.g[S5][0] &= 0xffff;
        c.g[S7][0] = u64::MAX;
        c.g[S6][0] = 1;
        loop {
            // One draw.
            c.cycles += 16;
            SPCH_RAND_NEXT.call(c, 0x003d_b648)?;
            c.g[V0][0] = sx(r32(c, V0) >> 16);
            c.g[T0][0] = c.g[S3][0];
            c.mult(0, V0, V0, S4);
            c.g[V1][0] = ((c.g[T0][0] as i64) < 33) as u64;
            c.g[A0][0] = 0x20;
            let gp = r32(c, GP);
            c.g[A1][0] = Addr(gp).off(SPCH_HISTORY_POS).lw(c);
            c.g[T5][0] = sx(r32(c, S0).wrapping_add(1));
            if c.g[V1][0] == 0 {
                c.g[T0][0] = 0x20;
            }
            c.g[T4][0] = u64::MAX;
            c.g[T1][0] = 0;
            c.g[T3][0] = 0;
            c.g[T2][0] = sx(r32(c, V0) >> 16);
            c.g[A2][0] = 0;

            // Search the history ring backwards for this draw.
            loop {
                c.cycles += 3;
                let in_window = (c.g[T1][0] as i64) < (c.g[T0][0] as i64);
                c.g[V0][0] = sx(0x0045_0000);
                if !in_window {
                    c.cycles += 1;
                    c.g[A2][0] = 1;
                    c.cycles += 1;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                } else {
                    c.cycles += 8;
                    let at = r32(c, A1) << 2;
                    c.g[A0][0] = sx(at);
                    c.g[A3][0] = sx(SPCH_HISTORY);
                    c.g[V0][0] = sx(at.wrapping_add(SPCH_HISTORY_KEY));
                    c.g[V1][0] = Addr(at.wrapping_add(SPCH_HISTORY_KEY)).lhu(c);
                    if c.g[V1][0] != c.g[S5][0] {
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = sx(at.wrapping_add(SPCH_HISTORY));
                        c.g[V0][0] = Addr(at.wrapping_add(SPCH_HISTORY)).lhu(c);
                        let same = c.g[V0][0] == c.g[T2][0];
                        c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
                        if same {
                            c.cycles += 3;
                            c.g[A2][0] = 1;
                            c.g[T4][0] = sx(r32(c, T1).wrapping_sub(1));
                        }
                        c.cycles += 1;
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                    }
                }
                c.cycles += 7;
                c.g[T3][0] = sx(r32(c, T3).wrapping_add(1));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x20));
                c.g[V1][0] = ((c.g[S7][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[A0][0] = ((c.g[T3][0] as i64) < 33) as u64;
                if c.g[A0][0] == 0 {
                    c.g[A2][0] = c.g[S6][0];
                }
                let again = c.g[A2][0] == 0;
                if c.g[V1][0] == 0 {
                    c.g[A1][0] = c.g[V0][0];
                }
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_b680);
                }
            }

            // Unseen: take it; seen longer ago than the best so far: keep it.
            c.cycles += 2;
            c.g[V0][0] = ((c.g[FP][0] as i64) < (c.g[T4][0] as i64)) as u64;
            if c.g[T4][0] == c.g[S7][0] {
                c.cycles += 3;
                c.g[S2][0] = c.g[T2][0];
                c.g[S1][0] = 1;
                c.cycles += 1;
                c.g[S0][0] = c.g[T5][0];
            } else {
                c.cycles += 2;
                c.g[S0][0] = c.g[T5][0];
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[FP][0] = c.g[T4][0];
                    c.g[S2][0] = c.g[T2][0];
                    c.cycles += 1;
                    c.g[S0][0] = c.g[T5][0];
                }
            }
            c.cycles += 4;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 32) as u64;
            if c.g[V0][0] == 0 {
                c.g[S1][0] = c.g[S6][0];
            }
            let more = c.g[S1][0] == 0;
            let gp = r32(c, GP);
            c.g[V0][0] = Addr(gp).off(SPCH_HISTORY_POS).lw(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_b640);
            }
        }

        // Push the pick into the history ring.
        c.cycles += 4;
        let gp = r32(c, GP);
        let pos = Addr(gp).off(SPCH_HISTORY_POS);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V1][0] = ((c.g[V0][0] as i64) < 32) as u64;
        let next = c.g[V0][0];
        pos.sw(c, next);
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            pos.sw(c, 0);
        }
        c.cycles += 9;
        let at = (pos.lw(c) as u32) << 2;
        c.g[V1][0] = sx(at);
        c.g[A1][0] = sx(at.wrapping_add(SPCH_HISTORY));
        c.g[A0][0] = sx(SPCH_HISTORY.wrapping_add(at));
        c.g[V0][0] = c.g[T2][0];
        let (pick, key) = (c.g[S2][0], c.g[S5][0]);
        Addr(SPCH_HISTORY.wrapping_add(at)).sh(c, pick);
        Addr(SPCH_HISTORY.wrapping_add(at)).off(2).sh(c, key);
        spch_random_no_repeat_epilogue(c, frame)
    })
}

/// SPCH_RandomNoRepeat's epilogue: `lq` ra, fp, s7..s0, pop the frame.
fn spch_random_no_repeat_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 12;
    for (reg, off) in [
        (RA, 0x90),
        (FP, 0x80),
        (S7, 0x70),
        (S6, 0x60),
        (S5, 0x50),
        (S4, 0x40),
        (S3, 0x30),
        (S2, 0x20),
        (S1, 0x10),
        (S0, 0x00),
    ] {
        frame.restore(c, reg, off);
    }
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// WScriptMan_FocusEnter (0x30a3a0)

/// Controller vtable slot v21 ("focus enter") and the object-type table
/// entry size; type 3 objects run their slot-1 object script.
const WSF_CTL_V21: u32 = 0xa8;
const WSF_MAN_CACHE: u32 = 0x28c;
const WSF_OBJ_TYPE: u32 = 0x78;
const WSF_TYPE_ENTRY: u32 = 0xc;
const WSF_TYPE_SCRIPTED: u64 = 3;
const WSF_HAS_OBJ_SCRIPT1: Callee = Callee { addr: 0x0030_a270, f: ee::s10::WScriptMan_HasObjScript1 };
const WSF_RUN_OBJ_SCRIPT1: Callee = Callee { addr: 0x0030_a298, f: ee::s10::WScriptMan_RunObjScript1 };

/// `void WScriptMan_FocusEnter(WScriptMan* self, WorldObj* obj)` (0x30a3a0):
/// the camera/focus enters `obj`. A controller that handles focus itself
/// (v20 non-zero) gets v21(obj); without a controller, an object whose type
/// (from the script cache's type table) is 3 runs its slot-1 object script
/// if it has one.
pub fn wscript_man_focus_enter(c: &mut Cpu) {
    use crate::game::wscript::wscriptman::{oc264_wctl_call, OC264_WCTL_V20, OC264_WOBJ_CTL};
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(OC264_WOBJ_CTL)).lw(c);
        c.g[S1][0] = c.g[A0][0];
        if c.g[A1][0] != 0 {
            c.cycles += 7;
            oc264_wctl_call(c, A1, V1, V0, OC264_WCTL_V20, 0x0030_a3d4)?;
            if c.g[V0][0] != 0 {
                c.cycles += 9;
                c.g[A2][0] = Addr(r32(c, S0).wrapping_add(OC264_WOBJ_CTL)).lw(c);
                c.g[A1][0] = c.g[S0][0];
                oc264_wctl_call(c, A2, V1, V0, WSF_CTL_V21, 0x0030_a3f8)?;
            }
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 11;
            // self->cache->+4->+4: the object-type table (12-byte entries).
            let cache = Addr(r32(c, S1).wrapping_add(WSF_MAN_CACHE)).lw(c);
            c.g[A0][0] = cache;
            c.g[A1][0] = WSF_TYPE_ENTRY as u64;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(WSF_OBJ_TYPE)).lbu(c);
            c.g[V0][0] = Addr((cache as u32).wrapping_add(4)).lw(c);
            c.mult(0, V1, V1, A1);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            c.g[V1][0] = crate::abi::addu(c.g[V1][0], c.g[A0][0]);
            c.g[V0][0] = Addr(r32(c, V1)).lh(c) ^ WSF_TYPE_SCRIPTED;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                WSF_HAS_OBJ_SCRIPT1.call(c, 0x0030_a438)?;
                let has = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S1][0];
                if has {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    WSF_RUN_OBJ_SCRIPT1.call(c, 0x0030_a448)?;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// scePadSetActDirect (0x400190)

/// Slot field: the pad's DMA receive buffer (two 0x80-byte halves; the
/// current one has the connection state byte at +0x72, 1 = stable).
const PAD_STATE_STABLE: u64 = 1;
const PAD_DMA_STATE: u32 = 0x72;
/// Actuator packet: command (+4) = 1, length (+8) = 6, the six motor bytes
/// at +0xc.
const PAD_ACT_CMD: u32 = 0x4;
const PAD_ACT_LEN: u32 = 0x8;
const PAD_ACT_DATA: u32 = 0xc;
const PAD_ACT_BYTES: u32 = 6;

/// `int scePadSetActDirect(int port, int slot, const u8* data)` (0x400190):
/// set the vibration motors of a connected pad. Returns 0 unless the pad's
/// state is "stable"; then copies the six actuator bytes into the slot's
/// actuator packet (command 1, length 6) and sends it
/// (`libpad_SendActDma`), returning 1 when that succeeded.
pub fn sce_pad_set_act_direct(c: &mut Cpu) {
    use crate::guest::Addr;
    const GET_DMA_BUFFER: Callee = Callee { addr: 0x003f_f9a8, f: ee::s15::libpad_GetDmaBuffer };
    const SEND_ACT_DMA: Callee = Callee { addr: 0x003f_f390, f: ee::s15::libpad_SendActDma };
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S2][0] = c.g[A1][0];
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S0][0] = c.g[A2][0];
        GET_DMA_BUFFER.call(c, 0x0040_01b4)?;
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(PAD_DMA_STATE)).lbu(c);
        let stable = c.g[V1][0] == PAD_STATE_STABLE;
        c.g[V0][0] = 0;
        if stable {
            c.cycles += 11;
            c.g[V1][0] = PAD_SLOT_STRIDE as u64;
            c.g[A0][0] = PAD_PORT_STRIDE as u64;
            c.mult(0, V1, S2, V1);
            c.mult(1, A0, S1, A0);
            c.g[A1][0] = 0;
            c.g[V1][0] = crate::abi::addu(c.g[V1][0], c.g[A0][0]);
            let slot = PAD_SLOTS.wrapping_add(r32(c, V1));
            c.g[V0][0] = sx(slot);
            let packet = Addr(slot).off(PAD_ACT_BUF).lw(c);
            c.g[A2][0] = packet;
            c.g[A3][0] = sx((packet as u32).wrapping_add(PAD_ACT_DATA));
            // Copy the six motor bytes.
            loop {
                c.cycles += 8;
                let i = r32(c, A1);
                let src = r32(c, S0).wrapping_add(i);
                let dst = r32(c, A3).wrapping_add(i);
                c.g[V0][0] = sx(src);
                c.g[A0][0] = sx(dst);
                let byte = Addr(src).lbu(c);
                c.g[V1][0] = byte;
                c.g[A1][0] = sx(i.wrapping_add(1));
                let more = ((c.g[A1][0] as i64) < PAD_ACT_BYTES as i64) as u64;
                c.g[V0][0] = more;
                Addr(dst).sb(c, byte);
                if more == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0040_01f0);
                }
            }
            c.cycles += 10;
            c.g[S0][0] = 1;
            c.g[V0][0] = PAD_ACT_BYTES as u64;
            let packet = Addr(r32(c, A2));
            packet.off(PAD_ACT_CMD).sw(c, 1);
            c.g[A0][0] = c.g[S1][0];
            packet.off(PAD_ACT_LEN).sw(c, PAD_ACT_BYTES as u64);
            c.g[A1][0] = c.g[S2][0];
            SEND_ACT_DMA.call(c, 0x0040_022c)?;
            c.g[V0][0] ^= 1;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = 0;
            }
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x30);
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAnim_SaveState (0x104d68)

/// Rider animation state: 0x50 bytes of plain state, then (+0x50) a pointer
/// to six 8-byte animation sequence lists.
const RAS_STATE_BYTES: u64 = 0x50;
const RAS_SEQ_LISTS: u32 = 0x50;
const RAS_SEQ_LIST_COUNT: u32 = 6;
/// cStream vtable slot "write(buf, n)" (this-adjust halfword +8, method +0xc).
const RAS_STREAM_WRITE: u32 = 0x8;
const RAS_ANIM_SEQ_LIST_SAVE: u32 = 0x0031_47f0;

/// `void RiderAnim_SaveState(RiderAnim* self, cStream* s)` (0x104d68): write
/// the first 0x50 bytes of the animation state to the stream, then save
/// each of the six animation sequence lists (`AnimSeqList_Save`).
pub fn rider_anim_save_state(c: &mut Cpu) {
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = RAS_STATE_BYTES;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S2][0];
        // s->write(self, 0x50)
        let stream = r32(c, S1);
        let vtable = Addr(stream).lw(c);
        c.g[V0][0] = vtable;
        let adjust = Addr((vtable as u32).wrapping_add(RAS_STREAM_WRITE)).lh(c);
        let method = Addr((vtable as u32).wrapping_add(RAS_STREAM_WRITE + 4)).lw(c);
        c.g[V1][0] = method;
        c.g[A0][0] = sx(stream.wrapping_add(adjust as u32));
        call_ptr(c, method as u32, 0x0010_4da4)?;
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(RAS_SEQ_LISTS)).lw(c);
        loop {
            c.cycles += 8;
            let i = r32(c, S0);
            c.g[A1][0] = c.g[S1][0];
            c.g[S0][0] = sx(i.wrapping_add(1));
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(i << 3));
            call_ptr(c, RAS_ANIM_SEQ_LIST_SAVE, 0x0010_4dbc)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < RAS_SEQ_LIST_COUNT as i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(r32(c, S2).wrapping_add(RAS_SEQ_LISTS)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0010_4da8);
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

// ---------------------------------------------------------------------------
// ModObj_v49 (0x3567e0)

/// Modifier object: its move node (+0x18; world-space bounds min at +0x60,
/// max at +0x6c, three floats each) and modifier block (+0x1c, whose first
/// word is the implementation).
const MOV49_NODE: u32 = 0x18;
const MOV49_BLOCK: u32 = 0x1c;
const MOV49_NODE_MIN: u32 = 0x60;
const MOV49_NODE_MAX: u32 = 0x6c;
/// Stack: new box (min +0, max +0x10), old box (+0x20, +0x30), a vector
/// staging slot (+0x40).
const MOV49_NEW: u32 = 0x00;
const MOV49_OLD: u32 = 0x20;
const MOV49_STAGE: u32 = 0x40;
const MOV49_WORLD_GET_NASH: u32 = 0x002d_1be0;
const MOV49_SPATIAL_HASH_UPDATE: u32 = 0x0032_91e0;

/// `void ModObj_v49(ModObj* self)` (0x3567e0): refresh the modifier's
/// bounds after its move node moved. Without a block implementation
/// nothing happens. Otherwise the old bounds (`GetTransformBounds`) are
/// kept, the new box is built from the node's world bounds (w = 1), set on
/// the block (`SetTransformBounds`) and the world's spatial hash ("nash") is
/// updated from the old box to the new one.
pub fn mod_obj_v49(c: &mut Cpu) {
    use crate::guest::Addr;
    const GET_BOUNDS: Callee = Callee { addr: 0x0035_2b88, f: ee::s11::ModifierBlock_GetTransformBounds };
    const SET_BOUNDS: Callee = Callee { addr: 0x0035_2bc0, f: ee::s11::ModifierBlock_SetTransformBounds };
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x50);
        c.g[S0][0] = c.g[A0][0];
        let block = Addr(r32(c, S0).wrapping_add(MOV49_BLOCK)).lw(c);
        c.g[V0][0] = 0;
        if block != 0 {
            c.cycles += 1;
            c.g[V0][0] = Addr(block as u32).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 38;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(MOV49_BLOCK)).lw(c);
            GET_BOUNDS.call(c, 0x0035_6810)?;
            // Keep the old box.
            let old = r32(c, V0);
            let sp0 = frame.at(c, 0);
            c.lq(V1, old);
            c.g[A1][0] = sx(sp0);
            let node = Addr(r32(c, S0).wrapping_add(MOV49_NODE)).lw(c) as u32;
            c.g[A2][0] = sx(node);
            c.sq(V1, sp0 + MOV49_OLD);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2);
            c.lq(V1, old.wrapping_add(0x10));
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(MOV49_BLOCK)).lw(c);
            c.sq(V1, sp0 + MOV49_OLD + 0x10);
            // New min (x, y, z, 1) through the staging slot.
            let stage = sp0 + MOV49_STAGE;
            c.lwc1(0, node + MOV49_NODE_MIN);
            c.swc1(0, stage);
            c.lwc1(1, node + MOV49_NODE_MIN + 4);
            c.swc1(1, stage + 4);
            c.lwc1(0, node + MOV49_NODE_MIN + 8);
            c.swc1(2, stage + 0xc);
            c.swc1(0, stage + 8);
            c.lq(V0, stage);
            c.sq(V0, sp0 + MOV49_NEW);
            // New max (x, y, z, w = 1 still in the slot).
            c.lwc1(0, node + MOV49_NODE_MAX);
            c.swc1(0, stage);
            c.lwc1(1, node + MOV49_NODE_MAX + 4);
            c.swc1(1, stage + 4);
            c.lwc1(0, node + MOV49_NODE_MAX + 8);
            c.swc1(0, stage + 8);
            c.lq(V0, stage);
            c.sq(V0, sp0 + MOV49_NEW + 0x10);
            SET_BOUNDS.call(c, 0x0035_687c)?;
            c.g[S0][0] = Addr(r32(c, S0).wrapping_add(MOV49_NODE)).lw(c);
            call_ptr(c, MOV49_WORLD_GET_NASH, 0x0035_6884)?;
            // SpatialHash_Update(nash, 0, node, &new, &old)
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            let sp0 = frame.at(c, 0);
            c.g[A3][0] = sx(sp0 + MOV49_NEW);
            c.g[T0][0] = sx(sp0 + MOV49_OLD);
            call_ptr(c, MOV49_SPATIAL_HASH_UPDATE, 0x0035_689c)?;
        }
        frame.restore(c, S0, 0x60);
        c.cycles += 3;
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SPCH_RandNext (0x3db4d0)

/// The speech RNG state: six words at 0x450b70 (a 160-bit sum chain plus a
/// counter in word 5).
const SPCH_RNG_BASE: u32 = 0x0045_0000;
const SPCH_RNG: u32 = 0x0045_0b70;

/// `u32 SPCH_RandNext(void)` (0x3db4d0): the speech system's random number
/// generator. Adds word 5 into word 4 and ripples the sum with carries
/// through words 3..0 (each word becomes the running sum), increments the
/// counter (word 5), and when the counter wraps to 0 increments words 4..0
/// in turn until one does not wrap. Returns the new word 0.
pub fn spch_rand_next(c: &mut Cpu) {
    use crate::guest::Addr;
    let w = |i: u32| Addr(SPCH_RNG + 4 * i);
    // 64-bit `addu` / `sltu` on sign-extended words, as the original.
    let add = |a: u64, b: u64| sx((a as u32).wrapping_add(b as u32));
    c.cycles += 9;
    c.g[T2][0] = sx(SPCH_RNG_BASE);
    c.g[V1][0] = sx(SPCH_RNG);
    let w5 = w(5).lw(c);
    let w4 = w(4).lw(c);
    let mut sum = add(w5, w4);
    let mut carry = 0u64;
    if sum < w5 {
        c.cycles += 2;
        carry = 1;
    } else {
        c.cycles += 3;
        if sum < w4 {
            c.cycles += 2;
            carry = 1;
        }
    }
    c.g[T0][0] = sx(SPCH_RNG);

    c.cycles += 24;
    let w0 = w(0).lw(c);
    let w3 = w(3).lw(c);
    let w2 = w(2).lw(c);
    w(4).sw(c, sum);
    sum = add(add(sum, w3), carry);
    let w1 = w(1).lw(c);
    carry = (sum < w3) as u64;
    w(3).sw(c, sum);
    sum = add(add(sum, w2), carry);
    let counter = add(w(5).lw(c), 1);
    carry = (sum < w2) as u64;
    w(2).sw(c, sum);
    sum = add(add(sum, w1), carry);
    w(5).sw(c, counter);
    w(1).sw(c, sum);
    let a0 = add(sum, w0);
    carry = (sum < w1) as u64;
    sum = add(a0, carry);
    w(0).sw(c, sum);
    c.g[V1][0] = counter;
    c.g[A0][0] = a0;
    c.g[A1][0] = w2;
    c.g[A2][0] = w1;
    c.g[T1][0] = carry;

    if counter == 0 {
        // The counter wrapped: increment words 4, 3, 2, 1 while they wrap.
        let mut wrapped = true;
        for i in [4u32, 3, 2, 1] {
            c.cycles += 4;
            let v = add(w(i).lw(c), 1);
            w(i).sw(c, v);
            c.g[V0][0] = v;
            if v != 0 {
                wrapped = false;
                break;
            }
        }
        if wrapped {
            c.cycles += 3;
            sum = add(sum, 1);
            w(0).sw(c, sum);
        }
    }
    c.cycles += 2;
    c.g[A3][0] = sum;
    c.g[V0][0] = sum;
    ret(c);
}

// ---------------------------------------------------------------------------
// Emitter_Save (0x370aa8)

/// Emitter description fields saved: rate (+0, float), lifetime (+0x18,
/// float) and the particle count (+0xc, word).
const EMS_RATE: u32 = 0x0;
const EMS_LIFE: u32 = 0x18;
const EMS_COUNT: u32 = 0xc;
/// cStream vtable slot "write(buf, n)" (this-adjust halfword +8, method +0xc).
const EMS_STREAM_WRITE: u32 = 0x8;

/// `void Emitter_Save(EmitterDesc* desc, cStream* s)` (0x370aa8): pack the
/// description's rate, lifetime and count into a 12-byte record on the
/// stack and write it to the stream.
pub fn emitter_save(c: &mut Cpu) {
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let rec = frame.at(c, 0);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0xc;
        let desc = r32(c, A0);
        c.g[V1][0] = Addr(desc.wrapping_add(EMS_COUNT)).lw(c);
        c.lwc1(1, desc.wrapping_add(EMS_RATE));
        c.lwc1(0, desc.wrapping_add(EMS_LIFE));
        c.swc1(1, rec);
        c.swc1(0, rec + 4);
        let count = c.g[V1][0];
        Addr(rec + 8).sw(c, count);
        // s->write(&rec, 12)
        let stream = r32(c, A3);
        let vtable = Addr(stream).lw(c);
        c.g[V0][0] = vtable;
        let adjust = Addr((vtable as u32).wrapping_add(EMS_STREAM_WRITE)).lh(c);
        let method = Addr((vtable as u32).wrapping_add(EMS_STREAM_WRITE + 4)).lw(c);
        c.g[V1][0] = method;
        c.g[A0][0] = sx(stream.wrapping_add(adjust as u32));
        call_ptr(c, method as u32, 0x0037_0ae8)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Range table at DATA (count, entries), cursor at DATA+0x40, entries at
    /// DATA+0x100 with sorted [lo, hi) ranges; x chosen to hit the cursor's
    /// next entry, a later one, or none.
    #[test]
    fn intersect_info_find_key_segment_matches() {
        use crate::difftest::{check_lohi, scramble, Rng, DATA};
        check_lohi(ee::s11::f_00351a80__generated, intersect_info_find_key_segment, 300, &|c, seed| {
            let mut r = Rng(0x351a80 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x0035_1a80;
            let count = (seed % 7) as i32 - 1; // -1..5
            let entries = DATA + 0x100;
            c.bus.write32(DATA, count as u32);
            c.bus.write32(DATA + 4, entries);
            for i in 0..8u32 {
                let lo = i as f32 * 10.0;
                c.bus.write32(entries + i * 0x18 + 0x10, lo.to_bits());
                c.bus.write32(entries + i * 0x18 + 0x14, (lo + 10.0).to_bits());
            }
            let cur = (seed / 7 % 6) as i32 - 1;
            c.bus.write32(DATA + 0x40, cur as u32);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x40);
            c.f[12] = match seed % 5 {
                0 => r.f32(100.0),
                1 => ((cur + 1) as f32 * 10.0 + 5.0).to_bits(),
                2 => ((cur + 1) as f32 * 10.0).to_bits(),
                3 => 1000.0f32.to_bits(),
                _ => r.edge_f32(),
            };
        });
    }

    /// Pad slot table at 0x526250 (port 0..1, slot 0..3) pointing at an
    /// actuator buffer in DATA. Odd seeds use the fake syscalls (DmaStat
    /// "busy", SetDma succeeds), even seeds the machine's SIF (DmaStat -1,
    /// the first SetDma returns id 0 = failure); the debug flag is set on
    /// some seeds so the busy message is printed.
    #[test]
    fn libpad_send_act_dma_matches() {
        use crate::difftest::{check_lohi, fakes, scramble, Rng, DATA};
        check_lohi(ee::s15::f_003ff390__generated, libpad_send_act_dma, 240, &|c, seed| {
            let mut r = if seed % 2 == 1 {
                fakes::machine(c, seed, 0x3ff390, 0)
            } else {
                let mut r = Rng(0x3ff390 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                r
            };
            c.pc = 0x003f_f390;
            let port = seed / 2 % 2;
            let slot = seed / 4 % 4;
            let entry = PAD_SLOTS + port * PAD_PORT_STRIDE + slot * PAD_SLOT_STRIDE;
            let buf = DATA + 0x400;
            c.bus.write32(entry + PAD_ACT_BUF, buf);
            c.bus.write32(entry + PAD_IOP_DEST, 0x0001_0000 + (r.next_u32() & 0xff0));
            let id = if seed % 3 == 0 { 0 } else { r.next_u32() & 0xff };
            c.bus.write32(entry + PAD_DMA_ID, id);
            c.bus.write32(buf, r.next_u32());
            for i in 1..8 {
                c.bus.write32(buf + 4 * i, r.next_u32());
            }
            let debug = if seed % 5 == 0 { 1 } else { 0 };
            c.bus.write32(PAD_DEBUG_BASE + PAD_DEBUG_FLAG, debug);
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
        });
    }

    /// The speech history ring at 0x450b88 filled with keys matching the
    /// request (or not) and small values, so draws hit, miss and wrap the
    /// ring; RNG state at 0x450b70; some seeds pass key -1. The real
    /// SPCH_RandNext runs.
    #[test]
    fn spch_random_no_repeat_matches() {
        use crate::difftest::{check_lohi, scramble, Rng, GAME_GP};
        check_lohi(ee::s14::f_003db5c0__generated, spch_random_no_repeat, 300, &|c, seed| {
            let mut r = Rng(0x3db5c0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x003d_b5c0;
            c.g[GP][0] = GAME_GP as u64;
            for i in 0..6 {
                c.bus.write32(0x0045_0b70 + 4 * i, r.next_u32());
            }
            let range = match seed % 6 {
                0 => r.next_u32() % 4 + 1,
                1 => r.next_u32() % 12,
                2 => r.next_u32() % 100,
                3 => (r.next_u32() % 50) as i32 as u32 ^ 0x8000_0000,
                4 => 2,
                _ => r.next_u32() % 70,
            };
            let key = r.next_u32() & 0xffff;
            c.g[A0][0] = sx(range);
            c.g[A1][0] = match seed % 7 {
                0 => u64::MAX,
                1 => (r.next_u32() as u64) << 32 | key as u64,
                _ => key as u64,
            };
            for i in 0..32 {
                let k = if r.next_u32() % 4 == 0 { r.next_u32() & 0xffff } else { key };
                let v = r.next_u32() % (range.min(64) + 1);
                c.bus.write16(SPCH_HISTORY + 4 * i, v as u16);
                c.bus.write16(SPCH_HISTORY + 4 * i + 2, k as u16);
            }
            c.bus.write32(GAME_GP + SPCH_HISTORY_POS, r.next_u32() % 32);
        });
    }

    /// WScriptMan_FocusEnter: no controller (object type 3 or not, with or
    /// without a slot-1 script) or a controller whose v20 is a fake
    /// (non-zero: v21 is called) or the real zero leaf 0x1406c0. The real
    /// HasObjScript1/RunObjScript1 and script cache lookup run; as in the
    /// FocusLeave test the "has script" entry points at `man + 0x290`, which
    /// RunObjScript1 overwrites with the object (whose function is -1), so
    /// the script call itself is skipped.
    #[test]
    fn wscript_man_focus_enter_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA};
        use crate::game::wscript::wscriptman::{OC264_WCTL_V20, OC264_WCTL_VTABLE, OC264_WOBJ_CTL};
        const MAN: u32 = DATA;
        const TABLE: u32 = DATA + 0x400;
        const CACHE: u32 = DATA + 0xc00;
        const SECTION: u32 = DATA + 0x1200;
        const NO_SCRIPT: u32 = DATA + 0x1400;
        const OBJ: u32 = DATA + 0x800;
        const CTL: u32 = DATA + 0xa00;
        const VTABLE: u32 = DATA + 0x1800;
        const FUNCS: u32 = DATA + 0x1_0000;
        const ZERO_LEAF: u32 = 0x0014_06c0;
        // Function entry `id` of slot 1.
        let func_at = |id: u32| FUNCS + (id >> 8) * 0x18 + 4;
        check(ee::s10::f_0030a3a0__generated, wscript_man_focus_enter, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x30_a3a0, 2);
            c.pc = 0x0030_a3a0;
            c.g[A0][0] = sx(MAN);
            c.g[A1][0] = sx(OBJ);
            // Script cache: man -> table -> cache (type table) -> section.
            c.bus.write32(MAN + 0x28c, TABLE);
            c.bus.write32(TABLE + 4, DATA + 0x600);
            c.bus.write32(DATA + 0x604, CACHE);
            c.bus.write32(CACHE + 8, SECTION);
            c.bus.write32(SECTION + 0x1c, FUNCS);
            c.bus.write32(NO_SCRIPT + 8, u32::MAX);
            c.bus.write32(MAN + 0x290, 0x100);
            c.bus.write32(func_at(0x100), r.next_u32() & 0x7fff_ffff);
            c.bus.write32(func_at(OBJ), u32::MAX);
            c.bus.write8(OBJ + 0x78, 0);
            let ty = if seed % 4 == 1 { r.next_u32() % 8 } else { 3 };
            c.bus.write16(CACHE, ty as u16);
            let has_script = seed % 2 == 0;
            let entry = if has_script { MAN + 0x290 - 8 } else { NO_SCRIPT };
            c.bus.write32(OBJ + 0x88, entry);
            // Controller.
            let has_ctl = seed % 6 >= 3;
            c.bus.write32(OBJ + OC264_WOBJ_CTL, if has_ctl { CTL } else { 0 });
            c.bus.write32(CTL + OC264_WCTL_VTABLE, VTABLE);
            let v20 = if seed % 3 == 0 { FAKE } else { ZERO_LEAF };
            for (slot, f) in [(OC264_WCTL_V20, v20), (WSF_CTL_V21, FAKE + 0x10)] {
                c.bus.write16(VTABLE + slot, (r.next_u32() % 0x20) as u16);
                c.bus.write32(VTABLE + slot + 4, f);
            }
        });
    }

    /// scePadSetActDirect: pad slot (port 0..1, slot 0..3) with a DMA
    /// buffer whose current half reports state 1 (stable) or not, and an
    /// actuator packet; the real libpad_GetDmaBuffer and libpad_SendActDma
    /// run, the SIF syscalls faked on odd seeds (send succeeds) and real on
    /// even ones (first SetDma returns id 0: failure).
    #[test]
    fn sce_pad_set_act_direct_matches() {
        use crate::difftest::{check_lohi, fakes, scramble, Rng, DATA};
        check_lohi(ee::s15::f_00400190__generated, sce_pad_set_act_direct, 240, &|c, seed| {
            let mut r = if seed % 2 == 1 {
                fakes::machine(c, seed, 0x400190, 0)
            } else {
                let mut r = Rng(0x400190 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                r
            };
            c.pc = 0x0040_0190;
            let port = seed / 2 % 2;
            let slot = seed / 4 % 4;
            let entry = PAD_SLOTS + port * PAD_PORT_STRIDE + slot * PAD_SLOT_STRIDE;
            let dma = DATA + 0x1000;
            let act = DATA + 0x400;
            let data = DATA + 0x300;
            c.bus.write32(entry, dma);
            c.bus.write32(entry + PAD_ACT_BUF, act);
            c.bus.write32(entry + PAD_IOP_DEST, 0x0001_0000 + (r.next_u32() & 0xff0));
            c.bus.write32(entry + PAD_DMA_ID, if seed % 3 == 0 { 0 } else { r.next_u32() & 0xff });
            // Which half is current: +0x58 < +0xd8 selects the second.
            c.bus.write32(dma + 0x58, r.next_u32() % 4);
            c.bus.write32(dma + 0xd8, r.next_u32() % 4);
            let state = |r: &mut Rng| if seed % 5 == 0 { r.next_u32() % 8 } else { 1 };
            let s0 = state(&mut r);
            let s1 = state(&mut r);
            c.bus.write8(dma + 0x72, s0 as u8);
            c.bus.write8(dma + 0xf2, s1 as u8);
            for i in 0..8 {
                c.bus.write32(act + 4 * i, r.next_u32());
                c.bus.write32(data + 4 * i, r.next_u32());
            }
            c.bus.write32(PAD_DEBUG_BASE + PAD_DEBUG_FLAG, 0);
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
            c.g[A2][0] = sx(data);
        });
    }

    /// RiderAnim_SaveState: the stream's write is a fake (some calls
    /// unwind); the real AnimSeqList_Save runs on six lists with no items
    /// (count 0 or negative), so it only writes its count through the fake.
    #[test]
    fn rider_anim_save_state_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = DATA + 0x20_0100;
        const LISTS: u32 = DATA + 0x20_0200;
        covered(ee::s00::f_00104d68__generated, rider_anim_save_state, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x10_4d68, 7);
            c.pc = 0x0010_4d68;
            let vt = vtable(c, &mut r, &[RAS_STREAM_WRITE]);
            c.bus.write32(STREAM, vt);
            c.bus.write32(SELF + RAS_SEQ_LISTS, LISTS);
            for i in 0..RAS_SEQ_LIST_COUNT {
                let count = if r.next_u32() % 3 == 0 { -(r.next_u32() as i32 % 4) } else { 0 };
                c.bus.write32(LISTS + 8 * i, count as u32);
                c.bus.write32(LISTS + 8 * i + 4, r.next_u32());
            }
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(STREAM);
        });
    }

    /// ModObj_v49: block / implementation present or not; the bounds
    /// getter (fake 0) returns an old box whose corners equal the node's
    /// new bounds, so the real `SpatialHash_Update` (after the real
    /// `World_GetNash`) finds the keys unchanged; the setter is fake 1.
    #[test]
    fn mod_obj_v49_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{put_vec, DATA, GAME_GP};
        const ONE: u32 = 0x3f80_0000;
        covered(ee::s11::f_003567e0__generated, mod_obj_v49, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x3567_e000, 2);
            c.pc = 0x0035_67e0;
            let this = DATA + 0x20_0000;
            let (block, imp, imp_vt, node) = (this + 0x100, this + 0x200, this + 0x300, this + 0x500);
            let (app, game, old) = (this + 0x600, this + 0x700, this + 0x1000);
            c.g[A0][0] = sx(this);
            c.bus.write32(this + MOV49_NODE, node);
            c.bus.write32(this + MOV49_BLOCK, if seed % 6 == 0 { 0 } else { block });
            c.bus.write32(block, if seed % 6 == 1 { 0 } else { imp });
            c.bus.write32(imp, imp_vt);
            for (i, off) in [0x60u32, 0x68].into_iter().enumerate() {
                c.bus.write16(imp_vt + off, (r.next_u32() % 0x20 & !3) as u16);
                c.bus.write32(imp_vt + off + 4, FAKE + 0x10 * i as u32);
            }
            // World_GetNash: *(gp - 0x848) -> +0x84 -> +0x20.
            c.bus.write32(GAME_GP - 0x848, app);
            c.bus.write32(app + 0x84, game);
            c.bus.write32(game + 0x20, r.next_u32());
            // Spatial key grid constants.
            c.bus.write32(GAME_GP - 0x2df8, 1.0f32.to_bits());
            c.bus.write32(GAME_GP - 0x2df4, 0.25f32.to_bits());
            let lo = [r.f32(1000.0), r.f32(1000.0), r.f32(1000.0)];
            let hi = lo.map(|v| (f32::from_bits(v) + (r.next_u32() % 100) as f32).to_bits());
            put_vec(c, old, [lo[0], lo[1], lo[2], ONE]);
            put_vec(c, old + 0x10, [hi[0], hi[1], hi[2], r.next_u32()]);
            for i in 0..3 {
                c.bus.write32(node + MOV49_NODE_MIN + 4 * i, lo[i as usize]);
                c.bus.write32(node + MOV49_NODE_MAX + 4 * i, hi[i as usize]);
            }
            c.bus.write32(LOG_RESULTS, old);
        });
    }

    /// SPCH_RandNext on random RNG states, with words near the top so the
    /// carries happen, and the counter at -1 (wraps) with some of words
    /// 4..1 at 0 (they become -1 after the sum) so each step of the
    /// increment chain runs.
    #[test]
    fn spch_rand_next_matches() {
        use crate::difftest::{check, scramble, Rng};
        check(ee::s14::f_003db4d0__generated, spch_rand_next, 400, &|c, seed| {
            let mut r = Rng(0x3db4d0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x003d_b4d0;
            let wrap = seed % 3 == 0;
            let chain = seed / 3 % 6; // how many of words 4..1 are -1
            for i in 0..6u32 {
                let v = match r.next_u32() % 3 {
                    0 => r.next_u32(),
                    1 => 0xffff_ffff - (r.next_u32() % 4),
                    _ => 0x8000_0000 ^ r.next_u32() % 16,
                };
                c.bus.write32(SPCH_RNG + 4 * i, v);
            }
            if wrap {
                c.bus.write32(SPCH_RNG + 0x14, u32::MAX);
                for k in 0..chain.min(4) {
                    c.bus.write32(SPCH_RNG + 4 * (4 - k), 0);
                }
            }
        });
    }

    /// Emitter_Save: random description fields; the stream's write is a
    /// fake (it logs the record pointer and size; some calls unwind).
    #[test]
    fn emitter_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        const DESC: u32 = DATA + 0x20_0000;
        const STREAM: u32 = DATA + 0x20_0100;
        covered(ee::s12::f_00370aa8__generated, emitter_save, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x37_0aa8, 1);
            c.pc = 0x0037_0aa8;
            let vt = vtable(c, &mut r, &[EMS_STREAM_WRITE]);
            c.bus.write32(STREAM, vt);
            c.bus.write32(DESC + EMS_RATE, r.edge_f32());
            c.bus.write32(DESC + EMS_LIFE, r.edge_f32());
            c.bus.write32(DESC + EMS_COUNT, r.next_u32());
            c.g[A0][0] = sx(DESC);
            c.g[A1][0] = sx(STREAM);
        });
    }
}
