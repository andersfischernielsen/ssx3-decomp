//! `src/md5.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::set_word;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_d368, md5_init, "MD5_Init"),
    (0x0031_7690, md5_hash_string64, "MD5_HashString64"),
    (
        0x0031_76f0,
        md5_hash_string64_discard,
        "MD5_HashString64Discard",
    ),
    (0x0031_7710, md5_hash_string, "MD5_HashString"),
    (
        0x0031_7798,
        md5_hash_string_discard,
        "MD5_HashStringDiscard",
    ),
    (0x0031_c350, md5_transform, "MD5_Transform"),
    (0x0031_d3a8, md5_update, "MD5_Update"),
    (0x0031_d4d0, md5_final, "MD5_Final"),
];

/// `void MD5_Init(MD5_CTX *ctx)` (0x31d368): load the standard MD5 constants.
pub fn md5_init(c: &mut Cpu) {
    c.cycles += 15;
    let this = abi::arg(c, 0);
    c.g[V1][0] = 0x6745_0000;
    c.g[A1][0] = 0xffff_ffff_efcd_0000;
    c.g[A2][0] = 0xffff_ffff_98ba_0000;
    c.g[V0][0] = 0x1032_0000;
    c.g[V1][0] = c.g[V1][0] | 0x2301;
    c.g[A1][0] = c.g[A1][0] | 0xab89;
    c.g[A2][0] = c.g[A2][0] | 0xdcfe;
    c.g[V0][0] = c.g[V0][0] | 0x5476;
    Addr(this).off(0x8).sw(c, c.g[V1][0]);
    Addr(this).off(0x14).sw(c, c.g[V0][0]);
    Addr(this).off(0xc).sw(c, c.g[A1][0]);
    Addr(this).off(0x10).sw(c, c.g[A2][0]);
    Addr(this).off(0x4).sw(c, 0);
    Addr(this).sw(c, 0);
    abi::ret(c);
}

pub(crate) mod callee {

    pub(crate) use crate::game::mem::memstd::callee::MD5_HASH_STRING64;
    pub(crate) use crate::game::mem::memstd::callee::MD5_TRANSFORM;
    pub(crate) use crate::game::mem::memstd::callee::MD5_UPDATE;

    pub(crate) use crate::game::misc::callee::MD5_FINAL;
    pub(crate) use crate::game::misc::callee::MD5_INIT;

    pub(crate) use crate::game::mem::memstd::callee::MD5_HASH_STRING;
}

/// The MD5 sine constants `K[i] = floor(|sin(i + 1)| * 2^32)` (RFC 1321).
pub(crate) const MD5_TRANSFORM_SINES: [u32; 64] = [
    0xd76a_a478,
    0xe8c7_b756,
    0x2420_70db,
    0xc1bd_ceee,
    0xf57c_0faf,
    0x4787_c62a,
    0xa830_4613,
    0xfd46_9501,
    0x6980_98d8,
    0x8b44_f7af,
    0xffff_5bb1,
    0x895c_d7be,
    0x6b90_1122,
    0xfd98_7193,
    0xa679_438e,
    0x49b4_0821,
    0xf61e_2562,
    0xc040_b340,
    0x265e_5a51,
    0xe9b6_c7aa,
    0xd62f_105d,
    0x0244_1453,
    0xd8a1_e681,
    0xe7d3_fbc8,
    0x21e1_cde6,
    0xc337_07d6,
    0xf4d5_0d87,
    0x455a_14ed,
    0xa9e3_e905,
    0xfcef_a3f8,
    0x676f_02d9,
    0x8d2a_4c8a,
    0xfffa_3942,
    0x8771_f681,
    0x6d9d_6122,
    0xfde5_380c,
    0xa4be_ea44,
    0x4bde_cfa9,
    0xf6bb_4b60,
    0xbebf_bc70,
    0x289b_7ec6,
    0xeaa1_27fa,
    0xd4ef_3085,
    0x0488_1d05,
    0xd9d4_d039,
    0xe6db_99e5,
    0x1fa2_7cf8,
    0xc4ac_5665,
    0xf429_2244,
    0x432a_ff97,
    0xab94_23a7,
    0xfc93_a039,
    0x655b_59c3,
    0x8f0c_cc92,
    0xffef_f47d,
    0x8584_5dd1,
    0x6fa8_7e4f,
    0xfe2c_e6e0,
    0xa301_4314,
    0x4e08_11a1,
    0xf753_7e82,
    0xbd3a_f235,
    0x2ad7_d2bb,
    0xeb86_d391,
];

/// Per-step left rotations, four per round.
pub(crate) const MD5_TRANSFORM_SHIFTS: [[u32; 4]; 4] = [
    [7, 12, 17, 22],
    [5, 9, 14, 20],
    [4, 11, 16, 23],
    [6, 10, 15, 21],
];

/// Frame size and the callee-saved slots (s0-s7, fp) of `MD5_Transform`.
pub(crate) const MD5_TRANSFORM_FRAME: u32 = 0x1a0;

pub(crate) const MD5_TRANSFORM_SAVES: [(usize, u32); 9] = [
    (S5, 0x140),
    (S0, 0x190),
    (S1, 0x180),
    (S2, 0x170),
    (S3, 0x160),
    (S4, 0x150),
    (S6, 0x130),
    (S7, 0x120),
    (FP, 0x110),
];

