//! Staging module oc1119 (wave wave269): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1119`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0040_5e10,
        lib_mpeg_decode_dm_vector,
        "libmpeg_DecodeDmVector",
    ),
    (
        0x0040_5e30,
        lib_mpeg_dual_prime_arithmetic,
        "libmpeg_DualPrimeArithmetic",
    ),
    (
        0x0040_5fb8,
        lib_mpeg_macroblock_address_increment,
        "libmpeg_MacroblockAddressIncrement",
    ),
];

/// `libmpeg_IpuVdec` (0x0040_6f78): the IPU variable-length decode; the code
/// it decodes comes back in v0.
pub(crate) const MPEG_IPU_VDEC: Callee = Callee {
    addr: 0x0040_6f78,
    f: ee::s15::f_00406f78,
};
/// `libmpeg_ShowBits` (0x0040_70f0): the next bits of the stream, not consumed.
pub(crate) const MPEG_SHOW_BITS: Callee = Callee {
    addr: 0x0040_70f0,
    f: ee::s15::f_004070f0,
};
/// `libmpeg_FlushBits` (0x0040_71f8): consumes bits of the stream.
pub(crate) const MPEG_FLUSH_BITS: Callee = Callee {
    addr: 0x0040_71f8,
    f: ee::s15::f_004071f8,
};
/// `libmpeg_Errorf` (0x0040_3470): reports a decode error.
pub(crate) const MPEG_ERRORF: Callee = Callee {
    addr: 0x0040_3470,
    f: ee::s15::f_00403470,
};

