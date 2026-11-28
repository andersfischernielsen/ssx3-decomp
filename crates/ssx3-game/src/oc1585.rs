//! Staging module oc1585 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1585`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_9d00, snd_midi_pause, "SNDMIDI_Pause"),
    (0x003b_d078, snd_enc_lfe_encode, "SNDENC_EncodeLfe"),
    (0x003b_eca0, snd_enc_buffer_sizes, "func_003BECA0"),
    (0x003c_0b10, snd_iop_call_rpc, "SND_IopCallRpc"),
    (0x003c_555c, snd_store_const_555c, "lbl_003C555C"),
    (0x003c_556c, snd_store_const_556c, "lbl_003C556C"),
    (0x003c_557c, snd_store_const_557c, "lbl_003C557C"),
    (0x003c_558c, snd_store_const_558c, "lbl_003C558C"),
];

/// `void SNDMIDI_Pause(int h)` (0x3b9d00, `SNDseqpause` in seg_2B4578.c):
/// under `SND_Lock`, finds the MIDI slot of handle `h`. An invalid slot is
/// unlocked again and returned. A valid one stops its live sounds (the
/// `SND_Stop` of every entry of the slot's table, when the slot record is in
/// state 3), then returns the slot's status under the lock.
pub fn snd_midi_pause(c: &mut Cpu) {
    guest(c, snd_midi_pause_body)
}

fn snd_midi_pause_body(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x50);
    frame.save(c, 19, 0x10);
    frame.save(c, 18, 0x20);
    frame.save(c, 16, 0x40);
    frame.save(c, 17, 0x30);
    frame.save_ra(c, 0);
    c.g[S3][0] = c.g[A0][0];
    call_ptr(c, 0x003b_58a0, 0x003b_9d20)?; // SND_Lock
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x003b_8d98, 0x003b_9d28)?; // SNDMIDI_GetSlot
    c.g[S2][0] = c.g[V0][0];
    let valid = (c.g[S2][0] as i64) >= 0;
    c.g[V1][0] = 0x70;
    if valid {
        snd_midi_pause_voices(c)?;
    } else {
        c.cycles += 4;
        call_ptr(c, 0x003b_58d8, 0x003b_9d3c)?; // SND_Unlock
        c.g[V0][0] = c.g[S2][0];
    }
    c.cycles += 7;
    frame.restore(c, 16, 0x40);
    frame.restore(c, 17, 0x30);
    frame.restore(c, 18, 0x20);
    frame.restore(c, 19, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// The valid-slot part of `SNDMIDI_Pause` (the generated `'b_3b9d98` block),
/// slot in s2: when the slot record (`0x510000 - 0x5ee0 + slot * 0x70`, state
/// byte at +0x28) is in state 3, every live entry of the slot's 23-entry table
/// (at +0x700, 8 bytes each: handle word, slot byte at +4) goes through
/// `SND_Stop`. Then the status is read and the lock released.
fn snd_midi_pause_voices(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    c.g[V0][0] = 0x0051_0000;
    c.mult(0, 3, 18, 3);
    c.g[A1][0] = sx(r32(c, V0).wrapping_sub(0x5ee0));
    c.g[A0][0] = 0x3;
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    c.g[V0][0] = c.lb(r32(c, V1).wrapping_add(0x28));
    let not_state3 = c.g[V0][0] != c.g[A0][0];
    c.g[V0][0] = 0x5;
    if !not_state3 {
        c.cycles += 3;
        c.g[S0][0] = sx(r32(c, A1).wrapping_add(0x700));
        let state = r32(c, V1).wrapping_add(0x28);
        let five = c.g[V0][0];
        c.sb(state, five);
        c.g[S1][0] = 0x17;
        loop {
            c.cycles += 3;
            let entry_slot = r32(c, S0).wrapping_add(0x4);
            c.g[V0][0] = c.lb(entry_slot);
            let same_slot = c.g[V0][0] == c.g[S2][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x1));
            if same_slot {
                c.cycles += 3;
                let entry = r32(c, S0);
                c.g[A0][0] = c.lw(entry);
                if (c.g[A0][0] as i64) >= 0 {
                    c.cycles += 2;
                    call_ptr(c, 0x003b_89e0, 0x003b_9d90)?; // SND_Stop
                }
            }
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x8));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_9d70);
            }
        }
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x003b_9fc8, 0x003b_9da0)?; // SNDMIDI_GetStatus
    c.g[S0][0] = c.g[V0][0];
    call_ptr(c, 0x003b_58d8, 0x003b_9da8)?; // SND_Unlock
    c.g[V0][0] = c.g[S0][0];
    Ok(())
}