/// `MD5_CTX` field: the four state words `a, b, c, d` (after the 64-bit bit count).
pub(crate) const MD5_TRANSFORM_STATE: u32 = 0x8;

/// Frame slot of the copy of an unaligned input block (16 words at `sp`).
pub(crate) const MD5_TRANSFORM_COPY: u32 = 0;

/// Frame slot where the compiler spills the 16 block words (`x[i]` at `+4i`).
pub(crate) const MD5_TRANSFORM_WORD_SPILL: u32 = 0x40;

/// The round constants the compiler keeps in frame slots for later rounds:
/// (slot, 0-based step).
pub(crate) const MD5_TRANSFORM_SINE_SPILLS: [(u32, usize); 33] = [
    (0x80, 18),
    (0x84, 20),
    (0x88, 21),
    (0x8c, 22),
    (0x90, 29),
    (0x94, 31),
    (0x98, 32),
    (0x9c, 33),
    (0xa0, 34),
    (0xa4, 35),
    (0xa8, 37),
    (0xac, 38),
    (0xb0, 39),
    (0xb4, 40),
    (0xb8, 41),
    (0xbc, 42),
    (0xc0, 44),
    (0xc4, 45),
    (0xc8, 47),
    (0xcc, 48),
    (0xd0, 50),
    (0xd4, 52),
    (0xd8, 53),
    (0xdc, 54),
    (0xe0, 55),
    (0xe4, 56),
    (0xe8, 57),
    (0xec, 58),
    (0xf0, 59),
    (0xf4, 60),
    (0xf8, 61),
    (0xfc, 62),
    (0x100, 63),
];

/// One MD5 step's intermediate values the compiled body leaves in scratch
/// registers at return.
#[derive(Clone, Copy, Default)]
pub(crate) struct Md5TransformStep {
    /// The round's "not" term ANDed with its partner (`~d & c` in round 2).
    masked: u32,
    /// `a + F(b, c, d) + x[k]`, before the constant is added.
    partial: u32,
    /// `a + F + x[k] + K[i]`, the value that is rotated.
    sum: u32,
    /// The step's new `b`.
    next: u32,
}

/// Run the 64 MD5 steps over `words` from `state`; returns the final round
/// registers `(a, b, c, d)` and each step's intermediates.
pub(crate) fn md5_transform_rounds(
    state: [u32; 4],
    words: &[u32; 16],
) -> ([u32; 4], [Md5TransformStep; 64]) {
    let [mut a, mut b, mut c, mut d] = state;
    let mut steps = [Md5TransformStep::default(); 64];
    for (i, step) in steps.iter_mut().enumerate() {
        let (mix, masked, k) = match i / 16 {
            0 => ((b & c) | (!b & d), !b & d, i),
            1 => ((d & b) | (!d & c), !d & c, (5 * i + 1) % 16),
            2 => (b ^ c ^ d, 0, (3 * i + 5) % 16),
            _ => (c ^ (b | !d), 0, (7 * i) % 16),
        };
        let partial = a.wrapping_add(mix).wrapping_add(words[k]);
        let sum = partial.wrapping_add(MD5_TRANSFORM_SINES[i]);
        let next = b.wrapping_add(sum.rotate_left(MD5_TRANSFORM_SHIFTS[i / 16][i % 4]));
        (a, d, c, b) = (d, c, b, next);
        *step = Md5TransformStep {
            masked,
            partial,
            sum,
            next,
        };
    }
    ([a, b, c, d], steps)
}

/// `void MD5_Transform(MD5_CTX *ctx, const unsigned char block[64])`
/// (0x31c350): fold one 64-byte block into the MD5 state at `ctx + 8`. An
/// input block that is not word-aligned is first copied into the frame.
pub fn md5_transform(c: &mut Cpu) {
    c.cycles += 17;
    let frame = Frame::open(c, MD5_TRANSFORM_FRAME);
    for (reg, off) in MD5_TRANSFORM_SAVES {
        frame.save(c, reg, off);
    }
    let ctx = Addr(c.g[A0][0] as u32).off(MD5_TRANSFORM_STATE);
    let input = c.g[A1][0] as u32;
    let state: [u32; 4] = std::array::from_fn(|i| ctx.off(4 * i as u32).lw(c) as u32);
    let block = if input & 3 != 0 {
        c.cycles += 33;
        let copy = frame.at(c, MD5_TRANSFORM_COPY);
        for i in 0..64 {
            let byte = Addr(input + i).lbu(c);
            Addr(copy + i).sb(c, byte);
        }
        copy
    } else {
        c.cycles += 2;
        input
    };

    c.cycles += 977;
    let words: [u32; 16] = std::array::from_fn(|i| Addr(block + 4 * i as u32).lw(c) as u32);
    let stack = Addr(sp(c));
    for (i, &word) in words.iter().enumerate() {
        stack
            .off(MD5_TRANSFORM_WORD_SPILL + 4 * i as u32)
            .sw(c, u64::from(word));
    }
    for (slot, step) in MD5_TRANSFORM_SINE_SPILLS {
        stack.off(slot).sw(c, u64::from(MD5_TRANSFORM_SINES[step]));
    }
    let (round, steps) = md5_transform_rounds(state, &words);

    // Feed forward: each state word is re-read and summed in the order a, d, c, b.
    let mut out = [0u32; 4];
    for i in [0, 3, 2, 1] {
        out[i] = (ctx.off(4 * i as u32).lw(c) as u32).wrapping_add(round[i]);
        ctx.off(4 * i as u32).sw(c, u64::from(out[i]));
    }
    for (reg, off) in MD5_TRANSFORM_SAVES {
        frame.restore(c, reg, off);
    }

    // What the compiled body leaves in the result and scratch registers.
    let last = steps[63];
    set_word(c, V0, out[1]);
    set_word(c, V1, last.sum >> (32 - MD5_TRANSFORM_SHIFTS[3][3]));
    set_word(c, A0, last.partial);
    set_word(c, A1, out[2]);
    set_word(c, A2, words[9]);
    set_word(c, A3, steps[28].masked);
    set_word(c, T0, MD5_TRANSFORM_SINES[63]);
    set_word(c, T1, MD5_TRANSFORM_SINES[23]);
    set_word(c, T2, MD5_TRANSFORM_SINES[30]);
    set_word(c, T3, MD5_TRANSFORM_SINES[36]);
    set_word(c, T4, state[1]);
    set_word(c, T5, MD5_TRANSFORM_SINES[46]);
    set_word(c, T6, MD5_TRANSFORM_SINES[49]);
    set_word(c, T7, MD5_TRANSFORM_SINES[51]);
    set_word(c, T8, steps[61].next);
    set_word(c, T9, steps[60].next);
    frame.close(c);
}