/// `libmpeg_DecodeDmVector(dec)` (0x0040_5e10): decodes one dual-prime
/// differential motion vector. It is the IPU variable-length decode with
/// table 3 (`a1 = 3`) on the decoder state, and returns what that decoded.
pub fn lib_mpeg_decode_dm_vector(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x3;
        MPEG_IPU_VDEC.call(c, 0x0040_5e20)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// H1 of the reference's `calc_DMV`: `(x + (x > 0)) >> 1` on the 32-bit word,
/// with `pos` = `x > 0` (the 64-bit sign test of the register).
fn dp_half(x: u32, pos: bool) -> u32 {
    ((x.wrapping_add(pos as u32)) as i32 >> 1) as u32
}

/// H3 of the reference's `calc_DMV`: `(3x + (x > 0)) >> 1`.
fn dp_third(x: u32, pos: bool) -> u32 {
    ((x.wrapping_mul(3).wrapping_add(pos as u32)) as i32 >> 1) as u32
}

/// Frame picture with the bottom field first (`dec+0x178 == 0`): the vectors
/// come out as `H3(a)`, `H3(b)`, `H1(a)`, `H1(b)` with the field offsets.
fn dp_bottom_first(c: &mut Cpu, out: u32, inp: u32, a: u64, b: u64) {
    let (a32, b32) = (a as u32, b as u32);
    let (a_pos, b_pos) = ((a as i64) > 0, (b as i64) > 0);
    c.cycles += 3;
    let in0 = Addr(inp).lw(c);
    if a_pos {
        c.cycles += 1;
    }
    c.cycles += 7;
    Addr(out).sw(c, sx(dp_third(a32, a_pos).wrapping_add(in0 as u32)));
    let in1 = Addr(inp.wrapping_add(4)).lw(c);
    if b_pos {
        c.cycles += 1;
    }
    c.cycles += 6;
    let out1 = dp_third(b32, b_pos)
        .wrapping_add(in1 as u32)
        .wrapping_sub(1);
    Addr(out.wrapping_add(4)).sw(c, sx(out1));
    let in0 = Addr(inp).lw(c);
    c.cycles += if a_pos { 3 } else { 1 };
    let h1a = dp_half(a32, a_pos);
    c.cycles += 4;
    Addr(out.wrapping_add(8)).sw(c, sx(h1a.wrapping_add(in0 as u32)));
    let in1 = Addr(inp.wrapping_add(4)).lw(c);
    c.cycles += if b_pos { 3 } else { 2 };
    let h1b = dp_half(b32, b_pos);
    c.cycles += 4;
    let out3 = sx(h1b.wrapping_add(in1 as u32).wrapping_add(1));
    Addr(out.wrapping_add(0xc)).sw(c, out3);
    c.g[V0][0] = out3;
    c.g[V1][0] = in0;
    c.g[A2][0] = in1;
}

/// Frame picture with the top field first (`dec+0x178 != 0`): the vectors
/// come out as `H1(a)`, `H1(b)`, `H3(a)`, `H3(b)` with the field offsets.
fn dp_top_first(c: &mut Cpu, out: u32, inp: u32, a: u64, b: u64) {
    let (a32, b32) = (a as u32, b as u32);
    let (a_pos, b_pos) = ((a as i64) > 0, (b as i64) > 0);
    c.cycles += 2;
    let in0 = Addr(inp).lw(c);
    c.cycles += if a_pos { 3 } else { 1 };
    let h1a = dp_half(a32, a_pos);
    c.cycles += 4;
    Addr(out).sw(c, sx(h1a.wrapping_add(in0 as u32)));
    let in1 = Addr(inp.wrapping_add(4)).lw(c);
    c.cycles += if b_pos { 3 } else { 1 };
    let h1b = dp_half(b32, b_pos);
    c.cycles += 7;
    let out1 = h1b.wrapping_add(in1 as u32).wrapping_sub(1);
    Addr(out.wrapping_add(4)).sw(c, sx(out1));
    let in0 = Addr(inp).lw(c);
    if a_pos {
        c.cycles += 1;
    }
    c.cycles += 7;
    Addr(out.wrapping_add(8)).sw(c, sx(dp_third(a32, a_pos).wrapping_add(in0 as u32)));
    let in1 = Addr(inp.wrapping_add(4)).lw(c);
    if b_pos {
        c.cycles += 1;
    }
    c.cycles += 1;
    let h3b = dp_third(b32, b_pos);
    c.cycles += 4;
    let out3 = sx(h3b.wrapping_add(in1 as u32).wrapping_add(1));
    Addr(out.wrapping_add(0xc)).sw(c, out3);
    c.g[V0][0] = out3;
    c.g[V1][0] = in0;
    c.g[A2][0] = in1;
}

/// Field picture (`dec+0x174 != 3`): one vector `H1(a)`, `H1(b)` corrected
/// by -1 when the picture structure is 1 (top field) and by +1 otherwise.
fn dp_field(c: &mut Cpu, dec: u32, out: u32, inp: u32, a: u64, b: u64) {
    let (a32, b32) = (a as u32, b as u32);
    let (a_pos, b_pos) = ((a as i64) > 0, (b as i64) > 0);
    c.cycles += 2;
    let in0 = Addr(inp).lw(c);
    c.cycles += if a_pos { 3 } else { 1 };
    let h1a = dp_half(a32, a_pos);
    c.cycles += 4;
    Addr(out).sw(c, sx(h1a.wrapping_add(in0 as u32)));
    let in1 = Addr(inp.wrapping_add(4)).lw(c);
    c.cycles += if b_pos { 3 } else { 1 };
    let h1b = dp_half(b32, b_pos);
    c.cycles += 6;
    let sum = sx(h1b.wrapping_add(in1 as u32));
    Addr(out.wrapping_add(4)).sw(c, sum);
    c.g[A2][0] = sum;
    c.g[V1][0] = 1;
    let structure = Addr(dec.wrapping_add(0x174)).lw(c);
    let not_top = structure != c.g[V1][0];
    c.g[V0][0] = sx((sum as u32).wrapping_add(1));
    if not_top {
        c.cycles += 2;
    } else {
        c.cycles += 3;
        c.g[V0][0] = sx((sum as u32).wrapping_sub(1));
    }
    let v = c.g[V0][0];
    Addr(out.wrapping_add(4)).sw(c, v);
}

/// `libmpeg_DualPrimeArithmetic(dec, out, in, a, b)` (0x0040_5e30): the
/// dual-prime derived motion vectors (MSSG's `calc_DMV`). `dec` (a0) is the
/// decoder state: the picture structure at `+0x174` (3 = frame) and the
/// top-field-first flag at `+0x178`. `out` (a1) receives four words, `inp`
/// (a2) holds the differential vector, `a` (a3) and `b` (t0) the horizontal
/// and vertical motion vectors. Frame pictures derive both field vectors,
/// scaled by 1/2 and 3/2 with rounding away from zero, in the order the
/// top-field-first flag gives, with the vertical -1 / +1 field corrections.
pub fn lib_mpeg_dual_prime_arithmetic(c: &mut Cpu) {
    let dec = c.g[A0][0] as u32;
    let out = c.g[A1][0] as u32;
    let inp = c.g[A2][0] as u32;
    let a = c.g[A3][0];
    let b = c.g[T0][0];
    c.cycles += 4;
    let structure = Addr(dec.wrapping_add(0x174)).lw(c);
    if structure == 3 {
        c.cycles += 3;
        if Addr(dec.wrapping_add(0x178)).lw(c) == 0 {
            dp_bottom_first(c, out, inp, a, b);
        } else {
            dp_top_first(c, out, inp, a, b);
        }
    } else {
        dp_field(c, dec, out, inp, a, b);
    }
    ret(c);
}

/// What one pass of the MacroblockAddressIncrement loop decided.
#[derive(Clone, Copy)]
enum MbiStep {
    /// Another pass (`again` set).
    Again,
    /// The code's value is added to the sum and the loop ends.
    Done,
    /// `Errorf` was reported: the function returns 1 at once.
    Errored,
}

/// Saves a register at `off(sp)` with `sd`, the width the generated code uses.
fn mbi_save(c: &mut Cpu, frame: Frame, reg: usize, off: u32) {
    let a = frame.at(c, off);
    let v = c.g[reg][0];
    c.sd(a, v);
}

/// Restores a register from `off(sp)` with `ld`.
fn mbi_restore(c: &mut Cpu, frame: Frame, reg: usize, off: u32) {
    let a = frame.at(c, off);
    c.g[reg][0] = c.ld(a);
}

/// One pass of the loop: the IPU VDEC call and the switch on the code it
/// returns (the generated body's blocks, one per case).
fn mbi_pass(c: &mut Cpu) -> Result<MbiStep, Unwound> {
    c.cycles += 5;
    c.g[A1][0] = 0;
    MPEG_IPU_VDEC.call(c, 0x0040_6008)?;
    c.g[S0][0] = c.g[V0][0];
    // Case 34 (stuffing): another pass.
    if c.g[S0][0] == c.g[S6][0] {
        c.cycles += 2;
        c.g[V0][0] = 1;
        return Ok(MbiStep::Again);
    }
    c.g[V0][0] = (c.g[S0][0] < 0x23u64) as u64;
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        // Codes from 35 up: 35 is the escape (+33, another pass); the rest
        // take the default arm.
        c.cycles += 2;
        if c.g[S0][0] != c.g[S5][0] {
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
            return Ok(MbiStep::Done);
        }
        c.cycles += 3;
        c.g[V0][0] = 1;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x21));
        return Ok(MbiStep::Again);
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S1][0];
    if c.g[S0][0] != 0 {
        // Codes 1..33: the default arm.
        c.cycles += 2;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
        return Ok(MbiStep::Done);
    }
    // Code 0: look at the next 11 bits. The 11-bit stuffing code (15) is
    // skipped when the flag at +0x848 is set; anything else is an error.
    c.cycles += 6;
    c.g[A1][0] = 0xb;
    MPEG_SHOW_BITS.call(c, 0x0040_6048)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x848)).lw(c);
    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x7ed0));
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.g[A2][0] = c.g[S0][0];
    } else {
        c.cycles += 2;
        c.g[A2][0] = c.g[S0][0];
        if c.g[V1][0] == c.g[S3][0] {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0xb;
            MPEG_FLUSH_BITS.call(c, 0x0040_606c)?;
            c.cycles += 2;
            c.g[V0][0] = 1;
            return Ok(MbiStep::Again);
        }
    }
    // Error: report it, mark the state, return 1.
    c.cycles += 5;
    c.g[A0][0] = c.g[S1][0];
    MPEG_ERRORF.call(c, 0x0040_6080)?;
    let flag = c.g[S7][0];
    Addr((c.g[S1][0] as u32).wrapping_add(0x11c)).sw(c, flag);
    c.g[V0][0] = 1;
    Ok(MbiStep::Errored)
}