/// `void SNDENC_EncodeLfe(int lfeInput, u8* idx, s8 out[8])` (0x3bd078,
/// `func_003BD078` in seg_2B4578.c): decimates the LFE input into eight
/// floats on the frame (the callee at 0x3c06b0), finds the largest magnitude
/// (the negated minimum included), picks the scale index from it (a table
/// lookup for magnitudes below 4.0, otherwise a log-like estimate from the
/// float's bit pattern), stores the index at `*idx` and the eight scaled
/// samples as bytes at `out`. The float compares and conversions follow the
/// generated body op by op.
pub fn snd_enc_lfe_encode(c: &mut Cpu) {
    guest(c, snd_enc_lfe_encode_body)
}

fn snd_enc_lfe_encode_body(c: &mut Cpu) -> Flow {
    c.cycles += 22;
    let frame = Frame::open(c, 0x60);
    c.g[V0][0] = 0x0051_0000;
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_sub(0x5398));
    c.g[A3][0] = 0x0045_0000;
    frame.save(c, 16, 0x40);
    c.g[A3][0] = sx(r32(c, A3).wrapping_sub(0x2300));
    frame.save(c, 17, 0x30);
    c.g[S0][0] = c.g[A2][0];
    frame.save_fpr(c, 21, 0x58);
    c.g[S1][0] = c.g[A1][0];
    frame.save_fpr(c, 20, 0x50);
    c.g[A1][0] = c.g[SP][0];
    frame.save_ra(c, 0x20);
    c.mtc1(0, 21);
    c.g[A2][0] = sx(r32(c, V1).wrapping_add(0x6c));
    call_ptr(c, 0x003c_06b0, 0x003b_d0b8)?; // SNDENC_LfeDecimate
    c.g[A3][0] = 0x0045_0000;
    c.g[V0][0] = c.g[SP][0];
    c.mov_s(20, 21);
    c.g[A1][0] = 0x7;
    c.lwc1(0, r32(c, V0));
    // Eight samples: the running maximum (f21) and minimum (f20).
    loop {
        c.cycles += 5;
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1));
        c.c_lt_s(21, 0);
        let not_less = !c.fcc();
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x4));
        if not_less {
            c.cycles += 4;
            c.c_lt_s(0, 20);
            if c.fcc() {
                c.mov_s(20, 0);
            }
        } else {
            c.cycles += 2;
            c.mov_s(21, 0);
        }
        c.cycles += 2;
        if (c.g[A1][0] as i64) < 0 {
            break;
        }
        c.lwc1(0, r32(c, V0));
        if event_due(c) {
            deliver_event_here(c, 0x003b_d0d0);
        }
    }
    // The negative minimum joins the maximum, then the scale index.
    c.cycles += 5;
    c.neg_s(20, 20);
    c.c_lt_s(21, 20);
    if c.fcc() {
        c.mov_s(21, 20);
    }
    c.cycles += 6;
    c.g[AT][0] = 0x4080_0000;
    c.mtc1(1, 0);
    c.c_lt_s(21, 0);
    let big = !c.fcc(); // max >= 4.0
    c.g[A0][0] = sx(r32(c, A3).wrapping_sub(0x38a0));
    if !big {
        c.cycles += 7;
        c.cvt_w_s(0, 21);
        c.mfc1(3, 0);
        c.g[V0][0] = 0x0045_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x22c8));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A1][0] = c.lb(r32(c, V1));
    } else {
        c.cycles += 19;
        c.mfc1(3, 21);
        c.g[AT][0] = 0x352f_0000;
        c.g[AT][0] = c.g[AT][0] | 0x2507;
        c.mtc1(1, 1);
        c.g[V0][0] = c.g[V1][0];
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(3, 1);
        c.g[A2][0] = sx(r32(c, V1).wrapping_sub(0x2b8));
        c.g[V0][0] = sx(r32(c, A2) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.lwc1(0, r32(c, V0));
        c.cvt_s_w(0, 0);
        c.c_lt_s(21, 0);
        let below = c.fcc();
        c.g[A1][0] = sx(r32(c, V1).wrapping_sub(0x2b7));
        if below {
            c.cycles += 1;
            c.g[A1][0] = c.g[A2][0];
        }
    }
    // The index goes to *idx; the scale is 28.57 / table[index].
    c.cycles += 15;
    let idx_out = r32(c, S1);
    let index = c.g[A1][0];
    c.sb(idx_out, index);
    c.g[V0][0] = sx(r32(c, A1) << 2);
    c.g[V1][0] = sx(r32(c, A3).wrapping_sub(0x38a0));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.g[AT][0] = 0x41e4_0000;
    c.g[AT][0] = c.g[AT][0] | 0x9249;
    c.mtc1(1, 2);
    c.lwc1(1, r32(c, V0));
    c.cvt_s_w(1, 1);
    c.g[A1][0] = 0u64;
    c.g[A2][0] = c.g[SP][0];
    c.div_s(0, 0, 1);
    c.mul_s(1, 0, 2);
    loop {
        c.cycles += 10;
        c.lwc1(0, r32(c, A2));
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A1)));
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1));
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x4));
        c.mul_s(0, 0, 1);
        c.g[V1][0] = ((c.g[A1][0] as i64) < 8i64) as u64;
        c.cvt_w_s(2, 0);
        c.mfc1(2, 2);
        let more = c.g[V1][0] != 0;
        let out_byte = r32(c, A0);
        let sample = c.g[V0][0];
        c.sb(out_byte, sample);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003b_d1d8);
        }
    }
    c.cycles += 7;
    frame.restore(c, 16, 0x40);
    frame.restore(c, 17, 0x30);
    frame.restore_ra(c, 0x20);
    frame.restore_fpr(c, 21, 0x58);
    frame.restore_fpr(c, 20, 0x50);
    frame.close(c);
    Ok(())
}

