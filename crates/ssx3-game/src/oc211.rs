//! Staging wave 201 (oc211): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size),
//! one function per writer. Each reproduces the generated body's registers,
//! cycles, memory and call order.

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
    (0x003c_de68, snd_utk_decode, "SND_UtkDecode"),
];

// ------------------------------------------------------------ SND_UtkDecode

/// `SND_Utk` (EA MicroTalk decoder state) fields read by `SND_UtkDecode`.
mod utk {
    /// Bit reader: next input byte, current byte, bits left in it.
    pub const BIT_PTR: u32 = 0x000;
    pub const BIT_BUF: u32 = 0x004;
    pub const BIT_COUNT: u32 = 0x008;
    /// `float[432]`: the last decoded frame.
    pub const DECODED: u32 = 0x684;
    /// Samples of `DECODED` not yet handed out (counted from its end).
    pub const AVAIL: u32 = 0xd44;
    /// The per-frame header after the frame's bits (mode 1).
    pub const HEADER: u32 = 0xd48;
    /// `float*`: where the next samples go.
    pub const OUT: u32 = 0xd4c;
    /// `s16` (stored big-endian in the stream): raw samples patched in.
    pub const PATCH_COUNT: u32 = 0xd50;
    /// `s16`: first `DECODED` index the raw samples replace.
    pub const PATCH_START: u32 = 0xd52;
    /// Samples left in the stream.
    pub const LEFT: u32 = 0xd54;
    /// Non-zero: the header carries a raw-sample patch (set by a 0xee tag).
    pub const PATCH_PENDING: u32 = 0xd60;
    /// 1: every frame is followed by a byte-aligned header.
    pub const MODE: u32 = 0xd64;
    /// Samples per frame.
    pub const FRAME: u32 = 0x1b0;
}

const UTK_DECODE_FRAME: Callee = Callee {
    addr: 0x003c_d878,
    f: ee::s14::SND_UtkDecodeFrame,
};

/// `memcpy` (called through the function table).
const MEMCPY: u32 = 0x0041_605c;

/// `int SND_UtkDecode(SND_Utk* u, float** out, int n)` (0x3cde68): hand out
/// up to `n` MicroTalk samples (no more than the stream has left): copy what
/// is left of the decoded frame to `*out`, decode further 432-sample frames
/// as needed. In mode 1 a byte-aligned header follows each frame: a pending
/// patch (`count` big-endian s16 samples replacing `DECODED[start..]`), then
/// a 0xee tag that announces a patch after the next frame; the bit reader
/// restarts on the byte after it. Returns the number of samples written
/// (0 when the stream is finished) and advances `LEFT`.
pub fn snd_utk_decode(c: &mut Cpu) {
    guest(c, utk_decode_body)
}

#[inline(always)]
fn fld(u: u64, off: u32) -> u32 {
    (u as u32).wrapping_add(off)
}