/// `libmpeg_MacroblockAddressIncrement(dec)` (0x0040_5fb8): reads the
/// macroblock address increment from the stream: the variable-length codes
/// are summed (34 is stuffing, 35 the escape worth 33), an error code is
/// reported through `Errorf` and returns 1. Returns the sum.
pub fn lib_mpeg_macroblock_address_increment(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        mbi_save(c, frame, S7, 0x70);
        mbi_save(c, frame, S6, 0x60);
        c.g[S7][0] = 1;
        mbi_save(c, frame, S5, 0x50);
        c.g[S6][0] = 0x22;
        mbi_save(c, frame, S4, 0x40);
        c.g[S5][0] = 0x23;
        mbi_save(c, frame, S3, 0x30);
        c.g[S4][0] = 0x0049_0000;
        mbi_save(c, frame, S2, 0x20);
        c.g[S3][0] = 0xf;
        mbi_save(c, frame, S1, 0x10);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0x80);
        c.g[S1][0] = c.g[A0][0];
        mbi_save(c, frame, S0, 0);
        c.g[A0][0] = c.g[S1][0];

        loop {
            match mbi_pass(c)? {
                MbiStep::Again => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_6000);
                    }
                }
                MbiStep::Done => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S2][0];
                    break;
                }
                MbiStep::Errored => break,
            }
        }

        c.cycles += 11;
        frame.restore_ra(c, 0x80);
        mbi_restore(c, frame, S7, 0x70);
        mbi_restore(c, frame, S6, 0x60);
        mbi_restore(c, frame, S5, 0x50);
        mbi_restore(c, frame, S4, 0x40);
        mbi_restore(c, frame, S3, 0x30);
        mbi_restore(c, frame, S2, 0x20);
        mbi_restore(c, frame, S1, 0x10);
        mbi_restore(c, frame, S0, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The dec struct of the dual-prime test, and its out and in words.
    const DP_DEC: u32 = DATA + 0x1000;
    const DP_OUT: u32 = DATA + 0x1100;
    const DP_IN: u32 = DATA + 0x1200;

    /// A motion vector operand: zero, a small negative or positive value, a
    /// random 32-bit word, or a random 64-bit value (its sign may sit in the
    /// high half only).
    fn dp_operand(r: &mut Rng, kind: u32) -> u64 {
        match kind {
            0 => 0,
            1 => (-(1 + (r.next_u32() % 300) as i64)) as u64,
            2 => (1 + r.next_u32() % 300) as u64,
            3 => sx(r.next_u32()),
            _ => ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
        }
    }

    /// The fixture of the dual-prime test: the picture structure (frame or
    /// field), the top-field-first flag, the operand kinds for a and b. The
    /// seed walks all 300 combinations (mixed radix 4 x 3 x 5 x 5).
    fn dp_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x405e30, 0);
        let structure = [3u32, 3, 1, 2][(seed % 4) as usize];
        let top_first = [0u32, 1, 2][((seed / 4) % 3) as usize];
        let a_kind = (seed / 12) % 5;
        let b_kind = (seed / 60) % 5;
        c.bus.write32(DP_DEC + 0x174, structure);
        c.bus.write32(DP_DEC + 0x178, top_first);
        fill(c, &mut r, DP_IN, 2);
        fill(c, &mut r, DP_OUT, 4);
        c.g[A0][0] = sx(DP_DEC);
        c.g[A1][0] = sx(DP_OUT);
        c.g[A2][0] = sx(DP_IN);
        c.g[A3][0] = dp_operand(&mut r, a_kind);
        c.g[T0][0] = dp_operand(&mut r, b_kind);
    }

    /// `libmpeg_DualPrimeArithmetic` against the generated body: frame
    /// pictures with either field first, field pictures, both signs and zero
    /// for both vectors.
    #[test]
    fn lib_mpeg_dual_prime_arithmetic_matches() {
        check(
            ee::s15::f_00405e30__generated,
            lib_mpeg_dual_prime_arithmetic,
            300,
            &dp_setup,
        );
    }

    /// Gate word of the DecodeDmVector test's VDEC fake (in the machine's RAM).
    const DM_CTRL: u32 = DATA + 0x3_0400;
    const DM_MAGIC: u32 = 0x1119_d0c5;
    static DM_PREV_VDEC: AtomicUsize = AtomicUsize::new(0);

    /// `libmpeg_IpuVdec`, gated: logs the call and returns a hashed result.
    fn dm_fake_vdec(c: &mut Cpu) {
        if c.bus.read32(DM_CTRL) != DM_MAGIC {
            return fakes::fall_back(c, &DM_PREV_VDEC, ee::s15::f_00406f78__generated);
        }
        fakes::fake_call(c, 0x0040_6f78)
    }

    fn dm_setup(c: &mut Cpu, seed: u32) {
        let _ = machine(c, seed, 0x405e10, 4);
        c.bus.write32(DM_CTRL, DM_MAGIC);
    }

    /// `libmpeg_DecodeDmVector` against the generated body: the IPU call with
    /// table 3, the return address and the callee's logged registers, with
    /// the VDEC fake unwinding at a random call.
    #[test]
    fn lib_mpeg_decode_dm_vector_matches() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406F78),
            dm_fake_vdec,
            &DM_PREV_VDEC,
        );
        check(
            ee::s15::f_00405e10__generated,
            lib_mpeg_decode_dm_vector,
            200,
            &dm_setup,
        );
    }

    /// Gate word of the MacroblockAddressIncrement test's fakes.
    const MBI_CTRL: u32 = DATA + 0x3_0000;
    const MBI_MAGIC: u32 = 0x1119_0b1a;
    /// Cursors into the code list and the peek list (each fake call takes
    /// the next entry; past the end the code is 40 and the peek is 15).
    const MBI_CUR: u32 = DATA + 0x3_0004;
    const MBI_PEEK_CUR: u32 = DATA + 0x3_0008;
    const MBI_CODES: u32 = DATA + 0x3_0100;
    const MBI_PEEKS: u32 = DATA + 0x3_0200;
    /// The decoder state of the fixture.
    const MBI_DEC: u32 = DATA + 0x1000;
    /// Codes the fixture draws: stuffing 34, escape 35, 0 (error or peek),
    /// the default range, and 36 (beyond the escape).
    const MBI_VLC: [u32; 9] = [34, 35, 0, 1, 7, 33, 36, 40, 35];
    /// The 11-bit peeks the fixture draws (15 is the stuffing code).
    const MBI_PEEK_BITS: [u32; 6] = [15, 15, 0, 3, 14, 16];

    static MBI_PREV_VDEC: AtomicUsize = AtomicUsize::new(0);
    static MBI_PREV_SHOW: AtomicUsize = AtomicUsize::new(0);
    static MBI_PREV_FLUSH: AtomicUsize = AtomicUsize::new(0);
    static MBI_PREV_ERRORF: AtomicUsize = AtomicUsize::new(0);

    /// `libmpeg_IpuVdec`, gated: logs the call and returns the next code.
    fn mbi_fake_vdec(c: &mut Cpu) {
        if c.bus.read32(MBI_CTRL) != MBI_MAGIC {
            return fakes::fall_back(c, &MBI_PREV_VDEC, ee::s15::f_00406f78__generated);
        }
        fakes::fake_call(c, 0x0040_6f78);
        let i = c.bus.read32(MBI_CUR);
        c.bus.write32(MBI_CUR, i + 1);
        c.g[V0][0] = if i < 16 {
            c.bus.read32(MBI_CODES + 4 * i) as u64
        } else {
            40
        };
    }

    /// `libmpeg_ShowBits`, gated: logs the call and returns the next peek.
    fn mbi_fake_show(c: &mut Cpu) {
        if c.bus.read32(MBI_CTRL) != MBI_MAGIC {
            return fakes::fall_back(c, &MBI_PREV_SHOW, ee::s15::f_004070f0__generated);
        }
        fakes::fake_call(c, 0x0040_70f0);
        let i = c.bus.read32(MBI_PEEK_CUR);
        c.bus.write32(MBI_PEEK_CUR, i + 1);
        c.g[V0][0] = if i < 16 {
            c.bus.read32(MBI_PEEKS + 4 * i) as u64
        } else {
            15
        };
    }

    /// `libmpeg_FlushBits`, gated: logs the call and returns.
    fn mbi_fake_flush(c: &mut Cpu) {
        if c.bus.read32(MBI_CTRL) != MBI_MAGIC {
            return fakes::fall_back(c, &MBI_PREV_FLUSH, ee::s15::f_004071f8__generated);
        }
        fakes::fake_call(c, 0x0040_71f8)
    }

    /// `libmpeg_Errorf`, gated: logs the call and returns.
    fn mbi_fake_errorf(c: &mut Cpu) {
        if c.bus.read32(MBI_CTRL) != MBI_MAGIC {
            return fakes::fall_back(c, &MBI_PREV_ERRORF, ee::s15::f_00403470__generated);
        }
        fakes::fake_call(c, 0x0040_3470)
    }

    fn mbi_hook_callees() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406F78),
            mbi_fake_vdec,
            &MBI_PREV_VDEC,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004070F0),
            mbi_fake_show,
            &MBI_PREV_SHOW,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004071F8),
            mbi_fake_flush,
            &MBI_PREV_FLUSH,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403470),
            mbi_fake_errorf,
            &MBI_PREV_ERRORF,
        );
    }

    /// The fixture: sixteen random codes and peeks, the decoder state with
    /// the stuffing flag at +0x848 set or clear, and random words around it.
    fn mbi_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x405fb8, 10);
        c.bus.write32(MBI_CTRL, MBI_MAGIC);
        c.bus.write32(MBI_CUR, 0);
        c.bus.write32(MBI_PEEK_CUR, 0);
        for i in 0..16 {
            let code = MBI_VLC[(r.next_u32() % MBI_VLC.len() as u32) as usize];
            c.bus.write32(MBI_CODES + 4 * i, code);
            let peek = MBI_PEEK_BITS[(r.next_u32() % MBI_PEEK_BITS.len() as u32) as usize];
            c.bus.write32(MBI_PEEKS + 4 * i, peek);
        }
        fill(c, &mut r, MBI_DEC, 0x240);
        c.bus
            .write32(MBI_DEC + 0x848, if seed % 3 == 0 { 0 } else { 1 });
        c.g[A0][0] = sx(MBI_DEC);
    }

    /// `libmpeg_MacroblockAddressIncrement` against the generated body: all
    /// the codes (stuffing, escape, error, peek branches, default), the
    /// stuffing flag, and the fakes unwinding at random calls.
    #[test]
    fn lib_mpeg_macroblock_address_increment_matches() {
        mbi_hook_callees();
        check(
            ee::s15::f_00405fb8__generated,
            lib_mpeg_macroblock_address_increment,
            300,
            &mbi_setup,
        );
    }
}