/// Leaves `v0 = value` and returns (the early exits of
/// `snd_enc_buffer_sizes`, which charge 2 cycles each).
fn snd_enc_buffer_sizes_fail(c: &mut Cpu, value: u64) {
    c.cycles += 2;
    c.g[V0][0] = value;
    ret(c);
}

/// `int SNDENC_BufferSizes(int mode, int quality, int* outStateSize, int* outWorkSize)`
/// (0x3beca0, `func_003BECA0` in seg_2B4578.c): the encoder's buffer sizes.
/// Quality below 0 or from 101 fails with -2; the table entry `w` is 0xe0
/// below 34, 0x160 below 67, else 0x1e0. Mode 1 has 4 channels, mode 2 has 5,
/// other modes fail with -1. Returns 0 and stores the state size
/// `(4w + 0x3c) * channels + 0xa8` and the work size `(2w + 0x200) * 4 + 0xc00`.
pub fn snd_enc_buffer_sizes(c: &mut Cpu) {
    c.cycles += 2;
    let quality_nonneg = (c.g[A1][0] as i64) >= 0;
    c.g[V0][0] = ((c.g[A1][0] as i64) < 34) as u64;
    if !quality_nonneg {
        return snd_enc_buffer_sizes_fail(c, 0xffff_ffff_ffff_fffe);
    }
    // Quality to table entry w (or the -2 exit).
    c.cycles += 2;
    let ge34 = c.g[V0][0] == 0;
    c.g[V0][0] = ((c.g[A1][0] as i64) < 67) as u64;
    if !ge34 {
        c.cycles += 2;
        c.g[A1][0] = 0xe0;
    } else {
        c.cycles += 2;
        let ge67 = c.g[V0][0] == 0;
        c.g[V0][0] = ((c.g[A1][0] as i64) < 101) as u64;
        if !ge67 {
            c.cycles += 2;
            c.g[A1][0] = 0x160;
        } else {
            c.cycles += 2;
            let ge101 = c.g[V0][0] == 0;
            c.g[A1][0] = 0x1e0;
            if ge101 {
                return snd_enc_buffer_sizes_fail(c, 0xffff_ffff_ffff_fffe);
            }
        }
    }
    // Mode to channel count (or the -1 exit).
    c.cycles += 3;
    c.g[V0][0] = 1;
    let not_mode1 = c.g[A0][0] != c.g[V0][0];
    c.g[V0][0] = 2;
    if !not_mode1 {
        c.cycles += 2;
        c.g[V0][0] = 4;
    } else {
        c.cycles += 2;
        let mode2 = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = 5;
        if !mode2 {
            return snd_enc_buffer_sizes_fail(c, 0xffff_ffff_ffff_ffff);
        }
    }
    // Sizes.
    c.cycles += 12;
    c.g[V1][0] = sx(r32(c, A1) << 2);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(0x200));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x3c));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    c.mult(0, 3, 3, 2);
    c.g[A0][0] = sx(r32(c, A0) << 2);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xc00));
    c.g[V0][0] = 0;
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0xa8));
    let state_out = r32(c, A2);
    let state_size = c.g[V1][0];
    c.sw(state_out, state_size);
    let ra = c.g[RA][0] as u32;
    let work_out = r32(c, A3);
    let work_size = c.g[A0][0];
    c.sw(work_out, work_size);
    c.pc = ra;
}