fn utk_decode_body(c: &mut Cpu) -> crate::call::Flow {
    use utk::*;

    c.cycles += 15;
    let frame = Frame::open(c, 0xb0);
    frame.save(c, S1, 0x90);
    frame.save(c, S0, 0xa0);
    let mut u = c.g[A0][0];
    c.g[S1][0] = u;
    frame.save(c, S2, 0x80);
    frame.save(c, S3, 0x70);
    frame.save(c, S4, 0x60);
    frame.save(c, S5, 0x50);
    frame.save(c, S6, 0x40);
    frame.save(c, S7, 0x30);
    frame.save(c, FP, 0x20);
    frame.save_ra(c, 0x10);
    let left = c.lw(fld(u, LEFT));
    c.g[V0][0] = left;
    // s2: samples to take from the current frame.
    let mut take = c.lw(fld(u, AVAIL));
    c.g[S2][0] = take;

    if left == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        // s4 = min(n, left) (64-bit signed, as compared), s3 = still to write.
        c.cycles += 7;
        let mut total = left;
        let out = c.lw(c.g[A1][0] as u32);
        c.g[V1][0] = out;
        let n = c.g[A2][0];
        let lt = ((n as i64) < (total as i64)) as u64;
        c.g[V0][0] = lt;
        if lt != 0 {
            total = n;
        }
        c.g[S4][0] = total;
        c.sw(fld(u, OUT), out);
        let mut todo = total;
        c.g[S3][0] = todo;
        if (total as i64) > 0 {
            c.cycles += 6;
            c.g[S6][0] = FRAME as u64;
            c.g[S5][0] = 1;
            c.g[FP][0] = sx(fld(u, PATCH_START));
            c.g[S7][0] = sx(fld(u, PATCH_COUNT));
            c.g[A1][0] = c.lw(fld(u, AVAIL));
            loop {
                // Copy min(todo, avail) samples from the frame's tail.
                c.cycles += 18;
                let lt = ((todo as i64) < (take as i64)) as u64;
                c.g[V0][0] = lt;
                if lt != 0 {
                    take = todo;
                    c.g[S2][0] = take;
                }
                c.g[A0][0] = c.lw(fld(u, OUT));
                let first = FRAME.wrapping_sub(c.g[A1][0] as u32);
                let bytes = sx((take as u32) << 2);
                c.g[S0][0] = bytes;
                c.g[A2][0] = bytes;
                c.g[A1][0] = sx((first << 2).wrapping_add(u as u32).wrapping_add(DECODED));
                todo = sx((todo as u32).wrapping_sub(take as u32));
                c.g[S3][0] = todo;
                call_ptr(c, MEMCPY, 0x003c_df0c)?;
                let out = sx((c.lw(fld(u, OUT)) as u32).wrapping_add(bytes as u32));
                c.g[V1][0] = out;
                let avail = sx((c.lw(fld(u, AVAIL)) as u32).wrapping_sub(take as u32));
                c.g[V0][0] = avail;
                c.sw(fld(u, OUT), out);
                c.sw(fld(u, AVAIL), avail);
                if (todo as i64) <= 0 {
                    break;
                }

                // Decode the next frame.
                c.cycles += 6;
                c.g[A0][0] = u;
                take = FRAME as u64;
                c.g[S2][0] = take;
                UTK_DECODE_FRAME.call(c, 0x003c_df34)?;
                let mode = c.lw(fld(u, MODE));
                c.g[V0][0] = mode;
                c.sw(fld(u, AVAIL), c.g[S6][0]);
                if mode == c.g[S5][0] {
                    utk_frame_header(c, u);
                }

                c.cycles += 2;
                if (todo as i64) > 0 {
                    c.g[A1][0] = c.lw(fld(u, AVAIL));
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_dee0);
                        u = c.g[S1][0];
                        todo = c.g[S3][0];
                        take = c.g[S2][0];
                    }
                } else {
                    break;
                }
            }
            total = c.g[S4][0];
        }
        // LEFT -= written; return written.
        c.cycles += 4;
        let rest = sx((c.lw(fld(u, LEFT)) as u32).wrapping_sub(total as u32));
        c.g[V1][0] = rest;
        c.g[V0][0] = total;
        c.sw(fld(u, LEFT), rest);
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
}

