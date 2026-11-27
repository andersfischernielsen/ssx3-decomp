//! Staging module oc1114 (wave wave269): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1114`.

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
    (
        0x003d_7038,
        spch_bank_clear_usage_bits,
        "SPCH_BankClearUsageBits",
    ),
    (0x003d_7b98, spch_flush_queue, "SPCH_FlushQueue"),
    (
        0x003d_80b0,
        spch_service_channel_direct,
        "SPCH_ServiceChannelDirect",
    ),
    (0x003d_8170, spch_dequeue_channel0, "SPCH_DequeueChannel0"),
    (0x003d_8190, spch_service_channel0, "SPCH_ServiceChannel0"),
    (0x003d_82b0, spch_find_bank_field14, "SPCH_FindBankField14"),
    (
        0x003d_8598,
        spch_call_ready_callback,
        "SPCH_CallReadyCallback",
    ),
    (0x003d_a5c8, spch_resolve_bank_ref, "SPCH_ResolveBankRef"),
];

/// Channel records of the speech scheduler (`0x450000 - 8`): 16 records of
/// `0x60` bytes at `+0`, the 8 channel words after them at `+0x600`.
const SPCH_CHANNEL_TABLE: u32 = 0x0044_fff8;
/// The bank slots (`0x450000 - 0x48`): 8 slots of two words, the first
/// holding the bank data pointer.
const SPCH_BANK_SLOT_TABLE: u32 = 0x0044_ffb8;
/// The ready callback pointer (`0x450000 - 0x64`).
const SPCH_READY_CALLBACK_SLOT: u32 = 0x0044_ff9c;
/// The scheduler's re-entry flag, `gp + 0x175c`.
const SPCH_BUSY_GP_OFFSET: u32 = 0x175c;

/// `SPCH_BankClearUsageBits(node)` (0x3d7038): clears the usage flags of a
/// bank. `node[4] & 0xf` is the field count `n` and `node[5]` the row count.
/// The flag bytes follow the row table, `(rows * (n + 2) + 0xf) & 0x3ffc`
/// bytes past `node`, plus `n * 4`. When the first flag byte is set, the
/// `(rows + 7) / 8` bytes after it (indices 1..=count) are zeroed.
pub fn spch_bank_clear_usage_bits(c: &mut Cpu) {
    c.cycles += 13;
    let node = c.g[A0][0] as u32;
    c.g[V1][0] = c.lbu(node.wrapping_add(0x4));
    c.g[A1][0] = c.lbu(node.wrapping_add(0x5));
    c.g[V1][0] &= 0xf;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
    c.mult(0, 2, 5, 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
    c.g[V0][0] &= 0x3ffc;
    c.g[A0][0] = sx(node.wrapping_add(c.g[V0][0] as u32));
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let first_flag_addr = c.g[A2][0] as u32;
    c.g[V0][0] = c.lbu(first_flag_addr);
    let first_clear = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7));
    if !first_clear {
        c.cycles += 7;
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe));
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.g[V0][0];
        }
        c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 3) as u32);
        let no_rows = c.g[A0][0] == 0;
        c.g[A1][0] = 1;
        if !no_rows {
            loop {
                c.cycles += 7;
                c.g[V1][0] = sx(first_flag_addr.wrapping_add(c.g[A1][0] as u32));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] == 0;
                let flag = c.g[V1][0] as u32;
                c.sb(flag, 0u64);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_7088);
                }
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `SPCH_FlushQueue()` (0x3d7b98): clears the 16 queue records (their flag
/// byte, and the count word of their slot), then the 8 channel words, and
/// resets the channel events.
pub fn spch_flush_queue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x450000;
        c.g[A0][0] = sx(SPCH_CHANNEL_TABLE);
        frame.save(c, RA, 0);
        c.g[A2][0] = c.g[A0][0];
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
        c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x6a));
        loop {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
            } else {
                c.cycles += 8;
                let rec = (c.g[A0][0] as u32).wrapping_add(0x6a);
                c.g[V1][0] = c.lbu(rec.wrapping_add(1));
                c.sb(rec, 0u64);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                let count_addr = c.g[V1][0] as u32;
                c.g[V0][0] = c.lw(count_addr);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                let count = c.g[V0][0];
                c.sw(count_addr, count);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x6a));
                if event_due(c) {
                    deliver_event_here(c, 0x003d_7bb8);
                }
                continue;
            }
            c.cycles += 5;
            c.g[V0][0] = 0x450000;
            c.g[A1][0] = u64::MAX;
            c.g[V1][0] = sx(SPCH_CHANNEL_TABLE);
            c.g[A0][0] = 0x7;
            loop {
                c.cycles += 4;
                let word = c.g[V1][0] as u32;
                c.g[V0][0] = c.lw(word);
                let clear = c.g[A1][0];
                c.sw(word.wrapping_add(0x40), clear);
                let empty = c.g[V0][0] == 0;
                c.sw(word.wrapping_add(0x20), clear);
                if !empty {
                    c.cycles += 1;
                    c.sw(word, 0u64);
                }
                c.cycles += 3;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                let more = (c.g[A0][0] as i64) >= 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_7c00);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 5;
            Callee {
                addr: 0x003d_69a8,
                f: ee::s14::f_003d69a8,
            }
            .call(c, 0x003d_7c28)?;
            frame.restore(c, RA, 0);
            frame.close(c);
            return Ok(());
        }
    });
}