/// `SND_IopCallRpc` (0x3c0b10): a resumable dispatcher (every state of the
/// generated `match pc` is kept, so an unwound call resumes at its state).
/// When a2 is non-zero the 0x510000 - 0x5300 buffer is copied by `SND_MemCopy`.
/// Then `sceSifCheckStatRpc` is polled on the buffer at s0 (the buffer at
/// 0x510000 - 0x4680), and between polls the loop counts to 10000 at
/// `sp + 0x10`. When the poll returns zero, `sceSifCallRpc` runs with the
/// handle (a1 = s3, a2 = s4, a3 = a0 - 0xc80, t0 = s1, and the rest zero).
pub fn snd_iop_call_rpc(c: &mut Cpu) {
    guest(c, snd_iop_call_rpc_body)
}

fn snd_iop_call_rpc_body(c: &mut Cpu) -> Flow {
    let mut pc = c.pc;
    loop {
        match pc {
            0x003c_0b10 => {
                c.cycles += 12;
                let frame = Frame::open(c, 0x80);
                frame.save(c, 19, 0x40);
                c.g[S3][0] = c.g[A0][0];
                frame.save(c, 17, 0x60);
                frame.save(c, 20, 0x30);
                c.g[V0][0] = c.g[S3][0] ^ 0x2;
                frame.save(c, 16, 0x70);
                c.g[S1][0] = c.g[A2][0];
                frame.save(c, 18, 0x50);
                c.g[S4][0] = (c.g[V0][0] < 0x1u64) as u64;
                let no_copy = c.g[S1][0] == 0;
                frame.save_ra(c, 0x20);
                pc = if no_copy { 0x003c_0b4c } else { 0x003c_0b40 };
            }
            0x003c_0b40 => {
                c.cycles += 3;
                c.g[A0][0] = 0x0051_0000;
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x5300));
                Callee {
                    addr: 0x003c_51e8,
                    f: ee::s13::f_003c51e8,
                }
                .call(c, 0x003c_0b4c)?; // SND_MemCopy
                pc = 0x003c_0b4c;
            }
            0x003c_0b4c => {
                c.cycles += 2;
                c.g[S2][0] = 0x0051_0000;
                pc = 0x003c_0b8c;
            }
            0x003c_0b54 => {
                c.cycles += 1;
                pc = 0x003c_0b58;
            }
            0x003c_0b58 => {
                c.cycles += 5;
                let counter = sp(c).wrapping_add(0x10);
                c.sw(counter, 0);
                c.g[V0][0] = c.lw(counter);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 10000i64) as u64;
                let finished = c.g[V0][0] == 0;
                c.g[S0][0] = sx(r32(c, S2).wrapping_sub(0x4680));
                pc = if finished { 0x003c_0b90 } else { 0x003c_0b6c };
            }
            0x003c_0b6c => {
                c.cycles += 1;
                pc = 0x003c_0b70;
            }
            0x003c_0b70 => {
                c.cycles += 7;
                let counter = sp(c).wrapping_add(0x10);
                c.g[V0][0] = c.lw(counter);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x1));
                let next = c.g[V0][0];
                c.sw(counter, next);
                c.g[V1][0] = c.lw(counter);
                c.g[V1][0] = ((c.g[V1][0] as i64) < 10000i64) as u64;
                let again = c.g[V1][0] != 0;
                if again {
                    if back_edge_unwinds(c, 0x003c_0b70) {
                        return Err(Unwound);
                    }
                    pc = 0x003c_0b70;
                } else {
                    pc = 0x003c_0b8c;
                }
            }
            0x003c_0b8c => {
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S2).wrapping_sub(0x4680));
                pc = 0x003c_0b90;
            }
            0x003c_0b90 => {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0042_6f08, 0x003c_0b98)?; // sceSifCheckStatRpc
                let polled_again = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S0][0];
                if polled_again {
                    if back_edge_unwinds(c, 0x003c_0b58) {
                        return Err(Unwound);
                    }
                    pc = 0x003c_0b58;
                } else {
                    pc = 0x003c_0ba0;
                }
            }
            0x003c_0ba0 => {
                c.cycles += 17;
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S4][0];
                c.g[T0][0] = c.g[S1][0];
                let frame = Frame { size: 0x80 };
                let slot = sp(c);
                c.sw(slot, 0);
                c.g[A3][0] = sx(r32(c, A0).wrapping_sub(0xc80));
                c.g[T1][0] = 0;
                c.g[T2][0] = 0;
                c.g[T3][0] = 0;
                call_ptr(c, 0x0042_6d18, 0x003c_0bc4)?; // sceSifCallRpc
                frame.restore(c, 16, 0x70);
                frame.restore(c, 17, 0x60);
                frame.restore(c, 18, 0x50);
                frame.restore(c, 19, 0x40);
                frame.restore(c, 20, 0x30);
                frame.restore_ra(c, 0x20);
                frame.close(c);
                return Ok(());
            }
            _ => {
                c.jump_out(pc, 0x003c_0b10, 0x003c_0be4);
                return Ok(());
            }
        }
    }
}