/// `void MD5_Final(...)` (0x31d4d0): finalize the MD5 context into the
/// digest.
pub fn md5_final(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0031_d4d0 => {
                    c.cycles += 8;
                    let frame = Frame::open(c, 0x40);
                    c.g[A2][0] = 0u64;
                    frame.save(c, 16, 0x30);
                    frame.save(c, 17, 0x20);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0x10);
                    c.g[S1][0] = c.g[A1][0];
                    pc = 0x0031_d4f0;
                    continue;
                }
                0x0031_d4f0 => {
                    c.cycles += 12;
                    c.g[V0][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
                    c.g[A0][0] = c.g[A2][0] & 0x3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 8i64) as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> ((c.g[A0][0] as u32) & 31));
                    let t = c.g[V0][0] != 0u64;
                    c.sb(c.g[A1][0] as u32, c.g[V1][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_d4f0);
                        }
                        pc = 0x0031_d4f0;
                        continue;
                    }
                    c.cycles += 16;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A2][0] = 0x37;
                    c.g[A1][0] = 0x490000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2508));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[A2][0] = c.g[A2][0] & 0x3f;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    MD5_UPDATE.call(c, 0x0031_d548)?;
                    c.g[A2][0] = 0x8;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[SP][0];
                    MD5_UPDATE.call(c, 0x0031_d558)?;
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = 0u64;
                    pc = 0x0031_d560;
                    continue;
                }
                0x0031_d560 => {
                    c.cycles += 12;
                    c.g[V0][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
                    c.g[A0][0] = c.g[A2][0] & 0x3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 16i64) as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> ((c.g[A0][0] as u32) & 31));
                    let t = c.g[V0][0] != 0u64;
                    c.sb(c.g[A1][0] as u32, c.g[V1][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_d560);
                        }
                        pc = 0x0031_d560;
                        continue;
                    }
                    c.cycles += 5;
                    let frame = Frame { size: 0x40 };
                    frame.restore(c, 16, 0x30);
                    frame.restore(c, 17, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0031_d4d0, 0x0031_d57c);
                    return Ok(());
                }
            }
        }
    })
}