/// `SPCH_ServiceChannelDirect(x)` (0x3d80b0): services channel `x` (0..15)
/// for the speech scheduler. Unless the scheduler is already busy (the flag
/// at `gp + 0x175c`), and the channel's two words are valid (below 0x10),
/// it resets the sentence, builds one from the channel's bank word, frees
/// the queue slot, and plays the sentence if one was built. The flag is
/// cleared on the way out.
pub fn spch_service_channel_direct(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        let busy_flag = (c.g[GP][0] as u32).wrapping_add(SPCH_BUSY_GP_OFFSET);
        c.g[V0][0] = c.lw(busy_flag);
        frame.save(c, S2, 0x20);
        frame.save(c, RA, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        let busy = c.g[V0][0] != 0;
        frame.save(c, S0, 0);
        let exit_8138 = if busy {
            true
        } else {
            c.cycles += 15;
            c.g[A2][0] = sx((c.g[S2][0] as u32) << 2);
            c.g[V0][0] = sx(SPCH_CHANNEL_TABLE);
            c.g[A0][0] = 0x1;
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V0][0] = c.g[A1][0];
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
            c.g[V1][0] = (c.g[V1][0] < 0x10u64) as u64;
            let flag_value = c.g[A0][0];
            c.sw(busy_flag, flag_value);
            if c.g[V1][0] == 0 {
                c.g[A0][0] = 0;
            }
            c.g[V0][0] = (c.g[S1][0] < 0x10u64) as u64;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = 0;
            }
            let idle = c.g[A0][0] == 0;
            if idle {
                frame.restore(c, RA, 0x30);
                false
            } else {
                c.cycles += 9;
                Callee {
                    addr: 0x003d_9a40,
                    f: ee::s14::f_003d9a40,
                }
                .call(c, 0x003d_8114)?;
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x003d_a418,
                    f: ee::s14::f_003da418,
                }
                .call(c, 0x003d_811c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x003d_7c38,
                    f: ee::s14::f_003d7c38,
                }
                .call(c, 0x003d_8128)?;
                let none_built = c.g[S0][0] == 0;
                frame.restore(c, RA, 0x30);
                if none_built {
                    false
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x003d_9bd8,
                        f: ee::s14::f_003d9bd8,
                    }
                    .call(c, 0x003d_8138)?;
                    true
                }
            }
        };
        if exit_8138 {
            c.cycles += 1;
            frame.restore(c, RA, 0x30);
        }
        c.cycles += 6;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0);
        c.sw(busy_flag, 0u64);
        frame.close(c);
        Ok(())
    });
}

