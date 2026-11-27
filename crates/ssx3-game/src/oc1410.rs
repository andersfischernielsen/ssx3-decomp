//! Staging module oc1410 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1410`.

use crate::abi::set_word;
use crate::call::{sp, Frame};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

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

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0031_c350, md5_transform, "MD5_Transform")];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_paths, fill, fresh, scramble, Rng, DATA};

    /// `MD5_CTX` at `MD5_TRANSFORM_CTX`, input blocks at `MD5_TRANSFORM_INPUT + misalign`.
    const MD5_TRANSFORM_CTX: u32 = DATA + 0x100;
    const MD5_TRANSFORM_INPUT: u32 = DATA + 0x1000;

    /// The RFC 1321 test-suite messages and digests (A.5).
    const MD5_TRANSFORM_RFC: [(&str, &str); 7] = [
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
    fn md5_transform_padded(message: &[u8]) -> Vec<[u8; 64]> {
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
    fn md5_transform_rfc_blocks() -> Vec<[u8; 64]> {
        MD5_TRANSFORM_RFC
            .iter()
            .flat_map(|(m, _)| md5_transform_padded(m.as_bytes()))
            .collect()
    }

    const MD5_TRANSFORM_INIT: [u32; 4] = [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476];

    /// Seed `s`: the input block is all zeros, all ones, an RFC test-suite
    /// block or random bytes (by `s % 4`); every other seed misaligns it by
    /// 1-3 bytes (the frame-copy arm). The state is the RFC initial value or
    /// random words; the frame is prefilled with random words.
    fn md5_transform_fixture(c: &mut Cpu, seed: u32) {
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
}