/// `void MD5_Update(...)` (0x31d3a8): add bytes to the MD5 context.
pub fn md5_update(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0031_d3a8 => {
                    c.cycles += 18;
                    let frame = Frame::open(c, 0x70);
                    frame.save(c, 16, 0x60);
                    frame.save(c, 17, 0x50);
                    frame.save(c, 18, 0x40);
                    frame.save(c, 19, 0x30);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, 20, 0x20);
                    frame.save(c, 21, 0x10);
                    c.g[S4][0] = c.g[A1][0];
                    frame.save_ra(c, 0);
                    c.g[S5][0] = c.g[A2][0];
                    c.g[S0][0] = c.g[S5][0];
                    c.g[S1][0] = c.g[S4][0];
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A1][0] = sx((c.g[S0][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 3);
                    let t = (c.g[S0][0] as i64) <= 0;
                    c.g[S3][0] = c.g[V0][0] & 0x3f;
                    if t {
                        pc = 0x0031_d4a8;
                        continue;
                    }
                    pc = 0x0031_d3f0;
                    continue;
                }
                0x0031_d3f0 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx(((c.g[S0][0] as u32) as i32 >> 29) as u32);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = (c.g[V1][0] < c.g[A1][0]) as u64;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    let t = c.g[A1][0] == 0u64;
                    c.sw(c.g[S2][0] as u32, c.g[V1][0]);
                    if t {
                        pc = 0x0031_d418;
                        continue;
                    }
                    pc = 0x0031_d410;
                    continue;
                }
                0x0031_d410 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    pc = 0x0031_d418;
                    continue;
                }
                0x0031_d418 => {
                    c.cycles += 2;
                    let t = c.g[S3][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    if t {
                        pc = 0x0031_d488;
                        continue;
                    }
                    pc = 0x0031_d420;
                    continue;
                }
                0x0031_d420 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 65i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.g[S0][0];
                    if t {
                        pc = 0x0031_d434;
                        continue;
                    }
                    pc = 0x0031_d42c;
                    continue;
                }
                0x0031_d42c => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x40;
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                    pc = 0x0031_d434;
                    continue;
                }
                0x0031_d434 => {
                    c.cycles += 9;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x0041_605c, 0x0031_d448)?;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 64i64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    if t {
                        pc = 0x0031_d4ac;
                        continue;
                    }
                    pc = 0x0031_d458;
                    continue;
                }
                0x0031_d458 => {
                    c.cycles += 7;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
                    MD5_TRANSFORM.call(c, 0x0031_d46c)?;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 64i64) as u64;
                    pc = 0x0031_d48c;
                    continue;
                }
                0x0031_d474 => {
                    c.cycles += 1;
                    pc = 0x0031_d478;
                    continue;
                }
                0x0031_d478 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    MD5_TRANSFORM.call(c, 0x0031_d480)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x40));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
                    pc = 0x0031_d488;
                    continue;
                }
                0x0031_d488 => {
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 64i64) as u64;
                    pc = 0x0031_d48c;
                    continue;
                }
                0x0031_d48c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        if back_edge_unwinds(c, 0x0031_d478) {
                            return Ok(());
                        }
                        pc = 0x0031_d478;
                        continue;
                    }
                    pc = 0x0031_d494;
                    continue;
                }
                0x0031_d494 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
                    if t {
                        pc = 0x0031_d4a8;
                        continue;
                    }
                    pc = 0x0031_d49c;
                    continue;
                }
                0x0031_d49c => {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x0041_605c, 0x0031_d4a8)?;
                    pc = 0x0031_d4a8;
                    continue;
                }
                0x0031_d4a8 => {
                    c.cycles += 1;
                    pc = 0x0031_d4ac;
                    continue;
                }
                0x0031_d4ac => {
                    c.cycles += 8;
                    let frame = Frame { size: 0x70 };
                    frame.restore(c, 16, 0x60);
                    frame.restore(c, 17, 0x50);
                    frame.restore(c, 18, 0x40);
                    frame.restore(c, 19, 0x30);
                    frame.restore(c, 20, 0x20);
                    frame.restore(c, 21, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0031_d3a8, 0x0031_d4cc);
                    return Ok(());
                }
            }
        }
    })
}