/// `SPCH_DequeueChannel0()` (0x3d8170): `SPCH_DequeueChannel(0)`.
pub fn spch_dequeue_channel0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save(c, RA, 0);
        c.g[A0][0] = 0;
        Callee {
            addr: 0x003d_7ec8,
            f: ee::s14::f_003d7ec8,
        }
        .call(c, 0x003d_8180)?;
        frame.restore(c, RA, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SPCH_ServiceChannel0()` (0x3d8190): the channel-0 service pass. Unless
/// the scheduler is busy (flag at `gp + 0x175c`), it sets the flag, and
/// plays the pending sentence if there is one; otherwise it dequeues
/// channel 0 and plays what it dequeued. The flag is cleared on the way out.
pub fn spch_service_channel0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let busy_flag = (c.g[GP][0] as u32).wrapping_add(SPCH_BUSY_GP_OFFSET);
        c.g[V0][0] = c.lw(busy_flag);
        let busy = c.g[V0][0] != 0;
        frame.save(c, RA, 0);
        let exit_81e0 = if busy {
            true
        } else {
            c.cycles += 6;
            c.g[V0][0] = 0x1;
            c.g[A0][0] = 0;
            c.sw(busy_flag, 0x1);
            Callee {
                addr: 0x003d_a3f0,
                f: ee::s14::f_003da3f0,
            }
            .call(c, 0x003d_81b0)?;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = 0;
                Callee {
                    addr: 0x003d_9bd8,
                    f: ee::s14::f_003d9bd8,
                }
                .call(c, 0x003d_81c0)?;
                frame.restore(c, RA, 0);
                false
            } else {
                c.cycles += 4;
                c.g[A0][0] = 0;
                Callee {
                    addr: 0x003d_7ec8,
                    f: ee::s14::f_003d7ec8,
                }
                .call(c, 0x003d_81d0)?;
                let none = c.g[V0][0] == 0;
                frame.restore(c, RA, 0);
                if none {
                    false
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = 0;
                    Callee {
                        addr: 0x003d_9bd8,
                        f: ee::s14::f_003d9bd8,
                    }
                    .call(c, 0x003d_81e0)?;
                    true
                }
            }
        };
        if exit_81e0 {
            c.cycles += 1;
            frame.restore(c, RA, 0);
        }
        c.cycles += 3;
        c.sw(busy_flag, 0u64);
        frame.close(c);
        Ok(())
    });
}

/// `SPCH_FindBankField14(key)` (0x3d82b0): the first bank slot (of 8) whose
/// bytes 8 and 9 match the key's bytes 3 and 2; returns the halfword at +0x14
/// of that bank's data, or 0 when no slot matches.
pub fn spch_find_bank_field14(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x450000;
    c.g[A2][0] = sx(SPCH_BANK_SLOT_TABLE);
    c.g[T2][0] = 0;
    c.g[T1][0] = c.g[SP][0];
    c.g[T0][0] = 0;
    c.g[A3][0] = 0;
    let key = c.g[A0][0] as u32;
    loop {
        c.cycles += 3;
        c.g[A1][0] = c.lw(c.g[A2][0] as u32);
        let matched = if c.g[A1][0] == 0 {
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            false
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x9));
            c.g[V0][0] = c.lbu(key.wrapping_add(0x2));
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                false
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lbu(key.wrapping_add(0x3));
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    false
                } else {
                    c.cycles += 3;
                    let result_slot = c.g[T1][0] as u32;
                    let slot = c.g[A2][0];
                    c.sw(result_slot, slot);
                    c.g[T0][0] = 1;
                    true
                }
            }
        };
        if matched {
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A3][0] as i64) < 8) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003d_82d0);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    let not_found = c.g[T0][0] == 0;
    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
    if !not_found {
        c.cycles += 2;
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[T2][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x14));
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[T2][0];
    frame.close(c);
}

/// `SPCH_CallReadyCallback()` (0x3d8598): calls the ready callback when one
/// is registered and returns its result; otherwise returns 1.
pub fn spch_call_ready_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[V0][0] = 0x450000;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.lw(SPCH_READY_CALLBACK_SLOT);
        frame.save(c, RA, 0);
        let none = c.g[V1][0] == 0;
        c.g[V0][0] = 1;
        if !none {
            c.cycles += 2;
            let target = c.g[V1][0] as u32;
            call_ptr(c, target, 0x003d_85b8)?;
        }
        c.cycles += 3;
        frame.restore(c, RA, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SPCH_ResolveBankRef(ref, table, out)` (0x3da5c8): resolves a sentence
/// reference to a bank. `ref` holds the id (halfword), the table index
/// (byte 2) and the kind (byte 3): kind 0 looks the id up; kind 1 looks it
/// up in `table` at the index (only below 20); kind 2 looks the id up and
/// checks that the bank has the sample named by `table[index]`. Writes the
/// bank (halfword at `out`) and the sample (halfword at `out + 2`), and
/// returns whether the bank was found (1) or not (0).
pub fn spch_resolve_bank_ref(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        c.g[A3][0] = u64::MAX;
        frame.save(c, S2, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S1, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, RA, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0);
        let reference = c.g[A0][0] as u32;
        c.g[S0][0] = c.lbu(reference.wrapping_add(0x2));
        c.g[A2][0] = c.lhu(reference);
        c.g[V1][0] = c.lbu(reference.wrapping_add(0x3));
        let out = c.g[S1][0] as u32;
        c.sh(out.wrapping_add(0x2), c.g[A3][0]);
        let kind = c.g[V1][0];
        c.sh(out, c.g[A3][0]);
        // Every path that wrote out[0] ends at the shared exit below (+1 and
        // the reload of out[0] into v1); the others reload it themselves.
        let exit_68c = if kind == 1 {
            c.cycles += 3;
            let index_ok = c.g[S0][0] < 0x14u64;
            c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
            if index_ok {
                c.cycles += 6;
                c.g[A0][0] = c.g[A2][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A1][0] = c.lhu(c.g[V0][0] as u32);
                Callee {
                    addr: 0x003d_6db8,
                    f: ee::s14::f_003d6db8,
                }
                .call(c, 0x003d_a650)?;
                c.sh(out, c.g[V0][0]);
            } else {
                c.cycles += 2;
                c.sh(out, c.g[A3][0]);
            }
            true
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x2;
            if kind == 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[A2][0];
                Callee {
                    addr: 0x003d_6d50,
                    f: ee::s14::f_003d6d50,
                }
                .call(c, 0x003d_a624)?;
                c.sh(out, c.g[V0][0]);
                true
            } else {
                c.cycles += 2;
                if kind == 2 {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[A2][0];
                    Callee {
                        addr: 0x003d_6d50,
                        f: ee::s14::f_003d6d50,
                    }
                    .call(c, 0x003d_a660)?;
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x003d_6cf8,
                        f: ee::s14::f_003d6cf8,
                    }
                    .call(c, 0x003d_a678)?;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = u64::MAX;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.lhu(c.g[S2][0] as u32);
                        c.sh(out.wrapping_add(0x2), c.g[V0][0]);
                    }
                    c.cycles += 1;
                    c.sh(out, c.g[S0][0]);
                    true
                } else {
                    c.cycles += 2;
                    c.g[V1][0] = c.lh(out);
                    false
                }
            }
        };
        if exit_68c {
            c.cycles += 1;
            c.g[V1][0] = c.lh(out);
        }
        c.cycles += 8;
        c.g[V0][0] = u64::MAX;
        frame.restore(c, RA, 0x30);
        frame.restore(c, S2, 0x20);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of the callee stand-ins, in the machine's RAM (its own
    /// region, away from the fakes' log at 0x900000).
    const BOX: u32 = DATA + 0x20_0000;
    const MAGIC: u32 = 0x1114_c0de;

    /// A stand-in for an SPCH callee, gated on [`BOX`]: with the gate on it
    /// logs the call (the register dance is compared) and returns word `k` of
    /// the result table; with the gate off it runs the body it replaced.
    macro_rules! stand_in {
        ($name:ident, $target:expr, $k:expr, $generated:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(BOX) != MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
                if c.pc == ra {
                    let w = c.bus.read32(BOX + 0x10 + 4 * $k);
                    c.g[V0][0] = sx(w);
                }
            }
        };
    }

    stand_in!(
        oc1114_reset_channel_events,
        0x003d_69a8,
        0,
        ee::s14::f_003d69a8__generated
    );
    stand_in!(
        oc1114_reset_sentence,
        0x003d_9a40,
        1,
        ee::s14::f_003d9a40__generated
    );
    stand_in!(
        oc1114_build_sentence,
        0x003d_a418,
        2,
        ee::s14::f_003da418__generated
    );
    stand_in!(
        oc1114_free_queue_slot,
        0x003d_7c38,
        3,
        ee::s14::f_003d7c38__generated
    );
    stand_in!(
        oc1114_play_sentence,
        0x003d_9bd8,
        4,
        ee::s14::f_003d9bd8__generated
    );
    stand_in!(
        oc1114_has_pending,
        0x003d_a3f0,
        5,
        ee::s14::f_003da3f0__generated
    );
    stand_in!(
        oc1114_dequeue_channel,
        0x003d_7ec8,
        6,
        ee::s14::f_003d7ec8__generated
    );
    stand_in!(
        oc1114_find_bank_by_id,
        0x003d_6d50,
        7,
        ee::s14::f_003d6d50__generated
    );
    stand_in!(
        oc1114_find_bank,
        0x003d_6db8,
        8,
        ee::s14::f_003d6db8__generated
    );
    stand_in!(
        oc1114_bank_has_sample,
        0x003d_6cf8,
        9,
        ee::s14::f_003d6cf8__generated
    );

    /// Install the stand-ins in their override slots (once per process).
    fn hook_stand_ins() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D69A8),
            oc1114_reset_channel_events,
            fakes::prev_cell(oc1114_reset_channel_events),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D9A40),
            oc1114_reset_sentence,
            fakes::prev_cell(oc1114_reset_sentence),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DA418),
            oc1114_build_sentence,
            fakes::prev_cell(oc1114_build_sentence),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D7C38),
            oc1114_free_queue_slot,
            fakes::prev_cell(oc1114_free_queue_slot),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D9BD8),
            oc1114_play_sentence,
            fakes::prev_cell(oc1114_play_sentence),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DA3F0),
            oc1114_has_pending,
            fakes::prev_cell(oc1114_has_pending),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D7EC8),
            oc1114_dequeue_channel,
            fakes::prev_cell(oc1114_dequeue_channel),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D6D50),
            oc1114_find_bank_by_id,
            fakes::prev_cell(oc1114_find_bank_by_id),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D6DB8),
            oc1114_find_bank,
            fakes::prev_cell(oc1114_find_bank),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D6CF8),
            oc1114_bank_has_sample,
            fakes::prev_cell(oc1114_bank_has_sample),
        );
    }

    /// Gate on, with the stand-in result words (word k for stand-in k).
    fn gate_on(c: &mut Cpu, words: &[u32; 10]) {
        c.bus.write32(BOX, MAGIC);
        for (k, w) in words.iter().enumerate() {
            c.bus.write32(BOX + 0x10 + 4 * k as u32, *w);
        }
    }

    /// Stand-in results for one seed: the flags (pending, dequeue, sample,
    /// build) and the bank indices vary by seed, the rest is random.
    fn stand_in_words(seed: u32, r: &mut Rng) -> [u32; 10] {
        [
            r.next_u32(),
            r.next_u32(),
            if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
            r.next_u32(),
            r.next_u32(),
            seed / 4 % 2,
            seed / 8 % 2,
            if seed % 3 == 0 {
                u32::MAX
            } else {
                r.next_u32() % 20
            },
            if seed % 3 == 1 {
                u32::MAX
            } else {
                r.next_u32() % 20
            },
            seed / 2 % 2,
        ]
    }

    /// `SPCH_BankClearUsageBits`: field count and row count from the seed, the
    /// first flag byte set on most seeds (cleared on the others).
    #[test]
    fn spch_bank_clear_usage_bits_matches() {
        check(
            ee::s14::f_003d7038__generated,
            spch_bank_clear_usage_bits,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3d7038, 0);
                let node = DATA + 0x1000;
                for i in 0..0x400 {
                    c.bus.write8(node + i, r.next_u32() as u8);
                }
                let n = (r.next_u32() & 0xf) as u8;
                c.bus.write8(node + 4, n | ((r.next_u32() as u8) & 0xf0));
                let rows = (r.next_u32() % 200) as u8;
                c.bus.write8(node + 5, rows);
                let flags = (((rows as u32) * (n as u32 + 2) + 0xf) & 0x3ffc) + (n as u32) * 4;
                let first = if seed % 4 == 0 {
                    0
                } else {
                    1 + (r.next_u32() % 255) as u8
                };
                c.bus.write8(node + flags, first);
                c.g[A0][0] = node as u64;
            },
        );
    }

    /// `SPCH_FlushQueue`: random queue records and channel words, the channel
    /// reset stood in.
    #[test]
    fn spch_flush_queue_matches() {
        hook_stand_ins();
        check(
            ee::s14::f_003d7b98__generated,
            spch_flush_queue,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3d7b98, 0);
                for i in 0..0x800 {
                    c.bus.write8(SPCH_CHANNEL_TABLE + i, r.next_u32() as u8);
                }
                gate_on(c, &stand_in_words(seed, &mut r));
            },
        );
    }

    /// `SPCH_ServiceChannelDirect`: busy and idle scheduler, valid and
    /// invalid channel words (x = 17 reads out of range), every stand-in arm.
    #[test]
    fn spch_service_channel_direct_matches() {
        hook_stand_ins();
        check(
            ee::s14::f_003d80b0__generated,
            spch_service_channel_direct,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3d80b0, 0);
                gate_on(c, &stand_in_words(seed, &mut r));
                c.bus.write32(
                    GAME_GP + SPCH_BUSY_GP_OFFSET,
                    if seed % 4 == 0 { 1 } else { 0 },
                );
                for k in 0..0x30u32 {
                    let v = if r.next_u32() % 3 == 0 {
                        r.next_u32() % 0x20
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(SPCH_CHANNEL_TABLE + 4 * k, v);
                }
                let x = if seed % 5 == 0 { 17 } else { r.next_u32() % 16 };
                c.g[A0][0] = x as u64;
            },
        );
    }

    /// `SPCH_DequeueChannel0`: the one call, with and without a result.
    #[test]
    fn spch_dequeue_channel0_matches() {
        hook_stand_ins();
        check(
            ee::s14::f_003d8170__generated,
            spch_dequeue_channel0,
            100,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3d8170, 0);
                gate_on(c, &stand_in_words(seed, &mut r));
            },
        );
    }

    /// `SPCH_ServiceChannel0`: busy, pending, and the dequeue arms.
    #[test]
    fn spch_service_channel0_matches() {
        hook_stand_ins();
        check(
            ee::s14::f_003d8190__generated,
            spch_service_channel0,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3d8190, 0);
                gate_on(c, &stand_in_words(seed, &mut r));
                c.bus.write32(
                    GAME_GP + SPCH_BUSY_GP_OFFSET,
                    if seed % 4 == 0 { 1 } else { 0 },
                );
            },
        );
    }

    /// `SPCH_FindBankField14`: 8 bank slots, some empty, some matching the
    /// key's two bytes, some not.
    #[test]
    fn spch_find_bank_field14_matches() {
        check(
            ee::s14::f_003d82b0__generated,
            spch_find_bank_field14,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3d82b0, 0);
                let key = DATA + 0x3000;
                let k2 = r.next_u32() as u8;
                let k3 = r.next_u32() as u8;
                c.bus.write8(key + 2, k2);
                c.bus.write8(key + 3, k3);
                for i in 0..8u32 {
                    let buf = DATA + 0x4000 + i * 0x100;
                    let slot = SPCH_BANK_SLOT_TABLE + 8 * i;
                    let p = if (seed + i) % 4 == 0 { 0 } else { buf };
                    c.bus.write32(slot, p);
                    c.bus.write32(slot + 4, r.next_u32());
                    if p != 0 {
                        let b9 = if (seed + i) % 2 == 0 {
                            k2
                        } else {
                            r.next_u32() as u8
                        };
                        let b8 = if (seed + i) % 3 == 0 {
                            k3
                        } else {
                            r.next_u32() as u8
                        };
                        c.bus.write8(buf + 9, b9);
                        c.bus.write8(buf + 8, b8);
                        c.bus.write16(buf + 0x14, r.next_u32() as u16);
                    }
                }
                c.g[A0][0] = key as u64;
            },
        );
    }

    /// `SPCH_CallReadyCallback`: no callback, and a callback that is a fake
    /// (outside the function table, so the miss hook runs it), some unwinding.
    #[test]
    fn spch_call_ready_callback_matches() {
        check(
            ee::s14::f_003d8598__generated,
            spch_call_ready_callback,
            200,
            &|c, seed| {
                fakes::machine(c, seed, 0x3d8598, 1);
                c.bus.write32(
                    SPCH_READY_CALLBACK_SLOT,
                    if seed % 3 == 0 { 0 } else { fakes::FAKE },
                );
            },
        );
    }

    /// `SPCH_ResolveBankRef`: all three kinds, the index inside and outside
    /// the table, the stand-ins' found and not-found results.
    #[test]
    fn spch_resolve_bank_ref_matches() {
        hook_stand_ins();
        check(
            ee::s14::f_003da5c8__generated,
            spch_resolve_bank_ref,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3da5c8, 0);
                gate_on(c, &stand_in_words(seed, &mut r));
                let reference = DATA + 0x2000;
                let table = DATA + 0x2100;
                let out = DATA + 0x2200;
                c.bus.write16(reference, r.next_u32() as u16);
                c.bus.write8(reference + 2, (r.next_u32() % 24) as u8);
                c.bus.write8(reference + 3, (seed % 4) as u8);
                for k in 0..24u32 {
                    c.bus.write32(table + 4 * k, r.next_u32());
                }
                c.bus.write16(out, r.next_u32() as u16);
                c.bus.write16(out + 2, r.next_u32() as u16);
                c.g[A0][0] = reference as u64;
                c.g[A1][0] = table as u64;
                c.g[A2][0] = out as u64;
            },
        );
    }
}