/// Mode 1, after a frame: apply a pending raw-sample patch from the header
/// at `BIT_PTR - 1`, check the 0xee tag after it and restart the bit reader
/// on the following byte.
fn utk_frame_header(c: &mut Cpu, u: u64) {
    use utk::*;

    c.cycles += 6;
    let ptr = c.lw(fld(u, BIT_PTR));
    c.g[A0][0] = ptr;
    let pending = c.lw(fld(u, PATCH_PENDING));
    c.g[V1][0] = pending;
    let header = sx((ptr as u32).wrapping_sub(1));
    c.g[V0][0] = header;
    c.g[A2][0] = header;
    c.sw(fld(u, HEADER), header);
    if pending != 0 {
        // Header: start (BE s16), count (BE s16), count BE s16 samples.
        c.cycles += 22;
        let p = ptr as u32;
        let b = c.lbu(p);
        c.g[V1][0] = b;
        c.g[A2][0] = sx(p.wrapping_add(3));
        c.sb(fld(u, PATCH_START), b);
        let b = c.lbu(p.wrapping_sub(1));
        c.g[V0][0] = b;
        c.sb(fld(u, PATCH_START + 1), b);
        let b = c.lbu(p.wrapping_add(2));
        c.g[V1][0] = b;
        c.sb(fld(u, PATCH_COUNT), b);
        let b = c.lbu(p.wrapping_add(1));
        c.g[V0][0] = b;
        c.sb(fld(u, PATCH_COUNT + 1), b);
        let count = c.lh(fld(u, PATCH_COUNT));
        c.g[A1][0] = count;
        let header = c.lw(fld(u, HEADER)) as u32;
        let start = c.lh(fld(u, PATCH_START));
        let tag = header.wrapping_add((count as u32) << 1);
        let next = sx(tag.wrapping_add(5));
        c.g[V0][0] = next;
        let dst_off = sx(((start as u32) << 2).wrapping_add(DECODED));
        c.g[A0][0] = dst_off;
        let tag = sx(tag.wrapping_add(4));
        c.g[V1][0] = tag;
        c.sw(fld(u, HEADER), tag);
        c.g[A3][0] = sx((u as u32).wrapping_add(dst_off as u32));
        c.sw(fld(u, BIT_PTR), next);
        if (count as i64) > 0 {
            c.cycles += 2;
            c.g[A1][0] = 0x10000;
            c.g[T0][0] = 0x10000;
            loop {
                // decoded[start + i] = (float)(s16 big-endian sample i);
                // a1 = (i + 1) << 16.
                c.cycles += 16;
                let src = c.g[A2][0] as u32;
                let lo = c.lbu(src.wrapping_add(1));
                c.g[V0][0] = lo;
                let k = c.g[A1][0] as u32;
                let i = sx(((k as i32) >> 16) as u32);
                c.g[A0][0] = i;
                c.g[A1][0] = sx(k.wrapping_add(c.g[T0][0] as u32));
                let tmp = sp(c);
                c.sb(tmp, lo);
                let hi = c.lbu(src);
                c.g[V1][0] = hi;
                c.g[A2][0] = sx(src.wrapping_add(2));
                c.sb(tmp.wrapping_add(1), hi);
                c.g[V0][0] = c.lh(tmp);
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A3][0] as u32);
                let u = c.g[S1][0];
                let count = c.lh(fld(u, PATCH_COUNT));
                c.g[V0][0] = count;
                let more = ((i as i64) < (count as i64)) as u64;
                c.g[A0][0] = more;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(4));
                if more == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_dfb8);
                }
            }
        }
        c.cycles += 1;
        c.sw(fld(c.g[S1][0], PATCH_PENDING), 0);
    }

    // A 0xee tag after the header: the next frame's header has a patch.
    c.cycles += 5;
    let header = c.lw(fld(u, HEADER));
    c.g[V0][0] = header;
    c.g[A0][0] = 0xee;
    let tag = c.lbu(header as u32);
    c.g[V1][0] = tag;
    if tag == 0xee {
        c.sw(fld(u, PATCH_PENDING), c.g[S5][0]);
    }

    // Restart the bit reader on the next byte.
    c.cycles += 7;
    c.g[V0][0] = 8;
    let p = c.lw(fld(u, BIT_PTR));
    c.g[V1][0] = p;
    c.sw(fld(u, BIT_COUNT), 8);
    let next = sx((p as u32).wrapping_add(1));
    c.g[A0][0] = next;
    let byte = c.lbu(p as u32);
    c.g[V0][0] = byte;
    c.sw(fld(u, BIT_PTR), next);
    c.sw(fld(u, BIT_BUF), byte);
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------- SND_UtkDecode

    /// `SND_UtkDecode` against the generated body, with the real callees
    /// (memcpy, `SND_UtkDecodeFrame` and its bit reader and synthesis on the
    /// game's tables). `seed % 8` picks the case:
    ///
    /// - 0: the stream is finished (`LEFT == 0`): returns 0;
    /// - 1: nothing to write (`n <= 0`, or `LEFT < 0`);
    /// - 2: the request fits in what is left of the frame (one copy);
    /// - 3: mode != 1, up to three frames decoded from random bits;
    /// - 4: mode 1 without a pending patch (random bits; the header byte
    ///   may be a 0xee tag by chance), one frame;
    /// - 5..7: mode 1 with a pending patch: a zero bit stream, run once
    ///   through the real frame decoder first to find where the frame ends,
    ///   then a header planted there: 5 a count <= 0, 6/7 1..24 samples at
    ///   a start that keeps them inside the frame, 7 followed by a 0xee tag.
    ///
    /// `n` against `LEFT` alternates which one bounds the write, and the
    /// state's 8-byte alignment alternates (the frame decoder's copy loops).
    #[test]
    fn snd_utk_decode_matches() {
        use crate::difftest::{check, elf_image, fill, scramble, Rng, DATA};
        use crate::game::lib::snd::utk_frame as f;
        use super::utk::*;

        check(ee::s14::f_003cde68__generated, snd_utk_decode, 96, &|c, seed| {
            let case = seed % 8;
            let mut r = Rng(0x003c_de68 ^ (seed as u64) << 8);
            elf_image(c);
            let u = DATA + 0x1000 + 4 * ((seed / 16) % 2);
            let input = DATA + 0x8000;
            let out_slot = DATA + 0x400;
            let out = DATA + 0x1_0000;
            let patch = case >= 5;

            // The decoder state (bit reader, mode fields, float history),
            // written from its own generator so it can be written twice.
            let state_seed = r.next_u32() as u64;
            let state = |c: &mut Cpu| {
                let mut s = Rng(state_seed | 1);
                c.bus.write32(u + BIT_PTR, input + s.next_u32() % 8);
                c.bus.write32(u + BIT_BUF, s.next_u32());
                c.bus.write32(u + BIT_COUNT, 8 + s.next_u32() % 25);
                let reduced = if s.next_u32() % 2 == 0 { 0 } else { s.next_u32() | 1 };
                c.bus.write32(u + f::REDUCED_BW, reduced);
                c.bus.write32(u + f::VOICED_THRESHOLD, s.next_u32() % 72);
                for i in 0..64 {
                    let v = s.f32(2.0);
                    c.bus.write32(u + f::FIXED_GAINS + 4 * i, v);
                }
                for i in 0..12 {
                    let v = s.f32(0.9);
                    c.bus.write32(u + f::RC + 4 * i, v);
                    let h = s.f32(100.0);
                    c.bus.write32(u + 0x144 + 4 * i, h);
                }
                for i in 0..324 {
                    let v = s.f32(1000.0);
                    c.bus.write32(u + f::ADAPT_CB + 4 * i, v);
                }
                fill(c, &mut s, u + DECODED, 432);
            };
            if !patch {
                fill(c, &mut r, input, 0x800);
            }
            state(c);

            if patch {
                // Where does the frame end on this (zero) bit stream?
                c.g[A0][0] = sx(u);
                c.g[RA][0] = sx(crate::difftest::RA);
                c.pc = 0x003c_d878;
                ee::s14::f_003cd878__generated(c);
                let end = c.bus.read32(u + BIT_PTR);
                state(c);
                // Header at end - 1 (that byte is consumed: 0, the start's
                // high byte): start, count (BE), samples, tag.
                let count: i32 = if case == 5 {
                    -((r.next_u32() % 50) as i32)
                } else {
                    1 + (r.next_u32() % 24) as i32
                };
                // start <= 255 and count <= 24: the patch stays in DECODED.
                let start = r.next_u32() % 256;
                c.bus.write8(end, start as u8);
                c.bus.write8(end + 1, (count >> 8) as u8);
                c.bus.write8(end + 2, count as u8);
                for i in 0..2 * count.max(0) as u32 {
                    let b = r.next_u32() as u8;
                    c.bus.write8(end + 3 + i, b);
                }
                // The tag after the samples (a count <= 0 points it back into
                // the consumed stream: left alone there).
                if count > 0 {
                    let tag = if case == 7 { 0xee } else { r.next_u32() as u8 & 0x7f };
                    c.bus.write8(end + 3 + 2 * count as u32, tag);
                }
            }

            scramble(c, &mut r);
            c.pc = 0x003c_de68;
            c.g[RA][0] = sx(crate::difftest::RA);

            let avail = match case {
                2 => 1 + r.next_u32() % 432,
                _ => r.next_u32() % 433,
            };
            let frames = if case == 3 { 3 } else { 1 };
            let n: i32 = match case {
                0 | 2 => 1 + (r.next_u32() % avail.max(1)) as i32,
                1 => -((r.next_u32() % 3) as i32),
                _ => (avail + 1 + r.next_u32() % (432 * frames)) as i32,
            };
            let left: i32 = match case {
                0 => 0,
                1 if seed % 16 < 8 => -1 - (r.next_u32() % 100) as i32,
                1 => 1 + (r.next_u32() % 100) as i32,
                2 => n + (r.next_u32() % 100) as i32,
                _ if seed % 16 < 8 => n + (r.next_u32() % 100) as i32,
                _ => avail as i32 + 1 + (r.next_u32() % (n as u32 - avail)) as i32,
            };
            let mode = match case {
                3 => {
                    if seed % 16 < 8 { 0 } else { 2 + r.next_u32() % 4 }
                }
                _ => 1,
            };
            let pending = if patch { 1 + r.next_u32() % 7 } else { 0 };
            c.bus.write32(u + AVAIL, avail);
            c.bus.write32(u + LEFT, left as u32);
            c.bus.write32(u + MODE, mode);
            c.bus.write32(u + PATCH_PENDING, pending);
            c.bus.write32(u + HEADER, r.next_u32());
            c.bus.write32(u + OUT, r.next_u32());
            c.bus.write32(u + PATCH_COUNT, r.next_u32());
            c.bus.write32(out_slot, out);
            c.g[A0][0] = sx(u);
            c.g[A1][0] = sx(out_slot);
            c.g[A2][0] = sx(n as u32);
        });
    }
}