/// Stores the constant `value` through the pointer in a1 and returns 0. The
/// generated body tail-jumps to 0x3c55f4, which is the `v0 = 0` return block
/// of the dispatcher at 0x3c55dc (two cycles, then `pc = ra`); this port takes
/// that block's effect directly.
fn snd_store_const_and_return_zero(c: &mut Cpu, value: u32) {
    c.cycles += 4;
    c.g[V0][0] = sx(value);
    let dest = r32(c, A1);
    let stored = c.g[V0][0];
    c.sw(dest, stored);
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// Stores the constant 0x4958c8 through a1, returns 0 (0x3c555c).
pub fn snd_store_const_555c(c: &mut Cpu) {
    snd_store_const_and_return_zero(c, 0x0049_58c8)
}

/// Stores the constant 0x4958d0 through a1, returns 0 (0x3c556c).
pub fn snd_store_const_556c(c: &mut Cpu) {
    snd_store_const_and_return_zero(c, 0x0049_58d0)
}

/// Stores the constant 0x4958d8 through a1, returns 0 (0x3c557c).
pub fn snd_store_const_557c(c: &mut Cpu) {
    snd_store_const_and_return_zero(c, 0x0049_58d8)
}

/// Stores the constant 0x4958e0 through a1, returns 0 (0x3c558c).
pub fn snd_store_const_558c(c: &mut Cpu) {
    snd_store_const_and_return_zero(c, 0x0049_58e0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The stack the fixtures run on (not `SP`: that name is the register index).
    const STACK: u32 = crate::difftest::SP;
    /// Gate and mailbox of this module's callee fakes: the magic word at
    /// `OC1585_CTRL`, a mailbox word at `+4`.
    const OC1585_CTRL: u32 = DATA + 0xf_e800;
    const OC1585_MAGIC: u32 = 0x1585_c0de;
    /// The MIDI slot records (0x70 bytes each, at `0x510000 - 0x5ee0`) and the
    /// slot table after them (at +0x700).
    const MIDI_RECORDS: u32 = 0x0050_a120;
    const MIDI_TABLE: u32 = 0x0050_a820;

    /// Gated fakes of the callees (hooked once per process; with the gate off
    /// they run the body they replaced, through the chain in `prev`).
    macro_rules! oc1585_fakes {
        ($($fake:ident, $prev:ident, $addr:literal, $slot:path, $gen:path;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1585_CTRL) != OC1585_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr)
                }
            )*
            fn oc1585_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1585_fakes! {
        oc1585_fake_3b58a0, OC1585_PREV_3B58A0, 0x003b_58a0, ee::s13::OVERRIDE_003B58A0, ee::s13::f_003b58a0__generated;
        oc1585_fake_3b58d8, OC1585_PREV_3B58D8, 0x003b_58d8, ee::s13::OVERRIDE_003B58D8, ee::s13::f_003b58d8__generated;
        oc1585_fake_3b89e0, OC1585_PREV_3B89E0, 0x003b_89e0, ee::s13::OVERRIDE_003B89E0, ee::s13::f_003b89e0__generated;
        oc1585_fake_3b9fc8, OC1585_PREV_3B9FC8, 0x003b_9fc8, ee::s13::OVERRIDE_003B9FC8, ee::s13::f_003b9fc8__generated;
        oc1585_fake_3c51e8, OC1585_PREV_3C51E8, 0x003c_51e8, ee::s13::OVERRIDE_003C51E8, ee::s13::f_003c51e8__generated;
        oc1585_fake_3c06b0, OC1585_PREV_3C06B0, 0x003c_06b0, ee::s13::OVERRIDE_003C06B0, ee::s13::f_003c06b0__generated;
        oc1585_fake_426d18, OC1585_PREV_426D18, 0x0042_6d18, ee::s15::OVERRIDE_00426D18, ee::s15::f_00426d18__generated;
    }

    static OC1585_PREV_3B8D98: AtomicUsize = AtomicUsize::new(0);
    static OC1585_PREV_426F08: AtomicUsize = AtomicUsize::new(0);

    /// `SNDMIDI_GetSlot`: the slot is the mailbox word (all ones: -1).
    fn oc1585_fake_3b8d98(c: &mut Cpu) {
        if c.bus.read32(OC1585_CTRL) != OC1585_MAGIC {
            return fakes::fall_back(c, &OC1585_PREV_3B8D98, ee::s13::f_003b8d98__generated);
        }
        let slot = c.bus.read32(OC1585_CTRL + 4);
        fakes::fake_call(c, 0x003b_8d98);
        c.g[V0][0] = sx(slot);
    }

    /// `sceSifCheckStatRpc`: nonzero for the first `OC1585_CTRL + 4` calls,
    /// then zero.
    fn oc1585_fake_426f08(c: &mut Cpu) {
        if c.bus.read32(OC1585_CTRL) != OC1585_MAGIC {
            return fakes::fall_back(c, &OC1585_PREV_426F08, ee::s15::f_00426f08__generated);
        }
        let left = c.bus.read32(OC1585_CTRL + 4);
        fakes::fake_call(c, 0x0042_6f08);
        c.g[V0][0] = (left > 0) as u64;
        c.bus.write32(OC1585_CTRL + 4, left.saturating_sub(1));
    }

    /// Hooks every fake of this module (each once per process).
    fn oc1585_install() {
        oc1585_install_fakes();
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B8D98),
            oc1585_fake_3b8d98,
            &OC1585_PREV_3B8D98,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00426F08),
            oc1585_fake_426f08,
            &OC1585_PREV_426F08,
        );
    }

    /// `SNDMIDI_Pause` against the generated body: invalid and valid slots,
    /// the state-3 stop loop (live and free entries), and the callees'
    /// unwinds.
    #[test]
    fn snd_midi_pause_matches() {
        oc1585_install();
        fakes::covered(
            ee::s13::f_003b9d00__generated,
            snd_midi_pause,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003b_9d00, 6);
                c.bus.write32(OC1585_CTRL, OC1585_MAGIC);
                // No slot for one seed in five, otherwise slot 0..15.
                let slot = if seed % 5 == 0 { u32::MAX } else { seed % 16 };
                c.bus.write32(OC1585_CTRL + 4, slot);
                if slot < 16 {
                    // State 3 runs the stop loop, other states skip it.
                    let record = MIDI_RECORDS + slot * 0x70;
                    c.bus.write8(record + 0x28, if seed % 3 == 0 { 2 } else { 3 });
                }
                for i in 0..23u32 {
                    let entry = MIDI_TABLE + 8 * i;
                    let handle = if i % 4 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() & 0x7fff
                    };
                    c.bus.write32(entry, handle);
                    let owner = if i % 3 == 0 {
                        slot as u8
                    } else {
                        (0x20 + i) as u8
                    };
                    c.bus.write8(entry + 4, owner);
                }
            },
        );
    }

    /// `SNDENC_EncodeLfe` against the generated body: the decimated samples
    /// come from the frame (eight floats below the stack), small and large
    /// magnitudes (the 4.0 branch), and the decimate callee's unwind.
    #[test]
    fn snd_enc_lfe_encode_matches() {
        oc1585_install();
        fakes::covered(
            ee::s13::f_003bd078__generated,
            snd_enc_lfe_encode,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003b_d078, 1);
                c.bus.write32(OC1585_CTRL, OC1585_MAGIC);
                let range: f32 = if seed % 2 == 0 { 6.0 } else { 2.0 };
                for i in 0..8u32 {
                    c.bus.write32(STACK - 0x60 + 4 * i, r.f32(range));
                }
                c.bus.write32(0x0050_ac68, r.next_u32());
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = sx(DATA + 0x2000);
            },
        );
    }

    /// `snd_enc_buffer_sizes` against the generated body: every quality
    /// range (below 0, below 34, 34..67, 67..101, 101 and up), every mode
    /// (1, 2, and the failing ones).
    #[test]
    fn snd_enc_buffer_sizes_matches() {
        check(
            ee::s13::f_003beca0__generated,
            snd_enc_buffer_sizes,
            300,
            &|c, seed| {
                let mut r = Rng(0x003b_eca0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[A0][0] = (seed % 4) as u64;
                c.g[A1][0] = sx((seed % 140).wrapping_sub(10));
                c.g[A2][0] = sx(DATA + 0x1000);
                c.g[A3][0] = sx(DATA + 0x1008);
            },
        );
    }

    /// `SND_IopCallRpc` against the generated dispatcher: the copy on and
    /// off, the poll loop with zero to two more polls (each one runs the
    /// 10000-count loop), and the callees' unwinds.
    #[test]
    fn snd_iop_call_rpc_matches() {
        oc1585_install();
        fakes::covered(
            ee::s13::f_003c0b10__generated,
            snd_iop_call_rpc,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003c_0b10, 4);
                c.bus.write32(OC1585_CTRL, OC1585_MAGIC);
                c.bus.write32(OC1585_CTRL + 4, seed % 3);
                c.g[A2][0] = if seed % 4 == 0 {
                    0
                } else {
                    (r.next_u32() | 1) as u64
                };
            },
        );
    }

    /// Fixture of the four 16-byte stores: scrambled registers, and a1 at one
    /// of four words of the test data.
    fn stub_setup(c: &mut Cpu, seed: u32, salt: u32) {
        let mut r = Rng(salt as u64 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        c.g[A1][0] = sx(DATA + 0x1000 + 4 * (seed % 4));
    }

    /// `lbl_003C555C` against its generated body.
    #[test]
    fn snd_store_const_555c_matches() {
        check(
            ee::s13::f_003c555c__generated,
            snd_store_const_555c,
            120,
            &|c, seed| stub_setup(c, seed, 0x003c_555c),
        );
    }

    /// `lbl_003C556C` against its generated body.
    #[test]
    fn snd_store_const_556c_matches() {
        check(
            ee::s13::f_003c556c__generated,
            snd_store_const_556c,
            120,
            &|c, seed| stub_setup(c, seed, 0x003c_556c),
        );
    }

    /// `lbl_003C557C` against its generated body.
    #[test]
    fn snd_store_const_557c_matches() {
        check(
            ee::s13::f_003c557c__generated,
            snd_store_const_557c,
            120,
            &|c, seed| stub_setup(c, seed, 0x003c_557c),
        );
    }

    /// `lbl_003C558C` against its generated body.
    #[test]
    fn snd_store_const_558c_matches() {
        check(
            ee::s13::f_003c558c__generated,
            snd_store_const_558c,
            120,
            &|c, seed| stub_setup(c, seed, 0x003c_558c),
        );
    }
}