/// `uint64_t MD5_HashString(const char *s, void *out, ...)` (0x317710):
/// hash `s` with MD5, writing the 128-bit digest to `out` and the first 16
/// bytes over `s`, returning `s`.
pub fn md5_hash_string(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, 18, 0x80);
        frame.save(c, 16, 0xa0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x90);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x70);
        c.g[A0][0] = c.g[SP][0];
        MD5_INIT.call(c, 0x0031_7738)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_6810, 0x0031_7740)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[S0][0];
        MD5_UPDATE.call(c, 0x0031_7750)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(frame.at(c, 0x60));
        MD5_FINAL.call(c, 0x0031_775c)?;
        c.g[V1][0] = c.ld(frame.at(c, 0x68));
        c.g[V0][0] = c.g[S2][0];
        c.g[A0][0] = c.ld(frame.at(c, 0x60));
        c.sd((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.sd(c.g[S1][0] as u32, c.g[A0][0]);
        c.sd(c.g[S2][0] as u32, c.g[A0][0]);
        frame.restore(c, 16, 0xa0);
        c.g[V1][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x8));
        frame.restore(c, 17, 0x90);
        c.sd((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        frame.restore(c, 18, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `uint64_t MD5_HashString64(const char *s, void *out)` (0x317690): hash
/// `s` with MD5, writing the digest to the stack context and its low word
/// to `out`, returning the low digest word.
pub fn md5_hash_string64(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, 17, 0x80);
        frame.save(c, 16, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x70);
        c.g[A0][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A1][0];
        MD5_INIT.call(c, 0x0031_76b0)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_6810, 0x0031_76b8)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[S0][0];
        MD5_UPDATE.call(c, 0x0031_76c8)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(frame.at(c, 0x60));
        MD5_FINAL.call(c, 0x0031_76d4)?;
        c.g[V0][0] = c.ld(frame.at(c, 0x60));
        frame.restore(c, 16, 0x90);
        c.sd(c.g[S1][0] as u32, c.g[V0][0]);
        frame.restore(c, 17, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `char *MD5_HashStringDiscard(const char *s, ...)` (0x317798): run the
/// MD5 string hash over `s` into stack scratch, returning `s` (the digest
/// is discarded; the hash overwrites the string's first 16 bytes).
pub fn md5_hash_string_discard(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, 16, 0x20);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        MD5_HASH_STRING.call(c, 0x0031_77b4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        frame.restore(c, 16, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void MD5_HashString64Discard(const char *s)` (0x3176f0): run the 64-bit
/// MD5 string hash over `s` into stack scratch (the digest is discarded).
pub fn md5_hash_string64_discard(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        MD5_HASH_STRING64.call(c, 0x0031_7704)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    #[test]
    fn md5_init_matches() {
        check(ssx3_ee::s10::f_0031d368, md5_init, 100, &|c, seed| {
            let mut r = Rng(0x2019 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    use crate::difftest::check_paths;

    use crate::difftest::fakes::machine;

    use crate::difftest::fill;
    use crate::difftest::fresh;
    use crate::difftest::require_paths;

    /// `MD5_CTX` at `MD5_TRANSFORM_CTX`, input blocks at `MD5_TRANSFORM_INPUT + misalign`.
    pub(crate) const MD5_TRANSFORM_CTX: u32 = DATA + 0x100;

    pub(crate) const MD5_TRANSFORM_INPUT: u32 = DATA + 0x1000;

    /// The RFC 1321 test-suite messages and digests (A.5).
    pub(crate) const MD5_TRANSFORM_RFC: [(&str, &str); 7] = [
        ("", "d41d8cd98f00b204e9800998ecf8427e"),
        ("a", "0cc175b9c0f1b6a831c399e269772661"),
        ("abc", "900150983cd24fb0d6963f7d28e17f72"),
        ("message digest", "f96b697d7cb7938d525a2f31aaf161d0"),
        (
            "abcdefghijklmnopqrstuvwxyz",
            "c3fcd3d76192e4007dfb496cca67e13b",
        ),
        (
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            "d174ab98d277d9f5a5611c2c9f419d9f",
        ),
        (
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
            "57edf4a22be3c955ac49da2e2107b67a",
        ),
    ];

    /// The padded 64-byte blocks of `message` (one bit, zeros, 64-bit length).
    pub(crate) fn md5_transform_padded(message: &[u8]) -> Vec<[u8; 64]> {
        let mut bytes = message.to_vec();
        bytes.push(0x80);
        while bytes.len() % 64 != 56 {
            bytes.push(0);
        }
        bytes.extend_from_slice(&(message.len() as u64 * 8).to_le_bytes());
        bytes
            .chunks_exact(64)
            .map(|b| b.try_into().unwrap())
            .collect()
    }

    /// Every block of the RFC test suite, in message order.
    pub(crate) fn md5_transform_rfc_blocks() -> Vec<[u8; 64]> {
        MD5_TRANSFORM_RFC
            .iter()
            .flat_map(|(m, _)| md5_transform_padded(m.as_bytes()))
            .collect()
    }

    pub(crate) const MD5_TRANSFORM_INIT: [u32; 4] =
        [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476];

    /// Seed `s`: the input block is all zeros, all ones, an RFC test-suite
    /// block or random bytes (by `s % 4`); every other seed misaligns it by
    /// 1-3 bytes (the frame-copy arm). The state is the RFC initial value or
    /// random words; the frame is prefilled with random words.
    pub(crate) fn md5_transform_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x6d35_7472 ^ u64::from(seed) << 16);
        scramble(c, &mut r);
        let frame = crate::difftest::SP - MD5_TRANSFORM_FRAME;
        fill(c, &mut r, frame, MD5_TRANSFORM_FRAME / 4);
        fill(c, &mut r, MD5_TRANSFORM_INPUT - 0x10, 0x20);
        let rfc = md5_transform_rfc_blocks();
        let block: [u8; 64] = match seed % 4 {
            0 => [0; 64],
            1 => [0xff; 64],
            2 => rfc[(seed as usize / 4) % rfc.len()],
            _ => std::array::from_fn(|_| r.next_u32() as u8),
        };
        let misalign = if seed % 8 < 4 { 0 } else { 1 + seed / 8 % 3 };
        let input = MD5_TRANSFORM_INPUT + misalign;
        for (i, &b) in block.iter().enumerate() {
            c.bus.write8(input + i as u32, b);
        }
        fill(c, &mut r, MD5_TRANSFORM_CTX, 0x18 / 4);
        if seed % 3 == 0 {
            crate::difftest::put_words(
                c,
                MD5_TRANSFORM_CTX + MD5_TRANSFORM_STATE,
                &MD5_TRANSFORM_INIT,
            );
        }
        set_word(c, A0, MD5_TRANSFORM_CTX);
        set_word(c, A1, input);
        c.pc = 0x0031_c350;
    }

    /// Aligned blocks cost 17 + 2 + 977 cycles, unaligned ones 17 + 33 + 977.
    #[test]
    fn md5_transform_matches() {
        check_paths(
            ssx3_ee::s10::f_0031c350__generated,
            md5_transform,
            64,
            &[996, 1027],
            &md5_transform_fixture,
        );
    }

    /// The RFC 1321 test suite through the port and through the generated
    /// body: both must produce the published digests.
    #[test]
    fn md5_transform_rfc_digests() {
        for f in [
            ssx3_ee::s10::f_0031c350__generated as GuestFn,
            md5_transform,
        ] {
            for (misalign, (message, digest)) in MD5_TRANSFORM_RFC.iter().enumerate() {
                let mut c = fresh();
                crate::difftest::put_words(
                    &mut c,
                    MD5_TRANSFORM_CTX + MD5_TRANSFORM_STATE,
                    &MD5_TRANSFORM_INIT,
                );
                let input = MD5_TRANSFORM_INPUT + misalign as u32 % 4;
                for block in md5_transform_padded(message.as_bytes()) {
                    for (i, &b) in block.iter().enumerate() {
                        c.bus.write8(input + i as u32, b);
                    }
                    set_word(&mut c, A0, MD5_TRANSFORM_CTX);
                    set_word(&mut c, A1, input);
                    c.pc = 0x0031_c350;
                    f(&mut c);
                }
                let hex: String = (0..16)
                    .map(|i| {
                        format!(
                            "{:02x}",
                            c.bus.read8(MD5_TRANSFORM_CTX + MD5_TRANSFORM_STATE + i)
                        )
                    })
                    .collect();
                assert_eq!(&hex, digest, "MD5({message:?})");
            }
        }
    }

    /// The MD5 Update/Final fixture: a context whose bit count puts the
    /// buffered bytes at index 0, 10, 50, 55, 60 or 63 (one count wraps the
    /// low word, so the carry into the high word happens), and inputs of 0
    /// to 200 bytes; the game image holds Final's padding block. Outputs
    /// (`a1` for Final) are prefilled. Update-only seeds plant a zero
    /// count (the carry arm) and the index + length pairs summing to 65
    /// and 128 (the two tail edges).
    pub(crate) fn md5uf_fixture(c: &mut Cpu, seed: u32, addr: u32) -> Rng {
        let mut r = machine(c, seed, addr as u64, 0);
        crate::difftest::elf_image(c);
        const CTX: u32 = DATA + 0x100;
        const BUF: u32 = DATA + 0x400;
        crate::difftest::fill(c, &mut r, CTX, 0x20);
        let index = [0u32, 10, 50, 55, 60, 63][seed as usize % 6];
        let update = addr == 0x0031_d3a8;
        let low = if seed % 7 == 6 {
            0xffff_fe00 | index << 3
        } else {
            (r.next_u32() & !0x1ff) | index << 3
        };
        // A fresh context (count 0) takes the carry arm; Update-only so
        // Final's seeds and pins are untouched.
        let low = if update && seed % 13 == 12 { 0 } else { low };
        c.bus.write32(CTX, low);
        let mut len = match (seed / 6) % 8 {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 9,
            4 => 64,
            5 => 64 - index as usize,
            6 => 100,
            _ => (r.next_u32() % 200) as usize,
        };
        // S3 + len == 65 takes the transform arm with a buffered tail;
        // S3 + len == 128 leaves exactly 64 bytes after it (Update-only).
        if update && index != 0 {
            if seed % 11 == 10 {
                len = 65 - index as usize;
            } else if seed % 11 == 9 {
                len = 128 - index as usize;
            }
        }
        crate::difftest::fill(c, &mut r, BUF, 0x40);
        for i in 0..len {
            c.bus.write8(BUF + i as u32, 1 + (r.next_u32() % 95) as u8);
        }
        c.bus.write8(BUF + len as u32, 0);
        c.g[A0][0] = CTX as u64;
        c.g[A1][0] = BUF as u64;
        c.g[A2][0] = len as u64;
        c.pc = addr;
        r
    }

    /// `MD5_Update` / `MD5_Final`: every buffer index against short and
    /// long inputs (both the "fits in the buffer" and the transform paths),
    /// the count carry, and both padding lengths of Final.
    #[test]
    fn md5_update_final_matches() {
        for (gen, hand, addr, pins) in [
            (
                ee::s10::f_0031d3a8__generated as fn(&mut Cpu),
                md5_update as fn(&mut Cpu),
                0x0031_d3a8,
                // Empty input 27; buffered-only copies 62..163; transforms
                // from index 0 (1045) up to 200 bytes from index 63 (3523).
                // Gap arms (Update-only seeds): the count carry 237; the
                // S3+len == 65 tail 1095..1469; the S3+len == 128 tail
                // 1086..2483.
                &[
                    27, 62, 65, 76, 79, 118, 121, 163, 1045, 3523, 237, 1095, 1469, 1086, 2483,
                ][..],
            ),
            (
                ee::s10::f_0031d4d0__generated as fn(&mut Cpu),
                md5_final as fn(&mut Cpu),
                0x0031_d4d0,
                // Both padding lengths (index below / from 56) at each index.
                &[
                    1504, 1506, 1539, 1541, 1822, 1824, 1889, 1891, 2922, 2924, 2943, 2945,
                ][..],
            ),
        ] {
            check(gen, hand, 96, &|c, seed| {
                md5uf_fixture(c, seed, addr);
            });
            require_paths(gen, 96, pins, &|c, seed| {
                md5uf_fixture(c, seed, addr);
            });
        }
    }

    driven!(upd_gen, ee::s10::f_0031d3a8__generated);

    driven!(upd_hand, md5_update);

    driven!(muf_gen, ee::s10::f_0031d4d0__generated);

    driven!(muf_hand, md5_final);

    /// The MD5 loops under delivered events. Final's first head (0x31d4f0)
    /// is structured: the port must carry on, never resume there (Update's
    /// 0x31d478 head is a correct dispatcher edge; the second Final head is
    /// covered by `md5_final_second_loop_carries_on`).
    #[test]
    fn md5_update_final_events() {
        for (gen, hand, addr, no_resume) in [
            (
                upd_gen as fn(&mut Cpu),
                upd_hand as fn(&mut Cpu),
                0x0031_d3a8,
                None,
            ),
            (
                muf_gen as fn(&mut Cpu),
                muf_hand as fn(&mut Cpu),
                0x0031_d4d0,
                Some(0x0031_d4f0),
            ),
        ] {
            let resumes = check_events(gen, hand, 60, &|c, seed| {
                let mut r = machine(c, seed, addr as u64, 0);
                const CTX: u32 = DATA + 0x100;
                const BUF: u32 = DATA + 0x400;
                for i in 0..0x20u32 {
                    c.bus.write32(CTX + 4 * i, 0x7777_0000 + i);
                }
                let len = 40 + (r.next_u32() % 200) as usize;
                for i in 0..len {
                    c.bus.write8(BUF + i as u32, 1 + (r.next_u32() % 95) as u8);
                }
                c.bus.write8(BUF + len as u32, 0);
                c.g[A0][0] = CTX as u64;
                c.g[A1][0] = BUF as u64;
                c.g[A2][0] = len as u64;
                schedule(c, &mut r, seed, addr)
            });
            if let Some(head) = no_resume {
                assert!(
                    resumes.get(&head).is_none(),
                    "unwound at structured head {head:#x}: {resumes:x?}"
                );
            }
        }
    }

    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;

    driven!(fin_carry_gen, ee::s10::f_0031d4d0__generated);
    driven!(fin_carry_hand, md5_final);

    /// An unwind requested at a structured back edge must not unwind
    /// the port — the generated loop carries on (`deliver_event_here`).
    /// Fails on the `back_edge_unwinds` body (resume at 0x31d4f0).
    ///
    /// The VBlank is due at the t-th back edge of the first loop by
    /// construction (prologue 8 cycles, body 12, start 1000): `schedule`
    /// cannot do this, its k < 48 fires before a loop this short.
    #[test]
    fn md5_final_back_edge_carries_on() {
        for t in 1..=7u64 {
            let resumes = check_events(fin_carry_gen, fin_carry_hand, 4, &|c, seed| {
                md5uf_fixture(c, seed, 0x0031_d4d0);
                let saved = (c.g[A0], c.g[A1], c.g[31]);
                ssx3_ps2::kernel::start_main_thread(c, 0x0031_d4d0, 0, 0);
                (c.g[A0], c.g[A1], c.g[31]) = saved;
                c.pc = 0x0031_d4d0;
                c.cycles = 1000;
                let at = 1000 + 8 + 12 * t;
                c.next_event = at;
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .sched
                    .next_vblank = at;
                Events::Unwind
            });
            assert!(
                resumes.get(&0x0031_d4f0).is_none(),
                "t={t}: unwound at structured head 0x31d4f0: {resumes:x?}"
            );
        }
    }

    driven!(fin_edge2_gen, ee::s10::f_0031d4d0__generated);
    driven!(fin_edge2_hand, md5_final);

    /// The second loop at 0x31d560 never sees an unwind under
    /// `schedule` (delivery fires in the first loop), so this test sweeps
    /// the VBlank past the first loop and the two Updates (short seeds:
    /// lens 0, 1, 3, 9). Fails on the `back_edge_unwinds` body.
    #[test]
    fn md5_final_second_loop_carries_on() {
        for k in [1100u64, 1150, 1200, 1250, 1300, 1350, 1400, 1500] {
            let resumes = check_events(fin_edge2_gen, fin_edge2_hand, 4, &|c, seed| {
                md5uf_fixture(c, seed, 0x0031_d4d0);
                let saved = (c.g[A0], c.g[A1], c.g[31]);
                ssx3_ps2::kernel::start_main_thread(c, 0x0031_d4d0, 0, 0);
                (c.g[A0], c.g[A1], c.g[31]) = saved;
                c.pc = 0x0031_d4d0;
                c.cycles = 1000;
                c.next_event = 1000 + k;
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .sched
                    .next_vblank = 1000 + k;
                Events::Unwind
            });
            assert!(
                resumes.get(&0x0031_d560).is_none(),
                "k={k}: unwound at structured head 0x31d560: {resumes:x?}"
            );
        }
    }

    /// `MD5_HashString`: the digest overwrites the string's first 16 bytes
    /// and the out area on both sides.
    #[test]
    fn md5_hash_string_matches() {
        check(
            ee::s10::f_00317710__generated,
            md5_hash_string,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_7710, 0);
                let len = match seed % 8 {
                    0 => 0,
                    1 => 1,
                    2 => 55,
                    3 => 56,
                    4 => 57,
                    5 => 63,
                    6 => 64,
                    _ => (r.next_u32() % 65) as usize,
                };
                let out = DATA + 0x100;
                let s = DATA + 0x200;
                for i in 0..4 {
                    c.bus.write32(out + 4 * i, 0xeeee_0000 + i);
                }
                for i in 0..len {
                    c.bus.write8(s + i as u32, 1 + (r.next_u32() % 95) as u8);
                }
                c.bus.write8(s + len as u32, 0);
                for i in 0..0x10 {
                    c.bus.write8(s + len as u32 + 1 + i, 0x80 + i as u8);
                }
                c.g[A0][0] = s as u64;
                c.g[A1][0] = out as u64;
                c.g[A2][0] = s as u64;
                c.pc = 0x0031_7710;
            },
        );
        require_paths(ee::s10::f_00317710__generated, 100, &[2004], &|c, seed| {
            let mut r = machine(c, seed, 0x0031_7710, 0);
            let len = match seed % 8 {
                0 => 0,
                1 => 1,
                2 => 55,
                3 => 56,
                4 => 57,
                5 => 63,
                6 => 64,
                _ => (r.next_u32() % 65) as usize,
            };
            let out = DATA + 0x100;
            let s = DATA + 0x200;
            for i in 0..4 {
                c.bus.write32(out + 4 * i, 0xeeee_0000 + i);
            }
            for i in 0..len {
                c.bus.write8(s + i as u32, 1 + (r.next_u32() % 95) as u8);
            }
            c.bus.write8(s + len as u32, 0);
            for i in 0..0x10 {
                c.bus.write8(s + len as u32 + 1 + i, 0x80 + i as u8);
            }
            c.g[A0][0] = s as u64;
            c.g[A1][0] = out as u64;
            c.g[A2][0] = s as u64;
            c.pc = 0x0031_7710;
        });
    }

    /// `MD5_HashString64`: strings from empty across the padding boundary,
    /// hashed into a prefilled out word on both sides.
    #[test]
    fn md5_hash_string64_matches() {
        check(
            ee::s10::f_00317690__generated,
            md5_hash_string64,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_7690, 0);
                let len = match seed % 8 {
                    0 => 0,
                    1 => 1,
                    2 => 55,
                    3 => 56,
                    4 => 57,
                    5 => 63,
                    6 => 64,
                    _ => (r.next_u32() % 65) as usize,
                };
                let out = DATA + 0x100;
                let s = DATA + 0x200;
                for i in 0..4 {
                    c.bus.write32(out + 4 * i, 0xeeee_0000 + i);
                }
                for i in 0..len {
                    c.bus.write8(s + i as u32, 1 + (r.next_u32() % 95) as u8);
                }
                c.bus.write8(s + len as u32, 0);
                c.g[A0][0] = out as u64;
                c.g[A1][0] = s as u64;
                c.pc = 0x0031_7690;
            },
        );
        require_paths(
            ee::s10::f_00317690__generated,
            100,
            &[
                1995, 2033, 2040, 2054, 2056, 2068, 2084, 2089, 2096, 2105, 2110, 2112, 2124, 2133,
                2152, 3052, 3586, 3593, 3635,
            ],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_7690, 0);
                let len = match seed % 8 {
                    0 => 0,
                    1 => 1,
                    2 => 55,
                    3 => 56,
                    4 => 57,
                    5 => 63,
                    6 => 64,
                    _ => (r.next_u32() % 65) as usize,
                };
                let out = DATA + 0x100;
                let s = DATA + 0x200;
                for i in 0..4 {
                    c.bus.write32(out + 4 * i, 0xeeee_0000 + i);
                }
                for i in 0..len {
                    c.bus.write8(s + i as u32, 1 + (r.next_u32() % 95) as u8);
                }
                c.bus.write8(s + len as u32, 0);
                c.g[A0][0] = out as u64;
                c.g[A1][0] = s as u64;
                c.pc = 0x0031_7690;
            },
        );
    }

    /// `MD5_HashStringDiscard`: strings across the padding boundary hash
    /// into the string's first 16 bytes on both sides, returning the string.
    #[test]
    fn md5_hash_string_discard_matches() {
        check(
            ee::s10::f_00317798__generated,
            md5_hash_string_discard,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_7798, 0);
                let len = match seed % 8 {
                    0 => 0,
                    1 => 1,
                    2 => 55,
                    3 => 56,
                    4 => 57,
                    5 => 63,
                    6 => 64,
                    _ => (r.next_u32() % 65) as usize,
                };
                let s = DATA + 0x100;
                for i in 0..len {
                    c.bus.write8(s + i as u32, 1 + (r.next_u32() % 95) as u8);
                }
                c.bus.write8(s + len as u32, 0);
                for i in 0..0x10u32 {
                    c.bus.write8(s + len as u32 + 1 + i, 0x80 + i as u8);
                }
                c.g[A0][0] = s as u64;
                c.g[A1][0] = s as u64;
                c.g[A2][0] = 0xdddd_0000 + seed as u64;
                c.pc = 0x0031_7798;
            },
        );
    }

    /// `MD5_HashString64Discard`: strings from empty to 64 bytes cross the
    /// padding boundary, so `MD5_Update`/`MD5_Final` take both arms on both
    /// sides; the digest lands in stack scratch.
    #[test]
    fn md5_hash_string64_discard_matches() {
        check(
            ee::s10::f_003176f0__generated,
            md5_hash_string64_discard,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_76f0, 0);
                let len = match seed % 6 {
                    0 => 0,
                    1 => 55,
                    2 => 56,
                    3 => 57,
                    4 => 64,
                    _ => (r.next_u32() % 65) as usize,
                };
                let s = DATA + 0x100;
                for i in 0..len {
                    c.bus.write8(s + i as u32, 1 + (r.next_u32() % 95) as u8);
                }
                c.bus.write8(s + len as u32, 0);
                c.g[A0][0] = s as u64;
                c.pc = 0x0031_76f0;
            },
        );
    }
}
